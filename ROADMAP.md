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
- [x] Compiler
- [x] Clocksource (current)
- [x] Available clocksources
- [x] Boot parameters (`/proc/cmdline`)
- [ ] Kernel bits (32/64)
- [ ] Distro (`/etc/os-release`)
- [ ] Desktop environment + version
- [ ] Window manager / compositor
- [ ] Console / tty vs display

## Machine — `inxi -M`
- [ ] Type (laptop / desktop / server, from DMI chassis)
- [ ] System vendor / product / version (`/sys/class/dmi/id/`)
- [ ] Motherboard vendor / model
- [ ] BIOS/UEFI vendor / version / date
- [ ] Serial (behind a filter flag)

## Battery — `inxi -B`
- [ ] Detect batteries (`/sys/class/power_supply/BAT*`)
- [ ] Charge (now / full / percent)
- [ ] Condition (full-capacity vs design)
- [ ] Volts, model, status, cycles

## CPU — `inxi -C`
- [x] Model (brand string)
- [x] Vendor (AMD / Intel abbreviation)
- [x] Cores
- [x] Threads (logical CPUs)
- [x] Stepping
- [x] Model ID (dec + hex)
- [x] Family (dec + hex)
- [ ] Socket / bits / type (MT/MCP)
- [ ] Microcode revision
- [ ] Cache (L1 / L2 / L3)
- [ ] Speed: avg / min / max (`/sys/.../cpufreq` or `/proc/cpuinfo` MHz)
- [ ] Per-core current speed
- [ ] Governor / scaling driver
- [ ] Bogomips
- [ ] Flags / features (`inxi -f`)

## Graphics — `inxi -G`
- [x] Device name
- [x] Device vendor
- [~] Driver (currently hardcoded "Unknown"; read from `/sys/.../uevent`)
- [~] Driver version (unavailable without OpenCL/OpenGL query)
- [ ] Bus ID
- [ ] Multiple GPUs (Device-2, Device-3, …)
- [ ] Display server (X11 / Wayland) + compositor
- [ ] Resolution
- [ ] API: OpenGL / EGL / Vulkan renderer + version
- [ ] Monitor / EDID (`inxi --edid`)

## Audio — `inxi -A`
- [ ] Device name / vendor
- [ ] Driver + bus ID
- [ ] Sound server (ALSA / PipeWire / PulseAudio) + version

## Network — `inxi -N`
- [x] Device (lspci name)
- [x] Interface name (IF-N)
- [x] MAC address
- [x] Active services (NetworkManager / sshd)
- [~] Interface state (up/down — parse currently misses it, see main.rs)
- [ ] Driver / port / bus ID per device
- [ ] Link speed / duplex
- [ ] IP addresses per interface (`inxi -i`)
- [ ] WAN IP (`inxi -i`)

## Drives — `inxi -D`
- [ ] Local storage total / used
- [ ] Per-drive ID (dev, vendor, model, size)
- [ ] Serial, speed, temp
- [ ] SMART status
- [ ] Optical drives (`inxi -d`)

## Partitions — `inxi -P`
- [ ] Mounted partitions (`/proc/mounts` + statvfs)
- [ ] Mount / size / used / percent / fs / dev
- [ ] Labels (`inxi -l`) and UUIDs (`inxi -u`)
- [ ] Unmounted partitions (`inxi -o`)

## Swap — `inxi -j`
- [ ] Swap entries (`/proc/swaps`)
- [ ] Type / size / used / percent / priority / device

## Sensors — `inxi -s`
- [ ] CPU / mobo / GPU temperatures (`/sys/class/hwmon`)
- [ ] Fan speeds

## Memory / RAM — `inxi -m`
- [ ] Total / available / used (`/proc/meminfo`)
- [ ] Per-slot modules (DMI type 17)

## Info — `inxi -I`
- [ ] Process count
- [ ] Uptime (`/proc/uptime`)
- [ ] Memory summary (total / used)
- [ ] Init system + runlevel
- [ ] Installed package count
- [ ] Default shell
- [ ] zinxi version / client

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
- [ ] Argument parsing (per-section flags: `-S -C -G …`)
- [ ] Verbosity levels `-v0`..`-v8`
- [ ] `-F` full report as default
- [ ] Color control (`-c`, `--no-color`, respect `NO_COLOR`)
- [ ] Output when not a TTY (strip ANSI)
- [ ] Filtering of serials/MACs/IPs (`-z`)
- [ ] `--help` / `--version`
