//! Hardware semaphores (SPH) — the cross-core lock primitive.
//!
//! The Cortex-M4 `LDREX`/`STREX` exclusive monitors are local to each core and
//! do **not** provide mutual exclusion *across* cores, so `core::sync::atomic`
//! compare-exchange cannot build a cross-core lock. The CXD5602 instead exposes
//! 16 hardware test-and-set semaphores at `0x4600_c800` (the [`pac::Sph`]
//! peripheral). Each is a 16-byte slot with a write-only `REQ` command register
//! and a read-only `STS` status register. Mirrors `cxd56_sph.c`.
//!
//! These are global hardware shared by every core, so the driver accesses them
//! through [`pac::Sph::PTR`] rather than owning a singleton handle.
//!
//! [`Sph<N>`] is a zero-size, const-generic token; the index lives in the type
//! and is validated at compile time. `Sph` is only the raw lock primitive —
//! building a data-guarding mutex on top of it is left to downstream consumers.
//! The lock/unlock operations imply **no memory barrier**: a consumer that
//! guards Normal-memory data must add a `cortex_m::asm::dmb()` after locking and
//! before unlocking (Normal-vs-Device accesses may otherwise reorder on
//! multi-core ARMv7-M — the `critical_section_impl` module is the reference
//! pattern).

use super::cpu;
use crate::pac;
use core::marker::PhantomData;

/// Number of hardware semaphores.
pub const COUNT: usize = 16;

/// SPH index reserved by the `critical-section` impl (mirrors rp2040 Spinlock31).
///
/// Rejected at compile time by [`Sph`] when the `critical-section-impl` feature
/// is enabled; the impl itself reaches the slot through the unchecked
/// [`raw_try_lock`]/[`raw_unlock`] helpers.
pub const RESERVED_CS_ID: usize = 15;

// REQ command field (`REQ[1:0]`).
const REQ_UNLOCK: u32 = 0;
const REQ_LOCK: u32 = 1;
#[allow(dead_code)]
const REQ_RESERVE: u32 = 2;
#[allow(dead_code)]
const REQ_INTRCLR: u32 = 3;

#[inline]
fn regs() -> &'static pac::sph::RegisterBlock {
    // SAFETY: SPH is a memory-mapped peripheral shared by all cores; we only
    // issue single-register reads/writes with no aliasing requirements.
    unsafe { &*pac::Sph::PTR }
}

/// Test-and-set semaphore `n` without spinning. Returns `true` iff THIS core now
/// holds it.
///
/// Unchecked in the index — callers must ensure `n < COUNT`. This is the
/// privileged path used by both [`Sph::try_lock`] and the `critical_section`
/// impl (which needs [`RESERVED_CS_ID`], the slot [`Sph`] rejects).
#[inline]
pub(crate) fn raw_try_lock(n: usize) -> bool {
    let sph = regs();
    sph.req(n).write(|w| unsafe { w.bits(REQ_LOCK) });
    // The owner field records the ADSP master id (= core index + 2). If the
    // semaphore was free, our LOCK request set the owner to us; if it was
    // already held, the request is ignored and the owner is unchanged.
    sph.sts(n).read().lock_owner().bits() == cpu::raw_pid()
}

/// Release semaphore `n`. Only meaningful if this core currently holds it.
/// Unchecked in the index.
#[inline]
pub(crate) fn raw_unlock(n: usize) {
    regs().req(n).write(|w| unsafe { w.bits(REQ_UNLOCK) });
}

/// The raw ADSP id of the core currently holding semaphore `n`, or `None` if it
/// is idle. Unchecked in the index.
#[inline]
pub(crate) fn raw_owner(n: usize) -> Option<u8> {
    let s = regs().sts(n).read();
    if s.state().bits() == 0 {
        None
    } else {
        Some(s.lock_owner().bits())
    }
}

/// A handle to hardware semaphore `N` (`0..16`).
///
/// `Sph<N>` is a zero-size, `Copy` token; the index lives in the type. Multiple
/// cores — and multiple call sites on one core — may hold a token to the same
/// semaphore; mutual exclusion is enforced by the hardware, not by Rust
/// ownership.
///
/// `N` is validated at **compile time**: any use of `Sph::<N>` fails to compile
/// if `N >= 16`, or if `N` equals [`RESERVED_CS_ID`] while the
/// `critical-section-impl` feature is enabled.
#[derive(Copy, Clone, Debug)]
pub struct Sph<const N: usize>(PhantomData<()>);

impl<const N: usize> Sph<N> {
    /// Compile-time index check. Every associated fn below binds it
    /// (`let () = Self::VALID;`), forcing evaluation at monomorphization so an
    /// invalid `N` is a hard compile error rather than a runtime panic.
    const VALID: () = {
        assert!(N < COUNT, "SPH index out of range (must be < 16)");
        #[cfg(feature = "critical-section-impl")]
        assert!(
            N != RESERVED_CS_ID,
            "this SPH index is reserved by critical-section-impl; pick another"
        );
    };

    /// Bind to hardware semaphore `N`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        let () = Self::VALID;
        Sph(PhantomData)
    }

    /// Attempt to acquire the semaphore without spinning.
    ///
    /// Issues a `LOCK` request and checks whether this core won arbitration by
    /// comparing the recorded owner against this core's raw ADSP id. Returns
    /// `true` iff this core now holds the lock.
    ///
    /// The hardware ignores a redundant `LOCK` from a core that already holds the
    /// slot, so a same-core re-lock also returns `true`. Code building a mutex on
    /// top of `Sph` must account for this: the lock is **not** reentrant, so a
    /// second guard would alias the first.
    #[inline]
    pub fn try_lock(self) -> bool {
        let () = Self::VALID;
        raw_try_lock(N)
    }

    /// Spin until the semaphore is acquired.
    #[inline]
    pub fn lock(self) {
        while !self.try_lock() {
            core::hint::spin_loop();
        }
    }

    /// Release the semaphore. Only meaningful if this core currently holds it.
    #[inline]
    pub fn unlock(self) {
        let () = Self::VALID;
        raw_unlock(N);
    }

    /// The raw ADSP id of the core currently holding the lock, or `None` if the
    /// semaphore is idle.
    #[inline]
    pub fn owner(self) -> Option<u8> {
        let () = Self::VALID;
        raw_owner(N)
    }
}
