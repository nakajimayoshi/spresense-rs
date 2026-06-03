//! Shared-register-block accessors.
//!
//! Several CXD5602 register blocks are reached from non-owning contexts — free
//! functions in `clocks`, pinmux helpers in `uart`/`i2c`, and `multicore::spawn`
//! — where no PAC singleton is in scope. Rather than scattering
//! `unsafe { &*pac::Foo::PTR }` at every call site (each with an implicit,
//! undocumented safety obligation), the single `unsafe` deref of each fixed MMIO
//! address lives here and is documented once.
//!
//! This mirrors the `regs()` convention already used in [`crate::multicore`]
//! (`Sph::regs`, `Mailbox::regs`) and in modern HALs (embassy's `T::regs()`,
//! esp-hal's `register_block()`).
//!
//! # Safety invariant (applies to all three functions)
//!
//! The address is the peripheral's fixed MMIO base; it is always valid.
//! Every field accessor in the `RegisterBlock` operates through
//! [`vcell::VolatileCell`] / `UnsafeCell`, so any number of aliased `&RegisterBlock`
//! references may coexist and issue reads/writes without violating Rust's aliasing
//! rules. Ordering between concurrent register accesses (e.g. the CRG `RESET`
//! register touched by both `clocks` and `multicore::spawn`) is the caller's
//! responsibility; the PMU refcount read-modify-writes already use
//! `critical_section` for that purpose.

use crate::pac;

/// CRG — clock/reset generator (`0x0201_1000`).
#[inline(always)]
pub(crate) fn crg() -> &'static pac::crg::RegisterBlock {
    unsafe { &*pac::Crg::PTR }
}

/// TOPREG — system/IO top registers (`0x0410_0000`).
///
/// Shared by the `clocks` module, GPIO pinmux (`gpio::Parts::new`), and the
/// UART/I2C pinmux helpers.
#[inline(always)]
pub(crate) fn topreg() -> &'static pac::topreg::RegisterBlock {
    unsafe { &*pac::Topreg::PTR }
}

/// TOPREG_SUB — sub-system IO registers (`0x0410_3000`).
#[inline(always)]
pub(crate) fn topreg_sub() -> &'static pac::topreg_sub::RegisterBlock {
    unsafe { &*pac::TopregSub::PTR }
}
