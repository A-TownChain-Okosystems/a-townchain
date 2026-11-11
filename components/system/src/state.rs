// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Vertragszustand und Zustandswurzel.
//!
//! Der Zustand eines Vertrags ist die Speicherbelegung der VM: eine Folge von
//! `u64`-Woertern. Die Wurzel bindet **Code und Zustand** zusammen, damit zwei
//! Ausfuehrungen nur dann dieselbe Wurzel ergeben, wenn sie mit demselben Code
//! zum selben Zustand gekommen sind.

use atc_vm::artifact::sha256_daten;

/// Domain-Trennung der Zustandswurzel.
pub const STATE_DOMAIN: &[u8] = b"ATC-STATE-V1\0";

/// Zustand eines Vertrags (Speicherwoerter der VM).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ContractState {
    words: Vec<u64>,
}

impl ContractState {
    pub fn new() -> Self {
        ContractState { words: Vec::new() }
    }

    pub fn from_words(words: Vec<u64>) -> Self {
        ContractState { words }
    }

    pub fn words(&self) -> &[u64] {
        &self.words
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// Zustandswurzel: SHA-256 ueber Domain, Code-Hash, Wortzahl und Woerter
    /// (Little Endian). Deterministisch und ohne Host-Zeit oder Zufall.
    pub fn state_root(&self, code_hash: &[u8; 32]) -> [u8; 32] {
        let mut buffer = Vec::with_capacity(STATE_DOMAIN.len() + 32 + 4 + self.words.len() * 8);
        buffer.extend_from_slice(STATE_DOMAIN);
        buffer.extend_from_slice(code_hash);
        buffer.extend_from_slice(&(self.words.len() as u32).to_le_bytes());
        for word in &self.words {
            buffer.extend_from_slice(&word.to_le_bytes());
        }
        sha256_daten(&buffer)
    }
}
