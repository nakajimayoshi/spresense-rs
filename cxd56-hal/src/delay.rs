use cortex_m::delay::Delay as SystDelay;
use cortex_m::peripheral::SYST;
use embedded_hal::delay::DelayNs;

use crate::clocks::Clocks;

/// App-core clock in SYSPLL high-performance mode (XOSC 26 MHz × 12 / 2).
///
/// Boot ROM leaves the Cortex-M4 running at this frequency before user code
/// starts. Provided as a fallback for early-boot delay setup before
/// [`Clocks`] is available.
pub const APP_CORE_CLOCK_HZ: u32 = 156_000_000;

/// SysTick-backed delay implementing [`DelayNs`].
pub struct Delay {
    inner: SystDelay,
}

impl Delay {
    /// Create a `Delay` using the current APP core frequency from `clocks`.
    pub fn new(syst: SYST, clocks: &Clocks) -> Self {
        Self::with_clock(syst, clocks.appsmp.to_Hz())
    }

    /// Create a `Delay` with an explicit core clock frequency. Useful before
    /// [`Clocks`] is available (e.g. during very early boot).
    pub fn with_clock(syst: SYST, sysclk_hz: u32) -> Self {
        Self {
            inner: SystDelay::new(syst, sysclk_hz),
        }
    }

    /// Release the `SYST` peripheral.
    pub fn free(self) -> SYST {
        self.inner.free()
    }
}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        // SysTick resolution at 156 MHz is ~6.4 ns; round up to the next
        // microsecond so the delay is always at least `ns` nanoseconds.
        self.inner.delay_us(ns.div_ceil(1_000));
    }

    fn delay_us(&mut self, us: u32) {
        self.inner.delay_us(us);
    }

    fn delay_ms(&mut self, ms: u32) {
        self.inner.delay_ms(ms);
    }
}
