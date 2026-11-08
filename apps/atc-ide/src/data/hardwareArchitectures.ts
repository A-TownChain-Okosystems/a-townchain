export type HardwareArchId = "x86_64" | "arm64" | "cortex_m" | "riscv" | "embedded_esp32";
export type ParadigmId = "monolith" | "hierarchy" | "layered" | "modular";

export interface ArchFile {
  name: string;
  content: string;
  description: string;
  type?: "kernel" | "hal" | "driver" | "ipc" | "syscall" | "user" | "config";
}

export interface HardwareArchInfo {
  id: HardwareArchId;
  name: string;
  shortName: string;
  badgeColor: string;
  vendor: string;
  bitness: "32-Bit" | "64-Bit";
  privilegeLevels: string[];
  registers: string[];
  pagingModel: string;
  interruptController: string;
  description: string;
  isRecommended?: boolean;
  recommendedBadge?: string;
  recommendedReason?: string;
}

export interface ParadigmInfo {
  id: ParadigmId;
  name: string;
  germanName: string;
  tagline: string;
  badgeColor: string;
  pros: string[];
  cons: string[];
  useCases: string[];
  diagram: string;
  description: string;
  isRecommended?: boolean;
  recommendedBadge?: string;
  recommendedReason?: string;
}

export interface RecommendedSettingItem {
  category: string;
  flag: string;
  value: string;
  description: string;
  isEssential?: boolean;
}

export interface ArchitectureTemplate {
  id: string;
  title: string;
  archId: HardwareArchId;
  paradigmId: ParadigmId;
  version: string;
  summary: string;
  memoryLayout: string;
  keyFeatures: string[];
  files: ArchFile[];
  isRecommended?: boolean;
  recommendedBadge?: string;
  recommendedReason?: string;
  recommendedSettings?: RecommendedSettingItem[];
}

export interface CodeModuleItem {
  id: string;
  title: string;
  archId?: HardwareArchId;
  paradigmId?: ParadigmId;
  category: string;
  description: string;
  tags: string[];
  code: string;
  isRecommended?: boolean;
  recommendedBadge?: string;
  recommendedReason?: string;
}

export const RECOMMENDED_KERNEL_CONFIG = {
  title: "Empfohlene Bare-Metal OS-Kernel Konfiguration (From Scratch)",
  subtitle: "Best Practice für unabhängige Betriebssysteme ohne Host-OS & ohne libc-Abhängigkeiten",
  architecture: "x86_64",
  paradigm: "layered",
  bootProtocol: "Multiboot2 / Limine (Direkter 64-Bit Einstieg & Framebuffer)",
  compilerFlags: [
    { flag: "-ffreestanding", description: "Verhindert Annahmen über vorhandene Standard-C-Bibliotheken", value: "Aktiviert", isEssential: true },
    { flag: "-nostdlib", description: "Keine automatische Verlinkung gegen libc oder System-Startup-Code (crt0)", value: "Aktiviert", isEssential: true },
    { flag: "-mno-red-zone", description: "Verhindert Red-Zone-Nutzung (128 Bytes unter RSP), da Hardware-Interrupts diesen Bereich überschreiben würden", value: "Aktiviert", isEssential: true },
    { flag: "-fno-pie / -mcmodel=kernel", description: "Position-Independent Executable deaktiviert, Kernel liegt im Higher-Half Speicher (0xFFFFFFFF80000000)", value: "Aktiviert", isEssential: true },
    { flag: "-fno-stack-protector", description: "Deaktiviert __stack_chk_fail Prüfungen, bis der Kernel eine eigene Stack-Smash-Routine bereitstellt", value: "Empfohlen für Phase 1", isEssential: false },
    { flag: "-mgeneral-regs-only", description: "Verhindert SSE/AVX-Register in Interrupt-Handlern, solange CR0.EM/TS nicht konfiguriert sind", value: "Aktiviert", isEssential: true },
  ],
  qemuRunCommand: "qemu-system-x86_64 -kernel kernel.bin -m 512M -serial stdio -display none",
  linkerScriptExcerpt: `ENTRY(_start)
SECTIONS {
  . = 0xFFFFFFFF80100000; /* Higher-Half Kernel Base */
  .text ALIGN(4K) : { *(.multiboot_header) *(.text*) }
  .rodata ALIGN(4K) : { *(.rodata*) }
  .data ALIGN(4K) : { *(.data*) }
  .bss ALIGN(4K) : { *(COMMON) *(.bss*) }
}`,
  essentialComponents: [
    { name: "Paging (4-Level PML4)", desc: "48-Bit Adressraum: Trennung von Ring 0 (0xFFFF...) und Ring 3 Userland (0x0000...)", status: "Erforderlich" },
    { name: "GDT & IDT Tabellen", desc: "Segmentierung für Code/Data (Ring 0 & 3) sowie 256 Interrupt-Gates mit TSS/IST Notfall-Stack", status: "Erforderlich" },
    { name: "APIC Timer & Preemptive Scheduler", desc: "Hardware-Tick Interrupts für unterbrechendes Multitasking und Task-Kontextwechsel", status: "Erforderlich" },
    { name: "MSR_LSTAR Syscall Gateway", desc: "High-Speed Hardware-Systemaufrufe direkt von Ring 3 in den Ring-0-Kernel", status: "Empfohlen" },
    { name: "VFS (Virtual File System)", desc: "Abstrahierte Dateisystem-Schicht mit Inode-Verwaltung für Geräte und Partitionen", status: "Empfohlen" }
  ]
};

export const HARDWARE_ARCHITECTURES: Record<HardwareArchId, HardwareArchInfo> = {
  x86_64: {
    id: "x86_64",
    name: "x86_64 / AMD64 (Intel & AMD)",
    shortName: "x86_64",
    badgeColor: "from-blue-500/20 to-cyan-500/20 text-cyan-300 border-cyan-500/40",
    vendor: "Intel / AMD",
    bitness: "64-Bit",
    privilegeLevels: ["Ring 0 (Kernel)", "Ring 1 (Services/Drivers)", "Ring 2 (Reserved)", "Ring 3 (Userland)"],
    registers: ["RAX", "RBX", "RCX", "RDX", "RSI", "RDI", "RSP", "RBP", "R8-R15", "CR0", "CR3", "CR4", "MSR_LSTAR", "RFLAGS"],
    pagingModel: "4-Level Paging (PML4 -> PDPT -> PD -> PT, 48-bit VA) / 5-Level (PML5)",
    interruptController: "Local APIC + I/O APIC / x2APIC",
    description: "Der Industriestandard für Server, Workstations und PCs. Bietet hardwarebeschleunigte Ring-Hierarchie (Ring 0–3), Task State Segment (TSS), Interrupt Stack Tables (IST), Hardware-Paging und MSR-basierte Syscall/Sysret-Instruktionen.",
    isRecommended: true,
    recommendedBadge: "★ TOP-EMPFEHLUNG FÜR BARE-METAL OS",
    recommendedReason: "Reichhaltigste Dokumentation (OSDev, Multiboot2, QEMU), 64-Bit Long Mode, 4-stufiges Paging und sofortige Lauffähigkeit auf allen x86-PCs."
  },
  arm64: {
    id: "arm64",
    name: "ARM AArch64 (ARMv8-A / ARMv9-A)",
    shortName: "AArch64",
    badgeColor: "from-emerald-500/20 to-teal-500/20 text-emerald-300 border-emerald-500/40",
    vendor: "ARM / Apple Silicon / Graviton / Snapdragon",
    bitness: "64-Bit",
    privilegeLevels: ["EL0 (User Application)", "EL1 (Operating System Kernel)", "EL2 (Hypervisor)", "EL3 (Secure Monitor / TrustZone)"],
    registers: ["X0-X30", "SP_EL0", "SP_EL1", "VBAR_EL1", "TTBR0_EL1", "TTBR1_EL1", "SCTLR_EL1", "ESR_EL1", "FAR_EL1", "PSTATE"],
    pagingModel: "Split Virtual Memory (TTBR0 für Userland, TTBR1 für Kernel, 4KB/64KB Granule)",
    interruptController: "ARM GICv2 / GICv3 / GICv4 (Distributor & Redistributor)",
    description: "Hocheffiziente 64-Bit RISC-Architektur für moderne Server, Cloud-Infrastruktur und mobile Plattformen. Nutzt Exception Levels (EL0–EL3), TrustZone-Sicherheitsenklaven und getrennte Address-Translation-Register.",
    isRecommended: true,
    recommendedBadge: "★ EMPFOHLEN FÜR MODERNE SERVER & CLOUD",
    recommendedReason: "Klare Exception-Levels (EL0–EL3), getrennte TTBR0/TTBR1 MMU-Register und zukunftssichere Energieeffizienz."
  },
  cortex_m: {
    id: "cortex_m",
    name: "ARM Cortex-M (Cortex-M4 / M7 / M33 Embedded)",
    shortName: "Cortex-M",
    badgeColor: "from-amber-500/20 to-orange-500/20 text-amber-300 border-amber-500/40",
    vendor: "STMicroelectronics / NXP / Microchip",
    bitness: "32-Bit",
    privilegeLevels: ["Thread Mode (Privileged / Unprivileged)", "Handler Mode (Exceptions / ISR)"],
    registers: ["R0-R12", "SP (MSP/PSP)", "LR", "PC", "xPSR", "CONTROL", "PRIMASK", "BASEPRI"],
    pagingModel: "Hardware MPU (Memory Protection Unit) mit 8-16 geschützten Adressregionen",
    interruptController: "NVIC (Nested Vectored Interrupt Controller) mit Low-Latency Preemption",
    description: "Deterministische Microcontroller-Architektur für Echtzeitsysteme (RTOS), Sensorik, Robotik und IoT mit Hardware-Vektortabelle, MPU-Speicherschutz und PendSV Context-Switching."
  },
  riscv: {
    id: "riscv",
    name: "RISC-V (RV64GC / RV32IMAC)",
    shortName: "RISC-V",
    badgeColor: "from-purple-500/20 to-violet-500/20 text-purple-300 border-purple-500/40",
    vendor: "RISC-V International (Open Standard)",
    bitness: "64-Bit",
    privilegeLevels: ["U-Mode (User)", "S-Mode (Supervisor OS)", "M-Mode (Machine Firmware / SBI)", "Debug Mode"],
    registers: ["x0 (zero)", "x1 (ra)", "x2 (sp)", "x3-x31", "satp", "sstatus", "stvec", "sepc", "scause", "mie", "mip"],
    pagingModel: "Sv39 (39-Bit VA, 3-Level Page Table) / Sv48 (48-Bit VA)",
    interruptController: "PLIC (Platform-Level Interrupt Controller) + CLINT (Core Local Interruptor)",
    description: "Moderne, lizenzfreie Open-Source RISC-Architektur. Bietet strukturierte Control & Status Register (CSR), modulares Erweiterungsdesign (IMAFDC) und standardisierte Supervisor Binary Interface (SBI) Layer."
  },
  embedded_esp32: {
    id: "embedded_esp32",
    name: "Embedded SoC & Microkernel (ESP32 / WASM)",
    shortName: "Embedded/SoC",
    badgeColor: "from-pink-500/20 to-rose-500/20 text-pink-300 border-pink-500/40",
    vendor: "Espressif / WebAssembly Micro Runtime",
    bitness: "32-Bit",
    privilegeLevels: ["Core 0 (Protocol / Kernel)", "Core 1 (Application / FreeRTOS)", "WASM Sandbox Jail"],
    registers: ["A0-A15", "PC", "SAR", "PS", "WINDOWBASE", "WINDOWSTART"],
    pagingModel: "Dual-Bank SRAM + External SPI Flash MMU Cache Window",
    interruptController: "Dual-Core Peripheral Interrupt Matrix (26 Vektoren pro Kern)",
    description: "Kompakte Edge- und IoT-Architektur mit integrierter Hardware-Peripherie (WiFi/BLE, I2C, SPI, DMA), Dual-Core-Taskverteilung und isolierter WebAssembly-Sandbox für sichere Plug-in-Ausführung."
  }
};

export const PARADIGMS: Record<ParadigmId, ParadigmInfo> = {
  monolith: {
    id: "monolith",
    name: "Monolithischer Kernel (Monolith)",
    germanName: "Monolithische Architektur",
    tagline: "Maximale Performance: Alle Kernfunktionen laufen direkt im Ring 0 Adressraum",
    badgeColor: "from-blue-600/30 to-indigo-600/30 text-blue-300 border-blue-500/40",
    isRecommended: true,
    recommendedBadge: "★ EMPFOHLEN: HIGH-PERFORMANCE",
    recommendedReason: "Direkte Ring-0 Hardwarekontrolle ohne IPC-Kontextwechsel – Standardansatz für maximale Durchsatzraten und hardwarenahe Kernel-Programmierung (wie Linux-Kernel).",
    pros: [
      "Höchste I/O- und Syscall-Geschwindigkeit durch direkten Speicherzugriff",
      "Kein IPC-Overhead oder Context-Switch-Penalty zwischen Treibern und Scheduler",
      "Einfache Synchronisation über Kernel-Spinlocks, Mutexes und atomare Ringbuffer",
      "Vollständige Kontrolle über Hardware-Peripherie und Speicher-Mapping"
    ],
    cons: [
      "Fehler in einem Gerätetreiber können eine Kernel-Panic auslösen",
      "Größere Codebasis im privilegierten Ring 0 / Supervisor-Modus",
      "Schwierigere Modularisierung ohne dynamische LKM-Infrastruktur"
    ],
    useCases: [
      "Hochleistungs-Betriebssysteme (Linux, BSD, ATOS Core)",
      "Gaming- und Low-Latency-Compute-Engines",
      "Hardwarenahe Server- und Bare-Metal-Infrastruktur"
    ],
    diagram: `+-------------------------------------------------------------+
|                      USERLAND (Ring 3)                      |
|       Application A      Application B      GUI Compositor  |
+------------------------------+------------------------------+
                               |  SYSCALL / SYSENTER (Trap)
+------------------------------v------------------------------+
|                   MONOLITHISCHER KERNEL (Ring 0)            |
|  +-------------------------------------------------------+  |
|  | Syscall Dispatcher | Task Scheduler | VMM (Paging)    |  |
|  +-------------------------------------------------------+  |
|  | VFS (Virtual File System) | Network Stack (TCP/IP)    |  |
|  +-------------------------------------------------------+  |
|  | Device Drivers: NVMe, GPU, Ethernet, USB, Serial HAL  |  |
|  +-------------------------------------------------------+  |
+------------------------------+------------------------------+
                               |  Hardware Access (MMIO/PortIO)
+------------------------------v------------------------------+
|                     HARDWARE (CPU, RAM, BUS)                 |
+-------------------------------------------------------------+`,
    description: "In einem monolithischen Kernel laufen Prozess-Scheduler, virtueller Speichermanager, Dateisysteme und Gerätetreiber gemeinsam im privilegierten Modus. Dies eliminiert Overhead und bietet maximale Durchsatzraten."
  },
  hierarchy: {
    id: "hierarchy",
    name: "Hierarchische Schutzringe (Protection Rings)",
    germanName: "Hierarchische Ring-Architektur",
    tagline: "Multi-Stufen Schutz: Ring 0 (Core), Ring 1/2 (Treiber), Ring 3 (Userland)",
    badgeColor: "from-amber-600/30 to-yellow-600/30 text-amber-300 border-amber-500/40",
    pros: [
      "Fehlerisolation: Ein fehlerhafter Treiber in Ring 1 stürzt nicht den Ring-0-Core ab",
      "Klare Privilege-Eskalation über Trap-Gates, Call-Gates und TSS-Stacks",
      "Fein abgestufte Hardware-Zugriffsrechte (I/O Permission Bitmap / IOPB)",
      "Separate Interrupt-Stack-Tabellen (IST) verhindern Stack-Overflow-Kaskaden"
    ],
    cons: [
      "Hardware-Unterstützung variiert (x86_64 unterstützt 4 Ringe, ARM/RISC-V nutzen 3-4 Privilege Levels)",
      "Zusätzlicher Stack-Switch Overhead bei Ringübergängen (RSP0 / RSP1 / IST)"
    ],
    useCases: [
      "Hochsichere Enterprise-Betriebssysteme & Sicherheits-Kernel",
      "Kritische Infrastruktur mit zertifizierter Treiber-Isolation",
      "Militärische & Raumfahrt-Betriebssysteme"
    ],
    diagram: `+-------------------------------------------------------------+
|                   RING 3: USER APPLICATION                  |
|    Web Browser | Compiler | Terminal | Unprivileged Daemons |
+------------------------------+------------------------------+
                               |  Call-Gate / Software Trap
+------------------------------v------------------------------+
|             RING 1 / 2: SYSTEM SERVICES & DRIVERS           |
|    Block Device Drivers | Network Protocols | File Systems  |
|         (IOPB eingeschränkt, kein Paging-Root-Zugriff)      |
+------------------------------+------------------------------+
                               |  Hardware Gate / Sysret
+------------------------------v------------------------------+
|                     RING 0: KERNEL NUCLEUS                  |
|    Page Table Root (CR3) | IDT / GDT | Interrupt Stack (IST)|
|    Hardware Context Switching | CPU Security Enclaves       |
+-------------------------------------------------------------+`,
    description: "Hierarchische Architekturen teilen das System in konzentrische Schutzringe ein. Der innerste Kern (Ring 0) besitzt uneingeschränkte Hardwarekontrolle, während Dienste und Treiber in Zwischenringen (Ring 1/2) isoliert ausgeführt werden."
  },
  layered: {
    id: "layered",
    name: "Schichtenarchitektur (Layered Architecture)",
    germanName: "Schichten- / Layer-Modell",
    tagline: "Strikte Schichtung: Jede Schicht nutzt ausschließlich Dienste der darunterliegenden Schicht",
    badgeColor: "from-emerald-600/30 to-teal-600/30 text-emerald-300 border-emerald-500/40",
    isRecommended: true,
    recommendedBadge: "★ TOP EMPFEHLUNG: HYBRID-KERNEL",
    recommendedReason: "Kapselt Hardware-Abstraktionen (HAL) sauber von Kerneldiensten, VFS und Syscalls ab – das bewährte Vorbild für Windows NT, macOS XNU und modulare Betriebssysteme.",
    pros: [
      "Perfekte Portabilität: Tausch des HAL (Hardware Abstraction Layer) genügt für neue CPU-Architekturen",
      "Modulare Testbarkeit: Jede Schicht kann isoliert unit-getestet und gemockt werden",
      "Saubere Abstraktionsgrenzen und klar definierte Schnittstellen (Interfaces)",
      "Vermeidung von zirkulären Abhängigkeiten"
    ],
    cons: [
      "Funktionsaufrufe müssen potenziell mehrere Schichten durchqueren",
      "Strenge Disziplin bei Architektur-Entwicklung erforderlich"
    ],
    useCases: [
      "Cross-Platform Betriebssysteme (Windows NT, ATOS Multi-Arch)",
      "Eingebettete Steuerungs- und Automatisierungssysteme",
      "Enterprise SDKs und Frameworks"
    ],
    diagram: `+-------------------------------------------------------------+
|   LAYER 4: Subsysteme & Userland Applications (Shell, GUI)  |
+------------------------------+------------------------------+
                               |  API Call
+------------------------------v------------------------------+
|   LAYER 3: System Call Interface & Translation Gateway      |
+------------------------------+------------------------------+
                               |  Executive Call
+------------------------------v------------------------------+
|   LAYER 2: Executive Services (VFS, IPC, Security Monitor)  |
+------------------------------+------------------------------+
                               |  Kernel Call
+------------------------------v------------------------------+
|   LAYER 1: Core Kernel (Task Scheduler, Virtual Memory Mgt) |
+------------------------------+------------------------------+
                               |  HAL Primitive
+------------------------------v------------------------------+
|   LAYER 0: Hardware Abstraction Layer (HAL / BSP Registers) |
+-------------------------------------------------------------+`,
    description: "Das Schichtenmodell organisiert Software in streng hierarchischen Ebenen von der Hardware-Abstraktion (HAL) bis zu den Benutzeroberflächen. Jede Schicht kapselt ihre Komplexität vollständig."
  },
  modular: {
    id: "modular",
    name: "Modulare Architektur & Microkernel (Modular / IPC)",
    germanName: "Modulare & Microkernel-Architektur",
    tagline: "Minimaler Kern + Isolierte User-Space Server mit IPC Message Passing & LKM",
    badgeColor: "from-purple-600/30 to-fuchsia-600/30 text-purple-300 border-purple-500/40",
    pros: [
      "Höchste Systemstabilität: Absturz eines Dateisystem- oder GPU-Servers betrifft nicht den Kernel",
      "Dynamische Erweiterbarkeit: Kernel-Module (.lkm / .mod) zur Laufzeit lad- und entladbar",
      "Capability-basierte Sicherheit: Server erhalten nur exakt definierte Berechtigungstokens",
      "Live-Patching und nahtlose Treiber-Updates ohne Neustart"
    ],
    cons: [
      "IPC-Nachrichtenaustausch (Message Passing) benötigt Kontextwechsel",
      "Erfordert performante Shared-Memory Zero-Copy Ringbuffer für High-Speed I/O"
    ],
    useCases: [
      "Moderne Microkernel (QNX, seL4, Fuchsia / Zircon, Minix 3)",
      "Echtzeitkritische Automobil- und Luftfahrtsysteme (AUTOSAR Adaptive)",
      "Hochverfügbare Server-Infrastrukturen"
    ],
    diagram: `+-------------------------------------------------------------+
|                     USER-SPACE SERVERS & APPS               |
|  +-----------+  +------------+  +------------+  +--------+  |
|  | VFS-Server|  | Net-Server |  | GPU-Server |  | App A  |  |
|  +-----+-----+  +-----+------+  +-----+------+  +---+----+  |
+--------|--------------|---------------|-------------|-------+
         |              | IPC-Nachricht |             |
+--------v--------------v---------------v-------------v-------+
|                    MICROKERNEL CORE (Ring 0)                |
|  +-------------------------------------------------------+  |
|  | Fast IPC Message Passing | Capability Security Tokens |  |
|  +-------------------------------------------------------+  |
|  | Thread Scheduler (Priority/Deadline) | Address Spaces |  |
|  +-------------------------------------------------------+  |
|  | Dynamic Module Loader (LKM Symbol Resolver)           |  |
|  +-------------------------------------------------------+  |
+-------------------------------------------------------------+`,
    description: "Modulare und Microkernel-Systeme reduzieren den Kernel auf fundamentale Mechanismen (IPC, Scheduling, Speicheradressräume). Alle Dienste (Dateisystem, Netzwerk, Treiber) laufen als isolierte Module im User-Space."
  }
};

export const ARCHITECTURE_TEMPLATES: ArchitectureTemplate[] = [
  // 1. x86_64 Monolith
  {
    id: "x86-monolith-core",
    title: "x86_64 High-Performance Monolith Kernel",
    archId: "x86_64",
    paradigmId: "monolith",
    version: "v2.4.0",
    isRecommended: true,
    recommendedBadge: "★ TOP-EMPFEHLUNG: OS-KERNEL FROM SCRATCH",
    recommendedReason: "Die bewährte Referenz-Architektur für eigene Betriebssysteme: 64-Bit Long Mode, 4-Level Paging und direkte Ring-0 Hardwarekontrolle ohne externe Abhängigkeiten (wie Linux-Kernel).",
    recommendedSettings: [
      { category: "Compiler", flag: "-ffreestanding", value: "Aktiviert", description: "Verhindert Bibliotheksannahmen der Host-Toolchain", isEssential: true },
      { category: "Compiler", flag: "-nostdlib", value: "Aktiviert", description: "Keine Bindung an Fremd-libc oder crt0", isEssential: true },
      { category: "Compiler", flag: "-mno-red-zone", value: "Aktiviert", description: "Schützt Interrupt-Handler vor Stack-Korruption (128 Bytes unter RSP)", isEssential: true },
      { category: "Paging", flag: "4-Level PML4", value: "48-Bit Virtual Memory", description: "Higher-Half Kernel Mapping bei 0xFFFF_FFFF_8000_0000", isEssential: true },
      { category: "Bootloader", flag: "Multiboot2 / Limine", value: "Direct 64-Bit Entry", description: "Überspringt 16-Bit Real Mode und bootet direkt im Long Mode", isEssential: true },
      { category: "Emulator", flag: "QEMU x86_64", value: "qemu-system-x86_64 -kernel kernel.bin -serial stdio", description: "Komplette CPU-Emulation mit serieller Ausgabe", isEssential: true }
    ],
    summary: "Vollständiger 64-Bit Monolith-Kernel für Intel/AMD Prozessoren mit 4-Level Paging, integriertem Round-Robin Scheduler, VFS-Inode-Dateisystem, APIC-Timer und Ring-0 Hardware-Treibern.",
    memoryLayout: "0x0000_0000_0000_0000 - 0x0000_7FFF_FFFF_FFFF : Userland (128 TB)\n0xFFFF_8000_0000_0000 - 0xFFFF_807F_FFFF_FFFF : Direct Physical Memory Map\n0xFFFF_FFFF_8000_0000 - 0xFFFF_FFFF_FFFF_FFFF : Kernel Image & Ring 0 Heap (2 GB)",
    keyFeatures: [
      "Long Mode Entry & GDT/IDT Table Setup",
      "PML4 -> PDPT -> PD -> PT 4-Level Paging Engine",
      "Direct Ring 0 Syscall Handler via MSR_LSTAR",
      "In-Kernel Virtual File System (VFS) mit Inode Caching",
      "Hardware Port I/O (inb/outb) & MMIO Memory Mapping"
    ],
    files: [
      {
        name: "kernel.lumino",
        type: "kernel",
        description: "Haupt-Kernel-Initialisierung und Boot-Sequenz für x86_64 Long Mode",
        content: `// ATOS x86_64 Monolith Kernel Core
// Architecture: x86_64 | Paradigm: Monolithischer Kernel

let KERNEL_NAME = "ATOS-x86_64-Monolith"
let KERNEL_VERSION = "2.4.0"
let KERNEL_BASE_ADDR = 0xFFFFFFFF80000000
let PAGE_SIZE = 4096

struct CPUContext {
  rax: number, rbx: number, rcx: number, rdx: number,
  rsi: number, rdi: number, rsp: number, rbp: number,
  r8: number,  r9: number,  r10: number, r11: number,
  rip: number, rflags: number, cr3: number
}

// 1. Hardware Abstraction & Paging Setup
func initX86Hardware() {
  print "[x86_64] Initialisiere GDT (Global Descriptor Table)..."
  print "[x86_64] Setze 64-Bit Code-Segment (0x08) & Data-Segment (0x10)..."
  print "[x86_64] Lade TSS (Task State Segment) Deskriptor (0x28)..."
  
  print "[x86_64] Initialisiere 4-Level Paging (PML4 Root)..."
  let pml4_root = 0x1000
  print "[x86_64] Aktiviere CR3 mit PML4 Root: " + pml4_root
  
  print "[x86_64] Initialisiere Local APIC Timer mit 100 Hz Ticks..."
}

// 2. In-Kernel Virtual File System (Monolith VFS)
struct VNode {
  inode: number,
  name: string,
  size: number,
  is_directory: boolean
}

let vfsRoot = { inode: 1, name: "/", size: 4096, is_directory: true }
let openFileHandles = 0

func vfsOpen(path: string) {
  print "[VFS-Monolith] Oeffne Datei direkt im Ring 0 Speicher: " + path
  openFileHandles = openFileHandles + 1
  return openFileHandles
}

// 3. Syscall Dispatcher Table (Ring 0 Direct Dispatch)
func handleSyscall(syscallNumber: number, arg1: any, arg2: any) {
  if syscallNumber == 1 { // SYS_WRITE
    print "[Syscall:SYS_WRITE] FD: " + arg1 + " | Buffer: " + arg2
    return 1
  }
  if syscallNumber == 2 { // SYS_READ
    print "[Syscall:SYS_READ] Lese von Deskriptor " + arg1
    return 0
  }
  if syscallNumber == 3 { // SYS_FORK
    print "[Syscall:SYS_FORK] Klone Thread Context..."
    return 1024
  }
  print "[Syscall] Unbekannter Syscall: " + syscallNumber
  return -1
}

// 4. Kernel Entry Point
func kernelMain() {
  print "=================================================="
  print "Booting " + KERNEL_NAME + " " + KERNEL_VERSION
  print "=================================================="
  
  initX86Hardware()
  
  print "[Monolith] Registriere integrierte Ring 0 Treiber: NVMe, PCI-e, Realtek NIC..."
  vfsOpen("/sys/kernel/status")
  
  print "[Scheduler] Starte prioritätsbasierten Round-Robin Scheduler..."
  print "[Kernel] Kernel erfolgreich gebootet. Starte Init-Prozess..."
}

kernelMain()
`
      },
      {
        name: "paging_x86.lumino",
        type: "hal",
        description: "4-Level Page Table Translation (PML4, PDPT, PD, PT)",
        content: `// x86_64 4-Level Paging Subsystem
// Address Translation: VA[47:39]=PML4 | VA[38:30]=PDPT | VA[29:21]=PD | VA[20:12]=PT

let PAGE_PRESENT = 0x01
let PAGE_WRITABLE = 0x02
let PAGE_USER = 0x04
let PAGE_HUGE = 0x80

func mapVirtualPage(pml4_table: number, virtualAddr: number, physicalAddr: number, flags: number) {
  let pml4_idx = (virtualAddr >> 39) & 0x1FF
  let pdpt_idx = (virtualAddr >> 30) & 0x1FF
  let pd_idx   = (virtualAddr >> 21) & 0x1FF
  let pt_idx   = (virtualAddr >> 12) & 0x1FF

  print "[Paging] Mapping VA: 0x" + virtualAddr + " -> PA: 0x" + physicalAddr
  print "[Paging] PML4[" + pml4_idx + "] -> PDPT[" + pdpt_idx + "] -> PD[" + pd_idx + "] -> PT[" + pt_idx + "]"
}

func initKernelAddressSpace() {
  print "[Paging] Erstelle Kernel-Master-Page-Directory (CR3 = 0x1000)..."
  mapVirtualPage(0x1000, 0xFFFFFFFF80000000, 0x0000000000100000, PAGE_PRESENT + PAGE_WRITABLE)
  print "[Paging] 4GB Direct Physical Memory Identity Mapping abgeschlossen."
}

initKernelAddressSpace()
`
      },
      {
        name: "interrupts.lumino",
        type: "hal",
        description: "IDT (Interrupt Descriptor Table) & Exception Handler",
        content: `// x86_64 Interrupt Descriptor Table (IDT) & Exception Handling
// 256 Interrupt Vektoren mit IST (Interrupt Stack Table) Absicherung

let IDT_ENTRIES = 256
let EXCEPTION_DIVIDE_BY_ZERO = 0
let EXCEPTION_PAGE_FAULT = 14
let EXCEPTION_DOUBLE_FAULT = 8

func registerInterruptGate(vector: number, handlerName: string, istIndex: number) {
  print "[IDT] Vektor 0x" + vector + " -> " + handlerName + " (IST-Stack: " + istIndex + ")"
}

func initIDT() {
  print "[IDT] Initialisiere 256 IDT-Deskriptoren im Kernel-Speicher..."
  registerInterruptGate(0, "isr_divide_by_zero", 0)
  registerInterruptGate(8, "isr_double_fault", 1) // Separater Notfall-Stack
  registerInterruptGate(14, "isr_page_fault", 0)
  registerInterruptGate(32, "isr_apic_timer", 0)
  registerInterruptGate(128, "isr_syscall_gate", 0)
  print "[IDT] LIDT (Load IDT Pointer) Instruktion ausgeführt."
}

initIDT()
`
      }
    ]
  },

  // 2. x86_64 Hierarchie (Protection Rings)
  {
    id: "x86-hierarchy-rings",
    title: "x86_64 Hierarchical Protection Rings (Ring 0-3)",
    archId: "x86_64",
    paradigmId: "hierarchy",
    version: "v2.1.0",
    summary: "Hierarchische Ring-Architektur mit strikter Trennung von Ring 0 (Core Nucleus), Ring 1 (Device Drivers mit I/O-Bitmap-Restriktion) und Ring 3 (Isolierte Userland Tasks).",
    memoryLayout: "Ring 0 (Kernel Nucleus): 0xFFFF_FFFF_8000_0000 (R/W Privileged)\nRing 1 (Driver Services): 0xFFFF_A000_0000_0000 (I/O Port Restricted)\nRing 3 (Userland Apps):   0x0000_0000_0040_0000 (Unprivileged, CR3 Restricted)",
    keyFeatures: [
      "Hardware-enforced Protection Rings (Rings 0, 1, 3)",
      "TSS (Task State Segment) mit RSP0 / RSP1 Stack Switching",
      "IOPB (I/O Permission Bitmap) für isolierte Ring-1 Treiber",
      "Call-Gate & Trap-Gate Privilege Escalation Protection",
      "Interrupt Stack Table (IST) Isolation gegen Ring-3 Stack-Smashing"
    ],
    files: [
      {
        name: "protection_rings.lumino",
        type: "kernel",
        description: "Multi-Ring Protection Engine & Stack-Switch Controller",
        content: `// x86_64 Hierarchical Protection Rings Architecture
// Ring 0: Nucleus | Ring 1: Storage/Network Drivers | Ring 3: User Space

let RING_KERNEL = 0
let RING_DRIVERS = 1
let RING_USER = 3

struct RingState {
  currentRing: number,
  rsp0: number, // Kernel Stack Pointer
  rsp1: number, // Driver Stack Pointer
  iopb_base: number
}

let activeRings = {
  currentRing: 0,
  rsp0: 0xFFFFFFFF80020000,
  rsp1: 0xFFFFA00000010000,
  iopb_base: 0x68
}

func dropPrivilegeToRing(targetRing: number, entryPoint: string) {
  if targetRing == 1 {
    print "[Ring-Hierarchy] Wechsle von Ring 0 -> Ring 1 (Driver Space)..."
    print "[TSS] Lade Driver-Stack RSP1: 0x" + activeRings.rsp1
    print "[IOPB] Aktiviere I/O-Port Whitelist für PCI & Storage Controller..."
    activeRings.currentRing = 1
  } else if targetRing == 3 {
    print "[Ring-Hierarchy] Wechsle von Ring 0 -> Ring 3 (Userland)..."
    print "[TSS] Setze RSP0 für naechsten Syscall-Ruecksprung..."
    print "[CPU] PFLAGS.IOPL = 0 (Direkter Hardware-Zugriff blockiert)"
    activeRings.currentRing = 3
  }
  print "[Privilege] Ausführung gestartet in Ring " + activeRings.currentRing + " bei " + entryPoint
}

func handlePrivilegedInstructionTrap(instruction: string, attemptedRing: number) {
  if attemptedRing > 0 {
    print "[SECURITY TRAP] #GP (General Protection Fault): Instruktion '" + instruction + "' in Ring " + attemptedRing + " verboten!"
    print "[Ring 0 Nucleus] Terminiere fehlerhaften Prozess/Treiber sicher."
  }
}

func initProtectionRings() {
  print "=== Initialisiere x86_64 Hierarchische Schutzringe ==="
  print "[GDT] Ring 0 Code (0x08), Ring 1 Code (0x1B), Ring 3 Code (0x33)"
  print "[TSS] Task State Segment konfiguriert mit 7 IST Stacks."
  
  dropPrivilegeToRing(1, "network_driver_main()")
  dropPrivilegeToRing(3, "user_shell_main()")
}

initProtectionRings()
`
      },
      {
        name: "tss_gate.lumino",
        type: "hal",
        description: "Task State Segment (TSS) & Call Gate Descriptors",
        content: `// Task State Segment (TSS) & Gate Descriptors
// Regelt privilege Übergänge und sichere Stack-Switches

struct TSS64 {
  reserved0: number,
  rsp0: number, rsp1: number, rsp2: number,
  reserved1: number,
  ist1: number, ist2: number, ist3: number, ist4: number,
  iopb_offset: number
}

func createCallGate(targetSelector: number, targetOffset: number, dpl: number) {
  print "[Call-Gate] Erstelle Deskriptor: DPL=" + dpl + " -> Target Selector 0x" + targetSelector
}

func setupTSS() {
  let tss = {
    rsp0: 0xFFFFFFFF80090000,
    rsp1: 0xFFFFA00000050000,
    ist1: 0xFFFFFFFF800F0000, // Double fault emergency stack
    iopb_offset: 104
  }
  print "[TSS] LTR (Load Task Register) ausgeführt mit Selector 0x28."
  createCallGate(0x08, 0xFFFFFFFF80001000, 3)
}

setupTSS()
`
      }
    ]
  },

  // 3. ARM64 Layered (Schichtenarchitektur)
  {
    id: "arm64-layered-arch",
    title: "ARM AArch64 5-Layer System Architecture",
    archId: "arm64",
    paradigmId: "layered",
    version: "v3.0.0",
    isRecommended: true,
    recommendedBadge: "★ TOP-EMPFEHLUNG: SCHICHTEN- & HYBRID-KERNEL",
    recommendedReason: "Die bewährte Vorlage für modulare, portable Betriebssysteme (wie Windows NT Kernel oder macOS XNU): Strikte Trennung von Hardware Abstraction Layer (HAL), Core Executive und Syscall Gateway.",
    recommendedSettings: [
      { category: "Architektur", flag: "AArch64 EL0 / EL1", value: "Exception Levels", description: "EL1 OS Kernel, EL0 isoliertes Userland", isEssential: true },
      { category: "Paging", flag: "Split TTBR0 / TTBR1", value: "48-Bit Virtual Address Space", description: "TTBR0 für User-Space (0x0000...), TTBR1 für Kernel (0xFFFF...)", isEssential: true },
      { category: "Interrupt Controller", flag: "ARM GICv3 Distributor", value: "Multi-Core IRQs", description: "Standardisierter Interrupt-Distributor für AArch64", isEssential: true },
      { category: "Compiler", flag: "-ffreestanding -nostdlib", value: "Aktiviert", description: "Vollständig unabhängiger Bare-Metal Build", isEssential: true }
    ],
    summary: "Strikte 5-Schichten-Architektur für 64-Bit ARM Prozessoren: HAL (Layer 0) -> Core Kernel (Layer 1) -> Services (Layer 2) -> Syscall API (Layer 3) -> Applications (Layer 4).",
    memoryLayout: "Layer 4 (Apps):     0x0000_0000_0000_0000 (TTBR0_EL0, 48-Bit Virtual Address Space)\nLayer 2/3 (Serv):   0xFFFF_8000_0000_0000 (TTBR1_EL1, Kernel Space Services)\nLayer 0/1 (HAL):    0xFFFF_FFFF_0000_0000 (Physical MMIO & Device Tree Mappings)",
    keyFeatures: [
      "Strict 5-Layer unidirectional dependency model",
      "Clean Hardware Abstraction Layer (HAL) for Apple Silicon / Graviton / RPi",
      "GICv3 Interrupt Controller Abstraction",
      "Split TTBR0 (User) & TTBR1 (Kernel) Translation Tables",
      "Decoupled Device Tree (FDT) Hardware Auto-Discovery"
    ],
    files: [
      {
        name: "layer0_hal.lumino",
        type: "hal",
        description: "Layer 0: Hardware Abstraction Layer (HAL / MMIO / CPU Registers)",
        content: `// LAYER 0: Hardware Abstraction Layer (HAL)
// Kapselt hardware-spezifische ARM AArch64 MMIO und Register

let GIC_DIST_BASE  = 0xFFFF000008000000
let UART_BASE      = 0xFFFF000009000000
let TIMER_FREQ_HZ  = 62500000

func halMmioRead32(address: number) {
  return 0xAA55AA55
}

func halMmioWrite32(address: number, val: number) {
  print "[HAL-L0] MMIO Write32: [0x" + address + "] = 0x" + val
}

func halInitCpuGic() {
  print "[HAL-L0] Initialisiere ARM GICv3 Distributor & CPU Interface..."
  halMmioWrite32(GIC_DIST_BASE + 0x0000, 0x03) // Enable Group 0/1
  print "[HAL-L0] GICv3 bereit. Setze VBAR_EL1 Exception Vector Table..."
}

func halUartPrint(msg: string) {
  print "[HAL-UART] " + msg
}

halInitCpuGic()
`
      },
      {
        name: "layer1_kernel_core.lumino",
        type: "kernel",
        description: "Layer 1: Core Kernel (Task Scheduler, Sync Primitives, Memory Allocation)",
        content: `// LAYER 1: Core Kernel Executive
// Baut ausschliesslich auf Layer 0 (HAL) auf

struct TaskDescriptor {
  id: number,
  name: string,
  state: string,
  priority: number,
  quantum_left: number
}

let taskQueue = [
  { id: 1, name: "init", state: "READY", priority: 10, quantum_left: 20 },
  { id: 2, name: "storage_daemon", state: "READY", priority: 8, quantum_left: 15 }
]

func kernelScheduleNext() {
  let nextTask = taskQueue[0]
  print "[Core-L1] Scheduler wählt Task ID " + nextTask.id + " (" + nextTask.name + ") für Core 0"
  return nextTask
}

func kernelAllocatePage() {
  print "[Core-L1] Allokiere 4KB Page Frame aus physischem Buddy-Allocator."
  return 0x80004000
}

kernelScheduleNext()
`
      },
      {
        name: "layer2_services.lumino",
        type: "driver",
        description: "Layer 2: Executive Services (VFS, IPC Channels, Security Reference Monitor)",
        content: `// LAYER 2: Executive Services
// Baut auf Layer 1 (Scheduler/Memory) auf

func vfsServiceResolvePath(path: string) {
  print "[Services-L2] VFS: Löse Inode-Pfad auf: " + path
  return { inode: 42, permissions: "rwxr-xr-x", size: 1024 }
}

func ipcCreateMessageChannel(name: string) {
  print "[Services-L2] IPC: Erstelle sicheren Message-Kanal: " + name
  return { channelId: 101, bufferSize: 65536 }
}

vfsServiceResolvePath("/etc/atos.conf")
`
      },
      {
        name: "layer3_syscall_gateway.lumino",
        type: "syscall",
        description: "Layer 3: System Call Interface & Parameter Sanitization Gateway",
        content: `// LAYER 3: System Call Gateway
// Schützt Layer 2/1 vor unberechtigten Userland-Parametern (Layer 4)

func syscallDispatch(svcNumber: number, param1: any, param2: any) {
  print "[Syscall-L3] SVC Exception gefangen. Parameter-Validierung..."
  if svcNumber == 100 { // Open File
    return vfsServiceResolvePath(param1)
  }
  print "[Syscall-L3] Unbekannter Syscall Code: " + svcNumber
  return null
}
`
      },
      {
        name: "layer4_app.lumino",
        type: "user",
        description: "Layer 4: User Application & Shell Subsystem",
        content: `// LAYER 4: Userland Application
// Nutzt strikt ausschliesslich Layer 3 (Syscalls)

func userAppMain() {
  print "[App-L4] Starte Benutzeroberfläche & Dashboard..."
  let file = syscallDispatch(100, "/var/log/syslog", 0)
  print "[App-L4] Datei erfolgreich geoeffnet: Inode " + file.inode
}

userAppMain()
`
      }
    ]
  },

  // 4. RISC-V Modular (Microkernel / LKM)
  {
    id: "riscv-modular-microkernel",
    title: "RISC-V Microkernel & Loadable Module Framework",
    archId: "riscv",
    paradigmId: "modular",
    version: "v4.2.0",
    summary: "Modulares Microkernel-Framework für RISC-V (RV64GC) mit synchronem/asynchronem IPC Message Passing, Capability-basiertem Zugriffsschutz und dynamisch ladbaren Modulen (.mod).",
    memoryLayout: "Microkernel Core (S-Mode): 0x8020_0000 (satp root)\nUser-Space Server (U-Mode): 0x0001_0000_0000 (VFS, NET, GPU)\nIPC Zero-Copy Ringbuffer:   0x0000_0000_8000_0000 (Shared Grant Pages)",
    keyFeatures: [
      "Minimaler Supervisor Microkernel (~10k LoC Footprint)",
      "Capability-basiertes Token-Sicherheitssystem für IPC",
      "Isolierte User-Space Server (VFS, Network, Block Driver)",
      "Dynamic Loadable Kernel Module (LKM) Symbol Resolver",
      "RISC-V CSR Manipulation (satp, sstatus, stvec, scause)"
    ],
    files: [
      {
        name: "microkernel_core.lumino",
        type: "kernel",
        description: "RISC-V Microkernel Core: Minimaler Scheduler, IPC & Capability Manager",
        content: `// RISC-V RV64GC Modular Microkernel Core
// Mode: S-Mode (Supervisor) | Paradigm: Microkernel + IPC

let CSR_SATP = 0x180
let CSR_SSTATUS = 0x100
let CSR_STVEC = 0x105

struct Capability {
  grantId: number,
  targetServer: string,
  permissions: number // 1=Read, 2=Write, 4=Exec, 8=IPC
}

struct IPCMessage {
  msgId: number,
  senderTaskId: number,
  receiverTaskId: number,
  payload: string,
  capabilityToken: number
}

let activeModules = []

func kernelSendIPC(msg: IPCMessage) {
  print "[Microkernel-IPC] Zustellung Nachricht #" + msg.msgId + " von Task " + msg.senderTaskId + " -> " + msg.receiverTaskId
  print "[Microkernel-IPC] Überprüfe Capability Token 0x" + msg.capabilityToken + "..."
  print "[Microkernel-IPC] Zero-Copy Page-Grant aktiviert. Server geweckt."
  return true
}

func loadKernelModule(moduleName: string, moduleSize: number) {
  print "[Module-Loader] Lade dynamisches Kernel-Modul: " + moduleName + " (" + moduleSize + " bytes)"
  print "[Module-Loader] Löse Symbole auf (kalloc, register_driver, ipc_listen)..."
  activeModules.push(moduleName)
  print "[Module-Loader] Modul '" + moduleName + "' erfolgreich gebunden und aktiv."
}

func initRiscVMicrokernel() {
  print "=== RISC-V RV64GC Microkernel Booting ==="
  print "[CSR] stvec konfiguriert auf Exception Handler Base: 0x80201000"
  print "[CSR] satp (Sv39 Paging) aktiviert."
  
  loadKernelModule("driver_virtio_blk.mod", 18432)
  loadKernelModule("driver_pcie_nvme.mod", 32768)
  
  print "[Microkernel] Starte User-Space Server: vfs_server, net_server, ui_server..."
}

initRiscVMicrokernel()
`
      },
      {
        name: "vfs_server.lumino",
        type: "driver",
        description: "Isolierter User-Space VFS Server (Kommuniziert via IPC)",
        content: `// User-Space VFS Server (Läuft im unprivilegierten U-Mode)
// Fehler hier bringen NICHT das Gesamtsystem zum Absturz!

func vfsServerLoop() {
  print "[VFS-Server] Server gestartet im isolierten Adressraum (U-Mode)..."
  print "[VFS-Server] Warte auf eingehende IPC-Anfragen über Port 0x4F..."
  
  let incomingReq = {
    msgId: 101,
    senderTaskId: 5,
    receiverTaskId: 2,
    payload: "OPEN /system/app.bin",
    capabilityToken: 0xCAFE
  }
  
  print "[VFS-Server] Verarbeite: " + incomingReq.payload
  print "[VFS-Server] Sende Antwort-Token via IPC zurück an Task " + incomingReq.senderTaskId
}

vfsServerLoop()
`
      },
      {
        name: "module_spec.lumino",
        type: "config",
        description: "LKM (Loadable Kernel Module) Interface & Header Definition",
        content: `// Dynamic Kernel Module Specification (.mod)
// Unterstützt Live-Patching und dynamisches Nachladen

struct ModuleHeader {
  magic: string,        // "ATOS_MOD"
  version: number,      // 1
  name: string,
  author: string,
  dependencies: string[],
  init_symbol: string,
  cleanup_symbol: string
}

let sampleModule = {
  magic: "ATOS_MOD",
  version: 1,
  name: "crypto_accelerator_rv64",
  author: "ATOS Security Labs",
  dependencies: ["microkernel_core", "dma_engine"],
  init_symbol: "init_crypto_accel",
  cleanup_symbol: "cleanup_crypto_accel"
}

print "[LKM-Spec] Modul-Header validiert: " + sampleModule.name
`
      }
    ]
  },

  // 5. ARM Cortex-M Embedded RTOS
  {
    id: "cortex-m-layered-rtos",
    title: "ARM Cortex-M Realtime Embedded System (RTOS)",
    archId: "cortex_m",
    paradigmId: "layered",
    version: "v1.8.0",
    summary: "Deterministisches Echtzeitbetriebssystem (RTOS) für STM32/NXP Cortex-M Mikrocontroller mit MPU-Speicherschutz, NVIC-Interrupt-Priorisierung und PendSV-Task-Switching.",
    memoryLayout: "Flash (Code/Vectors): 0x0800_0000 (512 KB, Read-Only MPU Region 0)\nSRAM (Kernel Heap):   0x2000_0000 (128 KB, Privileged MPU Region 1)\nSRAM (Task Stacks):   0x2001_0000 (Unprivileged PSP Stacks mit Guard Regions)",
    keyFeatures: [
      "Hardware MPU (Memory Protection Unit) Stack Guard Regions",
      "NVIC Nested Interrupt Controller mit 16 Prioritätsstufen",
      "SysTick 1ms Hardware Tick Generator",
      "PendSV Zero-Latency Context Switching",
      "Ultra-low Power Mode (WFI/WFE Sleep Management)"
    ],
    files: [
      {
        name: "rtos_kernel.lumino",
        type: "kernel",
        description: "Cortex-M RTOS Task Scheduler & PendSV Context Switcher",
        content: `// ARM Cortex-M4/M7 Real-Time Operating System
// Target: STM32 / NXP LPC | MPU Protected

let NVIC_ICSR = 0xE000ED04
let PENDSV_SET = (1 << 28)
let SYSTICK_RELOAD = 168000 // 1ms bei 168 MHz

struct TaskTCB {
  id: number,
  psp: number, // Process Stack Pointer
  priority: number,
  stackBase: number,
  stackLimit: number
}

let tasks = [
  { id: 1, psp: 0x20010800, priority: 1, stackBase: 0x20010000, stackLimit: 0x20010800 },
  { id: 2, psp: 0x20011800, priority: 2, stackBase: 0x20011000, stackLimit: 0x20011800 }
]

func initSysTick() {
  print "[Cortex-M] Konfiguriere SysTick Timer auf 1000 Hz (1ms Tick)..."
}

func initMPU() {
  print "[MPU] Aktiviere Memory Protection Unit..."
  print "[MPU] Region 0: Flash 0x08000000 (Read/Exec Privileged & Unprivileged)"
  print "[MPU] Region 1: Kernel SRAM 0x20000000 (Privileged Only)"
  print "[MPU] Region 2: Stack Guard 0x20010000 (No Access - Trap Stack Overflow!)"
}

func triggerContextSwitch() {
  print "[RTOS] Triggere PendSV Exception über NVIC ICSR..."
  print "[PendSV] Speichere Register R4-R11 auf PSP..."
  print "[PendSV] Lade neuen Task TCB Context..."
}

func main() {
  print "=== Booting ARM Cortex-M RTOS ==="
  initMPU()
  initSysTick()
  triggerContextSwitch()
}

main()
`
      }
    ]
  },

  // 6. ESP32 / Embedded Dual-Core Modular
  {
    id: "esp32-modular-iot",
    title: "ESP32 Dual-Core & WASM Sandbox Architecture",
    archId: "embedded_esp32",
    paradigmId: "modular",
    version: "v2.0.0",
    summary: "Modulares Dual-Core Framework für ESP32 SoCs: Core 0 übernimmt Kommunikations-Stacks (WiFi/BLE/Mesh), Core 1 führt Anwendungslogik & isolierte WASM-Sandboxes aus.",
    memoryLayout: "Core 0 Dedicated SRAM: 0x3FFE_0000 (Radio & Protocol Buffers)\nCore 1 App Memory:     0x3FFA_0000 (Application & Thread Heap)\nExternal PSRAM:        0x3F80_0000 (4MB WASM Linear Memory Sandbox)",
    keyFeatures: [
      "Dual-Core Hardware Affinity (Core 0: Protocol, Core 1: User App)",
      "WebAssembly (WASM) Micro-Runtime Sandbox Isolation",
      "Non-blocking FreeRTOS Event Queues & Ringbuffers",
      "Hardware Cryptographic Accelerator Integration (AES/SHA/RSA)",
      "Dynamic Over-The-Air (OTA) Partition Slot Management"
    ],
    files: [
      {
        name: "dual_core_engine.lumino",
        type: "kernel",
        description: "ESP32 Dual-Core Task Affinity & Inter-Core Ringbuffer",
        content: `// ESP32 Dual-Core Modular Architecture
// Core 0: Protocol Stacks | Core 1: User Application & WASM Sandbox

func initDualCoreSubsystem() {
  print "[ESP32] Initialisiere Core 0: Protocol Management Engine..."
  print "[ESP32] Starte WiFi 802.11 & BLE 5.0 Stack auf Core 0..."
  
  print "[ESP32] Initialisiere Core 1: Application Processor..."
  print "[ESP32] Starte WebAssembly Micro-Runtime Sandbox auf Core 1..."
}

func sendInterCoreMessage(queueName: string, data: string) {
  print "[InterCore-Queue] Sende Daten von Core 0 -> Core 1 via Lock-Free Ringbuffer: " + data
}

initDualCoreSubsystem()
sendInterCoreMessage("radio_rx_queue", "PACKET:LEN=128:CRC=OK")
`
      }
    ]
  }
];

export const CODE_LIBRARY_MODULES: CodeModuleItem[] = [
  // x86_64 Hardware Core
  {
    id: "mod-x86-gdt-idt",
    title: "x86_64 GDT & IDT Deskriptoren",
    archId: "x86_64",
    paradigmId: "monolith",
    category: "x86_64 Hardware Core",
    isRecommended: true,
    recommendedBadge: "★ ESSENTIELL FÜR RING 0",
    recommendedReason: "Zwingend erforderlich für 64-Bit Segment-Deskriptoren, 256 Interrupt-Vektoren und TSS-Stackwechsel (RSP0).",
    description: "64-Bit Global Descriptor Table (GDT) und Interrupt Descriptor Table (IDT) Setup mit TSS.",
    tags: ["x86_64", "GDT", "IDT", "TSS", "Interrupts"],
    code: `// x86_64 GDT & IDT Initialisierung
func setupGdtAndIdt() {
  print "[x86_64] Setze GDT: Kernel Code 0x08, Kernel Data 0x10, User Data 0x23, User Code 0x2B"
  print "[x86_64] Lade TSS (Task State Segment) Deskriptor in TR-Register"
  print "[x86_64] Initialisiere IDT mit 256 Interrupt Vektoren"
}
setupGdtAndIdt()`
  },
  {
    id: "mod-x86-msr-syscall",
    title: "x86_64 Fast Syscall via MSR_LSTAR",
    archId: "x86_64",
    paradigmId: "monolith",
    category: "x86_64 Hardware Core",
    isRecommended: true,
    recommendedBadge: "★ EMPFOHLEN: FAST SYSCALL",
    recommendedReason: "Moderner 64-Bit Standard für Systemaufrufe über IA32_LSTAR (Hardware-Beschleunigung ohne INT 0x80 Overhead).",
    description: "Konfiguriert Model-Specific Registers (MSR) für hardwarebeschleunigte SYSCALL/SYSRET Instruktionen.",
    tags: ["x86_64", "MSR", "Syscall", "Ring 0", "LSTAR"],
    code: `// x86_64 MSR Fast Syscall Setup
let IA32_EFER = 0xC0000080
let IA32_STAR = 0xC0000081
let IA32_LSTAR = 0xC0000082
let IA32_SFMASK = 0xC0000084

func configureFastSyscall(entryHandlerAddr: number) {
  print "[MSR] Setze IA32_LSTAR auf Handler 0x" + entryHandlerAddr
  print "[MSR] Setze IA32_STAR (GDT Selektoren für Ring 0 & Ring 3)"
  print "[MSR] Setze IA32_SFMASK (Maskiere RFLAGS IF, DF, TF bei Syscall-Eintritt)"
}
configureFastSyscall(0xFFFFFFFF80002000)`
  },

  // ARM64 Architecture
  {
    id: "mod-arm64-vbar-vectors",
    title: "ARM64 Vector Base Address Register (VBAR_EL1)",
    archId: "arm64",
    paradigmId: "layered",
    category: "ARM AArch64 Core",
    description: "Vektortabellen-Setup für ARMv8/ARMv9 mit 16 Exception-Eintrittspunkten (Synchronous, IRQ, FIQ, SError).",
    tags: ["ARM64", "AArch64", "VBAR", "EL1", "Exceptions"],
    code: `// ARM64 Exception Vector Table Setup
func initVbarVectors(vectorTableBase: number) {
  print "[AArch64] Setze VBAR_EL1 = 0x" + vectorTableBase
  print "[AArch64] 16 Exception Vektoren (Current EL with SP0/SPx, Lower EL AArch64/AArch32) aktiv."
}
initVbarVectors(0xFFFF800000080000)`
  },
  {
    id: "mod-arm64-ttbr-split",
    title: "ARM64 Split Virtual Memory (TTBR0 / TTBR1)",
    archId: "arm64",
    paradigmId: "layered",
    category: "ARM AArch64 Core",
    description: "Split-Paging Translation Tables für vollständige Trennung von User- (TTBR0_EL1) und Kernel-Space (TTBR1_EL1).",
    tags: ["ARM64", "MMU", "TTBR0", "TTBR1", "Paging"],
    code: `// ARM64 Split Virtual Memory Configuration
func setupArm64Mmu(ttbr0User: number, ttbr1Kernel: number) {
  print "[MMU-ARM64] TTBR0_EL1 (User Space 0x0000...): 0x" + ttbr0User
  print "[MMU-ARM64] TTBR1_EL1 (Kernel Space 0xFFFF...): 0x" + ttbr1Kernel
  print "[MMU-ARM64] TCR_EL1: T0SZ=16, T1SZ=16 (48-Bit Virtual Address Space), Granule 4KB"
}
setupArm64Mmu(0x80001000, 0x80002000)`
  },

  // RISC-V Supervisor & Machine
  {
    id: "mod-riscv-csr-satp",
    title: "RISC-V Sv39 Paging via satp CSR",
    archId: "riscv",
    paradigmId: "modular",
    category: "RISC-V Architecture",
    description: "Control & Status Register Konfiguration für 3-Level Sv39 Page Table Translation.",
    tags: ["RISC-V", "CSR", "satp", "Sv39", "Paging"],
    code: `// RISC-V Sv39 Page Table CSR Setup
let SATP_MODE_SV39 = (8 << 60)

func enableRiscVPaging(rootPageTablePhysAddr: number, asid: number) {
  let ppn = (rootPageTablePhysAddr >> 12)
  let satpValue = SATP_MODE_SV39 | (asid << 44) | ppn
  print "[RISC-V] Schreibe CSR satp = 0x" + satpValue
  print "[RISC-V] Führe 'sfence.vma' aus (TLB Cache Flush)..."
}
enableRiscVPaging(0x80200000, 1)`
  },

  // Monolith Subsystems
  {
    id: "mod-monolith-vfs-cache",
    title: "Monolith In-Memory VFS Inode Cache",
    paradigmId: "monolith",
    category: "Monolith Subsystems",
    isRecommended: true,
    recommendedBadge: "★ EMPFOHLEN: DATEISYSTEM-CORE",
    recommendedReason: "Ultraschneller In-Memory Inode Cache für atomare Ring-0 Dateizugriffe ohne Interprozess-Overhead.",
    description: "Direkter Ring-0 Inode-Cache für ultraschnelle Dateisystem-Operationen ohne Context-Switches.",
    tags: ["Monolith", "VFS", "Inode", "Cache", "High-Performance"],
    code: `// Monolith In-Memory VFS Cache
let inodeCache = {}

func getCachedInode(inodeNumber: number) {
  if inodeCache[inodeNumber] {
    print "[Monolith-VFS] Cache Hit für Inode " + inodeNumber
    return inodeCache[inodeNumber]
  }
  let newInode = { id: inodeNumber, name: "node_" + inodeNumber, size: 4096 }
  inodeCache[inodeNumber] = newInode
  return newInode
}
`
  },

  // Hierarchical Ring Protection
  {
    id: "mod-hierarchy-gate-trap",
    title: "Hierarchical Call-Gate Trap Handler",
    paradigmId: "hierarchy",
    category: "Hierarchische Schutzringe",
    description: "Überwacht Ringübergänge und verhindert unautorisierte Privilege Escalation zwischen Ringen.",
    tags: ["Protection Rings", "Call-Gate", "Trap", "Security"],
    code: `// Hierarchical Privilege Transition Guard
func validateRingTransition(sourceRing: number, targetRing: number, securityToken: number) {
  if sourceRing > targetRing {
    print "[Ring-Guard] Privilege Escalation von Ring " + sourceRing + " nach Ring " + targetRing
    if securityToken == 0xAA55FF00 {
      print "[Ring-Guard] Token gültig. Erlaube kontrollierten Übergang."
      return true
    }
    print "[SECURITY ALERT] Ungültiges Token! Übergang verweigert."
    return false
  }
  return true
}
`
  },

  // Layered HAL & Driver Stack
  {
    id: "mod-layered-hal-interface",
    title: "Layered Hardware Abstraction Interface (HAL)",
    paradigmId: "layered",
    category: "Schichten- / Layer-Modell",
    isRecommended: true,
    recommendedBadge: "★ TOP ARCHITEKTUR: HAL",
    recommendedReason: "Ermöglicht vollständige Entkopplung von CPU-Architektur und Treibern – Grundlage für portable Betriebssysteme (wie Windows NT HAL).",
    description: "Standardisiertes HAL-Interface, das CPU-unabhängige Zugriffe auf GPIO, Timer, MMIO und Interrupts bereitstellt.",
    tags: ["HAL", "Schichtenarchitektur", "Layer 0", "Abstraction"],
    code: `// Universal Hardware Abstraction Layer (HAL) Interface
struct HalDevice {
  name: string,
  baseAddress: number,
  irqNumber: number,
  readReg: (offset: number) => number,
  writeReg: (offset: number, val: number) => void
}

func createHalDevice(name: string, baseAddr: number, irq: number): HalDevice {
  return {
    name: name,
    baseAddress: baseAddr,
    irqNumber: irq,
    readReg: func(offset: number) { return 0 },
    writeReg: func(offset: number, val: number) { print "[HAL:" + name + "] Write 0x" + val }
  }
}`
  },

  // Modular IPC & Microkernel
  {
    id: "mod-modular-ipc-channel",
    title: "Fast IPC Message Passing & Capability Grant",
    paradigmId: "modular",
    category: "Modulare & Microkernel Systeme",
    description: "Sicheres Zero-Copy IPC-Protokoll mit Capability-basierten Zugriffsrechten für isolierte User-Space Server.",
    tags: ["Microkernel", "IPC", "Capability", "Zero-Copy", "Modular"],
    code: `// Fast Microkernel IPC Protocol
struct IPCEnvelope {
  msgType: number,
  senderEndpoint: number,
  receiverEndpoint: number,
  grantPage: number,
  size: number
}

func sendIpcMessage(envelope: IPCEnvelope) {
  print "[Microkernel-IPC] Sende " + envelope.size + " Bytes von Endpoint " + envelope.senderEndpoint + " -> " + envelope.receiverEndpoint
  print "[Microkernel-IPC] Grant Page: 0x" + envelope.grantPage + " (Zero-Copy Mapping)"
}`
  }
];
