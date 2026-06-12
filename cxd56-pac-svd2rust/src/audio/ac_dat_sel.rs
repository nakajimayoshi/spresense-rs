///Register `AC_DAT_SEL` reader
pub type R = crate::R<AcDatSelSpec>;
///Register `AC_DAT_SEL` writer
pub type W = crate::W<AcDatSelSpec>;
///Field `AU_DAT_SEL2` reader - AU_DAT_SEL2 source (0-3=MIC1-4, 4=BUSIF2 DMA out)
pub type AuDatSel2R = crate::FieldReader;
///Field `AU_DAT_SEL2` writer - AU_DAT_SEL2 source (0-3=MIC1-4, 4=BUSIF2 DMA out)
pub type AuDatSel2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AU_DAT_SEL1` reader - AU_DAT_SEL1 source (0-3=MIC1-4, 4=BUSIF1 DMA out)
pub type AuDatSel1R = crate::FieldReader;
///Field `AU_DAT_SEL1` writer - AU_DAT_SEL1 source (0-3=MIC1-4, 4=BUSIF1 DMA out)
pub type AuDatSel1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COD_INSEL3` reader - Codec DSP input 3 select
pub type CodInsel3R = crate::FieldReader;
///Field `COD_INSEL3` writer - Codec DSP input 3 select
pub type CodInsel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COD_INSEL2` reader - Codec DSP input 2 select (0=SRC1, 1=SRC2, 2=AU_DAT_SEL1, 3=AU_DAT_SEL2)
pub type CodInsel2R = crate::FieldReader;
///Field `COD_INSEL2` writer - Codec DSP input 2 select (0=SRC1, 1=SRC2, 2=AU_DAT_SEL1, 3=AU_DAT_SEL2)
pub type CodInsel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COD_INSEL1` reader - Codec DSP input 1 select (0=SRC1, 1=SRC2, 2=AU_DAT_SEL1, 3=AU_DAT_SEL2)
pub type CodInsel1R = crate::FieldReader;
///Field `COD_INSEL1` writer - Codec DSP input 1 select (0=SRC1, 1=SRC2, 2=AU_DAT_SEL1, 3=AU_DAT_SEL2)
pub type CodInsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:18 - AU_DAT_SEL2 source (0-3=MIC1-4, 4=BUSIF2 DMA out)
    #[inline(always)]
    pub fn au_dat_sel2(&self) -> AuDatSel2R {
        AuDatSel2R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - AU_DAT_SEL1 source (0-3=MIC1-4, 4=BUSIF1 DMA out)
    #[inline(always)]
    pub fn au_dat_sel1(&self) -> AuDatSel1R {
        AuDatSel1R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:25 - Codec DSP input 3 select
    #[inline(always)]
    pub fn cod_insel3(&self) -> CodInsel3R {
        CodInsel3R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Codec DSP input 2 select (0=SRC1, 1=SRC2, 2=AU_DAT_SEL1, 3=AU_DAT_SEL2)
    #[inline(always)]
    pub fn cod_insel2(&self) -> CodInsel2R {
        CodInsel2R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Codec DSP input 1 select (0=SRC1, 1=SRC2, 2=AU_DAT_SEL1, 3=AU_DAT_SEL2)
    #[inline(always)]
    pub fn cod_insel1(&self) -> CodInsel1R {
        CodInsel1R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    ///Bits 16:18 - AU_DAT_SEL2 source (0-3=MIC1-4, 4=BUSIF2 DMA out)
    #[inline(always)]
    pub fn au_dat_sel2(&mut self) -> AuDatSel2W<'_, AcDatSelSpec> {
        AuDatSel2W::new(self, 16)
    }
    ///Bits 20:22 - AU_DAT_SEL1 source (0-3=MIC1-4, 4=BUSIF1 DMA out)
    #[inline(always)]
    pub fn au_dat_sel1(&mut self) -> AuDatSel1W<'_, AcDatSelSpec> {
        AuDatSel1W::new(self, 20)
    }
    ///Bits 24:25 - Codec DSP input 3 select
    #[inline(always)]
    pub fn cod_insel3(&mut self) -> CodInsel3W<'_, AcDatSelSpec> {
        CodInsel3W::new(self, 24)
    }
    ///Bits 26:27 - Codec DSP input 2 select (0=SRC1, 1=SRC2, 2=AU_DAT_SEL1, 3=AU_DAT_SEL2)
    #[inline(always)]
    pub fn cod_insel2(&mut self) -> CodInsel2W<'_, AcDatSelSpec> {
        CodInsel2W::new(self, 26)
    }
    ///Bits 28:29 - Codec DSP input 1 select (0=SRC1, 1=SRC2, 2=AU_DAT_SEL1, 3=AU_DAT_SEL2)
    #[inline(always)]
    pub fn cod_insel1(&mut self) -> CodInsel1W<'_, AcDatSelSpec> {
        CodInsel1W::new(self, 28)
    }
}
/**Audio data routing — AU_DAT_SEL / COD_INSEL muxes

You can [`read`](crate::Reg::read) this register and get [`ac_dat_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ac_dat_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcDatSelSpec;
impl crate::RegisterSpec for AcDatSelSpec {
    type Ux = u32;
}
///`read()` method returns [`ac_dat_sel::R`](R) reader structure
impl crate::Readable for AcDatSelSpec {}
///`write(|w| ..)` method takes [`ac_dat_sel::W`](W) writer structure
impl crate::Writable for AcDatSelSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AC_DAT_SEL to value 0
impl crate::Resettable for AcDatSelSpec {}
