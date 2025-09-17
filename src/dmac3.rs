#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmacint_status: DmacintStatus,
    dmacint_tcstatus: DmacintTcstatus,
    dmacint_tcclear: DmacintTcclear,
    dmacint_error_status: DmacintErrorStatus,
    dmacint_err_clr: DmacintErrClr,
    dmacraw_int_tcstatus: DmacrawIntTcstatus,
    dmacraw_int_error_status: DmacrawIntErrorStatus,
    dmacenbld_chns: DmacenbldChns,
    dmacsoft_breq: DmacsoftBreq,
    dmacsoft_sreq: DmacsoftSreq,
    dmacsoft_lbreq: DmacsoftLbreq,
    dmacsoft_lsreq: DmacsoftLsreq,
    dmacconfiguration: Dmacconfiguration,
    dmacsync: Dmacsync,
    dmacsreqmask: Dmacsreqmask,
    _reserved15: [u8; 0xc4],
    dmacc0src_addr: Dmacc0srcAddr,
    dmacc0dest_addr: Dmacc0destAddr,
    dmacc0lli: Dmacc0lli,
    dmacc0control: Dmacc0control,
    dmacc0configuration: Dmacc0configuration,
    dmacc0def_lli: Dmacc0defLli,
    _reserved21: [u8; 0x08],
    dmacc1src_addr: Dmacc1srcAddr,
    dmacc1dest_addr: Dmacc1destAddr,
    dmacc1lli: Dmacc1lli,
    dmacc1control: Dmacc1control,
    dmacc1configuration: Dmacc1configuration,
    dmacc1def_lli: Dmacc1defLli,
    _reserved27: [u8; 0x08],
    dmacc2src_addr: Dmacc2srcAddr,
    dmacc2dest_addr: Dmacc2destAddr,
    dmacc2lli: Dmacc2lli,
    dmacc2control: Dmacc2control,
    dmacc2configuration: Dmacc2configuration,
    dmacc2def_lli: Dmacc2defLli,
    _reserved33: [u8; 0x08],
    dmacc3src_addr: Dmacc3srcAddr,
    dmacc3dest_addr: Dmacc3destAddr,
    dmacc3lli: Dmacc3lli,
    dmacc3control: Dmacc3control,
    dmacc3configuration: Dmacc3configuration,
    dmacc3def_lli: Dmacc3defLli,
    _reserved39: [u8; 0x08],
    dmacc4src_addr: Dmacc4srcAddr,
    dmacc4dest_addr: Dmacc4destAddr,
    dmacc4lli: Dmacc4lli,
    dmacc4control: Dmacc4control,
    dmacc4configuration: Dmacc4configuration,
    dmacc4def_lli: Dmacc4defLli,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn dmacint_status(&self) -> &DmacintStatus {
        &self.dmacint_status
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn dmacint_tcstatus(&self) -> &DmacintTcstatus {
        &self.dmacint_tcstatus
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn dmacint_tcclear(&self) -> &DmacintTcclear {
        &self.dmacint_tcclear
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn dmacint_error_status(&self) -> &DmacintErrorStatus {
        &self.dmacint_error_status
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn dmacint_err_clr(&self) -> &DmacintErrClr {
        &self.dmacint_err_clr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dmacraw_int_tcstatus(&self) -> &DmacrawIntTcstatus {
        &self.dmacraw_int_tcstatus
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn dmacraw_int_error_status(&self) -> &DmacrawIntErrorStatus {
        &self.dmacraw_int_error_status
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn dmacenbld_chns(&self) -> &DmacenbldChns {
        &self.dmacenbld_chns
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn dmacsoft_breq(&self) -> &DmacsoftBreq {
        &self.dmacsoft_breq
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn dmacsoft_sreq(&self) -> &DmacsoftSreq {
        &self.dmacsoft_sreq
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn dmacsoft_lbreq(&self) -> &DmacsoftLbreq {
        &self.dmacsoft_lbreq
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn dmacsoft_lsreq(&self) -> &DmacsoftLsreq {
        &self.dmacsoft_lsreq
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn dmacconfiguration(&self) -> &Dmacconfiguration {
        &self.dmacconfiguration
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn dmacsync(&self) -> &Dmacsync {
        &self.dmacsync
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn dmacsreqmask(&self) -> &Dmacsreqmask {
        &self.dmacsreqmask
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn dmacc0src_addr(&self) -> &Dmacc0srcAddr {
        &self.dmacc0src_addr
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn dmacc0dest_addr(&self) -> &Dmacc0destAddr {
        &self.dmacc0dest_addr
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn dmacc0lli(&self) -> &Dmacc0lli {
        &self.dmacc0lli
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn dmacc0control(&self) -> &Dmacc0control {
        &self.dmacc0control
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn dmacc0configuration(&self) -> &Dmacc0configuration {
        &self.dmacc0configuration
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn dmacc0def_lli(&self) -> &Dmacc0defLli {
        &self.dmacc0def_lli
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn dmacc1src_addr(&self) -> &Dmacc1srcAddr {
        &self.dmacc1src_addr
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn dmacc1dest_addr(&self) -> &Dmacc1destAddr {
        &self.dmacc1dest_addr
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn dmacc1lli(&self) -> &Dmacc1lli {
        &self.dmacc1lli
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn dmacc1control(&self) -> &Dmacc1control {
        &self.dmacc1control
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn dmacc1configuration(&self) -> &Dmacc1configuration {
        &self.dmacc1configuration
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn dmacc1def_lli(&self) -> &Dmacc1defLli {
        &self.dmacc1def_lli
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn dmacc2src_addr(&self) -> &Dmacc2srcAddr {
        &self.dmacc2src_addr
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn dmacc2dest_addr(&self) -> &Dmacc2destAddr {
        &self.dmacc2dest_addr
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn dmacc2lli(&self) -> &Dmacc2lli {
        &self.dmacc2lli
    }
    #[doc = "0x14c - "]
    #[inline(always)]
    pub const fn dmacc2control(&self) -> &Dmacc2control {
        &self.dmacc2control
    }
    #[doc = "0x150 - "]
    #[inline(always)]
    pub const fn dmacc2configuration(&self) -> &Dmacc2configuration {
        &self.dmacc2configuration
    }
    #[doc = "0x154 - "]
    #[inline(always)]
    pub const fn dmacc2def_lli(&self) -> &Dmacc2defLli {
        &self.dmacc2def_lli
    }
    #[doc = "0x160 - "]
    #[inline(always)]
    pub const fn dmacc3src_addr(&self) -> &Dmacc3srcAddr {
        &self.dmacc3src_addr
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn dmacc3dest_addr(&self) -> &Dmacc3destAddr {
        &self.dmacc3dest_addr
    }
    #[doc = "0x168 - "]
    #[inline(always)]
    pub const fn dmacc3lli(&self) -> &Dmacc3lli {
        &self.dmacc3lli
    }
    #[doc = "0x16c - "]
    #[inline(always)]
    pub const fn dmacc3control(&self) -> &Dmacc3control {
        &self.dmacc3control
    }
    #[doc = "0x170 - "]
    #[inline(always)]
    pub const fn dmacc3configuration(&self) -> &Dmacc3configuration {
        &self.dmacc3configuration
    }
    #[doc = "0x174 - "]
    #[inline(always)]
    pub const fn dmacc3def_lli(&self) -> &Dmacc3defLli {
        &self.dmacc3def_lli
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn dmacc4src_addr(&self) -> &Dmacc4srcAddr {
        &self.dmacc4src_addr
    }
    #[doc = "0x184 - "]
    #[inline(always)]
    pub const fn dmacc4dest_addr(&self) -> &Dmacc4destAddr {
        &self.dmacc4dest_addr
    }
    #[doc = "0x188 - "]
    #[inline(always)]
    pub const fn dmacc4lli(&self) -> &Dmacc4lli {
        &self.dmacc4lli
    }
    #[doc = "0x18c - "]
    #[inline(always)]
    pub const fn dmacc4control(&self) -> &Dmacc4control {
        &self.dmacc4control
    }
    #[doc = "0x190 - "]
    #[inline(always)]
    pub const fn dmacc4configuration(&self) -> &Dmacc4configuration {
        &self.dmacc4configuration
    }
    #[doc = "0x194 - "]
    #[inline(always)]
    pub const fn dmacc4def_lli(&self) -> &Dmacc4defLli {
        &self.dmacc4def_lli
    }
}
#[doc = "DMACIntStatus (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacint_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacint_status`] module"]
#[doc(alias = "DMACIntStatus")]
pub type DmacintStatus = crate::Reg<dmacint_status::DmacintStatusSpec>;
#[doc = ""]
pub mod dmacint_status;
#[doc = "DMACIntTCStatus (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacint_tcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacint_tcstatus`] module"]
#[doc(alias = "DMACIntTCStatus")]
pub type DmacintTcstatus = crate::Reg<dmacint_tcstatus::DmacintTcstatusSpec>;
#[doc = ""]
pub mod dmacint_tcstatus;
#[doc = "DMACIntTCClear (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacint_tcclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacint_tcclear`] module"]
#[doc(alias = "DMACIntTCClear")]
pub type DmacintTcclear = crate::Reg<dmacint_tcclear::DmacintTcclearSpec>;
#[doc = ""]
pub mod dmacint_tcclear;
#[doc = "DMACIntErrorStatus (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacint_error_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacint_error_status`] module"]
#[doc(alias = "DMACIntErrorStatus")]
pub type DmacintErrorStatus = crate::Reg<dmacint_error_status::DmacintErrorStatusSpec>;
#[doc = ""]
pub mod dmacint_error_status;
#[doc = "DMACIntErrClr (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacint_err_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacint_err_clr`] module"]
#[doc(alias = "DMACIntErrClr")]
pub type DmacintErrClr = crate::Reg<dmacint_err_clr::DmacintErrClrSpec>;
#[doc = ""]
pub mod dmacint_err_clr;
#[doc = "DMACRawIntTCStatus (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacraw_int_tcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacraw_int_tcstatus`] module"]
#[doc(alias = "DMACRawIntTCStatus")]
pub type DmacrawIntTcstatus = crate::Reg<dmacraw_int_tcstatus::DmacrawIntTcstatusSpec>;
#[doc = ""]
pub mod dmacraw_int_tcstatus;
#[doc = "DMACRawIntErrorStatus (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacraw_int_error_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacraw_int_error_status`] module"]
#[doc(alias = "DMACRawIntErrorStatus")]
pub type DmacrawIntErrorStatus = crate::Reg<dmacraw_int_error_status::DmacrawIntErrorStatusSpec>;
#[doc = ""]
pub mod dmacraw_int_error_status;
#[doc = "DMACEnbldChns (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacenbld_chns::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacenbld_chns`] module"]
#[doc(alias = "DMACEnbldChns")]
pub type DmacenbldChns = crate::Reg<dmacenbld_chns::DmacenbldChnsSpec>;
#[doc = ""]
pub mod dmacenbld_chns;
#[doc = "DMACSoftBReq (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsoft_breq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_breq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsoft_breq`] module"]
#[doc(alias = "DMACSoftBReq")]
pub type DmacsoftBreq = crate::Reg<dmacsoft_breq::DmacsoftBreqSpec>;
#[doc = ""]
pub mod dmacsoft_breq;
#[doc = "DMACSoftSReq (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsoft_sreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_sreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsoft_sreq`] module"]
#[doc(alias = "DMACSoftSReq")]
pub type DmacsoftSreq = crate::Reg<dmacsoft_sreq::DmacsoftSreqSpec>;
#[doc = ""]
pub mod dmacsoft_sreq;
#[doc = "DMACSoftLBReq (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsoft_lbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_lbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsoft_lbreq`] module"]
#[doc(alias = "DMACSoftLBReq")]
pub type DmacsoftLbreq = crate::Reg<dmacsoft_lbreq::DmacsoftLbreqSpec>;
#[doc = ""]
pub mod dmacsoft_lbreq;
#[doc = "DMACSoftLSReq (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsoft_lsreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_lsreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsoft_lsreq`] module"]
#[doc(alias = "DMACSoftLSReq")]
pub type DmacsoftLsreq = crate::Reg<dmacsoft_lsreq::DmacsoftLsreqSpec>;
#[doc = ""]
pub mod dmacsoft_lsreq;
#[doc = "DMACConfiguration (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacconfiguration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacconfiguration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacconfiguration`] module"]
#[doc(alias = "DMACConfiguration")]
pub type Dmacconfiguration = crate::Reg<dmacconfiguration::DmacconfigurationSpec>;
#[doc = ""]
pub mod dmacconfiguration;
#[doc = "DMACSync (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsync`] module"]
#[doc(alias = "DMACSync")]
pub type Dmacsync = crate::Reg<dmacsync::DmacsyncSpec>;
#[doc = ""]
pub mod dmacsync;
#[doc = "DMACSREQMask (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsreqmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsreqmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsreqmask`] module"]
#[doc(alias = "DMACSREQMask")]
pub type Dmacsreqmask = crate::Reg<dmacsreqmask::DmacsreqmaskSpec>;
#[doc = ""]
pub mod dmacsreqmask;
#[doc = "DMACC0SrcAddr (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacc0src_addr`] module"]
#[doc(alias = "DMACC0SrcAddr")]
pub type Dmacc0srcAddr = crate::Reg<dmacc0src_addr::Dmacc0srcAddrSpec>;
#[doc = ""]
pub mod dmacc0src_addr;
#[doc = "DMACC0DestAddr (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0dest_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0dest_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacc0dest_addr`] module"]
#[doc(alias = "DMACC0DestAddr")]
pub type Dmacc0destAddr = crate::Reg<dmacc0dest_addr::Dmacc0destAddrSpec>;
#[doc = ""]
pub mod dmacc0dest_addr;
#[doc = "DMACC0LLI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacc0lli`] module"]
#[doc(alias = "DMACC0LLI")]
pub type Dmacc0lli = crate::Reg<dmacc0lli::Dmacc0lliSpec>;
#[doc = ""]
pub mod dmacc0lli;
#[doc = "DMACC0Control (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacc0control`] module"]
#[doc(alias = "DMACC0Control")]
pub type Dmacc0control = crate::Reg<dmacc0control::Dmacc0controlSpec>;
#[doc = ""]
pub mod dmacc0control;
#[doc = "DMACC0DefLLI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0def_lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0def_lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacc0def_lli`] module"]
#[doc(alias = "DMACC0DefLLI")]
pub type Dmacc0defLli = crate::Reg<dmacc0def_lli::Dmacc0defLliSpec>;
#[doc = ""]
pub mod dmacc0def_lli;
#[doc = "DMACC0Configuration (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dmacc0configuration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0configuration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacc0configuration`] module"]
#[doc(alias = "DMACC0Configuration")]
pub type Dmacc0configuration = crate::Reg<dmacc0configuration::Dmacc0configurationSpec>;
#[doc = ""]
pub mod dmacc0configuration;
pub use Dmacc0configuration as Dmacc1configuration;
pub use Dmacc0configuration as Dmacc2configuration;
pub use Dmacc0configuration as Dmacc3configuration;
pub use Dmacc0configuration as Dmacc4configuration;
pub use Dmacc0control as Dmacc1control;
pub use Dmacc0control as Dmacc2control;
pub use Dmacc0control as Dmacc3control;
pub use Dmacc0control as Dmacc4control;
pub use Dmacc0defLli as Dmacc1defLli;
pub use Dmacc0defLli as Dmacc2defLli;
pub use Dmacc0defLli as Dmacc3defLli;
pub use Dmacc0defLli as Dmacc4defLli;
pub use Dmacc0destAddr as Dmacc1destAddr;
pub use Dmacc0destAddr as Dmacc2destAddr;
pub use Dmacc0destAddr as Dmacc3destAddr;
pub use Dmacc0destAddr as Dmacc4destAddr;
pub use Dmacc0lli as Dmacc1lli;
pub use Dmacc0lli as Dmacc2lli;
pub use Dmacc0lli as Dmacc3lli;
pub use Dmacc0lli as Dmacc4lli;
pub use Dmacc0srcAddr as Dmacc1srcAddr;
pub use Dmacc0srcAddr as Dmacc2srcAddr;
pub use Dmacc0srcAddr as Dmacc3srcAddr;
pub use Dmacc0srcAddr as Dmacc4srcAddr;
pub use dmacc0configuration as dmacc1configuration;
pub use dmacc0configuration as dmacc2configuration;
pub use dmacc0configuration as dmacc3configuration;
pub use dmacc0configuration as dmacc4configuration;
pub use dmacc0control as dmacc1control;
pub use dmacc0control as dmacc2control;
pub use dmacc0control as dmacc3control;
pub use dmacc0control as dmacc4control;
pub use dmacc0def_lli as dmacc1def_lli;
pub use dmacc0def_lli as dmacc2def_lli;
pub use dmacc0def_lli as dmacc3def_lli;
pub use dmacc0def_lli as dmacc4def_lli;
pub use dmacc0dest_addr as dmacc1dest_addr;
pub use dmacc0dest_addr as dmacc2dest_addr;
pub use dmacc0dest_addr as dmacc3dest_addr;
pub use dmacc0dest_addr as dmacc4dest_addr;
pub use dmacc0lli as dmacc1lli;
pub use dmacc0lli as dmacc2lli;
pub use dmacc0lli as dmacc3lli;
pub use dmacc0lli as dmacc4lli;
pub use dmacc0src_addr as dmacc1src_addr;
pub use dmacc0src_addr as dmacc2src_addr;
pub use dmacc0src_addr as dmacc3src_addr;
pub use dmacc0src_addr as dmacc4src_addr;
