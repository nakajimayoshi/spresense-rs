# uart_peripheral — on-hardware UART test (Option 1)

Plain-`defmt` style: three sub-tests run in `main`, each logged with
`defmt::println!`, ending with a `TEST RESULT: PASS`/`FAIL` line that
`cargo-spresense-flash --test` matches for the verdict.

| # | Sub-test | Wiring | What it checks |
|---|----------|--------|----------------|
| 1 | `console_uart1` | none | UART1 console + `defmt-serial` come up (reaching the host over defmt *is* the assertion) |
| 2 | `uart2_internal_loopback` | none | UART2 in PL011 loopback (`UARTCR.LBE`): write a byte pattern, read it back, assert equal |
| 3 | `uart2_external_loopback` | jumper **JP1 D01↔D00** | same over the real pads; gated behind `--features external-loopback` |

## Run

```
cargo run --release                                # sub-tests 1 + 2 (no wiring)
cargo run --release --features external-loopback   # + sub-test 3 (jumper D01↔D00)
echo $?                                             # 0 pass / 1 fail / 2 timeout
```

Prereq: `cargo install --path ../../tools/cargo-spresense-flash --force` and a
connected board. See `../README.md` for the harness details.

Sub-test 2 exercises UART2, which lives in the IMG power/clock domain (no other
example uses it). If `Uart2::new` can't bring it up, you'll get
`uart2_internal_loopback: FAIL: Uart2::new failed` and an overall FAIL — the
console sub-test still passes, so the failure is reported, not hung.
