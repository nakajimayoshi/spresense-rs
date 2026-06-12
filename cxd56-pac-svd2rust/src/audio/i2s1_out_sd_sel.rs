///Register `I2S1_OUT_SD_SEL` reader
pub type R = crate::R<I2s1OutSdSelSpec>;
///Register `I2S1_OUT_SD_SEL` writer
pub type W = crate::W<I2s1OutSdSelSpec>;
///Field `SD1_R_SEL` reader - SD1 right source: 0=DMA ch L, 1=DMA ch R, 2=unused
pub type Sd1RSelR = crate::FieldReader;
///Field `SD1_R_SEL` writer - SD1 right source: 0=DMA ch L, 1=DMA ch R, 2=unused
pub type Sd1RSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SD1_L_SEL` reader - SD1 left source: 0=DMA ch L, 1=DMA ch R, 2=unused
pub type Sd1LSelR = crate::FieldReader;
///Field `SD1_L_SEL` writer - SD1 left source: 0=DMA ch L, 1=DMA ch R, 2=unused
pub type Sd1LSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - SD1 right source: 0=DMA ch L, 1=DMA ch R, 2=unused
    #[inline(always)]
    pub fn sd1_r_sel(&self) -> Sd1RSelR {
        Sd1RSelR::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - SD1 left source: 0=DMA ch L, 1=DMA ch R, 2=unused
    #[inline(always)]
    pub fn sd1_l_sel(&self) -> Sd1LSelR {
        Sd1LSelR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - SD1 right source: 0=DMA ch L, 1=DMA ch R, 2=unused
    #[inline(always)]
    pub fn sd1_r_sel(&mut self) -> Sd1RSelW<'_, I2s1OutSdSelSpec> {
        Sd1RSelW::new(self, 0)
    }
    ///Bits 4:5 - SD1 left source: 0=DMA ch L, 1=DMA ch R, 2=unused
    #[inline(always)]
    pub fn sd1_l_sel(&mut self) -> Sd1LSelW<'_, I2s1OutSdSelSpec> {
        Sd1LSelW::new(self, 4)
    }
}
/**I2S0 TX DMA output channel routing

You can [`read`](crate::Reg::read) this register and get [`i2s1_out_sd_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_out_sd_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1OutSdSelSpec;
impl crate::RegisterSpec for I2s1OutSdSelSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_out_sd_sel::R`](R) reader structure
impl crate::Readable for I2s1OutSdSelSpec {}
///`write(|w| ..)` method takes [`i2s1_out_sd_sel::W`](W) writer structure
impl crate::Writable for I2s1OutSdSelSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S1_OUT_SD_SEL to value 0
impl crate::Resettable for I2s1OutSdSelSpec {}
