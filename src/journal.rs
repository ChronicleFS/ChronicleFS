//! journal.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
/// Public struct definition.
///
/// Structs hold data; comments explain each field.
pub struct JournalEntry { pub block_id: u64, pub data: Vec<u8> }
/// Public struct definition.
///
/// Structs hold data; comments explain each field.
pub struct Journal { pub entries: Vec<JournalEntry> }
/// Implementation block.
///
/// Rust 2024 supports cleaner trait impls with fewer explicit imports.
impl Journal {
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    pub fn new() -> Self { Self { entries: Vec::new() } }
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    pub fn log_write(&mut self, block_id: u64, data: Vec<u8>) {
        self.entries.push(JournalEntry { block_id, data });
    }
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    pub fn commit<D: crate::block_device::BlockDevice>(&mut self, dev: &mut D) -> Result<(), String> {
        for e in &self.entries { dev.write_block(e.block_id, &e.data)?; }
        self.entries.clear(); Ok(())
    }
}