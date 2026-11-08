# ATC Engineering — Architecture

> Normative Referenz: [`docs/architecture/ATC-ENGINEERING-SPEC-001.md`](docs/architecture/ATC-ENGINEERING-SPEC-001.md) (v1.0.0, Owner-Direktive 14.09.2026)

## Rollenbestimmung

`atc-engineering` ist die **Engineering- und Governance-Control-Plane** des A-TownChain-Ökosystems.

```text
AUTHORITY                    EXECUTION
    │                            │
    ▼                            ▼
atc-standards              atc-engineering
(WHAT: Standards,          (HOW: Enforcement, Validation,
 Requirements, Schemas,             Audit, Build, Test,
 Normative Rules)                   Security, Release)
                                    │
                                    ▼
                          A-TownChain Repository Fleet
```text

- `atc-standards` bleibt SSOT der normativen Regeln.
- `atc-engineering` setzt diese maschinenlesbar durch.
- Keine stillschweigende Überschreibung normativer Regeln.
- **Kein Mega-Repository**: Produkt-Code (atc-node, atclang, ShivaCore, GlobusOS, Aurora …) bleibt in spezialisierten Repos.

## Fünf-Ebenen-Modell

```text
GOVERNANCE PLANE   — Policies / Approvals / Gates / Roles
KNOWLEDGE PLANE    — Standards / Schemas / Rules / Registry
ENGINEERING PLANE  — Build / Test / Audit / Security / Release
EVIDENCE PLANE     — Evidence / Provenance / Attestations
EXECUTION PLANE    — Git / GitHub / CI / Runners / Repositories
```text

## Control Plane ≠ Execution Plane

```text
Configuration → Standards Registry → Policy Engine
→ Governance Engine → Gate Engine          (entscheidet)

Gate → Repo op / Build / Test / Audit /
Security / Release / Evidence              (führt aus)
```text

Eine Policy darf eine Operation erlauben — die Policy Engine führt sie nie selbst aus.

## Integritätsprinzip

> **«No Evidence, No Trust.»** — Declared state is not trusted state.

Derived State: `NOT_READY → DEVELOPMENT → VALIDATED → TESTNET_READY → PRODUCTION_READY`

## Domain Layer — Dependency Direction

```text
            DOMAIN
             /      \
       POLICY       GOVERNANCE
            \          /
           APPLICATION
                 │
        ┌────────┼────────┐
      GitHub    Git       CI
```text

Die Domain hängt nie direkt von GitHub ab (Adapter-Prinzip, `atc-github`).

## Workspace (16 Crates)

`atc-core` · `atc-config` · `atc-standards` · `atc-policy` · `atc-governance` · `atc-repository` · `atc-github` · `atc-audit` · `atc-evidence` · `atc-security` · `atc-build` · `atc-test` · `atc-release` · `atc-agent` · `atc-template` · `atc-schema`

CLI-Binary: `atc-engine` (`cli/atc-engine/`)

## Failure Model

Fail closed. Unknown / Missing Evidence / Invalid Policy / Invalid Signature / Missing Approval / Failed Gate / Unsupported Standard / Unknown State → **BLOCK**, nie ALLOW.

## Policy Gates (G0–G9)

G0 Identity · G1 Scope · G2 Standards · G3 Architecture · G4 Tests · G5 Security · G6 Evidence · G7 Review · G8 Approval · G9 Release. Fehlt ein Gate: **BLOCKED**.
