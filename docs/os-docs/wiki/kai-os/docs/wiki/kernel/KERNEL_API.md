# 🔧 KAI-OS Kernel API — Dezentrales KI-Betriebssystem

> **Standard:** ATC-97 | **Sprint:** 3.2 | **Version:** 1.0.0-alpha
> **Spec:** `modules/kernel/kernel_api.atc` (ATCLang v0.4)
> **Runtime:** `src/core/kernel/api.py` (Python 3.12)
> **Tests:** `tests/unit/test_kernel_api.py` (60 Tests, alle grün)

---

## Architektur

```
┌──────────────────────────────────────────────────────────┐
│              Kernel API (Syscall Layer)                    │
│              modules/kernel/kernel_api.atc                │
│              src/core/kernel/api.py                        │
├──────────┬──────────┬───────────┬────────────────────────┤
│ Process  │ Memory   │ IPC       │ AI Kernel               │
│ Manager  │ Manager  │ Bus       │ Orchestrator            │
│ (0x01-   │ (0x10-   │ (0x20-    │ (0x30-0x3F)             │
│  0x0F)   │  0x1F)   │  0x2F)    │                         │
├──────────┼──────────┼───────────┼────────────────────────┤
│ Capabil. │ Consensus│ Resource  │ Distributed AI           │
│ Manager  │ Layer    │ Monitor   │ Mesh                    │
│ (0x40-   │ (0x50-   │ (0x70-    │ (0x80-0x8F)            │
│  0x4F)   │  0x5F)   │  0x7F)    │                         │
├──────────┼──────────┼───────────┼────────────────────────┤
│ Agent Lifecycle (0x60-0x6F) │ Kernel (0x90-0x9F)        │
└──────────┴──────────┴───────────┴────────────────────────┘
                      │
                GCL Core (13 Buses)
                      │
         ┌──────────┼──────────┐
    P2P Network   ATCFS    Blockchain
```

---

## Syscall-Tabelle (50+ System Calls)

### Process Management (0x01–0x0F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x01 | `SPAWN` | Neuen Prozess starten | `pid: u64` |
| 0x02 | `KILL` | Prozess beenden | `bool` |
| 0x03 | `WAIT` | Auf Prozessende warten | `exit_code: i32` |
| 0x04 | `SIGNAL` | Signal an Prozess senden | `bool` |
| 0x05 | `SLEEP` | Prozess schlafen legen | `bool` |
| 0x06 | `WAKE` | Prozess wecken | `bool` |
| 0x07 | `YIELD` | CPU-Zeit abgeben | `bool` |
| 0x08 | `PROCESS_LIST` | Alle Prozesse auflisten | `List<Process>` |
| 0x09 | `PROCESS_INFO` | Prozess-Info abfragen | `Process` |
| 0x0A | `SET_PRIORITY` | Priorität ändern | `bool` |

### Memory Management (0x10–0x1F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x10 | `ALLOC` | Speicher allokieren | `region_id: u64` |
| 0x11 | `FREE` | Speicher freigeben | `bool` |
| 0x12 | `MEM_READ` | Geschützter Lesezugriff | `bytes` |
| 0x13 | `MEM_WRITE` | Geschützter Schreibzugriff | `bool` |
| 0x14 | `MEM_SHARE` | Speicher teilen | `bool` |
| 0x15 | `MEM_PROTECT` | Region schützen | `bool` |

### IPC (0x20–0x2F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x20 | `CHAN_CREATE` | IPC-Kanal erstellen | `channel_id: u64` |
| 0x21 | `CHAN_SEND` | Nachricht senden | `bool` |
| 0x22 | `CHAN_RECV` | Nachricht empfangen | `IPCMessage` |
| 0x23 | `CHAN_CLOSE` | Kanal schließen | `bool` |
| 0x24 | `CHAN_SUBSCRIBE` | Kanal abonnieren | `bool` |
| 0x25 | `CHAN_BROADCAST` | Broadcast senden | `bool` |

### AI Kernel (0x30–0x3F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x30 | `AI_ROUTE` | Modell für Task routen | `String` |
| 0x31 | `AI_INFER` | Inference-Anfrage | `(String, u32)` |
| 0x32 | `AI_DECISION` | Entscheidung erfassen | `Hash` |
| 0x33 | `AI_APPROVE` | Entscheidung genehmigen | `bool` |
| 0x34 | `AI_EXECUTE` | Entscheidung ausführen | `bool` |
| 0x35 | `AI_REJECT` | Entscheidung ablehnen | `bool` |
| 0x36 | `AI_AUDIT` | Audit-Trail abfragen | `List<AIDecision>` |

### Capabilities (0x40–0x4F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x40 | `CAP_GRANT` | Capability vergeben | `Hash` |
| 0x41 | `CAP_REVOKE` | Capability entziehen | `bool` |
| 0x42 | `CAP_DELEGATE` | Capability delegieren | `Hash` |
| 0x43 | `CAP_CHECK` | Capability prüfen | `bool` |

### Consensus (0x50–0x5F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x50 | `VALIDATOR_REGISTER` | Validator anmelden | `bool` |
| 0x51 | `VALIDATOR_VOTE` | Stimme abgeben | `bool` |
| 0x52 | `FORK_RESOLVE` | Fork auflösen | `u64` |
| 0x53 | `CHECKPOINT` | State-Checkpoint | `Hash` |

### Agent Lifecycle (0x60–0x6F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x60 | `AGENT_REGISTER` | Agent registrieren | `Hash` |
| 0x61 | `AGENT_DEREGISTER` | Agent abmelden | `bool` |
| 0x62 | `AGENT_MIGRATE` | Agent migrieren | `bool` |
| 0x63 | `AGENT_SNAPSHOT` | Agent-State sichern | `Hash` |

### Resource Management (0x70–0x7F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x70 | `GAS_REPORT` | Gas-Verbrauch | `GasReport` |
| 0x71 | `STAKE_LOCK` | Stake hinterlegen | `bool` |
| 0x72 | `STAKE_UNLOCK` | Stake freigeben | `bool` |
| 0x73 | `RESOURCE_LIMITS` | Limits abfragen | `(u32, u64, u64)` |

### Distributed Intelligence (0x80–0x8F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x80 | `FEDERATED_TRAIN` | Federated Learning starten | `Hash` |
| 0x81 | `MODEL_SYNC` | Modell synchronisieren | `bool` |
| 0x82 | `KNOWLEDGE_TRANSFER` | Wissen übertragen | `bool` |
| 0x83 | `NEURAL_MESH_JOIN` | Mesh beitreten | `bool` |
| 0x84 | `NEURAL_MESH_LEAVE` | Mesh verlassen | `bool` |

### Kernel (0x90–0x9F)

| Syscall | Name | Beschreibung | Return |
|--------|------|-------------|--------|
| 0x90 | `KERNEL_STATS` | System-Statistiken | `KernelStats` |
| 0x91 | `KERNEL_LOG` | Kernel-Log | `List<LogEntry>` |
| 0x92 | `KERNEL_SHUTDOWN` | Herunterfahren | `bool` |
| 0x93 | `KERNEL_REBOOT` | Neustart | `bool` |

---

## Unified Syscall Dispatch

Alle Syscalls laufen durch einen zentralen Dispatcher:

```python
from core.kernel.api import KernelAPI, Syscall, SyscallRequest

api = KernelAPI()

# Per Enum
resp = api.syscall(SyscallRequest(
    syscall=Syscall.AI_ROUTE,
    caller_pid=1,
    args=["reasoning"]
))
# resp.success → True
# resp.data → "mistral-7b"
# resp.gas_used → 100

# Per Name
resp = api.syscall_named("kernel_stats")
```

```atc
// ATCLang
let req = SyscallRequest {
    syscall: Syscall::AI_INFER,
    caller_pid: 1,
    args: [agent, task, input, model, max_tokens],
    cap_id: Hash::zero(),
}
let resp = kernel.syscall(req)
```

---

## Core Types

### KernelProcess
| Field | Type | Beschreibung |
|-------|------|-------------|
| pid | u64 | Prozess-ID |
| name | String | Name |
| ptype | ProcessType | AGENT/SERVICE/CONTRACT/SYSTEM/VALIDATOR/ORACLE |
| state | ProcessState | CREATED/RUNNING/SLEEPING/WAITING/STOPPED/ZOMBIE/KILLED |
| priority | u8 | 0=niedrig, 255=System |
| cpu_time_ms | u64 | CPU-Zeit in ms |
| gas_used | u64 | Gas-Verbrauch |
| stake | u128 | ATC-Stake |
| channels | List\<u64\> | IPC-Kanäle |

### AgentDescriptor
| Field | Type | Beschreibung |
|-------|------|-------------|
| agent_id | Hash | Eindeutige ID |
| owner | Address | Besitzer |
| status | AgentStatus | REGISTERED/ACTIVE/SUSPENDED/MIGRATING/TERMINATED |
| model_endpoint | String | KI-Modell Endpoint |
| stake | u128 | ATC-Stake |
| reputation | u32 | Reputation Score (Start: 100) |
| node_id | u64 | Neural Mesh Node |

### AIDecision
| Field | Type | Beschreibung |
|-------|------|-------------|
| decision_id | Hash | Eindeutige ID |
| agent | Address | Agent der entschieden hat |
| status | DecisionStatus | PENDING/APPROVED/REJECTED/EXECUTED/EXPIRED |
| confidence | f32 | Konfidenzwert 0.0–1.0 |
| block_height | u64 | Block bei Ausführung |

---

## Events

| Event | Parameter | Beschreibung |
|-------|-----------|-------------|
| `ProcessSpawned` | pid, name, ptype, owner | Prozess gestartet |
| `ProcessKilled` | pid, signal, by | Prozess beendet |
| `MemoryAllocated` | region_id, pid, size | Speicher allokiert |
| `ChannelCreated` | channel_id, sender, msg_type | IPC-Kanal erstellt |
| `AIDecisionMade` | decision_id, agent, status | KI-Entscheidung |
| `AgentRegistered` | agent_id, owner, name | Agent registriert |
| `ValidatorRegistered` | validator, stake | Validator angemeldet |
| `GasConsumed` | pid, amount, total | Gas verbraucht |
| `FederatedRound` | task_id, round, participants | Federated Learning Round |
| `SyscallExecuted` | syscall, caller, gas, success | Syscall ausgeführt |

---

## Konfiguration

| Parameter | Wert | Beschreibung |
|-----------|------|-------------|
| MAX_PROCESSES | 1024 | Max gleichzeitige Prozesse |
| MAX_MEMORY_PER_PROCESS | 256 MB | Max Speicher pro Prozess |
| GAS_PER_CPU_MS | 100 | Gas-Kosten pro CPU-ms |
| MIN_VALIDATOR_STAKE | 10.000 ATC | Min Stake für Validator |

---

## Model Routing

| Task | Modell |
|------|--------|
| reasoning | mistral-7b |
| code | phi-2 |
| summarize | llama-3.2-3b |
| qa | llama-3.2-3b |
| text (default) | gemma-2-2b |

---

*Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems*
*Aurora · 03.08.2026 (Europe/Berlin)*
