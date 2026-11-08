// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — Interrupt Descriptor Table + PIC-Remapping.
// K-Sprint 1: Breakpoint-, Double-Fault- und Page-Fault-Handler; die beiden
// klassischen 8259-PICs werden von ihren BIOS-Default-Vektoren (0x08-0x0F,
// 0x70-0x77 -- kollidieren mit CPU-Exceptions) auf 0x20-0x2F umgemappt.

use crate::gdt;
use crate::serial_println;
use core::arch::global_asm;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use shivacore::user_sched::{SavedContext, UserScheduler};
use shivacore::userspace::UserContext;
use spin::Mutex;
use x86_64::registers::rflags::RFlags;
use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame, InterruptStackFrameValue, PageFaultErrorCode,
};
use x86_64::VirtAddr;

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

static USER_SCHEDULER: Mutex<Option<UserScheduler>> = Mutex::new(None);

pub fn init_user_scheduler(first: &UserContext, second: &UserContext) {
    let mut scheduler = UserScheduler::new();
    scheduler.add_process(first.pid, first, 0);
    scheduler.add_process(second.pid, second, 0);
    let _ = scheduler.schedule(None);
    *USER_SCHEDULER.lock() = Some(scheduler);
    serial_println!(
        "ShivaCore: SCHED-001 scheduler armed: PID={} + PID={}",
        first.pid.0,
        second.pid.0
    );
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        unsafe {
            idt[InterruptIndex::Timer.as_u8()]
                .set_handler_addr(timer_interrupt_entry_addr())
        };
        idt[InterruptIndex::Keyboard.as_u8()].set_handler_fn(keyboard_interrupt_handler);
        // User-mode software interrupt gate. DPL3 permits CPL3 to enter the kernel.
        idt[0x80].set_handler_fn(syscall_interrupt_handler)
            .set_privilege_level(x86_64::PrivilegeLevel::Ring3);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub fn init_pics() {
    unsafe {
        PICS.lock().initialize();

        // UEFI firmware may leave IRQ0 masked in the legacy PIC. Make the
        // timer IRQ unmasked explicitly; keep all other legacy PIC IRQs masked
        // until their handlers and device initialization are ready.
        PICS.lock().write_masks(0xFE, 0xFF);

        // Program channel 0 explicitly so IRQ0 drives scheduler preemption
        // consistently in both BIOS and UEFI boots (100 Hz).
        const PIT_INPUT_HZ: u32 = 1_193_182;
        const PIT_HZ: u32 = 100;
        let divisor = (PIT_INPUT_HZ / PIT_HZ) as u16;
        let mut command: x86_64::instructions::port::Port<u8> =
            x86_64::instructions::port::Port::new(0x43);
        let mut channel0: x86_64::instructions::port::Port<u8> =
            x86_64::instructions::port::Port::new(0x40);
        command.write(0x36); // channel 0, lobyte/hibyte, mode 3, binary
        channel0.write((divisor & 0x00ff) as u8);
        channel0.write((divisor >> 8) as u8);
    }
    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    serial_println!("EXCEPTION: PAGE FAULT");
    serial_println!("Accessed Address: {:?}", Cr2::read());
    serial_println!("Error Code: {:?}", error_code);
    serial_println!("{:#?}", stack_frame);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct UserTrapFrame {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl UserTrapFrame {
    fn saved_context(&self) -> SavedContext {
        SavedContext {
            regs: shivacore::user_sched::SavedRegisters {
                rax: self.rax,
                rbx: self.rbx,
                rcx: self.rcx,
                rdx: self.rdx,
                rsi: self.rsi,
                rdi: self.rdi,
                rbp: self.rbp,
                rsp: self.rsp,
                r8: self.r8,
                r9: self.r9,
                r10: self.r10,
                r11: self.r11,
                r12: self.r12,
                r13: self.r13,
                r14: self.r14,
                r15: self.r15,
            },
            iret: shivacore::user_sched::IretFrame {
                rip: self.rip,
                cs: self.cs as u16,
                rflags: self.rflags,
                rsp: self.rsp,
                ss: self.ss as u16,
            },
        }
    }

    fn restore(&mut self, ctx: SavedContext) {
        self.rax = ctx.regs.rax;
        self.rbx = ctx.regs.rbx;
        self.rcx = ctx.regs.rcx;
        self.rdx = ctx.regs.rdx;
        self.rsi = ctx.regs.rsi;
        self.rdi = ctx.regs.rdi;
        self.rbp = ctx.regs.rbp;
        self.r8 = ctx.regs.r8;
        self.r9 = ctx.regs.r9;
        self.r10 = ctx.regs.r10;
        self.r11 = ctx.regs.r11;
        self.r12 = ctx.regs.r12;
        self.r13 = ctx.regs.r13;
        self.r14 = ctx.regs.r14;
        self.r15 = ctx.regs.r15;
        self.rip = ctx.iret.rip;
        self.cs = ctx.iret.cs as u64;
        self.rflags = ctx.iret.rflags;
        self.rsp = ctx.iret.rsp;
        self.ss = ctx.iret.ss as u64;
    }
}

extern "C" {
    fn shivacore_timer_trampoline();
}

global_asm!(
    r#"
    .global shivacore_timer_trampoline
shivacore_timer_trampoline:
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rbp
    push rdi
    push rsi
    push rdx
    push rcx
    push rbx
    push rax
    mov rdi, rsp
    call {handler}
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rbp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
    iretq
"#,
    handler = sym timer_trap_rust
);

#[no_mangle]
extern "C" fn timer_trap_rust(frame: &mut UserTrapFrame) {
    let mut guard = USER_SCHEDULER.lock();
    let Some(scheduler) = guard.as_mut() else {
        unsafe {
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
        }
        return;
    };

    let current_pid = scheduler.current_pid();
    let current = frame.saved_context();

    // SCHED-001 runtime invariant: each Ring-3 smoke process owns a distinct
    // register pattern. A mismatch fails the QEMU gate instead of masking
    // register-save/restore corruption.
    if let Some(pid) = current_pid {
        let (rax, rbx, r12) = match pid.0 {
            1000 => (0x1000, 0x1001, 0x1012),
            1001 => (0x2000, 0x2001, 0x2012),
            _ => (current.regs.rax, current.regs.rbx, current.regs.r12),
        };
        if current.regs.rax != rax || current.regs.rbx != rbx || current.regs.r12 != r12 {
            panic!(
                "SCHED-001 register corruption: PID={} RAX={:#x} RBX={:#x} R12={:#x}",
                pid.0, current.regs.rax, current.regs.rbx, current.regs.r12
            );
        }
        serial_println!(
            "ShivaCore: SCHED-001 register checkpoint PID={} RAX={:#x} RBX={:#x} R12={:#x}",
            pid.0,
            current.regs.rax,
            current.regs.rbx,
            current.regs.r12
        );
    }

    if let Some((next_pid, next)) = scheduler.timer_tick_saved(current) {
        serial_println!(
            "ShivaCore: scheduler preemption -> context switch to PID={} RIP={:#x} RAX={:#x}",
            next_pid.0,
            next.iret.rip,
            next.regs.rax
        );
        frame.restore(next);
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

fn timer_interrupt_entry_addr() -> VirtAddr {
    VirtAddr::new(shivacore_timer_trampoline as usize as u64)
}

extern "x86-interrupt" fn syscall_interrupt_handler(stack_frame: InterruptStackFrame) {
    serial_println!(
        "ShivaCore: CPL3 syscall gate entered at RIP={:#x}",
        stack_frame.instruction_pointer.as_u64()
    );
    // Return to the interrupted userspace instruction for now. The ABI dispatcher
    // is wired next; this gate is intentionally observable before SCHED-001/SYS-001.
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let mut port: Port<u8> = Port::new(0x60);
    let _scancode: u8 = unsafe { port.read() };
    // Vollstaendiges Scancode->Keycode-Mapping folgt in einem spaeteren Sprint.
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
