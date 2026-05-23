///Register `LCR_H` reader
pub type R = crate::R<LcrHSpec>;
///Register `LCR_H` writer
pub type W = crate::W<LcrHSpec>;
/**Send break

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brk {
    ///0: Normal operation
    NormalOps = 0,
    ///1: Send break
    SendBreak = 1,
}
impl From<Brk> for bool {
    #[inline(always)]
    fn from(variant: Brk) -> Self {
        variant as u8 != 0
    }
}
///Field `BRK` reader - Send break
pub type BrkR = crate::BitReader<Brk>;
impl BrkR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Brk {
        match self.bits {
            false => Brk::NormalOps,
            true => Brk::SendBreak,
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_normal_ops(&self) -> bool {
        *self == Brk::NormalOps
    }
    ///Send break
    #[inline(always)]
    pub fn is_send_break(&self) -> bool {
        *self == Brk::SendBreak
    }
}
///Field `BRK` writer - Send break
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG, Brk>;
impl<'a, REG> BrkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut crate::W<REG> {
        self.variant(Brk::NormalOps)
    }
    ///Send break
    #[inline(always)]
    pub fn send_break(self) -> &'a mut crate::W<REG> {
        self.variant(Brk::SendBreak)
    }
}
/**Parity enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pen {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Pen> for bool {
    #[inline(always)]
    fn from(variant: Pen) -> Self {
        variant as u8 != 0
    }
}
///Field `PEN` reader - Parity enable
pub type PenR = crate::BitReader<Pen>;
impl PenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Pen {
        match self.bits {
            false => Pen::Disabled,
            true => Pen::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pen::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pen::Enabled
    }
}
///Field `PEN` writer - Parity enable
pub type PenW<'a, REG> = crate::BitWriter<'a, REG, Pen>;
impl<'a, REG> PenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Enabled)
    }
}
/**Even parity select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eps {
    ///0: Odd parity
    OddParity = 0,
    ///1: Even parity
    EvenParity = 1,
}
impl From<Eps> for bool {
    #[inline(always)]
    fn from(variant: Eps) -> Self {
        variant as u8 != 0
    }
}
///Field `EPS` reader - Even parity select
pub type EpsR = crate::BitReader<Eps>;
impl EpsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Eps {
        match self.bits {
            false => Eps::OddParity,
            true => Eps::EvenParity,
        }
    }
    ///Odd parity
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        *self == Eps::OddParity
    }
    ///Even parity
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        *self == Eps::EvenParity
    }
}
///Field `EPS` writer - Even parity select
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG, Eps>;
impl<'a, REG> EpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Odd parity
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::OddParity)
    }
    ///Even parity
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::EvenParity)
    }
}
/**Two stop bits select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stp2 {
    ///0: Not selected
    NotSelected = 0,
    ///1: Selected
    Selected = 1,
}
impl From<Stp2> for bool {
    #[inline(always)]
    fn from(variant: Stp2) -> Self {
        variant as u8 != 0
    }
}
///Field `STP2` reader - Two stop bits select
pub type Stp2R = crate::BitReader<Stp2>;
impl Stp2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Stp2 {
        match self.bits {
            false => Stp2::NotSelected,
            true => Stp2::Selected,
        }
    }
    ///Not selected
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Stp2::NotSelected
    }
    ///Selected
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Stp2::Selected
    }
}
///Field `STP2` writer - Two stop bits select
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG, Stp2>;
impl<'a, REG> Stp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not selected
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Stp2::NotSelected)
    }
    ///Selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Stp2::Selected)
    }
}
/**Enable FIFOs

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fen {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Fen> for bool {
    #[inline(always)]
    fn from(variant: Fen) -> Self {
        variant as u8 != 0
    }
}
///Field `FEN` reader - Enable FIFOs
pub type FenR = crate::BitReader<Fen>;
impl FenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fen {
        match self.bits {
            false => Fen::Disabled,
            true => Fen::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fen::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fen::Enabled
    }
}
///Field `FEN` writer - Enable FIFOs
pub type FenW<'a, REG> = crate::BitWriter<'a, REG, Fen>;
impl<'a, REG> FenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fen::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fen::Enabled)
    }
}
/**Word Length

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wlen {
    ///3: 8 bits
    _8bits = 3,
    ///2: 7 bits
    _7bits = 2,
    ///1: 6 bits
    _6bits = 1,
    ///0: 5 bits
    _5bits = 0,
}
impl From<Wlen> for u8 {
    #[inline(always)]
    fn from(variant: Wlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wlen {
    type Ux = u8;
}
impl crate::IsEnum for Wlen {}
///Field `WLEN` reader - Word Length
pub type WlenR = crate::FieldReader<Wlen>;
impl WlenR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Wlen {
        match self.bits {
            3 => Wlen::_8bits,
            2 => Wlen::_7bits,
            1 => Wlen::_6bits,
            0 => Wlen::_5bits,
            _ => unreachable!(),
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == Wlen::_8bits
    }
    ///7 bits
    #[inline(always)]
    pub fn is_7bits(&self) -> bool {
        *self == Wlen::_7bits
    }
    ///6 bits
    #[inline(always)]
    pub fn is_6bits(&self) -> bool {
        *self == Wlen::_6bits
    }
    ///5 bits
    #[inline(always)]
    pub fn is_5bits(&self) -> bool {
        *self == Wlen::_5bits
    }
}
///Field `WLEN` writer - Word Length
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wlen, crate::Safe>;
impl<'a, REG> WlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_8bits)
    }
    ///7 bits
    #[inline(always)]
    pub fn _7bits(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_7bits)
    }
    ///6 bits
    #[inline(always)]
    pub fn _6bits(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_6bits)
    }
    ///5 bits
    #[inline(always)]
    pub fn _5bits(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_5bits)
    }
}
/**Stick parity select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sps {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Sps> for bool {
    #[inline(always)]
    fn from(variant: Sps) -> Self {
        variant as u8 != 0
    }
}
///Field `SPS` reader - Stick parity select
pub type SpsR = crate::BitReader<Sps>;
impl SpsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Sps {
        match self.bits {
            false => Sps::Disabled,
            true => Sps::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sps::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sps::Enabled
    }
}
///Field `SPS` writer - Stick parity select
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG, Sps>;
impl<'a, REG> SpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sps::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sps::Enabled)
    }
}
impl R {
    ///Bit 0 - Send break
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Parity enable
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Even parity select
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Two stop bits select
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable FIFOs
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Word Length
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Stick parity select
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Send break
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<'_, LcrHSpec> {
        BrkW::new(self, 0)
    }
    ///Bit 1 - Parity enable
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, LcrHSpec> {
        PenW::new(self, 1)
    }
    ///Bit 2 - Even parity select
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<'_, LcrHSpec> {
        EpsW::new(self, 2)
    }
    ///Bit 3 - Two stop bits select
    #[inline(always)]
    pub fn stp2(&mut self) -> Stp2W<'_, LcrHSpec> {
        Stp2W::new(self, 3)
    }
    ///Bit 4 - Enable FIFOs
    #[inline(always)]
    pub fn fen(&mut self) -> FenW<'_, LcrHSpec> {
        FenW::new(self, 4)
    }
    ///Bits 5:6 - Word Length
    #[inline(always)]
    pub fn wlen(&mut self) -> WlenW<'_, LcrHSpec> {
        WlenW::new(self, 5)
    }
    ///Bit 7 - Stick parity select
    #[inline(always)]
    pub fn sps(&mut self) -> SpsW<'_, LcrHSpec> {
        SpsW::new(self, 7)
    }
}
/**Line Control Register

You can [`read`](crate::Reg::read) this register and get [`lcr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LcrHSpec;
impl crate::RegisterSpec for LcrHSpec {
    type Ux = u32;
}
///`read()` method returns [`lcr_h::R`](R) reader structure
impl crate::Readable for LcrHSpec {}
///`write(|w| ..)` method takes [`lcr_h::W`](W) writer structure
impl crate::Writable for LcrHSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCR_H to value 0
impl crate::Resettable for LcrHSpec {}
