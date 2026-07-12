// Machine section — `inxi -M`. Source: /sys/class/dmi/id/.

use crate::util::{header, print_group, read_trim, seg};
use std::io::{self, Write};

fn dmi(field: &str) -> Option<String> {
    read_trim(&format!("/sys/class/dmi/id/{field}"))
        .filter(|s| !s.is_empty() && s != "To be filled by O.E.M.")
}

/// Map SMBIOS chassis type numbers to a human label (subset inxi uses).
fn chassis_type() -> &'static str {
    let code = dmi("chassis_type").and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    match code {
        3..=7 => "Desktop",
        8..=10 | 14 => "Laptop",
        11 => "Hand Held",
        13 => "All in One",
        17 | 23 | 25 => "Server",
        30..=32 => "Notebook",
        _ => "N/A",
    }
}

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "Machine")?;

    // Prefer motherboard fields (inxi shows Mobo on desktops).
    let mut line = vec![seg("Type", chassis_type())];

    let mobo_vendor = dmi("board_vendor");
    let mobo_name = dmi("board_name");
    if let Some(v) = &mobo_vendor {
        line.push(seg("Mobo", v));
    }
    if let Some(m) = &mobo_name {
        line.push(seg("model", m));
    }
    if let Some(v) = dmi("board_version") {
        line.push(seg("v", v));
    }
    line.push(seg("serial", dmi("board_serial").unwrap_or_else(|| "<superuser required>".into())));
    print_group(out, 2, 4, &line)?;

    // Firmware / UEFI line.
    let mut fw = vec![seg("Firmware", "UEFI")];
    if let Some(v) = dmi("bios_vendor") {
        fw.push(seg("vendor", v));
    }
    if let Some(v) = dmi("bios_version") {
        fw.push(seg("v", v));
    }
    if let Some(d) = dmi("bios_date") {
        fw.push(seg("date", d));
    }
    print_group(out, 4, 4, &fw)?;
    Ok(())
}
