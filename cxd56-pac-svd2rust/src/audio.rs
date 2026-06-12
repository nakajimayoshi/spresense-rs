#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    ac_pdn: AcPdn,
    ac_clken: AcClken,
    ac_dat_sel: AcDatSel,
    _reserved3: [u8; 0xf4],
    i2s_ctrl: I2sCtrl,
    _reserved4: [u8; 0x28],
    i2s_data_en: I2sDataEn,
    _reserved5: [u8; 0x04],
    i2s_spclkerr: I2sSpclkerr,
    _reserved6: [u8; 0x01c8],
    test_ctrl: TestCtrl,
    _reserved7: [u8; 0x0274],
    lr_swap: LrSwap,
    _reserved8: [u8; 0x0a04],
    i2s1_in_start_adr: I2s1InStartAdr,
    i2s1_in_sample_no: I2s1InSampleNo,
    i2s1_in_rtd_trg: I2s1InRtdTrg,
    i2s1_in_bitwt: I2s1InBitwt,
    i2s1_in_ch_sel: I2s1InChSel,
    i2s1_in_mon: I2s1InMon,
    _reserved14: [u8; 0x28],
    i2s1_out_start_adr: I2s1OutStartAdr,
    i2s1_out_sample_no: I2s1OutSampleNo,
    i2s1_out_rtd_trg: I2s1OutRtdTrg,
    i2s1_out_bitwt: I2s1OutBitwt,
    i2s1_out_sd_sel: I2s1OutSdSel,
    i2s1_out_mon: I2s1OutMon,
    _reserved20: [u8; 0x38],
    i2s_datarate: I2sDatarate,
    _reserved21: [u8; 0x30],
    i2s1_int_ctrl: I2s1IntCtrl,
    _reserved22: [u8; 0x08],
    i2s1_int_mask: I2s1IntMask,
    _reserved23: [u8; 0x04],
    int_m: IntM,
    int_clr: IntClr,
    _reserved25: [u8; 0x90],
    ahb_i2s_clken: AhbI2sClken,
}
impl RegisterBlock {
    ///0x100 - Audio block power-down control (1 = powered down)
    #[inline(always)]
    pub const fn ac_pdn(&self) -> &AcPdn {
        &self.ac_pdn
    }
    ///0x104 - Audio block clock enables
    #[inline(always)]
    pub const fn ac_clken(&self) -> &AcClken {
        &self.ac_clken
    }
    ///0x108 - Audio data routing — AU_DAT_SEL / COD_INSEL muxes
    #[inline(always)]
    pub const fn ac_dat_sel(&self) -> &AcDatSel {
        &self.ac_dat_sel
    }
    ///0x200 - Audio Control Register 0 — I2S0 (SD1) master/slave, format and SRC routing
    #[inline(always)]
    pub const fn i2s_ctrl(&self) -> &I2sCtrl {
        &self.i2s_ctrl
    }
    ///0x22c - Audio output control — I2S0 (SD1) serial data path enables (this register also holds line-in/DAC volume; only the I2S-relevant bits are modelled)
    #[inline(always)]
    pub const fn i2s_data_en(&self) -> &I2sDataEn {
        &self.i2s_data_en
    }
    ///0x234 - I2S BCK/LRCK error interrupt mask
    #[inline(always)]
    pub const fn i2s_spclkerr(&self) -> &I2sSpclkerr {
        &self.i2s_spclkerr
    }
    ///0x400 - Audio test/control — SRC filter bypass, soft reset, SRC clock-halt inhibit
    #[inline(always)]
    pub const fn test_ctrl(&self) -> &TestCtrl {
        &self.test_ctrl
    }
    ///0x678 - I2S L/R channel swap (set together with left-justified format)
    #[inline(always)]
    pub const fn lr_swap(&self) -> &LrSwap {
        &self.lr_swap
    }
    ///0x1080 - I2S0 RX DMA buffer start address (word-aligned; byte address >> 2)
    #[inline(always)]
    pub const fn i2s1_in_start_adr(&self) -> &I2s1InStartAdr {
        &self.i2s1_in_start_adr
    }
    ///0x1084 - I2S0 RX DMA transfer length (samples - 1)
    #[inline(always)]
    pub const fn i2s1_in_sample_no(&self) -> &I2s1InSampleNo {
        &self.i2s1_in_sample_no
    }
    ///0x1088 - I2S0 RX DMA trigger
    #[inline(always)]
    pub const fn i2s1_in_rtd_trg(&self) -> &I2s1InRtdTrg {
        &self.i2s1_in_rtd_trg
    }
    ///0x108c - I2S0 RX DMA sample width
    #[inline(always)]
    pub const fn i2s1_in_bitwt(&self) -> &I2s1InBitwt {
        &self.i2s1_in_bitwt
    }
    ///0x1090 - I2S0 RX DMA channel select
    #[inline(always)]
    pub const fn i2s1_in_ch_sel(&self) -> &I2s1InChSel {
        &self.i2s1_in_ch_sel
    }
    ///0x1094 - I2S0 RX DMA monitor (read-only)
    #[inline(always)]
    pub const fn i2s1_in_mon(&self) -> &I2s1InMon {
        &self.i2s1_in_mon
    }
    ///0x10c0 - I2S0 TX DMA buffer start address (word-aligned; byte address >> 2)
    #[inline(always)]
    pub const fn i2s1_out_start_adr(&self) -> &I2s1OutStartAdr {
        &self.i2s1_out_start_adr
    }
    ///0x10c4 - I2S0 TX DMA transfer length (samples - 1)
    #[inline(always)]
    pub const fn i2s1_out_sample_no(&self) -> &I2s1OutSampleNo {
        &self.i2s1_out_sample_no
    }
    ///0x10c8 - I2S0 TX DMA trigger
    #[inline(always)]
    pub const fn i2s1_out_rtd_trg(&self) -> &I2s1OutRtdTrg {
        &self.i2s1_out_rtd_trg
    }
    ///0x10cc - I2S0 TX DMA sample width
    #[inline(always)]
    pub const fn i2s1_out_bitwt(&self) -> &I2s1OutBitwt {
        &self.i2s1_out_bitwt
    }
    ///0x10d0 - I2S0 TX DMA output channel routing
    #[inline(always)]
    pub const fn i2s1_out_sd_sel(&self) -> &I2s1OutSdSel {
        &self.i2s1_out_sd_sel
    }
    ///0x10d4 - I2S0 TX DMA monitor (read-only)
    #[inline(always)]
    pub const fn i2s1_out_mon(&self) -> &I2s1OutMon {
        &self.i2s1_out_mon
    }
    ///0x1110 - I2S sample-rate class select
    #[inline(always)]
    pub const fn i2s_datarate(&self) -> &I2sDatarate {
        &self.i2s_datarate
    }
    ///0x1144 - I2S0 DMA interrupt status (write 1 to clear)
    #[inline(always)]
    pub const fn i2s1_int_ctrl(&self) -> &I2s1IntCtrl {
        &self.i2s1_int_ctrl
    }
    ///0x1150 - I2S0 DMA interrupt mask (1=masked)
    #[inline(always)]
    pub const fn i2s1_int_mask(&self) -> &I2s1IntMask {
        &self.i2s1_int_mask
    }
    ///0x1158 - Audio bus/BCK error interrupt mask (1=masked)
    #[inline(always)]
    pub const fn int_m(&self) -> &IntM {
        &self.int_m
    }
    ///0x115c - Audio bus/BCK error interrupt clear
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    ///0x11f0 - AHB interface clock enable for the audio DMA masters
    #[inline(always)]
    pub const fn ahb_i2s_clken(&self) -> &AhbI2sClken {
        &self.ahb_i2s_clken
    }
}
/**AC_PDN (rw) register accessor: Audio block power-down control (1 = powered down)

You can [`read`](crate::Reg::read) this register and get [`ac_pdn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ac_pdn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ac_pdn`] module*/
#[doc(alias = "AC_PDN")]
pub type AcPdn = crate::Reg<ac_pdn::AcPdnSpec>;
///Audio block power-down control (1 = powered down)
pub mod ac_pdn;
/**AC_CLKEN (rw) register accessor: Audio block clock enables

You can [`read`](crate::Reg::read) this register and get [`ac_clken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ac_clken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ac_clken`] module*/
#[doc(alias = "AC_CLKEN")]
pub type AcClken = crate::Reg<ac_clken::AcClkenSpec>;
///Audio block clock enables
pub mod ac_clken;
/**AC_DAT_SEL (rw) register accessor: Audio data routing — AU_DAT_SEL / COD_INSEL muxes

You can [`read`](crate::Reg::read) this register and get [`ac_dat_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ac_dat_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ac_dat_sel`] module*/
#[doc(alias = "AC_DAT_SEL")]
pub type AcDatSel = crate::Reg<ac_dat_sel::AcDatSelSpec>;
///Audio data routing — AU_DAT_SEL / COD_INSEL muxes
pub mod ac_dat_sel;
/**I2S_CTRL (rw) register accessor: Audio Control Register 0 — I2S0 (SD1) master/slave, format and SRC routing

You can [`read`](crate::Reg::read) this register and get [`i2s_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_ctrl`] module*/
#[doc(alias = "I2S_CTRL")]
pub type I2sCtrl = crate::Reg<i2s_ctrl::I2sCtrlSpec>;
///Audio Control Register 0 — I2S0 (SD1) master/slave, format and SRC routing
pub mod i2s_ctrl;
/**I2S_DATA_EN (rw) register accessor: Audio output control — I2S0 (SD1) serial data path enables (this register also holds line-in/DAC volume; only the I2S-relevant bits are modelled)

You can [`read`](crate::Reg::read) this register and get [`i2s_data_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_data_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_data_en`] module*/
#[doc(alias = "I2S_DATA_EN")]
pub type I2sDataEn = crate::Reg<i2s_data_en::I2sDataEnSpec>;
///Audio output control — I2S0 (SD1) serial data path enables (this register also holds line-in/DAC volume; only the I2S-relevant bits are modelled)
pub mod i2s_data_en;
/**I2S_SPCLKERR (rw) register accessor: I2S BCK/LRCK error interrupt mask

You can [`read`](crate::Reg::read) this register and get [`i2s_spclkerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_spclkerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_spclkerr`] module*/
#[doc(alias = "I2S_SPCLKERR")]
pub type I2sSpclkerr = crate::Reg<i2s_spclkerr::I2sSpclkerrSpec>;
///I2S BCK/LRCK error interrupt mask
pub mod i2s_spclkerr;
/**TEST_CTRL (rw) register accessor: Audio test/control — SRC filter bypass, soft reset, SRC clock-halt inhibit

You can [`read`](crate::Reg::read) this register and get [`test_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@test_ctrl`] module*/
#[doc(alias = "TEST_CTRL")]
pub type TestCtrl = crate::Reg<test_ctrl::TestCtrlSpec>;
///Audio test/control — SRC filter bypass, soft reset, SRC clock-halt inhibit
pub mod test_ctrl;
/**LR_SWAP (rw) register accessor: I2S L/R channel swap (set together with left-justified format)

You can [`read`](crate::Reg::read) this register and get [`lr_swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr_swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lr_swap`] module*/
#[doc(alias = "LR_SWAP")]
pub type LrSwap = crate::Reg<lr_swap::LrSwapSpec>;
///I2S L/R channel swap (set together with left-justified format)
pub mod lr_swap;
/**I2S1_IN_START_ADR (rw) register accessor: I2S0 RX DMA buffer start address (word-aligned; byte address >> 2)

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_start_adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_start_adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_in_start_adr`] module*/
#[doc(alias = "I2S1_IN_START_ADR")]
pub type I2s1InStartAdr = crate::Reg<i2s1_in_start_adr::I2s1InStartAdrSpec>;
///I2S0 RX DMA buffer start address (word-aligned; byte address >> 2)
pub mod i2s1_in_start_adr;
/**I2S1_IN_SAMPLE_NO (rw) register accessor: I2S0 RX DMA transfer length (samples - 1)

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_sample_no::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_sample_no::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_in_sample_no`] module*/
#[doc(alias = "I2S1_IN_SAMPLE_NO")]
pub type I2s1InSampleNo = crate::Reg<i2s1_in_sample_no::I2s1InSampleNoSpec>;
///I2S0 RX DMA transfer length (samples - 1)
pub mod i2s1_in_sample_no;
/**I2S1_IN_RTD_TRG (rw) register accessor: I2S0 RX DMA trigger

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_rtd_trg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_rtd_trg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_in_rtd_trg`] module*/
#[doc(alias = "I2S1_IN_RTD_TRG")]
pub type I2s1InRtdTrg = crate::Reg<i2s1_in_rtd_trg::I2s1InRtdTrgSpec>;
///I2S0 RX DMA trigger
pub mod i2s1_in_rtd_trg;
/**I2S1_IN_BITWT (rw) register accessor: I2S0 RX DMA sample width

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_bitwt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_bitwt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_in_bitwt`] module*/
#[doc(alias = "I2S1_IN_BITWT")]
pub type I2s1InBitwt = crate::Reg<i2s1_in_bitwt::I2s1InBitwtSpec>;
///I2S0 RX DMA sample width
pub mod i2s1_in_bitwt;
/**I2S1_IN_CH_SEL (rw) register accessor: I2S0 RX DMA channel select

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_ch_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_ch_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_in_ch_sel`] module*/
#[doc(alias = "I2S1_IN_CH_SEL")]
pub type I2s1InChSel = crate::Reg<i2s1_in_ch_sel::I2s1InChSelSpec>;
///I2S0 RX DMA channel select
pub mod i2s1_in_ch_sel;
/**I2S1_IN_MON (r) register accessor: I2S0 RX DMA monitor (read-only)

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_mon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_in_mon`] module*/
#[doc(alias = "I2S1_IN_MON")]
pub type I2s1InMon = crate::Reg<i2s1_in_mon::I2s1InMonSpec>;
///I2S0 RX DMA monitor (read-only)
pub mod i2s1_in_mon;
/**I2S1_OUT_START_ADR (rw) register accessor: I2S0 TX DMA buffer start address (word-aligned; byte address >> 2)

You can [`read`](crate::Reg::read) this register and get [`i2s1_out_start_adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_out_start_adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_out_start_adr`] module*/
#[doc(alias = "I2S1_OUT_START_ADR")]
pub type I2s1OutStartAdr = crate::Reg<i2s1_out_start_adr::I2s1OutStartAdrSpec>;
///I2S0 TX DMA buffer start address (word-aligned; byte address >> 2)
pub mod i2s1_out_start_adr;
/**I2S1_OUT_SAMPLE_NO (rw) register accessor: I2S0 TX DMA transfer length (samples - 1)

You can [`read`](crate::Reg::read) this register and get [`i2s1_out_sample_no::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_out_sample_no::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_out_sample_no`] module*/
#[doc(alias = "I2S1_OUT_SAMPLE_NO")]
pub type I2s1OutSampleNo = crate::Reg<i2s1_out_sample_no::I2s1OutSampleNoSpec>;
///I2S0 TX DMA transfer length (samples - 1)
pub mod i2s1_out_sample_no;
/**I2S1_OUT_RTD_TRG (rw) register accessor: I2S0 TX DMA trigger

You can [`read`](crate::Reg::read) this register and get [`i2s1_out_rtd_trg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_out_rtd_trg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_out_rtd_trg`] module*/
#[doc(alias = "I2S1_OUT_RTD_TRG")]
pub type I2s1OutRtdTrg = crate::Reg<i2s1_out_rtd_trg::I2s1OutRtdTrgSpec>;
///I2S0 TX DMA trigger
pub mod i2s1_out_rtd_trg;
/**I2S1_OUT_BITWT (rw) register accessor: I2S0 TX DMA sample width

You can [`read`](crate::Reg::read) this register and get [`i2s1_out_bitwt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_out_bitwt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_out_bitwt`] module*/
#[doc(alias = "I2S1_OUT_BITWT")]
pub type I2s1OutBitwt = crate::Reg<i2s1_out_bitwt::I2s1OutBitwtSpec>;
///I2S0 TX DMA sample width
pub mod i2s1_out_bitwt;
/**I2S1_OUT_SD_SEL (rw) register accessor: I2S0 TX DMA output channel routing

You can [`read`](crate::Reg::read) this register and get [`i2s1_out_sd_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_out_sd_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_out_sd_sel`] module*/
#[doc(alias = "I2S1_OUT_SD_SEL")]
pub type I2s1OutSdSel = crate::Reg<i2s1_out_sd_sel::I2s1OutSdSelSpec>;
///I2S0 TX DMA output channel routing
pub mod i2s1_out_sd_sel;
/**I2S1_OUT_MON (r) register accessor: I2S0 TX DMA monitor (read-only)

You can [`read`](crate::Reg::read) this register and get [`i2s1_out_mon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_out_mon`] module*/
#[doc(alias = "I2S1_OUT_MON")]
pub type I2s1OutMon = crate::Reg<i2s1_out_mon::I2s1OutMonSpec>;
///I2S0 TX DMA monitor (read-only)
pub mod i2s1_out_mon;
/**I2S_DATARATE (rw) register accessor: I2S sample-rate class select

You can [`read`](crate::Reg::read) this register and get [`i2s_datarate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_datarate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_datarate`] module*/
#[doc(alias = "I2S_DATARATE")]
pub type I2sDatarate = crate::Reg<i2s_datarate::I2sDatarateSpec>;
///I2S sample-rate class select
pub mod i2s_datarate;
/**I2S1_INT_CTRL (rw) register accessor: I2S0 DMA interrupt status (write 1 to clear)

You can [`read`](crate::Reg::read) this register and get [`i2s1_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_int_ctrl`] module*/
#[doc(alias = "I2S1_INT_CTRL")]
pub type I2s1IntCtrl = crate::Reg<i2s1_int_ctrl::I2s1IntCtrlSpec>;
///I2S0 DMA interrupt status (write 1 to clear)
pub mod i2s1_int_ctrl;
/**I2S1_INT_MASK (rw) register accessor: I2S0 DMA interrupt mask (1=masked)

You can [`read`](crate::Reg::read) this register and get [`i2s1_int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_int_mask`] module*/
#[doc(alias = "I2S1_INT_MASK")]
pub type I2s1IntMask = crate::Reg<i2s1_int_mask::I2s1IntMaskSpec>;
///I2S0 DMA interrupt mask (1=masked)
pub mod i2s1_int_mask;
/**INT_M (rw) register accessor: Audio bus/BCK error interrupt mask (1=masked)

You can [`read`](crate::Reg::read) this register and get [`int_m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_m`] module*/
#[doc(alias = "INT_M")]
pub type IntM = crate::Reg<int_m::IntMSpec>;
///Audio bus/BCK error interrupt mask (1=masked)
pub mod int_m;
/**INT_CLR (rw) register accessor: Audio bus/BCK error interrupt clear

You can [`read`](crate::Reg::read) this register and get [`int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
#[doc(alias = "INT_CLR")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
///Audio bus/BCK error interrupt clear
pub mod int_clr;
/**AHB_I2S_CLKEN (rw) register accessor: AHB interface clock enable for the audio DMA masters

You can [`read`](crate::Reg::read) this register and get [`ahb_i2s_clken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_i2s_clken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ahb_i2s_clken`] module*/
#[doc(alias = "AHB_I2S_CLKEN")]
pub type AhbI2sClken = crate::Reg<ahb_i2s_clken::AhbI2sClkenSpec>;
///AHB interface clock enable for the audio DMA masters
pub mod ahb_i2s_clken;
