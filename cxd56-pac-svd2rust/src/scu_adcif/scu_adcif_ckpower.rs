///Register `SCU_ADCIF_CKPOWER` reader
pub type R = crate::R<ScuAdcifCkpowerSpec>;
///Register `SCU_ADCIF_CKPOWER` writer
pub type W = crate::W<ScuAdcifCkpowerSpec>;
///Field `BITS` reader - Clock/power control bits
pub type BitsR = crate::FieldReader<u32>;
///Field `BITS` writer - Clock/power control bits
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Clock/power control bits
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Clock/power control bits
    #[inline(always)]
    pub fn bits_(&mut self) -> BitsW<'_, ScuAdcifCkpowerSpec> {
        BitsW::new(self, 0)
    }
}
/**ADCIF clock and power control (reset=0x1 = clocked; write 0 to gate)

You can [`read`](crate::Reg::read) this register and get [`scu_adcif_ckpower::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu_adcif_ckpower::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScuAdcifCkpowerSpec;
impl crate::RegisterSpec for ScuAdcifCkpowerSpec {
    type Ux = u32;
}
///`read()` method returns [`scu_adcif_ckpower::R`](R) reader structure
impl crate::Readable for ScuAdcifCkpowerSpec {}
///`write(|w| ..)` method takes [`scu_adcif_ckpower::W`](W) writer structure
impl crate::Writable for ScuAdcifCkpowerSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCU_ADCIF_CKPOWER to value 0x01
impl crate::Resettable for ScuAdcifCkpowerSpec {
    const RESET_VALUE: u32 = 0x01;
}
