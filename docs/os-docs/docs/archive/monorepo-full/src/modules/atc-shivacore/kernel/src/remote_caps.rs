// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Remote-Capability-Tickets (RCT) (Rust).
//!
//! Portiert remote_capability.py (Python, 08.07.2026) nach Rust.
//! Erlaubt es, eine Capability kryptographisch signiert an einen fremden
//! Knoten (DID) zu delegieren — ohne zentrale Vermittlungsinstanz.
//!
//! Eigenschaften:
//! - Signaturpruefung: jedes Ticket wird kryptographisch verifiziert
//! - Replay-Schutz: Nonce-Store verhindert Wiedereinspielung
//! - Delegationsketten: mehrstufige Delegation mit Attenuation
//! - Constraints: max_operations, deadline, energy_budget

use alloc::vec;
extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::capability::Rights;
use crate::did::{Did, CryptoProvider};

/// Fehler bei Ticket-Operationen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketError {
    InvalidSignature,
    WrongSubject,
    Replay,
    Expired,
    ConstraintsTooStrict,
    ChainBroken,
    ChainRightsExpanded,
    ChainOpsExpanded,
    ChainDeadlineExtended,
    ResourceMismatch,
    EmptyChain,
    MaxOperationsExceeded,
    CapabilityRevoked,
}

/// Ressourcen-Deskriktor im Ticket
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceDescriptor {
    pub resource_type: String,
    pub resource_id: u64,
    pub rights: Rights,
}

/// Einschraenkungen fuer ein Ticket
#[derive(Debug, Clone, PartialEq)]
pub struct Constraints {
    pub max_operations: u32,
    pub deadline_unix: f64,
    pub energy_budget_uj: Option<u64>,
}

/// Remote-Capability-Ticket — kryptographisch signierte Delegation
#[derive(Debug, Clone)]
pub struct RemoteCapabilityTicket {
    pub issuer_did: Did,
    pub subject_did: Did,
    pub resource: ResourceDescriptor,
    pub constraints: Constraints,
    pub nonce: String,
    pub issuer_signature: Vec<u8>,
    pub parent_ticket_nonce: Option<String>,
}

impl RemoteCapabilityTicket {
    /// Deterministische Byte-Repraesentation fuer Signatur/Verifikation.
    pub fn signing_payload(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(self.issuer_did.value.as_bytes());
        payload.extend_from_slice(self.subject_did.value.as_bytes());
        payload.extend_from_slice(self.resource.resource_type.as_bytes());
        payload.extend_from_slice(&self.resource.resource_id.to_le_bytes());
        payload.push(self.resource.rights.0);
        payload.extend_from_slice(&self.constraints.max_operations.to_le_bytes());
        payload.extend_from_slice(&self.constraints.deadline_unix.to_le_bytes());
        if let Some(eb) = self.constraints.energy_budget_uj {
            payload.extend_from_slice(&eb.to_le_bytes());
            payload.push(1);
        } else {
            payload.push(0);
        }
        payload.extend_from_slice(self.nonce.as_bytes());
        if let Some(ref parent) = self.parent_ticket_nonce {
            payload.extend_from_slice(parent.as_bytes());
            payload.push(1);
        } else {
            payload.push(0);
        }
        payload
    }
}

/// Erzeugt ein neues RCT (vom Issuer signiert)
pub fn issue_ticket<C: CryptoProvider>(
    issuer: &C,
    subject_did: Did,
    resource: ResourceDescriptor,
    constraints: Constraints,
    parent_ticket_nonce: Option<String>,
    nonce: String,
) -> RemoteCapabilityTicket {
    let unsigned = RemoteCapabilityTicket {
        issuer_did: issuer.did().clone(),
        subject_did: subject_did.clone(),
        resource: resource.clone(),
        constraints: constraints.clone(),
        nonce: nonce.clone(),
        issuer_signature: Vec::new(),
        parent_ticket_nonce: parent_ticket_nonce,
    };
    let signature = issuer.sign(&unsigned.signing_payload());
    RemoteCapabilityTicket {
        issuer_did: issuer.did().clone(),
        subject_did,
        resource,
        constraints,
        nonce,
        issuer_signature: signature,
        parent_ticket_nonce: unsigned.parent_ticket_nonce,
    }
}

/// Lokale Capability nach erfolgreicher Ticket-Einloesung
#[derive(Debug, Clone, PartialEq)]
pub struct LocalCap {
    pub cap_id: String,
    pub resource: ResourceDescriptor,
    pub constraints: Constraints,
    pub issuer_did: Did,
    pub operations_used: u32,
    pub revoked: bool,
}

impl LocalCap {
    pub fn new(cap_id: String, resource: ResourceDescriptor, constraints: Constraints, issuer_did: Did) -> Self {
        Self { cap_id, resource, constraints, issuer_did, operations_used: 0, revoked: false }
    }

    /// Verbraucht eine Operation. Widerruft bei Ueberschreitung.
    pub fn consume_operation(&mut self) -> Result<(), TicketError> {
        if self.revoked {
            return Err(TicketError::CapabilityRevoked);
        }
        if self.operations_used >= self.constraints.max_operations {
            self.revoked = true;
            return Err(TicketError::MaxOperationsExceeded);
        }
        self.operations_used += 1;
        Ok(())
    }
}

/// Replay-Schutz: Nonce-Store
pub struct NonceStore {
    seen: BTreeSet<String>,
}

impl NonceStore {
    pub fn new() -> Self { Self { seen: BTreeSet::new() } }

    /// Gibt true zurueck wenn der Nonce NEU ist (und merkt ihn sich).
    /// false bei Replay-Versuch.
    pub fn check_and_record(&mut self, nonce: &str) -> bool {
        if self.seen.contains(nonce) { return false; }
        self.seen.insert(nonce.to_string());
        true
    }
}

/// Remote-Capability-Resolver — laeuft als isolierter Userspace-Dienst
pub struct RemoteCapabilityResolver {
    own_did: Did,
    nonces: NonceStore,
    current_time: f64,  // Injektierbar fuer Tests
}

impl RemoteCapabilityResolver {
    pub fn new(own_did: Did) -> Self {
        Self { own_did, nonces: NonceStore::new(), current_time: 1_000_000.0 }
    }

    pub fn set_time(&mut self, t: f64) { self.current_time = t; }

    /// Validiert ein eingehendes Ticket vollstaendig und erzeugt eine lokale Capability.
    pub fn resolve<C: CryptoProvider>(&mut self, ticket: &RemoteCapabilityTicket) -> Result<LocalCap, TicketError> {
        // 1. Signatur pruefen
        if !C::verify(&ticket.issuer_did, &ticket.signing_payload(), &ticket.issuer_signature) {
            return Err(TicketError::InvalidSignature);
        }
        // 2. Subject pruefen
        if ticket.subject_did != self.own_did {
            return Err(TicketError::WrongSubject);
        }
        // 3. Replay-Schutz
        if !self.nonces.check_and_record(&ticket.nonce) {
            return Err(TicketError::Replay);
        }
        // 4. Deadline pruefen
        if self.current_time > ticket.constraints.deadline_unix {
            return Err(TicketError::Expired);
        }
        // 5. Constraints pruefen
        if ticket.constraints.max_operations == 0 {
            return Err(TicketError::ConstraintsTooStrict);
        }

        Ok(LocalCap::new(
            format!("rct-{}", &ticket.nonce),
            ticket.resource.clone(),
            ticket.constraints.clone(),
            ticket.issuer_did.clone(),
        ))
    }

    /// Mehrstufige Delegation (Bob -> Charlie -> Alice).
    /// Jedes Ticket muss gueltig sein UND darf Rechte nur einschraenken (Attenuation).
    pub fn resolve_chain<C: CryptoProvider>(
        &mut self,
        chain: &[RemoteCapabilityTicket],
    ) -> Result<LocalCap, TicketError> {
        if chain.is_empty() {
            return Err(TicketError::EmptyChain);
        }

        // 1. Kettenintegritaet pruefen (Attenuation)
        for i in 1..chain.len() {
            let parent = &chain[i - 1];
            let child = &chain[i];

            // Kettenbruch: child referenziert nicht parent
            if child.parent_ticket_nonce.as_deref() != Some(&parent.nonce) {
                return Err(TicketError::ChainBroken);
            }
            // Andere Ressource
            if child.resource.resource_type != parent.resource.resource_type
                || child.resource.resource_id != parent.resource.resource_id {
                return Err(TicketError::ResourceMismatch);
            }
            // Rechte erweitert (Attenuation-Verletzung)
            if (child.resource.rights.0 & !parent.resource.rights.0) != 0 {
                return Err(TicketError::ChainRightsExpanded);
            }
            // max_operations erweitert
            if child.constraints.max_operations > parent.constraints.max_operations {
                return Err(TicketError::ChainOpsExpanded);
            }
            // Deadline erweitert
            if child.constraints.deadline_unix > parent.constraints.deadline_unix {
                return Err(TicketError::ChainDeadlineExtended);
            }
        }

        // 2. Alle Kettenglieder signaturgueltig?
        for t in &chain[..chain.len() - 1] {
            if !C::verify(&t.issuer_did, &t.signing_payload(), &t.issuer_signature) {
                return Err(TicketError::InvalidSignature);
            }
        }

        // 3. Letztes Ticket einloesen
        self.resolve::<C>(&chain[chain.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::did::SoftwareSigner;

    fn alice() -> SoftwareSigner { SoftwareSigner::new("alice") }
    fn bob() -> SoftwareSigner { SoftwareSigner::new("bob") }
    fn charlie() -> SoftwareSigner { SoftwareSigner::new("charlie") }

    fn make_resource() -> ResourceDescriptor {
        ResourceDescriptor {
            resource_type: "MEMORY".to_string(),
            resource_id: 0x1000,
            rights: Rights::READ | Rights::WRITE | Rights::DELEGATE,
        }
    }

    fn make_constraints(max_ops: u32, deadline: f64) -> Constraints {
        Constraints { max_operations: max_ops, deadline_unix: deadline, energy_budget_uj: None }
    }

    #[test]
    fn test_issue_and_resolve_ticket() {
        let alice = alice();
        let bob = bob();
        let mut resolver = RemoteCapabilityResolver::new(bob.did().clone());

        let ticket = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(10, 2_000_000.0),
            None, "nonce-1".to_string(),
        );

        let cap = resolver.resolve::<SoftwareSigner>(&ticket).unwrap();
        assert_eq!(cap.cap_id, "rct-nonce-1");
        assert_eq!(cap.operations_used, 0);
        assert!(!cap.revoked);
    }

    #[test]
    fn test_invalid_signature_rejected() {
        let alice = alice();
        let bob = bob();
        let eve = SoftwareSigner::new("eve");
        let mut resolver = RemoteCapabilityResolver::new(bob.did().clone());

        // Eve signiert, behauptet aber Alice zu sein (unmoeglich ohne Alice's Key)
        let ticket = issue_ticket(
            &eve, bob.did().clone(), make_resource(), make_constraints(10, 2_000_000.0),
            None, "nonce-2".to_string(),
        );

        // Manuell Issuer-DID auf Alice setzen (Forge-Versuch)
        let mut forged = ticket.clone();
        forged.issuer_did = alice.did().clone();

        let result = resolver.resolve::<SoftwareSigner>(&forged);
        assert_eq!(result, Err(TicketError::InvalidSignature));
    }

    #[test]
    fn test_wrong_subject_rejected() {
        let alice = alice();
        let bob = bob();
        // Resolver ist Bob, aber Ticket ist fuer Charlie
        let charlie_did = charlie().did().clone();
        let mut resolver = RemoteCapabilityResolver::new(bob.did().clone());

        let ticket = issue_ticket(
            &alice, charlie_did, make_resource(), make_constraints(10, 2_000_000.0),
            None, "nonce-3".to_string(),
        );

        let result = resolver.resolve::<SoftwareSigner>(&ticket);
        assert_eq!(result, Err(TicketError::WrongSubject));
    }

    #[test]
    fn test_replay_protection() {
        let alice = alice();
        let bob = bob();
        let mut resolver = RemoteCapabilityResolver::new(bob.did().clone());

        let ticket = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(10, 2_000_000.0),
            None, "nonce-replay".to_string(),
        );

        // Erste Einloesung: OK
        assert!(resolver.resolve::<SoftwareSigner>(&ticket).is_ok());
        // Zweite Einloesung (gleicher Nonce): Replay
        let result = resolver.resolve::<SoftwareSigner>(&ticket);
        assert_eq!(result, Err(TicketError::Replay));
    }

    #[test]
    fn test_expired_ticket_rejected() {
        let alice = alice();
        let bob = bob();
        let mut resolver = RemoteCapabilityResolver::new(bob.did().clone());
        resolver.set_time(3_000_000.0); // Zeit nach Deadline

        let ticket = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(10, 1_000_000.0),
            None, "nonce-exp".to_string(),
        );

        let result = resolver.resolve::<SoftwareSigner>(&ticket);
        assert_eq!(result, Err(TicketError::Expired));
    }

    #[test]
    fn test_constraints_too_strict() {
        let alice = alice();
        let bob = bob();
        let mut resolver = RemoteCapabilityResolver::new(bob.did().clone());

        let ticket = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(0, 2_000_000.0),
            None, "nonce-strict".to_string(),
        );

        let result = resolver.resolve::<SoftwareSigner>(&ticket);
        assert_eq!(result, Err(TicketError::ConstraintsTooStrict));
    }

    #[test]
    fn test_local_cap_consumption() {
        let mut cap = LocalCap::new(
            "rct-test".to_string(),
            make_resource(),
            make_constraints(3, 2_000_000.0),
            alice().did().clone(),
        );

        assert!(cap.consume_operation().is_ok());
        assert_eq!(cap.operations_used, 1);
        assert!(cap.consume_operation().is_ok());
        assert_eq!(cap.operations_used, 2);
        assert!(cap.consume_operation().is_ok());
        assert_eq!(cap.operations_used, 3);
        // 4. Operation: ueberschritten
        let result = cap.consume_operation();
        assert_eq!(result, Err(TicketError::MaxOperationsExceeded));
        assert!(cap.revoked);
        // Nach Widerruf: weitere Operationen blockiert
        let result = cap.consume_operation();
        assert_eq!(result, Err(TicketError::CapabilityRevoked));
    }

    #[test]
    fn test_delegation_chain_alice_bob_charlie() {
        // Alice -> Bob -> Charlie (Delegationskette)
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let mut resolver = RemoteCapabilityResolver::new(charlie.did().clone());

        // Ticket 1: Alice -> Bob (volle Rechte, 100 Ops)
        let t1 = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(100, 2_000_000.0),
            None, "nonce-t1".to_string(),
        );

        // Ticket 2: Bob -> Charlie (eingeschraenkte Rechte: nur READ, 50 Ops)
        let attenuated_resource = ResourceDescriptor {
            resource_type: "MEMORY".to_string(),
            resource_id: 0x1000,
            rights: Rights::READ,  // Nur READ, nicht WRITE/DELEGATE
        };
        let t2 = issue_ticket(
            &bob, charlie.did().clone(), attenuated_resource, make_constraints(50, 1_500_000.0),
            Some("nonce-t1".to_string()), "nonce-t2".to_string(),
        );

        let chain = vec![t1, t2];
        let cap = resolver.resolve_chain::<SoftwareSigner>(&chain).unwrap();
        assert_eq!(cap.cap_id, "rct-nonce-t2");
        assert_eq!(cap.constraints.max_operations, 50);
    }

    #[test]
    fn test_chain_broken_nonce() {
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let mut resolver = RemoteCapabilityResolver::new(charlie.did().clone());

        let t1 = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(100, 2_000_000.0),
            None, "nonce-real-1".to_string(),
        );
        // t2 referenziert falschen Parent-Nonce
        let t2 = issue_ticket(
            &bob, charlie.did().clone(), make_resource(), make_constraints(50, 1_500_000.0),
            Some("nonce-WRONG".to_string()), "nonce-real-2".to_string(),
        );

        let result = resolver.resolve_chain::<SoftwareSigner>(&[t1, t2]);
        assert_eq!(result, Err(TicketError::ChainBroken));
    }

    #[test]
    fn test_chain_rights_expansion_rejected() {
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let mut resolver = RemoteCapabilityResolver::new(charlie.did().clone());

        // Alice gibt Bob nur READ
        let limited = ResourceDescriptor {
            resource_type: "MEMORY".to_string(),
            resource_id: 0x1000,
            rights: Rights::READ,
        };
        let t1 = issue_ticket(
            &alice, bob.did().clone(), limited, make_constraints(100, 2_000_000.0),
            None, "nonce-lim-1".to_string(),
        );
        // Bob versucht, Charlie READ+WRITE zu geben (Rechte-Erweiterung!)
        let expanded = ResourceDescriptor {
            resource_type: "MEMORY".to_string(),
            resource_id: 0x1000,
            rights: Rights::READ | Rights::WRITE,  // Erweiterung!
        };
        let t2 = issue_ticket(
            &bob, charlie.did().clone(), expanded, make_constraints(50, 1_500_000.0),
            Some("nonce-lim-1".to_string()), "nonce-lim-2".to_string(),
        );

        let result = resolver.resolve_chain::<SoftwareSigner>(&[t1, t2]);
        assert_eq!(result, Err(TicketError::ChainRightsExpanded));
    }

    #[test]
    fn test_chain_ops_expansion_rejected() {
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let mut resolver = RemoteCapabilityResolver::new(charlie.did().clone());

        let t1 = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(10, 2_000_000.0),
            None, "nonce-ops-1".to_string(),
        );
        // Bob versucht, Charlie 50 Ops zu geben (mehr als Alice's 10!)
        let t2 = issue_ticket(
            &bob, charlie.did().clone(), make_resource(), make_constraints(50, 1_500_000.0),
            Some("nonce-ops-1".to_string()), "nonce-ops-2".to_string(),
        );

        let result = resolver.resolve_chain::<SoftwareSigner>(&[t1, t2]);
        assert_eq!(result, Err(TicketError::ChainOpsExpanded));
    }

    #[test]
    fn test_chain_deadline_extension_rejected() {
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let mut resolver = RemoteCapabilityResolver::new(charlie.did().clone());

        let t1 = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(100, 1_000_000.0),
            None, "nonce-dl-1".to_string(),
        );
        // Bob versucht, Charlie eine laengere Deadline zu geben
        let t2 = issue_ticket(
            &bob, charlie.did().clone(), make_resource(), make_constraints(50, 2_000_000.0), // laenger!
            Some("nonce-dl-1".to_string()), "nonce-dl-2".to_string(),
        );

        let result = resolver.resolve_chain::<SoftwareSigner>(&[t1, t2]);
        assert_eq!(result, Err(TicketError::ChainDeadlineExtended));
    }

    #[test]
    fn test_chain_resource_mismatch_rejected() {
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let mut resolver = RemoteCapabilityResolver::new(charlie.did().clone());

        let t1 = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(100, 2_000_000.0),
            None, "nonce-res-1".to_string(),
        );
        // Bob versucht, Ticket fuer eine ANDERE Ressource
        let other_resource = ResourceDescriptor {
            resource_type: "MEMORY".to_string(),
            resource_id: 0x2000,  // Andere Resource-ID!
            rights: Rights::READ,
        };
        let t2 = issue_ticket(
            &bob, charlie.did().clone(), other_resource, make_constraints(50, 1_500_000.0),
            Some("nonce-res-1".to_string()), "nonce-res-2".to_string(),
        );

        let result = resolver.resolve_chain::<SoftwareSigner>(&[t1, t2]);
        assert_eq!(result, Err(TicketError::ResourceMismatch));
    }

    #[test]
    fn test_empty_chain_rejected() {
        let bob = bob();
        let mut resolver = RemoteCapabilityResolver::new(bob.did().clone());
        let result = resolver.resolve_chain::<SoftwareSigner>(&[]);
        assert_eq!(result, Err(TicketError::EmptyChain));
    }

    #[test]
    fn test_signing_payload_deterministic() {
        let alice = alice();
        let bob = bob();
        let t1 = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(10, 2_000_000.0),
            None, "nonce-det".to_string(),
        );
        let t2 = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(10, 2_000_000.0),
            None, "nonce-det".to_string(),
        );
        // Gleiche Parameter → gleiche Payload
        assert_eq!(t1.signing_payload(), t2.signing_payload());
    }

    #[test]
    fn test_three_hop_delegation() {
        // Alice -> Bob -> Charlie -> Dave (3-Hop Kette)
        let alice = alice();
        let bob = bob();
        let charlie = charlie();
        let dave = SoftwareSigner::new("dave");
        let mut resolver = RemoteCapabilityResolver::new(dave.did().clone());

        // Alice -> Bob (100 ops, all rights)
        let t1 = issue_ticket(
            &alice, bob.did().clone(), make_resource(), make_constraints(100, 2_000_000.0),
            None, "hop-1".to_string(),
        );
        // Bob -> Charlie (50 ops, READ only)
        let t2 = issue_ticket(
            &bob, charlie.did().clone(),
            ResourceDescriptor { resource_type: "MEMORY".to_string(), resource_id: 0x1000, rights: Rights::READ },
            make_constraints(50, 1_800_000.0),
            Some("hop-1".to_string()), "hop-2".to_string(),
        );
        // Charlie -> Dave (10 ops, READ only)
        let t3 = issue_ticket(
            &charlie, dave.did().clone(),
            ResourceDescriptor { resource_type: "MEMORY".to_string(), resource_id: 0x1000, rights: Rights::READ },
            make_constraints(10, 1_500_000.0),
            Some("hop-2".to_string()), "hop-3".to_string(),
        );

        let chain = vec![t1, t2, t3];
        let cap = resolver.resolve_chain::<SoftwareSigner>(&chain).unwrap();
        assert_eq!(cap.constraints.max_operations, 10);
        assert_eq!(cap.resource.rights, Rights::READ);
    }
}
