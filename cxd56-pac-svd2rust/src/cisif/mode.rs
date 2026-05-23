///Register `MODE` reader
pub type R = crate::R<ModeSpec>;
///Register `MODE` writer
pub type W = crate::W<ModeSpec>;
///Field `cis_mode` reader -
pub type CisModeR = crate::FieldReader;
///Field `cis_mode` writer -
pub type CisModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ycc_trns_en` reader -
pub type YccTrnsEnR = crate::BitReader;
///Field `ycc_trns_en` writer -
pub type YccTrnsEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `jpg_trns_en` reader -
pub type JpgTrnsEnR = crate::BitReader;
///Field `jpg_trns_en` writer -
pub type JpgTrnsEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `jpg_cap_mode` reader -
pub type JpgCapModeR = crate::BitReader;
///Field `jpg_cap_mode` writer -
pub type JpgCapModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn cis_mode(&self) -> CisModeR {
        CisModeR::new((self.bits & 3) as u8)
    }
    ///Bit 2
    #[inline(always)]
    pub fn ycc_trns_en(&self) -> YccTrnsEnR {
        YccTrnsEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn jpg_trns_en(&self) -> JpgTrnsEnR {
        JpgTrnsEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn jpg_cap_mode(&self) -> JpgCapModeR {
        JpgCapModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    pub fn cis_mode(&mut self) -> CisModeW<'_, ModeSpec> {
        CisModeW::new(self, 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn ycc_trns_en(&mut self) -> YccTrnsEnW<'_, ModeSpec> {
        YccTrnsEnW::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn jpg_trns_en(&mut self) -> JpgTrnsEnW<'_, ModeSpec> {
        JpgTrnsEnW::new(self, 3)
    }
    ///Bit 8
    #[inline(always)]
    pub fn jpg_cap_mode(&mut self) -> JpgCapModeW<'_, ModeSpec> {
        JpgCapModeW::new(self, 8)
    }
}
/**CIS input mode register

You can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
///`read()` method returns [`mode::R`](R) reader structure
impl crate::Readable for ModeSpec {}
///`write(|w| ..)` method takes [`mode::W`](W) writer structure
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MODE to value 0
impl crate::Resettable for ModeSpec {}
