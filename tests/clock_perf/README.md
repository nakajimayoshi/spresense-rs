# clock_perf — operating-point round-trip verification

On-hardware test that `Clock::request_perf` reaches a correct, **in-spec**
operating point in **both** directions via the multi-step SYSIOP FREQLOCK
handshake (the fix: ack every `CLK_CHG_START`/`CLK_CHG_END` pair — 3 each way on
CXD5602 — and complete on the trailing `FREQLOCK` reply, not the first
`CLK_CHG_END`).

## What it checks

The SP804 timer counts at `cpu_baseclk` (a *perf-dependent* clock). The RTC is a
free-running 32.768 kHz counter on the always-on crystal, **invariant** across
operating points. Counting the timer against the RTC over a fixed real-time
window recovers the *real* `cpu_baseclk`, compared to the HAL's belief at each
point. Because the LP console runs at a different COM than HP, **all measurement
happens with no printing** (captured to RAM); the verdict is printed once over
the restored-HP console.

- `[1] hp_boot`   — measured ≈ believed `cpu_baseclk` at boot (HP).
- `[2] lp`        — same after `request_perf(Lp)` (the downshift took, and the
  readback matches reality at LP).
- `[3] cache`     — after `request_perf(Lp)`, the cached `clock.com` (the `Fixed`
  field `uart_alt` reads) equals live `freeze().com` (the `resample_dyn` refresh).
- `[4] hp_recover`— measured ≈ believed back at HP (the LP→HP round-trip).
- `[5] changed`   — LP `cpu_baseclk` is clearly below HP's (physical proof the
  clock moved, not just the readback).

Ends with `TEST RESULT: PASS`/`FAIL`, which `cargo-spresense-flash --test`
matches to set the process exit code (0 pass / 1 fail / 2 timeout).

> Why measure at LP but print at HP? A UART sized for one COM garbles at another,
> and `defmt_serial` returns before bytes leave the FIFO — so printing across a
> perf change corrupts the line and desyncs the decoder. Measuring into RAM and
> reporting once at HP sidesteps both.

## Wiring

**None.** No external jumper. (CXD5602 GPIO is 1.8 V — never wire its pins to
3.3/5 V.)

## Run

```sh
cd tests/clock_perf
cargo run --release                    # the verification test; exits 0/1/2
cargo run --release --bin clock_dump   # diagnostic: raw register/clock dump + FIFO message log
```

Expected output (exact figures depend on the XOSC; APP ~156 MHz HV, ~31.2 MHz LV):

```text
clock_perf: request_perf operating-point round-trip (HP->LP->HP)
[1] hp_boot:    cpu_base believed=… measured=… -> PASS
[2] lp:         cpu_base believed=… measured=… -> PASS
[3] cache:      cached_com=… live_com=… -> PASS
[4] hp_recover: cpu_base believed=… measured=… -> PASS
[5] changed:    lp=… < hp/2=… -> PASS
TEST RESULT: PASS
```
