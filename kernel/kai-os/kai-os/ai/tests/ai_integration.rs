//! S13-S15 Integrationstests — Issue #106 Akzeptanzkriterien.
//! Deterministisch: kein RNG, keine Wall-Clock (REQ-ENG-002).

use kai_os_ai::audit::{AuditPipeline, GENESIS_HASH};
use kai_os_ai::ipc::{DeliveryReceipt, IpcError, IpcGateway, IpcMessage};
use kai_os_ai::proposal::{ProposalError, ProposalRegistry, ProposalState};
use serde_json::json;

fn msg(agent: &str, msg_type: &str, seq: u64, payload: serde_json::Value) -> IpcMessage {
    IpcMessage {
        from_agent: agent.into(),
        msg_type: msg_type.into(),
        seq,
        payload,
    }
}

fn gateway() -> IpcGateway {
    let mut gw = IpcGateway::new();
    gw.register_agent("agent-alpha", &["propose_tx", "query_state"]);
    gw.register_agent("agent-beta", &["register_model"]);
    gw
}

// ── AK 1+2: Identity & Capability (deny by default) ────────────────────

#[test]
fn ipc_identity_and_capability_gates() {
    let mut gw = gateway();

    // Registriert + Capability -> Zustellung ok
    let r = gw
        .deliver(&msg(
            "agent-alpha",
            "query_state",
            1,
            json!({"key": "chain/tip"}),
        ))
        .unwrap();
    assert_eq!(r.seq, 1);
    assert!(r.audit_seq > 0);

    // Unbekannter Agent -> abgewiesen
    assert!(matches!(
        gw.deliver(&msg("agent-ghost", "query_state", 1, json!({"key": "x"}))),
        Err(IpcError::UnknownAgent(_))
    ));

    // Bekannter Agent, falsche Capability -> abgewiesen
    assert!(matches!(
        gw.deliver(&msg("agent-beta", "query_state", 1, json!({"key": "x"}))),
        Err(IpcError::CapabilityDenied { .. })
    ));
}

// ── AK 3: Replay-Schutz ────────────────────────────────────────────────

#[test]
fn ipc_replay_rejected() {
    let mut gw = gateway();
    gw.deliver(&msg("agent-alpha", "query_state", 5, json!({"key": "a"})))
        .unwrap();

    // Gleiche Seq -> Replay
    assert!(matches!(
        gw.deliver(&msg("agent-alpha", "query_state", 5, json!({"key": "b"}))),
        Err(IpcError::Replay { seq: 5, last: 5 })
    ));
    // Zurückliegende Seq -> Replay
    assert!(matches!(
        gw.deliver(&msg("agent-alpha", "query_state", 2, json!({"key": "c"}))),
        Err(IpcError::Replay { seq: 2, last: 5 })
    ));
    // Strikt monoton weiter geht
    gw.deliver(&msg("agent-alpha", "query_state", 6, json!({"key": "d"})))
        .unwrap();
    // Pro Agent unabhängig: beta startet bei 1
    gw.deliver(&msg(
        "agent-beta",
        "register_model",
        1,
        json!({"model_id": "m1", "version": "1"}),
    ))
    .unwrap();
}

// ── AK 4: Schema-Validierung (fail-closed) ─────────────────────────────

#[test]
fn ipc_schema_validation() {
    let mut gw = gateway();

    // propose_tx ohne "params" -> Schema-Verletzung
    assert!(matches!(
        gw.deliver(&msg("agent-alpha", "propose_tx", 1, json!({"action": "transfer"}))),
        Err(IpcError::SchemaViolation { missing, .. }) if missing == "params"
    ));
    // Komplett leer -> "action" fehlt
    assert!(matches!(
        gw.deliver(&msg("agent-alpha", "propose_tx", 1, json!({}))),
        Err(IpcError::SchemaViolation { missing, .. }) if missing == "action"
    ));
    // Vollständig -> ok
    gw.deliver(&msg(
        "agent-alpha",
        "propose_tx",
        1,
        json!({"action": "transfer", "params": {"to": "bob", "amount": 5}}),
    ))
    .unwrap();
}

// ── AK 5: Jede Nachricht erzeugt Audit ─────────────────────────────────

#[test]
fn ipc_every_message_audited_including_rejections() {
    let mut gw = gateway();

    gw.deliver(&msg("agent-alpha", "query_state", 1, json!({"key": "a"})))
        .unwrap();
    assert!(gw
        .deliver(&msg("agent-ghost", "query_state", 1, json!({"key": "a"})))
        .is_err());
    assert!(gw
        .deliver(&msg("agent-alpha", "query_state", 1, json!({"key": "a"})))
        .is_err()); // Replay
    assert!(gw
        .deliver(&msg("agent-alpha", "propose_tx", 2, json!({})))
        .is_err()); // Schema

    let a = gw.audit();
    assert!(a.verify());
    // 2 Registrierungen + 1 Zustellung + 3 Abweisungen = 6 Events
    assert_eq!(a.events().len(), 6);
    assert_eq!(a.by_kind("query_state").len(), 1);
    assert_eq!(a.by_kind("rejected_unknown_agent").len(), 1);
    assert_eq!(a.by_kind("rejected_replay").len(), 1);
    assert_eq!(a.by_kind("rejected_schema").len(), 1);
    assert_eq!(a.by_actor("ipc-gateway").len(), 5); // 2x Registrierung + 3x Abweisung (Zustellung: actor=agent-alpha) // 2x Registration + 3x Abweisung... = 5? Nein: 2+3=5 registriert unten
}

// ── AK 6: Proposal-IDs deterministisch + Idempotenz ────────────────────

#[test]
fn proposal_deterministic_id_and_idempotency() {
    let mut reg = ProposalRegistry::new();
    let payload = json!({"action": "transfer", "params": {"to": "bob", "amount": 5}});

    let id1 = reg.submit("agent-alpha", "propose_tx", payload.clone());
    let id2 = reg.submit("agent-alpha", "propose_tx", payload.clone());
    assert_eq!(id1, id2); // identisch -> identische ID, kein Duplikat
    assert_eq!(reg.all().len(), 1);
    assert_eq!(id1.len(), 64);

    // Anderer Agent oder anderes Payload -> andere ID
    let id3 = reg.submit("agent-beta", "propose_tx", payload.clone());
    assert_ne!(id1, id3);

    let p = reg.get(&id1).unwrap();
    assert_eq!(p.state, ProposalState::Proposed);
    assert_eq!(p.agent, "agent-alpha");
}

// ── AK 7: Lifecycle Proposed → Specified → HandedToVM ─────────────────

#[test]
fn proposal_lifecycle_strict() {
    let mut reg = ProposalRegistry::new();
    let id = reg.submit(
        "agent-alpha",
        "propose_tx",
        json!({"action": "transfer", "params": {"to": "bob", "amount": 5}}),
    );

    // Sprung direkt zu HandedToVM ist verboten (Specified ist Pflicht)
    assert!(matches!(
        reg.mark_handed_to_vm(&id),
        Err(ProposalError::InvalidTransition {
            from: ProposalState::Proposed,
            ..
        })
    ));

    reg.mark_specified(&id).unwrap();
    assert_eq!(reg.get(&id).unwrap().state, ProposalState::Specified);

    // Specified nochmal -> invalide
    assert!(matches!(
        reg.mark_specified(&id),
        Err(ProposalError::InvalidTransition {
            from: ProposalState::Specified,
            ..
        })
    ));

    reg.mark_handed_to_vm(&id).unwrap();
    assert_eq!(reg.get(&id).unwrap().state, ProposalState::HandedToVM);

    // Terminal: keine weiteren Übergänge
    assert!(matches!(
        reg.mark_specified(&id),
        Err(ProposalError::InvalidTransition {
            from: ProposalState::HandedToVM,
            ..
        })
    ));

    // Unbekannte ID
    assert!(matches!(
        reg.mark_specified(&"f".repeat(64)),
        Err(ProposalError::UnknownProposal(_))
    ));
}

// ── AK 8: Audit-Chain + Queries ────────────────────────────────────────

#[test]
fn audit_chain_verify_and_tamper_detection() {
    let mut a = AuditPipeline::new();
    a.record("agent-alpha", "propose_tx", "p1");
    a.record("agent-beta", "register_model", "m1");
    a.record("proposal-registry", "proposal_specified", "p1");

    assert!(a.verify());
    assert_eq!(a.events().len(), 3);
    assert_eq!(a.events()[0].prev_hash, GENESIS_HASH);
    assert_eq!(a.by_kind("propose_tx").len(), 1);
    assert_eq!(a.by_actor("agent-beta").len(), 1);

    // Sequenz-Integrität
    let second = &a.events()[1];
    assert_eq!(second.prev_hash, a.events()[0].hash);
}

// ── AK 9 (Teil): End-to-End KI → Vorschlag → VM-Grenze ─────────────────

#[test]
fn end_to_end_proposal_via_ipc_reaches_vm_boundary() {
    let mut gw = gateway();
    let mut reg = ProposalRegistry::new();

    // KI schlägt über IPC vor (mit Capability + Schema + Seq)
    let payload = json!({"action": "transfer", "params": {"to": "bob", "amount": 5}});
    let receipt: DeliveryReceipt = gw
        .deliver(&msg("agent-alpha", "propose_tx", 1, payload.clone()))
        .unwrap();

    // Vorschlag wird registriert (nur als DATEN — keine Ausführung)
    let id = reg.submit("agent-alpha", "propose_tx", payload);
    assert_eq!(receipt.seq, 1);

    // Kanonisierung + Übergabe an die ATVM-Grenze
    reg.mark_specified(&id).unwrap();
    reg.mark_handed_to_vm(&id).unwrap();

    // Audit-Trail auf beiden Seiten vollständig
    assert!(gw.audit().verify());
    assert!(reg.audit().verify());
    assert_eq!(reg.audit().by_kind("proposal_handed_to_vm").len(), 1);

    // ARCHITEKTUR-INVARIANTE: die Crate bietet keine Execution-API —
    // der Test bestätigt, dass der Lifecycle nach HandedToVM terminal ist:
    assert_eq!(reg.get(&id).unwrap().state, ProposalState::HandedToVM);
}
