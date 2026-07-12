// CPU section — `inxi -C`. Sources: /proc/cpuinfo and /sys cpufreq/cache.

use crate::util::{header, human_size, key, print_group, read_trim, seg};
use std::collections::BTreeMap;
use std::io::{self, Write};

#[derive(Default)]
struct CpuInfo {
    brand: String,
    vendor: String,
    family: i64,
    model: i64,
    stepping: i64,
    microcode: String,
    bogomips: String,
    cores: i64,
    threads: i64,
    flags: Vec<String>,
}

fn parse_cpuinfo() -> CpuInfo {
    let mut info = CpuInfo::default();
    let content = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    for line in content.lines() {
        let Some((k, v)) = line.split_once(':') else {
            continue;
        };
        let (k, v) = (k.trim(), v.trim());
        match k {
            "processor" => info.threads += 1,
            "model name" if info.brand.is_empty() => info.brand = v.to_string(),
            "vendor_id" if info.vendor.is_empty() => info.vendor = v.to_string(),
            "cpu family" => info.family = v.parse().unwrap_or(0),
            "model" => info.model = v.parse().unwrap_or(0),
            "stepping" => info.stepping = v.parse().unwrap_or(0),
            "microcode" if info.microcode.is_empty() => info.microcode = v.to_string(),
            "bogomips" if info.bogomips.is_empty() => info.bogomips = v.to_string(),
            "cpu cores" => info.cores = v.parse().unwrap_or(0),
            "flags" if info.flags.is_empty() => {
                info.flags = v.split_whitespace().map(str::to_string).collect();
            }
            _ => {}
        }
    }
    if info.cores == 0 {
        info.cores = info.threads;
    }
    info
}

/// Trim marketing noise from the brand string the way inxi does,
/// e.g. "AMD Ryzen 5 5600 6-Core Processor" -> "AMD Ryzen 5 5600".
fn clean_model(brand: &str) -> String {
    let mut m = brand.trim();
    if let Some(pos) = m.find("-Core Processor")
        && let Some(sp) = m[..pos].rfind(' ')
    {
        m = m[..sp].trim();
    }
    for suffix in [" Processor", " CPU", " APU"] {
        if let Some(s) = m.strip_suffix(suffix) {
            m = s.trim();
        }
    }
    m.to_string()
}

fn vendor_short(v: &str) -> &str {
    match v {
        "AuthenticAMD" => "AMD",
        "GenuineIntel" => "Intel",
        _ => v,
    }
}

/// Parse a sysfs cache size like "32K" or "16M" into bytes.
fn parse_cache_size(s: &str) -> u64 {
    let s = s.trim();
    let (num, mult) = if let Some(n) = s.strip_suffix('K') {
        (n, 1024)
    } else if let Some(n) = s.strip_suffix('M') {
        (n, 1024 * 1024)
    } else {
        (s, 1)
    };
    num.parse::<u64>().unwrap_or(0) * mult
}

/// Sum unique caches per level across all CPUs (dedup shared caches by id).
fn cache_by_level() -> BTreeMap<u32, u64> {
    let mut seen = std::collections::HashSet::new();
    let mut totals: BTreeMap<u32, u64> = BTreeMap::new();
    let Ok(cpus) = std::fs::read_dir("/sys/devices/system/cpu") else {
        return totals;
    };
    for cpu in cpus.flatten() {
        let name = cpu.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with("cpu") || !name[3..].chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let cache_dir = cpu.path().join("cache");
        let Ok(indices) = std::fs::read_dir(&cache_dir) else {
            continue;
        };
        for idx in indices.flatten() {
            let p = idx.path();
            let level = read_trim(&p.join("level").to_string_lossy())
                .and_then(|s| s.parse::<u32>().ok());
            let ctype = read_trim(&p.join("type").to_string_lossy()).unwrap_or_default();
            let size = read_trim(&p.join("size").to_string_lossy()).unwrap_or_default();
            let id = read_trim(&p.join("id").to_string_lossy()).unwrap_or_default();
            let Some(level) = level else { continue };
            // Unique key: level + type + shared-cache id.
            if !seen.insert((level, ctype.clone(), id)) {
                continue;
            }
            *totals.entry(level).or_insert(0) += parse_cache_size(&size);
        }
    }
    totals
}

fn read_khz(path: &str) -> Option<u64> {
    read_trim(path).and_then(|s| s.parse::<u64>().ok())
}

/// Gather per-CPU current speeds (MHz) plus min/max.
fn speeds() -> (Vec<u64>, u64, u64) {
    let mut cur = Vec::new();
    let mut min = u64::MAX;
    let mut max = 0u64;
    let mut n = 0;
    loop {
        let base = format!("/sys/devices/system/cpu/cpu{n}/cpufreq");
        if !std::path::Path::new(&base).exists() {
            break;
        }
        if let Some(c) = read_khz(&format!("{base}/scaling_cur_freq")) {
            cur.push(c / 1000);
        }
        if let Some(mn) = read_khz(&format!("{base}/scaling_min_freq")) {
            min = min.min(mn / 1000);
        }
        if let Some(mx) = read_khz(&format!("{base}/scaling_max_freq")) {
            max = max.max(mx / 1000);
        }
        n += 1;
    }
    if min == u64::MAX {
        min = 0;
    }
    (cur, min, max)
}

/// inxi's "basic" flag subset, shown intersected with the CPU's real flags.
const BASIC_FLAGS: &[&str] = &[
    "avx", "avx2", "ht", "lm", "nx", "pae", "sse", "sse2", "sse3", "sse4_1", "sse4_2", "sse4a",
    "ssse3", "svm", "vmx",
];

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "CPU")?;
    let info = parse_cpuinfo();

    // Line 1: Info / model / bits / type / rev / cache.
    let mut line = vec![seg("Info", format!("{}-core", info.cores))];
    line.push(seg("model", clean_model(&info.brand)));
    let _ = vendor_short(&info.vendor); // vendor abbreviation available if needed
    line.push(seg("bits", if cfg!(target_pointer_width = "64") { "64" } else { "32" }));
    let mut ttype = Vec::new();
    if info.threads > info.cores {
        ttype.push("MT");
    }
    if info.cores > 1 {
        ttype.push("MCP");
    }
    if !ttype.is_empty() {
        line.push(seg("type", ttype.join(" ")));
    }
    line.push(seg("family", format!("0x{:x} ({})", info.family, info.family)));
    line.push(seg("model-id", format!("0x{:x} ({})", info.model, info.model)));
    line.push(seg("stepping", info.stepping));
    if !info.microcode.is_empty() {
        line.push(seg("microcode", &info.microcode));
    }

    let caches = cache_by_level();
    if !caches.is_empty() {
        line.push(key("cache"));
        if let Some(&l1) = caches.get(&1) {
            line.push(seg("L1", human_size(l1)));
        }
        if let Some(&l2) = caches.get(&2) {
            line.push(seg("L2", human_size(l2)));
        }
        if let Some(&l3) = caches.get(&3) {
            line.push(seg("L3", human_size(l3)));
        }
    }
    print_group(out, 2, 4, &line)?;

    // Line 2: Speeds.
    let (cur, min, max) = speeds();
    let mut sp = Vec::new();
    if !cur.is_empty() {
        let avg = cur.iter().sum::<u64>() / cur.len() as u64;
        sp.push(seg("Speed (MHz)", format!("avg {avg}")));
        if max > 0 {
            sp.push(seg("min/max", format!("{min}/{max}")));
        }
        sp.push(key("cores"));
        for (i, mhz) in cur.iter().enumerate() {
            sp.push(seg(&format!("{}", i + 1), mhz));
        }
    }
    if !info.bogomips.is_empty() {
        sp.push(seg("bogomips", info.bogomips.trim_end_matches(".00")));
    }
    if !sp.is_empty() {
        print_group(out, 2, 4, &sp)?;
    }

    // Line 3: basic flags.
    let basic: Vec<&String> = info
        .flags
        .iter()
        .filter(|f| BASIC_FLAGS.contains(&f.as_str()))
        .collect();
    if !basic.is_empty() {
        let mut names: Vec<&str> = basic.iter().map(|s| s.as_str()).collect();
        names.sort_unstable();
        let mut fl = vec![key("Flags-basic")];
        for n in names {
            fl.push(n.to_string());
        }
        print_group(out, 2, 4, &fl)?;
    }
    Ok(())
}
