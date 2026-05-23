///Register `INTR_DISABLE` writer
pub type W = crate::W<IntrDisableSpec>;
///Field `END_DIS` writer - Done Interrupt Disable
pub type EndDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `RD_ERR_DIS` writer - Read Error Disable
pub type RdErrDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `WR_ERR_DIS` writer - Write Error Disable
pub type WrErrDisW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl W {
    ///Bit 0 - Done Interrupt Disable
    #[inline(always)]
    pub fn end_dis(&mut self) -> EndDisW<'_, IntrDisableSpec> {
        EndDisW::new(self, 0)
    }
    ///Bit 8 - Read Error Disable
    #[inline(always)]
    pub fn rd_err_dis(&mut self) -> RdErrDisW<'_, IntrDisableSpec> {
        RdErrDisW::new(self, 8)
    }
    ///Bit 9 - Write Error Disable
    #[inline(always)]
    pub fn wr_err_dis(&mut self) -> WrErrDisW<'_, IntrDisableSpec> {
        WrErrDisW::new(self, 9)
    }
}
/**Interrupt Disable

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_disable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntrDisableSpec;
impl crate::RegisterSpec for IntrDisableSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`intr_disable::W`](W) writer structure
impl crate::Writable for IntrDisableSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0301;
}
///`reset()` method sets INTR_DISABLE to value 0
impl crate::Resettable for IntrDisableSpec {}
