---
document_id: ATC-DOC-TOWNCHAIN-004
title: "Project Status"
version: 1.2.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-11-19
standard: ATC-STD-MD-001
---

# Project Status — ATC A-TownChain Core

| Property | Value |
|---|---|
| Repository | a-townchain |
| Version | 0.1.0 |
| Status | development |
| Implementation | partial / prototype |
| Build | exact-SHA evidence exists for individual gates; no release-candidate green SHA |
| Tests | individual gates pass; complete launch suite not verified |
| Security | NOT AUDITED |
| Conformance | IN PROGRESS |
| Production Readiness | NO-GO |
| Mainnet Date | NONE APPROVED |
| Current Documentation Update | 2026-11-19 |

## Stand 19.11.2026 (verifiziert)

| Vorgang | Ergebnis |
|---|---|
| SCR-0148 Fehlerwelle | 8 Codefehler behoben (apps/atc-ide 7 TypeScript, atc-governance 1 Warnung); tsc 0 Fehler, vite build gruen |
| Produktionsabhaengigkeiten | 13 Live-Pakete: 0 critical, 0 high; Build-Werkzeuge aus `dependencies` entfernt |
| Kritische Advisories | 11 -> 5 im Repo; die verbleibenden fuenf sind gepatcht und fallen beim Merge von PR #118 |
| ATC-OS-SEC-UPDATE-001 | §9-Freigabe 18.11.2026, APPROVED, normativ wirksam; Implementierungsstatus `specification_only` (TUF-Rollen, Health-Check, Rollback-Orchestrierung, koordinierte Aktivierung offen) |
| Registry | 534 Standards (507 APPROVED / 27 CANDIDATE), Validator ALL COMPLIANT, Qualitaets-Gate Ebene 1-3 PASS |
| CI PR #118 | 20 gruen, 1 uebersprungen, CodeQL rot: 58 Befunde ausschliesslich in `docs/os-docs/**`; der `init`-Schritt laedt `.github/codeql/codeql-config.yml` nicht, es fehlt `config-file` und dafuer `workflow`-Scope |

Unveraendert: Security NOT AUDITED, Production Readiness NO-GO, kein
release-candidate-gruener SHA. Die Angaben oben sind Zwischenstaende, keine
Freigabe.

## Status Summary

a-townchain remains in development. The core contains working components and verified development/test gates, but the complete L1 mainnet path is not yet implemented and evidenced as one production release.

The canonical readiness source is docs/MAINNET_READINESS.md.

## P0 progress

The current crypto P0 work on PR #32 establishes the transaction/account trusted core around secp256k1 ECDSA, SHA-256, RFC6979 and Low-S. The canonical transaction encoding now uses u128 for economic fields and u64 for counters/resource limits.

This work is NOT yet a mainnet release. SDK/Node integration, Rust↔TypeScript conformance, consensus-key contract, mainnet genesis, production networking, state recovery, VM conformance, independent security audit and the final exact-SHA release gate remain open.

Purpose separation is mandatory: Transaction Key ≠ Consensus Key ≠ Node Identity ≠ Service Identity ≠ Aurora Agent Identity.

Ed25519 is not globally removed; it remains available only where explicitly defined by a purpose-bound identity contract.

## Integrationspfad (11.11.2026)

`components/system` verbindet Quelle, Compiler, Artefakt, Freigabe, VM und Block zu einem
Pfad. Die Stufen werden getrennt gefuehrt und nicht vermischt:

| Stufe | Stand |
|---|---|
| IMPLEMENTED | ja, Crate `atc-system` im Workspace |
| TESTED (lokal) | 46 Tests im Crate, 662 im Workspace, 0 Fehler, clippy/fmt sauber |
| GATES A-E | nachgewiesen, siehe `docs/audits/SYSTEM-INTEGRATION-2026-11-11.md` |
| CI_VERIFIED | **offen** (kein Actions-Lauf, Committer-Mail unbetaetigt) |
| E2E_VERIFIED | **offen** (ein Prozess, keine zwei Validatoren) |
| VERIFIED | **nicht erreicht** |

Bekannte Abweichungen, die ausdruecklich offen bleiben: Ueberlauf und Vorzeichen
(Entwurf in `docs/decisions/VM-INTEGER-SEMANTICS-DRAFT.md`), keine Aufrufkonvention,
Lokale als Speicherplaetze, kein Nonce, kein Gasverbrauch im Beleg, keine persistente
Authority. Lokale Tests sind `TESTED`, nie `CI_VERIFIED`.

## Mainnet release gate

Mainnet remains NO-GO until all mandatory gates reach RELEASED on one immutable release-candidate SHA.

Minimum gates:

1. canonical monetary and transaction contracts
2. frozen consensus/finality and consensus-key contract
3. reproducible mainnet genesis/bootstrap
4. production P2P and multi-node operation
5. state sync, restart and recovery
6. deterministic ATC-VM execution/state-transition conformance
7. Rust↔TypeScript↔Node transaction conformance
8. independent security audit and remediation
9. reproducible signed release artifacts
10. complete Wallet → SDK → Node → Mempool → Consensus → VM → State → Storage path
11. exact-SHA evidence bundle

Historical CI results or source-file presence do not satisfy the release gate.

## Evidence rule

SPECIFIED → IMPLEMENTED → TESTED → VERIFIED → AUDITED → RELEASED

**No evidence, no trust.**

For the detailed gate matrix, see docs/MAINNET_READINESS.md.
