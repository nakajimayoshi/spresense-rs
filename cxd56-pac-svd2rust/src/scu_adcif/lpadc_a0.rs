///Register `LPADC_A0` reader
pub type R = crate::R<LpadcA0Spec>;
///Register `LPADC_A0` writer
pub type W = crate::W<LpadcA0Spec>;
///Field `LV_ADC_EN` reader - LPADC enable: 0=standby, 1=active
pub type LvAdcEnR = crate::BitReader;
///Field `LV_ADC_EN` writer - LPADC enable: 0=standby, 1=active
pub type LvAdcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPADC enable: 0=standby, 1=active
    #[inline(always)]
    pub fn lv_adc_en(&self) -> LvAdcEnR {
        LvAdcEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LPADC enable: 0=standby, 1=active
    #[inline(always)]
    pub fn lv_adc_en(&mut self) -> LvAdcEnW<'_, LpadcA0Spec> {
        LvAdcEnW::new(self, 0)
    }
}
/**LPADC Enable Control — LV_ADC_EN (UM Table ADC-106)

You can [`read`](crate::Reg::read) this register and get [`lpadc_a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpadcA0Spec;
impl crate::RegisterSpec for LpadcA0Spec {
    type Ux = u32;
}
///`read()` method returns [`lpadc_a0::R`](R) reader structure
impl crate::Readable for LpadcA0Spec {}
///`write(|w| ..)` method takes [`lpadc_a0::W`](W) writer structure
impl crate::Writable for LpadcA0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPADC_A0 to value 0
impl crate::Resettable for LpadcA0Spec {}
