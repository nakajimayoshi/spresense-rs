///Register `SCU_CKEN` reader
pub type R = crate::R<ScuCkenSpec>;
///Register `SCU_CKEN` writer
pub type W = crate::W<ScuCkenSpec>;
///Field `SCU_SCU` reader - SCU core clock enable
pub type ScuScuR = crate::BitReader;
///Field `SCU_SCU` writer - SCU core clock enable
pub type ScuScuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCU_I2C0` reader - SCU I2C0 clock enable
pub type ScuI2c0R = crate::BitReader;
///Field `SCU_I2C0` writer - SCU I2C0 clock enable
pub type ScuI2c0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCU_I2C1` reader - SCU I2C1 clock enable
pub type ScuI2c1R = crate::BitReader;
///Field `SCU_I2C1` writer - SCU I2C1 clock enable
pub type ScuI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCU_SPI` reader - SCU SPI clock enable
pub type ScuSpiR = crate::BitReader;
///Field `SCU_SPI` writer - SCU SPI clock enable
pub type ScuSpiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCU_SEQ` reader - SCU sequencer clock enable
pub type ScuSeqR = crate::BitReader;
///Field `SCU_SEQ` writer - SCU sequencer clock enable
pub type ScuSeqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCU_32K` reader - SCU 32 kHz clock enable
pub type Scu32kR = crate::BitReader;
///Field `SCU_32K` writer - SCU 32 kHz clock enable
pub type Scu32kW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCU_U32KL` reader - SCU U32K low clock enable
pub type ScuU32klR = crate::BitReader;
///Field `SCU_U32KL` writer - SCU U32K low clock enable
pub type ScuU32klW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCU_U32KH` reader - SCU U32K high clock enable
pub type ScuU32khR = crate::BitReader;
///Field `SCU_U32KH` writer - SCU U32K high clock enable
pub type ScuU32khW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCU_SC` reader - SCU SC clock enable
pub type ScuScR = crate::BitReader;
///Field `SCU_SC` writer - SCU SC clock enable
pub type ScuScW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SCU core clock enable
    #[inline(always)]
    pub fn scu_scu(&self) -> ScuScuR {
        ScuScuR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SCU I2C0 clock enable
    #[inline(always)]
    pub fn scu_i2c0(&self) -> ScuI2c0R {
        ScuI2c0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SCU I2C1 clock enable
    #[inline(always)]
    pub fn scu_i2c1(&self) -> ScuI2c1R {
        ScuI2c1R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SCU SPI clock enable
    #[inline(always)]
    pub fn scu_spi(&self) -> ScuSpiR {
        ScuSpiR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SCU sequencer clock enable
    #[inline(always)]
    pub fn scu_seq(&self) -> ScuSeqR {
        ScuSeqR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SCU 32 kHz clock enable
    #[inline(always)]
    pub fn scu_32k(&self) -> Scu32kR {
        Scu32kR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SCU U32K low clock enable
    #[inline(always)]
    pub fn scu_u32kl(&self) -> ScuU32klR {
        ScuU32klR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SCU U32K high clock enable
    #[inline(always)]
    pub fn scu_u32kh(&self) -> ScuU32khR {
        ScuU32khR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SCU SC clock enable
    #[inline(always)]
    pub fn scu_sc(&self) -> ScuScR {
        ScuScR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SCU core clock enable
    #[inline(always)]
    pub fn scu_scu(&mut self) -> ScuScuW<'_, ScuCkenSpec> {
        ScuScuW::new(self, 0)
    }
    ///Bit 1 - SCU I2C0 clock enable
    #[inline(always)]
    pub fn scu_i2c0(&mut self) -> ScuI2c0W<'_, ScuCkenSpec> {
        ScuI2c0W::new(self, 1)
    }
    ///Bit 2 - SCU I2C1 clock enable
    #[inline(always)]
    pub fn scu_i2c1(&mut self) -> ScuI2c1W<'_, ScuCkenSpec> {
        ScuI2c1W::new(self, 2)
    }
    ///Bit 3 - SCU SPI clock enable
    #[inline(always)]
    pub fn scu_spi(&mut self) -> ScuSpiW<'_, ScuCkenSpec> {
        ScuSpiW::new(self, 3)
    }
    ///Bit 4 - SCU sequencer clock enable
    #[inline(always)]
    pub fn scu_seq(&mut self) -> ScuSeqW<'_, ScuCkenSpec> {
        ScuSeqW::new(self, 4)
    }
    ///Bit 5 - SCU 32 kHz clock enable
    #[inline(always)]
    pub fn scu_32k(&mut self) -> Scu32kW<'_, ScuCkenSpec> {
        Scu32kW::new(self, 5)
    }
    ///Bit 6 - SCU U32K low clock enable
    #[inline(always)]
    pub fn scu_u32kl(&mut self) -> ScuU32klW<'_, ScuCkenSpec> {
        ScuU32klW::new(self, 6)
    }
    ///Bit 7 - SCU U32K high clock enable
    #[inline(always)]
    pub fn scu_u32kh(&mut self) -> ScuU32khW<'_, ScuCkenSpec> {
        ScuU32khW::new(self, 7)
    }
    ///Bit 8 - SCU SC clock enable
    #[inline(always)]
    pub fn scu_sc(&mut self) -> ScuScW<'_, ScuCkenSpec> {
        ScuScW::new(self, 8)
    }
}
/**SCU peripheral clock enables

You can [`read`](crate::Reg::read) this register and get [`scu_cken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu_cken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ScuCkenSpec;
impl crate::RegisterSpec for ScuCkenSpec {
    type Ux = u32;
}
///`read()` method returns [`scu_cken::R`](R) reader structure
impl crate::Readable for ScuCkenSpec {}
///`write(|w| ..)` method takes [`scu_cken::W`](W) writer structure
impl crate::Writable for ScuCkenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCU_CKEN to value 0
impl crate::Resettable for ScuCkenSpec {}
