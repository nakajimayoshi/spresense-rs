//! PM ICC protocol — request CPU/bus operating-point changes from the SYSIOP.
//!
//! The CXD5602 SYSIOP (Cortex-M0+ running `loader.espk`) owns the PLL, root
//! mux, and bus dividers. The User Manual (§3.5.1 p.167, §3.13.3.3 p.974)
//! explicitly states they are "controlled by the API — use as RO registers."
//! The APP core requests operating-point changes over the ICC (Inter-Core
//! Comms) CPU-FIFO protocol, exactly as NuttX `cxd56_powermgr.c` does.
//!
//! Wire protocol (mirrors `cxd56_powermgr.c` / `cxd56_icc.h`):
//!
//! ```text
//! APP → SYSIOP: BOOT (once at first call, with target-id table addr)
//! APP → SYSIOP: FREQLOCK(flag)   — request HP or LP operating point
//! SYSIOP → APP: FREQLOCK ack     — lock accepted
//! SYSIOP → APP: CLK_CHG_START   — about to change; quiesce
//! APP → SYSIOP: CLK_CHG_START   — ack (ret=0)
//! [SYSIOP changes PLL/mux/dividers + PMIC voltage via CXD5247]
//! SYSIOP → APP: CLK_CHG_END     — clocks stable
//! APP → SYSIOP: CLK_CHG_END     — ack (ret=0)
//! [caller calls Crg::freeze() for updated rates]
//! ```
//!
//! # Limitations
//!
//! This implementation **polls** the CPU FIFO (blocking). Do not call
//! [`request_perf`] while other cores are actively exchanging CPU-FIFO
//! messages; non-PM FIFO messages received during the handshake are dropped.
//!
//! # References
//!
//! - NuttX: `arch/arm/src/cxd56xx/cxd56_powermgr.c`
//! - NuttX: `arch/arm/src/cxd56xx/cxd56_powermgr.h` (PM_CLOCK_* / PM_DOMAIN_* constants)
//! - NuttX: `arch/arm/include/cxd56xx/pm.h` (PM_CPUFREQLOCK_FLAG_*)
//! - NuttX: `arch/arm/src/cxd56xx/cxd56_icc.h` (CXD56_PROTO_PM, message layout)

use crate::multicore::Mailbox;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// CPU/bus performance level.
///
/// Switching between levels adjusts the APP CPU frequency AND the core voltage
/// (via the PMIC CXD5247 on the SYSIOP I2C bus); both transitions are
/// orchestrated by the SYSIOP loader firmware.
///
/// Operating points (XOSC = 26 MHz, from User Manual Table APP-807/808):
/// - **`Hp`**: APP CPU 156 MHz, VDD_CORE = 1.0 V
/// - **`Lp`**: APP CPU  39 MHz, VDD_CORE = 0.7 V
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Perf {
    /// High performance: ~156 MHz @ 1.0 V.
    Hp,
    /// Low power: ~39 MHz @ 0.7 V.
    Lp,
}

/// Error from a PM operating-point change.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PmError {
    /// The SYSIOP returned a non-zero status in `CLK_CHG_END`, indicating a
    /// firmware-side callback failure during the transition.
    ClockChangeFailed(u32),
}

// --- Protocol constants (cxd56_icc.h, cxd56_powermgr.h, pm.h) ---------------

// CXD56_PROTO_PM
const PROTO_PM: u32 = 10;

// The SYSIOP's CPU-FIFO cpuid / routing nibble.
// CXD56_PM_SYS_CPU = 0 (cxd56_powermgr.h)
const CPUID_SYSIOP: u32 = 0;

// PM message IDs carried in word0[15:0] (protodata field).
// MSGID_* (cxd56_powermgr.c:70-76 / cxd56_powermgr.h:35-41)
const MSGID_BOOT: u32 = 0;
const MSGID_FREQLOCK: u32 = 1;
const MSGID_CLK_CHG_START: u32 = 2;
const MSGID_CLK_CHG_END: u32 = 3;

// FREQLOCK flag bits (arch/arm/include/cxd56xx/pm.h:70-72).
const FLAG_INITIALIZED: u32 = 0x8000; // always set; == PM_CPUFREQLOCK_FLAG_HOLD
const FLAG_HV: u32 = 0x0001; // PM_CPUFREQLOCK_FLAG_HV — request high voltage / HP
const FLAG_LV: u32 = 0x4000; // PM_CPUFREQLOCK_FLAG_LV — request low voltage / LP

// --- Boot handshake ----------------------------------------------------------

// Placeholder cxd56_pm_target_id_s (6 × u32, all zero).
// The SYSIOP reads this address on BOOT to learn the APP's domain-callback
// configuration. Zero means no per-domain callbacks are registered; the
// CLK_CHG handshake still proceeds normally.
static TARGET_ID_TABLE: [u32; 6] = [0; 6];

static BOOTED: AtomicBool = AtomicBool::new(false);

/// Mirrors NuttX `g_freqlock_flag`: the last aggregate FREQLOCK flag sent to the
/// SYSIOP (`0` = none sent yet). `cxd56_pm_checkfreqlock` only messages the
/// SYSIOP when this value changes; the SYSIOP acks (and acts on) a FREQLOCK only
/// for a real change, so re-sending an unchanged flag would block forever.
static LAST_FLAG: AtomicU32 = AtomicU32::new(0);

fn boot_once() {
    if BOOTED
        .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        // cxd56_pm_bootup: cxd56_pmsendmsg(MSGID_BOOT, &g_target_id_table)
        send_pm(MSGID_BOOT, TARGET_ID_TABLE.as_ptr() as u32);
    }
}

// --- Wire-format helpers -----------------------------------------------------

// CPU-FIFO word layout (iccmsg_msg_s, little-endian):
//   word0[31:28] = cpuid  (4 bits) — destination on TX; sender on RX
//   word0[27:24] = proto  (4 bits) — CXD56_PROTO_PM = 10
//   word0[23:16] = msgid  (8 bits) — unused by PM (always 0)
//   word0[15:0]  = pdata (16 bits) — PM message ID (FREQLOCK/CLK_CHG/BOOT)
//   word1        = data  (32 bits) — payload (flag / ret / table ptr)

fn pack_pm(proto_data: u32, data: u32) -> [u32; 2] {
    let word0 = (CPUID_SYSIOP << 28) | (PROTO_PM << 24) | (proto_data & 0xffff);
    [word0, data]
}

// Returns Some((proto_data, data)) if the received words are a PM message.
fn unpack_pm(words: [u32; 2]) -> Option<(u32, u32)> {
    if (words[0] >> 24) & 0xf == PROTO_PM {
        Some((words[0] & 0xffff, words[1]))
    } else {
        None
    }
}

fn send_pm(proto_data: u32, data: u32) {
    Mailbox::send_blocking(pack_pm(proto_data, data));
}

// --- Public API --------------------------------------------------------------

/// Request a CPU/bus operating-point change from the SYSIOP loader firmware.
///
/// Sends `FREQLOCK` over the ICC CPU-FIFO and returns once the SYSIOP **acks the
/// lock** — the same completion signal NuttX waits on: `cxd56_pm_checkfreqlock`
/// sends `MSGID_FREQLOCK` then `nxsem_wait(&g_freqlockwait)`, and the semaphore
/// is posted by the FREQLOCK ack in `cxd56_pmmsghandler`.
///
/// The clock change proceeds on the SYSIOP side. The `CLK_CHG_START`/`CLK_CHG_END`
/// handshake exists only to quiesce **registered domain callbacks**
/// (`cxd56_pm_do_callback`) and is serviced asynchronously by NuttX's PM task —
/// `up_pm_acquire_freqlock` does **not** wait for it. With our empty target-id
/// table (no callbacks) the SYSIOP omits that handshake entirely, so waiting for
/// `CLK_CHG_END` here would block forever. We still ack `CLK_CHG_START`/`END` if
/// they do arrive (callbacks registered), and surface a non-zero `CLK_CHG_END`
/// status as an error.
///
/// Mirrors `cxd56_pm_checkfreqlock`'s `if (g_freqlock_flag != flag)` guard: a
/// request whose flag matches the last one sent is a no-op (the SYSIOP neither
/// acks nor acts on an unchanged FREQLOCK) and returns immediately.
///
/// After this returns `Ok(())`, call [`crate::clocks::Crg::freeze`] for updated
/// rates.
///
/// # Errors
///
/// Returns [`PmError::ClockChangeFailed`] if the SYSIOP reports a non-zero status
/// in a `CLK_CHG_END` message (a firmware-side callback failure).
pub fn request_perf(perf: Perf) -> Result<(), PmError> {
    boot_once();

    let flag = FLAG_INITIALIZED
        | match perf {
            Perf::Hp => FLAG_HV,
            Perf::Lp => FLAG_LV,
        };

    // cxd56_pm_checkfreqlock: only message the SYSIOP when the flag changes.
    // Re-sending an unchanged flag yields no ack and would hang the recv below.
    if LAST_FLAG.swap(flag, Ordering::Relaxed) == flag {
        return Ok(());
    }

    // cxd56_pmsendmsg(MSGID_FREQLOCK, flag)
    send_pm(MSGID_FREQLOCK, flag);

    // Completion is the FREQLOCK ack (NuttX returns from the g_freqlockwait post
    // here). Ack the optional CLK_CHG handshake if the SYSIOP drives it; do not
    // require CLK_CHG_END, which never comes with an empty target-id table.
    loop {
        let words = Mailbox::recv_blocking();
        let Some((proto_data, data)) = unpack_pm(words) else {
            continue;
        };
        match proto_data {
            // SYSIOP ack — the lock is accepted and the request is complete.
            MSGID_FREQLOCK => return Ok(()),
            MSGID_CLK_CHG_START => {
                // cxd56_pmsendmsg(MSGID_CLK_CHG_START, 0 /*ret=OK*/)
                send_pm(MSGID_CLK_CHG_START, 0);
            }
            MSGID_CLK_CHG_END => {
                // cxd56_pmsendmsg(MSGID_CLK_CHG_END, 0 /*ret=OK*/)
                send_pm(MSGID_CLK_CHG_END, 0);
                if data != 0 {
                    return Err(PmError::ClockChangeFailed(data));
                }
            }
            _ => {}
        }
    }
}
