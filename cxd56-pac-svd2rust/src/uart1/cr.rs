///Register `CR` reader
pub type R = crate::R<CrSpec>;
///Register `CR` writer
pub type W = crate::W<CrSpec>;
/**UART enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uarten {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Uarten> for bool {
    #[inline(always)]
    fn from(variant: Uarten) -> Self {
        variant as u8 != 0
    }
}
///Field `UARTEN` reader - UART enable
pub type UartenR = crate::BitReader<Uarten>;
impl UartenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Uarten {
        match self.bits {
            false => Uarten::Disabled,
            true => Uarten::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Uarten::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Uarten::Enabled
    }
}
///Field `UARTEN` writer - UART enable
pub type UartenW<'a, REG> = crate::BitWriter<'a, REG, Uarten>;
impl<'a, REG> UartenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Uarten::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Uarten::Enabled)
    }
}
/**SIR enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Siren {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Siren> for bool {
    #[inline(always)]
    fn from(variant: Siren) -> Self {
        variant as u8 != 0
    }
}
///Field `SIREN` reader - SIR enable
pub type SirenR = crate::BitReader<Siren>;
impl SirenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Siren {
        match self.bits {
            false => Siren::Disabled,
            true => Siren::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Siren::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Siren::Enabled
    }
}
///Field `SIREN` writer - SIR enable
pub type SirenW<'a, REG> = crate::BitWriter<'a, REG, Siren>;
impl<'a, REG> SirenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Siren::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Siren::Enabled)
    }
}
/**SIR low-power IrDA

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sirlp {
    ///0: Normal
    Normal = 0,
    ///1: Low-power
    LowPower = 1,
}
impl From<Sirlp> for bool {
    #[inline(always)]
    fn from(variant: Sirlp) -> Self {
        variant as u8 != 0
    }
}
///Field `SIRLP` reader - SIR low-power IrDA
pub type SirlpR = crate::BitReader<Sirlp>;
impl SirlpR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sirlp {
        match self.bits {
            false => Sirlp::Normal,
            true => Sirlp::LowPower,
        }
    }
    ///Normal
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Sirlp::Normal
    }
    ///Low-power
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Sirlp::LowPower
    }
}
///Field `SIRLP` writer - SIR low-power IrDA
pub type SirlpW<'a, REG> = crate::BitWriter<'a, REG, Sirlp>;
impl<'a, REG> SirlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Sirlp::Normal)
    }
    ///Low-power
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Sirlp::LowPower)
    }
}
/**Loopback enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbe {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Lbe> for bool {
    #[inline(always)]
    fn from(variant: Lbe) -> Self {
        variant as u8 != 0
    }
}
///Field `LBE` reader - Loopback enable
pub type LbeR = crate::BitReader<Lbe>;
impl LbeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Lbe {
        match self.bits {
            false => Lbe::Disabled,
            true => Lbe::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lbe::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lbe::Enabled
    }
}
///Field `LBE` writer - Loopback enable
pub type LbeW<'a, REG> = crate::BitWriter<'a, REG, Lbe>;
impl<'a, REG> LbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbe::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbe::Enabled)
    }
}
/**Transmit enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txe {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Txe> for bool {
    #[inline(always)]
    fn from(variant: Txe) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - Transmit enable
pub type TxeR = crate::BitReader<Txe>;
impl TxeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Txe {
        match self.bits {
            false => Txe::Disabled,
            true => Txe::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txe::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txe::Enabled
    }
}
///Field `TXE` writer - Transmit enable
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG, Txe>;
impl<'a, REG> TxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::Enabled)
    }
}
/**Receive enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxe {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Rxe> for bool {
    #[inline(always)]
    fn from(variant: Rxe) -> Self {
        variant as u8 != 0
    }
}
///Field `RXE` reader - Receive enable
pub type RxeR = crate::BitReader<Rxe>;
impl RxeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rxe {
        match self.bits {
            false => Rxe::Disabled,
            true => Rxe::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxe::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxe::Enabled
    }
}
///Field `RXE` writer - Receive enable
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG, Rxe>;
impl<'a, REG> RxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxe::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxe::Enabled)
    }
}
/**Data transmit ready

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtr {
    ///0: Low
    Low = 0,
    ///1: High
    High = 1,
}
impl From<Dtr> for bool {
    #[inline(always)]
    fn from(variant: Dtr) -> Self {
        variant as u8 != 0
    }
}
///Field `DTR` reader - Data transmit ready
pub type DtrR = crate::BitReader<Dtr>;
impl DtrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dtr {
        match self.bits {
            false => Dtr::Low,
            true => Dtr::High,
        }
    }
    ///Low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Dtr::Low
    }
    ///High
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Dtr::High
    }
}
///Field `DTR` writer - Data transmit ready
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG, Dtr>;
impl<'a, REG> DtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Dtr::Low)
    }
    ///High
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Dtr::High)
    }
}
/**Request to send

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rts {
    ///0: Low
    Low = 0,
    ///1: High
    High = 1,
}
impl From<Rts> for bool {
    #[inline(always)]
    fn from(variant: Rts) -> Self {
        variant as u8 != 0
    }
}
///Field `RTS` reader - Request to send
pub type RtsR = crate::BitReader<Rts>;
impl RtsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rts {
        match self.bits {
            false => Rts::Low,
            true => Rts::High,
        }
    }
    ///Low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Rts::Low
    }
    ///High
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Rts::High
    }
}
///Field `RTS` writer - Request to send
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG, Rts>;
impl<'a, REG> RtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Rts::Low)
    }
    ///High
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Rts::High)
    }
}
/**nUARTOut1 modem status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out1 {
    ///0: Output zero
    Zero = 0,
    ///1: Output one
    One = 1,
}
impl From<Out1> for bool {
    #[inline(always)]
    fn from(variant: Out1) -> Self {
        variant as u8 != 0
    }
}
///Field `Out1` reader - nUARTOut1 modem status
pub type Out1R = crate::BitReader<Out1>;
impl Out1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Out1 {
        match self.bits {
            false => Out1::Zero,
            true => Out1::One,
        }
    }
    ///Output zero
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Out1::Zero
    }
    ///Output one
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Out1::One
    }
}
///Field `Out1` writer - nUARTOut1 modem status
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG, Out1>;
impl<'a, REG> Out1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output zero
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Zero)
    }
    ///Output one
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::One)
    }
}
/**nUARTOut2 modem status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out2 {
    ///0: Output zero
    Zero = 0,
    ///1: Output one
    One = 1,
}
impl From<Out2> for bool {
    #[inline(always)]
    fn from(variant: Out2) -> Self {
        variant as u8 != 0
    }
}
///Field `Out2` reader - nUARTOut2 modem status
pub type Out2R = crate::BitReader<Out2>;
impl Out2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Out2 {
        match self.bits {
            false => Out2::Zero,
            true => Out2::One,
        }
    }
    ///Output zero
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Out2::Zero
    }
    ///Output one
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Out2::One
    }
}
///Field `Out2` writer - nUARTOut2 modem status
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG, Out2>;
impl<'a, REG> Out2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output zero
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Zero)
    }
    ///Output one
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::One)
    }
}
/**RTS hardware flow control enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsen {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Rtsen> for bool {
    #[inline(always)]
    fn from(variant: Rtsen) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSEn` reader - RTS hardware flow control enable
pub type RtsenR = crate::BitReader<Rtsen>;
impl RtsenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Rtsen {
        match self.bits {
            false => Rtsen::Disabled,
            true => Rtsen::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rtsen::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rtsen::Enabled
    }
}
///Field `RTSEn` writer - RTS hardware flow control enable
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG, Rtsen>;
impl<'a, REG> RtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Enabled)
    }
}
/**CTS hardware flow control enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSEn` reader - CTS hardware flow control enable
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::Disabled,
            true => Ctsen::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctsen::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctsen::Enabled
    }
}
///Field `CTSEn` writer - CTS hardware flow control enable
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Enabled)
    }
}
impl R {
    ///Bit 0 - UART enable
    #[inline(always)]
    pub fn uarten(&self) -> UartenR {
        UartenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SIR enable
    #[inline(always)]
    pub fn siren(&self) -> SirenR {
        SirenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SIR low-power IrDA
    #[inline(always)]
    pub fn sirlp(&self) -> SirlpR {
        SirlpR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - Loopback enable
    #[inline(always)]
    pub fn lbe(&self) -> LbeR {
        LbeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmit enable
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Receive enable
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data transmit ready
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Request to send
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - nUARTOut1 modem status
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - nUARTOut2 modem status
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RTS hardware flow control enable
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CTS hardware flow control enable
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - UART enable
    #[inline(always)]
    pub fn uarten(&mut self) -> UartenW<'_, CrSpec> {
        UartenW::new(self, 0)
    }
    ///Bit 1 - SIR enable
    #[inline(always)]
    pub fn siren(&mut self) -> SirenW<'_, CrSpec> {
        SirenW::new(self, 1)
    }
    ///Bit 2 - SIR low-power IrDA
    #[inline(always)]
    pub fn sirlp(&mut self) -> SirlpW<'_, CrSpec> {
        SirlpW::new(self, 2)
    }
    ///Bit 7 - Loopback enable
    #[inline(always)]
    pub fn lbe(&mut self) -> LbeW<'_, CrSpec> {
        LbeW::new(self, 7)
    }
    ///Bit 8 - Transmit enable
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, CrSpec> {
        TxeW::new(self, 8)
    }
    ///Bit 9 - Receive enable
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<'_, CrSpec> {
        RxeW::new(self, 9)
    }
    ///Bit 10 - Data transmit ready
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<'_, CrSpec> {
        DtrW::new(self, 10)
    }
    ///Bit 11 - Request to send
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<'_, CrSpec> {
        RtsW::new(self, 11)
    }
    ///Bit 12 - nUARTOut1 modem status
    #[inline(always)]
    pub fn out1(&mut self) -> Out1W<'_, CrSpec> {
        Out1W::new(self, 12)
    }
    ///Bit 13 - nUARTOut2 modem status
    #[inline(always)]
    pub fn out2(&mut self) -> Out2W<'_, CrSpec> {
        Out2W::new(self, 13)
    }
    ///Bit 14 - RTS hardware flow control enable
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<'_, CrSpec> {
        RtsenW::new(self, 14)
    }
    ///Bit 15 - CTS hardware flow control enable
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, CrSpec> {
        CtsenW::new(self, 15)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CrSpec {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CrSpec {}
