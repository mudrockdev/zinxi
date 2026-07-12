// Sensors section — `inxi -s`. Source: /sys/class/hwmon.

use crate::util::{header, print_group, read_trim, seg};
use std::io::{self, Write};

const CPU_CHIPS: &[&str] = &["k10temp", "zenpower", "coretemp", "cpu_thermal"];

fn read_milli(path: &str) -> Option<f64> {
    read_trim(path)
        .and_then(|s| s.parse::<f64>().ok())
        .map(|v| v / 1000.0)
}

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "Sensors")?;

    let mut cpu_temp: Option<f64> = None;
    let mut mobo_temp: Option<f64> = None;
    let mut fans: Vec<u64> = Vec::new();

    if let Ok(entries) = std::fs::read_dir("/sys/class/hwmon") {
        let mut dirs: Vec<_> = entries.flatten().map(|e| e.path()).collect();
        dirs.sort();
        for dir in dirs {
            let name = read_trim(&dir.join("name").to_string_lossy()).unwrap_or_default();

            if CPU_CHIPS.contains(&name.as_str()) && cpu_temp.is_none() {
                cpu_temp = read_milli(&dir.join("temp1_input").to_string_lossy());
            }
            if name == "acpitz" && mobo_temp.is_none() {
                mobo_temp = read_milli(&dir.join("temp1_input").to_string_lossy());
            }

            // Collect fan inputs from any chip.
            for i in 1..=8 {
                let p = dir.join(format!("fan{i}_input"));
                if let Some(rpm) = read_trim(&p.to_string_lossy()).and_then(|s| s.parse::<u64>().ok()) {
                    fans.push(rpm);
                }
            }
        }
    }

    let mut temps = vec![crate::util::key("System Temperatures")];
    temps.push(seg("cpu", cpu_temp.map_or("N/A".to_string(), |t| format!("{t:.1} C"))));
    temps.push(seg("mobo", mobo_temp.map_or("N/A".to_string(), |t| format!("{t:.1} C"))));
    print_group(out, 2, 4, &temps)?;

    if fans.is_empty() {
        print_group(out, 2, 4, &[seg("Fan Speeds (rpm)", "N/A")])?;
    } else {
        let mut line = vec![crate::util::key("Fan Speeds (rpm)")];
        for (i, rpm) in fans.iter().enumerate() {
            line.push(seg(&format!("fan-{}", i + 1), rpm));
        }
        print_group(out, 2, 4, &line)?;
    }
    Ok(())
}
