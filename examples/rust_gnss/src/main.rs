//! GNSS positioning on the Spresense main board.
//!
//! The GNSS receiver is `gnssfw` firmware running on a separate internal DSP
//! core; the APP core drives it as an RPC client over the CPU-FIFO mailbox (see
//! [`cxd56_hal::gnss`]). This example boots that firmware, selects the satellite
//! systems, starts positioning, then polls for fixes and prints latitude /
//! longitude / altitude to UART1.
//!
//! ## Choosing satellites
//!
//! Flip [`SELECTION`] between:
//!   * [`SatelliteSelection::Automatic`] — the HAL's default constellation set
//!     ([`cxd56_hal::gnss::AUTOMATIC`]): **GPS + GLONASS + QZSS-L1C/A + SBAS, 4
//!     satellite systems**. The example prints the count it applied.
//!   * [`SatelliteSelection::Manual`] — exactly the systems you list, e.g.
//!     `Manual(SatelliteSystem::GPS | SatelliteSystem::GALILEO)`.
//!
//! No external antenna logic is needed beyond the board's GNSS antenna; a clear
//! sky view is required for the first fix (cold start: tens of seconds to minutes).
//!
//! Status is mirrored on on-board LED0 (`gp_i2s1_bck`): **on** during bring-up,
//! **toggles** on each valid fix, **off** on failure.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::Peripherals as CorePeripherals;
use cortex_m_rt::entry;
use panic_halt as _;

use cxd56_hal::{
    clocks::{Config, RccExt},
    delay_alt::Delay,
    gnss::{Gnss, GnssError, Notify, SatelliteSelection, StartMode},
    gpio::{pins::Parts, Level, Output},
    pac,
    pac::topreg::GpI2s1Bck,
    uart_alt::{Uart, Uart1Pins},
};
use embedded_hal::delay::DelayNs;

/// How to pick constellations. Swap to
/// `SatelliteSelection::Manual(SatelliteSystem::GPS | SatelliteSystem::GLONASS)`
/// to choose them yourself.
const SELECTION: SatelliteSelection = SatelliteSelection::Automatic;

/// Cold start makes no assumptions about prior almanac/time — the safe default.
const START_MODE: StartMode = StartMode::Cold;

/// LED0 off, then spin forever — the failure indicator.
fn fail(led: &mut Output<GpI2s1Bck>) -> ! {
    led.set_low();
    loop {
        cortex_m::asm::wfe();
    }
}

#[entry]
fn main() -> ! {
    let pac = pac::Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let crg = pac.crg.constrain(Config::default());
    let mut clock = crg.into_clock();

    let parts = Parts::new(pac.topreg);

    // UART1 console (same pads as the other examples).
    let uart1_pins = Uart1Pins {
        tx: parts.gp_spi0_cs_x,
        rx: parts.gp_spi0_sck,
    };
    let mut uart =
        Uart::new(pac.uart1, uart1_pins, Default::default(), &clock).expect("uart1 init failed");

    // Calibrated from an explicit APP-core rate (not a `&clock` borrow) so
    // `clock` stays free for `resample()` after GNSS boot shifts the clock tree.
    let mut delay = Delay::with_clock(core.SYST, clock.appsmp().hz().to_Hz());

    // LED0 mirrors status: on = bringing up, toggles = fixes, off = fail.
    let mut led = parts.gp_i2s1_bck.into_output(Level::Low);
    led.set_high();

    writeln!(uart, "GNSS: loading gnssfw and starting the DSP core...").ok();

    let mut gnss = Gnss::new();

    // 1. Load + start the GNSS firmware and wait for BOOTCOMP. Booting gnssfw
    //    makes the loader raise the SYSIOP clock tree, which shifts the COM bus
    //    UART1 is clocked from. Capture the result, then re-sample the clock and
    //    rebuild UART1 before printing — otherwise the console baud is wrong and
    //    every line below comes out garbled.
    let boot_result = gnss.boot();
    clock.resample();
    let (uart1, uart1_pins) = uart.free();
    let mut uart =
        Uart::new(uart1, uart1_pins, Default::default(), &clock).expect("uart1 re-init failed");

    if let Err(e) = boot_result {
        if e == GnssError::BootTimeout {
            writeln!(uart, "boot notify timed out; settling and probing GNSS...").ok();
            delay.delay_ms(250);
        } else {
            writeln!(uart, "boot failed: {e:?}").ok();
            fail(&mut led);
        }
    }
    writeln!(uart, "gnssfw load/start complete").ok();

    // 2. Choose the satellite systems. With Automatic, report how many we used.
    let systems = match gnss.select_satellites(SELECTION) {
        Ok(s) => s,
        Err(e) => {
            writeln!(uart, "select_satellites failed: {e:?}").ok();
            fail(&mut led);
        }
    };
    match SELECTION {
        SatelliteSelection::Automatic => {
            writeln!(
                uart,
                "satellites: AUTOMATIC -> {} systems ({systems:?})",
                systems.count()
            )
            .ok();
        }
        SatelliteSelection::Manual(_) => {
            writeln!(uart, "satellites: MANUAL -> {systems:?}").ok();
        }
    }

    // 3. Begin positioning. Starting the correlator runs the GPS core harder,
    //    so the loader steps the SYSIOP clock a second time — re-sync UART1 once
    //    more, the same way we did after boot.
    let start_result = gnss.start(START_MODE);
    clock.resample();
    let (uart1, uart1_pins) = uart.free();
    let mut uart =
        Uart::new(uart1, uart1_pins, Default::default(), &clock).expect("uart1 re-init failed");

    if let Err(e) = start_result {
        writeln!(uart, "start failed: {e:?}").ok();
        fail(&mut led);
    }
    writeln!(
        uart,
        "positioning started ({START_MODE:?}); waiting for a fix..."
    )
    .ok();

    // 4. Read in response to the firmware's POSITION notification, not on a
    //    fixed timer. The firmware latches a fresh position buffer each cycle
    //    and signals POSITION; reading at any other time returns Status(-55)
    //    (no fresh data). This mirrors the NuttX gps example (sigwait → read).
    //    A valid buffer with no fix yet is normal — it reports sats=0 until lock.
    // LED0 blinks as a "searching" heartbeat until the first valid fix, then
    // holds solid. `blink` carries the heartbeat phase; `have_fix` the last state.
    let mut blink = false;
    let mut have_fix = false;
    loop {
        // Only read when a fresh POSITION notification arrives (reading at any
        // other time returns Status(-55)); the poll itself is non-blocking.
        if gnss.poll_notify() == Some(Notify::Position) {
            match gnss.read_fix() {
                Ok(fix) => {
                    if let Some(dims) = fix.dimensions() {
                        have_fix = true;
                        writeln!(
                            uart,
                            "FIX {dims}D  lat={:.6} lon={:.6} alt={:.1}m  sats={}  hdop={:.1}  {:04}-{:02}-{:02} {:02}:{:02}:{:02}Z",
                            fix.latitude,
                            fix.longitude,
                            fix.altitude,
                            fix.num_satellites,
                            fix.hdop,
                            fix.date.year,
                            fix.date.month,
                            fix.date.day,
                            fix.time.hour,
                            fix.time.minute,
                            fix.time.sec,
                        )
                        .ok();
                    } else {
                        // No fix yet. `tracking` is the number that matters here:
                        // if it climbs above ~4 the receiver is hearing satellites
                        // and just needs time/geometry; if it stays 0 under open
                        // sky the problem is RF/setup, not the weather.
                        have_fix = false;
                        writeln!(
                            uart,
                            "...acquiring  tracking={} visible={}  (used={})",
                            fix.num_tracking, fix.num_visible, fix.num_satellites
                        )
                        .ok();
                    }
                }
                Err(e) => {
                    writeln!(uart, "read_fix error: {e:?}").ok();
                }
            }
        }

        // Blink while searching, hold solid once we have a fix. The small delay
        // paces the loop: positioning runs ~1 Hz and the mailbox queues
        // notifications, so a relaxed poll still never misses one.
        if have_fix {
            led.set_high();
        } else {
            blink = !blink;
            led.set_level(if blink { Level::High } else { Level::Low });
        }
        delay.delay_ms(25);
    }
}
