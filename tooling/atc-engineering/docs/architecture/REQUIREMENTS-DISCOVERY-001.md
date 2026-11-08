# REQUIREMENTS-DISCOVERY-001 — Target Software Requirements Discovery

**Status:** IMPLEMENTED (initial deterministic engine)

## Purpose

`atc-engineering` must inspect the software being built and derive an explicit list of what that target requires. The result is a **requirements profile**, not a declaration that the target is complete.

The engine answers:

1. Which technologies and execution models are observable?
2. Which mandatory software components are required?
3. Which non-code contents are required?
4. Which tests and verification layers are required?
5. Which documentation and governance artifacts are required?
6. Which security and release controls are required?
7. Which requirements are still missing?
8. If ATCLang is detected, are detection, creation, editing and canonical conversion/compilation workflows required?

## Evidence rule

A detected file is only a signal. It is not proof that the corresponding subsystem is correctly implemented.

```text
Target software
      ↓
Observable repository signals
      ↓
Target profile
      ↓
Required components/content/tests/docs/security/release controls
      ↓
Missing requirements
      ↓
Implementation plan
```

This preserves the engineering principle **No Evidence, No Trust**.

## Requirement classes

| ID family | Meaning |
|---|---|
| `COMP-*` | Runtime and architectural components |
| `CONT-*` | Required specifications and inventories |
| `TEST-*` | Verification and negative/fail-closed testing |
| `DOC-*` | Architecture and operations documentation |
| `GOV-*` | Ownership and change control |
| `SEC-*` | Security and supply-chain controls |
| `REL-*` | Reproducible build and release evidence |
| `TECH-*` | Technology-specific enforcement |

## ATCLang discovery

ATCLang artifacts are recognized recursively by the extensions:

- `.atc` — ATCLang source artifacts.
- `.aes` — ATCLang ecosystem artifact class; exact semantics remain governed by the ATCLang specification.
- `.atvm` — ATC-VM artifact/bytecode class; conversion must use the canonical compiler/VM contract rather than treating a binary as ordinary text.

When an ATCLang artifact is detected, the planner derives explicit requirements for:

- **Detection** — classify ATCLang artifacts and their role.
- **Creation** — provide a governed mechanism for producing valid artifacts.
- **Editing** — support structured, validation-aware source/metadata changes.
- **Conversion/compilation** — route source through the canonical ATCLang compiler toward the supported VM artifact format without inventing semantics.
- **Testing** — syntax, semantic, bytecode and applicable round-trip tests.
- **Documentation** — versioned artifact and conversion contracts.

The detector intentionally does not claim that any of these workflows are already implemented merely because an extension exists.

## Current discovery signals

The initial implementation detects Rust, Python, JavaScript, TypeScript, shell automation, Docker and GitHub Actions plus recursive ATCLang `.atc`, `.aes` and `.atvm` artifacts. More detectors must be added for databases, network protocols, native toolchains, OS/kernel targets, blockchain nodes, smart contracts, AI/ML runtimes, storage, cryptography and hardware dependencies.

## CLI

```bash
cargo run --release -p atc-audit-cli -- /path/to/target --requirements
```

The command exits non-zero when the derived target profile still contains missing mandatory requirements.

## Non-goals

The discovery engine does not infer correctness from names, README claims, directory presence or commit messages. It also does not automatically mark a missing requirement as implemented.

## Next implementation layers

1. Standards Registry → requirement/control mapping.
2. Recursive source and manifest capability detection.
3. Component/dependency graph extraction.
4. Requirement-to-evidence traceability.
5. Repository-specific profiles and target classes.
6. Actual ATCLang create/edit/compile adapters backed by the canonical `atclang` toolchain.
7. Automatic implementation task generation.
8. Re-audit and requirement closure verification.
