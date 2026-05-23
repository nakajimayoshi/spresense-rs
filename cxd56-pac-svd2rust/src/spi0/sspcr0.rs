///Register `SSPCR0` reader
pub type R = crate::R<Sspcr0Spec>;
///Register `SSPCR0` writer
pub type W = crate::W<Sspcr0Spec>;
/**Data Size Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    ///3: 4bit data
    _4bit = 3,
    ///4: 5bit data
    _5bit = 4,
    ///5: 6bit data
    _6bit = 5,
    ///6: 7bit data
    _7bit = 6,
    ///7: 8bit data
    _8bit = 7,
    ///8: 9bit data
    _9bit = 8,
    ///9: 10bit data
    _10bit = 9,
    ///10: 11bit data
    _11bit = 10,
    ///11: 12bit data
    _12bit = 11,
    ///12: 13bit data
    _13bit = 12,
    ///13: 14bit data
    _14bit = 13,
    ///14: 15bit data
    _15bit = 14,
    ///15: 16bit data
    _16bit = 15,
}
impl From<Dss> for u8 {
    #[inline(always)]
    fn from(variant: Dss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dss {
    type Ux = u8;
}
impl crate::IsEnum for Dss {}
///Field `DSS` reader - Data Size Select
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            3 => Some(Dss::_4bit),
            4 => Some(Dss::_5bit),
            5 => Some(Dss::_6bit),
            6 => Some(Dss::_7bit),
            7 => Some(Dss::_8bit),
            8 => Some(Dss::_9bit),
            9 => Some(Dss::_10bit),
            10 => Some(Dss::_11bit),
            11 => Some(Dss::_12bit),
            12 => Some(Dss::_13bit),
            13 => Some(Dss::_14bit),
            14 => Some(Dss::_15bit),
            15 => Some(Dss::_16bit),
            _ => None,
        }
    }
    ///4bit data
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == Dss::_4bit
    }
    ///5bit data
    #[inline(always)]
    pub fn is_5bit(&self) -> bool {
        *self == Dss::_5bit
    }
    ///6bit data
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Dss::_6bit
    }
    ///7bit data
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Dss::_7bit
    }
    ///8bit data
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Dss::_8bit
    }
    ///9bit data
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == Dss::_9bit
    }
    ///10bit data
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Dss::_10bit
    }
    ///11bit data
    #[inline(always)]
    pub fn is_11bit(&self) -> bool {
        *self == Dss::_11bit
    }
    ///12bit data
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Dss::_12bit
    }
    ///13bit data
    #[inline(always)]
    pub fn is_13bit(&self) -> bool {
        *self == Dss::_13bit
    }
    ///14bit data
    #[inline(always)]
    pub fn is_14bit(&self) -> bool {
        *self == Dss::_14bit
    }
    ///15bit data
    #[inline(always)]
    pub fn is_15bit(&self) -> bool {
        *self == Dss::_15bit
    }
    ///16bit data
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Dss::_16bit
    }
}
///Field `DSS` writer - Data Size Select
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///4bit data
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_4bit)
    }
    ///5bit data
    #[inline(always)]
    pub fn _5bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_5bit)
    }
    ///6bit data
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_6bit)
    }
    ///7bit data
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_7bit)
    }
    ///8bit data
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_8bit)
    }
    ///9bit data
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_9bit)
    }
    ///10bit data
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_10bit)
    }
    ///11bit data
    #[inline(always)]
    pub fn _11bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_11bit)
    }
    ///12bit data
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_12bit)
    }
    ///13bit data
    #[inline(always)]
    pub fn _13bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_13bit)
    }
    ///14bit data
    #[inline(always)]
    pub fn _14bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_14bit)
    }
    ///15bit data
    #[inline(always)]
    pub fn _15bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_15bit)
    }
    ///16bit data
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_16bit)
    }
}
/**Frame format

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    ///0: Motorola SPI frame format
    Motorola = 0,
    ///1: TI synchronous serial frame format
    Ti = 1,
    ///2: National Microwire frame format
    Nm = 2,
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frf {
    type Ux = u8;
}
impl crate::IsEnum for Frf {}
///Field `FRF` reader - Frame format
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frf> {
        match self.bits {
            0 => Some(Frf::Motorola),
            1 => Some(Frf::Ti),
            2 => Some(Frf::Nm),
            _ => None,
        }
    }
    ///Motorola SPI frame format
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == Frf::Motorola
    }
    ///TI synchronous serial frame format
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == Frf::Ti
    }
    ///National Microwire frame format
    #[inline(always)]
    pub fn is_nm(&self) -> bool {
        *self == Frf::Nm
    }
}
///Field `FRF` writer - Frame format
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Motorola SPI frame format
    #[inline(always)]
    pub fn motorola(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Motorola)
    }
    ///TI synchronous serial frame format
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Ti)
    }
    ///National Microwire frame format
    #[inline(always)]
    pub fn nm(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Nm)
    }
}
///Field `SPO` reader - SSPCLKOUT polarity
pub type SpoR = crate::BitReader;
///Field `SPO` writer - SSPCLKOUT polarity
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPH` reader - SSPCLKOUT phase
pub type SphR = crate::BitReader;
///Field `SPH` writer - SSPCLKOUT phase
pub type SphW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCR` reader - Serial clock rate
pub type ScrR = crate::FieldReader;
///Field `SCR` writer - Serial clock rate
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3 - Data Size Select
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Frame format
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - SSPCLKOUT polarity
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SSPCLKOUT phase
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Serial clock rate
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:3 - Data Size Select
    #[inline(always)]
    pub fn dss(&mut self) -> DssW<'_, Sspcr0Spec> {
        DssW::new(self, 0)
    }
    ///Bits 4:5 - Frame format
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<'_, Sspcr0Spec> {
        FrfW::new(self, 4)
    }
    ///Bit 6 - SSPCLKOUT polarity
    #[inline(always)]
    pub fn spo(&mut self) -> SpoW<'_, Sspcr0Spec> {
        SpoW::new(self, 6)
    }
    ///Bit 7 - SSPCLKOUT phase
    #[inline(always)]
    pub fn sph(&mut self) -> SphW<'_, Sspcr0Spec> {
        SphW::new(self, 7)
    }
    ///Bits 8:15 - Serial clock rate
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<'_, Sspcr0Spec> {
        ScrW::new(self, 8)
    }
}
/**Control register 0

You can [`read`](crate::Reg::read) this register and get [`sspcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Sspcr0Spec;
impl crate::RegisterSpec for Sspcr0Spec {
    type Ux = u32;
}
///`read()` method returns [`sspcr0::R`](R) reader structure
impl crate::Readable for Sspcr0Spec {}
///`write(|w| ..)` method takes [`sspcr0::W`](W) writer structure
impl crate::Writable for Sspcr0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSPCR0 to value 0
impl crate::Resettable for Sspcr0Spec {}
