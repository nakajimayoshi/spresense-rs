///Register `FR` reader
pub type R = crate::R<FrSpec>;
///Field `CTS` reader - Clear to send
pub type CtsR = crate::BitReader;
///Field `DSR` reader - Data set ready
pub type DsrR = crate::BitReader;
///Field `DCD` reader - Data carrier detect
pub type DcdR = crate::BitReader;
///Field `BUSY` reader - UART busy
pub type BusyR = crate::BitReader;
///Field `RXFE` reader - Receive FIFO empty
pub type RxfeR = crate::BitReader;
///Field `TXFF` reader - Transmit FIFO full
pub type TxffR = crate::BitReader;
///Field `RXFF` reader - Receive FIFO full
pub type RxffR = crate::BitReader;
///Field `TXFE` reader - Transmit FIFO empty
pub type TxfeR = crate::BitReader;
///Field `RI` reader - Ring Indicator
pub type RiR = crate::BitReader;
impl R {
    ///Bit 0 - Clear to send
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data set ready
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Data carrier detect
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - UART busy
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive FIFO empty
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit FIFO full
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive FIFO full
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit FIFO empty
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Ring Indicator
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 8) & 1) != 0)
    }
}
/**Flags Register

You can [`read`](crate::Reg::read) this register and get [`fr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
///`read()` method returns [`fr::R`](R) reader structure
impl crate::Readable for FrSpec {}
///`reset()` method sets FR to value 0
impl crate::Resettable for FrSpec {}
