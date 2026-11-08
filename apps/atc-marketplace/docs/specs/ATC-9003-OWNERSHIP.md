---
spec_id: ATC-9003
title: "Asset Ownership Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-marketplace
layer: L5-Marketplace
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Asset Ownership Specification (ATC-9003)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindliche Ownership-Semantik (inklusive Locking während Sales).

## 2. Scope (gilt für)

- Owner-Zustandsübergänge
- Lock/Unlock (Sale-gebunden)
- Equivocation-Schutz

## 3. Normative Anforderungen (MUST)

- **REQ-A93-001:** Ownership-Übergänge: MINT→owner, TRANSFER→new_owner, BURN→null; illegale Übergänge ⇒ Reject (endliche Zustandsmaschine) — *Nachweis: unit+negative*
- **REQ-A93-002:** Lock: während einer aktiven Order ist Transfer gesperrt (Reflexivität: Lock→Unlock nur über Settlement/Cancel) — *Nachweis: unit+adversarial*
- **REQ-A93-003:** Doppeltransfer desselben Assets in derselben Höhe ⇒ genau einer gewinnt (Determinismus + Nonce/Order) — *Nachweis: adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Jedes Asset hat maximal einen Owner (kein geteiltes Eigentum auf dieser Ebene)

## 6. Conformance-Tests (Mindestkategorien)

- ownership_matrix.json
- double_transfer.json
- lock_bypass ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (unauthorized transfer)
