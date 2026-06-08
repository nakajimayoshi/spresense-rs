///Register `HPADC1_D0` reader
pub type R = crate::R<Hpadc1D0Spec>;
///Register `HPADC1_D0` writer
pub type W = crate::W<Hpadc1D0Spec>;
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
    pub fn bits_(&mut self) -> BitsW<'_, Hpadc1D0Spec> {
        BitsW::new(self, 0)
    }
}
/**HPADC1 software reset (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc1_d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc1_d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Hpadc1D0Spec;
impl crate::RegisterSpec for Hpadc1D0Spec {
    type Ux = u32;
}
///`read()` method returns [`hpadc1_d0::R`](R) reader structure
impl crate::Readable for Hpadc1D0Spec {}
///`write(|w| ..)` method takes [`hpadc1_d0::W`](W) writer structure
impl crate::Writable for Hpadc1D0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPADC1_D0 to value 0
impl crate::Resettable for Hpadc1D0Spec {}
