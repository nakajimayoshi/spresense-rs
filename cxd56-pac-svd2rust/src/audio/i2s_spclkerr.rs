///Register `I2S_SPCLKERR` reader
pub type R = crate::R<I2sSpclkerrSpec>;
///Register `I2S_SPCLKERR` writer
pub type W = crate::W<I2sSpclkerrSpec>;
///Field `M_SPCLKERR1` reader - Mask I2S0 BCK/LRCK error interrupt (1=masked)
pub type MSpclkerr1R = crate::BitReader;
///Field `M_SPCLKERR1` writer - Mask I2S0 BCK/LRCK error interrupt (1=masked)
pub type MSpclkerr1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M_SPCLKERR2` reader - Mask I2S1 BCK/LRCK error interrupt (1=masked)
pub type MSpclkerr2R = crate::BitReader;
///Field `M_SPCLKERR2` writer - Mask I2S1 BCK/LRCK error interrupt (1=masked)
pub type MSpclkerr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Mask I2S0 BCK/LRCK error interrupt (1=masked)
    #[inline(always)]
    pub fn m_spclkerr1(&self) -> MSpclkerr1R {
        MSpclkerr1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mask I2S1 BCK/LRCK error interrupt (1=masked)
    #[inline(always)]
    pub fn m_spclkerr2(&self) -> MSpclkerr2R {
        MSpclkerr2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Mask I2S0 BCK/LRCK error interrupt (1=masked)
    #[inline(always)]
    pub fn m_spclkerr1(&mut self) -> MSpclkerr1W<'_, I2sSpclkerrSpec> {
        MSpclkerr1W::new(self, 0)
    }
    ///Bit 1 - Mask I2S1 BCK/LRCK error interrupt (1=masked)
    #[inline(always)]
    pub fn m_spclkerr2(&mut self) -> MSpclkerr2W<'_, I2sSpclkerrSpec> {
        MSpclkerr2W::new(self, 1)
    }
}
/**I2S BCK/LRCK error interrupt mask

You can [`read`](crate::Reg::read) this register and get [`i2s_spclkerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_spclkerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2sSpclkerrSpec;
impl crate::RegisterSpec for I2sSpclkerrSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s_spclkerr::R`](R) reader structure
impl crate::Readable for I2sSpclkerrSpec {}
///`write(|w| ..)` method takes [`i2s_spclkerr::W`](W) writer structure
impl crate::Writable for I2sSpclkerrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S_SPCLKERR to value 0
impl crate::Resettable for I2sSpclkerrSpec {}
