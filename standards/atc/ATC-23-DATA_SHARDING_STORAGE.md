# ATC-23 — Data-Sharding & Storage Orchestration Protocol
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.3 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-23
> **Tier:** 3 (Operating System Infrastructure)
> **Referenzen:** ATC-02 (Failover), ATC-06 (Latency), ATC-07 (State Partitioning), ATC-08 (Ephemeral Streaming), ATC-21 (Wasm), ATC-22 (HAL)
> **Quelldatei:** Atc-23.docx (urspruengliche Spezifikation)
> **Kategorie:** OS Foundation  

---

## Abstract

ATC-23 definiert das Data-Sharding & Storage Orchestration Protocol. In einem
dezentralen Betriebssystem ist die Speicherung von Daten (wie KI-Modellgewichte,
Nutzerdaten oder Betriebssystem-Binaries) eine der groessten Herausforderungen.
Da KAI-OS auf das dezentrale Dateisystem IPFS/Filecoin setzt, regelt ATC-23 die
"intelligente" Verteilung und Vorhaltung dieser Daten ueber das Netzwerk hinweg.

> **ATC-23 = Dezentrales Dateisystem-Management.**
> ATC-21 = Wasm-Laufzeitumgebung (logisch).
> ATC-22 = Hardware-Abstraktion (physisch).
> ATC-23 = Dateisystem-Treiber (Storage).

---

## 1. Kernkonzepte

### 1.1 Content-Addressing & CID-Mapping
ATC-23 legt fest, dass keine Daten ueber Pfade (z. B. /user/data), sondern
ausschliesslich ueber ihren inhaltlichen Hash (Content Identifier – CID)
adressiert werden. Dies verhindert jede Form der pfadbasierten Manipulation.

**Aktueller Stand:** Teilweise implementiert
- ATCFS (`atcfs.py`) als proprieataeres Dateisystem implementiert
- SHA-256 Hashing fuer Datei-Integritaet vorhanden
- **Geplant:** IPFS-kompatibles CID-Mapping

```python
# atcfs.py — Aktuelle ATCFS Implementierung
class ATCFS:
    def store(self, data):
        # Hash-basierte Speicherung
        cid = sha256(data)
        self.storage[cid] = data
        return cid
```

### 1.2 Dynamisches Sharding
Da ein kompletter Datensatz (z. B. ein Multi-Gigabyte KI-Modell) nicht auf einen
einzelnen Node passt, teilt ATC-23 diesen in logische "Chunks" (Fragmente) auf.
Das Betriebssystem orchestriert, welche Nodes welche Chunks vorhalten, um eine
hohe Verfuegbarkeit und schnellen Zugriff zu garantieren.

**Aktueller Stand:** Nicht implementiert. ATCFS speichert aktuell lokal ohne
Sharding. Konzept fuer verteiltes Chunking geplant.

> **Geplant:** Chunk-basiertes Sharding mit Redundanz-Verteilung

### 1.3 Redundanz-Orchestrierung
ATC-23 berechnet basierend auf dem "Replikationsfaktor" des Netzwerks, wie oft
ein Datensatz (oder ein Chunk) im Netzwerk redundant vorhanden sein muss, um
auch bei Ausfall von Nodes (Failover nach ATC-02) den Zugriff auf die Daten zu
gewaehrleisten.

**Bezug:** ATC-02 (Liquid State Migration & Failover) — Failover-Mechanismus
fuer Node-Ausfaelle. ATC-23 sorgt dafuer, dass Daten bei Ausfall verfuegbar
bleiben.

---

## 2. Warum ATC-23 fuer KAI-OS essenziell ist

### 2.1 Skalierbarkeit des Dateisystems
Durch das Sharding koennen riesige Datenmengen (wie KI-Modellgewichte) ueber
das gesamte Mesh-Netzwerk verteilt werden, anstatt ein einzelnes Geraet zu
ueberlasten.

**Bezug:** AI-Kernel (`ai_kernel.py`) — KI-Modelle als verteilte Chunks im
Netzwerk. ATC-15 Proof-of-AI Mining benoetigt schnellen Modell-Zugriff.

### 2.2 Performance bei Inferenz
Wenn eine Inferenz-Aufgabe (Tier 4) gestartet wird, sorgt ATC-23 dafuer, dass
die benoetigten Daten-Chunks moeglichst nahe am Inferenz-Node (basierend auf
der Netzwerklatenz gemaess ATC-06) verfügbar sind.

**Bezug:** ATC-06 (Latency Optimization & Routing) — Latenz-basierte
Chunk-Platzierung fuer schnelle Inferenz.

### 2.3 Integritaet
Da jeder Chunk durch einen kryptografischen Hash gesichert ist, kann der Node
beim Laden der Daten sofort pruefen, ob das Fragment korrekt und unmanipuliert
ist.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-07 (State Partitioning)
Waehrend ATC-07 den logischen Zustand (State) der Blockchain partitioniert,
regelt ATC-23 die physische Ablage und Verteilung der zugehoerigen Daten auf
der Speicherebene.

### 3.2 ATC-08 (Ephemeral Data Streaming)
Kurzlebige Daten (Streams) werden oft direkt zwischen Nodes ausgetauscht,
waehrend permanente Daten (Checkpoints, Modelle) gemaess ATC-23 im Netzwerk-
Storage verankert werden.

### 3.3 ATC-21 (Wasm Engine)
ATC-21 laedt Wasm-Bytecode on-demand aus dem Dateisystem. ATC-23 stellt sicher,
dass dieser Bytecode verfuegbar, integ und effizient abrufbar ist.

### 3.4 ATC-02 (Failover)
Bei Node-Ausfaellen sorgt ATC-23 dafuer, dass redundante Chunks auf anderen
Nodes verfuegbar bleiben — Failover auf Storage-Ebene.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Content-Addressing | CID-basierte Adressierung | ATCFS mit SHA-256 Hash | OK Implementiert |
| CID-Mapping | IPFS-kompatibel | Lokales Hash-Mapping | PARTIAL Basis da |
| Dynamisches Sharding | Chunk-basierte Verteilung | Nicht implementiert | PARTIAL Geplant |
| Redundanz-Orchestrierung | Replikationsfaktor | Nicht implementiert | PARTIAL Geplant |
| IPFS-Integration | Dezentrales Dateisystem | ATCFS (lokal) | PARTIAL Basis da |
| Latenz-basierte Platzierung | Chunks nahe am Inferenz-Node | Nicht implementiert | PARTIAL Geplant |
| Chunk-Integritaet | Hash-Verifikation pro Chunk | SHA-256 vorhanden | PARTIAL Basis da |
| Failover-Storage | Redundante Chunks bei Ausfall | ATC-02 Failover (State) | PARTIAL Basis da |
| Proof-of-Retrievability | Daten-verfuegbar-Beweis | Nicht implementiert | PARTIAL Geplant |

> **Fazit:** ATCFS als proprieataeres Dateisystem mit Content-Addressing ist
> implementiert. Die Verteilung (Sharding, Redundanz, IPFS-Integration) ist
> der naechste Schritt — essientiell fuer dezentrale KI-Modell-Speicherung.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #4 | Persistenz | Done | ATC-23 ATCFS Basis |
| #50 | AI Kernel | Done | ATC-23 KI-Modell-Storage |
| #69 | Security-Audit | Open | ATC-23 Daten-Integritaet |
| Sprint 2.3 | IPFS-Integration | Geplant | ATC-23 Dezentraler Storage |
| Sprint 2.3 | Dynamisches Sharding | Geplant | ATC-23 Chunk-Verteilung |
| Sprint 2.3 | Redundanz-Orchestrierung | Geplant | ATC-23 Replikationsfaktor |
| Sprint 2.3 | Proof-of-Retrievability | Geplant | ATC-23 Daten-Verfuegbarkeit |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] IPFS-Integration: ATCFS -> IPFS-kompatibles CID-Mapping
- [ ] Dynamisches Sharding: Grosse Dateien in Chunks aufteilen
- [ ] Redundanz-Orchestrierung: Replikationsfaktor konfigurierbar
- [ ] Latenz-basierte Platzierung: Chunks nahe am Inferenz-Node (ATC-06)
- [ ] Chunk-Verifikation: Hash-Check beim Laden jedes Chunks
- [ ] Proof-of-Retrievability: Beweis dass Node Daten wirklich hat
- [ ] Proof-of-Spacetime: Kontinuierliche Verfuegbarkeits-Pruefung
- [ ] Garbage Collection: Veraltete Chunks automatisch entfernen
- [ ] Chunk-Recovery: Bei Node-Ausfall Chunks von anderen Nodes wiederherstellen
- [ ] Storage-Market: Nodes bieten Speicherplatz gegen ATC-11 Token

---

## 7. Storage-Architektur: Aktuell vs. Ziel

```
AKTUELL (v3.0.0)                           ZIEL (v5.0+)
ATCFS (lokal)                              IPFS-Integration
  -> SHA-256 Content-Addressing              -> IPFS CID-Mapping
  -> Lokale Speicherung                      -> Dezentrale Chunk-Verteilung
  -> Kein Sharding                           -> Dynamisches Sharding
  -> Keine Redundanz                         -> Replikationsfaktor N
  -> Kein Failover auf Storage-Ebene         -> Redundante Chunks bei Ausfall
```

### Migrationspfad:
1. **Phase 1:** IPFS-Integration — ATCFS CIDs zu IPFS-CIDs kompatibel
2. **Phase 2:** Sharding — Grosse Dateien in Chunks aufteilen
3. **Phase 3:** Redundanz — Replikationsfaktor und Chunk-Verteilung
4. **Phase 4:** Latenz-Optimierung — Chunks nahe am Verbraucher
5. **Phase 5:** Proof-of-Retrievability — Daten-verfuegbar-Beweis
6. **Phase 6:** Storage-Market — Dezentraler Speicher-Handel

---

*Dieses Dokument wurde aus der urspruenglichen Atc-23.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:22 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| store | 20000/kb |
| retrieve | 100/kb |
| delete | 5000 |

### Sprint-Zuweisung

- **Sprint 2.3** — #76
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

5+ Unit-Tests: Store, Retrieve, Delete, Shard-Balance, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
