# ATC-14 — Deterministic Smart Contract Execution Standard

> **Issue:** #76 | **Wiki:** Kap.39+40 | **Sprint:** 2.3
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.3 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-14
> **Tier:** 2 (Logik & Oekonomie)
> **Referenzen:** ATC-04 (DAG Consensus), ATC-10 (Global Time), ATC-21 (Wasm, geplant), Issue #1 (Base Contract)
> **Quelldatei:** Atc-14.docx (urspruengliche Spezifikation)
> **Kategorie:** Economy & Assets  

---

## Abstract

ATC-14 ist der Deterministic Smart Contract Execution Standard. Er ist das
Herzstueck der Konsistenz innerhalb des A-TownChain-Betriebssystems (KAI-OS).

In einem dezentralen Betriebssystem, in dem Tausende von Nodes dieselben
Programme ausfuehren, ist Determinismus die absolute Voraussetzung fuer
Vertrauen: Es muss garantiert sein, dass eine Berechnung auf jedem Node exakt
dasselbe Ergebnis liefert.

> **ATC-14 macht KAI-OS zu einem "globalen Computer".**
> ATC-01 bis 10 = Infrastruktur-Fundament (Tier 1).
> ATC-11 bis 13 = Oekonomie- und Asset-Logik (Tier 2).
> ATC-14 = Deterministische Ausführungsebene.

---

## 1. Kernkonzepte

### 1.1 Vollkommene Vorhersehbarkeit
ATC-14 erzwingt, dass die Ausfuehrung eines Smart Contracts keine externen,
nicht-deterministischen Einfluesse zulaesst. Funktionen wie Zufallszahlen
(ohne On-Chain-Seed), unspezifische Systemzeit-Aufrufe oder Abhaengigkeiten von
lokaler Hardware-Konfiguration sind verboten.

**Aktueller Stand:** Die Smart Contracts (`blockchain/contracts/`) sind in
Python implementiert. Python ist potenziell nicht-deterministisch (Float-
Praezision, Dictionary-Order, Zufallszahlen). Die Contracts verwenden jedoch
keine Zufallszahlen oder Systemzeit direkt — sie nutzen Block-Timestamps
(von PoH, ATC-10) als Zeitquelle.

```python
# base_contract.py — Basis mit deterministischen Eigenschaften
class BaseContract:
    def execute(self, state, input):
        # State-Transition: input + state -> new_state
        # Keine externen Einfluesse, keine Zufallszahlen
        # Block-Timestamp von PoH (ATC-10), nicht lokale Systemzeit
```

> **Status:** Basis da (keine Zufallszahlen, PoH-Timestamps). Vollstaendige
> Determinismus-Garantie erfordert Wasm-Sandbox (ATC-21).

### 1.2 State-Transition-Garantie
Jeder Schritt der Vertragsausfuehrung fuehrt zu einer praezisen, mathematisch
nachvollziehbaren Zustaendsaenderung (State Transition). Wenn zwei Nodes mit
demselben Startzustand und demselben Input beginnen, muessen sie nach der
Ausfuehrung nach ATC-14 exakt denselben Endzustand im Ledger verankert haben.

**Aktueller Stand:** State-Transitions werden in der DB gespeichert
(`backend/db/repository.py`). Die `validate_chain()` Methode in
`hybrid_consensus.py` prueft, ob alle Nodes denselben State berechnet haben.

### 1.3 Wasm-Runtime-Sandboxing
ATC-14 schreibt vor, dass Smart Contracts innerhalb einer strikt abgegrenzten
WebAssembly-Sandbox (ATC-21) ausgefuehrt werden. Diese Sandbox ist so
konfiguriert, dass sie keinen Zugriff auf "fluechtige" Systemumgebungen hat,
die das Ergebnis verfaelschen koennten.

**Aktueller Stand:** Nicht implementiert. Aktuelle Contracts laufen in
Python-Runtime (keine Sandbox). Wasm-Execution-Engine ist als ATC-21
geplant — wuerde volle Isolation und Determinismus garantieren.

> **Geplant:** Wasm-Sandbox mit:
> - Kein File-System-Zugriff
> - Kein Netzwerk-Zugriff
> - Kein Systemzeit-Zugriff (nur ATC-10 Netzwerkzeit)
> - Gas-Limit fuer Infinite-Loop-Schutz

---

## 2. Warum ATC-14 fuer KAI-OS entscheidend ist

### 2.1 Konsens-Stabilitaet
Der DAG-Konsens (ATC-04) wuerde sofort kollabieren, wenn Nodes ueber das
Ergebnis einer Vertragsausfuehrung streiten wuerden. ATC-14 eliminiert dieses
Risiko vollstaendig.

**Bezug:** `hybrid_consensus.py` mit PoW+PoS+PoH. Wenn alle Nodes denselben
Contract deterministisch ausfuehren, gibt es keinen Disput ueber das Ergebnis.

### 2.2 Proof-of-AI Validierung
Da auch KI-Modell-Inferenzen in KAI-OS wie "Smart Contracts" behandelt werden
(Tier 4), stellt ATC-14 sicher, dass Inferenz-Ergebnisse (z. B. "Welches Bild
wurde durch KI generiert?") von Auditoren reproduzierbar sind.

**Bezug:** AI-Kernel (`ai_kernel.py`) loggt KI-Entscheidungen mit Hashes
on-chain. Deterministische Inferenz-Logs sind fuer Auditoren reproduzierbar.

### 2.3 Sicherheit
Da die Ausfuehrung deterministisch ist, koennen Smart Contracts vor der
Ausfuehrung formal verifiziert werden (das bedeutet, man kann mathematisch
beweisen, dass der Code keine Fehler enthaelt), was das Risiko von Exploits
drastisch senkt.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-21 (Holographic On-Demand Execution)
ATC-14 nutzt die Wasm-Engine aus ATC-21 als Ausfuehrungsplattform.

> ATC-21 ist ein zukuenftiger Standard — definiert die Wasm-Sandbox-Engine.

### 3.2 ATC-10 (Global Time)
Damit ein Smart Contract deterministisch arbeitet, darf er nicht die lokale
Node-Zeit abfragen, sondern muss die durch ATC-10 konsensual festgelegte
"Netzwerkzeit" verwenden.

**Bezug:** PoH (`poh.py`) liefert verifizierbare Zeitstempel. Contracts
nutzen Block-Timestamps (von PoH), nicht `time.time()`.

### 3.3 Issue #1 (Smart Contract Implementation)
Die Implementierung des BaseContract in der aktuellen Roadmap (v2.1.0) ist die
praktische Umsetzung dieses ATC-14-Standards. Alle darauf aufbauenden Vertrage
erben diese deterministischen Eigenschaften.

**Status:** Issue #1 abgeschlossen. `base_contract.py` als Basis-Contract,
darauf aufbauend: Governance, Marketplace, Shivamon, ATC-89, ATC-90.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Determinismus | Keine nicht-deterministischen Einfluesse | Python-Contracts, keine Zufallszahlen | PARTIAL Basis da |
| State-Transition | Mathematisch nachvollziehbar | DB-Speicherung + validate_chain | OK Implementiert |
| Wasm-Sandbox | Strikt isolierte Runtime | Python-Runtime (keine Sandbox) | PARTIAL Geplant |
| Zeitquelle | ATC-10 Netzwerkzeit | PoH-Block-Timestamps | OK Implementiert |
| Formalverifikation | Mathematischer Beweis | Nicht implementiert | PARTIAL Geplant |
| Gas-Limit | Infinite-Loop-Schutz | Nicht implementiert | PARTIAL Geplant |
| KI-Inferenz-Determinismus | Reproduzierbare Ergebnisse | Hash-Logging on-chain | PARTIAL Basis da |
| Konsens-Stabilitaet | Kein Disput ueber Ergebnisse | validate_chain in Consensus | OK Implementiert |

> **Fazit:** Die Basis (deterministische Contracts ohne Zufallszahlen,
> PoH-Zeitstempel, State-Transitions) ist implementiert. Die volle
> Determinismus-Garantie erfordert die Wasm-Sandbox (ATC-21) mit Gas-Limit
> und Formalverifikation.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #1 | Smart Contracts (Base Contract) | Done | ATC-14 Basis-Contract |
| #9 | Governance | Done | ATC-14 deterministische Ausfuehrung |
| #13 | Marketplace | Done | ATC-14 deterministischer Handel |
| #69 | Security-Audit | Open | ATC-14 Formalverifikation |
| Sprint 2.3 | Wasm-Sandbox (ATC-21) | Geplant | ATC-14 Runtime-Isolation |
| Sprint 2.3 | Gas-Limit / Infinite-Loop-Schutz | Geplant | ATC-14 DoS-Schutz |
| Sprint 2.3 | Formalverifikation | Geplant | ATC-14 mathematischer Beweis |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Wasm-Sandbox: Smart Contracts in WebAssembly ausfuehren (ATC-21)
- [ ] Gas-Limit: Maximale Ausfuehrungsschritte pro Contract-Call
- [ ] Infinite-Loop-Schutz: Timeout + Gas-Verbrauch
- [ ] Deterministische Zufallszahlen: On-Chain-Seed (PoH + Block-Hash)
- [ ] Formalverifikation: Mathematischer Beweis fuer Contract-Sicherheit
- [ ] Kein File-System/Netzwerk-Zugriff in Sandbox
- [ ] ATC-10 Integration: Nur Netzwerkzeit, niemals lokale Zeit
- [ ] KI-Inferenz-Determinismus: Reproduzierbare Inferenz-Logs
- [ ] State-Transition-Log: Jeder Schritt hash-verankert
- [ ] Contract-Verification-Tool: Pre-Deployment Sicherheitscheck

---

*Dieses Dokument wurde aus der urspruenglichen Atc-14.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:50 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| deploy | 50000+10/byte |
| call | gas-limited |
| create | 32000 |

### Sprint-Zuweisung

- **Sprint 2.3** — #76
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

8+ Unit-Tests: Deploy, Call, Create, Revert, Gas-Out, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
