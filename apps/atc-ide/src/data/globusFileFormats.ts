/**
 * Globus File Format Architecture (GFFA) & Globus Native File Format Standard (GNFF) v1.0
 * Für Globus OS, ShivaCore, ATC-VM, ATCLang, Genesis Engine und Aurora AI.
 *
 * Dokumenten-ID: ATC-DOC-013
 */

export type FormatDomain = 'System' | 'Application' | 'Blockchain' | 'AI & Agents' | 'Security' | 'Configuration';

export interface HeaderBlockField {
  offset: string;
  bytes: number;
  fieldName: string;
  type: string;
  description: string;
}

export interface GffFormatDefinition {
  extension: string;
  name: string;
  domain: FormatDomain;
  magicString: string;
  magicHex: string;
  mimeType: string;
  targetConsumers: string[];
  description: string;
  sampleFileName: string;
  containerStructure: string[];
  defaultCapabilities: string[];
  verificationSteps: string[];
  badgeColor: string;
}

export const GFF_HEADER_FIELDS: HeaderBlockField[] = [
  { offset: '0x00', bytes: 4, fieldName: 'Magic Bytes', type: 'char[4]', description: "Fest 'GLOB' (0x47 0x4C 0x4F 0x42) zur globalen Objekterkennung" },
  { offset: '0x04', bytes: 4, fieldName: 'Format Tag', type: 'char[4]', description: "Sub-Typ z.B. 'GEXE', 'GDLL', 'GSYS', 'GDRV', 'GAPP', 'ATCB'" },
  { offset: '0x08', bytes: 2, fieldName: 'GNFF Version', type: 'u16', description: 'Major/Minor Version (v1.0 = 0x0100)' },
  { offset: '0x0A', bytes: 2, fieldName: 'Header Flags', type: 'u16 (Bitmask)', description: '0x01: Signed, 0x02: Encrypted, 0x04: Zstd Compressed, 0x08: Strict Sandbox' },
  { offset: '0x0C', bytes: 2, fieldName: 'Target Arch', type: 'u16', description: '0x01: x86_64, 0x02: aarch64, 0x03: riscv64, 0x04: atc-vm-reg256, 0x05: universal' },
  { offset: '0x0E', bytes: 2, fieldName: 'ABI Version', type: 'u16', description: 'Globus System ABI (v1 = 0x0001)' },
  { offset: '0x10', bytes: 8, fieldName: 'Entry Point Offset', type: 'u64', description: 'Byte-Offset zum ersten ausführbaren Instruktions-Vektor' },
  { offset: '0x18', bytes: 8, fieldName: 'Payload Size', type: 'u64', description: 'Nettogröße des Code- bzw. Daten-Payloads' },
  { offset: '0x20', bytes: 8, fieldName: 'Manifest Offset', type: 'u64', description: 'Offset zur eingebetteten .gmanifest Deskriptor-Tabelle' },
  { offset: '0x28', bytes: 8, fieldName: 'Capability Mask Offset', type: 'u64', description: 'Offset zur Objektrechte- und Berechtigungs-Tabelle' },
  { offset: '0x30', bytes: 8, fieldName: 'Signature Block Offset', type: 'u64', description: 'Offset zum kryptographischen Signatur-Block (Ed25519/Dilithium)' },
  { offset: '0x38', bytes: 8, fieldName: 'Blake3 Hash Trunk', type: 'u64', description: 'Erste 64-Bit des Blake3-Prüfsummen-Hashes zur Schnellvalidierung' },
];

export const GLOBUS_FILE_FORMATS: GffFormatDefinition[] = [
  // 1. SYSTEM DOMAIN
  {
    extension: '.gexe',
    name: 'Globus Executable',
    domain: 'System',
    magicString: 'GLOBGEXE',
    magicHex: '47 4C 4F 42 47 45 58 45',
    mimeType: 'application/x-globus-executable',
    targetConsumers: ['ShivaCore Process Loader', 'Globus Runtime Manager'],
    description: 'Sicherer nativer Container für ausführbare Globus OS Programme mit striktem Capability-Bounding, Signaturprüfung und ABI-Validierung.',
    sampleFileName: 'shiva-shell.gexe',
    containerStructure: ['GFF-Header (64B)', 'Capability Descriptors', 'Relocation Table', 'Code Segment (.text)', 'Read-Only Data (.rodata)', 'Data Segment (.data)', 'Embedded .gmanifest', 'Ed25519 Signature Block'],
    defaultCapabilities: ['memory.alloc', 'thread.spawn', 'ipc.listen'],
    verificationSteps: ['Blake3 Integrity Check', 'Publisher Ed25519 Signature', 'Capability Bounds Verification', 'ABI & Architecture Match', 'Static Instruction Safety Scan'],
    badgeColor: 'orange',
  },
  {
    extension: '.gdll',
    name: 'Globus Dynamic Library',
    domain: 'System',
    magicString: 'GLOBGDLL',
    magicHex: '47 4C 4F 42 47 44 4C 4C',
    mimeType: 'application/x-globus-dynamic-library',
    targetConsumers: ['ShivaCore Dynamic Linker (gld)', 'GEXE Runtimes'],
    description: 'Dynamische shared library für ShivaCore. Unterstützt typsichere C-ABI und Rust-v0 ABI mit feingranularer Funktions-Isolation.',
    sampleFileName: 'crypto.gdll',
    containerStructure: ['GFF-Header (64B)', 'Export Symbol Table', 'Import Symbol Table', 'Code Segment', 'Relocations', 'Interface Definition (IDL) Hash'],
    defaultCapabilities: ['link.export', 'memory.read'],
    verificationSteps: ['ABI Symbol Hash Match', 'Integrity Check', 'Signature Check'],
    badgeColor: 'blue',
  },
  {
    extension: '.gsys',
    name: 'Globus System Module',
    domain: 'System',
    magicString: 'GLOBGSYS',
    magicHex: '47 4C 4F 42 47 53 59 53',
    mimeType: 'application/x-globus-system-module',
    targetConsumers: ['ShivaCore Microkernel', 'Kernel Service Registry'],
    description: 'Privilegierte Systemdienste und Microkernel-Extensions (z.B. Scheduler, Virtual Memory Server, IPC Router).',
    sampleFileName: 'scheduler.gsys',
    containerStructure: ['GFF-Header (64B)', 'Kernel Ring-0/Ring-1 Capabilities', 'Direct Memory Mappings', 'Privileged Syscall Handlers', 'Hardware Root Signatures'],
    defaultCapabilities: ['kernel.sysenter', 'memory.phys_map', 'cpu.affinity', 'interrupt.manage'],
    verificationSteps: ['Hardware Root of Trust Key (TPM 2.0)', 'Microkernel Security Ring Verifier', 'Formal Specification Audit Check'],
    badgeColor: 'red',
  },
  {
    extension: '.gdrv',
    name: 'Globus Driver',
    domain: 'System',
    magicString: 'GLOBGDRV',
    magicHex: '47 4C 4F 42 47 44 52 56',
    mimeType: 'application/x-globus-driver',
    targetConsumers: ['ShivaCore HAL (Hardware Abstraction Layer)', 'Device Tree Manager'],
    description: 'Hardware-Treiber für ShivaCore. Läuft im isolierten User-Space-Treibermodell mit IOMMU-Schutz ohne Kernel-Panics auszulösen.',
    sampleFileName: 'gpu-vulkan.gdrv',
    containerStructure: ['GFF-Header (64B)', 'Hardware IDs (PCI/USB/ACPI Match List)', 'IOMMU Page Table Maps', 'Interrupt Descriptors', 'Driver Entry Points'],
    defaultCapabilities: ['hardware.io_port', 'hardware.dma_channel', 'hardware.irq_bind'],
    verificationSteps: ['Device Whitelist Match', 'IOMMU Safe Buffer Scan', 'OEM Driver Certificate'],
    badgeColor: 'purple',
  },
  {
    extension: '.gmod',
    name: 'Globus Module',
    domain: 'System',
    magicString: 'GLOBGMOD',
    magicHex: '47 4C 4F 42 47 4D 4F 44',
    mimeType: 'application/x-globus-module',
    targetConsumers: ['Globus Subsystem Manager', 'Pluggable Architecture Services'],
    description: 'Pluggable Subsystem-Erweiterung für Dateisysteme, Netzwerkprotokolle oder Krypto-Module.',
    sampleFileName: 'zfs-fs.gmod',
    containerStructure: ['GFF-Header (64B)', 'Hook Registrations', 'Service Protocols', 'Subsystem Payload'],
    defaultCapabilities: ['vfs.register_fs', 'net.protocol_register'],
    verificationSteps: ['Module Namespace Collision Check', 'System Policy Compliance'],
    badgeColor: 'slate',
  },

  // 2. APPLICATION DOMAIN
  {
    extension: '.gapp',
    name: 'Globus Application Package',
    domain: 'Application',
    magicString: 'GLOBGAPP',
    magicHex: '47 4C 4F 42 47 41 50 50',
    mimeType: 'application/x-globus-app-bundle',
    targetConsumers: ['Globus Package Manager', 'Application Desktop Launcher'],
    description: 'Eigenständiges, vollständiges Applikations-Bundle (Container), vergleichbar mit macOS .app oder Android .apk, jedoch mit nativer ShivaCore Sandbox.',
    sampleFileName: 'GenesisExplorer.gapp',
    containerStructure: ['manifest.gmanifest', 'app.gexe (Primary Binary)', 'libraries/*.gdll', 'assets/ (Icons, Audio, Textures)', 'permissions/ (Security Policy)', 'localization/*.json', 'signature.gsig'],
    defaultCapabilities: ['graphics.vulkan', 'audio.output', 'storage.user_documents', 'net.client'],
    verificationSteps: ['Complete Manifest SHA-256 Merkle Root', 'Developer & Store Double-Signature', 'Sandbox Security Profile Evaluation'],
    badgeColor: 'indigo',
  },
  {
    extension: '.gpkg',
    name: 'Globus Package',
    domain: 'Application',
    magicString: 'GLOBGPKG',
    magicHex: '47 4C 4F 42 47 50 4B 47',
    mimeType: 'application/x-globus-package',
    targetConsumers: ['Globus Package Manager (gpm)', 'Software Repositories'],
    description: 'Komprimiertes Distributionspaket für Bibliotheken, Entwickler-SDKs oder System-Assets.',
    sampleFileName: 'vulkan-sdk-1.3.gpkg',
    containerStructure: ['Index Header', 'Dependency Manifest', 'Zstd Compressed Payload', 'Package Signature'],
    defaultCapabilities: ['fs.install_target'],
    verificationSteps: ['Repository Origin GPG/Ed25519 Check', 'Dependency Resolution Verification'],
    badgeColor: 'sky',
  },
  {
    extension: '.ginst',
    name: 'Installation Package',
    domain: 'Application',
    magicString: 'GLOBGINS',
    magicHex: '47 4C 4F 42 47 49 4E 53',
    mimeType: 'application/x-globus-installer',
    targetConsumers: ['Globus Setup Engine', 'Installer Wizard UI'],
    description: 'Setup-Paket mit deterministischen Installations-Hooks, Transaktions-Rollbacks und Systemprüfungen.',
    sampleFileName: 'setup-genesis-suite.ginst',
    containerStructure: ['Installer Script (Declarative)', 'Component Payloads', 'Pre-Flight Requirements Check', 'Rollback Journal Engine'],
    defaultCapabilities: ['fs.write_appdir', 'system.register_mimetype'],
    verificationSteps: ['System Compatibility Pre-Flight', 'Cryptographic Authenticity Check'],
    badgeColor: 'emerald',
  },
  {
    extension: '.gupd',
    name: 'Globus Update Package',
    domain: 'Application',
    magicString: 'GLOBGUPD',
    magicHex: '47 4C 4F 42 47 55 50 44',
    mimeType: 'application/x-globus-update',
    targetConsumers: ['Globus A/B System Updater', 'Live-Patch Engine'],
    description: 'Delta-Update-Paket für atomare A/B-Partitionen oder Live-Kernel-Patches ohne System-Reboot.',
    sampleFileName: 'globus-patch-2.4.1.gupd',
    containerStructure: ['Delta Patch Header', 'Binary Diffs (bsdiff/zstd)', 'Target Hash Pre-Condition', 'Post-Apply State Tree Hash'],
    defaultCapabilities: ['partition.ab_write', 'kernel.live_patch'],
    verificationSteps: ['Base Hash Exact Match', 'Target Post-Hash Determinism', 'OS Signing Authority'],
    badgeColor: 'amber',
  },

  // 3. BLOCKCHAIN DOMAIN
  {
    extension: '.atcb',
    name: 'ATC Bytecode',
    domain: 'Blockchain',
    magicString: 'GLOBATCB',
    magicHex: '47 4C 4F 42 41 54 43 42',
    mimeType: 'application/x-atc-bytecode',
    targetConsumers: ['ATC-VM Register Engine', 'Blockchain Node Validator'],
    description: 'Kompiliertes, 32-Bit-ausgerichtetes 256-Register Bytecode-Format mit deterministischem Gas-Accounting und formaler Typsicherheit.',
    sampleFileName: 'TokenContract.atcb',
    containerStructure: ['ATCB Header (Magic, ISA Ver, Gas Meta)', 'Constant Pool Table', 'Capability Grants Table', 'Code Segment (32-Bit Opcodes)', 'Gas Schedule Descriptor', 'Compiler Provenance Proof'],
    defaultCapabilities: ['vm.reg_read', 'vm.reg_write', 'chain.read_state', 'chain.emit_event'],
    verificationSteps: ['Bytecode Static Linear Verifier', 'Reentrancy & Stack Safety Proof', 'Gas Limit Bound Check', 'Deterministic Reproducibility Check'],
    badgeColor: 'emerald',
  },
  {
    extension: '.atcp',
    name: 'ATC Package',
    domain: 'Blockchain',
    magicString: 'GLOBATCP',
    magicHex: '47 4C 4F 42 41 54 43 50',
    mimeType: 'application/x-atc-package',
    targetConsumers: ['ATC Node Deployer', 'Smart Contract Registry'],
    description: 'Vollständiges Smart-Contract-Deploy-Paket inklusive Quellcode-Hash, ABI, Verifikations-Beweis und Berechtigungs-Manifest.',
    sampleFileName: 'DEXLiquidityPool.atcp',
    containerStructure: ['contract.atcb', 'contract_abi.json', 'source_manifest.json', 'permissions.gpolicy', 'deploy_config.gconf', 'developer.gsig'],
    defaultCapabilities: ['chain.state_mutate', 'chain.transfer_resource'],
    verificationSteps: ['ABI-to-Bytecode Fingerprint Match', 'Author Signature', 'State Migration Safety Check'],
    badgeColor: 'teal',
  },
  {
    extension: '.atcc',
    name: 'ATC Contract Source / Artifact',
    domain: 'Blockchain',
    magicString: 'GLOBATCC',
    magicHex: '47 4C 4F 42 41 54 43 43',
    mimeType: 'application/x-atc-contract',
    targetConsumers: ['ATCLang Compiler', 'Lumino IDE Smart Contract Suite'],
    description: 'High-Level Quellcode- oder Compiler-Artefakt-Container mit AST-Cache und formalen Verifikations-Constraints.',
    sampleFileName: 'GovernanceVoting.atcc',
    containerStructure: ['Source Text (ATCLang)', 'Parsed AST Cache', 'Type Annotation Tree', 'Formal Invariants Definitions'],
    defaultCapabilities: ['compiler.read'],
    verificationSteps: ['Syntax Validation', 'Type Consistency Check', 'Formal Invariant Prover'],
    badgeColor: 'green',
  },
  {
    extension: '.atcs',
    name: 'ATC State Snapshot',
    domain: 'Blockchain',
    magicString: 'GLOBATCS',
    magicHex: '47 4C 4F 42 41 54 43 53',
    mimeType: 'application/x-atc-state-snapshot',
    targetConsumers: ['Blockchain State Engine', 'Fast-Sync Node Daemon'],
    description: 'Kryptografischer Zustandsschnappschuss (Merkle-Patricia-State-Tree) für Near-Instant Node Fast Syncing.',
    sampleFileName: 'block-1200000.atcs',
    containerStructure: ['Snapshot Header (Block Height, AppHash)', 'Sparse Merkle Tree Leaves', 'Account Balances Table', 'Contract Storage Buckets', 'Validator Commit Signatures'],
    defaultCapabilities: ['chain.state_replace'],
    verificationSteps: ['Consensus 2/3+ Validator BLS-Aggregate Signature', 'Merkle Root Exact Match against Canonical Chain'],
    badgeColor: 'cyan',
  },
  {
    extension: '.atct',
    name: 'ATC Transaction Envelope',
    domain: 'Blockchain',
    magicString: 'GLOBATCT',
    magicHex: '47 4C 4F 42 41 54 43 54',
    mimeType: 'application/x-atc-transaction',
    targetConsumers: ['Mempool Router', 'RPC Gateway', 'Wallet Engine'],
    description: 'Signierter Transaktions-Umschlag mit Nonce, Gas-Limit, Replay-Schutz und Bech32m Empfänger-Definition.',
    sampleFileName: 'tx-send-tokens.atct',
    containerStructure: ['Sender Account ID', 'Nonce', 'Gas Price & Max Gas', 'Action Payload', 'Witness / Signature Block'],
    defaultCapabilities: ['chain.spend_funds'],
    verificationSteps: ['Ed25519 Sender Signature', 'Nonce Monotonicity', 'Account Balance Coverage'],
    badgeColor: 'blue',
  },

  // 4. AI & AGENTS DOMAIN
  {
    extension: '.gmodel',
    name: 'AI Model Package',
    domain: 'AI & Agents',
    magicString: 'GLOBGMDL',
    magicHex: '47 4C 4F 42 47 4D 44 4C',
    mimeType: 'application/x-globus-ai-model',
    targetConsumers: ['Aurora AI Runtime (Rust Candle/ONNX)', 'NPU/GPU Accelerator'],
    description: 'Hoch-optimierter Modell-Container mit Gewichten, Tokenizer, Quantisierungs-Metadaten und NPU-Direct-Memory-Mapping.',
    sampleFileName: 'AuroraCore-7B-Q4.gmodel',
    containerStructure: ['Model Header (Layers, Dims, Context Length)', 'Tokenizer Vocab & BPE Merges', 'Quantization Metadata (GGUF/AWQ)', 'Direct mmap Weight Tensors', 'Safety Guardrail Spec', 'Model Signature'],
    defaultCapabilities: ['accelerator.npu', 'accelerator.gpu', 'memory.hugepages'],
    verificationSteps: ['Weight Tensor Integrity Check', 'Safety Spec Audit', 'Provider Signature Verification'],
    badgeColor: 'yellow',
  },
  {
    extension: '.gagent',
    name: 'AI Agent Package',
    domain: 'AI & Agents',
    magicString: 'GLOBGAGT',
    magicHex: '47 4C 4F 42 47 41 47 54',
    mimeType: 'application/x-globus-ai-agent',
    targetConsumers: ['Aurora Agent Host', 'Globus Assistant Service'],
    description: 'Vollständiger installierbarer autonomer AI-Agent als System-Entität inklusive Persona, Tool-Definitionen, Speicher-Schema und Capability-Grenzen.',
    sampleFileName: 'FoxyDevOps.gagent',
    containerStructure: ['agent_identity.json', 'system_prompt.md', 'skills/*.gskill', 'tools/*.gtool', 'memory_policy.json', 'security_policy.gpolicy', 'signature.gsig'],
    defaultCapabilities: ['agent.reasoning', 'agent.memory_access', 'tool.invoke_restricted'],
    verificationSteps: ['Agent Security Policy Sandbox Review', 'Author Key Verification', 'Tool Permission Boundaries Check'],
    badgeColor: 'amber',
  },
  {
    extension: '.gskill',
    name: 'AI Skill Definition',
    domain: 'AI & Agents',
    magicString: 'GLOBGSKL',
    magicHex: '47 4C 4F 42 47 53 4B 4C',
    mimeType: 'application/x-globus-ai-skill',
    targetConsumers: ['Aurora Agent Host', 'Skill Orchestrator'],
    description: 'Modulare Fähigkeiten-Definition für Agenten mit Ausführungs-Logik, Prompt-Templates und Validierungs-Regeln.',
    sampleFileName: 'git-automation.gskill',
    containerStructure: ['Skill Descriptor', 'Few-Shot Examples', 'Parameter Schemas', 'Sandbox Constraints'],
    defaultCapabilities: ['skill.execute'],
    verificationSteps: ['Schema Validation', 'Capability Audit'],
    badgeColor: 'orange',
  },
  {
    extension: '.gtool',
    name: 'AI Tool Definition',
    domain: 'AI & Agents',
    magicString: 'GLOBGTOL',
    magicHex: '47 4C 4F 42 47 54 4F 4C',
    mimeType: 'application/x-globus-ai-tool',
    targetConsumers: ['Agent Tool Router', 'ShivaCore IPC Gateway'],
    description: 'Deterministische Werkzeug-Schnittstelle (Tool Interface) zur Anbindung von System-APIs an AI-Modelle.',
    sampleFileName: 'code-compiler.gtool',
    containerStructure: ['JSON-Schema OpenAPI / Tool Declaration', 'IPC Binding Address', 'Safety Rate Limits'],
    defaultCapabilities: ['tool.bridge_ipc'],
    verificationSteps: ['API Contract Verification', 'IPC Target Endpoint Check'],
    badgeColor: 'rose',
  },
  {
    extension: '.gknow',
    name: 'Knowledge Package',
    domain: 'AI & Agents',
    magicString: 'GLOBGKNW',
    magicHex: '47 4C 4F 42 47 4B 4E 57',
    mimeType: 'application/x-globus-knowledge-pack',
    targetConsumers: ['Vector DB Store', 'RAG Retrieval Engine'],
    description: 'Strukturierte Vektor- & Dokumenten-Datenbank für Retrieval-Augmented Generation (RAG) mit indexierten Chunks.',
    sampleFileName: 'globus-os-manual-de.gknow',
    containerStructure: ['Document Chunks', 'Embedding Vectors (HNSW Index)', 'Source Attribution Index', 'Metadata Filters'],
    defaultCapabilities: ['rag.query_vectors'],
    verificationSteps: ['Embedding Dimensionality Match', 'Source Integrity Check'],
    badgeColor: 'violet',
  },

  // 5. SECURITY DOMAIN
  {
    extension: '.gcert',
    name: 'Globus Certificate',
    domain: 'Security',
    magicString: 'GLOBGCRT',
    magicHex: '47 4C 4F 42 47 43 52 54',
    mimeType: 'application/x-globus-certificate',
    targetConsumers: ['Globus PKI Subsystem', 'Trust Store'],
    description: 'X.509-Alternative auf Basis von Ed25519 und Post-Quantum Dilithium zur Validierung von Entwicklern, Nodes und Behörden.',
    sampleFileName: 'root-ca.gcert',
    containerStructure: ['Issuer ID', 'Subject ID', 'Public Key', 'Validity Period', 'Permitted OID Capabilities', 'CA Signature'],
    defaultCapabilities: ['pki.trust_anchor'],
    verificationSteps: ['Cryptographic Chain of Trust to Root Anchor', 'CRL / OCSP Revocation Status Check'],
    badgeColor: 'emerald',
  },
  {
    extension: '.gkey',
    name: 'Key Container',
    domain: 'Security',
    magicString: 'GLOBGKEY',
    magicHex: '47 4C 4F 42 47 4B 45 59',
    mimeType: 'application/x-globus-key-container',
    targetConsumers: ['Globus Keyring Daemon', 'Hardware Security Module (HSM)'],
    description: 'Verschlüsselter Schlüssel-Container (Argon2id + ChaCha20-Poly1305) für private Schlüssel und Validator-Seeds.',
    sampleFileName: 'validator-node-1.gkey',
    containerStructure: ['KDF Parameters (Argon2id Memory/Iterations)', 'Salt', 'Nonce', 'Encrypted Key Payload', 'Auth Tag (Poly1305)'],
    defaultCapabilities: ['keyring.decrypt_with_passphrase'],
    verificationSteps: ['AEAD Tag Check', 'Entropy Audit'],
    badgeColor: 'red',
  },
  {
    extension: '.gsig',
    name: 'Signed Artifact (Detached Signature)',
    domain: 'Security',
    magicString: 'GLOBGSIG',
    magicHex: '47 4C 4F 42 47 53 49 47',
    mimeType: 'application/x-globus-signature',
    targetConsumers: ['Code Verifier', 'Package Manager'],
    description: 'Freistehende kryptografische Signaturdatei zur Integritätsabsicherung beliebiger Datenartefakte.',
    sampleFileName: 'release.tar.gsig',
    containerStructure: ['Target File Blake3 Hash', 'Signer Certificate Hash', 'Timestamp Block', 'Cryptographic Signature'],
    defaultCapabilities: ['security.verify'],
    verificationSteps: ['Hash-Match Against Target File', 'Signer Certificate Validity'],
    badgeColor: 'rose',
  },
  {
    extension: '.gpolicy',
    name: 'Security Policy (Capabilities)',
    domain: 'Security',
    magicString: 'GLOBGPOL',
    magicHex: '47 4C 4F 42 47 50 4F 4C',
    mimeType: 'application/x-globus-policy',
    targetConsumers: ['ShivaCore Security Monitor', 'Sandbox Engine'],
    description: 'Formale Deklaration von Capabilities, Syscall-Filtern (seccomp-äquivalent) und Resource-Quotas für Prozesse.',
    sampleFileName: 'browser-sandbox.gpolicy',
    containerStructure: ['Whitelisted Syscalls Mask', 'Allowed Network Interfaces & Ports', 'Filesystem Mount Restrictions', 'Max Memory & CPU Quota'],
    defaultCapabilities: ['security.enforce_policy'],
    verificationSteps: ['Policy Schema Validation', 'Privilege Escalation Prevention Lint'],
    badgeColor: 'indigo',
  },

  // 6. CONFIGURATION DOMAIN
  {
    extension: '.gconf',
    name: 'Globus Configuration',
    domain: 'Configuration',
    magicString: 'GLOBGCNF',
    magicHex: '47 4C 4F 42 47 43 4E 46',
    mimeType: 'application/x-globus-config',
    targetConsumers: ['System Services', 'Application Config Engine'],
    description: 'Strukturierte, typisierte und schema-validierte System- und Anwendungs-Konfiguration (TOML/KDL Derivat).',
    sampleFileName: 'node-network.gconf',
    containerStructure: ['Schema Reference URL', 'Key-Value Configuration Hierarchy', 'Default Overrides', 'Checksum'],
    defaultCapabilities: ['config.read'],
    verificationSteps: ['Schema Conformance Check', 'Value Range Boundary Verification'],
    badgeColor: 'slate',
  },
  {
    extension: '.gmanifest',
    name: 'Manifest Descriptor',
    domain: 'Configuration',
    magicString: 'GLOBGMNF',
    magicHex: '47 4C 4F 42 47 4D 4E 46',
    mimeType: 'application/x-globus-manifest',
    targetConsumers: ['Package Manager', 'Bundle Extractor', 'IDE'],
    description: 'Zentraler Paket-Deskriptor, der Bestandteile, Abhängigkeiten, Entrypoint und Berechtigungen eines .gapp oder .gpkg beschreibt.',
    sampleFileName: 'manifest.gmanifest',
    containerStructure: ['Application Metadata (ID, Name, Version)', 'Target Architectures & ABIs', 'Entrypoint Executable', 'Capabilities Array', 'Component Merkle Tree Hashes'],
    defaultCapabilities: ['manifest.parse'],
    verificationSteps: ['Semantic Version Check', 'File Hash Reconciliation'],
    badgeColor: 'sky',
  },
  {
    extension: '.gprofile',
    name: 'User / System Profile',
    domain: 'Configuration',
    magicString: 'GLOBGPRF',
    magicHex: '47 4C 4F 42 47 50 52 46',
    mimeType: 'application/x-globus-profile',
    targetConsumers: ['Globus Account Manager', 'Desktop Environment'],
    description: 'Benutzerprofil, Arbeitsbereichseinstellungen, Desktop-Themes und Zugriffspräferenzen.',
    sampleFileName: 'developer.gprofile',
    containerStructure: ['User UUID', 'UI Theme & Font Scales', 'Default Application Associations', 'Environment Variables', 'Per-User Permission Grants'],
    defaultCapabilities: ['profile.read_user'],
    verificationSteps: ['User Identity Authentication', 'Tamper-Proof HMAC Check'],
    badgeColor: 'purple',
  },
];

/**
 * Vollständige Dokumentationsvorlage für das Architektur-Handbuch (ATC-DOC-013)
 */
export const GFFA_SPEC_MARKDOWN = `# Globus File Format Architecture (GFFA) v1.0
**Normativer Dateiformat-Standard für Globus OS, ShivaCore, ATC-VM, Genesis Engine & Aurora AI**  
*Dokumenten-Kennung: ATC-DOC-013 • Status: Normativ • Sicherheitsstufe: Kern-Standard*

---

## 1. Vision: Ein konsistentes Dateiformat-Ökosystem

Windows nutzt seit Jahrzehnten bewährte Formate wie \`.exe\`, \`.dll\`, \`.sys\` und \`.msi\`.  
Für **Globus OS** und den **ShivaCore Microkernel** schaffen wir ein modernes, sicheres und homogenes System:
- **Einheitlicher Namensraum**: Alle nativen Systemformate beginnen mit \`.g*\`, Blockchain-Formate mit \`.atc*\`.
- **Gemeinsamer Container-Standard (GFF)**: Jedes native Format beginnt mit dem standardisierten 64-Byte Header \`GLOB\`.
- **Eingebaute Sicherheit**: Kein Start ohne vorherige Signatur-, Hash- und Capability-Prüfung.
- **Trennung von Format und Dateisystem**: Formate sind autonome Objekte, die sowohl auf ext4/ZFS als auch im zukünftigen *Globus Native File System (GNFS)* laufen.

---

## 2. Der Globus File Format (GFF) 64-Byte Header

Alle nativen Binärdateien teilen den gleichen 64-Byte Vorspann:

\`\`\`
┌─────────────────────────────────────────────────────────────┐
│ Offset 0x00: Magic Bytes "GLOB" (0x47 0x4C 0x4F 0x42)       │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x04: Format Tag (z.B. "GEXE", "GDLL", "ATCB")       │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x08: Version (u16) | Offset 0x0A: Flags (u16)       │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x0C: Arch (u16)    | Offset 0x0E: ABI Version (u16) │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x10: Entry Point Offset (u64)                       │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x18: Payload Size (u64)                             │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x20: Manifest Offset (u64)                          │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x28: Capability Descriptor Offset (u64)             │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x30: Signature Block Offset (u64)                   │
├─────────────────────────────────────────────────────────────┤
│ Offset 0x38: Blake3 Content Checksum Trunk (u64)            │
└─────────────────────────────────────────────────────────────┘
\`\`\`

---

## 3. Die 6 Domänen der Globus-Dateiformate

### A. System-Domäne
- **\`.gexe\` (Globus Executable)**: Ausführbare Binärdateien mit obligatorischer Capability-Bindung.
- **\`.gdll\` (Globus Dynamic Library)**: Dynamisch ladbare Bibliotheken mit typsicherer ABI.
- **\`.gsys\` (Globus System Module)**: Hochprivilegierte Microkernel-Komponenten (Ring-0/1 Services).
- **\`.gdrv\` (Globus Driver)**: User-Space-Treiber mit IOMMU-Schutz gegen Kernel-Crashes.
- **\`.gmod\` (Globus Module)**: Pluggable Erweiterungsmodule für Subsysteme.

### B. Anwendungs-Domäne
- **\`.gapp\` (Globus Application Package)**: Vollständiges Bundle (Manifest, Binary, Assets, Libs, Signatur).
- **\`.gpkg\` (Globus Package)**: Zstd-komprimiertes Distributionspaket für Bibliotheken und Tools.
- **\`.ginst\` (Installation Package)**: Atomares Setup-Paket mit deterministischem Rollback-Journal.
- **\`.gupd\` (Globus Update Package)**: Atomare A/B-Partition-Deltas und Kernel-Livepatches.

### C. Blockchain-Domäne
- **\`.atcb\` (ATC Bytecode)**: 32-Bit ausgerichteter 256-Register Bytecode für die ATC-VM.
- **\`.atcp\` (ATC Package)**: Smart-Contract-Bereitstellungspaket mit ABI, Proofs und Manifest.
- **\`.atcc\` (ATC Contract Artifact)**: Quellcode- und Compiler-Zwischenzustände.
- **\`.atcs\` (ATC State Snapshot)**: Merkle-State-Tree Schnappschuss für Instant Node Fast-Sync.
- **\`.atct\` (ATC Transaction Envelope)**: Signierter Transaktionscontainer für den Mempool.

### D. AI & Agenten-Domäne
- **\`.gmodel\` (AI Model Package)**: Modellgewichte mit mmap-NPU-Mapping und Guardrail-Spezifikation.
- **\`.gagent\` (AI Agent Package)**: Autonomer Agent als System-Entität (Persona, Tools, Skills).
- **\`.gskill\` (AI Skill Definition)**: Ausführbare Logikblöcke und Prompt-Templates für Agenten.
- **\`.gtool\` (AI Tool Definition)**: Typsichere IPC-Schnittstelle zur Systemsteuerung.
- **\`.gknow\` (Knowledge Package)**: Vektordatenbank (HNSW) und RAG-Wissensbasis.

### E. Sicherheits-Domäne
- **\`.gcert\` (Globus Certificate)**: Ed25519 & Dilithium Identitätszertifikate.
- **\`.gkey\` (Key Container)**: Argon2id + ChaCha20-Poly1305 geschützte private Schlüssel.
- **\`.gsig\` (Signed Artifact)**: Freistehende kryptografische Signatur zur Artefaktprüfung.
- **\`.gpolicy\` (Security Policy)**: Feingranulare Capability-Masken und Syscall-Filter.

### F. Konfigurations-Domäne
- **\`.gconf\` (Globus Configuration)**: Typisierte Konfigurationshierarchie.
- **\`.gmanifest\` (Manifest Descriptor)**: Strukturierte Paket-Inhaltsverzeichnisse.
- **\`.gprofile\` (User/System Profile)**: Benutzerprofile und Desktop-Zustände.

---

## 4. Die ShivaCore Ausführungs- & Verifikations-Pipeline

Bevor eine \`.gexe\` oder \`.gapp\` im System ausgeführt wird, durchläuft sie eine strikte Verifikationskette:

\`\`\`
  [ Datei auf Speichermedium ]
              │
              ▼
   1. Header Magic Scan (0x00..0x08 == "GLOBGEXE")
              │
              ▼
   2. Blake3 Hash Verification (Integrität)
              │
              ▼
   3. Ed25519 Signatur-Prüfung (Authentizität)
              │
              ▼
   4. Publisher Trust Chain (Gültiges .gcert)
              │
              ▼
   5. Capability Matrix Review (Angefordertes Recht vs. User Policy)
              │
              ▼
   6. ABI & CPU Architektur Match (z.B. x86_64, aarch64, atcvm)
              │
              ▼
   7. Statischer Safety Verifier (Keine verbotenen Instruktionen)
              │
              ▼
  [ ShivaCore Sandboxed Execution ]
\`\`\`

---

## 5. Trennung: Dateiformat vs. Dateisystem (VFS)

\`\`\`
Globus OS User Space
         │
    ShivaCore VFS
         │
   ┌─────┴────────────────┐
   │                      │
Host Dateisystem      GFF Object Engine
(ext4, ZFS, Btrfs)   (GEXE, GAPP, ATCB, GMODEL)
                          │
                   Zukunft: GNFS (Globus Native File System)
\`\`\`
`;
