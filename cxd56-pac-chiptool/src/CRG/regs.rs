///eMMC clock setting.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CKEN_EMMC(pub u32);
impl CKEN_EMMC {
    ///Enable eMMC clock.
    #[must_use]
    #[inline(always)]
    pub const fn CLKIN(&self) -> super::vals::CLKIN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CLKIN::from_bits(val as u8)
    }
    ///Enable eMMC clock.
    #[inline(always)]
    pub const fn set_CLKIN(&mut self, val: super::vals::CLKIN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CKEN_EMMC {
    #[inline(always)]
    fn default() -> CKEN_EMMC {
        CKEN_EMMC(0)
    }
}
impl core::fmt::Debug for CKEN_EMMC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKEN_EMMC")
            .field("CLKIN", &self.CLKIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CKEN_EMMC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CKEN_EMMC {{ CLKIN: {:?} }}", self.CLKIN())
    }
}
///CPU/Peripheral clock gating control.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CK_GATE_AHB(pub u32);
impl CK_GATE_AHB {
    ///0=Gated, 1=Ungated.
    #[must_use]
    #[inline(always)]
    pub const fn CK_GATE_AUD(&self) -> super::vals::CK_GATE_AUD {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CK_GATE_AUD::from_bits(val as u8)
    }
    ///0=Gated, 1=Ungated.
    #[inline(always)]
    pub const fn set_CK_GATE_AUD(&mut self, val: super::vals::CK_GATE_AUD) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CK_GATE_AHB {
    #[inline(always)]
    fn default() -> CK_GATE_AHB {
        CK_GATE_AHB(0)
    }
}
impl core::fmt::Debug for CK_GATE_AHB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CK_GATE_AHB")
            .field("CK_GATE_AUD", &self.CK_GATE_AUD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CK_GATE_AHB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CK_GATE_AHB {{ CK_GATE_AUD: {:?} }}", self.CK_GATE_AUD())
    }
}
///Gear ratio (n/m) for AHB.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GEAR_AHB(pub u32);
impl GEAR_AHB {
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_M_AHB(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_GEAR_M_AHB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_N_AHB(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_GEAR_N_AHB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for GEAR_AHB {
    #[inline(always)]
    fn default() -> GEAR_AHB {
        GEAR_AHB(0)
    }
}
impl core::fmt::Debug for GEAR_AHB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEAR_AHB")
            .field("GEAR_M_AHB", &self.GEAR_M_AHB())
            .field("GEAR_N_AHB", &self.GEAR_N_AHB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GEAR_AHB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GEAR_AHB {{ GEAR_M_AHB: {=u8:?}, GEAR_N_AHB: {=u8:?} }}",
            self.GEAR_M_AHB(),
            self.GEAR_N_AHB()
        )
    }
}
///IMG SPI clock setting.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GEAR_IMG_SPI(pub u32);
impl GEAR_IMG_SPI {
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_M_SPI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_GEAR_M_SPI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_N_SPI(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GEAR_N_SPI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GEAR_IMG_SPI {
    #[inline(always)]
    fn default() -> GEAR_IMG_SPI {
        GEAR_IMG_SPI(0)
    }
}
impl core::fmt::Debug for GEAR_IMG_SPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEAR_IMG_SPI")
            .field("GEAR_M_SPI", &self.GEAR_M_SPI())
            .field("GEAR_N_SPI", &self.GEAR_N_SPI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GEAR_IMG_SPI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GEAR_IMG_SPI {{ GEAR_M_SPI: {=u8:?}, GEAR_N_SPI: {=bool:?} }}",
            self.GEAR_M_SPI(),
            self.GEAR_N_SPI()
        )
    }
}
///IMG UART clock setting.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GEAR_IMG_UART(pub u32);
impl GEAR_IMG_UART {
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_M_UART(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_GEAR_M_UART(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_N_UART(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GEAR_N_UART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GEAR_IMG_UART {
    #[inline(always)]
    fn default() -> GEAR_IMG_UART {
        GEAR_IMG_UART(0)
    }
}
impl core::fmt::Debug for GEAR_IMG_UART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEAR_IMG_UART")
            .field("GEAR_M_UART", &self.GEAR_M_UART())
            .field("GEAR_N_UART", &self.GEAR_N_UART())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GEAR_IMG_UART {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GEAR_IMG_UART {{ GEAR_M_UART: {=u8:?}, GEAR_N_UART: {=bool:?} }}",
            self.GEAR_M_UART(),
            self.GEAR_N_UART()
        )
    }
}
///IMG WSPI clock setting.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GEAR_IMG_WSPI(pub u32);
impl GEAR_IMG_WSPI {
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_M_IMG_WSPI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_GEAR_M_IMG_WSPI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_N_IMG_WSPI(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GEAR_N_IMG_WSPI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GEAR_IMG_WSPI {
    #[inline(always)]
    fn default() -> GEAR_IMG_WSPI {
        GEAR_IMG_WSPI(0)
    }
}
impl core::fmt::Debug for GEAR_IMG_WSPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEAR_IMG_WSPI")
            .field("GEAR_M_IMG_WSPI", &self.GEAR_M_IMG_WSPI())
            .field("GEAR_N_IMG_WSPI", &self.GEAR_N_IMG_WSPI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GEAR_IMG_WSPI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GEAR_IMG_WSPI {{ GEAR_M_IMG_WSPI: {=u8:?}, GEAR_N_IMG_WSPI: {=bool:?} }}",
            self.GEAR_M_IMG_WSPI(),
            self.GEAR_N_IMG_WSPI()
        )
    }
}
///SDIO clock setting.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GEAR_PER_SDIO(pub u32);
impl GEAR_PER_SDIO {
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_M_SDIO(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_GEAR_M_SDIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_N_SDIO(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GEAR_N_SDIO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GEAR_PER_SDIO {
    #[inline(always)]
    fn default() -> GEAR_PER_SDIO {
        GEAR_PER_SDIO(0)
    }
}
impl core::fmt::Debug for GEAR_PER_SDIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEAR_PER_SDIO")
            .field("GEAR_M_SDIO", &self.GEAR_M_SDIO())
            .field("GEAR_N_SDIO", &self.GEAR_N_SDIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GEAR_PER_SDIO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GEAR_PER_SDIO {{ GEAR_M_SDIO: {=u8:?}, GEAR_N_SDIO: {=bool:?} }}",
            self.GEAR_M_SDIO(),
            self.GEAR_N_SDIO()
        )
    }
}
///USB clock setting.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GEAR_PER_USB(pub u32);
impl GEAR_PER_USB {
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_M_USB(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_GEAR_M_USB(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GEAR_N_USB(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GEAR_N_USB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GEAR_PER_USB {
    #[inline(always)]
    fn default() -> GEAR_PER_USB {
        GEAR_PER_USB(0)
    }
}
impl core::fmt::Debug for GEAR_PER_USB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEAR_PER_USB")
            .field("GEAR_M_USB", &self.GEAR_M_USB())
            .field("GEAR_N_USB", &self.GEAR_N_USB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GEAR_PER_USB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GEAR_PER_USB {{ GEAR_M_USB: {=u16:?}, GEAR_N_USB: {=bool:?} }}",
            self.GEAR_M_USB(),
            self.GEAR_N_USB()
        )
    }
}
///Reset control.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESET(pub u32);
impl RESET {
    ///0=reset, 1=active.
    #[must_use]
    #[inline(always)]
    pub const fn XRS_AUD(&self) -> super::vals::XRS_AUD {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XRS_AUD::from_bits(val as u8)
    }
    ///0=reset, 1=active.
    #[inline(always)]
    pub const fn set_XRS_AUD(&mut self, val: super::vals::XRS_AUD) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for RESET {
    #[inline(always)]
    fn default() -> RESET {
        RESET(0)
    }
}
impl core::fmt::Debug for RESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET")
            .field("XRS_AUD", &self.XRS_AUD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESET {{ XRS_AUD: {:?} }}", self.XRS_AUD())
    }
}
