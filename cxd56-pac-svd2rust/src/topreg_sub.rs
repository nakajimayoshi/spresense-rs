#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0404],
    swreset_gnsdsp: SwresetGnsdsp,
    swreset_app: SwresetApp,
    _reserved2: [u8; 0x04],
    syscpu_cken: SyscpuCken,
    app_cken: AppCken,
    app_cksel: AppCksel,
    app_div: AppDiv,
    sysiop_sub_cken: SysiopSubCken,
    _reserved7: [u8; 0x0c],
    pmu_wake_trig0_clr: PmuWakeTrig0Clr,
    pmu_wake_trig1_clr: PmuWakeTrig1Clr,
    pmu_wake_trig0_raw: PmuWakeTrig0Raw,
    pmu_wake_trig1_raw: PmuWakeTrig1Raw,
    pmu_wake_trig0: PmuWakeTrig0,
    pmu_wake_trig1: PmuWakeTrig1,
    _reserved13: [u8; 0x07d8],
    gnsdsp_cken: GnsdspCken,
    _reserved14: [u8; 0x04],
    gnss_div: GnssDiv,
    _reserved15: [u8; 0x0864],
    chip_id: ChipId,
}
impl RegisterBlock {
    ///0x404 - GNSS DSP software reset
    #[inline(always)]
    pub const fn swreset_gnsdsp(&self) -> &SwresetGnsdsp {
        &self.swreset_gnsdsp
    }
    ///0x408 - Application domain software reset
    #[inline(always)]
    pub const fn swreset_app(&self) -> &SwresetApp {
        &self.swreset_app
    }
    ///0x410 - System CPU clock enable
    #[inline(always)]
    pub const fn syscpu_cken(&self) -> &SyscpuCken {
        &self.syscpu_cken
    }
    ///0x414 - Application domain clock enables
    #[inline(always)]
    pub const fn app_cken(&self) -> &AppCken {
        &self.app_cken
    }
    ///0x418 - Application domain clock source select
    #[inline(always)]
    pub const fn app_cksel(&self) -> &AppCksel {
        &self.app_cksel
    }
    ///0x41c - Application domain clock divider
    #[inline(always)]
    pub const fn app_div(&self) -> &AppDiv {
        &self.app_div
    }
    ///0x420 - SYSIOP sub-domain peripheral clock enables
    #[inline(always)]
    pub const fn sysiop_sub_cken(&self) -> &SysiopSubCken {
        &self.sysiop_sub_cken
    }
    ///0x430 - GPIO interrupt clear register, slots 0–11 (write 1<<(16+slot) to clear)
    #[inline(always)]
    pub const fn pmu_wake_trig0_clr(&self) -> &PmuWakeTrig0Clr {
        &self.pmu_wake_trig0_clr
    }
    ///0x434 - GPIO interrupt clear register, second bank
    #[inline(always)]
    pub const fn pmu_wake_trig1_clr(&self) -> &PmuWakeTrig1Clr {
        &self.pmu_wake_trig1_clr
    }
    ///0x438 - GPIO interrupt raw (pre-mask) status, slots 0–11
    #[inline(always)]
    pub const fn pmu_wake_trig0_raw(&self) -> &PmuWakeTrig0Raw {
        &self.pmu_wake_trig0_raw
    }
    ///0x43c - GPIO interrupt raw status, second bank
    #[inline(always)]
    pub const fn pmu_wake_trig1_raw(&self) -> &PmuWakeTrig1Raw {
        &self.pmu_wake_trig1_raw
    }
    ///0x440 - GPIO interrupt masked status, slots 0–11 (ISR reads this)
    #[inline(always)]
    pub const fn pmu_wake_trig0(&self) -> &PmuWakeTrig0 {
        &self.pmu_wake_trig0
    }
    ///0x444 - GPIO interrupt masked status, second bank
    #[inline(always)]
    pub const fn pmu_wake_trig1(&self) -> &PmuWakeTrig1 {
        &self.pmu_wake_trig1
    }
    ///0xc20 - GNSS DSP clock enables
    #[inline(always)]
    pub const fn gnsdsp_cken(&self) -> &GnsdspCken {
        &self.gnsdsp_cken
    }
    ///0xc28 - GNSS clock divider
    #[inline(always)]
    pub const fn gnss_div(&self) -> &GnssDiv {
        &self.gnss_div
    }
    ///0x1490 - Chip identification register (read-only)
    #[inline(always)]
    pub const fn chip_id(&self) -> &ChipId {
        &self.chip_id
    }
}
/**SWRESET_GNSDSP (rw) register accessor: GNSS DSP software reset

You can [`read`](crate::Reg::read) this register and get [`swreset_gnsdsp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset_gnsdsp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swreset_gnsdsp`] module*/
#[doc(alias = "SWRESET_GNSDSP")]
pub type SwresetGnsdsp = crate::Reg<swreset_gnsdsp::SwresetGnsdspSpec>;
///GNSS DSP software reset
pub mod swreset_gnsdsp;
/**SWRESET_APP (rw) register accessor: Application domain software reset

You can [`read`](crate::Reg::read) this register and get [`swreset_app::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset_app::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swreset_app`] module*/
#[doc(alias = "SWRESET_APP")]
pub type SwresetApp = crate::Reg<swreset_app::SwresetAppSpec>;
///Application domain software reset
pub mod swreset_app;
/**SYSCPU_CKEN (rw) register accessor: System CPU clock enable

You can [`read`](crate::Reg::read) this register and get [`syscpu_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscpu_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscpu_cken`] module*/
#[doc(alias = "SYSCPU_CKEN")]
pub type SyscpuCken = crate::Reg<syscpu_cken::SyscpuCkenSpec>;
///System CPU clock enable
pub mod syscpu_cken;
/**APP_CKEN (rw) register accessor: Application domain clock enables

You can [`read`](crate::Reg::read) this register and get [`app_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@app_cken`] module*/
#[doc(alias = "APP_CKEN")]
pub type AppCken = crate::Reg<app_cken::AppCkenSpec>;
///Application domain clock enables
pub mod app_cken;
/**APP_CKSEL (rw) register accessor: Application domain clock source select

You can [`read`](crate::Reg::read) this register and get [`app_cksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@app_cksel`] module*/
#[doc(alias = "APP_CKSEL")]
pub type AppCksel = crate::Reg<app_cksel::AppCkselSpec>;
///Application domain clock source select
pub mod app_cksel;
/**APP_DIV (rw) register accessor: Application domain clock divider

You can [`read`](crate::Reg::read) this register and get [`app_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@app_div`] module*/
#[doc(alias = "APP_DIV")]
pub type AppDiv = crate::Reg<app_div::AppDivSpec>;
///Application domain clock divider
pub mod app_div;
/**SYSIOP_SUB_CKEN (rw) register accessor: SYSIOP sub-domain peripheral clock enables

You can [`read`](crate::Reg::read) this register and get [`sysiop_sub_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysiop_sub_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sysiop_sub_cken`] module*/
#[doc(alias = "SYSIOP_SUB_CKEN")]
pub type SysiopSubCken = crate::Reg<sysiop_sub_cken::SysiopSubCkenSpec>;
///SYSIOP sub-domain peripheral clock enables
pub mod sysiop_sub_cken;
/**PMU_WAKE_TRIG0_CLR (rw) register accessor: GPIO interrupt clear register, slots 0–11 (write 1<<(16+slot) to clear)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig0_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig0_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig0_clr`] module*/
#[doc(alias = "PMU_WAKE_TRIG0_CLR")]
pub type PmuWakeTrig0Clr = crate::Reg<pmu_wake_trig0_clr::PmuWakeTrig0ClrSpec>;
///GPIO interrupt clear register, slots 0–11 (write 1<<(16+slot) to clear)
pub mod pmu_wake_trig0_clr;
/**PMU_WAKE_TRIG1_CLR (rw) register accessor: GPIO interrupt clear register, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig1_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig1_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig1_clr`] module*/
#[doc(alias = "PMU_WAKE_TRIG1_CLR")]
pub type PmuWakeTrig1Clr = crate::Reg<pmu_wake_trig1_clr::PmuWakeTrig1ClrSpec>;
///GPIO interrupt clear register, second bank
pub mod pmu_wake_trig1_clr;
/**PMU_WAKE_TRIG0_RAW (r) register accessor: GPIO interrupt raw (pre-mask) status, slots 0–11

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig0_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig0_raw`] module*/
#[doc(alias = "PMU_WAKE_TRIG0_RAW")]
pub type PmuWakeTrig0Raw = crate::Reg<pmu_wake_trig0_raw::PmuWakeTrig0RawSpec>;
///GPIO interrupt raw (pre-mask) status, slots 0–11
pub mod pmu_wake_trig0_raw;
/**PMU_WAKE_TRIG1_RAW (r) register accessor: GPIO interrupt raw status, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig1_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig1_raw`] module*/
#[doc(alias = "PMU_WAKE_TRIG1_RAW")]
pub type PmuWakeTrig1Raw = crate::Reg<pmu_wake_trig1_raw::PmuWakeTrig1RawSpec>;
///GPIO interrupt raw status, second bank
pub mod pmu_wake_trig1_raw;
/**PMU_WAKE_TRIG0 (r) register accessor: GPIO interrupt masked status, slots 0–11 (ISR reads this)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig0`] module*/
#[doc(alias = "PMU_WAKE_TRIG0")]
pub type PmuWakeTrig0 = crate::Reg<pmu_wake_trig0::PmuWakeTrig0Spec>;
///GPIO interrupt masked status, slots 0–11 (ISR reads this)
pub mod pmu_wake_trig0;
/**PMU_WAKE_TRIG1 (r) register accessor: GPIO interrupt masked status, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig1`] module*/
#[doc(alias = "PMU_WAKE_TRIG1")]
pub type PmuWakeTrig1 = crate::Reg<pmu_wake_trig1::PmuWakeTrig1Spec>;
///GPIO interrupt masked status, second bank
pub mod pmu_wake_trig1;
/**GNSDSP_CKEN (rw) register accessor: GNSS DSP clock enables

You can [`read`](crate::Reg::read) this register and get [`gnsdsp_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnsdsp_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gnsdsp_cken`] module*/
#[doc(alias = "GNSDSP_CKEN")]
pub type GnsdspCken = crate::Reg<gnsdsp_cken::GnsdspCkenSpec>;
///GNSS DSP clock enables
pub mod gnsdsp_cken;
/**GNSS_DIV (rw) register accessor: GNSS clock divider

You can [`read`](crate::Reg::read) this register and get [`gnss_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnss_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gnss_div`] module*/
#[doc(alias = "GNSS_DIV")]
pub type GnssDiv = crate::Reg<gnss_div::GnssDivSpec>;
///GNSS clock divider
pub mod gnss_div;
/**CHIP_ID (r) register accessor: Chip identification register (read-only)

You can [`read`](crate::Reg::read) this register and get [`chip_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chip_id`] module*/
#[doc(alias = "CHIP_ID")]
pub type ChipId = crate::Reg<chip_id::ChipIdSpec>;
///Chip identification register (read-only)
pub mod chip_id;
