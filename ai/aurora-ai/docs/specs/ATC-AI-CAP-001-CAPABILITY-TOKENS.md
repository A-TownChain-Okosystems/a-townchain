---
spec_id: ATC-AI-CAP-001
title: "Capability Token Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: aurora-ai
layer: L2-AI
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Capability Token Specification (ATC-AI-CAP-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Das verbindliche Capability-Token-Modell: Aurora darf anfragen — Policy entscheidet — Kernel führt aus.

## 2. Scope (gilt für)

- Token-Felder & Signatur
- Delegation & Revocation
- Replay-Schutz
- Kernel-Enforcement

## 3. Normative Anforderungen (MUST)

- **REQ-CAP-001:** Capability-Token: {id, issuer, subject (Agent-ID), capability (z. B. fs.read, net.connect), resource, scope, expires_at, nonce, audience, approval_ref, signature, audit_id} — Feldliste ist normativ — *Nachweis: unit+vector*
- **REQ-CAP-002:** Delegation nur mit dokumentierten Regeln (max Tiefe, Scope-Verengung Pflicht); Revocation über Revocation-Registry, geprüft je Ausführung — *Nachweis: negative+adversarial*
- **REQ-CAP-003:** Replay-Schutz: (nonce, audience) ist bindend; Wiederverwendung ⇒ Reject — *Nachweis: negative*
- **REQ-CAP-004:** Kernel-Enforcement: ShivaCore prüft Capability je Syscall (K3a-Kopplung); Userspace/AI kann Tokens nicht selbst „erfüllen“ — *Nachweis: architecture+adversarial*
- **REQ-CAP-005:** Human-Approval-Pflicht für definierte Risiko-Klassen (gelistet in der Spec, initial: alle Kernel-Write-Capabilities) — *Nachweis: governance*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Keine Kernel-Wirkung ohne gültiges, unrevokiertes, nicht abgelaufenes Token

## 6. Conformance-Tests (Mindestkategorien)

- cap_vectors.json
- cap_replay ⇒ Reject
- cap_forgery ⇒ Reject
- delegation_depth.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 09.09. (P1-8 Capability-Token-Semantik)
- ShivaCore K3a (Capability IPC)
