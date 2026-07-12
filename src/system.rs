// System section — `inxi -S`.
// Sources: /proc/sys/kernel, sysfs clocksource, /etc/os-release, env vars.

use crate::util::{header, print_group, read_or_empty, read_trim, run_capture, seg};
use std::io::{self, Write};

fn kernel_bits() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" | "aarch64" | "powerpc64" | "riscv64" | "loongarch64" | "s390x" | "mips64" => "64",
        _ => "32",
    }
}

/// Compiler name (from /usr/bin/cc) and version.
fn compiler() -> (String, String) {
    let name = std::fs::read_link("/usr/bin/cc")
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_default();
    if name.is_empty() {
        return (String::new(), String::new());
    }
    // `-dumpfullversion` gives the full x.y.z; fall back to `-dumpversion`.
    let version = run_capture(&name, &["-dumpfullversion"])
        .filter(|s| !s.trim().is_empty())
        .or_else(|| run_capture(&name, &["-dumpversion"]))
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    (name, version)
}

fn os_release_value(key: &str) -> Option<String> {
    let content = std::fs::read_to_string("/etc/os-release").ok()?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix(key)
            && let Some(val) = rest.strip_prefix('=')
        {
            return Some(val.trim().trim_matches('"').to_string());
        }
    }
    None
}

/// Display manager, from the systemd display-manager.service symlink.
fn display_manager() -> Option<String> {
    let target = std::fs::read_link("/etc/systemd/system/display-manager.service").ok()?;
    let name = target.file_stem()?.to_string_lossy().into_owned();
    // Capitalize common names (gdm -> GDM, lightdm -> LightDM best effort).
    Some(match name.as_str() {
        "gdm" => "GDM".to_string(),
        "sddm" => "SDDM".to_string(),
        "lightdm" => "LightDM".to_string(),
        other => other.to_string(),
    })
}

fn clocksource(base: &str) -> (String, String) {
    let current = read_or_empty(&format!("{base}/current_clocksource"));
    let avail_raw = read_or_empty(&format!("{base}/available_clocksource"));
    let avail: Vec<&str> = avail_raw.split_whitespace().filter(|s| *s != current).collect();
    (current, avail.join(","))
}

pub fn print<W: Write>(out: &mut W, show_host: bool) -> io::Result<()> {
    header(out, "System")?;

    // Line 1: Host / Kernel / arch / bits / compiler / clocksource.
    let mut line1 = Vec::new();
    if show_host
        && let Some(host) = read_trim("/proc/sys/kernel/hostname")
    {
        line1.push(seg("Host", host));
    }
    line1.push(seg("Kernel", read_or_empty("/proc/sys/kernel/osrelease")));
    line1.push(seg("arch", std::env::consts::ARCH));
    line1.push(seg("bits", kernel_bits()));
    let (cc_name, cc_ver) = compiler();
    if !cc_name.is_empty() {
        line1.push(seg("compiler", &cc_name));
        if !cc_ver.is_empty() {
            line1.push(seg("v", &cc_ver));
        }
    }

    let base = "/sys/devices/system/clocksource/clocksource0";
    let (clc_current, clc_avail) = clocksource(base);
    if !clc_current.is_empty() {
        line1.push(seg("clocksource", &clc_current));
    }
    if !clc_avail.is_empty() {
        line1.push(seg("avail", &clc_avail));
    }
    if let Some(params) = read_trim("/proc/cmdline") {
        line1.push(seg("parameters", params));
    }
    print_group(out, 2, 4, &line1)?;

    // Line 2: Desktop / wm / dm / Distro.
    let mut line2 = Vec::new();
    if let Some(de) = std::env::var("XDG_CURRENT_DESKTOP").ok().filter(|s| !s.is_empty()) {
        line2.push(seg("Desktop", de));
    }
    if let Some(wm) = std::env::var("WINDOW_MANAGER").ok().filter(|s| !s.is_empty()) {
        line2.push(seg("wm", wm));
    }
    if let Some(dm) = display_manager() {
        line2.push(seg("dm", dm));
    }
    if let Some(distro) = os_release_value("PRETTY_NAME") {
        line2.push(seg("Distro", distro));
    }
    if !line2.is_empty() {
        print_group(out, 2, 4, &line2)?;
    }
    Ok(())
}
