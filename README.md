# atc-sdk [L5]

ATC SDK — Developer Platform, CLI, Package System (atcpkg).

**Vault-Restauration (07.09.2026, AD-020/026/027):** Inhalt aus dem Wiki-Vault
(docs/archive/monorepo-full/) restauriert — vor der Repo-Leerung byte-identisch gesichert. Keine — Vault-Stand konsistent.

**Module:** atc-sdk, atc-cli, atc-atcpkg

**Meile (AD-027):** M6 GEPLANT — Dienste NICHT belegt (Evidence incomplete, SCR-0073)

**Hinweis:** Basis fuer den Rebuild; Gate-Kriterien laut LAUFFAEHIGKEITS_ROADMAP
(a-townchain-os-docs/docs/roadmap/).

---

## ATC Compliance & Governance (ATC-STD-201 / 202 / 203)

**ATC COMPLIANCE: R2** — auditiert am 2026-09-07 (atc-repo-audit; R-Level aus `.atc/repository.yaml`).
Architekturentscheidungen: zentral im [DECISIONS_REGISTER](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) (AD-Nummern verbindlich; lokale Entscheidungen in `docs/decisions/`).

- **Purpose:** SDK fuer ATCLang/ATVM und Blockchain-APIs (L5).
- **Scope:** Layer L5, Domain sdk — atc-sdk als SDK in der 23-Repo-Landschaft (AD-024/026).
- **Architecture:** Client-Bibliotheken fuer Chain-ID 658467; ATCLang-Toolchain-Anbindung.
- **Features:** SDK-Bindings, Wallet-Integration.
- **Installation:** Modul-Build je Sprache (rust); Integration via Monorepo-Workspace (a-townchain-os, sync_modules.py).
- **Development:** Conventional Commits; Governance-Regeln aus atc-standards; Naming gemaess ATC-STD-000 §7.
- **Testing:** Testplan bis M6; Governance-CI.
- **Security:** SECURITY.md; S-Klasse S2; ATC-STD-203 Release-Gates; Emergency-Prozess ATC-STD-000 §32.
- **Roadmap:** Einordnung in die Lauffaehigkeits-Roadmap M1-M8 (AD-027) und Bauhierarchie L0-L7 (AD-026).
- **Version:** CHANGELOG.md; SemVer; Releases als ATC-REL-X.Y.Z.
- **License:** Apache-2.0 — Apache-2.0, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC/ATS-LIC).
