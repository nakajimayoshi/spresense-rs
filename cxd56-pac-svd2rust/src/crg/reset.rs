///Register `RESET` reader
pub type R = crate::R<ResetSpec>;
///Register `RESET` writer
pub type W = crate::W<ResetSpec>;
/**0=reset, 1=active

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrsAud {
    ///0: Reset
    Reset = 0,
    ///1: Active
    Active = 1,
}
impl From<XrsAud> for bool {
    #[inline(always)]
    fn from(variant: XrsAud) -> Self {
        variant as u8 != 0
    }
}
///Field `XRS_AUD` reader - 0=reset, 1=active
pub type XrsAudR = crate::BitReader<XrsAud>;
impl XrsAudR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> XrsAud {
        match self.bits {
            false => XrsAud::Reset,
            true => XrsAud::Active,
        }
    }
    ///Reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == XrsAud::Reset
    }
    ///Active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == XrsAud::Active
    }
}
///Field `XRS_AUD` writer - 0=reset, 1=active
pub type XrsAudW<'a, REG> = crate::BitWriter<'a, REG, XrsAud>;
impl<'a, REG> XrsAudW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(XrsAud::Reset)
    }
    ///Active
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(XrsAud::Active)
    }
}
///0=reset, 1=active
pub use XrsAud as XrsImg;
///Field `XRS_IMG` reader - 0=reset, 1=active
pub use XrsAudR as XrsImgR;
///Field `XRS_IMG` writer - 0=reset, 1=active
pub use XrsAudW as XrsImgW;
///0=reset, 1=active
pub use XrsAud as XrsUsb;
///Field `XRS_USB` reader - 0=reset, 1=active
pub use XrsAudR as XrsUsbR;
///Field `XRS_USB` writer - 0=reset, 1=active
pub use XrsAudW as XrsUsbW;
///0=reset, 1=active
pub use XrsAud as XrsSdio;
///Field `XRS_SDIO` reader - 0=reset, 1=active
pub use XrsAudR as XrsSdioR;
///Field `XRS_SDIO` writer - 0=reset, 1=active
pub use XrsAudW as XrsSdioW;
///0=reset, 1=active
pub use XrsAud as XrsMmc;
///Field `XRS_MMC` reader - 0=reset, 1=active
pub use XrsAudR as XrsMmcR;
///Field `XRS_MMC` writer - 0=reset, 1=active
pub use XrsAudW as XrsMmcW;
///0=reset, 1=active
pub use XrsAud as XrsMmcCrg;
///Field `XRS_MMC_CRG` reader - 0=reset, 1=active
pub use XrsAudR as XrsMmcCrgR;
///Field `XRS_MMC_CRG` writer - 0=reset, 1=active
pub use XrsAudW as XrsMmcCrgW;
///0=reset, 1=active
pub use XrsAud as XrsDsp0;
///Field `XRS_DSP0` reader - 0=reset, 1=active
pub use XrsAudR as XrsDsp0R;
///Field `XRS_DSP0` writer - 0=reset, 1=active
pub use XrsAudW as XrsDsp0W;
///0=reset, 1=active
pub use XrsAud as XrsDsp1;
///Field `XRS_DSP1` reader - 0=reset, 1=active
pub use XrsAudR as XrsDsp1R;
///Field `XRS_DSP1` writer - 0=reset, 1=active
pub use XrsAudW as XrsDsp1W;
///0=reset, 1=active
pub use XrsAud as XrsDsp2;
///Field `XRS_DSP2` reader - 0=reset, 1=active
pub use XrsAudR as XrsDsp2R;
///Field `XRS_DSP2` writer - 0=reset, 1=active
pub use XrsAudW as XrsDsp2W;
///0=reset, 1=active
pub use XrsAud as XrsDsp3;
///Field `XRS_DSP3` reader - 0=reset, 1=active
pub use XrsAudR as XrsDsp3R;
///Field `XRS_DSP3` writer - 0=reset, 1=active
pub use XrsAudW as XrsDsp3W;
///0=reset, 1=active
pub use XrsAud as XrsDsp4;
///Field `XRS_DSP4` reader - 0=reset, 1=active
pub use XrsAudR as XrsDsp4R;
///Field `XRS_DSP4` writer - 0=reset, 1=active
pub use XrsAudW as XrsDsp4W;
///0=reset, 1=active
pub use XrsAud as XrsDsp5;
///Field `XRS_DSP5` reader - 0=reset, 1=active
pub use XrsAudR as XrsDsp5R;
///Field `XRS_DSP5` writer - 0=reset, 1=active
pub use XrsAudW as XrsDsp5W;
///0=reset, 1=active
pub use XrsAud as XrsDspGen;
///Field `XRS_DSP_GEN` reader - 0=reset, 1=active
pub use XrsAudR as XrsDspGenR;
///Field `XRS_DSP_GEN` writer - 0=reset, 1=active
pub use XrsAudW as XrsDspGenW;
impl R {
    ///Bit 0 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_aud(&self) -> XrsAudR {
        XrsAudR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_img(&self) -> XrsImgR {
        XrsImgR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_usb(&self) -> XrsUsbR {
        XrsUsbR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_sdio(&self) -> XrsSdioR {
        XrsSdioR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_mmc(&self) -> XrsMmcR {
        XrsMmcR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_mmc_crg(&self) -> XrsMmcCrgR {
        XrsMmcCrgR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp0(&self) -> XrsDsp0R {
        XrsDsp0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp1(&self) -> XrsDsp1R {
        XrsDsp1R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp2(&self) -> XrsDsp2R {
        XrsDsp2R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp3(&self) -> XrsDsp3R {
        XrsDsp3R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp4(&self) -> XrsDsp4R {
        XrsDsp4R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp5(&self) -> XrsDsp5R {
        XrsDsp5R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp_gen(&self) -> XrsDspGenR {
        XrsDspGenR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_aud(&mut self) -> XrsAudW<'_, ResetSpec> {
        XrsAudW::new(self, 0)
    }
    ///Bit 4 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_img(&mut self) -> XrsImgW<'_, ResetSpec> {
        XrsImgW::new(self, 4)
    }
    ///Bit 8 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_usb(&mut self) -> XrsUsbW<'_, ResetSpec> {
        XrsUsbW::new(self, 8)
    }
    ///Bit 9 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_sdio(&mut self) -> XrsSdioW<'_, ResetSpec> {
        XrsSdioW::new(self, 9)
    }
    ///Bit 10 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_mmc(&mut self) -> XrsMmcW<'_, ResetSpec> {
        XrsMmcW::new(self, 10)
    }
    ///Bit 11 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_mmc_crg(&mut self) -> XrsMmcCrgW<'_, ResetSpec> {
        XrsMmcCrgW::new(self, 11)
    }
    ///Bit 16 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp0(&mut self) -> XrsDsp0W<'_, ResetSpec> {
        XrsDsp0W::new(self, 16)
    }
    ///Bit 17 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp1(&mut self) -> XrsDsp1W<'_, ResetSpec> {
        XrsDsp1W::new(self, 17)
    }
    ///Bit 18 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp2(&mut self) -> XrsDsp2W<'_, ResetSpec> {
        XrsDsp2W::new(self, 18)
    }
    ///Bit 19 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp3(&mut self) -> XrsDsp3W<'_, ResetSpec> {
        XrsDsp3W::new(self, 19)
    }
    ///Bit 20 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp4(&mut self) -> XrsDsp4W<'_, ResetSpec> {
        XrsDsp4W::new(self, 20)
    }
    ///Bit 21 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp5(&mut self) -> XrsDsp5W<'_, ResetSpec> {
        XrsDsp5W::new(self, 21)
    }
    ///Bit 22 - 0=reset, 1=active
    #[inline(always)]
    pub fn xrs_dsp_gen(&mut self) -> XrsDspGenW<'_, ResetSpec> {
        XrsDspGenW::new(self, 22)
    }
}
/**Reset control

You can [`read`](crate::Reg::read) this register and get [`reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ResetSpec;
impl crate::RegisterSpec for ResetSpec {
    type Ux = u32;
}
///`read()` method returns [`reset::R`](R) reader structure
impl crate::Readable for ResetSpec {}
///`write(|w| ..)` method takes [`reset::W`](W) writer structure
impl crate::Writable for ResetSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RESET to value 0
impl crate::Resettable for ResetSpec {}
