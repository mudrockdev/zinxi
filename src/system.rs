// Port of c/system.c — Linux-specific system information, using only sysfs and
// procfs (no libc uname binding needed).

use std::fs;

#[derive(Default)]
pub struct SystemInfo {
    // KERNEL INFO
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,

    // COMPILER INFO
    pub compiler_name: String,

    // CLOCKSOURCE
    pub clc_current: String,
    pub clc_avail: String,

    // PARAMETERS
    pub params: String,
}

/// Read a file and trim surrounding whitespace/newlines, or "" on failure.
fn read_trim(path: &str) -> String {
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn kernel_info(info: &mut SystemInfo) {
    info.sysname = read_trim("/proc/sys/kernel/ostype");
    info.nodename = read_trim("/proc/sys/kernel/hostname");
    info.release = read_trim("/proc/sys/kernel/osrelease");
    info.version = read_trim("/proc/sys/kernel/version");
    // Equivalent of uname's `machine` field, resolved at compile time.
    info.machine = std::env::consts::ARCH.to_string();
}

fn compiler_info(info: &mut SystemInfo) {
    // Mirrors readlink("/usr/bin/cc", ...): resolve the symlink target.
    if let Ok(target) = fs::read_link("/usr/bin/cc") {
        info.compiler_name = target.to_string_lossy().into_owned();
    }
}

fn clocksource(info: &mut SystemInfo) {
    let base = "/sys/devices/system/clocksource/clocksource0";
    let current = read_trim(&format!("{base}/current_clocksource"));
    info.clc_current = current.clone();

    // Available sources minus the current one, space separated.
    let avail = read_trim(&format!("{base}/available_clocksource"));
    let mut out = String::new();
    for source in avail.split_whitespace() {
        if source != current {
            out.push_str(source);
            out.push(' ');
        }
    }
    info.clc_avail = out;
}

fn parameters(info: &mut SystemInfo) {
    info.params = read_trim("/proc/cmdline");
}

pub fn get_system_info() -> SystemInfo {
    let mut info = SystemInfo::default();
    kernel_info(&mut info);
    compiler_info(&mut info);
    clocksource(&mut info);
    parameters(&mut info);
    info
}
