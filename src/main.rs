// zinxi — an inxi-like system information tool. Pure Rust std, Linux only.

mod ansi;
mod audio;
mod battery;
mod cpu;
mod drives;
mod gpu;
mod info;
mod machine;
mod network;
mod partition;
mod pci;
mod sensors;
mod swap;
mod system;
mod util;

use std::io::{self, BufWriter, Write};

struct Options {
    show_host: bool,
    show_mac: bool,
}

fn main() -> io::Result<()> {
    // Minimal flag handling; -z filters personal data (host, MAC, serials).
    let mut opts = Options {
        show_host: true,
        show_mac: true,
    };
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-z" | "--filter" => {
                opts.show_host = false;
                opts.show_mac = false;
            }
            "--no-host" => opts.show_host = false,
            "-h" | "--help" => {
                println!("Usage: zinxi [-z] [--no-host]");
                return Ok(());
            }
            _ => {}
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    system::print(&mut out, opts.show_host)?;
    machine::print(&mut out)?;
    battery::print(&mut out)?;
    cpu::print(&mut out)?;
    gpu::print(&mut out)?;
    audio::print(&mut out)?;
    network::print(&mut out, opts.show_mac)?;
    drives::print(&mut out)?;
    partition::print(&mut out)?;
    swap::print(&mut out)?;
    sensors::print(&mut out)?;
    info::print(&mut out)?;

    out.flush()
}
