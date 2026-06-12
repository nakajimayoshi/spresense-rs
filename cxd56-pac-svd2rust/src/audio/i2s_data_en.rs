///Register `I2S_DATA_EN` reader
pub type R = crate::R<I2sDataEnSpec>;
///Register `I2S_DATA_EN` writer
pub type W = crate::W<I2sDataEnSpec>;
///Field `SDIN1_EN` reader - I2S0 serial data input enable
pub type Sdin1EnR = crate::BitReader;
///Field `SDIN1_EN` writer - I2S0 serial data input enable
pub type Sdin1EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDOUT1_EN` reader - I2S0 serial data output enable
pub type Sdout1EnR = crate::BitReader;
///Field `SDOUT1_EN` writer - I2S0 serial data output enable
pub type Sdout1EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLF_EN` reader - BLF (output filter) enable
pub type BlfEnR = crate::BitReader;
///Field `BLF_EN` writer - BLF (output filter) enable
pub type BlfEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2S0 serial data input enable
    #[inline(always)]
    pub fn sdin1_en(&self) -> Sdin1EnR {
        Sdin1EnR::new((self.bits & 1) != 0)
    }
    ///Bit 2 - I2S0 serial data output enable
    #[inline(always)]
    pub fn sdout1_en(&self) -> Sdout1EnR {
        Sdout1EnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - BLF (output filter) enable
    #[inline(always)]
    pub fn blf_en(&self) -> BlfEnR {
        BlfEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I2S0 serial data input enable
    #[inline(always)]
    pub fn sdin1_en(&mut self) -> Sdin1EnW<'_, I2sDataEnSpec> {
        Sdin1EnW::new(self, 0)
    }
    ///Bit 2 - I2S0 serial data output enable
    #[inline(always)]
    pub fn sdout1_en(&mut self) -> Sdout1EnW<'_, I2sDataEnSpec> {
        Sdout1EnW::new(self, 2)
    }
    ///Bit 5 - BLF (output filter) enable
    #[inline(always)]
    pub fn blf_en(&mut self) -> BlfEnW<'_, I2sDataEnSpec> {
        BlfEnW::new(self, 5)
    }
}
/**Audio output control — I2S0 (SD1) serial data path enables (this register also holds line-in/DAC volume; only the I2S-relevant bits are modelled)

You can [`read`](crate::Reg::read) this register and get [`i2s_data_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_data_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2sDataEnSpec;
impl crate::RegisterSpec for I2sDataEnSpec {
    type Ux = u32;
}
///`read()` method returns [`i2s_data_en::R`](R) reader structure
impl crate::Readable for I2sDataEnSpec {}
///`write(|w| ..)` method takes [`i2s_data_en::W`](W) writer structure
impl crate::Writable for I2sDataEnSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2S_DATA_EN to value 0x0033_3300
impl crate::Resettable for I2sDataEnSpec {
    const RESET_VALUE: u32 = 0x0033_3300;
}
