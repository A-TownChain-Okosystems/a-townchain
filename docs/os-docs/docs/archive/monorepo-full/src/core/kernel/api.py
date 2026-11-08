# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
KAI-OS Kernel API — Runtime Implementation
=========================================
Dezentrales KI-Betriebssystem Kernel API Layer
Version: 1.0.0-alpha | ATC-97 | Sprint 3.2

Diese Module implementiert die in modules/kernel/kernel_api.atc
spezifizierte Kernel API als ausführbare Python-Runtime.

Architektur:
    ┌────────────────────────────────────────────────┐
    │              Kernel API (api.py)                 │
    ├──────────┬──────────┬───────────┬──────────────┤
    │ Process  │ Memory   │ IPC       │ AI Kernel    │
    │ Mgr     │ Mgr      │ Bus       │ Orchestrator  │
    ├──────────┼──────────┼───────────┼──────────────┤
    │ Capabil. │ Consensus│ Resource  │ Distributed   │
    │ Mgr     │ Layer    │ Monitor   │ Intelligence  │
    └──────────┴──────────┴───────────┴──────────────┘
                      │
                GCL Core (13 Buses)
"""

import time
import hashlib
import threading
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Callable
from enum import IntEnum, auto
from dataclasses import dataclass

from src.core.kernel.kernel import ShivaKernel, KernelProcess, ProcessState, ProcessType
from src.core.kernel.capabilities import CapabilityManager, Right, ResourceType, CapabilityError


# ════════════════════════════════════════════════════════════════
#  SYSCALL NUMBERS (Stable ABI)
# ════════════════════════════════════════════════════════════════

class Syscall(IntEnum):
    # Process Management (0x01-0x0F)
    SPAWN = 0x01
    KILL = 0x02
    WAIT = 0x03
    SIGNAL = 0x04
    SLEEP = 0x05
    WAKE = 0x06
    YIELD = 0x07
    PROCESS_LIST = 0x08
    PROCESS_INFO = 0x09
    SET_PRIORITY = 0x0A

    # Memory Management (0x10-0x1F)
    ALLOC = 0x10
    FREE = 0x11
    MEM_READ = 0x12
    MEM_WRITE = 0x13
    MEM_SHARE = 0x14
    MEM_PROTECT = 0x15

    # IPC (0x20-0x2F)
    CHAN_CREATE = 0x20
    CHAN_SEND = 0x21
    CHAN_RECV = 0x22
    CHAN_CLOSE = 0x23
    CHAN_SUBSCRIBE = 0x24
    CHAN_BROADCAST = 0x25

    # AI Kernel (0x30-0x3F)
    AI_ROUTE = 0x30
    AI_INFER = 0x31
    AI_DECISION = 0x32
    AI_APPROVE = 0x33
    AI_EXECUTE = 0x34
    AI_REJECT = 0x35
    AI_AUDIT = 0x36

    # Capabilities (0x40-0x4F)
    CAP_GRANT = 0x40
    CAP_REVOKE = 0x41
    CAP_DELEGATE = 0x42
    CAP_CHECK = 0x43

    # Consensus (0x50-0x5F)
    VALIDATOR_REGISTER = 0x50
    VALIDATOR_VOTE = 0x51
    FORK_RESOLVE = 0x52
    CHECKPOINT = 0x53

    # Agent Lifecycle (0x60-0x6F)
    AGENT_REGISTER = 0x60
    AGENT_DEREGISTER = 0x61
    AGENT_MIGRATE = 0x62
    AGENT_SNAPSHOT = 0x63

    # Resource (0x70-0x7F)
    GAS_REPORT = 0x70
    STAKE_LOCK = 0x71
    STAKE_UNLOCK = 0x72
    RESOURCE_LIMITS = 0x73

    # Distributed Intelligence (0x80-0x8F)
    FEDERATED_TRAIN = 0x80
    MODEL_SYNC = 0x81
    KNOWLEDGE_TRANSFER = 0x82
    NEURAL_MESH_JOIN = 0x83
    NEURAL_MESH_LEAVE = 0x84

    # Kernel (0x90-0x9F)
    KERNEL_STATS = 0x90
    KERNEL_LOG = 0x91
    KERNEL_SHUTDOWN = 0x92
    KERNEL_REBOOT = 0x93


class AgentStatus(IntEnum):
    REGISTERED = auto()
    ACTIVE = auto()
    SUSPENDED = auto()
    MIGRATING = auto()
    TERMINATED = auto()


class DecisionStatus(IntEnum):
    PENDING = auto()
    APPROVED = auto()
    REJECTED = auto()
    EXECUTED = auto()
    EXPIRED = auto()


# ════════════════════════════════════════════════════════════════
#  DATA STRUCTURES
# ════════════════════════════════════════════════════════════════

@dataclass
class AgentDescriptor:
    agent_id: str
    owner: str
    name: str
    status: AgentStatus = AgentStatus.REGISTERED
    capabilities: List[str] = field(default_factory=list)
    model_endpoint: str = ""
    stake: int = 0
    reputation: int = 100
    last_active: int = 0
    node_id: int = 0


@dataclass
class AIDecision:
    decision_id: str
    agent: str
    task: str
    input_hash: str
    output_hash: str
    model: str = ""
    status: DecisionStatus = DecisionStatus.PENDING
    reasoning: str = ""
    confidence: float = 0.0
    timestamp: int = 0
    block_height: int = 0


@dataclass
class GasReport:
    pid: int
    total_gas: int = 0
    gas_limit: int = 10_000_000
    gas_used: int = 0
    gas_remaining: int = 10_000_000
    refund: int = 0


@dataclass
class ValidatorInfo:
    validator_id: str
    stake: int = 0
    active: bool = True
    uptime: int = 100
    blocks_proposed: int = 0
    blocks_validated: int = 0
    last_vote: int = 0


@dataclass
class FederatedTask:
    task_id: str
    model_hash: str
    participants: List[str] = field(default_factory=list)
    round: int = 0
    max_rounds: int = 10
    status: str = "initializing"
    gradient_hash: str = ""


@dataclass
class MemoryRegion:
    region_id: int
    owner_pid: int
    size: int
    cap_id: str = ""
    protected: bool = False
    shared_with: List[int] = field(default_factory=list)
    data: bytearray = field(default_factory=bytearray)


@dataclass
class SyscallRequest:
    syscall: Syscall
    caller_pid: int = 0
    args: List[Any] = field(default_factory=list)
    cap_id: str = ""


@dataclass
class SyscallResponse:
    success: bool = True
    error: str = ""
    data: Any = None
    gas_used: int = 0


# ════════════════════════════════════════════════════════════════
#  KERNEL API — Zentrale Implementierung
# ════════════════════════════════════════════════════════════════

class KernelAPI:
    """
    KAI-OS Kernel API — Dezentrales KI-Betriebssystem API Layer.
    Bietet einen einheitlichen Syscall-Interface für alle Kernel-Operationen.
    Implementiert ATC-97 Spec (modules/kernel/kernel_api.atc).
    """

    VERSION = "1.0.0-alpha"
    MAX_PROCESSES = 1024
    MAX_MEMORY_PER_PROCESS = 256 * 1024 * 1024  # 256 MB
    GAS_PER_CPU_MS = 100
    MIN_VALIDATOR_STAKE = 10000

    def __init__(self):
        self._kernel = ShivaKernel()
        self._lock = threading.Lock()

        # Extended state
        self.agents: Dict[str, AgentDescriptor] = {}
        self.decisions: Dict[str, AIDecision] = {}
        self.validators: Dict[str, ValidatorInfo] = {}
        self.memory_regions: Dict[int, MemoryRegion] = {}
        self.gas_ledger: Dict[int, GasReport] = {}
        self.federated_tasks: Dict[str, FederatedTask] = {}
        self.event_log: List[dict] = []

        self._next_region = 1
        self._kernel_start = int(time.time() * 1000)
        self.total_ai_requests = 0
        self.total_ai_tokens = 0

        # Build syscall dispatch table
        self._syscall_table: Dict[Syscall, Callable] = {
            Syscall.SPAWN: self.sys_spawn,
            Syscall.KILL: self.sys_kill,
            Syscall.WAIT: self.sys_wait,
            Syscall.SLEEP: self.sys_sleep,
            Syscall.WAKE: self.sys_wake,
            Syscall.PROCESS_LIST: self.sys_process_list,
            Syscall.PROCESS_INFO: self.sys_process_info,
            Syscall.SET_PRIORITY: self.sys_set_priority,
            Syscall.ALLOC: self.sys_alloc,
            Syscall.FREE: self.sys_free,
            Syscall.MEM_SHARE: self.sys_mem_share,
            Syscall.MEM_PROTECT: self.sys_mem_protect,
            Syscall.CHAN_CREATE: self.sys_chan_create,
            Syscall.CHAN_SEND: self.sys_chan_send,
            Syscall.CHAN_RECV: self.sys_chan_recv,
            Syscall.CHAN_CLOSE: self.sys_chan_close,
            Syscall.CHAN_SUBSCRIBE: self.sys_chan_subscribe,
            Syscall.AI_ROUTE: self.sys_ai_route,
            Syscall.AI_INFER: self.sys_ai_infer,
            Syscall.AI_DECISION: self.sys_ai_decision,
            Syscall.AI_APPROVE: self.sys_ai_approve,
            Syscall.AI_EXECUTE: self.sys_ai_execute,
            Syscall.AI_REJECT: self.sys_ai_reject,
            Syscall.AI_AUDIT: self.sys_ai_audit,
            Syscall.CAP_GRANT: self.sys_cap_grant,
            Syscall.CAP_REVOKE: self.sys_cap_revoke,
            Syscall.CAP_CHECK: self.sys_cap_check,
            Syscall.VALIDATOR_REGISTER: self.sys_validator_register,
            Syscall.VALIDATOR_VOTE: self.sys_validator_vote,
            Syscall.FORK_RESOLVE: self.sys_fork_resolve,
            Syscall.CHECKPOINT: self.sys_checkpoint,
            Syscall.AGENT_REGISTER: self.sys_agent_register,
            Syscall.AGENT_DEREGISTER: self.sys_agent_deregister,
            Syscall.AGENT_MIGRATE: self.sys_agent_migrate,
            Syscall.AGENT_SNAPSHOT: self.sys_agent_snapshot,
            Syscall.GAS_REPORT: self.sys_gas_report,
            Syscall.STAKE_LOCK: self.sys_stake_lock,
            Syscall.STAKE_UNLOCK: self.sys_stake_unlock,
            Syscall.RESOURCE_LIMITS: self.sys_resource_limits,
            Syscall.FEDERATED_TRAIN: self.sys_federated_train,
            Syscall.MODEL_SYNC: self.sys_model_sync,
            Syscall.KNOWLEDGE_TRANSFER: self.sys_knowledge_transfer,
            Syscall.NEURAL_MESH_JOIN: self.sys_neural_mesh_join,
            Syscall.NEURAL_MESH_LEAVE: self.sys_neural_mesh_leave,
            Syscall.KERNEL_STATS: self.sys_kernel_stats,
            Syscall.KERNEL_LOG: self.sys_kernel_log,
            Syscall.KERNEL_SHUTDOWN: self.sys_kernel_shutdown,
        }

    # ═══════════════════════════════════════════════════════════
    #  PROCESS MANAGEMENT
    # ═══════════════════════════════════════════════════════════

    def sys_spawn(self, name: str, ptype: ProcessType = ProcessType.AGENT,
                  owner: str = "", priority: int = 128,
                  mem_size: int = 4*1024*1024, stake: int = 0,
                  fn: Optional[Callable] = None, **kw) -> int:
        """SPAWN: Neuen Prozess starten."""
        if len(self._kernel.processes) >= self.MAX_PROCESSES:
            raise KernelAPIError("Max processes reached")
        if mem_size > self.MAX_MEMORY_PER_PROCESS:
            raise KernelAPIError("Memory limit exceeded")

        pid = self._kernel.spawn(name=name, ptype=ptype, fn=fn,
                                  owner=owner, stake=stake, priority=priority,
                                  mem_size=mem_size)
        self.gas_ledger[pid] = GasReport(pid=pid)
        self._log(f"SPAWN pid={pid} name={name} type={ptype.name}")
        return pid

    def sys_kill(self, pid: int, signal: int = 9) -> bool:
        """KILL: Prozess beenden."""
        ok = self._kernel.kill(pid, signal)
        if ok:
            self._log(f"KILL pid={pid} signal={signal}")
        return ok

    def sys_wait(self, pid: int, timeout_ms: int = 5000) -> int:
        """WAIT: Auf Prozessende warten."""
        return self._kernel.wait(pid, timeout_ms)

    def sys_sleep(self, pid: int, duration_ms: int = 0) -> bool:
        """SLEEP: Prozess schlafen legen."""
        proc = self._kernel.processes.get(pid)
        if not proc:
            return False
        proc.state = ProcessState.WAITING
        self._log(f"SLEEP pid={pid} duration={duration_ms}ms")
        return True

    def sys_wake(self, pid: int) -> bool:
        """WAKE: Prozess wecken."""
        proc = self._kernel.processes.get(pid)
        if not proc or proc.state != ProcessState.WAITING:
            return False
        proc.state = ProcessState.RUNNING
        self._log(f"WAKE pid={pid}")
        return True

    def sys_process_list(self) -> List[dict]:
        """PROCESS_LIST: Alle Prozesse auflisten."""
        return self._kernel.list_processes()

    def sys_process_info(self, pid: int) -> Optional[dict]:
        """PROCESS_INFO: Prozess-Info abfragen."""
        return self._kernel.process_info(pid)

    def sys_set_priority(self, pid: int, priority: int) -> bool:
        """SET_PRIORITY: Prozess-Priorität setzen."""
        proc = self._kernel.processes.get(pid)
        if not proc:
            return False
        proc.priority = max(0, min(255, priority))
        return True

    # ═══════════════════════════════════════════════════════════
    #  MEMORY MANAGEMENT
    # ═══════════════════════════════════════════════════════════

    def sys_alloc(self, pid: int, size: int) -> int:
        """ALLOC: Speicher allokieren. Gibt region_id zurück."""
        if size > self.MAX_MEMORY_PER_PROCESS:
            raise KernelAPIError(f"Memory request too large: {size}")
        with self._lock:
            region_id = self._next_region
            self._next_region += 1
        cap_id = hashlib.sha256(f"{region_id}{pid}{time.time()}".encode()).hexdigest()[:16]
        region = MemoryRegion(region_id=region_id, owner_pid=pid, size=size,
                              cap_id=cap_id, data=bytearray(size))
        self.memory_regions[region_id] = region
        self._log(f"ALLOC region={region_id} pid={pid} size={size}")
        return region_id

    def sys_free(self, region_id: int) -> bool:
        """FREE: Speicher freigeben."""
        region = self.memory_regions.get(region_id)
        if not region:
            return False
        del self.memory_regions[region_id]
        self._log(f"FREE region={region_id} pid={region.owner_pid}")
        return True

    def sys_mem_share(self, region_id: int, target_pid: int) -> bool:
        """MEM_SHARE: Speicher mit anderem Prozess teilen."""
        region = self.memory_regions.get(region_id)
        if not region or region.protected:
            return False
        if target_pid not in region.shared_with:
            region.shared_with.append(target_pid)
        return True

    def sys_mem_protect(self, region_id: int) -> bool:
        """MEM_PROTECT: Speicherregion schützen."""
        region = self.memory_regions.get(region_id)
        if not region:
            return False
        region.protected = True
        return True

    # ═══════════════════════════════════════════════════════════
    #  IPC
    # ═══════════════════════════════════════════════════════════

    def sys_chan_create(self, sender_pid: int, msg_type: str = "",
                        buffer_size: int = 64) -> int:
        """CHAN_CREATE: IPC-Kanal erstellen."""
        return self._kernel.create_channel(sender_pid=sender_pid, buffer=buffer_size)

    def sys_chan_send(self, channel_id: int, from_pid: int,
                      msg_type: str, data: Any) -> bool:
        """CHAN_SEND: Nachricht senden."""
        return self._kernel.channel_send(channel_id, from_pid, msg_type, data)

    def sys_chan_recv(self, channel_id: int, receiver_pid: int) -> Optional[Any]:
        """CHAN_RECV: Nachricht empfangen."""
        return self._kernel.channel_recv(channel_id, receiver_pid)

    def sys_chan_close(self, channel_id: int) -> bool:
        """CHAN_CLOSE: Kanal schließen."""
        if channel_id in self._kernel.channels:
            del self._kernel.channels[channel_id]
            self._log(f"CHAN_CLOSE cid={channel_id}")
            return True
        return False

    def sys_chan_subscribe(self, channel_id: int, subscriber_pid: int) -> bool:
        """CHAN_SUBSCRIBE: Kanal abonnieren."""
        self._kernel.subscribe_broadcast(channel_id, subscriber_pid)
        return True

    # ═══════════════════════════════════════════════════════════
    #  AI KERNEL
    # ═══════════════════════════════════════════════════════════

    def sys_ai_route(self, task: str) -> str:
        """AI_ROUTE: Modell für Task-Typ routen."""
        routing = {
            "reasoning": "mistral-7b",
            "code": "phi-2",
            "summarize": "llama-3.2-3b",
            "qa": "llama-3.2-3b",
            "text": "gemma-2-2b",
        }
        return routing.get(task, "gemma-2-2b")

    def sys_ai_infer(self, agent: str, task: str, input_text: str,
                     model: str = "", max_tokens: int = 2048) -> tuple:
        """AI_INFER: Inference-Anfrage stellen."""
        self.total_ai_requests += 1
        effective_tokens = min(max_tokens, 4096)
        self.total_ai_tokens += effective_tokens

        decision_id = hashlib.sha256(
            f"{agent}{task}{input_text}{time.time()}".encode()
        ).hexdigest()[:16]

        decision = AIDecision(
            decision_id=decision_id,
            agent=agent,
            task=task,
            input_hash=hashlib.sha256(input_text.encode()).hexdigest()[:16],
            output_hash=hashlib.sha256(decision_id.encode()).hexdigest()[:16],
            model=model or self.sys_ai_route(task),
            status=DecisionStatus.PENDING,
            confidence=0.0,
            timestamp=int(time.time() * 1000),
        )
        self.decisions[decision_id] = decision
        self._log(f"AI_INFER id={decision_id} agent={agent} model={decision.model}")
        return ("queued", effective_tokens)

    def sys_ai_decision(self, agent: str, task: str, input_text: str,
                        output_text: str, model: str = "",
                        reasoning: str = "", confidence: float = 0.0) -> str:
        """AI_DECISION: Entscheidung erfassen."""
        decision_id = hashlib.sha256(
            f"{agent}{task}{time.time()}".encode()
        ).hexdigest()[:16]
        decision = AIDecision(
            decision_id=decision_id,
            agent=agent,
            task=task,
            input_hash=hashlib.sha256(input_text.encode()).hexdigest()[:16],
            output_hash=hashlib.sha256(output_text.encode()).hexdigest()[:16],
            model=model,
            status=DecisionStatus.PENDING,
            reasoning=reasoning,
            confidence=confidence,
            timestamp=int(time.time() * 1000),
        )
        self.decisions[decision_id] = decision
        self._log(f"AI_DECISION id={decision_id} agent={agent} status=PENDING")
        return decision_id

    def sys_ai_approve(self, decision_id: str, approver: str = "") -> bool:
        """AI_APPROVE: Entscheidung genehmigen."""
        d = self.decisions.get(decision_id)
        if not d or d.status != DecisionStatus.PENDING:
            return False
        d.status = DecisionStatus.APPROVED
        self._log(f"AI_APPROVE id={decision_id} by={approver}")
        return True

    def sys_ai_execute(self, decision_id: str, block_height: int = 0) -> bool:
        """AI_EXECUTE: Entscheidung ausführen."""
        d = self.decisions.get(decision_id)
        if not d or d.status != DecisionStatus.APPROVED:
            return False
        d.status = DecisionStatus.EXECUTED
        d.block_height = block_height
        self._log(f"AI_EXECUTE id={decision_id} block={block_height}")
        return True

    def sys_ai_reject(self, decision_id: str, reason: str = "") -> bool:
        """AI_REJECT: Entscheidung ablehnen."""
        d = self.decisions.get(decision_id)
        if not d:
            return False
        d.status = DecisionStatus.REJECTED
        self._log(f"AI_REJECT id={decision_id} reason={reason}")
        return True

    def sys_ai_audit(self, agent: str) -> List[AIDecision]:
        """AI_AUDIT: Audit-Trail für Agenten abfragen."""
        return [d for d in self.decisions.values() if d.agent == agent]

    # ═══════════════════════════════════════════════════════════
    #  CAPABILITIES
    # ═══════════════════════════════════════════════════════════

    def sys_cap_grant(self, owner_pid: int, resource_type: str,
                      resource_id: int, rights: str = "ALL") -> str:
        """CAP_GRANT: Capability vergeben."""
        right_map = {
            "READ": Right.READ,
            "WRITE": Right.WRITE,
            "EXECUTE": Right.EXECUTE,
            "DELEGATE": Right.DELEGATE,
            "ALL": Right.ALL,
        }
        right = right_map.get(rights, Right.ALL)
        cap = self._kernel.capabilities.grant(
            owner_pid=owner_pid,
            resource_type=resource_type,
            resource_id=resource_id,
            rights=right,
            issued_by=0,
        )
        self._log(f"CAP_GRANT cap={cap.cap_id[:8]} pid={owner_pid} res={resource_type}:{resource_id}")
        return cap.cap_id

    def sys_cap_revoke(self, cap_id: str, reason: str = "") -> bool:
        """CAP_REVOKE: Capability entziehen."""
        try:
            self._kernel.capabilities.revoke(cap_id)
            self._log(f"CAP_REVOKE cap={cap_id[:8]} reason={reason}")
            return True
        except Exception:
            return False

    def sys_cap_check(self, cap_id: str, required_right: str = "READ") -> bool:
        """CAP_CHECK: Capability prüfen."""
        right_map = {
            "READ": Right.READ,
            "WRITE": Right.WRITE,
            "EXECUTE": Right.EXECUTE,
            "DELEGATE": Right.DELEGATE,
            "ALL": Right.ALL,
        }
        right = right_map.get(required_right, Right.READ)
        try:
            caps = self._kernel.capabilities._caps
            cap = caps.get(cap_id)
            return cap is not None and cap.has(right)
        except Exception:
            return False

    # ═══════════════════════════════════════════════════════════
    #  AGENT LIFECYCLE
    # ═══════════════════════════════════════════════════════════

    def sys_agent_register(self, owner: str, name: str,
                           model_endpoint: str = "", stake: int = 0,
                           capabilities: List[str] = None) -> str:
        """AGENT_REGISTER: Agent registrieren."""
        agent_id = hashlib.sha256(
            f"{owner}{name}{time.time()}".encode()
        ).hexdigest()[:16]
        agent = AgentDescriptor(
            agent_id=agent_id,
            owner=owner,
            name=name,
            status=AgentStatus.REGISTERED,
            capabilities=capabilities or [],
            model_endpoint=model_endpoint,
            stake=stake,
            reputation=100,
            last_active=int(time.time() * 1000),
        )
        self.agents[agent_id] = agent
        self._log(f"AGENT_REGISTER id={agent_id} name={name} owner={owner[:8]}")
        return agent_id

    def sys_agent_deregister(self, agent_id: str, reason: str = "") -> bool:
        """AGENT_DEREGISTER: Agent abmelden."""
        agent = self.agents.get(agent_id)
        if not agent:
            return False
        agent.status = AgentStatus.TERMINATED
        self._log(f"AGENT_DEREGISTER id={agent_id} reason={reason}")
        return True

    def sys_agent_migrate(self, agent_id: str, target_node: int) -> bool:
        """AGENT_MIGRATE: Agent zu anderem Node migrieren."""
        agent = self.agents.get(agent_id)
        if not agent:
            return False
        agent.status = AgentStatus.MIGRATING
        agent.node_id = target_node
        self._log(f"AGENT_MIGRATE id={agent_id} target_node={target_node}")
        return True

    def sys_agent_snapshot(self, agent_id: str) -> str:
        """AGENT_SNAPSHOT: Agent-State sichern."""
        agent = self.agents.get(agent_id)
        if not agent:
            return ""
        snapshot = hashlib.sha256(
            f"{agent.name}{agent.stake}{agent.reputation}{agent.status}".encode()
        ).hexdigest()[:16]
        return snapshot

    # ═══════════════════════════════════════════════════════════
    #  CONSENSUS
    # ═══════════════════════════════════════════════════════════

    def sys_validator_register(self, validator: str, stake: int) -> bool:
        """VALIDATOR_REGISTER: Validator anmelden."""
        if stake < self.MIN_VALIDATOR_STAKE:
            raise KernelAPIError(f"Minimum stake: {self.MIN_VALIDATOR_STAKE} ATC")
        info = ValidatorInfo(
            validator_id=validator,
            stake=stake,
            active=True,
            uptime=100,
        )
        self.validators[validator] = info
        self._log(f"VALIDATOR_REGISTER addr={validator[:8]} stake={stake}")
        return True

    def sys_validator_vote(self, validator: str, proposal_hash: str,
                           vote: bool) -> bool:
        """VALIDATOR_VOTE: Validator-Stimme abgeben."""
        v = self.validators.get(validator)
        if not v or not v.active:
            return False
        v.last_vote = int(time.time() * 1000)
        self._log(f"VALIDATOR_VOTE addr={validator[:8]} proposal={proposal_hash[:8]} vote={vote}")
        return True

    def sys_fork_resolve(self, chain_a: int, chain_b: int) -> int:
        """FORK_RESOLVE: Fork-Auflösung (heaviest chain)."""
        winner = max(chain_a, chain_b)
        self._log(f"FORK_RESOLVE a={chain_a} b={chain_b} winner={winner}")
        return winner

    def sys_checkpoint(self, block_height: int) -> str:
        """CHECKPOINT: State-Checkpoint setzen."""
        cp_hash = hashlib.sha256(
            f"checkpoint_{block_height}_{time.time()}".encode()
        ).hexdigest()[:16]
        self._log(f"CHECKPOINT block={block_height} hash={cp_hash}")
        return cp_hash

    # ═══════════════════════════════════════════════════════════
    #  RESOURCE MANAGEMENT
    # ═══════════════════════════════════════════════════════════

    def sys_gas_report(self, pid: int) -> GasReport:
        """GAS_REPORT: Gas-Verbrauch abfragen."""
        return self.gas_ledger.get(pid, GasReport(pid=pid))

    def sys_stake_lock(self, pid: int, amount: int) -> bool:
        """STAKE_LOCK: Stake hinterlegen."""
        proc = self._kernel.processes.get(pid)
        if not proc:
            return False
        proc.stake += amount
        self._log(f"STAKE_LOCK pid={pid} amount={amount} total={proc.stake}")
        return True

    def sys_stake_unlock(self, pid: int, amount: int) -> bool:
        """STAKE_UNLOCK: Stake freigeben."""
        proc = self._kernel.processes.get(pid)
        if not proc or proc.stake < amount:
            return False
        proc.stake -= amount
        self._log(f"STAKE_UNLOCK pid={pid} amount={amount} remaining={proc.stake}")
        return True

    def sys_resource_limits(self) -> tuple:
        """RESOURCE_LIMITS: System-Limits abfragen."""
        return (self.MAX_PROCESSES, self.MAX_MEMORY_PER_PROCESS, self.GAS_PER_CPU_MS)

    # ═══════════════════════════════════════════════════════════
    #  DISTRIBUTED INTELLIGENCE
    # ═══════════════════════════════════════════════════════════

    def sys_federated_train(self, model_hash: str, participants: List[str],
                            max_rounds: int = 10) -> str:
        """FEDERATED_TRAIN: Federated Learning Task starten."""
        task_id = hashlib.sha256(
            f"{model_hash}{time.time()}".encode()
        ).hexdigest()[:16]
        task = FederatedTask(
            task_id=task_id,
            model_hash=model_hash,
            participants=participants,
            max_rounds=max_rounds,
            status="initializing",
        )
        self.federated_tasks[task_id] = task
        self._log(f"FEDERATED_TRAIN id={task_id} participants={len(participants)}")
        return task_id

    def sys_model_sync(self, task_id: str, gradient_hash: str, round_num: int) -> bool:
        """MODEL_SYNC: Federated Model Round synchronisieren."""
        task = self.federated_tasks.get(task_id)
        if not task or round_num != task.round + 1:
            return False
        task.round = round_num
        task.gradient_hash = gradient_hash
        task.status = "completed" if round_num >= task.max_rounds else "training"
        self._log(f"MODEL_SYNC task={task_id[:8]} round={round_num}/{task.max_rounds}")
        return True

    def sys_knowledge_transfer(self, from_agent: str, to_agent: str,
                                knowledge_hash: str) -> bool:
        """KNOWLEDGE_TRANSFER: Wissen zwischen Agenten übertragen."""
        src = self.agents.get(from_agent)
        dst = self.agents.get(to_agent)
        if not src or not dst:
            return False
        src.last_active = int(time.time() * 1000)
        dst.last_active = int(time.time() * 1000)
        self._log(f"KNOWLEDGE_TRANSFER from={from_agent[:8]} to={to_agent[:8]}")
        return True

    def sys_neural_mesh_join(self, agent_id: str, mesh_id: int) -> bool:
        """NEURAL_MESH_JOIN: Neural Mesh beitreten."""
        agent = self.agents.get(agent_id)
        if not agent:
            return False
        agent.node_id = mesh_id
        self._log(f"NEURAL_MESH_JOIN agent={agent_id[:8]} mesh={mesh_id}")
        return True

    def sys_neural_mesh_leave(self, agent_id: str) -> bool:
        """NEURAL_MESH_LEAVE: Neural Mesh verlassen."""
        agent = self.agents.get(agent_id)
        if not agent:
            return False
        agent.node_id = 0
        self._log(f"NEURAL_MESH_LEAVE agent={agent_id[:8]}")
        return True

    # ═══════════════════════════════════════════════════════════
    #  KERNEL
    # ═══════════════════════════════════════════════════════════

    def sys_kernel_stats(self) -> dict:
        """KERNEL_STATS: System-Statistiken."""
        ks = self._kernel.stats()
        running = sum(1 for p in self._kernel.processes.values()
                      if p.state == ProcessState.RUNNING)
        total_stake = sum(v.stake for v in self.validators.values())
        pending = sum(1 for d in self.decisions.values()
                     if d.status == DecisionStatus.PENDING)
        executed = sum(1 for d in self.decisions.values()
                      if d.status == DecisionStatus.EXECUTED)
        return {
            "version": self.VERSION,
            "uptime_ms": int(time.time() * 1000) - self._kernel_start,
            "total_processes": len(self._kernel.processes),
            "running_processes": running,
            "total_channels": len(self._kernel.channels),
            "total_gas": ks["total_gas"],
            "total_stake": total_stake,
            "validator_count": len(self.validators),
            "agent_count": len(self.agents),
            "ai_requests": self.total_ai_requests,
            "ai_tokens_used": self.total_ai_tokens,
            "decisions_pending": pending,
            "decisions_executed": executed,
            "memory_regions": len(self.memory_regions),
            "federated_tasks": len(self.federated_tasks),
            "syscalls_registered": len(self._syscall_table),
        }

    def sys_kernel_log(self, last_n: int = 20) -> List[dict]:
        """KERNEL_LOG: Kernel-Log abfragen."""
        return self.event_log[-last_n:]

    def sys_kernel_shutdown(self, graceful: bool = True) -> bool:
        """KERNEL_SHUTDOWN: Kernel herunterfahren."""
        self._log(f"KERNEL_SHUTDOWN graceful={graceful}")
        # Stop all processes
        for pid in list(self._kernel.processes.keys()):
            self._kernel.kill(pid)
        return True

    # ═══════════════════════════════════════════════════════════
    #  UNIFIED SYSCALL DISPATCH
    # ═══════════════════════════════════════════════════════════

    def syscall(self, req: SyscallRequest) -> SyscallResponse:
        """Einheitliches Syscall-Interface für alle Kernel-Operationen."""
        handler = self._syscall_table.get(req.syscall)
        if not handler:
            return SyscallResponse(success=False, error=f"Unknown syscall: {req.syscall.name}")

        gas_before = 0
        report = self.gas_ledger.get(req.caller_pid)
        if report:
            gas_before = report.gas_used

        try:
            result = handler(*req.args)
            gas_used = 100  # Base gas per syscall
            if report:
                report.gas_used += gas_used
                report.gas_remaining = max(0, report.gas_limit - report.gas_used)
            self._log(f"SYSCALL {req.syscall.name} pid={req.caller_pid} gas={gas_used} OK")
            return SyscallResponse(success=True, data=result, gas_used=gas_used)
        except Exception as e:
            self._log(f"SYSCALL {req.syscall.name} pid={req.caller_pid} ERROR: {e}")
            return SyscallResponse(success=False, error=str(e), gas_used=100)

    def syscall_named(self, name: str, *args, **kwargs) -> SyscallResponse:
        """Convenience: Syscall per Namen aufrufen."""
        try:
            sc = Syscall[name.upper()]
        except KeyError:
            return SyscallResponse(success=False, error=f"Unknown syscall: {name}")
        return self.syscall(SyscallRequest(syscall=sc, args=list(args)))

    # ═══════════════════════════════════════════════════════════
    #  INTERNAL
    # ═══════════════════════════════════════════════════════════

    def _log(self, msg: str):
        self.event_log.append({"ts": int(time.time() * 1000), "msg": msg})

    @property
    def kernel(self) -> ShivaKernel:
        """Direkter Zugriff auf den underlying ShivaKernel."""
        return self._kernel


class KernelAPIError(Exception):
    """Kernel API Fehler."""
    pass
