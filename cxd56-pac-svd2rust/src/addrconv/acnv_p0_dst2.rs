///Register `ACNV_P0_DST2` reader
pub type R = crate::R<AcnvP0Dst2Spec>;
///Register `ACNV_P0_DST2` writer
pub type W = crate::W<AcnvP0Dst2Spec>;
///Field `AREA_4` reader - 0x40000 .. 0x50000
pub type Area4R = crate::FieldReader<u16>;
///Field `AREA_4` writer - 0x40000 .. 0x50000
pub type Area4W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `AREA_5` reader - 0x50000 .. 0x60000
pub type Area5R = crate::FieldReader<u16>;
///Field `AREA_5` writer - 0x50000 .. 0x60000
pub type Area5W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - 0x40000 .. 0x50000
    #[inline(always)]
    pub fn area_4(&self) -> Area4R {
        Area4R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - 0x50000 .. 0x60000
    #[inline(always)]
    pub fn area_5(&self) -> Area5R {
        Area5R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - 0x40000 .. 0x50000
    #[inline(always)]
    pub fn area_4(&mut self) -> Area4W<'_, AcnvP0Dst2Spec> {
        Area4W::new(self, 0)
    }
    ///Bits 16:26 - 0x50000 .. 0x60000
    #[inline(always)]
    pub fn area_5(&mut self) -> Area5W<'_, AcnvP0Dst2Spec> {
        Area5W::new(self, 16)
    }
}
/**CPU 0 address conversion area 4/5

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcnvP0Dst2Spec;
impl crate::RegisterSpec for AcnvP0Dst2Spec {
    type Ux = u32;
}
///`read()` method returns [`acnv_p0_dst2::R`](R) reader structure
impl crate::Readable for AcnvP0Dst2Spec {}
///`write(|w| ..)` method takes [`acnv_p0_dst2::W`](W) writer structure
impl crate::Writable for AcnvP0Dst2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACNV_P0_DST2 to value 0x0005_0004
impl crate::Resettable for AcnvP0Dst2Spec {
    const RESET_VALUE: u32 = 0x0005_0004;
}
