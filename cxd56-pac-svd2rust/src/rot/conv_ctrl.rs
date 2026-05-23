///Register `CONV_CTRL` reader
pub type R = crate::R<ConvCtrlSpec>;
///Register `CONV_CTRL` writer
pub type W = crate::W<ConvCtrlSpec>;
/**Convert RGB565 to YCbCr422

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ConvFormat {
    ///0: `0`
    Noconvert = 0,
    ///1: `1`
    Ycbcr422Rgb565 = 1,
    ///2: `10`
    Rgb565Ycbcr422 = 2,
}
impl From<ConvFormat> for u8 {
    #[inline(always)]
    fn from(variant: ConvFormat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ConvFormat {
    type Ux = u8;
}
impl crate::IsEnum for ConvFormat {}
///Field `CONV_FORMAT` reader - Convert RGB565 to YCbCr422
pub type ConvFormatR = crate::FieldReader<ConvFormat>;
impl ConvFormatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ConvFormat> {
        match self.bits {
            0 => Some(ConvFormat::Noconvert),
            1 => Some(ConvFormat::Ycbcr422Rgb565),
            2 => Some(ConvFormat::Rgb565Ycbcr422),
            _ => None,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_noconvert(&self) -> bool {
        *self == ConvFormat::Noconvert
    }
    ///`1`
    #[inline(always)]
    pub fn is_ycbcr422_rgb565(&self) -> bool {
        *self == ConvFormat::Ycbcr422Rgb565
    }
    ///`10`
    #[inline(always)]
    pub fn is_rgb565_ycbcr422(&self) -> bool {
        *self == ConvFormat::Rgb565Ycbcr422
    }
}
///Field `CONV_FORMAT` writer - Convert RGB565 to YCbCr422
pub type ConvFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2, ConvFormat>;
impl<'a, REG> ConvFormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///`0`
    #[inline(always)]
    pub fn noconvert(self) -> &'a mut crate::W<REG> {
        self.variant(ConvFormat::Noconvert)
    }
    ///`1`
    #[inline(always)]
    pub fn ycbcr422_rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(ConvFormat::Ycbcr422Rgb565)
    }
    ///`10`
    #[inline(always)]
    pub fn rgb565_ycbcr422(self) -> &'a mut crate::W<REG> {
        self.variant(ConvFormat::Rgb565Ycbcr422)
    }
}
/**Scale factor for Cb and Cr

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConvCalcSel {
    ///0: 16 .. 240 range
    From16to240range = 0,
    ///1: -127 .. 127 range
    Fromneg127to127range = 1,
}
impl From<ConvCalcSel> for bool {
    #[inline(always)]
    fn from(variant: ConvCalcSel) -> Self {
        variant as u8 != 0
    }
}
///Field `CONV_CALC_SEL` reader - Scale factor for Cb and Cr
pub type ConvCalcSelR = crate::BitReader<ConvCalcSel>;
impl ConvCalcSelR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ConvCalcSel {
        match self.bits {
            false => ConvCalcSel::From16to240range,
            true => ConvCalcSel::Fromneg127to127range,
        }
    }
    ///16 .. 240 range
    #[inline(always)]
    pub fn is_from16to240range(&self) -> bool {
        *self == ConvCalcSel::From16to240range
    }
    ///-127 .. 127 range
    #[inline(always)]
    pub fn is_fromneg127to127range(&self) -> bool {
        *self == ConvCalcSel::Fromneg127to127range
    }
}
///Field `CONV_CALC_SEL` writer - Scale factor for Cb and Cr
pub type ConvCalcSelW<'a, REG> = crate::BitWriter<'a, REG, ConvCalcSel>;
impl<'a, REG> ConvCalcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///16 .. 240 range
    #[inline(always)]
    pub fn from16to240range(self) -> &'a mut crate::W<REG> {
        self.variant(ConvCalcSel::From16to240range)
    }
    ///-127 .. 127 range
    #[inline(always)]
    pub fn fromneg127to127range(self) -> &'a mut crate::W<REG> {
        self.variant(ConvCalcSel::Fromneg127to127range)
    }
}
impl R {
    ///Bits 0:1 - Convert RGB565 to YCbCr422
    #[inline(always)]
    pub fn conv_format(&self) -> ConvFormatR {
        ConvFormatR::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Scale factor for Cb and Cr
    #[inline(always)]
    pub fn conv_calc_sel(&self) -> ConvCalcSelR {
        ConvCalcSelR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Convert RGB565 to YCbCr422
    #[inline(always)]
    pub fn conv_format(&mut self) -> ConvFormatW<'_, ConvCtrlSpec> {
        ConvFormatW::new(self, 0)
    }
    ///Bit 4 - Scale factor for Cb and Cr
    #[inline(always)]
    pub fn conv_calc_sel(&mut self) -> ConvCalcSelW<'_, ConvCtrlSpec> {
        ConvCalcSelW::new(self, 4)
    }
}
/**Color Convertion Control

You can [`read`](crate::Reg::read) this register and get [`conv_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ConvCtrlSpec;
impl crate::RegisterSpec for ConvCtrlSpec {
    type Ux = u32;
}
///`read()` method returns [`conv_ctrl::R`](R) reader structure
impl crate::Readable for ConvCtrlSpec {}
///`write(|w| ..)` method takes [`conv_ctrl::W`](W) writer structure
impl crate::Writable for ConvCtrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONV_CTRL to value 0
impl crate::Resettable for ConvCtrlSpec {}
