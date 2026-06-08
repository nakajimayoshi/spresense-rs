///Register `LPADC_A1` reader
pub type R = crate::R<LpadcA1Spec>;
///Register `LPADC_A1` writer
pub type W = crate::W<LpadcA1Spec>;
///Field `LV_CH_SEL_MODE` reader - Channel switching mode
pub type LvChSelModeR = crate::FieldReader;
///Field `LV_CH_SEL_MODE` writer - Channel switching mode
pub type LvChSelModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LV_CH_SEL_INV` reader - ADCH selector for GNSS (active channel: 0=CH0 … 3=CH3)
pub type LvChSelInvR = crate::FieldReader;
///Field `LV_CH_SEL_INV` writer - ADCH selector for GNSS (active channel: 0=CH0 … 3=CH3)
pub type LvChSelInvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Channel switching mode
    #[inline(always)]
    pub fn lv_ch_sel_mode(&self) -> LvChSelModeR {
        LvChSelModeR::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - ADCH selector for GNSS (active channel: 0=CH0 … 3=CH3)
    #[inline(always)]
    pub fn lv_ch_sel_inv(&self) -> LvChSelInvR {
        LvChSelInvR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - Channel switching mode
    #[inline(always)]
    pub fn lv_ch_sel_mode(&mut self) -> LvChSelModeW<'_, LpadcA1Spec> {
        LvChSelModeW::new(self, 0)
    }
    ///Bits 8:9 - ADCH selector for GNSS (active channel: 0=CH0 … 3=CH3)
    #[inline(always)]
    pub fn lv_ch_sel_inv(&mut self) -> LvChSelInvW<'_, LpadcA1Spec> {
        LvChSelInvW::new(self, 8)
    }
}
/**LPADC Channel Switching Control (UM Table ADC-107)

You can [`read`](crate::Reg::read) this register and get [`lpadc_a1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_a1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpadcA1Spec;
impl crate::RegisterSpec for LpadcA1Spec {
    type Ux = u32;
}
///`read()` method returns [`lpadc_a1::R`](R) reader structure
impl crate::Readable for LpadcA1Spec {}
///`write(|w| ..)` method takes [`lpadc_a1::W`](W) writer structure
impl crate::Writable for LpadcA1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPADC_A1 to value 0
impl crate::Resettable for LpadcA1Spec {}
