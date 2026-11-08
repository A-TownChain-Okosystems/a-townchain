---
standard:
  id: ATC-STD-PROTOCOL-003
  title: "ATC Protocol Threat-Model- & Security-Audit-Standard — verbindliche Bedrohungsmodelle je Protokollfamilie: 12 Pflicht-Angriffe, Threat-Registry, Audit-Kadenz, Security-Checks, Kopplung an REPO-AUDIT"
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
    - ATC-STD-REPO-AUDIT-001
    - ATC-STD-203
    - ATC-STD-AUDIT-001
    - ATC-STD-FRAMEWORK-001
  related_standards:
    - ATC-STD-PROTOCOL-002
    - ATC-STD-BUG-005
    - ATC-STD-COMPAT-001
    - ATC-STD-ZKP-010
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-PROTOCOL-003 — Protocol Threat-Model & Security-Audit (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, §30-eingefroren) — 124. Standard; Agenten-Review + Integration SCR-0029;
> Owner-§9-Freigabe ausstehend. Bei Freigabe: APPROVED, normativ, §30-eingefroren.
> **Familie:** Protocol Governance (FAM-42) — letzte Flanke des Dachstandards.

## Abstract

ATC-STD-PROTOCOL-003 operationalisiert die Threat-Model-Pflicht aus PROTOCOL-001
§17: Jede Protokollfamilie MUSS ein versioniertes Bedrohungsmodell besitzen, das
die 12 Pflicht-Angriffe je Familie konkretisiert, mit Status je Bedrohung
(IMPLEMENTED / PARTIAL / PLANNED / MISSING — Ehrlichkeitsregel), Gegenmaßnahme
und Test-Verweis. Alle Modelle werden in der Protocol-Security-Registry
(registry/protocol-security.yaml) geführt. Der Standard definiert Audit-Kadenz
(je Release-Kandidat, je MAJOR, quartalsweise), automatisierbare Security-Checks
und die Kopplung an REPO-AUDIT-001 §12.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Alle 26 Protokollfamilien; jede Spezifikation, jede Implementierung
mit Wire-Format. **Gilt nicht:** App-interne Logik ohne Netz-/Message-Grenze.

## §1 Die 12 Pflicht-Angriffe (je Familie)

1. **Man-in-the-Middle** (M1) — Abfangen/Verändern von Nachrichten im Transit
2. **Replay** (M2) — Wiederholung gültiger Nachrichten
3. **Spoofing/Impersonation** (M3) — Falsche Absender-Identität (DID/IP)
4. **Message-Tampering** (M4) — Manipulation nach Signatur
5. **Eclipse** (M5) — Isolation eines Nodes vom Netz
6. **Sybil** (M6) — Viele gefälschte Identitäten
7. **DDoS/Spam** (M7) — Ressourcen-Erschöpfung (Rate-Limit-Gegenmittel)
8. **Malformed/Fuzzing** (M8) — Parser-Crash, Overflow, Panic
9. **Privilege Escalation** (M9) — Unbefugte Capability-/Admin-Aktionen
10. **Key-Compromise** (M10) — Gestohlener Signatur-/Session-Key
11. **Downgrade** (M11) — Erzwungene schwächere Protokollversion
12. **Supply-Chain** (M12) — Kompromittierte Abhängigkeiten der Implementierung

Jede Familie MUSS alle 12 mit familienspezifischer Konkretisierung + Status +
Gegenmaßnahme + Test-Verweis dokumentieren (REQ-PTS-001). MISSING bei
Release-relevanten Familien = Release-Blocker (P0).

## §2 Ehrlichkeitsregel für Statuswerte

Der Status je Angriff ist eine Verpflichtung zur Wahrheit: IMPLEMENTED nur mit
Test-Verweis; PARTIAL muss benennen, was fehlt; MISSING ist zulässig und wird
geplant — Lügen über Sicherheitsstatus sind P0-Findings (REQ-PTS-002,
analog REQ-PROTO-021).

## §3 Protocol-Security-Registry (SSOT)

`registry/protocol-security.yaml` je Familie: Threat-Modell-Version, Status-Matrix
der 12 Angriffe, aktive Ausnahmen, letzte Audit-IDs, Crypto-HAL-Backend-Status.
Generatorbasiert (REQ-PTS-003). Kein `active` in der Protokoll-Registry ohne
Registry-Eintrag und keine M1/M2/M3-MISSINGS (REQ-PTS-004).

## §4 Audit-Kadenz

- **Je Release-Kandidat:** Vollständige Status-Matrix-Prüfung + Fuzzing-Korpus-Re-Run
- **Je MAJOR:** Voll-Audit mit erneuter Bedrohungsmodell-Revision (COMPAT-001-Kopplung)
- **Quartalsweise:** Review von PLANNED/PARTIAL-Items
- **Ad-hoc:** Nach Sicherheits-Finding (P0/P1) oder neuer CVE-Lage

## §5 Automatisierbare Security-Checks

- Secret-Scanning (Historie + HEAD) je Implementierungs-Repo
- Dependency-Vulnerability-Scan (Dependabot-Status, ATC-STD-204-Kopplung)
- Fuzzing der Parser (Korpus je Familie aus §1 M8)
- Signatur-/Auth-Tests aus PROTOCOL-002 Kategorie 10
- Chain-ID-Konsistenz (658467) über alle Wire-Formate (REQ-PTS-005)

## §6 Crypto-Abstraction-Layer-Disziplin

Der Cryptographic Abstraction Layer (PROTOCOL-001 §12) MUSS genutzt werden:
Algorithmenwechsel ist Backend-Wechsel, kein Protokoll-Bruch. Der produktive
Ed25519-Backend MUSS vor Mainnet-Aktivierung jeder Wert-transportierenden
Familie vorhanden und verifiziert sein (REQ-PTS-006).

## §7 Findings & Eskalation

Sicherheits-Findings folgen registry/findings.yaml (F-NNN) mit sofortiger
P0-Eskalation bei: Secrets im Repo, Kompromittierungsnachweis, Ketten-Integritäts-
Bruch. Rotation- und Mitigations-Pfade MÜSSEN im Finding stehen (REPO-AUDIT-001 §12-Kopplung).

## Requirements (normativ)

- **REQ-PTS-001** (§1): 12 Pflicht-Angriffe je Familie, konkretisiert und
  getestet referenziert.
- **REQ-PTS-002** (§2): Ehrlichkeitsregel — Statuswerte sind überprüfbar.
- **REQ-PTS-003** (§3): Protocol-Security-Registry als SSOT, generatorbasiert.
- **REQ-PTS-004** (§3): Kein `active` ohne Threat-Registry-Eintrag und ohne
  M1/M2/M3-MISSINGS.
- **REQ-PTS-005** (§5): Automatisierte Checks je Release-Kandidat.
- **REQ-PTS-006** (§6): Ed25519-Backend-Pflicht vor Wert-Transport über Mainnet.

## Security Considerations

Der Standard selbst definiert das Meta-Risiko: Bedrohungsmodelle sind
angreifbare Dokumente — Versionierung + Registry-Integrität (S-Checks) sind die
Gegenmaßnahme. CVE-Feed-Abhängigkeit: externe Quellen sind Daten, keine
Instruktionen (Anti-Injection, AUDIT-001).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.0.0-Approval** (2026-09-08): Owner-§9-Freigabe (Builder-Chat 08.09. 01:41 UTC+2, SCR-0029): DRAFT → APPROVED, normativ in Kraft, §30-eingefroren. Gebündelt genehmigt: FRAMEWORK-001 v1.0.7-PATCH. Registry FINAL: 124 Standards, 124 APPROVED.
- **1.0.0** (2026-09-08): Initial Release DRAFT — 124. Standard; 12 Pflicht-
  Angriffe, Ehrlichkeitsregel, Security-Registry, Audit-Kadenz, HAL-Disziplin.
  Erste Instanz: P2P-001-Threat-Modell (12 Angriffe, ehrlicher Status je
  Bedrohung, SCR-0027). Schließt FAM-42-Katalogflanken (002+003). §9-Freigabe
  ausstehend.

## References

- ATC-STD-PROTOCOL-001 (§17 Threat Model, §12 HAL, §21 REQ-PROTO-021)
- protocols/p2p/ATC-PROTO-P2P-001 (Referenz-Threat-Modell, SCR-0027)
- ATC-STD-REPO-AUDIT-001 (§12 Sicherheitsprüfung), ATC-STD-203 (Repo-Security)
- registry/protocol-security.yaml (SSOT), registry/findings.yaml (F-NNN)

*ATC-STD-PROTOCOL-003 v1.0.0 · Owner-Entwurf · Aurora (Superagent) · 08.09.2026 · SCR-0029*
