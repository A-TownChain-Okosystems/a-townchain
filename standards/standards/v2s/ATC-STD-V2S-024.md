---
standard:
  id: ATC-STD-V2S-024
  title: "Version Upgrade Standard"
  version: "1.0.0"
  status: candidate
  category: v2s
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Projekte gemäß ATC-STD-V2S-000 Scope"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-V2S-024 — Version Upgrade Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-024 regelt den geordneten Übergang auf eine neue Software- oder Protokollversion
(Version Upgrade) im ATC Vision-to-Software Lifecycle. Der Standard verbindet das Semantic
Versioning (ATC-STD-VERSION-001), die Aktualisierungsklassen (ATC-STD-UPDATE-001) und dezentrale
Netzwerk-Upgrades (ATC-STD-NET-006) zu einem verbindlichen Übergangsmodell.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** Alle Versionssprünge (Major, Minor, Patch) und Netzwerk-Upgrades (Hard Fork,
Soft Fork) von ATC-Anwendungen, Protokollen, Smart Contracts und KI-Agenten.
**Nicht-Gilt:** Reine Konfigurationsänderungen ohne Code- oder Schemaänderungen.

## §1 Gegenstand & Verortung

Version Upgrade bildet Station 19 im V2S-Lifecycle (ATC-STD-V2S-000 §1). Es übernimmt die aus
V2S-023 (Improvement Standard) definierten Anforderungen und führt das Gesamtsystem geordnet
in die nächste Entwicklungsiteration über.

Der Standard koppelt drei fundamentale Governance-Standards:
1. **ATC-STD-VERSION-001:** Regelwerk für Semantic Versioning (`MAJOR.MINOR.PATCH`).
2. **ATC-STD-UPDATE-001:** Prozess für Update-Klassifizierung und Notfall-Fixes.
3. **ATC-STD-NET-006:** Koordinationsmodell für dezentrale Netzwerk- und Konsens-Upgrades.

## §2 Kernregeln

1. **KR-1 (SemVer-Einhaltung nach ATC-STD-VERSION-001):** Jedes Version Upgrade MUSS strikt nach Semantic Versioning eingestuft werden (MAJOR bei inkompatiblen API-Änderungen, MINOR bei abwärtskompatibler Funktionalität, PATCH bei abwärtskompatiblen Bugfixes).
2. **KR-2 (Update-Klassifizierung nach ATC-STD-UPDATE-001):** Die Aktualisierung MUSS vor Durchsetzung gemäß ATC-STD-UPDATE-001 klassifiziert und freigegeben werden. Notfall-Patches MÜSSEN den Emergency-Update-Pfad nutzen.
3. **KR-3 (Netzwerk-Upgrades nach ATC-STD-NET-006):** Upgrades von dezentralen Protokoll-Knoten und Smart Contracts MÜSSEN den Signalisierungs-, Voting- und Aktivierungszeitplan nach ATC-STD-NET-006 einhalten.
4. **KR-4 (Migrationspfade & Abwärtskompatibilität):** Für Major-Upgrades MÜSSEN automatisierte Migrationsskripte (Datenbank, State, Konfiguration) bereitgestellt und nach ATC-STD-COMPAT-001 auf Abwärtskompatibilität getestet werden.
5. **KR-5 (Neustart des V2S-Prozessdurchlaufs):** Ein Major- oder Minor-Upgrade MUSS den V2S-Lifecycle für alle geänderten oder neuen Komponenten ab den Gates V2S-G0/G1 erneut vollständig durchlaufen.
6. **KR-6 (Lifecycle-State Fortschreibung):** Das Upgrade MUSS den maschinenlesbaren Lifecycle-State der neuen Zielversion initialisieren und die Phasen-Zustände für die Entwicklungs-Pipeline kontrolliert zurücksetzen.

## §3 Artefakte & Nachweispflichten

- **Version Upgrade Plan:** Detaillierter Ablaufplan inklusive Zeitstempeln, Kompatibilitätsmatrix und Rollback-Kriterien.
- **Migration Guide & Scripts:** Dokumentierte und automatisierte Migrationsanweisungen für Operatoren und Nutzer.
- **Network Upgrade Schedule:** Aktivierungsplan nach ATC-STD-NET-006 mit Signal-Perioden und Konsens-Schwellenwerten.
- **Updated Lifecycle-State:** Aktualisierter State-Eintrag für die neue Versionsnummer.

## §4 Lifecycle-State-Bezug

Der Standard pflegt und aktualisiert den maschinenlesbaren Lifecycle-State
(`.atc/lifecycle-state.yaml`) gemäß ATC-STD-V2S-000 §16:

- Sets `lifecycle.active_version`: Neue Versionsnummer nach erfolgreichem Upgrade
- Sets `lifecycle.target_version`: Ziel-Version während der Vorbereitung
- Sets `lifecycle.upgrade_status`: `planned` | `in_progress` | `completed` | `rolled_back`

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-023 (Improvement Standard).
- **Downstream:** ATC-STD-V2S-001 bis V2S-018 (Lifecycle-Durchlauf für das neue Release).
- **Andock-Standards:** ATC-STD-VERSION-001 (SemVer), ATC-STD-UPDATE-001 (Update Management), ATC-STD-NET-006 (Network Upgrades), ATC-STD-COMPAT-001 (Compatibility).

## Metriken & Akzeptanzkriterien

- **M1 (SemVer Compliance):** 100 % aller Releasetags folgen exakt dem Format `MAJOR.MINOR.PATCH` gemäß ATC-STD-VERSION-001.
- **M2 (Migration Script Reliability):** 100 % der Test-Datenbank- und State-Migrationen schließen fehlerfrei ab.
- **M3 (Network Upgrade Activation Rate):** ≥ 99 % aller Protokoll-Knoten vollziehen das Netzwerk-Upgrade ohne Chain-Split.
- **M4 (Traceability):** 100 % der Upgrade-Commits verweisen auf die freigegebenen Change Requests.

## Compliance & Verifikation

Die Einhaltung wird durch automatisierte SemVer-Checks, CI/CD-Migrations-Runner,
atc-repo-audit und Netzwerk-Konsens-Monitore verifiziert (S-01..S-25). Inkompatible Sprünge
ohne Major-Bump oder fehlende Migrationsskripte erzeugen Findings (F-001..F-999) nach ATC-STD-BUG-001.

## Security Considerations

Netzwerk- und Protokoll-Upgrades bergen das Risiko von Replay-Attacken, Network Splits
und Zustandskorruption. Alle Upgrade-Skripte und Smart-Contract-Migrationen MÜSSEN
vor der Ausführung einem formale Audit und Testnet-Dry-Run unterzogen werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-VERSION-001, ATC-STD-UPDATE-001, ATC-STD-NET-006, ATC-STD-COMPAT-001
**INFORMATIVE:** SCR-0042
