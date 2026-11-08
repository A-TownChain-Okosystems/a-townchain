//! S01-S03 Integrationstests — Issue #102 Akzeptanzkriterien.
//! Deterministisch: kein RNG, keine Wall-Clock-Abhängigkeiten (REQ-ENG-002).

use kai_os_node::audit::AuditLogger;
use kai_os_node::config::{ConfigError, NodeConfig, KNOWN_SUBSYSTEMS};
use kai_os_node::ensure_not_root;
use kai_os_node::health::HealthStatus;
use kai_os_node::lifecycle::{can_transition, Lifecycle, State};
use kai_os_node::shutdown::ShutdownCoordinator;
use kai_os_node::supervisor::{StubSubsystem, Supervisor};

// ── AK 1+3: Lifecycle-State-Machine ──────────────────────────────────────

#[test]
fn lifecycle_legal_transitions() {
    let mut lc = Lifecycle::new();
    assert_eq!(lc.current(), State::Init);

    lc.transition(State::Boot).unwrap();
    lc.transition(State::Running).unwrap();
    lc.transition(State::Degraded).unwrap();
    lc.transition(State::Running).unwrap();
    lc.transition(State::Shutdown).unwrap();
    lc.transition(State::Stopped).unwrap();

    assert_eq!(lc.history().len(), 6);
    // Boot-Fail-Pfad: Init -> Shutdown direkt erlaubt.
    let mut lc2 = Lifecycle::new();
    lc2.transition(State::Shutdown).unwrap();
    assert_eq!(lc2.current(), State::Shutdown);
}

#[test]
fn lifecycle_illegal_transitions_rejected() {
    use State::*;
    // Vollständige Negativmatrix: alles Nicht-Kanonische muss scheitern.
    let illegal = [
        (Init, Init),
        (Init, Running),
        (Init, Degraded),
        (Init, Stopped),
        (Boot, Init),
        (Boot, Boot),
        (Boot, Stopped),
        (Running, Init),
        (Running, Boot),
        (Running, Running),
        (Running, Stopped),
        (Degraded, Init),
        (Degraded, Boot),
        (Degraded, Degraded),
        (Degraded, Stopped),
        (Shutdown, Init),
        (Shutdown, Boot),
        (Shutdown, Running),
        (Shutdown, Degraded),
        (Shutdown, Shutdown),
        (Stopped, Init),
        (Stopped, Boot),
        (Stopped, Running),
        (Stopped, Degraded),
        (Stopped, Shutdown),
        (Stopped, Stopped),
    ];
    for (from, to) in illegal {
        assert!(
            !can_transition(from, to),
            "{from:?} -> {to:?} muss illegal sein"
        );
        let mut lc = Lifecycle::at(from);
        assert!(
            lc.transition(to).is_err(),
            "{from:?} -> {to:?} muss Err geben"
        );
    }
}

// ── AK 2: Supervisor-Überwachung + Crash-Restart-Policy ─────────────────

#[test]
fn supervisor_boot_fail_closed() {
    // Subsystem 2 scheitert beim Start -> bereits gestartete werden gestoppt.
    let mut audit = AuditLogger::in_memory();
    let mut sup = Supervisor::new(3, 0);
    sup.register(Box::new(StubSubsystem::new("consensus")));
    sup.register(Box::new(StubSubsystem::new("p2p").failing_starts(1)));
    sup.register(Box::new(StubSubsystem::new("storage")));

    let err = sup.start_all(&mut audit).unwrap_err();
    assert!(err.0.contains("p2p"));
    // Fail-closed: Boot bricht ab.
    assert_ne!(sup.overall_health(), HealthStatus::Healthy);
}

#[test]
fn supervisor_restart_policy_recovers() {
    // Tick-Crash -> Restart innerhalb max_restarts -> wieder Healthy.
    let mut audit = AuditLogger::in_memory();
    let mut sup = Supervisor::new(3, 0);
    sup.register(Box::new(StubSubsystem::new("consensus").failing_ticks(2)));
    sup.start_all(&mut audit).unwrap();

    let h1 = sup.tick_all(&mut audit); // 1. Crash -> Restart #1
    let h2 = sup.tick_all(&mut audit); // 2. Crash -> Restart #2
    let h3 = sup.tick_all(&mut audit); // stabil

    assert_eq!(h1, HealthStatus::Healthy);
    assert_eq!(h2, HealthStatus::Healthy);
    assert_eq!(h3, HealthStatus::Healthy);
    assert_eq!(sup.restart_count("consensus"), Some(2));
}

#[test]
fn supervisor_max_restarts_then_unhealthy() {
    // Dauer-Crash -> max_restarts überschritten -> Unhealthy -> Node muss Degraded werden.
    let mut audit = AuditLogger::in_memory();
    let mut sup = Supervisor::new(2, 0);
    sup.register(Box::new(StubSubsystem::new("p2p").failing_ticks(99)));
    sup.start_all(&mut audit).unwrap();

    let _ = sup.tick_all(&mut audit); // Restart #1
    let _ = sup.tick_all(&mut audit); // Restart #2
    let h = sup.tick_all(&mut audit); //exceeds max -> Unhealthy

    assert_eq!(h, HealthStatus::Unhealthy);
    assert_eq!(sup.restart_count("p2p"), Some(2));
}

// ── AK 8: Crash-Recovery (Persistenz) ───────────────────────────────────

#[test]
fn supervisor_state_persist_and_restore() {
    let dir = std::env::temp_dir().join("kai-os-crash-recovery-test");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("supervisor_state.json");

    let mut audit = AuditLogger::in_memory();
    let mut sup = Supervisor::new(3, 0);
    sup.register(Box::new(StubSubsystem::new("consensus").failing_ticks(1)));
    sup.register(Box::new(StubSubsystem::new("p2p")));
    sup.start_all(&mut audit).unwrap();
    let _ = sup.tick_all(&mut audit);
    sup.persist_state(&path).unwrap();

    // "Daemon-Neustart": neuer Supervisor lädt den Zustand des alten Laufs.
    let mut sup2 = Supervisor::new(3, 0);
    sup2.register(Box::new(StubSubsystem::new("consensus")));
    sup2.register(Box::new(StubSubsystem::new("p2p")));
    let state = Supervisor::load_state(&path).unwrap();
    sup2.restore(&state);

    assert_eq!(
        sup2.restart_count("consensus"),
        sup.restart_count("consensus")
    );
    assert_eq!(sup2.restart_count("p2p"), sup.restart_count("p2p"));
    let _ = std::fs::remove_file(&path);
}

// ── AK 5: Config fail-closed ────────────────────────────────────────────

#[test]
fn config_missing_file_fails() {
    let res = NodeConfig::from_file(std::path::Path::new("/nicht/vorhanden.json"));
    assert!(matches!(res, Err(ConfigError::Read(_))));
}

#[test]
fn config_invalid_json_fails() {
    let res = NodeConfig::from_json("{ nicht json");
    assert!(matches!(res, Err(ConfigError::Parse(_))));
}

#[test]
fn config_out_of_range_fails() {
    let mut cfg = NodeConfig::default_for_test();
    cfg.max_restarts = 99;
    assert!(matches!(cfg.validate(), Err(ConfigError::Validation(_))));

    let mut cfg = NodeConfig::default_for_test();
    cfg.tick_interval_ms = 0;
    assert!(matches!(cfg.validate(), Err(ConfigError::Validation(_))));

    let mut cfg = NodeConfig::default_for_test();
    cfg.enabled_subsystems = vec!["unbekanntes-subsystem".into()];
    assert!(matches!(cfg.validate(), Err(ConfigError::Validation(_))));

    let mut cfg = NodeConfig::default_for_test();
    cfg.node_name = "  ".into();
    assert!(matches!(cfg.validate(), Err(ConfigError::Validation(_))));
}

#[test]
fn config_valid_passes() {
    let cfg = NodeConfig::default_for_test();
    assert!(cfg.validate().is_ok());
}

// ── AK 7: Audit-Events + Integritäts-Hash-Chain ─────────────────────────

#[test]
fn audit_chain_valid_and_tamper_detected() {
    let mut audit = AuditLogger::in_memory();
    audit.record("lifecycle", "Init -> Boot").unwrap();
    audit.record("supervisor", "started consensus").unwrap();
    audit.record("lifecycle", "Boot -> Running").unwrap();
    assert!(audit.verify_chain());
    assert_eq!(audit.events().len(), 3);

    // Manipulation -> Chain bricht.
    let mut tampered = AuditLogger::in_memory();
    tampered.record("lifecycle", "Init -> Boot").unwrap();
    tampered.record("daemon", "gefaelschte detail").unwrap();
    tampered.events_mut()[1].detail = "manipuliert".into();
    assert!(!tampered.verify_chain());
}

// ── AK 1: Graceful Shutdown (umgekehrte Reihenfolge, Report) ────────────

#[test]
fn shutdown_reverse_order_and_report() {
    let mut audit = AuditLogger::in_memory();
    let mut sup = Supervisor::new(3, 0);
    for name in KNOWN_SUBSYSTEMS {
        sup.register(Box::new(StubSubsystem::new(name)));
    }
    sup.start_all(&mut audit).unwrap();

    let report = ShutdownCoordinator::shutdown_all(&mut sup);
    assert!(report.clean);
    // Umgekehrte Start-Reihenfolge: security zuerst, consensus zuletzt.
    assert_eq!(report.stopped_in_order.first().unwrap(), "security");
    assert_eq!(report.stopped_in_order.last().unwrap(), "consensus");
    assert_eq!(report.stopped_in_order.len(), KNOWN_SUBSYSTEMS.len());
    assert!(report.to_json().len() > 10);
}

// ── AK 6: Root-Guard ────────────────────────────────────────────────────

#[test]
fn root_guard_fail_closed() {
    // euid 0 ohne explizite Freigabe -> Boot verweigert.
    assert!(ensure_not_root(false, 0).is_err());
    // Normaler Nutzer -> ok.
    assert!(ensure_not_root(false, 1000).is_ok());
    // Explizite Freigabe -> ok (dokumentierte Ausnahme).
    assert!(ensure_not_root(true, 0).is_ok());
}

// ── Health-Aggregation ──────────────────────────────────────────────────

#[test]
fn health_worst_of_aggregation() {
    assert_eq!(
        HealthStatus::worst(HealthStatus::Healthy, HealthStatus::Degraded),
        HealthStatus::Degraded
    );
    assert_eq!(
        HealthStatus::worst(HealthStatus::Degraded, HealthStatus::Unhealthy),
        HealthStatus::Unhealthy
    );
    assert_eq!(
        HealthStatus::worst(HealthStatus::Healthy, HealthStatus::Healthy),
        HealthStatus::Healthy
    );
}
