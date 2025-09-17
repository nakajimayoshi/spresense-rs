#[doc = "Register `STAT_NORMAL_DESCRIPTOR_ADDRESS` reader"]
pub type R = crate::R<StatNormalDescriptorAddressSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Normal Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_normal_descriptor_address::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatNormalDescriptorAddressSpec;
impl crate::RegisterSpec for StatNormalDescriptorAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_normal_descriptor_address::R`](R) reader structure"]
impl crate::Readable for StatNormalDescriptorAddressSpec {}
#[doc = "`reset()` method sets STAT_NORMAL_DESCRIPTOR_ADDRESS to value 0"]
impl crate::Resettable for StatNormalDescriptorAddressSpec {}
