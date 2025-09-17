#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sspcr0: Sspcr0,
    sspcr1: Sspcr1,
    sspdr: Sspdr,
    sspsr: Sspsr,
    sspcpsr: Sspcpsr,
    sspimsc: Sspimsc,
    sspris: Sspris,
    sspmis: Sspmis,
    sspicr: Sspicr,
    sspdmacr: Sspdmacr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    #[inline(always)]
    pub const fn sspcr0(&self) -> &Sspcr0 {
        &self.sspcr0
    }
    #[doc = "0x04 - Control register 1"]
    #[inline(always)]
    pub const fn sspcr1(&self) -> &Sspcr1 {
        &self.sspcr1
    }
    #[doc = "0x08 - Data register"]
    #[inline(always)]
    pub const fn sspdr(&self) -> &Sspdr {
        &self.sspdr
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn sspsr(&self) -> &Sspsr {
        &self.sspsr
    }
    #[doc = "0x10 - Clock prescale register"]
    #[inline(always)]
    pub const fn sspcpsr(&self) -> &Sspcpsr {
        &self.sspcpsr
    }
    #[doc = "0x14 - Interrupt mask set or clear register"]
    #[inline(always)]
    pub const fn sspimsc(&self) -> &Sspimsc {
        &self.sspimsc
    }
    #[doc = "0x18 - Raw interrupt status register"]
    #[inline(always)]
    pub const fn sspris(&self) -> &Sspris {
        &self.sspris
    }
    #[doc = "0x1c - Masked interrupt status register"]
    #[inline(always)]
    pub const fn sspmis(&self) -> &Sspmis {
        &self.sspmis
    }
    #[doc = "0x20 - Interrupt clear register"]
    #[inline(always)]
    pub const fn sspicr(&self) -> &Sspicr {
        &self.sspicr
    }
    #[doc = "0x24 - DMA control register"]
    #[inline(always)]
    pub const fn sspdmacr(&self) -> &Sspdmacr {
        &self.sspdmacr
    }
}
#[doc = "SSPCR0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sspcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspcr0`] module"]
#[doc(alias = "SSPCR0")]
pub type Sspcr0 = crate::Reg<sspcr0::Sspcr0Spec>;
#[doc = "Control register 0"]
pub mod sspcr0;
#[doc = "SSPCR1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sspcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspcr1`] module"]
#[doc(alias = "SSPCR1")]
pub type Sspcr1 = crate::Reg<sspcr1::Sspcr1Spec>;
#[doc = "Control register 1"]
pub mod sspcr1;
#[doc = "SSPDR (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspdr`] module"]
#[doc(alias = "SSPDR")]
pub type Sspdr = crate::Reg<sspdr::SspdrSpec>;
#[doc = "Data register"]
pub mod sspdr;
#[doc = "SSPSR (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspsr`] module"]
#[doc(alias = "SSPSR")]
pub type Sspsr = crate::Reg<sspsr::SspsrSpec>;
#[doc = "Status register"]
pub mod sspsr;
#[doc = "SSPCPSR (rw) register accessor: Clock prescale register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspcpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspcpsr`] module"]
#[doc(alias = "SSPCPSR")]
pub type Sspcpsr = crate::Reg<sspcpsr::SspcpsrSpec>;
#[doc = "Clock prescale register"]
pub mod sspcpsr;
#[doc = "SSPIMSC (rw) register accessor: Interrupt mask set or clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspimsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspimsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspimsc`] module"]
#[doc(alias = "SSPIMSC")]
pub type Sspimsc = crate::Reg<sspimsc::SspimscSpec>;
#[doc = "Interrupt mask set or clear register"]
pub mod sspimsc;
#[doc = "SSPRIS (r) register accessor: Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspris`] module"]
#[doc(alias = "SSPRIS")]
pub type Sspris = crate::Reg<sspris::SsprisSpec>;
#[doc = "Raw interrupt status register"]
pub mod sspris;
#[doc = "SSPMIS (r) register accessor: Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspmis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspmis`] module"]
#[doc(alias = "SSPMIS")]
pub type Sspmis = crate::Reg<sspmis::SspmisSpec>;
#[doc = "Masked interrupt status register"]
pub mod sspmis;
#[doc = "SSPICR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspicr`] module"]
#[doc(alias = "SSPICR")]
pub type Sspicr = crate::Reg<sspicr::SspicrSpec>;
#[doc = "Interrupt clear register"]
pub mod sspicr;
#[doc = "SSPDMACR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspdmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspdmacr`] module"]
#[doc(alias = "SSPDMACR")]
pub type Sspdmacr = crate::Reg<sspdmacr::SspdmacrSpec>;
#[doc = "DMA control register"]
pub mod sspdmacr;
