///Register `REQ[%s]` writer
pub type W = crate::W<ReqSpec>;
///Field `REQ` writer - Request command
pub type ReqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    ///Bits 0:1 - Request command
    #[inline(always)]
    pub fn req(&mut self) -> ReqW<'_, ReqSpec> {
        ReqW::new(self, 0)
    }
}
/**Semaphore request command (write-only)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`req::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ReqSpec;
impl crate::RegisterSpec for ReqSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`req::W`](W) writer structure
impl crate::Writable for ReqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REQ[%s] to value 0
impl crate::Resettable for ReqSpec {}
