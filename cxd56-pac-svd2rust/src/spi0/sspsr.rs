#[doc = "Register `SSPSR` reader"]
pub type R = crate::R<SspsrSpec>;
#[doc = "Transmit FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfe {
    #[doc = "0: Transmit FIFO is not empty"]
    Notempty = 0,
    #[doc = "1: Transmit FIFO is empty"]
    Empty = 1,
}
impl From<Tfe> for bool {
    #[inline(always)]
    fn from(variant: Tfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO empty"]
pub type TfeR = crate::BitReader<Tfe>;
impl TfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfe {
        match self.bits {
            false => Tfe::Notempty,
            true => Tfe::Empty,
        }
    }
    #[doc = "Transmit FIFO is not empty"]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == Tfe::Notempty
    }
    #[doc = "Transmit FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Tfe::Empty
    }
}
#[doc = "Transmit FIFO not full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnf {
    #[doc = "0: Transmit FIFO is full"]
    Full = 0,
    #[doc = "1: Transmit FIFO is not full"]
    Notfull = 1,
}
impl From<Tnf> for bool {
    #[inline(always)]
    fn from(variant: Tnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNF` reader - Transmit FIFO not full"]
pub type TnfR = crate::BitReader<Tnf>;
impl TnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnf {
        match self.bits {
            false => Tnf::Full,
            true => Tnf::Notfull,
        }
    }
    #[doc = "Transmit FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Tnf::Full
    }
    #[doc = "Transmit FIFO is not full"]
    #[inline(always)]
    pub fn is_notfull(&self) -> bool {
        *self == Tnf::Notfull
    }
}
#[doc = "Rceive FIFIO not empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rne {
    #[doc = "0: Receive FIFO is empty"]
    Empty = 0,
    #[doc = "1: Receive FIFO is not empty"]
    Notempty = 1,
}
impl From<Rne> for bool {
    #[inline(always)]
    fn from(variant: Rne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNE` reader - Rceive FIFIO not empty"]
pub type RneR = crate::BitReader<Rne>;
impl RneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rne {
        match self.bits {
            false => Rne::Empty,
            true => Rne::Notempty,
        }
    }
    #[doc = "Receive FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rne::Empty
    }
    #[doc = "Receive FIFO is not empty"]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == Rne::Notempty
    }
}
#[doc = "Receive FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rff {
    #[doc = "0: Receive FIFO is not full"]
    Notfull = 0,
    #[doc = "1: Recieve FIFO is full"]
    Full = 1,
}
impl From<Rff> for bool {
    #[inline(always)]
    fn from(variant: Rff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO full"]
pub type RffR = crate::BitReader<Rff>;
impl RffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rff {
        match self.bits {
            false => Rff::Notfull,
            true => Rff::Full,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_notfull(&self) -> bool {
        *self == Rff::Notfull
    }
    #[doc = "Recieve FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rff::Full
    }
}
#[doc = "SSP busy flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsy {
    #[doc = "0: SSP is idle"]
    Idle = 0,
    #[doc = "1: SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty"]
    Busy = 1,
}
impl From<Bsy> for bool {
    #[inline(always)]
    fn from(variant: Bsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - SSP busy flag"]
pub type BsyR = crate::BitReader<Bsy>;
impl BsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsy {
        match self.bits {
            false => Bsy::Idle,
            true => Bsy::Busy,
        }
    }
    #[doc = "SSP is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Bsy::Idle
    }
    #[doc = "SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Bsy::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full"]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rceive FIFIO not empty"]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO full"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSP busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspsrSpec;
impl crate::RegisterSpec for SspsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspsr::R`](R) reader structure"]
impl crate::Readable for SspsrSpec {}
#[doc = "`reset()` method sets SSPSR to value 0"]
impl crate::Resettable for SspsrSpec {}
