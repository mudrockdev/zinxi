// Port of main.zig — inxi-like system information output.

mod ansi;
mod cpu;
mod gpu;
mod system;

use ansi::{BLUE_BOLD, GREEN_BOLD, RESET};
use std::io::{self, BufWriter, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    print_system(&mut out)?;
    print_machine(&mut out)?;
    print_cpu(&mut out)?;
    print_graphics(&mut out)?;
    print_audio(&mut out)?;
    print_network(&mut out)?;
    get_network_devices(&mut out)?;
    get_interface_info(&mut out)?;
    get_network_services(&mut out)?;
    print_drives(&mut out)?;
    print_partition(&mut out)?;
    print_swap(&mut out)?;
    print_sensors(&mut out)?;
    print_info(&mut out)?;

    out.flush()
}

/// Run a command and return its captured stdout, or None if it couldn't run.
fn run_capture(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd)
        .args(args)
        .stderr(Stdio::null())
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn print_system<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}System:{RESET}")?;

    let info = system::get_system_info();

    write!(out, "  {BLUE_BOLD}Host:{RESET} {}", info.nodename)?;
    write!(out, " {BLUE_BOLD}Kernel:{RESET} {} ", info.release)?;
    writeln!(out, " {BLUE_BOLD}Arch:{RESET} {}", info.machine)?;
    write!(out, "  {BLUE_BOLD}compiler:{RESET} {}", info.compiler_name)?;
    write!(out, " {BLUE_BOLD}clocksource:{RESET} {}", info.clc_current)?;
    writeln!(out, " {BLUE_BOLD}avail:{RESET} {}", info.clc_avail)?;
    writeln!(out, "  {BLUE_BOLD}parameters:{RESET} {}", info.params)?;
    Ok(())
}

fn print_machine<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Machine:{RESET}")
}

fn print_cpu<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}CPU:{RESET}")?;
    write!(out, "  {BLUE_BOLD}Info:{RESET} ")?;

    let info = cpu::get_cpu_info();

    let model = info.brand.trim();
    write!(out, "{BLUE_BOLD}model:{RESET} {model} ")?;

    // AuthenticAMD / GenuineIntel -> short name.
    let vendor = match info.vendor.as_str() {
        "AuthenticAMD" => "AMD",
        "GenuineIntel" => "Intel",
        _ => "Unknown",
    };
    writeln!(out, "{BLUE_BOLD}vendor:{RESET} {vendor}")?;

    write!(out, "  {BLUE_BOLD}Cores:{RESET} {} ", info.cores)?;
    write!(out, "{BLUE_BOLD}Threads:{RESET} {}", info.threads)?;
    writeln!(out, " {BLUE_BOLD}Stepping:{RESET} {}", info.stepping)?;

    write!(
        out,
        "  {BLUE_BOLD}Model ID:{RESET} {} 0x{:x}",
        info.model, info.model
    )?;
    writeln!(
        out,
        " {BLUE_BOLD}Family:{RESET} {} 0x{:x}",
        info.family, info.family
    )?;
    Ok(())
}

fn print_graphics<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Graphics:{RESET}")?;

    let info = gpu::get_gpu_info();
    write!(out, "  {BLUE_BOLD}Device-1:{RESET} ")?;
    write!(out, "{}", info.name)?;
    write!(out, " {BLUE_BOLD}Driver:{RESET} {}", "Unknown")?;
    writeln!(out, " {BLUE_BOLD}v:{RESET} {}", info.version)?;
    writeln!(out, "  {BLUE_BOLD}Vendor:{RESET} {}", info.vendor)?;
    Ok(())
}

fn print_audio<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Audio:{RESET}")
}

fn print_network<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Network:{RESET}")
}

fn print_drives<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Drives:{RESET}")
}

fn print_partition<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Partition:{RESET}")
}

fn print_swap<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Swap:{RESET}")
}

fn print_sensors<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Sensors:{RESET}")
}

fn print_info<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "{BLUE_BOLD}Info:{RESET}")
}

fn get_network_devices<W: Write>(out: &mut W) -> io::Result<()> {
    let Some(content) = run_capture("lspci", &["-v"]) else {
        return Ok(());
    };
    parse_lspci_v_output(out, &content)
}

fn parse_lspci_v_output<W: Write>(out: &mut W, content: &str) -> io::Result<()> {
    let mut device_count = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let starts_with_digit = trimmed.as_bytes()[0].is_ascii_digit();
        if starts_with_digit
            && (trimmed.contains("Ethernet controller") || trimmed.contains("Network controller"))
        {
            device_count += 1;

            let device_name = if let Some(pos) = trimmed.find("Ethernet controller:") {
                trimmed[pos + "Ethernet controller:".len()..].trim()
            } else if let Some(pos) = trimmed.find("Network controller:") {
                trimmed[pos + "Network controller:".len()..].trim()
            } else if let Some(pos) = trimmed.find(": ") {
                trimmed[pos + 2..].trim()
            } else {
                trimmed
            };

            writeln!(
                out,
                "  {GREEN_BOLD}Device-{device_count}:{RESET} {device_name}"
            )?;
        }
    }
    Ok(())
}

fn get_interface_info<W: Write>(out: &mut W) -> io::Result<()> {
    let Some(content) = run_capture("ip", &["addr", "show"]) else {
        return Ok(());
    };
    parse_interface_info(out, &content)
}

fn parse_interface_info<W: Write>(out: &mut W, content: &str) -> io::Result<()> {
    let mut interface_count = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.as_bytes()[0].is_ascii_digit() {
            // Lines like "2: eth0: <...>" — take the interface name.
            let mut parts = trimmed.split(':');
            parts.next(); // skip the index
            if let Some(iface_name) = parts.next() {
                let clean_name = iface_name.trim();
                if clean_name.starts_with("lo") {
                    continue; // skip loopback
                }

                interface_count += 1;
                if interface_count == 1 {
                    write!(out, "  {BLUE_BOLD}IF-1:{RESET} {clean_name}")?;
                } else {
                    write!(out, "\n  {BLUE_BOLD}IF-{interface_count}:{RESET} {clean_name}")?;
                }
            }
        } else if interface_count == 1 {
            // Detail lines for the first interface.
            if trimmed.contains("state UP") {
                write!(out, " {BLUE_BOLD}state:{RESET} up")?;
            } else if trimmed.contains("state DOWN") {
                write!(out, " {BLUE_BOLD}state:{RESET} down")?;
            } else if trimmed.contains("link/ether") {
                let mut link_parts = trimmed.split(' ');
                link_parts.next(); // "link/ether"
                if let Some(mac) = link_parts.next() {
                    write!(out, " {BLUE_BOLD}mac:{RESET} {}", mac.trim())?;
                }
            }
        }
    }

    writeln!(out)
}

fn get_network_services<W: Write>(out: &mut W) -> io::Result<()> {
    write!(out, "  {BLUE_BOLD}Info:{RESET} ")?;

    let mut services: Vec<&str> = Vec::new();
    if service_active("NetworkManager") {
        services.push("NetworkManager");
    }
    if service_active("sshd") {
        services.push("sshd");
    }

    print_services(&services, out)
}

/// True if a systemd service is active, falling back to a process check.
fn service_active(name: &str) -> bool {
    if let Some(output) = run_capture("systemctl", &["is-active", name]) {
        if output.trim() == "active" {
            return true;
        }
    }
    // Fallback: is any process by that name running?
    run_capture("pgrep", &[name]).is_some_and(|o| !o.trim().is_empty())
}

fn print_services<W: Write>(services: &[&str], out: &mut W) -> io::Result<()> {
    write!(out, "{BLUE_BOLD}services:{RESET} ")?;
    for (i, service) in services.iter().enumerate() {
        if i > 0 {
            write!(out, ",")?;
        }
        write!(out, "{service}")?;
    }
    writeln!(out)
}
