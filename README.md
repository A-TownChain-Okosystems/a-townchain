# a-townchain [L3]

A-TownChain Blockchain L1 — Chain-ID 658467, PoW+PoS+PoH, ZKP, Governance, DNS, Testnet (AD-012: Kernel-System-Service).

**Vault-Restauration (07.09.2026, AD-020/026/027):** Inhalt aus dem Wiki-Vault
(docs/archive/monorepo-full/) restauriert — vor der Repo-Leerung byte-identisch gesichert. Keine — Chain-ID 658467 im Vault-Stand (24 Dateien verifiziert, 0 x 9000).

**Module:** atc-blockchain, atcnet, atc-zkp, atc-governance, atc-dns, atc-testnet

**Meile (AD-027):** M4 — Blockchain laeuft: 2 Nodes Gossip-Sync, Transaktion validiert, Genesis Chain-ID 658467, ATCLang-Contract auf ATVM

**Hinweis:** Basis fuer den Rebuild; Gate-Kriterien laut LAUFFAEHIGKEITS_ROADMAP
(a-townchain-os-docs/docs/roadmap/).

---

## ATC Compliance & Governance (ATC-STD-201 / 202 / 203)

**ATC COMPLIANCE: R4** — auditiert am 2026-09-07 (atc-repo-audit; R-Level aus `.atc/repository.yaml`).
Architekturentscheidungen: zentral im [DECISIONS_REGISTER](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) (AD-Nummern verbindlich; lokale Entscheidungen in `docs/decisions/`).

- **Purpose:** A-TownChain Blockchain — die Chain des Oekosystems (L3).
- **Scope:** Layer L3, Domain blockchain — a-townchain als CORE in der 23-Repo-Landschaft (AD-024/026).
- **Architecture:** ShivaConsensus (PoW SHA3-ATC + PoS reputation-weighted + PoH VDF-basiert); Chain-ID 658467; ATC-8300/9000 Token-Standards; DAG als Evolutionspfad (ATC-04).
- **Features:** 6 Module (blockchain, atcnet, wallet, contracts, bridge, dex/explorer/governance); ECDSA secp256k1.
- **Installation:** Modul-Build je Sprache (rust); Integration via Monorepo-Workspace (a-townchain-os, sync_modules.py).
- **Development:** Conventional Commits; Governance-Regeln aus atc-standards; Naming gemaess ATC-STD-000 §7.
- **Testing:** Modul-Tests im Docker-Stack (10 Dienste compose).
- **Security:** SECURITY.md; S-Klasse S4; ATC-STD-203 Release-Gates; Emergency-Prozess ATC-STD-000 §32.
- **Roadmap:** Einordnung in die Lauffaehigkeits-Roadmap M1-M8 (AD-027) und Bauhierarchie L0-L7 (AD-026).
- **Version:** CHANGELOG.md; SemVer; Releases als ATC-REL-X.Y.Z.
- **License:** Proprietaer — All Rights Reserved, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC/ATS-LIC).
