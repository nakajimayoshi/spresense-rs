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
