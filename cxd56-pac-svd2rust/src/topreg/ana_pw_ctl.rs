///Register `ANA_PW_CTL` reader
pub type R = crate::R<AnaPwCtlSpec>;
///Register `ANA_PW_CTL` writer
pub type W = crate::W<AnaPwCtlSpec>;
///Field `RCOSC` reader - RC oscillator
pub type RcoscR = crate::BitReader;
///Field `RCOSC` writer - RC oscillator
pub type RcoscW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XOSC` reader - Crystal oscillator
pub type XoscR = crate::BitReader;
///Field `XOSC` writer - Crystal oscillator
pub type XoscW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSPLL` reader - System PLL
pub type SyspllR = crate::BitReader;
///Field `SYSPLL` writer - System PLL
pub type SyspllW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF_LNA` reader - RF LNA
pub type RfLnaR = crate::BitReader;
///Field `RF_LNA` writer - RF LNA
pub type RfLnaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF_MIX` reader - RF mixer
pub type RfMixR = crate::BitReader;
///Field `RF_MIX` writer - RF mixer
pub type RfMixW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF_IF` reader - RF IF block
pub type RfIfR = crate::BitReader;
///Field `RF_IF` writer - RF IF block
pub type RfIfW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF_ADC` reader - RF ADC
pub type RfAdcR = crate::BitReader;
///Field `RF_ADC` writer - RF ADC
pub type RfAdcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF_LO` reader - RF local oscillator
pub type RfLoR = crate::BitReader;
///Field `RF_LO` writer - RF local oscillator
pub type RfLoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF_PLL` reader - RF PLL
pub type RfPllR = crate::BitReader;
///Field `RF_PLL` writer - RF PLL
pub type RfPllW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPADC` reader - High-precision ADC
pub type HpadcR = crate::BitReader;
///Field `HPADC` writer - High-precision ADC
pub type HpadcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPADC` reader - Low-power ADC
pub type LpadcR = crate::BitReader;
///Field `LPADC` writer - Low-power ADC
pub type LpadcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RC oscillator
    #[inline(always)]
    pub fn rcosc(&self) -> RcoscR {
        RcoscR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Crystal oscillator
    #[inline(always)]
    pub fn xosc(&self) -> XoscR {
        XoscR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - System PLL
    #[inline(always)]
    pub fn syspll(&self) -> SyspllR {
        SyspllR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RF LNA
    #[inline(always)]
    pub fn rf_lna(&self) -> RfLnaR {
        RfLnaR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RF mixer
    #[inline(always)]
    pub fn rf_mix(&self) -> RfMixR {
        RfMixR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RF IF block
    #[inline(always)]
    pub fn rf_if(&self) -> RfIfR {
        RfIfR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RF ADC
    #[inline(always)]
    pub fn rf_adc(&self) -> RfAdcR {
        RfAdcR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RF local oscillator
    #[inline(always)]
    pub fn rf_lo(&self) -> RfLoR {
        RfLoR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RF PLL
    #[inline(always)]
    pub fn rf_pll(&self) -> RfPllR {
        RfPllR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - High-precision ADC
    #[inline(always)]
    pub fn hpadc(&self) -> HpadcR {
        HpadcR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Low-power ADC
    #[inline(always)]
    pub fn lpadc(&self) -> LpadcR {
        LpadcR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RC oscillator
    #[inline(always)]
    pub fn rcosc(&mut self) -> RcoscW<'_, AnaPwCtlSpec> {
        RcoscW::new(self, 0)
    }
    ///Bit 1 - Crystal oscillator
    #[inline(always)]
    pub fn xosc(&mut self) -> XoscW<'_, AnaPwCtlSpec> {
        XoscW::new(self, 1)
    }
    ///Bit 2 - System PLL
    #[inline(always)]
    pub fn syspll(&mut self) -> SyspllW<'_, AnaPwCtlSpec> {
        SyspllW::new(self, 2)
    }
    ///Bit 4 - RF LNA
    #[inline(always)]
    pub fn rf_lna(&mut self) -> RfLnaW<'_, AnaPwCtlSpec> {
        RfLnaW::new(self, 4)
    }
    ///Bit 5 - RF mixer
    #[inline(always)]
    pub fn rf_mix(&mut self) -> RfMixW<'_, AnaPwCtlSpec> {
        RfMixW::new(self, 5)
    }
    ///Bit 6 - RF IF block
    #[inline(always)]
    pub fn rf_if(&mut self) -> RfIfW<'_, AnaPwCtlSpec> {
        RfIfW::new(self, 6)
    }
    ///Bit 7 - RF ADC
    #[inline(always)]
    pub fn rf_adc(&mut self) -> RfAdcW<'_, AnaPwCtlSpec> {
        RfAdcW::new(self, 7)
    }
    ///Bit 8 - RF local oscillator
    #[inline(always)]
    pub fn rf_lo(&mut self) -> RfLoW<'_, AnaPwCtlSpec> {
        RfLoW::new(self, 8)
    }
    ///Bit 9 - RF PLL
    #[inline(always)]
    pub fn rf_pll(&mut self) -> RfPllW<'_, AnaPwCtlSpec> {
        RfPllW::new(self, 9)
    }
    ///Bit 12 - High-precision ADC
    #[inline(always)]
    pub fn hpadc(&mut self) -> HpadcW<'_, AnaPwCtlSpec> {
        HpadcW::new(self, 12)
    }
    ///Bit 13 - Low-power ADC
    #[inline(always)]
    pub fn lpadc(&mut self) -> LpadcW<'_, AnaPwCtlSpec> {
        LpadcW::new(self, 13)
    }
}
/**Analog circuit power control (1 = powered on)

You can [`read`](crate::Reg::read) this register and get [`ana_pw_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_pw_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AnaPwCtlSpec;
impl crate::RegisterSpec for AnaPwCtlSpec {
    type Ux = u32;
}
///`read()` method returns [`ana_pw_ctl::R`](R) reader structure
impl crate::Readable for AnaPwCtlSpec {}
///`write(|w| ..)` method takes [`ana_pw_ctl::W`](W) writer structure
impl crate::Writable for AnaPwCtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ANA_PW_CTL to value 0
impl crate::Resettable for AnaPwCtlSpec {}
