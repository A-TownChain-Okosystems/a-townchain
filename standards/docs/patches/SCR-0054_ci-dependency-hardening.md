# SCR-0054 — CI-Dependency-Härtung (Owner-Aktion wegen GH013/F-010)

## Status
**BEREIT — wartet auf Owner-Anwendung.** Workflow-Dateien sind per GH013 für
Agenten gesperrt (kein workflow-Scope). Zwei Wege:
- **Weg A:** GitHub-UI-Edit mit den kopierfertigen Patches unten (26 Dateien).
- **Weg B:** Einmalige Scope-Erweiterung (workflow-Scope) — danach pusht Aurora
  alles selbst und verifiziert.

## P1-Patch 1 — atc-standards/.github/workflows/naming-governance.yml
Nach dem `actions/setup-python`-Step einfuegen:

```yaml
      - name: Install dependencies (ATC-STD-CI-001, CI-003)
        run: pip install -r requirements.txt
```

## P1-Patch 2 — atc-standards/.github/workflows/ci.yml
Zeile `run: pip install pyyaml` ersetzen (ad-hoc-Installation unzulaessig, CI-001):

```yaml
      - name: Install dependencies (ATC-STD-CI-001, CI-003)
        run: pip install -r requirements.txt
```

## P1-Patch 3 — kanonisch fuer 25 Repos (Governance-CI-Propagation)
Der Propagation-Scan (ERR-005/CI-008, 08.09.) fand dasselbe Fehlermuster in
25 weiteren Repos: governance-ci.yml checkt atc-standards aus und fuehrt
deren Validatoren aus — OHNE die Abhaengigkeiten des ausgecheckten Toolings
zu installieren (heute gruen, weil atc_repo_audit.py stdlib-only ist —
CI-010-Verstoss: Erfolg durch zufaellig guenstigen Runner-Zustand).

Fix je Repo (identischer Zweizeiler nach dem zweiten checkout):

```yaml
      - name: Install validator dependencies (ATC-STD-CI-001, CI-003)
        run: pip install -r atc-standards/requirements.txt
```

Betroffen (25): a-townchain-os, atclang, a-townchain-os-docs, atc-shivacore,
a-townchain, globus-os, aurora-ai, genesis-engine, atc-sdk, atc-node,
atc-contracts, atc-wallet, atc-explorer, atc-indexer, atc-mining, atc-interop,
genesis-chronicles, atc-oracle, atc-storage, atc-launchpad, atc-marketplace,
atc-compute, atc-vm, atc-algorithm, atc-zkp (.github/workflows/governance-ci.yml).

## Nachweis nach Anwendung
E-5-Regressionstest (test_ci_dependency_governance.py) schaltet von FAIL auf
PASS; CI gruen auf frischem Runner; Issue #1 verifiziert schliessbar
("verified + prevented", alle 5 Acceptance Criteria).
