///Register `ACNV_P0_DST0` reader
pub type R = crate::R<AcnvP0Dst0Spec>;
///Register `ACNV_P0_DST0` writer
pub type W = crate::W<AcnvP0Dst0Spec>;
///Field `AREA_0` reader - 0x00000 .. 0x10000
pub type Area0R = crate::FieldReader<u16>;
///Field `AREA_0` writer - 0x00000 .. 0x10000
pub type Area0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `AREA_1` reader - 0x10000 .. 0x20000
pub type Area1R = crate::FieldReader<u16>;
///Field `AREA_1` writer - 0x10000 .. 0x20000
pub type Area1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - 0x00000 .. 0x10000
    #[inline(always)]
    pub fn area_0(&self) -> Area0R {
        Area0R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - 0x10000 .. 0x20000
    #[inline(always)]
    pub fn area_1(&self) -> Area1R {
        Area1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - 0x00000 .. 0x10000
    #[inline(always)]
    pub fn area_0(&mut self) -> Area0W<'_, AcnvP0Dst0Spec> {
        Area0W::new(self, 0)
    }
    ///Bits 16:26 - 0x10000 .. 0x20000
    #[inline(always)]
    pub fn area_1(&mut self) -> Area1W<'_, AcnvP0Dst0Spec> {
        Area1W::new(self, 16)
    }
}
/**CPU 0 address conversion area 0/1

You can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AcnvP0Dst0Spec;
impl crate::RegisterSpec for AcnvP0Dst0Spec {
    type Ux = u32;
}
///`read()` method returns [`acnv_p0_dst0::R`](R) reader structure
impl crate::Readable for AcnvP0Dst0Spec {}
///`write(|w| ..)` method takes [`acnv_p0_dst0::W`](W) writer structure
impl crate::Writable for AcnvP0Dst0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACNV_P0_DST0 to value 0x0001_0000
impl crate::Resettable for AcnvP0Dst0Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
