#[doc = "Register `LCR_H` reader"]
pub type R = crate::R<LcrHSpec>;
#[doc = "Register `LCR_H` writer"]
pub type W = crate::W<LcrHSpec>;
#[doc = "Send break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brk {
    #[doc = "0: Normal operation"]
    NormalOps = 0,
    #[doc = "1: Send break"]
    SendBreak = 1,
}
impl From<Brk> for bool {
    #[inline(always)]
    fn from(variant: Brk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK` reader - Send break"]
pub type BrkR = crate::BitReader<Brk>;
impl BrkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brk {
        match self.bits {
            false => Brk::NormalOps,
            true => Brk::SendBreak,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal_ops(&self) -> bool {
        *self == Brk::NormalOps
    }
    #[doc = "Send break"]
    #[inline(always)]
    pub fn is_send_break(&self) -> bool {
        *self == Brk::SendBreak
    }
}
#[doc = "Field `BRK` writer - Send break"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG, Brk>;
impl<'a, REG> BrkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut crate::W<REG> {
        self.variant(Brk::NormalOps)
    }
    #[doc = "Send break"]
    #[inline(always)]
    pub fn send_break(self) -> &'a mut crate::W<REG> {
        self.variant(Brk::SendBreak)
    }
}
#[doc = "Parity enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pen {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Pen> for bool {
    #[inline(always)]
    fn from(variant: Pen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - Parity enable"]
pub type PenR = crate::BitReader<Pen>;
impl PenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pen {
        match self.bits {
            false => Pen::Disabled,
            true => Pen::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pen::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pen::Enabled
    }
}
#[doc = "Field `PEN` writer - Parity enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG, Pen>;
impl<'a, REG> PenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Enabled)
    }
}
#[doc = "Even parity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eps {
    #[doc = "0: Odd parity"]
    OddParity = 0,
    #[doc = "1: Even parity"]
    EvenParity = 1,
}
impl From<Eps> for bool {
    #[inline(always)]
    fn from(variant: Eps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPS` reader - Even parity select"]
pub type EpsR = crate::BitReader<Eps>;
impl EpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eps {
        match self.bits {
            false => Eps::OddParity,
            true => Eps::EvenParity,
        }
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        *self == Eps::OddParity
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        *self == Eps::EvenParity
    }
}
#[doc = "Field `EPS` writer - Even parity select"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG, Eps>;
impl<'a, REG> EpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::OddParity)
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::EvenParity)
    }
}
#[doc = "Two stop bits select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stp2 {
    #[doc = "0: Not selected"]
    NotSelected = 0,
    #[doc = "1: Selected"]
    Selected = 1,
}
impl From<Stp2> for bool {
    #[inline(always)]
    fn from(variant: Stp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STP2` reader - Two stop bits select"]
pub type Stp2R = crate::BitReader<Stp2>;
impl Stp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stp2 {
        match self.bits {
            false => Stp2::NotSelected,
            true => Stp2::Selected,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Stp2::NotSelected
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Stp2::Selected
    }
}
#[doc = "Field `STP2` writer - Two stop bits select"]
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG, Stp2>;
impl<'a, REG> Stp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Stp2::NotSelected)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Stp2::Selected)
    }
}
#[doc = "Enable FIFOs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fen {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Fen> for bool {
    #[inline(always)]
    fn from(variant: Fen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEN` reader - Enable FIFOs"]
pub type FenR = crate::BitReader<Fen>;
impl FenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fen {
        match self.bits {
            false => Fen::Disabled,
            true => Fen::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fen::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fen::Enabled
    }
}
#[doc = "Field `FEN` writer - Enable FIFOs"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG, Fen>;
impl<'a, REG> FenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fen::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fen::Enabled)
    }
}
#[doc = "Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wlen {
    #[doc = "3: 8 bits"]
    _8bits = 3,
    #[doc = "2: 7 bits"]
    _7bits = 2,
    #[doc = "1: 6 bits"]
    _6bits = 1,
    #[doc = "0: 5 bits"]
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
#[doc = "Field `WLEN` reader - Word Length"]
pub type WlenR = crate::FieldReader<Wlen>;
impl WlenR {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == Wlen::_8bits
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_7bits(&self) -> bool {
        *self == Wlen::_7bits
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_6bits(&self) -> bool {
        *self == Wlen::_6bits
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn is_5bits(&self) -> bool {
        *self == Wlen::_5bits
    }
}
#[doc = "Field `WLEN` writer - Word Length"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wlen, crate::Safe>;
impl<'a, REG> WlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_8bits)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _7bits(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_7bits)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _6bits(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_6bits)
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn _5bits(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::_5bits)
    }
}
#[doc = "Stick parity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sps {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Sps> for bool {
    #[inline(always)]
    fn from(variant: Sps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPS` reader - Stick parity select"]
pub type SpsR = crate::BitReader<Sps>;
impl SpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sps {
        match self.bits {
            false => Sps::Disabled,
            true => Sps::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sps::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sps::Enabled
    }
}
#[doc = "Field `SPS` writer - Stick parity select"]
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG, Sps>;
impl<'a, REG> SpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sps::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sps::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Even parity select"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Two stop bits select"]
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Word Length"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Stick parity select"]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<'_, LcrHSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, LcrHSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - Even parity select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<'_, LcrHSpec> {
        EpsW::new(self, 2)
    }
    #[doc = "Bit 3 - Two stop bits select"]
    #[inline(always)]
    pub fn stp2(&mut self) -> Stp2W<'_, LcrHSpec> {
        Stp2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&mut self) -> FenW<'_, LcrHSpec> {
        FenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Word Length"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WlenW<'_, LcrHSpec> {
        WlenW::new(self, 5)
    }
    #[doc = "Bit 7 - Stick parity select"]
    #[inline(always)]
    pub fn sps(&mut self) -> SpsW<'_, LcrHSpec> {
        SpsW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrHSpec;
impl crate::RegisterSpec for LcrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr_h::R`](R) reader structure"]
impl crate::Readable for LcrHSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr_h::W`](W) writer structure"]
impl crate::Writable for LcrHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCR_H to value 0"]
impl crate::Resettable for LcrHSpec {}
