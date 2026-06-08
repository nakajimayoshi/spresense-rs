///Register `LPADC_D2` reader
pub type R = crate::R<LpadcD2Spec>;
///Register `LPADC_D2` writer
pub type W = crate::W<LpadcD2Spec>;
///Field `FIFO_EN` reader - Per-channel FIFO enable: bit0=ch0, bit1=ch1, bit2=ch2, bit3=ch3
pub type FifoEnR = crate::FieldReader;
///Field `FIFO_EN` writer - Per-channel FIFO enable: bit0=ch0, bit1=ch1, bit2=ch2, bit3=ch3
pub type FifoEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Per-channel FIFO enable: bit0=ch0, bit1=ch1, bit2=ch2, bit3=ch3
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Per-channel FIFO enable: bit0=ch0, bit1=ch1, bit2=ch2, bit3=ch3
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<'_, LpadcD2Spec> {
        FifoEnW::new(self, 0)
    }
}
/**LPADC per-channel FIFO enable (UM Table ADC-110). Bit n=1 enables ADCIF hardware FIFO for channel n.

You can [`read`](crate::Reg::read) this register and get [`lpadc_d2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpadcD2Spec;
impl crate::RegisterSpec for LpadcD2Spec {
    type Ux = u32;
}
///`read()` method returns [`lpadc_d2::R`](R) reader structure
impl crate::Readable for LpadcD2Spec {}
///`write(|w| ..)` method takes [`lpadc_d2::W`](W) writer structure
impl crate::Writable for LpadcD2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPADC_D2 to value 0
impl crate::Resettable for LpadcD2Spec {}
