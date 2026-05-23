///Register `CPU_GATECLK` reader
pub type R = crate::R<CpuGateclkSpec>;
///Register `CPU_GATECLK` writer
pub type W = crate::W<CpuGateclkSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CPU clock gating control

You can [`read`](crate::Reg::read) this register and get [`cpu_gateclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_gateclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CpuGateclkSpec;
impl crate::RegisterSpec for CpuGateclkSpec {
    type Ux = u32;
}
///`read()` method returns [`cpu_gateclk::R`](R) reader structure
impl crate::Readable for CpuGateclkSpec {}
///`write(|w| ..)` method takes [`cpu_gateclk::W`](W) writer structure
impl crate::Writable for CpuGateclkSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPU_GATECLK to value 0
impl crate::Resettable for CpuGateclkSpec {}
