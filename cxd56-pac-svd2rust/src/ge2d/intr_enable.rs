#[doc = "Register `INTR_ENABLE` reader"]
pub type R = crate::R<IntrEnableSpec>;
#[doc = "Register `INTR_ENABLE` writer"]
pub type W = crate::W<IntrEnableSpec>;
#[doc = "Field `HPU` reader - Normal Descriptor Command Update Interrupt"]
pub type HpuR = crate::BitReader;
#[doc = "Field `HPU` writer - Normal Descriptor Command Update Interrupt"]
pub type HpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDF` reader - Normal Descriptor Command Finished Interrupt"]
pub type NdfR = crate::BitReader;
#[doc = "Field `NDF` writer - Normal Descriptor Command Finished Interrupt"]
pub type NdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDB` reader - Normal Descriptor Command Branch Interrupt"]
pub type NdbR = crate::BitReader;
#[doc = "Field `NDB` writer - Normal Descriptor Command Branch Interrupt"]
pub type NdbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDE` reader - Normal Descriptor Command Error Interrupt"]
pub type NdeR = crate::BitReader;
#[doc = "Field `NDE` writer - Normal Descriptor Command Error Interrupt"]
pub type NdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSD` reader - Descriptor STOP Command Done Interrupt"]
pub type DsdR = crate::BitReader;
#[doc = "Field `DSD` writer - Descriptor STOP Command Done Interrupt"]
pub type DsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_ERR` reader - Read Error Interrupt"]
pub type RdErrR = crate::BitReader;
#[doc = "Field `RD_ERR` writer - Read Error Interrupt"]
pub type RdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_ERR` reader - Write Error Interrupt"]
pub type WrErrR = crate::BitReader;
#[doc = "Field `WR_ERR` writer - Write Error Interrupt"]
pub type WrErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Normal Descriptor Command Update Interrupt"]
    #[inline(always)]
    pub fn hpu(&self) -> HpuR {
        HpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Normal Descriptor Command Finished Interrupt"]
    #[inline(always)]
    pub fn ndf(&self) -> NdfR {
        NdfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Normal Descriptor Command Branch Interrupt"]
    #[inline(always)]
    pub fn ndb(&self) -> NdbR {
        NdbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Normal Descriptor Command Error Interrupt"]
    #[inline(always)]
    pub fn nde(&self) -> NdeR {
        NdeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Descriptor STOP Command Done Interrupt"]
    #[inline(always)]
    pub fn dsd(&self) -> DsdR {
        DsdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Read Error Interrupt"]
    #[inline(always)]
    pub fn rd_err(&self) -> RdErrR {
        RdErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write Error Interrupt"]
    #[inline(always)]
    pub fn wr_err(&self) -> WrErrR {
        WrErrR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Normal Descriptor Command Update Interrupt"]
    #[inline(always)]
    pub fn hpu(&mut self) -> HpuW<'_, IntrEnableSpec> {
        HpuW::new(self, 0)
    }
    #[doc = "Bit 1 - Normal Descriptor Command Finished Interrupt"]
    #[inline(always)]
    pub fn ndf(&mut self) -> NdfW<'_, IntrEnableSpec> {
        NdfW::new(self, 1)
    }
    #[doc = "Bit 2 - Normal Descriptor Command Branch Interrupt"]
    #[inline(always)]
    pub fn ndb(&mut self) -> NdbW<'_, IntrEnableSpec> {
        NdbW::new(self, 2)
    }
    #[doc = "Bit 3 - Normal Descriptor Command Error Interrupt"]
    #[inline(always)]
    pub fn nde(&mut self) -> NdeW<'_, IntrEnableSpec> {
        NdeW::new(self, 3)
    }
    #[doc = "Bit 8 - Descriptor STOP Command Done Interrupt"]
    #[inline(always)]
    pub fn dsd(&mut self) -> DsdW<'_, IntrEnableSpec> {
        DsdW::new(self, 8)
    }
    #[doc = "Bit 16 - Read Error Interrupt"]
    #[inline(always)]
    pub fn rd_err(&mut self) -> RdErrW<'_, IntrEnableSpec> {
        RdErrW::new(self, 16)
    }
    #[doc = "Bit 17 - Write Error Interrupt"]
    #[inline(always)]
    pub fn wr_err(&mut self) -> WrErrW<'_, IntrEnableSpec> {
        WrErrW::new(self, 17)
    }
}
#[doc = "2D Graphics Engine Interrupt Control\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEnableSpec;
impl crate::RegisterSpec for IntrEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_enable::R`](R) reader structure"]
impl crate::Readable for IntrEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_enable::W`](W) writer structure"]
impl crate::Writable for IntrEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_ENABLE to value 0"]
impl crate::Resettable for IntrEnableSpec {}
