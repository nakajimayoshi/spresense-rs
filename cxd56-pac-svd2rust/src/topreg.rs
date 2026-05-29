#[repr(C)]
///Register block
pub struct RegisterBlock {
    pwd_ctl: PwdCtl,
    ana_pw_ctl: AnaPwCtl,
    ana_en_ctl: AnaEnCtl,
    _reserved3: [u8; 0x24],
    pmu_pw_ctl: PmuPwCtl,
    _reserved4: [u8; 0x2c],
    pwd_reset0: PwdReset0,
    _reserved5: [u8; 0x019c],
    pwd_stat: PwdStat,
    _reserved6: [u8; 0x04],
    ana_pw_stat: AnaPwStat,
    _reserved7: [u8; 0x0208],
    clseldiv_wake: ClseldivWake,
    ckdiv_cpu_dsp_bus_wake: CkdivCpuDspBusWake,
    cksel_root_wake: CkselRootWake,
    _reserved10: [u8; 0x34],
    pmu_wake_trig_en0: PmuWakeTrigEn0,
    pmu_wake_trig_en1: PmuWakeTrigEn1,
    pmu_wake_trig_negen0: PmuWakeTrigNegen0,
    pmu_wake_trig_negen1: PmuWakeTrigNegen1,
    pmu_wake_trig_noisecuten0: PmuWakeTrigNoisecuten0,
    pmu_wake_trig_cpuintsel0: PmuWakeTrigCpuintsel0,
    pmu_wake_trig_cpuintsel1: PmuWakeTrigCpuintsel1,
    pmu_wake_trig_cpuintsel2: PmuWakeTrigCpuintsel2,
    pmu_wake_trig_intdet0: PmuWakeTrigIntdet0,
    pmu_wake_trig_intdet1: PmuWakeTrigIntdet1,
    pmu_wake_trig_intdet2: PmuWakeTrigIntdet2,
    _reserved21: [u8; 0x40],
    pmu_core_cken: PmuCoreCken,
    cksel_root: CkselRoot,
    cksel_pmu: CkselPmu,
    cksel_sysiop: CkselSysiop,
    cksel_sysiop_sub: CkselSysiopSub,
    cksel_scu: CkselScu,
    ckdiv_cpu_dsp_bus: CkdivCpuDspBus,
    ckdiv_com: CkdivCom,
    ckdiv_hostifc: CkdivHostifc,
    ckdiv_scu: CkdivScu,
    ckdiv_pmu: CkdivPmu,
    crg_int_clr0: CrgIntClr0,
    crg_int_mask0: CrgIntMask0,
    crg_int_stat_msk0: CrgIntStatMsk0,
    crg_int_stat_raw0: CrgIntStatRaw0,
    crg_int_clr1: CrgIntClr1,
    crg_int_mask1: CrgIntMask1,
    crg_int_stat_msk1: CrgIntStatMsk1,
    crg_int_stat_raw1: CrgIntStatRaw1,
    cpu_gateclk: CpuGateclk,
    usbphy_cken: UsbphyCken,
    crg_mon: CrgMon,
    gear_stat: GearStat,
    _reserved44: [u8; 0x64],
    xosc_ctrl: XoscCtrl,
    xosc_ctrl2: XoscCtrl2,
    sys_pll_ctrl1: SysPllCtrl1,
    sys_pll_ctrl2: SysPllCtrl2,
    rcosc_ctrl1: RcoscCtrl1,
    rcosc_ctrl2: RcoscCtrl2,
    rf_gpmbi_en: RfGpmbiEn,
    _reserved51: [u8; 0xa8],
    force_cken: ForceCken,
    _reserved52: [u8; 0x18],
    ckgate_ctl: CkgateCtl,
    _reserved53: [u8; 0x9c],
    swreset_bus: SwresetBus,
    swreset_scu: SwresetScu,
    _reserved55: [u8; 0x08],
    busrom_cken: BusromCken,
    sysiop_cken: SysiopCken,
    _reserved57: [u8; 0x04],
    scu_cken: ScuCken,
    _reserved58: [u8; 0x10],
    rtc0_ctl: Rtc0Ctl,
    _reserved59: [u8; 0x7c],
    iocsys_intsel0: IocsysIntsel0,
    iocsys_intsel1: IocsysIntsel1,
    _reserved61: [u8; 0x08],
    iocsys_iomd0: IocsysIomd0,
    iocsys_iomd1: IocsysIomd1,
    _reserved63: [u8; 0x7c],
    io_spi0_cs_x: IoSpi0CsX,
    io_spi0_sck: IoSpi0Sck,
    _reserved65: [u8; 0x64],
    io_i2c0_bck: IoI2c0Bck,
    io_i2c0_bdt: IoI2c0Bdt,
    _reserved67: [u8; 0x54],
    io_uart2_txd: IoUart2Txd,
    io_uart2_rxd: IoUart2Rxd,
    _reserved69: [u8; 0x0b7c],
    iocapp_intsel0: IocappIntsel0,
    iocapp_intsel1: IocappIntsel1,
    _reserved71: [u8; 0x08],
    iocapp_iomd: IocappIomd,
    _reserved72: [u8; 0x0b5c],
    gp_i2c4_bck: GpI2c4Bck,
    _reserved73: [u8; 0x0164],
    gp_i2s1_bck: GpI2s1Bck,
    gp_i2s1_lrck: GpI2s1Lrck,
    gp_i2s1_data_in: GpI2s1DataIn,
    gp_i2s1_data_out: GpI2s1DataOut,
}
impl RegisterBlock {
    ///0x00 - Power-domain control (1 = powered on)
    #[inline(always)]
    pub const fn pwd_ctl(&self) -> &PwdCtl {
        &self.pwd_ctl
    }
    ///0x04 - Analog circuit power control (1 = powered on)
    #[inline(always)]
    pub const fn ana_pw_ctl(&self) -> &AnaPwCtl {
        &self.ana_pw_ctl
    }
    ///0x08 - Analog enable control (paired set/clear bits — use raw write)
    #[inline(always)]
    pub const fn ana_en_ctl(&self) -> &AnaEnCtl {
        &self.ana_en_ctl
    }
    ///0x30 - PMU power-supply control request (write-only kick register)
    #[inline(always)]
    pub const fn pmu_pw_ctl(&self) -> &PmuPwCtl {
        &self.pmu_pw_ctl
    }
    ///0x60 - Per-domain power reset control
    #[inline(always)]
    pub const fn pwd_reset0(&self) -> &PwdReset0 {
        &self.pwd_reset0
    }
    ///0x200 - Power-domain status (read-only mirror of PWD_CTL)
    #[inline(always)]
    pub const fn pwd_stat(&self) -> &PwdStat {
        &self.pwd_stat
    }
    ///0x208 - Analog power status (read-only mirror of ANA_PW_CTL)
    #[inline(always)]
    pub const fn ana_pw_stat(&self) -> &AnaPwStat {
        &self.ana_pw_stat
    }
    ///0x414 - Clock select/divider setting used on wake from sleep
    #[inline(always)]
    pub const fn clseldiv_wake(&self) -> &ClseldivWake {
        &self.clseldiv_wake
    }
    ///0x418 - CPU/DSP/bus clock divider used on wake from sleep
    #[inline(always)]
    pub const fn ckdiv_cpu_dsp_bus_wake(&self) -> &CkdivCpuDspBusWake {
        &self.ckdiv_cpu_dsp_bus_wake
    }
    ///0x41c - Root clock source select used on wake from sleep
    #[inline(always)]
    pub const fn cksel_root_wake(&self) -> &CkselRootWake {
        &self.cksel_root_wake
    }
    ///0x454 - Positive wake-trigger enable, slots 0–11
    #[inline(always)]
    pub const fn pmu_wake_trig_en0(&self) -> &PmuWakeTrigEn0 {
        &self.pmu_wake_trig_en0
    }
    ///0x458 - Positive wake-trigger enable, second bank
    #[inline(always)]
    pub const fn pmu_wake_trig_en1(&self) -> &PmuWakeTrigEn1 {
        &self.pmu_wake_trig_en1
    }
    ///0x45c - Negative wake-trigger enable, slots 0–11
    #[inline(always)]
    pub const fn pmu_wake_trig_negen0(&self) -> &PmuWakeTrigNegen0 {
        &self.pmu_wake_trig_negen0
    }
    ///0x460 - Negative wake-trigger enable, second bank
    #[inline(always)]
    pub const fn pmu_wake_trig_negen1(&self) -> &PmuWakeTrigNegen1 {
        &self.pmu_wake_trig_negen1
    }
    ///0x464 - Noise filter enable for interrupt slots 0–11
    #[inline(always)]
    pub const fn pmu_wake_trig_noisecuten0(&self) -> &PmuWakeTrigNoisecuten0 {
        &self.pmu_wake_trig_noisecuten0
    }
    ///0x468 - CPU interrupt route select, slots 0–3 (3-bit field per slot at 16+slot*4)
    #[inline(always)]
    pub const fn pmu_wake_trig_cpuintsel0(&self) -> &PmuWakeTrigCpuintsel0 {
        &self.pmu_wake_trig_cpuintsel0
    }
    ///0x46c - CPU interrupt route select, slots 4–11 (3-bit field per slot at slot*4)
    #[inline(always)]
    pub const fn pmu_wake_trig_cpuintsel1(&self) -> &PmuWakeTrigCpuintsel1 {
        &self.pmu_wake_trig_cpuintsel1
    }
    ///0x470 - CPU interrupt route select, second bank (unused by GPIO driver)
    #[inline(always)]
    pub const fn pmu_wake_trig_cpuintsel2(&self) -> &PmuWakeTrigCpuintsel2 {
        &self.pmu_wake_trig_cpuintsel2
    }
    ///0x474 - Interrupt detect polarity, slots 0–3 (3-bit field per slot at 16+slot*4)
    #[inline(always)]
    pub const fn pmu_wake_trig_intdet0(&self) -> &PmuWakeTrigIntdet0 {
        &self.pmu_wake_trig_intdet0
    }
    ///0x478 - Interrupt detect polarity, slots 4–11 (3-bit field per slot at slot*4)
    #[inline(always)]
    pub const fn pmu_wake_trig_intdet1(&self) -> &PmuWakeTrigIntdet1 {
        &self.pmu_wake_trig_intdet1
    }
    ///0x47c - Interrupt detect polarity, second bank (unused by GPIO driver)
    #[inline(always)]
    pub const fn pmu_wake_trig_intdet2(&self) -> &PmuWakeTrigIntdet2 {
        &self.pmu_wake_trig_intdet2
    }
    ///0x4c0 - PMU core clock enable
    #[inline(always)]
    pub const fn pmu_core_cken(&self) -> &PmuCoreCken {
        &self.pmu_core_cken
    }
    ///0x4c4 - Root clock source select and RTC status
    #[inline(always)]
    pub const fn cksel_root(&self) -> &CkselRoot {
        &self.cksel_root
    }
    ///0x4c8 - PMU clock source select
    #[inline(always)]
    pub const fn cksel_pmu(&self) -> &CkselPmu {
        &self.cksel_pmu
    }
    ///0x4cc - SYSIOP clock source select
    #[inline(always)]
    pub const fn cksel_sysiop(&self) -> &CkselSysiop {
        &self.cksel_sysiop
    }
    ///0x4d0 - SYSIOP sub-domain clock source select
    #[inline(always)]
    pub const fn cksel_sysiop_sub(&self) -> &CkselSysiopSub {
        &self.cksel_sysiop_sub
    }
    ///0x4d4 - SCU clock source select
    #[inline(always)]
    pub const fn cksel_scu(&self) -> &CkselScu {
        &self.cksel_scu
    }
    ///0x4d8 - CPU / DSP / bus clock divider
    #[inline(always)]
    pub const fn ckdiv_cpu_dsp_bus(&self) -> &CkdivCpuDspBus {
        &self.ckdiv_cpu_dsp_bus
    }
    ///0x4dc - COM clock divider
    #[inline(always)]
    pub const fn ckdiv_com(&self) -> &CkdivCom {
        &self.ckdiv_com
    }
    ///0x4e0 - Host interface clock divider
    #[inline(always)]
    pub const fn ckdiv_hostifc(&self) -> &CkdivHostifc {
        &self.ckdiv_hostifc
    }
    ///0x4e4 - SCU clock divider
    #[inline(always)]
    pub const fn ckdiv_scu(&self) -> &CkdivScu {
        &self.ckdiv_scu
    }
    ///0x4e8 - PMU clock divider
    #[inline(always)]
    pub const fn ckdiv_pmu(&self) -> &CkdivPmu {
        &self.ckdiv_pmu
    }
    ///0x4ec - TOPREG clock-ready interrupt clear 0 (write 1 to clear)
    #[inline(always)]
    pub const fn crg_int_clr0(&self) -> &CrgIntClr0 {
        &self.crg_int_clr0
    }
    ///0x4f0 - TOPREG clock-ready interrupt mask 0
    #[inline(always)]
    pub const fn crg_int_mask0(&self) -> &CrgIntMask0 {
        &self.crg_int_mask0
    }
    ///0x4f4 - TOPREG clock-ready masked interrupt status 0 (read-only)
    #[inline(always)]
    pub const fn crg_int_stat_msk0(&self) -> &CrgIntStatMsk0 {
        &self.crg_int_stat_msk0
    }
    ///0x4f8 - TOPREG clock-ready raw interrupt status 0 (read-only)
    #[inline(always)]
    pub const fn crg_int_stat_raw0(&self) -> &CrgIntStatRaw0 {
        &self.crg_int_stat_raw0
    }
    ///0x4fc - TOPREG clock-ready interrupt clear 1 (write 1 to clear)
    #[inline(always)]
    pub const fn crg_int_clr1(&self) -> &CrgIntClr1 {
        &self.crg_int_clr1
    }
    ///0x500 - TOPREG clock-ready interrupt mask 1
    #[inline(always)]
    pub const fn crg_int_mask1(&self) -> &CrgIntMask1 {
        &self.crg_int_mask1
    }
    ///0x504 - TOPREG clock-ready masked interrupt status 1 (read-only)
    #[inline(always)]
    pub const fn crg_int_stat_msk1(&self) -> &CrgIntStatMsk1 {
        &self.crg_int_stat_msk1
    }
    ///0x508 - TOPREG clock-ready raw interrupt status 1 (read-only)
    #[inline(always)]
    pub const fn crg_int_stat_raw1(&self) -> &CrgIntStatRaw1 {
        &self.crg_int_stat_raw1
    }
    ///0x50c - CPU clock gating control
    #[inline(always)]
    pub const fn cpu_gateclk(&self) -> &CpuGateclk {
        &self.cpu_gateclk
    }
    ///0x510 - USB PHY clock enable
    #[inline(always)]
    pub const fn usbphy_cken(&self) -> &UsbphyCken {
        &self.usbphy_cken
    }
    ///0x514 - CRG monitor status (read-only)
    #[inline(always)]
    pub const fn crg_mon(&self) -> &CrgMon {
        &self.crg_mon
    }
    ///0x518 - Clock gear status (read-only)
    #[inline(always)]
    pub const fn gear_stat(&self) -> &GearStat {
        &self.gear_stat
    }
    ///0x580 - Crystal oscillator control
    #[inline(always)]
    pub const fn xosc_ctrl(&self) -> &XoscCtrl {
        &self.xosc_ctrl
    }
    ///0x584 - Crystal oscillator control 2
    #[inline(always)]
    pub const fn xosc_ctrl2(&self) -> &XoscCtrl2 {
        &self.xosc_ctrl2
    }
    ///0x588 - System PLL control 1
    #[inline(always)]
    pub const fn sys_pll_ctrl1(&self) -> &SysPllCtrl1 {
        &self.sys_pll_ctrl1
    }
    ///0x58c - System PLL control 2 (division ratios)
    #[inline(always)]
    pub const fn sys_pll_ctrl2(&self) -> &SysPllCtrl2 {
        &self.sys_pll_ctrl2
    }
    ///0x590 - RC oscillator control 1
    #[inline(always)]
    pub const fn rcosc_ctrl1(&self) -> &RcoscCtrl1 {
        &self.rcosc_ctrl1
    }
    ///0x594 - RC oscillator control 2
    #[inline(always)]
    pub const fn rcosc_ctrl2(&self) -> &RcoscCtrl2 {
        &self.rcosc_ctrl2
    }
    ///0x598 - RF GP MBI enable
    #[inline(always)]
    pub const fn rf_gpmbi_en(&self) -> &RfGpmbiEn {
        &self.rf_gpmbi_en
    }
    ///0x644 - Force clock enable (overrides gating)
    #[inline(always)]
    pub const fn force_cken(&self) -> &ForceCken {
        &self.force_cken
    }
    ///0x660 - Clock gate control
    #[inline(always)]
    pub const fn ckgate_ctl(&self) -> &CkgateCtl {
        &self.ckgate_ctl
    }
    ///0x700 - Bus peripheral software reset (0 = held in reset, 1 = released)
    #[inline(always)]
    pub const fn swreset_bus(&self) -> &SwresetBus {
        &self.swreset_bus
    }
    ///0x704 - SCU peripheral software reset (0 = held in reset, 1 = released)
    #[inline(always)]
    pub const fn swreset_scu(&self) -> &SwresetScu {
        &self.swreset_scu
    }
    ///0x710 - Bus ROM clock enable
    #[inline(always)]
    pub const fn busrom_cken(&self) -> &BusromCken {
        &self.busrom_cken
    }
    ///0x714 - SYSIOP peripheral clock enables
    #[inline(always)]
    pub const fn sysiop_cken(&self) -> &SysiopCken {
        &self.sysiop_cken
    }
    ///0x71c - SCU peripheral clock enables
    #[inline(always)]
    pub const fn scu_cken(&self) -> &ScuCken {
        &self.scu_cken
    }
    ///0x730 - RTC0 control
    #[inline(always)]
    pub const fn rtc0_ctl(&self) -> &Rtc0Ctl {
        &self.rtc0_ctl
    }
    ///0x7b0 - SYS-domain GPIO interrupt slot mux, slots 0–3 (1 byte per slot, pin index 0–63)
    #[inline(always)]
    pub const fn iocsys_intsel0(&self) -> &IocsysIntsel0 {
        &self.iocsys_intsel0
    }
    ///0x7b4 - SYS-domain GPIO interrupt slot mux, slots 4–5
    #[inline(always)]
    pub const fn iocsys_intsel1(&self) -> &IocsysIntsel1 {
        &self.iocsys_intsel1
    }
    ///0x7c0 - SYSIOP IO-cell mode-mux register 0
    #[inline(always)]
    pub const fn iocsys_iomd0(&self) -> &IocsysIomd0 {
        &self.iocsys_iomd0
    }
    ///0x7c4 - SYSIOP IO-cell mode-mux register 1
    #[inline(always)]
    pub const fn iocsys_iomd1(&self) -> &IocsysIomd1 {
        &self.iocsys_iomd1
    }
    ///0x844 - IOCELL control for SPI0_CS_X / UART1-TXD
    #[inline(always)]
    pub const fn io_spi0_cs_x(&self) -> &IoSpi0CsX {
        &self.io_spi0_cs_x
    }
    ///0x848 - IOCELL control for SPI0_SCK / UART1-RXD
    #[inline(always)]
    pub const fn io_spi0_sck(&self) -> &IoSpi0Sck {
        &self.io_spi0_sck
    }
    ///0x8b0 - IOCELL control for I2C0_BCK pad
    #[inline(always)]
    pub const fn io_i2c0_bck(&self) -> &IoI2c0Bck {
        &self.io_i2c0_bck
    }
    ///0x8b4 - IOCELL control for I2C0_BDT pad
    #[inline(always)]
    pub const fn io_i2c0_bdt(&self) -> &IoI2c0Bdt {
        &self.io_i2c0_bdt
    }
    ///0x90c - IOCELL control for UART2 TXD pin
    #[inline(always)]
    pub const fn io_uart2_txd(&self) -> &IoUart2Txd {
        &self.io_uart2_txd
    }
    ///0x910 - IOCELL control for UART2 RXD pin
    #[inline(always)]
    pub const fn io_uart2_rxd(&self) -> &IoUart2Rxd {
        &self.io_uart2_rxd
    }
    ///0x1490 - APP-domain GPIO interrupt slot mux, slots 6–9 (1 byte per slot, pin index 0–63)
    #[inline(always)]
    pub const fn iocapp_intsel0(&self) -> &IocappIntsel0 {
        &self.iocapp_intsel0
    }
    ///0x1494 - APP-domain GPIO interrupt slot mux, slots 10–11
    #[inline(always)]
    pub const fn iocapp_intsel1(&self) -> &IocappIntsel1 {
        &self.iocapp_intsel1
    }
    ///0x14a0 - APP-domain IO-cell mode-mux register
    #[inline(always)]
    pub const fn iocapp_iomd(&self) -> &IocappIomd {
        &self.iocapp_iomd
    }
    ///0x2000 - GPIO SYS pin 1 — I2C4 clock / Arduino D14
    #[inline(always)]
    pub const fn gp_i2c4_bck(&self) -> &GpI2c4Bck {
        &self.gp_i2c4_bck
    }
    ///0x2168 - GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board
    #[inline(always)]
    pub const fn gp_i2s1_bck(&self) -> &GpI2s1Bck {
        &self.gp_i2s1_bck
    }
    ///0x216c - GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board
    #[inline(always)]
    pub const fn gp_i2s1_lrck(&self) -> &GpI2s1Lrck {
        &self.gp_i2s1_lrck
    }
    ///0x2170 - GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board
    #[inline(always)]
    pub const fn gp_i2s1_data_in(&self) -> &GpI2s1DataIn {
        &self.gp_i2s1_data_in
    }
    ///0x2174 - GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board
    #[inline(always)]
    pub const fn gp_i2s1_data_out(&self) -> &GpI2s1DataOut {
        &self.gp_i2s1_data_out
    }
}
/**PWD_CTL (rw) register accessor: Power-domain control (1 = powered on)

You can [`read`](crate::Reg::read) this register and get [`pwd_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwd_ctl`] module*/
#[doc(alias = "PWD_CTL")]
pub type PwdCtl = crate::Reg<pwd_ctl::PwdCtlSpec>;
///Power-domain control (1 = powered on)
pub mod pwd_ctl;
/**ANA_PW_CTL (rw) register accessor: Analog circuit power control (1 = powered on)

You can [`read`](crate::Reg::read) this register and get [`ana_pw_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_pw_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ana_pw_ctl`] module*/
#[doc(alias = "ANA_PW_CTL")]
pub type AnaPwCtl = crate::Reg<ana_pw_ctl::AnaPwCtlSpec>;
///Analog circuit power control (1 = powered on)
pub mod ana_pw_ctl;
/**ANA_EN_CTL (rw) register accessor: Analog enable control (paired set/clear bits — use raw write)

You can [`read`](crate::Reg::read) this register and get [`ana_en_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_en_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ana_en_ctl`] module*/
#[doc(alias = "ANA_EN_CTL")]
pub type AnaEnCtl = crate::Reg<ana_en_ctl::AnaEnCtlSpec>;
///Analog enable control (paired set/clear bits — use raw write)
pub mod ana_en_ctl;
/**PWD_RESET0 (rw) register accessor: Per-domain power reset control

You can [`read`](crate::Reg::read) this register and get [`pwd_reset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwd_reset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwd_reset0`] module*/
#[doc(alias = "PWD_RESET0")]
pub type PwdReset0 = crate::Reg<pwd_reset0::PwdReset0Spec>;
///Per-domain power reset control
pub mod pwd_reset0;
/**PWD_STAT (r) register accessor: Power-domain status (read-only mirror of PWD_CTL)

You can [`read`](crate::Reg::read) this register and get [`pwd_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwd_stat`] module*/
#[doc(alias = "PWD_STAT")]
pub type PwdStat = crate::Reg<pwd_stat::PwdStatSpec>;
///Power-domain status (read-only mirror of PWD_CTL)
pub mod pwd_stat;
/**ANA_PW_STAT (r) register accessor: Analog power status (read-only mirror of ANA_PW_CTL)

You can [`read`](crate::Reg::read) this register and get [`ana_pw_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ana_pw_stat`] module*/
#[doc(alias = "ANA_PW_STAT")]
pub type AnaPwStat = crate::Reg<ana_pw_stat::AnaPwStatSpec>;
///Analog power status (read-only mirror of ANA_PW_CTL)
pub mod ana_pw_stat;
/**CLSELDIV_WAKE (rw) register accessor: Clock select/divider setting used on wake from sleep

You can [`read`](crate::Reg::read) this register and get [`clseldiv_wake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clseldiv_wake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clseldiv_wake`] module*/
#[doc(alias = "CLSELDIV_WAKE")]
pub type ClseldivWake = crate::Reg<clseldiv_wake::ClseldivWakeSpec>;
///Clock select/divider setting used on wake from sleep
pub mod clseldiv_wake;
/**CKDIV_CPU_DSP_BUS_WAKE (rw) register accessor: CPU/DSP/bus clock divider used on wake from sleep

You can [`read`](crate::Reg::read) this register and get [`ckdiv_cpu_dsp_bus_wake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_cpu_dsp_bus_wake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckdiv_cpu_dsp_bus_wake`] module*/
#[doc(alias = "CKDIV_CPU_DSP_BUS_WAKE")]
pub type CkdivCpuDspBusWake = crate::Reg<ckdiv_cpu_dsp_bus_wake::CkdivCpuDspBusWakeSpec>;
///CPU/DSP/bus clock divider used on wake from sleep
pub mod ckdiv_cpu_dsp_bus_wake;
/**CKSEL_ROOT_WAKE (rw) register accessor: Root clock source select used on wake from sleep

You can [`read`](crate::Reg::read) this register and get [`cksel_root_wake::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_root_wake::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cksel_root_wake`] module*/
#[doc(alias = "CKSEL_ROOT_WAKE")]
pub type CkselRootWake = crate::Reg<cksel_root_wake::CkselRootWakeSpec>;
///Root clock source select used on wake from sleep
pub mod cksel_root_wake;
/**PMU_WAKE_TRIG_EN0 (rw) register accessor: Positive wake-trigger enable, slots 0–11

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_en0`] module*/
#[doc(alias = "PMU_WAKE_TRIG_EN0")]
pub type PmuWakeTrigEn0 = crate::Reg<pmu_wake_trig_en0::PmuWakeTrigEn0Spec>;
///Positive wake-trigger enable, slots 0–11
pub mod pmu_wake_trig_en0;
/**PMU_WAKE_TRIG_EN1 (rw) register accessor: Positive wake-trigger enable, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_en1`] module*/
#[doc(alias = "PMU_WAKE_TRIG_EN1")]
pub type PmuWakeTrigEn1 = crate::Reg<pmu_wake_trig_en1::PmuWakeTrigEn1Spec>;
///Positive wake-trigger enable, second bank
pub mod pmu_wake_trig_en1;
/**PMU_WAKE_TRIG_NEGEN0 (rw) register accessor: Negative wake-trigger enable, slots 0–11

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_negen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_negen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_negen0`] module*/
#[doc(alias = "PMU_WAKE_TRIG_NEGEN0")]
pub type PmuWakeTrigNegen0 = crate::Reg<pmu_wake_trig_negen0::PmuWakeTrigNegen0Spec>;
///Negative wake-trigger enable, slots 0–11
pub mod pmu_wake_trig_negen0;
/**PMU_WAKE_TRIG_NEGEN1 (rw) register accessor: Negative wake-trigger enable, second bank

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_negen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_negen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_negen1`] module*/
#[doc(alias = "PMU_WAKE_TRIG_NEGEN1")]
pub type PmuWakeTrigNegen1 = crate::Reg<pmu_wake_trig_negen1::PmuWakeTrigNegen1Spec>;
///Negative wake-trigger enable, second bank
pub mod pmu_wake_trig_negen1;
/**PMU_WAKE_TRIG_NOISECUTEN0 (rw) register accessor: Noise filter enable for interrupt slots 0–11

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_noisecuten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_noisecuten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_noisecuten0`] module*/
#[doc(alias = "PMU_WAKE_TRIG_NOISECUTEN0")]
pub type PmuWakeTrigNoisecuten0 = crate::Reg<pmu_wake_trig_noisecuten0::PmuWakeTrigNoisecuten0Spec>;
///Noise filter enable for interrupt slots 0–11
pub mod pmu_wake_trig_noisecuten0;
/**PMU_WAKE_TRIG_CPUINTSEL0 (rw) register accessor: CPU interrupt route select, slots 0–3 (3-bit field per slot at 16+slot*4)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_cpuintsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_cpuintsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_cpuintsel0`] module*/
#[doc(alias = "PMU_WAKE_TRIG_CPUINTSEL0")]
pub type PmuWakeTrigCpuintsel0 = crate::Reg<pmu_wake_trig_cpuintsel0::PmuWakeTrigCpuintsel0Spec>;
///CPU interrupt route select, slots 0–3 (3-bit field per slot at 16+slot*4)
pub mod pmu_wake_trig_cpuintsel0;
/**PMU_WAKE_TRIG_CPUINTSEL1 (rw) register accessor: CPU interrupt route select, slots 4–11 (3-bit field per slot at slot*4)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_cpuintsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_cpuintsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_cpuintsel1`] module*/
#[doc(alias = "PMU_WAKE_TRIG_CPUINTSEL1")]
pub type PmuWakeTrigCpuintsel1 = crate::Reg<pmu_wake_trig_cpuintsel1::PmuWakeTrigCpuintsel1Spec>;
///CPU interrupt route select, slots 4–11 (3-bit field per slot at slot*4)
pub mod pmu_wake_trig_cpuintsel1;
/**PMU_WAKE_TRIG_CPUINTSEL2 (rw) register accessor: CPU interrupt route select, second bank (unused by GPIO driver)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_cpuintsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_cpuintsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_cpuintsel2`] module*/
#[doc(alias = "PMU_WAKE_TRIG_CPUINTSEL2")]
pub type PmuWakeTrigCpuintsel2 = crate::Reg<pmu_wake_trig_cpuintsel2::PmuWakeTrigCpuintsel2Spec>;
///CPU interrupt route select, second bank (unused by GPIO driver)
pub mod pmu_wake_trig_cpuintsel2;
/**PMU_WAKE_TRIG_INTDET0 (rw) register accessor: Interrupt detect polarity, slots 0–3 (3-bit field per slot at 16+slot*4)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_intdet0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_intdet0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_intdet0`] module*/
#[doc(alias = "PMU_WAKE_TRIG_INTDET0")]
pub type PmuWakeTrigIntdet0 = crate::Reg<pmu_wake_trig_intdet0::PmuWakeTrigIntdet0Spec>;
///Interrupt detect polarity, slots 0–3 (3-bit field per slot at 16+slot*4)
pub mod pmu_wake_trig_intdet0;
/**PMU_WAKE_TRIG_INTDET1 (rw) register accessor: Interrupt detect polarity, slots 4–11 (3-bit field per slot at slot*4)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_intdet1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_intdet1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_intdet1`] module*/
#[doc(alias = "PMU_WAKE_TRIG_INTDET1")]
pub type PmuWakeTrigIntdet1 = crate::Reg<pmu_wake_trig_intdet1::PmuWakeTrigIntdet1Spec>;
///Interrupt detect polarity, slots 4–11 (3-bit field per slot at slot*4)
pub mod pmu_wake_trig_intdet1;
/**PMU_WAKE_TRIG_INTDET2 (rw) register accessor: Interrupt detect polarity, second bank (unused by GPIO driver)

You can [`read`](crate::Reg::read) this register and get [`pmu_wake_trig_intdet2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_wake_trig_intdet2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_wake_trig_intdet2`] module*/
#[doc(alias = "PMU_WAKE_TRIG_INTDET2")]
pub type PmuWakeTrigIntdet2 = crate::Reg<pmu_wake_trig_intdet2::PmuWakeTrigIntdet2Spec>;
///Interrupt detect polarity, second bank (unused by GPIO driver)
pub mod pmu_wake_trig_intdet2;
/**PMU_CORE_CKEN (rw) register accessor: PMU core clock enable

You can [`read`](crate::Reg::read) this register and get [`pmu_core_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_core_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_core_cken`] module*/
#[doc(alias = "PMU_CORE_CKEN")]
pub type PmuCoreCken = crate::Reg<pmu_core_cken::PmuCoreCkenSpec>;
///PMU core clock enable
pub mod pmu_core_cken;
/**CKSEL_ROOT (rw) register accessor: Root clock source select and RTC status

You can [`read`](crate::Reg::read) this register and get [`cksel_root::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_root::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cksel_root`] module*/
#[doc(alias = "CKSEL_ROOT")]
pub type CkselRoot = crate::Reg<cksel_root::CkselRootSpec>;
///Root clock source select and RTC status
pub mod cksel_root;
/**CKSEL_PMU (rw) register accessor: PMU clock source select

You can [`read`](crate::Reg::read) this register and get [`cksel_pmu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_pmu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cksel_pmu`] module*/
#[doc(alias = "CKSEL_PMU")]
pub type CkselPmu = crate::Reg<cksel_pmu::CkselPmuSpec>;
///PMU clock source select
pub mod cksel_pmu;
/**CKSEL_SYSIOP (rw) register accessor: SYSIOP clock source select

You can [`read`](crate::Reg::read) this register and get [`cksel_sysiop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_sysiop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cksel_sysiop`] module*/
#[doc(alias = "CKSEL_SYSIOP")]
pub type CkselSysiop = crate::Reg<cksel_sysiop::CkselSysiopSpec>;
///SYSIOP clock source select
pub mod cksel_sysiop;
/**CKSEL_SYSIOP_SUB (rw) register accessor: SYSIOP sub-domain clock source select

You can [`read`](crate::Reg::read) this register and get [`cksel_sysiop_sub::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_sysiop_sub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cksel_sysiop_sub`] module*/
#[doc(alias = "CKSEL_SYSIOP_SUB")]
pub type CkselSysiopSub = crate::Reg<cksel_sysiop_sub::CkselSysiopSubSpec>;
///SYSIOP sub-domain clock source select
pub mod cksel_sysiop_sub;
/**CKSEL_SCU (rw) register accessor: SCU clock source select

You can [`read`](crate::Reg::read) this register and get [`cksel_scu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksel_scu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cksel_scu`] module*/
#[doc(alias = "CKSEL_SCU")]
pub type CkselScu = crate::Reg<cksel_scu::CkselScuSpec>;
///SCU clock source select
pub mod cksel_scu;
/**CKDIV_CPU_DSP_BUS (rw) register accessor: CPU / DSP / bus clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_cpu_dsp_bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_cpu_dsp_bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckdiv_cpu_dsp_bus`] module*/
#[doc(alias = "CKDIV_CPU_DSP_BUS")]
pub type CkdivCpuDspBus = crate::Reg<ckdiv_cpu_dsp_bus::CkdivCpuDspBusSpec>;
///CPU / DSP / bus clock divider
pub mod ckdiv_cpu_dsp_bus;
/**CKDIV_COM (rw) register accessor: COM clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_com::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_com::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckdiv_com`] module*/
#[doc(alias = "CKDIV_COM")]
pub type CkdivCom = crate::Reg<ckdiv_com::CkdivComSpec>;
///COM clock divider
pub mod ckdiv_com;
/**CKDIV_HOSTIFC (rw) register accessor: Host interface clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_hostifc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_hostifc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckdiv_hostifc`] module*/
#[doc(alias = "CKDIV_HOSTIFC")]
pub type CkdivHostifc = crate::Reg<ckdiv_hostifc::CkdivHostifcSpec>;
///Host interface clock divider
pub mod ckdiv_hostifc;
/**CKDIV_SCU (rw) register accessor: SCU clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_scu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_scu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckdiv_scu`] module*/
#[doc(alias = "CKDIV_SCU")]
pub type CkdivScu = crate::Reg<ckdiv_scu::CkdivScuSpec>;
///SCU clock divider
pub mod ckdiv_scu;
/**CKDIV_PMU (rw) register accessor: PMU clock divider

You can [`read`](crate::Reg::read) this register and get [`ckdiv_pmu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv_pmu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckdiv_pmu`] module*/
#[doc(alias = "CKDIV_PMU")]
pub type CkdivPmu = crate::Reg<ckdiv_pmu::CkdivPmuSpec>;
///PMU clock divider
pub mod ckdiv_pmu;
/**CRG_INT_CLR0 (rw) register accessor: TOPREG clock-ready interrupt clear 0 (write 1 to clear)

You can [`read`](crate::Reg::read) this register and get [`crg_int_clr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crg_int_clr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_int_clr0`] module*/
#[doc(alias = "CRG_INT_CLR0")]
pub type CrgIntClr0 = crate::Reg<crg_int_clr0::CrgIntClr0Spec>;
///TOPREG clock-ready interrupt clear 0 (write 1 to clear)
pub mod crg_int_clr0;
/**CRG_INT_MASK0 (rw) register accessor: TOPREG clock-ready interrupt mask 0

You can [`read`](crate::Reg::read) this register and get [`crg_int_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crg_int_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_int_mask0`] module*/
#[doc(alias = "CRG_INT_MASK0")]
pub type CrgIntMask0 = crate::Reg<crg_int_mask0::CrgIntMask0Spec>;
///TOPREG clock-ready interrupt mask 0
pub mod crg_int_mask0;
/**CRG_INT_STAT_MSK0 (r) register accessor: TOPREG clock-ready masked interrupt status 0 (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_int_stat_msk0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_int_stat_msk0`] module*/
#[doc(alias = "CRG_INT_STAT_MSK0")]
pub type CrgIntStatMsk0 = crate::Reg<crg_int_stat_msk0::CrgIntStatMsk0Spec>;
///TOPREG clock-ready masked interrupt status 0 (read-only)
pub mod crg_int_stat_msk0;
/**CRG_INT_STAT_RAW0 (r) register accessor: TOPREG clock-ready raw interrupt status 0 (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_int_stat_raw0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_int_stat_raw0`] module*/
#[doc(alias = "CRG_INT_STAT_RAW0")]
pub type CrgIntStatRaw0 = crate::Reg<crg_int_stat_raw0::CrgIntStatRaw0Spec>;
///TOPREG clock-ready raw interrupt status 0 (read-only)
pub mod crg_int_stat_raw0;
/**CRG_INT_CLR1 (rw) register accessor: TOPREG clock-ready interrupt clear 1 (write 1 to clear)

You can [`read`](crate::Reg::read) this register and get [`crg_int_clr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crg_int_clr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_int_clr1`] module*/
#[doc(alias = "CRG_INT_CLR1")]
pub type CrgIntClr1 = crate::Reg<crg_int_clr1::CrgIntClr1Spec>;
///TOPREG clock-ready interrupt clear 1 (write 1 to clear)
pub mod crg_int_clr1;
/**CRG_INT_MASK1 (rw) register accessor: TOPREG clock-ready interrupt mask 1

You can [`read`](crate::Reg::read) this register and get [`crg_int_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crg_int_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_int_mask1`] module*/
#[doc(alias = "CRG_INT_MASK1")]
pub type CrgIntMask1 = crate::Reg<crg_int_mask1::CrgIntMask1Spec>;
///TOPREG clock-ready interrupt mask 1
pub mod crg_int_mask1;
/**CRG_INT_STAT_MSK1 (r) register accessor: TOPREG clock-ready masked interrupt status 1 (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_int_stat_msk1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_int_stat_msk1`] module*/
#[doc(alias = "CRG_INT_STAT_MSK1")]
pub type CrgIntStatMsk1 = crate::Reg<crg_int_stat_msk1::CrgIntStatMsk1Spec>;
///TOPREG clock-ready masked interrupt status 1 (read-only)
pub mod crg_int_stat_msk1;
/**CRG_INT_STAT_RAW1 (r) register accessor: TOPREG clock-ready raw interrupt status 1 (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_int_stat_raw1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_int_stat_raw1`] module*/
#[doc(alias = "CRG_INT_STAT_RAW1")]
pub type CrgIntStatRaw1 = crate::Reg<crg_int_stat_raw1::CrgIntStatRaw1Spec>;
///TOPREG clock-ready raw interrupt status 1 (read-only)
pub mod crg_int_stat_raw1;
/**CPU_GATECLK (rw) register accessor: CPU clock gating control

You can [`read`](crate::Reg::read) this register and get [`cpu_gateclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_gateclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_gateclk`] module*/
#[doc(alias = "CPU_GATECLK")]
pub type CpuGateclk = crate::Reg<cpu_gateclk::CpuGateclkSpec>;
///CPU clock gating control
pub mod cpu_gateclk;
/**USBPHY_CKEN (rw) register accessor: USB PHY clock enable

You can [`read`](crate::Reg::read) this register and get [`usbphy_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbphy_cken`] module*/
#[doc(alias = "USBPHY_CKEN")]
pub type UsbphyCken = crate::Reg<usbphy_cken::UsbphyCkenSpec>;
///USB PHY clock enable
pub mod usbphy_cken;
/**CRG_MON (r) register accessor: CRG monitor status (read-only)

You can [`read`](crate::Reg::read) this register and get [`crg_mon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crg_mon`] module*/
#[doc(alias = "CRG_MON")]
pub type CrgMon = crate::Reg<crg_mon::CrgMonSpec>;
///CRG monitor status (read-only)
pub mod crg_mon;
/**GEAR_STAT (r) register accessor: Clock gear status (read-only)

You can [`read`](crate::Reg::read) this register and get [`gear_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gear_stat`] module*/
#[doc(alias = "GEAR_STAT")]
pub type GearStat = crate::Reg<gear_stat::GearStatSpec>;
///Clock gear status (read-only)
pub mod gear_stat;
/**XOSC_CTRL (rw) register accessor: Crystal oscillator control

You can [`read`](crate::Reg::read) this register and get [`xosc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xosc_ctrl`] module*/
#[doc(alias = "XOSC_CTRL")]
pub type XoscCtrl = crate::Reg<xosc_ctrl::XoscCtrlSpec>;
///Crystal oscillator control
pub mod xosc_ctrl;
/**XOSC_CTRL2 (rw) register accessor: Crystal oscillator control 2

You can [`read`](crate::Reg::read) this register and get [`xosc_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xosc_ctrl2`] module*/
#[doc(alias = "XOSC_CTRL2")]
pub type XoscCtrl2 = crate::Reg<xosc_ctrl2::XoscCtrl2Spec>;
///Crystal oscillator control 2
pub mod xosc_ctrl2;
/**SYS_PLL_CTRL1 (rw) register accessor: System PLL control 1

You can [`read`](crate::Reg::read) this register and get [`sys_pll_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_pll_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sys_pll_ctrl1`] module*/
#[doc(alias = "SYS_PLL_CTRL1")]
pub type SysPllCtrl1 = crate::Reg<sys_pll_ctrl1::SysPllCtrl1Spec>;
///System PLL control 1
pub mod sys_pll_ctrl1;
/**SYS_PLL_CTRL2 (rw) register accessor: System PLL control 2 (division ratios)

You can [`read`](crate::Reg::read) this register and get [`sys_pll_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_pll_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sys_pll_ctrl2`] module*/
#[doc(alias = "SYS_PLL_CTRL2")]
pub type SysPllCtrl2 = crate::Reg<sys_pll_ctrl2::SysPllCtrl2Spec>;
///System PLL control 2 (division ratios)
pub mod sys_pll_ctrl2;
/**RCOSC_CTRL1 (rw) register accessor: RC oscillator control 1

You can [`read`](crate::Reg::read) this register and get [`rcosc_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcosc_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rcosc_ctrl1`] module*/
#[doc(alias = "RCOSC_CTRL1")]
pub type RcoscCtrl1 = crate::Reg<rcosc_ctrl1::RcoscCtrl1Spec>;
///RC oscillator control 1
pub mod rcosc_ctrl1;
/**RCOSC_CTRL2 (rw) register accessor: RC oscillator control 2

You can [`read`](crate::Reg::read) this register and get [`rcosc_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcosc_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rcosc_ctrl2`] module*/
#[doc(alias = "RCOSC_CTRL2")]
pub type RcoscCtrl2 = crate::Reg<rcosc_ctrl2::RcoscCtrl2Spec>;
///RC oscillator control 2
pub mod rcosc_ctrl2;
/**RF_GPMBI_EN (rw) register accessor: RF GP MBI enable

You can [`read`](crate::Reg::read) this register and get [`rf_gpmbi_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_gpmbi_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rf_gpmbi_en`] module*/
#[doc(alias = "RF_GPMBI_EN")]
pub type RfGpmbiEn = crate::Reg<rf_gpmbi_en::RfGpmbiEnSpec>;
///RF GP MBI enable
pub mod rf_gpmbi_en;
/**FORCE_CKEN (rw) register accessor: Force clock enable (overrides gating)

You can [`read`](crate::Reg::read) this register and get [`force_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@force_cken`] module*/
#[doc(alias = "FORCE_CKEN")]
pub type ForceCken = crate::Reg<force_cken::ForceCkenSpec>;
///Force clock enable (overrides gating)
pub mod force_cken;
/**CKGATE_CTL (rw) register accessor: Clock gate control

You can [`read`](crate::Reg::read) this register and get [`ckgate_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgate_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckgate_ctl`] module*/
#[doc(alias = "CKGATE_CTL")]
pub type CkgateCtl = crate::Reg<ckgate_ctl::CkgateCtlSpec>;
///Clock gate control
pub mod ckgate_ctl;
/**SWRESET_BUS (rw) register accessor: Bus peripheral software reset (0 = held in reset, 1 = released)

You can [`read`](crate::Reg::read) this register and get [`swreset_bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset_bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swreset_bus`] module*/
#[doc(alias = "SWRESET_BUS")]
pub type SwresetBus = crate::Reg<swreset_bus::SwresetBusSpec>;
///Bus peripheral software reset (0 = held in reset, 1 = released)
pub mod swreset_bus;
/**SWRESET_SCU (rw) register accessor: SCU peripheral software reset (0 = held in reset, 1 = released)

You can [`read`](crate::Reg::read) this register and get [`swreset_scu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset_scu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swreset_scu`] module*/
#[doc(alias = "SWRESET_SCU")]
pub type SwresetScu = crate::Reg<swreset_scu::SwresetScuSpec>;
///SCU peripheral software reset (0 = held in reset, 1 = released)
pub mod swreset_scu;
/**BUSROM_CKEN (rw) register accessor: Bus ROM clock enable

You can [`read`](crate::Reg::read) this register and get [`busrom_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busrom_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busrom_cken`] module*/
#[doc(alias = "BUSROM_CKEN")]
pub type BusromCken = crate::Reg<busrom_cken::BusromCkenSpec>;
///Bus ROM clock enable
pub mod busrom_cken;
/**SYSIOP_CKEN (rw) register accessor: SYSIOP peripheral clock enables

You can [`read`](crate::Reg::read) this register and get [`sysiop_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysiop_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sysiop_cken`] module*/
#[doc(alias = "SYSIOP_CKEN")]
pub type SysiopCken = crate::Reg<sysiop_cken::SysiopCkenSpec>;
///SYSIOP peripheral clock enables
pub mod sysiop_cken;
/**SCU_CKEN (rw) register accessor: SCU peripheral clock enables

You can [`read`](crate::Reg::read) this register and get [`scu_cken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu_cken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scu_cken`] module*/
#[doc(alias = "SCU_CKEN")]
pub type ScuCken = crate::Reg<scu_cken::ScuCkenSpec>;
///SCU peripheral clock enables
pub mod scu_cken;
/**RTC0_CTL (rw) register accessor: RTC0 control

You can [`read`](crate::Reg::read) this register and get [`rtc0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc0_ctl`] module*/
#[doc(alias = "RTC0_CTL")]
pub type Rtc0Ctl = crate::Reg<rtc0_ctl::Rtc0CtlSpec>;
///RTC0 control
pub mod rtc0_ctl;
/**PMU_PW_CTL (w) register accessor: PMU power-supply control request (write-only kick register)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_pw_ctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmu_pw_ctl`] module*/
#[doc(alias = "PMU_PW_CTL")]
pub type PmuPwCtl = crate::Reg<pmu_pw_ctl::PmuPwCtlSpec>;
///PMU power-supply control request (write-only kick register)
pub mod pmu_pw_ctl;
/**IOCSYS_INTSEL0 (rw) register accessor: SYS-domain GPIO interrupt slot mux, slots 0–3 (1 byte per slot, pin index 0–63)

You can [`read`](crate::Reg::read) this register and get [`iocsys_intsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocsys_intsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iocsys_intsel0`] module*/
#[doc(alias = "IOCSYS_INTSEL0")]
pub type IocsysIntsel0 = crate::Reg<iocsys_intsel0::IocsysIntsel0Spec>;
///SYS-domain GPIO interrupt slot mux, slots 0–3 (1 byte per slot, pin index 0–63)
pub mod iocsys_intsel0;
/**IOCSYS_INTSEL1 (rw) register accessor: SYS-domain GPIO interrupt slot mux, slots 4–5

You can [`read`](crate::Reg::read) this register and get [`iocsys_intsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocsys_intsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iocsys_intsel1`] module*/
#[doc(alias = "IOCSYS_INTSEL1")]
pub type IocsysIntsel1 = crate::Reg<iocsys_intsel1::IocsysIntsel1Spec>;
///SYS-domain GPIO interrupt slot mux, slots 4–5
pub mod iocsys_intsel1;
/**IOCSYS_IOMD0 (rw) register accessor: SYSIOP IO-cell mode-mux register 0

You can [`read`](crate::Reg::read) this register and get [`iocsys_iomd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocsys_iomd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iocsys_iomd0`] module*/
#[doc(alias = "IOCSYS_IOMD0")]
pub type IocsysIomd0 = crate::Reg<iocsys_iomd0::IocsysIomd0Spec>;
///SYSIOP IO-cell mode-mux register 0
pub mod iocsys_iomd0;
/**IOCSYS_IOMD1 (rw) register accessor: SYSIOP IO-cell mode-mux register 1

You can [`read`](crate::Reg::read) this register and get [`iocsys_iomd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocsys_iomd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iocsys_iomd1`] module*/
#[doc(alias = "IOCSYS_IOMD1")]
pub type IocsysIomd1 = crate::Reg<iocsys_iomd1::IocsysIomd1Spec>;
///SYSIOP IO-cell mode-mux register 1
pub mod iocsys_iomd1;
/**IO_SPI0_CS_X (rw) register accessor: IOCELL control for SPI0_CS_X / UART1-TXD

You can [`read`](crate::Reg::read) this register and get [`io_spi0_cs_x::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_spi0_cs_x::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@io_spi0_cs_x`] module*/
#[doc(alias = "IO_SPI0_CS_X")]
pub type IoSpi0CsX = crate::Reg<io_spi0_cs_x::IoSpi0CsXSpec>;
///IOCELL control for SPI0_CS_X / UART1-TXD
pub mod io_spi0_cs_x;
/**IO_SPI0_SCK (rw) register accessor: IOCELL control for SPI0_SCK / UART1-RXD

You can [`read`](crate::Reg::read) this register and get [`io_spi0_sck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_spi0_sck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@io_spi0_sck`] module*/
#[doc(alias = "IO_SPI0_SCK")]
pub type IoSpi0Sck = crate::Reg<io_spi0_sck::IoSpi0SckSpec>;
///IOCELL control for SPI0_SCK / UART1-RXD
pub mod io_spi0_sck;
/**IO_I2C0_BCK (rw) register accessor: IOCELL control for I2C0_BCK pad

You can [`read`](crate::Reg::read) this register and get [`io_i2c0_bck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_i2c0_bck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@io_i2c0_bck`] module*/
#[doc(alias = "IO_I2C0_BCK")]
pub type IoI2c0Bck = crate::Reg<io_i2c0_bck::IoI2c0BckSpec>;
///IOCELL control for I2C0_BCK pad
pub mod io_i2c0_bck;
/**IO_I2C0_BDT (rw) register accessor: IOCELL control for I2C0_BDT pad

You can [`read`](crate::Reg::read) this register and get [`io_i2c0_bdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_i2c0_bdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@io_i2c0_bdt`] module*/
#[doc(alias = "IO_I2C0_BDT")]
pub type IoI2c0Bdt = crate::Reg<io_i2c0_bdt::IoI2c0BdtSpec>;
///IOCELL control for I2C0_BDT pad
pub mod io_i2c0_bdt;
/**IO_UART2_TXD (rw) register accessor: IOCELL control for UART2 TXD pin

You can [`read`](crate::Reg::read) this register and get [`io_uart2_txd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_uart2_txd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@io_uart2_txd`] module*/
#[doc(alias = "IO_UART2_TXD")]
pub type IoUart2Txd = crate::Reg<io_uart2_txd::IoUart2TxdSpec>;
///IOCELL control for UART2 TXD pin
pub mod io_uart2_txd;
/**IO_UART2_RXD (rw) register accessor: IOCELL control for UART2 RXD pin

You can [`read`](crate::Reg::read) this register and get [`io_uart2_rxd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_uart2_rxd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@io_uart2_rxd`] module*/
#[doc(alias = "IO_UART2_RXD")]
pub type IoUart2Rxd = crate::Reg<io_uart2_rxd::IoUart2RxdSpec>;
///IOCELL control for UART2 RXD pin
pub mod io_uart2_rxd;
/**IOCAPP_INTSEL0 (rw) register accessor: APP-domain GPIO interrupt slot mux, slots 6–9 (1 byte per slot, pin index 0–63)

You can [`read`](crate::Reg::read) this register and get [`iocapp_intsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocapp_intsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iocapp_intsel0`] module*/
#[doc(alias = "IOCAPP_INTSEL0")]
pub type IocappIntsel0 = crate::Reg<iocapp_intsel0::IocappIntsel0Spec>;
///APP-domain GPIO interrupt slot mux, slots 6–9 (1 byte per slot, pin index 0–63)
pub mod iocapp_intsel0;
/**IOCAPP_INTSEL1 (rw) register accessor: APP-domain GPIO interrupt slot mux, slots 10–11

You can [`read`](crate::Reg::read) this register and get [`iocapp_intsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocapp_intsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iocapp_intsel1`] module*/
#[doc(alias = "IOCAPP_INTSEL1")]
pub type IocappIntsel1 = crate::Reg<iocapp_intsel1::IocappIntsel1Spec>;
///APP-domain GPIO interrupt slot mux, slots 10–11
pub mod iocapp_intsel1;
/**IOCAPP_IOMD (rw) register accessor: APP-domain IO-cell mode-mux register

You can [`read`](crate::Reg::read) this register and get [`iocapp_iomd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocapp_iomd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iocapp_iomd`] module*/
#[doc(alias = "IOCAPP_IOMD")]
pub type IocappIomd = crate::Reg<iocapp_iomd::IocappIomdSpec>;
///APP-domain IO-cell mode-mux register
pub mod iocapp_iomd;
/**GP_I2C4_BCK (rw) register accessor: GPIO SYS pin 1 — I2C4 clock / Arduino D14

You can [`read`](crate::Reg::read) this register and get [`gp_i2c4_bck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2c4_bck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gp_i2c4_bck`] module*/
#[doc(alias = "GP_I2C4_BCK")]
pub type GpI2c4Bck = crate::Reg<gp_i2c4_bck::GpI2c4BckSpec>;
///GPIO SYS pin 1 — I2C4 clock / Arduino D14
pub mod gp_i2c4_bck;
/**GP_I2S1_BCK (rw) register accessor: GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`gp_i2s1_bck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2s1_bck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gp_i2s1_bck`] module*/
#[doc(alias = "GP_I2S1_BCK")]
pub type GpI2s1Bck = crate::Reg<gp_i2s1_bck::GpI2s1BckSpec>;
///GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board
pub mod gp_i2s1_bck;
/**GP_I2S1_LRCK (rw) register accessor: GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`gp_i2s1_lrck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2s1_lrck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gp_i2s1_lrck`] module*/
#[doc(alias = "GP_I2S1_LRCK")]
pub type GpI2s1Lrck = crate::Reg<gp_i2s1_lrck::GpI2s1LrckSpec>;
///GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board
pub mod gp_i2s1_lrck;
/**GP_I2S1_DATA_IN (rw) register accessor: GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`gp_i2s1_data_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2s1_data_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gp_i2s1_data_in`] module*/
#[doc(alias = "GP_I2S1_DATA_IN")]
pub type GpI2s1DataIn = crate::Reg<gp_i2s1_data_in::GpI2s1DataInSpec>;
///GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board
pub mod gp_i2s1_data_in;
/**GP_I2S1_DATA_OUT (rw) register accessor: GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`gp_i2s1_data_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_i2s1_data_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gp_i2s1_data_out`] module*/
#[doc(alias = "GP_I2S1_DATA_OUT")]
pub type GpI2s1DataOut = crate::Reg<gp_i2s1_data_out::GpI2s1DataOutSpec>;
///GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board
pub mod gp_i2s1_data_out;
