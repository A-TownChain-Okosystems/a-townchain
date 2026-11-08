// Globus OS / ShivaCore - Reference Architecture Model v2
// 5-Domain Architecture, Horizontal Planes, Specialized Infrastructure Planes,
// IPC Infrastructure, Container Sandbox, Formal Syscall ABI Standard & globus-os/ Repository Tree.

export interface DomainItem {
  id: string;
  name: string;
  badge: string;
  color: string;
  headline: string;
  description: string;
  elements: string[];
  interfaces: string[];
  responsibilities: string[];
}

export interface HorizontalPlaneItem {
  id: string;
  name: string;
  color: string;
  badge: string;
  description: string;
  pillars: {
    title: string;
    description: string;
    features: string[];
  }[];
}

export interface InfraPlaneItem {
  id: string;
  name: string;
  color: string;
  badge: string;
  description: string;
  tree: {
    name: string;
    sub?: string[];
  }[];
  flow: string;
}

export interface IpcPrimitiveItem {
  name: string;
  signature: string;
  parameters: string;
  returnVal: string;
  capabilityRequired: string;
  description: string;
  timing: "Synchronous (Blocking)" | "Asynchronous (Non-blocking)" | "Rendezvous";
}

export interface ContainerFeatureItem {
  dimension: string;
  mechanism: string;
  kernelPrimitive: string;
  enforcement: string;
}

export interface FormalSyscallSpec {
  id: number;
  mnemonic: string;
  abiVersion: string;
  args: string;
  returnType: string;
  capability: string;
  rights: string;
  memoryEffects: string;
  blocking: string;
  errorCodes: string;
  auditEvent: string;
  quota: string;
}

export interface RepoNode {
  name: string;
  type: "dir" | "file";
  desc: string;
  children?: RepoNode[];
}

// 1. THE 5-DOMAIN MODEL
export const GLOBUS_5_DOMAINS: DomainItem[] = [
  {
    id: "experience",
    name: "1. Experience Domain",
    badge: "Human & Machine Surfaces",
    color: "cyan",
    headline: "Multi-Surface UI Shell, Spatial Computing, CLI & AI Interfaces",
    description: "Zentrale Interaktionsschicht für menschliche und maschinelle Benutzer. Trennt Darstellung strikt von Logik über deklarative Datenströme.",
    elements: [
      "Aurora Desktop (Wayland / GPU Compositor)",
      "Mobile Interface (Touch, Gestures, Adaptive UI)",
      "Web Portal (WASM-based Canvas / DOM Shell)",
      "Game Surface (Direct3D/Vulkan low-latency canvas)",
      "VR / AR Spatial Shell (OpenXR integration)",
      "CLI & Shell (POS-Shell, Shiva Terminal, Bash-compat)",
      "Admin Console (Cluster telemetry & node diagnostics)",
      "AI Interface (Natural Language Agent Copilot UI)"
    ],
    interfaces: ["Wayland Compositor IPC", "WebAssembly Rendering Surface", "Shiva Display Server Pipe", "Input Subsystem Event Stream"],
    responsibilities: [
      "Render-Pipeline ohne direkte Ring-0-Zugriffe",
      "Erfassung und Dispatchen von Benutzereingaben (Touch, Tastatur, Maus, Voice)",
      "Adaptive Layoutanpassung je nach Formfaktor",
      "Sichere Isolation von Applikationsfenstern über Sandboxed Compositing"
    ]
  },
  {
    id: "application",
    name: "2. Application Domain",
    badge: "Business Logic & Ecosystem",
    color: "blue",
    headline: "Native Apps, Web3 dApps, Autonome KI-Agenten & Entwickler-Werkzeuge",
    description: "Umfasst alle Anwendungen, die auf der Plattform ausgeführt werden. Anwendungen agieren ausschließlich in Ring 3 mit restriktiven Capability-Tokens.",
    elements: [
      "Native Globus OS Applications (.gapp Pakete)",
      "Gaming Environments & Simulationen",
      "A-TownChain Wallet & Key Custody Vault",
      "DeFi & Decentralized Exchange (DEX)",
      "Decentralized Marketplace & App Store",
      "Globus Studio IDE & Visual Compiler",
      "Block Explorer & State Trie Inspector",
      "Autonome KI-Agenten & Task-Executioner",
      "On-Chain Governance Portal & DAO Voting",
      "Developer Tools, Debugger & Profiler"
    ],
    interfaces: ["ATC SDK Bindings", "ShivaCore Client RPC", "Agent Tool Interface", "Storage & Wallet Protocols"],
    responsibilities: [
      "Ausführung von Geschäfts- und Anwendungslogik",
      "Sichere Kapselung von Benutzer-Assets und privaten Schlüsseln",
      "Kommunikation über typisierte APIs ohne direkte Kernel-Hooks"
    ]
  },
  {
    id: "platform",
    name: "3. Platform Domain",
    badge: "Framework, API, Middleware & Services",
    color: "purple",
    headline: "SDKs, API-Gateways, Zero-Trust Middleware & Hybride Backend-Services",
    description: "Verbindet Applikationen mit den Runtimes. Bündelt Entwickler-Toolkits (SDKs), standardisierte API-Schnittstellen, Event-Buses und System-Services.",
    elements: [
      "SDK / Frameworks: ATC SDK, ShivaCore SDK, Agent SDK, Game SDK, UI SDK, Contract SDK",
      "API Domain: REST, GraphQL, gRPC, WebSocket, JSON-RPC, Shared-Memory IPC, Native C ABI",
      "Control & Middleware: API Gateway, Auth, Identity (DID), Policy (OPA), Service Mesh, Event Bus (NATS), MQ, Tracing",
      "Service Domain: Identity Vault, Token & NFT Service, Mining & Consensus Bridge, Oracle Network, Analytics Engine"
    ],
    interfaces: ["gRPC Multiplexing", "JSON-RPC 2.0 Web3", "Protocol Buffers v3", "ZeroMQ / NATS Event Streams"],
    responsibilities: [
      "Rate Limiting, Routing und TLS Termination",
      "Orchestrierung verteilter Microservices",
      "Bereitstellung standardisierter SDKs in C, Rust, TypeScript und Python",
      "Zero-Trust Durchsetzung vor dem Runtime-Eintritt"
    ]
  },
  {
    id: "execution",
    name: "4. Execution Domain",
    badge: "Runtimes & Sandboxes",
    color: "amber",
    headline: "ATC VM, WebAssembly, Coroutinen-Agenten & Container-Sandboxen",
    description: "Die eigentliche Rechen- und Ausführungsebene. Gewährleistet deterministische Smart-Contract-Zustandsübergänge und isolierte Prozess-Sandboxen.",
    elements: [
      "ATC VM (Deterministischer Bytecode-Interpreter, Gas-Metering, Stack-Isolation)",
      "ATCLang Native Runtime & JIT Engine",
      "WASM Runtime (Wasmtime-basierte isolierte Gast-Instanzen)",
      "Agent Runtime (Autonome Task-Planung & State-Management)",
      "Container Runtime (Lightweight micro-containers mit Namespace-Isolation)",
      "Plugin Runtime (Dynamische Erweiterungen mit restriktiven Rechten)",
      "Execution Scheduler (Kooperatives und preemptives Task-Multiplexing)"
    ],
    interfaces: ["Host ABI (eCall / Trap)", "Wasm Host Functions", "VM Memory Bounds Register", "Syscall Translation Shim"],
    responsibilities: [
      "Deterministische Ausführung von Transaktionen und Contracts",
      "Strikes Gas- und Memory-Metering zur Verhinderung von DoS-Angriffen",
      "Volle Speicherisolation zwischen gleichzeitig aktiven Runtimes",
      "Übergabe von I/O-Anfragen an das Syscall-Gateway"
    ]
  },
  {
    id: "system",
    name: "5. System Domain",
    badge: "Kernel, HAL & Silicon",
    color: "emerald",
    headline: "Syscall ABI, ShivaCore Ring-0-Microkernel, HAL & Hardware",
    description: "Das Herzstück des Betriebssystems. Übernimmt physische Ressourcenverwaltung, Speicherschutz, preemptives Multitasking und Hardwareansteuerung.",
    elements: [
      "Syscall ABI Dispatcher (Fast syscall via MSR_LSTAR / svc #0)",
      "ShivaCore Microkernel Nucleus (Preemptive Scheduler, Memory Paging, Fast IPC)",
      "Capability Object Manager & Security Subsystem",
      "Virtual File System (VFS) & Storage Drivers",
      "Zero-Copy Network Stack & Crypto Acceleration",
      "Hardware Abstraction Layer (HAL für x86_64, ARM64, RISC-V, Cortex-M)",
      "Silicon Hardware (CPU, GPU, RAM, NVMe, PCIe, NIC, TPM 2.0)"
    ],
    interfaces: ["x86_64 Long Mode ABI", "ARM AArch64 AAPCS", "RISC-V SBI", "PCIe Express MMIO", "ACPI / DeviceTree"],
    responsibilities: [
      "Durchsetzung physischer Speichertrennung über 4-Level-Paging (CR3/PML4/TTBR0)",
      "Hardware-Preemption über APIC / GICv3 Timer-Interrupts",
      "Unveränderliche Durchsetzung der Capability-Zugriffsmasken",
      "DMA-Transfer-Koordination über IOMMU für Zero-Trust Gerätetreiber"
    ]
  }
];

// 2. DREI HORIZONTALE QUERSCHNITTS-PLANES
export const HORIZONTAL_PLANES: HorizontalPlaneItem[] = [
  {
    id: "security",
    name: "Security Plane (Zero-Trust Capability Architecture)",
    badge: "End-to-End Across All 5 Domains",
    color: "red",
    description: "Sicherheit wird nicht als nachgelagerter Dienst implementiert, sondern durchzieht als unveränderlicher Kontrollfluss jede Schicht des Systems.",
    pillars: [
      {
        title: "Zero-Trust Pipeline (10 Stufen)",
        description: "Jeder Zugriff auf eine Ressource (Speicher, IPC, Datei, Hardware) folgt einer lückenlosen Validierungssequenz.",
        features: [
          "1. Request: Client initiiert Anforderung mit Handle-Token",
          "2. Identity: Kryptografischer Nachweis der Identität (DID / Signatur)",
          "3. Namespace: Validierung der Prozess-Sandbox & Chroot-Domain",
          "4. Handle Validation: O(1) Index- und Generation-Counter-Check",
          "5. Capability Validation: Kryptografische Integrität & Revocation-Status",
          "6. Rights Check: Bitweiser Abgleich gegen READ/WRITE/EXEC/IOCTL Maske",
          "7. Policy Check: Dynamische OPA/SELinux Regelwerk-Prüfung",
          "8. Quota & Rate Limit: CPU-, RAM- und I/O-Budget-Verifikation",
          "9. Resource Access: Ausführung der Operation im Ring 0 oder isoliertem Treiber",
          "10. Audit Event: Unveränderlicher Eintrag im kryptografischen Ringpuffer"
        ]
      },
      {
        title: "Kryptografische Hardware-Wurzel (Root of Trust)",
        description: "Hardware-gesicherte Schlüsselablage über TPM 2.0, ARM TrustZone und Secure Enclaves.",
        features: [
          "Measured Boot: PCR-Register validieren Kernel-Hash vor Übergabe",
          "Sealed Storage: Dateisystemschlüssel werden an Hardwarezustand gekoppelt",
          "Monotone Hardware-Zähler gegen Replay-Angriffe"
        ]
      },
      {
        title: "Capability Lineage & Rekursive Revocation",
        description: "Rechte können hierarchisch delegiert werden (DAG), wobei Unter-Capabilities strikt monoton abnehmende Rechte besitzen.",
        features: [
          "Monotone Delegation (Kind-Rechte <= Eltern-Rechte)",
          "Instant Revocation (Widerruf des Eltern-Tokens invalidiert alle Nachfahren)",
          "Zeitlich begrenzte Lease-Tokens mit automatischer Expiry"
        ]
      }
    ]
  },
  {
    id: "observability",
    name: "Observability Plane (Telemetry, Tracing & Health)",
    badge: "Systemweite Transparenz",
    color: "emerald",
    description: "Echtzeit-Telemetrie und distributed Tracing vom UI-Klick bis zum NVMe-DMA-Transfer.",
    pillars: [
      {
        title: "Metriken & Ressourcentechnologie",
        description: "Sub-Millisekunden Metrikerfassung ohne Kernel-Locking über atomare Ringspeicher.",
        features: [
          "CPU-Auslastung pro Thread & Task-Wait-Times",
          "Paging-Metriken: Minor/Major Page Faults, TLB-Shootdown-Zähler",
          "IPC-Durchsatz: Nachrichten pro Sekunde, Warteschlangentiefe",
          "Blockchain: Mempool-Transaktionsrate, Gas-Consumption, Block-Propagation"
        ]
      },
      {
        title: "Distributed Tracing & Context Propagation",
        description: "W3C Trace-Context-Propagation über Sprach- und Speichergrenzen hinweg.",
        features: [
          "Trace ID wandert von Aurora UI über SDK, API, ATC VM bis in den Syscall",
          "Syscall-Entry/Exit Latenzmessung über TSC (Time Stamp Counter)",
          "Automatische Erkennung von I/O-Flaschenhälsen"
        ]
      },
      {
        title: "Audit & Compliance Logging",
        description: "Manipulationssicheres Ereignisprotokoll für sicherheitsrelevante Aktionen.",
        features: [
          "Strikte Auditierung aller CAP_GRANT, CAP_REVOKE und PROCESS_CREATE Events",
          "Verkettete Merkle-Tree Hashes für Log-Integrität",
          "Exportmöglichkeit an externe SIEM-Systeme"
        ]
      }
    ]
  },
  {
    id: "governance",
    name: "Governance Plane (Policies, Rules & Economics)",
    badge: "Autonome Systemregeln",
    color: "yellow",
    description: "Regelt dezentrale und lokale Systementscheidungen, Ressourcenquoten, DAO-Abstimmungen und wirtschaftliche Anreize.",
    pillars: [
      {
        title: "On-Chain & System Governance",
        description: "Transparente Regelwerke für Protokoll-Upgrades und Systemparameter.",
        features: [
          "Kryptografische Stimmrechtsabstimmungen (Quadratic Voting / PoS)",
          "Automatisierte Ausführung genehmigter Code-Updates (Hard Fork Automation)",
          "Schatzkammer- und Belohnungsverwaltung für Validatoren und Entwickler"
        ]
      },
      {
        title: "Ressourcen-Ökonomie & Gas-Scheduling",
        description: "Dynamische Preisfindung für Rechenzeit, Speicher und Bandbreite.",
        features: [
          "EIP-1559 ähnliches Basis-Fee + Tip Modell für Systemressourcen",
          "Priorisierung systemkritischer Systemprozesse vor Hintergrund-Mining",
          "Strafmechanismen (Slashing) für fehlerhafte oder bösartige Validatoren"
        ]
      },
      {
        title: "Compliance & Security Policies",
        description: "Deklarative Durchsetzung gesetzlicher und betrieblicher Vorgaben.",
        features: [
          "Mandatory Access Control (MAC) Regeln",
          "DSGVO-konforme kryptografische Datenlöschung (Crypto-Shredding)",
          "Isolation regionaler Rechenlasten auf freigegebene Hardware-Cluster"
        ]
      }
    ]
  }
];

// 3. VIER SPEZIALISIERTE INFRASTRUKTUR-PLANES (AI, BLOCKCHAIN, STORAGE, NETWORK)
export const INFRA_PLANES: InfraPlaneItem[] = [
  {
    id: "ai-plane",
    name: "AI / Agent Platform",
    badge: "Autonome Intelligenz",
    color: "purple",
    description: "KI ist kein simples API-Feature, sondern eine vollwertige Systemebene mit Policy-Guardrails und isoliertem Berechtigungsmanagement.",
    tree: [
      { name: "Model Manager", sub: ["Lokale Quantisierte Modelle (GGUF, ONNX)", "Cloud LLM Gateways (Gemini, Claude)", "Multi-Modal Inference Router"] },
      { name: "Inference Engine", sub: ["NPU / GPU Tensor Core Beschleunigung", "Batching & Dynamic Scheduling", "Zero-Copy Memory Weight Sharing"] },
      { name: "Agent Runtime & Scheduler", sub: ["Autonome Loop Execution", "ReAct / Reflexion Framework", "Goal Decomposition & Priority Queue"] },
      { name: "Skill & Tool System", sub: ["Sandboxed Tool Invocation", "Schema Validation (JSON Schema)", "Hardware Safe Guards"] },
      { name: "Memory Hierarchy", sub: ["Short-Term Context Window", "Long-Term Episodic Storage", "Vector Database (HNSW / Milvus Embeddings)"] },
      { name: "Knowledge & RAG", sub: ["Graph Database (Entities & Relations)", "Hybrid Semantic / BM25 Search", "Real-Time Document Indexer"] },
      { name: "Policy & Approval Engine", sub: ["Human-in-the-Loop Gatekeeper", "Action Impact Estimation", "Strict Capability Token Request"] },
      { name: "Agent Audit Log", sub: ["Reversible Action Log", "Token Consumption Tracker", "Hallucination Risk Scorer"] }
    ],
    flow: "AI Agent → Tool Request → Agent Policy → Capability Request → Syscall ABI → ShivaCore"
  },
  {
    id: "blockchain-plane",
    name: "ATC Blockchain Platform",
    badge: "Dezentraler Konsens",
    color: "cyan",
    description: "Verwaltet den dezentralen State, Konsensfindung, Block-Produktion und Smart-Contract-Ausführung parallel zum OS-Service-Layer.",
    tree: [
      { name: "ATC Core Node", sub: ["Node Lifecycle & State Synchronization", "Fast-Sync / Archive Mode", "Light Client Prover (ZK-SNARK)"] },
      { name: "P2P Network Engine", sub: ["Kademlia DHT Peer Discovery", "GossipSub v1.2 Protocol", "Encrypted Noise Protocol Transports"] },
      { name: "Consensus Engine", sub: ["Proof of Stake (PoS) Finality Gadget", "Proof of Work (PoW) Fallback / Mining", "Hybrid Asynchronous BFT Engine"] },
      { name: "Mempool Subsystem", sub: ["Priority Queue (Gas / Priority Tip)", "Nonce Tracking & Replacement", "MEV-Schutz & Transaction Filtering"] },
      { name: "Transaction Engine", sub: ["Parallel Signature Verification (Ed25519 / Secp256k1)", "Batch State Validation", "Receipt Generator"] },
      { name: "State Machine & Trie DB", sub: ["Modified Merkle Patricia Trie", "RocksDB / LevelDB persistent storage", "State Pruning & Snapshotting"] },
      { name: "Smart Contract Engine", sub: ["ATC VM Bytecode JIT", "State Transition Validator", "Cross-Contract Call Dispatcher"] },
      { name: "Validator & Producer", sub: ["Block Proposal Assembly", "Threshold Signature Aggregation", "Slashing Detector"] },
      { name: "On-Chain Governance", sub: ["Proposal Voting Contract", "Timelock Controller", "Parameter Dynamic Upgrades"] }
    ],
    flow: "Application → ATC SDK → ATC API → Blockchain Service → Transaction Engine → Mempool → Consensus → Block Assembly → State Transition → ATC VM"
  },
  {
    id: "storage-plane",
    name: "Storage Platform",
    badge: "Persistenz & Datenhaltung",
    color: "blue",
    description: "Verwaltet Blockgeräte, Dateisysteme, relationale und verteilte Datenbanken sowie den Merkle-State unabhängig vom Backend.",
    tree: [
      { name: "Block Layer", sub: ["NVMe Driver / Multi-Queue Support", "AHCI / SATA Legacy Layer", "RAM-Disk & Virtual Block Devices"] },
      { name: "Volume Manager", sub: ["Logical Volume Manager (LVM)", "Software RAID 0/1/5/10", "Thin Provisioning & Dynamic Resizing"] },
      { name: "VFS (Virtual Filesystem)", sub: ["Inode & Dentry Caches", "POSIX File Descriptors", "Cross-FS Mount Table"] },
      { name: "Filesystems", sub: ["Ext4 / Btrfs Compatibility", "ShivaFS (Capability-basiertes COW-FS)", "FAT32 / ISO9660 Boot Partitionen"] },
      { name: "Object Storage", sub: ["Content-Addressable Storage (IPFS / SWARM)", "S3-kompatible Schnittstelle", "Deduplication Engine"] },
      { name: "Key-Value & DB", sub: ["Embedded LSM-Tree KV (RocksDB/Speedb)", "Relational SQLite / Postgres Engine", "In-Memory Caching (Redis compat)"] },
      { name: "Blockchain State DB", sub: ["Merkle Patricia Trie Storage", "Historical State Archives", "Light Client State Proofs"] },
      { name: "AI Knowledge Store", sub: ["High-Dimensional Vector Indexes", "Graph Triple Store", "Embedding Persistence"] }
    ],
    flow: "I/O Request → VFS Layer → Capability Check → Filesystem Implementation → Page Cache → Block Layer → NVMe Controller"
  },
  {
    id: "network-plane",
    name: "Network Platform",
    badge: "Konnektivität & Protokolle",
    color: "teal",
    description: "Vollständiger Netzwerkstack von Ring-0-Treibern über Zero-Copy-Sockets bis hin zu P2P-Gossip und verschlüsselten RPCs.",
    tree: [
      { name: "Hardware & L2", sub: ["NIC Device Drivers (Intel e1000, Realtek, VirtIO)", "Ethernet Framing & IEEE 802.1Q VLAN", "Wi-Fi (802.11ax) & Bluetooth Low Energy"] },
      { name: "Network Layer (L3)", sub: ["IPv4 & IPv6 Dual Stack", "ICMP / ICMPv6 Handler", "ARP / NDP Table Management", "Kernel Routing Table & NAT"] },
      { name: "Transport Layer (L4)", sub: ["TCP with BBR Congestion Control", "UDP Low-Latency Datagrams", "QUIC Protocol (Multiplexed UDP)"] },
      { name: "Security & Encryption", sub: ["TLS 1.3 Acceleration (Hardware AES-NI)", "WireGuard VPN Kernel Tunnel", "Noise Protocol Framework for P2P"] },
      { name: "Socket & IPC Layer", sub: ["BSD Socket API Compatibility", "Zero-Copy Ring Buffer Sockets (AF_XDP style)", "Unix Domain Sockets for Local IPC"] },
      { name: "Application Protocols", sub: ["HTTP/1.1, HTTP/2 & HTTP/3 Engine", "WebSocket Full-Duplex Gateway", "gRPC / Protobuf Framing"] },
      { name: "P2P Overlay Network", sub: ["Kademlia DHT Overlay Routing", "A-TownChain Gossip Protocol", "NAT Traversal (STUN, TURN, ICE)"] }
    ],
    flow: "Packet Ingress → NIC Ring Buffer → DMA to RAM → Driver ISR → L2/L3 Unpack → TCP/UDP Stack → Socket Buffer → Userland Application"
  }
];

// 4. IPC ALS ZENTRALE KERNEL-INFRASTRUKTUR
export const IPC_PRIMITIVES: IpcPrimitiveItem[] = [
  {
    name: "SC_IPC_CREATE",
    signature: "sc_status_t sc_ipc_create(sc_ipc_channel_flags_t flags, uint32_t max_msg_size, sc_handle_t* out_endpoint)",
    parameters: "flags (SYNC/ASYNC), max_msg_size (Byte-Limit), out_endpoint (Rückgabe des Endpunkts)",
    returnVal: "SC_SUCCESS, SC_ERR_OUT_OF_MEMORY, SC_ERR_QUOTA_EXCEEDED",
    capabilityRequired: "CAP_IPC_CREATE",
    description: "Erzeugt einen neuen bidirektionalen oder unidirektionalen Kommunikationsendpunkt mit zugewiesenem Ringspeicher.",
    timing: "Synchronous (Blocking)"
  },
  {
    name: "SC_IPC_SEND",
    signature: "sc_status_t sc_ipc_send(sc_handle_t endpoint, const sc_ipc_msg_t* msg, uint32_t timeout_ms)",
    parameters: "endpoint (Gültiges Handle), msg (Payload Pointer & Länge), timeout_ms (0 = Sofortabbruch)",
    returnVal: "SC_SUCCESS, SC_ERR_TIMEOUT, SC_ERR_PEER_DISCONNECTED",
    capabilityRequired: "CAP_IPC_WRITE",
    description: "Überträgt eine Nachricht an den Ziel-Endpunkt. Bei vollem Ringspeicher blockiert der aufrufende Thread bis Timeout.",
    timing: "Asynchronous (Non-blocking)"
  },
  {
    name: "SC_IPC_RECEIVE",
    signature: "sc_status_t sc_ipc_receive(sc_handle_t endpoint, sc_ipc_msg_t* out_msg, uint32_t timeout_ms)",
    parameters: "endpoint (Gültiges Handle), out_msg (Ziel-Puffer), timeout_ms",
    returnVal: "SC_SUCCESS, SC_ERR_TIMEOUT, SC_ERR_BUFFER_TOO_SMALL",
    capabilityRequired: "CAP_IPC_READ",
    description: "Liest die nächste anstehende Nachricht aus der Warteschlange des Endpunkts. Thread schläft bei leerer Queue.",
    timing: "Synchronous (Blocking)"
  },
  {
    name: "SC_IPC_CALL",
    signature: "sc_status_t sc_ipc_call(sc_handle_t endpoint, const sc_ipc_msg_t* req, sc_ipc_msg_t* out_resp, uint32_t timeout_ms)",
    parameters: "endpoint, req (Anfrage), out_resp (Antwortpuffer), timeout_ms",
    returnVal: "SC_SUCCESS, SC_ERR_TIMEOUT, SC_ERR_CALL_REJECTED",
    capabilityRequired: "CAP_IPC_CALL",
    description: "Synchroner RPC-Aufruf: Atomare Kombination aus Send, Rendeszvous-Yield und Blockieren bis zur Antwort (L4-Fastpath).",
    timing: "Rendezvous"
  },
  {
    name: "SC_IPC_REPLY",
    signature: "sc_status_t sc_ipc_reply(sc_handle_t endpoint, uint64_t transaction_id, const sc_ipc_msg_t* resp)",
    parameters: "endpoint, transaction_id (Zugehöriger Call), resp (Antwortdaten)",
    returnVal: "SC_SUCCESS, SC_ERR_TRANSACTION_NOT_FOUND",
    capabilityRequired: "CAP_IPC_WRITE",
    description: "Sendet die Antwort auf einen blockierenden IPC_CALL und weckt den wartenden Client-Thread unmittelbar auf.",
    timing: "Asynchronous (Non-blocking)"
  },
  {
    name: "SC_IPC_NOTIFY",
    signature: "sc_status_t sc_ipc_notify(sc_handle_t endpoint, uint64_t event_mask)",
    parameters: "endpoint, event_mask (64-Bit atomare Bitflags)",
    returnVal: "SC_SUCCESS, SC_ERR_INVALID_HANDLE",
    capabilityRequired: "CAP_IPC_NOTIFY",
    description: "Leichtgewichtige, kopierfreie Benachrichtigung über eine Bitmaske ohne Heap-Allokation für High-Frequency Events.",
    timing: "Asynchronous (Non-blocking)"
  },
  {
    name: "SC_IPC_SIGNAL",
    signature: "sc_status_t sc_ipc_signal(sc_handle_t process_handle, uint32_t signal_id)",
    parameters: "process_handle (Ziel-Prozess), signal_id (SIGKILL, SIGTERM, SIGSTOP, SIGCONT...)",
    returnVal: "SC_SUCCESS, SC_ERR_PERMISSION_DENIED",
    capabilityRequired: "CAP_PROCESS_SIGNAL",
    description: "Zustellung eines asynchronen Kernel-Signals an einen Zielprozess oder eine Thread-Gruppe.",
    timing: "Asynchronous (Non-blocking)"
  },
  {
    name: "SC_IPC_SHARE_MEMORY",
    signature: "sc_status_t sc_ipc_share_memory(sc_handle_t endpoint, void* vaddr, size_t size, uint32_t rights, sc_handle_t* out_shm)",
    parameters: "endpoint, vaddr (Startadresse), size (Page-aligned), rights (READ/WRITE), out_shm",
    returnVal: "SC_SUCCESS, SC_ERR_INVALID_ADDRESS, SC_ERR_SECURITY_VIOLATION",
    capabilityRequired: "CAP_MEM_SHARE",
    description: "Erstellt ein gemeinsames physikalisches Speichermapping zwischen zwei Prozessen für Zero-Copy-Übertragungen.",
    timing: "Synchronous (Blocking)"
  },
  {
    name: "SC_IPC_TRANSFER_CAPABILITY",
    signature: "sc_status_t sc_ipc_transfer_capability(sc_handle_t endpoint, sc_handle_t cap_to_transfer, sc_cap_transfer_mode_t mode)",
    parameters: "endpoint, cap_to_transfer, mode (MOVE oder DELEGATE)",
    returnVal: "SC_SUCCESS, SC_ERR_CAPABILITY_NOT_DELEGATABLE",
    capabilityRequired: "CAP_DELEGATE",
    description: "Verschiebt oder delegiert ein Capability-Handle atomar über einen IPC-Kanal an den Empfängerprozess.",
    timing: "Synchronous (Blocking)"
  },
  {
    name: "SC_IPC_CANCEL",
    signature: "sc_status_t sc_ipc_cancel(sc_handle_t endpoint, uint64_t transaction_id)",
    parameters: "endpoint, transaction_id",
    returnVal: "SC_SUCCESS, SC_ERR_TRANSACTION_NOT_FOUND",
    capabilityRequired: "CAP_IPC_WRITE",
    description: "Bricht eine noch ausstehende IPC_CALL Transaktion vorzeitig ab und gibt reservierte Puffer frei.",
    timing: "Asynchronous (Non-blocking)"
  },
  {
    name: "SC_IPC_CLOSE",
    signature: "sc_status_t sc_ipc_close(sc_handle_t endpoint)",
    parameters: "endpoint (Zu schließendes Handle)",
    returnVal: "SC_SUCCESS, SC_ERR_INVALID_HANDLE",
    capabilityRequired: "None (Besitzerrecht)",
    description: "Schließt den Endpunkt, invalidiert alle ausstehenden Aufrufe mit SC_ERR_PEER_DISCONNECTED und gibt Kernel-Puffer frei.",
    timing: "Synchronous (Blocking)"
  }
];

// 5. CONTAINER ARCHITECTURE & SANDBOXING
export const CONTAINER_FEATURES: ContainerFeatureItem[] = [
  {
    dimension: "PID Namespace",
    mechanism: "Isolierte Prozess-ID-Tabelle (PID 1 als Container-Init)",
    kernelPrimitive: "sc_ns_create(NS_PID)",
    enforcement: "Prozess kann nur Kind-Prozesse innerhalb seines eigenen Baums sehen und signalisieren"
  },
  {
    dimension: "Capability Set",
    mechanism: "Bounded Capability Whitelist (Minimal-Privilege-Prinzip)",
    kernelPrimitive: "sc_cap_restrict_set(mask)",
    enforcement: "Container kann keine Capabilities erlangen, die über das Basis-Template hinausgehen"
  },
  {
    dimension: "Resource Domain (cgroup)",
    mechanism: "Harte CPU-, RAM-, Swap- und I/O-Bandbreitenbegrenzung",
    kernelPrimitive: "sc_quota_assign(proc, limits)",
    enforcement: "OOM-Killer terminiert Container bei Überschreitung des Speicherkontingents ohne Host-Crash"
  },
  {
    dimension: "Virtual Filesystem (VFS)",
    mechanism: "Chroot / Pivot-Root mit Copy-on-Write (COW) ShivaFS Layer",
    kernelPrimitive: "sc_vfs_mount_overlay(base, diff)",
    enforcement: "Schreibzugriffe landen im flüchtigen Container-Overlay; Host-Dateisystem bleibt strikt schreibgeschützt"
  },
  {
    dimension: "Network Namespace",
    mechanism: "Dedizierter Virtual Ethernet (veth) Adapter & Routing-Table",
    kernelPrimitive: "sc_net_ns_create(veth_pair)",
    enforcement: "Container besitzt eigene Loopback- und IP-Adresse; kein Zugriff auf fremde Host-Sockets"
  },
  {
    dimension: "Device Policy",
    mechanism: "Exklusive Whitelist für Gerätedateien (/dev/null, /dev/urandom...)",
    kernelPrimitive: "sc_dev_grant_access(dev_id)",
    enforcement: "Kein Zugriff auf physische Festplatten, Grafikkarten oder MMIO-Register ohne explizite Freigabe"
  },
  {
    dimension: "Security Policy (Seccomp/Syscall Filter)",
    mechanism: "BPF-basierte Syscall-Filterung mit Default-Deny",
    kernelPrimitive: "sc_filter_attach(bytecode)",
    enforcement: "Verbotene Syscalls (z.B. Kernel-Modul-Laden, Reboot) führen zum sofortigen SIGSYS Trap"
  }
];

// 6. FORMALER SYSCALL ABI STANDARD (VOLLSTÄNDIGE SPEZIFIKATION)
export const FORMAL_SYSCALL_SPECS: FormalSyscallSpec[] = [
  {
    id: 1,
    mnemonic: "SC_PROCESS_CREATE",
    abiVersion: "v2.1",
    args: "const sc_proc_config_t* cfg, sc_handle_t* out_proc",
    returnType: "sc_status_t",
    capability: "CAP_PROCESS_SPAWN",
    rights: "EXEC | SPAWN",
    memoryEffects: "Allokiert neuen PML4 Adressraum & Stack",
    blocking: "Ja (Synchron bis Initial-Thread bereit)",
    errorCodes: "SC_ERR_OUT_OF_MEMORY, SC_ERR_PERMISSION_DENIED",
    auditEvent: "AUDIT_EVENT_PROCESS_SPAWN",
    quota: "CPU Time Budget, Process Table Slot"
  },
  {
    id: 2,
    mnemonic: "SC_PROCESS_EXIT",
    abiVersion: "v2.1",
    args: "int32_t exit_code",
    returnType: "void (No return)",
    capability: "None (Eigenprozess)",
    rights: "N/A",
    memoryEffects: "Gibt User-Pages frei, markiert Thread als ZOMBIE",
    blocking: "Terminiert Thread & weckt wartenden Elternprozess",
    errorCodes: "Keine (Gelingt immer)",
    auditEvent: "AUDIT_EVENT_PROCESS_TERMINATE",
    quota: "Gibt Quota-Ressourcen an Parent zurück"
  },
  {
    id: 3,
    mnemonic: "SC_THREAD_CREATE",
    abiVersion: "v2.1",
    args: "void* entry_point, void* arg, sc_handle_t* out_thread",
    returnType: "sc_status_t",
    capability: "CAP_THREAD_CREATE",
    rights: "SPAWN",
    memoryEffects: "Allokiert Thread-Kernel-Stack (16KB) & IST Slot",
    blocking: "Nein (Thread wird in Ready-Queue eingereiht)",
    errorCodes: "SC_ERR_OUT_OF_MEMORY, SC_ERR_MAX_THREADS_REACHED",
    auditEvent: "AUDIT_EVENT_THREAD_CREATE",
    quota: "Thread Count Quota"
  },
  {
    id: 4,
    mnemonic: "SC_THREAD_EXIT",
    abiVersion: "v2.1",
    args: "int32_t exit_code",
    returnType: "void (No return)",
    capability: "None (Eigenthread)",
    rights: "N/A",
    memoryEffects: "Gibt Kernel-Stack frei",
    blocking: "Terminiert aufrufenden Thread",
    errorCodes: "Keine",
    auditEvent: "AUDIT_EVENT_THREAD_EXIT",
    quota: "Gibt Thread-Slot frei"
  },
  {
    id: 5,
    mnemonic: "SC_MEMORY_ALLOC",
    abiVersion: "v2.1",
    args: "size_t size, uint32_t flags, void** out_ptr",
    returnType: "sc_status_t",
    capability: "CAP_MEM_ALLOC",
    rights: "ALLOC | READ | WRITE",
    memoryEffects: "Allokiert physische Frames, mapped VAs in PML4",
    blocking: "Nein (Kann bei hohem Memory-Druck swappen)",
    errorCodes: "SC_ERR_OUT_OF_MEMORY, SC_ERR_QUOTA_EXCEEDED",
    auditEvent: "AUDIT_EVENT_MEMORY_ALLOC",
    quota: "RAM Page Quota"
  },
  {
    id: 6,
    mnemonic: "SC_MEMORY_FREE",
    abiVersion: "v2.1",
    args: "void* ptr, size_t size",
    returnType: "sc_status_t",
    capability: "CAP_MEM_ALLOC",
    rights: "FREE",
    memoryEffects: "Unmapped VAs, dekrementiert Ref-Count physischer Pages",
    blocking: "Nein",
    errorCodes: "SC_ERR_INVALID_ADDRESS",
    auditEvent: "AUDIT_EVENT_MEMORY_FREE",
    quota: "Erhöht verfügbares RAM-Kontingent"
  },
  {
    id: 7,
    mnemonic: "SC_MEMORY_MAP",
    abiVersion: "v2.1",
    args: "sc_handle_t file_or_shm, size_t offset, size_t len, uint32_t prot, void** out_addr",
    returnType: "sc_status_t",
    capability: "CAP_MEM_MAP",
    rights: "MMAP | (PROT_WRITE ? WRITE : READ)",
    memoryEffects: "Erstellt File-Backed oder Shared-Memory VMA Eintrag",
    blocking: "Nein",
    errorCodes: "SC_ERR_PERMISSION_DENIED, SC_ERR_INVALID_OFFSET",
    auditEvent: "AUDIT_EVENT_MEMORY_MAP",
    quota: "Virtual Address Space Quota"
  },
  {
    id: 8,
    mnemonic: "SC_MEMORY_UNMAP",
    abiVersion: "v2.1",
    args: "void* addr, size_t len",
    returnType: "sc_status_t",
    capability: "None",
    rights: "N/A",
    memoryEffects: "Entfernt VMA Bereich und flusht TLB",
    blocking: "Nein",
    errorCodes: "SC_ERR_INVALID_ADDRESS",
    auditEvent: "AUDIT_EVENT_MEMORY_UNMAP",
    quota: "Gibt VA Space frei"
  },
  {
    id: 9,
    mnemonic: "SC_MEMORY_PROTECT",
    abiVersion: "v2.1",
    args: "void* addr, size_t len, uint32_t new_prot",
    returnType: "sc_status_t",
    capability: "CAP_MEM_PROTECT",
    rights: "MPROTECT",
    memoryEffects: "Ändert R/W/X Bits in den Page Table Entries",
    blocking: "Nein (Erfordert INVLPG TLB-Flush)",
    errorCodes: "SC_ERR_PERMISSION_DENIED, SC_ERR_W_AND_X_FORBIDDEN",
    auditEvent: "AUDIT_EVENT_PAGE_PROTECT",
    quota: "Keine Quota-Änderung"
  },
  {
    id: 10,
    mnemonic: "SC_IPC_CREATE",
    abiVersion: "v2.1",
    args: "sc_ipc_flags_t flags, uint32_t buffer_size, sc_handle_t* out_hdl",
    returnType: "sc_status_t",
    capability: "CAP_IPC_CREATE",
    rights: "CREATE",
    memoryEffects: "Reserviert Ringspeicher im Kernel-Heap",
    blocking: "Nein",
    errorCodes: "SC_ERR_OUT_OF_MEMORY, SC_ERR_MAX_CHANNELS",
    auditEvent: "AUDIT_EVENT_IPC_CREATE",
    quota: "IPC Channel Quota"
  },
  {
    id: 11,
    mnemonic: "SC_IPC_SEND",
    abiVersion: "v2.1",
    args: "sc_handle_t endpoint, const sc_ipc_msg_t* msg, uint32_t timeout_ms",
    returnType: "sc_status_t",
    capability: "CAP_IPC_WRITE",
    rights: "WRITE",
    memoryEffects: "Kopiert User-Daten in Ziel-Ringspeicher",
    blocking: "Ja (Wenn Ziel-Puffer voll)",
    errorCodes: "SC_ERR_TIMEOUT, SC_ERR_PEER_DISCONNECTED",
    auditEvent: "AUDIT_EVENT_IPC_SEND",
    quota: "IPC Bandbreiten-Quota"
  },
  {
    id: 12,
    mnemonic: "SC_IPC_RECEIVE",
    abiVersion: "v2.1",
    args: "sc_handle_t endpoint, sc_ipc_msg_t* out_msg, uint32_t timeout_ms",
    returnType: "sc_status_t",
    capability: "CAP_IPC_READ",
    rights: "READ",
    memoryEffects: "Kopiert Daten aus Ringspeicher in User-Puffer",
    blocking: "Ja (Wenn keine Nachricht ansteht)",
    errorCodes: "SC_ERR_TIMEOUT, SC_ERR_BUFFER_TOO_SMALL",
    auditEvent: "AUDIT_EVENT_IPC_RECEIVE",
    quota: "Keine Quota-Änderung"
  },
  {
    id: 13,
    mnemonic: "SC_IPC_CALL",
    abiVersion: "v2.1",
    args: "sc_handle_t endpoint, const sc_ipc_msg_t* req, sc_ipc_msg_t* out_resp, uint32_t timeout_ms",
    returnType: "sc_status_t",
    capability: "CAP_IPC_CALL",
    rights: "READ | WRITE | CALL",
    memoryEffects: "Zero-Copy Register-Übergabe bei kleinen Payloads",
    blocking: "Ja (Rendezvous bis Peer antwortet)",
    errorCodes: "SC_ERR_TIMEOUT, SC_ERR_CALL_REJECTED",
    auditEvent: "AUDIT_EVENT_IPC_CALL",
    quota: "IPC Bandbreiten-Quota"
  },
  {
    id: 14,
    mnemonic: "SC_IPC_REPLY",
    abiVersion: "v2.1",
    args: "sc_handle_t endpoint, uint64_t transaction_id, const sc_ipc_msg_t* resp",
    returnType: "sc_status_t",
    capability: "CAP_IPC_WRITE",
    rights: "WRITE",
    memoryEffects: "Weckt wartenden Thread im Client-Prozess",
    blocking: "Nein",
    errorCodes: "SC_ERR_TRANSACTION_NOT_FOUND",
    auditEvent: "AUDIT_EVENT_IPC_REPLY",
    quota: "Keine"
  },
  {
    id: 15,
    mnemonic: "SC_HANDLE_CREATE",
    abiVersion: "v2.1",
    args: "uint32_t obj_type, uint32_t obj_id, sc_handle_t* out_handle",
    returnType: "sc_status_t",
    capability: "CAP_HANDLE_MANAGE",
    rights: "MANAGE",
    memoryEffects: "Erstellt Eintrag in Prozess-Handletabelle",
    blocking: "Nein",
    errorCodes: "SC_ERR_TABLE_FULL, SC_ERR_INVALID_OBJECT",
    auditEvent: "AUDIT_EVENT_HANDLE_CREATE",
    quota: "Handle Table Limit"
  },
  {
    id: 16,
    mnemonic: "SC_HANDLE_CLOSE",
    abiVersion: "v2.1",
    args: "sc_handle_t handle",
    returnType: "sc_status_t",
    capability: "None (Besitz reicht)",
    rights: "N/A",
    memoryEffects: "Invalidiert Slot, dekrementiert Ref-Count des Objekts",
    blocking: "Nein",
    errorCodes: "SC_ERR_INVALID_HANDLE",
    auditEvent: "AUDIT_EVENT_HANDLE_CLOSE",
    quota: "Gibt Handle-Slot frei"
  },
  {
    id: 17,
    mnemonic: "SC_HANDLE_DUP",
    abiVersion: "v2.1",
    args: "sc_handle_t src_handle, sc_handle_t* out_new_handle",
    returnType: "sc_status_t",
    capability: "CAP_HANDLE_DUP",
    rights: "DUPLICATE",
    memoryEffects: "Erstellt Kopie mit identischen Rechten",
    blocking: "Nein",
    errorCodes: "SC_ERR_INVALID_HANDLE, SC_ERR_TABLE_FULL",
    auditEvent: "AUDIT_EVENT_HANDLE_DUP",
    quota: "Handle Table Limit"
  },
  {
    id: 18,
    mnemonic: "SC_HANDLE_TRANSFER",
    abiVersion: "v2.1",
    args: "sc_handle_t handle, sc_handle_t target_proc, sc_handle_t* out_target_hdl",
    returnType: "sc_status_t",
    capability: "CAP_HANDLE_TRANSFER",
    rights: "TRANSFER",
    memoryEffects: "Verschiebt Handle atomar zwischen zwei Prozesstabellen",
    blocking: "Nein",
    errorCodes: "SC_ERR_INVALID_HANDLE, SC_ERR_PERMISSION_DENIED",
    auditEvent: "AUDIT_EVENT_HANDLE_TRANSFER",
    quota: "Überträgt Quota-Last"
  },
  {
    id: 19,
    mnemonic: "SC_CAP_GRANT",
    abiVersion: "v2.1",
    args: "sc_handle_t target_proc, const sc_cap_desc_t* desc, sc_handle_t* out_cap_hdl",
    returnType: "sc_status_t",
    capability: "CAP_SECURITY_ADMIN",
    rights: "GRANT",
    memoryEffects: "Erzeugt neuen Eintrag im Capability DAG",
    blocking: "Nein",
    errorCodes: "SC_ERR_PERMISSION_DENIED, SC_ERR_RIGHTS_EXCEEDED",
    auditEvent: "AUDIT_EVENT_CAP_GRANT",
    quota: "Security Quota"
  },
  {
    id: 20,
    mnemonic: "SC_CAP_REVOKE",
    abiVersion: "v2.1",
    args: "sc_handle_t cap_hdl, sc_revoke_scope_t scope",
    returnType: "sc_status_t",
    capability: "CAP_SECURITY_ADMIN",
    rights: "REVOKE",
    memoryEffects: "Setzt SC_CAP_REVOKED Flag rekursiv im DAG",
    blocking: "Ja (Synchronisiert alle Kerne via IPI)",
    errorCodes: "SC_ERR_INVALID_HANDLE",
    auditEvent: "AUDIT_EVENT_CAP_REVOKE",
    quota: "Gibt Capability-Slots frei"
  },
  {
    id: 21,
    mnemonic: "SC_CAP_QUERY",
    abiVersion: "v2.1",
    args: "sc_handle_t cap_hdl, sc_cap_info_t* out_info",
    returnType: "sc_status_t",
    capability: "None (Selbstabfrage)",
    rights: "QUERY",
    memoryEffects: "Kopiert Masken und Expiry-Daten in User-Puffer",
    blocking: "Nein",
    errorCodes: "SC_ERR_INVALID_HANDLE",
    auditEvent: "Keiner (High Frequency Read)",
    quota: "Keine"
  },
  {
    id: 22,
    mnemonic: "SC_FILE_OPEN",
    abiVersion: "v2.1",
    args: "const char* path, uint32_t flags, uint32_t mode, sc_handle_t* out_hdl",
    returnType: "sc_status_t",
    capability: "CAP_FS_ACCESS",
    rights: "(flags & O_WRONLY) ? WRITE : READ",
    memoryEffects: "Erstellt File Description & Dentry Ref",
    blocking: "Ja (I/O Zugriff auf Blockgerät)",
    errorCodes: "SC_ERR_NOT_FOUND, SC_ERR_PERMISSION_DENIED",
    auditEvent: "AUDIT_EVENT_FILE_OPEN",
    quota: "Open Files Quota"
  },
  {
    id: 23,
    mnemonic: "SC_FILE_CLOSE",
    abiVersion: "v2.1",
    args: "sc_handle_t file_hdl",
    returnType: "sc_status_t",
    capability: "None",
    rights: "N/A",
    memoryEffects: "Flusht Dirty Pages aus Page-Cache",
    blocking: "Ja (Bei synchronem Flush)",
    errorCodes: "SC_ERR_INVALID_HANDLE",
    auditEvent: "AUDIT_EVENT_FILE_CLOSE",
    quota: "Gibt Open Files Slot frei"
  },
  {
    id: 24,
    mnemonic: "SC_FILE_READ",
    abiVersion: "v2.1",
    args: "sc_handle_t file_hdl, void* buf, size_t count, size_t* out_read",
    returnType: "sc_status_t",
    capability: "CAP_FS_READ",
    rights: "READ",
    memoryEffects: "Kopiert Page-Cache Frames oder triggert NVMe DMA",
    blocking: "Ja",
    errorCodes: "SC_ERR_PERMISSION_DENIED, SC_ERR_IO_FAILURE",
    auditEvent: "Keiner (Performance)",
    quota: "I/O Read Bandwidth Quota"
  },
  {
    id: 25,
    mnemonic: "SC_FILE_WRITE",
    abiVersion: "v2.1",
    args: "sc_handle_t file_hdl, const void* buf, size_t count, size_t* out_written",
    returnType: "sc_status_t",
    capability: "CAP_FS_WRITE",
    rights: "WRITE",
    memoryEffects: "Markiert Page-Cache Pages als DIRTY",
    blocking: "Nein (Asynchron) / Ja (O_SYNC)",
    errorCodes: "SC_ERR_PERMISSION_DENIED, SC_ERR_DISK_FULL",
    auditEvent: "Keiner (Performance)",
    quota: "Disk Space & Write Bandwidth Quota"
  },
  {
    id: 26,
    mnemonic: "SC_SOCKET_CREATE",
    abiVersion: "v2.1",
    args: "uint32_t domain, uint32_t type, uint32_t protocol, sc_handle_t* out_sock",
    returnType: "sc_status_t",
    capability: "CAP_NET_SOCKET",
    rights: "CREATE",
    memoryEffects: "Allokiert Socket-Kontrollblock & RX/TX Ringpuffer",
    blocking: "Nein",
    errorCodes: "SC_ERR_OUT_OF_MEMORY, SC_ERR_PERMISSION_DENIED",
    auditEvent: "AUDIT_EVENT_NET_SOCKET_CREATE",
    quota: "Socket Count Quota"
  },
  {
    id: 27,
    mnemonic: "SC_SOCKET_CONNECT",
    abiVersion: "v2.1",
    args: "sc_handle_t sock, const sc_sockaddr_t* addr, uint32_t timeout_ms",
    returnType: "sc_status_t",
    capability: "CAP_NET_CONNECT",
    rights: "CONNECT",
    memoryEffects: "Initiiert TCP 3-Way Handshake",
    blocking: "Ja (Bis SYN-ACK oder Timeout)",
    errorCodes: "SC_ERR_CONNECTION_REFUSED, SC_ERR_TIMEOUT",
    auditEvent: "AUDIT_EVENT_NET_CONNECT",
    quota: "Egress Bandwidth Quota"
  },
  {
    id: 28,
    mnemonic: "SC_SOCKET_SEND",
    abiVersion: "v2.1",
    args: "sc_handle_t sock, const void* buf, size_t len, uint32_t flags, size_t* out_sent",
    returnType: "sc_status_t",
    capability: "CAP_NET_WRITE",
    rights: "WRITE",
    memoryEffects: "Enqueued Pakete in NIC Transmit Queue via DMA",
    blocking: "Ja (Wenn TX-Puffer voll)",
    errorCodes: "SC_ERR_SOCKET_CLOSED, SC_ERR_TIMEOUT",
    auditEvent: "Keiner",
    quota: "Egress Bandbreite"
  },
  {
    id: 29,
    mnemonic: "SC_SOCKET_RECEIVE",
    abiVersion: "v2.1",
    args: "sc_handle_t sock, void* buf, size_t len, uint32_t flags, size_t* out_received",
    returnType: "sc_status_t",
    capability: "CAP_NET_READ",
    rights: "READ",
    memoryEffects: "Liest aus Socket Receive Queue",
    blocking: "Ja (Wenn keine Daten vorhanden)",
    errorCodes: "SC_ERR_SOCKET_CLOSED, SC_ERR_TIMEOUT",
    auditEvent: "Keiner",
    quota: "Ingress Bandbreite"
  },
  {
    id: 30,
    mnemonic: "SC_DEVICE_OPEN",
    abiVersion: "v2.1",
    args: "const char* dev_path, uint32_t flags, sc_handle_t* out_dev",
    returnType: "sc_status_t",
    capability: "CAP_DEVICE_ACCESS",
    rights: "DEVICE_ACCESS",
    memoryEffects: "Verbindet Device-Treiber-Dispatch-Tabelle",
    blocking: "Nein",
    errorCodes: "SC_ERR_NOT_FOUND, SC_ERR_PERMISSION_DENIED",
    auditEvent: "AUDIT_EVENT_DEVICE_OPEN",
    quota: "Device Handle Quota"
  },
  {
    id: 31,
    mnemonic: "SC_DEVICE_IOCTL",
    abiVersion: "v2.1",
    args: "sc_handle_t dev, uint32_t request, void* arg",
    returnType: "sc_status_t",
    capability: "CAP_DEVICE_IOCTL",
    rights: "IOCTL",
    memoryEffects: "Triggert gerätespezifische Treiber-Routine",
    blocking: "Je nach Treiber (Synchron / Asynchron)",
    errorCodes: "SC_ERR_INVALID_REQUEST, SC_ERR_DRIVER_ERROR",
    auditEvent: "AUDIT_EVENT_DEVICE_IOCTL",
    quota: "Keine"
  },
  {
    id: 32,
    mnemonic: "SC_CONTAINER_CREATE",
    abiVersion: "v2.1",
    args: "const sc_container_spec_t* spec, sc_handle_t* out_container",
    returnType: "sc_status_t",
    capability: "CAP_CONTAINER_ADMIN",
    rights: "CONTAINER_CREATE",
    memoryEffects: "Allokiert isolierte Namespaces (PID, VFS, NET, IPC)",
    blocking: "Ja (Setup der Sandbox)",
    errorCodes: "SC_ERR_OUT_OF_MEMORY, SC_ERR_PERMISSION_DENIED",
    auditEvent: "AUDIT_EVENT_CONTAINER_CREATE",
    quota: "Container Slot Quota"
  },
  {
    id: 33,
    mnemonic: "SC_CONTAINER_START",
    abiVersion: "v2.1",
    args: "sc_handle_t container_hdl",
    returnType: "sc_status_t",
    capability: "CAP_CONTAINER_ADMIN",
    rights: "CONTAINER_EXEC",
    memoryEffects: "Startet Root-Prozess (PID 1) in der Sandbox",
    blocking: "Nein",
    errorCodes: "SC_ERR_INVALID_HANDLE, SC_ERR_ALREADY_RUNNING",
    auditEvent: "AUDIT_EVENT_CONTAINER_START",
    quota: "Aktiviert Quota Limits"
  },
  {
    id: 34,
    mnemonic: "SC_CONTAINER_STOP",
    abiVersion: "v2.1",
    args: "sc_handle_t container_hdl, uint32_t timeout_ms",
    returnType: "sc_status_t",
    capability: "CAP_CONTAINER_ADMIN",
    rights: "CONTAINER_KILL",
    memoryEffects: "Sendet SIGTERM/SIGKILL an gesamten Prozessbaum",
    blocking: "Ja (Wartet bis alle Prozesse beendet sind)",
    errorCodes: "SC_ERR_INVALID_HANDLE",
    auditEvent: "AUDIT_EVENT_CONTAINER_STOP",
    quota: "Gibt Quotas frei"
  },
  {
    id: 35,
    mnemonic: "SC_VM_CREATE",
    abiVersion: "v2.1",
    args: "const sc_vm_config_t* cfg, sc_handle_t* out_vm",
    returnType: "sc_status_t",
    capability: "CAP_VM_CREATE",
    rights: "VM_MANAGE",
    memoryEffects: "Allokiert ATC VM Instanz mit isoliertem Gas- und Stack-Register",
    blocking: "Nein",
    errorCodes: "SC_ERR_OUT_OF_MEMORY",
    auditEvent: "AUDIT_EVENT_VM_CREATE",
    quota: "VM Instance Quota"
  },
  {
    id: 36,
    mnemonic: "SC_VM_EXECUTE",
    abiVersion: "v2.1",
    args: "sc_handle_t vm_hdl, const uint8_t* bytecode, size_t len, uint64_t gas_limit, sc_vm_result_t* out_res",
    returnType: "sc_status_t",
    capability: "CAP_VM_EXECUTE",
    rights: "EXECUTE",
    memoryEffects: "Führt deterministischen Smart Contract Bytecode aus",
    blocking: "Ja (Bis Bytecode beendet oder Out of Gas)",
    errorCodes: "SC_ERR_OUT_OF_GAS, SC_ERR_VM_HALT, SC_ERR_STACK_OVERFLOW",
    auditEvent: "AUDIT_EVENT_VM_EXECUTE",
    quota: "Verbraucht Gas Budget"
  },
  {
    id: 37,
    mnemonic: "SC_TIME_GET",
    abiVersion: "v2.1",
    args: "sc_clock_id_t clock_id, sc_timestamp_t* out_ts",
    returnType: "sc_status_t",
    capability: "None (Unbeschränkt)",
    rights: "READ",
    memoryEffects: "Liest Hardware TSC oder Monotone Uhr",
    blocking: "Nein (Zero-overhead via vDSO Page)",
    errorCodes: "SC_ERR_INVALID_CLOCK",
    auditEvent: "Keiner",
    quota: "Keine"
  },
  {
    id: 38,
    mnemonic: "SC_RANDOM_GET",
    abiVersion: "v2.1",
    args: "void* out_buf, size_t len",
    returnType: "sc_status_t",
    capability: "None",
    rights: "READ",
    memoryEffects: "Füllt Puffer mit kryptografischem Entropie-Stream (RDRAND/RDSEED)",
    blocking: "Nein",
    errorCodes: "SC_ERR_ENTROPY_DEPLETED",
    auditEvent: "Keiner",
    quota: "Rate Limit gegen Denial-of-Entropy"
  },
  {
    id: 39,
    mnemonic: "SC_AUDIT_WRITE",
    abiVersion: "v2.1",
    args: "uint32_t event_type, const void* payload, size_t len",
    returnType: "sc_status_t",
    capability: "CAP_AUDIT_WRITE",
    rights: "APPEND",
    memoryEffects: "Schreibt manipulationssicheren Hash in den Audit-Ringpuffer",
    blocking: "Nein (Lockless Ring Buffer)",
    errorCodes: "SC_ERR_PERMISSION_DENIED, SC_ERR_BUFFER_FULL",
    auditEvent: "AUDIT_EVENT_MANUAL_LOG",
    quota: "Audit Log Bandbreite"
  }
];

// 7. REPOSITORY-STRUKTUR globus-os/
export const GLOBUS_OS_REPO_TREE: RepoNode = {
  name: "globus-os/",
  type: "dir",
  desc: "Root des vollständigen Globus OS & ShivaCore Monorepositories",
  children: [
    {
      name: "kernel/",
      type: "dir",
      desc: "Ring 0 Microkernel Nucleus & Kernkomponenten",
      children: [
        { name: "shivacore/", type: "dir", desc: "Core Nucleus, Task Scheduler, IRQ Dispatcher, Lockless Primitives" },
        { name: "entry.S", type: "file", desc: "Hardware Entrypoint & Bootstrapping" },
        { name: "main.c", type: "file", desc: "Kernel Init, Subsystem Initialization & Core Handshake" }
      ]
    },
    {
      name: "hal/",
      type: "dir",
      desc: "Hardware Abstraction Layer",
      children: [
        { name: "x86_64/", type: "dir", desc: "AMD64 Long Mode, PML4 Paging, APIC/x2APIC, MSR Syscalls" },
        { name: "arm64/", type: "dir", desc: "AArch64 EL0-EL3, VBAR_EL1, TTBR0/1, GICv3" },
        { name: "riscv/", type: "dir", desc: "RV64GC Sv39 Paging, CSR Register, PLIC/CLINT" },
        { name: "cortex_m/", type: "dir", desc: "ARM Cortex-M NVIC, MPU, SysTick RTOS" }
      ]
    },
    {
      name: "drivers/",
      type: "dir",
      desc: "Isolierte Ring-1/Userland Gerätetreiber",
      children: [
        { name: "storage/", type: "dir", desc: "NVMe Multi-Queue, AHCI SATA, RAMDisk" },
        { name: "network/", type: "dir", desc: "Intel e1000/e1000e, Realtek 8139/8169, VirtIO-Net" },
        { name: "gpu/", type: "dir", desc: "KMS/DRM, Framebuffer GOP, VirtIO-GPU" },
        { name: "input/", type: "dir", desc: "PS/2 Keyboard, USB HID Controller" }
      ]
    },
    {
      name: "syscall/",
      type: "dir",
      desc: "Formale Hardware-Software ABI",
      children: [
        { name: "abi/", type: "dir", desc: "ABI Tabellen & Register-Konventionen je Architektur" },
        { name: "dispatcher/", type: "dir", desc: "Fast Syscall Trap Handler & Stack Switcher (RSP0)" },
        { name: "definitions/", type: "dir", desc: "Typisierte Syscall Signaturen, Header & Error Codes" }
      ]
    },
    {
      name: "ipc/",
      type: "dir",
      desc: "Schnelle Inter-Process Communication & Rendezvous Messaging",
      children: [
        { name: "channels/", type: "dir", desc: "Lockless SPSC/MPMC Ring-Buffer Pipes" },
        { name: "shm/", type: "dir", desc: "Shared Memory Page Mapper & TLB Invalidation" },
        { name: "rendezvous/", type: "dir", desc: "L4-Style Fastpath Context Switch (Call/Reply)" }
      ]
    },
    {
      name: "capability/",
      type: "dir",
      desc: "Zero-Trust Capability Engine",
      children: [
        { name: "manager.c", type: "file", desc: "Handle-Tabellen, O(1) Lookup & Generation Check" },
        { name: "lineage.c", type: "file", desc: "DAG Delegation Tracking & Monotone Rechte-Reduktion" },
        { name: "revocation.c", type: "file", desc: "Rekursive Invalidierung & TTL Expiration Engine" }
      ]
    },
    {
      name: "security/",
      type: "dir",
      desc: "Hardware Root of Trust & Policies",
      children: [
        { name: "tpm/", type: "dir", desc: "TPM 2.0 PCR Measured Boot & Key Sealing" },
        { name: "enclave/", type: "dir", desc: "Intel SGX / AMD SEV / ARM TrustZone Bindings" },
        { name: "policy/", type: "dir", desc: "OPA & SELinux-kompatible Regelwerk-Auswertung" }
      ]
    },
    {
      name: "memory/",
      type: "dir",
      desc: "Virtuelles & Physisches Speichermanagement",
      children: [
        { name: "pmm/", type: "dir", desc: "Physical Page Frame Allocator (Buddy Allocator)" },
        { name: "vmm/", type: "dir", desc: "Virtual Memory Manager, 4-Level Page Tables, VMA Tracker" },
        { name: "slab/", type: "dir", desc: "Kernel Object Slab/Slub Heap Caches" }
      ]
    },
    {
      name: "scheduler/",
      type: "dir",
      desc: "Preemptive & Real-Time Task Scheduler",
      children: [
        { name: "cfs.c", type: "file", desc: "Completely Fair Scheduler für Standard-Prozesse" },
        { name: "rt_priority.c", type: "file", desc: "Hard Real-Time Priority Scheduler für Audio/Sensorik" },
        { name: "affinity.c", type: "file", desc: "CPU Core Affinity & NUMA Topology Optimization" }
      ]
    },
    {
      name: "process/",
      type: "dir",
      desc: "Prozess-, Thread- & Namespace-Verwaltung",
      children: [
        { name: "task.c", type: "file", desc: "Process Control Block (PCB) & Thread Control Block (TCB)" },
        { name: "namespace.c", type: "file", desc: "PID, Mount, Net, IPC Sandboxes" },
        { name: "signals.c", type: "file", desc: "Asynchrone Signal-Zustellung & Trap Handler" }
      ]
    },
    {
      name: "filesystem/",
      type: "dir",
      desc: "VFS & Dateisystemtreiber",
      children: [
        { name: "vfs/", type: "dir", desc: "Virtual File System, Inodes, Dentries, File Descriptors" },
        { name: "shivafs/", type: "dir", desc: "Natives Capability-basiertes Copy-on-Write Filesystem" },
        { name: "ext4_compat/", type: "dir", desc: "Kompatibilitätstreiber für Linux Ext4 Partitionen" }
      ]
    },
    {
      name: "networking/",
      type: "dir",
      desc: "Zero-Copy TCP/IP, QUIC & Socket Stack",
      children: [
        { name: "stack/", type: "dir", desc: "IPv4/IPv6, ARP, ICMP, TCP (BBR), UDP, QUIC" },
        { name: "af_xdp/", type: "dir", desc: "Zero-Copy Userland Packet Ring Buffers" },
        { name: "tls/", type: "dir", desc: "In-Kernel TLS 1.3 Acceleration mit AES-NI" }
      ]
    },
    {
      name: "storage/",
      type: "dir",
      desc: "Persistenz, Caches & Merkle Storage",
      children: [
        { name: "block/", type: "dir", desc: "Block Device Management & I/O Scheduler" },
        { name: "merkle/", type: "dir", desc: "Merkle Patricia Trie Persistence für Blockchain-State" },
        { name: "kv/", type: "dir", desc: "Embedded Fast LSM-Tree Key-Value Engine" }
      ]
    },
    {
      name: "runtime/",
      type: "dir",
      desc: "Ausführungsumgebungen & VM Engines",
      children: [
        { name: "atc-vm/", type: "dir", desc: "A-TownChain Deterministischer Bytecode-Interpreter & Gas Meter" },
        { name: "atclang/", type: "dir", desc: "ATCLang Runtime, Garbage Collector & Typensystem" },
        { name: "wasm/", type: "dir", desc: "Wasmtime Sandboxed Bytecode Execution Engine" },
        { name: "agents/", type: "dir", desc: "Coroutinen-basierte autonome Agent-Runtime" },
        { name: "containers/", type: "dir", desc: "Lightweight Micro-Container Manager (ShivaBox)" },
        { name: "plugins/", type: "dir", desc: "Dynamische Erweiterungen mit restriktiven Rechten" }
      ]
    },
    {
      name: "blockchain/",
      type: "dir",
      desc: "A-TownChain Web3 Substrat",
      children: [
        { name: "node/", type: "dir", desc: "Core Node, Synchronizer, State Trie Validator" },
        { name: "consensus/", type: "dir", desc: "PoS Finality Gadget, PoW Mining, Hybrid BFT Engine" },
        { name: "mempool/", type: "dir", desc: "Priorisierte Transaktions-Warteschlange mit MEV-Schutz" },
        { name: "state/", type: "dir", desc: "Merkle Patricia Trie State Machine & Pruning" },
        { name: "p2p/", type: "dir", desc: "Kademlia DHT, GossipSub v1.2, Noise Protocol" },
        { name: "contracts/", type: "dir", desc: "Standard System Smart Contracts (Token, Governance)" }
      ]
    },
    {
      name: "ai/",
      type: "dir",
      desc: "KI-Plattform & Agenten-Infrastruktur",
      children: [
        { name: "inference/", type: "dir", desc: "NPU/GPU Tensor Acceleration & Model Dispatcher" },
        { name: "agents/", type: "dir", desc: "ReAct Task Executor & Planning Loop" },
        { name: "skills/", type: "dir", desc: "Modulare Fähigkeiten & Tool-Definitionen" },
        { name: "tools/", type: "dir", desc: "Sandboxed System-Tools mit Capability-Gate" },
        { name: "memory/", type: "dir", desc: "Vektor-Datenbank (HNSW) & Long-Term Episodic Storage" },
        { name: "knowledge/", type: "dir", desc: "Wissensgraph & Hybride Semantische Suche" }
      ]
    },
    {
      name: "services/",
      type: "dir",
      desc: "Systemdienste & Daemonen",
      children: [
        { name: "identity/", type: "dir", desc: "Decentralized Identifiers (DID) & Verifiable Credentials" },
        { name: "wallet/", type: "dir", desc: "Hardware-gesicherte Schlüsselverwaltung & Signer" },
        { name: "marketplace/", type: "dir", desc: "Dezentraler App & Asset Store Service" },
        { name: "mining/", type: "dir", desc: "Hintergrund-Validator & Proof Generator" },
        { name: "governance/", type: "dir", desc: "DAO Proposal Verifier & Voting Engine" },
        { name: "oracle/", type: "dir", desc: "Kryptografische Off-Chain Datenfeed Schnittstelle" },
        { name: "analytics/", type: "dir", desc: "On-Chain & System Metriken Aggregator" }
      ]
    },
    {
      name: "middleware/",
      type: "dir",
      desc: "Zero-Trust Gateway & Event Bus",
      children: [
        { name: "gateway/", type: "dir", desc: "Reverse Proxy, TLS Offloading, Rate Limiting" },
        { name: "service-mesh/", type: "dir", desc: "mTLS Inter-Service Routing, Circuit Breakers" },
        { name: "event-bus/", type: "dir", desc: "NATS-kompatibles High-Throughput Event Streaming" },
        { name: "queue/", type: "dir", desc: "Persistente Nachrichten-Warteschlangen" },
        { name: "cache/", type: "dir", desc: "Distributed Shared Cache mit Cache-Coherency" },
        { name: "policy/", type: "dir", desc: "Open Policy Agent (OPA) Integration" }
      ]
    },
    {
      name: "sdk/",
      type: "dir",
      desc: "Software Development Kits",
      children: [
        { name: "atc/", type: "dir", desc: "ATC Client SDK (Rust, C, TypeScript, Go)" },
        { name: "shivacore/", type: "dir", desc: "Natives C/Rust ShivaCore System API SDK" },
        { name: "agent/", type: "dir", desc: "KI Agent Framework & Tool Builder SDK" },
        { name: "game/", type: "dir", desc: "Low-Latency Direct Graphics & Input SDK" },
        { name: "wallet/", type: "dir", desc: "Key Custody & Web3 Signer SDK" }
      ]
    },
    {
      name: "api/",
      type: "dir",
      desc: "Kommunikationsverträge & Schnittstellen",
      children: [
        { name: "rest/", type: "dir", desc: "OpenAPI 3.1 REST Definitionen" },
        { name: "graphql/", type: "dir", desc: "GraphQL Schemas für Blockchain & Telemetrie" },
        { name: "grpc/", type: "dir", desc: "Protobuf v3 Definitionen für Microservices" },
        { name: "websocket/", type: "dir", desc: "Echtzeit-Event Streams & Subscriptions" },
        { name: "jsonrpc/", type: "dir", desc: "Ethereum / Web3 JSON-RPC 2.0 Kompatibilität" }
      ]
    },
    {
      name: "applications/",
      type: "dir",
      desc: "Native Globus OS Anwendungen",
      children: [
        { name: "studio/", type: "dir", desc: "Globus Studio IDE & Code Editor" },
        { name: "explorer/", type: "dir", desc: "Dateimanager & Block Explorer" },
        { name: "wallet_app/", type: "dir", desc: "Aurora Desktop Wallet Benutzeroberfläche" },
        { name: "terminal/", type: "dir", desc: "GPU-beschleunigtes Terminal & Shell" }
      ]
    },
    {
      name: "aurora/",
      type: "dir",
      desc: "Aurora Desktop Environment & Compositor",
      children: [
        { name: "compositor/", type: "dir", desc: "Wayland Display Server & Hardware Compositor" },
        { name: "shell/", type: "dir", desc: "Taskbar, Application Launcher, Quick Settings" },
        { name: "theme/", type: "dir", desc: "Design-System, Vektorgrafiken & Shaders" }
      ]
    },
    {
      name: "tools/",
      type: "dir",
      desc: "Entwickler-, Build- & Debugging-Werkzeuge",
      children: [
        { name: "shiva-dbg/", type: "dir", desc: "Kernel-Debugger mit GDB Remote Protocol" },
        { name: "packager/", type: "dir", desc: ".gapp Container Packaging & Signature Tool" },
        { name: "atc-cli/", type: "dir", desc: "Kommandozeilen-Werkzeug für Node & Smart Contracts" }
      ]
    },
    {
      name: "tests/",
      type: "dir",
      desc: "Umfassende Test-Suiten",
      children: [
        { name: "unit/", type: "dir", desc: "Unit-Tests für Kernel, VM und Datenstrukturen" },
        { name: "integration/", type: "dir", desc: "End-to-End Syscall- & IPC-Tests" },
        { name: "fuzzing/", type: "dir", desc: "AFL++ Fuzzing für Syscall Dispatcher & Bytecode Verifier" }
      ]
    },
    {
      name: "docs/",
      type: "dir",
      desc: "Architektur-Dokumentation & Spezifikationen",
      children: [
        { name: "architecture/", type: "dir", desc: "10-Layer Stack & 5-Domain Reference Model" },
        { name: "abi/", type: "dir", desc: "Syscall Spezifikationen & Opcode Tabellen" },
        { name: "security/", type: "dir", desc: "Zero-Trust Capability Whitepaper" }
      ]
    },
    {
      name: "infrastructure/",
      type: "dir",
      desc: "Bereitstellung, CI/CD & Cluster Orchestrierung",
      children: [
        { name: "containers/", type: "dir", desc: "Docker- & Podman-Build-Dateien" },
        { name: "kubernetes/", type: "dir", desc: "Helm Charts für Distributed Testnet Cluster" },
        { name: "terraform/", type: "dir", desc: "Cloud Provider Infrastructure as Code" }
      ]
    }
  ]
};

// 8. VOLLSTÄNDIGE MARKDOWN-SPEZIFIKATION V2 FÜR EXPORT
export const GLOBUS_V2_MARKDOWN_SPEC = `# GLOBUS OS / SHIVACORE – REFERENCE ARCHITECTURE MODEL V2
**Version 2.1 (Final Consolidated Reference Specification)**

---

## 1. DIE 5-DOMAIN COMPUTING ARCHITEKTUR

| Domain | Kennzeichnung | Primäre Verantwortung |
|---|---|---|
| **1. Experience Domain** | Human & Machine Surfaces | UI Shell, Aurora Desktop, Mobile, Game, Spatial, CLI, Admin, AI Assistant |
| **2. Application Domain** | Business Logic & Ecosystem | Native Apps, Web3 dApps, Games, Wallet, Explorer, AI Agents, Governance |
| **3. Platform Domain** | Framework, API & Services | ATC SDK, ShivaCore SDK, REST/gRPC/GraphQL, API Gateway, Service Mesh, Services |
| **4. Execution Domain** | Deterministic Runtimes | ATC VM, ATCLang Runtime, WASM Sandbox, Agent Runtime, Micro-Containers |
| **5. System Domain** | Kernel, HAL & Hardware | Formal Syscall ABI, ShivaCore Ring 0 Microkernel, HAL, Silicon (x86_64, ARM64, RISC-V) |

---

## 2. DREI HORIZONTALE QUERSCHNITTS-PLANES

### A. Security Plane (Zero-Trust Capability Architecture)
Die 10-stufige Validierungssequenz vor jedem Ressourcenzugriff:
\`\`\`
Request → Identity (DID) → Namespace Check → Handle Validation → Capability Validation
        → Rights Check → Policy Check (OPA) → Quota & Limits → Resource Access → Audit Event
\`\`\`

### B. Observability Plane (Metrics, Traces & Telemetry)
* Sub-Millisekunden Metrikerfassung (Lockless Ring Buffers)
* W3C Trace-Context-Propagation vom Aurora UI bis in den Ring-0 Syscall
* Manipulationssicheres Audit-Logging mit Merkle-Chain-Hashes

### C. Governance Plane (Rules, DAO & Economics)
* On-Chain Governance (Quadratic Voting, Auto-Execution von Upgrades)
* Dynamische Gas- und Ressourcenpreisbildung (EIP-1559 Style)
* Compliance & Mandatory Access Control (MAC)

---

## 3. VIER SPEZIALISIERTE INFRASTRUKTUR-PLANES

### 1. AI / Agent Platform
* **Model Manager**: Lokale Quantisierte Modelle & Cloud Gateways
* **Inference Engine**: NPU/GPU Tensor Beschleunigung mit Zero-Copy Memory
* **Agent Runtime**: Autonome ReAct Loop Execution mit Scheduler
* **Tool Gatekeeper**:
\`\`\`
AI Agent → Tool Request → Agent Policy → Capability Request → Syscall ABI → ShivaCore
\`\`\`

### 2. ATC Blockchain Platform
* **A-TownChain Node**: P2P Network (GossipSub v1.2, Kademlia DHT)
* **Consensus**: Hybrid PoS Finality / PoW Mining / BFT Engine
* **Execution Flow**:
\`\`\`
Application → ATC SDK → ATC API → Blockchain Service → Transaction Engine
            → Mempool → Consensus → Block Assembly → State Transition → ATC VM
\`\`\`

### 3. Storage Platform
* **Schichten**: Block Layer (NVMe MQ) → Volume Manager → VFS → Filesysteme (ShivaFS) → Object Storage → KV DB → Merkle State DB

### 4. Network Platform
* **Schichten**: NIC Hardware → Dual Stack IPv4/IPv6 → TCP (BBR)/UDP/QUIC → In-Kernel TLS 1.3 → Zero-Copy AF_XDP Sockets → P2P Gossip

---

## 4. IPC ALS ZENTRALE KERNEL-INFRASTRUKTUR

Kommunikation zwischen Prozessen und Diensten basiert auf synchronem und asynchronem Message-Passing:
\`\`\`
Process A ──[ IPC Message ]──> IPC Endpoint (Capability + Namespace + Rights + Quota + Audit) ──> Process B
\`\`\`

### Die 10 Kern-Primitive:
* \`SC_IPC_CREATE\`: Erzeugt neuen Endpunkt mit Ringspeicher
* \`SC_IPC_SEND\`: Asynchrones oder timeout-gesichertes Senden
* \`SC_IPC_RECEIVE\`: Blockierender oder zeitgesteuerter Empfang
* \`SC_IPC_CALL\`: Synchroner Rendezvous RPC (L4-Style Fastpath)
* \`SC_IPC_REPLY\`: Weckt den wartenden Client auf
* \`SC_IPC_NOTIFY\`: Kopierfreie 64-Bit Event-Maske
* \`SC_IPC_SIGNAL\`: Asynchrone Signalzustellung
* \`SC_IPC_SHARE_MEMORY\`: Zero-Copy Page-Mapping zwischen Prozessen
* \`SC_IPC_TRANSFER_CAPABILITY\`: Atomare Rechteübertragung
* \`SC_IPC_CANCEL\`: Vorzeitiger Abbruch ausstehender Aufrufe
* \`SC_IPC_CLOSE\`: Schließt Endpunkt & invalidiert Ressourcen

---

## 5. CONTAINER ARCHITECTURE (SHIVABOX)

Kein Fremdprodukt, sondern direkte Kernel-Abstraktion:
* **PID Namespace**: Isolierter Prozessbaum mit eigenem PID 1 Init
* **Capability Set**: Bounded Whitelist nach Least-Privilege
* **Resource Domain (cgroup)**: Harte CPU-, RAM- und I/O-Quotas
* **Virtual Filesystem**: Chroot mit Copy-on-Write ShivaFS Overlay
* **Network Namespace**: Virtuelle Ethernet-Paare (veth) mit separatem Routing
* **Device Policy**: Strikte Whitelist für Gerätedateien
* **Syscall Filter**: BPF-basierte Whitelist mit sofortigem SIGSYS Trap

\`\`\`
ATC Container ──> Capability Sandbox ──> ShivaCore Microkernel
\`\`\`

---

## 6. DAS ZIELBILD: DIE VERTIKAL INTEGRIERTE COMPUTING PLATFORM

\`\`\`
                            GLOBUS OS
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
     AURORA                 SERVICES                  AGENTS
  (Desktop, Web,         (Wallet, Identity,       (Autonome Tasks,
   Mobile, Games)           NFT, Mining)            Tools, Memory)
        │                       │                       │
        └───────────────────────┼───────────────────────┘
                                │
                            API / SDK
              (ATC SDK, ShivaCore SDK, REST, gRPC, IPC)
                                │
                            MIDDLEWARE
          (API Gateway, Service Mesh, Event Bus, OPA Policy)
                                │
              ┌─────────────────┴─────────────────┐
              │                                   │
         ATC PLATFORM                        OS SERVICES
      (Consensus, Mempool,                (VFS, Storage,
       State Trie, P2P)                    Network, Audio)
              │                                   │
              └─────────────────┬─────────────────┘
                                │
                            RUNTIMES
              ┌─────────────────┼─────────────────┐
              │                 │                 │
           ATC VM             WASM            AGENT VM
        (Bytecode JIT,     (Sandboxed       (Coroutinen,
         Gas-Metered)       Plugins)        Task Planner)
              │                 │                 │
              └─────────────────┼─────────────────┘
                                │
                           SYSCALL ABI
             (Fast Syscall MSR_LSTAR / ARM svc #0, 39 Calls)
                                │
                         SHIVACORE KERNEL
              ┌─────────────────┼─────────────────┐
              │                 │                 │
           MEMORY              IPC             SECURITY
        (PML4 Paging,     (Lockless Ring,     (Zero-Trust,
         Buddy Alloc)       Rendezvous)       Capabilities)
              │                 │                 │
              └─────────────────┼─────────────────┘
                                │
                           HAL / DRIVERS
                 (CPU, GPU, NVMe, PCIe, NIC, Audio)
                                │
                            HARDWARE
            (x86_64 AMD64, ARM64 AArch64, RISC-V RV64GC)
\`\`\`
`;
