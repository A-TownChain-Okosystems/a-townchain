// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Bestimmmtes Storage-Layout fuer Vertragszustand (G11 / Schnitt S1).
//!
//! Owner-Entscheidung 08.11.2026: Zustand liegt in **ABI-definierten Slots**
//! fester Breite, nicht in einer zur Laufzeit aufgeloesten Key-Value-Struktur.
//! Damit sind Speicherlayout, Gas-Kalkulation und statische Analyse
//! reproduzierbar, und Feldnamen bleiben reine Metadaten.
//!
//! Regeln dieses Moduls (bewusst klein und fail-closed):
//!
//! - Ein Slot ist **32 Byte** breit, `offset = index * 32`, also ohne Polster.
//! - Die Slot-Reihenfolge ist `(Vertrag, Feld)` lexikographisch sortiert, nie
//!   die Deklarationsreihenfolge. Umsortieren der Quelle aendert keine Adresse.
//! - Ein Schluessel `Vertrag::Feld` darf nur einmal vorkommen (Kollisionsschutz).
//! - Das Layout traegt eine eigene `layout_version`; Aenderungen an Zuordnung
//!   oder Typen erfordern eine neue Version und eine dokumentierte Migration.
//! - Dynamische Typen (`string`, `List`, `Map`) belegen keinen einzelnen
//!   32-Byte-Slot. Sie werden als `Dynamic` gefuehrt und sind in S1 **nicht**
//!   ausfuehrbar; ihre Kodierung ist S4.

use std::collections::BTreeSet;

use crate::ast::{Program, Stmt, TypeHint};
use crate::value::AbiType;

/// Slot-Breite in Byte. Feste Breite ist Teil der Owner-Entscheidung.
pub const SLOT_BYTES: u32 = 32;

/// Layout-Version dieses Moduls. Erhoehen, sobald sich Zuordnung oder Breite
/// aendert; die Version gehoert in das Artefakt (S6) und in die Migration.
pub const LAYOUT_VERSION: u32 = 1;

/// Art eines Slots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotKind {
    /// 32-Byte-Wert mit kanonischem ABI-Typ; in S1 adressierbar.
    Value(AbiType),
    /// 32 Byte breit, aber die Kodierung ist noch nicht kanonisch (S4).
    Opaque(String),
    /// Dynamischer Typ: kein einzelner 32-Byte-Slot, Kodierung offen (S4).
    Dynamic(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageSlot {
    /// Slot-Index, auch der Operand der spaeteren State-Instruktionen.
    pub index: u16,
    /// Byte-Offset im Zustandsspeicher: `index * SLOT_BYTES`.
    pub offset: u32,
    /// Vertragsname (Feldname allein ist nicht adressierbar).
    pub contract: String,
    pub field: String,
    /// Deklarierter ATCLang-Typ, wie er in der Quelle steht.
    pub declared_type: String,
    pub kind: SlotKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageLayout {
    pub version: u32,
    pub slots: Vec<StorageSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    /// Derselbe `Vertrag::Feld`-Schluessel wurde zweimal vergeben.
    DuplicateSlot { key: String, index: u16, previous: u16 },
    /// Mehr Felder als Slots adressierbar sind.
    TooManySlots { count: usize },
    /// Feld ohne Typangabe; fail-closed statt Raten.
    MissingType { key: String },
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateSlot { key, index, previous } => write!(
                f,
                "Storage-Slot doppelt vergeben: {key} (Index {index}, bereits {previous})"
            ),
            Self::TooManySlots { count } => {
                write!(f, "zu viele Zustandsfelder fuer u16-Slots: {count}")
            }
            Self::MissingType { key } => write!(f, "Zustandsfeld ohne Typ: {key}"),
        }
    }
}

impl std::error::Error for StorageError {}

/// Ordnet einen deklarierten Typ einem Slot zu. Unbekannte Typen sind
/// `Opaque`, dynamische Typen `Dynamic` — nichts wird stillschweigend als
/// 32-Byte-Wert angenommen.
fn classify(ty: &TypeHint) -> SlotKind {
    let name = ty.name.trim();
    if let Some(abi) = AbiType::from_name(name) {
        return match abi {
            // `bytes` ist laengenvariabel; der kanonische ABI-Typ existiert,
            // die Slot-Kodierung ist aber noch offen.
            AbiType::Bytes => SlotKind::Opaque(name.to_string()),
            other => SlotKind::Value(other),
        };
    }
    let normalized = name.split('<').next().unwrap_or(name).trim();
    match normalized {
        "string" | "str" | "List" | "Vec" | "Map" | "bytes32" => {
            SlotKind::Dynamic(name.to_string())
        }
        "" => SlotKind::Dynamic(String::new()),
        _ => SlotKind::Opaque(name.to_string()),
    }
}

impl StorageLayout {
    /// Baut das Layout aus einem geparsten Programm.
    ///
    /// Alle Vertraege der Quelldatei teilen sich einen Slot-Raum, damit ein
    /// Feld in jedem Vertrag genau eine Adresse hat.
    pub fn from_program(prog: &Program) -> Result<Self, StorageError> {
        let mut fields: Vec<(String, String, String)> = Vec::new(); // (key, type, field)
        for stmt in &prog.statements {
            if let Stmt::Contract { name, body, .. } = stmt {
                for item in body {
                    match item {
                        Stmt::State { name: field, ty, .. } => {
                            fields.push((format!("{name}::{field}"), ty.name.clone(), field.clone()));
                        }
                        Stmt::StateBlock { fields: block, .. } => {
                            for (field, ty, _) in block {
                                fields.push((
                                    format!("{name}::{field}"),
                                    ty.name.clone(),
                                    field.clone(),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Deterministische Ordnung: lexikographisch nach Schluessel.
        fields.sort_by(|a, b| a.0.cmp(&b.0));

        if fields.len() > u16::MAX as usize {
            return Err(StorageError::TooManySlots { count: fields.len() });
        }

        let mut slots: Vec<StorageSlot> = Vec::with_capacity(fields.len());
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for (key, declared_type, field) in &fields {
            if declared_type.trim().is_empty() {
                return Err(StorageError::MissingType { key: key.clone() });
            }
            let index = slots.len() as u16;
            if !seen.insert(key.as_str()) {
                let previous = slots
                    .iter()
                    .find(|s| format!("{}::{}", s.contract, s.field) == *key)
                    .map(|s| s.index)
                    .unwrap_or(0);
                return Err(StorageError::DuplicateSlot {
                    key: key.clone(),
                    index,
                    previous,
                });
            }
            let (contract, _) = key.split_once("::").unwrap_or((key.as_str(), ""));
            let kind = classify(&TypeHint {
                name: declared_type.clone(),
            });
            slots.push(StorageSlot {
                index,
                offset: index as u32 * SLOT_BYTES,
                contract: contract.to_string(),
                field: field.clone(),
                declared_type: declared_type.clone(),
                kind,
            });
        }

        Ok(Self {
            version: LAYOUT_VERSION,
            slots,
        })
    }

    pub fn slot_count(&self) -> u16 {
        self.slots.len() as u16
    }

    /// Zustandsspeicher in Byte, den dieses Layout belegt.
    pub fn storage_bytes(&self) -> u32 {
        self.slots.len() as u32 * SLOT_BYTES
    }

    pub fn slot_of(&self, contract: &str, field: &str) -> Option<&StorageSlot> {
        self.slots
            .iter()
            .find(|s| s.contract == contract && s.field == field)
    }

    /// Slots, die S1 adressieren kann (32-Byte-Werte mit kanonischem Typ).
    pub fn executable_slots(&self) -> impl Iterator<Item = &StorageSlot> {
        self.slots
            .iter()
            .filter(|s| matches!(s.kind, SlotKind::Value(_)))
    }

    /// Ist der Zustand dieses Layouts in S1 vollstaendig ausfuehrbar?
    pub fn is_s1_executable(&self) -> bool {
        self.slots
            .iter()
            .all(|s| matches!(s.kind, SlotKind::Value(_)))
    }

    /// Kanonische Textform fuer Hashing/Anzeige; stabil und versioniert.
    pub fn canonical(&self) -> String {
        let mut out = format!("atc-storage-layout-v{}\n", self.version);
        for s in &self.slots {
            let kind = match &s.kind {
                SlotKind::Value(t) => format!("value:{t:?}"),
                SlotKind::Opaque(t) => format!("opaque:{t}"),
                SlotKind::Dynamic(t) => format!("dynamic:{t}"),
            };
            out.push_str(&format!(
                "{} {} {} {} {}\n",
                s.index, s.offset, s.contract, s.field, kind
            ));
        }
        out
    }

    /// SHA-256 des kanonischen Layouts. Bindet Artefakt und Migration an genau
    /// diese Zuordnung.
    pub fn layout_hash(&self) -> String {
        use sha2::{Digest, Sha256};
        Sha256::digest(self.canonical().as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    fn layout(src: &str) -> StorageLayout {
        let prog = parse_program(src).expect("Quelle muss parsen");
        StorageLayout::from_program(&prog).expect("Layout muss entstehen")
    }

    #[test]
    fn felder_bekommen_feste_slots_in_sortierter_ordnung() {
        let l = layout(
            "contract Token { state supply: u256 state owner: Address }",
        );
        assert_eq!(l.version, LAYOUT_VERSION);
        assert_eq!(l.slot_count(), 2);
        // Sortiert nach Feldname: owner < supply
        assert_eq!(l.slot_of("Token", "owner").unwrap().index, 0);
        assert_eq!(l.slot_of("Token", "supply").unwrap().index, 1);
        assert_eq!(l.slot_of("Token", "owner").unwrap().offset, 0);
        assert_eq!(l.slot_of("Token", "supply").unwrap().offset, SLOT_BYTES);
        assert_eq!(l.storage_bytes(), 64);
        assert!(matches!(
            l.slot_of("Token", "owner").unwrap().kind,
            SlotKind::Value(AbiType::Address)
        ));
        assert!(matches!(
            l.slot_of("Token", "supply").unwrap().kind,
            SlotKind::Value(AbiType::U256)
        ));
    }

    #[test]
    fn deklarationsreihenfolge_aendert_keine_adresse() {
        let a = layout("contract Token { state a: u64 state b: u64 }");
        let b = layout("contract Token { state b: u64 state a: u64 }");
        assert_eq!(a.slot_of("Token", "a").unwrap().index, b.slot_of("Token", "a").unwrap().index);
        assert_eq!(a.layout_hash(), b.layout_hash());
    }

    #[test]
    fn vertraege_teilen_sich_einen_slot_raum_ohne_kollision() {
        let l = layout(
            "contract Alpha { state x: u64 } contract Beta { state x: u64 }",
        );
        assert_eq!(l.slot_count(), 2);
        assert_ne!(
            l.slot_of("Alpha", "x").unwrap().index,
            l.slot_of("Beta", "x").unwrap().index
        );
    }

    #[test]
    fn state_block_wird_ebenfalls_erfasst() {
        let l = layout("contract Vault { state { a: u64; b: bool } }");
        assert_eq!(l.slot_count(), 2);
        assert!(matches!(
            l.slot_of("Vault", "b").unwrap().kind,
            SlotKind::Value(AbiType::Bool)
        ));
    }

    #[test]
    fn dynamische_typen_sind_nicht_s1_ausfuehrbar() {
        let l = layout("contract Doc { state name: string state count: u64 }");
        assert!(matches!(
            l.slot_of("Doc", "name").unwrap().kind,
            SlotKind::Dynamic(_)
        ));
        assert!(!l.is_s1_executable());
        assert_eq!(l.executable_slots().count(), 1);
    }

    #[test]
    fn unbekannter_typ_wird_opaque_statt_geraten() {
        let l = layout("contract X { state v: Verdict }");
        assert!(matches!(
            l.slot_of("X", "v").unwrap().kind,
            SlotKind::Opaque(ref t) if t == "Verdict"
        ));
        assert!(!l.is_s1_executable());
    }

    #[test]
    fn layout_hash_ist_reproduzierbar_und_versionsabhaengig() {
        let l = layout("contract Token { state owner: Address }");
        let h = l.layout_hash();
        assert_eq!(h.len(), 64);
        assert_eq!(h, layout("contract Token { state owner: Address }").layout_hash());
        assert_ne!(
            h,
            layout("contract Token { state owner: Address state extra: u64 }").layout_hash()
        );
        assert!(l.canonical().starts_with("atc-storage-layout-v1"));
    }

    #[test]
    fn echter_basisvertrag_hat_kollisionsfreie_slots() {
        let src = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../../apps/atc-contracts/base/base_contract.atc"),
        );
        let Ok(src) = src else { return }; // Pfad ausserhalb des Checkouts
        let l = layout(&src);
        assert!(l.slot_count() > 0, "Basisvertrag sollte Zustand haben");
        let mut seen = BTreeSet::new();
        for s in &l.slots {
            assert!(seen.insert((s.contract.clone(), s.field.clone())), "Kollision {}", s.field);
            assert_eq!(s.offset, s.index as u32 * SLOT_BYTES);
        }
    }
}
