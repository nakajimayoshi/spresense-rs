///Register `XOSC_CTRL2` reader
pub type R = crate::R<XoscCtrl2Spec>;
///Register `XOSC_CTRL2` writer
pub type W = crate::W<XoscCtrl2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Crystal oscillator control 2

You can [`read`](crate::Reg::read) this register and get [`xosc_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XoscCtrl2Spec;
impl crate::RegisterSpec for XoscCtrl2Spec {
    type Ux = u32;
}
///`read()` method returns [`xosc_ctrl2::R`](R) reader structure
impl crate::Readable for XoscCtrl2Spec {}
///`write(|w| ..)` method takes [`xosc_ctrl2::W`](W) writer structure
impl crate::Writable for XoscCtrl2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets XOSC_CTRL2 to value 0
impl crate::Resettable for XoscCtrl2Spec {}
