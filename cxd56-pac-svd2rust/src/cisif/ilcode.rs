#[doc = "Register `ILCODE` reader"]
pub type R = crate::R<IlcodeSpec>;
#[doc = "Register `ILCODE` writer"]
pub type W = crate::W<IlcodeSpec>;
#[doc = "Field `sosi` reader - "]
pub type SosiR = crate::FieldReader;
#[doc = "Field `sosi` writer - "]
pub type SosiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `eosi` reader - "]
pub type EosiR = crate::FieldReader;
#[doc = "Field `eosi` writer - "]
pub type EosiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `soy` reader - "]
pub type SoyR = crate::FieldReader;
#[doc = "Field `soy` writer - "]
pub type SoyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `eoy` reader - "]
pub type EoyR = crate::FieldReader;
#[doc = "Field `eoy` writer - "]
pub type EoyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sosi(&self) -> SosiR {
        SosiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn eosi(&self) -> EosiR {
        EosiR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn soy(&self) -> SoyR {
        SoyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn eoy(&self) -> EoyR {
        EoyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sosi(&mut self) -> SosiW<'_, IlcodeSpec> {
        SosiW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn eosi(&mut self) -> EosiW<'_, IlcodeSpec> {
        EosiW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn soy(&mut self) -> SoyW<'_, IlcodeSpec> {
        SoyW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn eoy(&mut self) -> EoyW<'_, IlcodeSpec> {
        EoyW::new(self, 24)
    }
}
#[doc = "CIS input in line code setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`ilcode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilcode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IlcodeSpec;
impl crate::RegisterSpec for IlcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ilcode::R`](R) reader structure"]
impl crate::Readable for IlcodeSpec {}
#[doc = "`write(|w| ..)` method takes [`ilcode::W`](W) writer structure"]
impl crate::Writable for IlcodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ILCODE to value 0"]
impl crate::Resettable for IlcodeSpec {}
