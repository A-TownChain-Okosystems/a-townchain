---
document_id: ATC-DOC-SC-REG-001
title: Contract Registry — Struktur und Pflege
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-SC-019
---

# Contract Registry

Zentrale Registry aller Smart Contracts des A-TownChain-Oekosystems
(ATC-STD-SC-019). Registry-First: jeder Contract wird vor Implementierung
(SC-G0) angelegt und vor Mainnet (SC-G12) vollstaendig gepflegt.

```text
contracts/
├── registry/
│   ├── contracts.yaml      # Alle Contracts (Identitaet nach SC-002)
│   ├── deployments.yaml   # Deployment Records nach SC-006 (ATC-DEP-NNNN)
│   └── versions.yaml       # Contract-Versionshistorie
├── token/ nft/ defi/ governance/ bridge/ oracle/ gamefi/ mining/ system/
```

## Pflege

- Neue Contracts: Eintrag in registry/contracts.yaml (contract_id nach
  SC-002: ATC-SC-<KATEGORIE>-NNN), Kategorie-Verzeichnis fuer
  Specifications.
- Deployments: Deployment Record in registry/deployments.yaml (SC-006).
- Pruefung: `python3 tools/atc-sc-validator/check_contracts.py`
- Registry enthaelt KEINE Secrets oder privaten Keys (SC-019 Security).
