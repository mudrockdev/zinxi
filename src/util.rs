// Shared helpers: file reads, subprocess capture, size formatting, and an
// inxi-style line wrapper.

use crate::ansi::{KEY, RESET};
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Read a file and trim trailing whitespace, or None on failure.
pub fn read_trim(path: &str) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
}

/// Read a file, returning "" on failure.
pub fn read_or_empty(path: &str) -> String {
    read_trim(path).unwrap_or_default()
}

/// Run a command and return captured stdout, or None if it couldn't run.
pub fn run_capture(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd)
        .args(args)
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() && output.stdout.is_empty() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// True if a program is runnable (used to gate optional probes).
pub fn has_command(cmd: &str) -> bool {
    if let Some(path) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path) {
            if dir.join(cmd).is_file() {
                return true;
            }
        }
    }
    false
}

/// Format a byte count with binary units (KiB/MiB/GiB/TiB), inxi-style.
pub fn human_size(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    let b = bytes as f64;
    let (val, unit) = if b >= KIB.powi(4) {
        (b / KIB.powi(4), "TiB")
    } else if b >= KIB.powi(3) {
        (b / KIB.powi(3), "GiB")
    } else if b >= KIB.powi(2) {
        (b / KIB.powi(2), "MiB")
    } else if b >= KIB {
        (b / KIB, "KiB")
    } else {
        return format!("{bytes} B");
    };
    // Whole numbers print without decimals, like inxi (e.g. "8 GiB", "31.26 GiB").
    if (val.fract()).abs() < 0.01 {
        format!("{} {unit}", val.round() as u64)
    } else {
        format!("{val:.2} {unit}")
    }
}

/// Percentage string like inxi: "(28.3%)".
pub fn percent(part: u64, whole: u64) -> String {
    if whole == 0 {
        return "(0.0%)".to_string();
    }
    format!("({:.1}%)", part as f64 / whole as f64 * 100.0)
}

/// A colored "key value" segment, e.g. `Kernel 7.1.3`.
pub fn seg(key: &str, value: impl std::fmt::Display) -> String {
    format!("{KEY}{key}{RESET} {value}")
}

/// A bare colored key with no value (e.g. group headers like `cache`).
pub fn key(key: &str) -> String {
    format!("{KEY}{key}{RESET}")
}

/// Visible width of a string, ignoring ANSI escape sequences.
fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_esc = false;
    for c in s.chars() {
        if in_esc {
            if c == 'm' {
                in_esc = false;
            }
        } else if c == '\x1b' {
            in_esc = true;
        } else {
            len += 1;
        }
    }
    len
}

const WIDTH: usize = 112;

/// Print a logical line of segments, wrapping at WIDTH with a continuation
/// indent — mirrors inxi's wrapped, indented layout.
pub fn print_group<W: Write>(
    out: &mut W,
    first_indent: usize,
    cont_indent: usize,
    segments: &[String],
) -> io::Result<()> {
    let mut col = 0;
    let mut line_started = false;
    let mut indent = first_indent;

    for segment in segments {
        let seg_len = visible_len(segment);
        if !line_started {
            write!(out, "{:indent$}", "", indent = indent)?;
            write!(out, "{segment}")?;
            col = indent + seg_len;
            line_started = true;
        } else if col + 1 + seg_len > WIDTH {
            // wrap
            writeln!(out)?;
            indent = cont_indent;
            write!(out, "{:indent$}", "", indent = indent)?;
            write!(out, "{segment}")?;
            col = indent + seg_len;
        } else {
            write!(out, " {segment}")?;
            col += 1 + seg_len;
        }
    }
    writeln!(out)
}

/// A mounted filesystem, from `df`.
pub struct Mount {
    pub dev: String,
    pub fstype: String,
    pub size: u64,
    pub used: u64,
    pub mount: String,
}

/// Mounted filesystems with sizes in bytes, via `df -B1 -T -P`.
pub fn mounts() -> Vec<Mount> {
    let text = match run_capture("df", &["-B1", "-T", "-P"]) {
        Some(t) => t,
        None => return Vec::new(),
    };
    let mut out = Vec::new();
    for line in text.lines().skip(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        // Filesystem Type 1B-blocks Used Available Capacity Mounted-on
        if cols.len() < 7 {
            continue;
        }
        out.push(Mount {
            dev: cols[0].to_string(),
            fstype: cols[1].to_string(),
            size: cols[2].parse().unwrap_or(0),
            used: cols[3].parse().unwrap_or(0),
            mount: cols[6..].join(" "),
        });
    }
    out
}

/// True for pseudo/virtual filesystems that should not count as real storage.
pub fn is_pseudo_fs(fstype: &str) -> bool {
    matches!(
        fstype,
        "tmpfs"
            | "devtmpfs"
            | "proc"
            | "sysfs"
            | "cgroup"
            | "cgroup2"
            | "overlay"
            | "squashfs"
            | "efivarfs"
            | "ramfs"
            | "autofs"
            | "mqueue"
            | "debugfs"
            | "tracefs"
            | "fusectl"
            | "configfs"
            | "pstore"
            | "bpf"
            | "securityfs"
            | "hugetlbfs"
            | "devpts"
            | "binfmt_misc"
            | "nsfs"
    )
}

/// Print a section header (no indent), e.g. `System:`.
pub fn header<W: Write>(out: &mut W, name: &str) -> io::Result<()> {
    use crate::ansi::{HEADER, RESET};
    writeln!(out, "{HEADER}{name}:{RESET}")
}
