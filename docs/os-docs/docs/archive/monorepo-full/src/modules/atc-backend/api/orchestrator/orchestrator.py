# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/orchestrator/orchestrator.py
# A-TownChain — API Orchestrator (ATS-1000)
#
# Task-Queue-Orchestrator: nimmt Tasks per TaskType entgegen, dispatcht sie
# an registrierte Service-Funktionen und verarbeitet sie ueber einen
# Worker-Pool (Threads). Ersetzt die alte statische Service-Registry.

import queue
import threading
import time
import uuid
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable, Dict, List, Optional


class TaskType(Enum):
    CORE = "core"
    BLOCKCHAIN = "blockchain"
    WALLET = "wallet"
    AI = "ai"
    GAME = "game"
    NODES = "nodes"
    SYSTEM = "system"


class TaskStatus(Enum):
    PENDING = "pending"
    RUNNING = "running"
    DONE = "done"
    FAILED = "failed"


@dataclass
class Task:
    task_type: TaskType
    payload: dict
    task_id: str = field(default_factory=lambda: uuid.uuid4().hex)
    status: TaskStatus = TaskStatus.PENDING
    result: Any = None
    error: Optional[str] = None
    created_at: float = field(default_factory=time.time)


class APIOrchestrator:
    """Koordiniert Backend-Services ueber eine Task-Queue mit Worker-Pool."""

    def __init__(self):
        self._queue: "queue.Queue[Task]" = queue.Queue()
        self._handlers: Dict[str, Callable[[dict], Any]] = {}
        self._handler_types: Dict[str, List[TaskType]] = {}
        self._type_to_handlers: Dict[TaskType, List[str]] = {t: [] for t in TaskType}
        self._workers: List[threading.Thread] = []
        self._running = False
        self.start_time: Optional[float] = None

    def register_fn(self, name: str, fn: Callable[[dict], Any], task_types: List[TaskType]):
        """Registriert eine Service-Funktion fuer einen oder mehrere TaskTypes."""
        self._handlers[name] = fn
        self._handler_types[name] = task_types
        for t in task_types:
            self._type_to_handlers.setdefault(t, []).append(name)

    def start(self, workers: int = 2):
        """Startet den Worker-Pool."""
        self._running = True
        self.start_time = time.time()
        for _ in range(max(1, workers)):
            t = threading.Thread(target=self._worker_loop, daemon=True)
            t.start()
            self._workers.append(t)

    def stop(self):
        """Stoppt den Worker-Pool."""
        self._running = False
        for t in self._workers:
            t.join(timeout=1)
        self._workers = []

    def _worker_loop(self):
        while self._running:
            try:
                task = self._queue.get(timeout=0.1)
            except queue.Empty:
                continue
            self._process(task)
            self._queue.task_done()

    def _process(self, task: Task):
        task.status = TaskStatus.RUNNING
        handler_names = self._type_to_handlers.get(task.task_type, [])
        if not handler_names:
            task.status = TaskStatus.FAILED
            task.error = f"No service registered for {task.task_type.value}"
            return
        fn = self._handlers[handler_names[0]]
        try:
            task.result = fn(task.payload)
            task.status = TaskStatus.DONE
        except Exception as e:  # noqa: BLE001
            task.status = TaskStatus.FAILED
            task.error = str(e)

    def dispatch(self, task_type: TaskType, payload: dict) -> Task:
        """Reiht einen Task asynchron ein und gibt ihn (mit live-Status) zurueck."""
        task = Task(task_type=task_type, payload=payload)
        self._queue.put(task)
        return task

    def dispatch_sync(self, task_type: TaskType, payload: dict, timeout: float = 5.0) -> Any:
        """Reiht einen Task ein und wartet auf das Ergebnis (oder wirft bei FAILED/Timeout)."""
        task = self.dispatch(task_type, payload)
        deadline = time.time() + timeout
        while task.status in (TaskStatus.PENDING, TaskStatus.RUNNING) and time.time() < deadline:
            time.sleep(0.005)
        if task.status == TaskStatus.FAILED:
            raise RuntimeError(task.error or "Task failed")
        if task.status != TaskStatus.DONE:
            raise TimeoutError(f"Task {task.task_id} timed out")
        return task.result

    def health(self) -> Dict[str, Any]:
        return {
            "status": "ok" if self._running else "stopped",
            "workers": len(self._workers),
            "registered_services": list(self._handlers.keys()),
            "queue_size": self._queue.qsize(),
            "uptime_seconds": int(time.time() - self.start_time) if self.start_time else 0,
        }
