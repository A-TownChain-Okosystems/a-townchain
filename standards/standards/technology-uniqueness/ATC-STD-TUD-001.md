---
standard:
  id: ATC-STD-TUD-001
  title: "Technology Uniqueness & Differentiation Standard"
  version: "1.0.0"
  status: approved
  category: governance
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  applies_to: "Alle Technologien, die als 'unique', 'novel' oder 'differentiated' positioniert werden; SSOT: registry/technology-registry.yaml"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-11"
  review_date: "2027-09-11"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-TUD-001 — Technology Uniqueness & Differentiation (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Sammelfreigabe 11.09.2026 (Owner-Chat), approval/APPROVAL-DECISION-2026-09-11.md | Klassifikation | Definition | Nachweislast |
|---|---|---|
| **UNIQUE** | Nachweislich keine bekannte vergleichbare Implementierung oder Architektur | Vollständige Prior-Art-Analyse + negative Suche dokumentiert |
| **NOVEL** | Technische Kombination oder Ausführung weist neue Eigenschaften auf | Kombinations-Analyse + Abgrenzung zu bekannten Ansätzen |
| **DIFFERENTIATED** | Existierende Technologie-Kategorie, aber mit klar eigener Architektur/Implementierung | Implementierungs-Evidence + Differentiator-Matrix |

Klassifikationen ohne vollständige Evidence-Kette sind **ungültig**; die
Fallback-Klasse ist PENDING-EVIDENCE.

## Evidence-Kette (§2)

Jede Klassifikation MUSS die vollständige 10-stufige Kette führen
(SSOT: `registry/technology-registry.yaml`, Feld `evidence`):

```
Technology ID → Definition → Prior Art Analysis → Existing Alternatives
→ ATC Differentiator → Implementation Evidence → Benchmark/Test Evidence
→ Security Review → Patent/IP Assessment → Uniqueness Classification
```

1. **Technology ID** — stabile Kennung nach Benennungs-Schema (ATC-TECH-NNN).
2. **Definition** — präzise technische Definition (was es IST, nicht was es kann).
3. **Prior Art Analysis** — systematische Suche nach vergleichbaren
   Implementierungen/Architekturen, mit Quellen.
4. **Existing Alternatives** — die 3 relevantesten existierenden Alternativen
   mit ehrlicher Charakterisierung (kein Strawman).
5. **ATC Differentiator** — der konkrete architektonische/technische Unterschied,
   nicht der Marketing-Vorteil.
6. **Implementation Evidence** — reproduzierbarer Code (Repo, Commit, Build),
   kein Konzeptdokument.
7. **Benchmark/Test Evidence** — reproduzierbare Messung oder Test-Suite,
   die den Differentiator validiert.
8. **Security Review** — Sicherheitsbewertung des Differentiators
   (Gefahr der Schwäche durch Eigenbau ist Teil der Analyse).
9. **Patent/IP Assessment** — Status: geprüft-frei / geprüft-Risiko / offen.
10. **Uniqueness Classification** — UNIQUE / NOVEL / DIFFERENTIATED gem. §1.

## Klassifikations-Regeln (§3)

- **REQ-TUD-001:** Keine Technologie wird in offizieller Dokumentation als
  „unique", „einzigartig", „novel" oder „weltweit erste/r" bezeichnet ohne
  gültige Klassifikation im Technology-Registry. Verstoß = Finding
  (Severity P1, ATC-STD-BUG-001).
- **REQ-TUD-002:** Klassifikationen werden ausschließlich über die
  Evidence-Kette (§2) vergeben; Stufe 3 (Prior Art) und Stufe 6
  (Implementation Evidence) sind Mindest-Pflicht, sonst bleibt der Status
  PENDING-EVIDENCE.
- **REQ-TUD-003:** Der Technology-Registry ist SSOT; README-, Wiki- und
  Whitepaper-Aussagen zu Differenzierung sind abgeleitete Sichten.
  Registry + Repository schlagen Wiki (ATC-STD-000).
- **REQ-TUD-004:** Jede Klassifikation hat einen Ablauf-/Review-Zyklus
  (review_date); obsoletes Prior Art invalidiert die Klassifikation automatisch
  (Fail-Closed → Rückfall auf PENDING-EVIDENCE).
- **REQ-TUD-005:** Die vertikale Integration als solche („Sovereign Stack")
  ist eine eigene Registry-Position (ATC-TECH-000) mit eigener Evidence-Kette;
  die Stärke liegt in der Kombination, nicht in der Addition.
- **REQ-TUD-006:** Benchmark-Claims („schneller als X") erfordern reproduzierbare
  Benchmarks im Repo; Vergleichswerte ohne Quelle sind unzulässig.
- **REQ-TUD-007:** Klassifikations-Änderungen (Abwertung UNIQUE→DIFFERENTIATED)
  sind Ereignisse mit Changelog-Eintrag und Impact-Analyse auf betroffene
  Dokumentation.
- **REQ-TUD-008:** Der Tier-Status (Tier S identitätsstiftend / Tier A
  Infrastruktur-Differenzierung) ist ein Positionierungs-Attribut des Registry,
  kein Qualitätsurteil und ersetzt keine Evidence-Kette.
- **REQ-TUD-009:** Neue Technologien, die als Flagship positioniert werden
  sollen, werden innerhalb von 30 Tagen nach Positionierungs-Entscheidung im
  Registry erfasst (mind. Stufen 1–5 der Evidence-Kette).
- **REQ-TUD-010:** Externe Kommunikation (Whitepaper, Webseite, Pitch) darf nur
  Klassifikationen aus dem Registry zitieren; PENDING-EVIDENCE-Technologien
  werden nur als „in Entwicklung" beschrieben.
- **REQ-TUD-011:** Die ATC/ATS-Trennung (deterministische Systemwahrheit vs.
  KI-Interpretation) ist als Architekturprinzip selbst klassifiziert; KI darf
  nicht durch Klassifikations-Nachlässe privilegiert werden.
- **REQ-TUD-012:** Security-Eigenbau-Risiken („not invented here") sind in der
  Security Review (§2 Stufe 8) explizit zu bewerten; Krypto-Primitiven folgen
  ATC Algorithm Agility (atc-algorithm), keine eigene Kryptografie-Erfindung
  ohne NOVEL/UNIQUE-Nachweis.

## Validierung (§4)

Der Validator `tools/technology-uniqueness/validate_technology_registry.py`
prüft (CI-gebunden):

- **TUD-1:** Registry parsebar, IDs eindeutig, Schema konform.
- **TUD-2:** Jede Technologie hat Klassifikation aus {UNIQUE, NOVEL,
  DIFFERENTIATED, PENDING-EVIDENCE} + Tier aus {S, A}.
- **TUD-3:** PENDING-EVIDENCE erfordert fehlende_evidence-Liste; UNIQUE/NOVEL
  erfordert vollständige Kette (Stufen 1–10).
- **TUD-4:** review_date nicht überschritten (Fail-Closed wie ATC-EXC).
- **TUD-5:** Repo-Referenzen existieren (per GitHub-API im Org-Scope).
- **TUD-6:** DIFFERENTIATED erfordert mind. Implementierungs-Evidence
  (Stufe 6) oder ehrliche Soll-Status-Kennzeichnung (R-Skelett).

## Implementierungsstatus

- **Status:** SPECIFIED (SCR-0091, Owner-Entwurf 11.09.2026)
- Registry + Validator: implementiert (dieser Commit)
- Klassifikationen: INITIAL — Klassifikationsfelder der Registry-Einträge
  sind Ersteinschätzungen des Owner-Entwurfs; vollständige Prior-Art-Analysen
  (Stufe 3) stehen aus und sind als TODO im Registry markiert
- SSOT: `registry/standard-implementation.yaml` (Verweis ATC-STD-TUD-001)

## Security Considerations

- **Keine Eigenbau-Kryptografie:** Der Differenzierungsdruck darf nicht zur
  Erfindung eigener Krypto-Primitiven führen (REQ-TUD-012); PQC folgt
  atc-algorithm (Algorithm Agility), standardisierte Primitive.
- **Honest-Claim-Grenze:** Überbewertete Klassifikationen sind ein Reputations-
  und Rechtsrisiko (Wettbewerbsrecht, Investor-Compliance); der Standard
  ist eine Kontrollinstanz dagegen.
- **AI-Nachlass-Verbot:** Klassifikations-Evidence darf nicht durch LLM-Autoren
  „erzeugt" werden; Quellen sind prüfbar zu dokumentieren (REQ-TUD-011).
- **Secrets:** Das Technology-Registry enthält keine Zugangsdaten
  (Security-Regel: Tokens ausschließlich als `$ENV`-Platzhalter).

## Changelog

| Version | Datum | Änderung | Autor |
|---|---|---|---|
| 1.0.0 | 2026-09-11 | Initial-Entwurf: Owner-Strategie-Papier (Flagship-Technologien, Sovereign Stack, ATC/ATS) als normativer Standard formalisiert; Evidence-Kette, Klassifikationsmodell, REQ-Matrix | Michael (Owner-Entwurf) / Aurora (SCR-0091) |

## References

- Owner-Strategie-Papier 11.09.2026 (Flagship Technology Positioning, Tier S/A)
- ATC-STD-000 (Governance Root, §9 Freigabe, Registry-First)
- ATC-STD-201/202 (Repository Governance, Org-Scope)
- ATC-STD-ZKP-001 (ZKP-Layer als Registry-Referenz ATC-TECH-012)
- ATC Algorithm Agility (atc-algorithm, ATC-TECH-010)
- AD-045 (ZKP-Layer), AD-023 (Rebuild qualitätsgetrieben), AD-007 (non-EVM)
