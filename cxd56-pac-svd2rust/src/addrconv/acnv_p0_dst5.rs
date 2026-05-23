///Register `ACNV_P0_DST5` reader
pub type R = crate::R<AcnvP0Dst5Spec>;
///Register `ACNV_P0_DST5` writer
pub type W = crate::W<AcnvP0Dst5Spec>;
///Field `AREA_A` reader - 0xa0000 .. 0xb0000
pub type AreaAR = crate::FieldReader<u16>;
///Field `AREA_A` writer - 0xa0000 .. 0xb0000
pub type AreaAW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `AREA_B` reader - 0xb0000 .. 0xc0000
pub type AreaBR = crate::FieldReader<u16>;
///Field `AREA_B` writer - 0xb0000 .. 0xc0000
pub type AreaBW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - 0xa0000 .. 0xb0000
    #[inline(always)]
    pub fn area_a(&self) -> AreaAR {
        AreaAR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - 0xb0000 .. 0xc0000
    #[inline(always)]
    pub fn area_b(&self) -> AreaBR {
        AreaBR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - 0xa0000 .. 0xb0000
    #[inline(always)]
    pub fn area_a(&mut self) -> AreaAW<'_, AcnvP0Dst5Spec> {
        AreaAW::new(self, 0)
    }
    ///Bits 16:26 - 0xb0000 .. 0xc0000
    #[inline(always)]
    pub fn area_b(&mut self) -> AreaBW<'_, AcnvP0Dst5Spec> {
        AreaBW::new(self, 16)
    }
}
/**CPU 0 address conversion area A/B

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcnvP0Dst5Spec;
impl crate::RegisterSpec for AcnvP0Dst5Spec {
    type Ux = u32;
}
///`read()` method returns [`acnv_p0_dst5::R`](R) reader structure
impl crate::Readable for AcnvP0Dst5Spec {}
///`write(|w| ..)` method takes [`acnv_p0_dst5::W`](W) writer structure
impl crate::Writable for AcnvP0Dst5Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACNV_P0_DST5 to value 0x000b_000a
impl crate::Resettable for AcnvP0Dst5Spec {
    const RESET_VALUE: u32 = 0x000b_000a;
}
