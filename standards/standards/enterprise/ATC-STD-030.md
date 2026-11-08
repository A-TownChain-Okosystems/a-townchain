---
standard:
  id: ATC-STD-030
  title: "Automated Repository Health & Compliance Standard"
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

# ATC-STD-030 — Automated Repository Health & Compliance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-030 · Scanner-Struktur, Finding-Klassen, Continuous-Assurance-Prinzip · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-028" — Slot belegt (Attack Surface
> Management, SCR-0094), §37 ergab 030.

## Abstract

ATC-STD-030 macht aus dem Security-Set ein vollständiges automatisiertes
Repository-Assurance-System: Jedes Repository wird kontinuierlich maschinell
gegen die ATC-Standards geprüft (10 Dimensionen), Befunde in einheitlichen
Finding-Klassen erfasst und über Feedback-Loop/Evolution-Engine in das
Standardsystem zurückgespeist.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle Repositories der Organisation. Findings aus Compliance-Sicht;
Security-Incident-Kette: ATC-STD-020/035.

## §1 Assurance-Scanner (REQ-STD-001, MUST)

Jedes Repository MUSS kontinuierlich gegen 10 Dimensionen geprüft werden:
Structure, Documentation, Dependencies, Technology, Security, Code Quality,
Tests, Supply Chain, Licensing, Lifecycle. Ergebnis MUSS ein Compliance-Report
mit P-Klassifizierung sein.

## §2 Finding-Klassen (REQ-STD-002, MUST)

Einheitliche Sprache über alle Audits:

| Level | Bedeutung | Reaktion |
|---|---|---|
| P0 | kritischer Sicherheits-/Integritätsfehler | sofortige Blockierung |
| P1 | schwerwiegendes Problem | kurzfristige Behebung |
| P2 | mittlere Abweichung | priorisierte Behebung |
| P3 | Verbesserung / technische Schuld | Lifecycle |

Kategorie-Codes: SECURITY (SEC), BUG (BUG), COMPLIANCE (CMP), TECHNOLOGY
(TEC), DEPENDENCY (DEP), DOCUMENTATION (DOC), ARCHITECTURE (ARC),
SUPPLY_CHAIN (SUP), PERFORMANCE (PRF), RELIABILITY (REL).

Finding-ID MUSS dem Format P<n>-<CAT3>-<NNNN> entsprechen (z. B.
P1-SEC-0042, P2-TEC-0017, P3-DOC-0081). Alt-Findings (F-NNN in
registry/findings.yaml) MÜSSEN bei Berührung auf das neue Format migriert werden.

## §3 Continuous-Assurance-Prinzip (REQ-STD-003, MUST)

Verbindlich: Ein Repository gilt NICHT deshalb als konform, weil es einmal
geprüft wurde — Konformität IST ein kontinuierlich überprüfter Zustand.

Audit → Fix → Verify → Monitor → New Change → Re-Audit

Konformitäts-Ausweise MÜSSEN mit Prüfungszeitpunkt (valid_until) versehen sein.

## §4 Feedback-Loop & Standards-Evolution (REQ-STD-004, MUST)

Ein Finding DARF NICHT einfach geschlossen werden:

Finding → Root Cause → Fix → Test → Standard Review → (Standard-Verbesserung
via SCR | Close). Wiederkehrende Muster (z. B. 5 Repos mit gleichem Fehler)
MÜSSEN als organisatorischer Standard-Gap analysiert werden: Repository
Findings → Cross-Repository Analysis → Recurring Pattern → Standards Gap?
→ Change Request → neuer/geänderter ATC-STD → org-weites Enforcement.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Scanner: 10 Dimensionen MÜSSEN kontinuierlich maschinell geprüft werden.
- id: REQ-STD-002 — Finding-Klassen: P0-P3 + 10 Kategorien + ID-Format MÜSSEN einheitlich verwendet werden.
- id: REQ-STD-003 — Continuous Assurance: Konformität MUSS als fortlaufend überprüfter Zustand geführt werden (valid_until).
- id: REQ-STD-004 — Feedback-Loop: Findings MÜSSEN RCA und Standard Review durchlaufen; wiederkehrende Muster MÜSSEN Standard-Gap-Analyse auslösen.

## Compliance

Prüfung: Assurance-Engine-Rollout (SCR-Nachfolge); Report-Ablage unter
evidence/audits/; Finding-Führung in registry/findings.yaml (neues ID-Format).

## Security Considerations

- Scanner-Reports enthalten Schwachstellen-Details: zugriffsbeschränkt.
- Scanner selbst MUSS den Supply-Chain-Anforderungen (ATC-STD-019) genügen.

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Assurance-Engine-Implementierung
als SCR offen. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) — Scanner,
  Finding-Klassen, Continuous-Assurance-Prinzip, Feedback-Loop, Evolution-
  Engine. CANDIDATE.

## References

- ATC-STD-000 — Governance Root · ATC-STD-018/019/020 — Assurance-Familie
- ATC-STD-024 — Root-Cause-Pflicht · ATC-STD-031 — Health Score
- ATC-GATE-SEC-001 — Compliance Gate
