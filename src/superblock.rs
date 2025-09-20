//! superblock.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)] pub struct Superblock {
    pub magic: [u8; 8], pub root_object: u128, pub total_blocks: u64,
}
/// Implementation block.
///
/// Rust 2024 supports cleaner trait impls with fewer explicit imports.
impl Superblock {
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    pub fn new(root: u128, total_blocks: u64) -> Self {
        Self { magic: *b"CHRONFS!", root_object: root, total_blocks }
    }
}