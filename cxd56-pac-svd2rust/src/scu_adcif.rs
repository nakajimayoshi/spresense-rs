#[repr(C)]
///Register block
pub struct RegisterBlock {
    lpadc_fifo: [LpadcFifo; 4],
    _reserved1: [u8; 0x70],
    hpadc_fifo: [HpadcFifo; 2],
    _reserved2: [u8; 0x0178],
    lpadc_a0: LpadcA0,
    lpadc_a1: LpadcA1,
    _reserved4: [u8; 0x08],
    lpadc_d0: LpadcD0,
    lpadc_d1: LpadcD1,
    lpadc_d2: LpadcD2,
    lpadc_d4: LpadcD4,
    lpadc_d5: LpadcD5,
    lpadc_d6: LpadcD6,
    _reserved10: [u8; 0x18],
    hpadc_ac0: HpadcAc0,
    hpadc_ac1: HpadcAc1,
    _reserved12: [u8; 0x08],
    hpadc_dc: HpadcDc,
    _reserved13: [u8; 0x2c],
    hpadc0_a0: Hpadc0A0,
    hpadc0_a1: Hpadc0A1,
    hpadc0_a2: Hpadc0A2,
    hpadc0_a3: Hpadc0A3,
    hpadc0_d0: Hpadc0D0,
    hpadc0_d1: Hpadc0D1,
    hpadc0_d2: Hpadc0D2,
    _reserved20: [u8; 0x24],
    hpadc1_a0: Hpadc1A0,
    hpadc1_a1: Hpadc1A1,
    hpadc1_a2: Hpadc1A2,
    hpadc1_a3: Hpadc1A3,
    hpadc1_d0: Hpadc1D0,
    hpadc1_d1: Hpadc1D1,
    hpadc1_d2: Hpadc1D2,
    _reserved27: [u8; 0x24],
    lpadc_at0: LpadcAt0,
    lpadc_at1: LpadcAt1,
    _reserved29: [u8; 0xc8],
    adcif_dct: AdcifDct,
    scu_adcif_ckpower: ScuAdcifCkpower,
}
impl RegisterBlock {
    ///0x00..0x10 - LPADC ch%s FIFO Read Port — dequeue one 10-bit sample (MSB-aligned in 16-bit word; DATA = bits\[15:6\] as 0..=1023)
    #[inline(always)]
    pub const fn lpadc_fifo(&self, n: usize) -> &LpadcFifo {
        &self.lpadc_fifo[n]
    }
    ///Iterator for array of:
    ///0x00..0x10 - LPADC ch%s FIFO Read Port — dequeue one 10-bit sample (MSB-aligned in 16-bit word; DATA = bits\[15:6\] as 0..=1023)
    #[inline(always)]
    pub fn lpadc_fifo_iter(&self) -> impl Iterator<Item = &LpadcFifo> {
        self.lpadc_fifo.iter()
    }
    ///0x80..0x88 - HPADC ch%s FIFO Read Port (SEN_AIN0/1, Arduino A4/A5)
    #[inline(always)]
    pub const fn hpadc_fifo(&self, n: usize) -> &HpadcFifo {
        &self.hpadc_fifo[n]
    }
    ///Iterator for array of:
    ///0x80..0x88 - HPADC ch%s FIFO Read Port (SEN_AIN0/1, Arduino A4/A5)
    #[inline(always)]
    pub fn hpadc_fifo_iter(&self) -> impl Iterator<Item = &HpadcFifo> {
        self.hpadc_fifo.iter()
    }
    ///0x200 - LPADC Enable Control — LV_ADC_EN (UM Table ADC-106)
    #[inline(always)]
    pub const fn lpadc_a0(&self) -> &LpadcA0 {
        &self.lpadc_a0
    }
    ///0x204 - LPADC Channel Switching Control (UM Table ADC-107)
    #[inline(always)]
    pub const fn lpadc_a1(&self) -> &LpadcA1 {
        &self.lpadc_a1
    }
    ///0x210 - LPADC Software Reset, common to all channels (UM Table ADC-108)
    #[inline(always)]
    pub const fn lpadc_d0(&self) -> &LpadcD0 {
        &self.lpadc_d0
    }
    ///0x214 - LPADC ch0 sample-rate divider and FIFO watermark (UM Table ADC-109)
    #[inline(always)]
    pub const fn lpadc_d1(&self) -> &LpadcD1 {
        &self.lpadc_d1
    }
    ///0x218 - LPADC per-channel FIFO enable (UM Table ADC-110). Bit n=1 enables ADCIF hardware FIFO for channel n.
    #[inline(always)]
    pub const fn lpadc_d2(&self) -> &LpadcD2 {
        &self.lpadc_d2
    }
    ///0x21c - LPADC ch1 sample-rate divider and FIFO watermark (UM Table ADC-111)
    #[inline(always)]
    pub const fn lpadc_d4(&self) -> &LpadcD4 {
        &self.lpadc_d4
    }
    ///0x220 - LPADC ch2 (SEN_AIN4 / Arduino A2) sample-rate and watermark
    #[inline(always)]
    pub const fn lpadc_d5(&self) -> &LpadcD5 {
        &self.lpadc_d5
    }
    ///0x224 - LPADC ch3 (SEN_AIN5 / Arduino A3) sample-rate and watermark
    #[inline(always)]
    pub const fn lpadc_d6(&self) -> &LpadcD6 {
        &self.lpadc_d6
    }
    ///0x240 - HPADC common clock control (placeholder)
    #[inline(always)]
    pub const fn hpadc_ac0(&self) -> &HpadcAc0 {
        &self.hpadc_ac0
    }
    ///0x244 - HPADC common enable control (placeholder)
    #[inline(always)]
    pub const fn hpadc_ac1(&self) -> &HpadcAc1 {
        &self.hpadc_ac1
    }
    ///0x250 - HPADC two-element vector mode control (placeholder)
    #[inline(always)]
    pub const fn hpadc_dc(&self) -> &HpadcDc {
        &self.hpadc_dc
    }
    ///0x280 - HPADC0 clock selection (placeholder)
    #[inline(always)]
    pub const fn hpadc0_a0(&self) -> &Hpadc0A0 {
        &self.hpadc0_a0
    }
    ///0x284 - HPADC0 enable control (placeholder)
    #[inline(always)]
    pub const fn hpadc0_a1(&self) -> &Hpadc0A1 {
        &self.hpadc0_a1
    }
    ///0x288 - HPADC0 clock enable (placeholder)
    #[inline(always)]
    pub const fn hpadc0_a2(&self) -> &Hpadc0A2 {
        &self.hpadc0_a2
    }
    ///0x28c - HPADC0 LPF control (placeholder; reset=0x100)
    #[inline(always)]
    pub const fn hpadc0_a3(&self) -> &Hpadc0A3 {
        &self.hpadc0_a3
    }
    ///0x290 - HPADC0 software reset (placeholder)
    #[inline(always)]
    pub const fn hpadc0_d0(&self) -> &Hpadc0D0 {
        &self.hpadc0_d0
    }
    ///0x294 - HPADC0 basic setting (placeholder; reset=0x10)
    #[inline(always)]
    pub const fn hpadc0_d1(&self) -> &Hpadc0D1 {
        &self.hpadc0_d1
    }
    ///0x298 - HPADC0 ADC data acceptance (placeholder)
    #[inline(always)]
    pub const fn hpadc0_d2(&self) -> &Hpadc0D2 {
        &self.hpadc0_d2
    }
    ///0x2c0 - HPADC1 clock selection (placeholder)
    #[inline(always)]
    pub const fn hpadc1_a0(&self) -> &Hpadc1A0 {
        &self.hpadc1_a0
    }
    ///0x2c4 - HPADC1 enable control (placeholder)
    #[inline(always)]
    pub const fn hpadc1_a1(&self) -> &Hpadc1A1 {
        &self.hpadc1_a1
    }
    ///0x2c8 - HPADC1 clock enable (placeholder)
    #[inline(always)]
    pub const fn hpadc1_a2(&self) -> &Hpadc1A2 {
        &self.hpadc1_a2
    }
    ///0x2cc - HPADC1 LPF control (placeholder; reset=0x100)
    #[inline(always)]
    pub const fn hpadc1_a3(&self) -> &Hpadc1A3 {
        &self.hpadc1_a3
    }
    ///0x2d0 - HPADC1 software reset (placeholder)
    #[inline(always)]
    pub const fn hpadc1_d0(&self) -> &Hpadc1D0 {
        &self.hpadc1_d0
    }
    ///0x2d4 - HPADC1 basic setting (placeholder; reset=0x10)
    #[inline(always)]
    pub const fn hpadc1_d1(&self) -> &Hpadc1D1 {
        &self.hpadc1_d1
    }
    ///0x2d8 - HPADC1 ADC data acceptance (placeholder)
    #[inline(always)]
    pub const fn hpadc1_d2(&self) -> &Hpadc1D2 {
        &self.hpadc1_d2
    }
    ///0x300 - LPADC analog timing register 0 (bit0 must be cleared, cxd56_adc.c:469)
    #[inline(always)]
    pub const fn lpadc_at0(&self) -> &LpadcAt0 {
        &self.lpadc_at0
    }
    ///0x304 - LPADC analog warm-up timing (SDK sets bits\[7:0\]=0x04, cxd56_adc.c:462)
    #[inline(always)]
    pub const fn lpadc_at1(&self) -> &LpadcAt1 {
        &self.lpadc_at1
    }
    ///0x3d0 - ADCIF register-map revision code (RO; reads 0xADC1F000)
    #[inline(always)]
    pub const fn adcif_dct(&self) -> &AdcifDct {
        &self.adcif_dct
    }
    ///0x3d4 - ADCIF clock and power control (reset=0x1 = clocked; write 0 to gate)
    #[inline(always)]
    pub const fn scu_adcif_ckpower(&self) -> &ScuAdcifCkpower {
        &self.scu_adcif_ckpower
    }
}
/**LPADC_FIFO (r) register accessor: LPADC ch%s FIFO Read Port — dequeue one 10-bit sample (MSB-aligned in 16-bit word; DATA = bits\[15:6\] as 0..=1023)

You can [`read`](crate::Reg::read) this register and get [`lpadc_fifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_fifo`] module*/
#[doc(alias = "LPADC_FIFO")]
pub type LpadcFifo = crate::Reg<lpadc_fifo::LpadcFifoSpec>;
///LPADC ch%s FIFO Read Port — dequeue one 10-bit sample (MSB-aligned in 16-bit word; DATA = bits\[15:6\] as 0..=1023)
pub mod lpadc_fifo;
/**HPADC_FIFO (r) register accessor: HPADC ch%s FIFO Read Port (SEN_AIN0/1, Arduino A4/A5)

You can [`read`](crate::Reg::read) this register and get [`hpadc_fifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc_fifo`] module*/
#[doc(alias = "HPADC_FIFO")]
pub type HpadcFifo = crate::Reg<hpadc_fifo::HpadcFifoSpec>;
///HPADC ch%s FIFO Read Port (SEN_AIN0/1, Arduino A4/A5)
pub mod hpadc_fifo;
/**LPADC_A0 (rw) register accessor: LPADC Enable Control — LV_ADC_EN (UM Table ADC-106)

You can [`read`](crate::Reg::read) this register and get [`lpadc_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_a0`] module*/
#[doc(alias = "LPADC_A0")]
pub type LpadcA0 = crate::Reg<lpadc_a0::LpadcA0Spec>;
///LPADC Enable Control — LV_ADC_EN (UM Table ADC-106)
pub mod lpadc_a0;
/**LPADC_A1 (rw) register accessor: LPADC Channel Switching Control (UM Table ADC-107)

You can [`read`](crate::Reg::read) this register and get [`lpadc_a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_a1`] module*/
#[doc(alias = "LPADC_A1")]
pub type LpadcA1 = crate::Reg<lpadc_a1::LpadcA1Spec>;
///LPADC Channel Switching Control (UM Table ADC-107)
pub mod lpadc_a1;
/**LPADC_D0 (rw) register accessor: LPADC Software Reset, common to all channels (UM Table ADC-108)

You can [`read`](crate::Reg::read) this register and get [`lpadc_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_d0`] module*/
#[doc(alias = "LPADC_D0")]
pub type LpadcD0 = crate::Reg<lpadc_d0::LpadcD0Spec>;
///LPADC Software Reset, common to all channels (UM Table ADC-108)
pub mod lpadc_d0;
/**LPADC_D1 (rw) register accessor: LPADC ch0 sample-rate divider and FIFO watermark (UM Table ADC-109)

You can [`read`](crate::Reg::read) this register and get [`lpadc_d1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_d1`] module*/
#[doc(alias = "LPADC_D1")]
pub type LpadcD1 = crate::Reg<lpadc_d1::LpadcD1Spec>;
///LPADC ch0 sample-rate divider and FIFO watermark (UM Table ADC-109)
pub mod lpadc_d1;
/**LPADC_D2 (rw) register accessor: LPADC per-channel FIFO enable (UM Table ADC-110). Bit n=1 enables ADCIF hardware FIFO for channel n.

You can [`read`](crate::Reg::read) this register and get [`lpadc_d2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_d2`] module*/
#[doc(alias = "LPADC_D2")]
pub type LpadcD2 = crate::Reg<lpadc_d2::LpadcD2Spec>;
///LPADC per-channel FIFO enable (UM Table ADC-110). Bit n=1 enables ADCIF hardware FIFO for channel n.
pub mod lpadc_d2;
/**LPADC_D4 (rw) register accessor: LPADC ch1 sample-rate divider and FIFO watermark (UM Table ADC-111)

You can [`read`](crate::Reg::read) this register and get [`lpadc_d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_d4`] module*/
#[doc(alias = "LPADC_D4")]
pub type LpadcD4 = crate::Reg<lpadc_d4::LpadcD4Spec>;
///LPADC ch1 sample-rate divider and FIFO watermark (UM Table ADC-111)
pub mod lpadc_d4;
/**LPADC_D5 (rw) register accessor: LPADC ch2 (SEN_AIN4 / Arduino A2) sample-rate and watermark

You can [`read`](crate::Reg::read) this register and get [`lpadc_d5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_d5`] module*/
#[doc(alias = "LPADC_D5")]
pub type LpadcD5 = crate::Reg<lpadc_d5::LpadcD5Spec>;
///LPADC ch2 (SEN_AIN4 / Arduino A2) sample-rate and watermark
pub mod lpadc_d5;
/**LPADC_D6 (rw) register accessor: LPADC ch3 (SEN_AIN5 / Arduino A3) sample-rate and watermark

You can [`read`](crate::Reg::read) this register and get [`lpadc_d6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_d6`] module*/
#[doc(alias = "LPADC_D6")]
pub type LpadcD6 = crate::Reg<lpadc_d6::LpadcD6Spec>;
///LPADC ch3 (SEN_AIN5 / Arduino A3) sample-rate and watermark
pub mod lpadc_d6;
/**LPADC_AT0 (rw) register accessor: LPADC analog timing register 0 (bit0 must be cleared, cxd56_adc.c:469)

You can [`read`](crate::Reg::read) this register and get [`lpadc_at0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_at0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_at0`] module*/
#[doc(alias = "LPADC_AT0")]
pub type LpadcAt0 = crate::Reg<lpadc_at0::LpadcAt0Spec>;
///LPADC analog timing register 0 (bit0 must be cleared, cxd56_adc.c:469)
pub mod lpadc_at0;
/**LPADC_AT1 (rw) register accessor: LPADC analog warm-up timing (SDK sets bits\[7:0\]=0x04, cxd56_adc.c:462)

You can [`read`](crate::Reg::read) this register and get [`lpadc_at1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_at1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpadc_at1`] module*/
#[doc(alias = "LPADC_AT1")]
pub type LpadcAt1 = crate::Reg<lpadc_at1::LpadcAt1Spec>;
///LPADC analog warm-up timing (SDK sets bits\[7:0\]=0x04, cxd56_adc.c:462)
pub mod lpadc_at1;
/**HPADC_AC0 (rw) register accessor: HPADC common clock control (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc_ac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc_ac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc_ac0`] module*/
#[doc(alias = "HPADC_AC0")]
pub type HpadcAc0 = crate::Reg<hpadc_ac0::HpadcAc0Spec>;
///HPADC common clock control (placeholder)
pub mod hpadc_ac0;
/**HPADC_AC1 (rw) register accessor: HPADC common enable control (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc_ac1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc_ac1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc_ac1`] module*/
#[doc(alias = "HPADC_AC1")]
pub type HpadcAc1 = crate::Reg<hpadc_ac1::HpadcAc1Spec>;
///HPADC common enable control (placeholder)
pub mod hpadc_ac1;
/**HPADC_DC (rw) register accessor: HPADC two-element vector mode control (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc_dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc_dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc_dc`] module*/
#[doc(alias = "HPADC_DC")]
pub type HpadcDc = crate::Reg<hpadc_dc::HpadcDcSpec>;
///HPADC two-element vector mode control (placeholder)
pub mod hpadc_dc;
/**HPADC0_A0 (rw) register accessor: HPADC0 clock selection (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc0_a0`] module*/
#[doc(alias = "HPADC0_A0")]
pub type Hpadc0A0 = crate::Reg<hpadc0_a0::Hpadc0A0Spec>;
///HPADC0 clock selection (placeholder)
pub mod hpadc0_a0;
/**HPADC0_A1 (rw) register accessor: HPADC0 enable control (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc0_a1`] module*/
#[doc(alias = "HPADC0_A1")]
pub type Hpadc0A1 = crate::Reg<hpadc0_a1::Hpadc0A1Spec>;
///HPADC0 enable control (placeholder)
pub mod hpadc0_a1;
/**HPADC0_A2 (rw) register accessor: HPADC0 clock enable (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_a2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_a2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc0_a2`] module*/
#[doc(alias = "HPADC0_A2")]
pub type Hpadc0A2 = crate::Reg<hpadc0_a2::Hpadc0A2Spec>;
///HPADC0 clock enable (placeholder)
pub mod hpadc0_a2;
/**HPADC0_A3 (rw) register accessor: HPADC0 LPF control (placeholder; reset=0x100)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_a3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_a3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc0_a3`] module*/
#[doc(alias = "HPADC0_A3")]
pub type Hpadc0A3 = crate::Reg<hpadc0_a3::Hpadc0A3Spec>;
///HPADC0 LPF control (placeholder; reset=0x100)
pub mod hpadc0_a3;
/**HPADC0_D0 (rw) register accessor: HPADC0 software reset (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc0_d0`] module*/
#[doc(alias = "HPADC0_D0")]
pub type Hpadc0D0 = crate::Reg<hpadc0_d0::Hpadc0D0Spec>;
///HPADC0 software reset (placeholder)
pub mod hpadc0_d0;
/**HPADC0_D1 (rw) register accessor: HPADC0 basic setting (placeholder; reset=0x10)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_d1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_d1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc0_d1`] module*/
#[doc(alias = "HPADC0_D1")]
pub type Hpadc0D1 = crate::Reg<hpadc0_d1::Hpadc0D1Spec>;
///HPADC0 basic setting (placeholder; reset=0x10)
pub mod hpadc0_d1;
/**HPADC0_D2 (rw) register accessor: HPADC0 ADC data acceptance (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_d2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_d2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc0_d2`] module*/
#[doc(alias = "HPADC0_D2")]
pub type Hpadc0D2 = crate::Reg<hpadc0_d2::Hpadc0D2Spec>;
///HPADC0 ADC data acceptance (placeholder)
pub mod hpadc0_d2;
/**HPADC1_A0 (rw) register accessor: HPADC1 clock selection (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc1_a0`] module*/
#[doc(alias = "HPADC1_A0")]
pub type Hpadc1A0 = crate::Reg<hpadc1_a0::Hpadc1A0Spec>;
///HPADC1 clock selection (placeholder)
pub mod hpadc1_a0;
/**HPADC1_A1 (rw) register accessor: HPADC1 enable control (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc1_a1`] module*/
#[doc(alias = "HPADC1_A1")]
pub type Hpadc1A1 = crate::Reg<hpadc1_a1::Hpadc1A1Spec>;
///HPADC1 enable control (placeholder)
pub mod hpadc1_a1;
/**HPADC1_A2 (rw) register accessor: HPADC1 clock enable (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_a2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_a2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc1_a2`] module*/
#[doc(alias = "HPADC1_A2")]
pub type Hpadc1A2 = crate::Reg<hpadc1_a2::Hpadc1A2Spec>;
///HPADC1 clock enable (placeholder)
pub mod hpadc1_a2;
/**HPADC1_A3 (rw) register accessor: HPADC1 LPF control (placeholder; reset=0x100)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_a3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_a3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc1_a3`] module*/
#[doc(alias = "HPADC1_A3")]
pub type Hpadc1A3 = crate::Reg<hpadc1_a3::Hpadc1A3Spec>;
///HPADC1 LPF control (placeholder; reset=0x100)
pub mod hpadc1_a3;
/**HPADC1_D0 (rw) register accessor: HPADC1 software reset (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc1_d0`] module*/
#[doc(alias = "HPADC1_D0")]
pub type Hpadc1D0 = crate::Reg<hpadc1_d0::Hpadc1D0Spec>;
///HPADC1 software reset (placeholder)
pub mod hpadc1_d0;
/**HPADC1_D1 (rw) register accessor: HPADC1 basic setting (placeholder; reset=0x10)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_d1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_d1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc1_d1`] module*/
#[doc(alias = "HPADC1_D1")]
pub type Hpadc1D1 = crate::Reg<hpadc1_d1::Hpadc1D1Spec>;
///HPADC1 basic setting (placeholder; reset=0x10)
pub mod hpadc1_d1;
/**HPADC1_D2 (rw) register accessor: HPADC1 ADC data acceptance (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_d2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_d2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpadc1_d2`] module*/
#[doc(alias = "HPADC1_D2")]
pub type Hpadc1D2 = crate::Reg<hpadc1_d2::Hpadc1D2Spec>;
///HPADC1 ADC data acceptance (placeholder)
pub mod hpadc1_d2;
/**ADCIF_DCT (r) register accessor: ADCIF register-map revision code (RO; reads 0xADC1F000)

You can [`read`](crate::Reg::read) this register and get [`adcif_dct::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcif_dct`] module*/
#[doc(alias = "ADCIF_DCT")]
pub type AdcifDct = crate::Reg<adcif_dct::AdcifDctSpec>;
///ADCIF register-map revision code (RO; reads 0xADC1F000)
pub mod adcif_dct;
/**SCU_ADCIF_CKPOWER (rw) register accessor: ADCIF clock and power control (reset=0x1 = clocked; write 0 to gate)

You can [`read`](crate::Reg::read) this register and get [`scu_adcif_ckpower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu_adcif_ckpower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scu_adcif_ckpower`] module*/
#[doc(alias = "SCU_ADCIF_CKPOWER")]
pub type ScuAdcifCkpower = crate::Reg<scu_adcif_ckpower::ScuAdcifCkpowerSpec>;
///ADCIF clock and power control (reset=0x1 = clocked; write 0 to gate)
pub mod scu_adcif_ckpower;
