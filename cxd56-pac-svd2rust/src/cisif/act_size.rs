#[doc = "Register `ACT_SIZE` reader"]
pub type R = crate::R<ActSizeSpec>;
#[doc = "Register `ACT_SIZE` writer"]
pub type W = crate::W<ActSizeSpec>;
#[doc = "Field `act_hsz` reader - "]
pub type ActHszR = crate::FieldReader<u16>;
#[doc = "Field `act_hsz` writer - "]
pub type ActHszW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `act_vsz` reader - "]
pub type ActVszR = crate::FieldReader<u16>;
#[doc = "Field `act_vsz` writer - "]
pub type ActVszW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn act_hsz(&self) -> ActHszR {
        ActHszR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn act_vsz(&self) -> ActVszR {
        ActVszR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn act_hsz(&mut self) -> ActHszW<'_, ActSizeSpec> {
        ActHszW::new(self, 0)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn act_vsz(&mut self) -> ActVszW<'_, ActSizeSpec> {
        ActVszW::new(self, 16)
    }
}
#[doc = "Active area size setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`act_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`act_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActSizeSpec;
impl crate::RegisterSpec for ActSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_size::R`](R) reader structure"]
impl crate::Readable for ActSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`act_size::W`](W) writer structure"]
impl crate::Writable for ActSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACT_SIZE to value 0"]
impl crate::Resettable for ActSizeSpec {}
