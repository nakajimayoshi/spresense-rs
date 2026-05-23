///Register `DMACIntTCClear` writer
pub type W = crate::W<DmacintTcclearSpec>;
///Field `IntTCClear` writer - Terminal count request clear
pub type IntTcclearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Terminal count request clear
    #[inline(always)]
    pub fn int_tcclear(&mut self) -> IntTcclearW<'_, DmacintTcclearSpec> {
        IntTcclearW::new(self, 0)
    }
}
/**

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacint_tcclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacintTcclearSpec;
impl crate::RegisterSpec for DmacintTcclearSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dmacint_tcclear::W`](W) writer structure
impl crate::Writable for DmacintTcclearSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACIntTCClear to value 0
impl crate::Resettable for DmacintTcclearSpec {}
