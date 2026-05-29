# CXD5602 Rust PAC/HAL — Claude Instructions

## Project overview

This is a Rust PAC (Peripheral Access Crate) and HAL for the Sony CXD5602 SoC
(the chip on the Spresense board). There are two PAC flavors generated from the
same SVD: `cxd56-pac-chiptool` (embassy-style register API via chiptool) and
`cxd56-pac-svd2rust` (svd2rust API). `cxd56-hal` and `embassy-cxd56` sit on top.

## Looking up CXD5602 hardware details

**Always launch subagents** when you need authoritative information about the
CXD5602 — register addresses, interrupt numbers, bit layouts, or pin functions.
Do not guess or rely on training data. Two primary sources are available locally:

### 1. Spresense SDK / NuttX port — `/mnt/hotswap/src/spresense-1/`

The NuttX C drivers are the most reliable per-peripheral reference. Key paths:

- `nuttx/arch/arm/src/cxd56xx/hardware/cxd5602_topreg.h` — TOPREG and TOPREG_SUB
  base addresses and all register offsets (PMU, clock, IOCELL, GPIO, interrupt slots)
- `nuttx/arch/arm/src/cxd56xx/hardware/cxd5602_memorymap.h` — every peripheral
  base address (`CXD56_*_BASE` macros)
- `nuttx/arch/arm/include/cxd56xx/irq.h` — IRQ numbers (`CXD56_IRQ_EXTINT = 16`;
  every peripheral IRQ = 16 + source index)
- `nuttx/arch/arm/src/cxd56xx/hardware/cxd56_i2c.h` — DW_apb_i2c register offsets
  and bit definitions (used by all three CXD56 I2C controllers)
- `nuttx/arch/arm/src/cxd56xx/cxd56_gpioint.c` + `cxd56_gpioint.h` — GPIO
  interrupt slot machinery (EXDEVICE_0..11, PMU_WAKE_TRIG field packing)
- `nuttx/arch/arm/src/cxd56xx/cxd56_i2c.c` — I2C controller driver (base address
  selection, IRQ assignment, register usage)
- `nuttx/arch/arm/src/cxd56xx/cxd56_pinconfig.c` — pin mode-mux configuration

### 2. PDF documentation — `documentation/`

Three PDFs ship with the project:

- `p-28_CXD5602_user_manual.pdf` — register maps, memory layout, peripheral
  descriptions, interrupt tables (§3.x sections per peripheral)
- `p-28_CXD5602GG_technical_manual.pdf` — electrical and timing specifications
- `Spresense_pin_function_en-1.pdf` — pin function table and mux options

Use these to confirm base addresses, register reset values, and address-block
boundaries that may not be explicit in the NuttX source.

### Subagent guidance

Launch an **Explore** subagent (or multiple in parallel) with a specific question
and the relevant file paths above. Provide enough context in the prompt so the
agent can make judgment calls. Do not perform the same search yourself if you
delegate it — trust the agent's result and relay findings to the user.

## SVD and PAC workflow

The SVD pipeline is:

```
svd/cxd5602.svd        (base, upstream)
      ↓  svdtools patch svd/patch.yml
svd/cxd5602.svd.patched   (generated — NEVER edit directly)
      ↓  just chiptool   or   just svd2rust
cxd56-pac-chiptool/    cxd56-pac-svd2rust/
```

**`svd/patch.yml` is the authoritative source for all SVD changes.** All edits
to register definitions, peripherals, and interrupt declarations go there.
After editing `patch.yml`, run `just chiptool` then `just svd2rust` to regenerate.

### Key SVD conventions

- **Interrupt values** use the NuttX full-vector index: peripheral IRQ = `CXD56_IRQ_EXTINT(16) + source_offset`. Example: SCU_I2C0 = 16+17 = 33; EXDEVICE_0 = 16+20 = 36.
- **`interrupts:` key** (not `_interrupts:`) is the correct svdtools syntax for
  declaring interrupt vectors on a peripheral, including peripherals added via `_add:`.
- **Register field style**: `addressOffset`, `access`, `resetValue`, `fields` with
  `bitOffset`/`bitWidth`/`enumeratedValues`. Follow the existing TOPREG/I2C0 pattern.

### Currently modeled peripherals

| Peripheral | Base | IRQ(s) | Notes |
|---|---|---|---|
| TOPREG | 0x04100000 | EXDEVICE_0..11 (36–47) | Clock/PMU/GPIO interrupt config |
| TOPREG_SUB | 0x04103000 | — | GPIO interrupt status/clear/raw |
| I2C0 (SCU_I2C0) | 0x0418D400 | SCU_I2C0 (33) | Full DW_apb_i2c register set |
| SPI3 (SCU_SPI) | 0x0418D000 | SPI3 (32) | addressBlock trimmed to 0x400 |
| SPI0, SPI4, SPI5 | various | 90, 129, 125 | |
| UART1, UART2 | various | 27, 127 | CP2102N wired to UART1 (not UART2) |
| DMAC1, DMAC3 | various | 134, 126 | |

## Hardware notes

- **Serial console**: the CP2102N USB-serial adapter is wired to **UART1**, not UART2. UART2 goes to the JP1 extension header.
- **GPIO interrupt slots**: 12 hardware slots (EXDEVICE_0..11). A pin must be assigned to a slot via `IOCSYS_INTSEL`/`IOCAPP_INTSEL` before it can interrupt. SYS-domain pins use slots 0–5; APP-domain pins use slots 6–11.
- **Interrupt routing**: GPIO interrupts route through the PMU wake-trigger logic. Edge modes use `PMU_LATCH` route and require writing `PMU_WAKE_TRIG0_CLR` to clear the latch in the ISR.
- **SWD**: disabled by default per the boot ROM. Exposed on CN4[R] pins 16/18/100 (1.8 V). See `DEVELOPMENT.md` for OpenOCD setup.
