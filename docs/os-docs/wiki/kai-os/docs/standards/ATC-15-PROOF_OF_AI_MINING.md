# ATC-15 — Decentralized Mining Protocol (Proof-of-AI-Mining)
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.4 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-15
> **Tier:** 2 (Logik & Oekonomie) — Bruecke zu Tier 4 (KI)
> **Referenzen:** ATC-11 (Fungible Assets/Rewards), ATC-14 (Deterministic Exec), ATC-31 (Tensor Compute, geplant), Issue #2 (Gemini AI)
> **Quelldatei:** Atc-15.docx (urspruengliche Spezifikation)
> **Kategorie:** Economy & Assets  

---

## Abstract

ATC-15 definiert das Decentralized Mining Protocol (oder auch Proof-of-AI-Mining).
Im Gegensatz zu klassischen Blockchains, bei denen Miner durch das Loesen von
mathematischen Raetseln (Proof of Work) das Netzwerk sichern, verlagert KAI-OS
diesen Prozess auf die Bereitstellung von nuetzlicher Rechenleistung fuer
KI-Inferenz und Modelltraining.

> **ATC-15 ist der Uebergang zum "produktiven Mining".**
> ATC-01 bis 10 = Infrastruktur (Tier 1).
> ATC-11 bis 14 = Oekonomie und Ausfuehrung (Tier 2).
> ATC-15 = Bruecke: KI-Leistung (Tier 4) -> Oekonomie (Tier 2).

---

## 1. Kernkonzepte

### 1.1 Mining als Inferenz-Leistung
ATC-15 legt fest, dass "Mining" im A-TownChain-Oekosystem bedeutet, aktiv
Tensor-Compute-Leistung fuer das Netzwerk zur Verfuegung zu stellen (siehe
Tier 4). Anstatt Energie fuer das Hash-Rechnen zu verschwenden, erbringen die
Miner einen oekonomisch nuetzlichen Beitrag durch das Ausfuehren von KI-Aufgaben.

**Aktueller Stand:** Das aktuelle Mining nutzt klassisches PoW (SHA3-ATC) in
`blockchain/consensus/pow.py`. Der AI-Kernel (`ai_kernel.py`) mit LLMRouter
und Orchestrator (`orchestrator.py`) dispatcht KI-Tasks an verfuegbare Nodes.
Proof-of-AI (Mining = Inferenz) ist konzeptionell als Evolutionspfad geplant.

```python
# pow.py — Aktuelles klassisches Mining
class ProofOfWork:
    def mine(self, block, difficulty):
        # SHA3-ATC Hash-Raetsel loesen
        # Ziel: nonce finden mit hash < target

# GEPLANT: Proof-of-AI
class ProofOfAI:
    def mine(self, inference_task):
        # KI-Inferenz ausfuehren statt Hash rechnen
        # Reward basierend auf verifizierter Inferenz-Leistung
```

> **Status:** PoW implementiert. Proof-of-AI als Evolution geplant.

### 1.2 Validierung der Rechenarbeit
Da KI-Berechnungen (Inferenz) nicht sofort "beweisbar" sind wie ein einfacher
Hash, definiert ATC-15 kryptografische Protokolle, um zu beweisen, dass ein Node
tatsaechlich die versprochene Rechenleistung erbracht hat (Proof-of-AI).

**Konzept:**
- **Result-Verification:** Auditoren fuehren dieselbe Inferenz aus und vergleichen
- **Stake-Slashing:** Falsche Ergebnisse -> Stake verlieren (PoS Integration)
- **Reputation-Score:** Wiederholte korrekte Inferenz -> hoeherer Score
- **ZK-Proofs:** Zero-Knowledge-Beweise fuer Inferenz-Ergebnisse (Zukunft)

> **Geplant:** Proof-of-AI Verifikations-Protokoll mit Auditor-Netzwerk.

### 1.3 Mining-Belohnung (Rewards)
ATC-15 regelt die Ausschuettung der systemeigenen Token (gemaess ATC-11) an die
Miner. Die Belohnung ist dynamisch und korreliert direkt mit der Menge an
verifizierter Inferenz-Leistung, die ein Node in das Netzwerk einbringt.

**Aktueller Stand:** PoW-Mining belohnt Miner mit Block-Reward (ATCoin).
Die Reward-Logik ist in `atcoin.py` implementiert (Max Supply: 21M ATC).
Proof-of-Ai wuerde den Reward an Inferenz-Leistung koppeln, nicht an Hash-Rate.

---

## 2. Warum ATC-15 fuer KAI-OS entscheidend ist

### 2.1 Dezentralisierung der KI
ATC-15 ist der oekonomische Hebel, der es ermoeglicht, ein globales Netzwerk von
privaten GPUs aufzubauen, ohne dass man auf teure, zentrale Cloud-Provider
angewiesen ist.

**Bezug:** ATCNet P2P-Netzwerk + AI-Kernel. Jeder Node kann Miner + KI-Provider
sein — Dezentralisierung der Compute-Power.

### 2.2 Sicherheit durch Arbeit
Da die Miner gleichzeitig die Inferenz-Leistung bereitstellen, ist das Netzwerk
gegen Angriffe geschuetzt — ein Angreifer muesste ueber enorme GPU-Kapazitaeten
verfuegen, um das System zu dominieren, was bei KAI-OS jedoch direkt zur
Staerkung des KI-Oekosystems fuehrt.

### 2.3 Auto-Remediation Anbindung
Nodes, die gemaess ATC-15 "Mining" betreiben, werden vom Auto-Remediation-System
besonders eng ueberwacht, um sicherzustellen, dass sie nicht versuchen, ihre
Inferenz-Leistung vorzutaeuschen.

**Bezug:** Orchestrator (`orchestrator.py`) mit Circuit-Breaker und
Health-Check-Mechanismen. Fraud-Detection fuer fake Inferenz geplant.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-31 (Tensor Compute Distribution)
ATC-31 sucht die Miner, ATC-15 belohnt sie fuer ihre Arbeit.

> ATC-31 ist ein zukuenftiger Standard — definiert Task-Distribution an GPU-Miner.

### 3.2 ATC-14 (Deterministic Execution)
Da ein Mining-Reward eine Bilanzaenderung darstellt, muss die Berechnung des
Rewards deterministisch ablaufen, damit alle Nodes im Konsens dieselbe
Belohnung berechnen.

**Bezug:** ATC-14 sichert die deterministische Reward-Berechnung. Siehe
`base_contract.py` und `atcoin.py` fuer aktuelle Reward-Logik.

### 3.3 Tier 4 (Decentralized AI)
ATC-15 ist die Bruecke, die die "KI-Leistung" (Tier 4) in die "Oekonomie"
(Tier 2) ueberfuehrt.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Mining als Inferenz | Tensor-Compute statt Hash | PoW (SHA3-ATC) klassisch | PARTIAL Geplant |
| Proof-of-AI | Kryptografischer Beweis fuer Inferenz | Nicht implementiert | PARTIAL Geplant |
| Result-Verification | Auditoren vergleichen Ergebnisse | validate_chain in Consensus | PARTIAL Basis da |
| Mining-Reward | Dynamisch nach Inferenz-Leistung | Block-Reward (21M ATC) | PARTIAL Basis da |
| Stake-Slashing | Falsche Inferenz -> Stake verlieren | PoS in `pos.py` (Basis) | PARTIAL Basis da |
| Reputation-Score | Korrekte Inferenz -> hoeherer Score | Nicht implementiert | PARTIAL Geplant |
| ZK-Proofs | Zero-Knowledge fuer Inferenz | Nicht implementiert | PARTIAL Geplant |
| Auto-Remediation | Ueberwachung der Miner | Circuit-Breaker im Orchestrator | PARTIAL Basis da |

> **Fazit:** Die Mining-Infrastruktur (PoW, PoS, Block-Reward) ist voll
> implementiert. Der Uebergang von "Hash-Mining" zu "Inferenz-Mining"
> (Proof-of-AI) ist die grosste Innovation — und der grosste Entwicklungsschritt.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #2 | Gemini AI Integration | Done | ATC-15 KI-Inferenz-Basis |
| #8 | Testnet (Multi-Node) | Done | ATC-15 Multi-Node Mining |
| #29 | Federated Learning | Done | ATC-15 Dezentrales Training |
| #50 | AI Kernel | Done | ATC-15 KI-Orchestrierung |
| Sprint 2.4 | Proof-of-AI Protocol | Geplant | ATC-15 Inferenz-Mining |
| Sprint 2.4 | Dynamic Reward (Inferenz-basiert) | Geplant | ATC-15 Reward-Kopplung |
| Sprint 2.4 | Auditor-Netzwerk | Geplant | ATC-15 Result-Verification |
| Sprint 2.4 | Stake-Slashing fuer Fake-Inferenz | Geplant | ATC-15 Fraud-Prevention |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Proof-of-AI Protocol: Mining = KI-Inferenz ausfuehren
- [ ] Result-Verification: Auditoren wiederholen Inferenz und vergleichen
- [ ] Dynamic Reward: ATC-11 Token basierend auf verifizierter Inferenz-Leistung
- [ ] Reputation-Score: Korrekte Inferenz erhoht Score, falsche senkt ihn
- [ ] Stake-Slashing: Vorgetaeuschte Inferenz -> Stake verlieren (ATC-03)
- [ ] ZK-Proofs fuer Inferenz: Zero-Knowledge-Beweis ohne volle Offenlegung
- [ ] GPU-Benchmark: Standardisierte Tensor-Compute-Leistungsmessung
- [ ] ATC-31 Integration: Tensor Compute Distribution an Miner
- [ ] Auto-Remediation: Fraud-Detection fuer fake Inferenz-Ergebnisse
- [ ] Mining-Pool: Zusammenschluss kleiner GPU-Anbieter

---

## 7. Evolutionspfad: PoW -> Proof-of-AI

```
AKTUELL (v3.0.0)                      ZUKUNFT (v5.0+)
PoW Mining (SHA3-ATC)           ->    Proof-of-AI Mining (Inferenz)
Block-Reward (fix pro Block)    ->    Dynamic Reward (Inferenz-Leistung)
Hash-Rate = Sicherheit          ->    GPU-Kapazitaet = Sicherheit + Nutzen
Energie fuer Hash-Rechnen       ->    Energie fuer KI-Inferenz (produktiv)
Zentrale GPU-Clouds             ->    Dezentrales GPU-Netzwerk (ATCNet)
```

### Migrationsstrategie:
1. **Phase 1:** Proof-of-AI als zusaetzliche Mining-Methode (neben PoW)
2. **Phase 2:** Inferenz-Reward neben Block-Reward auszahlen
3. **Phase 3:** Auditor-Netzwerk fuer Result-Verification
4. **Phase 4:** Reputation-Score und Stake-Slashing
5. **Phase 5:** PoW graduell reduzieren, Proof-of-AI dominieren
6. **Phase 6:** Vollstaendiger Proof-of-AI (PoW als Fallback)

---

*Dieses Dokument wurde aus der urspruenglichen Atc-15.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:57 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| register_ai | 50000 |
| submit_work | 1000 |
| verify_work | 5000 |

### Sprint-Zuweisung

- **Sprint 2.5** — geplant
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

5+ Unit-Tests: AI-Registration, Work-Submission, Verification, Reward

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
