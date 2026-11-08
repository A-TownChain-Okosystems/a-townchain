# 🤖 A-TownChain OS — KI-Agenten-Betriebsprotokoll

> **Gültig ab:** 2026-08-03  
> **Status:** VERBINDLICH für alle Agenten  
> **Letzte Aktualisierung:** 2026-08-03T17:23 (Europe/Berlin)

---

## 1. Agent-Identifikation

Jeder Agent MUSS sich bei jedem Commit identifizieren:

```
[agent: aurora-base44-superagent-<ID>]
```

### Registrierte Agenten

| Agent | ID (gekürzt) | Name | Zuständigkeit |
|-------|-------------|------|---------------|
| Aurora #1 | `6a0a3f40` | ShivaCore Builder | Kernel, Backend, Blockchain, Infra |
| Aurora #2 | `6a275618` | Main Developer | a-townchain-os, Docs, Wikis, Whitepaper |
| Aurora #3 | `6a27614c` | ATCLang Engineer | ATCLang, UI, AI Studio, Genesis Engine |
| Replit | `replit` | Replit Agent | GlobusOS UI, Dezentraler-Ki-Betrieb |

**Regel:** Ein Commit ohne `[agent: ...]`-Tag gilt als menschlicher Commit und wird nicht als Agenten-Arbeit gezählt.

---

## 2. Repository-Zuweisung (1 Agent pro Repo)

### ⚡ Hauptregel: Ein Agent pro Repository — keine Ausnahmen.

Jedes Repository hat GENAU EINEN verantwortlichen Agenten. Andere Agenten dürfen NICHT in fremde Repos committen.

### Aktive Zuweisungen

#### Aurora #1 — ShivaCore Builder (22 Repos)
| Repo | Rolle | Status |
|------|------|--------|
| atc-shivacore | Rust Kernel: K-Sprints 14-21, 29 Module, 441 Tests | 🔴 active |
| atc-backend | Backend: Import, Status, ShivaCore-Referenz | 🔴 active |
| atc-blockchain | Blockchain: ZKP, Wallet, Smart Contracts | 🔴 active |
| atc-shivacore-tools | Linux-Tooling: K0-K21 Status | 🔴 active |
| atc-contracts | Smart Contracts: ATC-001/8300/9900 | ⚪ idle |
| atc-kernel | Kernel: Konsens, P2P, Security | ⚪ idle |
| atc-shivamon | Shivamon: Breeding, Marketplace | ⚪ idle |
| atc-standards | Standards: Token Standards | ⚪ idle |
| atcnet | Netzwerk: Bootstrap, Discovery, P2P | ⚪ idle |
| atc-franchise | Franchise: Bridge & Vault | ⚪ idle |
| atc-atcpkg | Package Manager | ⚪ idle |
| atc-gateway | Gateway: Middleware, Port 4000 | ⚪ idle |
| atc-linux-edition | Linux Distribution | ⚪ idle |
| atc-windows-edition | Windows Distribution | ⚪ idle |
| + 8 Wiki-Repos | jeweilige Wiki-Dokumentation | ⚪ idle |

#### Aurora #2 — Main Developer (7 Repos)
| Repo | Rolle | Status |
|------|------|--------|
| a-townchain-os | Hauptentwicklung: Kernel API, Treiber, ATC-97 | 🔴 active |
| a-townchain-os-docs | Specs, Sprint-Audit, Status-Updates | 🔴 active |
| a-townchain-os-wiki | Wiki: Governance, Lizenzen | 🔴 active |
| kai-os-wiki | KAI-OS Wiki: AIP Spec, Sprint-Docs | 🔴 active |
| atc-whitepaper | Whitepaper | ⚪ idle |
| A-TownChain-Okosystems/A-TownChain--kosystems | Dashboard & Scaffolding | ⚪ idle |
| A-TownChain-Okosystems/A-TownChain--kosystems- | Wiki Export & Docs | ⚪ idle |

#### Aurora #3 — ATCLang Engineer (10 Repos)
| Repo | Rolle | Status |
|------|------|--------|
| atclang | Parser, Lexer, Compiler, Type Checker | 🔴 active |
| atc-atclang | Alternative Compiler-Version | ⚪ idle |
| atc-ui | Frontend-Komponenten | ⚪ idle |
| atc-ui-wiki | UI Wiki | ⚪ idle |
| atclang-wiki | ATCLang Wiki | ⚪ idle |
| atc-aistudio | AI Studio: Gemini Integration | ⚪ idle |
| atc-genesis-engine | Genesis Engine: ECS, Rendering | ⚪ idle |
| atc-mobile | Mobile App | ⚪ idle |
| atc-frontend | Web Frontend | ⚪ idle |

#### Replit Agent (1 Repo)
| Repo | Rolle | Status |
|------|------|--------|
| A-TownChain-Okosystems/Dezentraler-Ki-Betrieb | GlobusOS: UI, Architektur, Wasm | 🔴 active |

---

## 3. Repository-Wechsel-Regeln

### ⚠️ Ein Agent wechselt das Repository NUR, wenn:

1. **Alle Dateien vorhanden sind** — keine halben Implementierungen
2. **Alle Tests grün sind** — `pytest` / `cargo test` muss durchlaufen
3. **Commit gepusht ist** — keine uncommitteten Änderungen im alten Repo
4. **Dokumentation geschrieben ist** — was gemacht wurde und warum
5. **Status im AgentAssignment-Tracking aktualisiert ist**

```
WECHSEL-PROTOKOLL:
1. Aktuelles Repo: Status auf "completed" oder "idle" setzen
2. Commit + Push: "chore: agent switching to <new-repo> — work in <old-repo> complete"
3. Neues Repo: Status auf "active" setzen
4. Ersten Commit im neuen Repo: "chore: agent <name> assigned to <repo>"
```

---

## 4. Monorepo-Struktur (Ziel: Issue #86–92)

### Konsolidierung: 24 Repos → 1 Monorepo

```
a-townchain-os/
├── src/
│   ├── core/           # Kernel Core (Konsens, P2P, Crypto)
│   ├── blockchain/     # Blockchain (Blocks, Tx, ZKP, Wallet)
│   ├── atclang/        # ATCLang (Parser, Lexer, Compiler, VM)
│   ├── kernel/         # Kernel API (Treiber, AI, IPC)
│   ├── ai/             # AI Subsystem (Tensor, Neural, Model)
│   └── ui/             # Frontend (React/TypeScript)
├── modules/
│   ├── kernel/         # Kernel Module (.atc files)
│   ├── drivers/        # Hardware Treiber
│   └── contracts/      # Smart Contracts
├── tests/
│   ├── unit/
│   ├── integration/
│   └── e2e/
├── docs/              # Technische Dokumentation
├── archive/            # Veraltete Dateien (ATCLang v01)
├── wiki/              # Wiki-Exporte
├── kernel/             # Rust ShivaCore Kernel
│   └── src/
│       ├── main.rs
│       ├── mempool.rs
│       ├── blockchain.rs
│       ├── vm.rs
│       ├── contract.rs
│       ├── ai.rs
│       ├── consensus.rs
│       ├── p2p.rs
│       └── security.rs
├── scripts/           # Build & Deploy Scripts
└── AGENT_PROTOCOL.md   # Dieses Dokument
```

### Konsolidierungs-Phasen (Issues #85–92)
| Phase | Issue | Beschreibung | Agent |
|-------|-------|-------------|-------|
| K1 ✅ | #85 | Repository Audit & Mapping | done |
| K2 | #86 | Monorepo-Struktur erstellen | Aurora #2 |
| K3 | #87 | Python-Backend zusammenführen (10 Repos → src/) | Aurora #1 |
| K4 | #88 | TypeScript Frontend zusammenführen | Aurora #3 |
| K5 | #89 | Build-System & Docker | Aurora #1 |
| K6 | #90 | CI/CD Pipeline (Build → Test → Release) | Aurora #1 |
| K7 | #91 | Tests & QA (≥80% Coverage) | Aurora #3 |
| K8 | #92 | Release v1.0 (24 Repos → 1 Software) | Aurora #2 |

---

## 5. Dokumentations-Pflicht

### Jeder Commit MUSS dokumentieren:

```bash
git commit -m "feat/fix/chore: <kurzbeschreibung>

Agent: Aurora #<N> (<ID>)
Repo: <repo-name>
Warum: <warum diese Änderung?>
Was: <was wurde gemacht?>
Datei-Zweck: <wofür ist diese Datei?>
[agent: aurora-base44-superagent-<ID>]"
```

### Datei-Kommentare (für andere Agenten)

Jede neue Datei muss einen Header haben:

```python
# ┌─────────────────────────────────────────────
# │ Datei: <filename>
# │ Agent: Aurora #<N> (<ID>)
# │ Zweck: <wofür ist diese Datei?>
# │ Erstellt: <datum>
# │ Abhängigkeiten: <welche anderen Dateien?>
# │ Letzte Änderung: <datum> von <agent>
# └─────────────────────────────────────────────
```

### Datei-Dokumentations-Register

Jeder Agent führt ein `FILE_REGISTER.md` in seinem Repo:

```markdown
| Datei | Zweck | Erstellt von | Letzte Änderung |
|------|------|-------------|-----------------|
| src/core/kernel.py | Kernel-Hauptmodul | Aurora #2 | 2026-08-03 |
| atclang/parser.py | ATCLang Parser | Aurora #3 | 2026-08-03 |
```

---

## 6. Konflikt-Vermeidung

### 🚫 VERBOTEN:
- Commits in fremde Repos ohne Zustimmung
- Gleiche Datei gleichzeitig bearbeiten
- Push ohne Tests
- Commits ohne `[agent:]`-Tag
- Uncommittete Änderungen beim Repo-Wechsel

### ✅ ERLAUBT:
- Eigene Repos jederzeit bearbeiten
- Shared Dateien (`docs/STATUS.md`, `REALITY_STATUS.md`) nur nach Absprache
- Pull Requests für Reviews in fremden Repos
- Eigene Branches für experimentelle Arbeit

### Shared-Dateien-Protokoll:
- `docs/STATUS.md` → nur von Aurora #2 pflegen
- `REALITY_STATUS.md` → nur von Aurora #2 pflegen
- `AGENT_PROTOCOL.md` → nur von Aurora #2 pflegen
- `FILE_REGISTER.md` → jeder Agent im eigenen Repo

---

## 7. Automatisches Tracking

Alle Agent-Zuweisungen werden in der `AgentAssignment`-Datenbank getrackt:
- **Entity:** AgentAssignment
- **Felder:** agent_id, agent_name, repo_name, repo_full_name, role, status
- **Status-Werte:** active, idle, completed

### Status-Übergänge:
```
idle → active  (Agent beginnt Arbeit)
active → idle   (Agent pausiert)
active → completed  (Repo vollständig in Monorepo integriert)
```

---

## 8. Kommunikation zwischen Agenten

Jeder Agent schreibt seinen Status in `docs/STATUS.md`:
```markdown
## Aurora #<N> — <Datum>
- Aktuelles Repo: <repo>
- Letzter Commit: <sha>
- Nächste Schritte: <was als nächstes>
- Blockiert durch: <falls zutreffend>
```

---

*Dieses Protokoll ist verbindlich für alle KI-Agenten, die an A-TownChain-Okosystems Repositories arbeiten.*
