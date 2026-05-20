#[doc = "Register `STAT_CURRENT_DESCRIPTOR_ADDRESS` reader"]
pub type R = crate::R<StatCurrentDescriptorAddressSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Current Descriptor Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_current_descriptor_address::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatCurrentDescriptorAddressSpec;
impl crate::RegisterSpec for StatCurrentDescriptorAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_current_descriptor_address::R`](R) reader structure"]
impl crate::Readable for StatCurrentDescriptorAddressSpec {}
#[doc = "`reset()` method sets STAT_CURRENT_DESCRIPTOR_ADDRESS to value 0"]
impl crate::Resettable for StatCurrentDescriptorAddressSpec {}
