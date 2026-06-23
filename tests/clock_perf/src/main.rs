//! On-hardware low-power operating-point verification (plain `defmt` + sentinel).
//!
//! Proves that [`Clock::request_perf`] actually changes the APP CPU clock *and*
//! that the HAL's clock model matches physical reality at each operating point —
//! the property every perf-dependent peripheral (UART2 baud, SPI gears, the
//! SP804 timer/watchdog) silently relies on.
//!
//! # How it works
//!
//! The thing-under-test is the **SP804 timer**, which counts at `cpu_baseclk`
//! (HP ≈ 156 MHz, LP ≈ 39 MHz) — a *perf-dependent* clock. The independent
//! witness is the **RTC**, a free-running 32.768 kHz counter on the always-on
//! external crystal that does **not** change with the operating point. Reading
//! the SP804 against the RTC over a fixed real-time window yields the *real*
//! `cpu_baseclk`, which we compare to the HAL's belief.
//!
//! A UART self-loopback can't do this: TX and RX share one baud generator, so an
//! absolute frequency error cancels and the loopback passes at the wrong clock.
//! An independent, perf-invariant timebase (the RTC) is what makes these
//! *absolute* checks possible.
//!
//! Sub-tests, each logged over UART1 (which runs on the perf-invariant COM clock,
//! so the console survives LP); ends with `TEST RESULT: PASS`/`FAIL`:
//!
//!   [0/4] rtc_alive   — the RTC counter advances (else there is no timebase).
//!   [1/4] hp          — measured cpu_baseclk ≈ believed, appsmp in the HP band.
//!   [2/4] lp          — same after dropping to Perf::Lp.
//!   [3/4] hp_recover  — back to HP: proves the LP→HP round-trip recovers.
//!   [4/4] ratio       — measured HP/LP ≈ 4×: physical proof the clock changed.
//!
//! No external jumper. CXD5602 GPIO is 1.8 V.

#![no_std]
#![no_main]

use core::ops::RangeInclusive;

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_serial as _;
use panic_probe as _;
use static_cell::StaticCell;

use cxd56_hal::clocks::{Clock, Config, Perf, RccExt};
use cxd56_hal::pac;
use cxd56_hal::timer::{Prescaler, Timer};
use cxd56_hal::uart::{Uart1, UartConfig};

static SERIAL: StaticCell<Uart1> = StaticCell::new();

/// RTC frequency (Hz) — the external 32.768 kHz crystal, perf-invariant.
const RTC_HZ: u64 = 32_768;
/// Measurement window: 1/4 s. At HP the SP804 down-counts ≈ 39e6 ticks in this
/// span — far below `u32::MAX`, so the free-running counter never wraps — while
/// the RTC quantization is ≈ 0.012 %.
const WINDOW_TICKS: u64 = RTC_HZ / 4;

/// Free-running RTC tick count: `(RTPOSTCNT << 15) | RTPRECNT`, mirroring NuttX
/// `cxd56_rtc.c:509-510`. RTPRECNT is the 15-bit (0..32767) sub-second counter;
/// RTPOSTCNT is the seconds counter. Re-reads RTPOSTCNT to discard a sample torn
/// across the 1 Hz carry.
fn rtc_ticks(rtc: &pac::Rtc0) -> u64 {
    loop {
        let hi0 = rtc.rtpostcnt().read().bits();
        let lo = rtc.rtprecnt().read().bits() & 0x7fff;
        let hi1 = rtc.rtpostcnt().read().bits();
        if hi0 == hi1 {
            return ((hi0 as u64) << 15) | lo as u64;
        }
    }
}

/// Measure the *real* `cpu_baseclk` (Hz) by counting SP804 ticks over a fixed
/// RTC window. Borrows `clock` only for the duration of the call and hands the
/// timer token back so the caller can change the operating point afterwards
/// (`request_perf` needs `&mut Clock`, which a live `Timer` would block).
fn measure(clock: &Clock, tok: pac::Timer0, rtc: &pac::Rtc0) -> (u32, pac::Timer0) {
    let mut timer = Timer::new(tok, clock).expect("cpu base clock unavailable");
    timer.start_free_running(Prescaler::Div1); // down-counts at cpu_baseclk

    let r0 = rtc_ticks(rtc);
    let c0 = timer.counter();
    while rtc_ticks(rtc) - r0 < WINDOW_TICKS {}
    let c1 = timer.counter();
    let r1 = rtc_ticks(rtc);

    let timer_elapsed = c0.wrapping_sub(c1) as u64; // down-counter
    let rtc_elapsed = r1 - r0;
    let measured = (timer_elapsed * RTC_HZ / rtc_elapsed) as u32;
    (measured, timer.free())
}

/// `true` when `meas` is within `tol_pct` % of `believed`.
fn within(meas: u32, believed: u32, tol_pct: u32) -> bool {
    (meas.abs_diff(believed) as u64) * 100 <= believed as u64 * tol_pct as u64
}

const HP_APPSMP: RangeInclusive<u32> = 140_000_000..=175_000_000;
const LP_APPSMP: RangeInclusive<u32> = 30_000_000..=48_000_000;
/// measured-vs-believed `cpu_baseclk` tolerance (hardware counters; ~0.05 % noise).
const TOL_PCT: u32 = 5;

/// Drive the operating point to `target`, but **only** send the request when the
/// HAL doesn't already believe we're in `band`. A `request_perf` that matches the
/// current operating point hangs: the SYSIOP elides the `CLK_CHG` handshake when
/// nothing changes, so `request_perf`'s blocking FIFO read never returns. Guarding
/// on the believed `appsmp` keeps every real call a genuine transition (and makes
/// the boot-state HP measurement a no-op rather than a hang).
fn ensure_perf(clock: &mut Clock, target: Perf, band: &RangeInclusive<u32>) {
    let cur = clock.appsmp().hz().to_Hz();
    if !band.contains(&cur) {
        clock.request_perf(target).expect("request_perf failed");
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut clock = dp.crg.constrain(Config::default()).into_clock();

    // UART1 console + defmt verdict channel. UART1 is on the COM clock
    // (perf-invariant), so its baud stays valid across the LP switch below.
    let clocks = clock.freeze();
    let uart1 = Uart1::new(dp.uart1, &clocks, UartConfig::default()).expect("uart1 init failed");
    defmt_serial::defmt_serial(SERIAL.init(uart1));

    defmt::println!("clock_perf: low-power operating-point verification");
    let mut all_ok = true;
    let mut tok = dp.timer0;

    // [0/4] The RTC must be running, or every later measurement is meaningless.
    let a = rtc_ticks(&dp.rtc0);
    asm::delay(4_000_000);
    let b = rtc_ticks(&dp.rtc0);
    if b > a {
        defmt::println!("[0/4] rtc_alive: PASS (RTC advanced {} ticks)", b - a);
    } else {
        all_ok = false;
        defmt::println!("[0/4] rtc_alive: FAIL (RTC not running; no reference timebase)");
    }

    // [1/4] High performance — the boot operating point. We do NOT request HP
    // here: a request matching the current point makes the SYSIOP skip the
    // CLK_CHG handshake and request_perf blocks forever. ensure_perf only sends
    // the request on a real transition, so this is a no-op at boot.
    ensure_perf(&mut clock, Perf::Hp, &HP_APPSMP);
    let believed_hp = clock.cpu_baseclk().to_Hz();
    let appsmp_hp = clock.appsmp().hz().to_Hz();
    let (meas_hp, t) = measure(&clock, tok, &dp.rtc0);
    tok = t;
    let ok_hp = within(meas_hp, believed_hp, TOL_PCT) && HP_APPSMP.contains(&appsmp_hp);
    all_ok &= ok_hp;
    defmt::println!(
        "[1/4] hp: appsmp={} Hz, cpu_baseclk believed={} measured={} Hz -> {}",
        appsmp_hp,
        believed_hp,
        meas_hp,
        verdict(ok_hp)
    );

    // [2/4] Low power (a real HP->LP transition).
    ensure_perf(&mut clock, Perf::Lp, &LP_APPSMP);
    let believed_lp = clock.cpu_baseclk().to_Hz();
    let appsmp_lp = clock.appsmp().hz().to_Hz();
    let (meas_lp, t) = measure(&clock, tok, &dp.rtc0);
    tok = t;
    let ok_lp = within(meas_lp, believed_lp, TOL_PCT) && LP_APPSMP.contains(&appsmp_lp);
    all_ok &= ok_lp;
    defmt::println!(
        "[2/4] lp: appsmp={} Hz, cpu_baseclk believed={} measured={} Hz -> {}",
        appsmp_lp,
        believed_lp,
        meas_lp,
        verdict(ok_lp)
    );

    // [3/4] Back to high performance (a real LP->HP transition) — proves the
    // round-trip recovers.
    ensure_perf(&mut clock, Perf::Hp, &HP_APPSMP);
    let believed_hp2 = clock.cpu_baseclk().to_Hz();
    let (meas_hp2, t) = measure(&clock, tok, &dp.rtc0);
    tok = t;
    let ok_hp2 = within(meas_hp2, believed_hp2, TOL_PCT) && within(meas_hp2, meas_hp, 8);
    all_ok &= ok_hp2;
    defmt::println!(
        "[3/4] hp_recover: cpu_baseclk believed={} measured={} Hz -> {}",
        believed_hp2,
        meas_hp2,
        verdict(ok_hp2)
    );

    // [4/4] Cross-mode ratio (x100). ~400 (4.0x) proves the perf switch
    // physically changed the clock, not merely the HAL's belief.
    let ratio_x100 = (meas_hp as u64 * 100 / meas_lp.max(1) as u64) as u32;
    let ok_ratio = (320..=500).contains(&ratio_x100);
    all_ok &= ok_ratio;
    defmt::println!(
        "[4/4] ratio hp/lp (x100) = {} (expect ~400) -> {}",
        ratio_x100,
        verdict(ok_ratio)
    );

    let _ = tok; // token retained to the end; nothing further uses it.

    // Decodes to the exact `TEST RESULT: PASS`/`FAIL` sentinel the harness greps.
    defmt::println!("TEST RESULT: {}", verdict(all_ok));

    // Verdict delivered; halt cleanly (UART FIFO has drained the line).
    loop {
        asm::wfi();
    }
}

/// Stable defmt-friendly verdict string.
fn verdict(ok: bool) -> &'static str {
    if ok { "PASS" } else { "FAIL" }
}
