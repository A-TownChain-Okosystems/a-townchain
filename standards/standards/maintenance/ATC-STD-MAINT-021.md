---
standard:
  id: ATC-STD-MAINT-021
  title: "ATC-STD-MAINT-021 — Emergency Maintenance Standard"
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
  applies_to: "Alle M3-Emergency-Faelle des Oekosystems (Konsensfehler, Kernel Security Boundary, VM-Inkompatibilitaet, Chain-State-Korruption, Datenverlust, kritische RCE, kryptografischer Vertrauensbruch)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-021 — Emergency Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Der Emergency-Maintenance-Pfad fuer M3: beschleunigt, aber vollstaendig — mit Incident Records und Post-Mortem-Evidence. Prioritaet P1.

## §1 Ausloeser (M3, Verweis MAINT-001)

Konsensfehler · Kernel Security Boundary gebrochen · ATC-VM-Inkompatibilitaet ·
Chain-State-Korruption · Datenverlust · kritische Remote Code Execution · kryptografischer
Vertrauensbruch.

## §2 Emergency-Pfad (normativ)

1. SOFORT Owner-Eskalation (kein Zyklus-Abwarten); ATC-STD-000 §32 bleibt verbindlich.
2. Beschleunigter, aber VOLLSTAENDIGER Lifecycle (MAINT-002): alle 13 Phasen, parallelisiert;
   EVIDENCE-Phase wird nachdokumentiert, nie ausgelassen.
3. Separation of Duties bleibt in Kraft (Implementer ≠ Validator ≠ Auditor;
   Release Authority ≠ Implementer).
4. Emergency-Commits sind gekennzeichnet (z.B. `EMERGENCY:`-Präfix) und zentral registriert.
5. Nach Stabilisierung: Rueckkehr in normale Governance + Post-Mortem (Ursache, Wirkung,
   Vermeidung, Regression-Check).

## §3 Incident Records

Jeder M3-Fall erzeugt einen Incident Record (maschinenlesbar, MAINT-019-Schema-Erweiterung):
Zeitachse (detect/acknowledge/resolve), Betroffene Schichten, Massnahmen, Rollen (SoD),
kommunikation, Post-Mortem-Verweis.

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-080 | M3-Verdacht eskaliert sofort an den Owner (s32) | MUST |
| REQ-MAINT-081 | Emergency durchlaeuft alle 13 Phasen beschleunigt; EVIDENCE nie ausgelassen | MUST |
| REQ-MAINT-082 | SoD gilt auch im Emergency | MUST |
| REQ-MAINT-083 | Jeder M3-Fall erzeugt Incident Record + Post-Mortem | MUST |
| REQ-MAINT-084 | Nach Stabilisierung Rueckkehr in normale Governance | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Der s32-Prozess ist normativ verankert; ein formalisierter Emergency-Pfad mit Incident Records ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
