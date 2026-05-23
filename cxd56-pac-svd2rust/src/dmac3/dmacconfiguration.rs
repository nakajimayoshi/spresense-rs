///Register `DMACConfiguration` reader
pub type R = crate::R<DmacconfigurationSpec>;
///Register `DMACConfiguration` writer
pub type W = crate::W<DmacconfigurationSpec>;
/**DMAC enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<E> for bool {
    #[inline(always)]
    fn from(variant: E) -> Self {
        variant as u8 != 0
    }
}
///Field `E` reader - DMAC enable
pub type ER = crate::BitReader<E>;
impl ER {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> E {
        match self.bits {
            false => E::Disabled,
            true => E::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == E::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == E::Enabled
    }
}
///Field `E` writer - DMAC enable
pub type EW<'a, REG> = crate::BitWriter<'a, REG, E>;
impl<'a, REG> EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(E::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(E::Enabled)
    }
}
/**AHB Master 1 endianess configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M1 {
    ///0: Little-endian mode
    Le = 0,
    ///1: Big-endian mode
    Be = 1,
}
impl From<M1> for bool {
    #[inline(always)]
    fn from(variant: M1) -> Self {
        variant as u8 != 0
    }
}
///Field `M1` reader - AHB Master 1 endianess configuration
pub type M1R = crate::BitReader<M1>;
impl M1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> M1 {
        match self.bits {
            false => M1::Le,
            true => M1::Be,
        }
    }
    ///Little-endian mode
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == M1::Le
    }
    ///Big-endian mode
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == M1::Be
    }
}
///Field `M1` writer - AHB Master 1 endianess configuration
pub type M1W<'a, REG> = crate::BitWriter<'a, REG, M1>;
impl<'a, REG> M1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Little-endian mode
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(M1::Le)
    }
    ///Big-endian mode
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(M1::Be)
    }
}
/**AHB Master 2 endianess configuration

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M2 {
    ///0: Little-endian mode
    Le = 0,
    ///1: Big-endian mode
    Be = 1,
}
impl From<M2> for bool {
    #[inline(always)]
    fn from(variant: M2) -> Self {
        variant as u8 != 0
    }
}
///Field `M2` reader - AHB Master 2 endianess configuration
pub type M2R = crate::BitReader<M2>;
impl M2R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> M2 {
        match self.bits {
            false => M2::Le,
            true => M2::Be,
        }
    }
    ///Little-endian mode
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == M2::Le
    }
    ///Big-endian mode
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == M2::Be
    }
}
///Field `M2` writer - AHB Master 2 endianess configuration
pub type M2W<'a, REG> = crate::BitWriter<'a, REG, M2>;
impl<'a, REG> M2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Little-endian mode
    #[inline(always)]
    pub fn le(self) -> &'a mut crate::W<REG> {
        self.variant(M2::Le)
    }
    ///Big-endian mode
    #[inline(always)]
    pub fn be(self) -> &'a mut crate::W<REG> {
        self.variant(M2::Be)
    }
}
/**Transfer Size Extended

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ts {
    ///0: Not extended
    Noextended = 0,
    ///1: Extended
    Extended = 1,
}
impl From<Ts> for bool {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as u8 != 0
    }
}
///Field `TS` reader - Transfer Size Extended
pub type TsR = crate::BitReader<Ts>;
impl TsR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Ts {
        match self.bits {
            false => Ts::Noextended,
            true => Ts::Extended,
        }
    }
    ///Not extended
    #[inline(always)]
    pub fn is_noextended(&self) -> bool {
        *self == Ts::Noextended
    }
    ///Extended
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Ts::Extended
    }
}
/**Default LLI enabled

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlli {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Dlli> for bool {
    #[inline(always)]
    fn from(variant: Dlli) -> Self {
        variant as u8 != 0
    }
}
///Field `DLLI` reader - Default LLI enabled
pub type DlliR = crate::BitReader<Dlli>;
impl DlliR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Dlli {
        match self.bits {
            false => Dlli::Disabled,
            true => Dlli::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dlli::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dlli::Enabled
    }
}
/**Trigger function enabled

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tr {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<Tr> for bool {
    #[inline(always)]
    fn from(variant: Tr) -> Self {
        variant as u8 != 0
    }
}
///Field `TR` reader - Trigger function enabled
pub type TrR = crate::BitReader<Tr>;
impl TrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Tr {
        match self.bits {
            false => Tr::Disabled,
            true => Tr::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tr::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tr::Enabled
    }
}
/**DMAC Arbitration logic

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arb {
    ///0: Static
    Static = 0,
    ///1: Round Robin
    Roundrobin = 1,
}
impl From<Arb> for bool {
    #[inline(always)]
    fn from(variant: Arb) -> Self {
        variant as u8 != 0
    }
}
///Field `ARB` reader - DMAC Arbitration logic
pub type ArbR = crate::BitReader<Arb>;
impl ArbR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Arb {
        match self.bits {
            false => Arb::Static,
            true => Arb::Roundrobin,
        }
    }
    ///Static
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == Arb::Static
    }
    ///Round Robin
    #[inline(always)]
    pub fn is_roundrobin(&self) -> bool {
        *self == Arb::Roundrobin
    }
}
/**Transfer Size Extended

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsize {
    ///0: 16 bytes
    _16 = 0,
    ///1: 32 bytes
    _32 = 1,
    ///2: 64 bytes
    _64 = 2,
    ///3: 128 bytes
    _128 = 3,
}
impl From<Fsize> for u8 {
    #[inline(always)]
    fn from(variant: Fsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsize {
    type Ux = u8;
}
impl crate::IsEnum for Fsize {}
///Field `FSIZE` reader - Transfer Size Extended
pub type FsizeR = crate::FieldReader<Fsize>;
impl FsizeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Fsize {
        match self.bits {
            0 => Fsize::_16,
            1 => Fsize::_32,
            2 => Fsize::_64,
            3 => Fsize::_128,
            _ => unreachable!(),
        }
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Fsize::_16
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Fsize::_32
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Fsize::_64
    }
    ///128 bytes
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Fsize::_128
    }
}
impl R {
    ///Bit 0 - DMAC enable
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AHB Master 1 endianess configuration
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHB Master 2 endianess configuration
    #[inline(always)]
    pub fn m2(&self) -> M2R {
        M2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - Transfer Size Extended
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Default LLI enabled
    #[inline(always)]
    pub fn dlli(&self) -> DlliR {
        DlliR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Trigger function enabled
    #[inline(always)]
    pub fn tr(&self) -> TrR {
        TrR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMAC Arbitration logic
    #[inline(always)]
    pub fn arb(&self) -> ArbR {
        ArbR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Transfer Size Extended
    #[inline(always)]
    pub fn fsize(&self) -> FsizeR {
        FsizeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - DMAC enable
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, DmacconfigurationSpec> {
        EW::new(self, 0)
    }
    ///Bit 1 - AHB Master 1 endianess configuration
    #[inline(always)]
    pub fn m1(&mut self) -> M1W<'_, DmacconfigurationSpec> {
        M1W::new(self, 1)
    }
    ///Bit 2 - AHB Master 2 endianess configuration
    #[inline(always)]
    pub fn m2(&mut self) -> M2W<'_, DmacconfigurationSpec> {
        M2W::new(self, 2)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dmacconfiguration::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacconfiguration::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DmacconfigurationSpec;
impl crate::RegisterSpec for DmacconfigurationSpec {
    type Ux = u32;
}
///`read()` method returns [`dmacconfiguration::R`](R) reader structure
impl crate::Readable for DmacconfigurationSpec {}
///`write(|w| ..)` method takes [`dmacconfiguration::W`](W) writer structure
impl crate::Writable for DmacconfigurationSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACConfiguration to value 0
impl crate::Resettable for DmacconfigurationSpec {}
