// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 31: ELF64 Loader + Signal Handling
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// ELF64-Parser und -Loader für User-Prozesse (Ring 3).
// Signal-Handling für Userspace-Prozesse (POSIX-ähnlich).

use crate::ats1000::{Pid, ExitCode};

// ═══════════════════════════════════════════════════════════════════════════════
// ELF64 Constants
// ═══════════════════════════════════════════════════════════════════════════════

const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;  // Little endian
const ET_EXEC: u16 = 2;     // Executable
const EM_X86_64: u16 = 62;   // x86-64

const PT_LOAD:    u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PT_NOTE:   u32 = 4;

const PF_X: u32 = 1;  // Execute
const PF_W: u32 = 2;  // Write
const PF_R: u32 = 4;  // Read

// ═══════════════════════════════════════════════════════════════════════════════
// ELF64 Header
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Elf64Header {
    pub e_class:      u8,
    pub e_data:       u8,
    pub e_type:       u16,
    pub e_machine:    u16,
    pub e_entry:      u64,
    pub e_phoff:      u64,   // Program header table offset
    pub e_shoff:      u64,   // Section header table offset
    pub e_phnum:      u16,   // Number of program headers
    pub e_shnum:      u16,   // Number of section headers
    pub e_shstrndx:   u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElfParseError {
    InvalidMagic,
    InvalidClass,
    InvalidEndian,
    InvalidType,
    InvalidMachine,
    Truncated,
    InvalidOffset,
    NoLoadableSegments,
}

impl core::fmt::Display for ElfParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            ElfParseError::InvalidMagic         => write!(f, "invalid ELF magic"),
            ElfParseError::InvalidClass          => write!(f, "not ELF64"),
            ElfParseError::InvalidEndian         => write!(f, "not little-endian"),
            ElfParseError::InvalidType           => write!(f, "not executable"),
            ElfParseError::InvalidMachine        => write!(f, "not x86-64"),
            ElfParseError::Truncated             => write!(f, "file truncated"),
            ElfParseError::InvalidOffset         => write!(f, "invalid header offset"),
            ElfParseError::NoLoadableSegments    => write!(f, "no loadable segments"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ELF64 Program Header
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Elf64ProgramHeader {
    pub p_type:   u32,
    pub p_flags:  u32,
    pub p_offset: u64,
    pub p_vaddr:  u64,
    pub p_paddr:  u64,
    pub p_filesz: u64,
    pub p_memsz:  u64,
    pub p_align:  u64,
}

impl Elf64ProgramHeader {
    pub fn is_loadable(&self) -> bool { self.p_type == PT_LOAD }
    pub fn is_executable(&self) -> bool { (self.p_flags & PF_X) != 0 }
    pub fn is_writable(&self) -> bool { (self.p_flags & PF_W) != 0 }
    pub fn is_readable(&self) -> bool { (self.p_flags & PF_R) != 0 }
    pub fn needs_bss(&self) -> bool { self.p_memsz > self.p_filesz }
    pub fn bss_size(&self) -> u64 { self.p_memsz.saturating_sub(self.p_filesz) }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ELF64 Parser
// ═══════════════════════════════════════════════════════════════════════════════

pub struct ElfParser<'a> {
    data: &'a [u8],
    header: Elf64Header,
    program_headers: Vec<Elf64ProgramHeader>,
}

impl<'a> ElfParser<'a> {
    /// Parse an ELF64 binary from raw bytes
    pub fn parse(data: &'a [u8]) -> Result<Self, ElfParseError> {
        if data.len() < 64 {
            return Err(ElfParseError::Truncated);
        }

        // Check magic
        if data[0..4] != ELF_MAGIC {
            return Err(ElfParseError::InvalidMagic);
        }

        let e_class = data[4];
        if e_class != ELFCLASS64 {
            return Err(ElfParseError::InvalidClass);
        }

        let e_data = data[5];
        if e_data != ELFDATA2LSB {
            return Err(ElfParseError::InvalidEndian);
        }

        let e_type = u16::from_le_bytes([data[16], data[17]]);
        if e_type != ET_EXEC {
            return Err(ElfParseError::InvalidType);
        }

        let e_machine = u16::from_le_bytes([data[18], data[19]]);
        if e_machine != EM_X86_64 {
            return Err(ElfParseError::InvalidMachine);
        }

        let e_entry    = u64::from_le_bytes(data[24..32].try_into().unwrap());
        let e_phoff    = u64::from_le_bytes(data[32..40].try_into().unwrap());
        let e_shoff    = u64::from_le_bytes(data[40..48].try_into().unwrap());
        let e_phnum    = u16::from_le_bytes([data[56], data[57]]);
        let e_shnum    = u16::from_le_bytes([data[60], data[61]]);
        let e_shstrndx = u16::from_le_bytes([data[62], data[63]]);

        let header = Elf64Header {
            e_class, e_data, e_type, e_machine,
            e_entry, e_phoff, e_shoff, e_phnum, e_shnum, e_shstrndx,
        };

        // Parse program headers
        let mut program_headers = Vec::new();
        let phdr_size = 56; // sizeof(Elf64_Phdr) = 56 bytes

        for i in 0..e_phnum as usize {
            let offset = e_phoff as usize + i * phdr_size;
            if offset + phdr_size > data.len() {
                return Err(ElfParseError::Truncated);
            }
            let phdr = Elf64ProgramHeader {
                p_type:   u32::from_le_bytes(data[offset..offset+4].try_into().unwrap()),
                p_flags:  u32::from_le_bytes(data[offset+4..offset+8].try_into().unwrap()),
                p_offset: u64::from_le_bytes(data[offset+8..offset+16].try_into().unwrap()),
                p_vaddr:  u64::from_le_bytes(data[offset+16..offset+24].try_into().unwrap()),
                p_paddr:  u64::from_le_bytes(data[offset+24..offset+32].try_into().unwrap()),
                p_filesz: u64::from_le_bytes(data[offset+32..offset+40].try_into().unwrap()),
                p_memsz:  u64::from_le_bytes(data[offset+40..offset+48].try_into().unwrap()),
                p_align:  u64::from_le_bytes(data[offset+48..offset+56].try_into().unwrap()),
            };
            program_headers.push(phdr);
        }

        let has_loadable = program_headers.iter().any(|p| p.is_loadable());
        if !has_loadable {
            return Err(ElfParseError::NoLoadableSegments);
        }

        Ok(Self { data, header, program_headers })
    }

    pub fn entry_point(&self) -> u64 { self.header.e_entry }
    pub fn header(&self) -> &Elf64Header { &self.header }
    pub fn program_headers(&self) -> &[Elf64ProgramHeader] { &self.program_headers }
    pub fn loadable_segments(&self) -> impl Iterator<Item = &Elf64ProgramHeader> {
        self.program_headers.iter().filter(|p| p.is_loadable())
    }

    /// Extract the code segment (first executable PT_LOAD)
    pub fn code_segment(&self) -> Option<&Elf64ProgramHeader> {
        self.loadable_segments().find(|p| p.is_executable())
    }
    /// Extract the data segment (first writable PT_LOAD)
    pub fn data_segment(&self) -> Option<&Elf64ProgramHeader> {
        self.loadable_segments().find(|p| p.is_writable() && !p.is_executable())
    }

    /// Get raw bytes for a segment
    pub fn segment_data(&self, phdr: &Elf64ProgramHeader) -> Result<&[u8], ElfParseError> {
        let start = phdr.p_offset as usize;
        let end = start + phdr.p_filesz as usize;
        if end > self.data.len() {
            return Err(ElfParseError::Truncated);
        }
        Ok(&self.data[start..end])
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ELF Loader (integrates with UserspaceManager)
// ═══════════════════════════════════════════════════════════════════════════════

use crate::userspace::{UserBinary, UserAddressSpace, UserspaceManager, UserspaceError};

pub struct ElfLoader;

impl ElfLoader {
    /// Load an ELF64 binary into a UserspaceManager, returns PID
    pub fn load_elf(mgr: &mut UserspaceManager, elf_data: &[u8]) -> Result<Pid, ElfLoadError> {
        let parser = ElfParser::parse(elf_data)?;

        // Extract code segment
        let code_seg = parser.code_segment()
            .ok_or(ElfLoadError::NoCodeSegment)?;
        let code_bytes = parser.segment_data(code_seg)?;

        // Extract data segment (optional)
        let data_bytes = match parser.data_segment() {
            Some(seg) => parser.segment_data(seg)?.to_vec(),
            None => vec![],
        };

        let binary = UserBinary {
            entry_point: parser.entry_point(),
            code:  code_bytes.to_vec(),
            data:  data_bytes,
            name:  "elf".to_string(),
        };

        mgr.load_binary(binary).map_err(ElfLoadError::Userspace)
    }

    /// Create a minimal valid ELF64 binary for testing
    pub fn create_minimal_elf(entry: u64, code: &[u8]) -> Vec<u8> {
        let mut elf = vec![0u8; 64 + 56]; // Ehdr + one Phdr

        // ELF magic
        elf[0..4].copy_from_slice(&ELF_MAGIC);
        elf[4] = ELFCLASS64;
        elf[5] = ELFDATA2LSB;
        elf[6] = 1;  // EI_VERSION
        elf[7] = 0;  // EI_OSABI = ELFOSABI_NONE

        // e_type = ET_EXEC
        elf[16..18].copy_from_slice(&ET_EXEC.to_le_bytes());
        // e_machine = EM_X86_64
        elf[18..20].copy_from_slice(&EM_X86_64.to_le_bytes());
        // e_version
        elf[20..24].copy_from_slice(&1u32.to_le_bytes());
        // e_entry
        elf[24..32].copy_from_slice(&entry.to_le_bytes());
        // e_phoff = 64 (right after Ehdr)
        elf[32..40].copy_from_slice(&64u64.to_le_bytes());
        // e_shoff = 0 (no sections)
        elf[40..48].copy_from_slice(&0u64.to_le_bytes());
        // e_ehsize = 64
        elf[52..54].copy_from_slice(&64u16.to_le_bytes());
        // e_phentsize = 56
        elf[54..56].copy_from_slice(&56u16.to_le_bytes());
        // e_phnum = 1
        elf[56..58].copy_from_slice(&1u16.to_le_bytes());
        // e_shentsize = 0, e_shnum = 0, e_shstrndx = 0
        // (already zero)

        // Program header (PT_LOAD)
        let phdr_off = 64;
        // p_type = PT_LOAD
        elf[phdr_off..phdr_off+4].copy_from_slice(&PT_LOAD.to_le_bytes());
        // p_flags = PF_R | PF_X
        elf[phdr_off+4..phdr_off+8].copy_from_slice(&(PF_R | PF_X).to_le_bytes());
        // p_offset = 120 (after Ehdr + Phdr)
        elf[phdr_off+8..phdr_off+16].copy_from_slice(&120u64.to_le_bytes());
        // p_vaddr = entry
        elf[phdr_off+16..phdr_off+24].copy_from_slice(&entry.to_le_bytes());
        // p_paddr = entry
        elf[phdr_off+24..phdr_off+32].copy_from_slice(&entry.to_le_bytes());
        // p_filesz = code.len()
        elf[phdr_off+32..phdr_off+40].copy_from_slice(&(code.len() as u64).to_le_bytes());
        // p_memsz = code.len()
        elf[phdr_off+40..phdr_off+48].copy_from_slice(&(code.len() as u64).to_le_bytes());
        // p_align = 0x1000
        elf[phdr_off+48..phdr_off+56].copy_from_slice(&0x1000u64.to_le_bytes());

        // Append code at offset 120
        elf.extend_from_slice(code);
        elf
    }
}

#[derive(Debug)]
pub enum ElfLoadError {
    Parse(ElfParseError),
    NoCodeSegment,
    Userspace(UserspaceError),
}

impl From<ElfParseError> for ElfLoadError {
    fn from(e: ElfParseError) -> Self { ElfLoadError::Parse(e) }
}
impl From<UserspaceError> for ElfLoadError {
    fn from(e: UserspaceError) -> Self { ElfLoadError::Userspace(e) }
}

impl core::fmt::Display for ElfLoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            ElfLoadError::Parse(e)     => write!(f, "ELF parse error: {}", e),
            ElfLoadError::NoCodeSegment => write!(f, "no executable code segment"),
            ElfLoadError::Userspace(e) => write!(f, "userspace error: {}", e),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Signal Handling (POSIX-ähnlich für User-Prozesse)
// ═══════════════════════════════════════════════════════════════════════════════

/// Signal numbers (POSIX-ähnlich)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Signal {
    SigHup   = 1,   // Hangup
    SigInt   = 2,   // Interrupt (Ctrl+C)
    SigKill  = 9,   // Kill (unblockable)
    SigTerm  = 15,  // Termination
    SigStop  = 19,  // Stop (unblockable)
    SigCont  = 18,  // Continue
    SigSegv  = 11,  // Segmentation fault
    SigAlrm  = 14,  // Alarm clock
    SigUsr1  = 10,  // User-defined 1
    SigUsr2  = 12,  // User-defined 2
    SigChld  = 17,  // Child process stopped/terminated
}

impl Signal {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            1  => Some(Signal::SigHup),
            2  => Some(Signal::SigInt),
            9  => Some(Signal::SigKill),
            15 => Some(Signal::SigTerm),
            19 => Some(Signal::SigStop),
            18 => Some(Signal::SigCont),
            11 => Some(Signal::SigSegv),
            14 => Some(Signal::SigAlrm),
            10 => Some(Signal::SigUsr1),
            12 => Some(Signal::SigUsr2),
            17 => Some(Signal::SigChld),
            _  => None,
        }
    }
    pub fn is_unblockable(&self) -> bool {
        matches!(self, Signal::SigKill | Signal::SigStop)
    }
    pub fn is_fatal(&self) -> bool {
        matches!(self, Signal::SigKill | Signal::SigTerm | Signal::SigSegv | Signal::SigHup | Signal::SigInt)
    }
    pub fn name(&self) -> &'static str {
        match self {
            Signal::SigHup   => "SIGHUP",
            Signal::SigInt   => "SIGINT",
            Signal::SigKill  => "SIGKILL",
            Signal::SigTerm  => "SIGTERM",
            Signal::SigStop  => "SIGSTOP",
            Signal::SigCont  => "SIGCONT",
            Signal::SigSegv  => "SIGSEGV",
            Signal::SigAlrm  => "SIGALRM",
            Signal::SigUsr1  => "SIGUSR1",
            Signal::SigUsr2  => "SIGUSR2",
            Signal::SigChld  => "SIGCHLD",
        }
    }
}

/// Signal disposition (what happens when a signal is received)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SignalDisposition {
    /// Default action (terminate, stop, continue, ignore)
    Default,
    /// Ignore the signal
    Ignore,
    /// Catch the signal (invoke a handler in user space)
    Catch { handler_addr: u64 },
}

impl Default for SignalDisposition {
    fn default() -> Self { SignalDisposition::Default }
}

/// Default action for a signal
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SignalAction {
    Terminate,
    TerminateCore,  // Terminate + core dump
    Stop,
    Continue,
    Ignore,
}

impl Signal {
    pub fn default_action(&self) -> SignalAction {
        match self {
            Signal::SigKill | Signal::SigTerm | Signal::SigHup | Signal::SigInt => SignalAction::Terminate,
            Signal::SigSegv => SignalAction::TerminateCore,
            Signal::SigStop => SignalAction::Stop,
            Signal::SigCont => SignalAction::Continue,
            Signal::SigChld | Signal::SigUsr1 | Signal::SigUsr2 | Signal::SigAlrm => SignalAction::Ignore,
        }
    }
}

/// Pending signal for a process
#[derive(Clone, Debug)]
pub struct PendingSignal {
    pub signal: Signal,
    pub sender: Option<Pid>,
    pub data: u64,
}

/// Signal manager for user processes
pub struct SignalManager {
    /// Signal dispositions per process: pid -> [disposition; 32]
    handlers: Vec<(Pid, [SignalDisposition; 32])>,
    /// Pending signals per process
    pending: Vec<(Pid, Vec<PendingSignal>)>,
    /// Blocked signals per process (signal mask)
    blocked: Vec<(Pid, u32)>,
    /// Statistics
    signals_sent: u64,
    signals_delivered: u64,
}

impl Default for SignalManager {
    fn default() -> Self { Self::new() }
}

impl SignalManager {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            pending:  Vec::new(),
            blocked:  Vec::new(),
            signals_sent: 0,
            signals_delivered: 0,
        }
    }

    /// Register a new process for signal handling
    pub fn register(&mut self, pid: Pid) {
        if !self.handlers.iter().any(|(p, _)| *p == pid) {
            self.handlers.push((pid, [SignalDisposition::Default; 32]));
            self.pending.push((pid, Vec::new()));
            self.blocked.push((pid, 0));
        }
    }

    /// Unregister a process
    pub fn unregister(&mut self, pid: Pid) {
        self.handlers.retain(|(p, _)| *p != pid);
        self.pending.retain(|(p, _)| *p != pid);
        self.blocked.retain(|(p, _)| *p != pid);
    }

    /// Set signal disposition (handler)
    pub fn set_handler(&mut self, pid: Pid, signal: Signal, disp: SignalDisposition) -> bool {
        if signal.is_unblockable() {
            return false;  // Cannot change SIGKILL/SIGSTOP
        }
        if let Some((_, handlers)) = self.handlers.iter_mut().find(|(p, _)| *p == pid) {
            handlers[signal as u32 as usize] = disp;
            true
        } else {
            false
        }
    }

    /// Get signal disposition for a process/signal
    pub fn get_handler(&self, pid: Pid, signal: Signal) -> SignalDisposition {
        self.handlers.iter()
            .find(|(p, _)| *p == pid)
            .map(|(_, h)| h[signal as u32 as usize])
            .unwrap_or(SignalDisposition::Default)
    }

    /// Block a signal for a process
    pub fn block(&mut self, pid: Pid, signal: Signal) {
        if signal.is_unblockable() { return; }
        if let Some((_, mask)) = self.blocked.iter_mut().find(|(p, _)| *p == pid) {
            *mask |= 1u32 << (signal as u32 as usize);
        }
    }

    /// Unblock a signal
    pub fn unblock(&mut self, pid: Pid, signal: Signal) {
        if let Some((_, mask)) = self.blocked.iter_mut().find(|(p, _)| *p == pid) {
            *mask &= !(1u32 << (signal as u32 as usize));
        }
    }

    /// Check if a signal is blocked
    pub fn is_blocked(&self, pid: Pid, signal: Signal) -> bool {
        if signal.is_unblockable() { return false; }
        self.blocked.iter()
            .find(|(p, _)| *p == pid)
            .map(|(_, mask)| (*mask & (1u32 << (signal as u32 as usize))) != 0)
            .unwrap_or(false)
    }

    /// Send a signal to a process
    pub fn send(&mut self, pid: Pid, signal: Signal, sender: Option<Pid>) -> bool {
        self.signals_sent += 1;
        if let Some((_, pending)) = self.pending.iter_mut().find(|(p, _)| *p == pid) {
            pending.push(PendingSignal { signal, sender, data: 0 });
            return true;
        }
        false
    }

    /// Send a signal with data (sigqueue-like)
    pub fn send_with_data(&mut self, pid: Pid, signal: Signal, sender: Option<Pid>, data: u64) -> bool {
        self.signals_sent += 1;
        if let Some((_, pending)) = self.pending.iter_mut().find(|(p, _)| *p == pid) {
            pending.push(PendingSignal { signal, sender, data });
            return true;
        }
        false
    }

    /// Get pending signals for a process (not blocked)
    pub fn pending_signals(&self, pid: Pid) -> Vec<Signal> {
        self.pending.iter()
            .find(|(p, _)| *p == pid)
            .map(|(_, sigs)| {
                sigs.iter()
                    .filter(|s| !self.is_blocked(pid, s.signal))
                    .map(|s| s.signal)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Deliver the next deliverable signal to a process.
    /// Returns the signal and what action to take.
    pub fn deliver(&mut self, pid: Pid) -> Option<(Signal, SignalDisposition)> {
        let blocked_mask = self.blocked.iter()
            .find(|(p, _)| *p == pid)
            .map(|(_, m)| *m)
            .unwrap_or(0);

        // Find first non-blocked pending signal
        let signal_idx = self.pending.iter_mut()
            .find(|(p, _)| *p == pid)
            .and_then(|(_, sigs)| {
                sigs.iter().position(|s| {
                    !s.signal.is_unblockable() && (blocked_mask & (1u32 << (s.signal as u32 as usize))) == 0
                        || s.signal.is_unblockable()
                })
            });

        if let Some(idx) = signal_idx {
            let signal = self.pending.iter_mut()
                .find(|(p, _)| *p == pid)
                .and_then(|(_, sigs)| {
                    sigs.remove(idx).map(|ps| ps.signal)
                })?;
            self.signals_delivered += 1;
            let disp = self.get_handler(pid, signal);
            Some((signal, disp))
        } else {
            None
        }
    }

    /// Process a delivered signal: return the action to take
    pub fn resolve_action(signal: Signal, disp: SignalDisposition) -> SignalResolution {
        match disp {
            SignalDisposition::Default => match signal.default_action() {
                SignalAction::Terminate     => SignalResolution::Terminate(0),
                SignalAction::TerminateCore => SignalResolution::Terminate(1),
                SignalAction::Stop           => SignalResolution::Stop,
                SignalAction::Continue       => SignalResolution::Continue,
                SignalAction::Ignore          => SignalResolution::Ignore,
            },
            SignalDisposition::Ignore => SignalResolution::Ignore,
            SignalDisposition::Catch { handler_addr } => SignalResolution::CallHandler(handler_addr),
        }
    }

    /// Has pending non-blocked signals?
    pub fn has_pending(&self, pid: Pid) -> bool {
        !self.pending_signals(pid).is_empty()
    }

    /// Statistics
    pub fn signals_sent(&self) -> u64 { self.signals_sent }
    pub fn signals_delivered(&self) -> u64 { self.signals_delivered }
    pub fn registered_count(&self) -> usize { self.handlers.len() }

    /// Clear all pending signals for a process
    pub fn clear_pending(&mut self, pid: Pid) {
        if let Some((_, pending)) = self.pending.iter_mut().find(|(p, _)| *p == pid) {
            pending.clear();
        }
    }
}

/// What to do when a signal is delivered
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SignalResolution {
    /// Terminate the process (exit code 0 = normal, 1 = core dump)
    Terminate(i32),
    /// Stop the process (can be continued with SIGCONT)
    Stop,
    /// Continue the process (if stopped)
    Continue,
    /// Ignore the signal
    Ignore,
    /// Call a user-space handler at the given address
    CallHandler(u64),
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- ELF Parser tests ---

    #[test]
    fn test_parse_minimal_elf() {
        let code = vec![0xF4]; // HLT
        let elf = ElfLoader::create_minimal_elf(0x400000, &code);
        let parser = ElfParser::parse(&elf);
        assert!(parser.is_ok());
        let parser = parser.unwrap();
        assert_eq!(parser.entry_point(), 0x400000);
        assert_eq!(parser.program_headers().len(), 1);
        assert!(parser.program_headers()[0].is_loadable());
    }

    #[test]
    fn test_parse_invalid_magic() {
        let bad = vec![0x00; 128];
        assert_eq!(ElfParser::parse(&bad), Err(ElfParseError::InvalidMagic));
    }

    #[test]
    fn test_parse_truncated() {
        let bad = vec![0x7F, b'E', b'L', b'F'];
        assert_eq!(ElfParser::parse(&bad), Err(ElfParseError::Truncated));
    }

    #[test]
    fn test_parse_wrong_class() {
        let mut elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
        elf[4] = 1; // ELFCLASS32
        assert_eq!(ElfParser::parse(&elf), Err(ElfParseError::InvalidClass));
    }

    #[test]
    fn test_parse_wrong_endianness() {
        let mut elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
        elf[5] = 2; // ELFDATA2MSB (big endian)
        assert_eq!(ElfParser::parse(&elf), Err(ElfParseError::InvalidEndian));
    }

    #[test]
    fn test_parse_wrong_type() {
        let mut elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
        elf[16..18].copy_from_slice(&3u16.to_le_bytes()); // ET_DYN
        assert_eq!(ElfParser::parse(&elf), Err(ElfParseError::InvalidType));
    }

    #[test]
    fn test_parse_wrong_machine() {
        let mut elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
        elf[18..20].copy_from_slice(&3u16.to_le_bytes()); // EM_386
        assert_eq!(ElfParser::parse(&elf), Err(ElfParseError::InvalidMachine));
    }

    #[test]
    fn test_program_header_flags() {
        let elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
        let parser = ElfParser::parse(&elf).unwrap();
        let phdr = &parser.program_headers()[0];
        assert!(phdr.is_loadable());
        assert!(phdr.is_readable());
        assert!(phdr.is_executable());
        assert!(!phdr.is_writable());
    }

    #[test]
    fn test_code_segment_extraction() {
        let code = vec![0x90, 0xF4]; // NOP, HLT
        let elf = ElfLoader::create_minimal_elf(0x400000, &code);
        let parser = ElfParser::parse(&elf).unwrap();
        let code_seg = parser.code_segment();
        assert!(code_seg.is_some());
        let code_seg = code_seg.unwrap();
        assert_eq!(code_seg.p_vaddr, 0x400000);
        assert_eq!(code_seg.p_filesz, 2);
    }

    #[test]
    fn test_segment_data() {
        let code = vec![0x90, 0xF4];
        let elf = ElfLoader::create_minimal_elf(0x400000, &code);
        let parser = ElfParser::parse(&elf).unwrap();
        let code_seg = parser.code_segment().unwrap();
        let data = parser.segment_data(code_seg).unwrap();
        assert_eq!(data, &code[..]);
    }

    #[test]
    fn test_bss_detection() {
        let mut elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
        // Set p_memsz > p_filesz to simulate BSS
        let phdr_off = 64;
        let memsz_offset = phdr_off + 40;
        elf[memsz_offset..memsz_offset+8].copy_from_slice(&100u64.to_le_bytes());
        let parser = ElfParser::parse(&elf).unwrap();
        let seg = &parser.program_headers()[0];
        assert!(seg.needs_bss());
        assert_eq!(seg.bss_size(), 99); // 100 - 1 byte code
    }

    #[test]
    fn test_no_loadable_segments() {
        let mut elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
        // Change p_type to PT_NOTE (not PT_LOAD)
        let phdr_off = 64;
        elf[phdr_off..phdr_off+4].copy_from_slice(&PT_NOTE.to_le_bytes());
        assert_eq!(ElfParser::parse(&elf), Err(ElfParseError::NoLoadableSegments));
    }

    // --- ELF Loader tests ---

    #[test]
    fn test_load_elf_into_userspace() {
        let mut mgr = UserspaceManager::new();
        let code = vec![0xF4];
        let elf = ElfLoader::create_minimal_elf(0x400000, &code);
        let pid = ElfLoader::load_elf(&mut mgr, &elf);
        assert!(pid.is_ok());
        let pid = pid.unwrap();
        assert_eq!(pid, Pid(1000));
        assert_eq!(mgr.user_count(), 1);
        let ctx = mgr.get_context(pid).unwrap();
        assert_eq!(ctx.rip, 0x400000);
    }

    #[test]
    fn test_load_invalid_elf() {
        let mut mgr = UserspaceManager::new();
        let bad = vec![0x00; 128];
        assert!(ElfLoader::load_elf(&mut mgr, &bad).is_err());
    }

    #[test]
    fn test_load_multiple_elf() {
        let mut mgr = UserspaceManager::new();
        for _ in 0..3 {
            let elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
            assert!(ElfLoader::load_elf(&mut mgr, &elf).is_ok());
        }
        assert_eq!(mgr.user_count(), 3);
    }

    // --- Signal tests ---

    #[test]
    fn test_signal_from_u32() {
        assert_eq!(Signal::from_u32(9), Some(Signal::SigKill));
        assert_eq!(Signal::from_u32(11), Some(Signal::SigSegv));
        assert_eq!(Signal::from_u32(99), None);
    }

    #[test]
    fn test_signal_is_unblockable() {
        assert!(Signal::SigKill.is_unblockable());
        assert!(Signal::SigStop.is_unblockable());
        assert!(!Signal::SigTerm.is_unblockable());
        assert!(!Signal::SigSegv.is_unblockable());
    }

    #[test]
    fn test_signal_is_fatal() {
        assert!(Signal::SigKill.is_fatal());
        assert!(Signal::SigTerm.is_fatal());
        assert!(Signal::SigSegv.is_fatal());
        assert!(!Signal::SigStop.is_fatal());
        assert!(!Signal::SigCont.is_fatal());
        assert!(!Signal::SigChld.is_fatal());
    }

    #[test]
    fn test_signal_names() {
        assert_eq!(Signal::SigKill.name(), "SIGKILL");
        assert_eq!(Signal::SigSegv.name(), "SIGSEGV");
        assert_eq!(Signal::SigChld.name(), "SIGCHLD");
    }

    #[test]
    fn test_default_actions() {
        assert_eq!(Signal::SigKill.default_action(), SignalAction::Terminate);
        assert_eq!(Signal::SigSegv.default_action(), SignalAction::TerminateCore);
        assert_eq!(Signal::SigStop.default_action(), SignalAction::Stop);
        assert_eq!(Signal::SigCont.default_action(), SignalAction::Continue);
        assert_eq!(Signal::SigChld.default_action(), SignalAction::Ignore);
    }

    // --- Signal Manager tests ---

    #[test]
    fn test_signal_manager_new() {
        let mgr = SignalManager::new();
        assert_eq!(mgr.registered_count(), 0);
        assert_eq!(mgr.signals_sent(), 0);
        assert_eq!(mgr.signals_delivered(), 0);
    }

    #[test]
    fn test_register_process() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        assert_eq!(mgr.registered_count(), 1);
        // Double register is no-op
        mgr.register(Pid(1000));
        assert_eq!(mgr.registered_count(), 1);
    }

    #[test]
    fn test_unregister_process() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.unregister(Pid(1000));
        assert_eq!(mgr.registered_count(), 0);
    }

    #[test]
    fn test_set_handler() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        assert!(mgr.set_handler(Pid(1000), Signal::SigUsr1, SignalDisposition::Catch { handler_addr: 0x400100 }));
        let h = mgr.get_handler(Pid(1000), Signal::SigUsr1);
        assert_eq!(h, SignalDisposition::Catch { handler_addr: 0x400100 });
    }

    #[test]
    fn test_cannot_set_kill_handler() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        assert!(!mgr.set_handler(Pid(1000), Signal::SigKill, SignalDisposition::Ignore));
        let h = mgr.get_handler(Pid(1000), Signal::SigKill);
        assert_eq!(h, SignalDisposition::Default);
    }

    #[test]
    fn test_block_unblock_signal() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        assert!(!mgr.is_blocked(Pid(1000), Signal::SigTerm));
        mgr.block(Pid(1000), Signal::SigTerm);
        assert!(mgr.is_blocked(Pid(1000), Signal::SigTerm));
        mgr.unblock(Pid(1000), Signal::SigTerm);
        assert!(!mgr.is_blocked(Pid(1000), Signal::SigTerm));
    }

    #[test]
    fn test_cannot_block_kill() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.block(Pid(1000), Signal::SigKill);
        assert!(!mgr.is_blocked(Pid(1000), Signal::SigKill));
    }

    #[test]
    fn test_send_signal() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        assert!(mgr.send(Pid(1000), Signal::SigTerm, None));
        assert_eq!(mgr.signals_sent(), 1);
        assert!(mgr.has_pending(Pid(1000)));
    }

    #[test]
    fn test_send_signal_unregistered() {
        let mut mgr = SignalManager::new();
        assert!(!mgr.send(Pid(9999), Signal::SigTerm, None));
    }

    #[test]
    fn test_send_with_data() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        assert!(mgr.send_with_data(Pid(1000), Signal::SigUsr1, Some(Pid(2000)), 0xDEAD));
        assert_eq!(mgr.signals_sent(), 1);
    }

    #[test]
    fn test_pending_signals() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.send(Pid(1000), Signal::SigTerm, None);
        mgr.send(Pid(1000), Signal::SigUsr1, None);
        let pending = mgr.pending_signals(Pid(1000));
        assert_eq!(pending.len(), 2);
        assert!(pending.contains(&Signal::SigTerm));
        assert!(pending.contains(&Signal::SigUsr1));
    }

    #[test]
    fn test_pending_signals_with_block() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.send(Pid(1000), Signal::SigTerm, None);
        mgr.send(Pid(1000), Signal::SigUsr1, None);
        mgr.block(Pid(1000), Signal::SigTerm);
        let pending = mgr.pending_signals(Pid(1000));
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0], Signal::SigUsr1);
    }

    #[test]
    fn test_deliver_signal() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.send(Pid(1000), Signal::SigTerm, None);
        let delivered = mgr.deliver(Pid(1000));
        assert!(delivered.is_some());
        let (sig, disp) = delivered.unwrap();
        assert_eq!(sig, Signal::SigTerm);
        assert_eq!(disp, SignalDisposition::Default);
        assert_eq!(mgr.signals_delivered(), 1);
        assert!(!mgr.has_pending(Pid(1000)));
    }

    #[test]
    fn test_deliver_blocked_signal() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.send(Pid(1000), Signal::SigTerm, None);
        mgr.block(Pid(1000), Signal::SigTerm);
        // Blocked signal should not be delivered (unless unblockable)
        let delivered = mgr.deliver(Pid(1000));
        assert!(delivered.is_none());
    }

    #[test]
    fn test_deliver_unblockable_signal() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.send(Pid(1000), Signal::SigKill, None);
        // SIGKILL is always delivered, even if "blocked"
        mgr.block(Pid(1000), Signal::SigKill); // has no effect
        let delivered = mgr.deliver(Pid(1000));
        assert!(delivered.is_some());
        let (sig, _) = delivered.unwrap();
        assert_eq!(sig, Signal::SigKill);
    }

    #[test]
    fn test_resolve_action_default() {
        let res = SignalManager::resolve_action(Signal::SigTerm, SignalDisposition::Default);
        assert_eq!(res, SignalResolution::Terminate(0));
    }

    #[test]
    fn test_resolve_action_core_dump() {
        let res = SignalManager::resolve_action(Signal::SigSegv, SignalDisposition::Default);
        assert_eq!(res, SignalResolution::Terminate(1));
    }

    #[test]
    fn test_resolve_action_ignore() {
        let res = SignalManager::resolve_action(Signal::SigTerm, SignalDisposition::Ignore);
        assert_eq!(res, SignalResolution::Ignore);
    }

    #[test]
    fn test_resolve_action_catch() {
        let res = SignalManager::resolve_action(Signal::SigUsr1, SignalDisposition::Catch { handler_addr: 0x400100 });
        assert_eq!(res, SignalResolution::CallHandler(0x400100));
    }

    #[test]
    fn test_resolve_action_stop_continue() {
        assert_eq!(SignalManager::resolve_action(Signal::SigStop, SignalDisposition::Default), SignalResolution::Stop);
        assert_eq!(SignalManager::resolve_action(Signal::SigCont, SignalDisposition::Default), SignalResolution::Continue);
    }

    #[test]
    fn test_clear_pending() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.send(Pid(1000), Signal::SigTerm, None);
        mgr.send(Pid(1000), Signal::SigUsr1, None);
        assert!(mgr.has_pending(Pid(1000)));
        mgr.clear_pending(Pid(1000));
        assert!(!mgr.has_pending(Pid(1000)));
    }

    #[test]
    fn test_multiple_signals_delivery_order() {
        let mut mgr = SignalManager::new();
        mgr.register(Pid(1000));
        mgr.send(Pid(1000), Signal::SigTerm, None);
        mgr.send(Pid(1000), Signal::SigUsr1, None);
        mgr.send(Pid(1000), Signal::SigHup, None);
        // Deliver first (should be SigTerm — first sent)
        let (sig1, _) = mgr.deliver(Pid(1000)).unwrap();
        assert_eq!(sig1, Signal::SigTerm);
        let (sig2, _) = mgr.deliver(Pid(1000)).unwrap();
        assert_eq!(sig2, Signal::SigUsr1);
        let (sig3, _) = mgr.deliver(Pid(1000)).unwrap();
        assert_eq!(sig3, Signal::SigHup);
        assert!(!mgr.has_pending(Pid(1000)));
    }

    #[test]
    fn test_full_signal_lifecycle() {
        let mut mgr = SignalManager::new();
        let pid = Pid(1000);
        mgr.register(pid);
        // Set custom handler for SIGUSR1
        mgr.set_handler(pid, Signal::SigUsr1, SignalDisposition::Catch { handler_addr: 0x500000 });
        // Send SIGUSR1
        mgr.send(pid, Signal::SigUsr1, Some(Pid(2000)));
        assert!(mgr.has_pending(pid));
        // Deliver
        let (sig, disp) = mgr.deliver(pid).unwrap();
        assert_eq!(sig, Signal::SigUsr1);
        let res = SignalManager::resolve_action(sig, disp);
        assert_eq!(res, SignalResolution::CallHandler(0x500000));
        // Send SIGTERM (no handler → default → terminate)
        mgr.send(pid, Signal::SigTerm, None);
        let (sig2, disp2) = mgr.deliver(pid).unwrap();
        assert_eq!(sig2, Signal::SigTerm);
        let res2 = SignalManager::resolve_action(sig2, disp2);
        assert_eq!(res2, SignalResolution::Terminate(0));
        assert_eq!(mgr.signals_sent(), 2);
        assert_eq!(mgr.signals_delivered(), 2);
    }

    #[test]
    fn test_elf_parse_error_display() {
        assert_eq!(format!("{}", ElfParseError::InvalidMagic), "invalid ELF magic");
        assert_eq!(format!("{}", ElfParseError::Truncated), "file truncated");
        assert_eq!(format!("{}", ElfParseError::NoLoadableSegments), "no loadable segments");
    }

    #[test]
    fn test_elf_load_error_display() {
        let e = ElfLoadError::NoCodeSegment;
        assert_eq!(format!("{}", e), "no executable code segment");
    }

    #[test]
    fn test_elf_with_multiple_segments() {
        // Create ELF with 2 PT_LOAD segments (code + data)
        let mut elf = ElfLoader::create_minimal_elf(0x400000, &[0xF4]);
        // Add a second program header for data segment
        let phdr2_off = 64 + 56; // After first Phdr
        // Extend with data Phdr
        elf.extend_from_slice(&[0u8; 56]);
        // p_type = PT_LOAD
        elf[phdr2_off..phdr2_off+4].copy_from_slice(&PT_LOAD.to_le_bytes());
        // p_flags = PF_R | PF_W (readable + writable, not executable)
        elf[phdr2_off+4..phdr2_off+8].copy_from_slice(&(PF_R | PF_W).to_le_bytes());
        // p_vaddr = 0x500000
        elf[phdr2_off+16..phdr2_off+24].copy_from_slice(&0x500000u64.to_le_bytes());
        // p_filesz = 16
        elf[phdr2_off+32..phdr2_off+40].copy_from_slice(&16u64.to_le_bytes());
        // p_memsz = 16
        elf[phdr2_off+40..phdr2_off+48].copy_from_slice(&16u64.to_le_bytes());
        // Update e_phnum = 2
        elf[56..58].copy_from_slice(&2u16.to_le_bytes());
        // Append 16 bytes of data
        elf.extend_from_slice(&[0xAA; 16]);
        // Update p_offset for second segment
        let data_offset = elf.len() - 16;
        elf[phdr2_off+8..phdr2_off+16].copy_from_slice(&(data_offset as u64).to_le_bytes());

        let parser = ElfParser::parse(&elf).unwrap();
        assert_eq!(parser.program_headers().len(), 2);
        let loadable: Vec<_> = parser.loadable_segments().collect();
        assert_eq!(loadable.len(), 2);
        assert!(parser.code_segment().is_some());
        assert!(parser.data_segment().is_some());
    }
}
