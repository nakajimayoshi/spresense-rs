#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gear_ahb: GearAhb,
    gear_img_uart: GearImgUart,
    gear_img_spi: GearImgSpi,
    gear_per_sdio: GearPerSdio,
    gear_per_usb: GearPerUsb,
    gear_m_img_venb: GearMImgVenb,
    gear_n_img_venb: GearNImgVenb,
    gear_img_wspi: GearImgWspi,
    cken_emmc: CkenEmmc,
    _reserved9: [u8; 0x0c],
    reset: Reset,
    _reserved10: [u8; 0x0c],
    ck_gate_ahb: CkGateAhb,
}
impl RegisterBlock {
    #[doc = "0x00 - Gear ratio (n/m) for AHB"]
    #[inline(always)]
    pub const fn gear_ahb(&self) -> &GearAhb {
        &self.gear_ahb
    }
    #[doc = "0x04 - IMG UART clock setting"]
    #[inline(always)]
    pub const fn gear_img_uart(&self) -> &GearImgUart {
        &self.gear_img_uart
    }
    #[doc = "0x08 - IMG SPI clock setting"]
    #[inline(always)]
    pub const fn gear_img_spi(&self) -> &GearImgSpi {
        &self.gear_img_spi
    }
    #[doc = "0x0c - SDIO clock setting"]
    #[inline(always)]
    pub const fn gear_per_sdio(&self) -> &GearPerSdio {
        &self.gear_per_sdio
    }
    #[doc = "0x10 - USB clock setting"]
    #[inline(always)]
    pub const fn gear_per_usb(&self) -> &GearPerUsb {
        &self.gear_per_usb
    }
    #[doc = "0x14 - VENB_M clock setting"]
    #[inline(always)]
    pub const fn gear_m_img_venb(&self) -> &GearMImgVenb {
        &self.gear_m_img_venb
    }
    #[doc = "0x18 - VENB_N clock setting"]
    #[inline(always)]
    pub const fn gear_n_img_venb(&self) -> &GearNImgVenb {
        &self.gear_n_img_venb
    }
    #[doc = "0x1c - IMG WSPI clock setting"]
    #[inline(always)]
    pub const fn gear_img_wspi(&self) -> &GearImgWspi {
        &self.gear_img_wspi
    }
    #[doc = "0x20 - eMMC clock setting"]
    #[inline(always)]
    pub const fn cken_emmc(&self) -> &CkenEmmc {
        &self.cken_emmc
    }
    #[doc = "0x30 - Reset control"]
    #[inline(always)]
    pub const fn reset(&self) -> &Reset {
        &self.reset
    }
    #[doc = "0x40 - CPU/Peripheral clock gating control"]
    #[inline(always)]
    pub const fn ck_gate_ahb(&self) -> &CkGateAhb {
        &self.ck_gate_ahb
    }
}
#[doc = "GEAR_AHB (rw) register accessor: Gear ratio (n/m) for AHB\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gear_ahb`] module"]
#[doc(alias = "GEAR_AHB")]
pub type GearAhb = crate::Reg<gear_ahb::GearAhbSpec>;
#[doc = "Gear ratio (n/m) for AHB"]
pub mod gear_ahb;
#[doc = "GEAR_IMG_UART (rw) register accessor: IMG UART clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_img_uart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_img_uart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gear_img_uart`] module"]
#[doc(alias = "GEAR_IMG_UART")]
pub type GearImgUart = crate::Reg<gear_img_uart::GearImgUartSpec>;
#[doc = "IMG UART clock setting"]
pub mod gear_img_uart;
#[doc = "GEAR_IMG_SPI (rw) register accessor: IMG SPI clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_img_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_img_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gear_img_spi`] module"]
#[doc(alias = "GEAR_IMG_SPI")]
pub type GearImgSpi = crate::Reg<gear_img_spi::GearImgSpiSpec>;
#[doc = "IMG SPI clock setting"]
pub mod gear_img_spi;
#[doc = "GEAR_PER_SDIO (rw) register accessor: SDIO clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_per_sdio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_per_sdio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gear_per_sdio`] module"]
#[doc(alias = "GEAR_PER_SDIO")]
pub type GearPerSdio = crate::Reg<gear_per_sdio::GearPerSdioSpec>;
#[doc = "SDIO clock setting"]
pub mod gear_per_sdio;
#[doc = "GEAR_PER_USB (rw) register accessor: USB clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_per_usb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_per_usb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gear_per_usb`] module"]
#[doc(alias = "GEAR_PER_USB")]
pub type GearPerUsb = crate::Reg<gear_per_usb::GearPerUsbSpec>;
#[doc = "USB clock setting"]
pub mod gear_per_usb;
#[doc = "GEAR_M_IMG_VENB (rw) register accessor: VENB_M clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_m_img_venb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_m_img_venb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gear_m_img_venb`] module"]
#[doc(alias = "GEAR_M_IMG_VENB")]
pub type GearMImgVenb = crate::Reg<gear_m_img_venb::GearMImgVenbSpec>;
#[doc = "VENB_M clock setting"]
pub mod gear_m_img_venb;
#[doc = "GEAR_N_IMG_VENB (rw) register accessor: VENB_N clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_n_img_venb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_n_img_venb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gear_n_img_venb`] module"]
#[doc(alias = "GEAR_N_IMG_VENB")]
pub type GearNImgVenb = crate::Reg<gear_n_img_venb::GearNImgVenbSpec>;
#[doc = "VENB_N clock setting"]
pub mod gear_n_img_venb;
#[doc = "GEAR_IMG_WSPI (rw) register accessor: IMG WSPI clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`gear_img_wspi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_img_wspi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gear_img_wspi`] module"]
#[doc(alias = "GEAR_IMG_WSPI")]
pub type GearImgWspi = crate::Reg<gear_img_wspi::GearImgWspiSpec>;
#[doc = "IMG WSPI clock setting"]
pub mod gear_img_wspi;
#[doc = "CKEN_EMMC (rw) register accessor: eMMC clock setting\n\nYou can [`read`](crate::Reg::read) this register and get [`cken_emmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cken_emmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cken_emmc`] module"]
#[doc(alias = "CKEN_EMMC")]
pub type CkenEmmc = crate::Reg<cken_emmc::CkenEmmcSpec>;
#[doc = "eMMC clock setting"]
pub mod cken_emmc;
#[doc = "RESET (rw) register accessor: Reset control\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`] module"]
#[doc(alias = "RESET")]
pub type Reset = crate::Reg<reset::ResetSpec>;
#[doc = "Reset control"]
pub mod reset;
#[doc = "CK_GATE_AHB (rw) register accessor: CPU/Peripheral clock gating control\n\nYou can [`read`](crate::Reg::read) this register and get [`ck_gate_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ck_gate_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ck_gate_ahb`] module"]
#[doc(alias = "CK_GATE_AHB")]
pub type CkGateAhb = crate::Reg<ck_gate_ahb::CkGateAhbSpec>;
#[doc = "CPU/Peripheral clock gating control"]
pub mod ck_gate_ahb;
