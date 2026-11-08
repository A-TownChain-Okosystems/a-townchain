---
standard:
  id: ATC-STD-MAINT-000
  title: "Maintenance Governance & Specification Standard"
  version: "1.0.0"
  status: approved
  category: maint
  family: MAINT
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-13"
  updated: "2026-09-14"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: null
  review_date: null
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-000 — Maintenance Governance & Specification Standard

> **ATC-STD-MAINT-000 is the normative Governance and Contract Layer of the entire
> Maintenance Family.**
>
> **Maintenance Capability is a Release Requirement.**
>
> *A system MUST NOT be released to a lifecycle state requiring operational support unless its
> required maintenance capabilities are implemented, validated, documented, and evidenced.*

## 1. Purpose

ATC-STD-MAINT-000 defines the normative governance and specification model for maintenance across
the A-TownChain Ecosystem.

The standard establishes a common maintenance framework for:

- KAI-OS
- GlobusOS
- ShivaCore
- A-TownChain
- ATC-VM
- ATCLang
- Aurora AI
- infrastructure
- repositories
- standards
- documentation
- applications
- ecosystem integrations

The purpose is to ensure that systems remain secure, maintainable, compatible, observable,
recoverable, and operational throughout their complete lifecycle.

Maintenance MUST be treated as a lifecycle capability and MUST NOT be treated solely as a
post-release activity.

## 2. Scope

This standard applies to all normative ATC systems, repositories, services, platforms, protocols,
runtimes, infrastructure components, and production-bound artifacts that require maintenance.

The standard covers:

- preventive maintenance
- corrective maintenance
- adaptive maintenance
- security maintenance
- operational maintenance
- emergency maintenance
- upgrade and migration
- compatibility management
- rollback and recovery
- end-of-life and retirement
- maintenance automation
- maintenance evidence

The following maintenance classes are defined:

| Class | Name | General Meaning |
|---|---|---|
| M0 | Routine | Normal, planned maintenance |
| M1 | Operational | Operationally significant maintenance |
| M2 | Security | Security-relevant maintenance |
| M3 | Critical | System-critical or emergency maintenance |

## 3. Normative Authority

ATC-STD-MAINT-000 is the parent standard for the "ATC-STD-MAINT-*" family.

It MUST conform to "ATC-STD-000".

Specialized Maintenance Standards MUST inherit the governance baseline defined by this standard.

The relationship is:

```
ATC-STD-000
      │
      ▼
ATC-STD-MAINT-000
      │
      ├── MAINT-001 Classification
      ├── MAINT-002 Lifecycle
      ├── MAINT-003 Code
      ├── MAINT-004 Dependencies
      ├── MAINT-005 Security
      ├── MAINT-006 Infrastructure
      ├── MAINT-007 OS
      ├── MAINT-008 Blockchain
      ├── MAINT-009 VM & Runtime
      ├── MAINT-010 AI
      ├── MAINT-011 Repository
      ├── MAINT-012 Standards
      ├── MAINT-013 Documentation
      ├── MAINT-014 Performance
      ├── MAINT-015 Reliability
      ├── MAINT-016 Compatibility
      ├── MAINT-017 Upgrade & Migration
      ├── MAINT-018 Rollback & Recovery
      ├── MAINT-019 Evidence
      ├── MAINT-020 Automation
      ├── MAINT-021 Emergency
      ├── MAINT-022 End-of-Life
      ├── MAINT-023 Vendor & Supply-Chain
      └── MAINT-024 Cross-Ecosystem
```

A specialized Maintenance Standard MUST NOT weaken a requirement established by this standard
unless an explicitly governed exception exists.

This standard defines the Governance Contract Layer of the maintenance framework: what maintenance
governance is, which capabilities exist, which gates apply, and which evidence is required.
Technical and procedural detail belongs to the specialized standards MAINT-001..024; this standard
MUST NOT accumulate their implementation details.

## 4. Normative Language

The following terms are normative:

| Keyword | Meaning |
|---|---|
| MUST | Mandatory requirement |
| MUST NOT | Prohibited behavior |
| REQUIRED | Mandatory |
| SHOULD | Strong recommendation |
| SHOULD NOT | Strong recommendation against |
| MAY | Optional |

Normative requirements MUST be unambiguous and machine-identifiable.

## 5. Maintenance Principles

### 5.1 Lifecycle Principle

Maintenance MUST exist throughout the complete lifecycle of a system.

### 5.2 Evidence Principle

Maintenance actions MUST produce sufficient evidence to establish what changed, why it changed,
who or what performed the action, and whether the resulting state was validated.

### 5.3 No Evidence, No Trust

A maintenance action MUST NOT be considered successfully completed solely because an
implementation claims that it succeeded.

The resulting state MUST be supported by verifiable evidence.

### 5.4 Fail-Closed Principle

A required maintenance gate MUST fail closed.

If required evidence, validation, approval, or safety information is missing, the affected release
or maintenance operation MUST be blocked.

### 5.5 Least-Privilege Principle

Maintenance automation MUST operate with the minimum permissions required for its assigned task.

### 5.6 Separation of Duties

For M2 and M3 maintenance:

```
Implementer != Validator != Auditor
```

For critical release operations:

```
Release Authority != Implementer
```

Exceptions MUST be explicitly authorized and evidenced.

### 5.7 Reproducibility

Maintenance operations SHOULD be reproducible.

Automated maintenance SHOULD use deterministic configuration, version-pinned tooling, and
machine-readable inputs wherever practical.

## 6. Requirements

Normative requirements carry unique, machine-identifiable IDs of the requirement class MAINT
(`REQ-MAINT-001..999`, registered as `maintRequirementId` in naming-conventions.schema.json),
per §4 and ATC-STD-000 §10/§11. The number space is family-wide: this standard holds
REQ-MAINT-001..015; the specialized standards continue from REQ-MAINT-016 (up to REQ-MAINT-094
as currently specified). Each requirement is defined by its normative statement, priority, and
verification method — CI and audit can address each requirement individually:

```yaml
conformance:
  REQ-MAINT-001: PASS
  REQ-MAINT-002: PASS
  REQ-MAINT-003: PASS
  # ...
  REQ-MAINT-015: PASS
  MAINT-000-CONFORMANCE: PASS
```


### REQ-MAINT-001

```yaml
requirement:
  id: REQ-MAINT-001
  statement: >
    A system MUST define its required Maintenance Capability before entering a lifecycle state that requires supported operation.
  priority: P0
  verification:
    type: evidence
    required:
      - maintenance_capability
      - lifecycle_status
      - release_record
```

Source: §7, §8.1. 

### REQ-MAINT-002

```yaml
requirement:
  id: REQ-MAINT-002
  statement: >
    Maintenance Readiness MUST be a release gate: a release to a state requiring operational support MUST pass the Maintenance Readiness Gate; a failed mandatory check MUST block release.
  priority: P0
  verification:
    type: gate
    required:
      - maintenance_readiness
      - gate_result
      - release_record
```

Source: §8, §8.1. 

### REQ-MAINT-003

```yaml
requirement:
  id: REQ-MAINT-003
  statement: >
    Every maintenance activity MUST use the M0–M3 classification (Routine, Operational, Security, Critical).
  priority: P0
  verification:
    type: evidence
    required:
      - maintenance_record
      - classification
```

Source: §2, §9. 

### REQ-MAINT-004

```yaml
requirement:
  id: REQ-MAINT-004
  statement: >
    Controlled maintenance MUST produce evidence establishing what changed, why, who performed it, and whether the resulting state was validated; completion MUST NOT be declared from implementation claims alone.
  priority: P0
  verification:
    type: evidence
    required:
      - maintenance_record
      - validation_results
```

Source: §5.2, §5.3, §12. 

### REQ-MAINT-005

```yaml
requirement:
  id: REQ-MAINT-005
  statement: >
    Mandatory maintenance gates MUST fail closed: missing evidence, validation, approval, or safety information MUST block the affected release or operation.
  priority: P0
  verification:
    type: ci
    required:
      - conformance_gate_result
      - blocked_on_missing_evidence
```

Source: §5.4. 

### REQ-MAINT-006

```yaml
requirement:
  id: REQ-MAINT-006
  statement: >
    M2 and M3 maintenance MUST enforce Separation of Duties (Implementer != Validator != Auditor); critical release operations MUST separate Release Authority from Implementer; exceptions MUST be explicitly authorized and evidenced.
  priority: P0
  verification:
    type: audit
    required:
      - role_assignment
      - approval_record
```

Source: §5.6, §11. 

### REQ-MAINT-007

```yaml
requirement:
  id: REQ-MAINT-007
  statement: >
    Production systems MUST define a rollback or recovery capability appropriate to their architecture; an unvalidated rollback strategy MUST NOT be represented as tested.
  priority: P0
  verification:
    type: evidence
    required:
      - rollback_strategy
      - recovery_plan
```

Source: §16. 

### REQ-MAINT-008

```yaml
requirement:
  id: REQ-MAINT-008
  statement: >
    Production systems MUST define a compatibility strategy; intentional compatibility breaks MUST be identified, documented, consumer-checked, validated, and rollback-evaluated.
  priority: P0
  verification:
    type: evidence
    required:
      - compatibility_declaration
```

Source: §14. 

### REQ-MAINT-009

```yaml
requirement:
  id: REQ-MAINT-009
  statement: >
    Maintenance automation MUST operate within explicit authorization boundaries, MUST NOT bypass mandatory governance gates, and an AI agent MUST NOT independently authorize actions requiring human approval.
  priority: P0
  verification:
    type: audit
    required:
      - automation_scope
      - authorization_record
```

Source: §5.5, §17. 

### REQ-MAINT-010

```yaml
requirement:
  id: REQ-MAINT-010
  statement: >
    Compliance MUST be evidence-based; a system MUST NOT be declared compliant solely from documentation claims.
  priority: P0
  verification:
    type: audit
    required:
      - compliance_report
      - evidence_bundle
```

Source: §18. 

### REQ-MAINT-011

```yaml
requirement:
  id: REQ-MAINT-011
  statement: >
    Maintenance records MUST be machine-readable and schema-conformant.
  priority: P1
  verification:
    type: ci
    required:
      - maintenance_record_schema_valid
```

Source: §12, §20.1. 

### REQ-MAINT-012

```yaml
requirement:
  id: REQ-MAINT-012
  statement: >
    Maintenance conformance SHOULD be CI-enforced with automated conformance checks; a mandatory conformance failure MUST block the applicable gate.
  priority: P1
  verification:
    type: ci
    required:
      - conformance_workflow
      - conformance_result
```

Source: §20.1, §20.2. 

### REQ-MAINT-013

```yaml
requirement:
  id: REQ-MAINT-013
  statement: >
    Maintenance metrics SHOULD be collected (MTTD, MTTA, MTTR, MTBF, patch latency, rollback success rate, evidence completeness, recovery readiness).
  priority: P1
  verification:
    type: evidence
    required:
      - metrics_report
```

Source: §20.3. 

### REQ-MAINT-014

```yaml
requirement:
  id: REQ-MAINT-014
  statement: >
    Maintenance crossing architectural boundaries MUST assess interface and compatibility impact; layered-system maintenance (e.g. ATC-VM) MUST NOT be treated as an isolated package update.
  priority: P0
  verification:
    type: evidence
    required:
      - impact_assessment
```

Source: §13, §24. 

### REQ-MAINT-015

```yaml
requirement:
  id: REQ-MAINT-015
  statement: >
    M3 (critical) maintenance MUST use the Emergency Maintenance process and MUST NOT eliminate evidence, validation, or post-event accountability.
  priority: P0
  verification:
    type: audit
    required:
      - incident_record
      - emergency_review
```

Source: §9.4, MAINT-021. 

Compact registry view (IDs, levels, priorities):

| ID | Level | Priority | Source |
|---|---|---|---|
| REQ-MAINT-001 | MUST | P0 | §7, §8.1 |
| REQ-MAINT-002 | MUST | P0 | §8, §8.1 |
| REQ-MAINT-003 | MUST | P0 | §2, §9 |
| REQ-MAINT-004 | MUST | P0 | §5.2, §5.3, §12 |
| REQ-MAINT-005 | MUST | P0 | §5.4 |
| REQ-MAINT-006 | MUST | P0 | §5.6, §11 |
| REQ-MAINT-007 | MUST | P0 | §16 |
| REQ-MAINT-008 | MUST | P0 | §14 |
| REQ-MAINT-009 | MUST | P0 | §5.5, §17 |
| REQ-MAINT-010 | MUST | P0 | §18 |
| REQ-MAINT-011 | MUST | P1 | §12, §20.1 |
| REQ-MAINT-012 | SHOULD | P1 | §20.1, §20.2 |
| REQ-MAINT-013 | SHOULD | P1 | §20.3 |
| REQ-MAINT-014 | MUST | P0 | §13, §24 |
| REQ-MAINT-015 | MUST | P0 | §9.4, MAINT-021 |

Applicability: all normative ATC systems, repositories, and production-bound artifacts (§2).
The machine-readable Requirements Registry is generated from this section into
`registry/maintenance-requirements.yaml` (§20.1).

## 7. Maintenance Capability

A system requiring operational support MUST define its required maintenance capabilities before
release.

At minimum, applicable systems MUST define:

- maintenance ownership
- maintenance documentation
- dependency inventory
- security maintenance process
- test strategy
- compatibility strategy
- rollback strategy
- recovery strategy
- monitoring
- evidence collection
- lifecycle status

The required capability set MAY be extended according to system risk.

Maintenance Capability is a release property, not a downstream operational process. The
anti-pattern this standard prohibits:

```
Release
  ↓
Production
  ↓
"Now we have to deal with maintenance."
```

Instead, the normative sequence is:

```
Development
   ↓
Test
   ↓
Validation
   ↓
Maintenance Readiness
   ↓
Release
   ↓
Operation
   ↓
Maintenance
   ↓
Evidence
   ↓
Verified State
```

### 7.1 Repository Requirements

Production repositories governed by this standard SHOULD contain:

- MAINTENANCE.md
- SECURITY.md
- CHANGELOG.md
- STATUS.md
- ROADMAP.md

Critical repositories SHOULD additionally maintain:

```
docs/
└── maintenance/
    ├── procedures/
    ├── runbooks/
    ├── recovery/
    └── evidence/
```

The applicable repository standard MAY define stricter requirements.

## 8. Maintenance Readiness Gate

A release MUST pass the Maintenance Readiness Gate when the target lifecycle state requires
operational support.

```
RELEASE CANDIDATE
        │
        ▼
MAINTENANCE READINESS GATE
        │
   ┌────┴────┐
   │         │
 FAIL       PASS
   │         │
   ▼         ▼
 BLOCK     RELEASE
             │
             ▼
          OBSERVE
             │
             ▼
          EVIDENCE
             │
             ▼
       VERIFIED STATE
```

The gate MUST verify, at minimum:

```yaml
maintenance_readiness:
  maintenance_owner: required
  maintenance_documentation: required
  dependency_inventory: required
  security_process: required
  test_suite: required
  rollback_strategy: required
  compatibility_strategy: required
  monitoring: required
  evidence_collection: required
  lifecycle_status: required
```

A failed mandatory check MUST block release.

### 8.1 Minimum Release Requirement

The following rule is normative:

> *A system MUST NOT be released to a lifecycle state requiring operational support unless its
> required maintenance capabilities are implemented, validated, documented, and evidenced.*

At minimum, the release decision MUST consider:

```
Maintenance Owner
       +
Maintenance Documentation
       +
Dependency Inventory
       +
Security Process
       +
Testing
       +
Compatibility
       +
Rollback
       +
Recovery
       +
Monitoring
       +
Evidence
       =
MAINTENANCE READY
```

If a mandatory capability is absent:

```
MAINTENANCE READY = FALSE
RELEASE = BLOCKED
```
## 9. Classification M0–M3

Every maintenance activity MUST receive a maintenance classification.

### 9.1 M0 — Routine

M0 covers normal, low-risk, planned maintenance.

Examples:

- documentation updates
- dependency updates without material security impact
- small refactorings
- CI optimization
- logging improvements
- metrics improvements
- non-critical performance optimization

M0 maintenance MAY be automated when the applicable controls are satisfied.

### 9.2 M1 — Operational

M1 covers operationally significant maintenance.

Examples:

- build failures
- release pipeline instability
- service instability
- storage problems
- node problems
- performance regressions
- recovery exercises

M1 maintenance MUST receive operational review appropriate to its risk.

### 9.3 M2 — Security

M2 covers security-relevant maintenance.

Examples:

- CVEs
- vulnerable dependencies
- supply-chain compromise
- secret exposure
- key rotation
- sandbox weaknesses
- permission bypasses
- security hardening

M2 maintenance MAY bypass normal release scheduling when required to reduce security exposure.

Security review MUST be performed when required by the affected security boundary.

### 9.4 M3 — Critical

M3 covers events that can materially compromise system integrity, availability, confidentiality,
or consensus.

Examples:

- consensus defects
- kernel security-boundary compromise
- critical ATC-VM incompatibility
- chain-state corruption
- confirmed data loss
- critical remote code execution
- cryptographic trust failure

M3 maintenance MUST use the Emergency Maintenance process.

M3 maintenance MAY temporarily override ordinary maintenance scheduling, but MUST NOT eliminate
evidence, validation, or post-event accountability.

## 10. Lifecycle

All controlled maintenance MUST follow the applicable lifecycle:

```
DETECT
  ↓
ASSESS
  ↓
CLASSIFY
  ↓
PLAN
  ↓
APPROVE
  ↓
IMPLEMENT
  ↓
TEST
  ↓
VALIDATE
  ↓
DEPLOY
  ↓
MONITOR
  ↓
EVIDENCE
  ↓
CLOSE
```

Not every lifecycle stage must require a separate human action, but required controls MUST be
demonstrably satisfied.

### 10.1 Detect

The maintenance requirement MUST be identified through one or more valid sources, such as:

- monitoring
- automated scanners
- vulnerability databases
- tests
- audits
- incident reports
- lifecycle schedules
- user reports
- engineering analysis

### 10.2 Assess

The impact, scope, affected components, dependencies, risks, and urgency MUST be assessed.

### 10.3 Classify

The activity MUST be assigned M0, M1, M2, or M3.

### 10.4 Plan

The implementation, testing, deployment, rollback, and evidence strategy MUST be defined according
to risk.

### 10.5 Approve

Required approvals MUST be obtained before implementation or deployment.

### 10.6 Implement

The approved change MUST be implemented within the authorized scope.

### 10.7 Test

Required automated and manual tests MUST be executed.

### 10.8 Validate

Validation MUST establish that the intended result was achieved and that unacceptable regressions
were not introduced.

### 10.9 Deploy

Deployment MUST follow applicable release and change-control requirements.

### 10.10 Monitor

The resulting system state MUST be monitored for the required observation period.

### 10.11 Evidence

Required evidence MUST be recorded.

### 10.12 Close

A maintenance record MUST NOT be closed until all mandatory acceptance conditions are satisfied.

## 11. Roles & Separation of Duties

The following roles SHOULD be used:

| Role | Responsibility |
|---|---|
| Maintenance Owner | Accountable for maintenance capability |
| Implementer | Performs approved changes |
| Validator | Independently validates results |
| Auditor | Reviews evidence and compliance |
| Security Reviewer | Reviews security impact |
| Release Authority | Authorizes release |
| System Owner | Accountable for system lifecycle |
| Automation Agent | Performs authorized automated maintenance |

Roles MAY be combined for low-risk M0 activities where permitted by the applicable governance
rules.

Roles MUST NOT be combined where separation is explicitly required.

## 12. Evidence

Each controlled maintenance activity MUST have a machine-readable maintenance record.

Minimum structure:

```yaml
maintenance:
  id: MAINT-2026-0001
  classification: M1
  component: atc-node
  detected: "2026-09-13"
  reason: dependency_update

  change:
    before: "x.y.z"
    after: "x.y.z+1"

  validation:
    unit_tests: PASS
    integration_tests: PASS
    security_scan: PASS
    compatibility: PASS
    performance: PASS

  rollback:
    available: true
    tested: true

  evidence:
    commit: "..."
    pull_request: "..."
    test_report: "..."
    audit_report: "..."

  status: CLOSED
```

The actual schema MUST be defined by the applicable machine-readable Maintenance schema.

### 12.1 Evidence Requirements

Evidence SHOULD establish:

- maintenance identifier
- classification
- affected component
- reason
- scope
- before-state
- after-state
- actor
- timestamp
- implementation reference
- validation results
- security results
- compatibility results
- rollback status
- approval
- deployment status
- monitoring result
- closure decision

Evidence MUST be tamper-evident or traceable to an authoritative source where the risk level
requires it.

### 12.2 M2/M3 Additional Controls

For M2 and M3 activities, the following controls are REQUIRED unless a documented emergency
exception applies:

```yaml
critical_maintenance:
  independent_validation: required
  security_review: required
  rollback_test: required
  incident_record: required
  audit_evidence: required
```

Emergency exceptions MUST be documented and reviewed retrospectively.

## 13. Dependencies

This standard distinguishes maintenance dependencies between
affected components (13.1) and standard/artifact dependencies (13.2). Circular dependencies
between standards are PROHIBITED (ATC-STD-000 §12).

### 13.1 Maintenance Dependencies

Maintenance MUST account for dependencies between affected components.

For KAI-OS and other layered systems, changes crossing architectural boundaries MUST validate the
affected interfaces.

For example:

```
ATCLang
   ↓
ATC-VM
   ↓
Contract Execution
   ↓
State Transition
   ↓
Consensus
   ↓
Node
   ↓
Network
```

An ATC-VM maintenance operation MUST NOT be treated as an isolated package update when it can
affect contract execution, state transitions, consensus, or network compatibility.

### 13.2 Standard Dependencies

This standard depends on and references the following artifacts. Circular dependencies between
standards are PROHIBITED (ATC-STD-000 §12).

| Dependency | Type | Direction |
|---|---|---|
| ATC-STD-000 | Meta-governance (conformance, §9 approval, §32 emergency) | upward (this standard MUST conform) |
| ATC-STD-MAINT-001..024 | Specialized standards | downward (inherit the governance baseline, §3) |
| schemas/maintenance/maintenance-record.schema.json | Machine-readable record schema (§11) | enforced by CI (§20, §21) |
| schemas/maintenance/maintenance-readiness.schema.json | Machine-readable gate schema (§7) | enforced by CI (§20, §21) |
| schemas/maintenance/maintenance-evidence.schema.json | Machine-readable evidence envelope (§12) | enforced by CI (§20, §21) |
| registry/standards.yaml + registry/maintenance-conformance.yaml | Registry records and conformance SSOT | mirrored, never diverged |
| GSEPF | Governed execution framework (§26) | GSEPF MUST NOT weaken this standard |
| ATC-STD-REPO-MAINT-001 / ATC-STD-UPDATE-001 / ATC-STD-COMPAT-001 | Domain SSOTs | referenced, not superseded |
| ATC-STD-MAINT-021 | Emergency Maintenance process (§8.4, REQ-015) | downward |

## 14. Compatibility

Maintenance MUST preserve defined compatibility guarantees.

Where compatibility is intentionally broken:

- the break MUST be identified,
- migration requirements MUST be documented,
- affected consumers MUST be identified,
- the new compatibility boundary MUST be validated,
- rollback or recovery MUST be evaluated.

Compatibility requirements are further specified by "ATC-STD-MAINT-016".

## 15. Upgrade & Migration

Production-bound upgrades MUST define:

- source version
- target version
- migration mechanism
- migration validation
- rollback strategy
- data/schema impact
- compatibility impact
- operational impact

Irreversible migrations MUST receive additional review proportional to their risk.

## 16. Rollback & Recovery

Production systems MUST define a rollback or recovery strategy appropriate to their architecture.

For critical systems:

- rollback SHOULD be tested before production deployment;
- recovery procedures MUST be documented;
- recovery evidence MUST be retained;
- recovery assumptions MUST be periodically validated.

A rollback strategy that has never been validated MUST NOT be represented as tested.

## 17. Automation

Maintenance automation MAY perform:

- dependency detection
- vulnerability detection
- SBOM generation
- static analysis
- test execution
- maintenance record creation
- low-risk updates
- evidence collection
- escalation

Automation MUST operate within explicit authorization boundaries.

Automated maintenance MUST NOT bypass mandatory governance gates.

An AI agent MUST NOT independently authorize a maintenance action requiring human approval unless
an applicable governance standard explicitly grants that authority.

## 18. Compliance

A system conforms to ATC-STD-MAINT-000 when:

1. applicable maintenance requirements are identified;
2. required maintenance capabilities exist;
3. applicable maintenance classifications are used;
4. required lifecycle controls are implemented;
5. required gates are enforced;
6. required evidence is generated;
7. required security and compatibility controls are satisfied;
8. applicable rollback/recovery requirements are satisfied;
9. machine-readable conformance requirements are satisfied where applicable.

A system MUST NOT be declared compliant solely from documentation claims.

Compliance MUST be supported by evidence.

## 19. Exceptions

Exceptions MUST:

- identify the affected requirement;
- document the reason;
- identify the affected system;
- assess the risk;
- define compensating controls;
- identify an owner;
- define an expiration or review date;
- be approved by the applicable authority;
- be recorded as evidence.

Permanent exceptions SHOULD NOT be used to bypass fundamental security or integrity requirements.

M3 emergency exceptions MUST receive retrospective review.

## 20. Validation

The validator MUST fail closed for mandatory requirements. Conformance validation SHOULD include:

- schema validation
- metadata validation
- lifecycle validation
- dependency validation
- security scanning
- test execution
- compatibility testing
- rollback verification
- evidence validation
- repository governance validation

### 20.1 Machine-Readable Conformance

The Maintenance framework MUST support machine-readable validation.

The canonical implementation SHOULD contain:

```
schemas/
└── maintenance/
    ├── maintenance-record.schema.json
    ├── maintenance-readiness.schema.json
    └── maintenance-evidence.schema.json
```

The schemas MUST use explicit types and MUST reject unknown properties where strict validation is
required.

CI SHOULD validate:

- maintenance metadata
- classification
- required evidence
- maintenance readiness
- lifecycle state
- required documentation
- dependency inventory
- security controls
- rollback declaration
- compatibility declaration

### 20.2 CI Maintenance Conformance

A repository implementing this standard SHOULD provide automated conformance checks.

Example:

```
Repository
    │
    ├── Dependency Scanner
    ├── Vulnerability Scanner
    ├── SBOM Generator
    ├── License Scanner
    ├── Static Analysis
    ├── Test Suite
    ├── Performance Tests
    ├── Compatibility Tests
    └── Repository Governance
              │
              ▼
      Maintenance Conformance
              │
       ┌──────┼──────┐
       ▼      ▼      ▼
      M0     M1    M2/M3
       │      │      │
       ▼      ▼      ▼
    Automate Review Escalate
```

A mandatory conformance failure MUST block the applicable gate.

### 20.3 Maintenance Metrics

Organizations implementing this standard SHOULD track:

| Metric | Description |
|---|---|
| MTTD | Mean Time to Detect |
| MTTA | Mean Time to Acknowledge |
| MTTR | Mean Time to Recovery/Repair |
| MTBF | Mean Time Between Failures |
| Patch Latency | Time from vulnerability identification to remediation |
| Dependency Freshness | Currency of dependencies |
| Technical Debt | Outstanding maintenance debt |
| Maintenance Backlog | Outstanding maintenance work |
| Rollback Success Rate | Successful rollback ratio |
| Regression Rate | Maintenance-induced regression ratio |
| Evidence Completeness | Percentage of complete evidence records |
| Recovery Readiness | Validated recovery capability |

Metrics MUST NOT be used to conceal individual critical failures.

## 21. Review Chain

This standard MUST NOT receive the status APPROVED or STABLE without the defined review chain
(ATC-STD-000 §14). The minimal review chain:

```
Author -> Technical Review -> Security Review -> Architecture Review -> Approval (Owner, §9)
```

Status flow (§25): DRAFT -> REVIEW -> CANDIDATE -> APPROVED -> STABLE.

## 22. Lifecycle / Status

This standard follows the lifecycle defined by "ATC-STD-000":

```
IDEA
  ↓
PROPOSED
  ↓
DRAFT
  ↓
REVIEW
  ↓
CANDIDATE
  ↓
APPROVED
  ↓
STABLE
  ↓
DEPRECATED
  ↓
RETIRED
```

The identifier "ATC-STD-MAINT-000" MUST remain immutable throughout its lifecycle.

Version changes MUST NOT change the standard identifier.

### 22.1 Version and Registry Separation (normative)

Document version and lifecycle status are independent axes. Three levels are distinguished:

```
DOCUMENT VERSION                 REGISTRY RECORD
ATC-STD-MAINT-000                registry_version: 1
version: 1.0.0                   standard_version: 1.0.0
status: DRAFT                    status: DRAFT
```

Status flow (version unchanged):

```
ATC-STD-MAINT-000
1.0.0 -> DRAFT
      ↓
1.0.0 -> REVIEW
      ↓
1.0.0 -> CANDIDATE
      ↓
1.0.0 -> APPROVED
      ↓
1.0.0 -> STABLE
```

- The version does not change merely because the governance status changes.
- A new SemVer version arises only from a normative change under the rules of ATC-STD-000.
- The registry record carries its own `registry_version` (record format version), separate from
  the standard version; the registry MUST mirror BOTH the standard version AND the lifecycle
  status at all times (divergence is a registry drift).
- Version 1.0.0 of this document is APPROVED per Sammelfreigabe SCR-0124 (Owner-Direktive
  14.09.2026, ATC-STD-000 §9); Immutabilitaet per §30.

## 23. GSEPF Integration

ATC-STD-MAINT-000 defines the normative Maintenance Governance layer.

GSEPF provides the governed execution framework.

The conceptual relationship is:

```
ATC Standards
      │
      ▼
     GSEPF
      │
      ├── Development Governance
      ├── Security Governance
      ├── Release Governance
      └── Maintenance Governance
                         │
                         ▼
                Maintenance Engine
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
             M0         M1        M2/M3
              │          │          │
              ▼          ▼          ▼
           Automate    Review     Escalate
                         │
                         ▼
                    Evidence
                         │
                         ▼
                  VERIFIED STATE
```

GSEPF MUST NOT weaken the normative requirements of ATC-STD-MAINT-000.

## 24. Ecosystem Integration

The maintenance framework integrates with the ecosystem's platforms. KAI-OS maintenance
MUST respect the architectural boundaries of its constituent systems.

Relevant maintenance domains include:

- ATCLang
- ShivaCore
- A-TownChain
- ATC-VM
- Aurora AI
- GlobusOS
- hardware and firmware interfaces

Maintenance crossing these boundaries MUST evaluate interface and compatibility impact.

Particularly critical boundaries include:

- ShivaCore ↔ GlobusOS
- GlobusOS ↔ Aurora AI
- ATCLang ↔ ATC-VM
- ATC-VM ↔ A-TownChain
- A-TownChain ↔ Node/Network
- Node ↔ Infrastructure

## 25. Family Definition

The following standards are assigned to the "ATC-STD-MAINT-*" family (role overview):

| ID | Role |
|---|---|
| ATC-STD-MAINT-000 | Governance (this standard: contract layer only) |
| ATC-STD-MAINT-001 | Classification |
| ATC-STD-MAINT-002 | Lifecycle |
| ATC-STD-MAINT-003 | Code Maintenance |
| ATC-STD-MAINT-004 | Dependency Maintenance |
| ATC-STD-MAINT-005 | Security Maintenance |
| ATC-STD-MAINT-006 | Infrastructure Maintenance |
| ATC-STD-MAINT-007 | OS Maintenance |
| ATC-STD-MAINT-008 | Blockchain Maintenance |
| ATC-STD-MAINT-009 | VM & Runtime Maintenance |
| ATC-STD-MAINT-010 | AI Maintenance |
| ATC-STD-MAINT-011 | Repository Maintenance |
| ATC-STD-MAINT-012 | Standards Maintenance |
| ATC-STD-MAINT-013 | Documentation Maintenance |
| ATC-STD-MAINT-014 | Performance Maintenance |
| ATC-STD-MAINT-015 | Reliability Maintenance |
| ATC-STD-MAINT-016 | Compatibility Maintenance |
| ATC-STD-MAINT-017 | Upgrade & Migration |
| ATC-STD-MAINT-018 | Rollback & Recovery |
| ATC-STD-MAINT-019 | Maintenance Evidence |
| ATC-STD-MAINT-020 | Maintenance Automation |
| ATC-STD-MAINT-021 | Emergency Maintenance |
| ATC-STD-MAINT-022 | End-of-Life / Retirement |
| ATC-STD-MAINT-023 | Vendor & Supply-Chain Maintenance |
| ATC-STD-MAINT-024 | Cross-Ecosystem Maintenance |

This standard defines only the governance contract of these standards; their technical procedures
belong to the specialized standards themselves.

Domain grouping of the family:

- Governance: MAINT-000..002
- Technical Maintenance: MAINT-003..010
- Operational Maintenance: MAINT-011..018
- Evidence & Control: MAINT-019..024

Governance model:

```
ATC-STD-000
                         │
                         ▼
              MAINT-000 Governance
                         │
             ┌───────────┴───────────┐
             ▼                       ▼
       Requirements             Maintenance
       & Governance               Family
                                  │
        ┌─────────────────────────┼──────────────────────┐
        ▼                         ▼                      ▼
   Technical                Operational             Evidence
   Maintenance              Maintenance             & Control
        │                         │                      │
   MAINT-003..010           MAINT-011..018         MAINT-019..024
        │                         │                      │
        └─────────────────────────┼──────────────────────┘
                                  ▼
                               GSEPF
                                  │
                                  ▼
                         Maintenance Engine
                                  │
                                  ▼
                           CI / Automation
                                  │
                                  ▼
                              Evidence
                                  │
                                  ▼
                          VERIFIED STATE
```


| ID | Standard |
|---|---|
| ATC-STD-MAINT-000 | Maintenance Governance |
| ATC-STD-MAINT-001 | Maintenance Classification |
| ATC-STD-MAINT-002 | Maintenance Lifecycle |
| ATC-STD-MAINT-003 | Code Maintenance |
| ATC-STD-MAINT-004 | Dependency Maintenance |
| ATC-STD-MAINT-005 | Security Maintenance |
| ATC-STD-MAINT-006 | Infrastructure Maintenance |
| ATC-STD-MAINT-007 | OS Maintenance |
| ATC-STD-MAINT-008 | Blockchain Maintenance |
| ATC-STD-MAINT-009 | VM & Runtime Maintenance |
| ATC-STD-MAINT-010 | AI Maintenance |
| ATC-STD-MAINT-011 | Repository Maintenance |
| ATC-STD-MAINT-012 | Standards Maintenance |
| ATC-STD-MAINT-013 | Documentation Maintenance |
| ATC-STD-MAINT-014 | Performance Maintenance |
| ATC-STD-MAINT-015 | Reliability Maintenance |
| ATC-STD-MAINT-016 | Compatibility Maintenance |
| ATC-STD-MAINT-017 | Upgrade & Migration |
| ATC-STD-MAINT-018 | Rollback & Recovery |
| ATC-STD-MAINT-019 | Maintenance Evidence |
| ATC-STD-MAINT-020 | Maintenance Automation |
| ATC-STD-MAINT-021 | Emergency Maintenance |
| ATC-STD-MAINT-022 | End-of-Life / Retirement |
| ATC-STD-MAINT-023 | Vendor & Supply-Chain Maintenance |
| ATC-STD-MAINT-024 | Cross-Ecosystem Maintenance |

Reserved identifiers MUST NOT be assigned to unrelated standards.

### 25.1 Relationship to ATC-STD-REPO-MAINT-001 (FAM-46)

ATC-STD-REPO-MAINT-001 (Repository Maintenance & Lifecycle Standard, family FAM-46) is a
pre-existing specialized Repository Domain Standard. It is NOT replaced by this family:

```
MAINT
                     │
          ┌──────────┴──────────┐
          │                     │
   General Maintenance     Repository Domain
          │                     │
   MAINT-000..024        REPO-MAINT-001
          │
          └────── governs ──────►
```

Normative relationship:

- REPO-MAINT-001 remains the repository-maintenance domain SSOT and is governed by this family
  (FAM-52 references FAM-46 via `family_refs`).
- A long-term consolidation into MAINT-011 (Repository Maintenance) is possible only via a
  controlled migration path under SCR; standard identifiers are immutable (ATC-STD-000 §30) —
  REPO-MAINT-001 MUST NOT simply be renamed.
- Until such a migration is approved, REPO-MAINT-001 is referenced as a specialized domain
  standard under MAINT governance (§13.2).

## References

Normative:

- "ATC-STD-000" — Standards Governance & Specification Standard
- applicable "ATC-STD-MAINT-*" specialized standards
- applicable GSEPF specifications
- applicable repository, security, OS, blockchain, AI, and architecture standards

## 26. Changelog

v1.0.0 — Draft — 2026-09-13

- Initial normative Maintenance Governance framework.
- Established "ATC-STD-MAINT-*" as a dedicated standard family.
- Established M0–M3 classification.
- Established Maintenance Lifecycle.
- Established Maintenance Readiness Gate.
- Established Maintenance Capability as a release prerequisite.
- Established Maintenance Evidence requirements.
- Established M2/M3 Separation of Duties.
- Established machine-readable conformance requirements.
- Established GSEPF integration.
- Established KAI-OS cross-layer maintenance requirements.
- Included MAINT-021 through MAINT-024 as committed family members (Emergency, End-of-Life,
  Vendor & Supply-Chain, Cross-Ecosystem).
2026-09-14 — v7 contract freeze (Owner-Direktive 08:23): restructured to the final 26-section
contract layout; Requirements moved to §6 with YAML definitions (statement, priority, verification
method) per requirement; version/registry three-level separation made normative (§22.1, registry
record carries its own registry_version); family role table and tri-domain governance model added
(§25); Minimum Release Requirement consolidated into §8.1. Role frozen: ATC-STD-MAINT-000 is the
normative Governance and Contract Layer of the entire Maintenance Family — no further scope growth.
