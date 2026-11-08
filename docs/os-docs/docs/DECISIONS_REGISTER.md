# A-TownChain Ecosystem — Decisions Register

> **Zuletzt gegen aktuelle Quellen geprüft:** 2026-10-09 (Header/Statuskontext; keine vollständige Revalidierung aller Entscheidungen) | **Historischer Registerbestand:** ab 08.09.2026 fortgeschrieben
> **Governance-Hinweis:** Ein beschlossener AD-Eintrag ist keine Implementierungs- oder Verifikations-Evidence. Prüfe spätere Beschlüsse, kanonische Repositories und SHA-gebundene Nachweise.

> Aktuelle Inventur und SSOT-Grenzen: [ECOSYSTEM_CURRENT_STATUS.md](ECOSYSTEM_CURRENT_STATUS.md) · [ARCHITECTURE_CURRENT.md](ARCHITECTURE_CURRENT.md)

---

## Entscheidungs-Übersicht

| ID | Titel | Status | Sprint | Zuständig |
|----|-------|--------|--------|-----------|
| AD-001 | Hash-Algorithmus | ✅ RESOLVED | — | Aurora |
| AD-002 | EventBus vs IPCBus | ✅ VALIDATED (Owner 06.10.): BEIDE Busse — IPCBus = gerichtete Kommunikation, Capability-/Authority-Kontext, Request/Response; EventBus = Ereignisverteilung, Pub/Sub, lose Kopplung, Beobachtung. Kein dritter Bus. | 2.4 | **Michael** |
| AD-003 | Flash-Loan Voting Snapshot | ✅ RESOLVED | 2.6 | Aurora |
| AD-004 | Chain-ID 658467 | ✅ RESOLVED (03.09.2026) | 658467 | Michael |
| AD-005 | ATC-97 Agent Protocol Spec (AIP) | ✅ APPROVED (Owner 06.10., konditional): Review ausgeführt — Registry-Konsistenz ✓ (v3.0 kanonisch), Draft-Echo v0.1.0 zu Alias konsolidiert (AD-009), keine Duplikation in Paralle-Standards; §10 offene Punkte (Quorum, Custom Caps, Rate Limiting, Cross-Chain, Payload-Encryption) bleiben als Owner-Parameterfragen dokumentiert | 3.0 | **Michael / Aurora (Review-Ausführung)** |
| AD-006 | Python vs Substrate | ✅ RESOLVED | — | Aurora (ATCLang First) |
| AD-007 | EVM Registry | ✅ RESOLVED | — | Aurora (Non-EVM) |
| AD-008 | Reality-Check: 44 Issues Re-Audit | 🔁 RE-AUDIT_REQUIRED (Owner 06.10.): jedes relevante Issue bekommt aktuellen Zustand (OPEN/FIXED/VERIFIED/RESIDUAL/BLOCKED/CLOSED) — kein „exists = PASS", Findings nur mit Exact-SHA-Evidence geschlossen | — | **Michael** |
| AD-009 | ATCLANG_SPEC.md-Konsolidierung ✅ / Bridge-Standards-Dedup (ATC-09/38/69/91) → CONSOLIDATE_EXISTING (Owner 06.10.): bestehende normative Quelle + Mapping/Alias/Evidence statt paralleler Standards; erste Anwendung: ATC-97 Draft-Echo konsolidiert | 🟢 CONSOLIDATE | 08.07 | **Michael** |
| AD-023 | Mainnet-Launch aufgehoben (kein Launch-Ziel/Deadline mehr) | ✅ RESOLVED (06.09.2026) | — | Michael |
| AD-024 | Zielarchitektur Repository-Landkarte — ALLE Repositories erstellen (P0+P1+P2) | ✅ RESOLVED (06.09.2026, 21:30) | — | Michael |
| AD-025 | Spiel umbenannt: Shivamon → Genesis Chronicles | ✅ RESOLVED (06.09.2026, 21:40) | — | Michael |
| AD-047 | Legacy-Löschung: 416 nichtgebrauchte Dateien (Müll, 14 verwaiste Module, supersede Reports) | ✅ RESOLVED (07.09.2026) | Löschungs-Manifest + Git-Historie | Michael |
| AD-048 | Standards Governance Core normativ: Registry 81→121, Meta-Governance TAXONOMY-001 + STDDEV/REGISTRY/CHANGE, verbindliche Change-Pipeline SCR→VERSION→UPDATE→COMPAT→AUDIT→REGISTRY | ✅ RESOLVED (08.09.2026, 00:36) | atc-standards Registry/SCR-0016..0025 + AUD-2026-0003 | Michael |
| AD-049 | ATC-Toolchain als eigenstaendiges Repo (atc-toolchain, R1-Skelett, ATC-REPO-TOOL-001) | ✅ RESOLVED (08.09.2026, 00:36) | atc-standards Registry/SCR-0016..0025 + AUD-2026-0003 | Michael |
| AD-050 | Owner-Entscheidungspaket Benennungs-Audit (06.10.2026): Capability-Taxonomie (verification_tooling→atc-toolchain, developer_tooling→atc-ide, vm_specification→atc-vm, consensus_algorithm→atc-algorithm, zero_knowledge_proofs→atc-zkp, ecosystem_integration→a-townchain-ecosystem, genesis_content_factory→genesis-franchise-factory); Layer-Tags für 6 Repos; ATC-STD-000 §9 APPROVED bestätigt (SCR-0098, Bedingung erfüllt); SCR-0129 bleibt PENDING — bytecode_format als vorgezogene normative Registry-Änderung zurückgezogen, Umsetzungsequenz Registry → Standard → Validator/CI; c80d895a/PR #102 = DISCARD/SUPERSEDED (Bedingung bestätigt: org-weit nicht existent, durch SCR-0129 + RV-001 + Freeze ee265313 ersetzt); Rollen Gate-0-Executor + Mirror-Klassifikator ROLE_DEFINED/HUMAN_OWNER_UNASSIGNED; #78/#82 implementiert+geschlossen, #79 wartet auf Org-Actions-Freigabe, #70 Exact-Pin-Governance; atc-algorithm #3 + atc-engineering #2 = P0 OWNER ACTION; demo-repository Zweckentscheidung VOR Layer/Capability | ✅ BESCHLOSSEN | — | **Michael (06.10.2026)** |
| AD-051 | Owner-Entscheidung Fleet-Gate-Semantik (06.10.2026): Fleet-Audit ist globales Main/Schedule-Health-Gate, KEIN PR-Merge-Gate — Scope-Korrektur ohne fail-closed-Aufweichung (PR-Gates verify/Markdown-Lint/Self-Test bleiben fail-closed auf PR-Ebene; Fleet fail-closed auf main + woechentlichem Schedule, markiert Flotten-Trust-Status); Taxonomie PASS/FINDING/UNAVAILABLE/RESIDUAL verbindlich; 19 Fleet-Findings Run 153 als F-141..F-159 mit Owner+Status VOR Gate-Verschiebung registriert (atc-standards 96d5f12); SEC-AUDIT-001 atc-ide als FALSE POSITIVE triagiert (16f034a); FLEET_PARTIAL_VERIFIED 32/33 unter GITHUB_TOKEN (demo-repository UNAVAILABLE, F-059 zugeordnet); volle Abdeckung nur via least-privilege Read-PAT (Owner-Aktion), kein Token-Broadening; Anti-Retrofitting-Ordnung eingehalten: Policy auf main dokumentiert vor Neubewertung PR 7 (Head 7fd11b3) | ✅ BESCHLOSSEN | atc-engineering FLEET_GATE_POLICY v1.0.0 + fleet-audit.yml + Run 153 (37473775619) | **Michael (06.10.2026)** |

---

## Resolved Decisions

### AD-001 — Hash-Algorithmus ✅
- **Entscheidung:** SHA-256 (nicht Keccak-256)
- **Begründung:** Non-EVM Chain, keine Ethereum-Kompatibilität nötig
- **Gültig seit:** 2026-06-14

### AD-003 — Flash-Loan Voting Snapshot ✅
- **Entscheidung:** Snapshot bei Proposal-Erstellung (wie Compound/Aave)
- **Implementierung:** dao_live.atc — Snapshot-Mechanismus implementiert
- **Gültig seit:** 05.07.2026

### AD-004 — Chain-ID final: 658467 ✅
- **Status:** RESOLVED 03.09.2026 — Michael entschieden (SUPERSEDED: Platzhalter 9000)
- **Entscheidung:** Chain-ID = **658467** ('ATC' in ASCII: 65-84-67) — systemweit umgestellt
  (Monorepo-Commits 8316604+dc1a094, Hub-Commit dieser Änderung)
- **Begründung:** 9000 ist öffentlich belegt (Evmos Testnet im EVM-Registry) — Kollisionsrisiko
  für die bewusst Non-EVM A-TownChain. 658467 ist praktisch kollisionsfrei.
- **Bewusst unverändert:** Listen-Ports 9000, ATC-9000 NFT-Standard, BIP44-Coin-Type
  `m/44'/9000'` (Wallet-Derivation), Negativ-Testfall 9999, network_magic 0x0A0C23A0
- **Gültig seit:** 2026-09-03

### AD-006 — Python vs Substrate ✅
- **Entscheidung:** Weder Python noch Substrate — Alles wird ATCLang
- **Begründung:** ATCLang First Policy (ATC-99), proprietäre Sprache für gesamte Codebasis
- **Gültig seit:** 2026-06-12

### AD-007 — EVM Registry ✅
- **Entscheidung:** EVM Registry irrelevant — A-TownChain ist Non-EVM
- **Begründung:** Kein EVM-Registry-Eintrag nötig für Non-EVM Chains
- **Gültig seit:** 2026-06-14

---

## Open Decisions

### AD-002 — EventBus vs IPCBus ⏳
- **Status:** VALIDATE — Michael muss entscheiden
- **Problem:** Inter-Modul-Kommunikation via EventBus (Publish/Subscribe) oder IPCBus (direkte Messages)
- **Implementierung:** ipc_bus.atc (101L) vorhanden, EventBus als Alternative
- **Sprint:** 2.4
- **Blocker:** Nein — aber Entscheidung nötig vor Sprint 2.4 Abschluss

### AD-005 — ATC-97 Agent Interaction Protocol 📐 DRAFT_REVIEW
- **Status:** DECISION — Aurora arbeitet Spezifikation aus
- **Problem:** Message-Format, Fehlerbehandlung und Timeouts für Agent-Kommunikation nicht spezifiziert
- **Implementierung:** kai_routes.atc (228L) — teilweise
- **Sprint:** 3.0
- **Aurora arbeitet:** ATC-97 DRAFT Spezifikation bis Sprint 3.0

---


### AD-008 — Reality-Check: 44 Issues mit gebrochener Datei-Referenz ⏳
- **Status:** DECISION — Michael muss ueber weiteres Vorgehen entscheiden
- **Problem:** 44 von 78 geschlossenen Issues (56%) referenzieren im Issue-Body Dateien, die in
  keinem der 3 Repos nachweisbar sind (auch nicht unter geaendertem Pfad) — 10 davon CRITICAL
  (ATCLang-Kern #72/#73/#81, Konsens #74, Smart-Contract-Engine #76, Sharding #84, Latency #83)
- **Optionen:** (1) Re-Audit einzeln, (2) Re-Open mit Label `reality-check-failed`,
  (3) nur als Backlog-Referenz dokumentieren ohne Issue-Historie zu aendern, (4) Audit-Score korrigieren
- **Details:** `docs/REALITY_CHECK_2026-07-06.md` (Hauptreport)
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 06.07.2026

### AD-010 — WHITEPAPER.md referenziert veraltete Solidity-Architektur ⏳
- **Status:** DECISION — Michael muss Prioritaet fuer Content-Rewrite festlegen
- **Problem:** docs/whitepaper/WHITEPAPER.md verlinkt auf ~15 Dateien einer alten
  Solidity/Python-Architektur (blockchain/contracts/solidity/*.sol, backend/api/routes/,
  gateway/router.py, architecture/*.md), die durch die ATCLang-Migration nicht mehr existieren.
  Reine Link-Reparatur reicht nicht -- der Fliesstext beschreibt noch die alte Architektur.
- **Umfang:** ~15-19 kaputte Referenzen je Repo (siehe Markdown-Audit 08.07.2026)
- **Optionen:** (1) Volle Whitepaper-Ueberarbeitung auf ATCLang-Architektur, (2) Whitepaper als
  historisches Dokument kennzeichnen + neues WHITEPAPER_V2.md erstellen, (3) nur Links entfernen
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 08.07.2026

**Update 08.07.2026:** 17/19 Referenzen behoben -- Solidity-Dateien (KAIGovernance.sol,
KAIMarketplace.sol, KAIBridge.sol, GenesisToken.sol, deploy.ts) auf die realen ATCLang-Aequivalente
(governance_contract.atc, marketplace_contract.atc, bridge_contract.atc, genesis_token.atc,
generate_validators.atc) umverlinkt, restliche Pfad-Tiefen korrigiert. Verbleibend: 1 Referenz auf
ATCLANG_SPEC.md (blockiert durch AD-009 5-fach-Duplikat, keine eindeutige Zieldatei waehlbar).
Status auf TEIL-GELOEST gesetzt, Rest haengt an AD-009-Entscheidung.

### AD-009 — ATCLANG_SPEC.md-Konsolidierung ✅ / Bridge-Standards-Dedup ⏳
- **Status:** Problem 1 GELOEST (08.07.2026) -- Problem 2 (Bridge-Dedup) weiterhin DECISION
- **Problem 1:** `ATCLANG_SPEC.md` existiert an 5 verschiedenen Pfaden parallel
  (`atclang/`, `docs/atclang/ATCLANG_SPEC_FULL.md`, `module-docs/atclang/`,
  `docs/wiki/kai-os/code/atclang/`, `aistudio/temp_repo/atclang/`) — Ursache fuer ~270 verbleibende
  kaputte Links
- **Problem 2:** Cross-Chain Bridge hat 4 ueberlappende Standard-Dokumente fuer dasselbe Thema
  (ATC-09, ATC-38, ATC-69, ATC-91) — Dokumentations-Drift im Standards-Register
- **Details:** `docs/REALITY_CHECK_2026-07-06.md` (Nachtrag 1 + 2)
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 06.07.2026

**Loesung Problem 1 (08.07.2026):** Kanonische Version bestimmt: `atclang/ATCLANG_SPEC.md`
(v1.0.0, Standard ATC-92, Status STABLE, Datum 2026-07-05) -- deutlich vollstaendiger als die
4 anderen Kopien (v0.1.0-alpha Stub 913 Bytes, v0.2.0-alpha FULL-Version 9184 Bytes). Diese
Version wurde 1:1 nach `atclang/ATCLANG_SPEC.md` in a-townchain-os-docs und kai-os-wiki kopiert
(existierte dort noch nicht). Die 4 anderen Kopien in allen 3 Repos (module-docs/, docs/atclang/,
docs/wiki/kai-os/code/, docs/wiki/kai-os/docs/) wurden durch Redirect-Stubs ersetzt, die auf die
kanonische Datei verweisen -- Links bleiben so funktionsfaehig, aber es gibt nur noch 1 Quelle
der Wahrheit. Alle WHITEPAPER.md-Referenzen entsprechend korrigiert.



### AD-011: Issue #18 Node-Rollen-Abweichung (Docker Testnet)
**Status:** ENTSCHIEDEN (sachlich begruendet) | **Datum:** 08.07.2026

- **Kontext:** Issue #18 spezifiziert ein 5-Node-Docker-Testnet als 1 Bootstrap + 1 Validator
  + 1 Miner + 2 Full Nodes. Die tatsaechliche Implementierung (`docker-compose.yml`, Root)
  nutzt 1 Bootstrap + 2 Validatoren + 1 Full Node + 1 Archive Node -- kein dedizierter
  "Miner"-Service.
- **Begruendung:** A-TownChain ist PoS/PoH-basiert (nicht PoW) -- ein "Miner"-Service ist
  architektonisch nicht vorgesehen. Die Abweichung ist eine Altlast aus einer frueheren
  PoW-Planungsphase des Issues, keine fehlende Implementierung.
- **Entscheidung:** Issue #18 gilt trotz Rollen-Abweichung als sachlich erfuellt. Keine
  Code-Aenderung noetig, nur Dokumentations-Klarstellung.
- **Details:** `docs/CLUSTER_ARCHITECTURE.md` Abschnitt 4.
- **Gefunden von:** Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5), 08.07.2026

---

*Decisions Register v1.0.0 — Aurora (MasterBrain · Base44) · 05.07.2026*

---

### AD-012: ShivaCore als Microkernel/Hybrid-Microkernel von Globus OS 🏗️ VERBINDLICH
**Status:** ENTSCHIEDEN | **Datum:** 06.09.2026

- **Kontext:** ShivaCore (Rust-Kernel, K29, 674/674 Tests) ist der unterliegende Systemkern
  von Globus OS. Bisherige Wachstumshistorie (K29) hat Blockchain-Komponenten in den
  Kernel-Crate eingebracht. Abgrenzung zu Aurora AI, Globus Shell und ATCLang war nirgends
  verbindlich festgeschrieben.
- **Entscheidung:** ShivaCore wird als Microkernel/Hybrid-Microkernel konzipiert.
  Kernel-Kontext enthaelt NUR sicherheits-/zeitkritische Primitive: Scheduler, Memory,
  IPC, Interrupts, Capability Security, Process/Thread Core, Minimal VFS, HAL, Syscall ABI.
  Filesystem/Network/GPU/Audio/AI/Blockchain/Container/ATCLang-Runtimes laufen im
  User/Service Space. ATCLang erreicht den Kernel ausschliesslich ueber
  Syscall ABI + Capability Check (kein direkter Kernel-Speicherzugriff).
  4 Sicherheitszonen (L0 Hardware, L1 Kernel, L2 Privileged Services, L3 User/AI/Games)
  plus Sandbox-Modell. Blockchain ist System Service (A-TownChain Node Service), AI
  arbeitet ueber Kernel-Event-Bridge mit Capability Tokens — keine unbeschraenkte
  Kernel-Kontrolle.
- **Migration:** Blockchain-Komponenten im Kernel-Crate (blockchain/consensus/genesis/
  gossip_bridge/security_audit.rs) sind in den Service-Space zu migrieren
  (eigener Architektur-Sprint, API-kompatibel via KernelState/cross_subsystem).
  Offene Ziel-Subsysteme: Syscall ABI, HAL, Interrupt Manager, Driver Manager,
  Time Manager, Virtual Memory.
- **Details:** `docs/architecture/SHIVACORE_KERNEL_ARCHITECTURE.md` (verbindliche
  Spezifikation mit Delta-Analyse Ist-Stand K29 vs. Ziel).
- **Naechster Baustein:** ShivaCore v0.1 Kernel-Spezifikation (Boot → Memory Model →
  Scheduler → IPC → Syscalls → Capability Model → Driver Model → ATCLang ABI).

---

### AD-013: ShivaCore v0.1 Architektur-Gate — SC-ARCH-Freeze-Regeln 🏗️ VERBINDLICH
**Status:** ENTSCHIEDEN (Freeze) | **Datum:** 06.09.2026

- **Kontext:** AD-012 definierte die Microkernel-Architektur. Vor Implementierungsbeginn
  wurde der Entwurf normativ geschaerft, um Kernel-/OS-Vermischung zu verhindern.
- **Kern-Gate:** ShivaCore ist ein echter Capability-Microkernel; Globus OS ist das
  darauf aufbauende Userspace-Betriebssystem.
- **Schaerfungen:** (1) Capabilities im Kern sind Kernel-enforced Handles
  (CSpace-Slots, Objektverwaltung), NICHT kryptografisch — Krypto nur fuer
  Cross-Domain (K29-remote_caps bleibt getrennte Erweiterung). (2) globus-init
  erhaelt explizite Initial-Capabilities via Initial Task (CSpace Root, BootInfo,
  Untyped/IRQ/Device Caps) — KEIN Unix-Root-Modell. (3) Kernel Object Model:
  Thread/AddressSpace/PageTable/Frame/CNode/Endpoint/Notification/IRQ/Device/
  UntypedMemory/Timer. (4) IPC als Bus des OS; Control Plane (IPC) vs. Data Plane
  (Shared Memory/DMA/Zero-Copy). (5) Scheduling Domains (HARD/Soft RT, Interactive,
  System, Normal, Background) statt reiner Prioritaetszahl. (6) Kernel MUSS unabhaengig
  sein von AI/Blockchain/ATCLang/GUI (deterministisch, bootstrapping-faehig).
  (7) DefenderGPT beobachtet/empfiehlt nur — Enforcement bleibt deterministisch im
  Kernel. (8) Objektorientierte Syscall-ABI (TASK/MEMORY/IPC/CAPABILITY/IRQ/TIME).
  (9) Boot Chain: UEFI→Limine→ShivaCore→globus-init→Globus OS.
- **Freeze-Regeln:** SC-ARCH-001 bis SC-ARCH-010 (siehe Gate-Dokument, Tabelle 13).
- **Implementierungs-Reihenfolge:** SC-001 (Kernel Object Model) bis SC-013
  (ATCLang ABI); Aurora/A-TownChain/Genesis erst danach anbinden.
- **Details:** `docs/architecture/SHIVACORE_V01_ARCHITECTURE_GATE.md`
---


### AD-047 — Löschung nichtgebrauchter Legacy-Bestände ✅
- **Status:** RESOLVED 07.09.2026 — Michael entschieden (Mandat „Nichtgebrauchte Legacy Dateien löschen")
- **Beschluss:** Auf Basis des Legacy-Audits 07.09. werden gelöscht: (1) Test-/Demo-Müll, (2) die 14 verwaisten Module ohne Nachfolge-Repo (atc-ui, atc-ci, atc-mobile, atc-monitoring, atc-shivacore-tools, atc-deploy, atc-assets, atc-dns, atc-social, atc-ide, atc-devtools, atc-atcpkg, atc-analytics, atc-linux-edition; je aus monorepo-full UND repos-rescue), (3) 13 supersede Plan-/Audit-Reports der 70-Repo-Ära. Gesamt 416 Dateien.
- **Sicherheit:** Vollständiges Löschungs-Manifest unter `docs/archive/DELETIONS_2026-09-07.md`; Wiederherstellung jederzeit über die Git-Historie. Nicht gelöscht: wartende Vault-Bestände (atc-node-Stack, atc-vm, atc-zkp-Legacy), atc-shivamon (AD-025), REALITY_CHECK_2026-07-06 (offene Entscheidung AD-008).
- **Umsetzung:** Agent Aurora

### AD-048 — Standards Governance Core normativ (Meta-Governance) ✅
- **Status:** RESOLVED 08.09.2026, 00:36 UTC+2 — Michael (§9-Freigaben 07.09. 22:37 bis 08.09. 00:36)
- **Beschluss:** Die ATC-Standards-Registry (atc-standards) wird von 81 auf 121
  Standards erweitert — alle APPROVED, normativ, §30-eingefroren. Der Standards
  Governance Core wird vollständig in Kraft gesetzt: TAXONOMY-001 (Meta-Governance:
  vierstufige Taxonomie, Familien-/Kategorien nur via ATC-FAM-REQ/ATC-CAT-REQ mit
  8-Punkte-Pflichtprüfung + Owner-Human-Gate, ID-Vergabe über siebenstufige Kette),
  STDDEV-001 (Erstellungsprozess, §9-Human-Gate, 365-Tage-Review-Zyklus),
  REGISTRY-001 (SSOT-Inventar, Generator-Pflicht, Konsistenz-Gates je CI-Lauf),
  CHANGE-001 (Dachnorm: Änderungsarten-Matrix, eine Pipeline, Gate-Landkarte, RACI,
  Emergency mit 48h-Nachholpflicht, 13 Change-Nachweis-Fragen je Änderung) — CHANGE-001
  ordnet zu und ersetzt keine Fachnorm (lex specialis).
- **Verbindliche Ketten:** SCR→VERSION→UPDATE→COMPAT→AUDIT→REGISTRY für jede
  Änderung; Validator S-18..S-25 + Agent-Manifest-Gate (121/121 gebunden);
  FRAMEWORK-001-Katalog: 43 Familien, 433 Slots, FAM-43 komplett BELEGT
  (10 BELEGT, 3 GEPLANT: REPO-AUDIT-003, PROTOCOL-002/003).
- **Nachweis:** SCR-0016..0025 mit versions.yaml-Approval-Einträgen; Selbst-Audit
  AUD-2026-0003 (Health 86→91/100 projiziert); REALITY_STATUS.md Abschnitt 5.
- **Umsetzung:** Agent Aurora

## AD-014: 5-Produkt-Repo-Struktur

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Neben dem Dual-Repo-Modell (Monorepo + Docs-Hub) werden 5 eigenständige
Produkt-Repos eingerichtet: `atclang` (ATCLang), `a-townchain` (Blockchain),
`globus-os` (Betriebssystem/ShivaCore-Kernel), `aurora-ai` (KI), `genesis-engine` (Game-Engine).
Module wurden aus `src/modules/` in die Produkt-Repos kopiert (Monorepo unverändert,
bleibt Integrations-/Deployment-Zentrale). `atclang` als vormals archiviertes Repo
reaktiviert (Historie erhalten, Inhalt aus Monorepo-Audits verlustfrei gesichert).

**Pflichten:** Produkt-Entwicklung im Produkt-Repo; Integration/Deployment im Monorepo;
Doku im Docs-Hub. Details & Mapping: [REPOSITORY_MAP.md](REPOSITORY_MAP.md).
API-Verifikation: 5/5 Repos aktiv, HEAD-Commits bestätigt (06.09.).
---

## AD-015: ShivaCore-Kernel-Repo-Trennung

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Der ShivaCore-Kernel erhält ein eigenständiges Repository: `atc-shivacore`
(reaktiviertes Archiv-Repo, 181-Commit-K29-Entwicklungshistorie erhalten).
Inhalt: `modules/atc-shivacore` (Kernel-Crate, K29, 674/674 Tests, Chain-ID 658467)
+ `modules/atc-shivacore-tools` — Stand Monorepo 06.09.2026.
Entsprechend AD-012/AD-013 (Kernel ≠ OS): GlobusOS-Repo behält nur den Userspace
(Shell, Desktop, FS, Net, Registry, Bootloader, Drivers, Editionen); Kernel-Referenz
im globus-os-README. Monorepo bleibt Integrations-Zentrale (Unified Cargo Workspace).
Kernel-Entwicklung künftig im Kernel-Repo; Service-Space-Migration der
Blockchain-Module aus dem Kernel-Crate bleibt eigener Sprint (unverändert).

API-Verifikation 06.09.: atc-shivacore aktiv (HEAD gepusht), globus-os Rückbau
gepusht, Beschreibungen/Topics aktualisiert.

---

## AD-016: Löschung der 124 archivierten Alt-Repos

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Nach vollständiger Daten-Sicherung wurden alle 124 archivierten
Alt-Repos (Namenspräfix `archiviert-`) aus der Organisation gelöscht.
Vorgehen: (1) SHA-256-Verifikation über alle 2.966 Dateien gegen Monorepo+Hub,
(2) Rettung von 527 Original-Ständen mit abweichendem Hash nach
docs/archive/repos-rescue/ (Commits 53a6a41, 51d949f), (3) Issues-Dump des
einzigen Issues führenden Alt-Repos (atc-genesis-engine, 9 Issues),
(4) API-Löschung 124/124, 0 Fehler (delete_repo-Scope, vom Owner autorisiert).

**Ergebnis:** Organisation umfasst exakt 8 aktive Repos: a-townchain-os
(Integration/Monorepo), a-townchain-os-docs (Wiki/Hub), atc-shivacore (Kernel),
atclang, a-townchain, globus-os, aurora-ai, genesis-engine (Produkte).
Git-Commit-Historien der gelöschten Repos sind mit ihnen entfernt; der finale
Datei-Stand jedes Repos ist vollständig erhalten (Hub-Rescue bzw. Monorepo).

---

## AD-017: Source-of-Truth & Modul-Sync (Produkt-Repos -> Monorepo)

**Datum:** 06.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore, Entscheidung an Agent Aurora delegiert) · **Umsetzung:** Agent Aurora

**Beschluss:** Kanonische Entwicklungsquelle fuer Modul-Code sind die
Produkt-Repos (atc-shivacore, atclang, a-townchain, globus-os, aurora-ai,
genesis-engine). Der Monorepo a-townchain-os ist ausschliesslich
INTEGRATIONSZIEL (Cargo-Workspace, Launch-Stack, Docker, CI, Gesamttests).

**Regeln:**
1. Modul-Code wird nie mehr direkt im Monorepo bearbeitet — nur ueber
   `scripts/sync_modules.py` einspielen (strikte Richtung: Produkt -> Monorepo).
2. Modi: `--check` = SHA-256-Divergenz-Bericht, `--sync` = einspielen
   (Produkt-Stand gewinnt). Sync-Punkte: vor Releases, Integration-Builds,
   Mainnet-Freeze, bei CI-Abweichung.
3. Monorepo-Integrationsdateien (Workspace-Files, docker/, scripts/, .github/,
   Meta-Files) bleiben vom Sync unberuehrt.
4. Dateien, die nur im Monorepo existieren, werden vom Sync NIE geloescht.
5. Startzustand 06.09.: 1.377 Dateien geprueft, 0 divergent, 0 neu (sauber).

**Rationale:** AD-014/AD-015 verlagern die Produkt-Entwicklung in die
Produkt-Repos (atc-shivacore 181 Commits, atclang 91 Commits Historie);
der Mainnet-Launch am 15.09. benoetigt den Monorepo als Workspace-Zentrale.
Damit ist die im Cross-Repo-Audit identifizierte Luecke (kein Sync-Mechanismus
-> Divergenz ab erstem produktseitigen Commit) geschlossen.

---

## AD-018: Gesamtvault im Wiki-Repository — Neuaufbau der Repositories

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Der komplette Stand des Monorepos a-townchain-os (Commit
8606e50: Launch-Stack, 60 Module, 2.500+ Dateien) wurde strukturgetreu nach
`docs/archive/monorepo-full/` in diesem Wiki-Repository verschoben. Der
Monorepo wurde daraufhin geleert (Redirect-README). Die uebrigen Repositories
(Monorepo + Produkt-Repos) werden neu aufgebaut; dieser Vault ist die
kanonische Quelle dafuer. Git-Historien bleiben als Reversibilitaetsnetz.

**Mainnet-Hinweis:** Der Launch-Stack (docker-compose, 10 Dienste) liegt
jetzt ausschließlich hier; fuer den 15.09.2026 muss der Rebuild ihn als
ersten Schritt restaurieren (Kopie aus dem Vault, nicht Neuentwicklung).

---

## AD-019: ATCLang 1.0 Master-Architektur verbindlich — Rebuild Phase 1

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** ATCLang-Zielarchitektur 1.0.0 EINGEFROREN (ARCHITECTURE
FREEZE: GO): Trennung nach Verantwortlichkeit, nicht nach Schlagworten —
frontend/semantics/ir/compiler/bytecode/artifact/vm/runtime/contracts/abi/
host/security/profiles/stdlib/package/cli. Kernentscheidungen: runtime
statt execution (VM = wie, Runtime = in welchem Zustand), artifact
Singular, security zentral (determinism dort, nicht in profiles),
Konsensgrenze mit Verbotsliste fuer Contract-Kontext, Source-of-Truth:
specs/ -> Implementierung -> Conformance.

**Konsolidierung (atclang 044604f):** Duplikate entfernt (atc-vm: 116-LOC-
Stub vs 978 LOC kanonisch; atc-stdlib 4/9 identisch; atc-atclang: kleinere
Parallel-Implementierungen) — bewahrt im Monorepo-Vault + Git-Historie.
modules/atclang -> src/atclang (kanonisch), frontend/-Zusammenfuehrung,
1.0-Subsystem-Platzhalter. Pipeline: Import + compile_source End-to-End
OK; pytest 74/106 gruen, 32 rot (Bytecode-API-Konformanz = Phase 2).
Reihenfolge geloeschter Mehrfachquellen: MULTIPLE SOURCES OF TRUTH →
aufgeloest (eine Implementierung je Verantwortlichkeit).

---

## AD-020: Rebuild-Startzustand — alle Repos geleert, Wiki-Vault ist Quelle

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Alle Repositories (ausser Wiki-Hub und atclang) wurden geleert;
der Neuaufbau startet aus dem Wiki-Vault. Vor der Leerung wurde byte-Identitaet
verifiziert: 1.239 Modul-Dateien der 5 Produkt-Repos + 2.157 Dateien des
Monorepos sind identisch in docs/archive/monorepo-full/ gesichert (0 fehlen).

**Endzustand der 8 Repos:**
- a-townchain-os-docs: WIKI-VAULT (alles Wissen: monorepo-full, monorepo-legacy,
  wiki-Archive, repos-rescue, Standards, Architektur-Entscheidungen)
- atclang: REBUILD PHASE 1 abgeschlossen (AD-019: 1.0-Struktur, 044604f)
- a-townchain-os, atc-shivacore, a-townchain, globus-os, aurora-ai,
  genesis-engine: GELEERT (Redirect-README; Git-Historien bleiben als
  Reversibilitaetsnetz erhalten)

**Reihenfolge des Wiederaufbaus:** Empfohlen zuerst Launch-Stack-Restauration
aus dem Vault (Mainnet 15.09.), dann atc-shivacore gem. AD-013 (SC-001…),
danach die uebrigen Produkt-Repos.

---

## AD-021: Sprachstrategie — ATCLang Rust-first, Python Referenz; Aurora Rust-Core + Python-AI-Layer

**Datum:** 06.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** ATCLang wird Rust-first: Compiler (Lexer/Parser/AST/Semantics/
ATC-IR/Optimizer/Codegen), Bytecode-Verifier, ATVM, Runtime, ABI/Artifact-
Validator, Security/Sandbox und CLI kanonisch in Rust. Python bleibt als
(1) Referenz-Implementierung — bestehender Code wird NICHT weggeworfen
(kein Rebuild), (2) SDK, (3) Test-/Fuzzing-Tooling, (4) AI-Integration.

**Phasen:** Phase 1: Python-Referenz weiterentwickeln (Sprachentwicklung).
Phase 2: Rust-Kernkomponenten (atclang-core/-ir/-bytecode/-verifier/-vm/
-runtime/-abi). Phase 3: Rust = kanonische Produktionsimplementierung.

**Dual-Stack-Differential-Modell:** Rust-Stack und Python-Referenz muessen
fuer identische Programme identische ASTs, IR, Bytecode und State-Transitions
erzeugen — Differential Testing wird zum verbindlichen Conformance-Kriterium.

**Aurora AI:** Rust Core (Model Manager, Scheduler, HAL, Security, IPC,
Plugin-Runtime) + Python AI-Layer (PyTorch/ONNX/LLM auf ROCm).
Kurzform: ATCLang -> Rust-first. Aurora -> Rust-Core + Python-AI-Layer.

**Begruendung:** Compiler != Runtime, beides Rust. ATCLang ist keine
Skriptsprache: Smart Contracts erfordern Memory Safety, Determinismus,
Geschwindigkeit, geringe Overheads, sichere Concurrency, Ressourcen-
Kontrolle und eine harte Host/VM-Security-Boundary.

---

## AD-022: ATCLang Architecture Baseline v1.0 — Rust Canonical Core, 21-Crate-Layout, Gates G0-G19

**Datum:** 06.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** ATCLang 1.0 = Rust Canonical Core + Python Reference/Tooling +
formal spezifizierte Bytecode/ABI/Artifact/VM-Schichten. Zielstruktur: crates/
mit 21 Rust-Crates (atclang-lexer, -parser, -ast, -semantics, -ir,
-ir-verifier, -compiler, -bytecode, -bytecode-verifier, -abi, -artifact, -vm,
-runtime, -host, -security, -profiles, -contracts, -gas, -state, -package,
-cli), python/ (sdk, reference, testing, fuzzing, tools), specs/ (language,
bytecode, abi, artifact, vm, gas, security, profiles), tests/ (conformance,
determinism, differential, …).

**Kernsatz:** Der Compiler erzeugt Code. Der Verifier entscheidet, ob Code
gueltig ist. Die ATVM fuehrt ausschliesslich verifizierten Code
deterministisch aus.

**Kernpunkte:** ATC-IR als zentrale Schicht (Typen, Kontroll-/Datenfluss,
Calls, Storage, Capabilities, Gas, Entry Points, ABI) · Independent Verifier
als Trust Boundary (vertraut nie dem Compiler) · ATCA-Artifact als Deployment
Unit (Magic…Signature) · Profiles CONTRACT/APPLICATION/SYSTEM/UNRESTRICTED ·
Capability-Security Deny-by-Default · Determinismus als Protokollregel ·
Konsensus-Grenze Transaction→…→Consensus · Differential Testing
Rust-vs-Python (Abweichung = FAIL) · Security Gates G0-G19, KEIN FREEZE vor
G18 (Security Audit) · Python kontrolliert NIE die Konsensus-Ausfuehrung.

**Migration:** KEIN zweiter Rewrite — Python-Referenz bleibt erhalten;
Pfad: preserve → specification extraction → formal Spec → Rust Canonical
Core → Differential Testing → Conformance → Security Audit → Freeze.
G0 (Repository Cleanup) erledigt; G1 (Language Specification) naechstes Gate,
specs/-Skelett angelegt. Verfeinert AD-019/AD-021.
Doku: atclang docs/ATCLANG_BASELINE_V1.md (Commit a7e1bd4).

---

## AD-023: Mainnet-Launch aufgehoben — kein Launch-Ziel, keine Deadline

**Datum:** 06.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (Michael) · **Umsetzung:** Agent Aurora

**Beschluss:** Der Mainnet-Launch ist als Ziel und als Deadline **aufgehoben**.
Das Datum 15.09.2026 ist kein Launch-Termin mehr. Es gibt **kein Datum** für
einen Mainnet-Launch — ein späterer Launch ist nicht terminiert und wird erst
durch eine neue Owner-Entscheidung festgelegt.

**Was das bedeutet:**
1. AD-018/AD-020-Reihenfolge wird **nicht** mehr durch die Mainnet-Deadline
   getrieben. Der Launch-Stack wird aus dem Vault restauriert, wenn es
   gebraucht wird (Testnet/Integration), nicht als Mainnet-Vorbereitung.
2. Die Rebuild-Priorität verschiebt sich auf **Qualität statt Termin**:
   atc-shivacore SC-001…SC-013 und die ATCLang-Gates G0-G19 (AD-022) ohne
   Zeitdruck — insbesondere G18 (Security Audit) vor jedem Freeze.
3. Roadmap-Milestone 4.0/4.1 („Mainnet Launch") ist **entfernt**; ein
   künftiger Launch erhält eine neue Milestone-Nummer und ein neues Datum,
   sobald der Owner es beschließt.
4. Historische Erwähnungen (Issues #36/#52/#69–71, AD-018-Text, Wiki-Kapitel,
   BaFin-Bericht) bleiben als Historie unangetastet; sie sind durch AD-023
   **superseded**, was den Launch-Zeitpunkt betrifft.

**Bewusst unverändert:** Testnet (5 Nodes, Docker) bleibt vollständiges Ziel;
Chain-ID 658467 (AD-004) bleibt final und wird beim künftigen Launch genutzt.

**Gültig seit:** 06.09.2026 (21:25 UTC+2)

---

## AD-024: Zielarchitektur Repository-Landkarte — ALLE Repositories erstellen (RESOLVED)

**Datum:** 06.09.2026 · **Status:** ✅ RESOLVED/UMGESETZT · **Entscheider:** Owner (Michael, 21:30 UTC+2: „Alle repository erstellen") · **Umsetzung:** Agent Aurora

**Beschluss (Variante a — erweitert):** SÄMTLICHE Repos der Landkarte werden JETZT
angelegt (P0+P1+P2 = 14 neue Repos). AD-016-Regel „exakt 8 aktive Repos" ist damit
fortgeschrieben: Ziel sind 22 aktive Repos mit vertikalen Produktgrenzen.
Wobei möglich wird Vault-Inhalt restauriert (atc-sdk, atc-contracts, atc-wallet,
atc-explorer, shivamon); neue Repos (atc-node, atc-indexer, atc-mining, atc-interop,
atc-oracle, atc-storage, atc-launchpad, atc-marketplace, atc-compute) erhalten
professionelle Grundstruktur (README/Architektur/LICENSE/STATUS).

**Vorschlag:** Wachstum der Organisation **vertikal entlang konkreter Produkte/Protokolle**
(Protocol → Node → SDK → Wallet/Explorer/Contracts → Applications), nicht horizontal
über weitere Core-Repos. Bestehende 8 Repos bleiben (🟢); neue vertikale Repos in Phasen:

- **Phase 1 (P0):** `atc-sdk` (Developer-Plattform), `atc-node` (Node-Binary getrennt vom
  Protocol-Core), `atc-contracts` (Standards + Contracts, NICHT im Core), `atc-wallet`
  (eigene Security Boundary: Keys/Seed/Signing/Recovery/Hardware)
- **Phase 2 (P1):** `atc-explorer` + `atc-indexer`, `atc-mining`, `atc-interop`
  (Bridges/IBC = eigene Sicherheitsdomäne), `shivamon` (Game getrennt von genesis-engine:
  Engine = Technologie, Shivamon = Produkt)
- **Phase 3 (P2):** `atc-oracle`, `atc-storage`, `atc-launchpad`, `atc-marketplace`,
  `atc-compute`

**Abgleich mit bestehender Struktur (AD-014–AD-020) — WICHTIG:**
1. Die meisten „neuen" Repos **existieren bereits als Vault-Module** in den Produkt-Repos:
   `a-townchain` enthält lt. REPOSITORY_MAP bereits atc-blockchain, atcnet, atc-wallet,
   atc-contracts, atc-bridge, atc-dex, atc-explorer, atc-assets, atc-zkp, atc-governance,
   atc-dns, atc-testnet (369 Dateien). atc-sdk, atc-contracts, atc-wallet, atc-explorer,
   atc-shivamon sind im Vault (`docs/archive/monorepo-full/`) nachweislich vorhanden.
   NEU ohne Vault-Bestand: atc-node, atc-indexer, atc-mining, atc-interop, atc-oracle,
   atc-storage.
2. **Rollen-Korrekturen zur Analyse:** `atc-shivacore` ist der **Microkernel** (AD-012/013,
   SC-001…SC-013), KEINE AI-/Runtime-Schicht. `a-townchain` ist bereits als Blockchain-Produkt-Repo
   zugewiesen (Chain-ID 658467). AI liegt bei `aurora-ai`.
3. **Konflikt mit AD-016:** AD-016 stellte auf „exakt 8 aktive Repos" ab (nach Löschung
   von 124 Alt-Repos wegen Wartungs-Overflow). AD-024 würde die Org mittelfristig wieder
   auf ~20 Repos wachsen lassen — mit saubereren Grenzen, aber mehr Wartungsoverhead.
4. **Timing per AD-023:** Kein Termin-Druck. Empfehlung: Repos erst anlegen, wenn der
   Rebuild den jeweiligen Bereich erreicht (Qualitäts-Gates), und dann als
   **Abspaltung aus dem Vault** (Restore), nicht als Neuentwicklung.

**Vorgeschlagene Promotion-Kriterien (statt fixer Phasen-Termine):**
- `atc-sdk`: sobald ATCLang Rust-ABI/Artifact-Stabilisierung erreicht ist (AD-022-Gates)
- `atc-node`: sobald Protocol-Core-Interface (AD-013-Kette) eingefroren ist
- `atc-wallet`: sobald atclang-Runtime Signing/HD-Wallet-Referenz (BIP44 m/44'/658467', AD-042) stabil
- `atc-contracts`: sobald ATVM-Contract-Kontext (AD-019/022) verifiziert ausführt
- Shivamon-Split: sobald genesis-engine den ECS-/World-Kern stabil liefert

**Entscheidung offen:** (a) P0-Repos SOFORT als eigenständige Repos anlegen (widerruft
AD-016-„exakt 8") oder (b) vertikale Grenzen als Modulgrenzen in den bestehenden
Produkt-Repos führen und Repos erst bei Erreichen der Promotion-Kriterien abspalten.
Empfehlung des Agents: (b) — qualitätsgetrieben per AD-023, keine Repos vor Content.

---

## AD-025: Spiel umbenannt — Shivamon → Genesis Chronicles

**Datum:** 06.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (Michael) · **Umsetzung:** Agent Aurora

**Beschluss:** Das Spiel „Shivamon" heißt ab sofort **„Genesis Chronicles"**.
Umbenannt wurden: GitHub-Repo (`shivamon` → `genesis-chronicles`, Alte URL
leitet automatisch um), Spiel-Titel und NFT-Bezeichnung in der gesamten Doku,
Contract-Identifier (`ShivamonContract` → `GenesisChroniclesContract`,
`ShivamonNFT` → `GenesisChroniclesNFT`, `ShivamonStats` →
`GenesisChroniclesStats`), Datei-/Verzeichnisnamen (snake_case) und der
Package-Name (`genesis-chronicles`).

**Bewusst unverändert:** Das Wiki-Vault-Archiv
(`docs/archive/monorepo-full/.../atc-shivamon/`) als historische Quelle;
AD-024-Entscheidungstext (Historie); Genesis-Engine-Repo-Name. Die ATC-9000
NFT-Standard-Nummer bleibt. Git-Historie des Repos bewahrt alle alten Namen.

**Kontext:** Repo war am 06.09. per AD-024 aus Vault-Inhalt (atc-shivamon)
restauriert und als eigenständiges Produkt-Repo neben genesis-engine angelegt
(Engine = Technologie, Spiel = Produkt). Keine Live-Systeme betroffen.

---

## AD-026: Repository-Bauhierarchie — sequenzieller Aufbau nach Abhängigkeit

**Datum:** 07.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore: „erst atclang, dann Kernel, KI, Blockchain, Betriebssystem usw.") · **Umsetzung:** Agent Aurora

**Beschluss:** Die 22 Repos bauen in fester Reihenfolge aufeinander auf:
L0 atclang → L1 atc-shivacore → L2 aurora-ai → L3 a-townchain → L4 globus-os
→ L5 Blockchain-Services (node, contracts, wallet, sdk, storage, compute,
oracle, indexer, explorer, interop, mining, marketplace, launchpad) →
L6 genesis-engine → genesis-chronicles → L7 a-townchain-os (Integration).
Wiki-Hub parallel. Ein Layer startet erst nach Gate-Erreichung des
vorherigen. AD-020-Empfehlung (Launch-Stack zuerst) damit ersetzt —
AD-023 hat die Launch-Deadline aufgehoben.

**GitHub-seitig umgesetzt:** Layer-Tags [L0]…[L7]/[Hub] in allen 22
Repo-Beschreibungen, Topics build-layer-N, verbindliche Tabelle in
REPOSITORY_MAP.md, AGENT_MANIFEST im Monorepo gespiegelt.

---

## AD-027: Lauffähigkeits-Roadmap M1-M8 — Ökosystem Stück für Stück lauffähig

**Datum:** 07.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (ShivaCore) · **Umsetzung:** Agent Aurora

**Beschluss:** Die AD-026-Bauhierarchie wird mit harten Lauffähigkeits-Meilensteinen
unterlegt: M1 Sprache läuft (atclang, Gates G1-G6, End-to-End hello.atc) → M2 Kernel
läuft (Vault-Restauration atc-shivacore, 674/674 Tests + boot L0-L10) → M3 KI läuft
(aurora-ai via Kernel-Event-Bridge) → M4 Blockchain läuft (2 Nodes sync, Chain-ID
658467, Contract auf ATVM) → M5 OS läuft (globus-init-Bootchain) → M6 Dienste laufen
(node→wallet→sdk→explorer→…) → M7 Spiel läuft (Engine→Genesis Chronicles, NFT auf
Chain) → M8 Ökosystem läuft (Monorepo-Launch-Stack, docker-compose healthy). Jede
Meile hat ein Run-Kriterium mit Test-/Boot-/Run-Nachweis (Reality-Check-Regel).
Vault-Restaurationen statt Neubau (AD-020). Volltext:
docs/roadmap/LAUFFAEHIGKEITS_ROADMAP.md.

---

## AD-028: Service-Space-Migration ausgefuehrt + Platzierungsentscheidungen

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (ShivaCore: „Korrigiere die Fehler") · **Umsetzung:** Agent Aurora

**1) AD-012-Delta BEHOBEN:** Die 10 Service-Module (blockchain, consensus,
genesis, genesis_bridge, gossip_bridge, atcnet, did, remote_caps,
knowledge_graph, security_audit) sind aus dem Rust-Kernel-Crate in den neuen
Crate service_space/ (shivacore-service-space) migriert — Abhaengigkeit
ausschliesslich abwaerts (Service -> Kernel). net.rs verbleibt als K12-Netz-
Primitive (HAL-Ebene) im Kernel. kernel_init.rs vom Blockchain-Stack befreit
(L9 = Mempool + VM + Contracts). allocator.rs: global_allocator nur im
echten Boot-Binary (x86-boot). Verifikation Rust 1.98.1 stable: Kernel
394/394 + Service-Space 280/280 = 674/674 wie Baseline, Boot L0-L10 gruen.
Commits: atc-shivacore 02c6845 (Migration), bbbd9f1 (atc-security-Restauration).

**2) Platzierungsentscheidungen (Tiefenanalyse-Fragen, delegiert entschieden):**
- atc-security (22 Dateien) -> atc-shivacore-Repo (Security-Tooling-Cluster
  beim Kernel, nicht im M8 begraben).
- atc-monitoring -> verbleibt im Vault bis M8 (Prometheus-Regeln zielen auf
  den Launch-Stack; atc-node-Monitoring entsteht bei dessen M6-Neubau).
- atc-standards-.atc-Referenzimplementierungen -> atc-contracts-Repo
  (modules/atc-standards-refs/, Commit dafd38d); die MD-Standards bleiben
  kanonisch im Hub (docs/standards/).

---

## G1-PASSED (ATCLang Language Specification, 07.09.2026)

Gate G1 der AD-022-Baseline erreicht (atclang Commit abb3262): Die vollstaendige
Sprach-Spezifikation wurde aus der Python-Referenz extrahiert — specs/language/
SPEC.md mit 63 Tokens, 76 Keywords, 84 Typ-Bezeichnern, 31 ATC-Namespaces,
28 Parser-Produktionen als EBNF (inkl. Praezedenztabelle, normativem Compound-
Desugar x+=y → x=x+y, lexikalischer Besonderheit let/reservierte Namen), 50
AST-Knoten, Semantik-Regeln (Scopes, Typ-Checks, 13 Builtins), 9 Stdlib-Modulen
und dem Fehlermodell (45 Klassen). Maschinenlesbares Extrakt specs/language/
registry.json fuer Conformance-/Differential-Tooling (G19). Normativitaet:
SPEC folgt Referenz a7e1bd4, Abweichung = Fehler. Naechstes Gate: G2 (Semantics).

---

## AD-029: ATC Repository Governance Standards (ATC-STD-REPO-001/002/003)

**Datum:** 07.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (Vorgabe), Agent Aurora (Ausformulierung + Harmonisierung)

Der Owner hat die Repository-Governance-Schicht definiert; ausformuliert als drei
Standards (Status PROPOSED v1.0.0, verbindlich fuer NEUE Repos sofort, Bestand
mit Migrationspflicht bis M8 fuer R2+):

1. **ATC-STD-REPO-001** (docs/standards/): Top-Level-Struktur, Verantwortlichkeits-
   Trennung, Domain-Strukturen (Blockchain/ATCLang/Wallet), Doku-Mindeststandard
   (12 README-Abschnitte), ADR-Standard, Repository-Hygiene, zentrale Datei
   docs/REPOSITORY_STANDARD.md je Repo.
2. **ATC-STD-REPO-002**: Namensregeln (atc-<domain> + Produktlinien), 8 Repository-
   Typen, Compliance-Level R0-R4, NORMATIVE Klassifizierung aller 22 aktiven
   Repos (atc-shivacore CORE/OS R4, atclang CORE R4, a-townchain CORE R4, Hub
   SPEC R3, Monorepo INFRA R3, Skelette R1 ...), Monorepo-Grenzen, Abhaengigkeits-
   richtungen (keine Zyklen; AD-026 L0-L7 ist die konkrete Auspraegung).
3. **ATC-STD-REPO-003**: SECURITY.md-Pflicht ab R2, Blockchain-Testkategorien
   (consensus/cryptography/state-transition/replay/serialization/adversarial),
   CI/CD-Minimum (ci/test/security/release), Pipeline mit Release-Gate
   (GATE: PASS / NO-GO), organisationsweiter SemVer + Trennung Protocol/
   Implementation/Specification, Secrets-Verbot.

**Harmonisierungen (Abweichungen vom Owner-Entwurf, der Realitaet AD-025/026/028
angepasst):** kein atc-standards-Repo (Standards kanonisch im Hub, .atc-Refs in
atc-contracts per AD-028); shivamon = genesis-chronicles (AD-025); atc-core-Rolle
= a-townchain; ADR-Zwei-Ebenen-Modell (zentrales DECISIONS_REGISTER bleibt
autoritativ fuer Organisations-Entscheidungen, Repo-ADRs fuer Lokal-Entscheidungen);
Bestands-Repos mit modules/-Layout dokumentieren Mapping statt Zwangs-Migration.

---

## AD-030: atc-standards — kanonisches Standards-Repository errichtet

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner („Erstelle ein Repository fuer alle meine definierten Standards") · **Umsetzung:** Agent Aurora (Repo-ID 1360175048, Commit 0eea944)

Das Repository **A-TownChain-Okosystems/atc-standards** ist ab sofort DIE
kanonische Heimat aller definierten Standards: 102 ATC-Standards (ATC-01…99,
ATC-LIC, ATC_ECOSYSTEM_STANDARDS), ATS-LIC, Governance-Standards
(ATC-STD-REPO-001/002/003), Master-Registry (STANDARDS_REGISTRY + OVERVIEW) und
die .atc-Referenzimplementierungen (registry.atc + 4 Standards-Vertraege).
Typ SPEC, Level R3, self-compliant nach ATC-STD-REPO-001 (12-Abschnitte-README,
LICENSE, SECURITY.md, CHANGELOG, docs/REPOSITORY_STANDARD.md). Topics gesetzt.

**Regeln:** (1) Kuenftige Standard-Aenderungen NUR im atc-standards-Repo —
die Hub-Kopie (docs/standards/) ist ab jetzt ARCHIV-SNAPSHOT (CANONICAL.md
weist dorthin). (2) Die AD-029-Harmonisierung „kein atc-standards-Repo,
Standards kanonisch im Hub" ist HIERMIT ERSETZT. (3) Repo-Landschaft: 22 → 23
aktive Repos. (4) Die .atc-Referenzen bleiben zusaetzlich in atc-contracts
ausfuehrbar (Dublette bewusst: Standards-Repo = Norm, atc-contracts =
Contract-Ausfuehrung).

---

## AD-031: ATC Repository Governance Specification — aus Empfehlungen werden verbindliche Regeln

**Datum:** 07.09.2026 · **Status:** RESOLVED/NORMATIV · **Entscheider:** Owner (Erweiterungs-Mandat Abschnitte 16-35), Agent Aurora (Spezifikation + Validator-Implementierung) · **Commits:** atc-standards 0211d29

Die ATC-STD-REPO-Serie wurde von der Ordnerstruktur-Empfehlung zur FORMALEN
GOVERNANCE SPEZIFIKATION erweitert (kanonisch: atc-standards-Repo, AD-030):

1. **ATC-STD-REPO-001 v1.0.0 (FORMAL):** RFC-2119-MUST/SHOULD/MAY-Regeln,
   Metadaten-Standard (.atc/repository|ownership|lifecycle|compliance.yaml),
   Compliance-Matrix M-01…M-16 je Level R0-R4, Validator-Regeln V-01…V-16
   (maschinenpruefbar), Compliance-Badge-Pflicht ab R3.
2. **ATC-STD-REPO-002 v1.0.1:** Ownership-Standard (CODEOWNERS aus ownership.yaml
   abgeleitet), Lifecycle-Zustandsmaschine (experimental→…→archived, Springen
   UNZULAESSIG), Security-Klassen S0-S4 (S4 = Protocol-Critical mit
   Reproducible-Build-Pflicht), zentraler Dependency Graph, Registry-Pflicht.
3. **ATC-STD-REPO-003 v1.0.1:** Branching-Standard, Conventional Commits,
   PR-Standard (ab S3/S4 mit Threat-Model/Consensus/State-Transition-Impact),
   Release-Gates GATE-01…GATE-10 (ein fehlgeschlagenes Gate = NO-GO),
   Dependency Policy (Approved/Restricted/Deprecated/Blocked), Third-Party-
   Dokumentation, API-Stability-Levels, Breaking-Change-Disziplin,
   Reproducible Builds, Artifact Management (SHA-256/512 + Signaturen),
   Health-Score-Verfahren (Schwellen 95/85/70/50, GATE: PASS ab 85 + 0 FAILs).

4. **Infrastruktur (physisch im atc-standards-Repo):** registry/repositories.yaml
   als ZENTRALE maschinenlesbare Registry (23 Repos mit Klassifizierung,
   R-Maturity, S-Security, Lifecycle-Status, Layer), teams.yaml,
   dependencies.yaml (L0-L7-Graph nach AD-026), 4 Metadaten-Schemas,
   Templates (.atc-Vorlagen, PR-Template, CI-Vorlage).

5. **tools/atc-repo-audit v0.1.0:** ECHTER, lauffaehiger Validator (Python
   stdlib-only, 16 Pruefregeln, 8 Score-Kategorien, GATE: PASS/NO-GO,
   Exit-Code CI-tauglich). Self-Compliance: atc-standards auditiert sich
   selbst (16/16 PASS, Score 100/100, GATE: PASS; .atc-Metadaten, CODEOWNERS,
   Governance-CI-Workflow). Bestands-Realitaet ehrlich vermessen: Demo-Audit
   atc-shivacore (R4) = 12 MUST-FAILs → dokumentierter Migrations-Backlog
   (Bestand R2+ erfuellt MUST-Regeln bis M8, gemaess 001 §0).

Governance Chain aktiv: Standards → Schema → Template → Repository Creation →
atc-repo-audit → PASS (Development → CI → Security → Architecture → Release →
Production) / FAIL (NO-GO).

---

## AD-032: Wiki-Konsolidierung II — weitere Standards ins atc-standards-Repo uebertragen

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner („Pruefe die wiki auf weitere Standards und verschiebe sie") · **Commits:** atc-standards 76e2c5e

Systematische Wiki-Durchsuchung nach nicht uebertragenen Standards. 6 weitere
Dokumente kanonisch ins atc-standards-Repo uebernommen (Repo-Total: 115):

1. **ats/ATS_STANDARDS.md** — ATS-1000…1007 (ShivaOS Kernel/Stack-Standards).
   Von 4 Hub-Kopien die vollstaendigste Fassung (283 Zeilen, wiki/standards/docs
   + module-docs/kernel identisch) gewaehlt; aeltere 198/199-Zeilen-Entwuerfe
   verworfen.
2. **atc/ATC_STANDARDS.md** — ATC-0001…0008 (Core-Protokolle: Identity, Token,
   Consensus, Network, ...). Vollstaendige 233-Zeilen-Fassung (inkl. ATC-9000-
   Sektion); die module-docs-Variante war am Ende abgebrochen (201 Zeilen,
   mitten in der Zeile).
3. **atc/ATC_TOKEN_STANDARD.md** — Kurzreferenz ATC-001/8300/9000/9900.
4. **licensing/ATVM_LICENSE_GATE_SPEC.md** — ATC-LIC-Enforcement-Spezifikation.
5. **licensing/IP_LICENSE_DASHBOARD_SPEC.md** — IP & License Registry (GlobusOS).
6. **licensing/SMART_CONTRACT_RICHTLINIE.md** — BaFin-Richtlinie (21KB, normativ).

**Abgrenzung (bewusst NICHT uebertragen):** COMPLIANCE_HANDBUCH.md und
BAFIN_KONFORMITAETSBERICHT.md bleiben im Hub — Handbuch/Report sind
Compliance-DOKUMENTATION, keine Standards. ATCLang-Gates-Spezifikationen leben
in atclang (AD-022). Hub-Kopien der uebertragenen Standards bleiben als
Archiv-Snapshot (AD-030-Regel), kanonische Pflege nur im atc-standards-Repo.
Registry-Sektion 'Aus dem Wiki konsolidiert' dokumentiert Herkunft je Dokument.

---

## AD-033: ShivaCore Memory Management & Interface-Protokolle (J-K09/J-K10)

**Datum:** 07.09.2026 · **Status:** RESOLVED/NORMATIV · **Entscheider:** Owner (Vorgabe), Agent Aurora (Dokumentation) · **Lokaler Stand:** Commit 4b04cd3 (noch nicht auf GitHub gepusht)

### J-K09: Memory Management (KOMPLETT, 32/32 Tests: 18 Memory + 14 Capability)

1. **Kernel owns Block-Allokator (J-K09.2 Phase 3):** Der Block-Allokator ist
   EIGENTUM des Kernels — nicht Userspace. Minimaler Default-Kernel-Heap,
   max. Block-Groesse 48 MiB.
2. **Memcheck-Standard:** (a) Memory-Audit bei Kernel-Boot — der Kernel
   durchlaeuft alle internen Speicherstrukturen (Frame-Allokator, Page-Tables,
   Slub-Caches) und prueft auf Korruption. (b) Slub-Allokator fuer kleine
   Objekte (Speicherverschnitt minimieren). (c) Rolling Updates NUR nach
   successful Memcheck — kein Update-Pfad ohne verifizierten Speicherzustand.
3. **Frame-Allokator-Integration:** Virtual Memory und Memcheck arbeiten
   direkt mit dem physischen Frame-Allokator — keine abstrakte
   Zwischenschicht.

### J-K10: Interface-Protokolle Kernel ↔ Userspace (SKELETT, Projektion 21:00)

4. **Objektmodell:** Prozess-, Kanal-, Thread-Objekte mit Thread-Gruppen
   fuer Gruppen-Scheduling.
5. **POSIX-FREI:** KEINE File-Deskriptoren (fd 0/1/2 entfernt), KEINE POSIX-
   Systemaufrufe. Stattdessen: ServerPort/S1/S2 in CSpace; RelayPort
   (Hardware-facing) auf Level 0 in CSpace. KEIN Nameserver.
6. **IPC:** Ring-Buffer mit Zero-Copy + Capability-Check. Direct Kernel
   Calls nur mit Capability. Shared Memory / Direct Map / unmapped
   Physical RAM NUR fuer Base-Drivers.
7. **VFS-Trennung:** H2/v4/v5-Inhalt in libvfs ab Tag 1 (Kernel haelt NUR
   Minimal-VFS, gemaess AD-012).

### Vergleich zu Linux (explizite Nicht-Ziele)

KEINE syscalls/fd/mmap/ioctl/signal/fork/exec — das Objektmodell ist
eigenstaendig. ~50% des Kernel-Codes betroffen. Fastboot-Ziel: Tick-Count-
Tracking der Boot-Phase.

**GitHub-Status:** Lokal implementiert und getestet (32/32 gruen), Push in
atc-shivacore steht aus. AD-028 gilt unveraendert (Service-Space-Trennung).

---

## AD-034: ATC-STD-000 — Standards Governance & Specification Standard (Verfassung)

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (Meta-Spezifikation vorgegeben), Agent Aurora (Ausformulierung, Validator, Umnummerierung) · **Commit:** atc-standards 4f318f0

ATC-STD-000 v1.0.0 (Status DRAFT gemaess Owner-Vorgabe; Approval-Chain nach
§15 ist der offene Schritt zu APPROVED/STABLE) ist ab sofort die VERFASSUNG
des gesamten ATC-Standardsystems — der Standard fuer Standards:

1. **ID-System mit Domain-Raedern:** 000 Governance (ATC-STD-000 selbst),
   100-199 Architecture, 200-299 Repository & Git, 300-399 Development,
   400-499 Security, 500-599 Protocol, 600-699 Blockchain, 700-799 AI,
   800-899 OS/Runtime, 900-999 Infrastructure, 1000+ Applications.
2. **Lifecycle-Zustandsmaschine:** IDEA→PROPOSED→DRAFT→REVIEW→CANDIDATE→
   APPROVED→STABLE→DEPRECATED→RETIRED — sequenziell, Springen unzulaessig
   (Ausnahme Rueckstufung nach abgelehntem Review).
3. **Metadaten-Pflicht:** Jeder Standard hat einen maschinenlesbaren YAML-
   Header (id/title/version/status/category/owner/created/updated/normative
   + supersedes/superseded_by).
4. **REQ-System:** Eindeutige REQ-IDs mit Klassifizierung MANDATORY/
   RECOMMENDED/OPTIONAL/CONDITIONAL; Compliance je REQ: PASS/FAIL/PARTIAL/N-A;
   Compliance-Level L0-L4.
5. **Versionierung:** SemVer; MAJOR = Breaking (MUST→MUST NOT, SHOULD→MUST,
   Semantik-/Compliance-Aenderung); Change Control an STABLE nur via SCR
   (SCR-0001-Schema); Review-Chain Technical→Security→Architecture→Approval;
   Evidence-Requirement; Supersession + Migration-Guide-Pflicht.
6. **Registry-Pflicht (§19):** registry/standards.yaml ist das Herzstueck —
   KEIN EINTRAG = KEIN STANDARD. Ergaenzt: categories.yaml, versions.yaml,
   lifecycle.yaml; dependencies.yaml um Standard-Graph (Zyklenerkennung).
7. **Governance-Grundsatz (§21, REQ-STD-000):** 'No ATC Standard is normative
   unless it is registered, versioned, reviewed and explicitly approved
   according to this specification.' — Registry + Repo schlagen bei
   Widerspruch README, Wiki, Issues, Chat und Code.

**Vollzogene Umnummerierung:** ATC-STD-REPO-001/002/003 → ATC-STD-201/202/203
(v1.0.1, supersedes-Vermerke, 14 Dateien umgestellt; atc-repo-audit,
.atc-Metadaten, Schemas, README, Registry referenzieren die neuen IDs).
Legacy-Serien (ATC-01…99, ATC-0001…0008, ATS-1000…1007) behalten ihre
Nummerierung, sind aber fuer alle Aenderungen durch ATC-STD-000 regiert.

**Validator:** tools/atc-std-validator v0.1.0 (15 Regeln S-01…S-15, inkl.
Zyklenerkennung, stdlib-only). Selbsttest: ATC-STD-000/201/202/203 alle
COMPLIANT. CI im Standards-Repo validiert ab jetzt beide: Repository-Audit
(16/16 PASS, Score 100) und Standard-Validation.

**Naechster Schritt fuer STABLE:** Durchlauf der Review-Chain (§15) und
formale Approval durch den Owner.

**AD-034-Nachtrag (07.09., atc-standards 8eda21a):** Review-Chain nach
ATC-STD-000 §15 fuer die Verfassung durchgefuehrt — Technical, Security und
Architecture Review alle PASS (0 blockierende, 5 nicht-blockierende Befunde
als SCR-Empfehlungen T-F01/T-F02/S-F01/S-F02/A-F01 dokumentiert in
governance/APPROVAL_PACKAGE_ATC-STD-000.md). ATC-STD-000 v1.0.0 steht auf
CANDIDATE und wartet auf die formale Owner-Approval → APPROVED/STABLE.
Sammel-Empfehlung im Paket: Co-Approval ATC-STD-201/202/203 nachziehen.

**AD-034-Nachtrag 2 (07.09., atc-standards 45e3d7d):** Owner-Formalfassung
ATC-STD-000 v1.0.0 (35 Abschnitte) als verbindlicher Text angenommen —
ersetzt den Agent-Entwurf. Erweitert um Governance-Hierarchy, Review-
Kataloge, Approval/STABLE-Regeln, Konflikt-Resolution (§30), Emergency
Changes (§31), Standard-Integrity (§33), Meta-Compliance (§34). Review-
Chain gegen DIESE Fassung durchlaufen: Technical/Security/Architecture
3/3 PASS, 0 Blocker, 16/16 REQ PASS → CANDIDATE. Approval BLOCKED beim
Owner (APPROVE/REQUEST CHANGES/REJECT). Damit laeuft die Verfassung durch
die eigene Kette — keine Ausnahme von den Regeln, die sie definiert.

---

## AD-035: Standards-Governance-Komponenten vervollstaendigt

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (Auftrag 'Fehlende Komponenten erstellen'), Agent Aurora (Ausfuehrung) · **Commit:** atc-standards ed3d048

Die gegenueber der Soll-Struktur (Owner-§27-Layout) und dem SCR-Backlog aus
dem ATC-STD-000-Approval-Paket fehlenden Komponenten sind errichtet:

1. **governance/CHANGE_CONTROL.md:** SCR-Verfahren operationalisiert
   (Lebenszyklus PROPOSED→REVIEW→DECIDED→IMPLEMENTED→CLOSED, SCR-Registry
   SCR-0001…0004, Emergency-Rueckkopplung gemaess §31).
2. **governance/APPROVAL_PROCESS.md:** Freigabe-Ablauf CANDIDATE→STABLE mit
   Pflichten je Entscheidung; Uebergangs-Rollenregelung (Owner = Approver,
   Agent = Autor/Reviewer/Executor, nie Approver) bis SCR-0004.
3. **change-requests/:** SCR-0001 (ID-Allokation, PENDING), SCR-0002
   (OBSOLETE — L↔R-Doppelskala durch Formalfassung §22 aufgeloest; Nachweis,
   dass der Prozess auch Schliessungen abbildet), SCR-0003 (§33-Integritaets-
   umsetzung, PENDING; Teilumsetzung CODEOWNERS; Branch-Absicherung wartet
   auf Owner-Option A: Agent auf PR-Flow / B: dokumentierte Ausnahme),
   SCR-0004 (Rollenmodell, PENDING).
4. **CODEOWNERS** (§33-Teilumsetzung).
5. **Root-Metadateien** (§27-Soll): ARCHITECTURE.md (Governance-Fluss),
   STATUS.md (Standard-/SCR-Status), ROADMAP.md (Q3/2026 + Ausbau 100/400/
   800er-Bereich nach Baulogik AD-026/027).

Regel beibehalten: ATC-STD-000 bleibt CANDIDATE, Approval BLOCKED beim Owner;
SCR-Entscheidungen sind Owner-Vorbehalt. Verifikation: 4/4 Standards
COMPLIANT, Repo-Audit R3 GATE PASS (Score 97; einziger WARN = historisches
Conventional-Commits-Verhaeltnis).

---

## AD-036: Naming Convention verbindlich (ATC-STD-000 §36)

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (Vorgabe), Agent Aurora (Verankerung, Schema, Validator) · **Commit:** atc-standards 824528b

Zentrale Benennungsnorm des ATC-Standardsystems, per Owner-Mandat als §36 in
ATC-STD-000 verankert (Candidate-Revision vor Approval — keine SCR-Pflicht,
da nicht STABLE):

1. **ID-Tabelle:** ATC-STD-NNN, REQ-<DOMAENE>-NNN, F-NNN, SCR-NNN, ADR-NNN
   (zentrales Register behaelt AD-NNN, Bestand unveraendert), ATC-SA-NNN,
   TC-/TS-/GATE-NNN, ATC-SCHEMA/-PROTO/-SPEC/-DOC-NNN, ATC-REL-X.Y.Z,
   ATC-DOC-NNN. Mindestens dreistellig, fuehrende Nullen erlaubt.
2. **Repository-Namen:** Neue Repos atc-<domain>-<component>; Bestand-Brand-
   Repos (atclang, a-townchain, globus-os, aurora-ai, genesis-engine,
   genesis-chronicles, a-townchain-os, a-townchain-os-docs) immutable.
3. **Dateinamen:** ATC-STD-NNN.md, <name>.schema.json, ATC-STD-NNN.integrity/
   review/compliance.yaml (aktuelle Realisierung ueber approval/ + registry/).
4. **ID-Immutabilitaet:** IDs werden nie umbenannt/wiederverwendet; nur
   Version/Status/Titel/Kategorie aendern sich (Version-Pinning §29).
5. **Maschinenpruefbarkeit (MUST):** schemas/naming-conventions.schema.json
   (valides JSON) + atc-std-validator v0.1.1 Regel S-16 (Dateiname==ID,
   3-stellige Mindest-IDs, Schema-Existenz) — CI lehnt ungueltige Namen ab.
6. **Findings-Registry:** registry/findings.yaml mit kanonischen F-001…F-005
   (Aliases T-F01/S-F01…/A-F01 als Herkunftsverweis, SCR-Verweise).
7. **GATE-Migration:** ATC-STD-203 Release-Gates auf GATE-001…GATE-010
   umgestellt (Naming-konform).

Verifikation: 4/4 Standards COMPLIANT (S-16 aktiv), Repo-Audit R3 GATE PASS,
Requirement-Matrix 17/17 PASS, Snapshot aktualisiert. Die offene Owner-
Approval fuer ATC-STD-000 deckt die erweiterte 36-Abschnitt-Fassung ab.

---

## AD-037: Prompt Engineering Theory verankert (ATC-SPEC-001)

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:**
Owner (Quellenmaterial APOS/ACE), Agent Aurora (Ausarbeitung) · **Commit:**
a-townchain-os-docs (dieser Stand)

Das vom Owner gelieferte lerntheoretische Material (APOS-Theorie nach
Dubinsky, ACE-Lehrzyklus, Instruktionsdesign-Strategien, kognitive Leiter)
ist zur kanonischen Prompt-Engineering-Theorie des Oekosystems ausgearbeitet:
**docs/ai/ATC-SPEC-001-PROMPT_ENGINEERING_THEORY.md** (v1.0.0, DRAFT,
normative: false — erste reale Nutzung der ATC-SPEC-NNN-Nomenklatur aus
ATC-STD-000 §36).

Kern: Prompt Engineering = Steuerung kognitiver Strukturen (APOS: Action →
Process → Object → Schema), geschlossene kognitive Schleife (Lernen →
Ausfuehren → Verifizieren → Schema), 12 Instruktionsdesign-Templates,
6-Kriterien-Validierungs-Framework (Stufen-Zuordnung, Mechanismus, Inhalt,
Transfer, Verifizierbarkeit, Determinismus). Oekosystem-Integration:
Aurora AI = Action/Process (deterministisch, konsistent mit AD-021/022
Differential-Testing), KAI = Process + Review-Pflicht, Genesis AI = Schema
(kreativ). Promotion-Pfad zu ATC-STD-701 (AI 700-799) nach
Praxisvalidierung.

---

## AD-038: Normatives Naming-Hardening §7 + CI-Enforcement

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (Vorgabe 'normative Fassung'), Agent Aurora (Umsetzung) · **Commit:** atc-standards 5a17d59

Die Naming Convention (AD-036) ist per Owner-Mandat zur normativen
Governance-Regel gehärtet — ATC-STD-000 erhaelt §7 'ATC Naming,
Identification and Namespace Convention' mit 7.1-7.11:

1. **7.1-7.2:** ASCII-only, Status niemals in der ID, 14 Identifier-Klassen
   (ATC-STD-NNN, REQ-STD-NNN, F-NNN, SCR-NNN, ADR-NNN, ATC-SA-NNN, TC-/TS-/
   GATE-NNN, ATC-SCHEMA/-PROTO/-SPEC/-DOC-NNN, ATC-REL-X.Y.Z); NNN >= 3
   Stellen (ATC-STD-1/-01 ungueltig).
2. **7.3-7.4:** ID-Immutability-MUST ('The canonical object identifier MUST
   remain immutable throughout the complete lifecycle'), Version getrennt,
   Pinning via ID@version.
3. **7.5-7.6:** Repository-Namen atc-<domain>-<component>; Familien-
   Namespaces atc-*/atclang-*/globus-*/aurora-*; Brand-Bestand grandfathered.
   Dateinamen-Schema inkl. verbotener Varianten (atc_std_000.md,
   standard-final-v2.md).
4. **7.7-7.9:** Registry = Allokations-Autoritaet; Duplicate Prevention
   (gleiches Objekt gueltig / anderes Objekt FAIL); Reserved Identifiers
   (Platzhalter, Status-Marker, ATC-STD-000 selbst, Legacy-Serien, AD-NNN
   als grandfathered Legacy-Form von ADR-NNN).
5. **7.10-7.11:** naming-conventions.schema.json = zentrale Regeldefinition
   (additionalProperties: false, 6 Regelgruppen); Validator/CI leiten Regeln
   NUR daraus ab; CI-Workflow naming-governance.yml validiert bei push/PR
   (S-01…S-16 je Standard, S-17 Duplicate Detection, Repo-Audit R3) —
   FAIL blockiert.

Abschnitts-Renumbering: 7-35 → 8-36 (alte §36-Vorversion in §7 aufgegangen).
Mapping für Alt-Referenzen: N → N+1 (N=7…35), 36 → 7. Review-Berichte und
frühere AD-Einträge beziehen sich auf die Vorfassung. Verifikation: 4/4
Standards COMPLIANT, S-17 PASS, Repo-Audit R3 GATE PASS, Requirement-Matrix
20/20 PASS. Owner-Approval für ATC-STD-000 weiterhin BLOCKED/PENDING und
deckt die erweiterte Struktur ab.

---

## AD-039: Repository-Landschaft auf ATC-STD-201/202/203 gebracht (Governance-Sweep)

**Datum:** 07.09.2026 · **Status:** RESOLVED/UMGESETZT · **Entscheider:** Owner (Auftrag), Agent Aurora (Ausfuehrung) · **Umfang:** 22/22 aktiven Repos (ausser atc-standards, das bereits compliant ist)

Alle 22 Repos der AD-024-Landschaft sind nach den Vorgaben des
atc-standards-Repos ueberarbeitet: Vorher 22/22 GATE NO-GO (Scores 25-44),
nachher 22/22 GATE PASS auf ihrem registry-eigenen R-Level. Je Repo errichtet:
.atc-Metadaten (repository/ownership/lifecycle/compliance.yaml), SECURITY.md,
CODEOWNERS, docs/REPOSITORY_STANDARD.md (Zwei-Ebenen-ADR-Modell), CHANGELOG,
tests/TESTPLAN.md (Skelette), governance-ci.yml (Audit bei jedem Push/PR
gegen atc-standards + repositories.yaml), README ATC-COMPLIANCE-Anhang
(11 Abschnitte, Badge, DECISIONS_REGISTER-Referenz). Bestehende Inhalte
wurden nie ueberschrieben — nur ergaenzt. CI-Verifikation auf GitHub:
Stichprobe 4/4 SUCCESS. Restpunkte: V-16 heilt mit konventionellen Commits;
Build-/Test-Pipelines je Meilenstein M1-M8; Branch-Absicherung SCR-0003
bleibt Owner-Option A/B. Bericht: docs/audits/REPO_GOVERNANCE_SWEEP.md.

---

## AD-040: Bug- & Konsistenz-Lebenszyklus verbindlich (ATC-STD-BUG-001…004)

**Datum:** 07.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (Mandat) · **Ort:** atc-standards (kanonisch)

Der Owner hat einen 4-stufigen Bug- und Konsistenzstandard als verbindlich
mandatiert. Kernprinzip: Trennung Finding → Dokumentation → Fix →
Synchronitätsprüfung. Errichtet als vier Standards (candidate, normativ
per Owner-Mandat, §33): BUG-001 Finding (Pflichtprozess, 14 Pflichtfelder,
Severity S0-S4), BUG-002 Documentation (F-NNN-Pflichtstruktur),
BUG-003 Fix-Lifecycle (12 Stufen, SCR-Pflicht, DoD 10 Punkte),
BUG-004 Repository Sync & Merge Gate (Konsistenzmatrix 11 Beziehungen,
SYNC-Statusmodell OK/DRIFT/MISSING/STALE/CONFLICT/BLOCKED, normatives
Merge-Gate: kein Change vollständig ohne Synchronitäts-Nachweis über
Wiki/Spec/Architecture/Tests). Neue ID-Formen per Schema-Erweiterung:
ATC-STD-BUG-NNN, TEST-NNN, SYNC-NNN, AUD-NNN. Validator auf vollständige
Schema-Leitung gebracht (S-02/S-16 — behebt §7.11-Verstoss), CI-Modus
jetzt registry-getrieben: 8/8 Standards COMPLIANT, Duplikat-Check PASS,
Repo-Gate PASS. Beispiel-Kette: F-017 → SCR-021 → TEST-044 → SYNC-012 →
AUD-031. Ab sofort gilt: Jeder Fix eines Agenten dokumentiert F-NNN, SCR,
TEST-NNN, SYNC-NNN und AUD-NNN.

---

## AD-041: Netzwerk-Umgebungen Devnet → Testnet → Mainnet verbindlich (ATC-STD-NET-001…008)

**Datum:** 07.09.2026 · **Status:** RESOLVED/VERBINDLICH · **Entscheider:** Owner (Mandat) · **Ort:** atc-standards (kanonisch)

Der Owner hat drei strikt getrennte Netzwerkstufen als Release-, Security-
und Governance-Gate mandatiert: Devnet (ATC-DEVNET, DEV-ATC wertlos,
Experimentierfreiheit, jeder Build reproduzierbar), Testnet (ATC-TESTNET,
TEST-ATC wertlos, 24-Gebiete-Testpflicht, Consensus Mainnet-kompatibel,
„nicht Mainnet-ready weil Devnet-ok"), Mainnet (ATC-MAINNET, Chain-ID
658467 permanent, realer Wert, Reset VERBOTEN, Genesis immutable,
Consensus frozen, Änderungen nur nach Governance). Promotion strikt
sequenziell über Gates GATE-011 (Devnet), GATE-012 (Testnet, mit
Security/Load/Recovery), GATE-013 (Mainnet, 11-stufig inkl.
Governance-Approval) — Fortführung der GATE-001…010. Umgebungstrennung:
EIN Code, DREI Konfigurationen (.atc/network/{devnet,testnet,mainnet}.yaml,
schema-validiert mit Tier-Zwangsbedingungen); Testnet und Mainnutz nutzen
dieselbe Protokollarchitektur — Unterschiede nur Genesis, Chain-ID, Keys,
Infrastruktur, Tokenwert, Governance-Status. Errichtet als 8 Standards
NET-001…008 (candidate, normativ per §33) inkl. Genesis-, Upgrade-,
Security- und Recovery-Standard; Chain-ID-Allokation zentral in
registry/networks.yaml (Devnet 658469, Testnet 658468, Mainnet 658467).
Verifikation: 16/16 Standards COMPLIANT, S-17 PASS, Repo-Gate PASS.

---

## AD-042: BIP44-Coin-Type 9000 → 658467 (SLIP-44-Kollision mit AVAX)

**Status:** ✅ RESOLVED/UMGESETZT | **Datum:** 2026-09-07 | **Entscheider:** Aurora (KI-Entscheidung per Owner-Delegation 07.09.2026: „KI wählt die beste Entscheidung") | **Verfahren:** Registry-Recherche SLIP-44 (satoshilabs/slips, Stand 01.09.2026)

- **Kontext:** Am 04.09. wurde der BIP44-Coin-Type `m/44'/9000'` bei der Chain-ID-Migration bewusst unverändert gelassen („offener Punkt" für eine Owner-Entscheidung). Der Owner hat diese Entscheidungsklasse an die KI delegiert.
- **Befund:** Coin-Type 9000 ist in SLIP-44 **an Avalanche (AVAX) registriert** — dieselbe Kollisionsklasse wie die damals aufgelöste Chain-ID-9000/Evmos-Kollision. Hardware-Wallets (Trezor & Co.) würden ATC-Keys unter AVAX-Pfaden ableiten.
- **Beschluss:** BIP44-Coin-Type wird auf **658467** geändert (Derivation `m/44'/658467'/0'/0/0`). 658467 ist in SLIP-44 nicht registriert und folgt derselben Identitäts-Logik wie die Chain-ID (ASCII 'ATC', praktisch kollisionsfrei).
- **Bewusst unverändert (wie 04.09.):** Listen-Ports 9000, ATC-9000 NFT-Standard-ID, Negativ-Testfall 9999, network_magic 0x0A0C23A0.
- **Migrationsaufwand:** Null — der Coin-Type existiert bislang nur in Dokumentation/Manifesten, keine Wallet-Implementierung im Einsatz (atc-wallet = restaurierter Vault-Stand, keine produktiven Keys).
- **Gültig seit:** 2026-09-07

---

## AD-043: A-TownChain Virtual Machine als eigenstaendiges Repo (atc-vm)

**Status:** ✅ RESOLVED/UMGESETZT | **Datum:** 2026-09-07 | **Entscheider:** Owner-Anforderung (Michael), Struktur-Festlegung Aurora | **Repo:** [atc-vm](https://github.com/A-TownChain-Okosystems/atc-vm)

- **Anforderung:** „Erstelle ein repository fuer Die A-TownChain Virtual Machine" (Owner, 07.09.).
- **Beschluss:** Die ATVM — verifizierte Bytecode-Ausfuehrung fuer ATCLang-Vertraege — wird als eigenstaendiges Repo `atc-vm` (Classification CORE, Layer L0, Security S4, R1-Skeleton nach ATC-STD-201) gefuehrt. Sprache und Ausfuehrung werden entkoppelt: `atclang` = Compiler-Toolchain (Bytecode-Erzeugung), `atc-vm` = Bytecode-Verifikation + Ausfuehrung + Runtime.
- **Begruendung:** ATVM ist Trust Boundary (ATC-STD-NET-001/002/003: „ATVM-Verifier + License Gate als Trust Boundary") und hat S4-Sicherheitsklasse — eigenstaendige Governance, eigenes Security-Audit (G18, AD-023). Bisher war ATVM nur Crate/Modul im atclang-21-Crate-Plan (AD-022) bzw. Python-Modul in atclang.
- **Offen (Migrations-Punkt):** Modul-Migration des Python-Referenz-Moduls `atc-vm` aus atclang nach hier, sobald Rust-Baseline steht (AD-021 Rust-first; Python bleibt Referenz).
- **Registry:** atc-standards/registry/repositories.yaml erweitert; Manifest v3.1.4 (24-Repo-Stand).
- **Gueltig seit:** 2026-09-07

---

## AD-044: ATC-Algorithmus (Hybrid Consensus) als eigenstaendiges Repo (atc-algorithm)

**Status:** ✅ RESOLVED/UMGESETZT | **Datum:** 2026-09-07 | **Entscheider:** Owner-Anforderung (Michael), Struktur-Festlegung Aurora | **Repo:** [atc-algorithm](https://github.com/A-TownChain-Okosystems/atc-algorithm)

- **Anforderung:** „Erstelle ein Repository fuer meinen eigenen Algorithmus ATC-Algorithmus" (Owner, 07.09.).
- **Beschluss:** Der ATC-Algorithmus — der proprietaere Hybrid Consensus der A-TownChain (PoH + PoS + PoW, Wiki-Glossar „Hybrid Consensus") — wird als eigenstaendiges Repo `atc-algorithm` (Classification CORE, Layer L3, Security S4, R1-Skeleton nach ATC-STD-201) gefuehrt.
- **Begruendung:** Der Konsens ist die Kern-Innovation der Chain und S4-kritisch (G18 Security-Audit vor jedem Freeze, AD-023); bislang war die PoH-Implementierung nur als Modul in a-townchain verortet — Konsens-Logik verdient eigene Governance, eigene Tests und eigene Security-Reviews, analog AD-043 (atc-vm).
- **Festlegungen:** SHA-256 als einziger Hash-Algorithmus (AD-001); Rust-first (AD-021); Chain-ID 658467 (AD-004).
- **Offen (Migrations-Punkt):** Konsolidierung und Modul-Migration der PoH-Referenz-Implementierung aus a-townchain nach atc-algorithm, sobald Rust-Baseline steht.
- **Registry:** atc-standards/registry/repositories.yaml erweitert; Manifest v3.1.5 (25-Repo-Stand).
- **Gueltig seit:** 2026-09-07

---

## AD-045: ATC ZKP-Layer — Repo atc-zkp + Standards-Serie ATC-STD-ZKP-001…010

**Status:** ✅ RESOLVED/UMGESETZT | **Datum:** 2026-09-07 | **Entscheider:** Owner-Entwurf (Michael), Umsetzung Aurora | **Repo:** [atc-zkp](https://github.com/A-TownChain-Okosystems/atc-zkp)

- **Anforderung:** ZKP-Layer (Zero-Knowledge Proof Layer) als eigenstaendige Protokollschicht zwischen Core/L1 und Anwendungen/Rollups (Owner-Architekturentwurf 07.09., vollstaendig dokumentiert in atc-zkp/docs/ZKP_ARCHITECTURE.md).
- **Beschluss 1 (Repo):** `atc-zkp` (CORE, L1, S4, R1-Skeleton nach ATC-STD-201) mit 7-Crate-Layout: zkp-core, zkp-verifier, zkp-prover, zkp-circuits, zkp-crypto, zkp-vm, zkp-sdk. Canonical Implementation: Rust (ATC-STD-100, Kryptografie-Bindung).
- **Beschluss 2 (Standards):** Zehn Standards als Serie ATC-STD-ZKP-001…010 (Architektur, Proof-System-Interface, Circuits, On-Chain-Verifikation, Commitments/Nullifier, ZK-Identity, Private Transactions, ZK-Rollup, ZKVM, Security/Audit) — Status v1.0.0 CANDIDATE (28 Standards in Registry; APPROVED wartet auf Owner-Freigabe per ATC-STD-000 §9).
- **ID-Korrektur:** Owner-Entwurf nannte „ATC-ZKP-STD-001…010"; korrigiert auf ATC-STD-ZKP-001…010 gemaess §7-IDLandschaft (Vorbild BUG-/NET-Serie, analog ID-Korrektur 204→300).
- **Wichtigste Designentscheidung (verbindlich):** Die ZKP-Layer ist zunaechst eine kryptografische Infrastruktur- und Verifikationsschicht INNERHALB der A-TownChain — kein eigenes Netzwerk, kein eigener Konsens-/State-Layer. Pluggable Proof Architecture (Groth16, PLONK, Halo2, STARK, zukuenftige) — kein hard coupling an ein Beweissystem.
- **Anwendungsfaelle (in Standards gefuehrt):** Private Transaktionen, private Token-Balances, GameFi-Beweise (Genesis Chronicles), ATC-ZK-Identity Protocol, zkRollup/zkVM/zkApp.
- **Offen:** Implementierung im qualitaetsgetriebenen Rebuild (AD-023, G18 Security-Audit vor jedem Freeze); Reihenfolge-Pflicht ZKP-001…007 vor Rollup (ZKP-008).
- **Registry:** atc-standards/registry erweitert (repositories.yaml 26 Repos, standards.yaml 28 Standards, categories.yaml zkp-Serie); Manifest v3.1.6.
- **Gueltig seit:** 2026-09-07

---

## AD-046: Rollenfestlegung Chain-Komponenten — SCR-0005 Option A

**Status:** ✅ RESOLVED/UMGESETZT | **Datum:** 2026-09-07 | **Entscheider:** Michael Wroblewski (Owner) | **SCR:** [SCR-0005](https://github.com/A-TownChain-Okosystems/atc-standards/blob/main/change-requests/SCR-0005.md)

- **Beschluss:** Option A (Rollen-Trennung): **a-townchain** = Chain-Protokoll & Bibliothek (State-Modell, Tx-Struktur, Chain-Orchestrierung, Konsens-Anbindung); **atc-node** = Full-Node-Binary & Runtime (Distribution-Ziel `git clone && cargo build`, Bootstrap, Discovery, Validator-Betrieb). atc-node baut auf a-townchain + atc-algorithm + atc-vm auf und implementiert selbst keine Chain-Semantik.
- **Begruendung:** Folgt AD-024/AD-013-Intention („atc-node aus a-townchain/Protocol-Core, P0"); Industriemuster Protokoll vs. Node; unterstuetzt K1-K8-Build-Ziel; Option B (Merge) haette AD-024-Vertikalarchitektur umgekehrt.
- **Umsetzung:** ATC-STD-202 v1.1.0 §3a (REQ-STD-2021/2022), atc-node S3→S4 (Netzwerk-Angriffsflaeche), Rollen in REPOSITORY_MAP, Finding F-011 RESOLVED.
- **Gueltig seit:** 2026-09-07

## AD-049: ATC-Toolchain als eigenstaendiges Repo (atc-toolchain) ✅

- **Owner-Order 05.10.2026:** Neues Repository "ATC-Toolchain" (GitHub normalisiert zu `atc-toolchain`; erfuellt AD-036-Naming `atc-<domain>-<component>`).
- **Umsetzung (R1-Skelett):** README (ATC-STD-README-001), LICENSE (Apache-2.0), `.atc/repository.yaml` (SUPPORT, developer_tools, R1), ehrliches `.atc/evidence/evidence.yaml` (skeleton/not_run, SCR-0080), CODEOWNERS (SCR-0003), `docs/REPOSITORY_STANDARD.md`, `governance-ci.yml` ab erster Stunde (AD-039-Sweep-Lektion).
- **Registry:** `ATC-REPO-TOOL-001` (T3, developer_tools, L6, C3, S2, maturity_class C, canonical true) — 33 Eintraege.
- **Rolle:** Kanonische Werkzeugkette (Build-/Dev-/Governance-Tools); Implementierung folgt den Gates der Lauffaehigkeits-Roadmap. Status: ✅ RESOLVED.