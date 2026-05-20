#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    rsr: Rsr,
    _reserved2: [u8; 0x10],
    fr: Fr,
    rto: Rto,
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
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x04 - Receive Status and Clear Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &Rsr {
        &self.rsr
    }
    #[doc = "0x18 - Flags Register"]
    #[inline(always)]
    pub const fn fr(&self) -> &Fr {
        &self.fr
    }
    #[doc = "0x1c - Receive Timeout Configuration Register"]
    #[inline(always)]
    pub const fn rto(&self) -> &Rto {
        &self.rto
    }
    #[doc = "0x20 - IrDA Low-Power Counter Register"]
    #[inline(always)]
    pub const fn ilpr(&self) -> &Ilpr {
        &self.ilpr
    }
    #[doc = "0x24 - The integer part of the baud rate divisor"]
    #[inline(always)]
    pub const fn ibrd(&self) -> &Ibrd {
        &self.ibrd
    }
    #[doc = "0x28 - The fractional part of the baud rate divisor"]
    #[inline(always)]
    pub const fn fbrd(&self) -> &Fbrd {
        &self.fbrd
    }
    #[doc = "0x2c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr_h(&self) -> &LcrH {
        &self.lcr_h
    }
    #[doc = "0x30 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x34 - Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x38 - Interrupt Mask Set and Clear Register"]
    #[inline(always)]
    pub const fn imsc(&self) -> &Imsc {
        &self.imsc
    }
    #[doc = "0x3c - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x40 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x44 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x48 - DMA Control Regsiter"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
}
#[doc = "DR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = ""]
pub mod dr;
#[doc = "RSR (r) register accessor: Receive Status and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`] module"]
#[doc(alias = "RSR")]
pub type Rsr = crate::Reg<rsr::RsrSpec>;
#[doc = "Receive Status and Clear Register"]
pub mod rsr;
#[doc = "FR (r) register accessor: Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`] module"]
#[doc(alias = "FR")]
pub type Fr = crate::Reg<fr::FrSpec>;
#[doc = "Flags Register"]
pub mod fr;
#[doc = "RTO (rw) register accessor: Receive Timeout Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rto`] module"]
#[doc(alias = "RTO")]
pub type Rto = crate::Reg<rto::RtoSpec>;
#[doc = "Receive Timeout Configuration Register"]
pub mod rto;
#[doc = "ILPR (rw) register accessor: IrDA Low-Power Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ilpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ilpr`] module"]
#[doc(alias = "ILPR")]
pub type Ilpr = crate::Reg<ilpr::IlprSpec>;
#[doc = "IrDA Low-Power Counter Register"]
pub mod ilpr;
#[doc = "IBRD (rw) register accessor: The integer part of the baud rate divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`] module"]
#[doc(alias = "IBRD")]
pub type Ibrd = crate::Reg<ibrd::IbrdSpec>;
#[doc = "The integer part of the baud rate divisor"]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: The fractional part of the baud rate divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`] module"]
#[doc(alias = "FBRD")]
pub type Fbrd = crate::Reg<fbrd::FbrdSpec>;
#[doc = "The fractional part of the baud rate divisor"]
pub mod fbrd;
#[doc = "LCR_H (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr_h`] module"]
#[doc(alias = "LCR_H")]
pub type LcrH = crate::Reg<lcr_h::LcrHSpec>;
#[doc = "Line Control Register"]
pub mod lcr_h;
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IFLS (rw) register accessor: Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`] module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod ifls;
#[doc = "IMSC (rw) register accessor: Interrupt Mask Set and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imsc`] module"]
#[doc(alias = "IMSC")]
pub type Imsc = crate::Reg<imsc::ImscSpec>;
#[doc = "Interrupt Mask Set and Clear Register"]
pub mod imsc;
#[doc = "RIS (r) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "DMACR (rw) register accessor: DMA Control Regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`] module"]
#[doc(alias = "DMACR")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
#[doc = "DMA Control Regsiter"]
pub mod dmacr;
