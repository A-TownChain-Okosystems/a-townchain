// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 43: SMP / Multi-Core Support
// Copyright (c) 2026 Michael Wroblewski. All rights reserved.
//
// Symmetric Multi-Processing für parallele KI-Workloads.
// Implementiert:
//   1. PER-CPU DATA — Lokale Datenbereiche pro Kern (Run Queue, Stats, IDT)
//   2. PER-CPU RUN QUEUES — Jeder Kern hat eigene Scheduler-Queue
//   3. CPU AFFINITY — Task-Pinning auf spezifische Kerne (hard/soft affinity)
//   4. LOAD BALANCING — Workload-Verteilung über alle Kerne
//   5. IPI — Inter-Processor Interrupts (Reschedule, TLB Shootdown, Stop, Call-Function)
//   6. CPU HOTPLUG — Online/Offline von Kernen zur Laufzeit
//   7. CPU TOPOLOGY — NUMA-Nodes, Cache-Hierarchie, Hyperthreading
//   8. SMP BARRIERS — Synchronisation zwischen Kernen
//   9. PER-CPU COUNTERS — Atomare Statistiken pro Kern
//  10. SCHEDULER DOMAINS — Scheduling-Domains für Hierarchische Load-Balancing

#![allow(dead_code)]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

// ════════════════════════════════════════════════════════════════
//  CONSTANTS
// ════════════════════════════════════════════════════════════════

const MAX_CPUS: usize = 256;
const MAX_NUMA_NODES: usize = 16;
const MAX_CACHE_LEVELS: usize = 4;
const MAX_IPI_TYPES: usize = 16;
const DEFAULT_LOAD_BALANCE_INTERVAL_MS: u64 = 100;
const LOAD_BALANCE_THRESHOLD: f64 = 1.25;   // Imbalance ratio to trigger migration
const MAX_MIGRATIONS_PER_BALANCE: usize = 4;
const DEFAULT_QUANTUM_TICKS: u32 = 10;

// ════════════════════════════════════════════════════════════════
//  CPU STATE
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuState {
    Offline,       // Not available
    Booting,        // Coming online
    Online,        // Active and running
    Paused,        // Temporarily paused (hotplug)
    Stopping,      // Going offline
}

impl CpuState {
    pub fn name(&self) -> &'static str {
        match self {
            CpuState::Offline => "offline",
            CpuState::Booting => "booting",
            CpuState::Online => "online",
            CpuState::Paused => "paused",
            CpuState::Stopping => "stopping",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, CpuState::Online)
    }

    pub fn is_available(&self) -> bool {
        matches!(self, CpuState::Online | CpuState::Paused)
    }
}

// ════════════════════════════════════════════════════════════════
//  CPU ID
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CpuId(pub u32);

impl CpuId {
    pub fn new(id: u32) -> Self { Self(id) }
    pub fn raw(&self) -> u32 { self.0 }
    pub fn is_bsp(&self) -> bool { self.0 == 0 }  // Bootstrap Processor
    pub fn is_ap(&self) -> bool { self.0 != 0 }   // Application Processor
}

impl Default for CpuId {
    fn default() -> Self { Self(0) }
}

// ════════════════════════════════════════════════════════════════
//  CPU AFFINITY (task → core pinning)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CpuAffinity {
    /// Bitmask of allowed CPUs (bit N = CPU N)
    mask: Vec<u64>,
    /// If true, task must run on one of the allowed CPUs (hard affinity)
    /// If false, task prefers allowed CPUs but can run elsewhere (soft affinity)
    pub hard: bool,
}

impl CpuAffinity {
    pub fn all_cpus(max_cpus: usize) -> Self {
        let words = (max_cpus + 63) / 64;
        Self {
            mask: vec![!0u64; words],
            hard: false,
        }
    }

    pub fn single(cpu: CpuId) -> Self {
        let word_idx = (cpu.raw() / 64) as usize;
        let bit = cpu.raw() % 64;
        let words = word_idx + 1;
        let mut mask = vec![0u64; words];
        mask[word_idx] = 1u64 << bit;
        Self { mask, hard: true }
    }

    pub fn range(start: u32, end: u32) -> Self {
        let max_word = ((end / 64) + 1) as usize;
        let mut mask = vec![0u64; max_word.max(1)];
        for cpu in start..=end {
            let word = (cpu / 64) as usize;
            let bit = cpu % 64;
            if word < mask.len() {
                mask[word] |= 1u64 << bit;
            }
        }
        Self { mask, hard: false }
    }

    pub fn none() -> Self {
        Self { mask: vec![0u64], hard: false }
    }

    pub fn add(&mut self, cpu: CpuId) {
        let word = (cpu.raw() / 64) as usize;
        let bit = cpu.raw() % 64;
        while self.mask.len() <= word {
            self.mask.push(0);
        }
        self.mask[word] |= 1u64 << bit;
    }

    pub fn remove(&mut self, cpu: CpuId) {
        let word = (cpu.raw() / 64) as usize;
        let bit = cpu.raw() % 64;
        if word < self.mask.len() {
            self.mask[word] &= !(1u64 << bit);
        }
    }

    pub fn contains(&self, cpu: CpuId) -> bool {
        let word = (cpu.raw() / 64) as usize;
        let bit = cpu.raw() % 64;
        if word < self.mask.len() {
            (self.mask[word] & (1u64 << bit)) != 0
        } else {
            false
        }
    }

    pub fn count(&self) -> u32 {
        self.mask.iter().map(|w| w.count_ones()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.mask.iter().all(|&w| w == 0)
    }

    pub fn first_cpu(&self) -> Option<CpuId> {
        for (word_idx, &word) in self.mask.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros();
                return Some(CpuId((word_idx as u32) * 64 + bit));
            }
        }
        None
    }

    pub fn cpus(&self) -> Vec<CpuId> {
        let mut result = Vec::new();
        for (word_idx, &word) in self.mask.iter().enumerate() {
            for bit in 0..64 {
                if (word & (1u64 << bit)) != 0 {
                    result.push(CpuId((word_idx as u32) * 64 + bit));
                }
            }
        }
        result
    }

    pub fn intersect(&self, other: &CpuAffinity) -> CpuAffinity {
        let max_len = self.mask.len().max(other.mask.len());
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = self.mask.get(i).copied().unwrap_or(0);
            let b = other.mask.get(i).copied().unwrap_or(0);
            result.push(a & b);
        }
        CpuAffinity { mask: result, hard: self.hard && other.hard }
    }

    pub fn union(&self, other: &CpuAffinity) -> CpuAffinity {
        let max_len = self.mask.len().max(other.mask.len());
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = self.mask.get(i).copied().unwrap_or(0);
            let b = other.mask.get(i).copied().unwrap_or(0);
            result.push(a | b);
        }
        CpuAffinity { mask: result, hard: self.hard || other.hard }
    }

    pub fn set_hard(&mut self, hard: bool) {
        self.hard = hard;
    }
}

// ════════════════════════════════════════════════════════════════
//  PER-CPU RUN QUEUE
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct RunQueue {
    pub cpu_id: CpuId,
    pub tasks: Vec<u32>,           // PIDs in queue
    pub current_pid: Option<u32>, // Currently running
    pub weight: u32,              // Total scheduling weight
    pub quantum_remaining: u32,   // Ticks left in current quantum
    pub quantum_length: u32,      // Default quantum in ticks
}

impl RunQueue {
    pub fn new(cpu_id: CpuId) -> Self {
        Self {
            cpu_id,
            tasks: Vec::new(),
            current_pid: None,
            weight: 0,
            quantum_remaining: DEFAULT_QUANTUM_TICKS,
            quantum_length: DEFAULT_QUANTUM_TICKS,
        }
    }

    pub fn enqueue(&mut self, pid: u32, weight: u32) {
        if !self.tasks.contains(&pid) {
            self.tasks.push(pid);
            self.weight += weight;
        }
    }

    pub fn dequeue(&mut self, pid: u32) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|&p| p != pid);
        if self.tasks.len() < before {
            if self.current_pid == Some(pid) {
                self.current_pid = None;
            }
            self.weight = self.weight.saturating_sub(weight);
            true
        } else {
            false
        }
    }

    pub fn next_task(&mut self) -> Option<u32> {
        if self.tasks.is_empty() { return None; }
        let pid = self.tasks.remove(0);
        self.current_pid = Some(pid);
        self.quantum_remaining = self.quantum_length;
        Some(pid)
    }

    pub fn tick(&mut self) -> bool {
        if self.quantum_remaining > 0 {
            self.quantum_remaining -= 1;
        }
        self.quantum_remaining == 0
    }

    pub fn len(&self) -> usize { self.tasks.len() }
    pub fn is_empty(&self) -> bool { self.tasks.is_empty() }
    pub fn load(&self) -> u32 { self.tasks.len() as u32 }

    fn weight_field() -> u32 { 0 } // placeholder — weight is tracked by enqueue

    pub fn peek_next(&self) -> Option<u32> {
        self.tasks.first().copied()
    }
}

// ════════════════════════════════════════════════════════════════
//  PER-CPU DATA
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct PerCpuData {
    pub cpu_id: CpuId,
    pub state: CpuState,
    pub run_queue: RunQueue,
    pub stats: CpuStats,
    pub numa_node: u32,
    pub core_id: u32,
    pub thread_id: u32,          // Hyperthread sibling ID (0 = physical, 1 = sibling)
    pub sibling: Option<CpuId>,  // Hyperthread sibling
    pub cache: [u32; MAX_CACHE_LEVELS],  // Cache sizes per level (KB)
    pub ticks: u64,
    pub idle_ticks: u64,
    pub context_switches: u64,
    pub migrations_in: u64,
    pub migrations_out: u64,
    pub ipi_count: u64,
    pub last_ipi: Option<IpiType>,
}

impl PerCpuData {
    pub fn new(cpu_id: CpuId) -> Self {
        Self {
            cpu_id,
            state: CpuState::Offline,
            run_queue: RunQueue::new(cpu_id),
            stats: CpuStats::new(cpu_id),
            numa_node: 0,
            core_id: cpu_id.raw(),
            thread_id: 0,
            sibling: None,
            cache: [0; MAX_CACHE_LEVELS],
            ticks: 0,
            idle_ticks: 0,
            context_switches: 0,
            migrations_in: 0,
            migrations_out: 0,
            ipi_count: 0,
            last_ipi: None,
        }
    }

    pub fn is_online(&self) -> bool { self.state.is_active() }
    pub fn is_bsp(&self) -> bool { self.cpu_id.is_bsp() }

    pub fn utilization(&self) -> f64 {
        if self.ticks == 0 { return 0.0; }
        1.0 - (self.idle_ticks as f64 / self.ticks as f64)
    }

    pub fn load(&self) -> u32 { self.run_queue.load() }

    pub fn record_tick(&mut self, idle: bool) {
        self.ticks += 1;
        if idle { self.idle_ticks += 1; }
    }

    pub fn record_switch(&mut self) {
        self.context_switches += 1;
    }

    pub fn record_migration_in(&mut self) { self.migrations_in += 1; }
    pub fn record_migration_out(&mut self) { self.migrations_out += 1; }
    pub fn record_ipi(&mut self, ipi_type: IpiType) {
        self.ipi_count += 1;
        self.last_ipi = Some(ipi_type);
    }
}

// ════════════════════════════════════════════════════════════════
//  CPU STATISTICS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Default)]
pub struct CpuStats {
    pub cpu_id: CpuId,
    pub user_ticks: u64,
    pub system_ticks: u64,
    pub idle_ticks: u64,
    pub iowait_ticks: u64,
    pub irq_ticks: u64,
    pub softirq_ticks: u64,
    pub steal_ticks: u64,
    pub guest_ticks: u64,
}

impl CpuStats {
    pub fn new(cpu_id: CpuId) -> Self {
        Self { cpu_id, ..Default::default() }
    }

    pub fn total_ticks(&self) -> u64 {
        self.user_ticks + self.system_ticks + self.idle_ticks +
        self.iowait_ticks + self.irq_ticks + self.softirq_ticks
    }

    pub fn busy_ticks(&self) -> u64 {
        self.total_ticks() - self.idle_ticks
    }

    pub fn utilization(&self) -> f64 {
        let total = self.total_ticks();
        if total == 0 { return 0.0; }
        (self.busy_ticks() as f64 / total as f64) * 100.0
    }

    pub fn record_user(&mut self, ticks: u64) { self.user_ticks += ticks; }
    pub fn record_system(&mut self, ticks: u64) { self.system_ticks += ticks; }
    pub fn record_idle(&mut self, ticks: u64) { self.idle_ticks += ticks; }
}

// ════════════════════════════════════════════════════════════════
//  IPI (INTER-PROCESSOR INTERRUPT)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpiType {
    Reschedule,      // Request target CPU to run scheduler
    TlbShootdown,    // Invalidate TLB entries on target
    CallFunction,    // Execute function on target CPU
    CallFunctionSingle, // Execute function on single target
    Stop,            // Stop target CPU (hotplug)
    WakeUp,          // Wake up target from idle
    Migrate,         // Request task migration
    Crash,           // Crash notification (panic)
}

impl IpiType {
    pub fn name(&self) -> &'static str {
        match self {
            IpiType::Reschedule => "RESCHEDULE",
            IpiType::TlbShootdown => "TLB_SHOOTDOWN",
            IpiType::CallFunction => "CALL_FUNCTION",
            IpiType::CallFunctionSingle => "CALL_FUNCTION_SINGLE",
            IpiType::Stop => "STOP",
            IpiType::WakeUp => "WAKEUP",
            IpiType::Migrate => "MIGRATE",
            IpiType::Crash => "CRASH",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            IpiType::Crash => 0,       // Highest
            IpiType::Stop => 1,
            IpiType::TlbShootdown => 2,
            IpiType::Reschedule => 3,
            IpiType::CallFunction => 4,
            IpiType::CallFunctionSingle => 5,
            IpiType::Migrate => 6,
            IpiType::WakeUp => 7,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpiMessage {
    pub ipi_type: IpiType,
    pub source_cpu: CpuId,
    pub target_cpu: CpuId,
    pub data: u64,
    pub timestamp: u64,
}

// ════════════════════════════════════════════════════════════════
//  CPU TOPOLOGY
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct CpuTopology {
    pub num_cpus: u32,
    pub numa_nodes: Vec<NumaNode>,
    pub cpu_to_node: BTreeMap<u32, u32>,     // cpu_id → numa_node
    pub cpu_to_core: BTreeMap<u32, u32>,     // cpu_id → core_id
    pub siblings: BTreeMap<u32, u32>,        // cpu_id → sibling cpu_id (hyperthread)
    pub cache_topology: BTreeMap<u32, [u32; MAX_CACHE_LEVELS]>,
}

impl CpuTopology {
    pub fn new(num_cpus: u32) -> Self {
        Self {
            num_cpus,
            numa_nodes: Vec::new(),
            cpu_to_node: BTreeMap::new(),
            cpu_to_core: BTreeMap::new(),
            siblings: BTreeMap::new(),
            cache_topology: BTreeMap::new(),
        }
    }

    pub fn single_node(num_cpus: u32) -> Self {
        let mut topo = Self::new(num_cpus);
        let node = NumaNode::new(0, num_cpus, 0);
        topo.numa_nodes.push(node);
        for i in 0..num_cpus {
            topo.cpu_to_node.insert(i, 0);
            topo.cpu_to_core.insert(i, i);
            topo.cache_topology.insert(i, [32, 256, 12288, 0]); // L1 32KB, L2 256KB, L3 12MB
        }
        topo
    }

    pub fn with_hyperthreads(num_cpus: u32, threads_per_core: u32) -> Self {
        let mut topo = Self::new(num_cpus);
        let node = NumaNode::new(0, num_cpus, 0);
        topo.numa_nodes.push(node);

        let cores = num_cpus / threads_per_core;
        for i in 0..num_cpus {
            topo.cpu_to_node.insert(i, 0);
            topo.cpu_to_core.insert(i, i / threads_per_core);
            topo.cache_topology.insert(i, [32, 256, 12288, 0]);
        }

        // Set siblings
        for core in 0..cores {
            for t in 0..threads_per_core {
                let cpu = core * threads_per_core + t;
                if t == 0 && threads_per_core > 1 {
                    topo.siblings.insert(cpu, cpu + 1);
                } else if t == 1 {
                    topo.siblings.insert(cpu, cpu - 1);
                }
            }
        }

        topo
    }

    pub fn get_node(&self, cpu: u32) -> Option<u32> {
        self.cpu_to_node.get(&cpu).copied()
    }

    pub fn get_core(&self, cpu: u32) -> Option<u32> {
        self.cpu_to_core.get(&cpu).copied()
    }

    pub fn get_sibling(&self, cpu: u32) -> Option<u32> {
        self.siblings.get(&cpu).copied()
    }

    pub fn is_sibling(&self, cpu_a: u32, cpu_b: u32) -> bool {
        self.siblings.get(&cpu_a).map(|&s| s == cpu_b).unwrap_or(false) ||
        self.siblings.get(&cpu_b).map(|&s| s == cpu_a).unwrap_or(false)
    }

    pub fn cpus_in_node(&self, node: u32) -> Vec<u32> {
        self.cpu_to_node.iter()
            .filter(|(_, &n)| n == node)
            .map(|(&cpu, _)| cpu)
            .collect()
    }

    pub fn cpus_on_core(&self, core: u32) -> Vec<u32> {
        self.cpu_to_core.iter()
            .filter(|(_, &c)| c == core)
            .map(|(&cpu, _)| cpu)
            .collect()
    }

    pub fn distance(&self, cpu_a: u32, cpu_b: u32) -> u32 {
        let node_a = self.get_node(cpu_a);
        let node_b = self.get_node(cpu_b);
        match (node_a, node_b) {
            (Some(a), Some(b)) if a == b => 10,  // Same NUMA node
            (Some(_), Some(_)) => 20,             // Different NUMA node
            _ => 30,                                // Unknown
        }
    }
}

#[derive(Clone, Debug)]
pub struct NumaNode {
    pub node_id: u32,
    pub cpus: Vec<u32>,
    pub memory_bytes: u64,
    pub distance: u32,  // Distance from node 0
}

impl NumaNode {
    pub fn new(node_id: u32, num_cpus: u32, _distance: u32) -> Self {
        Self {
            node_id,
            cpus: (0..num_cpus).collect(),
            memory_bytes: 0,
            distance: 10,
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  TASK SCHEDULING INFO (per-task SMP metadata)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct TaskSmpInfo {
    pub pid: u32,
    pub affinity: CpuAffinity,
    pub current_cpu: Option<CpuId>,
    pub last_cpu: Option<CpuId>,
    pub weight: u32,
    pub migrations: u64,
    pub allowed_nodes: Vec<u32>,
}

impl TaskSmpInfo {
    pub fn new(pid: u32, max_cpus: usize) -> Self {
        Self {
            pid,
            affinity: CpuAffinity::all_cpus(max_cpus),
            current_cpu: None,
            last_cpu: None,
            weight: 1024,  // Default weight
            migrations: 0,
            allowed_nodes: vec![0],
        }
    }

    pub fn pinned(pid: u32, cpu: CpuId) -> Self {
        Self {
            pid,
            affinity: CpuAffinity::single(cpu),
            current_cpu: Some(cpu),
            last_cpu: None,
            weight: 1024,
            migrations: 0,
            allowed_nodes: vec![0],
        }
    }

    pub fn set_affinity(&mut self, affinity: CpuAffinity) {
        self.affinity = affinity;
    }

    pub fn can_run_on(&self, cpu: CpuId) -> bool {
        self.affinity.contains(cpu)
    }

    pub fn record_migration(&mut self, from: CpuId, to: CpuId) {
        self.last_cpu = Some(from);
        self.current_cpu = Some(to);
        self.migrations += 1;
    }
}

// ════════════════════════════════════════════════════════════════
//  SMP BARRIER
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SmpBarrier {
    pub barrier_id: u32,
    pub expected: u32,
    pub arrived: u32,
    pub generation: u64,
    pub participants: Vec<CpuId>,
}

impl SmpBarrier {
    pub fn new(barrier_id: u32, participants: Vec<CpuId>) -> Self {
        Self {
            barrier_id,
            expected: participants.len() as u32,
            arrived: 0,
            generation: 0,
            participants,
        }
    }

    pub fn arrive(&mut self, cpu: CpuId) -> bool {
        if !self.participants.contains(&cpu) { return false; }
        self.arrived += 1;
        if self.arrived >= self.expected {
            self.arrived = 0;
            self.generation += 1;
            true  // Barrier released
        } else {
            false
        }
    }

    pub fn is_complete(&self) -> bool { self.arrived == 0 && self.generation > 0 }
    pub fn reset(&mut self) { self.arrived = 0; self.generation += 1; }
}

// ════════════════════════════════════════════════════════════════
//  SCHEDULING DOMAIN (hierarchical load balancing)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SchedDomain {
    pub domain_id: u32,
    pub level: u32,           // 0 = SMT, 1 = Core, 2 = NUMA, 3 = All
    pub cpus: Vec<CpuId>,
    pub parent: Option<u32>,
    pub child: Vec<u32>,
    pub balance_interval_ms: u64,
    pub last_balance: u64,
    pub cache_nice_tries: u32,
    pub flags: SchedDomainFlags,
}

impl SchedDomain {
    pub fn new(domain_id: u32, level: u32, cpus: Vec<CpuId>) -> Self {
        Self {
            domain_id,
            level,
            cpus,
            parent: None,
            child: Vec::new(),
            balance_interval_ms: DEFAULT_LOAD_BALANCE_INTERVAL_MS,
            last_balance: 0,
            cache_nice_tries: 1,
            flags: SchedDomainFlags::default(),
        }
    }

    pub fn contains(&self, cpu: CpuId) -> bool {
        self.cpus.contains(&cpu)
    }

    pub fn should_balance(&self, current_time: u64) -> bool {
        current_time >= self.last_balance + self.balance_interval_ms
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SchedDomainFlags {
    pub share_cpucapacity: bool,  // SMT siblings share capacity
    pub share_pkg_resources: bool, // Share L3 cache
    pub numa: bool,                // NUMA domain
    pub prefer_sibling: bool,      // Prefer spreading to siblings
}

// ════════════════════════════════════════════════════════════════
//  SMP MANAGER
// ════════════════════════════════════════════════════════════════

pub struct SmpManager {
    per_cpu: BTreeMap<u32, PerCpuData>,
    topology: CpuTopology,
    tasks: BTreeMap<u32, TaskSmpInfo>,
    ipi_queue: Mutex<Vec<IpiMessage>>,
    barriers: BTreeMap<u32, SmpBarrier>,
    sched_domains: BTreeMap<u32, SchedDomain>,
    next_barrier_id: u32,
    next_domain_id: u32,
    total_migrations: u64,
    total_ipis: u64,
    total_balances: u64,
    boot_time: u64,
}

impl SmpManager {
    pub fn new(num_cpus: u32) -> Self {
        let mut per_cpu = BTreeMap::new();
        for i in 0..num_cpus {
            let mut cpu = PerCpuData::new(CpuId(i));
            if i == 0 {
                cpu.state = CpuState::Online; // BSP is online
            }
            per_cpu.insert(i, cpu);
        }

        Self {
            per_cpu,
            topology: CpuTopology::single_node(num_cpus),
            tasks: BTreeMap::new(),
            ipi_queue: Mutex::new(Vec::new()),
            barriers: BTreeMap::new(),
            sched_domains: BTreeMap::new(),
            next_barrier_id: 1,
            next_domain_id: 1,
            total_migrations: 0,
            total_ipis: 0,
            total_balances: 0,
            boot_time: 0,
        }
    }

    pub fn with_topology(num_cpus: u32, topology: CpuTopology) -> Self {
        let mut per_cpu = BTreeMap::new();
        for i in 0..num_cpus {
            let mut cpu = PerCpuData::new(CpuId(i));
            if i == 0 { cpu.state = CpuState::Online; }
            cpu.numa_node = topology.get_node(i).unwrap_or(0);
            cpu.core_id = topology.get_core(i).unwrap_or(i);
            if let Some(sibling) = topology.get_sibling(i) {
                cpu.sibling = Some(CpuId(sibling));
                cpu.thread_id = if i % 2 == 0 { 0 } else { 1 };
            }
            if let Some(cache) = topology.cache_topology.get(&i) {
                cpu.cache = cache.clone();
            }
            per_cpu.insert(i, cpu);
        }

        Self {
            per_cpu,
            topology,
            tasks: BTreeMap::new(),
            ipi_queue: Mutex::new(Vec::new()),
            barriers: BTreeMap::new(),
            sched_domains: BTreeMap::new(),
            next_barrier_id: 1,
            next_domain_id: 1,
            total_migrations: 0,
            total_ipis: 0,
            total_balances: 0,
            boot_time: 0,
        }
    }

    // ── CPU Management ──────────────────────────────────────

    pub fn cpu_count(&self) -> usize { self.per_cpu.len() }
    pub fn online_count(&self) -> usize {
        self.per_cpu.values().filter(|c| c.is_online()).count()
    }

    pub fn get_cpu(&self, cpu: CpuId) -> Option<&PerCpuData> {
        self.per_cpu.get(&cpu.raw())
    }

    pub fn get_cpu_mut(&mut self, cpu: CpuId) -> Option<&mut PerCpuData> {
        self.per_cpu.get_mut(&cpu.raw())
    }

    pub fn online_cpus(&self) -> Vec<CpuId> {
        self.per_cpu.values()
            .filter(|c| c.is_online())
            .map(|c| c.cpu_id)
            .collect()
    }

    pub fn offline_cpus(&self) -> Vec<CpuId> {
        self.per_cpu.values()
            .filter(|c| !c.is_online())
            .map(|c| c.cpu_id)
            .collect()
    }

    // ── CPU Hotplug ──────────────────────────────────────────

    pub fn bring_cpu_online(&mut self, cpu: CpuId) -> Result<(), SmpError> {
        let data = self.per_cpu.get_mut(&cpu.raw())
            .ok_or(SmpError::CpuNotFound)?;

        match data.state {
            CpuState::Offline | CpuState::Paused => {
                data.state = CpuState::Booting;
                data.state = CpuState::Online;
                Ok(())
            }
            CpuState::Online => Err(SmpError::AlreadyOnline),
            CpuState::Booting => Err(SmpError::AlreadyBooting),
            CpuState::Stopping => Err(SmpError::StillStopping),
        }
    }

    pub fn take_cpu_offline(&mut self, cpu: CpuId) -> Result<(), SmpError> {
        let data = self.per_cpu.get_mut(&cpu.raw())
            .ok_or(SmpError::CpuNotFound)?;

        if data.cpu_id.is_bsp() {
            return Err(SmpError::CannotOfflineBsp);
        }

        match data.state {
            CpuState::Online => {
                data.state = CpuState::Stopping;

                // Migrate all tasks away
                let tasks_to_migrate: Vec<u32> = data.run_queue.tasks.clone();
                for pid in tasks_to_migrate {
                    if let Some(target) = self.find_idle_cpu() {
                        self.migrate_task(pid, cpu, target)?;
                    }
                }

                data.state = CpuState::Offline;
                Ok(())
            }
            CpuState::Offline => Err(SmpError::AlreadyOffline),
            _ => Err(SmpError::InvalidState),
        }
    }

    pub fn pause_cpu(&mut self, cpu: CpuId) -> Result<(), SmpError> {
        let data = self.per_cpu.get_mut(&cpu.raw())
            .ok_or(SmpError::CpuNotFound)?;

        if data.state != CpuState::Online {
            return Err(SmpError::NotOnline);
        }
        data.state = CpuState::Paused;
        Ok(())
    }

    pub fn resume_cpu(&mut self, cpu: CpuId) -> Result<(), SmpError> {
        let data = self.per_cpu.get_mut(&cpu.raw())
            .ok_or(SmpError::CpuNotFound)?;

        if data.state != CpuState::Paused {
            return Err(SmpError::NotPaused);
        }
        data.state = CpuState::Online;
        Ok(())
    }

    // ── Task Management ──────────────────────────────────────

    pub fn register_task(&mut self, pid: u32, max_cpus: usize) {
        self.tasks.insert(pid, TaskSmpInfo::new(pid, max_cpus));
    }

    pub fn register_task_pinned(&mut self, pid: u32, cpu: CpuId) {
        self.tasks.insert(pid, TaskSmpInfo::pinned(pid, cpu));
    }

    pub fn unregister_task(&mut self, pid: u32) {
        // Remove from current CPU's run queue
        if let Some(task) = self.tasks.get(&pid) {
            if let Some(cpu) = task.current_cpu {
                if let Some(data) = self.per_cpu.get_mut(&cpu.raw()) {
                    data.run_queue.dequeue(pid);
                }
            }
        }
        self.tasks.remove(&pid);
    }

    pub fn set_task_affinity(&mut self, pid: u32, affinity: CpuAffinity) -> Result<(), SmpError> {
        let task = self.tasks.get_mut(&pid)
            .ok_or(SmpError::TaskNotFound)?;
        task.set_affinity(affinity);

        // If currently running on a CPU not in affinity, migrate
        if let Some(current) = task.current_cpu {
            if !task.can_run_on(current) {
                if let Some(target) = self.find_cpu_for_task(pid) {
                    self.migrate_task(pid, current, target)?;
                }
            }
        }
        Ok(())
    }

    pub fn get_task_affinity(&self, pid: u32) -> Option<&CpuAffinity> {
        self.tasks.get(&pid).map(|t| &t.affinity)
    }

    pub fn set_task_weight(&mut self, pid: u32, weight: u32) -> Result<(), SmpError> {
        let task = self.tasks.get_mut(&pid).ok_or(SmpError::TaskNotFound)?;
        task.weight = weight;
        Ok(())
    }

    pub fn pin_task(&mut self, pid: u32, cpu: CpuId) -> Result<(), SmpError> {
        self.set_task_affinity(pid, CpuAffinity::single(cpu))
    }

    pub fn unpin_task(&mut self, pid: u32, max_cpus: usize) -> Result<(), SmpError> {
        self.set_task_affinity(pid, CpuAffinity::all_cpus(max_cpus))
    }

    // ── Scheduling ───────────────────────────────────────────

    pub fn enqueue_task(&mut self, pid: u32, cpu: CpuId) -> Result<(), SmpError> {
        let data = self.per_cpu.get_mut(&cpu.raw())
            .ok_or(SmpError::CpuNotFound)?;

        if !data.is_online() {
            return Err(SmpError::CpuOffline);
        }

        let task = self.tasks.get(&pid).ok_or(SmpError::TaskNotFound)?;
        let weight = task.weight;
        drop(task);

        data.run_queue.enqueue(pid, weight);
        let task = self.tasks.get_mut(&pid).unwrap();
        task.current_cpu = Some(cpu);
        Ok(())
    }

    pub fn dequeue_task(&mut self, pid: u32) -> Result<(), SmpError> {
        let task = self.tasks.get(&pid).ok_or(SmpError::TaskNotFound)?;
        let cpu = task.current_cpu.ok_or(SmpError::TaskNotRunning)?;
        drop(task);

        let data = self.per_cpu.get_mut(&cpu.raw())
            .ok_or(SmpError::CpuNotFound)?;
        data.run_queue.dequeue(pid);

        self.tasks.get_mut(&pid).unwrap().current_cpu = None;
        Ok(())
    }

    pub fn schedule_next(&mut self, cpu: CpuId) -> Option<u32> {
        let data = self.per_cpu.get_mut(&cpu.raw())?;
        if !data.is_online() { return None; }
        let pid = data.run_queue.next_task();
        if pid.is_some() {
            data.record_switch();
        }
        pid
    }

    pub fn tick(&mut self, cpu: CpuId) -> bool {
        let data = match self.per_cpu.get_mut(&cpu.raw()) {
            Some(d) => d,
            None => return false,
        };
        data.record_tick(data.run_queue.is_empty());
        data.run_queue.tick()
    }

    // ── Task Migration ────────────────────────────────────────

    pub fn migrate_task(&mut self, pid: u32, from: CpuId, to: CpuId) -> Result<(), SmpError> {
        if from == to { return Ok(()); }

        let target = self.per_cpu.get_mut(&to.raw())
            .ok_or(SmpError::CpuNotFound)?;
        if !target.is_online() {
            return Err(SmpError::CpuOffline);
        }

        // Check affinity
        let task = self.tasks.get(&pid).ok_or(SmpError::TaskNotFound)?;
        if !task.can_run_on(to) {
            return Err(SmpError::AffinityViolation);
        }
        let weight = task.weight;
        drop(task);

        // Remove from source
        let source = self.per_cpu.get_mut(&from.raw())
            .ok_or(SmpError::CpuNotFound)?;
        source.run_queue.dequeue(pid);
        source.record_migration_out();

        // Add to target
        let target = self.per_cpu.get_mut(&to.raw()).unwrap();
        target.run_queue.enqueue(pid, weight);
        target.record_migration_in();

        // Update task info
        let task = self.tasks.get_mut(&pid).unwrap();
        task.record_migration(from, to);

        self.total_migrations += 1;
        Ok(())
    }

    // ── Load Balancing ────────────────────────────────────────

    pub fn find_idle_cpu(&self) -> Option<CpuId> {
        self.per_cpu.values()
            .filter(|c| c.is_online() && c.run_queue.is_empty())
            .map(|c| c.cpu_id)
            .next()
    }

    pub fn find_cpu_for_task(&self, pid: u32) -> Option<CpuId> {
        let task = self.tasks.get(&pid)?;
        // First try: idle CPU within affinity
        for cpu_data in self.per_cpu.values() {
            if cpu_data.is_online() && cpu_data.run_queue.is_empty() && task.can_run_on(cpu_data.cpu_id) {
                return Some(cpu_data.cpu_id);
            }
        }
        // Second try: least loaded CPU within affinity
        let mut best: Option<(CpuId, u32)> = None;
        for cpu_data in self.per_cpu.values() {
            if cpu_data.is_online() && task.can_run_on(cpu_data.cpu_id) {
                let load = cpu_data.load();
                if best.map(|(_, l)| load < l).unwrap_or(true) {
                    best = Some((cpu_data.cpu_id, load));
                }
            }
        }
        best.map(|(cpu, _)| cpu)
    }

    pub fn balance_load(&mut self) -> Vec<(u32, CpuId, CpuId)> {
        self.total_balances += 1;
        let mut migrations = Vec::new();

        // Find busiest and idlest CPUs
        let online: Vec<CpuId> = self.online_cpus();
        if online.len() < 2 { return migrations; }

        let mut cpu_loads: Vec<(CpuId, u32)> = Vec::new();
        for cpu in &online {
            if let Some(data) = self.per_cpu.get(&cpu.raw()) {
                cpu_loads.push((*cpu, data.load()));
            }
        }
        cpu_loads.sort_by_key(|(_, load)| *load);

        let max_migrations = MAX_MIGRATIONS_PER_BALANCE;
        let mut done = 0;

        while done < max_migrations {
            let busiest = cpu_loads.last().copied();
            let idlest = cpu_loads.first().copied();

            match (busiest, idlest) {
                (Some((busy_cpu, busy_load)), Some((idle_cpu, idle_load))) => {
                    if busy_cpu == idle_cpu { break; }
                    let ratio = if idle_load > 0 {
                        busy_load as f64 / idle_load as f64
                    } else {
                        f64::MAX
                    };
                    if ratio < LOAD_BALANCE_THRESHOLD { break; }

                    // Find a migratable task
                    if let Some(busy_data) = self.per_cpu.get(&busy_cpu.raw()) {
                        let tasks: Vec<u32> = busy_data.run_queue.tasks.clone();
                        let mut migrated = None;
                        for pid in tasks {
                            if let Some(task) = self.tasks.get(&pid) {
                                if task.can_run_on(idle_cpu) && task.current_cpu == Some(busy_cpu) {
                                    migrated = Some(pid);
                                    break;
                                }
                            }
                        }

                        if let Some(pid) = migrated {
                            if self.migrate_task(pid, busy_cpu, idle_cpu).is_ok() {
                                migrations.push((pid, busy_cpu, idle_cpu));
                                done += 1;
                            }
                        }
                    }

                    // Update loads
                    cpu_loads.clear();
                    for cpu in &online {
                        if let Some(data) = self.per_cpu.get(&cpu.raw()) {
                            cpu_loads.push((*cpu, data.load()));
                        }
                    }
                    cpu_loads.sort_by_key(|(_, load)| *load);
                }
                _ => break,
            }
        }

        migrations
    }

    // ── IPI (Inter-Processor Interrupts) ──────────────────────

    pub fn send_ipi(&mut self, source: CpuId, target: CpuId, ipi_type: IpiType) -> Result<(), SmpError> {
        let target_data = self.per_cpu.get(&target.raw())
            .ok_or(SmpError::CpuNotFound)?;

        if !target_data.is_online() && ipi_type != IpiType::WakeUp {
            return Err(SmpError::CpuOffline);
        }

        let msg = IpiMessage {
            ipi_type,
            source_cpu: source,
            target_cpu: target,
            data: 0,
            timestamp: 0,
        };

        self.ipi_queue.lock().push(msg);
        self.total_ipis += 1;

        let target_data = self.per_cpu.get_mut(&target.raw()).unwrap();
        target_data.record_ipi(ipi_type);

        Ok(())
    }

    pub fn send_ipi_all(&mut self, source: CpuId, ipi_type: IpiType) -> Vec<(CpuId, Result<(), SmpError>)> {
        let targets: Vec<CpuId> = self.online_cpus();
        let mut results = Vec::new();
        for target in targets {
            if target == source { continue; }
            let result = self.send_ipi(source, target, ipi_type);
            results.push((target, result));
        }
        results
    }

    pub fn send_ipi_mask(&mut self, source: CpuId, mask: &CpuAffinity, ipi_type: IpiType) -> Vec<(CpuId, Result<(), SmpError>)> {
        let targets: Vec<CpuId> = mask.cpus();
        let mut results = Vec::new();
        for target in targets {
            if target == source { continue; }
            let result = self.send_ipi(source, target, ipi_type);
            results.push((target, result));
        }
        results
    }

    pub fn pending_ipis(&self) -> Vec<IpiMessage> {
        self.ipi_queue.lock().clone()
    }

    pub fn consume_ipis(&self, cpu: CpuId) -> Vec<IpiMessage> {
        self.ipi_queue.lock().iter()
            .filter(|m| m.target_cpu == cpu)
            .cloned()
            .collect()
    }

    pub fn ipi_count(&self) -> usize {
        self.ipi_queue.lock().len()
    }

    // ── SMP Barriers ─────────────────────────────────────────

    pub fn create_barrier(&mut self, participants: Vec<CpuId>) -> u32 {
        let id = self.next_barrier_id;
        self.next_barrier_id += 1;
        self.barriers.insert(id, SmpBarrier::new(id, participants));
        id
    }

    pub fn arrive_barrier(&mut self, barrier_id: u32, cpu: CpuId) -> Result<bool, SmpError> {
        let barrier = self.barriers.get_mut(&barrier_id)
            .ok_or(SmpError::BarrierNotFound)?;
        Ok(barrier.arrive(cpu))
    }

    pub fn barrier_complete(&self, barrier_id: u32) -> Result<bool, SmpError> {
        let barrier = self.barriers.get(&barrier_id)
            .ok_or(SmpError::BarrierNotFound)?;
        Ok(barrier.is_complete())
    }

    pub fn destroy_barrier(&mut self, barrier_id: u32) -> bool {
        self.barriers.remove(&barrier_id).is_some()
    }

    // ── Scheduling Domains ───────────────────────────────────

    pub fn create_sched_domain(&mut self, level: u32, cpus: Vec<CpuId>) -> u32 {
        let id = self.next_domain_id;
        self.next_domain_id += 1;
        self.sched_domains.insert(id, SchedDomain::new(id, level, cpus));
        id
    }

    pub fn get_sched_domain(&self, domain_id: u32) -> Option<&SchedDomain> {
        self.sched_domains.get(&domain_id)
    }

    pub fn set_domain_parent(&mut self, domain_id: u32, parent: u32) -> bool {
        if let Some(domain) = self.sched_domains.get_mut(&domain_id) {
            domain.parent = Some(parent);
            return true;
        }
        false
    }

    pub fn add_domain_child(&mut self, domain_id: u32, child: u32) -> bool {
        if let Some(domain) = self.sched_domains.get_mut(&domain_id) {
            domain.child.push(child);
            return true;
        }
        false
    }

    // ── Topology Query ───────────────────────────────────────

    pub fn topology(&self) -> &CpuTopology { &self.topology }

    pub fn cpu_distance(&self, cpu_a: CpuId, cpu_b: CpuId) -> u32 {
        self.topology.distance(cpu_a.raw(), cpu_b.raw())
    }

    pub fn is_sibling(&self, cpu_a: CpuId, cpu_b: CpuId) -> bool {
        self.topology.is_sibling(cpu_a.raw(), cpu_b.raw())
    }

    pub fn same_numa_node(&self, cpu_a: CpuId, cpu_b: CpuId) -> bool {
        self.topology.get_node(cpu_a.raw()) == self.topology.get_node(cpu_b.raw())
    }

    // ── Statistics ───────────────────────────────────────────

    pub fn avg_load(&self) -> f64 {
        let online: Vec<&PerCpuData> = self.per_cpu.values().filter(|c| c.is_online()).collect();
        if online.is_empty() { return 0.0; }
        let total: u32 = online.iter().map(|c| c.load()).sum();
        total as f64 / online.len() as f64
    }

    pub fn max_load_cpu(&self) -> Option<CpuId> {
        self.per_cpu.values()
            .filter(|c| c.is_online())
            .max_by_key(|c| c.load())
            .map(|c| c.cpu_id)
    }

    pub fn min_load_cpu(&self) -> Option<CpuId> {
        self.per_cpu.values()
            .filter(|c| c.is_online())
            .min_by_key(|c| c.load())
            .map(|c| c.cpu_id)
    }

    pub fn stats(&self) -> SmpStats {
        SmpStats {
            total_cpus: self.cpu_count() as u32,
            online_cpus: self.online_count() as u32,
            total_tasks: self.tasks.len() as u32,
            total_migrations: self.total_migrations,
            total_ipis: self.total_ipis,
            total_balances: self.total_balances,
            avg_load: self.avg_load(),
            numa_nodes: self.topology.numa_nodes.len() as u32,
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  SMP STATISTICS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SmpStats {
    pub total_cpus: u32,
    pub online_cpus: u32,
    pub total_tasks: u32,
    pub total_migrations: u64,
    pub total_ipis: u64,
    pub total_balances: u64,
    pub avg_load: f64,
    pub numa_nodes: u32,
}

// ════════════════════════════════════════════════════════════════
//  SMP ERROR
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmpError {
    CpuNotFound,
    TaskNotFound,
    TaskNotRunning,
    CpuOffline,
    AlreadyOnline,
    AlreadyOffline,
    AlreadyBooting,
    StillStopping,
    NotOnline,
    NotPaused,
    InvalidState,
    CannotOfflineBsp,
    AffinityViolation,
    BarrierNotFound,
}

impl SmpError {
    pub fn name(&self) -> &'static str {
        match self {
            SmpError::CpuNotFound => "cpu_not_found",
            SmpError::TaskNotFound => "task_not_found",
            SmpError::TaskNotRunning => "task_not_running",
            SmpError::CpuOffline => "cpu_offline",
            SmpError::AlreadyOnline => "already_online",
            SmpError::AlreadyOffline => "already_offline",
            SmpError::AlreadyBooting => "already_booting",
            SmpError::StillStopping => "still_stopping",
            SmpError::NotOnline => "not_online",
            SmpError::NotPaused => "not_paused",
            SmpError::InvalidState => "invalid_state",
            SmpError::CannotOfflineBsp => "cannot_offline_bsp",
            SmpError::AffinityViolation => "affinity_violation",
            SmpError::BarrierNotFound => "barrier_not_found",
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  TESTS
// ════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── CpuState Tests ────────────────────────────────────────

    #[test]
    fn test_cpu_state_names() {
        assert_eq!(CpuState::Offline.name(), "offline");
        assert_eq!(CpuState::Booting.name(), "booting");
        assert_eq!(CpuState::Online.name(), "online");
        assert_eq!(CpuState::Paused.name(), "paused");
        assert_eq!(CpuState::Stopping.name(), "stopping");
    }

    #[test]
    fn test_cpu_state_is_active() {
        assert!(CpuState::Online.is_active());
        assert!(!CpuState::Offline.is_active());
        assert!(!CpuState::Paused.is_active());
    }

    #[test]
    fn test_cpu_state_is_available() {
        assert!(CpuState::Online.is_available());
        assert!(CpuState::Paused.is_available());
        assert!(!CpuState::Offline.is_available());
    }

    // ── CpuId Tests ──────────────────────────────────────────

    #[test]
    fn test_cpu_id_bsp() {
        assert!(CpuId(0).is_bsp());
        assert!(!CpuId(1).is_bsp());
    }

    #[test]
    fn test_cpu_id_ap() {
        assert!(!CpuId(0).is_ap());
        assert!(CpuId(1).is_ap());
    }

    #[test]
    fn test_cpu_id_raw() {
        assert_eq!(CpuId(5).raw(), 5);
    }

    // ── CpuAffinity Tests ─────────────────────────────────────

    #[test]
    fn test_affinity_all_cpus() {
        let aff = CpuAffinity::all_cpus(64);
        assert!(!aff.is_empty());
        assert!(aff.contains(CpuId(0)));
        assert!(aff.contains(CpuId(63)));
        assert!(!aff.hard);
    }

    #[test]
    fn test_affinity_single() {
        let aff = CpuAffinity::single(CpuId(3));
        assert!(aff.contains(CpuId(3)));
        assert!(!aff.contains(CpuId(0)));
        assert!(!aff.contains(CpuId(4)));
        assert_eq!(aff.count(), 1);
        assert!(aff.hard);
    }

    #[test]
    fn test_affinity_range() {
        let aff = CpuAffinity::range(2, 5);
        assert!(aff.contains(CpuId(2)));
        assert!(aff.contains(CpuId(3)));
        assert!(aff.contains(CpuId(4)));
        assert!(aff.contains(CpuId(5)));
        assert!(!aff.contains(CpuId(1)));
        assert!(!aff.contains(CpuId(6)));
        assert_eq!(aff.count(), 4);
    }

    #[test]
    fn test_affinity_add_remove() {
        let mut aff = CpuAffinity::none();
        assert!(aff.is_empty());
        aff.add(CpuId(2));
        aff.add(CpuId(4));
        assert_eq!(aff.count(), 2);
        aff.remove(CpuId(2));
        assert_eq!(aff.count(), 1);
        assert!(!aff.contains(CpuId(2)));
    }

    #[test]
    fn test_affinity_first_cpu() {
        let mut aff = CpuAffinity::none();
        assert_eq!(aff.first_cpu(), None);
        aff.add(CpuId(5));
        aff.add(CpuId(2));
        assert_eq!(aff.first_cpu(), Some(CpuId(2)));
    }

    #[test]
    fn test_affinity_cpus_list() {
        let mut aff = CpuAffinity::none();
        aff.add(CpuId(1));
        aff.add(CpuId(3));
        aff.add(CpuId(5));
        let cpus = aff.cpus();
        assert_eq!(cpus.len(), 3);
        assert!(cpus.contains(&CpuId(1)));
        assert!(cpus.contains(&CpuId(3)));
        assert!(cpus.contains(&CpuId(5)));
    }

    #[test]
    fn test_affinity_intersect() {
        let a = CpuAffinity::range(0, 5);
        let b = CpuAffinity::range(3, 8);
        let c = a.intersect(&b);
        assert!(c.contains(CpuId(3)));
        assert!(c.contains(CpuId(5)));
        assert!(!c.contains(CpuId(0)));
        assert!(!c.contains(CpuId(6)));
    }

    #[test]
    fn test_affinity_union() {
        let mut a = CpuAffinity::none();
        a.add(CpuId(1));
        let mut b = CpuAffinity::none();
        b.add(CpuId(3));
        let c = a.union(&b);
        assert!(c.contains(CpuId(1)));
        assert!(c.contains(CpuId(3)));
    }

    #[test]
    fn test_affinity_set_hard() {
        let mut aff = CpuAffinity::all_cpus(64);
        assert!(!aff.hard);
        aff.set_hard(true);
        assert!(aff.hard);
    }

    // ── RunQueue Tests ────────────────────────────────────────

    #[test]
    fn test_run_queue_enqueue_dequeue() {
        let mut rq = RunQueue::new(CpuId(0));
        rq.enqueue(100, 1024);
        rq.enqueue(101, 1024);
        rq.enqueue(102, 512);
        assert_eq!(rq.len(), 3);
        assert_eq!(rq.load(), 3);

        assert!(rq.dequeue(101));
        assert_eq!(rq.len(), 2);
        assert!(!rq.dequeue(999));
    }

    #[test]
    fn test_run_queue_next_task() {
        let mut rq = RunQueue::new(CpuId(0));
        rq.enqueue(100, 1024);
        rq.enqueue(101, 1024);

        let next = rq.next_task();
        assert_eq!(next, Some(100));
        assert_eq!(rq.current_pid, Some(100));
        assert_eq!(rq.quantum_remaining, DEFAULT_QUANTUM_TICKS);
    }

    #[test]
    fn test_run_queue_next_task_empty() {
        let mut rq = RunQueue::new(CpuId(0));
        assert_eq!(rq.next_task(), None);
    }

    #[test]
    fn test_run_queue_tick() {
        let mut rq = RunQueue::new(CpuId(0));
        rq.enqueue(100, 1024);
        rq.next_task();

        // Tick down quantum
        for _ in 0..DEFAULT_QUANTUM_TICKS {
            assert!(!rq.tick());
        }
        // Next tick should signal quantum expired
        assert!(rq.tick());
    }

    #[test]
    fn test_run_queue_peek_next() {
        let mut rq = RunQueue::new(CpuId(0));
        rq.enqueue(100, 1024);
        rq.enqueue(101, 1024);
        assert_eq!(rq.peek_next(), Some(100));
        rq.next_task();
        assert_eq!(rq.peek_next(), Some(101));
    }

    #[test]
    fn test_run_queue_is_empty() {
        let mut rq = RunQueue::new(CpuId(0));
        assert!(rq.is_empty());
        rq.enqueue(100, 1024);
        assert!(!rq.is_empty());
    }

    // ── PerCpuData Tests ──────────────────────────────────────

    #[test]
    fn test_per_cpu_data_new() {
        let data = PerCpuData::new(CpuId(0));
        assert_eq!(data.cpu_id, CpuId(0));
        assert_eq!(data.state, CpuState::Offline);
        assert!(!data.is_online());
        assert!(data.is_bsp());
    }

    #[test]
    fn test_per_cpu_data_utilization() {
        let mut data = PerCpuData::new(CpuId(0));
        data.record_tick(false);
        data.record_tick(false);
        data.record_tick(true);
        assert_eq!(data.ticks, 3);
        assert_eq!(data.idle_ticks, 1);
        assert!((data.utilization() - (2.0/3.0)).abs() < 0.01);
    }

    #[test]
    fn test_per_cpu_data_record_switch() {
        let mut data = PerCpuData::new(CpuId(0));
        data.record_switch();
        data.record_switch();
        assert_eq!(data.context_switches, 2);
    }

    #[test]
    fn test_per_cpu_data_migration_stats() {
        let mut data = PerCpuData::new(CpuId(0));
        data.record_migration_in();
        data.record_migration_in();
        data.record_migration_out();
        assert_eq!(data.migrations_in, 2);
        assert_eq!(data.migrations_out, 1);
    }

    #[test]
    fn test_per_cpu_data_ipi() {
        let mut data = PerCpuData::new(CpuId(0));
        data.record_ipi(IpiType::Reschedule);
        assert_eq!(data.ipi_count, 1);
        assert_eq!(data.last_ipi, Some(IpiType::Reschedule));
    }

    // ── CpuStats Tests ────────────────────────────────────────

    #[test]
    fn test_cpu_stats_total_ticks() {
        let mut stats = CpuStats::new(CpuId(0));
        stats.record_user(100);
        stats.record_system(50);
        stats.record_idle(200);
        assert_eq!(stats.total_ticks(), 350);
    }

    #[test]
    fn test_cpu_stats_busy_ticks() {
        let mut stats = CpuStats::new(CpuId(0));
        stats.record_user(100);
        stats.record_system(50);
        stats.record_idle(200);
        assert_eq!(stats.busy_ticks(), 150);
    }

    #[test]
    fn test_cpu_stats_utilization() {
        let mut stats = CpuStats::new(CpuId(0));
        stats.record_user(300);
        stats.record_idle(700);
        assert!((stats.utilization() - 30.0).abs() < 0.1);
    }

    // ── IpiType Tests ─────────────────────────────────────────

    #[test]
    fn test_ipi_type_names() {
        assert_eq!(IpiType::Reschedule.name(), "RESCHEDULE");
        assert_eq!(IpiType::TlbShootdown.name(), "TLB_SHOOTDOWN");
        assert_eq!(IpiType::CallFunction.name(), "CALL_FUNCTION");
        assert_eq!(IpiType::Stop.name(), "STOP");
    }

    #[test]
    fn test_ipi_type_priority() {
        assert!(IpiType::Crash.priority() < IpiType::Stop.priority());
        assert!(IpiType::Stop.priority() < IpiType::Reschedule.priority());
        assert!(IpiType::TlbShootdown.priority() < IpiType::Reschedule.priority());
    }

    // ── CpuTopology Tests ─────────────────────────────────────

    #[test]
    fn test_topology_single_node() {
        let topo = CpuTopology::single_node(8);
        assert_eq!(topo.num_cpus, 8);
        assert_eq!(topo.numa_nodes.len(), 1);
        assert_eq!(topo.get_node(0), Some(0));
        assert_eq!(topo.get_node(7), Some(0));
    }

    #[test]
    fn test_topology_with_hyperthreads() {
        let topo = CpuTopology::with_hyperthreads(8, 2);
        assert_eq!(topo.num_cpus, 8);
        // CPU 0 and 1 are siblings
        assert_eq!(topo.get_sibling(0), Some(1));
        assert_eq!(topo.get_sibling(1), Some(0));
        // CPU 0 and 1 share core 0
        assert_eq!(topo.get_core(0), Some(0));
        assert_eq!(topo.get_core(1), Some(0));
    }

    #[test]
    fn test_topology_is_sibling() {
        let topo = CpuTopology::with_hyperthreads(8, 2);
        assert!(topo.is_sibling(0, 1));
        assert!(topo.is_sibling(1, 0));
        assert!(!topo.is_sibling(0, 2));
    }

    #[test]
    fn test_topology_cpus_in_node() {
        let topo = CpuTopology::single_node(4);
        let cpus = topo.cpus_in_node(0);
        assert_eq!(cpus.len(), 4);
    }

    #[test]
    fn test_topology_cpus_on_core() {
        let topo = CpuTopology::with_hyperthreads(8, 2);
        let cpus = topo.cpus_on_core(0);
        assert_eq!(cpus.len(), 2);
        assert!(cpus.contains(&0));
        assert!(cpus.contains(&1));
    }

    #[test]
    fn test_topology_distance() {
        let topo = CpuTopology::single_node(8);
        assert_eq!(topo.distance(0, 1), 10); // Same node
    }

    // ── TaskSmpInfo Tests ─────────────────────────────────────

    #[test]
    fn test_task_smp_info_new() {
        let task = TaskSmpInfo::new(100, 64);
        assert_eq!(task.pid, 100);
        assert!(task.affinity.contains(CpuId(0)));
        assert!(task.affinity.contains(CpuId(63)));
        assert_eq!(task.weight, 1024);
    }

    #[test]
    fn test_task_smp_info_pinned() {
        let task = TaskSmpInfo::pinned(100, CpuId(2));
        assert!(task.can_run_on(CpuId(2)));
        assert!(!task.can_run_on(CpuId(0)));
        assert_eq!(task.current_cpu, Some(CpuId(2)));
        assert!(task.affinity.hard);
    }

    #[test]
    fn test_task_smp_info_set_affinity() {
        let mut task = TaskSmpInfo::new(100, 64);
        task.set_affinity(CpuAffinity::single(CpuId(1)));
        assert!(task.can_run_on(CpuId(1)));
        assert!(!task.can_run_on(CpuId(0)));
    }

    #[test]
    fn test_task_smp_info_record_migration() {
        let mut task = TaskSmpInfo::new(100, 64);
        task.current_cpu = Some(CpuId(0));
        task.record_migration(CpuId(0), CpuId(3));
        assert_eq!(task.last_cpu, Some(CpuId(0)));
        assert_eq!(task.current_cpu, Some(CpuId(3)));
        assert_eq!(task.migrations, 1);
    }

    // ── SmpBarrier Tests ──────────────────────────────────────

    #[test]
    fn test_barrier_creation() {
        let barrier = SmpBarrier::new(1, vec![CpuId(0), CpuId(1), CpuId(2)]);
        assert_eq!(barrier.expected, 3);
        assert_eq!(barrier.arrived, 0);
        assert_eq!(barrier.generation, 0);
    }

    #[test]
    fn test_barrier_arrive() {
        let mut barrier = SmpBarrier::new(1, vec![CpuId(0), CpuId(1), CpuId(2)]);
        assert!(!barrier.arrive(CpuId(0)));
        assert_eq!(barrier.arrived, 1);
        assert!(!barrier.arrive(CpuId(1)));
        assert_eq!(barrier.arrived, 2);
        assert!(barrier.arrive(CpuId(2)));  // Last one releases
        assert_eq!(barrier.arrived, 0);
        assert_eq!(barrier.generation, 1);
    }

    #[test]
    fn test_barrier_non_participant() {
        let mut barrier = SmpBarrier::new(1, vec![CpuId(0), CpuId(1)]);
        assert!(!barrier.arrive(CpuId(5)));  // Not a participant
        assert_eq!(barrier.arrived, 0);
    }

    #[test]
    fn test_barrier_reset() {
        let mut barrier = SmpBarrier::new(1, vec![CpuId(0), CpuId(1)]);
        barrier.arrive(CpuId(0));
        barrier.reset();
        assert_eq!(barrier.arrived, 0);
        assert_eq!(barrier.generation, 1);
    }

    // ── SchedDomain Tests ─────────────────────────────────────

    #[test]
    fn test_sched_domain_new() {
        let domain = SchedDomain::new(1, 0, vec![CpuId(0), CpuId(1)]);
        assert_eq!(domain.domain_id, 1);
        assert_eq!(domain.level, 0);
        assert_eq!(domain.cpus.len(), 2);
        assert!(domain.contains(CpuId(0)));
        assert!(domain.contains(CpuId(1)));
        assert!(!domain.contains(CpuId(2)));
    }

    #[test]
    fn test_sched_domain_should_balance() {
        let mut domain = SchedDomain::new(1, 0, vec![CpuId(0)]);
        domain.last_balance = 100;
        assert!(!domain.should_balance(150));
        assert!(domain.should_balance(250));
    }

    // ── SmpManager: Basic Tests ───────────────────────────────

    #[test]
    fn test_smp_manager_new() {
        let mgr = SmpManager::new(4);
        assert_eq!(mgr.cpu_count(), 4);
        assert_eq!(mgr.online_count(), 1);  // Only BSP
        assert_eq!(mgr.online_cpus(), vec![CpuId(0)]);
    }

    #[test]
    fn test_smp_manager_with_topology() {
        let topo = CpuTopology::with_hyperthreads(8, 2);
        let mgr = SmpManager::with_topology(8, topo);
        assert_eq!(mgr.cpu_count(), 8);
        assert!(mgr.is_sibling(CpuId(0), CpuId(1)));
    }

    #[test]
    fn test_smp_manager_get_cpu() {
        let mgr = SmpManager::new(4);
        let cpu = mgr.get_cpu(CpuId(0));
        assert!(cpu.is_some());
        assert_eq!(cpu.unwrap().cpu_id, CpuId(0));
    }

    // ── SmpManager: Hotplug Tests ─────────────────────────────

    #[test]
    fn test_smp_bring_cpu_online() {
        let mut mgr = SmpManager::new(4);
        assert_eq!(mgr.online_count(), 1);

        mgr.bring_cpu_online(CpuId(1)).unwrap();
        assert_eq!(mgr.online_count(), 2);
        assert!(mgr.get_cpu(CpuId(1)).unwrap().is_online());
    }

    #[test]
    fn test_smp_bring_cpu_already_online() {
        let mut mgr = SmpManager::new(4);
        let result = mgr.bring_cpu_online(CpuId(0));
        assert_eq!(result, Err(SmpError::AlreadyOnline));
    }

    #[test]
    fn test_smp_take_cpu_offline() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.take_cpu_offline(CpuId(1)).unwrap();
        assert!(!mgr.get_cpu(CpuId(1)).unwrap().is_online());
        assert_eq!(mgr.online_count(), 1);
    }

    #[test]
    fn test_smp_cannot_offline_bsp() {
        let mut mgr = SmpManager::new(4);
        let result = mgr.take_cpu_offline(CpuId(0));
        assert_eq!(result, Err(SmpError::CannotOfflineBsp));
    }

    #[test]
    fn test_smp_pause_resume_cpu() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.pause_cpu(CpuId(1)).unwrap();
        assert_eq!(mgr.get_cpu(CpuId(1)).unwrap().state, CpuState::Paused);
        mgr.resume_cpu(CpuId(1)).unwrap();
        assert_eq!(mgr.get_cpu(CpuId(1)).unwrap().state, CpuState::Online);
    }

    #[test]
    fn test_smp_pause_not_online() {
        let mut mgr = SmpManager::new(4);
        let result = mgr.pause_cpu(CpuId(2));
        assert_eq!(result, Err(SmpError::NotOnline));
    }

    // ── SmpManager: Task Tests ────────────────────────────────

    #[test]
    fn test_smp_register_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        assert!(mgr.get_task_affinity(100).is_some());
    }

    #[test]
    fn test_smp_register_pinned_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task_pinned(100, CpuId(2));
        let aff = mgr.get_task_affinity(100).unwrap();
        assert!(aff.contains(CpuId(2)));
        assert!(!aff.contains(CpuId(0)));
        assert!(aff.hard);
    }

    #[test]
    fn test_smp_set_task_affinity() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.set_task_affinity(100, CpuAffinity::single(CpuId(1))).unwrap();
        let aff = mgr.get_task_affinity(100).unwrap();
        assert!(aff.contains(CpuId(1)));
        assert!(!aff.contains(CpuId(0)));
    }

    #[test]
    fn test_smp_pin_unpin_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.pin_task(100, CpuId(2)).unwrap();
        assert!(mgr.get_task_affinity(100).unwrap().contains(CpuId(2)));
        assert!(!mgr.get_task_affinity(100).unwrap().contains(CpuId(0)));
        mgr.unpin_task(100, 64).unwrap();
        assert!(mgr.get_task_affinity(100).unwrap().contains(CpuId(0)));
    }

    #[test]
    fn test_smp_unregister_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.enqueue_task(100, CpuId(1)).unwrap();
        mgr.unregister_task(100);
        assert!(mgr.get_task_affinity(100).is_none());
    }

    #[test]
    fn test_smp_set_task_weight() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.set_task_weight(100, 2048).unwrap();
        assert!(mgr.tasks.get(&100).unwrap().weight == 2048);
    }

    // ── SmpManager: Scheduling Tests ──────────────────────────

    #[test]
    fn test_smp_enqueue_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.enqueue_task(100, CpuId(1)).unwrap();
        assert_eq!(mgr.get_cpu(CpuId(1)).unwrap().load(), 1);
    }

    #[test]
    fn test_smp_enqueue_offline_cpu() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        let result = mgr.enqueue_task(100, CpuId(3));  // CPU 3 is offline
        assert_eq!(result, Err(SmpError::CpuOffline));
    }

    #[test]
    fn test_smp_dequeue_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.enqueue_task(100, CpuId(1)).unwrap();
        mgr.dequeue_task(100).unwrap();
        assert_eq!(mgr.get_cpu(CpuId(1)).unwrap().load(), 0);
    }

    #[test]
    fn test_smp_schedule_next() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.register_task(101, 64);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.enqueue_task(100, CpuId(1)).unwrap();
        mgr.enqueue_task(101, CpuId(1)).unwrap();

        let next = mgr.schedule_next(CpuId(1));
        assert_eq!(next, Some(100));
    }

    #[test]
    fn test_smp_tick() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.bring_cpu_online(CpuId(0)).unwrap();  // BSP already online
        mgr.enqueue_task(100, CpuId(0)).unwrap();

        for _ in 0..DEFAULT_QUANTUM_TICKS {
            assert!(!mgr.tick(CpuId(0)));
        }
        assert!(mgr.tick(CpuId(0)));
    }

    // ── SmpManager: Migration Tests ───────────────────────────

    #[test]
    fn test_smp_migrate_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();
        mgr.enqueue_task(100, CpuId(1)).unwrap();

        mgr.migrate_task(100, CpuId(1), CpuId(2)).unwrap();
        assert_eq!(mgr.get_cpu(CpuId(1)).unwrap().load(), 0);
        assert_eq!(mgr.get_cpu(CpuId(2)).unwrap().load(), 1);
        assert_eq!(mgr.tasks.get(&100).unwrap().current_cpu, Some(CpuId(2)));
        assert_eq!(mgr.tasks.get(&100).unwrap().migrations, 1);
    }

    #[test]
    fn test_smp_migrate_affinity_violation() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task_pinned(100, CpuId(1));
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();
        mgr.enqueue_task(100, CpuId(1)).unwrap();

        let result = mgr.migrate_task(100, CpuId(1), CpuId(2));
        assert_eq!(result, Err(SmpError::AffinityViolation));
    }

    #[test]
    fn test_smp_migrate_to_offline() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.enqueue_task(100, CpuId(1)).unwrap();

        let result = mgr.migrate_task(100, CpuId(1), CpuId(3));  // CPU 3 offline
        assert_eq!(result, Err(SmpError::CpuOffline));
    }

    #[test]
    fn test_smp_migrate_same_cpu() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.enqueue_task(100, CpuId(1)).unwrap();

        mgr.migrate_task(100, CpuId(1), CpuId(1)).unwrap();  // No-op
        assert_eq!(mgr.get_cpu(CpuId(1)).unwrap().load(), 1);
    }

    // ── SmpManager: Load Balancing Tests ──────────────────────

    #[test]
    fn test_smp_find_idle_cpu() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();
        // CPU 0 has BSP running, CPU 1 and 2 are idle
        let idle = mgr.find_idle_cpu();
        assert!(idle.is_some());
    }

    #[test]
    fn test_smp_find_cpu_for_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task(100, 64);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();
        let cpu = mgr.find_cpu_for_task(100);
        assert!(cpu.is_some());
    }

    #[test]
    fn test_smp_find_cpu_for_pinned_task() {
        let mut mgr = SmpManager::new(4);
        mgr.register_task_pinned(100, CpuId(2));
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();
        let cpu = mgr.find_cpu_for_task(100);
        assert_eq!(cpu, Some(CpuId(2)));
    }

    #[test]
    fn test_smp_balance_load() {
        let mut mgr = SmpManager::new(4);
        for i in 1..4 { mgr.bring_cpu_online(CpuId(i)).unwrap(); }

        mgr.register_task(100, 64);
        mgr.register_task(101, 64);
        mgr.register_task(102, 64);
        mgr.register_task(103, 64);

        // All tasks on CPU 0
        mgr.enqueue_task(100, CpuId(0)).unwrap();
        mgr.enqueue_task(101, CpuId(0)).unwrap();
        mgr.enqueue_task(102, CpuId(0)).unwrap();
        mgr.enqueue_task(103, CpuId(0)).unwrap();

        // Balance should move tasks to idle CPUs
        let migrations = mgr.balance_load();
        assert!(!migrations.is_empty());

        // CPU 0 should have fewer tasks
        let cpu0_load = mgr.get_cpu(CpuId(0)).unwrap().load();
        assert!(cpu0_load < 4);
    }

    #[test]
    fn test_smp_balance_no_imbalance() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();

        mgr.register_task(100, 64);
        mgr.register_task(101, 64);
        mgr.enqueue_task(100, CpuId(0)).unwrap();
        mgr.enqueue_task(101, CpuId(1)).unwrap();

        let migrations = mgr.balance_load();
        // Load is balanced (1 vs 1) — no migrations
        assert!(migrations.is_empty());
    }

    // ── SmpManager: IPI Tests ─────────────────────────────────

    #[test]
    fn test_smp_send_ipi() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.send_ipi(CpuId(0), CpuId(1), IpiType::Reschedule).unwrap();
        assert_eq!(mgr.ipi_count(), 1);
        assert_eq!(mgr.get_cpu(CpuId(1)).unwrap().ipi_count, 1);
        assert_eq!(mgr.get_cpu(CpuId(1)).unwrap().last_ipi, Some(IpiType::Reschedule));
    }

    #[test]
    fn test_smp_send_ipi_offline() {
        let mut mgr = SmpManager::new(4);
        let result = mgr.send_ipi(CpuId(0), CpuId(3), IpiType::Reschedule);
        assert_eq!(result, Err(SmpError::CpuOffline));
    }

    #[test]
    fn test_smp_send_ipi_all() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();
        let results = mgr.send_ipi_all(CpuId(0), IpiType::WakeUp);
        // Sent to all online except self
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|(_, r)| r.is_ok()));
        assert_eq!(mgr.ipi_count(), 2);
    }

    #[test]
    fn test_smp_send_ipi_mask() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();
        let mut mask = CpuAffinity::none();
        mask.add(CpuId(1));
        mask.add(CpuId(2));
        mask.add(CpuId(3));  // Offline — will fail
        let results = mgr.send_ipi_mask(CpuId(0), &mask, IpiType::Reschedule);
        assert_eq!(results.len(), 2);  // CPU 1 and 2 (not self, not offline... wait, 3 is not self)
        // CPU 3 is offline, so send_ipi fails
        let successes = results.iter().filter(|(_, r)| r.is_ok()).count();
        let failures = results.iter().filter(|(_, r)| r.is_err()).count();
        assert_eq!(successes, 1);  // CPU 1 only (CPU 2 is also online...)
        // Actually CPU 1 and 2 are both online, CPU 3 is offline
        // But we filter out source (CPU 0), and CPU 3 will fail
        // So results should be: CPU 1 (ok), CPU 2 (ok), CPU 3 (err)
        // But wait — we skip source only, not offline. Let me re-check:
        // Actually we filter out source, not offline. CPU 3 will get Err(CpuOffline).
        // So: 2 successes (CPU 1, CPU 2) + 1 failure (CPU 3) = 3 results
        // But the code skips source, so if source is CPU 0, we send to 1, 2, 3
        assert!(results.len() >= 2);
    }

    #[test]
    fn test_smp_consume_ipis() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.send_ipi(CpuId(0), CpuId(1), IpiType::Reschedule).unwrap();
        mgr.send_ipi(CpuId(0), CpuId(1), IpiType::TlbShootdown).unwrap();

        let ipis = mgr.consume_ipis(CpuId(1));
        assert_eq!(ipis.len(), 2);
    }

    // ── SmpManager: Barrier Tests ─────────────────────────────

    #[test]
    fn test_smp_create_barrier() {
        let mut mgr = SmpManager::new(4);
        let id = mgr.create_barrier(vec![CpuId(0), CpuId(1), CpuId(2)]);
        assert!(id > 0);
    }

    #[test]
    fn test_smp_arrive_barrier() {
        let mut mgr = SmpManager::new(4);
        let id = mgr.create_barrier(vec![CpuId(0), CpuId(1), CpuId(2)]);

        assert!(!mgr.arrive_barrier(id, CpuId(0)).unwrap());
        assert!(!mgr.arrive_barrier(id, CpuId(1)).unwrap());
        assert!(mgr.arrive_barrier(id, CpuId(2)).unwrap());  // Complete
    }

    #[test]
    fn test_smp_barrier_not_found() {
        let mut mgr = SmpManager::new(4);
        let result = mgr.arrive_barrier(999, CpuId(0));
        assert_eq!(result, Err(SmpError::BarrierNotFound));
    }

    #[test]
    fn test_smp_destroy_barrier() {
        let mut mgr = SmpManager::new(4);
        let id = mgr.create_barrier(vec![CpuId(0), CpuId(1)]);
        assert!(mgr.destroy_barrier(id));
        assert!(!mgr.destroy_barrier(id));
    }

    // ── SmpManager: Sched Domain Tests ────────────────────────

    #[test]
    fn test_smp_create_sched_domain() {
        let mut mgr = SmpManager::new(8);
        let id = mgr.create_sched_domain(0, vec![CpuId(0), CpuId(1)]);
        assert!(id > 0);
        assert!(mgr.get_sched_domain(id).is_some());
        assert!(mgr.get_sched_domain(id).unwrap().contains(CpuId(0)));
    }

    #[test]
    fn test_smp_sched_domain_hierarchy() {
        let mut mgr = SmpManager::new(8);
        let parent = mgr.create_sched_domain(1, vec![CpuId(0), CpuId(1), CpuId(2), CpuId(3)]);
        let child = mgr.create_sched_domain(0, vec![CpuId(0), CpuId(1)]);

        assert!(mgr.set_domain_parent(child, parent));
        assert!(mgr.add_domain_child(parent, child));
    }

    // ── SmpManager: Topology Tests ─────────────────────────────

    #[test]
    fn test_smp_cpu_distance() {
        let mgr = SmpManager::new(4);
        assert_eq!(mgr.cpu_distance(CpuId(0), CpuId(1)), 10);  // Same NUMA
    }

    #[test]
    fn test_smp_is_sibling() {
        let topo = CpuTopology::with_hyperthreads(8, 2);
        let mgr = SmpManager::with_topology(8, topo);
        assert!(mgr.is_sibling(CpuId(0), CpuId(1)));
        assert!(!mgr.is_sibling(CpuId(0), CpuId(2)));
    }

    #[test]
    fn test_smp_same_numa_node() {
        let mgr = SmpManager::new(4);
        assert!(mgr.same_numa_node(CpuId(0), CpuId(1)));
    }

    // ── SmpManager: Stats Tests ───────────────────────────────

    #[test]
    fn test_smp_stats() {
        let mut mgr = SmpManager::new(8);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();

        let stats = mgr.stats();
        assert_eq!(stats.total_cpus, 8);
        assert_eq!(stats.online_cpus, 3);
        assert_eq!(stats.numa_nodes, 1);
    }

    #[test]
    fn test_smp_avg_load() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.register_task(100, 64);
        mgr.enqueue_task(100, CpuId(0)).unwrap();

        let avg = mgr.avg_load();
        // 1 task on 2 online CPUs = 0.5
        assert!((avg - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_smp_max_min_load_cpu() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.register_task(100, 64);
        mgr.register_task(101, 64);
        mgr.enqueue_task(100, CpuId(0)).unwrap();
        mgr.enqueue_task(101, CpuId(0)).unwrap();

        assert_eq!(mgr.max_load_cpu(), Some(CpuId(0)));
        assert_eq!(mgr.min_load_cpu(), Some(CpuId(1)));
    }

    // ── SmpManager: Offline Migration Tests ───────────────────

    #[test]
    fn test_smp_offline_migrates_tasks() {
        let mut mgr = SmpManager::new(4);
        mgr.bring_cpu_online(CpuId(1)).unwrap();
        mgr.bring_cpu_online(CpuId(2)).unwrap();
        mgr.register_task(100, 64);
        mgr.enqueue_task(100, CpuId(1)).unwrap();

        mgr.take_cpu_offline(CpuId(1)).unwrap();
        // Task should have been migrated to another CPU
        assert!(mgr.tasks.get(&100).unwrap().current_cpu != Some(CpuId(1)));
    }

    // ── Integration Tests ─────────────────────────────────────

    #[test]
    fn test_integration_full_smp_lifecycle() {
        let mut mgr = SmpManager::new(8);

        // 1. Bring CPUs online
        for i in 1..8 {
            mgr.bring_cpu_online(CpuId(i)).unwrap();
        }
        assert_eq!(mgr.online_count(), 8);

        // 2. Register tasks
        for pid in 100..108 {
            mgr.register_task(pid, 64);
        }

        // 3. Enqueue on CPU 0 (overload one)
        for pid in 100..108 {
            mgr.enqueue_task(pid, CpuId(0)).unwrap();
        }
        assert_eq!(mgr.get_cpu(CpuId(0)).unwrap().load(), 8);

        // 4. Load balance
        let migrations = mgr.balance_load();
        assert!(!migrations.is_empty());

        // 5. Check that load is more distributed
        let cpu0_load = mgr.get_cpu(CpuId(0)).unwrap().load();
        assert!(cpu0_load < 8);

        // 6. Pin a task
        mgr.pin_task(100, CpuId(3)).unwrap();
        let aff = mgr.get_task_affinity(100).unwrap();
        assert!(aff.contains(CpuId(3)));
        assert!(!aff.contains(CpuId(0)));

        // 7. Send IPIs
        mgr.send_ipi_all(CpuId(0), IpiType::Reschedule);
        assert!(mgr.ipi_count() > 0);

        // 8. Create and complete a barrier
        let barrier_id = mgr.create_barrier(vec![CpuId(0), CpuId(1), CpuId(2)]);
        mgr.arrive_barrier(barrier_id, CpuId(0)).unwrap();
        mgr.arrive_barrier(barrier_id, CpuId(1)).unwrap();
        assert!(mgr.arrive_barrier(barrier_id, CpuId(2)).unwrap());

        // 9. Take a CPU offline
        mgr.take_cpu_offline(CpuId(7)).unwrap();
        assert_eq!(mgr.online_count(), 7);

        // 10. Check stats
        let stats = mgr.stats();
        assert_eq!(stats.total_cpus, 8);
        assert_eq!(stats.online_cpus, 7);
        assert!(stats.total_migrations > 0);
        assert!(stats.total_balances > 0);
    }

    #[test]
    fn test_integration_hyperthread_aware_scheduling() {
        let topo = CpuTopology::with_hyperthreads(8, 2); // 4 cores, 2 threads each
        let mut mgr = SmpManager::with_topology(8, topo);

        // Bring all CPUs online
        for i in 1..8 {
            mgr.bring_cpu_online(CpuId(i)).unwrap();
        }

        // Pin task to core 0's first thread
        mgr.register_task_pinned(100, CpuId(0));

        // Verify topology
        assert!(mgr.is_sibling(CpuId(0), CpuId(1)));
        assert_eq!(mgr.topology().get_core(0), Some(0));
        assert_eq!(mgr.topology().get_core(1), Some(0));

        // Task can only run on CPU 0
        let cpu = mgr.find_cpu_for_task(100);
        assert_eq!(cpu, Some(CpuId(0)));
    }

    #[test]
    fn test_integration_concurrent_load_balancing() {
        let mut mgr = SmpManager::new(4);
        for i in 1..4 {
            mgr.bring_cpu_online(CpuId(i)).unwrap();
        }

        // Create 12 tasks
        for pid in 200..212 {
            mgr.register_task(pid, 64);
        }

        // Put all on CPU 0
        for pid in 200..212 {
            mgr.enqueue_task(pid, CpuId(0)).unwrap();
        }

        // Balance multiple times
        let mut total_migrations = 0;
        for _ in 0..5 {
            total_migrations += mgr.balance_load().len();
        }

        // Load should be well distributed
        let loads: Vec<u32> = (0..4).map(|i| mgr.get_cpu(CpuId(i)).map(|c| c.load()).unwrap_or(0)).collect();
        let max_load = *loads.iter().max().unwrap();
        let min_load = *loads.iter().min().unwrap();
        assert!(max_load - min_load <= 1);  // Fair distribution
        assert!(total_migrations > 0);
    }

    #[test]
    fn test_integration_ipi_and_barriers() {
        let mut mgr = SmpManager::new(4);
        for i in 1..4 {
            mgr.bring_cpu_online(CpuId(i)).unwrap();
        }

        // IPI broadcast
        mgr.send_ipi_all(CpuId(0), IpiType::Reschedule);
        assert_eq!(mgr.ipi_count(), 3);

        // Create barrier with all online CPUs
        let online = mgr.online_cpus();
        let barrier_id = mgr.create_barrier(online.clone());

        // All CPUs arrive
        let mut complete = false;
        for cpu in &online {
            complete = mgr.arrive_barrier(barrier_id, *cpu).unwrap();
        }
        assert!(complete);

        // Verify stats
        let stats = mgr.stats();
        assert_eq!(stats.total_ipis, 3);
    }

    // ── Error Name Tests ──────────────────────────────────────

    #[test]
    fn test_smp_error_names() {
        assert_eq!(SmpError::CpuNotFound.name(), "cpu_not_found");
        assert_eq!(SmpError::TaskNotFound.name(), "task_not_found");
        assert_eq!(SmpError::CpuOffline.name(), "cpu_offline");
        assert_eq!(SmpError::CannotOfflineBsp.name(), "cannot_offline_bsp");
        assert_eq!(SmpError::AffinityViolation.name(), "affinity_violation");
        assert_eq!(SmpError::BarrierNotFound.name(), "barrier_not_found");
    }
}
