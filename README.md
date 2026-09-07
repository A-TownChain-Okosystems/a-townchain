# atc-wallet [L5]

ATC Wallet — Keys, Accounts, ATC-Praefix, BIP44-Derivation.

**Vault-Restauration (07.09.2026, AD-020/026/027):** Inhalt aus dem Wiki-Vault
(docs/archive/monorepo-full/) restauriert — vor der Repo-Leerung byte-identisch gesichert. Keine — Vault-Stand konsistent.

**Module:** atc-wallet

**Meile (AD-027):** M6 — Dienste laufen

**Hinweis:** Basis fuer den Rebuild; Gate-Kriterien laut LAUFFAEHIGKEITS_ROADMAP
(a-townchain-os-docs/docs/roadmap/).

---

## ATC Compliance & Governance (ATC-STD-201 / 202 / 203)

**ATC COMPLIANCE: R2** — auditiert am 2026-09-07 (atc-repo-audit; R-Level aus `.atc/repository.yaml`).
Architekturentscheidungen: zentral im [DECISIONS_REGISTER](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) (AD-Nummern verbindlich; lokale Entscheidungen in `docs/decisions/`).

- **Purpose:** ATC-Wallet (L5) — Adressen ATC+32 Zeichen.
- **Scope:** Layer L5, Domain wallet — atc-wallet als APPLICATION in der 23-Repo-Landschaft (AD-024/026).
- **Architecture:** BIP44 m/44'/9000' (Key-Stabilitaet bewusst bei 9000), ECDSA secp256k1, biometrische Gating-Referenzen (ATC-03).
- **Features:** Wallet-Core, Crypto, Keys.
- **Installation:** Modul-Build je Sprache (rust); Integration via Monorepo-Workspace (a-townchain-os, sync_modules.py).
- **Development:** Conventional Commits; Governance-Regeln aus atc-standards; Naming gemaess ATC-STD-000 §7.
- **Testing:** Testplan bis M6; Governance-CI.
- **Security:** SECURITY.md; S-Klasse S2; ATC-STD-203 Release-Gates; Emergency-Prozess ATC-STD-000 §32.
- **Roadmap:** Einordnung in die Lauffaehigkeits-Roadmap M1-M8 (AD-027) und Bauhierarchie L0-L7 (AD-026).
- **Version:** CHANGELOG.md; SemVer; Releases als ATC-REL-X.Y.Z.
- **License:** Proprietaer — All Rights Reserved, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC/ATS-LIC).
