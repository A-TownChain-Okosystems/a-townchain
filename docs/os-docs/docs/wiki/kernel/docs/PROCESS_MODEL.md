# Prozess-Modell (ATS-1001)

## Prozess-Typen
| Typ | Beschreibung | Priorität |
|-----|-------------|----------|
| `AGENT` | KI-Agent Prozesse | HIGH |
| `SERVICE` | Hintergrund-Dienste | NORMAL |
| `CONTRACT` | ATCLang Contract Runner | NORMAL |
| `DAEMON` | System-Daemons | LOW |
| `USER` | User-Prozesse | NORMAL |

## Prozess-States
```
INIT → STARTING → RUNNING → PAUSED → STOPPING → STOPPED
                     ↓                              ↑
                   ERROR  ────────────────────────→ ↑
```

## Process-API
```python
from shivaos.kernel.kernel import ShivaKernel

kernel = ShivaKernel()
kernel.start()

# Prozess spawnen
pid = kernel.spawn("my_service", ProcessType.SERVICE, start_fn)

# Prozess pausieren
kernel.pause(pid)

# Prozess beenden
kernel.kill(pid)

# Alle Prozesse
processes = kernel.list_processes()
```

## PID-Vergabe
- PID 0: Kernel selbst (reserviert)
- PID 1–99: System-Prozesse
- PID 100+: User/Contract-Prozesse
- PIDs werden nicht wiederverwendet (monoton steigend)

## Memory-Isolation (ATS-1002)
- Jeder Prozess hat eigenen Speicherbereich
- Kein direkter Speicherzugriff zwischen Prozessen
- IPC ausschließlich via EventBus oder typisierte Channels
