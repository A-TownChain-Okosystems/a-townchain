# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ShivaOS IPC — Inter-Process Communication (ATS-1003)
Message-Passing zwischen Kernel-Prozessen (kein Shared Memory).
"""
import threading, queue, time, uuid, logging
from dataclasses import dataclass, field
from typing import Dict, Optional, Any, Callable

logger = logging.getLogger("shivaos.ipc")

@dataclass
class IPCMessage:
    sender:    str
    receiver:  str
    msg_type:  str
    payload:   Any = None
    msg_id:    str = field(default_factory=lambda: uuid.uuid4().hex[:8])
    ts:        float = field(default_factory=time.time)
    reply_to:  Optional[str] = None

class IPCChannel:
    """Bidirektionaler Nachrichtenkanal zwischen zwei Prozessen."""

    def __init__(self, name: str, maxsize: int = 256):
        self.name    = name
        self._queue  = queue.Queue(maxsize=maxsize)
        self._lock   = threading.Lock()

    def send(self, msg: IPCMessage, timeout: float = 1.0) -> bool:
        try:
            self._queue.put(msg, timeout=timeout)
            return True
        except queue.Full:
            logger.warning(f"[IPC] Channel {self.name} voll — Nachricht verworfen")
            return False

    def recv(self, timeout: float = 0.1) -> Optional[IPCMessage]:
        try:
            return self._queue.get(timeout=timeout)
        except queue.Empty:
            return None

    def size(self) -> int:
        return self._queue.qsize()

class IPCBus:
    """
    Zentraler IPC-Bus des ShivaOS Kernels.
    Prozesse registrieren sich und kommunizieren über den Bus.
    """

    def __init__(self):
        self._channels: Dict[str, IPCChannel] = {}
        self._handlers: Dict[str, Callable]   = {}
        self._lock       = threading.Lock()
        self._running    = False

    def register(self, process_id: str) -> IPCChannel:
        with self._lock:
            ch = IPCChannel(process_id)
            self._channels[process_id] = ch
            logger.debug(f"[IPC] Prozess registriert: {process_id}")
            return ch

    def unregister(self, process_id: str):
        with self._lock:
            self._channels.pop(process_id, None)

    def send(self, msg: IPCMessage) -> bool:
        ch = self._channels.get(msg.receiver)
        if not ch:
            logger.warning(f"[IPC] Kein Kanal für {msg.receiver}")
            return False
        return ch.send(msg)

    def broadcast(self, sender: str, msg_type: str, payload: Any = None):
        """Nachricht an alle registrierten Prozesse."""
        with self._lock:
            targets = list(self._channels.keys())
        for target in targets:
            if target == sender: continue
            self.send(IPCMessage(sender=sender, receiver=target,
                                  msg_type=msg_type, payload=payload))

    def on(self, process_id: str, handler: Callable):
        """Handler für eingehende Nachrichten registrieren."""
        self._handlers[process_id] = handler

    def stats(self) -> dict:
        return {
            "channels": len(self._channels),
            "queued":   sum(ch.size() for ch in self._channels.values()),
        }
