# atc-indexer

> Indexing-Pipeline als eigenstaendige Infrastruktur - speist Explorer, Wallets und dApps mit Chain-Daten.

**Prioritaet:** P1 (Repository-Landkarte AD-024) | **Chain-ID:** 658467 (AD-004) | **Org:** [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems)

> ## Fuer KI-Agenten - Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo [`a-townchain-os-docs`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs):
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md)
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md)
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) - insb. AD-024 (Landkarte), AD-017 (Sync), AD-023 (kein Mainnet-Termin)

---

## Architektur

```
atc-indexer (Chain Indexing Pipeline)
    +-- indexer/ (Block-/TX-Ingestion)
    +-- database/ (PostgreSQL-Schema)
    +-- websocket/ (Live-Events)
    +-- search/ - analytics/
    +-- feeds: atc-explorer, atc-wallet, dApps
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
