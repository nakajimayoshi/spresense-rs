///Register `SET_DST_PITCH` reader
pub type R = crate::R<SetDstPitchSpec>;
///Register `SET_DST_PITCH` writer
pub type W = crate::W<SetDstPitchSpec>;
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
    pub fn pitch(&mut self) -> PitchW<'_, SetDstPitchSpec> {
        PitchW::new(self, 0)
    }
}
/**Destination Image Pitch (Actual size - 1)

You can [`read`](crate::Reg::read) this register and get [`set_dst_pitch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_dst_pitch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SetDstPitchSpec;
impl crate::RegisterSpec for SetDstPitchSpec {
    type Ux = u32;
}
///`read()` method returns [`set_dst_pitch::R`](R) reader structure
impl crate::Readable for SetDstPitchSpec {}
///`write(|w| ..)` method takes [`set_dst_pitch::W`](W) writer structure
impl crate::Writable for SetDstPitchSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SET_DST_PITCH to value 0
impl crate::Resettable for SetDstPitchSpec {}
