#[doc = "Register `ACNV_P0_DST1` reader"]
pub type R = crate::R<AcnvP0Dst1Spec>;
#[doc = "Register `ACNV_P0_DST1` writer"]
pub type W = crate::W<AcnvP0Dst1Spec>;
#[doc = "Field `AREA_2` reader - 0x20000 .. 0x30000"]
pub type Area2R = crate::FieldReader<u16>;
#[doc = "Field `AREA_2` writer - 0x20000 .. 0x30000"]
pub type Area2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `AREA_3` reader - 0x30000 .. 0x40000"]
pub type Area3R = crate::FieldReader<u16>;
#[doc = "Field `AREA_3` writer - 0x30000 .. 0x40000"]
pub type Area3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 0x20000 .. 0x30000"]
    #[inline(always)]
    pub fn area_2(&self) -> Area2R {
        Area2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 0x30000 .. 0x40000"]
    #[inline(always)]
    pub fn area_3(&self) -> Area3R {
        Area3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 0x20000 .. 0x30000"]
    #[inline(always)]
    pub fn area_2(&mut self) -> Area2W<'_, AcnvP0Dst1Spec> {
        Area2W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 0x30000 .. 0x40000"]
    #[inline(always)]
    pub fn area_3(&mut self) -> Area3W<'_, AcnvP0Dst1Spec> {
        Area3W::new(self, 16)
    }
}
#[doc = "CPU 0 address conversion area 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcnvP0Dst1Spec;
impl crate::RegisterSpec for AcnvP0Dst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acnv_p0_dst1::R`](R) reader structure"]
impl crate::Readable for AcnvP0Dst1Spec {}
#[doc = "`write(|w| ..)` method takes [`acnv_p0_dst1::W`](W) writer structure"]
impl crate::Writable for AcnvP0Dst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACNV_P0_DST1 to value 0x0003_0002"]
impl crate::Resettable for AcnvP0Dst1Spec {
    const RESET_VALUE: u32 = 0x0003_0002;
}
