///Register `RSR` reader
pub type R = crate::R<RsrSpec>;
/**Framing Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfe {
    ///0: no error
    None = 0,
    ///1: error
    Error = 1,
}
impl From<Rfe> for bool {
    #[inline(always)]
    fn from(variant: Rfe) -> Self {
        variant as u8 != 0
    }
}
///Field `RFE` reader - Framing Error
pub type RfeR = crate::BitReader<Rfe>;
impl RfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rfe {
        match self.bits {
            false => Rfe::None,
            true => Rfe::Error,
        }
    }
    ///no error
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rfe::None
    }
    ///error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Rfe::Error
    }
}
/**Parity Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpe {
    ///0: no error
    None = 0,
    ///1: error
    Error = 1,
}
impl From<Rpe> for bool {
    #[inline(always)]
    fn from(variant: Rpe) -> Self {
        variant as u8 != 0
    }
}
///Field `RPE` reader - Parity Error
pub type RpeR = crate::BitReader<Rpe>;
impl RpeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rpe {
        match self.bits {
            false => Rpe::None,
            true => Rpe::Error,
        }
    }
    ///no error
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rpe::None
    }
    ///error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Rpe::Error
    }
}
/**Break Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbe {
    ///0: no error
    None = 0,
    ///1: error
    Error = 1,
}
impl From<Rbe> for bool {
    #[inline(always)]
    fn from(variant: Rbe) -> Self {
        variant as u8 != 0
    }
}
///Field `RBE` reader - Break Error
pub type RbeR = crate::BitReader<Rbe>;
impl RbeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rbe {
        match self.bits {
            false => Rbe::None,
            true => Rbe::Error,
        }
    }
    ///no error
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rbe::None
    }
    ///error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Rbe::Error
    }
}
/**Overrun Error

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Roe {
    ///0: no error
    None = 0,
    ///1: error
    Error = 1,
}
impl From<Roe> for bool {
    #[inline(always)]
    fn from(variant: Roe) -> Self {
        variant as u8 != 0
    }
}
///Field `ROE` reader - Overrun Error
pub type RoeR = crate::BitReader<Roe>;
impl RoeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Roe {
        match self.bits {
            false => Roe::None,
            true => Roe::Error,
        }
    }
    ///no error
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Roe::None
    }
    ///error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Roe::Error
    }
}
impl R {
    ///Bit 0 - Framing Error
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Parity Error
    #[inline(always)]
    pub fn rpe(&self) -> RpeR {
        RpeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Break Error
    #[inline(always)]
    pub fn rbe(&self) -> RbeR {
        RbeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun Error
    #[inline(always)]
    pub fn roe(&self) -> RoeR {
        RoeR::new(((self.bits >> 3) & 1) != 0)
    }
}
/**Receive Status and Clear Register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RsrSpec;
impl crate::RegisterSpec for RsrSpec {
    type Ux = u32;
}
///`read()` method returns [`rsr::R`](R) reader structure
impl crate::Readable for RsrSpec {}
///`reset()` method sets RSR to value 0
impl crate::Resettable for RsrSpec {}
