# (WIP) Peripheral Access Crate for CXD5602

Collection of notes and outputs for setting up a PAC for the CXD5602 from the Sony SPRESENSE Main Board

## Notes

### Prior Art

[Embassy PR](https://github.com/embassy-rs/embassy/pull/3926)

### Relevant Repos

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

Some of the enumerateValues names are not valid identifiers in Rust

- Change "16 .. 240 range" to "FROM16TO240range"
  - the ".." can't be inside an identifier
- Change "don't" to "dont"
  - the "'" can't be inside an identifier

#### svd2rust

```bash
svd2rust -i cxd5602-fixed.svd
form -i lib.rs -o src/
```

#### `chiptool`

```bash
chiptool generate --svd cxd5602-fixed.svd
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
