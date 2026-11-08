---
standard:
  id: ATC-STD-018
  title: "Technology Currency, Vulnerability & Security Assurance Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-10"
  updated: "2026-09-10"
  normative: true
  effective_date: "2026-09-10"
  review_date: "2027-09-10"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards:
    - ATC-STD-016
    - ATC-STD-017
    - ATC-STD-019
    - ATC-STD-020
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-018 — Technology Currency, Vulnerability & Security Assurance (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, §30-eingefroren) — Owner-Entwurf + §9-FREIGEGEBEN via Owner-Direktive
> 10.09.2026 (Security & Technology Assurance Familie 018/019/020). Kerngrundsätze:
> **»Latest suitable technology, not blindly latest technology«** (aktuell + stabil + sicher +
> kompatibel + geprüft + reproduzierbar) und **»No Evidence = No Security Claim«**.

## Abstract

ATC-STD-018 ist der verbindliche Standard für Technologie-Aktualität, Known-Issue-/Vulnerability-
Management und Security Assurance in der ATC Enterprise & Governance Familie. Er definiert
Technologie-Inventar-Pflicht, EOL-Verbot (ADR-Ausnahmen), CVE/GHSA/OSV-SLAs, zu prüfende
Angriffsklassen inkl. Blockchain-spezifischer, eine Security Baseline, ein CI-Security-Gate mit
Merge-Block, Technology-Reviews, den Technology Currency Score (TCS) und den Grundsatz
No Evidence = No Security Claim. Partner: ATC-STD-016/017 (Artefakt-Lifecycle),
ATC-STD-019 (Supply Chain), ATC-STD-020 (Incident Response).

## Scope

**Gilt:** Technologie-Aktualität, Known-Issue-/Vulnerability-Management, Security Assurance und
Security Gates für alle 27 ATC-Repositories (Runtime, Compiler, Frameworks, Dependencies, Build,
Container, Kryptografie, Protokolle, CI/CD, Security-Tooling).

**Gilt nicht:** Supply-Chain-Integrität im Detail (ATC-STD-019); Incident-Response-Ablauf
(ATC-STD-020); fachliche Architekturentscheidungen anderer Familien.

## §1 Technologie-Inventar (REQ-STD-001, MUSS)
Jedes Repository erfasst seine wesentlichen Technologien (Runtime, Language, Compiler, Framework,
Libraries, Dependencies, Build System, Package Manager, Database, Container/Base Images, CI/CD,
Kryptografie, Protokolle, Developer-/Security-Tools) je mit: Technology, Version, Latest Stable,
Support Status (Active/Maintenance/EOL), Security Status (Secure/Advisory/Vulnerable),
Update Policy (Patch/Minor/Major), Compatibility, Last Review, Evidence.

## §2 EOL-Verbot (REQ-STD-002, MUSS)
Kein Einsatz von End-of-Life-Technologie. Ausnahme nur mit dokumentiertem ATC-ADR-NNN:
Begründung, Risikoanalyse, Kompensationsmaßnahmen, Verantwortlicher, Ablaufdatum, Migrationsplan.

## §3 Known Vulnerability Management (REQ-STD-003, MUSS)
Regelmäßige Prüfung gegen CVE, GHSA, OSV, Vendor-/CERT-Advisories, RustSec, npm-/PyPI-Advisories,
Container-Advisories. SLA-Modell: CRITICAL sofortige Behandlung (Detect→Triage→Patch/Upgrade→Test→
Security Verification→Deploy); HIGH priorisierte Behebung im definierten SLA; MEDIUM planmäßig;
LOW dokumentierte Bewertung im normalen Lifecycle.

## §4 Known Attack Classes (REQ-STD-004, MUSS)
Regelmäßige Prüfung bekannter Angriffsklassen — nicht nur CVEs: Authentication/Authorization-
Angriffe, Injection, RCE, XSS, CSRF, SSRF, Path Traversal, Deserialization, Supply-Chain,
Dependency Confusion, Typosquatting, Credential Theft, Secret Leakage, Replay, DoS/DDoS,
Privilege Escalation, Container Escape, Memory Corruption, Side-Channel. Für Blockchain-Komponenten
zusätzlich: Double Spending, Long-Range, Nothing-at-Stake, Stake Grinding, MEV, Reorg, Finality-,
Eclipse-/Sybil-, Konsens-Manipulation, Bridge-Compromise, Cross-Domain-Replay, Invalid State
Transition, Reentrancy, Oracle-Manipulation.

## §5 Security Baseline (REQ-STD-005, MUSS)
Mindestmaßnahmen je Repository: Dependency Scanning, Secret Scanning, SAST, DAST (wo anwendbar),
Container Scanning, SBOM, License Scanning, Fuzzing (wo anwendbar), Unit-/Integration-/Security-
Tests, Build- und Artefakt-Integritätsverifikation. Für kritische Repos (C1/S4) zusätzlich:
Threat Model, Attack Surface Analysis, Penetration Testing, Fuzz Testing, formale Verifikation
(wo anwendbar), unabhängiger Security Review.

## §6 Security Gate im CI (REQ-STD-006, MUSS)
CI überprüft die Sicherheitsbehauptung — nicht nur Dokumentation: Pull Request → Dependency Scan
→ SAST → Secret Scan → SBOM → Known Vulnerability Check → Tests → Security Policy → PASS/FAIL.
Nicht akzeptables CRITICAL/HIGH-Finding: **MERGE = BLOCKED**.

## §7 Technology Review (REQ-STD-007, MUSS)
Regelmäßiger Review je Repository über Dependencies, Runtime, Compiler, Framework, OS, Container,
Kryptografie, Protokolle, Build System, Security Tooling. Verdikte: CURRENT, UPDATE_REQUIRED,
SECURITY_UPDATE_REQUIRED, MIGRATION_REQUIRED, EOL, BLOCKED.

## §8 Technology Currency Score (REQ-STD-008, MUSS)
TCS je Repository: 100 = vollständig aktuell; 90–99 aktuell; 75–89 akzeptabel; 60–74 Update
erforderlich; <60 veraltet. Security-Findings werden UNABHÄNGIG vom Score behandelt —
TCS 95 mit kritischer CVE ist NICHT sicher.

## §9 Evidence-only-Sicherheit (REQ-STD-009, MUSS)
»No Evidence = No Security Claim«: Status VERIFIED nur mit Nachweisen (Dependency Scan, SAST,
Secret Scan, SBOM, Tests, Fuzzing, Security Review); andernfalls ehrlich NOT VERIFIED.
Kein »Secure«, wenn lediglich keine Probleme bekannt sind.

## §10 Security & Technology Lifecycle (REQ-STD-010, MUSS)
Verbindlicher Ablauf: DISCOVER → IDENTIFY → ASSESS → PRIORITIZE → UPDATE/PATCH/MITIGATE → TEST →
VERIFY → DOCUMENT → RELEASE → MONITOR.

## §11 Maschinenlesbare Statusfelder (REQ-STD-011, MUSS)
Je Repository (z. B. .atc/evidence/): technology_status, security_status, vulnerability_status,
eol_status, dependency_status, last_security_review, last_technology_review — Werte z. B.
CURRENT/UPDATE_REQUIRED, VERIFIED/NOT_VERIFIED, CLEAR/OPEN, MIGRATION_REQUIRED.

## §12 Org-weites Assurance Gate (REQ-STD-012, MUSS)
Technology Audit (EOL/Updates/Compat.) und Security Audit (CVE/SAST/Secrets) münden in ein
Evidence Gate: PASS → Release, FAIL → Block. ATC-STD-018 ist damit prüfbare Compliance-
Anforderung, keine Empfehlung.

## §13 Freigabe
FREIGEGEBEN 10.09.2026 via Owner-Direktive (Owner-Entwurf inhaltlich übernommen). Priorität P1.

## §30 Freeze & Change-Control
§30-eingefroren; Änderungen ausschließlich via ATC-STD-UPDATE-001 (PATCH/MINOR/MAJOR mit
COMPAT-001-Gate).

## Security Considerations
Der Standard selbst erzeugt keine falsche Sicherheit: Jede Behauptung ist an Evidence gebunden
(REQ-STD-009). Kryptografie-Relevanz: PQC-Migration nur als MAJOR via ATC-CRYPTO-001.
