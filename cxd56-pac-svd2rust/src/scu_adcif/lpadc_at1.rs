///Register `LPADC_AT1` reader
pub type R = crate::R<LpadcAt1Spec>;
///Register `LPADC_AT1` writer
pub type W = crate::W<LpadcAt1Spec>;
///Field `WARMUP` reader - Analog warm-up count (SDK: 0x04)
pub type WarmupR = crate::FieldReader;
///Field `WARMUP` writer - Analog warm-up count (SDK: 0x04)
pub type WarmupW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Analog warm-up count (SDK: 0x04)
    #[inline(always)]
    pub fn warmup(&self) -> WarmupR {
        WarmupR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Analog warm-up count (SDK: 0x04)
    #[inline(always)]
    pub fn warmup(&mut self) -> WarmupW<'_, LpadcAt1Spec> {
        WarmupW::new(self, 0)
    }
}
/**LPADC analog warm-up timing (SDK sets bits\[7:0\]=0x04, cxd56_adc.c:462)

You can [`read`](crate::Reg::read) this register and get [`lpadc_at1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpadc_at1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LpadcAt1Spec;
impl crate::RegisterSpec for LpadcAt1Spec {
    type Ux = u32;
}
///`read()` method returns [`lpadc_at1::R`](R) reader structure
impl crate::Readable for LpadcAt1Spec {}
///`write(|w| ..)` method takes [`lpadc_at1::W`](W) writer structure
impl crate::Writable for LpadcAt1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPADC_AT1 to value 0
impl crate::Resettable for LpadcAt1Spec {}
