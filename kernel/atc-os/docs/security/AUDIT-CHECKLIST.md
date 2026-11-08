# GATE-KAI-001 — Externes Security-Audit: Prüfcheckliste (G2-E)

**Stand:** 12.09.2026 · **Scope:** kai-os/ (10 Crates) + Übergänge zu ATVM/ATCLang (AD-008)
**Auditer-Entscheidung:** Owner (Michael Wroblewski) — Ausführung nach dieser Checkliste
**Status des Tracks:** S01–S26 implementiert · GATE-Review 12.09. (Issue #109): 5 PASS / 4 PARTIAL / 3 OPEN → G2-A bis G2-D geschlossen (Commits 6e88d27, 87fd225, 269228d, 7cc4edc)

## Prüfgebiete mit Evidenz-Verweisen

| # | Kategorie | Was zu prüfen ist | Evidenz |
|---|---|---|---|
| 1 | **Lifecycle & Supervisor** | Fail-Closed-Boot, Shutdown-Ordnung, keine Zombie-Services | `kai-os/node/src/*`, 14 Tests, GATE-Review #109 |
| 2 | **Audit-Trail** | SHA-256-Hash-Kette, Unveränderlichkeit, Lücken-Erkennung | `kai-os/node/src/audit.rs` |
| 3 | **Capability Sandbox** | Deny-by-default, Quota-Enforcement, Isolation zwischen Modulen | `kai-os/runtime/src/*`, 10 Tests |
| 4 | **Deterministische Ausführung** | Kein Wall-Clock/RNG im Konsens-Pfad, Quellcode-Scan | `tools/determinism_check.py` (12× in CI), REQ-ENG-002 |
| 5 | **P2P Authentifizierung** | Ed25519-Challenge-Response, Replay-Schutz, Stale-Peer-Eviction | `kai-os/network/src/auth.rs`, `session.rs` — über echte TCP bewiesen (`tcp_transport.rs`) |
| 6 | **Frame-Disziplin** | Oversized fail-closed, EOF-mid-Frame, kein OOM über Deklaration | `network/src/transport.rs`, `tcp.rs` |
| 7 | **Protokoll-Upgrade** | Deterministische Verhandlung, kein Blind-Parsing, Resync-Hint | `network/src/upgrade.rs`, 5 Tests |
| 8 | **Crash-Recovery (WAL)** | WAL-vor-State, Hash-Kette, Torn-Tail, Marker-Protokoll ohne Absturzfenster | `state/src/wal.rs`, `persistence.rs`, 9 Tests (echtes Dateisystem) |
| 9 | **Modell-Integrität** | Publish-Immutabilität, SHA-256 bei jedem Laden, FIFO-Eviction | `ai/src/model_registry.rs`, `verified_cache.rs`, 7 Tests |
| 10 | **Keyring-Isolation** | Schlüssel verlassen die Boundary nie, Rotation | `kai-os/keyring/src/*`, 7 Tests |
| 11 | **IPC & Proposal-Grenze** | AI may propose — keine State-Mutation-API in der AI-Schicht (AD-008 §7) | `ai/src/ipc.rs`, `proposal.rs`, `audit.rs` |
| 12 | **Boot-Pipeline & Multi-Node-DR** | 9 Boot-Schritte deterministisch; Multi-Node-DR mit echten Prozessen (2 Nodes, TCP, Kill/Restart, Partition) | `kai-os/integration/`, `kai-os/syncd/tests/multi_node_dr.rs` (5 Tests) |

## Explizite Prüfaufträge (aus Review #109 und Commits)

1. **Checkpoint-Fenster:** G2-B dokumentierte eine Grenze (Crash zwischen Snapshot-Sync und WAL-Truncate) — G2-D schließt sie per Marker-Protokoll. **Auditer prüft:** Beweis `checkpoint_crash_before_snapshot_sync_still_recovers` (state) und dass kein weiterer Pfad existiert, der in ein Doppel-Replay mündet.
2. **Tamper-Pfade:** VerifiedCache prüft Hash bei JEDEM Ladevorgang — **Auditer prüft:** ob es einen Pfad gibt, der Bytes ohne `get()` (also ohne Verifikation) herausgibt.
3. **TCP-Oberfläche:** Listener sind testweise Loopback — **Auditer prüft:** Produktions-Deployment (Bind-Address, TLS-Layer, DoS-Resistenz bei Frame-Flut).
4. **0 `unwrap()` in `src/`** (Owner-Regel) — **Auditer verifiziert:** `grep -rn "unwrap()" --include="*.rs" */src/` (Erwartung: 0 Fundstellen außerhalb `unwrap_or*`).

## Verifikation vor Ort (Auditer)

```bash
cd kai-os && cargo test            # Erwartung: 111/111 GRÜN
grep -rn "unwrap()" --include="*.rs" */src/ | grep -v unwrap_or   # Erwartung: leer
cd .. && python3 tools/determinism_check.py --lang rust           # Erwartung: PASS
```

## Release-Gate

Produktionsfreigabe erst nach: (1) externem Audit ohne CRITICAL/HIGH-Funde, (2) 106/106 Tests grün, (3) 26/26 Compliance (Code-Quality-Matrix nach Owner-Gate-Push, Issue atc-standards#13).
