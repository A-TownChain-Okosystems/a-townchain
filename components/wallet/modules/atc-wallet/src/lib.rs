//! ATCLang wallet integration module.
//!
//! Rust wallet functionality is owned by the canonical `components/wallet`
//! crate. This module intentionally re-exports that implementation instead of
//! maintaining a second wallet core.

pub use atc_wallet::{balance, gui, history, keys, tx};
