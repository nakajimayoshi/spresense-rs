///Register `ACNV_P0_DST7` reader
pub type R = crate::R<AcnvP0Dst7Spec>;
///Register `ACNV_P0_DST7` writer
pub type W = crate::W<AcnvP0Dst7Spec>;
///Field `AREA_E` reader - 0xe0000 .. 0xf0000
pub type AreaER = crate::FieldReader<u16>;
///Field `AREA_E` writer - 0xe0000 .. 0xf0000
pub type AreaEW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `AREA_F` reader - 0xf0000 .. 0x100000
pub type AreaFR = crate::FieldReader<u16>;
///Field `AREA_F` writer - 0xf0000 .. 0x100000
pub type AreaFW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - 0xe0000 .. 0xf0000
    #[inline(always)]
    pub fn area_e(&self) -> AreaER {
        AreaER::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - 0xf0000 .. 0x100000
    #[inline(always)]
    pub fn area_f(&self) -> AreaFR {
        AreaFR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - 0xe0000 .. 0xf0000
    #[inline(always)]
    pub fn area_e(&mut self) -> AreaEW<'_, AcnvP0Dst7Spec> {
        AreaEW::new(self, 0)
    }
    ///Bits 16:26 - 0xf0000 .. 0x100000
    #[inline(always)]
    pub fn area_f(&mut self) -> AreaFW<'_, AcnvP0Dst7Spec> {
        AreaFW::new(self, 16)
    }
}
/**CPU 0 address conversion area E/F

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcnvP0Dst7Spec;
impl crate::RegisterSpec for AcnvP0Dst7Spec {
    type Ux = u32;
}
///`read()` method returns [`acnv_p0_dst7::R`](R) reader structure
impl crate::Readable for AcnvP0Dst7Spec {}
///`write(|w| ..)` method takes [`acnv_p0_dst7::W`](W) writer structure
impl crate::Writable for AcnvP0Dst7Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACNV_P0_DST7 to value 0x000f_000e
impl crate::Resettable for AcnvP0Dst7Spec {
    const RESET_VALUE: u32 = 0x000f_000e;
}
