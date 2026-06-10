#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
//!Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))
#![no_std]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    ///1 - CRG interrupt
    CRG = 1,
    ///11 - UART1 interrupt
    UART1 = 11,
    ///16 - SPI3 interrupt
    SPI3 = 16,
    ///17 - I2C0 (SCU_I2C0) interrupt
    I2C0 = 17,
    ///18 - I2C1 (SCU_I2C1) interrupt
    I2C1 = 18,
    ///20 - GPIO external interrupt slot 0
    EXDEVICE_0 = 20,
    ///21 - GPIO external interrupt slot 1
    EXDEVICE_1 = 21,
    ///22 - GPIO external interrupt slot 2
    EXDEVICE_2 = 22,
    ///23 - GPIO external interrupt slot 3
    EXDEVICE_3 = 23,
    ///24 - GPIO external interrupt slot 4
    EXDEVICE_4 = 24,
    ///25 - GPIO external interrupt slot 5
    EXDEVICE_5 = 25,
    ///26 - GPIO external interrupt slot 6
    EXDEVICE_6 = 26,
    ///27 - GPIO external interrupt slot 7
    EXDEVICE_7 = 27,
    ///28 - GPIO external interrupt slot 8
    EXDEVICE_8 = 28,
    ///29 - GPIO external interrupt slot 9
    EXDEVICE_9 = 29,
    ///30 - GPIO external interrupt slot 10
    EXDEVICE_10 = 30,
    ///31 - GPIO external interrupt slot 11
    EXDEVICE_11 = 31,
    ///74 - SPI0 interrupt
    SPI0 = 74,
    ///78 - CPU FIFO transmit ready (PUSH FIFO not full)
    FIFO_TO = 78,
    ///79 - CPU FIFO message received (PULL FIFO not empty)
    FIFO_FROM = 79,
    ///80 - Hardware semaphore 0 released-to-reserver wake
    SPH0 = 80,
    ///81 - Hardware semaphore 1 released-to-reserver wake
    SPH1 = 81,
    ///82 - Hardware semaphore 2 released-to-reserver wake
    SPH2 = 82,
    ///83 - Hardware semaphore 3 released-to-reserver wake
    SPH3 = 83,
    ///84 - Hardware semaphore 4 released-to-reserver wake
    SPH4 = 84,
    ///85 - Hardware semaphore 5 released-to-reserver wake
    SPH5 = 85,
    ///86 - Hardware semaphore 6 released-to-reserver wake
    SPH6 = 86,
    ///87 - Hardware semaphore 7 released-to-reserver wake
    SPH7 = 87,
    ///88 - Hardware semaphore 8 released-to-reserver wake
    SPH8 = 88,
    ///89 - Hardware semaphore 9 released-to-reserver wake
    SPH9 = 89,
    ///90 - Hardware semaphore 10 released-to-reserver wake
    SPH10 = 90,
    ///91 - Hardware semaphore 11 released-to-reserver wake
    SPH11 = 91,
    ///92 - Hardware semaphore 12 released-to-reserver wake
    SPH12 = 92,
    ///93 - Hardware semaphore 13 released-to-reserver wake
    SPH13 = 93,
    ///94 - Hardware semaphore 14 released-to-reserver wake
    SPH14 = 94,
    ///95 - Hardware semaphore 15 released-to-reserver wake
    SPH15 = 95,
    ///97 - TIMER0 interrupt
    TIMER0 = 97,
    ///98 - TIMER1 interrupt
    TIMER1 = 98,
    ///106 - 2D interrupt
    GE2D = 106,
    ///107 - Rotation interrupt
    ROT = 107,
    ///108 - CISIF interrupt
    CISIF = 108,
    ///109 - SPI5 interrupt
    SPI5 = 109,
    ///110 - DMAC3 interrupt
    DMAC3 = 110,
    ///111 - UART2 interrupt
    UART2 = 111,
    ///113 - SPI4 interrupt
    SPI4 = 113,
    ///118 - DMAC1 interrupt
    DMAC1 = 118,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
///SRAM Control
pub const SMP_RAM_CTRL: SMP_RAM_CTRL::SMP_RAM_CTRL =
    unsafe { SMP_RAM_CTRL::SMP_RAM_CTRL::from_ptr(0x0200_1000usize as _) };
///Clock / Reset / Gating
pub const CRG: CRG::CRG = unsafe { CRG::CRG::from_ptr(0x0201_1000usize as _) };
///Address Converter
pub const ADDRCONV: ADDRCONV::ADDRCONV =
    unsafe { ADDRCONV::ADDRCONV::from_ptr(0x0201_2000usize as _) };
///DMA controller (ADMAC)
pub const DMAC1: DMAC1::DMAC1 = unsafe { DMAC1::DMAC1::from_ptr(0x0202_0000usize as _) };
///CMOS Image Sensor IF
pub const CISIF: CISIF::CISIF = unsafe { CISIF::CISIF::from_ptr(0x0210_0000usize as _) };
///Hardware 2D Graphics Engine
pub const GE2D: GE2D::GE2D = unsafe { GE2D::GE2D::from_ptr(0x0210_1000usize as _) };
///Image rotation
pub const ROT: ROT::ROT = unsafe { ROT::ROT::from_ptr(0x0210_1400usize as _) };
///DMA controller (IDMAC)
pub const DMAC3: DMAC3::DMAC3 = unsafe { DMAC3::DMAC3::from_ptr(0x0210_2000usize as _) };
///UART
pub const UART2: UART2::UART2 = unsafe { UART2::UART2::from_ptr(0x0210_3000usize as _) };
///Synchronous Serial Port Controller (IMG SPI)
pub const SPI4: SPI4::SPI4 = unsafe { SPI4::SPI4::from_ptr(0x0210_3400usize as _) };
///Synchronous Serial Port Controller (IMG WSPI)
pub const SPI5: SPI4::SPI4 = unsafe { SPI4::SPI4::from_ptr(0x0210_3c00usize as _) };
///Top-of-chip clock / PMU / oscillator / PLL controller
pub const TOPREG: TOPREG::TOPREG = unsafe { TOPREG::TOPREG::from_ptr(0x0410_0000usize as _) };
///APP / GNSS sub-domain clock and reset controller
pub const TOPREG_SUB: TOPREG_SUB::TOPREG_SUB =
    unsafe { TOPREG_SUB::TOPREG_SUB::from_ptr(0x0410_3000usize as _) };
///Real-time clock 0 (always-on, 32.768 kHz, 47-bit dual counter, 3 alarms)
pub const RTC0: RTC0::RTC0 = unsafe { RTC0::RTC0::from_ptr(0x0410_8000usize as _) };
///Real-time clock 1 (SYSIOP-gated, 32.768 kHz, 47-bit dual counter, 3 alarms)
pub const RTC1: RTC0::RTC0 = unsafe { RTC0::RTC0::from_ptr(0x0410_9000usize as _) };
///Synchronous Serial Port Controller (SCU SPI)
pub const SPI3: SPI0::SPI0 = unsafe { SPI0::SPI0::from_ptr(0x0418_d000usize as _) };
///DesignWare DW_apb_i2c master controller (SCU_I2C0 / sensor I2C bus)
pub const I2C0: I2C0::I2C0 = unsafe { I2C0::I2C0::from_ptr(0x0418_d400usize as _) };
///DesignWare DW_apb_i2c master controller (SCU_I2C1)
pub const I2C1: I2C0::I2C0 = unsafe { I2C0::I2C0::from_ptr(0x0418_d800usize as _) };
///SCU ADC Interface — LPADC/HPADC control and per-channel FIFO read ports. CPU APB read of LPADC_FIFO(n) dequeues one sample from the hardware FIFO of LPADC channel n (no iSoP needed). UM §3.21.12.1; Mirror base 0x0418DC00.
pub const SCU_ADCIF: SCU_ADCIF::SCU_ADCIF =
    unsafe { SCU_ADCIF::SCU_ADCIF::from_ptr(0x0418_dc00usize as _) };
///Synchronous Serial Port Controller (SPIM)
pub const SPI0: SPI0::SPI0 = unsafe { SPI0::SPI0::from_ptr(0x041a_a000usize as _) };
///UART
pub const UART1: UART1::UART1 = unsafe { UART1::UART1::from_ptr(0x041a_c000usize as _) };
///CPU FIFO Control
pub const CPU_FIFO: CPU_FIFO::CPU_FIFO =
    unsafe { CPU_FIFO::CPU_FIFO::from_ptr(0x4600_c400usize as _) };
///Hardware semaphores (16) for inter-CPU mutual exclusion
pub const SPH: SPH::SPH = unsafe { SPH::SPH::from_ptr(0x4600_c800usize as _) };
///ARM PrimeCell SP804 dual-input timer 0
pub const TIMER0: TIMER0::TIMER0 = unsafe { TIMER0::TIMER0::from_ptr(0xe004_3000usize as _) };
///ARM PrimeCell SP804 dual-input timer 1
pub const TIMER1: TIMER0::TIMER0 = unsafe { TIMER0::TIMER0::from_ptr(0xe004_3020usize as _) };
///ARM PrimeCell SP805 watchdog timer
pub const WDOG: WDOG::WDOG = unsafe { WDOG::WDOG::from_ptr(0xe004_4000usize as _) };
/// Number available in the NVIC for configuring priority
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod ADDRCONV;
pub mod CISIF;
pub mod CPU_FIFO;
pub mod CRG;
pub mod DMAC1;
pub mod DMAC3;
pub mod GE2D;
pub mod I2C0;
pub mod ROT;
pub mod RTC0;
pub mod SCU_ADCIF;
pub mod SMP_RAM_CTRL;
pub mod SPH;
pub mod SPI0;
pub mod SPI4;
pub mod TIMER0;
pub mod TOPREG;
pub mod TOPREG_SUB;
pub mod UART1;
pub mod UART2;
pub mod WDOG;
pub mod common;
