///Register `SET_DST_ADDRESS` reader
pub type R = crate::R<SetDstAddressSpec>;
///Register `SET_DST_ADDRESS` writer
pub type W = crate::W<SetDstAddressSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Destination Address

You can [`read`](crate::Reg::read) this register and get [`set_dst_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_dst_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SetDstAddressSpec;
impl crate::RegisterSpec for SetDstAddressSpec {
    type Ux = u32;
}
///`read()` method returns [`set_dst_address::R`](R) reader structure
impl crate::Readable for SetDstAddressSpec {}
///`write(|w| ..)` method takes [`set_dst_address::W`](W) writer structure
impl crate::Writable for SetDstAddressSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SET_DST_ADDRESS to value 0
impl crate::Resettable for SetDstAddressSpec {}
