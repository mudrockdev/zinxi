// Audio section — `inxi -A`. Sources: PCI inventory + sound-server detection.

use crate::pci;
use crate::util::{header, has_command, print_group, run_capture, seg};
use std::io::{self, Write};

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "Audio")?;

    let devices = pci::devices();
    let mut n = 0;
    for dev in devices.iter().filter(|d| pci::is_audio(&d.class)) {
        n += 1;
        let name = if dev.device.is_empty() {
            dev.short_vendor()
        } else {
            format!("{} {}", dev.short_vendor(), dev.device)
        };
        let mut line = vec![seg(&format!("Device-{n}"), name)];
        line.push(seg(
            "driver",
            if dev.driver.is_empty() { "N/A".into() } else { dev.driver.clone() },
        ));
        line.push(seg("bus-ID", &dev.slot));
        line.push(seg("chip-ID", dev.chip_id()));
        print_group(out, 2, 4, &line)?;
    }
    if n == 0 {
        print_group(out, 2, 4, &[seg("Device", "N/A")])?;
    }

    // Sound server(s).
    if let Some(server) = detect_sound_server() {
        print_group(out, 2, 4, &[seg("API", "ALSA"), seg("Server", server)])?;
    }
    Ok(())
}

/// Best-effort sound-server detection with version, via CLI probes.
fn detect_sound_server() -> Option<String> {
    if has_command("pipewire") {
        let ver = run_capture("pipewire", &["--version"])
            .and_then(|o| parse_version(&o))
            .unwrap_or_default();
        return Some(format!("PipeWire {ver}").trim().to_string());
    }
    if has_command("pulseaudio") {
        let ver = run_capture("pulseaudio", &["--version"])
            .and_then(|o| o.split_whitespace().nth(1).map(str::to_string))
            .unwrap_or_default();
        return Some(format!("PulseAudio {ver}").trim().to_string());
    }
    None
}

fn parse_version(text: &str) -> Option<String> {
    text.split_whitespace()
        .find(|t| t.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .map(str::to_string)
}
