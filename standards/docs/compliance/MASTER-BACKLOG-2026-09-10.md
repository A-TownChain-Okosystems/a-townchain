# Master-Backlog 2026-09-10 — abgeleitet aus der 27/27 Compliance Matrix (SCR-0079)

> Priorisierung nach P0–P3 gemäß ATC-STD-REPO-AUDIT-001. Vollständige Matrix:
> `COMPLIANCE-MATRIX-2026-09-10.md`. Status = Zustand bei Matrix-Erstellung.

## P0 — vor Release-/Mainnet-Baseline bereinigen

| Finding | Titel | Aktion | Owner-Anteil |
|---|---|---|---|
| F-094 | Registry-Count-Integrity: 4 Quellen/4 Zahlen (447 Registry · 431/432 README · 433 Hub · 443 Dateien) | views/README-Regeneration via `tools/gen_views` + SSOT-Vererbung (keine Handzahlen mehr) | – |
| F-095 | Test-Evidence-Integrity: 423/280/674/731-Zahlen divergieren, kein maschinenlesbares Schema | Evidence-Standard-DRAFT (Artifakt: test-report.json je Repo, auto-generiert in CI, JSON-Schema + Validator-Gate) | §9-Freigabe |
| F-096 | Layer-Taxonomie fehlt (L0–L7 frei interpretierbar) | ATC-LAYER-001-DRAFT (System-Layer vs. Blockchain-Protocol-Layer trennen), danach README-Validierung | §9-Freigabe |
| F-097 | ATCLang-Rollen widersprüchlich (Rust canonical vs. »optional«) | README/Dependencies eindeutig: Canonical=Rust, Reference=Python, Release-Authority=Rust | – |
| F-098 | Status-Claims ohne Evidence (PASS/M4/M5/AUDITED) | Statusmodell (9 Lifecycle-Phasen × 6 unabhängige Dimensionen) + Evidence-Bindung je Claim | §9-Freigabe |

## P1

| Finding | Titel | Aktion | Owner-Anteil |
|---|---|---|---|
| F-099 | CodeQL org-weit inaktiv (code-scanning 404; Permissions vorhanden, Scanner nie aktiviert) | Default-Setup-Rollout je Repo | **Owner-API-Aktion** (Issue #95) |
| F-100 | `.evidence/` in 0/27 Repos (F-079 org-weit) | Evidence-Verzeichnis + records.yaml je PRODUCT-Repo (mit F-095 bündeln) | – |
| F-101 | a-townchain-os: Cargo-Workspace ohne Test-Workflow | test-suite.yml ergänzen (Muster SCR-0078) | – |
| F-102 | 8 Repos mit exakt 0 Code-Dateien (vm, node, storage, oracle, mining, algorithm, compute, launchpad) | Portfolio-Entscheidung: implementieren oder offiziell SPEC-ONLY deklarieren (Matrix-Klasse) | **Owner-Entscheidung** |
| F-092 | Hub-PR #4 (.github) wartet auf 1× Approve | Review + Merge (SCR-0077) | **Owner-UI-Aktion** |

## P2

| Finding | Titel | Aktion |
|---|---|---|
| F-103 | Roadmap-Referenz fehlt in 5 Repos (a-townchain, atc-compute, atc-marketplace, atc-node, globus-os) | Zentrale Roadmap-Referenz ins README je Repo |
| F-104 | Test-Workflows fehlen für atc-interop (16 Code-Dateien) + atc-standards-Tools (26) | test-suite.yml nach SCR-0078-Muster |
| F-090* | FILE_REGISTER fehlt in 11 Repos | Inventar je Repo (mit F-095-Evidence bündeln) |
| — | .github-Hub-Konformanz (STATUS/Badge/eigenes Governance-Exempel) | Nach PR-4-Merge im Hub selbst nachziehen |

## P3

- Alert #170 (a-townchain-os): Dismissal-Kommentar kosmetisch kurz (»test«), Reason korrekt (inaccurate); API-seitig nicht mehr änderbar.
- .github-Auto-Runs des Dependabot-Updaters in den Lauf-Historien (irreführend grün) — Benennungsdokumentation im Hub-README.

## Erledigt in dieser Welle (Referenz)

- F-055 RESOLVED (14 verwaiste Alerts dismissed, RCA dokumentiert — SCR-0078)
- F-093 RESOLVED (13/13 Test-Suite-CIs grün — SCR-0078)
- genesis-engine Compliance-Badge ergänzt (Matrix-Dimension 3 jetzt 27/27)
