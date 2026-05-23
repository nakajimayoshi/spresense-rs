///Application domain clock enables.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct APP_CKEN(pub u32);
impl APP_CKEN {
    ///APP CPU clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn APP_CPU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///APP CPU clock enable.
    #[inline(always)]
    pub const fn set_APP_CPU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///APP MCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn APP_MCLK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///APP MCLK enable.
    #[inline(always)]
    pub const fn set_APP_MCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///APP AHB clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn APP_AHB(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///APP AHB clock enable.
    #[inline(always)]
    pub const fn set_APP_AHB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for APP_CKEN {
    #[inline(always)]
    fn default() -> APP_CKEN {
        APP_CKEN(0)
    }
}
impl core::fmt::Debug for APP_CKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CKEN")
            .field("APP_CPU", &self.APP_CPU())
            .field("APP_MCLK", &self.APP_MCLK())
            .field("APP_AHB", &self.APP_AHB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for APP_CKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "APP_CKEN {{ APP_CPU: {=bool:?}, APP_MCLK: {=bool:?}, APP_AHB: {=bool:?} }}",
            self.APP_CPU(),
            self.APP_MCLK(),
            self.APP_AHB()
        )
    }
}
///Application domain clock source select.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct APP_CKSEL(pub u32);
impl APP_CKSEL {
    ///Audio MCLK source select.
    #[must_use]
    #[inline(always)]
    pub const fn AUD_MCLK(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    ///Audio MCLK source select.
    #[inline(always)]
    pub const fn set_AUD_MCLK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for APP_CKSEL {
    #[inline(always)]
    fn default() -> APP_CKSEL {
        APP_CKSEL(0)
    }
}
impl core::fmt::Debug for APP_CKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CKSEL")
            .field("AUD_MCLK", &self.AUD_MCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for APP_CKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "APP_CKSEL {{ AUD_MCLK: {=u8:?} }}", self.AUD_MCLK())
    }
}
///GNSS DSP clock enables.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GNSDSP_CKEN(pub u32);
impl GNSDSP_CKEN {
    ///GNSS DSP P1 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn GNSDSP_P1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///GNSS DSP P1 clock enable.
    #[inline(always)]
    pub const fn set_GNSDSP_P1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///GNSS DSP coprocessor clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn GNSDSP_COP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///GNSS DSP coprocessor clock enable.
    #[inline(always)]
    pub const fn set_GNSDSP_COP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for GNSDSP_CKEN {
    #[inline(always)]
    fn default() -> GNSDSP_CKEN {
        GNSDSP_CKEN(0)
    }
}
impl core::fmt::Debug for GNSDSP_CKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNSDSP_CKEN")
            .field("GNSDSP_P1", &self.GNSDSP_P1())
            .field("GNSDSP_COP", &self.GNSDSP_COP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GNSDSP_CKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GNSDSP_CKEN {{ GNSDSP_P1: {=bool:?}, GNSDSP_COP: {=bool:?} }}",
            self.GNSDSP_P1(),
            self.GNSDSP_COP()
        )
    }
}
///SYSIOP sub-domain peripheral clock enables.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSIOP_SUB_CKEN(pub u32);
impl SYSIOP_SUB_CKEN {
    ///AHB bridge COMIF clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_AHB_BRG_COMIF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///AHB bridge COMIF clock enable.
    #[inline(always)]
    pub const fn set_CK_AHB_BRG_COMIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///COM bridge clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_COM_BRG(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///COM bridge clock enable.
    #[inline(always)]
    pub const fn set_CK_COM_BRG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///AHB DMAC3 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_AHB_DMAC3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///AHB DMAC3 clock enable.
    #[inline(always)]
    pub const fn set_CK_AHB_DMAC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///UART1 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_UART1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///UART1 clock enable.
    #[inline(always)]
    pub const fn set_CK_UART1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///SPI master clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SPIM(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///SPI master clock enable.
    #[inline(always)]
    pub const fn set_CK_SPIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///I2C master clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_I2CM(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///I2C master clock enable.
    #[inline(always)]
    pub const fn set_CK_I2CM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///SAKE HCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_HCLK_SAKE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///SAKE HCLK enable.
    #[inline(always)]
    pub const fn set_CK_HCLK_SAKE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Serial flash controller HCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SFC_HCLK(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///Serial flash controller HCLK enable.
    #[inline(always)]
    pub const fn set_CK_SFC_HCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///Serial flash controller SFCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SFC_SFCLK(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Serial flash controller SFCLK enable.
    #[inline(always)]
    pub const fn set_CK_SFC_SFCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Serial flash controller HCLK low-speed enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SFC_HCLK_LOW(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Serial flash controller HCLK low-speed enable.
    #[inline(always)]
    pub const fn set_CK_SFC_HCLK_LOW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///COM UART PCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CK_COM_UART_PCLK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///COM UART PCLK enable.
    #[inline(always)]
    pub const fn set_CK_COM_UART_PCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for SYSIOP_SUB_CKEN {
    #[inline(always)]
    fn default() -> SYSIOP_SUB_CKEN {
        SYSIOP_SUB_CKEN(0)
    }
}
impl core::fmt::Debug for SYSIOP_SUB_CKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSIOP_SUB_CKEN")
            .field("CK_AHB_BRG_COMIF", &self.CK_AHB_BRG_COMIF())
            .field("CK_COM_BRG", &self.CK_COM_BRG())
            .field("CK_AHB_DMAC3", &self.CK_AHB_DMAC3())
            .field("CK_UART1", &self.CK_UART1())
            .field("CK_SPIM", &self.CK_SPIM())
            .field("CK_I2CM", &self.CK_I2CM())
            .field("CK_HCLK_SAKE", &self.CK_HCLK_SAKE())
            .field("CK_SFC_HCLK", &self.CK_SFC_HCLK())
            .field("CK_SFC_SFCLK", &self.CK_SFC_SFCLK())
            .field("CK_SFC_HCLK_LOW", &self.CK_SFC_HCLK_LOW())
            .field("CK_COM_UART_PCLK", &self.CK_COM_UART_PCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYSIOP_SUB_CKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYSIOP_SUB_CKEN {{ CK_AHB_BRG_COMIF: {=bool:?}, CK_COM_BRG: {=bool:?}, CK_AHB_DMAC3: {=bool:?}, CK_UART1: {=bool:?}, CK_SPIM: {=bool:?}, CK_I2CM: {=bool:?}, CK_HCLK_SAKE: {=bool:?}, CK_SFC_HCLK: {=bool:?}, CK_SFC_SFCLK: {=bool:?}, CK_SFC_HCLK_LOW: {=bool:?}, CK_COM_UART_PCLK: {=bool:?} }}",
            self.CK_AHB_BRG_COMIF(),
            self.CK_COM_BRG(),
            self.CK_AHB_DMAC3(),
            self.CK_UART1(),
            self.CK_SPIM(),
            self.CK_I2CM(),
            self.CK_HCLK_SAKE(),
            self.CK_SFC_HCLK(),
            self.CK_SFC_SFCLK(),
            self.CK_SFC_HCLK_LOW(),
            self.CK_COM_UART_PCLK()
        )
    }
}
