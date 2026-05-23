///Register `CKDIV_SCU` reader
pub type R = crate::R<CkdivScuSpec>;
///Register `CKDIV_SCU` writer
pub type W = crate::W<CkdivScuSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**SCU clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_scu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_scu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkdivScuSpec;
impl crate::RegisterSpec for CkdivScuSpec {
    type Ux = u32;
}
///`read()` method returns [`ckdiv_scu::R`](R) reader structure
impl crate::Readable for CkdivScuSpec {}
///`write(|w| ..)` method takes [`ckdiv_scu::W`](W) writer structure
impl crate::Writable for CkdivScuSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKDIV_SCU to value 0
impl crate::Resettable for CkdivScuSpec {}
