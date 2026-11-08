---
document_id: ATC-DOC-GOV-001
title: Governance — atc-standards
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-14
standard: ATC-STD-MD-001
---

# Governance

atc-standards ist das Governance-Repository der A-TownChain-Okosystems und unterliegt dem A-TownChain Enterprise Governance Framework.

## Entscheidungsfindung

- **Verfassung:** ATC-STD-000 v1.3.0 (APPROVED) — maßgebliche Rangfolge, Change Control, Immutabilität, ID-Allokation und Security.
- **Entscheidungsautorität:** Owner-/Approver-Identität und Delegationen werden ausschließlich aus den autoritativen Authority-/Approval-Artefakten abgeleitet. Narrative Dokumente definieren keine davon abweichende kanonische Identität.
- **Enterprise-Layer:** ATC-ENT-001..015.
- **Agenten:** ATC-AAS-001..025 + Voll-Compliance-Mandat (`AGENT_MANIFEST.md`), Nachweis über AUD-Records (`.github/ai/audit/`).

## Änderungsprozess

Neue Standards: Registry-First → Validator → §9-Freigabe. Änderungen an APPROVED-Standards: ausschließlich SCR. Architektur-, API-, Security- und Consensus-kritische Änderungen erfordern den jeweiligen Review- und Approval-Prozess.

## Repository-Metadaten

Aktuelle Repository- und Governance-Zahlen werden nicht manuell gepflegt. Sie werden aus `registry/repositories.yaml` und den daraus generierten Views abgeleitet. Der aktuelle Registry-Stand enthält **31 Repository-Einträge**; `demo-repository` ist als nicht kanonisches/exempt Repository markiert.
