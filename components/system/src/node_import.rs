// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! P1b — Validator-Adapter: der Knoten fragt, `atc-system` antwortet.
//!
//! Der Port `ContainerValidator` liegt in `atc-node` (Variante C, Owner
//! 12.11.2026). Dieses Modul implementiert ihn und **delegiert vollstaendig** an
//! die bestehende System-Ingress-Kette (`block_payload::validate_payload` →
//! `System::check_container`, Huelle `ATCE`). Es gibt hier keinen zweiten Parser,
//! keine zweite Dekodierung und keine eigene Formatkenntnis: die Antwort ist
//! genau das Ergebnis der Systempruefung, nur in die Fehleridentitaet des Ports
//! abgebildet.

use atc_node::import::{ContainerValidator, PayloadRejection};

use crate::block_payload::{self, PayloadKind};
use crate::system::{System, SystemError};

/// Fehlercode fuer Systemfehler, die keine geschlossene Codeliste mitbringen.
/// Bewusst als eigener Wert sichtbar, damit die Abbildung nicht als
/// Formatidentitaet missverstanden wird.
pub const UNCLASSIFIED: &str = "S-000";

/// Adapter: prueft Block-Payloads ueber die Systemgrenze.
pub struct SystemPayloadValidator<'a> {
    system: &'a System,
}

impl<'a> SystemPayloadValidator<'a> {
    pub fn new(system: &'a System) -> Self {
        SystemPayloadValidator { system }
    }

    /// Ergebnis der Delegation, unveraendert durchgereicht (kein zweiter Weg).
    pub fn payload_kind(&self, payload: &str) -> Result<PayloadKind, SystemError> {
        block_payload::validate_payload(self.system, payload)
    }
}

/// `SystemError` → `PayloadRejection`, ohne die Identitaet zu erfinden:
/// Huellenfehler behalten `(Code, Offset)` der Huelle, Payload-Fehler ihren
/// `P-xxx`-Code, Freigabefehler ihren `ADM`-Code.
fn rejection(e: &SystemError) -> PayloadRejection {
    match e {
        SystemError::Container(env) => {
            PayloadRejection::with_offset("container", env.code(), env.offset())
        }
        SystemError::BlockPayload(bp) => PayloadRejection::new("block_payload", bp.code()),
        SystemError::Admission { code, .. } => {
            PayloadRejection::new("admission", code.unwrap_or(UNCLASSIFIED))
        }
        other => PayloadRejection::new(other.stage(), UNCLASSIFIED),
    }
}

impl<'a> ContainerValidator for SystemPayloadValidator<'a> {
    fn validate_block_payload(&self, payload: &str) -> Result<(), PayloadRejection> {
        self.payload_kind(payload)
            .map(|_| ())
            .map_err(|e| rejection(&e))
    }
}
