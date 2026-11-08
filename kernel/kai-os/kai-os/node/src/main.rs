//! KAI-OS Node Daemon (S01-S03, Issue #102).
//! Orchestrierungsebene — die Subsystem-Implementierungen entstehen in S04-S12.
//!
//! Boot-Sequenz (fail-closed): Config validieren -> Root-Guard -> Audit ->
//! Lifecycle INIT->BOOT -> Supervisor start_all -> RUNNING -> Tick-Loop ->
//! SIGTERM/SIGINT -> graceful Shutdown -> STOPPED.

use kai_os_node::audit::AuditLogger;
use kai_os_node::config::{NodeConfig, KNOWN_SUBSYSTEMS};
use kai_os_node::ensure_not_root;
use kai_os_node::health::HealthStatus;
use kai_os_node::lifecycle::{Lifecycle, State};
use kai_os_node::shutdown::ShutdownCoordinator;
use kai_os_node::supervisor::{StubSubsystem, Supervisor};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

fn main() {
    let exit_code = run();
    std::process::exit(exit_code);
}

fn run() -> i32 {
    // 1. Config laden — fail-closed: fehlt/ungültig -> Exit 2, kein Boot.
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "kai-os.json".to_string());
    let cfg = match NodeConfig::from_file(std::path::Path::new(&config_path)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[kai-os-node] BOOT REFUSED: {e}");
            return 2;
        }
    };

    // 2. Root-Guard — kein Daemon unter Root (außer explizit erlaubt).
    let euid = unsafe { libc::geteuid() };
    if let Err(reason) = ensure_not_root(cfg.allow_root, euid) {
        eprintln!("[kai-os-node] {reason}");
        return 2;
    }

    // 3. Audit-Logger (append-only, Hash-Chain).
    let audit_path = std::path::Path::new(&cfg.data_dir).join("audit.log");
    let mut audit = match AuditLogger::to_file(audit_path) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[kai-os-node] audit init failed: {e}");
            return 2;
        }
    };
    let _ = audit.record(
        "daemon",
        &format!("boot: node={} euid={euid}", cfg.node_name),
    );

    // 4. Lifecycle INIT -> BOOT (Übergänge werden unten explizit auditiert).
    let mut lc = Lifecycle::new();
    let _ = lc.transition(State::Boot).expect("INIT -> BOOT legal");
    let _ = audit.record("lifecycle", "Init -> Boot");

    // 5. Supervisor + Subsysteme (S01: Stubs, S04-S12: echte Implementierungen).
    let mut sup = Supervisor::new(cfg.max_restarts, cfg.restart_backoff_ms);
    for name in &cfg.enabled_subsystems {
        let static_name = KNOWN_SUBSYSTEMS.iter().find(|s| *s == name).copied();
        if let Some(sn) = static_name {
            sup.register(Box::new(StubSubsystem::new(sn)));
        }
    }

    if let Err(e) = sup.start_all(&mut audit) {
        let _ = audit.record("daemon", &format!("BOOT FAILED: {e}"));
        let _ = lc.transition(State::Shutdown);
        let _ = audit.record("lifecycle", "Boot -> Shutdown (boot failed)");
        let _ = lc.transition(State::Stopped);
        eprintln!("[kai-os-node] BOOT FAILED: {e}");
        return 1;
    }
    let _ = lc
        .transition(State::Running)
        .expect("BOOT -> RUNNING legal");
    let _ = audit.record("lifecycle", "Boot -> Running");

    // 6. Signal-Handler: SIGTERM/SIGINT -> geordnetes Herunterfahren.
    if ctrlc::set_handler(|| SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst)).is_err() {
        let _ = audit.record("daemon", "WARN: signal handler not installed");
    }

    // 7. Tick-Loop: Überwachung + Health-Aggregation + Degraded-Recovery.
    let tick = Duration::from_millis(cfg.tick_interval_ms);
    loop {
        if SHUTDOWN_REQUESTED.load(Ordering::SeqCst) {
            let _ = audit.record("daemon", "shutdown requested (signal)");
            break;
        }
        std::thread::sleep(tick);
        match sup.tick_all(&mut audit) {
            HealthStatus::Healthy => {
                if lc.current() == State::Degraded {
                    let _ = lc.transition(State::Running);
                    let _ = audit.record("lifecycle", "Degraded -> Running (recovered)");
                }
            }
            _ => {
                if lc.current() == State::Running {
                    let _ = lc.transition(State::Degraded);
                    let _ = audit.record("lifecycle", "Running -> Degraded (unhealthy subsystem)");
                }
            }
        }
    }

    // 8. Graceful Shutdown: Drain in umgekehrter Start-Reihenfolge.
    let _ = lc.transition(State::Shutdown);
    let _ = audit.record("lifecycle", "-> Shutdown");
    let report = ShutdownCoordinator::shutdown_all(&mut sup);
    let _ = audit.record("shutdown", &report.to_json().replace('\n', " "));
    let _ = lc.transition(State::Stopped);
    let _ = audit.record("lifecycle", "Shutdown -> Stopped");

    eprintln!(
        "[kai-os-node] stopped: {} stopped cleanly, {} forced",
        report.stopped_in_order.len(),
        report.forced.len()
    );
    0
}
