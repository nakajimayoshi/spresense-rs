#[doc = "Register `DMACEnbldChns` reader"]
pub type R = crate::R<DmacenbldChnsSpec>;
#[doc = "Field `EnabledChannels` reader - Channel enable status"]
pub type EnabledChannelsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Channel enable status"]
    #[inline(always)]
    pub fn enabled_channels(&self) -> EnabledChannelsR {
        EnabledChannelsR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacenbld_chns::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacenbldChnsSpec;
impl crate::RegisterSpec for DmacenbldChnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacenbld_chns::R`](R) reader structure"]
impl crate::Readable for DmacenbldChnsSpec {}
#[doc = "`reset()` method sets DMACEnbldChns to value 0"]
impl crate::Resettable for DmacenbldChnsSpec {}
