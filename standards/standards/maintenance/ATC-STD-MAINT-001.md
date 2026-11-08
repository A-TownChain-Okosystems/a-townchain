---
standard:
  id: ATC-STD-MAINT-001
  title: "ATC-STD-MAINT-001 — Maintenance Classification Standard"
  version: "1.0.0"
  status: approved
  category: maint
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-14"
  updated: "2026-09-14"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-14"
  review_date: null
  applies_to: "Alle Maintenance-Aufgaben des Oekosystems ab Finding-Anlage"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-001 — Maintenance Classification Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Die vier Klassen M0-M3 sind verbindlich.
> Dach: ATC-STD-MAINT-000; Lifecycle der Klassifizierung: MAINT-002.

## Abstract

Verbindliche Klassifizierung jeder Maintenance-Aufgabe in M0-Routine, M1-Operational,
M2-Security oder M3-Critical — mit Beispielen, SLAs und Eskalationsregeln.

## §1 Die vier Klassen (normativ)

**M0 — Routine** (normale, planbare Wartung): Dependency Updates · Dokumentationskorrekturen ·
kleinere Refactorings · CI-Optimierung · Log-/Metric-Anpassungen · nichtkritische
Performance-Optimierung. **SLA:** regulaerer Sprint / regulaerer Maintenance-Zyklus.

**M1 — Operational** (betriebsrelevante Wartung): Build-System defekt · Release-Pipeline
instabil · Performance-Regression · Node-Probleme · Storage-Probleme · Service-Ausfaelle ·
Recovery-Tests. **SLA:** priorisierte Bearbeitung.

**M2 — Security** (sicherheitsrelevante Wartung): CVE · kompromittierte Dependency ·
Supply-Chain-Risiko · Secret Leakage · Key Rotation · Sandbox Escape · Permission Bypass.
**Regel:** Security Maintenance darf normalen Release-Zyklen vorgelagert werden.

**M3 — Critical** (systemkritische Wartung): Konsensfehler · Kernel Security Boundary gebrochen ·
ATC-VM-Inkompatibilitaet · Chain-State-Korruption · Datenverlust · kritische Remote Code
Execution · kryptografischer Vertrauensbruch. **Regel:** M3 erhaelt den
Emergency-Maintenance-Pfad (ATC-STD-000 §32) und kann normale Governance-Zyklen uebersteuern.

## §2 Klassifizierungsregeln (normativ)

1. Jedes Finding/Maintenance-Item traegt seine Klasse **ab Anlage** (M0-M3).
2. Im Zweifel die HOEHERE Klasse.
3. M2 darf normalen Release-Zyklen vorgelagert werden; Disclosure-Vorsicht bei aktiv
   ausnutzbaren CVEs (Fix vor Oeffentlichkeit).
4. M3-Verdacht: SOFORT Eskalation an Owner (kein Abwarten des naechsten Zyklus);
   Emergency-Governance uebersteuert normale Zyklen.
5. Reklassifizierung ist dokumentierbar (Evidence-Record, MAINT-019).

## §3 Gates je Klasse (Verweis MAINT-000 §9 Classification)

M0 → Standard Review · M1 → Operational Review · M2 → Security Review (mit Separation of
Duties) · M3 → Emergency Governance (mit Release Authority ≠ Implementer).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-016 | Klasse ist ab Anlage eines jeden Maintenance-Items pflichtig | MUST |
| REQ-MAINT-017 | Im Zweifel gilt die hoehere Klasse | MUST |
| REQ-MAINT-018 | M2 darf Release-Zyklen vorlagern | MUST |
| REQ-MAINT-019 | M3 folgt dem Emergency-Pfad (ATC-STD-000 §32) und uebersteuert normale Zyklen | MUST |
| REQ-MAINT-020 | M2/M3 erzeugen Separation-of-Duties-Pflicht (MAINT-000 §5.6 Separation of Duties) | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Die Klassifizierung ist konzeptionell in bestehenden Ablaeufen ueber
BUG/UPDATE-Kategorien teilweise gelebt; ein formales M-Klasse-Feld in Finding-Workflows ist
NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Lifecycle: MAINT-002; Evidence: MAINT-019.
