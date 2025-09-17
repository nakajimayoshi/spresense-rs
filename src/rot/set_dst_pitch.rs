#[doc = "Register `SET_DST_PITCH` reader"]
pub type R = crate::R<SetDstPitchSpec>;
#[doc = "Register `SET_DST_PITCH` writer"]
pub type W = crate::W<SetDstPitchSpec>;
#[doc = "Field `PITCH` reader - "]
pub type PitchR = crate::FieldReader<u16>;
#[doc = "Field `PITCH` writer - "]
pub type PitchW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pitch(&self) -> PitchR {
        PitchR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pitch(&mut self) -> PitchW<'_, SetDstPitchSpec> {
        PitchW::new(self, 0)
    }
}
#[doc = "Destination Image Pitch (Actual size - 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`set_dst_pitch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_dst_pitch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetDstPitchSpec;
impl crate::RegisterSpec for SetDstPitchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set_dst_pitch::R`](R) reader structure"]
impl crate::Readable for SetDstPitchSpec {}
#[doc = "`write(|w| ..)` method takes [`set_dst_pitch::W`](W) writer structure"]
impl crate::Writable for SetDstPitchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_DST_PITCH to value 0"]
impl crate::Resettable for SetDstPitchSpec {}
