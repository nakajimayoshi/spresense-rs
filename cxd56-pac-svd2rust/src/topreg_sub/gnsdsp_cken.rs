///Register `GNSDSP_CKEN` reader
pub type R = crate::R<GnsdspCkenSpec>;
///Register `GNSDSP_CKEN` writer
pub type W = crate::W<GnsdspCkenSpec>;
///Field `GNSDSP_P1` reader - GNSS DSP P1 clock enable
pub type GnsdspP1R = crate::BitReader;
///Field `GNSDSP_P1` writer - GNSS DSP P1 clock enable
pub type GnsdspP1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GNSDSP_COP` reader - GNSS DSP coprocessor clock enable
pub type GnsdspCopR = crate::BitReader;
///Field `GNSDSP_COP` writer - GNSS DSP coprocessor clock enable
pub type GnsdspCopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - GNSS DSP P1 clock enable
    #[inline(always)]
    pub fn gnsdsp_p1(&self) -> GnsdspP1R {
        GnsdspP1R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - GNSS DSP coprocessor clock enable
    #[inline(always)]
    pub fn gnsdsp_cop(&self) -> GnsdspCopR {
        GnsdspCopR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - GNSS DSP P1 clock enable
    #[inline(always)]
    pub fn gnsdsp_p1(&mut self) -> GnsdspP1W<'_, GnsdspCkenSpec> {
        GnsdspP1W::new(self, 5)
    }
    ///Bit 7 - GNSS DSP coprocessor clock enable
    #[inline(always)]
    pub fn gnsdsp_cop(&mut self) -> GnsdspCopW<'_, GnsdspCkenSpec> {
        GnsdspCopW::new(self, 7)
    }
}
/**GNSS DSP clock enables

You can [`read`](crate::Reg::read) this register and get [`gnsdsp_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnsdsp_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GnsdspCkenSpec;
impl crate::RegisterSpec for GnsdspCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`gnsdsp_cken::R`](R) reader structure
impl crate::Readable for GnsdspCkenSpec {}
///`write(|w| ..)` method takes [`gnsdsp_cken::W`](W) writer structure
impl crate::Writable for GnsdspCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GNSDSP_CKEN to value 0
impl crate::Resettable for GnsdspCkenSpec {}
