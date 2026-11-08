# ATC-07 — Network-Level Sharding & State Partitioning

> **Issue:** #84 | **Wiki:** Kap.37 | **Sprint:** 2.2
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.3 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-07
> **Referenzen:** ATC-02 (Liquid State), ATC-04 (DAG Consensus), ATC-03 (Zero-Trust), ATC-05 (PQC)
> **Quelldatei:** Atc-07.docx (urspruengliche Spezifikation)
> **Kategorie:** P2P Networking  
> **Tier:** Tier 1 — Blockchain Infrastructure  

---

## Abstract

ATC-07 ist das Architektur-Protokoll fuer Network-Level Sharding & State
Partitioning. In einem System, das weltweit auf tausenden Nodes KI-Inferenz
und komplexe Transaktionen ausfuehrt, ist es unmoeglich, dass jeder Node den
gesamten Zustand des Netzwerks speichert. ATC-07 ist die Loesung fuer dieses
Skalierungsproblem.

> **Merksatz:** ATC-01 = Verbindung. ATC-02 = Gedaechtnis. ATC-03 = Identitaet.
> ATC-04 = Parallele Wahrheit. ATC-05 = Quantensicherheit. ATC-06 = Performance.
> ATC-07 = Skalierbarkeit durch Fragmentierung.

---

## 1. Kernkonzepte

### 1.1 Horizontales Sharding
ATC-07 teilt das Netzwerk in logische Gruppen oder "Shards" auf. Jeder Shard
ist nur fuer eine Teilmenge des globalen Zustands verantwortlich (z. B. eine
Gruppe von User-Wallets oder eine spezifische KI-Modell-Instanz). Dies
entlastet die einzelnen Nodes massiv.

**Aktueller Stand:** Die aktuelle Architektur hat Node-Typen (FULL, LIGHT,
VALIDATOR, MINER) in `blockchain/nodes/node.py`. FULL-Nodes speichern die
komplette Chain, LIGHT-Nodes nur Block-Header. Sharding (Partition-ID pro
Node) ist konzeptionell als naechste Evolutionsstufe geplant.

```python
class NodeType(Enum):
    FULL      = "full"    # Komplette Chain — entspricht "Global State"
    LIGHT     = "light"   # Nur Header — entspricht "Light Client"
    VALIDATOR = "validator"  # PoS-Teilnehmer
    MINER     = "miner"   # PoW-Teilnehmer
```

> **Geplant:** SHARD_NODE Typ mit Partition-ID (`0xAF32` etc.)

### 1.2 State Partitioning
Anstatt den kompletten Ledger (die Datenbank aller Transaktionen) auf jeden
Node zu kopieren, definiert ATC-07, wie der "State" (der aktuelle Stand) in
Partitionen zerlegt wird. Ein Node muss so nur noch den Teil des Zustands
validieren, der seinen Shard betrifft.

**Aktueller Stand:** Die DB-Persistenz (`backend/db/repository.py`) speichert
alle Daten in SQLite. Es gibt aktuell keine Partitionierung. Das Konzept der
State-Partitionierung baut auf ATC-02 (Liquid State) auf — die
Snapshot-Mechanik von ATC-02 wuerde pro Partition angewendet werden.

### 1.3 Cross-Shard Communication
Wenn ein Agent in Shard A mit einer Ressource in Shard B interagieren will,
regelt ATC-07 den sicheren Datenaustausch und die atomare
Zustandsaktualisierung ueber Shard-Grenzen hinweg.

**Konzept:**
- Cross-Shard-Transaction: TX beruehrt State in Shard A und Shard B
- Two-Phase Commit: Lock in A → Lock in B → Commit beide → oder Rollback
- Merkle-Proof: Shard A beweist via Merkle-Tree, dass State-X existiert
- Atomicity: Entweder beide Shards aktualisieren, oder keiner

> **Geplant:** `CROSS_SHARD_TX` Nachrichtentyp in ATCNet, Merkle-Proof-basierte
> State-Verifikation zwischen Shards.

---

## 2. Warum ATC-07 fuer KAI-OS essenziell ist

### 2.1 Skalierbarkeit
Ohne Sharding wuerde die Geschwindigkeit des Netzwerks durch den langsamsten
Node begrenzt. Mit ATC-07 waechst die Performance linear mit der Anzahl der
Nodes, da Shards parallel arbeiten koennen.

### 2.2 Ressourceneffizienz
Ein Node, der nur Inferenz fuer eine bestimmte KI-Instanz (Tier 4) bereitstellt,
muss nicht die Transaktionsdaten von tausenden anderen Anwendungen speichern.

**Bezug:** AI-Kernel (`ai_kernel.py`) mit LLMRouter fuer 4 Modelle. Ein
KI-spezifischer Shard koennte nur Inferenz-Daten und Decision-Logs halten.

### 2.3 Sicherheit trotz Fragmentierung
ATC-07 stellt sicher, dass Shards nicht "isoliert" agieren. Durch
kryptografische Beweise (aehnlich wie Zero-Knowledge Proofs) kann ein Shard
gegenueber dem Gesamtsystem beweisen, dass sein Zustand konsistent und
korrekt ist, ohne dass das gesamte Netzwerk den Zustand im Detail pruefen muss.

**Bezug:** ATC-05 (Quantum-Resistant Signatures) sichert die
Cross-Shard-Beweise. ATC-03 (Zero-Trust IAM) verifiziert, dass nur
autorisierte Nodes Cross-Shard-TXs initiieren duerfen.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-02 (Liquid State Migration)
ATC-07 legt fest, wie der Zustand partitioniert wird; ATC-02 sorgt dafuer,
dass dieser Zustand "fluessig" zwischen Nodes migrieren kann, falls sich die
Last eines Shards verschiebt.

**Synergie:** Sharding definiert die Partitionen → Liquid State migriert
Zustand zwischen Nodes innerhalb eines Shards → Load-Balancer verteilt Tasks.

### 3.2 ATC-04 (DAG Consensus)
Innerhalb eines Shards wird der DAG-Konsens angewendet. ATC-07 definiert die
Grenzen, an denen ein DAG-Event endet und wo ein Cross-Shard-Prozess beginnt.

**Konzept:** Jeder Shard hat eigenen DAG → Cross-Shard-Events sind
Meta-Events, die mehrere Shard-DAGs referenzieren.

### 3.3 Tier 3 (OS Infrastructure)
Das Betriebssystem verwaltet via ATC-07 die lokale Speicherallokation, sodass
der Node genau weiss: "Ich bin fuer Partitions-ID 0xAF32 zustaendig".

**Bezug:** ShivaOS Kernel (`core/kernel.py`), ATCFS (`shivaos/fs/atcfs.py`).
Die Partition-ID wuerde in der Node-Konfiguration (`settings.json`) hinterlegt.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Horizontales Sharding | Logische Shard-Gruppen | Node-Typen (FULL/LIGHT/VALIDATOR/MINER) | PARTIAL Basis da |
| State Partitioning | Partitionierter Ledger | SQLite (unpartitioniert) | PARTIAL Geplant |
| Cross-Shard Communication | Atomarer Datenaustausch | Keine Cross-Shard-TX aktuell | PARTIAL Geplant |
| Skalierbarkeit | Linear mit Node-Anzahl | Vertikal (Block-Groesse) | PARTIAL Geplant |
| Ressourceneffizienz | Nur relevanten State halten | FULL-Nodes halten alles | PARTIAL Basis da |
| Sicherheit/ZK-Proofs | Kryptografische Beweise | ECDSA-Signaturen (Basis) | PARTIAL Geplant |
| Merkle-Proof Cross-Shard | State-Verifikation | Merkle-Tree in Blocks (ATC-0004) | PARTIAL Basis da |

> **Fazit:** Die Node-Typ-Architektur (FULL/LIGHT) ist die Vorstufe zu
> Sharding. Die aktuelle Chain ist unpartitioniert (jeder FULL-Node hat alles).
> ATC-07 definiert den Uebergang zu partitioniertem State mit Cross-Shard-TX
> und kryptografischen Beweisen — die grosste Skalierungshuerde.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #8 | Testnet (Multi-Node) | Done | ATC-07 Multi-Node-Basis |
| #14 | Bootstrap Node | Done | ATC-07 Node-Discovery |
| #68 | DNS Seed & Discovery | Done | ATC-07 Node-Finding |
| Sprint 2.3 | Shard-Node-Typ | Geplant | ATC-07 Partition-ID |
| Sprint 2.3 | State-Partitioning in DB | Geplant | ATC-07 DB-Partitionen |
| Sprint 2.3 | Cross-Shard-TX Protocol | Geplant | ATC-07 Atomic TX |
| Sprint 2.3 | Merkle-Proof Cross-Shard | Geplant | ATC-07 ZK-Beweise |

---

## 6. Evolutionspfad: Full-Node zu Sharding

```
AKTUELL (v3.2.1)                    ZUKUNFT (v5.0+)
Full-Node (alle Daten)        ->    Shard-Node (Partition-ID)
SQLite (unpartitioniert)      ->    Partitionierte DB pro Shard
Flood-Fill (alle Peers)       ->    Shard-spezifisches Routing
Keine Cross-Shard-TX          ->    Atomic Cross-Shard Commit
Lineare Chain                 ->    DAG pro Shard + Cross-Shard-Meta
```

### Migrationsstrategie:
1. **Phase 1:** SHARD_NODE Typ in `node.py` ergaenzen (mit Partition-ID)
2. **Phase 2:** DB-Partitionierung: Tabelle pro Shard-ID
3. **Phase 3:** Cross-Shard-TX Format definieren (erweitert ATC-0003)
4. **Phase 4:** Two-Phase Commit fuer Cross-Shard-Atomicity
5. **Phase 5:** Merkle-Proof-basierte State-Verifikation zwischen Shards
6. **Phase 6:** Shard-Rebalancing (automatische Lastverteilung)

---

## 7. Verbesserungsvorschlaege (Zukunft)

- [ ] SHARD_NODE Typ in NodeType Enum ergaenzen
- [ ] Partition-ID in Node-Konfiguration (`settings.json`)
- [ ] DB-Partitionierung: Shard-ID als Primaerschluessel-Praefix
- [ ] CROSS_SHARD_TX Nachrichtentyp in ATCNet
- [ ] Two-Phase Commit: LOCK → COMMIT/ROLLBACK ueber Shards
- [ ] Merkle-Proof: Shard beweist State-Existenz ohne Full-Disclosure
- [ ] ZK-Proof Integration (ATC-05 PQC-konform)
- [ ] Shard-Rebalancing: Automatische Lastverteilung bei Ueberlast
- [ ] Cross-Shard-Konfliktloesung (Double-Spend ueber Shards hinweg)
- [ ] Shard-Explorer UI: Visuelle Partition-Topologie

---

*Dieses Dokument wurde aus der urspruenglichen Atc-07.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:31 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| assign_shard | 200 |
| cross_shard_tx | 500 |
| merge_state | 1000 |

### Sprint-Zuweisung

- **Sprint 2.2** — #84
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

5+ Unit-Tests: Shard-Assignment, Cross-Shard-Tx, State-Merge

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
