#[doc = "Register `EXE_CMD` writer"]
pub type W = crate::W<ExeCmdSpec>;
#[doc = "Field `exe_cmd` writer - "]
pub type ExeCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn exe_cmd(&mut self) -> ExeCmdW<'_, ExeCmdSpec> {
        ExeCmdW::new(self, 0)
    }
}
#[doc = "Execution command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exe_cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExeCmdSpec;
impl crate::RegisterSpec for ExeCmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`exe_cmd::W`](W) writer structure"]
impl crate::Writable for ExeCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXE_CMD to value 0"]
impl crate::Resettable for ExeCmdSpec {}
