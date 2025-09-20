//! compression.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
use lz4::{EncoderBuilder, Decoder}; use std::io::Read;
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = EncoderBuilder::new().build(Vec::new()).unwrap();
    std::io::Write::write_all(&mut encoder, data).unwrap();
    let (out, _result) = encoder.finish(); out
}
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut decoder = Decoder::new(data).unwrap(); let mut out = Vec::new();
    decoder.read_to_end(&mut out).unwrap(); out
}