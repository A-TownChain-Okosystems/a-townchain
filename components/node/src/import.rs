// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! P1b — Import fremder Bloecke ueber einen expliziten Validator-Port.
//!
//! Der Knoten **dekodiert nichts** (REQ-SCE-056): er kennt weder `ATCE` noch
//! `ATCB` und keinen Opcode. Er fragt einen Port, der ihm die eine Frage
//! beantwortet, die er stellen darf: *darf diese Payload als ausfuehrbarer Code
//! angenommen werden?*
//!
//! Trait-Position (Owner-Entscheidung 12.11.2026, Variante C): der Port liegt in
//! `atc-node`, weil `atc-system` bereits von `atc-node` abhaengt. Damit
//! implementiert `atc-system` den Trait ueber eine **bestehende** Kante; es
//! entsteht keine Gegenkante und kein Zyklus.
//!
//! Reihenfolge der Pruefung, erstes Problem gewinnt:
//!
//! 1. Blockhash gegen den neu berechneten Hash (`I-002`).
//! 2. Verkettung: Hoehe und Vorgaenger-Hash (`I-003`).
//! 3. Kapazitaet der Devnet-Kette (`I-004`).
//! 4. Validator vorhanden? Sonst `I-001` — **fail-closed**, jeder Fremdblock
//!    wird ohne Validator abgelehnt.
//! 5. Payload ueber den Port (`I-005` mit der Fehleridentitaet des Validators).
//!
//! Erst danach wird angehaengt. Ein abgelehnter Block aendert weder Chain-State
//! noch irgendeine Registry.

use crate::chain::{Block, Chain};

/// Ein Validator lehnt eine Payload ab. `(stage, code, offset)` ist die
/// Fehleridentitaet; Meldungstexte sind nicht normativ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadRejection {
    pub stage: &'static str,
    pub code: &'static str,
    pub offset: Option<usize>,
}

impl PayloadRejection {
    pub fn new(stage: &'static str, code: &'static str) -> Self {
        PayloadRejection {
            stage,
            code,
            offset: None,
        }
    }

    pub fn with_offset(stage: &'static str, code: &'static str, offset: usize) -> Self {
        PayloadRejection {
            stage,
            code,
            offset: Some(offset),
        }
    }
}

impl std::fmt::Display for PayloadRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.offset {
            Some(o) => write!(f, "{}/{} bei {}", self.stage, self.code, o),
            None => write!(f, "{}/{}", self.stage, self.code),
        }
    }
}

/// Der Port. Bewusst minimal: eine Methode, keine Formatbegriffe, kein Parser.
pub trait ContainerValidator {
    fn validate_block_payload(&self, payload: &str) -> Result<(), PayloadRejection>;
}

/// Warum ein fremder Block nicht angenommen wurde.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportError {
    /// `I-001`: kein Validator verdrahtet — fail-closed.
    NoValidator,
    /// `I-002`: Blockhash passt nicht zum Inhalt.
    BlockHash { expected: [u8; 32], found: [u8; 32] },
    /// `I-003`: Verkettung (Hoehe/Vorgaenger) passt nicht.
    Linkage(&'static str),
    /// `I-004`: Devnet-Kapazitaet erschoepft.
    Capacity,
    /// `I-005`: Validator hat die Payload abgelehnt.
    Payload(PayloadRejection),
}

impl ImportError {
    /// Geschlossene Fehleridentitaet `I-001` bis `I-005`.
    pub fn code(&self) -> &'static str {
        match self {
            ImportError::NoValidator => "I-001",
            ImportError::BlockHash { .. } => "I-002",
            ImportError::Linkage(_) => "I-003",
            ImportError::Capacity => "I-004",
            ImportError::Payload(_) => "I-005",
        }
    }
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::NoValidator => write!(
                f,
                "I-001: kein Validator verdrahtet — Fremdblock wird nicht angenommen"
            ),
            ImportError::BlockHash { expected, found } => write!(
                f,
                "I-002: Blockhash inkonsistent (erwartet {}, gefunden {})",
                crate::chain::hex_32(expected),
                crate::chain::hex_32(found)
            ),
            ImportError::Linkage(reason) => write!(f, "I-003: {reason}"),
            ImportError::Capacity => write!(f, "I-004: Devnet-Kette voll (64 Bloecke)"),
            ImportError::Payload(r) => write!(f, "I-005: Payload abgelehnt: {r}"),
        }
    }
}

impl std::error::Error for ImportError {}

impl Chain {
    /// Nimmt einen fremden Block an — nur ueber diesen Weg, nur mit Validator.
    pub fn import_block(
        &mut self,
        block: Block,
        validator: Option<&dyn ContainerValidator>,
    ) -> Result<(), ImportError> {
        let recomputed = block.recompute_hash();
        if recomputed != block.hash {
            return Err(ImportError::BlockHash {
                expected: recomputed,
                found: block.hash,
            });
        }

        let (best_height, best_hash) = {
            let b = self.best();
            (b.height, b.hash)
        };
        if block.height != best_height + 1 {
            return Err(ImportError::Linkage(
                "Hoehe springt gegenueber dem Best-Block",
            ));
        }
        if block.prev_hash != best_hash {
            return Err(ImportError::Linkage(
                "Vorgaenger-Hash passt nicht zum Best-Block",
            ));
        }
        if !self.has_capacity() {
            return Err(ImportError::Capacity);
        }

        let validator = validator.ok_or(ImportError::NoValidator)?;
        validator
            .validate_block_payload(&block.payload)
            .map_err(ImportError::Payload)?;

        self.append_validated(block);
        Ok(())
    }
}
