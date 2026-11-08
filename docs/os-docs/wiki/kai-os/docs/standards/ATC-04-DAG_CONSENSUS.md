# ATC-04 — DAG Consensus & Propagation
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.4 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-04
> **Referenzen:** ATC-01 (Mesh-Topologie), ATC-10 (Global Time Sync), ATC-17 (DAO Governance), ATC-0006 (Consensus Hybrid)
> **Quelldatei:** Atc-04.docx (urspruengliche Spezifikation)
> **Kategorie:** P2P Networking  
> **Tier:** Tier 1 — Blockchain Infrastructure  

---

## Abstract

ATC-04 definiert den Directed Acyclic Graph (DAG) Consensus & Propagation
innerhalb des KAI-OS. Waehrend klassische Blockchains (wie Bitcoin oder
Ethereum) Transaktionen in einer einzigen, strikten Kette aneinanderreihen,
nutzt KAI-OS einen DAG-Ansatz.

Das bedeutet, dass Transaktionen oder Ereignisse in einem "Netz" (Graph)
angeordnet sind, wobei jedes neue Ereignis auf mehrere vorherige Ereignisse
verweist.

> **Merksatz:** ATC-01 = Verbindung. ATC-02 = Gedaechtnis. ATC-03 = Identitaet.
> ATC-04 = Konsens und parallele Wahrheit.

---

## 1. Kernkonzepte

### 1.1 Parallele Verarbeitung
Da es keine einzelne Kette gibt, die als Flaschenhals fungiert, koennen mehrere
Transaktionen gleichzeitig validiert und verarbeitet werden. Dies ist fuer ein
Betriebssystem, das KI-Agenten und Echtzeit-Anwendungen unterstuetzt, zwingend
erforderlich.

**Aktueller Stand:** Die aktuelle Implementation (hybrid_consensus.py)
nutzt eine lineare Chain (PoW + PoS + PoH). Der DAG-Ansatz ist der
konzeptionelle Evolutionspfad fuer horizontale Skalierung.

### 1.2 Asynchrone Propagation
ATC-04 legt fest, wie Informationen (Events) von Node zu Node im Mesh-Netzwerk
(gemaess ATC-01) verbreitet werden. Ein Event ist erst dann "final", wenn es
eine ausreichende Anzahl von "Bestaetigungs-Referenzen" von anderen Nodes
erhalten hat.

**Aktueller Stand:** Die P2P-Propagation (p2p_propagation.py) nutzt
Flood-Fill-Broadcast mit Duplikat-Filter. Dies ist chain-basiert
(NEW_BLOCK, NEW_TX Nachrichten). Im DAG-Modus wuerden Events statt Bloecken
propagiert werden, mit Multi-Referenz-Bestaetigung statt Block-Tiefe.

### 1.3 Widerstandsfaehigkeit gegen Latenz
Im Gegensatz zu traditionellen Ketten, bei denen langsame Nodes das Netzwerk
verlangsamen, koennen bei einem DAG "schnellere" Pfade im Graph existieren,
was die Zeit bis zur Bestaetigung (Time-to-Finality) drastisch reduziert.

**PoH als Bruecke:** Proof of History (poh.py) bietet bereits eine
VDF-basierte Hash-Kette, die als zeitliche Vorordnung dient. Im DAG-Modell
koennte PoH die Event-Reihenfolge im Graph garantieren, ohne eine strikte
Blocknummerierung zu benoetigen.

---

## 2. Warum ATC-04 fuer KAI-OS essenziell ist

### 2.1 Skalierbarkeit
Durch die parallele Struktur von ATC-04 kann der Durchsatz des Netzwerks mit
der Anzahl der Nodes skalieren (Horizontal Scaling).

### 2.2 KI-Echtzeitfaehigkeit
Wenn ein KI-Agent in Tier 4 einen Entscheidungs-Trace oder eine
Inferenz-Abrechnung auf der Chain verankern will, kann er dies in den DAG
einfuegen, ohne auf den naechsten "Block-Slot" warten zu muessen.

**Aktueller Stand:** Der AI-Kernel (ai_kernel.py) loggt KI-Entscheidungen
als Hashes on-chain. Im DAG-Modell koennten KI-Events asynchron und ohne
Block-Wartezeit verankert werden.

### 2.3 Auto-Remediation Integration
Der Konsens-Algorithmus von ATC-04 erkennt Konflikte (sogenannte "Double
Spends" oder widerspruechliche Zustaende) automatisch anhand der Pfad-Struktur
im Graph und loest diese durch die vorgegebenen Konsens-Regeln auf.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-01 (Mesh-Topologie)
Bietet die physikalische Topologie fuer die Uebertragung der DAG-Events.
Das ATCNet Gossip Protocol wuerde DAG-Events statt Bloecke propagieren.

### 3.2 ATC-10 (Global Time Synchronization)
Da ein DAG keine strikte Blocknummerierung hat, ist ATC-10 entscheidend, um
den zeitlichen Ablauf der Ereignisse im Graph korrekt einzuordnen.

**Bezug zu PoH:** Proof of History (poh.py) mit SHA-3_256 VDF-Hash-Kette
bietet bereits eine verifizierbare Zeitachse. Im DAG-Modell wuerde PoH die
Event-Timestamps liefern, ATC-10 die Node-uebergreifende Synchronisation.

### 3.3 ATC-17 (DAO)
Governance-Entscheidungen nutzen die Struktur des DAGs, um Stimmen (Votes)
zu verifizieren, die auf verschiedene Aeste des Graphen verteilt sind.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| DAG-Struktur | Events als Graph mit Multi-References | Lineare Chain (PoW+PoS+PoH) | PARTIAL Konzeptionell |
| Parallele Verarbeitung | Gleichzeitige TX-Validierung | Sequentielle Block-Validierung | PARTIAL Konzeptionell |
| Asynchrone Propagation | Event-Propagation mit Refs | Flood-Fill Block-Propagation | Basis da, DAG-Modus geplant |
| Latenz-Widerstand | Schnelle Pfade im Graph | Block-Wartezeit als Faktor | PARTIAL Konzeptionell |
| Skalierbarkeit | Horizontal | Vertikal (Block-Groesse/-Zeit) | PARTIAL Konzeptionell |
| KI-Echtzeit | Events ohne Block-Slot | On-Chain Hash-Logging | Basis da |
| Auto-Remediation | Graph-Pfad-Konfliktloesung | Chain-basierte Double-Spend | Basis da |
| PoH-Integration | VDF fuer Event-Timestamps | PoH als Hash-Kette fuer Blocks | Basis da |

> **Fazit:** ATC-04 ist ein **konzeptioneller Standard** fuer die naechste
> Evolutionsstufe. Die aktuelle Chain-Architektur (PoW + PoS + PoH) ist voll
> funktionsfaehig. Der DAG-Ansatz baut auf den bestehenden Komponenten auf.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #8 | Testnet (Multi-Node) | Done | ATC-04 P2P-Propagation |
| #14 | Bootstrap Node | Done | ATC-04 Event-Uebertragung |
| #15 | Block Propagation | Done | ATC-04 Propagation-Basis |
| #68 | DNS Seed & Discovery | Done | ATC-04 Node-Finding |
| Sprint 2.4 | DAG-Consensus Implementation | Geplant | ATC-04 Vollimplementation |
| Sprint 2.4 | DAG Event-Format | Geplant | ATC-04 Event-Struktur |
| Sprint 2.4 | DAG Finality-Algorithm | Geplant | ATC-04 Bestaetigungs-Referenzen |

---

## 6. Evolutionspfad: Chain zu DAG

AKTUELL (v3.2.1) -> ZUKUNFT (v4.0.0+)
Lineare Chain (PoW+PoS+PoH) -> DAG-Graph (PoH+PoS)
Block-basiert, sequentiell -> Event-basiert, parallel
Block-Wartezeit -> Instant Finality
Vertikale Skalierung -> Horizontale Skalierung

### Migrationsstrategie:
1. Phase 1: DAG-Event-Format definieren (neben bestehenden Block-Format)
2. Phase 2: PoH als Event-Timestamp-Quelle nutzen (bestehend)
3. Phase 3: P2P-Propagation um DAG-Event-Typ erweitern
4. Phase 4: Finality-Algorithm mit Bestaetigungs-Referenzen
5. Phase 5: Hybrid-Modus (Chain fuer kritische TXs, DAG fuer KI-Events)

---

## 7. Verbesserungsvorschlaege (Zukunft)

- [ ] DAG-Event-Format definieren (event_id, references[], timestamp, payload, signature)
- [ ] DAG-Node-Datenstruktur in hybrid_consensus.py ergaenzen
- [ ] Finality-Algorithm: Event gilt als final bei N Bestaetigungs-Referenzen
- [ ] Konfliktloesung: Heaviest-Subgraph oder GHOST-Algorithm
- [ ] ATC-10 (Global Time Sync) fuer DAG-Event-Ordering spezifizieren
- [ ] KI-Event-DAG: Spezieller Sub-Graph fuer KI-Inferenz-Logging
- [ ] DAG-Explorer UI fuer visuelle Graph-Darstellung
- [ ] Benchmark: Chain vs. DAG Durchsatzvergleich

---

*Dieses Dokument wurde aus der urspruenglichen Atc-04.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:25 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| add_vertex | 200 |
| finalize_vertex | 500 |
| traverse_dag | 10/vertex |

### Sprint-Zuweisung

- **Sprint 2.6** — #78
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

5+ Unit-Tests: DAG-Construction, Traversal, Finality, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
