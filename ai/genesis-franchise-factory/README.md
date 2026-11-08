# Genesis Franchise Factory (GFF)

Repository identifier: `genesis-franchise-factory`

**[L6] Content-Pipeline-Orchestrator des Genesis-Ökosystems** — erstes eigenständiges Repo der Franchise Factory (AD-20–AD-43), gebaut am 13.09.2026 aus den kanonischen Specs im Org-Archiv (`a-townchain-os-docs/docs/archive/monorepo-full/src/modules/atc-franchise`).

> **Plattform-Regel (verbindlich):** GFF sitzt **über** der Genesis Engine (GCL v2.0) und ist strikte Plattform — **keinerlei Abhängigkeit zu Genesis Chronicles**. Das Spec-Gate (CI) erzwingt dies maschinell.

## Architektur

```
specs/*.atc          23 Factory-Specs (AD-21..43) + GFF Core (AD-20)
                     + ATC-9900 DAO-Kern (factory.atc, routes.atc)
                     + Contracts (registry.atc, revenue.atc)
gff/
  core.py            Pipeline-Orchestrator (AD-20): Franchise-Registry,
                     10-stufige Default-Pipeline, injizierbarer Executor,
                     ehrlicher Dry-Run (keine Fake-Outputs)
  dao.py             ATC-9900 DAO-Modell: Franchise-DAOs mit Token, Vault,
                     Royalty-Tiers (Bronze 5 %…Platinum 2 %), Governance
  lifecycle.py       12-Phasen-Lifecycle (AD-43): Idea→…→Archived,
                     Milestones, Health/KPI
  spec_loader.py     Deskriptor-Parser für .atc-Specs (KEIN ATCLang-
                     Compiler — der lebt in atclang, L0)
tools/
  validate_specs.py  CI-Spec-Gate: 23 Factories vollständig, AD-Mapping,
                     Copyright-Header, Plattform-Trennung
tests/               33 pytest-Tests (alle Module + Gate)
```

## Factories (23 + 2 Cores)

**v1.0 (AD-20–31):** GFF Core · IP · World · Character · Lore · Quest · Economy · LiveOps · AI Content · Merchandise · Community · Analytics

**v2.0 (AD-32–43):** Blueprint · Canon Engine · Asset Intelligence · Gameplay · Narrative · Multiplayer · Creator · Publishing · Commerce · AI Director · Security · Lifecycle Manager

## Quickstart

```bash
pip install -e ".[dev]"
python tools/validate_specs.py   # Spec-Gate
pytest -q                        # 33 Tests
ruff check gff tests tools
```

## Status: EXPERIMENTAL (R1)

> **Evidence freshness notice (2026-10-09):** the `33 pytest tests` count below is a repository-declared test inventory, not a statement that the current default-branch SHA passed those tests. The latest-commit query during the documentation refresh returned no PR-triggered workflow runs for the inspected SHA; this does not prove that no CI exists. Use the current Actions run/job/step/log evidence before claiming test verification.

- Kanonische Specs: **unverändert** aus dem Archiv übernommen (Copyright-Header original)
- Referenz-Implementierung: Core/DAO/Lifecycle mit Tests — **implementiert, nicht auditiert**
- Offen: GCL-Bus-Integration, ATC-VM-Bindung (AD-20-Specs nutzen `Chain::timestamp()`), Store-/Publishing-Anbindung
- Evidence: `.atc/evidence/evidence.yaml` (SCR-0080-Ehrlichkeit: partial, TESTED lokal)

## Governance

A-TownChain-Ökosystem · ATC-STD-201/202 · Registry: `ATC-REPO-GFF-001` · Chain-ID 658467
