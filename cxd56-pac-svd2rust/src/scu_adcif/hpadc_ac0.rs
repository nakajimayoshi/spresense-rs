///Register `HPADC_AC0` reader
pub type R = crate::R<HpadcAc0Spec>;
///Register `HPADC_AC0` writer
pub type W = crate::W<HpadcAc0Spec>;
///Field `BITS` reader - Raw register value
pub type BitsR = crate::FieldReader<u32>;
///Field `BITS` writer - Raw register value
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Raw register value
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Raw register value
    #[inline(always)]
    pub fn bits_(&mut self) -> BitsW<'_, HpadcAc0Spec> {
        BitsW::new(self, 0)
    }
}
/**HPADC common clock control (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc_ac0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc_ac0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HpadcAc0Spec;
impl crate::RegisterSpec for HpadcAc0Spec {
    type Ux = u32;
}
///`read()` method returns [`hpadc_ac0::R`](R) reader structure
impl crate::Readable for HpadcAc0Spec {}
///`write(|w| ..)` method takes [`hpadc_ac0::W`](W) writer structure
impl crate::Writable for HpadcAc0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPADC_AC0 to value 0
impl crate::Resettable for HpadcAc0Spec {}
