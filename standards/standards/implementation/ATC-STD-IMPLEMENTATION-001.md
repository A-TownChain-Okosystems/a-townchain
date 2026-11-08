---
standard:
  id: ATC-STD-IMPLEMENTATION-001
  title: "Standard Implementation Matrix"
  version: "1.0.0"
  status: approved
  category: implementation
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "atc-standards Registry und alle Repositories des Oekosystems; Agenten, CI, Audits"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-IMPLEMENTATION-001 — Standard Implementation Matrix (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0045, Owner-Re-Audit „von Standards definiert zu Standards nachweisbar implementiert
> und dauerhaft erzwungen"). Familie FAM-48 (Implementation Tracking, Kategorie
> implementation). **Prioritaet:** P0 (Kern des Implementierungs-Pivots).

## Zweck

Die Registry beantwortet bisher: **Welche Standards gibt es?** (registry/standards.yaml).
Dieser Standard fuegt die zweite SSOT hinzu: **Wo ist jeder Standard implementiert,
getestet, erzwungen und belegt?**

```
Standard → Applicability → Implementation → Test → CI-Gate → Evidence → Verified
```

## §1 Applicability-Modell (REQ-IMP-001)

Jeder Standard erhaelt eine Anwendbarkeits-Bewertung — „alle Standards verbindlich"
wird praegisiert zu:

| Urteil | Bedeutung |
|---|---|
| MANDATORY | fuer alle Repositories des Scopes verbindlich |
| CONDITIONAL | verbindlich, wenn Bedingung eintritt (z.B. Sprache/Domain/Classification) |
| REFERENCE | informativ/normativ-orientierend, kein Gate |
| NOT_APPLICABLE | explizit ausserhalb des Scopes (mit Begruendung) |

Bewertungsdimensionen: scope, repository_types, lifecycle_states, criticality.

## §2 Implementierungs-Status-Taxonomie (REQ-IMP-004)

| Status | Definition |
|---|---|
| enforced | in CI/Merge-Kette technisch erzwungen (Verstoss blockiert) |
| implemented | real angewendet mit nachweisbarer Evidence |
| specification_only | Standard existiert, noch ohne Anwendungs-Evidence |
| reference | als Referenz zitiert (informativ) |

## §3 SSOT: registry/standard-implementation.yaml (REQ-IMP-002, REQ-IMP-003)

Eintrags-Schema:

```yaml
- standard: ATC-STD-201
  version: 1.0.1
  applicability: MANDATORY
  implementation:
    status: enforced
    repositories: [atc-standards, atc-node, ...]
    files: [.atc/repository.yaml, .github/workflows/governance-ci.yml]
    tests: [governance/repository_structure]
    ci_gate: [GOV-201]
    evidence: [AUD-2026-0002]
    independence: L1
```

Fehlende Eintraege sind Coverage-Luecken und duerfen NICHT als „implementiert"
interpretiert werden (REQ-IMP-008; Analogie zu ERR-007 UNKNOWN != OK).

## §4 Evidence und Audit-Unabhaengigkeit (REQ-IMP-005)

Jede Evidence deklariert ihr Unabhaengigkeitslevel: **L1** Self-Audit (Repo prueft sich
selbst), **L2** Cross-Repository-Audit (Fremd-Repo/Agent prueft), **L3** Independent/Owner
Audit. P0-Standards benoetigen mindestens eine L2/L3-Evidence fuer Status enforced.

## §5 Generierte Views (REQ-IMP-006)

Kennzahlen (Anzahl Standards, APPROVED/CANDIDATE-Stand, Familien, SHA-256 der Registry)
duerfen NUR generiert werden aus registry/standards.yaml (tools/gen_views/generate_views.py).
Manuelle Zähler in README/AGENT_MANIFEST/STATUS sind verboten — jede Änderung der Registry
regeneriert die Views. registry/registry.lock versioniert die Registry kryptographisch
(SHA-256), damit jeder Agent seine Standards-Version verifizieren kann.

## §6 Coverage-KPIs (REQ-IMP-007)

Je Wartungszyklus (REPO-MAINT-001 §16): matrix_coverage (Eintraege/Registry-Groesse),
enforced_coverage, evidence_independence (L2/L3-Anteil). Ziele: monoton steigend;
Ruecklaeufe = Finding.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-IMP-001 | Applicability-Urteil je Standard (4 Urteile, Dimensionen dokumentiert) |
| REQ-IMP-002 | standard-implementation.yaml ist SSOT der Implementierungsmatrix |
| REQ-IMP-003 | Je Eintrag: repositories/files/tests/ci_gate/evidence |
| REQ-IMP-004 | Implementierungs-Status nur aus der 4-stufigen Taxonomie |
| REQ-IMP-005 | Evidence deklariert Unabhaengigkeitslevel L1/L2/L3; P0 braucht L2/L3 fuer enforced |
| REQ-IMP-006 | Kennzahlen ausschliesslich generiert; manuelle Zaehler verboten |
| REQ-IMP-007 | Coverage-KPIs je Wartungszyklus dokumentiert und steigend |
| REQ-IMP-008 | Fehlender Matrix-Eintrag = Coverage-Luecke, nicht „implementiert" |
| REQ-IMP-009 | Matrix-Aenderungen mit SCR/REQ-Traceability (ATC-STD-300 DTC) |

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | IMPLEMENTED (erster Zyklus) — Matrix initial 22 Eintraege, Generator aktiv |
| Evidence | registry/standard-implementation.yaml, generate_views.py, AUD-2026-0002, RUN-001, ATC-ERR-0001 |
| Coverage-KPI | Initial 22/420 Eintraege (5,2 %) — Ziel je Zyklus steigend |
| CI-Gate | Generator-Drift-Check geplant (README-Zaehler vs. Registry = FAIL) — folgt mit S-Check-Erweiterung |

## Security Considerations

registry.lock enthaelt keinen Konfigurationsinhalt, nur Hashes/Revisionen. Evidence-Dateien
folgen der Sicherheitsregel (keine Klartext-Zugangsdaten, nur $ENV-Platzhalter).

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — Owner-Re-Audit Schritt 1 (SCR-0045) |

## References

NORMATIV: ATC-STD-000 (§9/§33/§37), ATC-STD-300 (DTC), ATC-STD-ERR-007 (UNKNOWN-Analogie),
ATC-STD-REPO-MAINT-001 (§16 KPIs), ATC-STD-BUG-005 (Corrective/Preventive) ·
INFORMATIVE: SCR-0045, Owner-Re-Audit 08.09. (AMBER 84/100), registry/framework.yaml FAM-48
