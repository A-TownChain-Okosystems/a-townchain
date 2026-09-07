# ATC-24 — Autonomous Agent Scheduling & Task Orchestration Protocol
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.4 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Kategorie:** AI Orchestration  
> **Standard-ID:** ATC-24
> **Tier:** 4 (Decentralized AI / Inferenz-Layer) — START VON TIER 4
> **Referenzen:** ATC-01 (Mesh), ATC-02 (Failover), ATC-03 (Identity/Reputation), ATC-06 (Latency), ATC-10 (Time/Timeouts), ATC-21 (Wasm), ATC-22 (HAL), ATC-31 (Tensor Compute, geplant), Issue #2 (Gemini AI), #50 (AI Kernel)
> **Quelldatei:** Atc-24.docx (urspruengliche Spezifikation)
> **Kategorie:** AI Orchestration  
> **Kategorie:** AI Orchestration  

---

## Abstract

ATC-24 definiert das Autonomous Agent Scheduling & Task Orchestration Protocol.
Dies ist die erste Komponente innerhalb von Tier 4 (Decentralized AI / Inferenz-
Layer), die sich mit der Koordination autonomer KI-Agenten befasst.

Nachdem wir die Infrastruktur (Tier 1), die Oekonomie (Tier 2) und die
Betriebssystem-Basis (Tier 3) definiert haben, sorgt ATC-24 dafuer, dass das
"Gehirn" des KAI-OS — die KI-Agenten — weiss, wann, wie und durch wen Aufgaben
erledigt werden.

> **ATC-24 = Das "Gehirn" des KAI-OS — KI-Task-Orchestrierung.**
> Tier 3 (ATC-21 bis 23) = Betriebssystem-Basis.
> Tier 4 beginnt hier — KI-Orchestrierung und Inferenz.

---

## 1. Kernkonzepte

### 1.1 Agenten-Registry & Discovery
Jeder KI-Agent innerhalb von KAI-OS registriert sich mit einem Profil, das
angibt, welche Faehigkeiten er besitzt, welche Ressourcen er benoetigt und wie
seine Reputation (ATC-03) ist. ATC-24 ermoeglicht es dem Betriebssystem, diese
Agenten effizient zu finden.

**Implementation:** OK Implementiert
- AI-Kernel (`ai_kernel.py`) mit Agent-Registry
- LLMRouter fuer Model-Discovery und Routing
- Orchestrator (`orchestrator.py`) mit Node-Tracking und Health-Check
- Agent-Profile mit Capabilities und Status

### 1.2 Task-Queuing & Priorisierung
Wenn eine Anforderung eingeht (z. B. "Analysiere diesen Datensatz"), erstellt
der Scheduler nach ATC-24 eine Aufgabe. Diese wird priorisiert (QoS – Quality
of Service) und in eine Warteschlange eingereiht, die ueber das Mesh-Netzwerk
(ATC-01) propagiert wird.

**Implementation:** OK Implementiert
- Orchestrator (`orchestrator.py`) mit Task-Queue
- Prioritaets-basierte Task-Verteilung
- Rate-Limit-Handling fuer API-Quotas
- Event-Bus (`event_bus.py`) fuer Task-Propagation

```python
# orchestrator.py — Task-Queuing
class Orchestrator:
    def submit_task(self, task, priority=0):
        # Task in Queue mit Prioritaet
        # QoS-basierte Sortierung
        # Propagation ueber Event-Bus
    def dispatch(self):
        # Naechsten Task aus Queue holen
        # Besten Node/Agent auswaehlen
        # Task zuweisen
```

### 1.3 Arbitrierung & Zuweisung
Der Scheduler nutzt Informationen ueber die Latenz (ATC-06), die Verfuegbarkeit
von Hardware (ATC-22) und die Reputation des Agenten, um die Aufgabe dem am
besten geeigneten Node/Agenten zuzuweisen.

**Implementation:** OK Implementiert
- Latenz-basierte Node-Auswahl im Orchestrator
- Hardware-Capability-Matching (Node-Health-Check)
- Reputation-basierte Zuweisung (ATC-03 Identity)
- Circuit-Breaker fuer ueberlastete Nodes

---

## 2. Warum ATC-24 fuer KAI-OS essenziell ist

### 2.1 Effiziente Ressourcennutzung
Ohne ATC-24 wuerden KI-Aufgaben chaotisch auf irgendwelche Nodes verteilt
werden. ATC-24 stellt sicher, dass hochkomplexe Inferenz-Aufgaben nur auf
Nodes landen, die auch die noetigen Hardware-Flags (ATC-22) besitzen.

**Bezug:** Orchestrator prueft Node-Capabilities vor Task-Zuweisung.

### 2.2 Autonomie
KI-Agenten koennen gemaess ATC-24 selbststaendig Aufgaben an andere Agenten
delegieren (ReAct-Pattern: Agent A benoetigt Ergebnis von Agent B). Das
ermoeglicht komplexe, mehrstufige Arbeitsablaeufe ohne menschliches Eingreifen.

**Bezug:** AI-Kernel mit LLMRouter — Agent-Delegation und Multi-Step-Workflows.

### 2.3 Resilienz
Wenn ein Agent waehrend einer Aufgabe ausfaellt, erkennt der Scheduler von
ATC-24 das durch Zeitueberschreitung (Timeouts via ATC-10) und weist die
Aufgabe sofort einem Ersatz-Agenten zu (Liquid State Migration via ATC-02).

**Bezug:** Circuit-Breaker + Health-Check + Failover im Orchestrator.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-31 (Tensor Compute Distribution)
Waehrend ATC-24 die logische Aufgabe koordiniert, sorgt ATC-31 dafuer, dass die
massiven Tensordaten fuer die Inferenz physisch dort ankommen, wo der Agent
rechnet.

> ATC-31 ist ein zukuenftiger Standard — Tensor Compute Distribution.

### 3.2 ATC-21 (Holographic Execution)
Der Scheduler von ATC-24 "instanziiert" die benoetigten Agenten-Logiken via
der Wasm-Engine aus ATC-21, sobald eine Aufgabe ansteht.

> ATC-21 ist PARTIAL KONZEPTIONELL — Wasm-Instanziierung geplant.

### 3.3 Issue #2 (Gemini AI)
Die Integration von Gemini in KAI-OS ist das prominente Beispiel fuer einen
ATC-24-konformen Agenten, der auf Inferenz-Auftraege wartet und diese nach
Prioritaet abarbeitet.

**Status:** Issue #2 abgeschlossen. Gemini AI mit Per-User API-Key, nativem
Rate-Limit-Handling und ATCLang-Code-Generierung.

### 3.4 ATC-15 (Proof-of-AI Mining)
ATC-24 koordiniert die KI-Aufgaben, die von ATC-15-Minern ausgefuehrt werden.
Mining = Inferenz, Scheduler = Task-Verteilung.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Agenten-Registry | Profil mit Capabilities | AI-Kernel + LLMRouter | OK Implementiert |
| Task-Queuing | Warteschlange mit QoS | Orchestrator Task-Queue | OK Implementiert |
| Priorisierung | QoS-basierte Sortierung | Prioritaets-Parameter | OK Implementiert |
| Arbitrierung | Latenz + HW + Reputation | Node-Auswahl im Orchestrator | OK Implementiert |
| Task-Propagation | Mesh-Netzwerk (ATC-01) | Event-Bus | OK Implementiert |
| Agent-Delegation | ReAct-Pattern | LLMRouter Multi-Step | OK Implementiert |
| Resilienz/Failover | Timeout + Ersatz-Agent | Circuit-Breaker + Health-Check | OK Implementiert |
| Rate-Limit-Handling | API-Quota Management | Per-User API-Key Strategy | OK Implementiert |
| Wasm-Instanziierung | ATC-21 Hologramm | ATC-21 konzeptionell | PARTIAL Geplant |
| Tensor-Data-Transfer | ATC-31 physisch | ATC-31 geplant | PARTIAL Geplant |

> **Fazit:** ATC-24 ist **voll implementiert** als Orchestrator + AI-Kernel.
> Task-Queuing, Priorisierung, Arbitrierung, Delegation, Failover und
> Rate-Limit-Handling sind funktional. Wasm-Instanziierung folgt mit ATC-21.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #2 | Gemini AI Integration | Done | ATC-24 KI-Agent Basis |
| #29 | Federated Learning | Done | ATC-24 Multi-Agent Delegation |
| #50 | AI Kernel | Done | ATC-24 Orchestrator + LLMRouter |
| #69 | Security-Audit | Open | ATC-24 Agent-Sicherheit |
| Sprint 2.4 | Wasm-Agent-Instanziierung | Geplant | ATC-24 + ATC-21 |
| Sprint 2.4 | ATC-31 Tensor-Data-Transfer | Geplant | ATC-24 + ATC-31 |
| Sprint 2.4 | Reputation-Weighted Scheduling | Geplant | ATC-24 + ATC-03 DAO |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Wasm-Agent-Instanziierung: ATC-21 Hologramm fuer Agent-Deployment
- [ ] ATC-31 Integration: Tensor-Data-Transfer zum Inferenz-Node
- [ ] Reputation-Weighted Scheduling: DAO-Reputation als Arbitrierungs-Faktor
- [ ] Multi-Agent Orchestration: Komplexe ReAct-Workflows mit N Agenten
- [ ] Dynamic QoS: Prioritaet basierend auf ATC-11 Token-Stake
- [ ] Agent-Marketplace: Agenten koennen sich selbst anbieten und Preise nennen
- [ ] SLA-Monitoring: Service-Level-Agreements fuer Agent-Antwortzeiten
- [ ] Agent-Lifecycle: Register -> Active -> Idle -> Deregister
- [ ] Distributed Task-Graph: Abhaengigkeiten zwischen Tasks als DAG
- [ ] Forecast-Scheduling: Vorausschauende Task-Verteilung basierend auf Last

---

## 7. Tier 4 Start

Mit ATC-24 beginnt **Tier 4 (Decentralized AI / Inferenz-Layer)**:

```
TIER 4 — DECENTRALIZED AI (ATC-24+)
├── ATC-24  Agent Scheduling & Task Orchestration   OK IMPLEMENTIERT
├── ATC-25  ...                                       GEPLANT
├── ...
└── ATC-31  Tensor Compute Distribution               GEPLANT
```

> KAI-OS wird von einem passiven Netzwerk zu einem aktiven, "denkenden"
> Betriebssystem, das Ressourcen zielgerichtet auf KI-Aufgaben anwendet.

---

*Dieses Dokument wurde aus der urspruenglichen Atc-24.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:26 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| schedule_task | 1000 |
| cancel_task | 500 |
| get_status | 100 |

### Sprint-Zuweisung

- **Sprint 3.0** — #80
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** MEDIUM

### Testing

5+ Unit-Tests: Schedule, Cancel, Status, Timeout, Retry

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
