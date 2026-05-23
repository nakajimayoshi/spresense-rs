#[repr(C)]
///Register block
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
    _reserved14: [u8; 0xc8],
    dmacc0src_addr: Dmacc0srcAddr,
    dmacc0dest_addr: Dmacc0destAddr,
    dmacc0lli: Dmacc0lli,
    dmacc0control: Dmacc0control,
    dmacc0configuration: Dmacc0configuration,
    _reserved19: [u8; 0x0c],
    dmacc1src_addr: Dmacc1srcAddr,
    dmacc1dest_addr: Dmacc1destAddr,
    dmacc1lli: Dmacc1lli,
    dmacc1control: Dmacc1control,
    dmacc1configuration: Dmacc1configuration,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn dmacint_status(&self) -> &DmacintStatus {
        &self.dmacint_status
    }
    ///0x04 -
    #[inline(always)]
    pub const fn dmacint_tcstatus(&self) -> &DmacintTcstatus {
        &self.dmacint_tcstatus
    }
    ///0x08 -
    #[inline(always)]
    pub const fn dmacint_tcclear(&self) -> &DmacintTcclear {
        &self.dmacint_tcclear
    }
    ///0x0c -
    #[inline(always)]
    pub const fn dmacint_error_status(&self) -> &DmacintErrorStatus {
        &self.dmacint_error_status
    }
    ///0x10 -
    #[inline(always)]
    pub const fn dmacint_err_clr(&self) -> &DmacintErrClr {
        &self.dmacint_err_clr
    }
    ///0x14 -
    #[inline(always)]
    pub const fn dmacraw_int_tcstatus(&self) -> &DmacrawIntTcstatus {
        &self.dmacraw_int_tcstatus
    }
    ///0x18 -
    #[inline(always)]
    pub const fn dmacraw_int_error_status(&self) -> &DmacrawIntErrorStatus {
        &self.dmacraw_int_error_status
    }
    ///0x1c -
    #[inline(always)]
    pub const fn dmacenbld_chns(&self) -> &DmacenbldChns {
        &self.dmacenbld_chns
    }
    ///0x20 -
    #[inline(always)]
    pub const fn dmacsoft_breq(&self) -> &DmacsoftBreq {
        &self.dmacsoft_breq
    }
    ///0x24 -
    #[inline(always)]
    pub const fn dmacsoft_sreq(&self) -> &DmacsoftSreq {
        &self.dmacsoft_sreq
    }
    ///0x28 -
    #[inline(always)]
    pub const fn dmacsoft_lbreq(&self) -> &DmacsoftLbreq {
        &self.dmacsoft_lbreq
    }
    ///0x2c -
    #[inline(always)]
    pub const fn dmacsoft_lsreq(&self) -> &DmacsoftLsreq {
        &self.dmacsoft_lsreq
    }
    ///0x30 -
    #[inline(always)]
    pub const fn dmacconfiguration(&self) -> &Dmacconfiguration {
        &self.dmacconfiguration
    }
    ///0x34 -
    #[inline(always)]
    pub const fn dmacsync(&self) -> &Dmacsync {
        &self.dmacsync
    }
    ///0x100 -
    #[inline(always)]
    pub const fn dmacc0src_addr(&self) -> &Dmacc0srcAddr {
        &self.dmacc0src_addr
    }
    ///0x104 -
    #[inline(always)]
    pub const fn dmacc0dest_addr(&self) -> &Dmacc0destAddr {
        &self.dmacc0dest_addr
    }
    ///0x108 -
    #[inline(always)]
    pub const fn dmacc0lli(&self) -> &Dmacc0lli {
        &self.dmacc0lli
    }
    ///0x10c -
    #[inline(always)]
    pub const fn dmacc0control(&self) -> &Dmacc0control {
        &self.dmacc0control
    }
    ///0x110 -
    #[inline(always)]
    pub const fn dmacc0configuration(&self) -> &Dmacc0configuration {
        &self.dmacc0configuration
    }
    ///0x120 -
    #[inline(always)]
    pub const fn dmacc1src_addr(&self) -> &Dmacc1srcAddr {
        &self.dmacc1src_addr
    }
    ///0x124 -
    #[inline(always)]
    pub const fn dmacc1dest_addr(&self) -> &Dmacc1destAddr {
        &self.dmacc1dest_addr
    }
    ///0x128 -
    #[inline(always)]
    pub const fn dmacc1lli(&self) -> &Dmacc1lli {
        &self.dmacc1lli
    }
    ///0x12c -
    #[inline(always)]
    pub const fn dmacc1control(&self) -> &Dmacc1control {
        &self.dmacc1control
    }
    ///0x130 -
    #[inline(always)]
    pub const fn dmacc1configuration(&self) -> &Dmacc1configuration {
        &self.dmacc1configuration
    }
}
/**DMACIntStatus (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacint_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacint_status`] module*/
#[doc(alias = "DMACIntStatus")]
pub type DmacintStatus = crate::Reg<dmacint_status::DmacintStatusSpec>;
///
pub mod dmacint_status;
/**DMACIntTCStatus (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacint_tcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacint_tcstatus`] module*/
#[doc(alias = "DMACIntTCStatus")]
pub type DmacintTcstatus = crate::Reg<dmacint_tcstatus::DmacintTcstatusSpec>;
///
pub mod dmacint_tcstatus;
/**DMACIntTCClear (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacint_tcclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacint_tcclear`] module*/
#[doc(alias = "DMACIntTCClear")]
pub type DmacintTcclear = crate::Reg<dmacint_tcclear::DmacintTcclearSpec>;
///
pub mod dmacint_tcclear;
/**DMACIntErrorStatus (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacint_error_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacint_error_status`] module*/
#[doc(alias = "DMACIntErrorStatus")]
pub type DmacintErrorStatus = crate::Reg<dmacint_error_status::DmacintErrorStatusSpec>;
///
pub mod dmacint_error_status;
/**DMACIntErrClr (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacint_err_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacint_err_clr`] module*/
#[doc(alias = "DMACIntErrClr")]
pub type DmacintErrClr = crate::Reg<dmacint_err_clr::DmacintErrClrSpec>;
///
pub mod dmacint_err_clr;
/**DMACRawIntTCStatus (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacraw_int_tcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacraw_int_tcstatus`] module*/
#[doc(alias = "DMACRawIntTCStatus")]
pub type DmacrawIntTcstatus = crate::Reg<dmacraw_int_tcstatus::DmacrawIntTcstatusSpec>;
///
pub mod dmacraw_int_tcstatus;
/**DMACRawIntErrorStatus (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacraw_int_error_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacraw_int_error_status`] module*/
#[doc(alias = "DMACRawIntErrorStatus")]
pub type DmacrawIntErrorStatus = crate::Reg<
    dmacraw_int_error_status::DmacrawIntErrorStatusSpec,
>;
///
pub mod dmacraw_int_error_status;
/**DMACEnbldChns (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacenbld_chns::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacenbld_chns`] module*/
#[doc(alias = "DMACEnbldChns")]
pub type DmacenbldChns = crate::Reg<dmacenbld_chns::DmacenbldChnsSpec>;
///
pub mod dmacenbld_chns;
/**DMACSoftBReq (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacsoft_breq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_breq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacsoft_breq`] module*/
#[doc(alias = "DMACSoftBReq")]
pub type DmacsoftBreq = crate::Reg<dmacsoft_breq::DmacsoftBreqSpec>;
///
pub mod dmacsoft_breq;
/**DMACSoftSReq (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacsoft_sreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_sreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacsoft_sreq`] module*/
#[doc(alias = "DMACSoftSReq")]
pub type DmacsoftSreq = crate::Reg<dmacsoft_sreq::DmacsoftSreqSpec>;
///
pub mod dmacsoft_sreq;
/**DMACSoftLBReq (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacsoft_lbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_lbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacsoft_lbreq`] module*/
#[doc(alias = "DMACSoftLBReq")]
pub type DmacsoftLbreq = crate::Reg<dmacsoft_lbreq::DmacsoftLbreqSpec>;
///
pub mod dmacsoft_lbreq;
/**DMACSoftLSReq (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacsoft_lsreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_lsreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacsoft_lsreq`] module*/
#[doc(alias = "DMACSoftLSReq")]
pub type DmacsoftLsreq = crate::Reg<dmacsoft_lsreq::DmacsoftLsreqSpec>;
///
pub mod dmacsoft_lsreq;
/**DMACConfiguration (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacconfiguration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacconfiguration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacconfiguration`] module*/
#[doc(alias = "DMACConfiguration")]
pub type Dmacconfiguration = crate::Reg<dmacconfiguration::DmacconfigurationSpec>;
///
pub mod dmacconfiguration;
/**DMACSync (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacsync`] module*/
#[doc(alias = "DMACSync")]
pub type Dmacsync = crate::Reg<dmacsync::DmacsyncSpec>;
///
pub mod dmacsync;
/**DMACC0SrcAddr (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacc0src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacc0src_addr`] module*/
#[doc(alias = "DMACC0SrcAddr")]
pub type Dmacc0srcAddr = crate::Reg<dmacc0src_addr::Dmacc0srcAddrSpec>;
///
pub mod dmacc0src_addr;
/**DMACC0DestAddr (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacc0dest_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0dest_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacc0dest_addr`] module*/
#[doc(alias = "DMACC0DestAddr")]
pub type Dmacc0destAddr = crate::Reg<dmacc0dest_addr::Dmacc0destAddrSpec>;
///
pub mod dmacc0dest_addr;
/**DMACC0LLI (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacc0lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacc0lli`] module*/
#[doc(alias = "DMACC0LLI")]
pub type Dmacc0lli = crate::Reg<dmacc0lli::Dmacc0lliSpec>;
///
pub mod dmacc0lli;
/**DMACC0Control (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacc0control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacc0control`] module*/
#[doc(alias = "DMACC0Control")]
pub type Dmacc0control = crate::Reg<dmacc0control::Dmacc0controlSpec>;
///
pub mod dmacc0control;
/**DMACC0Configuration (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dmacc0configuration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0configuration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacc0configuration`] module*/
#[doc(alias = "DMACC0Configuration")]
pub type Dmacc0configuration = crate::Reg<dmacc0configuration::Dmacc0configurationSpec>;
///
pub mod dmacc0configuration;
pub use Dmacc0srcAddr as Dmacc1srcAddr;
pub use dmacc0src_addr as dmacc1src_addr;
pub use Dmacc0destAddr as Dmacc1destAddr;
pub use dmacc0dest_addr as dmacc1dest_addr;
pub use Dmacc0lli as Dmacc1lli;
pub use dmacc0lli as dmacc1lli;
pub use Dmacc0control as Dmacc1control;
pub use dmacc0control as dmacc1control;
pub use Dmacc0configuration as Dmacc1configuration;
pub use dmacc0configuration as dmacc1configuration;
