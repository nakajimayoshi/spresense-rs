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
}
pub mod regs;
