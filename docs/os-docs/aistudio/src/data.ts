// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export const TARGET_STATE = {
  mainSystems: { current: 48, vision: 48 },
  subsystems: { current: "150+", vision: "250-400" },
  coreComponents: { current: "10", vision: "10+" },
  services: { current: "3,000", vision: "10,000+" }
};

export const LAYERS = [
  // 1. Core OS & Kernel
  {
    id: 1,
    name: "Hardware Foundation",
    category: "Core OS & Kernel",
    purpose: "Hardware Kernel Layer",
    subLayers: ["Low-Level Bootloader", "IRQ Routing Engine", "MMU & Memory Controller", "Paging & Swap Engine", "Hardware Abstraction Layer (HAL)", "Native Device Drivers", "Direct DMA Controller", "BIOS/UEFI Emulation"],
    iconName: "Server"
  },
  {
    id: 2,
    name: "Kernel Core",
    category: "Core OS & Kernel",
    purpose: "ATC Kernel",
    subLayers: ["Task Scheduler (Preemptive)", "Memory Mapping Manager", "Process Execution Engine", "Thread Dispatcher", "System Call Interface", "Kernel Space Security Manager", "Inter-Process Communication (IPC)", "Driver Orchestration"],
    iconName: "Cpu"
  },
  {
    id: 3,
    name: "Runtime Layer",
    category: "Core OS & Kernel",
    purpose: "Runtime Engine",
    subLayers: ["Process Runtime Environment", "Dynamic Linker", "Garbage Collection Engine", "Resource Allocator", "Runtime Scheduler", "Memory Sandbox Manager", "Thread Pool Manager"],
    iconName: "Layers"
  },
  {
    id: 4,
    name: "Filesystem Ecosystem",
    category: "Core OS & Kernel",
    purpose: "Eigene Dateiformate",
    subLayers: ["Distributed File Allocation Table", ".atc (Smart Contract Binaries)", ".atvm (Virtual Machine States)", ".aes (Encrypted Storages)", ".ata (System Archives)", ".atsdb (State Databases)", ".atn (Network Configs)", ".atk (Keyfiles)", ".atpkg (Packages)"],
    iconName: "Database"
  },
  {
    id: 5,
    name: "Production Kernel Integration",
    category: "Core OS & Kernel",
    purpose: "PKIL - Finale Integrationsschicht",
    subLayers: ["Boot-to-Blockchain Bootstrap", "Unified Process Translation", "End-to-End Orchestrator", "Hardware-to-State Mapper", "Live Kernel Hot-Swapping"],
    iconName: "CheckCircle"
  },

  // 2. Virtualization & Compatibility
  {
    id: 6,
    name: "Compiler + IR Layer",
    category: "Virtualization & Compatibility",
    purpose: "Compiler Infrastructure",
    subLayers: ["Lexical Parser", "Abstract Syntax Tree (AST)", "Static Analyzer", "Intermediate Representation (IR)", "AOT/JIT Optimizer", "Bytegraph Compiler", "WebAssembly (Wasm) Backend", "Cross-Platform Target Emitter"],
    iconName: "Code"
  },
  {
    id: 7,
    name: "VM Layer",
    category: "Virtualization & Compatibility",
    purpose: "ATC VM",
    subLayers: ["Deterministic Execution Engine", "Bytegraph Processor", "Sandbox Isolation Boundary", "Gas/Resource Metering", "State Tree Access API", "Time & Randomness Oracle", "ZKP Execution Hooks"],
    iconName: "Box"
  },
  {
    id: 8,
    name: "Bridge Compatibility Layer (BCL)",
    category: "Virtualization & Compatibility",
    purpose: "Universal Interface Normalization",
    subLayers: ["Interface Normalization Runtime", "Schema Translation Engine", "OS-Level Adapter Mappings", "Memory Space Emulator", "File System Path Translator", "Network Socket Proxy"],
    iconName: "Link"
  },
  {
    id: 9,
    name: "Microsoft Windows Layer",
    category: "Virtualization & Compatibility",
    purpose: "Windows Desktop Environment",
    subLayers: ["Win32 API Bridge", "NT Kernel Translation", "DirectX to Metal/Vulkan Emulation", "Registry Virtualization", "Windows NTFS Translation", "PE Binary Loader", "COM/DCOM Broker"],
    iconName: "Monitor"
  },
  {
    id: 10,
    name: "Linux Layer",
    category: "Virtualization & Compatibility",
    purpose: "Linux Native Environment",
    subLayers: ["POSIX Compliance Engine", "Linux Package Manager Bridge (apt/pacman)", "ELF Binary Loader", "Syscall Translation (SysV/Linux)", "cgroups & Namespaces Emulation", "X11/Wayland Display Server Translator"],
    iconName: "Terminal"
  },
  {
    id: 11,
    name: "macOS Layer",
    category: "Virtualization & Compatibility",
    purpose: "macOS Application Support",
    subLayers: ["Darwin Kernel Translation", "Metal / CoreGraphics Emulation", "Objective-C / Swift Runtime Bindings", "Mach-O Binary Loader", "App Sandbox Bridge", "dmg & APFS Filesystem Support"],
    iconName: "MonitorSmartphone"
  },
  {
    id: 12,
    name: "Mobile OS Layer",
    category: "Virtualization & Compatibility",
    purpose: "Android & iOS App Compatibility",
    subLayers: ["APK / AAB Execution Engine", "Android Framework Emulation (ART)", "iOS App Sandbox", "Touch UI Translation to Desktop", "Mobile Sensor Emulation (Gyro, GPS)", "ARM to x86 Translation Layer"],
    iconName: "Smartphone"
  },

  // 3. Blockchain Core & Network
  {
    id: 13,
    name: "Network Layer",
    category: "Blockchain Core & Network",
    purpose: "ATC Transport Protocol (ATP)",
    subLayers: ["P2P Peer Discovery (DHT)", "Gossip Protocol Engine", "Block Propagation Router", "Fast State Sync", "Consensus Messaging Channel", "Bandwidth Optimization Profiler", "NAT Traversal & UPnP"],
    iconName: "Network"
  },
  {
    id: 14,
    name: "Storage Layer",
    category: "Blockchain Core & Network",
    purpose: "State Storage Engine",
    subLayers: ["Merkle Patricia Trie / Verkle Trees", "State Snapshot Engine", "Write-Ahead Journaling", "Cold Storage Archiver", "Distributed IPFS-like Blobs", "LevelDB/RocksDB Subsystem"],
    iconName: "HardDrive"
  },
  {
    id: 15,
    name: "Consensus Layer",
    category: "Blockchain Core & Network",
    purpose: "Consensus Finality Engine",
    subLayers: ["PoW/PoS Hybrid Router", "Validator Selection & Shuffling", "Block Proposal Engine", "Voting & Committing Subsystem", "Instant Finality Checkpoints", "Fork Resolution Logic", "Proof of AI Execution"],
    iconName: "CheckSquare"
  },
  {
    id: 16,
    name: "Transaction Layer",
    category: "Blockchain Core & Network",
    purpose: "Transaction Lifecycle (TTL)",
    subLayers: ["Transaction Formatting", "Signature Validation (ECDSA/BLS)", "High-Throughput Mempool", "Fee & Gas Calculator", "MEV Protection Protocol", "Transaction Execution Router", "Receipt Generation"],
    iconName: "Activity"
  },
  {
    id: 17,
    name: "Smart Contract Layer",
    category: "Blockchain Core & Network",
    purpose: "Native ATC Contracts",
    subLayers: ["ATS-1 Factory", "ATS-20 Fungible Handlers", "ATS-721/1155 NFT Engines", "Governance Logic Interpreters", "AI Smart Contract Execution", "Zero-Knowledge Logic", "On-Chain Oracles"],
    iconName: "FileText"
  },

  // 4. Security & Reliability
  {
    id: 18,
    name: "Runtime Security Layer",
    category: "Security & Reliability",
    purpose: "Security System",
    subLayers: ["Process Sandbox Enforcement", "Binary Signature Verification", "Hardware Root of Trust Mapping", "Memory Isolation (ASLR/DEP)", "Call Stack Verification", "Real-Time Threat Prevention"],
    iconName: "ShieldCheck"
  },
  {
    id: 19,
    name: "Network Security Layer",
    category: "Security & Reliability",
    purpose: "Advanced Network Integrity",
    subLayers: ["Zero Trust Sandbox Routers", "Native DDoS Protection", "Global Threat Intelligence Stream", "Sybil Attack Prevention", "BGP Hijack Mitigation", "Encrypted Tunneling (E2E)"],
    iconName: "Shield"
  },
  {
    id: 20,
    name: "Self-Healing Mainnet",
    category: "Security & Reliability",
    purpose: "Mainnet Self-Healing Mechanisms",
    subLayers: ["Auto Node Recovery", "Dynamic Auto Scaling Partitioning", "AI Incident Detection", "Runtime Patching without Reboot", "Corrupted State Rollbacks", "Validator Penalty Enforcement (Slashing)"],
    iconName: "Shield"
  },
  {
    id: 21,
    name: "Mainnet Bootstrap Layer",
    category: "Security & Reliability",
    purpose: "Genesis & Recovery",
    subLayers: ["Genesis Block Creation", "Initial Validator Synchronization", "Network Parameter Initialization", "Fallback Chain Activation", "Emergency State Snapshot Loaders"],
    iconName: "RefreshCw"
  },

  // 5. DevOps, Standards & Tooling
  {
    id: 22,
    name: "Dev Tooling",
    category: "DevOps, Standards & Tooling",
    purpose: "Software Development Kit (SDK)",
    subLayers: ["A-Town Developer SDKs", "ATC Command Line Interface (CLI)", "Local Testnet Spin-Up", "Deterministic Debugger", "Gas & Perf Profiling", "Smart Contract Auditor", "IDE Plugins"],
    iconName: "Terminal"
  },
  {
    id: 23,
    name: "Package Layer",
    category: "DevOps, Standards & Tooling",
    purpose: "ATC Package Manager",
    subLayers: ["Manifest Resolver", "Binary Distribution Network", "Metadata Registry", "Dependency Tree Mapping", "Censorship-Resistant Repos"],
    iconName: "Box"
  },
  {
    id: 24,
    name: "ATOS-STANDARDS Layer",
    category: "DevOps, Standards & Tooling",
    purpose: "OS Ecosystem Standards",
    subLayers: ["ATC-Lang Syntax & Semantics", "Component UI Specifications", "System API Interoperability", "Data Formatting Guidelines", "ATS Tokens Standard Sub-Layer", "Security Audit Checklists"],
    iconName: "FileCheck"
  },
  {
    id: 25,
    name: "Kubernetes Deployment Layer",
    category: "DevOps, Standards & Tooling",
    purpose: "Cluster System",
    subLayers: ["Kube-API Control Plane", "Helm Chart Managers", "Ingress/Egress Service Mesh", "Automated Pod Scaling", "Disaster Recovery Volumes", "Container Lifecycle Hooks"],
    iconName: "Server"
  },
  {
    id: 26,
    name: "Monitoring Layer",
    category: "DevOps, Standards & Tooling",
    purpose: "System Observability",
    subLayers: ["Time-Series Metrics (Prometheus)", "Distributed Log Aggregation", "Alerting & Pager System", "Node Hardware Monitoring", "App-Level Tracing", "Bandwidth Usage Meters"],
    iconName: "Activity"
  },
  {
    id: 27,
    name: "API Layer",
    category: "DevOps, Standards & Tooling",
    purpose: "Node Connectivity",
    subLayers: ["JSON-RPC Endpoints", "RESTful Interfaces", "WebSocket Streaming Subsystem", "gRPC Runtime API", "GraphQL Access Layer", "Direct Node C APIs"],
    iconName: "Link"
  },
  {
    id: 28,
    name: "API Orchestrator Layer",
    category: "DevOps, Standards & Tooling",
    purpose: "Centralized Microservices Routing",
    subLayers: ["Multi-Region API Gateway", "East-West Service Mesh", "GraphQL Data Federation", "Layer 7 Load Balancing", "Dynamic Rate Limiting", "Circuit Breakers"],
    iconName: "Share2"
  },

  // 6. Identity, Governance & Economy
  {
    id: 29,
    name: "Identity Layer",
    category: "Identity, Governance & Economy",
    purpose: "Decentralized Identifier (DID)",
    subLayers: ["DID Document Resolver", "Wallet-to-Identity Mapping", "Single Sign-On (Web3 SSO)", "On-Chain KYC Framework", "Zero-Knowledge Identity Verification", "Biometric Hardware Subsystem"],
    iconName: "Key"
  },
  {
    id: 30,
    name: "Nutzer Profil Layer",
    category: "Identity, Governance & Economy",
    purpose: "User Preferences & Settings",
    subLayers: ["Global User Preferences", "Access Control Roles", "Cross-Platform Avatar Data", "Privacy & Data Sharing Toggles", "Historical Activity Logging", "Reputation & Social Scoring"],
    iconName: "User"
  },
  {
    id: 31,
    name: "Wallet Layer",
    category: "Identity, Governance & Economy",
    purpose: "Storage & Transact",
    subLayers: ["Hardware Enclave Key Management", "Multi-Party Computation (MPC)", "Multi-Signature Approvals", "Social Recovery Mechanisms", "Enterprise Treasury Workflows", "UTXO & Account State Tracker"],
    iconName: "Briefcase"
  },
  {
    id: 32,
    name: "Governance Layer",
    category: "Identity, Governance & Economy",
    purpose: "DAO operations",
    subLayers: ["On-Chain Proposal System", "Liquid Democracy Engine", "Treasury Allocation Voting", "Protocol Upgrade Execution", "Delegation & Proxy Voting", "Veto & Emergency Stops"],
    iconName: "Building"
  },
  {
    id: 33,
    name: "Economic Layer",
    category: "Identity, Governance & Economy",
    purpose: "State Machine Economy",
    subLayers: ["Dynamic Emissions Model", "Algorithmic Fee Adjustment", "Central Treasury Reserve", "Token Burn & Buyback Controller", "Halving & Inflation Rules", "Staking APY Calculator"],
    iconName: "TrendingUp"
  },
  {
    id: 34,
    name: "Tax & Accounting Layer",
    category: "Identity, Governance & Economy",
    purpose: "Fiskales System",
    subLayers: ["Real-Time Wallet Accounting", "Cross-Border Tax Calculator", "Fiat-to-Crypto Valuation Engine", "PDF/CSV Export Tooling", "Automated Transaction Classification", "Audit Trail Generator"],
    iconName: "BookOpen"
  },
  {
    id: 35,
    name: "Enterprise Layer",
    category: "Identity, Governance & Economy",
    purpose: "Corporate Integrations",
    subLayers: ["ISO Compliance Checker", "Automated Audit Reporter", "Legal Smart Contract Wrapper", "Regulator Access API", "Corporate Permissioning (RBAC)", "Private Subnet Provisioning"],
    iconName: "Briefcase"
  },
  {
    id: 36,
    name: "Global Operations Control Plane",
    category: "Identity, Governance & Economy",
    purpose: "Ecosystem Command",
    subLayers: ["Mainnet Visualization Dashboard", "Governance Pulse Monitor", "Global Network Health Map", "AI Macro Monitoring", "Incident Response Center", "Supply Chain Tracker"],
    iconName: "Globe"
  },
  {
    id: 37,
    name: "Global Autonomous Economy",
    category: "Identity, Governance & Economy",
    purpose: "Macro Economic Management",
    subLayers: ["AI Economic Market Monitoring", "Dynamic Liquidity Policy Engine", "Cross-Layer Treasury Optimization", "Predictive Reward Adjustments", "Systemic Risk Circuit Breakers"],
    iconName: "TrendingUp"
  },

  // 7. AI & Autonomous Systems
  {
    id: 38,
    name: "KI Inference Layer",
    category: "AI & Autonomous Systems",
    purpose: "Core AI Capabilities",
    subLayers: ["LLM Model Inference Routing", "Real-Time Data NLP Pipeline", "Autonomous Multi-Agent Swarms", "Long-Term Context Memory Engine", "Predictive Analytics Subsystem", "Federated Learning Controller"],
    iconName: "BrainCircuit"
  },
  {
    id: 39,
    name: "ATS Autonomous AI Engine",
    category: "AI & Autonomous Systems",
    purpose: "Chain Monitoring & Meta-Rules",
    subLayers: ["Runtime Log Analyzer", "Governance Optimization Recommendations", "Anomalous Network Detection", "Smart Contract Auto-Auditor", "Automated Policy Enforcer"],
    iconName: "Brain"
  },
  {
    id: 40,
    name: "Franchise Factory Layer",
    category: "AI & Autonomous Systems",
    purpose: "Content Generation AI",
    subLayers: ["Game Flow & Logic Generation", "3D/2D Asset Generation", "VFX & Shader Creator AI", "Audio & Music Synthesizer", "Narrative & Dialog System", "Automated QA & Playtesting"],
    iconName: "Cpu"
  },
  {
    id: 41,
    name: "Franchise Factory Studio Layer",
    category: "AI & Autonomous Systems",
    purpose: "Local Creator Tooling",
    subLayers: ["Offline Content Databases", "Asset Management System (MAM)", "High-Speed Media Storage Bridge", "Cinematic Timeline Editor", "Interactive World Builder", "Prompt-to-Code Injectors"],
    iconName: "MonitorPlay"
  },

  // 8. Application & User Space
  {
    id: 42,
    name: "Bridge Layer",
    category: "Application & User Space",
    purpose: "Cross-Chain Relays",
    subLayers: ["Ethereum Fast-Relay", "Solana Wormhole Link", "Polkadot Parachain Adapter", "Cosmos IBC Gateway", "Bitcoin Multi-Sig Escrow", "Atomic Swap Channels"],
    iconName: "Globe"
  },
  {
    id: 43,
    name: "GateToHell Web Browser Layer",
    category: "Application & User Space",
    purpose: "Native Web3 Explorer",
    subLayers: ["Chromium/WebKit Web3 Rendering Engine", "Native Hardware Wallet Integration", "Anti-Tracking Privacy Shields", "Decentralized DApp Store", "IPFS/ENS Native Resolver", "Censorship Circumvention Router"],
    iconName: "Globe"
  },
  {
    id: 44,
    name: "App Layer",
    category: "Application & User Space",
    purpose: "Decentralized Apps",
    subLayers: ["DApp Hosting Environments", "Standardized SC Interfaces", "Progressive Web App (PWA) Wrappers", "Installed App Native Bindings", "Global App Registry", "Push Notification Relays"],
    iconName: "Layout"
  },
  {
    id: 45,
    name: "DeFi Layer",
    category: "Application & User Space",
    purpose: "Decentralized Finance",
    subLayers: ["Automated Market Makers (DEX)", "Over-collateralized Lending", "Unsecured Flash Loans", "Yield Farming Aggregators", "Algorithmic Stablecoin Controllers", "Dark Pool Liquidity"],
    iconName: "BarChart3"
  },
  {
    id: 46,
    name: "Marktplatz Layer",
    category: "Application & User Space",
    purpose: "Digital Asset Trading",
    subLayers: ["NFT Auction & Direct Sales", "Real-World Asset (RWA) Exchange", "High-Freq Order Book matching", "Fiat On/Off Payment Gateways", "Escrow & Dispute Resolution", "Royalty Enforcement Logic"],
    iconName: "ShoppingCart"
  },
  {
    id: 47,
    name: "Soziale Media Layer",
    category: "Application & User Space",
    purpose: "Decentralized Social Network",
    subLayers: ["Decentralized User Feeds", "Follower Social Graph (AT-Graph)", "Community-Driven Moderation", "E2E Encrypted Messaging", "Creator Tipping & Economy", "Token-Gated Communities"],
    iconName: "Share2"
  },
  {
    id: 48,
    name: "GameFi Layer",
    category: "Application & User Space",
    purpose: "Gaming Economies",
    subLayers: ["In-Game Tokenomics Models", "Gamified DeFi Modules", "Cross-Game NFT Equipping", "Tournament Prize Smart Contracts", "Play-and-Earn Yield Distribution", "Anti-Cheat Consensus Hooks"],
    iconName: "Gamepad2"
  }
];

export const LONG_TERM_VISION = LAYERS.map(l => l.name);
