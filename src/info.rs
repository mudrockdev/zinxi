// Info section — `inxi -I`. Sources: /proc/meminfo, /proc/uptime, /proc,
// systemd, package managers, compilers.

use crate::util::{has_command, header, human_size, key, percent, print_group, read_trim, run_capture, seg};
use std::io::{self, Write};

/// (total, available, used) memory in bytes, from /proc/meminfo.
pub fn memory() -> (u64, u64, u64) {
    let content = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0;
    let mut avail = 0;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("MemTotal:") {
            total = parse_kb(rest);
        } else if let Some(rest) = line.strip_prefix("MemAvailable:") {
            avail = parse_kb(rest);
        }
    }
    (total, avail, total.saturating_sub(avail))
}

fn parse_kb(s: &str) -> u64 {
    s.split_whitespace()
        .next()
        .and_then(|n| n.parse::<u64>().ok())
        .map(|kb| kb * 1024)
        .unwrap_or(0)
}

fn process_count() -> usize {
    std::fs::read_dir("/proc")
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .chars()
                        .all(|c| c.is_ascii_digit())
                })
                .count()
        })
        .unwrap_or(0)
}

fn uptime() -> String {
    let secs = read_trim("/proc/uptime")
        .and_then(|s| s.split_whitespace().next().map(str::to_string))
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0) as u64;
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if hours > 0 || days > 0 {
        parts.push(format!("{hours}h"));
    }
    parts.push(format!("{mins}m"));
    parts.join(" ")
}

/// Parent process name — the shell zinxi was launched from.
fn parent_shell() -> Option<String> {
    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    // Field 4 is ppid; comm (field 2) is wrapped in parens which may contain spaces.
    let close = stat.rfind(')')?;
    let after: Vec<&str> = stat[close + 1..].split_whitespace().collect();
    let ppid = after.get(1)?; // state, ppid, ...
    read_trim(&format!("/proc/{ppid}/comm"))
}

fn compiler_version(cmd: &str) -> Option<String> {
    if !has_command(cmd) {
        return None;
    }
    run_capture(cmd, &["-dumpfullversion"])
        .filter(|s| !s.trim().is_empty())
        .or_else(|| run_capture(cmd, &["-dumpversion"]))
        .map(|s| s.trim().to_string())
}

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "Info")?;

    // Memory line.
    let (total, avail, used) = memory();
    print_group(
        out,
        2,
        4,
        &[
            key("Memory"),
            seg("total", human_size(total)),
            seg("available", human_size(avail)),
            seg("used", format!("{} {}", human_size(used), percent(used, total))),
        ],
    )?;

    // Processes / uptime / init.
    let mut line = vec![seg("Processes", process_count()), seg("uptime", uptime())];
    if std::path::Path::new("/run/systemd/system").exists() {
        line.push(seg("Init", "systemd"));
        if let Some(v) = run_capture("systemctl", &["--version"])
            .and_then(|o| o.split_whitespace().nth(1).map(str::to_string))
        {
            line.push(seg("v", v));
        }
        if let Some(def) = run_capture("systemctl", &["get-default"]) {
            line.push(seg("default", def.trim().trim_end_matches(".target")));
        }
    }
    print_group(out, 2, 4, &line)?;

    // Packages / compilers / shell / version.
    let mut line = Vec::new();
    if let Some(pm) = package_manager() {
        line.push(seg("Packages", pm.0));
        line.push(seg("pkgs", pm.1));
    }
    let compilers: Vec<(&str, String)> = [("clang", "clang"), ("gcc", "gcc")]
        .iter()
        .filter_map(|(label, cmd)| compiler_version(cmd).map(|v| (*label, v)))
        .collect();
    if !compilers.is_empty() {
        line.push(key("Compilers"));
        for (label, v) in compilers {
            line.push(seg(label, v));
        }
    }
    if let Some(sh) = parent_shell() {
        line.push(seg("Shell", sh));
    }
    line.push(seg("zinxi", env!("CARGO_PKG_VERSION")));
    print_group(out, 2, 4, &line)?;
    Ok(())
}

/// Detect the primary package manager and count installed packages.
fn package_manager() -> Option<(String, String)> {
    if has_command("rpm") {
        let count = run_capture("rpm", &["-qa"]).map(count_lines).unwrap_or(0);
        return Some(("rpm".into(), count.to_string()));
    }
    if has_command("dpkg-query") {
        let count = run_capture("dpkg-query", &["-f", ".\n", "-W"])
            .map(count_lines)
            .unwrap_or(0);
        return Some(("dpkg".into(), count.to_string()));
    }
    if has_command("pacman") {
        let count = run_capture("pacman", &["-Qq"]).map(count_lines).unwrap_or(0);
        return Some(("pacman".into(), count.to_string()));
    }
    None
}

fn count_lines(s: String) -> usize {
    s.lines().filter(|l| !l.trim().is_empty()).count()
}
