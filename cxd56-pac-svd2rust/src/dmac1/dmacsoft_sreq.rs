///Register `DMACSoftSReq` reader
pub type R = crate::R<DmacsoftSreqSpec>;
///Register `DMACSoftSReq` writer
pub type W = crate::W<DmacsoftSreqSpec>;
///Field `SoftSReq` reader - Software single request
pub type SoftSreqR = crate::FieldReader<u16>;
///Field `SoftSReq` writer - Software single request
pub type SoftSreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Software single request
    #[inline(always)]
    pub fn soft_sreq(&self) -> SoftSreqR {
        SoftSreqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Software single request
    #[inline(always)]
    pub fn soft_sreq(&mut self) -> SoftSreqW<'_, DmacsoftSreqSpec> {
        SoftSreqW::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacsoft_sreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsoft_sreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacsoftSreqSpec;
impl crate::RegisterSpec for DmacsoftSreqSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacsoft_sreq::R`](R) reader structure
impl crate::Readable for DmacsoftSreqSpec {}
///`write(|w| ..)` method takes [`dmacsoft_sreq::W`](W) writer structure
impl crate::Writable for DmacsoftSreqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACSoftSReq to value 0
impl crate::Resettable for DmacsoftSreqSpec {}
