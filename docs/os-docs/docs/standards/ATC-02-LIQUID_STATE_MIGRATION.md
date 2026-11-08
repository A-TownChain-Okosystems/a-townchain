# ATC-02 — Liquid State Migration & Failover Mechanics
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.2 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-02
> **Referenzen:** ATC-01 (Core Node Protocol), ATC-98 (Kernel), Issue #4 (Persistenz), Issue #50 (AI Kernel)
> **Quelldatei:** Atc-02.docx (ursprüngliche Spezifikation)
> **Kategorie:** P2P Networking  
> **Tier:** Tier 1 — Blockchain Infrastructure  

---

## Abstract

ATC-02 ist die essenzielle Ergänzung zu den Netzwerkgrundlagen. Während ATC-01
die Konnektivität (die Verbindung) garantiert, sorgt ATC-02 für die Resilienz
(Widerstandsfähigkeit) und die flüssige Zustandsverwaltung des gesamten
KAI-OS-Ökosystems.

> **Merksatz:** ATC-01 stellt sicher, dass man sich "unterhalten" kann.
> ATC-02 stellt sicher, dass bei einem Abbruch der Unterhaltung das "Gedächtnis"
> der Aufgabe nicht gelöscht wird.

---

## 1. Kernkonzept: Was ist "Liquid State"?

In einem klassischen System sind Daten an den Speicher eines bestimmten
physischen Geräts gebunden. ATC-02 bricht diese Bindung auf:

### 1.1 State-Snapshotting
Jeder Node erzeugt in extrem kurzen Intervallen Snapshots seines aktuellen
Arbeitsspeichers und der laufenden Aufgaben (z. B. einer KI-Inferenz-Berechnung).

**Implementation:**
- `backend/api/orchestrator/orchestrator.py` — Task-Status-Tracking
  (PENDING → RUNNING → DONE/FAILED/TIMEOUT)
- `backend/db/migrate.py` — In-Memory → SQLite Migration (Issue #4)
- `backend/db/repository.py` — Persistente State-Speicherung

> **Status:** Task-States werden getrackt, aber echtes Memory-Snapshotting
> (Serialisierung des vollen Arbeitsspeichers) ist noch nicht implementiert.
> Aktuelle Persistenz erfolgt über SQLite als Zwischenspeicher.

### 1.2 Netzwerk-Verankerung
Diese Snapshots werden im Netzwerk verankert. So wissen andere Nodes im Mesh:
"Node A befindet sich gerade bei Status X".

**Implementation:**
- `core/event_bus.py` — Event-Bus für Status-Propagation
- ATCNet Gossip Protocol — für Netzwerkweite State-Verteilung
- `orchestrator.py` — `get_status()` liefert aktuellen Node-Status

> **Status:** Event-Bus propagiert Status-Änderungen innerhalb des Nodes.
> Netzwerkweite Snapshot-Verankerung über ATCNet ist konzipiert aber noch
> nicht voll implementiert.

### 1.3 Resume-Point
Wenn ein Inferenz-Node ausfällt oder überlastet ist, kann ein Ersatz-Node
den letzten Snapshot vom Netzwerk laden und die Arbeit exakt an diesem Punkt
wieder aufnehmen, anstatt bei Null zu beginnen.

**Implementation:**
- `orchestrator.py` — Circuit-Breaker erkennt Node-Ausfall
- Task-Warteschlange ermöglicht Wiederaufnahme nach Fehler
- `modules/kernel/ai_kernel/ai_kernel.py` — KI-Inferenz mit Status-Tracking

> **Status:** Circuit-Breaker und Task-Retry sind implementiert. Die
> nahtlose Migration des genauen Berechnungs-States auf einen anderen Node
> ist ein Zukunftsfeature.

---

## 2. Warum ATC-02 für KAI-OS entscheidend ist

Ohne diesen Standard wäre das KAI-OS extrem anfällig für "State-Loss"
(Zustandsverlust):

### 2.1 Vermeidung von Datenverlust
Wenn ein Node während einer rechenintensiven KI-Aufgabe ausfällt, geht ohne
ATC-02 die gesamte bisher geleistete Arbeit verloren. Mit ATC-02 wird die
Berechnung auf einem anderen Node nahtlos fortgesetzt.

**Implementation:**
- `orchestrator.py` — Task-Status (PENDING, RUNNING, DONE, FAILED, TIMEOUT)
- Fehlgeschlagene Tasks werden im Status `FAILED` erfasst
- `ai_kernel.py` — On-Chain Logging aller KI-Entscheidungen (Hashes)

### 2.2 Lastausgleich (Load-Balancing)
ATC-02 dient als Basis für den Load-Balancer. Wenn ein Node überlastet ist,
können Aufgaben "flüssig" auf einen unterlasteten Node migriert werden, ohne
die Transaktionslogik zu unterbrechen.

**Implementation:** ✅ Implementiert
- `orchestrator.py` — Load-Balancing mit Task-Routing
- Service-Registry: Blockchain, Wallet, AI, Game, Governance, Marketplace, Nodes
- Circuit-Breaker schützt vor Überlastung
- Dispatch-API: `/dispatch` route Tasks zu verfügbaren Services

```python
class TaskType(IntEnum):
    BLOCKCHAIN  = auto()
    WALLET      = auto()
    AI          = auto()
    GAME        = auto()
    GOVERNANCE  = auto()
    MARKETPLACE = auto()
    NODES       = auto()
    SYSTEM      = auto()
```

### 2.3 Wartungsfähigkeit
Nodes können für Software-Updates aus dem Netzwerk genommen werden, indem sie
ihren Zustand an andere Nodes "übergeben", ohne dass der Endnutzer einen
Service-Abbruch bemerkt.

**Implementation:**
- `orchestrator.py` — Graceful Shutdown mit Status-Export
- `backend/db/migrate.py` — State-Migration (In-Memory → SQLite)
- `config/settings.json` — Node-Rollen-Konfiguration (full, light, validator)

> **Status:** DB-Migration implementiert. Live-Node-Handoff (Zustand an
> anderen Node übergeben während Betrieb) ist Zukunftsfeature.

---

## 3. Einordnung in die technische Architektur

ATC-02 agiert als "Schmiermittel" zwischen den Schichten:

### 3.1 Tier 1 (Netzwerk)
Stellt sicher, dass die Snapshots über das Mesh-Netzwerk übertragen werden
können.

**Implementation:** ATCNet Gossip Protocol (ATC-01) propagiert Nachrichten
an alle Peers. State-Snapshots würden als spezieller Nachrichtentyp
(`STATE_SNAPSHOT`) über das gleiche Protokoll verteilt werden.

### 3.2 Tier 3 (Kernel/Infrastruktur)
Hier wird der Snapshot-Mechanismus technisch implementiert, indem der
Arbeitsspeicher-Zustand serialisiert wird.

**Implementation:**
- `core/kernel.py` — ShivaOS Kernel
- `core/event_bus.py` — Event-Bus für interne State-Propagation
- `backend/db/repository.py` — Serialisierung in SQLite
- `backend/db/migrate.py` — State-Migration

### 3.3 Tier 4 (KI-Inferenz)
Hier entfaltet ATC-02 seinen größten Nutzen, da KI-Berechnungen (wie die von
KAI) oft minutenlange Vorab-Berechnungen erfordern, die nicht bei jedem
kleinen Fehler erneut gestartet werden sollten.

**Implementation:** ✅ Teilweise implementiert
- `modules/kernel/ai_kernel/ai_kernel.py` (v3.2.1, 14.316 chars)
  - **LLMRouter** — HuggingFace Inference API (4 Modelle)
  - **ReasoningEngine** — Neurosymbolisches Reasoning (async)
  - **DecisionEngine** — Autonome Entscheidungen mit Constitutional AI
  - **AIKernel** — Haupt-Entry-Point für alle KI-OS Module
  - **On-Chain Logging** — Hashes aller Entscheidungen für Auditierbarkeit

> KI-Entscheidungen werden bereits mit Hashes on-chain geloggt. Die
> Wiederaufnahme einer abgebrochenen Inferenz an einem Resume-Point
> ist das nächste Entwicklungsziel.

---

## 4. Spezifikation vs. Implementation — Zusammenfassung

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| State-Snapshotting | Memory-Snapshots in kurzen Intervallen | Task-Status-Tracking + SQLite | ⚠️ Teilweise |
| Netzwerk-Verankerung | Snapshots im Mesh verankern | Event-Bus (lokal), Gossip (konzipiert) | ⚠️ Teilweise |
| Resume-Point | State auf anderem Node fortsetzen | Circuit-Breaker + Task-Retry | ⚠️ Teilweise |
| Load-Balancing | Aufgaben flüssig migrieren | Orchestrator Load-Balancer ✅ | ✅ Implementiert |
| Wartungsfähigkeit | Zustand übergeben bei Updates | DB-Migration + Graceful Shutdown | ⚠️ Teilweise |
| KI-Inferenz-Schutz | Minutenlange Berechnungen schützen | AI-Kernel mit On-Chain Logging | ⚠️ Teilweise |
| Circuit-Breaker | (implizit) | ✅ In Orchestrator | ✅ Implementiert |
| Event-Bus | (implizit) | ✅ In core/event_bus.py | ✅ Implementiert |

> **Fazit:** Das Load-Balancing und der Circuit-Breaker sind voll implementiert.
> Das echte "Liquid State" Konzept (Memory-Snapshotting + Netzwerk-Verankerung
> + Resume-Point auf fremdem Node) ist konzeptionell final und teilweise
> implementiert (DB-Persistenz, Event-Bus), aber die nahtlose Node-übergreifende
> State-Migration ist ein Zukunftsfeature.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #4 | Persistenz (In-Memory → SQLite) | ✅ Done | ATC-02 State-Persistenz |
| #2 | Gemini AI Integration | ✅ Done | ATC-02 KI-Inferenz |
| #50 | AI Kernel | ✅ Done | ATC-02 KI-Inferenz-Schutz |
| #60 | IPC Integration | ✅ Done | ATC-02 Kernel-State |
| #69 | Security-Audit | 🟡 Open | ATC-02 State-Migration-Sicherheit |
| Sprint 2.2 | Liquid State Migration | 📐 Geplant | ATC-02 Vollimplementation |

---

## 6. Verbesserungsvorschläge (Zukunft)

- [ ] `STATE_SNAPSHOT` Nachrichtentyp in ATCNet definieren
- [ ] Memory-State Serialisierung (pickle/JSON) im Kernel
- [ ] Netzwerkweite Snapshot-Verankerung über Gossip Protocol
- [ ] Resume-Point-Mechanismus: State von fremdem Node laden + fortsetzen
- [ ] Live-Node-Handoff bei Software-Updates (Zero-Downtime)
- [ ] Snapshot-Intervall konfigurierbar machen (config/settings.json)
- [ ] Checkpoint-Rollback bei fehlerhafter State-Migration
- [ ] Sicherheitskonzept: Snapshots vor Manipulation schützen (Hash-Chain)

---

*Dieses Dokument wurde aus der ursprünglichen Atc-02.docx Spezifikation
abgeleitet und mit der tatsächlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:17 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| migrate_state | 10000 |
| verify_checkpoint | 500 |
| rollback | 5000 |

### Sprint-Zuweisung

- **Sprint 2.6** — #78
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

5+ Unit-Tests: State-Migration, Checkpoint-Verify, Rollback, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
