# UART Echo Example

This example reads up to 256 bytes from UART1 and echoes them back to the host.

## Build

```bash
cd examples/rust_echo
cargo build --release
```

## Flash

Use your preferred flashing tool to write the binary to the Spresense board:

```bash
# Example using probe-run or similar
cargo run --release
```

## Test

Connect to the serial console on `/dev/ttyUSB1` at 115200 baud:

```bash
minicom -D /dev/ttyUSB1 -b 115200
```

Type characters — each will be echoed back. Send multiple characters rapidly; the example will buffer up to 256 bytes and echo them all at once.
