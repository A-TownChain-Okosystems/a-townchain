# atc-wallet

> Wallet mit eigener Security-Boundary - waechst nicht in andere Projekte hinein.

**Prioritaet:** P0 (Repository-Landkarte AD-024) | **Chain-ID:** 658467 (AD-004) | **Org:** [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems)

> ## Fuer KI-Agenten - Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo [`a-townchain-os-docs`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs):
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md)
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md)
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) - insb. AD-024 (Landkarte), AD-017 (Sync), AD-023 (kein Mainnet-Termin)

---

## Architektur

```
atc-wallet (eigene Security Boundary)
    +-- core/ - keys/ - accounts/ - signing/
    +-- transaction/ - recovery/ - security/
    +-- hardware/ (Ledger/Trezor)
    +-- mobile/ - desktop/

Schutzgueter: Key-Management, Seed/Recovery, Signing,
Address-Derivation (BIP44 m/44'/9000'), TX-Preview,
Chain-ID-Schutz (658467), Replay-Protection
```

## Status (AD-020-Rebuild-Aera)

Dieses Repo wurde per AD-024 (06.09.2026) als vertikales Produkt-Repo angelegt.
Vault-Inhalt restauriert (AD-018: docs/archive/monorepo-full/) - Bestand aus der 70-Repo-Aera.

## Regeln (verbindlich)

1. Produkt-Repo = kanonische Modul-Quelle (AD-017); Monorepo nur Integration via `scripts/sync_modules.py`.
2. Kein neuer Code ohne Test; ATCLang First (ATC-99), Rust-first per AD-021/022.
3. Commits signieren: `[agent: aurora-base44-superagent-<App-ID>]`.
4. Kein Mainnet-Termin (AD-023) - Rebuild qualitaetsgetrieben.

---

[agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5]
