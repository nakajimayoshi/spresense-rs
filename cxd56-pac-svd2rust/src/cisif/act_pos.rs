///Register `ACT_POS` reader
pub type R = crate::R<ActPosSpec>;
///Register `ACT_POS` writer
pub type W = crate::W<ActPosSpec>;
///Field `act_hst` reader -
pub type ActHstR = crate::FieldReader<u16>;
///Field `act_hst` writer -
pub type ActHstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `act_vst` reader -
pub type ActVstR = crate::FieldReader<u16>;
///Field `act_vst` writer -
pub type ActVstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn act_hst(&self) -> ActHstR {
        ActHstR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn act_vst(&self) -> ActVstR {
        ActVstR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    pub fn act_hst(&mut self) -> ActHstW<'_, ActPosSpec> {
        ActHstW::new(self, 0)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn act_vst(&mut self) -> ActVstW<'_, ActPosSpec> {
        ActVstW::new(self, 16)
    }
}
/**Active area position setting register

You can [`read`](crate::Reg::read) this register and get [`act_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`act_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ActPosSpec;
impl crate::RegisterSpec for ActPosSpec {
    type Ux = u32;
}
///`read()` method returns [`act_pos::R`](R) reader structure
impl crate::Readable for ActPosSpec {}
///`write(|w| ..)` method takes [`act_pos::W`](W) writer structure
impl crate::Writable for ActPosSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACT_POS to value 0
impl crate::Resettable for ActPosSpec {}
