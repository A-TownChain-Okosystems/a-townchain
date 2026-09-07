# atc-storage

> Dezentrale Storage-Schicht fuer Assets, Metadaten und grosse Dateien.

**Prioritaet:** P2 (Repository-Landkarte AD-024) | **Chain-ID:** 658467 (AD-004) | **Org:** [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems)

> ## Fuer KI-Agenten - Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo [`a-townchain-os-docs`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs):
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md)
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md)
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) - insb. AD-024 (Landkarte), AD-017 (Sync), AD-023 (kein Mainnet-Termin)

---

## Architektur

```
atc-storage (dezentrale Storage-Schicht)
    +-- protocol/ - node/ - client/
    +-- encryption/ - replication/ - indexing/
    +-- sdk/

Use: NFT-Metadaten - Game-Assets - AI-Modelle/Datasets - User-Files
```

## Status (AD-020-Rebuild-Aera)

Dieses Repo wurde per AD-024 (06.09.2026) als vertikales Produkt-Repo angelegt.
Neues Repo ohne Vault-Bestand - Grundstruktur, Implementierung folgt qualitaetsgetrieben (AD-023).

## Regeln (verbindlich)

1. Produkt-Repo = kanonische Modul-Quelle (AD-017); Monorepo nur Integration via `scripts/sync_modules.py`.
2. Kein neuer Code ohne Test; ATCLang First (ATC-99), Rust-first per AD-021/022.
3. Commits signieren: `[agent: aurora-base44-superagent-<App-ID>]`.
4. Kein Mainnet-Termin (AD-023) - Rebuild qualitaetsgetrieben.

---

[agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5]

---

## ATC Compliance & Governance (ATC-STD-201 / 202 / 203)

**ATC COMPLIANCE: R1** — auditiert am 2026-09-07 (atc-repo-audit; R-Level aus `.atc/repository.yaml`).
Architekturentscheidungen: zentral im [DECISIONS_REGISTER](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) (AD-Nummern verbindlich; lokale Entscheidungen in `docs/decisions/`).

- **Purpose:** Storage-Service (L5, Roadmap M6).
- **Scope:** Layer L5, Domain storage — atc-storage als CORE in der 23-Repo-Landschaft (AD-024/026).
- **Architecture:** Kernel-Fundament: Layer-5-ContentCap (BLAKE3 content-addressed, AD-012-Erweiterung).
- **Features:** Skelett mit Kernel-Fundament.
- **Installation:** Modul-Build je Sprache (rust); Integration via Monorepo-Workspace (a-townchain-os, sync_modules.py).
- **Development:** Conventional Commits; Governance-Regeln aus atc-standards; Naming gemaess ATC-STD-000 §7.
- **Testing:** Testplan bis M6; Governance-CI.
- **Security:** SECURITY.md; S-Klasse S2; ATC-STD-203 Release-Gates; Emergency-Prozess ATC-STD-000 §32.
- **Roadmap:** Einordnung in die Lauffaehigkeits-Roadmap M1-M8 (AD-027) und Bauhierarchie L0-L7 (AD-026).
- **Version:** CHANGELOG.md; SemVer; Releases als ATC-REL-X.Y.Z.
- **License:** Proprietaer — All Rights Reserved, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC/ATS-LIC).
