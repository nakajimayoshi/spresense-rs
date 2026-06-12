///Register `INT_CLR` reader
pub type R = crate::R<IntClrSpec>;
///Register `INT_CLR` writer
pub type W = crate::W<IntClrSpec>;
///Field `HRESP_ERR` reader - Clear AHB bus (HRESP) error interrupt
pub type HrespErrR = crate::BitReader;
///Field `HRESP_ERR` writer - Clear AHB bus (HRESP) error interrupt
pub type HrespErrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S1_BCK_ERR1` reader - Clear I2S0 BCK error 1 interrupt
pub type I2s1BckErr1R = crate::BitReader;
///Field `I2S1_BCK_ERR1` writer - Clear I2S0 BCK error 1 interrupt
pub type I2s1BckErr1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2S1_BCK_ERR2` reader - Clear I2S0 BCK error 2 interrupt
pub type I2s1BckErr2R = crate::BitReader;
///Field `I2S1_BCK_ERR2` writer - Clear I2S0 BCK error 2 interrupt
pub type I2s1BckErr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear AHB bus (HRESP) error interrupt
    #[inline(always)]
    pub fn hresp_err(&self) -> HrespErrR {
        HrespErrR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Clear I2S0 BCK error 1 interrupt
    #[inline(always)]
    pub fn i2s1_bck_err1(&self) -> I2s1BckErr1R {
        I2s1BckErr1R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clear I2S0 BCK error 2 interrupt
    #[inline(always)]
    pub fn i2s1_bck_err2(&self) -> I2s1BckErr2R {
        I2s1BckErr2R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear AHB bus (HRESP) error interrupt
    #[inline(always)]
    pub fn hresp_err(&mut self) -> HrespErrW<'_, IntClrSpec> {
        HrespErrW::new(self, 0)
    }
    ///Bit 8 - Clear I2S0 BCK error 1 interrupt
    #[inline(always)]
    pub fn i2s1_bck_err1(&mut self) -> I2s1BckErr1W<'_, IntClrSpec> {
        I2s1BckErr1W::new(self, 8)
    }
    ///Bit 9 - Clear I2S0 BCK error 2 interrupt
    #[inline(always)]
    pub fn i2s1_bck_err2(&mut self) -> I2s1BckErr2W<'_, IntClrSpec> {
        I2s1BckErr2W::new(self, 9)
    }
}
/**Audio bus/BCK error interrupt clear

You can [`read`](crate::Reg::read) this register and get [`int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
///`read()` method returns [`int_clr::R`](R) reader structure
impl crate::Readable for IntClrSpec {}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for IntClrSpec {}
