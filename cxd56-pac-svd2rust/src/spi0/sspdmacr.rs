///Register `SSPDMACR` reader
pub type R = crate::R<SspdmacrSpec>;
///Register `SSPDMACR` writer
pub type W = crate::W<SspdmacrSpec>;
/**Receive DMA enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmae {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Rxdmae> for bool {
    #[inline(always)]
    fn from(variant: Rxdmae) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAE` reader - Receive DMA enable
pub type RxdmaeR = crate::BitReader<Rxdmae>;
impl RxdmaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmae {
        match self.bits {
            false => Rxdmae::Disabled,
            true => Rxdmae::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdmae::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdmae::Enabled
    }
}
///Field `RXDMAE` writer - Receive DMA enable
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG, Rxdmae>;
impl<'a, REG> RxdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmae::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmae::Enabled)
    }
}
/**Transmit DMA enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmae {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Txdmae> for bool {
    #[inline(always)]
    fn from(variant: Txdmae) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAE` reader - Transmit DMA enable
pub type TxdmaeR = crate::BitReader<Txdmae>;
impl TxdmaeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txdmae {
        match self.bits {
            false => Txdmae::Disabled,
            true => Txdmae::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdmae::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdmae::Enabled
    }
}
///Field `TXDMAE` writer - Transmit DMA enable
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG, Txdmae>;
impl<'a, REG> TxdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmae::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmae::Enabled)
    }
}
impl R {
    ///Bit 0 - Receive DMA enable
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit DMA enable
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive DMA enable
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RxdmaeW<'_, SspdmacrSpec> {
        RxdmaeW::new(self, 0)
    }
    ///Bit 1 - Transmit DMA enable
    #[inline(always)]
    pub fn txdmae(&mut self) -> TxdmaeW<'_, SspdmacrSpec> {
        TxdmaeW::new(self, 1)
    }
}
/**DMA control register

You can [`read`](crate::Reg::read) this register and get [`sspdmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspdmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SspdmacrSpec;
impl crate::RegisterSpec for SspdmacrSpec {
    type Ux = u32;
}
///`read()` method returns [`sspdmacr::R`](R) reader structure
impl crate::Readable for SspdmacrSpec {}
///`write(|w| ..)` method takes [`sspdmacr::W`](W) writer structure
impl crate::Writable for SspdmacrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSPDMACR to value 0
impl crate::Resettable for SspdmacrSpec {}
