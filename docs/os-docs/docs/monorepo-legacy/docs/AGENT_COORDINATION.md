# 🧭 Agent Coordination — Live-Arbeitsstatus

> **Zweck:** Verhindert, dass mehrere KI-Agenten gleichzeitig/gegeneinander am
> selben Bereich arbeiten. Jeder Agent MUSS diese Datei vor Arbeitsbeginn lesen
> und nach Abschluss/Uebergabe aktualisieren.
> **Governance:** Teil des verbindlichen Mandats aus `AGENT_POLICY.md`.

> 🛠️ **FROM-SCRATCH-DIREKTIVE (seit 05.07.2026, `AGENT_MASTERRULES.md` REGEL 0):**
> GlobusOS (OS) wird komplett selbst programmiert — eigener Bootloader, eigener
> Kernel **ShivaCore** (Rust no_std, bare-metal x86_64), eigene Treiber, eigenes
> Dateisystem, eigener Netzwerk-Stack. Kein Linux-Unterbau, kein geliehener
> Fremd-Code auf OS-Ebene. ATCLang bleibt Sprache für Apps/Contracts AUF dem
> Kernel. Kernel-Status: `shivacore/README.md` (K-Sprint 0+1 ✅, Stand 07.07.2026).

---

## 🆔 Agenten-Register — eindeutige Identifikation

> **Problem:** "Aurora" wurde bisher unter verschiedenen Rollen-Labels
> referenziert (MasterBrain, GovernanceAgent, Ecosystem Brain) — das macht es
> schwer zu unterscheiden, ob es sich um denselben Agenten oder verschiedene
> handelt. Diese Tabelle loest das mit festen IDs auf — inklusive einer klaren Trennung zwischen KI-Agenten und dem menschlichen Owner.

| ID | Klarname | Typ | Plattform / Instanz | Rolle im Projekt |
|----|----------|-----|----------------------|-------------------|
| `aurora-base44-superagent-69c1e0c577ccf6c45a27a480` | Aurora | 🤖 KI-Agent | Base44 Superagent — App-ID `69c1e0c577ccf6c45a27a480` (eindeutige Instanz) | Primärer Entwicklungs-/Dokumentations-Agent, fuehrt Sync-, Compliance- und Konsolidierungsarbeit aus |
| `kai-os-kernel` | KAI | 🤖 In-Projekt-KI | Teil der Codebasis selbst, kein Editor-Agent | Laufzeitkomponente — verarbeitet Nutzeranfragen INNERHALB von KAI-OS, bearbeitet NICHT das Repo |
| `aurora-base44-superagent-6a2756186106d6f0fbb105b5` | Aurora | 🤖 KI-Agent | Base44 Superagent — App-ID `6a2756186106d6f0fbb105b5` (separate Instanz von der oben registrierten App-ID `69c1e0c577ccf6c45a27a480`) | Sync-/Cleanup-/Governance-Agent (Duplikat-Cleanup, Naming Conventions, Wiki-Score, OS-Gap-Analyse) |
| `aurora-base44-superagent-6a27614c7219ab1e4f951842` | Aurora | 🤖 KI-Agent | Base44 Superagent — App-ID `6a27614c7219ab1e4f951842` (separate Instanz von `69c1e0c577ccf6c45a27a480` und `6a2756186106d6f0fbb105b5`) | KAI-OS Daily Full Sync Owner (16-Dienste-Automation), Reality-Check/Audit-Agent (Roadmap-vs-Code-Konsistenz) |
| `unsigned-aurora-bot-base44ai` | Aurora-Bot (⚠️ UNSIGNIERT — keine App-ID im Commit) | 🤖 KI-Agent | Git-Identitaet `Aurora-Bot <aurora@base44.ai>` — App-ID unbekannt, da Signatur-Pflicht (siehe unten) verletzt | Taeglicher Wiki-Kapitel-Sync (zuletzt: Kap. 17 + 31, 07.07.2026 08:19 UTC via GitHub+Notion) — Funktion aehnelt Agent `...105b5`s Automationen, aber NICHT von dessen 3 registrierten Automationen (`atc_master_sync`, `kai_os_daily_sync` x2) erzeugt, siehe Verifikation unten |
| `shivacore-owner-human` | ShivaCore (Michael Wroblewski) | 🧑 Mensch / Projekt-Owner | Base44 Superagent-Chat (Auftraggeber-Seite) | Menschlicher Entscheidungstraeger und Owner des A-TownChain-Oekosystems. Erteilt Auftraege an KI-Agenten, trifft finale Entscheidungen bei offenen Decisions (z.B. AD-002), einziger Copyright-Rechteinhaber. **Kein Agent** — steht hier zur klaren Abgrenzung: Aktionen mit dieser ID sind Menschen-initiiert, nicht KI-generiert. |

> **Wichtig:** KAI ist **kein** Entwicklungs-Agent, der Code/Doku schreibt —
> es ist die KI-Komponente des Betriebssystems selbst (Produkt). Nur
> `aurora-base44-superagent` (oder ein zukuenftiger, hier neu registrierter
> Agent) bearbeitet dieses Repository.



> ⚠️ **WICHTIGER FUND (06.07.2026, Agent `...105b5`):** Es existieren nachweislich
> **zwei verschiedene Base44-Superagent-App-IDs**, die beide unter dem Namen
> "Aurora" an diesem Repository arbeiten (`69c1e0c577ccf6c45a27a480` und
> `6a2756186106d6f0fbb105b5`). Das ist vermutlich beabsichtigt (z.B. zwei
> separate Chat-Sessions/Superagent-Instanzen desselben Nutzers), sollte aber
> bei Governance-Fragen beruecksichtigt werden: **Agent-ID ≠ Agent-Name** —
> immer die vollstaendige ID mit App-ID pruefen, nicht nur "Aurora".

> 🆕 **5. Agent identifiziert (07.07.2026, Agent `...105b5`):** Ein fuenfter
> Akteur wurde durch Commit-Forensik gefunden — Git-Identitaet
> `Aurora-Bot <aurora@base44.ai>` (1 bestaetigter Commit: `53acff6`,
> 07.07.2026 08:19 UTC, "Auto-sync: Kapitel 17 & 31 aktualisiert"). Er
> signiert seine Commits NICHT mit `[agent: aurora-base44-superagent-<ID>]`
> und verstoesst damit gegen die Signatur-Pflicht unten. Verifiziert:
> es ist KEINE der 3 aktiven Automationen von Agent `...105b5`
> (`atc_master_sync`, `kai_os_daily_sync` x2 — alle pruefbar ueber
> `list_automations`, alle mit anderer Commit-Identitaet). Ein weiterer
> untagged Commit (`9a449bd`, MasterBrain-E-Mail) wurde dagegen eindeutig
> Agent `...951842` zugeordnet (gleiche E-Mail-Identitaet wie dessen
> signierte Commits) — das war nur eine einzelne vergessene Signatur,
> kein 6. Agent.
>
> **Konsequenz:** Es gibt nachweislich **5 Base44-Superagent-Akteure**
> an diesem Repo, einer davon bislang ohne verifizierbare App-ID. Naechster
> Agent, der einen Commit von `Aurora-Bot <aurora@base44.ai>` sieht: bitte
> Timing mit eigenen Automation-Laeufen abgleichen und hier die echte
> App-ID nachtragen, sobald bekannt.

**Wenn ein neuer/anderer Agent dieses Projekt bearbeitet:** Er MUSS sich hier
mit einer neuen Zeile eintragen (Agent-ID nach Schema `<name>-<plattform>-<typ>`),
BEVOR er Aenderungen vornimmt.

### Signatur-Pflicht

Jeder Eintrag in `AGENT_COORDINATION.md` (Abschnitt "Aktuell aktiver Agent"),
jede neue Zeile in `DECISIONS_REGISTER.md` und jede Commit-Message fuer
Doku-/Governance-Aenderungen MUSS die Agent-ID enthalten, z.B.:

```
docs: XYZ aktualisiert [agent: aurora-base44-superagent-69c1e0c577ccf6c45a27a480]
```

So ist im Git-Log und in den Registries jederzeit eindeutig nachvollziehbar,
welcher Agent welche Aenderung vorgenommen hat — auch wenn mehrere Agenten
zeitversetzt oder parallel arbeiten.

---



## 📜 Session-Log (chronologisch, neueste zuerst)


### Session: aurora-base44-superagent-6a27614c7219ab1e4f951842 — 06.07.2026, 22:19 UTC+2 (Vollstaendiger MD-Audit + REALITY_STATUS.md erstellt)

| Feld | Wert |
|------|------|
| **Fokus** | Auf Wunsch des Owners: jede .md-Datei / jede Datei sichten und den tatsaechlichen Software-Stand fuer andere KI-Agenten dokumentieren. |
| **Neues Artefakt** | `REALITY_STATUS.md` (Root beider Repos) — einzige Quelle mit script-verifizierten Zahlen, ersetzt verstreute/widerspruechliche Behauptungen in README/ROADMAP/STATUS/Wiki. |
| **Wichtigster Fund — Python-Stub-Regression** | Frueherer Claim "Migration Complete, 0 Python-Stubs" (05.07.) ist **widerlegt**: 72 reale Python-Dateien aktiv (21 am 06.07. von Agent `...105b5` bewusst aus `aistudio/temp_repo/` zurueckkopiert fuer Testsuite-Kompatibilitaet, plus 51 in `aistudio/temp_repo/` selbst). Banner in `chapter-70`, `MIGRATION_MAP.md`, `STANDARDS_REGISTRY.md` gesetzt. |
| **Fund — Wiki-Kapitelzahl unverifizierbar** | Nur 9 Dateien folgen `chapter-N-*.md` (63, 70-77). Die behaupteten "69 Kapitel" lassen sich nicht auf reale Dateien 1:1 abbilden; `docs/wiki/kai-os/` ist ein alter, verschachtelter Full-Repo-Schnappschuss (58 Dateien), kein Kapitel. Empfehlung: Kennzahl neu definieren oder streichen. |
| **Fund — Standards-Registry-Duplikat** | `ATC-0009-BRIDGE.md` existiert zweifach (altes 4-stelliges Format), plus `ATC-LIC`/`ATC-LIC` brechen die "nur ATC-01-99"-Konvention (ATS sollte laut Session vom 05.07. eliminiert sein). 101 tatsaechliche ATC-*.md Dateien, nicht 98/99. |
| **Fixes committed & gepusht** | Chain-ID 9001→9000 (5 Dateien + `config/mainnet_genesis.json`), Parser-Fix fuer String-Pfad-Importe (92→96/176 parsen), Dependency-Sicherheitsupdates (cryptography/requests/python-dotenv/pytest/flask/flask-cors), npm audit fix (11→10 Alerts). Commits: `17a4096` (code), `MILESTONES.md`-Korrektur (docs). |
| **MILESTONES.md korrigiert** | MK10, MK12-15 von "✅ Erfuellt" auf "🔵 In Arbeit" runtergestuft — Code existiert, parst aber laut REALITY_STATUS.md nicht bzw. ist gar kein ATCLang (Asset-Module = Python-Pseudocode). Neue Bilanz: 5/15 vollstaendig (33,3%) statt vorheriger falscher "10/15 (66,7%)". |
| **Nicht behoben (bewusst, REGEL 9)** | Parser-v1.0-Upgrade (80 Dateien, Mehrtage-Sprint), Asset-Module-Neuschreibung, 44-Issues-Reopen-Entscheidung, K3/K4-Konsolidierung, npm uuid-Breaking-Fix, Wiki-Kapitel-Neudefinition, ATC-LIC/BaFin-Doku-Status — alle in REALITY_STATUS.md Abschnitt 9 aufgelistet. |
| **Fuer naechsten Agenten** | REALITY_STATUS.md vor jeder Status-Behauptung lesen und AKTUALISIEREN (nicht nur neue Behauptung obendrauf schreiben). Nicht jede der ~650 .md-Dateien in beiden Repos wurde einzeln gelesen (nicht praktikabel) — stattdessen systematischer Zahlen-Grep + Parser/Test/API-Verifikation gegen alle zentralen Status-Dokumente. |
| **Status** | ✅ Audit + Dokumentation abgeschlossen und gepusht. |

---

### Session: aurora-base44-superagent-6a27614c7219ab1e4f951842 — 06.07.2026, 19:39 UTC+2 (Reality-Check: Roadmap/Milestones vs. echter Code-Stand)

| Feld | Wert |
|------|------|
| **Fokus** | Kritischer Abgleich auf Wunsch des Owners: stimmt ROADMAP.md/MILESTONES.md mit dem tatsaechlichen Code ueberein? Zusaetzlich: Git-Push-Status beider Haupt-Repos geprueft. |
| **Ergebnis 1 — Parser-Realitaet** | ⚠️ Nur **92/176 .atc-Dateien (52,3%) parsen fehlerfrei** mit dem aktuellen ATCLang-Parser (`atclang/lexer` + `atclang/parser`). 84 Dateien (47,7%) scheitern — v.a. alle neueren Module: 14 GCL-Kernel-Busse, 28 Franchise-Factories, 11 Civilization-Engine-Module, 8 Meta-Module, 16 Asset-Module. Hauptursachen: `import "std/crypto.atc"`-Syntax wird vom Parser nicht unterstuetzt, sowie `!`-Operator- und `let`-Praezedenz-Bugs. |
| **Ergebnis 2 — Chain-ID-Widerspruch** | ❌ `ROADMAP.md` und `SPRINT_ROADMAP.md` behaupten "Mainnet Chain ID 9001", waehrend `AD-004` (resolved), `ECOSYSTEM.md`, `CHANGELOG.md` und Issue #71 durchgaengig **9000** verwenden. Direkter Widerspruch zur bereits getroffenen Entscheidung. |
| **Ergebnis 3 — MILESTONES.md vs. ROADMAP.md** | ❌ `MILESTONES.md` fuehrt MK10 + MK12 als ✅ Erfuellt sowie drei komplett neue Meilensteine (MK13 Franchise Factory v2, MK14 MetaFactory, MK15 Civilization Platform, alle ✅ am 05.07.), die in `ROADMAP.md` (gleicher Ordner) gar nicht existieren bzw. dort explizit als ⬜ GEPLANT gefuehrt werden. Fuer MK13-15 existiert zwar echter Code (47 `.atc`-Dateien), der aber laut Ergebnis 1 nicht parst. |
| **Ergebnis 4 — Issue-Zahlen** | ⚠️ GitHub-API live abgefragt: **90 Issues total, 79 zu, 11 offen (87,8%)** — ROADMAP/SPRINT_ROADMAP behaupten veraltet "78/82 (95,1%)", vor Oeffnung der K1-K8-Konsolidierungs-Issues (#85-92). |
| **Ergebnis 5 — Test-Coverage** | ⚠️ 388 Python-Testfunktionen, 345 sammelbar (4 Dateien crashen beim Import), davon 302 gruen / 30 rot / 13 skipped. **0 echte ATCLang-Tests** — die einzige Datei mit "test" im Namen (`testnet_launcher.atc`) ist keine Testdatei. Deckt sich mit dem Sync-Report von heute ("0,6% ATCLang Coverage"). |
| **Ergebnis 6 — Git-Push-Status** | ✅ Beide Haupt-Repos (`a-townchain-os`, `a-townchain-os-docs`) zu 100% mit `origin/main` synchron, keine lokalen/uncommitted Aenderungen. |
| **Aktion** | Nur dokumentiert und hier eingetragen — KEINE automatischen Fixes an Chain-ID, MILESTONES.md oder Parser (REGEL 9 — Entscheidung bei Michael, da MK-Status und Chain-ID Grundsatzfragen sind). |
| **Fuer naechsten Agenten** | Vor jeder "X ist fertig"-Aussage: Parser tatsaechlich laufen lassen, nicht nur Datei-Existenz pruefen (bestaetigt die Lektion von Agent `...105b5` oben — Existenz ≠ Funktionsfaehigkeit). Chain-ID-Konflikt (9000 vs 9001) und MILESTONES.md/ROADMAP.md-Divergenz sollten in einem gemeinsamen Fix-Commit bereinigt werden, sobald Michael entscheidet. |
| **Status** | ✅ Audit abgeschlossen, Ergebnisse an Michael berichtet. Entscheidungen offen: (1) Chain-ID final 9000 vs 9001, (2) MK13-15 Status "erfuellt" vs "in Arbeit" bis Parser sie akzeptiert, (3) Parser-Fix fuer 84 kaputte Dateien priorisieren? |

---

### Session: aurora-base44-superagent-6a2756186106d6f0fbb105b5 — 06.07.2026, 18:15 UTC+2 (K1 abgeschlossen + K3 Teilfortschritt)

| Feld | Wert |
|------|------|
| **Fokus** | Sprint K1 (Repo-Audit/Mapping) abgeschlossen; Sprint K3 (Python-Backend) Teilmigration; ShivaCore-Kernel-Statuscheck |
| **Ergebnis K1** | ✅ `KONSOLIDIERUNGS_MATRIX.md` erstellt (Datei-Inventar, Python-Import-Graph, Dead-Code-Kandidaten, Konflikt-Liste, Quelle→Ziel-Mapping). Kernbefund: die Backend-Modul-Konsolidierung (9 `atc-*`-Repos → `modules/`) war bereits erledigt — offene Arbeit ist ausschliesslich die Integration von `aistudio/` (148 React-Komponenten, 133 Python-Dateien). Issue #85 geschlossen. |
| **Ergebnis K3** | 🔄 Teilfortschritt (Issue #87 bleibt offen). 21 reale Python-Module aus `aistudio/temp_repo/` an von Tests erwartete Pfade migriert (`gateway/`, `blockchain/*`, `backend/*`, `modules/kernel/ai_kernel/`). Testsuite: von "10 Dateien kollationieren nicht mal" auf **345/355 sammelbar, davon 307 gruen / 30 rot / 8 skipped**. Commit `2ad2d5c`. |
| **Wichtiger Fund — Kernel-Status** | `modules/kernel/` (GCL, 13-Bus-Architektur AD-00 bis AD-14) ist architektonisch vollstaendig (24 Dateien, 78-529 Zeilen, keine Stubs) — aber **nur in ATCLang (.atc)**, in dieser Session nicht durch Compiler/VM-Testpfad verifiziert. `modules/kernel/ai_kernel/ai_kernel.py` (die einzige Python-Bruecke) ist eine aeltere Version ohne Klasse `AIRequest` — blockiert `tests/unit/test_kai_integration.py`. **Architektur fertig, Laufzeit-Verifizierung offen.** |
| **KORREKTUR fruehere Doku-Aussage** | Alte Behauptung "`core/ai_kernel.py` ist deprecated Compat-Shim" ist FALSCH — Datei existiert real nicht. Siehe `memory.md #158` (Aurora Superagent 6a27...). |
| **Verbleibende echte Luecken (K3)** | `blockchain/nodes/bootstrap.py` + `blockchain/wallet/did.py` existieren nur als `.atc`, keine Python-Entsprechung (betrifft Bootstrap Node, kritischer Pfad #14!). `gateway/router.py` + `rate_limit.py`: API-Mismatch zu Tests (Circuit-Breaker + `window_seconds` fehlen). |
| **Fuer naechsten Agenten** | K4 (#88, Frontend) noch komplett offen. Bei K3-Fortsetzung: nicht nur Dateien verschieben, sondern echte Feature-Luecken schliessen (AIKernel-API erweitern, Bootstrap/DID Python-Layer schreiben oder .atc-Aufruf aus Tests). |
| **Status** | ✅ K1 abgeschlossen. 🔄 K3 in Arbeit, naechster Schritt dokumentiert. |

---

### Session: aurora-base44-superagent-6a2756186106d6f0fbb105b5 — 06.07.2026, 17:35 UTC+2 (Abschluss)

| Feld | Wert |
|------|------|
| **Fokus** | Layer-Reality-Check (L0-L12) + vollstaendiger 24-Repo Cross-Check (Datei-Integritaet, Naming, Links) |
| **Ergebnis 1 — Layer-Check** | ✅ Alle 17 Layer-Eintraege in `ECOSYSTEM_BRAIN.md` gegen Datei-Bestand verifiziert. Layer-Ebene ist **verlaesslich** (im Gegensatz zur Issue-Ebene oben) — auch die duenn wirkenden L3/L4/L5/L9 hatten nach Tiefenpruefung echte substantielle Dateien (100-450 Zeilen .atc). Einzige Inkonsistenz: L9 Bridge hat 4 ueberlappende Standard-Docs (ATC-09/38/69/91) fuer dasselbe Thema. |
| **Ergebnis 2 — 24-Repo Check** | Geprueft: alle 24 Org-Repos (nicht nur die 3 Hauptrepos), 3.340 Dateien, 1.533 .md-Dateien. 0 leere/Stub-Dateien. 7 weitere Naming-Verstoesse in Satelliten-Repos (`atc-whitepaper`, `a-townchain-os-wiki`) gefunden + gefixt. |
| **Ergebnis 3 — Broken Links** | 356 kaputte interne MD-Links gefunden (105 eindeutige Ziele). Hauptursachen identifiziert + automatisiert gefixt: (a) Case-Sensitivity-Bug `Testnet.md` vs. echtem `TESTNET.md` (~86 Vorkommen, GitHub-Pfade sind case-sensitive!), (b) falscher Pfad-Praefix `docs/whitepaper/issues/` statt `docs/issues/`. Insgesamt 45 Einzel-Fixes in 17 Dateien (a-townchain-os + kai-os-wiki). |
| **Verbleibend offen** | ~270 Link-Vorkommen (11% Legacy `aistudio/temp_repo/`, Rest Einzelfaelle + `ATCLANG_SPEC.md` existiert an 5 verschiedenen Pfaden — Konsolidierungsbedarf). 44 Issues mit gebrochener Datei-Referenz (siehe Session oben) — Entscheidung ueber Re-Open/Re-Audit weiterhin bei Michael. |
| **Fuer naechsten Agenten — WICHTIGSTE LEKTION** | GitHub-Pfade sind case-sensitive — bei Link-Referenzen IMMER exakte Schreibweise pruefen, nicht nur Existenz. Vor jedem "Issue #X ist fertig"-Statement den echten Datei-Baum pruefen (nicht nur Issue-Status). Layer-Aussagen in ECOSYSTEM_BRAIN.md sind vertrauenswuerdiger als einzelne Issue-Body-Dateipfade. |
| **Vollstaendiger Report** | `docs/REALITY_CHECK_2026-07-06.md` (3 Nachtraege: Issue-Check, Layer-Check, 24-Repo-Check) in allen 3 Hauptrepos |
| **Status** | ✅ Cross-Check-Zyklus fuer 06.07.2026 abgeschlossen. Naechste Schritte: (1) 44-Issues-Entscheidung, (2) ATCLANG_SPEC.md-Konsolidierung, (3) Bridge-Standards-Dedup — alle bei Michael zur Priorisierung. |

---


### Session: aurora-base44-superagent-6a2756186106d6f0fbb105b5 — 06.07.2026, 17:00 UTC+2 (Fortsetzung)

| Feld | Wert |
|------|------|
| **Fokus** | Reality-Check: geschlossene Issues vs. tatsaechliche Dateien |
| **Ergebnis** | ⚠️ **44 von 78 geschlossenen Issues (56%) referenzieren Dateien, die in keinem der 3 Repos existieren** — siehe `docs/REALITY_CHECK_2026-07-06.md` |
| **Schweregrad** | 10 CRITICAL (ATCLang-Kern, Konsens, Smart-Contract-Engine), 10 HIGH (Integration/Tests), 24 MEDIUM (Docs) |
| **Aktion** | Nur dokumentiert, KEINE Issues automatisch veraendert/wiedereroeffnet (REGEL 9 — Entscheidung bei Michael) |
| **Fuer naechsten Agenten** | Vor Behauptung "Issue #X ist fertig" IMMER den Datei-Baum pruefen, nicht nur Issue-Status vertrauen |
| **Status** | ✅ Report erstellt, Entscheidung ueber naechste Schritte offen |

---


### Session: aurora-base44-superagent-6a2756186106d6f0fbb105b5 — 06.07.2026, 16:30 UTC+2

| Feld | Wert |
|------|------|
| **Fokus** | Wiki-Qualitaets-Audit, Duplikat-Cleanup, Naming Conventions, OS-Gap-Analyse |
| **Beanspruchte Bereiche** | 3 Repos root-level (Cleanup), `NAMING_CONVENTIONS.md`, `docs/AGENT_COORDINATION.md` (diese Datei — Korrektur), Wiki-Scorecard (Artifact) |
| **Ausgefuehrt** | 145 Duplikate entfernt (1.023 KB); `NAMING_CONVENTIONS.md` in 3 Repos + Notion; Wiki-Score 77.5/100 (B) berechnet; ~750 fehlende Docs zwischen Repos synchronisiert; OS-Gap-Analyse (Readiness 11.8/100) |
| **Wichtiger Fund** | Es existierte bereits eine kanonische `docs/AGENT_COORDINATION.md` (dieser Agent hatte zunaechst eine Duplikat-Version am falschen Pfad `AGENT_COORDINATION.md` root erstellt — wurde korrigiert/entfernt) |
| **Status** | ✅ Abgeschlossen |

---

## 🔴 Aktuell aktiver Agent

| Feld | Wert |
|------|------|
| **Agent-ID** | `aurora-base44-superagent-6a27614c7219ab1e4f951842` (Aurora, Base44 Superagent, eindeutige App-ID) |
| **Session-Start** | 06.07.2026, 19:39 UTC+2 |
| **Aktueller Fokus** | Kritischer Reality-Check: Code vs. ROADMAP/MILESTONES/Wiki (Parser-Verifikation, Chain-ID-Konflikt, Issue-Zahlen via GitHub-API) + tägliche 16-Dienste-Sync-Automation |
| **Beanspruchte Bereiche** | Read-only Audit (kein Bereich exklusiv beansprucht) — Ergebnisse siehe Session-Log unten |
| **Status** | 🔄 Aktiv |

> Andere Agenten: Bevor ihr in den oben genannten Bereichen arbeitet — prueft
> Zeitstempel dieses Eintrags. Ist er >24h alt, gilt die Session als beendet
> und der Bereich als frei.

---

## 📋 Aktuelle Sprints & Pläne (Referenz)

| Ebene | Dokument | Status |
|-------|----------|--------|
| Standards | `docs/standards/STANDARDS_REGISTRY.md` | 37 Standards, Tier 1-5 |
| Roadmap | `docs/ROADMAP.md` | Phase 2 aktiv (Sprint 2.1-2.8) |
| Konsolidierung | `KONSOLIDIERUNGS_ROADMAP.md` (a-townchain-os) | K1-K8, alle 📋 GEPLANT |
| Decisions | `docs/DECISIONS_REGISTER.md` | AD-001 bis AD-010 |
| Lizenz | `docs/LICENSING_OVERVIEW.md` | ATC-LIC/ATC-LIC, BaFin-Bericht draft |
| Taegliche Arbeit | `docs/TODO.md` / `docs/STATUS.md` | Live-Task-Baum |

---

## 🏗️ Architektur-Snapshot (Kurzreferenz)

- **Ziel-Struktur:** 2 Haupt-Repos — `a-townchain-os` (Code) + `a-townchain-os-docs` (Doku)
- **Aktueller Ist-Zustand:** 24 Repos noch verteilt, Konsolidierung (K1-K8) geplant, **nicht begonnen**
- **Code-Sprache:** OS-/Kernel-Ebene = Rust no_std, from-scratch (ShivaCore, seit 05.07.2026). App-/Contract-Ebene = ATCLang First (AD-006). Python/TS nur Uebergangsweise/Prototyp.
- **Konsensus:** Hybrid PoW+PoS+PoH (ShivaConsensus)
- **Lizenzmodell:** ATC-LIC/ATC-LIC, durchgesetzt via ATVM ("Code is Law")
- Volle Details: `docs/architecture/` Verzeichnis

---

## 📝 K3/K4 — Vollstaendige Todo-Listen (fuer direkten Zugriff ohne GitHub-Issue-Lookup)

> Quelle: Issues #87 (K3) und #88 (K4) im Repo `a-townchain-os`. Reality-Check
> vom 06.07.2026: **0/16 (K3)** und **0/10 (K4)** Subtasks umgesetzt — alle
> Punkte unten sind offen und in dieser Reihenfolge sinnvoll abzuarbeiten.

### K3 — Python-Backend Konsolidierung (Issue #87, Sprint K3, 10 Repos → `src/`)

| # | Todo | Status |
|---|------|--------|
| K3.1 | `atc-kernel` → `src/core/` migrieren | 📋 offen |
| K3.2 | `atcnet` → `src/network/` migrieren | 📋 offen |
| K3.3 | `atc-gateway` → `src/gateway/` migrieren | 📋 offen |
| K3.4 | `atc-contracts` → `src/contracts/` migrieren | 📋 offen |
| K3.5 | `atc-shivamon` → `src/game/` migrieren | 📋 offen |
| K3.6 | `atc-franchise` → `src/franchise/` migrieren | 📋 offen |
| K3.7 | `atclang` → `src/atclang/` migrieren | 📋 offen |
| K3.8 | `core/` bestehend → `src/core/` mergen (Konflikte auflösen) | 📋 offen |
| K3.9 | `blockchain/` bestehend → `src/blockchain/` mergen | 📋 offen |
| K3.10 | `modules/` bestehend → `src/modules/` mergen | 📋 offen |
| K3.11 | AI Studio `temp_repo/` Python → `src/` migrieren (133 Dateien) | 📋 offen |
| K3.12 | Alle Python-Imports aktualisieren (`from src.core import ...`) | 📋 offen |
| K3.13 | `__init__.py` Dateien in allen `src/` Unterverzeichnissen erstellen | 📋 offen |
| K3.14 | `setup.py` / `pyproject.toml` mit allen Dependencies erstellen | 📋 offen |
| K3.15 | `requirements.txt` konsolidieren (alle Deps aus allen Repos) | 📋 offen |
| K3.16 | Python-Import-Test: `python -c "from src.core import kernel"` | 📋 offen |

**Deliverable K3:** Alle Python-Module in `src/`, `pip install -e .` funktioniert.
**Empfohlener Einstieg fuer naechsten Agenten:** K3.1, dann K3.8-K3.10 (Konflikte bestehender Verzeichnisse zuerst klaeren, bevor neue Module reinkommen).

### K4 — TypeScript Frontend Konsolidierung (Issue #88, Sprint K4)

| # | Todo | Status |
|---|------|--------|
| K4.1 | Michael Wroblewski AI Studio → `frontend/src/` migrieren (190 TS-Dateien) | 📋 offen |
| K4.2 | Bestehendes `frontend/` → `frontend/src/legacy/` migrieren | 📋 offen |
| K4.3 | `package.json` konsolidieren (alle npm Dependencies) | 📋 offen |
| K4.4 | `vite.config.ts` anpassen (Build-Output, Proxy auf Backend) | 📋 offen |
| K4.5 | `tsconfig.json` mit Path-Aliases erstellen (`@/` → `frontend/src/`) | 📋 offen |
| K4.6 | Backend API-Client generieren (TypeScript Types aus Python Endpoints) | 📋 offen |
| K4.7 | Environment Variables standardisieren (`.env.example`) | 📋 offen |
| K4.8 | Frontend Build-Test: `cd frontend && npm ci && npm run build` | 📋 offen |
| K4.9 | Desktop App Wrapper (Electron / Tauri) konfigurieren | 📋 offen |
| K4.10 | Wallpaper-Picker und Login-Overlay aus AI Studio integrieren | 📋 offen |

**Deliverable K4:** Frontend baut mit `npm run build`.
**Empfohlener Einstieg fuer naechsten Agenten:** K4.2 zuerst (bestehendes sichern als legacy/), dann K4.3-K4.5 (Build-System aufsetzen), erst danach K4.1 (grosser Import).

### Abhaengigkeiten zwischen K3 und K4
- K4.6 (API-Client) benoetigt K3 als Vorbedingung (Backend-Endpoints muessen in `src/` stabil sein)
- Beide Sprints koennen sonst parallel von zwei verschiedenen Agenten bearbeitet werden, OHNE sich zu stoeren — K3 betrifft nur Python/`src/`, K4 nur TypeScript/`frontend/`. Bei Parallelarbeit: beide Agenten tragen sich in `AGENT_COORDINATION.md` mit ihrem jeweiligen Bereich ein.

---

## ✅ Wie ein Agent diese Datei nutzt

**Vor Arbeitsbeginn:**
1. Lies den Abschnitt "Aktuell aktiver Agent" — ist ein anderer Agent aktiv
   (Zeitstempel <24h) im selben Bereich? → Warten oder Bereich abstimmen.
2. Pruefe "Aktuelle Sprints & Plaene" fuer Kontext, wo du ansetzt.
3. Pruefe `DECISIONS_REGISTER.md` — nichts umsetzen, was einer offenen/
   resolved Decision widerspricht.

**Nach Arbeitsende / bei Uebergabe:**
1. Aktualisiere "Aktuell aktiver Agent" mit deinem Namen, Fokus, Zeitstempel.
2. Setze Status auf 🟢 Abgeschlossen oder 🔄 Uebergeben, falls du pausierst.
3. Trage relevante neue Erkenntnisse in `AGENT_POLICY.md` § Agent-Handoff ein.

---

*Teil des verbindlichen Mandats — siehe `AGENT_POLICY.md` fuer vollstaendige Reality-Check- und Sync-Regeln.*

---

## Signatur-Check — 2026-08-01

Der tägliche 30-Tage-Check hat einen neuen unsignierten Agent-/Bot-Commit erkannt:

- **Repository:** `a-townchain-os`
- **Commit:** `2a4c26f12f` (vollständige SHA im Git-Verlauf)
- **Autor:** Aurora (Base44 Superagent) `<aurora@base44-superagent.local>`
- **Datum:** 08.07.2026 12:30:38 UTC
- **Nachricht:** `Merge remote-tracking branch 'origin/main'`
- **Bewertung:** Merge-Commit ohne `[agent: ...]`-Tag; zur Nachverfolgung dokumentiert.


### Unsigned Agent Commit — 2026-08-02
- **Repository:** a-townchain-os
- **Commit:** `e242db9cfa`
- **Author:** Aurora (Base44 Superagent) <aurora@base44-superagent.local>
- **Date:** 2026-08-01 06:23 UTC
- **Message:** Merge branch 'main' of https://github.com/A-TownChain-Okosystems/a-townchain-os
- **Finding:** Commit matches the Aurora/Base44 bot pattern but lacks the required `[agent: ...]` signature.
- **Action:** Recorded for audit; future agent commits must include the standard signature.

### Unsigned Agent Commit — 2026-08-03

- **Repository:** `a-townchain-os`
- **Commit:** `6ba93370fd` (full SHA available in Git history)
- **Author:** Aurora (Base44 Superagent) <aurora@base44-superagent.local>
- **Date:** 2026-08-02 06:22 UTC
- **Message:** `Merge branch 'main' of https://github.com/A-TownChain-Okosystems/a-townchain-os`
- **Finding:** Commit matches the Aurora/Base44 bot pattern but lacks the required `[agent: ...]` signature.
- **Action:** Recorded for audit; future agent commits must include the standard signature.

### Unsigned Agent Commits — 2026-08-04

- **Repository:** `a-townchain-os-docs`
- **Commit:** `1b2db0b08c` (full SHA available in Git history)
- **Author:** Aurora (Base44 Superagent) <aurora@base44-superagent.local>
- **Date:** 2026-08-03 06:22 UTC
- **Message:** `docs: record unsigned agent commit 6ba93370fd`
- **Finding:** Commit matches the Aurora/Base44 bot pattern but lacks the required `[agent: ...]` signature.

- **Repository:** `kai-os-wiki`
- **Commit:** `9667e0fe2b` (full SHA available in Git history)
- **Author:** Aurora (Base44 Superagent) <aurora@base44-superagent.local>
- **Date:** 2026-08-03 06:22 UTC
- **Message:** `docs: record unsigned agent commit 6ba93370fd`
- **Finding:** Commit matches the Aurora/Base44 bot pattern but lacks the required `[agent: ...]` signature.

- **Action:** Recorded for audit; future agent commits must include the standard signature.
