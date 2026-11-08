# ATC-30 — Decentralized Reputation & Trust Scoring Protocol
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 3.0 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-30
> **Tier:** 4/5 Uebergang (Decentralized AI -> User Experience)
> **Referenzen:** ATC-01 (Mesh/Uptime), ATC-03 (Identity), ATC-04 (DAG), ATC-15 (Mining), ATC-17 (DAO), ATC-18 (MultiSig/Slashing), ATC-27 (Model Audit), ATC-29 (Marketplace)
> **Quelldatei:** Atc-30.docx (urspruengliche Spezifikation)
> **Kategorie:** AI Orchestration  

---

## Abstract

ATC-30 definiert das Decentralized Reputation & Trust Scoring Protocol. In
einem dezentralen Betriebssystem (KAI-OS) ist Vertrauen keine Frage von
zentralen Zertifikaten, sondern eine quantifizierbare, kryptografisch
verankerte Kennzahl. ATC-30 ist die algorithmische Basis, um festzustellen,
welche Teilnehmer (Nodes, KI-Agenten oder Nutzer) innerhalb des Systems als
"vertrauenswuerdig" eingestuft werden.

> **ATC-30 = Algorithmische Reputation — mathematisches Vertrauen.**
> Tier 4 Abschluss + Tier 5 Uebergang.
> Vertrauen = quantifizierbar, kryptografisch verankert, on-chain.

---

## 1. Kernkonzepte

### 1.1 Multi-Dimensionales Scoring
Ein ATC-30-Reputationswert setzt sich aus mehreren Faktoren zusammen, anstatt
nur eine einzige Zahl zu sein:

| Faktor | Beschreibung | Referenz |
|--------|-------------|----------|
| Uptime/Reliability | Wie konstant ist der Node online? | ATC-01 |
| Inferenz-Qualitaet | Wie praezise sind die KI-Ergebnisse? | ATC-27 |
| Governance-Historie | Konstruktive DAO-Abstimmung? | ATC-17 |
| Economic Stake | "Skin in the Game" (eingesetzte Token) | ATC-11 |

**Implementation:** PARTIAL — Basis da
- ATC-03 (Identity) mit Reputation-Referenzen
- ATC-17 Governance mit Voting-Power
- Orchestrator Health-Check fuer Uptime-Tracking
- **Geplant:** Multi-dimensionaler Score mit gewichteten Faktoren

### 1.2 On-Chain Reputation Graph
Die Reputationswerte werden nicht in einer Datenbank gespeichert, sondern als
"Reputations-Graph" im DAG (ATC-04) gefuehrt. Ein Akteur kann eine
"Reputations-Transaktion" signieren, die seine Vertrauensbeziehung zu einem
anderen Akteur festigt oder — im Falle von Slashing (ATC-18) — massiv
reduziert.

**Bezug:** ATC-04 DAG als unveraenderlicher Reputation-Graph.
ATC-18 (MultiSig) fuer Slashing-Transaktionen.

### 1.3 Algorithmus-Agilitaet
ATC-30 ist so konzipiert, dass der Scoring-Algorithmus durch Governance-
Beschluesse (ATC-17) angepasst werden kann. Wenn die Community beispielsweise
beschliesst, dass "AI-Inferenz-Geschwindigkeit" wichtiger wird als
"Governance-Historie", kann die Gewichtung der Faktoren fuer alle Teilnehmer
dynamisch im Konsens aktualisiert werden.

> DAO kann Scoring-Gewichtung dynamisch anpassen — algortihmische Agilitaet.

---

## 2. Warum ATC-30 fuer KAI-OS essenziell ist

### 2.1 Zero-Trust-Filterung
Wenn ein Agent (Tier 4) eine Aufgabe verteilt, nutzt er den ATC-30-Score, um
Nodes zu filtern: "Sende diese kritische Aufgabe nur an Nodes mit einem
Reputation-Score > 0.8". Das ist der Schutzschild gegen boesartige Akteure.

**Bezug:** Orchestrator (`orchestrator.py`) — Reputation-Filter bei
Task-Zuweisung (ATC-24). Circuit-Breaker fuer low-Reputation Nodes.

### 2.2 Belohnungssystem
Miner (ATC-15) und Inferenz-Anbieter mit einem hohen ATC-30-Score erhalten
bevorzugt Aufgaben zugewiesen und koennen hoehere Gebuehren fuer ihre Dienste
verlangen.

**Bezug:** ATC-29 (Marketplace) — Reputation-Listing sichtbar.
ATC-24 (Scheduler) — Reputation-Weighted Scheduling.

### 2.3 Slashing & Disziplinierung
Ein niedriger ATC-30-Score ist oft die Vorstufe zum automatisierten "Slashing"
(Strafmechanismus). Wenn ein Node wiederholt schlechte KI-Modell-Audits
(ATC-27) abliefert, sinkt sein Score, bis das System ihn automatisch vom
Inferenz-Netzwerk ausschliesst.

**Bezug:** ATC-17 (DAO) + ATC-18 (MultiSig) — Slashing-Mechanismus.
ATC-27 (Model Auditing) — Audit-Fehler reduziert Score.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-03 (Decentralized Identity)
Die Identitaet (DID) ist der Traeger des Scores. Der Score ist untrennbar mit
der DID verknuepft.

### 3.2 ATC-17 (Governance)
Das Stimmgewicht in der DAO kann direkt an den ATC-30-Score gekoppelt sein
(Reputation-based Voting).

### 3.3 ATC-18 (MultiSig/Slashing)
Slashing-Transaktionen reduzieren den ATC-30-Score massiv. MultiSig schuetzt
vor falschem Slashing.

### 3.4 ATC-27 (Model Auditing)
Audit-Ergebnisse fliessen in den Inferenz-Qualitaet-Faktor des Scores ein.

### 3.5 ATC-24 (Task Orchestration)
ATC-24 nutzt den ATC-30-Score fuer Reputation-Weighted Scheduling.

### 3.6 ATC-29 (Marketplace)
Reputation-Score ist sichtbar im Marktplatz und beeinflusst Preis und
Auftragsvergabe.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Multi-Dimensional Score | Uptime + Inferenz + Gov + Stake | Orchestrator Health-Check | PARTIAL Basis da |
| Uptime-Tracking | ATC-01 Reliability | Health-Check implementiert | OK Implementiert |
| Inferenz-Qualitaet | ATC-27 validiert | ATC-27 konzeptionell | PARTIAL Geplant |
| Governance-Historie | DAO-Abstimmung | governance_contract.py | PARTIAL Basis da |
| Economic Stake | ATC-11 Token | ATC-11 implementiert | PARTIAL Basis da |
| On-Chain Reputation Graph | DAG-basierter Graph | ATC-04 DAG vorhanden | PARTIAL Basis da |
| Reputations-Transaktion | Signierte Trust-Relation | Nicht implementiert | PARTIAL Geplant |
| Slashing-Integration | ATC-18 Score-Reduktion | governance_contract da | PARTIAL Basis da |
| Algorithmus-Agilitaet | DAO adjustable Weights | Nicht implementiert | PARTIAL Geplant |
| Reputation-based Voting | ATC-17 + Score | Nicht implementiert | PARTIAL Geplant |
| Decay-Modell | Alte Fehler verblassen | Nicht implementiert | PARTIAL Geplant |
| Zero-Trust-Filter | Score > Threshold | Circuit-Breaker | PARTIAL Basis da |

> **Fazit:** Die Bausteine (Identity, Governance, DAG, Health-Check, ATC-11
> Stake, Circuit-Breaker) sind vorhanden. Der multi-dimensionale Score-
> Algorithmus und der Reputation-Graph sind der Kern von ATC-30 und noch
> konzeptionell.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #6 | ECDSA Implementation | Done | ATC-30 Signatur-Basis |
| #9 | Governance DAO | Done | ATC-30 Governance-Historie |
| #39 | DAO-Governance Live | Done | ATC-30 Reputation-Voting |
| #50 | AI Kernel | Done | ATC-30 KI-Agent Reputation |
| #69 | Security-Audit | Open | ATC-30 Reputation-Algorithmus |
| Sprint 3.0 | Multi-Dimensional Scoring | Geplant | ATC-30 Score-Algorithmus |
| Sprint 3.0 | Reputation Graph | Geplant | ATC-30 On-Chain Trust-Graph |
| Sprint 3.0 | Decay-Modell | Geplant | ATC-30 Zeitabhaengige Bewertung |
| Sprint 3.0 | Reputation-based Voting | Geplant | ATC-30 + ATC-17 |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Multi-Dimensional Score: Gewichtete Kombination aus 4+ Faktoren
- [ ] On-Chain Reputation Graph: Trust-Relationen im DAG
- [ ] Reputations-Transaktion: Signierte Trust/Slash-Events
- [ ] Algorithmus-Agilitaet: DAO-adjustable Weight-Parameter
- [ ] Decay-Modell: Alte Fehler verblassen, aktuelle zaehlen mehr
- [ ] Reputation-based Voting: ATC-17 Stimmgewicht = f(Score)
- [ ] Zero-Trust-Filter: Score-Threshold fuer kritische Aufgaben
- [ ] Slashing-Automation: ATC-27 Audit-Fehler -> Score-Reduktion -> Ausschluss
- [ ] Reputation-Explorer: Visualisierung des Trust-Graphs
- [ ] Cross-Agent Trust: KI-Agenten bewerten andere Agenten
- [ ] Reputation-Transfer: Score bei Node-Migration (ATC-02)
- [ ] Economic Stake Weighting: Hoher Stake = mehr Vertrauen (Skin in the Game)

---

## 7. Score-Berechnung — Konzept

### Multi-Dimensionaler Score:
```
ATC-30 Score = w1 * Uptime + w2 * InferenzQuality + w3 * GovernanceHist + w4 * Stake

w1, w2, w3, w4: DAO-adjustable Weights (sum = 1.0)
Uptime:         [0, 1] — Health-Check Verfuegbarkeit
InferenzQuality:[0, 1] — ATC-27 Audit-Erfolgsrate
GovernanceHist: [0, 1] — DAO-Abstimmungskonsistenz
Stake:          [0, 1] — Normalisierter ATC-11 Token-Stake
```

### Decay-Modell (geplant):
```
Score(t) = Score_current * decay_factor(t - t_event)
  - Neues Fehlverhalten: decay = 1.0 (volles Gewicht)
  - 30 Tage alt: decay = 0.7
  - 90 Tage alt: decay = 0.4
  - 180 Tage alt: decay = 0.1
  - Verbesserung: Positives Verhalten hebt Score wieder an
```

### Slashing-Kaskade:
```
1. Audit-Fehler (ATC-27) -> Score -= 0.15
2. Node offline > 1h -> Score -= 0.05
3. DAO-Missverhalten -> Score -= 0.10
4. Score < 0.3 -> Ausschluss vom Inferenz-Netzwerk
5. Score < 0.1 -> Slashing (ATC-18) + Stake-Verlust
```

---

## 8. Tier 4/5 Uebergang

ATC-30 markiert den **Uebergang von Tier 4 (Decentralized AI) zu Tier 5
(User Experience & Interface-Abstraktion)**:

```
TIER 4 — DECENTRALIZED AI (ATC-24 bis ATC-29)
├── ATC-24  Agent Scheduling              OK IMPLEMENTIERT
├── ATC-25  Tensor Compute                PARTIAL
├── ATC-26  Explainable AI                PARTIAL KONZEPTIONELL
├── ATC-27  Model Auditing                PARTIAL KONZEPTIONELL
├── ATC-28  Federated Learning            OK IMPLEMENTIERT + PARTIAL
└── ATC-29  AI Marketplace                PARTIAL

TIER 4/5 UEBERGANG
└── ATC-30  Reputation & Trust Scoring    PARTIAL

TIER 5 — USER EXPERIENCE (ATC-31+)
└── ...                                   GEPLANT
```

> KAI-OS verfuegt ueber ein soziales Sicherheitsnetz. Vertrauen basiert nicht
> auf blindem Glauben, sondern auf mathematisch ueberpruefbarer Historie.

---

*Dieses Dokument wurde aus der urspruenglichen Atc-30.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:39 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| update_reputation | 500 |
| query_reputation | 100 |
| slash | 5000 |

### Sprint-Zuweisung

- **Sprint 3.0** — #80
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** MEDIUM

### Testing

5+ Unit-Tests: Update, Query, Slash, Decay, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
