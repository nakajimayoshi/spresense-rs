#[doc = "Register `SET_SRC_ADDRESS` reader"]
pub type R = crate::R<SetSrcAddressSpec>;
#[doc = "Register `SET_SRC_ADDRESS` writer"]
pub type W = crate::W<SetSrcAddressSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Source Image Address\n\nYou can [`read`](crate::Reg::read) this register and get [`set_src_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_src_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetSrcAddressSpec;
impl crate::RegisterSpec for SetSrcAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set_src_address::R`](R) reader structure"]
impl crate::Readable for SetSrcAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`set_src_address::W`](W) writer structure"]
impl crate::Writable for SetSrcAddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_SRC_ADDRESS to value 0"]
impl crate::Resettable for SetSrcAddressSpec {}
