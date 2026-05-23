#[repr(C)]
///Register block
pub struct RegisterBlock {
    dr: Dr,
    rsr: Rsr,
    _reserved2: [u8; 0x10],
    fr: Fr,
    _reserved3: [u8; 0x04],
    ilpr: Ilpr,
    ibrd: Ibrd,
    fbrd: Fbrd,
    lcr_h: LcrH,
    cr: Cr,
    ifls: Ifls,
    imsc: Imsc,
    ris: Ris,
    mis: Mis,
    icr: Icr,
    dmacr: Dmacr,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    ///0x04 - Receive Status and Clear Register
    #[inline(always)]
    pub const fn rsr(&self) -> &Rsr {
        &self.rsr
    }
    ///0x18 - Flags Register
    #[inline(always)]
    pub const fn fr(&self) -> &Fr {
        &self.fr
    }
    ///0x20 - IrDA Low-Power Counter Register
    #[inline(always)]
    pub const fn ilpr(&self) -> &Ilpr {
        &self.ilpr
    }
    ///0x24 - The integer part of the baud rate divisor
    #[inline(always)]
    pub const fn ibrd(&self) -> &Ibrd {
        &self.ibrd
    }
    ///0x28 - The fractional part of the baud rate divisor
    #[inline(always)]
    pub const fn fbrd(&self) -> &Fbrd {
        &self.fbrd
    }
    ///0x2c - Line Control Register
    #[inline(always)]
    pub const fn lcr_h(&self) -> &LcrH {
        &self.lcr_h
    }
    ///0x30 - Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    ///0x34 - Interrupt FIFO Level Select Register
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    ///0x38 - Interrupt Mask Set and Clear Register
    #[inline(always)]
    pub const fn imsc(&self) -> &Imsc {
        &self.imsc
    }
    ///0x3c - Raw Interrupt Status Register
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    ///0x40 - Masked Interrupt Status Register
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    ///0x44 - Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    ///0x48 - DMA Control Regsiter
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
}
/**DR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dr`] module*/
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
///
pub mod dr;
/**RSR (r) register accessor: Receive Status and Clear Register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rsr`] module*/
#[doc(alias = "RSR")]
pub type Rsr = crate::Reg<rsr::RsrSpec>;
///Receive Status and Clear Register
pub mod rsr;
/**FR (r) register accessor: Flags Register

You can [`read`](crate::Reg::read) this register and get [`fr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fr`] module*/
#[doc(alias = "FR")]
pub type Fr = crate::Reg<fr::FrSpec>;
///Flags Register
pub mod fr;
/**ILPR (rw) register accessor: IrDA Low-Power Counter Register

You can [`read`](crate::Reg::read) this register and get [`ilpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ilpr`] module*/
#[doc(alias = "ILPR")]
pub type Ilpr = crate::Reg<ilpr::IlprSpec>;
///IrDA Low-Power Counter Register
pub mod ilpr;
/**IBRD (rw) register accessor: The integer part of the baud rate divisor

You can [`read`](crate::Reg::read) this register and get [`ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ibrd`] module*/
#[doc(alias = "IBRD")]
pub type Ibrd = crate::Reg<ibrd::IbrdSpec>;
///The integer part of the baud rate divisor
pub mod ibrd;
/**FBRD (rw) register accessor: The fractional part of the baud rate divisor

You can [`read`](crate::Reg::read) this register and get [`fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fbrd`] module*/
#[doc(alias = "FBRD")]
pub type Fbrd = crate::Reg<fbrd::FbrdSpec>;
///The fractional part of the baud rate divisor
pub mod fbrd;
/**LCR_H (rw) register accessor: Line Control Register

You can [`read`](crate::Reg::read) this register and get [`lcr_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcr_h`] module*/
#[doc(alias = "LCR_H")]
pub type LcrH = crate::Reg<lcr_h::LcrHSpec>;
///Line Control Register
pub mod lcr_h;
/**CR (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr`] module*/
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
///Control Register
pub mod cr;
/**IFLS (rw) register accessor: Interrupt FIFO Level Select Register

You can [`read`](crate::Reg::read) this register and get [`ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ifls`] module*/
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
///Interrupt FIFO Level Select Register
pub mod ifls;
/**IMSC (rw) register accessor: Interrupt Mask Set and Clear Register

You can [`read`](crate::Reg::read) this register and get [`imsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imsc`] module*/
#[doc(alias = "IMSC")]
pub type Imsc = crate::Reg<imsc::ImscSpec>;
///Interrupt Mask Set and Clear Register
pub mod imsc;
/**RIS (r) register accessor: Raw Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ris`] module*/
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
///Raw Interrupt Status Register
pub mod ris;
/**MIS (r) register accessor: Masked Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mis`] module*/
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
///Masked Interrupt Status Register
pub mod mis;
/**ICR (w) register accessor: Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icr`] module*/
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
///Interrupt Clear Register
pub mod icr;
/**DMACR (rw) register accessor: DMA Control Regsiter

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacr`] module*/
#[doc(alias = "DMACR")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
///DMA Control Regsiter
pub mod dmacr;
