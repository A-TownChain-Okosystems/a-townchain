# ATC-37 — Decentralized Reputation-Based Resource Allocation Protocol
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 3.0 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-37
> **Tier:** 5 (User & Application Layer)
> **Referenzen:** ATC-17 (DAO), ATC-24 (Task Orch.), ATC-30 (Reputation), ATC-31 (Tensor Load), ATC-11 (Token), Issue #50 (AI Kernel), #69 (Security-Audit)
> **Quelldatei:** Atc-37.docx (urspruengliche Spezifikation)
> **Kategorie:** Applications & Privacy  

---

## Abstract

ATC-37 definiert das Decentralized Reputation-Based Resource Allocation
Protocol. Es schliesst den Kreis zwischen der oekonomischen Reputation (ATC-30)
und der tatsaechlichen Zuweisung von Systemressourcen.

In einem dezentralen Betriebssystem (KAI-OS) muessen knappe Ressourcen — wie
Rechenzeit fuer KI-Modelle, Speicherplatz im dezentralen Dateisystem oder
Prioritaet in der DAG-Transaktionsverarbeitung — gerecht und effizient verteilt
werden. ATC-37 ist der Algorithmus, der dies ohne zentrale Instanz steuert.

> **ATC-37 = Reputation -> Ressourcen. Wer vertrauenswuerdig ist, bekommt mehr.**
> ATC-30 = Reputation messen. ATC-37 = Reputation anwenden fuer Ressourcen.

---

## 1. Kernkonzepte

### 1.1 Reputations-gewichtete Quoten
Anstatt Ressourcen "wer zuerst kommt, maehlt zuerst" zu vergeben, erhaelt jeder
Nutzer/Agent basierend auf seinem ATC-30-Reputations-Score ein dynamisches
Kontingent. Ein Akteur mit hoher Reputation (z. B. ein bewaehrter KI-
Entwickler oder ein langjaehriger Node-Betreiber) hat eine hoehere Prioritaet
bei der Ressourcenanforderung.

**Implementation:** PARTIAL — Basis da
- Orchestrator (`orchestrator.py`) mit Node-Capability-Matching und Prioritaet
- ATC-30 Reputation-System (konzeptionell) als Grundlage
- Circuit-Breaker fuer ueberlastete Nodes
- **Geplant:** Reputation-gewichtete Quoten pro Nutzer/Agent

### 1.2 Adaptive Angebotssteuerung
ATC-37 ueberwacht die Auslastung des gesamten Netzwerks. Wenn Ressourcen knapp
werden (z. B. bei einer hohen Anzahl von KI-Anfragen), werden die Zugangsschwellen
fuer Ressourcen automatisch angehoben. Nur Nutzer mit einer entsprechenden
Reputation koennen dann noch auf die primaeren Rechenressourcen zugreifen.

**Bezug:** ATC-31 (Tensor Load Balancing) — Echtzeit-Auslastung als Basis.
ATC-06 (Latency) — Netzwerk-Auslastungs-Metriken.

> **Geplant:** Adaptive Thresholds basierend auf Netzwerk-Load

### 1.3 Governance-gesteuerte Allokations-Logik
Die DAO (ATC-17) kann via ATC-37-Proposals bestimmen, welche Arten von Aufgaben
oder Nutzertypen "systemkritisch" sind und daher vorrangig behandelt werden
sollen (z. B. eine Priorisierung von medizinischen Forschungs-KI-Aufgaben
gegenueber Gaming-Anfragen).

**Bezug:** `governance_contract.py` (ATC-17) — DAO-Voting fuer Prioritaets-Klassen.

---

## 2. Warum ATC-37 fuer KAI-OS essenziell ist

### 2.1 Schutz vor Denial-of-Service (DoS) Angriffen
Indem Ressourcen nur an Teilnehmer mit ausreichender Reputation vergeben werden,
macht ATC-37 das KAI-OS extrem widerstandsfaehig gegen Angriffe. Ein Angreifer
ohne Reputation bekame schlicht keine Ressourcen zugewiesen.

### 2.2 Effizienz durch Priorisierung
KI-Agenten, die kritische Inferenz-Aufgaben (Tier 4) bearbeiten, erhalten durch
ATC-37 garantierte Rechenzeit-Slots, waehrend weniger dringende
Hintergrundaufgaben (wie das Indexieren von Daten) warten muessen.

**Bezug:** ATC-24 (Task Orchestration) — Scheduler nutzt ATC-37-Quoten.

### 2.3 Anreiz zur guten Mitarbeit
Da ein guter ATC-30-Score direkt in bessere Ressourcen-Zugaenge (ATC-37)
uebersetzt wird, haben alle Akteure einen starken oekonomischen Anreiz, sich im
Oekosystem "gut" zu verhalten.

> Gutes Verhalten -> Reputation hoch -> Mehr Ressourcen -> Mehr Einnahmen.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-30 (Reputation-Scoring)
ATC-30 liefert die Kennzahl; ATC-37 ist der Mechanismus, der diese Kennzahl in
die tatsaechliche Nutzung von Ressourcen umwandelt.

> ATC-30 = Score berechnen. ATC-37 = Score in Quoten umsetzen.

### 3.2 ATC-24 (Task Orchestration)
Der Scheduler nutzt ATC-37-Quoten, um zu entscheiden, welcher Inferenz-Auftrag
auf einen freien Node geladen wird.

### 3.3 ATC-31 (Tensor Compute Distribution)
Wenn mehrere Anfragen um die gleichen GPU-Ressourcen konkurrieren, nutzt ATC-31
die Zuweisungsregeln aus ATC-37, um die Tensor-Last fair aufzuteilen.

### 3.4 ATC-17 (DAO Governance)
DAO bestimmt Prioritaets-Klassen und Allokations-Parameter.

### 3.5 ATC-11 (Fungible Token)
ATC-11 Token als "Skin in the Game" — hoeherer Stake = hoeheres Quoten-
Kontingent (gekoppelt mit ATC-30).

### 3.6 ATC-29 (Marketplace)
Ressourcen-Quoten koennen im Marketplace gehandelt werden — Quota-Transfer.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Reputation-Quoten | ATC-30 Score -> Kontingent | ATC-30 konzeptionell | PARTIAL Geplant |
| Adaptive Thresholds | Load-basierte Schwellen | Circuit-Breaker | PARTIAL Basis da |
| Governance-Allokation | DAO Prioritaets-Klassen | governance_contract.py | PARTIAL Basis da |
| DoS-Schutz | Keine Rep -> keine Ressourcen | Circuit-Breaker | PARTIAL Basis da |
| Priorisierung | Kritische Tasks zuerst | Orchestrator Prioritaet | OK Implementiert |
| Quota-Transfer | Handel im Marketplace | ATC-29 Marketplace | PARTIAL Basis da |
| Free-Tier-Kontingent | Neue Nutzer nicht ausschliessen | Nicht implementiert | PARTIAL Geplant |
| Sandbox-Starter-Reputation | Bootstrap fuer Neue | Nicht implementiert | PARTIAL Geplant |
| Echtzeit-Load-Monitoring | Netzwerk-Auslastung | Health-Check | PARTIAL Basis da |
| Stake-gewichtete Quoten | ATC-11 als Faktor | ATC-11 implementiert | PARTIAL Basis da |

> **Fazit:** Die Basis (Orchestrator, Governance, ATC-11 Token, Health-Check,
> Circuit-Breaker) ist vorhanden. Die ATC-37-spezifischen Komponenten (Quoten-
> System, Adaptive Thresholds, Free-Tier, Stake-Gewichtung) sind geplant.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #50 | AI Kernel | Done | ATC-37 Orchestrator-Basis |
| #69 | Security-Audit | Open | ATC-37 DoS-Schutz |
| Sprint 3.0 | Reputation-Quoten-System | Geplant | ATC-37 ATC-30 -> Kontingent |
| Sprint 3.0 | Adaptive Thresholds | Geplant | ATC-37 Load-basierte Schwellen |
| Sprint 3.0 | Free-Tier-Kontingent | Geplant | ATC-37 Bootstrap fuer Neue |
| Sprint 3.0 | Quota-Transfer | Geplant | ATC-37 + ATC-29 Marketplace |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Reputation-Quoten: ATC-30 Score -> dynamisches Ressourcen-Kontingent
- [ ] Adaptive Thresholds: Load-basierte Zugangsschwellen
- [ ] Governance-Allokation: DAO bestimmt Prioritaets-Klassen
- [ ] DoS-Schutz: Keine Reputation -> keine Ressourcen
- [ ] Free-Tier: Neue Nutzer bekommen Starter-Kontingent
- [ ] Sandbox-Starter-Reputation: Bootstrap fuer neue Akteure
- [ ] Quota-Transfer: Ressourcen-Quoten handelbar im Marketplace
- [ ] Stake-Gewichtung: ATC-11 Token als zusaetzlicher Quoten-Faktor
- [ ] Echtzeit-Load-Monitor: Netzwerk-Auslastung fuer adaptive Steuerung
- [ ] Priority-Queues: Systemkritische Tasks vor Hintergrundaufgaben
- [ ] Quota-Delegation: Nutzer koennen Quoten an Agenten delegieren
- [ ] Quota-Expiry: Unbenutzte Quoten verfallen nach Zeit

---

## 7. Ressourcen-Allokations-Algorithmus

```
ATC-37 RESOURCE ALLOCATION
├── Input
│   ├── ATC-30 Reputation Score (0.0 - 1.0)
│   ├── ATC-11 Stake (Token amount)
│   ├── Resource Request (GPU/Storage/Bandwidth)
│   └── Current Network Load (ATC-31)
├── Quota Calculation
│   ├── Base Quota = f(Reputation, Stake)
│   ├── Load Multiplier = f(Network Load)
│   │   ├── Low Load: 1.0x (alle bekommen volle Quote)
│   │   ├── Medium Load: 0.7x (Threshold angehoben)
│   │   └── High Load: 0.4x (nur High-Reputation)
│   └── Priority Class = f(DAO Governance)
│       ├── Critical (Medical/Security): 1.5x
│       ├── Standard (Inferenz): 1.0x
│       └── Background (Indexierung): 0.5x
├── Decision
│   ├── Effective Quota = Base * LoadMult * PriorityClass
│   ├── If Effective Quota > Request -> APPROVE
│   ├── If Effective Quota < Request -> QUEUE or DENY
│   └── If Reputation < 0.3 -> DENY (DoS-Schutz)
└── Output
    ├── Resource Allocation (GPU time / Storage / Bandwidth)
    └── On-Chain Log (ATC-04 DAG)
```

### Free-Tier fuer neue Nutzer:
```
Neuer Nutzer (Reputation = 0):
  -> Free-Tier Kontingent (minimale Ressourcen)
  -> Sandbox-Starter-Reputation (0.3 initial)
  -> Nach 100 erfolgreichen Aktionen -> Reputation steigt
  -> Nach genug Reputation -> volle ATC-37 Quoten
```

---

## 8. Tier 5 Fortschritt

```
TIER 5 — USER & APPLICATION (ATC-32+)
├── ATC-32  UX & Interface Abstraction       PARTIAL
├── ATC-33  AI Feedback & RLHF               PARTIAL KONZEPTIONELL
├── ATC-34  Cross-Layer Interop (CLIP)       PARTIAL
├── ATC-35  Data Privacy & Anonymization     PARTIAL KONZEPTIONELL
├── ATC-36  Media Asset & Provenance         PARTIAL
└── ATC-37  Reputation-Based Resource Alloc  PARTIAL KONZEPTIONELL
```

> KAI-OS ist nun in der Lage, sich selbst zu regulieren. Das System
> "entscheidet" autonom, wem es seine knappen Ressourcen anvertraut.

---

*Dieses Dokument wurde aus der urspruenglichen Atc-37.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:53 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| allocate | 1000 |
| deallocate | 500 |
| rebalance | 2000 |

### Sprint-Zuweisung

- **Sprint 3.1** — geplant
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** MEDIUM

### Testing

4+ Unit-Tests: Allocate, Deallocate, Rebalance, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
