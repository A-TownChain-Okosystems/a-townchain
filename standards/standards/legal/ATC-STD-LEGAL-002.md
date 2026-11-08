---
standard:
  id: ATC-STD-LEGAL-002
  title: "ATC-STD-LEGAL-002 — Impressum & Anbieterkennzeichnung Standard"
  version: "1.0.0"
  status: approved
  category: legal
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-13"
  updated: "2026-09-13"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-14"
  review_date: null
  applies_to: "Alle öffentlich zugänglichen ATC-Ressourcen (Websites, Web-Apps, Docs, Pages, Wikis, Portale)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-LEGAL-002 — Impressum & Anbieterkennzeichnung Standard (v1.0.0, APPROVED — SCR-0124)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) (1.0.0-RC1) — freigegeben per Sammelfreigabe SCR-0124 gemäß ATC-STD-000; bis dahin nicht normativ in Kraft
> **Version:** 1.0.0 (FORMAL, APPROVED per Sammelfreigabe SCR-0124; vormals RC1) · **Datum:** 13.09.2026 · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-LEGAL-002 · **Scope:** Alle öffentlich zugänglichen Ressourcen der Organisation
> **Normative Sprache:** RFC 2119 / RFC 8174 (MUST/SHOULD/MAY)
> **Review Required:** Technical, Security, Architecture, Governance, Legal/Compliance
> **Registry:** registry/standards.yaml · **Kategorie:** legal · **SCR:** SCR-0117
> **Disclaimer:** Dieser Standard definiert einen Governance- und Compliance-Rahmen. Er ist KEINE Rechtsberatung. Gesetzliche Anforderungen gehen im Konfliktfall vor (§4, §45).

---

## 1. Purpose

This standard defines the requirements for the creation, publication, maintenance, verification, and governance of legal provider-identification information ("Impressum", "Provider Identification", or equivalent legal notice) for publicly accessible A-TownChain-Okosystems resources.

The objective is to ensure that applicable legal provider-identification requirements are handled consistently, verifiably, maintainably, and independently from individual repository implementations.

This standard establishes a compliance framework and does not constitute legal advice.

## 2. Scope

This standard applies to publicly accessible resources operated, published, maintained, or controlled by A-TownChain-Okosystems where provider-identification requirements may apply. This includes, but is not limited to:

- official websites;
- project websites;
- GitHub Pages;
- documentation portals;
- public wikis;
- web applications;
- dashboards;
- explorers;
- marketplaces;
- developer portals;
- public APIs with a user-facing web interface;
- hosted services;
- public landing pages;
- public project portals;
- downloadable software with an associated public service;
- other publicly accessible digital services.

The standard MAY also be applied to repositories and software projects where an explicit legal provider notice is appropriate.

## 3. Out of Scope

This standard does not itself define: privacy policies; cookie policies; terms of service; software licensing; third-party notices; copyright policy; financial regulation; securities regulation; tax compliance; consumer-protection compliance; export-control compliance; jurisdiction-specific legal advice.

These subjects SHALL be governed by separate policies or standards. Related standards SHOULD be maintained under the "LEGAL" standard family (siehe §43).

## 4. Normative Language

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL are to be interpreted as normative requirements (RFC 2119 / RFC 8174).

Where applicable law imposes requirements exceeding this standard, applicable law SHALL prevail.

## 5. Definitions

**5.1 Provider** — The natural person, legal entity, organization, or other legally responsible operator associated with a public resource.

**5.2 Provider Identification** — Information identifying the legally responsible provider of a public resource.

**5.3 Impressum** — A provider-identification notice intended to satisfy applicable legal requirements, particularly where German or comparable jurisdictional requirements apply.

**5.4 Canonical Source** — The authoritative source from which provider-identification data is maintained and distributed.

**5.5 Publication Target** — A website, application, documentation portal, repository, wiki, or other public resource where provider information is displayed.

**5.6 Legal Compliance Owner** — The person or organizational function responsible for ensuring that the applicable legal requirements are evaluated and maintained.

## 6. Governing Principles

**6.1 Legal Compliance** — A-TownChain-Okosystems resources MUST comply with applicable provider-identification requirements in the jurisdictions in which they are offered or operated.

**6.2 Single Source of Truth** — Provider-identification data SHOULD have one canonical authoritative source. Duplicated provider information MUST NOT become independently maintained authoritative copies.

**6.3 Consistency** — All publication targets using the same provider identity MUST use consistent and current information.

**6.4 Accessibility** — Where an Impressum or equivalent provider notice is legally required, it MUST be easily accessible from the relevant public resource.

**6.5 Accuracy** — Published provider information MUST be accurate and current. Known obsolete provider information MUST NOT remain publicly presented as the authoritative provider information.

**6.6 Auditability** — Changes to provider-identification information MUST be traceable through version control, change management, or an equivalent audit mechanism.

**6.7 Separation of Concerns** — Legal provider information MUST be separated conceptually from: technical architecture; source-code documentation; product documentation; privacy policy; licensing information; security policy.

## 7. Provider Identification Data Model

The canonical provider record SHOULD support the following fields (Referenz-Implementierung: `legal/imprint/provider.record.yaml`):

```yaml
provider:
  name: ""
  legal_name: ""
  legal_form: ""
  address:
    street: ""
    postal_code: ""
    city: ""
    country: ""
  contact:
    email: ""
    telephone: ""
    website: ""
  representative:
    name: ""
    role: ""
  registration:
    register_type: ""
    register_court: ""
    registration_number: ""
  tax:
    vat_id: ""
  responsible_person:
    name: ""
    address: ""
  additional_information: []
```

Fields SHALL only be published where applicable. Sensitive or unnecessary personal information MUST NOT be added merely for completeness.

## 8. Minimum Information Requirements

Where German provider-identification requirements apply, the published notice MUST be reviewed for the applicability of, among other things: provider/operator identity; legally required address; contact information; representation information where applicable; commercial-register information where applicable; VAT identification information where applicable; additional legally required information depending on the service and provider structure.

The exact mandatory fields SHALL be determined based on: (1) provider legal form; (2) nature of the service; (3) commercial activity; (4) jurisdiction; (5) target audience; (6) applicable statutory requirements.

The standard MUST NOT assume that every provider requires the same fields.

## 9. Publication Requirements

**9.1 Public Websites** — A public website subject to provider-identification requirements MUST provide an accessible provider notice.

**9.2 GitHub Pages** — GitHub Pages sites operated as public project or organization websites MUST include an appropriate provider notice where legally required.

**9.3 Documentation** — Public documentation portals MUST provide access to the applicable provider information where required.

**9.4 Web Applications** — A public web application MUST provide access to the applicable provider notice. The notice SHOULD be reachable from a persistent footer, legal menu, or equivalent navigation element.

**9.5 Repositories** — A source-code repository MAY contain a provider notice. A repository README MUST NOT be treated as the sole provider notice where applicable law requires a separately accessible legal notice.

## 10. Canonical Repository Structure

The canonical legal information SHOULD be maintained in the standards repository using a structure equivalent to:

```
atc-standards/
├── standards/
│   └── legal/
│       ├── ATC-STD-LEGAL-002.md
│       └── ...
├── registry/
│   └── standards.yaml          (Registry-Eintrag: ATC-STD-LEGAL-002)
└── legal/
    └── imprint/                (kanonische Anbieterdaten)
        ├── README.md
        └── provider.record.yaml
```

A project MAY maintain a generated or project-specific publication artifact. The canonical standard itself MUST remain in the standards repository.

## 11. Canonical Provider Record

Where multiple A-TownChain-Okosystems resources use the same provider identity, a canonical provider record SHOULD be maintained separately from individual websites (`legal/imprint/provider.record.yaml`). The provider record SHOULD be treated as the authoritative source for synchronized publication.

## 12. Publication Architecture

```
                 ┌──────────────────────┐
                 │ Canonical Provider   │
                 │ Record               │
                 └──────────┬───────────┘
                            │
                ┌───────────┼───────────┐
                ▼           ▼           ▼
             Website      Wiki       Docs
                │           │           │
                └───────────┼───────────┘
                            ▼
                    Compliance Check
```

Publication targets SHOULD consume the canonical provider information rather than independently re-entering the data.

## 13. Versioning

The standard SHALL use Semantic Versioning (MAJOR.MINOR.PATCH). Changes to legal requirements, scope, mandatory controls, or compliance gates MAY constitute a breaking standard change and SHALL be assessed accordingly.

Provider-data changes themselves SHOULD NOT require a new version of the standard (Standard: ATC-STD-LEGAL-002 v1.0.0; Provider data: Revision 2026-09-13).

## 14. Change Management

Changes to provider-identification information MUST follow controlled change management. Minimum process:

```
CHANGE REQUEST → IDENTIFY IMPACT → UPDATE CANONICAL DATA → LEGAL/COMPLIANCE REVIEW
→ CONSISTENCY VALIDATION → PUBLICATION → POST-PUBLICATION VERIFICATION
```

Changes affecting legal identity, address, representation, registration, or taxation SHOULD receive explicit compliance review.

## 15. Review Requirements

A change to this standard SHALL follow the applicable ATC standards governance lifecycle (ATC-STD-000). Recommended review chain:

Author → Technical Review → Security Review → Architecture Review → Legal/Compliance Review → Governance Approval → Candidate → Approved → Stable

ATC-STD-LEGAL-002 MUST NOT bypass the standard governance process solely because it is a legal standard.

## 16. Compliance Levels

Each publication target SHALL be classified:

- **L0 — Not Applicable:** No provider-identification obligation has been identified. Evidence MUST exist for the classification where the determination is non-obvious.
- **L1 — Required:** A provider notice is legally required or has been determined to be required by the responsible compliance function. The publication target MUST expose a valid provider notice.
- **L2 — Recommended:** A legal obligation has not been established, but publication is recommended for transparency.
- **L3 — Verified:** The provider notice has passed the applicable automated and manual compliance checks.

## 17. Compliance Requirements

- **LEGAL-002-REQ-001 — Applicability:** Each public publication target MUST have a documented applicability determination.
- **LEGAL-002-REQ-002 — Provider Identity:** A required provider notice MUST identify the legally responsible provider.
- **LEGAL-002-REQ-003 — Accuracy:** Published provider information MUST be accurate at the time of publication.
- **LEGAL-002-REQ-004 — Accessibility:** A required provider notice MUST be reasonably accessible from the relevant public resource.
- **LEGAL-002-REQ-005 — Currency:** Obsolete provider information MUST be removed or replaced when the authoritative provider information changes.
- **LEGAL-002-REQ-006 — Canonical Data:** Projects using centralized provider data SHOULD derive their published provider information from the canonical provider record.
- **LEGAL-002-REQ-007 — Change Traceability:** Changes MUST be traceable through an approved change-management mechanism.
- **LEGAL-002-REQ-008 — Validation:** A publication target classified as "L1" MUST pass the applicable compliance checks before release.
- **LEGAL-002-REQ-009 — Legal Review:** Material changes to legally relevant provider information MUST receive legal/compliance review.
- **LEGAL-002-REQ-010 — No False Representation:** A resource MUST NOT identify an individual or legal entity as provider where that entity has not been established as the responsible provider.
- **LEGAL-002-REQ-011 — Jurisdiction:** The responsible team MUST consider the jurisdictional requirements applicable to the target resource.
- **LEGAL-002-REQ-012 — Data Minimization:** Only legally required or operationally justified provider information SHOULD be published.
- **LEGAL-002-REQ-013 — Consistency:** Multiple official publication targets using the same provider identity MUST NOT intentionally publish contradictory provider information.
- **LEGAL-002-REQ-014 — Availability:** A required provider notice MUST remain available for as long as the associated publication target is publicly operated.
- **LEGAL-002-REQ-015 — Post-Change Verification:** After a material provider-data change, affected publication targets MUST be verified.

## 18. Compliance Gate

A release involving a public publication target classified as "L1" MUST NOT be considered compliant unless **[GATE-LEGAL-002]** has passed. Minimum gate criteria:

```
[PASS] Applicability determined          [PASS] Publication location verified
[PASS] Provider identified               [PASS] Canonical data verified
[PASS] Required fields reviewed          [PASS] No conflicting provider information
[PASS] Address reviewed                  [PASS] Legal/compliance review completed where required
[PASS] Contact information reviewed
[PASS] Jurisdiction reviewed
```

A missing evidence item MUST NOT be interpreted as "PASS" (SCR-0080-Ehrlichkeitsregel).

## 19. Evidence Requirements

Compliance MUST be evidence-based. Acceptable evidence MAY include: committed provider record; reviewed legal notice; publication URL; automated validation result; manual review record; change request; approval record; release artifact; deployment verification; legal/compliance review record.

The following MUST NOT be accepted as sufficient evidence by themselves: "looks correct"; "should be compliant"; "probably not required"; undocumented verbal approval; generated placeholder data.

## 20. Automated Validation

The ecosystem SHOULD implement automated validation. Recommended check identifier: **ATC-CHECK-LEGAL-002**. Minimum checks:

1. Provider record exists
2. Required fields are populated
3. No placeholder values remain
4. Provider data is syntactically valid
5. Publication target exists
6. Legal notice is reachable
7. Links resolve
8. Published data matches canonical data
9. No known conflicting provider records exist
10. Last verification timestamp exists

Automated checks MUST NOT claim legal compliance solely from successful syntax validation.

## 21. Placeholder Policy

Production publication targets MUST NOT contain placeholder provider data such as: `[NAME]`, `[ADDRESS]`, `TODO`, `TBD`, `Example Company`, `Lorem Ipsum`, `example@example.com`, `123 Example Street`.

A compliance gate MUST fail if known placeholder values are detected in a required provider notice.

## 22. Environment Separation

Development and test environments MAY use test provider data if clearly identified. Production environments MUST use the approved production provider information. The following states MUST be distinguishable: DEVELOPMENT, TEST, STAGING, PRODUCTION. A staging configuration MUST NOT accidentally become the production provider record.

## 23. Repository Integration

Repositories that publish a public service SHOULD contain a legal metadata section in their README (`## Legal` — Provider Identification/Impressum, Privacy, Terms). The README MAY link to the canonical provider notice. Eine README ist KEIN Ersatz für ein rechtskonformes Website-Impressum (§9.5).

## 24. Separation from License

```
LICENSE    → Defines software copyright and usage rights
IMPRESSUM  → Identifies provider/operator
PRIVACY    → Defines personal-data processing
TERMS      → Defines contractual/service conditions
```

The provider notice MUST NOT be confused with the software license. These documents SHOULD remain independently maintainable.

## 25. Third-Party Services

Where a public resource uses third-party infrastructure, the provider notice MUST identify the responsible provider/operator as required by applicable law. Hosting providers, CDNs, GitHub, cloud platforms, analytics providers, and similar services MUST NOT automatically be represented as the legal provider merely because they technically host the resource.

## 26. Decentralized Systems

A decentralized protocol introduces additional legal-compliance considerations. For blockchain protocols, smart contracts, nodes, explorers, wallets, and decentralized applications, this standard SHALL distinguish between: Protocol; Node Operator; Website Operator; Application Operator; Smart Contract Author; Foundation/Organization; Service Provider; Infrastructure Provider.

The existence of a decentralized protocol MUST NOT by itself be interpreted as eliminating provider-identification obligations for an identifiable operator of a public service.

## 27. Smart Contracts and On-Chain Components

Pure on-chain code does not automatically require an embedded Impressum. However, associated public interfaces MAY require provider identification:

```
Smart Contract ├── Explorer → provider notice may apply
                ├── Web UI   → provider notice may apply
                ├── API      → provider notice may apply
                └── Website  → provider notice may apply
```

The standard MUST NOT require personal legal information to be embedded directly into immutable blockchain state unless a separate legal requirement and approved architecture explicitly require it.

## 28. GitHub Organization

For the A-TownChain-Okosystems GitHub organization, the following hierarchy is RECOMMENDED: Organization Profile (MAY provide a central legal link) → atc-standards/legal/ → project repositories → public websites/applications. Each independent public service SHALL nevertheless be assessed individually.

## 29. Repository Classification

Every public repository SHOULD be assigned one of the following classifications: REPO-LEGAL-L0 (not applicable), REPO-LEGAL-L1 (required), REPO-LEGAL-L2 (recommended), REPO-LEGAL-L3 (verified). Example:

```yaml
legal:
  classification: L1
  standard: ATC-STD-LEGAL-002
  provider_source: canonical
  verified: true
  verified_at: "2026-09-13"
```

## 30. Audit Requirements

A legal compliance audit MUST verify: (1) provider identity; (2) applicability; (3) jurisdiction; (4) required information; (5) publication location; (6) accessibility; (7) accuracy; (8) consistency; (9) canonical-source relationship; (10) evidence; (11) change history; (12) affected deployment targets.

Audit findings SHALL use the ATC severity model where applicable: P0 — Critical; P1 — High; P2 — Medium; P3 — Low.

## 31. Non-Compliance

Examples of non-compliance include: missing required provider notice; inaccessible provider notice; materially incorrect provider identity; obsolete address; contradictory official provider information; placeholder information in production; missing evidence; unreviewed material provider changes; publication of information known to be false; failure of required compliance gate.

Non-compliance MUST be documented and assigned a remediation owner.

## 32. Remediation

Minimum remediation workflow:

```
FIND → CLASSIFY → DOCUMENT → ASSIGN OWNER → FIX → VERIFY → UPDATE EVIDENCE → CLOSE
```

A finding MUST NOT be closed solely because the source file was modified. The affected public deployment MUST also be verified where applicable.

## 33. Emergency Changes

Emergency legal changes MAY use an expedited process when delay could result in material legal or operational risk. The emergency process MUST: identify the reason; identify the approving authority; document the change; deploy the required correction; perform retrospective review. Emergency changes SHALL NOT permanently bypass normal governance.

## 34. Security and Privacy

Provider information itself may contain personal data. Implementations MUST therefore avoid unnecessary exposure of personal information. The following SHOULD be considered: access control for internal source records; protection against accidental secret inclusion; avoidance of private contact data where unnecessary; review of generated artifacts; repository history review where sensitive information was accidentally committed.

Secrets MUST NEVER be stored in the provider record (passwords, API keys, private keys, tokens, wallet seed phrases, authentication credentials).

## 35. Integrity

The canonical provider record SHOULD be version controlled. Material changes SHOULD include: change identifier; author; timestamp; reason; review status; approval; affected targets; deployment status. Where technically appropriate, publication artifacts MAY include a content hash.

## 36. Internationalization

A public resource targeting multiple jurisdictions MAY provide multiple language versions (z. B. `/legal/imprint`, `/legal/impressum`, `/legal/provider-information`). Translated versions MUST NOT introduce contradictory provider information. One language version SHOULD be designated as canonical for content governance.

## 37. Deprecation

A provider notice MUST NOT simply be deleted when a service is discontinued. Where legally required, archival or historical information SHOULD remain available for the legally appropriate period. The project owner SHALL determine applicable retention requirements.

## 38. Standard Lifecycle

This standard follows the ATC standards lifecycle (ATC-STD-000):

```
IDEA → PROPOSED → DRAFT → REVIEW → CANDIDATE → APPROVED → STABLE → DEPRECATED → RETIRED
```

The current lifecycle state is: **APPROVED** (Sammelfreigabe SCR-0124, 14.09.2026).

## 39. Acceptance Criteria for 1.0.0

ATC-STD-LEGAL-002 MAY advance to "CANDIDATE" when:

- [ ] scope has been reviewed;
- [ ] normative requirements have been reviewed;
- [ ] German applicability has been legally reviewed;
- [ ] international applicability has been reviewed;
- [ ] provider-data model has been approved;
- [ ] compliance gates have been validated;
- [x] registry entry exists;
- [x] metadata exists (Frontmatter, Haus-Konvention);
- [ ] repository integration has been tested;
- [ ] no unresolved P0/P1 standard defects remain.

"STABLE" additionally requires successful operational use and evidence according to the ATC governance lifecycle.

## 40. Conformance

A project conforms to this standard when all applicable mandatory requirements are satisfied and sufficient evidence exists. Conformance claims MUST identify: Standard ID; Standard Version; Project; Scope; Classification; Verification Date; Evidence; Reviewer. Example:

```yaml
conformance:
  standard: ATC-STD-LEGAL-002
  version: 1.0.0
  classification: L3
  verified: true
  verified_at: "2026-09-13"
```

## 41. Recommended Compliance Artifact

Each applicable project SHOULD maintain:

```
compliance/
└── legal/
    └── imprint/
        ├── applicability.yaml
        ├── evidence.md
        └── verification.yaml
```

This separates legal evidence from application code.

## 42. Registry Identity

The registry entry (registry/standards.yaml) contains at minimum:

```yaml
id: ATC-STD-LEGAL-002
title: "Impressum & Anbieterkennzeichnung Standard"
family: LEGAL
category: legal
version: 1.0.0-RC1
status: draft
normative: true
owner: ShivaCoreDev
repository: atc-standards
```

## 43. Related Standards

This standard SHOULD be used together with:

| ID | Titel | Status |
|---|---|---|
| ATC-STD-LEGAL-001 | Legal & Compliance Framework | GEPLANT — noch nicht registriert (Kein Eintrag = kein Standard) |
| ATC-STD-LEGAL-002 | Impressum & Anbieterkennzeichnung (dieser Standard) | APPROVED |
| ATC-STD-LEGAL-003 | Privacy & Data Protection Standard | GEPLANT — noch nicht registriert |
| ATC-STD-LEGAL-004 | Terms of Service Standard | GEPLANT — noch nicht registriert |
| ATC-STD-LEGAL-005 | Open-Source Licensing Standard | GEPLANT — noch nicht registriert |
| ATC-STD-LEGAL-006 | Third-Party Notices Standard | GEPLANT — noch nicht registriert |

Identifiers MUST be verified against the registry before assignment. Impressum, Datenschutz, Lizenz, Haftung und Nutzungsbedingungen werden NICHT vermischt — jede Domäne erhält einen eigenen Standard (Separation of Concerns, §6.7).

## 44. Implementation Rule

Projects MUST NOT create independent legal requirements that contradict this standard. Project-specific requirements MAY be stricter than this standard. Where a project requires additional legal controls, those controls SHOULD be documented as project-level compliance requirements.

## 45. Disclaimer

This standard defines an internal governance and compliance framework. It does not constitute legal advice. Applicable statutory requirements, court decisions, regulatory requirements, and professional legal advice SHALL take precedence over this standard. Where material uncertainty exists, the responsible project owner SHALL obtain appropriate legal/compliance review before publication.

## 46. Change Log

**1.0.0-RC1 (2026-09-13)** — Initial candidate specification (Owner-Mandat 13.09.2026, SCR-0117). Scope includes: provider identification; Impressum requirements; canonical provider data; publication requirements; compliance classification; compliance gates; evidence requirements; repository integration; decentralized-system considerations; audit and remediation; lifecycle and governance.

## 47. Final Requirement

A-TownChain-Okosystems MUST treat legal provider identification as a governed compliance concern rather than as arbitrary README content. Every applicable public resource MUST have a demonstrable answer to:

> «Who is legally responsible for this public resource, and where can the legally required provider information be verified?»

Failure to provide a defensible answer SHALL constitute a compliance finding.
