# Changelog — atc-contracts

## [Unreleased] — 2026-09-08 (P1-001, ATC-ORG-AUDIT-002 / Issue #99)
- **NEU: Contract-Test-Suite (tests/, 59 Tests, 9 Dateien):** Unit, Integration,
  Negative, Authorization, State-Transition, Failure/Revert, Supply-Cap (Gas/Resource),
  Property/Invariant (Fee-Routing, Snapshot-Immutability, Py↔Atc-Parität), Security
  (Bridge-Limits, Chain-Whitelist, Pause-Guards, Permanent Lock). TESTPLAN.md ergänzt.
- **BUGFIX (von der Suite gefunden):** `GovernanceContract` implementierte die abstrakte
  `BaseContract.name()` nicht — Instanziierung war unmöglich, Registry-Aufrufe
  (`list_all`) wären gecrasht. `name()` ergänzt ("ATC Governance DAO (ATC-9900)").

## v3.0.0 — 10.06.2026
Vollständiger Changelog: https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/CHANGELOG.md

### Relevante Fixes
- Fix #33 Gas-Fee Engine





- Fix #26 Integration Tests 9/9

- Fix #24 MultiSig Wallet




- Fix #10 Bridge ETH+POL+BSC