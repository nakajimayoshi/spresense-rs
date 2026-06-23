//! Async GPIO interrupt (EXDEVICE) demo on the JP1 RTS↔CTS loopback.
//!
//! Wire **D28 (`gp_uart2_rts`) ↔ D27 (`gp_uart2_cts`)** on JP1 (same jumper as
//! the `gpio_levels` test): D28 drives the line, D27 reads it and is wired to a
//! GPIO interrupt.
//!
//! `InterruptInput` implements the standard [`embedded_hal_async::digital::Wait`]
//! trait, so every wait is **interrupt-driven**: it unmasks the NVIC line and
//! sleeps in `WFE` until the `EXDEVICE_6` handler (which forwards to
//! [`gpio::on_interrupt`]) masks the line and wakes the task. The NVIC pending bit
//! captures the edge in hardware, so the brief PMU latch is never polled. A tiny
//! in-file [`rt`] runtime (`block_on` + `join` + `yield_now`, no executor crate)
//! drives the futures; an Embassy executor would work just as well.
//!
//! Because the demo drives its own loopback, each edge wait is `join`ed with a
//! small task that drives the edge *after* the wait has armed (a `yield_now` lets
//! the wait poll first) — so the CPU genuinely sleeps and the ISR wakes it.
//!
//! D27 (`gp_uart2_cts`) is pin 69 — the APP domain — so it takes the first free
//! APP interrupt slot (slot 6) and fires `EXDEVICE_6`. That mapping is
//! deterministic here only because this is the sole GPIO interrupt configured.
//!
//! # Expected output (115 200 baud on UART1)
//!
//! ```text
//! gpio wait demo — cts interrupt = EXDEVICE_6
//! phase A (async): 5 rising edges (expected 5)
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

use cortex_m_rt::entry;
use panic_halt as _;

use cxd56_hal::clocks::{Config, RccExt};
use cxd56_hal::gpio::{self, Level, Trigger, Wait, pins::Parts};
use cxd56_hal::pac::{self, Interrupt, interrupt};
use cxd56_hal::uart_alt::{Uart, Uart1Pins};

/// CTS (pin 69, APP domain) takes the first free APP slot — slot 6 → `EXDEVICE_6`.
/// Forward the vector to the HAL, which masks the line and wakes the waiting task.
#[interrupt]
fn EXDEVICE_6() {
    gpio::on_interrupt(Interrupt::EXDEVICE_6);
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

    rt::block_on(async {
        // --- Phase A: count rising edges, each driven concurrently ---
        const PULSES: u32 = 5;
        for _ in 0..PULSES {
            driver.set_low(); // hold the low baseline …
            cortex_m::asm::delay(400_000); // … long enough for the PMU detector to sample it
            rt::join(
                irq_in.wait_for_rising_edge(),
                async {
                    rt::yield_now().await; // let the wait arm first
                    driver.set_high(); // low→high = rising edge → EXDEVICE_6
                },
            )
            .await;
        }
        let _ = writeln!(uart, "phase A (async): {PULSES} rising edges (expected {PULSES})");

        // --- Phase B: each Wait method once ---
        driver.set_high();
        cortex_m::asm::delay(1_000);
        let _ = irq_in.wait_for_high().await; // already high → returns at once
        let _ = writeln!(
            uart,
            "phase B: wait_for_high  -> cts is_high={}",
            irq_in.is_high()
        );

        driver.set_low();
        cortex_m::asm::delay(1_000);
        let _ = irq_in.wait_for_low().await;
        let _ = writeln!(
            uart,
            "phase B: wait_for_low   -> cts is_low={}",
            irq_in.is_low()
        );

        // Rising edge: hold low baseline, then drive high once the wait has armed.
        driver.set_low();
        cortex_m::asm::delay(400_000);
        rt::join(irq_in.wait_for_rising_edge(), async {
            rt::yield_now().await;
            driver.set_high();
        })
        .await;
        let _ = writeln!(uart, "phase B: wait_for_rising_edge  -> ok");

        // Falling edge: hold high baseline, then drive low once the wait has armed.
        driver.set_high();
        cortex_m::asm::delay(400_000);
        rt::join(irq_in.wait_for_falling_edge(), async {
            rt::yield_now().await;
            driver.set_low();
        })
        .await;
        let _ = writeln!(uart, "phase B: wait_for_falling_edge -> ok");
    });

    let _ = writeln!(uart, "gpio wait demo complete");
    loop {
        cortex_m::asm::wfi();
    }
}

/// Minimal in-file async runtime: a `block_on` that genuinely sleeps the core in
/// `WFE` between polls, plus `join` / `yield_now` combinators. Kept local (no
/// executor crate) — `cxd56-hal` implements the standard `embedded-hal-async`
/// `Wait` trait, so an Embassy executor would work equally; this just keeps the
/// example dependency-free.
mod rt {
    use core::future::{Future, poll_fn};
    use core::pin::pin;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    // A waker that just sets the ARM event register (SEV). Combined with WFE in
    // `block_on`, a wake from any context (including the EXDEVICE ISR) makes the
    // next WFE fall through, so no wakeup is lost.
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop_noop);
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(core::ptr::null(), &VTABLE)
    }
    fn wake(_: *const ()) {
        cortex_m::asm::sev();
    }
    fn drop_noop(_: *const ()) {}

    fn make_waker() -> Waker {
        // Safety: the vtable's functions ignore the data pointer entirely, so the
        // null pointer they are paired with here is sound.
        unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) }
    }

    /// Drive `fut` to completion, sleeping in `WFE` between polls.
    pub fn block_on<F: Future>(fut: F) -> F::Output {
        let mut fut = pin!(fut);
        let waker = make_waker();
        let mut cx = Context::from_waker(&waker);
        loop {
            if let Poll::Ready(val) = fut.as_mut().poll(&mut cx) {
                return val;
            }
            cortex_m::asm::wfe();
        }
    }

    /// Yield once: `Pending` the first poll (waking itself so the executor re-polls
    /// promptly), `Ready` thereafter. Lets a sibling future run first.
    pub async fn yield_now() {
        let mut yielded = false;
        poll_fn(|cx| {
            if yielded {
                Poll::Ready(())
            } else {
                yielded = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await
    }

    /// Poll two futures concurrently until both complete.
    pub async fn join<A: Future, B: Future>(a: A, b: B) -> (A::Output, B::Output) {
        let mut a = pin!(a);
        let mut b = pin!(b);
        let mut ao: Option<A::Output> = None;
        let mut bo: Option<B::Output> = None;
        poll_fn(|cx| {
            if ao.is_none() {
                if let Poll::Ready(v) = a.as_mut().poll(cx) {
                    ao = Some(v);
                }
            }
            if bo.is_none() {
                if let Poll::Ready(v) = b.as_mut().poll(cx) {
                    bo = Some(v);
                }
            }
            if ao.is_some() && bo.is_some() {
                Poll::Ready((ao.take().unwrap(), bo.take().unwrap()))
            } else {
                Poll::Pending
            }
        })
        .await
    }
}
