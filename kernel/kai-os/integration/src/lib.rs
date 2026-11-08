//! KAI-OS Integration (S25-S26, Issue #111) — die End-to-End-Komposition.
//!
//! Kernprinzip: **Ein System bootet deterministisch oder gar nicht.**
//! Die Boot-Pipeline komponiert alle Crates in FESTER Reihenfolge; jeder
//! Schritt prüft seine Invariante. Ein Fehler stoppt den Boot fail-closed —
//! es gibt kein teilweise gebootetes System.

use kai_os_ai::ipc::{IpcGateway, IpcMessage};
use kai_os_ai::proposal::{ProposalRegistry, ProposalState};
use kai_os_health::gossip::{HealthGossip, HealthStatus};
use kai_os_health::watchdog::{ServiceState, Watchdog};
pub mod handoff;

pub use handoff::{handoff_message, verify_handoff, HandoffKey, HANDOFF_CHAIN_ID};

use kai_os_keyring::keyring::{KeyKind, Keyring};
use kai_os_node::lifecycle::{Lifecycle, State};
use kai_os_runtime::capability::CapabilitySet;
use kai_os_runtime::resources::ResourceManager;
use kai_os_runtime::sandbox::{Operation, Sandbox};
use kai_os_state::snapshot::Snapshot;
use kai_os_state::state::{StateStore, Tx};

/// Deterministische Boot-Konfiguration (Seeds statt RNG — REQ-ENG-002).
pub struct BootConfig {
    pub node_seed: [u8; 32],
    pub agent_seed: [u8; 32],
    /// Der Tx-Log: die einzige Quelle der State-Veränderung (Replay-fähig).
    pub txs: Vec<Tx>,
}

/// Der komplett gebootete Systemzustand.
pub struct BootedSystem {
    pub lifecycle: Lifecycle,
    pub keyring: Keyring,
    pub state: StateStore,
    pub snapshot: Snapshot,
    pub state_root: String,
    pub watchdog: Watchdog,
    pub ipc: IpcGateway,
    pub proposals: ProposalRegistry,
    pub proposal_id: String,
    pub proposal_signature: [u8; 64],
    /// 33-Byte secp256k1-Pubkey des Handoff-Schluessels (oeffentlich pruefbar).
    pub handoff_public_key: [u8; 33],
    pub gossip: HealthGossip,
}

/// Ein Boot-Schritt mit Invarianten-Ergebnis.
#[derive(Debug)]
pub struct BootReport {
    pub steps: Vec<(&'static str, String)>,
}

impl BootReport {
    pub fn ok(&self) -> bool {
        self.steps
            .iter()
            .all(|(_, detail)| !detail.starts_with("FAIL"))
    }
}

/// Boot-Fehler mit benanntem Schritt (fail-closed, keine Teilzustände).
#[derive(Debug)]
pub struct BootError {
    pub step: &'static str,
    pub detail: String,
}

impl std::fmt::Display for BootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "boot failed at step '{}': {}", self.step, self.detail)
    }
}

/// Die deterministische Boot-Pipeline — 9 Schritte, feste Reihenfolge.
pub fn boot(cfg: &BootConfig) -> Result<(BootedSystem, BootReport), BootError> {
    let mut report = BootReport { steps: Vec::new() };
    let fail = |step: &'static str, detail: String| -> BootError {
        BootError {
            step,
            detail: format!("FAIL: {detail}"),
        }
    };

    // 1. Lifecycle: INIT -> BOOT -> RUNNING (fail-closed Boot).
    let mut lifecycle = Lifecycle::new();
    for to in [State::Boot, State::Running] {
        lifecycle
            .transition(to)
            .map_err(|e| fail("lifecycle", e.to_string()))?;
    }
    if lifecycle.current() != State::Running {
        return Err(fail("lifecycle", "not RUNNING after transitions".into()));
    }
    report
        .steps
        .push(("lifecycle", "INIT->BOOT->RUNNING".into()));

    // 2. Keyring: Node- und Agent-Key (Seeds, Keys verlassen die Boundary nie).
    let mut keyring = Keyring::new();
    keyring
        .import_seed("node-key", KeyKind::NodeIdentity, &cfg.node_seed)
        .map_err(|e| fail("keyring", e.to_string()))?;
    keyring
        .import_seed("agent-key", KeyKind::AgentSigning, &cfg.agent_seed)
        .map_err(|e| fail("keyring", e.to_string()))?;
    report.steps.push((
        "keyring",
        "node-key + agent-key imported (public only)".into(),
    ));

    // 3. State: Tx-Log anwenden (einzige Mutationsquelle), Root berechnen.
    let mut state = StateStore::new();
    state.apply_all(&cfg.txs);
    let state_root = state.state_root();
    report.steps.push((
        "state",
        format!("{} entries, root {}", state.len(), &state_root[..16]),
    ));

    // 4. Snapshot: gegen die eigenen vertrauenswürdigen Werte verifizieren.
    let snapshot = Snapshot::from_state(1, "genesis-0001", &state);
    snapshot
        .verify_against(1, &state_root)
        .map_err(|e| fail("snapshot", e.to_string()))?;
    report
        .steps
        .push(("snapshot", "verified against trusted root".into()));

    // 5. Sandbox: Capability-geprüfte Operation (Kette Policy -> Ausführung).
    let caps = CapabilitySet::builder()
        .cpu_ms(100)
        .memory(1024)
        .build()
        .map_err(|e| fail("sandbox", e.to_string()))?;
    caps.validate()
        .map_err(|e| fail("sandbox", e.to_string()))?;
    let mut resources = ResourceManager::new();
    let sandbox = Sandbox::new("vm-exec", caps);
    sandbox
        .execute(Operation::Compute { cpu_ms: 50 }, &mut resources)
        .map_err(|e| fail("sandbox", e.to_string()))?;
    report
        .steps
        .push(("sandbox", "Compute(50ms) within capability".into()));

    // 6. IPC: Agent registrieren, Proposal-Nachricht zustellen (Schema-geprüft).
    let mut ipc = IpcGateway::new();
    ipc.register_agent("kai-agent-1", &["propose_tx", "query_state"]);
    let payload =
        serde_json::json!({"action": "transfer", "params": {"to": "peer-1", "amount": 7}});
    let message = IpcMessage {
        from_agent: "kai-agent-1".into(),
        msg_type: "propose_tx".into(),
        seq: 1,
        payload: payload.clone(),
    };
    ipc.deliver(&message)
        .map_err(|e| fail("ipc", e.to_string()))?;
    if !ipc.audit().verify() {
        return Err(fail("ipc", "audit chain broken after delivery".into()));
    }
    report
        .steps
        .push(("ipc", "propose_tx delivered, schema + audit ok".into()));

    // 7. Proposal: Lifecycle Proposed -> Specified -> HandedToVM (KI schlägt vor, sie führt NICHT aus).
    let mut proposals = ProposalRegistry::new();
    let proposal_id = proposals.submit("kai-agent-1", "propose_tx", payload);
    proposals
        .mark_specified(&proposal_id)
        .map_err(|e| fail("proposal", e.to_string()))?;
    proposals
        .mark_handed_to_vm(&proposal_id)
        .map_err(|e| fail("proposal", e.to_string()))?;
    if proposals.get(&proposal_id).map(|p| p.state) != Some(ProposalState::HandedToVM) {
        return Err(fail("proposal", "not HandedToVM after lifecycle".into()));
    }
    report
        .steps
        .push(("proposal", format!("{} HandedToVM", &proposal_id[..16])));

    // 8. Signatur: Übergabe an die VM-Grenze wird vom Node-Key signiert.
    // Handoff-Signatur: secp256k1/ECDSA (WAL-SIGN-001) mit Domain + Chain-ID.
    // Ed25519 bleibt ausschliesslich im Transport (ATC-STD-PROTOCOL-003).
    let handoff_key = HandoffKey::from_seed(&cfg.node_seed).map_err(|e| fail("signature", e))?;
    let handoff_public_key = handoff_key.public_key();
    let handoff_payload = handoff_message(HANDOFF_CHAIN_ID, &proposal_id);
    let proposal_signature = handoff_key.sign(&handoff_payload);
    if !verify_handoff(&handoff_public_key, &handoff_payload, &proposal_signature) {
        return Err(fail(
            "signature",
            "handoff signature does not verify".into(),
        ));
    }
    report
        .steps
        .push(("signature", "handoff signed by node-key, verified".into()));

    // 9. Health: Services beim Watchdog registrieren, Status gossip-fähig.
    let mut watchdog = Watchdog::new(0);
    for (svc, stale, dead) in [
        ("node-daemon", 3, 5),
        ("vm-exec", 2, 4),
        ("ai-runtime", 3, 5),
    ] {
        watchdog
            .register(svc, stale, dead)
            .map_err(|e| fail("health", e.to_string()))?;
    }
    let mut gossip = HealthGossip::new("self");
    for (svc, _, _) in [
        ("node-daemon", 0, 0),
        ("vm-exec", 0, 0),
        ("ai-runtime", 0, 0),
    ] {
        let s = watchdog
            .state(svc)
            .map_err(|e| fail("health", e.to_string()))?;
        gossip.observe(svc, HealthStatus::from(s), watchdog.now_tick());
    }
    if watchdog
        .state("node-daemon")
        .map_err(|e| fail("health", e.to_string()))?
        != ServiceState::Running
    {
        return Err(fail("health", "node-daemon not Running after boot".into()));
    }
    report.steps.push((
        "health",
        "3 services observed, gossip snapshot ready".into(),
    ));

    Ok((
        BootedSystem {
            lifecycle,
            keyring,
            state,
            snapshot,
            state_root,
            watchdog,
            ipc,
            proposals,
            proposal_id,
            proposal_signature,
            handoff_public_key,
            gossip,
        },
        report,
    ))
}
