// Drives section — `inxi -D`. Sources: /sys/block + df for usage totals.

use crate::util::{header, human_size, key, mounts, percent, print_group, read_trim, seg};
use std::io::{self, Write};

fn is_physical(name: &str) -> bool {
    !(name.starts_with("loop")
        || name.starts_with("ram")
        || name.starts_with("zram")
        || name.starts_with("dm-")
        || name.starts_with("md")
        || name.starts_with("sr"))
}

struct Drive {
    dev: String,
    vendor: String,
    model: String,
    size: u64,
}

fn drives() -> Vec<Drive> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir("/sys/block") else {
        return out;
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| is_physical(n))
        .collect();
    names.sort();

    for name in names {
        let base = format!("/sys/block/{name}");
        let sectors = read_trim(&format!("{base}/size"))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let model = read_trim(&format!("{base}/device/model")).unwrap_or_default();
        let vendor = read_trim(&format!("{base}/device/vendor"))
            .filter(|v| !v.is_empty() && v != "ATA")
            .unwrap_or_default();
        out.push(Drive {
            dev: format!("/dev/{name}"),
            vendor,
            model,
            size: sectors * 512,
        });
    }
    out
}

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "Drives")?;
    let list = drives();

    let total: u64 = list.iter().map(|d| d.size).sum();
    // Used = sum of used bytes across real filesystems.
    let used: u64 = mounts()
        .iter()
        .filter(|m| !crate::util::is_pseudo_fs(&m.fstype))
        .map(|m| m.used)
        .sum();
    print_group(
        out,
        2,
        4,
        &[
            key("Local Storage"),
            seg("total", human_size(total)),
            seg("used", format!("{} {}", human_size(used), percent(used, total))),
        ],
    )?;

    for (i, d) in list.iter().enumerate() {
        let mut line = vec![seg(&format!("ID-{}", i + 1), &d.dev)];
        if !d.vendor.is_empty() {
            line.push(seg("vendor", &d.vendor));
        }
        if !d.model.is_empty() {
            line.push(seg("model", &d.model));
        }
        line.push(seg("size", human_size(d.size)));
        print_group(out, 2, 4, &line)?;
    }
    Ok(())
}
