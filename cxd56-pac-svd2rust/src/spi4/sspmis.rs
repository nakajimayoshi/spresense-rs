#[doc = "Register `SSPMIS` reader"]
pub type R = crate::R<SspmisSpec>;
#[doc = "Receive overrun interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rormis {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Not masked"]
    Notmasked = 1,
}
impl From<Rormis> for bool {
    #[inline(always)]
    fn from(variant: Rormis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RORMIS` reader - Receive overrun interrupt mask"]
pub type RormisR = crate::BitReader<Rormis>;
impl RormisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rormis {
        match self.bits {
            false => Rormis::Masked,
            true => Rormis::Notmasked,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rormis::Masked
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rormis::Notmasked
    }
}
#[doc = "Receive timeout interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtmis {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Not masked"]
    Notmasked = 1,
}
impl From<Rtmis> for bool {
    #[inline(always)]
    fn from(variant: Rtmis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTMIS` reader - Receive timeout interrupt mask"]
pub type RtmisR = crate::BitReader<Rtmis>;
impl RtmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtmis {
        match self.bits {
            false => Rtmis::Masked,
            true => Rtmis::Notmasked,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rtmis::Masked
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rtmis::Notmasked
    }
}
#[doc = "Receive FIFO interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxmis {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Not masked"]
    Notmasked = 1,
}
impl From<Rxmis> for bool {
    #[inline(always)]
    fn from(variant: Rxmis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXMIS` reader - Receive FIFO interrupt mask"]
pub type RxmisR = crate::BitReader<Rxmis>;
impl RxmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxmis {
        match self.bits {
            false => Rxmis::Masked,
            true => Rxmis::Notmasked,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rxmis::Masked
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rxmis::Notmasked
    }
}
#[doc = "Transmit FIFO interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txmis {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Not masked"]
    Notmasked = 1,
}
impl From<Txmis> for bool {
    #[inline(always)]
    fn from(variant: Txmis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXMIS` reader - Transmit FIFO interrupt mask"]
pub type TxmisR = crate::BitReader<Txmis>;
impl TxmisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txmis {
        match self.bits {
            false => Txmis::Masked,
            true => Txmis::Notmasked,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Txmis::Masked
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Txmis::Notmasked
    }
}
impl R {
    #[doc = "Bit 0 - Receive overrun interrupt mask"]
    #[inline(always)]
    pub fn rormis(&self) -> RormisR {
        RormisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask"]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask"]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspmis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspmisSpec;
impl crate::RegisterSpec for SspmisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspmis::R`](R) reader structure"]
impl crate::Readable for SspmisSpec {}
#[doc = "`reset()` method sets SSPMIS to value 0"]
impl crate::Resettable for SspmisSpec {}
