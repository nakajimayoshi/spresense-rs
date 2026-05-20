#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Field `CTS` reader - Clear to send"]
pub type CtsR = crate::BitReader;
#[doc = "Field `DSR` reader - Data set ready"]
pub type DsrR = crate::BitReader;
#[doc = "Field `DCD` reader - Data carrier detect"]
pub type DcdR = crate::BitReader;
#[doc = "Field `BUSY` reader - UART busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `RXFE` reader - Receive FIFO empty"]
pub type RxfeR = crate::BitReader;
#[doc = "Field `TXFF` reader - Transmit FIFO full"]
pub type TxffR = crate::BitReader;
#[doc = "Field `RXFF` reader - Receive FIFO full"]
pub type RxffR = crate::BitReader;
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `RI` reader - Ring Indicator"]
pub type RiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear to send"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data set ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data carrier detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ring Indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FrSpec {}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FrSpec {}
