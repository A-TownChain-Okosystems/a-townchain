# ATC-08 — Ephemeral Data Streaming Protocol
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.3 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-08
> **Referenzen:** ATC-01 (Mesh-Topologie), ATC-06 (Routing/Latenz), Issue #2 (Gemini AI)
> **Quelldatei:** Atc-08.docx (urspruengliche Spezifikation)
> **Kategorie:** Blockchain Core  
> **Tier:** Tier 1 — Blockchain Infrastructure  

---

## Abstract

ATC-08 ist das Ephemeral Data Streaming Protocol. Im Kontext des
A-TownChain-Betriebssystems (KAI-OS) ist dies ein entscheidender Standard fuer
die Handhabung von Daten, die nicht dauerhaft auf der Blockchain gespeichert
werden muessen, aber dennoch sicher, schnell und verifiziert zwischen Nodes
oder zwischen KI-Agenten und Nutzern uebertragen werden sollen.

> **Merksatz:** ATC-01 = Verbindung. ATC-02 = Gedaechtnis. ATC-03 = Identitaet.
> ATC-04 = Parallele Wahrheit. ATC-05 = Quantensicherheit. ATC-06 = Performance.
> ATC-07 = Skalierbarkeit. ATC-08 = Ephemerales Streaming.

---

## 1. Kernkonzepte

### 1.1 Nicht-persistente Datenuebertragung
Waehrend Transaktionen (Tier 2) unveraenderlich im Ledger landen, dienen
"ephemere" Daten (fluechtige Daten) oft nur einem unmittelbaren Zweck, wie z. B.
dem Streaming einer KI-Inferenz-Antwort (KAI) an den Nutzer oder dem
Austausch von Zwischenzustaenden waehrend eines laufenden Prozesses.

**Aktueller Stand:** WebSocket-Server ist implementiert (`config/settings.json`:
Port 9944, max 100 Connections). AI-Routes (`backend/api/ai_routes.py`)
liefern KI-Antworten als HTTP-Response. Echte dezentrale Streaming-Pakete
via ATCNet sind geplant.

```json
// config/settings.json
"websocket": {
    "enabled": true,
    "host": "127.0.0.1",
    "port": 9944,
    "max_connections": 100
}
```

### 1.2 Streaming-Optimierung
ATC-08 baut auf dem P2P-Protokoll (ATC-01) auf, ist aber spezifisch fuer den
kontinuierlichen Datenfluss optimiert (aehnlich wie HTTP-Streaming oder
WebSockets, aber dezentralisiert).

**Aktueller Stand:** WebSocket auf Port 9944 bietet kontinuierliche
Verbindung. Der Orchestrator (`orchestrator.py`) dispatched AI-Tasks an
verfuegbare Services. Das dezentrale Streaming (Node-to-Node Token-Fluss)
baut auf ATCNet Gossip auf — aktuell noch Flood-Fill, nicht streaming-optimiert.

### 1.3 End-to-End Verifizierung
Auch wenn die Daten ephemer sind, werden sie gemaess ATC-08 kryptografisch
signiert. Der Empfaenger kann so sicherstellen, dass das Datenpaket tatsaechlich
vom korrekten Absender stammt und waehrend der Uebertragung nicht manipuliert
wurde (trotz fehlender permanenter Speicherung auf der Chain).

**Aktueller Stand:** `signature_verify.py` Middleware verifiziert ECDSA-
Signaturen bei API-Aufrufen. Der AI-Kernel (`ai_kernel.py`) loggt
Entscheidungs-Hashes on-chain (nur Hash, nicht die vollen Daten — das ist
die "ephemere" Eigenschaft). E2E-Signatur fuer Streaming-Pakete ist geplant.

---

## 2. Warum ATC-08 fuer KAI-OS entscheidend ist

### 2.1 Entlastung der Blockchain
Wuerden alle KI-Inferenz-Ergebnisse (auch die Zwischenschritte oder reine
Chat-Logs) permanent auf der Blockchain gespeichert, wuerde das System innerhalb
kuerzester Zeit an seine Speichergrenzen stossen. ATC-08 erlaubt es, den
"Daten-Muell" zu minimieren.

**Bezug:** AI-Kernel loggt nur Hashes on-chain (`ai_kernel.py`), nicht die
vollen Inferenz-Ergebnisse. Dies ist bereits eine Form der ephemeren
Datenbehandlung — die vollen Daten fliessen ueber WebSocket/HTTP, nur der
Hash wird persistiert.

### 2.2 Echtzeit-KI-Interaktion
Fuer Anwendungen wie den "Gemini AI Streaming Chat" (siehe Issue #2 der
Roadmap) ist ATC-08 das Protokoll, das die Antwort-Tokens in Echtzeit vom
Inferenz-Node zum Frontend des Nutzers transportiert.

**Bezug:** Issue #2 (Gemini AI Integration) ist abgeschlossen. Der
Orchestrator (`orchestrator.py`) mit `TaskType.AI` dispatcht KI-Tasks.
Der LLMRouter im AI-Kernel unterstuetzt 4 HuggingFace-Modelle mit
BYOK (Bring-Your-Own-Key) System.

### 2.3 Privacy & Datenschutz
Daten, die ueber ATC-08 fliessen, hinterlassen keine dauerhafte Spur im
oeffentlichen Ledger. Dies ist besonders wichtig fuer datenschutzsensible
Interaktionen, bei denen nur das Endergebnis (und nicht der gesamte Verlauf)
auf der Chain verankert werden muss.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-01 (Mesh-Topologie)
ATC-08 nutzt das existierende Mesh-Netzwerk fuer die Zustellung der ephemeren
Pakete.

**Implementation:** ATCNet (`p2p_propagation.py`) mit Gossip Protocol.
ATC-08 wuerde einen `STREAM_DATA` Nachrichtentyp definieren, der nicht
ge-flood-fill-t wird, sondern zielgerichtet an den Empfaenger geht.

### 3.2 ATC-06 (Routing)
ATC-08 profitiert von den Latenz-Optimierungen aus ATC-06, um die Verzoegerung
beim Streaming (z. B. Sprachausgabe oder Video-Generierung) so gering wie
moeglich zu halten.

### 3.3 Issue #2 (Gemini AI)
Die Implementierung des Gemini-Streamings im KAI-OS nutzt direkt die
Schnittstellen, die durch ATC-08 definiert sind.

**Status:** Issue #2 abgeschlossen. Gemini-Integration in `orchestrator.py`
mit nativem Rate-Limit-Handling und ATCLang-Code-Generierung.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Nicht-persistente Daten | Ephemer, nicht auf Chain | WebSocket + HTTP (nicht auf Chain) | PARTIAL Implementiert |
| Streaming-Optimierung | Dezentraler kontinuierlicher Fluss | WebSocket Port 9944, 100 Connections | PARTIAL Basis da |
| E2E Verifizierung | Kryptografisch signiert auch ephemer | ECDSA-Signatur in Middleware | PARTIAL Basis da |
| Blockchain-Entlastung | Nur Hashes auf Chain | AI-Kernel loggt nur Hashes | OK Implementiert |
| Echtzeit-KI-Streaming | Token-Fluss in Echtzeit | HTTP-Response (nicht gestreamt) | PARTIAL Geplant |
| Privacy/Datenschutz | Keine Chain-Spur fuer Verlauf | Nur Hash persistiert, Daten ephemer | OK Implementiert |
| Dezentrales Streaming | Node-to-Node via Mesh | WebSocket (zentral pro Node) | PARTIAL Geplant |

> **Fazit:** Die Basis (WebSocket, AI-Routes, Hash-only-Logging) ist
> implementiert. Das echte dezentrale Streaming (Node-to-Node Token-Fluss
> mit E2E-Signatur und QoS-Priorisierung) ist der naechste Schritt.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #2 | Gemini AI Integration | Done | ATC-08 KI-Streaming-Basis |
| #50 | AI Kernel | Done | ATC-08 Hash-Only Logging |
| #60 | IPC Integration | Done | ATC-08 Inter-Process-Streaming |
| Sprint 2.3 | STREAM_DATA Nachrichtentyp | Geplant | ATC-08 Dezentrales Streaming |
| Sprint 2.3 | E2E-Signatur fuer Streaming-Pakete | Geplant | ATC-08 Verifizierung |
| Sprint 2.3 | QoS-Priorisierung fuer KI-Streams | Geplant | ATC-08 Echtzeit-Prioritaet |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] `STREAM_DATA` Nachrichtentyp in ATCNet (zielgerichtet, nicht Flood-Fill)
- [ ] Token-basiertes Streaming: Inferenz-Ergebnisse als Token-Folge senden
- [ ] E2E-Signatur pro Streaming-Paket (ECDSA, ATC-05 PQC-konform)
- [ ] QoS-Klassen: HIGH fuer KI-Inferenz-Streams, LOW fuer Status-Updates
- [ ] Streaming-Buffer pro Connection (konfigurierbar in settings.json)
- [ ] Replay-Schutz: Sequence-Numbers fuer Streaming-Pakete
- [ ] Kompression:gzip/zstd fuer grosse Streaming-Pakete
- [ ] Dezentrales WebSocket-Relay: Node-to-Node WebSocket-Tunnel
- [ ] Privacy-Layer: Optional E2E-Verschluesselung fuer sensible Streams
- [ ] Streaming-Metriken: Latenz, Throughput, Packet-Loss pro Stream

---

*Dieses Dokument wurde aus der urspruenglichen Atc-08.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:34 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| stream_open | 100 |
| stream_send | 10+1/byte |
| stream_close | 50 |

### Sprint-Zuweisung

- **Sprint 2.4** — geplant
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

4+ Unit-Tests: Stream-Open, Send, Close, Timeout

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
