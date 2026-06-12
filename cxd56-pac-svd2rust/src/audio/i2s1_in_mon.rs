///Register `I2S1_IN_MON` reader
pub type R = crate::R<I2s1InMonSpec>;
///Field `MON_START` reader - DMA running
pub type MonStartR = crate::BitReader;
///Field `MON_ERROR_SETTING` reader - Channel-setting error code
pub type MonErrorSettingR = crate::FieldReader;
///Field `MON_MONBUF` reader - Internal buffer state (0=empty)
pub type MonMonbufR = crate::FieldReader;
impl R {
    ///Bit 0 - DMA running
    #[inline(always)]
    pub fn mon_start(&self) -> MonStartR {
        MonStartR::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - Channel-setting error code
    #[inline(always)]
    pub fn mon_error_setting(&self) -> MonErrorSettingR {
        MonErrorSettingR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Internal buffer state (0=empty)
    #[inline(always)]
    pub fn mon_monbuf(&self) -> MonMonbufR {
        MonMonbufR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
/**I2S0 RX DMA monitor (read-only)

You can [`read`](crate::Reg::read) this register and get [`i2s1_in_mon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2s1InMonSpec;
impl crate::RegisterSpec for I2s1InMonSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s1_in_mon::R`](R) reader structure
impl crate::Readable for I2s1InMonSpec {}
///`reset()` method sets I2S1_IN_MON to value 0
impl crate::Resettable for I2s1InMonSpec {}
