// Swap section — `inxi -j`. Source: /proc/swaps.

use crate::util::{header, human_size, percent, print_group, seg};
use std::io::{self, Write};

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "Swap")?;

    let content = std::fs::read_to_string("/proc/swaps").unwrap_or_default();
    for (i, line) in content.lines().skip(1).enumerate() {
        // Filename  Type  Size  Used  Priority   (Size/Used are in KiB)
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 5 {
            continue;
        }
        let dev = cols[0];
        let ptype = cols[1]; // "partition" or "file"
        let size = cols[2].parse::<u64>().unwrap_or(0) * 1024;
        let used = cols[3].parse::<u64>().unwrap_or(0) * 1024;
        let prio = cols[4];

        // zram devices report as partitions but are compressed RAM.
        let ptype = if dev.contains("zram") { "zram" } else { ptype };

        let line = vec![
            seg(&format!("ID-{}", i + 1), format!("swap-{}", i + 1)),
            seg("type", ptype),
            seg("size", human_size(size)),
            seg("used", format!("{} {}", human_size(used), percent(used, size))),
            seg("priority", prio),
            seg("dev", dev),
        ];
        print_group(out, 2, 4, &line)?;
    }
    Ok(())
}
