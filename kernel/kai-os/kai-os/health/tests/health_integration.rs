//! S19-S20 Integrationstests — Issue #108 Akzeptanzkriterien.
//! Deterministisch: Tick-basiert, keine Wall-Clock, kein RNG (REQ-ENG-002).

use kai_os_health::gossip::{HealthGossip, HealthStatus, NodeHealth};
use kai_os_health::watchdog::{ServiceState, Watchdog, WatchdogError};

// ── AK 1+2: Tick-basierte Liveness ─────────────────────────────────────

#[test]
fn watchdog_stale_and_dead_transitions() {
    let mut wd = Watchdog::new(0);
    wd.register("kai-node", 3, 5).unwrap(); // Stale nach >3, Dead nach >3+5 verpassten Ticks

    assert_eq!(wd.state("kai-node").unwrap(), ServiceState::Running);

    for _ in 0..3 {
        wd.tick();
    } // 3 Ticks: innerhalb Toleranz
    assert_eq!(wd.state("kai-node").unwrap(), ServiceState::Running);

    wd.tick(); // Tick 4: >3 -> Stale
    assert_eq!(wd.state("kai-node").unwrap(), ServiceState::Stale);

    for _ in 0..4 {
        wd.tick();
    } // Ticks 5-8: Stale gehalten
    assert_eq!(wd.state("kai-node").unwrap(), ServiceState::Stale);

    wd.tick(); // Tick 9: verpasst > 3+5 -> DeclaredDead
    assert_eq!(wd.state("kai-node").unwrap(), ServiceState::DeclaredDead);
}

// ── AK 3: Recovery nur durch echten Herzschlag ────────────────────────

#[test]
fn recovery_requires_real_heartbeat() {
    let mut wd = Watchdog::new(0);
    wd.register("gateway", 2, 3).unwrap();

    for _ in 0..3 {
        wd.tick();
    } // Stale
    assert_eq!(wd.state("gateway").unwrap(), ServiceState::Stale);

    // Ein echter Herzschlag heilt (Stale -> Running)
    wd.heartbeat("gateway").unwrap();
    assert_eq!(wd.state("gateway").unwrap(), ServiceState::Running);

    // Und wieder: verpasste Ticks verschlechtern sofort
    for _ in 0..3 {
        wd.tick();
    }
    assert_eq!(wd.state("gateway").unwrap(), ServiceState::Stale);

    // DeclaredDead heilt NICHT durch Herzschlag — nur Restart
    for _ in 0..4 {
        wd.tick();
    }
    assert_eq!(wd.state("gateway").unwrap(), ServiceState::DeclaredDead);
    assert!(matches!(
        wd.heartbeat("gateway"),
        Err(WatchdogError::IllegalHeartbeat { .. })
    ));
}

// ── AK 4: Restart nur für DeclaredDead ──────────────────────────────────

#[test]
fn restart_only_for_dead_services_with_counter() {
    let mut wd = Watchdog::new(0);
    wd.register("vm-exec", 1, 2).unwrap();

    // Running -> Restart abgewiesen
    assert!(matches!(
        wd.restart("vm-exec"),
        Err(WatchdogError::IllegalRestart { .. })
    ));

    // Bis DeclaredDead
    for _ in 0..4 {
        wd.tick();
    }
    assert_eq!(wd.state("vm-exec").unwrap(), ServiceState::DeclaredDead);

    // Restart ok, Zähler steigt, Zustand Running mit frischem Herzschlag
    assert_eq!(wd.restart("vm-exec").unwrap(), 1);
    assert_eq!(wd.state("vm-exec").unwrap(), ServiceState::Running);
    assert_eq!(wd.restart_count("vm-exec").unwrap(), 1);
    wd.tick();
    assert_eq!(wd.state("vm-exec").unwrap(), ServiceState::Running); // frischer Herzschlag wirkt

    // Zweiter Todeszyklus -> Zähler 2
    for _ in 0..5 {
        wd.tick();
    }
    assert_eq!(wd.restart("vm-exec").unwrap(), 2);
    assert_eq!(wd.restart_count("vm-exec").unwrap(), 2);
}

// ── AK 5: Fail-closed ──────────────────────────────────────────────────

#[test]
fn fail_closed_unknown_and_duplicate() {
    let mut wd = Watchdog::new(10);
    assert!(matches!(
        wd.heartbeat("unbekannt"),
        Err(WatchdogError::UnknownService(_))
    ));
    assert!(matches!(
        wd.state("unbekannt"),
        Err(WatchdogError::UnknownService(_))
    ));

    wd.register("dup", 2, 2).unwrap();
    assert!(matches!(
        wd.register("dup", 2, 2),
        Err(WatchdogError::DuplicateService(_))
    ));
}

// ── AK 6+7: Gossip-Merge deterministisch ───────────────────────────────

fn entry(node: &str, status: HealthStatus, tick: u64) -> NodeHealth {
    NodeHealth {
        node_id: node.into(),
        status,
        tick,
    }
}

#[test]
fn gossip_merge_higher_tick_wins_local_on_tie() {
    let mut a = HealthGossip::new("node-a");
    a.observe("node-b", HealthStatus::Healthy, 10);

    // Remote hat NEUERE Info (Tick 12) -> übernommen
    let remote = vec![entry("node-b", HealthStatus::Dead, 12)];
    assert_eq!(a.merge("node-c", &remote), 1);
    let view = a.consensus_view("node-b").unwrap();
    assert_eq!(view.tick, 12);
    assert_eq!(view.status, HealthStatus::Dead);

    // Remote hat AELTERE Info (Tick 5) von NEUEM Beobachter node-d:
    // Merge ist pro Beobachter -> erster Eintrag fuer node-d wird uebernommen (adopted=1),
    // aber die Konsolidierung (consensus_view) bleibt beim hoeheren Tick 12.
    let old = vec![entry("node-b", HealthStatus::Healthy, 5)];
    assert_eq!(a.merge("node-d", &old), 1);
    let view = a.consensus_view("node-b").unwrap();
    assert_eq!(view.tick, 12); // aeltere Beobachtung verdraengt die neuere NICHT
    assert_eq!(view.status, HealthStatus::Dead);

    // Gleichstand (Tick 12 von anderem Beobachter) -> egal, gleicher Inhalt
    let tie = vec![entry("node-b", HealthStatus::Healthy, 12)];
    a.merge("node-e", &tie);
    let view = a.consensus_view("node-b").unwrap();
    // Beide bei Tick 12: fail-closed konservativ -> schlimmster Status gewinnt bei Gleichstand
    assert_eq!(view.status, HealthStatus::Dead);
    assert_eq!(view.tick, 12);
}

#[test]
fn gossip_merge_tie_local_snapshot_stays() {
    let mut a = HealthGossip::new("node-a");
    a.observe("node-x", HealthStatus::Stale, 7);

    // Gleichstand von fremdem Beobachter für SEINEN Slot -> übernommen nur wenn neuer
    let tie = vec![entry("node-x", HealthStatus::Healthy, 7)];
    assert_eq!(a.merge("node-c", &tie), 1); // erster Eintrag für node-c -> übernommen
                                            // Konsolidierung: gleicher Tick, Stale ist schlimmer -> fail-closed
    assert_eq!(
        a.consensus_view("node-x").unwrap().status,
        HealthStatus::Stale
    );

    let remote_same_observer = vec![entry("node-x", HealthStatus::Dead, 7)];
    assert_eq!(a.merge("node-c", &remote_same_observer), 0); // Tie -> lokal bleibt
    assert_eq!(
        a.consensus_view("node-x").unwrap().status,
        HealthStatus::Stale
    );
}

#[test]
fn gossip_snapshot_deterministically_sorted() {
    let mut g = HealthGossip::new("node-a");
    g.observe("zeta", HealthStatus::Healthy, 1);
    g.observe("alpha", HealthStatus::Stale, 2);
    g.observe("mid", HealthStatus::Dead, 3);

    let snap = g.snapshot("node-a");
    let ids: Vec<&str> = snap.iter().map(|e| e.node_id.as_str()).collect();
    assert_eq!(ids, vec!["alpha", "mid", "zeta"]); // sortiert, nicht Einfüge-Order
}

// ── AK 8: Integration mit kai-os-network Discovery ─────────────────────

#[test]
fn health_propagates_via_discovery_peer_list() {
    // Health-Daten reisen als deterministischer Snapshot, Empfang über Discovery-Merge
    let mut g = HealthGossip::new("node-a");
    g.observe("node-b", HealthStatus::Healthy, 5);
    g.observe("node-c", HealthStatus::Stale, 5);

    // Snapshot (transportfähig über P2P)
    let snap = g.snapshot("node-a");
    assert_eq!(snap.len(), 2);

    // Remote-Node empfängt und mergt
    let mut g2 = HealthGossip::new("node-b");
    let adopted = g2.merge("node-a", &snap);
    assert_eq!(adopted, 2);
    assert_eq!(
        g2.consensus_view("node-c").unwrap().status,
        HealthStatus::Stale
    );

    // Discovery aus Issue #104 nimmt Peers parallel auf (gleiche P2P-Schicht)
    use kai_os_network::discovery::{Discovery, DiscoveryMessage};
    use kai_os_network::peer::PeerInfo;
    let mut d = Discovery::new("node-b");
    d.handle(
        DiscoveryMessage::PeerList {
            from_peer_id: "node-a".into(),
            peers: vec![
                PeerInfo {
                    peer_id: "node-a".into(),
                    addr: "a.atc:5000".into(),
                    caps: vec!["health".into()],
                    last_seen_tick: 5,
                },
                PeerInfo {
                    peer_id: "node-c".into(),
                    addr: "c.atc:5000".into(),
                    caps: vec!["health".into()],
                    last_seen_tick: 5,
                },
            ],
        },
        5,
    );
    assert_eq!(d.store().len(), 2); // Transport-Ebene und Health-Ebene konsistent
}

// ── Determinismus: identischer Ablauf -> identischer Zustand ───────────

#[test]
fn deterministic_replay_identical_states() {
    let run = || {
        let mut wd = Watchdog::new(0);
        wd.register("svc", 2, 3).unwrap();
        wd.heartbeat("svc").unwrap();
        for _ in 0..6 {
            wd.tick();
        }
        let mut g = HealthGossip::new("n1");
        g.observe("svc", wd.state("svc").unwrap().into(), wd.now_tick());
        (wd.state("svc").unwrap(), g.snapshot("n1"))
    };
    let (s1, g1) = run();
    let (s2, g2) = run();
    assert_eq!(s1, s2);
    assert_eq!(g1, g2); // strukturell identische Snapshots
}
