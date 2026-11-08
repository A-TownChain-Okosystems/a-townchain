# 📦 ATCLang Archive — Superseded Files

> **Stand:** 03.08.2026 | A-TownChain OS v1.0.0
> **Policy:** ATCLang First (ATC-99) | Non-EVM (AD-007)
> **Archiviert von:** Aurora (agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5)

---

## Archiv-Struktur

```
archive/
├── ATCLANG_ARCHIVE.md       ← Diese Datei
├── atclang-v01/              ← v0.1 Syntax (nicht parsebar mit v0.3+ Parser)
│   ├── atcos_main.atc        ← v1.0 Showcase (demo only)
│   ├── consensus/            ← v0.1 Consensus (superseded by _atc8X)
│   │   ├── pow.atc           → pow_atc81.atc
│   │   ├── pos.atc           → pos_atc82.atc
│   │   ├── poh.atc           → poh_atc83.atc
│   │   ├── hybrid_consensus  → hybrid_atc84.atc
│   │   ├── fork_resolution   → fork_atc85.atc
│   │   └── gas_fee.atc       → gas_fee_atc86.atc
│   └── contracts/            ← v0.1 Contracts (superseded by modules/contracts/)
│       ├── genesis_token.atc → modules/contracts/atc001/
│       ├── contract_engine   → modules/contracts/base/
│       ├── governance        → modules/contracts/governance/
│       └── breeding.atc      → modules/contracts/shivamon/
└── duplicates/               ← Duplikate aus src/ Migration (K3/K4)
    ├── contract_registry.atc
    ├── smart_contract_registry.atc
    ├── smart_contracts.atc
    └── kai_cli.atc
```

## Archivierungs-Regeln

1. **Kein Code-Verlust** — alle Dateien bleiben im Repo, nur verschoben
2. **Redirect-Stubs** — an jedem ursprünglichen Ort liegt ein Stub mit Link zum neuen Pfad
3. **Versioniert** — jedes archivierte File behält seinen Original-Header
4. **Dokumentiert** — diese Datei listet jede Verschiebung mit Begründung

---

## Archivierte Dateien

### 1. atcos_main.atc (v1.0 Showcase)
| Feld | Wert |
|------|------|
| **Original-Pfad** | `atclang/programs/atcos_main.atc` |
| **Archiv-Pfad** | `archive/atclang-v01/atcos_main.atc` |
| **Grund** | v1.0 Syntax-Demo, nicht parsebar mit v0.3+ Parser |
| **Status** | Demo/Historical — keine Runtime-Abhängigkeit |
| **Ersetzt durch** | modules/ (v0.3/v0.4 Module) |

### 2. Consensus v0.1 → v0.3 (_atc8X)
| Original-Pfad | Archiv-Pfad | Neuer Pfad | ATC-Standard |
|---------------|-------------|------------|-------------|
| `blockchain/consensus/pow.atc` | `archive/atclang-v01/consensus/pow.atc` | `blockchain/consensus/pow_atc81.atc` | ATC-81 |
| `blockchain/consensus/pos.atc` | `archive/atclang-v01/consensus/pos.atc` | `blockchain/consensus/pos_atc82.atc` | ATC-82 |
| `blockchain/consensus/poh.atc` | `archive/atclang-v01/consensus/poh.atc` | `blockchain/consensus/poh_atc83.atc` | ATC-83 |
| `blockchain/consensus/hybrid_consensus.atc` | `archive/atclang-v01/consensus/hybrid_consensus.atc` | `blockchain/consensus/hybrid_atc84.atc` | ATC-84 |
| `blockchain/consensus/fork_resolution.atc` | `archive/atclang-v01/consensus/fork_resolution.atc` | `blockchain/consensus/fork_atc85.atc` | ATC-85 |
| `blockchain/consensus/gas_fee.atc` | `archive/atclang-v01/consensus/gas_fee.atc` | `blockchain/consensus/gas_fee_atc86.atc` | ATC-86 |

### 3. Contracts v0.1 → v0.3 (modules/contracts/)
| Original-Pfad | Archiv-Pfad | Neuer Pfad |
|---------------|-------------|------------|
| `blockchain/contracts/atc001/genesis_token.atc` | `archive/atclang-v01/contracts/genesis_token.atc` | `modules/contracts/atcoin/atcoin.atc` |
| `blockchain/contracts/contract_engine_atc14.atc` | `archive/atclang-v01/contracts/contract_engine_atc14.atc` | `modules/contracts/base/base_contract.atc` |
| `blockchain/contracts/governance/governance_contract.atc` | `archive/atclang-v01/contracts/governance_contract.atc` | `modules/contracts/governance/governance_contract.atc` |
| `blockchain/contracts/shivamon/breeding.atc` | `archive/atclang-v01/contracts/breeding.atc` | `modules/contracts/shivamon/shivamon_contract.atc` |

### 4. src/ Duplikate (K3/K4 Migration)
| Original-Pfad | Archiv-Pfad | Kanonischer Pfad |
|---------------|-------------|-----------------|
| `src/blockchain/contract_registry.atc` | `archive/duplicates/contract_registry.atc` | `blockchain/contract_registry.atc` |
| `src/blockchain/smart_contract_registry.atc` | `archive/duplicates/smart_contract_registry.atc` | `blockchain/smart_contract_registry.atc` |
| `src/blockchain/smart_contracts.atc` | `archive/duplicates/smart_contracts.atc` | `blockchain/smart_contracts.atc` |
| `src/core/kai_cli.atc` | `archive/duplicates/kai_cli.atc` | `core/kai_cli.atc` |

---

## Behalten (nicht archiviert)

Gemäß `docs/DEPRECATED.md`:

| Komponente | Grund |
|-----------|-------|
| ATCLang Compiler (.py) | Infrastruktur — Lexer/Parser/Compiler/VM |
| bridge_contract.atc | Bridge + API Layer (Non-EVM zu anderen Chains) |
| config/mainnet_genesis.json | Mainnet-Konfiguration (Sprint 4.0) |
| Alle modules/*.atc | Aktive v0.3/v0.4 Module |
| Alle blockchain/consensus/*_atc8X.atc | Aktive v0.3 Standards |

---

*Archive v1.0.0 — Aurora · 03.08.2026 (Europe/Berlin)*
