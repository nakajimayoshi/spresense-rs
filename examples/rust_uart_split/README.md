# UART Split Example

This example builds the UART1 driver, then `split()`s it into independent
transmit and receive halves and runs a byte-at-a-time echo. The receive half
(`UartRx`) implements only `embedded_io::Read`, and the transmit half
(`UartTx`) only `embedded_io::Write`, so the two directions are owned by
separate values — typically one would hand the RX half to an interrupt handler
while the main loop keeps the TX half.

The UART stays enabled across the split; the halves can later be recombined with
`Uart::join`, after which `Uart::free` reclaims the GPIO pins and gates the clock.

## Build

```bash
cd examples/rust_uart_split
cargo build --release
```

## Flash

```bash
cargo run --release
```

## Test

Connect to the serial console on `/dev/ttyUSB1` at 115200 baud:

```bash
minicom -D /dev/ttyUSB1 -b 115200
```

On reset the board prints `uart split echo ready`. Every character you type is
read through the RX half and echoed back through the TX half.
