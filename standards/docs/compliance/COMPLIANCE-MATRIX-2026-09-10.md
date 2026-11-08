# 27/27 Repository Compliance Matrix — 2026-09-10 (SCR-0079)

> Reproduzierbar via `_matrix_collect.py` + `_matrix_gen.py` + `_matrix_final.py` (API-getrieben).
> Legende: 🟢 konform · 🔴 fehlt/rot · ➖ nicht anwendbar (EXEMPT/spec-only).
> `.github` = EXEMPT-Klasse gemäß SCR-0075 (Governance-Hub prüft sich via PR #4).
> Dimensionen: 1 ATC-STD-000 (.atc/repository.yaml) · 2 Governance-CI grün · 3 ATC-STD-README-001 (Compliance-Badge) ·
> 4 Agent-Governance (AGENTS.md) · 5 Lizenz Apache-2.0 · 6 Test-Suite-CI (➖ = kein testbarer Code) ·
> 7 CodeQL aktiv (via code-scanning-API verifiziert) · 8 Evidence (.evidence/) · 9 STATUS.md ·
> 10 Docs · 11 Roadmap · 12 Artifact-Inventar (FILE_REGISTER.md).

| Repository | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | Klasse |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| .github | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | EXEMPT |
| a-townchain | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | PRODUCT |
| a-townchain-os | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | PRODUCT |
| a-townchain-os-docs | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | SPEC-ONLY |
| atc-algorithm | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | SPEC-ONLY (0 Code) |
| atc-compute | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🔴 | 🔴 | SPEC-ONLY (0 Code) |
| atc-contracts | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| atc-explorer | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| atc-indexer | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| atc-interop | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| atc-launchpad | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | SPEC-ONLY (0 Code) |
| atc-marketplace | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | PRODUCT |
| atc-mining | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | SPEC-ONLY (0 Code) |
| atc-node | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🔴 | 🔴 | SPEC-ONLY (0 Code) |
| atc-oracle | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | SPEC-ONLY (0 Code) |
| atc-sdk | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| atc-shivacore | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| atc-standards | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | PRODUCT |
| atc-storage | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | SPEC-ONLY (0 Code) |
| atc-vm | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | SPEC-ONLY (0 Code) |
| atc-wallet | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| atc-zkp | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | ➖ | 🔴 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | SPEC-ONLY |
| atclang | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| aurora-ai | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| genesis-chronicles | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| genesis-engine | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | PRODUCT |
| globus-os | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 | 🟢 | 🔴 | 🟢 | PRODUCT |

**Bilanz: 246/302 anwendbare Zellen grün (81.5 %)** (➖-Zellen: 22, EXEMPT/spec-only).

## Rot-Verteilung (nur PRODUCT/SPEC-ONLY-Repos)

- **6 Tests-CI** (3): a-townchain-os, atc-interop, atc-standards
- **7 CodeQL** (26): a-townchain, a-townchain-os, a-townchain-os-docs, atc-algorithm, atc-compute, atc-contracts, atc-explorer, atc-indexer, atc-interop, atc-launchpad, atc-marketplace, atc-mining, atc-node, atc-oracle, atc-sdk, atc-shivacore, atc-standards, atc-storage, atc-vm, atc-wallet, atc-zkp, atclang, aurora-ai, genesis-chronicles, genesis-engine, globus-os
- **8 Evidence** (11): a-townchain-os, atc-algorithm, atc-compute, atc-launchpad, atc-mining, atc-node, atc-oracle, atc-standards, atc-storage, atc-vm, atc-zkp
- **11 Roadmap** (5): a-townchain, atc-compute, atc-marketplace, atc-node, globus-os
- **12 Inventar** (11): a-townchain-os, atc-algorithm, atc-compute, atc-launchpad, atc-mining, atc-node, atc-oracle, atc-standards, atc-storage, atc-vm, atc-zkp

## Verifizierte Kernbefunde

1. **F-055 RESOLVED:** Alle 14 Dependabot-Alerts in a-townchain-os waren verwaist
   (Manifest `aistudio/temp_repo/requirements-kai.txt` existiert nicht, 0 Commits in
   Historie) — 14/14 dokumentiert dismissed (SCR-0078/0079). Org-weit jetzt 0 offene Alerts.
2. **P0-01 verifiziert (4 Zahlen!):** Registry `standards.yaml` = 447 Einträge ·
   atc-standards README = 431/432 (4 Stellen) · .github README = 433 · Standard-Dateien = 443.
   Vier Quellen, vier Zahlen → SSOT-Vererbung P0.
3. **CodeQL org-weit inaktiv:** code-scanning-API 404 auf allen geprüften Repos —
   Rollout (Issue #95) nie abgeschlossen. `security-events:write`-Permissions existieren,
   aber kein Scanner läuft.
4. **Evidence-Lücke systemisch:** `.evidence/` existiert in 0/27 Repos (F-079 org-weit bestätigt).
5. **10 Repos ohne testbaren Code** (8 mit 0 Code-Dateien + atc-zkp 7 + a-townchain-os-docs
   als Docs-Hub) — „Implementation 0–30 %" erstmals exakt quantifiziert.
6. **13/13 Repos mit Test-Suite-CI grün** (F-093 RESOLVED): Kernel 674 Tests, Wallet,
   GlobusOS (Rust+npm), Explorer, Indexer, Marketplace, Contracts, SDK, ATCLang, A-TownChain,
   Genesis×2, Aurora (27/27 vitest) — alle nach der Dependabot-MAJOR-Welle verifiziert.
