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
