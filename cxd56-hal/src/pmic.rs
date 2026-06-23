//! PMIC control via the power-manager Far API.
//!
//! The CXD5247 companion is the board's PMIC. Its load switches and GPO pins —
//! which gate, among other things, the audio analog/digital supplies
//! ([`crate::audio_aca`]) — are programmed by asking the SYSIOP power-manager
//! firmware to run `fw_pm_pmiccontrol(cmd, arg)` over [`crate::farapi`].
//!
//! Mirrors `cxd56_pmic_set_gpo` / `cxd56_pmic_set_loadswitch`
//! (`arch/arm/src/cxd56xx/cxd56_pmic.c`) and the `board_power_control` mapping
//! (`boards/.../spresense/src/cxd56_power.c`).

use core::sync::atomic::{Ordering, compiler_fence};

use crate::farapi::{self, FarapiError};

/// Power-manager module index in the SYSIOP firmware module table — the first
/// `_modulelist_*` entry in `cxd56_farapistub.S` (`power_mgr == 0`).
const POWER_MGR_MODID: i32 = 0;

/// Far API function index for `fw_pm_pmiccontrol`. It is slot 26 of the
/// power-manager stub table in `cxd56_farapistub.S` (each slot is 4 bytes, the
/// function sits at table offset 104), and the asm stub derives the id as
/// `offset + 4`, hence 108. (The same `offset + 4` rule gives the ACA module's
/// id of 4, which is confirmed working on hardware.)
const FW_PM_PMICCONTROL_API_ID: i32 = 108;

// PMIC command codes — `enum pmic_cmd_type_e` (`cxd56_pmic.c:47`).
const PMIC_CMD_GPO: u32 = 2;
const PMIC_CMD_LOADSW: u32 = 3;

/// Issue `fw_pm_pmiccontrol(cmd, arg)`. `arg` is the address of the
/// command-specific argument struct (cast to `u32`); it must outlive the call.
/// Returns the firmware status word (`0` = OK).
fn pmiccontrol(cmd: u32, arg: u32) -> Result<u32, FarapiError> {
    // Far API arg mirrors the r0-r3 the asm stub pushes: [cmd, arg_ptr, _, _].
    let mut a = [cmd, arg, 0, 0];
    farapi::call(
        POWER_MGR_MODID,
        FW_PM_PMICCONTROL_API_ID,
        &mut a,
        farapi::DEFAULT_POLL_BUDGET,
    )?;
    Ok(a[0])
}

/// Set PMIC GPO channel(s) high or low. `chset` is a channel bitmask (bit `n` =
/// GPO`n`). Mirrors `cxd56_pmic_set_gpo`.
pub fn set_gpo(chset: u8, value: bool) -> Result<(), FarapiError> {
    // GPO register layout (cxd56_pmic.c): low nibble = level, high nibble =
    // output-enable; bytes 0/1 cover channels 0-3 / 4-7.
    let mut setbit0 = 0u8;
    let mut clrbit0 = 0u8;
    let mut setbit1 = 0u8;
    let mut clrbit1 = 0u8;

    let set = chset & 0x0f;
    if set != 0 {
        if value {
            setbit0 = (set << 4) | set;
        } else {
            setbit0 = set << 4;
            clrbit0 = set;
        }
    }
    let set = (chset >> 4) & 0x0f;
    if set != 0 {
        if value {
            setbit1 = (set << 4) | set;
        } else {
            setbit1 = set << 4;
            clrbit1 = set;
        }
    }

    /// `struct pmic_gpo_arg_s` (`cxd56_pmic.c:316`) — four in/out byte pointers.
    #[repr(C)]
    struct GpoArg {
        setbit0: *mut u8,
        clrbit0: *mut u8,
        setbit1: *mut u8,
        clrbit1: *mut u8,
    }
    let arg = GpoArg {
        setbit0: &mut setbit0,
        clrbit0: &mut clrbit0,
        setbit1: &mut setbit1,
        clrbit1: &mut clrbit1,
    };

    compiler_fence(Ordering::SeqCst);
    // Status is advisory (NuttX's board_power_control ignores it); only a
    // transport failure is propagated.
    pmiccontrol(PMIC_CMD_GPO, (&arg as *const GpoArg) as u32)?;
    Ok(())
}

/// Set PMIC load-switch channel(s) on or off. `chset` is a channel bitmask.
/// Mirrors `cxd56_pmic_set_loadswitch`.
pub fn set_loadswitch(chset: u8, value: bool) -> Result<(), FarapiError> {
    let mut setbit = if value { chset } else { 0 };
    let mut clrbit = if value { 0 } else { chset };

    /// `struct pmic_loadswitch_arg_s` (`cxd56_pmic.c:541`).
    #[repr(C)]
    struct LswArg {
        setbit: *mut u8,
        clrbit: *mut u8,
    }
    let arg = LswArg {
        setbit: &mut setbit,
        clrbit: &mut clrbit,
    };

    compiler_fence(Ordering::SeqCst);
    pmiccontrol(PMIC_CMD_LOADSW, (&arg as *const LswArg) as u32)?;
    Ok(())
}
