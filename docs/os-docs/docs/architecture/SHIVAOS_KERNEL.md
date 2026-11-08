# 🖥️ ShivaOS Kernel — Technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Datei:** `shivaos/kernel/kernel.py` (381 Zeilen)

---

## Überblick

ShivaOS ist ein vollständig proprietäres, dezentrales KI-Betriebssystem. Es ist **kein POSIX-Klon** und kein Linux-Fork. Alle Konzepte wurden eigenständig entwickelt und orientieren sich an den **ATS-1000–1007 Standards**.

```
┌────────────────────────────────────────────────┐
│             ShivaOS Kernel (ATS-1000)          │
├──────────┬──────────┬──────────┬───────────────┤
│ Prozess  │ Speicher │  IPC     │  Event System │
│ Manager  │ Manager  │  Layer   │  (EventBus)   │
├──────────┴──────────┴──────────┴───────────────┤
│            Module Loader                        │
├──────────┬──────────┬──────────┬───────────────┤
│  ATCFS   │  ATCNet  │ ATCLang  │ ShivaConsensus│
│  (FS)    │  (Net)   │  (VM)    │  (Chain)      │
└──────────┴──────────┴──────────┴───────────────┘
```

---

## Prozess-Typen

**Datei:** `shivaos/kernel/kernel.py`

```python
class ProcessType(IntEnum):
    AGENT     = auto()   # KI-Agent (Gemini, lokale Modelle)
    SERVICE   = auto()   # Hintergrund-Dienst (ATCNet, ATCFS)
    CONTRACT  = auto()   # Smart Contract (ATCLang VM)
    DAEMON    = auto()   # System-Daemon (Consensus, Mining)
    USER      = auto()   # User-Prozess (CLI, REPL)
```

### Prozess-Zustände (FSM)
```
CREATED → READY → RUNNING → BLOCKED → READY
                    │
                    └→ TERMINATED
```

---

## Prozess-Verwaltung

```python
# Prozess starten
pid = kernel.spawn(
    process_type = ProcessType.SERVICE,
    name         = "ATCNet-P2P",
    target       = atcnet.run,
    priority     = 5
)

# Prozess beenden
kernel.kill(pid)

# Auf Prozess warten
exit_code = kernel.wait(pid)

# Alle Prozesse auflisten
processes = kernel.list_processes()
# → [{"pid": 1, "name": "ATCNet-P2P", "state": "RUNNING", ...}]
```

### Prozess-Prioritäten
| Priorität | Level | Beschreibung |
|-----------|-------|-------------|
| CRITICAL | 10 | Kernel-Prozesse |
| HIGH | 7-9 | Consensus, Netzwerk |
| NORMAL | 4-6 | Services, APIs |
| LOW | 1-3 | User-Prozesse, REPL |

---

## Speicherverwaltung (ATS-1002)

```python
# Speicher allozieren
region = kernel.alloc(size=1024 * 1024, pid=current_pid)
# region.base_addr, region.size, region.pid

# Speicher freigeben
kernel.free(region)

# Speicher-Statistiken
stats = kernel.memory_stats()
# {
#   "total":     256 MB,
#   "used":       42 MB,
#   "free":      214 MB,
#   "processes":  {pid: used_bytes, ...}
# }
```

### Speicher-Isolation
- Jeder Prozess hat eigenen Adressraum
- Smart Contracts: isolierter Heap (kein Zugriff auf andere Contracts)
- KI-Agenten: können nur ihren eigenen State lesen/schreiben

---

## IPC — Inter-Prozess-Kommunikation (ATS-1007)

```python
# Kanal erstellen
chan = kernel.create_channel("atcnet-to-consensus")

# Nachricht senden (nicht-blockierend)
kernel.send(channel="atcnet-to-consensus", msg={"type": "new_block", "data": block})

# Nachricht empfangen (blockierend mit Timeout)
msg = kernel.recv(channel="atcnet-to-consensus", timeout=5.0)

# Broadcast an alle Subscriber
kernel.broadcast("system.alert", {"level": "warn", "msg": "Low disk space"})
```

---

## System-Calls

Vollständige Liste aller Kernel-System-Calls:

| System-Call | Signatur | Beschreibung |
|-------------|---------|-------------|
| `spawn` | `(type, name, target, priority) -> PID` | Prozess starten |
| `kill` | `(pid) -> bool` | Prozess beenden |
| `wait` | `(pid, timeout?) -> ExitCode` | Auf Prozess warten |
| `sleep` | `(seconds: float)` | Prozess schlafen lassen |
| `getpid` | `() -> PID` | Eigene PID |
| `getppid` | `() -> PID` | Eltern-PID |
| `list_processes` | `() -> List[ProcessInfo]` | Alle Prozesse |
| `alloc` | `(size, pid) -> MemRegion` | Speicher allozieren |
| `free` | `(region)` | Speicher freigeben |
| `mmap` | `(file, size) -> MemRegion` | Datei in Speicher mappen |
| `create_channel` | `(name) -> Channel` | IPC-Kanal erstellen |
| `send` | `(channel, msg)` | Nachricht senden |
| `recv` | `(channel, timeout?) -> msg` | Nachricht empfangen |
| `broadcast` | `(event, data)` | Broadcast senden |
| `open` | `(path, mode) -> FileHandle` | Datei öffnen (ATCFS) |
| `read` | `(handle, size) -> bytes` | Datei lesen |
| `write` | `(handle, data)` | Datei schreiben |
| `close` | `(handle)` | Datei schließen |
| `stat` | `(path) -> FileStat` | Datei-Metadaten |

---

## Boot-Sequenz

```
1. Kernel.__init__()   → EventBus + ModuleLoader initialisieren
2. kernel.start()
   ├── event_bus       → Built-in (immer verfügbar)
   ├── atcfs           → shivaos/fs/atcfs.py
   ├── atcnet          → shivaos/net/atcnet.py
   ├── consensus       → shivaos/consensus/shiva_consensus.py
   ├── blockchain      → blockchain/atcoin/atcoin.py
   ├── wallet          → blockchain/wallet/keygen.py
   ├── ai_orchestrator → backend/api/orchestrator/orchestrator.py
   └── api_gateway     → Extern (Port 4000)
3. kernel.event_bus.emit("kernel.ready")
```

Boot-Zeit (Testnet): **~0.3–0.8 Sekunden**

---

## Shutdown-Sequenz

```python
kernel.stop()
# Stoppt Module in umgekehrter Reihenfolge:
# api_gateway → ai_orchestrator → wallet → blockchain
# → consensus → atcnet → atcfs → event_bus
```

Alle Module implementieren `.stop()` für graceful shutdown.
