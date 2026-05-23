///Register `SSPMIS` reader
pub type R = crate::R<SspmisSpec>;
/**Receive overrun interrupt mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rormis {
    ///0: Masked
    Masked = 0,
    ///1: Not masked
    Notmasked = 1,
}
impl From<Rormis> for bool {
    #[inline(always)]
    fn from(variant: Rormis) -> Self {
        variant as u8 != 0
    }
}
///Field `RORMIS` reader - Receive overrun interrupt mask
pub type RormisR = crate::BitReader<Rormis>;
impl RormisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rormis {
        match self.bits {
            false => Rormis::Masked,
            true => Rormis::Notmasked,
        }
    }
    ///Masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rormis::Masked
    }
    ///Not masked
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rormis::Notmasked
    }
}
/**Receive timeout interrupt mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtmis {
    ///0: Masked
    Masked = 0,
    ///1: Not masked
    Notmasked = 1,
}
impl From<Rtmis> for bool {
    #[inline(always)]
    fn from(variant: Rtmis) -> Self {
        variant as u8 != 0
    }
}
///Field `RTMIS` reader - Receive timeout interrupt mask
pub type RtmisR = crate::BitReader<Rtmis>;
impl RtmisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtmis {
        match self.bits {
            false => Rtmis::Masked,
            true => Rtmis::Notmasked,
        }
    }
    ///Masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rtmis::Masked
    }
    ///Not masked
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rtmis::Notmasked
    }
}
/**Receive FIFO interrupt mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxmis {
    ///0: Masked
    Masked = 0,
    ///1: Not masked
    Notmasked = 1,
}
impl From<Rxmis> for bool {
    #[inline(always)]
    fn from(variant: Rxmis) -> Self {
        variant as u8 != 0
    }
}
///Field `RXMIS` reader - Receive FIFO interrupt mask
pub type RxmisR = crate::BitReader<Rxmis>;
impl RxmisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxmis {
        match self.bits {
            false => Rxmis::Masked,
            true => Rxmis::Notmasked,
        }
    }
    ///Masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rxmis::Masked
    }
    ///Not masked
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rxmis::Notmasked
    }
}
/**Transmit FIFO interrupt mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txmis {
    ///0: Masked
    Masked = 0,
    ///1: Not masked
    Notmasked = 1,
}
impl From<Txmis> for bool {
    #[inline(always)]
    fn from(variant: Txmis) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMIS` reader - Transmit FIFO interrupt mask
pub type TxmisR = crate::BitReader<Txmis>;
impl TxmisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txmis {
        match self.bits {
            false => Txmis::Masked,
            true => Txmis::Notmasked,
        }
    }
    ///Masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Txmis::Masked
    }
    ///Not masked
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Txmis::Notmasked
    }
}
impl R {
    ///Bit 0 - Receive overrun interrupt mask
    #[inline(always)]
    pub fn rormis(&self) -> RormisR {
        RormisR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receive timeout interrupt mask
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive FIFO interrupt mask
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit FIFO interrupt mask
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 3) & 1) != 0)
    }
}
/**Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`sspmis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SspmisSpec;
impl crate::RegisterSpec for SspmisSpec {
    type Ux = u32;
}
///`read()` method returns [`sspmis::R`](R) reader structure
impl crate::Readable for SspmisSpec {}
///`reset()` method sets SSPMIS to value 0
impl crate::Resettable for SspmisSpec {}
