---
standard:
  id: ATC-STD-ZKP-002
  title: "ATC-STD-ZKP-002 — Proof System Interface Standard"
  version: "1.1.0"
  status: approved
  category: zkp
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: null
  superseded_by: null

  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-ZKP-002 — Proof System Interface Standard (v1.1.0, APPROVED)

> **Version:** 1.1.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

Pluggable Proof Architecture: Beweissysteme (zunaechst Groth16, PLONK, Halo2, STARK) sind austauschbar, ohne die Blockchain-Architektur umzubauen. Definiert das ProofSystem-Interface (setup, prove, verify, serialize, deserialize, version).

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-201 | Jedes Beweissystem MUSS das ProofSystem-Interface (setup/prove/verify/serialize/deserialize/version) implementieren. | §S.2 |
| id: REQ-ZKP-202 | Die ZKP-Layer DARF sich nicht hart an ein einzelnes Beweissystem koppeln; Austauschbarkeit MUSS erhalten bleiben. | §S.2 |
| id: REQ-ZKP-203 | Neue Beweissysteme MUESSEN vor Zulassung ein Security-Review (ATC-STD-ZKP-010) durchlaufen. | §S.3 |
| id: REQ-ZKP-204 | Proof-Serialisierung MUESSEN versioniert sein; alte Formate MUSSEN deserialisierbar bleiben (Abwaertskompatibilitaet). | §S.2 |

| id: REQ-ZKP-205 | Das ProofSystem-Interface MUSS Timeouts für die Methoden prove() und verify() strikt erzwingen. | §S.2 |
| id: REQ-ZKP-206 | Fehlerhafte Serialisierungsdaten MÜSSEN zu einem kontrollierten Abbruch ohne Panik im Runtime-System führen. | §S.2 |
| id: REQ-ZKP-207 | Jedes registrierte Beweissystem MUSS eine eindeutige Versionskennung im Header der Proof-Datei mitführen. | §S.3 |
| id: REQ-ZKP-208 | Inkompatible Interface-Versionen MÜSSEN beim Laden vom System explizit abgewiesen werden. | §S.3 |

## Metriken & Akzeptanzkriterien

- **Metrik 1:** Maximale Ausführungszeit für verify() MUSS unter 200 ms liegen.
- **Metrik 2:** Kompatibilitätsrate der Deserialisierung älterer Versionen MUSS 100% betragen.
- **Metrik 3:** Fehlertoleranz bei ungültigen Proof-Eingaben MUSS 100% kontrollierte Abbrüche garantieren.
- **Metrik 4:** Testabdeckung aller ProofSystem-Interface-Methoden MUSS mindestens 98% betragen.


## Metriken & Akzeptanzkriterien

- **Metrik 1:** Maximale Ausführungszeit für verify() MUSS unter 200 ms liegen.
- **Metrik 2:** Kompatibilitätsrate der Deserialisierung älterer Versionen MUSS 100% betragen.
- **Metrik 3:** Fehlertoleranz bei ungültigen Proof-Eingaben MUSS 100% kontrollierte Abbrüche garantieren.
- **Metrik 4:** Testabdeckung aller ProofSystem-Interface-Methoden MUSS mindestens 98% betragen.
- **Metrik 5:** Laufzeitunterschiede zwischen verschiedenen Proof-System-Backends MÜSSEN unter 20% liegen.

## Compliance & Verifikation

Geprüft durch: (1) `atc-std-validator` (Registry-Sync S-14, Kopf-Sync S-19, Testläufe S-01 bis S-25), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen MÜSSEN als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 geführt und im Ticket-System protokolliert werden.

## Security Considerations

- Schnittstellen zum Proof-System MÜSSEN gegen Pufferüberläufe und Out-of-Memory-Angriffe abgesichert sein.
- Eingabedaten für prove() und verify() MÜSSEN vor der Verarbeitung einer strengen Längen- und Typprüfung unterzogen werden.
- Sicherheitsrelevante Interface-Änderungen MÜSSEN vor Release von der Security-Organisation geprüft werden.
- Serialisierte Proofs MÜSSEN frei von uninitialisierten Speicherbereichen sein.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung aus dem Owner-Entwurf ZKP-Layer (AD-045). Status: APPROVED |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung, v1.1.0 APPROVED)
- ATC-STD-100
- ATC-STD-201
- ATC-STD-BUG-001
- ATC-STD-ZKP-001
- ATC-STD-ZKP-002
- ATC-STD-ZKP-010

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie)
