// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 46: Kernel Tracing & Profiling
// ══════════════════════════════════════════════════════════════════════════════
// ftrace / strace / perf-Äquivalent für Kernel-Observability.
// Ring Buffer, Function Tracing, Syscall Tracing, Event Filtering,
// Histograms, Latency Tracking, Profiling Samples.
// ══════════════════════════════════════════════════════════════════════════════

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

// ══════════════════════════════════════════════════════════════════════════════
// GLOBAL COUNTERS
// ══════════════════════════════════════════════════════════════════════════════

static TRACE_SEQ: AtomicU64 = AtomicU64::new(0);
static EVENT_SEQ: AtomicU64 = AtomicU64::new(0);
static SAMPLE_SEQ: AtomicU64 = AtomicU64::new(0);

fn next_trace_seq() -> u64 { TRACE_SEQ.fetch_add(1, Ordering::SeqCst) }
fn next_event_seq() -> u64 { EVENT_SEQ.fetch_add(1, Ordering::SeqCst) }
fn next_sample_seq() -> u64 { SAMPLE_SEQ.fetch_add(1, Ordering::SeqCst) }

// ══════════════════════════════════════════════════════════════════════════════
// TRACE EVENT TYPES
// ══════════════════════════════════════════════════════════════════════════════

/// Trace point category
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TraceCategory {
    Function,       // Function entry/exit
    Syscall,         // System call
    Irq,             // Interrupt handler
    Sched,           // Scheduler events
    Mem,             // Memory management
    Net,             // Network
    Block,           // Block I/O
    Signal,          // Signal delivery
    Container,       // Container lifecycle
    User,            // Userspace event
    Custom,          // User-defined
}

impl TraceCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Function  => "function",
            Self::Syscall   => "syscall",
            Self::Irq       => "irq",
            Self::Sched     => "sched",
            Self::Mem       => "mem",
            Self::Net       => "net",
            Self::Block     => "block",
            Self::Signal    => "signal",
            Self::Container => "container",
            Self::User      => "user",
            Self::Custom    => "custom",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "function"  => Some(Self::Function),
            "syscall"   => Some(Self::Syscall),
            "irq"       => Some(Self::Irq),
            "sched"     => Some(Self::Sched),
            "mem"       => Some(Self::Mem),
            "net"       => Some(Self::Net),
            "block"     => Some(Self::Block),
            "signal"    => Some(Self::Signal),
            "container" => Some(Self::Container),
            "user"      => Some(Self::User),
            "custom"    => Some(Self::Custom),
            _           => None,
        }
    }
}

/// Trace event type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TraceEventType {
    Entry,      // Function/region entered
    Exit,       // Function/region exited
    Event,      // One-off event
    Begin,      // Transaction begin
    End,        // Transaction end
}

impl TraceEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Entry => "entry",
            Self::Exit  => "exit",
            Self::Event => "event",
            Self::Begin => "begin",
            Self::End   => "end",
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// TRACE EVENT
// ══════════════════════════════════════════════════════════════════════════════

/// A single trace event in the ring buffer
#[derive(Clone, Debug)]
pub struct TraceEvent {
    pub seq:        u64,           // Global sequence number
    pub timestamp:  u64,           // Timestamp (nanoseconds, monotonic)
    pub cpu:        u32,           // CPU ID
    pub pid:        u32,           // Process ID (0 = kernel)
    pub category:   TraceCategory,  // Event category
    pub event_type: TraceEventType, // Entry/Exit/Event
    pub name:       String,        // Function/event name
    pub arg1:       u64,           // Custom argument 1
    pub arg2:       u64,           // Custom argument 2
    pub arg3:       u64,           // Custom argument 3
    pub duration_ns: Option<u64>,  // Duration (for Exit events, None for Entry)
}

impl TraceEvent {
    /// Create a function entry event
    pub fn function_entry(cpu: u32, pid: u32, name: &str) -> Self {
        Self {
            seq: next_trace_seq(),
            timestamp: 0, // Set by ring buffer
            cpu,
            pid,
            category: TraceCategory::Function,
            event_type: TraceEventType::Entry,
            name: name.to_string(),
            arg1: 0, arg2: 0, arg3: 0,
            duration_ns: None,
        }
    }

    /// Create a function exit event with duration
    pub fn function_exit(cpu: u32, pid: u32, name: &str, duration_ns: u64) -> Self {
        Self {
            seq: next_trace_seq(),
            timestamp: 0,
            cpu,
            pid,
            category: TraceCategory::Function,
            event_type: TraceEventType::Exit,
            name: name.to_string(),
            arg1: 0, arg2: 0, arg3: 0,
            duration_ns: Some(duration_ns),
        }
    }

    /// Create a syscall event
    pub fn syscall(cpu: u32, pid: u32, syscall_name: &str, ret: u64) -> Self {
        Self {
            seq: next_trace_seq(),
            timestamp: 0,
            cpu,
            pid,
            category: TraceCategory::Syscall,
            event_type: TraceEventType::Event,
            name: syscall_name.to_string(),
            arg1: ret,
            arg2: 0, arg3: 0,
            duration_ns: None,
        }
    }

    /// Create a scheduler event
    pub fn sched_event(cpu: u32, pid: u32, event_name: &str, next_pid: u32) -> Self {
        Self {
            seq: next_trace_seq(),
            timestamp: 0,
            cpu,
            pid,
            category: TraceCategory::Sched,
            event_type: TraceEventType::Event,
            name: event_name.to_string(),
            arg1: next_pid as u64,
            arg2: 0, arg3: 0,
            duration_ns: None,
        }
    }

    /// Create a memory event
    pub fn mem_event(cpu: u32, pid: u32, event_name: &str, addr: u64, size: u64) -> Self {
        Self {
            seq: next_trace_seq(),
            timestamp: 0,
            cpu,
            pid,
            category: TraceCategory::Mem,
            event_type: TraceEventType::Event,
            name: event_name.to_string(),
            arg1: addr,
            arg2: size,
            arg3: 0,
            duration_ns: None,
        }
    }

    /// Create a custom event
    pub fn custom(cpu: u32, pid: u32, name: &str, a1: u64, a2: u64, a3: u64) -> Self {
        Self {
            seq: next_trace_seq(),
            timestamp: 0,
            cpu,
            pid,
            category: TraceCategory::Custom,
            event_type: TraceEventType::Event,
            name: name.to_string(),
            arg1: a1, arg2: a2, arg3: a3,
            duration_ns: None,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// RING BUFFER
// ══════════════════════════════════════════════════════════════════════════════

/// Lock-free ring buffer for trace events
pub struct RingBuffer {
    events:      VecDeque<TraceEvent>,
    capacity:    usize,
    total_count: u64,        // Total events ever recorded
    dropped:     u64,        // Events dropped due to full buffer
    start_time:  u64,        // Buffer creation time
    enabled:     bool,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(capacity),
            capacity,
            total_count: 0,
            dropped: 0,
            start_time: 0,
            enabled: true,
        }
    }

    /// Push an event into the ring buffer
    pub fn push(&mut self, mut event: TraceEvent, timestamp_ns: u64) -> bool {
        if !self.enabled { return false; }
        event.timestamp = timestamp_ns;
        self.total_count += 1;
        if self.events.len() >= self.capacity {
            self.events.pop_front();
            self.dropped += 1;
        }
        self.events.push_back(event);
        true
    }

    /// Get all events in the buffer
    pub fn events(&self) -> &VecDeque<TraceEvent> { &self.events }

    /// Get events filtered by category
    pub fn events_by_category(&self, cat: TraceCategory) -> Vec<&TraceEvent> {
        self.events.iter().filter(|e| e.category == cat).collect()
    }

    /// Get events filtered by PID
    pub fn events_by_pid(&self, pid: u32) -> Vec<&TraceEvent> {
        self.events.iter().filter(|e| e.pid == pid).collect()
    }

    /// Get events filtered by CPU
    pub fn events_by_cpu(&self, cpu: u32) -> Vec<&TraceEvent> {
        self.events.iter().filter(|e| e.cpu == cpu).collect()
    }

    /// Get events in time range [start_ns, end_ns)
    pub fn events_in_range(&self, start_ns: u64, end_ns: u64) -> Vec<&TraceEvent> {
        self.events.iter()
            .filter(|e| e.timestamp >= start_ns && e.timestamp < end_ns)
            .collect()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Enable tracing
    pub fn enable(&mut self) { self.enabled = true; }

    /// Disable tracing (events are silently dropped)
    pub fn disable(&mut self) { self.enabled = false; }

    /// Is tracing enabled?
    pub fn is_enabled(&self) -> bool { self.enabled }

    /// Current buffer utilization (0.0 - 1.0)
    pub fn utilization(&self) -> f64 {
        self.events.len() as f64 / self.capacity as f64
    }

    /// Total events recorded (including dropped)
    pub fn total_count(&self) -> u64 { self.total_count }

    /// Events dropped due to full buffer
    pub fn dropped_count(&self) -> u64 { self.dropped }

    /// Buffer capacity
    pub fn capacity(&self) -> usize { self.capacity }

    /// Current event count
    pub fn len(&self) -> usize { self.events.len() }

    /// Is buffer empty?
    pub fn is_empty(&self) -> bool { self.events.is_empty() }
}

// ══════════════════════════════════════════════════════════════════════════════
// TRACE FILTER
// ══════════════════════════════════════════════════════════════════════════════

/// Filter for trace events
#[derive(Clone, Default)]
pub struct TraceFilter {
    pub categories:   BTreeSet<TraceCategory>,  // Empty = all
    pub pids:         BTreeSet<u32>,            // Empty = all
    pub cpus:         BTreeSet<u32>,             // Empty = all
    pub name_substring: Option<String>,          // Filter by name substring
    pub min_duration_ns: Option<u64>,            // Only show events >= this duration
}

impl TraceFilter {
    pub fn new() -> Self { Self::default() }

    pub fn category(mut self, cat: TraceCategory) -> Self {
        self.categories.insert(cat);
        self
    }

    pub fn pid(mut self, pid: u32) -> Self {
        self.pids.insert(pid);
        self
    }

    pub fn cpu(mut self, cpu: u32) -> Self {
        self.cpus.insert(cpu);
        self
    }

    pub fn name_contains(mut self, substr: &str) -> Self {
        self.name_substring = Some(substr.to_string());
        self
    }

    pub fn min_duration(mut self, ns: u64) -> Self {
        self.min_duration_ns = Some(ns);
        self
    }

    /// Check if an event matches the filter
    pub fn matches(&self, event: &TraceEvent) -> bool {
        if !self.categories.is_empty() && !self.categories.contains(&event.category) {
            return false;
        }
        if !self.pids.is_empty() && !self.pids.contains(&event.pid) {
            return false;
        }
        if !self.cpus.is_empty() && !self.cpus.contains(&event.cpu) {
            return false;
        }
        if let Some(ref substr) = self.name_substring {
            if !event.name.contains(substr) {
                return false;
            }
        }
        if let Some(min_dur) = self.min_duration_ns {
            match event.duration_ns {
                Some(d) if d >= min_dur => {}
                Some(_) => return false,
                None => return false,
            }
        }
        true
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// HISTOGRAM
// ══════════════════════════════════════════════════════════════════════════════

/// Latency/value histogram for performance analysis
pub struct Histogram {
    pub name:       String,
    pub buckets:     BTreeMap<u64, u64>,   // upper_bound -> count
    pub min_value:   u64,
    pub max_value:   u64,
    pub sum:         u64,
    pub count:       u64,
    pub bucket_bounds: Vec<u64>,           // Custom bucket boundaries
}

impl Histogram {
    pub fn new(name: &str, bucket_bounds: Vec<u64>) -> Self {
        Self {
            name: name.to_string(),
            buckets: BTreeMap::new(),
            min_value: u64::MAX,
            max_value: 0,
            sum: 0,
            count: 0,
            bucket_bounds,
        }
    }

    /// Default latency histogram (1us, 10us, 100us, 1ms, 10ms, 100ms, 1s)
    pub fn latency(name: &str) -> Self {
        Self::new(name, vec![1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000])
    }

    /// Record a value
    pub fn record(&mut self, value: u64) {
        self.count += 1;
        self.sum += value;
        if value < self.min_value { self.min_value = value; }
        if value > self.max_value { self.max_value = value; }

        // Find bucket
        let bucket = self.bucket_bounds.iter()
            .find(|&&bound| value <= bound)
            .copied()
            .unwrap_or(u64::MAX);

        *self.buckets.entry(bucket).or_insert(0) += 1;
    }

    /// Get mean value
    pub fn mean(&self) -> f64 {
        if self.count == 0 { return 0.0; }
        self.sum as f64 / self.count as f64
    }

    /// Get percentile (0-100)
    pub fn percentile(&self, p: f64) -> u64 {
        if self.count == 0 { return 0; }
        let target = (self.count as f64 * p / 100.0).ceil() as u64;
        let mut cumulative = 0u64;
        for (&bound, &count) in &self.buckets {
            cumulative += count;
            if cumulative >= target {
                return bound;
            }
        }
        self.max_value
    }

    /// Get p50 (median)
    pub fn p50(&self) -> u64 { self.percentile(50.0) }

    /// Get p95
    pub fn p95(&self) -> u64 { self.percentile(95.0) }

    /// Get p99
    pub fn p99(&self) -> u64 { self.percentile(99.0) }

    /// Reset histogram
    pub fn reset(&mut self) {
        self.buckets.clear();
        self.min_value = u64::MAX;
        self.max_value = 0;
        self.sum = 0;
        self.count = 0;
    }

    /// Format as a string report
    pub fn report(&self) -> String {
        let mut s = format!(
            "Histogram: {}\n  Count: {}\n  Min: {} ns\n  Max: {} ns\n  Mean: {:.1} ns\n  P50: {} ns\n  P95: {} ns\n  P99: {} ns\n",
            self.name, self.count, self.min_value, self.max_value,
            self.mean(), self.p50(), self.p95(), self.p99()
        );
        s.push_str("  Buckets:\n");
        for (&bound, &count) in &self.buckets {
            let bar = "█".repeat((count as f64 / self.count as f64 * 40.0) as usize);
            s.push_str(&format!("    {:>12} ns: {} ({}%)\n", bound, count, count * 100 / self.count.max(1)));
        }
        s
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// FUNCTION TRACER
// ══════════════════════════════════════════════════════════════════════════════

/// Function call stack entry
#[derive(Clone, Debug)]
pub struct CallStackEntry {
    pub name:       String,
    pub entry_time:  u64,   // Entry timestamp (ns)
    pub cpu:        u32,
    pub pid:        u32,
    pub args:       [u64; 3],
}

/// Per-CPU function call stack
pub struct FunctionTracer {
    stacks:         HashMap<u32, Vec<CallStackEntry>>,  // cpu -> call stack
    max_depth:      usize,
    enabled:        bool,
    filter:         TraceFilter,
    call_counts:    HashMap<String, u64>,    // function -> call count
    total_time:     HashMap<String, u64>,    // function -> total time (ns)
    max_depth_reached: usize,
}

impl FunctionTracer {
    pub fn new(max_depth: usize) -> Self {
        Self {
            stacks: HashMap::new(),
            max_depth,
            enabled: false,
            filter: TraceFilter::new(),
            call_counts: HashMap::new(),
            total_time: HashMap::new(),
            max_depth_reached: 0,
        }
    }

    /// Enable function tracing
    pub fn enable(&mut self) { self.enabled = true; }

    /// Disable function tracing
    pub fn disable(&mut self) { self.enabled = false; }

    /// Set filter
    pub fn set_filter(&mut self, filter: TraceFilter) {
        self.filter = filter;
    }

    /// Record function entry
    pub fn enter(&mut self, cpu: u32, pid: u32, name: &str, timestamp_ns: u64, args: [u64; 3]) -> bool {
        if !self.enabled { return false; }

        let event = TraceEvent {
            seq: next_trace_seq(),
            timestamp: timestamp_ns,
            cpu, pid,
            category: TraceCategory::Function,
            event_type: TraceEventType::Entry,
            name: name.to_string(),
            arg1: args[0], arg2: args[1], arg3: args[2],
            duration_ns: None,
        };

        if !self.filter.matches(&event) { return false; }

        let stack = self.stacks.entry(cpu).or_insert_with(Vec::new);
        if stack.len() >= self.max_depth {
            return false;  // Stack overflow protection
        }
        stack.push(CallStackEntry {
            name: name.to_string(),
            entry_time: timestamp_ns,
            cpu, pid,
            args,
        });
        if stack.len() > self.max_depth_reached {
            self.max_depth_reached = stack.len();
        }

        *self.call_counts.entry(name.to_string()).or_insert(0) += 1;
        true
    }

    /// Record function exit and return duration
    pub fn exit(&mut self, cpu: u32, timestamp_ns: u64) -> Option<(String, u64)> {
        if !self.enabled { return None; }

        let stack = self.stacks.get_mut(&cpu)?;
        let entry = stack.pop()?;

        let duration = timestamp_ns.saturating_sub(entry.entry_time);
        *self.total_time.entry(entry.name.clone()).or_insert(0) += duration;

        Some((entry.name, duration))
    }

    /// Get current call stack depth for a CPU
    pub fn depth(&self, cpu: u32) -> usize {
        self.stacks.get(&cpu).map(|s| s.len()).unwrap_or(0)
    }

    /// Get current call stack for a CPU
    pub fn call_stack(&self, cpu: u32) -> Vec<&CallStackEntry> {
        self.stacks.get(&cpu).map(|s| s.iter().collect()).unwrap_or_default()
    }

    /// Get call count for a function
    pub fn call_count(&self, name: &str) -> u64 {
        self.call_counts.get(name).copied().unwrap_or(0)
    }

    /// Get total time spent in a function
    pub fn total_time_ns(&self, name: &str) -> u64 {
        self.total_time.get(name).copied().unwrap_or(0)
    }

    /// Get average time per call for a function
    pub fn avg_time_ns(&self, name: &str) -> f64 {
        let count = self.call_count(name);
        if count == 0 { return 0.0; }
        self.total_time_ns(name) as f64 / count as f64
    }

    /// Get all traced function names
    pub fn traced_functions(&self) -> Vec<String> {
        self.call_counts.keys().cloned().collect()
    }

    /// Get top N functions by total time
    pub fn top_by_time(&self, n: usize) -> Vec<(String, u64)> {
        let mut entries: Vec<(String, u64)> = self.total_time.iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(n);
        entries
    }

    /// Get top N functions by call count
    pub fn top_by_count(&self, n: usize) -> Vec<(String, u64)> {
        let mut entries: Vec<(String, u64)> = self.call_counts.iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(n);
        entries
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.call_counts.clear();
        self.total_time.clear();
        self.max_depth_reached = 0;
    }

    /// Get maximum depth reached
    pub fn max_depth_reached(&self) -> usize { self.max_depth_reached }
}

// ══════════════════════════════════════════════════════════════════════════════
// SYSCALL TRACER (strace equivalent)
// ══════════════════════════════════════════════════════════════════════════════

/// Syscall trace record
#[derive(Clone, Debug)]
pub struct SyscallRecord {
    pub seq:       u64,
    pub timestamp:  u64,
    pub pid:        u32,
    pub cpu:        u32,
    pub syscall:    String,
    pub args:       [u64; 6],
    pub ret:        i64,
    pub duration_ns: u64,
    pub error:      Option<String>,
}

/// Syscall tracer
pub struct SyscallTracer {
    records:       VecDeque<SyscallRecord>,
    capacity:      usize,
    enabled:       bool,
    traced_pids:    BTreeSet<u32>,    // Empty = all PIDs
    traced_syscalls: BTreeSet<String>, // Empty = all syscalls
    counts:        HashMap<String, u64>,  // syscall -> count
    errors:        HashMap<String, u64>,   // syscall -> error count
    total_time:    HashMap<String, u64>,   // syscall -> total duration
}

impl SyscallTracer {
    pub fn new(capacity: usize) -> Self {
        Self {
            records: VecDeque::with_capacity(capacity),
            capacity,
            enabled: false,
            traced_pids: BTreeSet::new(),
            traced_syscalls: BTreeSet::new(),
            counts: HashMap::new(),
            errors: HashMap::new(),
            total_time: HashMap::new(),
        }
    }

    pub fn enable(&mut self) { self.enabled = true; }
    pub fn disable(&mut self) { self.enabled = false; }

    pub fn trace_pid(&mut self, pid: u32) {
        self.traced_pids.insert(pid);
    }

    pub fn trace_syscall(&mut self, name: &str) {
        self.traced_syscalls.insert(name.to_string());
    }

    pub fn trace_all_pids(&mut self) {
        self.traced_pids.clear();
    }

    pub fn trace_all_syscalls(&mut self) {
        self.traced_syscalls.clear();
    }

    /// Record a syscall
    pub fn record(
        &mut self,
        pid: u32, cpu: u32, syscall: &str,
        args: [u64; 6], ret: i64,
        duration_ns: u64, timestamp_ns: u64,
        error: Option<String>,
    ) -> bool {
        if !self.enabled { return false; }

        // PID filter
        if !self.traced_pids.is_empty() && !self.traced_pids.contains(&pid) {
            return false;
        }

        // Syscall filter
        if !self.traced_syscalls.is_empty() && !self.traced_syscalls.contains(syscall) {
            return false;
        }

        let record = SyscallRecord {
            seq: next_event_seq(),
            timestamp: timestamp_ns,
            pid, cpu,
            syscall: syscall.to_string(),
            args, ret,
            duration_ns,
            error: error.clone(),
        };

        *self.counts.entry(syscall.to_string()).or_insert(0) += 1;
        *self.total_time.entry(syscall.to_string()).or_insert(0) += duration_ns;
        if error.is_some() {
            *self.errors.entry(syscall.to_string()).or_insert(0) += 1;
        }

        if self.records.len() >= self.capacity {
            self.records.pop_front();
        }
        self.records.push_back(record);
        true
    }

    /// Get all syscall records
    pub fn records(&self) -> &VecDeque<SyscallRecord> { &self.records }

    /// Get records for a specific PID
    pub fn records_for_pid(&self, pid: u32) -> Vec<&SyscallRecord> {
        self.records.iter().filter(|r| r.pid == pid).collect()
    }

    /// Get records for a specific syscall
    pub fn records_for_syscall(&self, name: &str) -> Vec<&SyscallRecord> {
        self.records.iter().filter(|r| r.syscall == name).collect()
    }

    /// Get syscall count
    pub fn count(&self, syscall: &str) -> u64 {
        self.counts.get(syscall).copied().unwrap_or(0)
    }

    /// Get error count for a syscall
    pub fn error_count(&self, syscall: &str) -> u64 {
        self.errors.get(syscall).copied().unwrap_or(0)
    }

    /// Get total time for a syscall
    pub fn total_time_ns(&self, syscall: &str) -> u64 {
        self.total_time.get(syscall).copied().unwrap_or(0)
    }

    /// Get average duration for a syscall
    pub fn avg_duration_ns(&self, syscall: &str) -> f64 {
        let count = self.count(syscall);
        if count == 0 { return 0.0; }
        self.total_time_ns(syscall) as f64 / count as f64
    }

    /// Get error rate for a syscall
    pub fn error_rate(&self, syscall: &str) -> f64 {
        let count = self.count(syscall);
        if count == 0 { return 0.0; }
        self.error_count(syscall) as f64 / count as f64
    }

    /// Get summary report
    pub fn summary(&self) -> String {
        let mut s = format!("Syscall Trace Summary ({} records)\n", self.records.len());
        let mut syscalls: Vec<(&String, &u64)> = self.counts.iter().collect();
        syscalls.sort_by(|a, b| b.1.cmp(a.1));
        for (name, count) in syscalls.iter().take(20) {
            let avg = self.avg_duration_ns(name);
            let err = self.error_rate(name);
            s.push_str(&format!(
                "  {:20s} count={:<6} avg={:>8.1}ns err={:.1}%\n",
                name, count, avg, err * 100.0
            ));
        }
        s
    }

    /// Clear all records
    pub fn clear(&mut self) {
        self.records.clear();
        self.counts.clear();
        self.errors.clear();
        self.total_time.clear();
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// PROFILING SAMPLER
// ══════════════════════════════════════════════════════════════════════════════

/// A profiling sample
#[derive(Clone, Debug)]
pub struct ProfileSample {
    pub seq:       u64,
    pub timestamp:  u64,
    pub cpu:        u32,
    pub pid:        u32,
    pub pc:         u64,      // Program counter
    pub stack:      Vec<u64>,  // Call stack (addresses)
    pub label:      String,    // Function label (if known)
    pub in_kernel:  bool,
}

/// Profiling sampler (perf equivalent)
pub struct Profiler {
    samples:        VecDeque<ProfileSample>,
    capacity:       usize,
    enabled:        bool,
    sample_period_ns: u64,     // Sample every N ns
    last_sample:    u64,       // Last sample timestamp
    pid_filter:     BTreeSet<u32>,
    kernel_only:    bool,
    // Statistics
    sample_count:   u64,
    by_function:    HashMap<String, u64>,
    by_pid:         HashMap<u32, u64>,
    by_cpu:         HashMap<u32, u64>,
    kernel_samples: u64,
    user_samples:   u64,
}

impl Profiler {
    pub fn new(capacity: usize, sample_period_ns: u64) -> Self {
        Self {
            samples: VecDeque::with_capacity(capacity),
            capacity,
            enabled: false,
            sample_period_ns,
            last_sample: 0,
            pid_filter: BTreeSet::new(),
            kernel_only: false,
            sample_count: 0,
            by_function: HashMap::new(),
            by_pid: HashMap::new(),
            by_cpu: HashMap::new(),
            kernel_samples: 0,
            user_samples: 0,
        }
    }

    pub fn enable(&mut self) { self.enabled = true; }
    pub fn disable(&mut self) { self.enabled = false; }

    pub fn set_period(&mut self, period_ns: u64) {
        self.sample_period_ns = period_ns;
    }

    pub fn trace_pid(&mut self, pid: u32) {
        self.pid_filter.insert(pid);
    }

    pub fn kernel_only(&mut self) {
        self.kernel_only = true;
    }

    pub fn user_only(&mut self) {
        self.kernel_only = false;
    }

    /// Take a sample (returns true if sample was recorded)
    pub fn sample(
        &mut self,
        cpu: u32, pid: u32, pc: u64,
        stack: Vec<u64>, label: &str,
        in_kernel: bool, timestamp_ns: u64,
    ) -> bool {
        if !self.enabled { return false; }

        // Check sample period
        if timestamp_ns < self.last_sample + self.sample_period_ns {
            return false;
        }

        // PID filter
        if !self.pid_filter.is_empty() && !self.pid_filter.contains(&pid) {
            return false;
        }

        // Kernel filter
        if self.kernel_only && !in_kernel {
            return false;
        }

        let sample = ProfileSample {
            seq: next_sample_seq(),
            timestamp: timestamp_ns,
            cpu, pid, pc,
            stack,
            label: label.to_string(),
            in_kernel,
        };

        self.last_sample = timestamp_ns;
        self.sample_count += 1;

        *self.by_function.entry(sample.label.clone()).or_insert(0) += 1;
        *self.by_pid.entry(pid).or_insert(0) += 1;
        *self.by_cpu.entry(cpu).or_insert(0) += 1;

        if in_kernel {
            self.kernel_samples += 1;
        } else {
            self.user_samples += 1;
        }

        if self.samples.len() >= self.capacity {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
        true
    }

    /// Get top N functions by sample count (hot functions)
    pub fn hot_functions(&self, n: usize) -> Vec<(String, u64)> {
        let mut entries: Vec<(String, u64)> = self.by_function.iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(n);
        entries
    }

    /// Get top N PIDs by sample count
    pub fn hot_pids(&self, n: usize) -> Vec<(u32, u64)> {
        let mut entries: Vec<(u32, u64)> = self.by_pid.iter()
            .map(|(&k, &v)| (k, v))
            .collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(n);
        entries
    }

    /// Get samples per CPU
    pub fn samples_per_cpu(&self, cpu: u32) -> u64 {
        self.by_cpu.get(&cpu).copied().unwrap_or(0)
    }

    /// Kernel vs user ratio (0.0 - 1.0 kernel fraction)
    pub fn kernel_ratio(&self) -> f64 {
        if self.sample_count == 0 { return 0.0; }
        self.kernel_samples as f64 / self.sample_count as f64
    }

    /// Total samples taken
    pub fn total_samples(&self) -> u64 { self.sample_count }

    /// Generate a profile report
    pub fn report(&self) -> String {
        let mut s = format!(
            "Profile Report ({} samples)\n  Kernel: {} ({:.1}%)\n  User:   {} ({:.1}%)\n\n",
            self.sample_count,
            self.kernel_samples, self.kernel_ratio() * 100.0,
            self.user_samples, (1.0 - self.kernel_ratio()) * 100.0
        );
        s.push_str("Top 20 Hot Functions:\n");
        for (name, count) in self.hot_functions(20) {
            let pct = count * 100 / self.sample_count.max(1);
            let bar = "█".repeat((pct as usize).min(40));
            s.push_str(&format!("  {:30s} {:>6} ({}%) {}\n", name, count, pct, bar));
        }
        s.push_str("\nPer-CPU Distribution:\n");
        for (&cpu, &count) in &self.by_cpu {
            s.push_str(&format!("  CPU {}: {} samples\n", cpu, count));
        }
        s
    }

    /// Clear all samples
    pub fn clear(&mut self) {
        self.samples.clear();
        self.sample_count = 0;
        self.by_function.clear();
        self.by_pid.clear();
        self.by_cpu.clear();
        self.kernel_samples = 0;
        self.user_samples = 0;
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// LATENCY TRACKER
// ══════════════════════════════════════════════════════════════════════════════

/// Latency tracker for critical paths
pub struct LatencyTracker {
    histograms:    HashMap<String, Histogram>,
    worst_case:    HashMap<String, u64>,
    threshold_ns:  u64,       // Alert threshold
    alerts:        VecDeque<LatencyAlert>,
    max_alerts:     usize,
}

#[derive(Clone, Debug)]
pub struct LatencyAlert {
    pub name:       String,
    pub latency_ns:  u64,
    pub timestamp:   u64,
    pub message:     String,
}

impl LatencyTracker {
    pub fn new(threshold_ns: u64, max_alerts: usize) -> Self {
        Self {
            histograms: HashMap::new(),
            worst_case: HashMap::new(),
            threshold_ns,
            alerts: VecDeque::new(),
            max_alerts,
        }
    }

    /// Record a latency measurement
    pub fn record(&mut self, name: &str, latency_ns: u64, timestamp_ns: u64) {
        let hist = self.histograms
            .entry(name.to_string())
            .or_insert_with(|| Histogram::latency(name));
        hist.record(latency_ns);

        let worst = self.worst_case.get(name).copied().unwrap_or(0);
        if latency_ns > worst {
            self.worst_case.insert(name.to_string(), latency_ns);
        }

        if latency_ns > self.threshold_ns {
            if self.alerts.len() >= self.max_alerts {
                self.alerts.pop_front();
            }
            self.alerts.push_back(LatencyAlert {
                name: name.to_string(),
                latency_ns,
                timestamp: timestamp_ns,
                message: format!(
                    "Latency {} exceeded threshold: {} ns > {} ns",
                    name, latency_ns, self.threshold_ns
                ),
            });
        }
    }

    /// Get histogram for a name
    pub fn histogram(&self, name: &str) -> Option<&Histogram> {
        self.histograms.get(name)
    }

    /// Get worst case latency for a name
    pub fn worst_case_ns(&self, name: &str) -> u64 {
        self.worst_case.get(name).copied().unwrap_or(0)
    }

    /// Get all tracked names
    pub fn tracked_names(&self) -> Vec<String> {
        self.histograms.keys().cloned().collect()
    }

    /// Get recent alerts
    pub fn alerts(&self) -> &VecDeque<LatencyAlert> { &self.alerts }

    /// Set alert threshold
    pub fn set_threshold(&mut self, threshold_ns: u64) {
        self.threshold_ns = threshold_ns;
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.histograms.clear();
        self.worst_case.clear();
        self.alerts.clear();
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// TRACE MANAGER (top-level orchestrator)
// ══════════════════════════════════════════════════════════════════════════════

/// Top-level tracing manager
pub struct TraceManager {
    pub ring_buffer:    RingBuffer,
    pub function_tracer: FunctionTracer,
    pub syscall_tracer:  SyscallTracer,
    pub profiler:        Profiler,
    pub latency:         LatencyTracker,
    pub filter:          TraceFilter,
    pub enabled:         bool,
    // Global stats
    pub total_events:    u64,
    pub events_by_cat:   HashMap<TraceCategory, u64>,
    pub start_time:      u64,
}

impl TraceManager {
    pub fn new(ring_capacity: usize, max_stack_depth: usize) -> Self {
        Self {
            ring_buffer: RingBuffer::new(ring_capacity),
            function_tracer: FunctionTracer::new(max_stack_depth),
            syscall_tracer: SyscallTracer::new(ring_capacity),
            profiler: Profiler::new(ring_capacity, 1_000_000), // 1ms default period
            latency: LatencyTracker::new(100_000_000, 100),    // 100ms threshold
            filter: TraceFilter::new(),
            enabled: false,
            total_events: 0,
            events_by_cat: HashMap::new(),
            start_time: 0,
        }
    }

    /// Enable all tracing
    pub fn enable(&mut self) {
        self.enabled = true;
        self.ring_buffer.enable();
        self.function_tracer.enable();
        self.syscall_tracer.enable();
        self.profiler.enable();
    }

    /// Disable all tracing
    pub fn disable(&mut self) {
        self.enabled = false;
        self.ring_buffer.disable();
        self.function_tracer.disable();
        self.syscall_tracer.disable();
        self.profiler.disable();
    }

    /// Set global filter
    pub fn set_filter(&mut self, filter: TraceFilter) {
        self.filter = filter.clone();
        self.function_tracer.set_filter(filter);
    }

    /// Record a trace event
    pub fn trace(&mut self, event: TraceEvent, timestamp_ns: u64) -> bool {
        if !self.enabled { return false; }
        if !self.filter.matches(&event) { return false; }
        self.total_events += 1;
        *self.events_by_cat.entry(event.category).or_insert(0) += 1;
        self.ring_buffer.push(event, timestamp_ns)
    }

    /// Record function entry
    pub fn function_enter(&mut self, cpu: u32, pid: u32, name: &str, timestamp_ns: u64) -> bool {
        self.function_tracer.enter(cpu, pid, name, timestamp_ns, [0; 3])
    }

    /// Record function exit
    pub fn function_exit(&mut self, cpu: u32, timestamp_ns: u64) -> Option<(String, u64)> {
        let result = self.function_tracer.exit(cpu, timestamp_ns);
        if let Some((ref name, duration)) = result {
            self.latency.record(name, duration, timestamp_ns);
        }
        result
    }

    /// Record a syscall
    pub fn syscall(
        &mut self, pid: u32, cpu: u32, name: &str,
        args: [u64; 6], ret: i64, duration_ns: u64,
        timestamp_ns: u64, error: Option<String>,
    ) -> bool {
        self.syscall_tracer.record(pid, cpu, name, args, ret, duration_ns, timestamp_ns, error)
    }

    /// Take a profiling sample
    pub fn sample(
        &mut self, cpu: u32, pid: u32, pc: u64,
        stack: Vec<u64>, label: &str,
        in_kernel: bool, timestamp_ns: u64,
    ) -> bool {
        self.profiler.sample(cpu, pid, pc, stack, label, in_kernel, timestamp_ns)
    }

    /// Get event count by category
    pub fn events_in_category(&self, cat: TraceCategory) -> u64 {
        self.events_by_cat.get(&cat).copied().unwrap_or(0)
    }

    /// Generate comprehensive trace report
    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══════════════════════════════════════════════════\n");
        s.push_str("  KERNEL TRACE REPORT\n");
        s.push_str("═══════════════════════════════════════════════════\n\n");

        // Ring buffer stats
        s.push_str(&format!(
            "Ring Buffer: {}/{} events ({:.1}% full)\n  Total recorded: {}  Dropped: {}\n\n",
            self.ring_buffer.len(), self.ring_buffer.capacity(),
            self.ring_buffer.utilization() * 100.0,
            self.ring_buffer.total_count(), self.ring_buffer.dropped_count()
        ));

        // Events by category
        s.push_str("Events by Category:\n");
        for cat in [TraceCategory::Function, TraceCategory::Syscall, TraceCategory::Sched,
                     TraceCategory::Mem, TraceCategory::Net, TraceCategory::Block,
                     TraceCategory::Signal, TraceCategory::Container, TraceCategory::Irq] {
            let count = self.events_in_category(cat);
            if count > 0 {
                s.push_str(&format!("  {:12s}: {}\n", cat.as_str(), count));
            }
        }
        s.push_str(&format!("  {:12s}: {}\n", "total", self.total_events));

        // Function tracer stats
        if !self.function_tracer.call_counts.is_empty() {
            s.push_str("\nTop 10 Functions by Time:\n");
            for (name, time_ns) in self.function_tracer.top_by_time(10) {
                let count = self.function_tracer.call_count(&name);
                let avg = if count > 0 { time_ns / count } else { 0 };
                s.push_str(&format!(
                    "  {:30s} calls={:<6} total={:>10} ns avg={:>8} ns\n",
                    name, count, time_ns, avg
                ));
            }
        }

        // Syscall summary
        if !self.syscall_tracer.counts.is_empty() {
            s.push_str(&format!("\n{}", self.syscall_tracer.summary()));
        }

        // Profiler report
        if self.profiler.total_samples() > 0 {
            s.push_str(&format!("\n{}", self.profiler.report()));
        }

        // Latency alerts
        if !self.latency.alerts.is_empty() {
            s.push_str(&format!("\nLatency Alerts ({}):\n", self.latency.alerts.len()));
            for alert in self.latency.alerts.iter().rev().take(10) {
                s.push_str(&format!("  [{}s] {} = {} ns\n",
                    alert.timestamp / 1_000_000_000, alert.name, alert.latency_ns));
            }
        }

        s
    }

    /// Clear all tracing data
    pub fn clear_all(&mut self) {
        self.ring_buffer.clear();
        self.function_tracer.reset_stats();
        self.syscall_tracer.clear();
        self.profiler.clear();
        self.latency.clear();
        self.total_events = 0;
        self.events_by_cat.clear();
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// TESTS
// ══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── Ring Buffer Tests ──

    #[test]
    fn test_ring_buffer_basic() {
        let mut rb = RingBuffer::new(100);
        assert!(rb.is_empty());
        assert_eq!(rb.capacity(), 100);

        let event = TraceEvent::custom(0, 1, "test_event", 1, 2, 3);
        assert!(rb.push(event, 1000));
        assert_eq!(rb.len(), 1);
        assert!(!rb.is_empty());
        assert_eq!(rb.total_count(), 1);
        assert_eq!(rb.dropped_count(), 0);
    }

    #[test]
    fn test_ring_buffer_overflow() {
        let mut rb = RingBuffer::new(5);
        for i in 0..10u64 {
            let event = TraceEvent::custom(0, 1, "evt", i, 0, 0);
            rb.push(event, i * 1000);
        }
        assert_eq!(rb.len(), 5);       // Capped at capacity
        assert_eq!(rb.total_count(), 10);
        assert_eq!(rb.dropped_count(), 5);
    }

    #[test]
    fn test_ring_buffer_disable() {
        let mut rb = RingBuffer::new(100);
        rb.disable();
        assert!(!rb.is_enabled());
        let event = TraceEvent::custom(0, 1, "test", 0, 0, 0);
        assert!(!rb.push(event, 1000));
        assert_eq!(rb.len(), 0);
        assert_eq!(rb.total_count(), 0);

        rb.enable();
        assert!(rb.is_enabled());
        let event2 = TraceEvent::custom(0, 1, "test2", 0, 0, 0);
        assert!(rb.push(event2, 2000));
        assert_eq!(rb.len(), 1);
    }

    #[test]
    fn test_ring_buffer_filter_by_category() {
        let mut rb = RingBuffer::new(100);
        rb.push(TraceEvent::custom(0, 1, "c1", 0, 0, 0), 100);
        rb.push(TraceEvent::syscall(0, 1, "read", 0), 200);
        rb.push(TraceEvent::sched_event(0, 1, "switch", 2), 300);
        rb.push(TraceEvent::custom(0, 2, "c2", 0, 0, 0), 400);

        assert_eq!(rb.events_by_category(TraceCategory::Custom).len(), 2);
        assert_eq!(rb.events_by_category(TraceCategory::Syscall).len(), 1);
        assert_eq!(rb.events_by_category(TraceCategory::Sched).len(), 1);
    }

    #[test]
    fn test_ring_buffer_filter_by_pid() {
        let mut rb = RingBuffer::new(100);
        rb.push(TraceEvent::custom(0, 1, "a", 0, 0, 0), 100);
        rb.push(TraceEvent::custom(0, 2, "b", 0, 0, 0), 200);
        rb.push(TraceEvent::custom(0, 1, "c", 0, 0, 0), 300);

        assert_eq!(rb.events_by_pid(1).len(), 2);
        assert_eq!(rb.events_by_pid(2).len(), 1);
        assert_eq!(rb.events_by_pid(3).len(), 0);
    }

    #[test]
    fn test_ring_buffer_filter_by_cpu() {
        let mut rb = RingBuffer::new(100);
        rb.push(TraceEvent::custom(0, 1, "a", 0, 0, 0), 100);
        rb.push(TraceEvent::custom(1, 1, "b", 0, 0, 0), 200);
        rb.push(TraceEvent::custom(2, 1, "c", 0, 0, 0), 300);

        assert_eq!(rb.events_by_cpu(0).len(), 1);
        assert_eq!(rb.events_by_cpu(1).len(), 1);
        assert_eq!(rb.events_by_cpu(2).len(), 1);
    }

    #[test]
    fn test_ring_buffer_time_range() {
        let mut rb = RingBuffer::new(100);
        rb.push(TraceEvent::custom(0, 1, "a", 0, 0, 0), 1000);
        rb.push(TraceEvent::custom(0, 1, "b", 0, 0, 0), 2000);
        rb.push(TraceEvent::custom(0, 1, "c", 0, 0, 0), 3000);
        rb.push(TraceEvent::custom(0, 1, "d", 0, 0, 0), 4000);

        assert_eq!(rb.events_in_range(1500, 3500).len(), 2);
        assert_eq!(rb.events_in_range(0, 10000).len(), 4);
        assert_eq!(rb.events_in_range(5000, 6000).len(), 0);
    }

    #[test]
    fn test_ring_buffer_utilization() {
        let mut rb = RingBuffer::new(10);
        assert_eq!(rb.utilization(), 0.0);

        for i in 0..5u64 {
            rb.push(TraceEvent::custom(0, 1, "e", i, 0, 0), i * 100);
        }
        assert!((rb.utilization() - 0.5).abs() < 0.01);

        for i in 5..10u64 {
            rb.push(TraceEvent::custom(0, 1, "e", i, 0, 0), i * 100);
        }
        assert!((rb.utilization() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ring_buffer_clear() {
        let mut rb = RingBuffer::new(100);
        for i in 0..10u64 {
            rb.push(TraceEvent::custom(0, 1, "e", i, 0, 0), i * 100);
        }
        assert_eq!(rb.len(), 10);
        rb.clear();
        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
    }

    // ── Trace Filter Tests ──

    #[test]
    fn test_trace_filter_empty_matches_all() {
        let filter = TraceFilter::new();
        let event = TraceEvent::custom(0, 1, "test", 0, 0, 0);
        assert!(filter.matches(&event));
    }

    #[test]
    fn test_trace_filter_by_category() {
        let filter = TraceFilter::new().category(TraceCategory::Syscall);
        assert!(filter.matches(&TraceEvent::syscall(0, 1, "read", 0)));
        assert!(!filter.matches(&TraceEvent::custom(0, 1, "x", 0, 0, 0)));
    }

    #[test]
    fn test_trace_filter_by_pid() {
        let filter = TraceFilter::new().pid(42);
        assert!(filter.matches(&TraceEvent::custom(0, 42, "x", 0, 0, 0)));
        assert!(!filter.matches(&TraceEvent::custom(0, 1, "x", 0, 0, 0)));
    }

    #[test]
    fn test_trace_filter_by_cpu() {
        let filter = TraceFilter::new().cpu(2);
        assert!(filter.matches(&TraceEvent::custom(2, 1, "x", 0, 0, 0)));
        assert!(!filter.matches(&TraceEvent::custom(0, 1, "x", 0, 0, 0)));
    }

    #[test]
    fn test_trace_filter_by_name() {
        let filter = TraceFilter::new().name_contains("sched");
        assert!(filter.matches(&TraceEvent::sched_event(0, 1, "sched_switch", 2)));
        assert!(!filter.matches(&TraceEvent::custom(0, 1, "other", 0, 0, 0)));
    }

    #[test]
    fn test_trace_filter_by_min_duration() {
        let filter = TraceFilter::new().min_duration(1000);
        assert!(filter.matches(&TraceEvent::function_exit(0, 1, "fn", 5000)));
        assert!(!filter.matches(&TraceEvent::function_exit(0, 1, "fn", 500)));
        assert!(!filter.matches(&TraceEvent::function_entry(0, 1, "fn")));
    }

    #[test]
    fn test_trace_filter_combined() {
        let filter = TraceFilter::new()
            .category(TraceCategory::Syscall)
            .pid(1)
            .name_contains("read");
        assert!(filter.matches(&TraceEvent::syscall(0, 1, "read", 0)));
        assert!(!filter.matches(&TraceEvent::syscall(0, 2, "read", 0)));
        assert!(!filter.matches(&TraceEvent::syscall(0, 1, "write", 0)));
    }

    // ── Histogram Tests ──

    #[test]
    fn test_histogram_basic() {
        let mut h = Histogram::new("test", vec![10, 100, 1000]);
        h.record(5);
        h.record(50);
        h.record(500);
        h.record(5000);

        assert_eq!(h.count, 4);
        assert_eq!(h.min_value, 5);
        assert_eq!(h.max_value, 5000);
        assert_eq!(h.sum, 5555);
    }

    #[test]
    fn test_histogram_mean() {
        let mut h = Histogram::new("test", vec![10, 100, 1000]);
        h.record(100);
        h.record(200);
        h.record(300);
        assert!((h.mean() - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_histogram_percentiles() {
        let mut h = Histogram::latency("lat");
        for i in 1..=100u64 {
            h.record(i * 1000);  // 1us to 100us
        }
        // P50 should be around 50us
        assert!(h.p50() >= 10_000);
        // P99 should be near max
        assert!(h.p99() >= 100_000);
    }

    #[test]
    fn test_histogram_reset() {
        let mut h = Histogram::new("test", vec![10, 100]);
        h.record(5);
        h.record(50);
        assert_eq!(h.count, 2);
        h.reset();
        assert_eq!(h.count, 0);
        assert_eq!(h.sum, 0);
    }

    #[test]
    fn test_histogram_report() {
        let mut h = Histogram::latency("test_lat");
        for i in 1..=10u64 {
            h.record(i * 1000);
        }
        let report = h.report();
        assert!(report.contains("test_lat"));
        assert!(report.contains("Count: 10"));
    }

    #[test]
    fn test_histogram_buckets() {
        let mut h = Histogram::new("test", vec![10, 100, 1000, 10000]);
        h.record(5);     // -> bucket 10
        h.record(50);    // -> bucket 100
        h.record(500);   // -> bucket 1000
        h.record(5000);  // -> bucket 10000
        h.record(50000); // -> bucket MAX

        assert_eq!(h.buckets.len(), 5);
        assert_eq!(h.buckets[&10], 1);
        assert_eq!(h.buckets[&100], 1);
        assert_eq!(h.buckets[&1000], 1);
        assert_eq!(h.buckets[&10000], 1);
    }

    // ── Function Tracer Tests ──

    #[test]
    fn test_function_tracer_enter_exit() {
        let mut ft = FunctionTracer::new(32);
        ft.enable();

        assert!(ft.enter(0, 1, "func_a", 1000, [0; 3]));
        assert_eq!(ft.depth(0), 1);
        assert!(ft.enter(0, 1, "func_b", 2000, [0; 3]));
        assert_eq!(ft.depth(0), 2);

        let exit_result = ft.exit(0, 3000);
        assert!(exit_result.is_some());
        let (name, dur) = exit_result.unwrap();
        assert_eq!(name, "func_b");
        assert_eq!(dur, 1000);

        let exit_result2 = ft.exit(0, 4000);
        assert!(exit_result2.is_some());
        let (name2, dur2) = exit_result2.unwrap();
        assert_eq!(name2, "func_a");
        assert_eq!(dur2, 3000);

        assert_eq!(ft.depth(0), 0);
    }

    #[test]
    fn test_function_tracer_disabled() {
        let mut ft = FunctionTracer::new(32);
        // Not enabled
        assert!(!ft.enter(0, 1, "func", 1000, [0; 3]));
        assert_eq!(ft.depth(0), 0);
        assert!(ft.exit(0, 2000).is_none());
    }

    #[test]
    fn test_function_tracer_max_depth() {
        let mut ft = FunctionTracer::new(3);
        ft.enable();

        assert!(ft.enter(0, 1, "f1", 1000, [0; 3]));
        assert!(ft.enter(0, 1, "f2", 2000, [0; 3]));
        assert!(ft.enter(0, 1, "f3", 3000, [0; 3]));
        assert!(!ft.enter(0, 1, "f4", 4000, [0; 3])); // Max depth
        assert_eq!(ft.depth(0), 3);
    }

    #[test]
    fn test_function_tracer_call_counts() {
        let mut ft = FunctionTracer::new(32);
        ft.enable();

        ft.enter(0, 1, "func_a", 100, [0; 3]);
        ft.exit(0, 200);
        ft.enter(0, 1, "func_a", 300, [0; 3]);
        ft.exit(0, 400);
        ft.enter(0, 1, "func_b", 500, [0; 3]);
        ft.exit(0, 600);

        assert_eq!(ft.call_count("func_a"), 2);
        assert_eq!(ft.call_count("func_b"), 1);
        assert_eq!(ft.call_count("func_c"), 0);
    }

    #[test]
    fn test_function_tracer_total_time() {
        let mut ft = FunctionTracer::new(32);
        ft.enable();

        ft.enter(0, 1, "func", 100, [0; 3]);
        ft.exit(0, 300);  // 200 ns
        ft.enter(0, 1, "func", 400, [0; 3]);
        ft.exit(0, 700);  // 300 ns

        assert_eq!(ft.total_time_ns("func"), 500);
        assert!((ft.avg_time_ns("func") - 250.0).abs() < 0.1);
    }

    #[test]
    fn test_function_tracer_top_by_time() {
        let mut ft = FunctionTracer::new(32);
        ft.enable();

        ft.enter(0, 1, "fast", 100, [0; 3]);
        ft.exit(0, 110);   // 10 ns
        ft.enter(0, 1, "slow", 200, [0; 3]);
        ft.exit(0, 5000);  // 4800 ns
        ft.enter(0, 1, "mid", 6000, [0; 3]);
        ft.exit(0, 7000);  // 1000 ns

        let top = ft.top_by_time(3);
        assert_eq!(top[0].0, "slow");
        assert_eq!(top[0].1, 4800);
        assert_eq!(top[1].0, "mid");
        assert_eq!(top[1].1, 1000);
        assert_eq!(top[2].0, "fast");
        assert_eq!(top[2].1, 10);
    }

    #[test]
    fn test_function_tracer_top_by_count() {
        let mut ft = FunctionTracer::new(32);
        ft.enable();

        for _ in 0..5 { ft.enter(0, 1, "frequent", 100, [0; 3]); ft.exit(0, 110); }
        for _ in 0..2 { ft.enter(0, 1, "rare", 100, [0; 3]); ft.exit(0, 110); }

        let top = ft.top_by_count(2);
        assert_eq!(top[0].0, "frequent");
        assert_eq!(top[0].1, 5);
        assert_eq!(top[1].0, "rare");
        assert_eq!(top[1].1, 2);
    }

    #[test]
    fn test_function_tracer_multi_cpu() {
        let mut ft = FunctionTracer::new(32);
        ft.enable();

        ft.enter(0, 1, "cpu0_func", 100, [0; 3]);
        ft.enter(1, 2, "cpu1_func", 200, [0; 3]);

        assert_eq!(ft.depth(0), 1);
        assert_eq!(ft.depth(1), 1);

        ft.exit(0, 300);
        ft.exit(1, 400);

        assert_eq!(ft.depth(0), 0);
        assert_eq!(ft.depth(1), 0);
    }

    #[test]
    fn test_function_tracer_reset_stats() {
        let mut ft = FunctionTracer::new(32);
        ft.enable();

        ft.enter(0, 1, "f", 100, [0; 3]);
        ft.exit(0, 200);
        assert_eq!(ft.call_count("f"), 1);

        ft.reset_stats();
        assert_eq!(ft.call_count("f"), 0);
    }

    #[test]
    fn test_function_tracer_max_depth_reached() {
        let mut ft = FunctionTracer::new(5);
        ft.enable();

        ft.enter(0, 1, "a", 100, [0; 3]);
        ft.enter(0, 1, "b", 200, [0; 3]);
        ft.enter(0, 1, "c", 300, [0; 3]);
        assert_eq!(ft.max_depth_reached(), 3);

        ft.exit(0, 400);
        ft.exit(0, 500);
        ft.exit(0, 600);
        // max_depth_reached stays at 3
        assert_eq!(ft.max_depth_reached(), 3);
    }

    // ── Syscall Tracer Tests ──

    #[test]
    fn test_syscall_tracer_basic() {
        let mut st = SyscallTracer::new(100);
        st.enable();

        let recorded = st.record(1, 0, "read", [0x1000, 0x2000, 0, 0, 0, 0], 100, 500, 1000, None);
        assert!(recorded);
        assert_eq!(st.count("read"), 1);
        assert_eq!(st.total_time_ns("read"), 500);
        assert!((st.avg_duration_ns("read") - 500.0).abs() < 0.1);
    }

    #[test]
    fn test_syscall_tracer_disabled() {
        let mut st = SyscallTracer::new(100);
        let recorded = st.record(1, 0, "read", [0; 6], 0, 100, 1000, None);
        assert!(!recorded);
        assert_eq!(st.count("read"), 0);
    }

    #[test]
    fn test_syscall_tracer_pid_filter() {
        let mut st = SyscallTracer::new(100);
        st.enable();
        st.trace_pid(42);

        assert!(st.record(42, 0, "read", [0; 6], 0, 100, 1000, None));
        assert!(!st.record(1, 0, "read", [0; 6], 0, 100, 1000, None));
        assert_eq!(st.count("read"), 1);
    }

    #[test]
    fn test_syscall_tracer_syscall_filter() {
        let mut st = SyscallTracer::new(100);
        st.enable();
        st.trace_syscall("read");

        assert!(st.record(1, 0, "read", [0; 6], 0, 100, 1000, None));
        assert!(!st.record(1, 0, "write", [0; 6], 0, 100, 1000, None));
        assert_eq!(st.count("read"), 1);
        assert_eq!(st.count("write"), 0);
    }

    #[test]
    fn test_syscall_tracer_errors() {
        let mut st = SyscallTracer::new(100);
        st.enable();

        st.record(1, 0, "open", [0; 6], -1, 100, 1000, Some("ENOENT".to_string()));
        st.record(1, 0, "open", [0; 6], 0, 200, 2000, None);

        assert_eq!(st.count("open"), 2);
        assert_eq!(st.error_count("open"), 1);
        assert!((st.error_rate("open") - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_syscall_tracer_overflow() {
        let mut st = SyscallTracer::new(5);
        st.enable();

        for i in 0..10u64 {
            st.record(1, 0, "read", [i, 0, 0, 0, 0, 0], 0, 100, i * 1000, None);
        }
        assert_eq!(st.records().len(), 5);
        assert_eq!(st.count("read"), 10);
    }

    #[test]
    fn test_syscall_tracer_records_for_pid() {
        let mut st = SyscallTracer::new(100);
        st.enable();

        st.record(1, 0, "read", [0; 6], 0, 100, 1000, None);
        st.record(2, 0, "read", [0; 6], 0, 100, 2000, None);
        st.record(1, 0, "write", [0; 6], 0, 100, 3000, None);

        assert_eq!(st.records_for_pid(1).len(), 2);
        assert_eq!(st.records_for_pid(2).len(), 1);
    }

    #[test]
    fn test_syscall_tracer_records_for_syscall() {
        let mut st = SyscallTracer::new(100);
        st.enable();

        st.record(1, 0, "read", [0; 6], 0, 100, 1000, None);
        st.record(1, 0, "write", [0; 6], 0, 100, 2000, None);
        st.record(1, 0, "read", [0; 6], 0, 100, 3000, None);

        assert_eq!(st.records_for_syscall("read").len(), 2);
        assert_eq!(st.records_for_syscall("write").len(), 1);
    }

    #[test]
    fn test_syscall_tracer_summary() {
        let mut st = SyscallTracer::new(100);
        st.enable();

        st.record(1, 0, "read", [0; 6], 0, 500, 1000, None);
        st.record(1, 0, "write", [0; 6], -1, 1000, 2000, Some("EBUSY".to_string()));

        let summary = st.summary();
        assert!(summary.contains("read"));
        assert!(summary.contains("write"));
    }

    #[test]
    fn test_syscall_tracer_clear() {
        let mut st = SyscallTracer::new(100);
        st.enable();

        st.record(1, 0, "read", [0; 6], 0, 100, 1000, None);
        assert_eq!(st.count("read"), 1);

        st.clear();
        assert_eq!(st.count("read"), 0);
        assert_eq!(st.records().len(), 0);
    }

    #[test]
    fn test_syscall_tracer_trace_all() {
        let mut st = SyscallTracer::new(100);
        st.enable();
        st.trace_pid(1);
        st.trace_syscall("read");

        // Clear filters
        st.trace_all_pids();
        st.trace_all_syscalls();

        assert!(st.record(2, 0, "write", [0; 6], 0, 100, 1000, None));
        assert!(st.record(3, 0, "open", [0; 6], 0, 100, 2000, None));
    }

    // ── Profiler Tests ──

    #[test]
    fn test_profiler_basic() {
        let mut p = Profiler::new(100, 1000);  // 1us period
        p.enable();

        assert!(p.sample(0, 1, 0x1000, vec![0x1000], "main", true, 1000));
        assert_eq!(p.total_samples(), 1);
    }

    #[test]
    fn test_profiler_disabled() {
        let mut p = Profiler::new(100, 1000);
        assert!(!p.sample(0, 1, 0x1000, vec![], "main", true, 1000));
        assert_eq!(p.total_samples(), 0);
    }

    #[test]
    fn test_profiler_sample_period() {
        let mut p = Profiler::new(100, 1000);  // 1us period
        p.enable();

        assert!(p.sample(0, 1, 0x1000, vec![], "a", true, 1000));
        assert!(!p.sample(0, 1, 0x2000, vec![], "b", true, 1500));  // Too soon
        assert!(p.sample(0, 1, 0x2000, vec![], "b", true, 2000));    // 1us later
    }

    #[test]
    fn test_profiler_pid_filter() {
        let mut p = Profiler::new(100, 1);
        p.enable();
        p.trace_pid(42);

        assert!(p.sample(0, 42, 0x1000, vec![], "a", true, 1000));
        assert!(!p.sample(0, 1, 0x2000, vec![], "b", true, 2000));
    }

    #[test]
    fn test_profiler_kernel_only() {
        let mut p = Profiler::new(100, 1);
        p.enable();
        p.kernel_only();

        assert!(p.sample(0, 1, 0x1000, vec![], "k_func", true, 1000));
        assert!(!p.sample(0, 1, 0x2000, vec![], "u_func", false, 2000));
    }

    #[test]
    fn test_profiler_hot_functions() {
        let mut p = Profiler::new(100, 1);
        p.enable();

        for _ in 0..10 { p.sample(0, 1, 0x1000, vec![], "hot_func", true, 0); p.last_sample = 0; }
        for _ in 0..2 { p.sample(0, 1, 0x2000, vec![], "cold_func", true, 0); p.last_sample = 0; }

        let hot = p.hot_functions(2);
        assert_eq!(hot[0].0, "hot_func");
        assert_eq!(hot[0].1, 10);
        assert_eq!(hot[1].0, "cold_func");
        assert_eq!(hot[1].1, 2);
    }

    #[test]
    fn test_profiler_hot_pids() {
        let mut p = Profiler::new(100, 1);
        p.enable();

        p.sample(0, 1, 0x1000, vec![], "a", true, 0); p.last_sample = 0;
        p.sample(0, 2, 0x2000, vec![], "b", true, 0); p.last_sample = 0;
        p.sample(0, 1, 0x3000, vec![], "c", true, 0); p.last_sample = 0;

        let hot = p.hot_pids(2);
        assert_eq!(hot[0].0, 1);
        assert_eq!(hot[0].1, 2);
    }

    #[test]
    fn test_profiler_kernel_ratio() {
        let mut p = Profiler::new(100, 1);
        p.enable();

        for _ in 0..7 { p.sample(0, 1, 0, vec![], "k", true, 0); p.last_sample = 0; }
        for _ in 0..3 { p.sample(0, 1, 0, vec![], "u", false, 0); p.last_sample = 0; }

        assert!((p.kernel_ratio() - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_profiler_per_cpu() {
        let mut p = Profiler::new(100, 1);
        p.enable();

        p.sample(0, 1, 0, vec![], "a", true, 0); p.last_sample = 0;
        p.sample(1, 1, 0, vec![], "b", true, 0); p.last_sample = 0;
        p.sample(0, 1, 0, vec![], "c", true, 0); p.last_sample = 0;

        assert_eq!(p.samples_per_cpu(0), 2);
        assert_eq!(p.samples_per_cpu(1), 1);
    }

    #[test]
    fn test_profiler_report() {
        let mut p = Profiler::new(100, 1);
        p.enable();

        p.sample(0, 1, 0, vec![], "main", true, 0);
        p.sample(0, 1, 0, vec![], "helper", false, 1); p.last_sample = 0;

        let report = p.report();
        assert!(report.contains("Profile Report"));
        assert!(report.contains("main"));
    }

    #[test]
    fn test_profiler_clear() {
        let mut p = Profiler::new(100, 1);
        p.enable();

        p.sample(0, 1, 0, vec![], "a", true, 0);
        assert_eq!(p.total_samples(), 1);

        p.clear();
        assert_eq!(p.total_samples(), 0);
    }

    #[test]
    fn test_profiler_overflow() {
        let mut p = Profiler::new(5, 1);
        p.enable();

        for i in 0..10u64 {
            p.sample(0, 1, i, vec![], "func", true, i);
        }
        assert_eq!(p.total_samples(), 10);
        // Buffer holds only 5, but count tracks all
    }

    // ── Latency Tracker Tests ──

    #[test]
    fn test_latency_tracker_basic() {
        let mut lt = LatencyTracker::new(1_000_000, 100);  // 1ms threshold
        lt.record("sched_switch", 500_000, 1000);
        lt.record("sched_switch", 1_500_000, 2000);

        let hist = lt.histogram("sched_switch").unwrap();
        assert_eq!(hist.count, 2);
        assert_eq!(lt.worst_case_ns("sched_switch"), 1_500_000);
    }

    #[test]
    fn test_latency_tracker_alerts() {
        let mut lt = LatencyTracker::new(1_000_000, 100);
        lt.record("irq_handler", 500_000, 1000);   // Below threshold
        lt.record("irq_handler", 2_000_000, 2000); // Above threshold

        assert_eq!(lt.alerts().len(), 1);
        assert_eq!(lt.alerts()[0].name, "irq_handler");
        assert_eq!(lt.alerts()[0].latency_ns, 2_000_000);
    }

    #[test]
    fn test_latency_tracker_alert_overflow() {
        let mut lt = LatencyTracker::new(0, 3);  // 0 threshold = all alert, max 3
        for i in 0..5u64 {
            lt.record("test", i * 1000, i * 1000);
        }
        assert_eq!(lt.alerts().len(), 3);  // Capped at max_alerts
    }

    #[test]
    fn test_latency_tracker_tracked_names() {
        let mut lt = LatencyTracker::new(1_000_000, 100);
        lt.record("a", 100, 0);
        lt.record("b", 200, 0);
        lt.record("c", 300, 0);

        let names = lt.tracked_names();
        assert!(names.contains(&"a".to_string()));
        assert!(names.contains(&"b".to_string()));
        assert!(names.contains(&"c".to_string()));
    }

    #[test]
    fn test_latency_tracker_set_threshold() {
        let mut lt = LatencyTracker::new(1_000_000, 100);
        lt.record("x", 500_000, 0);
        assert_eq!(lt.alerts().len(), 0);

        lt.set_threshold(100_000);  // Lower to 100us
        lt.record("x", 500_000, 0);
        assert_eq!(lt.alerts().len(), 1);
    }

    #[test]
    fn test_latency_tracker_clear() {
        let mut lt = LatencyTracker::new(1_000_000, 100);
        lt.record("a", 100, 0);
        lt.record("a", 2_000_000, 0);
        assert!(!lt.alerts().is_empty());

        lt.clear();
        assert!(lt.alerts().is_empty());
        assert!(lt.tracked_names().is_empty());
    }

    // ── Trace Manager Tests ──

    #[test]
    fn test_trace_manager_enable_disable() {
        let mut tm = TraceManager::new(100, 32);
        assert!(!tm.enabled);
        assert!(!tm.ring_buffer.is_enabled());

        tm.enable();
        assert!(tm.enabled);
        assert!(tm.ring_buffer.is_enabled());
        assert!(tm.function_tracer.enabled);
        assert!(tm.syscall_tracer.enabled);
        assert!(tm.profiler.enabled);

        tm.disable();
        assert!(!tm.enabled);
        assert!(!tm.ring_buffer.is_enabled());
    }

    #[test]
    fn test_trace_manager_trace_event() {
        let mut tm = TraceManager::new(100, 32);
        tm.enable();

        let event = TraceEvent::custom(0, 1, "test", 42, 0, 0);
        assert!(tm.trace(event, 1000));
        assert_eq!(tm.total_events, 1);
        assert_eq!(tm.events_in_category(TraceCategory::Custom), 1);
    }

    #[test]
    fn test_trace_manager_trace_disabled() {
        let mut tm = TraceManager::new(100, 32);
        let event = TraceEvent::custom(0, 1, "test", 0, 0, 0);
        assert!(!tm.trace(event, 1000));
        assert_eq!(tm.total_events, 0);
    }

    #[test]
    fn test_trace_manager_filter() {
        let mut tm = TraceManager::new(100, 32);
        tm.enable();

        let filter = TraceFilter::new().category(TraceCategory::Syscall);
        tm.set_filter(filter);

        // Syscall event passes
        assert!(tm.trace(TraceEvent::syscall(0, 1, "read", 0), 1000));
        // Custom event is filtered out
        assert!(!tm.trace(TraceEvent::custom(0, 1, "x", 0, 0, 0), 2000));

        assert_eq!(tm.total_events, 1);
        assert_eq!(tm.events_in_category(TraceCategory::Syscall), 1);
    }

    #[test]
    fn test_trace_manager_function_enter_exit() {
        let mut tm = TraceManager::new(100, 32);
        tm.enable();

        assert!(tm.function_enter(0, 1, "func_a", 1000));
        let exit = tm.function_exit(0, 3000);
        assert!(exit.is_some());
        let (name, dur) = exit.unwrap();
        assert_eq!(name, "func_a");
        assert_eq!(dur, 2000);
    }

    #[test]
    fn test_trace_manager_syscall_tracing() {
        let mut tm = TraceManager::new(100, 32);
        tm.enable();

        assert!(tm.syscall(1, 0, "read", [0; 6], 100, 500, 1000, None));
        assert_eq!(tm.syscall_tracer.count("read"), 1);
    }

    #[test]
    fn test_trace_manager_profiling() {
        let mut tm = TraceManager::new(100, 32);
        tm.enable();

        assert!(tm.sample(0, 1, 0x1000, vec![0x1000], "main", true, 1000));
        assert_eq!(tm.profiler.total_samples(), 1);
    }

    #[test]
    fn test_trace_manager_latency_tracking() {
        let mut tm = TraceManager::new(100, 32);
        tm.enable();

        tm.function_enter(0, 1, "slow_func", 1000);
        tm.function_exit(0, 5_000_000);  // 5ms

        let hist = tm.latency.histogram("slow_func");
        assert!(hist.is_some());
        assert_eq!(hist.unwrap().count, 1);
    }

    #[test]
    fn test_trace_manager_report() {
        let mut tm = TraceManager::new(100, 32);
        tm.enable();

        tm.trace(TraceEvent::syscall(0, 1, "read", 0), 1000);
        tm.trace(TraceEvent::custom(0, 1, "event", 0, 0, 0), 2000);
        tm.function_enter(0, 1, "func", 3000);
        tm.function_exit(0, 4000);

        let report = tm.report();
        assert!(report.contains("KERNEL TRACE REPORT"));
        assert!(report.contains("Ring Buffer"));
    }

    #[test]
    fn test_trace_manager_clear_all() {
        let mut tm = TraceManager::new(100, 32);
        tm.enable();

        tm.trace(TraceEvent::custom(0, 1, "x", 0, 0, 0), 1000);
        tm.syscall(1, 0, "read", [0; 6], 0, 100, 2000, None);
        tm.sample(0, 1, 0, vec![], "f", true, 3000);

        assert!(tm.total_events > 0);

        tm.clear_all();
        assert_eq!(tm.total_events, 0);
        assert_eq!(tm.ring_buffer.len(), 0);
        assert_eq!(tm.syscall_tracer.count("read"), 0);
        assert_eq!(tm.profiler.total_samples(), 0);
    }

    #[test]
    fn test_trace_manager_events_by_category() {
        let mut tm = TraceManager::new(200, 32);
        tm.enable();

        tm.trace(TraceEvent::custom(0, 1, "a", 0, 0, 0), 100);
        tm.trace(TraceEvent::syscall(0, 1, "read", 0), 200);
        tm.trace(TraceEvent::sched_event(0, 1, "switch", 2), 300);
        tm.trace(TraceEvent::mem_event(0, 1, "alloc", 0x1000, 4096), 400);
        tm.trace(TraceEvent::custom(0, 1, "b", 0, 0, 0), 500);

        assert_eq!(tm.events_in_category(TraceCategory::Custom), 2);
        assert_eq!(tm.events_in_category(TraceCategory::Syscall), 1);
        assert_eq!(tm.events_in_category(TraceCategory::Sched), 1);
        assert_eq!(tm.events_in_category(TraceCategory::Mem), 1);
        assert_eq!(tm.events_in_category(TraceCategory::Net), 0);
    }

    // ── TraceCategory Tests ──

    #[test]
    fn test_trace_category_roundtrip() {
        let categories = [
            TraceCategory::Function, TraceCategory::Syscall, TraceCategory::Irq,
            TraceCategory::Sched, TraceCategory::Mem, TraceCategory::Net,
            TraceCategory::Block, TraceCategory::Signal, TraceCategory::Container,
            TraceCategory::User, TraceCategory::Custom,
        ];
        for cat in &categories {
            let s = cat.as_str();
            let back = TraceCategory::from_str(s);
            assert_eq!(back, Some(*cat));
        }
        assert_eq!(TraceCategory::from_str("unknown"), None);
    }

    #[test]
    fn test_trace_event_types() {
        let entry = TraceEvent::function_entry(0, 1, "func");
        assert_eq!(entry.event_type, TraceEventType::Entry);
        assert_eq!(entry.category, TraceCategory::Function);

        let exit = TraceEvent::function_exit(0, 1, "func", 1000);
        assert_eq!(exit.event_type, TraceEventType::Exit);
        assert_eq!(exit.duration_ns, Some(1000));

        let syscall = TraceEvent::syscall(0, 1, "read", 42);
        assert_eq!(syscall.category, TraceCategory::Syscall);
        assert_eq!(syscall.arg1, 42);

        let sched = TraceEvent::sched_event(0, 1, "switch", 5);
        assert_eq!(sched.category, TraceCategory::Sched);
        assert_eq!(sched.arg1, 5);

        let mem = TraceEvent::mem_event(0, 1, "alloc", 0x1000, 4096);
        assert_eq!(mem.category, TraceCategory::Mem);
        assert_eq!(mem.arg1, 0x1000);
        assert_eq!(mem.arg2, 4096);
    }

    #[test]
    fn test_trace_event_seq_unique() {
        let e1 = TraceEvent::custom(0, 1, "a", 0, 0, 0);
        let e2 = TraceEvent::custom(0, 1, "b", 0, 0, 0);
        let e3 = TraceEvent::custom(0, 1, "c", 0, 0, 0);
        assert_ne!(e1.seq, e2.seq);
        assert_ne!(e2.seq, e3.seq);
        assert_ne!(e1.seq, e3.seq);
    }
}
