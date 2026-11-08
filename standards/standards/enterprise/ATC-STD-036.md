---
standard:
  id: ATC-STD-036
  title: "Emergency Security Change Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories und die Organisation gesamt"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
----

# ATC-STD-036 — Emergency Security Change Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-036 · Notfallpfad für sicherheitskritische Änderungen · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-034" — §37 ergab 036.

## Abstract

ATC-STD-036 definiert den Emergency-Pfad für sicherheitskritische Änderungen:
volle Rückverfolgbarkeit bei reduzierter Prozessschwere — normale Governance
für normale Änderungen, Notfallpfad nur für kritische Schwachstellen.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für Änderungen unter Critical-Finding-/Active-Exploit-Bedingungen.
Normale Änderungen durchlaufen die Standard-Governance (ATC-STD-000 §14).

## §1 Notfallpfad (REQ-STD-001, MUST)

Critical Finding → Emergency Assessment → Security Authorization → Minimal
Safe Change → Automated Verification → Emergency Release → Ex-Post Review.

Der Pfad MUSS die Normalstufen (Technical/Security/Architecture Review)
zusammenfassen, NICHT auslassen; die Security Authorization MUSS durch den
Owner bzw. die autorisierte Rolle erfolgen.

## §2 Minimale Sichere Änderung (REQ-STD-002, MUST)

Die Änderung MUSS minimal sein (kleinster sicherheitserreichender Eingriff);
Feature-Erweiterungen DARF der Emergency-Pfad NICHT transportieren.

## §3 Rückverfolgbarkeit (REQ-STD-003, MUST)

Jede Emergency Change MUSS vollständig rückverfolgbar sein: Emergency-Record
(Trigger, Authorization, Change, Verification, Release, Ex-Post-Review) unter
evidence/releases/ bzw. evidence/security/; Ex-Post Review MUSS innerhalb
der Review-Frist nach Release erfolgen und die Änderung gegen die normale
Governance nachbewerten.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — 7-Schritt-Notfallpfad MUSS mit Security Authorization durchlaufen werden.
- id: REQ-STD-002 — Minimal-Safe-Change-Prinzip MUSS gelten; keine Feature-Erweiterungen im Emergency-Pfad.
- id: REQ-STD-003 — Rückverfolgbarkeit + Ex-Post Review MÜSSEN vollständig sein.

## Compliance

Prüfung: Emergency-Records, Ex-Post-Review-Fristen, Git-History (Task-
Identity-Trailer).

## Security Considerations

- Emergency-Bypass ist Angriffsziel: Authorization MUSS zweistellig geprüft
  (Aussteller + Owner) und bei Missbrauch als P0 gewertet werden.
- Ex-Post Review MUSS prüfen, ob die Änderung dauerhaft in die normale
  Codebasis übernommen/abgelöst wurde.

## Implementierungsstatus

**Status: SPECIFIED** — Anwendung im Ernstfall (ATC-EXC-001 bildet bereits
einen Vorläufer mit Ablaufdatum als Governance-Ausgleich). SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) —
  Notfallpfad, Minimal-Safe-Change, Rückverfolgbarkeit, Ex-Post Review. CANDIDATE.

## References

- ATC-STD-000 §14 — Rollen/Delegation · ATC-STD-020/022/035 — Trigger
- ATC-GATE-SEC-001 — Release-Gate-Kopplung
