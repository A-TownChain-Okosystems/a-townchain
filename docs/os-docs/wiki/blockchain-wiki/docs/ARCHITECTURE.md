# 🏛️ Architektur — atc-blockchain

> **Kanonische Dokumentation für die Architektur von `atc-blockchain`**

---

## 1. Systemübersicht

`atc-blockchain` nimmt die Layer-3-Rolle im A-TownChain OS Ökosystem ein. Das Modul ist verantwortlich für die dezentrale Konsensfindung, die Erzeugung von Blöcken, die Transaktionsverwaltung im Mempool sowie die verifizierbare Ausführung von Smart Contracts in der ATVM.

---

## 2. Layering & Interaktion

```
+-------------------------------------------------------+
|  Layer L10/L7: Client Applications / API Gateway      |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
|  Layer L3: Blockchain Core (atc-blockchain)           |
|  ├─ Mempool & Tx Validator                            |
|  ├─ PoH Timestamp Generator (Verifiable Clock)        |
|  ├─ PoA/PoS Hybrid Consensus State Machine            |
|  ├─ ATVM Contract Engine & State Storage              |
|  └─ Governance & Treasury                             |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
|  Layer L5: P2P Network (atcnet)                       |
+-------------------------------------------------------+
|  Layer L2: ShivaOS Microkernel (atc-kernel)          |
+-------------------------------------------------------+
```

---

## 3. Subsysteme & Kern-Module

1. **Konsens-Subsystem (`consensus/`)**
   - **PoH Engine (`poh.py` / `poh.atc`)**: Sequentielles Hashing zur krypographischen Verifikation der verstrerichenen Zeit vor der Konsensfindung.
   - **Hybrid State Machine (`hybrid_consensus.atc`)**: PoH erzeugt Zeitstempel, PoA validiert Blöcke und entscheidet Finalität.
   - **Gabelungsauflösung (`fork_resolution.atc`)**: Longest-Chain- und Heaviest-PoH-Weight-Regel zur automatischen Auflösung von Netzwerk-Forks.
   - **Gaspreis-Modell (`gas_fee.atc`)**: Dynamische Anpassung der Ausführungsgebühren basierend auf der Blockauslastung.

2. **Knoten- & Netzwerk-Verwaltung (`nodes/`)**
   - **Validator Node Daemon (`node.atc`)**: Verwaltet den lokalen Ledger-Zustand, verifiziert eingehende Blöcke und nimmt an Konsensrunden teil.
   - **P2P Integration (`p2p_propagation.py`, `discovery.py`)**: Verbindet den Node direkt mit dem `atcnet` Overlay-Netzwerk.

3. **Vertrags- & Ausführungsebene (`contracts/`)**
   - Ausführung von Smart Contracts in ATCLang oder Python.
   - Lizenzprüfung jedes Vertrages gemäß **ATC-LIC** vor der Zustandstransformation.

4. **On-Chain Governance (`governance/`)**
   - Abstimmungen über Parameteränderungen, Validator-Sets und Treasury-Auszahlungen.

---

*Teil des [A-TownChain Ökosystems](https://github.com/A-TownChain-Okosystems)*
