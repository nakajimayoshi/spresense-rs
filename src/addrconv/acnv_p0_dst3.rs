#[doc = "Register `ACNV_P0_DST3` reader"]
pub type R = crate::R<AcnvP0Dst3Spec>;
#[doc = "Register `ACNV_P0_DST3` writer"]
pub type W = crate::W<AcnvP0Dst3Spec>;
#[doc = "Field `AREA_6` reader - 0x60000 .. 0x70000"]
pub type Area6R = crate::FieldReader<u16>;
#[doc = "Field `AREA_6` writer - 0x60000 .. 0x70000"]
pub type Area6W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `AREA_7` reader - 0x70000 .. 0x80000"]
pub type Area7R = crate::FieldReader<u16>;
#[doc = "Field `AREA_7` writer - 0x70000 .. 0x80000"]
pub type Area7W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 0x60000 .. 0x70000"]
    #[inline(always)]
    pub fn area_6(&self) -> Area6R {
        Area6R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 0x70000 .. 0x80000"]
    #[inline(always)]
    pub fn area_7(&self) -> Area7R {
        Area7R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 0x60000 .. 0x70000"]
    #[inline(always)]
    pub fn area_6(&mut self) -> Area6W<'_, AcnvP0Dst3Spec> {
        Area6W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 0x70000 .. 0x80000"]
    #[inline(always)]
    pub fn area_7(&mut self) -> Area7W<'_, AcnvP0Dst3Spec> {
        Area7W::new(self, 16)
    }
}
#[doc = "CPU 0 address conversion area 6/7\n\nYou can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcnvP0Dst3Spec;
impl crate::RegisterSpec for AcnvP0Dst3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acnv_p0_dst3::R`](R) reader structure"]
impl crate::Readable for AcnvP0Dst3Spec {}
#[doc = "`write(|w| ..)` method takes [`acnv_p0_dst3::W`](W) writer structure"]
impl crate::Writable for AcnvP0Dst3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACNV_P0_DST3 to value 0x0007_0006"]
impl crate::Resettable for AcnvP0Dst3Spec {
    const RESET_VALUE: u32 = 0x0007_0006;
}
