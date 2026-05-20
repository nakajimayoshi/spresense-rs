#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DATA` reader - Data\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Framing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: no error"]
    None = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error"]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::None,
            true => Fe::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Fe::None
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Fe::Error
    }
}
#[doc = "Parity Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: no error"]
    None = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Error"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::None,
            true => Pe::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pe::None
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Pe::Error
    }
}
#[doc = "Break Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be {
    #[doc = "0: no error"]
    None = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Be> for bool {
    #[inline(always)]
    fn from(variant: Be) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BE` reader - Break Error"]
pub type BeR = crate::BitReader<Be>;
impl BeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Be {
        match self.bits {
            false => Be::None,
            true => Be::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Be::None
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Be::Error
    }
}
#[doc = "Overrun Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oe {
    #[doc = "0: no error"]
    None = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Oe> for bool {
    #[inline(always)]
    fn from(variant: Oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - Overrun Error"]
pub type OeR = crate::BitReader<Oe>;
impl OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oe {
        match self.bits {
            false => Oe::None,
            true => Oe::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Oe::None
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Oe::Error
    }
}
impl R {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Framing Error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Break Error"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun Error"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, DrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {}
