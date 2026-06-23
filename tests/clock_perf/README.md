# clock_perf — low-power operating-point verification

On-hardware test that the APP CPU clock is **correctly set up in both the
low-power and high-power operating points**, and that `Clock::request_perf`
actually takes effect.

## What it checks

The SP804 timer counts at `cpu_baseclk` (HP ≈ 156 MHz, LP ≈ 39 MHz) — a
*perf-dependent* clock. The RTC is a free-running 32.768 kHz counter on the
always-on external crystal, **invariant** across operating points. Counting the
timer against the RTC over a fixed real-time window recovers the *real*
`cpu_baseclk`, which the test compares to the HAL's belief in each mode:

- `[0/4] rtc_alive` — the RTC counter advances (no timebase otherwise).
- `[1/4] hp` — measured ≈ believed `cpu_baseclk`, and `appsmp` ∈ 140..175 MHz.
- `[2/4] lp` — same after `request_perf(Perf::Lp)`; `appsmp` ∈ 30..48 MHz.
- `[3/4] hp_recover` — back to HP; proves the LP→HP round-trip recovers.
- `[4/4] ratio` — measured HP/LP ≈ 4×; physical proof the clock changed.

Ends with `TEST RESULT: PASS`/`FAIL`, which `cargo-spresense-flash --test`
matches to set the process exit code (0 pass / 1 fail / 2 timeout).

> Why not a UART loopback? TX and RX share one baud generator, so an absolute
> baud (clock) error cancels and the loopback passes anyway. An independent,
> perf-invariant witness (the RTC) is what makes the absolute check possible.

## Wiring

**None.** No external jumper. (CXD5602 GPIO is 1.8 V — never wire its pins to
3.3/5 V.)

## Run

```sh
cd tests/clock_perf
cargo run --release   # builds, flashes, runs as a test; exits 0/1/2
```

Expected output (exact figures depend on the XOSC):

```text
clock_perf: low-power operating-point verification
[0/4] rtc_alive: PASS (RTC advanced N ticks)
[1/4] hp: appsmp=156000000 Hz, cpu_baseclk believed=… measured=… Hz -> PASS
[2/4] lp: appsmp=39000000 Hz,  cpu_baseclk believed=… measured=… Hz -> PASS
[3/4] hp_recover: cpu_baseclk believed=… measured=… Hz -> PASS
[4/4] ratio hp/lp (x100) = 400 (expect ~400) -> PASS
TEST RESULT: PASS
```
