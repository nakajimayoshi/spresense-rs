///Register `CLSELDIV_WAKE` reader
pub type R = crate::R<ClseldivWakeSpec>;
///Register `CLSELDIV_WAKE` writer
pub type W = crate::W<ClseldivWakeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Clock select/divider setting used on wake from sleep

You can [`read`](crate::Reg::read) this register and get [`clseldiv_wake::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clseldiv_wake::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ClseldivWakeSpec;
impl crate::RegisterSpec for ClseldivWakeSpec {
    type Ux = u32;
}
///`read()` method returns [`clseldiv_wake::R`](R) reader structure
impl crate::Readable for ClseldivWakeSpec {}
///`write(|w| ..)` method takes [`clseldiv_wake::W`](W) writer structure
impl crate::Writable for ClseldivWakeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLSELDIV_WAKE to value 0
impl crate::Resettable for ClseldivWakeSpec {}
