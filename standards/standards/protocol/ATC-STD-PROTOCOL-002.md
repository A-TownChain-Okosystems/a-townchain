---
standard:
  id: ATC-STD-PROTOCOL-002
  title: "ATC Protocol Conformance- & Interoperabilitäts-Test-Standard — verbindliche Testpyramide je Protokollfamilie: CONF-Pläne, Testkategorien, Konferenz-Stufen, Konformance-Registry, active-Gate"
  version: "1.0.0"
  status: approved
  category: protocol
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
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
    - ATC-STD-PROTOCOL-001
    - ATC-STD-AUDIT-001
    - ATC-STD-MILESTONE-001
    - ATC-STD-FRAMEWORK-001
  related_standards:
    - ATC-STD-PROTOCOL-003
    - ATC-STD-UPDATE-001
    - ATC-STD-COMPAT-001
    - ATC-STD-BUG-005
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-PROTOCOL-002 — Conformance- & Interoperabilitäts-Tests (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, §30-eingefroren) — 123. Standard; Agenten-Review + Integration SCR-0029;
> Owner-§9-Freigabe ausstehend. Bei Freigabe: APPROVED, normativ, §30-eingefroren.
> **Familie:** Protocol Governance (Katalog FAM-42) — schließt mit PROTOCOL-003
> die offenen Flanken des Protokoll-Dachstandards.

## Abstract

ATC-STD-PROTOCOL-002 macht Konformanz messbar: Jede Protokollfamilie
(ATC-PROTO-*) MUSS einen versionierten Conformance-Test-Plan (CONF-Plan) besitzen,
der die 10 Pflicht-Testkategorien abdeckt — Envelope-Roundtrip, kanonische
Serialisierung (Determinismus), Handshake-Ablaüfe, Replay-Schutz, Rate-Limiting,
Fehlercodes, Kompatibilitätsmatrix, Interoperabilität (Node-Paare), Malformed-
Input/Parser-Robustheit und Security-Basis (PROTOCOL-003-Kopplung). Ergebnisse
werden in der Conformance-Registry (registry/protocol-conformance.yaml) geführt;
die Stufe CONF-BRONZE ist Voraussetzung für `active` in der Protokoll-Registry
(schärft REQ-PROTO-021 von „implementiert" zu „konform getestet").

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Alle 26 Protokollfamilien der registry/protocol-registry.yaml —
`draft`-Familien für den Weg zu `active`; `planned`-Familien ab erster
Implementierungsspur. **Gilt nicht:** Reine Governance-Standards ohne Wire-Format.

## §1 Conformance-Test-Pläne (CONF-Pläne)

Je Familie existiert ein CONF-Plan mit ID `CONF-[FAMILY]-NNN` (z. B.
CONF-P2P-001), Version semver, Referenz auf die Spezifikationsversion, die er
verifiziert, und auf die 10 Pflicht-Kategorien (§2). Der Plan lebt neben der
Spezifikation (protocols/[family]/) und MUSS mit der Spezifikationsversion
mitversioniert werden (REQ-CONF-001).

## §2 Die 10 Pflicht-Testkategorien

1. **Envelope-Roundtrip:** Alle Pflichtfelder serialisieren/parieren verlustfrei
2. **Kanonische Serialisierung:** Determinismus — gleiche Daten, gleiche Bytes;
   Abweichung = FAIL (REQ-CONF-002)
3. **Handshake-Abläufe:** Happy Path + jede Abbruchphase; kein Datenverkehr vor
   abgeschlossenem Handshake
4. **Replay-Schutz:** Nonce/Message-ID/Timestamp-Fenster je Absender
5. **Rate-Limiting:** Token-Bucket-Trigger, Fairness je Peer
6. **Fehlercodes:** Jeder Fehlerkatalog-Eintrag MUSS mindestens einen Test haben
   (REQ-CONF-003)
7. **Kompatibilitätsmatrix:** Version [current-1..current] × [current], inkl.
   Kompatibilitätsmodus der Spezifikation
8. **Interoperabilität:** Zwei unabhängige Implementierungen (mindestens Node-Paar
   im Simulator) tauschen alle Message-Types aus (REQ-CONF-004)
9. **Malformed Input:** Truncation, Feld-Fuzzing, Übersprungene Felder — Parser
   MUSS kontrolliert abweisen (kein Panic/Overflow)
10. **Security-Basis:** Signaturprüfung, Fälschungsversuche, Domain-Separation
    (Details PROTOCOL-003)

## §3 Conformance-Stufen

| Stufe | Anforderung | Registry-Wirkung |
|---|---|---|
| **CONF-BRONZE** | Kategorien 1–6 + 10 grün; Unit-Level | `active` erst ab hier (REQ-CONF-005) |
| **CONF-SILBER** | + 7–9 grün; Interop-Node-Paar simuliert | Release-Empfehlung |
| **CONF-GOLD** | + Cross-Repo-Evidence (zwei Repos implementieren unabhängig) | Referenz-Implementierung |

## §4 Conformance-Registry (SSOT)

`registry/protocol-conformance.yaml` führt je Familie: CONF-Plan-ID + Version,
Spezifikationsversion, Stufe, letzte Ausführung (Datum, Tool-Version,
Testzähler grün/rot), Abdeckung je Kategorie, offene Ausnahmen. Generator-
basiert, keine Hand-Edits (REQ-CONF-006). Validator S-26 prüft: jede `draft`/
`active`-Familie hat einen Registry-Eintrag; `active` ohne CONF-BRONZE = FAIL.

## §5 Ausführungsregeln

- Läufe in CI je Spec-Änderung; Re-Run je Release-Kandidat
- Failing Tests blockieren die zugehörige Spezifikations-Freigabe (BUG-005-Kopplung)
- Ausnahmen (SKIP) nur dokumentiert mit Grund + Frist
- Test-Code lebt im Ziel-Repo neben der Implementierung (ShivaCore-Konvention)

## §6 Interoperabilität über Repos

Für Familien mit mehreren Implementierungen (Kernel + SDK + Explorer) MUSS die
Konformanz aller Instanzen gegen EINEN CONF-Plan erfolgen; Abweichler werden als
COMP-Abweichung nach COMPAT-001 geführt (REQ-CONF-007).

## §7 Kopplung an Protokoll-Lifecycle

REQ-PROTO-021 (Ehrlichkeitsregel) wird geschärft: `active` = verifizierte
Implementierung **+ CONF-BRONZE**; `experimental` = implementiert, CONF-Plan in
Arbeit; `deprecated` behält letzte erreichte Stufe (REQ-CONF-005). Der
Upgrade-Prozess (PROTOCOL-001 §19) führt die Conformance-Evidence als Pflichtfeld.

## Requirements (normativ)

- **REQ-CONF-001** (§1): Versionierter CONF-Plan je Familie, an Spec-Version gebunden.
- **REQ-CONF-002** (§2.2): Determinismus-Tests sind Pflicht und FAIL-scharf.
- **REQ-CONF-003** (§2.6): 100 % Fehlerkatalog-Testabdeckung als Mindestmaß.
- **REQ-CONF-004** (§2.8): Interop-Test mit zwei unabhängigen Implementierungen
  je Familie (ab SILBER).
- **REQ-CONF-005** (§3/§7): `active` nur mit CONF-BRONZE; Stufen wirken in der
  Protokoll-Registry.
- **REQ-CONF-006** (§4): Registry-SSOT, generatorbasiert, Validator S-26.
- **REQ-CONF-007** (§6): Eine Familie, ein CONF-Plan über alle Repos.

## Security Considerations

Test-Suiten sind Angriffsflächen-Replikate: Malformed-Input-Korpusdateien
enthalten bewusst fehlgeformte Nachrichten — als Daten gekennzeichnet, nie im
Produktivdatenbestand. Fuzzing-Korpora werden versioniert, Ausgaben nicht.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.0.0-Approval** (2026-09-08): Owner-§9-Freigabe (Builder-Chat 08.09. 01:41 UTC+2, SCR-0029): DRAFT → APPROVED, normativ in Kraft, §30-eingefroren. Gebündelt genehmigt: FRAMEWORK-001 v1.0.7-PATCH. Registry FINAL: 124 Standards, 124 APPROVED.
- **1.0.0** (2026-09-08): Initial Release DRAFT — 123. Standard; 10 Pflicht-
  Kategorien, CONF-BRONZE/SILBER/GOLD, Conformance-Registry + Validator S-26
  (GEPLANT als automatisierte Prüfung), active-Gate-Verschärfung von
  REQ-PROTO-021. Erste Instanz: CONF-P2P-001 (29 Unit-Tests aus SCR-0028 als
  BRONZE-Basis, Silber/Gold offen). §9-Freigabe ausstehend.

## References

- ATC-STD-PROTOCOL-001 (§19 Lifecycle, §21 REQ-PROTO-021, §22 Tests)
- protocols/p2p/ATC-PROTO-P2P-001 + ShivaCore p2p_secure.rs (Referenzfall SCR-0027/0028)
- ATC-STD-PROTOCOL-003 (Security-Audit-Kopplung), ATC-STD-COMPAT-001 (§6)
- registry/protocol-conformance.yaml (SSOT), registry/protocol-registry.yaml

*ATC-STD-PROTOCOL-002 v1.0.0 · Owner-Entwurf · Aurora (Superagent) · 08.09.2026 · SCR-0029*
