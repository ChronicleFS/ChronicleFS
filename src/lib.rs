//! lib.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
//! ChronicleFS library entry point
//! Exports all filesystem modules.

pub mod block_device;
pub mod journal;
pub mod object;
pub mod superblock;
pub mod query;
pub mod compression;
pub mod checksum;
pub mod versioning;
pub mod crypto;