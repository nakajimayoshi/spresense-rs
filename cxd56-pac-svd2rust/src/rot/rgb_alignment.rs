#[doc = "Register `RGB_ALIGNMENT` reader"]
pub type R = crate::R<RgbAlignmentSpec>;
#[doc = "Register `RGB_ALIGNMENT` writer"]
pub type W = crate::W<RgbAlignmentSpec>;
#[doc = "RGB Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    #[doc = "0: `0`"]
    Rgb = 0,
    #[doc = "1: `1`"]
    Bgr = 1,
}
impl From<Format> for bool {
    #[inline(always)]
    fn from(variant: Format) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORMAT` reader - RGB Format"]
pub type FormatR = crate::BitReader<Format>;
impl FormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Format {
        match self.bits {
            false => Format::Rgb,
            true => Format::Bgr,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == Format::Rgb
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        *self == Format::Bgr
    }
}
#[doc = "Field `FORMAT` writer - RGB Format"]
pub type FormatW<'a, REG> = crate::BitWriter<'a, REG, Format>;
impl<'a, REG> FormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut crate::W<REG> {
        self.variant(Format::Rgb)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut crate::W<REG> {
        self.variant(Format::Bgr)
    }
}
impl R {
    #[doc = "Bit 0 - RGB Format"]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RGB Format"]
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<'_, RgbAlignmentSpec> {
        FormatW::new(self, 0)
    }
}
#[doc = "RGB format selector\n\nYou can [`read`](crate::Reg::read) this register and get [`rgb_alignment::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgb_alignment::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgbAlignmentSpec;
impl crate::RegisterSpec for RgbAlignmentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgb_alignment::R`](R) reader structure"]
impl crate::Readable for RgbAlignmentSpec {}
#[doc = "`write(|w| ..)` method takes [`rgb_alignment::W`](W) writer structure"]
impl crate::Writable for RgbAlignmentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RGB_ALIGNMENT to value 0"]
impl crate::Resettable for RgbAlignmentSpec {}
