///Register `RCOSC_CTRL1` reader
pub type R = crate::R<RcoscCtrl1Spec>;
///Register `RCOSC_CTRL1` writer
pub type W = crate::W<RcoscCtrl1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**RC oscillator control 1

You can [`read`](crate::Reg::read) this register and get [`rcosc_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcosc_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RcoscCtrl1Spec;
impl crate::RegisterSpec for RcoscCtrl1Spec {
    type Ux = u32;
}
///`read()` method returns [`rcosc_ctrl1::R`](R) reader structure
impl crate::Readable for RcoscCtrl1Spec {}
///`write(|w| ..)` method takes [`rcosc_ctrl1::W`](W) writer structure
impl crate::Writable for RcoscCtrl1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCOSC_CTRL1 to value 0
impl crate::Resettable for RcoscCtrl1Spec {}
