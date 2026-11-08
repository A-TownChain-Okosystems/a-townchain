/**
 * Ökosystem-Sprachstrategie & Systemebenen-Matrix
 * Für A-TownChain, Globus OS, ShivaCore, ATC-VM, ATCLang, Genesis Engine und Aurora AI.
 *
 * Leitprinzip: Programmiersprachen werden nicht nach "beste Sprache" gewählt,
 * sondern nach Systemebene, Sicherheitsanforderung, Performance und Entwicklerproduktivität.
 */

export interface LanguageProfile {
  id: string;
  name: string;
  category: 'Core Systems' | 'Engine & Graphics' | 'Managed & Apps' | 'AI & Automation' | 'Web & Cloud' | 'Smart Contracts & Data';
  bestFor: string;
  strengths: string[];
  weaknesses: string[];
  systemTier: 'L0-L2 Hardware/Kernel' | 'L3-L4 Runtime/VM' | 'L5-L6 Services/Cloud' | 'L7-L8 AI/Data' | 'L9-L10 App/UI';
  ecosystemRole: 'Core 6 Primary' | 'Engine Native' | 'Specialized Bridge' | 'Platform Target' | 'Domain Specific';
  safetyRating: number; // 1-10
  performanceRating: number; // 1-10
  productivityRating: number; // 1-10
  toolingRating: number; // 1-10
  sampleUseCases: string[];
  ecosystemComponents: string[];
  badgeColor: string;
}

export const LANGUAGE_PROFILES: LanguageProfile[] = [
  {
    id: 'rust',
    name: 'Rust',
    category: 'Core Systems',
    bestFor: 'Kernel, Blockchain, VM, Runtime, Security, Cryptography',
    strengths: ['Memory Safety ohne Garbage Collector', 'Deterministische Ausführung', 'Zero-Cost Abstractions', 'Fearless Concurrency', 'Modernes Tooling (Cargo)'],
    weaknesses: ['Steile Lernkurve (Borrow Checker, Lifetimes)', 'Längere Kompilierungszeiten'],
    systemTier: 'L0-L2 Hardware/Kernel',
    ecosystemRole: 'Core 6 Primary',
    safetyRating: 10,
    performanceRating: 10,
    productivityRating: 7,
    toolingRating: 9,
    sampleUseCases: ['ShivaCore Microkernel', 'ATC-VM Register Engine', 'Blockchain Node & BFT Consensus', 'Cryptographic Primitives', 'Storage Engine'],
    ecosystemComponents: ['ShivaCore Kernel', 'Globus OS Core', 'ATC-VM', 'ATCLang Compiler', 'P2P Networking', 'Cryptography', 'Wallet Core', 'Validator Node'],
    badgeColor: 'orange',
  },
  {
    id: 'c',
    name: 'C',
    category: 'Core Systems',
    bestFor: 'Low-Level, Hardware, Kernel-Treiber, Bootloader, Firmware',
    strengths: ['Maximale Hardware-Nähe', 'Minimaler Runtime-Overhead', 'Universeller ABI-Standard', 'Direkter Register- & Portzugriff'],
    weaknesses: ['Keine automatische Memory Safety (Buffer Overflows, Use-after-free)', 'Manuelle Speicherverwaltung'],
    systemTier: 'L0-L2 Hardware/Kernel',
    ecosystemRole: 'Specialized Bridge',
    safetyRating: 3,
    performanceRating: 10,
    productivityRating: 5,
    toolingRating: 7,
    sampleUseCases: ['Stage-0 Bootloader', 'Bare-Metal CPU Init', 'Hardware-Treiber für Spezialgeräte', 'Legacy C-FFI Wrapper'],
    ecosystemComponents: ['Bootloader Stage 1/2', 'Hardware Abstraction Layer (HAL) Stubs', 'Assembly Bindings'],
    badgeColor: 'slate',
  },
  {
    id: 'cpp',
    name: 'C++',
    category: 'Engine & Graphics',
    bestFor: 'Game Engine, High-Performance, Grafik, Physik, Audio',
    strengths: ['Maximale Rechenleistung', 'Direkte GPU-APIs (Vulkan, DirectX 12, Metal)', 'Riesiges AAA-Game-Ökosystem', 'Moderne Features (C++20/23)'],
    weaknesses: ['Extrem komplexes Sprachdesign', 'Memory Safety riskanter als Rust', 'Lange Buildzeiten'],
    systemTier: 'L3-L4 Runtime/VM',
    ecosystemRole: 'Core 6 Primary',
    safetyRating: 5,
    performanceRating: 10,
    productivityRating: 6,
    toolingRating: 8,
    sampleUseCases: ['Genesis Game Engine Core', 'Vulkan Rasterizer / Raytracing Pipeline', 'Spatial Audio DSP', 'PhysX / Custom Rigid Body Physics'],
    ecosystemComponents: ['Genesis Engine Core', 'Vulkan Renderer', 'Physics Pipeline', 'Spatial Audio Engine', 'Asset Pipeline'],
    badgeColor: 'blue',
  },
  {
    id: 'csharp',
    name: 'C#',
    category: 'Managed & Apps',
    bestFor: 'Game Development, Editor Tools, Game Logic, Desktop Tools',
    strengths: ['Hohe Entwicklerproduktivität', 'Exzellentes .NET Ökosystem & Unity-Kompatibilität', 'Moderne Sprachfeatures (LINQ, Async/Await, Records)'],
    weaknesses: ['Garbage Collection (kann Frame-Drops verursachen)', 'Geringere Low-Level-Hardware-Kontrolle als Rust/C++'],
    systemTier: 'L9-L10 App/UI',
    ecosystemRole: 'Engine Native',
    safetyRating: 8,
    performanceRating: 8,
    productivityRating: 9,
    toolingRating: 10,
    sampleUseCases: ['Genesis World Editor GUI', 'Gameplay Logic & Quest Scripting', 'Asset Import & Processing Tools', 'Engine Profiler GUI'],
    ecosystemComponents: ['Genesis Editor Tools', 'Gameplay API', 'Scene Pipeline', 'Content Importer'],
    badgeColor: 'violet',
  },
  {
    id: 'python',
    name: 'Python',
    category: 'AI & Automation',
    bestFor: 'AI, ML, Automation, Data Science, Modell-Orchestrierung',
    strengths: ['De-facto Standard für Machine Learning & Deep Learning', 'Riesiges AI-Ökosystem (PyTorch, HuggingFace, NumPy)', 'Extrem schnelle Prototyping-Geschwindigkeit'],
    weaknesses: ['Langsam für Core-Runtime & rechenintensive Schleifen (GIL)', 'Dynamische Typisierung kann Laufzeitfehler begünstigen'],
    systemTier: 'L7-L8 AI/Data',
    ecosystemRole: 'Core 6 Primary',
    safetyRating: 6,
    performanceRating: 3,
    productivityRating: 10,
    toolingRating: 9,
    sampleUseCases: ['Aurora AI Modell-Training & Fine-Tuning', 'ML/RAG Pipeline & Embeddings', 'AI Agent Orchestrierung', 'System-Automatisierung & Test-Suites'],
    ecosystemComponents: ['Aurora AI Platform', 'Model Fine-Tuning Pipeline', 'Vector RAG Orchestration', 'Automated DevOps Scripts'],
    badgeColor: 'yellow',
  },
  {
    id: 'typescript',
    name: 'TypeScript',
    category: 'Web & Cloud',
    bestFor: 'Web, Frontend, APIs, Control Plane, Management Dashboards',
    strengths: ['Statische Typsicherheit über JavaScript', 'Monopol im Web-Frontend (React, Vue, WebGL)', 'Riesiges npm-Ökosystem & exzellente Dev-Experience'],
    weaknesses: ['Browser-/Node.js/V8-Runtime-Abhängigkeit', 'Kein nativer Maschinencode'],
    systemTier: 'L9-L10 App/UI',
    ecosystemRole: 'Core 6 Primary',
    safetyRating: 8,
    performanceRating: 6,
    productivityRating: 10,
    toolingRating: 10,
    sampleUseCases: ['Globus Control Plane Web UI', 'Lumino IDE & Editor Frontend', 'ATC Explorer & Wallet Dashboard', 'Node.js Management API Gateways'],
    ecosystemComponents: ['Globus Web Control Plane', 'Lumino IDE', 'ATC Explorer', 'Web Wallet', 'Governance Portal'],
    badgeColor: 'cyan',
  },
  {
    id: 'javascript',
    name: 'JavaScript',
    category: 'Web & Cloud',
    bestFor: 'Web, Node.js Scripts, Lightweight Web Utilities',
    strengths: ['Universell im Web & jedem Browser lauffähig', 'Kein Build-Schritt für einfache Skripte erforderlich'],
    weaknesses: ['Weniger strukturiert und fehleranfälliger als TypeScript', 'Schwache dynamische Typisierung'],
    systemTier: 'L9-L10 App/UI',
    ecosystemRole: 'Specialized Bridge',
    safetyRating: 5,
    performanceRating: 6,
    productivityRating: 8,
    toolingRating: 8,
    sampleUseCases: ['Legacy Browser Scripts', 'Lightweight Hook Scripts', 'Bookmarklets'],
    ecosystemComponents: ['Web Fallback Client', 'Lightweight Embeds'],
    badgeColor: 'amber',
  },
  {
    id: 'go',
    name: 'Go (Golang)',
    category: 'Web & Cloud',
    bestFor: 'Cloud, Services, Networking, Kubernetes, DevOps Tools',
    strengths: ['Einfach, minimalistisch und extrem schnell kompilierend', 'Eingebaute Concurrency (Goroutines & Channels)', 'Hervorragende Microservice-Performance'],
    weaknesses: ['Weniger Low-Level-Kontrolle über Speicher & CPU als Rust', 'Einfaches Typsystem (Generics noch relativ jung)'],
    systemTier: 'L5-L6 Services/Cloud',
    ecosystemRole: 'Core 6 Primary',
    safetyRating: 8,
    performanceRating: 8,
    productivityRating: 9,
    toolingRating: 9,
    sampleUseCases: ['Cloud API Gateways & Edge Proxies', 'Service Mesh & Distributed Tracing', 'Deployment Agents & Cluster Monitoring', 'Relayer & Bridge Daemons'],
    ecosystemComponents: ['Cloud Gateway', 'Cluster Deploy Agent', 'Telemetry Collector', 'Cross-Chain Relayer'],
    badgeColor: 'sky',
  },
  {
    id: 'atclang',
    name: 'ATCLang',
    category: 'Smart Contracts & Data',
    bestFor: 'Native A-TownChain Smart Contracts, VM Applications, System DSLs',
    strengths: ['Maßgeschneidert auf ATC-VM Registerarchitektur', 'Eingebaute Capability-Security (Objektrechte statt ACLs)', 'Deterministische Gas-Modellierung', 'Asset-orientierte Semantik'],
    weaknesses: ['Proprietäre Sprache mit noch jungem Entwickler-Ökosystem', 'Eigene Toolchain-Wartung erforderlich'],
    systemTier: 'L3-L4 Runtime/VM',
    ecosystemRole: 'Core 6 Primary',
    safetyRating: 10,
    performanceRating: 9,
    productivityRating: 8,
    toolingRating: 8,
    sampleUseCases: ['ATC Smart Contracts & Sub-Tokens', 'Decentralized Identity & Permission Grants', 'Automated Liquidity Pools', 'In-Engine Game Scripts'],
    ecosystemComponents: ['ATCLang Compiler', 'ATC-VM Bytecode Targets', 'Capability Grants', 'DeFi Contracts'],
    badgeColor: 'emerald',
  },
  {
    id: 'solidity',
    name: 'Solidity',
    category: 'Smart Contracts & Data',
    bestFor: 'Ethereum Smart Contracts, EVM Kompatibilität, DeFi Brücken',
    strengths: ['Weltweiter De-facto-Standard für DeFi/NFTs', 'Riesige Entwicklerbasis & existierende Audits', 'Breiter Tooling-Support (Hardhat, Foundry)'],
    weaknesses: ['Spezielle Sicherheitsrisiken (Reentrancy, Integer Truncation, EVM Quirks)', 'Stack-basierte EVM Limite (Stack Too Deep)'],
    systemTier: 'L3-L4 Runtime/VM',
    ecosystemRole: 'Specialized Bridge',
    safetyRating: 5,
    performanceRating: 6,
    productivityRating: 7,
    toolingRating: 8,
    sampleUseCases: ['EVM Compatibility Shard / Bridge', 'Wrapped ATC (wATC) auf Ethereum', 'Cross-Chain Token Bridges'],
    ecosystemComponents: ['EVM Compatibility Layer', 'Cross-Chain Bridge Contracts'],
    badgeColor: 'indigo',
  },
  {
    id: 'move',
    name: 'Move',
    category: 'Smart Contracts & Data',
    bestFor: 'Sichere Asset-orientierte Smart Contracts, Linear Types',
    strengths: ['First-Class Resources (Assets können weder kopiert noch versehentlich gelöscht werden)', 'Formale Verifikation eingebaut', 'Hohe Sicherheit'],
    weaknesses: ['Kleineres Entwickler-Ökosystem als Solidity', 'Eingeschränkte Dynamik'],
    systemTier: 'L3-L4 Runtime/VM',
    ecosystemRole: 'Specialized Bridge',
    safetyRating: 9,
    performanceRating: 8,
    productivityRating: 7,
    toolingRating: 7,
    sampleUseCases: ['Inspiration für ATCLang Asset-Semantik', 'High-Security Vault Contracts', 'Resource-Linearity Verifier'],
    ecosystemComponents: ['ATC Capability Reference Model', 'Asset Security Engine'],
    badgeColor: 'teal',
  },
  {
    id: 'sql',
    name: 'SQL',
    category: 'Smart Contracts & Data',
    bestFor: 'Relationale Datenbanken, Transaktionen, Indexierte Abfragen',
    strengths: ['Universeller Standard für relationale Daten', 'ACID-Transaktionssicherheit', 'Mächtige deklarative Abfragesprache'],
    weaknesses: ['Keine universelle Allzweck-Programmiersprache', 'Skaliert horizontal schwerer als NoSQL Key-Value Stores'],
    systemTier: 'L7-L8 AI/Data',
    ecosystemRole: 'Domain Specific',
    safetyRating: 9,
    performanceRating: 8,
    productivityRating: 9,
    toolingRating: 10,
    sampleUseCases: ['PostgreSQL für Benutzer-, Auth- & Verlaufsdaten', 'Marketplace & Transaktions-Indizierung', 'Audit Logs & Governance Abstimmungsregister'],
    ecosystemComponents: ['PostgreSQL Relational DB', 'Indexer Database', 'Audit Log Warehouse'],
    badgeColor: 'blue',
  },
  {
    id: 'java',
    name: 'Java',
    category: 'Managed & Apps',
    bestFor: 'Enterprise Backends, Altsystem-Integration, Finanzdienstleister',
    strengths: ['Extrem ausgereiftes, stabiles Enterprise-Ökosystem', 'Hervorragende JVM JIT-Performance', 'Große Entwicklerbasis'],
    weaknesses: ['Relativ schwergewichtig und speicherhungrig', 'Hoher Boilerplate-Aufwand'],
    systemTier: 'L5-L6 Services/Cloud',
    ecosystemRole: 'Platform Target',
    safetyRating: 7,
    performanceRating: 7,
    productivityRating: 7,
    toolingRating: 9,
    sampleUseCases: ['Enterprise Banking Connector', 'Legacy ERP Integration'],
    ecosystemComponents: ['Enterprise API Bridge'],
    badgeColor: 'rose',
  },
  {
    id: 'kotlin',
    name: 'Kotlin',
    category: 'Managed & Apps',
    bestFor: 'Android Native Apps, moderne JVM Backends',
    strengths: ['Moderne, prägnante Syntax mit Null-Safety', 'Vollständig Java-kompatibel', 'Offizieller Google-Standard für Android'],
    weaknesses: ['JVM-Laufzeitumgebung auf Servern nötig', 'Längere Build-Zeiten als Go'],
    systemTier: 'L9-L10 App/UI',
    ecosystemRole: 'Platform Target',
    safetyRating: 8,
    performanceRating: 7,
    productivityRating: 9,
    toolingRating: 9,
    sampleUseCases: ['A-Town Mobile Wallet für Android', 'Validator Mobile Alert Node', 'Android Genesis Companion App'],
    ecosystemComponents: ['Android Native Wallet', 'Android Auth Authenticator'],
    badgeColor: 'purple',
  },
  {
    id: 'swift',
    name: 'Swift',
    category: 'Managed & Apps',
    bestFor: 'Native Apple Plattformen (iOS, iPadOS, macOS, watchOS, visionOS)',
    strengths: ['Erstklassige Apple-Hardware-Beschleunigung (Metal, Apple Neural Engine)', 'Modernes ARC-Speichermanagement', 'Sehr elegantes Typsystem'],
    weaknesses: ['Fokus primär auf das Apple-Ökosystem beschränkt', 'Server-seitige Nutzung nach wie vor Nische'],
    systemTier: 'L9-L10 App/UI',
    ecosystemRole: 'Platform Target',
    safetyRating: 9,
    performanceRating: 8,
    productivityRating: 8,
    toolingRating: 9,
    sampleUseCases: ['A-Town iOS Wallet mit Secure Enclave & FaceID', 'macOS Genesis Scene Previewer', 'Apple Vision Pro Spatial Dashboard'],
    ecosystemComponents: ['iOS Native Wallet', 'macOS Native Node Monitor'],
    badgeColor: 'orange',
  },
  {
    id: 'dart',
    name: 'Dart',
    category: 'Managed & Apps',
    bestFor: 'Cross-Platform Mobile & Desktop Apps mit Flutter',
    strengths: ['Eine Codebasis für iOS, Android, Web und Desktop', 'Reaktive UI mit Flutter & Hot Reload'],
    weaknesses: ['Kleineres Sprachökosystem außerhalb von Flutter', 'Nicht native Plattform-Controls'],
    systemTier: 'L9-L10 App/UI',
    ecosystemRole: 'Platform Target',
    safetyRating: 7,
    performanceRating: 7,
    productivityRating: 9,
    toolingRating: 8,
    sampleUseCases: ['Multi-Platform Mobile Light Client', 'Schneller Prototyp für Community Wallet'],
    ecosystemComponents: ['Cross-Platform Community App'],
    badgeColor: 'teal',
  },
  {
    id: 'zig',
    name: 'Zig',
    category: 'Core Systems',
    bestFor: 'Low-Level Systemsoftware, moderne C-Alternative, Bare-Metal',
    strengths: ['Kein versteckter Kontrollfluss oder Memory Allocations', 'Hervorragender C-Cross-Compiler (`zig cc`)', 'Compile-Time Code Execution (`comptime`)'],
    weaknesses: ['Noch junge Sprache (vor 1.0 Release)', 'Kleineres Ökosystem und häufige Syntaxänderungen'],
    systemTier: 'L0-L2 Hardware/Kernel',
    ecosystemRole: 'Specialized Bridge',
    safetyRating: 7,
    performanceRating: 10,
    productivityRating: 6,
    toolingRating: 7,
    sampleUseCases: ['Experimentelle Microkernel-Module', 'Ultra-kompakte WASM-Targets', 'C-Cross-Compilation Pipeline'],
    ecosystemComponents: ['Embedded Microcontroller Stubs', 'C-Toolchain Cross-Compiler'],
    badgeColor: 'amber',
  },
  {
    id: 'lua',
    name: 'Lua',
    category: 'Engine & Graphics',
    bestFor: 'Game Scripting, App Extensions, Lightweight Embeddings',
    strengths: ['Winzige C-Runtime (wenige hundert KB)', 'Extrem schnelles Embedding in C/C++', 'Einfachste Syntax für Game Designer'],
    weaknesses: ['Wenig geeignet für komplexe Core-Systeme oder Blockchain', 'Schwache Typisierung (ohne Luau/TypeScript)'],
    systemTier: 'L9-L10 App/UI',
    ecosystemRole: 'Engine Native',
    safetyRating: 5,
    performanceRating: 7,
    productivityRating: 8,
    toolingRating: 7,
    sampleUseCases: ['Genesis Engine Entity Scripting', 'Quest- & Dialog-Trigger', 'Modding-Schnittstelle für Spieler'],
    ecosystemComponents: ['Genesis Game Modding API', 'UI Trigger Scripts'],
    badgeColor: 'blue',
  },
  {
    id: 'haskell',
    name: 'Haskell',
    category: 'Core Systems',
    bestFor: 'Formale Systeme, Verifikation, Forschung, Mathematische Modellierung',
    strengths: ['Rein funktionale Paradigmen mit extrem starkem Typsystem', 'Hervorragend für formale Beweise und Compilerbau', 'Mathematisch exakte Semantik'],
    weaknesses: ['Sehr steile Lernkurve (Monaden, Kategorientheorie)', 'Lazy Evaluation kann Memory Leaks / Space Leaks erschweren'],
    systemTier: 'L3-L4 Runtime/VM',
    ecosystemRole: 'Specialized Bridge',
    safetyRating: 9,
    performanceRating: 6,
    productivityRating: 5,
    toolingRating: 6,
    sampleUseCases: ['Formale Verifikation der ATC-VM Spezifikation', 'Mathematische Beweise des BFT-Konsens-Algorithmus'],
    ecosystemComponents: ['Formal Verification Testbed', 'Consensus Proof Suite'],
    badgeColor: 'indigo',
  },
];

/**
 * Die 6 strategischen Kernsprachen für das Ökosystem
 */
export interface StrategicPillar {
  number: number;
  name: string;
  role: string;
  domains: string[];
  motto: string;
  why: string;
  colorClass: string;
  badgeBg: string;
}

export const STRATEGIC_CORE_6: StrategicPillar[] = [
  {
    number: 1,
    name: 'Rust',
    role: 'System-Core & Sicherheit',
    domains: ['ShivaCore Microkernel', 'Globus OS Core', 'ATC-VM Register Engine', 'Blockchain Node & BFT Konsens', 'P2P Networking & Kademlia', 'Kryptographie & Zero-Knowledge', 'Storage & State Engine'],
    motto: 'Rust baut die Plattform.',
    why: 'Memory Safety ohne Garbage Collector, deterministische Ausführung und unübertroffene Performance garantieren Ausfallsicherheit für das sicherheitskritische Fundament.',
    colorClass: 'text-orange-400',
    badgeBg: 'bg-orange-500/10 border-orange-500/30 text-orange-300',
  },
  {
    number: 2,
    name: 'C++',
    role: 'Game Engine & High Performance',
    domains: ['Genesis Engine Core', 'Vulkan 3D Rasterizer & Raytracer', 'Physik- & Kollisionsberechnung', 'Spatial Audio DSP Engine', 'Asset Pipeline & Mesh Streaming'],
    motto: 'C++ baut die Engine.',
    why: 'Direkter Hardware- und GPU-Zugriff ohne Zwischenschichten sowie Kompatibilität mit dem weltweiten AAA-Grafik- und Physik-Ökosystem.',
    colorClass: 'text-blue-400',
    badgeBg: 'bg-blue-500/10 border-blue-500/30 text-blue-300',
  },
  {
    number: 3,
    name: 'TypeScript',
    role: 'Web, UI & Control Plane',
    domains: ['Globus Control Plane (Web UI)', 'Lumino IDE & Workspace', 'ATC Explorer & Analytics', 'Web Wallet & Key Custody GUI', 'Management API Gateways'],
    motto: 'TypeScript baut die Benutzeroberfläche.',
    why: 'Mächtige statische Typsicherheit kombiniert mit der unendlichen Flexibilität und Komponenten-Reichhaltigkeit moderner Web-Technologien (React, Tailwind).',
    colorClass: 'text-cyan-400',
    badgeBg: 'bg-cyan-500/10 border-cyan-500/30 text-cyan-300',
  },
  {
    number: 4,
    name: 'Python',
    role: 'AI, ML & Automatisierung',
    domains: ['Aurora AI Model Pipeline', 'Machine Learning & Fine-Tuning', 'Vector RAG & Embeddings', 'Intelligente AI-Agenten', 'DevOps Automatisierung & Data Science'],
    motto: 'Python baut die Intelligenz.',
    why: 'Das unangefochtene Welt-Ökosystem für AI und Deep Learning (PyTorch, Transformers, NumPy) für schnelle Forschung und Orchestrierung. Die Ausführung erfolgt über Rust-Runtimes.',
    colorClass: 'text-yellow-400',
    badgeBg: 'bg-yellow-500/10 border-yellow-500/30 text-yellow-300',
  },
  {
    number: 5,
    name: 'Go (Golang)',
    role: 'Cloud & Netzwerk-Infrastruktur',
    domains: ['Microservice Architecture', 'Cloud API Gateways', 'Service Mesh & Telemetrie', 'Deployment Agents & Daemons', 'Cross-Chain Relayers & Oracles'],
    motto: 'Go baut Infrastrukturservices.',
    why: 'Extrem schnelles Kompilieren, intuitive Concurrency via Goroutines und minimaler Ressourcenverbrauch in Kubernetes- und Cloud-Umgebungen.',
    colorClass: 'text-sky-400',
    badgeBg: 'bg-sky-500/10 border-sky-500/30 text-sky-300',
  },
  {
    number: 6,
    name: 'ATCLang',
    role: 'Native On-Chain & VM Programmiersprache',
    domains: ['A-TownChain Smart Contracts', 'ATC-VM Register Bytecode', 'Capability-basierte Zugriffsrechte', 'DeFi Protokolle & Sub-Tokens', 'In-Engine Scripting DSL'],
    motto: 'ATCLang baut das programmierbare ATC-Ökosystem.',
    why: 'Eigene, maßgeschneiderte Programmiersprache mit nativer Asset-Semantik, garantierter Reentrancy-Immunität und direkter Abbildung auf die Register der ATC-VM.',
    colorClass: 'text-emerald-400',
    badgeBg: 'bg-emerald-500/10 border-emerald-500/30 text-emerald-300',
  },
];

/**
 * 25-System-Komponenten Matrix mit primärer und sekundärer Sprache
 */
export interface ComponentLanguageMapping {
  systemComponent: string;
  category: 'OS & Kernel' | 'Blockchain & VM' | 'AI & Intelligence' | 'Engine & Gaming' | 'Web, Cloud & Data';
  primaryLanguage: string;
  secondaryLanguage?: string;
  reasoning: string;
}

export const COMPONENT_LANGUAGE_MATRIX: ComponentLanguageMapping[] = [
  {
    systemComponent: 'ShivaCore Kernel',
    category: 'OS & Kernel',
    primaryLanguage: 'Rust',
    secondaryLanguage: 'C / Assembly (Boot)',
    reasoning: 'Memory-Safety auf Microkernel-Ebene verhindert Privilege-Escalation Bugs; Zero-Cost-IPC für höchste Performance.',
  },
  {
    systemComponent: 'Globus OS Core',
    category: 'OS & Kernel',
    primaryLanguage: 'Rust',
    reasoning: 'Sichere Prozess-Isolation, Treiber-Management und Capability-Verwaltung in User-Space-Servern.',
  },
  {
    systemComponent: 'ATC Blockchain Node',
    category: 'Blockchain & VM',
    primaryLanguage: 'Rust',
    reasoning: 'Deterministischer BFT-Konsens, blitzschnelle Blockvalidierung und ausfallsichere P2P-Verbindungen.',
  },
  {
    systemComponent: 'ATC-VM (Virtual Machine)',
    category: 'Blockchain & VM',
    primaryLanguage: 'Rust',
    reasoning: '256-Register-Maschine mit striktem Gas-Accounting, JIT-Compiler-Option und Null-Laufzeit-Overhead.',
  },
  {
    systemComponent: 'ATCLang Compiler',
    category: 'Blockchain & VM',
    primaryLanguage: 'Rust',
    reasoning: 'Compilerbau in Rust profitiert enorm von Enums, Pattern Matching und Typgarantien bei AST-Transformationen.',
  },
  {
    systemComponent: 'ATC Bytecode Spezifikation',
    category: 'Blockchain & VM',
    primaryLanguage: 'Eigene Spezifikation',
    reasoning: 'Kompaktes, 32-Bit-ausgerichtetes binäres Registerformat für blitzschnelles Dekodieren.',
  },
  {
    systemComponent: 'Smart-Contract Runtime',
    category: 'Blockchain & VM',
    primaryLanguage: 'Rust',
    reasoning: 'Sichere Sandboxing-Umgebung ohne Speicherlecks oder unkontrollierte Heap-Allokationen.',
  },
  {
    systemComponent: 'Kryptographie & Zero-Knowledge',
    category: 'Blockchain & VM',
    primaryLanguage: 'Rust',
    reasoning: 'Timing-Attack-resistente Implementierungen von Ed25519, BLS12-381 und Blake3.',
  },
  {
    systemComponent: 'P2P Netzwerk (Kademlia / libp2p)',
    category: 'Blockchain & VM',
    primaryLanguage: 'Rust',
    secondaryLanguage: 'Go',
    reasoning: 'Asynchrones I/O mit Tokio; hohe Bandbreite bei minimalem Memory-Footprint.',
  },
  {
    systemComponent: 'Storage Core & Merkle-DB',
    category: 'Blockchain & VM',
    primaryLanguage: 'Rust',
    reasoning: 'Crash-sicherer Append-Only State Tree mit Memory-Mapped Files (mmap).',
  },
  {
    systemComponent: 'Networking Core',
    category: 'OS & Kernel',
    primaryLanguage: 'Rust',
    reasoning: 'Zero-Copy Paketverarbeitung direkt aus ShivaCore Kernel Network-Buffers.',
  },
  {
    systemComponent: 'AI Forschung & Experimente',
    category: 'AI & Intelligence',
    primaryLanguage: 'Python',
    reasoning: 'Maximale Agilität beim Ausprobieren neuer Modellarchitekturen und Hyperparameter.',
  },
  {
    systemComponent: 'AI Training & Feintuning',
    category: 'AI & Intelligence',
    primaryLanguage: 'Python',
    reasoning: 'Vollständige Anbindung an PyTorch, DeepSpeed, CUDA und HuggingFace Ecosystems.',
  },
  {
    systemComponent: 'AI Agenten & Workflows',
    category: 'AI & Intelligence',
    primaryLanguage: 'Python',
    secondaryLanguage: 'Rust',
    reasoning: 'Python für schnelle Prompt- & Tool-Ketten, Rust für deterministische Ausführung und Tool-Sandboxing.',
  },
  {
    systemComponent: 'AI Production Runtime',
    category: 'AI & Intelligence',
    primaryLanguage: 'Rust',
    reasoning: 'ONNX/Candle Inferenz in nativer Rust-Engine für 10x niedrigere Latenz ohne Python-Interpreter.',
  },
  {
    systemComponent: 'Web Frontend & IDE',
    category: 'Web, Cloud & Data',
    primaryLanguage: 'TypeScript',
    reasoning: 'React + Tailwind + WebAssembly für flüssige Bedienung im Browser ohne Compile-To-Desktop Zwang.',
  },
  {
    systemComponent: 'Web Backend & APIs',
    category: 'Web, Cloud & Data',
    primaryLanguage: 'TypeScript',
    secondaryLanguage: 'Rust',
    reasoning: 'Express/Fastify mit vollständiger Typenteilung zwischen Client und Server.',
  },
  {
    systemComponent: 'Cloud Services & Gateway',
    category: 'Web, Cloud & Data',
    primaryLanguage: 'Go',
    secondaryLanguage: 'Rust',
    reasoning: 'Leichtgewichtige Container, minimale Kaltstartzeiten und exzellente Kubernetes-Integration.',
  },
  {
    systemComponent: 'Game Engine Core',
    category: 'Engine & Gaming',
    primaryLanguage: 'C++',
    reasoning: 'Vulkan/DirectX Rendering-Loop, Cache-freundliche Data-Oriented Pipelines (ECS).',
  },
  {
    systemComponent: 'Game World Editor',
    category: 'Engine & Gaming',
    primaryLanguage: 'C++',
    secondaryLanguage: 'C#',
    reasoning: 'Performantes Rendering im Viewport mit flexiblen UI-Steuerelementen.',
  },
  {
    systemComponent: 'Gameplay Logic',
    category: 'Engine & Gaming',
    primaryLanguage: 'C#',
    secondaryLanguage: 'ATCLang',
    reasoning: 'Hohe Produktivität für Game-Designer; ATCLang für On-Chain verknüpfte Spielgegenstände.',
  },
  {
    systemComponent: 'Mobile UI (Cross-Platform)',
    category: 'Web, Cloud & Data',
    primaryLanguage: 'TypeScript (React Native)',
    secondaryLanguage: 'Dart (Flutter)',
    reasoning: 'Schnelle Erreichbarkeit für alle Mobilnutzer mit einheitlicher Codebasis.',
  },
  {
    systemComponent: 'Apple Native Client',
    category: 'Web, Cloud & Data',
    primaryLanguage: 'Swift',
    reasoning: 'Hardware-Enklaven-Zugriff für biometrische Wallet-Freigaben und Metal-Grafik.',
  },
  {
    systemComponent: 'Android Native Client',
    category: 'Web, Cloud & Data',
    primaryLanguage: 'Kotlin',
    reasoning: 'Android KeyStore und StrongBox Hardware-Integration für maximale mobile Sicherheit.',
  },
  {
    systemComponent: 'Smart Contracts (EVM Interop)',
    category: 'Blockchain & VM',
    primaryLanguage: 'Solidity',
    reasoning: 'Kompatibilität zu bestehendem DeFi-Ökosystem und Inter-Chain Bridges.',
  },
  {
    systemComponent: 'System-Datenbanken',
    category: 'Web, Cloud & Data',
    primaryLanguage: 'SQL (PostgreSQL)',
    secondaryLanguage: 'Redis / Vector DB',
    reasoning: 'ACID-Transaktionsgarantien für Off-Chain Konten, Indizes und Audit-Logs.',
  },
  {
    systemComponent: 'Automatisierung & CI/CD',
    category: 'Web, Cloud & Data',
    primaryLanguage: 'Python',
    secondaryLanguage: 'Bash / Go',
    reasoning: 'Umfangreiche Scripting-Bibliotheken für Testautomatisierung und Release-Orchestrierung.',
  },
];

/**
 * Vollständiges Markdown-Dokument zur Sprachstrategie (exportierbar in das Workspace-Root)
 */
export const LANGUAGE_STRATEGY_MARKDOWN = `# Sprachstrategie & Systemebenen-Architektur
**A-TownChain • Globus OS • ShivaCore • ATC-VM • Genesis Engine • Aurora AI**
*Dokumenten-Klasse: Normatives Architektur-Handbuch (ATC-DOC-012)*

---

## 1. Leitprinzip der Sprachauswahl

Wenn du ein großes Ökosystem wie A-TownChain / Globus OS / ShivaCore / ATC-VM / ATCLang aufbaust, sollte man Programmiersprachen nicht nach „beste Sprache“ auswählen, sondern nach:
- **Systemebene (Hardware-Nähe vs. High-Level Abstraktion)**
- **Sicherheitsanforderung (Memory Safety, Formale Verifikation, Capability Rights)**
- **Performance (Latenz, Durchsatz, Determinizismus, GC-Pausen)**
- **Entwicklerproduktivität (Ökosystem, Tooling, Iterationsgeschwindigkeit)**

---

## 2. Die strategische Kurzfassung: Die 6 Kern-Sprachen

1. **Rust** → System-Core & Sicherheit
2. **C++** → Game Engine / High Performance
3. **TypeScript** → Web / UI / Control Plane
4. **Python** → AI / ML / Automatisierung
5. **Go** → Cloud / Infrastruktur
6. **ATCLang** → Eigene Application- / Blockchain- / VM-Sprache

> **Rust** baut die Plattform.  
> **C++** baut die Engine.  
> **Python** baut die Intelligenz.  
> **TypeScript** baut die Benutzeroberfläche.  
> **Go** baut Infrastrukturservices.  
> **ATCLang** baut das programmierbare ATC-Ökosystem.

---

## 3. Globus OS & ShivaCore: Der System-Core in Rust

\`\`\`
Globus OS
   │
   ├── ShivaCore Kernel       → Rust (Memory Safety ohne GC, Zero-Cost IPC)
   ├── ATC Blockchain Core    → Rust (BFT Consensus, P2P, Storage)
   ├── ATC-VM                 → Rust (256-Register Execution Engine)
   ├── ATCLang Compiler       → Rust (AST, Typechecker, Bytecode Emitter)
   ├── P2P Network            → Rust (libp2p / Tokio Async)
   ├── Crypto Engine          → Rust (Ed25519, BLS12-381, Blake3)
   └── Storage Engine         → Rust (Append-Only Merkle Tree)
\`\`\`

### Warum Rust?
Memory Safety ohne Garbage Collector, deterministische Ausführung und unübertroffene Performance machen Rust zum unverzichtbaren Fundament aller sicherheitskritischen Kern-Komponenten.

---

## 4. Genesis Engine: C++ & C#

\`\`\`
Genesis Engine
   │
   ├── Core              C++  (Data-Oriented Design, Memory Pools)
   ├── Renderer          C++  (Vulkan 1.3, Raytracing, Mesh Shaders)
   ├── Physics           C++  (Rigid Body, Fluids, Spatial Hashing)
   ├── Audio             C++  (Spatial 3D Audio, DSP Synthesizer)
   ├── Animation         C++  (Skeletal Blending, Inverse Kinematics)
   ├── Asset System      C++  (Virtual Geometry, Streaming)
   ├── Networking        C++  (UDP Packet Reliability Layer)
   │
   ├── Editor            C++ / C# (.NET Tooling & High Productivity)
   ├── Gameplay API      C#   (Type-Safe Object Hierarchy)
   └── Scripting         ATCLang / Lua (On-Chain Assets & Modding)
\`\`\`

---

## 5. Aurora AI: Python Orchestrierung & Rust Execution

\`\`\`
Aurora AI Platform
   │
   ├── Model Integration      → Python (PyTorch, HuggingFace)
   ├── ML Pipeline             → Python (Data Preprocessing, Tokenizer)
   ├── Training & Fine-Tuning  → Python (Distributed GPU Clusters)
   ├── AI Research             → Python (Jupyter, Prototyping)
   │
   └── Production Runtime
          └── Rust (Candle / ONNX Engine für Sub-Millisekunden-Inferenz)
\`\`\`

*Wichtig*: Python muss nicht deine Produktionsruntime sein. Python übernimmt die AI-Orchestrierung und Forschung, während performancekritische Komponenten in Rust laufen.

---

## 6. Globus Control Plane: Web in TypeScript

\`\`\`
Globus Control Plane
   │
   ├── Web UI                 → TypeScript (React, Tailwind CSS)
   ├── Lumino IDE             → TypeScript (WebAssembly VM Preview)
   ├── Admin Console          → TypeScript (Role-Based Access)
   ├── Wallet UI              → TypeScript (Private Key Security)
   ├── Marketplace & NFTs     → TypeScript (Responsive Grid)
   ├── Blockchain Explorer    → TypeScript (Live Block Feeds)
   └── API Gateway            → TypeScript / Go
\`\`\`

---

## 7. Cloud Infrastruktur: Go

\`\`\`
Cloud Infrastructure
   │
   ├── API Gateway            → Go (Minimal Footprint, High Concurrency)
   ├── Service Mesh           → Go (Envoy Control Plane)
   ├── Deployment Agent       → Go (Kubernetes CRDs, Daemons)
   ├── Monitoring & Metrics   → Go (Prometheus Exporters)
   └── Relayers & Bridges     → Go (Cross-Chain RPC)
\`\`\`

*Entscheidungshilfe*:
- Wenn Performance, Memory Safety und Low-Level Kontrolle entscheidend sind → **Rust**.
- Wenn schnelle Entwicklung einfacher, robuster Netzwerkdienste im Vordergrund steht → **Go**.

---

## 8. Smart Contracts: ATCLang vs. Solidity / Move

\`\`\`
Native ATC Architektur:
ATCLang
   ↓ (ATC Compiler in Rust)
ATC Bytecode (32-Bit Register Format)
   ↓
ATC-VM (Registerbasierte Sandbox mit Gas-Metering)
   ↓
ATC Blockchain (BFT Consensus State)
\`\`\`

- **Solidity**: Wird über ein dediziertes EVM-Kompatibilitäts-Subnetz für DeFi-Brücken unterstützt.
- **Move**: Diente als architektonische Inspiration für die lineare Ressourcen-Semantik in ATCLang.

---

## 9. Datenhaltung & State Hierarchy

\`\`\`
Datenarchitektur
   │
   ├── PostgreSQL (SQL)  → Relationale Daten: Nutzer, Konten, Transaktions-Index, Audit Logs
   ├── Redis             → Hochgeschwindigkeits-Cache für Session-Tokens & Mempool-Vorschau
   ├── Object Storage    → Große Binärdaten: Blobs, Audio-Samples, Game-Meshes, Snapshots
   ├── Vector DB         → Aurora AI: Embeddings, Semantic Search, RAG Knowledge Base
   ├── Graph DB          → Social Graph, On-Chain Transaktionsverflechtungen & AML Analyse
   └── ATC Blockchain    → Unveränderlicher, kryptografisch verifizierbarer State
\`\`\`
`;
