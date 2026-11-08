# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# ShivaOS Kernel v2.1.0
# Vollständige Implementierung → shivaos/kernel/kernel.py

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
            msg.