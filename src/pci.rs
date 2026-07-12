// Shared PCI device inventory, parsed from `lspci -vmmk` (machine-readable,
// includes the kernel driver). Used by the Graphics, Audio and Network sections.

use crate::util::run_capture;

#[derive(Default, Clone)]
pub struct PciDevice {
    pub slot: String,    // e.g. "28:00.0"
    pub class: String,   // e.g. "VGA compatible controller"
    pub vendor: String,  // e.g. "Intel Corporation"
    pub device: String,  // e.g. "Battlemage G21 [Arc B580]"
    pub svendor: String, // subsystem vendor
    pub rev: String,
    pub driver: String,
    pub vendor_id: String, // hex, e.g. "8086"
    pub device_id: String, // hex, e.g. "e20b"
}

impl PciDevice {
    /// Vendor with the trailing " Corporation"/"Inc."-style noise trimmed, the
    /// way inxi shortens it (best effort).
    pub fn short_vendor(&self) -> String {
        let v = self.vendor.trim();
        for suffix in [
            " Corporation",
            " Corp.",
            " Co., Ltd.",
            " Co. Ltd.",
            ", Inc.",
            " Inc.",
        ] {
            if let Some(stripped) = v.strip_suffix(suffix) {
                return stripped.to_string();
            }
        }
        v.to_string()
    }

    pub fn chip_id(&self) -> String {
        format!("{}:{}", self.vendor_id, self.device_id)
    }
}

/// Parse all PCI devices. Uses `-vmmk` for driver info plus `-nmm` (numeric) to
/// recover vendor/device IDs, keyed by slot.
pub fn devices() -> Vec<PciDevice> {
    let text = match run_capture("lspci", &["-vmmk"]) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let mut devices = Vec::new();
    let mut cur = PciDevice::default();
    let mut in_record = false;

    for line in text.lines() {
        if line.trim().is_empty() {
            if in_record {
                devices.push(std::mem::take(&mut cur));
                in_record = false;
            }
            continue;
        }
        let Some((k, v)) = line.split_once(':') else {
            continue;
        };
        let v = v.trim().to_string();
        in_record = true;
        match k {
            "Slot" => cur.slot = v,
            "Class" => cur.class = v,
            "Vendor" => cur.vendor = v,
            "Device" => cur.device = v,
            "SVendor" => cur.svendor = v,
            "Rev" => cur.rev = v,
            "Driver" => cur.driver = v,
            _ => {}
        }
    }
    if in_record {
        devices.push(cur);
    }

    fill_ids(&mut devices);
    devices
}

/// Fill vendor_id/device_id from `lspci -nmm`, matched by slot.
fn fill_ids(devices: &mut [PciDevice]) {
    let Some(text) = run_capture("lspci", &["-nmm"]) else {
        return;
    };
    for line in text.lines() {
        // Format: `28:00.0 "0300" "8086" "e20b" -r05 "1043" "8975"`
        let slot = match line.split_whitespace().next() {
            Some(s) => s,
            None => continue,
        };
        let ids: Vec<String> = quoted_fields(line);
        // ids: [class, vendor, device, svendor, sdevice]
        if let Some(dev) = devices.iter_mut().find(|d| d.slot == slot) {
            if let Some(vid) = ids.get(1) {
                dev.vendor_id = vid.clone();
            }
            if let Some(did) = ids.get(2) {
                dev.device_id = did.clone();
            }
        }
    }
}

/// Extract the contents of each double-quoted field on a line, in order.
pub fn quoted_fields(line: &str) -> Vec<String> {
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

/// True if a class string denotes a display controller.
pub fn is_display(class: &str) -> bool {
    class.contains("VGA") || class.contains("3D") || class.contains("Display")
}

/// True if a class string denotes an audio device.
pub fn is_audio(class: &str) -> bool {
    class.contains("Audio") || class.contains("Multimedia")
}

/// True if a class string denotes a network controller.
pub fn is_network(class: &str) -> bool {
    class.contains("Ethernet") || class.contains("Network")
}
