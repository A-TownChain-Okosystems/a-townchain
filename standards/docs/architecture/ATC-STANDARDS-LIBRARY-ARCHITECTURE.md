---
document:
  id: ATC-STD-LIB-001
  title: "ATC Standards Library Architecture"
  version: "1.0.0"
  status: APPROVED
  authority: A-TownChain Ecosystems
  source: Owner-Direktive (Alexander, 11.09.2026 15:20 UTC+2) — als verbindliche Zielarchitektur v1.0 festgeschrieben
  supersedes: null
  normative: true
---

# ATC-STD-LIB-001 — ATC Standards Library Architecture v1.0.0

## 1. Rolle (normativ)

atc-standards ist die **kanonische Standards-Bibliothek des A-TownChain-Oekosystems**:
normative Kontrollinstanz, nicht bloesses Dokumenten-Repository.

> Canonical source of truth for all approved and governed standards of the A-TownChain ecosystem.

Verbindliche Grundregeln (Hard Rules):

- **No Registry Entry -> No Official Standard.**
- **No Evidence -> No Conformance.**
- **No Required Review -> No Approval.**
- **No Approval -> No Stable Status.**

Ein Standard hat genau EINE kanonische Quelle. Repositories referenzieren Standards
(.atc/, README, CI), duplizieren sie nie normativ. Jede normative Anforderung traegt eine
nachvollziehbare ID (REQ-STD-NNN-MMM, etabliert).

## 2. Sechs-Ebenen-Modell

1. GOVERNANCE — Lifecycle, Review, Approval, Change-Control (ATC-STD-000, UPDATE-001, COMPAT-001)
2. REGISTRY — IDs, Versionen, Status, Dependencies (registry/*.yaml, SSOT per ATC-STD-003)
3. NORMATIVE KNOWLEDGE — Standards, Requirements, Specifications
4. VERIFICATION — Tests, Evidence, Conformance (.atc/evidence, Test-Suiten, Org-Scan)
5. ECOSYSTEM INTEGRATION — Profiles, Repository-Compliance (.atc/standards.yaml, Phase 2)
6. AUTOMATION — Schemas, Validator, CI, generierte Views (R1-R12, gen_views, validate_all)

## 3. Ebenen-Trennung: Standard vs. Specification vs. Documentation

| Ebene | Zweck | ID-Form |
|---|---|---|
| Standard | Verbindliche normative Regel | ATC-STD-NNN |
| Requirement | Ueberpruefbare Anforderung | REQ-STD-NNN-MMM |
| Specification | Exakte technische Definition | ATC-SPEC-NNN |
| Test | Conformance-Verifikation | ATC-TEST-NNN |
| Documentation/Wiki | Erklaerung, Anwendung | — |

Hierarchie: ATC-STD -> REQ-STD -> ATC-SPEC -> ATC-TEST -> Evidence -> Conformance.

## 4. ID-Modell (mit Grandfathering)

IDs sind dauerhaft stabil (kein Renumbering bestehender Standards). Fuer NEUE Standards
gilt das Range-Modell (Familie aus der Nummer):

000-099 Governance/Meta | 100-199 Architecture | 200-299 Repository/Git | 300-399 Development
400-499 Security | 500-599 Cryptography | 600-699 Blockchain | 700-799 AI/ATS
800-899 OS/Runtime | 900-999 Interoperability | 1000-1099 Language/VM | 1100-1199 Data/Storage
1200-1299 ZKP | 1300-1399 Testing/Conformance | 1400-1499 Release/Supply-Chain
1500-1599 Enterprise | 1600-1699 Networking | 1700-1799 Identity/Wallet
1800-1899 Economics/Tokenomics | 1900-1999 Application | 2000+ Future

Die 474 Bestands-Standards behalten ihre IDs (Registry-Feld `family` bleibt die
verbindliche Familienzuordnung; Range-Konsistenz-Check gilt nur fuer neue Range-IDs).

## 5. Lifecycle (9 Status, erweitert)

IDEA -> PROPOSED -> DRAFT -> REVIEW -> CANDIDATE -> APPROVED -> STABLE -> DEPRECATED -> RETRIED

**APPROVED != STABLE:** STABLE erfordert zusaetzlich Reference Implementation,
Conformance Evidence und Stability Period (GATE-006..008). Der Status STABLE wird per
Validator-Regel ergaenzt (Phase 4); Bestands-Standards sind APPROVED, nicht STABLE —
ehrlicher Ist-Zustand (implementations: 65 enforced / 129 implemented / 265 specification_only).

## 6. Artefakt-Modell je Standard

Zielform (Phase 3, MAJOR per COMPAT-001): Ordner je Standard mit standard.md,
metadata.yaml, requirements.md, changelog.md, review.md. metadata.yaml enthaelt id,
title, version, status, family, normative, applies_to, dependencies, review-Anforderungen,
conformance.required — maschinenlesbar.

Heutige Ist-Form: flache Standard-Datei + Registry-SSOT-Eintrag (Felder id/title/version/
status/category/authority/owner/normative/file). Die Metadaten-Pflicht existiert bereits
(§6 ATC-STD-000); applies_to/dependencies/review-Felder werden per Generator ergaenzt
(Phase 1) — die Registry bleibt SSOT (Ein-Zahl-Regel: metadata.yaml wird aus der Registry
GENERIERT, nie andersherum gepflegt).

## 7. Gates (GATE-001..009)

001 Metadata valid | 002 Technical Review | 003 Security Review | 004 Architecture Review
005 Governance Approval | 006 Conformance Evidence | 007 Reference Implementation
008 Stability Evaluation | 009 Release Authorization

STABLE nur bei Erfuellung aller relevanten Gates. Entsprechung im Bestand: §9-Human-Gate
(Owner), Evidence-Gate (Claim-to-Artifact, alle 27 Repos), Cross-Registry-Test R1-R12 (CI).

## 8. Profiles und Repository-Compliance

profiles/<repo>/ (atclang, atc-vm, a-townchain, shivacore, aurora, globus-os) definiert
required_standards je Produkt; jedes Repository fuehrt .atc/standards.yaml (Profile +
required-Liste), CI validiert gegen die Registry. Standards Profile = verbindlicher
Compliance-Vertrag. (Koppelt F-124: Repository-Klassifikation als Voraussetzung.)

## 9. Zielstruktur (Auszug, verbindlich fuer Neuaufbau/Migration)

registry/ (SSOT: standards.yaml, versions.yaml, dependencies.yaml, findings.yaml,
families/, compliance_state.yaml, generated Schemas) | governance/ (lifecycle, review,
change-control, emergency, deprecation — nur Governance-Dokumente, keine Standard-Duplikate)
| standards/ (alle ATC-STD-* in einheitlicher Struktur, inkl. 000-governance-meta/ATC-STD-000
— der Verfassungsumzug ist Teil von Phase 3, MAJOR-gated) | requirements/ | specifications/
| tests/ | reference/ | decisions/ | change-requests/ | audits/ | profiles/ | tools/
(validator, registry, lint, dependency-check) | templates/ | docs/ | .github/ (workflows:
validate, registry, conformance, release; CODEOWNERS fuer normative Bereiche — Agenten nie
Approver, ATC-STD-003 §9).

## 10. Migrationsplan (phasig, jede Phase = SCR mit Gates)

- Phase 0 (SCR-0099, dieser Stand): Architektur v1.0.0 verbindlich festgeschrieben.
- Phase 1: metadata.yaml je Standard — GENERIERT aus der Registry (keine zweite Handpflege);
  Generator-Erweiterung + R13 (metadata/Registry-Konsistenz).
- Phase 2: profiles/ + .atc/standards.yaml je Repository + Org-Scan-Erweiterung (Profile-Check).
- Phase 3: Strukturumzug auf Ordner-je-Standard inkl. ATC-STD-000 nach
  000-governance-meta/ — MAJOR, COMPAT-001-Pruefung, Owner-Freigabe, R2/R3-Anpassung.
- Phase 4: STABLE-Status + GATE-006..008-Validierung + Validator-CLI (atc-standards validate).
- Phase 5: ATC-SPEC-*/ATC-TEST-* Hierarchien fuer kritische Familien (Konsens zuerst).

## 11. Aenderungskontrolle

Diese Architektur ist normativ und §30-aequivalent eingefroren. Aenderungen nur via
MAJOR (COMPAT-001) mit Owner-Freigabe. Bibliotheksversion (Library) ist unabhaengig von
Einzelstandard-Versionen.
