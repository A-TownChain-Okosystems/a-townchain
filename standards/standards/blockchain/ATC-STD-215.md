---
standard:
  id: ATC-STD-215
  title: "Tokenomics Standard"
  version: "1.0.0"
  status: approved
  category: blockchain
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Auftrag) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards:
    - ATC-STD-200
    - ATC-STD-208
    - ATC-STD-209
    - ATC-STD-210
    - ATC-STD-212
    - ATC-STD-NET-003
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-215 — Tokenomics Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — KONFLIKT-Aufloesung SCR-0040 (08.09.2026, Owner-Auftrag [§9-Sammelfreigabe 11.09.2026, approval/APPROVAL-DECISION-2026-09-11.md]
> "Konflikte beheben"): Katalog-Slot von kollidierender ID auf freien Slot im richtigen
> Namensraum umallokiert (ATC-STD-000 §36 ID-Immutabilitaet — belegte IDs bleiben unangetastet).
> Normativ per Owner-Auftrag; Engineering-Bindung entsteht bei Slot-Aktivierung via SCR/MINOR.
> **Scope:** ATC-STD-215 · **Reihe:** Familie Token Standards (FAM-12)

## Abstract

ATC-STD-215 (Tokenomics Standard) ist der Standard fuer den gleichnamigen Katalog-Slot. Er definiert
Gegenstand, Verortung im Oekosystem, Kernregeln, Pruefkriterien, Compliance- und
Verifikationspflichten sowie Security-Betrachtungen.

Schluesselwoerter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemaess ATC-STD-000 §10.


## §1 Gegenstand & Verortung

Das gesamtwirtschaftliche Token-Modell des ATC-Tokens: Value-Flows, Incentive-Struktur,
Verteilungsphilosophie und deren Governance. Operative Token-Mechaniken liegen in den
Nachbar-Standards: Supply (ATC-STD-212), Emission (213), Burning (214), Minting (205),
Staking Rewards (206), Treasury (207), Allocation/Vesting (209/210).

## §2 Kernregeln

1. **KR-1:** Das Tokenomics-Modell MUSS als versioniertes Dokument gefuehrt und in der Registry referenziert sein.
2. **KR-2:** Alle oekonomischen Parameter MUESSEN maschinenlesbar definiert sein (Parameter-Registry), bevor die Stufe Testnet erreicht wird.
3. **KR-3:** Incentives MUSS mit der Netzwerk-Sicherheit konsistent sein (Stake-/Slash-Gewichte gemaess ATC-STD-NET-002/007).
4. **KR-4:** Aenderungen am Tokenomics nach Mainnet erfordern den Upgrade-Prozess (ATC-STD-NET-006) inkl. Governance-Approval — direkte Parameteraenderungen sind VERBOTEN (MUST NOT).
5. **KR-5:** Vor jeder Promotion DUERFEN Tokenomics-Aenderungen nur mit abgeschlossener Simulation auf Testnet (GATE-012) erfolgen.
6. **KR-6:** Der Realwert-Status je Stufe (DEV-ATC/TEST-ATC wertlos, ATC real) MUSS gemaess ATC-STD-NET-001..003 erzwungen sein.


## §3 Schnittstellen & Kopplungen

- **Registry-Kopplung:** standards.yaml/versions.yaml (Version/Status S-14/S-19),
  Katalog-Slot in registry/framework.yaml (SCR-0040).
- **Governance-Kette:** SCR-0040 → UPDATE (UPD-G03 MINOR) → COMPAT (bei MAJOR) →
  AUDIT; Findings via registry/findings.yaml (F-NNN).
- **Nachbar-Standards:** siehe related_standards — Subsidiaritaet: konkretere
  Standards gehen vor.
- **Agenten-Bindung:** Vollmandat via AGENT_MANIFEST; Umsetzungspflicht je
  Slot-Aktivierung.

## §4 Metriken & Akzeptanzkriterien

- **M1:** Alle Kernregeln (§2) operationalisiert und einer Pruefungsart zugeordnet
- **M2:** 0 offene Widersprueche zu verwandten Standards (Registry-Graph azyklisch)
- **M3:** Nachweisfuehrung je Pruefkriterium (§6) bei Slot-Aktivierung via AUD-Record

Akzeptanz gilt als nachgewiesen, wenn die Kriterien in einem AUD-Record oder
Validator-Lauf dokumentiert sind; fehlende Nachweise werden als Findings gefuehrt.

## §5 Compliance & Verifikation

Compliance wird ueber die Gesamt-Validierung (CI, S-01..S-25) je Registry-Eintrag
geprueft: Metadaten-Vollstaendigkeit, Naming, Status-/Version-Konsistenz und
Registry-Konsistenz. Abweichungen werden als Findings (F-NNN) gefuehrt und nach
ATC-STD-BUG-001..005 bearbeitet.

## §6 Slot-Spezifikation — verbindliche Pruefkriterien

Jedes Kriterium ist normativ (MUST). Nachweis je Kriterium: Konzept-/Design-Dokument
plus AUD-Record, oder Validator-/Testlauf — je nach Art des Kriteriums.

- **P1** (MUST): Gegenstandsdefinition und -abgrenzung gegenueber Nachbarn — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P2** (MUST): Zustaendigkeiten und Nachweispflicht je Regel — Nachweis: Validator-/Testlauf bzw. dokumentierte Pruefung
- **P3** (MUST): Kernregeln (§2) operationalisiert und pruefbar — Nachweis: Test-/Validator-Abdeckung je Regel
- **P4** (MUST): Schnittstellen- und Abhaengigkeitspruefung (keine Zyklen, keine Doppelwahrheit) — Nachweis: Registry-Graph-Lauf
- **P5** (MUST): Metriken & Akzeptanzkriterien mit Nachweisfuehrung — Nachweis: AUD-Record bei Slot-Aktivierung

## Compliance

Geprueft per Validator-Lauf (S-01..S-25) und Review; Verstoss gegen MUST-Kriterien =
Finding (F-NNN) nach ATC-STD-BUG-001..004.

## Security Considerations

Sicherheitsrelevante Regeln dieses Standards sind als MUST markiert und werden
ueber die Findings-Registry (F-NNN) und Audits (AUD-NNN) nachverfolgt.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — KONFLIKT-Aufloesung SCR-0040 |

## References

**NORMATIVE:** ATC-STD-000 (Verfassung, §36 IDs), registry/framework.yaml (Slot-Definition),
registry/standards.yaml (SSOT-Bestand) · **INFORMATIVE:** SCR-0040, SCR-0034 (Slot-Fertigbau)
