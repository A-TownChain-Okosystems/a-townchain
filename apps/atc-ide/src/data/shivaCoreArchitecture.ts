// ShivaCore / Globus OS - Full-Stack 10-Layer Architecture Data Model & Specification
// Comprehensive multi-tier definition encompassing Windows/Linux-Kernel-grade foundations,
// ShivaCore Microkernel, ATC VM, Globus OS, and A-TownChain Web3/P2P substrate.

export interface ArchLayerDetail {
  id: string;
  number: number;
  name: string;
  shortName: string;
  headline: string;
  color: string;
  badgeBg: string;
  badgeBorder: string;
  textColor: string;
  components: {
    title: string;
    description: string;
    techStack: string[];
    role: string;
  }[];
  interfaces: {
    type: string;
    protocol: string;
    description: string;
  }[];
  securityRole: string;
  diagram: string;
}

export interface PlatformItem {
  id: number;
  name: string;
  leadTech: string;
  objective: string;
  keyDeliverables: string[];
  scope: string;
}

export interface SyscallDef {
  code: string;
  opcode: string;
  category: "Process" | "Thread" | "Memory" | "IPC" | "Handle" | "File" | "Network" | "Capability" | "Device" | "Time" | "Audit" | "VM";
  signature: string;
  registers: string;
  description: string;
  capabilityRequired: string;
  ring: "Ring 3 -> 0" | "EL0 -> EL1" | "U -> S";
}

export interface CapabilityStep {
  step: number;
  name: string;
  description: string;
  codeSnippet: string;
  enforcement: string;
}

export interface VerticalChainStep {
  index: number;
  layer: string;
  action: string;
  payload: string;
  privilege: string;
  boundary?: string;
}

export const SHIVACORE_LAYERS: ArchLayerDetail[] = [
  {
    id: "layer-1-experience",
    number: 1,
    name: "Experience Layer",
    shortName: "Experience / UI",
    headline: "Aurora UI & Multimodal Human-Machine Interface",
    color: "cyan",
    badgeBg: "bg-cyan-500/10",
    badgeBorder: "border-cyan-500/30",
    textColor: "text-cyan-400",
    components: [
      {
        title: "Aurora Desktop",
        description: "Moderne Multi-Window Desktop-Oberfläche mit Tiling-WM, GPU-beschleunigtem Compositor und Wayland-Protokoll.",
        techStack: ["Rust", "Wayland", "WebGPU", "Aurora Design System"],
        role: "Primäre Workstation-Schnittstelle",
      },
      {
        title: "Aurora Mobile",
        description: "Gestenbasierte Touch-Umgebung mit adaptiver Skalierung für Smartphones, Tablets und faltbare Displays.",
        techStack: ["React Native", "TypeScript", "Vulkan / Metal"],
        role: "Mobile Client-Plattform",
      },
      {
        title: "Web Portal & Admin Console",
        description: "Zero-Install Web-Interface zur Systemadministration, Cluster-Überwachung und Remote-Wartung.",
        techStack: ["React", "TypeScript", "Tailwind CSS", "WASM"],
        role: "Cloud- & Browser-Zugang",
      },
      {
        title: "Wallet & Explorer UI",
        description: "Hardware-gesicherte Wallet-Oberfläche, Transaktionssignierung und Echtzeit-Blockchain-Explorer.",
        techStack: ["TypeScript", "WebAssembly", "WebCrypto API"],
        role: "Finanz- & Web3-Identitätszentrum",
      },
      {
        title: "Game Launcher & Game UI",
        description: "High-Performance Low-Latency Rendering-Oberfläche mit Gamepad-Unterstützung und Spatial Audio.",
        techStack: ["WebGPU", "Rust Core", "Shader Stage", "AudioEngine"],
        role: "Interaktive 2D/3D Gaming Experience",
      },
      {
        title: "AI Assistant & Developer Console",
        description: "Natürliche Sprachschnittstelle für Systemsteuerungen, Skriptgenerierung und interaktive Kernel-Telemetrie.",
        techStack: ["LLM Streaming", "WebSockets", "Terminal Emulation"],
        role: "Intelligente Co-Pilot-Bedienung",
      },
    ],
    interfaces: [
      { type: "UI Runtime", protocol: "Wayland IPC / DOM Event Loop", description: "Event-Dispatched Rendering & Input Injection" },
      { type: "API Client", protocol: "gRPC-Web / WebSocket / JSON-RPC", description: "Verbindung zu Middleware und Gateway" },
    ],
    securityRole: "Isolierter User Space ohne direkte Hardwareprivilegien. Alle Aktionen erfordern signierte UI-Tokens.",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                    EXPERIENCE LAYER (L1)                   │
├─────────────────────────────────────────────────────────────┤
│  Desktop  │  Mobile  │  Web  │  Game  │  VR/AR  │  CLI  │ AI │
│  Aurora UI / React / TypeScript / WebGPU / Native Rust UI   │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-2-application",
    number: 2,
    name: "Application Layer",
    shortName: "Applications",
    headline: "Dezentrale & Native Systemanwendungen",
    color: "blue",
    badgeBg: "bg-blue-500/10",
    badgeBorder: "border-blue-500/30",
    textColor: "text-blue-400",
    components: [
      {
        title: "Globus OS Native Apps",
        description: "Kompilierte Systemwerkzeuge, Dateimanager, Texteditoren und Multimediatools.",
        techStack: ["Rust", "Lumino", "C++23"],
        role: "Alltägliche Betriebssystem-Workflows",
      },
      {
        title: "AI Autonomous Agents",
        description: "Eigenständige Agenten mit deklarativen Zielvorgaben, lokalem Knowledge-Store und Tool-Calling.",
        techStack: ["Agent Runtime", "Vector Embeddings", "Prompt Engine"],
        role: "Autonome Workflow-Automation",
      },
      {
        title: "DeFi & Governance DApps",
        description: "Dezentrale Börsen, Liquiditätspools, Staking-Dashboards und DAO-Abstimmungsportale.",
        techStack: ["ATCLang", "Solidity/Vyper Transpiler", "Web3 SDK"],
        role: "Wirtschafts- und Konsensus-Anwendungen",
      },
      {
        title: "Studio & Developer Tools",
        description: "Integrierte Entwicklungsumgebung (IDE), Profiler, Disassembler und Debugging-Tools.",
        techStack: ["LSP", "DAP Debugger", "Prism Highlighter", "Live Audit"],
        role: "Software-Engineering auf Globus OS",
      },
      {
        title: "Marketplace & NFT Ecosystem",
        description: "Verteilter Paket- und Asset-Store für Module, Spiele, Themes und verifizierte Smart Contracts.",
        techStack: ["IPFS / Arweave Gateway", "Smart Contracts", "Metadata Engine"],
        role: "Ökosystem-Vertriebsplattform",
      },
    ],
    interfaces: [
      { type: "SDK Bindings", protocol: "Native FFI / TypeScript Typings", description: "Direkter Aufruf von SDK-Methoden" },
      { type: "Sandbox Bridge", protocol: "WASM ABI / seccomp-bpf", description: "Einschränkung der Berechtigungen" },
    ],
    securityRole: "Gekapselte Sandbox-Prozesse mit striktem Principle-of-Least-Privilege (PoLP).",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                   APPLICATION LAYER (L2)                    │
├─────────────────────────────────────────────────────────────┤
│  Apps  │  Games  │  Wallet  │  Marketplace  │  Studio       │
│  AI Agents  │  Developer Tools  │  Governance  │  DeFi     │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-3-sdk-api",
    number: 3,
    name: "SDK / API Layer",
    shortName: "SDK & APIs",
    headline: "Universelle Entwickler-Kits & Kommunikationsschnittstellen",
    color: "indigo",
    badgeBg: "bg-indigo-500/10",
    badgeBorder: "border-indigo-500/30",
    textColor: "text-indigo-400",
    components: [
      {
        title: "ATC SDK & ShivaCore SDK",
        description: "Typensichere Bibliotheken zur Interaktion mit Blockchain, Accounts, VM und Kernel-Handles.",
        techStack: ["TypeScript SDK", "Rust Crate", "C FFI Headers"],
        role: "Programmierschnittstellen für Entwickler",
      },
      {
        title: "Agent & Game SDKs",
        description: "High-Level Frameworks für KI-Agenten, ECS (Entity Component System) und Physik-Integration.",
        techStack: ["Agent Behavior Trees", "ECS Framework", "Netcode Engine"],
        role: "Spezialisierte Bibliotheken",
      },
      {
        title: "API-Familien (ATC, ShivaCore, AI)",
        description: "Strukturierte RPC- und IPC-Endpunkte für Blockchain (Token, NFT, Staking), OS-Kern (Process, Memory, Device) und AI (Inference, Tools, Memory).",
        techStack: ["Protobuf", "JSON-RPC 2.0", "Cap'n Proto"],
        role: "Standardisierte Nachrichtenverträge",
      },
    ],
    interfaces: [
      { type: "RPC / Transport", protocol: "gRPC / HTTP/2 & WebSocket", description: "Netzwerkweite Service-Kommunikation" },
      { type: "Local IPC", protocol: "Unix Domain Sockets / Shared Memory Ring", description: "Ultra-schnelle lokale Interprozesskommunikation" },
    ],
    securityRole: "Schema-Validierung, Serialisierungs-Sanitization und Signatur-Prüfung aller Payloads.",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                     SDK / API LAYER (L3)                    │
├─────────────────────────────────────────────────────────────┤
│  ATC SDK │ ShivaCore SDK │ Agent SDK │ Game SDK │ Wallet   │
│  REST │ GraphQL │ gRPC │ WebSocket │ JSON-RPC │ IPC API     │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-4-middleware",
    number: 4,
    name: "Middleware Layer",
    shortName: "Middleware",
    headline: "Orchestrierung, Gateway, Richtlinien & Observability",
    color: "purple",
    badgeBg: "bg-purple-500/10",
    badgeBorder: "border-purple-500/30",
    textColor: "text-purple-400",
    components: [
      {
        title: "API Gateway & Router",
        description: "Zentraler Eintrittspunkt mit Rate Limiting, DDoS-Schutz, TLS-Terminierung und Pfad-Routing.",
        techStack: ["Reverse Proxy", "Token Bucket", "Traffic Shaping"],
        role: "Netzwerk-Schutzschild & Lastverteilung",
      },
      {
        title: "Service Mesh & Service Discovery",
        description: "Dynamische Erkennung aktiver Microservices, Health-Checks, Retries und Circuit Breaker.",
        techStack: ["mTLS Sidecars", "Consul/DNS Registry", "Telemetry Hooks"],
        role: "Robuste interne Service-Kommunikation",
      },
      {
        title: "Identity, Session & Policy Engine",
        description: "Verwaltung kryptografischer Identitäten (DID), Session-Tokens und RBAC/ABAC Richtlinien.",
        techStack: ["OPA (Open Policy Agent)", "JWT / CapTokens", "Session Store"],
        role: "Sicherheits- und Zugriffskontrolle",
      },
      {
        title: "Event Bus & Message Queue",
        description: "Asynchrones Pub/Sub Messaging für entkoppelte Event-getriebene Systemreaktionen.",
        techStack: ["Kafka-Style Log", "ZeroMQ / NATS Ring", "Persistent Spooling"],
        role: "Event-Verteilung & Queuing",
      },
      {
        title: "Observability & Audit Pipeline",
        description: "Zentrales Tracing (OpenTelemetry), Metriken, Syslog und revisionssichere Audit-Protokollierung.",
        techStack: ["Prometheus Exporter", "Distributed Tracing", "Append-Only Audit Log"],
        role: "System-Transparenz & Forensik",
      },
    ],
    interfaces: [
      { type: "Inter-Service", protocol: "mTLS / gRPC Mesh", description: "Verschlüsselte Zero-Trust Kommunikation" },
      { type: "Event Stream", protocol: "Persistent Event Log", description: "Replayable Event Sourcing" },
    ],
    securityRole: "Zero-Trust Mesh, Richtliniendurchsetzung vor jedem Service-Aufruf.",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                      MIDDLEWARE (L4)                        │
├─────────────────────────────────────────────────────────────┤
│  API Gateway │ Service Mesh │ Auth/Session │ Event Bus      │
│  Message Queue │ Cache │ Workflow │ Rate Limiting │ Policy │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-5-service",
    number: 5,
    name: "Service Layer",
    shortName: "Services / Backend",
    headline: "Hybrid-Services: Zentral, Dezentral, On-Chain & Lokal",
    color: "amber",
    badgeBg: "bg-amber-500/10",
    badgeBorder: "border-amber-500/30",
    textColor: "text-amber-400",
    components: [
      {
        title: "Blockchain & Mining Service",
        description: "Zustandsvalidierung, Blockproduktion, Transaktions-Pool (Mempool) und Konsens-Treiber.",
        techStack: ["PoW/PoS Hybrid", "Merkle Mountain Ranges", "Consensus Engine"],
        role: "Verteilter Vertrauensanker",
      },
      {
        title: "Wallet, Identity & NFT Service",
        description: "Verwaltung von Adressbüchern, Multi-Sig Schlüsseln, Token-Bilanzen und Non-Fungible Tokens.",
        techStack: ["BIP-32/39/44 HD Wallets", "ERC/ATC Token Standards"],
        role: "Finanz- und Vermögensverwaltung",
      },
      {
        title: "AI Inference & Oracle Service",
        description: "Ausführung lokaler/verteilter KI-Modelle, RAG-Pipelines und realer Datenfeed-Verifikation.",
        techStack: ["ONNX Runtime", "llama.cpp", "Signed Data Feeds"],
        role: "Kognitive Dienste & Orakel",
      },
      {
        title: "Storage & Notification Service",
        description: "Objektspeicher, dezentrale Pinning-Dienste und Push-Benachrichtigungsinfrastruktur.",
        techStack: ["Content-Addressed Storage", "WebSocket Push", "WebPush"],
        role: "Persistenz- und Signalverteilung",
      },
    ],
    interfaces: [
      { type: "Runtime Bridge", protocol: "Host ABI / FFI Gateway", description: "Brücke zur ATC VM und WASM-Laufzeit" },
      { type: "P2P Network", protocol: "Libp2p / Custom Gossip", description: "Verbindung zum A-TownChain P2P-Schwarm" },
    ],
    securityRole: "Kryptografische Verifikation aller Aktionen gegen den globalen Konsenszustand.",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                    SERVICE LAYER (L5)                       │
├─────────────────────────────────────────────────────────────┤
│  Wallet Service │ Identity │ Marketplace │ NFT │ Mining     │
│  AI Service │ Game Service │ Storage │ Blockchain │ Oracle │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-6-runtime",
    number: 6,
    name: "Runtime Layer",
    shortName: "Runtimes / ATC VM",
    headline: "Mehrkern-Virtual-Machines & Isolierte Ausführungsumgebungen",
    color: "emerald",
    badgeBg: "bg-emerald-500/10",
    badgeBorder: "border-emerald-500/30",
    textColor: "text-emerald-400",
    components: [
      {
        title: "ATC VM (A-TownChain Virtual Machine)",
        description: "Deterministische Bytecode-Virtual Machine mit Gas-Metering, Formel-Verifikation und Stack-Isolation.",
        techStack: ["Custom Bytecode Engine", "Gas Metering", "Static Verifier", "Stack Frame Protection"],
        role: "Smarte Vertrags- & Logikausführung",
      },
      {
        title: "ATCLang Runtime & WASM Engine",
        description: "Native Sprachlaufzeit für Lumino/ATCLang sowie WebAssembly (Wasmtime/Wasmer) für portable Module.",
        techStack: ["JIT / AOT Compiler", "Wasmtime Core", "Memory Bounds Checker"],
        role: "Universelle Codeausführung",
      },
      {
        title: "Agent Runtime & Scheduler",
        description: "Laufzeitumgebung für autonome Agenten mit Task-Priorisierung, Event-Triggering und Tool-Permissions.",
        techStack: ["Cooperative Coroutines", "Token Bucket Memory", "Goal Planner"],
        role: "KI-Agenten-Orchestrierung",
      },
      {
        title: "Container & Plugin Runtime",
        description: "Leichtgewichtige Micro-Container mit Namespaces und cgroups-ähnlicher Ressourcen-Drosselung.",
        techStack: ["seccomp-bpf", "Capability Bounds", "OCI Compatible"],
        role: "Erweiterungs- und Plugin-Isolation",
      },
    ],
    interfaces: [
      { type: "Syscall ABI", protocol: "ShivaCore SC_* Traps", description: "Kontrollierter Einstieg in den Kernel über MSR/SVC" },
      { type: "Host Interface", protocol: "HostCall Table", description: "Schnittstelle für sichere VM-zu-Host Aufrufe" },
    ],
    securityRole: "Strikte Speichergrenzen (Memory Bounds), Gas-Limits gegen Denial-of-Service und Verifier-Garantie vor der Ausführung.",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                     RUNTIME LAYER (L6)                      │
├─────────────────────────────────────────────────────────────┤
│  ATC VM (Bytecode, Verifier, Gas, Memory, Execution)        │
│  ATCLang Runtime │ WASM Runtime │ Agent Runtime │ Container │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-7-syscall",
    number: 7,
    name: "System Call Layer",
    shortName: "Syscall ABI",
    headline: "Kontrollierte Hardwareschutzgrenze zwischen User- und Kernel-Space",
    color: "teal",
    badgeBg: "bg-teal-500/10",
    badgeBorder: "border-teal-500/30",
    textColor: "text-teal-400",
    components: [
      {
        title: "Syscall Dispatcher & Handler Table",
        description: "Zentrale Sprungtabelle (256 Vektoren / MSR_LSTAR), die CPU-Register sichert und Syscalls validiert.",
        techStack: ["x86_64 `syscall`", "ARM64 `svc #0`", "RISC-V `ecall`"],
        role: "Hardware-Trap & Register-Marshalling",
      },
      {
        title: "Prozess-, Thread- & Speicherschnittstellen",
        description: "SC_PROCESS_CREATE/EXIT, SC_THREAD_CREATE, SC_MEMORY_ALLOC/MAP/UNMAP.",
        techStack: ["Virtuelle Adressraum-Isolation", "Stack-Switch zu RSP0"],
        role: "Lebenszyklus- & Adressraum-Steuerung",
      },
      {
        title: "IPC & Handle Gateway",
        description: "SC_IPC_SEND/RECEIVE, SC_HANDLE_CREATE/CLOSE/DUP.",
        techStack: ["Message Queues", "Zero-Copy Shared Pages", "Handle Lookup"],
        role: "Sichere Interprozess-Kommunikation",
      },
      {
        title: "Capability & Audit Syscalls",
        description: "SC_CAP_GRANT, SC_CAP_REVOKE, SC_AUDIT_WRITE.",
        techStack: ["Token Lineage Tracking", "Revisionssichere Ringpuffer"],
        role: "Sicherheitsprimitive & Forensik",
      },
    ],
    interfaces: [
      { type: "CPU Instruction", protocol: "SYSCALL / SYSRET / SVC", description: "Hardware-Privilegwechsel von Ring 3 zu Ring 0" },
      { type: "ABI Calling Convention", protocol: "AMD64 System V / AArch64 ABI", description: "RDI, RSI, RDX, R10, R8, R9 Parameter" },
    ],
    securityRole: "Privileggrenze: Prüfung von Zeigern aus dem Userland (Memory Sanitization, Address Range Limits).",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                  SYSTEM CALL LAYER (L7)                     │
├─────────────────────────────────────────────────────────────┤
│  Process │ Memory │ IPC │ Capability │ File │ Network       │
│  Thread │ Device │ Time │ Container │ VM │ Audit Syscalls   │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-8-kernel",
    number: 8,
    name: "Kernel Space",
    shortName: "ShivaCore Kernel",
    headline: "ShivaCore Microkernel Nucleus (Ring 0 / EL1 / S-Mode)",
    color: "red",
    badgeBg: "bg-red-500/10",
    badgeBorder: "border-red-500/30",
    textColor: "text-red-400",
    components: [
      {
        title: "Scheduler & Process Manager",
        description: "Preemptiver Multitasking-Scheduler mit Prioritätsklassen (Real-Time, Fair-Share, Idle) und SMP-Core-Affinität.",
        techStack: ["O(1) / CFS Algorithmus", "Per-CPU Runqueues", "TSS RSP0 Switch"],
        role: "CPU-Zeit-Verteilung & Thread-Wechsel",
      },
      {
        title: "Memory Manager & Paging Core",
        description: "4-Level Paging (PML4, PDPT, PD, PT), Physical Frame Allocator (Buddy/Bitmap) und Higher-Half Kernel Mapping.",
        techStack: ["PML4 Paging", "Buddy Allocator", "Slab/kmalloc Heap"],
        role: "Speicherschutz & Virtuelle Adressräume",
      },
      {
        title: "Capability & Object Manager",
        description: "Zentrales Capability-Verzeichnis: Jedem Handle sind exakte Rechte, Namespace und Lebensdauer zugeordnet.",
        techStack: ["Capability Table", "Rechte-Masken", "Revocation Trees"],
        role: "Kern-Sicherheitsprimitive",
      },
      {
        title: "Virtual File System (VFS) & IPC Stack",
        description: "Abstrakte Dateisystem-Schicht mit Inode-Cache, Dentry-Tree und schnellem synchronem/asynchronem IPC.",
        techStack: ["VFS Inode Interface", "Fast-IPC Endpoints", "Pipe/Mailbox Rings"],
        role: "I/O- und Datenstrom-Routing",
      },
      {
        title: "Network Stack & Crypto Subsystem",
        description: "In-Kernel TCP/IP/UDP Stack, Hardware-Krypto (AES-NI, SHA-NI, Ed25519) und Zero-Copy Paket-Ringe.",
        techStack: ["Zero-Copy Ring Buffers", "Crypto Acceleration", "Packet Filter"],
        role: "Netzwerk- und Signaturtreiber",
      },
    ],
    interfaces: [
      { type: "HAL Interface", protocol: "ShivaCore HAL v1.0", description: "Hardware-unabhängige Aufrufe an Treiber" },
      { type: "Interrupt Handlers", protocol: "IDT / GICv3 Vector Table", description: "Reaktion auf Hardware-Interrupts" },
    ],
    securityRole: "Höchste CPU-Privilegebene (Ring 0). Verwaltet die gesamte Hardware und setzt Speichertrennung durch.",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                    KERNEL SPACE (L8)                        │
├─────────────────────────────────────────────────────────────┤
│  ShivaCore Kernel (Scheduler, Memory Manager, IPC)         │
│  Capability Manager │ Object Manager │ VFS │ Network Stack │
│  Security │ Audit │ Crypto │ Resource Manager               │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-9-hal-drivers",
    number: 9,
    name: "HAL / Drivers Layer",
    shortName: "HAL & Drivers",
    headline: "Hardware Abstraction Layer & Gerätetreiber",
    color: "amber",
    badgeBg: "bg-amber-500/10",
    badgeBorder: "border-amber-500/30",
    textColor: "text-amber-300",
    components: [
      {
        title: "CPU & Cache Management (HAL)",
        description: "Abstraktion für SMP-Booting, TLB-Shootdowns, Cache-Kohärenz, CPUID und Power Management (ACPI/PSCI).",
        techStack: ["APIC/x2APIC", "GICv3", "ACPI MADT/FADT", "MSR Kontrollen"],
        role: "Zentrale Prozessoren-Steuerung",
      },
      {
        title: "Speicher- & Bus-Treiber",
        description: "PCIe Bus Enumeration, NVMe/AHCI Massenspeichertreiber, DMA-Controller und USB xHCI Subsystem.",
        techStack: ["PCI Configuration Space", "NVMe Command Queues", "DMA Buffers"],
        role: "Bus- und Speicherzugriff",
      },
      {
        title: "Netzwerk- & GPU-Treiber",
        description: "Gigabit/10G Ethernet Treiber (Intel e1000, Realtek, VirtIO-Net) und Framebuffer/GOP/KMS Grafiktreiber.",
        techStack: ["VirtIO-Net/GPU", "e1000 Treiber", "VESA/GOP Framebuffer"],
        role: "Datenübertragung & Grafikausgabe",
      },
      {
        title: "Audio & Peripherie-Treiber",
        description: "Intel High Definition Audio (HDA), PS/2 Maus & Tastatur, serielle Schnittstellen (UART COM1-4).",
        techStack: ["Intel HDA", "PS/2 Controller", "16550 UART"],
        role: "Ein-/Ausgabe & Akustik",
      },
    ],
    interfaces: [
      { type: "Bus Protocol", protocol: "PCIe / MMIO / Port I/O", description: "Direkte Register- und Speicherabbildung" },
      { type: "DMA", protocol: "Bus-Mastering DMA", description: "Direkter Speicherzugriff ohne CPU-Belastung" },
    ],
    securityRole: "IOMMU / VT-d Schutz, um unberechtigten Speicherzugriff durch DMA-Geräte zu verhindern.",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                    HAL / DRIVERS (L9)                       │
├─────────────────────────────────────────────────────────────┤
│  CPU │ GPU │ RAM │ PCIe │ USB │ Storage │ Network │ Audio   │
│  Hardware Abstraction Layer (HAL) & Device Drivers          │
└──────────────────────────────┬──────────────────────────────┘`,
  },
  {
    id: "layer-10-hardware",
    number: 10,
    name: "Hardware Platform",
    shortName: "Physical Hardware",
    headline: "Physische Silizium-Architekturen & Bare-Metal-Infrastruktur",
    color: "slate",
    badgeBg: "bg-slate-500/10",
    badgeBorder: "border-slate-500/30",
    textColor: "text-slate-300",
    components: [
      {
        title: "x86_64 / AMD64 Prozessoren",
        description: "Intel Core/Xeon, AMD Ryzen/EPYC, x86_64 Server und PCs mit VT-x/AMD-V Virtualisierung.",
        techStack: ["Long Mode 64-Bit", "PML4/PML5", "APIC", "AVX-512"],
        role: "Primäre High-Performance Zielarchitektur",
      },
      {
        title: "ARM64 / AArch64 Architekturen",
        description: "Apple Silicon, Raspberry Pi 4/5, AWS Graviton, Ampere Altra und moderne Mobilprozessoren.",
        techStack: ["ARMv8/v9", "EL0-EL3", "GICv3", "TrustZone"],
        role: "Effiziente Server- & Edge-Systeme",
      },
      {
        title: "RISC-V (RV64GC) Prozessoren",
        description: "Offene Prozessorarchitektur für souveräne Systeme, SiFive Kerne und FPGA-Emulationen.",
        techStack: ["RV64GC ISA", "Sv39/Sv48 Paging", "PLIC/CLINT"],
        role: "Open-Source & Spezialhardware",
      },
      {
        title: "Embedded SoC & Microcontroller",
        description: "ARM Cortex-M (STM32, NXP) für IoT-Sensoren, HSMs (Hardware Security Modules) und Krypto-Dongles.",
        techStack: ["NVIC", "MPU", "SysTick", "Hardware Crypto"],
        role: "Sicherheitsmodule & IoT",
      },
    ],
    interfaces: [
      { type: "Physical Pins", protocol: "BGA / LGA Sockel, PCIe Lanes", description: "Elektrische Leiterbahnen" },
      { type: "Hardware RoT", protocol: "TPM 2.0 / Secure Boot Key", description: "Hardware-gesicherte Bootkette" },
    ],
    securityRole: "Hardware Root of Trust, Secure Boot Signaturen und physikalischer Speicherschutz.",
    diagram: `┌─────────────────────────────────────────────────────────────┐
│                       HARDWARE (L10)                        │
├─────────────────────────────────────────────────────────────┤
│  x86_64 (Intel/AMD) │ ARM64 (AArch64) │ RISC-V (RV64GC)     │
│  RAM, PCIe Gen5, NVMe, NIC, TPM 2.0, Secure Enclaves        │
└─────────────────────────────────────────────────────────────┘`,
  },
];

export const SHIVACORE_SYSCALLS: SyscallDef[] = [
  {
    code: "SC_PROCESS_CREATE",
    opcode: "0x0001",
    category: "Process",
    signature: "sc_status_t sc_process_create(const char* image_path, uint32_t flags, sc_handle_t* out_proc_handle)",
    registers: "RAX=0x01, RDI=image_path, RSI=flags, RDX=out_proc_handle",
    description: "Erzeugt einen neuen isolierten Adressraum mit eigener PML4-Tabelle und lädt die angegebene Binärdatei.",
    capabilityRequired: "CAP_PROCESS_SPAWN",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_PROCESS_EXIT",
    opcode: "0x0002",
    category: "Process",
    signature: "void sc_process_exit(int32_t exit_code)",
    registers: "RAX=0x02, RDI=exit_code",
    description: "Beendet den aufrufenden Prozess sofort, gibt alle reservierten Speicherframes frei und benachrichtigt Elternprozesse.",
    capabilityRequired: "NONE (Self)",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_THREAD_CREATE",
    opcode: "0x0005",
    category: "Thread",
    signature: "sc_status_t sc_thread_create(sc_handle_t proc, void (*entry)(void*), void* arg, sc_handle_t* out_thread)",
    registers: "RAX=0x05, RDI=proc, RSI=entry, RDX=arg, R10=out_thread",
    description: "Startet einen neuen Kernel-verwalteten Ausführungs-Thread im Adressraum des Zielprozesses.",
    capabilityRequired: "CAP_THREAD_CREATE",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_MEMORY_ALLOC",
    opcode: "0x0010",
    category: "Memory",
    signature: "void* sc_memory_alloc(size_t size, uint32_t prot_flags)",
    registers: "RAX=0x10, RDI=size, RSI=prot_flags",
    description: "Allokiert physische 4KiB/2MiB Speicherseiten und mappt sie in den virtuellen Adressraum des Prozesses.",
    capabilityRequired: "CAP_MEMORY_ALLOC",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_MEMORY_MAP",
    opcode: "0x0011",
    category: "Memory",
    signature: "sc_status_t sc_memory_map(sc_handle_t obj_handle, uint64_t offset, size_t size, uint32_t flags, void** out_addr)",
    registers: "RAX=0x11, RDI=obj_handle, RSI=offset, RDX=size, R10=flags, R8=out_addr",
    description: "Mappt eine Datei, Shared-Memory-Bereich oder MMIO-Hardwareseite in den virtuellen Adressraum.",
    capabilityRequired: "CAP_MEMORY_MAP",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_IPC_SEND",
    opcode: "0x0020",
    category: "IPC",
    signature: "sc_status_t sc_ipc_send(sc_handle_t endpoint, const sc_msg_t* msg, uint32_t timeout_ms)",
    registers: "RAX=0x20, RDI=endpoint, RSI=msg, RDX=timeout_ms",
    description: "Sendet eine synchrone oder asynchrone Nachricht an einen IPC-Endpunkt eines anderen Dienstes.",
    capabilityRequired: "CAP_IPC_SEND",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_IPC_RECEIVE",
    opcode: "0x0021",
    category: "IPC",
    signature: "sc_status_t sc_ipc_receive(sc_handle_t endpoint, sc_msg_t* out_msg, uint32_t timeout_ms)",
    registers: "RAX=0x21, RDI=endpoint, RSI=out_msg, RDX=timeout_ms",
    description: "Wartet auf eingehende IPC-Nachrichten auf dem registrierten Endpunkt des Prozesses.",
    capabilityRequired: "CAP_IPC_RECEIVE",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_HANDLE_CREATE",
    opcode: "0x0030",
    category: "Handle",
    signature: "sc_status_t sc_handle_create(uint32_t obj_type, void* obj_ptr, sc_handle_t* out_handle)",
    registers: "RAX=0x30, RDI=obj_type, RSI=obj_ptr, RDX=out_handle",
    description: "Erstellt ein neues gekapseltes Handle in der prozessinternen Capability-Tabelle.",
    capabilityRequired: "CAP_HANDLE_MANAGE",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_FILE_OPEN",
    opcode: "0x0040",
    category: "File",
    signature: "sc_status_t sc_file_open(const char* path, uint32_t mode, sc_handle_t* out_file)",
    registers: "RAX=0x40, RDI=path, RSI=mode, RDX=out_file",
    description: "Öffnet einen VFS-Dateiknoten nach vorheriger Prüfung der Pfad- und Namensraum-Capabilities.",
    capabilityRequired: "CAP_FS_READ / CAP_FS_WRITE",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_SOCKET_CREATE",
    opcode: "0x0050",
    category: "Network",
    signature: "sc_status_t sc_socket_create(int domain, int type, int protocol, sc_handle_t* out_sock)",
    registers: "RAX=0x50, RDI=domain, RSI=type, RDX=protocol, R10=out_sock",
    description: "Initialisiert ein Netzwerk-Socket (TCP/UDP/QUIC/Raw) im ShivaCore Netzwerk-Stack.",
    capabilityRequired: "CAP_NET_SOCKET",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_CAP_GRANT",
    opcode: "0x0060",
    category: "Capability",
    signature: "sc_status_t sc_cap_grant(sc_handle_t target_proc, sc_handle_t res_handle, uint64_t rights_mask)",
    registers: "RAX=0x60, RDI=target_proc, RSI=res_handle, RDX=rights_mask",
    description: "Überträgt kontrolliert ein Teilrecht an einen Child- oder Partner-Prozess unter Beibehaltung der Lineage.",
    capabilityRequired: "CAP_CAPABILITY_DELEGATE",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_CAP_REVOKE",
    opcode: "0x0061",
    category: "Capability",
    signature: "sc_status_t sc_cap_revoke(sc_handle_t res_handle, uint32_t cascade_flags)",
    registers: "RAX=0x61, RDI=res_handle, RSI=cascade_flags",
    description: "Widerruft augenblicklich alle delegierten Capabilities rekursiv entlang des Delegationsbaums.",
    capabilityRequired: "CAP_CAPABILITY_ADMIN",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_DEVICE_IOCTL",
    opcode: "0x0070",
    category: "Device",
    signature: "sc_status_t sc_device_ioctl(sc_handle_t dev_handle, uint32_t cmd, void* arg)",
    registers: "RAX=0x70, RDI=dev_handle, RSI=cmd, RDX=arg",
    description: "Sendet gerätespezifische Steuerbefehle an Gerätetreiber im HAL-Subsystem.",
    capabilityRequired: "CAP_DEVICE_IO",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_TIME_GET",
    opcode: "0x0080",
    category: "Time",
    signature: "sc_status_t sc_time_get(uint32_t clock_id, uint64_t* out_nanos)",
    registers: "RAX=0x80, RDI=clock_id, RSI=out_nanos",
    description: "Liefert hochpräzise monotone Nanosekunden (TSC/HPET/SysTick) für Benchmarking und Scheduling.",
    capabilityRequired: "NONE",
    ring: "Ring 3 -> 0",
  },
  {
    code: "SC_AUDIT_WRITE",
    opcode: "0x0090",
    category: "Audit",
    signature: "sc_status_t sc_audit_write(uint32_t event_type, const void* payload, size_t len)",
    registers: "RAX=0x90, RDI=event_type, RSI=payload, RDX=len",
    description: "Schreibt ein manipulationssicheres Sicherheitsereignis in das unveränderbare Kernel-Audit-Logbuch.",
    capabilityRequired: "CAP_AUDIT_LOG",
    ring: "Ring 3 -> 0",
  },
];

export const CAPABILITY_VERIFICATION_FLOW: CapabilityStep[] = [
  {
    step: 1,
    name: "1. Handle Validation",
    description: "Der Kernel prüft, ob das vom Userland übergebene Integer-Handle in der lokalen Prozesstabelle existiert und nicht als frei markiert ist.",
    codeSnippet: `sc_cap_entry_t* entry = proc->cap_table.lookup(handle);
if (!entry || entry->is_freed) return SC_ERR_INVALID_HANDLE;`,
    enforcement: "O(1) Array-Index Bounds-Check & Generation-Counter-Verifikation.",
  },
  {
    step: 2,
    name: "2. Namespace Isolation",
    description: "Prüfung, ob die Ressource im aktuellen Namespace (Filesystem-Chroot, IPC-Domain, Network-Sandbox) des Prozesses sichtbar ist.",
    codeSnippet: `if (!namespace_contains(proc->ns_domain, entry->object->ns_id)) {
    return SC_ERR_ACCESS_DENIED;
}`,
    enforcement: "Namensraum-Kapselung gegen Cross-Container-Leakage.",
  },
  {
    step: 3,
    name: "3. Rights Mask Check",
    description: "Bitweiser Abgleich der angeforderten Operation (z.B. WRITE) mit der erlaubten Rechte-Bitmaske des Capabilities.",
    codeSnippet: `if ((entry->rights_mask & requested_rights) != requested_rights) {
    sc_audit_log_violation(proc, handle, requested_rights);
    return SC_ERR_PERMISSION_DENIED;
}`,
    enforcement: "Granulare Flags (READ, WRITE, EXEC, DELEGATE, IOCTL, MAP).",
  },
  {
    step: 4,
    name: "4. Capability Lineage & Delegation",
    description: "Jede Weitergabe wird in einem gerichteten azyklischen Graphen (DAG) protokolliert. Ein Child kann niemals mehr Rechte erhalten als der Parent.",
    codeSnippet: `child_entry->parent_cap_id = parent_entry->cap_id;
child_entry->rights_mask = parent_entry->rights_mask & delegate_mask;`,
    enforcement: "Monotone Rechte-Reduktion (Transitive Restriktion).",
  },
  {
    step: 5,
    name: "5. Revocation & Lifetime Check",
    description: "Überprüfung, ob das Handle abgelaufen ist (TTL) oder die Eltern-Capability inzwischen widerrufen (revoked) wurde.",
    codeSnippet: `if (entry->is_revoked || (entry->expiry > 0 && now() > entry->expiry)) {
    sc_cap_release(entry);
    return SC_ERR_CAPABILITY_EXPIRED;
}`,
    enforcement: "Kaskadierender Widerruf: Alle abgeleiteten Kind-Handles werden ungültig.",
  },
  {
    step: 6,
    name: "6. Audit Log & Ressourcenzugriff",
    description: "Nach erfolgreicher Validierung wird der Zugriff mit Zeiger auf das reale Kernel-Objekt ausgeführt und im Audit-Log dokumentiert.",
    codeSnippet: `sc_audit_record(proc->id, SC_AUDIT_ACCESS_GRANTED, entry->object_id);
return entry->object->vtable->execute(op, args);`,
    enforcement: "Revisionssichere Nachvollziehbarkeit aller Kernel-Zugriffe.",
  },
];

export const VERTICAL_CHAIN_GENERAL: VerticalChainStep[] = [
  { index: 1, layer: "User", action: "Benutzer löst Aktion aus (z.B. Klick 'Starte Berechnung' oder UI-Befehl)", payload: "UI Event (Click, Touch, Keyboard)", privilege: "Physical World" },
  { index: 2, layer: "Frontend", action: "Aurora UI Desktop / React fängt Event ab und ruft TypeScript API-Methode auf", payload: "State Mutation & Dispatch", privilege: "User Space (Sandboxed)" },
  { index: 3, layer: "Application", action: "Globus OS App validiert Geschäftslogik und formt API-Anfrage", payload: "Business Transaction Object", privilege: "User Space (App Process)" },
  { index: 4, layer: "SDK", action: "ShivaCore SDK / ATC SDK serialisiert Parameter und erstellt typisierte Nachricht", payload: "Protobuf / JSON-RPC Packet", privilege: "Client Library" },
  { index: 5, layer: "API Gateway", action: "API Gateway prüft Rate Limits, entschlüsselt Payload und authentifiziert Aufruf", payload: "Validated RPC Envelope", privilege: "Middleware Mesh" },
  { index: 6, layer: "Middleware", action: "Service Mesh leitet weiter, Session Manager verifiziert Auth, Policy Engine prüft OPA", payload: "Security-Enriched Context", privilege: "Trusted Middleware" },
  { index: 7, layer: "Services", action: "Zieldienst (z.B. Compute/AI/Storage) führt Berechnungen durch", payload: "Service Response Payload", privilege: "Backend Service Node" },
  { index: 8, layer: "Runtime", action: "ATC VM / WASM Runtime führt Bytecode aus und triggert Systemaufruf", payload: "Bytecode Opcode Trapped", privilege: "VM Sandbox", boundary: "Privilege Transition: User -> Kernel" },
  { index: 9, layer: "Syscall ABI", action: "Hardware Syscall Trap (`syscall` / `svc #0`) schaltet CPU in Ring 0 / EL1 um", payload: "RAX=0x10, RDI=size, RSI=flags", privilege: "Ring 3 -> Ring 0 Trap" },
  { index: 10, layer: "ShivaCore Kernel", action: "Kernel Scheduler, Memory Manager & Capability Engine führen Operation aus", payload: "PML4 Page Table Updates", privilege: "Ring 0 Nucleus" },
  { index: 11, layer: "HAL", action: "Hardware Abstraction Layer steuert MMIO-Register und Interrupts hardwareunabhängig", payload: "MMIO Read/Write Commands", privilege: "Kernel Privileged" },
  { index: 12, layer: "Drivers", action: "Gerätetreiber kommunizieren über PCIe, DMA und APIC mit dem Controller", payload: "PCIe Packet / DMA Transfer", privilege: "Driver Domain" },
  { index: 13, layer: "Hardware", action: "Silizium (CPU-Cores, DRAM, SSD, NIC) führt physikalische Transistor-Schaltungen durch", payload: "Physical Voltage & Bits", privilege: "Physical Bare-Metal" },
];

export const VERTICAL_CHAIN_BLOCKCHAIN: VerticalChainStep[] = [
  { index: 1, layer: "Frontend", action: "Benutzer signiert Transaktion in der Aurora Wallet UI mit Hardware-Key / Passkey", payload: "Raw Transaction + Ed25519 Sig", privilege: "User UI" },
  { index: 2, layer: "ATC SDK", action: "ATC SDK baut kanonische Transaktionsstruktur (Nonce, GasPrice, Payload, To, Value)", payload: "Packed ATC Transaction", privilege: "Client SDK" },
  { index: 3, layer: "ATC API", action: "Übermittlung per WebSocket / gRPC an den lokalen oder Remote RPC-Knoten", payload: "eth_sendRawTransaction / atc_send", privilege: "Network Transport" },
  { index: 4, layer: "Blockchain Service", action: "Mempool-Service verifiziert Signatur, Balance und fügt TX dem Transaktionspool hinzu", payload: "Mempool Entry (Pending)", privilege: "Node Service" },
  { index: 5, layer: "ATC Runtime", action: "Blockproduzent zieht TX und instanziiert ATC VM Instanz zur Ausführung", payload: "Contract Bytecode + Input Args", privilege: "Execution Worker" },
  { index: 6, layer: "ATC VM", action: "Bytecode-Verifikation, Gas-Abzug pro Opcode und Zustandsmodifikation im State-Tree", payload: "Storage Slot Updates + Events", privilege: "VM Isolated Sandbox", boundary: "Host Syscall Bridge" },
  { index: 7, layer: "Host / Syscall", action: "Host-Call-Interface fordert Persistenz- und Netzwerk-Capabilities von ShivaCore an", payload: "SC_FILE_WRITE / SC_SOCKET_SEND", privilege: "Ring 3 -> 0 Boundary" },
  { index: 8, layer: "ShivaCore Kernel", action: "Kernel garantiert atomare Schreiboperationen (VFS Write) und Kryptoverifikation", payload: "Buffer Cache Flush & Crypto IO", privilege: "Ring 0 Microkernel" },
  { index: 9, layer: "Storage & Crypto", action: "State Root Hash wird in Merkle Tree (Trie) gespeichert; NVMe-Write via HAL-Treiber", payload: "Flash Page Write (NVMe DMA)", privilege: "Kernel Driver" },
  { index: 10, layer: "ATC Node", action: "Lokaler Node bündelt verarbeitete Transaktionen in einen neuen vorgeschlagenen Block", payload: "Proposed Block Header + Body", privilege: "Node Core Engine" },
  { index: 11, layer: "P2P Network", action: "Gossip-Protokoll propagiert den Block über Libp2p an alle Validatoren im Schwarm", payload: "GossipSub Message Broadcast", privilege: "P2P Swarm Network" },
  { index: 12, layer: "A-TownChain Consensus", action: "Validatoren stimmen per Konsensus ab (Finality) und verankern den Block global", payload: "Finalized State Root Hash", privilege: "Global Consensus" },
];

export const TEN_CORE_PLATFORMS: PlatformItem[] = [
  {
    id: 1,
    name: "1. Aurora Frontend Platform",
    leadTech: "React, TypeScript, WebGPU, Rust, Wayland",
    objective: "Universelle grafische Benutzeroberfläche über Desktop, Mobilgeräte, Browser und immersive Experiences.",
    keyDeliverables: [
      "Aurora Desktop Shell mit GPU-beschleunigtem Compositor",
      "Adaptive Mobile UI & Touch Navigation",
      "Web Portal & WebAssembly Admin Console",
      "Native Wayland/GTK Shell für Bare-Metal Installationen",
    ],
    scope: "Layer 1 (Experience)",
  },
  {
    id: 2,
    name: "2. Application Platform",
    leadTech: "Rust, Lumino, ATCLang, C++23",
    objective: "Entwicklung, Paketierung und Sandbox-Ausführung nativer und dezentraler Anwendungen.",
    keyDeliverables: [
      "Globus OS System-Apps (Dateimanager, Monitor, Settings)",
      "Autonome KI-Agenten mit lokaler Vektordatenbank",
      "DeFi, Wallet & DApp Ökosystem",
      "Globus Studio (Entwickler-IDE mit Compiler & Debugger)",
    ],
    scope: "Layer 2 (Applications)",
  },
  {
    id: 3,
    name: "3. ATC / ShivaCore SDK Platform",
    leadTech: "TypeScript, Rust Crate, C FFI, Protobuf",
    objective: "Standardisierte, typensichere Software Development Kits für alle Systemkomponenten.",
    keyDeliverables: [
      "ShivaCore Native System SDK (Kernel Handles, IPC, Events)",
      "ATC Blockchain SDK (Transaktionen, Smart Contracts, Wallets)",
      "Agent SDK für modulare KI-Tools und Verhaltensbäume",
      "Game SDK mit Audio- und WebGPU-Pipelines",
    ],
    scope: "Layer 3 (SDK / APIs)",
  },
  {
    id: 4,
    name: "4. API Platform",
    leadTech: "gRPC, GraphQL, WebSocket, REST, JSON-RPC",
    objective: "Strukturierte, latenzoptimierte Kommunikationsverträge für entfernte und lokale Interaktion.",
    keyDeliverables: [
      "ShivaCore System API (Process, Memory, IPC, Storage, Devices)",
      "ATC Blockchain API (Mining, Staking, NFTs, Governance)",
      "AI Inference & Skill API (Streaming, Prompting, Tools)",
      "High-Speed Local IPC Protokoll via Shared Memory",
    ],
    scope: "Layer 3 (Communication APIs)",
  },
  {
    id: 5,
    name: "5. Middleware Platform",
    leadTech: "API Gateway, Service Mesh, OPA, Event Bus, NATS",
    objective: "Orchestrierung, Lastverteilung, Sicherheit und Richtliniendurchsetzung zwischen Frontend und Backend.",
    keyDeliverables: [
      "API Gateway mit Rate Limiting und TLS/mTLS",
      "Zero-Trust Service Mesh mit Health Checks & Circuit Breaker",
      "Identity & Session Manager mit kryptografischen DIDs",
      "Revisionssichere Observability & Distributed Tracing Pipeline",
    ],
    scope: "Layer 4 (Middleware)",
  },
  {
    id: 6,
    name: "6. Backend / Service Platform",
    leadTech: "Go, Rust, Node.js, Distributed Storage",
    objective: "Hybride Plattformlogik: Kombination aus zentralen Diensten, dezentralen Nodes und On-Chain Verträgen.",
    keyDeliverables: [
      "Zustandsbehaftete Wallet-, Identity- und Governance-Services",
      "Dezentraler Objektspeicher & Pinning-Knoten",
      "AI Inference Server mit GPU-Offloading",
      "Orakeldienste für signierte externe Echtzeit-Feeds",
    ],
    scope: "Layer 5 (Services)",
  },
  {
    id: 7,
    name: "7. Runtime Platform",
    leadTech: "ATC VM, Wasmtime, Agent Scheduler, Container Core",
    objective: "Sichere, deterministische Mehrkern-Ausführungsumgebungen für Bytecode, Smart Contracts und Agenten.",
    keyDeliverables: [
      "ATC VM mit exaktem Gas-Metering und Stack-Isolation",
      "WASM / WASI Runtime für Cross-Platform Plugins",
      "Agent Runtime mit Coroutinen und Permission-Guards",
      "Micro-Container Runtime mit seccomp-bpf Isolation",
    ],
    scope: "Layer 6 (Runtimes)",
  },
  {
    id: 8,
    name: "8. ShivaCore Kernel Platform",
    leadTech: "C (Freestanding), Assembly, Rust Core",
    objective: "Hochperformanter, kompakter Microkernel mit Capability-basierter Sicherheit und Preemptive Scheduling.",
    keyDeliverables: [
      "Preemptiver Scheduler & SMP Multitasking Core",
      "4-Level Paging (PML4) & Virtuelle Speichertrennung",
      "Zentrales Capability System (Rechte-DAG, kaskadierender Widerruf)",
      "Virtuelles Dateisystem (VFS) & Fast-IPC Message Passing",
    ],
    scope: "Layer 7 & 8 (Kernel & Syscalls)",
  },
  {
    id: 9,
    name: "9. HAL / Driver Platform",
    leadTech: "Bare-Metal C, MMIO, Port I/O, DMA, ACPI, GICv3",
    objective: "Hardware Abstraction Layer zur Kapselung von CPU-, Bus- und Peripherietreibern.",
    keyDeliverables: [
      "PCIe Bus Enumeration & DMA Controller Management",
      "Massenspeichertreiber (NVMe, AHCI) & Dateisystemtreiber",
      "Netzwerktreiber (VirtIO-Net, Intel e1000, 10GbE)",
      "Audio (Intel HDA) & Grafiktreiber (GOP/KMS Framebuffer)",
    ],
    scope: "Layer 9 (HAL / Drivers)",
  },
  {
    id: 10,
    name: "10. Hardware / Infrastructure Platform",
    leadTech: "x86_64, ARM64, RISC-V, TPM 2.0, Secure Boot",
    objective: "Unterstützung realer physischer Architekturen vom Server bis zum Embedded-Knoten.",
    keyDeliverables: [
      "x86_64 (AMD64) Bare-Metal Support mit Long Mode & APIC",
      "ARM64 (AArch64) Server- und Edge-Support (EL0-EL3, GICv3)",
      "RISC-V (RV64GC) Open Hardware Support (Sv39 Paging)",
      "Hardware Root-of-Trust via TPM 2.0 & Secure Enclaves",
    ],
    scope: "Layer 10 (Silicon & Infrastructure)",
  },
];

export const SHIVACORE_FULLSTACK_MARKDOWN = `# ShivaCore / Globus OS – Full-Stack-Systemarchitektur

Dieses Dokument definiert die vollständige 10-Schichten-Systemarchitektur für **ShivaCore**, **Globus OS**, **ATC VM** und das **A-TownChain Ökosystem**. Es überwindet die limitierte 2-Tier-Trennung (Frontend / Backend) und beschreibt die vertikale Gesamtkette von der Benutzerinteraktion bis zu den Silizium-Transistoren.

---

## 1. Das 10-Schichten-Architekturmodell

\`\`\`
┌──────────────────────────────────────────────────────────────────┐
│                   1. EXPERIENCE LAYER (UI)                       │
├──────────────────────────────────────────────────────────────────┤
│ Desktop │ Mobile │ Web │ Game │ VR/AR │ CLI │ Admin │ AI UI      │
│ Aurora UI / React / TypeScript / Native Rust UI / Wayland        │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   2. APPLICATION LAYER                           │
├──────────────────────────────────────────────────────────────────┤
│ Apps │ Games │ Wallet │ Marketplace │ Studio │ Explorer          │
│ AI Agents │ Developer Tools │ Governance │ DeFi                  │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   3. SDK / API LAYER                             │
├──────────────────────────────────────────────────────────────────┤
│ ATC SDK │ ShivaCore SDK │ Agent SDK │ Game SDK │ Wallet SDK      │
│ REST │ GraphQL │ gRPC │ WebSocket │ JSON-RPC │ IPC API          │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   4. MIDDLEWARE LAYER                            │
├──────────────────────────────────────────────────────────────────┤
│ API Gateway │ Service Mesh │ Authentication │ Authorization     │
│ Identity / Session │ Event Bus │ Message Queue │ Cache          │
│ Workflow Engine │ Rate Limiting │ Observability │ Policy Engine  │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   5. SERVICE LAYER                               │
├──────────────────────────────────────────────────────────────────┤
│ Wallet Service       │ Identity Service                          │
│ Marketplace Service  │ NFT Service                               │
│ Mining Service       │ Governance Service                        │
│ AI Service           │ Game Service                              │
│ Storage Service      │ Notification Service                      │
│ Blockchain Service   │ Oracle Service                            │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   6. RUNTIME LAYER                               │
├──────────────────────────────────────────────────────────────────┤
│ ATC VM (Bytecode, Verifier, Gas, Memory, Execution)              │
│ ATCLang Runtime │ Agent Runtime │ WASM Runtime                   │
│ Container Runtime │ Plugin Runtime │ Scheduler                   │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   7. SYSTEM CALL LAYER                           │
├──────────────────────────────────────────────────────────────────┤
│ Process Syscalls │ Memory Syscalls │ IPC Syscalls                │
│ File Syscalls    │ Network Syscalls │ Capability Syscalls        │
│ Thread Syscalls  │ Device Syscalls │ Time Syscalls               │
│ Container Syscalls │ VM Syscalls │ Audit Syscalls                │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   8. KERNEL SPACE (Ring 0 / EL1)                 │
├──────────────────────────────────────────────────────────────────┤
│ ShivaCore Kernel                                                 │
│ Scheduler │ Memory Manager │ IPC │ Capability Manager            │
│ Object Manager │ VFS │ Network Stack │ Device Manager            │
│ Security │ Audit │ Crypto │ Resource Manager                     │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   9. HAL / DRIVERS                               │
├──────────────────────────────────────────────────────────────────┤
│ CPU │ GPU │ RAM │ PCIe │ USB │ Storage │ Network │ Audio        │
└───────────────────────────────┬──────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                   10. HARDWARE PLATFORM                          │
├──────────────────────────────────────────────────────────────────┤
│ x86_64 (AMD64) │ ARM64 (AArch64) │ RISC-V (RV64GC) │ SoC / MPU   │
└──────────────────────────────────────────────────────────────────┘
\`\`\`

---

## 2. Die zentrale Security Primitive: Capability-System

In ShivaCore sind Capabilities keine Zusatzfunktion, sondern die **fundamentale Sicherheitsprimitive**.

### Validierungskette:
\`\`\`
Process
   │
   ▼
Capability Table
   │
   ├── Memory
   ├── IPC Endpoint
   ├── File
   ├── Device
   ├── Network
   ├── Process
   └── ATC Resource

Application
    │
    │ READ(handle)
    ▼
Syscall Dispatcher
    │
    ▼
Handle Validation
    │
    ▼
Namespace Check
    │
    ▼
Capability Check (Rights Mask)
    │
    ▼
Capability Lineage & Delegation
    │
    ▼
Revocation & Lifetime Check
    │
    ▼
Policy Check & Audit Record
    │
    ▼
Physical Resource Access
\`\`\`

---

## 3. Die Vertikalen Ausführungsketten

### A. Regulärer System-Workflow:
\`USER → FRONTEND → APPLICATION → SDK → API GATEWAY → MIDDLEWARE → SERVICES → RUNTIME (ATC VM/WASM) → SYSCALL ABI → SHIVACORE KERNEL → HAL → DRIVERS → HARDWARE\`

### B. Blockchain- / Web3-Workflow:
\`Frontend (Wallet) → ATC SDK → ATC API → Blockchain Service → ATC Runtime → ATC VM → Syscall Host Interface → ShivaCore → Network / Storage / Crypto → ATC Node → P2P Gossip Network → A-TownChain Consensus\`

---

## 4. Die 10 Hauptplattformen

1. **Aurora Frontend Platform**: Multi-Surface Human Interface (Desktop, Mobil, Web, Game, XR).
2. **Application Platform**: Native Globus OS Apps, dezentrale Finanzsysteme und autonome KI-Agenten.
3. **ATC/ShivaCore SDK Platform**: Typensichere Entwicklerbibliotheken für OS, VM und Chain.
4. **API Platform**: RPC-, GraphQL-, WebSocket- und Shared-Memory IPC-Endpunkte.
5. **Middleware Platform**: API Gateway, Service Mesh, OPA Policy Engine und Reversible Event Bus.
6. **Backend/Service Platform**: Hybride Services (Zentral, Dezentral, On-Chain und Lokale Knoten).
7. **Runtime Platform**: Deterministische ATC VM, WebAssembly Sandbox und Coroutinen-Scheduler.
8. **ShivaCore Kernel Platform**: Microkernel Nucleus mit Ring 0/EL1 Speichertrennung und Preemptive Scheduler.
9. **HAL/Driver Platform**: Hardware Abstraction Layer für CPU, PCIe, DMA, NVMe und Netzwerk.
10. **Hardware/Infrastructure Platform**: Multi-Architektur Silizium-Support (x86_64, ARM64, RISC-V).
`;
