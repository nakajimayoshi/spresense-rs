# gpio_levels — on-hardware GPIO level test (Option 2)

`defmt-test` style. Two kinds of check:

* **Input levels** — a GPIO input tied to 1.8V reads High, one tied to GND reads Low.
* **Output loopback** — an output pin shorted to a floating input pin: driving the
  output High/Low makes the input read High/Low, proving the output driver works.

It's a `harness = false` integration test (in `tests/gpio.rs`) because
`defmt-test` only emits its entry point under `cfg(test)`.

## Wiring (do this first)

CXD5602 GPIO is **1.8 V** — wire to the board's 1.8V rail only, never 3.3/5 V.

| Pin | Header | Wire to | Expected |
|-----|--------|---------|----------|
| `gp_emmc_data3` / D21 | JP2 pin 4 | **1.8V** | High |
| `gp_emmc_data2` / D20 | JP2 pin 5 | **GND**  | Low  |
| `gp_uart2_rts` / D28 ↔ `gp_uart2_cts` / D27 | JP1 pin 4 ↔ pin 5 | **short the two pins together** (jumper) | out High→in High, out Low→in Low |

## Run

```
cargo test --release --test gpio     # cargo test exit status reflects pass/fail
```

Expected output (decoded by the harness):

```
(1/4) running `data3_high_reads_high`...
(2/4) running `data2_low_reads_low`...
(3/4) running `output_high_reads_high`...
(4/4) running `output_low_reads_low`...
all tests passed!
```

Prereq: `cargo install --path ../../tools/cargo-spresense-flash --force` and a
connected board. See `../README.md` for the harness details.

## How it works

`tests/gpio.rs`'s `#[init]` brings up the UART1/`defmt-serial` logger, enables
each input pad's input buffer (`ENZI`) and floats it (`PDN=PUN=1`) — `into_input()`
only flips `DIR`, so without this the `IN` bit wouldn't follow the pin — and
configures D28 as an output (`into_output`, which drops `DIR` to enable the
driver; the output pad needs no `IO_*` config). It then hands the `Input`/`Output`
pins to the tests as shared state. The level tests assert D21/D20 read their tied
rails; the loopback tests drive D28 High then Low and assert the shorted D27
follows. A wrong reading panics; `panic-probe` emits a `panicked at …` frame the
harness reports as FAIL. Swap the D21/D20 wires (or pull the D27↔D28 jumper) to
see it fail.
