///Register `INTR_CLEAR` writer
pub type W = crate::W<IntrClearSpec>;
///Field `END_CLR` writer - Done Interrupt Clear
pub type EndClrW<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `RD_ERR_CLR` writer - Read Error Clear
pub type RdErrClrW<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `WR_ERR_CLR` writer - Write Error Clear
pub type WrErrClrW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl W {
    ///Bit 0 - Done Interrupt Clear
    #[inline(always)]
    pub fn end_clr(&mut self) -> EndClrW<'_, IntrClearSpec> {
        EndClrW::new(self, 0)
    }
    ///Bit 8 - Read Error Clear
    #[inline(always)]
    pub fn rd_err_clr(&mut self) -> RdErrClrW<'_, IntrClearSpec> {
        RdErrClrW::new(self, 8)
    }
    ///Bit 9 - Write Error Clear
    #[inline(always)]
    pub fn wr_err_clr(&mut self) -> WrErrClrW<'_, IntrClearSpec> {
        WrErrClrW::new(self, 9)
    }
}
/**Interrupt Clear

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntrClearSpec;
impl crate::RegisterSpec for IntrClearSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`intr_clear::W`](W) writer structure
impl crate::Writable for IntrClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0301;
}
///`reset()` method sets INTR_CLEAR to value 0
impl crate::Resettable for IntrClearSpec {}
