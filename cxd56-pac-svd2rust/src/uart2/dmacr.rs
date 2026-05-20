#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DmacrSpec>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DmacrSpec>;
#[doc = "Receive DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmae {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Rxdmae> for bool {
    #[inline(always)]
    fn from(variant: Rxdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAE` reader - Receive DMA enable"]
pub type RxdmaeR = crate::BitReader<Rxdmae>;
impl RxdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmae {
        match self.bits {
            false => Rxdmae::Disabled,
            true => Rxdmae::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdmae::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdmae::Enabled
    }
}
#[doc = "Field `RXDMAE` writer - Receive DMA enable"]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG, Rxdmae>;
impl<'a, REG> RxdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmae::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmae::Enabled)
    }
}
#[doc = "Transmit DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmae {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Txdmae> for bool {
    #[inline(always)]
    fn from(variant: Txdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAE` reader - Transmit DMA enable"]
pub type TxdmaeR = crate::BitReader<Txdmae>;
impl TxdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmae {
        match self.bits {
            false => Txdmae::Disabled,
            true => Txdmae::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdmae::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdmae::Enabled
    }
}
#[doc = "Field `TXDMAE` writer - Transmit DMA enable"]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG, Txdmae>;
impl<'a, REG> TxdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmae::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmae::Enabled)
    }
}
#[doc = "Field `DMAONERR` reader - DMA on error"]
pub type DmaonerrR = crate::BitReader;
#[doc = "Field `DMAONERR` writer - DMA on error"]
pub type DmaonerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA on error"]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DmaonerrR {
        DmaonerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RxdmaeW<'_, DmacrSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TxdmaeW<'_, DmacrSpec> {
        TxdmaeW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA on error"]
    #[inline(always)]
    pub fn dmaonerr(&mut self) -> DmaonerrW<'_, DmacrSpec> {
        DmaonerrW::new(self, 2)
    }
}
#[doc = "DMA Control Regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrSpec;
impl crate::RegisterSpec for DmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DmacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DmacrSpec {}
