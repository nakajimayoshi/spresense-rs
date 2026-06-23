//! Chip-level interrupt controller (INTC) gate.
//!
//! On the CXD5602 a Sony-custom interrupt controller sits **in front of the
//! Cortex-M NVIC**: a peripheral interrupt request only reaches the NVIC if its
//! line is enabled in the INTC. So taking a real hardware peripheral interrupt
//! is a three-layer opt-in:
//!
//! 1. the peripheral's own mask (e.g. [`Timer::enable_interrupt`](crate::timer::Timer::enable_interrupt)),
//! 2. the INTC gate — [`enable`] here,
//! 3. the NVIC line — `cortex_m::peripheral::NVIC::unmask`.
//!
//! Software-pended interrupts (`NVIC::pend`) bypass the INTC and need only the
//! NVIC, which is why a pure-software interrupt demo works without this module.
//!
//! This mirrors the INTC half of NuttX's `up_enable_irq`
//! (`cxd56xx/cxd56_irq.c`); the NVIC half stays explicit in user code, matching
//! the svd2rust idiom and `esp_hal::interrupt`.
//!
//! ```ignore
//! use cortex_m::peripheral::NVIC;
//! use cxd56_hal::interrupt;
//!
//! timer.enable_interrupt();            // peripheral mask
//! interrupt::enable(timer.interrupt()); // INTC gate
//! unsafe { NVIC::unmask(timer.interrupt()) }; // NVIC line
//! ```

use crate::pac;

/// Base of the chip-level interrupt controller (`CXD56_INTC_BASE`).
const INTC_BASE: usize = 0xe004_5000;
/// First per-line enable register; `EN0..3` are one `u32` per 32 IRQs.
const INTC_EN: usize = INTC_BASE + 0x10;

/// Enable register holding `irq`'s bit (`INTC_EN(n)` in NuttX).
fn en_reg(irq: u32) -> *mut u32 {
    (INTC_EN + (((irq >> 5) << 2) as usize)) as *mut u32
}

/// Enable `interrupt` at the INTC so its requests reach the NVIC.
///
/// The peripheral mask and `NVIC::unmask(interrupt)` are still required; see the
/// [module docs](self).
pub fn enable(interrupt: pac::Interrupt) {
    let irq = interrupt as u32;
    let reg = en_reg(irq);
    let bit = 1u32 << (irq & 0x1f);
    // One enable register is shared by 32 lines, so the set is a read-modify-
    // write; guard it as NuttX guards the same write with `spin_lock_irqsave`.
    // SAFETY: `reg` is a fixed, aligned INTC enable register valid for the
    // program's lifetime; the RMW only toggles `irq`'s own bit.
    critical_section::with(|_| unsafe {
        reg.write_volatile(reg.read_volatile() | bit);
    });
}

/// Disable `interrupt` at the INTC (mirrors NuttX `up_disable_irq`).
pub fn disable(interrupt: pac::Interrupt) {
    let irq = interrupt as u32;
    let reg = en_reg(irq);
    let bit = 1u32 << (irq & 0x1f);
    // SAFETY: as in [`enable`].
    critical_section::with(|_| unsafe {
        reg.write_volatile(reg.read_volatile() & !bit);
    });
}
