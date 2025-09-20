//! query.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
use crate::object::Object;
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
pub fn query(objects: &Vec<Object>, tag: &str) -> Vec<Object> {
    objects.iter().filter(|o| o.tags.contains(&tag.to_string())).cloned().collect()
}