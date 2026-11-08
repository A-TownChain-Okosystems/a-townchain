---
standard:
  id: ATC-STD-020
  title: "Security Incident & Vulnerability Response Standard"
  version: "1.1.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-10"
  updated: "2026-09-11"
  normative: true
  effective_date: "2026-09-10"
  review_date: "2027-09-10"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards:
    - ATC-STD-016
    - ATC-STD-017
    - ATC-STD-019
    - ATC-STD-020
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-020 — Security Incident & Vulnerability Response (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Entwurf Continuous Assurance 11.09.2026 (SCR-0094): ATC-VULN-Record-Schema (§8) + ACCEPT/MITIGATE-Pflicht (§1). Die Vorgängervassung par.9-freigegeben 11.09.2026 per Owner-Direktive (SCR-0102) — in Kraft. **Scope:** ATC-STD-020 · Schwachstellen-Response-Kette + Vulnerability-Records · **Governance:** ATC-STD-000
> passiert, wenn eine neue kritische Schwachstelle oder ein neuer Angriff bekannt wird.

## Abstract

ATC-STD-020 definiert den verbindlichen Ablauf für Security-Incidents und neu bekannt werdende
Schwachstellen: Erkennung → Eskalation → Containment → Patch → Verifikation → Disclosure →
Post-Incident Review, mit SEV-1..4-Klassifikation und SLAs, Human-Gate für SEV-1/2-Declaration
und öffentliche Advisories sowie RCA-Pflicht nach ATC-STD-BUG-005. Ergänzt die Assurance-Familie
(ATC-STD-018/019) um den Notfall-Ablauf.

## Scope

**Gilt:** Erkennung, Eskalation und Behandlung von Security-Incidents und neu bekannt werdenden
Schwachstellen für alle 27 Repositories und die Organisation.

**Gilt nicht:** Präventive Baselines (ATC-STD-018 §5) und Supply-Chain-Integrität (ATC-STD-019);
generelle Änderungsprozesse (ATC-STD-UPDATE-001) bleiben für normale Updates zuständig.

## §1 Response-Lifecycle (REQ-STD-001, MUSS)
Verbindliche Kette: Erkennung → Eskalation → Containment → Patch → Verifikation → Disclosure →
Post-Incident Review. Kein Schritt darf übersprungen werden. Nach SEVERITY ASSESSMENT MUSS die
Entscheidung ACCEPT (dokumentiertes, begründetes Risiko-Akzeptieren mit
Owner-Genehmigung und Review-Termin) oder MITIGATE→PATCH→VERIFY getroffen
werden; ACCEPT ohne Owner-Genehmigung IST unzulässig.

## §2 Schweregrad-Modell (REQ-STD-002, MUSS)
SEV-Klassifikation (SEV-1 kritisch/aktiv ausgenutzt, SEV-2 kritisch/nicht ausgenutzt, SEV-3 hoch,
SEV-4 mittel/niedrig) mit Eskalations-SLA je Stufe; SEV-1/SEV-2 = Owner-Benachrichtigung sofort.

## §3 Containment-Optionen (REQ-STD-003, MUSS)
Dokumentierte Maßnahmenkette: Rollback (git revert), Feature-Disable, Traffic-Off, Patch-Forward;
Containment darf Data-Integrity nicht verschlechtern.

## §4 Verifikation (REQ-STD-004, MUSS)
Patch-Verifikation über Test + Security-Scan + Evidence-Record; Exploit-Reproduktion wo möglich;
kein Close ohne verifizierten Nachweis (Evidence-only-Grundsatz aus ATC-STD-018 §9).

## §5 Disclosure (REQ-STD-005, MUSS)
Dokumentierte Disclosure-Entscheidung (intern/öffentlich) mit Owner-Gate für öffentliche
Advisories; kein Silent-Patch kritischer Schwachstellen.

## §6 Post-Incident Review (REQ-STD-006, MUSS)
RCA nach ATC-STD-BUG-005 (Vorfall-Aufzeichnung in REALITY_STATUS); Lessons-Learned als Findings;
Wiedereintritts-Prävention (Regressionstest, Gate-Erweiterung).

## §7 Agenten-Rolle (REQ-STD-007, MUSS)
KI-Agenten erkennen, triagieren und patchen — aber: SEV-1/SEV-2-Declaration und Disclosure
erfordern Human-Gate (ai/policies.yaml AP-016; ATC Agent Governance).

## §8 Vulnerability Record (REQ-STD-008, MUST) — v1.1.0
Jede Vulnerability MUSS einen maschinenlesbaren Record mit folgenden Pflichtfeldern
führen:

```
id: ATC-VULN-NNN
source:            (CVE/GHSA/OSV/Advisory/interne Detektion)
type:              (Schwachstellen-/Angriffsklasse, ATC-STD-018 §4)
affected_component:
affected_versions:
severity:          (CRITICAL/HIGH/MEDIUM/LOW)
exploitability:    (aktiv ausgenutzt / PoC vorhanden / theoretisch)
impact:
exposure:          (internet-exponiert / intern / build-only)
mitigation:
patch:
verification:
regression_test:   (ATC-STD-026)
status:            (OPEN/TRIAGED/ACCEPTED/MITIGATED/PATCHED/VERIFIED/DISCLOSED/CLOSED)
disclosure:
```

Records MÜSSEN unter evidence/vulnerabilities/ (ATC-GATE-SEC-001 §4) liegen;
SLA-Felder (detected_at/patched_at) MUSS die SLA-Prüfung nach ATC-STD-022 §1
tragen.

## §9 Freigabe
FREIGEGEBEN 10.09.2026 via Owner-Direktive. Priorität P1.

## §30 Freeze & Change-Control
§30-eingefroren; Änderungen ausschließlich via ATC-STD-UPDATE-001.

## Security Considerations
Der Standard verhindert ad-hoc-Panik-Reaktionen: jede Phase erzeugt Records (Auditierbarkeit);
kommunikationskritische Schritte liegen beim Owner.
