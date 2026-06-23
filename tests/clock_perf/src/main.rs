//! On-hardware low-power clock-readback test.
//!
//! Verifies the fix for the "no output in LP" bug: after `request_perf(Perf::Lp)`
//! the COM-bus clock that UART1/SPI0/I2C2 derive their baud from must be live, not
//! the stale boot value. Three checks, all at the LP operating point:
//!
//!   [1/3] cached == live — `clock.com` (the `Fixed` field `uart_alt` reads to
//!         build UART1) must equal `clock.freeze().com` (a fresh live read). This
//!         is the `resample_dyn` refresh: before the fix the cached field kept its
//!         boot value (48.75 MHz) at LP.
//!   [2/3] console works — the verdict reaches the host over UART1, whose baud was
//!         computed from the LP COM clock. A clean decode *is* the assertion that
//!         the COM readback is correct (wrong COM ⇒ wrong baud ⇒ no verdict).
//!   [3/3] readback == reality — the SP804 timer (cpu_baseclk) measured against the
//!         fixed 32.768 kHz RTC matches the HAL's believed cpu_baseclk.
//!
//! Note: the LP point reached here is ~78 MHz APP (non-canonical — a separate
//! request_perf matter), so this asserts internal consistency, not a fixed
//! frequency. No external jumper. CXD5602 GPIO is 1.8 V.

#![no_std]
#![no_main]

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

const RTC_HZ: u64 = 32_768;
const WINDOW_TICKS: u64 = RTC_HZ / 4; // 250 ms
const TOL_PCT: u32 = 8;

/// Free-running 32.768 kHz RTC counter, `(RTPOSTCNT<<15)|RTPRECNT`.
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

/// Real cpu_baseclk (Hz) via SP804 ticks over a fixed RTC window.
fn measure(clock: &Clock, tok: pac::Timer0, rtc: &pac::Rtc0) -> (u32, pac::Timer0) {
    let mut timer = Timer::new(tok, clock).expect("cpu base clock unavailable");
    timer.start_free_running(Prescaler::Div1);
    let r0 = rtc_ticks(rtc);
    let c0 = timer.counter();
    while rtc_ticks(rtc) - r0 < WINDOW_TICKS {}
    let c1 = timer.counter();
    let r1 = rtc_ticks(rtc);
    let measured = (c0.wrapping_sub(c1) as u64 * RTC_HZ / (r1 - r0)) as u32;
    (measured, timer.free())
}

fn within(meas: u32, believed: u32, tol_pct: u32) -> bool {
    (meas.abs_diff(believed) as u64) * 100 <= believed as u64 * tol_pct as u64
}

fn verdict(ok: bool) -> &'static str {
    if ok { "PASS" } else { "FAIL" }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut clock = dp.crg.constrain(Config::default()).into_clock();

    // Enter low power (real HV->LV transition from boot). request_perf refreshes
    // the cached SYSIOP-tree clocks (the fix under test).
    clock
        .request_perf(Perf::Lp)
        .expect("request_perf(Lp) failed");

    // The cached COM rate `uart_alt` would read to size the UART1 baud divisor.
    let cached_com = clock.com.hz().to_Hz();

    // Build the console from a fresh live snapshot so its baud is correct at LP
    // regardless of the fix (this is the reference the cached value must match).
    let live = clock.freeze();
    let live_com = live.com.to_Hz();
    let uart1 = Uart1::new(dp.uart1, &live, UartConfig::default()).expect("uart1 init failed");
    defmt_serial::defmt_serial(SERIAL.init(uart1));

    defmt::println!(
        "clock_perf (LOW POWER): appsmp={} sys={} com(cached)={} com(live)={} Hz",
        live.appsmp.to_Hz(),
        live.sys.to_Hz(),
        cached_com,
        live_com
    );
    let mut all_ok = true;

    // [1/3] The fix: resample_dyn refreshed the cached com to the live value.
    let cache_ok = cached_com == live_com;
    all_ok &= cache_ok;
    defmt::println!(
        "[1/3] cache_refresh: cached==live ({} == {}) -> {}",
        cached_com,
        live_com,
        verdict(cache_ok)
    );

    // [2/3] Reaching the host with a decodable verdict over the LP-clocked UART1
    // proves the COM baud is correct. (Implicitly asserted by a clean run; we
    // also sanity-check the RTC reference is alive.)
    let a = rtc_ticks(&dp.rtc0);
    asm::delay(2_000_000);
    let b = rtc_ticks(&dp.rtc0);
    let console_ok = b > a;
    all_ok &= console_ok;
    defmt::println!(
        "[2/3] console+rtc: this line decoded at the LP baud; rtc +{} ticks -> {}",
        b - a,
        verdict(console_ok)
    );

    // [3/3] Readback matches reality: measured cpu_baseclk ~ believed.
    let (meas, _tok) = measure(&clock, dp.timer0, &dp.rtc0);
    let believed = clock.cpu_baseclk().to_Hz();
    let meas_ok = within(meas, believed, TOL_PCT);
    all_ok &= meas_ok;
    defmt::println!(
        "[3/3] readback==reality: cpu_base believed={} measured={} -> {}",
        believed,
        meas,
        verdict(meas_ok)
    );

    defmt::println!("TEST RESULT: {}", verdict(all_ok));
    loop {
        asm::wfi();
    }
}
