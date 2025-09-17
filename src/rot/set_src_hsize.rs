#[doc = "Register `SET_SRC_HSIZE` reader"]
pub type R = crate::R<SetSrcHsizeSpec>;
#[doc = "Register `SET_SRC_HSIZE` writer"]
pub type W = crate::W<SetSrcHsizeSpec>;
#[doc = "Field `SIZE` reader - "]
pub type SizeR = crate::FieldReader<u16>;
#[doc = "Field `SIZE` writer - "]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, SetSrcHsizeSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "Source Image Horizontal Size (Actual size - 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`set_src_hsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_src_hsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetSrcHsizeSpec;
impl crate::RegisterSpec for SetSrcHsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set_src_hsize::R`](R) reader structure"]
impl crate::Readable for SetSrcHsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`set_src_hsize::W`](W) writer structure"]
impl crate::Writable for SetSrcHsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_SRC_HSIZE to value 0"]
impl crate::Resettable for SetSrcHsizeSpec {}
