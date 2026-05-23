///Register `INTR_STAT` reader
pub type R = crate::R<IntrStatSpec>;
///Field `vs_int` reader -
pub type VsIntR = crate::BitReader;
///Field `eoy_int` reader -
pub type EoyIntR = crate::BitReader;
///Field `soy_int` reader -
pub type SoyIntR = crate::BitReader;
///Field `eoi_int` reader -
pub type EoiIntR = crate::BitReader;
///Field `soi_int` reader -
pub type SoiIntR = crate::BitReader;
///Field `ycc_vact_end_int` reader -
pub type YccVactEndIntR = crate::BitReader;
///Field `jpg_vact_end_int` reader -
pub type JpgVactEndIntR = crate::BitReader;
///Field `ycc_axi_trdn_int` reader -
pub type YccAxiTrdnIntR = crate::BitReader;
///Field `ycc_nstorage_int` reader -
pub type YccNstorageIntR = crate::BitReader;
///Field `ycc_darea_end_int` reader -
pub type YccDareaEndIntR = crate::BitReader;
///Field `jpg_axi_trdn_int` reader -
pub type JpgAxiTrdnIntR = crate::BitReader;
///Field `jpg_nstorage_int` reader -
pub type JpgNstorageIntR = crate::BitReader;
///Field `jpg_darea_end_int` reader -
pub type JpgDareaEndIntR = crate::BitReader;
///Field `vlatch_int` reader -
pub type VlatchIntR = crate::BitReader;
///Field `size_over_int` reader -
pub type SizeOverIntR = crate::BitReader;
///Field `size_under_int` reader -
pub type SizeUnderIntR = crate::BitReader;
///Field `ycc_marker_err_int` reader -
pub type YccMarkerErrIntR = crate::BitReader;
///Field `ycc_axi_trerr__int` reader -
pub type YccAxiTrerr_IntR = crate::BitReader;
///Field `ycc_fifo_ovf_int` reader -
pub type YccFifoOvfIntR = crate::BitReader;
///Field `ycc_mem_ovf_int` reader -
pub type YccMemOvfIntR = crate::BitReader;
///Field `jpg_marker_err_int` reader -
pub type JpgMarkerErrIntR = crate::BitReader;
///Field `jpg_axi_trerr_int` reader -
pub type JpgAxiTrerrIntR = crate::BitReader;
///Field `jpg_fifo_ovf_int` reader -
pub type JpgFifoOvfIntR = crate::BitReader;
///Field `jpg_mem_ovf_int` reader -
pub type JpgMemOvfIntR = crate::BitReader;
///Field `jpg_err_status_int` reader -
pub type JpgErrStatusIntR = crate::BitReader;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn vs_int(&self) -> VsIntR {
        VsIntR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn eoy_int(&self) -> EoyIntR {
        EoyIntR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn soy_int(&self) -> SoyIntR {
        SoyIntR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn eoi_int(&self) -> EoiIntR {
        EoiIntR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn soi_int(&self) -> SoiIntR {
        SoiIntR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn ycc_vact_end_int(&self) -> YccVactEndIntR {
        YccVactEndIntR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn jpg_vact_end_int(&self) -> JpgVactEndIntR {
        JpgVactEndIntR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn ycc_axi_trdn_int(&self) -> YccAxiTrdnIntR {
        YccAxiTrdnIntR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn ycc_nstorage_int(&self) -> YccNstorageIntR {
        YccNstorageIntR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn ycc_darea_end_int(&self) -> YccDareaEndIntR {
        YccDareaEndIntR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn jpg_axi_trdn_int(&self) -> JpgAxiTrdnIntR {
        JpgAxiTrdnIntR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn jpg_nstorage_int(&self) -> JpgNstorageIntR {
        JpgNstorageIntR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12
    #[inline(always)]
    pub fn jpg_darea_end_int(&self) -> JpgDareaEndIntR {
        JpgDareaEndIntR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn vlatch_int(&self) -> VlatchIntR {
        VlatchIntR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn size_over_int(&self) -> SizeOverIntR {
        SizeOverIntR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn size_under_int(&self) -> SizeUnderIntR {
        SizeUnderIntR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn ycc_marker_err_int(&self) -> YccMarkerErrIntR {
        YccMarkerErrIntR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn ycc_axi_trerr__int(&self) -> YccAxiTrerr_IntR {
        YccAxiTrerr_IntR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn ycc_fifo_ovf_int(&self) -> YccFifoOvfIntR {
        YccFifoOvfIntR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn ycc_mem_ovf_int(&self) -> YccMemOvfIntR {
        YccMemOvfIntR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24
    #[inline(always)]
    pub fn jpg_marker_err_int(&self) -> JpgMarkerErrIntR {
        JpgMarkerErrIntR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25
    #[inline(always)]
    pub fn jpg_axi_trerr_int(&self) -> JpgAxiTrerrIntR {
        JpgAxiTrerrIntR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26
    #[inline(always)]
    pub fn jpg_fifo_ovf_int(&self) -> JpgFifoOvfIntR {
        JpgFifoOvfIntR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27
    #[inline(always)]
    pub fn jpg_mem_ovf_int(&self) -> JpgMemOvfIntR {
        JpgMemOvfIntR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28
    #[inline(always)]
    pub fn jpg_err_status_int(&self) -> JpgErrStatusIntR {
        JpgErrStatusIntR::new(((self.bits >> 28) & 1) != 0)
    }
}
/**Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`intr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntrStatSpec;
impl crate::RegisterSpec for IntrStatSpec {
    type Ux = u32;
}
///`read()` method returns [`intr_stat::R`](R) reader structure
impl crate::Readable for IntrStatSpec {}
///`reset()` method sets INTR_STAT to value 0
impl crate::Resettable for IntrStatSpec {}
