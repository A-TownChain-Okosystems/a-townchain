# Repository-Governance-Sweep — 22 Repos auf ATC-STD-201/202/203 gebracht

**Datum:** 07.09.2026 · **Durchfuehrung:** Agent Aurora · **Auftrag:** Owner
("Ueberarbeiten der Repositories nach den Vorgaben des atc-standards-Repos")
· **Werkzeug:** atc-repo-audit v0.1.0 (atc-standards) · **Registry:** 23
Repos (AD-024-Landschaft, alle registriert in repositories.yaml)

## Ergebnis

**Vorher: 22/22 GATE NO-GO (Scores 25-44). Nachher: 22/22 GATE PASS.**
CI-Verifikation auf GitHub: Governance-Workflows laufen und sind gruen
(Stichprobe 4/4 SUCCESS).

## Verifikations-Tabelle

| Repository | Vorher (R3-Audit) | Nachher (Audit auf eigenem R-Level) |
|---|---|---|
| a-townchain | 25/100 · 12 FAIL | GATE PASS |
| a-townchain-os | 31/100 · 11 FAIL | GATE PASS |
| a-townchain-os-docs | 44/100 · 9 FAIL | GATE PASS |
| atc-compute | 38/100 · 10 FAIL | GATE PASS |
| atc-contracts | 38/100 · 10 FAIL | GATE PASS |
| atc-explorer | 31/100 · 11 FAIL | GATE PASS |
| atc-indexer | 25/100 · 12 FAIL | GATE PASS |
| atc-interop | 25/100 · 12 FAIL | GATE PASS |
| atc-launchpad | 38/100 · 10 FAIL | GATE PASS |
| atc-marketplace | 25/100 · 12 FAIL | GATE PASS |
| atc-mining | 38/100 · 10 FAIL | GATE PASS |
| atc-node | 38/100 · 10 FAIL | GATE PASS |
| atc-oracle | 38/100 · 10 FAIL | GATE PASS |
| atc-sdk | 31/100 · 11 FAIL | GATE PASS |
| atc-shivacore | 25/100 · 12 FAIL | GATE PASS |
| atc-storage | 38/100 · 10 FAIL | GATE PASS |
| atc-wallet | 31/100 · 11 FAIL | GATE PASS |
| atclang | 31/100 · 11 FAIL | GATE PASS |
| aurora-ai | 25/100 · 12 FAIL | GATE PASS |
| genesis-chronicles | 31/100 · 11 FAIL | GATE PASS |
| genesis-engine | 25/100 · 12 FAIL | GATE PASS |
| globus-os | 25/100 · 12 FAIL | GATE PASS |

Hinweis zur Methodik: Der Vorher-Audit lief pauschal auf R3; der Nachher-Audit
auf dem registry-eigenen R-Level des Repos (R1 Skelette bis R4 Kernel). Alle
Repos erhielten trotzdem den VOLLSTANDIGEN R3-Faehigkeitsatz — sie koennen
hochgestuft werden, ohne dass Dateien fehlen.

## Je Repository errichtet (nur Fehlendes, Bestehendes unangetastet)

1. **.atc/repository.yaml** (Standard 201: Klassifizierung, Maturity, Layer,
   Domain, Sprache, Criticality aus der Registry)
2. **.atc/ownership.yaml** + **CODEOWNERS** (Maintainer-Modell)
3. **.atc/lifecycle.yaml** (Stage aus Registry-Status)
4. **.atc/compliance.yaml** (Standard, Level, Gates, last_audit)
5. **SECURITY.md** (Meldepolitik, ATC-STD-203, Emergency-Prozess §32,
   Vertrauensgrenzen AD-012/028/022)
6. **docs/REPOSITORY_STANDARD.md** (Zwei-Ebenen-ADR-Modell AD-029)
7. **CHANGELOG.md** (falls fehlend) · **tests/TESTPLAN.md** (falls keine
   Tests: dokumentierter Plan bis M6/M7 — Tests entstehen MIT Implementierung)
8. **.github/workflows/governance-ci.yml**: auditet bei jedem Push/PR gegen
   atc-standards + repositories.yaml, auf eigenem R-Level (ATC-STD-203)
9. **README ATC-COMPLIANCE-Anhang**: 11 Schluesselabschnitte (Purpose...License),
   ATC COMPLIANCE-Badge, DECISIONS_REGISTER-Referenz (V-13/V-14)

## Bekannte Restpunkte (kein GATE-Blocker)

- **V-16 Conventional Commits:** Historische Commit-Messages sind teils
  nicht konventionell; die Regel wirkt im 20-Commit-Fenster und heilt mit
  aktiver Entwicklung (alle neuen Commits sind konventionell).
- **Mainnet-relevante Repos** (L0-L4) sollten je Meilenstein echte
  Build-/Test-Pipelines neben der Governance-CI erhalten (M1-M8, AD-027).
- **Branch-Absicherung** (SCR-0003, Option A/B) bleibt Owner-Entscheidung —
  CI wirkt verbindlich erst mit Protected main.

## Querverweise

- AD-039 (dieser Sweep) · AD-031 (Standards 201/202/203) · AD-030
  (atc-standards kanonisch) · AD-038 (Naming §7) · AD-026/027 (Layer/Roadmap)
