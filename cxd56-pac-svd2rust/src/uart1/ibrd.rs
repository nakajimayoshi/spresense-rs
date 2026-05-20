#[doc = "Register `IBRD` reader"]
pub type R = crate::R<IbrdSpec>;
#[doc = "Register `IBRD` writer"]
pub type W = crate::W<IbrdSpec>;
#[doc = "Field `BAUD_DIVINT` reader - The integer baud rate divisor"]
pub type BaudDivintR = crate::FieldReader<u16>;
#[doc = "Field `BAUD_DIVINT` writer - The integer baud rate divisor"]
pub type BaudDivintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The integer baud rate divisor"]
    #[inline(always)]
    pub fn baud_divint(&self) -> BaudDivintR {
        BaudDivintR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The integer baud rate divisor"]
    #[inline(always)]
    pub fn baud_divint(&mut self) -> BaudDivintW<'_, IbrdSpec> {
        BaudDivintW::new(self, 0)
    }
}
#[doc = "The integer part of the baud rate divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`ibrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IbrdSpec;
impl crate::RegisterSpec for IbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibrd::R`](R) reader structure"]
impl crate::Readable for IbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`ibrd::W`](W) writer structure"]
impl crate::Writable for IbrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IbrdSpec {}
