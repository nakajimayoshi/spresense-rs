#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fif_push_full: FifPushFull,
    fif_push_wrd0: FifPushWrd0,
    fif_push_wrd1: FifPushWrd1,
    fif_push_cmp: FifPushCmp,
    fif_pull_emp: FifPullEmp,
    fif_pull_wrd0: FifPullWrd0,
    fif_pull_wrd1: FifPullWrd1,
    fif_pull_cmp: FifPullCmp,
}
impl RegisterBlock {
    #[doc = "0x00 - TX buffer is full (=1)"]
    #[inline(always)]
    pub const fn fif_push_full(&self) -> &FifPushFull {
        &self.fif_push_full
    }
    #[doc = "0x04 - TX data word 0"]
    #[inline(always)]
    pub const fn fif_push_wrd0(&self) -> &FifPushWrd0 {
        &self.fif_push_wrd0
    }
    #[doc = "0x08 - TX data word 1"]
    #[inline(always)]
    pub const fn fif_push_wrd1(&self) -> &FifPushWrd1 {
        &self.fif_push_wrd1
    }
    #[doc = "0x0c - TX data write complete"]
    #[inline(always)]
    pub const fn fif_push_cmp(&self) -> &FifPushCmp {
        &self.fif_push_cmp
    }
    #[doc = "0x10 - RX buffer is empty (=1)"]
    #[inline(always)]
    pub const fn fif_pull_emp(&self) -> &FifPullEmp {
        &self.fif_pull_emp
    }
    #[doc = "0x14 - RX data word 0"]
    #[inline(always)]
    pub const fn fif_pull_wrd0(&self) -> &FifPullWrd0 {
        &self.fif_pull_wrd0
    }
    #[doc = "0x18 - RX data word 1"]
    #[inline(always)]
    pub const fn fif_pull_wrd1(&self) -> &FifPullWrd1 {
        &self.fif_pull_wrd1
    }
    #[doc = "0x1c - RX data read complete"]
    #[inline(always)]
    pub const fn fif_pull_cmp(&self) -> &FifPullCmp {
        &self.fif_pull_cmp
    }
}
#[doc = "FIF_PUSH_FULL (r) register accessor: TX buffer is full (=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_push_full::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif_push_full`] module"]
#[doc(alias = "FIF_PUSH_FULL")]
pub type FifPushFull = crate::Reg<fif_push_full::FifPushFullSpec>;
#[doc = "TX buffer is full (=1)"]
pub mod fif_push_full;
#[doc = "FIF_PUSH_WRD0 (rw) register accessor: TX data word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_push_wrd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_push_wrd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif_push_wrd0`] module"]
#[doc(alias = "FIF_PUSH_WRD0")]
pub type FifPushWrd0 = crate::Reg<fif_push_wrd0::FifPushWrd0Spec>;
#[doc = "TX data word 0"]
pub mod fif_push_wrd0;
#[doc = "FIF_PUSH_WRD1 (rw) register accessor: TX data word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_push_wrd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_push_wrd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif_push_wrd1`] module"]
#[doc(alias = "FIF_PUSH_WRD1")]
pub type FifPushWrd1 = crate::Reg<fif_push_wrd1::FifPushWrd1Spec>;
#[doc = "TX data word 1"]
pub mod fif_push_wrd1;
#[doc = "FIF_PUSH_CMP (w) register accessor: TX data write complete\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_push_cmp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif_push_cmp`] module"]
#[doc(alias = "FIF_PUSH_CMP")]
pub type FifPushCmp = crate::Reg<fif_push_cmp::FifPushCmpSpec>;
#[doc = "TX data write complete"]
pub mod fif_push_cmp;
#[doc = "FIF_PULL_EMP (r) register accessor: RX buffer is empty (=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_pull_emp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif_pull_emp`] module"]
#[doc(alias = "FIF_PULL_EMP")]
pub type FifPullEmp = crate::Reg<fif_pull_emp::FifPullEmpSpec>;
#[doc = "RX buffer is empty (=1)"]
pub mod fif_pull_emp;
#[doc = "FIF_PULL_WRD0 (rw) register accessor: RX data word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_pull_wrd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_pull_wrd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif_pull_wrd0`] module"]
#[doc(alias = "FIF_PULL_WRD0")]
pub type FifPullWrd0 = crate::Reg<fif_pull_wrd0::FifPullWrd0Spec>;
#[doc = "RX data word 0"]
pub mod fif_pull_wrd0;
#[doc = "FIF_PULL_WRD1 (rw) register accessor: RX data word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fif_pull_wrd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_pull_wrd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif_pull_wrd1`] module"]
#[doc(alias = "FIF_PULL_WRD1")]
pub type FifPullWrd1 = crate::Reg<fif_pull_wrd1::FifPullWrd1Spec>;
#[doc = "RX data word 1"]
pub mod fif_pull_wrd1;
#[doc = "FIF_PULL_CMP (w) register accessor: RX data read complete\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif_pull_cmp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif_pull_cmp`] module"]
#[doc(alias = "FIF_PULL_CMP")]
pub type FifPullCmp = crate::Reg<fif_pull_cmp::FifPullCmpSpec>;
#[doc = "RX data read complete"]
pub mod fif_pull_cmp;
