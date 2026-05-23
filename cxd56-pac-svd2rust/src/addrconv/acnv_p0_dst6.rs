///Register `ACNV_P0_DST6` reader
pub type R = crate::R<AcnvP0Dst6Spec>;
///Register `ACNV_P0_DST6` writer
pub type W = crate::W<AcnvP0Dst6Spec>;
///Field `AREA_C` reader - 0xc0000 .. 0xd0000
pub type AreaCR = crate::FieldReader<u16>;
///Field `AREA_C` writer - 0xc0000 .. 0xd0000
pub type AreaCW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `AREA_D` reader - 0xd0000 .. 0xe0000
pub type AreaDR = crate::FieldReader<u16>;
///Field `AREA_D` writer - 0xd0000 .. 0xe0000
pub type AreaDW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - 0xc0000 .. 0xd0000
    #[inline(always)]
    pub fn area_c(&self) -> AreaCR {
        AreaCR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - 0xd0000 .. 0xe0000
    #[inline(always)]
    pub fn area_d(&self) -> AreaDR {
        AreaDR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - 0xc0000 .. 0xd0000
    #[inline(always)]
    pub fn area_c(&mut self) -> AreaCW<'_, AcnvP0Dst6Spec> {
        AreaCW::new(self, 0)
    }
    ///Bits 16:26 - 0xd0000 .. 0xe0000
    #[inline(always)]
    pub fn area_d(&mut self) -> AreaDW<'_, AcnvP0Dst6Spec> {
        AreaDW::new(self, 16)
    }
}
/**CPU 0 address conversion area C/D

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcnvP0Dst6Spec;
impl crate::RegisterSpec for AcnvP0Dst6Spec {
    type Ux = u32;
}
///`read()` method returns [`acnv_p0_dst6::R`](R) reader structure
impl crate::Readable for AcnvP0Dst6Spec {}
///`write(|w| ..)` method takes [`acnv_p0_dst6::W`](W) writer structure
impl crate::Writable for AcnvP0Dst6Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACNV_P0_DST6 to value 0x000d_000c
impl crate::Resettable for AcnvP0Dst6Spec {
    const RESET_VALUE: u32 = 0x000d_000c;
}
