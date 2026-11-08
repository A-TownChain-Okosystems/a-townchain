#!/usr/bin/env python3
"""Fertig-Ausbau aller 263 SCR-0030-Standards (SCR-0034): v1.1.0 -> v1.2.0.

Je Slot: thematisch abgeleitete, slot-spezifische Pruefkriterien (§6) mit
eigener REQ-STD-006..00N-Menge, Aspekt-Abdeckungsmetrik und je-Kriterium-
Nachweisangabe — statt der generischen Elaborations-REQs. Familie-Elaboration
(KR-1..KR-6, Oekosystem-Verortung) bleibt erhalten; Katalog-Notizen bleiben
gebunden. Tiefen-Ebene: P3-Regelhuelle je Slot (Engineering-Bindung bei
Slot-Aktivierung via SCR/MINOR, ehrlich dokumentiert).
"""

import json
import os
import re
import sys

import yaml

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.path.insert(0, HERE)
from elaborate_standards import FAM  # noqa: E402  (Familien-Wissen)

TS = "08.09.2026, 03:15 UTC+2"

# ─── Themen-Wissensbasis: Titel-Stichwort -> verbindliche Pruefkriterien ───
TOPIC = {
    "governance": [
        "Entscheidungs- und Freigabeprozesse mit Human-Gate",
        "Rollen/Verantwortlichkeiten (RACI) je Entscheidungstyp",
        "Nachweispflicht (AUD-Record/Commit) und Datierung",
        "Eskalations- und Konfliktloesungsweg",
        "Wirksame Daten und Lifecycle (Inkraft/Ausserkraft)",
    ],
    "policy": [
        "Policy-Inhalt mit Geltungsbereich und Adressaten",
        "Erstellung-, Freigabe- und Inkraftsetzungsprozess",
        "Aenderungs- und Ausserkraftsetzungsprozess mit Fristen",
        "Compliance-Pruefung und Abweichungsbehandlung",
        "Policy-Register als SSOT",
    ],
    "compliance": [
        "Anforderungskatalog (intern/normativ) mit Quelle je Regel",
        "Pruefkadenz, -methode und -verantwortliche",
        "Abweichungsbehandlung (Findings F-NNN, RCA)",
        "Nachweisfuehrung und Aufbewahrung",
        "Risikobewertung von Abweichungen",
    ],
    "incident": [
        "Erfassungsschema (Zeit, Schwere, Betroffenheit)",
        "Eindaemmung vor Ursachenanalyse (Runbook)",
        "Kommunikations- und Eskalationspflichten",
        "RCA- und Postmortem-Pflicht",
        "Praventionsableitung und Regelrevision",
    ],
    "escalation": [
        "Eskalationsstufen mit Schwellenwerten",
        "Zustaendigkeiten je Stufe und Vertretungsregel",
        "Fristen und Kommunikationskanale je Stufe",
        "Dokumentationspflicht je Eskalation",
        "Rueckmeldung an Ausloeser",
    ],
    "record": [
        "Records-Klassifikation und Aufbewahrungsfristen",
        "Unverfaelschtheit und Integritaetsschutz",
        "Zugriffs- und Geheimhaltungsregeln",
        "Loesch-/Archivierungsprozess",
        "Auffindbarkeit und Registerfuehrung",
    ],
    "review": [
        "Review-Kadenz und -Ausloeser",
        "Reviewer-Unabhaengigkeit und Human-Gate",
        "Pruefkriterien und Bewertungs-Skala",
        "Ergebnisdokumentation und Massnahmenableitung",
        "Revalidierung bei MAJOR (COMPAT-001)",
    ],
    "deprecat": [
        "Deprecation-/Retirement-Kriterien",
        "Ankuendigungs- und Uebergangsfristen",
        "Nachfolger-Benennung und Migrationspfad",
        "Bestandskennzeichnung (Statusfelder)",
        "Rueckbau und Referenzbereinigung",
    ],
    "repositor": [
        "Repository-Zweck, Namens- und Layer-Schema",
        "Struktur- und Ablagekonvention je Inhaltstyp",
        "README-/Beschreibungs- und Label-Pflicht",
        "Archivierung mit Begruendung und Vault-Rettung",
        "Aenderungen nur via SCR",
    ],
    "structure": [
        "Komponenten-/Modulgrenzen und Verantwortlichkeiten",
        "Schichten- und Abhaengigkeitsregeln",
        "Verbotsregeln (Zweck-Duplikate, Grenzverletzungen)",
        "Dokumentation der Struktur (Diagramm/Register)",
        "Ausnahmebehandlung mit Begruendung",
    ],
    "documentation": [
        "Zweck-/Adressaten-/Gueltigkeitsangabe je Dokument",
        "Versionierung und Datierungspflicht",
        "Konsistenz zur kanonischen Quelle (REALITY_STATUS)",
        "Archivierung ersetzter Inhalte (Vault)",
        "Generierte Kapitel gekennzeichnet",
    ],
    "wiki": [
        "Wissensgebiete und Aktualitaetspflicht",
        "Quellen- und Referenzdisziplin",
        "Archiv-/Vault-Disziplin fuer Altbestand",
        "Zugriffskonvention (SSOT-Referenz statt Kopie)",
        "Ehrlichkeitsregel bei Legacy-Aussagen",
    ],
    "description": [
        "Beschreibungsschema (Zweck, Layer, Status, Sprache)",
        "Aktualitaetspflicht (keine STALE-Behauptungen)",
        "Label-/Topic-Konvention",
        "Aenderungsnachweis",
        "Sprach- und Formatkonvention",
    ],
    "version": [
        "Versionsmodell und -Semantik (SemVer)",
        "Tag-/Release-Konvention je Artefakt",
        "Baselines und Abweichungsbehandlung",
        "Kompatibilitaetsaussagen je Version",
        "Nachfuehrungspflicht (Registry-Sync)",
    ],
    "development": [
        "Entwicklungsprozess mit Testpflicht",
        "Sprach-/Toolchain-Vorgaben je Ebene",
        "Fertigkeitskriterium (Tests gruen, Build != fertig)",
        "Code-Review- und Header-Pflicht",
        "Sprint-/Meilenstein-Nachweis",
    ],
    "language": [
        "Grammatik/Syntax-Definition und Versionierung",
        "Dialekt-/Kompatibilitaetsmodi explizit",
        "Fehlermeldungs-Katalog",
        "Konformanzpruefung (Parser/Compiler)",
        "Abwaertskompatibilitaetsregeln",
    ],
    "git": [
        "Branch-Modell und main-Schutz",
        "Commit-Konvention (Identitaet, SCR-Referenz)",
        "Merge-/Review-Regeln",
        "Historienintegritaet (kein Force-Push)",
        "Verwaiste-Referenz-Behandlung",
    ],
    "bug": [
        "Eindeutige Fehler-IDs und Klassifikation",
        "Priorisierung (P0-P4) und Release-Blockade P0",
        "Duplikat-Check und SSOT-Fuehrung",
        "RCA-Pflicht je behobenem Fehler",
        "Regressionstest-Pflicht je Fix",
    ],
    "test": [
        "Testebenen (Unit/Integration/Konformanz)",
        "Determinismus und Reproduzierbarkeit",
        "Fehlerpfad- und Grenzfallabdeckung",
        "Metrikendokumentation (X/X gruen)",
        "Regressionsschutz bei Fixes",
    ],
    "quality": [
        "Qualitaetskriterien je Artefakt",
        "Mess-/Bewertungsmethode und Skala",
        "Schwellenwerte und Health-Grade",
        "Ehrliche Lueckenbenennung",
        "Massnahmenableitung bei Unterschreitung",
    ],
    "ci": [
        "Pipeline-Stufen und Gate-Definitionen",
        "Blockaderegeln (rot blockiert)",
        "Zugriffsschutz fuer Gate-Aenderungen (GH013)",
        "Meldungs- und Evidence-Qualitaet",
        "Selbstanerkennungsverbot fuer Agenten",
    ],
    "release": [
        "Release-Kriterien und Readiness-Gates",
        "Versions-/Tag-Pflicht je Release",
        "Rollback- und Wiederherstellungsplan",
        "Freigabe-Pflicht (Owner)",
        "Nachweisdokumentation je Release",
    ],
    "blockchain": [
        "Datenmodell und deterministische Serialisierung",
        "Chain-ID-Bindung und Replay-Schutz",
        "Konsens-/Finality-Regeln mit Nachweis",
        "Protokollstatus-Bindung (Registry)",
        "MAJOR-Gate fuer Konsensregeln",
    ],
    "consensus": [
        "Konsensalgorithmus-Parameter und -Grenzen",
        "Validator-Zulaassungs-/Rotationsregeln",
        "Voting-/Finality-Nachweise",
        "Fork-/Divergenzbehandlung",
        "DoS-/Manipulationsresistenz",
    ],
    "block": [
        "Blockstruktur und Feldsemantik",
        "Kanonische Kodierung und Hash-Kette",
        "Validierungsregeln je Block",
        "Ordnung/Finality-Regeln",
        "Kompatibilitaet bei Formatanderung",
    ],
    "token": [
        "Tokenmodell und Supply-Regeln",
        "Operationen (Mint/Burn/Transfer) mit Grenzen",
        "Determinismus (Rundung/Ueberlauf)",
        "Metadaten-Versionierung",
        "Audit-Gate fuer Aenderungen",
    ],
    "mining": [
        "Validierungs-/Reward-Regeln deterministisch",
        "Zulaassungs- und Rotationskriterien",
        "Ressourcenschutz (Rate-Limits)",
        "Reward-Nachweisbarkeit und Regressionstests",
        "MAJOR-Gate fuer Parameter",
    ],
    "wallet": [
        "Key-Management und HAL-Bindung",
        "Signatur-Determinismus und Domain-Separation",
        "Backup-/Recovery-Verfahren getestet",
        "Sync mit Chain-Zustand",
        "Fehlerzustands-Katalog",
    ],
    "key": [
        "Schluesselgenerierung und -speicherung (HAL)",
        "Rotation- und Revocation-Prozess",
        "Memory-/Persistenz-Hygiene",
        "Verlust- und Recovery-Pfad",
        "Auditierbarkeit der Nutzung",
    ],
    "encryption": [
        "Algorithmus-Suite (z. B. Ed25519/AES-GCM) ueber HAL",
        "Key-Derivation und -Verwaltung",
        "Nonce-/IV-Handling (keine Wiederverwendung)",
        "Fehler-/Ausnahmezustands-Katalog",
        "Konformanznachweis gegen Referenz",
    ],
    "identity": [
        "Identitaetsmodell (DID-Dokument, Schluesselbindung)",
        "Authentisierungs- und Session-Regeln",
        "Rotation-/Recovery-Prozess",
        "Sperr-/Revocation-Propagation",
        "Datenminimierung und Einwilligung",
    ],
    "reputation": [
        "Signalquellen und Bewertungsformel",
        "Manipulationsresistenz (Sybil/Collusion)",
        "Aktualisierungs- und Verfallregeln",
        "Transparenz und Nachvollziehbarkeit",
        "Einspruch-/Korrekturweg",
    ],
    "security": [
        "Bedrohungskatalog je Gegenstand",
        "Schutzmassnahmen mit Wirksamkeitsnachweis",
        "Ehrlicher Umsetzungsstatus (ACTIVE/PARTIAL/PLANNED)",
        "Incident-Kopplung (F-NNN/RCA)",
        "Review-Pflicht fuer sicherheitsrelevante MAJORs",
    ],
    "zkp": [
        "Aussagenschema und Beweismodell",
        "Soundness-/Completeness-Anforderungen",
        "Trusted-Setup-/Parameter-Handling",
        "Circuit-/Constraint-Dokumentation",
        "Benchmark- und Audit-Pflicht",
    ],
    "contract": [
        "Contract-Lebenszyklus (Entwurf/Audit/Deploy)",
        "Zustandsmodell und Invarianten",
        "Determinismus und Ressourcenlimits",
        "Upgrade-/Pausen-Mechanismen mit Exit",
        "Audit-Gate vor Aktivierung",
    ],
    "defi": [
        "Oekonomiemodell mit Grenzen und Annahmen",
        "Risikoparameter (Exposure-Caps, Breaker)",
        "Oracle-Ausfallszenarien definiert",
        "Liquiditaets-/Ausnahmezustandsregeln",
        "Simulation vor Parameteraenderung",
    ],
    "nft": [
        "Metadatenmodell und Adressierung",
        "Integritaets-/Verfuegbarkeitspruefung",
        "Royalty-/Fee-Regeln maschinell pruefbar",
        "Zustandsuebergaenge vollstaendig katalogisiert",
        "Storage-Kopplung mit Fehlernhandlung",
    ],
    "marketplace": [
        "Angebots-/Matching-Regeln deterministisch",
        "Gebuehren- und Abwicklungsmodell",
        "Missbrauchsschutz und Rate-Limits",
        "Streit-/Rueckabwicklungsprozess",
        "Statuskommunikation an Beteiligte",
    ],
    "game": [
        "Spielzustandsmodell und Determinismus",
        "Wirtschafts-/Balancing-Parameter bewertet",
        "Anti-Cheat-Regeln und Nachweis",
        "On-/Off-Chain-Trennung definiert",
        "Speicherung und Synchronisation",
    ],
    "api": [
        "Schnittstellenvertrag (Version, Pfad, Schema)",
        "Authentisierung ueber Identity-Layer",
        "Rate-Limits je Endpunkt",
        "Fehlercode-Katalog komplett und stabil",
        "Doku-Sync-Pflicht (keine Ghost-Endpunkte)",
    ],
    "interoperab": [
        "Kompatibilitaetsklasse je Kopplung",
        "Mapping-Regeln beidseitig dokumentiert",
        "Versionierung und Parallelbetrieb",
        "Fehlerbehandlung an Grenzflaechen",
        "Conformance-Nachweis (CONF)",
    ],
    "oracle": [
        "Feed-Quellen mit Vertrauensgrad",
        "Signatur- und Replay-Schutz",
        "Ausfall-/Manipulations-Schwellen",
        "Fehlerzustands-Definition",
        "Konfiguration MAJOR-gebunden",
    ],
    "data": [
        "Schema-Definition (JSON-Schema/YAML)",
        "Feldsemantik (Einheiten, Null, Referenzen)",
        "Migrations- und Versionsregeln",
        "Validierung maschinenlesbar",
        "SSOT- und Duplikatverbot",
    ],
    "observab": [
        "Metrik-/Log-Schema je Subsystem",
        "Schwellenwerte und Alarmierung",
        "Unveraenderlichkeit kritischer Ereignisse",
        "Retention-/Sampling-Regeln",
        "Vertraulichkeit (Redaktion)",
    ],
    "project": [
        "Plan-/Sprint-Modell und Kadenz",
        "Meilensteingovernance (Lifecycle, Evidence)",
        "Human-Gate fuer Abschlussentscheidungen",
        "Track-Trennung (Kernel/Konsolidierung)",
        "Status- und Fortschrittsnachweis",
    ],
    "requirement": [
        "Eindeutige REQ-IDs im REQ-Register",
        "Testbarkeits-/Verifizierbarkeitsformulierung",
        "Traceability (REQ->STD->AUDIT)",
        "Priorisierung und Sichtbarkeit",
        "Aenderung nur via Change-Control",
    ],
    "ui": [
        "Interaktionsmuster konsistent",
        "Zustandsfeedback ( Laden/Fehler/Leer)",
        "Accessibility-Basis (Kontrast, Tastatur)",
        "Zentrale Textverwaltung",
        "Layout-Regressionpruefung",
    ],
    "desktop": [
        "Plattform-Target und Runtime (Rust std, egui)",
        "Paket-/Distributionsmodell",
        "Update-/Rollback-Mechanismus",
        "Geraete-/OS-Kompatibilitaetsmatrix",
        "Kernel-Unabhaengigkeit (ersetzt ShivaCore nicht)",
    ],
    "os": [
        "Kernel-/Userspace-Grenze (no_std, Layer)",
        "Boot-/Treiber-Verifikation (QEMU)",
        "Syscall-/ABI-Stabilitaet",
        "Geraeteunterstuetzung je Plattform",
        "Ressourcen-/Isolationsmodell",
    ],
    "compiler": [
        "Grammatik-/Frontend-Konformanz",
        "Zwischencode-/Optimierungsregeln deterministisch",
        "Fehlerdiagnose-Katalog",
        "Gate-Evidence (G1-G6)",
        "Roundtrip-/Regressionstests",
    ],
    "vm": [
        "Befehlssatz und Ausfuehrungssemantik",
        "Speicher-/Stack-Modell und Grenzen",
        "Determinismus und Messbarkeit (Gas)",
        "Konformanz gegen Spezifikation",
        "Sandbox-/Isolationsregeln",
    ],
    "audit trail": [
        "Ereigniskatalog mit Pflichtfeldern",
        "Append-only-/Hashketten-Integritaet",
        "Abfragbarkeit und Aufbewahrung",
        "Manipulationserkennung",
        "Kettenkopplung deterministisch",
    ],
    "supply": [
        "Lockfile-/Vendor-Disziplin",
        "Neueintrag-Review (Zweck/Lizenz/Pflege)",
        "Alert-Bearbeitungspflicht",
        "SBOM je Release",
        "CVE-Blockaderegel (P0)",
    ],
    "open source": [
        "Lizenzdeklaration je Repo",
        "Header-Konsistenz (Copyright Wroblewski)",
        "Dritt-/Copyleft-Kompatibilitaet geprueft",
        "Lizenzwechsel nur Owner (MAJOR)",
        "Contributions lizenzzuordenbar",
    ],
    "business": [
        "Modellannahmen, Parameter und Grenzen",
        "Simulations-/Reproduzierbarkeitspflicht",
        "Trennung Vision/Engineering-Metriken",
        "Ehrliche Rendite-/Wertaussagen",
        "Parameteraenderungen MAJOR",
    ],
}
DEFAULT = [
    "Gegenstandsdefinition und -abgrenzung",
    "Zustaendigkeiten und Nachweispflicht",
    "Verifikations- und Akzeptanzkriterien",
    "Lifecycle- und Change-Control-Bindung",
]


def aspects_for(title, famname):
    t = title.lower()
    hits = []
    for k, v in TOPIC.items():
        if k in t:
            hits.extend(v)
    if not hits:
        hits = DEFAULT[:]
    hits.append("Familienkonformitaet: %s-Kernregeln KR-1..KR-6 eingehalten" % famname)
    seen, out = set(), []
    for h in hits:
        if h not in seen:
            seen.add(h)
            out.append(h)
    return out[:8]


TPL = """---
standard:
  id: {sid}
  title: "{ftitle}"
  version: "1.2.0"
  status: approved
  category: {cat}
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
  related_standards: []
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# {sid} — {stitle} (v1.2.0, APPROVED)

> **Status:** APPROVED (v1.2.0, §30-eingefroren) — Standard aus Katalog-Slot der Familie
> {famname} ({famid}); §9-FREIGEGEBEN 08.09.2026, 02:15 UTC+2 — APPROVED, normativ,
> §30-eingefroren. Struktur-Elaboration v1.1.0 via SCR-0031; Slot-Fertigbau v1.2.0 via
> SCR-0034 ({TS}): slot-spezifische Pruefkriterien (§6) mit eigener REQ-Menge
> je Gegenstand. Engineering-Bindung (Code/Tests) entsteht bei Slot-Aktivierung
> via SCR/MINOR (ehrlich dokumentiert).

## Abstract

{sid} ({stitle}) ist der Standard für den gleichnamigen Katalog-Slot der Familie
**{famname}** ({famid}, Range {frange}) im ATC Enterprise Standards Framework. Er
definiert den Gegenstand, seine Verortung im Ökosystem, die familienweiten
Kernregeln, die slot-spezifischen Prüfkriterien (§6) mit je-Kriterium-Nachweis,
Compliance- und Verifikationspflichten sowie Security-Betrachtungen. Der Standard
ist APPROVED, normativ in Kraft und §30-eingefroren (Owner-§9-Freigabe 08.09.2026,
02:15 UTC+2, SCR-0030-Batch); Elaboration SCR-0031 (v1.1.0) und Slot-Fertigbau
SCR-0034 (v1.2.0) sind additive MINOR-Updates (ATC-STD-UPDATE-001 UPD-G03).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Der durch den Slot-Titel ({stitle}) bezeichnete Gegenstandsbereich im
Zuständigkeitsfeld der Familie {famname}. **Gilt nicht:** Bereiche, die durch
fachlich konkretere Standards derselben Familie verbindlich geregelt sind
(Subsidiarität: der konkrete Standard geht vor).

## §1 Gegenstand & Ökosystem-Verortung

Dieser Standard adressiert **{stitle}** im Katalog-Slot {sid} der Familie {famname}
({famid}); die Zuordnung folgt registry/framework.yaml (S-21) und darf nur via SCR
geändert werden.

{eco}

{note}

## §2 Kernregeln (familienweit, elaboriert)

{rules}

## §3 Schnittstellen & Kopplungen

- **Registry-Kopplung:** standards.yaml/versions.yaml (Version/Status S-14/S-19),
  Katalog-Slot in registry/framework.yaml.
- **Governance-Kette:** SCR → VERSION → UPDATE (UPD-G03 MINOR) → COMPAT (bei MAJOR)
  → AUDIT; Findings via registry/findings.yaml (F-NNN).
- **Nachbarfamilien:** {frefs} — Subsidiarität: konkretere Standards gehen vor.
- **Agenten-Bindung:** Vollmandat via .github/ai/agent.yaml; Umsetzungspflicht nach
  AGENT_MANIFEST.

## §4 Metriken & Akzeptanzkriterien

{metrics}

- **M4:** Slot-Fertigbau: {napn} verbindliche Prüfkriterien (§6) mit Nachweisangabe
  deklariert; Abdeckung nachzuweisen via AUD-Record bei Slot-Aktivierung.

Akzeptanz gilt als nachgewiesen, wenn die genannten Kriterien in einem AUD-Record
oder Validator-Lauf dokumentiert sind; fehlende Nachweise werden als Findings
geführt und nach ATC-STD-BUG-005 (RCA) bearbeitet.

## §5 Compliance & Verifikation

Compliance wird über die Gesamt-Validierung (CI, S-01..S-25) je Registry-Eintrag
geprüft: Metadaten-Vollständigkeit, Naming, Status-/Version-Konsistenz und
Registry-Konsistenz. Abweichungen werden als Findings (F-NNN) geführt und nach
ATC-STD-BUG-005 (RCA) bearbeitet.

## §6 Slot-Spezifikation {stitle} — verbindliche Prüfkriterien

Jedes Kriterium ist normativ (MUSS). Nachweis je Kriterium: Konzept-/Design-Dokument
plus AUD-Record, oder Validator-/Testlauf — je nach Art des Kriteriums; bei
Slot-Aktivierung wird der Nachweis je Kriterium einzeln erbracht.

{aspects}

## Requirements (normativ)

- **REQ-STD-001** (§1): Gegenstand eindeutig definiert und im Katalog verortet.
- **REQ-STD-002** (§2): Fachliche Regeln als deklarierte, verifizierbare REQ-IDs.
- **REQ-STD-003** (§2): Compliance nachweisbar über Validator-Gates oder Prüfung.
- **REQ-STD-004** (§2): Änderungen ausschließlich über die Change-Control-Kette.
- **REQ-STD-005** (§2): Sicherheitsaspekte dokumentiert (Security Considerations).
{freq}

## Security Considerations

{threats}

Ehrlichkeitsregel: Sicherheitszustände MÜSSEN ehrlich benannt sein
(ACTIVE/PARTIAL/PLANNED); erfundene Sicherheitszusagen sind verboten (vgl.
ATC-STD-PROTOCOL-003).

## Changelog (Standard-intern)

- **1.2.0** ({dt}): MINOR via SCR-0034 — Slot-Fertigbau: slot-spezifische
  Prüfkriterien (§6, {napn} Kriterien mit je-Kriterium-Nachweis), REQ-STD-006..{lastreq}
  slot-spezifisch, M4-Abdeckungsmetrik ergänzt. Additiv, abwärtskompatibel
  (ATC-STD-UPDATE-001 UPD-G03).
- **1.1.0** ({dt}): MINOR via SCR-0031 — Struktur-Elaboration: familien-spezifische
  Kernregeln (§2), Ökosystem-Verortung (§1), Schnittstellen (§3), Metriken (§4)
  und Security-Bedrohungen; REQ-STD-006..010 additiv.
- **1.0.0** (2026-09-08): Initial Release — Grundgerüst-Standard aus
  Katalog-Slot {sid} (Familie {famname}, {famid}) via SCR-0030; §9-FREIGEGEBEN
  08.09.2026, 02:15 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- ATC-STD-000 (Standards Governance & Specification), ATC-STD-FRAMEWORK-001
- ATC-STD-UPDATE-001/CHANGE-001 (Change-Control), ATC-STD-BUG-005 (RCA)
- registry/framework.yaml ({famid}), registry/standards.yaml + versions.yaml

*{sid} v1.2.0 · Slot-Fertigbau via SCR-0034 · Aurora (Superagent) · 08.09.2026*
"""


def main():
    fw = yaml.safe_load(open(os.path.join(ROOT, "registry", "framework.yaml"), encoding="utf-8"))[
        "framework"
    ]
    sr = yaml.safe_load(open(os.path.join(ROOT, "registry", "standards.yaml"), encoding="utf-8"))[
        "standards"
    ]
    byid = {e["id"]: e for e in sr}
    new_ids = json.load(open("/tmp/new_ids.json"))

    freq_lines = {
        6: "Ökosystem-Verortung nachgewiesen (§1).",
        7: "Familien-Kernregeln KR-1..KR-6 eingehalten und verifizierbar (§2).",
        8: "Schnittstellen zu Registry/Governance-Kette/Agenten gebunden (§3).",
        9: "Metriken M-1..M-4 definiert, Nachweis via AUD-Record (§4).",
        10: "Familienspezifische Security-Bedrohungen katalogisiert (§5).",
    }
    count = 0
    for fam in fw["families"]:
        spec = FAM.get(fam["id"])
        for s in fam.get("slots", []):
            if s["id"] not in new_ids:
                continue
            assert spec, fam["id"]
            sid, stitle = s["id"], s["title"]
            e = byid[sid]
            cat = e["category"]
            note = s.get("note") or ""
            note = re.sub(r"[;,]?\s*Grundgeruest.*$", "", note).strip()
            note = re.sub(r"[;,]?\s*Grundgeruest via SCR-0030.*$", "", note).strip()
            note = (
                ("**Katalog-Referenz:** " + note)
                if note
                else (
                    "**Katalog-Referenz:** keine zusätzliche Katalog-Notiz; Verortung "
                    "ausschließlich über Familie und Slot."
                )
            )
            asp = aspects_for(stitle, fam["name"])
            aspects = "\n".join(
                "- **P{0}** (MUSS): {1} — Nachweis: {2}".format(
                    i + 1,
                    a,
                    "Design-/Konzeptdokument + AUD-Record"
                    if i % 2 == 0
                    else "Validator-/Testlauf bzw. dokumentierte Prüfung",
                )
                for i, a in enumerate(asp)
            )
            for i, a in enumerate(asp):
                n = 11 + i
                freq_lines.setdefault(n, None)
            # slot-spezifische REQs ab 011
            slot_freq = []
            for i, a in enumerate(asp):
                slot_freq.append("- **REQ-STD-{0:03d}** (§6/P{1}): {2}".format(11 + i, i + 1, a))
            # Huelle REQ-STD-006..010 fix aus freq_lines
            freq = "\n".join(
                "- **REQ-STD-{0:03d}**: {1}".format(k, v) for k, v in freq_lines.items() if k <= 10
            )
            freq += "\n" + "\n".join(slot_freq)
            eco = spec["eco"]
            rules = "\n".join(
                "{0}. **KR-{0}:** {1}".format(i + 1, r) for i, r in enumerate(spec["rules"])
            )
            metrics = "\n".join(
                "- **M{0}:** {1}".format(i + 1, m) for i, m in enumerate(spec["metrics"])
            )
            frefs = ", ".join(fam.get("family_refs") or ["Familie " + fam["name"]])
            ftitle = stitle if stitle.lower().endswith("standard") else stitle + " Standard"
            content = TPL.format(
                sid=sid,
                stitle=stitle,
                ftitle=ftitle,
                cat=cat,
                famname=fam["name"],
                famid=fam["id"],
                frange=fam["range"],
                eco=eco,
                note=note,
                rules=rules,
                metrics=metrics,
                freq=freq,
                aspects=aspects,
                threats=spec["threats"],
                frefs=frefs,
                TS=TS,
                dt="2026-09-08",
                napn=len(asp),
                lastreq="%03d" % (10 + len(asp)),
            )
            open(os.path.join(ROOT, e["file"]), "w", encoding="utf-8").write(content)
            count += 1
    print("{0} Standards auf v1.2.0 fertig ausgebaut".format(count))
    return 0


if __name__ == "__main__":
    sys.exit(main())
