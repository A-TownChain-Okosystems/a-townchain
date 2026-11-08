//! S21-S22 Integrationstests — Issue #110. Deterministisch (REQ-ENG-002).

use kai_os_cli::commands::{
    cmd_audit_report, cmd_keys_list, cmd_state_root, cmd_status, dispatch, CliError,
};
use kai_os_health::watchdog::Watchdog;
use kai_os_keyring::keyring::{KeyKind, Keyring};
use kai_os_state::state::StateStore;

#[test]
fn status_reports_all_services_deterministically() {
    let mut wd = Watchdog::new(0);
    wd.register("node-daemon", 3, 5).unwrap();
    wd.register("vm-exec", 2, 4).unwrap();
    wd.tick();

    let out1 = cmd_status(&wd, wd.now_tick());
    let out2 = cmd_status(&wd, wd.now_tick());
    assert_eq!(out1, out2); // deterministisch
    assert!(out1.contains("kai-os status (tick 1)"));
    assert!(out1.contains("node-daemon"));
    assert!(out1.contains("vm-exec"));
}

#[test]
fn keys_list_shows_public_only() {
    let mut ring = Keyring::new();
    ring.import_seed("node-key", KeyKind::NodeIdentity, &[42u8; 32])
        .unwrap();
    ring.import_seed("agent-1", KeyKind::AgentSigning, &[7u8; 32])
        .unwrap();

    let out = cmd_keys_list(&ring);
    assert!(out.contains("node-identity"));
    assert!(out.contains("agent-signing"));
    assert!(out.contains("node-key"));
    // Die Ausgabe enthält NUR Public-Material (32 Bytes als Hex-Tuple), keine Seeds
    assert!(!out.contains("[42, 42")); // Seed 42x würde als Seed erkennbar; public ist abgeleitet
}

#[test]
fn state_root_and_audit_report() {
    let mut state = StateStore::new();
    state.apply(&kai_os_state::state::Tx::Set {
        key: "k".into(),
        value: "v".into(),
    });
    let out = cmd_state_root(&state);
    assert!(out.starts_with("state-root: "));
    assert!(out.contains("(1 entries)"));

    let mut audit = kai_os_ai::audit::AuditPipeline::new();
    audit.record("alpha", "propose_tx", "p1");
    let report = cmd_audit_report(&audit);
    assert!(report.contains("audit: 1 events, chain-ok: true"));
    assert!(report.contains("#1 alpha propose_tx"));
}

#[test]
fn dispatch_fail_closed_on_unknown_commands() {
    assert!(dispatch("status").is_ok());
    assert!(dispatch("keys").is_ok());
    assert!(dispatch("state").is_ok());
    assert!(dispatch("audit").is_ok());
    assert!(matches!(dispatch("rm-rf"), Err(CliError(_))));
    assert!(matches!(dispatch(""), Err(CliError(_))));
}
