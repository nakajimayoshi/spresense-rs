#[doc = "Register `ACNV_P0_DST4` reader"]
pub type R = crate::R<AcnvP0Dst4Spec>;
#[doc = "Register `ACNV_P0_DST4` writer"]
pub type W = crate::W<AcnvP0Dst4Spec>;
#[doc = "Field `AREA_8` reader - 0x80000 .. 0x90000"]
pub type Area8R = crate::FieldReader<u16>;
#[doc = "Field `AREA_8` writer - 0x80000 .. 0x90000"]
pub type Area8W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `AREA_9` reader - 0x90000 .. 0xa0000"]
pub type Area9R = crate::FieldReader<u16>;
#[doc = "Field `AREA_9` writer - 0x90000 .. 0xa0000"]
pub type Area9W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 0x80000 .. 0x90000"]
    #[inline(always)]
    pub fn area_8(&self) -> Area8R {
        Area8R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 0x90000 .. 0xa0000"]
    #[inline(always)]
    pub fn area_9(&self) -> Area9R {
        Area9R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 0x80000 .. 0x90000"]
    #[inline(always)]
    pub fn area_8(&mut self) -> Area8W<'_, AcnvP0Dst4Spec> {
        Area8W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 0x90000 .. 0xa0000"]
    #[inline(always)]
    pub fn area_9(&mut self) -> Area9W<'_, AcnvP0Dst4Spec> {
        Area9W::new(self, 16)
    }
}
#[doc = "CPU 0 address conversion area 8/9\n\nYou can [`read`](crate::Reg::read) this register and get [`acnv_p0_dst4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acnv_p0_dst4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcnvP0Dst4Spec;
impl crate::RegisterSpec for AcnvP0Dst4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acnv_p0_dst4::R`](R) reader structure"]
impl crate::Readable for AcnvP0Dst4Spec {}
#[doc = "`write(|w| ..)` method takes [`acnv_p0_dst4::W`](W) writer structure"]
impl crate::Writable for AcnvP0Dst4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACNV_P0_DST4 to value 0x0009_0008"]
impl crate::Resettable for AcnvP0Dst4Spec {
    const RESET_VALUE: u32 = 0x0009_0008;
}
