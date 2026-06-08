///Register `HPADC1_D2` reader
pub type R = crate::R<Hpadc1D2Spec>;
///Register `HPADC1_D2` writer
pub type W = crate::W<Hpadc1D2Spec>;
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
    pub fn bits_(&mut self) -> BitsW<'_, Hpadc1D2Spec> {
        BitsW::new(self, 0)
    }
}
/**HPADC1 ADC data acceptance (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_d2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_d2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Hpadc1D2Spec;
impl crate::RegisterSpec for Hpadc1D2Spec {
    type Ux = u32;
}
///`read()` method returns [`hpadc1_d2::R`](R) reader structure
impl crate::Readable for Hpadc1D2Spec {}
///`write(|w| ..)` method takes [`hpadc1_d2::W`](W) writer structure
impl crate::Writable for Hpadc1D2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPADC1_D2 to value 0
impl crate::Resettable for Hpadc1D2Spec {}
