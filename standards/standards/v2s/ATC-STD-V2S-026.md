---
standard:
  id: ATC-STD-V2S-026
  title: "End-of-Life Standard"
  version: "1.0.0"
  status: candidate
  category: v2s
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-10-03"
  updated: "2026-10-03"
  normative: true
  applies_to: "Alle ATC-Projekte gemäß ATC-STD-V2S-000 Scope"
  supersedes: []
  superseded_by: null
  effective_date: "2026-10-03"
  review_date: "2027-10-03"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-V2S-026 — End-of-Life (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (SCR-0042);
> letzte Station des V2S-Lifecycle. Dockt an ATC-STD-V2S-000 an; direkter Upstream
> ist ATC-STD-V2S-025 (Deprecation). Vollendung der Bergung 03.10.2026 (SCR-0041).

## Abstract

ATC-STD-V2S-026 definiert den verbindlichen Abschluss des Software-Lifecycle:
Abschaltung, Daten- und Artefakt-Archivierung sowie die nachvollziehbare
Historisierung. End-of-Life ist kein Fehlerzustand, sondern die geplante,
dokumentierte und reversible Endstation — Software gilt erst mit vollständigem
EOL-Record als ordnungsgemäß beendet.

## Scope

**Gilt für:** alle ATC-Projekte, Komponenten, Dienste und Standards, die ihren
Lifecycle beenden. **Nicht-Gilt:** vorübergehende Stilllegungen (Pause, siehe
ATC-STD-V2S-024 Version Upgrade); vorläufige Deprecation (V2S-025).

## §1 Gegenstand & Verortung

EOL ist die 26. und letzte Station des V2S-Lifecycle (ATC-STD-V2S-000 §1):
IMPROVEMENT → NEXT VERSION → (bei Nicht-Fortführung) DEPRECATION → END-OF-LIFE.
Voraussetzung ist ein abgeschlossener Deprecation-Zyklus (V2S-025) mit
abgelaufener Frist; ein EOL ohne vorherige Deprecation ist VERBOTEN (MUST NOT).

## §2 Kernregeln

1. **KR-1:** EOL MUSS durch einen EOL-Record eingeleitet werden: Begründung, Datum, betroffene Komponenten, Dateninventar, Verantwortlicher, Genehmigung (MUST).
2. **KR-2:** Vor Abschaltung MÜSSEN alle Nutzer und abhängigen Systeme benachrichtigt und Migrationspfade dokumentiert sein (Kopplung V2S-025 §3).
3. **KR-3:** Betriebsdaten MÜSSEN vor Abschaltung vollständig archiviert werden — Chain-Daten in Archive-Nodes/State-Snapshots, Nutzerdaten nach Dateninventar mit Prüfung der DSGVO/BaFin-Anforderungen (ATC-STD-AUDIT-001).
4. **KR-4:** Der letzte Release-Stand MUSS als reproduzierbares Artefakt (Quelle, Build, Checksum, Tag) archiviert werden (MUST — Reversibilität).
5. **KR-5:** Git-Historie, Issues und Findings DÜRFEN NICHT gelöscht werden (MUST NOT) — das Repository wird auf ARCHIVED gesetzt, read-only versiegelt.
6. **KR-6:** Der Lifecycle-State (ATC-STD-V2S-000 §16) MUSS auf maturity M9-Abschluss mit operations: ended und superseded_by (Nachfolger oder explizit null) gesetzt und committet werden (MUST).

## §3 Artefakte & Nachweispflichten

| Artefakt | Pflicht |
|---|---|
| EOL-Record (SCR/Issue mit Begründung + Genehmigung) | MUSS |
| Dateninventar + Archiv-Nachweis (Checksummen) | MUSS |
| Final-Artefakt (Tag/Release, reproduzierbar) | MUSS |
| Nutzer-/Dependenten-Benachrichtigungs-Log | MUSS |
| Abschaltungs-Log (Zeitstempel, Verifier) | MUSS |
| Nachlauf-Monitoring (Rest-Traffic, 30 Tage) | SOLLTE |

## §4 Lifecycle-State-Bezug

Der maschinenlesbare State setzt: `operations: ended`, `monitoring: off`,
Abschluss-Eintrag in `gate_records` (V2S-G0..G9 abgeschlossen bzw. dokumentiert
übersprungen). Damit ist der Projekt-State maschinell als beendet auswertbar.

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-025 (Deprecation — EOL nur nach Fristablauf)
- **Querschnitt:** ATC-STD-VERSION-001 (finale Version), ATC-STD-UPDATE-001
  (Emergency-Decommissioning als Sonderpfad), ATC-STD-AUDIT-001 (Archiv-Audit)
- **Datenhaltung:** ATC-STD-NET-001..008 (Archive-Nodes je Netzwerkstufe),
  Chain-ID 658467 State-Snapshots
- **Governance:** SCR-Prozess (ATC-STD-000 §13), Findings (F-NNN) bis RESOLVED

## Metriken & Akzeptanzkriterien

- **M1 (Datenverlust):** 0 nicht archivierte Datenobjekte laut Inventar (Nachweis: Checksummen-Abgleich)
- **M2 (Benachrichtigung):** 100 % der registrierten Dependenten haben Bestätigungs-Log-Einträge
- **M3 (Reversibilität):** Final-Artefakt aus dem Archiv baut reproduzierbar (2/2 identische Checksummen)
- **M4 (State-Konsistenz):** Lifecycle-State schema-konform (schemas/software-lifecycle.schema.json), operations: ended

## Compliance & Verifikation

Geprüft per atc-repo-audit (Archiv-Status des Repositories), Lifecycle-State-
Schema-Validierung und EOL-Record-Review (S-01..S-25). Ein EOL ohne vollständigen
Record oder mit offenen P0/P1-Findings ist ungültig und erzeugt ein Finding
(F-NNN) nach ATC-STD-BUG-001.

## Security Considerations

Abgeschaltete Systeme ohne Versiegelung sind Angriffsflächen (vergessene
Endpunkte, ungepatchte Abhängigkeiten). Der read-only-Siegel-Zustand MUSS
per Branch-Protection erzwungen werden; Archive-Zugänge MÜSSEN minimal-privilegiert
bleiben. Vernichtete Zugangsdaten (Keys, Secrets) sind im EOL-Record zu
dokumentieren.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-10-03 | Initiale Fassung — SCR-0042; Vollendung der V2S-Phasen-Bergung (SCR-0041) |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-V2S-025, ATC-STD-VERSION-001, ATC-STD-000, schemas/software-lifecycle.schema.json
**INFORMATIVE:** SCR-0041, SCR-0042
