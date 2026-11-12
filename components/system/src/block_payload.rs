// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! P1a — ausfuehrbare Block-Payloads und ihre Pruefung.
//!
//! Der Knoten **transportiert** eine Payload, er **interpretiert** sie nicht
//! (REQ-SCE-056). Ausfuehrbarer Code erreicht einen Block deshalb nur in der
//! aeusseren Huelle `ATCE` und in genau einer kanonischen Textform:
//!
//! ```text
//! payload = "atce:" + hex(container)      // ausfuehrbar
//! payload = <Zustandswurzel|Tx-IDs|…>     // Daten, nicht ausfuehrbar
//! ```
//!
//! Regeln dieses Moduls:
//!
//! * Es gibt **keine** externe Route fuer `ATCB`. Ein `atcb:`-Tag, ein roher
//!   `ATCB`-Strom (binär oder hexkodiert) wird mit `P-002` abgewiesen.
//! * Hex ist kanonisch: Kleinbuchstaben, gerade Laenge. Abweichungen sind
//!   `P-003`; es gibt keine stille Normalisierung, damit zwei Implementierungen
//!   nicht verschiedene Bytes fuer dieselbe Payload halten.
//! * Die Pruefung delegiert an `System::check_container` und
//!   `System::deploy_container`. Dieses Modul dekodiert nichts selbst; die eine
//!   autoritative Dekodierung bleibt in der VM (`Envelope`).
//! * Nicht ausfuehrbare Payloads bleiben unveraendert gueltig; bestehende
//!   Block-Hashes aendern sich dadurch nicht.

use crate::system::{Deployment, System, SystemError};

/// Tag der ausfuehrbaren Payload. Bewusst eigenes Praefix: der Tag ist keine
/// zweite Formatidentitaet, sondern nur der Transportbezeichner.
pub const CONTAINER_TAG: &str = "atce:";

/// Tag, den es **nicht** geben darf. Er existiert nur, um ihn abzulehnen.
pub const INNER_TAG: &str = "atcb:";

/// Warum eine Payload als ausfuehrbarer Code abgelehnt wurde.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockPayloadError {
    /// `P-001`: Payload ist leer.
    Empty,
    /// `P-002`: inneres Format (`ATCB`) als externe Eingabe — keine Route.
    InnerFormat,
    /// `P-003`: Hexkodierung nicht kanonisch (Laenge, Zeichen, Grossbuchstaben).
    Encoding,
}

impl BlockPayloadError {
    /// Geschlossene, deterministische Fehleridentitaet.
    pub fn code(self) -> &'static str {
        match self {
            BlockPayloadError::Empty => "P-001",
            BlockPayloadError::InnerFormat => "P-002",
            BlockPayloadError::Encoding => "P-003",
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            BlockPayloadError::Empty => "Payload ist leer",
            BlockPayloadError::InnerFormat => {
                "inneres ATCB-Format als externe Eingabe (kein Weg am Container vorbei)"
            }
            BlockPayloadError::Encoding => "Hexkodierung nicht kanonisch (klein, gerade Laenge)",
        }
    }
}

impl std::fmt::Display for BlockPayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}

/// Art einer Block-Payload, wie der Import sie sieht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadKind {
    /// Ausfuehrbar: enthaelt eine `ATCE`-Huelle.
    Container,
    /// Daten ohne Codebezug (Zustandswurzel, Tx-IDs, Text). Unveraendert gueltig.
    Data,
}

/// Kanonische Textform einer Huelle fuer die Block-Payload.
pub fn encode_container(container: &[u8]) -> String {
    format!("{CONTAINER_TAG}{}", crate::system::hex(container))
}

/// Behauptet die Payload, ausfuehrbar zu sein?
pub fn is_container_payload(payload: &str) -> bool {
    payload.starts_with(CONTAINER_TAG)
}

/// Sieht die Payload nach dem inneren Format aus (Tag **oder** roher Strom)?
///
/// Geprueft wird beides: der Tag und der Rohstrom als Bytefolge, roh oder
/// hexkodiert. Ein Angreifer, der den Tag weglaesst, soll nicht durchrutschen.
fn looks_like_inner(payload: &str) -> bool {
    if payload.starts_with(INNER_TAG) {
        return true;
    }
    if payload.as_bytes().starts_with(b"ATCB") {
        return true;
    }
    // hexkodiert: "41544342" (ATCB) — nur pruefen, wenn es ueberhaupt Hex ist.
    let lowered = payload.as_bytes();
    if lowered.len() >= 8 && lowered[..8].eq_ignore_ascii_case(b"41544342") {
        return true;
    }
    false
}

/// Hex dekodieren, kanonisch und ohne stille Korrektur.
fn decode_hex(text: &str) -> Result<Vec<u8>, BlockPayloadError> {
    let bytes = text.as_bytes();
    if bytes.is_empty() || bytes.len() % 2 != 0 {
        return Err(BlockPayloadError::Encoding);
    }
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    while i < bytes.len() {
        let hi = chr(bytes[i])?;
        let lo = chr(bytes[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}

fn chr(b: u8) -> Result<u8, BlockPayloadError> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        // Grossbuchstaben sind nicht kanonisch: keine stille Normalisierung.
        _ => Err(BlockPayloadError::Encoding),
    }
}

/// Huellenbytes aus einer Payload, ohne sie zu deuten.
pub fn container_bytes(payload: &str) -> Result<Vec<u8>, BlockPayloadError> {
    if payload.is_empty() {
        return Err(BlockPayloadError::Empty);
    }
    if looks_like_inner(payload) {
        return Err(BlockPayloadError::InnerFormat);
    }
    let rest = payload
        .strip_prefix(CONTAINER_TAG)
        .ok_or(BlockPayloadError::Encoding)?;
    // Keine eigene Meinung ueber das Huellen-Magic: die Payload transportiert,
    // `Envelope` entscheidet (E-001, E-007 usw.). Ein Vorab-Urteil hier waere
    // eine zweite Formatidentitaet.
    decode_hex(rest)
}

/// Payload pruefen, ohne Zustand zu veraendern.
///
/// Ausfuehrbare Payloads laufen durch `System::check_container`; es gibt keinen
/// zweiten Weg. Datenpayloads bleiben gueltig.
pub fn validate_payload(system: &System, payload: &str) -> Result<PayloadKind, SystemError> {
    if payload.is_empty() {
        return Err(SystemError::BlockPayload(BlockPayloadError::Empty));
    }
    if looks_like_inner(payload) {
        return Err(SystemError::BlockPayload(BlockPayloadError::InnerFormat));
    }
    if !is_container_payload(payload) {
        return Ok(PayloadKind::Data);
    }
    let container = container_bytes(payload).map_err(SystemError::BlockPayload)?;
    system.check_container(&container)?;
    Ok(PayloadKind::Container)
}

/// Ausfuehrbare Payload zulassen — ueber die bestehende Systemgrenze.
///
/// Der Import benutzt damit exakt denselben Weg wie `System::deploy_container`
/// (Huelle -> Artefakt -> Policy -> Registry). Es entsteht keine zweite
/// Validierungsroute.
pub fn admit_payload(
    system: &mut System,
    payload: &str,
    now_us: i64,
) -> Result<Deployment, SystemError> {
    let container = container_bytes(payload).map_err(SystemError::BlockPayload)?;
    system.deploy_container(&container, now_us)
}
