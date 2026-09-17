---
spec_id: WAL-TB-001
title: "Wallet Trust Boundary Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-wallet
layer: L5-Wallet
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Wallet Trust Boundary Specification (WAL-TB-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Die verbindliche Grenze: Rust Trusted Core (Keys/Signing/Serialization/Replay/Secure Storage) vs. UI/Tooling (Python, GUI) außerhalb.

## 2. Scope (gilt für)

- Trusted-Core-API (eingehend)
- Verbotene Zugriffe
- Hardware-Schnittstelle (PLANNED)

## 3. Normative Anforderungen (MUST)

- **REQ-WTB-001:** Trusted Core = Rust: Key-Management, Signing, kanonische Serialisierung, Replay, Secure Storage (per WAL-SEC-001) — *Nachweis: architecture*
- **REQ-WTB-002:** Python/UI/externe Services sehen NIE private Keys und signieren NIE; Kommunikation ausschließlich über eine schmale, versionierte FFI-API mit Fehlercodes (kein Panik-Durchschlag) — *Nachweis: architecture+negative*
- **REQ-WTB-003:** Hardware-Wallet-Integration ist PLANNED (WAL-HW-001 folgt); bis zur Implementierung darf keine Hardware-Fähigkeit behauptet werden (Capability-Claim-Bereinigung) — *Nachweis: governance+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Keine Ausführung von Signatur- oder Schlüsselpfaden außerhalb des Trusted Core

## 6. Conformance-Tests (Mindestkategorien)

- boundary_static_check (kein Python-Modul importiert crypto-signing direkt)
- api_surface_test

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-1 Python+Rust-Trennung; P1-4 Hardware-Claims)
