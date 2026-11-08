# 🤖 Agent Policy — Sync- und Entwicklungs-Protokoll

> **Verbindlich für:** Aurora (A-Town Ecosystem Brain)
> **Version:** 1.0.0 | **Stand:** 2026-06-12
> **Gilt für:** Alle Sync-Läufe, Code-Änderungen, Wiki-Updates, Issue-Erstellungen

> 🧭 **Live-Arbeitsstatus:** Siehe [`AGENT_COORDINATION.md`](AGENT_COORDINATION.md) — wer gerade woran arbeitet, aktuelle Sprints/Plaene, Architektur-Snapshot. Vor Arbeitsbeginn immer zuerst dort pruefen.

---

## OBERSTE REGEL: REALITY-CHECK

Der Agent darf **niemals** behaupten, eine Aktion ausgeführt zu haben,
ohne einen bestätigten API-Response erhalten zu haben.

### Ausführungsstatus (immer explizit angeben)

| Status | Symbol | Bedeutung |
|--------|--------|-----------|
| AUSGEFÜHRT | ✅ | API-Call erfolgreich, Response mit ID/SHA bestätigt |
| VORBEREITET | 🔄 | Content fertig, Push noch ausstehend |
| GEPLANT | 📋 | Logik definiert, kein API-Call gemacht |
| SIMULIERT | 🔲 | Nur lokal berechnet, kein externer Effekt |
| FEHLGESCHLAGEN | ❌ | API-Call gemacht, Fehler-Response erhalten |
| UNKLAR | ⚠️ | Response mehrdeutig, Verifizierung nötig |

### Reality-Check-Checkliste vor jeder Aktion

```
Vor GitHub Push:
  □ Token gültig? (401 = nein)
  □ Repo existiert? (404 = nein)
  □ Datei existiert → SHA vorhanden?
  □ Content tatsächlich geändert?
  □ Response enthält commit.sha?

Vor Issue-Erstellung:
  □ Repo erreichbar?
  □ Kein Duplikat (Titel-Check)?
  □ Response enthält number?

Vor E-Mail-Versand:
  □ Token gültig?
  □ Response enthält id (Gmail) oder HTTP 202 (Outlook)?

Vor Notion-Update:
  □ Token gültig?
  □ Datenbank-ID bekannt?
  □ Response enthält id?
```

---

## CODE ↔ DOKUMENTATION ABGLEICH

Bei **jeder** Änderung — in dieser Reihenfolge:

```
1. Code analysieren      → Was existiert tatsächlich?
2. Architektur prüfen    → Layer-Zuordnung korrekt?
3. Wiki analysieren      → Kapitel vorhanden und aktuell?
4. Roadmap prüfen        → Sprint-Zuordnung stimmt?
5. Standards prüfen      → ATC-Nummer referenziert?
6. Issues prüfen         → Ticket vorhanden?
```

**Sechs Abgleichsfragen (alle müssen "nein" sein vor Release):**

```
□ Dokumentation ohne Implementierung?  → Status: DOCUMENTED (Warnung)
□ Implementierung ohne Dokumentation?  → SOFORT Wiki erstellen (Blocker)
□ Ticket ohne Umsetzung?               → Status: TODO
□ Umsetzung ohne Ticket?               → SOFORT Issue erstellen
□ Roadmap-Eintrag ohne Epic?           → Epic ableiten
□ Epic ohne Roadmap-Eintrag?           → Roadmap aktualisieren
```

---

## WIKI-VALIDIERUNG

Wiki-Status-Lifecycle:
```
DRAFT → REVIEW → APPROVED → PUBLISHED → DEPRECATED
```

Vor PUBLISHED: Alle 5 Checks müssen grün sein:
- [ ] Technisch korrekt (Code stimmt überein)
- [ ] Konsistent mit Code (Klassen/Methoden exakt)
- [ ] Konsistent mit Architektur (Layer stimmt)
- [ ] Konsistent mit Standards (ATC-Nummer vorhanden)
- [ ] Konsistent mit Roadmap (Sprint stimmt)

---

## ISSUE-ERSTELLUNG

**Pflichtfelder für jedes neue Issue:**

```yaml
title: "[Kap. XX] Komponente — Kurzbeschreibung"
description: |
  ## Was
  ## Warum
  ## Akzeptanzkriterien
  - [ ] Kriterium 1 (messbar)
  - [ ] Kriterium 2 (messbar)
  - [ ] Kriterium 3 (messbar)
priority: CRITICAL | HIGH | MEDIUM | LOW
sprint: X.X
wiki_chapter: Kap. XX
standard: ATC-XXXX
depends_on: #XX, #XX
```

**Nur erstellen wenn:**
- GitHub API ✅ erreichbar
- Kein Duplikat ✅ (Titel nicht bereits vorhanden)

---

## RELEASE-BLOCKER

Release ist GESPERRT solange:

```
❌ Audit-Score < 80/100
❌ Kritische Sicherheitslücke offen
❌ Neue Klasse/API ohne Wiki-Kapitel
❌ Neues Modul ohne Tests
❌ Offene CRITICAL/HIGH Blocker-Issues
❌ Standard im DRAFT-Status (nicht APPROVED)
❌ Offene DECISION-Items die den Release betreffen
❌ CI/CD schlägt fehl
❌ Wiki-Kapitel im DRAFT-Status
```

---

## SYNC-REIHENFOLGE (nach jeder Änderung)

```
1. GitHub Code → ✅/❌ mit Commit-SHA
2. GitHub Wiki → ✅/❌ mit Commit-SHA
3. EcosystemNode (Base44) → Zustand aktualisieren
4. Wiki-Status → PUBLISHED setzen
5. Issues → Verlinkungen prüfen/ergänzen
6. Roadmap / Sprint → Status aktualisieren
7. CHANGELOG.md → Eintrag hinzufügen
8. Notion → Tagesprotokoll
9. Google Tasks → Offene Tasks synchronisieren
10. Google Drive → Report hochladen
11. Gmail + Outlook → Status-Report senden
```

Jeden Schritt mit Status ✅/❌ protokollieren.

---

## OFFENE DECISIONS (MENSCH ERFORDERLICH)

Diese Items werden bei **jedem** Sync-Lauf gemeldet bis sie resolved sind:

| ID | Impact | Thema | Status | Quelle |
|----|--------|-------|--------|--------|
| AD-001 | ✅ RESOLVED | SHA-256 (nicht Keccak-256) | Gueltig seit 14.06.2026 | DECISIONS_REGISTER |
| AD-002 | 🟠 MEDIUM | EventBus vs. IPCBus | ⏳ VALIDATE — Michael entscheidet, Sprint 2.4 | DECISIONS_REGISTER |
| AD-003 | ✅ RESOLVED | Flash-Loan Voting Snapshot | Gueltig seit 05.07.2026 | DECISIONS_REGISTER |
| AD-004 | ✅ RESOLVED | Chain-ID 9000 (Non-EVM, kein XDC-Konflikt) | Gueltig seit 14.06.2026 | DECISIONS_REGISTER |
| AD-005 | 🟠 MEDIUM | ATC-97 Agent Interaction Protocol | ⏳ DECISION — Aurora arbeitet Spec aus, Sprint 3.0 | DECISIONS_REGISTER |
| AD-006 | ✅ RESOLVED | Python vs. Substrate → ATCLang First | Gueltig seit 12.06.2026 | DECISIONS_REGISTER |
| AD-007 | ✅ RESOLVED | EVM Chain Registry irrelevant (Non-EVM) | Gueltig seit 14.06.2026 | DECISIONS_REGISTER |
| AD-008 | ✅ RESOLVED | Lizenzmodell ATC-LIC/ATC-LIC etabliert | 05.07.2026 | DECISIONS_REGISTER |
| AD-009 | ✅ RESOLVED | BaFin-Konformitaetsbericht erstellt | 06.07.2026 | DECISIONS_REGISTER |
| AD-010 | ✅ RESOLVED | Proprietary Lizenz auf 24 Repos | 05.07.2026 | DECISIONS_REGISTER |

> **Hinweis:** Nur AD-002 und AD-005 sind aktuell offen. Alle anderen wurden im
> DECISIONS_REGISTER.md als RESOLVED verifiziert — diese Tabelle war zuvor
> veraltet (zeigte AD-004/006/007 faelschlich als CRITICAL/offen mit
> abgelaufenen Juni-Deadlines). Korrigiert am 06.07.2026.

---

*Diese Policy ist verbindlich für Aurora (A-Town Ecosystem Brain)*
*Automatisch durchgesetzt bei jedem Sync-Lauf (täglich 08:00 Uhr)*
*Letzte Aktualisierung: 2026-06-12*

---

## 🔄 Uebergabe-Status fuer andere KI-Agenten (Agent-Handoff)

> Diese Sektion dokumentiert den aktuellen Bearbeitungsstand fuer jeden
> KI-Agenten, der dieses Projekt weiterbearbeitet (z.B. bei Agenten-Wechsel
> oder parallelem Einsatz mehrerer Agenten).

**Zuletzt aktualisiert von:** Aurora (Superagent, Base44) — 06.07.2026

### Aktuell in Bearbeitung
- **BaFin-Compliance Dokumentation** — Status: ✅ Kern-Dokumentation abgeschlossen
  - 8 Kern-Dokumente (Lizenz/Compliance) vollstaendig, cross-referenziert
  - BaFin-Konformitaetsbericht (BAFIN-ATC-LIC-2026-001) als Entwurf v1.0.0 fertig
  - 24/24 Repo-READMEs verlinkt, 24/24 Repos mit proprietaerer LICENSE
  - **Naechster Schritt:** BaFin-Vorab-Abklaerung einreichen (wartet auf Menschen-Freigabe)
  - **Noch offen:** ATVM License Gate Implementation, License Registry Smart Contract,
    IP & License Dashboard (GlobusOS), ATC-LIC Hardware-Integration — siehe TODO.md

### Fuer den naechsten Agenten wichtig
1. **Reality-Check strikt einhalten** — jede Behauptung ueber ausgefuehrte Aktionen
   MUSS durch API-Response (SHA/ID) verifizierbar sein. Nicht einfach als "erledigt"
   annehmen, immer gegenchecken.
2. **DECISIONS_REGISTER.md ist Quelle der Wahrheit** fuer Decision-Status — nicht
   diese Policy-Datei blind uebernehmen, sie kann veralten (siehe Korrektur oben).
3. **Diese Datei existiert in 3 Repos** (a-townchain-os, a-townchain-os-docs,
   kai-os-wiki) — bei Aenderungen alle 3 synchron halten.
4. **BaFin-Compliance-Aufgaben** stehen strukturiert in TASKS.md (lokal) und
   docs/TODO.md (Wiki) — dort weiterarbeiten, nicht neu erfinden.

### K3/K4 Code-Migration — Reality-Check-Ergebnis (06.07.2026)

> **Status: 📋 GEPLANT — NICHT begonnen.** Issues #87 (K3) und #88 (K4) sind
> reine Planung ohne jegliche Umsetzung. Verifiziert per direktem Repo-Scan
> (nicht nur Issue-Status).

**K3 — Python-Backend (10 Repos → `src/`):**
- `src/` Zielverzeichnis existiert **nicht** im Repo
- 0 von 16 K3-Subtasks umgesetzt
- Kein einziger Commit mit K3-Bezug im Verlauf
- Stattdessen liegen `core/`, `blockchain/`, `modules/`, `atclang/`, `backend/`,
  `gateway/`, `shivaos/`, `aistudio/` weiterhin unkonsolidiert nebeneinander
- Kein `setup.py`, kein konsolidiertes Dependency-Management

**K4 — TypeScript Frontend:**
- `frontend/` existiert, aber nur Basis-Struktur (`assets/`, `battle/`, `bootscreen/`)
- 0 von 10 K4-Subtasks umgesetzt
- Kein `package.json`, `vite.config.ts`, `tsconfig.json` — kein Build-System vorhanden
- Kein AI-Studio-Import (190 TS-Dateien laut K4.1 noch nicht migriert)

**Fuer den naechsten Agenten:** Nicht davon ausgehen, dass K3/K4 "in Arbeit"
sind, nur weil Issues existieren. Immer per Reality-Check den tatsaechlichen
Repo-Zustand pruefen (Verzeichnis-Scan), bevor Status behauptet wird. Naechster
konkreter Schritt waere K3.1 (`core/` → `src/core/` migrieren) als kleinster
sinnvoller erster Baustein.

---

### Bekannte Inkonsistenzen (Stand 06.07.2026)
- Sync-Reihenfolge (taeglich 08:00 Uhr) ist als Automatismus beschrieben, aber
  kein aktives Scheduled-Automation dafuer im aktuellen Setup bestaetigt —
  vor Verlass auf Automatik gegenchecken.
- GlobusOS wird nur in ATC-LIC-Kontext erwaehnt, moeglicherweise Rebranding/
  Sub-Komponente von KAI-OS — Terminologie bei neuen Docs klaeren.

---

## 📜 VERBINDLICHES MANDAT — Standards-Einhaltung & Konsolidierungsziel

> **Stand:** 06.07.2026 | **Gilt fuer:** ALLE KI-Agenten, die an diesem Projekt arbeiten

### Pflicht zur Standard-Einhaltung
Jeder Agent (Aurora oder jeder andere KI-Agent, der dieses Projekt bearbeitet)
MUSS folgende Dokumente vor jeder Aenderung konsultieren und einhalten:
- Diese Datei (`AGENT_POLICY.md`) — Sync-Protokoll, Reality-Check, Release-Blocker
- `docs/standards/STANDARDS_REGISTRY.md` — alle ATC-/ATS-Standards (aktuell 37+)
- `docs/DECISIONS_REGISTER.md` — verbindliche Architektur-Entscheidungen (Source of Truth)
- `docs/LICENSING_OVERVIEW.md` — ATC-LIC/ATC-LIC Lizenz-Compliance

Kein Agent darf einen Standard, ein Protokoll oder eine Richtlinie umgehen,
ohne dies explizit als neue offene Decision im DECISIONS_REGISTER zu vermerken.

### Konsolidierungsziel: Zwei Haupt-Repositories
Alle Konzepte, Module und Standards aus den aktuell 24 verteilten Repositories
werden schrittweise (siehe Issues K1–K8, #85–#92) als **lauffaehige Software**
in den **zwei Haupt-Repositories** zusammengefuehrt:

| Haupt-Repo | Rolle | Inhalt |
|-----------|-------|--------|
| **a-townchain-os** | Code / Runtime | Konsolidierter Monorepo-Code: Kernel, Consensus, ATCLang, Contracts, UI, Gateway, Standards-Referenz-Implementierung |
| **a-townchain-os-docs** | Dokumentation / Wiki | Konsolidierte Wiki: alle Standards, Compliance, Roadmap, Decisions, Agent-Policy |

Die uebrigen 22 Repositories bleiben als historische/spezialisierte
Einzel-Module bestehen, werden aber **nicht** mehr als primaere
Entwicklungsziele behandelt — neue Features und Fixes fliessen direkt in die
zwei Haupt-Repos ein. Bestehende Spezial-Repos werden schrittweise in die
Haupt-Repos gemergt (Issue #87 Backend, #88 Frontend).

### Fuer jeden Agenten verbindlich
1. Neue Code-Beitraege → **immer** in `a-townchain-os` (nicht in Einzel-Repos)
2. Neue Dokumentation → **immer** in `a-townchain-os-docs` (nicht in Einzel-Wikis)
3. Vor jeder Aenderung: Standards-Registry + Decisions-Register pruefen
4. Nach jeder Aenderung: Reality-Check-Protokoll (siehe oben) einhalten
5. Abweichungen von Standards nur nach expliziter DAO-/Michael-Freigabe


---

## 🆕 Agent-Log-Eintrag — Aurora (Base44 Superagent) — 06.07.2026, 16:30 UTC+2

**Was ich getan habe (Reality-Check: alle Aktionen per API-Response bestaetigt):**

1. ✅ `AGENT_COORDINATION.md` erstellt — war in README.md und dieser Policy
   verlinkt, existierte aber in keinem der 3 Repos. Jetzt vorhanden mit Live-Status,
   Architektur-Snapshot und Uebergabe-Hinweisen.
2. ✅ Duplikat-Cleanup: 145 redundante Dateien (1.023 KB) aus 3 Repos entfernt,
   Commit-SHAs bestaetigt. `aistudio/temp_repo/` war Hauptquelle der Duplikate.
3. ✅ `NAMING_CONVENTIONS.md` erstellt und in 3 Repos + Notion deployt — 19
   Konzepte geprueft, 13 Inkonsistenzen gefunden (z.B. ATCLang vs ATCLang).
4. ✅ Wiki Quality Scorecard erstellt — Gesamt-Score 77.5/100 (B), schwaechste
   Kategorien: GCL Communication Layer (30/F), GCP Civilization Platform (40/D).
5. ✅ ~750 fehlende Dokumentationsdateien zwischen a-townchain-os-docs und
   den beiden anderen Repos synchronisiert (main + wiki).
6. ✅ OS-Gap-Analyse fuer "Windows-artigen Installer" erstellt — Readiness
   11.8/100. Grundlegende OS-Komponenten (Bootloader, HAL, Memory Manager,
   Installer) fehlen komplett; aktuelles Projekt ist Python/ATCLang-Prototyp
   mit Blockchain-Layer, kein bare-metal-faehiges Betriebssystem.

**Fuer den naechsten Agenten:** Siehe `AGENT_COORDINATION.md` fuer den vollen
Uebergabe-Status inkl. K1-K8-Fortschritt und offene Prioritaeten.
