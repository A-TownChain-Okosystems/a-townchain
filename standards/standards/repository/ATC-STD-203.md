---
standard:
  id: ATC-STD-203
  title: "ATC-STD-203 — Repository Security & Release Standard"
  version: "1.0.1"
  status: approved
  category: repository
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: ["ATC-STD-REPO-003@1.0.x"]
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-203 — Repository Security & Release Standard (v1.0.1, APPROVED)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft| **Datum:** 07.09.2026 | **Autor:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-203 | **Scope:** Alle produktiven (R2+) Repositories
> **Referenzen:** AD-022 (19 Gates, KEIN FREEZE vor G18 Security Audit), AD-027 (Lauffaehigkeits-Roadmap), ATC-STD-201/-002, Issue #69 (Dependabot-Audit)

---

## Abstract

ATC-STD-203 definiert Security-Gates, CI/CD-Mindeststandards, Versionierung
und Release-Disziplin. Das Compliance-Level aus ATC-STD-202 bestimmt die
harteste Gate-Stufe.

## 1. SECURITY.md (Pflicht ab R2)

Muss definieren: Vulnerability Reporting, Security Contact, Supported Versions,
Disclosure Policy, Security Scope, Known Limitations. Zusaetzlich tests/security/
fuer sicherheitsrelevante Tests.

**Blockchain-Repositories zusaetzlich** (tests/): consensus/, cryptography/,
state-transition/, replay/, serialization/, adversarial/.

## 2. CI/CD-Mindeststandard

.github/workflows/ deckt mindestens ab: ci.yml, test.yml, security.yml,
release.yml. Pipeline:

```
Commit → Format/Lint → Compile → Unit Tests → Integration Tests
      → Security Checks → E2E → Release Gate
```

Fuer kritische Repositories (R3+): expliziter Release-Status
`GATE: PASS` oder `GATE: NO-GO` als Artefakt des Laufs.

Harmonisierung: AD-022-Gates (ATCLang G0-G19) und AD-027-Run-Kriterien
(M1-M8) sind die fachlichen Gates; dieser Standard definiert die
Pipeline-Seite. Dependabot-Warnungen (Issue #69) werden via security.yml
getrackt, nicht ignoriert.

## 3. Versionierung

Organisationsweit Semantic Versioning: MAJOR.MINOR.PATCH.

Bei Protokoll-Repositories sind zu trennen (nie implizit gekoppelt):
Protocol Version · Implementation Version · Specification Version.
Beispiel: Protocol 1.0 / Spec 1.0 / Node 1.4.2. Fix-Referenzen wie die
Chain-ID 658467 sind Konstanten, keine Versionsnummern.

## 4. Release-Gate nach Compliance-Level

| Level | Release erfordert |
|---|---|
| R0/R1 | CI gruen |
| R2 | CI + Tests + SECURITY.md vorhanden + Changelog gepflegt |
| R3 | + Security-Checks gruen + Operations-Doku aktualisiert |
| R4 | + Differential-/Verifier-Kriterien (AD-021/022) + Freeze-Review |

## 5. Secrets & Credentials

Keine Secrets im Repository (Detailregeln ATC-STD-201 Abschnitt 6).
`.env.example` dokumentiert die erwarteten Variablen OHNE Werte.
Secret-Rotation ueber Plattform-/Deployment-Mechanismen, nie ueber Commits.

---

## 6. Branching-Standard (Abschnitt 21, MUST ab R3; SHOULD ab R2)

`main` enthaelt NUR getesteten, reviewten Code. Branches: `feature/*`
(Funktionalitaet), `fix/*` (Bugfixes), `security/*` (Security-Fixes),
`release/*` (Release-Vorbereitung). Direkte Pushes auf main bei S3/S4
empfehlen Fast-Forward-Merges ueber PRs.

## 7. Commit-Standard (Abschnitt 22, MUST ab R2)

Conventional Commits: `type(scope): description` mit den Typen
feat, fix, docs, refactor, test, security, perf, build, ci, chore, spec.
Breaking Changes MUSSEN im Commit-Body mit `BREAKING CHANGE:` markiert werden.

## 8. Pull-Request-Standard (Abschnitt 23, MUST ab R2)

Jeder PR MUSS liefern: Summary, Changes, Architecture Impact, Security Impact,
Breaking Changes, Tests, Documentation, Migration. Fuer kritische Komponenten
(S3/S4) ZUSAETZLICH: Threat Model Impact, Consensus Impact, State Transition
Impact, Backward Compatibility. Vorlage: templates/repository/PULL_REQUEST_TEMPLATE.md.

## 9. Release-Gates GATE-001…GATE-010 (Abschnitt 24, MUST ab R3; R4 alle zwingend)

| Gate | Pruefung |
|---|---|
| GATE-001 | Structure (ATC-STD-201 Matrix) |
| GATE-002 | Build/Compile reproduzierbar |
| GATE-003 | Unit-Tests |
| GATE-004 | Integration-Tests |
| GATE-005 | Security-Checks (Secret-Scan, Security-Tests) |
| GATE-006 | Dependency Audit (Policy-konform) |
| GATE-007 | API/Protokoll-Kompatibilitaet |
| GATE-008 | Dokumentation aktuell |
| GATE-009 | Architektur (Abhaengigkeitsrichtung, Layer-Grenzen) |
| GATE-010 | Release-Approvals + Artefakt-Hashes/-Signaturen |

Ein fehlgeschlagenes Gate = **NO-GO**. Reihenfolge: CI → Security → Tests →
Architecture → Documentation → Approved.

## 10. Dependency-Management (Abschnitt 25, MUST ab R2)

Abhaengigkeiten NUR deklarativ (Cargo.toml, package.json, pyproject.toml,
go.mod). Dependency Policy je Kategorie Approved/Restricted/Deprecated/Blocked
— strenger fuer Cryptography, Networking, Serialization, Smart Contracts, VM,
Consensus, OS-Kernel (S3/S4: MUST NOT blocked Abhaengigkeiten aufnehmen).
Keine unbekannten Copy/Paste-Komponenten im Core.

## 11. Third-Party-Code (Abschnitt 26, MUST ab R2)

Externe Komponenten MUSSSEN in docs/third-party/THIRD_PARTY.md dokumentiert
sein: Component, Version, License, Source, Purpose, Security Status, ATC Approval.

## 12. API-Stabilitaet & Breaking Changes (Abschnitte 27-28, MUST ab R2)

Stability-Level: EXPERIMENTAL → ALPHA → BETA → STABLE → FROZEN → DEPRECATED
(zu fuehren in .atc/repository.yaml api.stability). Breaking Changes MUSSSEN
explizit (BREAKING CHANGE: + CHANGELOG-Eintrag) markiert werden; bei
Protokoll-Versionen DUERFEN sie NICHT in Patches versteckt werden (Trennung
Protocol/Implementation/Specification, Abschnitt 3).

## 13. Reproducible Builds (Abschnitt 29, MUST ab S4; SHOULD ab S3)

Gleicher Source + gleiche Toolchain + gleiche Build-Konfiguration →
identisches Artefakt. Dafuer: build/toolchain.lock + build-manifest.json +
reproducibility.md. Betrifft insbesondere: ATC Node, ATC Core, ATCLang
Compiler/VM, Globus OS, ShivaCore, Wallet.

## 14. Artifact-Management (Abschnitt 30, MUST ab R3)

Generierte Releases gehoeren NICHT unkontrolliert in Git. Release-Artefakte
MUSSSEN SHA-256- und SHA-512-Checksummen besitzen; bei kritischen Komponenten
(S3/S4) ZUSAETZLICH Signaturen.

## 15. Health Score (Abschnitt 19, MUST-Verfahren)

Der ATC-Validator (atc-repo-audit) erzeugt je Repository einen Score ueber
die Kategorien Structure, Documentation, Testing, Security, CI/CD, Ownership,
Versioning, Dependencies. Schwellenwerte:

| Score | Status |
|---|---|
| 95-100 | EXCELLENT |
| 85-94 | COMPLIANT |
| 70-84 | CONDITIONAL |
| 50-69 | NON-COMPLIANT |
| <50 | CRITICAL |

GATE: PASS erfordert Score >= 85 UND 0 MUST-FAILs (ATC-STD-201 §10).

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

**Naming-Anpassung (07.09.):** Gate-IDs per ATC-STD-000 §36 auf
dreistellige Form migriert (GATE-001…10 → GATE-001…010; Candidate-Revision
vor Approval, Gate-Substanz unveraendert).
