# ATC-21 — Holographic On-Demand Execution Engine (Wasm)
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 3.0 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-21
> **Tier:** 3 (Operating System Infrastructure) — START VON TIER 3
> **Referenzen:** ATC-14 (Deterministic Exec), ATC-22 (HAL, geplant), Issue #1 (Smart Contracts), #18 (Docker/CI)
> **Quelldatei:** Atc-21.docx (urspruengliche Spezifikation)
> **Kategorie:** OS Foundation  

---

## Abstract

Mit ATC-21 (Holographic On-Demand Execution Engine) verlassen wir die rein
oekonomische Ebene (Tier 2) und betreten Tier 3: Operating System Infrastructure.
Dies ist das Herzstueck des KAI-OS als Betriebssystem, das auf der
WebAssembly-Technologie (Wasm) basiert.

> **ATC-21 = Die Ausfuehrungs-Engine des KAI-OS.**
> Tier 1 (ATC-01 bis 10) = Infrastruktur.
> Tier 2 (ATC-11 bis 20) = Oekonomie.
> Tier 3 beginnt hier — die Betriebssystem-Infrastruktur.

---

## 1. Was bedeutet "Holographic On-Demand Execution"?

Der Begriff "holographisch" bezieht sich hier auf die Faehigkeit des
Betriebssystems, Code (Smart Contracts, KI-Algorithmen oder System-Services)
nicht als statische Datei auf jedem Node zu speichern, sondern sie bei Bedarf
in einer isolierten, effizienten Laufzeitumgebung zu instanziieren.

**Konzept:**
```
Code als "Hologramm":
  1. Code liegt im dezentralen Dateisystem (IPFS)
  2. Node braucht Code -> laedt Wasm-Bytecode on-demand
  3. Hash-Validierung (Manipulationsschutz)
  4. Instanziierung in isolierter Wasm-Sandbox
  5. Ausfuehrung mit definierten ATC-Schnittstellen
  6. Nach Ausfuehrung: Hologramm wird entladen (RAM frei)
```

---

## 2. Kernkonzepte

### 2.1 WebAssembly (Wasm) als Runtime
KAI-OS verwendet Wasm als universellen Bytecode. Da Wasm extrem performant,
sicher (Sandbox-Isolation) und plattformunabhaengig ist, koennen dieselben
Programme auf unterschiedlichster Hardware (Server, IoT-Devices, mobile
Endgeraete) identisch ausgefuehrt werden.

**Aktueller Stand:** Nicht implementiert. Aktuelle Smart Contracts laufen in
Python-Runtime (`blockchain/contracts/`). ATCLang VM (`atcvm.py`) als
proprieataere Zwischenloesung — Wasm als Ziel-Platform geplant.

```python
# atcvm.py — Aktuelle ATCLang VM (Stack-basiert, 30+ Opcodes)
# Zukunft: ATCLang -> Wasm Compiler (atc -> .wasm)
```

> **Geplant:** Wasm-Runtime als Execution Engine fuer alle Smart Contracts,
# KI-Algorithmen und System-Services.

### 2.2 On-Demand Bereitstellung
Anstatt dass ein Node die gesamte Anwendungslogik lokal vorhalten muss, laedt
die Execution Engine den benoetigten Bytecode bei Bedarf aus dem dezentralen
Dateisystem (IPFS) nach, validiert den Hash (um Manipulationen auszuschliessen)
und fuehrt ihn in der Sandbox aus.

**Bezug:** ATCFS (`atcfs.py`) als dezentrales Dateisystem. IPFS-Integration
als zukuenftige Erweiterung. Aktuell: lokale Datei-Ablage.

### 2.3 Determinismus durch Sandbox
ATC-21 stellt sicher, dass die Wasm-Instanz keinen Zugriff auf die lokale
Systemzeit oder Hardware-Ressourcen hat, ausser ueber die definierten ATC-
Schnittstellen (z. B. ATC-22 fuer Hardware-Beschleunigung). Dies garantiert,
dass die Ausfuehrung ueberall dasselbe Ergebnis liefert (ATC-14).

**Bezug:** ATC-14 (Deterministic Execution) — ATC-21 ist die physische
Ausfuehrungsumgebung, in der die Regeln von ATC-14 erzwungen werden.

---

## 3. Warum ATC-21 fuer KAI-OS essenziell ist

### 3.1 Portabilitaet
Entwickler koennen ihre Anwendungen (z. B. KI-Modelle oder Battle-Games) einmal
in Wasm kompilieren und sie laufen sofort ueberall im KAI-OS-Netzwerk.

### 3.2 Sicherheit
Da jeder Code in einer isolierten Wasm-Sandbox laeuft, kann eine schadhafte
Anwendung das Betriebssystem des Nodes nicht korrumpieren.

### 3.3 Effizienz (On-Demand)
Nodes muessen nicht mit unnoetigem Code vollgestopft werden. Nur die aktuell
benoetigten "Hologramme" (instanziierte Anwendungen) werden in den RAM geladen.

---

## 4. Zusammenhang mit anderen Standards

### 4.1 ATC-14 (Deterministic Execution)
ATC-21 ist die physikalische Ausfuehrungsumgebung, in der die Regeln von ATC-14
erzwungen werden.

### 4.2 ATC-22 (Hardware Abstraction Layer)
Waehrend ATC-21 die Logik ausfuehrt, stellt ATC-22 die Schnittstellen zur
Verfuegung, damit diese Logik auf die GPU/CPU des Nodes zugreifen kann, ohne
die Sandbox zu verlassen.

> ATC-22 ist ein zukuenftiger Standard — HAL fuer GPU/CPU Zugriff aus Sandbox.

### 4.3 Issue #1 & #18 (Smart Contract & Docker)
Die Engine in ATC-21 ist das Ziel fuer die Deployments, die in der Roadmap
geplant sind — sie sorgt dafuer, dass die Smart Contracts nicht nur "da" sind,
sondern auch korrekt "leben" und arbeiten.

**Bezug:** Issue #1 (Smart Contracts) abgeschlossen. Issue #18 (Docker/CI/CD)
abgeschlossen. Docker-Deployment als aktuelle Infrastruktur, Wasm als
zukuenftige Runtime.

---

## 5. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Wasm-Runtime | WebAssembly als Bytecode | Python-Runtime + ATCLang VM | PARTIAL Geplant |
| On-Demand Loading | IPFS + Hash-Validierung | ATCFS (lokal), IPFS geplant | PARTIAL Basis da |
| Sandbox-Isolation | Strikt isolierte Wasm-Sandbox | Python (keine Sandbox) | PARTIAL Geplant |
| Determinismus | ATC-14 Kompatibel | ATCLang VM deterministisch | PARTIAL Basis da |
| Hash-Validierung | Code-Integritaet pruefen | ECDSA + SHA-256 vorhanden | PARTIAL Basis da |
| Portabilitaet | Einmal kompilieren, ueberall laufen | Python plattformabhaengig | PARTIAL Geplant |
| ATC-22 Schnittstelle | HAL fuer GPU/CPU | ATC-22 konzeptionell | PARTIAL Geplant |
| Hologramm-Konzept | Dynamische Instanziierung | Statische Contracts | PARTIAL Geplant |

> **Fazit:** Die ATCLang VM (`atcvm.py`) als proprieataere Zwischenloesung ist
> implementiert. Der Uebergang zu Wasm als universelle Runtime ist der
> grosste Infrastruktur-Schritt in Tier 3 — er macht KAI-OS zu einem echten
> "Universal-Computer".

---

## 6. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #1 | Smart Contracts (Base) | Done | ATC-21 Contract-Ausfuehrung |
| #18 | Docker/CI-CD Pipeline | Done | ATC-21 Deployment-Infrastruktur |
| #35 | ATCLang v0.3.0 | Done | ATC-21 ATCLang VM als Basis |
| #69 | Security-Audit | Open | ATC-21 Sandbox-Sicherheit |
| Sprint 3.0 | Wasm-Runtime Engine | Geplant | ATC-21 WebAssembly Integration |
| Sprint 3.0 | ATCLang -> Wasm Compiler | Geplant | ATC-21 .atc -> .wasm |
| Sprint 3.0 | IPFS-Integration | Geplant | ATC-21 On-Demand Loading |
| Sprint 3.0 | ATC-22 HAL | Geplant | ATC-21 + ATC-22 GPU/CPU |

---

## 7. Verbesserungsvorschlaege (Zukunft)

- [ ] Wasm-Runtime: WebAssembly als universelle Execution Engine
- [ ] ATCLang -> Wasm Compiler: .atc Dateien zu .wasm kompilieren
- [ ] IPFS-Integration: Dezentrale Code-Ablage + On-Demand Loading
- [ ] Hash-Validierung: Code-Integritaet vor Ausfuehrung pruefen
- [ ] Sandbox-Isolation: Kein FS/Netzwerk/Systemzeit-Zugriff
- [ ] Gas-Limit: Maximale Ausfuehrungsschritte (ATC-14)
- [ ] ATC-22 Schnittstelle: GPU/CPU Zugriff aus Sandbox
- [ ] Hologramm-Lifecycle: Load -> Execute -> Unload
- [ ] Multi-Language Support: Rust, C++, AssemblyScript -> Wasm
- [ ] Code-Caching: Haeufig genutzte Wasm-Module cachen

---

## 8. Architektur: Aktuell vs. Ziel

```
AKTUELL (v3.0.0)                           ZIEL (v5.0+)
Python-Runtime                             Wasm-Runtime
  -> blockchain/contracts/*.py               -> .wasm Bytecode
  -> Direkte Ausfuehrung                     -> Sandbox-Isolation
  -> ATCLang VM (atcvm.py)                   -> ATCLang -> Wasm Compiler
  -> Lokale Datei-Ablage                     -> IPFS On-Demand Loading
  -> Docker-Deployment                        -> Hologramm-Instanziierung
  -> Statische Contracts                      -> Dynamische Hologramme
```

### Migrationspfad:
1. **Phase 1:** Wasm-Runtime als zusaetzliche Execution Engine (neben Python)
2. **Phase 2:** ATCLang -> Wasm Compiler (.atc -> .wasm)
3. **Phase 3:** IPFS-Integration fuer On-Demand Code-Loading
4. **Phase 4:** Sandbox-Isolation mit ATC-22 HAL Schnittstellen
5. **Phase 5:** Python-Contracts nach Wasm migrieren
6. **Phase 6:** Vollstaendige Wasm-basierte Ausfuehrung

---

## 9. Tier 3 Start

Mit ATC-21 beginnt **Tier 3 (Operating System Infrastructure)**:

```
TIER 3 — OS INFRASTRUCTURE (ATC-21+)
├── ATC-21  Holographic On-Demand Exec (Wasm)  PARTIAL KONZEPTIONELL
├── ATC-22  Hardware Abstraction Layer           GEPLANT
├── ATC-23  ...                                   GEPLANT
└── ...
```

> **Naechster Schritt:** ATC-22 (Hardware Abstraction Layer) — GPU/CPU
> Schnittstellen fuer die Wasm-Sandbox.

---

*Dieses Dokument wurde aus der urspruenglichen Atc-21.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:18 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| execute_wasm | gas-limited |
| compile | 10/byte |
| instantiate | 32000 |

### Sprint-Zuweisung

- **Sprint 2.4** — #77
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

6+ Unit-Tests: Compile, Execute, Instantiate, Gas-Out, Memory-Isolation

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
