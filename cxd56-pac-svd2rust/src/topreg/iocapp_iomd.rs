///Register `IOCAPP_IOMD` reader
pub type R = crate::R<IocappIomdSpec>;
///Register `IOCAPP_IOMD` writer
pub type W = crate::W<IocappIomdSpec>;
///Field `UART2` reader - Mode select for UART2 TXD/RXD pins (UART2 = Func1)
pub type Uart2R = crate::FieldReader;
///Field `UART2` writer - Mode select for UART2 TXD/RXD pins (UART2 = Func1)
pub type Uart2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EMMCA` reader - Mode select for EMMC_CLK/CMD/DATA0/DATA1 pins (Func1=eMMC, Func2=SPI5)
pub type EmmcaR = crate::FieldReader;
///Field `EMMCA` writer - Mode select for EMMC_CLK/CMD/DATA0/DATA1 pins (Func1=eMMC, Func2=SPI5)
pub type EmmcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EMMCB` reader - Mode select for EMMC_DATA2/DATA3 pins (Func1=eMMC)
pub type EmmcbR = crate::FieldReader;
///Field `EMMCB` writer - Mode select for EMMC_DATA2/DATA3 pins (Func1=eMMC)
pub type EmmcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2S0` reader - Mode select for I2S0 BCK/LRCK/DATA_IN/DATA_OUT pins (I2S0 = Func1)
pub type I2s0R = crate::FieldReader;
///Field `I2S0` writer - Mode select for I2S0 BCK/LRCK/DATA_IN/DATA_OUT pins (I2S0 = Func1)
pub type I2s0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 2:3 - Mode select for UART2 TXD/RXD pins (UART2 = Func1)
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - Mode select for EMMC_CLK/CMD/DATA0/DATA1 pins (Func1=eMMC, Func2=SPI5)
    #[inline(always)]
    pub fn emmca(&self) -> EmmcaR {
        EmmcaR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Mode select for EMMC_DATA2/DATA3 pins (Func1=eMMC)
    #[inline(always)]
    pub fn emmcb(&self) -> EmmcbR {
        EmmcbR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 18:19 - Mode select for I2S0 BCK/LRCK/DATA_IN/DATA_OUT pins (I2S0 = Func1)
    #[inline(always)]
    pub fn i2s0(&self) -> I2s0R {
        I2s0R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    ///Bits 2:3 - Mode select for UART2 TXD/RXD pins (UART2 = Func1)
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, IocappIomdSpec> {
        Uart2W::new(self, 2)
    }
    ///Bits 6:7 - Mode select for EMMC_CLK/CMD/DATA0/DATA1 pins (Func1=eMMC, Func2=SPI5)
    #[inline(always)]
    pub fn emmca(&mut self) -> EmmcaW<'_, IocappIomdSpec> {
        EmmcaW::new(self, 6)
    }
    ///Bits 8:9 - Mode select for EMMC_DATA2/DATA3 pins (Func1=eMMC)
    #[inline(always)]
    pub fn emmcb(&mut self) -> EmmcbW<'_, IocappIomdSpec> {
        EmmcbW::new(self, 8)
    }
    ///Bits 18:19 - Mode select for I2S0 BCK/LRCK/DATA_IN/DATA_OUT pins (I2S0 = Func1)
    #[inline(always)]
    pub fn i2s0(&mut self) -> I2s0W<'_, IocappIomdSpec> {
        I2s0W::new(self, 18)
    }
}
/**APP-domain IO-cell mode-mux register

You can [`read`](crate::Reg::read) this register and get [`iocapp_iomd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocapp_iomd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IocappIomdSpec;
impl crate::RegisterSpec for IocappIomdSpec {
    type Ux = u32;
}
///`read()` method returns [`iocapp_iomd::R`](R) reader structure
impl crate::Readable for IocappIomdSpec {}
///`write(|w| ..)` method takes [`iocapp_iomd::W`](W) writer structure
impl crate::Writable for IocappIomdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCAPP_IOMD to value 0
impl crate::Resettable for IocappIomdSpec {}
