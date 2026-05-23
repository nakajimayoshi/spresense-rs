///Register `CKDIV_CPU_DSP_BUS` reader
pub type R = crate::R<CkdivCpuDspBusSpec>;
///Register `CKDIV_CPU_DSP_BUS` writer
pub type W = crate::W<CkdivCpuDspBusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CPU / DSP / bus clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_cpu_dsp_bus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_cpu_dsp_bus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CkdivCpuDspBusSpec;
impl crate::RegisterSpec for CkdivCpuDspBusSpec {
    type Ux = u32;
}
///`read()` method returns [`ckdiv_cpu_dsp_bus::R`](R) reader structure
impl crate::Readable for CkdivCpuDspBusSpec {}
///`write(|w| ..)` method takes [`ckdiv_cpu_dsp_bus::W`](W) writer structure
impl crate::Writable for CkdivCpuDspBusSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKDIV_CPU_DSP_BUS to value 0
impl crate::Resettable for CkdivCpuDspBusSpec {}
