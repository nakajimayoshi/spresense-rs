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
///GPIO APP pin 75 — SPI5_SCK / Arduino D23 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_EMMC_CLK(pub u32);
impl GP_EMMC_CLK {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_EMMC_CLK {
    #[inline(always)]
    fn default() -> GP_EMMC_CLK {
        GP_EMMC_CLK(0)
    }
}
impl core::fmt::Debug for GP_EMMC_CLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_EMMC_CLK")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_EMMC_CLK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_EMMC_CLK {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 76 — SPI5_CS_X / Arduino D24 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_EMMC_CMD(pub u32);
impl GP_EMMC_CMD {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_EMMC_CMD {
    #[inline(always)]
    fn default() -> GP_EMMC_CMD {
        GP_EMMC_CMD(0)
    }
}
impl core::fmt::Debug for GP_EMMC_CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_EMMC_CMD")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_EMMC_CMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_EMMC_CMD {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 77 — SPI5_MOSI / Arduino D16 (JP2).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_EMMC_DATA0(pub u32);
impl GP_EMMC_DATA0 {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_EMMC_DATA0 {
    #[inline(always)]
    fn default() -> GP_EMMC_DATA0 {
        GP_EMMC_DATA0(0)
    }
}
impl core::fmt::Debug for GP_EMMC_DATA0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_EMMC_DATA0")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_EMMC_DATA0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_EMMC_DATA0 {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 78 — SPI5_MISO / Arduino D17 (JP2).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_EMMC_DATA1(pub u32);
impl GP_EMMC_DATA1 {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_EMMC_DATA1 {
    #[inline(always)]
    fn default() -> GP_EMMC_DATA1 {
        GP_EMMC_DATA1(0)
    }
}
impl core::fmt::Debug for GP_EMMC_DATA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_EMMC_DATA1")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_EMMC_DATA1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_EMMC_DATA1 {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 79 — GPIO / Arduino D20 (JP2).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_EMMC_DATA2(pub u32);
impl GP_EMMC_DATA2 {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_EMMC_DATA2 {
    #[inline(always)]
    fn default() -> GP_EMMC_DATA2 {
        GP_EMMC_DATA2(0)
    }
}
impl core::fmt::Debug for GP_EMMC_DATA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_EMMC_DATA2")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_EMMC_DATA2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_EMMC_DATA2 {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 80 — GPIO / Arduino D21 (JP2).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_EMMC_DATA3(pub u32);
impl GP_EMMC_DATA3 {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_EMMC_DATA3 {
    #[inline(always)]
    fn default() -> GP_EMMC_DATA3 {
        GP_EMMC_DATA3(0)
    }
}
impl core::fmt::Debug for GP_EMMC_DATA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_EMMC_DATA3")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_EMMC_DATA3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_EMMC_DATA3 {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO SYS pin 44 — I2C0_SCL / Arduino D15 (JP2).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2C0_BCK(pub u32);
impl GP_I2C0_BCK {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2C0_BCK {
    #[inline(always)]
    fn default() -> GP_I2C0_BCK {
        GP_I2C0_BCK(0)
    }
}
impl core::fmt::Debug for GP_I2C0_BCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2C0_BCK")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2C0_BCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2C0_BCK {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO SYS pin 45 — I2C0_SDA / Arduino D14 (JP2).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2C0_BDT(pub u32);
impl GP_I2C0_BDT {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2C0_BDT {
    #[inline(always)]
    fn default() -> GP_I2C0_BDT {
        GP_I2C0_BDT(0)
    }
}
impl core::fmt::Debug for GP_I2C0_BDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2C0_BDT")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2C0_BDT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2C0_BDT {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO SYS pin 1 — I2C4 clock / Arduino D14.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2C4_BCK(pub u32);
impl GP_I2C4_BCK {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2C4_BCK {
    #[inline(always)]
    fn default() -> GP_I2C4_BCK {
        GP_I2C4_BCK(0)
    }
}
impl core::fmt::Debug for GP_I2C4_BCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2C4_BCK")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2C4_BCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2C4_BCK {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 93 — I2S0_BCK / Arduino D26 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2S0_BCK(pub u32);
impl GP_I2S0_BCK {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2S0_BCK {
    #[inline(always)]
    fn default() -> GP_I2S0_BCK {
        GP_I2S0_BCK(0)
    }
}
impl core::fmt::Debug for GP_I2S0_BCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2S0_BCK")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2S0_BCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2S0_BCK {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 95 — I2S0_DATA_IN / Arduino D19 (JP2).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2S0_DATA_IN(pub u32);
impl GP_I2S0_DATA_IN {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2S0_DATA_IN {
    #[inline(always)]
    fn default() -> GP_I2S0_DATA_IN {
        GP_I2S0_DATA_IN(0)
    }
}
impl core::fmt::Debug for GP_I2S0_DATA_IN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2S0_DATA_IN")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2S0_DATA_IN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2S0_DATA_IN {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 96 — I2S0_DATA_OUT / Arduino D18 (JP2).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2S0_DATA_OUT(pub u32);
impl GP_I2S0_DATA_OUT {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2S0_DATA_OUT {
    #[inline(always)]
    fn default() -> GP_I2S0_DATA_OUT {
        GP_I2S0_DATA_OUT(0)
    }
}
impl core::fmt::Debug for GP_I2S0_DATA_OUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2S0_DATA_OUT")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2S0_DATA_OUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2S0_DATA_OUT {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 94 — I2S0_LRCK / Arduino D25 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2S0_LRCK(pub u32);
impl GP_I2S0_LRCK {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2S0_LRCK {
    #[inline(always)]
    fn default() -> GP_I2S0_LRCK {
        GP_I2S0_LRCK(0)
    }
}
impl core::fmt::Debug for GP_I2S0_LRCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2S0_LRCK")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2S0_LRCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2S0_LRCK {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2S1_BCK(pub u32);
impl GP_I2S1_BCK {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2S1_BCK {
    #[inline(always)]
    fn default() -> GP_I2S1_BCK {
        GP_I2S1_BCK(0)
    }
}
impl core::fmt::Debug for GP_I2S1_BCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2S1_BCK")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2S1_BCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2S1_BCK {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2S1_DATA_IN(pub u32);
impl GP_I2S1_DATA_IN {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2S1_DATA_IN {
    #[inline(always)]
    fn default() -> GP_I2S1_DATA_IN {
        GP_I2S1_DATA_IN(0)
    }
}
impl core::fmt::Debug for GP_I2S1_DATA_IN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2S1_DATA_IN")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2S1_DATA_IN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2S1_DATA_IN {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2S1_DATA_OUT(pub u32);
impl GP_I2S1_DATA_OUT {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2S1_DATA_OUT {
    #[inline(always)]
    fn default() -> GP_I2S1_DATA_OUT {
        GP_I2S1_DATA_OUT(0)
    }
}
impl core::fmt::Debug for GP_I2S1_DATA_OUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2S1_DATA_OUT")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2S1_DATA_OUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2S1_DATA_OUT {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_I2S1_LRCK(pub u32);
impl GP_I2S1_LRCK {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_I2S1_LRCK {
    #[inline(always)]
    fn default() -> GP_I2S1_LRCK {
        GP_I2S1_LRCK(0)
    }
}
impl core::fmt::Debug for GP_I2S1_LRCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_I2S1_LRCK")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_I2S1_LRCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_I2S1_LRCK {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO SYS pin 37 — SEN_IRQ / Arduino D22 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_SEN_IRQ_IN(pub u32);
impl GP_SEN_IRQ_IN {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_SEN_IRQ_IN {
    #[inline(always)]
    fn default() -> GP_SEN_IRQ_IN {
        GP_SEN_IRQ_IN(0)
    }
}
impl core::fmt::Debug for GP_SEN_IRQ_IN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_SEN_IRQ_IN")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_SEN_IRQ_IN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_SEN_IRQ_IN {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO SYS pin 17 — SPI0_CS_X / UART1_TX (on-board console).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_SPI0_CS_X(pub u32);
impl GP_SPI0_CS_X {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_SPI0_CS_X {
    #[inline(always)]
    fn default() -> GP_SPI0_CS_X {
        GP_SPI0_CS_X(0)
    }
}
impl core::fmt::Debug for GP_SPI0_CS_X {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_SPI0_CS_X")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_SPI0_CS_X {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_SPI0_CS_X {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO SYS pin 18 — SPI0_SCK / UART1_RX (on-board console).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_SPI0_SCK(pub u32);
impl GP_SPI0_SCK {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_SPI0_SCK {
    #[inline(always)]
    fn default() -> GP_SPI0_SCK {
        GP_SPI0_SCK(0)
    }
}
impl core::fmt::Debug for GP_SPI0_SCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_SPI0_SCK")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_SPI0_SCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_SPI0_SCK {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 69 — UART2_CTS / Arduino D27 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_UART2_CTS(pub u32);
impl GP_UART2_CTS {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_UART2_CTS {
    #[inline(always)]
    fn default() -> GP_UART2_CTS {
        GP_UART2_CTS(0)
    }
}
impl core::fmt::Debug for GP_UART2_CTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_UART2_CTS")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_UART2_CTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_UART2_CTS {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 70 — UART2_RTS / Arduino D28 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_UART2_RTS(pub u32);
impl GP_UART2_RTS {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_UART2_RTS {
    #[inline(always)]
    fn default() -> GP_UART2_RTS {
        GP_UART2_RTS(0)
    }
}
impl core::fmt::Debug for GP_UART2_RTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_UART2_RTS")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_UART2_RTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_UART2_RTS {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 68 — UART2_RX / Arduino D00 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_UART2_RXD(pub u32);
impl GP_UART2_RXD {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_UART2_RXD {
    #[inline(always)]
    fn default() -> GP_UART2_RXD {
        GP_UART2_RXD(0)
    }
}
impl core::fmt::Debug for GP_UART2_RXD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_UART2_RXD")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_UART2_RXD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_UART2_RXD {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///GPIO APP pin 67 — UART2_TX / Arduino D01 (JP1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GP_UART2_TXD(pub u32);
impl GP_UART2_TXD {
    ///Sampled pin level (read).
    #[must_use]
    #[inline(always)]
    pub const fn IN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    ///Sampled pin level (read).
    #[inline(always)]
    pub const fn set_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    ///Output data.
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    ///Output data.
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Output enable, active-low (0 = drive output, 1 = high-Z input).
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GP_UART2_TXD {
    #[inline(always)]
    fn default() -> GP_UART2_TXD {
        GP_UART2_TXD(0)
    }
}
impl core::fmt::Debug for GP_UART2_TXD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GP_UART2_TXD")
            .field("IN", &self.IN())
            .field("OUT", &self.OUT())
            .field("DIR", &self.DIR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GP_UART2_TXD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GP_UART2_TXD {{ IN: {=bool:?}, OUT: {=bool:?}, DIR: {=bool:?} }}",
            self.IN(),
            self.OUT(),
            self.DIR()
        )
    }
}
///APP-domain GPIO interrupt slot mux, slots 6–9 (1 byte per slot, pin index 0–63).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCAPP_INTSEL0(pub u32);
impl IOCAPP_INTSEL0 {
    ///Pin index for APP slot 6 → EXDEVICE_6 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    ///Pin index for APP slot 6 → EXDEVICE_6 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    ///Pin index for APP slot 7 → EXDEVICE_7 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    ///Pin index for APP slot 7 → EXDEVICE_7 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    ///Pin index for APP slot 8 → EXDEVICE_8 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    ///Pin index for APP slot 8 → EXDEVICE_8 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    ///Pin index for APP slot 9 → EXDEVICE_9 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    ///Pin index for APP slot 9 → EXDEVICE_9 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for IOCAPP_INTSEL0 {
    #[inline(always)]
    fn default() -> IOCAPP_INTSEL0 {
        IOCAPP_INTSEL0(0)
    }
}
impl core::fmt::Debug for IOCAPP_INTSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCAPP_INTSEL0")
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCAPP_INTSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCAPP_INTSEL0 {{ SLOT6: {=u8:?}, SLOT7: {=u8:?}, SLOT8: {=u8:?}, SLOT9: {=u8:?} }}",
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9()
        )
    }
}
///APP-domain GPIO interrupt slot mux, slots 10–11.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCAPP_INTSEL1(pub u32);
impl IOCAPP_INTSEL1 {
    ///Pin index for APP slot 10 → EXDEVICE_10 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    ///Pin index for APP slot 10 → EXDEVICE_10 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    ///Pin index for APP slot 11 → EXDEVICE_11 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    ///Pin index for APP slot 11 → EXDEVICE_11 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for IOCAPP_INTSEL1 {
    #[inline(always)]
    fn default() -> IOCAPP_INTSEL1 {
        IOCAPP_INTSEL1(0)
    }
}
impl core::fmt::Debug for IOCAPP_INTSEL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCAPP_INTSEL1")
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCAPP_INTSEL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCAPP_INTSEL1 {{ SLOT10: {=u8:?}, SLOT11: {=u8:?} }}",
            self.SLOT10(),
            self.SLOT11()
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
    ///Mode select for EMMC_CLK/CMD/DATA0/DATA1 pins (Func1=eMMC, Func2=SPI5).
    #[must_use]
    #[inline(always)]
    pub const fn EMMCA(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    ///Mode select for EMMC_CLK/CMD/DATA0/DATA1 pins (Func1=eMMC, Func2=SPI5).
    #[inline(always)]
    pub const fn set_EMMCA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    ///Mode select for EMMC_DATA2/DATA3 pins (Func1=eMMC).
    #[must_use]
    #[inline(always)]
    pub const fn EMMCB(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    ///Mode select for EMMC_DATA2/DATA3 pins (Func1=eMMC).
    #[inline(always)]
    pub const fn set_EMMCB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    ///Mode select for I2S0 BCK/LRCK/DATA_IN/DATA_OUT pins (I2S0 = Func1).
    #[must_use]
    #[inline(always)]
    pub const fn I2S0(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    ///Mode select for I2S0 BCK/LRCK/DATA_IN/DATA_OUT pins (I2S0 = Func1).
    #[inline(always)]
    pub const fn set_I2S0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
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
            .field("EMMCA", &self.EMMCA())
            .field("EMMCB", &self.EMMCB())
            .field("I2S0", &self.I2S0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCAPP_IOMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCAPP_IOMD {{ UART2: {=u8:?}, EMMCA: {=u8:?}, EMMCB: {=u8:?}, I2S0: {=u8:?} }}",
            self.UART2(),
            self.EMMCA(),
            self.EMMCB(),
            self.I2S0()
        )
    }
}
///SYS-domain GPIO interrupt slot mux, slots 0–3 (1 byte per slot, pin index 0–63).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCSYS_INTSEL0(pub u32);
impl IOCSYS_INTSEL0 {
    ///Pin index for SYS slot 0 → EXDEVICE_0 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    ///Pin index for SYS slot 0 → EXDEVICE_0 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    ///Pin index for SYS slot 1 → EXDEVICE_1 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    ///Pin index for SYS slot 1 → EXDEVICE_1 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    ///Pin index for SYS slot 2 → EXDEVICE_2 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    ///Pin index for SYS slot 2 → EXDEVICE_2 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    ///Pin index for SYS slot 3 → EXDEVICE_3 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    ///Pin index for SYS slot 3 → EXDEVICE_3 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for IOCSYS_INTSEL0 {
    #[inline(always)]
    fn default() -> IOCSYS_INTSEL0 {
        IOCSYS_INTSEL0(0)
    }
}
impl core::fmt::Debug for IOCSYS_INTSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCSYS_INTSEL0")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCSYS_INTSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCSYS_INTSEL0 {{ SLOT0: {=u8:?}, SLOT1: {=u8:?}, SLOT2: {=u8:?}, SLOT3: {=u8:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3()
        )
    }
}
///SYS-domain GPIO interrupt slot mux, slots 4–5.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCSYS_INTSEL1(pub u32);
impl IOCSYS_INTSEL1 {
    ///Pin index for SYS slot 4 → EXDEVICE_4 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    ///Pin index for SYS slot 4 → EXDEVICE_4 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    ///Pin index for SYS slot 5 → EXDEVICE_5 (0x3f = unused).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    ///Pin index for SYS slot 5 → EXDEVICE_5 (0x3f = unused).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for IOCSYS_INTSEL1 {
    #[inline(always)]
    fn default() -> IOCSYS_INTSEL1 {
        IOCSYS_INTSEL1(0)
    }
}
impl core::fmt::Debug for IOCSYS_INTSEL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCSYS_INTSEL1")
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCSYS_INTSEL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCSYS_INTSEL1 {{ SLOT4: {=u8:?}, SLOT5: {=u8:?} }}",
            self.SLOT4(),
            self.SLOT5()
        )
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
///SYSIOP IO-cell mode-mux register 1.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCSYS_IOMD1(pub u32);
impl IOCSYS_IOMD1 {
    ///Mode select for SEN_IRQ_IN pin (SEN_IRQ_IN = Func1).
    #[must_use]
    #[inline(always)]
    pub const fn SEN_IRQ_IN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    ///Mode select for SEN_IRQ_IN pin (SEN_IRQ_IN = Func1).
    #[inline(always)]
    pub const fn set_SEN_IRQ_IN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    ///Mode select for I2C0_BCK / I2C0_BDT (I2C0 = Func1).
    #[must_use]
    #[inline(always)]
    pub const fn I2C0(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    ///Mode select for I2C0_BCK / I2C0_BDT (I2C0 = Func1).
    #[inline(always)]
    pub const fn set_I2C0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for IOCSYS_IOMD1 {
    #[inline(always)]
    fn default() -> IOCSYS_IOMD1 {
        IOCSYS_IOMD1(0)
    }
}
impl core::fmt::Debug for IOCSYS_IOMD1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCSYS_IOMD1")
            .field("SEN_IRQ_IN", &self.SEN_IRQ_IN())
            .field("I2C0", &self.I2C0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCSYS_IOMD1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCSYS_IOMD1 {{ SEN_IRQ_IN: {=u8:?}, I2C0: {=u8:?} }}",
            self.SEN_IRQ_IN(),
            self.I2C0()
        )
    }
}
///IOCELL control for EMMC_CLK / SPI5_SCK pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_EMMC_CLK(pub u32);
impl IO_EMMC_CLK {
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
impl Default for IO_EMMC_CLK {
    #[inline(always)]
    fn default() -> IO_EMMC_CLK {
        IO_EMMC_CLK(0)
    }
}
impl core::fmt::Debug for IO_EMMC_CLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_EMMC_CLK")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_EMMC_CLK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_EMMC_CLK {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for EMMC_CMD / SPI5_CS_X pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_EMMC_CMD(pub u32);
impl IO_EMMC_CMD {
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
impl Default for IO_EMMC_CMD {
    #[inline(always)]
    fn default() -> IO_EMMC_CMD {
        IO_EMMC_CMD(0)
    }
}
impl core::fmt::Debug for IO_EMMC_CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_EMMC_CMD")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_EMMC_CMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_EMMC_CMD {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for EMMC_DATA0 / SPI5_MOSI pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_EMMC_DATA0(pub u32);
impl IO_EMMC_DATA0 {
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
impl Default for IO_EMMC_DATA0 {
    #[inline(always)]
    fn default() -> IO_EMMC_DATA0 {
        IO_EMMC_DATA0(0)
    }
}
impl core::fmt::Debug for IO_EMMC_DATA0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_EMMC_DATA0")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_EMMC_DATA0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_EMMC_DATA0 {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for EMMC_DATA1 / SPI5_MISO pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_EMMC_DATA1(pub u32);
impl IO_EMMC_DATA1 {
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
impl Default for IO_EMMC_DATA1 {
    #[inline(always)]
    fn default() -> IO_EMMC_DATA1 {
        IO_EMMC_DATA1(0)
    }
}
impl core::fmt::Debug for IO_EMMC_DATA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_EMMC_DATA1")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_EMMC_DATA1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_EMMC_DATA1 {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for EMMC_DATA2 pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_EMMC_DATA2(pub u32);
impl IO_EMMC_DATA2 {
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
impl Default for IO_EMMC_DATA2 {
    #[inline(always)]
    fn default() -> IO_EMMC_DATA2 {
        IO_EMMC_DATA2(0)
    }
}
impl core::fmt::Debug for IO_EMMC_DATA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_EMMC_DATA2")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_EMMC_DATA2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_EMMC_DATA2 {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for EMMC_DATA3 pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_EMMC_DATA3(pub u32);
impl IO_EMMC_DATA3 {
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
impl Default for IO_EMMC_DATA3 {
    #[inline(always)]
    fn default() -> IO_EMMC_DATA3 {
        IO_EMMC_DATA3(0)
    }
}
impl core::fmt::Debug for IO_EMMC_DATA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_EMMC_DATA3")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_EMMC_DATA3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_EMMC_DATA3 {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for I2C0_BCK pad.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_I2C0_BCK(pub u32);
impl IO_I2C0_BCK {
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
impl Default for IO_I2C0_BCK {
    #[inline(always)]
    fn default() -> IO_I2C0_BCK {
        IO_I2C0_BCK(0)
    }
}
impl core::fmt::Debug for IO_I2C0_BCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_I2C0_BCK")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_I2C0_BCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_I2C0_BCK {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for I2C0_BDT pad.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_I2C0_BDT(pub u32);
impl IO_I2C0_BDT {
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
impl Default for IO_I2C0_BDT {
    #[inline(always)]
    fn default() -> IO_I2C0_BDT {
        IO_I2C0_BDT(0)
    }
}
impl core::fmt::Debug for IO_I2C0_BDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_I2C0_BDT")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_I2C0_BDT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_I2C0_BDT {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for I2S0_BCK pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_I2S0_BCK(pub u32);
impl IO_I2S0_BCK {
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
impl Default for IO_I2S0_BCK {
    #[inline(always)]
    fn default() -> IO_I2S0_BCK {
        IO_I2S0_BCK(0)
    }
}
impl core::fmt::Debug for IO_I2S0_BCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_I2S0_BCK")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_I2S0_BCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_I2S0_BCK {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for I2S0_DATA_IN pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_I2S0_DATA_IN(pub u32);
impl IO_I2S0_DATA_IN {
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
impl Default for IO_I2S0_DATA_IN {
    #[inline(always)]
    fn default() -> IO_I2S0_DATA_IN {
        IO_I2S0_DATA_IN(0)
    }
}
impl core::fmt::Debug for IO_I2S0_DATA_IN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_I2S0_DATA_IN")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_I2S0_DATA_IN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_I2S0_DATA_IN {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for I2S0_DATA_OUT pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_I2S0_DATA_OUT(pub u32);
impl IO_I2S0_DATA_OUT {
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
impl Default for IO_I2S0_DATA_OUT {
    #[inline(always)]
    fn default() -> IO_I2S0_DATA_OUT {
        IO_I2S0_DATA_OUT(0)
    }
}
impl core::fmt::Debug for IO_I2S0_DATA_OUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_I2S0_DATA_OUT")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_I2S0_DATA_OUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_I2S0_DATA_OUT {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for I2S0_LRCK pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_I2S0_LRCK(pub u32);
impl IO_I2S0_LRCK {
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
impl Default for IO_I2S0_LRCK {
    #[inline(always)]
    fn default() -> IO_I2S0_LRCK {
        IO_I2S0_LRCK(0)
    }
}
impl core::fmt::Debug for IO_I2S0_LRCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_I2S0_LRCK")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_I2S0_LRCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_I2S0_LRCK {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for SEN_IRQ_IN pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_SEN_IRQ_IN(pub u32);
impl IO_SEN_IRQ_IN {
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
impl Default for IO_SEN_IRQ_IN {
    #[inline(always)]
    fn default() -> IO_SEN_IRQ_IN {
        IO_SEN_IRQ_IN(0)
    }
}
impl core::fmt::Debug for IO_SEN_IRQ_IN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_SEN_IRQ_IN")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_SEN_IRQ_IN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_SEN_IRQ_IN {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
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
///IOCELL control for UART2 CTS pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_UART2_CTS(pub u32);
impl IO_UART2_CTS {
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
impl Default for IO_UART2_CTS {
    #[inline(always)]
    fn default() -> IO_UART2_CTS {
        IO_UART2_CTS(0)
    }
}
impl core::fmt::Debug for IO_UART2_CTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_UART2_CTS")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_UART2_CTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_UART2_CTS {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
            self.ENZI(),
            self.PUN(),
            self.PDN(),
            self.LOWEMI()
        )
    }
}
///IOCELL control for UART2 RTS pin.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IO_UART2_RTS(pub u32);
impl IO_UART2_RTS {
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
impl Default for IO_UART2_RTS {
    #[inline(always)]
    fn default() -> IO_UART2_RTS {
        IO_UART2_RTS(0)
    }
}
impl core::fmt::Debug for IO_UART2_RTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_UART2_RTS")
            .field("ENZI", &self.ENZI())
            .field("PUN", &self.PUN())
            .field("PDN", &self.PDN())
            .field("LOWEMI", &self.LOWEMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IO_UART2_RTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IO_UART2_RTS {{ ENZI: {=bool:?}, PUN: {=bool:?}, PDN: {=bool:?}, LOWEMI: {=bool:?} }}",
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
///CPU interrupt route select, slots 0–3 (3-bit field per slot at 16+slot*4).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG_CPUINTSEL0(pub u32);
impl PMU_WAKE_TRIG_CPUINTSEL0 {
    ///Slot 0 route (EXDEVICE_0).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    ///Slot 0 route (EXDEVICE_0).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    ///Slot 1 route (EXDEVICE_1).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    ///Slot 1 route (EXDEVICE_1).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    ///Slot 2 route (EXDEVICE_2).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    ///Slot 2 route (EXDEVICE_2).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    ///Slot 3 route (EXDEVICE_3).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    ///Slot 3 route (EXDEVICE_3).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for PMU_WAKE_TRIG_CPUINTSEL0 {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG_CPUINTSEL0 {
        PMU_WAKE_TRIG_CPUINTSEL0(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG_CPUINTSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG_CPUINTSEL0")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG_CPUINTSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG_CPUINTSEL0 {{ SLOT0: {=u8:?}, SLOT1: {=u8:?}, SLOT2: {=u8:?}, SLOT3: {=u8:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3()
        )
    }
}
///CPU interrupt route select, slots 4–11 (3-bit field per slot at slot*4).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG_CPUINTSEL1(pub u32);
impl PMU_WAKE_TRIG_CPUINTSEL1 {
    ///Slot 4 route (EXDEVICE_4).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    ///Slot 4 route (EXDEVICE_4).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    ///Slot 5 route (EXDEVICE_5).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    ///Slot 5 route (EXDEVICE_5).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    ///Slot 6 route (EXDEVICE_6).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    ///Slot 6 route (EXDEVICE_6).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    ///Slot 7 route (EXDEVICE_7).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    ///Slot 7 route (EXDEVICE_7).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    ///Slot 8 route (EXDEVICE_8).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    ///Slot 8 route (EXDEVICE_8).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    ///Slot 9 route (EXDEVICE_9).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    ///Slot 9 route (EXDEVICE_9).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    ///Slot 10 route (EXDEVICE_10).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    ///Slot 10 route (EXDEVICE_10).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    ///Slot 11 route (EXDEVICE_11).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    ///Slot 11 route (EXDEVICE_11).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for PMU_WAKE_TRIG_CPUINTSEL1 {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG_CPUINTSEL1 {
        PMU_WAKE_TRIG_CPUINTSEL1(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG_CPUINTSEL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG_CPUINTSEL1")
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG_CPUINTSEL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG_CPUINTSEL1 {{ SLOT4: {=u8:?}, SLOT5: {=u8:?}, SLOT6: {=u8:?}, SLOT7: {=u8:?}, SLOT8: {=u8:?}, SLOT9: {=u8:?}, SLOT10: {=u8:?}, SLOT11: {=u8:?} }}",
            self.SLOT4(),
            self.SLOT5(),
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9(),
            self.SLOT10(),
            self.SLOT11()
        )
    }
}
///Positive wake-trigger enable, slots 0–11.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG_EN0(pub u32);
impl PMU_WAKE_TRIG_EN0 {
    ///Slot 0 positive-trigger enable (EXDEVICE_0).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Slot 0 positive-trigger enable (EXDEVICE_0).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Slot 1 positive-trigger enable (EXDEVICE_1).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Slot 1 positive-trigger enable (EXDEVICE_1).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    ///Slot 2 positive-trigger enable (EXDEVICE_2).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    ///Slot 2 positive-trigger enable (EXDEVICE_2).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    ///Slot 3 positive-trigger enable (EXDEVICE_3).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    ///Slot 3 positive-trigger enable (EXDEVICE_3).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    ///Slot 4 positive-trigger enable (EXDEVICE_4).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    ///Slot 4 positive-trigger enable (EXDEVICE_4).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    ///Slot 5 positive-trigger enable (EXDEVICE_5).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    ///Slot 5 positive-trigger enable (EXDEVICE_5).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    ///Slot 6 positive-trigger enable (EXDEVICE_6).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    ///Slot 6 positive-trigger enable (EXDEVICE_6).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    ///Slot 7 positive-trigger enable (EXDEVICE_7).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    ///Slot 7 positive-trigger enable (EXDEVICE_7).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    ///Slot 8 positive-trigger enable (EXDEVICE_8).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Slot 8 positive-trigger enable (EXDEVICE_8).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    ///Slot 9 positive-trigger enable (EXDEVICE_9).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    ///Slot 9 positive-trigger enable (EXDEVICE_9).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    ///Slot 10 positive-trigger enable (EXDEVICE_10).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    ///Slot 10 positive-trigger enable (EXDEVICE_10).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    ///Slot 11 positive-trigger enable (EXDEVICE_11).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    ///Slot 11 positive-trigger enable (EXDEVICE_11).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for PMU_WAKE_TRIG_EN0 {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG_EN0 {
        PMU_WAKE_TRIG_EN0(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG_EN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG_EN0")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG_EN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG_EN0 {{ SLOT0: {=bool:?}, SLOT1: {=bool:?}, SLOT2: {=bool:?}, SLOT3: {=bool:?}, SLOT4: {=bool:?}, SLOT5: {=bool:?}, SLOT6: {=bool:?}, SLOT7: {=bool:?}, SLOT8: {=bool:?}, SLOT9: {=bool:?}, SLOT10: {=bool:?}, SLOT11: {=bool:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3(),
            self.SLOT4(),
            self.SLOT5(),
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9(),
            self.SLOT10(),
            self.SLOT11()
        )
    }
}
///Interrupt detect polarity, slots 0–3 (3-bit field per slot at 16+slot*4).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG_INTDET0(pub u32);
impl PMU_WAKE_TRIG_INTDET0 {
    ///Slot 0 polarity (EXDEVICE_0).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    ///Slot 0 polarity (EXDEVICE_0).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    ///Slot 1 polarity (EXDEVICE_1).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    ///Slot 1 polarity (EXDEVICE_1).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    ///Slot 2 polarity (EXDEVICE_2).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    ///Slot 2 polarity (EXDEVICE_2).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    ///Slot 3 polarity (EXDEVICE_3).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    ///Slot 3 polarity (EXDEVICE_3).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for PMU_WAKE_TRIG_INTDET0 {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG_INTDET0 {
        PMU_WAKE_TRIG_INTDET0(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG_INTDET0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG_INTDET0")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG_INTDET0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG_INTDET0 {{ SLOT0: {=u8:?}, SLOT1: {=u8:?}, SLOT2: {=u8:?}, SLOT3: {=u8:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3()
        )
    }
}
///Interrupt detect polarity, slots 4–11 (3-bit field per slot at slot*4).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG_INTDET1(pub u32);
impl PMU_WAKE_TRIG_INTDET1 {
    ///Slot 4 polarity (EXDEVICE_4).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    ///Slot 4 polarity (EXDEVICE_4).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    ///Slot 5 polarity (EXDEVICE_5).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    ///Slot 5 polarity (EXDEVICE_5).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    ///Slot 6 polarity (EXDEVICE_6).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    ///Slot 6 polarity (EXDEVICE_6).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    ///Slot 7 polarity (EXDEVICE_7).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    ///Slot 7 polarity (EXDEVICE_7).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    ///Slot 8 polarity (EXDEVICE_8).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    ///Slot 8 polarity (EXDEVICE_8).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    ///Slot 9 polarity (EXDEVICE_9).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    ///Slot 9 polarity (EXDEVICE_9).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    ///Slot 10 polarity (EXDEVICE_10).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    ///Slot 10 polarity (EXDEVICE_10).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    ///Slot 11 polarity (EXDEVICE_11).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    ///Slot 11 polarity (EXDEVICE_11).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for PMU_WAKE_TRIG_INTDET1 {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG_INTDET1 {
        PMU_WAKE_TRIG_INTDET1(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG_INTDET1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG_INTDET1")
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG_INTDET1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG_INTDET1 {{ SLOT4: {=u8:?}, SLOT5: {=u8:?}, SLOT6: {=u8:?}, SLOT7: {=u8:?}, SLOT8: {=u8:?}, SLOT9: {=u8:?}, SLOT10: {=u8:?}, SLOT11: {=u8:?} }}",
            self.SLOT4(),
            self.SLOT5(),
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9(),
            self.SLOT10(),
            self.SLOT11()
        )
    }
}
///Negative wake-trigger enable, slots 0–11.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG_NEGEN0(pub u32);
impl PMU_WAKE_TRIG_NEGEN0 {
    ///Slot 0 negative-trigger enable (EXDEVICE_0).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Slot 0 negative-trigger enable (EXDEVICE_0).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Slot 1 negative-trigger enable (EXDEVICE_1).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Slot 1 negative-trigger enable (EXDEVICE_1).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    ///Slot 2 negative-trigger enable (EXDEVICE_2).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    ///Slot 2 negative-trigger enable (EXDEVICE_2).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    ///Slot 3 negative-trigger enable (EXDEVICE_3).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    ///Slot 3 negative-trigger enable (EXDEVICE_3).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    ///Slot 4 negative-trigger enable (EXDEVICE_4).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    ///Slot 4 negative-trigger enable (EXDEVICE_4).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    ///Slot 5 negative-trigger enable (EXDEVICE_5).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    ///Slot 5 negative-trigger enable (EXDEVICE_5).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    ///Slot 6 negative-trigger enable (EXDEVICE_6).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    ///Slot 6 negative-trigger enable (EXDEVICE_6).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    ///Slot 7 negative-trigger enable (EXDEVICE_7).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    ///Slot 7 negative-trigger enable (EXDEVICE_7).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    ///Slot 8 negative-trigger enable (EXDEVICE_8).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Slot 8 negative-trigger enable (EXDEVICE_8).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    ///Slot 9 negative-trigger enable (EXDEVICE_9).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    ///Slot 9 negative-trigger enable (EXDEVICE_9).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    ///Slot 10 negative-trigger enable (EXDEVICE_10).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    ///Slot 10 negative-trigger enable (EXDEVICE_10).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    ///Slot 11 negative-trigger enable (EXDEVICE_11).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    ///Slot 11 negative-trigger enable (EXDEVICE_11).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for PMU_WAKE_TRIG_NEGEN0 {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG_NEGEN0 {
        PMU_WAKE_TRIG_NEGEN0(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG_NEGEN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG_NEGEN0")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG_NEGEN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG_NEGEN0 {{ SLOT0: {=bool:?}, SLOT1: {=bool:?}, SLOT2: {=bool:?}, SLOT3: {=bool:?}, SLOT4: {=bool:?}, SLOT5: {=bool:?}, SLOT6: {=bool:?}, SLOT7: {=bool:?}, SLOT8: {=bool:?}, SLOT9: {=bool:?}, SLOT10: {=bool:?}, SLOT11: {=bool:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3(),
            self.SLOT4(),
            self.SLOT5(),
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9(),
            self.SLOT10(),
            self.SLOT11()
        )
    }
}
///Noise filter enable for interrupt slots 0–11.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMU_WAKE_TRIG_NOISECUTEN0(pub u32);
impl PMU_WAKE_TRIG_NOISECUTEN0 {
    ///Slot 0 noise filter enable (EXDEVICE_0).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    ///Slot 0 noise filter enable (EXDEVICE_0).
    #[inline(always)]
    pub const fn set_SLOT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    ///Slot 1 noise filter enable (EXDEVICE_1).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    ///Slot 1 noise filter enable (EXDEVICE_1).
    #[inline(always)]
    pub const fn set_SLOT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    ///Slot 2 noise filter enable (EXDEVICE_2).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    ///Slot 2 noise filter enable (EXDEVICE_2).
    #[inline(always)]
    pub const fn set_SLOT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    ///Slot 3 noise filter enable (EXDEVICE_3).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    ///Slot 3 noise filter enable (EXDEVICE_3).
    #[inline(always)]
    pub const fn set_SLOT3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    ///Slot 4 noise filter enable (EXDEVICE_4).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    ///Slot 4 noise filter enable (EXDEVICE_4).
    #[inline(always)]
    pub const fn set_SLOT4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    ///Slot 5 noise filter enable (EXDEVICE_5).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    ///Slot 5 noise filter enable (EXDEVICE_5).
    #[inline(always)]
    pub const fn set_SLOT5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    ///Slot 6 noise filter enable (EXDEVICE_6).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    ///Slot 6 noise filter enable (EXDEVICE_6).
    #[inline(always)]
    pub const fn set_SLOT6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    ///Slot 7 noise filter enable (EXDEVICE_7).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    ///Slot 7 noise filter enable (EXDEVICE_7).
    #[inline(always)]
    pub const fn set_SLOT7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    ///Slot 8 noise filter enable (EXDEVICE_8).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    ///Slot 8 noise filter enable (EXDEVICE_8).
    #[inline(always)]
    pub const fn set_SLOT8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    ///Slot 9 noise filter enable (EXDEVICE_9).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    ///Slot 9 noise filter enable (EXDEVICE_9).
    #[inline(always)]
    pub const fn set_SLOT9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    ///Slot 10 noise filter enable (EXDEVICE_10).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    ///Slot 10 noise filter enable (EXDEVICE_10).
    #[inline(always)]
    pub const fn set_SLOT10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    ///Slot 11 noise filter enable (EXDEVICE_11).
    #[must_use]
    #[inline(always)]
    pub const fn SLOT11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    ///Slot 11 noise filter enable (EXDEVICE_11).
    #[inline(always)]
    pub const fn set_SLOT11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for PMU_WAKE_TRIG_NOISECUTEN0 {
    #[inline(always)]
    fn default() -> PMU_WAKE_TRIG_NOISECUTEN0 {
        PMU_WAKE_TRIG_NOISECUTEN0(0)
    }
}
impl core::fmt::Debug for PMU_WAKE_TRIG_NOISECUTEN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_WAKE_TRIG_NOISECUTEN0")
            .field("SLOT0", &self.SLOT0())
            .field("SLOT1", &self.SLOT1())
            .field("SLOT2", &self.SLOT2())
            .field("SLOT3", &self.SLOT3())
            .field("SLOT4", &self.SLOT4())
            .field("SLOT5", &self.SLOT5())
            .field("SLOT6", &self.SLOT6())
            .field("SLOT7", &self.SLOT7())
            .field("SLOT8", &self.SLOT8())
            .field("SLOT9", &self.SLOT9())
            .field("SLOT10", &self.SLOT10())
            .field("SLOT11", &self.SLOT11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMU_WAKE_TRIG_NOISECUTEN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMU_WAKE_TRIG_NOISECUTEN0 {{ SLOT0: {=bool:?}, SLOT1: {=bool:?}, SLOT2: {=bool:?}, SLOT3: {=bool:?}, SLOT4: {=bool:?}, SLOT5: {=bool:?}, SLOT6: {=bool:?}, SLOT7: {=bool:?}, SLOT8: {=bool:?}, SLOT9: {=bool:?}, SLOT10: {=bool:?}, SLOT11: {=bool:?} }}",
            self.SLOT0(),
            self.SLOT1(),
            self.SLOT2(),
            self.SLOT3(),
            self.SLOT4(),
            self.SLOT5(),
            self.SLOT6(),
            self.SLOT7(),
            self.SLOT8(),
            self.SLOT9(),
            self.SLOT10(),
            self.SLOT11()
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
