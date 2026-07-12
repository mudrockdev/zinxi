# AGENTS.md

Guidance for AI agents (and humans) working in this repo. `CLAUDE.md` is a
symlink to this file.

## What this is

`zinxi` is a Rust rewrite of an inxi-like system information tool for Linux.

## Hard constraints

- **Rust standard library only. No third-party crates.** `[dependencies]` in
  `Cargo.toml` must stay empty. If a task seems to need a crate, get the data
  from `/proc`, `/sys`, `/dev`, or by shelling out to a standard CLI tool via
  `std::process::Command` instead.
- **Linux-only** is fine; prefer kernel interfaces (`/proc`, `/sys`) over
  subprocesses, and fall back to subprocesses (`lspci`, `ip`, `systemctl`,
  `lsblk`, …) only when there is no clean file-based source.
- Missing data must degrade gracefully (print "Unknown" / empty, never panic).

## Feature reference

The target behavior is the bundled at `example` folder. To see what a section

## The roadmap is the source of truth

[`ROADMAP.md`](ROADMAP.md) is the authoritative checklist of features. When you
work on this project:

1. Pick an unchecked (`[ ]`) or partial (`[~]`) item from `ROADMAP.md`.
2. Implement it.
3. **Verify it actually works** — run `cargo run` and confirm the section
   produces correct output on this machine (compare against `example/inxi`).
4. Only then, in the **same change**, update the checkbox:
   - `[x]` when implemented and verified working.
   - `[~]` when partially done or using a placeholder value (note why).
   - Leave `[ ]` if it isn't working yet.
5. Never check a box for code you haven't run and observed working. A green
   `cargo build` is not enough — the output must be correct.

If you add a feature that isn't listed, add a checkbox for it in the right
section first, then check it once verified.

## Project layout

- `src/main.rs` — entry point, output/printing, subprocess orchestration.
- `src/ansi.rs` — ANSI color escape sequences.
- `src/system.rs` — System section (`/proc/sys/kernel`, sysfs clocksource).
- `src/cpu.rs` — CPU section (parses `/proc/cpuinfo`).
- `src/gpu.rs` — Graphics section (parses `lspci -mm`).
- New sections should get their own `src/<section>.rs` module, added with
  `mod <section>;` in `main.rs`.

## Build & run

```bash
cargo run              # debug build + run
cargo build --release  # optimized binary at target/release/zinxi
cargo check            # fast typecheck
cargo clippy           # lint (if available)
```

## Conventions

- Match the existing print style: `"  {LABEL_BOLD}Field:{RESET} value"`, two
  leading spaces for sub-fields, section headers with no indent.
- Keep each section in a function returning `io::Result<()>` and take a generic
  `W: Write` writer, as the existing sections do.
