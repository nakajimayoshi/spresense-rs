///Register `CKGATE_CTL` reader
pub type R = crate::R<CkgateCtlSpec>;
///Register `CKGATE_CTL` writer
pub type W = crate::W<CkgateCtlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Clock gate control

You can [`read`](crate::Reg::read) this register and get [`ckgate_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgate_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkgateCtlSpec;
impl crate::RegisterSpec for CkgateCtlSpec {
    type Ux = u32;
}
///`read()` method returns [`ckgate_ctl::R`](R) reader structure
impl crate::Readable for CkgateCtlSpec {}
///`write(|w| ..)` method takes [`ckgate_ctl::W`](W) writer structure
impl crate::Writable for CkgateCtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKGATE_CTL to value 0
impl crate::Resettable for CkgateCtlSpec {}
