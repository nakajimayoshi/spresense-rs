///Register `SSPRIS` reader
pub type R = crate::R<SsprisSpec>;
/**Receive overrun interrupt mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rorris {
    ///0: Masked
    Masked = 0,
    ///1: Not masked
    Notmasked = 1,
}
impl From<Rorris> for bool {
    #[inline(always)]
    fn from(variant: Rorris) -> Self {
        variant as u8 != 0
    }
}
///Field `RORRIS` reader - Receive overrun interrupt mask
pub type RorrisR = crate::BitReader<Rorris>;
impl RorrisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rorris {
        match self.bits {
            false => Rorris::Masked,
            true => Rorris::Notmasked,
        }
    }
    ///Masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rorris::Masked
    }
    ///Not masked
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rorris::Notmasked
    }
}
/**Receive timeout interrupt mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtris {
    ///0: Masked
    Masked = 0,
    ///1: Not masked
    Notmasked = 1,
}
impl From<Rtris> for bool {
    #[inline(always)]
    fn from(variant: Rtris) -> Self {
        variant as u8 != 0
    }
}
///Field `RTRIS` reader - Receive timeout interrupt mask
pub type RtrisR = crate::BitReader<Rtris>;
impl RtrisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtris {
        match self.bits {
            false => Rtris::Masked,
            true => Rtris::Notmasked,
        }
    }
    ///Masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rtris::Masked
    }
    ///Not masked
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rtris::Notmasked
    }
}
/**Receive FIFO interrupt mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxris {
    ///0: Masked
    Masked = 0,
    ///1: Not masked
    Notmasked = 1,
}
impl From<Rxris> for bool {
    #[inline(always)]
    fn from(variant: Rxris) -> Self {
        variant as u8 != 0
    }
}
///Field `RXRIS` reader - Receive FIFO interrupt mask
pub type RxrisR = crate::BitReader<Rxris>;
impl RxrisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxris {
        match self.bits {
            false => Rxris::Masked,
            true => Rxris::Notmasked,
        }
    }
    ///Masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rxris::Masked
    }
    ///Not masked
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rxris::Notmasked
    }
}
/**Transmit FIFO interrupt mask

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txris {
    ///0: Masked
    Masked = 0,
    ///1: Not masked
    Notmasked = 1,
}
impl From<Txris> for bool {
    #[inline(always)]
    fn from(variant: Txris) -> Self {
        variant as u8 != 0
    }
}
///Field `TXRIS` reader - Transmit FIFO interrupt mask
pub type TxrisR = crate::BitReader<Txris>;
impl TxrisR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txris {
        match self.bits {
            false => Txris::Masked,
            true => Txris::Notmasked,
        }
    }
    ///Masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Txris::Masked
    }
    ///Not masked
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Txris::Notmasked
    }
}
impl R {
    ///Bit 0 - Receive overrun interrupt mask
    #[inline(always)]
    pub fn rorris(&self) -> RorrisR {
        RorrisR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receive timeout interrupt mask
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive FIFO interrupt mask
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit FIFO interrupt mask
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 3) & 1) != 0)
    }
}
/**Raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`sspris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SsprisSpec;
impl crate::RegisterSpec for SsprisSpec {
    type Ux = u32;
}
///`read()` method returns [`sspris::R`](R) reader structure
impl crate::Readable for SsprisSpec {}
///`reset()` method sets SSPRIS to value 0
impl crate::Resettable for SsprisSpec {}
