///Register `I2S1_IN_CH_SEL` reader
pub type R = crate::R<I2s1InChSelSpec>;
///Register `I2S1_IN_CH_SEL` writer
pub type W = crate::W<I2s1InChSelSpec>;
///Field `CH2_SEL` reader - DMA channel 2 source: 0=SRC1L, 1=SRC1R, 2=unused
pub type Ch2SelR = crate::FieldReader;
///Field `CH2_SEL` writer - DMA channel 2 source: 0=SRC1L, 1=SRC1R, 2=unused
pub type Ch2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CH1_SEL` reader - DMA channel 1 source: 0=SRC1L, 1=SRC1R, 2=unused
pub type Ch1SelR = crate::FieldReader;
///Field `CH1_SEL` writer - DMA channel 1 source: 0=SRC1L, 1=SRC1R, 2=unused
pub type Ch1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - DMA channel 2 source: 0=SRC1L, 1=SRC1R, 2=unused
    #[inline(always)]
    pub fn ch2_sel(&self) -> Ch2SelR {
        Ch2SelR::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - DMA channel 1 source: 0=SRC1L, 1=SRC1R, 2=unused
    #[inline(always)]
    pub fn ch1_sel(&self) -> Ch1SelR {
        Ch1SelR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - DMA channel 2 source: 0=SRC1L, 1=SRC1R, 2=unused
    #[inline(always)]
    pub fn ch2_sel(&mut self) -> Ch2SelW<'_, I2s1InChSelSpec> {
        Ch2SelW::new(self, 0)
    }
    ///Bits 4:5 - DMA channel 1 source: 0=SRC1L, 1=SRC1R, 2=unused
    #[inline(always)]
    pub fn ch1_sel(&mut self) -> Ch1SelW<'_, I2s1InChSelSpec> {
        Ch1SelW::new(self, 4)
    }
}
/**I2S0 RX DMA channel select

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_ch_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_ch_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1InChSelSpec;
impl crate::RegisterSpec for I2s1InChSelSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_in_ch_sel::R`](R) reader structure
impl crate::Readable for I2s1InChSelSpec {}
///`write(|w| ..)` method takes [`i2s1_in_ch_sel::W`](W) writer structure
impl crate::Writable for I2s1InChSelSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S1_IN_CH_SEL to value 0
impl crate::Resettable for I2s1InChSelSpec {}
