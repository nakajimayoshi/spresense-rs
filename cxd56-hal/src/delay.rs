use cortex_m::delay::Delay as SystDelay;
use cortex_m::peripheral::SYST;
use embedded_hal::delay::DelayNs;

/// App-core clock in SYSPLL high-performance mode (XOSC 26 MHz × 12 / 2).
///
/// This is the frequency the boot ROM leaves the Cortex-M4 running at before
/// user code starts. Verified against the SYSPLL formula in
/// `nuttx/arch/arm/src/cxd56xx/cxd56_clock.c`. Use [`Delay::with_clock`] if
/// you are running in low-power mode (~39 MHz) or have reconfigured the PLL.
pub const APP_CORE_CLOCK_HZ: u32 = 156_000_000;

/// SysTick-backed delay implementing [`DelayNs`].
pub struct Delay {
    inner: SystDelay,
}

impl Delay {
    /// Create a `Delay` assuming [`APP_CORE_CLOCK_HZ`] (156 MHz, HP mode).
    pub fn new(syst: SYST) -> Self {
        Self::with_clock(syst, APP_CORE_CLOCK_HZ)
    }

    /// Create a `Delay` for an explicit core clock frequency.
    pub fn with_clock(syst: SYST, sysclk_hz: u32) -> Self {
        Self { inner: SystDelay::new(syst, sysclk_hz) }
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
