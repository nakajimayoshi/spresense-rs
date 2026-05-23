///Register `CKSEL_ROOT_WAKE` reader
pub type R = crate::R<CkselRootWakeSpec>;
///Register `CKSEL_ROOT_WAKE` writer
pub type W = crate::W<CkselRootWakeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Root clock source select used on wake from sleep

You can [`read`](crate::Reg::read) this register and get [`cksel_root_wake::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_root_wake::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkselRootWakeSpec;
impl crate::RegisterSpec for CkselRootWakeSpec {
    type Ux = u32;
}
///`read()` method returns [`cksel_root_wake::R`](R) reader structure
impl crate::Readable for CkselRootWakeSpec {}
///`write(|w| ..)` method takes [`cksel_root_wake::W`](W) writer structure
impl crate::Writable for CkselRootWakeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKSEL_ROOT_WAKE to value 0
impl crate::Resettable for CkselRootWakeSpec {}
