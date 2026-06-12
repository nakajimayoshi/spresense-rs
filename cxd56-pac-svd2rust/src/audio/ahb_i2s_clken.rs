///Register `AHB_I2S_CLKEN` reader
pub type R = crate::R<AhbI2sClkenSpec>;
///Register `AHB_I2S_CLKEN` writer
pub type W = crate::W<AhbI2sClkenSpec>;
///Field `AHBMIC_CLKEN` reader - MIC DMA AHB clock enable (1=enable)
pub type AhbmicClkenR = crate::BitReader;
///Field `AHBMIC_CLKEN` writer - MIC DMA AHB clock enable (1=enable)
pub type AhbmicClkenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBI2S1_CLKEN` reader - I2S0 AHB clock enable (1=enable)
pub type Ahbi2s1ClkenR = crate::BitReader;
///Field `AHBI2S1_CLKEN` writer - I2S0 AHB clock enable (1=enable)
pub type Ahbi2s1ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBI2S2_CLKEN` reader - I2S1 AHB clock enable (1=enable)
pub type Ahbi2s2ClkenR = crate::BitReader;
///Field `AHBI2S2_CLKEN` writer - I2S1 AHB clock enable (1=enable)
pub type Ahbi2s2ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MIC DMA AHB clock enable (1=enable)
    #[inline(always)]
    pub fn ahbmic_clken(&self) -> AhbmicClkenR {
        AhbmicClkenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2S0 AHB clock enable (1=enable)
    #[inline(always)]
    pub fn ahbi2s1_clken(&self) -> Ahbi2s1ClkenR {
        Ahbi2s1ClkenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I2S1 AHB clock enable (1=enable)
    #[inline(always)]
    pub fn ahbi2s2_clken(&self) -> Ahbi2s2ClkenR {
        Ahbi2s2ClkenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MIC DMA AHB clock enable (1=enable)
    #[inline(always)]
    pub fn ahbmic_clken(&mut self) -> AhbmicClkenW<'_, AhbI2sClkenSpec> {
        AhbmicClkenW::new(self, 0)
    }
    ///Bit 1 - I2S0 AHB clock enable (1=enable)
    #[inline(always)]
    pub fn ahbi2s1_clken(&mut self) -> Ahbi2s1ClkenW<'_, AhbI2sClkenSpec> {
        Ahbi2s1ClkenW::new(self, 1)
    }
    ///Bit 2 - I2S1 AHB clock enable (1=enable)
    #[inline(always)]
    pub fn ahbi2s2_clken(&mut self) -> Ahbi2s2ClkenW<'_, AhbI2sClkenSpec> {
        Ahbi2s2ClkenW::new(self, 2)
    }
}
/**AHB interface clock enable for the audio DMA masters

You can [`read`](crate::Reg::read) this register and get [`ahb_i2s_clken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_i2s_clken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AhbI2sClkenSpec;
impl crate::RegisterSpec for AhbI2sClkenSpec {
    type Ux = u32;
}
///`read()` method returns [`ahb_i2s_clken::R`](R) reader structure
impl crate::Readable for AhbI2sClkenSpec {}
///`write(|w| ..)` method takes [`ahb_i2s_clken::W`](W) writer structure
impl crate::Writable for AhbI2sClkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB_I2S_CLKEN to value 0
impl crate::Resettable for AhbI2sClkenSpec {}
