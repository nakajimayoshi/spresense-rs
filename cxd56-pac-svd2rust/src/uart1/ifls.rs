///Register `IFLS` reader
pub type R = crate::R<IflsSpec>;
///Register `IFLS` writer
pub type W = crate::W<IflsSpec>;
/**Transmit interrupt FIFO level select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txiflsel {
    ///0: Receive FIFO becomes >= 1/8 full
    Fifo1_8Full = 0,
    ///1: Receive FIFO becomes >= 1/4 full
    Fifo1_4Full = 1,
    ///2: Receive FIFO becomes >= 1/2 full
    Fifo1_2Full = 2,
    ///3: Receive FIFO becomes >= 3/4 full
    Fifo3_4Full = 3,
    ///4: Receive FIFO becomes >= 7/8 full
    Fifo7_8Full = 4,
}
impl From<Txiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Txiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Txiflsel {}
///Field `TXIFLSEL` reader - Transmit interrupt FIFO level select
pub type TxiflselR = crate::FieldReader<Txiflsel>;
impl TxiflselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txiflsel> {
        match self.bits {
            0 => Some(Txiflsel::Fifo1_8Full),
            1 => Some(Txiflsel::Fifo1_4Full),
            2 => Some(Txiflsel::Fifo1_2Full),
            3 => Some(Txiflsel::Fifo3_4Full),
            4 => Some(Txiflsel::Fifo7_8Full),
            _ => None,
        }
    }
    ///Receive FIFO becomes >= 1/8 full
    #[inline(always)]
    pub fn is_fifo_1_8_full(&self) -> bool {
        *self == Txiflsel::Fifo1_8Full
    }
    ///Receive FIFO becomes >= 1/4 full
    #[inline(always)]
    pub fn is_fifo_1_4_full(&self) -> bool {
        *self == Txiflsel::Fifo1_4Full
    }
    ///Receive FIFO becomes >= 1/2 full
    #[inline(always)]
    pub fn is_fifo_1_2_full(&self) -> bool {
        *self == Txiflsel::Fifo1_2Full
    }
    ///Receive FIFO becomes >= 3/4 full
    #[inline(always)]
    pub fn is_fifo_3_4_full(&self) -> bool {
        *self == Txiflsel::Fifo3_4Full
    }
    ///Receive FIFO becomes >= 7/8 full
    #[inline(always)]
    pub fn is_fifo_7_8_full(&self) -> bool {
        *self == Txiflsel::Fifo7_8Full
    }
}
///Field `TXIFLSEL` writer - Transmit interrupt FIFO level select
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txiflsel>;
impl<'a, REG> TxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Receive FIFO becomes >= 1/8 full
    #[inline(always)]
    pub fn fifo_1_8_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Fifo1_8Full)
    }
    ///Receive FIFO becomes >= 1/4 full
    #[inline(always)]
    pub fn fifo_1_4_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Fifo1_4Full)
    }
    ///Receive FIFO becomes >= 1/2 full
    #[inline(always)]
    pub fn fifo_1_2_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Fifo1_2Full)
    }
    ///Receive FIFO becomes >= 3/4 full
    #[inline(always)]
    pub fn fifo_3_4_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Fifo3_4Full)
    }
    ///Receive FIFO becomes >= 7/8 full
    #[inline(always)]
    pub fn fifo_7_8_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Fifo7_8Full)
    }
}
/**Receive interrupt FIFO level select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxiflsel {
    ///0: Receive FIFO becomes >= 1/8 full
    Fifo1_8Full = 0,
    ///1: Receive FIFO becomes >= 1/4 full
    Fifo1_4Full = 1,
    ///2: Receive FIFO becomes >= 1/2 full
    Fifo1_2Full = 2,
    ///3: Receive FIFO becomes >= 3/4 full
    Fifo3_4Full = 3,
    ///4: Receive FIFO becomes >= 7/8 full
    Fifo7_8Full = 4,
}
impl From<Rxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Rxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Rxiflsel {}
///Field `RXIFLSEL` reader - Receive interrupt FIFO level select
pub type RxiflselR = crate::FieldReader<Rxiflsel>;
impl RxiflselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxiflsel> {
        match self.bits {
            0 => Some(Rxiflsel::Fifo1_8Full),
            1 => Some(Rxiflsel::Fifo1_4Full),
            2 => Some(Rxiflsel::Fifo1_2Full),
            3 => Some(Rxiflsel::Fifo3_4Full),
            4 => Some(Rxiflsel::Fifo7_8Full),
            _ => None,
        }
    }
    ///Receive FIFO becomes >= 1/8 full
    #[inline(always)]
    pub fn is_fifo_1_8_full(&self) -> bool {
        *self == Rxiflsel::Fifo1_8Full
    }
    ///Receive FIFO becomes >= 1/4 full
    #[inline(always)]
    pub fn is_fifo_1_4_full(&self) -> bool {
        *self == Rxiflsel::Fifo1_4Full
    }
    ///Receive FIFO becomes >= 1/2 full
    #[inline(always)]
    pub fn is_fifo_1_2_full(&self) -> bool {
        *self == Rxiflsel::Fifo1_2Full
    }
    ///Receive FIFO becomes >= 3/4 full
    #[inline(always)]
    pub fn is_fifo_3_4_full(&self) -> bool {
        *self == Rxiflsel::Fifo3_4Full
    }
    ///Receive FIFO becomes >= 7/8 full
    #[inline(always)]
    pub fn is_fifo_7_8_full(&self) -> bool {
        *self == Rxiflsel::Fifo7_8Full
    }
}
///Field `RXIFLSEL` writer - Receive interrupt FIFO level select
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxiflsel>;
impl<'a, REG> RxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Receive FIFO becomes >= 1/8 full
    #[inline(always)]
    pub fn fifo_1_8_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Fifo1_8Full)
    }
    ///Receive FIFO becomes >= 1/4 full
    #[inline(always)]
    pub fn fifo_1_4_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Fifo1_4Full)
    }
    ///Receive FIFO becomes >= 1/2 full
    #[inline(always)]
    pub fn fifo_1_2_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Fifo1_2Full)
    }
    ///Receive FIFO becomes >= 3/4 full
    #[inline(always)]
    pub fn fifo_3_4_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Fifo3_4Full)
    }
    ///Receive FIFO becomes >= 7/8 full
    #[inline(always)]
    pub fn fifo_7_8_full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Fifo7_8Full)
    }
}
impl R {
    ///Bits 0:2 - Transmit interrupt FIFO level select
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Receive interrupt FIFO level select
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Transmit interrupt FIFO level select
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TxiflselW<'_, IflsSpec> {
        TxiflselW::new(self, 0)
    }
    ///Bits 3:5 - Receive interrupt FIFO level select
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RxiflselW<'_, IflsSpec> {
        RxiflselW::new(self, 3)
    }
}
/**Interrupt FIFO Level Select Register

You can [`read`](crate::Reg::read) this register and get [`ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
///`read()` method returns [`ifls::R`](R) reader structure
impl crate::Readable for IflsSpec {}
///`write(|w| ..)` method takes [`ifls::W`](W) writer structure
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFLS to value 0
impl crate::Resettable for IflsSpec {}
