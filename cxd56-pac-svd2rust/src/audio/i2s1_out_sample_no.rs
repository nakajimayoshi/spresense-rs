///Register `I2S1_OUT_SAMPLE_NO` reader
pub type R = crate::R<I2s1OutSampleNoSpec>;
///Register `I2S1_OUT_SAMPLE_NO` writer
pub type W = crate::W<I2s1OutSampleNoSpec>;
///Field `SAMPLE_NO` reader - Number of samples to transfer, minus one
pub type SampleNoR = crate::FieldReader<u32>;
///Field `SAMPLE_NO` writer - Number of samples to transfer, minus one
pub type SampleNoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Number of samples to transfer, minus one
    #[inline(always)]
    pub fn sample_no(&self) -> SampleNoR {
        SampleNoR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Number of samples to transfer, minus one
    #[inline(always)]
    pub fn sample_no(&mut self) -> SampleNoW<'_, I2s1OutSampleNoSpec> {
        SampleNoW::new(self, 0)
    }
}
/**I2S0 TX DMA transfer length (samples - 1)

You can [`read`](crate::Reg::read) this register and get [`i2s1_out_sample_no::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_out_sample_no::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1OutSampleNoSpec;
impl crate::RegisterSpec for I2s1OutSampleNoSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_out_sample_no::R`](R) reader structure
impl crate::Readable for I2s1OutSampleNoSpec {}
///`write(|w| ..)` method takes [`i2s1_out_sample_no::W`](W) writer structure
impl crate::Writable for I2s1OutSampleNoSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S1_OUT_SAMPLE_NO to value 0
impl crate::Resettable for I2s1OutSampleNoSpec {}
