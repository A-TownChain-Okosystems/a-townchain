// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Knowledge Graph (Rust).
//!
//! Nativer Kernel-Speicher fuer strukturiertes Wissen.
//! Tripel-basierter Graph: (Subject, Predicate, Object).
//! Integration mit Capability-System fuer Zugriffskontrolle.
//!
//! Eigenschaften:
//! - Triple-Store: SPO (Subject-Predicate-Object) Tripel
//! - Entity-IDs: Kernel-weit eindeutig (wie PIDs)
//! - Capability-gated: Lese-/Schreiboperationen erfordern Caps
//! - Query-Engine: Lookup mit Wildcards (None = Match-All)
//! - Literal-Werte: Zahlen, Strings, Bytes
//! - Bidirektional: Rueckwaerts-Lookup (Object -> Subject)
//! - Transitivitaet: Pfad-Suche ueber mehrere Hops

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::capability::{CapabilityTable, CapId, Pid, ResourceType, Rights};

/// Eindeutige Entity-ID im Knowledge Graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(pub u64);

/// Praedikat-Typ (Art der Relation)
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Predicate(pub String);

/// Objekt-Wert: entweder Entity-Referenz oder Literal
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectValue {
    Entity(EntityId),
    Integer(i64),
    String(String),
    Bytes(Vec<u8>),
    Boolean(bool),
}

/// Ein Triple: (Subject, Predicate, Object)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triple {
    pub subject: EntityId,
    pub predicate: Predicate,
    pub object: ObjectValue,
}

/// Entity mit Metadaten
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub label: String,
    pub entity_type: String,
    pub created_by: Pid,
    pub triples_count: u32,
}

/// Knowledge Graph — der zentrale Triple-Store
pub struct KnowledgeGraph {
    entities: BTreeMap<EntityId, Entity>,
    triples: Vec<Triple>,
    /// Index: subject -> [triple indices]
    spo_index: BTreeMap<EntityId, Vec<usize>>,
    /// Index: object-entity -> [triple indices] (Rueckwaerts-Lookup)
    osp_index: BTreeMap<EntityId, Vec<usize>>,
    /// Index: predicate -> [triple indices]
    pso_index: BTreeMap<String, Vec<usize>>,
    next_entity_id: AtomicU64,
}

/// Fehler beim Graph-Zugriff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KgError {
    EntityNotFound,
    TripleNotFound,
    NoWriteCapability,
    NoReadCapability,
    CapabilityRevoked,
    InvalidObjectValue,
}

/// Query-Pattern: None = Wildcard (Match-All)
#[derive(Debug, Clone)]
pub struct QueryPattern {
    pub subject: Option<EntityId>,
    pub predicate: Option<String>,
    pub object: Option<ObjectValue>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            entities: BTreeMap::new(),
            triples: Vec::new(),
            spo_index: BTreeMap::new(),
            osp_index: BTreeMap::new(),
            pso_index: BTreeMap::new(),
            next_entity_id: AtomicU64::new(1),
        }
    }

    /// Erzeugt eine neue Entity und gibt dem Ersteller WRITE-Capability.
    pub fn create_entity(
        &mut self,
        caps: &mut CapabilityTable,
        creator: Pid,
        label: &str,
        entity_type: &str,
    ) -> Result<EntityId, KgError> {
        let id = EntityId(self.next_entity_id.fetch_add(1, Ordering::SeqCst));
        let resource_id = id.0;

        // Creator bekommt READ + WRITE + DELEGATE
        caps.create(creator, ResourceType::Memory, resource_id,
            Rights::READ | Rights::WRITE | Rights::DELEGATE);

        let entity = Entity {
            id, label: label.to_string(), entity_type: entity_type.to_string(),
            created_by: creator, triples_count: 0,
        };
        self.entities.insert(id, entity);
        Ok(id)
    }

    /// Fuegt ein Triple hinzu. Prueft WRITE-Capability fuer das Subject.
    pub fn add_triple(
        &mut self,
        caps: &CapabilityTable,
        caller: Pid,
        subject: EntityId,
        predicate: Predicate,
        object: ObjectValue,
    ) -> Result<(), KgError> {
        // Entity muss existieren
        if !self.entities.contains_key(&subject) {
            return Err(KgError::EntityNotFound);
        }

        // Capability-Check: Caller braucht WRITE auf Subject-Entity
        if !caps.check(caller, ResourceType::Memory, subject.0, Rights::WRITE) {
            return Err(KgError::NoWriteCapability);
        }

        let triple = Triple { subject, predicate, object: object.clone() };
        let idx = self.triples.len();
        self.triples.push(triple);

        // Indices aktualisieren
        self.spo_index.entry(subject).or_default().push(idx);

        if let ObjectValue::Entity(oid) = &object {
            self.osp_index.entry(*oid).or_default().push(idx);
        }

        self.pso_index.entry(
            self.triples[idx].predicate.0.clone()
        ).or_default().push(idx);

        // Triple-Counter erhoehen
        if let Some(e) = self.entities.get_mut(&subject) {
            e.triples_count += 1;
        }

        Ok(())
    }

    /// Query: findet alle Tripel die zum Pattern passen.
    /// None-Felder sind Wildcards.
    /// Prueft READ-Capability fuer jedes zurueckgegebene Subject.
    pub fn query(
        &self,
        caps: &CapabilityTable,
        caller: Pid,
        pattern: &QueryPattern,
    ) -> Vec<Triple> {
        let mut results = Vec::new();

        for triple in &self.triples {
            // Subject-Match
            if let Some(s) = &pattern.subject {
                if &triple.subject != s { continue; }
            }
            // Predicate-Match
            if let Some(p) = &pattern.predicate {
                if triple.predicate.0 != *p { continue; }
            }
            // Object-Match
            if let Some(o) = &pattern.object {
                if &triple.object != o { continue; }
            }

            // Capability-Check: Caller braucht READ auf Subject
            if !caps.check(caller, ResourceType::Memory, triple.subject.0, Rights::READ) {
                continue;
            }

            results.push(triple.clone());
        }

        results
    }

    /// Holt alle ausgehenden Tripel einer Entity (Subject-Lookup).
    pub fn outgoing(
        &self,
        caps: &CapabilityTable,
        caller: Pid,
        entity: EntityId,
    ) -> Vec<Triple> {
        self.query(caps, caller, &QueryPattern {
            subject: Some(entity),
            predicate: None,
            object: None,
        })
    }

    /// Holt alle eingehenden Tripel einer Entity (Object-Lookup).
    /// "Wer referenziert mich?"
    pub fn incoming(
        &self,
        caps: &CapabilityTable,
        caller: Pid,
        entity: EntityId,
    ) -> Vec<Triple> {
        let mut results = Vec::new();

        if let Some(indices) = self.osp_index.get(&entity) {
            for &idx in indices {
                let triple = &self.triples[idx];
                // Capability-Check
                if caps.check(caller, ResourceType::Memory, triple.subject.0, Rights::READ) {
                    results.push(triple.clone());
                }
            }
        }

        results
    }

    /// Transitive Pfadsuche: folgt einem Praedikat ueber mehrere Hops.
    /// Bsp: "depends_on" transitiv — A depends_on B depends_on C -> [A->B, B->C]
    pub fn transitive_closure(
        &self,
        caps: &CapabilityTable,
        caller: Pid,
        start: EntityId,
        predicate: &str,
        max_depth: usize,
    ) -> Vec<EntityId> {
        let mut visited = alloc::collections::BTreeSet::new();
        let mut frontier = Vec::new();
        frontier.push(start);
        let mut result = Vec::new();

        for _ in 0..max_depth {
            if frontier.is_empty() { break; }
            let mut next_frontier = Vec::new();

            for node in frontier {
                if visited.contains(&node) { continue; }
                visited.insert(node);

                let out = self.outgoing(caps, caller, node);
                for t in out {
                    if t.predicate.0 == predicate {
                        if let ObjectValue::Entity(target) = t.object {
                            if !visited.contains(&target) {
                                result.push(target);
                                next_frontier.push(target);
                            }
                        }
                    }
                }
            }
            frontier = next_frontier;
        }

        result
    }

    /// Loescht ein Triple. Prueft WRITE-Capability.
    pub fn remove_triple(
        &mut self,
        caps: &CapabilityTable,
        caller: Pid,
        subject: EntityId,
        predicate: &str,
        object: &ObjectValue,
    ) -> Result<(), KgError> {
        if !caps.check(caller, ResourceType::Memory, subject.0, Rights::WRITE) {
            return Err(KgError::NoWriteCapability);
        }

        let before = self.triples.len();
        self.triples.retain(|t| {
            !(t.subject == subject && t.predicate.0 == predicate && t.object == *object)
        });

        if self.triples.len() == before {
            return Err(KgError::TripleNotFound);
        }

        // Indices neu aufbauen (einfach, aber korrekt)
        self.rebuild_indices();

        if let Some(e) = self.entities.get_mut(&subject) {
            e.triples_count = e.triples_count.saturating_sub(1);
        }

        Ok(())
    }

    /// Loescht eine Entity und alle ihre Tripel. Prueft WRITE-Capability.
    pub fn delete_entity(
        &mut self,
        caps: &mut CapabilityTable,
        caller: Pid,
        entity: EntityId,
    ) -> Result<(), KgError> {
        if !self.entities.contains_key(&entity) {
            return Err(KgError::EntityNotFound);
        }
        if !caps.check(caller, ResourceType::Memory, entity.0, Rights::WRITE) {
            return Err(KgError::NoWriteCapability);
        }

        // Alle Tripel entfernen (als Subject UND als Object)
        self.triples.retain(|t| t.subject != entity && !matches!(&t.object, ObjectValue::Entity(e) if *e == entity));
        self.rebuild_indices();

        // Capabilities widerrufen
        let cap_ids: Vec<CapId> = caps.list_for(caller).iter()
            .filter(|c| c.resource_type == ResourceType::Memory && c.resource_id == entity.0)
            .map(|c| c.id)
            .collect();
        for cap_id in cap_ids {
            caps.revoke(cap_id);
        }

        self.entities.remove(&entity);
        Ok(())
    }

    /// Holt eine Entity (Metadaten).
    pub fn get_entity(&self, entity: EntityId) -> Option<&Entity> {
        self.entities.get(&entity)
    }

    /// Anzahl Entities im Graph
    pub fn entity_count(&self) -> usize { self.entities.len() }

    /// Anzahl Tripel im Graph
    pub fn triple_count(&self) -> usize { self.triples.len() }

    /// Grant: delegiert READ an einen anderen Prozess
    pub fn grant_read(
        &self,
        caps: &mut CapabilityTable,
        owner: Pid,
        entity: EntityId,
        target: Pid,
    ) -> Result<CapId, KgError> {
        if !self.entities.contains_key(&entity) {
            return Err(KgError::EntityNotFound);
        }
        let owner_caps: Vec<CapId> = caps.list_for(owner).iter()
            .filter(|c| c.resource_type == ResourceType::Memory
                && c.resource_id == entity.0
                && c.rights.has(Rights::READ))
            .map(|c| c.id)
            .collect();

        let source_cap = owner_caps.first().ok_or(KgError::NoReadCapability)?;
        caps.delegate(owner, *source_cap, target, Rights::READ)
            .map_err(|_| KgError::CapabilityRevoked)
    }

    /// Indices neu aufbauen (nach Loeschungen)
    fn rebuild_indices(&mut self) {
        self.spo_index.clear();
        self.osp_index.clear();
        self.pso_index.clear();

        for (idx, triple) in self.triples.iter().enumerate() {
            self.spo_index.entry(triple.subject).or_default().push(idx);
            if let ObjectValue::Entity(oid) = &triple.object {
                self.osp_index.entry(*oid).or_default().push(idx);
            }
            self.pso_index.entry(triple.predicate.0.clone()).or_default().push(idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(n: u32) -> Pid { Pid(n) }

    fn setup() -> (CapabilityTable, KnowledgeGraph) {
        (CapabilityTable::new(), KnowledgeGraph::new())
    }

    #[test]
    fn test_create_entity() {
        let (mut caps, mut kg) = setup();
        let eid = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();
        assert!(eid.0 > 0);
        assert_eq!(kg.entity_count(), 1);
        let e = kg.get_entity(eid).unwrap();
        assert_eq!(e.label, "Alice");
        assert_eq!(e.entity_type, "Person");
    }

    #[test]
    fn test_add_and_query_triple() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();
        let bob = kg.create_entity(&mut caps, pid(1), "Bob", "Person").unwrap();

        kg.add_triple(&caps, pid(1), alice,
            Predicate("knows".to_string()),
            ObjectValue::Entity(bob)).unwrap();

        // Query: Alice knows ?
        let results = kg.query(&caps, pid(1), &QueryPattern {
            subject: Some(alice),
            predicate: Some("knows".to_string()),
            object: None,
        });
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].object, ObjectValue::Entity(bob));
    }

    #[test]
    fn test_add_triple_without_write_cap_rejected() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();

        // pid(2) hat keine WRITE-Cap auf Alice
        let result = kg.add_triple(&caps, pid(2), alice,
            Predicate("knows".to_string()),
            ObjectValue::String("someone".to_string()));
        assert_eq!(result, Err(KgError::NoWriteCapability));
    }

    #[test]
    fn test_query_without_read_cap_filtered() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();
        let bob = kg.create_entity(&mut caps, pid(1), "Bob", "Person").unwrap();

        kg.add_triple(&caps, pid(1), alice,
            Predicate("knows".to_string()),
            ObjectValue::Entity(bob)).unwrap();

        // pid(1) hat READ (ist Creator) -> sieht Tripel
        let r1 = kg.query(&caps, pid(1), &QueryPattern {
            subject: Some(alice), predicate: None, object: None,
        });
        assert_eq!(r1.len(), 1);

        // pid(2) hat keine READ -> sieht nichts
        let r2 = kg.query(&caps, pid(2), &QueryPattern {
            subject: Some(alice), predicate: None, object: None,
        });
        assert_eq!(r2.len(), 0);
    }

    #[test]
    fn test_grant_read_enables_query() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();
        let bob = kg.create_entity(&mut caps, pid(1), "Bob", "Person").unwrap();

        kg.add_triple(&caps, pid(1), alice,
            Predicate("knows".to_string()),
            ObjectValue::Entity(bob)).unwrap();

        // pid(2) sieht nichts
        assert_eq!(kg.query(&caps, pid(2), &QueryPattern {
            subject: Some(alice), predicate: None, object: None,
        }).len(), 0);

        // pid(1) delegiert READ an pid(2)
        kg.grant_read(&mut caps, pid(1), alice, pid(2)).unwrap();

        // Jetzt sieht pid(2) das Tripel
        let results = kg.query(&caps, pid(2), &QueryPattern {
            subject: Some(alice), predicate: None, object: None,
        });
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_outgoing_and_incoming() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();
        let bob = kg.create_entity(&mut caps, pid(1), "Bob", "Person").unwrap();
        let charlie = kg.create_entity(&mut caps, pid(1), "Charlie", "Person").unwrap();

        // Alice -> knows -> Bob
        kg.add_triple(&caps, pid(1), alice,
            Predicate("knows".to_string()),
            ObjectValue::Entity(bob)).unwrap();
        // Charlie -> knows -> Alice
        kg.add_triple(&caps, pid(1), charlie,
            Predicate("knows".to_string()),
            ObjectValue::Entity(alice)).unwrap();

        // Outgoing von Alice: 1 (Alice knows Bob)
        let out = kg.outgoing(&caps, pid(1), alice);
        assert_eq!(out.len(), 1);

        // Incoming zu Alice: 1 (Charlie knows Alice)
        let inc = kg.incoming(&caps, pid(1), alice);
        assert_eq!(inc.len(), 1);
        assert_eq!(inc[0].subject, charlie);
    }

    #[test]
    fn test_transitive_closure() {
        let (mut caps, mut kg) = setup();
        let a = kg.create_entity(&mut caps, pid(1), "A", "Module").unwrap();
        let b = kg.create_entity(&mut caps, pid(1), "B", "Module").unwrap();
        let c = kg.create_entity(&mut caps, pid(1), "C", "Module").unwrap();
        let d = kg.create_entity(&mut caps, pid(1), "D", "Module").unwrap();

        // A depends_on B, B depends_on C, C depends_on D
        kg.add_triple(&caps, pid(1), a, Predicate("depends_on".to_string()), ObjectValue::Entity(b)).unwrap();
        kg.add_triple(&caps, pid(1), b, Predicate("depends_on".to_string()), ObjectValue::Entity(c)).unwrap();
        kg.add_triple(&caps, pid(1), c, Predicate("depends_on".to_string()), ObjectValue::Entity(d)).unwrap();

        // Transitive: A depends_on -> [B, C, D]
        let closure = kg.transitive_closure(&caps, pid(1), a, "depends_on", 10);
        assert_eq!(closure.len(), 3);
        assert!(closure.contains(&b));
        assert!(closure.contains(&c));
        assert!(closure.contains(&d));
    }

    #[test]
    fn test_transitive_closure_respects_max_depth() {
        let (mut caps, mut kg) = setup();
        let a = kg.create_entity(&mut caps, pid(1), "A", "Module").unwrap();
        let b = kg.create_entity(&mut caps, pid(1), "B", "Module").unwrap();
        let c = kg.create_entity(&mut caps, pid(1), "C", "Module").unwrap();

        kg.add_triple(&caps, pid(1), a, Predicate("depends_on".to_string()), ObjectValue::Entity(b)).unwrap();
        kg.add_triple(&caps, pid(1), b, Predicate("depends_on".to_string()), ObjectValue::Entity(c)).unwrap();

        // max_depth=1: nur direkte Nachbarn (B)
        let closure = kg.transitive_closure(&caps, pid(1), a, "depends_on", 1);
        assert_eq!(closure.len(), 1);
        assert_eq!(closure[0], b);

        // max_depth=2: B und C
        let closure = kg.transitive_closure(&caps, pid(1), a, "depends_on", 2);
        assert_eq!(closure.len(), 2);
    }

    #[test]
    fn test_transitive_closure_no_cycle() {
        let (mut caps, mut kg) = setup();
        let a = kg.create_entity(&mut caps, pid(1), "A", "Module").unwrap();
        let b = kg.create_entity(&mut caps, pid(1), "B", "Module").unwrap();

        kg.add_triple(&caps, pid(1), a, Predicate("depends_on".to_string()), ObjectValue::Entity(b)).unwrap();
        // Zyklus: B depends_on A
        kg.add_triple(&caps, pid(1), b, Predicate("depends_on".to_string()), ObjectValue::Entity(a)).unwrap();

        // Trotz Zyklus sollte terminieren (visited-Set)
        let closure = kg.transitive_closure(&caps, pid(1), a, "depends_on", 100);
        assert_eq!(closure.len(), 1); // Nur B (A bereits visited)
    }

    #[test]
    fn test_literal_values() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();

        // String-Literal
        kg.add_triple(&caps, pid(1), alice,
            Predicate("name".to_string()),
            ObjectValue::String("Alice Wonder".to_string())).unwrap();

        // Integer-Literal
        kg.add_triple(&caps, pid(1), alice,
            Predicate("age".to_string()),
            ObjectValue::Integer(30)).unwrap();

        // Boolean-Literal
        kg.add_triple(&caps, pid(1), alice,
            Predicate("active".to_string()),
            ObjectValue::Boolean(true)).unwrap();

        // Query alle Tripel
        let results = kg.query(&caps, pid(1), &QueryPattern {
            subject: Some(alice), predicate: None, object: None,
        });
        assert_eq!(results.len(), 3);

        // Query mit Object-Match
        let r = kg.query(&caps, pid(1), &QueryPattern {
            subject: Some(alice),
            predicate: Some("age".to_string()),
            object: Some(ObjectValue::Integer(30)),
        });
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn test_remove_triple() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();
        let bob = kg.create_entity(&mut caps, pid(1), "Bob", "Person").unwrap();

        kg.add_triple(&caps, pid(1), alice,
            Predicate("knows".to_string()),
            ObjectValue::Entity(bob)).unwrap();
        assert_eq!(kg.triple_count(), 1);

        kg.remove_triple(&caps, pid(1), alice, "knows", &ObjectValue::Entity(bob)).unwrap();
        assert_eq!(kg.triple_count(), 0);

        // Nochmal loeschen: nicht gefunden
        let result = kg.remove_triple(&caps, pid(1), alice, "knows", &ObjectValue::Entity(bob));
        assert_eq!(result, Err(KgError::TripleNotFound));
    }

    #[test]
    fn test_remove_triple_without_cap_rejected() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();

        kg.add_triple(&caps, pid(1), alice,
            Predicate("name".to_string()),
            ObjectValue::String("Alice".to_string())).unwrap();

        // pid(2) kann nicht loeschen (keine WRITE-Cap)
        let result = kg.remove_triple(&caps, pid(2), alice, "name",
            &ObjectValue::String("Alice".to_string()));
        assert_eq!(result, Err(KgError::NoWriteCapability));
    }

    #[test]
    fn test_delete_entity() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();
        let bob = kg.create_entity(&mut caps, pid(1), "Bob", "Person").unwrap();

        kg.add_triple(&caps, pid(1), alice,
            Predicate("knows".to_string()),
            ObjectValue::Entity(bob)).unwrap();

        assert_eq!(kg.entity_count(), 2);
        assert_eq!(kg.triple_count(), 1);

        // Alice loeschen
        kg.delete_entity(&mut caps, pid(1), alice).unwrap();
        assert_eq!(kg.entity_count(), 1);
        assert_eq!(kg.triple_count(), 0); // Triple entfernt
    }

    #[test]
    fn test_delete_entity_without_cap_rejected() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();

        let result = kg.delete_entity(&mut caps, pid(2), alice);
        assert_eq!(result, Err(KgError::NoWriteCapability));
    }

    #[test]
    fn test_wildcard_query() {
        let (mut caps, mut kg) = setup();
        let a = kg.create_entity(&mut caps, pid(1), "A", "Node").unwrap();
        let b = kg.create_entity(&mut caps, pid(1), "B", "Node").unwrap();
        let c = kg.create_entity(&mut caps, pid(1), "C", "Node").unwrap();

        kg.add_triple(&caps, pid(1), a, Predicate("link".to_string()), ObjectValue::Entity(b)).unwrap();
        kg.add_triple(&caps, pid(1), b, Predicate("link".to_string()), ObjectValue::Entity(c)).unwrap();
        kg.add_triple(&caps, pid(1), c, Predicate("link".to_string()), ObjectValue::Entity(a)).unwrap();

        // Wildcard: alle Tripel mit Praedikat "link"
        let results = kg.query(&caps, pid(1), &QueryPattern {
            subject: None,
            predicate: Some("link".to_string()),
            object: None,
        });
        assert_eq!(results.len(), 3);

        // Vollstaendige Wildcard: alle Tripel
        let results = kg.query(&caps, pid(1), &QueryPattern {
            subject: None, predicate: None, object: None,
        });
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_entity_not_found() {
        let (mut caps, mut kg) = setup();
        let result = kg.add_triple(&caps, pid(1), EntityId(999),
            Predicate("test".to_string()),
            ObjectValue::Integer(1));
        assert_eq!(result, Err(KgError::EntityNotFound));
    }

    #[test]
    fn test_cross_process_isolation() {
        let (mut caps, mut kg) = setup();
        // pid(1) erstellt Alice
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();
        // pid(2) erstellt Bob
        let bob = kg.create_entity(&mut caps, pid(2), "Bob", "Person").unwrap();

        // pid(1) kann auf Alice schreiben, nicht auf Bob
        assert!(kg.add_triple(&caps, pid(1), alice,
            Predicate("name".to_string()),
            ObjectValue::String("A".to_string())).is_ok());
        assert_eq!(kg.add_triple(&caps, pid(1), bob,
            Predicate("name".to_string()),
            ObjectValue::String("B".to_string())), Err(KgError::NoWriteCapability));

        // pid(2) kann auf Bob schreiben, nicht auf Alice
        assert!(kg.add_triple(&caps, pid(2), bob,
            Predicate("name".to_string()),
            ObjectValue::String("B".to_string())).is_ok());
        assert_eq!(kg.add_triple(&caps, pid(2), alice,
            Predicate("name".to_string()),
            ObjectValue::String("A".to_string())), Err(KgError::NoWriteCapability));
    }

    #[test]
    fn test_triple_count_tracking() {
        let (mut caps, mut kg) = setup();
        let alice = kg.create_entity(&mut caps, pid(1), "Alice", "Person").unwrap();

        kg.add_triple(&caps, pid(1), alice, Predicate("a".to_string()), ObjectValue::Integer(1)).unwrap();
        kg.add_triple(&caps, pid(1), alice, Predicate("b".to_string()), ObjectValue::Integer(2)).unwrap();
        kg.add_triple(&caps, pid(1), alice, Predicate("c".to_string()), ObjectValue::Integer(3)).unwrap();

        let e = kg.get_entity(alice).unwrap();
        assert_eq!(e.triples_count, 3);

        // Eines loeschen
        kg.remove_triple(&caps, pid(1), alice, "b", &ObjectValue::Integer(2)).unwrap();
        let e = kg.get_entity(alice).unwrap();
        assert_eq!(e.triples_count, 2);
    }
}
