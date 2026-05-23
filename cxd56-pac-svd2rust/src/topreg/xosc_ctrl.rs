///Register `XOSC_CTRL` reader
pub type R = crate::R<XoscCtrlSpec>;
///Register `XOSC_CTRL` writer
pub type W = crate::W<XoscCtrlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Crystal oscillator control

You can [`read`](crate::Reg::read) this register and get [`xosc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XoscCtrlSpec;
impl crate::RegisterSpec for XoscCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`xosc_ctrl::R`](R) reader structure
impl crate::Readable for XoscCtrlSpec {}
///`write(|w| ..)` method takes [`xosc_ctrl::W`](W) writer structure
impl crate::Writable for XoscCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets XOSC_CTRL to value 0
impl crate::Resettable for XoscCtrlSpec {}
