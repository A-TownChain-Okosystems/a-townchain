// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export interface RoadmapGoal {
  text: string;
  completed: boolean;
  notes?: string;
  tags?: string[];
}

export interface RoadmapPhase {
  id: string;
  title: string;
  timeframe?: string;
  status?: string;
  network?: 'devnet' | 'testnet' | 'mainnet' | 'all';
  sections: RoadmapSection[];
  goals: RoadmapGoal[];
  githubTotalIssues?: number;
  githubClosedIssues?: number;
  history?: { status: string; timestamp: string }[];
}

export interface RoadmapSection {
  title: string;
  items: string[];
}

export const ROADMAP_DATA: RoadmapPhase[] = [
  {
    id: "phase0",
    title: "Foundation & ZK-Proofs (Enterprise Initialization)",
    timeframe: "Phase 0",
    status: "Abgeschlossen",
    network: "mainnet",
    history: [
      { status: "Abgeschlossen", timestamp: "2024-03-01 10:00:00" },
      { status: "Abgeschlossen", timestamp: "2023-12-01 09:00:00" },
    ],
    sections: [
      {
        title: "Architektur & Mathempirische Modelle",
        items: [
          "BFT Synchronous Model: N ≥ 3f + 1",
          "ZKP Circuits: $e(g_1, g_2)^{\alpha\beta} = e(g_1^{\alpha}, g_2^{\beta})$",
          "Wirtschaftsmodell: Deflationary Tokenomics mit $\\rho(t) = \\rho_0 e^{-\\lambda t}$",
          "Automated Theorem Proving Integration",
        ],
      },
      {
        title: "ATC-OS Kernel Spec",
        items: [
          "Memory Management Unit (MMU) Isolation",
          "POSIX-compliant VFS over IPFS",
          "Deterministic Execution Layer (ATVM-based)",
        ],
      },
    ],
    goals: [
      {
        text: "Ergebnis: Vollständig verifizierbarer Blueprint (Coq/Isabelle evaluiert)",
        completed: true,
        tags: ["Research", "Design", "Math"],
      },
    ],
  },
  {
    id: "phase1",
    title: "Core Protocol & Slashing Mechanics",
    timeframe: "Phase 1",
    status: "Abgeschlossen",
    network: "mainnet",
    history: [{ status: "Abgeschlossen", timestamp: "2024-04-15 14:30:00" }],
    sections: [
      {
        title: "Blockchain State Machine",
        items: [
          "Merkle-Patricia Trie (O(log(N)) lookup)",
          "Deterministic Transaktions-Pool DAG",
          "Verifiable Delay Functions (VDFs) Modul",
        ],
      },
      {
        title: "Consensus (PoI + PoS + PoW + PoH)",
        items: [
          "Sybil Resistance via $C(x) > \\tau$",
          "Equivocation Slashing: Penalty $\\Pi = S \\cdot \\max(0.1, \\frac{f}{N})$",
        ],
      },
    ],
    goals: [
      {
        text: "Meilenstein M1: Mathematisch beweisbare State Transitions",
        completed: true,
        tags: ["Core", "Consensus"],
      },
    ],
  },
  {
    id: "phase2",
    title: "Network & Subgraph Telemetry",
    timeframe: "Phase 2",
    status: "Abgeschlossen",
    network: "mainnet",
    sections: [
      {
        title: "P2P Topology",
        items: [
          "Kademlia DHT ($O(\\log N)$ routing)",
          "GossipSub v2 mit PX",
          "Byzantine resilient peer discovery",
        ],
      },
      {
        title: "Metrics & Tracing",
        items: [
          "Jaeger Distributed Tracing",
          "Prometheus Time-Series",
          "Grafana Enterprise Dashboards",
        ],
      },
    ],
    goals: [
      {
        text: "Meilenstein M2: 100k TPS Gossip Network Throughput",
        completed: true,
      },
    ],
  },
  {
    id: "phase3",
    title: "AI Kernel & Neural Scheduling",
    timeframe: "Phase 3",
    status: "Abgeschlossen",
    network: "mainnet",
    sections: [
      {
        title: "Local Inference Layer",
        items: [
          "Tensor-Parallel Execution",
          "ONNX/GGUF Checkpoint Verifikation",
          "LoRA Injection API",
        ],
      },
      {
        title: "Neural Orchestration",
        items: [
          "Self-Attention basierte Task Priorisierung",
          "Dynamic Parameter Quantization (INT4/INT8)",
        ],
      },
    ],
    goals: [
      {
        text: "Meilenstein M3: Deterministic LLM Execution per Block",
        completed: true,
      },
    ],
  },
  {
    id: "phase4",
    title: "Distributed Datalake & Rollups",
    timeframe: "Phase 4",
    status: "Abgeschlossen",
    network: "testnet",
    sections: [
      {
        title: "Data Availability (DA)",
        items: [
          "Erasure Coding: Reed-Solomon $RS(N, K)$",
          "KZG Commitments für Data Blobs",
        ],
      },
      {
        title: "Storage Layout",
        items: [
          "Content-Addressed Vector DB",
          "Post-Quantum Cryptographic Sharding",
        ],
      },
    ],
    goals: [
      {
        text: "Meilenstein M4: Beweisbare Datenverfügbarkeit für Rollups",
        completed: true,
      },
    ],
  },
  {
    id: "phase5",
    title: "Enterprise Agent Civilization",
    timeframe: "Phase 5",
    status: "Abgeschlossen",
    network: "testnet",
    sections: [
      {
        title: "Agent Swarms",
        items: [
          "Byzantine Fault Tolerant Agent Coordination",
          "Multi-Agent Reinforcement Learning (MARL)",
        ],
      },
      {
        title: "Nash Equilibrium Markets",
        items: [
          "Automated Market Maker (AMM) für Agents",
          "Stable Matching Theory (Gale-Shapley)",
        ],
      },
    ],
    goals: [
      {
        text: "Meilenstein M5: Nash-Gleichgewicht in autonomen Märkten",
        completed: true,
      },
    ],
  },
  {
    id: "phase6",
    title: "Smart Contract Formal Methods",
    timeframe: "Phase 6",
    status: "Abgeschlossen",
    network: "testnet",
    sections: [
      {
        title: "VM Compilation",
        items: [
          "ATVM deterministische Bytecode-Extraktion",
          "Gas Limit = $\\sum (OpCode_i \\times Cost_i)$",
        ],
      },
      {
        title: "Formal Verification",
        items: ["Bounded Model Checking", "Symbolic Execution Coverage > 99%"],
      },
    ],
    goals: [
      {
        text: "Meilenstein M6: Exploit-freie Smart Contracts (Mathematisch bewiesen)",
        completed: true,
      },
    ],
  },
  {
    id: "phase7",
    title: "Zero-Knowledge Hardware Subsystem",
    timeframe: "Phase 7",
    status: "Abgeschlossen",
    network: "devnet",
    sections: [
      {
        title: "ZK-Rollups",
        items: [
          "Groth16 / PLONK Circuit Implementierung",
          "Polynomial Commitments",
        ],
      },
      {
        title: "Hardware Enclave",
        items: ["Intel SGX / ARM TrustZone Bridge", "Side-channel Resistenz"],
      },
    ],
    goals: [
      {
        text: "Meilenstein M7: Hardware-beschleunigte ZK-Proofs",
        completed: true,
      },
    ],
  },
  {
    id: "phase8",
    title: "Federated Enterprise Sync & Compliance",
    timeframe: "Phase 8",
    status: "Abgeschlossen",
    network: "devnet",
    history: [
      {
        status: "Abgeschlossen",
        timestamp: new Date().toISOString().replace("T", " ").substring(0, 19),
      },
      { status: "Abgeschlossen", timestamp: "2026-06-08 10:00:00" },
    ],
    sections: [
      {
        title: "Cross-Chain Sync",
        items: [
          "Atomic Swaps ($H(x)$ Hash Time-Locked Contracts)",
          "Light Client State Proofs",
        ],
      },
      {
        title: "Offline Queue Engine",
        items: [
          "CRDT (Conflict-free Replicated Data Types)",
          "Vector Clocks $V(x) < V(y)$",
        ],
      },
      {
        title: "Privacy & Compliance",
        items: [
          "Zero-Knowledge Proofs (ZK-SNARKs)",
          "Off-Chain Data Enclaves (Right to be Forgotten)",
          "Stealth Addresses & AML Oracles",
        ],
      },
    ],
    goals: [
      {
        text: "Meilenstein M8: 100% Conflict-Free Enterprise Datensynchronisation & DSGVO/MiCA Compliance",
        completed: true,
      },
    ],
  },
];
