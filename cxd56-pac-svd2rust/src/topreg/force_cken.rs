///Register `FORCE_CKEN` reader
pub type R = crate::R<ForceCkenSpec>;
///Register `FORCE_CKEN` writer
pub type W = crate::W<ForceCkenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Force clock enable (overrides gating)

You can [`read`](crate::Reg::read) this register and get [`force_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ForceCkenSpec;
impl crate::RegisterSpec for ForceCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`force_cken::R`](R) reader structure
impl crate::Readable for ForceCkenSpec {}
///`write(|w| ..)` method takes [`force_cken::W`](W) writer structure
impl crate::Writable for ForceCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FORCE_CKEN to value 0
impl crate::Resettable for ForceCkenSpec {}
