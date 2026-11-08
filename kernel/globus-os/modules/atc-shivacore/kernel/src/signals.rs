// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 42: Advanced Signal Handling + POSIX Real-Time Signals
// Copyright (c) 2026 Michael Wroblewski. All rights reserved.
//
// Erweitert die Basis-Signal-Infrastruktur (K-Sprint 31, elf_loader.rs) um:
//   1. REAL-TIME SIGNALS — SIGRTMIN(32) bis SIGRTMAX(63), Queuing (kein Coalescing)
//   2. SIGNAL GROUPS — killpg, Prozessgruppen-Signale
//   3. SIGNAL TARGETING — Process, Process Group, Container, Broadcast
//   4. SIGNAL INFO — siginfo_t-ähnliche Struktur mit Sender, Code, Value
//   5. SIGNAL TIMERS — Interval-Timer (ITIMER_REAL, ITIMER_VIRTUAL, ITIMER_PROF)
//   6. SIGNAL ALTSTACK — sigaltstack für Stack-Overflow-Handling
//   7. SIGNAL COALESCING — Standard-Signale verschmelzen, RT-Signale queue
//   8. CONTAINER FORWARDING — Signal-Weiterleitung zwischen Container-Grenzen
//   9. SIGNAL AUDIT — Logging aller Signal-Sendungen für Security
//  10. SIGNAL PRIORITY — Delivery-Reihenfolge nach Dringlichkeit

#![allow(dead_code)]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

// ════════════════════════════════════════════════════════════════
//  CONSTANTS
// ════════════════════════════════════════════════════════════════

const SIGRTMIN: u8 = 32;
const SIGRTMAX: u8 = 63;
const MAX_STANDARD_SIGNALS: usize = 31;
const MAX_ALL_SIGNALS: usize = 64;
const MAX_PENDING_PER_PROCESS: usize = 256;
const MAX_TIMER_COUNT: usize = 32;
const MAX_ALTSTACK_SIZE: usize = 1024 * 1024;  // 1 MB
const MIN_ALTSTACK_SIZE: usize = 4096;         // 4 KB
const MAX_AUDIT_ENTRIES: usize = 4096;

// ════════════════════════════════════════════════════════════════
//  FULL POSIX SIGNAL SET (1-63)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FullSignal {
    // Standard POSIX signals (1-31)
    Sighup = 1,      // Hangup (terminal closed)
    Sigint = 2,      // Interrupt (Ctrl+C)
    Sigquit = 3,     // Quit (Ctrl+\)
    Sigill = 4,      // Illegal instruction
    Sigtrap = 5,     // Trace trap (debugger)
    Sigabrt = 6,     // Abort (abort())
    Sigbus = 7,      // Bus error (memory alignment)
    Sigfpe = 8,      // Floating-point exception
    Sigkill = 9,     // Kill (unblockable)
    Sigusr1 = 10,    // User-defined 1
    Sigsegv = 11,    // Segmentation fault
    Sigusr2 = 12,    // User-defined 2
    Sigpipe = 13,    // Broken pipe
    Sigalrm = 14,    // Alarm clock
    Sigterm = 15,    // Termination
    Sigstkflt = 16,  // Stack fault (coprocessor)
    Sigchld = 17,    // Child status changed
    Sigcont = 18,    // Continue (if stopped)
    Sigstop = 19,    // Stop (unblockable)
    Sigtstp = 20,    // Terminal stop (Ctrl+Z)
    Sigttin = 21,    // Background read
    Sigttou = 22,    // Background write
    Sigurg = 23,     // Urgent data on socket
    Sigxcpu = 24,    // CPU limit exceeded
    Sigxfsz = 25,    // File size limit exceeded
    Sigvtalrm = 26,  // Virtual timer alarm
    Sigprof = 27,    // Profiling timer alarm
    Sigwinch = 28,   // Window size change
    Sigio = 29,      // I/O now possible (async I/O)
    Sigpwr = 30,     // Power failure
    Sigsys = 31,     // Bad system call

    // Real-time signals (32-63)
    SigrtMin = 32,
    Sigrt1 = 33,
    Sigrt2 = 34,
    Sigrt3 = 35,
    Sigrt4 = 36,
    Sigrt5 = 37,
    Sigrt6 = 38,
    Sigrt7 = 39,
    Sigrt8 = 40,
    Sigrt9 = 41,
    Sigrt10 = 42,
    Sigrt11 = 43,
    Sigrt12 = 44,
    Sigrt13 = 45,
    Sigrt14 = 46,
    Sigrt15 = 47,
    Sigrt16 = 48,
    Sigrt17 = 49,
    Sigrt18 = 50,
    Sigrt19 = 51,
    Sigrt20 = 52,
    Sigrt21 = 53,
    Sigrt22 = 54,
    Sigrt23 = 55,
    Sigrt24 = 56,
    Sigrt25 = 57,
    Sigrt26 = 58,
    Sigrt27 = 59,
    Sigrt28 = 60,
    Sigrt29 = 61,
    Sigrt30 = 62,
    SigrtMax = 63,
}

impl FullSignal {
    pub fn from_u8(n: u8) -> Option<Self> {
        if n == 0 || n > SIGRTMAX { return None; }
        // Use unsafe transmute alternative — match
        match n {
            1 => Some(FullSignal::Sighup),
            2 => Some(FullSignal::Sigint),
            3 => Some(FullSignal::Sigquit),
            4 => Some(FullSignal::Sigill),
            5 => Some(FullSignal::Sigtrap),
            6 => Some(FullSignal::Sigabrt),
            7 => Some(FullSignal::Sigbus),
            8 => Some(FullSignal::Sigfpe),
            9 => Some(FullSignal::Sigkill),
            10 => Some(FullSignal::Sigusr1),
            11 => Some(FullSignal::Sigsegv),
            12 => Some(FullSignal::Sigusr2),
            13 => Some(FullSignal::Sigpipe),
            14 => Some(FullSignal::Sigalrm),
            15 => Some(FullSignal::Sigterm),
            16 => Some(FullSignal::Sigstkflt),
            17 => Some(FullSignal::Sigchld),
            18 => Some(FullSignal::Sigcont),
            19 => Some(FullSignal::Sigstop),
            20 => Some(FullSignal::Sigtstp),
            21 => Some(FullSignal::Sigttin),
            22 => Some(FullSignal::Sigtou),
            23 => Some(FullSignal::Sigurg),
            24 => Some(FullSignal::Sigxcpu),
            25 => Some(FullSignal::Sigxfsz),
            26 => Some(FullSignal::Sigvtalrm),
            27 => Some(FullSignal::Sigprof),
            28 => Some(FullSignal::Sigwinch),
            29 => Some(FullSignal::Sigio),
            30 => Some(FullSignal::Sigpwr),
            31 => Some(FullSignal::Sigsys),
            32 => Some(FullSignal::SigrtMin),
            33 => Some(FullSignal::Sigrt1),
            34 => Some(FullSignal::Sigrt2),
            35 => Some(FullSignal::Sigrt3),
            36 => Some(FullSignal::Sigrt4),
            37 => Some(FullSignal::Sigrt5),
            38 => Some(FullSignal::Sigrt6),
            39 => Some(FullSignal::Sigrt7),
            40 => Some(FullSignal::Sigrt8),
            41 => Some(FullSignal::Sigrt9),
            42 => Some(FullSignal::Sigrt10),
            43 => Some(FullSignal::Sigrt11),
            44 => Some(FullSignal::Sigrt12),
            45 => Some(FullSignal::Sigrt13),
            46 => Some(FullSignal::Sigrt14),
            47 => Some(FullSignal::Sigrt15),
            48 => Some(FullSignal::Sigrt16),
            49 => Some(FullSignal::Sigrt17),
            50 => Some(FullSignal::Sigrt18),
            51 => Some(FullSignal::Sigrt19),
            52 => Some(FullSignal::Sigrt20),
            53 => Some(FullSignal::Sigrt21),
            54 => Some(FullSignal::Sigrt22),
            55 => Some(FullSignal::Sigrt23),
            56 => Some(FullSignal::Sigrt24),
            57 => Some(FullSignal::Sigrt25),
            58 => Some(FullSignal::Sigrt26),
            59 => Some(FullSignal::Sigrt27),
            60 => Some(FullSignal::Sigrt28),
            61 => Some(FullSignal::Sigrt29),
            62 => Some(FullSignal::Sigrt30),
            63 => Some(FullSignal::SigrtMax),
            _ => None,
        }
    }

    pub fn number(&self) -> u8 { *self as u8 }

    pub fn is_standard(&self) -> bool { self.number() <= MAX_STANDARD_SIGNALS as u8 }
    pub fn is_realtime(&self) -> bool { self.number() >= SIGRTMIN }
    pub fn is_unblockable(&self) -> bool { matches!(self, FullSignal::Sigkill | FullSignal::Sigstop) }
    pub fn is_fatal(&self) -> bool {
        matches!(self, FullSignal::Sighup | FullSignal::Sigint | FullSignal::Sigkill |
                      FullSignal::Sigterm | FullSignal::Sigsegv | FullSignal::Sigquit |
                      FullSignal::Sigill | FullSignal::Sigabrt | FullSignal::Sigbus |
                      FullSignal::Sigfpe | FullSignal::Sigsys | FullSignal::Sigxcpu | FullSignal::Sigxfsz)
    }

    pub fn name(&self) -> &'static str {
        match self {
            FullSignal::Sighup => "SIGHUP", FullSignal::Sigint => "SIGINT",
            FullSignal::Sigquit => "SIGQUIT", FullSignal::Sigill => "SIGILL",
            FullSignal::Sigtrap => "SIGTRAP", FullSignal::Sigabrt => "SIGABRT",
            FullSignal::Sigbus => "SIGBUS", FullSignal::Sigfpe => "SIGFPE",
            FullSignal::Sigkill => "SIGKILL", FullSignal::Sigusr1 => "SIGUSR1",
            FullSignal::Sigsegv => "SIGSEGV", FullSignal::Sigusr2 => "SIGUSR2",
            FullSignal::Sigpipe => "SIGPIPE", FullSignal::Sigalrm => "SIGALRM",
            FullSignal::Sigterm => "SIGTERM", FullSignal::Sigstkflt => "SIGSTKFLT",
            FullSignal::Sigchld => "SIGCHLD", FullSignal::Sigcont => "SIGCONT",
            FullSignal::Sigstop => "SIGSTOP", FullSignal::Sigtstp => "SIGTSTP",
            FullSignal::Sigttin => "SIGTTIN", FullSignal::Sigtou => "SIGTTOU",
            FullSignal::Sigurg => "SIGURG", FullSignal::Sigxcpu => "SIGXCPU",
            FullSignal::Sigxfsz => "SIGXFSZ", FullSignal::Sigvtalrm => "SIGVTALRM",
            FullSignal::Sigprof => "SIGPROF", FullSignal::Sigwinch => "SIGWINCH",
            FullSignal::Sigio => "SIGIO", FullSignal::Sigpwr => "SIGPWR",
            FullSignal::Sigsys => "SIGSYS",
            FullSignal::SigrtMin => "SIGRTMIN", FullSignal::Sigrt1 => "SIGRT1",
            FullSignal::Sigrt2 => "SIGRT2", FullSignal::Sigrt3 => "SIGRT3",
            FullSignal::Sigrt4 => "SIGRT4", FullSignal::Sigrt5 => "SIGRT5",
            FullSignal::Sigrt6 => "SIGRT6", FullSignal::Sigrt7 => "SIGRT7",
            FullSignal::Sigrt8 => "SIGRT8", FullSignal::Sigrt9 => "SIGRT9",
            FullSignal::Sigrt10 => "SIGRT10", FullSignal::Sigrt11 => "SIGRT11",
            FullSignal::Sigrt12 => "SIGRT12", FullSignal::Sigrt13 => "SIGRT13",
            FullSignal::Sigrt14 => "SIGRT14", FullSignal::Sigrt15 => "SIGRT15",
            FullSignal::Sigrt16 => "SIGRT16", FullSignal::Sigrt17 => "SIGRT17",
            FullSignal::Sigrt18 => "SIGRT18", FullSignal::Sigrt19 => "SIGRT19",
            FullSignal::Sigrt20 => "SIGRT20", FullSignal::Sigrt21 => "SIGRT21",
            FullSignal::Sigrt22 => "SIGRT22", FullSignal::Sigrt23 => "SIGRT23",
            FullSignal::Sigrt24 => "SIGRT24", FullSignal::Sigrt25 => "SIGRT25",
            FullSignal::Sigrt26 => "SIGRT26", FullSignal::Sigrt27 => "SIGRT27",
            FullSignal::Sigrt28 => "SIGRT28", FullSignal::Sigrt29 => "SIGRT29",
            FullSignal::Sigrt30 => "SIGRT30", FullSignal::SigrtMax => "SIGRTMAX",
        }
    }

    pub fn default_action(&self) -> SignalAction {
        match self {
            FullSignal::Sighup | FullSignal::Sigint | FullSignal::Sigkill |
            FullSignal::Sigterm | FullSignal::Sigpipe => SignalAction::Terminate,
            FullSignal::Sigquit | FullSignal::Sigill | FullSignal::Sigabrt |
            FullSignal::Sigbus | FullSignal::Sigfpe | FullSignal::Sigsegv |
            FullSignal::Sigsys | FullSignal::Sigxcpu | FullSignal::Sigxfsz => SignalAction::TerminateCore,
            FullSignal::Sigstop | FullSignal::Sigtstp | FullSignal::Sigttin |
            FullSignal::Sigtou => SignalAction::Stop,
            FullSignal::Sigcont => SignalAction::Continue,
            FullSignal::Sigchld | FullSignal::Sigurg | FullSignal::Sigwinch |
            FullSignal::Sigio | FullSignal::Sigpwr => SignalAction::Ignore,
            FullSignal::Sigalrm => SignalAction::Terminate,
            FullSignal::Sigvtalrm | FullSignal::Sigprof => SignalAction::Terminate,
            FullSignal::Sigtrap | FullSignal::Sigstkflt => SignalAction::Terminate,
            _ => {
                if self.is_realtime() { SignalAction::Terminate } else { SignalAction::Ignore }
            }
        }
    }

    /// Priority for delivery ordering (lower = higher priority)
    pub fn delivery_priority(&self) -> u8 {
        match self {
            FullSignal::Sigkill => 0,      // Highest — unblockable, immediate
            FullSignal::Sigstop => 1,
            FullSignal::Sigsegv | FullSignal::Sigill | FullSignal::Sigbus |
            FullSignal::Sigfpe | FullSignal::Sigabrt => 2,  // Fatal hardware
            FullSignal::Sigterm => 3,
            FullSignal::Sigint | FullSignal::Sighup => 4,
            FullSignal::Sigquit => 5,
            FullSignal::Sigalrm | FullSignal::Sigvtalrm | FullSignal::Sigprof => 6,
            FullSignal::Sigchld => 7,
            FullSignal::Sigcont => 8,
            FullSignal::Sigtstp | FullSignal::Sigttin | FullSignal::Sigtou => 9,
            FullSignal::Sigusr1 | FullSignal::Sigusr2 => 10,
            FullSignal::Sigpipe | FullSignal::Sigurg | FullSignal::Sigwinch |
            FullSignal::Sigio | FullSignal::Sigpwr | FullSignal::Sigstkflt |
            FullSignal::Sigsys | FullSignal::Sigxcpu | FullSignal::Sigxfsz => 11,
            _ => {
                if self.is_realtime() { 12 + (self.number() - SIGRTMIN) / 4 } else { 13 }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  SIGNAL ACTION (default behavior)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalAction {
    Terminate,       // Exit process
    TerminateCore,   // Exit + core dump
    Stop,            // Stop process (SIGSTOP-like)
    Continue,        // Continue if stopped
    Ignore,          // Discard signal
}

// ════════════════════════════════════════════════════════════════
//  SIGNAL DISPOSITION (handler configuration)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalDisposition {
    Default,
    Ignore,
    Catch { handler_addr: u64, flags: SignalHandlerFlags, mask: u64 },
}

impl Default for SignalDisposition {
    fn default() -> Self { SignalDisposition::Default }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SignalHandlerFlags {
    pub restart_syscall: bool,    // SA_RESTART
    pub oneshot: bool,             // SA_RESETHAND — reset to default after delivery
    pub nocldstop: bool,           // SA_NOCLDSTOP — don't send SIGCHLD on stop
    pub nocldwait: bool,           // SA_NOCLDWAIT — don't create zombies
    pub onstack: bool,             // SA_ONSTACK — use altstack
    pub siginfo: bool,              // SA_SIGINFO — use siginfo_t handler
}

impl SignalHandlerFlags {
    pub fn none() -> Self { Self::default() }
    pub fn restart() -> Self { Self { restart_syscall: true, ..Self::default() } }
}

// ════════════════════════════════════════════════════════════════
//  SIGNAL INFO (siginfo_t equivalent)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Default)]
pub struct SignalInfo {
    pub signo: u8,
    pub code: SignalCode,
    pub sender_pid: Option<u32>,
    pub sender_uid: Option<u32>,
    pub value: u64,
    pub status: i32,
    pub addr: u64,
    pub timestamp: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SignalCode {
    #[default]
    User,           // kill(2) from user
    Kernel,          // Sent by kernel
    Timer,           // POSIX timer expired
    Queue,           // sigqueue(2)
    MesgQ,           // Message queue state changed
    Io,              // Async I/O completed
    Tsync,           // Thread synchronization (futex)
    DetThread,      // Thread completed
    Child,           // Child process status
    Fault,           // Hardware fault (SIGSEGV, SIGBUS, etc.)
    Trap,            // Debugger trap
    Regv,            // Region violation
    Unk,             // Unknown
}

impl SignalCode {
    pub fn name(&self) -> &'static str {
        match self {
            SignalCode::User => "SI_USER", SignalCode::Kernel => "SI_KERNEL",
            SignalCode::Timer => "SI_TIMER", SignalCode::Queue => "SI_QUEUE",
            SignalCode::MesgQ => "SI_MESGQ", SignalCode::Io => "SI_IO",
            SignalCode::Tsync => "SI_TSYNC", SignalCode::DetThread => "SI_DETTHREAD",
            SignalCode::Child => "SI_CHILD", SignalCode::Fault => "SI_FAULT",
            SignalCode::Trap => "SI_TRAP", SignalCode::Regv => "SI_REGV",
            SignalCode::Unk => "SI_UNK",
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  PENDING SIGNAL ENTRY
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct PendingEntry {
    pub signal: FullSignal,
    pub info: SignalInfo,
}

// ════════════════════════════════════════════════════════════════
//  SIGNAL MASK (blocked signals)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SignalMask {
    bits: u64,   // 64 bits for 64 signals
}

impl SignalMask {
    pub fn empty() -> Self { Self::default() }
    pub fn full() -> Self { Self { bits: !0 } }

    pub fn add(&mut self, sig: FullSignal) {
        self.bits |= 1u64 << (sig.number() as u64 - 1);
    }

    pub fn remove(&mut self, sig: FullSignal) {
        self.bits &= !(1u64 << (sig.number() as u64 - 1));
    }

    pub fn contains(&self, sig: FullSignal) -> bool {
        (self.bits & (1u64 << (sig.number() as u64 - 1))) != 0
    }

    pub fn is_empty(&self) -> bool { self.bits == 0 }
    pub fn count(&self) -> u8 { self.bits.count_ones() as u8 }

    pub fn union(&self, other: &SignalMask) -> SignalMask {
        SignalMask { bits: self.bits | other.bits }
    }

    pub fn intersection(&self, other: &SignalMask) -> SignalMask {
        SignalMask { bits: self.bits & other.bits }
    }

    pub fn difference(&self, other: &SignalMask) -> SignalMask {
        SignalMask { bits: self.bits & !other.bits }
    }
}

// ════════════════════════════════════════════════════════════════
//  ALTERNATE SIGNAL STACK (sigaltstack)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AltStack {
    pub base_addr: u64,
    pub size: u64,
    pub flags: AltStackFlags,
}

impl AltStack {
    pub fn new(base: u64, size: u64) -> Option<Self> {
        if size < MIN_ALTSTACK_SIZE as u64 || size > MAX_ALTSTACK_SIZE as u64 {
            return None;
        }
        Some(Self { base_addr: base, size, flags: AltStackFlags::default() })
    }

    pub fn is_valid(&self) -> bool { !self.flags.is_disabled() }
    pub fn top(&self) -> u64 { self.base_addr + self.size }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AltStackFlags {
    pub onstack: bool,     // Currently executing on altstack
    pub disabled: bool,    // SS_DISABLE
}

impl AltStackFlags {
    pub fn is_disabled(&self) -> bool { self.disabled }
}

// ════════════════════════════════════════════════════════════════
//  INTERVAL TIMERS (setitimer/getitimer)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerType {
    Real,       // ITIMER_REAL — decrements real time, sends SIGALRM
    Virtual,     // ITIMER_VIRTUAL — decrements user CPU time, sends SIGVTALRM
    Prof,        // ITIMER_PROF — decrements user+sys CPU time, sends SIGPROF
}

impl TimerType {
    pub fn signal(&self) -> FullSignal {
        match self {
            TimerType::Real => FullSignal::Sigalrm,
            TimerType::Virtual => FullSignal::Sigvtalrm,
            TimerType::Prof => FullSignal::Sigprof,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TimerType::Real => "ITIMER_REAL",
            TimerType::Virtual => "ITIMER_VIRTUAL",
            TimerType::Prof => "ITIMER_PROF",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeVal {
    pub seconds: u64,
    pub microseconds: u64,
}

impl TimeVal {
    pub fn zero() -> Self { Self { seconds: 0, microseconds: 0 } }
    pub fn from_ms(ms: u64) -> Self { Self { seconds: ms / 1000, microseconds: (ms % 1000) * 1000 } }
    pub fn to_us(&self) -> u64 { self.seconds * 1_000_000 + self.microseconds }
    pub fn is_zero(&self) -> bool { self.seconds == 0 && self.microseconds == 0 }
}

#[derive(Clone, Debug)]
pub struct IntervalTimer {
    pub timer_type: TimerType,
    pub initial: TimeVal,
    pub interval: TimeVal,
    pub remaining: TimeVal,
    pub overrun: u32,
    pub active: bool,
}

impl IntervalTimer {
    pub fn new(timer_type: TimerType) -> Self {
        Self {
            timer_type,
            initial: TimeVal::zero(),
            interval: TimeVal::zero(),
            remaining: TimeVal::zero(),
            overrun: 0,
            active: false,
        }
    }

    pub fn set(&mut self, initial: TimeVal, interval: TimeVal) {
        self.initial = initial;
        self.interval = interval;
        self.remaining = initial;
        self.active = !initial.is_zero();
        self.overrun = 0;
    }

    pub fn cancel(&mut self) {
        self.active = false;
        self.remaining = TimeVal::zero();
    }

    pub fn tick(&mut self, elapsed_us: u64) -> bool {
        if !self.active { return false; }
        let remaining_us = self.remaining.to_us();
        if elapsed_us >= remaining_us {
            self.overrun += 1;
            if !self.interval.is_zero() {
                self.remaining = self.interval;
            } else {
                self.active = false;
            }
            return true;  // Timer fired
        } else {
            let new_remaining = remaining_us - elapsed_us;
            self.remaining = TimeVal {
                seconds: new_remaining / 1_000_000,
                microseconds: new_remaining % 1_000_000,
            };
            return false;
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  SIGNAL AUDIT LOG ENTRY
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SignalAuditEntry {
    pub timestamp: u64,
    pub sender_pid: Option<u32>,
    pub target_pid: u32,
    pub signal: FullSignal,
    pub code: SignalCode,
    pub target_type: SignalTargetType,
    pub result: SignalSendResult,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalTargetType {
    Process,
    ProcessGroup,
    Container,
    Broadcast,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalSendResult {
    Delivered,
    Queued,
    Blocked,
    Coalesced,   // Merged with existing pending standard signal
    NoTarget,
    PermissionDenied,
}

// ════════════════════════════════════════════════════════════════
//  SIGNAL RESOLUTION (what to do when delivered)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalResolution {
    Terminate(i32),       // exit code (0 = normal, 1 = core)
    Stop,
    Continue,
    Ignore,
    CallHandler { addr: u64, use_altstack: bool, flags: SignalHandlerFlags },
}

// ════════════════════════════════════════════════════════════════
//  PER-PROCESS SIGNAL STATE
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ProcessSignalState {
    pub pid: u32,
    pub dispositions: [SignalDisposition; MAX_ALL_SIGNALS],
    pub blocked_mask: SignalMask,
    pub saved_mask: SignalMask,        // Saved during handler execution
    pub pending: Vec<PendingEntry>,
    pub standard_pending: u64,          // Bitmask for standard signals (coalescing)
    pub altstack: Option<AltStack>,
    pub timers: [IntervalTimer; 3],    // Real, Virtual, Prof
    pub in_handler: bool,
    pub signals_received: u64,
    pub signals_delivered: u64,
    pub signals_dropped: u64,
    pub signals_coalesced: u64,
}

impl ProcessSignalState {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            dispositions: [SignalDisposition::Default; MAX_ALL_SIGNALS],
            blocked_mask: SignalMask::empty(),
            saved_mask: SignalMask::empty(),
            pending: Vec::new(),
            standard_pending: 0,
            altstack: None,
            timers: [
                IntervalTimer::new(TimerType::Real),
                IntervalTimer::new(TimerType::Virtual),
                IntervalTimer::new(TimerType::Prof),
            ],
            in_handler: false,
            signals_received: 0,
            signals_delivered: 0,
            signals_dropped: 0,
            signals_coalesced: 0,
        }
    }

    pub fn set_disposition(&mut self, sig: FullSignal, disp: SignalDisposition) -> bool {
        if sig.is_unblockable() { return false; }
        self.dispositions[sig.number() as usize - 1] = disp;
        true
    }

    pub fn get_disposition(&self, sig: FullSignal) -> SignalDisposition {
        self.dispositions[sig.number() as usize - 1]
    }

    pub fn block_signal(&mut self, sig: FullSignal) {
        if sig.is_unblockable() { return; }
        self.blocked_mask.add(sig);
    }

    pub fn unblock_signal(&mut self, sig: FullSignal) {
        self.blocked_mask.remove(sig);
    }

    pub fn is_blocked(&self, sig: FullSignal) -> bool {
        if sig.is_unblockable() { return false; }
        self.blocked_mask.contains(sig)
    }

    pub fn set_altstack(&mut self, stack: AltStack) {
        self.altstack = Some(stack);
    }

    pub fn get_altstack(&self) -> Option<&AltStack> {
        self.altstack.as_ref()
    }

    pub fn has_pending_standard(&self, sig: FullSignal) -> bool {
        let bit = 1u64 << (sig.number() as u64 - 1);
        (self.standard_pending & bit) != 0
    }

    pub fn mark_pending_standard(&mut self, sig: FullSignal) -> bool {
        let bit = 1u64 << (sig.number() as u64 - 1);
        if (self.standard_pending & bit) != 0 {
            return false;  // Already pending — coalesced
        }
        self.standard_pending |= bit;
        true
    }

    pub fn clear_pending_standard(&mut self, sig: FullSignal) {
        let bit = 1u64 << (sig.number() as u64 - 1);
        self.standard_pending &= !bit;
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub fn is_full(&self) -> bool {
        self.pending.len() >= MAX_PENDING_PER_PROCESS
    }

    pub fn get_timer_mut(&mut self, timer_type: TimerType) -> &mut IntervalTimer {
        match timer_type {
            TimerType::Real => &mut self.timers[0],
            TimerType::Virtual => &mut self.timers[1],
            TimerType::Prof => &mut self.timers[2],
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  ADVANCED SIGNAL MANAGER
// ════════════════════════════════════════════════════════════════

pub struct AdvancedSignalManager {
    processes: BTreeMap<u32, ProcessSignalState>,
    process_groups: BTreeMap<u32, Vec<u32>>,  // pgid → [pid]
    container_pids: BTreeMap<u32, Vec<u32>>,    // container_id → [pid]
    audit_log: Mutex<Vec<SignalAuditEntry>>,
    total_sent: u64,
    total_delivered: u64,
    total_coalesced: u64,
    total_dropped: u64,
}

impl AdvancedSignalManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            process_groups: BTreeMap::new(),
            container_pids: BTreeMap::new(),
            audit_log: Mutex::new(Vec::new()),
            total_sent: 0,
            total_delivered: 0,
            total_coalesced: 0,
            total_dropped: 0,
        }
    }

    // ── Process Registration ──────────────────────────────────

    pub fn register(&mut self, pid: u32) {
        self.processes.insert(pid, ProcessSignalState::new(pid));
    }

    pub fn unregister(&mut self, pid: u32) {
        self.processes.remove(&pid);
        // Remove from all process groups
        for (_, members) in self.process_groups.iter_mut() {
            members.retain(|&p| p != pid);
        }
        // Remove from all containers
        for (_, pids) in self.container_pids.iter_mut() {
            pids.retain(|&p| p != pid);
        }
    }

    pub fn is_registered(&self, pid: u32) -> bool {
        self.processes.contains_key(&pid)
    }

    // ── Process Group Management ────────────────────────────

    pub fn create_process_group(&mut self, pgid: u32, leader_pid: u32) {
        self.process_groups.insert(pgid, vec![leader_pid]);
    }

    pub fn add_to_group(&mut self, pgid: u32, pid: u32) {
        self.process_groups.entry(pgid).or_insert_with(Vec::new).push(pid);
    }

    pub fn remove_from_group(&mut self, pgid: u32, pid: u32) -> bool {
        if let Some(members) = self.process_groups.get_mut(&pgid) {
            let before = members.len();
            members.retain(|&p| p != pid);
            return members.len() < before;
        }
        false
    }

    pub fn get_group_members(&self, pgid: u32) -> Option<&Vec<u32>> {
        self.process_groups.get(&pgid)
    }

    pub fn group_count(&self) -> usize { self.process_groups.len() }

    // ── Container Registration ──────────────────────────────

    pub fn register_container_pid(&mut self, container_id: u32, pid: u32) {
        self.container_pids.entry(container_id).or_insert_with(Vec::new).push(pid);
    }

    pub fn unregister_container_pid(&mut self, container_id: u32, pid: u32) {
        if let Some(pids) = self.container_pids.get_mut(&container_id) {
            pids.retain(|&p| p != pid);
        }
    }

    pub fn get_container_pids(&self, container_id: u32) -> Option<&Vec<u32>> {
        self.container_pids.get(&container_id)
    }

    // ── Signal Sending ────────────────────────────────────────

    pub fn send_to_process(&mut self, pid: u32, sig: FullSignal, info: SignalInfo) -> SignalSendResult {
        self.total_sent += 1;

        let mut info = info;
        info.signo = sig.number();
        if info.sender_pid.is_none() { info.sender_pid = Some(0); }

        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => {
                self.audit(SignalAuditEntry {
                    timestamp: 0, sender_pid: info.sender_pid, target_pid: pid,
                    signal: sig, code: info.code, target_type: SignalTargetType::Process,
                    result: SignalSendResult::NoTarget,
                });
                return SignalSendResult::NoTarget;
            }
        };

        state.signals_received += 1;

        // Check if blocked (but unblockable signals bypass)
        if state.is_blocked(sig) && !sig.is_unblockable() {
            // Still queue — will be delivered when unblocked
        }

        // Standard signals coalesce; RT signals queue
        if sig.is_standard() {
            if state.has_pending_standard(sig) {
                state.signals_coalesced += 1;
                self.total_coalesced += 1;
                self.audit(SignalAuditEntry {
                    timestamp: 0, sender_pid: info.sender_pid, target_pid: pid,
                    signal: sig, code: info.code, target_type: SignalTargetType::Process,
                    result: SignalSendResult::Coalesced,
                });
                return SignalSendResult::Coalesced;
            }
            state.mark_pending_standard(sig);
        }

        // Check queue capacity
        if state.is_full() {
            state.signals_dropped += 1;
            self.total_dropped += 1;
            if sig.is_standard() {
                state.clear_pending_standard(sig);
            }
            return SignalSendResult::NoTarget;  // Queue full
        }

        state.pending.push(PendingEntry { signal: sig, info });
        self.audit(SignalAuditEntry {
            timestamp: 0, sender_pid: info.sender_pid, target_pid: pid,
            signal: sig, code: info.code, target_type: SignalTargetType::Process,
            result: SignalSendResult::Queued,
        });

        SignalSendResult::Queued
    }

    pub fn send_to_group(&mut self, pgid: u32, sig: FullSignal, sender: Option<u32>) -> Vec<(u32, SignalSendResult)> {
        let members = match self.process_groups.get(&pgid) {
            Some(m) => m.clone(),
            None => return Vec::new(),
        };

        let mut results = Vec::new();
        for pid in members {
            let info = SignalInfo {
                signo: sig.number(),
                code: SignalCode::User,
                sender_pid: sender,
                sender_uid: None,
                value: 0, status: 0, addr: 0, timestamp: 0,
            };
            let result = self.send_to_process(pid, sig, info);
            results.push((pid, result));
        }
        results
    }

    pub fn send_to_container(&mut self, container_id: u32, sig: FullSignal, sender: Option<u32>) -> Vec<(u32, SignalSendResult)> {
        let pids = match self.container_pids.get(&container_id) {
            Some(p) => p.clone(),
            None => return Vec::new(),
        };

        let mut results = Vec::new();
        for pid in pids {
            let info = SignalInfo {
                signo: sig.number(),
                code: SignalCode::User,
                sender_pid: sender,
                sender_uid: None,
                value: 0, status: 0, addr: 0, timestamp: 0,
            };
            let result = self.send_to_process(pid, sig, info);
            results.push((pid, result));
        }
        results
    }

    pub fn broadcast(&mut self, sig: FullSignal, sender: Option<u32>) -> Vec<(u32, SignalSendResult)> {
        let pids: Vec<u32> = self.processes.keys().cloned().collect();
        let mut results = Vec::new();
        for pid in pids {
            if Some(pid) == sender { continue; }  // Don't send to self
            let info = SignalInfo {
                signo: sig.number(),
                code: SignalCode::Kernel,
                sender_pid: sender,
                sender_uid: None,
                value: 0, status: 0, addr: 0, timestamp: 0,
            };
            let result = self.send_to_process(pid, sig, info);
            results.push((pid, result));
        }
        results
    }

    // ── Signal Delivery ───────────────────────────────────────

    pub fn deliver(&mut self, pid: u32) -> Option<(FullSignal, SignalInfo, SignalResolution)> {
        let state = self.processes.get_mut(&pid)?;

        // Sort pending by priority (find highest priority non-blocked)
        let blocked = state.blocked_mask;
        let mut best_idx: Option<usize> = None;
        let mut best_priority = u8::MAX;

        for (i, entry) in state.pending.iter().enumerate() {
            let sig = entry.signal;
            // Unblockable signals bypass the mask
            if !sig.is_unblockable() && blocked.contains(sig) { continue; }
            let priority = sig.delivery_priority();
            if priority < best_priority {
                best_priority = priority;
                best_idx = Some(i);
            }
        }

        let idx = best_idx?;
        let entry = state.pending.remove(idx);
        let sig = entry.signal;
        let info = entry.info;

        // Clear standard pending bit
        if sig.is_standard() {
            state.clear_pending_standard(sig);
        }

        // Resolve disposition
        let disp = state.get_disposition(sig);
        let resolution = self.resolve(pid, sig, disp, &state);

        state.signals_delivered += 1;
        self.total_delivered += 1;

        // Set in_handler flag if calling handler
        if let SignalResolution::CallHandler { .. } = resolution {
            state.in_handler = true;
            state.saved_mask = state.blocked_mask;
            // Block the current signal during handler (SA_NODEFER would prevent this)
            state.blocked_mask.add(sig);
        }

        Some((sig, info, resolution))
    }

    fn resolve(&self, _pid: u32, sig: FullSignal, disp: SignalDisposition, state: &ProcessSignalState) -> SignalResolution {
        match disp {
            SignalDisposition::Default => match sig.default_action() {
                SignalAction::Terminate => SignalResolution::Terminate(0),
                SignalAction::TerminateCore => SignalResolution::Terminate(1),
                SignalAction::Stop => SignalResolution::Stop,
                SignalAction::Continue => SignalResolution::Continue,
                SignalAction::Ignore => SignalResolution::Ignore,
            },
            SignalDisposition::Ignore => SignalResolution::Ignore,
            SignalDisposition::Catch { handler_addr, flags, mask } => {
                let use_altstack = flags.onstack && state.altstack.as_ref().map(|s| s.is_valid()).unwrap_or(false);
                SignalResolution::CallHandler { addr: handler_addr, use_altstack, flags }
            }
        }
    }

    pub fn return_from_handler(&mut self, pid: u32) -> bool {
        let state = self.processes.get_mut(&pid)?;
        if !state.in_handler { return false; }
        state.in_handler = false;
        state.blocked_mask = state.saved_mask;
        true
    }

    pub fn has_pending(&self, pid: u32) -> bool {
        self.processes.get(&pid).map(|s| !s.pending.is_empty()).unwrap_or(false)
    }

    pub fn pending_count(&self, pid: u32) -> usize {
        self.processes.get(&pid).map(|s| s.pending.len()).unwrap_or(0)
    }

    // ── Handler Management ──────────────────────────────────

    pub fn set_handler(&mut self, pid: u32, sig: FullSignal, disp: SignalDisposition) -> bool {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        state.set_disposition(sig, disp)
    }

    pub fn get_handler(&self, pid: u32, sig: FullSignal) -> Option<SignalDisposition> {
        self.processes.get(&pid).map(|s| s.get_disposition(sig))
    }

    // ── Mask Management ─────────────────────────────────────

    pub fn block(&mut self, pid: u32, sig: FullSignal) -> bool {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        state.block_signal(sig);
        true
    }

    pub fn unblock(&mut self, pid: u32, sig: FullSignal) -> bool {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        state.unblock_signal(sig);
        true
    }

    pub fn set_mask(&mut self, pid: u32, mask: SignalMask) -> bool {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        state.blocked_mask = mask;
        true
    }

    pub fn get_mask(&self, pid: u32) -> Option<SignalMask> {
        self.processes.get(&pid).map(|s| s.blocked_mask)
    }

    pub fn is_blocked(&self, pid: u32, sig: FullSignal) -> bool {
        self.processes.get(&pid).map(|s| s.is_blocked(sig)).unwrap_or(false)
    }

    // ── AltStack Management ─────────────────────────────────

    pub fn set_altstack(&mut self, pid: u32, base: u64, size: u64) -> bool {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        match AltStack::new(base, size) {
            Some(stack) => { state.set_altstack(stack); true }
            None => false,
        }
    }

    pub fn get_altstack(&self, pid: u32) -> Option<&AltStack> {
        self.processes.get(&pid).and_then(|s| s.altstack.as_ref())
    }

    // ── Timer Management ────────────────────────────────────

    pub fn set_timer(&mut self, pid: u32, timer_type: TimerType, initial: TimeVal, interval: TimeVal) -> bool {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        let timer = state.get_timer_mut(timer_type);
        timer.set(initial, interval);
        true
    }

    pub fn cancel_timer(&mut self, pid: u32, timer_type: TimerType) -> bool {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        let timer = state.get_timer_mut(timer_type);
        timer.cancel();
        true
    }

    pub fn tick_timers(&mut self, pid: u32, elapsed_real_us: u64, elapsed_virt_us: u64, elapsed_prof_us: u64) -> Vec<FullSignal> {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return Vec::new(),
        };

        let mut fired = Vec::new();
        let elapsed = [elapsed_real_us, elapsed_virt_us, elapsed_prof_us];
        for i in 0..3 {
            if state.timers[i].tick(elapsed[i]) {
                fired.push(state.timers[i].timer_type.signal());
            }
        }
        fired
    }

    pub fn get_timer(&self, pid: u32, timer_type: TimerType) -> Option<&IntervalTimer> {
        self.processes.get(&pid).map(|s| &s.timers[match timer_type {
            TimerType::Real => 0, TimerType::Virtual => 1, TimerType::Prof => 2,
        }])
    }

    // ── Pending Query ────────────────────────────────────────

    pub fn pending_signals(&self, pid: u32) -> Vec<FullSignal> {
        self.processes.get(&pid).map(|s| {
            s.pending.iter().map(|e| e.signal).collect()
        }).unwrap_or_default()
    }

    pub fn clear_pending(&mut self, pid: u32) -> bool {
        let state = match self.processes.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        state.pending.clear();
        state.standard_pending = 0;
        true
    }

    // ── Audit ───────────────────────────────────────────────

    fn audit(&self, entry: SignalAuditEntry) {
        let mut log = self.audit_log.lock();
        if log.len() < MAX_AUDIT_ENTRIES {
            log.push(entry);
        }
    }

    pub fn audit_log(&self) -> Vec<SignalAuditEntry> {
        self.audit_log.lock().clone()
    }

    pub fn audit_count(&self) -> usize {
        self.audit_log.lock().len()
    }

    // ── Stats ───────────────────────────────────────────────

    pub fn stats(&self) -> SignalStats {
        SignalStats {
            registered_processes: self.processes.len(),
            process_groups: self.process_groups.len(),
            containers: self.container_pids.len(),
            total_sent: self.total_sent,
            total_delivered: self.total_delivered,
            total_coalesced: self.total_coalesced,
            total_dropped: self.total_dropped,
            audit_entries: self.audit_count(),
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  SIGNAL STATS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SignalStats {
    pub registered_processes: usize,
    pub process_groups: usize,
    pub containers: usize,
    pub total_sent: u64,
    pub total_delivered: u64,
    pub total_coalesced: u64,
    pub total_dropped: u64,
    pub audit_entries: usize,
}

// ════════════════════════════════════════════════════════════════
//  TESTS
// ════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── FullSignal Tests ──────────────────────────────────────

    #[test]
    fn test_full_signal_from_u8() {
        assert_eq!(FullSignal::from_u8(1), Some(FullSignal::Sighup));
        assert_eq!(FullSignal::from_u8(9), Some(FullSignal::Sigkill));
        assert_eq!(FullSignal::from_u8(15), Some(FullSignal::Sigterm));
        assert_eq!(FullSignal::from_u8(32), Some(FullSignal::SigrtMin));
        assert_eq!(FullSignal::from_u8(63), Some(FullSignal::SigrtMax));
        assert_eq!(FullSignal::from_u8(0), None);
        assert_eq!(FullSignal::from_u8(64), None);
    }

    #[test]
    fn test_full_signal_number() {
        assert_eq!(FullSignal::Sighup.number(), 1);
        assert_eq!(FullSignal::Sigkill.number(), 9);
        assert_eq!(FullSignal::SigrtMax.number(), 63);
    }

    #[test]
    fn test_full_signal_is_standard() {
        assert!(FullSignal::Sighup.is_standard());
        assert!(FullSignal::Sigterm.is_standard());
        assert!(!FullSignal::SigrtMin.is_standard());
    }

    #[test]
    fn test_full_signal_is_realtime() {
        assert!(!FullSignal::Sighup.is_realtime());
        assert!(FullSignal::SigrtMin.is_realtime());
        assert!(FullSignal::SigrtMax.is_realtime());
    }

    #[test]
    fn test_full_signal_is_unblockable() {
        assert!(FullSignal::Sigkill.is_unblockable());
        assert!(FullSignal::Sigstop.is_unblockable());
        assert!(!FullSignal::Sigterm.is_unblockable());
        assert!(!FullSignal::Sigrt1.is_unblockable());
    }

    #[test]
    fn test_full_signal_is_fatal() {
        assert!(FullSignal::Sigkill.is_fatal());
        assert!(FullSignal::Sigterm.is_fatal());
        assert!(FullSignal::Sigsegv.is_fatal());
        assert!(!FullSignal::Sigchld.is_fatal());
        assert!(!FullSignal::Sigcont.is_fatal());
    }

    #[test]
    fn test_full_signal_names() {
        assert_eq!(FullSignal::Sighup.name(), "SIGHUP");
        assert_eq!(FullSignal::Sigkill.name(), "SIGKILL");
        assert_eq!(FullSignal::Sigterm.name(), "SIGTERM");
        assert_eq!(FullSignal::SigrtMin.name(), "SIGRTMIN");
        assert_eq!(FullSignal::SigrtMax.name(), "SIGRTMAX");
    }

    #[test]
    fn test_full_signal_default_action() {
        assert_eq!(FullSignal::Sigkill.default_action(), SignalAction::Terminate);
        assert_eq!(FullSignal::Sigterm.default_action(), SignalAction::Terminate);
        assert_eq!(FullSignal::Sigsegv.default_action(), SignalAction::TerminateCore);
        assert_eq!(FullSignal::Sigstop.default_action(), SignalAction::Stop);
        assert_eq!(FullSignal::Sigcont.default_action(), SignalAction::Continue);
        assert_eq!(FullSignal::Sigchld.default_action(), SignalAction::Ignore);
        assert_eq!(FullSignal::SigrtMin.default_action(), SignalAction::Terminate);
    }

    #[test]
    fn test_full_signal_delivery_priority() {
        // SIGKILL has highest priority (lowest number)
        assert!(FullSignal::Sigkill.delivery_priority() < FullSignal::Sigterm.delivery_priority());
        assert!(FullSignal::Sigstop.delivery_priority() < FullSignal::Sigchld.delivery_priority());
        assert!(FullSignal::Sigsegv.delivery_priority() < FullSignal::Sigusr1.delivery_priority());
    }

    #[test]
    fn test_all_signals_have_names() {
        for n in 1..=63u8 {
            let sig = FullSignal::from_u8(n).unwrap();
            let name = sig.name();
            assert!(!name.is_empty());
        }
    }

    // ── SignalMask Tests ─────────────────────────────────────

    #[test]
    fn test_signal_mask_empty() {
        let mask = SignalMask::empty();
        assert!(mask.is_empty());
        assert_eq!(mask.count(), 0);
    }

    #[test]
    fn test_signal_mask_full() {
        let mask = SignalMask::full();
        assert!(!mask.is_empty());
        assert_eq!(mask.count(), 64);
    }

    #[test]
    fn test_signal_mask_add_remove() {
        let mut mask = SignalMask::empty();
        mask.add(FullSignal::Sigterm);
        mask.add(FullSignal::Sigkill);
        assert_eq!(mask.count(), 2);
        assert!(mask.contains(FullSignal::Sigterm));
        assert!(mask.contains(FullSignal::Sigkill));
        assert!(!mask.contains(FullSignal::Sighup));

        mask.remove(FullSignal::Sigterm);
        assert!(!mask.contains(FullSignal::Sigterm));
        assert!(mask.contains(FullSignal::Sigkill));
        assert_eq!(mask.count(), 1);
    }

    #[test]
    fn test_signal_mask_union() {
        let mut a = SignalMask::empty();
        a.add(FullSignal::Sigterm);
        let mut b = SignalMask::empty();
        b.add(FullSignal::Sigkill);
        let c = a.union(&b);
        assert!(c.contains(FullSignal::Sigterm));
        assert!(c.contains(FullSignal::Sigkill));
    }

    #[test]
    fn test_signal_mask_intersection() {
        let mut a = SignalMask::empty();
        a.add(FullSignal::Sigterm);
        a.add(FullSignal::Sigkill);
        let mut b = SignalMask::empty();
        b.add(FullSignal::Sigkill);
        let c = a.intersection(&b);
        assert!(c.contains(FullSignal::Sigkill));
        assert!(!c.contains(FullSignal::Sigterm));
    }

    #[test]
    fn test_signal_mask_difference() {
        let mut a = SignalMask::empty();
        a.add(FullSignal::Sigterm);
        a.add(FullSignal::Sigkill);
        let mut b = SignalMask::empty();
        b.add(FullSignal::Sigkill);
        let c = a.difference(&b);
        assert!(c.contains(FullSignal::Sigterm));
        assert!(!c.contains(FullSignal::Sigkill));
    }

    // ── SignalInfo Tests ─────────────────────────────────────

    #[test]
    fn test_signal_info_default() {
        let info = SignalInfo::default();
        assert_eq!(info.signo, 0);
        assert_eq!(info.code, SignalCode::User);
        assert_eq!(info.sender_pid, None);
    }

    #[test]
    fn test_signal_code_names() {
        assert_eq!(SignalCode::User.name(), "SI_USER");
        assert_eq!(SignalCode::Kernel.name(), "SI_KERNEL");
        assert_eq!(SignalCode::Timer.name(), "SI_TIMER");
        assert_eq!(SignalCode::Fault.name(), "SI_FAULT");
    }

    // ── AltStack Tests ───────────────────────────────────────

    #[test]
    fn test_altstack_creation() {
        let stack = AltStack::new(0x100000, 64 * 1024).unwrap();
        assert_eq!(stack.base_addr, 0x100000);
        assert_eq!(stack.size, 64 * 1024);
        assert!(stack.is_valid());
        assert_eq!(stack.top(), 0x100000 + 64 * 1024);
    }

    #[test]
    fn test_altstack_too_small() {
        assert!(AltStack::new(0x100000, 100).is_none());  // Below MIN
    }

    #[test]
    fn test_altstack_too_large() {
        assert!(AltStack::new(0x100000, 2 * 1024 * 1024).is_none());  // Above MAX
    }

    #[test]
    fn test_altstack_disabled() {
        let mut stack = AltStack::new(0x100000, 64 * 1024).unwrap();
        stack.flags.disabled = true;
        assert!(!stack.is_valid());
    }

    // ── TimeVal Tests ────────────────────────────────────────

    #[test]
    fn test_timeval_zero() {
        let tv = TimeVal::zero();
        assert!(tv.is_zero());
        assert_eq!(tv.to_us(), 0);
    }

    #[test]
    fn test_timeval_from_ms() {
        let tv = TimeVal::from_ms(1500);
        assert_eq!(tv.seconds, 1);
        assert_eq!(tv.microseconds, 500_000);
        assert_eq!(tv.to_us(), 1_500_000);
    }

    #[test]
    fn test_timeval_to_us() {
        let tv = TimeVal { seconds: 2, microseconds: 500_000 };
        assert_eq!(tv.to_us(), 2_500_000);
    }

    // ── IntervalTimer Tests ──────────────────────────────────

    #[test]
    fn test_interval_timer_new() {
        let timer = IntervalTimer::new(TimerType::Real);
        assert!(!timer.active);
        assert!(timer.interval.is_zero());
    }

    #[test]
    fn test_interval_timer_set() {
        let mut timer = IntervalTimer::new(TimerType::Real);
        timer.set(TimeVal::from_ms(100), TimeVal::from_ms(50));
        assert!(timer.active);
        assert_eq!(timer.initial.to_us(), 100_000);
        assert_eq!(timer.interval.to_us(), 50_000);
    }

    #[test]
    fn test_interval_timer_cancel() {
        let mut timer = IntervalTimer::new(TimerType::Real);
        timer.set(TimeVal::from_ms(100), TimeVal::from_ms(50));
        timer.cancel();
        assert!(!timer.active);
        assert!(timer.remaining.is_zero());
    }

    #[test]
    fn test_interval_timer_tick_fires() {
        let mut timer = IntervalTimer::new(TimerType::Real);
        timer.set(TimeVal::from_ms(100), TimeVal::from_ms(50));
        assert_eq!(timer.remaining.to_us(), 100_000);

        let fired = timer.tick(100_000);
        assert!(fired);
        assert_eq!(timer.overrun, 1);
        // Repeating timer — interval resets
        assert_eq!(timer.remaining.to_us(), 50_000);
        assert!(timer.active);
    }

    #[test]
    fn test_interval_timer_tick_no_fire() {
        let mut timer = IntervalTimer::new(TimerType::Real);
        timer.set(TimeVal::from_ms(100), TimeVal::zero());
        let fired = timer.tick(50_000);
        assert!(!fired);
        assert_eq!(timer.remaining.to_us(), 50_000);
    }

    #[test]
    fn test_interval_timer_oneshot() {
        let mut timer = IntervalTimer::new(TimerType::Real);
        timer.set(TimeVal::from_ms(100), TimeVal::zero());
        let fired = timer.tick(100_000);
        assert!(fired);
        assert!(!timer.active);  // No interval — one-shot
    }

    #[test]
    fn test_timer_type_signal() {
        assert_eq!(TimerType::Real.signal(), FullSignal::Sigalrm);
        assert_eq!(TimerType::Virtual.signal(), FullSignal::Sigvtalrm);
        assert_eq!(TimerType::Prof.signal(), FullSignal::Sigprof);
    }

    #[test]
    fn test_timer_type_names() {
        assert_eq!(TimerType::Real.name(), "ITIMER_REAL");
        assert_eq!(TimerType::Virtual.name(), "ITIMER_VIRTUAL");
        assert_eq!(TimerType::Prof.name(), "ITIMER_PROF");
    }

    // ── ProcessSignalState Tests ─────────────────────────────

    #[test]
    fn test_process_signal_state_new() {
        let state = ProcessSignalState::new(100);
        assert_eq!(state.pid, 100);
        assert!(state.pending.is_empty());
        assert!(state.blocked_mask.is_empty());
        assert!(!state.in_handler);
    }

    #[test]
    fn test_process_signal_state_disposition() {
        let mut state = ProcessSignalState::new(100);
        let disp = SignalDisposition::Ignore;
        assert!(state.set_disposition(FullSignal::Sigterm, disp));
        assert_eq!(state.get_disposition(FullSignal::Sigterm), SignalDisposition::Ignore);

        // Cannot set disposition for SIGKILL
        assert!(!state.set_disposition(FullSignal::Sigkill, SignalDisposition::Ignore));
    }

    #[test]
    fn test_process_signal_state_block_unblock() {
        let mut state = ProcessSignalState::new(100);
        state.block_signal(FullSignal::Sigterm);
        assert!(state.is_blocked(FullSignal::Sigterm));
        assert!(!state.is_blocked(FullSignal::Sighup));

        // SIGKILL cannot be blocked
        state.block_signal(FullSignal::Sigkill);
        assert!(!state.is_blocked(FullSignal::Sigkill));

        state.unblock_signal(FullSignal::Sigterm);
        assert!(!state.is_blocked(FullSignal::Sigterm));
    }

    #[test]
    fn test_process_signal_state_standard_coalescing() {
        let mut state = ProcessSignalState::new(100);
        assert!(state.mark_pending_standard(FullSignal::Sigterm));
        assert!(state.has_pending_standard(FullSignal::Sigterm));
        // Second mark — coalesced
        assert!(!state.mark_pending_standard(FullSignal::Sigterm));
        state.clear_pending_standard(FullSignal::Sigterm);
        assert!(!state.has_pending_standard(FullSignal::Sigterm));
    }

    #[test]
    fn test_process_signal_state_altstack() {
        let mut state = ProcessSignalState::new(100);
        let stack = AltStack::new(0x200000, 128 * 1024).unwrap();
        state.set_altstack(stack);
        assert!(state.altstack.is_some());
        assert_eq!(state.get_altstack().unwrap().base_addr, 0x200000);
    }

    #[test]
    fn test_process_signal_state_timers() {
        let mut state = ProcessSignalState::new(100);
        let timer = state.get_timer_mut(TimerType::Real);
        timer.set(TimeVal::from_ms(500), TimeVal::zero());
        assert!(timer.active);
        assert_eq!(timer.remaining.to_us(), 500_000);
    }

    #[test]
    fn test_process_signal_state_pending_full() {
        let mut state = ProcessSignalState::new(100);
        assert!(!state.is_full());
        // Simulate filling pending queue
        for i in 0..MAX_PENDING_PER_PROCESS {
            state.pending.push(PendingEntry {
                signal: FullSignal::SigrtMin,
                info: SignalInfo::default(),
            });
        }
        assert!(state.is_full());
    }

    // ── AdvancedSignalManager: Registration Tests ───────────

    #[test]
    fn test_manager_register() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        assert!(mgr.is_registered(100));
        assert!(!mgr.is_registered(200));
    }

    #[test]
    fn test_manager_unregister() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.unregister(100);
        assert!(!mgr.is_registered(100));
    }

    // ── AdvancedSignalManager: Signal Sending Tests ─────────

    #[test]
    fn test_manager_send_to_process() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        let info = SignalInfo::default();
        let result = mgr.send_to_process(100, FullSignal::Sigterm, info);
        assert_eq!(result, SignalSendResult::Queued);
        assert!(mgr.has_pending(100));
        assert_eq!(mgr.pending_count(100), 1);
    }

    #[test]
    fn test_manager_send_no_target() {
        let mut mgr = AdvancedSignalManager::new();
        let info = SignalInfo::default();
        let result = mgr.send_to_process(999, FullSignal::Sigterm, info);
        assert_eq!(result, SignalSendResult::NoTarget);
    }

    #[test]
    fn test_manager_send_standard_coalescing() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);

        let info1 = SignalInfo { signo: 15, code: SignalCode::User, ..SignalInfo::default() };
        let info2 = SignalInfo { signo: 15, code: SignalCode::User, ..SignalInfo::default() };

        let r1 = mgr.send_to_process(100, FullSignal::Sigterm, info1);
        let r2 = mgr.send_to_process(100, FullSignal::Sigterm, info2);

        assert_eq!(r1, SignalSendResult::Queued);
        assert_eq!(r2, SignalSendResult::Coalesced);
        assert_eq!(mgr.pending_count(100), 1);  // Only one entry
    }

    #[test]
    fn test_manager_send_realtime_no_coalescing() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);

        let info1 = SignalInfo { signo: 32, code: SignalCode::Queue, ..SignalInfo::default() };
        let info2 = SignalInfo { signo: 32, code: SignalCode::Queue, ..SignalInfo::default() };

        let r1 = mgr.send_to_process(100, FullSignal::SigrtMin, info1);
        let r2 = mgr.send_to_process(100, FullSignal::SigrtMin, info2);

        assert_eq!(r1, SignalSendResult::Queued);
        assert_eq!(r2, SignalSendResult::Queued);
        assert_eq!(mgr.pending_count(100), 2);  // Both queued (no coalescing for RT)
    }

    // ── AdvancedSignalManager: Delivery Tests ────────────────

    #[test]
    fn test_manager_deliver_default_terminate() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.send_to_process(100, FullSignal::Sigterm, SignalInfo::default());

        let result = mgr.deliver(100);
        assert!(result.is_some());
        let (sig, _info, resolution) = result.unwrap();
        assert_eq!(sig, FullSignal::Sigterm);
        assert_eq!(resolution, SignalResolution::Terminate(0));
        assert!(!mgr.has_pending(100));
    }

    #[test]
    fn test_manager_deliver_sigstop() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.send_to_process(100, FullSignal::Sigstop, SignalInfo::default());

        let result = mgr.deliver(100);
        let (_sig, _info, resolution) = result.unwrap();
        assert_eq!(resolution, SignalResolution::Stop);
    }

    #[test]
    fn test_manager_deliver_sigcont() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.send_to_process(100, FullSignal::Sigcont, SignalInfo::default());

        let result = mgr.deliver(100);
        let (_sig, _info, resolution) = result.unwrap();
        assert_eq!(resolution, SignalResolution::Continue);
    }

    #[test]
    fn test_manager_deliver_ignore() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        // SIGCHLD default is Ignore
        mgr.send_to_process(100, FullSignal::Sigchld, SignalInfo::default());

        let result = mgr.deliver(100);
        let (_sig, _info, resolution) = result.unwrap();
        assert_eq!(resolution, SignalResolution::Ignore);
    }

    #[test]
    fn test_manager_deliver_catch_handler() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.set_handler(100, FullSignal::Sigusr1, SignalDisposition::Catch {
            handler_addr: 0xDEADBEEF,
            flags: SignalHandlerFlags::none(),
            mask: 0,
        });

        mgr.send_to_process(100, FullSignal::Sigusr1, SignalInfo::default());
        let result = mgr.deliver(100);
        let (_sig, _info, resolution) = result.unwrap();
        match resolution {
            SignalResolution::CallHandler { addr, .. } => assert_eq!(addr, 0xDEADBEEF),
            _ => panic!("Expected CallHandler"),
        }
    }

    #[test]
    fn test_manager_deliver_priority_order() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);

        // Send SIGUSR1 (priority 10) first, then SIGKILL (priority 0)
        mgr.send_to_process(100, FullSignal::Sigusr1, SignalInfo::default());
        mgr.send_to_process(100, FullSignal::Sigkill, SignalInfo::default());

        // SIGKILL should be delivered first (higher priority)
        let result = mgr.deliver(100);
        let (sig, _, _) = result.unwrap();
        assert_eq!(sig, FullSignal::Sigkill);

        // Then SIGUSR1
        let result2 = mgr.deliver(100);
        let (sig2, _, _) = result2.unwrap();
        assert_eq!(sig2, FullSignal::Sigusr1);
    }

    #[test]
    fn test_manager_deliver_blocked_signal() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.block(100, FullSignal::Sigterm);
        mgr.send_to_process(100, FullSignal::Sigterm, SignalInfo::default());

        // Should not deliver blocked signal
        let result = mgr.deliver(100);
        assert!(result.is_none());  // Blocked, no delivery
        assert!(mgr.has_pending(100));  // Still pending
    }

    #[test]
    fn test_manager_deliver_unblockable_bypass() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        // SIGKILL is unblockable — try to block it
        mgr.block(100, FullSignal::Sigkill);
        assert!(!mgr.is_blocked(100, FullSignal::Sigkill));

        mgr.send_to_process(100, FullSignal::Sigkill, SignalInfo::default());
        let result = mgr.deliver(100);
        assert!(result.is_some());
    }

    #[test]
    fn test_manager_return_from_handler() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.set_handler(100, FullSignal::Sigusr1, SignalDisposition::Catch {
            handler_addr: 0x4000,
            flags: SignalHandlerFlags::none(),
            mask: 0,
        });
        mgr.send_to_process(100, FullSignal::Sigusr1, SignalInfo::default());
        let _ = mgr.deliver(100);

        // In handler — SIGUSR1 should be blocked
        assert!(mgr.is_blocked(100, FullSignal::Sigusr1));

        // Return from handler
        assert!(mgr.return_from_handler(100));
        assert!(!mgr.is_blocked(100, FullSignal::Sigusr1));
    }

    // ── AdvancedSignalManager: Process Group Tests ──────────

    #[test]
    fn test_manager_process_group() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.register(101);
        mgr.register(102);

        mgr.create_process_group(500, 100);
        mgr.add_to_group(500, 101);
        mgr.add_to_group(500, 102);

        let members = mgr.get_group_members(500).unwrap();
        assert_eq!(members.len(), 3);
        assert!(members.contains(&100));
        assert!(members.contains(&101));
    }

    #[test]
    fn test_manager_send_to_group() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.register(101);
        mgr.create_process_group(500, 100);
        mgr.add_to_group(500, 101);

        let results = mgr.send_to_group(500, FullSignal::Sigterm, Some(999));
        assert_eq!(results.len(), 2);
        for (_, result) in &results {
            assert_eq!(*result, SignalSendResult::Queued);
        }
    }

    #[test]
    fn test_manager_remove_from_group() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.register(101);
        mgr.create_process_group(500, 100);
        mgr.add_to_group(500, 101);

        assert!(mgr.remove_from_group(500, 101));
        let members = mgr.get_group_members(500).unwrap();
        assert_eq!(members.len(), 1);
        assert!(!members.contains(&101));
    }

    #[test]
    fn test_manager_send_to_nonexistent_group() {
        let mut mgr = AdvancedSignalManager::new();
        let results = mgr.send_to_group(999, FullSignal::Sigterm, None);
        assert!(results.is_empty());
    }

    // ── AdvancedSignalManager: Container Tests ──────────────

    #[test]
    fn test_manager_container_signals() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.register(101);
        mgr.register_container_pid(1, 100);
        mgr.register_container_pid(1, 101);

        let results = mgr.send_to_container(1, FullSignal::Sigterm, None);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_manager_unregister_removes_from_container() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.register_container_pid(1, 100);
        mgr.unregister(100);

        let results = mgr.send_to_container(1, FullSignal::Sigterm, None);
        assert!(results.is_empty());
    }

    // ── AdvancedSignalManager: Broadcast Tests ───────────────

    #[test]
    fn test_manager_broadcast() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.register(101);
        mgr.register(102);

        let results = mgr.broadcast(FullSignal::Sighup, Some(100));
        // Should not send to sender
        assert_eq!(results.len(), 2);
        assert!(!results.iter().any(|(pid, _)| *pid == 100));
    }

    // ── AdvancedSignalManager: Handler Tests ─────────────────

    #[test]
    fn test_manager_set_handler_unblockable_fails() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        assert!(!mgr.set_handler(100, FullSignal::Sigkill, SignalDisposition::Ignore));
        assert!(!mgr.set_handler(100, FullSignal::Sigstop, SignalDisposition::Ignore));
    }

    #[test]
    fn test_manager_set_handler_success() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        assert!(mgr.set_handler(100, FullSignal::Sigterm, SignalDisposition::Ignore));
        let handler = mgr.get_handler(100, FullSignal::Sigterm).unwrap();
        assert_eq!(handler, SignalDisposition::Ignore);
    }

    // ── AdvancedSignalManager: Mask Tests ───────────────────

    #[test]
    fn test_manager_set_get_mask() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        let mut mask = SignalMask::empty();
        mask.add(FullSignal::Sigterm);
        mgr.set_mask(100, mask);
        let retrieved = mgr.get_mask(100).unwrap();
        assert!(retrieved.contains(FullSignal::Sigterm));
        assert!(!retrieved.contains(FullSignal::Sighup));
    }

    #[test]
    fn test_manager_block_unblock() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.block(100, FullSignal::Sigterm);
        assert!(mgr.is_blocked(100, FullSignal::Sigterm));
        mgr.unblock(100, FullSignal::Sigterm);
        assert!(!mgr.is_blocked(100, FullSignal::Sigterm));
    }

    // ── AdvancedSignalManager: AltStack Tests ───────────────

    #[test]
    fn test_manager_set_altstack() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        assert!(mgr.set_altstack(100, 0x100000, 64 * 1024));
        assert!(mgr.get_altstack(100).is_some());
        assert_eq!(mgr.get_altstack(100).unwrap().base_addr, 0x100000);
    }

    #[test]
    fn test_manager_set_altstack_invalid_size() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        assert!(!mgr.set_altstack(100, 0x100000, 100));  // Too small
    }

    // ── AdvancedSignalManager: Timer Tests ──────────────────

    #[test]
    fn test_manager_set_timer() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        assert!(mgr.set_timer(100, TimerType::Real, TimeVal::from_ms(500), TimeVal::from_ms(100)));
        let timer = mgr.get_timer(100, TimerType::Real).unwrap();
        assert!(timer.active);
        assert_eq!(timer.initial.to_us(), 500_000);
    }

    #[test]
    fn test_manager_cancel_timer() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.set_timer(100, TimerType::Real, TimeVal::from_ms(500), TimeVal::from_ms(100));
        assert!(mgr.cancel_timer(100, TimerType::Real));
        let timer = mgr.get_timer(100, TimerType::Real).unwrap();
        assert!(!timer.active);
    }

    #[test]
    fn test_manager_tick_timers() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.set_timer(100, TimerType::Real, TimeVal::from_ms(100), TimeVal::from_ms(50));
        mgr.set_timer(100, TimerType::Virtual, TimeVal::from_ms(200), TimeVal::zero());

        // Tick 100ms real, 200ms virtual
        let fired = mgr.tick_timers(100, 100_000, 200_000, 0);
        assert!(fired.contains(&FullSignal::Sigalrm));
        assert!(fired.contains(&FullSignal::Sigvtalrm));
    }

    // ── AdvancedSignalManager: Clear Pending Tests ──────────

    #[test]
    fn test_manager_clear_pending() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.send_to_process(100, FullSignal::Sigterm, SignalInfo::default());
        mgr.send_to_process(100, FullSignal::Sigusr1, SignalInfo::default());
        assert_eq!(mgr.pending_count(100), 2);

        assert!(mgr.clear_pending(100));
        assert_eq!(mgr.pending_count(100), 0);
        assert!(!mgr.has_pending(100));
    }

    // ── AdvancedSignalManager: Audit Tests ──────────────────

    #[test]
    fn test_manager_audit() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.send_to_process(100, FullSignal::Sigterm, SignalInfo::default());
        assert!(mgr.audit_count() > 0);
    }

    #[test]
    fn test_manager_audit_log() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.send_to_process(100, FullSignal::Sigterm, SignalInfo::default());
        let log = mgr.audit_log();
        assert!(!log.is_empty());
        assert_eq!(log[0].signal, FullSignal::Sigterm);
        assert_eq!(log[0].target_type, SignalTargetType::Process);
    }

    // ── AdvancedSignalManager: Stats Tests ──────────────────

    #[test]
    fn test_manager_stats() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);
        mgr.register(101);
        mgr.create_process_group(500, 100);
        mgr.register_container_pid(1, 100);

        mgr.send_to_process(100, FullSignal::Sigterm, SignalInfo::default());
        mgr.send_to_process(101, FullSignal::Sigkill, SignalInfo::default());

        let stats = mgr.stats();
        assert_eq!(stats.registered_processes, 2);
        assert_eq!(stats.process_groups, 1);
        assert_eq!(stats.containers, 1);
        assert_eq!(stats.total_sent, 2);
    }

    // ── Integration Tests ────────────────────────────────────

    #[test]
    fn test_integration_full_signal_lifecycle() {
        let mut mgr = AdvancedSignalManager::new();

        // 1. Register processes
        mgr.register(100);
        mgr.register(101);
        mgr.register(102);

        // 2. Create process group
        mgr.create_process_group(500, 100);
        mgr.add_to_group(500, 101);
        mgr.add_to_group(500, 102);

        // 3. Register container
        mgr.register_container_pid(1, 100);
        mgr.register_container_pid(1, 101);

        // 4. Set handler on process 100
        assert!(mgr.set_handler(100, FullSignal::Sigusr1, SignalDisposition::Catch {
            handler_addr: 0x4000,
            flags: SignalHandlerFlags::restart(),
            mask: 0,
        }));

        // 5. Set altstack on process 100
        assert!(mgr.set_altstack(100, 0x200000, 256 * 1024));

        // 6. Set timer on process 100
        assert!(mgr.set_timer(100, TimerType::Real, TimeVal::from_ms(1000), TimeVal::from_ms(500)));

        // 7. Block SIGTERM on process 101
        mgr.block(101, FullSignal::Sigterm);

        // 8. Send signals
        mgr.send_to_process(100, FullSignal::Sigusr1, SignalInfo::default());
        mgr.send_to_group(500, FullSignal::Sighup, Some(999));
        mgr.send_to_container(1, FullSignal::Sigchld, None);

        // 9. Deliver to process 100 — should get SIGUSR1 (only pending)
        let result = mgr.deliver(100);
        assert!(result.is_some());
        let (sig, _, resolution) = result.unwrap();
        assert_eq!(sig, FullSignal::Sigusr1);
        match resolution {
            SignalResolution::CallHandler { addr, flags, .. } => {
                assert_eq!(addr, 0x4000);
                assert!(flags.restart_syscall);
            }
            _ => panic!("Expected CallHandler"),
        }

        // 10. Return from handler
        assert!(mgr.return_from_handler(100));

        // 11. Process 101 has SIGTERM blocked — deliver should skip it
        let result101 = mgr.deliver(101);
        // SIGHUP was also sent to group — should deliver that
        assert!(result101.is_some());
        let (sig101, _, _) = result101.unwrap();
        assert_eq!(sig101, FullSignal::Sighup);

        // 12. Check stats
        let stats = mgr.stats();
        assert!(stats.total_sent > 0);
        assert!(stats.audit_entries > 0);
    }

    #[test]
    fn test_integration_rt_signal_queueing() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);

        // Send 5 RT signals — all should queue (no coalescing)
        for i in 0..5 {
            let info = SignalInfo {
                signo: 32,
                code: SignalCode::Queue,
                value: i,
                ..SignalInfo::default()
            };
            let result = mgr.send_to_process(100, FullSignal::SigrtMin, info);
            assert_eq!(result, SignalSendResult::Queued);
        }

        assert_eq!(mgr.pending_count(100), 5);

        // Deliver all 5
        let mut count = 0;
        while let Some((sig, info, _)) = mgr.deliver(100) {
            assert_eq!(sig, FullSignal::SigrtMin);
            count += 1;
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn test_integration_standard_signal_coalescing() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);

        // Send 5 SIGTERM signals — only 1 should be pending (coalescing)
        for _ in 0..5 {
            mgr.send_to_process(100, FullSignal::Sigterm, SignalInfo::default());
        }

        assert_eq!(mgr.pending_count(100), 1);

        // Deliver — only 1 delivery
        let result = mgr.deliver(100);
        assert!(result.is_some());
        assert!(!mgr.has_pending(100));
    }

    #[test]
    fn test_integration_timer_fires_signal() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);

        // Set a 100ms one-shot timer
        mgr.set_timer(100, TimerType::Real, TimeVal::from_ms(100), TimeVal::zero());

        // Tick 100ms of real time
        let fired = mgr.tick_timers(100, 100_000, 0, 0);
        assert!(fired.contains(&FullSignal::Sigalrm));

        // Timer is one-shot — should not fire again
        let fired2 = mgr.tick_timers(100, 100_000, 0, 0);
        assert!(!fired2.contains(&FullSignal::Sigalrm));
    }

    #[test]
    fn test_integration_blocked_then_unblocked_delivery() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);

        // Block SIGTERM
        mgr.block(100, FullSignal::Sigterm);

        // Send SIGTERM — queued but not deliverable
        mgr.send_to_process(100, FullSignal::Sigterm, SignalInfo::default());
        assert!(mgr.has_pending(100));

        // Delivery should fail (blocked)
        assert!(mgr.deliver(100).is_none());
        assert!(mgr.has_pending(100));  // Still pending

        // Unblock SIGTERM
        mgr.unblock(100, FullSignal::Sigterm);

        // Now should deliver
        let result = mgr.deliver(100);
        assert!(result.is_some());
        let (sig, _, resolution) = result.unwrap();
        assert_eq!(sig, FullSignal::Sigterm);
        assert_eq!(resolution, SignalResolution::Terminate(0));
    }

    #[test]
    fn test_integration_container_signal_forwarding() {
        let mut mgr = AdvancedSignalManager::new();

        // Two containers, each with 2 processes
        mgr.register(100);
        mgr.register(101);
        mgr.register(200);
        mgr.register(201);

        mgr.register_container_pid(1, 100);
        mgr.register_container_pid(1, 101);
        mgr.register_container_pid(2, 200);
        mgr.register_container_pid(2, 201);

        // Send SIGTERM to container 1 only
        let results = mgr.send_to_container(1, FullSignal::Sigterm, Some(999));
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|(_, r)| *r == SignalSendResult::Queued));

        // Container 2 should not have pending signals
        assert!(!mgr.has_pending(200));
        assert!(!mgr.has_pending(201));

        // Container 1 processes should have pending
        assert!(mgr.has_pending(100));
        assert!(mgr.has_pending(101));
    }

    #[test]
    fn test_integration_handler_with_altstack() {
        let mut mgr = AdvancedSignalManager::new();
        mgr.register(100);

        // Set altstack
        assert!(mgr.set_altstack(100, 0x100000, 128 * 1024));

        // Set handler with SA_ONSTACK flag
        mgr.set_handler(100, FullSignal::Sigsegv, SignalDisposition::Catch {
            handler_addr: 0x5000,
            flags: SignalHandlerFlags { onstack: true, ..SignalHandlerFlags::default() },
            mask: 0,
        });

        // Send SIGSEGV
        mgr.send_to_process(100, FullSignal::Sigsegv, SignalInfo {
            code: SignalCode::Fault,
            addr: 0xBADADDR,
            ..SignalInfo::default()
        });

        // Deliver — should use altstack
        let result = mgr.deliver(100);
        assert!(result.is_some());
        let (_, _, resolution) = result.unwrap();
        match resolution {
            SignalResolution::CallHandler { addr, use_altstack, .. } => {
                assert_eq!(addr, 0x5000);
                assert!(use_altstack);
            }
            _ => panic!("Expected CallHandler with altstack"),
        }
    }
}
