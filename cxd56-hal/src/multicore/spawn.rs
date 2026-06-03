//! Spawning APP-domain worker cores (single combined-image model).
//!
//! All cores run from one binary image with multiple entry points (embassy-rp
//! style — no ELF loader). The main core ([`Core::Core0`]) brings up a worker
//! by writing its initial stack pointer and reset vector into the shared boot
//! mailbox at `0x0d00_0000`/`0x0d00_0004`, replicating the booting core's
//! address-converter view, then releasing the worker's reset and clock gate.
//! Mirrors `up_cpu_start` (`cxd56_cpustart.c`).
//!
//! Because the boot mailbox is a single shared location, only one worker may be
//! in flight at a time: a freshly-started worker must call [`ack_boot`] as its
//! first action so the spawner knows the mailbox is free for the next core
//! (mirrors the `spin_unlock(&g_appdsp_boot)` handshake in `appdsp_boot`).
//!
//! Spawn must be driven from a single core (normally `Core0`); concurrent
//! spawns from different cores are not supported.

use super::cpu::Core;
use crate::regs::crg;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

/// Base of the ADSP shared SRAM / boot mailbox (`CXD56_ADSP_RAM_BASE`).
const ADSP_RAM_BASE: usize = 0x0d00_0000;
/// Worker initial stack pointer (`VECTOR_ISTACK`).
const VECTOR_ISTACK: *mut u32 = ADSP_RAM_BASE as *mut u32;
/// Worker reset vector / entry point (`VECTOR_RESETV`).
const VECTOR_RESETV: *mut u32 = (ADSP_RAM_BASE + 4) as *mut u32;
/// Per-core address-converter table (APP-local view, `CXD56_ACNV_P0_DST0`).
/// 12 tiles of 4 bytes each; each core's table is `0x20` apart.
const ACNV_P0_DST0: usize = 0x0e01_2004;
const ACNV_TILES: usize = 12;
const ACNV_CPU_STRIDE: usize = 0x20;

/// Handshake flag: set by a worker via [`ack_boot`], cleared by the spawner.
static BOOT_ACK: AtomicBool = AtomicBool::new(false);

/// Errors from [`spawn`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpawnError {
    /// `Core0` is the main core and cannot be spawned.
    MainCore,
    /// The worker did not call [`ack_boot`] within the spin budget.
    Timeout,
}

/// Start an APP-domain worker `core` running `entry` on a stack ending at
/// `stack_top` (the highest address; the SP grows down from here).
///
/// The worker's `entry` must call [`ack_boot`] as early as possible, then set up
/// its own vector table / FPU as needed and never return.
///
/// # Safety
/// - `stack_top` must point just past a stack region in memory both cores can
///   access (the shared combined image's RAM), exclusively owned by `core`.
/// - `entry` must be a valid worker entry that never returns.
/// - Must be called from a single core; spawns are serialised only against each
///   other on the calling core, via the [`ack_boot`] handshake.
pub unsafe fn spawn(
    core: Core,
    stack_top: u32,
    entry: extern "C" fn() -> !,
) -> Result<(), SpawnError> {
    if core == Core::Core0 {
        return Err(SpawnError::MainCore);
    }
    let cpu = core.index() as u32;
    let bit = 1u32 << (16 + cpu);

    BOOT_ACK.store(false, Ordering::Release);

    // 1. Hold the worker in reset (active-low: clear bit 16+cpu).
    let r = crg().reset().read().bits();
    crg().reset().write(|w| unsafe { w.bits(r & !bit) });

    // 2. Write the worker's initial stack and reset vector into the boot mailbox.
    unsafe {
        ptr::write_volatile(VECTOR_ISTACK, stack_top);
        ptr::write_volatile(VECTOR_RESETV, entry as usize as u32);
    }

    // 3. Clock supply, then stop (boot-prep pulse).
    let g = crg().ck_gate_ahb().read().bits();
    crg().ck_gate_ahb().write(|w| unsafe { w.bits(g | bit) });
    crg().ck_gate_ahb().write(|w| unsafe { w.bits(g & !bit) });

    // 4. Replicate the booting core's address-converter view to the worker so it
    //    sees the same flat memory map (single combined image).
    for i in 0..ACNV_TILES {
        let src = (ACNV_P0_DST0 + 4 * i) as *const u32;
        let dst = (ACNV_P0_DST0 + 4 * i + (cpu as usize) * ACNV_CPU_STRIDE) as *mut u32;
        unsafe { ptr::write_volatile(dst, ptr::read_volatile(src)) };
    }

    // 5. Release reset (set bit) and supply the clock.
    let r = crg().reset().read().bits();
    crg().reset().write(|w| unsafe { w.bits(r | bit) });
    let g = crg().ck_gate_ahb().read().bits();
    crg().ck_gate_ahb().write(|w| unsafe { w.bits(g | bit) });

    // 6. Wait for the worker to report it has consumed the boot mailbox.
    let mut budget = 5_000_000u32;
    while !BOOT_ACK.load(Ordering::Acquire) {
        if budget == 0 {
            return Err(SpawnError::Timeout);
        }
        budget -= 1;
        core::hint::spin_loop();
    }

    Ok(())
}

/// Called by a freshly-started worker core as its **first** action, to release
/// the spawner so the shared boot mailbox can be reused for the next core.
/// Mirrors `spin_unlock(&g_appdsp_boot)` in `appdsp_boot` (cxd56_cpustart.c).
#[inline]
pub fn ack_boot() {
    BOOT_ACK.store(true, Ordering::Release);
}
