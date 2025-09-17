#[doc = "Register `INTR_STAT` reader"]
pub type R = crate::R<IntrStatSpec>;
#[doc = "Field `vs_int` reader - "]
pub type VsIntR = crate::BitReader;
#[doc = "Field `eoy_int` reader - "]
pub type EoyIntR = crate::BitReader;
#[doc = "Field `soy_int` reader - "]
pub type SoyIntR = crate::BitReader;
#[doc = "Field `eoi_int` reader - "]
pub type EoiIntR = crate::BitReader;
#[doc = "Field `soi_int` reader - "]
pub type SoiIntR = crate::BitReader;
#[doc = "Field `ycc_vact_end_int` reader - "]
pub type YccVactEndIntR = crate::BitReader;
#[doc = "Field `jpg_vact_end_int` reader - "]
pub type JpgVactEndIntR = crate::BitReader;
#[doc = "Field `ycc_axi_trdn_int` reader - "]
pub type YccAxiTrdnIntR = crate::BitReader;
#[doc = "Field `ycc_nstorage_int` reader - "]
pub type YccNstorageIntR = crate::BitReader;
#[doc = "Field `ycc_darea_end_int` reader - "]
pub type YccDareaEndIntR = crate::BitReader;
#[doc = "Field `jpg_axi_trdn_int` reader - "]
pub type JpgAxiTrdnIntR = crate::BitReader;
#[doc = "Field `jpg_nstorage_int` reader - "]
pub type JpgNstorageIntR = crate::BitReader;
#[doc = "Field `jpg_darea_end_int` reader - "]
pub type JpgDareaEndIntR = crate::BitReader;
#[doc = "Field `vlatch_int` reader - "]
pub type VlatchIntR = crate::BitReader;
#[doc = "Field `size_over_int` reader - "]
pub type SizeOverIntR = crate::BitReader;
#[doc = "Field `size_under_int` reader - "]
pub type SizeUnderIntR = crate::BitReader;
#[doc = "Field `ycc_marker_err_int` reader - "]
pub type YccMarkerErrIntR = crate::BitReader;
#[doc = "Field `ycc_axi_trerr__int` reader - "]
pub type YccAxiTrerr_IntR = crate::BitReader;
#[doc = "Field `ycc_fifo_ovf_int` reader - "]
pub type YccFifoOvfIntR = crate::BitReader;
#[doc = "Field `ycc_mem_ovf_int` reader - "]
pub type YccMemOvfIntR = crate::BitReader;
#[doc = "Field `jpg_marker_err_int` reader - "]
pub type JpgMarkerErrIntR = crate::BitReader;
#[doc = "Field `jpg_axi_trerr_int` reader - "]
pub type JpgAxiTrerrIntR = crate::BitReader;
#[doc = "Field `jpg_fifo_ovf_int` reader - "]
pub type JpgFifoOvfIntR = crate::BitReader;
#[doc = "Field `jpg_mem_ovf_int` reader - "]
pub type JpgMemOvfIntR = crate::BitReader;
#[doc = "Field `jpg_err_status_int` reader - "]
pub type JpgErrStatusIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vs_int(&self) -> VsIntR {
        VsIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eoy_int(&self) -> EoyIntR {
        EoyIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn soy_int(&self) -> SoyIntR {
        SoyIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn eoi_int(&self) -> EoiIntR {
        EoiIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soi_int(&self) -> SoiIntR {
        SoiIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ycc_vact_end_int(&self) -> YccVactEndIntR {
        YccVactEndIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn jpg_vact_end_int(&self) -> JpgVactEndIntR {
        JpgVactEndIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ycc_axi_trdn_int(&self) -> YccAxiTrdnIntR {
        YccAxiTrdnIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ycc_nstorage_int(&self) -> YccNstorageIntR {
        YccNstorageIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ycc_darea_end_int(&self) -> YccDareaEndIntR {
        YccDareaEndIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn jpg_axi_trdn_int(&self) -> JpgAxiTrdnIntR {
        JpgAxiTrdnIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn jpg_nstorage_int(&self) -> JpgNstorageIntR {
        JpgNstorageIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn jpg_darea_end_int(&self) -> JpgDareaEndIntR {
        JpgDareaEndIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn vlatch_int(&self) -> VlatchIntR {
        VlatchIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn size_over_int(&self) -> SizeOverIntR {
        SizeOverIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn size_under_int(&self) -> SizeUnderIntR {
        SizeUnderIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ycc_marker_err_int(&self) -> YccMarkerErrIntR {
        YccMarkerErrIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ycc_axi_trerr__int(&self) -> YccAxiTrerr_IntR {
        YccAxiTrerr_IntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ycc_fifo_ovf_int(&self) -> YccFifoOvfIntR {
        YccFifoOvfIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ycc_mem_ovf_int(&self) -> YccMemOvfIntR {
        YccMemOvfIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn jpg_marker_err_int(&self) -> JpgMarkerErrIntR {
        JpgMarkerErrIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn jpg_axi_trerr_int(&self) -> JpgAxiTrerrIntR {
        JpgAxiTrerrIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn jpg_fifo_ovf_int(&self) -> JpgFifoOvfIntR {
        JpgFifoOvfIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn jpg_mem_ovf_int(&self) -> JpgMemOvfIntR {
        JpgMemOvfIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn jpg_err_status_int(&self) -> JpgErrStatusIntR {
        JpgErrStatusIntR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrStatSpec;
impl crate::RegisterSpec for IntrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_stat::R`](R) reader structure"]
impl crate::Readable for IntrStatSpec {}
#[doc = "`reset()` method sets INTR_STAT to value 0"]
impl crate::Resettable for IntrStatSpec {}
