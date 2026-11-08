# A-TownChain Ecosystem Knowledge Base

Canonical knowledge index for the current A-TownChain ecosystem.

Evidence rule: repository existence is not evidence of implementation. Implementation claims require source, tests, CI, and where applicable E2E evidence.

## Purpose

This knowledge base connects architecture, repositories, standards, code, CI/CD, E2E verification, security, AI, OS, and Genesis into one navigable model.

## Knowledge domains

1. Architecture — system layers, trust boundaries, dependencies, data flows.
2. Repository Knowledge — repository purpose, ownership, interfaces, implementation state.
3. Standards & Governance — ATC standards, SCRs, lifecycle, evidence and release authority.
4. Code Knowledge — canonical source files, APIs, invariants, compatibility boundaries.
5. CI/CD Knowledge — workflows, gates, runs, failures, artifacts and remediation history.
6. E2E Knowledge — verified paths and their exact evidence.
7. Security Knowledge — trust boundaries, threat models, controls and security evidence.
8. AI Knowledge — Aurora models, runtimes, agents, tools, skills, memory, RAG, policy and approval.
9. OS Knowledge — ShivaCore, HAL, IPC, memory, storage, TPM/TEE, secure boot and GlobusOS.
10. Genesis Knowledge — Genesis Engine, runtime, mod system, Chronicles and integration boundaries.

## Evidence states

- MISSING — no implementation/evidence found.
- STUB — placeholder or partial implementation.
- IMPLEMENTED — source implementation exists.
- TESTED — relevant automated tests pass.
- CI_VERIFIED — CI verifies the relevant behavior.
- E2E_VERIFIED — complete path is demonstrated end-to-end.
- BROKEN — implementation exists but currently fails verification.
- DISCONNECTED — implementation exists but is not connected to its required boundary.
- DUPLICATE — competing implementation exists where a canonical source is required.

Never promote a component to a stronger state without evidence.

## Canonical dependency model

Core chain:
atc-algorithm → a-townchain → atc-node → atc-vm → atc-storage → atc-indexer

Execution chain:
ATCLang → Bytecode → BytecodeVerifier → ATC-VM → State Transition → Storage

Financial chain:
Wallet → SDK → Node → VM → State → DEX/Marketplace/Launchpad/DAO/Treasury

OS / AI chain:
Hardware → ShivaCore → A-TownChain OS → GlobusOS → Aurora

Genesis chain:
Aurora → GlobusOS → Genesis Engine → Genesis Chronicles

Governance chain:
atc-standards → atc-engineering → CI/Evidence → Governance → Release Gates

## Canonical economic invariant

Current implementation policy:

- Max supply: 360,000,000 ATC
- Precision: 18 decimals
- Base unit: 10^-18 ATC
- Economic values: u128
- Protocol counters such as block height, nonce and timestamp remain semantically u64
- Canonical monetary emission source: atc-algorithm::economics
- Downstream components consume the canonical schedule and must not create competing emission logic.

The corresponding governance change is tracked as SCR-0129 in atc-standards. Its lifecycle state must be read from GitHub and must not be described as approved until the governance process proves that state.

## Current repository inventory

The GitHub organization currently exposes 32 repositories. See REPOSITORIES.md for the role map. Repository count alone is not an implementation claim.

## Working method

For every integration claim:

Architecture claim → source evidence → test evidence → CI evidence → E2E evidence

For CI failures:

Run → Job → Step → Log → Root cause → Minimal fix → Commit → CI rerun

## Maintenance

This directory is a knowledge index, not a substitute for source code or CI. When a factual status changes, update the knowledge entry with the corresponding GitHub evidence.
