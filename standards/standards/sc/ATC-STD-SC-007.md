---
standard:
  id: ATC-STD-SC-007
  title: "ATC-STD-SC-007 — Smart Contract Upgrade Standard"
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

# ATC-STD-SC-007 — Smart Contract Upgrade Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Default ist Immutabilitaet. Upgradeable Contracts deklarieren Mechanismus, Admin und Timelock explizit und unterliegen zusaetzlichen Governance- und Security-Gates.

## Scope

Gilt fuer alle Contracts mit Upgradeability; Verzahnung mit ATC-STD-204 (Interface-Stabilitaet) und Governance-Standards.

## 1. Deklarationspflicht (REQ-SC-020)

Jeder Contract MUSS explizit deklarieren: `upgradeability: {enabled: false}` (immutable, Default) oder `upgradeability: {enabled: true, mechanism: proxy, admin: ATC-Governance, timelock: required}`.

## 2. Zusatz-Gates (REQ-SC-021)

Upgradeable Contracts BENOTIGEN zusaetzliche Governance- und Security-Gates: Storage-Collision-Pruefung (SC-003), Upgrade-Tests (SC-004 §1), Timelock-Pflicht.

## 3. Default: Immutable (REQ-SC-022)

Ohne explizite Deklaration gilt ein Contract als immutable; Upgradeability muss begruendet per DEC-Record (ATC-ENT-002) entschieden sein.




## 4. Storage-Collision-Schutz (REQ-SC-075)

Upgradeable Contracts MUESSEN automatisierte Storage-Layout-Vergleiche in CI/CD nutzen, um Storage Collisions zwischen Versionen auszuschließen.

## 5. Standardisierte Proxy-Muster (REQ-SC-076)

Als Upgradeability-Mechanismus MUESSEN ausschließlich erprobte Standards wie UUPS oder Transparent Proxies verwendet werden.

## 6. Zweiphasiges Upgrade-Verfahren (REQ-SC-077)

Jedes Upgrade MUSS eine zweiphasige Propose-Timelock-Execute Sequenz durchlaufen, um der Governance und den Nutzern Reaktionszeit zu gewähren.

## 7. Initializer-Reentrancy-Schutz (REQ-SC-078)

Initialisierungsfunktionen von Logik-Contracts MUESSEN durch initializer-Modifier gegen mehrfache Ausführung und Bypasses geschützt werden.

## 8. Metriken & Akzeptanzkriterien

- Storage Layout Compatibility: MUSS 0 Layout-Konflikte im CI-Scan nachweisen.
- Timelock-Dauer: MUSS mindestens 172.800 Sekunden (48 Stunden) betragen.
- Proxy Standard Konformität: MUSS UUPS / ERC-1967 Konformität zu 100% erfüllen.
- Upgrade Test Pass-Rate: MUSS 100% bei State-Migrationstests auf dem Testnet betragen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-020 | Deklarationspflicht (§1) | MUSS |
| REQ-SC-021 | Zusatz-Gates (§2) | MUSS |
| REQ-SC-022 | Default immutable (§3) | MUSS |
| REQ-SC-075 | Storage-Collision-Schutz | MUSS |
| REQ-SC-076 | Standardisierte Proxy-Muster | MUSS |
| REQ-SC-077 | Zweiphasiges Upgrade-Verfahren | MUSS |
| REQ-SC-078 | Initializer-Reentrancy-Schutz | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Upgrade-Mechanismen stellen ein erhöhtes Angriffspotenzial dar. Unbefugte Upgrades oder Storage-Verwüstungen MUESSEN durch strikte Governance-Schranken und technische Timelocks verhindert werden.

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

- **NORMATIVE:** ATC-STD-000 (Verfassung), ATC-STD-201/202, ATC-STD-203
  (Security), ATC-STD-204 (Dependency & Interface), ATC-STD-SC-001
  (Familien-Hauptstandard)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025 (Agenten-Standards), AI-DEV-001..012,
  contracts/registry/ (Contract Registry), ATC-STD-BUG-001..005 (Bug Standards)
