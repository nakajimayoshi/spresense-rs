///Register `CRG_INT_MASK0` reader
pub type R = crate::R<CrgIntMask0Spec>;
///Register `CRG_INT_MASK0` writer
pub type W = crate::W<CrgIntMask0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**TOPREG clock-ready interrupt mask 0

You can [`read`](crate::Reg::read) this register and get [`crg_int_mask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crg_int_mask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrgIntMask0Spec;
impl crate::RegisterSpec for CrgIntMask0Spec {
    type Ux = u32;
}
///`read()` method returns [`crg_int_mask0::R`](R) reader structure
impl crate::Readable for CrgIntMask0Spec {}
///`write(|w| ..)` method takes [`crg_int_mask0::W`](W) writer structure
impl crate::Writable for CrgIntMask0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRG_INT_MASK0 to value 0
impl crate::Resettable for CrgIntMask0Spec {}
