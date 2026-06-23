//! Async GPIO interrupt (EXDEVICE) demo at the **low-power** operating point.
//!
//! Identical to the `rust_gpio_wait` example but switches the APP CPU to
//! [`Perf::Lp`] (~39 MHz, VDD_CORE 0.7 V) before doing anything, to confirm the
//! async EXDEVICE edge handling still works at the slowest core clock.
//!
//! Why this is a useful check: the GPIO edge detector is clocked by the fixed
//! 32.768 kHz RTC, so its timing is a fixed *real* time (~1–2 RTC periods). The
//! HAL's edge baseline-settle and clear-wait, and the example's pulse delays, are
//! all in *CPU cycles* — at the slower LP clock each cycle is more real time, so
//! every "wait at least" margin only grows. LP is therefore the easy direction
//! (HP ~156 MHz is the worst case for those CPU-cycle minimums); this proves it
//! empirically while the CPU genuinely sleeps in `WFE` between each edge.
//!
//! Wire **D28 (`gp_uart2_rts`) ↔ D27 (`gp_uart2_cts`)** on JP1, same as
//! `rust_gpio_wait`. Each edge wait is `join`ed with a task that drives the edge
//! after the wait has armed, so the ISR is what wakes the sleeping core.
//!
//! # Expected output (115 200 baud on UART1)
//!
//! ```text
//! gpio wait demo (LOW POWER) — APP CPU = 39000000 Hz, cts interrupt = EXDEVICE_6
//! phase A (async): 5 rising edges (expected 5)
//! phase B: wait_for_high  -> cts is_high=true
//! phase B: wait_for_low   -> cts is_low=true
//! phase B: wait_for_rising_edge  -> ok
//! phase B: wait_for_falling_edge -> ok
//! gpio wait demo complete
//! ```
//! (The exact APP CPU figure depends on the XOSC; ~39 MHz at the LP point.)
//!
//! CXD5602 GPIO is 1.8 V — never wire these pins to 3.3/5 V.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use panic_halt as _;

use cxd56_hal::clocks::{Config, Perf, RccExt};
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

    // Drop to the low-power operating point *before* the UART is created, so the
    // UART computes its baud divisor from the LP clock. If this RPC to the SYSIOP
    // loader fails the board halts here (before any output) — no output on an
    // otherwise-correct setup means the perf switch itself failed.
    let mut clock = dp.crg.constrain(Config::default()).into_clock();
    clock
        .request_perf(Perf::Lp)
        .expect("failed to enter low-power operating point");

    // `into_floating_input()` enables the CTS pad's input buffer (ENZI) and sets
    // the pull; the RTS output pad's driver is enabled by `into_output`.
    let parts = Parts::new(dp.topreg);

    // UART1 console (baud computed from the now-LP clock).
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

    let appsmp_hz = clock.freeze().appsmp.to_Hz();
    let _ = writeln!(
        uart,
        "gpio wait demo (LOW POWER) — APP CPU = {} Hz, cts interrupt = {:?}",
        appsmp_hz,
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
