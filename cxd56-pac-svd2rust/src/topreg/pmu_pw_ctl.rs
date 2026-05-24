///Register `PMU_PW_CTL` writer
pub type W = crate::W<PmuPwCtlSpec>;
///Field `POWER_CTRL_ON` writer - Write 1 to request a PMU power-state transition
pub type PowerCtrlOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    ///Bit 0 - Write 1 to request a PMU power-state transition
    #[inline(always)]
    pub fn power_ctrl_on(&mut self) -> PowerCtrlOnW<'_, PmuPwCtlSpec> {
        PowerCtrlOnW::new(self, 0)
    }
}
/**PMU power-supply control request (write-only kick register)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_pw_ctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PmuPwCtlSpec;
impl crate::RegisterSpec for PmuPwCtlSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pmu_pw_ctl::W`](W) writer structure
impl crate::Writable for PmuPwCtlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMU_PW_CTL to value 0
impl crate::Resettable for PmuPwCtlSpec {}
