# 🧠 A-Town Ecosystem Brain — Masterregeln für KI-Agenten

> **Dokument-Typ:** Verbindliche Betriebsregeln
> **Gilt für:** Alle KI-Agenten die am A-TownChain Ökosystem arbeiten
> **Verwaltet von:** Aurora (MasterBrain) · Base44 Superagent
> **Stand:** 2026-06-12, REGEL 0 korrigiert am 07.07.2026 | **Version:** 1.1

---

## REGEL 0 — GRUNDPRINZIP

> **ALLES WIRD FROM-SCRATCH PROGRAMMIERT. Kein Linux-Unterbau, kein**
> **geliehener Code fremder Betriebssysteme/Kernel/Treiber.**

Architektur-Entscheidung (05.07.2026, verbindlich, ersetzt jede
gegenteilige ältere Aussage in diesem Dokument): **GlobusOS** wird als
eigenständiges Betriebssystem komplett selbst programmiert — eigener
Bootloader, eigener Kernel (**ShivaCore**), eigene Treiber, eigenes
Dateisystem, eigener Netzwerk-Stack. Explizit KEINE Linux-Distribution,
KEIN Linux-Kernel als Unterbau, KEINE geliehenen/gepackten Treiber oder
Kernel-Module fremder Systeme.

### Sprachen-Realität (korrigiert — ersetzt alte "Alles ist ATCLang"-Aussage)

```
OS-Ebene (Bootloader, Kernel ShivaCore, Treiber, Dateisystem, Netzwerk):
    → Rust, no_std, bare-metal (x86_64-unknown-none)
    → Code liegt unter shivacore/ im Repo — siehe shivacore/README.md
    → NICHT ATCLang, NICHT Python, NICHT C

App-/Contract-Ebene (läuft AUF dem GlobusOS-Kernel, nicht Teil des Kernels):
    → ATCLang (Smart Contracts, Anwendungslogik)
    → Python-Code im Repo ist Alt-Prototyp/Simulation, kein Zielzustand
```

**Wichtig für alle Agenten:** "Alles ist ATCLang" (alte Version dieser Regel)
gilt NICHT für den Kernel/OS-Layer. Wer Kernel-/Treiber-/Bootloader-Code
schreibt oder reviewt: Rust no_std ist Pflicht, ATCLang-Vorschläge für diese
Ebene sind ein Fehler. Wer App-/Contract-Code auf dem OS schreibt: ATCLang
ist Pflicht.

Der aktuelle Stand des Kernels (K-Sprint-Fortschritt, Boot-Anleitung,
Testverfahren) steht in `shivacore/README.md` und `KERNEL_FROM_SCRATCH_PLAN.md`
— dort immer zuerst nachschauen, bevor an Kernel-Code gearbeitet wird.

---

## REGEL 1 — REALITY-CHECK (Oberste Pflicht)

Ein Agent darf **niemals** behaupten, eine Aktion ausgeführt zu haben,
ohne einen bestätigten API-Response erhalten zu haben.

### Ausführungsstatus — immer explizit angeben

| Symbol | Status | Bedeutung |
|--------|--------|-----------|
| ✅ | AUSGEFÜHRT | API-Response mit ID/SHA bestätigt |
| 🔄 | VORBEREITET | Content fertig, Push steht aus |
| 📋 | GEPLANT | Logik steht, kein API-Call |
| 🔲 | SIMULIERT | Nur lokal, kein externer Effekt |
| ❌ | FEHLGESCHLAGEN | API-Fehler erhalten |
| ⚠️ | UNKLAR | Response mehrdeutig |

### Checkliste vor jeder externen Aktion

```
GitHub Push:
  □ Token gültig?          (401 = nein → abbrechen)
  □ Repo existiert?         (404 = nein → abbrechen)
  □ SHA der Datei geladen?  (PUT ohne SHA = neue Datei)
  □ Response enthält commit.sha?

Issue/Ticket erstellen:
  □ API erreichbar?
  □ Kein Duplikat? (Titel prüfen)
  □ Response enthält number/id?

E-Mail senden:
  □ Token gültig?
  □ Response enthält Message-ID?
  □ WICHTIG: Nur senden — niemals lesen!

Notion aktualisieren:
  □ Token gültig?
  □ Datenbank-ID bekannt?
  □ Response enthält page id?
```

---

## REGEL 2 — WISSENS-ZUSTANDSMODELL

Jede Information trägt einen expliziten Zustand.
Niemals Information ohne Zustand verarbeiten oder weitergeben.

| Zustand | Symbol | Bedeutung |
|---------|--------|-----------|
| PROVEN | ✓✓ | Mathematisch/formal verifiziert |
| IMPLEMENTED | ✓ | Code + Tests + Wiki vorhanden |
| DOCUMENTED | 📄 | Nur dokumentiert, Code fehlt |
| DERIVED | ~ | Logisch abgeleitet |
| ASSUMPTION | ? | Annahme, Bestätigung empfohlen |
| TODO | ○ | Erkannte Lücke, muss implementiert werden |
| VALIDATE | ⚠️ | Widerspruch, menschliche Prüfung |
| DECISION | 🔴 | Fachentscheidung offen, Mensch required |
| CONFLICT | ✗ | Zwei Quellen widersprechen sich |

---

## REGEL 3 — CODE ↔ DOKUMENTATION ABGLEICH

Bei **jeder** Änderung — diese 6 Schritte in dieser Reihenfolge:

```
1. Code analysieren       Was existiert tatsächlich?
2. Architektur prüfen     Layer L0-L12 korrekt?
3. Wiki prüfen            Kapitel vorhanden und aktuell?
4. Roadmap prüfen         Sprint-Zuordnung stimmt?
5. Standards prüfen       ATC-Nummer referenziert?
6. Issues prüfen          Ticket vorhanden?
```

6 Abgleichsfragen — alle müssen NEIN sein vor jedem Release:

```
□ Dokumentation ohne Implementierung?  → DOCUMENTED (Warnung)
□ Implementierung ohne Dokumentation?  → Wiki erstellen (Blocker!)
□ Ticket ohne Umsetzung?               → TODO
□ Umsetzung ohne Ticket?               → Issue erstellen (Pflicht!)
□ Roadmap-Eintrag ohne Epic?           → Epic ableiten
□ Epic ohne Roadmap-Eintrag?           → Roadmap aktualisieren
```

**Keine Änderung darf isoliert existieren.**
**Jede Änderung propagiert auf alle betroffenen Systeme.**

---

## REGEL 4 — WIKI-VALIDIERUNG

### Status-Lifecycle

```
DRAFT → REVIEW → APPROVED → PUBLISHED → DEPRECATED
```

### Vor PUBLISHED: Alle 5 Checks müssen grün sein

```
□ Technisch korrekt?          (Code stimmt mit Wiki überein)
□ Konsistent mit Code?        (Klassen, Methoden, Signaturen exakt)
□ Konsistent mit Architektur? (Layer-Zuordnung stimmt)
□ Konsistent mit Standards?   (ATC-Nummer vorhanden)
□ Konsistent mit Roadmap?     (Sprint-Zuordnung stimmt)
```

### Wiki-Kapitel-Format (Pflicht)

Jedes Kapitel enthält:
- Zugehörige GitHub Issues (Kap. 31 = zentrale Registry)
- Implementierungsstatus (PROVEN / IMPLEMENTED / DOCUMENTED / TODO)
- Layer-Zuordnung (L0–L12)
- Verknüpfte ATC-Standards
- Sprint-Zuordnung

---

## REGEL 5 — ISSUE-ERSTELLUNG (Automatisch)

**Trigger:** Neues Wiki-Kapitel, neue Komponente, neue Klasse, neuer API-Endpunkt

### Pflichtformat

```yaml
title:       "[Kap. XX] Komponente — Kurzbeschreibung"
body: |
  ## Was
  Kurze Beschreibung der Aufgabe

  ## Warum
  Begründung (Architektur / Standard / User Story)

  ## Akzeptanzkriterien
  - [ ] Messbare Bedingung 1
  - [ ] Messbare Bedingung 2
  - [ ] Messbare Bedingung 3

  ## Verknüpfungen
  - Wiki: Kap. XX
  - Standard: ATC-XXXX
  - Layer: LX
  - Sprint: X.X
  - Abhängigkeiten: #XX, #XX

labels:      [priority:HIGH, sprint:2.x, layer:LX]
```

**Nur erstellen wenn:**
- GitHub API erreichbar ✅
- Kein Duplikat (Titel-Check zuerst!) ✅

---

## REGEL 6 — GITHUB SYNC-PROTOKOLL

### Vor jedem Push (Pflicht-Checklist)

```
□ Linting:           ruff check --ignore=E501,F401,F811
□ Tests vorhanden:   tests/test_*.atc (oder .py als Stub)
□ Wiki-Kapitel:      Vorhanden und APPROVED?
□ Architektur:       Layer-Zuordnung konsistent?
□ Standard:          ATC-Nummer referenziert?
```

### Nach jedem erfolgreichen Push (Pflicht)

```
□ Commit-SHA notieren
□ CHANGELOG.md aktualisieren
□ VERSION erhöhen (bei Release)
□ Wiki-Status → PUBLISHED setzen
□ EcosystemNode (Base44) aktualisieren
□ Alle 10 Sync-Schritte ausführen (Regel 10)
```

### Push-Ausgabe immer so formatieren

```
✅ AUSGEFÜHRT: commit abc12345 — Beschreibung
❌ FEHLGESCHLAGEN: {"message": "..."} — Beschreibung
```

---

## REGEL 7 — AUDIT-PROZESS

Für jede Hauptkomponente fünf Audit-Typen:

| Typ | Was wird geprüft |
|-----|-----------------|
| Architektur-Audit | Layer-Zuordnung, Abhängigkeiten, Interfaces |
| Code-Audit | Klassen, Methoden, Signaturen, Logik |
| Sicherheits-Audit | Kryptografie, Input-Validierung, Replay-Schutz |
| Dokumentations-Audit | Wiki-Deckung, Konsistenz, Aktualität |
| Standard-Audit | ATC/KIP/AIP/ATS Compliance |

### Jeder Audit-Fund enthält

```
Problem:    Konkret, mit Datei + Zeile
Risiko:     CRITICAL / HIGH / MEDIUM / LOW
Sprint:     Wann beheben?
Empfehlung: Konkreter Fix-Befehl
Status:     TODO / IN_PROGRESS / DONE
```

### Audit-Score

```
90-100:  ✅ Release erlaubt
80-89:   ⚠️ Release mit Auflagen
< 80:    ❌ Release GESPERRT
```

---

## REGEL 8 — RELEASE-BLOCKER

Release ist **gesperrt** solange einer dieser Punkte gilt:

```
❌ Audit-Score unter 80/100
❌ Kritische Sicherheitslücke offen
❌ Neue Klasse/API ohne Wiki-Kapitel
❌ Neues Modul ohne Tests
❌ Offene CRITICAL oder HIGH Blocker-Issues
❌ Standard im DRAFT-Status (nicht APPROVED)
❌ Offenes DECISION-Item das Release betrifft
❌ CI/CD schlägt fehl
❌ Wiki-Kapitel im DRAFT-Status
```

---

## REGEL 9 — ENTSCHEIDUNGS-GRENZEN

Diese Themen löst kein Agent automatisch.
Sie werden als `DECISION` im Register erfasst und an Michael gemeldet.

### Offene Decisions (Stand: 2026-06-12)

| ID | Impact | Thema | Deadline |
|----|--------|-------|---------|
| AD-004 | 🔴 CRITICAL | Chain-ID 9000 vs. XDC Network | 20.06.2026 |
| AD-001 | 🟡 HIGH | SHA-256 vs. Keccak-256 | Sprint 3.0 |
| AD-003 | 🟡 HIGH | Voting-Power Snapshot (Flash-Loan) | vor Mainnet |
| AD-002 | 🟠 MEDIUM | EventBus vs. IPCBus Redundanz? | Sprint 2.4 |
| AD-005 | 🟠 MEDIUM | ATC-97 Spezifikation fehlt | Sprint 2.3 |
| AD-007 | 🔴 CRITICAL | EVM Chain Registry Konflikt | 20.06.2026 |

> AD-006 (Python vs. Substrate) → **RESOLVED (überholt durch Entscheidung 05.07.2026)**: Kernel/OS-Ebene = Rust no_std (from-scratch, kein Linux, kein Substrate). App-/Contract-Ebene = ATCLang.

### Grundsatz

```
Mathematische Beweise ohne Verifikation  → DERIVED (nicht PROVEN)
Token-Ökonomie Parameter                 → VALIDATE → DECISION
Mainnet-Konfiguration                    → DECISION (Mensch)
Kryptografie-Algorithmus-Wechsel         → DECISION (Mensch)
Chain-ID / Netzwerk-Identität            → DECISION (Mensch)
```

---

## REGEL 10 — SYNC-REIHENFOLGE (Nach jeder Änderung)

Immer in dieser Reihenfolge, jeden Schritt mit Status:

```
 1. GitHub Code     → ✅/❌ mit Commit-SHA
 2. GitHub Wiki     → ✅/❌ mit Commit-SHA
 3. EcosystemNode   → Zustand in Base44 aktualisieren
 4. Wiki-Status     → PUBLISHED setzen
 5. Issues          → Verlinkungen prüfen / ergänzen
 6. Roadmap/Sprint  → Status aktualisieren
 7. CHANGELOG.md    → Eintrag hinzufügen
 8. Notion          → Tagesprotokoll aktualisieren
 9. Google Tasks    → Offene Tasks synchronisieren
10. Google Drive    → Report hochladen
11. Gmail           → Status-Report senden (NUR SENDEN, NIEMALS LESEN)
12. Outlook         → Status-Report senden
```

---

## REGEL 11 — 12 AGENTEN-ROLLEN

| # | Agent | Aufgabe | Trigger |
|---|-------|---------|---------|
| 1 | KnowledgeAgent | Alle Dienste scannen, Wissensgraph bauen | Jeder Sync |
| 2 | ArchitectAgent | Layer L0-L12, APIs, Abhängigkeiten | Code-Änderung |
| 3 | StandardsAgent | ATC/ATS/KIP/AIP pflegen | Neue Features |
| 4 | RoadmapAgent | Epics, Milestones, Sprints ableiten | Issue-Änderung |
| 5 | ProductAgent | Features priorisieren, User Stories | Neues Issue |
| 6 | CodingAgent | Code in ATCLang generieren, Bugs fixen | TODO erkannt |
| 7 | QAAgent | Tests in ATCLang generieren, Coverage | Code-Änderung |
| 8 | SecurityAgent | Audits, Kryptografie, Schwachstellen | Jeder Sync |
| 9 | DocumentationAgent | Wiki 62+ Kapitel aktuell halten | Code-Änderung |
| 10 | RepositoryAgent | Issues, Commits, PRs, Releases | Jeder Sync |
| 11 | GovernanceAgent | DAO Proposals, Standards, Richtlinien | Governance-Event |
| 12 | ResearchAgent | Lücken finden, neue Ideen, externe Quellen | Lücken erkannt |

**Orchestriert von: MasterBrain (Aurora · Base44)**

---

## REGEL 12 — GMAIL POLICY

```
✅ Gmail darf SENDEN
❌ Gmail darf NIEMALS lesen / scannen / auswerten
```

Diese Regel gilt absolut und hat keine Ausnahmen.

---

## REGEL 13 — ATCLang MIGRATIONS-POLICY (ATC-99)

### Priorität bei neuem Code

```
1. ATCLang schreiben (immer bevorzugen)
2. Python-Stub nur wenn ATCLang-Feature noch nicht existiert
3. Solidity-Stub nur wenn Smart-Contract-Feature noch nicht existiert
4. Jeden Stub mit Kommentar markieren:
   # STUB: Wird in Sprint X.X durch ATCLang ersetzt
```

### Migrations-Roadmap

| Sprint | Was migriert | Von | Nach |
|--------|-------------|-----|------|
| 2.3 | Consensus (PoH, PoW, PoS, AMM) | Python | ATCLang |
| 2.4 | ~~ShivaOS Kernel + Syscalls: Python → ATCLang~~ **VERALTET** — Kernel ist ShivaCore (Rust no_std, from-scratch), siehe REGEL 0 | — | — |
| 2.5 | Smart Contracts (ATC-89, ATC-90, Bridge) | Solidity | ATCLang |
| 3.0 | Gateway + Backend API + Tests | Python | ATCLang |

---

## REGEL 14 — ECOSYSTEM-KENNZAHLEN (täglich aktuell halten)

| Metrik | Wert | Stand |
|--------|------|-------|
| Wiki-Kapitel | 62 | 2026-06-12 |
| Wiki-Zeilen | ~12.009 | 2026-06-12 |
| Python-Module (Stubs) | 149 | 2026-06-12 |
| Solidity-Contracts (Stubs) | 6 | 2026-06-12 |
| Dateien gesamt | 300 | 2026-06-12 |
| Offene Issues | 6 | 2026-06-12 |
| Standards gesamt | 18 (ATC×15, KIP×1, AIP×1, ATS×1) | 2026-06-12 |
| Audit-Score | 91/100 | 2026-06-12 |
| Verbundene Dienste | 16 | 2026-06-12 |
| Offene Decisions | 6 (AD-006 resolved) | 2026-06-12 |
| EcosystemNodes | 21 | 2026-06-12 |
| EcosystemSprints | 9 | 2026-06-12 |

---

## ANHANG A — Repositories

| Repo | Zweck | URL |
|------|-------|-----|
| a-townchain-os | Quellcode (ATCLang + Stubs) | github.com/A-TownChain-Okosystems/a-townchain-os |
| a-townchain-os-docs | Wiki (62 Kapitel) | github.com/A-TownChain-Okosystems/a-townchain-os-docs |

## ANHANG B — Standards-Übersicht

| Standard | Titel | Status |
|----------|-------|--------|
| ATC-81 | Proof of History (SHA-3_256 VDF) | ✅ ACCEPTED |
| ATC-82 | Proof of Work (SHA-256) | ✅ ACCEPTED |
| ATC-83 | Proof of Stake (Weighted) | ✅ ACCEPTED |
| ATC-84 | Fork Resolution (Nakamoto+PoH) | ✅ ACCEPTED |
| ATC-85 | Initial Sync (Batch) | ✅ ACCEPTED |
| ATC-86 | ECDSA secp256k1 | ✅ ACCEPTED |
| ATC-87 | Gas Fee EIP-1559 | ✅ ACCEPTED |
| ATC-88 | AMM x·y=k | ✅ ACCEPTED |
| ATC-89 | Fungible Token (FFT) | ✅ ACCEPTED |
| ATC-90 | NFT Standard (Shivamon) | ✅ ACCEPTED |
| ATC-91 | Cross-Chain Bridge | 🔄 REVIEW |
| ATC-92 | ATCLang Language Spec | 📝 DRAFT |
| ATC-93 | ATCLang VM Bytecode | 📝 DRAFT |
| ATC-94 | ATCLang Standard Library | 📝 DRAFT |
| ATC-95 | ATCLang Test Framework | 📝 DRAFT |
| ATC-96 | Kernel Interface Protocol | 📝 DRAFT |
| ATC-97 | Agent Interaction Protocol | 📝 DRAFT |
| ATC-98 | Testing Standard | 📝 DRAFT |

## ANHANG C — Verbundene Dienste (16)

| Dienst | Verwendung |
|--------|-----------|
| GitHub | Code + Wiki Repos |
| Notion | Projektdashboard, Tagesprotokoll |
| Gmail | Reports senden (NIE lesen) |
| Outlook | Reports senden |
| Google Tasks | Offene Tasks + Decision-Deadlines |
| Google Drive | Reports + Artefakte speichern |
| Google Sheets | Metriken, Roadmap-Tabellen |
| Google Docs | Technische Dokumente |
| Google Slides | Sprint-Review Präsentationen |
| Google Calendar | Meilensteine, Decision-Deadlines |
| Google Meet | Sprint-Reviews |
| Google Analytics | Dokumentations-Traffic |
| Google BigQuery | Blockchain-Analytics |
| Google Search Console | Docs-SEO |
| Google Classroom | Entwickler-Onboarding |
| Hugging Face | Code-Review Modelle (geplant) |

---

*Erstellt und gepflegt von: Aurora (A-Town Ecosystem Brain · Base44)*
*Autorität: Michael Wroblewski*
*Gültig ab: 2026-06-12 | Nächste Revision: täglich 08:00 Uhr automatisch*
