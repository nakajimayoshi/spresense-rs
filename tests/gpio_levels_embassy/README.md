# gpio_levels_embassy — on-hardware GPIO level test (embassy HAL)

The **embassy-HAL** variant of `tests/gpio_levels`. Same `defmt-test` shape and
same 1.8 V wiring, but driven entirely through `embassy-cxd56` — it depends only
on that crate (chiptool PAC), **not** `cxd56-hal` or `cxd56-pac-svd2rust`. It's a
`harness = false` integration test (in `tests/gpio.rs`) because `defmt-test` only
emits its entry point under `cfg(test)`.

## Wiring (do this first)

CXD5602 GPIO is **1.8 V** — wire to the board's 1.8V rail only, never 3.3/5 V.
Reuses the exact wires from `tests/gpio_levels`.

| Pin | Header | Wire to | Expected |
|-----|--------|---------|----------|
| `P1q_01` / GP_EMMC_DATA3 / D21 | JP2 pin 4 | **1.8V** | High |
| `P1q_00` / GP_EMMC_DATA2 / D20 | JP2 pin 5 | **GND**  | Low  |

No extra wiring is needed for the logger: it rides the existing UART1 CP2102N
USB-serial console (pads `SPI0_CS_X`/`SPI0_SCK`, distinct from the test pins).

## Run

```
cargo test --release --test gpio     # cargo test exit status reflects pass/fail
```

Expected output (decoded by the harness):

```
(1/2) running `data3_high_reads_high`...
(2/2) running `data2_low_reads_low`...
all tests passed!
```

Prereq: `cargo install --path ../../tools/cargo-spresense-flash --force` and a
connected board. See `../README.md` for the harness details.

## How it works

`tests/gpio.rs`'s `#[init]` brings up UART1 with the embassy blocking PL011
driver (`Uart::new`) and installs it as the `defmt-serial` logger, then hands the
two `embassy_cxd56::gpio::Input` pins to the tests as shared state. Unlike the
cxd56-hal version there is **no manual `IO_*` pad poke**: `Input::new(.., Pull::None)`
enables the input buffer (ENZI) and floats the pad itself. A wrong reading panics;
`panic-probe` emits a `panicked at …` frame the harness reports as FAIL. Swap the
two wires to see it fail.

> **Note on `COM_HZ`:** the blocking driver re-initialises the console and
> recomputes the baud from `COM_HZ`. If that value doesn't match the board's real
> COM clock, the serial output is garbled and the harness can't decode it
> (timeout). Validate the value with `examples/rust_hello_uart_embassy` (clean
> greeting at 115200) and keep the two in sync.
