///Register `I2S1_IN_START_ADR` reader
pub type R = crate::R<I2s1InStartAdrSpec>;
///Register `I2S1_IN_START_ADR` writer
pub type W = crate::W<I2s1InStartAdrSpec>;
///Field `START_ADR` reader - Buffer byte address >> 2 (must be 4-byte aligned)
pub type StartAdrR = crate::FieldReader<u32>;
///Field `START_ADR` writer - Buffer byte address >> 2 (must be 4-byte aligned)
pub type StartAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Buffer byte address >> 2 (must be 4-byte aligned)
    #[inline(always)]
    pub fn start_adr(&self) -> StartAdrR {
        StartAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 2:31 - Buffer byte address >> 2 (must be 4-byte aligned)
    #[inline(always)]
    pub fn start_adr(&mut self) -> StartAdrW<'_, I2s1InStartAdrSpec> {
        StartAdrW::new(self, 2)
    }
}
/**I2S0 RX DMA buffer start address (word-aligned; byte address >> 2)

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_start_adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_in_start_adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1InStartAdrSpec;
impl crate::RegisterSpec for I2s1InStartAdrSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_in_start_adr::R`](R) reader structure
impl crate::Readable for I2s1InStartAdrSpec {}
///`write(|w| ..)` method takes [`i2s1_in_start_adr::W`](W) writer structure
impl crate::Writable for I2s1InStartAdrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S1_IN_START_ADR to value 0
impl crate::Resettable for I2s1InStartAdrSpec {}
