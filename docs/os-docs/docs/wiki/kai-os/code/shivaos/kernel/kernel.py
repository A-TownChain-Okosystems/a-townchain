# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ShivaOS Kernel — Dezentrales KI-Betriebssystem
Version: 1.0.0-alpha | ATS-1000 konform
Kein POSIX-Klon — eigenständige Architektur
"""

import time, threading, uuid, hashlib
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Callable, Any
from enum import IntEnum, auto


# ══════════════════════════════════════════════════════════
#  PROZESS-TYPEN & STATUS
# ══════════════════════════════════════════════════════════

class ProcessType(IntEnum):
    AGENT     = auto()   # KI-Agent
    SERVICE   = auto()   # Hintergrund-Dienst
    CONTRACT  = auto()   # Smart Contract
    SYSTEM    = auto()   # System-Prozess
    VALIDATOR = auto()   # Consensus-Validator

class ProcessState(IntEnum):
    CREATED  = auto()
    RUNNING  = auto()
    SLEEPING = auto()
    WAITING  = auto()
    STOPPED  = auto()
    KILLED   = auto()


@dataclass
class MemRegion:
    pid:   int
    size:  int
    data:  bytearray = field(default_factory=bytearray)
    addr:  int = 0

    def read(self, offset: int, length: int) -> bytes:
        return bytes(self.data[offset:offset+length])

    def write(self, offset: int, data: bytes):
        end = offset + len(data)
        if end > len(self.data):
            self.data.extend(bytearray(end - len(self.data)))
        self.data[offset:end] = data


@dataclass
class KernelProcess:
    pid:        int
    name:       str
    ptype:      ProcessType
    state:      ProcessState = ProcessState.CREATED
    memory:     Optional[MemRegion] = None
    stake:      int = 0           # ATC-Stake
    priority:   int = 128         # 0=niedrig, 255=System
    created_at: int = field(default_factory=lambda: int(time.time() * 1000))
    cpu_time:   int = 0           # ms
    gas_used:   int = 0
    owner:      str = ""          # ATC-Adresse
    channels:   List[int] = field(default_factory=list)
    exit_code:  int = 0
    _thread:    Any = field(default=None, repr=False)
    _fn:        Any = field(default=None, repr=False)


# ══════════════════════════════════════════════════════════
#  IPC — INTER-PROCESS-COMMUNICATION (ATS-1003)
# ══════════════════════════════════════════════════════════

class ChannelType(IntEnum):
    PIPE      = auto()
    QUEUE     = auto()
    STREAM    = auto()
    BROADCAST = auto()

@dataclass
class IPCMessage:
    channel:   int
    from_pid:  int
    msg_type:  str
    data:      Any
    seq:       int = 0
    timestamp: int = field(default_factory=lambda: int(time.time() * 1000))

@dataclass
class Channel:
    channel_id: int
    ctype:      ChannelType
    sender_pid: int
    buffer:     int = 64    # Max gepufferte Nachrichten
    _queue:     List[IPCMessage] = field(default_factory=list)
    _lock:      Any = field(default_factory=threading.Lock, repr=False)
    subscribers: List[int] = field(default_factory=list)

    def send(self, msg: IPCMessage) -> bool:
        with self._lock:
            if len(self._queue) >= self.buffer:
                return False
            msg.channel = self.channel_id
            self._queue.append(msg)
            return True

    def recv(self, pid: int) -> Optional[IPCMessage]:
        with self._lock:
            if self._queue:
                return self._queue.pop(0)
        return None

    def peek(self) -> int:
        return len(self._queue)


# ══════════════════════════════════════════════════════════
#  SHIVA OS KERNEL
# ══════════════════════════════════════════════════════════

class ShivaKernel:
    """
    ShivaOS Kernel — Kern des dezentralen KI-Betriebssystems.
    ATS-1000 konform. Eigene Architektur.
    """

    VERSION     = "1.0.0-alpha"
    MAX_PROCS   = 1024
    MEM_LIMIT   = 256 * 1024 * 1024   # 256MB pro Prozess
    GAS_PER_MS  = 100                  # Gas-Kosten pro CPU-ms

    def __init__(self):
        self._pid_counter  = 0
        self._chan_counter = 0
        self.processes:  Dict[int, KernelProcess] = {}
        self.channels:   Dict[int, Channel]       = {}
        self.syscall_table: Dict[str, Callable]   = {}
        self._lock       = threading.Lock()
        self._running    = False
        self._scheduler_thread = None
        self.event_log:  List[dict] = []
        self._register_builtins()
        self._boot()

    def _boot(self):
        """Kernel booten."""
        print(f"  🖥️  ShivaOS Kernel v{self.VERSION} — Booting...")
        # System-Prozess PID 0
        self._spawn_system("kernel_idle",   ProcessType.SYSTEM,   priority=0)
        self._spawn_system("ipc_daemon",    ProcessType.SERVICE,  priority=200)
        self._spawn_system("ki_orchestrator", ProcessType.AGENT,  priority=240)
        print(f"  ✅ Kernel bereit | {len(self.processes)} System-Prozesse")

    def _spawn_system(self, name: str, ptype: ProcessType, priority: int = 128) -> int:
        pid = self._next_pid()
        proc = KernelProcess(pid=pid, name=name, ptype=ptype,
                              priority=priority, state=ProcessState.RUNNING,
                              owner="ATC" + "0" * 32)
        proc.memory = MemRegion(pid=pid, size=1024 * 1024)  # 1MB
        self.processes[pid] = proc
        return pid

    def _next_pid(self) -> int:
        with self._lock:
            self._pid_counter += 1
            return self._pid_counter

    def _next_chan(self) -> int:
        with self._lock:
            self._chan_counter += 1
            return self._chan_counter

    def _register_builtins(self):
        """System-Calls registrieren."""
        self.syscall_table.update({
            "spawn":         self.spawn,
            "kill":          self.kill,
            "alloc":         self.alloc,
            "free":          self.free,
            "chan_create":   self.create_channel,
            "chan_send":     self.channel_send,
            "chan_recv":     self.channel_recv,
            "proc_list":     self.list_processes,
            "proc_info":     self.process_info,
            "ki_query":      self.ki_query,
            "log":           self._log,
        })

    # ── Prozess-Management ────────────────────────────────

    def spawn(self, name: str, ptype: ProcessType,
              fn: Optional[Callable] = None,
              owner: str = "", stake: int = 0,
              priority: int = 128,
              mem_size: int = 4 * 1024 * 1024) -> int:
        """Neuen Prozess starten."""
        if len(self.processes) >= self.MAX_PROCS:
            raise KernelError("Maximale Prozessanzahl erreicht")

        pid  = self._next_pid()
        proc = KernelProcess(
            pid=pid, name=name, ptype=ptype,
            owner=owner, stake=stake, priority=priority,
            state=ProcessState.CREATED,
        )
        proc.memory = MemRegion(pid=pid, size=min(mem_size, self.MEM_LIMIT))
        proc._fn    = fn

        self.processes[pid] = proc
        self._log(f"SPAWN pid={pid} name={name} type={ptype.name}")

        if fn:
            self._run_process(proc)

        return pid

    def _run_process(self, proc: KernelProcess):
        """Prozess in eigenem Thread starten."""
        def _target():
            proc.state = ProcessState.RUNNING
            start = time.time()
            try:
                proc._fn(proc, self)
                proc.exit_code = 0
            except Exception as e:
                proc.exit_code = 1
                self._log(f"PROC_ERROR pid={proc.pid} err={e}")
            finally:
                proc.cpu_time = int((time.time() - start) * 1000)
                proc.gas_used = proc.cpu_time * self.GAS_PER_MS
                proc.state    = ProcessState.STOPPED

        t = threading.Thread(target=_target, daemon=True, name=f"atc-{proc.name}")
        proc._thread = t
        t.start()

    def kill(self, pid: int, signal: int = 9) -> bool:
        """Prozess beenden."""
        if pid not in self.processes:
            return False
        proc = self.processes[pid]
        proc.state = ProcessState.KILLED
        proc.exit_code = -signal
        if proc._thread and proc._thread.is_alive():
            pass  # Thread kann nicht von außen gekillt werden — Kooperation nötig
        self._log(f"KILL pid={pid} signal={signal}")
        return True

    def wait(self, pid: int, timeout_ms: int = 5000) -> int:
        """Auf Prozess-Ende warten."""
        if pid not in self.processes:
            return -1
        proc = self.processes[pid]
        if proc._thread:
            proc._thread.join(timeout=timeout_ms / 1000)
        return proc.exit_code

    def list_processes(self) -> List[dict]:
        return [
            {"pid": p.pid, "name": p.name, "type": p.ptype.name,
             "state": p.state.name, "priority": p.priority,
             "cpu_ms": p.cpu_time, "gas": p.gas_used}
            for p in sorted(self.processes.values(), key=lambda x: x.pid)
        ]

    def process_info(self, pid: int) -> Optional[dict]:
        p = self.processes.get(pid)
        if not p: return None
        return {"pid": p.pid, "name": p.name, "type": p.ptype.name,
                "state": p.state.name, "owner": p.owner,
                "stake": p.stake, "priority": p.priority,
                "mem_size": p.memory.size if p.memory else 0,
                "cpu_ms": p.cpu_time, "gas": p.gas_used,
                "channels": p.channels}

    # ── Speicher-Management ───────────────────────────────

    def alloc(self, size: int, pid: int) -> MemRegion:
        """Speicher für Prozess allokieren."""
        if size > self.MEM_LIMIT:
            raise KernelError(f"Speicheranfrage zu groß: {size} > {self.MEM_LIMIT}")
        region = MemRegion(pid=pid, size=size)
        self._log(f"ALLOC pid={pid} size={size}")
        return region

    def free(self, region: MemRegion) -> bool:
        region.data.clear()
        self._log(f"FREE pid={region.pid}")
        return True

    # ── IPC ──────────────────────────────────────────────

    def create_channel(self, ptype: ChannelType = ChannelType.QUEUE,
                       sender_pid: int = 0, buffer: int = 64) -> int:
        cid  = self._next_chan()
        chan = Channel(channel_id=cid, ctype=ptype,
                       sender_pid=sender_pid, buffer=buffer)
        self.channels[cid] = chan
        if sender_pid in self.processes:
            self.processes[sender_pid].channels.append(cid)
        self._log(f"CHAN_CREATE cid={cid} type={ptype.name}")
        return cid

    def channel_send(self, cid: int, from_pid: int,
                     msg_type: str, data: Any) -> bool:
        chan = self.channels.get(cid)
        if not chan: return False
        msg = IPCMessage(channel=cid, from_pid=from_pid,
                         msg_type=msg_type, data=data,
                         seq=chan.peek())
        ok = chan.send(msg)
        if ok:
            self._log(f"IPC_SEND cid={cid} from={from_pid} type={msg_type}")
        return ok

    def channel_recv(self, cid: int, pid: int) -> Optional[IPCMessage]:
        chan = self.channels.get(cid)
        if not chan: return None
        return chan.recv(pid)

    def subscribe_broadcast(self, cid: int, pid: int):
        chan = self.channels.get(cid)
        if chan and pid not in chan.subscribers:
            chan.subscribers.append(pid)

    # ── KI-Orchestrator ───────────────────────────────────

    def ki_query(self, prompt: str, model: str = "atc-local",
                 from_pid: int = 0) -> dict:
        """
        KI-Anfrage an den Orchestrator.
        Dezentral: Anfrage wird über P2P-Netz verteilt.
        """
        query_id = hashlib.sha3_256(
            f"{prompt}{time.time()}{from_pid}".encode()
        ).hexdigest()[:16]

        self._log(f"KI_QUERY id={query_id} model={model} pid={from_pid}")

        # Lokale Verarbeitung (Platzhalter für echtes KI-Modell)
        response = {
            "query_id": query_id,
            "model":    model,
            "status":   "queued",
            "gas_est":  len(prompt) * 10,
            "from_pid": from_pid,
        }
        return response

    # ── Syscall-Interface ─────────────────────────────────

    def syscall(self, name: str, *args, **kwargs) -> Any:
        """Einheitliches Syscall-Interface für Prozesse."""
        if name not in self.syscall_table:
            raise KernelError(f"Unbekannter Syscall: '{name}'")
        return self.syscall_table[name](*args, **kwargs)

    # ── Logging ──────────────────────────────────────────

    def _log(self, msg: str):
        entry = {"ts": int(time.time() * 1000), "msg": msg}
        self.event_log.append(entry)

    def get_log(self, last_n: int = 20) -> List[dict]:
        return self.event_log[-last_n:]

    # ── Stats ─────────────────────────────────────────────

    def stats(self) -> dict:
        procs = list(self.processes.values())
        return {
            "version":    self.VERSION,
            "processes":  len(procs),
            "running":    sum(1 for p in procs if p.state == ProcessState.RUNNING),
            "channels":   len(self.channels),
            "total_gas":  sum(p.gas_used for p in procs),
            "syscalls":   len(self.syscall_table),
        }


class KernelError(Exception):
    pass
