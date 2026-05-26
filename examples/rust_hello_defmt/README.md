# defmt Hello world on the Spresense with Rust

This is a defmt-based logging example using the [`cxd56-hal`](../../cxd56-hal/) and the [`defmt-serial`](https://github.com/gauteh/defmt-serial) crate. In this example, the onboard LED closest to the center of the board will blink three times (SOS) to indicate start up. Then it will log "hello from spresense rust, n={n}" via `defmt` over UART1, followed by a strobe blinking of the LED.

defmt provides compact binary-encoded log messages with interned strings, making logs smaller and faster than ASCII formatting. The binary frames are decoded on the host with `defmt-print`.

## Build and flash

Must build the binary with `DEFMT_LOG` variable set to the correct level or no printing will show.

```bash
DEFMT_LOG=debug cargo build --release \
    && mkspk -c 2 target/thumbv7em-none-eabihf/release/rust_hello_defmt nuttx target/rust_hello_defmt.spk \
    && flash_writer -c /dev/ttyUSB1 target/rust_hello_defmt.spk 
```

## Observing defmt output on Linux

The binary defmt frames must be decoded using `defmt-print`. Use `socat` to pipe the raw serial data to `defmt-print`:

```bash
$ cargo install defmt-print
$ sudo dmesg | tail  # Find out the connected serial port
[  483.786639] usb 1-2.2: cp210x converter now attached to ttyUSB1
$ socat -u /dev/ttyUSB1,rawer,b115200 STDOUT | defmt-print -e target/thumbv7em-none-eabihf/release/rust_hello_defmt
# defmt-print output
INFO  hello from spresense rust, n=37
INFO  hello from spresense rust, n=38
INFO  hello from spresense rust, n=39
INFO  hello from spresense rust, n=40
INFO  hello from spresense rust, n=41
# ...
```

NOTE: `picocom` or other ASCII-only terminals will show garbled binary data. Use the `socat | defmt-print` pipeline to decode the output.
NOTE: Likely the board will get reset when you connect.
