///Register `USBPHY_CKEN` reader
pub type R = crate::R<UsbphyCkenSpec>;
///Register `USBPHY_CKEN` writer
pub type W = crate::W<UsbphyCkenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**USB PHY clock enable

You can [`read`](crate::Reg::read) this register and get [`usbphy_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UsbphyCkenSpec;
impl crate::RegisterSpec for UsbphyCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`usbphy_cken::R`](R) reader structure
impl crate::Readable for UsbphyCkenSpec {}
///`write(|w| ..)` method takes [`usbphy_cken::W`](W) writer structure
impl crate::Writable for UsbphyCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBPHY_CKEN to value 0
impl crate::Resettable for UsbphyCkenSpec {}
