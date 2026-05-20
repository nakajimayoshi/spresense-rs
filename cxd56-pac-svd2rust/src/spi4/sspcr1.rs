#[doc = "Register `SSPCR1` reader"]
pub type R = crate::R<Sspcr1Spec>;
#[doc = "Register `SSPCR1` writer"]
pub type W = crate::W<Sspcr1Spec>;
#[doc = "Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbm {
    #[doc = "0: Normal serial port operation enabled"]
    Disabled = 0,
    #[doc = "1: Output of transmit serial shifter is connected to input of recieve serial shifter internally"]
    Enabled = 1,
}
impl From<Lbm> for bool {
    #[inline(always)]
    fn from(variant: Lbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - Loop back mode"]
pub type LbmR = crate::BitReader<Lbm>;
impl LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbm {
        match self.bits {
            false => Lbm::Disabled,
            true => Lbm::Enabled,
        }
    }
    #[doc = "Normal serial port operation enabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lbm::Disabled
    }
    #[doc = "Output of transmit serial shifter is connected to input of recieve serial shifter internally"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lbm::Enabled
    }
}
#[doc = "Field `LBM` writer - Loop back mode"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG, Lbm>;
impl<'a, REG> LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal serial port operation enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Disabled)
    }
    #[doc = "Output of transmit serial shifter is connected to input of recieve serial shifter internally"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Enabled)
    }
}
#[doc = "Synchronous serial port enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sse {
    #[doc = "0: SSP operation disabled"]
    Disabled = 0,
    #[doc = "1: SSP operation enabled"]
    Enabled = 1,
}
impl From<Sse> for bool {
    #[inline(always)]
    fn from(variant: Sse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSE` reader - Synchronous serial port enable"]
pub type SseR = crate::BitReader<Sse>;
impl SseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sse {
        match self.bits {
            false => Sse::Disabled,
            true => Sse::Enabled,
        }
    }
    #[doc = "SSP operation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sse::Disabled
    }
    #[doc = "SSP operation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sse::Enabled
    }
}
#[doc = "Field `SSE` writer - Synchronous serial port enable"]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG, Sse>;
impl<'a, REG> SseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSP operation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::Disabled)
    }
    #[doc = "SSP operation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sse::Enabled)
    }
}
#[doc = "Master or slave mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ms {
    #[doc = "0: Master mode"]
    Master = 0,
    #[doc = "1: Slave mode"]
    Slave = 1,
}
impl From<Ms> for bool {
    #[inline(always)]
    fn from(variant: Ms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - Master or slave mode select"]
pub type MsR = crate::BitReader<Ms>;
impl MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ms {
        match self.bits {
            false => Ms::Master,
            true => Ms::Slave,
        }
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Ms::Master
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Ms::Slave
    }
}
#[doc = "Field `MS` writer - Master or slave mode select"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG, Ms>;
impl<'a, REG> MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Master)
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Slave)
    }
}
#[doc = "Slave-mode output disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sod {
    #[doc = "0: SSP can drive the SSPTXD output in slave mode"]
    Enable = 0,
    #[doc = "1: SSP must not drive the SSPTXD output in slave mode"]
    Disable = 1,
}
impl From<Sod> for bool {
    #[inline(always)]
    fn from(variant: Sod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOD` reader - Slave-mode output disable"]
pub type SodR = crate::BitReader<Sod>;
impl SodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sod {
        match self.bits {
            false => Sod::Enable,
            true => Sod::Disable,
        }
    }
    #[doc = "SSP can drive the SSPTXD output in slave mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sod::Enable
    }
    #[doc = "SSP must not drive the SSPTXD output in slave mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sod::Disable
    }
}
#[doc = "Field `SOD` writer - Slave-mode output disable"]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG, Sod>;
impl<'a, REG> SodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSP can drive the SSPTXD output in slave mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sod::Enable)
    }
    #[doc = "SSP must not drive the SSPTXD output in slave mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sod::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - Loop back mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronous serial port enable"]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master or slave mode select"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave-mode output disable"]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Loop back mode"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<'_, Sspcr1Spec> {
        LbmW::new(self, 0)
    }
    #[doc = "Bit 1 - Synchronous serial port enable"]
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<'_, Sspcr1Spec> {
        SseW::new(self, 1)
    }
    #[doc = "Bit 2 - Master or slave mode select"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, Sspcr1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave-mode output disable"]
    #[inline(always)]
    pub fn sod(&mut self) -> SodW<'_, Sspcr1Spec> {
        SodW::new(self, 3)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sspcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sspcr1Spec;
impl crate::RegisterSpec for Sspcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspcr1::R`](R) reader structure"]
impl crate::Readable for Sspcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sspcr1::W`](W) writer structure"]
impl crate::Writable for Sspcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSPCR1 to value 0"]
impl crate::Resettable for Sspcr1Spec {}
