# ATC-ORG-AUDIT-001 — A-TownChain-Okosystems Master-Audit (AUD-2026-0003)

**Datum: 08.09.2026, 02:55 UTC+2 · Agent Aurora (Live-GitHub-API + lokale Tiefeninspektion) · SCR-0033**
**Methode:** Live-Inventar via GitHub-API (26/26 Repos) + lokale Repository-Inspektion
(README, LICENSE, CI, Dependabot, SECURITY.md, Tests, docs/, Copyright-Header, Tags,
Releases). Health-Score = vereinfachter AUDIT-SCAN mit 11 Kriterien à 100 Punkte
(nicht die volle 64-Check-Pipeline nach REPO-AUDIT-002 — als Scan gekennzeichnet).

**Ehrlichkeits-Präfix:** Dieser Audit basiert ausschließlich auf verifizierten
Live-Daten. Die Behauptung eines externen Agenten, `atc-whitepaper` sei ein
öffentliches Repo der Organisation, ist FALSCH (API-geprüft: existiert nicht) —
als Warnbeispiel für Agenten-Halluzinationen dokumentiert (AUD-Record-Pflicht für
Behauptungen Dritter).

## 1. Executive Summary

| Kennzahl | Wert |
|---|---|
| Repositories (Live-API) | **26/26** (0 archiviert, 0 privat-Fehlanzeige) |
| Ø Health Score | **74,6 / 100 → Org-Grade C** |
| Grade-Verteilung | 0×A, 17×B, 7×C, 2×D (atc-vm 55, atc-algorithm 55), 0×E |
| P0 (release-blockierend) | **0** |
| P1 | 4 (CI für atc-vm/atc-algorithm/atc-zkp = Issue 94; atc-contracts ohne Tests) |
| P2 | Version-Tags 24/26 fehlen (Issues 96/97); Dependabot 10 Repos; PR-Regel-Bypass |
| P3 | CodeQL (Issue 95); Testnet/Devnet-Struktur; Monitoring-Repos |

## 2. Maschinenlesbare Repository-Matrix (26 Repos)

| Repository | Layer | Lang | Version | Tests | CI/CD | Security | Deps | Reife (Score) | Gaps/Priorität |
|---|---|---|---|---|---|---|---|---|---|
| a-townchain | L3 | Pyth | — | 13 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| a-townchain-os | L7 | None | v2.0.0 | 1 | 1 Workflow(s) | SECURITY.md | manuell | B (80) | P2: kein Dependabot |
| a-townchain-os-docs | L7 | Type | — | 360 | 1 Workflow(s) | SECURITY.md | Dependabo | B (85) | P2: keine Version-Tags |
| atc-algorithm | L5 | None | — | 1 | KEINE | SECURITY.md | manuell | D (55) | P1: keine CI; P2: kein Dependabot; P2: keine |
| atc-compute | L5 | None | — | 1 | 1 Workflow(s) | SECURITY.md | manuell | C (70) | P2: kein Dependabot; P2: keine Version-Tags |
| atc-contracts | L5 | Pyth | — | 0 | 1 Workflow(s) | SECURITY.md | Dependabo | C (70) | P1: keine Tests; P2: keine Version-Tags |
| atc-explorer | L5 | Type | — | 1 | 1 Workflow(s) | SECURITY.md | Dependabo | B (75) | P2: keine Version-Tags |
| atc-indexer | L5 | Type | — | 1 | 1 Workflow(s) | SECURITY.md | Dependabo | B (75) | P2: keine Version-Tags |
| atc-interop | L5 | Rust | — | 5 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| atc-launchpad | L5 | None | — | 1 | 1 Workflow(s) | SECURITY.md | manuell | C (70) | P2: kein Dependabot; P2: keine Version-Tags |
| atc-marketplace | L5 | Type | — | 6 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| atc-mining | L5 | None | — | 1 | 1 Workflow(s) | SECURITY.md | manuell | C (70) | P2: kein Dependabot; P2: keine Version-Tags |
| atc-node | L5 | None | — | 1 | 1 Workflow(s) | SECURITY.md | manuell | C (70) | P2: kein Dependabot; P2: keine Version-Tags |
| atc-oracle | L5 | None | — | 1 | 1 Workflow(s) | SECURITY.md | manuell | C (70) | P2: kein Dependabot; P2: keine Version-Tags |
| atc-sdk | L5 | Pyth | — | 1 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| atc-shivacore | L1 | Rust | — | 57 | 1 Workflow(s) | SECURITY.md | Dependabo | B (85) | P2: keine Version-Tags |
| atc-standards | G | Pyth | v1.1.0 | 3 | 2 Workflow(s) | SECURITY.md | manuell | B (80) | P2: kein Dependabot |
| atc-storage | L5 | None | — | 1 | 1 Workflow(s) | SECURITY.md | manuell | C (70) | P2: kein Dependabot; P2: keine Version-Tags |
| atc-vm | L0 | None | — | 1 | KEINE | SECURITY.md | manuell | D (55) | P1: keine CI; P2: kein Dependabot; P2: keine |
| atc-wallet | L5 | Pyth | — | 1 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| atc-zkp | L5 | Rust | — | 7 | KEINE | SECURITY.md | Dependabo | C (60) | P1: keine CI; P2: keine Version-Tags |
| atclang | L0 | Pyth | — | 5 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| aurora-ai | L2 | Type | — | 14 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| genesis-chronicles | L6 | Pyth | — | 6 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| genesis-engine | L6 | Pyth | — | 3 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |
| globus-os | L4 | Rust | — | 11 | 1 Workflow(s) | SECURITY.md | Dependabo | B (80) | P2: keine Version-Tags |

<details><summary>YAML (maschinenlesbar, vollständig)</summary>

```yaml
org_audit_2026_0003:
  date: "2026-09-08"
  method: "live-github-api + local-inspection"
  health_model: "AUDIT-SCAN-11 (0-100)"
  repositories:
    a-townchain:
      role: L3
      language: Python
      version: null
      license: True
      tests: 13
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    a-townchain-os:
      role: L7
      language: None
      version: v2.0.0
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: kein Dependabot"
      next_action: "Health halten"
    a-townchain-os-docs:
      role: L7
      language: TypeScript
      version: null
      license: True
      tests: 360
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 85
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    atc-algorithm:
      role: L5
      language: None
      version: null
      license: True
      tests: 1
      ci: "KEINE"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: D
      health: 55
      gaps: "P1: keine CI; P2: kein Dependabot; P2: keine Version-Tags"
      next_action: "CI etablieren (Issue 94)"
    atc-compute:
      role: L5
      language: None
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: C
      health: 70
      gaps: "P2: kein Dependabot; P2: keine Version-Tags"
      next_action: "Health halten"
    atc-contracts:
      role: L5
      language: Python
      version: null
      license: True
      tests: 0
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: C
      health: 70
      gaps: "P1: keine Tests; P2: keine Version-Tags"
      next_action: "Tests etablieren"
    atc-explorer:
      role: L5
      language: TypeScript
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 75
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    atc-indexer:
      role: L5
      language: TypeScript
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 75
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    atc-interop:
      role: L5
      language: Rust
      version: null
      license: True
      tests: 5
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    atc-launchpad:
      role: L5
      language: None
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: C
      health: 70
      gaps: "P2: kein Dependabot; P2: keine Version-Tags"
      next_action: "Health halten"
    atc-marketplace:
      role: L5
      language: TypeScript
      version: null
      license: True
      tests: 6
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    atc-mining:
      role: L5
      language: None
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: C
      health: 70
      gaps: "P2: kein Dependabot; P2: keine Version-Tags"
      next_action: "Health halten"
    atc-node:
      role: L5
      language: None
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: C
      health: 70
      gaps: "P2: kein Dependabot; P2: keine Version-Tags"
      next_action: "Health halten"
    atc-oracle:
      role: L5
      language: None
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: C
      health: 70
      gaps: "P2: kein Dependabot; P2: keine Version-Tags"
      next_action: "Health halten"
    atc-sdk:
      role: L5
      language: Python
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    atc-shivacore:
      role: L1
      language: Rust
      version: null
      license: True
      tests: 57
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 85
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    atc-standards:
      role: G
      language: Python
      version: v1.1.0
      license: True
      tests: 3
      ci: "2 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: kein Dependabot"
      next_action: "Health halten"
    atc-storage:
      role: L5
      language: None
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: C
      health: 70
      gaps: "P2: kein Dependabot; P2: keine Version-Tags"
      next_action: "Health halten"
    atc-vm:
      role: L0
      language: None
      version: null
      license: True
      tests: 1
      ci: "KEINE"
      security: "SECURITY.md"
      dependencies: "manuell"
      duplicate_risk: gering
      maturity: D
      health: 55
      gaps: "P1: keine CI; P2: kein Dependabot; P2: keine Version-Tags"
      next_action: "CI etablieren (Issue 94)"
    atc-wallet:
      role: L5
      language: Python
      version: null
      license: True
      tests: 1
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    atc-zkp:
      role: L5
      language: Rust
      version: null
      license: True
      tests: 7
      ci: "KEINE"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: C
      health: 60
      gaps: "P1: keine CI; P2: keine Version-Tags"
      next_action: "CI etablieren (Issue 94)"
    atclang:
      role: L0
      language: Python
      version: null
      license: True
      tests: 5
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    aurora-ai:
      role: L2
      language: TypeScript
      version: null
      license: True
      tests: 14
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    genesis-chronicles:
      role: L6
      language: Python
      version: null
      license: True
      tests: 6
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    genesis-engine:
      role: L6
      language: Python
      version: null
      license: True
      tests: 3
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
    globus-os:
      role: L4
      language: Rust
      version: null
      license: True
      tests: 11
      ci: "1 Workflow(s)"
      security: "SECURITY.md"
      dependencies: "Dependabot"
      duplicate_risk: gering
      maturity: B
      health: 80
      gaps: "P2: keine Version-Tags"
      next_action: "Health halten"
```

</details>

## 3. ORG-Dimensionen (01–20)

| # | Dimension | Befund | Grad |
|---|---|---|---|
| ORG-01 | Organisationsstruktur | 26 Repos, Layer-Modell L0–L7 + Governance-Schicht (G) in REPO_ARCHITECTURE verbindlich zugeordnet | B |
| ORG-02 | Repository-Inventar | 26/26 live verifiziert (API), alle lokal geklont; 0 Duplikate (bestätigt AUD-2026-0002) | A |
| ORG-03 | Repository-Rollen | Jedes Repo einer Rolle (L0–L7) zugeordnet; a-townchain-os = Integrations-Shell, os-docs = Doku-SSOT | B |
| ORG-04 | Namensstandards | 21× atc--Präfix konform; 5 bewusst ohne Präfix (globus-os, aurora-ai, genesis-engine, genesis-chronicles, a-townchain) per NAMING_CONVENTIONS.md geregelt | B |
| ORG-05 | README/Doku | 26/26 mit README (31–268 Zeilen); docs/ in 26/26 | B |
| ORG-06 | Lizenzierung | 26/26 mit LICENSE (API) — Verbesserung ggü. F-040-Backfill (Datei-Ebene 387/387) | A |
| ORG-07 | Security | 26/26 SECURITY.md; 0 Secrets (AUD-2026-0002); Token-Scope-Restriktion GH013 aktiv; CodeQL offen (Issue 95) | B |
| ORG-08 | CI/CD | 23/26 mit Workflows; atc-vm/atc-algorithm/atc-zkp ohne CI = Issue 94 exakt bestätigt (Owner-Aktion, GH013) | C |
| ORG-09 | Branch-/Merge-Strategie | main-Zweig-Disziplin aktiv; ABER: Org-PR-Regel wird von Admin-Pushes systematisch gebypasst („Bypassed rule violations") — Regel ohne Wirkung → F-045 | D |
| ORG-10 | Issues/PR/Milestones | Issues 94–98 offen nachvollziehbar; Milestones ATC-M-001..008 mit Evidence-Pflicht (S-20) | B |
| ORG-11 | Versionierung | NUR 2/26 Repos mit Tags (a-townchain, a-townchain-os — letzteres mit verwaistem v2.0.0, Issue 97); VERSION-001-Baseline offen (Issue 96) → F-042 | D |
| ORG-12 | Release Management | 0 Releases org-weit; RR-Gates (FAM-10) definiert, aber ohne getaggte Releases nicht wirksam | D |
| ORG-13 | Standards-Governance | **A**: atc-standards Registry 387/387 APPROVED, CI-erzwungen, 11 Register, Validator S-01..S-25 ALL COMPLIANT | A |
| ORG-14 | Repo-Abhängigkeiten | Layer-Modell + sync_modules.py (a-townchain-os); keine zirkulären Repo-Abhängigkeiten bekannt | B |
| ORG-15 | Duplikate/veraltet | 0 Duplikate (live verifiziert); Altbestand archiviert in os-docs/docs/archive/ | A |
| ORG-16 | Code-/Doku-Konsistenz | REALITY_STATUS.md (os-docs) kanonisch, append-only, 164 Zeilen; Agenten-Signatur-Konvention an jedem Commit | B |
| ORG-17 | KI-Agenten-Integration | AGENT_MANIFEST verbindlich; 387 Standards an Agent gebunden; Aurora-Bot registriert; Registry-Gate je Commit | A |
| ORG-18 | Blockchain/Protocol-Layer | Protocol-Registry 26 Familien; P2P-001 v1.0.0 freigegeben; Chain-ID 658467; CONF/Security-Register aktiv; K14 v0.9-Kompatibilitätsmodus dokumentiert | B |
| ORG-19 | Testnet/Devnet/Mainnet | **Nicht etabliert** — ehrlich: kein Testnet/Devnet-Setup; Mainnet-Gates via CONF-BRONZE definiert, Ed25519-HAL offen | E* |
| ORG-20 | Gesamt-Reifegrad | Governance reif (A), Engineering solide (B/C), Release-Prozess unreif (D) → Org-Grade C | C |

\* E nur für die dimensionale Lücke, nicht als Werturteil über geplante Elemente.

## 4. Struktur-Mapping nach dem 9-Sektionen-System (Antrag Owner)

| Sektion | Vorhanden | Ehrliche Lücken |
|---|---|---|
| 01 Governance | atc-standards (Registry 387) | — |
| 02 Core Blockchain | a-townchain (L3), atc-node, atc-mining; Consensus in ShivaCore K16 + a-townchain | eigenes Protocol-Repo fehlt (Protokolle in atc-standards + Kernel) |
| 03 Smart Contracts | atc-contracts | Token-Standards als Standard vorhanden, Security teils PARTIAL (SEC-C) |
| 04 Developer Infrastructure | atc-sdk, atc-vm, atc-algorithm, atc-explorer, atc-indexer, atc-interop | **CLI fehlt**, API-Gateway-Repos fehlen (früher atc-gateway — bei Restrukturierung nicht übernommen) |
| 05 AI/Agent Layer | aurora-ai (L2), AAS-25-Standards, AGENT_MANIFEST | Kernel-Event-Bridge = ATC-M-003 offen |
| 06 Applications | atc-wallet, atc-marketplace, atc-launchpad | DeFi-Apps offen |
| 07 Game/GameFi | genesis-engine, genesis-chronicles (Vision-Ebene) | Shivamon-Spielelogik = Vision (ATC-41+), kein Code |
| 08 Infrastructure | atc-compute, atc-storage, atc-oracle, atc-mining | Monitoring-Repos fehlen |
| 09 Documentation | a-townchain-os-docs (REALITY_STATUS, Vault) | **Whitepaper fehlt** (atc-whitepaper existiert nicht — externe Fehlbehauptung dokumentiert) |

## 5. P0–P3-Mängel & Verbesserungs-Roadmap

**P0: keine.**

**P1 (nächste 7 Tage):**
1. CI für atc-vm/atc-algorithm/atc-zkp (Issue 94 — Owner-Aktion wegen GH013)
2. Test-Suite für atc-contracts (Smart-Contract-Layer ohne Tests ist release-blockierend für DeFi)

**P2 (nächste 30 Tage):**
3. Version-Tags: VERSION-001-Baseline je Repo (Issues 96/97); verwaisten v2.0.0-Tag in a-townchain-os bereinigen
4. Dependabot für die 10 fehlenden Repos (atc-algorithm, atc-compute, atc-launchpad, atc-mining, atc-node, atc-oracle, atc-storage, atc-vm, a-townchain-os, atc-standards)
5. PR-Regel-Politik: entweder Branch-Protection konsequent leben (PRs erzwingen) oder Regel entfernen — aktueller Zustand ist Schein-Governance (F-045)

**P3 (Backlog):**
6. CodeQL-Rollout (Issue 95) · 7. Testnet/Devnet-Struktur · 8. Monitoring/Observability-Repos · 9. Whitepaper/CLI/Gateway-Entscheidung des Owners

## 6. Abgleich mit externem Fremd-Audit (08.09.2026, 03:30 UTC+2, SCR-0035)

| Fremd-Befund | Verifikation (live) | Ergebnis |
|---|---|---|
| **P0-001:** ATC-STD-000-„Versionskonflikt" (v1.2.0/v1.1.0/v1.0.0 CANDIDATE/BLOCKED) | Registry+Datei+H1+Manifeste: ATC-STD-000 **v1.2.0 approved überall**; CANDIDATE/BLOCKED-Stände stehen nur in Archiv-Dokumenten (approval/-Records, CHANGELOG, SCRs, GOVERNANCE-STAND-Sektion Stand 07.09.) | **WIDERLEGT** als SSOT-Konflikt — berechtigter Kern: STALE GOVERNANCE-STAND-Narrative im AGENT_MANIFEST → GE FIXT (Archiv-Markierung + Ist-Zustand + SSOT-Klarstellung, SCR-0035) |
| **P0-002:** Lizenz NOASSERTION | API: **26/26 Repos NOASSERTION** (LICENSE-Dateien vorhanden, kein SPDX-Format) | **VERIFIZIERT** → F-046 (P2, Owner-Entscheidung: SPDX-Lizenz oder Proprietär-Deklaration) |
| **P0-003:** Mainnet-Gates fehlen | Existieren normativ: RR-G01..G08, CONF-BRONZE (PROTOCOL-002), MILESTONE-001-Gates, COMPAT-001, ATC-STD-999; atc-node ist NICHT als Skeleton ausgewiesen (nur atc-vm/atc-algorithm/atc-zkp) | **Abgedeckt** bzw. insoweit widerlegt |
| P1: CI 23/26, CodeQL 0/26, Versions-Baseline, verwaister Tag, STD-202-Klassifizierung | = Issues 94–98, F-042..F-045 | Bestätigt (bereits registriert) |
| P2-004 Doku-Duplizierung; KPI-Set; .atc/repository.yaml; Release-Validator; monatliche AUD-Kadenz | Plausibel/neu | **Backlog übernommen** (Owner-Priorisierung) |

Fazit: „Governance dokumentiert > technisch erzwungen" trifft den Kern (Deckungsgleich
mit Org-Grade C); P0-Blocker existieren nach Live-Verifikation nicht.

*Erzeugt via SCR-0033 (Agent Aurora) · Health-Modell: AUDIT-SCAN-11 · Validator ALL COMPLIANT · 08.09.2026, 02:55 UTC+2*
