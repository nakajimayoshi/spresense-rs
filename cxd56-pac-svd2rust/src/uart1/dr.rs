///Register `DR` reader
pub type R = crate::R<DrSpec>;
///Register `DR` writer
pub type W = crate::W<DrSpec>;
/**Field `DATA` reader - Data

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DataR = crate::FieldReader;
///Field `DATA` writer - Data
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Framing Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    ///0: no error
    None = 0,
    ///1: error
    Error = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
///Field `FE` reader - Framing Error
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::None,
            true => Fe::Error,
        }
    }
    ///no error
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Fe::None
    }
    ///error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Fe::Error
    }
}
/**Parity Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    ///0: no error
    None = 0,
    ///1: error
    Error = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Parity Error
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::None,
            true => Pe::Error,
        }
    }
    ///no error
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pe::None
    }
    ///error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Pe::Error
    }
}
/**Break Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be {
    ///0: no error
    None = 0,
    ///1: error
    Error = 1,
}
impl From<Be> for bool {
    #[inline(always)]
    fn from(variant: Be) -> Self {
        variant as u8 != 0
    }
}
///Field `BE` reader - Break Error
pub type BeR = crate::BitReader<Be>;
impl BeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Be {
        match self.bits {
            false => Be::None,
            true => Be::Error,
        }
    }
    ///no error
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Be::None
    }
    ///error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Be::Error
    }
}
/**Overrun Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oe {
    ///0: no error
    None = 0,
    ///1: error
    Error = 1,
}
impl From<Oe> for bool {
    #[inline(always)]
    fn from(variant: Oe) -> Self {
        variant as u8 != 0
    }
}
///Field `OE` reader - Overrun Error
pub type OeR = crate::BitReader<Oe>;
impl OeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Oe {
        match self.bits {
            false => Oe::None,
            true => Oe::Error,
        }
    }
    ///no error
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Oe::None
    }
    ///error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Oe::Error
    }
}
impl R {
    ///Bits 0:7 - Data
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Framing Error
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity Error
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Break Error
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Overrun Error
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Data
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, DrSpec> {
        DataW::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DrSpec {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DrSpec {}
