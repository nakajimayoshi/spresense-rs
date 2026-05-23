///Register `CRG_INT_MASK1` reader
pub type R = crate::R<CrgIntMask1Spec>;
///Register `CRG_INT_MASK1` writer
pub type W = crate::W<CrgIntMask1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**TOPREG clock-ready interrupt mask 1

You can [`read`](crate::Reg::read) this register and get [`crg_int_mask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crg_int_mask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrgIntMask1Spec;
impl crate::RegisterSpec for CrgIntMask1Spec {
    type Ux = u32;
}
///`read()` method returns [`crg_int_mask1::R`](R) reader structure
impl crate::Readable for CrgIntMask1Spec {}
///`write(|w| ..)` method takes [`crg_int_mask1::W`](W) writer structure
impl crate::Writable for CrgIntMask1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRG_INT_MASK1 to value 0
impl crate::Resettable for CrgIntMask1Spec {}
