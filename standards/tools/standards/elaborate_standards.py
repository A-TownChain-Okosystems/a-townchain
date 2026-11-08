#!/usr/bin/env python3
"""Struktur-Elaboration der SCR-0030-Batch-Standards (SCR-0031).

Hebt alle 263 Grundgerüst-Standards (v1.0.0) auf v1.1.0 (MINOR, additiv,
ATC-STD-UPDATE-001 UPD-G03): je Standard familien-spezifische Kernregeln,
Ökosystem-Verortung (aus Katalog-Notiz + Familien-Wissen), Schnittstellen,
Metriken/Akzeptanzkriterien, erweiterte REQ-STD-Menge und Security-Bedrohungen.

Tiefen-Ebene: P2-Struktur-Elaboration (dokumentiert in SCR-0031). Besondere
Engineering-Elaboration eines Slots bleibt via Einzel-SCR/MINOR möglich.
"""

import json
import os
import re
import sys

import yaml

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))

TS = "08.09.2026, 02:30 UTC+2"

# ─── Familien-Wissen: Ökosystem-Verortung, Kernregeln, Metriken, Bedrohungen ───
FAM = {
    "FAM-01": dict(
        eco=(
            "Verortet in der Enterprise-Governance-Schicht des Ökosystems (ATC-ENT-Bestand, "
            "Org A-TownChain-Okosystems). Zuständigkeitsfeld: verbindliche Unternehmens-, "
            "Policy- und Nachweisführung über alle Layer L0–L7."
        ),
        rules=[
            "Entscheidungen MÜSSEN mit Eigentümer (Owner-Delegation), Datum und Nachweis "
            "(Commit/AUD-Record) dokumentiert sein.",
            "Agenten DÜRFEN keine Eigentümer-Entscheidungen (ACCEPTED/CLOSED/Freigaben) "
            "allein treffen — Human-Gate ist Pflicht.",
            "Policies MÜSSEN einen Lifecycle (inkraftgetreten/geändert/außer Kraft) mit "
            "wirksamen Daten führen.",
            "Eskalationsketten MÜSSEN definiert sein (Wer entscheidet, Wer informiert, "
            "in welcher Frist).",
            "Nachweispflicht: Jede Regelverletzung erzeugt ein Finding (F-NNN) in "
            "registry/findings.yaml.",
            "Statusberichte MÜSSEN an REALITY_STATUS.md (kanonisch, append-only) angebunden sein.",
        ],
        metrics=[
            "100 % der Entscheidungen mit Nachweis dokumentiert",
            "0 Entscheidungen ohne Datierung",
            "Findings innerhalb der Zielfrist bearbeitet",
        ],
        threats=(
            "Governance-Umgehung durch nicht dokumentierte Ad-hoc-Entscheidungen; "
            "Verlust der Nachweisfähigkeit durch nicht angehängte Statusberichte."
        ),
    ),
    "FAM-02": dict(
        eco=(
            "Standards-Governance des ATC-Ökosystems: SCR→Registry→Manifest→Validator-Kette "
            "(ATC-STD-000, STDDEV-001, REGISTRY-001, CHANGE-001 sind normativ)."
        ),
        rules=[
            "Jeder Standard MUSS vor §9-Freigabe als DRAFT mit SCR-Eintrag existieren.",
            "Version/Status MÜSSEN zwischen Datei, standards.yaml und versions.yaml "
            "übereinstimmen (Validator S-14/S-19).",
            "Reviews SOLLTEN zur Review-Kadenz von ATC-STD-UPDATE-001 erfolgen "
            "(MAJOR-Revalidation via COMPAT-001).",
            "Deprecation MUSS dem dokumentierten Pfad folgen (Retirement-Meldung, "
            "Nachfolger benannt, Transition befristet).",
            "Änderungen am normativen Kern APPROVEDer Standards nur via UPDATE-001-Kette "
            "(§30-Freeze).",
            "Kein Standard ohne Katalog-Slot (FRAMEWORK-001, S-21).",
        ],
        metrics=["0 Version-/Status-Drift", "100 % Registry-Konsistenz", "Review-Quote zur Kadenz"],
        threats=(
            "Stille Änderungen an eingefrorenen Standards; Registry-/Datei-Divergenz "
            "durch parallele Agenten-Läufe."
        ),
    ),
    "FAM-03": dict(
        eco=(
            "Repository-Landschaft des Ökosystems: 26 Repos der Org, Namensregel atc--Präfix, "
            "Layer-Zuordnung L0–L7 (REPO_ARCHITECTURE, ATC-STD-202)."
        ),
        rules=[
            "Neue Repos MÜSSEN dem Namens- und Layer-Schema folgen und ein README mit "
            "Zweck/Layer/Status besitzen.",
            "Jedes Repo MUSS eine aktuelle Beschreibung und korrekte Labels tragen "
            "(ACTIVE/DEVELOPMENT/EXPERIMENTAL/ARCHIVIERT).",
            "Archivierung MUSS dokumentiert werden (Begründung, Nachfolger, Datenrettung "
            "in den Vault des Docs-Repos).",
            "Repos DÜRFEN keine Zweck-Duplikate bilden — Konsolidierung geht vor Neugründung.",
            "Beschreibungen MÜSSEN dem REALITY-Status entsprechen (keine STALE-Behauptungen).",
            "Änderungen an der Repo-Landschaft erfordern einen SCR mit Owner-Freigabe.",
        ],
        metrics=[
            "0 zweckgleiche Repos",
            "100 % beschriebene/gelabelte Repos",
            "0 STALE-Beschreibungen",
        ],
        threats=(
            "Zersplitterung durch unkoordinierte Repo-Neugründungen; veraltete "
            "Beschreibungen führen zu Fehlentscheidungen."
        ),
    ),
    "FAM-04": dict(
        eco=(
            "Dokumentationsschicht: REALITY_STATUS.md (a-townchain-os-docs) ist die einzige "
            "kanonische Statusquelle (append-only); Wiki-Archive im Vault."
        ),
        rules=[
            "Statusdokumentation MUSS angehängt werden (append-only), nie parallel neu "
            "aufgebaut werden.",
            "Dokumentation MUSS den REALITY_STATUS widerspruchsfrei spiegeln; bei "
            "Widerspruch gewinnt REALITY_STATUS.",
            "Behauptungen MÜSSEN datiert und verifizierbar sein (Ehrlichkeitsregel: keine "
            "überholten Ist-Behauptungen).",
            "Jedes Dokument MUSS Version/Datum/Verantwortlichen tragen.",
            "Gelöschte oder ersetzte Inhalte MÜSSEN im Vault archiviert bleiben.",
            "Automatisch generierte Kapitel MÜSSEN als solche gekennzeichnet sein.",
        ],
        metrics=[
            "0 Widersprüche zu REALITY_STATUS",
            "100 % datierte Dokumente",
            "Vault-Vollständigkeit",
        ],
        threats=(
            "Datenverlust durch Kollaps/Stummschaltung von Statusdateien (Vorfall "
            "05.07.2026); still veraltende Kopien."
        ),
    ),
    "FAM-05": dict(
        eco=(
            "Engineering-Schicht: OS-Ebene Rust no_std (atc-shivacore), App-/Contract-Ebene "
            "ATCLang — Trennung ist verbindlich. Testpflicht je Modul (cargo test)."
        ),
        rules=[
            "Kernel-Code MUSS in Rust no_std vorliegen; Userspace-Apps/Contracts in ATCLang.",
            "Jedes Modul MUSS eine eigene Testsuite haben; nur 100 % grüne Pipelines "
            "gelten als fertig (Build ≠ fertig).",
            "Subsysteme SOLLTEN trait-basiert mit simulierbaren Backends sein "
            "(Determinismus, Testbarkeit).",
            "Quellcode MUSS den verbindlichen Copyright-Header (Michael Wroblewski) tragen.",
            "Sprint-Ergebnisse MÜSSEN mit Testzahl und Modulzahl nachgewiesen werden.",
            "Keine stillen Sprünge: Sprint-Abschluss nur mit dokumentiertem Nachweis.",
        ],
        metrics=["Tests X/X grün", "Modulanzahl dokumentiert", "0 Copyright-Verstöße"],
        threats=(
            "Sprachvermischung (Rust/ATCLang) bricht die Architektur; „fertig“-Behauptung "
            "ohne grüne Tests verfälscht den Zustand."
        ),
    ),
    "FAM-06": dict(
        eco=(
            "Versionsführung: main-Zweig, Registry-Gate je Commit, dokumentierter Historien-"
            "Reset 03.07.2026 (alle vorbenannten Commit-SHAs verwaist)."
        ),
        rules=[
            "Jeder Commit MUSS die Registry-Konvention einhalten (signierte Agent-Identität, "
            "SCR-Referenz).",
            "Merges in main NUR mit zugehörigem SCR-Eintrag.",
            "Force-Push auf main ist VERBOTEN; Historien-Resets erfordern Owner-Entscheidung "
            "plus Doku-Banner über verwaiste SHAs.",
            "Commit-Messages MÜSSEN Was/Warum/Nachweis (Issue/SCR) enthalten.",
            "Parallele Agenten-Läufe DÜRFEN keine Duplikat-Commits erzeugen (Koordination "
            "über AGENT_COORDINATION).",
            "Verwaiste Referenzen (tote SHAs) MÜSSEN bereinigt oder als verwaist markiert werden.",
        ],
        metrics=[
            "0 Force-Pushes",
            "0 unkoordinierte Duplikat-Commits",
            "100 % SCR-referenzierende Merges",
        ],
        threats=(
            "Historienverlust durch Resets; Referenz-Korruption (Datenbanken zeigen auf tote SHAs)."
        ),
    ),
    "FAM-07": dict(
        eco=(
            "Fehler-Management: Findings F-NNN in registry/findings.yaml (SSOT), RCA-Kette "
            "ATC-STD-BUG-005, Priorisierung P0–P4."
        ),
        rules=[
            "Jeder Fehler MUSS eindeutig identifizierbar sein (F-NNN, Kategorie, Schwere, "
            "Entdecker, Datum).",
            "Fehler DÜRFEN nicht doppelt geführt werden (Duplikat-Check bei Anlage).",
            "P0-Fehler blockieren Releases (Health E).",
            "Jeder behobene Fehler MUSS die Ursache dokumentieren (RCA) — Symptom-Fixes "
            "allein genügen nicht.",
            "Fehlerzustände in Datenbanken (KaiOsTodo) MÜSSEN mit der Registry konsistent "
            "sein; Phantom-Einträge werden bereinigt.",
            "Track-Präfixe (K/WIN/DESKTOP/…) MÜSSEN Namenskollisionen vermeiden.",
        ],
        metrics=["0 doppelte Findings", "RCA-Quote 100 % bei P0/P1", "P0-Aufräumzeit im Ziel"],
        threats=(
            "Phantom-Einträge verfälschen Fortschrittsdaten; Symptom-Fixes ohne RCA "
            "führen zu Wiederholungsfehlern."
        ),
    ),
    "FAM-08": dict(
        eco=(
            "Qualitätssicherung: Test-Suiten je Modul, QEMU-Verifikation für Kernel-Sprints, "
            "Mutationstest-Suite im Validator (S-19), FAIL-scharfe Konformanztests."
        ),
        rules=[
            "Tests MÜSSEN deterministisch sein (keine Flakiness ohne Kennzeichnung).",
            "Grenzfälle und Fehlerpfade MÜSSEN getestet sein, nicht nur Happy Paths.",
            "Konformanzprüfungen MÜSSEN FAIL-scharf sein: ein Regelverstoß MUSS zum Fehler führen.",
            "Testergebnisse MÜSSEN maschinell reproduzierbar dokumentiert sein.",
            "Regressionstests MÜSSEN bei jedem Fix ergänzt werden.",
            "Coverage-Lücken MÜSSEN ehrlich benannt werden (keine geschönten Quoten).",
        ],
        metrics=[
            "Tests grün (X/X)",
            "Mutationstest-Trefferquote",
            "0 ungetestete Fehlerpfade laut Doku",
        ],
        threats=(
            "Grün-Getestete Oberfläche mit ungetesteten Kernpfaden; Flaky-Tests "
            "verwässern das Vertrauen in die Pipeline."
        ),
    ),
    "FAM-09": dict(
        eco=(
            "CI/CD-Schicht: governance-ci-Workflows (Naming/Governance), Dependabot in "
            "Manifest-Repos, Agent-Token-Scope-Restriktion (GH013)."
        ),
        rules=[
            "Jeder Commit MUSS die CI-Gates passieren; rote Haupt-Pipelines blockieren.",
            "Workflow-Dateien DÜRFEN nur vom Owner geändert werden (Token ohne "
            "workflow-Scope — GH013).",
            "Abhängigkeits-Alerts MÜSSEN aktiviert sein (Dependabot/CodeQL-Rollout gemanagt).",
            "CI-Meldungen MÜSSEN eindeutig benannte Checks liefern (S-01..S-26).",
            "CI-Failures MÜSSEN vor dem nächsten Merge behoben oder dokumentiert blockiert werden.",
            "Selbst-Änderungen an Gates durch Agenten sind VERBOTEN (keine Selbstanerkennung).",
        ],
        metrics=[
            "Pipeline-Erfolgsquote",
            "0 unautorisierte Workflow-Änderungen",
            "Alert-Bearbeitungszeit",
        ],
        threats=(
            "Selbstanerkennung: Agenten schwächen Gates, die sie selbst prüfen; "
            "unbehandelte Abhängigkeits-Alerts werden zur Schwachstelle."
        ),
    ),
    "FAM-11": dict(
        eco=(
            "Blockchain-Kernschicht: a-townchain (L3, Chain-ID 658467), Kernel-Kopplung K14 "
            "(P2P-Consensus Foundation) und K16 (DAG+PoH+Validator+Voting+Finality)."
        ),
        rules=[
            "Blockchain-Datenformate MÜSSEN deterministisch serialisierbar sein "
            "(kanonische Kodierung).",
            "Chain-ID (658467) MUSS in allen werttragenden Nachrichten verankert sein "
            "(Replay-Schutz).",
            "Konsens-Entscheidungen MÜSSEN nachvollziehbar (DAG-Referenzen, Voting-"
            "Nachweise) protokolliert werden.",
            "Mainnet-Wert-Transport erfordert Ed25519-Backend (REQ-PTS-006, Crypto-HAL).",
            "Protokoll-Status MUSS der Protocol-Registry entsprechen (draft→active nur "
            "mit CONF-BRONZE).",
            "Konsens-Kernregeln DÜRFEN nur via MAJOR-Kette (COMPAT-001-Gate) geändert werden.",
        ],
        metrics=[
            "Finality-Latenz",
            "0 nicht deterministische Serialisierungspfade",
            "Registry-Status konsistent",
        ],
        threats=(
            "Replay/Chain-Split durch fehlende Chain-ID-Bindung; Konsens-Divergenz "
            "durch nicht deterministische Serialisierung."
        ),
    ),
    "FAM-12": dict(
        eco=(
            "Token-Schicht auf der Contract-Ebene (ATCLang, atc-contracts): deterministische "
            "Token-Formate, Kompatibilitäts-Matrix-Pflicht."
        ),
        rules=[
            "Token-Operationen MÜSSEN deterministisch sein (Reihenfolge, Rundung, "
            "Überlauf-Verhalten definiert).",
            "Token-Standards MÜSSEN eine Kompatibilitätsmatrix je Version führen.",
            "Werttransport MUSS protokolliert werden (AuditTrail-Anbindung).",
            "Contract-Änderungen erfordern ein Audit-Gate vor Aktivierung.",
            "Token-Metadaten MÜSSEN versioniert und migrierbar sein.",
            "Supply-Regeln MÜSSEN maschinell prüfbar definiert sein (Mint/Burn-Grenzen).",
        ],
        metrics=[
            "0 Rundungs-/Überlauf-Divergenzen",
            "Matrix-Abdeckung 100 %",
            "Audit-Gate erfüllt",
        ],
        threats=(
            "Supply-Drift durch undefinierte Rundung/Überlauf; nicht migrierbare "
            "Metadaten brechen Upgrade-Pfade."
        ),
    ),
    "FAM-14": dict(
        eco=(
            "Interoperabilitätsschicht: atc-interop, Kompatibilitätsklassen nach "
            "ATC-STD-COMPAT-001, Interop-Tests als CONF-Pflichtkategorie 8."
        ),
        rules=[
            "Grenzüberschreitende Kopplungen MÜSSEN eine Kompatibilitätsklasse tragen "
            "(kein UNKNOWN bei Release).",
            "Interop-Schnittstellen MÜSSEN Conformance-Tests bestehen (CONF-Kategorie "
            "Interoperability).",
            "Breaking Changes an Interop-Grenzflächen erfordern Owner-Freigabe plus "
            "Wiederherstellungsplan (Methoden A–F).",
            "Mapping-Regeln MÜSSEN beidseitig dokumentiert sein.",
            "Versionierte Schnittstellen MÜSSEN Parallelbetrieb während der Migration erlauben.",
            "Fehlercodes an Grenzflächen MÜSSEN vollständig katalogisiert sein.",
        ],
        metrics=[
            "0 UNKNOWN-Klassen bei Release",
            "Interop-Testabdeckung",
            "Migrationsfenster eingehalten",
        ],
        threats=(
            "Stille Breaking Changes brechen Nachbarsysteme; UNKNOWN-Kompatibilität "
            "verhindert Rollback-Planung."
        ),
    ),
    "FAM-16": dict(
        eco=(
            "Oracle-/Externe-Daten-Schicht (atc-oracle): signierte Datenfeeds in die "
            "Contract-/Blockchain-Schicht."
        ),
        rules=[
            "Externe Daten MÜSSEN herkunfts-signiert (Ed25519/DID-basiert) in das "
            "Ökosystem gelangen.",
            "Oracle-Feeds MÜSSEN Replay-geschützt sein (Nonce/Timestamp/Chain-ID).",
            "Feed-Ausfälle MÜSSEN definierte Fehlerzustände erzeugen (keine stillen "
            "Alt-Daten-Verwendungen).",
            "Datenherkünfte MÜSSEN mit Quellen- und Vertrauensgrad registriert sein.",
            "Manipulationsschwellen MÜSSEN definiert sein (Abweichungs-/Ausfall-Alarme).",
            "Oracle-Konfiguration ist MAJOR-gebunden (COMPAT-001).",
        ],
        metrics=["Feed-Verfügbarkeit", "0 unsignierte Datenannahmen", "Ausfall-Erkennungszeit"],
        threats="Feed-Manipulation; Replay alter Preise/Daten ohne Zeit-/Nonce-Schutz.",
    ),
    "FAM-17": dict(
        eco=(
            "Identitäts-/Reputationsschicht: DID (Kernel K6), Ed25519 (K6b), Reputation "
            "(K15), RCT — Self-Sovereign Identity als Referenzmodell."
        ),
        rules=[
            "Identitäten MÜSSEN kryptografisch an Schlüssel gebunden sein (DID-Dokument, "
            "Rotation definiert).",
            "Reputation MUSS aus nachvollziehbaren Signalen berechnet werden (Formel "
            "dokumentiert, Manipulationsresistenz bewertet).",
            "Schlüsselverlust MUSS einen definierten Recovery-Pfad haben (Social/Threshold).",
            "Identity-Daten DÜRFEN nicht ohne Zustimmung weitergegeben werden (Datenminimierung).",
            "Sperrungen (Banned/Revoked) MÜSSEN ökosystemweit konsistent propagieren.",
            "Anonymisierung vs. Accountability MUSS explizit ausbalanciert dokumentiert sein.",
        ],
        metrics=[
            "Key-Rotation ohne Dienstausfall",
            "Reputation-Manipulationsresistenz bewertet",
            "Recovery-Erfolgsquote",
        ],
        threats=(
            "Identitätsdiebstahl durch schwache Rotation; Reputation-Gaming durch "
            "Sybil-/Collusion-Angriffe."
        ),
    ),
    "FAM-18": dict(
        eco=(
            "Cybersecurity-Schicht: SEC-C-Controls (registry/security.yaml), Protocol "
            "Threat-Modelle (ATC-STD-PROTOCOL-003, M1–M12), Zero-Trust-Ansatz."
        ),
        rules=[
            "Security-Controls MÜSSEN je System benannt und einem Zustand (ACTIVE/"
            "PARTIAL/PLANNED) zugeordnet sein — ehrlich, nicht geschönt.",
            "Secrets DÜRFEN nie im Code liegen (Secret-Scanning, 0-Fund-Pflicht).",
            "Bedrohungen MÜSSEN je Familie katalogisiert und bewertet sein "
            "(M1–M12-Matrix der Protocol-Security-Registry).",
            "Krypto-Operationen MÜSSEN über den HAL laufen (keine handgerollten Primitiven).",
            "Sicherheitsvorfälle MÜSSEN der Incident-Kette (F-NNN→RCA→Fix) folgen.",
            "Sicherheitsrelevante MAJOR-Änderungen brauchen ein Review-Gate.",
        ],
        metrics=[
            "0 Secrets in Repos",
            "Threat-Katalog-Abdeckung 100 %",
            "Time-to-Fix kritischer Findings",
        ],
        threats=(
            "HAL-Disziplin-Bruch (handgerollte Krypto); ehrlichkeitswidrige Threat-Status-Schönung."
        ),
    ),
    "FAM-21": dict(
        eco=(
            "Mining-/Validierungsschicht: Kernel K16 (DAG+PoH+Validator+Voting+Finality), "
            "Rate-Limits (K15), Validator-Rotation."
        ),
        rules=[
            "Mining-/Validierungsregeln MÜSSEN deterministisch sein (kein "
            "Nichtdeterminismus im Reward-Pfad).",
            "Validator-Zulassung/-Rotation MUSS nach dokumentierten Kriterien erfolgen.",
            "Rate-Limits MÜSSEN die Ressourcen schützen (DoS-Resistenz bewertet).",
            "Reward-Berechnung MUSS testbar und auditierbar sein (Regressionstests).",
            "Angriffe auf den Validierungsprozess MÜSSEN im Threat-Model katalogisiert sein.",
            "Änderungen an Belohnungs-/Validierungsparametern sind MAJOR (COMPAT-001).",
        ],
        metrics=[
            "0 nicht deterministische Reward-Pfade",
            "DoS-Resistenz nachgewiesen",
            "Rotations-Nachweise",
        ],
        threats=(
            "Nothing-at-Stake/Long-Range-Angriffe; Parameter-Manipulation zugunsten "
            "einzelner Validatoren."
        ),
    ),
    "FAM-22": dict(
        eco=(
            "Wallet-Schicht (atc-wallet): Key-Management, Transaktions-Erstellung, "
            "DID-Bindung, Crypto-HAL-Disziplin."
        ),
        rules=[
            "Private Keys DÜRFEN nie im Klartext persistieren (HAL-Verschlüsselung, "
            "Memory-Hygiene).",
            "Transaktions-Erstellung MUSS deterministisch signieren (Domain-Separation, Chain-ID).",
            "Backups/Recovery MÜSSEN dokumentierte, testbare Verfahren sein.",
            "Wallet-Zustand MUSS mit Chain-Status synchronisierbar sein (Indexer-Anbindung).",
            "Gerätekopplung MUSS eindeutig (Geräte-ID, Authorization-Flow) erfolgen.",
            "Fehlerzustände (ungültige Tx, abgelaufene Nonce) MÜSSEN katalogisiert sein.",
        ],
        metrics=[
            "0 Plaintext-Key-Vorfälle",
            "Signatur-Determinismus 100 %",
            "Recovery-Test erfolgreich",
        ],
        threats="Key-Diebstahl über Memory/Logs; verlustbehaftete Backup-Verfahren.",
    ),
    "FAM-23": dict(
        eco=(
            "DeFi-Schicht: Contract-Ebene (ATCLang, atc-contracts), Oracle-Kopplung, "
            "Risikolimits, Audit-Gates."
        ),
        rules=[
            "Jeder DeFi-Contract MUSS vor Aktivierung ein Audit bestanden haben.",
            "Risikoparameter MÜSSEN nach oben begrenzt sein (Exposure-Caps, Circuit Breaker).",
            "Oracle-Abhängigkeiten MÜSSEN Ausfallszenarien definieren (s. Oracle-Familie).",
            "Ökonomische Formeln MÜSSEN deterministisch und regressionstestbar sein.",
            "Upgrades MÜSSEN über Zeitfenster mit Exit-Möglichkeit laufen.",
            "Ausnahmezustände (z. B. LiquidityEngine) MÜSSEN deterministische Regeln folgen.",
        ],
        metrics=[
            "Audit-Abdeckung 100 %",
            "Exposure-Caps konfiguriert",
            "0 nicht deterministische Formeln",
        ],
        threats=(
            "Oracle-Manipulation → Bad-Debt; Governance-Übernahmen durch "
            "Konzentration von Stimmrechten."
        ),
    ),
    "FAM-24": dict(
        eco=(
            "NFT-/Marktplatz-Schicht (atc-marketplace, atc-storage): deterministische "
            "Metadaten, Royalty-Regeln, Storage-Kopplung."
        ),
        rules=[
            "NFT-Metadaten MÜSSEN deterministisch adressierbar sein (Content-Addressing).",
            "Royalty-Regeln MÜSSEN maschinell prüfbar definiert sein.",
            "Marktplatz-Matching MUSS deterministisch sein (Reihenfolge definiert).",
            "Storage-Referenzen MÜSSEN Verfügbarkeits-/Integritätsprüfungen haben.",
            "Zustandsübergänge (List/Bid/Sale) MÜSSEN vollständig katalogisiert sein.",
            "Gebühren-/Fee-Regeln MÜSSEN versioniert und auditierbar sein.",
        ],
        metrics=[
            "0 nicht deterministische Matchings",
            "Metadata-Verfügbarkeit",
            "Royalty-Compliance",
        ],
        threats="Metadata-Drift (Verweis-Verlust); Fraud über nicht geprüfte Storage-Referenzen.",
    ),
    "FAM-25": dict(
        eco=(
            "GameFi-/Shivamon-Schicht: genesis-engine (L6, Vision-Dokument), "
            "deterministische Game-Logik, Ökonomie-Balancing."
        ),
        rules=[
            "Game-Logik MUSS deterministisch sein (gleichbefindete Simulationen "
            "erzeugen gleiche Ergebnisse).",
            "Ökonomie-Parameter MÜSSEN simulierbar und balanciert sein (Inflation-Modelle "
            "ehrlich bewertet).",
            "Vision-Komponenten (ATC-41+) MÜSSEN von Engineering-Metriken getrennt "
            "bleiben (keine Sprint-ETAs aus Vision-Docs).",
            "On-Chain/Off-Chain-Trennung MUSS definiert sein (was landet auf der Chain).",
            "Anti-Cheat-Regeln MÜSSEN katalogisiert sein.",
            "Content-Updates DÜRFEN Konsens-Kern nicht brechen (MAJOR-Gate).",
        ],
        metrics=[
            "Determinismus-Simulationen identisch",
            "Ökonomie-Modelle bewertet",
            "0 Vision-Metriken in Sprint-ETAs",
        ],
        threats=(
            "Nicht deterministische Game-Logik bricht Konsens-Zustände; "
            "Ökonomie-Exploits durch unbewertete Parameter."
        ),
    ),
    "FAM-26": dict(
        eco=(
            "API-Schicht (atc-gateway): versionierte Schnittstellen, Rate-Limiting, "
            "vollständige Fehlerkataloge."
        ),
        rules=[
            "APIs MÜSSEN versioniert sein (Version im Pfad/Header, Deprecation-Fenster definiert).",
            "Fehlercodes MÜSSEN 100 % katalogisiert und stabil sein (keine Umbenennung "
            "ohne MAJOR).",
            "Rate-Limits MÜSSEN je Endpunkt definiert und durchgesetzt sein.",
            "Authentisierung MUSS über die Identity-Schicht (DID/Session) laufen.",
            "API-Änderungen erfordern Kompatibilitätsprüfung (COMPAT-001, Interop-Klasse).",
            "Dokumentation MUSS synchron zur Implementierung sein (keine Ghost-Endpunkte).",
        ],
        metrics=["Fehlerkatalog-Abdeckung 100 %", "Rate-Limit-Treue", "Doku-Sync 100 %"],
        threats=(
            "Breaking Changes ohne Versionsfenster; Endpoint-Enumeration über "
            "undokumentierte Routen."
        ),
    ),
    "FAM-27": dict(
        eco=(
            "Datenstandards-Schicht: SSOT-Register (registry/*.yaml), JSON-Schemas "
            "(schemas/), Migration-Pflichten."
        ),
        rules=[
            "Datenmodelle MÜSSEN als Schema (JSON-Schema/YAML) definiert sein.",
            "Register MÜSSEN SSOT sein — Duplikat-Quellen werden abgeschafft, nicht gepflegt.",
            "Migrationen MÜSSEN Versionssprünge definieren und reversibel dokumentiert sein.",
            "Feld-Semantik MUSS dokumentiert sein (Einheiten, Null-Semantik, Referenzen).",
            "Maschinenlesbarkeit MUSS gewährleistet sein (Validator-Suite prüfbar).",
            "Schema-Änderungen laufen über die Change-Control-Kette.",
        ],
        metrics=[
            "0 Duplikat-Register",
            "Schema-Validierungsquote 100 %",
            "Migration reversibel getestet",
        ],
        threats="Semantik-Drift bei Feldern; Parallelregister zersplittern die SSOT.",
    ),
    "FAM-28": dict(
        eco=(
            "Observability-Schicht: Kernel security_audit (K15), strukturierte Logs, "
            "Metriken je Subsystem."
        ),
        rules=[
            "Betriebsdaten MÜSSEN strukturiert (Schema, Level, Kontext) erhoben werden.",
            "Kritische Ereignisse MÜSSEN unveränderlich protokolliert sein (AuditTrail-Kopplung).",
            "Metriken MÜSSEN definierte Schwellen mit Alarmierung haben.",
            "Health-/Statusberichte MÜSSEN automatisch generierbar sein.",
            "Log-Flut MUSS begrenzt sein (Retention, Sampling dokumentiert).",
            "Sensible Daten DÜRFEN nicht in Logs landen (Redaktionsregeln).",
        ],
        metrics=[
            "Alert-Treue (0 False Negatives kritisch)",
            "Log-Strukturkonformanz",
            "Metrik-Schwellen konfiguriert",
        ],
        threats="Alarm-Ermüdung durch Rauschen; Beweisverlust durch rotierte/ungefilterte Logs.",
    ),
    "FAM-29": dict(
        eco=(
            "Incident-/Recovery-Schicht: F-NNN→RCA→Fix-Kette, Rollback-Verfahren nach "
            "UPDATE-001, Postmortem-Pflicht."
        ),
        rules=[
            "Vorfälle MÜSSEN mit Zeitstempel, Schwere und Betroffenheit erfasst werden.",
            "Eindämmung MUSS vor Ursachenanalyse möglich sein (definierte Runbooks).",
            "Rollbacks MÜSSEN getestete Verfahren sein (UPD-G-Kette, Wiederherstellung A–F).",
            "Postmortems MÜSSEN Ursache, Wirkung, Fix und Prävention dokumentieren.",
            "Wiederholungsvorfälle MÜSSEN zur Regelrevision führen.",
            "Kommunikationspflichten (wer informiert wen) MÜSSEN definiert sein.",
        ],
        metrics=["Time-to-Mitigate", "Postmortem-Quote 100 %", "Rollback-Tests erfolgreich"],
        threats=(
            "Recovery ohne getesteten Rollback verschlimmert Vorfälle; kulturelle "
            "Blame-Orientierung verhindert ehrliche RCAs."
        ),
    ),
    "FAM-31": dict(
        eco=(
            "Projektmanagement-Schicht: Meilenstein-Governance ATC-STD-MILESTONE-001 "
            "(13-Status-Lifecycle, 8 Gates, Evidence-Pflicht, ATC-M-001..008)."
        ),
        rules=[
            "Meilensteinfortschritt MUSS am dokumentierten Lifecycle gemessen werden "
            "(keine Statussprünge).",
            "Acceptance-Gates MÜSSEN mit Evidence (Commits, Tests, Reports) bedient werden.",
            "Agenten DÜRFEN Meilensteine nicht selbst ACCEPTED/CLOSED setzen (Human-Gate).",
            "Kritische offene Dependencies blockieren ACCEPTED.",
            "MAJOR-Revalidierung bindet COMPAT-001.",
            "Sprint-Tracks (Kernel vs. Konsolidierung) MÜSSEN getrennt geführt werden.",
        ],
        metrics=["0 Statussprünge", "Evidence-Abdeckung 100 %", "Gate-Durchlaufzeiten"],
        threats=(
            "Papier-Fortschritt ohne Evidence; Meilenstein-Inflation durch Agenten-Selbstabschluss."
        ),
    ),
    "FAM-32": dict(
        eco=(
            "Requirements-Engineering: REQ-Registry (registry/requirements.yaml, SSOT), "
            "Traceability REQ→STD→AUDIT, Familien-/Kategorie-Anträge (ATC-FAM-REQ, "
            "ATC-CAT-REQ)."
        ),
        rules=[
            "Anforderungen MÜSSEN eindeutige REQ-IDs tragen und im REQ-Register geführt werden.",
            "Traceability MUSS lückenlos sein (REQ→Standard→Implementierung→Nachweis).",
            "Anforderungsänderungen MÜSSEN über die Change-Control-Kette laufen.",
            "Anforderungen MÜSSEN testbar/verifizierbar formuliert sein (keine "
            "Wunsch-Formulierungen).",
            "Kategorien-/Familienbildung folgt der Taxonomie (TAXONOMY-001).",
            "Offene Anforderungen MÜSSEN sichtbar sein (keine stillen Backlog-Items).",
        ],
        metrics=["Traceability-Abdeckung 100 %", "0 untestbare REQs", "Backlog-Sichtbarkeit"],
        threats=(
            "Phantom-Anforderungen ohne Verifikationspfad; Traceability-Brüche "
            "verhindern Impact-Analysen."
        ),
    ),
    "FAM-33": dict(
        eco=(
            "UI/UX-Schicht: Desktop-Clients (atc-windows-/atc-linux-edition) mit egui, "
            "DESKTOP-S2–S5-Plan, Konsistenz- und Accessibility-Basis."
        ),
        rules=[
            "UI-Änderungen MÜSSEN dem Plattformplan folgen (egui, gemeinsame Codebasis).",
            "Interaktionsmuster MÜSSEN konsistent sein (ein Verhalten, eine Bedeutung).",
            "Zustandsfeedback MUSS für alle asynchronen Operationen existieren "
            "(Loading/Fehler/Leer).",
            "Accessibility-Basis (Kontrast, Tastaturbedienung, Skalierung) MUSS "
            "gewährleistet sein.",
            "UI-Texte MÜSSEN zentral verwaltet sein (keine hartverdrahteten Dubletten).",
            "Layout-Regressionen MÜSSEN über UI-Tests/Screenshots prüfbar sein.",
        ],
        metrics=[
            "0 hartverdrahtete UI-Texte",
            "Feedback-Abdeckung 100 %",
            "Accessibility-Basis-Checks bestanden",
        ],
        threats="Zweigeteilte UX durch Client-Divergenz; unbemerkte Regressionen ohne UI-Tests.",
    ),
    "FAM-34": dict(
        eco=(
            "Plattform-/OS-Schicht: ShivaCore (Kernel, no_std) vs. Desktop-Editions "
            "(Rust std) vs. globus-os (L4 Userspace) — Trennung verbindlich (AD-026)."
        ),
        rules=[
            "OS-Komponenten MÜSSEN der Layer-Zuordnung folgen (L1 Kernel, L4 Userspace, "
            "Editions separat).",
            "Editions-Repos DÜRFEN den bare-metal-Kernel nicht ersetzen (parallele Vorhaben).",
            "Boot-/Treiber-Verhalten MUSS in QEMU/hardwarenah verifizierbar sein.",
            "Plattform-APIs MÜSSEN stabil dokumentiert sein (Syscalls, ABI).",
            "Geräteunterstützung MUSS je Zielplattform katalogisiert sein.",
            "Kernel-/Userspace-Grenze MUSS sauber gehalten werden (keine Ring-0-Helfer "
            "im Userspace).",
        ],
        metrics=[
            "QEMU-Boot-Nachweise",
            "Syscall-/ABI-Doku 100 %",
            "0 Grenzverletzungen Kernel/Userspace",
        ],
        threats="Feature-Creep im Kernel destabilisiert das System; ABI-Brüche brechen Userspace-Programme.",
    ),
    "FAM-35": dict(
        eco=(
            "ATCLang-Schicht: atclang (L0), Compiler-/VM-Pipeline (ATVM, atc-vm), "
            "AD-022-Gates G1–G6, Parser-Realität ehrlich dokumentiert."
        ),
        rules=[
            "Sprachgrammatik MUSS versioniert sein (Syntax-Änderungen = MAJOR).",
            "Compiler-Verhalten MUSS deterministisch sein (gleiche Eingabe, gleiche Ausgabe).",
            "Dialekt-/Kompatibilitätsmodi MÜSSEN explizit deklariert sein (kein stiller "
            "v1.0-Dialekt).",
            "Fehlermeldungen MÜSSEN katalogisiert und versionierbar sein.",
            "Gate-Fortschritt (G1–G6) MUSS mit Test-Evidence geführt werden.",
            "VM-Semantik MUSS gegen die Spezifikation getestet sein (Konformanz).",
        ],
        metrics=[
            "Parser-/Compiler-Konformanz (X/X Dateien)",
            "Gate-Evidence G1–G6",
            "0 nicht deterministische Kompilate",
        ],
        threats="Still wachsender Dialekt-Wildwuchs; Nichtübereinstimmung VM↔Spezifikation.",
    ),
    "FAM-36": dict(
        eco=(
            "AuditTrail-/LogChain-Schicht: Kernel security_audit (K15), AUD-Record-Pflicht "
            "(AI-DEV-009), unveränderliche Protokollketten."
        ),
        rules=[
            "Auditrelevante Ereignisse MÜSSEN unveränderlich protokolliert sein "
            "(Append-only, ggf. hashverkettet).",
            "AUD-Records MÜSSEN die Pflichtfelder (WER/WAS/WANN/Nachweis) vollständig führen.",
            "Protokollierung MUSS strukturiert und abfragbar sein.",
            "Manipulationsversuche an Protokollen MÜSSEN erkennbar sein (Integritätsprüfung).",
            "Aufbewahrungs-/Zugriffsregeln MÜSSEN definiert sein.",
            "Kettenkopplung an die Blockchain MUSS deterministisch erfolgen.",
        ],
        metrics=[
            "AUD-Record-Vollständigkeit 100 %",
            "Integritätsprüfungen bestanden",
            "Abfragbarkeit nachgewiesen",
        ],
        threats="Protokoll-Lücken verhindern Forensik; nachträgliche Protokoll-Änderung fälscht Nachweise.",
    ),
    "FAM-37": dict(
        eco=(
            "Supply-Chain-/Abhängigkeitsschicht: Dependabot-Abdeckung, Lockfiles, SBOM-"
            "Pflicht, Vendor-Audit."
        ),
        rules=[
            "Abhängigkeiten MÜSSEN gesperrt sein (Lockfiles, keine floating Tags).",
            "Neue Abhängigkeiten MÜSSEN reviewt werden (Zweck, Lizenz, Pflegezustand).",
            "Automatische Abhängigkeits-Alerts MÜSSEN aktiv sein und bearbeitet werden.",
            "SBOM/Abhängigkeitslisten MÜSSEN je Release aktuell sein.",
            "Build-Reproduzierbarkeit MUSS gegeben sein (Vendor-Quellen versioniert).",
            "Kritische Abhängigkeitslücken (CVE) blockieren Releases (P0-Kette).",
        ],
        metrics=[
            "Lockfile-Abdeckung 100 %",
            "Alert-Bearbeitungszeit",
            "0 unaufgelöste kritische CVEs",
        ],
        threats="Supply-Chain-Kompromittierung (typosquatted/übernommene Pakete); nicht reproduzierbare Builds erschweren Forensik.",
    ),
    "FAM-38": dict(
        eco=(
            "Open-Source-/Lizenzschicht: Copyright-Name „Michael Wroblewski“ verbindlich "
            "(>500 Vorkommen), Lizenz-Konsistenz über alle Repos."
        ),
        rules=[
            "Der Copyright-Name MUSS unverändert „Michael Wroblewski“ bleiben.",
            "Lizenzen MÜSSEN je Repo eindeutig deklariert sein (LICENSE-Datei).",
            "Lizenz-Header MÜSSEN konsistent sein (Automatisierung prüfbar).",
            "Drittanbieter-Lizenzen MÜSSEN erfasst und kompatibel sein (Pflichten dokumentiert).",
            "Lizenzwechsel sind Owner-Entscheidungen (MAJOR, dokumentiert).",
            "Contributions MÜSSEN den Lizenzbedingungen zugeordnet werden können.",
        ],
        metrics=[
            "Lizenz-Deklaration 100 %",
            "Header-Konsistenz 100 %",
            "0 inkompatible Dritt-Lizenzen",
        ],
        threats="Lizenz-Inkonsistenzen erzeugen Rechtsrisiken; still eingeführte Copyleft-Abhängigkeiten kontaminieren Projekte.",
    ),
    "FAM-39": dict(
        eco=(
            "Business-/Ökonomie-Schicht: Tokenomics-Modelle, Wirtschaftlichkeitsnachweise, "
            "Trennung Vision (ATC-41+) vs. Engineering-Realität."
        ),
        rules=[
            "Ökonomische Modelle MÜSSEN mit Annahmen, Parametern und Grenzen dokumentiert sein.",
            "Simulationen MÜSSEN deterministisch wiederholbar sein.",
            "Vision-Dokumente DÜRFEN nicht als Engineering-Fortschritt gezählt werden "
            "(getrennte Metriken).",
            "Wertschöpfungsannahmen MÜSSEN ehrlich bewertet sein (keine garantierte-"
            "Rendite-Formulierungen).",
            "Parameteränderungen mit ökonomischer Wirkung sind MAJOR (COMPAT-001).",
            "Berichte MÜSSEN datiert und versioniert sein.",
        ],
        metrics=[
            "Modell-Dokumentation vollständig",
            "Simulationen reproduzierbar",
            "0 Vermischung Vision/Engineering",
        ],
        threats="Überoptimistische Modelle (ohne Grenzen) werden zu Fehlinvestitionen; nicht reproduzierbare Simulationen sind nicht auditierbar.",
    ),
}

TPL = """---
standard:
  id: {sid}
  title: "{ftitle}"
  version: "1.1.0"
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

# {sid} — {stitle} (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0, §30-eingefroren) — Grundgerüst-Standard aus Katalog-Slot der Familie
> {famname} ({famid}); §9-FREIGEGEBEN 08.09.2026, 02:15 UTC+2 — APPROVED, normativ,
> §30-eingefroren. Struktur-Elaboration v1.1.0 via SCR-0031 ({TS}):
> familien-spezifische Kernregeln, Ökosystem-Verortung, Schnittstellen, Metriken und
> Security-Bedrohungen additiv ergänzt (MINOR, ATC-STD-UPDATE-001 UPD-G03).

## Abstract

{sid} ({stitle}) ist der Standard für den gleichnamigen Katalog-Slot der Familie
**{famname}** ({famid}, Range {frange}) im ATC Enterprise Standards Framework. Er
definiert den Gegenstand, seine Verortung im Ökosystem, die verbindlichen
Kernregeln, Compliance- und Verifikationspflichten sowie Security-Betrachtungen.
Der Standard ist APPROVED, normativ in Kraft und §30-eingefroren (Owner-§9-Freigabe
08.09.2026, 02:15 UTC+2, SCR-0030-Batch); die Struktur-Elaboration erfolgte via
SCR-0031 als MINOR v1.1.0. Besondere Engineering-Vertiefung erfolgt inkrementell
via eigener SCR/MINOR-Kette (ATC-STD-UPDATE-001).

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

## §2 Kernregeln (elaboriert)

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

Akzeptanz gilt als nachgewiesen, wenn die genannten Kriterien in einem AUD-Record
oder Validator-Lauf dokumentiert sind; fehlende Nachweise werden als Findings
geführt und nach ATC-STD-BUG-005 (RCA) bearbeitet.

## §5 Compliance & Verifikation

Compliance wird über die Gesamt-Validierung (CI, S-01..S-25) je Registry-Eintrag
geprüft: Metadaten-Vollständigkeit, Naming, Status-/Version-Konsistenz und
Registry-Konsistenz. Abweichungen werden als Findings (F-NNN) geführt und nach
ATC-STD-BUG-005 (RCA) bearbeitet.

## Requirements (normativ)

- **REQ-STD-001** (§1): Gegenstand eindeutig definiert und im Katalog verortet.
- **REQ-STD-002** (§2): Fachliche Regeln als deklarierte, verifizierbare REQ-IDs.
- **REQ-STD-003** (§2): Compliance nachweisbar über Validator-Gates oder Prüfung.
- **REQ-STD-004** (§2): Änderungen ausschließlich über die Change-Control-Kette.
- **REQ-STD-005** (§2): Sicherheitsaspekte dokumentiert (Security Considerations).
{freq}

## Security Considerations

{threats}

Bis zur Engineering-Vertiefung gilt die Ehrlichkeitsregel: Sicherheitszustände
MÜSSEN ehrlich benannt sein (ACTIVE/PARTIAL/PLANNED); erfundene Sicherheitszusagen
sind verboten (vgl. ATC-STD-PROTOCOL-003).

## Changelog (Standard-intern)

- **1.1.0** ({dt}): MINOR via SCR-0031 — Struktur-Elaboration: familien-spezifische
  Kernregeln (§2), Ökosystem-Verortung (§1), Schnittstellen (§3), Metriken/
  Akzeptanzkriterien (§4) und Security-Bedrohungen ergänzt; REQ-STD-006..010
  additiv. Additiv und abwärtskompatibel (ATC-STD-UPDATE-001 UPD-G03).
- **1.0.0** (2026-09-08): Initial Release — Grundgerüst-Standard aus
  Katalog-Slot {sid} (Familie {famname}, {famid}) via SCR-0030; §9-FREIGEGEBEN
  08.09.2026, 02:15 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- ATC-STD-000 (Standards Governance & Specification), ATC-STD-FRAMEWORK-001
- ATC-STD-UPDATE-001/CHANGE-001 (Change-Control), ATC-STD-BUG-005 (RCA)
- registry/framework.yaml ({famid}), registry/standards.yaml + versions.yaml

*{sid} v1.1.0 · Struktur-Elaboration via SCR-0031 · Aurora (Superagent) · 08.09.2026*
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
        6: "Gegenstand im Ökosystem verortet (§1-Verortung).",
        7: "Familien-Kernregeln deklariert und verifizierbar (§2).",
        8: "Schnittstellen zu Registry/Governance-Kette/Agenten gebunden (§3).",
        9: "Metriken/Akzeptanzkriterien definiert, Nachweis via AUD-Record (§4).",
        10: "Familienspezifische Security-Bedrohungen katalogisiert (§5).",
    }
    count = 0
    for fam in fw["families"]:
        spec = FAM.get(fam["id"])
        for s in fam.get("slots", []):
            if s["id"] not in new_ids:
                continue
            assert spec, "Keine Familien-Spez für " + fam["id"]
            sid, stitle = s["id"], s["title"]
            e = byid[sid]
            cat = e["category"]
            note = s.get("note") or ""
            note = re.sub(r"[;,]?\s*Grundgeruest via SCR-0030.*$", "", note).strip()
            note = (
                ("**Katalog-Referenz:** " + note)
                if note
                else (
                    "**Katalog-Referenz:** keine zusätzliche Katalog-Notiz; Verortung "
                    "ausschließlich über Familie und Slot."
                )
            )
            eco = spec["eco"]
            rules = "\n".join(
                "{0}. **KR-{0}:** {1}".format(i + 1, r) for i, r in enumerate(spec["rules"])
            )
            metrics = "\n".join(
                "- **M{0}:** {1}".format(i + 1, m) for i, m in enumerate(spec["metrics"])
            )
            freq = "\n".join(
                "- **REQ-STD-{0:03d}** (§2/§4): {1}".format(k, v) for k, v in freq_lines.items()
            )
            threats = spec["threats"]
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
                threats=threats,
                frefs=frefs,
                TS=TS,
                dt="2026-09-08",
            )
            path = os.path.join(ROOT, e["file"])
            open(path, "w", encoding="utf-8").write(content)
            count += 1
    print("{0} Standards auf v1.1.0 elaboriert".format(count))
    return 0


if __name__ == "__main__":
    sys.exit(main())
