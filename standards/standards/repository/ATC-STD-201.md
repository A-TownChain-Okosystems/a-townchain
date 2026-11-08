---
standard:
  id: ATC-STD-201
  title: "ATC-STD-201 — Repository Structure Standard"
  version: "1.0.1"
  status: approved
  category: repository
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: ["ATC-STD-REPO-001@1.0.x"]
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  applies_to: "Alle ATC-Repositories"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-201 — Repository Structure Standard (v1.0.1, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Version:** 1.0.1 (FORMAL) · **Datum:** 07.09.2026 · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-201 · **Scope:** Alle offiziellen Repositories der Organisation
> **Referenzen:** AD-025/026/027/028/029/031, ATC-STD-202 (Naming/Classification/Ownership/Lifecycle/S-Klassen), ATC-STD-203 (Security/Release/Branching/Commits/Gates)
> **Validator:** tools/atc-repo-audit (Regeln V-01…V-16) · **Registry:** registry/repositories.yaml

---

## Abstract

Diese Spezifikation definiert die verbindliche Repository-Struktur aller
offiziellen ATC-Repositories: Top-Level-Layout, Verantwortlichkeits-Trennung,
maschinenlesbare Metadaten (.atc/), Dokumentations-Mindeststandard, Hygiene
und die Compliance-Matrix M-01…M-16 je Reifegrad R0-R4 mit maschinenpruefbaren
Validator-Regeln V-01…V-16.

## 0. Normativitaet

Die Schluesselwoerter **MUST / MUST NOT / SHOULD / SHOULD NOT / MAY** in dieser
Spezifikation sind im Sinne von RFC 2119 zu verstehen.

- **MUST** = zwingend; Verstoss = Validator-FAIL = GATE: NO-GO
- **SHOULD** = empfohlen; Verstoss = WARN (Score-Abzug, kein NO-GO allein)
- **MAY** = optional; nicht geprueft

Ein Repository gilt als ATC-STD-201-konform, wenn ALLE fuer sein
Compliance-Level (R0-R4, ATC-STD-202) geltenden MUST-Regeln erfuellt sind.

## 1. Repository-Metadaten (Abschnitt 16 des Erweiterungs-Mandats)

Jedes offizielles Repository MUSS eine maschinenlesbare Metadatei
`.atc/repository.yaml` besitzen (ab R1) mit mindestens:

```yaml
standard: ATC-STD-201
standard_version: "1.0.0"
repository:
  name: <repo-name>          # MUSS mit GitHub-Name uebereinstimmen
  classification: CORE|SPEC|SDK|APPLICATION|INFRA|AI|OS|GAME
  maturity: R0|R1|R2|R3|R4
  status: experimental|development|beta|production|deprecated|archived
project:
  organization: A-TownChain-Okosystems
  domain: <domain>
  component: <component>
language:
  primary: <sprache>
license:
  type: proprietary
security:
  criticality: public|standard|important|critical|protocol-critical  # S0-S4
api:
  stability: experimental|alpha|beta|stable|frozen|deprecated
protocol:
  compatibility: required|none
```

Zusaetzlich MUSS ab R2 vorliegen:
- `.atc/ownership.yaml` (Verantwortlichkeiten, ATC-STD-202 Abschnitt 2)
- `.atc/lifecycle.yaml` (Lebenszyklus, ATC-STD-202 Abschnitt 3)
- `.atc/compliance.yaml` (Gate-Status-Selbstdeklaration, durch Audit ersetzt)

Ein automatischer ATC Repository Validator (atc-repo-audit) prueft MUSS-Regeln maschinell.

## 2. Top-Level-Struktur

Jedes Repository MUSS der Verantwortlichkeits-Trennung folgen (Abschnitt 3).
Nicht jeder Ordner MUSS existieren — existiert er, MUSS er der Standardrolle
entsprechen. Referenzstruktur (SHOULD fuer neue Repositories):

src/ tests/ docs/ examples/ scripts/ tools/ configs/ deployments/
benches/ assets/ .github/ + Root-Meta-Dateien (README, LICENSE, CHANGELOG,
SECURITY, CONTRIBUTING, CODE_OF_CONDUCT, VERSION, .gitignore).

Bestands-Harmonisierung (AD-025): restaurierte Repos dokumentieren ihr
modules/-Mapping in docs/REPOSITORY_STANDARD.md; src/-Layout ist Zielstruktur.

## 3. Verantwortlichkeits-Trennung

| Bereich | Zweck | Regel |
|---|---|---|
| src/ | Produktionscode | Produktionscode MUSS NICHT in scripts/ liegen |
| tests/ | Tests | Tests MUssen getrennt von Produktionscode liegen |
| docs/ | technische Dokumentation | Spezifikationen MUESSEN NICHT in src/ liegen |
| examples/ scripts/ tools/ configs/ deployments/ benches/ assets/ | je Zweck | temporaere Artefakte MUessen draussen bleiben |

## 4. Domain-Strukturen (SHOULD, neue Repos)

Blockchain-Core / ATCLang / Wallet-Referenzmuster wie in v1.0.0-PROPOSED.

## 5. Dokumentation

- README.md MUSS existieren (alle Level); ab R2 MUSS es die 12 Pflichtabschnitte
  enthalten (Purpose, Scope, Architecture, Features, Repository Structure,
  Installation, Development, Testing, Security, Roadmap, Versioning, License).
- LICENSE MUSS existieren (proprietaer, Apache-2.0 lizenziert — siehe LICENSE).
- SECURITY.md MUSS ab R2, CHANGELOG.md MUSS ab R2 existieren.
- docs/REPOSITORY_STANDARD.md MUSS ab R1 existieren (Klassifizierung + Mapping).
- docs/decisions/ (ADRs) MUSS ab R2 gefuehrt werden — ODER das Repository
  verweist auf den zentralen DECISIONS_REGISTER (Hub) fuer
  organisationsweite Entscheidungen (Zwei-Ebenen-Modell AD-029).

## 6. Repository-Hygiene (MUST, alle Level)

Verboten im Git-Tree: .env, private keys, wallet seeds, API secrets, production
credentials, node_modules/, target/, dist/ (generiert), build/, *.log, tmp/,
personenbezogene Daten. Erlaubt: .env.example (ohne Werte).

## 7. Compliance-Badge (SHOULD ab R2, MUST ab R3)

Im README (Anfang): ATC-Compliance-Block mit Level, Standard-Version,
Gate-Stati (Structure/CI/Security/Architecture: PASS) — macht den Zustand
sofort sichtbar. Format:

```
ATC COMPLIANCE: R3 · PRODUCTION · Score 9x% · Standard 1.0.0 · GATE: PASS
```

## 8. Compliance-Matrix (MUST-Regeln je Level)

| ID | Anforderung | R0 | R1 | R2 | R3 | R4 |
|----|-------------|----|----|----|----|----|
| M-01 | LICENSE vorhanden | MUST | MUST | MUST | MUST | MUST |
| M-02 | README.md (Purpose+Scope) | MUST | MUST | MUST | MUST | MUST |
| M-03 | README 12 Pflichtabschnitte | — | SHOULD | MUST | MUST | MUST |
| M-04 | .atc/repository.yaml (gueltig) | MAY | MUST | MUST | MUST | MUST |
| M-05 | .atc/ownership.yaml + CODEOWNERS | — | MAY | MUST | MUST | MUST |
| M-06 | .atc/lifecycle.yaml (gueltiger Pfad) | — | SHOULD | MUST | MUST | MUST |
| M-07 | .atc/compliance.yaml | — | — | MUST | MUST | MUST |
| M-08 | SECURITY.md (6 Inhalte) | — | SHOULD | MUST | MUST | MUST |
| M-09 | CHANGELOG.md gepflegt | — | SHOULD | MUST | MUST | MUST |
| M-10 | docs/REPOSITORY_STANDARD.md | — | MUST | MUST | MUST | MUST |
| M-11 | Verantwortlichkeits-Trennung | MUST | MUST | MUST | MUST | MUST |
| M-12 | Hygiene (keine Secrets/Artefakte im Tree) | MUST | MUST | MUST | MUST | MUST |
| M-13 | tests/ + laufende CI | — | MUST | MUST | MUST | MUST |
| M-14 | ADR-Pflicht (lokal oder zentral) | — | — | MUST | MUST | MUST |
| M-15 | Compliance-Badge im README | — | MAY | SHOULD | MUST | MUST |
| M-16 | Eintrag in registry/repositories.yaml | MUST | MUST | MUST | MUST | MUST |

## 9. Validator-Regeln (V-01…V-16, maschinenpruefbar)

Jede V-Regel prueft eine M-Regel. Semantik:
- M-Regel = MUST fuer das Level des Repos → Verletzung = **[FAIL]** → GATE: NO-GO
- M-Regel = SHOULD → **[WARN]** (Score-Abzug)
- nicht anwendbar → Pruefung entfaellt

| Regel | Pruefung | Quelle |
|-------|----------|--------|
| V-01 | LICENSE existiert, nicht leer | M-01 |
| V-02 | README existiert mit Purpose+Scope | M-02 |
| V-03 | README enthaelt 12 Abschnitts-Schluesselworte | M-03 |
| V-04 | .atc/repository.yaml existiert, Pflichtschluessel vorhanden, name == Verzeichnisname | M-04 |
| V-05 | .atc/ownership.yaml existiert; CODEOWNERS vorhanden | M-05 |
| V-06 | .atc/lifecycle.yaml: stage in {experimental,development,beta,production,deprecated,archived} | M-06 |
| V-07 | .atc/compliance.yaml mit standard+level+gates | M-07 |
| V-08 | SECURITY.md existiert | M-08 |
| V-09 | CHANGELOG.md existiert, nicht leer | M-09 |
| V-10 | docs/REPOSITORY_STANDARD.md existiert | M-10 |
| V-11 | keine verbotenen Pfade/Dateien im Git-Tree | M-12 |
| V-12 | tests/- oder Test-Verzeichnis/-dateien vorhanden; .github/workflows/ nicht leer | M-13 |
| V-13 | docs/decisions/ ODER Verweis auf zentrales DECISIONS_REGISTER | M-14 |
| V-14 | Compliance-Badge-Zeile im README | M-15 |
| V-15 | Repo in registry/repositories.yaml gelistet (Konsistenz name/level) | M-16 |
| V-16 | Conventional-Commits-Disziplin (letzte 20 Commits, ≥80%) | ATC-STD-203 §2 |

Referenz-Implementierung: **tools/atc-repo-audit** (v0.1.0, Python, stdlib-only).

## 10. Score-Modell (Health Score, ATC-STD-203 §8)

Kategorien mit Gewichtung: Structure, Documentation, Testing, Security,
CI/CD, Ownership, Versioning, Dependencies. Schwellenwerte und Badge-Ableitung:
ATC-STD-203. GATE: PASS erfordert Score >= 85 UND 0 MUST-FAILs.

## 11. Obergrenzen dieser Spezifikation

Struktur, Metadaten, Dokumentation, Hygiene, Badge. Naming, Classification,
Ownership-Teams, Lifecycle-Uebergange, S-Klassen und der zentrale Dependency
Graph: ATC-STD-202. Security, Branching, Commits, PRs, Release-Gates,
Dependency Policy, Third-Party, API-Stability, Breaking Changes, Reproducible
Builds, Artifact Management: ATC-STD-203.

## Security Considerations

Struktur-/Hygiene-Regeln sind Security-relevant (keine Secrets im Tree,
ATC-STD-203 zusaetzlich anwendbar). Vertiefte Security-Anforderungen:
ATC-STD-203.

## References

NORMATIVE: ATC-STD-000 (Governance), ATC-STD-202, ATC-STD-203 ·
INFORMATIVE: AD-025/026/028/029/031 (DECISIONS_REGISTER, Hub) ·
IMPLEMENTATION: tools/atc-repo-audit, schemas/repository.schema.yaml.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

- 1.0.1 (07.09.2026): Unter ATC-STD-000 Governance gestellt; ID von
  ATC-STD-REPO-001 auf ATC-STD-201 umgestellt (supersedes); Metadaten-Header
  ergaenzt.
- 1.0.0 (07.09.2026): Formale Spezifikation (AD-031): RFC-2119,
  Compliance-Matrix M-01…M-16, Validator-Regeln V-01…V-16.
