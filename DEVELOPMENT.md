# Random additional development notes

Collection of notes and things to remember for development of the PAC and HAL

## Scope of 0.1 release

- GPIO Pins
  - Have atleast one, that is automatically connected to the onboard LED
- Clock configuration
  - Still needs testing work but initial implementation seems to work
- Delay
  - The cortex-m SysTick one is available, the SP804 is probably next

Maybe?

- Pin muxing
  - Is needed downstream for alot of other things anyways
- Interrupts?
  - very useful, especially if I want to make progress on the embassy hal
  -
- Watchdog
  - Chip uses SP805 and there is already a arm-firmware 805 driver so work might only need to get the SVD patch setup
- SPI
  - Already available on the board spi0/4/5 etc, so it might be relatively straightforward to set up the HAL without needing pinmuxing stuff
  - SPI04 uses PL022 so look for a driver crate to speed things up

For Later

- I2C
- DMA
  - Looks uses some PL080/81 so might look for drivers for those, still need other peripherals first to be useful.
- Fancy stuff like GNSS

## `memory.x` and `rt` features

I wanted to avoid having to keep copies in each of the examples. Seems like the `rt` feature is needed to automatically get cargo to pick it up. PACs its not default, HALs its default but might need some machinery to allow for opting out and let users choose their own `memory.x`.

## arm-firmware crates

TrustFirmware.org has a few different drivers for common peripherals. Testing the pl011 driver in the corresponding feature branch. There are a few that look like they might work for the sony chip, like the 805 watchdog driver, so should look to pull that in if the pl011 driver works.

## SWD debugging

There are exposed pins on the extension board but you need to solder a CoreSight 10 connector (boo!).

CMSIS-DAP probe on CN4[R] pins 16/18/100, check [pin function docs](./documentation/Spresense_pin_function_en-1.pdf).
The pins are 1.8V so need to buy a debug probe that can take a reference voltage and handle the lower voltage

there are some cmsis-dap and .gdbinit files in the SDK

```bash
openocd -s sdk/tools -f interface/cmsis-dap.cfg -f cxd5602.cfg
  rust-gdb target/thumbv7em-none-eabihf/debug/rust_blink \
    -ex "target remote :3333"
```

The SWD might be disabled by default (boo!) so I need to look that up when it comes to it.

  From nuttx/arch/arm/src/cxd56xx/cxd56_start.c:

  ▎ "By default the JTAG access to the chip is disabled at reset. The boot ROM determines the boot mode based on OTP
  ▎ BOOT_SRC or reset state pins."

## LED debugging

Probably should have thought of this myself, but writing it down to remember. Running into issues with UART and clock configuration setup. The constructor returns a Result but since the UART is what is the issue, I cant tell what the problem is. Instead, I can use the LED to encode the enum value, with different light patterns indicating the problem.

- SOS:  (150 ms on -> 150 ms off) x 3, wait 1000 ms
- Strobe: (50 ms on -> 50 ms off) x 10, wait 500 ms

## Previous Development Notes

[Sony Development Guide](https://developer.spresense.sony-semicon.com/development-guides/?page=home&lang=en)

### Prior Art

[Embassy PR](https://github.com/embassy-rs/embassy/pull/3926)

- Useful broad guide for what was needed to start getting the PAC working.

### Relevant Repos

[SPRESENSE SDK](https://github.com/sonydevworld/spresense)
[SPRESENSE Nuttx Fork](https://github.com/SPRESENSE/nuttx/tree/99dbb7fa0111dc26baca2689d67088a34d238d94)

- Contains CXD5602 specific C code for studying
- [Pin configuration code](https://github.com/SPRESENSE/nuttx/blob/99dbb7fa0111dc26baca2689d67088a34d238d94/arch/arm/src/cxd56xx/cxd56_pinconfig.c#L371) (PAC/SVD doesn't have anything obvious for controlling pin muxing)
- [cxd5602_memorymap.h](https://github.com/SPRESENSE/nuttx/blob/99dbb7fa0111dc26baca2689d67088a34d238d94/arch/arm/src/cxd56xx/hardware/cxd5602_memorymap.h#L4)

[CXD5602 Hardware Design files](https://github.com/sonydevworld/spresense-hw-design-files/tree/master)

- Contain Pin function maps too

### CXD5602 User Manual

Link to the [user manual](https://www.sony-semicon.com/files/62/pdf/p-28_CXD5602_user_manual.pdf).
Link to the [technical manual](https://www.sony-semicon.com/files/62/pdf/p-28_CXD5602GG_technical_manual.pdf).

### SVD file

~~Some of the enumerateValues names are not valid identifiers in Rust~~

- ~~Change "16 .. 240 range" to "FROM16TO240range"~~
  - ~~the ".." can't be inside an identifier~~
- ~~Change "don't" to "dont"~~
  - ~~the "'" can't be inside an identifier~~

SVD changes are now encoded into a `svdtools` patch file at `patch.yml`

```bash
svdtools patch patch.yml
```

#### svd2rust

```bash
svd2rust -i cxd5602.svd.patched
form -i lib.rs -o src/
```

#### `chiptool`

Chiptool panics on SVD names that start with a digit (`syn::Ident::new` rejects
them). `transform.chiptool.yaml` patches these at code-generation time so the
upstream `patch.yml` stays usable for other generators like `svd2rust`.

```bash
svdtools patch patch.yml
chiptool generate --svd cxd5602.svd.patched --transform transform.chiptool.yaml
form -i lib.rs -o src/
```

#### `svd2pac`

Couldn't get it to work.

```bash
$ touch LICENSE  # Need to fix another error, an demonstrate relevant issues
$ brandonsaint-john@saint-john-M93 spresense % svd2pac --license-file LICENSE cxd5602-fixed.svd test
[2025-09-17T00:41:47Z INFO  svd2pac] Reading register description file cxd5602-fixed.svd
[2025-09-17T00:41:47Z INFO  svd2pac::rust_gen] Start generating rust code
[2025-09-17T00:41:47Z ERROR svd2pac::rust_gen::xml2ir] Inheritance of access is not supported. Bitfield: TILE_CLK_GATING_ENB access shall be specified. Bitfield skipped
[2025-09-17T00:41:47Z ERROR svd2pac] Failed to generate code with err Unsupported feature derived_from is not supported in field
```

## GPIO edge-detector wall-clock floor (why the HAL settles but NuttX doesn't)

The CXD5602's GPIO *edge* interrupt isn't caught by a fast bus-clock comparator
like a normal MCU's EXTI — it runs through the PMU "wake trigger" detector, which
samples the pin on the always-on 32.768 kHz RTC ("two consecutive sampling",
manual §3.2.4.4 Fig GPIO-22). Two consequences fall out of that slow sampling:

1. **Re-arm cost.** After you (re)configure a trigger or clear the latch, the
   detector has to re-sample the current pin level as its baseline before it can
   recognise an edge. The manual quantifies it in Table GPIO-31 / Fig GPIO-24,
   "Time Interval for a Signal to be Able to Detect an Event Again": ~10–13 RTC
   cycles. At 32.768 kHz that's ~300–400 µs of *real* time — a wall-clock floor
   that does **not** shrink with a faster core.
2. **Misses are total.** An edge that arrives inside that window is *silently
   dropped*, not delayed. It never latches, never pends, so an async
   `wait_for_*_edge` would hang forever.

So the HAL holds a baseline settle (`edge_arm_settle`, 16 RTC ticks for margin)
after the arming clear, before unmasking the line. It's an *async* delay now (RTC
alarm 0, via `cxd56_hal::async_delay`), so the core `WFE`-sleeps the ~488 µs
instead of busy-spinning it — but the wall-clock floor itself is unavoidable.

### Why NuttX doesn't

`cxd56_gpioint.c` has *no* settle anywhere — no udelay/spin after the latch clear
or the route write. It gets away with it by *usage pattern*, not because the floor
doesn't exist: every GPIO interrupt source the SDK ships is an external chip
(ALT1250 modem, NRC7292 / GS2200M Wi-Fi, WizNet Ethernet) whose pin sits at its
baseline level for milliseconds before the edge, so the detector has long since
re-sampled. There is no self-loopback (clear-then-immediately-drive the same pin
on the same core) anywhere in the tree — that pattern is the only thing that hits
the window, and `examples/rust_gpio_wait_lp` deliberately uses it (D28→D27).

### Realistic ways to hit it *without* a loopback

The floor is a genuine hardware ceiling, not just a test artifact. Even with a real
external source you drop edges when:

- **Edges land < ~400 µs apart** — i.e. faster than ~2.5–3 kHz. The "detect an
  event again" interval *is* the detector's maximum edge rate; the second edge of a
  tight pair is lost. Fast pulse trains, a quadrature-encoder channel at speed, or a
  chip that double-pulses / coalesces its IRQ line all qualify. NuttX would drop
  these too — it just never wires a source that fast to `gpioint`.
- **You flip polarity right before the edge** — switching rising→falling, or arming
  "both edges", immediately before the line moves: the re-baseline window applies
  after the config write just as it does after a clear.

Most HALs never need to handle this because their edge detectors run on a MHz clock
and the re-arm window is nanoseconds. The CXD5602 is unusual in routing edges
through a 32 kHz PMU wake detector (it's built for wake-from-sleep, not high-rate
counting), so the window is hundreds of microseconds and has to be handled
explicitly whenever you self-drive the pin or feed it fast edges.
