///Register `CKDIV_CPU_DSP_BUS_WAKE` reader
pub type R = crate::R<CkdivCpuDspBusWakeSpec>;
///Register `CKDIV_CPU_DSP_BUS_WAKE` writer
pub type W = crate::W<CkdivCpuDspBusWakeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CPU/DSP/bus clock divider used on wake from sleep

You can [`read`](crate::Reg::read) this register and get [`ckdiv_cpu_dsp_bus_wake::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_cpu_dsp_bus_wake::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkdivCpuDspBusWakeSpec;
impl crate::RegisterSpec for CkdivCpuDspBusWakeSpec {
    type Ux = u32;
}
///`read()` method returns [`ckdiv_cpu_dsp_bus_wake::R`](R) reader structure
impl crate::Readable for CkdivCpuDspBusWakeSpec {}
///`write(|w| ..)` method takes [`ckdiv_cpu_dsp_bus_wake::W`](W) writer structure
impl crate::Writable for CkdivCpuDspBusWakeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKDIV_CPU_DSP_BUS_WAKE to value 0
impl crate::Resettable for CkdivCpuDspBusWakeSpec {}
