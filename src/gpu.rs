// Port of c/gpu.c — GPU information. The original queried OpenCL for the device
// name/vendor/driver version. OpenCL isn't reachable from std alone, so we read
// the same identity from `lspci -mm` (driver version is not available this way).

use std::process::{Command, Stdio};

pub struct GpuInfo {
    pub name: String,
    pub version: String,
    pub vendor: String,
}

impl Default for GpuInfo {
    fn default() -> Self {
        GpuInfo {
            name: "Unknown".to_string(),
            version: "Unknown".to_string(),
            vendor: "Unknown".to_string(),
        }
    }
}

/// Extract the contents of each double-quoted field on an `lspci -mm` line, in
/// order. For a display device the fields are: [class, vendor, device, ...].
fn quoted_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    for c in line.chars() {
        if c == '"' {
            if in_quote {
                fields.push(std::mem::take(&mut current));
            }
            in_quote = !in_quote;
        } else if in_quote {
            current.push(c);
        }
    }
    fields
}

pub fn get_gpu_info() -> GpuInfo {
    let mut info = GpuInfo::default();

    let output = Command::new("lspci")
        .arg("-mm")
        .stderr(Stdio::null())
        .output();
    let output = match output {
        Ok(o) => o,
        Err(_) => return info,
    };
    let text = String::from_utf8_lossy(&output.stdout);

    for line in text.lines() {
        let fields = quoted_fields(line);
        let Some(class) = fields.first() else {
            continue;
        };
        // VGA compatible / 3D / Display controller classes.
        if class.contains("VGA") || class.contains("3D") || class.contains("Display") {
            if let Some(vendor) = fields.get(1) {
                info.vendor = vendor.clone();
            }
            if let Some(device) = fields.get(2) {
                info.name = device.clone();
            }
            break;
        }
    }
    info
}
