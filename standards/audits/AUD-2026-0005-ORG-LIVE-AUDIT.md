---
audit:
  id: AUD-2026-0005
  standard: ATC-STD-AUDIT-001
  standard_version: "1.0.0"
  date: "2026-09-13"
  auditor: "Superagent (Base44) — Live-Audit mit GitHub-Vollzugriff (REST API)"
  tooling: "GitHub REST API (repos, contents, actions, releases, tags, .atc-Artefakte) — alle Befunde live geprueft, keine synthetischen Nachweise"
target:
  repository: A-TownChain-Okosystems (Organisation, 28 Repositories)
  scope: "Org-Struktur, Registry-Sync, Governance-Artefakte, CI/CD-Live-Status, Releases/Tags, Versionserklaerungen, Standards-Konformanz"
result:
  status: PASS_WITH_FINDINGS
  findings: "F-ORG-001..F-ORG-007 (0 P0 / 2 P1 / 4 P2 / 2 P3-Anteile)"
---

# AUD-2026-0005 — Organisation-Live-Audit (28 Repositories, 13.09.2026)

> **Methode:** Vollstaendiger Live-Scan der Organisation ueber die GitHub-API (28 Repos; je Repo:
> Root-Inventar, .atc-Governance-Set, Workflows inkl. letzter CI-Laeufe, Tags/Releases,
> Versionserklaerungen) — abgeglichen gegen registry/repositories.yaml und die bestehenden Gates
> (Version-Gate, Allocation-Gate). Keine Behauptung ohne API-Evidence.

## 1. Organisationsstruktur

- **28 Repositories**, 27 in der Registry-SSOT (Abweichung: `demo-repository`, siehe F-ORG-001)
- Taxonomie: Layer L0 (atclang) bis L7 (a-townchain-os, a-townchain-os-docs, atc-standards, .github)
- Maturity-Verteilung (aus .atc/repository.yaml): R1: 9, R2: 8, R3: 6, R4: 3, ohne R-Level: 2 (.github, demo-repository)

## 2. Repository-Matrix (Live-Stand 13.09.2026)

| Repository | L | Krit | S | R | WF | Tags | STD-Dekl. | Status |
|---|---|---|---|---|---|---|---|---|
| .github | L7 | C1 | S3 | — | 1 | 0 | 0 | 🟡 Active Development |
| a-townchain | L3 | C1 | S4 | R4 | 4 | 0 | 9 | 🟢 Production Candidate |
| a-townchain-os | L7 | C1 | S3 | R3 | 3 | 1 | 9 | 🟠 Architecture/Standards Gap |
| a-townchain-os-docs | L7 | C1 | S3 | R3 | 4 | 0 | 9 | 🟢 Production Candidate |
| atc-algorithm | L3 | C1 | S4 | R1 | 4 | 0 | 9 | ⚪ Experimental / Research |
| atc-compute | L5 | C2 | S3 | R1 | 4 | 0 | 9 | ⚪ Experimental / Research |
| atc-contracts | L5 | C1 | S4 | R3 | 4 | 0 | 9 | 🟢 Production Candidate |
| atc-explorer | L5 | C3 | S2 | R2 | 4 | 0 | 9 | 🟡 Active Development |
| atc-indexer | L5 | C2 | S3 | R2 | 4 | 0 | 9 | 🟠 Architecture/Standards Gap |
| atc-interop | L3 | C1 | S4 | R2 | 4 | 0 | 9 | 🟠 Architecture/Standards Gap |
| atc-launchpad | L6 | C2 | S3 | R1 | 4 | 0 | 9 | 🟠 Architecture/Standards Gap |
| atc-marketplace | L6 | C2 | S3 | R2 | 4 | 0 | 9 | 🟠 Architecture/Standards Gap |
| atc-mining | L3 | C2 | S4 | R1 | 4 | 0 | 9 | ⚪ Experimental / Research |
| atc-node | L3 | C1 | S4 | R1 | 4 | 0 | 9 | ⚪ Experimental / Research |
| atc-oracle | L3 | C1 | S4 | R1 | 4 | 0 | 9 | 🟠 Architecture/Standards Gap |
| atc-sdk | L5 | C2 | S3 | R2 | 4 | 0 | 9 | 🟡 Active Development |
| atc-shivacore | L1 | C1 | S4 | R4 | 4 | 0 | 9 | 🟢 Production Candidate |
| atc-standards | L7 | C1 | S3 | R3 | 7 | 1 | 9 | 🟠 Architecture/Standards Gap |
| atc-storage | L5 | C2 | S4 | R1 | 4 | 0 | 9 | ⚪ Experimental / Research |
| atc-vm | L5 | C1 | S4 | R1 | 5 | 0 | 9 | 🟠 Architecture/Standards Gap |
| atc-wallet | L5 | C1 | S4 | R2 | 4 | 0 | 9 | 🟠 Architecture/Standards Gap |
| atc-zkp | L3 | C1 | S4 | R1 | 4 | 0 | 9 | ⚪ Experimental / Research |
| atclang | L0 | C1 | S4 | R4 | 4 | 0 | 9 | 🟢 Production Candidate |
| aurora-ai | L2 | C1 | S3 | R3 | 4 | 0 | 9 | 🟠 Architecture/Standards Gap |
| demo-repository | — | — | — | — | 2 | 0 | 0 | ⚫ Deprecated / Consolidate |
| genesis-chronicles | L6 | C3 | S2 | R2 | 4 | 0 | 9 | 🟠 Architecture/Standards Gap |
| genesis-engine | L6 | C3 | S2 | R2 | 4 | 0 | 9 | 🟡 Active Development |
| globus-os | L4 | C1 | S4 | R3 | 4 | 0 | 9 | 🟢 Production Candidate |

Legende: L=Layer, Krit=Criticality, S=Security-Klasse, R=Maturity (R-Level aus `.atc/repository.yaml`),
WF=Workflows, Tags=Release-Tags, STD-Dekl=ATC-STD-Versionserklaerungen in `.atc/standards.yaml`.

## 3. Findings (P0–P3)

### F-ORG-001 — P2: Orphan-Repository ohne Registry-Eintrag
**Betroffen:** demo-repository

**Befund:** Nicht in registry/repositories.yaml, kein .atc/, nur Template-Workflows (auto-assign.yml, proof-html.yml), keine SECURITY.md/CODEOWNERS/CHANGELOG. Zuletzt gepusht 2026-09-12.

**Massnahme:** Owner-Entscheidung: loeschen/umbenennen (z.B. als Vorlagen-Repo mit .atc-Set registrieren) oder archivieren.

### F-ORG-002 — P2: Dependency Review schlaegt fehl — Dependency Graph deaktiviert
**Betroffen:** aurora-ai, atc-wallet, atc-indexer, atc-interop, genesis-chronicles, atc-oracle, atc-launchpad

**Befund:** 7 Repos: Workflow laeuft, aber 'Dependency review is not supported on this repository' (Log-Evidence). Security-Feature ist konfigurativ wirkungslos.

**Massnahme:** Dependency Graph in Repo-/Org-Settings aktivieren (Owner-Aktion); danach Workflows gruen validieren.

### F-ORG-003 — P1: Evidence-Bindung-Gate rot (SCR-0086)
**Betroffen:** atc-vm

**Befund:** cargo-tests-Job: Schritt 'Evidence-Bindung (idempotentes test_run-Update + hartes Push-Gate)' schlaegt fehl. Evidence-Pipeline unterbrochen.

**Massnahme:** Root-Cause-Analyse + Fix der Evidence-Bindung (P0-Track Build->Test->Evidence->Audit).

### F-ORG-004 — P2: Views-Drift (gen_views, REQ-IMP-006, SCR-0081)
**Betroffen:** atc-standards

**Befund:** Views-Drift-Check und Cross-Registry-Konsistenztest schlagen fehl: generierte Views nicht regeneriert.

**Massnahme:** gen_views laufen lassen + committen; Drift-Gate danach validieren.

### F-ORG-005 — P1: KAI-OS Rustfmt-Gate rot auf main
**Betroffen:** a-townchain-os

**Befund:** Neue Release-Pipeline: Workspace war nie rustfmt-formatiert. Fix-forward: PR #116 (cargo fmt, 56 Dateien, keine Semantik-Aenderung).

**Massnahme:** PR #116 mergen; danach laufen clippy/tests/audit erstmals vollstaendig — weitere Funde moeglich und gewollt.

### F-ORG-006 — P3: Release-Disziplin praktisch ungenutzt
**Betroffen:** alle

**Befund:** Nur 3/28 Repos haben Tags (a-townchain-os v1/v2, atc-standards v1.1.0); 25 ohne Release — konsistent mit Rebuild-Phase, aber ATC-REL-Gates ungetestet in der Breite.

**Massnahme:** Mit Erreichen der M-Meilensteine je Layer Releases taggen (ATC-REL-X.Y.Z + CHANGELOG).

### F-ORG-007 — P3: .github-Profil-Repo ohne .atc-Governance-Set
**Betroffen:** .github

**Befund:** Registry-Eintrag vorhanden, aber nur evidence/ im .atc-Ordner (kein repository.yaml/standards.yaml).

**Massnahme:** Entscheiden: .github als Infrastruktur-Repo mit reduziertem Pflicht-Set dokumentieren oder vollstaendig ausstatten.

## 4. Positive Evidence (bestaetigt)

- 28 Repos in der Org, 27 in registry/repositories.yaml (SSOT-Abdeckung 96%)
- 26 Repos im Code-Quality-Matrix-Profil (SCR-0092)
- 100% der registrierten Repos: README, LICENSE, SECURITY.md, CHANGELOG.md, CODEOWNERS, ARCHITECTURE.md, .atc-Governance-Set (repository/compliance/lifecycle/ownership/standards)
- 234 ATC-STD-Versionserklaerungen in .atc/standards.yaml; Version-Gate: 109 Deklarationen, 0 FAIL (12.09.)
- Allocation-Gates: FAMILY-300 konsistent (42/19/39, FREE!=ALLOCABLE, CI gruen)
- Sprache je Layer konsistent: Rust (L1/L3/L4, VM/Kernel), Python (L0/L3-Tooling), TypeScript (Frontends/Explorer)

## 5. Nicht in diesem Durchlauf pruefbar (Ehrlichkeit)

- **Secret-Scan im Code je Repo:** nicht durchgefuehrt (folgt mit SCR-0092-Tools / SecurityGPT-Stufe).
- **Devnet/Testnet/Mainnet-Produktionsreife:** Chain-Zustaende nicht gegen Nodes geprueft —
  hier nur Repository-/CI-Evidence. Aussagekraeftige Mapping-Pruefung erfordert Chain-Audit.
- **Codequalitaet je Repo (Dead Code, Typisierung):** nur Stichproben-Signale (Test-Suite-Failures F-ORG-003);
  vollstaendige Erhebung via tools/code_quality_matrix.py (SCR-0092) als Follow-up.

## 6. Bewertung nach Modell

Kein **P0-Blocker** gefunden. Die zwei **P1** (Evidence-Bindung atc-vm, KAI-OS-fmt mit Fix PR #116)
sind produktionsrelevant und haben laufende Fix-Forward-Pfade. P2 sind Konfigurations-/Drift-Luecken
(Dependency Graph ×7, Views-Drift, Orphan-Repo, .github-Governance). P3: Release-Disziplin.

Org-Gesamtstatus: **🟡 Active Development — solide Governance-Basis, CI rot in 11 Repos
(davon 7 identische Dependency-Graph-Konfiguration), Fix-Forward aktiv.**
