# ATC-STD-REPO-003 — Repository Security & Release Standard
> **Status:** 📐 PROPOSED (v1.0.0) | **Datum:** 07.09.2026 | **Autor:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-REPO-003 | **Scope:** Alle produktiven (R2+) Repositories
> **Referenzen:** AD-022 (19 Gates, KEIN FREEZE vor G18 Security Audit), AD-027 (Lauffaehigkeits-Roadmap), ATC-STD-REPO-001/-002, Issue #69 (Dependabot-Audit)

---

## Abstract

ATC-STD-REPO-003 definiert Security-Gates, CI/CD-Mindeststandards, Versionierung
und Release-Disziplin. Das Compliance-Level aus ATC-STD-REPO-002 bestimmt die
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

Keine Secrets im Repository (Detailregeln ATC-STD-REPO-001 Abschnitt 6).
`.env.example` dokumentiert die erwarteten Variablen OHNE Werte.
Secret-Rotation ueber Plattform-/Deployment-Mechanismen, nie ueber Commits.
