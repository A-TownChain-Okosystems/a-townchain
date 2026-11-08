// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export interface StandardCategory {
  id: string;
  title: string;
  description: string;
  items: StandardItem[];
}

export interface StandardItem {
  id: string;
  title: string;
  description: string;
  status: 'Core' | 'Active' | 'Draft' | 'Proposed';
}

export const ATC_STANDARDS: StandardCategory[] = [
  {
    id: "atc-core",
    title: "Core Protocol (ATC)",
    description: "Definitionen für die Kernarchitektur, Konsens und Basisprotokolle der A-TownChain.",
    items: [
      { id: "ATC-1", title: "Consensus Mechanism", description: "Spezifikation des Proof-of-Stake und Reputation Layers.", status: "Core" },
      { id: "ATC-2", title: "Network Cryptography", description: "Standards für Ed25519 und Post-Quantum Readiness.", status: "Core" },
      { id: "ATC-3", title: "State Machine", description: "Struktur des Global State Objekts und State Transition Rules.", status: "Core" },
      { id: "ATC-4", title: "P2P Communication", description: "Node-Discovery, Gossip-Protokolle und Kademlia-Routing.", status: "Core" }
    ]
  },
  {
    id: "atc-vm",
    title: "Virtual Machine & Execution",
    description: "Ausführungsumgebung für Smart Contracts und OS-Runtimes.",
    items: [
      { id: "ATC-10", title: "ATVM Runtime Spec", description: "Speicherverwaltung und Instruktionssatz der Contract VM.", status: "Core" },
      { id: "ATC-11", title: "Gas Metering", description: "Berechnungsmodelle für CPU, Storage, und AI-Inferenz-Ressourcen.", status: "Active" },
      { id: "ATC-12", title: "Cross-VM Communication", description: "Interoperabilitätsstandards zwischen Sub-Instanzen.", status: "Proposed" }
    ]
  }
];

export const ATS_STANDARDS: StandardCategory[] = [
  {
    id: "ats-tokens",
    title: "Digital Assets & Economy (ATS)",
    description: "Standards für Token, NFTs, Economics und digitale Besitzverhältnisse.",
    items: [
      { id: "ATS-20", title: "Fungible Assets", description: "Einheitlicher Standard für austauschbare Werte und Währungen.", status: "Active" },
      { id: "ATS-721", title: "Non-Fungible Assets", description: "Standard für einzigartige digitale Entitäten und Zertifikate.", status: "Active" },
      { id: "ATS-1155", title: "Multi-Asset Containers", description: "Batch-Transfers und Verwaltung komplexer Asset-Strukturen.", status: "Active" },
      { id: "ATS-4626", title: "Yield Bearing Vaults", description: "Standardisierte Schnittstellen für Treasury- und Staking-Vaults.", status: "Proposed" }
    ]
  },
  {
    id: "ats-agents",
    title: "AI & Agent Civilization",
    description: "Schnittstellen und Verhaltensnormen für autonome KI-Agenten.",
    items: [
      { id: "ATS-Agent-1", title: "Agent Identity & Registration", description: "On-Chain Identitätsnachweise für autonome Agenten.", status: "Active" },
      { id: "ATS-Agent-2", title: "Swarm Communication Protocol", description: "Standardisiertes P2P Messaging zwischen autonomen Entitäten.", status: "Active" },
      { id: "ATS-Agent-3", title: "Capability Discovery", description: "Framework für Agenten zur Offenlegung ihrer Fähigkeiten.", status: "Active" },
      { id: "ATS-Agent-4", title: "Resource Budgeting", description: "Verwaltung von Compute- und Token-Budgets durch Agenten.", status: "Proposed" }
    ]
  },
  {
    id: "ats-gov",
    title: "Governance & DAOs",
    description: "Konsensbildung, Abstimmungsverfahren und Protokoll-Aktualisierungen.",
    items: [
      { id: "ATS-Gov-1", title: "Constitutional Core", description: "Die unumstößlichen Grundprinzipien des ATC-OS und Netzwerks.", status: "Core" },
      { id: "ATS-Gov-2", title: "Proposal Framework", description: "Status-Lifecycles und Vorlagen für Systemänderungsanträge.", status: "Active" },
      { id: "ATS-Gov-3", title: "Delegation Metrics", description: "Reputationsbasiertes Voting und Veto-Szenarien.", status: "Active" }
    ]
  },
  {
    id: "ats-verify",
    title: "Verification & Trust",
    description: "Mathematische und epistemische Verifizierungsstandards.",
    items: [
      { id: "ATS-Verify-1", title: "Proof-of-Truth", description: "Abgleich und Konsensfindung für Faktenwissen.", status: "Active" },
      { id: "ATS-Verify-2", title: "Zero-Knowledge State Proofs", description: "Interaktionsfreie Beweise für On-Chain Statusänderungen.", status: "Proposed" },
      { id: "ATS-Verify-3", title: "AI Alignment Output", description: "Kennzeichnung verifizierter und nicht-verifizierter KI-Inferenz.", status: "Active" }
    ]
  }
];
