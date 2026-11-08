# ARCHITECTURE — genesis-franchise-factory

| Feld | Wert |
|---|---|
| Registry-ID | ATC-REPO-GFF-001 (Antrag via atc-standards-PR) |
| Layer | L6 (applications) |
| Domain | franchise-factory |
| Criticality / Security | C3 / S2 |
| Kanonische Spec-Quelle | `specs/*.atc` (AD-20..AD-43, unverändert aus Org-Archiv) |
| Sprache | Python 3.11 (stdlib-only) + ATCLang-Specs |
| Reife | R1 · EXPERIMENTAL |

## 1. Zweck

Die Genesis Franchise Factory (GFF) ist die übergeordnete Plattform zur Erzeugung, Verwaltung und Skalierung ganzer Marken im Genesis-Ökosystem (AD-20). Sie orchestriert 23 Factories über definierte Pipelines und verwaltet Franchise-Lebenszyklen (AD-43) sowie dezentrale Franchise-DAOs (ATC-9900).

## 2. Position im Ökosystem

```
Genesis Chronicles (L6, Spiel)   ← nutzt Engine + GFF-Factories, wird NIE importiert
        │
Genesis Engine (L6, Engine)      ← GCL v2.0
        │
GFF (L6, Factory-Plattform)      ← dieses Repo; nutzt ATC-VM/ATCLang-Konzepte,
        │                           importiert aber derzeit KEINE Engine-Artefakte
ATC-VM / ATCLang / A-TownChain   ← Contract-/Settlement-Ziel der DAO-Modelle (geplant)
```

**Trennungsregeln (CI-erzwungen):**
1. GFF importiert niemals Genesis Chronicles (`tools/validate_specs.py` blockt `import *chronicles*` in allen `.atc`-Dateien).
2. Genesis-Engine-Features werden generisch entwickelt und von GFF nur konsumiert (Engine-First-Modell).

## 3. Modul-Map

| Modul | Kanonische Spec | Inhalt |
|---|---|---|
| `gff/core.py` | `gff_core_ad20.atc` | GFFCore, Franchise-Registry, Pipeline (10 Stufen), Events, Executor-Schnittstelle |
| `gff/dao.py` | `factory.atc` + Prototyp `factory.py` | ATC-9900: FranchiseFactory, FranchiseVault, RoyaltyTier, Revenue-Distribution |
| `gff/lifecycle.py` | `lifecycle_manager_ad43.atc` | 12-Phasen-Statemachine, Milestones, Templates, Health |
| `gff/spec_loader.py` | — | Deskriptor-Parser (Metadaten), kein Compiler |
| `specs/` | AD-21..43 + AD-20 + ATC-9900 + Contracts | Kanonische Quelle (unverändert) |

## 4. Datenmodell-Kern

- `Franchise` (AD-20): id (sha256), name, universe, status (6-stufig), blueprint, progress
- `FranchiseBlueprint` (AD-20): name, genre, target_audience, Zählgrößen, economy_model, monetization, platforms
- `LFranchise` (AD-43): phase, history, budget/spent, risk/prob, milestones, KPI
- `Franchise` (ATC-9900): token_symbol/supply, members (addr→stake), vault, royalty_tier, proposals

## 5. Pipeline-Semantik (AD-20)

1. Stufen laufen streng nach `order` (Default: Blueprint→World→Character→Lore→Quest→Asset→Game→Testing→LiveOps→Merchandise).
2. `enabled=false` → `Skipped` (Progress zählt nur aktivierte Stufen).
3. Executor-Ergebnis `success=false` → Abbruch mit `StageFailed`-Event (kein Stillhalten, kein Fake-Erfolg).
4. Ohne injizierten Executor: Dry-Run (`StageResult(note="dry-run")`) — ehrlich, keine generierten Inhalte vorgetäuscht.

## 6. Lifecycle-Semantik (AD-43)

- 12 Phasen (Enum-Reihenfolge verbindlich); erlaubte Übergänge: adjacent (+1) oder Direkt-Archivierung; Rückwärts sprünge verboten.
- Ehrlichkeits-Notiz: Das GFF-v2-Wiki nennt „11 Phasen"; die kanonische Spec deklariert 12 — **die Spec gewinnt**.

## 7. Grenzen (aktuell, ehrlich)

- Keine ATC-VM-/Chain-Bindung (Specs nutzen `Chain::timestamp()`; Implementierung nutzt injizierbare `now`-Parameter)
- Kein GCL-Bus; Executor-Schnittstelle ist bewusst injizierbar statt hart verdrahtet
- DAO-Modell ist Referenz-Implementierung, kein ausgelagerter Smart Contract

## 8. Standards-Compliance

- ATC-STD-201: repository.yaml, evidence.yaml, ARCHITECTURE.md ✓
- SCR-0080: Evidence-Datei ehrlich (implementation: partial, tests: implemented, security: not_audited)
- Plattform-Trennung: maschinell geprüft (Spec-Gate, CI-verbindlich)

## 9. Roadmap

1. R2: Executor-Integration (Genesis-Engine-Asset-Pipeline als erster realer Executor)
2. ATC-9900: ATCLang-Contract-Portierung (atc-contracts, nach ATC-VM-Gate)
3. Factory-Profile: fachliche Standards je Factory via SCR (SCR-0118-Muster)
