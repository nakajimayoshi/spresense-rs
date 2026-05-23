///Register `CKDIV_COM` reader
pub type R = crate::R<CkdivComSpec>;
///Register `CKDIV_COM` writer
pub type W = crate::W<CkdivComSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**COM clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_com::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_com::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkdivComSpec;
impl crate::RegisterSpec for CkdivComSpec {
    type Ux = u32;
}
///`read()` method returns [`ckdiv_com::R`](R) reader structure
impl crate::Readable for CkdivComSpec {}
///`write(|w| ..)` method takes [`ckdiv_com::W`](W) writer structure
impl crate::Writable for CkdivComSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKDIV_COM to value 0
impl crate::Resettable for CkdivComSpec {}
