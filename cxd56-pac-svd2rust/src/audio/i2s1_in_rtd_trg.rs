///Register `I2S1_IN_RTD_TRG` reader
pub type R = crate::R<I2s1InRtdTrgSpec>;
///Register `I2S1_IN_RTD_TRG` writer
pub type W = crate::W<I2s1InRtdTrgSpec>;
///Field `RTD_TRG` reader - DMA trigger: 1=start, 0=idle (NuttX stop writes 0)
pub type RtdTrgR = crate::FieldReader;
///Field `RTD_TRG` writer - DMA trigger: 1=start, 0=idle (NuttX stop writes 0)
pub type RtdTrgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NOINTR` reader - Suppress per-buffer done interrupt
pub type NointrR = crate::BitReader;
///Field `NOINTR` writer - Suppress per-buffer done interrupt
pub type NointrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - DMA trigger: 1=start, 0=idle (NuttX stop writes 0)
    #[inline(always)]
    pub fn rtd_trg(&self) -> RtdTrgR {
        RtdTrgR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Suppress per-buffer done interrupt
    #[inline(always)]
    pub fn nointr(&self) -> NointrR {
        NointrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - DMA trigger: 1=start, 0=idle (NuttX stop writes 0)
    #[inline(always)]
    pub fn rtd_trg(&mut self) -> RtdTrgW<'_, I2s1InRtdTrgSpec> {
        RtdTrgW::new(self, 0)
    }
    ///Bit 2 - Suppress per-buffer done interrupt
    #[inline(always)]
    pub fn nointr(&mut self) -> NointrW<'_, I2s1InRtdTrgSpec> {
        NointrW::new(self, 2)
    }
}
/**I2S0 RX DMA trigger

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_rtd_trg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_rtd_trg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1InRtdTrgSpec;
impl crate::RegisterSpec for I2s1InRtdTrgSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_in_rtd_trg::R`](R) reader structure
impl crate::Readable for I2s1InRtdTrgSpec {}
///`write(|w| ..)` method takes [`i2s1_in_rtd_trg::W`](W) writer structure
impl crate::Writable for I2s1InRtdTrgSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S1_IN_RTD_TRG to value 0
impl crate::Resettable for I2s1InRtdTrgSpec {}
