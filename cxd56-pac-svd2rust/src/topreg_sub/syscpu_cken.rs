///Register `SYSCPU_CKEN` reader
pub type R = crate::R<SyscpuCkenSpec>;
///Register `SYSCPU_CKEN` writer
pub type W = crate::W<SyscpuCkenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**System CPU clock enable

You can [`read`](crate::Reg::read) this register and get [`syscpu_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscpu_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SyscpuCkenSpec;
impl crate::RegisterSpec for SyscpuCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`syscpu_cken::R`](R) reader structure
impl crate::Readable for SyscpuCkenSpec {}
///`write(|w| ..)` method takes [`syscpu_cken::W`](W) writer structure
impl crate::Writable for SyscpuCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCPU_CKEN to value 0
impl crate::Resettable for SyscpuCkenSpec {}
