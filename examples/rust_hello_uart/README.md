# Hello world on the Spresense with Rust

This is a Hello world example using the [`cxd56-hal`](../../cxd56-hal/). In this example, the onboard LED closest to the center of the board will blink three times (SOS) to indicate start up. Then it will print "Hello rust, {n}" following by a strobe blinking of the LED.

This is also an empirical test to validate that the UART and clock configuration setup is all working correctly.

## Build and flash

```bash
cargo build --release && mkspk -c 2 target/thumbv7em-none-eabihf/release/rust_hello_uart nuttx target/rust_hello.spk && flash_writer -c /dev/ttyUSB1 target/rust_hello.spk 
```

Expected output

```text
    Finished `release` profile [optimized] target(s) in 4.54s
File target/rust_hello.spk is successfully created.
install
Install target/rust_hello.spk
|0%--------------------------------50%------------------------------100%|
######################################################################
4240 bytes loaded.
Package validation is OK.
Saving package to "nuttx"
sync
Restarting the board ...
reboot
```

### Observing UART output `picocom` on Linux

```bash
$ sudo dmesg | tail  # Find out the connected serial port
[  483.786639] usb 1-2.2: cp210x converter now attached to ttyUSB1
$ picocom -b 115200 --imap lfcrlf --noreset /dev/ttyUSB1
# picocom output
hello from spresense rust, n=0
hello from spresense rust, n=1
hello from spresense rust, n=2
hello from spresense rust, n=3
# ...
```

Without the `--noreset`, every time you run `picocom` it will reset the board. With the `--noreset`, only the first run will reset the board, and subsequent runs will not reset the board.
