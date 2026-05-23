#[repr(C)]
///Register block
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
    ///0x00 - Control register 0
    #[inline(always)]
    pub const fn sspcr0(&self) -> &Sspcr0 {
        &self.sspcr0
    }
    ///0x04 - Control register 1
    #[inline(always)]
    pub const fn sspcr1(&self) -> &Sspcr1 {
        &self.sspcr1
    }
    ///0x08 - Data register
    #[inline(always)]
    pub const fn sspdr(&self) -> &Sspdr {
        &self.sspdr
    }
    ///0x0c - Status register
    #[inline(always)]
    pub const fn sspsr(&self) -> &Sspsr {
        &self.sspsr
    }
    ///0x10 - Clock prescale register
    #[inline(always)]
    pub const fn sspcpsr(&self) -> &Sspcpsr {
        &self.sspcpsr
    }
    ///0x14 - Interrupt mask set or clear register
    #[inline(always)]
    pub const fn sspimsc(&self) -> &Sspimsc {
        &self.sspimsc
    }
    ///0x18 - Raw interrupt status register
    #[inline(always)]
    pub const fn sspris(&self) -> &Sspris {
        &self.sspris
    }
    ///0x1c - Masked interrupt status register
    #[inline(always)]
    pub const fn sspmis(&self) -> &Sspmis {
        &self.sspmis
    }
    ///0x20 - Interrupt clear register
    #[inline(always)]
    pub const fn sspicr(&self) -> &Sspicr {
        &self.sspicr
    }
    ///0x24 - DMA control register
    #[inline(always)]
    pub const fn sspdmacr(&self) -> &Sspdmacr {
        &self.sspdmacr
    }
}
/**SSPCR0 (rw) register accessor: Control register 0

You can [`read`](crate::Reg::read) this register and get [`sspcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspcr0`] module*/
#[doc(alias = "SSPCR0")]
pub type Sspcr0 = crate::Reg<sspcr0::Sspcr0Spec>;
///Control register 0
pub mod sspcr0;
/**SSPCR1 (rw) register accessor: Control register 1

You can [`read`](crate::Reg::read) this register and get [`sspcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspcr1`] module*/
#[doc(alias = "SSPCR1")]
pub type Sspcr1 = crate::Reg<sspcr1::Sspcr1Spec>;
///Control register 1
pub mod sspcr1;
/**SSPDR (rw) register accessor: Data register

You can [`read`](crate::Reg::read) this register and get [`sspdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspdr`] module*/
#[doc(alias = "SSPDR")]
pub type Sspdr = crate::Reg<sspdr::SspdrSpec>;
///Data register
pub mod sspdr;
/**SSPSR (r) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sspsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspsr`] module*/
#[doc(alias = "SSPSR")]
pub type Sspsr = crate::Reg<sspsr::SspsrSpec>;
///Status register
pub mod sspsr;
/**SSPCPSR (rw) register accessor: Clock prescale register

You can [`read`](crate::Reg::read) this register and get [`sspcpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspcpsr`] module*/
#[doc(alias = "SSPCPSR")]
pub type Sspcpsr = crate::Reg<sspcpsr::SspcpsrSpec>;
///Clock prescale register
pub mod sspcpsr;
/**SSPIMSC (rw) register accessor: Interrupt mask set or clear register

You can [`read`](crate::Reg::read) this register and get [`sspimsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspimsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspimsc`] module*/
#[doc(alias = "SSPIMSC")]
pub type Sspimsc = crate::Reg<sspimsc::SspimscSpec>;
///Interrupt mask set or clear register
pub mod sspimsc;
/**SSPRIS (r) register accessor: Raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`sspris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspris`] module*/
#[doc(alias = "SSPRIS")]
pub type Sspris = crate::Reg<sspris::SsprisSpec>;
///Raw interrupt status register
pub mod sspris;
/**SSPMIS (r) register accessor: Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`sspmis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspmis`] module*/
#[doc(alias = "SSPMIS")]
pub type Sspmis = crate::Reg<sspmis::SspmisSpec>;
///Masked interrupt status register
pub mod sspmis;
/**SSPICR (w) register accessor: Interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspicr`] module*/
#[doc(alias = "SSPICR")]
pub type Sspicr = crate::Reg<sspicr::SspicrSpec>;
///Interrupt clear register
pub mod sspicr;
/**SSPDMACR (rw) register accessor: DMA control register

You can [`read`](crate::Reg::read) this register and get [`sspdmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sspdmacr`] module*/
#[doc(alias = "SSPDMACR")]
pub type Sspdmacr = crate::Reg<sspdmacr::SspdmacrSpec>;
///DMA control register
pub mod sspdmacr;
