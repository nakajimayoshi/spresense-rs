///Register `LPADC_D0` reader
pub type R = crate::R<LpadcD0Spec>;
///Register `LPADC_D0` writer
pub type W = crate::W<LpadcD0Spec>;
///Field `SW_RESET` reader - LPADC software reset (write 1 to reset, then clear)
pub type SwResetR = crate::BitReader;
///Field `SW_RESET` writer - LPADC software reset (write 1 to reset, then clear)
pub type SwResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPADC software reset (write 1 to reset, then clear)
    #[inline(always)]
    pub fn sw_reset(&self) -> SwResetR {
        SwResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LPADC software reset (write 1 to reset, then clear)
    #[inline(always)]
    pub fn sw_reset(&mut self) -> SwResetW<'_, LpadcD0Spec> {
        SwResetW::new(self, 0)
    }
}
/**LPADC Software Reset, common to all channels (UM Table ADC-108)

You can [`read`](crate::Reg::read) this register and get [`lpadc_d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpadcD0Spec;
impl crate::RegisterSpec for LpadcD0Spec {
    type Ux = u32;
}
///`read()` method returns [`lpadc_d0::R`](R) reader structure
impl crate::Readable for LpadcD0Spec {}
///`write(|w| ..)` method takes [`lpadc_d0::W`](W) writer structure
impl crate::Writable for LpadcD0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPADC_D0 to value 0
impl crate::Resettable for LpadcD0Spec {}
