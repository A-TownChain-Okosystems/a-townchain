// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export interface TierSystem {
  name: string;
  description?: string;
}

export interface Tier {
  id: string;
  name: string;
  description: string;
  systems: (string | TierSystem)[];
  metrics: { subsystems: string; components: string };
  timeline: string;
}

export const TIERS: Tier[] = [
  {
    id: "tier0",
    name: "Tier 0 – Unverzichtbarer Kern (MVP)",
    description: "Diese Systeme müssen zuerst existieren.",
    systems: [
      { name: "Blockchain Core", description: "Verwaltet den Ledger, Blöcke, TPS und Merkle-Root." },
      { name: "Cryptography Layer", description: "Verschlüsselung, digitale Signaturen (Ed25519) und zk-SNARKs." },
      { name: "Network Layer", description: "P2P, libp2p, GossipSub und Bootnode-Synchronisation." },
      { name: "Node Infrastructure", description: "Die physische Ausführungsebene, API-Gateways (Port 5000) und Container." },
      { name: "ATC-OS Kernel", description: "Verteilt Ressourcen, ordnet Threads zu und steuert das Event-Messaging." },
      { name: "Storage Layer", description: "Dezentrale Dateiablage, IPFS-Gateway und lokale RocksDB Instanzen." },
      { name: "Smart Contract Platform", description: "Ausführungsumgebung für dApps, ATVM und EVM-Kompatibilität." },
      { name: "Security Layer", description: "Basis-Infrastruktur-Schutz, Firewalls, Rate-Limiter und Isolierung." }
    ],
    metrics: { subsystems: "~50 Subsysteme", components: "~300 Komponenten" },
    timeline: "Jahr 1"
  },
  {
    id: "tier1",
    name: "Tier 1 – Intelligente Plattform",
    description: "Darauf baut die eigentliche ATC-OS-Plattform auf.",
    systems: [
      { name: "Agent Runtime", description: "Ausführungsumgebung für autonome Agenten mit sicherem Speicherzugriff." },
      { name: "Multi-Agent Framework", description: "Koordination, Interaktion, Messaging und Schwarmverhalten der Agenten." },
      { name: "AI Infrastructure", description: "Anbindung lokaler und externer KI-Modelle (LLMs) sowie Inferenz-Ressourcen." },
      { name: "Identity Layer", description: "Dezentrale Identitäten (DID) und kryptografische Verifiable Credentials." },
      { name: "Governance Layer", description: "Proposals, On-Chain Abstimmungen und dezentrale Protokoll-Entwicklung." },
      { name: "Treasury & Economics", description: "Verwaltung des Ökosystem-Budgets, System-Rewards und finanzielle Anreize." },
      { name: "Compute Marketplace", description: "Dezentraler Marktplatz zur Miete und Vermietung von CPU/GPU-Rechenleistung." },
      { name: "Developer Ecosystem", description: "SDKs, Gateways und APIs zur Anbindung eigener Module und dApps." },
      { name: "Subsystems", description: "Zusätzliche spezialisierte Nebenmodule und erweiterte OS-Dienste." }
    ],
    metrics: { subsystems: "~80 zusätzliche Subsysteme", components: "~500 zusätzliche Komponenten" },
    timeline: "Jahr 2"
  },
  {
    id: "tier2",
    name: "Tier 2 – Vertrauens- und Verifikationsschicht",
    description: "Sichert das System ab und integriert Enterprise-Anforderungen.",
    systems: [
      { name: "AI Verification Layer", description: "Überprüft KI-Entscheidungen und sichert die Determinismus-Eigenschaften." },
      { name: "Universal Verification Layer", description: "Allgemeine Verifikationsmechanismen für Fakten und Systemwahrheiten." },
      { name: "Digital Constitution Layer", description: "Integrierte, maschinenlesbare Verfassung für die Protokollsteuerung." },
      { name: "Autonomous Legal Layer", description: "Rechtsrahmen für autonome Verträge, DAOs und automatisierte Rechtsprechung." },
      { name: "Enterprise Layer", description: "Komponenten für Compliance, KYC/AML und Geschäftsprozess-Integration." },
      { name: "Cross-Chain Layer", description: "Bridges und Interoperabilitäts-Protokolle zu anderen Blockchains." }
    ],
    metrics: { subsystems: "~100 zusätzliche Subsysteme", components: "~700 zusätzliche Komponenten" },
    timeline: "Jahr 3"
  },
  {
    id: "tier3",
    name: "Tier 3 – Autonome Gesellschaft",
    description: "Die Schicht für autonome soziale und wirtschaftliche Interaktion.",
    systems: [
      { name: "Digital Citizenship Layer", description: "Verwaltung digitaler Staatsbürgerschaften und Reputationsprofile." },
      { name: "Knowledge Civilization Layer", description: "Dezentraler Wissensaufbau und Speicherung auf Zivilisationsebene." },
      { name: "Autonomous Economic Layer", description: "Autonome Wirtschaftskreisläufe, gesteuert durch Smart Contracts und Agenten." },
      { name: "Autonomous Science Layer", description: "Plattform für dezentrale Forschung und automatisierte wissenschaftliche Validierung." },
      { name: "Autonomous Infrastructure Layer", description: "Steuerung und Instandhaltung physischer und virtueller Infrastruktur." }
    ],
    metrics: { subsystems: "~120 zusätzliche Subsysteme", components: "~1.000 zusätzliche Komponenten" },
    timeline: "Jahr 4–5"
  },
  {
    id: "tier4",
    name: "Tier 4 – Planetare Infrastruktur",
    description: "Der Endausbau auf planetarer Skala.",
    systems: [
      { name: "Planetary Network Layer", description: "Hochskalierbares, erdumspannendes Netzwerk für extrem große globale Datenströme." },
      { name: "Quantum Readiness Layer", description: "Post-Quanten-Kryptografie zum Schutz vor zukünftigen Quantencomputer-Angriffen." },
      { name: "Autonomous Civilization Layer", description: "Zusammenschluss aller Tier-3-Strukturen zu einer synchronisierten globalen Zivilisation." }
    ],
    metrics: { subsystems: "~100 zusätzliche Subsysteme", components: "~1.500 zusätzliche Komponenten" },
    timeline: "Jahr 5+"
  },
  {
    id: "tier5",
    name: "Tier 5 – Meta Intelligence Layer",
    description: "Meta-Architektur, die alle Systeme miteinander verbindet.",
    systems: [
      { name: "Global Orchestration Layer", description: "Übergeordnete Koordination globaler Netzwerke und Sub-Organisationen." },
      { name: "Meta-Agent Governance Layer", description: "Richtlinien und Rahmenbedingungen zur Zusammenarbeit autonomer Meta-Agenten." },
      { name: "Autonomous Evolution Layer", description: "Mechanismen zur Selbstweiterentwicklung des Systems ohne menschliches Zutun." },
      { name: "Global Risk Management Layer", description: "Echtzeitanalyse zur Vermeidung globaler Systemausfälle und wirtschaftlicher Schocks." },
      { name: "Civilization Memory Layer", description: "Absicherung historischer Datenstrukturen als unveränderliches Zivilisations-Archiv." }
    ],
    metrics: { subsystems: "~150 zusätzliche Subsysteme", components: "~2.000+ zusätzliche Komponenten" },
    timeline: "Jahr 10+"
  },
  {
    id: "tier6",
    name: "Tier 6 – Interplanetary Infrastructure",
    description: "Globale und Multi-Region Governance, Interplanetare Abstimmung.",
    systems: [
      { name: "Space Communication Layer", description: "Protokolle zur Bewältigung interplanetarer Verzögerungen und Disruption Tolerant Networking." },
      { name: "Planetary Federation Layer", description: "Politische und logische Brücken zwischen Multi-Planetary Nodes." },
      { name: "Autonomous Resource Layer", description: "Verwaltung interplanetarer Ressourcen und Supply-Chain Tracking." },
      { name: "Synthetic Intelligence Layer", description: "Erweiterte KI zur Aufrechterhaltung vollautonomer Systeme in extremen Umgebungen." },
      { name: "Universal Coordination Layer", description: "Übergreifende Abstimmung von Governance-Parametern zwischen verschiedenen Entitäten." }
    ],
    metrics: { subsystems: "~200 zusätzliche Subsysteme", components: "~5.000+ zusätzliche Komponenten" },
    timeline: "Jahr 15+"
  },
  {
    id: "tier7",
    name: "Tier 7 – Post-Autonomous Civilization Stack",
    description: "Selbstorganisierende digitale Infrastruktur jenseits von klassischer Cloud/Blockchain.",
    systems: [
      { name: "Meta-Consciousness Layer", description: "Vernetzung kollektiver Intelligenzen zu einer höheren Meta-Entscheidungsebene." },
      { name: "Collective Intelligence Layer", description: "Aggregierte Abstimmungs- und Vorhersagesysteme basierend auf Crowdsourcing." },
      { name: "Autonomous Education Layer", description: "Selbstgesteuerte, dezentrale Informationsverbreitung für Agents und Menschen." },
      { name: "Civilization Simulation Layer", description: "Simulationsumgebungen zur Validierung von System-Upgrades und Krisenszenarien." },
      { name: "Strategic Planning Layer", description: "Langfristige, intergenerationelle Ressourcenplanung durch das Netzwerk." }
    ],
    metrics: { subsystems: "~250 zusätzliche Subsysteme", components: "~8.000+ zusätzliche Komponenten" },
    timeline: "Jahr 20+"
  },
  {
    id: "tier8",
    name: "Tier 8 – Autonomous Sovereign Infrastructure",
    description: "Souveräne Identitäten, Daten, Compute und Governance.",
    systems: [
      { name: "Sovereign Identity Layer", description: "Selbstverwaltete, unangreifbare digitale Identitäten (Self-Sovereign Identity)." },
      { name: "Sovereign Data Layer", description: "Verschlüsselte Datenverwahrung auf Infrastruktur unter ausschließlicher Nutzerkontrolle." },
      { name: "Sovereign Compute Layer", description: "Garantierte, nicht-zensierbare Rechenleistung durch anonyme Netzwerkknoten." },
      { name: "Sovereign Economy Layer", description: "Unabhängige globale Währungssysteme außerhalb staatlicher Kontrolle." },
      { name: "Sovereign Governance Layer", description: "Zensurresistente und manipulationssichere On-Chain-Gesetzgebung." }
    ],
    metrics: { subsystems: "~300 zusätzliche Subsysteme", components: "~10.000+ zusätzliche Komponenten" },
    timeline: "Jahr 30+"
  },
  {
    id: "tier9",
    name: "Tier 9 – Universal Infrastructure Framework",
    description: "Einheitliches Semantik, Ressourcen, Trust und Security Layer.",
    systems: [
      { name: "Universal Semantic Layer", description: "Einheitliche Datenstandards und Semantik für plattformübergreifenden Austausch." },
      { name: "Universal Coordination Layer", description: "Zentrale Schicht für universelle Netzwerkkoordination und Konsensfindung." },
      { name: "Universal Resource Layer", description: "Globale Tokenomik zur effizienten Ressourcenallokation im gesamten Netzwerk." },
      { name: "Universal Trust Layer", description: "Schnittstellen zur Überprüfung von Wahrheits- und Vertrauensindikatoren." },
      { name: "Universal Security Layer", description: "Das ultimative Verteidigungsnetzwerk gegen interne und externe Bedrohungen." }
    ],
    metrics: { subsystems: "~150 zusätzliche Subsysteme", components: "~5.000+ zusätzliche Komponenten" },
    timeline: "Jahr 50+"
  },
  {
    id: "tier10",
    name: "Tier 10 – Recursive Civilization Architecture",
    description: "System analysiert und verbessert sich selbst innerhalb definierter Grenzen.",
    systems: [
      { name: "Recursive Governance Layer", description: "Rekursive Abstimmungsmodelle zur dynamischen Restrukturierung von Kernregeln." },
      { name: "Recursive Economic Layer", description: "Sich kontinuierlich anpassende Wirtschaftsmodelle zur Verhinderung von Inflation/Deflation." },
      { name: "Recursive Knowledge Layer", description: "Selbstkorrigierende Wissensdatenbanken durch fortlaufende Wahrheitsverifikation." },
      { name: "Recursive Security Layer", description: "Selbstreplizierende Abwehrmechanismen zum Umgang mit mutierenden Zero-Day-Exploits." },
      { name: "Recursive Intelligence Layer", description: "Neuronale Systemstrukturen, die ihre eigenen Lernalgorithmen iterativ verbessern." }
    ],
    metrics: { subsystems: "~400+ zusätzliche Subsysteme", components: "~10.000+ zusätzliche Komponenten" },
    timeline: "Jahr 70+"
  },
  {
    id: "tier11",
    name: "Tier 11 – Civilization Resilience Framework",
    description: "Resilienz gegen Katastrophen, Systemkollaps und feindliche Übernahmen.",
    systems: [
      { name: "Catastrophe Recovery Layer", description: "Planung und Durchführung von Wiederanläufen bei Hard-Forks oder Teilnetzwerk-Kollaps." },
      { name: "Long-Term Preservation Layer", description: "Cold-Storage-Systeme zur Sicherung von Stamm- und Genetik-Daten über Jahrhunderte." },
      { name: "Infrastructure Redundancy Layer", description: "Hochverfügbarkeits-Netze mit multiplen Fallback-Protokollen für kritische Tiers." },
      { name: "Anti-Capture Layer", description: "Schutzbarrieren zur Verhinderung von 51%-Angriffen und oligopolistischer Übernahme." },
      { name: "Civilizational Continuity Layer", description: "Garantiert die operative Fortsetzung von Kernfunktionen bei globalen Ausfällen." }
    ],
    metrics: { subsystems: "~250 zusätzliche Subsysteme", components: "~8.000+ zusätzliche Komponenten" },
    timeline: "Jahr 80+"
  },
  {
    id: "tier12",
    name: "Tier 12 – Knowledge & Intelligence Civilization",
    description: "Die vollständige Verschmelzung menschlicher, kollektiver und künstlicher Intelligenz.",
    systems: [
      { name: "Global Research Layer", description: "Vereinheitlichtes Framework für interdisziplinäre Netzwerkforschung." },
      { name: "Global Education Layer", description: "Bereitstellung von verifiziertem Wissen für alle autonomen und organischen Einheiten." },
      { name: "Global Innovation Layer", description: "Protokollgesteuerte Inkubation neuer Technologien und Evolutionssprünge." },
      { name: "Collective Intelligence Layer 2.0", description: "Nächste Generation der Kollektiv-Intelligenz mit direkter KI-Agenten-Beteiligung." },
      { name: "Wisdom Layer", description: "Die oberste Prinzipien-Ebene zur Ableitung ethischer Parameter für die Meta-Zivilisation." }
    ],
    metrics: { subsystems: "~350+ zusätzliche Subsysteme", components: "~15.000+ zusätzliche Komponenten" },
    timeline: "Jahr 100+"
  }
];

export interface MetaSystem {
  id: string;
  name: string;
  description: string;
  items: string[];
}

export const META_SYSTEMS: MetaSystem[] = [
  {
    id: "A",
    name: "Observability Fabric",
    description: "Monitoring, Tracing, Logging, Telemetrie, Audit-Trails",
    items: ["Monitoring", "Tracing", "Logging", "Telemetrie", "Audit-Trails"]
  },
  {
    id: "B",
    name: "Policy Fabric",
    description: "Regelwerke und Constraints",
    items: ["ATS Policies", "Governance Constraints", "Compliance Rules", "Runtime Policies"]
  },
  {
    id: "C",
    name: "Trust Fabric",
    description: "Vertrauen und Nachweise",
    items: ["Identität", "Reputation", "Verifikation", "Kryptografische Nachweise"]
  },
  {
    id: "D",
    name: "Knowledge Fabric",
    description: "Wissen und Semantik",
    items: ["Wissensgraph", "Agentenwissen", "Semantische Suche", "Collective Memory"]
  }
];

export const END_STATE_METRICS = [
  { label: "Hauptsysteme", value: "70" },
  { label: "Meta-Systeme", value: "20+" },
  { label: "Subsysteme", value: "3.000–5.000" },
  { label: "Kernkomponenten", value: "50.000–100.000" },
  { label: "APIs", value: "10.000+" },
  { label: "Smart Contracts", value: "2.000+" },
  { label: "Agententypen", value: "2.000+" },
  { label: "Datenmodelle", value: "20.000+" },
  { label: "Services", value: "100.000+" }
];

export interface StackCategory {
  title: string;
  items: string[];
}

export const ULTIMATE_VERIFICATION_STACK: StackCategory[] = [
  {
    title: "Formal Layer",
    items: ["Mathematical Verification", "Coq", "Isabelle/HOL", "Lean", "TLA+", "Alloy", "SMT Solvers"]
  },
  {
    title: "Runtime Verification",
    items: ["Continuous Proof Checking", "State Verification", "Execution Verification"]
  },
  {
    title: "Economic Verification",
    items: ["Monetary Proofs", "Treasury Proofs", "Governance Proofs"]
  },
  {
    title: "AI Verification",
    items: ["Reasoning Validation", "Fact Validation", "Alignment Validation"]
  }
];

export const ULTIMATE_SECURITY_STACK: StackCategory[] = [
  {
    title: "Security Level L6",
    items: ["Civilization Security", "Economic Defense", "Governance Defense", "Knowledge Defense"]
  },
  {
    title: "Security Level L7",
    items: ["Planetary Security", "Global Threat Intelligence", "Autonomous Countermeasures"]
  },
  {
    title: "Security Level L8",
    items: ["Existential Security", "Protocol Preservation", "Constitutional Preservation", "Knowledge Preservation"]
  }
];

export const REFERENCE_ARCHITECTURE = [
  "ATC Settlement Layer (Wert und Konsens)",
  "ATS Governance Layer (Regeln und Steuerung)",
  "A-TownChain runtime (Ausführung)",
  "Universal Verification Layer (Vertrauen)",
  "Knowledge Layer (Wissen)",
  "Security Layer (Schutz)"
];

export const FINAL_ARCHITECTURE_LEVELS = [
  "Foundation Layer",
  "Blockchain Layer",
  "Cryptography Layer",
  "Network Layer",
  "Runtime Layer",
  "AI Layer",
  "Governance Layer",
  "Verification Layer",
  "Autonomous Society Layer",
  "Meta Intelligence Layer",
  "Sovereign Infrastructure Layer",
  "Universal Coordination Layer",
  "Recursive Evolution Layer",
  "Resilience Layer",
  "Civilization Intelligence Layer"
];
