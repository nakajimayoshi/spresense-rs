///Register `FORMAT` reader
pub type R = crate::R<FormatSpec>;
///Register `FORMAT` writer
pub type W = crate::W<FormatSpec>;
/**

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YcOrder {
    ///0: Y/Cb/Y/Cr
    YcbYcr = 0,
    ///1: Y/Cr/Y/Cb
    YcrYcb = 1,
    ///2: Cb/Y/Cr/Y
    CbYcrY = 2,
    ///3: Cr/Y/Cb/Y
    CrYcbY = 3,
}
impl From<YcOrder> for u8 {
    #[inline(always)]
    fn from(variant: YcOrder) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for YcOrder {
    type Ux = u8;
}
impl crate::IsEnum for YcOrder {}
///Field `yc_order` reader -
pub type YcOrderR = crate::FieldReader<YcOrder>;
impl YcOrderR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> YcOrder {
        match self.bits {
            0 => YcOrder::YcbYcr,
            1 => YcOrder::YcrYcb,
            2 => YcOrder::CbYcrY,
            3 => YcOrder::CrYcbY,
            _ => unreachable!(),
        }
    }
    ///Y/Cb/Y/Cr
    #[inline(always)]
    pub fn is_ycb_ycr(&self) -> bool {
        *self == YcOrder::YcbYcr
    }
    ///Y/Cr/Y/Cb
    #[inline(always)]
    pub fn is_ycr_ycb(&self) -> bool {
        *self == YcOrder::YcrYcb
    }
    ///Cb/Y/Cr/Y
    #[inline(always)]
    pub fn is_cb_ycr_y(&self) -> bool {
        *self == YcOrder::CbYcrY
    }
    ///Cr/Y/Cb/Y
    #[inline(always)]
    pub fn is_cr_ycb_y(&self) -> bool {
        *self == YcOrder::CrYcbY
    }
}
///Field `yc_order` writer -
pub type YcOrderW<'a, REG> = crate::FieldWriter<'a, REG, 2, YcOrder, crate::Safe>;
impl<'a, REG> YcOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Y/Cb/Y/Cr
    #[inline(always)]
    pub fn ycb_ycr(self) -> &'a mut crate::W<REG> {
        self.variant(YcOrder::YcbYcr)
    }
    ///Y/Cr/Y/Cb
    #[inline(always)]
    pub fn ycr_ycb(self) -> &'a mut crate::W<REG> {
        self.variant(YcOrder::YcrYcb)
    }
    ///Cb/Y/Cr/Y
    #[inline(always)]
    pub fn cb_ycr_y(self) -> &'a mut crate::W<REG> {
        self.variant(YcOrder::CbYcrY)
    }
    ///Cr/Y/Cb/Y
    #[inline(always)]
    pub fn cr_ycb_y(self) -> &'a mut crate::W<REG> {
        self.variant(YcOrder::CrYcbY)
    }
}
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn yc_order(&self) -> YcOrderR {
        YcOrderR::new((self.bits & 3) as u8)
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    pub fn yc_order(&mut self) -> YcOrderW<'_, FormatSpec> {
        YcOrderW::new(self, 0)
    }
}
/**CIS input data format setting register

You can [`read`](crate::Reg::read) this register and get [`format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FormatSpec;
impl crate::RegisterSpec for FormatSpec {
    type Ux = u32;
}
///`read()` method returns [`format::R`](R) reader structure
impl crate::Readable for FormatSpec {}
///`write(|w| ..)` method takes [`format::W`](W) writer structure
impl crate::Writable for FormatSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FORMAT to value 0
impl crate::Resettable for FormatSpec {}
