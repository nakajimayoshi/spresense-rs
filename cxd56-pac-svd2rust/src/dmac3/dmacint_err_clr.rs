///Register `DMACIntErrClr` writer
pub type W = crate::W<DmacintErrClrSpec>;
///Field `IntErrClr` writer - Interrupt error clear
pub type IntErrClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Interrupt error clear
    #[inline(always)]
    pub fn int_err_clr(&mut self) -> IntErrClrW<'_, DmacintErrClrSpec> {
        IntErrClrW::new(self, 0)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacint_err_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacintErrClrSpec;
impl crate::RegisterSpec for DmacintErrClrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dmacint_err_clr::W`](W) writer structure
impl crate::Writable for DmacintErrClrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACIntErrClr to value 0
impl crate::Resettable for DmacintErrClrSpec {}
