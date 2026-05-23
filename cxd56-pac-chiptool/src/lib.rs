#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
//!Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    ///1 - CRG interrupt
    CRG = 1,
    ///27 - UART1 interrupt
    UART1 = 27,
    ///32 - SPI3 interrupt
    SPI3 = 32,
    ///90 - SPI0 interrupt
    SPI0 = 90,
    ///106 - 2D interrupt
    GE2D = 106,
    ///107 - Rotation interrupt
    ROT = 107,
    ///124 - CISIF interrupt
    CISIF = 124,
    ///125 - SPI5 interrupt
    SPI5 = 125,
    ///126 - DMAC3 interrupt
    DMAC3 = 126,
    ///127 - UART2 interrupt
    UART2 = 127,
    ///129 - SPI4 interrupt
    SPI4 = 129,
    ///134 - DMAC1 interrupt
    DMAC1 = 134,
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
///GPIO Port 0 — GP_* output-enable/data registers
pub const GPIO0: GPIO0::GPIO0 = unsafe { GPIO0::GPIO0::from_ptr(0x0410_2000usize as _) };
///APP / GNSS sub-domain clock and reset controller
pub const TOPREG_SUB: TOPREG_SUB::TOPREG_SUB =
    unsafe { TOPREG_SUB::TOPREG_SUB::from_ptr(0x0410_3000usize as _) };
///Synchronous Serial Port Controller (SCU SPI)
pub const SPI3: SPI0::SPI0 = unsafe { SPI0::SPI0::from_ptr(0x0418_d000usize as _) };
///Synchronous Serial Port Controller (SPIM)
pub const SPI0: SPI0::SPI0 = unsafe { SPI0::SPI0::from_ptr(0x041a_a000usize as _) };
///UART
pub const UART1: UART1::UART1 = unsafe { UART1::UART1::from_ptr(0x041a_c000usize as _) };
///CPU FIFO Control
pub const CPU_FIFO: CPU_FIFO::CPU_FIFO =
    unsafe { CPU_FIFO::CPU_FIFO::from_ptr(0x4600_c400usize as _) };
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
pub mod GPIO0;
pub mod ROT;
pub mod SMP_RAM_CTRL;
pub mod SPI0;
pub mod SPI4;
pub mod TOPREG;
pub mod TOPREG_SUB;
pub mod UART1;
pub mod UART2;
pub mod common;
