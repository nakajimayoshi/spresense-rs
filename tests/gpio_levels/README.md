# gpio_levels — on-hardware GPIO level test (Option 2)

`defmt-test` style. Four kinds of check:

* **Input levels** — a GPIO input tied to 1.8V reads High, one tied to GND reads Low.
* **Output loopback** — an output pin shorted to a floating input pin: driving the
  output High/Low makes the input read High/Low, proving the output driver works.
* **Internal pulls** — an unconnected pin reads High with its internal pull-up
  enabled and Low with its pull-down, proving the `IO_*` pad pull config.
* **EXDEVICE interrupts** — the same loopback pin, configured as a GPIO interrupt:
  level `wait_for_high/low` return when the driven level is already present, and
  the PMU edge latch (`is_pending`) catches a rising/falling/any edge driven on
  the line, which `wait_for_rising_edge/falling_edge/any_edge` then consume. These
  are single-threaded, so the stimulus is created before each `wait_*` call.

It's a `harness = false` integration test (in `tests/gpio.rs`) because
`defmt-test` only emits its entry point under `cfg(test)`.

## Wiring (do this first)

CXD5602 GPIO is **1.8 V** — wire to the board's 1.8V rail only, never 3.3/5 V.

| Pin | Header | Wire to | Expected |
|-----|--------|---------|----------|
| `gp_emmc_data3` / D21 | JP2 pin 4 | **1.8V** | High |
| `gp_emmc_data2` / D20 | JP2 pin 5 | **GND**  | Low  |
| `gp_uart2_rts` / D28 ↔ `gp_uart2_cts` / D27 | JP1 pin 4 ↔ pin 5 | **short the two pins together** (jumper) | out High→in High, out Low→in Low |
| `gp_sen_irq_in` / D22 | JP1 pin 12 | **leave unconnected** | pull-up→High, pull-down→Low |

## Run

```
cargo test --release --test gpio     # cargo test exit status reflects pass/fail
```

Expected output (decoded by the harness):

```
(1/12) running `data3_high_reads_high`...
(2/12) running `data2_low_reads_low`...
(3/12) running `output_high_reads_high`...
(4/12) running `output_low_reads_low`...
(5/12) running `pull_up_reads_high`...
(6/12) running `pull_down_reads_low`...
(7/12) running `wait_for_high_returns_when_high`...
(8/12) running `wait_for_low_returns_when_low`...
(9/12) running `rising_edge_latches_and_waits`...
(10/12) running `falling_edge_latches_and_waits`...
(11/12) running `any_edge_latches_and_waits`...
(12/12) running `is_pending_tracks_and_clears`...
all tests passed!
```

Prereq: `cargo install --path ../../tools/cargo-spresense-flash --force` and a
connected board. See `../README.md` for the harness details.

## How it works

`tests/gpio.rs`'s `#[init]` brings up the UART1/`defmt-serial` logger and
configures the pins: `into_floating_input()` enables each input pad's buffer
(`ENZI`) and sets its pull, while `into_output()` drops `DIR` to enable D28's
driver (the output pad needs no `IO_*` config). It then hands the pins to the
tests as shared state — D27 as an `InterruptInput` (EXDEVICE slot 6), which still
reads its level via `is_high`/`is_low`. The level tests assert D21/D20 read their
tied rails; the loopback tests drive D28 High then Low and assert the shorted D27
follows; the pull tests switch the unconnected D22 to pull-up / pull-down via
`set_pull` and assert it reads the pulled level; the interrupt tests drive an edge
on D28 and assert the PMU latch (`is_pending`) catches it and the matching
`wait_*` consumes it. A wrong reading panics; `panic-probe` emits a `panicked at …`
frame the harness reports as FAIL. Swap the D21/D20 wires (or pull the D27↔D28
jumper) to see it fail.
