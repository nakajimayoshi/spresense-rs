//! Multicore-safe [`critical_section`] implementation for the CXD5602.
//!
//! Adapted from `rp2040-hal`'s spinlock-based implementation.  The design is
//! identical in structure; the CXD5602-specific substitutions are:
//!
//! | rp2040 | CXD5602 |
//! |---|---|
//! | `Spinlock31::try_claim()` + `mem::forget` | `sph::raw_try_lock(CS_SPH_ID)` |
//! | `Spinlock31::release()` | `sph::raw_unlock(CS_SPH_ID)` |
//! | `Sio::core() as u8 + 1` (values 1/2) | `cpu::raw_pid()` (values 2..=7) |
//!
//! # Token encoding
//!
//! The `RawRestoreState` token (enabled as `u8` by the `restore-state-u8`
//! feature of `critical-section`) carries:
//!
//! - `0` — outermost acquire; interrupts were **disabled** on entry; do not
//!   re-enable on release.
//! - `1` — outermost acquire; interrupts were **enabled** on entry; re-enable
//!   on release.
//! - `2` (`LOCK_ALREADY_OWNED`) — recursive call; this core already held the
//!   section when `acquire` was called; `release` must do nothing.
//!
//! # Correctness notes
//!
//! **SPH owns the cross-core mutex.** `LOCK_OWNER` only supports two things:
//! recursion detection (each core compares the stored value against its own
//! `raw_pid`, which can only match if *that* core wrote it) and deciding
//! whether to release the SPH in `release`.  Because every core writes
//! `LOCK_OWNER` only while it holds the SPH (single writer at a time), and
//! reads it only before attempting to win the SPH, cross-core coherence of
//! `LOCK_OWNER` is not load-bearing.  (The APP cores have no data cache, so
//! there is no coherence/visibility problem; that is a separate concern from
//! the *ordering* one below.)
//!
//! **Memory barriers are required.** The data a critical section protects lives
//! in SRAM (Normal memory); the SPH lives in the peripheral region (Device
//! memory). On ARMv7-M a Normal access and a Device access to different regions
//! may be observed out of order unless separated by a `DMB`, and "no cache"
//! does **not** excuse this on a multi-core part (per ARM DAI0321A the
//! cache-free exemption applies only to single-core processors). So `acquire`
//! issues a `dmb` after winning the SPH (the caller's protected accesses must be
//! ordered after the lock) and `release` issues one before unlocking it (the
//! caller's protected stores must be globally visible before the lock frees).
//! `compiler_fence` alone is insufficient — it constrains only the compiler, not
//! the memory system / store buffer. The Spresense SDK does the same in its own
//! SPH-based SMP lock (`cxd56_testset.c`: `SP_DMB()` before the SPH unlock).
//!
//! **LDREX/STREX do not work cross-core** on the CXD5602 (exclusive monitors
//! are core-local).  The SPH block is the only sound cross-core lock primitive.
//!
//! # Cargo feature
//!
//! Gate-kept by the `critical-section-impl` feature of `cxd56-hal`.  It is
//! mutually exclusive with any other `critical_section::set_impl!` in the same
//! binary (the linker emits a duplicate-symbol error if two impls are linked).
//! In particular, remove `cortex-m`'s `critical-section-single-core` feature
//! from any crate that links `cxd56-hal` with `critical-section-impl`.

use core::sync::atomic::{compiler_fence, AtomicU8, Ordering};

use crate::multicore::cpu;
use crate::multicore::sph;

/// Hardware semaphore index reserved for the `critical_section` implementation.
///
/// SPH 15 (the highest) mirrors rp2040-hal's choice of Spinlock31.  No other
/// code in this HAL claims SPH 15; user code must not claim it either while
/// `critical-section-impl` is active.
pub const CS_SPH_ID: usize = sph::RESERVED_CS_ID;

/// Marker: no core currently holds the section.  Must not equal any valid
/// `raw_pid` (which is `core_index + 2`, range 2..=7).
const LOCK_UNOWNED: u8 = 0;

/// Token value indicating the calling core already owned the critical section
/// when `acquire` was entered (recursive call).  `release` must not release
/// the SPH or restore interrupt state.
const LOCK_ALREADY_OWNED: u8 = 2;

/// Stores the `raw_pid` of the core that currently holds the section, or
/// `LOCK_UNOWNED` (0) when free.
static LOCK_OWNER: AtomicU8 = AtomicU8::new(LOCK_UNOWNED);

struct Cxd56SphCs;
critical_section::set_impl!(Cxd56SphCs);

unsafe impl critical_section::Impl for Cxd56SphCs {
    unsafe fn acquire() -> u8 {
        unsafe { Cxd56SphCs::acquire() }
    }
    unsafe fn release(token: u8) {
        unsafe { Cxd56SphCs::release(token) }
    }
}

impl Cxd56SphCs {
    unsafe fn acquire() -> u8 {
        let interrupts_active = cortex_m::register::primask::read().is_active();
        let core = cpu::raw_pid();

        if LOCK_OWNER.load(Ordering::Acquire) == core {
            // Recursive call — we already own the SPH; tell release to do nothing.
            return LOCK_ALREADY_OWNED;
        }

        loop {
            // Disable interrupts before attempting the SPH so that an interrupt
            // handler cannot try to enter a critical section after we win the
            // SPH but before we record ownership — which would deadlock.
            cortex_m::interrupt::disable();
            // Prevent the compiler from moving memory accesses across this point.
            compiler_fence(Ordering::SeqCst);

            if sph::raw_try_lock(CS_SPH_ID) {
                // Won arbitration.  Record ownership so recursive acquires on
                // this core are detected without re-entering the SPH spin loop.
                LOCK_OWNER.store(core, Ordering::Relaxed);
                // Order the lock acquisition (Device access) before the caller's
                // protected accesses (Normal memory).  Without this `dmb` those
                // accesses may be observed before the lock is held on a multi-core
                // ARMv7-M part.  See the module header; `compiler_fence` is not
                // enough — it does not emit a hardware barrier.
                cortex_m::asm::dmb();
                break;
            }

            // Lost arbitration.  Re-enable interrupts (if they were on) so
            // this core does not starve interrupt service while spinning.
            if interrupts_active {
                // SAFETY: restoring the state that existed before we disabled.
                unsafe { cortex_m::interrupt::enable() };
            }
            // Hint to the core that we are in a spin-wait loop.
            core::hint::spin_loop();
        }

        // Return 0 or 1 (disabled / enabled) so `release` knows whether to
        // restore interrupts.
        interrupts_active as u8
    }

    unsafe fn release(token: u8) {
        if token != LOCK_ALREADY_OWNED {
            // Outermost release.  Clear ownership before unlocking the SPH so
            // a core that wins the SPH immediately after sees LOCK_UNOWNED.
            LOCK_OWNER.store(LOCK_UNOWNED, Ordering::Relaxed);
            // Make the caller's protected stores (Normal memory) globally visible
            // before the SPH unlock (Device memory) is observed; otherwise a core
            // that wins the SPH next could read stale data on a multi-core ARMv7-M
            // part.  The `compiler_fence` only stops *compiler* reordering, so the
            // `dmb` is the load-bearing barrier here.  See the module header.
            compiler_fence(Ordering::SeqCst);
            cortex_m::asm::dmb();
            sph::raw_unlock(CS_SPH_ID);

            if token != 0 {
                // Interrupts were active when `acquire` was called; restore them.
                // SAFETY: restoring the state captured at acquire time.
                unsafe { cortex_m::interrupt::enable() };
            }
        }
        // If token == LOCK_ALREADY_OWNED: recursive — do nothing.
    }
}
