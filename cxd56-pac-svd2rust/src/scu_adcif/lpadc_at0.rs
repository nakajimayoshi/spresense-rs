///Register `LPADC_AT0` reader
pub type R = crate::R<LpadcAt0Spec>;
///Register `LPADC_AT0` writer
pub type W = crate::W<LpadcAt0Spec>;
///Field `TIMING0` reader - Analog timing control bit 0 (SDK always writes 0)
pub type Timing0R = crate::BitReader;
///Field `TIMING0` writer - Analog timing control bit 0 (SDK always writes 0)
pub type Timing0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Analog timing control bit 0 (SDK always writes 0)
    #[inline(always)]
    pub fn timing0(&self) -> Timing0R {
        Timing0R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Analog timing control bit 0 (SDK always writes 0)
    #[inline(always)]
    pub fn timing0(&mut self) -> Timing0W<'_, LpadcAt0Spec> {
        Timing0W::new(self, 0)
    }
}
/**LPADC analog timing register 0 (bit0 must be cleared, cxd56_adc.c:469)

You can [`read`](crate::Reg::read) this register and get [`lpadc_at0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_at0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpadcAt0Spec;
impl crate::RegisterSpec for LpadcAt0Spec {
    type Ux = u32;
}
///`read()` method returns [`lpadc_at0::R`](R) reader structure
impl crate::Readable for LpadcAt0Spec {}
///`write(|w| ..)` method takes [`lpadc_at0::W`](W) writer structure
impl crate::Writable for LpadcAt0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPADC_AT0 to value 0
impl crate::Resettable for LpadcAt0Spec {}
