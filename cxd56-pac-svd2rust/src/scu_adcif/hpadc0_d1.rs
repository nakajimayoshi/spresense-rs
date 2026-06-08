///Register `HPADC0_D1` reader
pub type R = crate::R<Hpadc0D1Spec>;
///Register `HPADC0_D1` writer
pub type W = crate::W<Hpadc0D1Spec>;
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
    pub fn bits_(&mut self) -> BitsW<'_, Hpadc0D1Spec> {
        BitsW::new(self, 0)
    }
}
/**HPADC0 basic setting (placeholder; reset=0x10)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_d1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_d1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Hpadc0D1Spec;
impl crate::RegisterSpec for Hpadc0D1Spec {
    type Ux = u32;
}
///`read()` method returns [`hpadc0_d1::R`](R) reader structure
impl crate::Readable for Hpadc0D1Spec {}
///`write(|w| ..)` method takes [`hpadc0_d1::W`](W) writer structure
impl crate::Writable for Hpadc0D1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPADC0_D1 to value 0x10
impl crate::Resettable for Hpadc0D1Spec {
    const RESET_VALUE: u32 = 0x10;
}
