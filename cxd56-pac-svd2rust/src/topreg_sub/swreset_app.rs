///Register `SWRESET_APP` reader
pub type R = crate::R<SwresetAppSpec>;
///Register `SWRESET_APP` writer
pub type W = crate::W<SwresetAppSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Application domain software reset

You can [`read`](crate::Reg::read) this register and get [`swreset_app::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset_app::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SwresetAppSpec;
impl crate::RegisterSpec for SwresetAppSpec {
    type Ux = u32;
}
///`read()` method returns [`swreset_app::R`](R) reader structure
impl crate::Readable for SwresetAppSpec {}
///`write(|w| ..)` method takes [`swreset_app::W`](W) writer structure
impl crate::Writable for SwresetAppSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWRESET_APP to value 0
impl crate::Resettable for SwresetAppSpec {}
