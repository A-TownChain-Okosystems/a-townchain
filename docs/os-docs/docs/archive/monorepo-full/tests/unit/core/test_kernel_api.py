# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# Kernel API Tests — ATC-97 | Sprint 3.2
# Tests the complete KAI-OS Kernel API for Decentralized AI Operating System

import sys
import os
import pytest

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'src'))

from core.kernel.api import (
    KernelAPI, Syscall, SyscallRequest, SyscallResponse,
    AgentStatus, DecisionStatus, GasReport, AgentDescriptor,
    AIDecision, ValidatorInfo, FederatedTask, MemoryRegion,
    KernelAPIError,
)
from core.kernel.kernel import ProcessType, ProcessState


@pytest.fixture
def api():
    """Fresh KernelAPI instance per test."""
    return KernelAPI()


# ════════════════════════════════════════════════════════════════
#  PROCESS MANAGEMENT TESTS
# ════════════════════════════════════════════════════════════════

class TestProcessManagement:
    def test_spawn_creates_process(self, api):
        pid = api.sys_spawn("test_agent", ProcessType.AGENT, owner="ATC1", priority=100)
        assert pid > 0
        info = api.sys_process_info(pid)
        assert info["name"] == "test_agent"
        assert info["type"] == "AGENT"

    def test_spawn_creates_gas_ledger(self, api):
        pid = api.sys_spawn("proc", ProcessType.SERVICE, owner="ATC2")
        report = api.sys_gas_report(pid)
        assert report.pid == pid
        assert report.gas_limit > 0

    def test_kill_terminates_process(self, api):
        pid = api.sys_spawn("victim", ProcessType.AGENT, owner="ATC3")
        assert api.sys_kill(pid, 9) is True
        info = api.sys_process_info(pid)
        assert info["state"] == "KILLED"

    def test_kill_nonexistent_returns_false(self, api):
        assert api.sys_kill(9999, 9) is False

    def test_sleep_and_wake(self, api):
        pid = api.sys_spawn("sleeper", ProcessType.AGENT, owner="ATC4")
        assert api.sys_sleep(pid, 100) is True
        assert api.sys_process_info(pid)["state"] == "WAITING"
        assert api.sys_wake(pid) is True
        assert api.sys_process_info(pid)["state"] == "RUNNING"

    def test_wake_non_sleeping_returns_false(self, api):
        pid = api.sys_spawn("active", ProcessType.AGENT, owner="ATC5")
        assert api.sys_wake(pid) is False

    def test_set_priority(self, api):
        pid = api.sys_spawn("prio_test", ProcessType.AGENT, owner="ATC6", priority=100)
        assert api.sys_set_priority(pid, 200) is True
        assert api.sys_process_info(pid)["priority"] == 200

    def test_set_priority_clamps_to_range(self, api):
        pid = api.sys_spawn("clamp", ProcessType.AGENT, owner="ATC7")
        api.sys_set_priority(pid, 300)
        assert api.sys_process_info(pid)["priority"] == 255
        api.sys_set_priority(pid, -10)
        assert api.sys_process_info(pid)["priority"] == 0

    def test_process_list_returns_all(self, api):
        initial_count = len(api.sys_process_list())
        api.sys_spawn("extra1", ProcessType.AGENT, owner="ATC8")
        api.sys_spawn("extra2", ProcessType.SERVICE, owner="ATC9")
        procs = api.sys_process_list()
        assert len(procs) == initial_count + 2


# ════════════════════════════════════════════════════════════════
#  MEMORY MANAGEMENT TESTS
# ════════════════════════════════════════════════════════════════

class TestMemoryManagement:
    def test_alloc_creates_region(self, api):
        pid = api.sys_spawn("mem_test", ProcessType.AGENT, owner="ATC10")
        rid = api.sys_alloc(pid, 8192)
        assert rid > 0

    def test_free_releases_region(self, api):
        pid = api.sys_spawn("mem_free", ProcessType.AGENT, owner="ATC11")
        rid = api.sys_alloc(pid, 4096)
        assert api.sys_free(rid) is True
        assert api.sys_free(rid) is False

    def test_mem_share(self, api):
        pid1 = api.sys_spawn("sharer", ProcessType.AGENT, owner="ATC12")
        pid2 = api.sys_spawn("sharee", ProcessType.AGENT, owner="ATC13")
        rid = api.sys_alloc(pid1, 4096)
        assert api.sys_mem_share(rid, pid2) is True

    def test_mem_protect_prevents_share(self, api):
        pid1 = api.sys_spawn("owner", ProcessType.AGENT, owner="ATC14")
        pid2 = api.sys_spawn("intruder", ProcessType.AGENT, owner="ATC15")
        rid = api.sys_alloc(pid1, 4096)
        assert api.sys_mem_protect(rid) is True
        assert api.sys_mem_share(rid, pid2) is False

    def test_alloc_rejects_oversize(self, api):
        pid = api.sys_spawn("big", ProcessType.AGENT, owner="ATC16")
        with pytest.raises(KernelAPIError, match="Memory request too large"):
            api.sys_alloc(pid, 512 * 1024 * 1024)  # 512 MB > 256 MB limit


# ════════════════════════════════════════════════════════════════
#  IPC TESTS
# ════════════════════════════════════════════════════════════════

class TestIPC:
    def test_channel_create(self, api):
        pid = api.sys_spawn("ipc_test", ProcessType.AGENT, owner="ATC17")
        cid = api.sys_chan_create(pid, "test_msg", 32)
        assert cid > 0

    def test_channel_send_recv(self, api):
        pid = api.sys_spawn("sender", ProcessType.AGENT, owner="ATC18")
        cid = api.sys_chan_create(pid, "data", 64)
        assert api.sys_chan_send(cid, pid, "msg", {"key": "value"}) is True
        msg = api.sys_chan_recv(cid, pid)
        assert msg is not None
        assert msg.data == {"key": "value"}

    def test_channel_close(self, api):
        pid = api.sys_spawn("closer", ProcessType.AGENT, owner="ATC19")
        cid = api.sys_chan_create(pid, "temp", 8)
        assert api.sys_chan_close(cid) is True
        assert api.sys_chan_close(cid) is False

    def test_channel_subscribe(self, api):
        pid1 = api.sys_spawn("pub", ProcessType.AGENT, owner="ATC20")
        pid2 = api.sys_spawn("sub", ProcessType.AGENT, owner="ATC21")
        cid = api.sys_chan_create(pid1, "broadcast", 64)
        assert api.sys_chan_subscribe(cid, pid2) is True


# ════════════════════════════════════════════════════════════════
#  AI KERNEL TESTS
# ════════════════════════════════════════════════════════════════

class TestAIKernel:
    def test_ai_route_returns_correct_model(self, api):
        assert api.sys_ai_route("reasoning") == "mistral-7b"
        assert api.sys_ai_route("code") == "phi-2"
        assert api.sys_ai_route("summarize") == "llama-3.2-3b"
        assert api.sys_ai_route("unknown") == "gemma-2-2b"

    def test_ai_infer_creates_decision(self, api):
        result, tokens = api.sys_ai_infer("ATC_agent", "code", "def f():", "phi-2", 512)
        assert result == "queued"
        assert tokens == 512
        assert api.total_ai_requests == 1

    def test_ai_decision_full_lifecycle(self, api):
        # Create
        did = api.sys_ai_decision("ATC_agent", "reasoning", "input", "output",
                                   "mistral-7b", "logical deduction", 0.92)
        assert did  # non-empty string

        # Approve
        assert api.sys_ai_approve(did) is True
        d = api.decisions[did]
        assert d.status == DecisionStatus.APPROVED

        # Execute
        assert api.sys_ai_execute(did, 100) is True
        d = api.decisions[did]
        assert d.status == DecisionStatus.EXECUTED
        assert d.block_height == 100

    def test_ai_reject(self, api):
        did = api.sys_ai_decision("ATC_agent", "qa", "q?", "a!", "gemma-2-2b", "", 0.5)
        assert api.sys_ai_reject(did, "low confidence") is True
        assert api.decisions[did].status == DecisionStatus.REJECTED

    def test_ai_approve_nonexistent_returns_false(self, api):
        assert api.sys_ai_approve("nonexistent") is False

    def test_ai_execute_without_approval_returns_false(self, api):
        did = api.sys_ai_decision("agent", "code", "in", "out", "phi-2")
        assert api.sys_ai_execute(did, 1) is False  # still PENDING

    def test_ai_audit_returns_agent_decisions(self, api):
        api.sys_ai_decision("agent_a", "code", "in1", "out1", "phi-2")
        api.sys_ai_decision("agent_b", "qa", "in2", "out2", "gemma-2-2b")
        api.sys_ai_decision("agent_a", "reasoning", "in3", "out3", "mistral-7b")
        audit_a = api.sys_ai_audit("agent_a")
        audit_b = api.sys_ai_audit("agent_b")
        assert len(audit_a) == 2
        assert len(audit_b) == 1

    def test_ai_infer_caps_tokens(self, api):
        _, tokens = api.sys_ai_infer("agent", "code", "input", "model", 8192)
        assert tokens == 4096  # capped


# ════════════════════════════════════════════════════════════════
#  CAPABILITIES TESTS
# ════════════════════════════════════════════════════════════════

class TestCapabilities:
    def test_cap_grant_returns_id(self, api):
        pid = api.sys_spawn("cap_test", ProcessType.AGENT, owner="ATC22")
        cap_id = api.sys_cap_grant(pid, "memory", 123, "READ")
        assert cap_id and len(cap_id) > 0

    def test_cap_check_valid(self, api):
        pid = api.sys_spawn("cap_check", ProcessType.AGENT, owner="ATC23")
        cap_id = api.sys_cap_grant(pid, "ipc_channel", 456, "WRITE")
        assert api.sys_cap_check(cap_id, "WRITE") is True

    def test_cap_check_wrong_right(self, api):
        pid = api.sys_spawn("cap_wrong", ProcessType.AGENT, owner="ATC24")
        cap_id = api.sys_cap_grant(pid, "memory", 789, "READ")
        assert api.sys_cap_check(cap_id, "WRITE") is False

    def test_cap_revoke(self, api):
        pid = api.sys_spawn("cap_revoke", ProcessType.AGENT, owner="ATC25")
        cap_id = api.sys_cap_grant(pid, "device", 999, "ALL")
        assert api.sys_cap_revoke(cap_id, "no longer needed") is True


# ════════════════════════════════════════════════════════════════
#  AGENT LIFECYCLE TESTS
# ════════════════════════════════════════════════════════════════

class TestAgentLifecycle:
    def test_agent_register(self, api):
        agent_id = api.sys_agent_register("ATC_owner", "Aurora",
                                           "https://api.model.ai", 50000,
                                           ["read", "write", "execute"])
        assert agent_id
        agent = api.agents[agent_id]
        assert agent.name == "Aurora"
        assert agent.status == AgentStatus.REGISTERED
        assert agent.stake == 50000
        assert agent.reputation == 100

    def test_agent_deregister(self, api):
        agent_id = api.sys_agent_register("ATC_owner", "Temp", "", 1000)
        assert api.sys_agent_deregister(agent_id, "done") is True
        assert api.agents[agent_id].status == AgentStatus.TERMINATED

    def test_agent_migrate(self, api):
        agent_id = api.sys_agent_register("ATC_owner", "Migrator", "", 1000)
        assert api.sys_agent_migrate(agent_id, 5) is True
        assert api.agents[agent_id].node_id == 5
        assert api.agents[agent_id].status == AgentStatus.MIGRATING

    def test_agent_snapshot(self, api):
        agent_id = api.sys_agent_register("ATC_owner", "Snapshot", "", 1000)
        snap = api.sys_agent_snapshot(agent_id)
        assert snap and len(snap) > 0

    def test_agent_deregister_nonexistent(self, api):
        assert api.sys_agent_deregister("nonexistent") is False


# ════════════════════════════════════════════════════════════════
#  CONSENSUS TESTS
# ════════════════════════════════════════════════════════════════

class TestConsensus:
    def test_validator_register(self, api):
        assert api.sys_validator_register("ATC_val1", 15000) is True
        v = api.validators["ATC_val1"]
        assert v.stake == 15000
        assert v.active is True

    def test_validator_register_low_stake_fails(self, api):
        with pytest.raises(KernelAPIError, match="Minimum stake"):
            api.sys_validator_register("ATC_val2", 5000)

    def test_validator_vote(self, api):
        api.sys_validator_register("ATC_val3", 20000)
        assert api.sys_validator_vote("ATC_val3", "proposal_123", True) is True
        assert api.validators["ATC_val3"].last_vote > 0

    def test_validator_vote_nonexistent(self, api):
        assert api.sys_validator_vote("ATC_nonexist", "prop", True) is False

    def test_fork_resolve_heaviest(self, api):
        assert api.sys_fork_resolve(100, 95) == 100
        assert api.sys_fork_resolve(80, 120) == 120
        assert api.sys_fork_resolve(50, 50) == 50

    def test_checkpoint(self, api):
        cp = api.sys_checkpoint(1000)
        assert cp and len(cp) > 0


# ════════════════════════════════════════════════════════════════
#  RESOURCE MANAGEMENT TESTS
# ════════════════════════════════════════════════════════════════

class TestResourceManagement:
    def test_stake_lock_unlock(self, api):
        pid = api.sys_spawn("staker", ProcessType.AGENT, owner="ATC30")
        assert api.sys_stake_lock(pid, 5000) is True
        assert api.sys_process_info(pid)["stake"] == 5000
        assert api.sys_stake_unlock(pid, 2000) is True
        assert api.sys_process_info(pid)["stake"] == 3000

    def test_stake_unlock_insufficient(self, api):
        pid = api.sys_spawn("poor", ProcessType.AGENT, owner="ATC31")
        api.sys_stake_lock(pid, 1000)
        assert api.sys_stake_unlock(pid, 2000) is False

    def test_resource_limits(self, api):
        max_procs, max_mem, gas_per_ms = api.sys_resource_limits()
        assert max_procs == 1024
        assert max_mem == 256 * 1024 * 1024
        assert gas_per_ms == 100

    def test_gas_report(self, api):
        pid = api.sys_spawn("gas_test", ProcessType.AGENT, owner="ATC32")
        report = api.sys_gas_report(pid)
        assert report.gas_limit > 0
        assert report.gas_remaining > 0


# ════════════════════════════════════════════════════════════════
#  DISTRIBUTED INTELLIGENCE TESTS
# ════════════════════════════════════════════════════════════════

class TestDistributedIntelligence:
    def test_federated_train(self, api):
        task_id = api.sys_federated_train("model_hash_123",
                                           ["ATC1", "ATC2", "ATC3"], 10)
        assert task_id
        task = api.federated_tasks[task_id]
        assert task.round == 0
        assert task.max_rounds == 10
        assert len(task.participants) == 3

    def test_model_sync(self, api):
        task_id = api.sys_federated_train("model_hash", ["ATC1", "ATC2"], 5)
        assert api.sys_model_sync(task_id, "grad_1", 1) is True
        assert api.federated_tasks[task_id].round == 1
        assert api.federated_tasks[task_id].status == "training"

    def test_model_sync_wrong_round(self, api):
        task_id = api.sys_federated_train("model", ["ATC1"], 5)
        assert api.sys_model_sync(task_id, "grad", 5) is False  # round 0→5 skip

    def test_model_sync_completion(self, api):
        task_id = api.sys_federated_train("model", ["ATC1"], 2)
        api.sys_model_sync(task_id, "g1", 1)
        assert api.sys_model_sync(task_id, "g2", 2) is True
        assert api.federated_tasks[task_id].status == "completed"

    def test_knowledge_transfer(self, api):
        a1 = api.sys_agent_register("ATC_a1", "Agent1", "", 1000)
        a2 = api.sys_agent_register("ATC_a2", "Agent2", "", 1000)
        assert api.sys_knowledge_transfer(a1, a2, "knowledge_hash") is True

    def test_knowledge_transfer_nonexistent(self, api):
        assert api.sys_knowledge_transfer("nope1", "nope2", "kh") is False

    def test_neural_mesh_join_leave(self, api):
        agent_id = api.sys_agent_register("ATC_a3", "MeshAgent", "", 1000)
        assert api.sys_neural_mesh_join(agent_id, 42) is True
        assert api.agents[agent_id].node_id == 42
        assert api.sys_neural_mesh_leave(agent_id) is True
        assert api.agents[agent_id].node_id == 0


# ════════════════════════════════════════════════════════════════
#  UNIFIED SYSCALL DISPATCH TESTS
# ════════════════════════════════════════════════════════════════

class TestUnifiedSyscall:
    def test_syscall_named_kernel_stats(self, api):
        resp = api.syscall_named("kernel_stats")
        assert resp.success is True
        stats = resp.data
        assert "version" in stats
        assert "total_processes" in stats
        assert "agent_count" in stats

    def test_syscall_dispatch_success(self, api):
        resp = api.syscall(SyscallRequest(
            syscall=Syscall.AI_ROUTE,
            args=["code"]
        ))
        assert resp.success is True
        assert resp.data == "phi-2"

    def test_syscall_dispatch_unknown(self, api):
        # All syscalls are registered, so test with invalid args
        resp = api.syscall(SyscallRequest(
            syscall=Syscall.PROCESS_INFO,
            args=[99999]  # nonexistent PID
        ))
        assert resp.success is True  # returns None, not error
        assert resp.data is None

    def test_syscall_gas_accounting(self, api):
        pid = api.sys_spawn("gas_test", ProcessType.AGENT, owner="ATC40")
        resp = api.syscall(SyscallRequest(
            syscall=Syscall.KERNEL_STATS,
            caller_pid=pid,
        ))
        assert resp.gas_used > 0
        report = api.sys_gas_report(pid)
        assert report.gas_used > 0

    def test_syscall_kernel_log(self, api):
        api.sys_spawn("loggen", ProcessType.AGENT, owner="ATC41")
        logs = api.sys_kernel_log(5)
        assert len(logs) > 0
        assert "ts" in logs[0]
        assert "msg" in logs[0]


# ════════════════════════════════════════════════════════════════
#  KERNEL STATS TESTS
# ════════════════════════════════════════════════════════════════

class TestKernelStats:
    def test_stats_initial_state(self, api):
        stats = api.sys_kernel_stats()
        assert stats["version"] == "1.0.0-alpha"
        assert stats["total_processes"] >= 3  # kernel boot procs
        assert stats["agent_count"] == 0
        assert stats["validator_count"] == 0
        assert stats["ai_requests"] == 0

    def test_stats_reflect_activity(self, api):
        api.sys_spawn("proc", ProcessType.AGENT, owner="ATC42")
        api.sys_agent_register("ATC42", "Agent", "", 1000)
        api.sys_validator_register("ATC_val", 20000)
        api.sys_ai_infer("agent", "code", "test", "phi-2", 100)

        stats = api.sys_kernel_stats()
        assert stats["total_processes"] > 3
        assert stats["agent_count"] == 1
        assert stats["validator_count"] == 1
        assert stats["ai_requests"] == 1
        assert stats["ai_tokens_used"] == 100

    def test_stats_tracks_decisions(self, api):
        did = api.sys_ai_decision("agent", "task", "in", "out", "model")
        stats = api.sys_kernel_stats()
        assert stats["decisions_pending"] == 1
        assert stats["decisions_executed"] == 0

        api.sys_ai_approve(did)
        api.sys_ai_execute(did, 1)
        stats = api.sys_kernel_stats()
        assert stats["decisions_pending"] == 0
        assert stats["decisions_executed"] == 1
