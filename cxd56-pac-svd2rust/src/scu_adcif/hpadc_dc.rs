///Register `HPADC_DC` reader
pub type R = crate::R<HpadcDcSpec>;
///Register `HPADC_DC` writer
pub type W = crate::W<HpadcDcSpec>;
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
    pub fn bits_(&mut self) -> BitsW<'_, HpadcDcSpec> {
        BitsW::new(self, 0)
    }
}
/**HPADC two-element vector mode control (placeholder)

You can [`read`](crate::Reg::read) this register and get [`hpadc_dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpadc_dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HpadcDcSpec;
impl crate::RegisterSpec for HpadcDcSpec {
    type Ux = u32;
}
///`read()` method returns [`hpadc_dc::R`](R) reader structure
impl crate::Readable for HpadcDcSpec {}
///`write(|w| ..)` method takes [`hpadc_dc::W`](W) writer structure
impl crate::Writable for HpadcDcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HPADC_DC to value 0
impl crate::Resettable for HpadcDcSpec {}
