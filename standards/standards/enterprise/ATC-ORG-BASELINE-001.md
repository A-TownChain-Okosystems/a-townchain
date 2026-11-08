---
standard:
  id: ATC-ORG-BASELINE-001
  title: "ATC Organization Engineering Baseline (Sicherheit, Geschwindigkeit, Stabilit\u00e4t als einheitliches Engineering-Governance-System)"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle Repositories der Organisation A-TownChain-Okosystems"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-GOV-001
    - ATC-STD-LIB-001
----

# ATC-ORG-BASELINE-001 — Organization Engineering Baseline (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Direktive 11.09.2026 17:30 (\u201eSicherheit, Geschwindigkeit und
> Stabilit\u00e4t als EIN einheitliches Engineering-Governance-System\u201c) + Freigabe 17:33;
> SCR-0103. Ableitungsprinzip: eine organisationweite Baseline definiert je Repository-Tier
> verbindliche Gates — Sicherheit, Geschwindigkeit und Stabilit\u00e4t werden systemisch erzwungen,
> nicht je Repository gepflegt.

## Abstract

Dieser Standard definiert die ATC Organization Engineering Baseline: f\u00fcnf technische
Schutzschichten (Security, Performance, Stability, Quality, Governance), eine
Repository-Tier-Klassifikation T0-T4 mit maschinell pr\u00fcfbarer Ableitungsregel, ein
einheitliches Merge-Gate je Tier sowie die P0-P3-Engineering-Roadmap mit ehrlichem
IST-Stand (SOLL/IST-Analyse 11.09.2026). Er ist das Ableitungsfundament: aus der
Baseline werden je Tier die erforderlichen Gates automatisch abgeleitet.

## par.1 Zweck und Scope

Sicherheit, Geschwindigkeit und Stabilit\u00e4t werden nicht als drei getrennte Themen
behandelt, sondern als ein einheitliches Engineering-Governance-System. Scope:
Organisation, alle 27 governed Repositories, alle Pipelines, alle Releases.

## par.2 F\u00fcnf Schutzschichten

| Schicht | Ziel |
|---|---|
| Security | Supply-Chain, Secrets, Code, Releases und CI absichern |
| Performance | Build-, Test-, CI- und Runtime-Zeiten reduzieren |
| Stability | Keine instabilen \u00c4nderungen in main |
| Quality | Jede \u00c4nderung automatisch verifizieren |
| Governance | Einheitliche Regeln f\u00fcr alle Repositories |

Baseline-Zielniveau (intern, je Schicht 10/10): Security, Stability, Reliability,
Performance, CI/CD, Governance, Supply Chain, Reproducibility, Documentation,
Observability. Behauptungen nur mit Evidence (No-Evidence-No-Claim, ATC-STD-LIB-001).

## par.3 Repository-Tiers T0-T4

Ableitungsregel (maschinell erzwungen, Pr\u00fcfregel R15 im Cross-Registry-Test):
T0 = {.github, atc-standards}; T1 = Owner-Liste {a-townchain, atclang, atc-shivacore,
atc-zkp, atc-wallet, atc-algorithm} PLUS Regel (criticality C1 UND security_class S4);
T2 = Infrastruktur; T3 = Applikationen; T4 = Experimental (Security-Floor bleibt).

Aktuelle Zuordnung (27 Repos, SSOT-Feld `tier` je repositories.yaml-Eintrag):
- **T0 Governance:** .github, atc-standards — maximale Governance, PR-Pflicht, 2 Reviews.
- **T1 Core/Security-Critical:** a-townchain, atclang, atc-shivacore, atc-zkp, atc-wallet,
  atc-algorithm, atc-vm, atc-node, atc-contracts, atc-interop, atc-oracle, globus-os —
  maximale Security + Reproduzierbarkeit, CodeQL, CODEOWNERS-Review, signierte Releases (Ziel).
- **T2 Infrastructure:** a-townchain-os, a-townchain-os-docs, atc-storage, atc-compute,
  atc-mining, atc-indexer — hohe Stabilit\u00e4tsanforderungen.
- **T3 Applications:** aurora-ai, atc-sdk, atc-explorer, atc-marketplace, atc-launchpad,
  genesis-engine, genesis-chronicles — Standard Enterprise CI/CD.
- **T4 Experimental:** (aktuell leer) — lockere Merge-Regeln, KEINE Lockerung
  fundamentaler Security-Regeln.

## par.4 Einheitliches Merge-Gate (Ableitung je Tier)

    MERGE_ALLOWED =
        build == PASS
    AND tests == PASS
    AND security == PASS
    AND dependency == PASS
    AND required_reviews == PASS
    AND repository_standard == PASS
    AND (tier <= T1: codeowners == PASS)

Reviews je Tier: T0 = 2 Reviews (Owner + CODEOWNERS), T1 = 2 Reviews (davon 1
fachlicher CODEOWNER), T2/T3 = 1 Review, T4 = Security-Floor. Kein Repository kann
seine Qualit\u00e4tsregeln umgehen; KEIN Agent ist je Reviewer oder Approver (par.9,
ATC-STD-000/ATC-STD-003). Branch-Protection-Enforcement ist Owner-Aktion (F-122).

## par.5 Engineering-Roadmap P0-P3 mit ehrlichem IST-Stand (11.09.2026)

- **P0 SOFORT:** Org-Rulesets/Branch-Protection (IST: 1/26 gesch\u00fctzt — F-122 Owner),
  CODEOWNERS 26/26 ERF\u00dcLLT, Least-Privilege weitgehend ERF\u00dcLLT (F-051 Rest),
  Secret-Scanning aktiv (1 offener Alert: google_api_key, a-townchain-os, TRIAGE offen),
  Dependabot (IST: 16/26, F-043), Dependency Review (IST: 0/26, Rollout SCR-0103),
  CodeQL (IST: 6/26 — L\u00fccken in atc-shivacore, atc-wallet, atc-zkp, atc-node, atclang;
  a-townchain-os legitim exempt, F-133), Actions-SHA-Pinning (IST: 0/130 Referenzen,
  F-134), SECURITY.md 26/26 ERF\u00dcLLT, Release Protection (IST: 0, F-135), keine direkten
  Pushes auf Core-Repos (IST: 25/26 ungesch\u00fctzt — F-122).
- **P1 DANACH:** REPO-AUDIT-002 laufend ERF\u00dcLLT (64 Checks, w\u00f6chentlicher Org-Scan),
  Compliance-Score ERF\u00dcLLT (Readiness/Org-Scan), einheitliche CI (IST: divergent 1-4
  Workflows je Repo), SBOM 0 (F-135), signierte Releases 0 (F-135), Reproducible Builds
  0 (F-135), Performance-Benchmarks 0, automatische Regression Gates 0.
- **P2 OPTIMIERUNG:** CI-Caching 0/26, Parallelisierung, zentrale Reusable Workflows
  (Hub ist PR-gated), Build-Matrix, Test-Pyramide, automatische Dependency-Updates
  (Dependabot aktiv).
- **P3 ENTERPRISE-AUSBAU:** SLSA/Supply-Chain-H\u00e4rtung, OpenSSF Scorecard, Security
  Dashboard, Repository Health Dashboard, Release Evidence System (Proto vorhanden:
  Evidence-Gate 26/26 mit evidence.yaml), organisationweite Engineering-KPIs.

## par.6 Architektur-Verortung

ATC ist Security/Integrity Root: Governance Layer (atc-standards) \u00fcber Security
(CodeQL, Supply Chain) und Quality (Tests, Gates) \u2192 Release (Signed Artifact).
KI-Instanzen (Aurora/ATS) d\u00fcrfen keine fundamentale Sicherheitsentscheidung
\u00fcberschreiben: KI erkennt, Policy entscheidet, System f\u00fchrt aus.

## par.7 Anforderungen (REQ-ORG-BASELINE-001..010)

- REQ-ORG-BASELINE-001 (MUSS): Jedes governed Repository f\u00fchrt ein `tier`-Feld
  (T0-T4) in der Repository-Registry; Werte werden per R15 gepr\u00fcft.
- REQ-ORG-BASELINE-002 (MUSS): Tier-Zuordnung folgt par.3-Regel; C1+S4 MUSS T0/T1 sein.
- REQ-ORG-BASELINE-003 (MUSS): MERGE_ALLOWED (par.4) gilt je Tier abgeleitet;
  kein Merge ohne alle PASS-Gates.
- REQ-ORG-BASELINE-004 (MUSS): T0/T1-Repos ben\u00f6tigen CODEOWNERS-Review; Agenten
  sind nie Approver.
- REQ-ORG-BASELINE-005 (MUSS): Keine ungepr\u00fcfte Dependency erreicht einen
  Security-Critical-Release (T1).
- REQ-ORG-BASELINE-006 (MUSS): Security-Floor (Least-Privilege, Secret-Scanning,
  SECURITY.md, keine Secrets im Repo) gilt ausnahmslos, auch T4.
- REQ-ORG-BASELINE-007 (SOLLTE): Performance-Gates mit Regressions-Schwellen
  (z.B. >10 % Build-/Testzeit-Regressions = FAIL) je Core-Repo.
- REQ-ORG-BASELINE-008 (SOLLTE): Reproducible Builds + Hash + Signatur fuer
  Blockchain, VM, Compiler, Kernel, Cryptography, Wallet, ZKP, Release-Artefakte (T1).
- REQ-ORG-BASELINE-009 (MUSS): Baseline-Aenderungen nur via SCR; MAJOR (z.B. Tier-
  Umordnung mit Enforcement-Folgen) nur mit Owner-Freigabe (COMPAT-001).
- REQ-ORG-BASELINE-010 (MUSS): IST-Stand-Angaben in par.5 sind Evidence-gebunden;
  Veraenderungen nur mit maschinenlesbarem Nachweis (No-Evidence-No-Claim).

## par.8 Change Control

\u00c4nderungen ausschliesslich via SCR; Tier-Umordnungen mit Enforcement-Folgen sind
MAJOR (COMPAT-001). Dieser Standard wurde aus der Owner-Direktive 11.09.2026 17:30
abgeleitet (SCR-0103) und mit der Freigabe 17:33 par.9-freigegeben.
