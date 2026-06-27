//! GNSS positioning via the `gnssfw` firmware on the GNSS DSP core.
//!
//! There are **no GNSS registers to poke**. The receiver runs as proprietary
//! firmware (`gnssfw`) on a separate internal DSP core (the GPS CPU, routing id
//! [`GNSS_CPUID`] = 1). The APP core drives it as an RPC client over the on-chip
//! CPU-FIFO mailbox. Two channels reach the firmware (mirroring NuttX
//! `cxd56_gnss.c`):
//!
//! * **Far API** ([`crate::farapi`]) — synchronous verbs marshalled to the GNSS
//!   module (`_modulelist_gnss`, [`ids::GNSS_MODID`] = 8, which runs on CPU 1).
//!   Used here for satellite-system selection and reading the fix buffer.
//! * **CPU1 signal channel** — the async path. Start/stop are issued as
//!   `CPUFIFOAPI` commands sent with proto [`PROTO_GNSS`]; the firmware delivers
//!   `BOOTCOMP` / `POSITION` / backup-request notifications **back** as generic
//!   [`PROTO_MSG`] frames tagged with the DSP's source id (see [`PROTO_MSG`]).
//!
//! Like [`crate::farapi`] / [`crate::clocks::pm`], this is **polling only**: it
//! spins on the CPU FIFO rather than taking the `FIFO_FROM` interrupt (see
//! [`crate::multicore`]). Do not run another CPU-FIFO user concurrently while a
//! GNSS call is in flight.
//!
//! # Bring-up sequence (mirrors NuttX `cxd56_gnss_open` / `_start`)
//!
//! 1. [`Gnss::boot`] — power the TCXO/LNA rail, `fw_pm_loadimage(1,"gnssfw")` →
//!    `fw_pm_startcpu(1,1)`, then wait for the firmware's `BOOTCOMP` signal.
//! 2. [`Gnss::select_satellites`] — `fw_gd_selectsatellitesystem`: choose the
//!    constellations ([`SatelliteSelection::Manual`]) or take the
//!    [`SatelliteSelection::Automatic`] default ([`AUTOMATIC`]).
//! 3. [`Gnss::start`] — `GD_GNSS_START` over the signal channel begins positioning.
//! 4. Poll [`Gnss::read_fix`]; once the firmware has a fix, [`Fix::is_valid`] is
//!    true and latitude/longitude/altitude are populated.
//! 5. [`Gnss::stop`] when done.
//!
//! # "Satellites": constellations, not space vehicles
//!
//! On this chip you do not address individual satellites — you select satellite
//! **systems** (GPS, GLONASS, QZSS, …) via [`SatelliteSystem`]. The live count of
//! space vehicles actually used for a fix is reported back per-fix in
//! [`Fix::num_satellites`]. A 3D fix still needs ≥4 space vehicles in view at
//! runtime regardless of how many systems you enable.
//!
//! # Constants are coupled to the flashed loader
//!
//! The Far API indices ([`ids`]), the signal-channel encoding, and the
//! [`PositionData`] layout are all taken verbatim from the Spresense SDK
//! (`cxd56_farapistub.S`, `cxd56_cpu1signal.*`, `gnss_type.h`) and are tied to
//! the loader/`gnssfw` build flashed on the board (`FARAPISTUB_VERSION`). They
//! match the SDK at the time of writing; if you update the loader and calls start
//! timing out, re-derive [`ids`] from the new `cxd56_farapistub.S`.

use core::sync::atomic::{Ordering, compiler_fence};

use crate::farapi::{self, FarapiError};
use crate::multicore::Mailbox;

/// Routing id of the GNSS DSP core in the CPU-FIFO header nibble
/// (`CXD56_GNSS_GPS_CPUID`). Both the `gnss` Far API module (`cpuno == 1`) and
/// the signal channel target this core.
pub const GNSS_CPUID: u32 = 1;

/// ICC protocol id used **to send** on the GNSS signal channel
/// (`CXD56_PROTO_GNSS`, `cxd56_icc.h`). `cxd56_cpu1sigsend` tags host→DSP
/// `CPUFIFOAPI` commands and backup-handshake replies with this proto.
pub const PROTO_GNSS: u32 = 13;

/// ICC protocol id the DSP uses **to send back** to us (`CXD56_PROTO_MSG`).
///
/// Asymmetry that bit us once: outbound (host→DSP) signal-channel traffic is
/// proto [`PROTO_GNSS`], but the DSP's async notifications (BOOTCOMP, POSITION,
/// REQBKUPDAT) and its `CPUFIFOAPI` acks arrive as generic *MSG* frames routed
/// purely by the source-CPU nibble. NuttX delivers them through
/// `cxd56_iccrecvmsg(g_cpumsg[1])`, which is only fed by `icc_irqhandler`'s
/// `protoid == CXD56_PROTO_MSG` branch — so inbound GNSS messages must be matched
/// on (source cpuid == [`GNSS_CPUID`]) ∧ (proto == `MSG`), **not** proto GNSS.
const PROTO_MSG: u32 = 0;

/// CPU1 signal data-type for the API command/response sub-channel
/// (`CXD56_CPU1_DATA_TYPE_CPUFIFOAPI`).
const DATA_TYPE_CPUFIFOAPI: u32 = 13;
/// CPU1 signal data-type for GNSS position/boot notifications
/// (`CXD56_CPU1_DATA_TYPE_GNSS`); also the `type` arg to `fw_gd_readbuffer`.
const DATA_TYPE_GNSS: u32 = 0;
/// CPU1 signal/write-buffer data-type for shared driver state
/// (`CXD56_CPU1_DATA_TYPE_INFO`).
const DATA_TYPE_INFO: u32 = 6;
/// CPU1 signal data-type that terminates a backup-file transfer
/// (`CXD56_CPU1_DATA_TYPE_BKUPFILE`).
const DATA_TYPE_BKUPFILE: u32 = 10;

/// `fw_pm_sleepcpu` mode that pins the GNSS core out of hot-sleep
/// (`PM_SLEEP_MODE_HOT_DISABLE`). NuttX issues this right after `startcpu`
/// (unless `CONFIG_CXD56_GNSS_HOT_SLEEP`, which is off by default) so the DSP
/// stays awake and serviceable — our Far API path has no hot-sleep wakeup, so
/// leaving hot-sleep enabled would let the core nap and stall later calls.
const PM_SLEEP_MODE_HOT_DISABLE: u32 = 8;

/// `CPUFIFOAPI` command: start positioning (`CXD56_GNSS_GD_GNSS_START`).
const GD_GNSS_START: u32 = 0;
/// `CPUFIFOAPI` command: stop positioning (`CXD56_GNSS_GD_GNSS_STOP`).
const GD_GNSS_STOP: u32 = 1;

/// Firmware image the loader pulls onto the GNSS DSP core (`CXD56_GNSS_FWNAME`).
/// RTK builds use `"gnssfwrtk"`; positioning-only wants plain `"gnssfw"`.
/// NUL-terminated so its pointer can be handed straight to `fw_pm_loadimage`.
static GNSSFW_IMAGE: [u8; 7] = *b"gnssfw\0";

/// PMIC load-switch channel for the GNSS TCXO + LNA rail
/// (`POWER_LNA`/`POWER_TCXO == PMIC_LSW(4)`, `board.h`). Must be on for the RF
/// front-end to receive; [`Gnss::boot`] enables it via [`crate::pmic`].
const LNA_TCXO_LSW_CH: u8 = 1 << 4;

/// Far API module and function indices for the GNSS bring-up + verbs, taken from
/// `cxd56_farapistub.S`. The api-id rule (confirmed on hardware for
/// [`crate::pmic`] / [`crate::audio_aca`]) is `api_id = (slot_within_module + 1)
/// * 4`; the slots below are counted directly from the SDK stub table.
pub mod ids {
    /// Power-manager module — the first `_modulelist_*` entry (`power_mgr`,
    /// `cpuno 0`). Hosts the `fw_pm_*` verbs. (Same module [`crate::pmic`] uses.)
    pub const POWER_MGR_MODID: i32 = 0;

    /// GNSS module — `_modulelist_gnss`, the 9th module entry in the section, so
    /// index 8. Its entry has `cpuno == 1`, so its calls route to
    /// [`super::GNSS_CPUID`] via [`crate::farapi::call_on_cpu`].
    pub const GNSS_MODID: i32 = 8;

    // --- power_mgr module (modid 0), to SYSIOP ------------------------------

    /// `fw_pm_startcpu(cpuid, wait)` — slot 15.
    pub const FW_PM_STARTCPU: i32 = 64;
    /// `fw_pm_sleepcpu(cpuid, mode)` — slot 17.
    pub const FW_PM_SLEEPCPU: i32 = 72;
    /// `fw_pm_loadimage(cpuid, name)` — slot 22.
    pub const FW_PM_LOADIMAGE: i32 = 92;

    // --- gnss module (modid 8), to the GNSS DSP -----------------------------

    /// `fw_gd_start(startmode)` — slot 0. (Used by the GPS-test path; production
    /// positioning starts via the signal channel — see [`super::Gnss::start`].)
    pub const FW_GD_START: i32 = 4;
    /// `fw_gd_stop()` — slot 1.
    pub const FW_GD_STOP: i32 = 8;
    /// `fw_gd_selectsatellitesystem(system)` — slot 2.
    pub const FW_GD_SELECT_SATELLITE_SYSTEM: i32 = 12;
    /// `fw_gd_getsatellitesystem(*system)` — slot 3.
    pub const FW_GD_GET_SATELLITE_SYSTEM: i32 = 16;
    /// `fw_gd_setoperationmode(mode, cycle)` — slot 6.
    pub const FW_GD_SET_OPERATION_MODE: i32 = 28;
    /// `fw_gd_readbuffer(type, offset, buf, len)` — slot 35.
    pub const FW_GD_READBUFFER: i32 = 144;
    /// `fw_gd_writebuffer(type, offset, buf, len)` — slot 36.
    pub const FW_GD_WRITEBUFFER: i32 = 148;
}

/// Async notification code the firmware sends on the GNSS signal channel
/// (`CXD56_GNSS_NOTIFY_TYPE_*`). Carried in the high bits of the data word.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Notify {
    /// A fresh position fix is ready — read it with [`Gnss::read_fix`].
    Position,
    /// `gnssfw` finished booting; safe to issue GNSS verbs.
    BootComplete,
    /// Firmware is asking for backup data (only relevant with AGPS/backup).
    RequestBackup,
    /// Anything else, carried through raw.
    Other(u32),
}

impl Notify {
    fn from_code(code: u32) -> Self {
        match code {
            0 => Notify::Position,      // CXD56_GNSS_NOTIFY_TYPE_POSITION
            1 => Notify::BootComplete,  // CXD56_GNSS_NOTIFY_TYPE_BOOTCOMP
            2 => Notify::RequestBackup, // CXD56_GNSS_NOTIFY_TYPE_REQBKUPDAT
            other => Notify::Other(other),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SharedInfo {
    retval: i32,
    argc: u32,
    argv: [u32; 6],
}

impl SharedInfo {
    const fn empty() -> Self {
        Self {
            retval: 0,
            argc: 0,
            argv: [0; 6],
        }
    }
}

/// Positioning start mode — the `mode` argument to `GD_GNSS_START`
/// (`CXD56_GNSS_STMOD_*`). The colder the start, the longer the first fix:
/// `Cold` assumes no prior almanac/ephemeris/time, `Hot` assumes all are still
/// valid. `Cold` is the always-safe default.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum StartMode {
    Cold = 0,
    Warm = 1,
    WarmAccuracy2 = 2,
    Hot = 3,
    HotAccuracy = 4,
    HotAccuracy2 = 5,
    HotAccuracy3 = 6,
    /// XTAL correction mode 1 (`CXD56_GNSS_STMOD_XTC1`).
    Xtc1 = 7,
    /// XTAL correction mode 2 (`CXD56_GNSS_STMOD_XTC2`).
    Xtc2 = 8,
}

// ============================================================================
// Satellite-system selection
// ============================================================================

/// A set of satellite **systems** (constellations) to track — a bitmask
/// mirroring `gnss_type.h` `CXD56_GNSS_SAT_*`. Combine with `|`.
///
/// Note SBAS, QZSS-L1S and IMES are augmentation/messaging services layered on
/// the ranging constellations (GPS/GLONASS/Galileo/BeiDou/QZSS-L1CA), not
/// independent position sources.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SatelliteSystem(u32);

impl SatelliteSystem {
    /// GPS (USA) — `CXD56_GNSS_SAT_GPS`.
    pub const GPS: Self = Self(1 << 0);
    /// GLONASS (Russia) — `CXD56_GNSS_SAT_GLONASS`.
    pub const GLONASS: Self = Self(1 << 1);
    /// SBAS augmentation — `CXD56_GNSS_SAT_SBAS`.
    pub const SBAS: Self = Self(1 << 2);
    /// QZSS L1C/A (Japan/Asia-Pacific) — `CXD56_GNSS_SAT_QZ_L1CA`.
    pub const QZ_L1CA: Self = Self(1 << 3);
    /// IMES indoor messaging — `CXD56_GNSS_SAT_IMES`.
    pub const IMES: Self = Self(1 << 4);
    /// QZSS L1S augmentation — `CXD56_GNSS_SAT_QZ_L1S`.
    pub const QZ_L1S: Self = Self(1 << 5);
    /// BeiDou (China) — `CXD56_GNSS_SAT_BEIDOU`.
    pub const BEIDOU: Self = Self(1 << 6);
    /// Galileo (EU) — `CXD56_GNSS_SAT_GALILEO`.
    pub const GALILEO: Self = Self(1 << 7);

    /// The raw `CXD56_GNSS_SAT_*` bitmask.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Build from a raw bitmask (e.g. one read back from the firmware).
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Union of two sets.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Whether every system in `other` is present.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// How many distinct systems are selected.
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }
}

impl core::ops::BitOr for SatelliteSystem {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl core::fmt::Debug for SatelliteSystem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SatelliteSystem(0x{:02x}, {} systems)", self.0, self.count())
    }
}

/// The constellation set [`SatelliteSelection::Automatic`] resolves to.
///
/// **GPS + GLONASS + QZSS-L1C/A + SBAS — four satellite systems.** GPS and
/// GLONASS give global ranging coverage; QZSS-L1C/A adds high-elevation Asia-
/// Pacific satellites and SBAS adds wide-area corrections, both of which improve
/// time-to-first-fix and accuracy without the power cost of every constellation.
pub const AUTOMATIC: SatelliteSystem = SatelliteSystem::GPS
    .union(SatelliteSystem::GLONASS)
    .union(SatelliteSystem::QZ_L1CA)
    .union(SatelliteSystem::SBAS);

/// Number of satellite systems enabled by [`SatelliteSelection::Automatic`] — 4.
pub const AUTOMATIC_SYSTEM_COUNT: u32 = AUTOMATIC.count();

/// How the caller chooses which constellations to track.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SatelliteSelection {
    /// Let the HAL pick a sensible default — [`AUTOMATIC`]
    /// ([`AUTOMATIC_SYSTEM_COUNT`] systems).
    Automatic,
    /// Track exactly the systems the caller specifies.
    Manual(SatelliteSystem),
}

impl SatelliteSelection {
    /// The concrete [`SatelliteSystem`] bitmask this selection resolves to.
    pub const fn systems(self) -> SatelliteSystem {
        match self {
            SatelliteSelection::Automatic => AUTOMATIC,
            SatelliteSelection::Manual(s) => s,
        }
    }
}

// ============================================================================
// Position data — exact `#[repr(C)]` mirror of gnss_type.h
// ============================================================================

/// Calendar date from a fix (`cxd56_gnss_date_s`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct GnssDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// UTC time-of-day from a fix (`cxd56_gnss_time_s`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct GnssTime {
    pub hour: u8,
    pub minute: u8,
    pub sec: u8,
    pub usec: u32,
}

/// Dilution-of-precision values (`cxd56_gnss_dop_s`, 9 × f32).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct GnssDop {
    pub pdop: f32,
    pub hdop: f32,
    pub vdop: f32,
    pub tdop: f32,
    pub ewdop: f32,
    pub nsdop: f32,
    pub majdop: f32,
    pub mindop: f32,
    pub oridop: f32,
}

/// Position variance (`cxd56_gnss_var_s`, 2 × f32).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct GnssVar {
    pub hvar: f32,
    pub vvar: f32,
}

/// Receiver sub-record of a fix — a `#[repr(C)]` mirror of
/// `cxd56_gnss_receiver_s` (`gnss_type.h`), truncated after `time` (everything
/// before it is laid out byte-for-byte as the C ABI does, so the field offsets —
/// and thus latitude/longitude — match what `fw_gd_readbuffer` writes).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Receiver {
    /// Position type: 0 invalid, 1 GNSS, 2 IMES, 3 user, 4 previous.
    pub kind: u8,
    /// FALSE: SGPS, TRUE: DGPS.
    pub dgps: u8,
    /// Position fix mode: 1 = invalid, 2 = 2D, 3 = 3D.
    pub pos_fixmode: u8,
    /// Velocity fix mode: 1 = invalid, 2 = 2DVZ, 3 = 2D-offset, 4 = 3D, …
    pub vel_fixmode: u8,
    /// Number of visible satellites.
    pub numsv: u8,
    /// Number of tracking satellites.
    pub numsv_tracking: u8,
    /// Number of satellites used to calculate position.
    pub numsv_calcpos: u8,
    /// Number of satellites used to calculate velocity.
    pub numsv_calcvel: u8,
    /// Assist bit field (user/CEP/AEP).
    pub assist: u8,
    /// 0 none, 1 exists.
    pub pos_dataexist: u8,
    /// Using sv-system bit field (see [`SatelliteSystem`]).
    pub svtype: u16,
    /// Position sv-system bit field.
    pub pos_svtype: u16,
    /// Velocity sv-system bit field.
    pub vel_svtype: u16,
    /// Position source: 0 invalid, 1 GNSS, 2 IMES, 3 user, 4 previous.
    pub possource: u32,
    /// TCXO offset [Hz].
    pub tcxo_offset: f32,
    /// DOPs of position.
    pub pos_dop: GnssDop,
    /// Weighted DOPs of velocity.
    pub vel_idx: GnssDop,
    /// Accuracy of position.
    pub pos_accuracy: GnssVar,
    /// Latitude [degree].
    pub latitude: f64,
    /// Longitude [degree].
    pub longitude: f64,
    /// Altitude [m].
    pub altitude: f64,
    /// Geoid height [m].
    pub geoid: f64,
    /// Velocity [m/s].
    pub velocity: f32,
    /// Direction [degree].
    pub direction: f32,
    /// Current day (UTC).
    pub date: GnssDate,
    /// Current time (UTC).
    pub time: GnssTime,
}

/// Top-level fix buffer head — a `#[repr(C)]` mirror of
/// `cxd56_gnss_positiondata_s` up to and including [`Receiver`]. The trailing
/// per-satellite array is omitted; `fw_gd_readbuffer` does a short read into this
/// smaller buffer.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PositionData {
    /// Firmware timestamp of the fix.
    pub data_timestamp: u64,
    /// Positioning status: 0 = valid, <0 = invalid.
    pub status: u32,
    /// Number of satellites in the trailing (omitted) per-SV array.
    pub svcount: u32,
    /// The receiver/fix record.
    pub receiver: Receiver,
}

// Lock the layout to the C ABI (`gnss_type.h`). `#[repr(C)]` reproduces the
// firmware struct's offsets only if the fields/types/order match; these
// assertions fail the build if a field is ever added/reordered wrongly.
const _: () = {
    assert!(core::mem::offset_of!(PositionData, status) == 8);
    assert!(core::mem::offset_of!(PositionData, receiver) == 16);
    assert!(core::mem::offset_of!(Receiver, latitude) == 104);
    assert!(core::mem::offset_of!(Receiver, longitude) == 112);
    assert!(core::mem::offset_of!(Receiver, altitude) == 120);
    assert!(core::mem::offset_of!(Receiver, date) == 144);
    assert!(core::mem::offset_of!(Receiver, time) == 148);
};

/// A parsed position fix — the friendly surface over [`PositionData`].
#[derive(Copy, Clone, Debug)]
pub struct Fix {
    /// Latitude in degrees (north positive).
    pub latitude: f64,
    /// Longitude in degrees (east positive).
    pub longitude: f64,
    /// Altitude above the ellipsoid in metres.
    pub altitude: f64,
    /// Space vehicles used in the position solution (`numsv_calcpos`). Zero
    /// until a fix actually exists, so it is *not* an acquisition-progress signal.
    pub num_satellites: u8,
    /// Space vehicles currently being tracked (`numsv_tracking`). Climbs above
    /// zero while acquiring, before any fix — the real cold-start progress signal.
    pub num_tracking: u8,
    /// Space vehicles currently visible (`numsv`).
    pub num_visible: u8,
    /// Raw fix mode: 1 = invalid, 2 = 2D, 3 = 3D.
    pub fix_mode: u8,
    /// UTC date of the fix.
    pub date: GnssDate,
    /// UTC time of the fix.
    pub time: GnssTime,
    /// Horizontal dilution of precision.
    pub hdop: f32,
}

impl Fix {
    fn from_data(pd: &PositionData) -> Self {
        let r = &pd.receiver;
        Self {
            latitude: r.latitude,
            longitude: r.longitude,
            altitude: r.altitude,
            num_satellites: r.numsv_calcpos,
            num_tracking: r.numsv_tracking,
            num_visible: r.numsv,
            fix_mode: r.pos_fixmode,
            date: r.date,
            time: r.time,
            hdop: r.pos_dop.hdop,
        }
    }

    /// Whether this fix carries a usable position (2D or 3D). `pos_fixmode` is
    /// `CXD56_GNSS_PVT_POSFIX_2D` (2) or `_3D` (3) when valid.
    pub fn is_valid(&self) -> bool {
        self.fix_mode == 2 || self.fix_mode == 3
    }

    /// Dimensionality of the fix: `Some(2)`/`Some(3)`, or `None` if invalid.
    pub fn dimensions(&self) -> Option<u8> {
        match self.fix_mode {
            2 => Some(2),
            3 => Some(3),
            _ => None,
        }
    }
}

// ============================================================================
// Driver
// ============================================================================

/// Error from a GNSS operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GnssError {
    /// The Far API transport timed out — the SYSIOP/GNSS firmware did not reply.
    Farapi,
    /// A firmware verb returned a non-zero status.
    Status(i32),
    /// Powering the TCXO/LNA rail via the PMIC failed.
    Pmic,
    /// `gnssfw` did not report `BOOTCOMP` within the budget.
    BootTimeout,
    /// A signal-channel command got no response within the budget.
    SignalTimeout,
}

impl From<FarapiError> for GnssError {
    fn from(_: FarapiError) -> Self {
        GnssError::Farapi
    }
}

/// Poll budget for a GNSS Far API round-trip. `read`/`select` are sub-ms;
/// `loadimage` copies a firmware blob and is much slower, so this is generous.
const GNSS_POLL_BUDGET: u32 = 20_000_000;

/// Budget to wait for the `BOOTCOMP` notification after `startcpu`.
const BOOT_NOTIFY_BUDGET: u32 = 50_000_000;

/// Budget to wait for a `CPUFIFOAPI` command (start/stop) response.
const SIGNAL_BUDGET: u32 = 20_000_000;

/// GNSS receiver handle.
///
/// Zero-sized: like [`crate::pmic`] / [`crate::audio_aca`] it reaches the
/// firmware purely through the shared CPU-FIFO, so there is no peripheral to own.
/// Construct with [`Gnss::new`], then [`boot`](Self::boot) →
/// [`select_satellites`](Self::select_satellites) → [`start`](Self::start) →
/// poll [`read_fix`](Self::read_fix).
pub struct Gnss {
    boot_complete_seen: bool,
}

impl Default for Gnss {
    fn default() -> Self {
        Self::new()
    }
}

impl Gnss {
    /// Create a GNSS handle. Does not touch hardware; call [`boot`](Self::boot).
    pub const fn new() -> Self {
        Self {
            boot_complete_seen: false,
        }
    }

    /// Issue one `fw_pm_*` (power-manager module, SYSIOP) call; return its status.
    fn pm_call(&mut self, api_id: i32, arg: &mut [u32]) -> Result<i32, GnssError> {
        let boot_complete_seen = &mut self.boot_complete_seen;
        farapi::call_on_cpu_with_unrelated(
            0,
            ids::POWER_MGR_MODID,
            api_id,
            arg,
            GNSS_POLL_BUDGET,
            |w0, w1| {
                if let Some((DATA_TYPE_GNSS, code)) = Self::sig_decode(w0, w1) {
                    match Notify::from_code(code) {
                        Notify::BootComplete => *boot_complete_seen = true,
                        Notify::RequestBackup => Self::finish_empty_backup_restore(),
                        _ => {}
                    }
                }
            },
        )?;
        Ok(arg[0] as i32)
    }

    /// Issue one `fw_gd_*` (GNSS module) call, routed to the GNSS DSP (CPU 1).
    /// Maps a negative return to [`GnssError::Status`]; on success the raw
    /// return (e.g. `fw_gd_readbuffer`'s byte count) is in `arg[0]`.
    fn gd_call(&mut self, api_id: i32, arg: &mut [u32]) -> Result<i32, GnssError> {
        farapi::call_on_cpu(GNSS_CPUID, ids::GNSS_MODID, api_id, arg, GNSS_POLL_BUDGET)?;
        let ret = arg[0] as i32;
        if ret < 0 {
            Err(GnssError::Status(ret))
        } else {
            Ok(ret)
        }
    }

    /// Power the TCXO/LNA rail, load `gnssfw` onto the GNSS DSP core, start it,
    /// and wait for the firmware to report `BOOTCOMP`.
    ///
    /// Mirrors `cxd56_gnss_open`: enable `PMIC_LSW(4)` (so the RF front-end can
    /// receive), `fw_pm_loadimage(1,"gnssfw")` → `fw_pm_startcpu(1,1)`, then spin
    /// for `BOOTCOMP`. After `Ok`, [`select_satellites`](Self::select_satellites)
    /// and [`start`](Self::start) are safe.
    ///
    /// If this returns [`GnssError::BootTimeout`] but the FAR calls themselves
    /// succeeded, the firmware likely *did* boot and the boot signal was just
    /// missed — fall back to [`load_and_start`](Self::load_and_start) plus a fixed
    /// settle delay.
    pub fn boot(&mut self) -> Result<(), GnssError> {
        self.load_and_start()?;
        self.wait_boot_complete(BOOT_NOTIFY_BUDGET)?;
        self.publish_shared_info()
    }

    /// Power the rail, load `gnssfw`, and release the DSP core — **without**
    /// waiting for `BOOTCOMP`. Most callers want [`boot`](Self::boot).
    pub fn load_and_start(&mut self) -> Result<(), GnssError> {
        // Power the GNSS TCXO + LNA rail (shared PMIC load switch 4).
        crate::pmic::set_loadswitch(LNA_TCXO_LSW_CH, true).map_err(|_| GnssError::Pmic)?;

        // fw_pm_loadimage(cpuid, image_name)
        let mut arg = [GNSS_CPUID, GNSSFW_IMAGE.as_ptr() as u32, 0, 0];
        compiler_fence(Ordering::SeqCst);
        let status = self.pm_call(ids::FW_PM_LOADIMAGE, &mut arg)?;
        if status < 0 {
            return Err(GnssError::Status(status));
        }

        // fw_pm_startcpu(cpuid, wait=1)
        let mut arg = [GNSS_CPUID, 1, 0, 0];
        let status = self.pm_call(ids::FW_PM_STARTCPU, &mut arg)?;
        if status < 0 {
            return Err(GnssError::Status(status));
        }

        // fw_pm_sleepcpu(cpuid, PM_SLEEP_MODE_HOT_DISABLE) — keep the DSP awake
        // so it stays serviceable (NuttX `cxd56_gnss_open`, default config). A
        // non-zero status here is non-fatal: it only affects power, not boot.
        let mut arg = [GNSS_CPUID, PM_SLEEP_MODE_HOT_DISABLE, 0, 0];
        let _ = self.pm_call(ids::FW_PM_SLEEPCPU, &mut arg)?;
        Ok(())
    }

    /// Choose the constellation set, then push it to the firmware with
    /// `fw_gd_selectsatellitesystem`. Returns the [`SatelliteSystem`] applied
    /// (handy to log what `Automatic` picked). Call after [`boot`](Self::boot)
    /// and before [`start`](Self::start).
    pub fn select_satellites(
        &mut self,
        selection: SatelliteSelection,
    ) -> Result<SatelliteSystem, GnssError> {
        let systems = selection.systems();
        let mut arg = [systems.bits(), 0, 0, 0];
        self.gd_call(ids::FW_GD_SELECT_SATELLITE_SYSTEM, &mut arg)?;
        Ok(systems)
    }

    /// Read back the currently-selected constellation set
    /// (`fw_gd_getsatellitesystem`).
    pub fn selected_satellites(&mut self) -> Result<SatelliteSystem, GnssError> {
        let mut system = 0u32;
        let mut arg = [(&mut system as *mut u32) as u32, 0, 0, 0];
        compiler_fence(Ordering::SeqCst);
        self.gd_call(ids::FW_GD_GET_SATELLITE_SYSTEM, &mut arg)?;
        compiler_fence(Ordering::SeqCst);
        Ok(SatelliteSystem::from_bits(system))
    }

    /// Set the operation mode and positioning cycle (`fw_gd_setoperationmode`).
    /// `cycle_ms` is the fix interval in milliseconds (default 1000). Optional —
    /// the firmware defaults to 1 Hz normal mode.
    pub fn set_operation_mode(&mut self, mode: u32, cycle_ms: u32) -> Result<(), GnssError> {
        let mut arg = [mode, cycle_ms, 0, 0];
        self.gd_call(ids::FW_GD_SET_OPERATION_MODE, &mut arg)?;
        Ok(())
    }

    /// Begin positioning. Sends `GD_GNSS_START(mode)` over the signal channel
    /// (the path NuttX's `cxd56_gnss_start` uses) and waits for the firmware's
    /// command ack.
    pub fn start(&mut self, mode: StartMode) -> Result<(), GnssError> {
        self.cpufifo_api(GD_GNSS_START, mode as u32)
    }

    /// Stop positioning. Sends `GD_GNSS_STOP` over the signal channel.
    pub fn stop(&mut self) -> Result<(), GnssError> {
        self.cpufifo_api(GD_GNSS_STOP, 0)
    }

    /// Pull the current fix out of firmware with
    /// `fw_gd_readbuffer(GNSS, 0, &buf, len)` and parse it.
    ///
    /// The firmware refreshes this buffer each positioning cycle; the returned
    /// [`Fix`] is only a real position once [`Fix::is_valid`] is true. Poll this
    /// after [`start`](Self::start) — optionally gated on
    /// [`poll_fix`](Self::poll_fix) so you only read when a new fix is signalled.
    pub fn read_fix(&mut self) -> Result<Fix, GnssError> {
        let mut pd = PositionData::default();
        let mut arg = [
            DATA_TYPE_GNSS,
            0, // offset
            (&mut pd as *mut PositionData) as u32,
            core::mem::size_of::<PositionData>() as u32,
        ];
        // The firmware writes `pd` across the core boundary; fence both sides.
        compiler_fence(Ordering::SeqCst);
        self.gd_call(ids::FW_GD_READBUFFER, &mut arg)?;
        compiler_fence(Ordering::SeqCst);
        Ok(Fix::from_data(&pd))
    }

    /// Publish the small driver-info block expected by `gnssfw` after boot.
    /// NuttX sends this immediately after the boot/backup-data handshake.
    fn publish_shared_info(&mut self) -> Result<(), GnssError> {
        let mut info = SharedInfo::empty();
        let mut arg = [
            DATA_TYPE_INFO,
            0, // offset
            (&mut info as *mut SharedInfo) as u32,
            core::mem::size_of::<SharedInfo>() as u32,
        ];
        compiler_fence(Ordering::SeqCst);
        self.gd_call(ids::FW_GD_WRITEBUFFER, &mut arg)?;
        compiler_fence(Ordering::SeqCst);
        Ok(())
    }

    /// Tell `gnssfw` there is no persisted backup file to restore.
    ///
    /// NuttX reads `/mnt/spif/gnss_backup.bin` when it receives
    /// `REQBKUPDAT`, then always sends this completion signal. Bare-metal does
    /// not have that filesystem service yet, so we complete the sequence with
    /// no data.
    fn finish_empty_backup_restore() {
        Self::sig_send(DATA_TYPE_BKUPFILE, 0);
    }

    /// Non-blocking: if a `POSITION` notification is waiting, consume it and read
    /// the fix; otherwise return `Ok(None)`.
    pub fn poll_fix(&mut self) -> Result<Option<Fix>, GnssError> {
        match self.poll_notify() {
            Some(Notify::Position) => self.read_fix().map(Some),
            _ => Ok(None),
        }
    }

    // -- Signal channel (CXD56_PROTO_GNSS) -----------------------------------

    /// Send one `cxd56_cpu1sigsend(datatype, data)` on the GNSS signal channel.
    /// `datatype` goes in `word0.msgid`; `data` is `word1`.
    fn sig_send(datatype: u32, data: u32) {
        let w0 = (GNSS_CPUID << 28) | (PROTO_GNSS << 24) | ((datatype & 0xff) << 16);
        compiler_fence(Ordering::SeqCst);
        Mailbox::send_blocking([w0, data]);
    }

    /// Decode a received CPU-FIFO message as a GNSS signal-channel message.
    /// Returns `(datatype, code)` where `datatype = word1 & 0xff` (the data type
    /// the firmware addressed, `CXD56_CPU1_GET_DEV`) and `code = word1 >> 8`
    /// (`CXD56_CPU1_GET_DATA`). `None` for anything not from the GNSS DSP.
    ///
    /// Inbound notifications carry the DSP's id (`MSGFROM` = [`GNSS_CPUID`]) in
    /// word0's source nibble and arrive as generic [`PROTO_MSG`] frames — see
    /// [`PROTO_MSG`]. Matching proto `MSG` (not `GNSS`) is also what keeps a
    /// stray Far API `FLG` completion from the same core (proto 3) from being
    /// misread as a notification.
    fn sig_decode(w0: u32, w1: u32) -> Option<(u32, u32)> {
        let source = (w0 >> 28) & 0xf;
        let proto = (w0 >> 24) & 0xf;
        if source == GNSS_CPUID && proto == PROTO_MSG {
            Some((w1 & 0xff, w1 >> 8))
        } else {
            None
        }
    }

    /// Issue a `CPUFIFOAPI` command (`api`, `data`) and wait for its response.
    /// Mirrors NuttX `cxd56_gnss_cpufifo_api`: send
    /// `CPUFIFOAPI((data << 8) | api)`, then wait for the firmware's
    /// `CPUFIFOAPI` reply and return its status (negative ⇒
    /// [`GnssError::Status`]).
    fn cpufifo_api(&mut self, api: u32, data: u32) -> Result<(), GnssError> {
        Self::sig_send(DATA_TYPE_CPUFIFOAPI, (data << 8) | api);

        for _ in 0..SIGNAL_BUDGET {
            let Some([w0, w1]) = Mailbox::try_recv() else {
                core::hint::spin_loop();
                continue;
            };
            // Only the CPUFIFOAPI response acks the command; GNSS position/boot
            // notifications that arrive meanwhile are dropped (we poll fixes
            // directly via read_fix, so this is harmless).
            match Self::sig_decode(w0, w1) {
                Some((DATA_TYPE_CPUFIFOAPI, ret)) => {
                    let ret = ret as i32;
                    return if ret < 0 {
                        Err(GnssError::Status(ret))
                    } else {
                        Ok(())
                    };
                }
                _ => continue,
            }
        }
        Err(GnssError::SignalTimeout)
    }

    /// Non-blocking poll of the CPU-FIFO for a GNSS notification from the DSP
    /// core ([`Notify`]). Returns `None` if no GNSS-type message is waiting.
    ///
    /// Non-GNSS FIFO traffic is consumed and dropped — do not run this
    /// concurrently with another CPU-FIFO user (same constraint as
    /// [`crate::farapi`] / [`crate::clocks::pm`]).
    pub fn poll_notify(&mut self) -> Option<Notify> {
        let [w0, w1] = Mailbox::try_recv()?;
        match Self::sig_decode(w0, w1) {
            Some((DATA_TYPE_GNSS, code)) => Some(Notify::from_code(code)),
            _ => None,
        }
    }

    /// Block until a `BOOTCOMP` notification arrives or `budget` poll iterations
    /// elapse. Used by [`boot`](Self::boot); exposed so callers that drove
    /// [`load_and_start`](Self::load_and_start) directly can still wait for it.
    pub fn wait_boot_complete(&mut self, budget: u32) -> Result<(), GnssError> {
        if self.boot_complete_seen {
            return Ok(());
        }

        for _ in 0..budget {
            match self.poll_notify() {
                Some(Notify::BootComplete) => {
                    self.boot_complete_seen = true;
                    return Ok(());
                }
                Some(Notify::RequestBackup) => Self::finish_empty_backup_restore(),
                _ => core::hint::spin_loop(),
            }
        }
        Err(GnssError::BootTimeout)
    }
}
