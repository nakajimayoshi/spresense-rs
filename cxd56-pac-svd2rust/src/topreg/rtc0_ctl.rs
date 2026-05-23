///Register `RTC0_CTL` reader
pub type R = crate::R<Rtc0CtlSpec>;
///Register `RTC0_CTL` writer
pub type W = crate::W<Rtc0CtlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**RTC0 control

You can [`read`](crate::Reg::read) this register and get [`rtc0_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc0_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Rtc0CtlSpec;
impl crate::RegisterSpec for Rtc0CtlSpec {
    type Ux = u32;
}
///`read()` method returns [`rtc0_ctl::R`](R) reader structure
impl crate::Readable for Rtc0CtlSpec {}
///`write(|w| ..)` method takes [`rtc0_ctl::W`](W) writer structure
impl crate::Writable for Rtc0CtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTC0_CTL to value 0
impl crate::Resettable for Rtc0CtlSpec {}
