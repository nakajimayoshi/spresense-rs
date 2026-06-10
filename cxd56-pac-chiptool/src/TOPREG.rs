///Top-of-chip clock / PMU / oscillator / PLL controller.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TOPREG {
    ptr: *mut u8,
}
unsafe impl Send for TOPREG {}
unsafe impl Sync for TOPREG {}
impl TOPREG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    ///Power-domain control (1 = powered on).
    #[inline(always)]
    pub const fn PWD_CTL(self) -> crate::common::Reg<regs::PWD_CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    ///Analog circuit power control (1 = powered on).
    #[inline(always)]
    pub const fn ANA_PW_CTL(self) -> crate::common::Reg<regs::ANA_PW_CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    ///Analog enable control (paired set/clear bits — use raw write).
    #[inline(always)]
    pub const fn ANA_EN_CTL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    ///PMU power-supply control request (write-only kick register).
    #[inline(always)]
    pub const fn PMU_PW_CTL(self) -> crate::common::Reg<regs::PMU_PW_CTL, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    ///Per-domain power reset control.
    #[inline(always)]
    pub const fn PWD_RESET0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    ///Power-domain status (read-only mirror of PWD_CTL).
    #[inline(always)]
    pub const fn PWD_STAT(self) -> crate::common::Reg<regs::PWD_STAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    ///Analog power status (read-only mirror of ANA_PW_CTL).
    #[inline(always)]
    pub const fn ANA_PW_STAT(self) -> crate::common::Reg<regs::ANA_PW_STAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    ///Clock select/divider setting used on wake from sleep.
    #[inline(always)]
    pub const fn CLSELDIV_WAKE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    ///CPU/DSP/bus clock divider used on wake from sleep.
    #[inline(always)]
    pub const fn CKDIV_CPU_DSP_BUS_WAKE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    ///Root clock source select used on wake from sleep.
    #[inline(always)]
    pub const fn CKSEL_ROOT_WAKE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    ///Positive wake-trigger enable, slots 0–11.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_EN0(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG_EN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    ///Positive wake-trigger enable, second bank.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_EN1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    ///Negative wake-trigger enable, slots 0–11.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_NEGEN0(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG_NEGEN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    ///Negative wake-trigger enable, second bank.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_NEGEN1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    ///Noise filter enable for interrupt slots 0–11.
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_NOISECUTEN0(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG_NOISECUTEN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    ///CPU interrupt route select, slots 0–3 (3-bit field per slot at 16+slot*4).
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_CPUINTSEL0(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG_CPUINTSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    ///CPU interrupt route select, slots 4–11 (3-bit field per slot at slot*4).
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_CPUINTSEL1(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG_CPUINTSEL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    ///CPU interrupt route select, second bank (unused by GPIO driver).
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_CPUINTSEL2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    ///Interrupt detect polarity, slots 0–3 (3-bit field per slot at 16+slot*4).
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_INTDET0(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG_INTDET0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    ///Interrupt detect polarity, slots 4–11 (3-bit field per slot at slot*4).
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_INTDET1(
        self,
    ) -> crate::common::Reg<regs::PMU_WAKE_TRIG_INTDET1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    ///Interrupt detect polarity, second bank (unused by GPIO driver).
    #[inline(always)]
    pub const fn PMU_WAKE_TRIG_INTDET2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    ///PMU core clock enable.
    #[inline(always)]
    pub const fn PMU_CORE_CKEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    ///Root clock source select and RTC status.
    #[inline(always)]
    pub const fn CKSEL_ROOT(self) -> crate::common::Reg<regs::CKSEL_ROOT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    ///PMU clock source select.
    #[inline(always)]
    pub const fn CKSEL_PMU(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    ///SYSIOP clock source select.
    #[inline(always)]
    pub const fn CKSEL_SYSIOP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    ///SYSIOP sub-domain clock source select.
    #[inline(always)]
    pub const fn CKSEL_SYSIOP_SUB(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    ///SCU clock source select.
    #[inline(always)]
    pub const fn CKSEL_SCU(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    ///CPU / DSP / bus clock divider.
    #[inline(always)]
    pub const fn CKDIV_CPU_DSP_BUS(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    ///COM clock divider.
    #[inline(always)]
    pub const fn CKDIV_COM(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    ///Host interface clock divider.
    #[inline(always)]
    pub const fn CKDIV_HOSTIFC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    ///SCU clock divider.
    #[inline(always)]
    pub const fn CKDIV_SCU(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    ///PMU clock divider.
    #[inline(always)]
    pub const fn CKDIV_PMU(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    ///TOPREG clock-ready interrupt clear 0 (write 1 to clear).
    #[inline(always)]
    pub const fn CRG_INT_CLR0(self) -> crate::common::Reg<regs::CRG_INT_CLR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    ///TOPREG clock-ready interrupt mask 0.
    #[inline(always)]
    pub const fn CRG_INT_MASK0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    ///TOPREG clock-ready masked interrupt status 0 (read-only).
    #[inline(always)]
    pub const fn CRG_INT_STAT_MSK0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    ///TOPREG clock-ready raw interrupt status 0 (read-only).
    #[inline(always)]
    pub const fn CRG_INT_STAT_RAW0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    ///TOPREG clock-ready interrupt clear 1 (write 1 to clear).
    #[inline(always)]
    pub const fn CRG_INT_CLR1(self) -> crate::common::Reg<regs::CRG_INT_CLR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    ///TOPREG clock-ready interrupt mask 1.
    #[inline(always)]
    pub const fn CRG_INT_MASK1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    ///TOPREG clock-ready masked interrupt status 1 (read-only).
    #[inline(always)]
    pub const fn CRG_INT_STAT_MSK1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    ///TOPREG clock-ready raw interrupt status 1 (read-only).
    #[inline(always)]
    pub const fn CRG_INT_STAT_RAW1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    ///CPU clock gating control.
    #[inline(always)]
    pub const fn CPU_GATECLK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    ///USB PHY clock enable.
    #[inline(always)]
    pub const fn USBPHY_CKEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    ///CRG monitor status (read-only).
    #[inline(always)]
    pub const fn CRG_MON(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    ///Clock gear status (read-only).
    #[inline(always)]
    pub const fn GEAR_STAT(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    ///Crystal oscillator control.
    #[inline(always)]
    pub const fn XOSC_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    ///Crystal oscillator control 2.
    #[inline(always)]
    pub const fn XOSC_CTRL2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    ///System PLL control 1.
    #[inline(always)]
    pub const fn SYS_PLL_CTRL1(self) -> crate::common::Reg<regs::SYS_PLL_CTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    ///System PLL control 2 (division ratios).
    #[inline(always)]
    pub const fn SYS_PLL_CTRL2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    ///RC oscillator control 1.
    #[inline(always)]
    pub const fn RCOSC_CTRL1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    ///RC oscillator control 2.
    #[inline(always)]
    pub const fn RCOSC_CTRL2(self) -> crate::common::Reg<regs::RCOSC_CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    ///RF GP MBI enable.
    #[inline(always)]
    pub const fn RF_GPMBI_EN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    ///Force clock enable (overrides gating).
    #[inline(always)]
    pub const fn FORCE_CKEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0644usize) as _) }
    }
    ///Clock gate control.
    #[inline(always)]
    pub const fn CKGATE_CTL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    ///Bus peripheral software reset (0 = held in reset, 1 = released).
    #[inline(always)]
    pub const fn SWRESET_BUS(self) -> crate::common::Reg<regs::SWRESET_BUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    ///SCU peripheral software reset (0 = held in reset, 1 = released).
    #[inline(always)]
    pub const fn SWRESET_SCU(self) -> crate::common::Reg<regs::SWRESET_SCU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    ///Bus ROM clock enable.
    #[inline(always)]
    pub const fn BUSROM_CKEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    ///SYSIOP peripheral clock enables.
    #[inline(always)]
    pub const fn SYSIOP_CKEN(self) -> crate::common::Reg<regs::SYSIOP_CKEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    ///SCU peripheral clock enables.
    #[inline(always)]
    pub const fn SCU_CKEN(self) -> crate::common::Reg<regs::SCU_CKEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    ///RTC0 control.
    #[inline(always)]
    pub const fn RTC0_CTL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0730usize) as _) }
    }
    ///SYS-domain GPIO interrupt slot mux, slots 0–3 (1 byte per slot, pin index 0–63).
    #[inline(always)]
    pub const fn IOCSYS_INTSEL0(
        self,
    ) -> crate::common::Reg<regs::IOCSYS_INTSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b0usize) as _) }
    }
    ///SYS-domain GPIO interrupt slot mux, slots 4–5.
    #[inline(always)]
    pub const fn IOCSYS_INTSEL1(
        self,
    ) -> crate::common::Reg<regs::IOCSYS_INTSEL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b4usize) as _) }
    }
    ///SYSIOP IO-cell mode-mux register 0.
    #[inline(always)]
    pub const fn IOCSYS_IOMD0(self) -> crate::common::Reg<regs::IOCSYS_IOMD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize) as _) }
    }
    ///SYSIOP IO-cell mode-mux register 1.
    #[inline(always)]
    pub const fn IOCSYS_IOMD1(self) -> crate::common::Reg<regs::IOCSYS_IOMD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c4usize) as _) }
    }
    ///IOCELL control for SPI0_CS_X / UART1-TXD.
    #[inline(always)]
    pub const fn IO_SPI0_CS_X(self) -> crate::common::Reg<regs::IO_SPI0_CS_X, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0844usize) as _) }
    }
    ///IOCELL control for SPI0_SCK / UART1-RXD.
    #[inline(always)]
    pub const fn IO_SPI0_SCK(self) -> crate::common::Reg<regs::IO_SPI0_SCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0848usize) as _) }
    }
    ///IOCELL control for SEN_IRQ_IN pin.
    #[inline(always)]
    pub const fn IO_SEN_IRQ_IN(self) -> crate::common::Reg<regs::IO_SEN_IRQ_IN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0894usize) as _) }
    }
    ///IOCELL control for I2C0_BCK pad.
    #[inline(always)]
    pub const fn IO_I2C0_BCK(self) -> crate::common::Reg<regs::IO_I2C0_BCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08b0usize) as _) }
    }
    ///IOCELL control for I2C0_BDT pad.
    #[inline(always)]
    pub const fn IO_I2C0_BDT(self) -> crate::common::Reg<regs::IO_I2C0_BDT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08b4usize) as _) }
    }
    ///IOCELL control for UART2 TXD pin.
    #[inline(always)]
    pub const fn IO_UART2_TXD(self) -> crate::common::Reg<regs::IO_UART2_TXD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x090cusize) as _) }
    }
    ///IOCELL control for UART2 RXD pin.
    #[inline(always)]
    pub const fn IO_UART2_RXD(self) -> crate::common::Reg<regs::IO_UART2_RXD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0910usize) as _) }
    }
    ///IOCELL control for UART2 CTS pin.
    #[inline(always)]
    pub const fn IO_UART2_CTS(self) -> crate::common::Reg<regs::IO_UART2_CTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0914usize) as _) }
    }
    ///IOCELL control for UART2 RTS pin.
    #[inline(always)]
    pub const fn IO_UART2_RTS(self) -> crate::common::Reg<regs::IO_UART2_RTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0918usize) as _) }
    }
    ///IOCELL control for EMMC_CLK / SPI5_SCK pin.
    #[inline(always)]
    pub const fn IO_EMMC_CLK(self) -> crate::common::Reg<regs::IO_EMMC_CLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x092cusize) as _) }
    }
    ///IOCELL control for EMMC_CMD / SPI5_CS_X pin.
    #[inline(always)]
    pub const fn IO_EMMC_CMD(self) -> crate::common::Reg<regs::IO_EMMC_CMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0930usize) as _) }
    }
    ///IOCELL control for EMMC_DATA0 / SPI5_MOSI pin.
    #[inline(always)]
    pub const fn IO_EMMC_DATA0(self) -> crate::common::Reg<regs::IO_EMMC_DATA0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0934usize) as _) }
    }
    ///IOCELL control for EMMC_DATA1 / SPI5_MISO pin.
    #[inline(always)]
    pub const fn IO_EMMC_DATA1(self) -> crate::common::Reg<regs::IO_EMMC_DATA1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0938usize) as _) }
    }
    ///IOCELL control for EMMC_DATA2 pin.
    #[inline(always)]
    pub const fn IO_EMMC_DATA2(self) -> crate::common::Reg<regs::IO_EMMC_DATA2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x093cusize) as _) }
    }
    ///IOCELL control for EMMC_DATA3 pin.
    #[inline(always)]
    pub const fn IO_EMMC_DATA3(self) -> crate::common::Reg<regs::IO_EMMC_DATA3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    ///IOCELL control for I2S0_BCK pin.
    #[inline(always)]
    pub const fn IO_I2S0_BCK(self) -> crate::common::Reg<regs::IO_I2S0_BCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0974usize) as _) }
    }
    ///IOCELL control for I2S0_LRCK pin.
    #[inline(always)]
    pub const fn IO_I2S0_LRCK(self) -> crate::common::Reg<regs::IO_I2S0_LRCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0978usize) as _) }
    }
    ///IOCELL control for I2S0_DATA_IN pin.
    #[inline(always)]
    pub const fn IO_I2S0_DATA_IN(
        self,
    ) -> crate::common::Reg<regs::IO_I2S0_DATA_IN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x097cusize) as _) }
    }
    ///IOCELL control for I2S0_DATA_OUT pin.
    #[inline(always)]
    pub const fn IO_I2S0_DATA_OUT(
        self,
    ) -> crate::common::Reg<regs::IO_I2S0_DATA_OUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    ///APP-domain GPIO interrupt slot mux, slots 6–9 (1 byte per slot, pin index 0–63).
    #[inline(always)]
    pub const fn IOCAPP_INTSEL0(
        self,
    ) -> crate::common::Reg<regs::IOCAPP_INTSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1490usize) as _) }
    }
    ///APP-domain GPIO interrupt slot mux, slots 10–11.
    #[inline(always)]
    pub const fn IOCAPP_INTSEL1(
        self,
    ) -> crate::common::Reg<regs::IOCAPP_INTSEL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1494usize) as _) }
    }
    ///APP-domain IO-cell mode-mux register.
    #[inline(always)]
    pub const fn IOCAPP_IOMD(self) -> crate::common::Reg<regs::IOCAPP_IOMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14a0usize) as _) }
    }
    ///GPIO SYS pin 1 — I2C4 clock / Arduino D14.
    #[inline(always)]
    pub const fn GP_I2C4_BCK(self) -> crate::common::Reg<regs::GP_I2C4_BCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    ///GPIO SYS pin 17 — SPI0_CS_X / UART1_TX (on-board console).
    #[inline(always)]
    pub const fn GP_SPI0_CS_X(self) -> crate::common::Reg<regs::GP_SPI0_CS_X, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    ///GPIO SYS pin 18 — SPI0_SCK / UART1_RX (on-board console).
    #[inline(always)]
    pub const fn GP_SPI0_SCK(self) -> crate::common::Reg<regs::GP_SPI0_SCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    ///GPIO SYS pin 37 — SEN_IRQ / Arduino D22 (JP1).
    #[inline(always)]
    pub const fn GP_SEN_IRQ_IN(self) -> crate::common::Reg<regs::GP_SEN_IRQ_IN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2090usize) as _) }
    }
    ///GPIO SYS pin 44 — I2C0_SCL / Arduino D15 (JP2).
    #[inline(always)]
    pub const fn GP_I2C0_BCK(self) -> crate::common::Reg<regs::GP_I2C0_BCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20acusize) as _) }
    }
    ///GPIO SYS pin 45 — I2C0_SDA / Arduino D14 (JP2).
    #[inline(always)]
    pub const fn GP_I2C0_BDT(self) -> crate::common::Reg<regs::GP_I2C0_BDT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b0usize) as _) }
    }
    ///GPIO APP pin 67 — UART2_TX / Arduino D01 (JP1).
    #[inline(always)]
    pub const fn GP_UART2_TXD(self) -> crate::common::Reg<regs::GP_UART2_TXD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f0usize) as _) }
    }
    ///GPIO APP pin 68 — UART2_RX / Arduino D00 (JP1).
    #[inline(always)]
    pub const fn GP_UART2_RXD(self) -> crate::common::Reg<regs::GP_UART2_RXD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f4usize) as _) }
    }
    ///GPIO APP pin 69 — UART2_CTS / Arduino D27 (JP1).
    #[inline(always)]
    pub const fn GP_UART2_CTS(self) -> crate::common::Reg<regs::GP_UART2_CTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f8usize) as _) }
    }
    ///GPIO APP pin 70 — UART2_RTS / Arduino D28 (JP1).
    #[inline(always)]
    pub const fn GP_UART2_RTS(self) -> crate::common::Reg<regs::GP_UART2_RTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20fcusize) as _) }
    }
    ///GPIO APP pin 75 — SPI5_SCK / Arduino D23 (JP1).
    #[inline(always)]
    pub const fn GP_EMMC_CLK(self) -> crate::common::Reg<regs::GP_EMMC_CLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2110usize) as _) }
    }
    ///GPIO APP pin 76 — SPI5_CS_X / Arduino D24 (JP1).
    #[inline(always)]
    pub const fn GP_EMMC_CMD(self) -> crate::common::Reg<regs::GP_EMMC_CMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2114usize) as _) }
    }
    ///GPIO APP pin 77 — SPI5_MOSI / Arduino D16 (JP2).
    #[inline(always)]
    pub const fn GP_EMMC_DATA0(self) -> crate::common::Reg<regs::GP_EMMC_DATA0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2118usize) as _) }
    }
    ///GPIO APP pin 78 — SPI5_MISO / Arduino D17 (JP2).
    #[inline(always)]
    pub const fn GP_EMMC_DATA1(self) -> crate::common::Reg<regs::GP_EMMC_DATA1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x211cusize) as _) }
    }
    ///GPIO APP pin 79 — GPIO / Arduino D20 (JP2).
    #[inline(always)]
    pub const fn GP_EMMC_DATA2(self) -> crate::common::Reg<regs::GP_EMMC_DATA2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    ///GPIO APP pin 80 — GPIO / Arduino D21 (JP2).
    #[inline(always)]
    pub const fn GP_EMMC_DATA3(self) -> crate::common::Reg<regs::GP_EMMC_DATA3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2124usize) as _) }
    }
    ///GPIO APP pin 93 — I2S0_BCK / Arduino D26 (JP1).
    #[inline(always)]
    pub const fn GP_I2S0_BCK(self) -> crate::common::Reg<regs::GP_I2S0_BCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2158usize) as _) }
    }
    ///GPIO APP pin 94 — I2S0_LRCK / Arduino D25 (JP1).
    #[inline(always)]
    pub const fn GP_I2S0_LRCK(self) -> crate::common::Reg<regs::GP_I2S0_LRCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x215cusize) as _) }
    }
    ///GPIO APP pin 95 — I2S0_DATA_IN / Arduino D19 (JP2).
    #[inline(always)]
    pub const fn GP_I2S0_DATA_IN(
        self,
    ) -> crate::common::Reg<regs::GP_I2S0_DATA_IN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2160usize) as _) }
    }
    ///GPIO APP pin 96 — I2S0_DATA_OUT / Arduino D18 (JP2).
    #[inline(always)]
    pub const fn GP_I2S0_DATA_OUT(
        self,
    ) -> crate::common::Reg<regs::GP_I2S0_DATA_OUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2164usize) as _) }
    }
    ///GPIO APP pin 97 — I2S1_BCK / LED0 on Spresense main board.
    #[inline(always)]
    pub const fn GP_I2S1_BCK(self) -> crate::common::Reg<regs::GP_I2S1_BCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2168usize) as _) }
    }
    ///GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board.
    #[inline(always)]
    pub const fn GP_I2S1_LRCK(self) -> crate::common::Reg<regs::GP_I2S1_LRCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x216cusize) as _) }
    }
    ///GPIO APP pin 99 — I2S1_DATA_IN / LED2 on Spresense main board.
    #[inline(always)]
    pub const fn GP_I2S1_DATA_IN(
        self,
    ) -> crate::common::Reg<regs::GP_I2S1_DATA_IN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2170usize) as _) }
    }
    ///GPIO APP pin 100 — I2S1_DATA_OUT / LED3 on Spresense main board.
    #[inline(always)]
    pub const fn GP_I2S1_DATA_OUT(
        self,
    ) -> crate::common::Reg<regs::GP_I2S1_DATA_OUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2174usize) as _) }
    }
}
pub mod regs;
