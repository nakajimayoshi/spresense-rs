#[doc = "Register `RSR` reader"]
pub type R = crate::R<RsrSpec>;
#[doc = "Framing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfe {
    #[doc = "0: no error"]
    None = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Rfe> for bool {
    #[inline(always)]
    fn from(variant: Rfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - Framing Error"]
pub type RfeR = crate::BitReader<Rfe>;
impl RfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfe {
        match self.bits {
            false => Rfe::None,
            true => Rfe::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rfe::None
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Rfe::Error
    }
}
#[doc = "Parity Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpe {
    #[doc = "0: no error"]
    None = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Rpe> for bool {
    #[inline(always)]
    fn from(variant: Rpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPE` reader - Parity Error"]
pub type RpeR = crate::BitReader<Rpe>;
impl RpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpe {
        match self.bits {
            false => Rpe::None,
            true => Rpe::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rpe::None
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Rpe::Error
    }
}
#[doc = "Break Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbe {
    #[doc = "0: no error"]
    None = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Rbe> for bool {
    #[inline(always)]
    fn from(variant: Rbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBE` reader - Break Error"]
pub type RbeR = crate::BitReader<Rbe>;
impl RbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbe {
        match self.bits {
            false => Rbe::None,
            true => Rbe::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rbe::None
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Rbe::Error
    }
}
#[doc = "Overrun Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Roe {
    #[doc = "0: no error"]
    None = 0,
    #[doc = "1: error"]
    Error = 1,
}
impl From<Roe> for bool {
    #[inline(always)]
    fn from(variant: Roe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROE` reader - Overrun Error"]
pub type RoeR = crate::BitReader<Roe>;
impl RoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Roe {
        match self.bits {
            false => Roe::None,
            true => Roe::Error,
        }
    }
    #[doc = "no error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Roe::None
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Roe::Error
    }
}
impl R {
    #[doc = "Bit 0 - Framing Error"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error"]
    #[inline(always)]
    pub fn rpe(&self) -> RpeR {
        RpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break Error"]
    #[inline(always)]
    pub fn rbe(&self) -> RbeR {
        RbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error"]
    #[inline(always)]
    pub fn roe(&self) -> RoeR {
        RoeR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Receive Status and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsrSpec;
impl crate::RegisterSpec for RsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RsrSpec {}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RsrSpec {}
