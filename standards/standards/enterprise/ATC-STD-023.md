---
standard:
  id: ATC-STD-023
  title: "Technology Lifecycle Management Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle Technologien in ATC-Repositories (SSOT registry/technology-registry.yaml)"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
    - ATC-STD-TUD-001
----

# ATC-STD-023 — Technology Lifecycle Management (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Continuous Assurance 11.09.2026 (SCR-0094):
> Verbindlicher Lebenszyklus und Adoption-Gate für Technologien — verhindert
> Technologie-Wildwuchs. Bis zur §9-Freigabe nicht wirksam. **Scope:** ATC-STD-023 ·
> Technologie-Lebenszyklus + Adoption Gate · **Governance:** ATC-STD-000

## Abstract

ATC-STD-023 regelt den verbindlichen Lebenszyklus jeder Technologie im Ökosystem
und das Adoption Gate für Neuentraete. Neue Technologie DARF NICHT eingeführt
werden, nur weil sie populär ist — Aufnahme nur über das normative Gate.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für jede Technologie (Sprache, Runtime, Framework, Library, Tool, Base Image,
Protokoll). SSOT des Bestands: registry/technology-registry.yaml (TUD-001).

## §1 Technologie-Lebenszyklus (REQ-STD-001, MUST)

Jede Technologie MUSS einen Lifecycle-Status führen:

PROPOSED → EVALUATED → APPROVED → ADOPTED → SUPPORTED → DEPRECATED → EOL → REMOVED

Statusübergänge MÜSSEN in der Technology-Registry (SSOT) mit Evidence und SCR-
Verweis dokumentiert sein. Technologie-Wildwuchs (Nutzung ohne APPROVED/ADOPTED)
IST ein Finding.

## §2 Technology Adoption Gate (REQ-STD-002, MUST)

Jeder Kandidat MUSS das Gate durchlaufen (erste Nein-Antwort = Ausnahme-Prüfung
oder REJECT):

Security? → Maturity? → Maintenance? → License? → Performance? → Compatibility?
→ Interoperability? → Vendor-/Supply-Chain-Risk? → Long-Term-Viability?

Ergebnis MUSS einer von fünf Entscheidungen sein:

| Entscheidung | Bedeutung |
|---|---|
| ADOPT | Aufnahme in die Registry (ADOPTED nach Implementierung) |
| ADOPT WITH CONDITIONS | Auflagen dokumentiert, Nachweis pflichtig |
| PILOT | zeitlich/projektlich begrenzter Versuch mit Review-Termin |
| DEFER | Vertagt; Re-Evaluation mit Termin |
| REJECT | Ablehnung mit Begründung |

## §3 Deprecation & Removal (REQ-STD-003, MUST)

DEPRECATED MUSS Ersatzpfad und Frist nennen; EOL-Technologie unterliegt dem
EOL-Verbot von ATC-STD-018 §2 (ADR-Ausnahme möglich); REMOVED MUSS aus allen
Repos entfernt sein (Verifikation via Repository-Audit).

## §4 Registry-Kopplung (REQ-STD-004, MUST)

Adoption und Lifecycle MUssen über registry/technology-registry.yaml laufen;
Klassifikationspflicht (UNIQUE/NOVEL/DIFFERENTIATED/PENDING) per ATC-STD-TUD-001
bleibt unberührt. Gate-Entscheidungen MÜSSEN als Technology-Registry-Eintrag mit
Evidence-Verweis (evidence/technology/) vorliegen.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Lebenszyklus: 8-Status-Kette MUSS je Technologie geführt werden; Wildwuchs IST Finding.
- id: REQ-STD-002 — Adoption Gate: 9-Kriterien-Prüfung MUSS vor jeder Aufnahme erfolgen; 5 Entscheidungen.
- id: REQ-STD-003 — Deprecation/Removal: Ersatzpfad, Frist, EOL-Verbot-Kopplung, Repo-Verifikation.
- id: REQ-STD-004 — Registry-Kopplung: Technology-Registry MUSS SSOT sein; Entscheidungen mit Evidence.

## Compliance

Prüfung: Technology-Registry-Validierung (TUD-1..6) + Repository-Audit gegen
nicht-ADOPTED-Technologien; Adoption-Records in evidence/technology/.

## Security Considerations

- Populärität IST kein Kriterium (Supply-Chain-Risiko populärer Pakete: Typosquatting,
  Dependency Confusion — ATC-STD-019 §6).
- PILOT ohne Frist = verdeckte Adaption: PILOT MUSS Review-Termin haben.

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Evidence ab erster
Adoption-Entscheidung. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Continuous Assurance (SCR-0094) — 8-Status-
  Lebenszyklus, 9-Kriterien-Adoption-Gate, 5 Entscheidungen, Registry-Kopplung. CANDIDATE.

## References

- ATC-STD-000 — Governance Root
- ATC-STD-018 — Technology Currency (EOL-Verbot §2, Reviews §7)
- ATC-STD-TUD-001 — Technology Uniqueness & Differentiation (Registry-SSOT)
- ATC-STD-019 — Supply Chain Security
