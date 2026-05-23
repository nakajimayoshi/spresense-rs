///Register `CKDIV_HOSTIFC` reader
pub type R = crate::R<CkdivHostifcSpec>;
///Register `CKDIV_HOSTIFC` writer
pub type W = crate::W<CkdivHostifcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Host interface clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_hostifc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_hostifc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkdivHostifcSpec;
impl crate::RegisterSpec for CkdivHostifcSpec {
    type Ux = u32;
}
///`read()` method returns [`ckdiv_hostifc::R`](R) reader structure
impl crate::Readable for CkdivHostifcSpec {}
///`write(|w| ..)` method takes [`ckdiv_hostifc::W`](W) writer structure
impl crate::Writable for CkdivHostifcSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKDIV_HOSTIFC to value 0
impl crate::Resettable for CkdivHostifcSpec {}
