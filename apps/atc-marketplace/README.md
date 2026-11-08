# atc-marketplace [L5]

ATC Marketplace — DEX, NFT/Asset-Registry (ATC-9000).

**Vault-Restauration (07.09.2026, AD-020/026/027):** Inhalt aus dem Wiki-Vault
(docs/archive/monorepo-full/) restauriert — vor der Repo-Leerung byte-identisch gesichert. Keine — Vault-Stand konsistent.

**Module:** atc-dex, atc-assets

**Meile (AD-027):** M6 — Dienste laufen

**Hinweis:** Basis fuer den Rebuild; Gate-Kriterien laut LAUFFAEHIGKEITS_ROADMAP
(a-townchain-os-docs/docs/roadmap/).

---

## ATC Compliance & Governance (ATC-STD-201 / 202 / 203)

**ATC COMPLIANCE: R2 — HISTORISCHER AUDIT-STAND (2026-09-07).** Dieser datierte R-Level ist keine aktuelle Exact-SHA-Verifikation und keine Produktionsfreigabe; aktuellen Status und Evidence im Repository prüfen.
Architekturentscheidungen: zentral im [DECISIONS_REGISTER](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) (AD-Nummern verbindlich; lokale Entscheidungen in `docs/decisions/`).

- **Purpose:** NFT-/Genesis-Marketplace (L5).
- **Scope:** Layer L5, Domain marketplace — atc-marketplace als Marketplace-Repository im Organisationsinventar; der GitHub-Bestand vom 2026-10-09 umfasst 33 Repositories (26 nicht archiviert, 7 archiviert).
- **Architecture:** Genesis-Chronicles-NFTs (ATC-9000) auf Chain-ID 658467.
- **Features:** Marketplace-Core.
- **Installation:** Modul-Build je Sprache (typescript); Integration via Monorepo-Workspace (a-townchain-os, sync_modules.py).
- **Development:** Conventional Commits; Governance-Regeln aus atc-standards; Naming gemaess ATC-STD-000 §7.
- **Testing:** Testplan bis M6; Governance-CI.
- **Security:** SECURITY.md; S-Klasse S1; ATC-STD-203 Release-Gates; Emergency-Prozess ATC-STD-000 §32.
- **Roadmap:** Einordnung in die Lauffaehigkeits-Roadmap M1-M8 (AD-027) und Bauhierarchie L0-L7 (AD-026).
- **Version:** CHANGELOG.md; SemVer; Releases als ATC-REL-X.Y.Z.
- **License:** Apache-2.0 — Apache-2.0, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC/ATS-LIC).
