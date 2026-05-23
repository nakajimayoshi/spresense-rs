///Register `CIS_SIZE` reader
pub type R = crate::R<CisSizeSpec>;
///Register `CIS_SIZE` writer
pub type W = crate::W<CisSizeSpec>;
///Field `cis_hst` reader -
pub type CisHstR = crate::FieldReader<u16>;
///Field `cis_hst` writer -
pub type CisHstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `cis_vst` reader -
pub type CisVstR = crate::FieldReader<u16>;
///Field `cis_vst` writer -
pub type CisVstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn cis_hst(&self) -> CisHstR {
        CisHstR::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn cis_vst(&self) -> CisVstR {
        CisVstR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    pub fn cis_hst(&mut self) -> CisHstW<'_, CisSizeSpec> {
        CisHstW::new(self, 0)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn cis_vst(&mut self) -> CisVstW<'_, CisSizeSpec> {
        CisVstW::new(self, 16)
    }
}
/**CIS input activa area size setting register

You can [`read`](crate::Reg::read) this register and get [`cis_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cis_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CisSizeSpec;
impl crate::RegisterSpec for CisSizeSpec {
    type Ux = u32;
}
///`read()` method returns [`cis_size::R`](R) reader structure
impl crate::Readable for CisSizeSpec {}
///`write(|w| ..)` method takes [`cis_size::W`](W) writer structure
impl crate::Writable for CisSizeSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIS_SIZE to value 0
impl crate::Resettable for CisSizeSpec {}
