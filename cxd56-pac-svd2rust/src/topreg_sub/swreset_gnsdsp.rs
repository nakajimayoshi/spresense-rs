///Register `SWRESET_GNSDSP` reader
pub type R = crate::R<SwresetGnsdspSpec>;
///Register `SWRESET_GNSDSP` writer
pub type W = crate::W<SwresetGnsdspSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**GNSS DSP software reset

You can [`read`](crate::Reg::read) this register and get [`swreset_gnsdsp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset_gnsdsp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SwresetGnsdspSpec;
impl crate::RegisterSpec for SwresetGnsdspSpec {
    type Ux = u32;
}
///`read()` method returns [`swreset_gnsdsp::R`](R) reader structure
impl crate::Readable for SwresetGnsdspSpec {}
///`write(|w| ..)` method takes [`swreset_gnsdsp::W`](W) writer structure
impl crate::Writable for SwresetGnsdspSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWRESET_GNSDSP to value 0
impl crate::Resettable for SwresetGnsdspSpec {}
