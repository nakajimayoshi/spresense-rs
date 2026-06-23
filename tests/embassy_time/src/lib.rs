//! Support crate for the `embassy_time` on-hardware test.
//!
//! The actual test is the `harness = false` integration test in `tests/time.rs`
//! (run with `cargo test --release --test time`). This library target exists only to
//! give the package something to build; it intentionally has no code.
#![no_std]
