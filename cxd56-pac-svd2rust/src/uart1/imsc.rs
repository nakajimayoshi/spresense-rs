#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RIMIM` reader - nUARTRI modem interrupt mask"]
pub type RimimR = crate::BitReader;
#[doc = "Field `RIMIM` writer - nUARTRI modem interrupt mask"]
pub type RimimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIM` reader - nUARTCTS modem interrupt mask"]
pub type CtsmimR = crate::BitReader;
#[doc = "Field `CTSMIM` writer - nUARTCTS modem interrupt mask"]
pub type CtsmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMIM` reader - nUARTDCD modem interrupt mask"]
pub type DcdmimR = crate::BitReader;
#[doc = "Field `DCDMIM` writer - nUARTDCD modem interrupt mask"]
pub type DcdmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMIM` reader - nUARTDSR modem interrupt mask"]
pub type DsrmimR = crate::BitReader;
#[doc = "Field `DSRMIM` writer - nUARTDSR modem interrupt mask"]
pub type DsrmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - Receive interrupt mask"]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - Receive interrupt mask"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - Transmit interrupt mask"]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - Transmit interrupt mask"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask"]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIM` reader - Framing error interrupt mask"]
pub type FeimR = crate::BitReader;
#[doc = "Field `FEIM` writer - Framing error interrupt mask"]
pub type FeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIM` reader - Parity error interrupt mask"]
pub type PeimR = crate::BitReader;
#[doc = "Field `PEIM` writer - Parity error interrupt mask"]
pub type PeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIM` reader - Break error interrupt mask"]
pub type BeimR = crate::BitReader;
#[doc = "Field `BEIM` writer - Break error interrupt mask"]
pub type BeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIM` reader - Overrun error interrupt mask"]
pub type OeimR = crate::BitReader;
#[doc = "Field `OEIM` writer - Overrun error interrupt mask"]
pub type OeimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&self) -> RimimR {
        RimimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask"]
    #[inline(always)]
    pub fn ctsmim(&self) -> CtsmimR {
        CtsmimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DcdmimR {
        DcdmimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DsrmimR {
        DsrmimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&self) -> FeimR {
        FeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&self) -> PeimR {
        PeimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&self) -> BeimR {
        BeimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&self) -> OeimR {
        OeimR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&mut self) -> RimimW<'_, ImscSpec> {
        RimimW::new(self, 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask"]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CtsmimW<'_, ImscSpec> {
        CtsmimW::new(self, 1)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DcdmimW<'_, ImscSpec> {
        DcdmimW::new(self, 2)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DsrmimW<'_, ImscSpec> {
        DsrmimW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RximW<'_, ImscSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TximW<'_, ImscSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RtimW<'_, ImscSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn feim(&mut self) -> FeimW<'_, ImscSpec> {
        FeimW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn peim(&mut self) -> PeimW<'_, ImscSpec> {
        PeimW::new(self, 8)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn beim(&mut self) -> BeimW<'_, ImscSpec> {
        BeimW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oeim(&mut self) -> OeimW<'_, ImscSpec> {
        OeimW::new(self, 10)
    }
}
#[doc = "Interrupt Mask Set and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscSpec;
impl crate::RegisterSpec for ImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for ImscSpec {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for ImscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for ImscSpec {}
