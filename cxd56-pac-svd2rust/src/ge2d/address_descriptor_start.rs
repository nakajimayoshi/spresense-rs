///Register `ADDRESS_DESCRIPTOR_START` reader
pub type R = crate::R<AddressDescriptorStartSpec>;
///Register `ADDRESS_DESCRIPTOR_START` writer
pub type W = crate::W<AddressDescriptorStartSpec>;
///Field `MSEL` reader - must be 1
pub type MselR = crate::BitReader;
///Field `MSEL` writer - must be 1
pub type MselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DA` reader - Descriptor Address
pub type DaR = crate::FieldReader<u32>;
///Field `DA` writer - Descriptor Address
pub type DaW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bit 0 - must be 1
    #[inline(always)]
    pub fn msel(&self) -> MselR {
        MselR::new((self.bits & 1) != 0)
    }
    ///Bits 4:31 - Descriptor Address
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    ///Bit 0 - must be 1
    #[inline(always)]
    pub fn msel(&mut self) -> MselW<'_, AddressDescriptorStartSpec> {
        MselW::new(self, 0)
    }
    ///Bits 4:31 - Descriptor Address
    #[inline(always)]
    pub fn da(&mut self) -> DaW<'_, AddressDescriptorStartSpec> {
        DaW::new(self, 4)
    }
}
/**Descriptor Address Set Register

You can [`read`](crate::Reg::read) this register and get [`address_descriptor_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address_descriptor_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AddressDescriptorStartSpec;
impl crate::RegisterSpec for AddressDescriptorStartSpec {
    type Ux = u32;
}
///`read()` method returns [`address_descriptor_start::R`](R) reader structure
impl crate::Readable for AddressDescriptorStartSpec {}
///`write(|w| ..)` method takes [`address_descriptor_start::W`](W) writer structure
impl crate::Writable for AddressDescriptorStartSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRESS_DESCRIPTOR_START to value 0
impl crate::Resettable for AddressDescriptorStartSpec {}
