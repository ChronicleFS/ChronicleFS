//! block_device.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
/// Block device abstraction for ChronicleFS.
/// Public trait definition.
///
/// Traits in Rust 2024 benefit from improved prelude imports,
/// reducing boilerplate. This trait defines a core interface.
pub trait BlockDevice {
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    fn block_size(&self) -> usize;
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    fn total_blocks(&self) -> u64;
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> Result<(), String>;
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    fn write_block(&mut self, block_id: u64, data: &[u8]) -> Result<(), String>;
}
/// Public struct definition.
///
/// Structs hold data; comments explain each field.
pub struct MemBlockDevice { storage: Vec<u8>, block_size: usize, total_blocks: u64 }
/// Implementation block.
///
/// Rust 2024 supports cleaner trait impls with fewer explicit imports.
impl MemBlockDevice { pub fn new(total_blocks: u64, block_size: usize) -> Self {
    Self { storage: vec![0; (total_blocks * block_size as u64) as usize], block_size, total_blocks }
} }
/// Implementation block.
///
/// Rust 2024 supports cleaner trait impls with fewer explicit imports.
impl BlockDevice for MemBlockDevice {
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    fn block_size(&self) -> usize { self.block_size }
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    fn total_blocks(&self) -> u64 { self.total_blocks }
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> Result<(), String> {
        let start = (block_id as usize) * self.block_size;
        buf.copy_from_slice(&self.storage[start..start+self.block_size]); Ok(())
    }
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
    fn write_block(&mut self, block_id: u64, data: &[u8]) -> Result<(), String> {
        let start = (block_id as usize) * self.block_size;
        self.storage[start..start+self.block_size].copy_from_slice(data); Ok(())
    }
}