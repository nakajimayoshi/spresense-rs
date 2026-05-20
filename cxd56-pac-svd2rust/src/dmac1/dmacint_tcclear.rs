#[doc = "Register `DMACIntTCClear` writer"]
pub type W = crate::W<DmacintTcclearSpec>;
#[doc = "Field `IntTCClear` writer - Terminal count request clear"]
pub type IntTcclearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Terminal count request clear"]
    #[inline(always)]
    pub fn int_tcclear(&mut self) -> IntTcclearW<'_, DmacintTcclearSpec> {
        IntTcclearW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacint_tcclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacintTcclearSpec;
impl crate::RegisterSpec for DmacintTcclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmacint_tcclear::W`](W) writer structure"]
impl crate::Writable for DmacintTcclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACIntTCClear to value 0"]
impl crate::Resettable for DmacintTcclearSpec {}
