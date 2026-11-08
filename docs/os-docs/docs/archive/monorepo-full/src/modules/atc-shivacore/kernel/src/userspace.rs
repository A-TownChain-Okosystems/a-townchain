// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 30: Userspace / Ring-3 Implementation
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// User-Level Prozesse (Ring 3): Privilege-Level-Wechsel, User-Address-Spaces,
// Binary-Loader, User-Context-Verwaltung, Syscall-Entry aus Ring 3.

use crate::ats1000::{Pid, ExitCode};

// ═══════════════════════════════════════════════════════════════════════════════
// Privilege Levels
// ═══════════════════════════════════════════════════════════════════════════════

/// CPU Privilege Levels (Protection Rings)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PrivilegeLevel {
    /// Ring 0 — Kernel mode (full hardware access)
    Kernel = 0,
    /// Ring 3 — User mode (restricted access, syscalls via interrupt gate)
    User = 3,
}

impl PrivilegeLevel {
    pub fn is_kernel(&self) -> bool {
        matches!(self, PrivilegeLevel::Kernel)
    }
    pub fn is_user(&self) -> bool {
        matches!(self, PrivilegeLevel::User)
    }
    pub fn dpl(&self) -> u8 {
        *self as u8
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// User Address Space
// ═══════════════════════════════════════════════════════════════════════════════

/// Layout eines User-Address-Space (flat model, 4 GiB window)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserAddressSpace {
    pub code_base:   u64,
    pub code_size:   u64,
    pub data_base:   u64,
    pub data_size:   u64,
    pub stack_base:  u64,   // Top of stack (grows downward)
    pub stack_size:  u64,
    pub heap_base:   u64,
    pub heap_size:   u64,
}

impl Default for UserAddressSpace {
    fn default() -> Self {
        Self {
            code_base:  0x00400000,   // 4 MiB
            code_size:  0x00100000,   // 1 MiB code region
            data_base:  0x00500000,   // 5 MiB
            data_size:  0x00100000,   // 1 MiB data region
            stack_base: 0x7FFFF000,   // ~2 GiB (top, grows down)
            stack_size: 0x00010000,   // 64 KiB stack
            heap_base:  0x00600000,   // 6 MiB
            heap_size:  0x00200000,   // 2 MiB heap
        }
    }
}

impl UserAddressSpace {
    /// Check if an address falls within any user region
    pub fn contains(&self, addr: u64) -> bool {
        self.in_code(addr) || self.in_data(addr) || self.in_stack(addr) || self.in_heap(addr)
    }
    pub fn in_code(&self, addr: u64) -> bool {
        addr >= self.code_base && addr < self.code_base + self.code_size
    }
    pub fn in_data(&self, addr: u64) -> bool {
        addr >= self.data_base && addr < self.data_base + self.data_size
    }
    pub fn in_stack(&self, addr: u64) -> bool {
        addr > self.stack_base - self.stack_size && addr <= self.stack_base
    }
    pub fn in_heap(&self, addr: u64) -> bool {
        addr >= self.heap_base && addr < self.heap_base + self.heap_size
    }
    /// Stack pointer initial value (top of stack)
    pub fn initial_rsp(&self) -> u64 {
        self.stack_base
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// User Binary (simplified ELF-like format for ShivaCore)
// ═══════════════════════════════════════════════════════════════════════════════

/// A loadable user binary (simplified — code + data + entry point)
#[derive(Clone, Debug)]
pub struct UserBinary {
    pub entry_point: u64,
    pub code:         Vec<u8>,
    pub data:         Vec<u8>,
    pub name:         String,
}

impl UserBinary {
    /// Create a minimal "hello world" binary (just a HLT instruction)
    pub fn hello_world() -> Self {
        Self {
            entry_point: 0x00400000,
            code: vec![0xF4],   // HLT instruction
            data: vec![],
            name: "hello".to_string(),
        }
    }
    /// Create a binary from raw bytes
    pub fn from_bytes(name: &str, code: Vec<u8>, entry: u64) -> Self {
        Self { entry_point: entry, code, data: vec![], name: name.to_string() }
    }
    pub fn code_len(&self) -> usize { self.code.len() }
    pub fn data_len(&self) -> usize { self.data.len() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// User CPU Context (saved state for ring switches)
// ═══════════════════════════════════════════════════════════════════════════════

/// Saved CPU state for a user process (what IRET needs to restore)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserContext {
    pub pid:        Pid,
    pub privilege:  PrivilegeLevel,
    pub rip:        u64,    // Instruction Pointer
    pub rsp:        u64,    // Stack Pointer
    pub rbp:        u64,    // Base Pointer
    pub rax:        u64,    // Return value register
    pub cs:         u16,    // Code Segment selector (ring 3 = 0x1B, ring 0 = 0x08)
    pub ss:         u16,    // Stack Segment selector (ring 3 = 0x23, ring 0 = 0x10)
    pub rflags:     u64,    // CPU flags (IF must be set for user mode)
    pub addr_space: UserAddressSpace,
    pub exit_code:  Option<ExitCode>,
}

impl UserContext {
    /// Create initial context for a new user process
    pub fn new(pid: Pid, binary: &UserBinary, addr_space: UserAddressSpace) -> Self {
        Self {
            pid,
            privilege:    PrivilegeLevel::User,
            rip:          binary.entry_point,
            rsp:          addr_space.initial_rsp(),
            rbp:          addr_space.initial_rsp(),
            rax:          0,
            cs:           0x1B,   // GDT entry 3, ring 3 (0x1B = (3 << 3) | 3)
            ss:           0x23,   // GDT entry 4, ring 3 (0x23 = (4 << 3) | 3)
            rflags:       0x202,  // IF=1 (interrupts enabled), reserved bit 1
            addr_space,
            exit_code:    None,
        }
    }

    /// Check if this context is in user mode
    pub fn is_user_mode(&self) -> bool {
        self.privilege.is_user()
    }
    /// Check if an address is within this process's address space
    pub fn valid_address(&self, addr: u64) -> bool {
        self.addr_space.contains(addr)
    }
    /// Set syscall return value
    pub fn set_return(&mut self, value: u64) {
        self.rax = value;
    }
    /// Mark process as exited
    pub fn exit(&mut self, code: ExitCode) {
        self.exit_code = Some(code);
    }
    /// Check if process has exited
    pub fn is_exited(&self) -> bool {
        self.exit_code.is_some()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// GDT Segment Selectors for Ring 3
// ═══════════════════════════════════════════════════════════════════════════════

/// GDT layout for ShivaCore with user segments:
///   Entry 0: Null
///   Entry 1: Kernel Code (ring 0)  → selector 0x08
///   Entry 2: Kernel Data (ring 0)  → selector 0x10
///   Entry 3: User Code (ring 3)    → selector 0x1B
///   Entry 4: User Data (ring 3)    → selector 0x23
///   Entry 5: TSS                    → selector 0x2B
pub struct GdtSelectors {
    pub kernel_cs: u16,  // 0x08
    pub kernel_ds: u16,  // 0x10
    pub user_cs:    u16,  // 0x1B
    pub user_ds:    u16,  // 0x23
    pub tss:        u16,  // 0x2B
}

impl Default for GdtSelectors {
    fn default() -> Self {
        Self {
            kernel_cs: 0x08,
            kernel_ds: 0x10,
            user_cs:   0x1B,   // (3 << 3) | 3 = 24 + 3 = 0x1B
            user_ds:   0x23,   // (4 << 3) | 3 = 32 + 3 = 0x23
            tss:       0x2B,   // (5 << 3) | 3 = 40 + 3 = 0x2B
        }
    }
}

impl GdtSelectors {
    /// Verify that user selectors have ring 3 DPL
    pub fn verify(&self) -> bool {
        (self.user_cs & 0x03) == 3 && (self.user_ds & 0x03) == 3
            && (self.kernel_cs & 0x03) == 0 && (self.kernel_ds & 0x03) == 0
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Userspace Errors
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UserspaceError {
    ProcessNotFound,
    ProcessAlreadyExited,
    InvalidAddress,
    SegmentFault,
    StackOverflow,
    InvalidBinary,
    PermissionDenied,
    MaxProcessesReached,
}

impl core::fmt::Display for UserspaceError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            UserspaceError::ProcessNotFound      => write!(f, "user process not found"),
            UserspaceError::ProcessAlreadyExited => write!(f, "process already exited"),
            UserspaceError::InvalidAddress        => write!(f, "invalid memory address"),
            UserspaceError::SegmentFault          => write!(f, "segmentation fault"),
            UserspaceError::StackOverflow         => write!(f, "stack overflow"),
            UserspaceError::InvalidBinary         => write!(f, "invalid binary format"),
            UserspaceError::PermissionDenied      => write!(f, "permission denied"),
            UserspaceError::MaxProcessesReached   => write!(f, "max user processes reached"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Userspace Manager
// ═══════════════════════════════════════════════════════════════════════════════

/// Maximum number of concurrent user processes
const MAX_USER_PROCESSES: usize = 64;

pub struct UserspaceManager {
    users:        Vec<UserContext>,
    next_pid:     u32,
    syscall_count: u64,
    gdt:          GdtSelectors,
}

impl Default for UserspaceManager {
    fn default() -> Self { Self::new() }
}

impl UserspaceManager {
    pub fn new() -> Self {
        Self {
            users:         Vec::new(),
            next_pid:      1000,    // User PIDs start at 1000
            syscall_count: 0,
            gdt:           GdtSelectors::default(),
        }
    }

    /// Load a user binary and create a user process (ring 3)
    pub fn load_binary(&mut self, binary: UserBinary) -> Result<Pid, UserspaceError> {
        if self.users.len() >= MAX_USER_PROCESSES {
            return Err(UserspaceError::MaxProcessesReached);
        }
        if binary.code.is_empty() {
            return Err(UserspaceError::InvalidBinary);
        }
        let pid = Pid(self.next_pid);
        self.next_pid += 1;
        let addr_space = UserAddressSpace::default();
        let ctx = UserContext::new(pid, &binary, addr_space);
        self.users.push(ctx);
        Ok(pid)
    }

    /// Get the context for a user process
    pub fn get_context(&self, pid: Pid) -> Option<&UserContext> {
        self.users.iter().find(|u| u.pid == pid)
    }
    pub fn get_context_mut(&mut self, pid: Pid) -> Option<&mut UserContext> {
        self.users.iter_mut().find(|u| u.pid == pid)
    }

    /// Prepare ring switch: validate context and return selectors
    pub fn enter_userspace(&self, pid: Pid) -> Result<&UserContext, UserspaceError> {
        let ctx = self.users.iter().find(|u| u.pid == pid)
            .ok_or(UserspaceError::ProcessNotFound)?;
        if ctx.is_exited() {
            return Err(UserspaceError::ProcessAlreadyExited);
        }
        if !ctx.privilege.is_user() {
            return Err(UserspaceError::PermissionDenied);
        }
        if !ctx.valid_address(ctx.rip) {
            return Err(UserspaceError::SegmentFault);
        }
        if !ctx.valid_address(ctx.rsp) {
            return Err(UserspaceError::StackOverflow);
        }
        Ok(ctx)
    }

    /// Handle a syscall from user mode
    pub fn handle_syscall(&mut self, pid: Pid, num: u32, _args: &[u64]) -> Result<u64, UserspaceError> {
        self.syscall_count += 1;
        let ctx = self.users.iter_mut().find(|u| u.pid == pid)
            .ok_or(UserspaceError::ProcessNotFound)?;
        if ctx.is_exited() {
            return Err(UserspaceError::ProcessAlreadyExited);
        }
        // Simulate syscall execution
        match num {
            0 => { ctx.exit(0); Ok(0) }             // exit(0)
            1 => { Ok(ctx.rsp) }                      // get_stack_ptr
            2 => { Ok(ctx.rip) }                      // get_instruction_ptr
            3 => { Ok(self.users.len() as u64) }      // get_process_count
            _ => Err(UserspaceError::PermissionDenied),
        }
    }

    /// Exit a user process
    pub fn exit_process(&mut self, pid: Pid, code: ExitCode) -> bool {
        if let Some(ctx) = self.get_context_mut(pid) {
            if ctx.is_exited() { return false; }
            ctx.exit(code);
            return true;
        }
        false
    }

    /// Remove exited processes (cleanup)
    pub fn reap_dead(&mut self) -> usize {
        let before = self.users.len();
        self.users.retain(|u| !u.is_exited());
        before - self.users.len()
    }

    /// List all user processes
    pub fn list_users(&self) -> Vec<&UserContext> { self.users.iter().collect() }
    pub fn user_count(&self) -> usize { self.users.len() }
    pub fn active_count(&self) -> usize { self.users.iter().filter(|u| !u.is_exited()).count() }
    pub fn syscall_count(&self) -> u64 { self.syscall_count }
    pub fn gdt_selectors(&self) -> &GdtSelectors { &self.gdt }

    /// Validate a memory access from user space (bounds check)
    pub fn check_memory_access(&self, pid: Pid, addr: u64, size: u64) -> Result<(), UserspaceError> {
        let ctx = self.get_context(pid).ok_or(UserspaceError::ProcessNotFound)?;
        if ctx.is_exited() { return Err(UserspaceError::ProcessAlreadyExited); }
        let end = addr.checked_add(size).ok_or(UserspaceError::InvalidAddress)?;
        if !ctx.valid_address(addr) || !ctx.valid_address(end - 1) {
            return Err(UserspaceError::SegmentFault);
        }
        Ok(())
    }

    /// Simulate stack push from user mode
    pub fn push_stack(&mut self, pid: Pid, value: u64) -> Result<(), UserspaceError> {
        let ctx = self.get_context_mut(pid).ok_or(UserspaceError::ProcessNotFound)?;
        if ctx.is_exited() { return Err(UserspaceError::ProcessAlreadyExited); }
        let new_rsp = ctx.rsp.checked_sub(8).ok_or(UserspaceError::StackOverflow)?;
        if !ctx.addr_space.in_stack(new_rsp) {
            return Err(UserspaceError::StackOverflow);
        }
        ctx.rsp = new_rsp;
        Ok(())
    }

    /// Simulate stack pop from user mode
    pub fn pop_stack(&mut self, pid: Pid) -> Result<u64, UserspaceError> {
        let ctx = self.get_context_mut(pid).ok_or(UserspaceError::ProcessNotFound)?;
        if ctx.is_exited() { return Err(UserspaceError::ProcessAlreadyExited); }
        if !ctx.addr_space.in_stack(ctx.rsp) {
            return Err(UserspaceError::StackOverflow);
        }
        let val = ctx.rax;  // Simulated pop goes to rax
        ctx.rsp += 8;
        if ctx.rsp > ctx.addr_space.stack_base {
            ctx.rsp = ctx.addr_space.stack_base;
        }
        Ok(val)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- PrivilegeLevel tests ---

    #[test]
    fn test_privilege_levels() {
        assert_eq!(PrivilegeLevel::Kernel.dpl(), 0);
        assert_eq!(PrivilegeLevel::User.dpl(), 3);
        assert!(PrivilegeLevel::Kernel.is_kernel());
        assert!(PrivilegeLevel::User.is_user());
        assert!(!PrivilegeLevel::Kernel.is_user());
        assert!(!PrivilegeLevel::User.is_kernel());
    }

    // --- UserAddressSpace tests ---

    #[test]
    fn test_default_address_space() {
        let asp = UserAddressSpace::default();
        assert_eq!(asp.code_base, 0x00400000);
        assert_eq!(asp.stack_base, 0x7FFFF000);
        assert!(asp.code_size > 0);
        assert!(asp.stack_size > 0);
    }

    #[test]
    fn test_address_space_contains() {
        let asp = UserAddressSpace::default();
        assert!(asp.in_code(0x00400000));
        assert!(asp.in_code(0x004FFFFF));
        assert!(!asp.in_code(0x00500000));
        assert!(asp.in_data(0x00500000));
        assert!(asp.in_stack(0x7FFFEFFF));
        assert!(asp.in_stack(0x7FFFF000));
        assert!(asp.in_heap(0x00600000));
        assert!(!asp.in_heap(0x005FFFFF));
    }

    #[test]
    fn test_initial_rsp() {
        let asp = UserAddressSpace::default();
        assert_eq!(asp.initial_rsp(), 0x7FFFF000);
    }

    #[test]
    fn test_address_outside_space() {
        let asp = UserAddressSpace::default();
        assert!(!asp.contains(0x00000000));
        assert!(!asp.contains(0xFFFFFFFF));
        assert!(!asp.contains(0x003FFFFF));
    }

    // --- UserBinary tests ---

    #[test]
    fn test_hello_world_binary() {
        let bin = UserBinary::hello_world();
        assert_eq!(bin.entry_point, 0x00400000);
        assert_eq!(bin.code, vec![0xF4]);
        assert_eq!(bin.name, "hello");
        assert_eq!(bin.code_len(), 1);
        assert_eq!(bin.data_len(), 0);
    }

    #[test]
    fn test_binary_from_bytes() {
        let code = vec![0x90, 0xF4]; // NOP, HLT
        let bin = UserBinary::from_bytes("test", code.clone(), 0x00400000);
        assert_eq!(bin.code, code);
        assert_eq!(bin.name, "test");
        assert_eq!(bin.code_len(), 2);
    }

    // --- UserContext tests ---

    #[test]
    fn test_user_context_creation() {
        let bin = UserBinary::hello_world();
        let asp = UserAddressSpace::default();
        let ctx = UserContext::new(Pid(1000), &bin, asp);
        assert_eq!(ctx.pid, Pid(1000));
        assert!(ctx.is_user_mode());
        assert_eq!(ctx.rip, 0x00400000);
        assert_eq!(ctx.rsp, 0x7FFFF000);
        assert_eq!(ctx.cs, 0x1B);
        assert_eq!(ctx.ss, 0x23);
        assert!(!ctx.is_exited());
    }

    #[test]
    fn test_user_context_exit() {
        let bin = UserBinary::hello_world();
        let asp = UserAddressSpace::default();
        let mut ctx = UserContext::new(Pid(1001), &bin, asp);
        assert!(!ctx.is_exited());
        ctx.exit(42);
        assert!(ctx.is_exited());
        assert_eq!(ctx.exit_code, Some(42));
    }

    #[test]
    fn test_user_context_set_return() {
        let bin = UserBinary::hello_world();
        let asp = UserAddressSpace::default();
        let mut ctx = UserContext::new(Pid(1002), &bin, asp);
        assert_eq!(ctx.rax, 0);
        ctx.set_return(0x1234);
        assert_eq!(ctx.rax, 0x1234);
    }

    #[test]
    fn test_user_context_valid_address() {
        let bin = UserBinary::hello_world();
        let asp = UserAddressSpace::default();
        let ctx = UserContext::new(Pid(1003), &bin, asp);
        assert!(ctx.valid_address(0x00400000));
        assert!(ctx.valid_address(0x7FFFEFFF));
        assert!(!ctx.valid_address(0x00000000));
    }

    // --- GdtSelectors tests ---

    #[test]
    fn test_gdt_selectors_default() {
        let gdt = GdtSelectors::default();
        assert_eq!(gdt.kernel_cs, 0x08);
        assert_eq!(gdt.kernel_ds, 0x10);
        assert_eq!(gdt.user_cs, 0x1B);
        assert_eq!(gdt.user_ds, 0x23);
    }

    #[test]
    fn test_gdt_selectors_verify() {
        let gdt = GdtSelectors::default();
        assert!(gdt.verify());
        // User selectors must have ring 3 DPL
        assert_eq!(gdt.user_cs & 0x03, 3);
        assert_eq!(gdt.user_ds & 0x03, 3);
        // Kernel selectors must have ring 0 DPL
        assert_eq!(gdt.kernel_cs & 0x03, 0);
        assert_eq!(gdt.kernel_ds & 0x03, 0);
    }

    // --- UserspaceManager tests ---

    #[test]
    fn test_manager_new() {
        let mgr = UserspaceManager::new();
        assert_eq!(mgr.user_count(), 0);
        assert_eq!(mgr.syscall_count(), 0);
        assert!(mgr.gdt_selectors().verify());
    }

    #[test]
    fn test_load_binary_creates_user() {
        let mut mgr = UserspaceManager::new();
        let bin = UserBinary::hello_world();
        let pid = mgr.load_binary(bin).unwrap();
        assert_eq!(pid, Pid(1000));
        assert_eq!(mgr.user_count(), 1);
        assert_eq!(mgr.active_count(), 1);
    }

    #[test]
    fn test_load_multiple_binaries() {
        let mut mgr = UserspaceManager::new();
        for i in 0..5 {
            let bin = UserBinary::from_bytes(&format!("proc{}", i), vec![0xF4], 0x00400000);
            let pid = mgr.load_binary(bin).unwrap();
            assert_eq!(pid, Pid(1000 + i));
        }
        assert_eq!(mgr.user_count(), 5);
        assert_eq!(mgr.active_count(), 5);
    }

    #[test]
    fn test_load_empty_binary_fails() {
        let mut mgr = UserspaceManager::new();
        let bin = UserBinary::from_bytes("empty", vec![], 0);
        assert_eq!(mgr.load_binary(bin), Err(UserspaceError::InvalidBinary));
    }

    #[test]
    fn test_enter_userspace_valid() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let ctx = mgr.enter_userspace(pid);
        assert!(ctx.is_ok());
        let ctx = ctx.unwrap();
        assert_eq!(ctx.rip, 0x00400000);
        assert_eq!(ctx.rsp, 0x7FFFF000);
    }

    #[test]
    fn test_enter_userspace_not_found() {
        let mgr = UserspaceManager::new();
        assert_eq!(mgr.enter_userspace(Pid(9999)), Err(UserspaceError::ProcessNotFound));
    }

    #[test]
    fn test_enter_userspace_exited() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        mgr.exit_process(pid, 0);
        assert_eq!(mgr.enter_userspace(pid), Err(UserspaceError::ProcessAlreadyExited));
    }

    #[test]
    fn test_handle_syscall_exit() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let result = mgr.handle_syscall(pid, 0, &[]); // syscall 0 = exit
        assert_eq!(result, Ok(0));
        assert_eq!(mgr.syscall_count(), 1);
        let ctx = mgr.get_context(pid).unwrap();
        assert!(ctx.is_exited());
    }

    #[test]
    fn test_handle_syscall_get_info() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let rsp = mgr.handle_syscall(pid, 1, &[]).unwrap();
        assert_eq!(rsp, 0x7FFFF000);
        let rip = mgr.handle_syscall(pid, 2, &[]).unwrap();
        assert_eq!(rip, 0x00400000);
        let count = mgr.handle_syscall(pid, 3, &[]).unwrap();
        assert_eq!(count, 1);
        assert_eq!(mgr.syscall_count(), 3);
    }

    #[test]
    fn test_handle_syscall_unknown() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        assert_eq!(mgr.handle_syscall(pid, 99, &[]), Err(UserspaceError::PermissionDenied));
    }

    #[test]
    fn test_handle_syscall_exited_process() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        mgr.exit_process(pid, 0);
        assert_eq!(mgr.handle_syscall(pid, 1, &[]), Err(UserspaceError::ProcessAlreadyExited));
    }

    #[test]
    fn test_exit_process() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        assert!(mgr.exit_process(pid, 42));
        let ctx = mgr.get_context(pid).unwrap();
        assert!(ctx.is_exited());
        assert_eq!(ctx.exit_code, Some(42));
        assert_eq!(mgr.active_count(), 0);
    }

    #[test]
    fn test_exit_process_twice() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        assert!(mgr.exit_process(pid, 0));
        assert!(!mgr.exit_process(pid, 0)); // Already exited
    }

    #[test]
    fn test_exit_nonexistent() {
        let mut mgr = UserspaceManager::new();
        assert!(!mgr.exit_process(Pid(9999), 0));
    }

    #[test]
    fn test_reap_dead() {
        let mut mgr = UserspaceManager::new();
        let p1 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let p2 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let _p3 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        mgr.exit_process(p1, 0);
        mgr.exit_process(p2, 1);
        assert_eq!(mgr.user_count(), 3);
        assert_eq!(mgr.active_count(), 1);
        let reaped = mgr.reap_dead();
        assert_eq!(reaped, 2);
        assert_eq!(mgr.user_count(), 1);
        assert_eq!(mgr.active_count(), 1);
    }

    #[test]
    fn test_reap_no_dead() {
        let mut mgr = UserspaceManager::new();
        let _p1 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let _p2 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        assert_eq!(mgr.reap_dead(), 0);
        assert_eq!(mgr.user_count(), 2);
    }

    #[test]
    fn test_list_users() {
        let mut mgr = UserspaceManager::new();
        let _p1 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let _p2 = mgr.load_binary(UserBinary::from_bytes("b", vec![0x90], 0x400000)).unwrap();
        let users = mgr.list_users();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].pid, Pid(1000));
        assert_eq!(users[1].pid, Pid(1001));
    }

    #[test]
    fn test_check_memory_access_valid() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        assert!(mgr.check_memory_access(pid, 0x00400000, 4).is_ok());
        assert!(mgr.check_memory_access(pid, 0x7FFFEFFF, 1).is_ok());
    }

    #[test]
    fn test_check_memory_access_invalid() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        assert_eq!(mgr.check_memory_access(pid, 0x00000000, 4), Err(UserspaceError::SegmentFault));
        assert_eq!(mgr.check_memory_access(pid, 0xFFFFFFFF, 1), Err(UserspaceError::SegmentFault));
    }

    #[test]
    fn test_check_memory_access_exited() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        mgr.exit_process(pid, 0);
        assert_eq!(mgr.check_memory_access(pid, 0x00400000, 4), Err(UserspaceError::ProcessAlreadyExited));
    }

    #[test]
    fn test_push_stack() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let ctx = mgr.get_context(pid).unwrap();
        let initial_rsp = ctx.rsp;
        assert!(mgr.push_stack(pid, 0x1234).is_ok());
        let ctx = mgr.get_context(pid).unwrap();
        assert_eq!(ctx.rsp, initial_rsp - 8);
    }

    #[test]
    fn test_pop_stack() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        mgr.push_stack(pid, 0x5678).unwrap();
        let val = mgr.pop_stack(pid).unwrap();
        let ctx = mgr.get_context(pid).unwrap();
        assert_eq!(ctx.rsp, 0x7FFFF000); // Back to original
    }

    #[test]
    fn test_stack_overflow_protection() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        // Exhaust stack by pushing past the limit
        let ctx = mgr.get_context(pid).unwrap();
        let stack_bottom = ctx.addr_space.stack_base - ctx.addr_space.stack_size;
        // Set rsp to just above stack bottom
        if let Some(c) = mgr.get_context_mut(pid) {
            c.rsp = stack_bottom + 8;
        }
        // This push should overflow
        assert_eq!(mgr.push_stack(pid, 0), Err(UserspaceError::StackOverflow));
    }

    #[test]
    fn test_max_user_processes() {
        let mut mgr = UserspaceManager::new();
        // Load MAX_USER_PROCESSES (64) processes
        for _ in 0..MAX_USER_PROCESSES {
            let bin = UserBinary::hello_world();
            assert!(mgr.load_binary(bin).is_ok());
        }
        assert_eq!(mgr.user_count(), MAX_USER_PROCESSES);
        // Next one should fail
        let bin = UserBinary::hello_world();
        assert_eq!(mgr.load_binary(bin), Err(UserspaceError::MaxProcessesReached));
    }

    #[test]
    fn test_pid_increment() {
        let mut mgr = UserspaceManager::new();
        let p1 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let p2 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let p3 = mgr.load_binary(UserBinary::hello_world()).unwrap();
        assert_eq!(p1, Pid(1000));
        assert_eq!(p2, Pid(1001));
        assert_eq!(p3, Pid(1002));
    }

    #[test]
    fn test_get_context_mut() {
        let mut mgr = UserspaceManager::new();
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        let ctx = mgr.get_context_mut(pid).unwrap();
        ctx.rax = 0xDEAD;
        let ctx = mgr.get_context(pid).unwrap();
        assert_eq!(ctx.rax, 0xDEAD);
    }

    #[test]
    fn test_full_lifecycle() {
        let mut mgr = UserspaceManager::new();
        // 1. Load binary
        let pid = mgr.load_binary(UserBinary::hello_world()).unwrap();
        assert_eq!(mgr.active_count(), 1);
        // 2. Enter userspace (validate)
        let ctx = mgr.enter_userspace(pid).unwrap();
        assert!(ctx.is_user_mode());
        // 3. Handle syscalls
        let rsp = mgr.handle_syscall(pid, 1, &[]).unwrap();
        assert!(rsp > 0);
        // 4. Check memory
        assert!(mgr.check_memory_access(pid, ctx.rip, 1).is_ok());
        // 5. Exit
        assert!(mgr.exit_process(pid, 0));
        assert_eq!(mgr.active_count(), 0);
        // 6. Reap
        assert_eq!(mgr.reap_dead(), 1);
        assert_eq!(mgr.user_count(), 0);
    }

    #[test]
    fn test_userspace_error_display() {
        assert_eq!(format!("{}", UserspaceError::ProcessNotFound), "user process not found");
        assert_eq!(format!("{}", UserspaceError::SegmentFault), "segmentation fault");
        assert_eq!(format!("{}", UserspaceError::StackOverflow), "stack overflow");
        assert_eq!(format!("{}", UserspaceError::InvalidBinary), "invalid binary format");
        assert_eq!(format!("{}", UserspaceError::MaxProcessesReached), "max user processes reached");
    }
}
