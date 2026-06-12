///Register `INT_M` reader
pub type R = crate::R<IntMSpec>;
///Register `INT_M` writer
pub type W = crate::W<IntMSpec>;
///Field `HRESP_ERR` reader - Mask AHB bus (HRESP) error interrupt
pub type HrespErrR = crate::BitReader;
///Field `HRESP_ERR` writer - Mask AHB bus (HRESP) error interrupt
pub type HrespErrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S1_BCK_ERR1` reader - Mask I2S0 BCK error 1 interrupt
pub type I2s1BckErr1R = crate::BitReader;
///Field `I2S1_BCK_ERR1` writer - Mask I2S0 BCK error 1 interrupt
pub type I2s1BckErr1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S1_BCK_ERR2` reader - Mask I2S0 BCK error 2 interrupt
pub type I2s1BckErr2R = crate::BitReader;
///Field `I2S1_BCK_ERR2` writer - Mask I2S0 BCK error 2 interrupt
pub type I2s1BckErr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Mask AHB bus (HRESP) error interrupt
    #[inline(always)]
    pub fn hresp_err(&self) -> HrespErrR {
        HrespErrR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Mask I2S0 BCK error 1 interrupt
    #[inline(always)]
    pub fn i2s1_bck_err1(&self) -> I2s1BckErr1R {
        I2s1BckErr1R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Mask I2S0 BCK error 2 interrupt
    #[inline(always)]
    pub fn i2s1_bck_err2(&self) -> I2s1BckErr2R {
        I2s1BckErr2R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Mask AHB bus (HRESP) error interrupt
    #[inline(always)]
    pub fn hresp_err(&mut self) -> HrespErrW<'_, IntMSpec> {
        HrespErrW::new(self, 0)
    }
    ///Bit 8 - Mask I2S0 BCK error 1 interrupt
    #[inline(always)]
    pub fn i2s1_bck_err1(&mut self) -> I2s1BckErr1W<'_, IntMSpec> {
        I2s1BckErr1W::new(self, 8)
    }
    ///Bit 9 - Mask I2S0 BCK error 2 interrupt
    #[inline(always)]
    pub fn i2s1_bck_err2(&mut self) -> I2s1BckErr2W<'_, IntMSpec> {
        I2s1BckErr2W::new(self, 9)
    }
}
/**Audio bus/BCK error interrupt mask (1=masked)

You can [`read`](crate::Reg::read) this register and get [`int_m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntMSpec;
impl crate::RegisterSpec for IntMSpec {
    type Ux = u32;
}
///`read()` method returns [`int_m::R`](R) reader structure
impl crate::Readable for IntMSpec {}
///`write(|w| ..)` method takes [`int_m::W`](W) writer structure
impl crate::Writable for IntMSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INT_M to value 0x01e6_0701
impl crate::Resettable for IntMSpec {
    const RESET_VALUE: u32 = 0x01e6_0701;
}
