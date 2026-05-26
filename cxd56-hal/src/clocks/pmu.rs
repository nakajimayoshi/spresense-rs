//! PMU power-domain sequencer.
//!
//! Each per-peripheral clock-enable on the CXD5602 is preceded by a PMU
//! power-domain enable, then optionally a reset release, all gated by the
//! 20000-iteration "kick" loop described in
//! `nuttx/arch/arm/src/cxd56xx/cxd56_clock.c:254-387`.
//!
//! Refcounts mirror the SDK's `g_digital.refs[]` / `g_analog.refs[]`. We use
//! `critical_section` for the read-modify-write of the refcount + register
//! write — same scope as `spin_lock_irqsave` in the C.

use crate::pac;
use core::sync::atomic::{AtomicU32, Ordering};

use super::peripheral::ClockError;

/// Digital power domains. Bit positions in `PWD_CTL` / `PWD_STAT`.
/// Mirrors the PDID_* enum in `cxd56_clock.c`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmuDomain {
    Scu = 0,
    Core = 4,
    Sysiop = 5,
    SysiopSub = 6,
    App = 8,
    AppDsp = 9,
    AppSub = 10,
    GnssItp = 11,
    Gnss = 12,
    AppAudio = 13,
}

/// Analog power domains. Bit positions in `ANA_PW_CTL` / `ANA_PW_STAT`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AnaDomain {
    Rcosc = 0,
    Xosc = 1,
    Syspll = 2,
    Hpadc = 9,
    Lpadc = 10,
}

/// Domains whose reset must be explicitly released after power-on.
/// Mask = 0x3141 in the SDK (`cxd56_clock.c:304`).
const RESET_RELEASE_MASK: u32 = 0x3141;

const POWER_CONTROL_RETRY: u32 = 20_000;
const KICK_DELAY_US: u32 = 100;
const SETTLE_DELAY_US: u32 = 400;

/// Refcounts indexed by `PmuDomain as u8`. Length matches the highest enum
/// discriminant (`AppAudio` = 13) plus a small margin.
static DIGITAL_REFS: [AtomicU32; 16] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];
static ANALOG_REFS: [AtomicU32; 16] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// Cached APPSMP frequency for [`delay_us`] scaling. Defaults to the boot
/// ROM HP-mode value so calls before [`super::Crg::new`] still work.
static APPSMP_HZ: AtomicU32 = AtomicU32::new(156_000_000);

pub(crate) fn set_appsmp_hz(hz: u32) {
    if hz != 0 {
        APPSMP_HZ.store(hz, Ordering::Relaxed);
    }
}

/// Busy-wait without depending on the public [`Delay`](crate::delay::Delay).
fn delay_us(us: u32) {
    let hz = APPSMP_HZ.load(Ordering::Relaxed);
    let cycles = ((hz as u64) * us as u64 / 1_000_000) as u32;
    cortex_m::asm::delay(cycles.max(1));
}

/// Read `CHIP_ID` once per iteration to flush the bus path — used by the
/// SYSIOP_SUB enable-reset-enable dance (`cxd56_clock.c:246-252`).
/// `CXD56_TOPREG_CHIP_ID = CXD56_TOPREG_SUB_BASE + 0x1490`.
pub(crate) fn busy_wait(cnt: u32) {
    let sub = unsafe { &*pac::TopregSub::PTR };
    for _ in 0..cnt {
        let _ = sub.chip_id().read().bits();
    }
}

/// Drive PMU_PW_CTL=1 in a loop until `(reg & mask) == want`. Returns
/// [`ClockError::PmuTimeout`] after 20000 unproductive iterations.
/// Mirrors `do_power_control` (`cxd56_clock.c:254`).
fn kick_and_poll(read: impl Fn() -> u32, mask: u32, want: u32) -> Result<(), ClockError> {
    let topreg = unsafe { &*pac::Topreg::PTR };
    for _ in 0..POWER_CONTROL_RETRY {
        topreg.pmu_pw_ctl().write(|w| w.power_ctrl_on().set_bit());
        delay_us(KICK_DELAY_US);
        if read() & mask == want {
            delay_us(SETTLE_DELAY_US);
            return Ok(());
        }
    }
    Err(ClockError::PmuTimeout)
}

#[inline]
fn write_pwd_ctl(mask: u32) {
    let topreg = unsafe { &*pac::Topreg::PTR };
    topreg
        .pwd_ctl()
        .write(|w| unsafe { w.bits(mask | (mask << 16)) });
}

#[inline]
fn write_pwd_ctl_off(mask: u32) {
    let topreg = unsafe { &*pac::Topreg::PTR };
    topreg.pwd_ctl().write(|w| unsafe { w.bits(mask << 16) });
}

#[inline]
fn read_pwd_stat() -> u32 {
    let topreg = unsafe { &*pac::Topreg::PTR };
    topreg.pwd_stat().read().bits()
}

#[inline]
fn write_ana_pw_ctl(mask: u32) {
    let topreg = unsafe { &*pac::Topreg::PTR };
    topreg
        .ana_pw_ctl()
        .write(|w| unsafe { w.bits(mask | (mask << 16)) });
}

#[inline]
fn write_ana_pw_ctl_off(mask: u32) {
    let topreg = unsafe { &*pac::Topreg::PTR };
    topreg.ana_pw_ctl().write(|w| unsafe { w.bits(mask << 16) });
}

#[inline]
fn read_ana_pw_stat() -> u32 {
    let topreg = unsafe { &*pac::Topreg::PTR };
    topreg.ana_pw_stat().read().bits()
}

fn release_pwd_reset(mask: u32) {
    if mask & RESET_RELEASE_MASK == 0 {
        return;
    }
    let topreg = unsafe { &*pac::Topreg::PTR };
    topreg
        .pwd_reset0()
        .write(|w| unsafe { w.bits(mask | (mask << 16)) });
}

/// Enable a digital power domain. Idempotent — second caller only bumps the
/// refcount. Mirrors `enable_pwd` (`cxd56_clock.c:312`).
pub fn enable_domain(d: PmuDomain) -> Result<(), ClockError> {
    let mask = 1u32 << (d as u8);
    critical_section::with(|_| {
        let prev = DIGITAL_REFS[d as usize].fetch_add(1, Ordering::Relaxed);
        if prev > 0 {
            return Ok(());
        }
        if read_pwd_stat() & mask != mask {
            write_pwd_ctl(mask);
            kick_and_poll(read_pwd_stat, mask, mask)?;
            release_pwd_reset(mask);
        }
        Ok(())
    })
}

/// Decrement refcount; power-off when it hits zero.
pub fn disable_domain(d: PmuDomain) -> Result<(), ClockError> {
    let mask = 1u32 << (d as u8);
    critical_section::with(|_| {
        let prev =
            DIGITAL_REFS[d as usize].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                if v == 0 { None } else { Some(v - 1) }
            });
        let prev = match prev {
            Ok(v) => v,
            Err(_) => return Ok(()),
        };
        if prev != 1 {
            return Ok(());
        }
        if read_pwd_stat() & mask != 0 {
            write_pwd_ctl_off(mask);
            kick_and_poll(read_pwd_stat, mask, 0)?;
        }
        Ok(())
    })
}

/// Enable an analog power domain. Mirrors `enable_apwd` (`cxd56_clock.c:351`).
pub fn enable_analog(d: AnaDomain) -> Result<(), ClockError> {
    let mask = 1u32 << (d as u8);
    critical_section::with(|_| {
        let prev = ANALOG_REFS[d as usize].fetch_add(1, Ordering::Relaxed);
        if prev > 0 {
            return Ok(());
        }
        if read_ana_pw_stat() & mask != mask {
            write_ana_pw_ctl(mask);
            kick_and_poll(read_ana_pw_stat, mask, mask)?;
        }
        Ok(())
    })
}

pub fn disable_analog(d: AnaDomain) -> Result<(), ClockError> {
    let mask = 1u32 << (d as u8);
    critical_section::with(|_| {
        let prev =
            ANALOG_REFS[d as usize].fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                if v == 0 { None } else { Some(v - 1) }
            });
        let prev = match prev {
            Ok(v) => v,
            Err(_) => return Ok(()),
        };
        if prev != 1 {
            return Ok(());
        }
        if read_ana_pw_stat() & mask != 0 {
            write_ana_pw_ctl_off(mask);
            kick_and_poll(read_ana_pw_stat, mask, 0)?;
        }
        Ok(())
    })
}

/// User-facing escape hatch returned by [`super::Crg::pmu`].
pub struct PmuCtl<'a> {
    _crg: &'a mut super::Crg,
}

impl<'a> PmuCtl<'a> {
    pub(crate) fn new(crg: &'a mut super::Crg) -> Self {
        Self { _crg: crg }
    }
    pub fn enable(&mut self, d: PmuDomain) -> Result<(), ClockError> {
        enable_domain(d)
    }
    pub fn disable(&mut self, d: PmuDomain) -> Result<(), ClockError> {
        disable_domain(d)
    }
    pub fn enable_analog(&mut self, d: AnaDomain) -> Result<(), ClockError> {
        enable_analog(d)
    }
    pub fn disable_analog(&mut self, d: AnaDomain) -> Result<(), ClockError> {
        disable_analog(d)
    }
}
