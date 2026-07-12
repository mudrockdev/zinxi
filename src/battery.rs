// Battery section — `inxi -B`. Source: /sys/class/power_supply/BAT*.
// Prints nothing when no battery is present (matches inxi's default -F).

use crate::util::{header, print_group, read_trim, seg};
use std::io::{self, Write};

fn batteries() -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
        for e in entries.flatten() {
            let name = e.file_name();
            if name.to_string_lossy().starts_with("BAT") {
                out.push(e.path());
            }
        }
    }
    out.sort();
    out
}

fn read_micro(path: &str) -> Option<f64> {
    read_trim(path)
        .and_then(|s| s.parse::<f64>().ok())
        .map(|v| v / 1_000_000.0)
}

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    let bats = batteries();
    if bats.is_empty() {
        return Ok(());
    }
    header(out, "Battery")?;

    for (i, bat) in bats.iter().enumerate() {
        let p = |f: &str| bat.join(f).to_string_lossy().into_owned();

        let mut line = vec![seg(&format!("ID-{}", i + 1), "battery")];
        if let Some(cap) = read_trim(&p("capacity")) {
            line.push(seg("charge", format!("{cap}%")));
        }
        if let Some(v) = read_micro(&p("voltage_now")) {
            line.push(seg("volts", format!("{v:.1}")));
        }
        if let Some(model) = read_trim(&p("model_name")).filter(|s| !s.is_empty()) {
            line.push(seg("model", model));
        }
        if let Some(status) = read_trim(&p("status")) {
            line.push(seg("status", status));
        }
        print_group(out, 2, 4, &line)?;
    }
    Ok(())
}
