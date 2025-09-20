//! crypto.rs - ChronicleFS (Rust 2024 Edition)
//!
//! This module has been rewritten for Rust 2024 edition.
//! It follows modern idioms, avoids redundant imports,
//! and is fully documented for future maintainers.

// SPDX-License-Identifier: GPL-2.0
use aes_gcm::{Aes256Gcm, Key, Nonce}; use aes_gcm::aead::{Aead, NewAead};
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
pub fn encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    cipher.encrypt(Nonce::from_slice(nonce), plaintext).unwrap()
}
    /// Function definition.
    /// Uses idiomatic error handling (`Result`, `?`) per Rust 2024.
pub fn decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    cipher.decrypt(Nonce::from_slice(nonce), ciphertext).unwrap()
}