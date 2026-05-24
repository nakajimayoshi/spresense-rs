///Register `IOCSYS_IOMD0` reader
pub type R = crate::R<IocsysIomd0Spec>;
///Register `IOCSYS_IOMD0` writer
pub type W = crate::W<IocsysIomd0Spec>;
///Field `SPI0A` reader - Mode select for SPI0_CS_X / SPI0_SCK (UART1 = Func1)
pub type Spi0aR = crate::FieldReader;
///Field `SPI0A` writer - Mode select for SPI0_CS_X / SPI0_SCK (UART1 = Func1)
pub type Spi0aW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 12:13 - Mode select for SPI0_CS_X / SPI0_SCK (UART1 = Func1)
    #[inline(always)]
    pub fn spi0a(&self) -> Spi0aR {
        Spi0aR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    ///Bits 12:13 - Mode select for SPI0_CS_X / SPI0_SCK (UART1 = Func1)
    #[inline(always)]
    pub fn spi0a(&mut self) -> Spi0aW<'_, IocsysIomd0Spec> {
        Spi0aW::new(self, 12)
    }
}
/**SYSIOP IO-cell mode-mux register 0

You can [`read`](crate::Reg::read) this register and get [`iocsys_iomd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocsys_iomd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IocsysIomd0Spec;
impl crate::RegisterSpec for IocsysIomd0Spec {
    type Ux = u32;
}
///`read()` method returns [`iocsys_iomd0::R`](R) reader structure
impl crate::Readable for IocsysIomd0Spec {}
///`write(|w| ..)` method takes [`iocsys_iomd0::W`](W) writer structure
impl crate::Writable for IocsysIomd0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCSYS_IOMD0 to value 0
impl crate::Resettable for IocsysIomd0Spec {}
