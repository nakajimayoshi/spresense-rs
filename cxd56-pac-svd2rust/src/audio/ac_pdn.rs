///Register `AC_PDN` reader
pub type R = crate::R<AcPdnSpec>;
///Register `AC_PDN` writer
pub type W = crate::W<AcPdnSpec>;
///Field `PDN_AMICEXT` reader - Analog mic ext power-down
pub type PdnAmicextR = crate::BitReader;
///Field `PDN_AMICEXT` writer - Analog mic ext power-down
pub type PdnAmicextW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_AMIC1` reader - Analog mic 1 power-down
pub type PdnAmic1R = crate::BitReader;
///Field `PDN_AMIC1` writer - Analog mic 1 power-down
pub type PdnAmic1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_AMIC2` reader - Analog mic 2 power-down
pub type PdnAmic2R = crate::BitReader;
///Field `PDN_AMIC2` writer - Analog mic 2 power-down
pub type PdnAmic2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_DAC` reader - DAC power-down
pub type PdnDacR = crate::BitReader;
///Field `PDN_DAC` writer - DAC power-down
pub type PdnDacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_LINEIN` reader - Line-in power-down
pub type PdnLineinR = crate::BitReader;
///Field `PDN_LINEIN` writer - Line-in power-down
pub type PdnLineinW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_MIC` reader - Mic block power-down
pub type PdnMicR = crate::BitReader;
///Field `PDN_MIC` writer - Mic block power-down
pub type PdnMicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_DMIC` reader - Digital mic power-down
pub type PdnDmicR = crate::BitReader;
///Field `PDN_DMIC` writer - Digital mic power-down
pub type PdnDmicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_DSPB` reader - Output filter (DSP B) power-down
pub type PdnDspbR = crate::BitReader;
///Field `PDN_DSPB` writer - Output filter (DSP B) power-down
pub type PdnDspbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_ANC` reader - ANC power-down
pub type PdnAncR = crate::BitReader;
///Field `PDN_ANC` writer - ANC power-down
pub type PdnAncW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_DNC1` reader - DNC1 power-down
pub type PdnDnc1R = crate::BitReader;
///Field `PDN_DNC1` writer - DNC1 power-down
pub type PdnDnc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_DNC2` reader - DNC2 power-down
pub type PdnDnc2R = crate::BitReader;
///Field `PDN_DNC2` writer - DNC2 power-down
pub type PdnDnc2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_SMSTR` reader - S-Master power-down
pub type PdnSmstrR = crate::BitReader;
///Field `PDN_SMSTR` writer - S-Master power-down
pub type PdnSmstrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_DSPS2` reader - SRC2 (I2S1 pin port) power-down
pub type PdnDsps2R = crate::BitReader;
///Field `PDN_DSPS2` writer - SRC2 (I2S1 pin port) power-down
pub type PdnDsps2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_DSPS1` reader - SRC1 (I2S0 pin port) power-down
pub type PdnDsps1R = crate::BitReader;
///Field `PDN_DSPS1` writer - SRC1 (I2S0 pin port) power-down
pub type PdnDsps1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDN_DSPC` reader - Codec DSP power-down
pub type PdnDspcR = crate::BitReader;
///Field `PDN_DSPC` writer - Codec DSP power-down
pub type PdnDspcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - Analog mic ext power-down
    #[inline(always)]
    pub fn pdn_amicext(&self) -> PdnAmicextR {
        PdnAmicextR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog mic 1 power-down
    #[inline(always)]
    pub fn pdn_amic1(&self) -> PdnAmic1R {
        PdnAmic1R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog mic 2 power-down
    #[inline(always)]
    pub fn pdn_amic2(&self) -> PdnAmic2R {
        PdnAmic2R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - DAC power-down
    #[inline(always)]
    pub fn pdn_dac(&self) -> PdnDacR {
        PdnDacR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Line-in power-down
    #[inline(always)]
    pub fn pdn_linein(&self) -> PdnLineinR {
        PdnLineinR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Mic block power-down
    #[inline(always)]
    pub fn pdn_mic(&self) -> PdnMicR {
        PdnMicR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Digital mic power-down
    #[inline(always)]
    pub fn pdn_dmic(&self) -> PdnDmicR {
        PdnDmicR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output filter (DSP B) power-down
    #[inline(always)]
    pub fn pdn_dspb(&self) -> PdnDspbR {
        PdnDspbR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ANC power-down
    #[inline(always)]
    pub fn pdn_anc(&self) -> PdnAncR {
        PdnAncR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DNC1 power-down
    #[inline(always)]
    pub fn pdn_dnc1(&self) -> PdnDnc1R {
        PdnDnc1R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DNC2 power-down
    #[inline(always)]
    pub fn pdn_dnc2(&self) -> PdnDnc2R {
        PdnDnc2R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - S-Master power-down
    #[inline(always)]
    pub fn pdn_smstr(&self) -> PdnSmstrR {
        PdnSmstrR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SRC2 (I2S1 pin port) power-down
    #[inline(always)]
    pub fn pdn_dsps2(&self) -> PdnDsps2R {
        PdnDsps2R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SRC1 (I2S0 pin port) power-down
    #[inline(always)]
    pub fn pdn_dsps1(&self) -> PdnDsps1R {
        PdnDsps1R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Codec DSP power-down
    #[inline(always)]
    pub fn pdn_dspc(&self) -> PdnDspcR {
        PdnDspcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - Analog mic ext power-down
    #[inline(always)]
    pub fn pdn_amicext(&mut self) -> PdnAmicextW<'_, AcPdnSpec> {
        PdnAmicextW::new(self, 4)
    }
    ///Bit 5 - Analog mic 1 power-down
    #[inline(always)]
    pub fn pdn_amic1(&mut self) -> PdnAmic1W<'_, AcPdnSpec> {
        PdnAmic1W::new(self, 5)
    }
    ///Bit 6 - Analog mic 2 power-down
    #[inline(always)]
    pub fn pdn_amic2(&mut self) -> PdnAmic2W<'_, AcPdnSpec> {
        PdnAmic2W::new(self, 6)
    }
    ///Bit 8 - DAC power-down
    #[inline(always)]
    pub fn pdn_dac(&mut self) -> PdnDacW<'_, AcPdnSpec> {
        PdnDacW::new(self, 8)
    }
    ///Bit 9 - Line-in power-down
    #[inline(always)]
    pub fn pdn_linein(&mut self) -> PdnLineinW<'_, AcPdnSpec> {
        PdnLineinW::new(self, 9)
    }
    ///Bit 10 - Mic block power-down
    #[inline(always)]
    pub fn pdn_mic(&mut self) -> PdnMicW<'_, AcPdnSpec> {
        PdnMicW::new(self, 10)
    }
    ///Bit 15 - Digital mic power-down
    #[inline(always)]
    pub fn pdn_dmic(&mut self) -> PdnDmicW<'_, AcPdnSpec> {
        PdnDmicW::new(self, 15)
    }
    ///Bit 16 - Output filter (DSP B) power-down
    #[inline(always)]
    pub fn pdn_dspb(&mut self) -> PdnDspbW<'_, AcPdnSpec> {
        PdnDspbW::new(self, 16)
    }
    ///Bit 17 - ANC power-down
    #[inline(always)]
    pub fn pdn_anc(&mut self) -> PdnAncW<'_, AcPdnSpec> {
        PdnAncW::new(self, 17)
    }
    ///Bit 18 - DNC1 power-down
    #[inline(always)]
    pub fn pdn_dnc1(&mut self) -> PdnDnc1W<'_, AcPdnSpec> {
        PdnDnc1W::new(self, 18)
    }
    ///Bit 19 - DNC2 power-down
    #[inline(always)]
    pub fn pdn_dnc2(&mut self) -> PdnDnc2W<'_, AcPdnSpec> {
        PdnDnc2W::new(self, 19)
    }
    ///Bit 20 - S-Master power-down
    #[inline(always)]
    pub fn pdn_smstr(&mut self) -> PdnSmstrW<'_, AcPdnSpec> {
        PdnSmstrW::new(self, 20)
    }
    ///Bit 21 - SRC2 (I2S1 pin port) power-down
    #[inline(always)]
    pub fn pdn_dsps2(&mut self) -> PdnDsps2W<'_, AcPdnSpec> {
        PdnDsps2W::new(self, 21)
    }
    ///Bit 22 - SRC1 (I2S0 pin port) power-down
    #[inline(always)]
    pub fn pdn_dsps1(&mut self) -> PdnDsps1W<'_, AcPdnSpec> {
        PdnDsps1W::new(self, 22)
    }
    ///Bit 23 - Codec DSP power-down
    #[inline(always)]
    pub fn pdn_dspc(&mut self) -> PdnDspcW<'_, AcPdnSpec> {
        PdnDspcW::new(self, 23)
    }
}
/**Audio block power-down control (1 = powered down)

You can [`read`](crate::Reg::read) this register and get [`ac_pdn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ac_pdn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcPdnSpec;
impl crate::RegisterSpec for AcPdnSpec {
    type Ux = u32;
}
///`read()` method returns [`ac_pdn::R`](R) reader structure
impl crate::Readable for AcPdnSpec {}
///`write(|w| ..)` method takes [`ac_pdn::W`](W) writer structure
impl crate::Writable for AcPdnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AC_PDN to value 0x00ff_8770
impl crate::Resettable for AcPdnSpec {
    const RESET_VALUE: u32 = 0x00ff_8770;
}
