#[doc = "Register `SSPIMSC` reader"]
pub type R = crate::R<SspimscSpec>;
#[doc = "Register `SSPIMSC` writer"]
pub type W = crate::W<SspimscSpec>;
#[doc = "Receive overrun interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rorim {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Not masked"]
    Notmasked = 1,
}
impl From<Rorim> for bool {
    #[inline(always)]
    fn from(variant: Rorim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RORIM` reader - Receive overrun interrupt mask"]
pub type RorimR = crate::BitReader<Rorim>;
impl RorimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rorim {
        match self.bits {
            false => Rorim::Masked,
            true => Rorim::Notmasked,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rorim::Masked
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rorim::Notmasked
    }
}
#[doc = "Field `RORIM` writer - Receive overrun interrupt mask"]
pub type RorimW<'a, REG> = crate::BitWriter<'a, REG, Rorim>;
impl<'a, REG> RorimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Rorim::Masked)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn notmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Rorim::Notmasked)
    }
}
#[doc = "Receive timeout interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtim {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Not masked"]
    Notmasked = 1,
}
impl From<Rtim> for bool {
    #[inline(always)]
    fn from(variant: Rtim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask"]
pub type RtimR = crate::BitReader<Rtim>;
impl RtimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtim {
        match self.bits {
            false => Rtim::Masked,
            true => Rtim::Notmasked,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rtim::Masked
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rtim::Notmasked
    }
}
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG, Rtim>;
impl<'a, REG> RtimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Rtim::Masked)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn notmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Rtim::Notmasked)
    }
}
#[doc = "Receive FIFO interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxim {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Not masked"]
    Notmasked = 1,
}
impl From<Rxim> for bool {
    #[inline(always)]
    fn from(variant: Rxim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIM` reader - Receive FIFO interrupt mask"]
pub type RximR = crate::BitReader<Rxim>;
impl RximR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxim {
        match self.bits {
            false => Rxim::Masked,
            true => Rxim::Notmasked,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Rxim::Masked
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Rxim::Notmasked
    }
}
#[doc = "Field `RXIM` writer - Receive FIFO interrupt mask"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG, Rxim>;
impl<'a, REG> RximW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Rxim::Masked)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn notmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Rxim::Notmasked)
    }
}
#[doc = "Transmit FIFO interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txim {
    #[doc = "0: Masked"]
    Masked = 0,
    #[doc = "1: Not masked"]
    Notmasked = 1,
}
impl From<Txim> for bool {
    #[inline(always)]
    fn from(variant: Txim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIM` reader - Transmit FIFO interrupt mask"]
pub type TximR = crate::BitReader<Txim>;
impl TximR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txim {
        match self.bits {
            false => Txim::Masked,
            true => Txim::Notmasked,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Txim::Masked
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn is_notmasked(&self) -> bool {
        *self == Txim::Notmasked
    }
}
#[doc = "Field `TXIM` writer - Transmit FIFO interrupt mask"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG, Txim>;
impl<'a, REG> TximW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Txim::Masked)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn notmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Txim::Notmasked)
    }
}
impl R {
    #[doc = "Bit 0 - Receive overrun interrupt mask"]
    #[inline(always)]
    pub fn rorim(&self) -> RorimR {
        RorimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive overrun interrupt mask"]
    #[inline(always)]
    pub fn rorim(&mut self) -> RorimW<'_, SspimscSpec> {
        RorimW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RtimW<'_, SspimscSpec> {
        RtimW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RximW<'_, SspimscSpec> {
        RximW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TximW<'_, SspimscSpec> {
        TximW::new(self, 3)
    }
}
#[doc = "Interrupt mask set or clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspimsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspimsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspimscSpec;
impl crate::RegisterSpec for SspimscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspimsc::R`](R) reader structure"]
impl crate::Readable for SspimscSpec {}
#[doc = "`write(|w| ..)` method takes [`sspimsc::W`](W) writer structure"]
impl crate::Writable for SspimscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSPIMSC to value 0"]
impl crate::Resettable for SspimscSpec {}
