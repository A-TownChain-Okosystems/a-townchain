export interface PresetCategory {
  name: string;
  color: string;
}

export interface PresetTask {
  text: string;
  category: string;
  priority: 'Low' | 'Medium' | 'High';
  subtasks: string[];
  notes: string;
  dueDateOffsetDays?: number;
}

export interface ProjectPreset {
  id: string;
  title: string;
  categoryBadge: string;
  icon: string;
  description: string;
  categories: PresetCategory[];
  tasks: PresetTask[];
}

export interface SubtaskTemplatePreset {
  id: string;
  name: string;
  category: string;
  subtasks: string[];
}

export const KERNEL_SUBTASK_PRESETS: SubtaskTemplatePreset[] = [
  {
    id: 'k-idt',
    name: 'IDT & Exception Handling (Bare Metal)',
    category: 'Kernel',
    subtasks: [
      'GDT (Global Descriptor Table) und 64-Bit TSS mit IST1 definieren',
      'IDT-Struktur (256 Entries) im Kernel BSS anlegen',
      'Assembly ISR-Stubs (isr0 bis isr31) für CPU-Exceptions erstellen',
      'Error-Code Push/Dummy-Push für Ausnahme-Handling angleichen',
      'Exceptions in C/Rust Handler routen & Register-Dump ausgeben',
      'lidt Instruction laden und Interrupts mit sti testen'
    ]
  },
  {
    id: 'k-paging',
    name: '4-Level Paging & Virtual Memory',
    category: 'Memory',
    subtasks: [
      'E820 BIOS / Multiboot2 Memory Map parsen',
      'Physical Frame Bitmap Allocator (4KB Blöcke) initialisieren',
      'PML4, PDPT, PD und PT Root-Tabellen 4KB-aligned allozieren',
      'Kernel Identity-Mapping (untere 4MB / 1GB) einrichten',
      'Higher-Half Kernel Mapping (0xFFFF800000000000) konfigurieren',
      'CR3 Control Register laden und Paging Flag in CR0/CR4 setzen',
      'Page Fault (#PF, Vector 14) Exception Handler mit CR2 Logging testen'
    ]
  },
  {
    id: 'k-uart',
    name: 'Early Serial UART 16550 Driver',
    category: 'Drivers',
    subtasks: [
      'COM1 I/O Port Base (0x3F8) definieren',
      'DLAB Bit in Line Control Register setzen für Baudrate (115200 Baud)',
      'Divisor Latch Low/High (0x01, 0x00) beschreiben',
      'Line Control: 8 Bits, keine Parität, 1 Stop-Bit konfigurieren',
      'FIFO Control Register (14-Byte Threshold, FIFO enable) aktivieren',
      'uart_putc() und uart_puts() mit Line Status Register Polling implementieren',
      'Early kprintf() Formatter für Bootloader-Meldungen verknüpfen'
    ]
  },
  {
    id: 'k-heap',
    name: 'Kernel Heap Allocator (kmalloc/kfree)',
    category: 'Memory',
    subtasks: [
      'Virtuellen Heap-Bereich (z.B. 0xFFFF900000000000, 32MB) reservieren',
      'Slab / Free-List Block-Header (magic, size, is_free, next) strukturieren',
      'kmalloc(size_t) mit First-Fit / Best-Fit Allokationslogik schreiben',
      'Block-Splitting bei übergroßen freien Blöcken implementieren',
      'kfree(void* ptr) mit Block-Merging (Coalescing) zur Defragmentierung bauen',
      'Unit-Tests für aufeinanderfolgende Alloc/Free Zyklen durchführen'
    ]
  },
  {
    id: 'k-sched',
    name: 'Preemptive Task Scheduler & Context Switch',
    category: 'Arch',
    subtasks: [
      'Process Control Block (PCB) & Thread-Struktur mit Stack-Pointer anlegen',
      'Initialen Stack Frame für neuen Task (RIP, CS, RFLAGS, RSP, SS) präparieren',
      'Assembly switch_to(prev_task, next_task) für Register-Sicherung (RBX, RBP, R12-R15) schreiben',
      'PIT / APIC Timer Interrupt @ 100Hz an Scheduler Hook binden',
      'Round-Robin Ready-Queue mit Task-Zuständen (READY, RUNNING, SLEEPING, ZOMBIE) bauen',
      'Kooperatives yield() und preemptives Timer-Tick Switching testen'
    ]
  },
  {
    id: 'k-syscall',
    name: 'Ring 3 User Mode & Syscall Handler',
    category: 'Kernel',
    subtasks: [
      'GDT um User Code (Ring 3, DPL 3) und User Data Descriptoren erweitern',
      'IA32_EFER MSR (SCE Bit 0) für SYSCALL/SYSRET aktivieren',
      'IA32_LSTAR MSR mit Kernel Syscall Entrypoint-Adresse belegen',
      'IA32_FMASK MSR mit RFLAGS-Maske (Interrupts disable bei Syscall) konfigurieren',
      'Syscall Dispatcher Table (sys_read, sys_write, sys_exit, sys_fork) anlegen',
      'User-Mode Testprogramm nach Ring 3 springen lassen und sys_write testen'
    ]
  }
];

export const PROJECT_PRESETS: ProjectPreset[] = [
  {
    id: 'baremetal-x86-64',
    title: 'x86_64 Bare Metal Kernel (C / Asm)',
    categoryBadge: 'OS Development',
    icon: 'Cpu',
    description: 'Vollständige Entwicklungs-Roadmap für einen 64-Bit x86_64 Monolithic oder Microkernel von Scratch.',
    categories: [
      { name: 'Boot', color: '#f59e0b' },
      { name: 'Kernel', color: '#10b981' },
      { name: 'Memory', color: '#6366f1' },
      { name: 'Interrupts', color: '#ec4899' },
      { name: 'Drivers', color: '#06b6d4' },
      { name: 'Arch', color: '#8b5cf6' },
      { name: 'VFS', color: '#14b8a6' },
      { name: 'Userland', color: '#f97316' }
    ],
    tasks: [
      {
        text: 'Multiboot2 Header & Linker Script aufsetzen',
        category: 'Boot',
        priority: 'High',
        subtasks: [
          'Multiboot2 Magic (0xE85250D6), Architecture 0 (i386/x86_64) und Header Tags definieren',
          'Linker Script (linker.ld) mit Kernel Base Address 0x100000 und 4KB Alignment erstellen',
          'Entry-Point _start in x86 Assembly (boot.asm) schreiben',
          'Von 32-Bit Protected Mode in 64-Bit Long Mode wechseln (CR4.PAE, EFER.LME, CR0.PG)',
          'Early Stack (4096 Bytes in .bss) reservieren und rsp initialisieren'
        ],
        notes: 'Basis für das Booten via GRUB2 oder QEMU (-kernel option). Übergabe von EBX als Pointer zur Multiboot-Information Table.',
        dueDateOffsetDays: 1
      },
      {
        text: 'Early Console: VGA Text Buffer (0xB8000) & UART COM1 Serial Driver',
        category: 'Drivers',
        priority: 'High',
        subtasks: [
          'VGA Video Buffer 0xB8000 Memory Mapping für 80x25 Zeichen',
          'VGA Farbattribut-Byte & Cursor-Positionierung via I/O Ports 0x3D4 / 0x3D5',
          '16550 UART COM1 Serial Port (0x3F8) mit 115200 Baud initialisieren',
          'kprintf() Format-String Parser (%s, %d, %x, %p) für Early Debugging implementieren'
        ],
        notes: 'Erlaubt sofortige Konsolenausgabe auf dem Bildschirm sowie Logging in QEMU über -serial stdio.',
        dueDateOffsetDays: 2
      },
      {
        text: 'GDT (Global Descriptor Table) & IDT (Interrupt Descriptor Table) initialisieren',
        category: 'Interrupts',
        priority: 'High',
        subtasks: [
          '64-Bit GDT mit Kernel Code (0x08), Kernel Data (0x10), User Code (0x18), User Data (0x20) und TSS (0x28) anlegen',
          'TSS (Task State Segment) mit separatem Interrupt Stack (IST1 für Double Fault) konfigurieren',
          'IDT mit 256 Gates (Interrupt & Trap Gates) strukturieren',
          'Assembly ISR-Stubs (0-31) für CPU Exceptions (Divide-by-Zero, Double Fault, Page Fault, GPF) schreiben',
          'lidt Instruction ausführen und Ausnahmen mit Software-Interrupt int $3 testen'
        ],
        notes: 'Verhindert Triple Faults bei CPU-Ausnahmen und bildet das Fundament für Interrupt-getriebene Treiber.',
        dueDateOffsetDays: 3
      },
      {
        text: 'Physical Memory Manager (Bitmap / Buddy Frame Allocator)',
        category: 'Memory',
        priority: 'High',
        subtasks: [
          'Multiboot2 Memory Map (Tag 6) durchsuchen und freie RAM-Bereiche erfassen',
          'Bitmask-Array für alle 4KB Frames im Speicher allozieren',
          'pmm_alloc_frame() zur Rückgabe eines freien 4KB Physical Address Frames schreiben',
          'pmm_free_frame(uintptr_t addr) zum Freigeben von Frames implementieren',
          'Statistik-Tracker für freien und belegten Gesamtspeicher anbinden'
        ],
        notes: 'Verwaltet den physischen RAM blockweise und ist Voraussetzung für dynamisches Paging.',
        dueDateOffsetDays: 4
      },
      {
        text: 'Virtual Memory & 4-Level Paging (PML4, PDPT, PD, PT)',
        category: 'Memory',
        priority: 'High',
        subtasks: [
          'PML4, PDPT, PD und PT Tabellenstruktur (je 512 Einträge à 8 Bytes) definieren',
          'Identity Mapping für die ersten 4MB physischen RAMs einrichten',
          'Higher-Half Kernel Mapping bei 0xFFFF800000000000 konfigurieren',
          'vmm_map_page(uintptr_t virt, uintptr_t phys, uint32_t flags) implementieren',
          'vmm_unmap_page(uintptr_t virt) mit TLB Invalidation (invlpg) schreiben',
          'Page Fault Handler (#PF, Vector 14) mit CR2 Fehleradress-Analyse registrieren'
        ],
        notes: 'Trennt Kernel- von Userspace-Adressen und schützt Speicherbereiche mit Present/Writable/User Flags.',
        dueDateOffsetDays: 5
      },
      {
        text: 'Kernel Heap Allocator (kmalloc / kfree / Slab Cache)',
        category: 'Memory',
        priority: 'Medium',
        subtasks: [
          'Virtuellen Adressbereich für Kernel Heap (z.B. 64MB ab 0xFFFF900000000000) reservieren',
          'Free-List oder Slab-Allocator für häufige Objektgrößen (32B, 64B, 128B, 512B, 4KB) bauen',
          'kmalloc(size_t size) mit 8-Byte/16-Byte Alignment implementieren',
          'kfree(void* ptr) mit automatischer Block-Verschmelzung (Coalescing) schreiben',
          'Memory-Leak & Boundary-Corruption Detection (Guard Bytes / Magic Cookies) integrieren'
        ],
        notes: 'Ermöglicht dynamische Datenstrukturen (verkettete Listen, Queues, Strings) im Kernelcode.',
        dueDateOffsetDays: 6
      },
      {
        text: 'PIC / Local APIC & PIT Timer Interrupts (IRQ 0-15)',
        category: 'Interrupts',
        priority: 'Medium',
        subtasks: [
          'Legacy 8259 PIC remappen (Master auf 0x20..0x27, Slave auf 0x28..0x2F)',
          'Programmable Interval Timer (PIT 8254) auf 100Hz Taktfrequenz initialisieren',
          'IRQ-Handler für Timer-Tick registrieren (Uptime Counter in Millisekunden)',
          'Optional: Local APIC und IO-APIC Erkennung via ACPI MADT Tabelle'
        ],
        notes: 'Der Timer-Interrupt bildet die Taktrate für den späteren preemptiven Task-Scheduler.',
        dueDateOffsetDays: 7
      },
      {
        text: 'PS/2 Keyboard Driver & Scancode Decoder',
        category: 'Drivers',
        priority: 'Medium',
        subtasks: [
          'PS/2 Controller Status (Port 0x64) und Data (Port 0x60) abfragen',
          'IRQ 1 Interrupt Service Routine für Tastatureingaben verdrahten',
          'Scancode Set 1 & 2 Decoder mit Shift-, Ctrl- und CapsLock-Status schreiben',
          'Circular Key-Buffer (Ringbuffer FIFO) für asynchrone Eingabeverarbeitung implementieren'
        ],
        notes: 'Ermöglicht interaktive Tasten-Eingaben für eine Kernel-Shell oder Terminals.',
        dueDateOffsetDays: 8
      },
      {
        text: 'Preemptive Multitasking & Task Scheduler (PCB & Context Switch)',
        category: 'Arch',
        priority: 'High',
        subtasks: [
          'Process Control Block (PCB) mit Stackpointer, CR3 Page Table, PID und State anlegen',
          'Initialen Task-Stack mit gefakten Register-Pushes für neuen Thread erzeugen',
          'Assembly switch_task(old_pcb, new_pcb) für Context Switch (Push/Pop Register & RSP Tausch) schreiben',
          'Round-Robin Scheduler im Timer-IRQ 0 aufrufen',
          'Task-States (READY, RUNNING, SLEEPING, TERMINATED) und sys_yield() implementieren'
        ],
        notes: 'Ermöglicht echten parallelen Mehrbenutzer- / Multithread-Betrieb auf Kernel-Ebene.',
        dueDateOffsetDays: 9
      },
      {
        text: 'Ring 3 User Mode & Syscall Interface (syscall / sysret)',
        category: 'Userland',
        priority: 'High',
        subtasks: [
          'IA32_EFER MSR aktivieren und IA32_LSTAR MSR mit syscall_entry Pointer laden',
          'IA32_STAR MSR mit Kernel & User CS/SS Segment-Selektoren konfigurieren',
          'Syscall Dispatcher (sys_read, sys_write, sys_exit, sys_fork, sys_yield) verdrahten',
          'TSS rsp0 bei jedem Interrupt/Syscall auf aktuellen Kernel-Stack des Threads setzen',
          'Ersten Ring 3 User-Prozess starten und Test-Syscall ausführen'
        ],
        notes: 'Trennt Kernel-Berechtigungen (Ring 0) sicher vom Userspace (Ring 3).',
        dueDateOffsetDays: 10
      },
      {
        text: 'Virtual File System (VFS) & Initramfs / TAR Parser',
        category: 'VFS',
        priority: 'Medium',
        subtasks: [
          'VFS Node Abstraktion (read, write, open, close, readdir, finddir) definieren',
          'Initramfs (Ustar / TAR Archiv oder CPIO) vom Bootloader als Multiboot-Modul laden',
          'Root-Filesystem / aus dem Initrd-Archiv im VFS mounten',
          'Standard file descriptors (0: stdin, 1: stdout, 2: stderr) an Console binden'
        ],
        notes: 'Macht Dateien, Treiber und Geräte als einheitliche UNIX-ähnliche Pfade (/dev, /bin) zugänglich.',
        dueDateOffsetDays: 11
      },
      {
        text: 'ELF64 Executable Binary Loader',
        category: 'Userland',
        priority: 'Medium',
        subtasks: [
          'ELF64 Header Validierung (Magic \x7FELF, Class 2 für 64-Bit, Executable Type)',
          'Program Header durchiterieren (PT_LOAD Segmente auslesen)',
          'Virtuelle User-Pages allozieren und Segment-Daten (.text, .rodata, .data) kopieren',
          '.bss Sektion mit Nullen initialisieren',
          'Instruction Pointer (RIP) auf e_entry setzen und Userland-Binary ausführen'
        ],
        notes: 'Erlaubt das Ausführen echter kompilierten C/Rust/Go Userland-Programme aus dem Dateisystem.',
        dueDateOffsetDays: 12
      }
    ]
  },
  {
    id: 'baremetal-rust-os',
    title: 'Rust #![no_std] Operating System',
    categoryBadge: 'Rust Bare Metal',
    icon: 'Shield',
    description: 'Moderner, speichersicherer Microkernel / Kernel in Rust unter Verwendung von #![no_std] und x86_64 Crate.',
    categories: [
      { name: 'Core', color: '#f59e0b' },
      { name: 'Memory', color: '#6366f1' },
      { name: 'Interrupts', color: '#ec4899' },
      { name: 'Async', color: '#10b981' },
      { name: 'Drivers', color: '#06b6d4' }
    ],
    tasks: [
      {
        text: 'Freestanding Rust Binary & Panic Handler aufsetzen',
        category: 'Core',
        priority: 'High',
        subtasks: [
          'Cargo.toml mit #![no_std] und custom target JSON (x86_64-unknown-none) einrichten',
          '#[panic_handler] Funktion mit Serial/Console Output implementieren',
          'Linker-Skript und #[no_main] pub extern "C" fn _start() Entrypoint definieren',
          'Bootloader Crate (bootloader = "0.9") in Cargo.toml einbinden'
        ],
        notes: 'Eliminiert C-Runtime und libstd Abhängigkeiten zugunsten von core und compiler_builtins.',
        dueDateOffsetDays: 1
      },
      {
        text: 'VGA Text Buffer & Type-Safe println! Macro',
        category: 'Drivers',
        priority: 'High',
        subtasks: [
          'Writer-Struktur mit volatile::Volatile Wrapper für Speicherzugriffe auf 0xb8000 bauen',
          'ColorCode und ScreenChar Repräsentationen mit #[repr(transparent)] definieren',
          'spin::Mutex für thread-sicheren statischen WRITER anlegen',
          'Standard print! und println! Makros via core::fmt::Write implementieren'
        ],
        notes: 'Verhindert Deadlocks und optimierte Writes durch Type-Safety und Volatile Wrappers.',
        dueDateOffsetDays: 2
      },
      {
        text: 'IDT & Double Fault Stack Switching (IST)',
        category: 'Interrupts',
        priority: 'High',
        subtasks: [
          'x86_64::structures::idt::InterruptDescriptorTable instanziieren',
          'Breakpoint und Page-Fault Handler registrieren',
          'GDT mit Interrupt Stack Table (IST) für Double Faults konfigurieren',
          'Stack-Overflow Test via Endlos-Rekursion im Kernel provozieren und sauberen IST-Dump validieren'
        ],
        notes: 'Ein separater IST-Stack garantiert, dass bei einem Kernel-Stack-Overflow ein fassbarer Double Fault geworfen wird.',
        dueDateOffsetDays: 3
      },
      {
        text: 'Paging & Memory Management mit BootInfo Frame Allocator',
        category: 'Memory',
        priority: 'High',
        subtasks: [
          'BootInfo MemoryMap parsen und usable Regionen an FrameAllocator übergeben',
          'OffsetPageTable Mapper aus active Level 4 Page Table initialisieren',
          'UnusedFrameAllocator mit Page-Iteratoren und safety-invarianten Traits ausstatten',
          'Virtuelle Adresse zu physischer Adresse übersetzen und probe-mappen'
        ],
        notes: 'Kombiniert Rust Ownership mit x86_64 Paging-Strukturen für fehlerfreie Speicherverwaltung.',
        dueDateOffsetDays: 4
      },
      {
        text: 'Heap Allokator & alloc Crate Integration',
        category: 'Memory',
        priority: 'High',
        subtasks: [
          'GlobalAlloc Trait für Kernel-Heap implementieren (z.B. linked_list_allocator oder bump allocator)',
          '#[global_allocator] definieren und Heap-Speicherbereich (100KB bis 10MB) mappen',
          'extern crate alloc; einbinden und Box<T>, Vec<T>, String und Arc<T> im Kernel verwenden',
          'Stress-Test mit dynamischen Vektoren und Deallokationen durchführen'
        ],
        notes: 'Bringt den vollen Komfort von dynamischen Standard-Datenstrukturen in den bare-metal Kernel.',
        dueDateOffsetDays: 5
      },
      {
        text: 'Asynchrones Multitasking & Executor mit Wakers',
        category: 'Async',
        priority: 'Medium',
        subtasks: [
          'Task-Struktur mit core::future::Future und TaskId definieren',
          'SimpleExecutor mit Task-Queue (crossbeam::ArrayQueue oder alloc::collections::VecDeque) bauen',
          'Waker-Implementierung mit alloc::sync::Arc und core::task::Wake Trait verknüpfen',
          'ScancodeStream für asynchrone Tastatureingaben mit futures_util::stream::Stream bereitstellen'
        ],
        notes: 'Ermöglicht extrem leichtgewichtiges kooperatives Multitasking ohne Thread-Stack-Overhead.',
        dueDateOffsetDays: 6
      }
    ]
  },
  {
    id: 'baremetal-embedded-arm',
    title: 'ARM Cortex-M / RISC-V Embedded Bare Metal',
    categoryBadge: 'Embedded Systems',
    icon: 'Cpu',
    description: 'Low-Level Firmware & Microkernel für Mikrocontroller (STM32, RP2040, ESP32, RISC-V).',
    categories: [
      { name: 'Hardware', color: '#f59e0b' },
      { name: 'Registers', color: '#6366f1' },
      { name: 'Interrupts', color: '#ec4899' },
      { name: 'Drivers', color: '#06b6d4' },
      { name: 'RTOS', color: '#10b981' }
    ],
    tasks: [
      {
        text: 'Vector Table & Startup Code (.s / .c)',
        category: 'Hardware',
        priority: 'High',
        subtasks: [
          'Vector Table Array mit Initial Stack Pointer und Reset_Handler anlegen',
          'HardFault_Handler, NMI_Handler, SysTick_Handler Stubs schreiben',
          'Linker Script mit FLASH (z.B. 0x08000000, 512K) und SRAM (0x20000000, 128K) konfigurieren',
          'C-Runtime Startup: .data Sektion von Flash ins RAM kopieren & .bss Sektion mit Nullen füllen',
          'Sprung zu main() ausführen'
        ],
        notes: 'Grundlage für jeden ARM Cortex-M Mikrocontroller ohne Abhängigkeit von Hersteller-HALs.',
        dueDateOffsetDays: 1
      },
      {
        text: 'MMIO Peripheral Register Abstraction & Clock Config',
        category: 'Registers',
        priority: 'High',
        subtasks: [
          'RCC (Reset and Clock Control) Base Address & Register Struct definieren',
          'PLL und System Clock auf Maximaltakt (z.B. 72MHz / 168MHz) hochschalten',
          'GPIO Clock Gating für Ports (GPIOA, GPIOB, GPIOC) aktivieren',
          'Volatile Register Pointer Macros für bitweise Manipulation implementieren'
        ],
        notes: 'Stellt sicher, dass Peripheriemodule mit stabiler Taktfrequenz versorgt werden.',
        dueDateOffsetDays: 2
      },
      {
        text: 'UART Driver mit Ringbuffer & Interrupts',
        category: 'Drivers',
        priority: 'Medium',
        subtasks: [
          'USART Baud Rate Register (BRR) für 115200 Baud berechnen und setzen',
          'TX und RX Enable Bits im Control Register (CR1) aktivieren',
          'NVIC (Nested Vectored Interrupt Controller) für USART IRQ freischalten',
          'Ringbuffer für eingehende Bytes im RX-Interrupt befüllen',
          'printf Redirect via _write() Syscall implementieren'
        ],
        notes: 'Verbindet das Board mit einer seriellen Konsole für Debugging und CLI Kommandos.',
        dueDateOffsetDays: 3
      },
      {
        text: 'SysTick Timer & Kooperativer RTOS Coroutine Scheduler',
        category: 'RTOS',
        priority: 'Medium',
        subtasks: [
          'SysTick Reload Value Register für 1ms System-Tick einstellen',
          'SysTick_Handler für millis() Uptime Tracker nutzen',
          'Task-Struktur mit Zuständen und Funktionszeigern anlegen',
          'PendSV Exception für sauberen Context-Switch via Software-Trigger nutzen'
        ],
        notes: 'Minimalistisches RTOS Fundament für nebenläufige Embedded Applikationen.',
        dueDateOffsetDays: 4
      }
    ]
  },
  {
    id: 'baremetal-drivers-pci',
    title: 'Hardware Driver & PCI Bring-Up Sprint',
    categoryBadge: 'Hardware Drivers',
    icon: 'Terminal',
    description: 'Treiberentwicklung für PCI-Bus, Festplatten-Controller (AHCI/NVMe) und Netzwerkkarten.',
    categories: [
      { name: 'PCI', color: '#f59e0b' },
      { name: 'Storage', color: '#10b981' },
      { name: 'Network', color: '#06b6d4' },
      { name: 'ACPI', color: '#ec4899' }
    ],
    tasks: [
      {
        text: 'PCI Bus Scanning & Device Enumeration',
        category: 'PCI',
        priority: 'High',
        subtasks: [
          'PCI Configuration Ports 0xCF8 (CONFIG_ADDRESS) und 0xCFC (CONFIG_DATA) ansprechen',
          'Alle Busse (0..255), Devices (0..31) und Functions (0..7) scannen',
          'Vendor ID, Device ID, Class Code und Subclass aus dem PCI Header auslesen',
          'Base Address Registers (BAR0..BAR5) für Memory-Mapped I/O oder I/O Ports parsen'
        ],
        notes: 'Erkennt alle gesteckten Hardware-Geräte (Grafikkarten, Netzwerkkarten, Festplatten-Controller).',
        dueDateOffsetDays: 2
      },
      {
        text: 'AHCI SATA Controller Driver (Block Device I/O)',
        category: 'Storage',
        priority: 'High',
        subtasks: [
          'AHCI Controller via PCI Class 0x01 (Mass Storage), Subclass 0x06 (SATA) lokalisieren',
          'Generic Host Control (GHC) Register und AHCI Ports (ABAR / BAR5) mappen',
          'Command List und Received FIS Strukturen 1KB-aligned allozieren',
          'Physical Region Descriptor Table (PRDT) für DMA-Transfers einrichten',
          'ahci_read_sector(uint64_t lba, void* buffer) und ahci_write_sector() implementieren'
        ],
        notes: 'Erlaubt schnellen Direct Memory Access (DMA) Zugriff auf Festplatten und SSDs.',
        dueDateOffsetDays: 4
      },
      {
        text: 'Intel e1000 / Realtek RTL8139 Ethernet NIC Driver',
        category: 'Network',
        priority: 'Medium',
        subtasks: [
          'PCI Device ID (z.B. 0x8086:0x100E für Intel e1000 in QEMU) identifizieren',
          'EEPROM auslesen und 48-Bit MAC-Adresse erfassen',
          'Transmit & Receive Circular Descriptor Ring Buffers im physischen RAM allozieren',
          'Interrupts für RX Packet Available freischalten und Ethernet Frame Header verarbeiten',
          'net_send_packet(void* data, size_t len) Funktion schreiben'
        ],
        notes: 'Voraussetzung für TCP/IP Stack, ARP Anfragen und Ping-Antworten im Kernel.',
        dueDateOffsetDays: 6
      },
      {
        text: 'ACPI RSDP / MADT Parser & Multi-Core SMP Boot',
        category: 'ACPI',
        priority: 'Medium',
        subtasks: [
          'Root System Description Pointer (RSDP) im EBDA / BIOS Speicherbereich suchen',
          'Multiple APIC Description Table (MADT) auf Local APIC und IOAPIC Einträge parsen',
          'Anzahl physischer CPU-Kerne erfassen',
          'Application Processors (APs) via INIT-SIPI-SIPI IPI Sequenz aus dem Real Mode aufwecken'
        ],
        notes: 'Schaltet moderne Mehrkern-Prozessoren (SMP) für parallele Kernel-Threads frei.',
        dueDateOffsetDays: 8
      }
    ]
  }
];
