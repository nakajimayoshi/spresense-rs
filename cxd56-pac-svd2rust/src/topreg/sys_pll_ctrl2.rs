///Register `SYS_PLL_CTRL2` reader
pub type R = crate::R<SysPllCtrl2Spec>;
///Register `SYS_PLL_CTRL2` writer
pub type W = crate::W<SysPllCtrl2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**System PLL control 2 (division ratios)

You can [`read`](crate::Reg::read) this register and get [`sys_pll_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_pll_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SysPllCtrl2Spec;
impl crate::RegisterSpec for SysPllCtrl2Spec {
    type Ux = u32;
}
///`read()` method returns [`sys_pll_ctrl2::R`](R) reader structure
impl crate::Readable for SysPllCtrl2Spec {}
///`write(|w| ..)` method takes [`sys_pll_ctrl2::W`](W) writer structure
impl crate::Writable for SysPllCtrl2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYS_PLL_CTRL2 to value 0
impl crate::Resettable for SysPllCtrl2Spec {}
