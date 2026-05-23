///Register `ANA_PW_STAT` reader
pub type R = crate::R<AnaPwStatSpec>;
///Field `RCOSC` reader - RC oscillator powered on
pub type RcoscR = crate::BitReader;
///Field `XOSC` reader - Crystal oscillator powered on
pub type XoscR = crate::BitReader;
///Field `SYSPLL` reader - System PLL powered on
pub type SyspllR = crate::BitReader;
///Field `RF_LNA` reader - RF LNA powered on
pub type RfLnaR = crate::BitReader;
///Field `RF_MIX` reader - RF mixer powered on
pub type RfMixR = crate::BitReader;
///Field `RF_IF` reader - RF IF block powered on
pub type RfIfR = crate::BitReader;
///Field `RF_ADC` reader - RF ADC powered on
pub type RfAdcR = crate::BitReader;
///Field `RF_LO` reader - RF LO powered on
pub type RfLoR = crate::BitReader;
///Field `RF_PLL` reader - RF PLL powered on
pub type RfPllR = crate::BitReader;
///Field `HPADC` reader - High-precision ADC powered on
pub type HpadcR = crate::BitReader;
///Field `LPADC` reader - Low-power ADC powered on
pub type LpadcR = crate::BitReader;
impl R {
    ///Bit 0 - RC oscillator powered on
    #[inline(always)]
    pub fn rcosc(&self) -> RcoscR {
        RcoscR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Crystal oscillator powered on
    #[inline(always)]
    pub fn xosc(&self) -> XoscR {
        XoscR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - System PLL powered on
    #[inline(always)]
    pub fn syspll(&self) -> SyspllR {
        SyspllR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RF LNA powered on
    #[inline(always)]
    pub fn rf_lna(&self) -> RfLnaR {
        RfLnaR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RF mixer powered on
    #[inline(always)]
    pub fn rf_mix(&self) -> RfMixR {
        RfMixR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RF IF block powered on
    #[inline(always)]
    pub fn rf_if(&self) -> RfIfR {
        RfIfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RF ADC powered on
    #[inline(always)]
    pub fn rf_adc(&self) -> RfAdcR {
        RfAdcR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RF LO powered on
    #[inline(always)]
    pub fn rf_lo(&self) -> RfLoR {
        RfLoR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RF PLL powered on
    #[inline(always)]
    pub fn rf_pll(&self) -> RfPllR {
        RfPllR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - High-precision ADC powered on
    #[inline(always)]
    pub fn hpadc(&self) -> HpadcR {
        HpadcR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Low-power ADC powered on
    #[inline(always)]
    pub fn lpadc(&self) -> LpadcR {
        LpadcR::new(((self.bits >> 13) & 1) != 0)
    }
}
/**Analog power status (read-only mirror of ANA_PW_CTL)

You can [`read`](crate::Reg::read) this register and get [`ana_pw_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AnaPwStatSpec;
impl crate::RegisterSpec for AnaPwStatSpec {
    type Ux = u32;
}
///`read()` method returns [`ana_pw_stat::R`](R) reader structure
impl crate::Readable for AnaPwStatSpec {}
///`reset()` method sets ANA_PW_STAT to value 0
impl crate::Resettable for AnaPwStatSpec {}
