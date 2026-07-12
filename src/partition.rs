// Partitions section — `inxi -P`. Source: df (filtered to notable mounts).

use crate::util::{header, human_size, is_pseudo_fs, mounts, percent, print_group, seg};
use std::io::{self, Write};

/// Mount points inxi highlights by default.
const NOTABLE: &[&str] = &["/", "/boot", "/boot/efi", "/home", "/var", "/opt", "/tmp", "/srv"];

pub fn print<W: Write>(out: &mut W) -> io::Result<()> {
    header(out, "Partition")?;

    let mut list: Vec<_> = mounts()
        .into_iter()
        .filter(|m| !is_pseudo_fs(&m.fstype) && NOTABLE.contains(&m.mount.as_str()))
        .collect();
    // Order by NOTABLE priority for stable, inxi-like output.
    list.sort_by_key(|m| NOTABLE.iter().position(|n| *n == m.mount).unwrap_or(usize::MAX));

    for (i, m) in list.iter().enumerate() {
        let line = vec![
            seg(&format!("ID-{}", i + 1), &m.mount),
            seg("size", human_size(m.size)),
            seg(
                "used",
                format!("{} {}", human_size(m.used), percent(m.used, m.size)),
            ),
            seg("fs", &m.fstype),
            seg("dev", &m.dev),
        ];
        print_group(out, 2, 4, &line)?;
    }
    Ok(())
}
