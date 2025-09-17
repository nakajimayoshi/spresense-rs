#[doc = "Register `FBRD` reader"]
pub type R = crate::R<FbrdSpec>;
#[doc = "Register `FBRD` writer"]
pub type W = crate::W<FbrdSpec>;
#[doc = "Field `BAUD_DIVFRAC` reader - The fractional baud rate divisor"]
pub type BaudDivfracR = crate::FieldReader;
#[doc = "Field `BAUD_DIVFRAC` writer - The fractional baud rate divisor"]
pub type BaudDivfracW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The fractional baud rate divisor"]
    #[inline(always)]
    pub fn baud_divfrac(&self) -> BaudDivfracR {
        BaudDivfracR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The fractional baud rate divisor"]
    #[inline(always)]
    pub fn baud_divfrac(&mut self) -> BaudDivfracW<'_, FbrdSpec> {
        BaudDivfracW::new(self, 0)
    }
}
#[doc = "The fractional part of the baud rate divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`fbrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbrdSpec;
impl crate::RegisterSpec for FbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbrd::R`](R) reader structure"]
impl crate::Readable for FbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`fbrd::W`](W) writer structure"]
impl crate::Writable for FbrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FbrdSpec {}
