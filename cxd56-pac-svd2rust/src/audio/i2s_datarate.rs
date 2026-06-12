///Register `I2S_DATARATE` reader
pub type R = crate::R<I2sDatarateSpec>;
///Register `I2S_DATARATE` writer
pub type W = crate::W<I2sDatarateSpec>;
///Field `I2SALL_DATARATE` reader - I2S data rate: 0=48kHz, 1=192kHz
pub type I2sallDatarateR = crate::BitReader;
///Field `I2SALL_DATARATE` writer - I2S data rate: 0=48kHz, 1=192kHz
pub type I2sallDatarateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2S data rate: 0=48kHz, 1=192kHz
    #[inline(always)]
    pub fn i2sall_datarate(&self) -> I2sallDatarateR {
        I2sallDatarateR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I2S data rate: 0=48kHz, 1=192kHz
    #[inline(always)]
    pub fn i2sall_datarate(&mut self) -> I2sallDatarateW<'_, I2sDatarateSpec> {
        I2sallDatarateW::new(self, 0)
    }
}
/**I2S sample-rate class select

You can [`read`](crate::Reg::read) this register and get [`i2s_datarate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_datarate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2sDatarateSpec;
impl crate::RegisterSpec for I2sDatarateSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s_datarate::R`](R) reader structure
impl crate::Readable for I2sDatarateSpec {}
///`write(|w| ..)` method takes [`i2s_datarate::W`](W) writer structure
impl crate::Writable for I2sDatarateSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S_DATARATE to value 0
impl crate::Resettable for I2sDatarateSpec {}
