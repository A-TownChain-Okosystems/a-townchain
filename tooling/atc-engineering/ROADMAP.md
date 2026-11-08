# ATC Engineering Roadmap

## Vision

Build the engineering and governance control plane used to develop, validate, audit and release the complete A-TownChain ecosystem.

---

## Current State — 2026-09-14

**Implementation level:** `IMPLEMENTED (Phase 0, partial) + Phase 1 partial`.

Evidence in `STATUS.md` is authoritative for what actually exists. The roadmap describes target capabilities and must not be interpreted as an implementation claim.

Implemented on main:

- `atc-core`
- `atc-config`
- `atc-standards` registry client
- `atc-maintenance`
- Rust workspace and baseline CI structure
- Standards lifecycle validation and normative/released checks
- Standards version-pinning primitives

Not yet implemented:

- `atc-engine` CLI
- Policy/audit/evidence engines
- GitHub App integration
- Most target crates from the v1.0 architecture
- Live CI enforcement for the complete workspace

---

## Phase 0 — Foundation

**Target: v0.1.x**

- [x] Create Rust workspace
- [x] Establish repository governance
- [x] Implement `atc-core`
- [x] Implement `atc-config`
- [x] Establish schema conventions
- [x] Establish error model
- [x] Establish initial documentation
- [x] Establish security baseline
- [ ] Complete CI enforcement
- [ ] Complete logging/tracing platform

### Exit Criteria

Phase 0 is **partially complete**. Build and local test evidence exists; production CI enforcement remains open.

---

## Phase 1 — Standards Integration

**Target: v0.2.x**

- [x] Integrate `atc-standards` registry client
- [x] Implement standard resolver
- [x] Implement lifecycle and normative/released validation
- [x] Implement standards version-pinning primitives
- [ ] Implement requirement resolver (`REQ-STD-XXX-NNN`)
- [ ] Implement complete standards validation workflow
- [ ] Implement machine-readable requirement mapping
- [ ] Implement conformance reports

### Exit Criteria

A repository can be validated against selected ATC standards automatically, with complete requirement-level evidence.

---

## Phase 2 — Repository Intelligence

**Target: v0.3.x**

- [ ] Repository discovery
- [ ] Repository metadata
- [ ] Repository classification
- [ ] File inventory
- [ ] Repository lifecycle
- [ ] Ownership metadata
- [ ] Dependency discovery
- [ ] Repository health reporting

### Exit Criteria

The organization repository fleet can be discovered and evaluated programmatically.

---

## Phase 3 — Policy & Governance Engine

**Target: v0.4.x**

- [ ] Policy schema
- [ ] Policy resolver
- [ ] Policy evaluator
- [ ] Governance roles
- [ ] Separation of duties
- [ ] Governance gates
- [ ] Exceptions
- [ ] Expiration handling
- [ ] Fail-closed enforcement

---

## Phase 4 — Audit & Evidence

**Target: v0.5.x**

- [ ] Audit engine
- [ ] Evidence schema
- [ ] Evidence collection
- [ ] Evidence verification
- [ ] Evidence export
- [ ] Provenance
- [ ] Evidence integrity
- [ ] Derived state engine

---

## Phase 5 — GitHub Integration

**Target: v0.6.x**

- [ ] GitHub App integration
- [ ] Repository discovery
- [ ] Commit integration
- [ ] Pull request integration
- [ ] Checks integration
- [ ] Actions integration
- [ ] Release integration
- [ ] Controlled write operations

---

## Phase 6 — Security Engineering

**Target: v0.7.x**

- [ ] Dependency auditing
- [ ] Secret detection
- [ ] SBOM
- [ ] SAST integration
- [ ] License validation
- [ ] Artifact hashing
- [ ] Supply-chain validation
- [ ] Security evidence

---

## Phase 7 — Repository Bootstrap

**Target: v0.8.x**

- [ ] Repository templates
- [ ] Governance templates
- [ ] GitHub workflow generation
- [ ] Documentation generation
- [ ] CODEOWNERS generation
- [ ] Standards mapping generation
- [ ] Repository initialization CLI

---

## Phase 8 — Release Engineering

**Target: v0.9.x**

- [ ] Build orchestration
- [ ] Release candidate generation
- [ ] Release gates
- [ ] Artifact verification
- [ ] Provenance
- [ ] Approval workflow
- [ ] Release promotion
- [ ] Devnet release support
- [ ] Testnet release support
- [ ] Mainnet release support

---

## Phase 9 — AI Engineering Governance

**Target: v0.10.x**

- [ ] Agent identity
- [ ] Agent scopes
- [ ] Agent authorization
- [ ] Agent task model
- [ ] Agent evidence
- [ ] Agent audit
- [ ] Human approval boundaries
- [ ] AI engineering workflow enforcement

---

## Phase 10 — Production Platform

**Target: v1.0.0**

- [ ] Stable domain APIs
- [ ] Stable schemas
- [ ] Stable CLI
- [ ] Production GitHub integration
- [ ] Organization-wide governance
- [ ] Production release gates
- [ ] Evidence-backed readiness state
- [ ] Security hardening
- [ ] Reproducible builds
- [ ] Complete operational documentation

### v1.0 Definition

ATC Engineering v1.0 is production-ready only when it can govern and validate the complete applicable A-TownChain repository lifecycle without relying on undocumented manual steps.

---

## Long-Term

Potential future capabilities:

- Distributed workers
- Remote execution
- Web dashboard
- Multi-organization governance
- Advanced technology radar
- Reproducible-build verification
- Artifact transparency
- Enterprise federation
- Advanced AI engineering orchestration
