///Analog circuit power control (1 = powered on).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANA_PW_CTL(pub u32);
impl ANA_PW_CTL {
    ///RC oscillator.
    #[must_use]
    #[inline(always)]
    pub const fn RCOSC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///RC oscillator.
    #[inline(always)]
    pub const fn set_RCOSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Crystal oscillator.
    #[must_use]
    #[inline(always)]
    pub const fn XOSC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Crystal oscillator.
    #[inline(always)]
    pub const fn set_XOSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///System PLL.
    #[must_use]
    #[inline(always)]
    pub const fn SYSPLL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///System PLL.
    #[inline(always)]
    pub const fn set_SYSPLL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///RF LNA.
    #[must_use]
    #[inline(always)]
    pub const fn RF_LNA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///RF LNA.
    #[inline(always)]
    pub const fn set_RF_LNA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///RF mixer.
    #[must_use]
    #[inline(always)]
    pub const fn RF_MIX(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///RF mixer.
    #[inline(always)]
    pub const fn set_RF_MIX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///RF IF block.
    #[must_use]
    #[inline(always)]
    pub const fn RF_IF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///RF IF block.
    #[inline(always)]
    pub const fn set_RF_IF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///RF ADC.
    #[must_use]
    #[inline(always)]
    pub const fn RF_ADC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///RF ADC.
    #[inline(always)]
    pub const fn set_RF_ADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///RF local oscillator.
    #[must_use]
    #[inline(always)]
    pub const fn RF_LO(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///RF local oscillator.
    #[inline(always)]
    pub const fn set_RF_LO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///RF PLL.
    #[must_use]
    #[inline(always)]
    pub const fn RF_PLL(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///RF PLL.
    #[inline(always)]
    pub const fn set_RF_PLL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///High-precision ADC.
    #[must_use]
    #[inline(always)]
    pub const fn HPADC(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///High-precision ADC.
    #[inline(always)]
    pub const fn set_HPADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///Low-power ADC.
    #[must_use]
    #[inline(always)]
    pub const fn LPADC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///Low-power ADC.
    #[inline(always)]
    pub const fn set_LPADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for ANA_PW_CTL {
    #[inline(always)]
    fn default() -> ANA_PW_CTL {
        ANA_PW_CTL(0)
    }
}
impl core::fmt::Debug for ANA_PW_CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_PW_CTL")
            .field("RCOSC", &self.RCOSC())
            .field("XOSC", &self.XOSC())
            .field("SYSPLL", &self.SYSPLL())
            .field("RF_LNA", &self.RF_LNA())
            .field("RF_MIX", &self.RF_MIX())
            .field("RF_IF", &self.RF_IF())
            .field("RF_ADC", &self.RF_ADC())
            .field("RF_LO", &self.RF_LO())
            .field("RF_PLL", &self.RF_PLL())
            .field("HPADC", &self.HPADC())
            .field("LPADC", &self.LPADC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANA_PW_CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANA_PW_CTL {{ RCOSC: {=bool:?}, XOSC: {=bool:?}, SYSPLL: {=bool:?}, RF_LNA: {=bool:?}, RF_MIX: {=bool:?}, RF_IF: {=bool:?}, RF_ADC: {=bool:?}, RF_LO: {=bool:?}, RF_PLL: {=bool:?}, HPADC: {=bool:?}, LPADC: {=bool:?} }}",
            self.RCOSC(),
            self.XOSC(),
            self.SYSPLL(),
            self.RF_LNA(),
            self.RF_MIX(),
            self.RF_IF(),
            self.RF_ADC(),
            self.RF_LO(),
            self.RF_PLL(),
            self.HPADC(),
            self.LPADC()
        )
    }
}
///Analog power status (read-only mirror of ANA_PW_CTL).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANA_PW_STAT(pub u32);
impl ANA_PW_STAT {
    ///RC oscillator powered on.
    #[must_use]
    #[inline(always)]
    pub const fn RCOSC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///RC oscillator powered on.
    #[inline(always)]
    pub const fn set_RCOSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Crystal oscillator powered on.
    #[must_use]
    #[inline(always)]
    pub const fn XOSC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Crystal oscillator powered on.
    #[inline(always)]
    pub const fn set_XOSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///System PLL powered on.
    #[must_use]
    #[inline(always)]
    pub const fn SYSPLL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///System PLL powered on.
    #[inline(always)]
    pub const fn set_SYSPLL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///RF LNA powered on.
    #[must_use]
    #[inline(always)]
    pub const fn RF_LNA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///RF LNA powered on.
    #[inline(always)]
    pub const fn set_RF_LNA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///RF mixer powered on.
    #[must_use]
    #[inline(always)]
    pub const fn RF_MIX(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///RF mixer powered on.
    #[inline(always)]
    pub const fn set_RF_MIX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///RF IF block powered on.
    #[must_use]
    #[inline(always)]
    pub const fn RF_IF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///RF IF block powered on.
    #[inline(always)]
    pub const fn set_RF_IF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///RF ADC powered on.
    #[must_use]
    #[inline(always)]
    pub const fn RF_ADC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///RF ADC powered on.
    #[inline(always)]
    pub const fn set_RF_ADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///RF LO powered on.
    #[must_use]
    #[inline(always)]
    pub const fn RF_LO(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///RF LO powered on.
    #[inline(always)]
    pub const fn set_RF_LO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///RF PLL powered on.
    #[must_use]
    #[inline(always)]
    pub const fn RF_PLL(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///RF PLL powered on.
    #[inline(always)]
    pub const fn set_RF_PLL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///High-precision ADC powered on.
    #[must_use]
    #[inline(always)]
    pub const fn HPADC(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///High-precision ADC powered on.
    #[inline(always)]
    pub const fn set_HPADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///Low-power ADC powered on.
    #[must_use]
    #[inline(always)]
    pub const fn LPADC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///Low-power ADC powered on.
    #[inline(always)]
    pub const fn set_LPADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for ANA_PW_STAT {
    #[inline(always)]
    fn default() -> ANA_PW_STAT {
        ANA_PW_STAT(0)
    }
}
impl core::fmt::Debug for ANA_PW_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_PW_STAT")
            .field("RCOSC", &self.RCOSC())
            .field("XOSC", &self.XOSC())
            .field("SYSPLL", &self.SYSPLL())
            .field("RF_LNA", &self.RF_LNA())
            .field("RF_MIX", &self.RF_MIX())
            .field("RF_IF", &self.RF_IF())
            .field("RF_ADC", &self.RF_ADC())
            .field("RF_LO", &self.RF_LO())
            .field("RF_PLL", &self.RF_PLL())
            .field("HPADC", &self.HPADC())
            .field("LPADC", &self.LPADC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANA_PW_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANA_PW_STAT {{ RCOSC: {=bool:?}, XOSC: {=bool:?}, SYSPLL: {=bool:?}, RF_LNA: {=bool:?}, RF_MIX: {=bool:?}, RF_IF: {=bool:?}, RF_ADC: {=bool:?}, RF_LO: {=bool:?}, RF_PLL: {=bool:?}, HPADC: {=bool:?}, LPADC: {=bool:?} }}",
            self.RCOSC(),
            self.XOSC(),
            self.SYSPLL(),
            self.RF_LNA(),
            self.RF_MIX(),
            self.RF_IF(),
            self.RF_ADC(),
            self.RF_LO(),
            self.RF_PLL(),
            self.HPADC(),
            self.LPADC()
        )
    }
}
///Root clock source select and RTC status.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CKSEL_ROOT(pub u32);
impl CKSEL_ROOT {
    ///Enable RF PLL1 as clock source.
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_RF_PLL1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Enable RF PLL1 as clock source.
    #[inline(always)]
    pub const fn set_ENABLE_RF_PLL1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///Enable clock source selection.
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_SOURCE_SEL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Enable clock source selection.
    #[inline(always)]
    pub const fn set_ENABLE_SOURCE_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///RTC clock source status (read).
    #[must_use]
    #[inline(always)]
    pub const fn STATUS_RTC(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    ///RTC clock source status (read).
    #[inline(always)]
    pub const fn set_STATUS_RTC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for CKSEL_ROOT {
    #[inline(always)]
    fn default() -> CKSEL_ROOT {
        CKSEL_ROOT(0)
    }
}
impl core::fmt::Debug for CKSEL_ROOT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKSEL_ROOT")
            .field("ENABLE_RF_PLL1", &self.ENABLE_RF_PLL1())
            .field("ENABLE_SOURCE_SEL", &self.ENABLE_SOURCE_SEL())
            .field("STATUS_RTC", &self.STATUS_RTC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CKSEL_ROOT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CKSEL_ROOT {{ ENABLE_RF_PLL1: {=bool:?}, ENABLE_SOURCE_SEL: {=bool:?}, STATUS_RTC: {=u8:?} }}",
            self.ENABLE_RF_PLL1(),
            self.ENABLE_SOURCE_SEL(),
            self.STATUS_RTC()
        )
    }
}
///TOPREG clock-ready interrupt clear 0 (write 1 to clear).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CRG_INT_CLR0(pub u32);
impl CRG_INT_CLR0 {
    ///UART0 PCLK ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_PCLK_UART0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///UART0 PCLK ready.
    #[inline(always)]
    pub const fn set_CK_PCLK_UART0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///UART0 clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_UART0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///UART0 clock ready.
    #[inline(always)]
    pub const fn set_CK_UART0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Host bridge clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_BRG_HOST(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Host bridge clock ready.
    #[inline(always)]
    pub const fn set_CK_BRG_HOST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///Host interface PCLK ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_PCLK_HOSTIFC(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///Host interface PCLK ready.
    #[inline(always)]
    pub const fn set_CK_PCLK_HOSTIFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Host interface sequencer clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_HOSTIFC_SEQ(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Host interface sequencer clock ready.
    #[inline(always)]
    pub const fn set_CK_HOSTIFC_SEQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///I2C slave clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_I2CS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///I2C slave clock ready.
    #[inline(always)]
    pub const fn set_CK_I2CS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///RTC origin clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_RTC_ORG(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///RTC origin clock ready.
    #[inline(always)]
    pub const fn set_CK_RTC_ORG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///SYSIOP RTC clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SYSIOP_RTC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///SYSIOP RTC clock ready.
    #[inline(always)]
    pub const fn set_CK_SYSIOP_RTC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///SCU bridge clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_BRG_SCU(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///SCU bridge clock ready.
    #[inline(always)]
    pub const fn set_CK_BRG_SCU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///SCU clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SCU(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///SCU clock ready.
    #[inline(always)]
    pub const fn set_CK_SCU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///SCU SPI clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SCU_SPI(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///SCU SPI clock ready.
    #[inline(always)]
    pub const fn set_CK_SCU_SPI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///SCU I2C0 clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SCU_I2C0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///SCU I2C0 clock ready.
    #[inline(always)]
    pub const fn set_CK_SCU_I2C0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    ///SCU I2C1 clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SCU_I2C1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///SCU I2C1 clock ready.
    #[inline(always)]
    pub const fn set_CK_SCU_I2C1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///SCU sequencer clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SCU_SEQ(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///SCU sequencer clock ready.
    #[inline(always)]
    pub const fn set_CK_SCU_SEQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    ///SCU SC clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SCU_SC(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    ///SCU SC clock ready.
    #[inline(always)]
    pub const fn set_CK_SCU_SC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    ///32 kHz clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_32K(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    ///32 kHz clock ready.
    #[inline(always)]
    pub const fn set_CK_32K(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    ///U32K high clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_U32KH(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///U32K high clock ready.
    #[inline(always)]
    pub const fn set_CK_U32KH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///U32K low clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_U32KL(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///U32K low clock ready.
    #[inline(always)]
    pub const fn set_CK_U32KL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    ///Temperature ADC clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_TADC(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    ///Temperature ADC clock ready.
    #[inline(always)]
    pub const fn set_CK_TADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    ///RTC PCLK ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_RTC_PCLK(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    ///RTC PCLK ready.
    #[inline(always)]
    pub const fn set_CK_RTC_PCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    ///PMU RTC PCLK ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_PMU_RTC_PCLK(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    ///PMU RTC PCLK ready.
    #[inline(always)]
    pub const fn set_CK_PMU_RTC_PCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    ///APP domain clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_APP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    ///APP domain clock ready.
    #[inline(always)]
    pub const fn set_CK_APP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for CRG_INT_CLR0 {
    #[inline(always)]
    fn default() -> CRG_INT_CLR0 {
        CRG_INT_CLR0(0)
    }
}
impl core::fmt::Debug for CRG_INT_CLR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRG_INT_CLR0")
            .field("CK_PCLK_UART0", &self.CK_PCLK_UART0())
            .field("CK_UART0", &self.CK_UART0())
            .field("CK_BRG_HOST", &self.CK_BRG_HOST())
            .field("CK_PCLK_HOSTIFC", &self.CK_PCLK_HOSTIFC())
            .field("CK_HOSTIFC_SEQ", &self.CK_HOSTIFC_SEQ())
            .field("CK_I2CS", &self.CK_I2CS())
            .field("CK_RTC_ORG", &self.CK_RTC_ORG())
            .field("CK_SYSIOP_RTC", &self.CK_SYSIOP_RTC())
            .field("CK_BRG_SCU", &self.CK_BRG_SCU())
            .field("CK_SCU", &self.CK_SCU())
            .field("CK_SCU_SPI", &self.CK_SCU_SPI())
            .field("CK_SCU_I2C0", &self.CK_SCU_I2C0())
            .field("CK_SCU_I2C1", &self.CK_SCU_I2C1())
            .field("CK_SCU_SEQ", &self.CK_SCU_SEQ())
            .field("CK_SCU_SC", &self.CK_SCU_SC())
            .field("CK_32K", &self.CK_32K())
            .field("CK_U32KH", &self.CK_U32KH())
            .field("CK_U32KL", &self.CK_U32KL())
            .field("CK_TADC", &self.CK_TADC())
            .field("CK_RTC_PCLK", &self.CK_RTC_PCLK())
            .field("CK_PMU_RTC_PCLK", &self.CK_PMU_RTC_PCLK())
            .field("CK_APP", &self.CK_APP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CRG_INT_CLR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CRG_INT_CLR0 {{ CK_PCLK_UART0: {=bool:?}, CK_UART0: {=bool:?}, CK_BRG_HOST: {=bool:?}, CK_PCLK_HOSTIFC: {=bool:?}, CK_HOSTIFC_SEQ: {=bool:?}, CK_I2CS: {=bool:?}, CK_RTC_ORG: {=bool:?}, CK_SYSIOP_RTC: {=bool:?}, CK_BRG_SCU: {=bool:?}, CK_SCU: {=bool:?}, CK_SCU_SPI: {=bool:?}, CK_SCU_I2C0: {=bool:?}, CK_SCU_I2C1: {=bool:?}, CK_SCU_SEQ: {=bool:?}, CK_SCU_SC: {=bool:?}, CK_32K: {=bool:?}, CK_U32KH: {=bool:?}, CK_U32KL: {=bool:?}, CK_TADC: {=bool:?}, CK_RTC_PCLK: {=bool:?}, CK_PMU_RTC_PCLK: {=bool:?}, CK_APP: {=bool:?} }}",
            self.CK_PCLK_UART0(),
            self.CK_UART0(),
            self.CK_BRG_HOST(),
            self.CK_PCLK_HOSTIFC(),
            self.CK_HOSTIFC_SEQ(),
            self.CK_I2CS(),
            self.CK_RTC_ORG(),
            self.CK_SYSIOP_RTC(),
            self.CK_BRG_SCU(),
            self.CK_SCU(),
            self.CK_SCU_SPI(),
            self.CK_SCU_I2C0(),
            self.CK_SCU_I2C1(),
            self.CK_SCU_SEQ(),
            self.CK_SCU_SC(),
            self.CK_32K(),
            self.CK_U32KH(),
            self.CK_U32KL(),
            self.CK_TADC(),
            self.CK_RTC_PCLK(),
            self.CK_PMU_RTC_PCLK(),
            self.CK_APP()
        )
    }
}
///TOPREG clock-ready interrupt clear 1 (write 1 to clear).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CRG_INT_CLR1(pub u32);
impl CRG_INT_CLR1 {
    ///CPU bus clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_CPU_BUS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///CPU bus clock ready.
    #[inline(always)]
    pub const fn set_CK_CPU_BUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///CPU bus clock timeout.
    #[must_use]
    #[inline(always)]
    pub const fn CK_CPU_BUS_TO(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///CPU bus clock timeout.
    #[inline(always)]
    pub const fn set_CK_CPU_BUS_TO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///RF PLL1 clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_RFPLL1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///RF PLL1 clock ready.
    #[inline(always)]
    pub const fn set_CK_RFPLL1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///RF PLL1 clock timeout.
    #[must_use]
    #[inline(always)]
    pub const fn CK_RFPLL1_TO(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///RF PLL1 clock timeout.
    #[inline(always)]
    pub const fn set_CK_RFPLL1_TO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///RTC pre-divider clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_RTC_PRE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///RTC pre-divider clock ready.
    #[inline(always)]
    pub const fn set_CK_RTC_PRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///RTC pre-divider clock timeout.
    #[must_use]
    #[inline(always)]
    pub const fn CK_RTC_PRE_TO(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///RTC pre-divider clock timeout.
    #[inline(always)]
    pub const fn set_CK_RTC_PRE_TO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///APP pre-divider clock ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_APP_PRE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///APP pre-divider clock ready.
    #[inline(always)]
    pub const fn set_CK_APP_PRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///APP pre-divider clock timeout.
    #[must_use]
    #[inline(always)]
    pub const fn CK_APP_PRE_TO(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///APP pre-divider clock timeout.
    #[inline(always)]
    pub const fn set_CK_APP_PRE_TO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///SP clock select ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SEL_SP(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///SP clock select ready.
    #[inline(always)]
    pub const fn set_CK_SEL_SP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///SP clock select timeout.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SEL_SP_TO(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///SP clock select timeout.
    #[inline(always)]
    pub const fn set_CK_SEL_SP_TO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///RO RTC clock select ready.
    #[must_use]
    #[inline(always)]
    pub const fn CK_SEL_RO_RTC(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///RO RTC clock select ready.
    #[inline(always)]
    pub const fn set_CK_SEL_RO_RTC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///Frequency fix error.
    #[must_use]
    #[inline(always)]
    pub const fn CRG_FREQFIX_ERR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///Frequency fix error.
    #[inline(always)]
    pub const fn set_CRG_FREQFIX_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for CRG_INT_CLR1 {
    #[inline(always)]
    fn default() -> CRG_INT_CLR1 {
        CRG_INT_CLR1(0)
    }
}
impl core::fmt::Debug for CRG_INT_CLR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRG_INT_CLR1")
            .field("CK_CPU_BUS", &self.CK_CPU_BUS())
            .field("CK_CPU_BUS_TO", &self.CK_CPU_BUS_TO())
            .field("CK_RFPLL1", &self.CK_RFPLL1())
            .field("CK_RFPLL1_TO", &self.CK_RFPLL1_TO())
            .field("CK_RTC_PRE", &self.CK_RTC_PRE())
            .field("CK_RTC_PRE_TO", &self.CK_RTC_PRE_TO())
            .field("CK_APP_PRE", &self.CK_APP_PRE())
            .field("CK_APP_PRE_TO", &self.CK_APP_PRE_TO())
            .field("CK_SEL_SP", &self.CK_SEL_SP())
            .field("CK_SEL_SP_TO", &self.CK_SEL_SP_TO())
            .field("CK_SEL_RO_RTC", &self.CK_SEL_RO_RTC())
            .field("CRG_FREQFIX_ERR", &self.CRG_FREQFIX_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CRG_INT_CLR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CRG_INT_CLR1 {{ CK_CPU_BUS: {=bool:?}, CK_CPU_BUS_TO: {=bool:?}, CK_RFPLL1: {=bool:?}, CK_RFPLL1_TO: {=bool:?}, CK_RTC_PRE: {=bool:?}, CK_RTC_PRE_TO: {=bool:?}, CK_APP_PRE: {=bool:?}, CK_APP_PRE_TO: {=bool:?}, CK_SEL_SP: {=bool:?}, CK_SEL_SP_TO: {=bool:?}, CK_SEL_RO_RTC: {=bool:?}, CRG_FREQFIX_ERR: {=bool:?} }}",
            self.CK_CPU_BUS(),
            self.CK_CPU_BUS_TO(),
            self.CK_RFPLL1(),
            self.CK_RFPLL1_TO(),
            self.CK_RTC_PRE(),
            self.CK_RTC_PRE_TO(),
            self.CK_APP_PRE(),
            self.CK_APP_PRE_TO(),
            self.CK_SEL_SP(),
            self.CK_SEL_SP_TO(),
            self.CK_SEL_RO_RTC(),
            self.CRG_FREQFIX_ERR()
        )
    }
}
///APP-domain IO-cell mode-mux register.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCAPP_IOMD(pub u32);
impl IOCAPP_IOMD {
    ///Mode select for UART2 TXD/RXD pins (UART2 = Func1).
    #[must_use]
    #[inline(always)]
    pub const fn UART2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    ///Mode select for UART2 TXD/RXD pins (UART2 = Func1).
    #[inline(always)]
    pub const fn set_UART2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
}
impl Default for IOCAPP_IOMD {
    #[inline(always)]
    fn default() -> IOCAPP_IOMD {
        IOCAPP_IOMD(0)
    }
}
impl core::fmt::Debug for IOCAPP_IOMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCAPP_IOMD")
            .field("UART2", &self.UART2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCAPP_IOMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IOCAPP_IOMD {{ UART2: {=u8:?} }}", self.UART2())
    }
}
///SYSIOP IO-cell mode-mux register 0.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCSYS_IOMD0(pub u32);
impl IOCSYS_IOMD0 {
    ///Mode select for SPI0_CS_X / SPI0_SCK (UART1 = Func1).
    #[must_use]
    #[inline(always)]
    pub const fn SPI0A(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    ///Mode select for SPI0_CS_X / SPI0_SCK (UART1 = Func1).
    #[inline(always)]
    pub const fn set_SPI0A(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for IOCSYS_IOMD0 {
    #[inline(always)]
    fn default() -> IOCSYS_IOMD0 {
        IOCSYS_IOMD0(0)
    }
}
impl core::fmt::Debug for IOCSYS_IOMD0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCSYS_IOMD0")
            .field("SPI0A", &self.SPI0A())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCSYS_IOMD0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IOCSYS_IOMD0 {{ SPI0A: {=u8:?} }}", self.SPI0A())
    }
}
///IOCELL control for SPI0_CS_X / UART1-TXD.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_SPI0_CS_X(pub u32);
impl IO_SPI0_CS_X {
    ///Input enable: 0=disabled, 1=enabled.
    #[must_use]
    #[inline(always)]
    pub const fn ENZI(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Input enable: 0=disabled, 1=enabled.
    #[inline(always)]
    pub const fn set_ENZI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Pullup: 0=pullup enabled, 1=normal (off).
    #[must_use]
    #[inline(always)]
    pub const fn PUN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Pullup: 0=pullup enabled, 1=normal (off).
    #[inline(always)]
    pub const fn set_PUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Pulldown: 0=pulldown enabled, 1=normal (off).
    #[must_use]
    #[inline(always)]
    pub const fn PDN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Pulldown: 0=pulldown enabled, 1=normal (off).
    #[inline(always)]
    pub const fn set_PDN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz).
    #[must_use]
    #[inline(always)]
    pub const fn LOWEMI(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz).
    #[inline(always)]
    pub const fn set_LOWEMI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for IO_SPI0_CS_X {
    #[inline(always)]
    fn default() -> IO_SPI0_CS_X {
        IO_SPI0_CS_X(0)
    }
}
impl core::fmt::Debug for IO_SPI0_CS_X {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_SPI0_CS_X")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_SPI0_CS_X {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_SPI0_CS_X {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for SPI0_SCK / UART1-RXD.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_SPI0_SCK(pub u32);
impl IO_SPI0_SCK {
    ///Input enable: 0=disabled, 1=enabled.
    #[must_use]
    #[inline(always)]
    pub const fn ENZI(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Input enable: 0=disabled, 1=enabled.
    #[inline(always)]
    pub const fn set_ENZI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Pullup: 0=pullup enabled, 1=normal (off).
    #[must_use]
    #[inline(always)]
    pub const fn PUN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Pullup: 0=pullup enabled, 1=normal (off).
    #[inline(always)]
    pub const fn set_PUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Pulldown: 0=pulldown enabled, 1=normal (off).
    #[must_use]
    #[inline(always)]
    pub const fn PDN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Pulldown: 0=pulldown enabled, 1=normal (off).
    #[inline(always)]
    pub const fn set_PDN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz).
    #[must_use]
    #[inline(always)]
    pub const fn LOWEMI(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz).
    #[inline(always)]
    pub const fn set_LOWEMI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for IO_SPI0_SCK {
    #[inline(always)]
    fn default() -> IO_SPI0_SCK {
        IO_SPI0_SCK(0)
    }
}
impl core::fmt::Debug for IO_SPI0_SCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_SPI0_SCK")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_SPI0_SCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_SPI0_SCK {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for UART2 RXD pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_UART2_RXD(pub u32);
impl IO_UART2_RXD {
    ///Input enable: 0=disabled, 1=enabled.
    #[must_use]
    #[inline(always)]
    pub const fn ENZI(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Input enable: 0=disabled, 1=enabled.
    #[inline(always)]
    pub const fn set_ENZI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Pullup: 0=pullup enabled, 1=normal (off).
    #[must_use]
    #[inline(always)]
    pub const fn PUN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Pullup: 0=pullup enabled, 1=normal (off).
    #[inline(always)]
    pub const fn set_PUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Pulldown: 0=pulldown enabled, 1=normal (off).
    #[must_use]
    #[inline(always)]
    pub const fn PDN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Pulldown: 0=pulldown enabled, 1=normal (off).
    #[inline(always)]
    pub const fn set_PDN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz).
    #[must_use]
    #[inline(always)]
    pub const fn LOWEMI(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz).
    #[inline(always)]
    pub const fn set_LOWEMI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for IO_UART2_RXD {
    #[inline(always)]
    fn default() -> IO_UART2_RXD {
        IO_UART2_RXD(0)
    }
}
impl core::fmt::Debug for IO_UART2_RXD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_UART2_RXD")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_UART2_RXD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_UART2_RXD {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for UART2 TXD pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_UART2_TXD(pub u32);
impl IO_UART2_TXD {
    ///Input enable: 0=disabled, 1=enabled.
    #[must_use]
    #[inline(always)]
    pub const fn ENZI(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Input enable: 0=disabled, 1=enabled.
    #[inline(always)]
    pub const fn set_ENZI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Pullup: 0=pullup enabled, 1=normal (off).
    #[must_use]
    #[inline(always)]
    pub const fn PUN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Pullup: 0=pullup enabled, 1=normal (off).
    #[inline(always)]
    pub const fn set_PUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Pulldown: 0=pulldown enabled, 1=normal (off).
    #[must_use]
    #[inline(always)]
    pub const fn PDN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Pulldown: 0=pulldown enabled, 1=normal (off).
    #[inline(always)]
    pub const fn set_PDN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz).
    #[must_use]
    #[inline(always)]
    pub const fn LOWEMI(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Output current: 0=4mA(max 64MHz), 1=2mA(max 32MHz).
    #[inline(always)]
    pub const fn set_LOWEMI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for IO_UART2_TXD {
    #[inline(always)]
    fn default() -> IO_UART2_TXD {
        IO_UART2_TXD(0)
    }
}
impl core::fmt::Debug for IO_UART2_TXD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_UART2_TXD")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_UART2_TXD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_UART2_TXD {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///PMU power-supply control request (write-only kick register).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_PW_CTL(pub u32);
impl PMU_PW_CTL {
    ///Write 1 to request a PMU power-state transition.
    #[must_use]
    #[inline(always)]
    pub const fn POWER_CTRL_ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Write 1 to request a PMU power-state transition.
    #[inline(always)]
    pub const fn set_POWER_CTRL_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PMU_PW_CTL {
    #[inline(always)]
    fn default() -> PMU_PW_CTL {
        PMU_PW_CTL(0)
    }
}
impl core::fmt::Debug for PMU_PW_CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_PW_CTL")
            .field("POWER_CTRL_ON", &self.POWER_CTRL_ON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_PW_CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_PW_CTL {{ POWER_CTRL_ON: {=bool:?} }}",
            self.POWER_CTRL_ON()
        )
    }
}
///Power-domain control (1 = powered on).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CTL(pub u32);
impl PWD_CTL {
    ///Sensor Control Unit domain.
    #[must_use]
    #[inline(always)]
    pub const fn SCU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sensor Control Unit domain.
    #[inline(always)]
    pub const fn set_SCU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Core domain.
    #[must_use]
    #[inline(always)]
    pub const fn CORE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Core domain.
    #[inline(always)]
    pub const fn set_CORE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///SYSIOP domain.
    #[must_use]
    #[inline(always)]
    pub const fn SYSIOP(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///SYSIOP domain.
    #[inline(always)]
    pub const fn set_SYSIOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///SYSIOP sub-domain.
    #[must_use]
    #[inline(always)]
    pub const fn SYSIOP_SUB(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///SYSIOP sub-domain.
    #[inline(always)]
    pub const fn set_SYSIOP_SUB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Application domain.
    #[must_use]
    #[inline(always)]
    pub const fn APP(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Application domain.
    #[inline(always)]
    pub const fn set_APP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Application DSP domain.
    #[must_use]
    #[inline(always)]
    pub const fn APP_DSP(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Application DSP domain.
    #[inline(always)]
    pub const fn set_APP_DSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///Application sub-domain.
    #[must_use]
    #[inline(always)]
    pub const fn APP_SUB(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///Application sub-domain.
    #[inline(always)]
    pub const fn set_APP_SUB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///GNSS ITP domain.
    #[must_use]
    #[inline(always)]
    pub const fn GNSS_ITP(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///GNSS ITP domain.
    #[inline(always)]
    pub const fn set_GNSS_ITP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///GNSS domain.
    #[must_use]
    #[inline(always)]
    pub const fn GNSS(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///GNSS domain.
    #[inline(always)]
    pub const fn set_GNSS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    ///Application audio domain.
    #[must_use]
    #[inline(always)]
    pub const fn APP_AUDIO(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    ///Application audio domain.
    #[inline(always)]
    pub const fn set_APP_AUDIO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for PWD_CTL {
    #[inline(always)]
    fn default() -> PWD_CTL {
        PWD_CTL(0)
    }
}
impl core::fmt::Debug for PWD_CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CTL")
            .field("SCU", &self.SCU())
            .field("CORE", &self.CORE())
            .field("SYSIOP", &self.SYSIOP())
            .field("SYSIOP_SUB", &self.SYSIOP_SUB())
            .field("APP", &self.APP())
            .field("APP_DSP", &self.APP_DSP())
            .field("APP_SUB", &self.APP_SUB())
            .field("GNSS_ITP", &self.GNSS_ITP())
            .field("GNSS", &self.GNSS())
            .field("APP_AUDIO", &self.APP_AUDIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CTL {{ SCU: {=bool:?}, CORE: {=bool:?}, SYSIOP: {=bool:?}, SYSIOP_SUB: {=bool:?}, APP: {=bool:?}, APP_DSP: {=bool:?}, APP_SUB: {=bool:?}, GNSS_ITP: {=bool:?}, GNSS: {=bool:?}, APP_AUDIO: {=bool:?} }}",
            self.SCU(),
            self.CORE(),
            self.SYSIOP(),
            self.SYSIOP_SUB(),
            self.APP(),
            self.APP_DSP(),
            self.APP_SUB(),
            self.GNSS_ITP(),
            self.GNSS(),
            self.APP_AUDIO()
        )
    }
}
///Power-domain status (read-only mirror of PWD_CTL).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_STAT(pub u32);
impl PWD_STAT {
    ///Sensor Control Unit domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn SCU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sensor Control Unit domain powered on.
    #[inline(always)]
    pub const fn set_SCU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Core domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn CORE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Core domain powered on.
    #[inline(always)]
    pub const fn set_CORE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///SYSIOP domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn SYSIOP(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///SYSIOP domain powered on.
    #[inline(always)]
    pub const fn set_SYSIOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///SYSIOP sub-domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn SYSIOP_SUB(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///SYSIOP sub-domain powered on.
    #[inline(always)]
    pub const fn set_SYSIOP_SUB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Application domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn APP(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Application domain powered on.
    #[inline(always)]
    pub const fn set_APP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Application DSP domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn APP_DSP(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Application DSP domain powered on.
    #[inline(always)]
    pub const fn set_APP_DSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///Application sub-domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn APP_SUB(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///Application sub-domain powered on.
    #[inline(always)]
    pub const fn set_APP_SUB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///GNSS ITP domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn GNSS_ITP(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///GNSS ITP domain powered on.
    #[inline(always)]
    pub const fn set_GNSS_ITP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///GNSS domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn GNSS(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///GNSS domain powered on.
    #[inline(always)]
    pub const fn set_GNSS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    ///Application audio domain powered on.
    #[must_use]
    #[inline(always)]
    pub const fn APP_AUDIO(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    ///Application audio domain powered on.
    #[inline(always)]
    pub const fn set_APP_AUDIO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for PWD_STAT {
    #[inline(always)]
    fn default() -> PWD_STAT {
        PWD_STAT(0)
    }
}
impl core::fmt::Debug for PWD_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_STAT")
            .field("SCU", &self.SCU())
            .field("CORE", &self.CORE())
            .field("SYSIOP", &self.SYSIOP())
            .field("SYSIOP_SUB", &self.SYSIOP_SUB())
            .field("APP", &self.APP())
            .field("APP_DSP", &self.APP_DSP())
            .field("APP_SUB", &self.APP_SUB())
            .field("GNSS_ITP", &self.GNSS_ITP())
            .field("GNSS", &self.GNSS())
            .field("APP_AUDIO", &self.APP_AUDIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_STAT {{ SCU: {=bool:?}, CORE: {=bool:?}, SYSIOP: {=bool:?}, SYSIOP_SUB: {=bool:?}, APP: {=bool:?}, APP_DSP: {=bool:?}, APP_SUB: {=bool:?}, GNSS_ITP: {=bool:?}, GNSS: {=bool:?}, APP_AUDIO: {=bool:?} }}",
            self.SCU(),
            self.CORE(),
            self.SYSIOP(),
            self.SYSIOP_SUB(),
            self.APP(),
            self.APP_DSP(),
            self.APP_SUB(),
            self.GNSS_ITP(),
            self.GNSS(),
            self.APP_AUDIO()
        )
    }
}
///RC oscillator control 2.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCOSC_CTRL2(pub u32);
impl RCOSC_CTRL2 {
    ///Disable RCOSC logic clock output (1 = disabled).
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_LOGICLK(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///Disable RCOSC logic clock output (1 = disabled).
    #[inline(always)]
    pub const fn set_DISABLE_LOGICLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    ///Disable RCOSC sensor clock output (1 = disabled).
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_SENSCLK(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    ///Disable RCOSC sensor clock output (1 = disabled).
    #[inline(always)]
    pub const fn set_DISABLE_SENSCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for RCOSC_CTRL2 {
    #[inline(always)]
    fn default() -> RCOSC_CTRL2 {
        RCOSC_CTRL2(0)
    }
}
impl core::fmt::Debug for RCOSC_CTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCOSC_CTRL2")
            .field("DISABLE_LOGICLK", &self.DISABLE_LOGICLK())
            .field("DISABLE_SENSCLK", &self.DISABLE_SENSCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCOSC_CTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCOSC_CTRL2 {{ DISABLE_LOGICLK: {=bool:?}, DISABLE_SENSCLK: {=bool:?} }}",
            self.DISABLE_LOGICLK(),
            self.DISABLE_SENSCLK()
        )
    }
}
///SCU peripheral clock enables.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCU_CKEN(pub u32);
impl SCU_CKEN {
    ///SCU core clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_SCU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///SCU core clock enable.
    #[inline(always)]
    pub const fn set_SCU_SCU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///SCU I2C0 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_I2C0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///SCU I2C0 clock enable.
    #[inline(always)]
    pub const fn set_SCU_I2C0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///SCU I2C1 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_I2C1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///SCU I2C1 clock enable.
    #[inline(always)]
    pub const fn set_SCU_I2C1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///SCU SPI clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_SPI(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///SCU SPI clock enable.
    #[inline(always)]
    pub const fn set_SCU_SPI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///SCU sequencer clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_SEQ(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///SCU sequencer clock enable.
    #[inline(always)]
    pub const fn set_SCU_SEQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///SCU 32 kHz clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_32K(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///SCU 32 kHz clock enable.
    #[inline(always)]
    pub const fn set_SCU_32K(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///SCU U32K low clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_U32KL(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///SCU U32K low clock enable.
    #[inline(always)]
    pub const fn set_SCU_U32KL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///SCU U32K high clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_U32KH(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///SCU U32K high clock enable.
    #[inline(always)]
    pub const fn set_SCU_U32KH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///SCU SC clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn SCU_SC(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///SCU SC clock enable.
    #[inline(always)]
    pub const fn set_SCU_SC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for SCU_CKEN {
    #[inline(always)]
    fn default() -> SCU_CKEN {
        SCU_CKEN(0)
    }
}
impl core::fmt::Debug for SCU_CKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCU_CKEN")
            .field("SCU_SCU", &self.SCU_SCU())
            .field("SCU_I2C0", &self.SCU_I2C0())
            .field("SCU_I2C1", &self.SCU_I2C1())
            .field("SCU_SPI", &self.SCU_SPI())
            .field("SCU_SEQ", &self.SCU_SEQ())
            .field("SCU_32K", &self.SCU_32K())
            .field("SCU_U32KL", &self.SCU_U32KL())
            .field("SCU_U32KH", &self.SCU_U32KH())
            .field("SCU_SC", &self.SCU_SC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCU_CKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SCU_CKEN {{ SCU_SCU: {=bool:?}, SCU_I2C0: {=bool:?}, SCU_I2C1: {=bool:?}, SCU_SPI: {=bool:?}, SCU_SEQ: {=bool:?}, SCU_32K: {=bool:?}, SCU_U32KL: {=bool:?}, SCU_U32KH: {=bool:?}, SCU_SC: {=bool:?} }}",
            self.SCU_SCU(),
            self.SCU_I2C0(),
            self.SCU_I2C1(),
            self.SCU_SPI(),
            self.SCU_SEQ(),
            self.SCU_32K(),
            self.SCU_U32KL(),
            self.SCU_U32KH(),
            self.SCU_SC()
        )
    }
}
///Bus peripheral software reset (0 = held in reset, 1 = released).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWRESET_BUS(pub u32);
impl SWRESET_BUS {
    ///SPI master reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SPIM(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///SPI master reset.
    #[inline(always)]
    pub const fn set_XRST_SPIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Serial flash controller reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SFC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Serial flash controller reset.
    #[inline(always)]
    pub const fn set_XRST_SFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///SAKE reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SAKE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///SAKE reset.
    #[inline(always)]
    pub const fn set_XRST_SAKE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///UART1 reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_UART1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///UART1 reset.
    #[inline(always)]
    pub const fn set_XRST_UART1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///KAKI reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_KAKI(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///KAKI reset.
    #[inline(always)]
    pub const fn set_XRST_KAKI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///Host interface reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_HOSTIFC(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Host interface reset.
    #[inline(always)]
    pub const fn set_XRST_HOSTIFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Host interface ISOP reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_HOSTIFC_ISOP(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Host interface ISOP reset.
    #[inline(always)]
    pub const fn set_XRST_HOSTIFC_ISOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///UART0 reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_UART0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///UART0 reset.
    #[inline(always)]
    pub const fn set_XRST_UART0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///I2C master reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_I2CM(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///I2C master reset.
    #[inline(always)]
    pub const fn set_XRST_I2CM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    ///PMU I2C master reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_PMU_I2CM(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///PMU I2C master reset.
    #[inline(always)]
    pub const fn set_XRST_PMU_I2CM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for SWRESET_BUS {
    #[inline(always)]
    fn default() -> SWRESET_BUS {
        SWRESET_BUS(0)
    }
}
impl core::fmt::Debug for SWRESET_BUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRESET_BUS")
            .field("XRST_SPIM", &self.XRST_SPIM())
            .field("XRST_SFC", &self.XRST_SFC())
            .field("XRST_SAKE", &self.XRST_SAKE())
            .field("XRST_UART1", &self.XRST_UART1())
            .field("XRST_KAKI", &self.XRST_KAKI())
            .field("XRST_HOSTIFC", &self.XRST_HOSTIFC())
            .field("XRST_HOSTIFC_ISOP", &self.XRST_HOSTIFC_ISOP())
            .field("XRST_UART0", &self.XRST_UART0())
            .field("XRST_I2CM", &self.XRST_I2CM())
            .field("XRST_PMU_I2CM", &self.XRST_PMU_I2CM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWRESET_BUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SWRESET_BUS {{ XRST_SPIM: {=bool:?}, XRST_SFC: {=bool:?}, XRST_SAKE: {=bool:?}, XRST_UART1: {=bool:?}, XRST_KAKI: {=bool:?}, XRST_HOSTIFC: {=bool:?}, XRST_HOSTIFC_ISOP: {=bool:?}, XRST_UART0: {=bool:?}, XRST_I2CM: {=bool:?}, XRST_PMU_I2CM: {=bool:?} }}",
            self.XRST_SPIM(),
            self.XRST_SFC(),
            self.XRST_SAKE(),
            self.XRST_UART1(),
            self.XRST_KAKI(),
            self.XRST_HOSTIFC(),
            self.XRST_HOSTIFC_ISOP(),
            self.XRST_UART0(),
            self.XRST_I2CM(),
            self.XRST_PMU_I2CM()
        )
    }
}
///SCU peripheral software reset (0 = held in reset, 1 = released).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWRESET_SCU(pub u32);
impl SWRESET_SCU {
    ///SCU high-precision ADC reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SCU_HPADC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///SCU high-precision ADC reset.
    #[inline(always)]
    pub const fn set_XRST_SCU_HPADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///SCU low-power ADC reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SCU_LPADC(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///SCU low-power ADC reset.
    #[inline(always)]
    pub const fn set_XRST_SCU_LPADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///SCU I2C2 reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SCU_I2C2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///SCU I2C2 reset.
    #[inline(always)]
    pub const fn set_XRST_SCU_I2C2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///SCU I2C1 reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SCU_I2C1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///SCU I2C1 reset.
    #[inline(always)]
    pub const fn set_XRST_SCU_I2C1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///SCU ISOP reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SCU_ISOP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///SCU ISOP reset.
    #[inline(always)]
    pub const fn set_XRST_SCU_ISOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///SCU SPI reset.
    #[must_use]
    #[inline(always)]
    pub const fn XRST_SCU_SPI(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///SCU SPI reset.
    #[inline(always)]
    pub const fn set_XRST_SCU_SPI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for SWRESET_SCU {
    #[inline(always)]
    fn default() -> SWRESET_SCU {
        SWRESET_SCU(0)
    }
}
impl core::fmt::Debug for SWRESET_SCU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRESET_SCU")
            .field("XRST_SCU_HPADC", &self.XRST_SCU_HPADC())
            .field("XRST_SCU_LPADC", &self.XRST_SCU_LPADC())
            .field("XRST_SCU_I2C2", &self.XRST_SCU_I2C2())
            .field("XRST_SCU_I2C1", &self.XRST_SCU_I2C1())
            .field("XRST_SCU_ISOP", &self.XRST_SCU_ISOP())
            .field("XRST_SCU_SPI", &self.XRST_SCU_SPI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWRESET_SCU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SWRESET_SCU {{ XRST_SCU_HPADC: {=bool:?}, XRST_SCU_LPADC: {=bool:?}, XRST_SCU_I2C2: {=bool:?}, XRST_SCU_I2C1: {=bool:?}, XRST_SCU_ISOP: {=bool:?}, XRST_SCU_SPI: {=bool:?} }}",
            self.XRST_SCU_HPADC(),
            self.XRST_SCU_LPADC(),
            self.XRST_SCU_I2C2(),
            self.XRST_SCU_I2C1(),
            self.XRST_SCU_ISOP(),
            self.XRST_SCU_SPI()
        )
    }
}
///SYSIOP peripheral clock enables.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSIOP_CKEN(pub u32);
impl SYSIOP_CKEN {
    ///UART0 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_UART0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///UART0 clock enable.
    #[inline(always)]
    pub const fn set_CKEN_UART0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///UART0 PCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_PCLK_UART0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///UART0 PCLK enable.
    #[inline(always)]
    pub const fn set_CKEN_PCLK_UART0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Host interface PCLK enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_PCLK_HOSTIFC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    ///Host interface PCLK enable.
    #[inline(always)]
    pub const fn set_CKEN_PCLK_HOSTIFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    ///I2C slave clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_I2CS(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///I2C slave clock enable.
    #[inline(always)]
    pub const fn set_CKEN_I2CS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    ///Host bridge clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_BRG_HOST(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    ///Host bridge clock enable.
    #[inline(always)]
    pub const fn set_CKEN_BRG_HOST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    ///AHB DMAC0 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_AHB_DMAC0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    ///AHB DMAC0 clock enable.
    #[inline(always)]
    pub const fn set_CKEN_AHB_DMAC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    ///AHB DMAC1 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_AHB_DMAC1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    ///AHB DMAC1 clock enable.
    #[inline(always)]
    pub const fn set_CKEN_AHB_DMAC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    ///AHB DMAC2 clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_AHB_DMAC2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    ///AHB DMAC2 clock enable.
    #[inline(always)]
    pub const fn set_CKEN_AHB_DMAC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    ///APB clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_APB(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///APB clock enable.
    #[inline(always)]
    pub const fn set_CKEN_APB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Frequency discriminator clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_FREQDIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    ///Frequency discriminator clock enable.
    #[inline(always)]
    pub const fn set_CKEN_FREQDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    ///RTC origin clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_RTC_ORG(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    ///RTC origin clock enable.
    #[inline(always)]
    pub const fn set_CKEN_RTC_ORG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    ///AP clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_AP_CLK(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    ///AP clock enable.
    #[inline(always)]
    pub const fn set_CKEN_AP_CLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    ///RCOSC output clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_RCOSC_OUT(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    ///RCOSC output clock enable.
    #[inline(always)]
    pub const fn set_CKEN_RCOSC_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    ///SYSIOP RTC clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_SYSIOP_RTC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    ///SYSIOP RTC clock enable.
    #[inline(always)]
    pub const fn set_CKEN_SYSIOP_RTC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    ///SCU bridge clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_BRG_SCU(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    ///SCU bridge clock enable.
    #[inline(always)]
    pub const fn set_CKEN_BRG_SCU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    ///Host interface sequencer clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_HOSTIFC_SEQ(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    ///Host interface sequencer clock enable.
    #[inline(always)]
    pub const fn set_CKEN_HOSTIFC_SEQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    ///Host I2C clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_HOSI2C(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Host I2C clock enable.
    #[inline(always)]
    pub const fn set_CKEN_HOSI2C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Host SPI clock enable.
    #[must_use]
    #[inline(always)]
    pub const fn CKEN_HOSSPI(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Host SPI clock enable.
    #[inline(always)]
    pub const fn set_CKEN_HOSSPI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for SYSIOP_CKEN {
    #[inline(always)]
    fn default() -> SYSIOP_CKEN {
        SYSIOP_CKEN(0)
    }
}
impl core::fmt::Debug for SYSIOP_CKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSIOP_CKEN")
            .field("CKEN_UART0", &self.CKEN_UART0())
            .field("CKEN_PCLK_UART0", &self.CKEN_PCLK_UART0())
            .field("CKEN_PCLK_HOSTIFC", &self.CKEN_PCLK_HOSTIFC())
            .field("CKEN_I2CS", &self.CKEN_I2CS())
            .field("CKEN_BRG_HOST", &self.CKEN_BRG_HOST())
            .field("CKEN_AHB_DMAC0", &self.CKEN_AHB_DMAC0())
            .field("CKEN_AHB_DMAC1", &self.CKEN_AHB_DMAC1())
            .field("CKEN_AHB_DMAC2", &self.CKEN_AHB_DMAC2())
            .field("CKEN_APB", &self.CKEN_APB())
            .field("CKEN_FREQDIS", &self.CKEN_FREQDIS())
            .field("CKEN_RTC_ORG", &self.CKEN_RTC_ORG())
            .field("CKEN_AP_CLK", &self.CKEN_AP_CLK())
            .field("CKEN_RCOSC_OUT", &self.CKEN_RCOSC_OUT())
            .field("CKEN_SYSIOP_RTC", &self.CKEN_SYSIOP_RTC())
            .field("CKEN_BRG_SCU", &self.CKEN_BRG_SCU())
            .field("CKEN_HOSTIFC_SEQ", &self.CKEN_HOSTIFC_SEQ())
            .field("CKEN_HOSI2C", &self.CKEN_HOSI2C())
            .field("CKEN_HOSSPI", &self.CKEN_HOSSPI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYSIOP_CKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYSIOP_CKEN {{ CKEN_UART0: {=bool:?}, CKEN_PCLK_UART0: {=bool:?}, CKEN_PCLK_HOSTIFC: {=bool:?}, CKEN_I2CS: {=bool:?}, CKEN_BRG_HOST: {=bool:?}, CKEN_AHB_DMAC0: {=bool:?}, CKEN_AHB_DMAC1: {=bool:?}, CKEN_AHB_DMAC2: {=bool:?}, CKEN_APB: {=bool:?}, CKEN_FREQDIS: {=bool:?}, CKEN_RTC_ORG: {=bool:?}, CKEN_AP_CLK: {=bool:?}, CKEN_RCOSC_OUT: {=bool:?}, CKEN_SYSIOP_RTC: {=bool:?}, CKEN_BRG_SCU: {=bool:?}, CKEN_HOSTIFC_SEQ: {=bool:?}, CKEN_HOSI2C: {=bool:?}, CKEN_HOSSPI: {=bool:?} }}",
            self.CKEN_UART0(),
            self.CKEN_PCLK_UART0(),
            self.CKEN_PCLK_HOSTIFC(),
            self.CKEN_I2CS(),
            self.CKEN_BRG_HOST(),
            self.CKEN_AHB_DMAC0(),
            self.CKEN_AHB_DMAC1(),
            self.CKEN_AHB_DMAC2(),
            self.CKEN_APB(),
            self.CKEN_FREQDIS(),
            self.CKEN_RTC_ORG(),
            self.CKEN_AP_CLK(),
            self.CKEN_RCOSC_OUT(),
            self.CKEN_SYSIOP_RTC(),
            self.CKEN_BRG_SCU(),
            self.CKEN_HOSTIFC_SEQ(),
            self.CKEN_HOSI2C(),
            self.CKEN_HOSSPI()
        )
    }
}
///System PLL control 1.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYS_PLL_CTRL1(pub u32);
impl SYS_PLL_CTRL1 {
    ///Enable GP ADC clock from PLL.
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_GPADCLK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    ///Enable GP ADC clock from PLL.
    #[inline(always)]
    pub const fn set_ENABLE_GPADCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    ///Enable DSP clock from PLL.
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_DSPCLK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    ///Enable DSP clock from PLL.
    #[inline(always)]
    pub const fn set_ENABLE_DSPCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for SYS_PLL_CTRL1 {
    #[inline(always)]
    fn default() -> SYS_PLL_CTRL1 {
        SYS_PLL_CTRL1(0)
    }
}
impl core::fmt::Debug for SYS_PLL_CTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_PLL_CTRL1")
            .field("ENABLE_GPADCLK", &self.ENABLE_GPADCLK())
            .field("ENABLE_DSPCLK", &self.ENABLE_DSPCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYS_PLL_CTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYS_PLL_CTRL1 {{ ENABLE_GPADCLK: {=bool:?}, ENABLE_DSPCLK: {=bool:?} }}",
            self.ENABLE_GPADCLK(),
            self.ENABLE_DSPCLK()
        )
    }
}
