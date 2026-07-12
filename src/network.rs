// Network section — `inxi -N`. Sources: PCI inventory, /sys/class/net, services.

use crate::pci;
use crate::util::{header, print_group, read_trim, run_capture, seg};
use std::io::{self, Write};

pub fn print<W: Write>(out: &mut W, show_mac: bool) -> io::Result<()> {
    header(out, "Network")?;

    // PCI network controllers.
    let devices = pci::devices();
    let mut n = 0;
    for dev in devices.iter().filter(|d| pci::is_network(&d.class)) {
        n += 1;
        let mut line = vec![seg(
            &format!("Device-{n}"),
            format!("{} {}", dev.short_vendor(), dev.device),
        )];
        line.push(seg(
            "driver",
            if dev.driver.is_empty() { "N/A".into() } else { dev.driver.clone() },
        ));
        line.push(seg("bus-ID", &dev.slot));
        line.push(seg("chip-ID", dev.chip_id()));
        print_group(out, 2, 4, &line)?;
    }

    // Interfaces from sysfs.
    print_interfaces(out, show_mac)?;

    // Active services.
    let mut services = Vec::new();
    if service_active("NetworkManager") {
        services.push("NetworkManager");
    }
    if service_active("sshd") {
        services.push("sshd");
    }
    if !services.is_empty() {
        print_group(
            out,
            2,
            4,
            &[crate::util::key("Info"), seg("services", services.join(","))],
        )?;
    }
    Ok(())
}

fn print_interfaces<W: Write>(out: &mut W, show_mac: bool) -> io::Result<()> {
    let Ok(entries) = std::fs::read_dir("/sys/class/net") else {
        return Ok(());
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n != "lo")
        .collect();
    names.sort();

    let mut idx = 0;
    for name in names {
        idx += 1;
        let base = format!("/sys/class/net/{name}");
        let state = read_trim(&format!("{base}/operstate")).unwrap_or_else(|| "unknown".into());
        let mut line = vec![seg(&format!("IF-{idx}"), &name), seg("state", &state)];
        if let Some(speed) = read_trim(&format!("{base}/speed")) {
            // Virtual interfaces report -1; skip those.
            if speed != "-1" && !speed.is_empty() {
                line.push(seg("speed", format!("{speed} Mbps")));
            }
        }
        if show_mac {
            let mac = read_trim(&format!("{base}/address")).filter(|m| !m.is_empty());
            line.push(seg("mac", mac.unwrap_or_else(|| "N/A".into())));
        } else {
            line.push(seg("mac", "<filter>"));
        }
        print_group(out, 2, 4, &line)?;
    }
    Ok(())
}

fn service_active(name: &str) -> bool {
    if let Some(output) = run_capture("systemctl", &["is-active", name])
        && output.trim() == "active"
    {
        return true;
    }
    run_capture("pgrep", &[name]).is_some_and(|o| !o.trim().is_empty())
}
