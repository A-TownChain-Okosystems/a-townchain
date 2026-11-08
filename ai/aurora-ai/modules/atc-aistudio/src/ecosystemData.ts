// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export interface EcosystemCategory {
  id: number;
  title: string;
  subcategories: {
    title: string;
    items: string[];
  }[];
}

export const ECOSYSTEM_DATA: EcosystemCategory[] = [
  {
    id: 1,
    title: "Foundation Layer",
    subcategories: [
      {
        title: "Vision Layer",
        items: ["A-TownChain", "ATC-OS", "ATS Governance Framework", "Autonomous Civilization Architecture", "AI-Native Infrastructure"]
      },
      {
         title: "Economic Layer",
         items: ["ATC Coin", "Protocol Token", "Treasury", "Emission Engine", "Reward Engine", "Incentive System", "Reputation Economy", "Compute Economy"]
      }
    ]
  },
  {
    id: 2,
    title: "Blockchain Core",
    subcategories: [
      {
        title: "Consensus Engine",
        items: ["Proof of Stake", "Reputation Consensus", "AI Validation Consensus", "Governance Consensus", "Finality Layer", "Fork Resolution"]
      },
      {
        title: "Block Production",
        items: ["Transaction Pool", "Block Builder", "Block Validator", "Finalization Engine", "State Transition Engine"]
      },
      {
        title: "State Layer",
        items: ["Global State Database", "Merkle Trees", "State Proofs", "State Snapshots", "State Recovery"]
      }
    ]
  },
  {
    id: 3,
    title: "Cryptography Layer",
    subcategories: [
      {
         title: "Core Crypto",
         items: ["Ed25519", "secp256k1", "BLS Signatures", "Threshold Signatures"]
      },
      {
         title: "Security Primitives",
         items: ["Hash Functions", "Merkle Proofs", "Zero Knowledge Proofs", "zk-SNARKs", "zk-STARKs"]
      },
      {
         title: "Identity",
         items: ["DID Framework", "Verifiable Credentials", "Reputation Identity", "Capability Tokens"]
      }
    ]
  },
  {
     id: 4,
     title: "Networking Layer",
     subcategories: [
        {
           title: "P2P Infrastructure",
           items: ["libp2p", "GossipSub", "Kademlia DHT", "Peer Discovery", "NAT Traversal"]
        },
        {
           title: "Node Infrastructure",
           items: ["Boot Nodes", "Validators", "Full Nodes", "Archive Nodes", "Light Clients"]
        },
        {
           title: "Communication",
           items: ["RPC", "WebSocket", "gRPC", "Event Streaming"]
        }
     ]
  },
  {
     id: 5,
     title: "ATC-OS Kernel",
     subcategories: [
        { title: "AI Microkernel", items: ["Scheduler", "Resource Manager", "Process Manager", "Security Manager", "Policy Manager"] },
        { title: "Runtime Layer", items: ["Task Execution", "Memory Management", "Capability Management", "Isolation Layer", "Sandboxing"] },
        { title: "Self-Healing Engine", items: ["Fault Detection", "Recovery Engine", "Auto Repair", "Failover System"] }
     ]
  },
  {
     id: 6,
     title: "Agent System",
     subcategories: [
        { title: "Agent Runtime", items: ["Agent Deployment", "Agent Registry", "Agent Communication", "Agent State Management"] },
        { title: "Multi-Agent Framework", items: ["Swarm Intelligence", "Agent Collaboration", "Capability Negotiation", "Distributed Planning"] },
        { title: "Agent Memory", items: ["Short-Term Memory", "Long-Term Memory", "Knowledge Graph", "Semantic Memory"] }
     ]
  },
  {
     id: 7,
     title: "Artificial Intelligence Layer",
     subcategories: [
        { title: "AI Core", items: ["Local Inference", "Distributed Inference", "Federated Learning", "Model Routing"] },
        { title: "Model Infrastructure", items: ["GGUF Models", "ONNX Models", "Transformer Models", "Agent Models"] },
        { title: "Intelligence Services", items: ["Reasoning Engine", "Planning Engine", "Prediction Engine", "Optimization Engine"] }
     ]
  },
  {
     id: 8,
     title: "Storage Layer",
     subcategories: [
        { title: "Distributed Storage", items: ["IPFS", "Content Addressing", "CID Verification", "Replication"] },
        { title: "Databases", items: ["Merkle State Database", "Vector Database", "Knowledge Graph Database", "Time Series Database"] },
        { title: "Data Protection", items: ["Encryption", "Redundancy", "Backup", "Disaster Recovery"] }
     ]
  },
  {
     id: 9,
     title: "Smart Contract Layer",
     subcategories: [
        { title: "Execution Engine", items: ["ATVM", "Contract Runtime", "Gas Accounting", "Contract Scheduler"] },
        { title: "Contract Framework", items: ["Ink!", "Solidity Compatibility", "Application Contracts", "Governance Contracts"] },
        { title: "Marketplace Contracts", items: ["Resource Market", "Compute Market", "Storage Market", "AI Market"] }
     ]
  },
  {
     id: 10,
     title: "Compute Marketplace",
     subcategories: [
        { title: "Resource Exchange", items: ["CPU Marketplace", "GPU Marketplace", "Storage Marketplace", "Bandwidth Marketplace"] },
        { title: "Compute Economy", items: ["Compute Pricing", "Resource Auctions", "Dynamic Markets", "Incentive System"] }
     ]
  },
  {
     id: 11,
     title: "Governance Layer",
     subcategories: [
        { title: "DAO Core", items: ["Proposal System", "Voting System", "Treasury System", "Delegation System"] },
        { title: "ATS Governance", items: ["Constitutional Layer", "Policy Engine", "Economic Governance", "Monetary Governance"] },
        { title: "Autonomous Governance", items: ["AI Risk Analysis", "Proposal Simulation", "Governance Automation", "Constitutional Validation"] }
     ]
  },
  {
     id: 12,
     title: "Security Layer L0–L5",
     subcategories: [
        { title: "L0 Foundation Security", items: ["Hardware Trust", "Secure Boot", "TPM Integration"] },
        { title: "L1 Infrastructure Security", items: ["Network Security", "Node Security", "Identity Security"] },
        { title: "L2 Runtime Security", items: ["Sandboxing", "Capability Security", "Isolation"] },
        { title: "L3 AI Security", items: ["Prompt Protection", "Model Security", "Adversarial Detection"] },
        { title: "L4 Blockchain Security", items: ["Consensus Security", "Smart Contract Security", "Treasury Security"] },
        { title: "L5 Autonomous Defense", items: ["Threat Detection", "Automated Response", "Cyber Defense Agents"] }
     ]
  },
  {
     id: 13,
     title: "Verification Layer",
     subcategories: [
        { title: "AI Verification Layer", items: ["Fact Verification", "Truth Validation", "Multi-Agent Validation", "Semantic Consistency"] },
        { title: "Formal Verification", items: ["Consensus Proofs", "Smart Contract Proofs", "Runtime Proofs", "Protocol Proofs"] },
        { title: "Mathematical Verification", items: ["Coq Proofs", "Isabelle/HOL", "SMT Solvers", "Theorem Proving"] }
     ]
  },
  {
     id: 14,
     title: "Trusted Computing Base",
     subcategories: [
        { title: "Verifiable Foundation", items: ["Verified Boot Chain", "Verified Compiler", "Verified Runtime", "Verified Kernel"] },
        { title: "Minimal TCB", items: ["Consensus Core", "Cryptography Core", "Verification Core", "Security Core"] }
     ]
  },
  {
     id: 15,
     title: "Enterprise Layer",
     subcategories: [
        { title: "Enterprise Services", items: ["Federation", "Multi-Tenant Infrastructure", "Compliance", "Auditing"] },
        { title: "Regulatory Layer", items: ["GDPR", "AML", "KYC", "Audit Trails"] }
     ]
  },
  {
     id: 16,
     title: "Cross-Chain Layer",
     subcategories: [
        { title: "Blockchain Bridges", items: ["Bitcoin", "Ethereum", "Polkadot", "Cosmos", "Avalanche"] },
        { title: "Interoperability", items: ["Cross-Chain Messaging", "Cross-Chain Assets", "Shared Security"] }
     ]
  },
  {
     id: 17,
     title: "Digital Asset Layer",
     subcategories: [
        { title: "NFT Infrastructure", items: ["Identity NFTs", "Governance NFTs", "AI Model NFTs", "Asset NFTs"] },
        { title: "Token Infrastructure", items: ["ATC Coin", "Protocol Token", "Compute Credits", "Reputation Tokens"] }
     ]
  },
  {
     id: 18,
     title: "Developer Ecosystem",
     subcategories: [
        { title: "DSKs", items: ["atc-lang DSK", "atc-lang DSK", "atc-lang DSK"] },
        { title: "APIs", items: ["REST API", "WebSocket API", "GraphQL API"] },
        { title: "Tooling", items: ["atc-lang IDE Software", "CLI", "Explorer", "Dashboard", "Monitoring"] }
     ]
  },
  {
     id: 19,
     title: "Spatial Computing Layer",
     subcategories: [
        { title: "Metaverse Infrastructure", items: ["AR Systems", "VR Systems", "Digital Twins", "Spatial Mapping"] },
        { title: "Semantic Reality Layer", items: ["Knowledge Spaces", "Semantic Navigation", "Autonomous Environments"] }
     ]
  },
  {
     id: 20,
     title: "Autonomous Civilization Layer",
     subcategories: [
        { title: "Autonomous Economy", items: ["Autonomous Markets", "Autonomous Organizations", "Autonomous Services"] },
        { title: "Autonomous Society", items: ["Digital Citizenship", "Reputation Governance", "Autonomous Communities"] },
        { title: "Autonomous Knowledge", items: ["Global Knowledge Graph", "Collective Intelligence", "Self-Evolving AI"] }
     ]
  },
  {
     id: 21,
     title: "Digital Constitution Layer",
     subcategories: [
        { title: "Constitutional Core", items: ["Immutable Principles", "Governance Constraints", "Rights Framework", "Constitutional Court Engine", "Amendment Framework", "AI Constitutional Validation"] }
     ]
  },
  {
     id: 22,
     title: "Knowledge Civilization Layer",
     subcategories: [
        { title: "Knowledge System", items: ["Global Knowledge Graph", "Scientific Repository", "Collective Memory", "Semantic Search", "Knowledge Verification", "Research Layer", "Academic Validation"] }
     ]
  },
  {
     id: 23,
     title: "Autonomous Science Layer",
     subcategories: [
        { title: "AI Research", items: ["Research Agents", "Experiment Agents", "Hypothesis Generator", "Simulation Engine", "Peer Review AI", "Discovery Network"] }
     ]
  },
  {
     id: 24,
     title: "Autonomous Economic Layer",
     subcategories: [
        { title: "Economic Framework", items: ["Dynamic Pricing", "Market Intelligence", "Economic Simulations", "Resource Allocation", "Autonomous Trade", "Economic Forecasting"] }
     ]
  },
  {
     id: 25,
     title: "Autonomous Legal Layer",
     subcategories: [
        { title: "Digital Jurisdiction", items: ["Smart Arbitration", "Dispute Resolution", "Contract Enforcement", "Compliance Agents", "Regulatory Mapping"] }
     ]
  },
  {
     id: 26,
     title: "Digital Citizenship Layer",
     subcategories: [
        { title: "Digital Society", items: ["Reputation Systems", "Identity Framework", "Citizenship Levels", "Civic Participation", "Community Governance"] }
     ]
  },
  {
     id: 27,
     title: "Autonomous Infrastructure Layer",
     subcategories: [
        { title: "Self-Managing Infra", items: ["Self-Healing Nodes", "Autonomous Scaling", "Autonomous Deployment", "Autonomous Upgrades", "Autonomous Monitoring"] }
     ]
  },
  {
     id: 28,
     title: "Quantum Readiness Layer",
     subcategories: [
        { title: "Post-Quantum Security", items: ["PQ Cryptography", "Hybrid Signatures", "Quantum Detection", "Quantum Migration Framework"] }
     ]
  },
  {
     id: 29,
     title: "Planetary Network Layer",
     subcategories: [
        { title: "Global Infrastructure", items: ["Mesh Networks", "Satellite Nodes", "Edge Computing", "Distributed Routing", "Global Synchronization"] }
     ]
  },
  {
     id: 30,
     title: "Universal Verification Layer",
     subcategories: [
        { title: "Mathematical Verification", items: ["Coq Verification", "Isabelle Verification", "TLA+ Specifications", "Model Checking", "Runtime Verification", "Consensus Verification", "Kernel Verification", "Economic Verification"] }
     ]
  }
];
