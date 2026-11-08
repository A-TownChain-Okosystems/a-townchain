// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 33: User Process Scheduling + Context Switching
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// Preemptive Round-Robin Scheduler für Ring-3-Prozesse.
// Context Switch (IRET-Frame), Timer-Driven Preemption, Quantum-Based Scheduling,
// Integration von UserspaceManager + SignalManager + PageFaultHandler.

use crate::ats1000::{Pid, ExitCode};
use crate::userspace::{UserspaceManager, UserContext, UserspaceError, PrivilegeLevel};
use crate::elf_loader::SignalManager;

// ═══════════════════════════════════════════════════════════════════════════════
// IRET Frame (CPU state for ring-0 → ring-3 transition)
// ═══════════════════════════════════════════════════════════════════════════════

/// The IRET frame: what the CPU needs to restore when transitioning to ring 3.
/// On x86-64, IRET pops: SS, RSP, RFLAGS, CS, RIP (in that order from stack).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IretFrame {
    pub rip:    u64,
    pub cs:     u16,
    pub rflags:  u64,
    pub rsp:    u64,
    pub ss:     u16,
}

impl IretFrame {
    /// Build an IRET frame from a UserContext
    pub fn from_user_context(ctx: &UserContext) -> Self {
        Self {
            rip:    ctx.rip,
            cs:     ctx.cs,     // 0x1B for ring 3
            rflags: ctx.rflags,  // IF=1
            rsp:    ctx.rsp,
            ss:     ctx.ss,     // 0x23 for ring 3
        }
    }

    /// Verify the frame targets ring 3
    pub fn is_ring3(&self) -> bool {
        (self.cs & 0x03) == 3 && (self.ss & 0x03) == 3
    }

    /// Verify the frame is valid for IRET
    pub fn is_valid(&self) -> bool {
        self.is_ring3()
            && self.rsp > 0
            && self.rip > 0
            && (self.rflags & 0x200) != 0  // IF must be set
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Saved Registers (general-purpose registers saved on context switch)
// ═══════════════════════════════════════════════════════════════════════════════

/// Saved general-purpose registers for a context switch.
/// On x86-64, these are pushed/popped manually before/after IRET.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SavedRegisters {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub rsp: u64,
    pub r8:  u64, pub r9:  u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
}

impl SavedRegisters {
    pub fn from_user_context(ctx: &UserContext) -> Self {
        Self {
            rax: ctx.rax,
            rbp: ctx.rbp,
            rsp: ctx.rsp,
            ..Default::default()
        }
    }

    pub fn apply_to(&self, ctx: &mut UserContext) {
        ctx.rax = self.rax;
        ctx.rbp = self.rbp;
        ctx.rsp = self.rsp;
    }
}

/// Full saved context: registers + IRET frame
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SavedContext {
    pub regs:  SavedRegisters,
    pub iret:  IretFrame,
}

impl SavedContext {
    pub fn from_user_context(ctx: &UserContext) -> Self {
        Self {
            regs: SavedRegisters::from_user_context(ctx),
            iret: IretFrame::from_user_context(ctx),
        }
    }

    pub fn apply_to(&self, ctx: &mut UserContext) {
        self.regs.apply_to(ctx);
        ctx.rip = self.iret.rip;
        ctx.rsp = self.iret.rsp;
        ctx.rflags = self.iret.rflags;
    }

    pub fn is_ring3(&self) -> bool { self.iret.is_ring3() }
    pub fn is_valid(&self) -> bool { self.iret.is_valid() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Scheduling Quantum
// ═══════════════════════════════════════════════════════════════════════════════

/// Time slice (quantum) for a user process
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Quantum {
    /// Timer ticks remaining in this quantum
    pub ticks_remaining: u32,
    /// Total ticks per quantum (reset value)
    pub ticks_total:    u32,
}

impl Quantum {
    pub fn new(ticks: u32) -> Self {
        Self { ticks_remaining: ticks, ticks_total: ticks }
    }

    pub fn default_quantum() -> Self { Self::new(10) }

    /// Decrement the quantum. Returns true if quantum expired.
    pub fn tick(&mut self) -> bool {
        if self.ticks_remaining > 0 {
            self.ticks_remaining -= 1;
        }
        self.ticks_remaining == 0
    }

    pub fn reset(&mut self) {
        self.ticks_remaining = self.ticks_total;
    }

    pub fn is_expired(&self) -> bool { self.ticks_remaining == 0 }
    pub fn remaining(&self) -> u32 { self.ticks_remaining }
}

// ═══════════════════════════════════════════════════════════════════════════════
// User Process State (extended for scheduling)
// ═══════════════════════════════════════════════════════════════════════════════

/// Scheduling state for a user process
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SchedState {
    /// Process is ready to run
    Ready,
    /// Process is currently running
    Running,
    /// Process is blocked (waiting for I/O, signal, etc.)
    Blocked(BlockReason),
    /// Process has exited (zombie, waiting to be reaped)
    Zombie(ExitCode),
}

/// Why a process is blocked
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockReason {
    /// Waiting for I/O
    IoWait,
    /// Waiting for a signal
    SignalWait,
    /// Waiting for IPC
    IpcWait,
    /// Sleeping for a duration
    Sleep(u64),
    /// Stopped by a signal (SIGSTOP)
    Stopped,
    /// Waiting for a child to exit
    WaitChild,
}

impl SchedState {
    pub fn is_runnable(&self) -> bool {
        matches!(self, SchedState::Ready | SchedState::Running)
    }
    pub fn is_running(&self) -> bool { matches!(self, SchedState::Running) }
    pub fn is_blocked(&self) -> bool { matches!(self, SchedState::Blocked(_)) }
    pub fn is_zombie(&self) -> bool { matches!(self, SchedState::Zombie(_)) }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Scheduled Process Entry
// ═══════════════════════════════════════════════════════════════════════════════

/// A user process entry in the scheduler
#[derive(Clone, Debug)]
pub struct SchedEntry {
    pub pid:        Pid,
    pub state:      SchedState,
    pub quantum:    Quantum,
    pub priority:   u8,          // 0 = highest
    pub saved_ctx:  SavedContext,
    pub total_cpu_ticks: u64,
    pub context_switches: u64,
    pub wake_tick:   Option<u64>,  // When to wake (for Sleep)
}

impl SchedEntry {
    pub fn new(pid: Pid, ctx: &UserContext, priority: u8) -> Self {
        Self {
            pid,
            state: SchedState::Ready,
            quantum: Quantum::default_quantum(),
            priority,
            saved_ctx: SavedContext::from_user_context(ctx),
            total_cpu_ticks: 0,
            context_switches: 0,
            wake_tick: None,
        }
    }

    pub fn is_runnable(&self) -> bool { self.state.is_runnable() }

    /// Save context when being preempted
    pub fn save_context(&mut self, ctx: &UserContext) {
        self.saved_ctx = SavedContext::from_user_context(ctx);
        self.context_switches += 1;
    }

    /// Restore context when being scheduled
    pub fn restore_context(&self) -> SavedContext { self.saved_ctx }

    /// Give the process a fresh quantum
    pub fn reset_quantum(&mut self) { self.quantum.reset(); }

    /// Tick the quantum. Returns true if expired.
    pub fn tick_quantum(&mut self) -> bool {
        self.total_cpu_ticks += 1;
        self.quantum.tick()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// User Process Scheduler
// ═══════════════════════════════════════════════════════════════════════════════

/// Preemptive round-robin scheduler for user processes
pub struct UserScheduler {
    entries:       Vec<SchedEntry>,
    current:       Option<Pid>,
    timer_ticks:   u64,
    context_switches: u64,
    preemptions:   u64,
    voluntary_yields: u64,
}

impl Default for UserScheduler {
    fn default() -> Self { Self::new() }
}

impl UserScheduler {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current: None,
            timer_ticks: 0,
            context_switches: 0,
            preemptions: 0,
            voluntary_yields: 0,
        }
    }

    /// Add a user process to the scheduler
    pub fn add_process(&mut self, pid: Pid, ctx: &UserContext, priority: u8) {
        self.entries.push(SchedEntry::new(pid, ctx, priority));
    }

    /// Remove a process from the scheduler
    pub fn remove_process(&mut self, pid: Pid) -> bool {
        if self.current == Some(pid) {
            self.current = None;
        }
        self.entries.retain(|e| e.pid != pid)
    }

    /// Get the currently running process
    pub fn current_pid(&self) -> Option<Pid> { self.current }

    /// Get a scheduler entry by PID
    pub fn get_entry(&self, pid: Pid) -> Option<&SchedEntry> {
        self.entries.iter().find(|e| e.pid == pid)
    }
    pub fn get_entry_mut(&mut self, pid: Pid) -> Option<&mut SchedEntry> {
        self.entries.iter_mut().find(|e| e.pid == pid)
    }

    /// Pick the next process to run (round-robin with priority)
    fn pick_next(&self) -> Option<usize> {
        // Priority: find highest-priority (lowest number) runnable process
        // that isn't the current one
        let current_pid = self.current;

        let mut best: Option<(usize, u8)> = None;
        for (i, entry) in self.entries.iter().enumerate() {
            if !entry.is_runnable() { continue; }
            if Some(entry.pid) == current_pid { continue; } // Skip current (round-robin)
            match best {
                None => best = Some((i, entry.priority)),
                Some((_, bp)) if entry.priority < bp => best = Some((i, entry.priority)),
                _ => {}
            }
        }

        // If no other runnable process, keep current if it's still runnable
        if best.is_none() {
            if let Some(cpid) = current_pid {
                if let Some(idx) = self.entries.iter().position(|e| e.pid == cpid) {
                    if self.entries[idx].is_runnable() {
                        return Some(idx);
                    }
                }
            }
        }

        best.map(|(i, _)| i)
    }

    /// Schedule: pick next process and perform context switch.
    /// Returns the PID to run and the saved context to restore.
    pub fn schedule(&mut self, current_ctx: Option<&UserContext>) -> Option<(Pid, SavedContext)> {
        self.context_switches += 1;

        // Save current context if we have a running process
        if let Some(pid) = self.current {
            if let Some(ctx) = current_ctx {
                if let Some(entry) = self.get_entry_mut(pid) {
                    entry.save_context(ctx);
                }
            }
        }

        // Pick next process
        let next_idx = self.pick_next()?;

        // Update states
        if let Some(old_pid) = self.current {
            if let Some(entry) = self.get_entry_mut(old_pid) {
                if entry.state == SchedState::Running {
                    entry.state = SchedState::Ready;
                }
            }
        }

        let next_pid = self.entries[next_idx].pid;
        self.entries[next_idx].state = SchedState::Running;
        self.entries[next_idx].reset_quantum();
        self.current = Some(next_pid);

        let saved = self.entries[next_idx].restore_context();
        Some((next_pid, saved))
    }

    /// Timer tick handler: called on every timer interrupt.
    /// Returns Some(pid) if a context switch is needed, None if current can continue.
    pub fn timer_tick(&mut self, current_ctx: &UserContext) -> Option<(Pid, SavedContext)> {
        self.timer_ticks += 1;

        // Check wake-ups (Sleep entries)
        for entry in &mut self.entries {
            if let SchedState::Blocked(BlockReason::Sleep(wake)) = entry.state {
                if self.timer_ticks >= *wake {
                    entry.state = SchedState::Ready;
                    entry.wake_tick = None;
                }
            }
        }

        // Tick the current process's quantum
        if let Some(pid) = self.current {
            if let Some(entry) = self.get_entry_mut(pid) {
                let expired = entry.tick_quantum();
                if expired {
                    // Preempt: schedule next
                    self.preemptions += 1;
                    return self.schedule(Some(current_ctx));
                }
            }
        } else {
            // No current process, schedule one
            return self.schedule(None);
        }

        None
    }

    /// Voluntarily yield (sys_yield)
    pub fn yield_now(&mut self, current_ctx: &UserContext) -> Option<(Pid, SavedContext)> {
        self.voluntary_yields += 1;
        self.schedule(Some(current_ctx))
    }

    /// Block the current process
    pub fn block_current(&mut self, reason: BlockReason, current_ctx: &UserContext) -> Option<(Pid, SavedContext)> {
        if let Some(pid) = self.current {
            if let Some(entry) = self.get_entry_mut(pid) {
                entry.save_context(current_ctx);
                entry.state = SchedState::Blocked(reason);
            }
        }
        self.current = None;
        self.schedule(None)
    }

    /// Unblock a process
    pub fn unblock(&mut self, pid: Pid) {
        if let Some(entry) = self.get_entry_mut(pid) {
            entry.state = SchedState::Ready;
        }
    }

    /// Mark a process as exited (zombie)
    pub fn exit_process(&mut self, pid: Pid, exit_code: ExitCode, current_ctx: Option<&UserContext>) -> Option<(Pid, SavedContext)> {
        if let Some(entry) = self.get_entry_mut(pid) {
            if let Some(ctx) = current_ctx {
                entry.save_context(ctx);
            }
            entry.state = SchedState::Zombie(exit_code);
        }
        if self.current == Some(pid) {
            self.current = None;
        }
        self.schedule(None)
    }

    /// Reap zombie processes
    pub fn reap_zombies(&mut self) -> Vec<(Pid, ExitCode)> {
        let zombies: Vec<(Pid, ExitCode)> = self.entries.iter()
            .filter(|e| e.is_runnable() == false && e.state.is_zombie())
            .map(|e| (e.pid, match e.state {
                SchedState::Zombie(c) => c,
                _ => 0,
            }))
            .collect();
        self.entries.retain(|e| !e.state.is_zombie());
        zombies
    }

    /// Put a process to sleep for N ticks
    pub fn sleep(&mut self, pid: Pid, ticks: u64, current_ctx: &UserContext) -> Option<(Pid, SavedContext)> {
        let wake_at = self.timer_ticks + ticks;
        if let Some(entry) = self.get_entry_mut(pid) {
            entry.wake_tick = Some(wake_at);
            entry.state = SchedState::Blocked(BlockReason::Sleep(wake_at));
        }
        if self.current == Some(pid) {
            self.current = None;
        }
        self.schedule(Some(current_ctx))
    }

    /// Stop a process (SIGSTOP)
    pub fn stop_process(&mut self, pid: Pid, current_ctx: Option<&UserContext>) -> Option<(Pid, SavedContext)> {
        if let Some(entry) = self.get_entry_mut(pid) {
            if let Some(ctx) = current_ctx {
                entry.save_context(ctx);
            }
            entry.state = SchedState::Blocked(BlockReason::Stopped);
        }
        if self.current == Some(pid) {
            self.current = None;
        }
        self.schedule(None)
    }

    /// Continue a stopped process (SIGCONT)
    pub fn continue_process(&mut self, pid: Pid) {
        if let Some(entry) = self.get_entry_mut(pid) {
            if matches!(entry.state, SchedState::Blocked(BlockReason::Stopped)) {
                entry.state = SchedState::Ready;
            }
        }
    }

    /// List runnable processes
    pub fn runnable_count(&self) -> usize {
        self.entries.iter().filter(|e| e.is_runnable()).count()
    }

    /// List all entries
    pub fn entries(&self) -> &[SchedEntry] { &self.entries }
    pub fn entry_count(&self) -> usize { self.entries.len() }

    /// Statistics
    pub fn timer_ticks(&self) -> u64 { self.timer_ticks }
    pub fn context_switches(&self) -> u64 { self.context_switches }
    pub fn preemptions(&self) -> u64 { self.preemptions }
    pub fn voluntary_yields(&self) -> u64 { self.voluntary_yields }

    /// Check if any process is runnable
    pub fn has_runnable(&self) -> bool {
        self.entries.iter().any(|e| e.is_runnable())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Integrated User Process Manager
// ═══════════════════════════════════════════════════════════════════════════════

/// Combines UserspaceManager + SignalManager + UserScheduler
/// into a single coherent user-space process management system.
pub struct UserProcessSystem {
    pub userspace:  UserspaceManager,
    pub signals:    SignalManager,
    pub scheduler:  UserScheduler,
}

impl Default for UserProcessSystem {
    fn default() -> Self { Self::new() }
}

impl UserProcessSystem {
    pub fn new() -> Self {
        Self {
            userspace:  UserspaceManager::new(),
            signals:    SignalManager::new(),
            scheduler:  UserScheduler::new(),
        }
    }

    /// Spawn a new user process (full lifecycle)
    pub fn spawn(&mut self, binary: crate::userspace::UserBinary, priority: u8) -> Result<Pid, UserspaceError> {
        let pid = self.userspace.load_binary(binary)?;
        self.signals.register(pid);
        if let Some(ctx) = self.userspace.get_context(pid) {
            self.scheduler.add_process(pid, ctx, priority);
        }
        Ok(pid)
    }

    /// Kill a user process (signal + scheduler + userspace)
    pub fn kill(&mut self, pid: Pid, exit_code: ExitCode) -> bool {
        self.signals.unregister(pid);
        self.scheduler.exit_process(pid, exit_code, None);
        self.userspace.exit_process(pid, exit_code);
        true
    }

    /// Timer tick: preempt, deliver signals, schedule
    pub fn timer_tick(&mut self) -> Option<(Pid, SavedContext)> {
        let current = self.scheduler.current_pid();

        // Check for pending signals on current process
        if let Some(pid) = current {
            if let Some((signal, disp)) = self.signals.deliver(pid) {
                use crate::elf_loader::{SignalResolution, Signal};
                let resolution = SignalManager::resolve_action(signal, disp);
                match resolution {
                    SignalResolution::Terminate(_) | SignalResolution::Terminate(_)
                        if signal == Signal::SigKill || signal == Signal::SigTerm || signal == Signal::SigSegv =>
                    {
                        self.kill(pid, 128 + signal as i32);
                        return self.scheduler.schedule(None);
                    }
                    SignalResolution::Stop => {
                        if let Some(ctx) = current.and_then(|p| self.userspace.get_context(p)) {
                            return self.scheduler.stop_process(pid, Some(ctx));
                        }
                    }
                    _ => {}
                }
            }
        }

        // Normal timer tick → quantum check
        if let Some(pid) = current {
            if let Some(ctx) = self.userspace.get_context(pid) {
                let ctx_copy = *ctx;
                return self.scheduler.timer_tick(&ctx_copy);
            }
        }
        self.scheduler.schedule(None)
    }

    /// Reap zombie processes
    pub fn reap(&mut self) -> Vec<(Pid, ExitCode)> {
        let zombies = self.scheduler.reap_zombies();
        for (pid, _) in &zombies {
            self.signals.unregister(*pid);
        }
        zombies
    }

    /// Get process count
    pub fn process_count(&self) -> usize { self.scheduler.entry_count() }
    pub fn runnable_count(&self) -> usize { self.scheduler.runnable_count() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::userspace::{UserBinary, UserAddressSpace};

    fn make_context(pid: u32) -> UserContext {
        let bin = UserBinary::hello_world();
        let asp = UserAddressSpace::default();
        UserContext::new(Pid(pid), &bin, asp)
    }

    // --- IretFrame tests ---

    #[test]
    fn test_iret_frame_from_context() {
        let ctx = make_context(1000);
        let frame = IretFrame::from_user_context(&ctx);
        assert_eq!(frame.rip, ctx.rip);
        assert_eq!(frame.cs, 0x1B);
        assert_eq!(frame.ss, 0x23);
        assert_eq!(frame.rsp, ctx.rsp);
    }

    #[test]
    fn test_iret_frame_is_ring3() {
        let ctx = make_context(1000);
        let frame = IretFrame::from_user_context(&ctx);
        assert!(frame.is_ring3());
    }

    #[test]
    fn test_iret_frame_valid() {
        let ctx = make_context(1000);
        let frame = IretFrame::from_user_context(&ctx);
        assert!(frame.is_valid());
    }

    #[test]
    fn test_iret_frame_invalid_ring0() {
        let frame = IretFrame { rip: 0x1000, cs: 0x08, rflags: 0x202, rsp: 0x8000, ss: 0x10 };
        assert!(!frame.is_ring3());
        assert!(!frame.is_valid());
    }

    #[test]
    fn test_iret_frame_invalid_no_if() {
        let frame = IretFrame { rip: 0x1000, cs: 0x1B, rflags: 0x000, rsp: 0x8000, ss: 0x23 };
        assert!(!frame.is_valid()); // IF not set
    }

    // --- SavedRegisters tests ---

    #[test]
    fn test_saved_registers_from_context() {
        let mut ctx = make_context(1000);
        ctx.rax = 0x1234;
        let regs = SavedRegisters::from_user_context(&ctx);
        assert_eq!(regs.rax, 0x1234);
        assert_eq!(regs.rsp, ctx.rsp);
    }

    #[test]
    fn test_saved_registers_apply() {
        let mut ctx = make_context(1000);
        let regs = SavedRegisters { rax: 0xDEAD, rbp: 0x5000, rsp: 0x7000, ..Default::default() };
        regs.apply_to(&mut ctx);
        assert_eq!(ctx.rax, 0xDEAD);
        assert_eq!(ctx.rbp, 0x5000);
        assert_eq!(ctx.rsp, 0x7000);
    }

    // --- SavedContext tests ---

    #[test]
    fn test_saved_context_roundtrip() {
        let mut ctx = make_context(1000);
        ctx.rax = 0xBEEF;
        let saved = SavedContext::from_user_context(&ctx);

        // Modify ctx
        ctx.rax = 0;
        ctx.rip = 0;

        // Restore
        saved.apply_to(&mut ctx);
        assert_eq!(ctx.rax, 0xBEEF);
        assert_eq!(ctx.rip, saved.iret.rip);
    }

    #[test]
    fn test_saved_context_is_ring3() {
        let ctx = make_context(1000);
        let saved = SavedContext::from_user_context(&ctx);
        assert!(saved.is_ring3());
        assert!(saved.is_valid());
    }

    // --- Quantum tests ---

    #[test]
    fn test_quantum_new() {
        let q = Quantum::new(10);
        assert_eq!(q.ticks_remaining, 10);
        assert_eq!(q.ticks_total, 10);
        assert!(!q.is_expired());
    }

    #[test]
    fn test_quantum_tick() {
        let mut q = Quantum::new(3);
        assert!(!q.tick()); // 2 remaining
        assert!(!q.tick()); // 1 remaining
        assert!(q.tick());  // 0 → expired
    }

    #[test]
    fn test_quantum_reset() {
        let mut q = Quantum::new(5);
        q.tick(); q.tick(); q.tick();
        assert_eq!(q.remaining(), 2);
        q.reset();
        assert_eq!(q.remaining(), 5);
    }

    #[test]
    fn test_quantum_default() {
        let q = Quantum::default_quantum();
        assert_eq!(q.ticks_total, 10);
    }

    #[test]
    fn test_quantum_expired_tick_stays_zero() {
        let mut q = Quantum::new(1);
        assert!(q.tick()); // expired
        assert!(q.tick()); // still expired (stays at 0)
    }

    // --- SchedState tests ---

    #[test]
    fn test_sched_state_runnable() {
        assert!(SchedState::Ready.is_runnable());
        assert!(SchedState::Running.is_runnable());
        assert!(!SchedState::Blocked(BlockReason::IoWait).is_runnable());
        assert!(!SchedState::Zombie(0).is_runnable());
    }

    #[test]
    fn test_sched_state_is_running() {
        assert!(SchedState::Running.is_running());
        assert!(!SchedState::Ready.is_running());
    }

    #[test]
    fn test_sched_state_is_blocked() {
        assert!(SchedState::Blocked(BlockReason::IoWait).is_blocked());
        assert!(!SchedState::Ready.is_blocked());
    }

    #[test]
    fn test_sched_state_is_zombie() {
        assert!(SchedState::Zombie(42).is_zombie());
        assert!(!SchedState::Ready.is_zombie());
    }

    #[test]
    fn test_block_reason_variants() {
        let io = BlockReason::IoWait;
        let sleep = BlockReason::Sleep(100);
        let stopped = BlockReason::Stopped;
        assert_ne!(io, stopped);
        assert_eq!(sleep, BlockReason::Sleep(100));
    }

    // --- SchedEntry tests ---

    #[test]
    fn test_sched_entry_new() {
        let ctx = make_context(1000);
        let entry = SchedEntry::new(Pid(1000), &ctx, 5);
        assert_eq!(entry.pid, Pid(1000));
        assert!(entry.is_runnable());
        assert_eq!(entry.priority, 5);
        assert_eq!(entry.total_cpu_ticks, 0);
    }

    #[test]
    fn test_sched_entry_save_restore() {
        let mut ctx = make_context(1000);
        let mut entry = SchedEntry::new(Pid(1000), &ctx, 0);

        ctx.rax = 0xCAFE;
        entry.save_context(&ctx);
        assert_eq!(entry.context_switches, 1);

        let saved = entry.restore_context();
        assert_eq!(saved.regs.rax, 0xCAFE);
    }

    #[test]
    fn test_sched_entry_quantum_tick() {
        let ctx = make_context(1000);
        let mut entry = SchedEntry::new(Pid(1000), &ctx, 0);
        entry.quantum = Quantum::new(2);
        assert!(!entry.tick_quantum());
        assert_eq!(entry.total_cpu_ticks, 1);
        assert!(entry.tick_quantum()); // expired
        assert_eq!(entry.total_cpu_ticks, 2);
    }

    #[test]
    fn test_sched_entry_reset_quantum() {
        let ctx = make_context(1000);
        let mut entry = SchedEntry::new(Pid(1000), &ctx, 0);
        entry.quantum = Quantum::new(3);
        entry.tick_quantum();
        entry.tick_quantum();
        entry.reset_quantum();
        assert_eq!(entry.quantum.remaining(), 3);
    }

    // --- UserScheduler tests ---

    #[test]
    fn test_scheduler_new() {
        let sched = UserScheduler::new();
        assert_eq!(sched.entry_count(), 0);
        assert_eq!(sched.current_pid(), None);
        assert_eq!(sched.timer_ticks(), 0);
    }

    #[test]
    fn test_add_process() {
        let mut sched = UserScheduler::new();
        let ctx = make_context(1000);
        sched.add_process(Pid(1000), &ctx, 0);
        assert_eq!(sched.entry_count(), 1);
        assert!(sched.has_runnable());
    }

    #[test]
    fn test_remove_process() {
        let mut sched = UserScheduler::new();
        let ctx = make_context(1000);
        sched.add_process(Pid(1000), &ctx, 0);
        assert!(sched.remove_process(Pid(1000)));
        assert_eq!(sched.entry_count(), 0);
    }

    #[test]
    fn test_schedule_first_process() {
        let mut sched = UserScheduler::new();
        let ctx = make_context(1000);
        sched.add_process(Pid(1000), &ctx, 0);

        let result = sched.schedule(None);
        assert!(result.is_some());
        let (pid, _) = result.unwrap();
        assert_eq!(pid, Pid(1000));
        assert_eq!(sched.current_pid(), Some(Pid(1000)));
    }

    #[test]
    fn test_schedule_round_robin() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);

        // First schedule: P1000
        let (pid, _) = sched.schedule(None).unwrap();
        assert_eq!(pid, Pid(1000));

        // Second schedule: P1001 (round-robin, skip current)
        let (pid, _) = sched.schedule(Some(&ctx1)).unwrap();
        assert_eq!(pid, Pid(1001));

        // Third schedule: P1000 again
        let (pid, _) = sched.schedule(Some(&ctx2)).unwrap();
        assert_eq!(pid, Pid(1000));
    }

    #[test]
    fn test_schedule_priority() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 5);  // Lower priority
        sched.add_process(Pid(1001), &ctx2, 1);  // Higher priority

        let (pid, _) = sched.schedule(None).unwrap();
        assert_eq!(pid, Pid(1001)); // Higher priority (lower number) first
    }

    #[test]
    fn test_schedule_no_runnable() {
        let mut sched = UserScheduler::new();
        assert_eq!(sched.schedule(None), None);
    }

    #[test]
    fn test_timer_tick_no_preemption() {
        let mut sched = UserScheduler::new();
        let ctx = make_context(1000);
        sched.add_process(Pid(1000), &ctx, 0);
        sched.schedule(None); // Start P1000

        // Tick within quantum → no switch
        let result = sched.timer_tick(&ctx);
        assert!(result.is_none()); // No preemption yet
        assert_eq!(sched.timer_ticks(), 1);
    }

    #[test]
    fn test_timer_tick_preemption() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);
        sched.schedule(None); // Start P1000

        // Set quantum to 1 tick
        if let Some(e) = sched.get_entry_mut(Pid(1000)) {
            e.quantum = Quantum::new(1);
        }

        // First tick: quantum expires → preempt
        let result = sched.timer_tick(&ctx1);
        assert!(result.is_some());
        let (pid, _) = result.unwrap();
        assert_eq!(pid, Pid(1001)); // Switched to P1001
        assert_eq!(sched.preemptions(), 1);
    }

    #[test]
    fn test_voluntary_yield() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);
        sched.schedule(None); // Start P1000

        let result = sched.yield_now(&ctx1);
        assert!(result.is_some());
        let (pid, _) = result.unwrap();
        assert_eq!(pid, Pid(1001)); // Yielded to P1001
        assert_eq!(sched.voluntary_yields(), 1);
    }

    #[test]
    fn test_block_and_unblock() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);
        sched.schedule(None); // Start P1000

        // Block P1000
        let result = sched.block_current(BlockReason::IoWait, &ctx1);
        assert!(result.is_some());
        let (pid, _) = result.unwrap();
        assert_eq!(pid, Pid(1001)); // Switched to P1001

        // P1000 should be blocked
        let entry = sched.get_entry(Pid(1000)).unwrap();
        assert!(entry.state.is_blocked());

        // Unblock P1000
        sched.unblock(Pid(1000));
        let entry = sched.get_entry(Pid(1000)).unwrap();
        assert!(entry.is_runnable());
    }

    #[test]
    fn test_exit_process() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);
        sched.schedule(None); // Start P1000

        let result = sched.exit_process(Pid(1000), 42, Some(&ctx1));
        assert!(result.is_some()); // Should schedule P1001
        let (pid, _) = result.unwrap();
        assert_eq!(pid, Pid(1001));

        let entry = sched.get_entry(Pid(1000)).unwrap();
        assert!(entry.state.is_zombie());
        if let SchedState::Zombie(code) = entry.state {
            assert_eq!(code, 42);
        }
    }

    #[test]
    fn test_reap_zombies() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);

        sched.exit_process(Pid(1000), 0, None);
        assert_eq!(sched.entry_count(), 2); // Still there as zombie

        let zombies = sched.reap_zombies();
        assert_eq!(zombies.len(), 1);
        assert_eq!(zombies[0], (Pid(1000), 0));
        assert_eq!(sched.entry_count(), 1); // Reaped
    }

    #[test]
    fn test_sleep_and_wake() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);
        sched.schedule(None); // Start P1000

        // Sleep P1000 for 5 ticks
        let result = sched.sleep(Pid(1000), 5, &ctx1);
        assert_eq!(result.unwrap().0, Pid(1001)); // Switched to P1001

        let entry = sched.get_entry(Pid(1000)).unwrap();
        assert!(entry.state.is_blocked());

        // Tick 5 times → P1000 should wake up
        for _ in 0..5 {
            sched.timer_tick(&ctx2);
        }

        let entry = sched.get_entry(Pid(1000)).unwrap();
        assert!(entry.is_runnable()); // Woke up
    }

    #[test]
    fn test_stop_and_continue() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);
        sched.schedule(None); // Start P1000

        // Stop P1000
        sched.stop_process(Pid(1000), Some(&ctx1));
        let entry = sched.get_entry(Pid(1000)).unwrap();
        assert_eq!(entry.state, SchedState::Blocked(BlockReason::Stopped));

        // Continue P1000
        sched.continue_process(Pid(1000));
        let entry = sched.get_entry(Pid(1000)).unwrap();
        assert!(entry.is_runnable());
    }

    #[test]
    fn test_stats_tracking() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);

        sched.schedule(None); // 1 context switch
        sched.yield_now(&ctx1); // 1 yield + 1 context switch
        assert_eq!(sched.context_switches(), 2);
        assert_eq!(sched.voluntary_yields(), 1);
    }

    #[test]
    fn test_scheduler_with_three_processes() {
        let mut sched = UserScheduler::new();
        let ctxs = [make_context(1000), make_context(1001), make_context(1002)];
        for (i, ctx) in ctxs.iter().enumerate() {
            sched.add_process(Pid(1000 + i as u32), ctx, 0);
        }

        // Schedule round-robin through all three
        let p1 = sched.schedule(None).unwrap().0;
        let p2 = sched.schedule(Some(&ctxs[0])).unwrap().0;
        let p3 = sched.schedule(Some(&ctxs[1])).unwrap().0;
        let p4 = sched.schedule(Some(&ctxs[2])).unwrap().0;

        assert_ne!(p1, p2);
        assert_ne!(p2, p3);
        assert_ne!(p3, p4);
        // After 3 schedules, should cycle back
    }

    #[test]
    fn test_runnable_count() {
        let mut sched = UserScheduler::new();
        let ctx1 = make_context(1000);
        let ctx2 = make_context(1001);
        sched.add_process(Pid(1000), &ctx1, 0);
        sched.add_process(Pid(1001), &ctx2, 0);
        assert_eq!(sched.runnable_count(), 2);

        sched.block_current(BlockReason::IoWait, &ctx1);
        // After blocking current (P1000), only P1001 is runnable
        // But P1001 is now running, so runnable = running + ready
        assert_eq!(sched.runnable_count(), 1); // Only P1001 (running)
    }

    // --- UserProcessSystem (integrated) tests ---

    #[test]
    fn test_system_new() {
        let sys = UserProcessSystem::new();
        assert_eq!(sys.process_count(), 0);
        assert_eq!(sys.runnable_count(), 0);
    }

    #[test]
    fn test_system_spawn() {
        let mut sys = UserProcessSystem::new();
        let pid = sys.spawn(UserBinary::hello_world(), 0);
        assert!(pid.is_ok());
        assert_eq!(sys.process_count(), 1);
        assert_eq!(sys.runnable_count(), 1);
    }

    #[test]
    fn test_system_spawn_multiple() {
        let mut sys = UserProcessSystem::new();
        for _ in 0..3 {
            sys.spawn(UserBinary::hello_world(), 0).unwrap();
        }
        assert_eq!(sys.process_count(), 3);
    }

    #[test]
    fn test_system_kill() {
        let mut sys = UserProcessSystem::new();
        let pid = sys.spawn(UserBinary::hello_world(), 0).unwrap();
        assert!(sys.kill(pid, 0));
        let zombies = sys.reap();
        assert_eq!(zombies.len(), 1);
        assert_eq!(zombies[0].0, pid);
    }

    #[test]
    fn test_system_timer_tick_no_switch() {
        let mut sys = UserProcessSystem::new();
        let pid = sys.spawn(UserBinary::hello_world(), 0).unwrap();

        // First tick: should start the process but no switch needed
        let _ = sys.scheduler.schedule(None);

        // Tick within quantum
        let result = sys.timer_tick();
        // Might or might not switch depending on quantum
        // Just verify it doesn't crash
        assert!(sys.scheduler.timer_ticks() >= 1);
    }

    #[test]
    fn test_system_reap() {
        let mut sys = UserProcessSystem::new();
        let p1 = sys.spawn(UserBinary::hello_world(), 0).unwrap();
        let _p2 = sys.spawn(UserBinary::hello_world(), 0).unwrap();

        sys.kill(p1, 42);
        let zombies = sys.reap();
        assert_eq!(zombies.len(), 1);
        assert_eq!(zombies[0], (p1, 42));
        assert_eq!(sys.process_count(), 1); // Only p2 remains
    }

    #[test]
    fn test_full_lifecycle_integrated() {
        let mut sys = UserProcessSystem::new();

        // Spawn 3 processes
        let p1 = sys.spawn(UserBinary::hello_world(), 0).unwrap();
        let p2 = sys.spawn(UserBinary::hello_world(), 1).unwrap();
        let p3 = sys.spawn(UserBinary::hello_world(), 2).unwrap();

        assert_eq!(sys.process_count(), 3);
        assert!(sys.runnable_count() >= 1);

        // Schedule
        let first = sys.scheduler.schedule(None).unwrap().0;

        // Kill one
        sys.kill(p2, 0);
        let zombies = sys.reap();
        assert_eq!(zombies.len(), 1);
        assert_eq!(sys.process_count(), 2);

        // Timer ticks
        for _ in 0..20 {
            sys.timer_tick();
        }
        assert!(sys.scheduler.timer_ticks() >= 20);
    }
}
