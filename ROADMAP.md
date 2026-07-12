# zinxi Roadmap

Feature checklist for the Rust port. The reference implementation is
[`example`](example).

**Rules for this file** — see [AGENTS.md](AGENTS.md):
- `[x]` = implemented **and** verified working via `cargo run`.
- `[~]` = partially implemented / placeholder value.
- `[ ]` = not started.

Legend maps each section to its inxi flag, e.g. Graphics = `inxi -G`.

---

## System — `inxi -S`
- [x] Host (nodename)
- [x] Kernel (release)
- [x] Arch (machine)
- [x] Kernel bits (32/64)
- [x] Compiler + version
- [x] Clocksource (current)
- [x] Available clocksources
- [x] Boot parameters (`/proc/cmdline`)
- [x] Distro (`/etc/os-release`)
- [x] Display manager (dm)
- [~] Desktop environment (name from `XDG_CURRENT_DESKTOP`; version not detected)
- [ ] Window manager / compositor
- [ ] Toolkit (tk)
- [ ] Console / tty vs display

## Machine — `inxi -M`
- [x] Type (chassis type from DMI)
- [x] Motherboard vendor / model / version
- [x] BIOS/UEFI vendor / version / date
- [~] Serial (needs root — prints `<superuser required>`)
- [ ] System (OEM product/version, for prebuilt/laptops)

## Battery — `inxi -B`
- [~] Detect batteries (skips cleanly when none; **unverified** — no battery on
      this machine to confirm field output)
- [~] Charge / volts / model / status (coded, not yet observed working)
- [ ] Condition (full-capacity vs design)
- [ ] Cycles

## CPU — `inxi -C`
- [x] Model (brand string, cleaned)
- [x] Cores
- [x] Threads (logical CPUs)
- [x] Bits
- [x] Type (MT / MCP)
- [x] Family (dec + hex)
- [x] Model ID (dec + hex)
- [x] Stepping
- [x] Microcode revision
- [x] Cache (L1 / L2 / L3)
- [x] Speed: avg / min / max
- [x] Per-core current speed
- [x] Bogomips
- [~] Flags (basic subset shown; full `inxi -f` list not)
- [ ] Socket
- [ ] Arch label (e.g. "Zen 3+")
- [ ] Governor / scaling driver
- [ ] Boost enabled/disabled

## Graphics — `inxi -G`
- [x] Device name
- [x] Device vendor
- [x] Driver (real DRM driver from sysfs)
- [x] Bus ID
- [x] Chip ID
- [x] Multiple GPUs (Device-2, Device-3, …)
- [~] Display server (session type only; no X.org/compositor/version)
- [ ] Resolution
- [ ] API: OpenGL / EGL / Vulkan renderer + version
- [ ] Monitor / EDID (`inxi --edid`)

## Audio — `inxi -A`
- [x] Device name / vendor
- [x] Driver + bus ID + chip ID
- [x] Sound server (PipeWire / PulseAudio) + version
- [ ] Multiple servers with sub-status (pipewire-pulse, wireplumber, …)

## Network — `inxi -N`
- [x] Device (PCI name / vendor)
- [x] Driver / bus ID / chip ID
- [x] Interface name (IF-N)
- [x] Interface state (up/down/unknown)
- [x] Link speed
- [x] MAC address (with `-z` filter)
- [x] Active services (NetworkManager / sshd)
- [ ] Duplex
- [ ] Port
- [ ] IP addresses per interface (`inxi -i`)
- [ ] WAN IP (`inxi -i`)

## Drives — `inxi -D`
- [x] Local storage total / used
- [x] Per-drive ID (dev, vendor, model, size)
- [ ] Serial (root/smartctl)
- [ ] Speed / lanes (NVMe/SATA)
- [ ] Temperature (hwmon/nvme)
- [ ] SMART status
- [ ] Optical drives (`inxi -d`)

## Partitions — `inxi -P`
- [x] Notable mounts (`/`, `/boot`, `/home`, …)
- [x] Mount / size / used / percent / fs / dev
- [ ] Labels (`inxi -l`) and UUIDs (`inxi -u`)
- [ ] Unmounted partitions (`inxi -o`)
- [ ] Full mounted list (`inxi -p`)

## Swap — `inxi -j`
- [x] Swap entries (`/proc/swaps`)
- [x] Type / size / used / percent / priority / device

## Sensors — `inxi -s`
- [x] CPU temperature (`/sys/class/hwmon`)
- [x] Fan speeds
- [~] Motherboard temp (best effort; often N/A without driver)
- [ ] GPU temperature line

## Memory / RAM — `inxi -m`
- [x] Total / available / used (shown in Info line)
- [ ] Standalone `-m` section
- [ ] Per-slot modules (DMI type 17, needs root)

## Info — `inxi -I`
- [x] Process count
- [x] Uptime (`/proc/uptime`)
- [x] Memory summary (total / available / used)
- [x] Init system + version + default target
- [x] Package count (rpm / dpkg / pacman)
- [x] Compilers (gcc / clang)
- [x] Default shell (parent process)
- [x] zinxi version
- [ ] Flatpak / snap package counts
- [ ] Client detection

---

## Extended sections (later)
- [ ] USB devices — `inxi -J`
- [ ] Bluetooth — `inxi -E`
- [ ] RAID — `inxi -R`
- [ ] Logical devices (LVM / LUKS) — `inxi -L`
- [ ] Repositories — `inxi -r`
- [ ] Processes (top CPU/mem) — `inxi -t`
- [ ] Weather — `inxi -w`

## CLI & infrastructure
- [~] Argument parsing (only `-z`, `--no-host`, `--help`; no per-section flags)
- [x] `--help`
- [x] `-z` / `--filter` (filters host + MAC)
- [ ] Per-section flags (`-S -C -G …`) and `-F` selection
- [ ] Verbosity levels `-v0`..`-v8`
- [ ] `--version`
- [ ] Color control (`-c`, `--no-color`, respect `NO_COLOR`)
- [ ] Strip ANSI when output is not a TTY
- [ ] Filter serials / IPs (extend `-z`)
- [ ] inxi-exact line wrapping at terminal width
