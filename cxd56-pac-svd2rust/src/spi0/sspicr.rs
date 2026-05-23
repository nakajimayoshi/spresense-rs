///Register `SSPICR` writer
pub type W = crate::W<SspicrSpec>;
///Field `RORIC` writer - Clear the SSPRORINTR interrupt
pub type RoricW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTIC` writer - Clear the SSPRTINTR interrupt
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    ///Bit 0 - Clear the SSPRORINTR interrupt
    #[inline(always)]
    pub fn roric(&mut self) -> RoricW<'_, SspicrSpec> {
        RoricW::new(self, 0)
    }
    ///Bit 1 - Clear the SSPRTINTR interrupt
    #[inline(always)]
    pub fn rtic(&mut self) -> RticW<'_, SspicrSpec> {
        RticW::new(self, 1)
    }
}
/**Interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SspicrSpec;
impl crate::RegisterSpec for SspicrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`sspicr::W`](W) writer structure
impl crate::Writable for SspicrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSPICR to value 0
impl crate::Resettable for SspicrSpec {}
