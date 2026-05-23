///Register `DMACC0Control` reader
pub type R = crate::R<Dmacc0controlSpec>;
///Register `DMACC0Control` writer
pub type W = crate::W<Dmacc0controlSpec>;
///Field `TransferSize` reader - Transfer size
pub type TransferSizeR = crate::FieldReader<u16>;
///Field `TransferSize` writer - Transfer size
pub type TransferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Source burst size
pub use Dbsize as Sbsize;
///Field `SBSize` reader - Source burst size
pub use DbsizeR as SbsizeR;
///Field `SBSize` writer - Source burst size
pub use DbsizeW as SbsizeW;
/**Destination burst size

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbsize {
    ///0: 1
    _1byte = 0,
    ///1: 4
    _4bytes = 1,
    ///2: 8
    _8bytes = 2,
    ///3: 16
    _16bytes = 3,
    ///4: 32
    _32bytes = 4,
    ///5: 64
    _64bytes = 5,
    ///6: 128
    _128bytes = 6,
    ///7: 256
    _256bytes = 7,
}
impl From<Dbsize> for u8 {
    #[inline(always)]
    fn from(variant: Dbsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbsize {
    type Ux = u8;
}
impl crate::IsEnum for Dbsize {}
///Field `DBSize` reader - Destination burst size
pub type DbsizeR = crate::FieldReader<Dbsize>;
impl DbsizeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dbsize {
        match self.bits {
            0 => Dbsize::_1byte,
            1 => Dbsize::_4bytes,
            2 => Dbsize::_8bytes,
            3 => Dbsize::_16bytes,
            4 => Dbsize::_32bytes,
            5 => Dbsize::_64bytes,
            6 => Dbsize::_128bytes,
            7 => Dbsize::_256bytes,
            _ => unreachable!(),
        }
    }
    ///1
    #[inline(always)]
    pub fn is_1byte(&self) -> bool {
        *self == Dbsize::_1byte
    }
    ///4
    #[inline(always)]
    pub fn is_4bytes(&self) -> bool {
        *self == Dbsize::_4bytes
    }
    ///8
    #[inline(always)]
    pub fn is_8bytes(&self) -> bool {
        *self == Dbsize::_8bytes
    }
    ///16
    #[inline(always)]
    pub fn is_16bytes(&self) -> bool {
        *self == Dbsize::_16bytes
    }
    ///32
    #[inline(always)]
    pub fn is_32bytes(&self) -> bool {
        *self == Dbsize::_32bytes
    }
    ///64
    #[inline(always)]
    pub fn is_64bytes(&self) -> bool {
        *self == Dbsize::_64bytes
    }
    ///128
    #[inline(always)]
    pub fn is_128bytes(&self) -> bool {
        *self == Dbsize::_128bytes
    }
    ///256
    #[inline(always)]
    pub fn is_256bytes(&self) -> bool {
        *self == Dbsize::_256bytes
    }
}
///Field `DBSize` writer - Destination burst size
pub type DbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dbsize, crate::Safe>;
impl<'a, REG> DbsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1
    #[inline(always)]
    pub fn _1byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dbsize::_1byte)
    }
    ///4
    #[inline(always)]
    pub fn _4bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Dbsize::_4bytes)
    }
    ///8
    #[inline(always)]
    pub fn _8bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Dbsize::_8bytes)
    }
    ///16
    #[inline(always)]
    pub fn _16bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Dbsize::_16bytes)
    }
    ///32
    #[inline(always)]
    pub fn _32bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Dbsize::_32bytes)
    }
    ///64
    #[inline(always)]
    pub fn _64bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Dbsize::_64bytes)
    }
    ///128
    #[inline(always)]
    pub fn _128bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Dbsize::_128bytes)
    }
    ///256
    #[inline(always)]
    pub fn _256bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Dbsize::_256bytes)
    }
}
///Source transfer width
pub use Dwidth as Swidth;
///Field `SWidth` reader - Source transfer width
pub use DwidthR as SwidthR;
///Field `SWidth` writer - Source transfer width
pub use DwidthW as SwidthW;
/**Destination transfer width

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dwidth {
    ///0: Byte, 8-bit
    Byte = 0,
    ///1: Halfword, 16-bit
    Halfword = 1,
    ///2: Word, 32-bit
    Word = 2,
}
impl From<Dwidth> for u8 {
    #[inline(always)]
    fn from(variant: Dwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dwidth {
    type Ux = u8;
}
impl crate::IsEnum for Dwidth {}
///Field `DWidth` reader - Destination transfer width
pub type DwidthR = crate::FieldReader<Dwidth>;
impl DwidthR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dwidth> {
        match self.bits {
            0 => Some(Dwidth::Byte),
            1 => Some(Dwidth::Halfword),
            2 => Some(Dwidth::Word),
            _ => None,
        }
    }
    ///Byte, 8-bit
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dwidth::Byte
    }
    ///Halfword, 16-bit
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == Dwidth::Halfword
    }
    ///Word, 32-bit
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dwidth::Word
    }
}
///Field `DWidth` writer - Destination transfer width
pub type DwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dwidth>;
impl<'a, REG> DwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Byte, 8-bit
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidth::Byte)
    }
    ///Halfword, 16-bit
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidth::Halfword)
    }
    ///Word, 32-bit
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dwidth::Word)
    }
}
/**Source AHB master select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S {
    ///0: AHB Master 1
    Ahb1 = 0,
    ///1: AHB Master 2
    Ahb2 = 1,
}
impl From<S> for bool {
    #[inline(always)]
    fn from(variant: S) -> Self {
        variant as u8 != 0
    }
}
///Field `S` reader - Source AHB master select
pub type SR = crate::BitReader<S>;
impl SR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> S {
        match self.bits {
            false => S::Ahb1,
            true => S::Ahb2,
        }
    }
    ///AHB Master 1
    #[inline(always)]
    pub fn is_ahb1(&self) -> bool {
        *self == S::Ahb1
    }
    ///AHB Master 2
    #[inline(always)]
    pub fn is_ahb2(&self) -> bool {
        *self == S::Ahb2
    }
}
///Field `S` writer - Source AHB master select
pub type SW<'a, REG> = crate::BitWriter<'a, REG, S>;
impl<'a, REG> SW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB Master 1
    #[inline(always)]
    pub fn ahb1(self) -> &'a mut crate::W<REG> {
        self.variant(S::Ahb1)
    }
    ///AHB Master 2
    #[inline(always)]
    pub fn ahb2(self) -> &'a mut crate::W<REG> {
        self.variant(S::Ahb2)
    }
}
/**Destination AHB master select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D {
    ///0: AHB Master 1
    Ahb1 = 0,
    ///1: AHB Master 2
    Ahb2 = 1,
}
impl From<D> for bool {
    #[inline(always)]
    fn from(variant: D) -> Self {
        variant as u8 != 0
    }
}
///Field `D` reader - Destination AHB master select
pub type DR = crate::BitReader<D>;
impl DR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> D {
        match self.bits {
            false => D::Ahb1,
            true => D::Ahb2,
        }
    }
    ///AHB Master 1
    #[inline(always)]
    pub fn is_ahb1(&self) -> bool {
        *self == D::Ahb1
    }
    ///AHB Master 2
    #[inline(always)]
    pub fn is_ahb2(&self) -> bool {
        *self == D::Ahb2
    }
}
///Field `D` writer - Destination AHB master select
pub type DW<'a, REG> = crate::BitWriter<'a, REG, D>;
impl<'a, REG> DW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB Master 1
    #[inline(always)]
    pub fn ahb1(self) -> &'a mut crate::W<REG> {
        self.variant(D::Ahb1)
    }
    ///AHB Master 2
    #[inline(always)]
    pub fn ahb2(self) -> &'a mut crate::W<REG> {
        self.variant(D::Ahb2)
    }
}
/**Source increment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Si {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Si> for bool {
    #[inline(always)]
    fn from(variant: Si) -> Self {
        variant as u8 != 0
    }
}
///Field `SI` reader - Source increment
pub type SiR = crate::BitReader<Si>;
impl SiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Si {
        match self.bits {
            false => Si::Disabled,
            true => Si::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Si::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Si::Enabled
    }
}
///Field `SI` writer - Source increment
pub type SiW<'a, REG> = crate::BitWriter<'a, REG, Si>;
impl<'a, REG> SiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Si::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Si::Enabled)
    }
}
/**Destination increment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Di {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Di> for bool {
    #[inline(always)]
    fn from(variant: Di) -> Self {
        variant as u8 != 0
    }
}
///Field `DI` reader - Destination increment
pub type DiR = crate::BitReader<Di>;
impl DiR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Di {
        match self.bits {
            false => Di::Disabled,
            true => Di::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Di::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Di::Enabled
    }
}
///Field `DI` writer - Destination increment
pub type DiW<'a, REG> = crate::BitWriter<'a, REG, Di>;
impl<'a, REG> DiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Di::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Di::Enabled)
    }
}
///Field `Prot` reader - Protection
pub type ProtR = crate::FieldReader;
///Field `Prot` writer - Protection
pub type ProtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Terminal count interrupt enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<I> for bool {
    #[inline(always)]
    fn from(variant: I) -> Self {
        variant as u8 != 0
    }
}
///Field `I` reader - Terminal count interrupt enable
pub type IR = crate::BitReader<I>;
impl IR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I {
        match self.bits {
            false => I::Disabled,
            true => I::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I::Enabled
    }
}
///Field `I` writer - Terminal count interrupt enable
pub type IW<'a, REG> = crate::BitWriter<'a, REG, I>;
impl<'a, REG> IW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I::Enabled)
    }
}
impl R {
    ///Bits 0:11 - Transfer size
    #[inline(always)]
    pub fn transfer_size(&self) -> TransferSizeR {
        TransferSizeR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:14 - Source burst size
    #[inline(always)]
    pub fn sbsize(&self) -> SbsizeR {
        SbsizeR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Destination burst size
    #[inline(always)]
    pub fn dbsize(&self) -> DbsizeR {
        DbsizeR::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Source transfer width
    #[inline(always)]
    pub fn swidth(&self) -> SwidthR {
        SwidthR::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Destination transfer width
    #[inline(always)]
    pub fn dwidth(&self) -> DwidthR {
        DwidthR::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 24 - Source AHB master select
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Destination AHB master select
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Source increment
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Destination increment
    #[inline(always)]
    pub fn di(&self) -> DiR {
        DiR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - Protection
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - Terminal count interrupt enable
    #[inline(always)]
    pub fn i(&self) -> IR {
        IR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - Transfer size
    #[inline(always)]
    pub fn transfer_size(&mut self) -> TransferSizeW<'_, Dmacc0controlSpec> {
        TransferSizeW::new(self, 0)
    }
    ///Bits 12:14 - Source burst size
    #[inline(always)]
    pub fn sbsize(&mut self) -> SbsizeW<'_, Dmacc0controlSpec> {
        SbsizeW::new(self, 12)
    }
    ///Bits 15:17 - Destination burst size
    #[inline(always)]
    pub fn dbsize(&mut self) -> DbsizeW<'_, Dmacc0controlSpec> {
        DbsizeW::new(self, 15)
    }
    ///Bits 18:20 - Source transfer width
    #[inline(always)]
    pub fn swidth(&mut self) -> SwidthW<'_, Dmacc0controlSpec> {
        SwidthW::new(self, 18)
    }
    ///Bits 21:23 - Destination transfer width
    #[inline(always)]
    pub fn dwidth(&mut self) -> DwidthW<'_, Dmacc0controlSpec> {
        DwidthW::new(self, 21)
    }
    ///Bit 24 - Source AHB master select
    #[inline(always)]
    pub fn s(&mut self) -> SW<'_, Dmacc0controlSpec> {
        SW::new(self, 24)
    }
    ///Bit 25 - Destination AHB master select
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, Dmacc0controlSpec> {
        DW::new(self, 25)
    }
    ///Bit 26 - Source increment
    #[inline(always)]
    pub fn si(&mut self) -> SiW<'_, Dmacc0controlSpec> {
        SiW::new(self, 26)
    }
    ///Bit 27 - Destination increment
    #[inline(always)]
    pub fn di(&mut self) -> DiW<'_, Dmacc0controlSpec> {
        DiW::new(self, 27)
    }
    ///Bits 28:30 - Protection
    #[inline(always)]
    pub fn prot(&mut self) -> ProtW<'_, Dmacc0controlSpec> {
        ProtW::new(self, 28)
    }
    ///Bit 31 - Terminal count interrupt enable
    #[inline(always)]
    pub fn i(&mut self) -> IW<'_, Dmacc0controlSpec> {
        IW::new(self, 31)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacc0control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacc0control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dmacc0controlSpec;
impl crate::RegisterSpec for Dmacc0controlSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacc0control::R`](R) reader structure
impl crate::Readable for Dmacc0controlSpec {}
///`write(|w| ..)` method takes [`dmacc0control::W`](W) writer structure
impl crate::Writable for Dmacc0controlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACC0Control to value 0
impl crate::Resettable for Dmacc0controlSpec {}
