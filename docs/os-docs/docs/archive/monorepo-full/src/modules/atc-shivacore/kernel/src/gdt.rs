// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — Global Descriptor Table + Task State Segment.
// K-Sprint 1: Stellt einen dedizierten Stack (IST) fuer Double-Fault-Handler
// bereit, damit ein Stack-Overflow nicht zu einem Triple-Fault (Reboot-Loop)
// fuehrt, sondern sauber als Double-Fault abgefangen werden kann.

use lazy_static::lazy_static;
use x86_64::instructions::segmentation::{Segment, CS, SS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

const STACK_SIZE: usize = 4096 * 5;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // Statischer Stack (kein Heap -- Heap gibt es erst ab K-Sprint 2).
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(core::ptr::addr_of!(STACK));
            stack_start + STACK_SIZE as u64
        };
        tss
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
        // WICHTIG: Der alte SS-Selektor (vom Bootloader-eigenen GDT) zeigt nach
        // dem Laden unseres neuen, minimalen GDT ins Leere/auf einen ungueltigen
        // Deskriptor. Beim naechsten IRETQ (z.B. Rueckkehr aus einem Interrupt-
        // Handler) wird SS zwingend neu geladen und validiert -- mit dem alten
        // Wert fuehrt das zu #GP waehrend des IRETQ, was der Prozessor als
        // Double Fault eskaliert. Long-Mode erlaubt bei CPL0 explizit einen
        // Null-Selektor fuer SS (Stack-Segment wird im Flat-Modell ohnehin
        // nicht ausgewertet) -- das behebt den Double Fault sauber.
        SS::set_reg(SegmentSelector::NULL);
    }
}
