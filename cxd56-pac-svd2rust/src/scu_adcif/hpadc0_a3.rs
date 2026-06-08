///Register `HPADC0_A3` reader
pub type R = crate::R<Hpadc0A3Spec>;
///Register `HPADC0_A3` writer
pub type W = crate::W<Hpadc0A3Spec>;
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
    pub fn bits_(&mut self) -> BitsW<'_, Hpadc0A3Spec> {
        BitsW::new(self, 0)
    }
}
/**HPADC0 LPF control (placeholder; reset=0x100)

You can [`read`](crate::Reg::read) this register and get [`hpadc0_a3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc0_a3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Hpadc0A3Spec;
impl crate::RegisterSpec for Hpadc0A3Spec {
    type Ux = u32;
}
///`read()` method returns [`hpadc0_a3::R`](R) reader structure
impl crate::Readable for Hpadc0A3Spec {}
///`write(|w| ..)` method takes [`hpadc0_a3::W`](W) writer structure
impl crate::Writable for Hpadc0A3Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPADC0_A3 to value 0x0100
impl crate::Resettable for Hpadc0A3Spec {
    const RESET_VALUE: u32 = 0x0100;
}
