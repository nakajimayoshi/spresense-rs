///Register `I2S1_IN_BITWT` reader
pub type R = crate::R<I2s1InBitwtSpec>;
///Register `I2S1_IN_BITWT` writer
pub type W = crate::W<I2s1InBitwtSpec>;
///Field `BITWT` reader - 0=24-bit (one channel per word), 1=16-bit (L/R packed per word)
pub type BitwtR = crate::BitReader;
///Field `BITWT` writer - 0=24-bit (one channel per word), 1=16-bit (L/R packed per word)
pub type BitwtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 0=24-bit (one channel per word), 1=16-bit (L/R packed per word)
    #[inline(always)]
    pub fn bitwt(&self) -> BitwtR {
        BitwtR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - 0=24-bit (one channel per word), 1=16-bit (L/R packed per word)
    #[inline(always)]
    pub fn bitwt(&mut self) -> BitwtW<'_, I2s1InBitwtSpec> {
        BitwtW::new(self, 0)
    }
}
/**I2S0 RX DMA sample width

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_bitwt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_bitwt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1InBitwtSpec;
impl crate::RegisterSpec for I2s1InBitwtSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_in_bitwt::R`](R) reader structure
impl crate::Readable for I2s1InBitwtSpec {}
///`write(|w| ..)` method takes [`i2s1_in_bitwt::W`](W) writer structure
impl crate::Writable for I2s1InBitwtSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S1_IN_BITWT to value 0
impl crate::Resettable for I2s1InBitwtSpec {}
