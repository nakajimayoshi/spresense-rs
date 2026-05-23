///Register `CK_GATE_AHB` reader
pub type R = crate::R<CkGateAhbSpec>;
///Register `CK_GATE_AHB` writer
pub type W = crate::W<CkGateAhbSpec>;
/**0=Gated, 1=Ungated

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CkGateAud {
    ///0: Gated
    Gated = 0,
    ///1: Ungated
    Ungated = 1,
}
impl From<CkGateAud> for bool {
    #[inline(always)]
    fn from(variant: CkGateAud) -> Self {
        variant as u8 != 0
    }
}
///Field `CK_GATE_AUD` reader - 0=Gated, 1=Ungated
pub type CkGateAudR = crate::BitReader<CkGateAud>;
impl CkGateAudR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CkGateAud {
        match self.bits {
            false => CkGateAud::Gated,
            true => CkGateAud::Ungated,
        }
    }
    ///Gated
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == CkGateAud::Gated
    }
    ///Ungated
    #[inline(always)]
    pub fn is_ungated(&self) -> bool {
        *self == CkGateAud::Ungated
    }
}
///Field `CK_GATE_AUD` writer - 0=Gated, 1=Ungated
pub type CkGateAudW<'a, REG> = crate::BitWriter<'a, REG, CkGateAud>;
impl<'a, REG> CkGateAudW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Gated
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(CkGateAud::Gated)
    }
    ///Ungated
    #[inline(always)]
    pub fn ungated(self) -> &'a mut crate::W<REG> {
        self.variant(CkGateAud::Ungated)
    }
}
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateImg;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateUsb;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateSdio;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateMmc;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateDsp0;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateDsp1;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateDsp2;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateDsp3;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateDsp4;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateDsp5;
///0=Gated, 1=Ungated
pub use CkGateAud as CkGateDmac;
///Field `CK_GATE_IMG` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateImgR;
///Field `CK_GATE_USB` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateUsbR;
///Field `CK_GATE_SDIO` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateSdioR;
///Field `CK_GATE_MMC` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateMmcR;
///Field `CK_GATE_DSP0` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateDsp0R;
///Field `CK_GATE_DSP1` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateDsp1R;
///Field `CK_GATE_DSP2` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateDsp2R;
///Field `CK_GATE_DSP3` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateDsp3R;
///Field `CK_GATE_DSP4` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateDsp4R;
///Field `CK_GATE_DSP5` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateDsp5R;
///Field `CK_GATE_DMAC` reader - 0=Gated, 1=Ungated
pub use CkGateAudR as CkGateDmacR;
///Field `CK_GATE_IMG` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateImgW;
///Field `CK_GATE_USB` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateUsbW;
///Field `CK_GATE_SDIO` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateSdioW;
///Field `CK_GATE_MMC` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateMmcW;
///Field `CK_GATE_DSP0` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateDsp0W;
///Field `CK_GATE_DSP1` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateDsp1W;
///Field `CK_GATE_DSP2` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateDsp2W;
///Field `CK_GATE_DSP3` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateDsp3W;
///Field `CK_GATE_DSP4` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateDsp4W;
///Field `CK_GATE_DSP5` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateDsp5W;
///Field `CK_GATE_DMAC` writer - 0=Gated, 1=Ungated
pub use CkGateAudW as CkGateDmacW;
impl R {
    ///Bit 0 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_aud(&self) -> CkGateAudR {
        CkGateAudR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_img(&self) -> CkGateImgR {
        CkGateImgR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_usb(&self) -> CkGateUsbR {
        CkGateUsbR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_sdio(&self) -> CkGateSdioR {
        CkGateSdioR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_mmc(&self) -> CkGateMmcR {
        CkGateMmcR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp0(&self) -> CkGateDsp0R {
        CkGateDsp0R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp1(&self) -> CkGateDsp1R {
        CkGateDsp1R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp2(&self) -> CkGateDsp2R {
        CkGateDsp2R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp3(&self) -> CkGateDsp3R {
        CkGateDsp3R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp4(&self) -> CkGateDsp4R {
        CkGateDsp4R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp5(&self) -> CkGateDsp5R {
        CkGateDsp5R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 28 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dmac(&self) -> CkGateDmacR {
        CkGateDmacR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_aud(&mut self) -> CkGateAudW<'_, CkGateAhbSpec> {
        CkGateAudW::new(self, 0)
    }
    ///Bit 4 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_img(&mut self) -> CkGateImgW<'_, CkGateAhbSpec> {
        CkGateImgW::new(self, 4)
    }
    ///Bit 8 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_usb(&mut self) -> CkGateUsbW<'_, CkGateAhbSpec> {
        CkGateUsbW::new(self, 8)
    }
    ///Bit 9 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_sdio(&mut self) -> CkGateSdioW<'_, CkGateAhbSpec> {
        CkGateSdioW::new(self, 9)
    }
    ///Bit 10 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_mmc(&mut self) -> CkGateMmcW<'_, CkGateAhbSpec> {
        CkGateMmcW::new(self, 10)
    }
    ///Bit 16 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp0(&mut self) -> CkGateDsp0W<'_, CkGateAhbSpec> {
        CkGateDsp0W::new(self, 16)
    }
    ///Bit 17 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp1(&mut self) -> CkGateDsp1W<'_, CkGateAhbSpec> {
        CkGateDsp1W::new(self, 17)
    }
    ///Bit 18 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp2(&mut self) -> CkGateDsp2W<'_, CkGateAhbSpec> {
        CkGateDsp2W::new(self, 18)
    }
    ///Bit 19 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp3(&mut self) -> CkGateDsp3W<'_, CkGateAhbSpec> {
        CkGateDsp3W::new(self, 19)
    }
    ///Bit 20 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp4(&mut self) -> CkGateDsp4W<'_, CkGateAhbSpec> {
        CkGateDsp4W::new(self, 20)
    }
    ///Bit 21 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dsp5(&mut self) -> CkGateDsp5W<'_, CkGateAhbSpec> {
        CkGateDsp5W::new(self, 21)
    }
    ///Bit 28 - 0=Gated, 1=Ungated
    #[inline(always)]
    pub fn ck_gate_dmac(&mut self) -> CkGateDmacW<'_, CkGateAhbSpec> {
        CkGateDmacW::new(self, 28)
    }
}
/**CPU/Peripheral clock gating control

You can [`read`](crate::Reg::read) this register and get [`ck_gate_ahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ck_gate_ahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkGateAhbSpec;
impl crate::RegisterSpec for CkGateAhbSpec {
    type Ux = u32;
}
///`read()` method returns [`ck_gate_ahb::R`](R) reader structure
impl crate::Readable for CkGateAhbSpec {}
///`write(|w| ..)` method takes [`ck_gate_ahb::W`](W) writer structure
impl crate::Writable for CkGateAhbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CK_GATE_AHB to value 0
impl crate::Resettable for CkGateAhbSpec {}
