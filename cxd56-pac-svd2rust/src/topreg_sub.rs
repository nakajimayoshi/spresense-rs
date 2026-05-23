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
    _reserved7: [u8; 0x07fc],
    gnsdsp_cken: GnsdspCken,
    _reserved8: [u8; 0x04],
    gnss_div: GnssDiv,
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
