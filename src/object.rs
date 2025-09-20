//! object.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)] pub struct Extent { pub start_block: u64, pub length: u64 }
#[derive(Serialize, Deserialize, Debug, Clone)] pub struct Object {
    pub id: u128, pub parent: Option<u128>, pub tags: Vec<String>,
    pub extents: Vec<Extent>, pub history: Vec<crate::versioning::Version>,
}
/// Implementation block.
///
/// Rust 2024 supports cleaner trait impls with fewer explicit imports.
impl Object {
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    pub fn new(id: u128, parent: Option<u128>) -> Self { Self { id, parent, tags: Vec::new(), extents: Vec::new(), history: Vec::new() } }
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    pub fn new_root(id: u128) -> Self { Self::new(id, None) }
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    pub fn add_tag(&mut self, tag: &str) { self.tags.push(tag.to_string()); }
}