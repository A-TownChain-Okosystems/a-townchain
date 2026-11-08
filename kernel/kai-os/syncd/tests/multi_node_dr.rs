//! Issue #113 — Multi-Node Disaster-Recovery E2E.
//! ECHTE OS-PROZESSE (keine Threads), echtes TCP, echte Kills.
//! Deterministisch: feste Txs, keine Uhr im Protokoll, keine Zufallsschlüssel.

use kai_os_state::wal::PersistentState;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_kai-sync-node");

struct NodeProc {
    child: Child,
    stdout: BufReader<std::process::ChildStdout>,
}

impl NodeProc {
    fn spawn(args: &[&str]) -> Self {
        let mut child = Command::new(BIN)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("kai-sync-node muss spawnbar sein");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        Self { child, stdout }
    }

    /// Auf eine Zeile mit erwartetem Präfix warten (blockierend — das
    /// Protokoll druckt jede Phase sofort und geflusht).
    fn expect_line(&mut self, prefix: &str) -> String {
        let mut line = String::new();
        loop {
            line.clear();
            let n = self.stdout.read_line(&mut line).expect("stdout lesbar");
            assert!(n > 0, "Prozess vor '{prefix}' beendet");
            let t = line.trim();
            if t.starts_with(prefix) {
                return t.to_string();
            }
            // Andere Zeilen (z. B. SYNC-FAILED-Retry-Meldungen) übergehen
        }
    }

    fn root_of(&mut self, prefix: &str) -> String {
        self.expect_line(prefix)
            .split(' ')
            .nth(1)
            .expect("root-feld")
            .to_string()
    }

    fn kill(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }

    fn wait_exit_ok(&mut self) {
        let status = self.child.wait().expect("wait");
        assert!(status.success(), "Prozess muss mit 0 enden, war: {status}");
    }
}

impl Drop for NodeProc {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn test_dir(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("kai-dr-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn load_root(dir: &std::path::Path) -> String {
    let ps = PersistentState::open(&dir.join("state.json"), &dir.join("wal.log")).unwrap();
    ps.store.state_root()
}

// ── AK 1: Zwei Prozesse, Sync über echtes TCP, identische Roots ─────────

#[test]
fn two_processes_sync_over_real_tcp() {
    let dir_b = test_dir("sync-b");
    let dir_a = test_dir("sync-a");

    let mut sink = NodeProc::spawn(&["--role", "sink", "--dir", dir_b.to_str().unwrap()]);
    let listen = sink.expect_line("LISTEN");
    let addr = listen.split(' ').nth(1).expect("addr").to_string();

    let mut source = NodeProc::spawn(&[
        "--role",
        "source",
        "--dir",
        dir_a.to_str().unwrap(),
        "--peer",
        &addr,
        "--apply",
        "5",
    ]);
    let applied = source.root_of("APPLIED");
    let synced_a = source.root_of("SYNC-OK");
    let synced_b = sink.root_of("SYNC-OK");

    // Drei Roots müssen übereinstimmen: Anwendung, Bestätigung, Persistenz
    assert_eq!(
        applied, synced_a,
        "Ack-Root muss dem Anwendungs-Root entsprechen"
    );
    assert_eq!(
        synced_a, synced_b,
        "beide Knoten müssen denselben verifizierten Root haben"
    );
    source.wait_exit_ok();
    sink.wait_exit_ok();

    // Und auf Disk: beide PersistentState laden identisch
    assert_eq!(load_root(&dir_a), synced_a);
    assert_eq!(load_root(&dir_b), synced_b);
}

// ── AK 2: Absturz der Source MITTEN im Lauf — Recovery aus eigenem WAL ─

#[test]
fn crashed_source_recovers_and_syncs() {
    let dir_b = test_dir("crashsrc-b");
    let dir_a = test_dir("crashsrc-a");

    let mut sink = NodeProc::spawn(&["--role", "sink", "--dir", dir_b.to_str().unwrap()]);
    let addr = sink
        .expect_line("LISTEN")
        .split(' ')
        .nth(1)
        .expect("addr")
        .to_string();

    // Quelle: 5 Txs, dann bewusst langes Orchestrierungs-Fenster —
    // der Absturz trifft NACH dem Checkpoint, VOR dem Sync
    let mut source = NodeProc::spawn(&[
        "--role",
        "source",
        "--dir",
        dir_a.to_str().unwrap(),
        "--peer",
        &addr,
        "--apply",
        "5",
        "--delay-ms",
        "3000",
    ]);
    let applied_before_crash = source.root_of("APPLIED");
    source.kill(); // ECHTER PROZESS-TOD
    drop(source);

    // Neustart mit GLEICHEM Verzeichnis: Recovery aus WAL/Snapshot (G2-D),
    // keine neuen Txs — danach Sync-Resumption
    let mut revived = NodeProc::spawn(&[
        "--role",
        "source",
        "--dir",
        dir_a.to_str().unwrap(),
        "--peer",
        &addr,
        "--apply",
        "0",
        "--retries",
        "10",
    ]);
    let recovered = revived.root_of("APPLIED");
    assert_eq!(
        recovered, applied_before_crash,
        "Recovery muss den Zustand VOR dem Absturz reproduzieren"
    );
    let synced = revived.root_of("SYNC-OK");
    assert_eq!(synced, recovered);
    let synced_b = sink.root_of("SYNC-OK");
    assert_eq!(
        synced_b, synced,
        "Sink muss nach Source-Absturz denselben Root übernehmen"
    );
    revived.wait_exit_ok();
    sink.wait_exit_ok();
}

// ── AK 3: Partition (Sink tot) — kein Tx-Verlust, Resumption danach ────

#[test]
fn partition_then_resumption_without_tx_loss() {
    let dir_b = test_dir("part-b");
    let dir_a = test_dir("part-a");

    // Partition: Sink lauscht kurz und stirbt VOR jeder Verbindung
    let mut dead_sink = NodeProc::spawn(&["--role", "sink", "--dir", dir_b.to_str().unwrap()]);
    let addr = dead_sink
        .expect_line("LISTEN")
        .split(' ')
        .nth(1)
        .expect("addr")
        .to_string();
    dead_sink.kill();
    drop(dead_sink);

    // Source versucht Sync gegen die tote Gegenstelle — muss geordnet scheitern
    let mut source = NodeProc::spawn(&[
        "--role",
        "source",
        "--dir",
        dir_a.to_str().unwrap(),
        "--peer",
        &addr,
        "--apply",
        "3",
        "--retries",
        "2",
    ]);
    let applied = source.root_of("APPLIED");
    let status = source.child.wait().expect("wait");
    assert!(
        !status.success(),
        "Source muss bei erschöpftem Retry mit Fehler enden (Partition)"
    );
    drop(source);

    // Resumption: Sink zurück, Source startet neu — KEIN Tx-Verlust,
    // KEIN Doppel-Apply (Zustand kommt aus eigenem WAL, identischer Root)
    let mut sink = NodeProc::spawn(&["--role", "sink", "--dir", dir_b.to_str().unwrap()]);
    let addr2 = sink
        .expect_line("LISTEN")
        .split(' ')
        .nth(1)
        .expect("addr")
        .to_string();
    let mut source2 = NodeProc::spawn(&[
        "--role",
        "source",
        "--dir",
        dir_a.to_str().unwrap(),
        "--peer",
        &addr2,
        "--apply",
        "0",
        "--retries",
        "10",
    ]);
    assert_eq!(
        source2.root_of("APPLIED"),
        applied,
        "kein Tx-Verlust durch Partition"
    );
    let synced = source2.root_of("SYNC-OK");
    assert_eq!(synced, applied, "kein Doppel-Apply — Root unverändert");
    assert_eq!(sink.root_of("SYNC-OK"), synced);
    source2.wait_exit_ok();
    sink.wait_exit_ok();
}

// ── AK 4: Beide Knoten tot — unabhängiges Recovery, Konvergenz beim Sync ─

#[test]
fn both_crashed_converge_on_restart() {
    let dir_b = test_dir("both-b");
    let dir_a = test_dir("both-a");

    // Runde 1: normaler Sync (Baseline)
    let mut sink = NodeProc::spawn(&["--role", "sink", "--dir", dir_b.to_str().unwrap()]);
    let addr = sink
        .expect_line("LISTEN")
        .split(' ')
        .nth(1)
        .expect("addr")
        .to_string();
    let mut source = NodeProc::spawn(&[
        "--role",
        "source",
        "--dir",
        dir_a.to_str().unwrap(),
        "--peer",
        &addr,
        "--apply",
        "5",
    ]);
    let baseline = source.root_of("SYNC-OK");
    assert_eq!(sink.root_of("SYNC-OK"), baseline);
    source.wait_exit_ok();
    sink.wait_exit_ok();
    drop(source);
    drop(sink);
    // Beide Prozesse sind tot — "beide abgestürzt" zwischen den Runden

    // Runde 2: BEIDE unabhängig aus Disk recovern (keine Kommunikation nötig),
    // danach Re-Sync — Konvergenz muss den Baseline-Root reproduzieren
    let mut sink2 = NodeProc::spawn(&["--role", "sink", "--dir", dir_b.to_str().unwrap()]);
    let addr2 = sink2
        .expect_line("LISTEN")
        .split(' ')
        .nth(1)
        .expect("addr")
        .to_string();
    let mut source2 = NodeProc::spawn(&[
        "--role",
        "source",
        "--dir",
        dir_a.to_str().unwrap(),
        "--peer",
        &addr2,
        "--apply",
        "0",
        "--retries",
        "10",
    ]);
    let recovered_a = source2.root_of("APPLIED");
    assert_eq!(
        recovered_a, baseline,
        "Source-Recovery muss Baseline reproduzieren"
    );
    let synced = source2.root_of("SYNC-OK");
    assert_eq!(synced, baseline);
    assert_eq!(
        sink2.root_of("SYNC-OK"),
        baseline,
        "Sink-Recovery + Re-Sync muss konvergieren"
    );
    assert_eq!(load_root(&dir_b), baseline);
    source2.wait_exit_ok();
    sink2.wait_exit_ok();
}

// ── AK 5 (Unit): Verifikation statt Vertrauen — Tamper-Snapshot ────────

#[test]
fn snapshot_verification_rejects_tampered_entries() {
    use kai_os_state::state::{StateStore, Tx};
    use kai_os_syncd::{snapshot_from_store, verify_snapshot, SyncError};

    let mut store = StateStore::new();
    store.apply(&Tx::Set {
        key: "a".into(),
        value: "1".into(),
    });
    let snap = snapshot_from_store(&store, 1, "tip");
    assert!(verify_snapshot(&snap).is_ok());

    // Manipulation: Root deklariert einen ANDEREN Zustand als die Einträge
    let mut tampered = snap.clone();
    tampered.entries.push(("b".into(), "boese".into()));
    match verify_snapshot(&tampered) {
        Err(SyncError::RootMismatch { .. }) => {} // abgewiesen, wie gefordert
        Err(_) => panic!("erwartet RootMismatch, anderer Fehler"),
        Ok(_) => panic!("Tamper-Snapshot darf NIE verifizieren (Verifikation statt Vertrauen)"),
    }
}
