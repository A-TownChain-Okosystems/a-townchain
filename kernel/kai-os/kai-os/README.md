# KAI-OS Core Runtime — Node

> **Terminologie (ATC-ARCH-001):** KAI-OS ist der Architekturbegriff für den
> Gesamtstack — kein Betriebssystem. Dieses Verzeichnis enthält die Core-Runtime-
> Implementierung der KAI-OS-Architektur im Integrations-Repo (a-townchain-os, L7).
> Das eigentliche Betriebssystem ist GlobusOS.

> Implementierungsblock S01–S03 · [Issue #102](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/102)
> Spezifikation: [KAI-CORE-RUNTIME-001](../docs/architecture/KAI-CORE-RUNTIME-001.md) · Sprachgrenze: [AD-008](../docs/architecture/AD-008.md) (native Runtime = Rust)

## Struktur

```
kai-os/
├── node/               # kai-os-node Daemon (Orchestrierungsebene, S01-S03)
├── runtime/            # kai-os-runtime: Sandbox + Resources + Scheduler (S04-S06)
    ├── capability.rs   # Capability Policy — deny by default
    ├── resources.rs    # Resource Manager — Quota-Enforcement pro Sandbox
    ├── sandbox.rs      # Policy -> Resource -> Execution (fail-closed)
    └── scheduler.rs    # Deterministischer Round-Robin
├── network/            # kai-os-network: P2P Transport + Discovery + Peer Security (S07-S09)
    ├── peer.rs         # PeerId (Ed25519-Pubkey), PeerStore (Dedup, Stale-Eviction)
    ├── discovery.rs    # ANNOUNCE / PING-PONG / PEER_LIST
    ├── auth.rs         # Challenge-Response-Handshake + Nonce-Registry (Replay-Schutz)
    ├── session.rs      # SecureSession: Seq-Nummern + signierte Envelopes
    └── transport.rs    # Length-Prefix-Framing (fail-closed)
├── state/              # kai-os-state: Sync + Snapshots + Merkle Verification (S10-S12)
    ├── merkle.rs       # Merkle-Tree (sortierte Entries), Inclusion-Proofs
    ├── state.rs        # StateStore (BTreeMap), Tx-Modell (Set/Remove)
    ├── snapshot.rs     # Snapshot mit Height/Tip/Root, Verifikation gegen vertrauten Root
    └── sync.rs         # SyncEngine: strikte Höhen, Prev-Kette, Root-Verifikation, Atomicität
├── ai/                 # kai-os-ai: AI Runtime + IPC + Audit Trail (S13-S15)
    ├── ipc.rs          # IPC-Gateway: Identity/Capability/Schema/Replay/Audit — kein MQ
    ├── proposal.rs     # Proposal-Registry: Proposed → Specified → HandedToVM (keine Ausführung)
    └── audit.rs        # Audit-Pipeline mit SHA-256-Hash-Chain + Queries
├── keyring/            # kai-os-keyring: Ed25519-Key-Isolation (S16-S18)
    └── keyring.rs      # Keys verlassen die Boundary NIE: nur Public + Signaturen, Revoke/Rotation
├── health/             # kai-os-health: Watchdog + Health-Gossip (S19-S20)
    ├── watchdog.rs     # Tick-basierte Liveness: Running→Stale→DeclaredDead, Restart-Zähler
    └── gossip.rs       # Deterministisches Merge (höherer Tick gewinnt, Tie→schlimmster Status), Snapshots
├── cli/                # kai-os-cli: Kompositions-CLI (S21-S22)
│   ├── commands.rs     # status/keys/state/audit — testbare Command-Funktionen, fail-closed dispatch
│   └── main.rs         # Demo-Binary
├── pkg/                # kai-os-pkg: Deterministischer Package Manager (S23-S24)
    ├── version.rs      # SemVer parse/compare, Exact/Caret-Reqs
    ├── manifest.rs     # PackageManifest mit Dependencies
    ├── registry.rs     # Publish (Immutabilität), best_match (höchste passende), DFS-Resolution mit Zyklenerkennung
    └── lockfile.rs     # Sortiert, SHA-256-Integrität, byte-deterministisch, verifizierbar
└── (in kai-os-ai, S21-S22 nachgeholt:)
    ├── model_registry.rs  # Deterministische Modell-Registry: Publish-Immutabilität, höchste passende Version
    └── verified_cache.rs # SHA-256-verifizierter Cache: Prüfung beim Einstellen UND bei jedem Laden, FIFO-Eviction
├── integration/        # kai-os-integration: Boot-Pipeline über ALLE Crates (S25-S26)
    └── lib.rs          # 9 Schritte fest: Lifecycle→Keyring→State→Snapshot→Sandbox→IPC→Proposal→Signatur→Health
    ├── src/
    │   ├── main.rs     # Boot-Sequenz, Signal-Handling, Tick-Loop
    │   ├── lifecycle.rs    # State-Machine: INIT→BOOT→RUNNING→DEGRADED→SHUTDOWN→STOPPED
    │   ├── supervisor.rs   # Subsystem-Überwachung, Crash-Restart-Policy, State-Persistenz
    │   ├── health.rs       # Worst-of Health-Aggregation
    │   ├── config.rs      # Fail-Closed-Validierung (kein Boot bei ungültiger Config)
    │   ├── shutdown.rs     # Graceful Drain (umgekehrte Start-Reihenfolge, Report)
    │   └── audit.rs        # Append-only Audit-Log mit SHA-256-Hash-Chain
    └── tests/
        └── node_integration.rs  # 14 Tests — alle Akzeptanzkriterien aus Issue #102
```

## Boot-Sequenz (fail-closed)

```
Config validieren (Exit 2 bei Fehler)
  → Root-Guard: euid=0 verweigert Boot (außer allow_root=true)
  → Audit-Logger initialisieren (Hash-Chain)
  → Lifecycle: INIT → BOOT
  → Supervisor.start_all (fail-closed: Abbruch + Rollback bei Start-Fehler)
  → Lifecycle: BOOT → RUNNING
  → Signal-Handler: SIGTERM/SIGINT → geordnetes Herunterfahren
  → Tick-Loop: Subsystem-Health → DEGRADED ↔ RUNNING
  → Shutdown: Drain in umgekehrter Start-Reihenfolge → STOPPED (Exit 0)
```

## Ausführen

```bash
cd kai-os
cargo test                          # 14 Integrationstests
cargo run -- /pfad/zur/config.json  # Daemon starten
kill -TERM <pid>                    # Graceful Shutdown (Exit 0)
```

## Runtime (S04–S06, Issue #103)

Ausführungskette **Policy → Sandbox → Capability → Execution**:
- Capability-Builder: CPU/Memory/Storage-Quotas, Port-basierte Network-Policy (inbound/outbound getrennt), Syscall-Allowlist, Crypto-Capability — alles **deny by default**
- Resource-Manager: Verbrauchs-Tracking, exakt-am-Limit ok, Überschreitung → Fehler; Memory mit alloc/free (kein Negativsaldo), Storage append-only
- Sandbox: Operationen nur nach Policy-Gate + Ressourcen-Buchung; Isolationstest: erschöpft Sandbox A ihr Budget, arbeitet B unbeeinflusst weiter
- Scheduler: Round-Robin, identische Registration → identische Turn-Sequenz (deterministisch, REQ-ENG-002)

## Network (S07–S09, Issue #104)

Kette: `Peer Discovery → Secure Transport → Peer Authentication`
- PeerId deterministisch aus Ed25519-Pubkey (Keypairs aus Seeds, kein RNG — REQ-ENG-002)
- Challenge-Response-Handshake, Nonces strikt monoton (Replay → Fehler), Session-ID via SHA-256
- SecureSession: Sequenznummern strikt steigend, signierte Envelopes — Payload-Manipulation bricht die Verifikation
- Length-Prefix-Framing: trunzierte/übergroße Frames fail-closed abgewiesen
- Discovery: Announce-Dedup (Upsert über PeerId), Ping→Pong, PeerList-Merge, deterministisch sortierte Peer-Listen, Stale-Eviction

## State (S10–S12, Issue #105)

Kernprinzip: **Ein Snapshot wird NIEMALS vertraut, sondern gegen einen kryptographisch definierten State-Root verifiziert.**
- Merkle-Root deterministisch (Einfüge-Order egal), Inclusion-Proofs mit Manipulations-Detection
- SyncEngine akzeptiert einen Block NUR bei: Höhe = Tip+1 (keine Lücken), Prev-Hash = Tip-Hash (Kontinuität), recomputeter Root == claimed Root (Verifikation statt Vertrauen)
- Atomic: ungültige Blöcke verändern den State nicht (Scratch-Apply → Verifikation → Commit)
- Fork-Detection vor Höhen-Check; Duplikate abgewiesen; deterministisches Replay-Test (gleiche Blöcke → identischer Root)

## AI Runtime (S13–S15, Issue #106)

Kernregel (AD-008 §7): **AI may propose. ATCLang specifies. ATVM executes. ATC commits.**
- IPC-Gateway: Reihenfolge verbindlich Identity → Capability → Replay → Schema → Audit; deny by default; kein Message-Queue, sondern eine standardisierte Schnittstelle
- Proposal-Registry: KI-Vorschläge sind **Daten, keine Ausführungen** — deterministische IDs (Idempotenz), strikter Lifecycle Proposed → Specified → HandedToVM, danach terminal (Verantwortung bei ATVM/Consensus-Stack)
- Architektur-Invariante: die Crate besitzt **keine API, die Chain-State mutiert**
- Audit-Pipeline: jede Zustellung UND jede Abweisung erzeugt einen Event; Hash-Chain verifizierbar

## Keyring (S16–S18, Issue #107)

Kernregel: **Keys verlassen die Keyring-Boundary NIE.**
- Keine API gibt privates Schlüsselmaterial heraus — nur Public Keys (32 B) und Signaturen (64 B)
- Import ausschließlich über Seed-Bytes (deterministisch, kein RNG — REQ-ENG-002)
- Revoke: Key raus, Secret wird beim Drop gelöscht (ed25519-dalek Zeroize-on-Drop); danach fail-closed
- Rotation: neues Secret, neuer Public Key — alte Signaturen bleiben mit dem extern aufbewahrten alten Public Key prüfbar
- Kompatibilitätstest: identischer Seed → identischer Public Key wie kai-os-network (eine Quelle der Wahrheit)

## Health (S19–S20, Issue #108)

Kernprinzip: **Liveness wird NICHT angenommen, sondern beobachtet.**
- Watchdog: Tick-basiert (keine Wall-Clock, REQ-ENG-002) — Running → Stale nach Toleranzüberschreitung, → DeclaredDead nach Limit; vergangene Zeit verschlechtert nur, heilt nie
- Recovery NUR durch echten Herzschlag; DeclaredDead-Services heilen nie selbst — Restart (mit Zähler, Supervisor-Kopplung) ist Pflicht
- Health-Gossip: pro Beobachter geführte Maps, deterministisches Merge (höherer Tick gewinnt); Konsolidierung fail-closed konservativ — bei Tick-Gleichstand gewinnt der schlimmste Status
- Deterministisch sortierte Snapshots, transportfähig über die P2P-Discovery (Issue #104)

## CLI & Package Manager (S21–S24, Issue #110)

Kernprinzip: **Werkzeuge sind Komposition, keine neue Logik.**
- CLI: Kommandos als reine Funktionen über bestehende Crates (health, keyring, state, ai) — deterministische Ausgaben, kein Wall-Clock; unbekannte Kommandos fail-closed
- Pkg: Versionswahl deterministisch — höchste PASSENDE Version (Caret respektiert Major-Grenze), unabhängig von Publish-Reihenfolge; Publish-Immutabilität (gleiche Version + anderer Content abgewiesen, identischer Content idempotent)
- DFS-Resolution: verankert Pakete erst NACH vollständiger Rekursion (Zyklen werden sicher erkannt), Versionen per best_match gepinnt; fehlende Dependencies fail-closed
- Lockfile: sortiert nach Name, byte-deterministisch, SHA-256-Integrität gegen Registry verifizierbar

## Model Registry + Verified Cache (S21–S22 nachgeholt, G2-A aus #112)

Die nachgeholte Spezifikations-Lücke (ROADMAP-Abgleich 12.09.): keine synthetischen Pässe, echte Invarianten.
- **Model Registry:** Publish-Immutabilität (gleiche Version + anderer SHA-256 abgewiesen, identisch idempotent); Versionswahl deterministisch — höchste *passende* Version unabhängig von Publish-Reihenfolge, Caret respektiert Major-Grenze; unbekannte Modelle/Requirements fail-closed
- **Verified Cache:** kein ungeprüftes Laden — SHA-256-Prüfung beim Einstellen **und bei jedem einzelnen Ladevorgang** (Korruption im Lager wird erkannt, ungeprüfte Bytes werden nie zurückgegeben); FIFO-Eviction deterministisch
- Integritätskette: das Registry-Manifest ist die Verifikationsgrundlage — nur artefakte mit passendem Hash kommen in den Cache

## Echter TCP-Transport (G2-C aus #112)

`network/tcp.rs`: Socket-Schicht über dem bestehenden Length-Prefix-Framing (fail-closed).
- Frame-Disziplin: eine Nachricht = ein Frame; unvollständige Frames bleiben gepuffert; EOF mitten im Frame → `ClosedMidFrame`, keine Teilzustände
- Backpressure über die Grenze: `MAX_FRAME` (4 MiB) — Oversized-Deklaration → Verbindung fail-closed beendet (kein OOM)
- Getestet über echte Loopback-Sockets: Roundtrip, Frame-Grenzen bei Mehrfachsendung, **kompletter Ed25519-Challenge-Response-Handshake über die Leitung** (S07-S09-Sicherheitsschicht + echtes TCP), Connection-Churn mit Reconnect

## Multi-Node Disaster Recovery (Issue #113 — GATE-KAI-001 Kat. 12)

`syncd`-Crate: State-Sync zwischen **echten OS-Prozessen** über TCP + spawnbares Node-Binary (`kai-sync-node`).
- **Verifikation statt Vertrauen:** der Sink baut den Snapshot neu auf, berechnet den Root und weicht ab → Abweisung (RootMismatch), kein Teilzustand
- Persistenz auf der Sink über das Marker-Protokoll (G2-D); Ack mit Root-Gegenprüfung auf der Source
- DR-Szenarien mit echten Kills getestet: Source-Absturz nach Checkpoint → Recovery aus eigenem WAL → Resumption ohne Tx-Verlust; Partition (toter Peer) → geordnetes Scheitern mit Retry → Resumption; beidseitiger Tod → unabhängiges Recovery → Konvergenz auf den Baseline-Root
- Deterministische Txs (feste Keys/Values, REQ-ENG-002); Orchestrierung über geflusste Protokoll-Zeilen statt Zeitmessung

## Upgrade/Rollback-Design (G2-D aus #112)

**Rollback = Snapshot-Return, nie semantische Umkehr.** Angewendete Konsens-Effekte werden nicht "rückgängig gemacht" — das System kehrt auf die letzte verifizierte Zustandsgrenze zurück (`last_verified_boundary`: Höhe, Tip-Hash, State-Root).
- **Atomares Checkpointing (schließt die G2-B-Grenze):** WAL strikt append-only — kein Truncate mehr. Checkpoint = Marker-Record in der Kette, DANACH Snapshot-Sync. Absturz vor dem Sync → Genesis-Replay (korrekt); danach → Snapshot-Pfad. Es gibt kein korruptes Zwischenreich mehr (Test: Marker ohne Snapshot → identischer Root).
- **Protokoll-Verhandlung** (`network/upgrade.rs`): deterministische Version-Auswahl min(current_a, current_b) ≥ max(min_a, min_b) — kommutativ. Inkompatible Peers werden abgewiesen und erhalten den State-Root der letzten verifizierten Grenze des Gegenübers als **Resync-Hinweis** (Snapshot-Return statt Best-Effort).
- **Versionierte Envelopes:** Versions-Check BEVOR der Payload interpretiert wird — unbekannte/zukünftige Versionen fail-closed, kein Blind-Parsing.

## Persistente Storage-Layer (G2-B aus #112)

WAL-Vertrag (Crash-Sicherheit): **WAL vor State** — jede Tx wird zuerst durable in den Log geschrieben (`sync_data`), erst danach angewendet. Ein Absturz verliert nie eine angewendete Änderung.
- Hash-Kette über alle Records — Corruption mitten im Log wird beim Replay fail-closed erkannt (`ChainBroken`)
- Torn Tail: unvollständiger LETZTER Record (Absturz während des Schreibens) wird verworfen — Standard-WAL-Vertrag
- `PersistentState`: open/apply/checkpoint — Recovery = Snapshot laden + WAL-Replay, im Test mit echtem Dateisystem verifiziert (Drop → Reopen → identischer Root)
- Snapshot-Persistenz mit Write-Verify (nach dem Speichern zurücklesen und Root vergleichen) und Load-Verify (Tamper-Erkennung beim Laden)
- Bekannte dokumentierte Grenze: Absturz GENAU zwischen Snapshot-Sync und WAL-Truncate im Checkpoint → Doppel-Replay (G2-D verfeinert)

## Integration (S25–S26, Issue #111) — Track-Finale

Kernprinzip: **Ein System bootet deterministisch oder gar nicht.**
- Boot-Pipeline: 9 Schritte in FESTER Reihenfolge, jeder mit Invarianten-Prüfung; ein Fehler → `BootError` mit benanntem Schritt (keine Teilzustände)
- End-to-End getestet: Lifecycle RUNNING, State-Root == Snapshot-Root, Proposal `HandedToVM` (KI führt nie aus), node-key-signierte Übergabe verifizierbar, Services Running, Audit-Ketten intakt
- Replay-Determinismus: identischer Boot → identische Roots/IDs/Signaturen/Reports
- Crash-Recovery (in-process): System verwerfen → Tx-Log-Replay → identischer State-Root; fremder Tx-Log → anderer Root (keine Fake-Recovery)

## Subsysteme (S01: Stubs)

`consensus` · `p2p` · `state-sync` · `ai-runtime` · `storage` · `security`
Echte Implementierungen entstehen in S04–S12 (Sandbox, P2P Transport, State Sync).
Der Supervisor implementiert Consensus-**Infrastruktur** (Orchestrierung),
niemals Consensus-**Semantik** (AD-008.4).

## Determinismus

Kein RNG, keine Wall-Clock-Timestamps im Audit-Hash (REQ-ENG-002).
Restart-Backoffs sind fixe Konfigurationswerte. Tests laufen ohne Netz- und Zeitabhängigkeit.
