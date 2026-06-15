# {{project-name}}

Bare-metal Rust firmware for the [Sony Spresense](https://developer.spresense.sony-semicon.com/)
(CXD5602), generated from `cxd56-template` and built on
[`cxd56-hal`](https://github.com/bsaintjo/spresense).

It blinks the onboard LED0 (~1 Hz) and prints a counter over UART1 (the USB
serial console, 115200 8N1):

```text
hello from {{project-name}}, n=0
hello from {{project-name}}, n=1
...
```

## Prerequisites

- A Rust toolchain. The pinned `rust-toolchain.toml` installs the
  `thumbv7em-none-eabihf` target automatically on first build.
- The board must already have the **Sony bootloader flashed and the EULA
  accepted** — see the
  [Spresense setup guide](https://developer.spresense.sony-semicon.com/development-guides/?page=sdk_set_up&lang=en#_flashing_bootloader).
  Rust programs will not boot without it.
- The flashing tool (build + package + flash + serial monitor, as a Cargo
  subcommand):

  ```bash
  cargo install --git https://github.com/bsaintjo/spresense.git cargo-spresense-flash
  ```

## Build, flash, and monitor

```bash
cargo run --release
```

This builds the firmware, flashes it (serial port auto-detected; override with
`--port /dev/ttyUSB0` or the `SPRESENSE_PORT` env var), and opens a read-only
serial monitor on UART1. The LED should blink and the `hello from
{{project-name}}` counter should stream by.

Flash without opening the monitor (or without `cargo run`):

```bash
cargo spresense-flash --release            # build + flash
cargo spresense-flash --release --monitor  # build + flash + monitor
```

### Manual flashing (Spresense SDK tools)

```bash
cargo build --release
mkspk -c 2 target/thumbv7em-none-eabihf/release/{{project-name}} nuttx target/{{project-name}}.spk
flash_writer -c /dev/ttyUSB0 target/{{project-name}}.spk
```

Always use `nuttx` as the SPK program header — the bootloader expects that name.

## Pinning cxd56-hal

`Cargo.toml` tracks `cxd56-hal` on the `main` branch, which can change without
notice. For reproducible builds, pin to a specific commit:

```toml
cxd56-hal = { git = "https://github.com/bsaintjo/spresense.git", rev = "<commit-sha>", features = ["rt", "critical-section"] }
```
