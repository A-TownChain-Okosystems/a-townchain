// Globus OS / ShivaCore - Reference Architecture Model v3: Formal Platform Architecture
// Complete formal interfaces, horizontal control planes, architectural contracts C1-C5,
// system boundaries, unified resource/identity/event models, and the 19-Standard specification family.

export interface DomainV3 {
  id: string;
  code: "D1" | "D2" | "D3" | "D4" | "D5";
  name: string;
  badge: string;
  color: string;
  headline: string;
  description: string;
  components: string[];
  inboundContract: string;
  outboundContract: string;
  boundaryRule: string;
}

export interface ControlPlaneV3 {
  id: string;
  name: string;
  color: string;
  tagline: string;
  coreFunctions: string[];
  nature: "Plattformfunktion (Cross-Cutting)" | "Systeminvariante";
  enforcementMechanism: string;
  auditScope: string;
}

export interface ContractV3 {
  id: string;
  code: "C1" | "C2" | "C3" | "C4" | "C5";
  title: string;
  sourceLayer: string;
  targetLayer: string;
  color: string;
  description: string;
  flowSteps: string[];
  contractClauses: {
    name: string;
    description: string;
    schemaExample?: string;
  }[];
  failureModes: string[];
}

export interface SystemBoundaryV3 {
  id: string;
  title: string;
  subtitle: string;
  color: string;
  internalResponsibilities: string[];
  strictlyExcludedResponsibilities: string[];
  formalPipeline: string[];
  rationale: string;
}

export interface UnifiedModelV3 {
  id: string;
  standardCode: string;
  title: string;
  color: string;
  concept: string;
  schemaFields: {
    field: string;
    type: string;
    description: string;
    example: string;
  }[];
  principalsOrConsumers: string[];
  guarantees: string[];
}

export interface StandardSpecV3 {
  code: string;
  title: string;
  category: "Architecture" | "ABI & Kernel" | "Security & Identity" | "Blockchain & VM" | "Platform API";
  status: "Draft v1.0" | "Standardized" | "Normative";
  description: string;
  primaryArtifact: string;
  referencedBy: string[];
}

// 1. DIE 5 VERTIKALEN DOMAINS (D1 - D5)
export const V3_DOMAINS: DomainV3[] = [
  {
    id: "d1-experience",
    code: "D1",
    name: "Experience Domain",
    badge: "User & Machine Surface",
    color: "cyan",
    headline: "Aurora Desktop, Web, Mobile, Game, VR/AR, CLI & Admin Console",
    description: "Die primäre Interaktionsgrenze. Trennt Rendering und UI-Zustand strikt von Anwendungs- und Kernel-Logik. Kommuniziert ausschließlich über typsichere Schemas und Client-APIs.",
    components: [
      "Aurora Desktop Shell (Wayland Compositor / GPU Canvas)",
      "Web Shell (WASM Canvas & DOM Bridge)",
      "Mobile Adaptive Surface (Touch & Sensor Input)",
      "Game Surface (Direct3D/Vulkan low-latency swapchain)",
      "VR / AR Spatial Shell (OpenXR integration)",
      "CLI / POS-Shell (POSIX-kompatible Kommandostruktur)",
      "Admin & Telemetry Console (Cluster Diagnostics & Node Telemetry)"
    ],
    inboundContract: "Menschliche Eingaben, Sensorströme, Netzwerk-Events",
    outboundContract: "Contract C1 (UI → Application)",
    boundaryRule: "Kein direkter Zugriff auf Kernel-Ressourcen oder Speichermaps; alle Aktionen müssen über autorisierte Application Services laufen."
  },
  {
    id: "d2-application",
    code: "D2",
    name: "Application Domain",
    badge: "Business Logic & Ecosystem",
    color: "blue",
    headline: "Native Apps, Games, Wallet, DeFi, Marketplace & Globus Studio",
    description: "Ausführungsebene für funktionale Benutzerapplikationen und Web3 dApps. Agiert streng im unprivilegierten Ring 3 mit restriktiven Capability-Tokens.",
    components: [
      "Native Apps (.gapp Pakete)",
      "Gaming Environments & Simulationen",
      "A-TownChain Wallet & Key Custody Vault",
      "DeFi & DEX Smart Protocols",
      "Decentralized Marketplace & App Store",
      "Globus Studio IDE & Visual Compiler",
      "Block Explorer & State Trie Inspector"
    ],
    inboundContract: "Contract C1 (UI → Application)",
    outboundContract: "Contract C2 (Application → Platform)",
    boundaryRule: "Verboten sind Kernel-Aufrufe ohne SDK-Vermittlung oder das Auslesen fremder Adressräume."
  },
  {
    id: "d3-platform",
    code: "D3",
    name: "Platform Domain",
    badge: "Middleware & Frameworks",
    color: "purple",
    headline: "SDKs, API-Gateways, Middleware, Services, AI & Blockchain",
    description: "Zentrale Orchestrierungs- und Integrationsplattform. Stellt Standard-Bibliotheken bereit, managt den Zugriff auf verteilte Ledger und KI-Agenten.",
    components: [
      "SDK Layer: ATC SDK, ShivaCore SDK, Agent SDK, Game SDK, UI SDK",
      "Platform APIs: REST, GraphQL, gRPC, WebSocket, JSON-RPC, Shared-Memory IPC",
      "Control & Middleware: API Gateway, Service Mesh, NATS Event Bus, Message Queue",
      "Platform Services: Identity (DID), Token & NFT Engine, Oracles, Analytics",
      "AI Platform: Model Manager, Inference Router, Agent Policy Gatekeeper",
      "ATC Blockchain Platform: Core Node, P2P Kademlia/Gossip, Mempool, Consensus"
    ],
    inboundContract: "Contract C2 (Application → Platform)",
    outboundContract: "Contract C3 (Platform → Runtime)",
    boundaryRule: "Services sind logische Microservices; Querschnittsfunktionen (Security, Policy, Audit) sind Plattform-Funktionen und keine austauschbaren Services."
  },
  {
    id: "d4-execution",
    code: "D4",
    name: "Execution Domain",
    badge: "Runtimes & Sandboxes",
    color: "amber",
    headline: "ATC VM, WASM Runtime, Agent Runtime, Containers & Plugins",
    description: "Deterministische Ausführungsumgebungen und isolierte Gast-Instanzen. Verwaltet Gas-Metering, Stack-Limits und Berechtigungs-Sandboxen.",
    components: [
      "ATC VM (Deterministischer Bytecode-Interpreter, Gas-Metering, Stack-Isolation)",
      "ATCLang Native Runtime & JIT Engine",
      "WASM Runtime (Wasmtime-basierte isolierte Gast-Instanzen)",
      "Agent Runtime (Autonome Task-Planung & Coroutinen-Scheduler)",
      "Container Runtime (Lightweight micro-containers mit Namespace-Isolation)",
      "Plugin Runtime (Dynamische Erweiterungen mit restriktiven Rechten)"
    ],
    inboundContract: "Contract C3 (Platform → Runtime)",
    outboundContract: "Contract C4 (Host ABI → Syscall ABI)",
    boundaryRule: "Gäste (Smart Contracts, WASM, Agenten) greifen NIEMALS direkt auf Kernel-Syscalls zu, sondern ausschließlich über die Host ABI (Trap/eCall)."
  },
  {
    id: "d5-system",
    code: "D5",
    name: "System Domain",
    badge: "Kernel, HAL & Silicon",
    color: "emerald",
    headline: "Syscall ABI, ShivaCore Microkernel, HAL, Drivers & Hardware",
    description: "Hardware-autoritative Basisschicht. Enthält den minimalistischen Ring-0-Microkernel, das Capability-Subsystem und die Hardware-Abstraktionen.",
    components: [
      "Syscall ABI Dispatcher (Fast syscall via MSR_LSTAR / svc #0)",
      "ShivaCore Microkernel Nucleus (Preemptive Scheduler, Memory Paging, Fast IPC)",
      "Capability Object Manager & Security Subsystem",
      "Virtual File System (VFS) & Storage Drivers",
      "Zero-Copy Network Stack & Crypto Acceleration",
      "Hardware Abstraction Layer (HAL für x86_64, ARM64, RISC-V)",
      "Silicon Hardware (CPU, GPU, RAM, NVMe, PCIe, NIC, TPM 2.0)"
    ],
    inboundContract: "Contract C4 & Contract C5 (Syscall ABI)",
    outboundContract: "Hardware Architecture (MMIO, Port I/O, DMA, CPU Rings)",
    boundaryRule: "Der Kernel enthält nur primitive, vertrauenswürdige Funktionen (CPU, Memory, IPC, Process, Capabilities). Keine Business-, NFT- oder KI-Logik im Ring 0."
  }
];

// 2. DIE 7 HORIZONTALEN CONTROL PLANES
export const V3_CONTROL_PLANES: ControlPlaneV3[] = [
  {
    id: "security-plane",
    name: "Security Plane",
    color: "red",
    tagline: "End-to-End Zero-Trust, Identity, Capabilities & Cryptographic Isolation",
    nature: "Plattformfunktion (Cross-Cutting)",
    coreFunctions: [
      "Identity (Decentralized Identifiers - DID / Public Keys)",
      "Authentication (Kryptografische Signaturverifikation, Challenge-Response)",
      "Authorization (Capability-Token basierter Zugriffsschutz)",
      "Capability Lineage & Rekursive Revocation (DAG)",
      "Encryption (AES-256-GCM, ChaCha20-Poly1305, Zero-Copy in-flight)",
      "Secrets Vault (TPM 2.0 Sealed Storage & Hardware Enclaves)",
      "Process & Memory Isolation (PML4 / EPT Hardware Enclaves)"
    ],
    enforcementMechanism: "12-Stufen ShivaCore Security Execution Pipeline",
    auditScope: "Alle CAP_GRANT, CAP_REVOKE, AUTH_FAIL und SECURITY_VIOLATION Events"
  },
  {
    id: "governance-plane",
    name: "Governance Plane",
    color: "yellow",
    tagline: "Regelwerke, Systemberechtigungen, Ressourcen-Ökonomie & Compliance",
    nature: "Plattformfunktion (Cross-Cutting)",
    coreFunctions: [
      "Decentralized Governance (On-Chain DAO Proposals & Quadratic Voting)",
      "Rule Engines (Deklarative Protokoll- und Konsens-Regeln)",
      "System Permissions & Role Elevation Gates",
      "Economics (EIP-1559 Base Fee & Tip, Gas Pricing, Staking Slashing)",
      "Compliance & Data Protection (Crypto-Shredding, DSGVO-konforme Löschung)"
    ],
    enforcementMechanism: "Smart Contract Timelocks & Kernel Policy Gates",
    auditScope: "Abstimmungsergebnisse, Parameteränderungen, Slashing-Vorvermerke"
  },
  {
    id: "observability-plane",
    name: "Observability Plane",
    color: "emerald",
    tagline: "Echtzeit-Telemetrie, verteilte Traces, Health-Checks & Metriken",
    nature: "Plattformfunktion (Cross-Cutting)",
    coreFunctions: [
      "Metrics (Lockless Ring-Buffer Erfassung von CPU, Memory, IPC, Gas)",
      "Logs (Strukturierte JSON-ND / Protocol Buffer Stream Logs)",
      "Distributed Tracing (W3C Trace Context Propagation von UI bis Ring 0)",
      "Events (Event-Stream Erfassung aller Domain-Übergänge)",
      "Health (Liveness-, Readiness- und Hardware-Sensor-Probes)",
      "Telemetry Reporting (Sub-Millisekunden Auswertung ohne Kernel-Locks)"
    ],
    enforcementMechanism: "Lockless Atomare Ringspeicher & vDSO TSC Timer",
    auditScope: "Aggregierte Performance-Profile und Anomalie-Detektionen"
  },
  {
    id: "audit-plane",
    name: "Audit Plane",
    color: "cyan",
    tagline: "Manipulationssicheres Audit-Trail, Sicherheitsereignisse & State-Provenance",
    nature: "Plattformfunktion (Cross-Cutting)",
    coreFunctions: [
      "Audit Trail (Lineare, kryptografisch verkettete Merkle-Tree Hashes)",
      "Security Events (Strikte Erfassung aller Syscall-Traps & Privileg-Anfragen)",
      "State Changes (Transaktionsnachweise & State-Trie Mutation Records)",
      "Provenance (Lückenlose Herkunftsnachweise für Daten, Binaries & Modelle)"
    ],
    enforcementMechanism: "Kryptografisch versiegelte Audit-Puffer im Ring 0",
    auditScope: "100% aller zustandsverändernden Syscalls und Host-ABI Aufrufe"
  },
  {
    id: "policy-plane",
    name: "Policy Plane",
    color: "purple",
    tagline: "Deklarative Zugriffs-, Ressourcen-, Agenten- und Contract-Policies",
    nature: "Plattformfunktion (Cross-Cutting)",
    coreFunctions: [
      "Access Policies (Open Policy Agent / Rego-kompatible Regelwerke)",
      "Resource Policies (Dynamische Begrenzungen basierend auf Systemlast)",
      "Agent Policies (Human-in-the-Loop Freigaben für KI-Werkzeuge)",
      "Contract Policies (Reentrancy-Prüfung & Whitelist für fremde Contracts)",
      "Runtime Policies (Seccomp/BPF Syscall Filtering für Container)"
    ],
    enforcementMechanism: "In-Memory Policy Evaluator vor Syscall/Host Trap Dispatch",
    auditScope: "Policy Allow/Deny Entscheidungen inklusive Kontext-Snapshot"
  },
  {
    id: "resource-plane",
    name: "Resource Plane",
    color: "blue",
    tagline: "Globale Ressourcenverwaltung: CPU, Memory, Storage, Network, GPU & Quotas",
    nature: "Plattformfunktion (Cross-Cutting)",
    coreFunctions: [
      "CPU Scheduling & Core Affinity",
      "Memory Allocation Quotas & OOM Handling",
      "Storage I/O Bandwidth & IOPS Throttling",
      "Network Ingress/Egress Rate Limiting",
      "GPU / NPU Tensor Core Allocation",
      "Unified Quota Accounting über alle Runtimes hinweg"
    ],
    enforcementMechanism: "Hardware APIC Timer, cgroup v2 Accounting & Gas Metering",
    auditScope: "Quota-Überschreitungen und Ressourcen-Drosselungen"
  },
  {
    id: "configuration-plane",
    name: "Configuration Plane",
    color: "teal",
    tagline: "Zentrale Konfiguration, Feature Flags, Runtime-Parameter & Geheimnisse",
    nature: "Plattformfunktion (Cross-Cutting)",
    coreFunctions: [
      "Configuration State (Typisierte, unveränderliche Konfigurationsblöcke)",
      "Feature Flags (Dynamische Freischaltung ohne Neustart)",
      "Runtime Parameters (Tuning für Scheduler, Netzwerk-Puffer, Mempool)",
      "Secrets Injection (Zeitlich beschränkte Ephemeral Tokens für Prozesse)"
    ],
    enforcementMechanism: "Atomic Pointer Swapping & Versionierte Konfigurations-Tries",
    auditScope: "Änderungen an Systemparametern und Flag-Zuständen"
  }
];

// 3. DIE 5 ARCHITEKTURVERTRÄGE (C1 - C5)
export const V3_CONTRACTS: ContractV3[] = [
  {
    id: "c1-ui-app",
    code: "C1",
    title: "Contract C1 — UI → Application",
    sourceLayer: "D1: Experience Domain (Aurora / Web / Mobile)",
    targetLayer: "D2: Application Domain (Apps / Wallet / Studio)",
    color: "cyan",
    description: "Definiert die standardisierte Schnittstelle zwischen UI-Präsentation und Anwendungslogik. Garantiert Zustandsisolation, asynchrone Events und Eingabevalidierung.",
    flowSteps: [
      "UI sendet typisierten User-Action Request (JSON-RPC oder FlatBuffers)",
      "Application API validiert das Request-Schema und die Session-Tokens",
      "Berechtigungsprüfung gegen den aktiven Benutzerzustand (AuthN/AuthZ)",
      "Application Service führt Logik aus und erzeugt State-Updates",
      "Rückgabe einer typisierten Response oder eines Fehler-Modells inklusive Trace-ID"
    ],
    contractClauses: [
      {
        name: "Authentication & Session",
        description: "Jeder Request muss eine signierte Session-ID oder ein ephemeres JWT/DID-Token mitführen."
      },
      {
        name: "Request / Response Schema",
        description: "Deklaratives JSON-Schema oder Protobuf v3 Vertrag. Unbekannte Felder führen zu SCHEMA_VALIDATION_ERROR."
      },
      {
        name: "Error Model",
        description: "Einheitliche Fehlerstruktur mit { code, message, details, retryable, traceId }."
      },
      {
        name: "Telemetry & Context",
        description: "W3C Trace-Context Header (traceparent, tracestate) werden verpflichtend übergeben."
      }
    ],
    failureModes: ["SESSION_EXPIRED", "SCHEMA_INVALID", "UNAUTHORIZED_ACTION", "RATE_LIMIT_EXCEEDED"]
  },
  {
    id: "c2-app-platform",
    code: "C2",
    title: "Contract C2 — Application → Platform",
    sourceLayer: "D2: Application Domain (Wallet / DeFi / Games)",
    targetLayer: "D3: Platform Domain (SDK / API / Services)",
    color: "blue",
    description: "Verbindet Client-Applikationen mit den zentralen Plattformdiensten. Geregelt über offizielle SDKs und Middleware-Gateways.",
    flowSteps: [
      "Application ruft SDK-Methode auf (z.B. walletSdk.submitTransaction(...))",
      "SDK serialisiert Payload und hängt Client-Signatur & Capability-Handle an",
      "API Gateway authentifiziert den Principal und erzwingt OPA-Policies",
      "Service Mesh routet an Ziel-Microservice (z.B. Wallet Service oder Mempool Engine)",
      "Service liefert Ergebnis über asynchronen Stream oder synchrone Antwort zurück"
    ],
    contractClauses: [
      {
        name: "SDK Bindings",
        description: "Applikationen nutzen ausschließlich typsichere SDKs (Rust, TypeScript, C, Go), keine ungesicherten Raw-Sockets."
      },
      {
        name: "API Gateway Enforcement",
        description: "Gateway terminiert TLS, prüft DIDs, verifiziert Nonces und berechnet Quota-Verbrauch."
      },
      {
        name: "Service Contract",
        description: "gRPC oder JSON-RPC 2.0 Schnittstelle mit idempotenten Transaktions-Hashes."
      }
    ],
    failureModes: ["GATEWAY_REJECTED", "CAPABILITY_INSUFFICIENT", "SERVICE_UNAVAILABLE", "NONCE_MISMATCH"]
  },
  {
    id: "c3-platform-runtime",
    code: "C3",
    title: "Contract C3 — Platform → Runtime",
    sourceLayer: "D3: Platform Domain (Services / Nodes / Agent Planner)",
    targetLayer: "D4: Execution Domain (ATC VM / WASM / Agent Runtime)",
    color: "purple",
    description: "Formaler Vertrag für die Codeausführung in einer isolierten Sandbox. Definiert Speicherobergrenzen, Gas-Budgets, Timeout und deterministische Haltebedingungen.",
    flowSteps: [
      "Platform Service instanziiert Runtime Context mit festgelegtem Gas-/Memory-Limit",
      "Runtime verifiziert Bytecode-Signatur und lädt ihn in isolierten Stack",
      "Ausführung beginnt unter kontinuierlichem Gas- und Stack-Metering",
      "Gast-Code ruft bei Bedarf Host-Funktionen über Trap/eCall auf",
      "Nach Beendigung liefert Runtime Execution State, Exit Code, Gas-Verbrauch und Mutationen zurück"
    ],
    contractClauses: [
      {
        name: "Execution & Determinism",
        description: "Gleicher Input + Bytecode führt auf jedem Knoten zum exakt identischen State Root."
      },
      {
        name: "Gas & Resource Metering",
        description: "Jeder Opcode und jeder Host-Call konsumiert Gas. Bei 0 stoppt die Ausführung mit OUT_OF_GAS."
      },
      {
        name: "Memory Bounds",
        description: "Maximaler linearer Speicher (z.B. 64 MB für Smart Contracts, 512 MB für WASM Plugins)."
      },
      {
        name: "Timeout & Cancellation",
        description: "Asynchrone Cancellation-Tokens ermöglichen den Abbruch hängender oder bösartiger Prozesse."
      }
    ],
    failureModes: ["OUT_OF_GAS", "STACK_OVERFLOW", "MEMORY_LIMIT_EXCEEDED", "ILLEGAL_INSTRUCTION", "TIMEOUT"]
  },
  {
    id: "c4-runtime-host-syscall",
    code: "C4",
    title: "Contract C4 — Runtime → Syscall ABI (Host ABI vs Syscall ABI)",
    sourceLayer: "D4: Execution Domain (ATC VM / WASM / Containers)",
    targetLayer: "D5: System Domain (Syscall Dispatcher / Kernel)",
    color: "amber",
    description: "Kritische Systemgrenze: Strikte Trennung zwischen nativer Syscall ABI (Ring 3 OS-Prozesse) und Host ABI (GOS-HABI-001 für VM-Gäste, WASM, Agenten).",
    flowSteps: [
      "Gast-Code (z.B. Smart Contract) löst Trap/eCall aus (Host Function Request)",
      "ATC VM fängt Trap ab und prüft Sandbox-Berechtigung",
      "Host Interface validiert Host ABI Parameter (GOS-HABI-001)",
      "Host Interface übersetzt bei Berechtigung in einen echten Kernel-Syscall (GOS-ABI-001)",
      "Syscall ABI Dispatcher führt Hardware-Register Trap durch (MSR_LSTAR / svc #0)"
    ],
    contractClauses: [
      {
        name: "Duale ABI-Trennung",
        description: "Syscall ABI ist für native Ring-3-Prozesse. Host ABI ist für VM-Gäste. Smart Contracts kennen niemals direkte Kernel-Syscalls."
      },
      {
        name: "Trap & Translation Shim",
        description: "Host-Funktionen sind typisierte Traps (z.B. host_storage_read, host_crypto_verify), die durch den Host gemanagt werden."
      },
      {
        name: "Memory Safety",
        description: "Kernel schreibt niemals ungeprüft in den Gast-Adressraum; Datenübertragungen laufen über validierte Puffer."
      }
    ],
    failureModes: ["HOST_TRAP_DISALLOWED", "SANDBOX_ESCAPE_ATTEMPT", "BAD_HOST_POINTER", "ABI_VERSION_MISMATCH"]
  },
  {
    id: "c5-syscall-kernel",
    code: "C5",
    title: "Contract C5 — Syscall → Kernel (Security Execution Pipeline)",
    sourceLayer: "D5 Syscall Dispatcher (Ring 3/0 Transition)",
    targetLayer: "D5 ShivaCore Kernel Nucleus (Ring 0 Resource)",
    color: "emerald",
    description: "Das verbindliche 12-Stufen ShivaCore Security Execution Pipeline Model. Kein Kernel-Zugriff ohne lückenlosen Validierungsdurchlauf.",
    flowSteps: [
      "1. Request: Syscall Stub lädt Opcode & Argumente in Register (RAX, RDI, RSI...)",
      "2. ABI: Hardware Trap (syscall / svc #0) wechselt Ring 3 → Ring 0 (MSR_LSTAR)",
      "3. Validation: Überprüfung von Pointern, Adressbereich und Alignment",
      "4. Identity: Authentifizierung des aufrufenden Principals (Thread/Process ID)",
      "5. Namespace: Validierung des Prozess- und Container-Namespaces (PID/Mount/Net)",
      "6. Handle: O(1) Check des Ressourcen-Handles in der Prozesstabelle",
      "7. Capability: Kryptografische Verifikation des Capability-Tokens & Revocation-Status",
      "8. Rights: Bitweiser Abgleich gegen READ/WRITE/EXEC/TRANSFER Masken",
      "9. Policy: Evaluierung dynamischer Kernel-Sicherheitsregeln (OPA/MAC)",
      "10. Quota: Überprüfung und Dekrementierung des Ressourcenkontingents",
      "11. Resource: Ausführung der Operation im Kernel-Subsystem (CPU/RAM/VFS/Net)",
      "12. Audit & Result: Unveränderlicher Log-Eintrag im Merkle-Buffer und Rückgabe an Userland"
    ],
    contractClauses: [
      {
        name: "Atomare Pipeline",
        description: "Jeder Schritt muss erfolgreich sein. Ein Fehlschlag an einer Stufe bricht sofort ab (Fail-Closed)."
      },
      {
        name: "Zero-Trust Kernel",
        description: "Selbst Kernel-Module und Treiber unterliegen der Capability- und Rights-Prüfung."
      },
      {
        name: "Audit Guarantees",
        description: "Sicherheitsrelevante Operationen erzeugen garantiert einen synchronen Audit-Eintrag vor Rückkehr."
      }
    ],
    failureModes: ["ABI_FAULT", "BAD_HANDLE", "CAPABILITY_REVOKED", "RIGHTS_VIOLATION", "QUOTA_EXCEEDED", "POLICY_DENIED"]
  }
];

// 4. FORMALE SYSTEM-BOUNDARIES (KERNEL, ATC VM, BLOCKCHAIN, AI, CONTAINER)
export const V3_SYSTEM_BOUNDARIES: SystemBoundaryV3[] = [
  {
    id: "kernel-boundary",
    title: "Kernel Boundary (ShivaCore Microkernel)",
    subtitle: "Ring-0-Nucleus: Ausschließlich primitive, vertrauenswürdige Funktionen",
    color: "emerald",
    internalResponsibilities: [
      "CPU: Preemptive Priority Scheduler, APIC/GIC Timer Interrupts, Exceptions & Fault Handling",
      "Memory: PML4/TTBR0 4-Level Paging, Physical Frame Allocator (Buddy), Address Spaces & Protection (NX/WP)",
      "Process: Process Lifecycle, Thread State Machine, Thread Local Storage (TLS)",
      "IPC: SPSC/MPMC Ring-Buffer Channels, Fastpath Rendezvous (Call/Reply), Shared Memory Mapping",
      "Capability: Handle Table Management, Rights Mask Enforcement, Capability Lineage DAG & Revocation",
      "Objects: Kernel Object Lifetime & Reference Counting",
      "Resources: Hardware CPU Time Slices, Memory Pages, Device Port Access & Harte Quotas",
      "Security: Hardware Ring 0 Enforcement, IOMMU DMA Protection, Zero-Trust Execution Pipeline",
      "Audit: Lockless Ring Buffer, Kernel Trace Events & Security Audits"
    ],
    strictlyExcludedResponsibilities: [
      "KEIN Marketplace, NFT-Handel oder Token-Logik im Kernel",
      "KEINE Blockchain-Konsensfindung oder Mempool-Verwaltung in Ring 0",
      "KEINE KI-Modell-Inferenz, RAG-Embeddings oder Agenten-Planung",
      "KEINE GUI-Render-Pipelines, Fenster-Management oder Compositor-Code",
      "KEIN komplexes Dateisystem-Parsing (Treiber laufen in Ring 1/Userland)"
    ],
    formalPipeline: [
      "Userland Syscall",
      "Hardware Trap",
      "Register Save",
      "Pipeline Validation (C5)",
      "Microkernel Subsystem",
      "Result Register",
      "Hardware Return (sysretq)"
    ],
    rationale: "Minimale Angriffsfläche (Minimal Trusted Computing Base). Abstürze in Applikationen, VMs oder Treibern bringen niemals den Kernel zum Erliegen."
  },
  {
    id: "vm-boundary",
    title: "ATC VM Boundary (Execution Engine)",
    subtitle: "Deterministische Ausführungsebene für Bytecode und Smart Contracts",
    color: "amber",
    internalResponsibilities: [
      "Bytecode Parsing & Ahead-Of-Time/JIT Validation (ATCB Bytecode)",
      "Instruction Set Execution (Deterministische Register/Stack Virtual Machine)",
      "Gas Metering: Exakter Instruktions- und Speicher-Kostenabzug pro Opcode",
      "Contract Memory Sandbox (Isolierter Linear Memory, Stack & Storage Cache)",
      "Execution Determinism: 100% reproduzierbare Zustandsübergänge auf allen CPUs",
      "Exception & Reentrancy Guards innerhalb des Ausführungskontexts"
    ],
    strictlyExcludedResponsibilities: [
      "ATC VM ist NICHT die Blockchain (VM führt nur aus, Blockchain regelt State & Konsens)",
      "KEIN direkter Zugriff auf physischen RAM, Dateisysteme oder Kernel-Threads",
      "KEINE P2P-Netzwerkverbindungen direkt aus dem Bytecode",
      "KEINE unbeschränkten Endlosschleifen (Gas-Limit garantiert Halt-Eigenschaft)"
    ],
    formalPipeline: [
      "ATCLang Source",
      "Compiler",
      "ATCB Bytecode",
      "Bytecode Verifier",
      "ATC VM Runtime",
      "Gas Metered Execution",
      "Host Interface (GOS-HABI-001)",
      "State Transition Receipt"
    ],
    rationale: "Die VM ist eine reine Ausführungsmaschine. Sie weiß nichts von P2P-Blöcken oder physischer Hardware."
  },
  {
    id: "blockchain-boundary",
    title: "Blockchain Boundary (Distributed State & Consensus)",
    subtitle: "A-TownChain Protokoll-Engine: Dezentraler Ledger und State Trie",
    color: "cyan",
    internalResponsibilities: [
      "P2P Network Overlay (Kademlia DHT, GossipSub v1.2, Noise Protocol Encryption)",
      "Mempool Management (Priorisierung, Nonce-Tracking, MEV-Protection)",
      "Consensus Engine (Proof of Stake Finality Gadget, Threshold Signatures)",
      "Block Assembly & Proposal Packaging",
      "State Machine & Trie Database (Modified Merkle Patricia Trie, State Snapshots)",
      "Transaktions-Signaturprüfung (Ed25519, Secp256k1) über SIMD/Crypto Hal",
      "Aufruf der ATC VM zur Ausführung von Transaktions-Batches"
    ],
    strictlyExcludedResponsibilities: [
      "Blockchain führt keinen Bytecode selbst aus (delegiert strikt an ATC VM)",
      "KEIN Betriebssystem-Scheduler oder direkte Hardware-Treiber-Verwaltung",
      "KEINE Speicherung riesiger unstrukturierter Rohdaten auf der Chain (nur IPFS/Storage Hashes)"
    ],
    formalPipeline: [
      "App Transaction",
      "ATC SDK",
      "ATC API Gateway",
      "Mempool Ingress",
      "Consensus Round",
      "Block Commit",
      "State Transition to ATC VM",
      "Merkle Trie Commit"
    ],
    rationale: "Klare Trennung: Blockchain = Konsens und persistenter State; ATC VM = deterministische Berechnung."
  },
  {
    id: "ai-boundary",
    title: "AI / Agent Boundary (Autonome Principals)",
    subtitle: "KI ist ein kontrollierter Principal, kein privilegierter Systemprozess",
    color: "purple",
    internalResponsibilities: [
      "Model Management (Lokale quantisierte Modelle GGUF/ONNX & Cloud LLMs)",
      "Tensor Inferenz-Scheduling über NPU/GPU Hardware Enclaves",
      "Agent Runtime: ReAct Loop, Ziel-Dekomposition, Task Priorisierung",
      "Skill & Tool Dispatcher (Schema-validierte JSON Tool Calls)",
      "Memory Hierarchy: Kontext-Fenster, episodischer Speicher & Vektor-Embeddings",
      "Policy & Approval Gate: Human-in-the-Loop bei risikobehafteten Aktionen"
    ],
    strictlyExcludedResponsibilities: [
      "KEIN direkter Kernel-Zugriff (Agent → Skill → Tool → Policy → Capability → Host API → Syscall → Kernel)",
      "Agenten besitzen keine Superuser-Rechte per Default",
      "KEIN unbeschränkter Netzwerkzugriff ohne Policy-Freigabe"
    ],
    formalPipeline: [
      "User / Event Trigger",
      "Agent Planner",
      "Tool Invocation Request",
      "Policy & Risk Evaluation",
      "Capability Request",
      "Host API Bridge",
      "Syscall ABI Dispatch",
      "Audited Execution"
    ],
    rationale: "Verhindert Prompt-Injection-Escapes und unkontrollierte Hardwareressourcen-Auslastung durch KI-Agenten."
  },
  {
    id: "container-boundary",
    title: "Container Boundary (ShivaBox Sandbox)",
    subtitle: "Erstklassige Kernel-Sandboxen als isolierte Execution Domains",
    color: "teal",
    internalResponsibilities: [
      "Process Namespace (PID Isolation; PID 1 als Container Init)",
      "Network Namespace (Dediziertes Virtual Ethernet veth Paar, eigene Routing-Tabelle)",
      "Mount Namespace (Chroot/Pivot-Root mit Copy-on-Write ShivaFS Layer)",
      "IPC Namespace (Isolierte POSIX/ShivaCore Message Queues und Shared Memory)",
      "Resource Domain (cgroup v2 CPU, Memory, I/O Quotas mit OOM-Schutz)",
      "Bounded Capability Set (Minimal-Privilege Whitelist)",
      "BPF Syscall Filter (Seccomp-Default-Deny für unautorisierte Syscalls)"
    ],
    strictlyExcludedResponsibilities: [
      "Container ist kein Hardware-Hypervisor (teilt denselben Microkernel)",
      "KEIN direkter Hardware-MMIO-Zugriff ohne explizite Device-Capability",
      "KEIN Ausbruch aus dem chroot-Dateisystem"
    ],
    formalPipeline: [
      "sc_container_create()",
      "Namespace Setup",
      "Capability Mask Clamp",
      "COW Overlay Mount",
      "BPF Filter Attach",
      "sc_container_start()",
      "Isolated PID 1 Exec"
    ],
    rationale: "Volle Prozess-Sicherheit bei minimalem Overhead (< 2 ms Startzeit, kein Hypervisor-Verlust)."
  }
];

// 5. EINHEITLICHE GLOBALE MODELLE (RESOURCE, IDENTITY, EVENT)
export const V3_UNIFIED_MODELS: UnifiedModelV3[] = [
  {
    id: "gos-res-001",
    standardCode: "GOS-RES-001",
    title: "Einheitliches Resource Model",
    color: "blue",
    concept: "Standardisierte Abstraktion für alle zählbaren und limitierbaren Systemgüter über OS, Container, VM, Agent und Blockchain hinweg.",
    principalsOrConsumers: ["ShivaCore Scheduler", "cgroup Controller", "ATC VM Gas Engine", "Agent Quota Manager"],
    guarantees: [
      "Keine Ressource existiert ohne eindeutige ID, Owner und Capability-Zuordnung",
      "Verbrauch wird atomar gegen harte und weiche Limits abgerechnet",
      "Überschreitungen triggern definierte Drosselung oder OOM/OUT_OF_GAS Exceptions"
    ],
    schemaFields: [
      { field: "resource_id", type: "uuid / u64", description: "Global eindeutiger Identifier der Ressource", example: "res-cpu-core-03" },
      { field: "owner_principal", type: "principal_id_t", description: "Verantwortlicher Principal (User, Process, Contract)", example: "proc:0x4f12a" },
      { field: "namespace", type: "string", description: "Zugeordneter Isolations-Namespace", example: "cgroup:/system/atc-node" },
      { field: "capability_handle", type: "sc_handle_t", description: "Zugehöriges Zugriffs-Token", example: "0x0000000a:gen_1" },
      { field: "resource_type", type: "enum", description: "CPU | MEMORY | STORAGE | NET | GPU | IPC | GAS | BUDGET", example: "RESOURCE_GAS" },
      { field: "quota_soft_limit", type: "uint64_t", description: "Schwelle für Warn-Events und sanfte Drosselung", example: "8000000" },
      { field: "quota_hard_limit", type: "uint64_t", description: "Absolute Obergrenze; Überschreitung bricht Ausführung ab", example: "10000000" },
      { field: "current_usage", type: "uint64_t", description: "Aktuell belegtes Kontingent", example: "2419200" },
      { field: "policy_rule", type: "policy_id_t", description: "Regelwerk für Nachallokation oder Slashing", example: "pol-strict-burst-deny" },
      { field: "lifecycle_state", type: "enum", description: "ACTIVE | THROTTLED | DEPLETED | RELEASED", example: "ACTIVE" }
    ]
  },
  {
    id: "gos-id-001",
    standardCode: "GOS-ID-001",
    title: "Einheitliches Identity Model",
    color: "red",
    concept: "Einheitliche Identitätsabstraktion: Jeder Akteur (Mensch, Prozess, Thread, Container, Agent, Smart Contract, Validator) ist ein typisierter Principal.",
    principalsOrConsumers: ["Security Plane", "Capability Manager", "API Gateway", "Audit Subsystem"],
    guarantees: [
      "Jeder Principal besitzt eine kryptografisch überprüfbare Identität (DID / Signatur)",
      "Berechtigungen basieren auf dezentralen Capabilities, nicht auf statischen UID-Tabellen",
      "Identitäten können Rollen und temporäre Ephemeral-Tokens delegieren"
    ],
    schemaFields: [
      { field: "principal_id", type: "principal_t", description: "Globales typisiertes Identifikations-Token", example: "did:atc:0x71C...a89" },
      { field: "principal_type", type: "enum", description: "USER | PROCESS | THREAD | SERVICE | AGENT | CONTAINER | CONTRACT | VALIDATOR", example: "PRINCIPAL_AGENT" },
      { field: "display_alias", type: "string", description: "Menschenlesbarer Name für Audit und UI", example: "DevOps-Assistant-v1" },
      { field: "credentials", type: "credential_set_t", description: "Kryptografische Nachweise (Ed25519 / TPM Attestation)", example: "{ pubkey: '0xabc...', type: 'ed25519' }" },
      { field: "root_capabilities", type: "sc_handle_t[]", description: "Basis-Berechtigungen des Principals", example: "[0x01, 0x05, 0x12]" },
      { field: "rights_mask", type: "uint64_t", description: "Globales Privilegien-Bitfeld", example: "0x000000000000000F" },
      { field: "active_namespace", type: "ns_handle_t", description: "Aktuell zugewiesener Namespace-Kontext", example: "ns:container-sandbox-04" },
      { field: "policy_bindings", type: "policy_id_t[]", description: "Verbindliche Sicherheitsrichtlinien", example: "['pol-agent-sandbox', 'pol-net-egress-deny']" },
      { field: "creation_ts", type: "uint64_t", description: "Erstellungszeitpunkt in Nanosekunden (Monotone Uhr)", example: "1725278400000000" }
    ]
  },
  {
    id: "gos-aud-001",
    standardCode: "GOS-AUD-001",
    title: "Einheitliches Event & Audit Model",
    color: "emerald",
    concept: "Universelles Event-Schema für Observability, Telemetrie, Sicherheits-Audits und State-Provenance über alle 5 Domains hinweg.",
    principalsOrConsumers: ["Observability Plane", "Audit Plane", "Security SIEM", "Blockchain Block Explorer"],
    guarantees: [
      "Jedes Ereignis enthält Pflichtfelder für Herkunft, Principal, Ressource und Entscheidung",
      "Kryptografische Verkettung verhindert nachträgliche Modifikationen",
      "Sub-Millisekunden Enqueue über lockless Ringpuffer"
    ],
    schemaFields: [
      { field: "event_id", type: "uuid / u64", description: "Monoton steigender Ereignis-Identifier", example: "evt-00049281-99" },
      { field: "timestamp_ns", type: "uint64_t", description: "Hardware TSC Timestamp in Nanosekunden", example: "1725278491823192" },
      { field: "actor_principal", type: "principal_t", description: "Auslösender Principal", example: "proc:0x4a11 (Aurora Shell)" },
      { field: "subject", type: "string", description: "Betroffene Entität oder Operation", example: "SC_MEMORY_ALLOC" },
      { field: "resource_id", type: "string", description: "Betroffene Ressource", example: "res-mem-page-pool" },
      { field: "namespace", type: "string", description: "Namespace-Kontext des Events", example: "cgroup:/system" },
      { field: "action", type: "string", description: "Ausgeführte Aktion", example: "MEMORY_MAP_VMA" },
      { field: "result", type: "enum", description: "SUCCESS | DENIED | FAULT | TIMEOUT", example: "SUCCESS" },
      { field: "policy_decision", type: "enum", description: "ALLOW | DENY | ESCALATE | AUDIT_ONLY", example: "ALLOW" },
      { field: "capability_used", type: "sc_handle_t", description: "Verwendetes Capability Token", example: "0x0000001f" },
      { field: "correlation_id", type: "string (W3C Trace ID)", description: "Globale Distributed Tracing Correlation ID", example: "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01" },
      { field: "metadata", type: "bytes / JSON", description: "Zusätzliche typisierte Nutzdaten", example: "{ bytes_allocated: 4096, vaddr: '0x7fff0000' }" }
    ]
  }
];

// 6. DIE 19 FORMALEN SPEZIFIKATIONS-STANDARDS (NORMENFAMILIE)
export const V3_STANDARDS_FAMILY: StandardSpecV3[] = [
  {
    code: "GOS-ARCH-001",
    title: "Globus OS Reference Architecture Standard",
    category: "Architecture",
    status: "Normative",
    description: "Definition der 5 vertikalen Domains, 7 horizontalen Control Planes und der End-to-End Systemarchitektur.",
    primaryArtifact: "SPEC_GOS_ARCH_001.md",
    referencedBy: ["GOS-ABI-001", "GOS-RUN-001", "ATC-NODE-001"]
  },
  {
    code: "GOS-ABI-001",
    title: "ShivaCore Syscall ABI Specification",
    category: "ABI & Kernel",
    status: "Standardized",
    description: "Formale Opcode-Matrix, Calling Conventions, Registerbelegung (x86_64, ARM64, RISC-V) für native Syscalls.",
    primaryArtifact: "include/shivacore/syscall_abi.h",
    referencedBy: ["GOS-HABI-001", "GOS-CAP-001", "GOS-IPC-001"]
  },
  {
    code: "GOS-HABI-001",
    title: "Runtime Host ABI Specification",
    category: "ABI & Kernel",
    status: "Standardized",
    description: "Spezifikation der Trap-/eCall-Schnittstelle zwischen Runtimes (ATC VM, WASM, Agents) und dem OS Host Interface.",
    primaryArtifact: "include/shivacore/host_abi.h",
    referencedBy: ["ATC-VM-001", "GOS-RUN-001", "GOS-ABI-001"]
  },
  {
    code: "GOS-IPC-001",
    title: "Inter-Process Communication (IPC) Protocol Standard",
    category: "ABI & Kernel",
    status: "Normative",
    description: "Lockless SPSC/MPMC Ringpuffer, Rendezvous L4-Style Fastpath (Call/Reply) und Shared-Memory Mapping.",
    primaryArtifact: "kernel/ipc/ipc_core.c",
    referencedBy: ["GOS-ABI-001", "GOS-CAP-001"]
  },
  {
    code: "GOS-CAP-001",
    title: "Capability Security & Delegation Model",
    category: "Security & Identity",
    status: "Normative",
    description: "Kryptografische Handles, Generation Counters, DAG-basierte Lineage und synchrone/rekursive Revocation.",
    primaryArtifact: "kernel/capability/cap_mgr.c",
    referencedBy: ["GOS-ABI-001", "GOS-HND-001", "GOS-POL-001"]
  },
  {
    code: "GOS-HND-001",
    title: "Kernel Object Handle Management Standard",
    category: "ABI & Kernel",
    status: "Standardized",
    description: "O(1) Prozesstabellen-Lookup, Handle-Duplikation, Übertragung und Lebenszyklus-Garantien.",
    primaryArtifact: "kernel/object/handle_table.c",
    referencedBy: ["GOS-CAP-001", "GOS-ABI-001"]
  },
  {
    code: "GOS-MEM-001",
    title: "Virtual & Physical Memory Model",
    category: "ABI & Kernel",
    status: "Normative",
    description: "4-Level Paging (PML4/TTBR0), Buddy Frame Allocator, Copy-on-Write (COW), TLB Invalidation & vDSO.",
    primaryArtifact: "kernel/memory/vm_manager.c",
    referencedBy: ["GOS-ABI-001", "GOS-RES-001"]
  },
  {
    code: "GOS-RES-001",
    title: "Unified Resource & Quota Accounting Model",
    category: "Architecture",
    status: "Standardized",
    description: "Globales Ressourcen- und Quota-Framework für CPU, Memory, Storage, Net, Gas und Agenten-Budgets.",
    primaryArtifact: "kernel/resource/quota_mgr.c",
    referencedBy: ["GOS-POL-001", "ATC-VM-001"]
  },
  {
    code: "GOS-ID-001",
    title: "Unified Identity & Principal Specification",
    category: "Security & Identity",
    status: "Normative",
    description: "Dezentrale Identitäten (DID), typisierte Principals, Ephemeral Tokens und Credential Binding.",
    primaryArtifact: "platform/identity/did_resolver.c",
    referencedBy: ["GOS-CAP-001", "GOS-AUD-001"]
  },
  {
    code: "GOS-POL-001",
    title: "Declarative Policy Engine Standard",
    category: "Security & Identity",
    status: "Draft v1.0",
    description: "Deklarative Sicherheits- und Zugriffsrichtlinien (OPA/Rego), Human-in-the-Loop Freigaben und BPF Filter.",
    primaryArtifact: "platform/policy/evaluator.c",
    referencedBy: ["GOS-CAP-001", "GOS-RES-001"]
  },
  {
    code: "GOS-AUD-001",
    title: "Unified Audit Event & Provenance Standard",
    category: "Security & Identity",
    status: "Normative",
    description: "Manipulationssicheres Ringpuffer-Schema, Merkle-Baum Hashing und lückenlose State Provenance.",
    primaryArtifact: "kernel/audit/audit_ring.c",
    referencedBy: ["GOS-ABI-001", "GOS-ID-001"]
  },
  {
    code: "GOS-RUN-001",
    title: "Runtime Architecture & Sandbox Standard",
    category: "Architecture",
    status: "Standardized",
    description: "Orchestrierung isolierter Runtimes (WASM, ATC VM, Agenten, Container) und Lebenszyklus-Hooks.",
    primaryArtifact: "runtime/sandbox/runtime_mgr.c",
    referencedBy: ["GOS-HABI-001", "ATC-VM-001"]
  },
  {
    code: "ATC-VM-001",
    title: "ATC Virtual Machine Specification",
    category: "Blockchain & VM",
    status: "Normative",
    description: "ATCB Bytecode Spezifikation, deterministischer Opcode-Satz, Stack-Maschine und Gas-Accounting-Tabelle.",
    primaryArtifact: "runtime/atc_vm/vm_core.c",
    referencedBy: ["ATC-ABI-001", "GOS-HABI-001"]
  },
  {
    code: "ATC-ABI-001",
    title: "ATC VM Host Interface Specification",
    category: "Blockchain & VM",
    status: "Normative",
    description: "Host-Funktionen für Storage Read/Write, Crypto Hashes, Transaktions-Kontext und Gas-Abfrage.",
    primaryArtifact: "runtime/atc_vm/host_interface.c",
    referencedBy: ["ATC-VM-001", "GOS-HABI-001"]
  },
  {
    code: "ATC-NODE-001",
    title: "A-TownChain Core Node Architecture",
    category: "Blockchain & VM",
    status: "Standardized",
    description: "Node-Lebenszyklus, Fast-Sync, Archive-Modus, ZK-Light Client Prover und RPC-Schnittstelle.",
    primaryArtifact: "blockchain/node/node_main.c",
    referencedBy: ["ATC-P2P-001", "ATC-STATE-001"]
  },
  {
    code: "ATC-P2P-001",
    title: "ATC Peer-to-Peer Protocol Standard",
    category: "Blockchain & VM",
    status: "Standardized",
    description: "Kademlia DHT Peer Discovery, GossipSub v1.2 Block/Transaktions-Propagation und Noise Protocol Verschlüsselung.",
    primaryArtifact: "blockchain/p2p/p2p_engine.c",
    referencedBy: ["ATC-NODE-001", "ATC-CONS-001"]
  },
  {
    code: "ATC-STATE-001",
    title: "ATC Merkle Patricia Trie State Model",
    category: "Blockchain & VM",
    status: "Normative",
    description: "Deterministischer State Trie, Account State, Storage Roots, State Pruning und Snapshotting.",
    primaryArtifact: "blockchain/state/state_trie.c",
    referencedBy: ["ATC-NODE-001", "ATC-VM-001"]
  },
  {
    code: "ATC-CONS-001",
    title: "ATC Consensus Engine Specification",
    category: "Blockchain & VM",
    status: "Normative",
    description: "Hybrid Asynchronous BFT Finality Gadget, Proof-of-Stake Validator Management und Slashing Rules.",
    primaryArtifact: "blockchain/consensus/consensus_engine.c",
    referencedBy: ["ATC-NODE-001", "ATC-P2P-001"]
  },
  {
    code: "GOS-API-001",
    title: "Globus OS Public Platform API Standard",
    category: "Platform API",
    status: "Draft v1.0",
    description: "REST, gRPC und JSON-RPC Schnittstellen für externe Entwickler, Wallets, Oracles und Services.",
    primaryArtifact: "platform/api/api_gateway.c",
    referencedBy: ["GOS-ARCH-001"]
  }
];

// 7. END-TO-END TECHNISCHE KETTE (MARKDOWN REPRÄSENTATION)
export const GLOBUS_V3_MARKDOWN_SPEC = `# Globus OS / ShivaCore — Reference Architecture v3: Formal Platform Architecture
**Normatives Referenzdokument: GOS-ARCH-001**
**Status: Standardized Platform Specification v3.0**

---

## 1. Die 5 Vertikalen Domains (Architekturgrenzen)

\`\`\`
┌──────────────────────────────────────────────────────────────┐
│  D1 — EXPERIENCE DOMAIN                                     │
│  Aurora │ Web │ Mobile │ Game │ VR/AR │ CLI │ Admin          │
└──────────────────────────┬───────────────────────────────────┘
                           │ (Contract C1)
┌──────────────────────────▼───────────────────────────────────┐
│  D2 — APPLICATION DOMAIN                                    │
│  Apps │ Games │ Wallet │ DeFi │ Marketplace │ Studio         │
└──────────────────────────┬───────────────────────────────────┘
                           │ (Contract C2)
┌──────────────────────────▼───────────────────────────────────┐
│  D3 — PLATFORM DOMAIN                                       │
│  SDK │ API │ Middleware │ Services │ AI │ Blockchain         │
└──────────────────────────┬───────────────────────────────────┘
                           │ (Contract C3)
┌──────────────────────────▼───────────────────────────────────┐
│  D4 — EXECUTION DOMAIN                                      │
│  ATC VM │ WASM │ Agent Runtime │ Containers │ Plugins        │
└──────────────────────────┬───────────────────────────────────┘
                           │ (Contract C4: Host ABI vs Syscall ABI)
┌──────────────────────────▼───────────────────────────────────┐
│  D5 — SYSTEM DOMAIN                                         │
│  Syscall ABI │ Kernel │ HAL │ Drivers │ Hardware             │
└──────────────────────────────────────────────────────────────┘
\`\`\`

---

## 2. Die 7 Horizontalen Control Planes

> **Zentrales Architekturprinzip:**
> Security, Identity, Policy, Resources und Audit sind **keine austauschbaren Applikationsdienste**. Sie sind fundamentale **Plattformfunktionen**, die quer durch alle 5 Domains verlaufen.

1. **SECURITY PLANE**: Identity (DID), Authentication, Authorization, Capabilities, Encryption, Secrets Vault, Process Isolation.
2. **GOVERNANCE PLANE**: Governance, Rules, Permissions, Economics (EIP-1559 Base Fee & Gas), Compliance, Crypto-Shredding.
3. **OBSERVABILITY PLANE**: Metrics (Lockless Rings), Logs (Structured JSON/Protobuf), Distributed Tracing (W3C), Events, Health.
4. **AUDIT PLANE**: AuditTrail (Merkle Chains), Security Events, State Changes, Provenance Tracking.
5. **POLICY PLANE**: Access Policies (OPA/Rego), Resource Policies, Agent Safety Policies, Contract & Runtime Policies.
6. **RESOURCE PLANE**: CPU Core Scheduling, Memory Quotas, Storage IOPS, Network Bandwidth, GPU/NPU Allocation.
7. **CONFIGURATION PLANE**: Configuration State, Feature Flags, Runtime Parameters, Ephemeral Secrets.

---

## 3. Die 5 Architekturverträge (C1 - C5)

### Contract C1 — UI → Application
- **Schnittstelle**: UI Shell → Application API → Application Service
- **Klauseln**:
  - Authentication (Session-Token / Ephemeres JWT)
  - Authorization (Benutzerberechtigungsprüfung)
  - Request & Response Schema (JSON-Schema / Protobuf v3)
  - Error Model (Typsichere Fehlercodes mit Correlation-ID)
  - Telemetry (W3C Trace-Context-Propagation)

### Contract C2 — Application → Platform
- **Schnittstelle**: Application → SDK → Platform API → Services
- **Beispiel**: Wallet Application → Wallet SDK → Wallet API → Wallet Service
- **Klauseln**: Typsichere SDK-Bindings, API-Gateway Rate Limiting, OPA Policy Validation, Service Mesh Routing.

### Contract C3 — Platform → Runtime
- **Schnittstelle**: Service → Runtime API → ATC VM / WASM / Agent Runtime
- **Klauseln**:
  - Execution & Determinism (Deterministische Zustandsübergänge)
  - Memory Bounds (Harte lineare Speicherlimits)
  - Gas Metering (Kostenabzug pro Opcode)
  - Capabilities & Host Entitlements
  - Timeout & Asynchrone Cancellation

### Contract C4 — Runtime → Syscall ABI (Host ABI vs. Syscall ABI)
- **Kritische Grenze**: Strikte Trennung von **Syscall ABI (GOS-ABI-001)** und **Host ABI (GOS-HABI-001)**.
- **Prinzip**: Smart Contracts, WASM-Plugins und autonome Agenten greifen **niemals** direkt auf Ring-0-Kernel-Syscalls zu. Sie rufen typisierte Host-Funktionen (Traps/eCalls) auf. Das Host Interface übersetzt und sichert diese Anfragen ab.

### Contract C5 — Syscall → Kernel (ShivaCore Security Execution Pipeline)
Verbindliche 12-Stufen Validierungssequenz vor jedem physischen Ressourcenzugriff:
\`\`\`
Request
  ↓
ABI (Hardware Trap MSR_LSTAR / svc #0)
  ↓
Validation (Pointer & Bounds Check)
  ↓
Identity (Principal ID)
  ↓
Namespace (PID, Mount, Net Sandbox)
  ↓
Handle (O(1) Table Slot Verification)
  ↓
Capability (Token Integrity & Revocation Check)
  ↓
Rights (Bitmask READ | WRITE | EXEC | TRANSFER)
  ↓
Policy (Dynamic OPA / MAC Rule Check)
  ↓
Quota (Resource Allocation & Gas Counter)
  ↓
Resource Access (Kernel Ring 0 Subsystem)
  ↓
Audit & Result (Merkle Tree Log & Userland Return)
\`\`\`

---

## 4. Formale System-Boundaries

### Kernel Boundary (ShivaCore Microkernel)
- **Ring 0 enthält ausschließlich**: CPU (Scheduler, Interrupts), MEMORY (PML4 Paging, Allocator), PROCESS (Thread Management), IPC (Channels, Endpoints, Shared Memory), CAPABILITY (Handles, Rights, DAG Revocation), OBJECT (Lifetime), RESOURCE (Quotas), SECURITY (Enforcement), AUDIT (Events).
- **Strikt ausgeschlossen**: Marketplace, NFT-Handel, KI-Modell-Inferenz, RAG-Embeddings, Blockchain-Konsens, Desktop-Compositor.

### ATC VM Boundary (Execution Engine)
- **ATC VM kontrolliert**: ATCB Bytecode Parsing, Opcode Execution, Stack, VM State Registers, Linear Memory, Gas Metering, Determinismus.
- **Der Kernel kontrolliert**: Physischen Speicher, Prozesse, Threads, IPC, physische Hardware, Treiber, DMA.
- **Wichtig**: ATC VM ≠ Blockchain. Die VM ist die reine Rechenmaschine; die Blockchain ist der verteilte Konsens- und State-Layer.

### Blockchain Boundary (Distributed State)
- **Verantwortlich für**: P2P Overlay (Kademlia/Gossip), Mempool (MEV-Schutz), Konsens (PoS BFT), Block Proposal, State Trie (Modified Merkle Patricia Trie).
- Ruft die ATC VM zur Ausführung von Transaktions-Batches auf.

### AI Boundary (Controlled Principals)
- Agenten sind kontrollierte Principals:
  \`Agent → Skill → Tool → Policy → Capability → Host API → Syscall → Kernel\`
- Kein Agent besitzt ungeprüfte Superuser-Rechte oder unbeschränkten Hardware-Zugriff.

### Container Boundary (ShivaBox Sandbox)
- Erstklassige Kernel-Sandbox: Process Namespace + Network Namespace + Mount Namespace (COW Overlay) + IPC Namespace + Resource Domain (cgroup v2) + Bounded Capability Set + BPF Syscall Filter.

---

## 5. Globale Einheitliche Modelle

### GOS-RES-001: Unified Resource Model
- Eindeutige Attribute: \`resource_id\`, \`owner_principal\`, \`namespace\`, \`capability_handle\`, \`resource_type\`, \`quota_soft_limit\`, \`quota_hard_limit\`, \`current_usage\`, \`policy_rule\`, \`lifecycle_state\`.

### GOS-ID-001: Unified Identity Model
- Universeller Principal: User, Process, Thread, Service, Agent, Container, Contract, Validator, Device.
- \`Principal → Identity → Credentials → Capabilities → Rights → Policy\`.

### GOS-AUD-001: Unified Event & Audit Model
- \`event_id\`, \`timestamp_ns\`, \`actor_principal\`, \`subject\`, \`resource_id\`, \`namespace\`, \`action\`, \`result\`, \`policy_decision\`, \`capability_used\`, \`correlation_id (W3C)\`, \`metadata\`.

---

## 6. Die Endgültige Technische Kette (End-to-End)

\`\`\`
USER
 │
 ▼
AURORA / EXPERIENCE (D1)
 │
 ▼
APPLICATION (D2)
 │
 ▼
SDK (D3)
 │
 ▼
API (D3)
 │
 ▼
API GATEWAY (D3)
 │
 ▼
AUTH / IDENTITY (Control Plane)
 │
 ▼
POLICY ENGINE (Control Plane)
 │
 ▼
MIDDLEWARE (D3)
 │
 ▼
SERVICE (D3)
 │
 ├───────────────┐
 ▼               ▼
AI PLANE      ATC PLANE
 │               │
 ▼               ▼
AGENT         TRANSACTION
RUNTIME          ENGINE
 │               │
 └───────┬───────┘
         ▼
      RUNTIME (D4)
         │
 ├── ATC VM
 ├── WASM
 ├── Agent
 ├── Container
 └── Plugin
         │
         ▼
      HOST ABI (GOS-HABI-001)
         │
         ▼
     SYSCALL ABI (GOS-ABI-001)
         │
         ▼
  SYSCALL DISPATCHER (MSR_LSTAR / svc #0)
         │
         ▼
 12-STEP SECURITY PIPELINE (C5)
         │
         ▼
   SHIVACORE KERNEL (D5)
         │
 ┌───────┼────────┐
 ▼       ▼        ▼
CPU    MEMORY    IPC
 │       │        │
 └───────┼────────┘
         ▼
      HAL
         │
         ▼
     DRIVERS
         │
         ▼
     HARDWARE (Silicon x86_64, ARM64, RISC-V)
\`\`\`

---

## 7. Die 19 Formalen Standards (Normenfamilie)
1. **GOS-ARCH-001**: Globus OS Reference Architecture Standard
2. **GOS-ABI-001**: ShivaCore Syscall ABI Specification
3. **GOS-HABI-001**: Runtime Host ABI Specification
4. **GOS-IPC-001**: Inter-Process Communication (IPC) Protocol Standard
5. **GOS-CAP-001**: Capability Security & Delegation Model
6. **GOS-HND-001**: Kernel Object Handle Management Standard
7. **GOS-MEM-001**: Virtual & Physical Memory Model
8. **GOS-RES-001**: Unified Resource & Quota Accounting Model
9. **GOS-ID-001**: Unified Identity & Principal Specification
10. **GOS-POL-001**: Declarative Policy Engine Standard
11. **GOS-AUD-001**: Unified Audit Event & Provenance Standard
12. **GOS-RUN-001**: Runtime Architecture & Sandbox Standard
13. **ATC-VM-001**: ATC Virtual Machine Specification
14. **ATC-ABI-001**: ATC VM Host Interface Specification
15. **ATC-NODE-001**: A-TownChain Core Node Architecture
16. **ATC-P2P-001**: ATC Peer-to-Peer Protocol Standard
17. **ATC-STATE-001**: ATC Merkle Patricia Trie State Model
18. **ATC-CONS-001**: ATC Consensus Engine Specification
19. **GOS-API-001**: Globus OS Public Platform API Standard
`;
