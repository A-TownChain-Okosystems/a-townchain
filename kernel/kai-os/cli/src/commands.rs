//! CLI-Kommandos als reine Funktionen über bestehende Crates.

use kai_os_ai::audit::AuditPipeline;
use kai_os_health::watchdog::Watchdog;
use kai_os_keyring::keyring::KeyKind;
use kai_os_keyring::keyring::Keyring;
use kai_os_state::state::StateStore;

#[derive(Debug, PartialEq)]
pub struct CliError(pub String);

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kai-cli: {} (fail-closed)", self.0)
    }
}

fn kind_str(k: KeyKind) -> &'static str {
    match k {
        KeyKind::NodeIdentity => "node-identity",
        KeyKind::AgentSigning => "agent-signing",
    }
}

/// `kai status` — Gesundheitsbericht aller beobachteten Services.
pub fn cmd_status(wd: &Watchdog, tick: u64) -> String {
    let mut out = format!("kai-os status (tick {tick})\n");
    for (id, state, restarts) in wd.report() {
        out.push_str(&format!("  {id}: {state:?} (restarts: {restarts})\n"));
    }
    out
}

/// `kai keys list` — öffentliche Key-Informationen (NIEMALS Secrets).
pub fn cmd_keys_list(ring: &Keyring) -> String {
    let mut out = String::from("keyring (public information only):\n");
    let mut keys = ring.list();
    keys.sort_by(|a, b| a.0.cmp(&b.0)); // deterministische Ausgabe (nach Key-ID)
    for (id, public, kind) in keys {
        out.push_str(&format!(
            "  {id} [{kind}] {public:?}\n",
            kind = kind_str(kind)
        ));
    }
    out
}

/// `kai state root` — aktueller State-Root.
pub fn cmd_state_root(state: &StateStore) -> String {
    format!(
        "state-root: {} ({} entries)\n",
        state.state_root(),
        state.len()
    )
}

/// `kai audit report` — deterministische Audit-Zusammenfassung.
pub fn cmd_audit_report(pipeline: &AuditPipeline) -> String {
    let events = pipeline.events();
    let mut out = format!(
        "audit: {} events, chain-ok: {}\n",
        events.len(),
        pipeline.verify()
    );
    for e in events {
        out.push_str(&format!("  #{} {} {}\n", e.seq, e.actor, e.kind));
    }
    out
}

/// Kommando-Dispatcher — fail-closed bei unbekannten Kommandos.
pub fn dispatch(cmd: &str) -> Result<String, CliError> {
    match cmd {
        "status" | "keys" | "state" | "audit" => Ok(format!("usage: kai {cmd} <args>")),
        _ => Err(CliError(format!("unknown command '{cmd}'"))),
    }
}
