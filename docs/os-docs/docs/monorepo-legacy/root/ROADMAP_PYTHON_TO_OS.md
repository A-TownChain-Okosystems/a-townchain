# 🎯 A-TownChain OS — Roadmap: Python-App → Installierbares OS
## Die härteste Wahrheit des Projekts

> **Erstellt:** 05.07.2026 21:03 (Europe/Berlin)
> **Autor:** Aurora Agent (Verifiziert via GitHub API, nicht aus Dokumentation übernommen)
> **Zweck:** Ehrlicher Realitäts-Check + belastbarer Fahrplan von "verteilte Python-Codebasis" zu "installierbares Betriebssystem"

---

## 0. Die harte Wahrheit — Doku vs. Realität

Die bestehenden Dokumente (`ROADMAP.md`, `MILESTONES.md`, `SPRINT_ROADMAP.md`) behaupten:
- 78/82 Issues geschlossen (95,1%)
- 10/15 MK-Meilensteine erfüllt
- Sprint 2.1–2.7 "✅ Abgeschlossen"

**Was die GitHub-API tatsächlich zeigt (verifiziert 05.07.2026, 21:03):**

| Fakt | Behauptet | Real (API-geprüft) |
|---|---|---|
| Commit-Historie Hauptrepo | Kontinuierliche Entwicklung | **1 einziger Root-Commit**, keine Historie davor |
| Offene Issues | 4 offen | **12 offen** — davon 8× Konsolidierung (K1–K8), noch nicht begonnen |
| 21 Nebenrepos (atclang, atc-kernel, atc-*, etc.) | Aktiv entwickelt | Letzter Commit jeweils **"chore: add MIT LICENSE"** — Bootstrap-Commits, keine echte Feature-Historie |
| Datenpersistenz | "Sprint 2.x abgeschlossen" | **In-Memory only** — Datenverlust bei jedem Neustart |
| Wallet-Sicherheit (ECDSA) | "Implementiert" | Code existiert, ist aber **nicht in die Wallet-API integriert** — Signaturen werden nicht verifiziert |
| ATC-001 Genesis Token | "Fertig" | **Nicht implementiert** — nur Contract-Gerüst |
| Code Coverage | — | **~45%** (Ziel: 70%) |
| Dependabot-Alerts | — | **18 Sicherheitslücken** (4 High, 10 Moderate, 4 Low) |
| Verwaiste Duplikate | — | 2 private Repos unter `A-TownChain-Okosystems/*` mit kaputtem Umlaut-Encoding, nie aufgeräumt |

**Fazit:** A-TownChain OS ist heute eine **gut dokumentierte, aber fragmentierte Python-Codebasis** — verteilt über 24 Repositories, ohne durchgängig lauffähiges, getestetes, persistentes System. Die K1–K8 Konsolidierungs-Issues (bereits auf GitHub offen) beschreiben exakt den ersten notwendigen Schritt. Alles was in `MILESTONES.md` als "✅ Erfüllt" markiert ist, muss als **unverifiziert** behandelt werden, bis es durch echte Tests + CI/CD + Commit-Historie belegt ist.

Dieser Bericht ersetzt keine Motivation — er ist die Grundlage, um ab jetzt nur noch Fortschritt zu berichten, der durch die GitHub-API nachweisbar ist.

---

## 1. Zielbild: Was heißt "installierbares OS"?

Nicht gemeint ist zwingend ein bare-metal-Betriebssystem. Gemeint ist ein **vollständiges, distributionsfähiges Software-Paket**, das:

1. Aus **einem** Build-Vorgang entsteht (nicht 24 Repos manuell zusammenklicken)
2. Sich per **Installer/Image** auf einer Maschine oder einem Node installieren lässt (Docker-Compose-Bundle, Live-ISO, oder Ein-Klick-Installer-Skript)
3. Beim Start durch den eigenen **Kernel** (`core/kernel.py`, `module_loader.py`) hochfährt, alle Services (Gateway, Chain, Wallet, AI, Game) als Module lädt, und einen **Bootscreen** zeigt
4. Persistente Daten hat, sicher signierte Transaktionen verarbeitet, und einem Node erlaubt, dem P2P-Netzwerk beizutreten
5. Über einen Paketmanager (`atcpkg`) erweiterbar ist

---

## 2. Phasenmodell (4 Phasen, 26 Sprints — gemäß Projektstandard)

Jede Sprint-Karte enthält: **Ziel**, **DoD (Definition of Done)**, **Fehlerbehebung**, **Deployment-Checklist**. Alle Sprints sind mit L0-Security (S1–S6) und dem OS-Kernel (Kapitel 24) rückverknüpft.

---

### PHASE 1 — KONSOLIDIERUNG & FUNDAMENT (Sprints 1.1–1.7)
*Ziel der Phase: Aus 24 Repos wird 1 baubares Repo. MK-Gate 1 am Ende.*

#### Sprint 1.1 — Repository-Audit & Mapping *(= GitHub Issue #85 / K1)*
- **Ziel:** Vollständige Inventarisierung aller 24 Repos: Sprache, Zeilen, Abhängigkeiten, Duplikate.
- **DoD:** Mapping-Tabelle mit Ziel-Pfad im Monorepo für jede Datei; 2 verwaiste `A-TownChain-Okosystems/*`-Repos bewertet (löschen oder migrieren).
- **Fehlerbehebung:** Bei unklaren Abhängigkeiten → `grep -r "import atc"` je Repo, Abhängigkeitsgraph erzeugen.
- **Deployment-Checklist:** [ ] Mapping-Datei committed [ ] Keine Datei ohne Zielpfad [ ] Audit-Report an L0-Security zur Prüfung.

#### Sprint 1.2 — Monorepo-Struktur erstellen *(Issue #86 / K2)*
- **Ziel:** `a-townchain-os` bekommt finale Ordnerstruktur (`src/`, `contracts/`, `kernel/`, `docs/`, `tests/`).
- **DoD:** Struktur existiert, README beschreibt sie, kein Code liegt mehr außerhalb.
- **Fehlerbehebung:** Namenskollisionen zwischen Repos → Präfix-Konvention (`atc_`) durchgängig anwenden.
- **Deployment-Checklist:** [ ] Ordnerstruktur gepusht [ ] CI läuft auf neuer Struktur [ ] Alte Pfade als deprecated markiert.

#### Sprint 1.3 — Python-Backend zusammenführen *(Issue #87 / K3)*
- **Ziel:** 10 Backend-Repos (atc-kernel, atcnet, atc-contracts, atc-gateway, etc.) → `src/`.
- **DoD:** Ein `requirements.txt`, ein Entry-Point (`start.py`), alle Services importierbar ohne Repo-Wechsel.
- **Fehlerbehebung:** Circular Imports → `core/module_loader.py` als zentrale Registry nutzen statt direkter Cross-Imports.
- **Deployment-Checklist:** [ ] `pip install -e .` funktioniert [ ] Alle bisherigen Unit-Tests laufen im neuen Pfad.

#### Sprint 1.4 — TypeScript-Frontend zusammenführen *(Issue #88 / K4)*
- **Ziel:** `atc-ui` + Frontend-Assets → `frontend/`.
- **DoD:** Ein `package.json`, ein Build-Befehl (`npm run build`) erzeugt das komplette UI.
- **Fehlerbehebung:** Bekannter CI-Fehler `npm ci` schlägt fehl → `npm install` nutzen bis Lockfile konsistent ist (siehe MASTER_TODO T-603).
- **Deployment-Checklist:** [ ] Build erzeugt `dist/` [ ] UI spricht Gateway auf Port 4000 an [ ] Kein Hardcoded-Localhost.

#### Sprint 1.5 — Build-System & Docker *(Issue #89 / K5)*
- **Ziel:** Ein-Klick-Build für das gesamte System.
- **DoD:** `docker-compose up` startet Gateway, Core, Chain, Wallet, AI, Game aus einem Repo heraus.
- **Fehlerbehebung:** Port-Konflikte → feste Zuordnung dokumentieren (4000–5004) und in `.env.example` festschreiben.
- **Deployment-Checklist:** [ ] `Dockerfile` je Service [ ] `docker-compose.yml` validiert [ ] Healthchecks für jeden Container.

#### Sprint 1.6 — CI/CD Pipeline *(Issue #90 / K6)*
- **Ziel:** Build → Test → Release vollautomatisch, ohne Branch-Protection-Blocker.
- **DoD:** Grüner Pipeline-Lauf auf `main`, CodeQL + Pages-Deploy funktionieren.
- **Fehlerbehebung:** Bekannter Blocker T-603/T-604/T-605 (Branch-Protection verhindert API-Push) → Michael muss Repo-Einstellung "Require status checks" anpassen oder PR-Flow statt Direct-Push nutzen.
- **Deployment-Checklist:** [ ] `.github/workflows/ci-cd.yml` grün [ ] CodeQL ohne kritische Findings [ ] Pages-Deploy live.

#### Sprint 1.7 — Tests & QA ≥80% Coverage *(Issue #91 / K7)*
- **Ziel:** Reale Testabdeckung statt Stub-Tests.
- **DoD:** Coverage-Report zeigt ≥80% für `src/`, alle Blocker-Module (Wallet, Persistence, Consensus) mit Integrationstests.
- **Fehlerbehebung:** Aktuell 45% Coverage, 15/16 Gateway-Tests skipped wegen fehlendem `create_app()` → zuerst diesen Blocker fixen.
- **Deployment-Checklist:** [ ] `pytest --cov` ≥80% [ ] Keine `@skip`-Tests ohne Ticket-Referenz.

> **🔒 MK-Gate 1 (Ende Phase 1):** Ein Repo, ein Build, CI grün, ≥80% Coverage, L0-Security-Review bestanden (S1 Zero-Trust-Basis). *Erst danach zählt Phase 2 als gestartet — kein Vorgriff.*

---

### PHASE 2 — KERN-FUNKTIONALITÄT (Sprints 2.1–2.7)
*Ziel der Phase: Die App tut wirklich das, was die Doku behauptet. MK-Gate 2 am Ende.*

#### Sprint 2.1 — Datenpersistenz (SQLite → Postgres-fähig)
- **Ziel:** Kein Datenverlust bei Neustart.
- **DoD:** `backend/db/repository.py` vollständig, Migrationen vorhanden, Wallet-/Chain-Daten überleben Restart.
- **Fehlerbehebung:** In-Memory-Fallback nur für Tests, nie für Produktionspfad; Migration-Skript idempotent machen.
- **Deployment-Checklist:** [ ] Migration läuft in CI [ ] Restart-Test (Daten vorhanden nach `docker restart`) [ ] Backup-Skript vorhanden.

#### Sprint 2.2 — Wallet-Security: ECDSA-Integration
- **Ziel:** `/api/wallet/send` verifiziert Signaturen wirklich.
- **DoD:** `ECDSASigner.verify()` in `backend/api/routes/wallet.py` eingebaut, Negativ-Test (gefälschte Signatur wird abgelehnt) grün.
- **Fehlerbehebung:** Aktuell akzeptiert Endpoint jede Anfrage — höchstes Sicherheitsrisiko, vor Sprint 2.3 zwingend zu schließen.
- **Deployment-Checklist:** [ ] Penetrationstest mit gefälschter Signatur schlägt fehl (erwartet) [ ] Audit-Log für abgelehnte Signaturen.

#### Sprint 2.3 — ATC-001 Genesis Token produktionsreif
- **Ziel:** `mint()`, `transfer()`, `balance()` echt implementiert und mit Shivamon-Contract verbunden.
- **DoD:** Marktplatz kann Token minten/transferieren, Tests decken Edge-Cases (Overflow, Negativbeträge) ab.
- **Fehlerbehebung:** Contract-Gerüst ohne Logik → zuerst Unit-Tests schreiben (TDD), dann Logik implementieren.
- **Deployment-Checklist:** [ ] Testnet-Deploy erfolgreich [ ] Marketplace-Smoke-Test grün.

#### Sprint 2.4 — P2P-Netzwerk Ende-zu-Ende
- **Ziel:** Discovery + Propagation zwischen ≥3 echten Nodes nachweisbar.
- **DoD:** `test_discovery.py` + `test_p2p_propagation.py` laufen gegen echte (nicht gemockte) Nodes.
- **Fehlerbehebung:** Sharding-Tests aktuell nur lokal simuliert → Multi-Container-Testnetz in CI aufbauen.
- **Deployment-Checklist:** [ ] 3-Node-Devnet startet automatisiert [ ] Nachrichten propagieren nachweisbar (Log-Trace).

#### Sprint 2.5 — Konsens (PoH/PoS-Hybrid) produktionsreif
- **Ziel:** Fork-Resolution funktioniert unter Last.
- **DoD:** Chaos-Test (simulierter Netzwerk-Split) löst sich korrekt auf.
- **Fehlerbehebung:** ATC-1000 (PoH) als kritischer Blocker markiert seit Wochen → dedizierte Sprint-Zeit ohne Ablenkung durch Doku-Arbeit einplanen.
- **Deployment-Checklist:** [ ] Fork-Test-Suite grün [ ] Konsens-Metriken (Blockzeit, Finalität) dokumentiert.

#### Sprint 2.6 — Smart-Contract-Registry & Deployment-Pipeline
- **Ziel:** Contracts lassen sich versioniert deployen, nicht nur lokal ausführen.
- **DoD:** Registry trackt Contract-Adressen + Versionen, Rollback möglich.
- **Fehlerbehebung:** Doppel-Deployments vermeiden → Idempotenz-Check vor jedem Deploy.
- **Deployment-Checklist:** [ ] Deploy-Skript versioniert [ ] Rollback getestet.

#### Sprint 2.7 — API-Gateway produktionsreif
- **Ziel:** Echte Endpoints statt Stubs, Rate-Limiting, Auth.
- **DoD:** Gateway blockt unautorisierte Requests, Rate-Limit greift nachweisbar.
- **Fehlerbehebung:** `create_app()`-Bug (blockiert 15/16 Tests) hier final fixen, falls nicht schon in 1.7 erledigt.
- **Deployment-Checklist:** [ ] Auth-Middleware aktiv [ ] Load-Test mit Rate-Limit-Verifikation.

> **🔒 MK-Gate 2 (Ende Phase 2):** Wallet sicher, Daten persistent, Konsens beweisbar stabil, Token-Wirtschaft funktionsfähig. *Das ist der Punkt, an dem "Python-App" zu "echtem Blockchain-Backend" wird.*

---

### PHASE 3 — SYSTEM-INTEGRATION & OS-KERNEL (Sprints 3.1–3.7)
*Ziel der Phase: Aus dem Backend wird ein bootendes System. MK-Gate 3 am Ende.*

#### Sprint 3.1 — OS-Kernel-Kopplung
- **Ziel:** Alle Services (Gateway, Chain, Wallet, AI, Game) laufen als Module unter `core/kernel.py` + `module_loader.py`, nicht als unabhängige Prozesse.
- **DoD:** Ein Kernel-Start bootet alle Module in definierter Reihenfolge, Event-Bus (`core/event_bus.py`) verbindet sie.
- **Fehlerbehebung:** Modul-Ladefehler → Abhängigkeitsreihenfolge im `module_loader.py` explizit definieren (kein impliziter Zufall).
- **Deployment-Checklist:** [ ] Kernel-Boot-Log zeigt alle Module geladen [ ] Fehlschlag eines Moduls stoppt nicht den ganzen Kernel (Fault-Isolation).

#### Sprint 3.2 — Bootscreen & Init-System
- **Ziel:** Echter Boot-Vorgang mit visuellem Feedback (`frontend/bootscreen/bootscreen_complete.py`).
- **DoD:** System zeigt Boot-Fortschritt, Fehler werden im Bootscreen sichtbar statt nur im Log.
- **Fehlerbehebung:** Boot-Hänger → Timeout je Modul-Start definieren, sonst hängt der ganze Boot.
- **Deployment-Checklist:** [ ] Bootscreen zeigt alle Phasen [ ] Timeout-Handling getestet.

#### Sprint 3.3 — Security-Hardening (L0, S1–S6)
- **Ziel:** Die 18 offenen Dependabot-Alerts (4 High) schließen, Zero-Trust-Review abschließen.
- **DoD:** 0 High/Critical-Alerts, Pen-Test-Report ohne kritische Findings.
- **Fehlerbehebung:** Dependency-Updates einzeln testen (nicht Bulk-Update), um Breaking Changes zu isolieren.
- **Deployment-Checklist:** [ ] Dependabot-Dashboard grün [ ] S1–S6 Security-Domains einzeln abgezeichnet.

#### Sprint 3.4 — Paketmanager (atcpkg)
- **Ziel:** Module lassen sich unabhängig installieren/aktualisieren.
- **DoD:** `atcpkg install <modul>` löst Abhängigkeiten korrekt auf.
- **Fehlerbehebung:** Versionskonflikte → Semver-Constraint-Resolver einbauen statt "neueste Version gewinnt".
- **Deployment-Checklist:** [ ] Install/Uninstall-Zyklus getestet [ ] Abhängigkeitsgraph zirkelfrei.

#### Sprint 3.5 — DevNet & Multi-Node-Testnet
- **Ziel:** Echtes Mehr-Node-Testnetz (≥5 Nodes), nicht nur simuliert.
- **DoD:** Testnet läuft ≥24h stabil, Block-Produktion ohne manuelles Eingreifen.
- **Fehlerbehebung:** Node-Drift → Zeitsynchronisation (NTP) zwischen Nodes erzwingen.
- **Deployment-Checklist:** [ ] 24h-Stabilitätstest-Log [ ] Node-Join/Leave ohne Netz-Unterbrechung.

#### Sprint 3.6 — Performance & Sharding
- **Ziel:** Lasttests mit realistischem Transaktionsvolumen.
- **DoD:** Definierte TPS-Zielmarke erreicht und dokumentiert (Benchmark-Report).
- **Fehlerbehebung:** Bottlenecks per Profiling identifizieren, nicht per Vermutung.
- **Deployment-Checklist:** [ ] Benchmark-Report vorhanden [ ] Sharding-Test unter Last grün.

#### Sprint 3.7 — Security-Audit-Abschluss + Doku v2
- **Ziel:** Externer/interner Security-Audit abgeschlossen (MK8), Doku auf Systemstand aktualisiert.
- **DoD:** Audit-Report ohne offene Critical-Findings, Doku widerspricht nicht mehr der GitHub-Realität (siehe Abschnitt 0).
- **Fehlerbehebung:** Bei Audit-Findings → Fix-Sprint einschieben, kein Gate-Durchbruch ohne Abzeichnung.
- **Deployment-Checklist:** [ ] Audit-Report signiert [ ] Doku-Diff gegen echten Repo-Stand geprüft.

> **🔒 MK-Gate 3 (Ende Phase 3):** System bootet als Ganzes, ist sicherheitsgeprüft, läuft stabil im Mehr-Node-Betrieb. *Das ist der Punkt, an dem aus dem Backend ein "Betriebssystem" wird.*

---

### PHASE 4 — DISTRIBUTION & INSTALLIERBARES OS (Sprints 4.1–4.5)
*Ziel der Phase: Jemand Drittes kann es installieren, ohne den Quellcode zu verstehen. MK-Gate 4 = Release.*

#### Sprint 4.1 — Installer-Image-Pipeline
- **Ziel:** Automatisierter Build eines Installer-Artefakts (Docker-Bundle, Live-ISO, oder Ein-Klick-Skript — Entscheidung hier treffen).
- **DoD:** `make image` (o.ä.) erzeugt reproduzierbar ein installierbares Artefakt.
- **Fehlerbehebung:** Nicht-reproduzierbare Builds → Build-Umgebung containerisieren (kein "funktioniert nur auf meinem Rechner").
- **Deployment-Checklist:** [ ] Build ist reproduzierbar (Hash-Vergleich zweier Builds) [ ] Artefakt signiert.

#### Sprint 4.2 — Installer-UX
- **Ziel:** Setup-Wizard für Erstinstallation (Konfiguration, Wallet-Erstellung, Node-Typ-Wahl).
- **DoD:** Unbedarfter Nutzer kann von Null zu laufendem Node in <15 Minuten ohne CLI-Kenntnisse.
- **Fehlerbehebung:** Unklare Fehlermeldungen im Installer → jede Fehlermeldung mit konkretem Lösungsschritt versehen.
- **Deployment-Checklist:** [ ] Usability-Test mit unbeteiligter Person [ ] Rollback bei fehlgeschlagener Installation.

#### Sprint 4.3 — Genesis-Block & Mainnet-Launch (MK9)
- **Ziel:** Chain-ID final, Genesis-Block signiert, Mainnet live.
- **DoD:** Mainnet-Readiness-Checklist (bestehende 55 Punkte) zu 100% erfüllt und einzeln nachweisbar (nicht pauschal abgehakt).
- **Fehlerbehebung:** Genesis-Fehler sind irreversibel → Dry-Run auf Testnet mit identischer Konfiguration zwingend vor Mainnet.
- **Deployment-Checklist:** [ ] 55-Punkte-Checklist einzeln abgezeichnet [ ] Genesis-Dry-Run erfolgreich [ ] Rollback-Plan für die ersten 48h existiert.

#### Sprint 4.4 — Multi-Chain-Support (MK11, optional)
- **Ziel:** Bridge zu ≥1 externer Chain funktionsfähig.
- **DoD:** Bridge-Contract getestet mit echten Testnet-Transaktionen auf beiden Seiten.
- **Fehlerbehebung:** Bridge-Exploits sind das häufigste Blockchain-Sicherheitsrisiko → externes Audit speziell für Bridge-Code.
- **Deployment-Checklist:** [ ] Bridge-Audit bestanden [ ] Zwei-Wege-Test (hin und zurück) erfolgreich.

#### Sprint 4.5 — Release v1.0
- **Ziel:** Signierte Installer-Images öffentlich verfügbar, Update-Mechanismus funktioniert.
- **DoD:** Ein Nutzer kann von der Release-Seite laden, installieren, und bekommt spätere Updates automatisch.
- **Fehlerbehebung:** Update-Bricking vermeiden → Update-Pfad immer mit vorheriger Version testen, nie nur "frisch installiert".
- **Deployment-Checklist:** [ ] Release-Notes vollständig [ ] Update von v0.x auf v1.0 getestet [ ] Downgrade-Pfad dokumentiert (auch wenn nicht empfohlen).

> **🔒 MK-Gate 4 (Release):** Installierbares OS, öffentlich verfügbar, mit funktionierendem Update-Mechanismus. *Ab hier heißt das Projekt nicht mehr "Python-App", sondern "A-TownChain OS v1.0".*

---

## 3. Sofort-Prioritäten (diese Woche)

1. **K1–K8 Issues (#85–#92)** sind bereits auf GitHub offen — das ist Phase 1 in Reinform. Hier anfangen, nicht bei Phase 3/4-Themen.
2. **Wallet-ECDSA-Fix** (Sprint 2.2) — höchstes Sicherheitsrisiko, sollte parallel zu Phase 1 laufen, nicht bis Phase 2 warten.
3. **Dependabot 18 Alerts** — sollten so früh wie möglich geschlossen werden, unabhängig von der Sprint-Reihenfolge.
4. **Doku-Ehrlichkeit:** `MILESTONES.md` und `ROADMAP.md` sollten mit einem Hinweis versehen werden, dass Status-Angaben bis zum Abschluss von Sprint 3.7 als "unverifiziert" gelten.

---

## 4. L0–L12 Layer-Bezug

| Layer | Betroffen in Phase |
|---|---|
| L0 Security (S1–S6) | Alle Phasen, cross-cutting — insbesondere Sprint 2.2, 3.3 |
| L1–L10 (Hardware→dApp) | Phase 2–3 (Kern-Funktionalität + Kernel-Integration) |
| L11 DeFi | Phase 2 (Token/Contracts), Phase 4 (Bridge) |
| L12 Gamification | Phase 2–3 (Shivamon-Contract-Integration) |

Jede Layer-Änderung in Phase 2–4 benötigt laut Standing-Instruction ein L0-Security-Sign-off, bevor der jeweilige Sprint als "Done" gilt.

---

*Dieser Bericht basiert auf verifizierten GitHub-API-Daten vom 05.07.2026, 21:03 Uhr — nicht auf vorherigen Automations-Zusammenfassungen, die sich teilweise als nicht durch die Repo-Historie gedeckt herausgestellt haben.*
