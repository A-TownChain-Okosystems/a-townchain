// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 39: Threading + Futex
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// Multi-Threaded User-Prozesse: Thread (tid, regs, stack, TLS), ThreadGroup,
// clone() Syscall, Futex (wait/wake/requeue), ThreadLocal Storage,
// Thread-Exit/Detach/Join, Thread-Scheduler Integration.

#![allow(dead_code)]

// ─── Thread ID ────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tid(pub u64);

impl Tid {
    pub fn new(n: u64) -> Self { Tid(n) }
    pub fn as_u64(&self) -> u64 { self.0 }
    pub fn is_zero(&self) -> bool { self.0 == 0 }
}

// ─── Thread State ─────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreadState {
    Created,
    Ready,
    Running,
    Blocked(BlockReason),
    Exited(ExitCode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockReason {
    Futex,
    Signal,
    IoWait,
    Sleep,
    Stopped,
    Parked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExitCode {
    Success,
    Error(u32),
    Killed(SignalType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalType {
    Kill,
    Term,
    Segv,
    Abort,
}

impl ExitCode {
    pub fn is_success(&self) -> bool {
        matches!(self, ExitCode::Success)
    }
    pub fn code(&self) -> i32 {
        match self {
            ExitCode::Success => 0,
            ExitCode::Error(c) => *c as i32,
            ExitCode::Killed(_) => -1,
        }
    }
}

// ─── Saved Registers ──────────────────────────────────────────────
#[derive(Clone, Copy, Debug, Default)]
pub struct SavedRegs {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub rsp: u64,
    pub r8: u64, pub r9: u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64, pub rflags: u64,
}

impl SavedRegs {
    pub fn new() -> Self { Self::default() }
    pub fn new_with_rsp_rip(rsp: u64, rip: u64) -> Self {
        let mut r = Self::default();
        r.rsp = rsp;
        r.rip = rip;
        r.rflags = 0x202; // IF=1
        r
    }
}

// ─── Thread-Local Storage ────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct TlsBlock {
    pub addr: u64,
    pub size: usize,
    pub data: Vec<u8>,
}

impl TlsBlock {
    pub fn new(size: usize) -> Self {
        Self { addr: 0, size, data: vec![0; size] }
    }
    pub fn new_init(addr: u64, data: &[u8]) -> Self {
        Self { addr, size: data.len(), data: data.to_vec() }
    }
    pub fn write_at(&mut self, offset: usize, src: &[u8]) -> Result<(), ThreadError> {
        if offset.checked_add(src.len()).map_or(true, |e| e > self.size) {
            return Err(ThreadError::TlsOutOfBounds);
        }
        self.data[offset..offset + src.len()].copy_from_slice(src);
        Ok(())
    }
    pub fn read_at(&self, offset: usize, len: usize) -> Result<Vec<u8>, ThreadError> {
        if offset.checked_add(len).map_or(true, |e| e > self.size) {
            return Err(ThreadError::TlsOutOfBounds);
        }
        Ok(self.data[offset..offset + len].to_vec())
    }
}

// ─── Thread ───────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct Thread {
    pub tid: Tid,
    pub pid: u64,
    pub state: ThreadState,
    pub regs: SavedRegs,
    pub stack_top: u64,
    pub stack_size: usize,
    pub tls: Option<TlsBlock>,
    pub user_arg: u64,
    pub exit_code: Option<ExitCode>,
    pub create_time_ns: u64,
    pub cpu_time_ns: u64,
    pub futex_addr: Option<u64>,
}

impl Thread {
    pub fn new(tid: Tid, pid: u64, stack_top: u64, stack_size: usize) -> Self {
        Self {
            tid, pid,
            state: ThreadState::Created,
            regs: SavedRegs::new(),
            stack_top, stack_size,
            tls: None,
            user_arg: 0,
            exit_code: None,
            create_time_ns: 0,
            cpu_time_ns: 0,
            futex_addr: None,
        }
    }

    pub fn new_main(pid: u64, rsp: u64, rip: u64) -> Self {
        Self {
            tid: Tid(pid),
            pid,
            state: ThreadState::Running,
            regs: SavedRegs::new_with_rsp_rip(rsp, rip),
            stack_top: rsp,
            stack_size: 8 * 1024 * 1024,
            tls: None,
            user_arg: 0,
            exit_code: None,
            create_time_ns: 0,
            cpu_time_ns: 0,
            futex_addr: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        matches!(self.state, ThreadState::Ready | ThreadState::Running | ThreadState::Blocked(_))
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, ThreadState::Exited(_))
    }

    pub fn set_blocked(&mut self, reason: BlockReason) {
        self.state = ThreadState::Blocked(reason);
    }

    pub fn set_ready(&mut self) {
        self.state = ThreadState::Ready;
    }

    pub fn set_running(&mut self) {
        self.state = ThreadState::Running;
    }

    pub fn exit(&mut self, code: ExitCode) {
        self.exit_code = Some(code);
        self.state = ThreadState::Exited(code);
    }

    pub fn set_tls(&mut self, tls: TlsBlock) {
        self.tls = Some(tls);
    }

    pub fn tick_cpu(&mut self, ns: u64) {
        self.cpu_time_ns += ns;
    }
}

// ─── Thread Group ─────────────────────────────────────────────────
#[derive(Debug)]
pub struct ThreadGroup {
    pub pid: u64,
    pub threads: Vec<Thread>,
    pub next_tid: u64,
    pub main_tid: Tid,
}

impl ThreadGroup {
    pub fn new(pid: u64, main_thread: Thread) -> Self {
        let main_tid = main_thread.tid;
        Self {
            pid,
            threads: vec![main_thread],
            next_tid: pid + 1,
            main_tid,
        }
    }

    pub fn get_thread(&self, tid: Tid) -> Option<&Thread> {
        self.threads.iter().find(|t| t.tid == tid)
    }

    pub fn get_thread_mut(&mut self, tid: Tid) -> Option<&mut Thread> {
        self.threads.iter_mut().find(|t| t.tid == tid)
    }

    pub fn alloc_tid(&mut self) -> Tid {
        let t = Tid(self.next_tid);
        self.next_tid += 1;
        t
    }

    pub fn add_thread(&mut self, thread: Thread) -> Tid {
        let tid = thread.tid;
        self.threads.push(thread);
        tid
    }

    pub fn remove_thread(&mut self, tid: Tid) -> Option<Thread> {
        if tid == self.main_tid {
            return None;
        }
        let pos = self.threads.iter().position(|t| t.tid == tid)?;
        Some(self.threads.remove(pos))
    }

    pub fn alive_threads(&self) -> usize {
        self.threads.iter().filter(|t| t.is_alive()).count()
    }

    pub fn exited_threads(&self) -> Vec<Tid> {
        self.threads.iter().filter(|t| t.is_exited()).map(|t| t.tid).collect()
    }

    pub fn reap_exited(&mut self) -> Vec<Thread> {
        let mut reaped = Vec::new();
        self.threads.retain(|t| {
            if t.is_exited() && t.tid != self.main_tid {
                reaped.push(t.clone());
                false
            } else {
                true
            }
        });
        reaped
    }

    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }

    pub fn ready_count(&self) -> usize {
        self.threads.iter().filter(|t| matches!(t.state, ThreadState::Ready | ThreadState::Running)).count()
    }
}

// ─── Clone Flags ──────────────────────────────────────────────────
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CloneFlags: u64 {
        const VM         = 1 << 0;  // CLONE_VM — share memory space
        const FS         = 1 << 1;  // CLONE_FS — share cwd/root
        const FILES      = 1 << 2;  // CLONE_FILES — share fd table
        const SIGHAND    = 1 << 3;  // CLONE_SIGHAND — share signal handlers
        const PARENT     = 1 << 4;  // CLONE_PARENT — same parent
        const THREAD     = 1 << 5;  // CLONE_THREAD — same thread group
        const NEWNS      = 1 << 6;  // CLONE_NEWNS — new mount namespace
        const SYSVSEM    = 1 << 7;  // CLONE_SYSVSEM — share SysV semaphores
        const SETTLS     = 1 << 8;  // CLONE_SETTLS — set TLS area
        const PARENT_SETTID = 1 << 9;  // CLONE_PARENT_SETTID
        const CHILD_CLEARTID = 1 << 10; // CLONE_CHILD_CLEARTID
        const CHILD_SETTID  = 1 << 11; // CLONE_CHILD_SETTID
        const VFORK     = 1 << 12; // CLONE_VFORK
        const IO        = 1 << 13; // CLONE_IO — share i/o context
    }
}

impl CloneFlags {
    pub fn default_thread() -> Self {
        Self::VM | Self::FS | Self::FILES | Self::SIGHAND | Self::THREAD
    }

    pub fn default_process() -> Self {
        CloneFlags::empty()
    }

    pub fn is_thread(&self) -> bool {
        self.contains(Self::THREAD)
    }

    pub fn shares_vm(&self) -> bool {
        self.contains(Self::VM)
    }

    pub fn shares_files(&self) -> bool {
        self.contains(Self::FILES)
    }

    pub fn sets_tls(&self) -> bool {
        self.contains(Self::SETTLS)
    }
}

// ─── Futex ────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct FutexWaiter {
    pub tid: Tid,
    pub pid: u64,
    pub addr: u64,
    pub val: u32,
    pub bitset: u32,
    pub queued_ns: u64,
}

impl FutexWaiter {
    pub fn new(tid: Tid, pid: u64, addr: u64, val: u32) -> Self {
        Self { tid, pid, addr, val, bitset: 0xFFFFFFFF, queued_ns: 0 }
    }

    pub fn new_bitset(tid: Tid, pid: u64, addr: u64, val: u32, bitset: u32) -> Self {
        Self { tid, pid, addr, val, bitset, queued_ns: 0 }
    }
}

#[derive(Debug, Default)]
pub struct FutexTable {
    waiters: Vec<FutexWaiter>,
    next_key: u64,
}

impl FutexTable {
    pub fn new() -> Self {
        Self { waiters: Vec::new(), next_key: 1 }
    }

    pub fn wait(&mut self, tid: Tid, pid: u64, addr: u64, val: u32, current_val: u32) -> Result<bool, ThreadError> {
        if current_val != val {
            return Ok(false); // EAGAIN — value changed
        }
        let waiter = FutexWaiter::new(tid, pid, addr, val);
        self.waiters.push(waiter);
        Ok(true)
    }

    pub fn wait_bitset(&mut self, tid: Tid, pid: u64, addr: u64, val: u32, bitset: u32, current_val: u32) -> Result<bool, ThreadError> {
        if current_val != val {
            return Ok(false);
        }
        let waiter = FutexWaiter::new_bitset(tid, pid, addr, val, bitset);
        self.waiters.push(waiter);
        Ok(true)
    }

    pub fn wake(&mut self, addr: u64, max_count: usize) -> usize {
        let mut woken = 0;
        let mut to_wake = Vec::new();
        self.waiters.retain(|w| {
            if w.addr == addr && woken < max_count {
                to_wake.push(w.tid);
                woken += 1;
                false
            } else {
                true
            }
        });
        woken
    }

    pub fn wake_bitset(&mut self, addr: u64, max_count: usize, bitset: u32) -> usize {
        let mut woken = 0;
        self.waiters.retain(|w| {
            if w.addr == addr && (w.bitset & bitset) != 0 && woken < max_count {
                woken += 1;
                false
            } else {
                true
            }
        });
        woken
    }

    pub fn requeue(&mut self, addr1: u64, addr2: u64, max_wake: usize, max_requeue: usize) -> (usize, usize) {
        let mut woken = 0;
        let mut requeued = 0;
        for w in &mut self.waiters {
            if w.addr == addr1 {
                if woken < max_wake {
                    woken += 1;
                } else if requeued < max_requeue {
                    w.addr = addr2;
                    requeued += 1;
                }
            }
        }
        self.waiters.retain(|w| {
            if w.addr == addr1 && woken > 0 {
                woken -= 1;
                false
            } else {
                true
            }
        });
        (woken, requeued)
    }

    pub fn waiting_count(&self) -> usize {
        self.waiters.len()
    }

    pub fn waiting_at(&self, addr: u64) -> usize {
        self.waiters.iter().filter(|w| w.addr == addr).count()
    }

    pub fn waiting_for(&self, tid: Tid) -> bool {
        self.waiters.iter().any(|w| w.tid == tid)
    }

    pub fn remove_waiter(&mut self, tid: Tid) -> bool {
        let pos = self.waiters.iter().position(|w| w.tid == tid);
        if let Some(i) = pos {
            self.waiters.remove(i);
            true
        } else {
            false
        }
    }

    pub fn clear_pid(&mut self, pid: u64) -> usize {
        let before = self.waiters.len();
        self.waiters.retain(|w| w.pid != pid);
        before - self.waiters.len()
    }
}

// ─── Thread Manager ───────────────────────────────────────────────
#[derive(Debug)]
pub struct ThreadManager {
    pub groups: Vec<ThreadGroup>,
    pub futex: FutexTable,
    pub next_pid: u64,
    pub tick_count: u64,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self { groups: Vec::new(), futex: FutexTable::new(), next_pid: 1, tick_count: 0 }
    }

    pub fn create_process(&mut self, main_rsp: u64, main_rip: u64) -> u64 {
        let pid = self.next_pid;
        self.next_pid += 1;
        let main = Thread::new_main(pid, main_rsp, main_rip);
        let group = ThreadGroup::new(pid, main);
        self.groups.push(group);
        pid
    }

    pub fn create_thread(&mut self, pid: u64, stack_top: u64, stack_size: usize, entry: u64, arg: u64, flags: CloneFlags) -> Result<Tid, ThreadError> {
        let group = self.groups.iter_mut().find(|g| g.pid == pid)
            .ok_or(ThreadError::ProcessNotFound)?;
        let tid = group.alloc_tid();
        let mut thread = Thread::new(tid, pid, stack_top, stack_size);
        thread.regs = SavedRegs::new_with_rsp_rip(stack_top, entry);
        thread.regs.rdi = arg;
        thread.user_arg = arg;
        thread.set_ready();
        if flags.sets_tls() {
            thread.tls = Some(TlsBlock::new(4096));
        }
        group.add_thread(thread);
        Ok(tid)
    }

    pub fn exit_thread(&mut self, pid: u64, tid: Tid, code: ExitCode) -> Result<(), ThreadError> {
        let group = self.groups.iter_mut().find(|g| g.pid == pid)
            .ok_or(ThreadError::ProcessNotFound)?;
        let thread = group.get_thread_mut(tid)
            .ok_or(ThreadError::ThreadNotFound)?;
        thread.exit(code);
        self.futex.remove_waiter(tid);
        Ok(())
    }

    pub fn join_thread(&mut self, pid: u64, tid: Tid) -> Result<ExitCode, ThreadError> {
        let group = self.groups.iter_mut().find(|g| g.pid == pid)
            .ok_or(ThreadError::ProcessNotFound)?;
        let thread = group.get_thread(tid)
            .ok_or(ThreadError::ThreadNotFound)?;
        if !thread.is_exited() {
            return Err(ThreadError::ThreadNotExited);
        }
        thread.exit_code.ok_or(ThreadError::ThreadNotExited)
    }

    pub fn kill_thread(&mut self, pid: u64, tid: Tid, sig: SignalType) -> Result<(), ThreadError> {
        let group = self.groups.iter_mut().find(|g| g.pid == pid)
            .ok_or(ThreadError::ProcessNotFound)?;
        let thread = group.get_thread_mut(tid)
            .ok_or(ThreadError::ThreadNotFound)?;
        if !thread.is_alive() {
            return Err(ThreadError::ThreadNotAlive);
        }
        thread.exit(ExitCode::Killed(sig));
        self.futex.remove_waiter(tid);
        Ok(())
    }

    pub fn block_thread(&mut self, pid: u64, tid: Tid, reason: BlockReason) -> Result<(), ThreadError> {
        let group = self.groups.iter_mut().find(|g| g.pid == pid)
            .ok_or(ThreadError::ProcessNotFound)?;
        let thread = group.get_thread_mut(tid)
            .ok_or(ThreadError::ThreadNotFound)?;
        thread.set_blocked(reason);
        Ok(())
    }

    pub fn unblock_thread(&mut self, pid: u64, tid: Tid) -> Result<(), ThreadError> {
        let group = self.groups.iter_mut().find(|g| g.pid == pid)
            .ok_or(ThreadError::ProcessNotFound)?;
        let thread = group.get_thread_mut(tid)
            .ok_or(ThreadError::ThreadNotFound)?;
        thread.set_ready();
        Ok(())
    }

    pub fn futex_wait(&mut self, pid: u64, tid: Tid, addr: u64, val: u32, current_val: u32) -> Result<bool, ThreadError> {
        let waited = self.futex.wait(tid, pid, addr, val, current_val)?;
        if waited {
            self.block_thread(pid, tid, BlockReason::Futex)?;
        }
        Ok(waited)
    }

    pub fn futex_wake(&mut self, addr: u64, max_count: usize) -> Result<usize, ThreadError> {
        let woken = self.futex.wake(addr, max_count);
        for group in &mut self.groups {
            for thread in &mut group.threads {
                if thread.futex_addr == Some(addr) && thread.is_alive() {
                    thread.set_ready();
                    thread.futex_addr = None;
                }
            }
        }
        Ok(woken)
    }

    pub fn futex_wake_bitset(&mut self, addr: u64, max_count: usize, bitset: u32) -> Result<usize, ThreadError> {
        let woken = self.futex.wake_bitset(addr, max_count, bitset);
        Ok(woken)
    }

    pub fn futex_requeue(&mut self, addr1: u64, addr2: u64, max_wake: usize, max_requeue: usize) -> Result<(usize, usize), ThreadError> {
        let (woken, requeued) = self.futex.requeue(addr1, addr2, max_wake, max_requeue);
        Ok((woken, requeued))
    }

    pub fn reap_exited(&mut self, pid: u64) -> Vec<Thread> {
        let group = self.groups.iter_mut().find(|g| g.pid == pid);
        match group {
            Some(g) => g.reap_exited(),
            None => Vec::new(),
        }
    }

    pub fn get_thread(&self, pid: u64, tid: Tid) -> Option<&Thread> {
        self.groups.iter().find(|g| g.pid == pid)?.get_thread(tid)
    }

    pub fn get_thread_mut(&mut self, pid: u64, tid: Tid) -> Option<&mut Thread> {
        self.groups.iter_mut().find(|g| g.pid == pid)?.get_thread_mut(tid)
    }

    pub fn get_group(&self, pid: u64) -> Option<&ThreadGroup> {
        self.groups.iter().find(|g| g.pid == pid)
    }

    pub fn get_group_mut(&mut self, pid: u64) -> Option<&mut ThreadGroup> {
        self.groups.iter_mut().find(|g| g.pid == pid)
    }

    pub fn tick(&mut self, quantum_ns: u64) {
        self.tick_count += 1;
        for group in &mut self.groups {
            for thread in &mut group.threads {
                if matches!(thread.state, ThreadState::Running) {
                    thread.tick_cpu(quantum_ns);
                }
            }
        }
    }

    pub fn total_threads(&self) -> usize {
        self.groups.iter().map(|g| g.thread_count()).sum()
    }

    pub fn alive_threads(&self) -> usize {
        self.groups.iter().map(|g| g.alive_threads()).sum()
    }

    pub fn process_count(&self) -> usize {
        self.groups.len()
    }

    pub fn destroy_process(&mut self, pid: u64) -> Result<(), ThreadError> {
        let pos = self.groups.iter().position(|g| g.pid == pid)
            .ok_or(ThreadError::ProcessNotFound)?;
        self.futex.clear_pid(pid);
        self.groups.remove(pos);
        Ok(())
    }

    pub fn stats(&self) -> ThreadStats {
        ThreadStats {
            processes: self.process_count(),
            threads: self.total_threads(),
            alive: self.alive_threads(),
            futex_waiters: self.futex.waiting_count(),
            ticks: self.tick_count,
        }
    }
}

// ─── Thread Stats ─────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct ThreadStats {
    pub processes: usize,
    pub threads: usize,
    pub alive: usize,
    pub futex_waiters: usize,
    pub ticks: u64,
}

// ─── Errors ───────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadError {
    ProcessNotFound,
    ThreadNotFound,
    ThreadNotAlive,
    ThreadNotExited,
    TlsOutOfBounds,
    InvalidCloneFlags,
    FutexValueMismatch,
}

// ─── Mutex (Userspace Helper) ─────────────────────────────────────
#[derive(Debug, Clone)]
pub struct FutexMutex {
    pub addr: u64,
    pub state: u32,
}

impl FutexMutex {
    pub const UNLOCKED: u32 = 0;
    pub const LOCKED: u32 = 1;
    pub const CONTESTED: u32 = 2;

    pub fn new(addr: u64) -> Self {
        Self { addr, state: Self::UNLOCKED }
    }

    pub fn try_lock(&mut self) -> bool {
        if self.state == Self::UNLOCKED {
            self.state = Self::LOCKED;
            true
        } else {
            false
        }
    }

    pub fn lock(&mut self) -> bool {
        if self.try_lock() {
            return true;
        }
        self.state = Self::CONTESTED;
        false // caller should futex_wait
    }

    pub fn unlock(&mut self) -> bool {
        let was_contested = self.state == Self::CONTESTED;
        self.state = Self::UNLOCKED;
        was_contested // if contested, caller should futex_wake
    }

    pub fn is_locked(&self) -> bool {
        self.state != Self::UNLOCKED
    }
}

// ─── Condition Variable (Userspace Helper) ────────────────────────
#[derive(Debug, Clone)]
pub struct FutexCondvar {
    pub seq_addr: u64,
    pub seq: u32,
}

impl FutexCondvar {
    pub fn new(seq_addr: u64) -> Self {
        Self { seq_addr, seq: 0 }
    }

    pub fn signal(&mut self) -> u32 {
        self.seq += 1;
        self.seq
    }

    pub fn broadcast(&mut self) -> u32 {
        self.seq += 1;
        self.seq
    }

    pub fn current_seq(&self) -> u32 {
        self.seq
    }
}

// ─── Barrier ──────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct FutexBarrier {
    pub count_addr: u64,
    pub target: usize,
    pub current: usize,
    pub generation: u32,
}

impl FutexBarrier {
    pub fn new(count_addr: u64, target: usize) -> Self {
        Self { count_addr, target, current: 0, generation: 0 }
    }

    pub fn arrive(&mut self) -> BarrierResult {
        self.current += 1;
        if self.current >= self.target {
            self.current = 0;
            self.generation += 1;
            BarrierResult::Released(self.generation)
        } else {
            BarrierResult::Waiting(self.generation)
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarrierResult {
    Waiting(u32),
    Released(u32),
}

// ─── RwLock (Read-Write Lock) ─────────────────────────────────────
#[derive(Debug, Clone)]
pub struct FutexRwLock {
    pub state: i32,
}

impl FutexRwLock {
    pub fn new() -> Self {
        Self { state: 0 }
    }

    pub fn try_read_lock(&mut self) -> bool {
        if self.state >= 0 {
            self.state += 1;
            true
        } else {
            false
        }
    }

    pub fn try_write_lock(&mut self) -> bool {
        if self.state == 0 {
            self.state = -1;
            true
        } else {
            false
        }
    }

    pub fn read_unlock(&mut self) {
        if self.state > 0 {
            self.state -= 1;
        }
    }

    pub fn write_unlock(&mut self) {
        if self.state == -1 {
            self.state = 0;
        }
    }

    pub fn is_write_locked(&self) -> bool {
        self.state == -1
    }

    pub fn reader_count(&self) -> usize {
        if self.state > 0 { self.state as usize } else { 0 }
    }
}

// ─── Tests ────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── TID Tests ──
    #[test]
    fn test_tid_basic() {
        let t = Tid::new(42);
        assert_eq!(t.as_u64(), 42);
        assert!(!t.is_zero());
        assert!(Tid::new(0).is_zero());
    }

    #[test]
    fn test_tid_eq() {
        assert_eq!(Tid::new(1), Tid::new(1));
        assert_ne!(Tid::new(1), Tid::new(2));
    }

    // ── Thread State Tests ──
    #[test]
    fn test_thread_state_transitions() {
        let mut t = Thread::new(Tid(1), 100, 0x8000_0000, 8 * 1024 * 1024);
        assert_eq!(t.state, ThreadState::Created);
        t.set_ready();
        assert_eq!(t.state, ThreadState::Ready);
        t.set_running();
        assert_eq!(t.state, ThreadState::Running);
        t.set_blocked(BlockReason::Futex);
        assert_eq!(t.state, ThreadState::Blocked(BlockReason::Futex));
        t.set_ready();
        assert_eq!(t.state, ThreadState::Ready);
    }

    #[test]
    fn test_thread_exit() {
        let mut t = Thread::new(Tid(1), 100, 0x8000_0000, 4096);
        t.exit(ExitCode::Success);
        assert!(t.is_exited());
        assert!(!t.is_alive());
        assert!(t.exit_code.unwrap().is_success());
    }

    #[test]
    fn test_thread_exit_error() {
        let mut t = Thread::new(Tid(2), 100, 0x8000_0000, 4096);
        t.exit(ExitCode::Error(42));
        assert!(t.is_exited());
        assert_eq!(t.exit_code.unwrap().code(), 42);
    }

    #[test]
    fn test_thread_exit_killed() {
        let mut t = Thread::new(Tid(3), 100, 0x8000_0000, 4096);
        t.exit(ExitCode::Killed(SignalType::Kill));
        assert!(t.is_exited());
        assert_eq!(t.exit_code.unwrap().code(), -1);
    }

    #[test]
    fn test_thread_main() {
        let t = Thread::new_main(1, 0x7fff_0000, 0x401000);
        assert_eq!(t.tid, Tid(1));
        assert_eq!(t.pid, 1);
        assert_eq!(t.state, ThreadState::Running);
        assert_eq!(t.regs.rsp, 0x7fff_0000);
        assert_eq!(t.regs.rip, 0x401000);
        assert_eq!(t.regs.rflags, 0x202);
    }

    #[test]
    fn test_thread_cpu_time() {
        let mut t = Thread::new(Tid(1), 100, 0x8000_0000, 4096);
        t.tick_cpu(1_000_000);
        t.tick_cpu(500_000);
        assert_eq!(t.cpu_time_ns, 1_500_000);
    }

    // ── TLS Tests ──
    #[test]
    fn test_tls_basic() {
        let mut tls = TlsBlock::new(256);
        assert_eq!(tls.size, 256);
        tls.write_at(0, &[1, 2, 3, 4]).unwrap();
        assert_eq!(tls.read_at(0, 4).unwrap(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_tls_oob() {
        let mut tls = TlsBlock::new(16);
        assert!(tls.write_at(14, &[1, 2, 3]).is_err());
        assert!(tls.read_at(20, 4).is_err());
    }

    #[test]
    fn test_tls_init() {
        let tls = TlsBlock::new_init(0x1000, &[0xFF; 32]);
        assert_eq!(tls.addr, 0x1000);
        assert_eq!(tls.size, 32);
        assert_eq!(tls.data, vec![0xFF; 32]);
    }

    // ── ThreadGroup Tests ──
    #[test]
    fn test_thread_group_basic() {
        let main = Thread::new_main(1, 0x7fff_0000, 0x401000);
        let mut g = ThreadGroup::new(1, main);
        assert_eq!(g.pid, 1);
        assert_eq!(g.thread_count(), 1);
        assert_eq!(g.main_tid, Tid(1));
    }

    #[test]
    fn test_thread_group_add_remove() {
        let main = Thread::new_main(1, 0x7fff_0000, 0x401000);
        let mut g = ThreadGroup::new(1, main);
        let tid = g.alloc_tid();
        let t = Thread::new(tid, 1, 0x8000_0000, 4096);
        let added = g.add_thread(t);
        assert_eq!(added, tid);
        assert_eq!(g.thread_count(), 2);
        let removed = g.remove_thread(tid);
        assert!(removed.is_some());
        assert_eq!(g.thread_count(), 1);
    }

    #[test]
    fn test_thread_group_no_remove_main() {
        let main = Thread::new_main(1, 0x7fff_0000, 0x401000);
        let mut g = ThreadGroup::new(1, main);
        assert!(g.remove_thread(Tid(1)).is_none());
    }

    #[test]
    fn test_thread_group_alive_exited() {
        let main = Thread::new_main(1, 0x7fff_0000, 0x401000);
        let mut g = ThreadGroup::new(1, main);
        let t1 = g.alloc_tid();
        g.add_thread(Thread::new(t1, 1, 0x8000_0000, 4096));
        assert_eq!(g.alive_threads(), 2);
        g.get_thread_mut(t1).unwrap().exit(ExitCode::Success);
        assert_eq!(g.alive_threads(), 1);
        assert_eq!(g.exited_threads(), vec![t1]);
    }

    #[test]
    fn test_thread_group_reap() {
        let main = Thread::new_main(1, 0x7fff_0000, 0x401000);
        let mut g = ThreadGroup::new(1, main);
        let t1 = g.alloc_tid();
        let t2 = g.alloc_tid();
        g.add_thread(Thread::new(t1, 1, 0x8000_0000, 4096));
        g.add_thread(Thread::new(t2, 1, 0x8000_0000, 4096));
        g.get_thread_mut(t1).unwrap().exit(ExitCode::Success);
        g.get_thread_mut(t2).unwrap().exit(ExitCode::Error(1));
        let reaped = g.reap_exited();
        assert_eq!(reaped.len(), 2);
        assert_eq!(g.thread_count(), 1); // main only
    }

    #[test]
    fn test_thread_group_ready_count() {
        let main = Thread::new_main(1, 0x7fff_0000, 0x401000);
        let mut g = ThreadGroup::new(1, main);
        let t1 = g.alloc_tid();
        let mut t = Thread::new(t1, 1, 0x8000_0000, 4096);
        t.set_ready();
        g.add_thread(t);
        assert_eq!(g.ready_count(), 2); // main (Running) + t1 (Ready)
    }

    // ── CloneFlags Tests ──
    #[test]
    fn test_clone_flags_default_thread() {
        let f = CloneFlags::default_thread();
        assert!(f.is_thread());
        assert!(f.shares_vm());
        assert!(f.shares_files());
        assert!(!f.sets_tls());
    }

    #[test]
    fn test_clone_flags_default_process() {
        let f = CloneFlags::default_process();
        assert!(!f.is_thread());
        assert!(!f.shares_vm());
    }

    #[test]
    fn test_clone_flags_sets_tls() {
        let f = CloneFlags::default_thread() | CloneFlags::SETTLS;
        assert!(f.sets_tls());
    }

    // ── Futex Tests ──
    #[test]
    fn test_futex_wait_wake() {
        let mut ft = FutexTable::new();
        let r = ft.wait(Tid(1), 1, 0x1000, 0, 0);
        assert!(r.unwrap());
        assert_eq!(ft.waiting_count(), 1);
        assert_eq!(ft.waiting_at(0x1000), 1);
        let woken = ft.wake(0x1000, 1);
        assert_eq!(woken, 1);
        assert_eq!(ft.waiting_count(), 0);
    }

    #[test]
    fn test_futex_value_mismatch() {
        let mut ft = FutexTable::new();
        let r = ft.wait(Tid(1), 1, 0x1000, 0, 1); // val=0, current=1
        assert_eq!(r.unwrap(), false);
        assert_eq!(ft.waiting_count(), 0);
    }

    #[test]
    fn test_futex_multiple_waiters() {
        let mut ft = FutexTable::new();
        ft.wait(Tid(1), 1, 0x1000, 0, 0).unwrap();
        ft.wait(Tid(2), 1, 0x1000, 0, 0).unwrap();
        ft.wait(Tid(3), 1, 0x1000, 0, 0).unwrap();
        assert_eq!(ft.waiting_count(), 3);
        let woken = ft.wake(0x1000, 2);
        assert_eq!(woken, 2);
        assert_eq!(ft.waiting_count(), 1);
    }

    #[test]
    fn test_futex_wake_all() {
        let mut ft = FutexTable::new();
        ft.wait(Tid(1), 1, 0x2000, 0, 0).unwrap();
        ft.wait(Tid(2), 1, 0x2000, 0, 0).unwrap();
        ft.wait(Tid(3), 1, 0x2000, 0, 0).unwrap();
        let woken = ft.wake(0x2000, usize::MAX);
        assert_eq!(woken, 3);
    }

    #[test]
    fn test_futex_wake_no_match() {
        let mut ft = FutexTable::new();
        ft.wait(Tid(1), 1, 0x1000, 0, 0).unwrap();
        let woken = ft.wake(0x2000, 1);
        assert_eq!(woken, 0);
    }

    #[test]
    fn test_futex_bitset() {
        let mut ft = FutexTable::new();
        ft.wait_bitset(Tid(1), 1, 0x1000, 0, 0x01, 0).unwrap();
        ft.wait_bitset(Tid(2), 1, 0x1000, 0, 0x02, 0).unwrap();
        let woken = ft.wake_bitset(0x1000, 1, 0x02);
        assert_eq!(woken, 1);
        assert_eq!(ft.waiting_count(), 1);
    }

    #[test]
    fn test_futex_requeue() {
        let mut ft = FutexTable::new();
        ft.wait(Tid(1), 1, 0x1000, 0, 0).unwrap();
        ft.wait(Tid(2), 1, 0x1000, 0, 0).unwrap();
        ft.wait(Tid(3), 1, 0x1000, 0, 0).unwrap();
        let (w, r) = ft.requeue(0x1000, 0x2000, 1, 2);
        assert_eq!(w, 1);
        assert_eq!(r, 2);
    }

    #[test]
    fn test_futex_remove_waiter() {
        let mut ft = FutexTable::new();
        ft.wait(Tid(1), 1, 0x1000, 0, 0).unwrap();
        assert!(ft.remove_waiter(Tid(1)));
        assert!(!ft.waiting_for(Tid(1)));
    }

    #[test]
    fn test_futex_clear_pid() {
        let mut ft = FutexTable::new();
        ft.wait(Tid(1), 1, 0x1000, 0, 0).unwrap();
        ft.wait(Tid(2), 1, 0x1000, 0, 0).unwrap();
        ft.wait(Tid(3), 2, 0x1000, 0, 0).unwrap();
        let removed = ft.clear_pid(1);
        assert_eq!(removed, 2);
        assert_eq!(ft.waiting_count(), 1);
    }

    // ── ThreadManager Tests ──
    #[test]
    fn test_tm_create_process() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        assert_eq!(pid, 1);
        assert_eq!(tm.process_count(), 1);
        assert_eq!(tm.total_threads(), 1);
        assert_eq!(tm.alive_threads(), 1);
    }

    #[test]
    fn test_tm_create_thread() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0xAB, CloneFlags::default_thread()).unwrap();
        assert_ne!(tid, Tid(pid));
        assert_eq!(tm.total_threads(), 2);
    }

    #[test]
    fn test_tm_create_thread_with_tls() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let flags = CloneFlags::default_thread() | CloneFlags::SETTLS;
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, flags).unwrap();
        let t = tm.get_thread(pid, tid).unwrap();
        assert!(t.tls.is_some());
    }

    #[test]
    fn test_tm_exit_thread() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        tm.exit_thread(pid, tid, ExitCode::Success).unwrap();
        let t = tm.get_thread(pid, tid).unwrap();
        assert!(t.is_exited());
    }

    #[test]
    fn test_tm_join_thread() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        tm.exit_thread(pid, tid, ExitCode::Success).unwrap();
        let code = tm.join_thread(pid, tid).unwrap();
        assert!(code.is_success());
    }

    #[test]
    fn test_tm_join_not_exited() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        assert_eq!(tm.join_thread(pid, tid), Err(ThreadError::ThreadNotExited));
    }

    #[test]
    fn test_tm_kill_thread() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        tm.kill_thread(pid, tid, SignalType::Kill).unwrap();
        let t = tm.get_thread(pid, tid).unwrap();
        assert!(t.is_exited());
    }

    #[test]
    fn test_tm_block_unblock() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        tm.block_thread(pid, tid, BlockReason::IoWait).unwrap();
        let t = tm.get_thread(pid, tid).unwrap();
        assert_eq!(t.state, ThreadState::Blocked(BlockReason::IoWait));
        tm.unblock_thread(pid, tid).unwrap();
        let t = tm.get_thread(pid, tid).unwrap();
        assert_eq!(t.state, ThreadState::Ready);
    }

    #[test]
    fn test_tm_futex_wait_wake() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        let waited = tm.futex_wait(pid, tid, 0x1000, 0, 0).unwrap();
        assert!(waited);
        let t = tm.get_thread(pid, tid).unwrap();
        assert_eq!(t.state, ThreadState::Blocked(BlockReason::Futex));
        let woken = tm.futex_wake(0x1000, 1).unwrap();
        assert_eq!(woken, 1);
    }

    #[test]
    fn test_tm_reap_exited() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        tm.exit_thread(pid, tid, ExitCode::Success).unwrap();
        let reaped = tm.reap_exited(pid);
        assert_eq!(reaped.len(), 1);
    }

    #[test]
    fn test_tm_tick() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        tm.tick(1_000_000);
        let t = tm.get_thread(pid, Tid(pid)).unwrap();
        assert_eq!(t.cpu_time_ns, 1_000_000);
    }

    #[test]
    fn test_tm_stats() {
        let mut tm = ThreadManager::new();
        let p1 = tm.create_process(0x7fff_0000, 0x401000);
        let p2 = tm.create_process(0x7fff_0000, 0x401000);
        tm.create_thread(p1, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        let stats = tm.stats();
        assert_eq!(stats.processes, 2);
        assert_eq!(stats.threads, 3);
        assert_eq!(stats.alive, 3);
    }

    #[test]
    fn test_tm_destroy_process() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        tm.destroy_process(pid).unwrap();
        assert_eq!(tm.process_count(), 0);
    }

    #[test]
    fn test_tm_not_found() {
        let mut tm = ThreadManager::new();
        assert_eq!(tm.create_thread(999, 0, 0, 0, 0, CloneFlags::default_thread()), Err(ThreadError::ProcessNotFound));
    }

    // ── FutexMutex Tests ──
    #[test]
    fn test_futex_mutex_lock_unlock() {
        let mut m = FutexMutex::new(0x1000);
        assert!(m.try_lock());
        assert!(m.is_locked());
        assert!(!m.try_lock());
        let was_contested = m.unlock();
        assert!(!was_contested);
        assert!(!m.is_locked());
    }

    #[test]
    fn test_futex_mutex_contested() {
        let mut m = FutexMutex::new(0x1000);
        m.try_lock();
        let needs_wait = m.lock();
        assert!(!needs_wait);
        assert_eq!(m.state, FutexMutex::CONTESTED);
        let was_contested = m.unlock();
        assert!(was_contested);
    }

    // ── Condvar Tests ──
    #[test]
    fn test_condvar_signal() {
        let mut cv = FutexCondvar::new(0x2000);
        let s1 = cv.signal();
        let s2 = cv.signal();
        assert_eq!(s1, 1);
        assert_eq!(s2, 2);
        assert_eq!(cv.current_seq(), 2);
    }

    #[test]
    fn test_condvar_broadcast() {
        let mut cv = FutexCondvar::new(0x2000);
        cv.broadcast();
        assert_eq!(cv.current_seq(), 1);
    }

    // ── Barrier Tests ──
    #[test]
    fn test_barrier_waiting() {
        let mut b = FutexBarrier::new(0x3000, 3);
        let r1 = b.arrive();
        assert!(matches!(r1, BarrierResult::Waiting(0)));
        let r2 = b.arrive();
        assert!(matches!(r2, BarrierResult::Waiting(0)));
        let r3 = b.arrive();
        assert!(matches!(r3, BarrierResult::Released(1)));
        assert!(b.is_complete());
    }

    #[test]
    fn test_barrier_generation() {
        let mut b = FutexBarrier::new(0x3000, 2);
        b.arrive();
        b.arrive();
        assert_eq!(b.generation, 1);
        b.arrive();
        b.arrive();
        assert_eq!(b.generation, 2);
    }

    // ── RwLock Tests ──
    #[test]
    fn test_rwlock_read_lock() {
        let mut l = FutexRwLock::new();
        assert!(l.try_read_lock());
        assert!(l.try_read_lock());
        assert_eq!(l.reader_count(), 2);
        l.read_unlock();
        assert_eq!(l.reader_count(), 1);
    }

    #[test]
    fn test_rwlock_write_lock() {
        let mut l = FutexRwLock::new();
        assert!(l.try_write_lock());
        assert!(l.is_write_locked());
        assert!(!l.try_read_lock());
        assert!(!l.try_write_lock());
        l.write_unlock();
        assert!(!l.is_write_locked());
    }

    #[test]
    fn test_rwlock_no_write_with_readers() {
        let mut l = FutexRwLock::new();
        l.try_read_lock();
        assert!(!l.try_write_lock());
    }

    #[test]
    fn test_rwlock_no_read_with_writer() {
        let mut l = FutexRwLock::new();
        l.try_write_lock();
        assert!(!l.try_read_lock());
    }

    // ── SavedRegs Tests ──
    #[test]
    fn test_saved_regs_default() {
        let r = SavedRegs::new();
        assert_eq!(r.rsp, 0);
        assert_eq!(r.rip, 0);
        assert_eq!(r.rflags, 0);
    }

    #[test]
    fn test_saved_regs_with_rsp_rip() {
        let r = SavedRegs::new_with_rsp_rip(0x7fff_0000, 0x401000);
        assert_eq!(r.rsp, 0x7fff_0000);
        assert_eq!(r.rip, 0x401000);
        assert_eq!(r.rflags, 0x202);
    }

    // ── ExitCode Tests ──
    #[test]
    fn test_exit_code_success() {
        assert!(ExitCode::Success.is_success());
        assert_eq!(ExitCode::Success.code(), 0);
    }

    #[test]
    fn test_exit_code_error() {
        let e = ExitCode::Error(99);
        assert!(!e.is_success());
        assert_eq!(e.code(), 99);
    }

    #[test]
    fn test_exit_code_killed() {
        let e = ExitCode::Killed(SignalType::Term);
        assert!(!e.is_success());
        assert_eq!(e.code(), -1);
    }

    // ── Integration Tests ──
    #[test]
    fn test_full_thread_lifecycle() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 42, CloneFlags::default_thread()).unwrap();
        
        // Thread is ready
        assert!(tm.get_thread(pid, tid).unwrap().is_alive());
        
        // Thread blocks on futex
        tm.futex_wait(pid, tid, 0x5000, 0, 0).unwrap();
        assert_eq!(tm.get_thread(pid, tid).unwrap().state, ThreadState::Blocked(BlockReason::Futex));
        
        // Wake the thread
        tm.futex_wake(0x5000, 1).unwrap();
        
        // Thread exits
        tm.exit_thread(pid, tid, ExitCode::Success).unwrap();
        assert!(tm.get_thread(pid, tid).unwrap().is_exited());
        
        // Join
        let code = tm.join_thread(pid, tid).unwrap();
        assert!(code.is_success());
        
        // Reap
        let reaped = tm.reap_exited(pid);
        assert_eq!(reaped.len(), 1);
    }

    #[test]
    fn test_multi_process_multi_thread() {
        let mut tm = ThreadManager::new();
        let p1 = tm.create_process(0x7fff_0000, 0x401000);
        let p2 = tm.create_process(0x7fff_0000, 0x401000);
        
        tm.create_thread(p1, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        tm.create_thread(p1, 0x8000_0000, 4096, 0x403000, 0, CloneFlags::default_thread()).unwrap();
        tm.create_thread(p2, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        
        let stats = tm.stats();
        assert_eq!(stats.processes, 2);
        assert_eq!(stats.threads, 5); // 2 main + 3 created
        assert_eq!(stats.alive, 5);
    }

    #[test]
    fn test_futex_cross_process() {
        let mut tm = ThreadManager::new();
        let p1 = tm.create_process(0x7fff_0000, 0x401000);
        let p2 = tm.create_process(0x7fff_0000, 0x401000);
        
        let t1 = tm.create_thread(p1, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        let t2 = tm.create_thread(p2, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        
        // Both wait on same shared futex addr
        tm.futex_wait(p1, t1, 0xSHARED, 0, 0).unwrap();
        tm.futex_wait(p2, t2, 0xSHARED, 0, 0).unwrap();
        
        assert_eq!(tm.futex.waiting_count(), 2);
        
        // Wake all
        let woken = tm.futex_wake(0xSHARED, 10).unwrap();
        assert_eq!(woken, 2);
    }

    #[test]
    fn test_destroy_clears_futex() {
        let mut tm = ThreadManager::new();
        let pid = tm.create_process(0x7fff_0000, 0x401000);
        let tid = tm.create_thread(pid, 0x8000_0000, 4096, 0x402000, 0, CloneFlags::default_thread()).unwrap();
        tm.futex_wait(pid, tid, 0x6000, 0, 0).unwrap();
        assert_eq!(tm.futex.waiting_count(), 1);
        tm.destroy_process(pid).unwrap();
        assert_eq!(tm.futex.waiting_count(), 0);
    }
}
