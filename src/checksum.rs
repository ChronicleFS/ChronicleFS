//! checksum.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
use blake3::Hasher;
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
pub fn checksum(data: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new(); hasher.update(data);
    let hash = hasher.finalize(); *hash.as_bytes()
}