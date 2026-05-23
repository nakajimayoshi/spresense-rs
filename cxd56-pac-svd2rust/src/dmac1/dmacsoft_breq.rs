///Register `DMACSoftBReq` reader
pub type R = crate::R<DmacsoftBreqSpec>;
///Register `DMACSoftBReq` writer
pub type W = crate::W<DmacsoftBreqSpec>;
///Field `SoftBReq` reader - Software burst request
pub type SoftBreqR = crate::FieldReader<u16>;
///Field `SoftBReq` writer - Software burst request
pub type SoftBreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Software burst request
    #[inline(always)]
    pub fn soft_breq(&self) -> SoftBreqR {
        SoftBreqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Software burst request
    #[inline(always)]
    pub fn soft_breq(&mut self) -> SoftBreqW<'_, DmacsoftBreqSpec> {
        SoftBreqW::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacsoft_breq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_breq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacsoftBreqSpec;
impl crate::RegisterSpec for DmacsoftBreqSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacsoft_breq::R`](R) reader structure
impl crate::Readable for DmacsoftBreqSpec {}
///`write(|w| ..)` method takes [`dmacsoft_breq::W`](W) writer structure
impl crate::Writable for DmacsoftBreqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACSoftBReq to value 0
impl crate::Resettable for DmacsoftBreqSpec {}
