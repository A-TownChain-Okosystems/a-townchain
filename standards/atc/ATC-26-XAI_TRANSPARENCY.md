# ATC-26 — Explainable AI (XAI) & Transparency Protocol
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 3.0 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-26
> **Tier:** 4 (Decentralized AI / Inferenz-Layer)
> **Referenzen:** ATC-04 (DAG), ATC-10 (Oracles), ATC-17 (DAO/Slashing), ATC-24 (Task Orchestration), ATC-25 (Tensor Compute), Issue #2 (Gemini AI), #50 (AI Kernel)
> **Quelldatei:** Atc-26.docx (urspruengliche Spezifikation)
> **Kategorie:** AI Orchestration  

---

## Abstract

ATC-26 definiert das Explainable AI (XAI) & Transparency Protocol. In einem
System, das KI-Agenten fuer kritische Entscheidungen (wie Finanztransaktionen
oder Governance) einsetzt, ist die "Blackbox-Problematik" von neuronalen Netzen
ein grosses Risiko. ATC-26 erzwigt, dass jede KI-Entscheidung innerhalb des
KAI-OS mit einem nachvollziehbaren "Begruendungspfad" versehen werden muss.

> **ATC-26 = Transparenz und Erklarbarkeit der KI.**
> ATC-24 = Was zu tun ist.
> ATC-25 = Wie die Tensor-Daten fliessen.
> ATC-26 = Warum die KI so entschieden hat.

---

## 1. Kernkonzepte

### 1.1 Attributions-Tracing
Wenn eine KI eine Entscheidung trifft, muss der ATC-26-konforme Agent einen
Trace mitliefern. Dieser Trace zeigt, welche Eingabedaten (Input-Tensoren) und
welche Gewichtungen des Modells massgeblich fuer das Ergebnis waren.

**Aktueller Stand:** Nicht implementiert. AI-Kernel (`ai_kernel.py`) liefert
Inferenz-Ergebnisse ohne Attribution-Traces. Konzept fuer SHAP-Werte und
Attention-Maps geplant.

> **Geplant:** Attributions-Trace als Metadata bei jeder Inferenz

### 1.2 XAI-Standard-Metadaten
Jedes Inferenz-Ergebnis, das in den DAG (ATC-04) einfliesst, wird mit
Metadaten nach ATC-26 angereichert. Diese enthalten eine kompakte mathematische
Darstellung (z. B. SHAP-Werte oder Attention-Maps), die erklaert, warum der
Agent so gehandelt hat.

**Konzept:**
```
Inferenz-Ergebnis + XAI-Metadata:
  {
    result: {...},
    xai_trace: {
      shap_values: [...],       // Feature-Wichtigkeit
      attention_map: [...],     // Aufmerksamkeits-Verteilung
      model_version: "v3.0",
      input_hash: "sha256(...)", // Welcher Input
      confidence: 0.94          // Konfidenz-Score
    }
  }
```

> **Geplant:** XAI-Metadaten als Standard-Feld in DAG-Transaktionen

### 1.3 Human-in-the-loop Verifikation
Fuer besonders kritische Entscheidungen (z. B. Slashing eines Nodes) erlaubt
ATC-26 eine "Proof-of-Explainability"-Anfrage. Ein Mensch oder ein Governance-
Agent kann den Trace anfordern und die KI-Entscheidung auf ihre Plausibilitaet
pruefen lassen, bevor die Transaktion finalisiert wird.

**Bezug:** ATC-17 (DAO Governance) — Governance-Agent fordert XAI-Trace an
vor Slashing-Freigabe. ATC-18 (MultiSig) — Kritische KI-Aktion braucht
zusaetzliche Freigabe.

---

## 2. Warum ATC-26 fuer KAI-OS essenziell ist

### 2.1 Vertrauen & Compliance
Damit KAI-OS in regulierten Umgebungen (Finanzen, Recht) eingesetzt werden
kann, muss es den Grundsatz der "Erklarbarkeit" erfuellen. Nutzer muessen
verstehen, warum ein KI-Agent eine bestimmte Investition getaetigt hat.

### 2.2 Debugging der KI
Wenn ein Agent "Fehler" macht (z. B. Inferenz-Ergebnisse liefert, die zu
Inkonsistenzen fuehren), dient der ATC-26-Trace Entwicklern und Auditoren dazu,
den Fehler im neuronalen Netz oder in der Entscheidungslogik gezielt zu
identifizieren.

### 2.3 Reputation & Slashing
Die DAO-Governance nutzt ATC-26-Traces, um bei Fehlverhalten eines KI-Agenten
zu entscheiden: War es ein vorsaetzlicher Betrug, ein Systemfehler oder eine
Fehlinterpretation der Daten? Nur mit XAI-Daten kann das Slashing
(Strafmechanismus) fair angewendet werden.

**Bezug:** ATC-17 (DAO Governance) — Slashing-Entscheidung basierend auf
XAI-Traces. Fairness durch Transparenz.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-24 (Task Orchestration)
ATC-24 schickt den Auftrag, ATC-26 verlangt die "Begruendung" fuer das
gelieferte Ergebnis.

### 3.2 ATC-10 (Global Time & Oracles)
XAI-Daten werden oft mit den Oracle-Daten korreliert, um zu beweisen, dass die
KI auf der Basis der damals korrekten Marktinformationen gehandelt hat.

### 3.3 ATC-04 (DAG Consensus)
Der XAI-Trace wird als Teil des Transaktions-Events unveraenderlich im Ledger
gespeichert, sodass er auch Jahre spaeter noch auditierbar ist.

### 3.4 ATC-17 (DAO Governance)
DAO nutzt XAI-Traces fuer Slashing-Entscheidungen und Reputation-Management.

### 3.5 ATC-25 (Tensor Compute)
ATC-25 liefert die Tensor-Daten, ATC-26 erklaert welche davon massgeblich waren.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Attributions-Tracing | Trace mit Input + Gewichtung | Nicht implementiert | PARTIAL Geplant |
| XAI-Metadaten | SHAP/Attention in DAG | Nicht implementiert | PARTIAL Geplant |
| Human-in-the-loop | Proof-of-Explainability | Nicht implementiert | PARTIAL Geplant |
| Inferenz-Logging | Ergebnis + Begruendung | AI-Kernel liefert Ergebnis | PARTIAL Basis da |
| DAG-Integration | XAI-Trace im Ledger | DAG (ATC-04) vorhanden | PARTIAL Basis da |
| Governance-Integration | DAO nutzt XAI fuer Slashing | governance_contract da | PARTIAL Basis da |
| Oracle-Korrelation | XAI + ATC-10 Marktdaten | ATC-10 Oracle geplant | PARTIAL Geplant |
| Compressed XAI-Proof | Platzsparende Beweise | Nicht implementiert | PARTIAL Geplant |

> **Fazit:** Die Infrastruktur (AI-Kernel, DAG, Governance) ist als Basis da.
> Die XAI-spezifischen Komponenten (Attributions-Tracing, SHAP-Werte,
> Proof-of-Explainability) sind konzeptionell und ein klarer Entwicklungs-
> schritt fuer regulierte KI-Anwendungen.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #2 | Gemini AI Integration | Done | ATC-26 KI-Inferenz Basis |
| #50 | AI Kernel | Done | ATC-26 AI-Kernel Logging |
| #69 | Security-Audit | Open | ATC-26 XAI-Auditierbarkeit |
| Sprint 3.0 | Attributions-Tracing | Geplant | ATC-26 SHAP/Attention |
| Sprint 3.0 | XAI-Metadaten in DAG | Geplant | ATC-26 DAG-Integration |
| Sprint 3.0 | Proof-of-Explainability | Geplant | ATC-26 Human-in-the-loop |
| Sprint 3.0 | Compressed XAI-Proofs | Geplant | ATC-26 Platzsparende Beweise |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Attributions-Tracing: SHAP-Werte bei jeder Inferenz
- [ ] Attention-Maps: Aufmerksamkeits-Verteilung als XAI-Metadaten
- [ ] XAI-Standard-Feld: Metadaten in DAG-Transaktionen
- [ ] Proof-of-Explainability: Anfrage-Protokoll fuer kritische Entscheidungen
- [ ] Human-in-the-loop: Mensch/Governance prueft KI-Entscheidung
- [ ] Compressed XAI-Proofs: Platzsparende Beweise (Merkle-Tree)
- [ ] XAI-Explorer: Visualisierung von Traces im Blockchain-Explorer
- [ ] Model-Version-Tracking: Welche Modellversion hat entschieden?
- [ ] Confidence-Score: Konfidenz bei jeder KI-Entscheidung
- [ ] Audit-Trail: XAI-Traces ueber Jahre im DAG auditierbar
- [ ] DAO-Slashing-XAI: Slashing nur mit XAI-Trace als Begruendung
- [ ] Regulatory-Mode: XAI-Pflicht fuer Finanz-/Rechts-Entscheidungen

---

## 7. Proof-of-Explainability — Konzept

### Anfrage-Prozess:
```
1. KI-Agent trifft Entscheidung (z. B. Slashing vorschlagen)
2. Governance-Agent fordert Proof-of-Explainability an
3. KI-Agent liefert XAI-Trace (SHAP, Attention, Confidence)
4. Governance-Agent (oder Mensch) prueft Plausibilitaet
5. Bei positivem Review -> Transaktion finalisiert
6. Bei negativem Review -> Entscheidung blockiert, Agent flagged
```

### Komprimierte Beweise:
XAI-Traces koennen sehr gross sein (Attention-Maps fuer LLMs). Loesung:
- Merkle-Tree-Kompression: Nur Root-Hash on-chain, voller Trace off-chain
- Sampled-Attributions: Repraentative Stichprobe statt voller Daten
- Layer-Summary: Zusammenfassung pro Layer statt pro Neuron

---

*Dieses Dokument wurde aus der urspruenglichen Atc-26.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:30 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| explain | 5000 |
| audit_log | 200 |
| get_trace | 100 |

### Sprint-Zuweisung

- **Sprint 3.0** — #80
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** MEDIUM

### Testing

4+ Unit-Tests: Explain, Audit-Log, Trace, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
