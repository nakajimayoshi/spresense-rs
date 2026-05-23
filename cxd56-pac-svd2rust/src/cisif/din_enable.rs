///Register `DIN_ENABLE` reader
pub type R = crate::R<DinEnableSpec>;
///Register `DIN_ENABLE` writer
pub type W = crate::W<DinEnableSpec>;
///Field `ycin_enable` reader -
pub type YcinEnableR = crate::BitReader;
///Field `ycin_enable` writer -
pub type YcinEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn ycin_enable(&self) -> YcinEnableR {
        YcinEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn ycin_enable(&mut self) -> YcinEnableW<'_, DinEnableSpec> {
        YcinEnableW::new(self, 0)
    }
}
/**Input data enable register

You can [`read`](crate::Reg::read) this register and get [`din_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DinEnableSpec;
impl crate::RegisterSpec for DinEnableSpec {
    type Ux = u32;
}
///`read()` method returns [`din_enable::R`](R) reader structure
impl crate::Readable for DinEnableSpec {}
///`write(|w| ..)` method takes [`din_enable::W`](W) writer structure
impl crate::Writable for DinEnableSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIN_ENABLE to value 0
impl crate::Resettable for DinEnableSpec {}
