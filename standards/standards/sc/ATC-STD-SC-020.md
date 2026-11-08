---
standard:
  id: ATC-STD-SC-020
  title: "ATC-STD-SC-020 — AI-Assisted Smart Contract Development Standard"
  version: "1.1.0"
  status: approved
  category: sc
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle Smart Contracts des A-TownChain-Oekosystems"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-SC-020 — AI-Assisted Smart Contract Development Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

KI-Agenten duerfen Contracts nicht eigenmaechtig deployen: standardisierter Workflow von der Specification bis zur Registry-Aktualisierung mit Human/Governance-Approval-Gates — verbindet die AAS/AI-DEV-Agentenstandards mit der SC-Familie.

## Scope

Gilt fuer alle Agenten (ATC-AI-ARCH-001 etc.), die Smart Contracts spezifizieren, implementieren, testen oder deployen.

## 1. Agenten-Workflow (REQ-SC-056)

```text
AI Agent → Contract Specification → Requirement Validation → Implementation
→ Automated Tests → Security Scan → Human/Governance Approval → Testnet Deployment
→ Validation → Mainnet Approval → Deployment → Verification → Registry Update
```

## 2. Deploy-Verbot (REQ-SC-057)

Ein Agent DARF einen Smart Contract nicht ohne bestandene Gates deployen; Mainnet-Deployment erfordert explizite Governance-/Owner-Freigabe (AAS-017 Human Approval, SC-G9/G10).

## 3. Nachweispflicht (REQ-SC-058)

Agenten-Beitraege sind AUD-pflichtig (AI-DEV-009, AAS-010 Evidence): Task → Commit → Testergebnis → Gate-Stand gehoeren in den AUD-Record.




## 4. Automatisierter Gate-Durchlauf für KI-Code (REQ-SC-123)

Von KI-Agenten generierter Smart-Contract-Code MUSS alle automatisierten Gates SC-G0 bis SC-G6 ohne manuellen Override bestehen.

## 5. Human Governance Signaturpflicht (REQ-SC-124)

Ein Mainnet-Deployment von KI-generiertem Code MUSS eine kryptografische Signatur eines menschlichen Owners oder der DAO aufweisen.

## 6. Prompt- & Spec-Audit-Trail (REQ-SC-125)

Das zur Code-Generierung genutzte Prompt-Set und die Spezifikations-Referenz MUESSEN im Registry-Record verlinkt werden.

## 7. Synthetische Agenten-Regressions-Tests (REQ-SC-126)

Vor der Einreichung zu SC-G7 MUSS der KI-Agent einen synthetischen Regressions-Testsuite-Lauf mit mindestens 50 Testfällen nachweisen.

## 8. Metriken & Akzeptanzkriterien

- Gate Pass Rate: MUSS 100% erfolgreiche Durchläufe bei SC-G0..SC-G6 vorweisen.
- Human-Signatur Vollständigkeit: MUSS zu 100% vor SC-G10 vorliegen.
- Audit-Trail Verknüpfung: MUSS 100% der Prompt-Hashes in der Registry dokumentieren.
- Synthetische Testabdeckung: MUSS >= 50 KI-generierte Edge-Case Tests beinhalten.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-056 | Agenten-Workflow (§1) | MUSS |
| REQ-SC-057 | Deploy-Verbot ohne Gates (§2) | MUSS |
| REQ-SC-058 | AUD-Nachweispflicht (§3) | MUSS |
| REQ-SC-123 | Automatisierter Gate-Durchlauf für KI-Code | MUSS |
| REQ-SC-124 | Human Governance Signaturpflicht | MUSS |
| REQ-SC-125 | Prompt- & Spec-Audit-Trail | MUSS |
| REQ-SC-126 | Synthetische Agenten-Regressions-Tests | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

KI-basierte Entwicklung erhöht die Geschwindigkeit, gebietet jedoch zusätzliche Kontrollen. Kein KI-Agent darf autonom Verträge deployen; menschliche Letztentscheidung und ständige Verifikation sind zwingend.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## References

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), AAS-007 (Task), AAS-017 (Human Approval), AI-DEV-009 (Audit)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/, ATC-STD-BUG-001..005 (Bug Standards)
