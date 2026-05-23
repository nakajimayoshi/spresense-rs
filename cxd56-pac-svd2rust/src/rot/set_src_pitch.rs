///Register `SET_SRC_PITCH` reader
pub type R = crate::R<SetSrcPitchSpec>;
///Register `SET_SRC_PITCH` writer
pub type W = crate::W<SetSrcPitchSpec>;
///Field `PITCH` reader -
pub type PitchR = crate::FieldReader<u16>;
///Field `PITCH` writer -
pub type PitchW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn pitch(&self) -> PitchR {
        PitchR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    pub fn pitch(&mut self) -> PitchW<'_, SetSrcPitchSpec> {
        PitchW::new(self, 0)
    }
}
/**Source Image Pitch (Actual size - 1)

You can [`read`](crate::Reg::read) this register and get [`set_src_pitch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_src_pitch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SetSrcPitchSpec;
impl crate::RegisterSpec for SetSrcPitchSpec {
    type Ux = u32;
}
///`read()` method returns [`set_src_pitch::R`](R) reader structure
impl crate::Readable for SetSrcPitchSpec {}
///`write(|w| ..)` method takes [`set_src_pitch::W`](W) writer structure
impl crate::Writable for SetSrcPitchSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SET_SRC_PITCH to value 0
impl crate::Resettable for SetSrcPitchSpec {}
