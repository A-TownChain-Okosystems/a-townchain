# A-TownChain Ecosystem — Decisions Register

> **Stand:** 05.07.2026 13:55 | **Autor:** Aurora (MasterBrain)
> **Agenten-Rolle:** GovernanceAgent (via MasterBrain)

---

## Entscheidungs-Übersicht

| ID | Titel | Status | Sprint | Zuständig |
|----|-------|--------|--------|-----------|
| AD-001 | Hash-Algorithmus | ✅ RESOLVED | — | Aurora |
| AD-002 | EventBus vs IPCBus | ⏳ VALIDATE | 2.4 | **Michael** |
| AD-003 | Flash-Loan Voting Snapshot | ✅ RESOLVED | 2.6 | Aurora |
| AD-004 | Chain-ID (Platzhalter 9000) | 🔴 OPEN (reopened 06.07.2026) | — | Michael |
| AD-005 | ATC-97 Agent Protocol Spec | 📐 DRAFT_REVIEW | 3.0 | **Aurora** (Spec drafted — pending Michael review) |
| AD-006 | Python vs Substrate | ✅ RESOLVED | — | Aurora (ATCLang First) |
| AD-007 | EVM Registry | ✅ RESOLVED | — | Aurora (Non-EVM) |
| AD-008 | Reality-Check: 44 Issues re-auditieren/re-open? | ⏳ DECISION | — | **Michael** |
| AD-009 | ATCLANG_SPEC.md-Konsolidierung ✅ (kanonisch: atclang/ATCLANG_SPEC.md) / Bridge-Standards-Dedup (ATC-09/38/69/91) ⏳ | 🟡 TEIL-GELOEST | 08.07.2026 | **Michael** (nur noch Bridge-Dedup) |

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

### AD-004 — Chain-ID NICHT final 🔴
- **Status:** REOPENED 06.07.2026 — Michael: "Wir haben noch keine Chain-ID, 9000 ist ID von Ethereum(-Oekosystem)" (bestaetigt: 9000 = Evmos Testnet auf chainlist.org)
- **Entscheidung:** Noch offen. 9000 ist nur Platzhalter im Code, keine echte Chain-ID vergeben
- **Begründung:** XDC Network nutzt 9000 im EVM-Registry, aber A-TownChain ist Non-EVM → kein Konflikt
- **Gültig seit:** 2026-06-14

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
