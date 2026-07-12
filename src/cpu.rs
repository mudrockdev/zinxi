// Port of c/cpu.c — CPU information. The original used libcpuid; here we parse
// /proc/cpuinfo, which exposes the same fields with no dependencies.

use std::fs;

#[derive(Default)]
pub struct CpuInfo {
    pub brand: String,
    pub vendor: String,
    pub family: i32,
    pub model: i32,
    pub stepping: i32,
    pub cores: i32,
    pub threads: i32,
}

pub fn get_cpu_info() -> CpuInfo {
    let mut info = CpuInfo::default();
    let content = match fs::read_to_string("/proc/cpuinfo") {
        Ok(c) => c,
        Err(_) => return info,
    };

    for line in content.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        match key {
            // One "processor" block per logical CPU (thread).
            "processor" => info.threads += 1,
            "model name" if info.brand.is_empty() => info.brand = value.to_string(),
            "vendor_id" if info.vendor.is_empty() => info.vendor = value.to_string(),
            "cpu family" => info.family = value.parse().unwrap_or(0),
            "model" => info.model = value.parse().unwrap_or(0),
            "stepping" => info.stepping = value.parse().unwrap_or(0),
            "cpu cores" => info.cores = value.parse().unwrap_or(0),
            _ => {}
        }
    }

    // Fall back to logical count if "cpu cores" was absent.
    if info.cores == 0 {
        info.cores = info.threads;
    }
    info
}
