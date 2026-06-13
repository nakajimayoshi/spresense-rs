//! GPIO interrupt (EXDEVICE) demo on the JP1 RTS↔CTS loopback.
//!
//! Wire **D28 (`gp_uart2_rts`) ↔ D27 (`gp_uart2_cts`)** on JP1 (same jumper as
//! the `gpio_levels` test): D28 drives the line, D27 reads it and is wired to a
//! GPIO interrupt. Two parts:
//!
//! 1. **Handler mode** — D27 is configured as a rising-edge `EXDEVICE` source and
//!    the NVIC line is unmasked. `main` pulses D28; the `#[interrupt]` handler
//!    counts each rising edge and clears the latch. Proves real NVIC dispatch.
//! 2. **Blocking waits** — the line is masked and `main` exercises
//!    `wait_for_high/low/rising_edge/falling_edge`. `main` drives the stimulus
//!    immediately before each call, so the condition is already met and the wait
//!    returns at once; a real application would instead sleep in `WFE` until
//!    external hardware moved the pin.
//!
//! D27 (`gp_uart2_cts`) is pin 69 — the APP domain — so it takes the first free
//! APP interrupt slot (slot 6) and fires `EXDEVICE_6`. That mapping is
//! deterministic here only because this is the sole GPIO interrupt configured.
//!
//! # Expected output (115 200 baud on UART1)
//!
//! ```text
//! gpio wait demo — cts interrupt = EXDEVICE_6
//! phase A (handler): 5 rising edges (expected 5)
//! phase B: wait_for_high  -> cts is_high=true
//! phase B: wait_for_low   -> cts is_low=true
//! phase B: wait_for_rising_edge  -> ok
//! phase B: wait_for_falling_edge -> ok
//! gpio wait demo complete
//! ```
//!
//! CXD5602 GPIO is 1.8 V — never wire these pins to 3.3/5 V.

#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m_rt::entry;
use panic_halt as _;

use cxd56_hal::clocks::{Config, RccExt};
use cxd56_hal::gpio::{self, Level, Trigger, pins::Parts};
use cxd56_hal::pac::{self, Interrupt, interrupt};
use cxd56_hal::uart_alt::{Uart, Uart1Pins};

/// Rising edges counted by the EXDEVICE handler in phase A.
static EDGE_COUNT: AtomicU32 = AtomicU32::new(0);

/// CTS (pin 69, APP domain) takes the first free APP slot — slot 6 → `EXDEVICE_6`.
#[interrupt]
fn EXDEVICE_6() {
    // Edge interrupts latch in the PMU; clear the latch or the line re-asserts.
    gpio::clear_interrupt(Interrupt::EXDEVICE_6);
    EDGE_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let clock = dp.crg.constrain(Config::default()).into_clock();

    // `into_floating_input()` enables the CTS pad's input buffer (ENZI) and sets
    // the pull, so no manual IO_* write is needed here. The RTS output pad needs
    // none either (`into_output` enables its driver).
    let parts = Parts::new(dp.topreg);

    // UART1 console.
    let uart1_pins = Uart1Pins {
        tx: parts.gp_spi0_cs_x,
        rx: parts.gp_spi0_sck,
    };
    let mut uart =
        Uart::new(dp.uart1, uart1_pins, Default::default(), &clock).expect("uart1 init failed");

    // JP1 loopback: D28 (UART2_RTS) drives the line, D27 (UART2_CTS) reads it and
    // is wired to the EXDEVICE interrupt. Short the two header pins together.
    let mut driver = parts.gp_uart2_rts.into_output(Level::Low);
    let mut irq_in = parts
        .gp_uart2_cts
        .into_floating_input()
        .into_interrupt(Trigger::RisingEdge, false)
        .expect("no free EXDEVICE slot");

    let _ = writeln!(
        uart,
        "gpio wait demo — cts interrupt = {:?}",
        irq_in.interrupt()
    );

    // --- Phase A: handler mode (real NVIC dispatch) ---
    // Unmask the line; the #[interrupt] handler counts rising edges that `main`
    // generates by pulsing the RTS driver.
    irq_in.clear_pending();
    irq_in.enable_interrupt();

    const PULSES: u32 = 5;
    for _ in 0..PULSES {
        driver.set_high(); // low→high = rising edge → EXDEVICE_6
        cortex_m::asm::delay(400_000);
        driver.set_low(); // re-arm low
        cortex_m::asm::delay(400_000);
    }
    cortex_m::asm::delay(400_000);
    let count = EDGE_COUNT.load(Ordering::Relaxed);
    let _ = writeln!(
        uart,
        "phase A (handler): {count} rising edges (expected {PULSES})"
    );

    // --- Phase B: blocking waits (line masked) ---
    irq_in.disable_interrupt();

    driver.set_high();
    irq_in.wait_for_high();
    let _ = writeln!(
        uart,
        "phase B: wait_for_high  -> cts is_high={}",
        irq_in.is_high()
    );

    driver.set_low();
    irq_in.wait_for_low();
    let _ = writeln!(
        uart,
        "phase B: wait_for_low   -> cts is_low={}",
        irq_in.is_low()
    );

    driver.set_low();
    irq_in.set_trigger(Trigger::RisingEdge);
    irq_in.clear_pending();
    driver.set_high(); // latch a rising edge before waiting
    irq_in.wait_for_rising_edge();
    let _ = writeln!(uart, "phase B: wait_for_rising_edge  -> ok");

    driver.set_high();
    irq_in.set_trigger(Trigger::FallingEdge);
    irq_in.clear_pending();
    driver.set_low(); // latch a falling edge before waiting
    irq_in.wait_for_falling_edge();
    let _ = writeln!(uart, "phase B: wait_for_falling_edge -> ok");

    let _ = writeln!(uart, "gpio wait demo complete");
    loop {
        cortex_m::asm::wfi();
    }
}
