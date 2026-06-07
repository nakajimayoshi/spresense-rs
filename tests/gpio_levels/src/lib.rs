//! Support crate for the `gpio_levels` on-hardware test.
//!
//! The actual test is the `harness = false` integration test in `tests/gpio.rs`
//! (run with `cargo test --release --test gpio`). This library target exists
//! only to give the package something to build; it intentionally has no code.
#![no_std]
