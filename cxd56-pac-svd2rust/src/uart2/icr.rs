///Register `ICR` writer
pub type W = crate::W<IcrSpec>;
///Field `RIMIC` writer - nUARTRI modem interrupt clear
pub type RimicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSMIC` writer - nUARTCTS modem interrupt clear
pub type CtsmicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCDMIC` writer - nUARTDCD modem interrupt clear
pub type DcdmicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSRMIC` writer - nUARTDSR modem interrupt clear
pub type DsrmicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXIC` writer - Receive interrupt clear
pub type RxicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXIC` writer - Transmit interrupt clear
pub type TxicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTIC` writer - Receive timeout interrupt clear
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEIC` writer - Framing error interrupt clear
pub type FeicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEIC` writer - Parity error interrupt clear
pub type PeicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BEIC` writer - Break error interrupt clear
pub type BeicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OEIC` writer - Overrun error interrupt clear
pub type OeicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    ///Bit 0 - nUARTRI modem interrupt clear
    #[inline(always)]
    pub fn rimic(&mut self) -> RimicW<'_, IcrSpec> {
        RimicW::new(self, 0)
    }
    ///Bit 1 - nUARTCTS modem interrupt clear
    #[inline(always)]
    pub fn ctsmic(&mut self) -> CtsmicW<'_, IcrSpec> {
        CtsmicW::new(self, 1)
    }
    ///Bit 2 - nUARTDCD modem interrupt clear
    #[inline(always)]
    pub fn dcdmic(&mut self) -> DcdmicW<'_, IcrSpec> {
        DcdmicW::new(self, 2)
    }
    ///Bit 3 - nUARTDSR modem interrupt clear
    #[inline(always)]
    pub fn dsrmic(&mut self) -> DsrmicW<'_, IcrSpec> {
        DsrmicW::new(self, 3)
    }
    ///Bit 4 - Receive interrupt clear
    #[inline(always)]
    pub fn rxic(&mut self) -> RxicW<'_, IcrSpec> {
        RxicW::new(self, 4)
    }
    ///Bit 5 - Transmit interrupt clear
    #[inline(always)]
    pub fn txic(&mut self) -> TxicW<'_, IcrSpec> {
        TxicW::new(self, 5)
    }
    ///Bit 6 - Receive timeout interrupt clear
    #[inline(always)]
    pub fn rtic(&mut self) -> RticW<'_, IcrSpec> {
        RticW::new(self, 6)
    }
    ///Bit 7 - Framing error interrupt clear
    #[inline(always)]
    pub fn feic(&mut self) -> FeicW<'_, IcrSpec> {
        FeicW::new(self, 7)
    }
    ///Bit 8 - Parity error interrupt clear
    #[inline(always)]
    pub fn peic(&mut self) -> PeicW<'_, IcrSpec> {
        PeicW::new(self, 8)
    }
    ///Bit 9 - Break error interrupt clear
    #[inline(always)]
    pub fn beic(&mut self) -> BeicW<'_, IcrSpec> {
        BeicW::new(self, 9)
    }
    ///Bit 10 - Overrun error interrupt clear
    #[inline(always)]
    pub fn oeic(&mut self) -> OeicW<'_, IcrSpec> {
        OeicW::new(self, 10)
    }
}
/**Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for IcrSpec {}
