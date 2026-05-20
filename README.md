# (WIP) Peripheral Access Crate for CXD5602

Collection of notes and outputs for setting up a PAC for the CXD5602 from the Sony SPRESENSE Main Board

## Notes

### Prior Art

[Embassy PR](https://github.com/embassy-rs/embassy/pull/3926)

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

## Flashing manually with SDK tools

This section describes how to flash a Rust binary using only the SDK's own
tools (`mkspk` and `flash_writer`). This is useful when you want to understand
the underlying process or when installing the Rust tools is not practical.

### Prerequisites

The SDK's `mkspk` ships as C source and must be built once:

```bash
make -C $SPRESENSE_SDK/nuttx/tools/cxd56 -f Makefile.host TOPDIR=$SPRESENSE_SDK/nuttx
```

### Step 1 — Build the Rust binary

```bash
cargo build --release \
  --target thumbv7em-none-eabihf \
  --features rt \
  --example gpio_blink      # or --bin <name>
```

The ELF lands at `target/thumbv7em-none-eabihf/release/examples/gpio_blink`.

### Step 2 — Package into an SPK

```bash
$SPRESENSE_SDK/nuttx/tools/cxd56/mkspk \
  -c 2 \
  target/thumbv7em-none-eabihf/release/examples/gpio_blink \
  nuttx \
  gpio_blink.spk
```

_IMPORTANT_: Must be called nuttx! Hard-coded check by bootloader

`-c 2` selects the M0P application core. The second positional argument is the
install name stored in the package header (max 63 bytes).

### Step 3 — Flash

```bash
$SPRESENSE_SDK/sdk/tools/flash.sh \
  -c /dev/ttyUSB0 \
  gpio_blink.spk
```
