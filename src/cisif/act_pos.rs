#[doc = "Register `ACT_POS` reader"]
pub type R = crate::R<ActPosSpec>;
#[doc = "Register `ACT_POS` writer"]
pub type W = crate::W<ActPosSpec>;
#[doc = "Field `act_hst` reader - "]
pub type ActHstR = crate::FieldReader<u16>;
#[doc = "Field `act_hst` writer - "]
pub type ActHstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `act_vst` reader - "]
pub type ActVstR = crate::FieldReader<u16>;
#[doc = "Field `act_vst` writer - "]
pub type ActVstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn act_hst(&self) -> ActHstR {
        ActHstR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn act_vst(&self) -> ActVstR {
        ActVstR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn act_hst(&mut self) -> ActHstW<'_, ActPosSpec> {
        ActHstW::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn act_vst(&mut self) -> ActVstW<'_, ActPosSpec> {
        ActVstW::new(self, 16)
    }
}
#[doc = "Active area position setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`act_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`act_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActPosSpec;
impl crate::RegisterSpec for ActPosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_pos::R`](R) reader structure"]
impl crate::Readable for ActPosSpec {}
#[doc = "`write(|w| ..)` method takes [`act_pos::W`](W) writer structure"]
impl crate::Writable for ActPosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACT_POS to value 0"]
impl crate::Resettable for ActPosSpec {}
