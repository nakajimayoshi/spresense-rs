# gpio_levels — on-hardware GPIO level test (Option 2)

`defmt-test` style. Asserts that a GPIO input tied to 1.8V reads High and one
tied to GND reads Low. It's a `harness = false` integration test (in
`tests/gpio.rs`) because `defmt-test` only emits its entry point under
`cfg(test)`.

## Wiring (do this first)

CXD5602 GPIO is **1.8 V** — wire to the board's 1.8V rail only, never 3.3/5 V.

| Pin | Header | Wire to | Expected |
|-----|--------|---------|----------|
| `gp_emmc_data3` / D21 | JP2 pin 4 | **1.8V** | High |
| `gp_emmc_data2` / D20 | JP2 pin 5 | **GND**  | Low  |

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

`tests/gpio.rs`'s `#[init]` brings up the UART1/`defmt-serial` logger, enables
each pad's input buffer (`ENZI`) and floats it (`PDN=PUN=1`) — `into_input()`
only flips `DIR`, so without this the `IN` bit wouldn't follow the pin — then
hands the two `Input` pins to the tests as shared state. A wrong reading panics;
`panic-probe` emits a `panicked at …` frame the harness reports as FAIL. Swap the
two wires to see it fail.
