//! FARAPI — remote-procedure calls into the SYSIOP loader firmware.
//!
//! Some CXD5602 peripherals are not driven by the APP core directly; instead
//! the APP core asks the SYSIOP Cortex-M0+ (running `loader.espk`, already
//! flashed on the board) to run a firmware routine on its behalf. NuttX calls
//! these `fw_*` stubs "Far API". The audio analog companion (CXD5247 / ACA)
//! bring-up — [`crate::audio_aca`] — is reached this way: `fw_as_acacontrol`.
//!
//! # How NuttX does it, and what we replicate
//!
//! NuttX's `fw_*` symbols are tiny asm stubs (`cxd56_farapistub.S`) that capture
//! an API index from their own PC and jump to `farapi_main`
//! (`cxd56_farapi.c:181`). `farapi_main` fills a `farmsg` on the stack, hands
//! its **address** to the SYSIOP over the CPU-FIFO mailbox using the `MBX`
//! protocol, and blocks until the SYSIOP signals completion with the `FLG`
//! protocol. The SYSIOP reads the argument buffer the message points at, runs
//! the routine, and writes the return value back into the first word of that
//! buffer.
//!
//! This module is a faithful bare-metal port of `farapi_main` over the existing
//! [`Mailbox`]. The one piece NuttX gets from the linker — the per-module
//! `modid` (index into the `.modulelist` section) and the `cpuno`/`mbxid` (both
//! statically `0` for every module in `cxd56_farapistub.S`) — we pass in as
//! plain constants, so no special linker section or loader-patched weak symbol
//! is needed. See [`crate::audio_aca`] for the ACA module's `modid`/api id.
//!
//! # Wire format (mirrors `cxd56_icc.c` `struct iccmsg_msg_s`, little-endian)
//!
//! ```text
//! word0: [31:28] cpuid  [27:24] proto  [23:16] msgid  [15:0] protodata
//! word1: data
//! ```
//!
//! # Polling, not interrupts
//!
//! Like [`crate::clocks::pm`], this polls the CPU FIFO and drops any unrelated
//! messages it sees while waiting. Do not issue a Far API call concurrently with
//! other CPU-FIFO traffic.

use core::sync::atomic::{Ordering, compiler_fence};

use crate::multicore::Mailbox;
use crate::multicore::cpu;

// --- ICC protocol ids (cxd56_icc.h) -----------------------------------------

const PROTO_MBX: u32 = 1;
const PROTO_FLG: u32 = 3;

/// The SYSIOP core is CPU 0 — most `_modulelist_*` entries in
/// `cxd56_farapistub.S` have `cpuno == 0` (power_mgr, flash_mgr, aca, …). A few
/// run on a different core: the `gnss` module list has `cpuno == 1` (the GNSS
/// DSP), reached with [`call_on_cpu`].
const CPUID_SYSIOP: u32 = 0;

/// `mbxid` is `0` for every module in `cxd56_farapistub.S`.
const MBXID: i16 = 0;

/// Low nibble of `flagid` — the "magic. not zero" 7 from `farapi_main`
/// (`api->flagid = (cpuid + 1) << 8 | 7`). The SYSIOP echoes it back in the
/// completion `FLG` message, which is how we recognise our own done event.
const FLAG_MAGIC: u32 = 7;

/// Default completion budget. A Far API round-trip is sub-millisecond when the
/// SYSIOP firmware exposes the module; this bounds the wait well past that so a
/// **missing** module (no reply) fails as [`FarapiError::Timeout`] instead of
/// hanging the core forever — the whole point of the ACA availability gate.
pub const DEFAULT_POLL_BUDGET: u32 = 5_000_000;

/// Error from a Far API call.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FarapiError {
    /// The SYSIOP never sent a completion event within the poll budget —
    /// typically the requested module is not present in the loader firmware.
    Timeout,
}

/// NuttX `struct apimsg_s` (`cxd56_farapi.c:73`). `#[repr(C)]` so its layout
/// matches what the SYSIOP firmware expects.
#[repr(C)]
struct ApiMsg {
    id: i32,
    arg: *mut u32,
    mbxid: i16,
    flagid: i16,
    flagbitno: i32,
}

/// NuttX `struct farmsg_s` (`cxd56_farapi.c:94`). The leading `head.next`
/// pointer is part of the firmware-visible layout; we zero it.
#[repr(C)]
struct FarMsg {
    next: *mut u32,
    cpuid: i32,
    modid: i32,
    api: ApiMsg,
}

#[inline]
fn pack_word0(dest_cpuid: u32, proto: u32, msgid: u32, pdata: u32) -> u32 {
    ((dest_cpuid & 0xf) << 28) | ((proto & 0xf) << 24) | ((msgid & 0xff) << 16) | (pdata & 0xffff)
}

/// Issue one Far API call to a SYSIOP module and block until it completes.
///
/// `modid` is the module's index in the firmware module table; `api_id` is the
/// function index the NuttX asm stub would have derived from its PC; `arg` is
/// the argument buffer the firmware reads (`arg[0]` = command on entry) and
/// writes (`arg[0]` = return value on completion). `arg` must outlive the call
/// and hold at least the words the firmware touches (4 is always safe — it
/// mirrors the `r0-r3` the asm stub pushes).
///
/// On `Ok(())`, read the firmware return value from `arg[0]`.
///
/// Routes to the SYSIOP (CPU 0), which hosts most modules. For a module whose
/// `_modulelist_*` entry has a non-zero `cpuno` (the `gnss` module runs on the
/// GNSS DSP, `cpuno == 1`), use [`call_on_cpu`] with that core's id.
pub fn call(modid: i32, api_id: i32, arg: &mut [u32], budget: u32) -> Result<(), FarapiError> {
    call_on_cpu(CPUID_SYSIOP, modid, api_id, arg, budget)
}

/// Like [`call`], but sends the request to `dest_cpuid` — the `cpuno` of the
/// target module's `_modulelist_*` entry. Use `0` for SYSIOP modules (or just
/// call [`call`]); use [`crate::gnss::GNSS_CPUID`] (1) for the `gnss` module.
///
/// The completion `FLG` is matched and acknowledged generically (back to
/// whichever core sent it), so only the request destination differs.
pub fn call_on_cpu(
    dest_cpuid: u32,
    modid: i32,
    api_id: i32,
    arg: &mut [u32],
    budget: u32,
) -> Result<(), FarapiError> {
    call_on_cpu_with_unrelated(dest_cpuid, modid, api_id, arg, budget, |_, _| {})
}

/// Like [`call_on_cpu`], but invokes `on_unrelated` for every FIFO message that
/// is not this call's FLG completion.
///
/// NuttX uses an interrupt dispatcher, so unrelated messages can still reach
/// their protocol handlers while `farapi_main` blocks. Polling users that need
/// that behavior, such as GNSS boot, can provide a small dispatcher here.
pub fn call_on_cpu_with_unrelated(
    dest_cpuid: u32,
    modid: i32,
    api_id: i32,
    arg: &mut [u32],
    budget: u32,
    mut on_unrelated: impl FnMut(u32, u32),
) -> Result<(), FarapiError> {
    // `cpuid` of the *caller* — `getreg32(CPU_ID)` in NuttX, which is this
    // core's ADSP id (index + 2).
    let cpuid = cpu::raw_pid() as i32;

    let mut msg = FarMsg {
        next: core::ptr::null_mut(),
        cpuid,
        modid,
        api: ApiMsg {
            id: api_id,
            arg: arg.as_mut_ptr(),
            mbxid: MBXID,
            // api->flagid = (cpuid + 1) << 8 | 7
            flagid: (((cpuid + 1) << 8) | FLAG_MAGIC as i32) as i16,
            flagbitno: 0,
        },
    };

    // The SYSIOP reads `msg` and `arg` out of our RAM; make sure every field is
    // committed before we hand over the pointer, and that the firmware's writes
    // to `arg` are observed only after completion.
    compiler_fence(Ordering::SeqCst);

    // Send request: cxd56_sendmsg(cpuno, PROTO_MBX, msgtype=4, pdata=1<<8|1,
    // &msg). msgid = msgtype << 4 = 0x40.
    let req_w0 = pack_word0(dest_cpuid, PROTO_MBX, 4 << 4, (1 << 8) | 1);
    Mailbox::send_blocking([req_w0, (&mut msg as *mut FarMsg) as u32]);

    // Wait for the FLG completion event (`pdata & 0xf == 7`), then acknowledge
    // it exactly as NuttX's `cxd56_farapidonehandler` does.
    for _ in 0..budget {
        let Some([w0, _w1]) = Mailbox::try_recv() else {
            core::hint::spin_loop();
            continue;
        };
        let proto = (w0 >> 24) & 0xf;
        if proto != PROTO_FLG || (w0 & 0xf) != FLAG_MAGIC {
            // Unrelated FIFO traffic (e.g. a stray MBX) — drop and keep waiting.
            on_unrelated(w0, _w1);
            continue;
        }
        // Send event-flag response: cxd56_sendmsg(sender, PROTO_FLG, msgtype=5,
        // pdata = received & 0xff00, 0). msgid = 5 << 4 = 0x50.
        let sender = (w0 >> 28) & 0xf;
        let ack_w0 = pack_word0(sender, PROTO_FLG, 5 << 4, w0 & 0xff00);
        Mailbox::send_blocking([ack_w0, 0]);

        compiler_fence(Ordering::SeqCst);
        return Ok(());
    }

    Err(FarapiError::Timeout)
}
