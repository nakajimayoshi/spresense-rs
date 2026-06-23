use crate::pac;
use crate::regs;
use cortex_m::peripheral::NVIC;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

/// Input pull configuration for a pad's internal resistors.
///
/// Programmed via the per-pin TOPREG `IO_*` register, whose `PUN` (pull-up) and
/// `PDN` (pull-down) bits are both *active-low* (0 = that resistor enabled).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pull {
    /// No pull; the pin floats when undriven (`PUN=1, PDN=1`).
    Floating,
    /// Internal pull-up to the 1.8 V pad rail (`PUN=0, PDN=1`).
    Up,
    /// Internal pull-down to ground (`PUN=1, PDN=0`).
    Down,
    /// Bus keeper: both resistors on, weakly holding the last driven level
    /// (`PUN=0, PDN=0`). Matches NuttX `PINCONF_BUSKEEPER`.
    BusKeeper,
}

// Bit positions — identical layout for every TOPREG GP_* register.
const IN_BIT: u32 = 1 << 0;
const OUT_BIT: u32 = 1 << 8;
const DIR_BIT: u32 = 1 << 16; // active-low: 0 = drive output, 1 = high-Z input

// Bit positions — identical layout for every TOPREG IO_* (IOCELL) pad register.
// PUN/PDN are active-low: 0 = that pull resistor enabled. LOWEMI (bit 24, drive
// strength) is deliberately never written, so it keeps whatever value pinmux set.
const ENZI_BIT: u32 = 1 << 0; // input buffer: 1 = enabled
const PUN_BIT: u32 = 1 << 8; // pull-up:   0 = enabled, 1 = off
const PDN_BIT: u32 = 1 << 16; // pull-down: 0 = enabled, 1 = off

mod sealed {
    pub trait Sealed {
        fn read_bits(&self) -> u32;
        fn write_bits(&self, val: u32);
        fn read_io_bits(&self) -> u32;
        fn write_io_bits(&self, val: u32);
    }
}

/// Marker trait for svd2rust GPIO register accessors with the standard
/// IN[0] / OUT[8] / DIR[16] (active-low) field layout, paired with the pin's
/// `IO_*` (IOCELL) pad register for input-buffer and pull configuration.
///
/// `PIN` is the CXD5602 pin number (the `PIN_*` numbering from NuttX
/// `cxd56_pinconfig`), used to route the pad to an EXDEVICE interrupt slot.
pub trait PinReg: sealed::Sealed + 'static {
    const PIN: u8;
}

// Each pin has a GP_* data register (`$gp`, IN/OUT/DIR) and a matching IO_*
// pad register (`$io`, ENZI/PUN/PDN/LOWEMI). The GP_* one is owned by the
// `GpioPin`; the IO_* one is reached through the shared `regs::topreg()` block
// (its aliasing invariant is documented there) since it is not individually
// owned. Macros cannot case-convert `GpFoo` → `io_foo`, so the pairs are
// spelled out explicitly.
macro_rules! impl_pinreg {
    ($($gp:ident => $io:ident => $pin:literal),+ $(,)?) => {$(
        impl sealed::Sealed for pac::topreg::$gp {
            fn read_bits(&self) -> u32 {
                self.read().bits()
            }
            fn write_bits(&self, val: u32) {
                self.modify(|_, w| unsafe { w.bits(val) });
            }
            fn read_io_bits(&self) -> u32 {
                crate::regs::topreg().$io().read().bits()
            }
            fn write_io_bits(&self, val: u32) {
                crate::regs::topreg().$io().modify(|_, w| unsafe { w.bits(val) });
            }
        }
        impl PinReg for pac::topreg::$gp {
            const PIN: u8 = $pin;
        }
    )+};
}

// The third column is the CXD5602 pin number (NuttX `PIN_*` numbering), used to
// route the pad to an EXDEVICE interrupt slot.
impl_pinreg!(
    // UART1 console pads (SPI0_CS_X = TX, SPI0_SCK = RX); Func0 is GPIO.
    GpSpi0CsX => io_spi0_cs_x => 17,
    GpSpi0Sck => io_spi0_sck => 18,
    // Main-board LEDs + Arduino D14 (pre-existing)
    GpI2s1Bck => io_i2s1_bck => 97,
    GpI2s1Lrck => io_i2s1_lrck => 98,
    GpI2s1DataIn => io_i2s1_data_in => 99,
    GpI2s1DataOut => io_i2s1_data_out => 100,
    GpI2c4Bck => io_i2c4_bck => 1,
    // JP1 header
    GpUart2Txd => io_uart2_txd => 67,
    GpUart2Rxd => io_uart2_rxd => 68,
    GpUart2Rts => io_uart2_rts => 70,
    GpUart2Cts => io_uart2_cts => 69,
    GpI2s0Bck => io_i2s0_bck => 93,
    GpI2s0Lrck => io_i2s0_lrck => 94,
    GpEmmcCmd => io_emmc_cmd => 76,
    GpEmmcClk => io_emmc_clk => 75,
    GpSenIrqIn => io_sen_irq_in => 37,
    // JP2 header
    GpEmmcData3 => io_emmc_data3 => 80,
    GpEmmcData2 => io_emmc_data2 => 79,
    GpI2s0DataIn => io_i2s0_data_in => 95,
    GpI2s0DataOut => io_i2s0_data_out => 96,
    GpEmmcData1 => io_emmc_data1 => 78,
    GpEmmcData0 => io_emmc_data0 => 77,
    GpI2c0Bck => io_i2c0_bck => 44,
    GpI2c0Bdt => io_i2c0_bdt => 45,
);

/// Program a pad's `IO_*` register for input use: enable the input buffer
/// (ENZI) and select the requested pull, preserving LOWEMI (drive strength).
fn configure_input_pad<R: PinReg>(reg: &R, pull: Pull) {
    let mut raw = reg.read_io_bits() | ENZI_BIT;
    // PUN/PDN are active-low: clear a bit to enable that resistor.
    match pull {
        Pull::Floating => raw |= PUN_BIT | PDN_BIT,
        Pull::Up => {
            raw &= !PUN_BIT;
            raw |= PDN_BIT;
        }
        Pull::Down => {
            raw |= PUN_BIT;
            raw &= !PDN_BIT;
        }
        Pull::BusKeeper => raw &= !(PUN_BIT | PDN_BIT),
    }
    reg.write_io_bits(raw);
}

/// Unconfigured GPIO pin. Call [`into_output`](GpioPin::into_output) or
/// [`into_input`](GpioPin::into_input) to configure the direction.
pub struct GpioPin<R: PinReg> {
    reg: &'static R,
}

impl<R: PinReg> GpioPin<R> {
    /// # Safety
    /// Caller must ensure exclusive access to this pin register for the
    /// lifetime of the program (no other `GpioPin` or direct register access
    /// may exist simultaneously).
    pub unsafe fn new(reg: &'static R) -> Self {
        Self { reg }
    }

    /// Configure as a push-pull output driving `initial`.
    ///
    /// This intentionally leaves the pad's `IO_*` register untouched: the reset
    /// state already has both pulls off, the output driver works regardless of
    /// the input-buffer/pull bits, and rewriting it would clobber a deliberate
    /// drive-strength (LOWEMI) or pull setting. Pad config is orthogonal to
    /// direction, matching NuttX's split between `cxd56_gpio` and pinmux.
    pub fn into_output(self, initial: Level) -> Output<R> {
        let raw = self.reg.read_bits() & !DIR_BIT; // DIR=0 → drive output
        let raw = match initial {
            Level::High => raw | OUT_BIT,
            Level::Low => raw & !OUT_BIT,
        };
        self.reg.write_bits(raw);
        Output { reg: self.reg }
    }

    /// Configure as a high-Z input with the given pull.
    ///
    /// Enables the pad input buffer (ENZI) and programs the pull in the `IO_*`
    /// register, *then* releases the GP_* direction to high-Z — so the chosen
    /// pull is already active the instant the pin stops driving.
    pub fn into_input(self, pull: Pull) -> Input<R> {
        configure_input_pad(self.reg, pull);
        let raw = self.reg.read_bits() | DIR_BIT; // DIR=1 → high-Z input
        self.reg.write_bits(raw);
        Input { reg: self.reg }
    }

    /// Configure as a floating (no-pull) input. See [`into_input`](Self::into_input).
    pub fn into_floating_input(self) -> Input<R> {
        self.into_input(Pull::Floating)
    }

    /// Configure as an input with the internal pull-up enabled.
    pub fn into_pull_up_input(self) -> Input<R> {
        self.into_input(Pull::Up)
    }

    /// Configure as an input with the internal pull-down enabled.
    pub fn into_pull_down_input(self) -> Input<R> {
        self.into_input(Pull::Down)
    }
}

/// Push-pull output.
pub struct Output<R: PinReg> {
    reg: &'static R,
}

impl<R: PinReg> Output<R> {
    pub fn set_high(&mut self) {
        let raw = self.reg.read_bits() | OUT_BIT;
        self.reg.write_bits(raw);
    }

    pub fn set_low(&mut self) {
        let raw = self.reg.read_bits() & !OUT_BIT;
        self.reg.write_bits(raw);
    }

    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low => self.set_low(),
        }
    }

    pub fn is_set_high(&self) -> bool {
        self.reg.read_bits() & OUT_BIT != 0
    }
}

impl<R: PinReg> embedded_hal::digital::ErrorType for Output<R> {
    type Error = core::convert::Infallible;
}

impl<R: PinReg> embedded_hal::digital::OutputPin for Output<R> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Output::set_high(self);
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Output::set_low(self);
        Ok(())
    }
}

impl<R: PinReg> embedded_hal::digital::StatefulOutputPin for Output<R> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Output::is_set_high(self))
    }
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!Output::is_set_high(self))
    }
}

/// High-Z input.
///
/// The pad's input buffer (ENZI) and pull (`PUN`/`PDN`, both active-low) are set
/// when the pin is configured via [`GpioPin::into_input`] and friends; change the
/// pull at runtime with [`set_pull`](Self::set_pull).
pub struct Input<R: PinReg> {
    reg: &'static R,
}

impl<R: PinReg> Input<R> {
    pub fn is_high(&self) -> bool {
        self.reg.read_bits() & IN_BIT != 0
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    pub fn get_level(&self) -> Level {
        if self.is_high() {
            Level::High
        } else {
            Level::Low
        }
    }

    /// Change the pad pull at runtime, leaving the input buffer enabled.
    ///
    /// Switching pull is not instantaneous: the weak (~100 kΩ) resistor charges
    /// the pin/trace capacitance, so allow a brief settle before sampling a pin
    /// that nothing else is driving.
    pub fn set_pull(&mut self, pull: Pull) {
        configure_input_pad(self.reg, pull);
    }
}

impl<R: PinReg> embedded_hal::digital::ErrorType for Input<R> {
    type Error = core::convert::Infallible;
}

impl<R: PinReg> embedded_hal::digital::InputPin for Input<R> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_high(self))
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_low(self))
    }
}

// =============================================================================
// GPIO interrupts (EXDEVICE)
// =============================================================================
//
// The CXD5602 has 12 GPIO external-interrupt slots (EXDEVICE_0..11 → NVIC IRQ
// 20..31). A slot is bound to a pin through the TOPREG `IOCSYS/IOCAPP_INTSEL`
// mux, and its trigger is configured by the `PMU_WAKE_TRIG_*` registers. Edge
// modes use the PMU latch (TRIG0_RAW status, TRIG0_CLR to re-arm); level modes
// route the pad straight through. Encodings and register layout follow the NuttX
// driver `cxd56_gpioint.c`.
//
// `wait_for_*` keep the NVIC line masked and sleep in WFE, woken by SCR.SEVONPEND
// when the (masked) line goes pending. For handler-driven use call
// `enable_interrupt` and clear the latch from your `#[interrupt]` with
// [`clear_interrupt`].

const MAX_SLOT: u8 = 12;
const MAX_SYS_SLOT: u8 = 6;
const SLOT_UNUSED: u8 = 0x3f;
/// First pin number of the APP domain; lower pin numbers are in the SYS domain.
const PIN_APP_BASE: u8 = 56;

/// Trigger condition for a GPIO interrupt.
///
/// Level modes assert while the pin holds the level; edge modes latch a single
/// transition (held until cleared). Values are the 3-bit `PMU_WAKE_TRIG_INTDET`
/// polarity encoding.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    LevelHigh,
    LevelLow,
    RisingEdge,
    FallingEdge,
    AnyEdge,
}

impl Trigger {
    /// 3-bit `INTDET` polarity field.
    fn intdet(self) -> u32 {
        match self {
            Trigger::LevelHigh => 2,
            Trigger::LevelLow => 3,
            Trigger::RisingEdge => 4,
            Trigger::FallingEdge => 5,
            Trigger::AnyEdge => 7,
        }
    }

    /// 3-bit `CPUINTSEL` route field. Levels route Through/Inverter (or Pmu with
    /// the noise filter); every edge mode uses the PMU latch.
    fn route(self, filter: bool) -> u32 {
        match self {
            Trigger::LevelHigh => {
                if filter {
                    2 // Pmu
                } else {
                    0 // Through
                }
            }
            Trigger::LevelLow => {
                if filter {
                    2 // Pmu
                } else {
                    1 // Inverter
                }
            }
            Trigger::RisingEdge | Trigger::FallingEdge | Trigger::AnyEdge => 3, // PmuLatch
        }
    }

    /// Whether this is an edge trigger (latches a single transition).
    fn is_edge(self) -> bool {
        matches!(
            self,
            Trigger::RisingEdge | Trigger::FallingEdge | Trigger::AnyEdge
        )
    }
}

/// Error returned by [`Input::into_interrupt`].
#[derive(Debug, Error)]
pub enum InterruptError {
    /// Every EXDEVICE slot in the pin's domain is taken (6 SYS, 6 APP).
    #[error("no free EXDEVICE slot for this pin's domain")]
    NoSlotAvailable,
}

/// Pin index stored in the INTSEL mux for `pin` (index within its domain).
fn intsel_value(pin: u8) -> u8 {
    if pin < PIN_APP_BASE {
        pin - 1
    } else {
        pin - PIN_APP_BASE
    }
}

/// Read the 6-bit pin index in INTSEL slot `slot` (0..12). One byte per slot:
/// SYS slots 0–3/4–5 in `IOCSYS_INTSEL0/1`, APP slots 6–9/10–11 in `IOCAPP_INTSEL0/1`.
fn intsel_read(slot: u8) -> u8 {
    let t = regs::topreg();
    let (bits, byte) = match slot {
        0..=3 => (t.iocsys_intsel0().read().bits(), slot),
        4..=5 => (t.iocsys_intsel1().read().bits(), slot - 4),
        6..=9 => (t.iocapp_intsel0().read().bits(), slot - 6),
        10..=11 => (t.iocapp_intsel1().read().bits(), slot - 10),
        _ => unreachable!(),
    };
    ((bits >> (byte as u32 * 8)) & 0xff) as u8
}

/// Write the 6-bit pin index `val` into INTSEL slot `slot`. Not interrupt-safe on
/// its own — callers run it under a critical section.
fn intsel_write(slot: u8, val: u8) {
    let t = regs::topreg();
    let val = val as u32;
    macro_rules! put {
        ($reg:ident, $byte:expr) => {{
            let shift = ($byte as u32) * 8;
            t.$reg()
                .modify(|r, w| unsafe { w.bits((r.bits() & !(0xffu32 << shift)) | (val << shift)) });
        }};
    }
    match slot {
        0..=3 => put!(iocsys_intsel0, slot),
        4..=5 => put!(iocsys_intsel1, slot - 4),
        6..=9 => put!(iocapp_intsel0, slot - 6),
        10..=11 => put!(iocapp_intsel1, slot - 10),
        _ => unreachable!(),
    }
}

/// Allocate (or reuse) an EXDEVICE slot for `pin`. Scans the pin's domain (SYS
/// slots 0–5, APP slots 6–11): reuses the slot if the pin is already mapped, else
/// claims the first free (`0x3f`) one. Runs under a critical section — the INTSEL
/// mux is shared across all interrupt pins.
fn allocate_slot(pin: u8) -> Result<u8, InterruptError> {
    let (lo, hi) = if pin < PIN_APP_BASE {
        (0, MAX_SYS_SLOT)
    } else {
        (MAX_SYS_SLOT, MAX_SLOT)
    };
    let want = intsel_value(pin);
    critical_section::with(|_| {
        let mut free: Option<u8> = None;
        for slot in lo..hi {
            let v = intsel_read(slot);
            if v == want {
                return Ok(slot); // already mapped → reuse
            }
            if v == SLOT_UNUSED && free.is_none() {
                free = Some(slot);
            }
        }
        match free {
            Some(slot) => {
                intsel_write(slot, want);
                Ok(slot)
            }
            None => Err(InterruptError::NoSlotAvailable),
        }
    })
}

/// EXDEVICE interrupt for `slot` (0..12).
fn slot_interrupt(slot: u8) -> pac::Interrupt {
    use pac::Interrupt::*;
    match slot {
        0 => EXDEVICE_0,
        1 => EXDEVICE_1,
        2 => EXDEVICE_2,
        3 => EXDEVICE_3,
        4 => EXDEVICE_4,
        5 => EXDEVICE_5,
        6 => EXDEVICE_6,
        7 => EXDEVICE_7,
        8 => EXDEVICE_8,
        9 => EXDEVICE_9,
        10 => EXDEVICE_10,
        11 => EXDEVICE_11,
        _ => unreachable!(),
    }
}

/// Slot index for an EXDEVICE interrupt, or `None` for any other line.
fn interrupt_slot(irq: pac::Interrupt) -> Option<u8> {
    use pac::Interrupt::*;
    Some(match irq {
        EXDEVICE_0 => 0,
        EXDEVICE_1 => 1,
        EXDEVICE_2 => 2,
        EXDEVICE_3 => 3,
        EXDEVICE_4 => 4,
        EXDEVICE_5 => 5,
        EXDEVICE_6 => 6,
        EXDEVICE_7 => 7,
        EXDEVICE_8 => 8,
        EXDEVICE_9 => 9,
        EXDEVICE_10 => 10,
        EXDEVICE_11 => 11,
        _ => return None,
    })
}

/// Cycles held after configuring an edge trigger so the PMU samples its baseline.
/// The detector runs on the ~32 kHz PMU clock; ≥~5k CPU cycles (≈1 sample period
/// at the max core clock) were needed on hardware for reliable capture, so this
/// keeps a 4× margin (and only more at slower core clocks). One-time per
/// (re)configure, so the cost is irrelevant next to the wait it precedes.
const EDGE_ARM_SETTLE: u32 = 20_000;

/// Program polarity (`INTDET`), route (`CPUINTSEL`) and noise filter
/// (`NOISECUTEN`) for `slot`. The register writes run under a critical section
/// (read-modify-write of shared per-bank registers); the settle below does not.
///
/// After (re)configuring an edge trigger this holds for [`EDGE_ARM_SETTLE`]
/// cycles so the slow PMU detector samples the *current* pin level as its edge
/// baseline. Without it, an edge driven immediately after the config write — e.g.
/// a `RisingEdge`→`FallingEdge` switch then a drop — is silently dropped because
/// the detector never saw the pre-edge level (measured on hardware; the miss is
/// total, not just late, which then hangs a `wait_for_*edge`). The caller must
/// hold the pin at the baseline level across this call, which it naturally does
/// (the edge is driven afterwards).
fn set_gpioint_config(slot: u8, trigger: Trigger, filter: bool) {
    let t = regs::topreg();
    let intdet = trigger.intdet();
    let route = trigger.route(filter);
    critical_section::with(|_| {
        // Noise filter: bit (16 + slot) in NOISECUTEN0.
        let fbit = 1u32 << (16 + slot as u32);
        t.pmu_wake_trig_noisecuten0().modify(|r, w| unsafe {
            w.bits(if filter { r.bits() | fbit } else { r.bits() & !fbit })
        });

        // Polarity + route are 3-bit fields. Slots 0–3 live in INTDET0/CPUINTSEL0
        // at bit 16+slot*4; slots 4–11 in INTDET1/CPUINTSEL1 at bit (slot-4)*4.
        let shift = 16 + (slot as u32) * 4;
        if shift < 32 {
            t.pmu_wake_trig_intdet0()
                .modify(|r, w| unsafe { w.bits((r.bits() & !(0x7u32 << shift)) | (intdet << shift)) });
            t.pmu_wake_trig_cpuintsel0()
                .modify(|r, w| unsafe { w.bits((r.bits() & !(0x7u32 << shift)) | (route << shift)) });
        } else {
            let shift = shift - 32;
            t.pmu_wake_trig_intdet1()
                .modify(|r, w| unsafe { w.bits((r.bits() & !(0x7u32 << shift)) | (intdet << shift)) });
            t.pmu_wake_trig_cpuintsel1()
                .modify(|r, w| unsafe { w.bits((r.bits() & !(0x7u32 << shift)) | (route << shift)) });
        }
    });

    // Let the PMU detector latch the current level as the edge baseline before the
    // caller drives the edge (see the doc comment). Levels need no such arming.
    if trigger.is_edge() {
        cortex_m::asm::delay(EDGE_ARM_SETTLE);
    }
}

/// Raw PMU edge-latch status for `slot` (bit 16+slot in `PMU_WAKE_TRIG0_RAW`).
/// Upstream of the INTC/NVIC, so it reflects a captured edge regardless of mask.
fn edge_latched(slot: u8) -> bool {
    regs::topreg_sub().pmu_wake_trig0_raw().read().bits() & (1u32 << (16 + slot as u32)) != 0
}

/// Iterations spent waiting for a `PMU_WAKE_TRIG0_CLR` write to land in the raw
/// latch. The clear propagates a few slow PMU-clock cycles after the write
/// (~3k CPU cycles measured on hardware); bounded so a still-asserted level
/// trigger cannot spin forever.
const CLEAR_SETTLE_ITERS: u32 = 20_000;

/// Clear the PMU edge latch for `slot` (write 1 to bit 16+slot in
/// `PMU_WAKE_TRIG0_CLR`) and wait for it to take effect.
///
/// The PMU is in a slow clock domain, so the raw latch ([`edge_latched`]) does
/// not read clear until a few PMU cycles after the write. Spinning until it does
/// makes the clear synchronous: an immediately following [`edge_latched`] read or
/// edge re-arm sees a clean latch (the polled `wait_for_*`/`is_pending` API), and
/// an `#[interrupt]` handler's [`clear_interrupt`] de-asserts the line before it
/// returns so the (level-type) EXDEVICE request does not re-fire.
fn clear_latch(slot: u8) {
    let sub = regs::topreg_sub();
    let bit = 1u32 << (16 + slot as u32);
    sub.pmu_wake_trig0_clr().write(|w| unsafe { w.bits(bit) });
    for _ in 0..CLEAR_SETTLE_ITERS {
        if sub.pmu_wake_trig0_raw().read().bits() & bit == 0 {
            break;
        }
    }
}

/// Set `SCR.SEVONPEND` so a masked-but-pending NVIC line is a `WFE` wake event.
/// Process-global and idempotent; harmless because every `WFE` wait re-checks its
/// own condition, so a spurious wake only costs a loop iteration.
fn set_sevonpend() {
    const SEVONPEND: u32 = 1 << 4;
    // Safety: SCB is a fixed core peripheral; this is one RMW of its SCR register.
    unsafe {
        (*cortex_m::peripheral::SCB::PTR)
            .scr
            .modify(|scr| scr | SEVONPEND);
    }
}

/// Clear the EXDEVICE edge latch for `interrupt` from within its `#[interrupt]`
/// handler. The NVIC pending bit is cleared by exception entry, but the PMU latch
/// must be cleared here or the line re-asserts after the handler returns. No-op for
/// a non-EXDEVICE interrupt. Mirrors [`crate::timer::clear_pending`].
pub fn clear_interrupt(interrupt: pac::Interrupt) {
    if let Some(slot) = interrupt_slot(interrupt) {
        clear_latch(slot);
    }
}

/// A GPIO input wired to an EXDEVICE interrupt slot.
///
/// Created by [`Input::into_interrupt`]. Offers blocking
/// [`wait_for_high`](Self::wait_for_high) … [`wait_for_any_edge`](Self::wait_for_any_edge)
/// plus a handler-driven mode ([`enable_interrupt`](Self::enable_interrupt) with your
/// own `#[interrupt]` calling [`clear_interrupt`]). Reading the pin
/// ([`is_high`](Self::is_high) / [`is_low`](Self::is_low)) still works.
pub struct InterruptInput<R: PinReg> {
    input: Input<R>,
    slot: u8,
    irq: pac::Interrupt,
    filter: bool,
    trigger: Trigger,
}

impl<R: PinReg> Input<R> {
    /// Allocate an EXDEVICE slot for this pin and program `trigger`.
    ///
    /// `noise_filter` routes level triggers through the PMU noise filter (edge
    /// triggers are reliable without it, given the baseline settle in
    /// [`set_gpioint_config`]). The pad input buffer (ENZI) must already be
    /// enabled, exactly as for reading via
    /// [`Input`]. The chip-level INTC gate is opened here (see [`crate::interrupt`])
    /// so a latched edge reaches the NVIC; the NVIC line itself is left **masked**:
    /// [`wait_for_high`](InterruptInput::wait_for_high) and friends sleep in `WFE`,
    /// woken when the masked-but-pending line raises an event via `SCR.SEVONPEND`
    /// (this sets it once). The INTC gate is required for that wake as well as for
    /// handler-driven dispatch via [`enable_interrupt`](InterruptInput::enable_interrupt);
    /// [`release`](InterruptInput::release) closes it again.
    ///
    /// Returns [`InterruptError::NoSlotAvailable`] if every slot in the pin's
    /// domain (SYS: pins < 56, APP: pins ≥ 56) is in use.
    pub fn into_interrupt(
        self,
        trigger: Trigger,
        noise_filter: bool,
    ) -> Result<InterruptInput<R>, InterruptError> {
        let slot = allocate_slot(R::PIN)?;
        set_sevonpend();
        let mut this = InterruptInput {
            input: self,
            slot,
            irq: slot_interrupt(slot),
            filter: noise_filter,
            trigger,
        };
        this.apply_trigger();
        this.clear_pending();
        // Open the INTC gate so a latched edge reaches the NVIC — needed both for
        // handler dispatch and for `SCR.SEVONPEND` to wake the masked `wait_for_*`
        // sleeps. The NVIC line stays masked until `enable_interrupt`.
        crate::interrupt::enable(this.irq);
        Ok(this)
    }
}

impl<R: PinReg> InterruptInput<R> {
    /// The EXDEVICE interrupt this pin's slot is wired to.
    pub fn interrupt(&self) -> pac::Interrupt {
        self.irq
    }

    pub fn is_high(&self) -> bool {
        self.input.is_high()
    }

    pub fn is_low(&self) -> bool {
        self.input.is_low()
    }

    pub fn get_level(&self) -> Level {
        self.input.get_level()
    }

    /// The currently configured trigger.
    pub fn trigger(&self) -> Trigger {
        self.trigger
    }

    /// Reprogram the trigger condition.
    pub fn set_trigger(&mut self, trigger: Trigger) {
        self.trigger = trigger;
        self.apply_trigger();
    }

    fn apply_trigger(&self) {
        set_gpioint_config(self.slot, self.trigger, self.filter);
    }

    /// Unmask the NVIC line so the configured trigger dispatches your `#[interrupt]`
    /// handler. The INTC gate in front of the NVIC was already opened by
    /// [`into_interrupt`](Input::into_interrupt). Not needed for the `wait_for_*`
    /// methods (they keep the NVIC line masked and sleep in `WFE`).
    pub fn enable_interrupt(&mut self) {
        // Safety: this HAL uses `critical-section` (PRIMASK), not BASEPRI priority
        // masking, so unmasking a line cannot escape an in-progress critical section.
        unsafe { NVIC::unmask(self.irq) };
    }

    /// Mask the NVIC line.
    pub fn disable_interrupt(&mut self) {
        NVIC::mask(self.irq);
    }

    /// Whether an edge has been latched since the last clear (raw PMU status,
    /// independent of the NVIC mask). Meaningful for edge triggers; for level
    /// triggers read [`is_high`](Self::is_high) / [`is_low`](Self::is_low).
    pub fn is_pending(&self) -> bool {
        edge_latched(self.slot)
    }

    /// Clear a latched edge and the NVIC pending bit.
    pub fn clear_pending(&mut self) {
        clear_latch(self.slot);
        NVIC::unpend(self.irq);
    }

    /// Block until the pin reads high. Returns immediately if it already is.
    pub fn wait_for_high(&mut self) {
        self.wait_level(Trigger::LevelHigh, Level::High);
    }

    /// Block until the pin reads low. Returns immediately if it already is.
    pub fn wait_for_low(&mut self) {
        self.wait_level(Trigger::LevelLow, Level::Low);
    }

    /// Block until a rising edge is latched. An edge already latched since
    /// configuration / the last [`clear_pending`](Self::clear_pending) satisfies
    /// this immediately — call `clear_pending` first to discard earlier edges.
    pub fn wait_for_rising_edge(&mut self) {
        self.wait_edge(Trigger::RisingEdge);
    }

    /// Block until a falling edge is latched. See [`wait_for_rising_edge`](Self::wait_for_rising_edge).
    pub fn wait_for_falling_edge(&mut self) {
        self.wait_edge(Trigger::FallingEdge);
    }

    /// Block until either edge is latched. See [`wait_for_rising_edge`](Self::wait_for_rising_edge).
    pub fn wait_for_any_edge(&mut self) {
        self.wait_edge(Trigger::AnyEdge);
    }

    fn wait_level(&mut self, trigger: Trigger, level: Level) {
        if self.input.get_level() == level {
            return;
        }
        if self.trigger != trigger {
            self.set_trigger(trigger);
        }
        // The level trigger pends the (masked) line when the pin reaches `level`,
        // waking WFE via SEVONPEND; the pin is the source of truth each iteration.
        while self.input.get_level() != level {
            cortex_m::asm::wfe();
        }
        self.clear_pending();
    }

    fn wait_edge(&mut self, trigger: Trigger) {
        if self.trigger != trigger {
            self.set_trigger(trigger);
            // Drop any edge captured under the previous trigger.
            self.clear_pending();
        }
        // Return on an edge already latched since config / last clear; otherwise
        // sleep until one arrives. Checking before WFE, together with the ARM event
        // register, closes the latch-vs-sleep race (an edge in the window sets the
        // event register, so the next WFE returns immediately and we re-check).
        while !self.is_pending() {
            cortex_m::asm::wfe();
        }
        self.clear_pending();
    }

    /// Release the EXDEVICE slot and return the plain [`Input`]. Masks the NVIC
    /// line, closes the INTC gate, frees the slot in the mux, parks its trigger at
    /// level-high (the unused-slot convention) and clears any latch.
    pub fn release(mut self) -> Input<R> {
        self.disable_interrupt();
        // Close the INTC gate opened by `into_interrupt` (mirror of its `enable`).
        crate::interrupt::disable(self.irq);
        set_gpioint_config(self.slot, Trigger::LevelHigh, false);
        critical_section::with(|_| intsel_write(self.slot, SLOT_UNUSED));
        self.clear_pending();
        self.input
    }
}

/// Per-port split structs.
///
/// Mirrors the convention used by `stm32f4xx-hal` (`gpioa::Parts`),
/// `nrf-hal` (`p0::Parts`), etc. Construct via [`Parts::new`], which
/// consumes the corresponding PAC singleton — proving exclusive access
/// so no `unsafe` is needed at the call site.
pub mod pins {
    use super::GpioPin;
    use crate::pac;

    /// GPIO pins accessible via the TOPREG GP_* registers.
    ///
    /// The four `gp_i2s1_*` pins drive the Spresense main-board LEDs
    /// (`gp_i2s1_bck` = LED0, `gp_i2s1_lrck` = LED1, `gp_i2s1_data_in` = LED2,
    /// `gp_i2s1_data_out` = LED3). The remaining fields are the digital GPIO
    /// pins broken out on the main-board headers JP1 and JP2; field names follow
    /// the CXD5602 signal name, and the doc comments give the Arduino pin label.
    /// Pins default to mode0 (GPIO); selecting an alternate function (UART2,
    /// I2S0, SPI5, I2C0, …) is done via the `IOCAPP_IOMD` / `IOCSYS_IOMD1`
    /// mode-mux registers, which share a group across all pins of a peripheral.
    pub struct Parts {
        /// SPI0_CS_X — UART1_TX / on-board console.
        /// Func0 = GPIO, Func1 = UART1, Func2 = SPI0.
        pub gp_spi0_cs_x: GpioPin<pac::topreg::GpSpi0CsX>,
        /// SPI0_SCK — UART1_RX / on-board console.
        /// Func0 = GPIO, Func1 = UART1, Func2 = SPI0.
        pub gp_spi0_sck: GpioPin<pac::topreg::GpSpi0Sck>,
        pub gp_i2s1_bck: GpioPin<pac::topreg::GpI2s1Bck>,
        pub gp_i2s1_lrck: GpioPin<pac::topreg::GpI2s1Lrck>,
        pub gp_i2s1_data_in: GpioPin<pac::topreg::GpI2s1DataIn>,
        pub gp_i2s1_data_out: GpioPin<pac::topreg::GpI2s1DataOut>,
        pub gp_i2c4_bck: GpioPin<pac::topreg::GpI2c4Bck>,

        // --- JP1 header ---
        /// JP1 pin 2 — UART2_TX (Arduino D01).
        pub gp_uart2_txd: GpioPin<pac::topreg::GpUart2Txd>,
        /// JP1 pin 3 — UART2_RX (Arduino D00).
        pub gp_uart2_rxd: GpioPin<pac::topreg::GpUart2Rxd>,
        /// JP1 pin 4 — UART2_RTS (Arduino D28).
        pub gp_uart2_rts: GpioPin<pac::topreg::GpUart2Rts>,
        /// JP1 pin 5 — UART2_CTS (Arduino D27).
        pub gp_uart2_cts: GpioPin<pac::topreg::GpUart2Cts>,
        /// JP1 pin 6 — I2S0_BCK (Arduino D26).
        pub gp_i2s0_bck: GpioPin<pac::topreg::GpI2s0Bck>,
        /// JP1 pin 7 — I2S0_LRCK (Arduino D25).
        pub gp_i2s0_lrck: GpioPin<pac::topreg::GpI2s0Lrck>,
        /// JP1 pin 8 — SPI5_CS_X / EMMC_CMD (Arduino D24).
        pub gp_emmc_cmd: GpioPin<pac::topreg::GpEmmcCmd>,
        /// JP1 pin 9 — SPI5_SCK / EMMC_CLK (Arduino D23).
        pub gp_emmc_clk: GpioPin<pac::topreg::GpEmmcClk>,
        /// JP1 pin 12 — SEN_IRQ (Arduino D22).
        pub gp_sen_irq_in: GpioPin<pac::topreg::GpSenIrqIn>,

        // --- JP2 header ---
        /// JP2 pin 4 — GPIO / EMMC_DATA3 (Arduino D21).
        pub gp_emmc_data3: GpioPin<pac::topreg::GpEmmcData3>,
        /// JP2 pin 5 — GPIO / EMMC_DATA2 (Arduino D20).
        pub gp_emmc_data2: GpioPin<pac::topreg::GpEmmcData2>,
        /// JP2 pin 6 — I2S0_DATA_IN (Arduino D19).
        pub gp_i2s0_data_in: GpioPin<pac::topreg::GpI2s0DataIn>,
        /// JP2 pin 7 — I2S0_DATA_OUT (Arduino D18).
        pub gp_i2s0_data_out: GpioPin<pac::topreg::GpI2s0DataOut>,
        /// JP2 pin 8 — SPI5_MISO / EMMC_DATA1 (Arduino D17).
        pub gp_emmc_data1: GpioPin<pac::topreg::GpEmmcData1>,
        /// JP2 pin 9 — SPI5_MOSI / EMMC_DATA0 (Arduino D16).
        pub gp_emmc_data0: GpioPin<pac::topreg::GpEmmcData0>,
        /// JP2 pin 11 — I2C0_SCL / I2C0_BCK (Arduino D15).
        pub gp_i2c0_bck: GpioPin<pac::topreg::GpI2c0Bck>,
        /// JP2 pin 12 — I2C0_SDA / I2C0_BDT (Arduino D14).
        pub gp_i2c0_bdt: GpioPin<pac::topreg::GpI2c0Bdt>,
    }

    impl Parts {
        pub fn new(_topreg: pac::Topreg) -> Self {
            // Safety: ownership of `pac::Topreg` — obtainable only via
            // `pac::Peripherals::take()` — guarantees no other code holds
            // a reference to this register block. The accessor in
            // `crate::regs::topreg()` documents the general aliasing invariant.
            let block = crate::regs::topreg();
            Self {
                gp_spi0_cs_x: unsafe { GpioPin::new(block.gp_spi0_cs_x()) },
                gp_spi0_sck: unsafe { GpioPin::new(block.gp_spi0_sck()) },
                gp_i2s1_bck: unsafe { GpioPin::new(block.gp_i2s1_bck()) },
                gp_i2s1_lrck: unsafe { GpioPin::new(block.gp_i2s1_lrck()) },
                gp_i2s1_data_in: unsafe { GpioPin::new(block.gp_i2s1_data_in()) },
                gp_i2s1_data_out: unsafe { GpioPin::new(block.gp_i2s1_data_out()) },
                gp_i2c4_bck: unsafe { GpioPin::new(block.gp_i2c4_bck()) },

                // JP1 header
                gp_uart2_txd: unsafe { GpioPin::new(block.gp_uart2_txd()) },
                gp_uart2_rxd: unsafe { GpioPin::new(block.gp_uart2_rxd()) },
                gp_uart2_rts: unsafe { GpioPin::new(block.gp_uart2_rts()) },
                gp_uart2_cts: unsafe { GpioPin::new(block.gp_uart2_cts()) },
                gp_i2s0_bck: unsafe { GpioPin::new(block.gp_i2s0_bck()) },
                gp_i2s0_lrck: unsafe { GpioPin::new(block.gp_i2s0_lrck()) },
                gp_emmc_cmd: unsafe { GpioPin::new(block.gp_emmc_cmd()) },
                gp_emmc_clk: unsafe { GpioPin::new(block.gp_emmc_clk()) },
                gp_sen_irq_in: unsafe { GpioPin::new(block.gp_sen_irq_in()) },

                // JP2 header
                gp_emmc_data3: unsafe { GpioPin::new(block.gp_emmc_data3()) },
                gp_emmc_data2: unsafe { GpioPin::new(block.gp_emmc_data2()) },
                gp_i2s0_data_in: unsafe { GpioPin::new(block.gp_i2s0_data_in()) },
                gp_i2s0_data_out: unsafe { GpioPin::new(block.gp_i2s0_data_out()) },
                gp_emmc_data1: unsafe { GpioPin::new(block.gp_emmc_data1()) },
                gp_emmc_data0: unsafe { GpioPin::new(block.gp_emmc_data0()) },
                gp_i2c0_bck: unsafe { GpioPin::new(block.gp_i2c0_bck()) },
                gp_i2c0_bdt: unsafe { GpioPin::new(block.gp_i2c0_bdt()) },
            }
        }
    }
}
