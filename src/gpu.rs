// Graphics section — `inxi -G`. Sources: PCI inventory, sysfs DRM, env vars.

use crate::pci;
use crate::util::{header, print_group, read_trim, seg};
use std::io::{self, Write};

/// Read the DRM driver bound to a PCI slot, e.g. "xe" / "amdgpu".
fn drm_driver(slot: &str) -> Option<String> {
    // /sys/bus/pci/devices/0000:<slot>/driver -> ../../../bus/pci/drivers/<name>
    let path = format!("/sys/bus/pci/devices/0000:{slot}/driver");
    let target = std::fs::read_link(&path).ok()?;
    target.file_name().map(|n| n.to_string_lossy().into_owned())
}

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "Graphics")?;

    let devices = pci::devices();
    let mut n = 0;
    for dev in devices.iter().filter(|d| pci::is_display(&d.class)) {
        n += 1;
        let mut line = vec![seg(
            &format!("Device-{n}"),
            format!("{} {}", dev.short_vendor(), dev.device),
        )];
        let driver = drm_driver(&dev.slot).unwrap_or_else(|| {
            if dev.driver.is_empty() {
                "N/A".into()
            } else {
                dev.driver.clone()
            }
        });
        line.push(seg("driver", driver));
        line.push(seg("v", "kernel"));
        line.push(seg("bus-ID", &dev.slot));
        line.push(seg("chip-ID", dev.chip_id()));
        print_group(out, 2, 4, &line)?;
    }
    if n == 0 {
        print_group(out, 2, 4, &[seg("Device", "N/A")])?;
    }

    // Display server / session type from environment.
    let mut disp = Vec::new();
    if let Ok(session) = std::env::var("XDG_SESSION_TYPE")
        && !session.is_empty()
    {
        disp.push(seg("Display", session));
    }
    if let Ok(res) = read_resolution() {
        disp.push(seg("resolution", res));
    }
    if !disp.is_empty() {
        print_group(out, 2, 4, &disp)?;
    }
    Ok(())
}

/// Current framebuffer resolution from sysfs (best effort, no X needed).
fn read_resolution() -> Result<String, ()> {
    if let Some(size) = read_trim("/sys/class/graphics/fb0/virtual_size") {
        // Format "2560,1440" -> "2560x1440".
        if let Some((w, h)) = size.split_once(',') {
            return Ok(format!("{w}x{h}"));
        }
    }
    Err(())
}
