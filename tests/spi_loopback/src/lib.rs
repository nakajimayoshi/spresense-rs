//! Support crate for the `spi_loopback` on-hardware test.
//!
//! The actual test is the `harness = false` integration test in `tests/spi.rs`
//! (run with `cargo test --release --test spi`). This library target exists
//! only to give the package something to build; it intentionally has no code.
#![no_std]
