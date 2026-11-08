# AUD-2026-0002 — ATC Enterprise GitHub Organization Audit

| Feld | Wert |
|---|---|
| Audit-Run-ID | AUD-2026-0002 |
| Typ | Organization Audit (18 Prüfbereiche, ATC-STD-AUDIT-001 Rahmen) |
| Scope | GitHub-Organisation A-TownChain-Okosystems — alle 26 Repositories + Org-Settings |
| Datum | 07.09.2026, 23:00–23:20 UTC+2 |
| Auditor | Aurora (Superagent, Builder-Chat) — Live-API-Zugriff + vollständige lokale Klone |
| Auftrag | Owner Michael Wroblewski (Builder-Chat 07.09.2026, 23:09) |
| Ergebnis | CONDITIONAL_PASS — Governance reif, Security-Abdeckung lückenhaft, Lauffähigkeit M2/8 |
| Findings | F-024…F-031 (1 × S2 offen-beauftragt, 3 × S2/S3 behebbar, 4 × Info/erledigt) |

## 1. Zusammenfassung

Die Organisation ist nach der Restrukturierung vom 06.–07.09.2026 (26 Repos, Layer L0–L7, AD-024/026) in einem strukturell sauberen Zustand: Governance-Hygiene 26/26, keine Secrets, keine Duplikate, 1 offenes Issue, 0 offene PRs, single-branch überall. Die Sicherheitsautomatisierung (Dependabot/CodeQL) fehlte komplett und wird in diesem Audit-Lauf nachgerüstt (16 Repos sofort, Rest via Issues). Die Org-Level-Settings-Prüfung wurde per Owner-Entscheidung (07.09., 23:13, Builder-Chat) übersprungen und dokumentiert (F-024). Die externe Referenz „atc-whitepaper" existiert nicht (F-030).

## 2. Prüfbereiche (18)

| # | Bereich | Status | Ergebnis |
|---|---|---|---|
| 01 | Organisation | ⚠️ SKIP | Org-Settings per Owner-Entscheidung übersprungen (F-024): Default-Repo-Permission `admin`, 2FA-Enforcement aus, keine Beschreibung — Owner-Aktion, nicht Agent-behebbar |
| 02 | Repository-Inventar | ✅ PASS | 26/26 Repos per API verifiziert (public, kein Fork, kein Archiviertes), alle mit Zweck/Layer in Registry + Beschreibung |
| 03 | Architektur | ✅ PASS | L0–L7 sauber zugeordnet (ATC-STD-202 v1.1.0, AD-026), Abhängigkeitsrichtungen dokumentiert (a-townchain-os → alle; atc-node → a-townchain+atc-algorithm+atc-vm) |
| 04 | Naming | ✅ PASS | ATC-STD-202 v1.1.0 konform: atc-*-Familie + 9 eigenständige Produktlinien (atclang, globus-os, aurora-ai, genesis-engine, genesis-chronicles, a-townchain, atc-shivacore, a-townchain-os, a-townchain-os-docs); keine verbotenen Namen |
| 05 | README | ✅ PASS | 26/26 README.md vorhanden (26–265 Zeilen); README-Standard ATC-STD-README-001 approved |
| 06 | Standards | ✅ PASS | 110 Standards, 109 APPROVED + 1 DRAFT (UPDATE-001, §9 ausstehend); Validator ALL COMPLIANT; Registry SSOT standards.yaml |
| 07 | Dokumentation | ✅ PASS | Docs-Hub a-townchain-os-docs mit Vault (repos-rescue + monorepo-full, AD-018/020), Wiki 15.933 Zeilen (docs/kai-os-wiki.md), DECISIONS_REGISTER AD-001..AD-046 |
| 08 | Roadmap | ✅ PASS | Lauffähigkeits-Roadmap M1–M8 (AD-027, verbindlich): M1 ✅ (G1+G2), M2 ✅ (674/674 + Boot L0–L10), M3 (KI) als nächstes |
| 09 | Codequalität | ⚠️ WARN | atc-shivacore: 68 .rs, 2.002 Testfunktionen (674/674 M2-Gate verifiziert), Cargo-Workspace; atclang 23k Python + Tests; 13 L5-Skeletons ohne ausführbaren Code (per M6-Reihenfolge beabsichtigt) |
| 10 | CI/CD | ⚠️ WARN | 23/26 governance-ci.yml (Repository Governance ATC-STD-201/202/203) + atc-standards: ATC Governance CI + Naming-Governance; FEHLT: atc-vm, atc-algorithm, atc-zkp (F-029, blockiert durch Workflow-Scope F-010); kein Build/Test-CI für Code-Repos |
| 11 | Security | ⚠️ WARN | 26/26 SECURITY.md + CODEOWNERS; Secret-Scan über ~7.200 Dateien: 0 Funde; Dependabot war 0/26 → in diesem Audit-Lauf 16 Manifest-Repos ausgestattet (F-025 FIXED); CodeQL 0/26 (F-025, offen → Issue); Org-Settings übersprungen (F-024) |
| 12 | Governance | ✅ PASS | 1 offenes Issue (#80 AIP-001), 0 offene PRs, main-only 26/26, AGENT_MANIFEST.md 26/26, Commit-Konventionen per CI erzwungen |
| 13 | Versionierung | ⚠️ WARN | Inkonsistente Manifest-Versionen (0.1.0 × 5, 1.0.0 atc-contracts, Rest unversioniert — F-026); Releases nur a-townchain-os (v1.0.0) + atc-standards (v1.1.0); a-townchain-os verwaister Tag v2.0.0 (F-027) |
| 14 | KI-Agenten | ✅ PASS | AGENT_MANIFEST.md 26/26, AAS-001..025 approved, AI-DECISION-001 approved, Aurora-Bot registriert (SIGN-Off per AAS-017 bei normativen Änderungen — heute 2× praktiziert) |
| 15 | Konsistenz | ✅ PASS | Chain-ID 658467 konsistent über alle Repos (391+ Referenzen), Copyright „Michael Wroblewski" 26/26, Layer-Angaben Registry ↔ README konform; Ausnahme: ATC-STD-202 zählt 22 statt 26 (F-028) |
| 16 | Redundanz | ✅ PASS | 0 DUPLICATE-Repos; Kernel-Subsysteme korrekt als Primitive (ai.rs 75 Z., vm.rs 54 Z., contract.rs 38 Z., AD-012); Vault ist dokumentierte Archivierung (AD-018/020), keine Redundanz; Dual-Implementierung Blockchain (service_space Rust vs. a-townchain ATCLang) per Design (AD-012/021) — Drift-Watch via M4-Gate |
| 17 | Lauffähigkeit | ⚠️ PARTIAL | Läuft: atclang (compile_source, 20/20) + atc-shivacore (M2-Gate). Nicht lauffähig: L2/L3/L4 (M3–M5 ausstehend), 10 L5-Skeletons, genesis-engine. 2/8 Meilensteine |
| 18 | Enterprise Readiness | ⚠️ EARLY | Governance 9/10 (Standards, Registry, CI, Audit-Trail), Runtime 2/8 (M1/M2), Security-Automatisierung unvollständig, keine Releases/Tags-Disziplin org-weit |

## 3. Repository-Klassifizierung (26)

| Repo | Layer | Klassifizierung | Begründung |
|---|---|---|---|
| atc-shivacore | L1 | **ACTIVE** | M2-Gate erfüllt (674/674, Boot L0–L10, AD-028 verifiziert) |
| atc-standards | parallel | **ACTIVE** | 110 Standards, Validator, CI, heute 5× normative Freigaben |
| a-townchain-os | L7 | **ACTIVE** | Integrations-Shell vollständig (21 Dateien, sync_modules.py) |
| a-townchain-os-docs | Hub | **ACTIVE** | Vault + Wiki + DECISIONS_REGISTER — Source of Truth |
| atclang | L0 | **DEVELOPMENT** | G1+G2 PASSED, nächstes Gate G3 (ATC-IR), Rust-Canonical-Core Phase 2/3 |
| aurora-ai | L2 | **DEVELOPMENT** | 295 Code-Dateien (TS/ATCLang), M3-Kriterium (Kernel-Event-Bridge) offen |
| a-townchain | L3 | **DEVELOPMENT** | Chain-Protokoll vorhanden, M4-Kriterium (2 Nodes Gossip + Tx) offen |
| globus-os | L4 | **DEVELOPMENT** | 12 OS-Module restauriert, M5-Kriterium (globus-init auf ShivaCore) offen |
| genesis-chronicles | L6 | **DEVELOPMENT** | Python-Code + ATCLang, M7-Order #2 |
| atc-sdk / atc-contracts / atc-wallet / atc-interop / atc-explorer / atc-indexer | L5 | **DEVELOPMENT** | Code-Bestand vorhanden, M6-Reihenfolge definiert |
| atc-vm / atc-algorithm | L0/L3 | **EXPERIMENTAL** | Spec-only (AD-043/044), Implementierung folgt in M4/M6 |
| atc-zkp | L1 | **EXPERIMENTAL** | Spec + Cargo-Skelett (AD-045), ZKP-001..010 approved |
| genesis-engine | L6 | **EXPERIMENTAL** | M7-Order #1 (Engine-Loop, ECS offen) |
| atc-node / atc-mining / atc-oracle / atc-storage / atc-launchpad / atc-marketplace / atc-compute | L5 | **EXPERIMENTAL** | Governance-komplett, Code ausstehend (per M6 bewusst) |

**ARCHIVED:** keine · **DUPLICATE:** keine · **UNKNOWN:** keine

## 4. Findings (F-024…F-031)

Vollständige Einträge in `registry/findings.yaml` (Registry-Gate). Kurzfassung:

- **F-024 (S2, SKIP):** Org-Default-Repo-Permission `admin`, 2FA-Enforcement aus — per Owner-Entscheidung 07.09. 23:13 übersprungen (Builder-Chat); dokumentiert, kein Agent-Mandat.
- **F-025 (S2, PARTIAL/FIXED):** Dependabot 0/26, CodeQL 0/26 → Dependabot in 16 Manifest-Repos in diesem Audit-Lauf eingerichtet (Commits je Repo, 07.09.); CodeQL-Rollout offen → Issue #95.
- **F-026 (S3, OPEN):** Manifest-Versionen inkonsistent → VERSION-001-Baseline nötig → Issue #96.
- **F-027 (S3, OPEN):** a-townchain-os verwaister Tag v2.0.0 neben Release v1.0.0 → Issue #97.
- **F-028 (S3, OPEN):** ATC-STD-202 v1.1.0 klassifiziert 22 statt 26 Repos → SCR/Minor-Update → Issue #98.
- **F-029 (S3, PARTIAL):** governance-ci.yml fehlt in atc-vm/atc-algorithm/atc-zkp — Agent-Push an Workflow-Dateien ohne workflow-Scope abgelehnt (s. F-010/GH013, bestätigt in diesem Lauf) → Owner-Aktion → Issue #94.
- **F-030 (S4, RESOLVED-DOKUMENTATION):** Externe Referenz „atc-whitepaper" (ChatGPT-Prüfung 07.09.) existiert nicht in Org noch GitHub-Suche — veraltet/halluziniert; kein Handlungsbedarf.
- **F-031 (S4, POSITIV):** Governance-Hygiene 26/26 (README, CHANGELOG, LICENSE, SECURITY, CODEOWNERS, AGENT_MANIFEST, .atc/repository.yaml); Secret-Scan 0 Funde; keine Duplikate; Chain-ID + Copyright konsistent.

## 5. Cleanup & Standardization Roadmap (priorisiert)

**In diesem Audit-Lauf erledigt (07.09.2026):**
1. ✅ Dependabot-Konfiguration in 16 Repos (alle mit npm/cargo/pip-Manifesten)
2. ✅ Audit-Report AUD-2026-0002 + Findings-Registry + STATUS/CHANGELOG (dieser Commit)
3. ✅ Klassifizierung aller 26 Repos (Abschnitt 3)

**Offene Roadmap (Issues in a-townchain-os):**
1. P1 — governance-ci.yml für atc-vm/atc-algorithm/atc-zkp (Owner-Aktion, workflow-Scope; Issue #94)
2. P2 — CodeQL Code-Scanning für Code-Repos (atc-shivacore, atclang, aurora-ai, a-townchain, atc-contracts, atc-zkp, globus-os, genesis-chronicles) (Issue #95)
3. P2 — Version-Baseline nach ATC-STD-VERSION-001: einheitliche Startversion + Release/Tag-Disziplin für M-Repos (Issue #96)
4. P3 — a-townchain-os Tag v2.0.0 entfernen oder Release nachziehen (Issue #97)
5. P3 — ATC-STD-202 v1.2.0: Klassifizierungstabelle auf 26 Repos (SCR-0017) (Issue #98)
6. ÜBERSPRUNGEN (Owner 07.09. 23:13): Org-Settings Default-Permission/2FA/Beschreibung (F-024) — bei Bedarf durch Owner direkt in GitHub-Org-Settings

## 6. Sign-off

| Prüfung | Ergebnis |
|---|---|
| Gesamtstruktur (Bereiche 01–18) | 9 PASS · 7 WARN/PARTIAL · 1 SKIP · 1 EARLY — keine S0/S1-Blocker |
| Audit-Status | CONDITIONAL_PASS |
| Nächster Org-Audit | nach M3-Gate oder Owner-Auftrag (AUD-2026-0003) |

*AUD-2026-0002 · Aurora (Superagent, Base44) · Auftrag Michael Wroblewski · 07.09.2026 · Registry: registry/findings.yaml F-024…F-031*
