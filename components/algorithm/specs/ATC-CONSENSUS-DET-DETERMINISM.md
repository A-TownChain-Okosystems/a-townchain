---
spec_id: ATC-CONSENSUS-DET
title: "Consensus Determinism Contract"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-algorithm
layer: L3-Konsens
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Consensus Determinism Contract (ATC-CONSENSUS-DET)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Der Execution-Contract, der garantiert: gleicher Input ⇒ gleicher Konsensentscheid — Voraussetzung für unabhängige Implementierungen.

## 2. Scope (gilt für)

- Arithmetik
- Sortierung & Encoding
- Randomness-Verbot
- Fehlerbehandlung

## 3. Normative Anforderungen (MUST)

- **REQ-DET-001:** Nur Integer-Arithmetik (u64/u128) mit checked ops; Overflow/Underflow ⇒ Ergebnis invalid, niemals wrap — *Nachweis: unit+property*
- **REQ-DET-002:** Fließkommazahlen sind im Konsenspfad VERBOTEN (statisch prüfbar) — *Nachweis: negative*
- **REQ-DET-003:** Kanonische Sortierung: byte-lexikografisch über kanonische Serialisierung; Endianness fixiert auf Little-Endian für alle Längen-/Zahlenfelder, Hashes als 32-Byte-Rohwerte — *Nachweis: unit+vector*
- **REQ-DET-004:** Jede Quelle von Nichtdeterminismus (Wallclock, Locale, Map-Iteration-Reihenfolge, Thread-Scheduling) ist ausgeschlossen; Zeitmessung ausschließlich über PoH-Slots — *Nachweis: property+adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Zwei korrekte Implementierungen erzeugen für dieselbe Eingabe bit-identische Konsensentscheidungen (differenzierbar testbar)

## 6. Conformance-Tests (Mindestkategorien)

- differential_replay (Rust vs. Referenz-Oracle)
- integer_boundary.json
- sort_stability.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0-04 Determinismus-Spezifikation)
- ATC-CONSENSUS-ENC
