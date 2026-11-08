---
standard:
  id: ATC-STD-024
  title: "Known Bug Management Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
    - ATC-STD-026
----

# ATC-STD-024 — Known Bug Management (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Continuous Assurance 11.09.2026
> (SCR-0094): separater Known-Bug-Lifecycle und Root-Cause-Pflicht — nicht jeder
> Fehler ist eine Security Vulnerability. Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-024 · Bug-Lifecycle + RCA · **Governance:** ATC-STD-000

## Abstract

ATC-STD-024 definiert einen eigenständigen Lifecycle für Known Bugs (abseits der
Vulnerability-Prozesse) und die Root-Cause-Pflicht nach kritischen Fehlern: Jeder
kritische Fehler MUSS das Standardsystem verbessern (Symptom → Technical Cause →
Process Cause → Systemic Cause → Preventive Control).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für bestätigte Software-Fehler. Security-Schwachstellen laufen über
ATC-STD-018/020/022; Security-Komponente eines Bugs MUSS dorthin eskaliert werden.

## §1 Known-Bug-Lifecycle (REQ-STD-001, MUST)

DISCOVERED → CONFIRMED → CLASSIFIED → PRIORITIZED → ASSIGNED → FIXED → TESTED →
VERIFIED → CLOSED. Kein Schritt DARF übersprungen werden; Wiederverwendung
(CLOSED ohne VERIFIED) IST verboten.

## §2 Bug-Record (REQ-STD-002, MUST)

Jeder Bug MUSS einen maschinenlesbaren Record mit mindestens folgenden Feldern
führen: BUG-ID, Severity, Affected Version, Affected Component, Root Cause,
Reproduction, Fix, Regression Test, Verification Evidence.

## §3 Root-Cause-Pflicht (REQ-STD-003, MUST)

Nach kritischen Fehlern (P0/SEV-1/2) MUSS eine Root-Cause-Analyse entlang der
Kette erfolgen:

Symptom → Failure → Technical Cause → Process Cause → Systemic Cause →
Preventive Control

Beispiel: Memory corruption ← unsafe implementation ← missing bounds validation ←
missing security requirement ← missing standard ← new validation rule.

Systemic Cause MUSS in ein Preventive Control münden: neuer/amended Standard
(SCR), Validator-Regel oder Regressionstest (ATC-STD-026). Bug beheben ≠ Problem
vollständig behoben.

## §4 Kopplung Error-Knowledge (REQ-STD-004, MUST)

Kritische Fehler MÜSSEN zusätzlich als Error-Knowledge-Record nach der
ATC-STD-ERR-Familie (ERR-000: kein lokaler Fix ohne Systemverifikation)
dokumentiert werden; Muster-Wiederverwendung über die Error-Pattern-Bibliothek.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Bug-Lifecycle: 9-Status-Kette MUSS vollständig durchlaufen werden.
- id: REQ-STD-002 — Bug-Record: 9 Pflichtfelder MÜSSEN je Bug geführt werden.
- id: REQ-STD-003 — Root-Cause-Pflicht: RCA-Kette MUSS nach kritischen Fehlern bis zum Preventive Control führen.
- id: REQ-STD-004 — Error-Knowledge-Kopplung: kritische Fehler MÜSSEN ERR-Records erzeugen.

## Compliance

Prüfung: Bug-Records (je Repo bzw. org-zentral), RCA-Verweise auf SCR/
Standard-Änderung; Audit gegen Lifecycle-Vollständigkeit.

## Security Considerations

- Bug-Triage MUSS auf Security-Relevanz prüfen (Fehl-Klassifikation = verdeckte
  Vulnerability).
- Bug-Records mit Reproduktionsschritten MÜSSEN zugriffsbeschränkt sein, solange
  nicht gepatcht (responsible disclosure, ATC-STD-020 §5).

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Evidence ab Bug-Records-Betrieb.
SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Continuous Assurance (SCR-0094) — Bug-Lifecycle,
  Record-Schema, Root-Cause-Pflicht, ERR-Kopplung. CANDIDATE.

## References

- ATC-STD-000 — Governance Root
- ATC-STD-018 — Vulnerability Management (Abgrenzung)
- ATC-STD-020 — Incident & Vulnerability Response
- ATC-STD-026 — Security Regression Prevention
- ATC-STD-ERR-000..015 — Error-Knowledge-Familie
