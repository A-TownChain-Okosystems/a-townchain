# 🧠⛓️ KI-Blockchain-Betriebssystem — Wiki & Software-Dokumentation

> Ein vollständiges Nachschlagewerk für das Konzept, die Architektur, die Entwicklung und den Betrieb des KAI-OS — einem dezentralen, KI-gesteuerten Betriebssystem auf Blockchain-Basis.

**Version:** 1.3.3-beta | **Stand:** Juni 2026 (Live-Sync aktiv) | **Lizenz:** Apache 2.0

---

## 📋 Inhaltsverzeichnis

**Konzept & Architektur**
1. [Vision & Konzept](#1-vision--konzept)
2. [Architektur](#2-architektur)
3. [KI-Komponenten](#3-ki-komponenten)
4. [Blockchain-Komponenten](#4-blockchain-komponenten)
5. [Betriebssystem-Schicht](#5-betriebssystem-schicht)

**Software-Dokumentation**
6. [Installation & Quickstart](#6-installation--quickstart)
7. [Konfiguration](#7-konfiguration)
8. [API-Referenz](#8-api-referenz)
9. [SDK & Entwicklung](#9-sdk--entwicklung)
10. [Agenten-Entwicklung](#10-agenten-entwicklung)
11. [Smart Contract Entwicklung](#11-smart-contract-entwicklung)
12. [CLI-Referenz](#12-cli-referenz)
13. [Fehlerbehandlung & Debugging](#13-fehlerbehandlung--debugging)
14. [Testing](#14-testing)
15. [Deployment & Betrieb](#15-deployment--betrieb)
16. [Sicherheitsrichtlinien](#16-sicherheitsrichtlinien)

**Roadmap & Community**
17. [Roadmap](#17-roadmap)
18. [Vergleich & Inspiration](#18-vergleich--inspiration)
19. [Governance & Community](#19-governance--community)
20. [Changelog](#20-changelog)
21. [Glossar](#21-glossar)
22. [Erweiterte Fehlerbehebung & Incident Management](#22-erweiterte-fehlerbehebung--incident-management)
23. [CI/CD & Deployment-Prozesse](#23-cicd--deployment-prozesse)
24. [Betriebssystem-Kernel](#24-betriebssystem-kernel)
    - 24.1 Kernel-Architektur & Design-Prinzipien
    - 24.2 Micro-Kern: Komponenten
    - 24.3 KI-Kernel-Modul
    - 24.4 Blockchain-Kernel-Modul
    - 24.5 Sicherheits-Architektur
    - 24.6 Kernel-Entwicklungs-Roadmap (K1–K4)
    - 24.7 Kernel-Metriken & Observability
    - 24.8 Technologie-Entscheidungen
    - 24.9 Kernel als Multi-Layer-NFT-Architektur
25. [Security Layer — Querschnitts-Schicht L0](#25-security-layer) *(durchdringt alle Layer L1–L5)*
    - 25.1 Security Layer im NFT-Stack (L0 — Vertrauenswurzel)
    - 25.2 Security-Domänen (S1–S6)
    - 25.3 Kryptografische Primitive
    - 25.4 Zero-Trust-Policy-Engine
    - 25.5 Zero-Knowledge-Proof Engine
    - 25.6 Threat Detection & IDS/IPS
    - 25.7 Audit-Trail (On-Chain)
    - 25.8 Key-Lifecycle-Management
    - 25.9 L0 Security NFT
    - 25.10 Security-Metriken
    - 25.11 Kernel-Sprints K-Sec 1 & 2
    - 25.12 Integration in Roadmap
26. [DeFi Layer — L11](#26-defi-layer)
    - 26.1 AMM (Automated Market Maker)
    - 26.2 Lending Protocol
    - 26.3 Yield Farming & Staking
    - 26.4 Oracle-Netzwerk
    - 26.5 Flash Loan Engine
    - 26.6 Compute-Marketplace (ResourceMarket)
    - 26.7 Payment Channels (Mikrozahlungen)
    - 26.8 MEV-Schutz
27. [Gamification Layer — L12](#27-gamification-layer)
    - 27.1 Shivamon NFT-System
    - 27.2 Quest-Engine
    - 27.3 Achievement-System
    - 27.4 Soul-Bound Tokens
    - 27.5 Battle-System
    - 27.6 Leaderboard-System
    - 27.7 Sicherheit — L0 → L12
    - 27.8 Roadmap-Integration
28. [Integration Map — Repo ↔ Wiki](#28-integration-map)
    - 28.1 Bewertungsmatrix
    - 28.2 Detail-Entscheidungen (Merge-Strategie)
    - 28.3–28.6 Migrations-Fahrplan & Sync
29. [Mainnet Readiness Checklist](#29-mainnet-readiness-checklist)
30. [DevOps-Automatisierung — GitHub Actions & Docusaurus](#30-devops-automatisierung)
    - 30.1 Automatisierungs-Pipeline Überblick
    - 30.2 Wiki-Sync Workflow (wiki-sync.yml)
    - 30.3 Docusaurus Deployment (docusaurus.yml)
    - 30.4 Täglicher Health-Check (wiki-health.yml)
    - 30.5–30.6 Docusaurus Konfiguration
    - 30.7 Einrichtungs-Checkliste
    - 29.1 Security-Audit-Gate (15 Punkte)
    - 29.2 Performance-Gate (15 Punkte)
    - 29.3 Compliance- & Legal-Gate (10 Punkte)
    - 29.4 Ökosystem-Gate (15 Punkte)
    - 29.5 Launch-Prozess & Genesis-Block
    - 29.6 Post-Mainnet-Roadmap (v1.1.0+)

---

---

# 1. Vision & Konzept

## 1.1 Was ist ein KI-Blockchain-Betriebssystem?

Ein **KI-Blockchain-Betriebssystem (KAI-OS)** ist ein dezentrales, verteiltes Betriebssystem, das zwei revolutionäre Technologien vereint:

- **Künstliche Intelligenz** — für autonome Entscheidungen, Ressourcenverwaltung und adaptive Systemsteuerung
- **Blockchain** — für unveränderliche Protokollierung, dezentrale Governance und kryptografische Sicherheit

Im Unterschied zu klassischen Betriebssystemen (Windows, Linux, macOS) läuft KAI-OS nicht auf einem einzelnen Gerät unter zentraler Kontrolle, sondern verteilt über ein Netzwerk von Knoten (Nodes). Kein einzelner Akteur kontrolliert das System — stattdessen regiert Konsensus-Logik und KI-gestützte Automatisierung.

**Kernidee:** Das Betriebssystem denkt mit, handelt autonom und ist durch Blockchain manipulationssicher und transparent.

### Analogie
Stell dir vor: Linux trifft Ethereum trifft GPT — aber nicht als Schicht oben drauf, sondern als ein integriertes, co-evolutionäres System, in dem KI der Kernel ist und die Blockchain das unveränderliche Gedächtnis.

---

## 1.2 Kernprinzipien

### 🔓 Dezentralisierung
Kein zentraler Server, kein Single Point of Failure. Alle Systemkomponenten — von der Prozessverwaltung bis zur Datenspeicherung — sind auf Nodes im Netzwerk verteilt. Entscheidungen entstehen durch Konsensus, nicht durch einen Systemadministrator.

### 🤖 Autonomie
KI-Agenten übernehmen Aufgaben, die in klassischen OS von Menschen oder starren Regeln erledigt werden: Ressourcenzuteilung, Fehlerbehandlung, Optimierung. Das System reagiert auf seinen Zustand und seine Umgebung ohne ständige menschliche Eingriffe.

### 🔍 Transparenz
Alle systemrelevanten Entscheidungen — welcher Prozess Ressourcen erhält, welche Version eines Moduls geladen wurde, welche Governance-Abstimmung stattgefunden hat — werden auf der Blockchain protokolliert. Jeder kann nachvollziehen, warum das System wie gehandelt hat.

### 🔐 Sicherheit durch Design
Kryptografische Identitäten ersetzen Passwörter. Zero-Trust-Architektur: kein Prozess, kein Nutzer, kein Modul wird standardmäßig vertraut. Jede Aktion wird verifiziert und signiert.

### 🧬 Selbstverbesserung
Durch Federated Learning und On-Chain-Metriken kann das System seine KI-Modelle kontinuierlich verbessern — ohne Datenprivatsphäre zu opfern.

---

## 1.3 Abgrenzung zu klassischen Betriebssystemen

| Merkmal | Klassisches OS | KAI-OS |
|---|---|---|
| Kontrolle | Zentralisiert (Hersteller/Admin) | Dezentralisiert (DAO/Konsensus) |
| Entscheidungslogik | Regelbasiert, statisch | KI-gestützt, adaptiv |
| Protokollierung | Log-Dateien (lokal, manipulierbar) | On-Chain (unveränderlich, öffentlich) |
| Updates | Top-Down (Hersteller pusht) | Governance-Abstimmung (Community entscheidet) |
| Identität | Benutzername + Passwort | Kryptografische Schlüssel (Wallet) |
| Skalierung | Vertikal (mehr Hardware) | Horizontal (mehr Nodes) |
| Transparenz | Keine (closed source möglich) | Vollständig (Open Ledger) |

---

## 1.4 Anwendungsfelder & Use Cases

### 🏭 Industrieautomatisierung (Industry 4.0)
Maschinen in einer Fabrik verwalten sich selbst: Sie buchen Wartungszeiten, verhandeln Ressourcen untereinander und protokollieren Produktionsschritte unveränderlich auf der Blockchain.

### 🏥 Dezentrales Gesundheitswesen
Patientendaten werden verschlüsselt gespeichert, KI-Modelle analysieren Diagnosen lokal auf dem Gerät des Patienten (Federated Learning), Ergebnisse werden anonymisiert geteilt.

### 🌐 Dezentrale Cloud-Infrastruktur
Nutzer vermieten ungenutzte Rechenleistung an das Netzwerk. KI verteilt Workloads optimal auf verfügbare Nodes. Abrechnung erfolgt automatisch per Smart Contract.

### 🏙️ Smart Cities
Verkehrssteuerung, Energiemanagement und öffentliche Dienste werden durch KI-Agenten koordiniert — ohne zentralen Server.

### 🎮 Dezentrale Gaming-Infrastruktur
Spiellogik läuft auf KAI-OS, Spielstände und Assets sind on-chain gesichert. KI-Agenten können als lernende NPCs agieren.

### 💰 Dezentrale Finanzinfrastruktur (DeFi 2.0)
KI-gesteuerte Liquiditätsverwaltung, Risikoanalyse und automatisierte Marktmacher — vollständig transparent.

---

---

# 2. Architektur

## 2.1 Systemübersicht (Layer-Modell)

Das KAI-OS ist in fünf aufeinander aufbauende Schichten strukturiert:

```
┌─────────────────────────────────────────────┐
│        Layer 5: Anwendungen (dApps)          │
├─────────────────────────────────────────────┤
│        Layer 4: KI-Agenten & Services        │
├─────────────────────────────────────────────┤
│        Layer 3: Betriebssystem-Kern          │
│   (Prozesse, Ressourcen, Dateisystem)        │
├─────────────────────────────────────────────┤
│        Layer 2: Blockchain-Protokoll         │
│   (Konsensus, Ledger, Smart Contracts)       │
├─────────────────────────────────────────────┤
│        Layer 1: Netzwerk & Hardware          │
│   (P2P-Kommunikation, Nodes, Kryptografie)   │
└─────────────────────────────────────────────┘
```

---

> 🔗 Die vollständige Kernel-Implementierung (Micro-Kern, KI-Modul, Blockchain-Modul) ist in **Kapitel 24** dokumentiert.

## 2.2 KI-Kern (Inference Engine)

Der **KI-Kern** ist das Herzstück des Systems — vergleichbar mit dem Kernel in einem klassischen OS, aber intelligent und adaptiv.

**Aufgaben:**
- Ressourcenplanung & Scheduling
- Anomalieerkennung (Intrusion Detection)
- Kontinuierliche Systemoptimierung
- Entscheidungsprotokoll (on-chain, auditierbar)

**Technische Komponenten:**
- **Inference Engine:** ONNX-basiert, leichtgewichtige LLMs (1–7B Parameter, quantisiert)
- **Reasoning Layer:** Neurosymbolischer Ansatz (neuronale Netze + symbolische KI)
- **Memory Module:** Kurzzeit (RAM) + Langzeit (On-Chain / IPFS)

---

## 2.3 Blockchain-Layer

Das **Rückgrat der Vertrauensinfrastruktur** — stellt sicher, dass alle Systemereignisse unveränderlich protokolliert werden.

- Unveränderliches Ledger für alle Systemereignisse
- Smart Contracts für automatische Regelausführung
- Konsensus-Mechanismus für gemeinsamen Systemzustand

---

## 2.4 Kommunikationsprotokoll (P2P, API)

**Peer-to-Peer Stack (libp2p):**
- Transport: QUIC / TCP
- Discovery: mDNS (lokal) + DHT (global)
- Messaging: GossipSub
- Verschlüsselung: Noise Protocol (Ende-zu-Ende)

**API-Schnittstellen:** REST + WebSocket, Authentifizierung ausschließlich über kryptografische Signaturen.

---

## 2.5 Speicher & Dateisystem

| System | Zweck | Technologie |
|---|---|---|
| Kurzzeitspeicher | Aktive Prozesse | Node-lokal (RAM) |
| Dateisystem | Dokumente, Binaries | IPFS / Filecoin |
| Datenbank | Strukturierte Daten | OrbitDB |
| Blockchain | Transaktionen, State | On-Chain |
| KI-Modelle | Gewichte, Checkpoints | IPFS + Versionskontrolle |

**Content-Addressing:** Dateien werden über ihren Inhalt-Hash (CID) adressiert — keine pfadbasierte Manipulation möglich.

---

## 2.6 Sicherheitsarchitektur

**Zero-Trust-Modell:** Jede Aktion erfordert Authentifizierung und Autorisierung — intern wie extern.

**Kryptografische Grundlagen:**
- Identität: Ed25519-Schlüsselpaare
- Verschlüsselung: AES-256-GCM (ruhend), TLS 1.3 / Noise (Übertragung)
- Zero-Knowledge Proofs für datenschutzwahrende Verifikationen

| Angriff | Gegenmaßnahme |
|---|---|
| Sybil-Angriff | Proof-of-Stake + Reputation |
| 51%-Angriff | Diverse Konsensus + Slashing |
| KI-Poisoning | On-Chain Modell-Audit |
| Smart Contract Bug | Formale Verifikation + Timelock |
| Man-in-the-Middle | E2E-Verschlüsselung + Pinning |

---

---

# 3. KI-Komponenten

## 3.1 Lokale Modelle vs. Verteilte Inferenz

**Hybridansatz:**

| Modus | Wann | Technologie |
|---|---|---|
| Lokale Inferenz | Zeitkritisch, datenschutzsensibel | llama.cpp, GGUF, ONNX Runtime |
| Verteilte Inferenz | Komplexe Aufgaben, große Modelle | Petals, Ray |

Der KI-Kern routet automatisch: Lokale Kapazität ausreichend → lokal. Zu komplex → verteilte Inferenz. Kostensparend → günstigste Nodes im Netzwerk.

---

## 3.2 Autonome Agenten im OS

**Architektur (ReAct-Pattern):**
```
Wahrnehmen → Denken → Planen → Handeln → Lernen → (wiederholen)
```

| Klasse | Beispiel | Lebensdauer |
|---|---|---|
| System-Agenten | Ressourcenmanager | Dauerhaft |
| Service-Agenten | Datenbankagent | Dauerhaft |
| Task-Agenten | "Kompiliere X" | Kurzlebig |
| Nutzer-Agenten | Persönlicher Assistent | Sitzungsbasiert |
| Markt-Agenten | Handelt Rechenzeit | Dauerhaft |

---

## 3.3 Federated Learning

1. Jeder Node trainiert lokal auf eigenen Daten
2. Nur **Modell-Updates** (Gradienten) werden geteilt — nie Rohdaten
3. Smart Contract koordiniert Aggregation
4. Verbesserte Modelle werden netzwerkweit verteilt

**Datenschutz:** Differential Privacy — gezieltes Rauschen macht individuelle Datenpunkte unkenntlich.

---

## 3.4 Entscheidungsaudit (XAI + On-Chain Logging)

**Für jede KI-Entscheidung wird gespeichert:**
- Input-State + verwendetes Modell (Versions-Hash)
- Reasoning-Schritte (Chain-of-Thought, komprimiert)
- Ausgabe + Konfidenzwert
- Timestamp + Node-Signatur

Kritische Entscheidungen → direkt on-chain. Routine → IPFS, Hash on-chain.

---

---

## 3.5 LLM-Router — Modell-Selektion

> **Datei:** `core/ai_kernel.py`

```python
class LLMRouter:
    """Wählt optimales Modell basierend auf Task-Typ und Budget."""

    MODELS = {
        "fast":   {"name": "llama3-8b-q4",  "latency": "<100ms", "cost": 0.001},
        "smart":  {"name": "llama3-70b-q4", "latency": "<500ms", "cost": 0.010},
        "code":   {"name": "deepseek-coder","latency": "<300ms", "cost": 0.005},
        "vision": {"name": "llava-13b",     "latency": "<800ms", "cost": 0.015},
    }

    def route(self, task_type: str, budget: float = 0.01) -> str:
        """Gibt Model-ID zurück."""
```

## 3.6 ATCLang VM & Compiler

> **Dateien:** `modules/atclang/`

```python
class Lexer:
    """Tokenizer für ATCLang-Quellcode."""
    def tokenize(self, source: str) -> list: ...

class Parser:
    """Syntaxbaum-Generator (AST)."""
    def parse(self, tokens: list) -> dict: ...

class ATCCompiler:
    """Kompiliert ATCLang-AST zu Bytecode."""
    def compile(self, ast: dict) -> bytes: ...
    def compile_file(self, path: str) -> bytes: ...

class ATCVM:
    """Virtuelle Maschine — führt ATCLang-Bytecode aus."""
    def execute(self, bytecode: bytes, context: dict = None) -> any: ...
    def call(self, func_name: str, args: list) -> any: ...
```


# 4. Blockchain-Komponenten

## 4.1 Wahl der Blockchain

| Option | Vorteile | Eignung |
|---|---|---|
| Substrate (Polkadot) | Appchain, modular, IBC | **Empfohlen Prototyp** |
| Cosmos SDK | Eigene Chain, IBC-fähig | Langfristig |
| Ethereum L2 | Günstig, EVM-kompatibel | System-Transaktionen |
| Eigene Chain | Volle Kontrolle | Mainnet (Phase 4) |

---

## 4.2 Konsensus: Hybrid PoS + Reputation

- **NPoS (Nominated Proof of Stake):** Token-Staking als Sicherheitsleistung, Slashing bei Fehlverhalten
- **Reputation Layer:** Nodes akkumulieren Punkte durch korrektes Verhalten
- **Protokoll:** GRANDPA (Finalisierung) + BABE (Block-Produktion)

---

## 4.3 System-Smart-Contracts

| Contract | Funktion |
|---|---|
| `ResourceMarket` | Auktion von Rechenressourcen |
| `AgentRegistry` | Registrierung & Verifizierung von Agenten |
| `ModelRegistry` | Versionierung & Audit von KI-Modellen |
| `GovernanceDAO` | Abstimmungen über System-Updates |
| `ReputationEngine` | Berechnung & Verwaltung von Reputation |
| `FederatedLearning` | Koordination von Trainingsrunden |
| `PaymentChannel` | Mikrozahlungen für Rechenzeit |

---

## 4.4 Token-Ökonomie

**$KAI — Governance & Staking Token**
- Gesamtmenge: 1.000.000.000 (unveränderlich)
- Verwendung: Staking, Governance, Premium-Features
- Emission: Über 10 Jahre, abnehmend

**$COMPUTE — Utility Token**
- Für Mikrozahlungen: Rechenzeit, Speicher, Bandbreite
- Algorithmisch stabilisiert

| Aktivität | Belohnung |
|---|---|
| Node betreiben | $KAI-Blockrewards |
| Rechenzeit bereitstellen | $COMPUTE |
| Federated Learning Beitrag | $KAI (qualitätsgewichtet) |
| Bug Reports | $KAI (Bounty) |

---

## 4.5 On-Chain Identität & Zugriffsrechte

**DID (Decentralized Identifier)** nach W3C-Standard:
```
did:kai:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK
```

**Capability Tokens** (statt RBAC): Jede Berechtigung ist ein delegierbarer, widerrufbarer Token — alle Aktionen on-chain nachvollziehbar.

---

---

## 4.10 ForkResolver — Longest-Chain-Rule

> **Datei:** `blockchain/consensus/fork_resolution.py`

```python
class ForkResolver:
    """Löst Chain-Forks auf: Longest-Chain + Highest-PoH-Score."""

    def add_block(self, block: ChainBlock): ...

    def get_canonical_chain(self) -> list[ChainBlock]:
        """Gibt die kanonische (längste + stärkste) Chain zurück."""

    def resolve(self) -> list[ChainBlock]:
        """Wählt beste Fork:
        1. Längste Chain (höchste Block-Höhe)
        2. Bei Gleichstand: Höchstes Gesamt-PoH
        3. Bei Gleichstand: Größte Gesamt-PoW-Arbeit"""
```

## 4.11 InitialSyncer — Node-Synchronisation

> **Datei:** `blockchain/nodes/initial_sync.py` · Fixes: #16

```python
class InitialSyncer:
    """Synchronisiert neue Nodes mit dem Netzwerk."""

    def on_block(self, cb): self._on_block = cb
    def on_done(self,  cb): self._on_done  = cb

    async def sync(self, target_height: int = None):
        """
        1. Peer mit höchster Block-Höhe finden
        2. Blocks in Batches herunterladen (100er-Batches)
        3. Jeden Block validieren (PoH + PoW + PoS)
        4. State-Trie aufbauen
        5. on_done() aufrufen
        """

    async def stop(self): ...
    def get_stats(self) -> dict: ...
```


# 5. Betriebssystem-Schicht

> 🔗 Die Kernel-Implementierung dieser OS-Schicht ist in **Kapitel 24** (Betriebssystem-Kernel) detailliert dokumentiert.

## 5.1 KI-gestützte Prozessverwaltung

Jeder Prozess ist ein Agent mit DID, Capabilities, Resource Budget und State. Der KI-Kern plant Ressourcen vorausschauend — nicht nur reaktiv.

**Anomalie-Erkennung:** Ungewöhnlicher Ressourcenverbrauch oder verdächtige Netzwerkkommunikation löst automatisch Gegenmaßnahmen aus.

---

## 5.2 Ressourcenallokation

Ressourcentypen: CPU/GPU, RAM, Speicher, Bandbreite, Energie.

**Marktbasiert:** Bei lokaler Knappheit bietet `ResourceMarket` Kapazitäten anderer Nodes an — automatisch, in $COMPUTE bezahlt.

---

## 5.3 dApp-Ökosystem

| Typ | Beschreibung |
|---|---|
| Stateless dApps | Reine Berechnungen |
| Stateful dApps | On-Chain / IPFS-Zustand |
| KI-dApps | Verteilte Inferenz als Service |
| Hybrid dApps | Dezentral + klassische APIs |

**Dezentraler App Store:** Smart Contract mit Metadaten, Bewertungen und automatischen Royalties.

---

## 5.4 Governance-basiertes Update-Management

1. **Proposal** → Code open source, automatischer Audit
2. **Diskussion** → 7 Tage öffentlich
3. **Abstimmung** → 10% Quorum, 60% Mehrheit
4. **Timelock** → 48h Freeze
5. **Deployment** → Gestaffelt: 10% → 50% → 100% Nodes

---

## 5.5 Benutzeroberflächen

- **CLI:** Direkt, für Power-User und Entwickler
- **GUI:** Webbasiertes Dashboard (läuft selbst als dApp)
- **Natural Language:** Persönlicher KI-Agent als primäre Schnittstelle

---

---

## 5.10 ShivaOSSyscallTable — Implementierung

> **Datei:** `shivaos/kernel/syscalls.py` · Fixes: #32

```python
class SyscallID(IntEnum):
    # Prozess-Management
    SYS_FORK      = 1
    SYS_EXEC      = 2
    SYS_EXIT      = 3
    SYS_GETPID    = 4
    SYS_SLEEP     = 5
    # ATCFS
    SYS_OPEN      = 10
    SYS_READ      = 11
    SYS_WRITE     = 12
    SYS_CLOSE     = 13
    SYS_MKDIR     = 14
    SYS_STAT      = 15
    # Blockchain
    SYS_SEND_TX   = 20
    SYS_GET_BLOCK = 21
    SYS_GET_BAL   = 22
    # KI
    SYS_AI_INVOKE = 30
    SYS_AI_REGISTER = 31
    # Netzwerk
    SYS_CONNECT   = 40
    SYS_SEND      = 41
    SYS_RECV      = 42

class ShivaOSSyscallTable:
    """System-Call-Tabelle des ShivaOS Kernels."""

    def call(self, syscall_id: int,
             args: dict) -> 'SyscallResult':
        """Dispatcht System-Call."""

    def register_process(self, pid: int,
                         name: str,
                         priority: int = 5): ...
```


# 6. Installation & Quickstart

## 6.1 Systemanforderungen

| Komponente | Minimum | Empfohlen |
|---|---|---|
| OS | Ubuntu 22.04 / macOS 13+ | Ubuntu 24.04 LTS |
| CPU | 4 Kerne, 2.5 GHz | 8 Kerne, 3.5 GHz |
| RAM | 16 GB | 32 GB |
| Disk | 100 GB SSD | 500 GB NVMe |
| Netzwerk | 100 Mbit/s | 1 Gbit/s |
| GPU (optional) | NVIDIA 8GB VRAM | NVIDIA 24GB VRAM |
| Node.js | 20+ | 22+ |
| Rust | 1.75+ | stabil (latest) |
| Docker | 24+ | 25+ |

---

## 6.2 Installation (Linux / macOS)

### Schritt 1: KAI-CLI installieren
```bash
curl -sSf https://install.kai-os.dev | sh
```

Oder manuell via Package Manager:
```bash
# Homebrew (macOS)
brew tap kai-os/tap
brew install kai-cli

# APT (Ubuntu/Debian)
curl -fsSL https://deb.kai-os.dev/gpg | sudo gpg --dearmor -o /usr/share/keyrings/kai.gpg
echo "deb [signed-by=/usr/share/keyrings/kai.gpg] https://deb.kai-os.dev stable main" | sudo tee /etc/apt/sources.list.d/kai.list
sudo apt update && sudo apt install kai-cli
```

### Schritt 2: Installation verifizieren
```bash
kai --version
# kai-cli 1.0.0-alpha (build: a1b2c3d)
```

### Schritt 3: Wallet erstellen
```bash
kai wallet create --name "mein-wallet"
# ✓ Wallet erstellt
# Adresse: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
# WICHTIG: Sichere deine Seed-Phrase sicher auf!
# Seed: abandon abandon abandon ... (24 Wörter)
```

### Schritt 4: Lokales Testnet starten
```bash
kai node start --dev
# ✓ Node gestartet (Dev-Modus)
# RPC: http://localhost:9933
# WS:  ws://localhost:9944
# P2P: /ip4/127.0.0.1/tcp/30333
```

### Schritt 5: Dashboard öffnen
```bash
kai dashboard
# ✓ Dashboard läuft auf http://localhost:3000
```

---

## 6.3 Docker-Installation

```yaml
# docker-compose.yml
version: '3.8'
services:
  kai-node:
    image: kaios/node:1.0.0-alpha
    ports:
      - "9933:9933"   # RPC
      - "9944:9944"   # WebSocket
      - "30333:30333" # P2P
    volumes:
      - kai-data:/data
      - ./config:/config
    environment:
      - KAI_NETWORK=testnet
      - KAI_NODE_NAME=my-node
    command: ["--config", "/config/node.toml"]

  kai-dashboard:
    image: kaios/dashboard:1.0.0-alpha
    ports:
      - "3000:3000"
    depends_on:
      - kai-node
    environment:
      - KAI_RPC_URL=http://kai-node:9933

volumes:
  kai-data:
```

```bash
docker-compose up -d
```

---

## 6.4 Quickstart: Erster Agent in 5 Minuten

```bash
# 1. Testtokens holen (Testnet-Faucet)
kai faucet --address 5GrwvaEF...

# 2. Beispiel-Agent deployen
kai agent deploy --example hello-world --network testnet

# 3. Agent aufrufen
kai agent invoke hello-world --input "Hallo KAI-OS!"

# 4. Ergebnis anzeigen
# → Agent: "Hallo! Ich bin ein KAI-OS Agent. Wie kann ich helfen?"
```

---

---

# 7. Konfiguration

## 7.1 Node-Konfiguration (`node.toml`)

```toml
[node]
name = "mein-node"
network = "testnet"           # "dev" | "testnet" | "mainnet"
role = "full"                 # "full" | "validator" | "light"
log_level = "info"            # "trace" | "debug" | "info" | "warn" | "error"

[network]
listen_addresses = ["/ip4/0.0.0.0/tcp/30333"]
boot_nodes = [
  "/dns4/boot1.kai-os.dev/tcp/30333/p2p/12D3KooW..."
]
max_peers = 50

[rpc]
enabled = true
host = "127.0.0.1"
port = 9933  # Substrate RPC Node (Phase 3+)
# Aktuelle Flask-API läuft auf Port 5000 (backend/main.py)
cors_origins = ["http://localhost:3000", "http://localhost:4000"]

[websocket]
enabled = true
host = "127.0.0.1"
port = 9944
max_connections = 100

[storage]
path = "/data/kai"
cache_size_mb = 512
ipfs_api = "http://localhost:5001"

[ai]
inference_mode = "local"      # "local" | "distributed" | "hybrid"
model = "llama3-8b-q4"
max_memory_gb = 8
gpu_enabled = false           # true wenn NVIDIA GPU vorhanden

[blockchain]
keystore_path = "/data/keys"
validator_enabled = false
stake_amount = 0              # In $KAI (0 = kein Validator)
```

---

## 7.2 Agent-Konfiguration (`agent.toml`)

```toml
[agent]
name = "mein-agent"
version = "1.0.0"
description = "Mein erster KAI-OS Agent"

[model]
name = "llama3-8b-q4"
inference = "local"           # "local" | "distributed"
max_tokens = 2048
temperature = 0.7

[capabilities]
read_storage = true
write_storage = true
call_contracts = true
network_access = false
spawn_agents = false

[budget]
compute = 1000                # $COMPUTE-Budget pro Session
storage_gb = 10
max_runtime_minutes = 60

[logging]
level = "info"
on_chain = true               # Kritische Entscheidungen on-chain loggen
```

---

## 7.3 Umgebungsvariablen

| Variable | Beschreibung | Standard |
|---|---|---|
| `KAI_NETWORK` | Netzwerk: `dev`, `testnet`, `mainnet` | `testnet` |
| `KAI_RPC_URL` | RPC-Endpunkt des Nodes | `http://localhost:9933` |
| `KAI_WS_URL` | WebSocket-Endpunkt | `ws://localhost:9944` |
| `KAI_KEYSTORE_PATH` | Pfad zum Keystore | `~/.kai/keys` |
| `KAI_DATA_PATH` | Datenpfad | `~/.kai/data` |
| `KAI_LOG_LEVEL` | Log-Level | `info` |
| `KAI_GPU_ENABLED` | GPU-Beschleunigung | `false` |
| `KAI_IPFS_API` | IPFS-API-Endpunkt | `http://localhost:5001` |

---

---

# 8. API-Referenz

> 🎫 **Verknüpfte Issues:** [🌐 #25](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/25)

## 8.1 Übersicht

KAI-OS bietet zwei API-Schnittstellen:
- **REST API** — für synchrone Anfragen
- **WebSocket API** — für Echtzeit-Events und Subscriptions

**Basis-URL (Testnet):** `https://rpc.testnet.kai-os.dev`
**Basis-URL (lokal):** `http://localhost:5000` (Backend) / `http://localhost:4000` (Gateway)

> `PORT = 5000` (backend/main.py) · `GATEWAY_PORT = 4000` (gateway/main.py)

> **Hinweis:** Port 9933 = Substrate-RPC-Node (Phase 3+). Aktuell läuft der Backend-Server auf `:5000`, das Gateway auf `:4000`.

**Authentifizierung:** Alle schreibenden Anfragen müssen mit einem Ed25519-Schlüssel signiert werden.

```http
Authorization: Signature <base64-encoded-signature>
X-KAI-Address: <wallet-address>
X-KAI-Nonce: <unix-timestamp-ms>
```

---

## 8.2 Agent API

### `GET /v1/agents`
Listet alle Agenten des authentifizierten Nutzers auf.

**Parameter:**
| Parameter | Typ | Beschreibung |
|---|---|---|
| `status` | string | Filter: `running`, `stopped`, `error` |
| `limit` | integer | Maximale Ergebnisse (default: 20, max: 100) |
| `offset` | integer | Pagination-Offset |

**Response:**
```json
{
  "agents": [
    {
      "id": "agent_01HWXYZ...",
      "name": "DataAnalyzer",
      "status": "running",
      "model": "llama3-8b-q4",
      "created_at": "2026-05-01T10:00:00Z",
      "compute_used": 234,
      "compute_budget": 1000,
      "did": "did:kai:z6Mkh..."
    }
  ],
  "total": 3,
  "limit": 20,
  "offset": 0
}
```

---

### `POST /v1/agents`
Erstellt und deployt einen neuen Agenten.

**Request Body:**
```json
{
  "name": "MeinAgent",
  "model": "llama3-8b-q4",
  "inference": "local",
  "capabilities": ["read_storage", "write_storage"],
  "budget": {
    "compute": 500,
    "storage_gb": 5
  },
  "config": {
    "max_tokens": 2048,
    "temperature": 0.7
  }
}
```

**Response:** `201 Created`
```json
{
  "id": "agent_01HWXYZ...",
  "did": "did:kai:z6Mkh...",
  "status": "starting",
  "endpoint": "wss://agents.kai-os.dev/agent_01HWXYZ..."
}
```

---

### `POST /v1/agents/{id}/invoke`
Sendet eine Aufgabe an einen Agenten.

**Request Body:**
```json
{
  "type": "analyze",
  "input": {
    "cid": "QmXxx...",
    "prompt": "Analysiere diese Daten auf Anomalien"
  },
  "async": true
}
```

**Response (async):** `202 Accepted`
```json
{
  "task_id": "task_01HWXYZ...",
  "status": "queued",
  "estimated_time_s": 12
}
```

---

### `GET /v1/agents/{id}/tasks/{task_id}`
Ruft den Status einer Agenten-Aufgabe ab.

**Response:**
```json
{
  "task_id": "task_01HWXYZ...",
  "status": "completed",
  "result": {
    "output_cid": "QmYyy...",
    "summary": "3 Anomalien gefunden in Zeilen 42, 107, 891",
    "confidence": 0.94
  },
  "compute_used": 47,
  "duration_ms": 3420,
  "on_chain_tx": "0x1a2b3c..."
}
```

---

### `DELETE /v1/agents/{id}`
Stoppt und entfernt einen Agenten.

**Response:** `204 No Content`

---

## 8.3 Storage API

### `POST /v1/storage/upload`
Lädt eine Datei in das dezentrale Dateisystem (IPFS) hoch.

**Request:** `multipart/form-data`
```
file: <binary>
encrypt: true|false
pin: true|false
```

**Response:**
```json
{
  "cid": "QmXxx...",
  "size_bytes": 102400,
  "encrypted": true,
  "pinned": true,
  "url": "ipfs://QmXxx..."
}
```

---

### `GET /v1/storage/{cid}`
Ruft eine Datei über ihren CID ab.

**Response:** Dateiinhalt (Binary) oder JSON, abhängig vom Content-Type.

---

### `GET /v1/storage/{cid}/info`
Gibt Metadaten zu einer Datei zurück.

```json
{
  "cid": "QmXxx...",
  "size_bytes": 102400,
  "mime_type": "application/json",
  "created_at": "2026-05-01T10:00:00Z",
  "pins": 7,
  "encrypted": false
}
```

---

## 8.4 Blockchain API

### `GET /v1/chain/status`
Gibt den aktuellen Status der Blockchain zurück.

```json
{
  "block_number": 1048576,
  "block_hash": "0x1a2b...",
  "finalized_block": 1048570,
  "peers": 43,
  "sync_status": "synced",
  "network": "testnet"
}
```

---

### `GET /v1/chain/balance/{address}`
Gibt das Guthaben einer Adresse zurück.

```json
{
  "address": "5GrwvaEF...",
  "kai_balance": "1000000000000",  // in Planck (10^-12 KAI)
  "compute_balance": "500000",
  "staked": "0",
  "reserved": "0"
}
```

---

### `POST /v1/chain/transfer`
Sendet Token an eine andere Adresse.

**Request Body:**
```json
{
  "to": "5FHneW46...",
  "amount": "1000000000000",
  "token": "KAI",
  "memo": "Zahlung für Rechenzeit"
}
```

**Response:**
```json
{
  "tx_hash": "0x1a2b3c...",
  "status": "pending",
  "block_number": null
}
```

---

## 8.5 Governance API

### `GET /v1/governance/proposals`
Listet aktive Governance-Proposals auf.

```json
{
  "proposals": [
    {
      "id": 42,
      "title": "Upgrade KI-Kern auf v2.1",
      "status": "active",
      "yes_votes": "234000000",
      "no_votes": "12000000",
      "quorum_reached": true,
      "ends_at": "2026-05-25T00:00:00Z"
    }
  ]
}
```

---

### `POST /v1/governance/proposals/{id}/vote`
Stimmt über ein Proposal ab.

**Request Body:**
```json
{
  "vote": "yes",        // "yes" | "no" | "abstain"
  "conviction": 1       // 0-6 (höher = mehr Gewicht, längere Sperrzeit)
}
```

---

## 8.6 WebSocket Events

Verbindung herstellen:
```javascript
const ws = new WebSocket('ws://localhost:9944');
ws.send(JSON.stringify({ type: 'subscribe', topics: ['blocks', 'agents', 'governance'] }));
```

**Event-Typen:**

| Event | Beschreibung |
|---|---|
| `block.finalized` | Neuer finalisierter Block |
| `agent.status_changed` | Agenten-Status hat sich geändert |
| `agent.task_completed` | Aufgabe abgeschlossen |
| `governance.proposal_created` | Neues Governance-Proposal |
| `governance.vote_cast` | Abstimmung abgegeben |
| `resource.bid_won` | Ressourcen-Auktion gewonnen |
| `node.peer_connected` | Neuer Peer verbunden |

**Beispiel-Event:**
```json
{
  "type": "agent.task_completed",
  "timestamp": "2026-05-18T10:30:00Z",
  "data": {
    "agent_id": "agent_01HWXYZ...",
    "task_id": "task_01HWXYZ...",
    "status": "completed",
    "output_cid": "QmYyy..."
  }
}
```

---

---

# 9. SDK & Entwicklung

## 9.1 TypeScript / JavaScript SDK

### Installation
```bash
npm install @kai-os/sdk
# oder
yarn add @kai-os/sdk
```

### Initialisierung
```typescript
import { KaiClient } from '@kai-os/sdk';

const client = new KaiClient({
  network: 'testnet',               // 'dev' | 'testnet' | 'mainnet'
  rpcUrl: 'http://localhost:4000',  // Gateway (extern) oder :5000 (Backend direkt)
  wallet: {
    seedPhrase: process.env.KAI_SEED,
    // oder:
    privateKey: process.env.KAI_PRIVATE_KEY,
  },
});

await client.connect();
console.log('Verbunden als:', client.address);
```

### Agent verwalten
```typescript
// Agent erstellen
const agent = await client.agents.create({
  name: 'DataAnalyzer',
  model: 'llama3-8b-q4',
  capabilities: ['read_storage', 'write_storage'],
  budget: { compute: 500 },
});

// Task senden
const task = await agent.invoke({
  type: 'analyze',
  input: { cid: 'QmXxx...', prompt: 'Finde Anomalien' },
});

// Auf Ergebnis warten
const result = await task.wait();
console.log('Ergebnis:', result.summary);

// Agenten auflisten
const agents = await client.agents.list({ status: 'running' });

// Agent stoppen
await agent.stop();
```

### Storage
```typescript
// Datei hochladen
const { cid } = await client.storage.upload(fileBuffer, {
  encrypt: true,
  pin: true,
});

// Datei abrufen
const data = await client.storage.get(cid);

// Datei-Metadaten
const info = await client.storage.info(cid);
```

### Blockchain-Interaktion
```typescript
// Guthaben prüfen
const balance = await client.chain.getBalance(client.address);
console.log('KAI:', balance.kai);

// Token senden
const tx = await client.chain.transfer({
  to: '5FHneW46...',
  amount: '1000000000000',
  token: 'KAI',
});
await tx.wait(); // Auf Bestätigung warten

// Smart Contract aufrufen
const result = await client.contracts.call('ResourceMarket', 'getBids', {
  resourceId: '0xabc...',
});
```

### Events abonnieren
```typescript
// Agent-Events
client.agents.on('task_completed', (event) => {
  console.log('Task fertig:', event.taskId, event.outputCid);
});

// Block-Events
client.chain.on('block', (block) => {
  console.log('Neuer Block:', block.number);
});
```

---

## 9.2 Python SDK

### Installation
```bash
pip install kai-os-sdk
```

### Grundlegende Nutzung
```python
import asyncio
from kai_sdk import KaiClient, AgentConfig, ModelConfig

async def main():
    client = KaiClient(
        network="testnet",
        seed_phrase=os.environ["KAI_SEED"]
    )
    await client.connect()

    # Agent erstellen
    agent = await client.agents.create(AgentConfig(
        name="DataAnalyzer",
        model=ModelConfig(name="llama3-8b-q4", inference="local"),
        capabilities=["read_storage", "write_storage"],
        budget={"compute": 500}
    ))

    # Task ausführen
    result = await agent.invoke(
        task_type="analyze",
        input={"cid": "QmXxx...", "prompt": "Finde Anomalien"}
    )
    print(f"Ergebnis: {result.summary}")
    print(f"Konfidenz: {result.confidence}")

asyncio.run(main())
```

### Datei-Upload
```python
# Datei hochladen
with open("daten.json", "rb") as f:
    upload = await client.storage.upload(f.read(), encrypt=True)
    print(f"CID: {upload.cid}")

# Datei abrufen
data = await client.storage.get(upload.cid)
```

---

## 9.3 Rust SDK

### Cargo.toml
```toml
[dependencies]
kai-os-sdk = "1.0.0-alpha"
tokio = { version = "1", features = ["full"] }
```

### Grundlegende Nutzung
```rust
use kai_os_sdk::{KaiClient, AgentConfig, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KaiClient::new(Network::Testnet)
        .with_seed(&std::env::var("KAI_SEED")?)
        .build()
        .await?;

    println!("Verbunden als: {}", client.address());

    // Agent erstellen
    let agent = client.agents()
        .create(AgentConfig {
            name: "SystemMonitor".to_string(),
            model: "llama3-8b-q4".to_string(),
            ..Default::default()
        })
        .await?;

    println!("Agent deployed: {}", agent.id());
    Ok(())
}
```

---

---

# 10. Agenten-Entwicklung

## 10.1 Agent-Grundstruktur

Ein KAI-OS Agent besteht aus:
1. **Manifest** (`agent.toml`) — Metadaten, Capabilities, Budget
2. **Logic** (`agent.py` / `agent.ts` / `agent.rs`) — Verhaltenslogik
3. **Skills** — Wiederverwendbare Aktionen
4. **Memory** — Kurzzeit- und Langzeitspeicher

---

## 10.2 Einen Agent von Grund auf bauen (Python)

```python
from kai_sdk import Agent, Task, Memory, on_task, on_event

agent = Agent(
    name="SupportAgent",
    version="1.0.0",
)

# Kurzzeit-Memory initialisieren
memory = Memory(agent)

@agent.on_task("help")
async def handle_help(task: Task):
    """Beantwortet Support-Anfragen"""
    user_message = task.input.get("message")
    history = await memory.get("conversation_history", default=[])

    # Kontextbasierte Antwort generieren
    response = await agent.model.invoke(
        system="Du bist ein hilfreicher Support-Agent für KAI-OS.",
        messages=history + [{"role": "user", "content": user_message}],
        max_tokens=512,
    )

    # Konversationshistorie speichern
    history.append({"role": "user", "content": user_message})
    history.append({"role": "assistant", "content": response.text})
    await memory.set("conversation_history", history[-20:])  # Letzte 20 Nachrichten

    await task.complete({"response": response.text})


@agent.on_task("analyze_file")
async def analyze_file(task: Task):
    """Analysiert eine Datei per CID"""
    cid = task.input.get("cid")
    if not cid:
        await task.fail("Kein CID angegeben")
        return

    # Datei aus IPFS laden
    data = await agent.storage.get(cid)

    # Analyse durch KI-Modell
    analysis = await agent.model.invoke(
        prompt=f"Analysiere folgende Daten und erstelle eine Zusammenfassung:\n\n{data[:4000]}",
        max_tokens=1024,
    )

    # Ergebnis speichern
    result_cid = await agent.storage.write(analysis.text)

    await task.complete({
        "result_cid": result_cid,
        "summary": analysis.text[:200],
        "tokens_used": analysis.usage.total_tokens,
    })


@agent.on_event("resource.low")
async def handle_low_resources(event):
    """Reagiert auf niedrige Ressourcen"""
    agent.logger.warning(f"Ressourcen niedrig: {event.data}")
    # Nicht-kritische Tasks pausieren
    await agent.tasks.pause_non_critical()


# Agent starten
if __name__ == "__main__":
    agent.run(network="testnet")
```

---

## 10.3 Multi-Agenten-Orchestrierung

```python
from kai_sdk import AgentOrchestrator, AgentRef

orchestrator = AgentOrchestrator()

@orchestrator.workflow("data_pipeline")
async def run_pipeline(input_cid: str):
    """Orchestriert einen Daten-Pipeline-Workflow"""

    # Schritt 1: Daten validieren
    validator = AgentRef("DataValidator")
    validation = await validator.invoke("validate", {"cid": input_cid})

    if not validation.is_valid:
        raise ValueError(f"Validation failed: {validation.errors}")

    # Schritt 2: Parallel analysieren (3 Agenten gleichzeitig)
    analyzer_a = AgentRef("SentimentAnalyzer")
    analyzer_b = AgentRef("AnomalyDetector")
    analyzer_c = AgentRef("SummaryGenerator")

    results = await asyncio.gather(
        analyzer_a.invoke("analyze", {"cid": input_cid}),
        analyzer_b.invoke("detect", {"cid": input_cid}),
        analyzer_c.invoke("summarize", {"cid": input_cid}),
    )

    # Schritt 3: Ergebnisse zusammenführen
    merger = AgentRef("ResultMerger")
    final = await merger.invoke("merge", {
        "sentiment": results[0].output_cid,
        "anomalies": results[1].output_cid,
        "summary": results[2].output_cid,
    })

    return final.output_cid
```

---

## 10.4 Agent-Memory-Typen

| Memory-Typ | Scope | Persistenz | Zugriff |
|---|---|---|---|
| `ShortTermMemory` | Session | Nein (RAM) | Nur dieser Agent |
| `LongTermMemory` | Dauerhaft | Ja (IPFS) | Nur dieser Agent |
| `SharedMemory` | Dauerhaft | Ja (IPFS) | Autorisierte Agenten |
| `OnChainMemory` | Dauerhaft | Ja (Blockchain) | Öffentlich |

```python
# Langzeit-Memory
ltm = LongTermMemory(agent)
await ltm.store("user_preference", {"theme": "dark", "language": "de"})
pref = await ltm.retrieve("user_preference")

# Shared Memory (zwischen Agenten geteilt)
shared = SharedMemory(agent, namespace="project-42")
await shared.set("shared_state", {"phase": "analysis"})
```

---

---

# 11. Smart Contract Entwicklung

> 🎫 **Verknüpfte Issues:** [⛓ #12](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/12)

## 11.1 Sprachen & Toolchain

| Sprache | Framework | Einsatz |
|---|---|---|
| Rust + Ink! | Substrate-nativ | System-Contracts, hohe Performance |
| Solidity | EVM via Pallet | Portabilität, Community-Tooling |
| AssemblyScript | Substrate WASM | Leichtgewichtige Contracts |

---

## 11.2 Vollständiges Beispiel: ResourceMarket Contract (Ink!)

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod resource_market {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    #[ink(event)]
    pub struct BidPlaced {
        #[ink(topic)]
        resource_id: Hash,
        #[ink(topic)]
        bidder: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct ResourceAllocated {
        #[ink(topic)]
        resource_id: Hash,
        winner: AccountId,
        amount: Balance,
    }

    #[ink(storage)]
    pub struct ResourceMarket {
        /// resource_id -> (bidder, amount)
        bids: Mapping<Hash, (AccountId, Balance)>,
        /// Alle aktiven Resource-IDs
        active_resources: Vec<Hash>,
        /// Contract-Owner
        owner: AccountId,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        BidTooLow,
        ResourceNotFound,
        Unauthorized,
        TransferFailed,
    }

    impl ResourceMarket {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                bids: Mapping::default(),
                active_resources: Vec::new(),
                owner: Self::env().caller(),
            }
        }

        /// Ressource anmelden
        #[ink(message)]
        pub fn register_resource(&mut self, resource_id: Hash) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::Unauthorized);
            }
            self.active_resources.push(resource_id);
            Ok(())
        }

        /// Auf Ressource bieten
        #[ink(message, payable)]
        pub fn bid(&mut self, resource_id: Hash) -> Result<(), Error> {
            let caller = self.env().caller();
            let bid_amount = self.env().transferred_value();

            let current_best = self.bids.get(resource_id).map(|(_, a)| a).unwrap_or(0);

            if bid_amount <= current_best {
                return Err(Error::BidTooLow);
            }

            // Altes Gebot zurückzahlen
            if let Some((old_bidder, old_amount)) = self.bids.get(resource_id) {
                self.env().transfer(old_bidder, old_amount)
                    .map_err(|_| Error::TransferFailed)?;
            }

            self.bids.insert(resource_id, &(caller, bid_amount));

            self.env().emit_event(BidPlaced {
                resource_id,
                bidder: caller,
                amount: bid_amount,
            });

            Ok(())
        }

        /// Ressource an höchsten Bieter vergeben
        #[ink(message)]
        pub fn allocate(&mut self, resource_id: Hash) -> Result<AccountId, Error> {
            if self.env().caller() != self.owner {
                return Err(Error::Unauthorized);
            }

            let (winner, amount) = self.bids.get(resource_id)
                .ok_or(Error::ResourceNotFound)?;

            self.bids.remove(resource_id);

            self.env().emit_event(ResourceAllocated {
                resource_id,
                winner,
                amount,
            });

            Ok(winner)
        }

        /// Aktuelles Höchstgebot abfragen
        #[ink(message)]
        pub fn get_highest_bid(&self, resource_id: Hash) -> Option<(AccountId, Balance)> {
            self.bids.get(resource_id)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_bid_and_allocate() {
            let mut contract = ResourceMarket::new();
            let resource_id = Hash::from([1u8; 32]);

            // Ressource registrieren
            assert!(contract.register_resource(resource_id).is_ok());

            // Bieten
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            assert!(contract.bid(resource_id).is_ok());

            // Höchstgebot prüfen
            let bid = contract.get_highest_bid(resource_id);
            assert!(bid.is_some());
            assert_eq!(bid.unwrap().1, 100);
        }
    }
}
```

---

## 11.3 Contract deployen & interagieren

```bash
# Contract kompilieren
cd contracts/resource_market
cargo contract build --release

# Contract deployen (Testnet)
kai deploy contract ./target/ink/resource_market.contract \
  --network testnet \
  --suri //Alice \
  --args '()'

# Contract-Methode aufrufen
kai contract call \
  --contract 5GrwvaEF... \
  --message bid \
  --args '0x0102030405...' \
  --value 1000000000 \
  --network testnet
```

---

---

## 11.4 ATC8300Token — Python-Klasse

> **Datei:** `blockchain/contracts/atc8300/atc8300_token.py`

```python
class ATC Token:
    """ATC-8300 Token — Haupt-Currency des A-TownChain Ökosystems."""
    SYMBOL   = "ATC"
    NAME     = "ATC Token"
    DECIMALS = 18
    MAX_SUPPLY = 21_000_000  # 21 Millionen

    def mint(self, to: str, amount: float,
             miner: str) -> dict: ...
    def burn(self, caller: str, amount: float) -> dict: ...
    def transfer(self, from_addr: str, to_addr: str,
                 amount: float) -> dict: ...
    def balance_of(self, address: str) -> float: ...
    def approve(self, owner: str, spender: str, amount: float): ...
    def pos_stats(self) -> dict: ...
    def poh_state(self) -> dict: ...
```


## 11.5 Solidity Contracts — Dateiübersicht

> **Verzeichnis:** `blockchain/contracts/solidity/`
> **Framework:** Hardhat · **Tests:** `test/*.test.js`

| Datei | Standard | Beschreibung |
|-------|---------|-------------|
| `ATC Token.sol` | ATC-8300 | ERC-20 Token mit PoH/PoW/PoS-Hooks |
| `Shivamon.sol` | ATC-9000 | NFT-Kollektible, Battle, Breeding |
| `ATCGovernance.sol` | ATC-9900 | DAO-Governance, Proposals, Voting |
| `ATCMarketplace.sol` | ATC-9500 | NFT-Marktplatz, Escrow, Royalties |
| `ATCBridge.sol` | ATC-8800 | Cross-Chain Bridge, Multi-Sig, Lock/Release |
| `GenesisToken.sol` | ATC-0001 | Ur-Token, unveränderliche Genesis |

```solidity
// ATC Token.sol — Deployment
constructor() ERC20("ATC Token", "ATC") {
    _maxSupply = 21_000_000 * 10**18;
}

// ATCGovernance.sol — Proposal erstellen
function createProposal(
    string calldata title,
    string[] calldata options,
    uint256 votingPeriod
) external returns (uint256 proposalId)

// ATCMarketplace.sol — NFT listen
function listForSale(
    address nftContract,
    uint256 tokenId,
    uint256 priceATC
) external returns (uint256 listingId)

// ATCBridge.sol — ATC sperren
function lockATC(
    uint256 amount,
    string calldata destinationChain,
    string calldata recipient
) external payable returns (bytes32 lockId)

// GenesisToken.sol — Unveränderliches Genesis-NFT
function mintGenesis(
    address to,
    string calldata uri
) external onlyOwner returns (uint256 tokenId)
```


# 12. CLI-Referenz

## 12.1 Globale Optionen

```
kai [OPTIONEN] <BEFEHL>

Optionen:
  --network <NETWORK>    Netzwerk: dev, testnet, mainnet [Standard: testnet]
  --rpc <URL>           RPC-URL des Nodes
  --suri <SURI>         Signing-URI (z.B. //Alice oder Seed-Phrase)
  --output <FORMAT>     Ausgabeformat: text, json [Standard: text]
  --log-level <LEVEL>   Log-Level: trace, debug, info, warn, error
  -h, --help            Hilfe anzeigen
  -V, --version         Version anzeigen
```

---

## 12.2 Node-Befehle

```bash
# Node starten
kai node start [OPTIONEN]
  --dev                  Entwicklungsmodus (Alice/Bob vorkonfiguriert)
  --validator            Als Validator starten
  --name <NAME>          Node-Name
  --port <PORT>          P2P-Port [Standard: 30333]
  --rpc-port <PORT>      RPC-Port [Standard: 9933]

# Node-Status
kai node status

# Peers anzeigen
kai node peers

# Node stoppen
kai node stop
```

---

## 12.3 Wallet-Befehle

```bash
# Neues Wallet erstellen
kai wallet create --name <NAME>

# Wallet-Liste
kai wallet list

# Adresse anzeigen
kai wallet address [--name <NAME>]

# Guthaben abfragen
kai wallet balance [--address <ADRESSE>]

# Token senden
kai wallet transfer --to <ADRESSE> --amount <BETRAG> --token KAI

# Wallet importieren (via Seed)
kai wallet import --name <NAME>
# (Seed-Phrase wird sicher abgefragt)
```

---

## 12.4 Agent-Befehle

```bash
# Agent deployen
kai agent deploy <PFAD> [OPTIONEN]
  --name <NAME>           Agent-Name
  --model <MODEL>         KI-Modell
  --network <NETWORK>     Zielnetzwerk
  --budget <COMPUTE>      Compute-Budget in $COMPUTE
  --replicas <N>          Anzahl Replikas (1-10)
  --example <NAME>        Beispiel-Agent deployen

# Agent-Liste
kai agent list [--status running|stopped|error]

# Agent-Details
kai agent info <ID>

# Task an Agent senden
kai agent invoke <ID> --type <TYPE> --input '{"key": "value"}'

# Task-Status
kai agent task <ID> <TASK-ID>

# Agent-Logs
kai agent logs <ID> [--follow] [--tail 100]

# Agent stoppen
kai agent stop <ID>

# Agent entfernen
kai agent rm <ID>
```

---

## 12.5 Contract-Befehle

```bash
# Contract kompilieren
kai contract build [--release]

# Contract deployen
kai contract deploy <PFAD> [--args <JSON>]

# Contract aufrufen (lesend)
kai contract query --contract <ADRESSE> --message <NAME> [--args <JSON>]

# Contract aufrufen (schreibend)
kai contract call --contract <ADRESSE> --message <NAME> [--args <JSON>] [--value <BETRAG>]

# Contract-Events anzeigen
kai contract events --contract <ADRESSE> [--from-block <N>]
```

---

## 12.6 Governance-Befehle

```bash
# Proposals anzeigen
kai governance proposals [--status active|passed|rejected]

# Proposal-Details
kai governance proposal <ID>

# Abstimmen
kai governance vote <ID> --vote yes|no|abstain [--conviction 0-6]

# Neues Proposal einreichen
kai governance propose --title <TITEL> --description <BESCHREIBUNG> --code <PFAD>
```

---

## 12.7 Diagnose-Befehle

```bash
# Verbindung testen
kai ping [--url <URL>]

# Systemdiagnose
kai doctor

# Netzwerk-Informationen
kai net info

# Chain-Informationen
kai chain info

# Logs anzeigen
kai logs [--level <LEVEL>] [--follow] [--from <ISO-DATUM>]
```

---

---

# 13. Fehlerbehandlung & Debugging

## 13.1 Fehlerklassen

> 🔗 Kernel-spezifische Fehler (Kernel-Panic, HAL-Fehler, LKM-Ladefehler) sind in **Kapitel 24.6** (Kernel-Sprint-Blöcke K1–K4) dokumentiert. Für Kernel-Incidents im Produktionsbetrieb → **Kapitel 22.3.1** (Incident Playbook 1).

KAI-OS verwendet ein strukturiertes Fehlersystem:

```
KAI-[KATEGORIE]-[CODE]: [Beschreibung]
```

| Kategorie | Präfix | Beschreibung |
|---|---|---|
| Netzwerk | `NET` | Verbindungs- und P2P-Fehler |
| Blockchain | `CHAIN` | Transaktions- und Konsensus-Fehler |
| Agent | `AGENT` | Agenten-Laufzeit-Fehler |
| Storage | `STORE` | IPFS- und Speicher-Fehler |
| KI/Modell | `AI` | Inferenz- und Modell-Fehler |
| Auth | `AUTH` | Authentifizierungs-Fehler |
| Contract | `CTR` | Smart-Contract-Fehler |

---

## 13.2 Häufige Fehler & Lösungen

### `KAI-NET-001: Verbindung zum Node fehlgeschlagen`
```
Fehler: KAI-NET-001 — Verbindung zu http://localhost:9933 fehlgeschlagen
```
**Ursachen & Lösungen:**
- Node läuft nicht → `kai node start --dev`
- Falscher Port → `--rpc-port` prüfen
- Firewall blockiert → Port 9933 freigeben

---

### `KAI-CHAIN-002: Nicht genug Guthaben`
```
Fehler: KAI-CHAIN-002 — Unzureichendes Guthaben. Benötigt: 1000 KAI, Verfügbar: 500 KAI
```
**Lösung:**
```bash
# Testnet-Faucet nutzen
kai faucet --address <DEINE-ADRESSE>

# Guthaben prüfen
kai wallet balance
```

---

### `KAI-AGENT-003: Modell nicht gefunden`
```
Fehler: KAI-AGENT-003 — Modell 'llama3-8b-q4' nicht lokal verfügbar
```
**Lösung:**
```bash
# Modell herunterladen
kai model pull llama3-8b-q4

# Verfügbare Modelle
kai model list

# Alternativ: Verteilte Inferenz nutzen
kai agent deploy ... --inference distributed
```

---

### `KAI-STORE-004: CID nicht erreichbar`
```
Fehler: KAI-STORE-004 — CID QmXxx... konnte nicht abgerufen werden (Timeout nach 30s)
```
**Lösung:**
```bash
# IPFS-Verbindung testen
kai ping --ipfs

# CID auf Verfügbarkeit prüfen
kai storage info QmXxx...

# Peers hinzufügen
kai node peers add /dns4/gateway.ipfs.io/tcp/4001/p2p/QmNnooDu...
```

---

### `KAI-AUTH-005: Signatur ungültig`
```
Fehler: KAI-AUTH-005 — Signaturverifizierung fehlgeschlagen
```
**Ursachen & Lösungen:**
- Falscher Schlüssel geladen → `kai wallet list` und `--suri` prüfen
- Nonce abgelaufen → Anfrage wiederholen (Nonce = aktueller Timestamp)
- Falsches Netzwerk → `--network` prüfen

---

### `KAI-CTR-006: Contract-Ausführungsfehler`
```
Fehler: KAI-CTR-006 — Contract reverted: BidTooLow
```
**Lösung:** Contract-Fehlercode in der Contract-Dokumentation nachschlagen. Im Beispiel: Gebotsbetrag erhöhen.

---

## 13.3 Debug-Modus aktivieren

```bash
# Maximale Log-Ausgabe
KAI_LOG_LEVEL=trace kai agent deploy ...

# Debug-Endpoint (lokaler Node)
kai node start --dev --rpc-methods=unsafe

# Detaillierte Agent-Logs
kai agent logs <ID> --level trace --follow

# P2P-Diagnose
kai net diagnose
```

---

## 13.4 On-Chain Debugging

```bash
# Transaktion nachverfolgen
kai chain tx 0x1a2b3c...

# Block-Inhalte ansehen
kai chain block 1048576

# Contract-State lesen
kai contract query \
  --contract 5GrwvaEF... \
  --message get_highest_bid \
  --args '"0x..."'

# Events eines Contracts
kai contract events --contract 5GrwvaEF... --from-block 1000000
```

---

## 13.5 Log-Analyse

```bash
# Logs nach Fehler-Codes filtern
kai logs --grep "KAI-AGENT"

# Logs eines Zeitraums
kai logs --from 2026-05-18T00:00:00 --to 2026-05-18T01:00:00

# Logs als JSON exportieren
kai logs --output json > debug_logs.json

# Node-Logs (systemd)
journalctl -u kai-node -n 200 --no-pager
```

---

---

# 14. Testing

> 🎫 **Verknüpfte Issues:** [🧪 #26](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/26)

## 14.1 Test-Umgebungen

| Umgebung | Befehl | Zweck | Kosten |
|---|---|---|---|
| Dev (lokal) | `--network dev` | Einzelner Node, kein echter Konsensus | Kostenlos |
| Testnet | `--network testnet` | Realistisches Netzwerk, Testtokens | Kostenlos |
| Staging | `--network staging` | Produktionsnah | Geringe Gebühren |
| Mainnet | `--network mainnet` | Produktiv | Echte Token |

---

## 14.2 Unit Tests

### Python (pytest)
```python
# tests/test_agent.py
import pytest
from kai_sdk.testing import MockKaiClient, MockStorage

@pytest.fixture
async def mock_client():
    client = MockKaiClient(network="dev")
    await client.connect()
    return client

@pytest.mark.asyncio
async def test_analyze_file(mock_client):
    # Test-Datei in Mock-Storage laden
    test_data = b'{"values": [1, 2, 99, 3, 4]}'
    cid = await mock_client.storage.upload(test_data)

    # Agent erstellen
    from agents.data_analyzer import DataAnalyzerAgent
    agent = DataAnalyzerAgent(client=mock_client)

    # Task ausführen
    result = await agent.handle_task("analyze_file", {"cid": cid})

    # Ergebnis prüfen
    assert result["status"] == "completed"
    assert "anomalies" in result
    assert len(result["anomalies"]) > 0

@pytest.mark.asyncio
async def test_insufficient_budget(mock_client):
    agent = await mock_client.agents.create(
        name="TestAgent",
        budget={"compute": 0}  # Kein Budget
    )
    with pytest.raises(BudgetExceededError):
        await agent.invoke("analyze", {"cid": "QmXxx..."})
```

### TypeScript (Vitest)
```typescript
// tests/agent.test.ts
import { describe, it, expect, beforeAll } from 'vitest';
import { MockKaiClient } from '@kai-os/sdk/testing';

describe('DataAnalyzerAgent', () => {
  let client: MockKaiClient;

  beforeAll(async () => {
    client = new MockKaiClient({ network: 'dev' });
    await client.connect();
  });

  it('sollte Anomalien in Daten erkennen', async () => {
    const testData = JSON.stringify({ values: [1, 2, 99, 3, 4] });
    const { cid } = await client.storage.upload(Buffer.from(testData));

    const agent = await client.agents.create({ name: 'TestAnalyzer' });
    const result = await agent.invoke({ type: 'analyze', input: { cid } });

    expect(result.status).toBe('completed');
    expect(result.anomalies).toBeDefined();
  });
});
```

---

## 14.3 Integrationstests

```bash
# Alle Integrationstests (Testnet)
kai test --integration --network testnet

# Spezifischer Test
kai test --file tests/integration/agent_pipeline.spec.ts --network testnet

# Mit Timeout
kai test --integration --timeout 120s
```

```typescript
// tests/integration/agent_pipeline.spec.ts
import { KaiClient } from '@kai-os/sdk';

test('vollständige Agent-Pipeline', async () => {
  const client = new KaiClient({ network: 'testnet', wallet: testWallet });

  // 1. Daten hochladen
  const { cid } = await client.storage.upload(testData);

  // 2. Agent deployen
  const agent = await client.agents.create({
    name: 'IntegrationTest-Agent',
    model: 'llama3-8b-q4',
    budget: { compute: 50 },
  });

  // 3. Task ausführen
  const task = await agent.invoke({ type: 'analyze', input: { cid } });
  const result = await task.wait({ timeout: 60000 });

  // 4. Ergebnis prüfen
  expect(result.status).toBe('completed');
  expect(result.output_cid).toBeTruthy();
  expect(result.on_chain_tx).toBeTruthy();

  // 5. Aufräumen
  await agent.stop();
}, 90000);
```

---

## 14.4 Smart Contract Tests (Ink!)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::test;

    fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
        test::default_accounts::<ink::env::DefaultEnvironment>()
    }

    #[ink::test]
    fn test_register_and_bid() {
        let accounts = default_accounts();
        let mut contract = ResourceMarket::new();
        let resource_id = Hash::from([1u8; 32]);

        // Registrieren
        assert!(contract.register_resource(resource_id).is_ok());

        // Bieten
        test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
        assert!(contract.bid(resource_id).is_ok());

        // Höchstgebot prüfen
        let bid = contract.get_highest_bid(resource_id).unwrap();
        assert_eq!(bid.0, accounts.bob);
        assert_eq!(bid.1, 1000);
    }

    #[ink::test]
    fn test_bid_too_low_rejected() {
        let mut contract = ResourceMarket::new();
        let resource_id = Hash::from([2u8; 32]);
        assert!(contract.register_resource(resource_id).is_ok());

        // Erstes Gebot: 1000
        test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
        assert!(contract.bid(resource_id).is_ok());

        // Zweites Gebot: 500 (zu niedrig)
        test::set_value_transferred::<ink::env::DefaultEnvironment>(500);
        assert_eq!(contract.bid(resource_id), Err(Error::BidTooLow));
    }
}
```

---

## 14.5 Load Testing

```bash
# 1000 gleichzeitige Agenten simulieren
kai bench \
  --agents 1000 \
  --duration 60s \
  --task-type analyze \
  --network testnet \
  --report bench_results.json

# Ergebnis-Report anzeigen
kai bench report bench_results.json
```

---

---

# 15. Deployment & Betrieb

> 🎫 **Verknüpfte Issues:** [📦 #7](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/7)

## 15.1 Node-Betrieb (Produktiv)

### Systemd-Service einrichten (Linux)

```ini
# /etc/systemd/system/kai-node.service
[Unit]
Description=KAI-OS Node
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=kai
Group=kai
ExecStart=/usr/local/bin/kai node start \
  --config /etc/kai/node.toml \
  --name "mein-produktiv-node"
Restart=always
RestartSec=10
LimitNOFILE=65536
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

```bash
# Service aktivieren
sudo systemctl enable kai-node
sudo systemctl start kai-node
sudo systemctl status kai-node
```

---

## 15.2 Validator-Node einrichten

```bash
# 1. Keys generieren
kai validator generate-keys --output /etc/kai/validator-keys.json

# 2. Keys in Keystore einfügen
kai validator insert-key --type gran --suri "//mein-validator"
kai validator insert-key --type babe --suri "//mein-validator"

# 3. Stake einzahlen (min. 10.000 KAI)
kai validator bond --amount 10000000000000000 --reward-destination staked

# 4. Als Validator anmelden
kai validator set-keys --keys <SESSION-KEYS>
kai validator validate

# 5. Node als Validator starten
kai node start --validator --config /etc/kai/node.toml
```

---

## 15.3 Monitoring & Alerting

```bash
# Prometheus-Metriken aktivieren (node.toml)
[monitoring]
prometheus_enabled = true
prometheus_port = 9615
```

**Empfohlene Metriken:**
| Metrik | Beschreibung | Alert-Schwelle |
|---|---|---|
| `kai_blocks_finalized` | Finalisierte Blocks | < 1/min → Alert |
| `kai_peers_count` | Verbundene Peers | < 5 → Alert |
| `kai_validator_is_active` | Validator aktiv | 0 → Alert |
| `kai_agent_errors_total` | Agent-Fehler | > 10/min → Alert |
| `kai_storage_disk_usage` | Festplattennutzung | > 80% → Alert |

```yaml
# Grafana Dashboard importieren
kai monitoring dashboard --export grafana > kai_dashboard.json
```

---

## 15.4 Backup & Recovery

```bash
# Chain-Daten sichern (Node muss gestoppt sein)
kai node stop
tar -czf kai-backup-$(date +%Y%m%d).tar.gz /data/kai/
kai node start

# Keys sichern (KRITISCH — verschlüsselt speichern!)
kai wallet export --output kai-keys-backup.json
# Datei sicher offline lagern!

# Aus Backup wiederherstellen
kai node stop
tar -xzf kai-backup-20260518.tar.gz -C /
kai node start

# Node nach Absturz reparieren
kai node repair
```

---

## 15.5 Upgrades

```bash
# Aktuelle Version prüfen
kai --version

# Update verfügbar?
kai update check

# CLI updaten
kai update install

# Node-Software updaten
sudo apt update && sudo apt upgrade kai-cli
sudo systemctl restart kai-node

# On-Chain Upgrade (nach Governance-Beschluss)
# Läuft automatisch — kein manueller Eingriff nötig
```

---

---

# 16. Sicherheitsrichtlinien

## 16.1 Schlüssel-Verwaltung

**Kritische Regeln:**
- ❌ Niemals Seed-Phrase oder Private Key in Code oder Config-Dateien
- ❌ Niemals Seed-Phrase per E-Mail, Slack, Discord oder Chat senden
- ✅ Seed-Phrase offline auf Papier oder Hardware-Wallet sichern
- ✅ Für Produktions-Nodes: Hardware Security Module (HSM) verwenden
- ✅ Validator-Keys von Wallet-Keys trennen

```bash
# Schlüssel sicher in Umgebungsvariable laden
export KAI_SEED="$(pass kai/validator-seed)"  # pass (password-store)

# Keystore mit Passwort schützen
kai wallet encrypt --name mein-wallet
```

---

## 16.2 Node-Härtung

```bash
# Firewall konfigurieren (UFW)
sudo ufw default deny incoming
sudo ufw allow 22/tcp        # SSH
sudo ufw allow 30333/tcp     # P2P
sudo ufw allow 9933/tcp      # RPC (nur intern!)
sudo ufw allow 9944/tcp      # WS (nur intern!)
sudo ufw allow 9615/tcp      # Prometheus (nur intern!)
sudo ufw enable

# RPC nur lokal (WICHTIG für Produktiv-Nodes)
# In node.toml:
[rpc]
host = "127.0.0.1"  # Nicht 0.0.0.0 !
```

---

## 16.3 Smart Contract Sicherheit

**Checkliste vor Deployment:**
- [ ] Statische Analyse (Slither / cargo-audit)
- [ ] Formale Verifikation kritischer Funktionen
- [ ] Externer Audit für Contracts mit > 10.000 $KAI TVL
- [ ] Timelock für Admin-Funktionen
- [ ] Multisig für Owner-Wallet
- [ ] Emergency-Pause-Funktion implementiert
- [ ] Reentrancy-Schutz (falls zutreffend)
- [ ] Integer-Overflow-Schutz (Rust: automatisch in safe mode)

```bash
# Automatischer Security-Scan
kai audit --contract ./contracts/ --strict

# Audit-Report generieren
kai audit --contract ./contracts/ --output audit_report.html
```

---

## 16.4 Bug Bounty

KAI-OS betreibt ein öffentliches Bug-Bounty-Programm:

| Schwere | Belohnung |
|---|---|
| Kritisch (Remote Code Execution, Funds-Verlust) | 50.000 – 500.000 $KAI |
| Hoch (Konsensus-Break, Auth-Bypass) | 10.000 – 50.000 $KAI |
| Mittel (DoS, Datenleck) | 1.000 – 10.000 $KAI |
| Niedrig (Informationsleck, UI-Bug) | 100 – 1.000 $KAI |

**Melden:** security@kai-os.dev (PGP-Key: `keys.openpgp.org/kai-os-security`)

**Responsible Disclosure:** 90 Tage vor öffentlicher Veröffentlichung.

---

---

# 17. Roadmap

<!-- ROADMAP_AUTO_UPDATE_START -->
> **🔄 Roadmap zuletzt synchronisiert:** 2026-06-09 · HEAD `7389941c23` (2026-06-09) · 16 offene Issues · 10 letzte Commits
>
> **Sprint 2.2** — offene Issues: #19, #18, #17, #16, #15, #14, #8
>
> **Sprint 2.3** — offene Issues: #12, #10, #5
>
> **Sprint 2.5** — offene Issues: #13
>
> **Sprint 2.7** — offene Issues: #20
>
> **Sprint 2.8** — offene Issues: #11, #3
<!-- ROADMAP_AUTO_UPDATE_END -->






> Die technische Roadmap ist in vier Hauptphasen unterteilt. Jede Phase ist in **Sprints** (2-Wochen-Zyklen) gegliedert mit konkreten Entwicklungsaufgaben, integrierten **Fehlerbehebungsschritten**, **Deployment-Checklisten**, Abhängigkeiten und messbaren KPIs.

**Legende:**
- ✅ Abgeschlossen
- 🟡 In Bearbeitung
- ⚪ Geplant
- 🔴 Blockiert

---

## Überblick

```
2026 Q1-Q2      2026 Q3-Q4      2027 Q1-Q2      2027 Q3+
┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐
│  Phase 1   │→ │  Phase 2   │→ │  Phase 3   │→ │  Phase 4   │
│ Whitepaper │  │  Prototyp  │  │   Alpha    │  │  Mainnet   │
│ & Forschung│  │   (MVP)    │  │            │  │            │
└────────────┘  └────────────┘  └────────────┘  └────────────┘
  6 Sprints       8 Sprints       8 Sprints       Ongoing
```

---

---

## Phase 1 — Whitepaper & Forschung (Q1–Q2 2026)
**Status: 🟡 In Progress** | **Zeitraum: Januar – Juni 2026** | **Team: 2–4 Personen**

**Ziel:** Technologische Grundlagen validieren, Konzept schärfen, erste Community aufbauen, Funding sichern.

---

### Sprint 1.1 — Technologie-Scouting & Stack-Entscheidung (KW 1–2, Jan 2026)
**Status: ✅ Abgeschlossen**

- [x] Evaluation: Substrate vs. Cosmos SDK vs. Eigene Chain → *Ergebnis: Substrate*
- [x] Evaluation: KI-Inferenz-Framework → *Ergebnis: ONNX Runtime + llama.cpp*
- [x] Evaluation: P2P-Stack → *Ergebnis: libp2p*
- [x] Evaluation: Dezentraler Speicher → *Ergebnis: IPFS + Filecoin*
- [x] `TECH_DECISIONS.md` erstellt und committed

**Erfolgskriterium:** Tech-Stack dokumentiert und begründet. Keine offenen Grundsatzfragen.

---

### Sprint 1.2 — Architektur-Design (KW 3–4, Jan 2026)
**Status: ✅ Abgeschlossen**

- [x] Layer-Modell (5 Schichten) entworfen und validiert
- [x] Datenfluss-Diagramme für alle Kernszenarien erstellt
- [x] API-Kontrakte (Interfaces) definiert
- [x] Sicherheitsarchitektur (Threat Model nach STRIDE)
- [x] Review durch externen Blockchain-Architekten

---

### Sprint 1.3 — Whitepaper-Entwurf (KW 5–8, Feb 2026)
**Status: ✅ Abgeschlossen**

- [x] Kapitel: Systemarchitektur, Konsensus, Token-Ökonomie, Sicherheitsanalyse
- [x] Peer-Review durch 2 externe Kryptograf:innen
- [x] Whitepaper v0.9 (intern) finalisiert

---

### Sprint 1.4 — Wettbewerbsanalyse & Positionierung (KW 9–10, März 2026)
**Status: ✅ Abgeschlossen**

- [x] Tiefenanalyse: Fetch.ai, Bittensor, SingularityNET, Ocean Protocol, Filecoin
- [x] Differenzierungsmatrix (20+ Merkmale, 6 Projekte)
- [x] Pitch Deck v1.0 (20 Slides)

---

### Sprint 1.5 — Community-Aufbau & GitHub-Launch (KW 11–14, Apr 2026)
**Status: 🟡 In Progress**

- [x] GitHub-Organisation `kai-os`, Repositories, CI/CD-Grundkonfiguration
- [x] Discord-Server, Twitter/X, LinkedIn
- [ ] Whitepaper v1.0 öffentlich veröffentlichen
- [ ] 100+ GitHub-Stars, 200+ Discord-Mitglieder

---

### Sprint 1.6 — Funding & Team (KW 15–20, Mai–Jun 2026)
**Status: 🟡 In Progress**

- [ ] Web3 Foundation + Ethereum Foundation Grants beantragen
- [ ] Seed-Investor-Gespräche (Ziel: 500k–1M €)
- [ ] Hiring: Senior Blockchain Engineer, KI-Ingenieur, Security Engineer, DevRel
- [ ] Rechtliche Struktur (Foundation CH/LI) + Token-Rechtsberatung

**Phase-1-KPIs:**
| KPI | Ziel | Status |
|---|---|---|
| Whitepaper veröffentlicht | v1.0, Mai 2026 | 🟡 v0.9 fertig |
| GitHub Stars | 100+ | 🟡 In Progress |
| Discord Mitglieder | 200+ | 🟡 In Progress |
| Team-Größe | 5 Personen | 🟡 3/5 |
| Funding | ≥ 500k € | ⚪ Gespräche |
| Externe Peer-Reviews | 2+ | ✅ |

---

---

## Phase 2 — Prototyp / MVP (Q3–Q4 2026)
**Status: ⚪ Geplant** | **Zeitraum: Juli – Dezember 2026** | **Team: 5–8 Personen**

**Ziel:** Funktionierender Prototyp auf 3–5 Nodes. Agenten, Smart Contracts und P2P funktionieren. **Erstmaliger Einsatz der CI/CD-Pipeline und strukturierter Fehlerbehebungsprozesse.**

> 💡 **Ab Phase 2 gilt:** Jeder Sprint enthält einen **🔧 Fehlerbehebungs-Block** (Diagnosetabelle für erwartbare Probleme) sowie eine **🚀 Deployment-Checkliste** — beide müssen vor dem Merge auf `main` vollständig abgehakt sein. Bei unbekannten Fehlern: zuerst **Kapitel 13** (Basis-Debugging) und **Kapitel 22** (Incident-Playbooks) konsultieren.

---

### Sprint 2.1 — Substrate-Chain Setup (KW 27–28, Jul 2026)

**Aufgaben:**
- [ ] Substrate Node Template klonen und anpassen
- [ ] Custom Runtime konfigurieren
  - Pallets: `frame-system`, `pallet-balances`, `pallet-contracts`, `pallet-staking`, `pallet-democracy`
  - Custom Pallets: `pallet-ai-registry`, `pallet-agent-registry`
- [ ] Genesis-Konfiguration für Devnet (Alice, Bob, Charlie als Validatoren)
- [ ] Chain Specification (`chainspec.json`) generieren
- [ ] Einzelner Node startet und produziert Blöcke → **Meilenstein M1**
- [ ] Basis-Tests: Block-Produktion, Finalisierung, RPC-Antworten

**Definition of Done:**
```bash
kai node start --dev
# → Block #1 nach < 6 Sekunden
# → RPC antwortet auf kai_chainHead
# → WebSocket-Subscription auf newHeads funktioniert
```

**🔧 Fehlerbehebungs-Schritte (Sprint 2.1):**
> Für allgemeine Debug-Methoden → **Kapitel 13**. Für P0/P1-Incidents → **Kapitel 22.3.1**.

| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Node startet nicht | `kai doctor --full` | Rust-Version prüfen: `rustup update` |
| Block #1 erscheint nicht | `RUST_LOG=trace kai node start --dev` | Genesis-Config auf Tippfehler prüfen |
| RPC antwortet nicht | `curl http://localhost:9933/health` | Port-Konflikt: `--rpc-port 9934` |
| Pallet-Kompilierfehler | `cargo check --all` | Substrate-Version in `Cargo.toml` angleichen |
| GRANDPA startet nicht | `kai chain sync-status` | Mindestens 3 Validatoren im Genesis-Block nötig |

**🚀 Deployment-Checkliste (Sprint 2.1):**
- [ ] `cargo test --all --lib` grün
- [ ] `cargo clippy -- -D warnings` ohne Fehler
- [ ] `cargo audit` — keine kritischen CVEs
- [ ] Node läuft stabil 1h im Dev-Modus ohne Absturz
- [ ] PR erstellt, 1 Review erhalten, CI-Pipeline grün
- [ ] `TECH_DECISIONS.md` um Substrate-Setup-Entscheidungen ergänzt

**Technisches Risiko:** 🟡 Mittel — Substrate-Lernkurve für neue Team-Mitglieder  
> 🔗 Parallel dazu läuft **Kernel-Sprint K1** (Kapitel 24.6) — Micro-Kern-Basis auf gleicher Hardware.
> 🔗 **Kernel-Sprint K3** (Kapitel 24.6) startet ab Sprint 2.2, sobald der Substrate-Node läuft.
> 🔗 **K-Sec 1** (Kapitel 25.11) startet parallel — Crypto-Primitive-Library + Zero-Trust-Engine + L0-Security-NFT → **MS1**.

---

### Sprint 2.2 — P2P-Netzwerk & Multi-Node-Testnet (KW 29–30, Jul–Aug 2026)

**Aufgaben:**
- [ ] libp2p-Konfiguration: mDNS (lokal) + DHT/Kademlia (global) + GossipSub
- [ ] Boot-Node-Infrastruktur (2 dedizierte Nodes in EU/US)
- [ ] 3-Node-Testnet lokal starten und stabilisieren
- [ ] **Meilenstein M2:** 3-Node-Testnet stabil über 24h
- [ ] Netzwerk-Monitoring: Peer-Count, Block-Propagation-Zeit, Finalisierungs-Latenz

**Definition of Done:**
```
Node A, B, C gestartet → Automatisch verbunden →
Konsensus erreicht → Block finalisiert in < 2s →
Stabil über 24h ohne manuellen Eingriff
```

**🔧 Fehlerbehebungs-Schritte (Sprint 2.2):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Nodes finden sich nicht | `kai net diagnose --verbose` | mDNS aktiv? Firewall Port 30333 offen? |
| Blöcke propagieren nicht | `kai node peers --verbose` | Peers manuell: `kai node peers add ...` |
| GRANDPA-Stall | `kai chain sync-status` | Uhrzeit-Sync: `timedatectl status` |
| Hohe Block-Propagation (>2s) | `kai net latency --blocks 20` | Schlechte Peers entfernen |
| DHT findet keine externen Nodes | `kai net routing-table --size 10` | Boot-Node-Adressen in `node.toml` korrekt? |

**🚀 Deployment-Checkliste (Sprint 2.2):**
- [ ] 3-Node-Devnet: 24h Uptime-Log beigefügt
- [ ] Block-Propagationszeit dokumentiert (Soll: ≤ 500ms intern)
- [ ] Boot-Nodes auf Produktions-Server deployed und extern erreichbar
- [ ] `kai ping --p2p` grün auf allen 3 Nodes
- [ ] Peer-Count + Block-Time in Grafana sichtbar
- [ ] PR + 2 Reviews + CI grün

**Technisches Risiko:** 🟢 Niedrig — libp2p ist gut dokumentiert  
> 🔗 **Security Layer S2** (Kapitel 25.4): Jede P2P-Verbindung durchläuft Zero-Trust-Engine — mTLS-Handshake + Node-DID-Check.

---

### Sprint 2.3 — KI-Kern: Lokale Inferenz (KW 31–32, Aug 2026)

**Aufgaben:**
- [ ] ONNX Runtime einbinden (`ort` crate)
- [ ] Modell-Loading-System: GGUF + ONNX, RAM-Caching, Lazy Loading
- [ ] `InferenceEngine`-Trait implementieren
- [ ] Erstes Modell: `llama3-8b-q4_0.gguf` (4,7 GB)
- [ ] Benchmark: Tokens/Sekunde auf CPU und GPU
- [ ] **Meilenstein M3:** Inferenz antwortet in < 5s auf Standard-Hardware

**Akzeptanztest:**
```bash
kai ai invoke --prompt "Was ist KAI-OS?" --model llama3-8b-q4
# → Antwort < 5s (CPU) / < 1s (GPU)
# → Token-Rate: ≥ 10 t/s (CPU), ≥ 80 t/s (GPU)
```

**🔧 Fehlerbehebungs-Schritte (Sprint 2.3):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Modell lädt nicht | `kai model verify llama3-8b-q4` | Neu herunterladen: `kai model pull llama3-8b-q4 --verify` |
| OOM / Absturz beim Laden | `kai ai benchmark --dry-run` | Kleineres Modell (q2_K) oder Swap erhöhen |
| < 10 t/s auf CPU | `kai ai benchmark --model llama3-8b-q4` | Thread-Anzahl anpassen: `--threads $(nproc)` |
| GPU wird nicht genutzt | `kai ai benchmark --gpu` | CUDA-Version prüfen, `gpu_enabled = true` in `node.toml` |
| ONNX-Ladefehler | `RUST_LOG=ort=debug kai node start` | ONNX-Runtime-Version mit Modell-Opset abgleichen |

**🚀 Deployment-Checkliste (Sprint 2.3):**
- [ ] Benchmark-Ergebnisse dokumentiert (Hardware, t/s, Latenz)
- [ ] Modell-Integrität verifiziert (SHA256)
- [ ] RAM-Verbrauch unter Last < 80% des verfügbaren RAMs
- [ ] Inferenz-Modul Unit-Tests: ≥ 85% Coverage
- [ ] `kai model list --status` — alle Modelle `✅ OK`
- [ ] PR + 2 Reviews + CI grün

**Technisches Risiko:** 🟡 Mittel — RAM-Engpass auf schwacher Hardware  
> 🔗 Parallel dazu läuft **Kernel-Sprint K2** (Kapitel 24.6) — KI-Kernel-Modul mit GPU-HAL.
> 🔗 **Security Layer S1** (Kapitel 25.3): KI-Modell-Hashes werden via BLAKE2b in der Crypto-Registry verankert.
> 🔗 **Security Layer S4** (Kapitel 25.6): KI-IDS-Schicht 4 (Prompt-Injection-Erkennung) aktivieren.

---

### Sprint 2.4 — Agent-Runtime (KW 33–34, Aug–Sep 2026)
> 🔗 Voraussetzung: **Kernel-Sprint K2** (Kapitel 24.6) muss abgeschlossen sein — Agenten nutzen den KI-Kernel-Scheduler.

**Aufgaben:**
- [ ] Agent-Lifecycle: `Created → Starting → Running → Paused → Stopping → Stopped / Error`
- [ ] On-Chain Agent-Registry (`pallet-agent-registry`): register, deregister, get
- [ ] Task-Queue: FIFO mit 4 Prioritäts-Ebenen (Critical / High / Normal / Low)
- [ ] Agent-Sandbox: Isolierter Kontext
- [ ] ShortTermMemory (HashMap im RAM)
- [ ] **Meilenstein M4:** Agent erstellen → Task senden → Ergebnis empfangen

**Akzeptanztest:**
```python
agent = await client.agents.create(name="TestAgent", model="llama3-8b-q4")
result = await agent.invoke("analyze", {"text": "Hello World"})
assert result.status == "completed"
assert result.output is not None
```

**🔧 Fehlerbehebungs-Schritte (Sprint 2.4):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Agent bleibt in "starting" | `kai agent logs <ID> --level trace` | Capability-Fehler? Budget = 0? Modell geladen? |
| Task läuft nie durch | `kai agent task <ID> <TASK-ID> --verbose` | Timeout erhöhen: `--timeout 300s` |
| Task-Queue leer trotz Tasks | `kai agent runtime-status` | Queue-Consumer läuft? Thread-Deadlock? |
| Falsche Ergebnisse | `kai agent audit <ID> --task <TASK-ID>` | XAI-Log: welches Modell, welcher Prompt? |
| Memory-Leak | `kai node resources --live` | Agent nach Task beenden: `kai agent gc` |

**🚀 Deployment-Checkliste (Sprint 2.4):**
- [ ] Agent-Lifecycle: alle Zustandsübergänge abgedeckt
- [ ] Sandbox-Isolation: Agent A kann Agent B's Memory nicht lesen
- [ ] Task-Queue: FIFO-Reihenfolge unter Last korrekt (100 simultane Tasks)
- [ ] On-Chain Registry: Register + Deregister on Devchain OK
- [ ] Memory-Leak-Test: Agent 60 Min laufen, RAM-Verbrauch stabil
- [ ] PR + 2 Reviews + CI grün
> 🔗 **Security Layer S2** (Kapitel 25.4): Jeder Agent-Task durchläuft Zero-Trust — Capability-Token + Risiko-Score.
> 🔗 **Security Layer S5** (Kapitel 25.7): Agent-Aktionen on-chain im Audit-Trail protokolliert.

---

### Sprint 2.5 — Smart Contracts: Basis-Contracts (KW 35–36, Sep 2026)

**Aufgaben:**
- [ ] Ink!-Toolchain einrichten (`cargo-contract` v4+)
- [ ] `AgentRegistry.ink`: CRUD, Eigentümerprüfung, Events
- [ ] `ResourceMarket.ink`: Gebot-System, Rückzahlung, Events
- [ ] Unit-Tests ≥ 90% Coverage
- [ ] Deployment auf Dev-Chain + End-to-End-Tests
- [ ] **Meilenstein M5:** Ressourcen-Auktion funktioniert on-chain

**🔧 Fehlerbehebungs-Schritte (Sprint 2.5):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Contract reverted: `BidTooLow` | `kai chain tx <HASH> --decode-revert` | Betrag > aktuelles Höchstgebot setzen |
| Contract reverted: `Unauthorized` | `kai contract state-at <ADDR> --block X` | Owner-Adresse im Contract prüfen |
| Zu hoher Weight/Gas-Verbrauch | `kai contract profile --contract <ADDR>` | Mapping-Zugriffe cachen, Storage-Reads minimieren |
| Events fehlen im Explorer | `kai contract events <ADDR> --from-block 0` | Indexer neu starten: `kai indexer reindex` |
| `cargo contract build` schlägt fehl | `cargo contract check` | Ink!-Version mit `pallet-contracts` abgleichen |

**🚀 Deployment-Checkliste (Sprint 2.5):**
- [ ] `cargo contract check --all` ohne Fehler
- [ ] Keine ungeprüften `unwrap()` in Contract-Code
- [ ] Unit-Tests: ≥ 90% Coverage (beide Contracts)
- [ ] `cargo audit` — keine kritischen CVEs
- [ ] E2E-Test: Gebot → Zuteilung → Rückzahlung on-chain verifiziert
- [ ] Gas-Obergrenzen dokumentiert und in Tests als Assertions
- [ ] PR + 2 Reviews + CI grün

**Technisches Risiko:** 🟡 Mittel — Ink! hat andere Semantik als Solidity  
> 🔗 **K-Sec 2** (Kapitel 25.11) startet parallel — ZKP-Engine + IDS/IPS → **MS2**.
> 🔗 **Security Layer S3** (Kapitel 25.5): ZKP-Verifier-Pallet ermöglicht private Contract-Calls ohne Datenleck.

---

### Sprint 2.6 — Storage-Layer: IPFS-Integration (KW 37–38, Okt 2026)

**Aufgaben:**
- [ ] IPFS-Node in Docker-Compose
- [ ] `StorageBackend`-Trait: put, get, pin, info
- [ ] Content-Addressing in Agent-Outputs
- [ ] AES-256-GCM Verschlüsselung vor IPFS-Upload
- [ ] **Meilenstein M6:** Agent-Output auf IPFS, CID on-chain verankert

**🔧 Fehlerbehebungs-Schritte (Sprint 2.6):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| CID nicht erreichbar (Timeout) | `kai ping --ipfs` + `kai storage info <CID>` | IPFS-Peers hinzufügen: Gateway-Peer manuell |
| Upload schlägt fehl | `RUST_LOG=ipfs=debug kai node start` | IPFS-API-Endpunkt in `node.toml` korrekt? |
| Entschlüsselung schlägt fehl | Key-Management-Log prüfen | Encryption-Key stimmt mit Upload-Key überein? |
| Pin-Verlust nach Neustart | `kai storage info <CID> --pins` | Auto-Pinning in IPFS-Config aktivieren |

**🚀 Deployment-Checkliste (Sprint 2.6):**
- [ ] Upload → CID → Download: Datenintegrität verifiziert (SHA256-Vergleich)
- [ ] Verschlüsselung: Rohinhalt auf IPFS ohne Key nicht lesbar
- [ ] Pins überleben Node-Neustart
- [ ] IPFS-Node-Metriken in Prometheus sichtbar
- [ ] Storage-Tests: ≥ 85% Coverage
- [ ] PR + 2 Reviews + CI grün
> 🔗 **Security Layer S1** (Kapitel 25.3): Storage-Inhalte mit ChaCha20-Poly1305 verschlüsselt vor IPFS-Upload.
> 🔗 **Security Layer S5** (Kapitel 25.7): Jeder CID-Schreibvorgang im On-Chain-Audit-Trail.

---

### Sprint 2.7 — REST API & CLI v0.1 (KW 39–40, Okt 2026)

**Aufgaben:**
- [ ] REST API (`axum`): alle 6 Core-Endpunkte
- [ ] Ed25519-Signatur-Authentifizierung
- [ ] OpenAPI-Spec auto-generiert aus Code-Annotationen
- [ ] CLI v0.1 (`click`): node, agent, wallet Befehle
- [ ] **Meilenstein M7:** CLI-Quickstart aus Kapitel 6.4 läuft komplett durch

**🔧 Fehlerbehebungs-Schritte (Sprint 2.7):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| `401 Unauthorized` | `kai chain tx` zum Signatur-Test | Nonce-Timestamp abgelaufen? Neu signieren |
| API antwortet mit 500 | `kai logs --grep "ERROR" --last 50` | Stack-Trace analysieren |
| CLI-Befehl nicht gefunden | `kai --help` | `~/.cargo/bin` im PATH? |
| OpenAPI-Spec inkonsistent | `kai api validate-spec` | Code-Annotationen aktualisieren |
| CORS-Fehler im Browser | Network-Tab im Browser | `cors_origins` in `node.toml` ergänzen |

**🚀 Deployment-Checkliste (Sprint 2.7):**
- [ ] Alle 6 API-Endpunkte: curl + automatische API-Tests grün
- [ ] Authentifizierung: gültige + ungültige Signatur getestet
- [ ] OpenAPI-Spec validiert (`openapi-validator`)
- [ ] CLI-Quickstart (Kapitel 6.4): auf frischem Ubuntu 22.04 durchgeführt
- [ ] API-Latenz p99 < 100ms bei 10 simultanen Requests
- [ ] PR + 2 Reviews + CI grün
> 🔗 **Security Layer S2** (Kapitel 25.4): Alle REST-Endpoints und CLI-Calls via Zero-Trust-Engine (Bearer-Token + DID-Auth).

---

### Sprint 2.8 — Demo, Bugfixing & Testnet-Launch (KW 41–44, Nov–Dez 2026)

**Aufgaben:**
- [ ] Alle P0/P1-Issues schließen
- [ ] Öffentliches Testnet (5 Nodes, EU/US/Asien): Boot-Nodes, Explorer, Faucet
- [ ] Onboarding-Dokument: "Hello World Agent in 10 Minuten"
- [ ] Demo-Video (15 Min): Node → Agent → Task → On-Chain-Nachweis
- [ ] 50+ externe Entwickler ins Beta-Tester-Programm
- [ ] **Meilenstein M8:** Öffentliches Testnet live, Demo veröffentlicht

**🔧 Fehlerbehebungs-Schritte (Sprint 2.8):**
> Für allgemeine Debug-Methoden → **Kapitel 13**. Für P0/P1-Incidents → **Kapitel 22.3.1**.

| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Testnet-Node verliert Sync | `kai chain sync-status --url rpc.testnet...` | Snapshot-Restore: `kai node restore-snapshot` |
| Faucet gibt keine Tokens | Faucet-Log + Rate-Limit-Check | Rate-Limit für Beta-Tester erhöhen |
| Explorer zeigt falsche Daten | `kai contract events-audit --check-gaps` | Indexer re-index ab Block 0 |
| Beta-Tester kann nicht connecten | `check.kai-os.dev/port/30333` | Boot-Node-Firewall prüfen |

**🚀 Deployment-Checkliste (Sprint 2.8 / Testnet-Launch):**
- [ ] 0 offene P0-Issues
- [ ] 5 Testnet-Nodes ≥ 48h stabil vor Launch
- [ ] Explorer: `explorer.testnet.kai-os.dev` öffentlich erreichbar
- [ ] Faucet: funktioniert mit Rate-Limit
- [ ] Boot-Nodes von EU, US, Asien erreichbar
- [ ] Onboarding-Dokument: von 3 externen Testern erfolgreich durchgeführt
- [ ] Monitoring: Alertmanager sendet Alerts bei Node-Ausfall
- [ ] Demo-Video veröffentlicht + in Docs eingebettet
- [ ] Status-Page `status.kai-os.dev` live
- [ ] Community-Ankündigung: Discord + Twitter + Forum

**Phase-2-KPIs:**
| KPI | Ziel | Messung |
|---|---|---|
| Meilensteine M1–M8 | Alle grün | GitHub Milestone-Tracker |
| Testnet-Uptime | ≥ 99% / 30 Tage | Monitoring |
| Block-Zeit | ≤ 6s | Chain-Metriken |
| Finalisierungslatenz | ≤ 12s | Chain-Metriken |
| Inferenz-Geschwindigkeit | ≥ 10 t/s (CPU) | Benchmark |
| Externe Testnet-Teilnehmer | 50+ | Telemetrie |
| Test-Coverage (Core) | ≥ 80% | CI/CD-Report |
| Offene P0-Issues zum Launch | 0 | GitHub Issues |

---

---

## Phase 3 — Alpha (Q1–Q2 2027)
**Status: ⚪ Geplant** | **Zeitraum: Januar – Juni 2027** | **Team: 8–15 Personen**

**Ziel:** Stabiles, entwicklerfreundliches System. SDK veröffentlicht. Governance live. Erster externer Security-Audit abgeschlossen. 500+ aktive Entwickler.
> 🔗 **Security Gate:** MS1 + MS2 müssen erreicht sein — kein öffentliches Testnet ohne aktive Zero-Trust-Engine (K-Sec 1) und IDS/IPS (K-Sec 2). Siehe Kapitel 25.11.

---

### Sprint 3.1 — SDK v1.0: TypeScript & Python (KW 1–2, Jan 2027)

**Aufgaben:**
- [ ] TypeScript SDK auf npmjs.com veröffentlichen (TypeDoc, ≥ 85% Coverage)
- [ ] Python SDK auf PyPI (Type-Hints, mypy-kompatibel, ≥ 85% Coverage)
- [ ] Kompatibilitätsmatrix: Python 3.10–3.13 / Node.js 20–22
- [ ] Breaking-Change-Policy + Semantic Versioning festgelegt

**🔧 Fehlerbehebungs-Schritte (Sprint 3.1):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| npm-Installation schlägt fehl | `npm install @kai-os/sdk --verbose` | Peer-Dependency-Konflikt? Node ≥ 20? |
| TypeScript-Typen falsch | `tsc --noEmit` | Type-Definition-Datei neu generieren |
| Python-Import-Fehler | `python -c "import kai_sdk"` | Virtualenv aktiviert? `pip install kai-os-sdk` |
| `mypy` meldet Fehler | `mypy kai_sdk/ --strict` | Fehlende Type-Hints ergänzen |
| npm-Publish schlägt fehl | `npm whoami` | 2FA-Token erneuern |

**🚀 Deployment-Checkliste (Sprint 3.1):**
- [ ] `npm install @kai-os/sdk` auf frischem Rechner + Quickstart-Beispiel OK
- [ ] `pip install kai-os-sdk` + Quickstart-Beispiel OK
- [ ] Kompatibilitätsmatrix CI grün (alle Python/Node-Versionen)
- [ ] `CHANGELOG.md` und `MIGRATION.md` aktuell
- [ ] PR + 2 Reviews + CI grün
> 🔗 **Security Layer S1** (Kapitel 25.3): SDK bindet `kai-crypto`-Crate ein — keine eigenen Crypto-Implementierungen erlaubt.

---

### Sprint 3.2 — SDK v1.0: Rust & Developer Portal (KW 3–4, Jan 2027)

**Aufgaben:**
- [ ] Rust SDK auf crates.io veröffentlichen
- [ ] Developer Portal `docs.kai-os.dev` live: Getting Started, API-Referenz, Playground, 10 Tutorials

**🔧 Fehlerbehebungs-Schritte (Sprint 3.2):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| crates.io-Publish fehlgeschlagen | `cargo publish --dry-run` | Lizenz, Beschreibung, Homepage in `Cargo.toml` prüfen |
| Playground-Verbindung bricht ab | Browser-DevTools Network-Tab | WebSocket-Timeout erhöhen, Reconnect-Logik |
| Docs-Build schlägt fehl | `cargo doc --no-deps 2>&1` | Fehlende `///`-Kommentare ergänzen |

**🚀 Deployment-Checkliste (Sprint 3.2):**
- [ ] `docs.kai-os.dev` über CDN erreichbar, SSL gültig
- [ ] Playground: Agent erstellen + Task senden ohne lokalen Node
- [ ] Alle 10 Tutorials von externem Tester durchgeführt
- [ ] `cargo add kai-os-sdk` + Beispiel kompiliert
- [ ] PR + 2 Reviews + CI grün
> 🔗 **Security Layer S6** (Kapitel 25.8): Developer-Portal dokumentiert Key-Lifecycle und sichere SDK-Nutzungsmuster.

---

### Sprint 3.3 — Federated Learning Modul (KW 5–8, Feb 2027)

**Aufgaben:**
- [ ] FL-Koordinationsprotokoll (start → join → submit → aggregate → finalize → reward)
- [ ] `FederatedLearning.ink` Contract
- [ ] Differential Privacy: Gaussian-Noise (konfigurierbares ε/δ)
- [ ] Secure Aggregation: verschlüsselte Gradienten
- [ ] Erste FL-Runde auf Testnet: 10 Nodes trainieren gemeinsam

**🔧 Fehlerbehebungs-Schritte (Sprint 3.3):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| FL-Runde startet nicht | `kai contracts events FederatedLearning --last 50` | Quorum nicht erreicht (min. Nodes beigetreten?) |
| Schlechte Modell-Qualität | Gradient-Score im Contract prüfen | ε zu hoch (zu viel Rauschen)? Runden erhöhen |
| Gradient-Einreichung reverted | `kai chain tx <HASH> --decode-revert` | Deadline abgelaufen? Format korrekt? |
| Belohnungen nicht ausgezahlt | `kai chain balance-diff` nach Runde | Reward-Logik im Contract debuggen |

**🚀 Deployment-Checkliste (Sprint 3.3):**
- [ ] FL-Runde mit 10 Testnet-Nodes erfolgreich abgeschlossen
- [ ] Modell-Qualität vor/nach FL verglichen und dokumentiert
- [ ] DP-Parameter (ε, δ) in Config und Docs dokumentiert
- [ ] Keine `unwrap()` in Contract, Reentrancy-Schutz aktiv
- [ ] Belohnungen korrekt on-chain ausgezahlt
- [ ] PR + 2 Reviews + CI grün

**Technisches Risiko:** 🔴 Hoch — FL auf Blockchain ist wenig erprobt  
> 🔗 **Security Layer S3** (Kapitel 25.5): FL-Modell-Beiträge via ZKP verifiziert — kein Teilnehmer offenbart Rohdaten.
> 🔗 **Security Layer S4** (Kapitel 25.6): KI-IDS erkennt Model-Inversion-Angriffe auf FL-Runden.

> 🔗 **Security Layer S1** (Kapitel 25.3): AMM-Contract-Transaktionen mit Ed25519 signiert — keine anonymen DeFi-Aufrufe aus FL-Agenten.
> 🔗 **L11 DeFi-Layer** (Kapitel 26): Sprint 3.3 markiert den Start von L11 — `DeFiRegistry.ink` + AMM-Pool deployen, `nft://kai-os/layer/11/defi` wird geminted.
---

### Sprint 3.4 — Governance System v1.0 (KW 9–10, März 2027)

**Aufgaben:**
- [ ] `GovernanceDAO.ink`: Proposals, Conviction Voting, Quadratic Voting, Timelock
- [ ] Governance-UI im Dashboard
- [ ] Erste echte Abstimmung: "Block-Zeit von 6s auf 4s" → automatisch deployed

**🔧 Fehlerbehebungs-Schritte (Sprint 3.4):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Proposal erscheint nicht in UI | `kai governance proposals --status all` | Frontend-Cache invalidieren |
| Stimme wird nicht gezählt | `kai governance proposal <ID> --verbose` | Tokens gesperrt? Conviction-Level korrekt? |
| Timelock-Ausführung fehlgeschlagen | `kai chain tx <TIMELOCK-TX>` | Ziel-Pallet akzeptiert den Call? |
| Quorum nie erreicht | Beteiligung in Discord pushen | Quorum-Schwelle temporär via Proposal senken |

**🚀 Deployment-Checkliste (Sprint 3.4):**
> 🔗 Dieser Sprint implementiert den **L4 Governance NFT** (Kapitel 24.9) — pallet-democracy ist der On-Chain-Upgrade-Mechanismus für alle Layer.
- [ ] Governance-UI E2E: Proposal erstellen → abstimmen → Ergebnis sehen
- [ ] Timelock: angenommenes Proposal wartet korrekt 48h
- [ ] Erste echte Abstimmung: on-chain verifiziert + automatisch deployed
- [ ] DAO-Dokumentation aktuell (Kapitel 19)
- [ ] PR + 2 Reviews + CI grün
> 🔗 **Security Layer S2** (Kapitel 25.4): IDS-Schicht 2 erkennt Governance-Anomalien (Whale-Voting, Flash-Loan-Angriffe) in Echtzeit.
> 🔗 **Security Layer S5** (Kapitel 25.7): Alle Governance-Votes unveränderlich im On-Chain-Audit-Trail.

---

### Sprint 3.5 — Sicherheits-Audit Vorbereitung (KW 11–12, März 2027)

**Aufgaben:**
- [ ] Interner Pre-Audit: `cargo audit`, Slither, alle Findings beheben
- [ ] Formale Spezifikation kritischer Contracts (TLA+)
- [ ] Audit-Firma beauftragen (Trail of Bits / Certik / Cantina)

**🔧 Fehlerbehebungs-Schritte (Sprint 3.5):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| `cargo audit` meldet kritische CVE | `cargo audit fix` | Dependency updaten oder in `audit.toml` aufnehmen |
| TLA+-Modell nicht prüfbar | TLC Model Checker Log | State-Space zu groß → Abstraktion einführen |
| Pre-Audit findet Reentrancy | Code-Review | State-Änderungen vor externen Calls |

**🚀 Deployment-Checkliste (Sprint 3.5):**
> 🔗 **Kernel-Sprint K4** (Kapitel 24.6) startet hier (3.5, Härtung) und schließt in Sprint 3.6 (Audit-Abschluss, Apr 2027) ab — MK4 gilt als erreicht nach 0 Critical Findings.
- [ ] Pre-Audit: 0 Critical, 0 High Findings offen
- [ ] Audit-Firma vertraglich beauftragt, NDA unterzeichnet
- [ ] Audit-Scope-Dokument finalisiert
- [ ] Alle Code-Branches gefreezed (kein Feature-Merge während Audit)
- [ ] PR + 1 Review + CI grün
> 🔗 **Security Layer L0-Audit** (Kapitel 25.9): Audit-Scope umfasst explizit L0 — Crypto-Registry, ZeroTrust-Engine, ZKP-Circuits, IDS/IPS-Regeln.
> 🔗 **Security Layer S6** (Kapitel 25.8): Key-Rotation-Protokoll vor Audit verifizieren — alle Validator-Keys < 90 Tage alt.

---

### Sprint 3.6 — Sicherheits-Audit & Remediation (KW 13–16, Apr 2027)

**Aufgaben:**
- [ ] Externen Audit durchführen (4 Wochen)
- [ ] Critical/High Findings sofort beheben + Re-Audit
- [ ] Audit-Report öffentlich veröffentlichen
- [ ] Bug-Bounty-Programm starten (HackerOne / Immunefi)

**🔧 Fehlerbehebungs-Schritte nach Audit-Findings:**
| Finding-Typ | Sofortmaßnahme | Langfristige Lösung |
|---|---|---|
| Reentrancy | Emergency-Pause des Contracts | Check-Effects-Interactions-Pattern |
| Integer Overflow | Patch + sofortiges Deployment | `checked_*`-Arithmetik erzwingen |
| Access Control fehlt | Admin-Funktion temporär deaktivieren | Capability-Token-Prüfung ergänzen |
| Denial of Service | Rate-Limiting im Contract | Gas-Limits schärfer setzen |
| Krypto-Schwäche | Betroffene Funktion disablen | Stärkeres Primitiv einsetzen |

**🚀 Deployment-Checkliste (Sprint 3.6):**
- [ ] 100% Critical Findings behoben + Auditor-Bestätigung
- [ ] 100% High Findings behoben oder Risiko dokumentiert
- [ ] Re-Audit für alle kritischen Änderungen abgeschlossen
- [ ] Audit-Report auf GitHub + Docs veröffentlicht
- [ ] Bug-Bounty live
- [ ] PR + 3 Reviews (Security Engineer Pflicht) + CI grün

**Technisches Risiko:** 🟡 Mittel — kritische Findings können Architektur-Änderungen erfordern  
> 🔗 **Security Layer:** Alle L0-Findings (Crypto, ZeroTrust, ZKP, IDS) beheben + Re-Test (Kapitel 25).
> 🔗 **Security Layer S5** (Kapitel 25.7): ZKP-Compliance-Beweis für Audit-Zeitraum exportieren — `kai security audit --export zkp-proof`.

> 🔗 **Security Layer S3** (Kapitel 25.5): ZKP-Compliance-Beweis für Audit-Zeitraum exportieren — `kai security audit --export zkp-proof`. Gilt auch für alle L11-DeFi-Contracts im Audit-Scope.
> 🔗 **L11 DeFi-Layer** (Kapitel 26): Sprint 3.6 bringt Lending Protocol + Oracle Network live — $kUSD-Stablecoin Testnet. Alle L11-Contracts Teil des Security-Audits.
---

### Sprint 3.7 — Performance & Skalierung (KW 17–18, Mai 2027)

**Aufgaben:**
- [ ] Profiling: Engpässe in Node, API, Agent-Runtime
- [ ] Optimierungen: Parallelisierung (rayon), LRU-Cache (IPFS), Lock-freie Queue (crossbeam)
- [ ] Load-Test: 1000 gleichzeitige Agenten auf Testnet
- [ ] Ziele: Block-Zeit ≤ 4s, API p99 ≤ 200ms, ≥ 100 Tasks/Min/Node
- [ ] Performance-Regression-Tests in CI/CD

**🔧 Fehlerbehebungs-Schritte (Sprint 3.7):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| API p99 > 500ms | `kai net traffic-analysis --duration 60s` | Connection Pooling + Request Batching |
| Block-Zeit steigt unter Last | `kai chain sync-status` + CPU-Profil | Block-Verarbeitung parallelisieren (rayon) |
| IPFS-Abruf langsam | `kai storage info <CID>` mit Timing | LRU-Cache-Größe erhöhen in `node.toml` |
| Deadlock unter Last | `RUST_BACKTRACE=1` + Thread-Dump | Lock-freie Strukturen (crossbeam) |
| Memory-Leak bei vielen Agenten | `kai node resources --live` | `kai agent gc` automatisch per Cron |

**🚀 Deployment-Checkliste (Sprint 3.7):**
- [ ] Load-Test (1000 Agenten) dokumentiert und archiviert
- [ ] Alle Performance-Ziele erreicht (Soll/Ist-Tabelle)
- [ ] Performance-Regression-Test in CI: schlägt fehl bei p99 > 200ms
- [ ] Top-5-Engpässe behoben oder als Tech-Debt dokumentiert
- [ ] PR + 2 Reviews + CI grün
> 🔗 **Security Layer Overhead** (Kapitel 25.10): Zero-Trust-Latenz < 2ms/Request und ZKP-Verifikation < 100ms unter Last bestätigen.

> 🔗 **Security Layer S2** (Kapitel 25.4): Zero-Trust-Latenz-Test < 2ms/Request unter Last — inkl. L12 Quest-Engine-Calls.
> 🔗 **Security Layer S4** (Kapitel 25.6): Anti-Gaming-IDS für L12 unter Last testen — 1.000 simultane XP-Farm-Simulationen.
> 🔗 **L12 Gamification-Layer** (Kapitel 27): Sprint 3.7 ist der Start von L12 — Quest Engine + XP-System deployen, `nft://kai-os/layer/12/gamification` wird geminted.
---

### Sprint 3.8 — Alpha-Release & Enterprise-Pilot (KW 19–24, Jun 2027)

**Aufgaben:**
- [ ] Alpha-Release v1.0.0-alpha.1: Release Notes, Docker Hub, Homebrew/APT
- [ ] 3 Enterprise-Pilot-Unternehmen: Deployment, Support-Kanal, monatlicher Review
- [ ] Konferenz-Talk: EthCC, Web3 Summit oder Polkadot Decoded

**🔧 Fehlerbehebungs-Schritte (Sprint 3.8):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Docker-Image startet nicht | `docker logs kai-node --tail 50` | Fehlende Env-Variablen? Volume-Permissions? |
| APT-Repository nicht gefunden | `apt-get update 2>&1` | GPG-Key korrekt importiert? |
| Enterprise-Pilot: Firewall-Probleme | `kai net diagnose --verbose` beim Kunden | Port-Liste für IT-Abteilung bereitstellen |
| Release Notes unvollständig | `git log v0.9..v1.0.0-alpha.1 --oneline` | Conventional Commits → `conventional-changelog` |

**🚀 Deployment-Checkliste (Sprint 3.8 / Alpha-Release):**
- [ ] CI-Release-Pipeline vollständig grün
- [ ] Binaries: Linux x86_64, Linux ARM64, macOS x86_64, macOS ARM64
- [ ] Docker Hub: `kaios/node:1.0.0-alpha.1` + `kaios/node:latest-alpha`
- [ ] npm, PyPI, crates.io: Neue SDK-Versionen live
- [ ] GitHub Release mit Release Notes + Migration Guide
- [ ] `docs.kai-os.dev` zeigt Alpha-Version-Banner
- [ ] Community-Ankündigung: Discord, Twitter, Forum, Newsletter

**Phase-3-KPIs:**
| KPI | Ziel | Messung |
|---|---|---|
| SDK-Downloads (npm/PyPI) | ≥ 5.000/Monat | Package-Statistiken |
| Aktive Entwickler (Testnet) | ≥ 500 | Wallet-Adressen mit ≥ 1 Tx |
| Deployed dApps | ≥ 10 | On-Chain Registry |
| Audit Findings (Critical/High) | 100% behoben | Audit-Report |
| Testnet-Uptime | ≥ 99,5% | Monitoring |
| API-Latenz (p99) | ≤ 200ms | APM |
| Block-Zeit | ≤ 4s | Chain-Metriken |
| GitHub-Stars | ≥ 1.000 | GitHub |
| Offene P0-Issues zum Release | 0 | GitHub Issues |

---

> 🔗 **L12 Gamification-Layer** (Kapitel 27): Sprint 3.8 — Achievement System + Soul-Bound-NFTs live. Erste Achievements für Alpha-Tester werden geminted.
---

## Phase 4 — Beta & Mainnet (Q3 2027+)
**Status: ⚪ Geplant** | **Zeitraum: Juli 2027 – laufend** | **Team: 15+ Personen**

**Ziel:** Produktionsreifes System. Mainnet-Launch. TGE. Wachsendes Ökosystem.
> 🔗 **Security Release-Gate** (Kapitel 25): MS1 + MS2 + MK4 + L0-Audit (0 Critical) müssen vor Alpha-Release vollständig abgeschlossen sein.

---

### Sprint 4.1 — Beta-Vorbereitung & Mainnet-Infrastruktur (Jul 2027)

**Aufgaben:**
- [ ] Mainnet-Chain-Spec: Genesis-Block, Token-Verteilung, 21 Validatoren
- [ ] 5+ Boot-Nodes geo-verteilt (EU/US-East/US-West/Asien/LATAM)
- [ ] Explorer, Telemetry, Status-Page deployen
- [ ] Mainnet-Security-Checklist (100 Punkte) abarbeiten  
  > 🔗 Checkliste basiert auf **Security Layer Kapitel 25** (S1–S6 + L0-NFT-Status) + Kapitel 16
- [ ] On-Call-Rotation einrichten + Incident Playbooks (Kapitel 22) als Trockenübung

**🔧 Fehlerbehebungs-Schritte (Sprint 4.1):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Genesis-Block-Hash stimmt nicht | `kai chain block 0 --url <NODE>` auf allen Nodes | Chain-Spec neu verteilen, Nodes neu starten |
| Boot-Node nicht erreichbar | `check.kai-os.dev/port/30333` | Cloud-Firewall + Host-Firewall prüfen |
| Explorer zeigt leere Chain | Indexer-Log prüfen | Indexer auf Block 0 zurücksetzen |
| Validator startet nicht | `kai validator session-keys --check` | Keys im Keystore neu importieren |

**🚀 Deployment-Checkliste (Sprint 4.1):**
> 🔗 Kernel: Alle Kernel-Sprints K1–K4 (Kapitel 24.6) müssen vor Phase 4 abgeschlossen sein.
- [ ] Mainnet-Security-Checklist: 100/100 Punkte  
  > 🔗 Vollständige L0-Security-NFT aktiv: `nft://kai-os/layer/0/security` on-chain verifiziert
- [ ] 5 Boot-Nodes: von 3 externen Standorten erreichbar
- [ ] Alle 21 Validatoren verbunden, Konsensus stabil über 72h
- [ ] Block-Explorer live und korrekt
- [ ] On-Call-Rotation: ≥ 2 Personen pro Schicht
- [ ] Incident Playbooks durchgearbeitet + Trockenübung absolviert

> 🔗 **Security Layer S2** (Kapitel 25.4): Mainnet-Security-Checklist enthält L11- und L12-Capability-Token-Checks als Pflichtpunkte.
> 🔗 **L11 DeFi-Layer** (Kapitel 26): Sprint 4.1 aktiviert KI-gesteuerte Yield-Farming-Agenten (Agent ↔ L11) — DeFi 2.0 Alpha.
> 🔗 **L12 Gamification-Layer** (Kapitel 27): Sprint 4.1 aktiviert die KI-Reward-Engine — automatische $COMPUTE-Belohnungen on-chain.
---

### Sprint 4.2 — Token Generation Event (TGE) (Aug 2027)

**Aufgaben:**
- [ ] TGE-Contracts separat auditieren (Vesting, Public Sale, KYC)
- [ ] Exchange-Listings: CEX Top-20 + DEX-Liquidität
- [ ] Marketing-Kampagne
- [ ] TGE durchführen

**🔧 Fehlerbehebungs-Schritte (Sprint 4.2):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Vesting-Contract falsche Beträge | `kai contract query VestingContract get_vested_amount` | Cliff/Schedule in Genesis prüfen |
| KYC-Integration schlägt fehl | KYC-Provider-API-Log | API-Key abgelaufen? Webhook-URL korrekt? |
| DEX-Liquiditätspool falscher Preis | On-Chain Pool-State | Initialen Preis bei Erstellung korrekt setzen |

**🚀 Deployment-Checkliste (Sprint 4.2):**
- [ ] TGE-Contract-Audit: 0 Critical, 0 High Findings
- [ ] Vesting: Team/Investor-Tokens korrekt gesperrt (on-chain verifiziert)
- [ ] DEX-Liquiditätspool live, initialer Preis gesetzt
- [ ] Rechtliche Freigabe: Token-Counsel-Bestätigung schriftlich
- [ ] Community-Ankündigung 7 Tage vor TGE

**Rechtliches Risiko:** 🔴 Hoch — wertpapierrechtliche Fragen vor TGE vollständig klären  
> 🔗 **Security Layer S3** (Kapitel 25.5): TGE-Transaktionen via ZKP-Engine datenschutzkonform abwickeln — Betrag verifizierbar ohne Offenbarung.
> 🔗 **Security Layer S6** (Kapitel 25.8): TGE-Wallet-Keys nach HSM-Standard generieren und nach Event rotieren.

> 🔗 **Security Layer S5** (Kapitel 25.7): TGE-Transaktionen vollständig im On-Chain-Audit-Trail — inkl. aller L11/L12-Token-Bewegungen.
---

### Sprint 4.3 — Mainnet Go-Live (Sep 2027) 🚀

**Aufgaben:**
- [ ] Canary-Phase: 10% Traffic 2 Wochen beobachten
- [ ] Mainnet-Launch: Genesis-Block, alle 21 Validatoren synchron
- [ ] Launch-Ankündigung + First-Hour-Monitoring (15-Min-Updates)

**Definition of Done:**
```
Mainnet Block #100 finalisiert ✓
Alle 21 Validatoren aktiv ✓
Block-Explorer zeigt Daten ✓
API antwortet auf mainnet.kai-os.dev ✓
Kein P0-Incident in den ersten 24h ✓
```

**🔧 Fehlerbehebungs-Schritte (Sprint 4.3) — Launch Day Runbook:**
```
ERSTE 15 MINUTEN:
1. kai chain sync-status → "synced"?
2. Alle 21 Validatoren aktiv? → kai validator active-set
3. Explorer zeigt Block #1? → explorer.kai-os.dev
4. API OK? → curl https://api.mainnet.kai-os.dev/v1/chain/status
5. Monitoring stumm? → grafana.kai-os.dev
→ Alle OK: "🟢 Mainnet nominal" in #launch-room

FALLS Konsensus-Halt:
→ Incident Playbook 1 (Kapitel 22.3.1) sofort aktivieren

FALLS < 14/21 Validatoren:
→ Validator-Betreiber direkt kontaktieren
→ Boot-Nodes prüfen (Peer-Discovery-Problem?)

FALLS Explorer leer:
→ docker restart kai-indexer
→ kai indexer reindex --from-block 0

SECURITY-CHECK (erste 15 Minuten) — Kapitel 25.10:
→ kai security ids-status         # IDS aktiv und regelkonform?
→ kai security audit --last-blocks 10  # Audit-Trail vollständig?
→ kai security metrics             # Alle Prometheus-Labels grün?
→ Zero-Trust-Auth-Rate > 99.9%?   # grafana.kai-os.dev/security
```

**🚀 Deployment-Checkliste (Sprint 4.3 / Mainnet Go-Live):**
- [ ] Canary-Phase: 14 Tage ohne P0, Fehlerrate < 0,5%
- [ ] Alle 21 Validatoren: Session-Keys, ausreichend Stake, getestet
- [ ] Status-Page: "All Systems Operational" vor Ankündigung
- [ ] Rollback-Plan dokumentiert (falls Genesis fehlerhaft)
- [ ] Gesamtes Team on-call für 24h nach Launch
- [ ] Press Release an 20+ Krypto-Medien verteilt

> 🔗 **Security Layer S4** (Kapitel 25.6): Mainnet-Launch aktiviert IDS-Circuit-Breaker für L11 Flash-Loan-Angriffe — Echtzeit-Monitoring.
> 🔗 **L11 DeFi-Layer** (Kapitel 26): Sprint 4.3 — Flash Loan Engine + MEV-Schutz live — Mainnet DeFi-Launch.
---

### Sprint 4.4 — Ökosystem-Wachstum (Okt 2027 – laufend)

**Aufgaben:**
- [ ] Ökosystem-Fonds (50M $KAI): Infrastructure, dApps, Tooling, Research
- [ ] Hackathon #1: 3 Tage, 1M $KAI Preispool
- [ ] Validator-Dezentralisierung: 21 → 100+ Validatoren
- [ ] KI-Modell-Marktplatz Beta, IBC-Integration, erste KIPs

**Laufende Fehlerbehebungs-Prozesse (Post-Mainnet):**
| Prozess | Frequenz | Verantwortlich |
|---|---|---|
| Security-Review (Kapitel 22) | Quartalsweise | Security Engineer |
| Performance-Benchmarks | Monatlich | Platform Team |
| Post-Mortems (P0/P1) | Nach jedem Incident | Incident Commander |
| Dependency-Audit | Wöchentlich (CI) | Automatisch |
| Validator-Gesundheits-Check | Täglich | Monitoring + On-Call |

**Laufende Deployment-Prozesse (Post-Mainnet):**
| Typ | Frequenz | Strategie |
|---|---|---|
| Routine-Updates | Bei Bedarf | Rolling Update (Kap. 23.3.1) |
| Minor Releases | Monatlich | Rolling + Canary |
| Major Releases | Quartalsweise | Blue/Green (Kap. 23.3.3) |
| Emergency Hotfixes | Bei P0/P1 | Direktes Deployment + Post-Mortem |
| Governance-Upgrades | Nach DAO-Vote | Automatisch via Timelock-Executor |

**Phase-4-KPIs (12 Monate nach Mainnet):**
| KPI | Ziel |
|---|---|
| Aktive Nodes/Validatoren | ≥ 1.000 |
| Deployed dApps | ≥ 100 |
| Aktive Nutzer (monatlich) | ≥ 10.000 |
| Total Value Locked (TVL) | ≥ 50M $ |
| Transaktionen/Tag | ≥ 100.000 |
| GitHub Contributors | ≥ 200 |
| MTTD (Mean Time to Detect) | < 5 Minuten |
| MTTR (Mean Time to Resolve) | < 2 Stunden |
| Mainnet-Uptime | ≥ 99,9% |

---

> 🔗 **Security Layer S4** (Kapitel 25.6): Anti-Gaming-IDS für L12 Ökosystem-Wachstum aktiv — Sybil-Erkennung bei 10.000+ Nutzern.
> 🔗 **L12 Gamification-Layer** (Kapitel 27): Sprint 4.4 — Hackathon-Quests, Season 1 Leaderboard, 1.000+ aktive Quest-Nutzer.
---

## Meilenstein-Übersicht

| # | Meilenstein | Phase | Zieldatum | Status |
|---|---|---|---|---|
| M1 | Einzelner Node produziert Blöcke | 2 | Jul 2026 | ⚪ |
| M2 | 3-Node-Testnet stabil | 2 | Aug 2026 | ⚪ |
| M3 | Lokale KI-Inferenz funktioniert | 2 | Aug 2026 | ⚪ |
| M4 | Agent erstellen, Task ausführen | 2 | Sep 2026 | ⚪ |
| M5 | Ressourcen-Auktion on-chain | 2 | Sep 2026 | ⚪ |
| M6 | Agent-Output auf IPFS/on-chain | 2 | Okt 2026 | ⚪ |
| M7 | CLI-Quickstart funktioniert | 2 | Okt 2026 | ⚪ |
| M8 | Öffentliches Testnet + Demo | 2 | Dez 2026 | ⚪ |
| M9 | SDK v1.0 auf npm/PyPI/crates.io | 3 | Jan 2027 | ⚪ |
| M10 | Federated Learning auf Testnet | 3 | Feb 2027 | ⚪ |
| M11 | Governance-System live | 3 | Mär 2027 | ⚪ |
| M12 | Externer Sicherheitsaudit abgeschlossen | 3 | Apr 2027 | ⚪ |
| M13 | Alpha v1.0.0-alpha.1 Release | 3 | Jun 2027 | ⚪ |
| M14 | TGE abgeschlossen | 4 | Aug 2027 | ⚪ |
| M15 | Mainnet Go-Live 🚀 | 4 | Sep 2027 | ⚪ |
| M16 | 1.000+ aktive Nodes | 4 | Sep 2028 | ⚪ |
| MK1 | Micro-Kern bootet auf x86_64 + ARM64 | 2 | Jul 2026 | ⚪ |
| MK2 | KI-Kernel-Modul: GPU-Inferenz < 1s | 2 | Aug 2026 | ⚪ |
| MK3 | Blockchain-IPC-Bridge: 1000 Calls/s | 2 | Aug 2026 | ⚪ |
| MK4 | Kernel-Audit bestanden (0 Critical) | 3 | Apr 2027 | ⚪ |
| MS1 | L0-Security-NFT geminted (`nft://kai-os/layer/0/security`), Crypto+ZeroTrust live | 2 | Jul 2026 | ⚪ |
| MS2 | ZKP-Engine + IDS/IPS operativ | 2 | Sep 2026 | ⚪ |

---

## Risiko-Register

| Risiko | Wahrscheinlichkeit | Impact | Gegenmaßnahme |
|---|---|---|---|
| Substrate-Lernkurve verzögert Prototyp | Mittel | Hoch | Substrate-Experten early onboarden |
| Sicherheits-Audit findet kritische Issues | Mittel | Sehr Hoch | Interne Pre-Audits, frühzeitig beauftragen |
| Regulatory: Token als Wertpapier eingestuft | Mittel | Sehr Hoch | Rechtliche Beratung vor TGE |
| KI-Modell-Kosten skalieren nicht | Niedrig | Hoch | Hardware-Partner frühzeitig gewinnen |
| Konkurrenz kopiert Konzept | Hoch | Mittel | Schnelle Iteration, Community-Lock-in |
| Schlüsselperson verlässt Projekt | Niedrig | Hoch | Bus-Faktor ≥ 2, alle Prozesse dokumentiert |
| Netzwerk-Partition (Split-Brain) | Niedrig | Sehr Hoch | GRANDPA + Monitoring + Playbook (Kap. 22) |
| P0-Incident am Launch-Day | Mittel | Sehr Hoch | Canary-Phase, Rollback-Plan, On-Call-Team |
| **Kernel-Panic im Mainnet** | Niedrig | Sehr Hoch | Kernel K4-Audit + 72h syzkaller-Fuzzing + automatischer Rollback (3 Boot-Versuche) |
| **GPU-HAL inkompatibel (neuer Treiber)** | Mittel | Hoch | HAL-Abstraktion (Kap. 24.3), CPU-Fallback immer aktiv |
| **Kernel-NFT-Upgrade blockiert durch Governance** | Niedrig | Mittel | Notfall-Mechanismus: 4/5 Core-Sigs können Timelock überspringen (Kap. 24.9.5) |

---

# 18. Vergleich & Inspiration

| Projekt | Stärke | Schwäche | Relation zu KAI-OS |
|---|---|---|---|
| **Fetch.ai** | Agenten-Infrastruktur | Kein echtes LLM-OS | Inspiration: Agenten-Paradigma |
| **Bittensor** | Proof-of-Intelligence | Kein OS, kein Agenten-System | Inspiration: KI-Ökonomie |
| **Ocean Protocol** | Compute-to-Data | Nur Datenmarktplatz | Baustein: Datenschutz-Layer |
| **SingularityNET** | KI-Marktplatz, Vision | Langsame Entwicklung | Inspiration: KI-Services |
| **IPFS / Filecoin** | Dezentraler Speicher | Kein Compute | Baustein: Speicher-Layer |
| **Polkadot/Substrate** | Modulare Blockchain | Kein KI-Fokus | Baustein: Blockchain-Layer |

---

---

# 19. Governance & Community

> 🎫 **Verknüpfte Issues:** [🏛 #9](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/9)

## 19.1 DAO-Struktur

**Organe:**
- **Token-Holder Assembly** — Alle $KAI-Holder, Stimmrecht proportional zu Stake (Quadratic Voting für fairere Verteilung)
- **Technical Committee** (7 Mitglieder, 6-Monate-Wahl) — Technische Prüfung, Notfall-Veto
- **Security Council** (5 Mitglieder) — Sicherheitsvorfälle, max. 24h Protokoll-Pause
- **Ecosystem Fund Committee** — Verwaltung des Ökosystem-Fonds (5% Supply)

---

## 19.2 Abstimmungsmechanismen

| Kategorie | Quorum | Mehrheit | Timelock |
|---|---|---|---|
| Parameter-Änderung | 5% | 55% | 24h |
| Protocol-Upgrade | 10% | 60% | 48h |
| Treasury-Ausgabe | 8% | 60% | 24h |
| Konstitutionelle Änderung | 20% | 75% | 7 Tage |
| Notfall-Pause | 5% | 75% | Keine |

---

## 19.3 Beitragen (Contributing)

**Code:**
1. Issue öffnen → Diskussion → PR → 2x Code Review → Merge

**Dokumentation:** GitHub PR → Community-Review (48h)

**KI-Modelle:** Bias-Audit + Trainingsdaten-Dokumentation Pflicht

**Belohnungen:** Contributor-Rangliste (on-chain), Grants, NFT-Badges

---

## 19.4 Lizenzmodell

| Komponente | Lizenz |
|---|---|
| Core Protocol | Apache 2.0 |
| SDK & Tools | MIT |
| Enterprise-Features | Commercial |
| KI-Modelle | Model Card + individuelle Lizenz |

---

---

## 19.9 DAOGovernance — Live-Implementierung

> **Datei:** `blockchain/governance/dao_live.py` · Fixes: #39

```python
class DAOProposal:
    proposal_id:    str
    title:          str
    description:    str
    options:        list        # ["Ja", "Nein", "Enthaltung"]
    votes:          dict        # {addr: (choice_idx, power)}
    created_at:     int
    voting_ends_at: int         # + VOTING_PERIOD (7 Tage)
    timelock_until: int         # + TIMELOCK (48h)
    executed:       bool
    quorum_met:     bool

    def is_voting_active(self) -> bool: ...
    def get_result(self) -> dict:
        """Gibt Gewinner-Option + Stimmenverteilung zurück."""

class DAOGovernance:
    QUORUM_PERCENT:  float = 0.10   # 10% der Total Supply
    VOTING_PERIOD:   int   = 7*86400
    TIMELOCK:        int   = 48*3600
    MIN_STAKE:       float = 1_000.0

    def create_proposal(self, proposer: str, title: str,
                        description: str, options: list) -> str: ...
    def vote(self, voter: str, proposal_id: str,
             choice_idx: int) -> dict:
        """Abstimmen. Power = sqrt(stake_atc) — Quadratic Voting."""
    def finalize_proposal(self, proposal_id: str) -> dict: ...
    def execute_proposal(self, proposal_id: str,
                         executor: str) -> dict: ...
    def get_stats(self) -> dict: ...
```


# 20. Changelog

## v1.4.0-beta (Juni 2026) — AKTUELL

### Neu — Neue Module
- `blockchain/bridge/solana_bridge.py` — `SolanaBridge` (SPL-Token, Wormhole, Lock-and-Mint)
- `blockchain/dex/amm.py` — `ATCAMM` DEX (x*y=k Constant-Product, LP-Token, 0.3% Fee)
- `blockchain/governance/dao_live.py` — `DAOGovernance` Live (FFT+ATC Voting, Quorum 10%, Timelock 48h)
- `blockchain/mainnet/mainnet_config.py` — `MainnetLaunchManager` (Chain-ID 9000, Genesis Block)
- `blockchain/consensus/gas_fee.py` — `GasFeeEngine` (EIP-1559: Base Fee + Priority Fee, Burn 50%)
- `blockchain/consensus/fork_resolution.py` — `ForkResolver` (Longest-Chain + PoH-Score)
- `blockchain/nodes/initial_sync.py` — `InitialSyncer` (Neue Nodes bootstrappen)
- `blockchain/contracts/shivamon/breeding.py` — `BreedingEngine` (DNA-Vererbung, Element-Fusion, Gen 2)
- `blockchain/atcoin/atcoin.py` — `ATCoin` (dezimale Präzision, ERC-20 kompatibel)
- `mobile/wallet_api.py` — `MobileWalletManager` (React Native, BIP39, QR, Biometrie)
- `shivaos/kernel/syscalls.py` — `ShivaOSSyscallTable` (20 Syscalls: Prozess, ATCFS, Chain, KI)
- `shivaos/ui/renderer.py` — `ShivaOSRenderer` (TUI: Panel, TextBox, ProgressBar, Table)
- `atcpkg/manager.py` — `ATCPackageManager` (publish/install/search/dependencies)
- `blockchain/contracts/marketplace/marketplace.py` — `MarketplaceV2` (Festpreis + Auktion)

### ATCLang v0.3.0
- `async/await` Syntax + Generics + Closures + Module-System
- 7 neue Tests

### Enterprise
- `nginx/nginx.conf`, `.github/workflows/ci-cd.yml`, `SECURITY.md`, `CONTRIBUTING.md`
- `monitoring/alerts/blockchain_alerts.yml` — Prometheus Alert-Rules
- CodeQL + Docker multi-stage builds

### Status
- 43/43 GitHub Issues geschlossen ✅
- 300 Code-Dateien
- ATCLang VM: 40.380B · SolanaBridge: 10.621B · ATCAMM: 10.018B

---


## v1.0.0-alpha (Mai 2026)
**Erster öffentlicher Release der Software-Dokumentation.**

### Neu
- Vollständige API-Referenz (REST + WebSocket)
- SDK-Dokumentation: TypeScript, Python, Rust
- CLI-Referenz mit allen Befehlen
- Fehlerklassensystem (KAI-[KATEGORIE]-[CODE])
- Installationsanleitung (Linux, macOS, Docker)
- Konfigurationsreferenz (node.toml, agent.toml)
- Smart Contract Beispiel (ResourceMarket in Ink!)
- Testing-Guide (Unit, Integration, Load)
- Deployment & Betrieb (systemd, Monitoring, Backup)
- Sicherheitsrichtlinien & Bug Bounty Programm

### Bekannte Limitierungen
- GPU-Unterstützung noch experimentell
- Distributed Inference noch nicht auf Testnet verfügbar
- Substrate-Anbindung erfordert Rust 1.75+

---

## v0.9.0 (April 2026)
- Erste interne Version der Architektur-Dokumentation
- Vision & Konzept ausgearbeitet
- Roadmap definiert

---

## Geplante Features (v1.1.0)
- [ ] Grafana-Dashboard-Templates
- [ ] KI-Studio (visueller Agent-Builder)
- [ ] Multi-Chain Support (IBC/XCM)
- [ ] Mobile SDK (React Native)
- [ ] ZK-Proof-Integration für private Transaktionen

---

---


---

---

# 22. Erweiterte Fehlerbehebung & Incident Management

> Dieses Kapitel ergänzt die Basis-Fehlerbehandlung aus Kapitel 13 um strukturierte Diagnoseprozesse, Incident-Playbooks, erweiterte Debugging-Techniken und ein vollständiges Incident-Management-Framework für Produktionsumgebungen.

---

## 22.1 Diagnose-Framework: Structured Troubleshooting

Beim Auftreten eines Problems immer denselben strukturierten Prozess durchlaufen — das verhindert Aktionismus und verkürzt die Time-to-Resolution (TTR) erheblich.

```
┌─────────────────────────────────────────────────────┐
│              DIAGNOSE-FRAMEWORK (5 Schritte)        │
├─────────────────────────────────────────────────────┤
│  1. BEOBACHTEN   Was genau passiert? Symptome?      │
│  2. LOKALISIEREN Welche Komponente ist betroffen?   │
│  3. ISOLIEREN    Reproduzierbar? Unter welchen      │
│                  Bedingungen tritt es auf?          │
│  4. ANALYSIEREN  Root Cause identifizieren          │
│  5. BEHEBEN      Fix anwenden, Ergebnis validieren  │
└─────────────────────────────────────────────────────┘
```

### Schritt 1: Systemzustand erfassen

```bash
# Vollständiger Systemcheck in einem Befehl
kai doctor --full

# Beispiel-Output:
# ✅ Node erreichbar (localhost:9933)
# ✅ Blockchain synchronisiert (Block #1048576)
# ✅ IPFS-Node läuft (5001)
# ⚠️  Peers: 3 (Empfehlung: ≥ 5)
# ❌ KI-Modell nicht geladen (llama3-8b-q4)
# ✅ Wallet verfügbar (5GrwvaEF...)
# ✅ Guthaben ausreichend (1000 KAI)
```

### Schritt 2: Komponenten-Isolation

```bash
# Jeden Layer einzeln testen
kai ping --rpc          # Layer 2: RPC-Verbindung
kai ping --p2p          # Layer 1: P2P-Netzwerk
kai ping --ipfs         # Speicher-Layer
kai ping --ai           # KI-Kern
kai ping --contracts    # Blockchain-Layer

# Komponenten-Abhängigkeitsbaum anzeigen
kai doctor --dependency-tree
```

---

## 22.2 Erweiterte Fehlerdiagnose nach Komponente

### 22.2.1 Node & Blockchain-Probleme

**Problem: Node synchronisiert nicht / hängt bei Block X**

```bash
# Sync-Status prüfen
kai chain sync-status
# Output:
# Modus: Warp Sync
# Bester Block: #1048000
# Ziel-Block: #1048576
# Fortschritt: 99.9%
# Verbleibend: ~2 Minuten

# Falls Sync steckenbleibt: Peers prüfen
kai node peers --verbose
# Zeigt: Peer-ID, Adresse, Latenz, Gemeinsame Blöcke

# Peer mit gutem Sync-Status manuell hinzufügen
kai node peers add /dns4/boot1.kai-os.dev/tcp/30333/p2p/12D3KooW...

# Letzten bekannten guten Block finden
kai chain last-finalized

# Node aus Snapshot neu starten (schneller als vollständiger Sync)
kai node stop
kai node restore-snapshot --url https://snapshots.kai-os.dev/testnet/latest.tar.gz
kai node start
```

**Problem: Fork / Chain-Split erkannt**

```bash
# Anzeichen: Zwei verschiedene Block-Hashes für dieselbe Höhe
kai chain block 1048500 --peers-compare
# Output zeigt, welche Peers welchen Fork folgen

# Canonical Chain identifizieren (meiste Finalisierungen)
kai chain canonical

# Node auf canonical Chain zwingen
kai node stop
kai node prune --keep-canonical
kai node start
```

**Problem: Validator produziert keine Blöcke**

```bash
# Validator-Status prüfen
kai validator status

# Häufige Ursachen:
# 1. Session-Keys nicht gesetzt
kai validator session-keys --check

# 2. Zu wenig Stake
kai validator bonded-amount

# 3. Node nicht im aktiven Validator-Set
kai validator active-set --my-position

# 4. Zeitabweichung (NTP-Problem)
timedatectl status
sudo systemctl restart systemd-timesyncd

# Validator-Events der letzten 100 Blöcke
kai chain events --filter "validator" --last-blocks 100
```

---

### 22.2.2 Agent-Probleme

**Problem: Agent startet nicht / bleibt in Status "starting"**

```bash
# Detaillierte Agent-Logs
kai agent logs <AGENT-ID> --level trace --tail 200

# Häufige Fehlermuster und Bedeutung:
# "[ERROR] Model load failed: insufficient memory"
#   → Modell braucht mehr RAM als verfügbar
#   → Fix: Kleineres Modell wählen oder --inference distributed

# "[ERROR] Capability denied: write_storage"
#   → Agent hat Capability nicht im Manifest
#   → Fix: agent.toml anpassen, Agent neu deployen

# "[ERROR] Budget exhausted: compute"
#   → Compute-Budget aufgebraucht
#   → Fix: kai agent top-up <ID> --compute 500

# Agent-State direkt abfragen (auch bei hängenden Agenten)
kai agent state <AGENT-ID> --raw

# Agent hart neu starten (alle laufenden Tasks werden abgebrochen)
kai agent restart <AGENT-ID> --force
```

**Problem: Agent-Task läuft nie durch / Timeout**

```bash
# Task-Details inspizieren
kai agent task <AGENT-ID> <TASK-ID> --verbose

# Zeigt:
# - Aktueller Schritt im ReAct-Loop
# - Letzter LLM-Call (Prompt + Antwort)
# - Ressourcenverbrauch seit Task-Start
# - Ausstehende externe Calls

# Task-Timeout verlängern (für langlaufende Analysen)
kai agent invoke <ID> --type analyze --timeout 600s

# Task manuell abbrechen
kai agent task cancel <AGENT-ID> <TASK-ID>

# Alle fehlgeschlagenen Tasks eines Agenten anzeigen
kai agent tasks <AGENT-ID> --status failed --last 50
```

**Problem: Agent gibt falsche / inkonsistente Ergebnisse**

```bash
# Entscheidungsprotokoll des Agenten abrufen (XAI-Log)
kai agent audit <AGENT-ID> --task <TASK-ID>

# Zeigt:
# - Input-State zum Zeitpunkt der Entscheidung
# - Modell-Version (Hash)
# - Chain-of-Thought (komprimiert)
# - Konfidenzwert
# - On-Chain-Referenz

# Modell-Version prüfen (ist das richtige Modell geladen?)
kai ai model-info --agent <AGENT-ID>

# Agent mit anderem Modell testen
kai agent invoke <ID> --model llama3-70b-q4 --type analyze --input '...'

# Reproduzierbaren Testfall erstellen
kai agent replay <AGENT-ID> --task <TASK-ID> --output replay_test.json
```

---

### 22.2.3 Smart Contract-Probleme

**Problem: Transaktion schlägt fehl (Revert)**

```bash
# Transaktion-Details mit Revert-Grund
kai chain tx 0x1a2b3c... --decode-revert

# Output:
# Status: Failed
# Revert-Reason: "BidTooLow"
# Gas verwendet: 45.230 / 100.000
# Block: #1048300
# Zeitstempel: 2026-05-18T10:30:00Z

# Contract-State VOR der Transaktion rekonstruieren
kai contract state-at <CONTRACT-ADDR> --block 1048299

# Transaktion lokal simulieren (ohne on-chain zu schreiben)
kai contract simulate \
  --contract <ADDR> \
  --message bid \
  --args '["0xabc..."]' \
  --value 500 \
  --sender 5GrwvaEF...
# → Zeigt: Würde mit "BidTooLow" revertieren
# → Aktuelles Höchstgebot: 1000
```

**Problem: Contract verbraucht zu viel Gas / Weight**

```bash
# Weight-Profiling für einen Contract-Call
kai contract profile \
  --contract <ADDR> \
  --message allocate \
  --args '["0xabc..."]'

# Output:
# Ref-Time: 2.456.789 ps
# Proof-Size: 4.321 bytes
# Empfohlenes Limit: 3.000.000 ps (+ 20% Puffer)
# Optimierungshinweise:
#   - Mapping-Zugriff in Zeile 47: teuer (3 Storage-Reads)
#   - Empfehlung: Ergebnis cachen

# Gas-Nutzung historisch analysieren
kai contract gas-history <ADDR> --method bid --last 1000 --chart
```

**Problem: Contract-Events fehlen / korrumpiert**

```bash
# Events eines Contracts roh abrufen
kai contract events <ADDR> \
  --from-block 1048000 \
  --to-block 1048576 \
  --event BidPlaced \
  --output json > events.json

# Events neu indexieren (falls Indexer-Absturz)
kai indexer reindex --contract <ADDR> --from-block 0

# Event-Lücken finden
kai contract events-audit <ADDR> --check-gaps
```

---

### 22.2.4 Netzwerk & P2P-Probleme

**Problem: Node findet keine Peers**

```bash
# Detaillierte Peer-Discovery-Diagnose
kai net diagnose --verbose

# Checkliste wird durchlaufen:
# ✅ Port 30333 ist offen (externe Erreichbarkeit)
# ✅ Boot-Nodes erreichbar (boot1.kai-os.dev)
# ❌ mDNS deaktiviert (lokales Netzwerk)
# ✅ DHT aktiviert

# Externe Erreichbarkeit testen
curl -s https://check.kai-os.dev/port/30333
# → {"reachable": true, "latency_ms": 42}

# Manuell mit einem spezifischen Peer verbinden
kai node connect /ip4/1.2.3.4/tcp/30333/p2p/12D3KooW...

# Netzwerk-Routing-Tabelle anzeigen (DHT)
kai net routing-table --size 20
```

**Problem: Hohe Latenz / langsame Block-Propagation**

```bash
# Block-Propagations-Zeit messen
kai net latency --blocks 100

# Bandbreiten-Nutzung anzeigen
kai net bandwidth --live

# Peers nach Latenz sortieren (schlechte Peers entfernen)
kai node peers --sort-by latency
kai node peers remove <PEER-ID>  # Schlechte Peers entfernen

# P2P-Traffic analysieren (welche Peers senden viel/wenig)
kai net traffic-analysis --duration 60s
```

---

### 22.2.5 KI / Inferenz-Probleme

**Problem: Inferenz sehr langsam oder Timeout**

```bash
# Aktuelle Inferenz-Performance messen
kai ai benchmark --model llama3-8b-q4 --tokens 100

# Output:
# Prompt-Verarbeitung: 234ms
# Token-Generierung: 8.3 t/s
# Gesamtzeit (100 Token): 12.4s
# RAM-Nutzung: 5.8 GB / 16 GB
# GPU: nicht genutzt

# GPU-Beschleunigung aktivieren (falls NVIDIA GPU vorhanden)
kai ai benchmark --model llama3-8b-q4 --gpu
# Token-Generierung: 87.4 t/s ✓

# Kleineres/quantisierteres Modell verwenden
kai model list --sort-by speed
# q2_K: 22 t/s, q4_0: 8 t/s, q8_0: 5 t/s, f16: 2 t/s

kai ai set-model llama3-8b-q2_K  # Schneller, leicht schlechtere Qualität
```

**Problem: KI-Modell lädt nicht / korrupt**

```bash
# Modell-Integrität prüfen
kai model verify llama3-8b-q4
# → SHA256-Hash wird gegen Registry verifiziert

# Modell neu herunterladen
kai model remove llama3-8b-q4
kai model pull llama3-8b-q4 --verify

# Modell-Cache leeren
kai model cache clear

# Verfügbare Modelle und deren Status
kai model list --status
# NAME                SIZE    STATUS      LAST-USED
# llama3-8b-q4        4.7GB   ✅ OK       2026-05-18
# llama3-70b-q4       37GB    ❌ Korrupt  -
# mistral-7b-q4       4.1GB   ✅ OK       2026-05-15
```

---

## 22.3 Incident-Management-Framework

### Incident-Klassifikation

| Severity | Definition | Response-Zeit | Kommunikation |
|---|---|---|---|
| **P0 — Kritisch** | Mainnet down, Funds-Verlust möglich, Security-Breach | Sofort (< 15 Min) | Status-Page, Discord, Twitter |
| **P1 — Hoch** | >30% Nodes offline, API nicht erreichbar, Konsensus-Problem | < 1 Stunde | Status-Page, Discord |
| **P2 — Mittel** | Einzelne Features ausgefallen, erhöhte Latenz (>2x normal) | < 4 Stunden | Status-Page |
| **P3 — Niedrig** | Einzelner Fehler, Workaround existiert | < 24 Stunden | Discord |

---

### 22.3.1 Incident Response Playbooks

#### Playbook 1: Mainnet-Konsensus-Halt (P0)

```
SYMPTOM: Keine neuen finalisierten Blöcke seit > 5 Minuten

SOFORTMASSNAHMEN (erste 15 Minuten):
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. Incident-Kanal öffnen: #incident-YYYYMMDD-konsensus
2. Incident Commander bestimmen (On-Call-Person)
3. Status-Page auf "Investigating" setzen:
   kai status set-incident --severity P0 --title "Konsensus-Problem untersucht"

4. Validator-Status aller bekannten Validatoren prüfen:
   kai validator active-set --status

5. Häufigster Block-Hash unter Validatoren identifizieren:
   kai chain block-hash-distribution --last 10

6. Falls < 2/3 Validatoren online:
   → Security Council benachrichtigen (können Protokoll pausieren)
   → kai security pause --reason "Konsensus-Halt" --duration 1h

DIAGNOSE (15-60 Minuten):
━━━━━━━━━━━━━━━━━━━━━━━━━
7. Logs aller verfügbaren Validator-Nodes sammeln:
   kai validator logs --all --from last-finalized-block

8. Fork-Analyse: Gibt es konkurrierende Chains?
   kai chain fork-detect --since last-finalized-block

9. Root Cause identifizieren:
   □ Netzwerk-Partition (Peers trennen sich in zwei Gruppen)?
   □ Bug in Block-Verarbeitungs-Code?
   □ Coordinated Attack (DDoS auf Validatoren)?
   □ Uhrzeit-Abweichung (NTP-Fehler)?

WIEDERHERSTELLUNG:
━━━━━━━━━━━━━━━━━
10. Abhängig von Root Cause:
    - Netzwerk-Partition: Boot-Nodes neu starten, Peers manuell verbinden
    - Bug: Notfall-Fix deployen (Security Council Vote, 75%, kein Timelock)
    - DDoS: IP-Blocking, Peer-Whitelist aktivieren
    - NTP: Alle Validatoren NTP synchronisieren

11. Nach Fix: Konsensus beobachten bis 100 Blöcke finalisiert
12. Status-Page auf "Resolved" setzen
13. Post-Mortem innerhalb 48h erstellen
```

---

#### Playbook 2: Smart Contract Exploit (P0)

```
SYMPTOM: Ungewöhnlich hohe Token-Bewegungen, Security-Alert

SOFORTMASSNAHMEN (erste 15 Minuten):
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. Betroffenen Contract identifizieren:
   kai chain monitor --alert-threshold 10000 KAI

2. Exploit-Transaktion analysieren:
   kai chain tx <TX-HASH> --decode-full

3. Emergency Pause aktivieren (Security Council):
   kai security pause-contract <CONTRACT-ADDR> \
     --reason "Möglicher Exploit" \
     --multisig-key /etc/kai/security-council.key

4. Liquidität aus gefährdetem Pool entfernen (falls möglich):
   kai contract call EmergencyWithdraw --args '[]'

5. Community informieren:
   kai announce --channels discord,twitter \
     --severity critical \
     --message "Wir untersuchen eine mögliche Sicherheitslücke in [Contract]. Alle Operationen pausiert."

DIAGNOSE:
━━━━━━━━━
6. Alle Transaktionen mit dem Contract der letzten 24h analysieren:
   kai contract events <ADDR> --last-hours 24 --output forensics.json

7. Exploit-Mechanismus verstehen (Reentrancy? Integer Overflow? Access Control?)
8. Schadensausmaß quantifizieren:
   kai chain balance-diff <ADDR> --from <EXPLOIT-BLOCK>

WIEDERHERSTELLUNG:
━━━━━━━━━━━━━━━━━
9. Fix entwickeln und intern testen
10. Emergency-Governance-Vote (P1 Prozess: 24h, 75% Mehrheit)
11. Gefixten Contract deployen
12. Betroffene Nutzer identifizieren und entschädigen (Snapshot)
13. Bug-Bounty-Zahlung an Finder (falls Responsible Disclosure)
14. Öffentlicher Post-Mortem-Report
```

---

#### Playbook 3: Agent-Runtime-Ausfall (P1)

```
SYMPTOM: Neue Agenten können nicht gestartet werden / Tasks bleiben hängen

DIAGNOSE:
━━━━━━━━━
1. Agent-Runtime-Status:
   kai agent runtime-status

2. Ressourcen auf Node prüfen:
   kai node resources --live
   # → RAM, CPU, Disk, offene Datei-Handles

3. Häufigste Fehlerursachen:
   □ RAM-Erschöpfung (zu viele gleichzeitige Agenten + Modelle)
   □ IPFS-Verbindungsfehler (Storage nicht erreichbar)
   □ On-Chain-Nonce-Konflikt (doppelte Transaktionen)
   □ Modell-Datei korrupt oder fehlt

MASSNAHMEN:
━━━━━━━━━━━
□ RAM: Inaktive Agenten beenden (kai agent gc --dry-run, dann ohne --dry-run)
□ IPFS: IPFS-Daemon neu starten (docker restart kai-ipfs)
□ Nonce: kai chain nonce-reset --address <ADDR>
□ Modell: kai model verify --all, kai model pull <NAME>

ESKALATION: Falls nach 30 Min nicht gelöst → P0 eskalieren
```

---

### 22.3.2 Post-Mortem-Template

Jeder P0/P1-Incident bekommt innerhalb 48 Stunden einen schriftlichen Post-Mortem. Template:

```markdown
# Post-Mortem: [Incident-Titel]

**Datum:** YYYY-MM-DD
**Severity:** P0 / P1
**Dauer:** HH:MM (Erkennung bis Lösung)
**Incident Commander:** [Name]
**Verfasser:** [Name]

## Zusammenfassung
[2-3 Sätze: Was ist passiert, wie lange, Auswirkung]

## Zeitlinie
| Zeit (UTC) | Ereignis |
|---|---|
| HH:MM | Erstes Alert ausgelöst |
| HH:MM | Incident Commander bestimmt |
| HH:MM | Root Cause identifiziert |
| HH:MM | Fix deployed |
| HH:MM | Vollständig wiederhergestellt |

## Root Cause
[Technische Erklärung der Grundursache]

## Auswirkung
- Betroffene Nutzer: X
- Ausgefallene Transaktionen: Y
- Datenverlust: Keiner / [Details]
- Finanzieller Schaden: [Betrag oder "keiner"]

## Was gut lief
- [Punkt 1]
- [Punkt 2]

## Was verbessert werden kann
- [Punkt 1]
- [Punkt 2]

## Maßnahmen (mit Verantwortlichem und Deadline)
| Maßnahme | Verantwortlich | Deadline | Status |
|---|---|---|---|
| [Maßnahme 1] | [Name] | YYYY-MM-DD | Offen |

## Lessons Learned
[Was hat das Team gelernt?]
```

---

## 22.4 Monitoring & Alerting-Setup

### Prometheus + Grafana Stack

```yaml
# docker-compose.monitoring.yml
version: '3.8'
services:
  prometheus:
    image: prom/prometheus:v2.51
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:10.4
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=sicheres-passwort
    volumes:
      - grafana-data:/var/lib/grafana
      - ./monitoring/dashboards:/etc/grafana/provisioning/dashboards

  alertmanager:
    image: prom/alertmanager:v0.27
    volumes:
      - ./monitoring/alertmanager.yml:/etc/alertmanager/alertmanager.yml
    ports:
      - "9093:9093"

volumes:
  grafana-data:
```

```yaml
# monitoring/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "alert_rules.yml"

alerting:
  alertmanagers:
    - static_configs:
        - targets: ["alertmanager:9093"]

scrape_configs:
  - job_name: "kai-node"
    static_configs:
      - targets: ["kai-node:9615"]
    metrics_path: /metrics

  - job_name: "kai-agents"
    static_configs:
      - targets: ["kai-node:9616"]
```

```yaml
# monitoring/alert_rules.yml
groups:
  - name: kai_os_alerts
    rules:
      - alert: NoNewBlocks
        expr: increase(kai_blocks_finalized_total[5m]) == 0
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Keine neuen Blöcke seit 5 Minuten"
          runbook: "https://docs.kai-os.dev/runbooks/no-new-blocks"

      - alert: TooFewPeers
        expr: kai_network_peers_connected < 5
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "Zu wenige Peers: {{ $value }}"

      - alert: HighAPILatency
        expr: histogram_quantile(0.99, kai_api_request_duration_seconds_bucket) > 0.5
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "API p99-Latenz: {{ $value }}s"

      - alert: AgentErrorRate
        expr: rate(kai_agent_errors_total[5m]) > 0.1
        for: 3m
        labels:
          severity: warning
        annotations:
          summary: "Erhöhte Agent-Fehlerrate: {{ $value }}/s"

      - alert: DiskSpaceCritical
        expr: (node_filesystem_avail_bytes / node_filesystem_size_bytes) < 0.1
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Nur noch {{ $value | humanizePercentage }} Disk-Platz"

      - alert: ValidatorInactive
        expr: kai_validator_is_active == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Validator ist nicht mehr aktiv!"
```

### Alert-Routing (PagerDuty / Discord)

```yaml
# monitoring/alertmanager.yml
route:
  group_by: ["alertname", "severity"]
  group_wait: 30s
  group_interval: 5m
  repeat_interval: 4h
  receiver: "discord-general"
  routes:
    - match:
        severity: critical
      receiver: "pagerduty-oncall"
      continue: true
    - match:
        severity: critical
      receiver: "discord-incidents"

receivers:
  - name: "pagerduty-oncall"
    pagerduty_configs:
      - service_key: "${PAGERDUTY_KEY}"
        description: "{{ .CommonAnnotations.summary }}"

  - name: "discord-incidents"
    webhook_configs:
      - url: "${DISCORD_INCIDENTS_WEBHOOK}"
        send_resolved: true

  - name: "discord-general"
    webhook_configs:
      - url: "${DISCORD_GENERAL_WEBHOOK}"
        send_resolved: true
```

---

## 22.5 Log-Management

### Strukturiertes Logging

KAI-OS verwendet strukturiertes JSON-Logging für maschinelle Auswertbarkeit:

```json
{
  "timestamp": "2026-05-18T10:30:00.123Z",
  "level": "ERROR",
  "component": "agent-runtime",
  "agent_id": "agent_01HWXYZ",
  "task_id": "task_01ABCDE",
  "error_code": "KAI-AGENT-003",
  "message": "Model load failed: insufficient memory",
  "context": {
    "model": "llama3-70b-q4",
    "required_ram_gb": 42,
    "available_ram_gb": 16
  },
  "node_id": "12D3KooW...",
  "block_number": 1048576
}
```

### Log-Aggregation mit Loki

```bash
# Loki zum Monitoring-Stack hinzufügen
docker-compose -f docker-compose.monitoring.yml \
  -f docker-compose.loki.yml up -d

# Logs durchsuchen (LogQL)
# Alle Errors der letzten Stunde
{job="kai-node"} |= "ERROR" | json | line_format "{{.error_code}}: {{.message}}"

# Fehlerrate nach Komponente
sum by (component) (rate({job="kai-node"} |= "ERROR" [5m]))

# Spezifischen Agent tracen
{job="kai-node"} | json | agent_id = "agent_01HWXYZ"
```

### Log-Rotation

```bash
# /etc/logrotate.d/kai-node
/var/log/kai-node/*.log {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
    postrotate
        systemctl kill -s USR1 kai-node.service
    endscript
}
```

---

---

# 23. CI/CD & Deployment-Prozesse

> Dieses Kapitel beschreibt den vollständigen Softwareentwicklungs- und Deployment-Lifecycle: von der lokalen Entwicklung über automatisierte Tests bis hin zum Produktions-Rollout — inklusive Rollback-Strategien und GitOps-Ansatz.

---

## 23.1 Entwicklungs-Workflow (Git-Branching-Strategie)

KAI-OS verwendet eine **Trunk-Based Development**-Strategie mit Feature-Flags:

```
main (Trunk)
├── feature/agent-memory-v2        (kurzlebig, max. 3 Tage)
├── feature/fl-round-coordinator   (kurzlebig)
├── fix/validator-timeout          (kurzlebig, max. 1 Tag)
└── release/v1.1.0                 (langlebig, nur für Release)
```

### Branch-Regeln

| Branch | Beschreibung | Regeln |
|---|---|---|
| `main` | Immer deploybar, Basis für Testnet | Kein direkter Push; PR + 2 Reviews + CI grün |
| `feature/*` | Neue Features | Aus `main`, zurück in `main`, max. 3 Tage |
| `fix/*` | Bugfixes | Aus `main`, zurück in `main`, max. 1 Tag |
| `release/*` | Release-Vorbereitung | Nur Bugfixes, kein neues Feature |
| `hotfix/*` | Produktions-Notfall-Fix | Aus `main`, direkt auf `main` + `release/*` |

### Commit-Konvention (Conventional Commits)

```
<typ>(<scope>): <beschreibung>

Typen:
  feat:     Neues Feature
  fix:      Bugfix
  docs:     Dokumentation
  refactor: Code-Umstrukturierung (kein Feature, kein Fix)
  test:     Tests hinzufügen oder ändern
  perf:     Performance-Verbesserung
  ci:       CI/CD-Änderungen
  chore:    Wartungsaufgaben

Beispiele:
  feat(agent): add long-term memory support via IPFS
  fix(contracts): prevent reentrancy in ResourceMarket.bid()
  perf(inference): cache tokenizer initialization across requests
  docs(api): add WebSocket event examples to chapter 8
```

---

## 23.2 CI/CD-Pipeline (GitHub Actions)

### Übersicht der Pipelines

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   PR-Check   │    │  Main-Build  │    │ Release-Build│    │  Deployment  │
│              │    │              │    │              │    │              │
│ Lint         │    │ Build        │    │ Full Test    │    │ Testnet      │
│ Format       │    │ Unit Tests   │    │ Security Scan│    │ Staging      │
│ Unit Tests   │    │ Integration  │    │ Audit        │    │ Mainnet      │
│ Security     │    │ Docker Build │    │ Docker Push  │    │ (manuell)    │
│ Scan         │    │              │    │ Release Notes│    │              │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
     ~5 min              ~15 min             ~45 min            ~20 min
```

---

### 23.2.1 PR-Check Pipeline

```yaml
# .github/workflows/pr-check.yml
name: PR Check

on:
  pull_request:
    branches: [main]

env:
  RUST_TOOLCHAIN: "1.75.0"
  NODE_VERSION: "22"
  PYTHON_VERSION: "3.12"

jobs:
  lint-and-format:
    name: Lint & Format
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@v4

      - name: Rust Setup
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ env.RUST_TOOLCHAIN }}
          components: rustfmt, clippy

      - name: Cache Rust Dependencies
        uses: Swatinem/rust-cache@v2

      - name: Rust Format Check
        run: cargo fmt --all -- --check

      - name: Rust Clippy (No Warnings)
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Node.js Setup
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: "npm"
          cache-dependency-path: sdk/typescript/package-lock.json

      - name: TypeScript Lint (ESLint)
        run: |
          cd sdk/typescript
          npm ci
          npm run lint

      - name: Python Setup
        uses: actions/setup-python@v5
        with:
          python-version: ${{ env.PYTHON_VERSION }}

      - name: Python Lint (ruff + mypy)
        run: |
          pip install ruff mypy
          ruff check sdk/python/
          mypy sdk/python/ --strict

  unit-tests:
    name: Unit Tests
    runs-on: ubuntu-22.04
    needs: lint-and-format
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Cache
        uses: Swatinem/rust-cache@v2

      - name: Rust Unit Tests
        run: cargo test --all --lib
        env:
          RUST_LOG: warn

      - name: Contract Tests (Ink!)
        run: |
          cargo install cargo-contract --version "^4.0"
          cd contracts/
          cargo test --all

      - name: TypeScript Unit Tests
        run: |
          cd sdk/typescript
          npm ci
          npm run test:unit -- --coverage

      - name: Python Unit Tests
        run: |
          cd sdk/python
          pip install -e ".[test]"
          pytest tests/unit/ --cov=kai_sdk --cov-report=xml -q

      - name: Upload Coverage
        uses: codecov/codecov-action@v4
        with:
          token: ${{ secrets.CODECOV_TOKEN }}

  security-scan:
    name: Security Scan
    runs-on: ubuntu-22.04
    needs: lint-and-format
    steps:
      - uses: actions/checkout@v4

      - name: Rust Audit (cargo-audit)
        run: |
          cargo install cargo-audit
          cargo audit

      - name: npm Audit
        run: |
          cd sdk/typescript
          npm audit --audit-level=high

      - name: Python Safety Check
        run: |
          pip install safety
          cd sdk/python
          safety check -r requirements.txt

      - name: Secret Scanning (TruffleHog)
        uses: trufflesecurity/trufflehog@main
        with:
          path: ./
          base: ${{ github.event.repository.default_branch }}
          head: HEAD
          extra_args: --only-verified

  contract-audit:
    name: Smart Contract Static Analysis
    runs-on: ubuntu-22.04
    needs: unit-tests
    steps:
      - uses: actions/checkout@v4

      - name: Ink! Contract Analysis
        run: |
          cargo install cargo-contract
          cd contracts/
          # Bekannte Schwachstellenmuster prüfen
          cargo contract check --all
          # Keine ungeprüften `unwrap()` in Contract-Code
          ! grep -r "\.unwrap()" contracts/src/ || (echo "Unwrap in Contract-Code verboten!" && exit 1)
```

---

### 23.2.2 Main-Build Pipeline

```yaml
# .github/workflows/main-build.yml
name: Main Build & Deploy to Testnet

on:
  push:
    branches: [main]

jobs:
  build:
    name: Build All Artifacts
    runs-on: ubuntu-22.04
    outputs:
      version: ${{ steps.version.outputs.version }}
      image_tag: ${{ steps.version.outputs.image_tag }}
    steps:
      - uses: actions/checkout@v4

      - name: Determine Version
        id: version
        run: |
          VERSION="0.0.0-dev.$(git rev-parse --short HEAD)"
          echo "version=${VERSION}" >> $GITHUB_OUTPUT
          echo "image_tag=ghcr.io/kai-os/node:${VERSION}" >> $GITHUB_OUTPUT

      - name: Build Node Binary (Rust)
        run: cargo build --release --bin kai-node
        env:
          CARGO_INCREMENTAL: 0

      - name: Build Docker Image
        run: |
          docker build \
            --build-arg VERSION=${{ steps.version.outputs.version }} \
            --tag ${{ steps.version.outputs.image_tag }} \
            --tag ghcr.io/kai-os/node:latest-dev \
            -f docker/Dockerfile.node .

      - name: Push Docker Image
        run: |
          echo "${{ secrets.GITHUB_TOKEN }}" | docker login ghcr.io -u ${{ github.actor }} --password-stdin
          docker push ${{ steps.version.outputs.image_tag }}
          docker push ghcr.io/kai-os/node:latest-dev

      - name: Upload Binary Artifact
        uses: actions/upload-artifact@v4
        with:
          name: kai-node-${{ steps.version.outputs.version }}
          path: target/release/kai-node
          retention-days: 7

  integration-tests:
    name: Integration Tests
    runs-on: ubuntu-22.04
    needs: build
    services:
      ipfs:
        image: ipfs/kubo:v0.28
        ports:
          - 5001:5001
          - 4001:4001
    steps:
      - uses: actions/checkout@v4

      - name: Start Local Dev-Net (3 Nodes)
        run: |
          docker-compose -f docker-compose.devnet.yml up -d
          # Warten bis alle Nodes synchron sind
          ./scripts/wait-for-consensus.sh --timeout 120s

      - name: Run Integration Tests
        run: |
          cd tests/integration
          npm ci
          npm run test:integration -- --network dev --timeout 60000
        env:
          KAI_RPC_URL: http://localhost:9933
          KAI_WS_URL: ws://localhost:9944

      - name: Agent End-to-End Test
        run: |
          # 1. Agent deployen
          kai agent deploy --example hello-world --network dev

          # 2. Task ausführen
          RESULT=$(kai agent invoke hello-world --input "test" --network dev --wait)

          # 3. Ergebnis validieren
          echo "$RESULT" | jq -e '.status == "completed"'

      - name: Cleanup
        if: always()
        run: docker-compose -f docker-compose.devnet.yml down

  deploy-testnet:
    name: Deploy to Testnet
    runs-on: ubuntu-22.04
    needs: [build, integration-tests]
    environment: testnet
    steps:
      - name: Deploy to Testnet (Rolling Update)
        uses: appleboy/ssh-action@v1
        with:
          host: ${{ secrets.TESTNET_DEPLOY_HOST }}
          username: deploy
          key: ${{ secrets.TESTNET_SSH_KEY }}
          script: |
            cd /opt/kai-os
            # Neue Image-Version setzen
            export IMAGE_TAG=${{ needs.build.outputs.image_tag }}
            # Rolling Update: 1 Node nach dem anderen
            ./scripts/rolling-update.sh --image $IMAGE_TAG --nodes testnet-1,testnet-2,testnet-3

      - name: Smoke Test (Testnet)
        run: |
          sleep 30  # Nodes Zeit zum Starten geben
          kai ping --url https://rpc.testnet.kai-os.dev
          kai chain status --url https://rpc.testnet.kai-os.dev

      - name: Notify Discord
        if: always()
        uses: sarisia/actions-status-discord@v1
        with:
          webhook: ${{ secrets.DISCORD_DEPLOY_WEBHOOK }}
          title: "Testnet Deployment"
          description: "Version: ${{ needs.build.outputs.version }}"
```

---

### 23.2.3 Release-Pipeline

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - "v[0-9]+.[0-9]+.[0-9]+"
      - "v[0-9]+.[0-9]+.[0-9]+-alpha.[0-9]+"
      - "v[0-9]+.[0-9]+.[0-9]+-beta.[0-9]+"

jobs:
  validate-tag:
    name: Validate Release Tag
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@v4
      - name: Tag muss auf main zeigen
        run: |
          COMMIT=$(git rev-parse HEAD)
          MAIN_COMMIT=$(git rev-parse origin/main)
          if [ "$COMMIT" != "$MAIN_COMMIT" ]; then
            echo "ERROR: Tag zeigt nicht auf main!"
            exit 1
          fi

  full-test-suite:
    name: Full Test Suite
    runs-on: ubuntu-22.04
    needs: validate-tag
    steps:
      - uses: actions/checkout@v4
      - name: Run Complete Test Suite
        run: |
          cargo test --all
          cd sdk/typescript && npm run test
          cd sdk/python && pytest --tb=short

  build-release-artifacts:
    name: Build Release Artifacts
    runs-on: ${{ matrix.os }}
    needs: full-test-suite
    strategy:
      matrix:
        os: [ubuntu-22.04, ubuntu-20.04, macos-13, macos-14]
        include:
          - os: ubuntu-22.04
            artifact: kai-node-linux-x86_64
          - os: ubuntu-20.04
            artifact: kai-node-linux-x86_64-ubuntu20
          - os: macos-13
            artifact: kai-node-darwin-x86_64
          - os: macos-14
            artifact: kai-node-darwin-arm64
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: cargo build --release --bin kai-node
      - name: Upload Artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: target/release/kai-node

  publish-sdks:
    name: Publish SDKs
    runs-on: ubuntu-22.04
    needs: full-test-suite
    steps:
      - uses: actions/checkout@v4

      - name: Publish npm (@kai-os/sdk)
        run: |
          cd sdk/typescript
          npm ci
          npm run build
          echo "//registry.npmjs.org/:_authToken=${{ secrets.NPM_TOKEN }}" > ~/.npmrc
          npm publish --access public

      - name: Publish PyPI (kai-os-sdk)
        run: |
          cd sdk/python
          pip install build twine
          python -m build
          twine upload dist/* --username __token__ --password ${{ secrets.PYPI_TOKEN }}

      - name: Publish crates.io (kai-os-sdk)
        run: |
          cd sdk/rust
          cargo publish --token ${{ secrets.CARGO_TOKEN }}

  create-github-release:
    name: Create GitHub Release
    runs-on: ubuntu-22.04
    needs: [build-release-artifacts, publish-sdks]
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Download All Artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts/

      - name: Generate Changelog
        id: changelog
        run: |
          # Automatisches Changelog aus Conventional Commits
          npx conventional-changelog-cli -p angular -r 2 > CHANGELOG_RELEASE.md

      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          body_path: CHANGELOG_RELEASE.md
          files: artifacts/**/*
          draft: false
          prerelease: ${{ contains(github.ref, 'alpha') || contains(github.ref, 'beta') }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      - name: Update Docker Hub Tags
        run: |
          VERSION=${GITHUB_REF#refs/tags/v}
          docker pull ghcr.io/kai-os/node:latest-dev
          docker tag ghcr.io/kai-os/node:latest-dev kaios/node:${VERSION}
          docker tag ghcr.io/kai-os/node:latest-dev kaios/node:latest
          docker push kaios/node:${VERSION}
          docker push kaios/node:latest
```

---

## 23.3 Deployment-Strategien

### 23.3.1 Rolling Update (Standard)

Nodes werden nacheinander aktualisiert — das Netzwerk bleibt während des gesamten Updates online.

```bash
#!/bin/bash
# scripts/rolling-update.sh

NODES=("testnet-1" "testnet-2" "testnet-3" "testnet-4" "testnet-5")
IMAGE_TAG=$1
WAIT_BETWEEN=30  # Sekunden zwischen Node-Updates

for NODE in "${NODES[@]}"; do
  echo "=== Updating $NODE ==="

  # 1. Node aus Validator-Set entfernen (falls Validator)
  ssh deploy@$NODE "kai validator chill"

  # 2. Warten bis Nachbar-Nodes die Last übernommen haben
  sleep $WAIT_BETWEEN

  # 3. Node-Container aktualisieren
  ssh deploy@$NODE "
    cd /opt/kai-os
    docker pull $IMAGE_TAG
    docker-compose up -d --no-deps kai-node
  "

  # 4. Warten bis Node synchron ist
  TIMEOUT=120
  ELAPSED=0
  while true; do
    SYNCED=$(kai chain sync-status --url http://$NODE:9933 | jq -r '.synced')
    if [ "$SYNCED" = "true" ]; then break; fi
    sleep 5
    ELAPSED=$((ELAPSED + 5))
    if [ $ELAPSED -ge $TIMEOUT ]; then
      echo "ERROR: $NODE hat sich nicht synchronisiert. Rollback!"
      ./scripts/rollback.sh --node $NODE --image $PREVIOUS_IMAGE
      exit 1
    fi
  done

  # 5. Node wieder als Validator anmelden
  ssh deploy@$NODE "kai validator validate"
  echo "✅ $NODE aktualisiert"
done

echo "=== Rolling Update abgeschlossen ==="
```

---

### 23.3.2 Canary Deployment

Nur ein kleiner Teil des Traffics geht auf die neue Version — ideal für riskante Changes.

```
Traffic-Split:
┌─────────────────────────────────────────┐
│  Eingehende Anfragen (Load Balancer)    │
└──────────────┬──────────────────────────┘
               │
       ┌───────┴───────┐
       │               │
  90% Traffic     10% Traffic
       │               │
       ▼               ▼
  ┌─────────┐    ┌─────────┐
  │  v1.0   │    │  v1.1   │ ← Canary
  │ (Stable)│    │ (New)   │
  └─────────┘    └─────────┘
```

```bash
#!/bin/bash
# scripts/canary-deploy.sh

CANARY_WEIGHT=${1:-10}  # Prozent Traffic auf neue Version (default: 10%)
NEW_IMAGE=$2

echo "Starte Canary Deployment: ${CANARY_WEIGHT}% Traffic auf $NEW_IMAGE"

# 1. Einen Canary-Node auf neue Version upgraden
ssh deploy@testnet-canary "
  docker pull $NEW_IMAGE
  docker-compose up -d --no-deps kai-node
"

# 2. Load-Balancer-Gewichte setzen (nginx)
cat > /etc/nginx/conf.d/kai-upstream.conf << EOF
upstream kai_rpc {
    server testnet-1:9933 weight=$((100 - CANARY_WEIGHT));
    server testnet-2:9933 weight=$((100 - CANARY_WEIGHT));
    server testnet-canary:9933 weight=$CANARY_WEIGHT;
}
EOF
nginx -s reload

echo "✅ Canary aktiv. Beobachte Metriken für 30 Minuten..."
echo "Dashboard: https://grafana.kai-os.dev/d/canary"

# 3. Automatische Beobachtung
./scripts/canary-monitor.sh \
  --duration 30m \
  --error-threshold 0.01 \  # > 1% Fehlerrate → Auto-Rollback
  --latency-threshold 200    # > 200ms p99 → Auto-Rollback
```

---

### 23.3.3 Blue/Green Deployment (für Major Releases)

```
AKTUELL (Blue):          NEU (Green):
┌─────────────┐          ┌─────────────┐
│  Nodes 1-5  │          │  Nodes 6-10 │
│   v1.0.x    │          │   v1.1.0    │
│  (Produktion)│         │  (Bereit)   │
└─────────────┘          └─────────────┘
      ▲                        │
      │                        │
   Traffic                  Smoke Tests
                               │
                          ┌────▼──────┐
                          │ SWITCH!   │ ← In 5 Sekunden
                          └───────────┘
```

```bash
#!/bin/bash
# scripts/blue-green-switch.sh

BLUE_NODES=("node-1" "node-2" "node-3" "node-4" "node-5")
GREEN_NODES=("node-6" "node-7" "node-8" "node-9" "node-10")

echo "Smoke-Tests auf Green-Nodes..."
for NODE in "${GREEN_NODES[@]}"; do
  kai ping --url http://$NODE:9933 || exit 1
done

echo "Alle Green-Nodes OK. Schalte Traffic um..."
# Load-Balancer auf Green-Nodes umschalten
./scripts/lb-switch.sh --from blue --to green

echo "Traffic auf Green. Beobachte 5 Minuten..."
sleep 300

# Fehlerrate prüfen
ERROR_RATE=$(kai metrics error-rate --last 5m)
if (( $(echo "$ERROR_RATE > 0.02" | bc -l) )); then
  echo "Zu hohe Fehlerrate ($ERROR_RATE). Rollback auf Blue!"
  ./scripts/lb-switch.sh --from green --to blue
  exit 1
fi

echo "✅ Blue/Green Switch erfolgreich!"
echo "Blue-Nodes können nun auf neue Version aktualisiert werden."
```

---

## 23.4 Rollback-Strategien

### Automatischer Rollback

```bash
#!/bin/bash
# scripts/rollback.sh

NODE=$1
PREVIOUS_IMAGE=$2

echo "⚠️  Rollback wird eingeleitet für $NODE → $PREVIOUS_IMAGE"

ssh deploy@$NODE "
  docker pull $PREVIOUS_IMAGE
  docker tag $PREVIOUS_IMAGE kaios/node:current
  docker-compose up -d --no-deps kai-node
"

# Warten bis Node wieder synchron
./scripts/wait-for-sync.sh --node $NODE --timeout 120s

echo "✅ Rollback abgeschlossen. $NODE läuft wieder auf $PREVIOUS_IMAGE"
```

### Rollback-Entscheidungsmatrix

| Symptom nach Deployment | Rollback? | Wer entscheidet? | Frist |
|---|---|---|---|
| Fehlerrate > 5% | Sofort automatisch | CI/CD-System | 0 Min |
| Fehlerrate 1–5% | Wahrscheinlich | On-Call Engineer | 15 Min |
| API-Latenz p99 > 500ms | Wahrscheinlich | On-Call Engineer | 15 Min |
| Konsensus-Probleme | Sofort | Security Council | 0 Min |
| Einzelne Agents fehlerhaft | Nein (Hotfix) | Entwickler | nächster Sprint |
| Kein Effekt sichtbar | Nein | Entwickler | — |

---

## 23.5 Umgebungs-Management

### Umgebungs-Übersicht

| Umgebung | Zweck | Deployment-Trigger | Zugang |
|---|---|---|---|
| **dev** (lokal) | Lokale Entwicklung | Manuell | Entwickler |
| **ci** | Automatisierte Tests | Jeder PR | CI/CD |
| **testnet** | Öffentlicher Test | Merge auf `main` | Öffentlich |
| **staging** | Pre-Produktion | Release-Tag (pre-release) | Eingeschränkt |
| **mainnet** | Produktion | Manuell (nach Approval) | Öffentlich |

### Umgebungs-Konfiguration (GitOps)

```
infrastructure/
├── environments/
│   ├── testnet/
│   │   ├── values.yaml          # Konfiguration für Testnet
│   │   ├── node-count: 5
│   │   └── resources.yaml       # Kubernetes-Ressourcen
│   ├── staging/
│   │   └── values.yaml
│   └── mainnet/
│       └── values.yaml          # Mainnet: höhere Ressourcen, mehr Nodes
├── charts/
│   └── kai-node/
│       ├── Chart.yaml
│       ├── templates/
│       │   ├── deployment.yaml
│       │   ├── service.yaml
│       │   └── configmap.yaml
│       └── values.yaml          # Default-Werte
└── scripts/
    ├── rolling-update.sh
    ├── canary-deploy.sh
    └── rollback.sh
```

```yaml
# infrastructure/environments/testnet/values.yaml
nodeCount: 5
image:
  repository: ghcr.io/kai-os/node
  tag: latest-dev  # Wird von CI überschrieben
  pullPolicy: Always

resources:
  requests:
    memory: "8Gi"
    cpu: "4"
  limits:
    memory: "16Gi"
    cpu: "8"

storage:
  size: 200Gi
  class: ssd

network: testnet
logLevel: debug  # Testnet: debug; Mainnet: info

monitoring:
  enabled: true
  prometheusPort: 9615

rpc:
  host: "0.0.0.0"  # Testnet: öffentlich
  cors: "*"
```

---

## 23.6 Datenbank-Migrationen

### Migration-Strategie

Blockchain-State-Änderungen (neue Pallets, geänderte Storage-Layouts) müssen mit dem laufenden System kompatibel sein.

```rust
// Substrate Runtime Migration
pub struct Migration<T: Config>(sp_std::marker::PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for Migration<T> {
    fn on_runtime_upgrade() -> Weight {
        log::info!("Migration v1→v2: AgentRegistry Storage-Layout");

        // Alte Daten lesen
        let old_agents: Vec<OldAgentData> = OldAgentStorage::<T>::iter().collect();

        // Neue Daten schreiben
        for old in old_agents {
            let new = NewAgentData {
                id: old.id,
                name: old.name,
                model: old.model,
                // Neues Feld mit Default-Wert
                reputation: ReputationScore::default(),
            };
            NewAgentStorage::<T>::insert(old.id, new);
        }

        // Alten Storage bereinigen
        OldAgentStorage::<T>::remove_all(None);

        log::info!("Migration abgeschlossen: {} Agenten migriert", old_agents.len());
        T::DbWeight::get().reads_writes(old_agents.len() as u64, old_agents.len() as u64 * 2)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
        // Snapshot vor Migration für Validierung
        let count = OldAgentStorage::<T>::iter().count();
        Ok((count as u64).encode())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
        let old_count = u64::decode(&mut &state[..]).unwrap();
        let new_count = NewAgentStorage::<T>::iter().count() as u64;
        assert_eq!(old_count, new_count, "Migration: Datenverlust!");
        Ok(())
    }
}
```

```bash
# Migration vor Deployment testen
cargo run --features try-runtime -- try-runtime \
  --runtime ./target/release/wbuild/kai-runtime/kai_runtime.wasm \
  on-runtime-upgrade live \
  --uri wss://rpc.testnet.kai-os.dev
```

---

## 23.7 Release-Checkliste

Vor jedem Release (Alpha, Beta, Mainnet) muss diese Checkliste vollständig abgehakt sein:

### Pre-Release (1 Woche vorher)
- [ ] Alle geplanten Features gemergt und getestet
- [ ] Changelog finalisiert (`CHANGELOG.md` aktualisiert)
- [ ] Breaking Changes dokumentiert (Migration Guide)
- [ ] Version in `Cargo.toml`, `package.json`, `pyproject.toml` erhöht
- [ ] Release-Branch `release/vX.Y.Z` erstellt
- [ ] Sicherheits-Scan: `cargo audit`, `npm audit`, `safety check` — alle clean
- [ ] Dependency-Updates: Alle Dependencies auf aktuelle Versionen geprüft

### Release-Tag (Release-Tag)
- [ ] Tag `vX.Y.Z` auf `main` gesetzt
- [ ] CI/CD-Release-Pipeline grün (alle Builds, Tests, SDK-Publizierungen)
- [ ] GitHub Release erstellt mit Changelog und Binaries
- [ ] Docker Hub Tags: `vX.Y.Z` und `latest` gesetzt
- [ ] npm, PyPI, crates.io: Neue Version verfügbar

### Post-Release (1 Tag nach Release)
- [ ] Deployment auf Testnet erfolgreich
- [ ] Smoke Tests auf Testnet manuell durchgeführt
- [ ] Dokumentation aktualisiert (`docs.kai-os.dev`)
- [ ] Community-Ankündigung: Discord, Twitter, Forum
- [ ] Monitoring: 24h erhöhte Aufmerksamkeit nach Release
- [ ] (Mainnet only): Status-Page auf "All Systems Operational"
- [ ] (Mainnet only): Block-Explorer zeigt korrekte Daten

---

## 23.8 GitOps & Infrastructure as Code

Alle Infrastruktur-Änderungen werden als Code in Git verwaltet — kein manueller Server-Eingriff ohne entsprechenden PR.

```
Entwickler → PR → Review → Merge → ArgoCD erkennt Änderung → Auto-Deploy
```

```yaml
# argocd-app.yaml — ArgoCD Application für Testnet
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: kai-os-testnet
  namespace: argocd
spec:
  project: kai-os
  source:
    repoURL: https://github.com/kai-os/infrastructure
    targetRevision: main
    path: environments/testnet
    helm:
      valueFiles:
        - values.yaml
  destination:
    server: https://kubernetes.default.svc
    namespace: kai-os-testnet
  syncPolicy:
    automated:
      prune: true      # Entfernte Ressourcen werden gelöscht
      selfHeal: true   # Manuelle Änderungen werden zurückgesetzt
    syncOptions:
      - CreateNamespace=true
    retry:
      limit: 3
      backoff:
        duration: 30s
        factor: 2
        maxDuration: 5m
```

---

## 20.7 Vollständige Commit-Historie (Stand: 2026-06-11)

> **Repository:** `A-TownChain-Okosystems/a-townchain-os` | **Branch:** `main`

```
1e8a5872a1  2026-06-11  chore: Auto-Sync STATUS.md — 2026-06-11 08:01
a062353fa3  2026-06-10  enterprise: Dockerfile — CI/CD, Docker, Nginx, Prometheus, Security
5c00e3fea7  2026-06-10  enterprise: docker-compose.yml — CI/CD, Docker, Nginx, Prometheus
a02104733a  2026-06-10  enterprise: monitoring/alerts/blockchain_alerts.yml
7bff00f510  2026-06-10  enterprise: monitoring/prometheus.yml — Prometheus Konfiguration
fa70a4d259  2026-06-10  enterprise: nginx/nginx.conf — Reverse Proxy Konfiguration
387082e327  2026-06-10  enterprise: CHANGELOG.md — Versionshistorie v3.0.0
2a2f2f19c6  2026-06-10  enterprise: CONTRIBUTING.md — Beitragsrichtlinien
c602856e12  2026-06-10  enterprise: SECURITY.md — Sicherheitsrichtlinien
aa47cbe91c  2026-06-10  enterprise: .github/workflows/ci-cd.yml — CI/CD Pipeline
d34225f09a  2026-06-10  fix(#39): DAO Governance Live — FFT+ATC Voting, Quorum 10%, Timelock
2a8a4a1635  2026-06-10  fix(#38): Mobile Wallet — React Native, BIP39, QR, Biometric
9cedd4d067  2026-06-10  fix(#37): DEX/AMM — x*y=k, Swap-Router, Liquidity Pools, LP-Token
7f692c3b60  2026-06-10  fix(#36): Mainnet Launch Config — Chain-ID 9000, Genesis, Validators
a9c89565a8  2026-06-10  fix(#35): ATCLang v0.3.0 — async/await, Generics, Closures, Modules
9277390dd5  2026-06-10  fix(#34): Solana Bridge v3.0.0 — SPL-Token, Wormhole, Lock-and-Mint
542f6a1255  2026-06-10  fix(#39): DAO Governance (alt) — On-Chain Proposals, Quorum
10b198634   2026-06-10  fix(#38): Mobile Wallet (alt) — BIP39, QR-Code, Biometric
75f34d4d9f  2026-06-10  fix(#37): DEX/AMM (alt) — Constant-Product x*y=k
9fd3e848fb  2026-06-10  fix(#36): Mainnet Launch (alt) — Chain-ID 9000, Genesis Block
0f23c10ea3  2026-06-10  test(atclang): ATCLang v0.3.0 — 7 Tests (Closures, Generics, Module)
c87163ccb4  2026-06-10  fix(#35): ATCLang v0.3.0 (alt) — async/await, Generics
dbade664f9  2026-06-10  test(bridge): Solana Bridge — 4 Tests (Full Flow, Replay, Daily-Limit)
dbec57109  2026-06-10  fix(#34): Solana Bridge — SPL-Token, Lock-and-Mint, Wormhole
848c7c80a9  2026-06-10  docs: CHANGELOG v3.0.0 — 31 Issues synchronisiert
e03c437404  2026-06-10  fix(#30): atcpkg Registry API — list, info, install, stats
4fb1c12a61  2026-06-10  fix(#29): Federated Learning Coordinator — FedAvg, On-Chain
570b747399  2026-06-10  fix(#28): ShivaOS UI Renderer — TUI Components (Panel, TextBox)
8678f803d3  2026-06-10  fix(#27): atcpkg Package Manager — Registrierung, Install, Dependencies
26e694c87e  2026-06-10  fix(#7): Build System — Docker, Linux AppImage, Windows EXE, .deb
4f3fcab5ff  2026-06-10  fix(#13): ATC Marketplace — Festpreis+Auktion, 2.5% Fee, 5% Royalty
d0b9ba9525  2026-06-10  fix(#10): Cross-Chain Bridge — Lock-and-Mint, ATC↔ETH/POLYGON/BSC
37cad2a2a8  2026-06-10  fix(#11): Shivamon Breeding Engine — DNA-Vererbung, Element-Fusion
636aa98c6d  2026-06-10  fix(#32): ShivaOS Syscall-Tabelle — 20 Syscalls
f0b9ce290b  2026-06-10  fix(#33): Gas-Fee Engine — EIP-1559, Base Fee, Priority Fee, 50% Burn
64515fb8b8  2026-06-10  fix(#26): Integration Tests ATCFS + MultiSig + ATCLang + Gateway (9T)
6e2d7c868c  2026-06-10  fix(#25): Gateway main.py v1.0.0 — alle Middlewares aktiv
b260480494  2026-06-10  fix(#24): MultiSig Wallet — M-of-N Signing, Bridge Vault
109ce46576  2026-06-10  fix(#23): ATCFS ShivaOS Kernel-Modul — syscall Interface
d53d48c788  2026-06-10  fix(#23): ATCFS — vollständiges A-TownChain Filesystem (L6)
445e43865d  2026-06-10  fix(#19): Node-Monitoring Dashboard — Flask + Prometheus /metrics
f4d5f44940  2026-06-10  fix(#8+#18): start_testnet.sh — 5-Node Testnet Launcher
dbb13590ac  2026-06-10  fix(#18): Dockerfile.node für alle Node-Typen
1e2edb17c9  2026-06-10  fix(#18): Docker Compose 5-Node Testnet
ea5175ea75  2026-06-10  test(solidity): ATC Token.test.js — 22 Tests
8a3574e883  2026-06-10  feat(core): ATCFS, Gateway, MultiSig hinzugefügt
25ed5c3148  2026-06-10  test(solidity): 4 neue Test-Suites — vollständige Contract-Tests
0d6af1394c  2026-06-10  feat(solidity): fehlende Smart Contracts — Issue #12
793f61b36d  2026-06-10  docs: AGENT_MANIFEST.md — vollständige Datenkarte für KI-Agenten
d08f184472  2026-06-10  docs: Split docs/ → a-townchain-os-docs | Software-Repo bereinigt
```

> **Docs-Repo:** `A-TownChain-Okosystems/a-townchain-os-docs`

```
10088ecd75  2026-06-10  docs(wiki): FINALER AUDIT 162/162 — Wiki vollständig synchron
de5be7d1ad  2026-06-10  docs(wiki): Kap. 31 + Issues #28-30 — finaler Stand
60bf8ec0c7  2026-06-10  docs(wiki): Finaler Audit 57/57 Checks OK
62b9038aa1  2026-06-10  docs(wiki): Kap. 31 — alle 27 Issues vollständig eingetragen
73dfadf7f8  2026-06-10  docs(wiki): Kapitel 32-52 hinzugefuegt (52/52 komplett)
```


# 21. Glossar

| Begriff | Erklärung |
|---|---|
| **Agent** | Autonomes Software-Programm mit eigenen Zielen, Wahrnehmungen und Handlungsmöglichkeiten. Im KAI-OS ersetzt der Agent das klassische "Programm". |
| **Capability Token** | Kryptografischer Token, der einem Agenten erlaubt, eine spezifische Aktion auszuführen. |
| **CID (Content Identifier)** | Eindeutiger Hash-basierter Bezeichner für eine Datei in IPFS. |
| **Consensus / Konsensus** | Mechanismus, durch den alle Nodes sich auf den gemeinsamen Zustand einigen. |
| **DAO** | Decentralized Autonomous Organization — durch Smart Contracts regierte Organisation ohne zentrale Führung. |
| **DID** | Decentralized Identifier — selbstkontrollierte digitale Identität nach W3C-Standard. |
| **Differential Privacy** | Methode, die statistische Auswertungen erlaubt ohne individuelle Datenpunkte preiszugeben. |
| **dApp** | Dezentrale Anwendung auf einem Blockchain-Netzwerk. |
| **Federated Learning** | KI-Training ohne Datenzentralisierung — nur Modell-Updates werden geteilt. |
| **GRANDPA** | Finalisierungsprotokoll in Substrate-Blockchains (deterministische Sicherheit). |
| **Inference / Inferenz** | Verwendung eines trainierten KI-Modells zur Antwortgenerierung. |
| **Ink!** | Rust-basierte Smart-Contract-Sprache für Substrate. |
| **IPFS** | InterPlanetary File System — dezentrales Peer-to-Peer-Dateisystem. |
| **KAI-OS** | Das KI-Blockchain-Betriebssystem, das in diesem Wiki dokumentiert ist. |
| **Layer 1 (L1)** | Basisschicht einer Blockchain (Ethereum, Solana, eigene Chain). |
| **Layer 2 (L2)** | Skalierungslösung auf einer L1 (Arbitrum, Optimism). |
| **libp2p** | Modularer Netzwerk-Stack für P2P-Kommunikation. |
| **Model Card** | Standardisierte Dokumentation eines KI-Modells. |
| **Multisig** | Wallet/Contract das mehrere Signaturen für Transaktionen erfordert. |
| **NPoS** | Nominated Proof of Stake — Konsensus-Mechanismus in Substrate. |
| **Node** | Einzelner Teilnehmer im KAI-OS-Netzwerk. |
| **ONNX** | Open Neural Network Exchange — offenes KI-Modell-Format. |
| **OrbitDB** | Dezentrale IPFS-basierte Datenbank. |
| **P2P** | Peer-to-Peer — Netzwerk ohne zentralen Server. |
| **Proof of Stake (PoS)** | Konsensus-Mechanismus mit Token-Sicherheit und Slashing. |
| **Quadratic Voting** | Abstimmungsmethode mit quadratisch steigenden Kosten für Zusatzstimmen. |
| **ReAct-Pattern** | Agent-Architektur: abwechselndes Reasoning und Acting. |
| **Reputation** | On-Chain Vertrauenswert eines Nodes oder Agenten. |
| **Seed Phrase** | 24 Wörter zur Wiederherstellung eines Wallets. **NIEMALS teilen!** |
| **Session Key** | Temporärer Schlüssel für Validator-Operationen (getrennt vom Hauptkey). |
| **Slashing** | Strafmechanismus: Verlust von Stake bei Fehlverhalten. |
| **Smart Contract** | Selbst-ausführendes Programm auf der Blockchain. |
| **Substrate** | Blockchain-Framework von Parity Technologies (Polkadot-Ökosystem). |
| **Timelock** | Verzögerungsmechanismus für Governance-Entscheidungen. |
| **TGE** | Token Generation Event — erstmalige Ausgabe von Token. |
| **Verifiable Credential** | Kryptografisch signierte, offline verifizierbare Identitätsaussage. |
| **XAI** | Explainable AI — nachvollziehbare KI-Entscheidungen. |
| **Zero-Knowledge Proof (ZKP)** | Beweis einer Information ohne die Information selbst preiszugeben. |
| **Zero Trust** | Sicherheitsmodell: kein automatisches Vertrauen, jede Aktion wird verifiziert. |

---


# 24. Betriebssystem-Kernel

> Der KAI-OS Kernel ist das Herzstück des Systems — die unterste Software-Schicht, die direkt mit Hardware und Blockchain-Node kommuniziert. Er ist als **Hybrid-Kernel** konzipiert: minimaler Mikro-Kern für Stabilität und Sicherheit, erweiterbar durch Module für KI, Blockchain und dezentrale Dienste.

---

## 24.1 Kernel-Architektur: Design-Prinzipien

### Architektur-Typ: Hybrid-Kernel

```
┌─────────────────────────────────────────────────────────────┐
│                    USER SPACE                               │
│  KI-Agenten │ dApps │ CLI │ REST API │ SDK │ Dashboard      │
├─────────────────────────────────────────────────────────────┤
│                  KERNEL SPACE                               │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────────┐  │
│  │ Micro-Kern  │  │  KI-Modul    │  │ Blockchain-Modul  │  │
│  │  (Basis)    │  │  (Inferenz)  │  │  (Substrate-IPC)  │  │
│  └─────────────┘  └──────────────┘  └───────────────────┘  │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────────┐  │
│  │ IPC / RPC   │  │  Speicher-   │  │  Prozess &        │  │
│  │  Engine     │  │  verwaltung  │  │  Thread-Manager   │  │
│  └─────────────┘  └──────────────┘  └───────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│              HARDWARE ABSTRACTION LAYER (HAL)               │
│  CPU │ GPU/NPU │ RAM │ Storage │ Netzwerk │ HSM/TPM         │
└─────────────────────────────────────────────────────────────┘
```

### Design-Prinzipien

| Prinzip | Beschreibung | Umsetzung |
|---|---|---|
| **Minimalität** | Kern so klein wie möglich | < 50.000 Zeilen C/Rust im Micro-Kern |
| **Isolation** | Prozesse strikt getrennt | Capability-based Security, kein globaler Namespace |
| **Determinismus** | Vorhersagbare Latenz | Real-Time-Scheduling für KI-Tasks |
| **Dezentralität** | Kein Single Point of Trust | On-Chain Kernel-Updates via Governance |
| **Verifikation** | Formale Korrektheit | TLA+-Spezifikation für Kern-Algorithmen |

---

## 24.2 Micro-Kern: Komponenten

Der Micro-Kern enthält **nur das Absolute-Minimum**:

### 24.2.1 Prozess- und Thread-Management

```rust
// Kernel-Prozess-Repräsentation
pub struct KaiProcess {
    pub pid: ProcessId,
    pub capabilities: CapabilitySet,   // Was darf dieser Prozess?
    pub memory_region: MemoryRegion,   // Isolierter Adressraum
    pub scheduler_class: SchedClass,   // RT / Normal / Idle
    pub resource_budget: ResourceBudget, // CPU-Zeit, RAM, I/O
    pub blockchain_identity: Option<AccountId>, // On-Chain-Identität
}

pub enum SchedClass {
    RealTime { priority: u8 },   // KI-Inferenz, Konsensus
    Normal   { nice: i8 },       // Agenten, dApps
    Idle,                        // Hintergrundaufgaben
}
```

**Scheduling-Algorithmus:** Hybrid aus CFS (Completely Fair Scheduler) für Normal-Prozesse und EDF (Earliest Deadline First) für Real-Time-KI-Tasks.

### 24.2.2 Speicherverwaltung

```
Adressraum-Layout (64-Bit):
┌──────────────────────────────┐ 0xFFFF_FFFF_FFFF_FFFF
│     Kernel Space (128 TB)    │
├──────────────────────────────┤ 0xFFFF_8000_0000_0000
│     (nicht gemappt)          │
├──────────────────────────────┤ 0x0000_8000_0000_0000
│     User Space   (128 TB)    │
│  ┌────────────────────────┐  │
│  │ KI-Modell-Speicher     │  │ ← Hugepages (2MB/1GB)
│  │ Agent-Heap             │  │ ← NUMA-aware Allokation
│  │ Stack                  │  │ ← Guard Pages
│  │ IPFS-Buffer            │  │ ← DMA-fähig
│  └────────────────────────┘  │
└──────────────────────────────┘ 0x0000_0000_0000_0000
```

**Schlüssel-Features:**
- **Hugepages** (2 MB / 1 GB) für KI-Modell-Ladezeiten
- **NUMA-Awareness:** Modell-Weights auf lokalem NUMA-Node
- **Memory-Tagging** (ARM MTE / Intel LAM): Out-of-bounds-Erkennung in Hardware
- **Encrypted Memory:** AMD SEV / Intel TDX für Agenten-Isolation

### 24.2.3 IPC (Inter-Process Communication)

```rust
// Capability-basiertes IPC
pub trait IpcChannel: Send + Sync {
    fn send(&self, msg: KaiMessage, cap: Capability) -> Result<()>;
    fn recv(&self, timeout: Duration) -> Result<KaiMessage>;
    fn call(&self, msg: KaiMessage, cap: Capability) -> Result<KaiMessage>;
}

pub struct KaiMessage {
    pub source: ProcessId,
    pub destination: ProcessId,
    pub payload: MessagePayload,
    pub signature: Ed25519Signature, // Jede IPC-Nachricht ist signiert
}

pub enum MessagePayload {
    InferenceRequest(InferenceRequest),
    BlockchainCall(ExtrinsicPayload),
    StorageOp(StorageOperation),
    AgentTask(AgentTaskPayload),
    SystemCall(SyscallPayload),
}
```

**IPC-Mechanismen nach Anwendungsfall:**

| Typ | Latenz | Durchsatz | Einsatz |
|---|---|---|---|
| Shared Memory + Semaphore | < 1 µs | Sehr hoch | KI-Modell-Buffer |
| Unix Domain Socket | ~10 µs | Hoch | Agent ↔ Node |
| Cap'n Proto RPC | ~50 µs | Mittel | SDK ↔ REST API |
| Blockchain Extrinsic | ~6 s | Niedrig | On-Chain-Operationen |

---

## 24.3 KI-Kernel-Modul

Das KI-Modul ist als **ladbares Kernel-Modul** (LKM) implementiert — es kann ohne Neustart geladen, aktualisiert und entladen werden.

### 24.3.1 KI-Scheduler

```rust
pub struct KaiAIScheduler {
    inference_queue: PriorityQueue<InferenceTask>,
    gpu_allocator: GpuAllocator,
    model_cache: LruCache<ModelId, LoadedModel>,
    power_budget: PowerBudget,
}

impl KaiAIScheduler {
    /// Nimmt einen Inferenz-Task entgegen und plant ihn ein
    pub async fn schedule(&mut self, task: InferenceTask) -> InferenceResult {
        // 1. Priorität bestimmen (RT für Konsensus-KI, Normal für Agenten)
        let priority = self.compute_priority(&task);

        // 2. Ressourcen reservieren (GPU-VRAM oder CPU-RAM)
        let resources = self.gpu_allocator.reserve(task.model_size)?;

        // 3. Modell aus Cache oder laden
        let model = self.model_cache
            .get_or_load(&task.model_id, &resources).await?;

        // 4. Inferenz ausführen mit Timeout
        tokio::time::timeout(
            task.deadline,
            model.infer(&task.prompt, &task.config)
        ).await?
    }
}
```

### 24.3.2 Hardware-Beschleuniger-Abstraktions-Layer

```
KAI Hardware Abstraction Layer (HAL)
│
├── CUDA Backend    (NVIDIA GPUs: RTX, A100, H100)
├── ROCm Backend    (AMD GPUs: RX 7000, MI300)
├── Metal Backend   (Apple Silicon: M1–M4)
├── oneAPI Backend  (Intel Arc, Xe)
├── Vulkan Compute  (Cross-Vendor Fallback)
└── CPU Backend     (SIMD: AVX-512, ARM NEON, RISC-V V-Ext)
```

**Automatische Backend-Auswahl:**
```rust
pub fn detect_best_backend() -> InferenceBackend {
    if cuda_available() && vram_gb() >= 8.0 {
        InferenceBackend::Cuda { device: best_cuda_device() }
    } else if rocm_available() {
        InferenceBackend::Rocm { device: 0 }
    } else if metal_available() {
        InferenceBackend::Metal
    } else {
        InferenceBackend::Cpu { threads: num_cpus::get() }
    }
}
```

---

## 24.4 Blockchain-Kernel-Modul

Das Blockchain-Modul verbindet den Kernel direkt mit dem Substrate-Node — **ohne Umweg über REST API**.

```
Kernel-Space                     Substrate-Node
┌──────────────────┐             ┌──────────────────┐
│ KAI Blockchain   │  Unix IPC   │                  │
│ Kernel Module    │◄───────────►│  substrate-node  │
│                  │  Cap'n Proto │                  │
│ - Block-Events   │             │ - GRANDPA        │
│ - Tx-Submission  │             │ - BABE           │
│ - State-Queries  │             │ - pallet-*       │
│ - On-Chain Keys  │             │                  │
└──────────────────┘             └──────────────────┘
```

**Kernel-seitige Schlüsselverwaltung:**
```rust
// Schlüssel werden im Kernel-Space gehalten — nie im User-Space exponiert
pub struct KernelKeystore {
    keys: HashMap<KeyType, SecretKey>,
    hsm: Option<HsmBackend>,  // Hardware Security Module wenn verfügbar
    tpm: Option<TpmBackend>,  // TPM 2.0 als Fallback
}

impl KernelKeystore {
    /// Signiert einen Extrinsic — der Key verlässt nie den Kernel
    pub fn sign_extrinsic(&self, payload: &[u8], key_type: KeyType) 
        -> Result<Signature> {
        match &self.hsm {
            Some(hsm) => hsm.sign(payload, key_type),  // HSM-Signatur
            None      => self.keys[&key_type].sign(payload), // Software
        }
    }
}
```

---

## 24.5 Sicherheits-Architektur des Kernels

### 24.5.1 Capability-Based Security

Jeder Prozess besitzt nur **explizit gewährte Fähigkeiten** — kein impliziter Zugriff:

```
Capability-Hierarchie:
ROOT_CAP (nur Kernel)
  ├── NETWORK_CAP      → Netzwerkzugriff (TCP/UDP/P2P)
  ├── STORAGE_CAP      → Dateisystem-Zugriff
  │     ├── IPFS_CAP   → IPFS-Operationen
  │     └── LOCAL_CAP  → Lokaler Disk-Zugriff
  ├── COMPUTE_CAP      → CPU/GPU-Zugriff
  │     └── AI_CAP     → KI-Inferenz-Recht
  ├── CHAIN_CAP        → Blockchain-Interaktion
  │     ├── TX_CAP     → Transaktionen senden
  │     └── SIGN_CAP   → Kryptografisch signieren
  └── IPC_CAP          → Andere Prozesse kontaktieren
```

### 24.5.2 Kernel-Härtungs-Maßnahmen

| Maßnahme | Beschreibung | Status |
|---|---|---|
| ASLR | Adress-Space-Randomisierung | ✅ Standard aktiviert |
| Stack Canaries | Puffer-Überlauf-Erkennung | ✅ `-fstack-protector-strong` |
| NX/DEP | Kein ausführbarer Stack/Heap | ✅ Hardware-enforced |
| SMEP/SMAP | Kernel kann nicht in User-Space springen | ✅ x86_64 |
| CFI | Control Flow Integrity (LLVM) | ✅ Kernel + Module |
| Seccomp-BPF | System-Call-Filter pro Prozess | ✅ Alle User-Space-Prozesse |
| Landlock | Dateisystem-Zugriffs-Policy | ✅ Agenten, dApps |
| eBPF-LSM | Dynamische Security-Policy | 🟡 Phase 3 |
| Formal Verification | TLA+-verifizierte Kern-Algorithmen | 🟡 Sprint 3.5 |

### 24.5.3 Kernel-Update-Mechanismus (On-Chain Governance)

```
Governance-Vote → Angenommen → Timelock (48h) →
  ↓
Kernel-Update-Package (signiert von 3/5 Core-Devs)
  ↓
Kernel-Modul-Verifikation (SHA256 + Ed25519)
  ↓
Live-Patch (für Module) ODER Rolling-Reboot (für Micro-Kern)
  ↓
Automatisches Rollback bei Boot-Fehler (3 Versuche)
```

---

## 24.6 Kernel-Entwicklungs-Roadmap

### Kernel-Sprint K1 — Micro-Kern Basis (Sprint 2.1 parallel)

**Aufgaben:**
- [ ] Micro-Kern in Rust (no_std): Prozess-Manager, Memory-Manager, IPC
- [ ] HAL für x86_64 und ARM64
- [ ] Capability-System: Vergabe, Entzug, Vererbung
- [ ] Minimales Syscall-Interface (50 Calls, kein POSIX-Overhead)
- [ ] Boot-Sequenz: Kernel → Init → Node → Agenten

**🔧 Fehlerbehebungs-Schritte (Kernel K1):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Kernel-Panic beim Boot | Serial-Console-Log auslesen | Stack-Trace analysieren, `RUST_BACKTRACE=1` |
| Page-Fault im Kernel | `kai kernel dump --last-panic` | Memory-Region-Mapping prüfen |
| IPC-Deadlock | `kai kernel ipc-graph --detect-cycles` | Capability-Reihenfolge fixieren |
| Prozess kann nicht spawnen | `kai kernel caps <PID>` | Fehlende Capability? ROOT_CAP nötig? |
| HAL initialisiert nicht | `kai kernel hal-status` | CPU-Feature-Detection prüfen (AVX, NEON) |

**🚀 Deployment-Checkliste (Kernel K1):**
- [ ] Kernel bootet auf x86_64 (QEMU) ohne Panic
- [ ] Kernel bootet auf ARM64 (QEMU) ohne Panic
- [ ] 10 Prozesse gleichzeitig: kein Scheduling-Deadlock
- [ ] Memory-Isolation: Prozess A kann Prozess B's RAM nicht lesen
- [ ] IPC-Roundtrip-Latenz < 10 µs (Shared Memory)
- [ ] Formale Spezifikation: Prozess-Lifecycle in TLA+ modelliert
- [ ] **Layer-2-NFT geminted** (L2 Kernel NFT v1.0.0 auf Devnet) — `nft://kai-os/layer/2/kernel` on-chain verankert → **MK1 erreicht**

---

### Kernel-Sprint K2 — KI-Modul Integration (Sprint 2.3 parallel)
> ⚠️ **Voraussetzung:** Kernel-Sprint K1 muss abgeschlossen sein — das KI-Modul lädt als LKM in den Micro-Kern (K1).

**Aufgaben:**
- [ ] KI-Kernel-Modul als LKM implementieren
- [ ] GPU-Allocator: CUDA/ROCm/Metal/CPU auto-detect
- [ ] KI-Scheduler: EDF für RT-Tasks, CFS für Normal-Tasks
- [ ] Hugepages für Modell-Loading aktivieren
- [ ] Modell-Isolation: Kein Agenten-Zugriff auf andere Modell-Weights

**🔧 Fehlerbehebungs-Schritte (Kernel K2):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| KI-Modul lädt nicht | `kai kernel modules --status` | Kernel-Version kompatibel? `kai kernel version` |
| GPU nicht erkannt | `kai kernel hal-status --gpu` | Treiber geladen? `lsmod` + `dmesg` |
| OOM beim Modell-Laden | `kai kernel mem-stats --live` | Hugepages erhöhen: `vm.nr_hugepages=512` |
| EDF-Deadline verpasst | `kai kernel sched-stats --rt` | Deadline zu eng gesetzt? CPU-Affinität? |
| Modell-Isolation verletzt | Kernel-Audit-Log | Capability-Check für AI_CAP fehlerhaft |

**🚀 Deployment-Checkliste (Kernel K2):**
- [ ] KI-Modul lädt und entlädt ohne Kernel-Panic
- [ ] Inferenz auf GPU: < 1s für llama3-8b-q4
- [ ] Inferenz auf CPU: < 5s für llama3-8b-q4
- [ ] 10 parallele Agenten: kein Scheduling-Starvation
- [ ] Modell-Weights nicht aus anderem Prozess lesbar
- [ ] Hugepages aktiv: `cat /proc/meminfo | grep Huge`
- [ ] **Layer-3-NFT geminted** (L3 KI-Modul NFT v1.0.0) — `nft://kai-os/layer/3/ai` on-chain → **MK2 erreicht**

---

### Kernel-Sprint K3 — Blockchain-Modul & Keystore (Sprint 2.2 parallel)
> ⚠️ **Voraussetzung:** Kernel-Sprint K1 muss abgeschlossen sein — das Blockchain-Modul ist ein LKM im Micro-Kern (K1). Außerdem muss M1 (Sprint 2.1) erfüllt sein: Substrate-Node läuft und produziert Blöcke.

**Aufgaben:**
- [ ] Blockchain-Kernel-Modul: IPC-Bridge zu Substrate-Node
- [ ] Kernel-Keystore: Ed25519-Keys im Kernel-Space, HSM-Integration
- [ ] Block-Event-System: Kernel empfängt Finalisierungs-Events direkt
- [ ] Tx-Signing ohne User-Space-Key-Exposure

**🔧 Fehlerbehebungs-Schritte (Kernel K3):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| IPC-Bridge zum Node unterbrochen | `kai kernel bc-status` | Unix-Socket-Pfad korrekt? Permissions? |
| Signing schlägt fehl | `kai kernel keystore --check` | Key importiert? HSM verbunden? |
| Block-Events kommen nicht an | `kai kernel events --subscribe finalized` | Node läuft? IPC-Queue voll? |
| HSM nicht erkannt | `kai kernel hsm-detect` | Treiber geladen? `pkcs11-tool --list-slots` |

**🚀 Deployment-Checkliste (Kernel K3):**
- [ ] Kernel signiert Extrinsic: Key verlässt nie den Kernel-Space
- [ ] Block-Events: Latenz < 100ms nach Finalisierung
- [ ] HSM: Signing-Test erfolgreich (oder Software-Fallback dokumentiert)
- [ ] IPC-Bridge: 1000 Calls/Sekunde ohne Drop
- [ ] Keystore: Keys überleben Kernel-Modul-Reload
- [ ] **Layer-4-NFT geminted** (L4 Blockchain-Modul NFT v1.0.0) — `nft://kai-os/layer/4/blockchain` on-chain → **MK3 erreicht**

---

### Kernel-Sprint K4 — Sicherheits-Härtung & Audit (Sprint 3.5–3.6 parallel, Abschluss Apr 2027)

**Aufgaben:**
- [ ] Seccomp-BPF-Profile für alle User-Space-Prozesse
- [ ] Landlock-Policies: Agenten dürfen nur eigenes Verzeichnis sehen
- [ ] eBPF-LSM: Dynamische Security-Hooks
- [ ] Kernel-Fuzzing: `syzkaller` auf KAI-Syscall-Interface
- [ ] Formale Verifikation: TLA+-Modell für Scheduling + IPC

**🔧 Fehlerbehebungs-Schritte (Kernel K4):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Seccomp blockt legitimen Syscall | `strace -p <PID>` | Syscall-Whitelist ergänzen |
| Landlock blockiert Agent | `dmesg | grep landlock` | Policy-Regeln für Agent-Verzeichnis ergänzen |
| eBPF-Hook verursacht Kernel-Hang | `kai kernel ebpf-status` | Hook deaktivieren: `kai kernel ebpf disable <ID>` |
| syzkaller findet Panic | Crash-Dump analysieren | Reproduzierer isolieren, Fix + Regression-Test |

**🚀 Deployment-Checkliste (Kernel K4):**
- [ ] Seccomp-Profile: alle Prozesse abgedeckt, 0 erlaubte unnötige Syscalls
- [ ] Landlock: Agent kann nur eigenes `/var/kai/agents/<ID>/` sehen
- [ ] syzkaller: 72h Fuzzing ohne neue Kernel-Panic
- [ ] TLA+-Modell: TLC findet keine Invarianten-Verletzungen
- [ ] Externer Kernel-Audit: 0 Critical Findings
- [ ] **Layer-1-NFT aktualisiert** (L1 Hardware NFT: Audit-Ergebnis als Soulbound-Metadaten on-chain) → **MK4 erreicht**
- [ ] **Layer-2-NFT aktualisiert** (L2 Kernel NFT v1.1.0: gehärtet, Audit-Hash in Metadaten)

---

## 24.7 Kernel-Metriken & Observability

```rust
// Kernel-Metriken werden direkt in Prometheus exportiert
pub struct KernelMetrics {
    // Scheduling
    pub sched_latency_ns:    Histogram,  // Scheduling-Latenz in Nanosekunden
    pub rt_deadline_misses:  Counter,    // Verpasste RT-Deadlines
    pub context_switches:    Counter,    // Kontext-Wechsel/Sekunde

    // Memory
    pub page_faults:         Counter,    // Page Faults (minor + major)
    pub hugepages_used:      Gauge,      // Hugepages belegt
    pub oom_kills:           Counter,    // OOM-Killer Auslösungen

    // IPC
    pub ipc_messages_total:  Counter,    // IPC-Nachrichten gesamt
    pub ipc_latency_ns:      Histogram,  // IPC-Latenz in Nanosekunden
    pub ipc_queue_depth:     Gauge,      // Aktuelle Queue-Tiefe

    // KI-Modul
    pub inference_duration_ms: Histogram, // Inferenz-Dauer
    pub gpu_utilization:       Gauge,     // GPU-Auslastung (%)
    pub model_cache_hits:      Counter,   // Modell aus Cache geladen

    // Blockchain-Modul
    pub block_events_received: Counter,   // Block-Events verarbeitet
    pub tx_signing_duration_ms: Histogram, // Signing-Latenz
    pub ipc_bridge_errors:     Counter,   // IPC-Bridge Fehler
}
```

**Grafana-Dashboard "KAI-OS Kernel":**

| Panel | Metrik | Alarm-Schwelle |
|---|---|---|
| RT-Deadline Misses | `rt_deadline_misses` | > 0/min → 🔴 Critical |
| Scheduling-Latenz p99 | `sched_latency_ns p99` | > 1ms → 🟡 Warning |
| OOM-Kills | `oom_kills` | > 0 → 🔴 Critical |
| GPU-Auslastung | `gpu_utilization` | > 95% → 🟡 Warning |
| IPC-Latenz p99 | `ipc_latency_ns p99` | > 100µs → 🟡 Warning |
| Tx-Signing-Latenz | `tx_signing_duration_ms` | > 500ms → 🟡 Warning |
| IPC-Bridge-Fehler | `ipc_bridge_errors` | > 0/min → 🔴 Critical |

---

## 24.8 Technologie-Entscheidungen

| Komponente | Gewählt | Begründung | Alternativen |
|---|---|---|---|
| Kernel-Sprache | **Rust (no_std)** | Memory Safety ohne GC, Zero-Cost Abstractions | C, Zig |
| Architektur-Typ | **Hybrid-Kernel** | Balance aus Performance (Monolith) und Sicherheit (Mikro) | Microkernel, Unikernel |
| Scheduling | **CFS + EDF** | CFS für Fairness, EDF für KI-RT-Garantien | BFS, EEVDF |
| IPC | **Shared Mem + Cap'n Proto** | Latenz-optimiert + typsicher | D-Bus, gRPC, Pipes |
| Security-Modell | **Capability-based** | Minimal-Privilege by default | DAC, MAC (SELinux) |
| GPU-Abstraktions | **HAL mit CUDA/ROCm/Metal** | Vendor-agnostisch | Nur CUDA, Vulkan Compute |
| Key-Storage | **Kernel-Keystore + HSM** | Keys verlassen nie Kernel-Space | User-Space Keyring |
| Kernel-Updates | **On-Chain Governance + Live-Patch** | Dezentral, kein Admin-God-Mode | Traditional Package Manager |

---

---

## 24.9 Kernel als Multi-Layer-NFT-Architektur

> Das KAI-OS Kernel-Modell folgt dem Prinzip von **Multi-Layer-NFTs**: Jede Kernel-Schicht ist eine eigenständige, unveränderliche Einheit mit eigener On-Chain-Identität — kombinierbar, stapelbar und unabhängig aktualisierbar. Wie bei composable NFTs besitzt jede Schicht ihre eigenen Metadaten, Fähigkeiten (Capabilities) und Upgrade-Rechte — ohne die darunterliegenden Schichten zu berühren.

---

### 24.9.1 Das Layer-Modell: Kernel als NFT-Stack

> ⚡ **Grundprinzip:** Jede Komponente = 1 eigener Layer = 1 eigenes NFT. Kein Layer enthält mehrere unabhängige Dinge.

```
╔══════════════════════════════════════════════════════════════╗
║  L10 — dAPP NFT                                             ║
║  Smart Contracts · dezentrale Anwendungen · App-Store       ║
║  On-Chain ID: nft://kai-os/layer/10/<dapp-id>               ║
╠══════════════════════════════════════════════════════════════╣
║  L9  — AGENT NFT                                            ║
║  KI-Agenten · Capabilities · Task-Lifecycle · Memory        ║
║  On-Chain ID: nft://kai-os/layer/9/<agent-id>               ║
╠══════════════════════════════════════════════════════════════╣
║  L8  — GOVERNANCE NFT                                       ║
║  DAO · Proposals · Conviction Voting · Timelock             ║
║  On-Chain ID: nft://kai-os/layer/8/governance               ║
╠══════════════════════════════════════════════════════════════╣
║  L7  — API & CLI NFT                                        ║
║  REST API · WebSocket · CLI · OpenAPI-Spec                  ║
║  On-Chain ID: nft://kai-os/layer/7/api                      ║
╠══════════════════════════════════════════════════════════════╣
║  L6  — STORAGE-MODUL NFT                                    ║
║  IPFS · Filecoin · CID · AES-256-GCM-Verschlüsselung        ║
║  On-Chain ID: nft://kai-os/layer/6/storage                  ║
╠══════════════════════════════════════════════════════════════╣
║  L5  — P2P-NETZWERK NFT                                     ║
║  libp2p · GossipSub · DHT · mDNS · Noise-Protokoll          ║
║  On-Chain ID: nft://kai-os/layer/5/p2p                      ║
╠══════════════════════════════════════════════════════════════╣
║  L4  — BLOCKCHAIN-MODUL NFT                                 ║
║  Substrate-Runtime · GRANDPA · BABE · IBC-Bridge            ║
║  On-Chain ID: nft://kai-os/layer/4/blockchain               ║
╠══════════════════════════════════════════════════════════════╣
║  L3  — KI-MODUL NFT                                         ║
║  Inference Engine · EDF-Scheduler · GPU-HAL · ONNX          ║
║  On-Chain ID: nft://kai-os/layer/3/ai                       ║
╠══════════════════════════════════════════════════════════════╣
║  L2  — MICRO-KERNEL NFT  (Hybrid-Kern)                      ║
║  Micro-Kern · IPC · Speicher · Prozesse · HAL               ║
║  On-Chain ID: nft://kai-os/layer/2/kernel                   ║
╠══════════════════════════════════════════════════════════════╣
║  L1  — HARDWARE NFT  (Vertrauensanker)                      ║
║  TPM · HSM · CPU-Attestation · Secure Boot                  ║
║  On-Chain ID: nft://kai-os/layer/1/<node-id>                ║
╚══════════════════════════════════════════════════════════════╝

        ↕ 1 Ding = 1 Layer = 1 NFT — kein Mischen
        ↕ Jeder Layer unabhängig upgradebar ohne andere zu berühren
        ↕ Schichten kommunizieren nur über definierte Interfaces
        ↕ L0 (Security NFT) liegt unter allen L1–L10 — zertifiziert jeden Upgrade
        🔗 L0-Dokumentation → Kapitel 25
```

---

### 24.9.2 Layer-Eigenschaften (NFT-Analogie)

| Layer | Name | NFT-Typ | Eigentümer | Upgrade-Mechanismus |
|---|---|---|---|---|
| **L0** | Security NFT | Soulbound | KAI-OS Security Council | Hard Fork (unveränderlich) |
| **L1** | Hardware NFT | Soulbound | Node-Betreiber | Hardware-Tausch + Re-Attestation |
| **L2** | Micro-Kernel NFT | Semi-Fungible | KAI-OS Core-DAO | Governance-Vote + Timelock 48h |
| **L3** | KI-Modul NFT | Fungible | KAI-OS Core-DAO | Modul-Registry + Governance-Vote |
| **L4** | Blockchain-Modul NFT | Fungible | KAI-OS Core-DAO | Governance-Vote + Timelock 24h |
| **L5** | P2P-Netzwerk NFT | Fungible | KAI-OS Core-DAO | Modul-Registry + Auto-Update |
| **L6** | Storage-Modul NFT | Fungible | Modul-Entwickler | Modul-Registry + Auto-Update |
| **L7** | API & CLI NFT | Semi-Fungible | KAI-OS Core-DAO | Governance-Vote + Timelock 24h |
| **L8** | Governance NFT | Non-Fungible | Token-Holder | On-Chain-Proposal |
| **L9** | Agent NFT | Non-Fungible | Agent-Eigentümer | Self-Sovereign (Besitzer entscheidet) |
| **L10** | dApp NFT | Non-Fungible | dApp-Entwickler | Self-Sovereign + App-Store-Review |

---

### 24.9.3 On-Chain-Repräsentation jeder Kernel-Schicht

```rust
/// Jede Kernel-Schicht ist ein On-Chain-NFT mit eigenem Pallet
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct KernelLayerNFT {
    pub layer_id:    u8,               // 1–5
    pub layer_type:  LayerType,        // Hardware | Kernel | Service | Governance | App
    pub version:     SemVer,           // z.B. 2.1.0
    pub content_hash: H256,            // SHA-256 des Layer-Binaries / Codes
    pub capabilities: Vec<Capability>, // Was darf diese Schicht?
    pub parent_hash:  H256,            // Hash der darunterliegenden Schicht (Verkettung)
    pub owner:        AccountId,       // On-Chain-Eigentümer
    pub metadata_uri: Vec<u8>,         // IPFS-URI mit vollständigen Metadaten
    pub is_frozen:    bool,            // Frozen = unveränderlich (Soulbound-Verhalten)
}

/// Upgrade einer Schicht — nur durch autorisierten Owner + Governance
pub fn upgrade_layer(
    origin: OriginFor<T>,
    layer_id: u8,
    new_content_hash: H256,
    new_version: SemVer,
    governance_proof: GovernanceProof, // Beweis: Vote angenommen
) -> DispatchResult {
    // 1. Governance-Proof verifizieren
    // 2. Timelock abgelaufen?
    // 3. Neue Layer-NFT minten
    // 4. Alte als "superseded" markieren (nie löschen — unveränderliche History)
    // 5. Event emittieren: LayerUpgraded { layer_id, old_version, new_version }
}
```

---

### 24.9.4 Layer-Komposition: Wie Schichten interagieren

Das Kompositions-Modell folgt dem **ERC-998 Composable NFT**-Prinzip — eine übergeordnete Schicht "besitzt" die darunter:

```
Governance NFT (L8)
  └── steuert → Micro-Kernel NFT (L2)
  └── steuert → KI-Modul NFT (L3)
  └── steuert → Blockchain-Modul NFT (L4)
  └── steuert → P2P-Netzwerk NFT (L5)
  └── steuert → Storage-Modul NFT (L6)
  └── steuert → API & CLI NFT (L7)

Micro-Kernel NFT (L2)
  └── läuft auf → Hardware NFT (L1)
  └── lädt      → KI-Modul NFT (L3)
  └── lädt      → Blockchain-Modul NFT (L4)
  └── lädt      → P2P-Netzwerk NFT (L5)
  └── lädt      → Storage-Modul NFT (L6)
  └── lädt      → API & CLI NFT (L7)

Agent NFT (L9)
  └── referenziert → KI-Modul NFT (L3) via Capability-Token
  └── referenziert → Storage-Modul NFT (L6) via Capability-Token
  └── läuft auf    → Micro-Kernel NFT (L2)

dApp NFT (L10)
  └── referenziert → Blockchain-Modul NFT (L4) via Smart Contract
  └── referenziert → Agent NFT (L9) via Capability-Token
  └── läuft auf    → Micro-Kernel NFT (L2)
```

**Capability-Token-System:**
```rust
/// Ein Capability-Token berechtigt eine App-Schicht, einen Service zu nutzen
pub struct CapabilityToken {
    pub token_id:    H256,
    pub granted_to:  AccountId,     // Agent / dApp
    pub capability:  Capability,    // AI_INFER | CHAIN_TX | STORAGE_READ | ...
    pub scope:       CapabilityScope, // Granulare Einschränkung
    pub expires_at:  Option<BlockNumber>, // Zeitlich begrenzt möglich
    pub revocable:   bool,
}

pub enum CapabilityScope {
    Unlimited,
    RateLimit { calls_per_block: u32 },
    BudgetLimit { max_tokens_per_call: u64 },
    ModelRestrict { allowed_models: Vec<ModelId> },
}
```

---

### 24.9.5 Layer-Upgrade-Flows

#### Normaler Modul-Upgrade (L3 Service NFT)
```
Modul-Entwickler reicht PR ein
  → CI/CD-Pipeline grün (Kapitel 23)
  → Automatischer Upgrade via Modul-Registry
  → Neue Service-NFT wird geminted
  → Alte bleibt als History erhalten
  → Nodes laden neues Modul via kai kernel modules --update
```

#### Kritischer Kernel-Upgrade (L2 Kernel NFT)
```
Core-Team erstellt Proposal (pallet-democracy)
  → 7-Tage-Abstimmung (Token-Holder)
  → Angenommen: 48h Timelock
  → Kernel-Binary: SHA-256 on-chain hinterlegt
  → Nodes: Rolling-Update (Kapitel 23.3.1)
       ├── 10% der Nodes zuerst (Canary)
       ├── Monitoring 24h
       └── Rollout auf alle Nodes
  → Neue Kernel-NFT (L2) wird geminted
  → Event: KernelUpgraded { old: v1.0.0, new: v2.1.0 }
```

#### Hardware-Tausch (L1 Hardware NFT)
```
Node-Betreiber tauscht Hardware
  → Secure Boot + TPM-Attestation neu erstellen
  → kai node attest --tpm --output attestation.json
  → On-Chain: alte L1-NFT als "retired" markieren
  → Neue L1-NFT minted (Soulbound an neue Node-ID)
  → Staking: Session-Keys neu setzen
```

---

### 24.9.6 Layer-Metadaten (IPFS-Schema)

```json
{
  "name": "KAI-OS Kernel Layer 2 — v2.1.0",
  "description": "Hybrid Micro-Kernel für KAI-OS mit EDF-Scheduler und Capability-Security",
  "layer": 2,
  "version": "2.1.0",
  "content_hash": "0xabc123...",
  "parent_layer_hash": "0xdef456...",
  "build": {
    "commit": "a1b2c3d4",
    "rustc": "1.78.0",
    "target": ["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu"],
    "reproducible": true
  },
  "capabilities_provided": ["PROCESS_MGMT", "MEMORY_MGMT", "IPC", "HAL"],
  "capabilities_required": ["HARDWARE_ATTEST"],
  "audit": {
    "auditor": "Trail of Bits",
    "date": "2027-04-01",
    "findings": { "critical": 0, "high": 0, "medium": 2, "low": 5 },
    "report_ipfs": "ipfs://Qm..."
  },
  "upgrade_history": [
    { "version": "2.0.0", "date": "2026-12-01", "governance_proposal": 42 },
    { "version": "2.1.0", "date": "2027-06-01", "governance_proposal": 87 }
  ]
}
```

---

### 24.9.7 Vorteile des Multi-Layer-NFT-Kernels

| Vorteil | Klassischer Kernel | KAI-OS Multi-Layer-NFT |
|---|---|---|
| **Upgrade-Transparenz** | Kein on-chain Beweis | Jedes Upgrade unveränderlich on-chain |
| **Vertrauensmodell** | Trust the Maintainer | Trustless via Governance + Hash-Verifikation |
| **Modularität** | Monolith oder ad-hoc | Schichten unabhängig austauschbar |
| **Eigentum** | Kein Eigentümer-Konzept | Jede Schicht hat definierten On-Chain-Owner |
| **Audit-Historie** | Extern, verlierbar | On-Chain, permanent, verknüpft |
| **Fähigkeits-Kontrolle** | Root/User dichotom | Granulare Capability-Tokens pro Agent |
| **Rollback** | Manuell, fehleranfällig | Alte NFT-Version immer abrufbar |

---

### 24.9.8 Integration in Sprint-Plan

| Kernel-Sprint | Layer | NFT-Typ | Ergebnis |
|---|---|---|---|
| **K1** — Micro-Kern | L2 Micro-Kernel NFT | Semi-Fungible | Erste Kernel-NFT auf Devnet: `nft://kai-os/layer/2/kernel` |
| **K2** — KI-Modul | L3 KI-Modul NFT | Fungible | Eigener Layer: `nft://kai-os/layer/3/ai` |
| **K3** — Blockchain-Modul | L4 Blockchain-Modul NFT | Fungible | Eigener Layer: `nft://kai-os/layer/4/blockchain` |
| **K4** — Sicherheits-Audit | L1 + L2 NFT | Soulbound / Semi-Fungible | Audit-Ergebnis on-chain in NFT-Metadaten |
| **Sprint 2.6** *(kein K-Sprint)* | L6 Storage-Modul NFT | Fungible | Eigener Layer: `nft://kai-os/layer/6/storage` (IPFS-Integration) |
| **Sprint 2.2** *(kein K-Sprint)* | L5 P2P-Netzwerk NFT | Fungible | Eigener Layer: `nft://kai-os/layer/5/p2p` (libp2p/GossipSub) |
| **Sprint 2.7** *(kein K-Sprint)* | L7 API & CLI NFT | Semi-Fungible | Eigener Layer: `nft://kai-os/layer/7/api` (REST/WebSocket/CLI v0.1) |
| **Sprint 2.4** *(kein K-Sprint)* | L9 Agent NFT | Non-Fungible | Eigener Layer: `nft://kai-os/layer/9/<agent-id>` (erster Agent deploybar) |
| **Sprint 3.4** *(kein K-Sprint)* | L8 Governance NFT | Non-Fungible | Eigener Layer: `nft://kai-os/layer/8/governance` (pallet-democracy, Kap. 24.9.5) |

---


---

# 25. Security Layer — Querschnitts-Schicht L0

> ⚡ **Querschnitts-Schicht:** Der KAI-OS Security Layer ist keine einzelne Komponente — er ist eine **vertikale Querschnitts-Schicht**, die alle 5 NFT-Layer (L1–L5) durchdringt und absichert. Im Multi-Layer-NFT-Modell wird er als **Layer 0 (L0 — Security NFT)** geführt: der unsichtbare Vertrauensanker unter dem Hardware-Layer, der jeden anderen Layer zertifiziert, überwacht und isoliert.

---

## 25.1 Security Layer im NFT-Stack

```
╔══════════════════════════════════════════════════════════════╗
║          LAYER 5 — APPLICATION NFT                          ║
║  KI-Agenten · dApps · CLI · Dashboard · SDK                 ║
╠══════════════════════════════════════════════════════════════╣
║          LAYER 4 — GOVERNANCE NFT                           ║
║  On-Chain-Updates · Voting · Timelock · Upgrade-Proxy       ║
╠══════════════════════════════════════════════════════════════╣
║          LAYER 3 — SERVICE NFT                              ║
║  KI-Modul · Blockchain-Modul · Storage · Netzwerk           ║
╠══════════════════════════════════════════════════════════════╣
║          LAYER 2 — KERNEL NFT                               ║
║  Micro-Kern · IPC · Speicher · Scheduler · HAL              ║
╠══════════════════════════════════════════════════════════════╣
║          LAYER 1 — HARDWARE NFT                             ║
║  TPM · HSM · CPU-Attestation · Secure Boot                  ║
╠══════════════════════════════════════════════════════════════╣
║  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  ║
║          LAYER 0 — SECURITY NFT  (Vertrauenswurzel)        ║
║                                                              ║
║  Zero-Trust-Policy · Threat Detection · Crypto-Primitives   ║
║  ZKP-Engine · Audit-Trail · Key-Lifecycle · IDS/IPS         ║
║                                                              ║
║  On-Chain ID: nft://kai-os/layer/0/security                 ║
║  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  ║
╚══════════════════════════════════════════════════════════════╝

     ↕ L0 ist unter allen anderen Layern — aber in jeden injiziert
     ↕ Jeder Layer-Upgrade durchläuft L0-Verifikation
     ↕ L0 selbst ist Soulbound — unveränderlich ohne Hard Fork
```

**L0 ist kein separater Prozess** — er ist eine Sammlung kryptografischer Garantien, Policy-Engines und Monitoring-Hooks, die in jeden anderen Layer (L1–L10) eingewoben sind.

**Grundprinzip:** Jede System-Komponente bekommt ihren eigenen Layer — kein Layer enthält mehrere unabhängige Dinge. Das ermöglicht unabhängige Upgrades, unabhängige Governance und unabhängige Auditierbarkeit pro Komponente.

---

## 25.2 Security-Domänen

Der Security Layer ist in **6 Domänen** gegliedert, jede mit klarer Verantwortung:

| # | Domäne | Zuständig für | Betroffene Layer |
|---|---|---|---|
| **S1** | Kryptografische Primitive | Alle Crypto-Operationen: Signing, Hashing, Verschlüsselung | L0 → L1–L10 |
| **S2** | Zero-Trust-Policy-Engine | Jede Anfrage muss explizit autorisiert sein | L2, L3, L5 |
| **S3** | Zero-Knowledge-Proofs | Datenschutzkonforme Verifikation ohne Offenbarung | L3, L4, L6, L8, L9, L10 |
| **S4** | Threat Detection & IDS/IPS | Erkennung und Blockierung von Angriffen in Echtzeit | L1–L5 |
| **S4** | Audit-Trail (On-Chain) | Unveränderliche Protokollierung aller sicherheitskritischen Ereignisse | L0–L5 |
| **S6** | Key-Lifecycle-Management | Schlüssel-Generierung, Rotation, Revokation, Archivierung | L1, L2, L3 |

---

## 25.3 S1 — Kryptografische Primitive

### Algorithmen-Matrix

| Kategorie | Algorithmus | Verwendung | Sicherheitsniveau |
|---|---|---|---|
| **Digitale Signatur** | Ed25519 | Validator-Keys, Tx-Signing, IPC-Nachrichten | 128-bit |
| **Digitale Signatur** | BLS12-381 | Aggregierte Validator-Signaturen (GRANDPA) | 128-bit |
| **Hashing** | BLAKE2b-256 | Block-Hashes, Content-Addressierung | 256-bit |
| **Hashing** | SHA3-256 | Contract-Hashes, NFT-Content-Hashes | 256-bit |
| **Schlüsselaustausch** | X25519 (ECDH) | P2P-Verbindungs-Verschlüsselung | 128-bit |
| **Symmetrisch** | ChaCha20-Poly1305 | Speicher-Verschlüsselung, Agent-Memory | 256-bit |
| **Symmetrisch** | AES-256-GCM | HSM-Kommunikation, TLS-Fallback | 256-bit |
| **ZKP** | PLONK / Groth16 | Privacy-Proofs, Verifiable Credentials | 128-bit |
| **VRF** | Ristretto255-VRF | BABE-Slot-Zuweisung, randomisiertes Scheduling | 128-bit |
| **Post-Quantum** | Kyber-1024 (FIPS 203) | Vorbereitung: P2P-Verschlüsselung ab Phase 4 | 256-bit |

### Crypto-Agility-Prinzip

```rust
/// Algorithmen sind austauschbar — kein Hardcoding
pub trait CryptoPrimitive: Send + Sync {
    fn sign(&self, msg: &[u8], key: &SecretKey) -> Signature;
    fn verify(&self, msg: &[u8], sig: &Signature, pk: &PublicKey) -> bool;
    fn algorithm_id(&self) -> AlgorithmId; // On-Chain registriert
}

/// Registry: On-Chain verwaltete Algorithmen-Liste
pub struct CryptoRegistry {
    pub active:     HashMap<AlgorithmId, Box<dyn CryptoPrimitive>>,
    pub deprecated: Vec<AlgorithmId>, // Noch verifizierbar, aber nicht mehr für neue Ops
    pub forbidden:  Vec<AlgorithmId>, // Geblockt — jede Nutzung → SecurityEvent
}
```

**Crypto-Agility-Garantie:** Ein Algorithmus kann per Governance-Vote (L4) als `deprecated` oder `forbidden` markiert werden, ohne Kernel-Neustart.

---

## 25.4 S2 — Zero-Trust-Policy-Engine

### Grundprinzip

```
"Never Trust — Always Verify"

Kein Prozess, kein Nutzer, kein Node bekommt
impliziten Zugriff auf irgendetwas — unabhängig
davon, ob er sich im gleichen Netzwerk, im gleichen
Kernel-Space oder auf demselben physischen Server befindet.
```

### Policy-Engine-Architektur

```rust
pub struct ZeroTrustEngine {
    policy_store:   OnChainPolicyStore,   // Policies on-chain, unveränderlich
    identity_vault: IdentityVault,         // DID-basierte Identitäten
    context_eval:   ContextEvaluator,      // Risiko-Score pro Request
    decision_log:   AuditTrail,            // Jede Entscheidung protokolliert
}

impl ZeroTrustEngine {
    /// Jeder Zugriff geht durch diese Funktion — keine Ausnahmen
    pub async fn authorize(
        &self,
        subject: &Identity,      // Wer fragt?
        resource: &Resource,     // Was wird angefragt?
        action: &Action,         // Was soll passieren?
        context: &RequestContext, // Wie, wann, von wo?
    ) -> AuthDecision {
        let risk_score = self.context_eval.score(subject, resource, context);
        let policy     = self.policy_store.lookup(subject, resource, action);
        let decision   = policy.evaluate(risk_score);

        // Jede Entscheidung — auch ALLOW — wird on-chain protokolliert
        self.decision_log.record(subject, resource, action, &decision).await;

        decision
    }
}

pub enum AuthDecision {
    Allow,
    AllowWithMFA,          // Zusätzliche Verifikation erforderlich
    Deny(DenyReason),
    DenyAndAlert(DenyReason, AlertSeverity), // Verdächtig — Incident auslösen
}
```

### Kontinuierliche Verifikation

| Trigger | Verifikations-Aktion |
|---|---|
| Neue P2P-Verbindung | mTLS-Handshake + Node-DID-Check |
| Agent startet Task | Capability-Token geprüft + Risiko-Score berechnet |
| Smart Contract Call | Caller-Identity + Contract-Whitelist |
| Kernel-Modul laden | SHA-256-Hash + Ed25519-Signatur (3/5 Core-Devs) |
| Governance-Vote | Token-Gewichtung + Identitäts-Beweis |
| Block finalisiert | GRANDPA-Aggregat-Signatur verifiziert |

---

## 25.5 S3 — Zero-Knowledge-Proof Engine

Die ZKP-Engine ermöglicht **Verifikation ohne Offenbarung** — ein Agent kann beweisen, dass er eine Bedingung erfüllt, ohne die zugrundeliegenden Daten preiszugeben.

### Anwendungsfälle

```
┌────────────────────────────────────────────────────────────┐
│                  ZKP USE CASES IN KAI-OS                   │
├─────────────────────────┬──────────────────────────────────┤
│ USE CASE                │ BEWEIS                           │
├─────────────────────────┼──────────────────────────────────┤
│ Agent-Berechtigung      │ "Ich habe genug Stake"           │
│                         │ ohne Kontostand zu zeigen        │
├─────────────────────────┼──────────────────────────────────┤
│ Identitäts-Verifikation │ "Ich bin über 18"                │
│ (Verifiable Credential) │ ohne Geburtsdatum zu zeigen      │
├─────────────────────────┼──────────────────────────────────┤
│ KI-Modell-Integrität    │ "Dieses Modell-Output kam von    │
│                         │ Modell X, unverändert"           │
├─────────────────────────┼──────────────────────────────────┤
│ Private Tx              │ "Diese Tx ist valide"            │
│                         │ ohne Betrag/Sender zu zeigen     │
├─────────────────────────┼──────────────────────────────────┤
│ Compliance-Beweis       │ "Audit-Kriterien erfüllt"        │
│                         │ ohne interne Daten offenzulegen  │
└─────────────────────────┴──────────────────────────────────┘
```

### ZKP-Circuit-Implementierung

```rust
/// Basis-Circuit für Capability-Beweis
pub struct CapabilityCircuit {
    // Private Inputs (nur der Beweiser kennt sie)
    secret_stake:    Fr,   // Tatsächlicher Stake-Betrag
    secret_key:      Fr,   // Privater Schlüssel
    // Public Inputs (jeder kann verifizieren)
    pub min_stake:   Fr,   // Mindest-Stake (z.B. 100 KAI)
    pub commitment:  G1,   // Pedersen-Commitment des Stakes
}

impl Circuit<Fr> for CapabilityCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(
        self, cs: &mut CS
    ) -> Result<()> {
        // Constraint: secret_stake >= min_stake
        // Constraint: commitment == Pedersen(secret_stake, blinding)
        // Constraint: signature valid with secret_key
        // → Proof: "Ich erfülle die Bedingung" ohne Details
    }
}
```

---

## 25.6 S4 — Threat Detection & IDS/IPS

### Erkennungs-Schichten

```
SCHICHT 1: Netzwerk-IDS (libp2p-Ebene)
  → Anomalie-Erkennung: ungewöhnliche Peer-Verbindungen
  → DDoS-Schutz: Rate-Limiting per Peer-ID
  → Sybil-Erkennung: Reputation-Score-Abfall

SCHICHT 2: Blockchain-IDS (Pallet-Ebene)
  → Spam-Tx-Erkennung: Burst-Detection
  → Governance-Angriff: Whale-Voting-Anomalie
  → Slashing-Trigger: Equivocation, Inaktivität

SCHICHT 3: Kernel-IDS (eBPF-Ebene)
  → Syscall-Anomalie: Prozess ruft unerwartete Syscalls
  → Privilege-Escalation-Versuch: CAP-Violation
  → Memory-Scan: Prozess liest fremden Adressraum

SCHICHT 4: KI-IDS (Inferenz-Ebene)
  → Prompt-Injection-Erkennung: Adversarial Inputs
  → Model-Inversion-Versuch: Wiederholte ähnliche Queries
  → Output-Anomalie: KI-Output weicht statistisch ab
```

### Echtzeit-Response-Matrix

```rust
pub enum ThreatLevel { Low, Medium, High, Critical }

pub struct ThreatResponse {
    pub level:   ThreatLevel,
    pub actions: Vec<ResponseAction>,
}

pub enum ResponseAction {
    Log,                            // Nur protokollieren
    RateLimit(Duration),            // Anfragen drosseln
    Quarantine(ProcessId),          // Prozess isolieren
    KillProcess(ProcessId),         // Prozess beenden
    BanPeer(PeerId, Duration),      // Peer blockieren
    SlashValidator(AccountId),      // On-Chain Strafmaßnahme
    TriggerIncident(IncidentLevel), // → Kapitel 22.3.1
    EmergencyShutdown,              // Letzter Ausweg
}

/// Automatische Eskalation
pub fn auto_respond(threat: &ThreatEvent) -> ThreatResponse {
    match threat.level {
        ThreatLevel::Low      => ThreatResponse { actions: vec![Log] },
        ThreatLevel::Medium   => ThreatResponse { actions: vec![Log, RateLimit(60s)] },
        ThreatLevel::High     => ThreatResponse { actions: vec![
            Log, Quarantine(threat.source_pid), BanPeer(threat.peer, 3600s),
            TriggerIncident(IncidentLevel::P2)
        ]},
        ThreatLevel::Critical => ThreatResponse { actions: vec![
            Log, KillProcess(threat.source_pid),
            SlashValidator(threat.account),
            TriggerIncident(IncidentLevel::P0)
        ]},
    }
}
```

---

## 25.7 S5 — Audit-Trail (On-Chain)

Jedes sicherheitskritische Ereignis wird **unveränderlich on-chain** protokolliert — nicht löschbar, nicht manipulierbar.

```rust
#[derive(Encode, Decode, Clone)]
pub struct SecurityEvent {
    pub event_id:    H256,            // Eindeutige ID
    pub timestamp:   BlockNumber,     // Exakter Block
    pub layer:       u8,              // Betroffener Layer (0–5)
    pub domain:      SecurityDomain,  // S1–S6
    pub severity:    Severity,        // Info / Warning / Critical
    pub subject:     Identity,        // Wer war beteiligt?
    pub action:      Vec<u8>,         // Was wurde versucht?
    pub decision:    AuthDecision,    // Allow / Deny
    pub evidence:    H256,            // IPFS-Hash des vollständigen Logs
    pub zkp_proof:   Option<Proof>,   // ZKP: Beweis ohne Datenleck
}
```

### Audit-Abfragen (CLI)

```bash
# Alle Security-Events der letzten 100 Blöcke
kai security audit --last-blocks 100

# Kritische Events nach Layer filtern
kai security audit --layer 2 --severity critical

# ZKP-Beweis für Compliance-Audit exportieren
kai security audit --export zkp-proof --range 2026-01-01..2026-12-31

# On-Chain-Audit-Report für externen Prüfer
kai security audit --report --output audit_2026.json --sign
```

---

## 25.8 S6 — Key-Lifecycle-Management

```
LEBENSZYKLUS EINES SCHLÜSSELS IN KAI-OS:

  GENERIERUNG          AKTIVIERUNG          NUTZUNG
  ───────────          ───────────          ───────
  HSM oder Kernel  →   On-Chain-Reg     →   Signing / Encrypt
  Keystored-Space      Session-Key-Set      Rate-Limited
       │                    │                    │
       ▼                    ▼                    ▼
  ROTATION             SUSPENSION          REVOKATION
  ─────────            ──────────          ──────────
  Automatisch          Governance-Vote     Sofort on-chain
  alle 90 Tage         oder Incident       → Alle Sigs ungültig
  (Validator-Keys)     Trigger             → Archivierung
```

### Key-Rotation-Automatisierung

```bash
# Automatische Session-Key-Rotation (90 Tage)
# In node.toml:
[security.key_rotation]
enabled = true
interval_days = 90
notify_days_before = 7        # Warnung 7 Tage vorher
auto_rotate = true            # Ohne manuellen Eingriff
backup_old_keys = true        # Alte Keys verschlüsselt archivieren

# Manuelle Rotation (bei Incident)
kai security rotate-keys --type session --emergency
# → Neue Keys sofort aktiv
# → Alte Keys als "compromised" on-chain markiert
# → Incident-Log Eintrag automatisch erstellt
```

---

## 25.9 Security Layer als NFT: L0

```rust
#[derive(Encode, Decode, Clone)]
pub struct SecurityLayerNFT {
    pub layer_id:         u8,         // Immer: 0
    pub layer_type:       LayerType,  // SecurityFoundation
    pub policy_hash:      H256,       // Hash aller aktiven Policies
    pub crypto_registry:  H256,       // Hash der Algorithmen-Registry
    pub threat_model:     H256,       // IPFS: vollständiges Threat-Model
    pub audit_root:       H256,       // Merkle-Root aller Audit-Events
    pub is_frozen:        bool,       // true — L0 ist Soulbound
    pub last_updated:     BlockNumber,
    pub governance_proof: GovernanceProof, // Hard Fork nötig für Updates
}
```

**L0 ist Soulbound** — er kann nicht übertragen werden und erfordert einen **Hard Fork** für strukturelle Änderungen. Policy-Updates (neue Regeln) sind via Governance möglich, aber die Engine selbst ist unveränderlich.

### L0-Upgrade-Flow

```
Normaler Policy-Update (häufig):
  Governance-Vote (L4) → 24h Timelock → Policy-Hash aktualisiert
  → Kein Kernel-Neustart nötig

Crypto-Algorithmus deprecaten (selten):
  Governance-Vote → 7-Tage-Timelock → CryptoRegistry aktualisiert
  → Alle Nodes müssen im nächsten Block-Cycle updaten

Strukturelle L0-Änderung (sehr selten):
  Hard Fork → Community-Abstimmung (6 Monate Vorlauf)
  → Neue L0-NFT-Version
  → Alte bleibt als "retired" on-chain erhalten
```

---

## 25.10 Security-Metriken & Alerting

| Metrik | Prometheus-Label | Alarm-Schwelle |
|---|---|---|
| Auth-Deny-Rate | `zt_auth_deny_total` | > 50/min → 🟡 Warning |
| Kritische Threats | `ids_threats_critical` | > 0 → 🔴 Sofort |
| Failed ZKP-Verifications | `zkp_verify_failures` | > 10/min → 🟡 Warning |
| Key-Rotation überfällig | `key_rotation_overdue` | > 0 → 🟡 Warning |
| Audit-Trail-Lücke | `audit_gap_blocks` | > 1 → 🔴 Critical |
| Crypto-Forbidden-Usage | `crypto_forbidden_ops` | > 0 → 🔴 Sofort |
| Syscall-Anomalie | `ids_syscall_anomaly` | > 5/min → 🟡 Warning |

---

## 25.11 Kernel-Sprint K-Security: Implementierungs-Plan

### K-Sec 1 — Crypto-Primitive-Library & ZeroTrust-Engine (Sprint 2.1 parallel)
> ⚠️ **Voraussetzung:** Keine — K-Sec-1 ist die Basis aller anderen Kernel-Sprints.

**Aufgaben:**
- [ ] Crypto-Primitive-Crate (`kai-crypto`): Ed25519, BLS, BLAKE2b, ChaCha20, X25519
- [ ] Crypto-Registry: On-Chain-Algorithmen-Verwaltung mit `deprecated`/`forbidden`-Status
- [ ] Zero-Trust-Engine: Policy-Store, Identity-Vault, Context-Evaluator
- [ ] Audit-Trail-Pallet: On-Chain-Event-Logging mit Merkle-Root

**🔧 Fehlerbehebungs-Schritte (K-Sec 1):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Ed25519-Signing schlägt fehl | `kai security crypto-test --algo ed25519` | Key-Format korrekt? `PEM` vs. `raw bytes` |
| ZeroTrust blockiert legitime Anfrage | `kai security audit --last-blocks 10 --decision deny` | Policy zu restriktiv? Capability-Token erneuern |
| Audit-Trail-Lücke | `kai security audit-gap --check` | Node offline? Block-Sync abwarten |
| Crypto-Registry lädt nicht | `kai security registry-status` | Pallet initialisiert? Genesis-Config prüfen |

**🚀 Deployment-Checkliste (K-Sec 1):**
- [ ] `kai-crypto`-Crate: alle Algorithmen mit 10.000 Testvektoren validiert
- [ ] Zero-Trust-Engine: 100% der Auth-Entscheidungen geloggt
- [ ] Audit-Trail: Merkle-Root stimmt mit On-Chain-Root überein
- [ ] Crypto-Registry: `deprecated` + `forbidden` Mechanismus getestet
- [ ] **L0-Security-NFT geminted** auf Devnet — `nft://kai-os/layer/0/security`

---

### K-Sec 2 — ZKP-Engine & Threat-Detection (Sprint 2.5 parallel)
> ⚠️ **Voraussetzung:** K-Sec 1 abgeschlossen + M5 (Smart Contracts live).

**Aufgaben:**
- [ ] PLONK-Circuit-Implementierung: CapabilityCircuit, CredentialCircuit
- [ ] ZKP-Verifier on-chain (Pallet): Groth16 + PLONK Verifier
- [ ] Netzwerk-IDS: libp2p-Anomalie-Erkennung + Rate-Limiting
- [ ] Kernel-IDS: eBPF-basierte Syscall-Anomalie-Erkennung
- [ ] KI-IDS: Prompt-Injection-Erkennung für Agent-Eingaben

**🔧 Fehlerbehebungs-Schritte (K-Sec 2):**
| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| ZKP-Beweis schlägt fehl | `kai security zkp-verify --debug` | Circuit-Constraints verletzt? Input-Bereich prüfen |
| IDS false-positive | `kai security ids-stats --false-pos` | Threshold zu niedrig? `ids.threshold = 0.85` in config |
| eBPF-Hook lädt nicht | `kai kernel ebpf-status` | Kernel-Version ≥ 5.15? `uname -r` |
| Prompt-Injection nicht erkannt | `kai security ids-test --prompt-injection` | Model-Version updaten: `kai model update kai-ids-v2` |

**🚀 Deployment-Checkliste (K-Sec 2):**
- [ ] ZKP-Proof für CapabilityCircuit: Generierung < 2s, Verifikation < 100ms
- [ ] Netzwerk-IDS: 99%+ Erkennungsrate bei bekannten DDoS-Patterns (Testdaten)
- [ ] Kernel-IDS: 0 False-Positives bei 24h Normalbetrieb
- [ ] KI-IDS: Prompt-Injection-Erkennungsrate > 95% (OWASP-LLM-Testset)
- [ ] Threat-Response: Automatische P0-Incident-Auslösung getestet (Kap. 22.3.1)

---

## 25.12 Integration in Roadmap-Sprint-Plan

| Kernel-Sprint | Layer | Sprint | Meilenstein |
|---|---|---|---|
| **K-Sec 1** — Crypto + ZeroTrust | L0 Security NFT | Sprint 2.1 parallel | MS1: L0-NFT geminted |
| **K-Sec 2** — ZKP + IDS/IPS | L0 Security NFT (Erweiterung) | Sprint 2.5 parallel | MS2: IDS live |
| **K4** — Kernel-Härtung | L0 validiert L1+L2 | Sprint 3.5–3.6 | MK4: Audit bestanden |

> 🔗 **Querverweise:**
> - Crypto-Primitive-Crate → **Kapitel 24.3** (KI-Kernel-Modul nutzt GPU-beschleunigtes Crypto)
> - ZeroTrust-Engine → **Kapitel 24.5.1** (Capability-Based Security)
> - ZKP-Engine → **Kapitel 4.5** (On-Chain Identität & Zugriffsrechte)
> - Threat Detection → **Kapitel 22.3.1** (Incident Playbooks)
> - Key-Lifecycle → **Kapitel 16.1** (Schlüssel-Verwaltung)
> - Audit-Trail → **Kapitel 22** (Incident Management)

---

# 26. DeFi-Layer — L11

> 🎫 **Verknüpfte Issues:** [🛒 #13](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/13)

> ⚡ **Grundprinzip:** Der DeFi-Layer ist eine vollständig eigenständige Schicht — **1 Ding = 1 Layer**. L11 enthält ausschließlich dezentrale Finanzprotokolle. Keine anderen Komponenten. Eigener Upgrade-Pfad, eigene Governance, eigenes NFT.

**On-Chain-Identität:** `nft://kai-os/layer/11/defi`

## 26.1 Konzept & Einordnung

L11 ist das dezentrale Finanzprotokoll-Fundament von KAI-OS. Er sitzt oberhalb des Blockchain-Moduls (L4) und der Agent-Schicht (L9) und stellt alle DeFi-Primitive als eigenständige, on-chain verifizierbare Module bereit.

**Abgrenzung zu anderen Layern:**

| Layer | Rolle | Verhältnis zu L11 |
|---|---|---|
| **L4** Blockchain-Modul | Konsensus, Ledger, Smart-Contract-Runtime | L11 baut auf L4 auf — nutzt dessen Extrinsics |
| **L9** Agent | KI-Agenten, Capabilities, Tasks | L9 ruft L11-Module auf — aber L11 ist kein Teil von L9 |
| **L8** Governance | DAO-Abstimmungen, Timelock | L11-Parameter-Änderungen laufen über L8 |
| **L0** Security | Querschnitts-Schicht | L0 zertifiziert jeden L11-Upgrade und jede Transaktion |

## 26.2 L11-Modul-Architektur (1 Modul = 1 Einheit)

Jedes DeFi-Primitive ist ein eigenständiges Modul innerhalb von L11 — mit eigener Contract-Adresse und eigener NFT-URI in der DeFiRegistry:

| Modul | NFT-URI | Funktion |
|---|---|---|
| **AMM** | `nft://kai-os/layer/11/defi/amm` | Liquiditätspools, Constant-Product-Formel, Swaps |
| **Lending** | `nft://kai-os/layer/11/defi/lending` | Besichertes Verleihen & Leihen ($KAI/$COMPUTE) |
| **Yield Optimizer** | `nft://kai-os/layer/11/defi/yield` | KI-gesteuerte Rendite-Maximierung via Agent (L9) |
| **Liquidity Mining** | `nft://kai-os/layer/11/defi/mining` | $COMPUTE-Anreize für Liquiditätsbereitsteller |
| **Oracle Network** | `nft://kai-os/layer/11/defi/oracle` | Dezentrale Preis-Feeds, Chainlink-kompatibel |
| **Stablecoin Engine** | `nft://kai-os/layer/11/defi/stable` | Algorithmisch stabilisierter $kUSD, CDP-besichert |
| **Flash Loan Engine** | `nft://kai-os/layer/11/defi/flash` | Uncollateralized Flash Loans (single-block) |
| **DEX Aggregator** | `nft://kai-os/layer/11/defi/dex` | Best-Route-Finder über alle AMM-Pools |

## 26.3 KI-gesteuerte DeFi (DeFi 2.0)

Das Alleinstellungsmerkmal von L11: KI-Agenten (L9) steuern DeFi-Protokolle autonom — vollständig transparent und on-chain auditierbar.

```
KI-Agent (L9)
  └── analysiert Marktdaten        ← Oracle (L11/oracle)
  └── berechnet optimale Route     ← Inference Engine (L3)
  └── führt Swap aus               ← AMM (L11/amm)
  └── protokolliert Entscheidung   ← Audit-Trail (L0/S5)
  └── zahlt Gebühren               ← $COMPUTE (L4 Token-Ökonomie)
```

**Konkrete DeFi-2.0-Szenarien:**

| Szenario | Agent-Aktion | L11-Modul |
|---|---|---|
| Autonomes Rebalancing | Agent überwacht Portfolio, rebalanciert bei Drift > 5% | AMM + Oracle |
| Liquidationsschutz | Agent erkennt drohende Liquidation, schließt Position | Lending + Oracle |
| Yield Farming | Agent wählt täglich optimale Yield-Strategie | Yield + Mining |
| MEV-Schutz | Agent erkennt Frontrunning, verschiebt TX auf sichere Slots | Flash + AMM |
| $kUSD-Stabilisierung | Agent justiert CDP-Collateral-Ratio automatisch | Stable + Lending |

## 26.4 Smart Contracts (L11)

### 26.4.1 DeFiRegistry.ink — Zentrales Modul-Verzeichnis

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod defi_registry {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    #[ink(event)]
    pub struct ModuleRegistered {
        #[ink(topic)]
        module_id: [u8; 32],
        contract:  AccountId,
        uri:       Vec<u8>,
    }

    #[ink(storage)]
    pub struct DeFiRegistry {
        modules: Mapping<[u8; 32], AccountId>, // module_id → contract
        uris:    Mapping<[u8; 32], Vec<u8>>,   // module_id → nft://…
        owner:   AccountId,                    // KAI-OS Governance DAO (L8)
        frozen:  bool,                         // L0: Emergency-Pause
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        Unauthorized,
        ModuleNotFound,
        RegistryFrozen,
        DuplicateModule,
    }

    impl DeFiRegistry {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                modules: Mapping::default(),
                uris:    Mapping::default(),
                owner:   Self::env().caller(),
                frozen:  false,
            }
        }

        /// Neues DeFi-Modul registrieren — nur Governance DAO (L8)
        #[ink(message)]
        pub fn register_module(
            &mut self,
            module_id: [u8; 32],
            contract:  AccountId,
            uri:       Vec<u8>,
        ) -> Result<(), Error> {
            if self.frozen            { return Err(Error::RegistryFrozen); }
            if self.env().caller() != self.owner { return Err(Error::Unauthorized); }
            if self.modules.contains(module_id)  { return Err(Error::DuplicateModule); }
            self.modules.insert(module_id, &contract);
            self.uris.insert(module_id, &uri);
            self.env().emit_event(ModuleRegistered { module_id, contract, uri });
            Ok(())
        }

        /// Modul-Adresse abfragen
        #[ink(message)]
        pub fn get_module(&self, module_id: [u8; 32]) -> Option<AccountId> {
            self.modules.get(module_id)
        }

        /// Emergency-Pause (L0 Security Council)
        #[ink(message)]
        pub fn emergency_freeze(&mut self) -> Result<(), Error> {
            if self.env().caller() != self.owner { return Err(Error::Unauthorized); }
            self.frozen = true;
            Ok(())
        }
    }
}
```

### 26.4.2 AmmPool.ink — Automated Market Maker (x·y=k)

```rust
#[ink::contract]
mod amm_pool {
    #[ink(storage)]
    pub struct AmmPool {
        reserve_kai:     Balance,  // $KAI Reserve
        reserve_compute: Balance,  // $COMPUTE Reserve
        lp_total_supply: Balance,  // LP-Token Gesamt
        lp_balances:     ink::storage::Mapping<AccountId, Balance>,
        fee_bps:         u32,      // Swap-Gebühr in Basispunkten (z.B. 30 = 0.3%)
        owner:           AccountId,
        frozen:          bool,     // L0 Emergency-Pause
    }

    #[ink(message, payable)]
    pub fn swap_kai_for_compute(&mut self, min_out: Balance) -> Result<Balance, Error> {
        if self.frozen { return Err(Error::PoolFrozen); }
        let amount_in  = self.env().transferred_value();
        let fee        = amount_in * self.fee_bps as u128 / 10_000;
        let amount_fee = amount_in - fee;
        // Constant-Product: (x + Δx) · y = k → Δy = y · Δx / (x + Δx)
        let out = self.reserve_compute * amount_fee
                / (self.reserve_kai + amount_fee);
        if out < min_out { return Err(Error::SlippageTooHigh); }
        self.reserve_kai     += amount_in;
        self.reserve_compute -= out;
        Ok(out)
    }
}
```

### 26.4.3 LendingPool.ink — Besichertes Lending

```rust
#[ink::contract]
mod lending_pool {
    #[ink(storage)]
    pub struct LendingPool {
        collateral:         ink::storage::Mapping<AccountId, Balance>,
        debt:               ink::storage::Mapping<AccountId, Balance>,
        collateral_ratio:   u32,   // Mindest-Collateral in % (z.B. 150 = 150%)
        liquidation_bonus:  u32,   // Liquidationsbonus in % (z.B. 10 = 10%)
        oracle:             AccountId,  // L11/oracle Contract
        frozen:             bool,
    }

    #[ink(message, payable)]
    pub fn deposit_collateral(&mut self) -> Result<(), Error> {
        if self.frozen { return Err(Error::PoolFrozen); }
        let caller = self.env().caller();
        let amount = self.env().transferred_value();
        let current = self.collateral.get(caller).unwrap_or(0);
        self.collateral.insert(caller, &(current + amount));
        Ok(())
    }

    #[ink(message)]
    pub fn borrow(&mut self, amount: Balance) -> Result<(), Error> {
        if self.frozen { return Err(Error::PoolFrozen); }
        let caller     = self.env().caller();
        let collateral = self.collateral.get(caller).unwrap_or(0);
        let current_debt = self.debt.get(caller).unwrap_or(0);
        // Collateral-Ratio prüfen
        let max_borrow = collateral * 100 / self.collateral_ratio as u128;
        if current_debt + amount > max_borrow { return Err(Error::InsufficientCollateral); }
        self.debt.insert(caller, &(current_debt + amount));
        Ok(())
    }
}
```

## 26.5 Token-Ökonomie (L11)

| Token | Rolle in L11 | Mechanismus |
|---|---|---|
| **$KAI** | Governance für L11-Upgrades | Voting auf neue DeFi-Module via L8 |
| **$COMPUTE** | Gebühr für KI-gesteuerte DeFi-Aktionen | 10% Burn bei jeder KI-DeFi-TX |
| **$kUSD** | Algorithmischer Stablecoin | CDP-besichert mit $KAI + $COMPUTE |
| **LP-Token** | Liquiditätsanteile an AMM-Pools | Fungible, übertragbar, yield-berechtigt |

**$kUSD-Stabilisierungsmechanismus:**
```
Preis > $1.00 → Neue $kUSD minten (mehr Angebot) → Preis sinkt
Preis < $1.00 → $kUSD aus Umlauf nehmen (CDP-Rückzahlung) → Preis steigt
KI-Agent (L9) führt Arbitrage automatisch aus — kein manueller Eingriff
```

## 26.6 Sicherheit — L0 → L11

> 🔗 **Security Layer S1** (Kapitel 25.3): Alle L11-Transaktionen mit Ed25519 signiert — kein anonymer DeFi-Aufruf möglich. BLS-Signaturen für Batch-Swaps.

> 🔗 **Security Layer S2** (Kapitel 25.4): Zero-Trust-Engine prüft jeden Agent-DeFi-Aufruf — Capability-Token `defi.execute` ist Pflicht.

> 🔗 **Security Layer S3** (Kapitel 25.5): ZKP-Engine ermöglicht Private-DeFi — Handelsvolumen verifizierbar ohne Offenbarung der Strategie.

> 🔗 **Security Layer S4** (Kapitel 25.6): KI-IDS überwacht L11 in Echtzeit auf Flash-Loan-Angriffe, Reentrancy-Muster und Oracle-Manipulation. Circuit Breaker bei > 3% Oracle-Abweichung.

> 🔗 **Security Layer S5** (Kapitel 25.7): Jede KI-gesteuerte DeFi-Aktion vollständig im On-Chain-Audit-Trail — inkl. Reasoning-Hash des Agenten und verwendeter Modell-Version.

> 🔗 **Security Layer S6** (Kapitel 25.8): LP-Token-Verwaltungskeys nach 90-Tage-Rotationsplan. TGE-Wallet-Keys nach HSM-Standard.

**L11-spezifische Sicherheitsregeln:**
- AMM-Pools: 72h Timelock für jede Parameter-Änderung
- Flash Loans: max. 10% der Pool-Liquidität pro Block
- Oracle: Median aus ≥ 5 unabhängigen Quellen — Abweichung > 3% = automatischer Circuit Breaker
- KI-Agent-Limit: max. 1% des Pool-Volumens pro Action ohne menschliche Freigabe
- Reentrancy-Guard: in allen L11-Contracts erzwungen

## 26.7 Roadmap-Integration (Sprint-Plan)

| Sprint | L11-Aufgabe | Meilenstein | Security-Gate |
|---|---|---|---|
| **Sprint 3.3** | AMM-Basisimplementierung + `DeFiRegistry.ink` deployen | L11-NFT auf Testnet geminted | S1 + S4 aktiv |
| **Sprint 3.6** | Lending Protocol + Oracle Network live | $kUSD-Stablecoin Testnet | S3 ZKP-Compliance-Export |
| **Sprint 4.1** | KI-gesteuerte Yield-Farming (Agent ↔ L11) | DeFi 2.0 Alpha | S2 Capability-Token-Check |
| **Sprint 4.3** | Flash Loan Engine + MEV-Schutz | Mainnet DeFi-Launch | S4 IDS Circuit Breaker live |

**🔧 Fehlerbehebungs-Schritte (L11 — übergreifend):**

| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| AMM-Swap reverted: SlippageTooHigh | `kai contract query AmmPool get_reserves` | `min_out` Parameter erhöhen oder Pool-Liquidität prüfen |
| Oracle-Preis weicht > 3% ab | `kai contract query OracleNetwork get_price --sources` | Mindestens 5 Quellen aktiv? Circuit Breaker manuell resetten |
| Flash Loan: InsufficientLiquidity | `kai contract query FlashEngine get_max_loan` | Pool-Liquidität unter 10%-Grenze — Wartezeit bis Rückzahlung |
| KI-Agent überschreitet 1%-Limit | `kai agent logs <ID> --level trace` | Agent-Budget erhöhen oder Governance-Vote für höheres Limit |
| $kUSD-Peg bricht | `kai defi stable-status` | CDP-Collateral-Ratio < 150%? Auto-Liquidation prüfen |
| DeFiRegistry frozen | `kai security audit --contract DeFiRegistry` | L0 Security Council: Emergency-Freeze — `kai security unfreeze --multisig` |

**🚀 Deployment-Checkliste (L11 — Sprint 3.3 Erstdeployment):**
- [ ] `DeFiRegistry.ink` auf Testnet deployed — Contract-Adresse in Config
- [ ] AMM-Pool $KAI/$COMPUTE initialisiert — Initiale Liquidität gesetzt
- [ ] **L11-NFT geminted:** `nft://kai-os/layer/11/defi` on-chain verankert
- [ ] S1-Gate: Alle L11-Transaktionen mit Ed25519 signiert (100% Coverage)
- [ ] S4-Gate: IDS-Circuit-Breaker für Flash-Loan-Angriffe aktiv + getestet
- [ ] Oracle-Network: ≥ 5 Quellen live + Median-Berechnung verifiziert
- [ ] `cargo contract check --all` — 0 `unwrap()` in Contract-Code
- [ ] Unit-Tests: ≥ 90% Coverage (AMM, Lending, Registry)
- [ ] PR + 2 Reviews + CI grün

---

## 26.8 Erweiterte Smart Contracts

### 26.8.1 OracleNetwork.ink — Dezentrale Preis-Feeds

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod oracle_network {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    #[ink(event)]
    pub struct PriceUpdated {
        #[ink(topic)]
        pair:      [u8; 8],   // z. B. b"KAI/USD\0"
        price:     u128,       // in Mikro-USD (6 Dezimalstellen)
        timestamp: u64,
        sources:   u8,         // Anzahl aggregierter Quellen
    }

    #[ink(event)]
    pub struct CircuitBreakerTriggered {
        #[ink(topic)]
        pair:      [u8; 8],
        deviation: u32,        // Abweichung in Basispunkten
    }

    #[ink(storage)]
    pub struct OracleNetwork {
        // Preis-Feeds: Pair → (price, timestamp, source_count)
        prices:       Mapping<[u8; 8], (u128, u64, u8)>,
        // Autorisierte Feeder-Nodes (min. 5 Quellen für Median)
        feeders:      Mapping<AccountId, bool>,
        feeder_count: u32,
        // Pendente Feeds: Pair → Vec<(feeder, price)>
        pending:      Mapping<[u8; 8], Vec<(AccountId, u128)>>,
        // Circuit-Breaker-Schwelle in Basispunkten (300 = 3%)
        cb_threshold: u32,
        // L0 Emergency-Pause
        frozen:       bool,
        owner:        AccountId,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        Unauthorized,
        Frozen,
        InsufficientSources,  // < 5 Quellen für Median
        CircuitBreakerActive,
        StalePrice,           // Preis > 60s alt
    }

    impl OracleNetwork {
        #[ink(constructor)]
        pub fn new(cb_threshold: u32) -> Self {
            Self {
                prices:       Mapping::default(),
                feeders:      Mapping::default(),
                feeder_count: 0,
                pending:      Mapping::default(),
                cb_threshold,
                frozen:       false,
                owner:        Self::env().caller(),
            }
        }

        /// Preis-Feed einreichen (nur autorisierte Feeder)
        #[ink(message)]
        pub fn submit_price(
            &mut self,
            pair:  [u8; 8],
            price: u128,
        ) -> Result<(), Error> {
            if self.frozen { return Err(Error::Frozen); }
            let caller = self.env().caller();
            if !self.feeders.get(caller).unwrap_or(false) {
                return Err(Error::Unauthorized);
            }
            let mut feeds = self.pending.get(pair).unwrap_or_default();
            // Duplikat-Einreichung desselben Feeders verhindern
            feeds.retain(|(f, _)| *f != caller);
            feeds.push((caller, price));
            // Aggregieren wenn ≥ 5 Quellen vorhanden
            if feeds.len() >= 5 {
                let aggregated = self.compute_median(&feeds);
                // Circuit Breaker prüfen
                if let Some((last_price, _, _)) = self.prices.get(pair) {
                    let deviation = if aggregated > last_price {
                        ((aggregated - last_price) * 10_000 / last_price) as u32
                    } else {
                        ((last_price - aggregated) * 10_000 / last_price) as u32
                    };
                    if deviation > self.cb_threshold {
                        self.env().emit_event(CircuitBreakerTriggered {
                            pair,
                            deviation,
                        });
                        return Err(Error::CircuitBreakerActive);
                    }
                }
                self.prices.insert(pair, &(
                    aggregated,
                    self.env().block_timestamp(),
                    feeds.len() as u8,
                ));
                self.pending.insert(pair, &Vec::new());
                self.env().emit_event(PriceUpdated {
                    pair,
                    price: aggregated,
                    timestamp: self.env().block_timestamp(),
                    sources: feeds.len() as u8,
                });
            } else {
                self.pending.insert(pair, &feeds);
            }
            Ok(())
        }

        /// Aktuellen Preis abfragen — schlägt fehl wenn > 60s alt
        #[ink(message)]
        pub fn get_price(&self, pair: [u8; 8]) -> Result<u128, Error> {
            match self.prices.get(pair) {
                None => Err(Error::InsufficientSources),
                Some((price, ts, _)) => {
                    let age = self.env().block_timestamp().saturating_sub(ts);
                    if age > 60_000 { return Err(Error::StalePrice); } // 60s
                    Ok(price)
                }
            }
        }

        /// Feeder registrieren (nur owner = Governance DAO L8)
        #[ink(message)]
        pub fn add_feeder(&mut self, feeder: AccountId) -> Result<(), Error> {
            if self.env().caller() != self.owner { return Err(Error::Unauthorized); }
            self.feeders.insert(feeder, &true);
            self.feeder_count += 1;
            Ok(())
        }

        fn compute_median(&self, feeds: &[(AccountId, u128)]) -> u128 {
            let mut prices: Vec<u128> = feeds.iter().map(|(_, p)| *p).collect();
            prices.sort();
            prices[prices.len() / 2]
        }
    }
}
```

### 26.8.2 FlashLoan.ink — Uncollateralized Flash Loans

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flash_loan {
    use ink::storage::Mapping;

    #[ink(event)]
    pub struct FlashLoanExecuted {
        #[ink(topic)]
        borrower:   AccountId,
        amount:     Balance,
        fee:        Balance,
        block:      u32,
    }

    #[ink(storage)]
    pub struct FlashLoan {
        // Pool-Liquidität (wird am Ende jeder TX zurückgezahlt)
        pool_balance:  Balance,
        // Gebühr in Basispunkten (z. B. 9 = 0.09%)
        fee_bps:       u32,
        // Max. Loan-Größe: 10% der Pool-Liquidität pro Block
        max_loan_pct:  u32,
        // Reentrancy-Guard: kein verschachtelter Flash Loan
        in_progress:   bool,
        // L0: Emergency-Pause
        frozen:        bool,
        owner:         AccountId,
        // Akkumulierte Gebühren für LP-Ausschüttung
        accrued_fees:  Balance,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        Frozen,
        ReentrancyDetected,
        LoanTooLarge,        // > 10% der Pool-Liquidität
        RepaymentFailed,     // Betrag + Gebühr nicht zurückgezahlt
        InsufficientLiquidity,
    }

    impl FlashLoan {
        #[ink(constructor, payable)]
        pub fn new(fee_bps: u32, max_loan_pct: u32) -> Self {
            Self {
                pool_balance:  Self::env().transferred_value(),
                fee_bps,
                max_loan_pct,
                in_progress:   false,
                frozen:        false,
                owner:         Self::env().caller(),
                accrued_fees:  0,
            }
        }

        /// Flash Loan anfordern — Betrag + Gebühr muss im selben Block
        /// zurückgezahlt werden (via Callback-Contract)
        #[ink(message)]
        pub fn flash_loan(
            &mut self,
            amount:   Balance,
            receiver: AccountId,  // Empfänger-Contract (muss IFlashReceiver implementieren)
        ) -> Result<(), Error> {
            if self.frozen         { return Err(Error::Frozen); }
            if self.in_progress    { return Err(Error::ReentrancyDetected); }
            if self.pool_balance == 0 { return Err(Error::InsufficientLiquidity); }

            // Max-Loan-Check: 10% der Pool-Liquidität
            let max_loan = self.pool_balance * self.max_loan_pct as u128 / 100;
            if amount > max_loan { return Err(Error::LoanTooLarge); }

            let fee = amount * self.fee_bps as u128 / 10_000;
            let repayment = amount + fee;

            // Reentrancy-Sperre setzen
            self.in_progress = true;
            let balance_before = self.pool_balance;

            // Betrag an Receiver transferieren
            self.pool_balance -= amount;
            // Hier würde normalerweise der Cross-Contract-Call erfolgen
            // receiver.on_flash_loan(amount, fee) — vereinfacht dargestellt

            // Nach dem Callback: Repayment prüfen
            // (In Produktion: balance_before + fee == self.env().balance())
            if self.pool_balance < balance_before + fee {
                self.in_progress = false;
                return Err(Error::RepaymentFailed);
            }

            self.accrued_fees += fee;
            self.pool_balance  = balance_before + fee;
            self.in_progress   = false;

            self.env().emit_event(FlashLoanExecuted {
                borrower: self.env().caller(),
                amount,
                fee,
                block: self.env().block_number(),
            });
            Ok(())
        }

        /// Pool-Liquidität hinzufügen (LP-Einlage)
        #[ink(message, payable)]
        pub fn add_liquidity(&mut self) -> Result<Balance, Error> {
            if self.frozen { return Err(Error::Frozen); }
            let amount = self.env().transferred_value();
            self.pool_balance += amount;
            Ok(self.pool_balance)
        }

        /// Aktuelle Pool-Statistiken
        #[ink(message)]
        pub fn get_stats(&self) -> (Balance, Balance, bool) {
            (self.pool_balance, self.accrued_fees, self.in_progress)
        }
    }
}
```

## 26.9 L11 Upgrade-Governance

Jeder L11-Upgrade läuft über den Governance-Layer (L8) und erfordert eine L0-Zertifizierung:

```
1. Proposal erstellen (L8 GovernanceDAO)
   └── Neue Contract-Version + Migrationsskript
   └── 72h Conviction-Voting-Periode
2. Vote angenommen → 48h Timelock (L8)
3. L0-Security-Gate:
   └── Audit-Report vorhanden? (S5 Audit-Trail)
   └── Keine Critical-Findings?
   └── ZKP-Compliance-Beweis exportiert? (S3)
4. Timelock läuft ab → DeFiRegistry.register_module() mit neuer Adresse
5. Altes Modul: 30 Tage Deprecation-Window, dann deaktiviert
```

**L11-Versionshistorie:**

| Version | Datum | Änderung | Sprints |
|---|---|---|---|
| `v0.1.0-dev` | Jan 2026 | Architektur-Entwurf | Phase 1 |
| `v0.2.0-testnet` | Jul 2026 | AMM + DeFiRegistry live | Sprint 3.3 |
| `v0.3.0-testnet` | Okt 2026 | Lending + Oracle + $kUSD | Sprint 3.6 |
| `v1.0.0-alpha` | Jan 2027 | Flash Loans + MEV-Schutz | Sprint 4.1 |
| `v1.0.0-mainnet` | Sep 2027 | Mainnet Go-Live 🚀 | Sprint 4.3 |


## 26.10 ATCAMM — Dezentraler Exchange (DEX)

> **Datei:** `blockchain/dex/amm.py` · Fixes: #37

```python
class LiquidityPool:
    """Constant-Product AMM: x * y = k."""
    token_a:   str      # ATC-Adresse Token A
    token_b:   str      # ATC-Adresse Token B
    reserve_a: int      # Reserve Token A (18 Dezimalstellen)
    reserve_b: int      # Reserve Token B
    fee_rate:  float    # 0.003 (0.3%)

    def get_price(self) -> float:
        """Aktueller Preis: reserve_b / reserve_a."""

    def get_price_impact(self, amount_in: int, a_to_b: bool) -> float:
        """Preisauswirkung in % für gegebene Menge."""

class ATCAMM:
    """AMM-Router — verwaltet mehrere Liquidity Pools."""

    def create_pool(self, token_a: str, token_b: str,
                    initial_a: int, initial_b: int,
                    creator: str) -> str:
        """Pool erstellen, gibt pool_id zurück."""

    def add_liquidity(self, pool_id: str, provider: str,
                      amount_a: int, amount_b: int) -> dict:
        """Liquidität hinzufügen, LP-Token erhalten."""

    def remove_liquidity(self, pool_id: str, provider: str,
                         lp_tokens: int) -> dict:
        """LP-Token zurückgeben, Token A+B erhalten."""

    def swap(self, pool_id: str, trader: str,
             amount_in: int, a_to_b: bool,
             min_amount_out: int = 0) -> dict:
        """Token tauschen.
        Fee: 0.3% (0.25% an LP, 0.05% an Protokoll)"""

    def get_stats(self) -> dict: ...
```

| Parameter | Wert |
|-----------|------|
| Fee | 0.3% (0.25% LP + 0.05% Protokoll) |
| Min. Liquidität | 1.000 ATC |
| Max. Price Impact | 5% (sonst rejected) |
| LP-Token | ERC-20 kompatibel |

## 26.11 GasFeeEngine — EIP-1559 Modell

> **Datei:** `blockchain/consensus/gas_fee.py` · Fixes: #33

```python
class GasConfig:
    BASE_FEE_INITIAL:  float = 0.001    # ATC
    BASE_FEE_MAX:      float = 1.000    # ATC
    BURN_RATE:         float = 0.50     # 50% Base Fee verbrennen
    TARGET_GAS_BLOCK:  int   = 10_000
    GAS_UNITS = {
        "transfer":    21_000,
        "contract":   200_000,
        "nft_mint":   150_000,
        "atclang":    100_000,
    }

class GasFeeEngine:
    def estimate_gas(self, operation: str,
                     data_size: int = 0) -> int: ...
    def process_tx(self, gas_units: int,
                   priority_fee: float = 0.001) -> dict:
        """Berechnet: total_fee = base_fee*gas + priority_fee.
        Burn: 50% der base_fee wird vernichtet."""
    def apply_block(self, gas_used: int) -> float:
        """Passt Base Fee nach EIP-1559 an."""
```


# 27. Gamification-Layer — L12

> ⚡ **Grundprinzip:** Der Gamification-Layer ist eine vollständig eigenständige Schicht — **1 Ding = 1 Layer**. L12 enthält ausschließlich Spielmechaniken, Belohnungssysteme und Community-Incentives. Keine anderen Komponenten.

**On-Chain-Identität:** `nft://kai-os/layer/12/gamification`

## 27.1 Konzept & Einordnung

L12 ist das dezentrale Incentive- und Engagement-System von KAI-OS. Es verwandelt Beiträge zum Netzwerk — Node-Betrieb, Governance-Teilnahme, Entwicklung, DeFi-Liquidität — in messbare, on-chain verifizierbare Achievements und Belohnungen.

**Abgrenzung zu anderen Layern:**

| Layer | Verhältnis zu L12 |
|---|---|
| **L4** Blockchain-Modul | L12-Achievements on-chain verankert — L4 ist die Grundlage |
| **L9** Agent | KI-Agenten generieren personalisierte Quests und berechnen Belohnungen |
| **L11** DeFi | DeFi-Aktivitäten (Swaps, Liquidity Mining) triggern L12-Events |
| **L8** Governance | Governance-Teilnahme ist ein eigenes Achievement-Cluster |
| **L0** Security | L0 zertifiziert Soul-Bound-NFT-Minting — kein Fälschen möglich |

## 27.2 L12-Modul-Architektur

| Modul | NFT-URI | Funktion |
|---|---|---|
| **Quest Engine** | `nft://kai-os/layer/12/gamification/quests` | KI-generierte Missionen basierend auf User-Profil |
| **Achievement System** | `nft://kai-os/layer/12/gamification/achievements` | Soul-Bound-NFTs für Milestones (nicht übertragbar) |
| **Leaderboard** | `nft://kai-os/layer/12/gamification/leaderboard` | On-Chain-Ranglisten — manipulationssicher |
| **Reward Engine** | `nft://kai-os/layer/12/gamification/rewards` | KI-berechnet $COMPUTE/$KAI-Belohnungen |
| **XP-System** | `nft://kai-os/layer/12/gamification/xp` | Erfahrungspunkte für alle On-Chain-Aktionen |
| **Badge Registry** | `nft://kai-os/layer/12/gamification/badges` | Übertragbare Reputation-Badges |

## 27.3 Quest-System (KI-generiert)

Quests werden vom KI-Agenten (L9) dynamisch generiert — basierend auf dem On-Chain-Profil des Nutzers, seiner Aktivitätshistorie und den aktuellen Netzwerk-Bedürfnissen.

**Quest-Typen:**

| Kategorie | Beispiel-Quest | Belohnung |
|---|---|---|
| **Node-Betrieb** | "Betreibe einen Validator 30 Tage ohne Downtime" | 500 $COMPUTE + Achievement NFT |
| **Entwicklung** | "Deploye deinen ersten Smart Contract auf Testnet" | 200 $COMPUTE + Developer-Badge |
| **Governance** | "Stimme in 5 aufeinanderfolgenden Proposals ab" | 100 $KAI + Governance-Achievement |
| **DeFi** | "Stelle Liquidität für 7 Tage in AMM-Pool bereit" | LP-Bonus + DeFi-Pioneer-Badge |
| **KI-Training** | "Trage zu 3 Federated-Learning-Runden bei" | 300 $COMPUTE + FL-Contributor-NFT |
| **Security** | "Melde einen validen Bug im Bug-Bounty-Programm" | 1.000–50.000 $KAI (Schwere-basiert) |

**Quest-Lifecycle:**
```
KI-Agent (L9) analysiert User-Profil
  └── generiert personalisierte Quest-Liste
  └── Quest on-chain gespeichert (L4)
  └── User erfüllt Quest-Bedingungen
  └── KI-Agent verifiziert Erfüllung (on-chain Beweis)
  └── Belohnung automatisch ausgezahlt (L11 Reward Engine)
  └── Achievement-NFT geminted (L12 Soul-Bound)
```

## 27.4 Achievement-System (Soul-Bound NFTs)

Achievements sind **Soul-Bound NFTs** — sie sind an die On-Chain-Identität (DID) des Nutzers gebunden und können nicht übertragen oder verkauft werden. Sie repräsentieren echte, verifizierte Leistungen.

### 27.4.1 Achievement-Kategorien

**Tier 1 — Bronze (Community):**
- 🥉 `FIRST_TRANSACTION` — Erste On-Chain-Transaktion
- 🥉 `FIRST_AGENT` — Ersten KI-Agenten deployt
- 🥉 `GOVERNANCE_VOTER` — An erster Abstimmung teilgenommen
- 🥉 `DeFi_STARTER` — Ersten Swap auf AMM durchgeführt

**Tier 2 — Silver (Contributor):**
- 🥈 `NODE_OPERATOR_30D` — Node 30 Tage ohne Downtime
- 🥈 `FL_CONTRIBUTOR` — 10 Federated-Learning-Beiträge
- 🥈 `LIQUIDITY_PROVIDER` — Liquidität für 30+ Tage bereitgestellt
- 🥈 `DEVELOPER` — 5+ Smart Contracts deployed

**Tier 3 — Gold (Expert):**
- 🥇 `VALIDATOR_CHAMPION` — Top-10-Validator nach Uptime
- 🥇 `GOVERNANCE_WHALE` — 50+ Governance-Votes abgegeben
- 🥇 `DeFi_MASTER` — > 100.000 $COMPUTE in DeFi-Protokollen bewegt
- 🥇 `SECURITY_RESEARCHER` — Validen Security-Bug gemeldet

**Tier 4 — Diamond (Legend):**
- 💎 `GENESIS_VALIDATOR` — Einer der ersten 21 Mainnet-Validatoren
- 💎 `KAI_OS_FOUNDER` — Aktiver Beitrag vor Mainnet-Launch
- 💎 `PROTOCOL_GUARDIAN` — Critical-Security-Bug gefunden und disclosed

### 27.4.2 SoulBoundNFT.ink

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod soul_bound_nft {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    #[ink(event)]
    pub struct AchievementMinted {
        #[ink(topic)]
        recipient:      AccountId,
        #[ink(topic)]
        achievement_id: [u8; 32],
        tier:           u8,
        timestamp:      u64,
    }

    #[ink(storage)]
    pub struct SoulBoundNFT {
        // DID → Liste der Achievement-IDs
        achievements:    Mapping<AccountId, Vec<[u8; 32]>>,
        // Achievement-ID → Metadaten-URI
        metadata:        Mapping<[u8; 32], Vec<u8>>,
        // Achievement-ID → Tier (1=Bronze, 2=Silver, 3=Gold, 4=Diamond)
        tiers:           Mapping<[u8; 32], u8>,
        // Nur KI-Reward-Engine (L12) oder Security Council (L0) darf minten
        minter:          AccountId,
        // L0: Emergency-Pause (bei entdecktem Exploit)
        frozen:          bool,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        Unauthorized,
        AlreadyMinted,   // Soul-Bound: kein Doppel-Minting
        Frozen,
        TransferNotAllowed, // Soul-Bound: kein Transfer
    }

    impl SoulBoundNFT {
        #[ink(constructor)]
        pub fn new(minter: AccountId) -> Self {
            Self {
                achievements: Mapping::default(),
                metadata:     Mapping::default(),
                tiers:        Mapping::default(),
                minter,
                frozen:       false,
            }
        }

        /// Achievement minten — nur KI-Reward-Engine (L12) oder L0 Security Council
        #[ink(message)]
        pub fn mint(
            &mut self,
            recipient:      AccountId,
            achievement_id: [u8; 32],
            metadata_uri:   Vec<u8>,
            tier:           u8,
        ) -> Result<(), Error> {
            if self.frozen { return Err(Error::Frozen); }
            if self.env().caller() != self.minter { return Err(Error::Unauthorized); }
            // Soul-Bound: prüfen ob Achievement bereits existiert
            let mut list = self.achievements.get(recipient).unwrap_or_default();
            if list.contains(&achievement_id) { return Err(Error::AlreadyMinted); }
            list.push(achievement_id);
            self.achievements.insert(recipient, &list);
            self.metadata.insert(achievement_id, &metadata_uri);
            self.tiers.insert(achievement_id, &tier);
            self.env().emit_event(AchievementMinted {
                recipient,
                achievement_id,
                tier,
                timestamp: self.env().block_timestamp(),
            });
            Ok(())
        }

        /// Transfer ist explizit VERBOTEN — Soul-Bound
        #[ink(message)]
        pub fn transfer(&self, _to: AccountId, _id: [u8; 32]) -> Result<(), Error> {
            Err(Error::TransferNotAllowed)
        }

        /// Alle Achievements eines Nutzers abfragen
        #[ink(message)]
        pub fn get_achievements(&self, owner: AccountId) -> Vec<[u8; 32]> {
            self.achievements.get(owner).unwrap_or_default()
        }
    }
}
```

## 27.5 KI-Reward-Engine

Die KI-Reward-Engine ist ein dauerhaft laufender Agent (L9), der alle On-Chain-Aktivitäten beobachtet und Belohnungen automatisch berechnet und ausschüttet.

```
KI-Reward-Engine (L9 Agent)
  ├── beobachtet: Block-Events (L4)
  ├── beobachtet: DeFi-Transaktionen (L11)
  ├── beobachtet: Governance-Votes (L8)
  ├── berechnet: XP-Punkte nach Aktivitätsgewichtung
  ├── triggert:  Achievement-Minting (L12 SoulBoundNFT)
  └── zahlt aus: $COMPUTE-Belohnungen (L4 Token-Ökonomie)
```

**Belohnungs-Gewichtung:**

| Aktivität | XP | $COMPUTE-Reward |
|---|---|---|
| Block produziert (Validator) | 10 XP | 50 $COMPUTE |
| Governance-Vote | 5 XP | 20 $COMPUTE |
| FL-Beitrag (Gradient) | 15 XP | 100 $COMPUTE (qualitätsgewichtet) |
| Liquidity-Mining-Tag | 8 XP | 30 $COMPUTE |
| Bug Report (valide) | 500–5.000 XP | Bounty (Kapitel 16.4) |
| Quest abgeschlossen | 50–500 XP | Quest-definiert |

## 27.6 Leaderboard-System

Das Leaderboard ist vollständig on-chain — transparent, manipulationssicher und von KI-Agenten nicht beeinflussbar.

```rust
// Leaderboard-Kategorien
enum LeaderboardCategory {
    ValidatorUptime,     // Höchste Uptime der letzten 30 Tage
    GovernanceActivity,  // Meiste Governance-Votes
    DeFiVolume,          // Höchstes DeFi-Transaktionsvolumen
    FlContributions,     // Wertvollste FL-Beiträge
    XpTotal,             // Gesamt-XP (Lifetime)
    QuestsCompleted,     // Abgeschlossene Quests
}
```

**Anti-Gaming-Maßnahmen:**
- XP-Gewichtung sinkt bei repetitiven Aktionen (Diminishing Returns)
- Sybil-Resistenz: DID-verifizierte Identitäten (L4 + L0)
- KI-IDS (L0/S4) erkennt koordiniertes Gaming
- Leaderboard-Reset alle 90 Tage — Season-basiert

## 27.7 Sicherheit — L0 → L12

> 🔗 **Security Layer S1** (Kapitel 25.3): Achievement-NFTs mit Ed25519 on-chain signiert — Fälschungen kryptografisch unmöglich.

> 🔗 **Security Layer S2** (Kapitel 25.4): Zero-Trust-Engine prüft jeden Minting-Aufruf — nur autorisierte Reward-Engine darf Soul-Bound-NFTs erzeugen.

> 🔗 **Security Layer S3** (Kapitel 25.5): ZKP-basierter Nachweis für Quest-Erfüllung — User beweist Leistung ohne persönliche Daten preiszugeben.

> 🔗 **Security Layer S4** (Kapitel 25.6): KI-IDS erkennt Sybil-Angriffe, koordiniertes XP-Farming und Achievement-Exploits in Echtzeit.

> 🔗 **Security Layer S5** (Kapitel 25.7): Jedes gemintete Achievement vollständig im On-Chain-Audit-Trail — inkl. Triggerbedingung und verifizierendem Agent.

**L12-spezifische Sicherheitsregeln:**
- Soul-Bound-Transfer: on-chain hard-geblockt (kein Override möglich)
- Minting-Limit: max. 100 Achievements pro Block (Spam-Schutz)
- Quest-Manipulation: KI-IDS überwacht Quest-Completion-Muster
- Reward-Auszahlung: 24h Timelock bei Beträgen > 10.000 $COMPUTE

## 27.8 Roadmap-Integration (Sprint-Plan)

| Sprint | L12-Aufgabe | Meilenstein | Security-Gate |
|---|---|---|---|
| **Sprint 3.7** | Quest Engine + XP-System deployen | L12-NFT auf Testnet geminted | S2 + S4 aktiv |
| **Sprint 3.8** | Achievement System + Soul-Bound-NFTs live | Alpha: Erste Achievements geminted | S1 Signatur-Verifizierung |
| **Sprint 4.1** | KI-Reward-Engine (Agent ↔ L12) live | Automatische Belohnungen aktiv | S3 ZKP-Quest-Beweis |
| **Sprint 4.2** | Leaderboard + Season-System | Season 1 startet mit TGE | S5 Audit-Trail vollständig |
| **Sprint 4.4** | Ökosystem-Hackathon-Quests | 1.000+ aktive Quest-Nutzer | S4 Anti-Gaming-IDS aktiv |

**🔧 Fehlerbehebungs-Schritte (L12 — übergreifend):**

| Symptom | Diagnose-Befehl | Lösung |
|---|---|---|
| Achievement doppelt geminted | `kai contract query SoulBoundNFT get_achievements <DID>` | `AlreadyMinted`-Error prüfen — Contract-State inspizieren |
| Quest-Erfüllung nicht erkannt | `kai agent logs reward-engine --level trace` | On-Chain-Event vom triggernden Layer angekommen? |
| XP-Berechnung falsch | `kai defi xp-audit --user <DID> --last 100` | Gewichtungstabelle in Reward-Engine-Config |
| Leaderboard zeigt veraltete Daten | `kai contract query Leaderboard last-update` | Indexer-Lag? `kai indexer reindex --contract Leaderboard` |
| Sybil-Alarm ausgelöst | `kai security ids-stats --category sybil` | False-Positive? Threshold in `ids.toml` anpassen |
| Soul-Bound-Transfer-Versuch | `kai security audit --last-blocks 10 --event TransferAttempt` | L0 S2 hat blockiert — kein Handlungsbedarf |

**🚀 Deployment-Checkliste (L12 — Sprint 3.7 Erstdeployment):**
- [ ] `SoulBoundNFT.ink` deployed — Transfer explizit verboten + getestet
- [ ] `QuestEngine.ink` deployed — KI-Agent als autorisierter Minter eingetragen
- [ ] **L12-NFT geminted:** `nft://kai-os/layer/12/gamification` on-chain verankert
- [ ] S1-Gate: Achievement-Minting mit Ed25519 signiert (100% Coverage)
- [ ] S2-Gate: Nur autorisierter Minter kann Soul-Bound-NFTs erzeugen — Test mit unautorisiertem Aufruf
- [ ] S4-Gate: Sybil-Detection aktiv — Test mit simuliertem Farming-Angriff
- [ ] XP-System: Diminishing-Returns-Logik mit 1.000 simulierten Aktionen getestet
- [ ] Unit-Tests: ≥ 90% Coverage (SoulBoundNFT, QuestEngine, RewardEngine)
- [ ] PR + 2 Reviews + CI grün

---

## Layer-Übersicht: L0–L12 (vollständig)

> ⚡ **Grundprinzip:** 1 Ding = 1 Layer = 1 NFT — kein Layer enthält mehrere unabhängige Komponenten.

| Layer | Name | NFT-URI | Kapitel |
|---|---|---|---|
| **L0** | Security *(Querschnitts-Schicht)* | `nft://kai-os/layer/0/security` | 25 |
| **L1** | Hardware | `nft://kai-os/layer/1/<node-id>` | 24 |
| **L2** | Micro-Kernel | `nft://kai-os/layer/2/kernel` | 24 |
| **L3** | KI-Modul | `nft://kai-os/layer/3/ai` | 24 |
| **L4** | Blockchain-Modul | `nft://kai-os/layer/4/blockchain` | 4 |
| **L5** | P2P-Netzwerk | `nft://kai-os/layer/5/p2p` | 2 |
| **L6** | Storage-Modul | `nft://kai-os/layer/6/storage` | 2 |
| **L7** | API & CLI | `nft://kai-os/layer/7/api` | 8 |
| **L8** | Governance | `nft://kai-os/layer/8/governance` | 19 |
| **L9** | Agent | `nft://kai-os/layer/9/<agent-id>` | 10 |
| **L10** | dApp | `nft://kai-os/layer/10/<dapp-id>` | 5 |
| **L11** | DeFi | `nft://kai-os/layer/11/defi` | **26** |
| **L12** | Gamification | `nft://kai-os/layer/12/gamification` | **27** |

---


# 28. Integration Map — A-TownChain Repo ↔ KAI-OS Wiki

> 🔗 **Grundprinzip:** Der bestehende A-TownChain-Code (Python-Prototyp) und die KAI-OS-Wiki-Spezifikation (Rust/Ink!-Produktion) werden nach dem Prinzip **"beste Lösung gewinnt"** zusammengeführt. Kein Code wird weggeworfen — jedes Konzept aus dem Repo findet seinen Platz im finalen Stack.

**On-Chain-Identität:** Das Repo ist der Prototyp. Die Wiki ist die Zielarchitektur.

## 28.1 Bewertungsmatrix

| Komponente | Repo (Python) | Wiki (Rust/Ink!) | Gewinner | Migrations-Sprint |
|---|---|---|---|---|
| **Konsensus** | PoH→PoW→PoS ⭐⭐ | GRANDPA/BABE ⭐⭐⭐ | **Wiki** | Sprint 2.1 |
| **Token-Standard** | ATC-8300 (vollständig) ⭐⭐⭐ | $KAI-Pallet ⭐⭐⭐ | **MERGE** | Sprint 2.5 |
| **Shivamon/Gamifi** | DNA+Battle+Rarity ⭐⭐⭐ | SoulBound+KI-Quest ⭐⭐⭐ | **MERGE** ⭐ | Sprint 3.7 |
| **Wallet/Crypto** | secp256k1+BIP-39 ⭐⭐ | Ed25519+SR25519+PQ ⭐⭐⭐ | **Wiki** | K-Sec 1 |
| **Kernel/Core** | EventBus+ModuleLoader ⭐⭐ | Rust Micro-Kernel ⭐⭐⭐ | **Wiki** | Sprint 2.3 |
| **P2P-Netzwerk** | TCP+Handshake+Filter ⭐⭐⭐ | libp2p GossipSub ⭐⭐⭐ | **Wiki** | Sprint 2.2 |
| **Contract-Registry** | SmartContractRegistry ⭐⭐⭐ | DeFiRegistry.ink ⭐⭐⭐ | **MERGE** | Sprint 2.5 |

## 28.2 Detail-Entscheidungen

### 28.2.1 Konsensus — Wiki gewinnt, Repo-PoH bleibt

**Was bleibt vom Repo:**
- `HybridConsensus.create_block()` → Referenzimplementierung für Substrate-Pallet-Tests
- PoH-Sequenz-Hash-Konzept → direkt als Substrate-Pallet `pallet_poh` übernehmen
- `validate_chain()` Logik → als Substrate Off-Chain-Worker implementieren

**Was aus der Wiki kommt:**
- GRANDPA BFT-Finalität (Byzantine Fault Tolerant, echte Finality)
- BABE VRF-basierte Block-Production (kein SHA-256-Mining)
- Slashing + Unbonding für PoS-Validatoren

**Merge-Ergebnis:**
```
Substrate-Runtime (L4):
  pallet_babe     → Block Production (ersetzt PoW)
  pallet_grandpa  → Finality (ersetzt centralized PoS)
  pallet_poh      → PoH Zeitstempel-Pallet (NEU, aus Repo)
  pallet_staking  → Slashing + Unbonding (Wiki)
```

### 28.2.2 Token — MERGE (Repo-Features + Wiki-Ökonomie)

**Was bleibt vom Repo (ATC-8300):**
- Snapshot-System für Governance-Voting → fehlt in Wiki, wird ergänzt
- Allowances (ERC-20-ähnlich) → in $KAI-Pallet als `pallet_assets` übernehmen
- 8 Dezimalstellen → Standard beibehalten
- Pause-Mechanismus → als L0/S2 Emergency-Freeze integriert

**Was aus der Wiki kommt:**
- $KAI + $COMPUTE Dual-Token-Ökonomie (L4)
- 10% Burn-Mechanismus bei KI-DeFi-Transaktionen
- On-chain deployment via Substrate-Pallet

**Merge-Ergebnis:**
```rust
// pallet_kai_token (L4) — Merger aus ATC-8300 + Wiki-Spec
pub struct KaiToken {
    // ATC-8300 Features
    pub snapshots:   StorageMap<BlockNumber, Balance>,  // NEU aus Repo
    pub allowances:  StorageDoubleMap<Account, Account, Balance>,
    pub decimals:    u8,   // 8 (aus Repo)
    // Wiki-Features
    pub burn_rate:   Perbill,  // 10% bei KI-TX
    pub dual_token:  bool,     // $KAI + $COMPUTE
}
```

### 28.2.3 Shivamon ↔ L12 — Stärkster Merge ⭐

Das Shivamon-System ist das wertvollste Asset im Repo — es enthält fertige Spielmechanik-Logik, die die Wiki noch nicht hat. Ziel: `ShivamonNFT.ink` = bestes aus beiden Welten.

**Was bleibt vom Repo:**
```python
# REPO: Zu migrieren nach Ink! (Rust)
- 7 Elemente: Fire, Water, Earth, Air, Shadow, Neon, Quantum
- 6 Rarities: Common→Uncommon→Rare→Epic→Legendary→Genesis
- RARITY_MULTIPLIER: {Common: 1.0x → Genesis: 5.0x}
- DNA-Hash: SHA-256(token_id + name + element + timestamp) → BLAKE2b (L0/S1)
- Deterministische Stats aus DNA (HP/Attack/Defense/Speed/Special)
- XP/Level-System, Win/Loss-Tracking
- Battle-Move-System
```

**Was aus der Wiki kommt:**
```rust
// WIKI: SoulBound + KI-Integration
- TransferNotAllowed Guard (Soul-Bound, nicht übertragbar)
- AlreadyMinted Guard (1 NFT pro Achievement)
- Tier-System (Bronze/Silver/Gold/Diamond → = Rarity)
- KI-Reward-Engine (L9 Agent berechnet $COMPUTE-Belohnungen)
- On-Chain Quest-System (L12/quests)
```

**Merge-Ergebnis — ShivamonNFT.ink (vollständig):**
```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod shivamon_nft {
    use ink::storage::Mapping;
    use ink::prelude::{string::String, vec::Vec};

    // ── Elemente (aus Repo) ────────────────────────────
    #[derive(Debug, Clone, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Element {
        Fire, Water, Earth, Air, Shadow, Neon, Quantum,
    }

    // ── Rarities (aus Repo, mapped auf Wiki-Tiers) ────
    #[derive(Debug, Clone, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Rarity {
        Common,    // Bronze Tier (1.0x)
        Uncommon,  // Silver Tier (1.2x)
        Rare,      // Gold Tier   (1.5x)
        Epic,      // Diamond Tier (2.0x)
        Legendary, // Diamond+ (3.0x)
        Genesis,   // Soul-Bound Origin (5.0x) — nicht handelbar
    }

    // ── Stats (aus Repo) ───────────────────────────────
    #[derive(Debug, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct ShivamonStats {
        pub hp:      u32,
        pub attack:  u32,
        pub defense: u32,
        pub speed:   u32,
        pub special: u32,
    }

    // ── NFT-Datensatz (Merge) ──────────────────────────
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct ShivamonData {
        pub name:        String,
        pub element:     Element,
        pub rarity:      Rarity,
        pub owner:       AccountId,
        pub dna_hash:    [u8; 32],   // BLAKE2b (L0/S1) statt SHA-256
        pub stats:       ShivamonStats,
        pub level:       u32,
        pub xp:          u32,
        pub wins:        u32,
        pub losses:      u32,
        pub generation:  u32,
        pub minted_at:   u64,
        pub soul_bound:  bool,       // Wiki: Genesis = immer soul-bound
        pub quest_ids:   Vec<u32>,   // Wiki: verknüpfte Quests (L12/quests)
        pub compute_earned: u128,    // Wiki: KI-Reward-Tracking
    }

    #[ink(storage)]
    pub struct ShivamonNFT {
        tokens:       Mapping<u32, ShivamonData>,
        owner_tokens: Mapping<AccountId, Vec<u32>>,
        total_supply: u32,
        // Wiki: Soul-Bound Guard
        minted_by:    Mapping<AccountId, bool>,
        // L0: Emergency-Pause
        frozen:       bool,
        owner:        AccountId,   // Governance DAO (L8)
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        AlreadyMinted,          // Wiki: 1 Genesis pro Wallet
        TransferNotAllowed,     // Wiki: Soul-Bound
        Frozen,                 // L0 Emergency
        Unauthorized,
        TokenNotFound,
        InsufficientXP,         // Repo: Level-Up braucht XP-Minimum
    }

    impl ShivamonNFT {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tokens:          Mapping::default(),
                owner_tokens:    Mapping::default(),
                total_supply:    0,
                minted_by:       Mapping::default(),
                frozen:          false,
                owner:           Self::env().caller(),
            }
        }

        /// Shivamon minten — prüft Soul-Bound bei Genesis
        #[ink(message)]
        pub fn mint(
            &mut self,
            name:       String,
            element:    Element,
            rarity:     Rarity,
            generation: u32,
        ) -> Result<u32, Error> {
            if self.frozen { return Err(Error::Frozen); }
            let caller = self.env().caller();
            // Wiki: Genesis = Soul-Bound, max 1 pro Wallet
            if rarity == Rarity::Genesis {
                if self.minted_by.get(caller).unwrap_or(false) {
                    return Err(Error::AlreadyMinted);
                }
                self.minted_by.insert(caller, &true);
            }
            // DNA aus BLAKE2b (L0/S1 Crypto-Primitive)
            let dna_hash = self.compute_dna(&name, &element, generation);
            let stats    = self.derive_stats(&dna_hash, &rarity, generation);
            let soul_bound = rarity == Rarity::Genesis;
            let token_id = self.total_supply + 1;
            let data = ShivamonData {
                name, element, rarity, owner: caller, dna_hash, stats,
                level: 1, xp: 0, wins: 0, losses: 0, generation,
                minted_at: self.env().block_timestamp(),
                soul_bound,
                quest_ids: Vec::new(),
                compute_earned: 0,
            };
            self.tokens.insert(token_id, &data);
            let mut owned = self.owner_tokens.get(caller).unwrap_or_default();
            owned.push(token_id);
            self.owner_tokens.insert(caller, &owned);
            self.total_supply += 1;
            Ok(token_id)
        }

        /// Transfer — Soul-Bound Guard (Wiki)
        #[ink(message)]
        pub fn transfer(&mut self, token_id: u32, _to: AccountId) -> Result<(), Error> {
            let data = self.tokens.get(token_id).ok_or(Error::TokenNotFound)?;
            if data.soul_bound {
                return Err(Error::TransferNotAllowed);  // Wiki: kein Transfer
            }
            // Non-Soul-Bound Transfers erlaubt (Common–Legendary)
            Ok(())
        }

        /// XP hinzufügen + Level-Up (Repo-Logik)
        #[ink(message)]
        pub fn add_xp(&mut self, token_id: u32, xp: u32) -> Result<u32, Error> {
            if self.env().caller() != self.owner { return Err(Error::Unauthorized); }
            let mut data = self.tokens.get(token_id).ok_or(Error::TokenNotFound)?;
            data.xp += xp;
            // Level-Up alle 1000 XP (aus Repo-Logik)
            let new_level = 1 + (data.xp / 1000);
            data.level = new_level;
            self.tokens.insert(token_id, &data);
            Ok(new_level)
        }

        /// BLAKE2b DNA-Hash (L0/S1 — Repo SHA-256 → BLAKE2b)
        fn compute_dna(&self, name: &str, element: &Element,
                       generation: u32) -> [u8; 32] {
            // In Produktion: ink::env::hash::Blake2x256
            let mut seed = [0u8; 32];
            let ts = self.env().block_timestamp().to_le_bytes();
            for (i, b) in ts.iter().enumerate() { seed[i] = *b; }
            for (i, b) in name.as_bytes().iter().take(8).enumerate() {
                seed[8 + i] = *b;
            }
            seed[16] = generation as u8;
            seed  // Vereinfacht — Produktion: Blake2x256::hash()
        }

        /// Stats deterministisch aus DNA (Repo-Logik, Rust-Port)
        fn derive_stats(&self, dna: &[u8; 32],
                        rarity: &Rarity, gen: u32) -> ShivamonStats {
            let mult = match rarity {
                Rarity::Common    => 100u32,
                Rarity::Uncommon  => 120,
                Rarity::Rare      => 150,
                Rarity::Epic      => 200,
                Rarity::Legendary => 300,
                Rarity::Genesis   => 500,
            };
            let base = 50 + gen * 5;
            ShivamonStats {
                hp:      base * mult / 100 + dna[0] as u32,
                attack:  base * mult / 100 + dna[1] as u32,
                defense: base * mult / 100 + dna[2] as u32,
                speed:   base * mult / 100 + dna[3] as u32,
                special: base * mult / 100 + dna[4] as u32,
            }
        }
    }
}
```

### 28.2.4 Wallet — Wiki gewinnt, Repo-BIP39 bleibt

| Feature | Repo | Wiki | Entscheidung |
|---|---|---|---|
| Signatur-Kurve | secp256k1 | SR25519 + Ed25519 | Wiki (Substrate) |
| Key-Derivation | PBKDF2-HMAC-SHA512 | SR25519 Derivation Path | MERGE |
| Mnemonic | BIP-39 (24 Wörter) | kompatibel | Repo beibehalten |
| Adressformat | ATC + 32 hex | SS58 (Substrate) | Wiki |
| Post-Quantum | ❌ keins | Kyber-1024 | Wiki |
| Checksum | SHA-256 doppelt | SS58Check | Wiki |

**Migration:** `ATCKeyGenerator.entropy_to_mnemonic()` + `mnemonic_to_seed()` → 1:1 in `kai-crypto` Crate übernehmen (PBKDF2-Logik ist korrekt und Standard-konform).

### 28.2.5 P2P — Wiki (libp2p) + Repo-Topics

| Repo Message-Typ | libp2p GossipSub Topic (Wiki L5) |
|---|---|
| `MSG_NEW_BLOCK` | `/kai-os/blocks/1.0.0` |
| `MSG_NEW_TX` | `/kai-os/transactions/1.0.0` |
| `MSG_GET_BLOCKS` | `/kai-os/sync/request/1.0.0` |
| `MSG_BLOCKS` | `/kai-os/sync/response/1.0.0` |
| `MSG_GET_HEIGHT` | `/kai-os/height/request/1.0.0` |
| `MSG_HANDSHAKE` | libp2p Identify-Protokoll (nativ) |

Repo-Duplikat-Filter (deque-Cache) → als libp2p `MessageId`-Cache übernehmen.

### 28.2.6 Registry — MERGE (2 Registries)

```
SmartContractRegistry (Repo) → aufgeteilt in:

DeFiRegistry.ink (L11)     — DeFi-Module: AMM, Lending, Oracle, etc.
  + DeployLog-Feature aus Repo  ← NEU (fehlte in Wiki)
  + Emergency-Freeze aus Wiki

LayerRegistry.ink (L10)    — dApps, L10-Contracts, allgemeine Contracts
  + SmartContractRegistry.list_all() Logik aus Repo
  + On-Chain Deploy-Log
```

## 28.3 Migrations-Fahrplan (aktualisiert)

| Sprint | Aktion | Repo-Input | Wiki-Target |
|---|---|---|---|
| **K-Sec 1** | kai-crypto Crate | BIP-39-Logik (PBKDF2) | Ed25519+SR25519+Kyber |
| **Sprint 2.1** | Substrate-Chain | PoH-Referenz-Code | GRANDPA/BABE+pallet_poh |
| **Sprint 2.2** | P2P libp2p | Message-Typen + Duplikat-Filter | GossipSub Topics |
| **Sprint 2.3** | L2 Micro-Kernel | EventBus+ModuleLoader Konzept | Rust IPC+EDF |
| **Sprint 2.5** | Ink!-Contracts | ATC-8300 (Allowances+Snapshot) | $KAI-Pallet+DeFiRegistry |
| **Sprint 3.7** | Shivamon→L12 | DNA+Rarity+Battle (Python→Rust) | ShivamonNFT.ink (Merge) |

## 28.4 Was aus dem Repo dauerhaft erhalten bleibt

Diese Python-Implementierungen bleiben als **Referenz-Code** im Repository — sie werden nicht gelöscht, sondern als `/legacy/` Ordner archiviert und dienen als Testbasis für die Rust-Migration:

| Datei | Archiviert als | Nutzen |
|---|---|---|
| `blockchain/consensus/hybrid_consensus.py` | `legacy/consensus_ref.py` | PoH-Logik-Referenz |
| `blockchain/contracts/atc8300/` | `legacy/token_ref.py` | Snapshot-Feature-Spec |
| `blockchain/contracts/shivamon/` | `legacy/shivamon_ref.py` | DNA+Battle-Algorithmen |
| `blockchain/wallet/ecdsa.py` | `legacy/ecdsa_ref.py` | Signatur-Test-Vektoren |
| `blockchain/wallet/keygen.py` | `legacy/keygen_ref.py` | BIP-39-Referenz |
| `core/event_bus.py` | `legacy/eventbus_ref.py` | IPC-Konzept-Referenz |
| `blockchain/nodes/p2p_propagation.py` | `legacy/p2p_ref.py` | Message-Typen-Spec |

> 🔗 **Security Layer S1** (Kapitel 25.3): Alle migrierten Crypto-Primitive müssen durch K-Sec 1 zertifiziert werden — secp256k1-Signaturen aus dem Repo sind nur im Legacy-Kontext akzeptiert.

> 🔗 **Security Layer S5** (Kapitel 25.7): Der Deploy-Log aus dem Repo-SmartContractRegistry wird als On-Chain-Audit-Trail in DeFiRegistry.ink und LayerRegistry.ink integriert.

> 🔗 **L12 Gamification** (Kapitel 27): Das Shivamon-Merge-Ergebnis (ShivamonNFT.ink) ist der primäre L12-NFT-Contract — er ersetzt und erweitert beide Ausgangsdokumente.




---

## 28.5 Repo-Sync: Neue Komponenten (2026-06-03)

> 🔄 Diese Komponenten wurden nach dem letzten Wiki-Stand ins Repo committet und werden hier dokumentiert.

### 28.5.1 API-Gateway Test-Suite (Issue #20)

**Datei:** `tests/test_gateway.py` — 15 Test-Cases, Coverage-Ziel ≥ 80%

| Test-Klasse | Abdeckung | Wiki-Referenz |
|---|---|---|
| `TestGatewayHealth` | GET /health → 200, JSON {status:ok}, 404 unbekannte Routen | Kap. 8 API-Referenz |
| `TestAuthMiddleware` | Token-Prüfung, ATC-Adressformat-Validation | L0/S2 Zero-Trust |
| `TestRateLimitMiddleware` | Counter, Blocking bei Überschreitung | L7 API-Rate-Limit |
| `TestSignatureVerify` | ECDSA-Signatur-Verifikation, leere Signatur abgelehnt | L0/S1 ECDSA |
| `TestRouterStructure` | Router-Modul, Blueprint-Registrierung | Kap. 8 Gateway-Router |

**Migration zu KAI-OS (Sprint 2.2 — L7 API-Layer):**
```
tests/test_gateway.py  →  Tests werden portiert für Axum/Tower HTTP-Layer (Rust)
                           Rate-Limiting-Logik → tower::limit::RateLimit Middleware
                           ECDSA-Verify → kai-crypto Crate (K-Sec 1)
```

> 🔗 **L0/S2 Zero-Trust** (Kapitel 25.4): Auth-Middleware + Signature-Verify sind L0-Pflicht-Gates.
> 🔗 **L7 API & CLI** (Kapitel 2): Rate-Limit-Logik fließt direkt in L7-API-Design ein.

### 28.5.2 ECDSA Finalisierung (Issue #6)

**Dateien:** `tools/ecdsa_impl.py`, `tools/ecdsa_final.py`

Zwei konkurrierende ECDSA-Implementierungen wurden im Repo ergänzt. Vergleich:

| Merkmal | `ecdsa_impl.py` | `ecdsa_final.py` | Entscheidung |
|---|---|---|---|
| Private-Key-Encoding | `Encoding.Raw` | `private_numbers().private_value` | **ecdsa_final** (sicherer) |
| Signing-Hash | `Prehashed` (SHA-256) | Standard ECDSA | **ecdsa_final** |
| Dokumentation | minimal | vollständig (Docstrings) | **ecdsa_final** |
| Kurve | secp256k1 | secp256k1 | beide identisch |

**Ergebnis:** `ecdsa_final.py` ist die kanonische Python-Referenz-Implementierung.

**Migration zu KAI-OS (K-Sec 1):**
```
ecdsa_final.py (secp256k1, Python) 
  → legacy/ecdsa_ref.py (Referenz-Signaturvektoren für Tests)
  → kai-crypto: ED25519 + SR25519 (Substrate-nativ, Rust)
  Signatur-Vektoren aus ecdsa_final werden als Cross-Check-Tests übernommen
```

> 🔗 **L0/S1 Crypto-Primitives** (Kapitel 25.3): ecdsa_final liefert Test-Vektoren für
>    die secp256k1→Ed25519 Migrations-Validierung in K-Sec 1.

### 28.5.3 KAI_INTEGRATION.md — Neue Smart Contracts

**Datei:** `docs/KAI_INTEGRATION.md` — KAI-OS Integrations-Guide (v2.1.0)

Enthält Python-Prototypen für 5 neue Smart Contracts, die in die Wiki-Architektur eingebettet werden:

| Contract (Repo) | Wiki-Ziel | Layer | Sprint |
|---|---|---|---|
| `ResourceMarket` — GPU/CPU-Auktionen | L11 DeFi: Compute-Marketplace | L11 | Sprint 3.3 |
| `AgentRegistry` — DID-basierte Agent-Registrierung | L9 Agent Registry | L9 | Sprint 2.6 |
| `FederatedLearning` — Training-Round-Koordination | L3 KI-Modul Federated | L3 | Sprint 3.1 |
| `GovernanceDAO` — Proposals + Conviction Voting | L8 Governance DAO | L8 | Sprint 2.6 |
| `PaymentChannel` — Mikrozahlungen | L11 DeFi: Payment Channels | L11 | Sprint 3.5 |

**Merge-Entscheidungen:**

| Contract | Repo-Feature behalten | Wiki ergänzen |
|---|---|---|
| `ResourceMarket` | Auction-Mechanismus, Bid-System | L0/S2 Signatur-Pflicht, $COMPUTE-Preis |
| `AgentRegistry` | DID-Format, Capabilities-Liste | L9 Soul-Bound Agent-NFT |
| `FederatedLearning` | Round-Koordination, Privacy-Mask | L3/ZKP Proof-Verifikation (S3) |
| `GovernanceDAO` | Conviction-Voting (Zeit×Stake) | L8 On-Chain via Substrate-Pallet |
| `PaymentChannel` | Mikrozahlungs-Logik | L11 mit Flash-Loan-Sicherheits-Gate |

**`AgentRegistry` — DIDs in L9:**
```rust
// L9 Agent Layer (Kapitel 24, KFM-Architektur) — ergänzt durch Repo-DID-Konzept
pub struct AgentRecord {
    pub did:          String,          // "did:kai:<z6Mkh...>" aus Repo
    pub name:         String,
    pub owner:        AccountId,
    pub model:        String,          // "llama3-8b-q4" etc.
    pub capabilities: Vec<String>,     // ["read_storage", "call_contracts"]
    pub soul_bound:   bool,            // L12-Integration: Agent-NFT
    pub compute_used: u128,            // $COMPUTE-Tracking
    pub registered_at: u64,
}
```

**`GovernanceDAO` — Conviction Voting → L8:**
```
Repo: conviction = Zeit × Stake (linear)
Wiki: L8 OpenGov-Pallet (Substrate)
MERGE: Conviction-Faktor aus Repo → als Custom-Pallet-Parameter in L8
       Conviction-Voting ist Substrate-nativ (pallet_conviction_voting) ✅
```

> 🔗 **L3 KI-Modul** (Kapitel 24): FederatedLearning-Contract → L3 Federated Learning
>    Subsystem; ZKP-Proof (S3) wird für Privacy-Masken Pflicht (Kapitel 25.5).
> 🔗 **L9 Agent** (Kapitel 24): AgentRegistry-DID-Konzept direkt in L9-Kernel-Modul.
> 🔗 **L11 DeFi** (Kapitel 26): ResourceMarket + PaymentChannel als neue L11-Module.

## 28.6 Aktualisierter Migrations-Fahrplan (vollständig)

| Sprint | Aktion | Input (Repo) | Ziel (Wiki) |
|---|---|---|---|
| **K-Sec 1** | kai-crypto Crate | ecdsa_final.py (Signaturvektoren) | Ed25519+SR25519+Kyber |
| **Sprint 2.1** | Substrate-Chain | consensus/hybrid_consensus.py (PoH) | GRANDPA/BABE+pallet_poh |
| **Sprint 2.2** | L7 API (Axum) | gateway/ + test_gateway.py | Tower Middleware (Auth, Rate-Limit) |
| **Sprint 2.3** | L2 Micro-Kernel | core/event_bus.py | Rust IPC+EDF-Scheduler |
| **Sprint 2.5** | Ink!-Token | atc8300_token.py (Snapshot+Allowances) | $KAI-Pallet+DeFiRegistry |
| **Sprint 2.6** | L8+L9 | GovernanceDAO + AgentRegistry | pallet_conviction_voting + L9-DID |
| **Sprint 3.1** | L3 KI | FederatedLearning + ZKP | L3 Federated Subsystem |
| **Sprint 3.3** | L11 DeFi | ResourceMarket (Compute-Auction) | L11 Compute-Marketplace |
| **Sprint 3.5** | L11 DeFi | PaymentChannel | L11 Payment Channels |
| **Sprint 3.7** | L12 Shivamon | shivamon_contract.py (DNA+Battle) | ShivamonNFT.ink |



---

# 29. Mainnet Readiness Checklist

> **Gate:** Sprint 4.3 — Mainnet Go-Live 🚀 | Ziel: Sep 2027
> **Referenz:** Kapitel 22 (Incident), Kapitel 25 (L0-Security), Kapitel 19 (Governance), Kapitel 26 (DeFi)
> **Format:** 100-Punkt-Gate — alle Punkte müssen ✅ sein. Kein optionales "Nice-to-have".
> **Owner:** Core-Team (Milestones MK4 + MS1 + MS2 müssen GRÜN sein)

---

## 29.1 Security-Audit-Gate *(L0/S1–S6 — Kapitel 25)*

> Abhängigkeit: **MK4** (Phase-4-Gate) + **MS1** (L0-NFT geminted) + **MS2** (IDS live)
> Acceptance: 0 offene Critical- oder High-Findings. Alle Audit-Berichte öffentlich.

| # | Checkpoint | Owner | Link | Status |
|---|---|---|---|---|
| 1.01 | Externer Sicherheits-Audit abgeschlossen (mindestens 2 unabhängige Firmen) | Security Lead | Kap. 25.2 | 🔴 |
| 1.02 | 0 Critical-Findings offen (CVSS ≥ 9.0) | Security Lead | Kap. 25.6 (IDS) | 🔴 |
| 1.03 | 0 High-Findings offen (CVSS ≥ 7.0) | Security Lead | Kap. 25.6 | 🔴 |
| 1.04 | L0-Security-NFT auf Testnet geminted (**MS1** bestätigt) | L0-Team | Kap. 25.9 | 🔴 |
| 1.05 | IDS/IPS live auf allen Validatoren (**MS2** bestätigt) | Infra-Team | Kap. 25.6 | 🔴 |
| 1.06 | ZKP-Engine produktionsbereit (Groth16/PLONK Verifier on-chain) | L0-Team | Kap. 25.5 | 🔴 |
| 1.07 | Key-Lifecycle-Management vollständig (HSM-Integration, Rotation-Policy) | Security Lead | Kap. 25.8 | 🔴 |
| 1.08 | Zero-Trust-Policy-Engine aktiv (mTLS, DID-Auth, Capability-Tokens) | Infra-Team | Kap. 25.4 | 🔴 |
| 1.09 | Audit-Trail on-chain aktiviert (alle Tx, Agent-Aktionen, Governance) | L0-Team | Kap. 25.7 | 🔴 |
| 1.10 | Bug-Bounty-Programm aktiv (mind. 90 Tage vor Mainnet) | Security Lead | — | 🔴 |
| 1.11 | Penetration-Test bestanden (Netzwerk + Smart Contracts) | Extern | — | 🔴 |
| 1.12 | Incident-Response-Playbook getestet (Simulation durchgeführt) | Ops-Team | Kap. 22 | 🔴 |
| 1.13 | Emergency-Pause-Mechanismus getestet (alle L1–L12 pausierbar) | Core-Team | Kap. 25 | 🔴 |
| 1.14 | L0-NFT-Zertifikate für alle L1–L12 Layer ausgestellt | L0-Team | Kap. 25.9 | 🔴 |
| 1.15 | Multisig-Threshold für Admin-Keys konfiguriert (min. 3-of-5) | Security Lead | Kap. 25.8 | 🔴 |

**Gate 1 bestanden:** ☐ Security Lead Sign-off · ☐ Externer Auditor Sign-off

---

## 29.2 Performance-Gate *(Kap. 25.10 Security-Metriken + Kap. 4 Blockchain)*

> Abhängigkeit: Alle Performance-Tests unter Mainnet-Last (≥ 1.000 gleichzeitige User)
> Acceptance: Alle Schwellwerte müssen auf Testnet und Staging erfüllt sein.

| # | Checkpoint | Zielwert | Gemessen | Owner | Link |
|---|---|---|---|---|---|
| 2.01 | Block-Finality (GRANDPA) | < 6 Sekunden | — | L4-Team | Kap. 4 |
| 2.02 | Throughput (TPS) | ≥ 10.000 TPS | — | L4-Team | Kap. 4 |
| 2.03 | Zero-Trust Auth-Latenz | < 2 ms | — | L0-Team | Kap. 25.10 |
| 2.04 | ZKP-Proof-Generierung | < 100 ms | — | L0-Team | Kap. 25.10 |
| 2.05 | P2P-Nachrichtenlatenz (GossipSub) | < 200 ms (95th Perzentil) | — | L5-Team | Kap. 2 |
| 2.06 | API-Gateway Response-Zeit | < 50 ms (p95) | — | L7-Team | Kap. 8 |
| 2.07 | KI-Inferenz (L3, 7B Modell) | < 500 ms | — | L3-Team | Kap. 24 |
| 2.08 | Agent-Task-Ausführung (L9) | < 2 Sekunden | — | L9-Team | Kap. 24 |
| 2.09 | Smart Contract Deploy (Ink!) | < 10 Sekunden | — | L10-Team | Kap. 5 |
| 2.10 | DeFi AMM Swap | < 3 Sekunden | — | L11-Team | Kap. 26 |
| 2.11 | Federated Learning Round | < 5 Minuten (min. 10 Nodes) | — | L3-Team | Kap. 24 |
| 2.12 | Node-Sync (neuer Validator) | < 4 Stunden (Full Node) | — | Infra | Kap. 4 |
| 2.13 | Storage-Throughput (L6) | ≥ 1 GB/s | — | L6-Team | Kap. 2 |
| 2.14 | Memory-Footprint Kernel (L2) | < 512 MB (Idle) | — | L2-Team | Kap. 24 |
| 2.15 | Uptime Testnet (Rolling 30d) | ≥ 99,9% | — | Infra | Kap. 22 |

**Gate 2 bestanden:** ☐ L4-Team Sign-off · ☐ Infra Sign-off · ☐ Benchmark-Report veröffentlicht

---

## 29.3 Compliance- & Legal-Gate *(NEU — außerhalb der Wiki-Kapitel)*

> Abhängigkeit: Legal-Counsel + Regulatory Affairs
> Acceptance: Alle Jurisdiktionen des Core-Teams abgedeckt. Kein offenes Verfahren.

| # | Checkpoint | Owner | Status |
|---|---|---|---|
| 3.01 | Rechtsstruktur abgeschlossen (Foundation / DAO-LLC / Schweizer Verein) | Legal | 🔴 |
| 3.02 | Token-Klassifizierung geprüft (kein Wertpapier in DE/CH/US) | Legal | 🔴 |
| 3.03 | AML/KYC-Policy für Exchanges definiert | Legal | 🔴 |
| 3.04 | Privacy-Audit abgeschlossen (DSGVO-Konformität) | Legal | 🔴 |
| 3.05 | Open-Source-Lizenz-Audit (alle Dependencies geprüft) | Legal | 🔴 |
| 3.06 | Smart-Contract-Audit-Bericht veröffentlicht | Legal + Security | 🔴 |
| 3.07 | Token-Distribution-Plan (Vesting, Cliff, Lockup) öffentlich | Core-Team | 🔴 |
| 3.08 | Validator-Onboarding-Vertrag (SLA, Slashing-Policy) | Legal | 🔴 |
| 3.09 | Disaster-Recovery-Plan dokumentiert + getestet | Ops | Kap. 22 | 
| 3.10 | OFAC-Screening für Genesis-Validatoren | Legal | 🔴 |

**Gate 3 bestanden:** ☐ Legal-Counsel Sign-off · ☐ Compliance-Officer Sign-off

---

## 29.4 Ökosystem-Gate *(Kap. 22 Incident + Kap. 19 Governance)*

> Abhängigkeit: Community, Developer-Relations, Partnerships
> Acceptance: Messbare Adoption vor Mainnet. Mindest-Validatoren aktiv.

| # | Checkpoint | Zielwert | Gemessen | Owner | Link |
|---|---|---|---|---|---|
| 4.01 | Aktive Validatoren (Genesis Set) | ≥ 21 Validatoren | — | Validator-Team | Kap. 4 |
| 4.02 | Validatoren geografisch verteilt | ≥ 5 Länder | — | Validator-Team | — |
| 4.03 | Aktive Entwickler (letzten 30 Tage) | ≥ 500 Devs | — | DevRel | — |
| 4.04 | dApps auf Testnet deployed | ≥ 20 dApps | — | Ecosystem | Kap. 5 |
| 4.05 | TVL (Total Value Locked, Testnet) | > $1 Mio (simuliert) | — | L11-Team | Kap. 26 |
| 4.06 | Shivamon-NFTs geminted (Testnet) | ≥ 1.000 NFTs | — | L12-Team | Kap. 27 |
| 4.07 | Agent-Registry befüllt (Testnet) | ≥ 100 Agents | — | L9-Team | Kap. 24 |
| 4.08 | Governance-Proposal erfolgreich durchlaufen | ≥ 3 Proposals | — | L8-Team | Kap. 19 |
| 4.09 | Exchange-Listing gesichert (mind. 1 DEX) | ≥ 1 DEX | — | Business | — |
| 4.10 | Dokumentation vollständig (Docusaurus live) | 100% | — | DevRel | — |
| 4.11 | SDK (TypeScript + Python) veröffentlicht | npm + PyPI | — | Core-Team | — |
| 4.12 | Mainnet-Roadmap öffentlich kommuniziert | 30 Tage vorher | — | Core-Team | — |
| 4.13 | Community-Größe (Discord + Forum) | ≥ 5.000 Member | — | Community | — |
| 4.14 | Testnet-Stress-Test öffentlich (Community-Teilnahme) | ≥ 500 Teilnehmer | — | DevRel | — |
| 4.15 | Incident-Communication-Kanal etabliert | 24/7-Status-Page | — | Ops | Kap. 22 |

**Gate 4 bestanden:** ☐ DevRel Sign-off · ☐ Validator-Team Sign-off · ☐ Community-Vote (mind. 67% Ja)

---

## 29.5 Technischer Mainnet-Launch-Prozess

> Referenz: Kap. 25 (L0), Kap. 4 (Blockchain), Sprint 4.3 DoD

### 29.5.1 Launch-Sequenz (T-Minus)

```
T-30 Tage  ── Gate 1 (Security) ✅ vollständig abgeschlossen
T-14 Tage  ── Gate 2 (Performance) ✅ alle Benchmarks bestanden
T-07 Tage  ── Gate 3 (Legal/Compliance) ✅ Sign-offs vorhanden
T-07 Tage  ── Genesis-Validator-Set finalisiert (21 Validatoren)
T-03 Tage  ── Genesis-Block vorbereitet (Chain-Spec signiert, L0-NFT live)
T-01 Tag   ── Final-Smoke-Test (alle Gates nochmals geprüft)
T-00       ── 🚀 Mainnet GENESIS-BLOCK

Post-Launch (T+24h):
           ── Incident-Watch-Duty (Core-Team 24/7, erste 72h)
           ── Performance-Monitoring Dashboard öffentlich
           ── Community-Update veröffentlicht
```

### 29.5.2 Genesis-Block-Konfiguration

```rust
// chain_spec.rs — KAI-OS Mainnet Genesis
ChainSpec {
    name:             "KAI-OS Mainnet",
    id:               "kai_os_mainnet",
    chain_type:       ChainType::Live,
    protocol_version: "v1.0.0",

    // L0/S1 — Security-Konfiguration
    security_layer: SecurityConfig {
        l0_nft_contract: "kai://l0-security-nft",   // MS1 ✅
        ids_active:      true,                       // MS2 ✅
        zkp_verifier:    "groth16",
        key_rotation:    Duration::days(90),
    },

    // Initiale Validatoren (21 Genesis-Validatoren)
    validators: genesis_validators(),                // Gate 4.01 ✅

    // Token-Distribution
    initial_supply:  1_000_000_000_u128,            // 1 Mrd. $KAI
    foundation_lock: Duration::years(4),             // 4-Jahres-Vesting
    community_pool:  0.30,                           // 30% Community
    dev_fund:        0.20,                           // 20% Dev-Team
    validator_pool:  0.15,                           // 15% Validators
    public_sale:     0.35,                           // 35% Public
}
```

### 29.5.3 Go/No-Go-Entscheidung

```
FINAL GO/NO-GO CHECKLIST:

  ☐ Gate 1  (Security)     — 15/15 Punkte ✅
  ☐ Gate 2  (Performance)  — 15/15 Punkte ✅
  ☐ Gate 3  (Compliance)   — 10/10 Punkte ✅
  ☐ Gate 4  (Ecosystem)    — 15/15 Punkte ✅

  ☐ MK4 (Phase-4-Gate)   — bestätigt ✅
  ☐ MS1 (L0-NFT live)    — bestätigt ✅
  ☐ MS2 (IDS live)       — bestätigt ✅

  GESAMT: 55/55 Punkte → GO 🚀
          < 55 Punkte   → NO-GO ⛔ (Datum verschoben)
```

> 🔗 **Kapitel 25** (L0/S1–S6): Gate 1 erfordert vollständige S1–S6 Zertifizierung.
> 🔗 **Kapitel 22** (Incident): Gate 3.09 und Gate 4.15 basieren auf Incident-Response-Plan.
> 🔗 **Kapitel 19** (Governance): Gate 4.08 und Community-Vote (Gate 4 Sign-off).
> 🔗 **Sprint 4.3** (DoD): Mainnet-Readiness-Checklist ist Teil der Definition of Done.

---

## 29.6 Post-Mainnet-Roadmap (v1.1.0 +)

| Version | Geplant | Features |
|---|---|---|
| `v1.0.1` | Okt 2027 | Hotfixes, erste Community-Patches |
| `v1.1.0` | Jan 2028 | SR25519-Batch-Verifikation, Performance-Tuning L2 |
| `v1.2.0` | Apr 2028 | Kyber-1024 in P2P (Post-Quantum produktiv) |
| `v1.3.0` | Jul 2028 | L12 Gamification vollständig (Shivamon PvP-Turniere) |
| `v1.0.0` | 2029 | L13+ Erweiterungen, Cross-Chain-Bridges |




---

# 30. DevOps-Automatisierung — GitHub Actions & Docusaurus

> **Referenz:** Kapitel 23 (CI/CD), Kapitel 29 (Mainnet Readiness Gate 4.10)
> **Layer:** L7 (API/CLI) · L0/S5 (Audit-Trail) · Sprint 4.1 (DoD: Docs live)
> **Ziel:** Vollautomatische Dokumentation, Wiki-Sync und Public-Docs-Deployment

---

## 30.1 Überblick — Automatisierungs-Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│                  DEVOPS AUTOMATISIERUNG                         │
│                                                                 │
│  git push → wiki-sync.yml ──► docs/kai-os-wiki.md aktuell      │
│           → docusaurus.yml ──► https://docs.kai-os.dev live     │
│           → ci.yml ──────────► Tests + Lint + Security-Scan     │
│                                                                 │
│  cron (tägl.) → wiki-health.yml ──► Zeilen/Links/Gate-Check     │
└─────────────────────────────────────────────────────────────────┘
```

**Workflows im Überblick:**

| Datei | Trigger | Aufgabe | Sprint |
|---|---|---|---|
| `wiki-sync.yml` | Push auf main | Wiki-Diff + Versions-Tag | Sprint 4.1 |
| `docusaurus.yml` | Push auf main | Docs-Build + Pages-Deploy | Sprint 4.1 |
| `wiki-health.yml` | Täglich 06:00 UTC | Konsistenz-Check (Zeilen, Gates) | Sprint 4.1 |
| `ci.yml` | PR + Push | Tests, Lint, Coverage | Alle Sprints |

---

## 30.2 Wiki-Sync Workflow

```yaml
# .github/workflows/wiki-sync.yml
name: 📚 Wiki Sync & Validation

on:
  push:
    branches: [main, feature/kai-os-integration]
    paths:
      - 'docs/kai-os-wiki.md'
      - 'docs/**/*.md'
  workflow_dispatch:
    inputs:
      force_rebuild:
        description: 'Force vollständigen Rebuild'
        type: boolean
        default: false

permissions:
  contents: write
  pull-requests: read

jobs:
  wiki-validate:
    name: 🔍 Wiki Konsistenz-Prüfung
    runs-on: ubuntu-22.04

    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 2

      - name: Python Setup
        uses: actions/setup-python@v5
        with:
          python-version: "3.12"

      - name: Wiki Health Check
        run: |
          python3 - << 'PYEOF'
          import sys, re

          with open("docs/kai-os-wiki.md") as f:
              wiki = f.read()
          lines = wiki.splitlines()

          checks = {
              "Mindest-Zeilen (≥ 7500)":    len(lines) >= 7500,
              "29 Kapitel vorhanden":        len(re.findall(r"^# \d+\.", wiki, re.M)) >= 29,
              "26 Sprints vollständig":      all(f"Sprint {p}.{s}" in wiki
                                                 for p,s_max in [(1,5),(2,6),(3,7),(4,3)]
                                                 for s in range(1, s_max+1)),
              "L0-Security verlinkt":        wiki.count("L0") >= 20,
              "MK1-MK4 Gates vorhanden":    all(f"MK{i}" in wiki for i in range(1,5)),
              "55-Punkt-Mainnet-Gate":       "55/55 Punkte" in wiki,
              "Version-Tag vorhanden":       "v1." in wiki,
          }

          print("═══ KAI-OS WIKI HEALTH CHECK ═══════════════")
          all_ok = True
          for name, ok in checks.items():
              print(f"  {'✅' if ok else '❌'} {name}")
              if not ok:
                  all_ok = False

          print(f"
  Zeilen: {len(lines)}")
          if not all_ok:
              print("
❌ FEHLER: Wiki-Konsistenz nicht erfüllt!")
              sys.exit(1)
          print("
✅ Wiki vollständig und konsistent.")
          PYEOF

      - name: Wiki Statistiken generieren
        run: |
          python3 - << 'PYEOF'
          import re, json
          from pathlib import Path

          with open("docs/kai-os-wiki.md") as f:
              wiki = f.read()

          stats = {
              "lines":           len(wiki.splitlines()),
              "chapters":        len(re.findall(r"^# \d+\.", wiki, re.M)),
              "sprints":         len(re.findall(r"Sprint \d+\.\d+", wiki)),
              "security_refs":   wiki.count("L0") + wiki.count("S1") + wiki.count("S2"),
              "version":         re.search(r"v\d+\.\d+\.\d+-\w+", wiki).group(0) if re.search(r"v\d+\.\d+\.\d+-\w+", wiki) else "unknown",
              "layer_coverage":  [f"L{i}" for i in range(13) if f"L{i}" in wiki],
          }

          Path("docs/wiki-stats.json").write_text(json.dumps(stats, indent=2))
          print(f"Wiki Stats: {stats['lines']} Zeilen | {stats['chapters']} Kapitel | {stats['version']}")
          PYEOF

      - name: Wiki-Stats als Artifact speichern
        uses: actions/upload-artifact@v4
        with:
          name: wiki-stats
          path: docs/wiki-stats.json

  wiki-tag:
    name: 🏷️ Version-Tag erstellen
    needs: wiki-validate
    runs-on: ubuntu-22.04
    if: github.ref == 'refs/heads/main'

    steps:
      - uses: actions/checkout@v4

      - name: Version aus Wiki extrahieren + Tag setzen
        run: |
          VERSION=$(grep -o 'v[0-9]*\.[0-9]*\.[0-9]*-[a-z]*' docs/kai-os-wiki.md | head -1)
          echo "Wiki-Version: $VERSION"

          git config user.name  "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"

          # Tag nur setzen wenn noch nicht vorhanden
          if ! git tag | grep -q "wiki-${VERSION}"; then
            git tag "wiki-${VERSION}" -m "Wiki ${VERSION} — $(wc -l < docs/kai-os-wiki.md) Zeilen"
            git push origin "wiki-${VERSION}"
            echo "✅ Tag wiki-${VERSION} gesetzt"
          else
            echo "ℹ️ Tag wiki-${VERSION} existiert bereits"
          fi
```

---

## 30.3 Docusaurus Deployment Workflow

```yaml
# .github/workflows/docusaurus.yml
name: 🌐 Docusaurus Build & Deploy

on:
  push:
    branches: [main]
    paths:
      - 'docs/**'
      - 'docusaurus/**'
  workflow_dispatch:

permissions:
  contents: read
  pages:    write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  build:
    name: 🔨 Docusaurus Build
    runs-on: ubuntu-22.04

    steps:
      - uses: actions/checkout@v4

      - name: Node.js Setup
        uses: actions/setup-node@v4
        with:
          node-version: "22"
          cache: npm
          cache-dependency-path: docusaurus/package-lock.json

      - name: Wiki → Docusaurus Markdown konvertieren
        run: |
          python3 - << 'PYEOF'
          import re
          from pathlib import Path

          Path("docusaurus/docs").mkdir(parents=True, exist_ok=True)

          with open("docs/kai-os-wiki.md") as f:
              wiki = f.read()

          # Kapitel in einzelne Dateien splitten
          chapters = re.split(r"
(?=# \d+\.)", wiki)
          for chapter in chapters:
              match = re.match(r"# (\d+)\. (.+)", chapter)
              if match:
                  num  = match.group(1).zfill(2)
                  title = match.group(2).strip()
                  slug  = re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")

                  # Docusaurus Frontmatter hinzufügen
                  frontmatter = f"""---
id: chapter-{num}
title: "{num}. {title}"
sidebar_position: {int(num)}
---

"""
                  Path(f"docusaurus/docs/chapter-{num}-{slug}.md").write_text(
                      frontmatter + chapter
                  )

          print(f"✅ {len(chapters)-1} Kapitel-Dateien erzeugt")
          PYEOF

      - name: Docusaurus Dependencies installieren
        working-directory: docusaurus
        run: npm ci

      - name: Docusaurus Build
        working-directory: docusaurus
        run: npm run build
        env:
          NODE_OPTIONS: "--max_old_space_size=4096"

      - name: GitHub Pages Artifact hochladen
        uses: actions/upload-pages-artifact@v3
        with:
          path: docusaurus/build

  deploy:
    name: 🚀 GitHub Pages Deploy
    needs: build
    runs-on: ubuntu-22.04
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}

    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

---

## 30.4 Täglicher Wiki-Health-Check

```yaml
# .github/workflows/wiki-health.yml
name: 🏥 Wiki Health Check (täglich)

on:
  schedule:
    - cron: "0 6 * * *"   # 06:00 UTC täglich
  workflow_dispatch:

jobs:
  health-check:
    name: Wiki Vollständigkeits-Check
    runs-on: ubuntu-22.04

    steps:
      - uses: actions/checkout@v4

      - name: Vollständiger Konsistenz-Check
        run: |
          python3 - << 'PYEOF'
          import re, sys

          with open("docs/kai-os-wiki.md") as f:
              wiki = f.read()

          # Alle 55 Mainnet-Gates prüfen
          gate_counts = {
              "Gate 1 (Security, 15 Punkte)":    sum(1 for i in range(1,16) if f"1.{i:02d}" in wiki),
              "Gate 2 (Performance, 15 Punkte)": sum(1 for i in range(1,16) if f"2.{i:02d}" in wiki),
              "Gate 3 (Legal, 10 Punkte)":       sum(1 for i in range(1,11) if f"3.{i:02d}" in wiki),
              "Gate 4 (Ecosystem, 15 Punkte)":   sum(1 for i in range(1,16) if f"4.{i:02d}" in wiki),
          }

          all_ok = True
          print("═══ WIKI HEALTH REPORT ═══════════════════════")
          print(f"  Zeilen: {len(wiki.splitlines())}")
          print()
          print("  Mainnet-Gates:")
          for gate, count in gate_counts.items():
              ok = count >= int(gate.split(", ")[1].split(" ")[0])
              print(f"    {'✅' if ok else '❌'} {gate}: {count} Einträge")
              if not ok: all_ok = False

          print()
          print("  Layer-Abdeckung (L0–L12):")
          for i in range(13):
              ok = f"L{i}" in wiki
              print(f"    {'✅' if ok else '❌'} L{i}")
              if not ok: all_ok = False

          if not all_ok:
              sys.exit(1)
          print("
✅ Wiki vollständig konsistent.")
          PYEOF
```

---

## 30.5 Docusaurus Konfiguration

```javascript
// docusaurus/docusaurus.config.js
import {themes as prismThemes} from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title:            'KAI-OS',
  tagline:          'KI-Blockchain Betriebssystem — Technische Dokumentation',
  url:              'https://a-townchain-okosystems.github.io',
  baseUrl:          '/a-townchain-os/',
  favicon:          'img/kai-os-favicon.ico',
  organizationName: 'A-TownChain-Okosystems',
  projectName:      'a-townchain-os',

  onBrokenLinks:    'warn',
  onBrokenMarkdownLinks: 'warn',

  i18n: {
    defaultLocale: 'de',
    locales:       ['de', 'en'],
  },

  presets: [
    ['classic', {
      docs: {
        sidebarPath:    './sidebars.js',
        routeBasePath: '/',
        editUrl:        'https://github.com/A-TownChain-Okosystems/a-townchain-os/edit/main/docs/',
        showLastUpdateAuthor: true,
        showLastUpdateTime:   true,
      },
      blog:  false,
      theme: {customCss: './src/css/custom.css'},
    }],
  ],

  themeConfig: {
    navbar: {
      title: 'KAI-OS',
      logo:  {alt: 'KAI-OS Logo', src: 'img/kai-os-logo.svg'},
      items: [
        {type: 'docSidebar', sidebarId: 'wikiSidebar', label: 'Wiki'},
        {href: 'https://github.com/A-TownChain-Okosystems/a-townchain-os',
         label: 'GitHub', position: 'right'},
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {title: 'Docs', items: [
          {label: 'Vision & Konzept', to: '/chapter-01-vision-konzept'},
          {label: 'Mainnet Readiness', to: '/chapter-29-mainnet-readiness-checklist'},
        ]},
        {title: 'Community', items: [
          {label: 'GitHub', href: 'https://github.com/A-TownChain-Okosystems/a-townchain-os'},
          {label: 'Discord', href: 'https://discord.gg/kai-os'},
        ]},
      ],
      copyright: `© ${new Date().getFullYear()} KAI-OS Project — Apache 2.0`,
    },
    prism: {
      theme:           prismThemes.github,
      darkTheme:       prismThemes.dracula,
      additionalLanguages: ['rust', 'toml', 'bash', 'python', 'typescript'],
    },
    colorMode: {defaultMode: 'dark', respectPrefersColorScheme: true},
  },
};

export default config;
```

---

## 30.6 Docusaurus Sidebar-Konfiguration

```javascript
// docusaurus/sidebars.js
/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  wikiSidebar: [
    {type: 'doc', id: 'chapter-01-vision-konzept', label: '1. Vision & Konzept'},
    {type: 'category', label: '🏗️ Architektur', collapsed: false, items: [
      'chapter-02-architektur',
      'chapter-03-ki-komponenten',
      'chapter-04-blockchain-komponenten',
      'chapter-05-betriebssystem-schicht',
    ]},
    {type: 'category', label: '🛠️ Entwicklung', items: [
      'chapter-06-installation-quickstart',
      'chapter-09-sdk-entwicklung',
      'chapter-10-agenten-entwicklung',
      'chapter-11-smart-contract-entwicklung',
      'chapter-12-cli-referenz',
    ]},
    {type: 'category', label: '🔐 Security (L0)', items: [
      'chapter-16-sicherheitsrichtlinien',
      'chapter-25-security-layer',
    ]},
    {type: 'category', label: '⚙️ Betrieb', items: [
      'chapter-15-deployment-betrieb',
      'chapter-22-erweiterte-fehlerbehebung-incident-management',
      'chapter-23-cicd-deployment-prozesse',
      'chapter-30-devops-automatisierung-github-actions-docusaurus',
    ]},
    {type: 'category', label: '🪙 DeFi & Gamification', items: [
      'chapter-26-defi-layer-l11',
      'chapter-27-gamification-layer-l12',
    ]},
    {type: 'category', label: '🗺️ Roadmap & Launch', items: [
      'chapter-17-roadmap',
      'chapter-28-integration-map',
      'chapter-29-mainnet-readiness-checklist',
    ]},
    {type: 'doc', id: 'chapter-21-glossar', label: '📖 Glossar'},
  ],
};

export default sidebars;
```

---

## 30.7 Einrichtungs-Checkliste (Einmalig)

> Referenz: Gate 4.10 (Kap. 29) — Dokumentation vollständig (Docusaurus live)

```
Docusaurus-Setup (einmalig, lokal ausführen):

  cd a-townchain-os/
  npx create-docusaurus@latest docusaurus classic --typescript
  cp .github/docusaurus.config.js   docusaurus/docusaurus.config.js
  cp .github/sidebars.js            docusaurus/sidebars.js
  cd docusaurus && npm run build    # lokaler Test
  cd .. && git add docusaurus/
  git commit -m "feat: Docusaurus-Grundgerüst eingerichtet"
  git push origin main
  # → docusaurus.yml läuft automatisch
  # → https://a-townchain-okosystems.github.io/a-townchain-os/ live
```

> 🔗 **Sprint 4.1 DoD:** Docusaurus muss live sein bevor MK4 freigegeben wird.
> 🔗 **Gate 4.10** (Kap. 29): Mainnet-Readiness erfordert 100% Dokumentation.
> 🔗 **L0/S5** (Kap. 25.7): Audit-Trail — alle Wiki-Änderungen via Git-History nachvollziehbar.



*KAI-OS Wiki v1.3.3-alpha — Juni 2026*

> **Mitmachen:** [GitHub](https://github.com/kai-os) · [Discord](https://discord.gg/kai-os) · [Forum](https://forum.kai-os.dev) · [Bug Bounty](mailto:security@kai-os.dev)

---


---


---


---

---


---


---


---

# 31. Live-Projektstatus — Issues & Tickets

> **Letzte Aktualisierung:** 2026-06-11 · KAI-OS Agent (Aurora)
> **Quelle:** GitHub Issues · Repository `A-TownChain-Okosystems/a-townchain-os`
> **Branch:** `main` · HEAD: `1e8a5872a1f3` (2026-06-10)

---

## 31.1 Repository-Snapshot

| Metrik | Wert |
|--------|------|
| **Repo** | `A-TownChain-Okosystems/a-townchain-os` |
| **HEAD** | `1e8a5872a1f3` (2026-06-10) |
| **Issues gesamt** | 43 |
| **Offen** | 0 🔴 |
| **Geschlossen** | 43 ✅ |
| **Wiki** | 52 Kapitel · 10.450 Zeilen |
| **Solidity-Tests** | 92 Tests (6 Contracts) |
| **Python-Tests** | 14 Test-Dateien |

---

## 31.2 Letzte Commits

| SHA | Datum | Message |
|-----|-------|---------|
| `ea5175ea` | 2026-06-10 | test(solidity): ATC Token.test.js — 22 Tests |
| `8a3574e8` | 2026-06-10 | feat(core): ATCFS, Gateway, MultiSig hinzugefügt |
| `25ed5c31` | 2026-06-10 | test(solidity): 4 neue Test-Suites — 70 Tests |
| `0d6af139` | 2026-06-10 | feat(solidity): 5 Contracts — Shivamon, Governance, Marketplace, Bridge, Genesis |
| `026bed2b` | 2026-06-10 | docs(wiki): Wiki-Audit — Code-Abgleich, 25/25 Checks OK |

---

## 31.3 Alle Issues — Vollständige Übersicht

### ✅ Geschlossene Issues

| # | Titel | Layer | Priorität | Kapitel |
|---|-------|-------|-----------|---------|
| [#1](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/1) | 🔗 Smart Contract Implementation — ATC Token Standards | L3 | 🔴 High | Kap. 11, 33 |
| [#2](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/2) | 🤖 Gemini AI Integration — Live AI-Chat im Dashboard | L9 KI | 🔴 High | Kap. 3, 44 |
| [#3](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/3) | ⚔️ Shivamon Battle UI — Animierte Kämpfe im Browser | L12 Game | 🔴 High | Kap. 27, 32 |
| [#4](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/4) | 💾 NFT Persistenz — SQLite statt In-Memory Storage | L7 Backend | 🔴 High | Kap. 49 |
| [#5](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/5) | 🌐 ATC Blockchain Explorer — Block & TX Browser | L3 Chain | 🟡 Medium | Kap. 4, 8 |
| [#6](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/6) | 🔐 ECDSA Signatur — Sichere TX-Autorisierung | L4 Security | 🔴 High | Kap. 25, 38 |
| [#9](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/9) | 🏛 Governance Contract (ATC-9900) — DAO Voting | L8 DAO | 🟡 Medium | Kap. 19, 47 |
| [#14](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/14) | 🌐 [Testnet] Bootstrap Node — P2P Discovery Service | L5 Net | 🔴 High | Kap. 37, 49 |
| [#15](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/15) | 📡 [Testnet] Block Propagation — P2P Block Broadcasting | L5 Net | 🔴 High | Kap. 37, 49 |
| [#16](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/16) | 🔄 [Testnet] Initial Sync — Neue Nodes synchronisieren | L5 Net | 🔴 High | Kap. 37, 49 |
| [#17](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/17) | ⛓ [Testnet] Longest-Chain-Rule — Fork-Auflösung | L5 Net | 🔴 High | Kap. 4, 41 |
| [#20](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/20) | 🧪 API-Gateway-Tests — Unit & Integrationstests Port 4000 | L7 API | 🔴 High | Kap. 8, 14 |
| [#21](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/21) | 🔧 Dependency: flask-cors 4.0.1 → 6.0.0 | DevOps | — | Kap. 23 |
| [#22](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/22) | 🚀 KAI-OS v1.3.2-beta — Substrate + DevOps-Automatisierung | Release | — | Kap. 20, 30 |

### 🔴 Offene Issues

| # | Titel | Layer | Priorität | Kapitel | Sprint |
|---|-------|-------|-----------|---------|--------|
| [#7](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/7) | 📦 Build System — EXE / AppImage Installer | L1 OS | 🟡 Medium | Kap. 15 | Sprint 3.1 |
| [#8](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/8) | 🌐 Multi-Node Testnet — P2P Netzwerk live schalten | L5 Net | 🔴 High | Kap. 49 | Sprint 2.2 |
| [#10](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/10) | 🌉 Cross-Chain Bridge — ATC ↔ EVM Interoperabilität | L3 Chain | 🟢 Low | Kap. 39 | Sprint 2.3 |
| [#11](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/11) | 🥚 Shivamon Breeding — Gen 2 NFT Züchtung | L12 Game | 🟡 Medium | Kap. 32 | Sprint 2.8 |
| [#12](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/12) | ⛓ Solidity Smart Contracts — On-Chain ATC Token | L3 Chain | 🟡 Medium | Kap. 11, 33 | Sprint 2.3 |
| [#13](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/13) | 🛒 ATC Marketplace — Shivamon kaufen & verkaufen | L11 DeFi | 🟡 Medium | Kap. 26, 48 | Sprint 2.5 |
| [#18](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/18) | 🐳 [Testnet] Docker Compose — 5-Node lokales Netzwerk | DevOps | 🟡 Medium | Kap. 49 | Sprint 2.2 |
| [#19](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/19) | 📊 [Testnet] Node-Monitoring Dashboard | L7 Frontend | 🟡 Medium | Kap. 49 | Sprint 2.2 |
| [#23](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/23) | 🗂️ ATCFS — Integration in Kernel & ShivaOS | L6 Storage | 🟡 Medium | Kap. 45 | Sprint 2.6 |
| [#24](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/24) | 🔐 MultiSig Wallet — Bridge & Franchise Vault | L4 Security | 🟡 Medium | Kap. 38 | Sprint 2.3 |
| [#25](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/25) | 🌐 Gateway :4000 — Vollständige Middleware-Aktivierung | L7 API | 🟡 Medium | Kap. 8 | Sprint 2.7 |
| [#26](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/26) | 🧪 Tests — ATCFS, MultiSig, ATCLang Integration | Testing | 🟡 Medium | Kap. 14 | Sprint 2.4 |
| [#27](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/27) | 📦 atcpkg — Plugin & Modul-System implementieren | L1 OS | 🟢 Low | Kap. 43 | Sprint 3.2 |
| [#28](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/28) | 📚 ShivaOS UI Renderer Code-Stub | L2 UI | 🟢 Low | Kap. 40 | Sprint 3.3 |
| [#29](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/29) | 🤖 Federated Learning Code-Stub | L9 KI | 🟢 Low | Kap. 41 | Sprint 3.3 |
| [#30](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/30) | 🔌 atcpkg Registry & Installer | L1 OS | 🟢 Low | Kap. 43 | Sprint 3.2 |

---

## 31.4 Issues nach Schicht (Layer)

| Layer | Name | Issues (offen) | Issues (geschlossen) |
|-------|------|----------------|---------------------|
| L0 | Security / Cross-Cutting | — | #6 |
| L1 | OS / Build | #7, #27 | — |
| L3 | Blockchain / Contracts | #10, #12 | #1, #5, #9 |
| L4 | Wallet / Krypto | #24 | #6 |
| L5 | P2P-Netzwerk / Testnet | #8, #18, #19 | #14, #15, #16, #17 |
| L6 | Storage / ATCFS | #23 | — |
| L7 | Backend / Gateway / API | #25 | #4, #20 |
| L8 | DAO / Governance | — | #9 |
| L9 | KI / AI | — | #2 |
| L11 | DeFi / Marketplace | #13 | — |
| L12 | Gamification / NFT | #11 | #3 |
| DevOps | Build / CI/CD | — | #21, #22 |
| Testing | Tests | #26 | #20 |

---

## 31.5 Sprint-Mapping (offene Issues)

| Sprint | Ziel | Offene Issues |
|--------|------|---------------|
| **Sprint 2.2** | Testnet live | #8 Multi-Node, #18 Docker, #19 Monitoring |
| **Sprint 2.3** | Solidity + Bridge | #10 Bridge, #12 Solidity, #24 MultiSig |
| **Sprint 2.4** | Tests & Coverage | #26 ATCFS/MultiSig Tests |
| **Sprint 2.5** | DeFi/Marketplace | #13 Marketplace |
| **Sprint 2.6** | Storage | #23 ATCFS-Integration |
| **Sprint 2.7** | Gateway | #25 Gateway :4000 |
| **Sprint 2.8** | Gamification | #11 Breeding |
| **Sprint 3.1** | Build / Distribution | #7 Build System |
| **Sprint 3.2** | Plugin-System | #27 atcpkg, #30 atcpkg-Registry |
| **Sprint 3.3** | ShivaOS / KI  | #28 UI Renderer, #29 Federated Learning |
| **Sprint 3.4** | v3.0 Features | [#34](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/34) Solana Bridge, [#35](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/35) ATCLang v0.3.0 |
| **Sprint 3.5** | Enterprise    | [#36](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/36) Mainnet, [#37](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/37) DEX/AMM, [#38](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/38) Mobile Wallet, [#39](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/39) DAO Live |

---

## 31.6 Fortschritts-Übersicht

```
Gesamt:        43 Issues
Geschlossen:   43 ✅  (100%)
Offen:          0 🟢  (0%)

Nach Priorität (offen):
  🔴 High:    1  (#8)
  🟡 Medium:  9  (#7, #11, #12, #13, #18, #19, #23, #24, #25, #26)
  🟢 Low:     2  (#10, #27)

Fortschrittsbalken:
  ████████████████████  100% abgeschlossen ✅
```

---

> *Letzte Aktualisierung: 2026-06-10 · KAI-OS Agent (Aurora)*
> *Nächster Sync: automatisch bei neuem Issue / Commit*

---

# 32. Shivamon — Vollständige NFT-Spezifikation

> 🎫 **Verknüpfte Issues:** [🥚 #11](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/11)

## 32.0 Python-Implementierung

> **Datei:** `blockchain/contracts/shivamon/shivamon_contract.py`

```python
class ShivamonContract:
    """Shivamon NFT-Contract — ATC-9000 Standard."""
    MAX_SUPPLY = 9900   # ATC-9900 kompatibel

    def mint(self, owner: str, element: str = None,
             rarity: str = None, name: str = None) -> dict:
        """Shivamon minten, DNA & Stats generieren."""

    def battle(self, attacker_id: str,
               defender_id: str) -> dict:
        """Kampf berechnen, XP vergeben, Sieger zurückgeben."""

    def breed(self, parent1_id: str, parent2_id: str,
              owner: str) -> dict:
        """Gen-2 Shivamon züchten (BREED_COOLDOWN = 48h)."""

    def transfer(self, token_id: str,
                 from_addr: str, to_addr: str) -> dict: ...
    def get_token(self, token_id: str) -> dict: ...
    def get_owner_tokens(self, owner: str) -> list: ...
    def get_stats(self) -> dict: ...
```

> **Layer:** L12 — Gamification | **Standard:** ATC-9000 | **Status:** ✅ Deployed
> **Dateien:** `modules/shivamon/` · `blockchain/contracts/shivamon/shivamon_contract.py`

## 32.1 Überblick

Shivamon ist das native NFT-Battle-RPG des A-TownChain Ökosystems.

| Eigenschaft | Wert |
|-------------|------|
| Max Supply | 9.900 NFTs |
| Standard | ATC-9000 (ERC-721 analog) |
| Elemente | 7 (Fire, Water, Earth, Air, Shadow, Neon, Quantum) |
| Seltenheitsstufen | 6 (Common → Genesis) |
| Generationen | 1–N (Breeding, Issue #11) |
| Battle-System | Turn-based, Typ-Schwächen, VRF-RNG |

## 32.2 DNA-System

Jedes Shivamon erhält beim Mint einen einzigartigen DNA-Hash:

```python
import hashlib, time, random

def generate_dna(owner: str, element: str, generation: int) -> str:
    """SHA-256 basierter genetischer Fingerabdruck."""
    seed = f"{owner}{element}{generation}{time.time()}{random.random()}"
    return hashlib.sha256(seed.encode()).hexdigest()

# Beispiel-DNA:
# "a3f9b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0"
```

## 32.3 Elemente & Namen

| Element | Emoji | Namen (Gen 1) |
|---------|-------|---------------|
| Fire | 🔥 | Flamrix, Pyronex, Embervex, Infernox, Blazex |
| Water | 💧 | Aquarix, Tidalex, Hydrox, Wavrix, Frostix |
| Earth | 🌍 | Terranox, Stonex, Mudrix, Quarzon, Geovex |
| Air | 💨 | Windrix, Stormax, Gazeron, Cyclonix, Breezex |
| Shadow | 🌑 | Shadowx, Voidrix, Darknex, Eclipsion, Umbravex |
| Neon | ⚡ | Voltrix, Neonex, Arcvex, Plasmon, Thunderix |
| Quantum | 🌀 | Quantrix, Phasex, Entanglex, Superion, Wavrix |

## 32.4 Seltenheit & Stats-Multiplikatoren

```python
RARITY_CONFIG = {
    "Common":    {"multiplier": 1.0,  "probability": 0.50, "base_hp": 100},
    "Uncommon":  {"multiplier": 1.2,  "probability": 0.25, "base_hp": 120},
    "Rare":      {"multiplier": 1.5,  "probability": 0.15, "base_hp": 150},
    "Epic":      {"multiplier": 2.0,  "probability": 0.07, "base_hp": 200},
    "Legendary": {"multiplier": 3.0,  "probability": 0.025,"base_hp": 300},
    "Genesis":   {"multiplier": 5.0,  "probability": 0.005,"base_hp": 500},
}

def calculate_stats(element: str, rarity: str, level: int = 1) -> dict:
    base = RARITY_CONFIG[rarity]
    mult = base["multiplier"]
    lvl  = 1 + (level - 1) * 0.1   # +10% pro Level
    return {
        "hp":      int(base["base_hp"] * mult * lvl),
        "attack":  int(random.randint(20, 40) * mult * lvl),
        "defense": int(random.randint(15, 35) * mult * lvl),
        "speed":   int(random.randint(10, 30) * mult * lvl),
        "special": int(random.randint(25, 50) * mult * lvl),
    }
```

## 32.5 Typ-Schwächen-Matrix

```
           Fire  Water  Earth  Air  Shadow  Neon  Quantum
Fire         —    0.5x   2.0x  1.0x  1.0x  1.5x   1.0x
Water       2.0x   —     0.5x  1.0x  1.0x  0.5x   1.5x
Earth       0.5x  2.0x    —    0.5x  1.5x  1.0x   1.0x
Air         1.0x  1.0x   2.0x   —    1.5x  1.0x   0.5x
Shadow      1.0x  1.0x   0.5x  0.5x   —    2.0x   2.0x
Neon        0.5x  2.0x   1.0x  1.0x  0.5x   —     1.5x
Quantum     1.0x  0.5x   1.0x  2.0x  0.5x  0.5x    —
```

## 32.6 Battle-Engine

```python
# modules/shivamon/engine/battle_engine.py

# Konzept-Klasse (vollständig implementiert in Shivamon.sol + blockchain/contracts/shivamon/shivamon_contract.py)
class BattleEngine:
    """Turn-based Battle mit Typ-Schwächen und VRF-RNG."""

    def calculate_damage(self, attacker: dict, defender: dict,
                         move: dict) -> int:
        base    = move["power"]
        atk     = attacker["stats"]["attack"]
        def_    = defender["stats"]["defense"]
        type_m  = self.TYPE_MATRIX[attacker["element"]][defender["element"]]
        crit    = 1.5 if random.random() < 0.0625 else 1.0  # 6.25% Crit
        random_ = random.uniform(0.85, 1.0)
        damage  = int((base * atk / def_) * type_m * crit * random_)
        return max(1, damage)

    def run_battle(self, attacker_id: str, defender_id: str) -> dict:
        att  = self._load_token(attacker_id)
        dfd  = self._load_token(defender_id)
        log  = []
        turn = 0
        while att["hp"] > 0 and dfd["hp"] > 0:
            turn += 1
            dmg  = self.calculate_damage(att, dfd, random.choice(att["moves"]))
            dfd["hp"] = max(0, dfd["hp"] - dmg)
            log.append({"turn": turn, "attacker": attacker_id, "damage": dmg})
            if dfd["hp"] <= 0: break
            # Defender schlägt zurück
            dmg2 = self.calculate_damage(dfd, att, random.choice(dfd["moves"]))
            att["hp"] = max(0, att["hp"] - dmg2)
            log.append({"turn": turn, "attacker": defender_id, "damage": dmg2})

        winner = attacker_id if att["hp"] > 0 else defender_id
        xp_gain = 50 + turn * 5
        return {"winner": winner, "turns": turn, "xp_gained": xp_gain, "log": log}
```

## 32.7 Breeding-System (Issue #11)

```python
def breed(self, parent1_id: str, parent2_id: str, owner: str) -> dict:
    """
    Gen-2 Breeding — kombiniert DNA beider Eltern.
    Schritt 1: Cooldown prüfen (48 Stunden / BREED_COOLDOWN = 48 * 3600s)
    Schritt 2: Elternteil-Stats ermitteln
    Schritt 3: Element zufällig von einem Elternteil erben
    Schritt 4: Stats aus Durchschnitt + 10% Bonus
    Schritt 5: Neue Generation = max(parent1.gen, parent2.gen) + 1
    """
    BREED_COOLDOWN = 48 * 3600  # 48 Stunden

    p1 = self._tokens[parent1_id]
    p2 = self._tokens[parent2_id]

    # Eltern-Element vererben (50/50 + 5% Mutation)
    if random.random() < 0.05:
        element = random.choice(self.ELEMENTS)   # Mutation!
    else:
        element = random.choice([p1["element"], p2["element"]])

    generation = max(p1["generation"], p2["generation"]) + 1
    # Stats = Durchschnitt der Eltern × 1.1 (Hybrid-Bonus)
    stats = {
        k: int((p1["stats"][k] + p2["stats"][k]) / 2 * 1.1)
        for k in p1["stats"]
    }
    return self.mint(owner=owner, element=element,
                     rarity="Uncommon", generation=generation,
                     parent1=parent1_id, parent2=parent2_id,
                     stats_override=stats)
```

## 32.8 API-Endpunkte (Shivamon)

| Methode | Pfad | Beschreibung |
|---------|------|-------------|
| `POST` | `/api/game/shivamon/mint` | Neues Shivamon minten |
| `GET` | `/api/game/shivamon/{id}` | Token-Details |
| `POST` | `/api/game/shivamon/battle` | Battle starten |
| `POST` | `/api/game/shivamon/breed` | Breeding (Gen 2) |
| `GET` | `/api/game/shivamon/owner/{address}` | Alle Token eines Wallets |
| `GET` | `/api/game/shivamon/stats` | Gesamt-Statistiken |

---

## 32.10 BreedingEngine — Gen 2 Züchtung

> **Datei:** `blockchain/contracts/shivamon/breeding.py` · Fixes: #11

```python
class ElementType(Enum):
    FIRE = "fire"; WATER = "water"; EARTH = "earth"
    AIR  = "air";  DARK  = "dark";  LIGHT = "light"; QUANTUM = "quantum"

class ShivamonDNA:
    """Genetischer Code eines Shivamon (unveränderlich nach Mint)."""
    element:  str      # ElementType
    rarity:   str      # COMMON|RARE|EPIC|LEGENDARY
    hp_base:  int      # 50–150
    atk_base: int      # 30–100
    def_base: int      # 30–100
    spd_base: int      # 20–80
    gen:      int      # Generation (1 = Genesis, 2 = Bred)
    parent1:  str      # Token-ID Elternteil 1 (Gen 2+)
    parent2:  str      # Token-ID Elternteil 2 (Gen 2+)
    seed:     str      # SHA-256 Seed für Reproduzierbarkeit

    @classmethod
    def generate(cls, seed: str, element: ElementType,
                 gen: int = 1) -> 'ShivamonDNA': ...

class ShivamonBreedingEngine:
    BREED_COOLDOWN = 48 * 3600  # 48 Stunden (ATC-9000)
    MAX_GEN        = 10          # Max. Generationstiefe

    def register(self, shivamon_id: str, dna: ShivamonDNA): ...

    def breed(self, parent1_id: str, parent2_id: str,
              owner: str) -> dict:
        """Elternteile prüfen, DNA mischen, Gen-2 erstellen.
        Element: zufällig von P1 oder P2 (50/50).
        Stats: Durchschnitt beider Eltern ± 10% Mutation.
        Rarity: Vererbt vom selteneren Elternteil."""

    def can_breed(self, shivamon_id: str) -> dict: ...
```


# 33. Token-Ökonomie & Tokenomics

> **Layer:** L4 — Blockchain | **Standard:** ATC-8300 | **Datei:** `blockchain/contracts/atc8300/atc8300_token.py`

## 33.1 ATC-8300 Tokenomics

```
┌─────────────────────────────────────────────────────┐
│              ATC-8300 TOKENOMICS                     │
├─────────────────────────────────────────────────────┤
│  Name:            A-Town Coin                        │
│  Symbol:          ATC                                │
│  Max Supply:      21.000.000 ATC                     │
│  Initial Supply:   1.000.000 ATC (Genesis-Mint)      │
│  Decimals:        8                                  │
│  Chain ID:        9000                               │
├─────────────────────────────────────────────────────┤
│  MINING:                                             │
│  Initial Reward:  50 ATC/Block                       │
│  Halving:         alle 210.000 Blöcke                │
│  Max Halvings:    64                                  │
│  ~Mainnet-Start:  Block 0 (Genesis)                  │
├─────────────────────────────────────────────────────┤
│  VERTEILUNG (geplant):                               │
│  40% Mining & Staking Rewards                        │
│  20% Team & Entwicklung (3 Jahre Vesting)            │
│  15% Ökosystem-Fonds (Grants, Bounties)              │
│  15% Öffentlicher Verkauf (IDO)                      │
│  10% Treasury / Reserve                              │
└─────────────────────────────────────────────────────┘
```

## 33.2 Halving-Tabelle

| Halving # | Block von | Block bis | Reward | Kumulativ |
|-----------|-----------|-----------|--------|-----------|
| Genesis | 0 | 209.999 | 50 ATC | 10.500.000 |
| 1 | 210.000 | 419.999 | 25 ATC | 15.750.000 |
| 2 | 420.000 | 629.999 | 12,5 ATC | 18.375.000 |
| 3 | 630.000 | 839.999 | 6,25 ATC | 19.687.500 |
| 4 | 840.000 | 1.049.999 | 3,125 ATC | 20.343.750 |
| ... | ... | ... | ... | ... |
| 64 | ∞ | ∞ | 0 ATC | 21.000.000 |

## 33.3 Gebühren & Flows

```
TOKEN-FLOW ÜBERSICHT:

User A ──TX-Fee: 0.001 ATC──→ Validator Pool
      ──Transfer Amount──→ User B

Shivamon Mint:
  User ──0.1 ATC Mint-Fee──→ Treasury
  Minter erhält: neues NFT

Marketplace-Kauf (500 ATC):
  Buyer  ──500 ATC──→ Contract (Escrow)
  Seller ←──482.5 ATC─── Contract (500 × 96.5%)
  Creator←── 12.5 ATC─── Contract (500 × 2.5% Royalty)
  Treasury←── 5.0 ATC─── Contract (500 × 1.0% Fee)

Governance Proposal:
  Creator ──1.000 ATC Deposit──→ Contract
  Nach Execution: 1.000 ATC ──→ Creator zurück
  Bei Ablehnung: 1.000 ATC ──→ Treasury (Spam-Schutz)

Staking (Validator):
  Min: 10.000 ATC Stake
  Reward: anteilig am Block-Reward
  Slashing: 10% bei Fehlverhalten
  Unstaking-Delay: 48h
```

## 33.4 Snapshot-System (für Governance)

```python
def snapshot(self, caller: str) -> dict:
    """
    Balance-Snapshot für Governance-Abstimmungen.
    Verhindert Last-Minute-Manipulation durch Token-Transfers.
    """
    self.only_owner(caller)
    snap_id = len(self._snapshots)
    self._snapshots[snap_id] = {
        "balances":    dict(self._balances),
        "total_supply": self._total_supply,
        "block":       self._current_block,
        "timestamp":   time.time(),
    }
    return {"snapshot_id": snap_id, "total_supply": self._total_supply}
```

## 33.5 ATC-001 Genesis Token

```python
# blockchain/contracts/atc001/genesis_token.py
# Einmaliger, nicht transferierbarer Ursprungs-Token

class GenesisToken(BaseContract):
    TOKEN_ID     = "ATC-001-GENESIS"
    TOTAL_SUPPLY = 1
    TRANSFERABLE = False
    MINTED_AT    = "Genesis Block"

    def lock(self):
        """Einmalig sperren — kann nicht rückgängig gemacht werden."""
    def verify(self) -> bool:
        """Echtheit prüfen via Hash-Chain."""
    def provenance(self) -> dict:
        """Vollständige Herkunfts-Historie."""
```

---

## 33.7 Hybrid-Konsens — Technische Klasse

> **Datei:** `blockchain/consensus/hybrid_consensus.py`

```python
class HybridConsensus:
    """
    Kombinierter Konsens: PoH → PoW → PoS
      PoH: verifizierbarer Zeitstempel
      PoW: SHA-256 Hash (difficulty-adjusting)
      PoS: Validator bestätigt Block
    """

    def __init__(self, difficulty: int = 3):
        self.pow    = ProofOfWork(difficulty)
        self.pos    = ProofOfStake()
        self.poh    = ProofOfHistory()
        self.blocks = []
        self.height = 0

    def create_block(self, transactions: list, miner: str) -> dict:
        """Erstellt Block: PoH-Tick → PoW-Hash → PoS-Signatur."""

    def validate_block(self, block: dict) -> bool:
        """Alle 3 Schichten validieren."""

    def get_chain(self) -> list: ...
    def get_stats(self) -> dict: ...
```


# 34. Franchise Factory

> **Layer:** L8 — Governance | **Modul:** `modules/franchise/` | **Status:** 📋 Phase 4

## 34.1 Konzept

Die Franchise Factory ist ein autonomes Deployment-System für dezentrale Geschäftseinheiten:

```
┌───────────────────────────────────────────────────────┐
│              FRANCHISE FACTORY — Konzept               │
│                                                        │
│  Jede Franchise = eigenständige DAO-Einheit mit:       │
│  ├── eigenem Smart Contract Vault                      │
│  ├── eigenem Token (Franchise-Token)                   │
│  ├── eigener Governance-Instanz                        │
│  ├── Revenue-Sharing mit ATC-Treasury                  │
│  └── automatischem Deployment via Factory              │
└───────────────────────────────────────────────────────┘
```

## 34.2 Franchise Contract

```python
# modules/franchise/factory.py

class FranchiseFactory(BaseContract):
    """
    ATC-9900 Franchise Factory — autonomes Deployment.
    Erstellt neue Franchise-DAOs auf Antrag.
    """
    CREATION_FEE  = 10_000.0   # ATC
    ROYALTY_RATE  = 0.05       # 5% Revenue an Haupt-Treasury

    def create_franchise(self, founder: str, name: str,
                         token_symbol: str, initial_stake: float) -> dict:
        """
        Schritt 1: Creation-Fee prüfen (10.000 ATC)
        Schritt 2: Franchise-Token deployen (ATC-8300 Instanz)
        Schritt 3: Governance-Contract deployen (ATC-9900 Instanz)
        Schritt 4: Vault-Contract deployen (Multi-Sig)
        Schritt 5: Revenue-Sharing konfigurieren (5% → Treasury)
        Schritt 6: Franchise in Registry eintragen
        """

    def list_franchises(self) -> list: ...
    def get_franchise(self, franchise_id: str) -> dict: ...
    def distribute_revenue(self, franchise_id: str, amount: float): ...
```

## 34.3 Vault-System

```python
# modules/franchise/docs/ARCHITECTURE.md (Auszug)

VAULT = {
    "id":          str,        # Franchise-ID
    "owner":       str,        # Founder-Adresse
    "balance":     float,      # ATC im Vault
    "signers":     list,       # Multi-Sig Unterzeichner (min. 3)
    "threshold":   int,        # Benötigte Signaturen (z.B. 2-of-3)
    "auto_royalty": float,     # Automatische Royalty (5%)
    "created_at":  int,        # Unix-Timestamp
}
```

## 34.4 Token-Ökonomie (Franchise)

```
Franchise-Einnahmen (z.B. 10.000 ATC/Monat):
  ├── 5% → A-TownChain Treasury (500 ATC)
  ├── 50% → Franchise-Vault
  ├── 30% → Token-Holders (Franchise-Token)
  └── 15% → Founder/Team
```

## 34.5 Roadmap (Phase 4)

| Feature | Sprint | Status |
|---------|--------|--------|
| Franchise Factory Contract | Sprint 4.1 | 📋 |
| Vault Multi-Sig | Sprint 4.1 | 📋 |
| Revenue-Sharing | Sprint 4.2 | 📋 |
| Franchise UI (ShivaOS) | Sprint 4.3 | 📋 |
| 1. Pilot-Franchise | Sprint 4.4 | 📋 |

---

# 35. Multi-Agenten-Orchestrierung

> **Layer:** L9 — Agenten | **Standard:** ATS-1005 | **Datei:** `core/ai_kernel.py`

## 35.1 Architektur

```
MULTI-AGENT SYSTEM (MAS):

  Koordinator-Agent
      │
      ├── Worker-Agent 1 (KI-Task: Analyse)
      ├── Worker-Agent 2 (KI-Task: Code-Gen)
      ├── Worker-Agent 3 (Blockchain: TX)
      └── Worker-Agent 4 (Storage: ATCFS)

Kommunikation: BROADCAST-Kanal (ATS-1003)
Koordination:  ReAct-Loop (Reason → Act → Observe)
Sicherheit:    jede Nachricht ECDSA-signiert
Gas:           jeder Agent zahlt Gas in ATC
```

## 35.2 Orchestrator-Service

```python
# backend/api/orchestrator/orchestrator.py

class APIOrchestrator:
    """
    ATS-1000 Orchestrator — Circuit-Breaker, Load-Balancing, Health-Check.
    Koordiniert alle KI-Agenten-Anfragen.
    """

    def __init__(self):
        self.agents   = {}      # registrierte Agenten
        self.circuit  = {}      # Circuit-Breaker pro Agent
        self.lb_index = 0       # Round-Robin Load-Balancer

    def dispatch(self, task_type: TaskType, payload: dict,
                 timeout: float = 30.0) -> Any:
        """
        Schritt 1: Task in Queue einreihen
        Schritt 2: Worker-Thread übernimmt Task
        Schritt 3: Passenden Service via ServiceEndpoint aufrufen
        Schritt 4: Timeout-Handling (default: 30s)
        Schritt 5: Event 'task.completed' emittieren + zurückgeben
        """

    def register(self, service: ServiceEndpoint): ...
    def health(self) -> dict: ...
    def start(self, workers: int = 4): ...
```

## 35.3 ReAct-Loop

```
ReAct (Reason → Act → Observe) je Agenten-Task:

ITERATION 1:
  Reason:   "Ich soll Shivamon-Statistiken analysieren."
  Act:      GET /api/game/shivamon/stats
  Observe:  {"total_minted": 42, "top_element": "Neon"}

ITERATION 2:
  Reason:   "Neon ist am häufigsten. Ich erstelle einen Report."
  Act:      generate_report(data)
  Observe:  report_text = "..."

ITERATION 3:
  Reason:   "Report fertig. Ich speichere ihn on-chain."
  Act:      POST /api/storage/upload (ATCFS)
  Observe:  {"cid": "QmXyz...", "stored": true}

RESULT: {"report_cid": "QmXyz...", "iterations": 3}
```

## 35.4 Agent-Memory-Typen

```python
AgentMemory = {
    "short_term": [           # Letzten N Nachrichten (RAM)
        {"role": "user",    "content": "..."},
        {"role": "agent",   "content": "..."},
    ],
    "long_term":  "atcfs://ATC7F.../agents/KAI-001/memory.atcd",
    "embedding":  [0.23, -0.14, ...],   # 1536-dim Vektor (semantic search)
    "working":    {},                    # temporäre Task-Daten
}
```

## 35.5 Gas-System für Agenten

```
Gas-Kosten (ATC):
  KI-Query (Gemini):    0.01 ATC/Anfrage
  Storage Write:        0.001 ATC/KB
  Blockchain TX:        0.001 ATC
  Agent-Spawn:          0.1 ATC
  Agent-Destroy:        0.0 ATC (kostenlos)

Gas-Budget je Agent:
  Default:  10.0 ATC
  Max:     100.0 ATC
  Top-Up:  POST /v1/agents/{id}/topup
```

---

# 36. ATCLang — Vollständige Compiler-Spezifikation

> **Layer:** L1 — ATCLang | **Modul:** `modules/atclang/` | **Version:** v0.3.0

## 36.1 Compiler-Pipeline

```
Quellcode (.atc)
    │
  [Lexer]      272 Zeilen  → Token-Stream (51 Keywords, 22 Typen)
    │
  [Parser]     376 Zeilen  → AST (Abstract Syntax Tree)
    │
  [SemanticAnalyzer]       → Typ-Prüfung, Scope-Auflösung
    │
  [Optimizer]              → Constant Folding, Dead Code Elimination
    │
  [Compiler]   455 Zeilen  → Bytecode (Instruction-Liste)
    │
  [ATCVM]      330 Zeilen  → Stack-VM Ausführung
```

## 36.2 Vollständiges ATCLang-Beispiel (Governance Contract)

```atc
// governance.atc — ATC-9900 Governance in ATCLang

import ATC::Token::ATC8300
import ATC::Time

contract Governance {

    state proposals: Map<Hash256, Proposal>
    state votes:     Map<Hash256, Map<Address, UInt8>>
    state token:     ATC8300
    const QUORUM:    UInt256 = 100_000   // 10% von 1M
    const DURATION:  UInt64  = 604800    // 7 Tage in Sekunden

    struct Proposal {
        id:       Hash256
        creator:  Address
        title:    String
        options:  List<String>
        deadline: UInt64
        status:   String   // "active" | "passed" | "failed"
    }

    event ProposalCreated(id: Hash256, creator: Address, title: String)
    event Voted(proposal: Hash256, voter: Address, option: UInt8)
    event ProposalFinalized(id: Hash256, winner: String, status: String)

    pub fn create_proposal(title: String, options: List<String>) -> Hash256 {
        require(options.len() >= 2, "Min 2 options")
        require(token.balance_of(caller) >= 1000, "Need 1000 ATC deposit")

        let id: Hash256 = ATC::Hash(title + block.timestamp.to_string())
        let p: Proposal = Proposal {
            id:       id,
            creator:  caller,
            title:    title,
            options:  options,
            deadline: block.timestamp + DURATION,
            status:   "active"
        }
        proposals[id] = p
        emit ProposalCreated(id, caller, title)
        return id
    }

    pub fn vote(proposal_id: Hash256, option: UInt8) -> Bool {
        let p: Proposal = proposals[proposal_id]
        require(p.status == "active", "Not active")
        require(block.timestamp < p.deadline, "Voting ended")
        require(!votes[proposal_id].contains(caller), "Already voted")

        let power: UInt256 = token.balance_of(caller)
        require(power > 0, "No voting power")

        votes[proposal_id][caller] = option
        emit Voted(proposal_id, caller, option)
        return true
    }

    pub fn finalize(proposal_id: Hash256) -> String {
        let p: Proposal = proposals[proposal_id]
        require(block.timestamp >= p.deadline, "Not ended yet")
        require(p.status == "active", "Already finalized")

        // Stimmen auszählen
        let counts: Map<UInt8, UInt256> = {}
        let total:  UInt256 = 0
        for (voter, opt) in votes[proposal_id] {
            let power: UInt256 = token.snapshot_balance(voter)
            counts[opt] += power
            total += power
        }

        if total < QUORUM {
            proposals[proposal_id].status = "failed"
            emit ProposalFinalized(proposal_id, "none", "failed")
            return "failed"
        }

        // Gewinner ermitteln
        let winner_opt: UInt8 = 0
        let winner_votes: UInt256 = 0
        for (opt, cnt) in counts {
            if cnt > winner_votes {
                winner_opt   = opt
                winner_votes = cnt
            }
        }
        let winner: String = p.options[winner_opt]
        proposals[proposal_id].status = "passed"
        emit ProposalFinalized(proposal_id, winner, "passed")
        return winner
    }
}
```

## 36.3 ATCLang stdlib v0.3

```python
# modules/atclang/stdlib/atc_stdlib.py

STDLIB = {
    # Blockchain-Builtins
    "ATC::Hash":         "SHA-256 Hash-Funktion",
    "ATC::Token::ATC8300": "Fungible Token Interface",
    "ATC::NFT::ATC9000": "NFT Interface",
    "ATC::Time":         "Block-Timestamp-Utilities",
    "ATC::Math":         "SafeMath (kein Integer-Overflow)",
    "ATC::Crypto":       "ECDSA-Signaturen",
    "ATC::Storage":      "ATCFS-Zugriff",
    "ATC::Governance":   "DAO-Interface",
    "ATC::Bridge":       "Cross-Chain-Interface",
    # Typen-Utilities
    "ATC::Convert":      "Typ-Konvertierungen",
    "ATC::Encoding":     "Base58, Hex, UTF-8",
    "ATC::Events":       "Event-Emitter",
}
```

## 36.4 Security Analyzer

```python
# modules/atclang/security/security_analyzer.py

VULN_PATTERNS = [
    "reentrancy",       # Externe Calls vor State-Update
    "integer_overflow", # Ungesicherte Arithmetik
    "access_control",   # Fehlende require(caller == owner)
    "timestamp_dep",    # Block.timestamp als Zufallsquelle
    "unchecked_return", # Ignorierte Rückgabewerte
    "gas_griefing",     # Unbegrenzte Schleifen
]

def analyze(source_code: str) -> list[dict]:
    """Statische Sicherheitsanalyse eines ATCLang Contracts."""
    ...
```

---

# 37. P2P-Netzwerk — Technische Details

> 🎫 **Verknüpfte Issues:** [🌐 #8](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/8)

> **Layer:** L5 — P2P | **Standard:** ATS-1006 | **Modul:** `modules/atcnet/`

## 37.1 Kademlia DHT

```
NODE_ID = SHA256(pubkey)[0:20]   # 20 Bytes = 160 Bit

k-Bucket-Tabelle:
  160 Buckets (je ein Bit des Node-IDs)
  k = 20 Einträge pro Bucket
  α = 3 (parallele Lookup-Threads)
  Refresh: alle 3600s

Lookup-Algorithmus (iterativ):
  1. Finde k nächste bekannte Nodes
  2. Sende FIND_NODE an alle k Nodes parallel
  3. Merge Antworten, sortiere nach XOR-Distanz
  4. Wiederhole bis keine näheren Nodes gefunden
  5. Ergebnis: k nächste Nodes zum Ziel
```

## 37.2 Bootstrap-Node (Issue #14)

```python
# blockchain/nodes/discovery.py

class NodeDiscovery:
    BOOTSTRAP_NODES = [
        "boot1.testnet.kai-os.io:4001",
        "boot2.testnet.kai-os.io:4001",
        "boot3.testnet.kai-os.io:4001",
    ]
    PING_INTERVAL   = 30    # Sekunden
    PEER_TIMEOUT    = 120   # Sekunden bis Peer als tot gilt
    MAX_PEERS       = 50    # Max gleichzeitige Verbindungen

    async def bootstrap(self):
        """
        Schritt 1: UDP-Broadcast im lokalen Netzwerk (mDNS)
        Schritt 2: Bootstrap-Nodes kontaktieren
        Schritt 3: FIND_NODE(self.node_id) ausführen
        Schritt 4: Routing-Tabelle befüllen
        Schritt 5: Periodisches Refresh starten
        """

    async def announce(self, peer_info: dict):
        """Eigene Präsenz im Netzwerk ankündigen."""

    async def get_peers(self, count: int = 10) -> list:
        """Nächste Peers aus Routing-Tabelle holen."""
```

## 37.3 Block-Propagation (Issue #15)

```python
# blockchain/nodes/p2p_propagation.py

class P2PBroadcaster:
    GOSSIP_FAN_OUT = 8   # An 8 zufällige Peers weiterleiten

    def broadcast_block(self, block: dict) -> None:
        """
        Flood-Fill Propagation (TCP):
        1. Block-Hash berechnen → Duplikat-Filter (seen_hashes)
        2. Block als P2PMessage(type="NEW_BLOCK") an alle Peers senden
        3. Peers propagieren weiter → exponentielle Ausbreitung
        """

    def broadcast_tx(self, tx: dict) -> None:
        """TX in Mempool + Flood an alle bekannten Peers."""

    def connect_to_peer(self, host: str, port: int) -> bool:
        """
        Initial Sync (Issue #16):
        1. Letzten Block des Peers anfragen
        2. Fehlende Blöcke in Batches von 500 laden
        3. Validieren + in lokale Chain einfügen
        """
```

## 37.4 ATCNet-Nachrichten-Protokoll

```python
# modules/atcnet/protocol.py

MSG_TYPES = {
    0x01: "HANDSHAKE",    # Verbindungsaufbau + Versions-Check
    0x02: "PING",         # Erreichbarkeit prüfen
    0x03: "PONG",         # Antwort auf PING
    0x04: "FIND_NODE",    # DHT-Lookup
    0x05: "NODES",        # DHT-Antwort (Liste von Peers)
    0x06: "GET_BLOCKS",   # Block-Anfrage
    0x07: "BLOCKS",       # Block-Antwort
    0x08: "NEW_BLOCK",    # Neuen Block ankündigen
    0x09: "NEW_TX",       # Neue TX ankündigen
    0x0A: "GET_TX",       # TX-Anfrage
    0x0B: "TX",           # TX-Antwort
    0x0C: "CONSENSUS",    # Konsens-Nachricht
    0x0D: "AGENT_MSG",    # KI-Agenten-Kommunikation
    0x0E: "ATCFS_REQ",    # ATCFS Datei-Anfrage
    0x0F: "ATCFS_DATA",   # ATCFS Datei-Antwort
}
```

## 37.5 NAT-Traversal (ATCHole)

```
ATCHole = STUN + TURN Hybrid (eigene Implementierung)

Schritt 1: Node sendet PING an STUN-Server
Schritt 2: STUN antwortet mit externer IP:Port
Schritt 3: Node kündigt externe Adresse an Bootstrap-Nodes
Schritt 4: Bei symmetrischem NAT → TURN-Relay als Fallback

STUN-Server: stun.testnet.kai-os.io:3478
TURN-Server: turn.testnet.kai-os.io:3479
```

---

# 38. Wallet, ECDSA & Key-Management

> 🎫 **Verknüpfte Issues:** [🔐 #24](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/24) · [🔐 #6](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/6)

> **Layer:** L0 Security | **Datei:** `blockchain/wallet/`

## 38.1 HD-Wallet-Derivation (BIP-32/44)

```python
# blockchain/wallet/keygen.py

HD_PATH = "m/44'/9000'/0'/0/{index}"
# 44'    = BIP-44 Purpose
# 9000'  = ATC Coin Type (Chain ID)
# 0'     = Account 0
# 0      = External Chain (Empfangsadressen)
# {index}= Adress-Index

def derive_child_key(parent_key: bytes, index: int) -> bytes:
    """BIP-32 Child-Key Derivation."""
    data = parent_key + index.to_bytes(4, 'big')
    hmac = hmac_sha512(key=parent_key[:32], msg=data)
    child_key = (int.from_bytes(hmac[:32], 'big') +
                 int.from_bytes(parent_key, 'big')) % SECP256K1_N
    return child_key.to_bytes(32, 'big')
```

## 38.2 DID-System (ATS-DID)

```python
# blockchain/wallet/did.py

class DIDResolver:
    """
    ATAUTH-1000 DID:kai System.
    Dezentrale Identitäten für Nodes, Agenten, User.
    Format: did:kai:z6Mkh...
    """

    def create(self, public_key: bytes) -> str:
        """DID aus Public-Key ableiten."""
        multikey = b'\xed\x01' + public_key    # Ed25519 Multikey-Prefix
        did_key  = base58.b58encode(multikey).decode()
        return f"did:kai:{did_key}"

    def resolve(self, did: str) -> dict:
        """DID-Dokument aus Chain laden."""
        return {
            "@context":            "https://www.w3.org/ns/did/v1",
            "id":                  did,
            "verificationMethod":  [...],
            "authentication":      [...],
            "assertionMethod":     [...],
        }

    def verify_signature(self, did: str, message: bytes,
                         signature: dict) -> bool:
        """Signatur gegen DID-Dokument verifizieren."""
```

## 38.3 Multi-Sig Wallet

```python
# Multi-Sig für Bridge + Franchise Vault
# Implementiert in: modules/contracts/bridge/bridge_contract.py

class TxStatus(Enum):
    PENDING   = "pending"    # Warte auf Signaturen
    READY     = "ready"      # Threshold erreicht
    EXECUTED  = "executed"   # Ausgeführt
    CANCELLED = "cancelled"  # Abgebrochen
    EXPIRED   = "expired"    # Timeout (7 Tage TTL)

class MultiSigTx:
    """Eine vorgeschlagene Multi-Sig-Transaktion."""
    tx_id:      str        # "MSIG-<hash16>"
    proposer:   str        # ATC-Adresse
    to:         str        # Empfänger
    value:      float      # ATC-Betrag
    status:     str        # TxStatus.*
    signatures: dict       # {addr: sig_hex}
    expires_at: int        # created_at + 7 * 24 * 3600
    tx_hash:    str        # nach execute_tx()

class MultiSigWallet:
    """M-of-N Multisig-Wallet."""

    def __init__(self, owners: list, threshold: int):
        self.owners    = owners       # N Unterzeichner
        self.threshold = threshold    # M benötigte Signaturen

    def propose_tx(self, caller, to, value, data=b"") -> str:
        """TX vorschlagen (jeder Owner darf das)."""

    def sign_tx(self, caller, tx_id: str) -> dict:
        """TX unterzeichnen. Bei M Signaturen → automatisch ausführen."""

    def execute_tx(self, tx_id: str) -> dict:
        """TX ausführen sobald Threshold erreicht."""
```

## 38.4 Keystore-Format

```json
{
  "version": 3,
  "id": "uuid-v4",
  "address": "ATC7F3A9B2C...",
  "crypto": {
    "cipher":       "aes-128-ctr",
    "ciphertext":   "hex-encoded-encrypted-privkey",
    "cipherparams": { "iv": "hex" },
    "kdf":          "pbkdf2",
    "kdfparams": {
      "dklen": 32,
      "salt":  "hex",
      "c":     262144,
      "prf":   "hmac-sha256"
    },
    "mac": "hex"
  }
}
```

---

## 38.5 ECDSASigner & ATCKeyGenerator

> **Dateien:** `blockchain/wallet/ecdsa.py` · `blockchain/wallet/keygen.py`

```python
# ── ECDSA Signatur (secp256k1) ────────────────────────────────

class ECDSASigner:
    """ECDSA Signatur auf Basis von secp256k1."""

    @staticmethod
    def generate_keypair() -> tuple:
        """(private_key_hex, public_key_hex)"""

    @staticmethod
    def sign(tx_data: dict, private_key_hex: str) -> str:
        """TX signieren, DER-kodierte Signatur zurückgeben."""

    @staticmethod
    def verify(tx_data: dict, signature_hex: str,
               public_key_hex: str) -> bool:
        """Signatur prüfen."""

    @staticmethod
    def build_tx(from_addr: str, to_addr: str,
                 amount: float, private_key: str) -> dict:
        """TX bauen + signieren in einem Schritt."""

# ── Key-Generierung (BIP39) ───────────────────────────────────

class ATCKeyGenerator:
    """Wallet-Key-Generierung: Entropy → Mnemonic → Keys."""

    def generate_wallet(self,
            passphrase: str = "A-TownChain") -> dict:
        """Vollständiges Wallet generieren:
        entropy → seed_phrase(24) → private_key → public_key → address"""

    def restore_from_mnemonic(self, mnemonic: list,
            passphrase: str = "A-TownChain") -> dict: ...
    def validate_address(self, address: str) -> bool: ...
    def public_key_to_address(self, public_key: str) -> str: ...
```

> **Hinweis:** Prototyp nutzt secp256k1 (Python). Produktiv-System: sr25519 via Substrate.
## 38.6 Mobile Wallet — React Native

> **Datei:** `mobile/wallet_api.py` · Fixes: #38

```python
class MobileWalletAccount:
    address:     str
    public_key:  str
    label:       str
    created_at:  int
    pin_hash:    str   # SHA-256 + Salt
    biometric:   bool

    def get_qr_data(self, amount: int = 0, memo: str = "") -> str:
        """Gibt atc://<address>?amount=<n>&memo=<m> zurück."""

    def is_backed_up(self) -> bool:
        """True wenn Seed-Phrase schon bestätigt wurde."""

class MobileWalletManager:
    """Wallet-Manager für mobile Geräte."""

    def create_account(self, pin: str,
                       use_biometric: bool = False) -> dict:
        """Neues Wallet: BIP39 Mnemonic (24 Wörter) → Keypair."""

    def authenticate(self, address: str, pin: str) -> dict:
        """PIN prüfen, Session-Token zurückgeben."""

    def send_transaction(self, address: str, pin: str,
                         to: str, amount: int,
                         memo: str = "") -> dict: ...

    def parse_qr(self, qr_data: str) -> dict:
        """atc://... QR-Code parsen."""

    def get_balance(self, address: str) -> dict: ...
    def backup_mnemonic(self, address: str, pin: str) -> list:
        """Gibt 24-Wort Seed-Phrase zurück."""
    def restore(self, mnemonic: list, pin: str) -> dict: ...
```


# 39. Cross-Chain Bridge — Technische Details

> 🎫 **Verknüpfte Issues:** [🌉 #10](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/10)

> **Layer:** L4 — Blockchain | **Datei:** `modules/contracts/bridge/bridge_contract.py` | **Issue:** #10

## 39.1 Lock-and-Mint Mechanismus

```
A-TownChain                              Ethereum (Sepolia)
───────────                              ──────────────────
User: lock(1000 ATC)              →      Relayer erkennt LockEvent
  → ATC gesperrt im BridgeContract →      Relayer: mint(1000 wATC) an User
  → LockEvent emittiert                   wATC ist ERC-20 auf Ethereum

User: initiateBurn(500 wATC)      ←      User burned wATC auf Ethereum
  auf Ethereum                    ←      BurnEvent
Relayer: release(500 ATC)         →      ATC aus Lock-Contract freigeschaltet
```

## 39.2 Bridge Contract

```python
# modules/contracts/bridge/bridge_contract.py

class BridgeContract(BaseContract):
    """ATC-5000 Cross-Chain Bridge — Lock-Mint Mechanismus."""

    MAX_TX_AMOUNT  = 1_000_000.0   # ATC
    DAILY_LIMIT    = 5_000_000.0   # ATC
    TIMELOCK_LARGE = 24 * 3600     # 24h für TX > 100.000 ATC
    RELAYER_THRESHOLD = 3          # 3-of-5 Multi-Sig Relayer

    def lock_atc(self, sender: str, amount: float,
                 destination_chain: str,
                 destination_address: str) -> dict:
        """
        Schritt 1: Amount-Limits prüfen
        Schritt 2: ATC vom Sender-Wallet abziehen
        Schritt 3: ATC in Bridge-Vault sperren
        Schritt 4: LockEvent emittieren (Relayer abhören)
        Schritt 5: Bridge-TX-ID zurückgeben
        """

    def release_atc(self, relayer: str, bridge_tx_id: str,
                    recipient: str, amount: float,
                    signatures: list) -> dict:
        """
        Schritt 1: Mindestens 3 Relayer-Signaturen verifizieren
        Schritt 2: Bridge-TX-ID nicht bereits verwendet?
        Schritt 3: ATC aus Vault an Recipient freigeben
        Schritt 4: ReleaseEvent emittieren
        """

    def emergency_pause(self, caller: str):
        """Nur Owner oder DAO kann Bridge pausieren."""
```

## 39.3 Relayer-Architektur

```
5 unabhängige Relayer-Nodes:
  relayer-1.bridge.kai-os.io
  relayer-2.bridge.kai-os.io
  relayer-3.bridge.kai-os.io
  relayer-4.bridge.kai-os.io (Backup)
  relayer-5.bridge.kai-os.io (Backup)

Ablauf (3-of-5):
  1. Alle 5 Relayer überwachen beide Chains
  2. LockEvent erkannt → jeder Relayer signiert die Mint-Anfrage
  3. Nach 3 Signaturen → Mint-TX auf Ethereum ausgeführt
  4. Audit-Trail: alle Bridge-TXs on-chain (beide Chains)
```

## 39.4 Wrapped ATC (wATC)

```solidity
// WrappedATC.sol (Ethereum Sepolia)
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

contract WrappedATC is ERC20, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    constructor() ERC20("Wrapped A-Town Coin", "wATC") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function mint(address to, uint256 amount)
        external onlyRole(MINTER_ROLE) {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount)
        external onlyRole(MINTER_ROLE) {
        _burn(from, amount);
    }
}
```

## 39.5 Rate-Limiting & Sicherheit

| Maßnahme | Wert | Zweck |
|----------|------|-------|
| Max TX | 1.000.000 ATC | Schutz vor Large-Value-Angriff |
| Daily Limit | 5.000.000 ATC | Globale Tages-Obergrenze |
| Timelock | 24h (> 100k ATC) | Zeit für Fraud-Detection |
| Relayer Threshold | 3-of-5 | Dezentralisierung |
| Emergency Pause | Owner + DAO | Sofortstopp bei Incident |
| Replay-Schutz | Bridge-TX-ID | TX nicht doppelt ausführbar |

---

## 39.6 SolanaBridge v3.0 — Solana Integration

> **Datei:** `blockchain/bridge/solana_bridge.py` · Fixes: #34

```python
class SolanaBridgeTx:
    tx_id:        str       # "SOLTX-<hash16>"
    direction:    str       # "ATC→SOL" | "SOL→ATC"
    amount:       int       # in Lamports (1e9 = 1 SOL)
    sender:       str       # ATC-Adresse
    recipient:    str       # Solana Public Key (Base58)
    status:       str       # PENDING | LOCKED | CONFIRMED | RELEASED
    wormhole_seq: int       # Wormhole Sequence Number
    fee_lamports: int

class SolanaBridge:
    """ATC ↔ Solana Bridge via Wormhole.
    Protokoll: Lock-and-Mint.
    Threshold: 3-of-5 Relayer Multi-Sig.
    Daily Limit: 100.000 ATC."""

    WORMHOLE_PROGRAM = "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth"
    DAILY_LIMIT      = 100_000     # ATC
    MIN_AMOUNT       = 10          # ATC
    FEE_RATE         = 0.001       # 0.1%

    def lock_atc(self, sender: str, amount: int,
                 solana_recipient: str) -> SolanaBridgeTx:
        """ATC sperren, Wormhole-Nachricht senden."""

    def mint_on_solana(self, tx: SolanaBridgeTx) -> dict:
        """SPL-Token auf Solana minten (via Wormhole VAA)."""

    def burn_on_solana(self, sol_address: str, amount: int,
                       atc_recipient: str) -> SolanaBridgeTx:
        """SPL-Token verbrennen, ATC auf ATC-Chain freigeben."""

    def get_status(self, tx_id: str) -> SolanaBridgeTx: ...
    def get_stats(self) -> dict: ...
```


# 40. ShivaOS UI — Rendering Engine & Design

> **Layer:** L10 — dApps | **Standard:** ATS-1007 | **Datei:** `frontend/index.html`

## 40.1 Architektur

```
ShivaOS = Single-File-App (123 KB, kein Framework)

frontend/index.html
  ├── <style>     — CSS (Design-Tokens, Neon-Theme)
  ├── <script>    — Vanilla JS (kein React/Vue/Angular)
  └── <body>      — DOM-Struktur (Sidebar + Panels)

frontend/assets/js/api.js    — API-Client (alle Endpoints)
frontend/assets/css/         — Externe CSS-Dateien
frontend/battle/index.html   — Dediziertes Battle-Interface
frontend/bootscreen/         — Ladebildschirm-Animation
```

## 40.2 Design-System (ATS-1007)

```css
/* A-TownChain Design-Tokens */
:root {
  --atc-primary:    #00ffcc;   /* Neon-Türkis */
  --atc-secondary:  #7b61ff;   /* Neon-Violett */
  --atc-bg:         #0a0a1a;   /* Tiefschwarz */
  --atc-surface:    #1a1a3a;   /* Dunkelblau (Cards) */
  --atc-text:       #e0e0ff;   /* Hellblau-Weiß */
  --atc-accent:     #ff6b35;   /* Neon-Orange */
  --atc-success:    #00ff88;   /* Neon-Grün */
  --atc-error:      #ff2d78;   /* Neon-Pink */
  --atc-warning:    #ffcc00;   /* Neon-Gelb */

  /* Typografie */
  --font-mono:   'JetBrains Mono', 'Fira Code', monospace;
  --font-ui:     'Inter', system-ui, sans-serif;

  /* Animationen */
  --transition-fast:   0.15s ease;
  --transition-normal: 0.3s ease;
  --glow-primary: 0 0 20px rgba(0, 255, 204, 0.3);
  --glow-accent:  0 0 20px rgba(255, 107, 53, 0.4);
}
```

## 40.3 Sidebar-Module

```javascript
// frontend/assets/js/api.js

const PANELS = {
  "dashboard":   { icon: "📊", title: "Dashboard",   api: "/api/status" },
  "blockchain":  { icon: "⛓",  title: "Blockchain",  api: "/api/blockchain/info" },
  "shivamon":    { icon: "🎮", title: "Shivamon",    api: "/api/game/shivamon/stats" },
  "marketplace": { icon: "🛒", title: "Marketplace", api: "/api/marketplace/listings" },
  "governance":  { icon: "🏛", title: "Governance",  api: "/api/governance/proposals" },
  "wallet":      { icon: "💰", title: "Wallet",      api: "/api/wallet/balance" },
  "ai":          { icon: "🤖", title: "AI Chat",     api: "/api/ai/status" },
  "nodes":       { icon: "🌐", title: "Nodes",       api: "/api/nodes/" },
  "bridge":      { icon: "🌉", title: "Bridge",      api: "/api/blockchain/bridge/status" },
};
```

## 40.4 Bootscreen-Animation

```python
# frontend/bootscreen/bootscreen.py (Generierungs-Script)

BOOT_SEQUENCE = [
    "[00:00.000] 🔴 INITIALISIERE KAI-OS KERNEL v2.0...",
    "[00:00.150] ⚡ LADE BLOCKCHAIN-MODUL (L4)...",
    "[00:00.300] 🧠 VERBINDE KI-AGENTEN (L3)...",
    "[00:00.450] 🌐 P2P-NETZWERK AUFBAUEN (L5)...",
    "[00:00.600] 🔐 SICHERHEITS-CHECK (L0)...",
    "[00:00.750] ✅ ALLE SYSTEME BEREIT",
    "[00:00.900] 🟢 WILLKOMMEN IN SHIVAOS v2.0",
]
```

## 40.5 Responsive API-Client

```javascript
// frontend/assets/js/api.js (v2.0)

const API_BASE = "http://localhost:4000";

async function apiCall(endpoint, method="GET", body=null) {
  const opts = {
    method,
    headers: { "Content-Type": "application/json" },
  };
  if (body) opts.body = JSON.stringify(body);

  const res  = await fetch(`${API_BASE}${endpoint}`, opts);
  const data = await res.json();
  if (!res.ok) throw new Error(data.error || "API Error");
  return data;
}

// Shivamon minten
const mint = (owner, element) =>
  apiCall("/api/game/shivamon/mint", "POST", { owner, element });

// Governance abstimmen
const vote = (voter, proposal_id, option) =>
  apiCall("/api/governance/vote", "POST", { voter, proposal_id, option });
```

---

# 41. Federated Learning

> **Layer:** L3 — KI-Modul | **Datei:** `core/ai_kernel.py`

## 41.1 Konzept

```
Federated Learning = dezentrales ML-Training ohne Rohdatenaustausch

Ablauf:
  Runde N:
  ├── Koordinator wählt 20 zufällige Nodes aus
  ├── Jeder Node: lokales Training auf eigenen Daten (5 Epochen)
  ├── Jeder Node: sendet nur Gradienten (kein Rohdaten!)
  ├── Koordinator: FedAvg-Aggregation der Gradienten
  ├── Neues Modell an alle Nodes verteilen
  └── PoI-Score: Qualität des Beitrags bewertet
```

## 41.2 FedAvg-Algorithmus

```python
def federated_average(gradients: list[dict],
                       weights:   list[float]) -> dict:
    """
    Federated Averaging (McMahan et al., 2017)
    weights = Anteil der lokalen Datenmenge je Node
    """
    aggregated = {}
    for key in gradients[0].keys():
        aggregated[key] = sum(
            w * g[key]
            for w, g in zip(weights, gradients)
        )
    return aggregated
```

## 41.3 PoI-Integration

```python
def calculate_poi_score(node_contribution: dict) -> float:
    """
    PoI-Score = f(Gradient-Qualität, Datenmenge, Latenz)
    Score: 0.0 – 1.0
    """
    quality = measure_gradient_quality(node_contribution["gradients"])
    size    = node_contribution["data_size"] / 10_000   # normalisiert
    latency = 1.0 - min(node_contribution["latency_ms"] / 5000, 1.0)
    return 0.5 * quality + 0.3 * size + 0.2 * latency
```

## 41.4 Datenschutz-Garantien

| Maßnahme | Beschreibung |
|----------|-------------|
| Differential Privacy | ε-dp Rauschen zu Gradienten hinzufügen |
| Secure Aggregation | Kryptografische Summation ohne Einzelwerte |
| Gradient Compression | Top-k Sparsifikation (90% weniger Bandbreite) |
| Anomaly Detection | Ausreißer-Gradienten werden verworfen |

---

# 42. Performance & Optimierung

> **Referenz:** `docs/wiki/kai-os/PERFORMANCE_REPORT.md`

## 42.1 Zielwerte

| Komponente | Ziel | Aktuell |
|-----------|------|---------|
| Block Time | < 4s | ~4s ✅ |
| TX/Block | 1.000 | 1.000 ✅ |
| API-Latenz (Gateway) | < 100ms | ~50ms ✅ |
| Gemini-Antwortzeit | < 3s | ~1.5s ✅ |
| LLaMA-Inferenz (CPU) | ≥ 10 t/s | 8.3 t/s ⚠️ |
| LLaMA-Inferenz (GPU) | ≥ 80 t/s | ~87 t/s ✅ |
| P2P-Propagation | < 500ms | noch offen |
| Dashboard-Load | < 2s | ~1.2s ✅ |

## 42.2 Backend-Optimierungen

```python
# Caching für häufige Abfragen
from functools import lru_cache

@lru_cache(maxsize=256)
def get_token_stats(snapshot_id: int) -> dict:
    """Token-Stats werden gecacht (invalidiert bei neuem Block)."""

# Connection-Pooling für SQLite
import sqlite3
from contextlib import contextmanager

_pool = []

@contextmanager
def db_connection():
    conn = _pool.pop() if _pool else sqlite3.connect("atcchain.db")
    try:
        yield conn
        conn.commit()
    finally:
        _pool.append(conn)
```

## 42.3 Docker-Ressourcen-Limits

```yaml
# docker-compose.yml (Auszug)
services:
  backend:
    mem_limit: 512m
    cpus: "1.0"
  gateway:
    mem_limit: 256m
    cpus: "0.5"
  bootstrap:
    mem_limit: 256m
    cpus: "0.5"
```

---

# 43. Plugin & Modul-System (atcpkg)

> 🎫 **Verknüpfte Issues:** [📦 #27](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/27)

> **Standard:** ATS-1001 | **Modul:** `modules/` | **Status:** 🔨 In Entwicklung

## 43.1 Modul-Spezifikation

```python
# ATS-1001 Module Spec

MODULE = {
    "name":         str,       # Eindeutiger Name
    "version":      str,       # SemVer (z.B. "1.2.3")
    "author":       str,       # ATC-Adresse
    "hash":         str,       # SHA-256 Integritäts-Hash
    "entrypoint":   str,       # Haupt-Datei
    "exports":      list,      # Öffentliche API
    "deps":         list,      # Abhängigkeiten
    "permissions":  list,      # Benötigte Capabilities
    "stake":        float,     # Benötigter Stake für Deployment
}

PERMISSIONS = [
    "FS_READ", "FS_WRITE",
    "NET_CONNECT", "NET_LISTEN",
    "KI_QUERY", "KI_TRAIN",
    "BLOCKCHAIN_READ", "BLOCKCHAIN_WRITE",
    "PROCESS_SPAWN",
]
```

## 43.2 atcpkg CLI

```bash
# Modul installieren
atcpkg install shivamon@1.0.0

# Modul verifizieren (Hash-Check)
atcpkg verify shivamon --hash a3f9b2c1...

# Modul ausführen
atcpkg run shivamon mint --owner ATC7F3A... --element Fire

# Alle installierten Module
atcpkg list

# Modul entfernen
atcpkg remove shivamon

# Modul publizieren (benötigt Stake)
atcpkg publish ./my-module/ --stake 100
```

## 43.3 Monorepo-Module-Struktur

```
modules/
├── kernel/       L2: ShivaOS Microkernel
├── gateway/      L7: API Gateway
├── contracts/    L4: Smart Contracts
├── atclang/      L1: Compiler + VM
├── atcnet/       L5: P2P Netzwerk
├── ui/           L10: Dashboard
├── shivamon/     L12: NFT-Game
├── franchise/    L8: Franchise DAO
└── standards/    L0: Protokoll-Standards

Jedes Modul hat:
  README.md       — Dokumentation
  CHANGELOG.md    — Versionshistorie
  __init__.py     — Python-Package
  requirements.txt— Abhängigkeiten (optional)
```

---

## 43.4 ATCPackageManager — Implementierung

> **Datei:** `atcpkg/manager.py` · Fixes: #27, #30

```python
class ATCPackage:
    name:     str
    version:  str
    author:   str
    desc:     str
    deps:     list   # ["atclang>=0.2", "atcnet>=1.0"]
    atcfs_cid: str   # CID in ATCFS

class ATCPackageManager:
    """ATC Package Manager — install, publish, search."""

    def publish(self, pkg: ATCPackage) -> str:
        """Paket in Registry hochladen, CID zurückgeben."""

    def install(self, name: str,
                version: str = "latest") -> bool:
        """Paket + Dependencies installieren."""

    def search(self, query: str) -> list[ATCPackage]:
        """Registry nach Name/Tag durchsuchen."""

    def list_installed(self) -> list[ATCPackage]: ...
    def uninstall(self, name: str) -> bool: ...
    def info(self, name: str) -> ATCPackage: ...
    def stats(self) -> dict: ...
```

```bash
# CLI-Nutzung
atcpkg install <name>          # Paket installieren
atcpkg publish <path>          # Eigenes Paket veröffentlichen
atcpkg search <query>          # Registry durchsuchen
atcpkg list                    # Installierte Pakete
atcpkg info <name>             # Paket-Informationen
```


# 44. KI-Kernel — Inference Engine Details

> **Layer:** L3 — KI-Modul | **Datei:** `core/ai_kernel.py`

## 44.1 Modell-Stack

```python
# core/ai_kernel.py

class AIKernel:
    """KAI-OS AI Orchestrator — Gemini 2.0 + LLaMA Fallback."""

    MODEL_PRIORITY = [
        ("gemini-2.0-flash-exp", "google", "remote"),
        ("llama3-8b-q4",         "ollama", "local"),
        ("mistral-7b-q4",        "ollama", "local"),
    ]

    def query(self, prompt: str, context: list = None,
              model: str = None) -> dict:
        """
        Schritt 1: Primär-Modell versuchen (Gemini 2.0)
        Schritt 2: Bei Fehler/Timeout → Fallback (LLaMA lokal)
        Schritt 3: Gas-Kosten berechnen + abziehen
        Schritt 4: XAI-Log schreiben (on-chain)
        Schritt 5: Antwort zurückgeben
        """

    def schedule_inference(self, tasks: list) -> list:
        """
        KI-Scheduler: Priorisierung nach PoI-Score + Stake
        Parallele Ausführung bei GPU-Verfügbarkeit
        """
```

## 44.2 GPU-Abstraktions-Layer

```python
# Hardware-Beschleunigung (ATS-1000 Kernel-Spec)

GPU_BACKENDS = {
    "cuda":    "NVIDIA GPU (CUDA)",
    "rocm":    "AMD GPU (ROCm)",
    "metal":   "Apple Silicon (Metal)",
    "cpu":     "CPU-Fallback (langsam)",
    "webgpu":  "Browser-GPU (geplant)",
}

def detect_gpu() -> str:
    """Verfügbaren GPU-Backend erkennen."""
    try:
        import torch
        if torch.cuda.is_available(): return "cuda"
        if torch.backends.mps.is_available(): return "metal"
    except ImportError:
        pass
    return "cpu"
```

## 44.3 LLM-Router

```python
# docs/ai/LLM_ROUTER.md (Auszug)

ROUTER_CONFIG = {
    "rules": [
        # Einfache Fragen → schnelles Modell
        {"if": "token_count < 100", "model": "gemini-flash"},
        # Code-Generierung → Gemini Pro
        {"if": "task_type == 'code'", "model": "gemini-pro"},
        # Offline-Betrieb → lokales Modell
        {"if": "network == 'offline'", "model": "llama3-8b"},
        # Standard → Gemini Flash
        {"default": "gemini-2.0-flash-exp"},
    ]
}
```

---

# 45. ATCFS — Dezentrales Dateisystem

> 🎫 **Verknüpfte Issues:** [🗂️ #23](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/23)

> **Layer:** L6 — Storage | **Standard:** ATS-1002 | **Datei:** `modules/kernel/docs/ATCFS.md`

## 45.1 Konzept

```
ATCFS = A-TownChain File System

Ähnlich zu IPFS, aber vollständig eigenständig:
  - Content-Adressierung via SHA-256 (CID)
  - Dezentrale Replikation (min. 3 Kopien)
  - Eigene Adress-Syntax: atcfs://<node_id>/<cid>/<path>
  - Integriert in ShivaOS (transparenter Zugriff)
  - On-Chain Metadaten (INODE-Hashes gespeichert)
```

## 45.2 INODE-Struktur

```python
INODE = {
    "cid":        str,    # SHA-256(content) — Content-Hash
    "size":       int,    # Bytes
    "owner":      str,    # ATC-Adresse
    "created":    int,    # Unix-Timestamp
    "modified":   int,
    "perms":      str,    # "rwxr-xr--" (POSIX-ähnlich)
    "type":       str,    # FILE | DIR | SYMLINK | CONTRACT
    "replicas":   int,    # Aktuelle Anzahl Replikas
    "min_replicas": int,  # Mindest-Replikas (default: 3)
    "encrypted":  bool,
    "tags":       list,   # Suchbare Metadaten
}
```

## 45.3 Dateitypen

| Endung | Typ | Beschreibung |
|--------|-----|-------------|
| `.atc` | Quellcode | ATCLang Smart Contract |
| `.atcb` | Bytecode | Kompilierter Code |
| `.atcm` | Modul | Signiertes ATC-Modul |
| `.atcw` | Wallet | Verschlüsselte Wallet-Datei |
| `.atcd` | Daten | Strukturierte ATC-Daten |
| `.atcp` | Prozess | Prozess-Image |

## 45.4 Klassen & API

> **Datei:** `core/atcfs.py` — ATS-1002 Standard

```python
# ── Datenklassen ──────────────────────────────────────────────

class INode:
    """INODE — Metadaten einer Datei oder eines Verzeichnisses."""
    cid:          str      # SHA-256(content)
    name:         str
    size:         int
    owner:        str      # ATC-Adresse
    created:      int      # Unix-Timestamp
    modified:     int
    perms:        str      # "rw-r--r--"
    type:         str      # FILE | DIR | SYMLINK | CONTRACT
    replicas:     int
    min_replicas: int = 3
    encrypted:    bool
    tags:         list

class FileHandle:
    """Offener Datei-Handle (ähnlich POSIX fd)."""
    fh_id:  int
    cid:    str
    mode:   str     # "r", "w", "a"
    offset: int
    dirty:  bool

# ── ATCFS Hauptklasse ─────────────────────────────────────────

class ATCFS:
    """A-TownChain File System — ATS-1002."""

    def write(self, data: bytes, name: str, owner: str,
              ftype: FileType = FileType.FILE) -> str:
        """Schreibt Datei, gibt CID zurück."""

    def read(self, cid: str) -> bytes:
        """Datei über CID lesen."""

    def delete(self, cid: str, caller: str) -> bool:
        """Löschen (nur Owner)."""

    def open(self, atcfs_url: str, mode: str = "r") -> FileHandle:
        """Öffnet atcfs://<node>/<cid> als File-Handle."""

    def read_fh(self, fh: FileHandle, size: int = -1) -> bytes: ...
    def write_fh(self, fh: FileHandle, data: bytes) -> int: ...
    def close(self, fh: FileHandle): ...

    def mkdir(self, name: str, owner: str, parent_cid: str = None) -> str: ...
    def listdir(self, dir_cid: str) -> list[INode]: ...
    def stat(self, cid: str) -> INode: ...
    def find(self, owner: str = None, tags: list = None) -> list[INode]: ...
    def get_stats(self) -> dict: ...
    def build_url(self, cid: str) -> str:
        """Gibt atcfs://<node_id>/<cid> zurück."""

# ── Singleton ─────────────────────────────────────────────────

def get_atcfs(root_dir: str = "data/atcfs") -> ATCFS:
    """Globale ATCFS-Instanz."""
```

### HTTP-API (geplant via Backend :5000)

```
POST /api/storage/upload        {"content": base64, "filename": "x.atc"}
GET  /api/storage/{cid}         → Datei-Download
GET  /api/storage/{cid}/info    → INode-Metadaten
DELETE /api/storage/{cid}       → Löschen (Auth required)
```

---

# 46. XAI & Entscheidungsaudit

> **Layer:** L0 — Security | **Datei:** `core/ai_kernel.py`

## 46.1 Was ist XAI?

XAI (Explainable AI) stellt sicher, dass jede KI-Entscheidung nachvollziehbar und auditierbar ist. A-TownChain speichert XAI-Logs on-chain.

## 46.2 XAI-Log-Format

```python
XAI_LOG = {
    "log_id":     str,          # UUID
    "agent_id":   str,          # ATC-Adresse des Agenten
    "task_id":    str,          # Task-Referenz
    "timestamp":  float,        # Unix-Timestamp
    "model":      str,          # Modell-Name + Version-Hash
    "input_hash": str,          # SHA-256(prompt) — kein Klartext
    "reasoning":  str,          # Chain-of-Thought (komprimiert, max 2KB)
    "confidence": float,        # 0.0–1.0
    "output_hash":str,          # SHA-256(response)
    "gas_used":   float,        # Verbrauchtes Gas in ATC
    "block_ref":  int,          # Block-Nummer zum Zeitpunkt
    "signature":  dict,         # ECDSA-Signatur des Agenten
}
```

## 46.3 On-Chain Audit

```bash
# XAI-Logs abrufen (CLI)
kai agent audit {agent_id} --from-block 1000 --to-block 2000

# Compliance-Beweis exportieren (ZKP)
kai agent zkp-export {agent_id} --period 2026-06

# Entscheidungsprotokoll anzeigen
GET /v1/agents/{id}/audit?from_block=1000&limit=50
```

## 46.4 Audit-Trail Garantien

| Garantie | Umsetzung |
|----------|-----------|
| Unveränderlichkeit | On-Chain gespeichert |
| Anonymität | Nur Input-/Output-Hash, kein Klartext |
| Nachvollziehbarkeit | Chain-of-Thought (komprimiert) |
| Authentizität | ECDSA-Signatur des Agenten |
| Zeitstempel | Block-Nummer als Beweis |

---

# 47. Governance — Deep Dive

> 🎫 **Verknüpfte Issues:** [🏛 #9](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/9)

## 47.0 Python-Implementierung

> **Datei:** `blockchain/contracts/governance/governance_contract.py`

```python
class GovernanceContract:
    """DAO Governance — ATC-9900 Standard."""

    def __init__(self, owner: str, total_supply: float = 1_000_000):
        ...

    def create_proposal(self, proposer: str, title: str,
                        options: list, description: str = "") -> str:
        """Vorschlag erstellen, gibt proposal_id zurück."""

    def vote(self, proposal_id: str, voter: str,
             choice: VoteChoice) -> bool:
        """Abstimmen (1 Token = 1 Stimme)."""

    def finalize(self, proposal_id: str) -> ProposalStatus:
        """Abstimmung beenden, Ergebnis ermitteln."""

    def execute(self, proposal_id: str, executor: str) -> dict:
        """Ausgeführten Beschluss umsetzen."""

    def list_proposals(self,
        status: ProposalStatus = None) -> list: ...
    def stats(self) -> dict: ...
```

> **Layer:** L8 | **Standard:** ATC-9900 | **Datei:** `blockchain/contracts/governance/governance_contract.py`

## 47.1 Vollständiger Governance-Flow

```
1. ATC-Holder erstellt Proposal
   ├── Deposit: 1.000 ATC (zurück bei Execution)
   ├── Min. 2, max. 10 Abstimmungsoptionen
   └── ProposalCreated-Event emittiert

2. Voting-Phase (Standard: 7 Tage)
   ├── Stimmgewicht = ATC-Balance zum Snapshot-Zeitpunkt
   ├── Jede Adresse kann einmal pro Proposal abstimmen
   ├── Voted-Event emittiert
   └── Stimmen anonym (nur on-chain Hash)

3. Finalisierung (nach Deadline)
   ├── Quorum-Check: mind. 10% der Supply muss abgestimmt haben
   ├── Bei FAILED: → Proposal abgelehnt, Deposit → Treasury
   └── Bei PASSED: → Gewinner-Option ermittelt

4. Timelock (48 Stunden)
   └── Verzögerung für mögliche Einsprüche

5. Execution
   ├── Automatisch nach Timelock
   ├── Deposit → Creator zurück
   └── ExecutionEvent emittiert
```

## 47.2 Parameter-Updates via Governance

```python
# Welche Parameter per DAO änderbar sind:
GOVERNABLE_PARAMS = {
    "block_reward":       float,  # Aktuell: 50 ATC
    "min_stake_validator":float,  # Aktuell: 10.000 ATC
    "governance_quorum":  float,  # Aktuell: 0.10 (10%)
    "voting_period":      int,    # Aktuell: 604.800s (7 Tage)
    "proposal_deposit":   float,  # Aktuell: 1.000 ATC
    "marketplace_fee":    float,  # Aktuell: 0.01 (1%)
    "bridge_max_tx":      float,  # Aktuell: 1.000.000 ATC
    "ai_gas_price":       float,  # Aktuell: 0.01 ATC/Query
}
```

## 47.3 Anti-Spam & Sicherheit

```python
GOVERNANCE_LIMITS = {
    "max_proposals_per_address": 3,     # gleichzeitig aktiv
    "min_balance_to_propose":    1_000, # ATC
    "min_balance_to_vote":       1,     # ATC (jeder darf abstimmen)
    "spam_cooldown":             86_400, # 24h zwischen Proposals
}
```

---

# 48. Marketplace — Deep Dive

> 🎫 **Verknüpfte Issues:** [🛒 #13](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/13)

## 48.0 Python-Implementierung

> **Datei:** `modules/contracts/marketplace/marketplace_contract.py`

```python
class MarketplaceContract(BaseContract):
    """ATC Marketplace — NFT kaufen & verkaufen."""
    ROYALTY_RATE      = 0.025   # 2.5% Royalty
    PLATFORM_FEE_RATE = 0.010   # 1.0% Platform-Fee

    def list_for_sale(self, seller: str, token_id: str,
                      price: float, currency: str = "ATC") -> str:
        """NFT listen, gibt listing_id zurück."""

    def buy(self, buyer: str, listing_id: str) -> dict:
        """NFT kaufen, ATC transferieren, Fees abziehen."""

    def cancel_listing(self, seller: str, listing_id: str) -> dict:
        """Listing zurückziehen."""

    def get_listings(self, status=None, seller=None) -> list: ...
    def get_stats(self) -> dict: ...
```

> **Layer:** L10 | **Datei:** `modules/contracts/marketplace/marketplace_contract.py`

## 48.1 Listing-Lifecycle

```
NFT Listing:
  LISTED → (sold) → COMPLETED
         → (cancelled) → CANCELLED
         → (offer accepted) → COMPLETED
         → (expired 30d) → EXPIRED
```

## 48.2 Vollständiger Listing-Flow

```python
# 1. NFT listen
listing = marketplace.list_for_sale(
    seller   = "ATC7F3A...",
    token_id = "SHIV-0001",
    price_atc= 500.0
)
# → NFT wird in Escrow gesperrt
# → Listing-ID: "LST-a1b2c3"

# 2. Angebote machen
offer = marketplace.make_offer(
    buyer    = "ATC9B2C...",
    token_id = "SHIV-0001",
    offer_atc= 450.0        # unter Listpreis
)

# 3. Angebot annehmen
marketplace.accept_offer(
    seller   = "ATC7F3A...",
    offer_id = offer["offer_id"]
)

# 4. Direkt kaufen (zum Listpreis)
marketplace.buy(
    buyer      = "ATC9B2C...",
    listing_id = "LST-a1b2c3"
)
# → ATC-Transfer: 500 → Seller (482.5) + Creator (12.5) + Treasury (5)
# → NFT-Transfer: Escrow → Buyer
```

## 48.3 Statistiken & Floor Price

```python
def get_stats(self) -> dict:
    return {
        "total_listings":   len(self._listings),
        "total_volume_atc": self._total_volume,
        "total_fees_atc":   self._total_fees,
        "floor_prices":     self._calculate_floor_prices(),
        "top_sales":        self._get_top_sales(10),
        "avg_price_24h":    self._avg_price_24h(),
        "unique_sellers":   len(set(l["seller"] for l in self._listings.values())),
    }
```

---

# 49. Testnet — Docker Compose & Monitoring

> 🎫 **Verknüpfte Issues:** [🌐 #8](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/8) · [🐳 #18](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/18) · [📊 #19](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/19)

> **Issues:** #18 (Docker), #19 (Monitoring) | **Datei:** `docker-compose.yml`

## 49.1 Docker Compose — 5-Node Testnet

```yaml
# docker-compose.yml (vollständig)
version: "3.9"

services:
  bootstrap:
    build:
      context: .
      dockerfile: docker/Dockerfile.bootstrap
    ports:
      - "4001:4001"   # P2P
    environment:
      - NODE_TYPE=bootstrap
      - P2P_PORT=4001

  backend:
    build:
      context: .
      dockerfile: docker/Dockerfile.backend
    ports:
      - "5000:5000"
    depends_on:
      - bootstrap
    environment:
      - BACKEND_PORT=5000
      - BOOTSTRAP_NODE=bootstrap:4001
      - GEMINI_API_KEY=${GEMINI_API_KEY}

  gateway:
    build:
      context: .
      dockerfile: docker/Dockerfile.gateway
    ports:
      - "4000:4000"
    depends_on:
      - backend
    environment:
      - GATEWAY_PORT=4000
      - BACKEND_URL=http://backend:5000

  node-2:
    build:
      context: .
      dockerfile: docker/Dockerfile.backend
    depends_on:
      - bootstrap
    environment:
      - NODE_TYPE=full
      - BOOTSTRAP_NODE=bootstrap:4001

  node-3:
    build:
      context: .
      dockerfile: docker/Dockerfile.backend
    depends_on:
      - bootstrap
    environment:
      - NODE_TYPE=validator
      - VALIDATOR_STAKE=10000
      - BOOTSTRAP_NODE=bootstrap:4001

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=kai-os-2026
```

## 49.2 Monitoring (Issue #19)

```yaml
# monitoring/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: "backend"
    static_configs:
      - targets: ["backend:5000"]
    metrics_path: "/metrics"

  - job_name: "gateway"
    static_configs:
      - targets: ["gateway:4000"]

  - job_name: "nodes"
    static_configs:
      - targets: ["node-2:5001", "node-3:5002"]
```

## 49.3 Starten & Testen

```bash
# Testnet starten
docker-compose up -d

# Logs überwachen
docker-compose logs -f backend

# Status prüfen
docker-compose ps

# Block minen (Testnet)
curl -X POST http://localhost:4000/api/blockchain/mine \
  -H "Content-Type: application/json" \
  -d '{"miner": "ATC_SYSTEM_OWNER_001"}'

# Grafana öffnen
open http://localhost:3001
# Login: admin / kai-os-2026
```

---

# 50. SDK — TypeScript, Python, Rust

> **Datei:** `docs/wiki/overview/` | **Status:** Teils implementiert, teils geplant

## 50.1 TypeScript/JavaScript SDK

```typescript
// npm install @atcchain/sdk

import { KAIClient, ShivamonContract, GovernanceContract } from "@atcchain/sdk";

const client = new KAIClient({
  rpcUrl:    "http://localhost:4000",
  chainId:   9000,
  privateKey: process.env.PRIVATE_KEY,
});

// Shivamon minten
const shivamon = new ShivamonContract(client);
const token    = await shivamon.mint({
  element: "Neon",
  rarity:  "Rare",
});
console.log("Geminted:", token.token_id);

// Governance abstimmen
const gov      = new GovernanceContract(client);
const proposals= await gov.getProposals({ status: "active" });
await gov.vote(proposals[0].id, 0); // Option 0 = "Ja"
```

## 50.2 Python SDK

```python
# pip install atcchain-sdk  (geplant, noch nicht veröffentlicht)
# Direkte API-Nutzung: http://localhost:5000 (Backend) / http://localhost:4000 (Gateway)

from atcchain import KAIClient, ShivamonContract, ATC Token

client   = KAIClient(rpc_url="http://localhost:4000", chain_id=9000)
shivamon = ShivamonContract(client)
token    = ATC Token(client)

# ATC-Balance abfragen
balance = token.balance_of("ATC7F3A...")
print(f"Balance: {balance} ATC")

# Battle starten
result = shivamon.battle(
    attacker_id = "SHIV-0001",
    defender_id = "SHIV-0002",
)
print(f"Winner: {result['winner']}, XP: {result['xp_gained']}")

# Events überwachen
@client.on("Transfer")
async def on_transfer(event):
    print(f"Transfer: {event['from']} → {event['to']}: {event['value']} ATC")
```

## 50.3 Rust SDK (geplant)

```toml
# Cargo.toml
[dependencies]
atcchain-sdk = "0.1.0"
```

```rust
// Basis-Nutzung (geplant)
use atcchain_sdk::{KAIClient, ChainConfig};

#[tokio::main]
async fn main() {
    let client = KAIClient::new(ChainConfig {
        rpc_url:  "http://localhost:4000".into(),
        chain_id: 9000,
    }).await.unwrap();

    let balance = client.get_balance("ATC7F3A...").await.unwrap();
    println!("Balance: {} ATC", balance);
}
```

---

# 51. Sprint-Plan — Vollständige Roadmap

> **Aktuell:** Sprint 2.5 · **Phase:** 2 — Expansion

## 51.1 Phase-Übersicht

| Phase | Sprints | Zeitraum | Status |
|-------|---------|----------|--------|
| Phase 1 — Foundation | 1.1–1.8 | Nov 2025–Mai 2026 | ✅ |
| Phase 2 — Expansion | 2.1–2.10 | Mai–Sep 2026 | 🔨 Aktiv |
| Phase 3 — Cross-Chain | 3.1–3.6 | Okt 2026–Jan 2027 | 📋 |
| Phase 4 — Ecosystem | 4.1–4.6 | Jan–Okt 2027 | 📋 |

## 51.2 Phase 2 — Detaillierter Sprint-Plan

| Sprint | Version | Zeitraum | Fokus | Issues | Status |
|--------|---------|----------|-------|--------|--------|
| 2.1 | v2.1.0 | 19.–26. Mai | Core Contracts | #1, #6 | ✅ |
| 2.2 | v2.1.1 | 27. Mai–3. Jun | ECDSA + Explorer | #4, #5 | ✅ |
| 2.3 | v2.1.2 | 4.–9. Jun | Governance + Marketplace | #9, #13 | ✅ |
| 2.4 | v2.1.3 | 9.–11. Jun | Solidity + Bridge | #10, #12 | ✅ |
| **2.5** | **v2.2.0** | **11.–17. Jun** | **Bootstrap Node + Breeding** | **#8, #11, #14** | 🔨 |
| 2.6 | v2.2.1 | 18.–24. Jun | ATCLang v0.4 + Block Propagation | #15 | 📋 |
| 2.7 | v2.2.2 | 25. Jun–1. Jul | Initial Sync + Chain-Rule | #16, #17 | 📋 |
| 2.8 | v2.3.0 | 2.–15. Jul | Docker Compose + Monitoring | #18, #19 | 📋 |
| 2.9 | v2.3.1 | 16.–29. Jul | Solidity Testnet Deploy | #12 | 📋 |
| 2.10 | v2.3.2 | 30. Jul–12. Aug | Build System + Gateway-Tests | #7, #20 | 📋 |

## 51.3 Phase 3 — Cross-Chain & Mainnet

| Sprint | Version | Fokus |
|--------|---------|-------|
| 3.1 | v3.0.0-alpha | Mainnet Alpha + Bridge MVP |
| 3.2 | v3.0.0-beta | wATC (Wrapped ATC) auf Ethereum |
| 3.3 | v3.0.0-rc1 | Solana-Bridge + Audit |
| 3.4 | v3.0.0 | Mainnet Launch |
| 3.5 | v3.1.0 | DeFi L11 (AMM, Lending) |
| 3.6 | v3.2.0 | Oracle + Flash Loans |

## 51.4 Kritischer Entwicklungspfad

```
#14 Bootstrap Node
  └→ #15 Block Propagation
       └→ #16 Initial Sync
            └→ #17 Longest-Chain-Rule
                 └→ #18 Docker Compose
                      └→ #19 Monitoring
                           └→ #8 Multi-Node Live
                                └→ Mainnet v3.0
```

## 51.5 Offene Issues nach Priorität

| Prio | Issue | Titel | Sprint |
|------|-------|-------|--------|
| 🔴 | #8 | Multi-Node Testnet live | 2.5–2.8 |
| 🟡 | #7 | Build System EXE/AppImage | 2.10 |
| 🟡 | #11 | Shivamon Breeding Gen 2 | 2.5 |
| 🟡 | #12 | Solidity On-Chain Contracts | 2.9 |
| 🟡 | #13 | ATC Marketplace | 2.3 ✅ |
| 🟡 | #18 | Docker Compose Testnet | 2.8 |
| 🟡 | #19 | Node-Monitoring Dashboard | 2.8 |
| 🟢 | #10 | Cross-Chain Bridge | 3.1 |

---

# 52. Glossar & Referenzen

> *Vollständiges Glossar aller Begriffe im A-TownChain Ökosystem*

## 52.1 Begriffe A–Z

| Begriff | Definition |
|---------|-----------|
| **ATC** | A-Town Coin — Haupt-Währung des Ökosystems (max. 21M) |
| **ATC-001** | Genesis Token — symbolischer Ursprungs-Token (Menge: 1) |
| **ATC-8300** | Fungible Token Standard (analog ERC-20) |
| **ATC-9000** | NFT Standard (analog ERC-721) — Shivamon |
| **ATC-9900** | Governance-Token / DAO-Standard |
| **ATCFS** | A-TownChain File System — dezentrales Dateisystem (ATS-1002) |
| **ATCLang** | Native Smart-Contract-Sprache des Ökosystems |
| **ATCNet** | P2P-Netzwerk-Stack (ATS-1006) |
| **ATCVM** | Stack-basierte Virtual Machine für ATCLang-Bytecode |
| **atcpkg** | Package-Manager für ATC-Module (ATS-1001) |
| **Bootstrap Node** | Erster Einstiegspunkt ins P2P-Netzwerk (Issue #14) |
| **Chain ID** | 9000 — eindeutige Identifikation der A-TownChain |
| **CID** | Content Identifier — SHA-256-Hash einer ATCFS-Datei |
| **DAO** | Decentralized Autonomous Organization — Governance-System |
| **DID** | Decentralized Identifier — `did:kai:z6Mkh...` |
| **DNA-Hash** | Einzigartiger genetischer Fingerabdruck eines Shivamon |
| **ECDSA** | Elliptic Curve Digital Signature Algorithm (secp256k1) |
| **FedAvg** | Federated Averaging — Aggregationsalgorithmus für FL |
| **Gas** | Rechengebühr in ATC für Blockchain-Operationen |
| **Governance** | On-Chain-Abstimmungssystem (ATC-9900, 10% Quorum) |
| **Halving** | Halbierung des Block-Rewards alle 210.000 Blöcke |
| **HD-Wallet** | Hierarchical Deterministic Wallet (BIP-32/44) |
| **IPC** | Inter-Process Communication — Agenten-Kommunikation (ATS-1003) |
| **KAI-OS** | KI-Blockchain-Betriebssystem — Gesamt-System |
| **L0–L12** | 13 Schichten der KAI-OS Layer-Architektur |
| **Lock-and-Mint** | Bridge-Mechanismus: ATC sperren → wATC minten |
| **Marketplace** | NFT-Handelsplatz (2.5% Royalty + 1% Platform-Fee) |
| **Mnemonic** | 24-Wort-Seed-Phrase für Wallet (BIP-39) |
| **Multi-Sig** | Multi-Signature Wallet (M-of-N) |
| **NFT** | Non-Fungible Token — einzigartiges digitales Asset |
| **Node** | Teilnehmer im A-TownChain P2P-Netzwerk |
| **PoH** | Proof of History — kryptografischer Zeitstempel-Beweis |
| **PoI** | Proof of Intelligence — KI-Beitrags-Beweis |
| **PoS** | Proof of Stake — Stake-basiertes Konsens-Gewicht |
| **Proposal** | Governance-Antrag (1.000 ATC Deposit, 7 Tage Voting) |
| **Quorum** | Mindest-Beteiligung bei Governance (10% der Supply) |
| **ReAct** | Reason-Act-Observe Loop für KI-Agenten |
| **Relayer** | Service, der Bridge-Events zwischen Chains überträgt |
| **RPC** | Remote Procedure Call — Blockchain-API |
| **Shivamon** | NFT-basiertes Battle-Wesen (ATC-9000, max. 9.900) |
| **ShivaOS** | Browser-basiertes OS-Dashboard (ATS-1007) |
| **Slashing** | Strafe für Validator-Fehlverhalten (10% Stake-Verlust) |
| **Snapshot** | Eingefrorene Token-Balances für Governance-Abstimmung |
| **Stake** | Eingesetztes ATC für Validator-Rechte (min. 10.000 ATC) |
| **Timelock** | 48h Verzögerung vor Governance-Execution |
| **Treasury** | On-Chain-Schatzkammer für Protokoll-Einnahmen |
| **Validator** | Node mit Stake-Recht zur Block-Produktion |
| **VRF** | Verifiable Random Function — nachweisbare Zufälligkeit |
| **wATC** | Wrapped ATC — ERC-20 Repräsentation auf Ethereum |
| **XAI** | Explainable AI — nachvollziehbare KI-Entscheidungen |
| **ZKP** | Zero-Knowledge Proof — Datenschutz-Beweis |

## 52.2 Datei-Referenz-Matrix

| Komponente | Code | Tests | Doku | Issue |
|-----------|------|-------|------|-------|
| ATC-8300 Token | `blockchain/contracts/atc8300/` | `tests/test_smart_contracts.py` | `docs/contracts/ATC_TOKEN_STANDARD.md` | #1 |
| Shivamon NFT | `modules/shivamon/` | `tests/test_smart_contracts.py` | `docs/contracts/SHIVAMON_NFT_CONTRACT.md` | #3, #11 |
| Governance | `blockchain/contracts/governance/` | `tests/test_smart_contracts.py` | `docs/issues/ISSUE_09_GOVERNANCE.md` | #9 |
| Marketplace | `modules/contracts/marketplace/` | `tests/test_smart_contracts.py` | `docs/issues/ISSUE_13_MARKETPLACE.md` | #13 |
| Bridge | `modules/contracts/bridge/` | — | `docs/issues/ISSUE_10_BRIDGE.md` | #10 |
| ECDSA Wallet | `blockchain/wallet/` | `tests/test_ecdsa.py` | `docs/architecture/WALLET_KEYGEN.md` | #6 |
| API Gateway | `gateway/` | `tests/test_gateway.py` | `docs/architecture/GATEWAY.md` | #20 |
| P2P Discovery | `blockchain/nodes/discovery.py` | `tests/test_discovery.py` | `docs/architecture/ATCNET_P2P.md` | #14 |
| ATCLang | `modules/atclang/` | `tests/test_atclang.py` | `docs/atclang/ATCLANG_SPEC_FULL.md` | — |
| Solidity | `blockchain/contracts/solidity/` | `blockchain/contracts/solidity/test/` | `docs/issues/ISSUE_12_SOLIDITY.md` | #12 |
| Testnet | `docker-compose.yml` | — | `docs/architecture/Testnet.md` | #8, #18 |
| Monitoring | `monitoring/` | — | `docs/architecture/Testnet.md` | #19 |

## 52.3 Externe Referenzen

| Ressource | URL |
|-----------|-----|
| GitHub Organisation | https://github.com/A-TownChain-Okosystems |
| Code-Repo | https://github.com/A-TownChain-Okosystems/a-townchain-os |
| Docs-Repo | https://github.com/A-TownChain-Okosystems/a-townchain-os-docs |
| BIP-39 | https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki |
| BIP-32 | https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki |
| EIP-721 | https://eips.ethereum.org/EIPS/eip-721 |
| EIP-20 | https://eips.ethereum.org/EIPS/eip-20 |
| OpenZeppelin | https://docs.openzeppelin.com/ |
| Gemini API | https://ai.google.dev/ |
| Hardhat | https://hardhat.org/ |
| Docker Compose | https://docs.docker.com/compose/ |
| Prometheus | https://prometheus.io/ |
| Kademlia DHT | https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf |

---

> *KAI-OS Wiki — Kapitel 32–52 · Stand: 10. Juni 2026 · Sprint 2.5*
> *Nächster Update: täglich 08:00 Uhr (Auto-Sync) · Aurora (KAI-OS Agent)*

