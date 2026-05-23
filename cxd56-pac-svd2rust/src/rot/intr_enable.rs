///Register `INTR_ENABLE` reader
pub type R = crate::R<IntrEnableSpec>;
///Register `INTR_ENABLE` writer
pub type W = crate::W<IntrEnableSpec>;
///Done Interrupt Enable
pub use WrErrEnb as EndEnb;
///Read Error
pub use WrErrEnb as RdErrEnb;
///Field `END_ENB` reader - Done Interrupt Enable
pub use WrErrEnbR as EndEnbR;
///Field `RD_ERR_ENB` reader - Read Error
pub use WrErrEnbR as RdErrEnbR;
///Field `END_ENB` writer - Done Interrupt Enable
pub use WrErrEnbW as EndEnbW;
///Field `RD_ERR_ENB` writer - Read Error
pub use WrErrEnbW as RdErrEnbW;
/**Write Error Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrErrEnb {
    ///0: `0`
    Disable = 0,
    ///1: `1`
    Enable = 1,
}
impl From<WrErrEnb> for bool {
    #[inline(always)]
    fn from(variant: WrErrEnb) -> Self {
        variant as u8 != 0
    }
}
///Field `WR_ERR_ENB` reader - Write Error Enable
pub type WrErrEnbR = crate::BitReader<WrErrEnb>;
impl WrErrEnbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WrErrEnb {
        match self.bits {
            false => WrErrEnb::Disable,
            true => WrErrEnb::Enable,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WrErrEnb::Disable
    }
    ///`1`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WrErrEnb::Enable
    }
}
///Field `WR_ERR_ENB` writer - Write Error Enable
pub type WrErrEnbW<'a, REG> = crate::BitWriter<'a, REG, WrErrEnb>;
impl<'a, REG> WrErrEnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///`0`
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WrErrEnb::Disable)
    }
    ///`1`
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WrErrEnb::Enable)
    }
}
impl R {
    ///Bit 0 - Done Interrupt Enable
    #[inline(always)]
    pub fn end_enb(&self) -> EndEnbR {
        EndEnbR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Read Error
    #[inline(always)]
    pub fn rd_err_enb(&self) -> RdErrEnbR {
        RdErrEnbR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Write Error Enable
    #[inline(always)]
    pub fn wr_err_enb(&self) -> WrErrEnbR {
        WrErrEnbR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Done Interrupt Enable
    #[inline(always)]
    pub fn end_enb(&mut self) -> EndEnbW<'_, IntrEnableSpec> {
        EndEnbW::new(self, 0)
    }
    ///Bit 8 - Read Error
    #[inline(always)]
    pub fn rd_err_enb(&mut self) -> RdErrEnbW<'_, IntrEnableSpec> {
        RdErrEnbW::new(self, 8)
    }
    ///Bit 9 - Write Error Enable
    #[inline(always)]
    pub fn wr_err_enb(&mut self) -> WrErrEnbW<'_, IntrEnableSpec> {
        WrErrEnbW::new(self, 9)
    }
}
/**Interrupt Enable

You can [`read`](crate::Reg::read) this register and get [`intr_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntrEnableSpec;
impl crate::RegisterSpec for IntrEnableSpec {
    type Ux = u32;
}
///`read()` method returns [`intr_enable::R`](R) reader structure
impl crate::Readable for IntrEnableSpec {}
///`write(|w| ..)` method takes [`intr_enable::W`](W) writer structure
impl crate::Writable for IntrEnableSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTR_ENABLE to value 0
impl crate::Resettable for IntrEnableSpec {}
