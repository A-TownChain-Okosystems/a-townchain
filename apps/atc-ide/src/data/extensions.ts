export interface ExtensionCategory {
  name: string;
  extensions: {
    ext: string;
    name: string;
    description: string;
  }[];
}

export const EXTENSION_CATEGORIES: ExtensionCategory[] = [
  {
    "name": "ATC v1.0 - Infrastructure Layer",
    "extensions": [
      {
        "ext": ".atc",
        "name": "A-TownChain Core File",
        "description": "Basisdatei für Blockchain-Datenstrukturen, State-Definitionen und Netzwerkparameter"
      },
      {
        "ext": ".atb",
        "name": "A-Town Block",
        "description": "Einzelner Block inklusive Header, Merkle Root, Signaturen und Transaktionen"
      },
      {
        "ext": ".attx",
        "name": "A-Town Transaction",
        "description": "Standardisierte Transaktionsdatei"
      },
      {
        "ext": ".atst",
        "name": "A-Town State",
        "description": "State-Snapshot einer bestimmten Blockhöhe"
      },
      {
        "ext": ".atv",
        "name": "A-Town Validator",
        "description": "Validator-Konfiguration und Staking-Parameter"
      },
      {
        "ext": ".atid",
        "name": "A-Town Identity",
        "description": "DID-, Wallet- und Agenten-Identitäten"
      },
      {
        "ext": ".atcsc",
        "name": "A-Town Smart Contract",
        "description": "Smart-Contract-Definitionen"
      },
      {
        "ext": ".atex",
        "name": "A-Town Execution Package",
        "description": "Deterministische Execution-Payload"
      },
      {
        "ext": ".atmsg",
        "name": "A-Town Message",
        "description": "Cross-Chain- oder Inter-Agent-Nachrichten"
      },
      {
        "ext": ".ator",
        "name": "A-Town Oracle",
        "description": "Oracle-Datenfeeds und Signaturen"
      },
      {
        "ext": ".atzk",
        "name": "A-Town Zero-Knowledge Proof",
        "description": "ZK-Proofs und Verifizierungsdaten"
      },
      {
        "ext": ".atm",
        "name": "A-Town Mempool",
        "description": "Mempool-Snapshot"
      },
      {
        "ext": ".atl",
        "name": "A-Town Ledger",
        "description": "Vollständiger Ledger-Export"
      },
      {
        "ext": ".atbk",
        "name": "A-Town Backup",
        "description": "Vollständiges Chain-Backup"
      },
      {
        "ext": ".atcfg",
        "name": "A-Town Configuration",
        "description": "Netzwerk- und Node-Konfiguration"
      }
    ]
  },
  {
    "name": "ATS v1.0 - AI & System Layer",
    "extensions": [
      {
        "ext": ".ats",
        "name": "A-Town System File",
        "description": "Allgemeine ATS-Systemdatei"
      },
      {
        "ext": ".atag",
        "name": "A-Town Agent",
        "description": "Agenten-Konfiguration"
      },
      {
        "ext": ".atwf",
        "name": "A-Town Workflow",
        "description": "Agenten-Workflow-Definition"
      },
      {
        "ext": ".atpol",
        "name": "A-Town Policy",
        "description": "Governance- und Compliance-Regeln"
      },
      {
        "ext": ".atsim",
        "name": "A-Town Simulation",
        "description": "Monte-Carlo- und RL-Simulationen"
      },
      {
        "ext": ".atkg",
        "name": "A-Town Knowledge Graph",
        "description": "Wissensgraphen"
      },
      {
        "ext": ".atrag",
        "name": "A-Town RAG Package",
        "description": "Retrieval-Augmented-Generation Daten"
      },
      {
        "ext": ".ateco",
        "name": "A-Town Economic Model",
        "description": "Tokenomics- und Treasury-Modelle"
      },
      {
        "ext": ".atrisk",
        "name": "A-Town Risk Model",
        "description": "Risikoanalysen"
      },
      {
        "ext": ".atai",
        "name": "A-Town AI Model",
        "description": "Trainierte KI-Modelle"
      },
      {
        "ext": ".atzkml",
        "name": "A-Town zkML Model",
        "description": "Verifizierbare KI-Inferenzmodelle"
      },
      {
        "ext": ".atgov",
        "name": "A-Town Governance Proposal",
        "description": "Governance-Vorschläge"
      },
      {
        "ext": ".atsec",
        "name": "A-Town Security Report",
        "description": "Audit- und Sicherheitsberichte"
      }
    ]
  },
  {
    "name": "ATC v1.0 - Developer Tools",
    "extensions": [
      {
        "ext": ".atapi",
        "name": "A-Town API",
        "description": "API-Spezifikation"
      },
      {
        "ext": ".atsdk",
        "name": "A-Town SDK",
        "description": "SDK-Paket"
      },
      {
        "ext": ".atmod",
        "name": "A-Town Module",
        "description": "Erweiterungsmodul"
      },
      {
        "ext": ".atplug",
        "name": "A-Town Plugin",
        "description": "Plugin"
      },
      {
        "ext": ".attest",
        "name": "A-Town Test",
        "description": "Testfälle"
      },
      {
        "ext": ".atdoc",
        "name": "A-Town Documentation",
        "description": "Dokumentation"
      },
      {
        "ext": ".atpkg",
        "name": "A-Town Package",
        "description": "Installierbares Paket"
      },
      {
        "ext": ".atupd",
        "name": "A-Town Update",
        "description": "Netzwerk-Update"
      },
      {
        "ext": ".atpatch",
        "name": "A-Town Patch",
        "description": "Hotfix/Patch"
      }
    ]
  },
  {
    "name": "ATC v1.1 - Network & Consensus",
    "extensions": [
      {
        "ext": ".atcons",
        "name": "Consensus File",
        "description": "Konsensparameter"
      },
      {
        "ext": ".atvote",
        "name": "Validator Vote",
        "description": "Validator-Abstimmungen"
      },
      {
        "ext": ".atprop",
        "name": "Consensus Proposal",
        "description": "Block- und Konsensvorschläge"
      },
      {
        "ext": ".atslash",
        "name": "Slashing Event",
        "description": "Strafereignisse"
      },
      {
        "ext": ".atstake",
        "name": "Stake Position",
        "description": "Staking-Daten"
      },
      {
        "ext": ".atreward",
        "name": "Reward Distribution",
        "description": "Belohnungsverteilung"
      },
      {
        "ext": ".atepoch",
        "name": "Epoch Snapshot",
        "description": "Epochenstatus"
      },
      {
        "ext": ".atfinal",
        "name": "Finalization Record",
        "description": "Finalisierte Blöcke"
      },
      {
        "ext": ".atpeer",
        "name": "Peer Registry",
        "description": "Peer-Informationen"
      },
      {
        "ext": ".atnode",
        "name": "Node Configuration",
        "description": "Node-Daten"
      }
    ]
  },
  {
    "name": "ATC v1.1 - Cryptography",
    "extensions": [
      {
        "ext": ".atsig",
        "name": "Digital Signature",
        "description": "Digitale Signaturen"
      },
      {
        "ext": ".atcert",
        "name": "Certificate",
        "description": "Zertifikate"
      },
      {
        "ext": ".athash",
        "name": "Hash Reference",
        "description": "Hash-Referenzen"
      },
      {
        "ext": ".atkey",
        "name": "Public Key",
        "description": "Öffentliche Schlüssel"
      },
      {
        "ext": ".atpriv",
        "name": "Private Key",
        "description": "Verschlüsselte Private-Key-Datei"
      },
      {
        "ext": ".atmulti",
        "name": "Multisig Config",
        "description": "Multisignatur-Konfiguration"
      },
      {
        "ext": ".atproof",
        "name": "Cryptographic Proof",
        "description": "Allgemeine kryptografische Beweise"
      },
      {
        "ext": ".atmerkle",
        "name": "Merkle Structure",
        "description": "Merkle-Strukturen"
      },
      {
        "ext": ".atcommit",
        "name": "Commitment",
        "description": "Commitments"
      },
      {
        "ext": ".atverify",
        "name": "Verification Data",
        "description": "Verifikationsdaten"
      }
    ]
  },
  {
    "name": "ATC v1.1 - Smart Contract Layer",
    "extensions": [
      {
        "ext": ".atabi",
        "name": "Contract Interface",
        "description": "Contract Interface"
      },
      {
        "ext": ".atwasm",
        "name": "WASM Smart Contract",
        "description": "WASM Smart Contract"
      },
      {
        "ext": ".atevm",
        "name": "EVM Contract",
        "description": "EVM-kompatibler Contract"
      },
      {
        "ext": ".atbyte",
        "name": "Bytecode",
        "description": "Bytecode"
      },
      {
        "ext": ".atexec",
        "name": "Execution Log",
        "description": "Ausführungsprotokoll"
      },
      {
        "ext": ".atcall",
        "name": "Contract Call",
        "description": "Smart-Contract-Aufruf"
      },
      {
        "ext": ".atevent",
        "name": "Contract Event",
        "description": "Contract Events"
      },
      {
        "ext": ".atgas",
        "name": "Gas Report",
        "description": "Gas-Report"
      },
      {
        "ext": ".atvm",
        "name": "VM Config",
        "description": "VM-Konfiguration"
      }
    ]
  },
  {
    "name": "ATC v1.1 - Oracle & Data Feeds",
    "extensions": [
      {
        "ext": ".atfeed",
        "name": "Price Feed",
        "description": "Preisfeed"
      },
      {
        "ext": ".atagg",
        "name": "Aggregated Data",
        "description": "Aggregierte Oracle-Daten"
      },
      {
        "ext": ".atmarket",
        "name": "Market Info",
        "description": "Marktinformationen"
      },
      {
        "ext": ".atindex",
        "name": "Market Index",
        "description": "Marktindex"
      },
      {
        "ext": ".atprice",
        "name": "Price Snapshot",
        "description": "Preis-Snapshot"
      },
      {
        "ext": ".atliq",
        "name": "Liquidity Data",
        "description": "Liquiditätsdaten"
      },
      {
        "ext": ".atvol",
        "name": "Volatility Data",
        "description": "Volatilitätsdaten"
      }
    ]
  },
  {
    "name": "ATS v1.1 - AI & Machine Learning",
    "extensions": [
      {
        "ext": ".atnn",
        "name": "Neural Network",
        "description": "Neuronales Netzwerk"
      },
      {
        "ext": ".attrain",
        "name": "Training Data",
        "description": "Trainingsdaten"
      },
      {
        "ext": ".atinfer",
        "name": "Inference Data",
        "description": "Inferenzdaten"
      },
      {
        "ext": ".atmodel",
        "name": "Model Weights",
        "description": "Modellgewichte"
      },
      {
        "ext": ".atagent",
        "name": "Agent State",
        "description": "Agentenstatus"
      },
      {
        "ext": ".atmemory",
        "name": "Agent Memory",
        "description": "Agentenspeicher"
      },
      {
        "ext": ".atplan",
        "name": "Action Plan",
        "description": "Aktionsplan"
      },
      {
        "ext": ".atreason",
        "name": "Reasoning Log",
        "description": "Reasoning-Protokoll"
      },
      {
        "ext": ".atlearn",
        "name": "Learning State",
        "description": "Lernzustand"
      },
      {
        "ext": ".atpolicyml",
        "name": "AI Policy",
        "description": "KI-Policy"
      }
    ]
  },
  {
    "name": "ATS v1.1 - Simulation Layer",
    "extensions": [
      {
        "ext": ".atmcs",
        "name": "Monte-Carlo Sim",
        "description": "Monte-Carlo-Simulation"
      },
      {
        "ext": ".atrl",
        "name": "Reinforcement Learning",
        "description": "Reinforcement Learning"
      },
      {
        "ext": ".atscenario",
        "name": "Simulation Scenario",
        "description": "Simulationsszenario"
      },
      {
        "ext": ".atforecast",
        "name": "Forecast Model",
        "description": "Prognosemodell"
      },
      {
        "ext": ".atstress",
        "name": "Stress Test",
        "description": "Stresstest"
      },
      {
        "ext": ".atdigitaltwin",
        "name": "Digital Twin",
        "description": "Digital Twin"
      },
      {
        "ext": ".atopt",
        "name": "Optimization Log",
        "description": "Optimierungslauf"
      },
      {
        "ext": ".atpredict",
        "name": "Predictive Model",
        "description": "Vorhersagemodell"
      }
    ]
  },
  {
    "name": "ATC v1.1 - Governance",
    "extensions": [
      {
        "ext": ".atdao",
        "name": "DAO Data",
        "description": "DAO-Daten"
      },
      {
        "ext": ".atlaw",
        "name": "Governance Rules",
        "description": "Governance-Regeln"
      },
      {
        "ext": ".atrule",
        "name": "Guidelines",
        "description": "Richtlinien"
      },
      {
        "ext": ".atconstitution",
        "name": "Network Constitution",
        "description": "Netzwerkverfassung"
      },
      {
        "ext": ".atreferendum",
        "name": "Referendum",
        "description": "Referenden"
      },
      {
        "ext": ".atvoteroll",
        "name": "Voter Roll",
        "description": "Stimmberechtigte"
      },
      {
        "ext": ".atproposal",
        "name": "Governance Proposal",
        "description": "Governance-Vorschlag"
      },
      {
        "ext": ".atdecision",
        "name": "Resolution",
        "description": "Beschluss"
      }
    ]
  },
  {
    "name": "ATC v1.1 - Treasury & Economy",
    "extensions": [
      {
        "ext": ".attreasury",
        "name": "Treasury State",
        "description": "Treasury-Zustand"
      },
      {
        "ext": ".atemission",
        "name": "Token Emission",
        "description": "Tokenemission"
      },
      {
        "ext": ".atinflation",
        "name": "Inflation Model",
        "description": "Inflationsmodell"
      },
      {
        "ext": ".atburn",
        "name": "Burn Protocol",
        "description": "Burn-Protokoll"
      },
      {
        "ext": ".atreserve",
        "name": "Reserves",
        "description": "Reserven"
      },
      {
        "ext": ".atbudget",
        "name": "Budget Plan",
        "description": "Budgetplanung"
      },
      {
        "ext": ".atyield",
        "name": "Yield Model",
        "description": "Renditemodell"
      },
      {
        "ext": ".atloan",
        "name": "Loan Model",
        "description": "Kreditmodell"
      },
      {
        "ext": ".atbond",
        "name": "Bond Model",
        "description": "Anleihenmodell"
      }
    ]
  },
  {
    "name": "ATC v1.1 - Interoperability",
    "extensions": [
      {
        "ext": ".atbridge",
        "name": "Bridge Data",
        "description": "Bridge-Daten"
      },
      {
        "ext": ".atrelay",
        "name": "Relayer",
        "description": "Relayer"
      },
      {
        "ext": ".atxcc",
        "name": "Cross-Chain Comms",
        "description": "Cross-Chain-Kommunikation"
      },
      {
        "ext": ".atroute",
        "name": "Routing Config",
        "description": "Routing"
      },
      {
        "ext": ".atswap",
        "name": "Asset Swap",
        "description": "Asset-Swap"
      },
      {
        "ext": ".atwrap",
        "name": "Wrapped Asset",
        "description": "Wrapped Assets"
      },
      {
        "ext": ".atinterop",
        "name": "Interop Data",
        "description": "Interoperabilitätsdaten"
      }
    ]
  },
  {
    "name": "ATC v1.1 - DevOps",
    "extensions": [
      {
        "ext": ".atbuild",
        "name": "Build Artifact",
        "description": "Build-Artefakte"
      },
      {
        "ext": ".atdeploy",
        "name": "Deployment Script",
        "description": "Deployment"
      },
      {
        "ext": ".atpipeline",
        "name": "CI/CD Pipeline",
        "description": "CI/CD Pipeline"
      },
      {
        "ext": ".atcontainer",
        "name": "Container Definition",
        "description": "Containerdefinition"
      },
      {
        "ext": ".atmonitor",
        "name": "Monitoring Config",
        "description": "Monitoring"
      },
      {
        "ext": ".atmetric",
        "name": "Metrics",
        "description": "Metriken"
      },
      {
        "ext": ".atlog",
        "name": "Logs",
        "description": "Logs"
      },
      {
        "ext": ".attrace",
        "name": "Trace Log",
        "description": "Tracing"
      },
      {
        "ext": ".atdebug",
        "name": "Debug Data",
        "description": "Debugging"
      },
      {
        "ext": ".atrelease",
        "name": "Release Package",
        "description": "Release-Paket"
      }
    ]
  },
  {
    "name": "ATS v1.1 - Sovereign AI Layer",
    "extensions": [
      {
        "ext": ".atsovereign",
        "name": "Sovereign AI State",
        "description": "Sovereign-AI-Zustand"
      },
      {
        "ext": ".atmeta",
        "name": "Metacognition Log",
        "description": "Metakognition"
      },
      {
        "ext": ".atautonomy",
        "name": "Autonomy Level",
        "description": "Autonomie-Level"
      },
      {
        "ext": ".atobjective",
        "name": "Network Objectives",
        "description": "Netzwerkziele"
      },
      {
        "ext": ".atmission",
        "name": "Mission Parameters",
        "description": "Langfristige Missionsparameter"
      },
      {
        "ext": ".atethics",
        "name": "AI Ethics Rules",
        "description": "KI-Ethikregeln"
      },
      {
        "ext": ".atalignment",
        "name": "Alignment Model",
        "description": "Alignment-Modell"
      },
      {
        "ext": ".atcollective",
        "name": "Multi-Agent Collective",
        "description": "Multi-Agent-Kollektiv"
      },
      {
        "ext": ".atswarm",
        "name": "Swarm Intelligence",
        "description": "Swarm-Intelligence"
      },
      {
        "ext": ".atworld",
        "name": "World Model",
        "description": "Weltmodell"
      }
    ]
  },
  {
    "name": "ATC v1.2 - Asset Layer",
    "extensions": [
      {
        "ext": ".atasset",
        "name": "Digital Asset",
        "description": "Digitales Asset"
      },
      {
        "ext": ".attoken",
        "name": "Token Definition",
        "description": "Token-Definition"
      },
      {
        "ext": ".atcoin",
        "name": "Native Coin Data",
        "description": "Native Coin-Daten"
      },
      {
        "ext": ".atnft",
        "name": "NFT Object",
        "description": "NFT-Objekt"
      },
      {
        "ext": ".atcollection",
        "name": "NFT Collection",
        "description": "NFT-Sammlung"
      },
      {
        "ext": ".atfraction",
        "name": "Fractional NFT",
        "description": "Fractional NFT"
      },
      {
        "ext": ".atreal",
        "name": "Real-World Asset",
        "description": "Real-World-Asset"
      },
      {
        "ext": ".atcommodity",
        "name": "Commodity Token",
        "description": "Rohstoff-Token"
      },
      {
        "ext": ".atsecurity",
        "name": "Security Token",
        "description": "Security Token"
      },
      {
        "ext": ".atstable",
        "name": "Stablecoin Data",
        "description": "Stablecoin-Daten"
      }
    ]
  },
  {
    "name": "ATC v1.2 - DeFi Layer",
    "extensions": [
      {
        "ext": ".atdex",
        "name": "DEX Config",
        "description": "DEX-Konfiguration"
      },
      {
        "ext": ".atpool",
        "name": "Liquidity Pool",
        "description": "Liquiditätspool"
      },
      {
        "ext": ".atfarm",
        "name": "Yield Farm",
        "description": "Yield Farming"
      },
      {
        "ext": ".atlp",
        "name": "LP Position",
        "description": "LP-Position"
      },
      {
        "ext": ".atamm",
        "name": "AMM State",
        "description": "AMM-Zustand"
      },
      {
        "ext": ".atlending",
        "name": "Lending Protocol",
        "description": "Lending-Protokoll"
      },
      {
        "ext": ".atborrow",
        "name": "Borrow Position",
        "description": "Kreditposition"
      },
      {
        "ext": ".atvault",
        "name": "Vault Data",
        "description": "Vault-Daten"
      },
      {
        "ext": ".atoption",
        "name": "Options Contract",
        "description": "Optionskontrakt"
      },
      {
        "ext": ".atfuture",
        "name": "Futures Contract",
        "description": "Futures-Kontrakt"
      }
    ]
  },
  {
    "name": "ATC v1.2 - Payments",
    "extensions": [
      {
        "ext": ".atpay",
        "name": "Payment Order",
        "description": "Zahlungsauftrag"
      },
      {
        "ext": ".atinvoice",
        "name": "Invoice",
        "description": "Rechnung"
      },
      {
        "ext": ".atreceipt",
        "name": "Payment Receipt",
        "description": "Zahlungsbeleg"
      },
      {
        "ext": ".atsettle",
        "name": "Settlement Log",
        "description": "Settlement-Protokoll"
      },
      {
        "ext": ".atchannel",
        "name": "Payment Channel",
        "description": "Payment Channel"
      },
      {
        "ext": ".atln",
        "name": "Lightning Channel",
        "description": "Lightning-ähnlicher Kanal"
      },
      {
        "ext": ".atstream",
        "name": "Streaming Payment",
        "description": "Streaming Payment"
      },
      {
        "ext": ".atescrow",
        "name": "Escrow Contract",
        "description": "Treuhandvertrag"
      }
    ]
  },
  {
    "name": "ATC v1.2 - Identity & Reputation",
    "extensions": [
      {
        "ext": ".atrep",
        "name": "Reputation Score",
        "description": "Reputation"
      },
      {
        "ext": ".atkyd",
        "name": "Know-Your-Decentralization",
        "description": "Know-Your-Decentralization"
      },
      {
        "ext": ".atcred",
        "name": "Credentials",
        "description": "Credentials"
      },
      {
        "ext": ".atbadge",
        "name": "Badge System",
        "description": "Badge-System"
      },
      {
        "ext": ".attrust",
        "name": "Trust Score",
        "description": "Vertrauensscore"
      },
      {
        "ext": ".atproofid",
        "name": "Identity Proof",
        "description": "Identity Proof"
      },
      {
        "ext": ".atprofile",
        "name": "User Profile",
        "description": "Profil"
      },
      {
        "ext": ".atpassport",
        "name": "Digital Passport",
        "description": "Digitaler Pass"
      }
    ]
  },
  {
    "name": "ATC v1.2 - Communications",
    "extensions": [
      {
        "ext": ".atchat",
        "name": "Chat Message",
        "description": "Chat-Nachrichten"
      },
      {
        "ext": ".atmail",
        "name": "Decentralized Mail",
        "description": "Dezentrale Mail"
      },
      {
        "ext": ".atforum",
        "name": "Forum Data",
        "description": "Forum-Daten"
      },
      {
        "ext": ".atsocial",
        "name": "Social Network Data",
        "description": "Social-Network-Daten"
      },
      {
        "ext": ".atpost",
        "name": "Social Post",
        "description": "Beitrag"
      },
      {
        "ext": ".atcomment",
        "name": "Comment",
        "description": "Kommentar"
      },
      {
        "ext": ".atmedia",
        "name": "Media Reference",
        "description": "Medienreferenz"
      },
      {
        "ext": ".atnotify",
        "name": "Notification",
        "description": "Benachrichtigung"
      }
    ]
  },
  {
    "name": "ATC v1.2 - Storage",
    "extensions": [
      {
        "ext": ".atfs",
        "name": "Filesystem Object",
        "description": "Dateisystemobjekt"
      },
      {
        "ext": ".atblob",
        "name": "Binary Blob",
        "description": "Binärdaten"
      },
      {
        "ext": ".atarchive",
        "name": "Archive",
        "description": "Archiv"
      },
      {
        "ext": ".atstorage",
        "name": "Storage Management",
        "description": "Speicherverwaltung"
      },
      {
        "ext": ".atshard",
        "name": "Shard Data",
        "description": "Shard-Daten"
      },
      {
        "ext": ".atreplica",
        "name": "Replication Data",
        "description": "Replikation"
      },
      {
        "ext": ".atcache",
        "name": "Cache File",
        "description": "Cache-Datei"
      },
      {
        "ext": ".atindexdb",
        "name": "Index Structure",
        "description": "Indexstruktur"
      }
    ]
  },
  {
    "name": "ATS v1.2 - Metaverse & Digital Twin",
    "extensions": [
      {
        "ext": ".atworldstate",
        "name": "World State",
        "description": "Weltzustand"
      },
      {
        "ext": ".atavatar",
        "name": "Avatar",
        "description": "Avatar"
      },
      {
        "ext": ".atland",
        "name": "Virtual Land",
        "description": "Virtuelles Land"
      },
      {
        "ext": ".atobject",
        "name": "Digital Object",
        "description": "Digitales Objekt"
      },
      {
        "ext": ".atscene",
        "name": "Scene",
        "description": "Szene"
      },
      {
        "ext": ".atphysics",
        "name": "Physics Model",
        "description": "Physikmodell"
      },
      {
        "ext": ".atmetaverse",
        "name": "Metaverse State",
        "description": "Metaverse-Zustand"
      },
      {
        "ext": ".atdigitalasset",
        "name": "Virtual Asset",
        "description": "Virtuelles Asset"
      }
    ]
  },
  {
    "name": "ATS v1.2 - Agent Economy",
    "extensions": [
      {
        "ext": ".attask",
        "name": "Agent Task",
        "description": "Agentenaufgabe"
      },
      {
        "ext": ".atjob",
        "name": "Executable Job",
        "description": "Auszuführender Auftrag"
      },
      {
        "ext": ".atworkflowrun",
        "name": "Workflow Execution",
        "description": "Workflow-Ausführung"
      },
      {
        "ext": ".atbounty",
        "name": "Bounty Order",
        "description": "Belohnungsauftrag"
      },
      {
        "ext": ".atservice",
        "name": "Agent Service",
        "description": "Agentenservice"
      },
      {
        "ext": ".atmarketplace",
        "name": "Agent Marketplace",
        "description": "Agenten-Marktplatz"
      },
      {
        "ext": ".atnegotiation",
        "name": "Agent Negotiation",
        "description": "Agentenverhandlung"
      },
      {
        "ext": ".atcontractai",
        "name": "AI Contract",
        "description": "AI-Vertrag"
      },
      {
        "ext": ".atcoordination",
        "name": "Multi-Agent Coordination",
        "description": "Multi-Agent-Koordination"
      }
    ]
  },
  {
    "name": "ATS v1.2 - Security & Forensics",
    "extensions": [
      {
        "ext": ".atalert",
        "name": "Security Alert",
        "description": "Sicherheitsalarm"
      },
      {
        "ext": ".atincident",
        "name": "Security Incident",
        "description": "Sicherheitsvorfall"
      },
      {
        "ext": ".atforensic",
        "name": "Forensic Data",
        "description": "Forensische Daten"
      },
      {
        "ext": ".atanomaly",
        "name": "Anomaly Detection",
        "description": "Anomalieerkennung"
      },
      {
        "ext": ".atexploit",
        "name": "Exploit Analysis",
        "description": "Exploit-Analyse"
      },
      {
        "ext": ".atthreat",
        "name": "Threat Model",
        "description": "Bedrohungsmodell"
      },
      {
        "ext": ".atriskscore",
        "name": "Risk Score",
        "description": "Risikobewertung"
      },
      {
        "ext": ".atcompliance",
        "name": "Compliance Report",
        "description": "Compliance-Bericht"
      }
    ]
  },
  {
    "name": "ATC v1.2 - Quant & Economic Simulation",
    "extensions": [
      {
        "ext": ".atmacro",
        "name": "Macro-Economic Model",
        "description": "Makroökonomisches Modell"
      },
      {
        "ext": ".atmicro",
        "name": "Micro-Economic Model",
        "description": "Mikroökonomisches Modell"
      },
      {
        "ext": ".atorderbook",
        "name": "Orderbook",
        "description": "Orderbuch"
      },
      {
        "ext": ".attrade",
        "name": "Trade Data",
        "description": "Handelsdaten"
      },
      {
        "ext": ".atalpha",
        "name": "Alpha Model",
        "description": "Alpha-Modell"
      },
      {
        "ext": ".atfactor",
        "name": "Factor Model",
        "description": "Faktor-Modell"
      },
      {
        "ext": ".atportfolio",
        "name": "Portfolio",
        "description": "Portfolio"
      },
      {
        "ext": ".athedge",
        "name": "Hedging Model",
        "description": "Hedging-Modell"
      },
      {
        "ext": ".atmarketmaker",
        "name": "Market Maker State",
        "description": "Market-Maker-Zustand"
      }
    ]
  },
  {
    "name": "ATC v1.2 - Autonomous Network Governance",
    "extensions": [
      {
        "ext": ".atconstitutionv2",
        "name": "Extended Constitution",
        "description": "Erweiterte Verfassung"
      },
      {
        "ext": ".atpolicyengine",
        "name": "Policy Engine",
        "description": "Policy Engine"
      },
      {
        "ext": ".atdecisiontree",
        "name": "Governance Decision Tree",
        "description": "Governance-Entscheidungsbaum"
      },
      {
        "ext": ".atconsensusai",
        "name": "AI Consensus Data",
        "description": "KI-Konsensdaten"
      },
      {
        "ext": ".atstrategy",
        "name": "Network Strategy",
        "description": "Netzwerkstrategie"
      },
      {
        "ext": ".atobjectivefn",
        "name": "Optimization Function",
        "description": "Optimierungsfunktion"
      },
      {
        "ext": ".atcollectivevote",
        "name": "Collective Vote",
        "description": "Kollektive Abstimmung"
      },
      {
        "ext": ".atgovernor",
        "name": "Governance Agent",
        "description": "Governance-Agent"
      }
    ]
  },
  {
    "name": "ATOS - System Core",
    "extensions": [
      {
        "ext": ".atsys",
        "name": "System Core",
        "description": "Systemkern"
      },
      {
        "ext": ".atkernel",
        "name": "OS Kernel",
        "description": "Betriebssystem-Kernel"
      },
      {
        "ext": ".atboot",
        "name": "Boot File",
        "description": "Boot-Datei"
      },
      {
        "ext": ".atruntime",
        "name": "Runtime Environment",
        "description": "Runtime"
      },
      {
        "ext": ".atregistry",
        "name": "System Registry",
        "description": "Registry"
      },
      {
        "ext": ".atmanifest",
        "name": "System Manifest",
        "description": "Manifest"
      },
      {
        "ext": ".atmodule",
        "name": "System Module",
        "description": "Modul"
      },
      {
        "ext": ".atengine",
        "name": "Core Engine",
        "description": "Engine"
      },
      {
        "ext": ".atservicecore",
        "name": "Core Service",
        "description": "Kerndienst"
      },
      {
        "ext": ".atroot",
        "name": "Network Root Config",
        "description": "Netzwerk-Root-Konfiguration"
      }
    ]
  },
  {
    "name": "ATOS - System & Kernel Layer",
    "extensions": [
      {
        "ext": ".atdriver",
        "name": "Device Driver",
        "description": "Gerätetreiber"
      },
      {
        "ext": ".atdevice",
        "name": "Device Definition",
        "description": "Gerätebeschreibung"
      },
      {
        "ext": ".atbootloader",
        "name": "Bootloader",
        "description": "Bootloader"
      },
      {
        "ext": ".atfirmware",
        "name": "Firmware",
        "description": "Firmware"
      },
      {
        "ext": ".athal",
        "name": "Hardware Abstraction Layer",
        "description": "Hardware Abstraction Layer"
      },
      {
        "ext": ".atscheduler",
        "name": "Process Scheduler",
        "description": "Prozess-Scheduler"
      },
      {
        "ext": ".atinterrupt",
        "name": "Interrupt Table",
        "description": "Interrupt-Tabelle"
      },
      {
        "ext": ".atmemorymap",
        "name": "Memory Management",
        "description": "Speicherverwaltung"
      },
      {
        "ext": ".atprocess",
        "name": "Process Definition",
        "description": "Prozessdefinition"
      }
    ]
  },
  {
    "name": "ATOS - Applications",
    "extensions": [
      {
        "ext": ".atapp",
        "name": "Installed Application",
        "description": "Installierte Anwendung"
      },
      {
        "ext": ".atexe",
        "name": "Executable binary",
        "description": "Ausführbare Datei"
      },
      {
        "ext": ".atbin",
        "name": "Binary file",
        "description": "Binärdatei"
      },
      {
        "ext": ".atpkgx",
        "name": "Installation Package",
        "description": "Installationspaket"
      },
      {
        "ext": ".atbundle",
        "name": "App Bundle",
        "description": "Anwendungspaket"
      },
      {
        "ext": ".atportable",
        "name": "Portable App",
        "description": "Portable Anwendung"
      },
      {
        "ext": ".atservice",
        "name": "System Service",
        "description": "Systemdienst"
      },
      {
        "ext": ".atdaemon",
        "name": "Background Daemon",
        "description": "Hintergrunddienst"
      },
      {
        "ext": ".atplugin",
        "name": "App Extension",
        "description": "Erweiterung"
      },
      {
        "ext": ".atwidget",
        "name": "Desktop Widget",
        "description": "Desktop-Widget"
      }
    ]
  },
  {
    "name": "ATOS - User Interface",
    "extensions": [
      {
        "ext": ".atdesktop",
        "name": "Desktop Configuration",
        "description": "Desktop-Konfiguration"
      },
      {
        "ext": ".atwindow",
        "name": "Window Definition",
        "description": "Fensterdefinition"
      },
      {
        "ext": ".attheme",
        "name": "System Theme",
        "description": "System-Theme"
      },
      {
        "ext": ".aticon",
        "name": "Icon File",
        "description": "Symboldatei"
      },
      {
        "ext": ".atfont",
        "name": "Font File",
        "description": "Schriftart"
      },
      {
        "ext": ".atcursor",
        "name": "Mouse Cursor",
        "description": "Mauszeiger"
      },
      {
        "ext": ".atlayout",
        "name": "UI Layout",
        "description": "UI-Layout"
      },
      {
        "ext": ".atdock",
        "name": "Dock Configuration",
        "description": "Dock-Konfiguration"
      },
      {
        "ext": ".atmenu",
        "name": "Menu Definition",
        "description": "Menüdefinition"
      },
      {
        "ext": ".atworkspace",
        "name": "Workspace Configuration",
        "description": "Arbeitsbereich"
      }
    ]
  },
  {
    "name": "ATOS - User Management",
    "extensions": [
      {
        "ext": ".atuser",
        "name": "User Account",
        "description": "Benutzerkonto"
      },
      {
        "ext": ".atgroup",
        "name": "User Group",
        "description": "Benutzergruppe"
      },
      {
        "ext": ".atrole",
        "name": "Role Model",
        "description": "Rollenmodell"
      },
      {
        "ext": ".atpermission",
        "name": "Permissions",
        "description": "Berechtigungen"
      },
      {
        "ext": ".atsession",
        "name": "User Session",
        "description": "Sitzung"
      },
      {
        "ext": ".atprofile",
        "name": "User Profile",
        "description": "Benutzerprofil"
      },
      {
        "ext": ".atauth",
        "name": "Authentication Data",
        "description": "Authentifizierung"
      },
      {
        "ext": ".atvault",
        "name": "Secure Vault",
        "description": "Sichere Benutzerdaten"
      },
      {
        "ext": ".atcredential",
        "name": "Credentials",
        "description": "Zugangsdaten"
      },
      {
        "ext": ".atpolicyuser",
        "name": "User Policies",
        "description": "Benutzerregeln"
      }
    ]
  },
  {
    "name": "ATOS - Networking",
    "extensions": [
      {
        "ext": ".atnet",
        "name": "Network Profile",
        "description": "Netzwerkprofil"
      },
      {
        "ext": ".atwifi",
        "name": "WLAN Configuration",
        "description": "WLAN-Konfiguration"
      },
      {
        "ext": ".atvpn",
        "name": "VPN Profile",
        "description": "VPN-Profil"
      },
      {
        "ext": ".atdns",
        "name": "DNS Configuration",
        "description": "DNS-Konfiguration"
      },
      {
        "ext": ".atroute",
        "name": "Routing Table",
        "description": "Routing-Tabelle"
      },
      {
        "ext": ".atfirewall",
        "name": "Firewall Rules",
        "description": "Firewall-Regeln"
      },
      {
        "ext": ".atproxy",
        "name": "Proxy Settings",
        "description": "Proxy-Einstellungen"
      },
      {
        "ext": ".atsocket",
        "name": "Socket Definition",
        "description": "Socket-Definition"
      },
      {
        "ext": ".atcluster",
        "name": "Cluster Network",
        "description": "Cluster-Netzwerk"
      },
      {
        "ext": ".atmesh",
        "name": "Mesh Network",
        "description": "Mesh-Netzwerk"
      }
    ]
  },
  {
    "name": "ATOS - Filesystem",
    "extensions": [
      {
        "ext": ".atfsmeta",
        "name": "FS Metadata",
        "description": "Dateisystem-Metadaten"
      },
      {
        "ext": ".atinode",
        "name": "Inode Structure",
        "description": "Inode-Struktur"
      },
      {
        "ext": ".atvolume",
        "name": "Storage Volume",
        "description": "Datenträger"
      },
      {
        "ext": ".atpartition",
        "name": "Partition",
        "description": "Partition"
      },
      {
        "ext": ".atmount",
        "name": "Mount Point",
        "description": "Mount-Punkt"
      },
      {
        "ext": ".atjournal",
        "name": "Journaling Data",
        "description": "Journaling-Daten"
      },
      {
        "ext": ".atsnapshot",
        "name": "FS Snapshot",
        "description": "Dateisystem-Snapshot"
      },
      {
        "ext": ".atquota",
        "name": "Storage Quota",
        "description": "Speicherquoten"
      },
      {
        "ext": ".atrecovery",
        "name": "Recovery Payload",
        "description": "Wiederherstellung"
      },
      {
        "ext": ".atbackupset",
        "name": "Backup Set",
        "description": "Backup-Satz"
      }
    ]
  },
  {
    "name": "ATOS - Multimedia",
    "extensions": [
      {
        "ext": ".atimage",
        "name": "Image File",
        "description": "Bilddatei"
      },
      {
        "ext": ".atvideo",
        "name": "Video File",
        "description": "Videodatei"
      },
      {
        "ext": ".ataudio",
        "name": "Audio File",
        "description": "Audiodatei"
      },
      {
        "ext": ".atstreammedia",
        "name": "Media Stream",
        "description": "Livestream"
      },
      {
        "ext": ".at3d",
        "name": "3D Model",
        "description": "3D-Modell"
      },
      {
        "ext": ".atanimation",
        "name": "Animation File",
        "description": "Animation"
      },
      {
        "ext": ".atholo",
        "name": "Holographic Data",
        "description": "Holografische Daten"
      },
      {
        "ext": ".atvr",
        "name": "VR Space",
        "description": "Virtual-Reality-Datei"
      },
      {
        "ext": ".atar",
        "name": "AR Overlay",
        "description": "Augmented-Reality-Datei"
      },
      {
        "ext": ".atmediaindex",
        "name": "Media Index",
        "description": "Medienindex"
      }
    ]
  },
  {
    "name": "ATOS - Development",
    "extensions": [
      {
        "ext": ".atsource",
        "name": "Source Code",
        "description": "Quellcode"
      },
      {
        "ext": ".atproject",
        "name": "Project File",
        "description": "Projektdatei"
      },
      {
        "ext": ".atworkspaceproj",
        "name": "Workspace Config",
        "description": "Workspace"
      },
      {
        "ext": ".atlibrary",
        "name": "Library",
        "description": "Bibliothek"
      },
      {
        "ext": ".atobject",
        "name": "Object File",
        "description": "Objektdatei"
      },
      {
        "ext": ".atdebugsym",
        "name": "Debug Symbols",
        "description": "Debug-Symbole"
      },
      {
        "ext": ".atcompile",
        "name": "Compiler Config",
        "description": "Compiler-Konfiguration"
      },
      {
        "ext": ".atbuildcfg",
        "name": "Build Config",
        "description": "Build-Konfiguration"
      },
      {
        "ext": ".attestcase",
        "name": "Test Case",
        "description": "Testfall"
      },
      {
        "ext": ".atpackage",
        "name": "Developer Package",
        "description": "Entwicklerpaket"
      }
    ]
  },
  {
    "name": "ATOS - Cloud & Distributed",
    "extensions": [
      {
        "ext": ".atcloud",
        "name": "Cloud Config",
        "description": "Cloud-Konfiguration"
      },
      {
        "ext": ".atsync",
        "name": "Sync State",
        "description": "Synchronisation"
      },
      {
        "ext": ".atedge",
        "name": "Edge Node Config",
        "description": "Edge-Knoten"
      },
      {
        "ext": ".atdistributed",
        "name": "Distributed Resource",
        "description": "Verteilte Ressource"
      },
      {
        "ext": ".atreplication",
        "name": "Replication Protocol",
        "description": "Replikation"
      },
      {
        "ext": ".atnodegroup",
        "name": "Node Group",
        "description": "Node-Gruppe"
      },
      {
        "ext": ".atclusterstate",
        "name": "Cluster State",
        "description": "Clusterstatus"
      },
      {
        "ext": ".atglobalstate",
        "name": "Global State",
        "description": "Globaler Zustand"
      },
      {
        "ext": ".atcdn",
        "name": "CDN Config",
        "description": "Content Distribution"
      },
      {
        "ext": ".atmirror",
        "name": "Server Mirror",
        "description": "Spiegelserver"
      }
    ]
  },
  {
    "name": "ATOS - AI-Native OS (ATS Integration)",
    "extensions": [
      {
        "ext": ".atassistant",
        "name": "AI Assistant Config",
        "description": "KI-Assistent"
      },
      {
        "ext": ".atagentcore",
        "name": "Agent Core",
        "description": "Agentenkern"
      },
      {
        "ext": ".atreasoning",
        "name": "Reasoning Model",
        "description": "Schlussfolgerungsmodell"
      },
      {
        "ext": ".atplanner",
        "name": "Planning Module",
        "description": "Planungsmodul"
      },
      {
        "ext": ".atdecision",
        "name": "Decision Matrices",
        "description": "Entscheidungsdaten"
      },
      {
        "ext": ".atcontext",
        "name": "Context Storage",
        "description": "Kontextspeicher"
      },
      {
        "ext": ".atknowledge",
        "name": "Knowledge Base",
        "description": "Wissensbasis"
      },
      {
        "ext": ".atmemorygraph",
        "name": "Memory Graph",
        "description": "Gedächtnisgraph"
      },
      {
        "ext": ".atcognition",
        "name": "Cognition Model",
        "description": "Kognitionsmodell"
      },
      {
        "ext": ".atautonomycore",
        "name": "Autonomy Core",
        "description": "Autonomiekern"
      }
    ]
  },
  {
    "name": "ATOS v3.0 - Kernel Extensions",
    "extensions": [
      {
        "ext": ".atthread",
        "name": "Thread Object",
        "description": "Thread-Objekt"
      },
      {
        "ext": ".atcpu",
        "name": "CPU Configuration",
        "description": "CPU-Konfiguration"
      },
      {
        "ext": ".atcore",
        "name": "Core Allocation",
        "description": "Prozessorkern-Zuweisung"
      },
      {
        "ext": ".atnuma",
        "name": "NUMA Setup",
        "description": "NUMA-Speicherstruktur"
      },
      {
        "ext": ".atcachemap",
        "name": "CPU Cache Mapping",
        "description": "CPU-Cache-Mapping"
      },
      {
        "ext": ".atio",
        "name": "I/O Controller Schema",
        "description": "I/O-Controller"
      },
      {
        "ext": ".atdma",
        "name": "DMA Config",
        "description": "DMA-Konfiguration"
      },
      {
        "ext": ".atbus",
        "name": "System Bus Def",
        "description": "Systembus"
      },
      {
        "ext": ".atpci",
        "name": "PCI Devices",
        "description": "PCI-Geräte"
      },
      {
        "ext": ".atusb",
        "name": "USB Devices",
        "description": "USB-Geräte"
      }
    ]
  },
  {
    "name": "ATOS v3.0 - Hardware Abstraction",
    "extensions": [
      {
        "ext": ".atgpu",
        "name": "GPU Driver Node",
        "description": "GPU-Treiber"
      },
      {
        "ext": ".atnpu",
        "name": "NPU Descriptor",
        "description": "Neural Processing Unit"
      },
      {
        "ext": ".atfpga",
        "name": "FPGA Definition",
        "description": "FPGA-Gerät"
      },
      {
        "ext": ".atasic",
        "name": "ASIC Definition",
        "description": "ASIC-Gerät"
      },
      {
        "ext": ".atsensor",
        "name": "Sensor Profile",
        "description": "Sensor"
      },
      {
        "ext": ".atcamera",
        "name": "Camera Profile",
        "description": "Kamera"
      },
      {
        "ext": ".atdisplay",
        "name": "Display Profile",
        "description": "Display"
      },
      {
        "ext": ".atbattery",
        "name": "Battery Management",
        "description": "Akku"
      },
      {
        "ext": ".atpower",
        "name": "Power Management",
        "description": "Energieverwaltung"
      },
      {
        "ext": ".atthermal",
        "name": "Thermal Control",
        "description": "Thermische Steuerung"
      }
    ]
  },
  {
    "name": "ATOS v4.0 - Kernel Mods & LL Libs",
    "extensions": [
      {
        "ext": ".atkm",
        "name": "Kernel Module",
        "description": "Kernel-Modul"
      },
      {
        "ext": ".atko",
        "name": "Dynamic Kernel Object",
        "description": "Dynamisches Kernel-Objekt"
      },
      {
        "ext": ".atso",
        "name": "Shared OS Object",
        "description": "Shared Object Library"
      },
      {
        "ext": ".atdll",
        "name": "Dynamic Linked Library",
        "description": "Dynamische Bibliothek"
      },
      {
        "ext": ".atapi",
        "name": "API Reference Library",
        "description": "API-Bibliothek"
      },
      {
        "ext": ".atruntimecore",
        "name": "Base Runtime Component",
        "description": "Runtime-Komponente"
      },
      {
        "ext": ".atsyscall",
        "name": "System Call Table Map",
        "description": "System-Call-Definition"
      },
      {
        "ext": ".atabi",
        "name": "App Binary Interface",
        "description": "Application Binary Interface"
      },
      {
        "ext": ".atlinker",
        "name": "Dynamic Linker Config",
        "description": "Linker-Konfiguration"
      },
      {
        "ext": ".atloader",
        "name": "Binary Executable Loader",
        "description": "Executable Loader"
      }
    ]
  },
  {
    "name": "ATOS v4.0 - Compilers & Toolchains",
    "extensions": [
      {
        "ext": ".atc",
        "name": "A-Town C Source",
        "description": "A-Town C Source"
      },
      {
        "ext": ".atcpp",
        "name": "A-Town C++ Source",
        "description": "A-Town C++ Source"
      },
      {
        "ext": ".atrs",
        "name": "A-Town Rust Source",
        "description": "A-Town Rust Source"
      },
      {
        "ext": ".atgo",
        "name": "A-Town Go Source",
        "description": "A-Town Go Source"
      },
      {
        "ext": ".atpy",
        "name": "A-Town Python Source",
        "description": "A-Town Python Source"
      },
      {
        "ext": ".atts",
        "name": "A-Town TypeScript",
        "description": "A-Town TypeScript Source"
      },
      {
        "ext": ".atjava",
        "name": "A-Town Java Source",
        "description": "A-Town Java Source"
      },
      {
        "ext": ".atbytecode",
        "name": "A-Town IL Bytecode",
        "description": "Bytecode"
      },
      {
        "ext": ".atir",
        "name": "Intermediate Rep",
        "description": "Intermediate Representation"
      },
      {
        "ext": ".atasm",
        "name": "A-Town Assembly",
        "description": "Assembler-Code"
      }
    ]
  },
  {
    "name": "ATOS v5.0 - Enterprise ERP Systems",
    "extensions": [
      {
        "ext": ".aterp",
        "name": "Enterprise Resource Plan",
        "description": "Enterprise Resource Planning"
      },
      {
        "ext": ".atcrm",
        "name": "Client Tracking Graph",
        "description": "Customer Relationship Management"
      },
      {
        "ext": ".athrm",
        "name": "Personnel Human Resources",
        "description": "Human Resource Management"
      },
      {
        "ext": ".atpayroll",
        "name": "Payment Dispense Ledger",
        "description": "Gehaltsabrechnung"
      },
      {
        "ext": ".atinvoicecore",
        "name": "Invoice Accounts Ledger",
        "description": "Rechnungsmanagement"
      },
      {
        "ext": ".atprocurement",
        "name": "Resource Ordering Log",
        "description": "Beschaffung"
      },
      {
        "ext": ".atsupply",
        "name": "Supply Chain Verifier Node",
        "description": "Lieferkette"
      },
      {
        "ext": ".atwarehousemgmt",
        "name": "Physical Supply Vault",
        "description": "Lagerverwaltung"
      },
      {
        "ext": ".atassetmgmt",
        "name": "Physical Machine Ledger",
        "description": "Asset Management"
      },
      {
        "ext": ".atbusinessprocess",
        "name": "B2B Operation Flow Map",
        "description": "Geschäftsprozesse"
      }
    ]
  },
  {
    "name": "ATOS v6.0 - Science & Labs",
    "extensions": [
      {
        "ext": ".atresearch",
        "name": "Lab Experiment Scope",
        "description": "Forschungsprojekt"
      },
      {
        "ext": ".atpaper",
        "name": "Web3 Publish Doc",
        "description": "Wissenschaftliche Publikation"
      },
      {
        "ext": ".atexperiment",
        "name": "Beaker / Laser control",
        "description": "Experiment"
      },
      {
        "ext": ".athypothesis",
        "name": "Conjecture testing state",
        "description": "Hypothese"
      },
      {
        "ext": ".atdatasetraw",
        "name": "Gigabyte Sensor Raw",
        "description": "Rohdaten"
      },
      {
        "ext": ".atpeerreview",
        "name": "Validator Science Stamp",
        "description": "Peer Review"
      },
      {
        "ext": ".atcitation",
        "name": "Immutable Paper Link",
        "description": "Literaturverweis"
      },
      {
        "ext": ".atsimulationrun",
        "name": "Monte Carlo Climate Run",
        "description": "Simulationslauf"
      },
      {
        "ext": ".atlab",
        "name": "Digital Clean-Room VM",
        "description": "Laborumgebung"
      },
      {
        "ext": ".atdiscovery",
        "name": "AI patent filing log",
        "description": "Forschungsergebnis"
      }
    ]
  },
  {
    "name": "ATOS v6.0 - Complete Digital Twins",
    "extensions": [
      {
        "ext": ".atdigitalhuman",
        "name": "Biometric AI Clone",
        "description": "Digitaler Mensch"
      },
      {
        "ext": ".atdigitalcity",
        "name": "Smart-City Traffic Sim",
        "description": "Digitale Stadt"
      },
      {
        "ext": ".atdigitalnation",
        "name": "Macro Country Ledger",
        "description": "Digitale Nation"
      },
      {
        "ext": ".atdigitalplanet",
        "name": "Climate Biosphere Sync",
        "description": "Digitaler Planet"
      },
      {
        "ext": ".atdigitaleconomy",
        "name": "Bank Ledger Replica",
        "description": "Digitale Wirtschaft"
      },
      {
        "ext": ".atdigitalorg",
        "name": "Corporate Twin Entity",
        "description": "Digitale Organisation"
      },
      {
        "ext": ".atdigitalnetwork",
        "name": "Internet Topology Map",
        "description": "Digitales Netzwerk"
      },
      {
        "ext": ".atdigitalsystem",
        "name": "Factory / Plant Twin",
        "description": "Digitales System"
      },
      {
        "ext": ".atdigitalmarket",
        "name": "Stock Exchange SIM",
        "description": "Digitaler Markt"
      },
      {
        "ext": ".atdigitaluniverse",
        "name": "Astrophysics Simulation",
        "description": "Digitales Universum"
      }
    ]
  },
  {
    "name": "ATOS v6.0 - Interplanetary Link Base",
    "extensions": [
      {
        "ext": ".atlunar",
        "name": "Moon Base Relay IP",
        "description": "Mondnetzwerk"
      },
      {
        "ext": ".atmars",
        "name": "Martian Colony Link",
        "description": "Marsnetzwerk"
      },
      {
        "ext": ".atorbitalnode",
        "name": "ISS / Station Switch",
        "description": "Orbitaler Knoten"
      },
      {
        "ext": ".atdeeprelay",
        "name": "Voyager / Deep space",
        "description": "Deep-Space-Relay"
      },
      {
        "ext": ".atplanetmesh",
        "name": "Global planet surface",
        "description": "Planeten-Mesh"
      },
      {
        "ext": ".atspaceeconomy",
        "name": "Asteroid Mining Tx",
        "description": "Weltraumwirtschaft"
      },
      {
        "ext": ".atinterplanetary",
        "name": "Solar system Sync",
        "description": "Interplanetare Daten"
      },
      {
        "ext": ".atstarlink",
        "name": "LEO Constellation map",
        "description": "Satellitenverbund"
      },
      {
        "ext": ".atgalacticroute",
        "name": "Interstellar travel alg",
        "description": "Galaktische Route"
      },
      {
        "ext": ".atcosmos",
        "name": "Total universal constant",
        "description": "Kosmische Daten"
      }
    ]
  },
  {
    "name": "ATBRS - OS Web Browser Base",
    "extensions": [
      {
        "ext": ".atbrowser",
        "name": "Core Web Settings",
        "description": "Browser-Konfiguration"
      },
      {
        "ext": ".atweb",
        "name": "A-Town Web 3.0 Document",
        "description": "Webdokument"
      },
      {
        "ext": ".atpage",
        "name": "DOM Page Structure",
        "description": "Webseitenobjekt"
      },
      {
        "ext": ".attab",
        "name": "Saved Tab State",
        "description": "Browser-Tab"
      },
      {
        "ext": ".atsessionweb",
        "name": "OAuth / Login States",
        "description": "Browsersitzung"
      },
      {
        "ext": ".atbookmark",
        "name": "Saved URL Pointer",
        "description": "Lesezeichen"
      },
      {
        "ext": ".athistory",
        "name": "Browsing Archive",
        "description": "Browserverlauf"
      },
      {
        "ext": ".atdownload",
        "name": "Downloading Blob",
        "description": "Downloadobjekt"
      },
      {
        "ext": ".atupload",
        "name": "Uploading Blob",
        "description": "Uploadobjekt"
      },
      {
        "ext": ".atcacheweb",
        "name": "Local Fast-Load Image",
        "description": "Browser-Cache"
      }
    ]
  },
  {
    "name": "ATBRS - Engine Elements",
    "extensions": [
      {
        "ext": ".atrender",
        "name": "CPU/GPU Paint Settings",
        "description": "Rendering-Konfiguration"
      },
      {
        "ext": ".atdom",
        "name": "Page Hierarchy Tree",
        "description": "Dokumentenobjektmodell"
      },
      {
        "ext": ".atcss",
        "name": "Cascading Decor Style",
        "description": "Stylesheet"
      },
      {
        "ext": ".atlayoutengine",
        "name": "Flex/Grid Calculator",
        "description": "Layoutberechnung"
      },
      {
        "ext": ".atfontengine",
        "name": "TrueType/WOFF Renderer",
        "description": "Schriftengine"
      },
      {
        "ext": ".atmediaengine",
        "name": "Video / Audio Decoder",
        "description": "Multimedia-Engine"
      },
      {
        "ext": ".atcanvas",
        "name": "2D Draw Space",
        "description": "Grafik-Canvas"
      },
      {
        "ext": ".atwebgl",
        "name": "3D GPU Space",
        "description": "3D-Rendering"
      },
      {
        "ext": ".atviewport",
        "name": "Screen Size Logic",
        "description": "Viewport"
      },
      {
        "ext": ".atframe",
        "name": "IFrame / Sub-window",
        "description": "Frame"
      }
    ]
  },
  {
    "name": "ATBRS - Blockchain Web Native",
    "extensions": [
      {
        "ext": ".atdapp",
        "name": "Web3 Contract UI",
        "description": "Dezentrale Anwendung"
      },
      {
        "ext": ".atwalletconnect",
        "name": "Key-signer tunnel",
        "description": "Wallet-Verbindung"
      },
      {
        "ext": ".atwebtx",
        "name": "Pending Transaction",
        "description": "Web-Transaktion"
      },
      {
        "ext": ".atwebsign",
        "name": "Signature request modal",
        "description": "Signaturanforderung"
      },
      {
        "ext": ".atwebproof",
        "name": "Local ZK-SNARK Builder",
        "description": "ZK-Proof im Browser"
      },
      {
        "ext": ".atweboracle",
        "name": "Off-chain Data Fetch",
        "description": "Oracle-Anfrage"
      },
      {
        "ext": ".atwebdao",
        "name": "Voting Dashboard",
        "description": "DAO-Oberfläche"
      },
      {
        "ext": ".atwebidentity",
        "name": "Decentralized DID login",
        "description": "Dezentrale Identität"
      },
      {
        "ext": ".atwebasset",
        "name": "3D NFT viewer data",
        "description": "Asset-Objekt"
      },
      {
        "ext": ".atwebnft",
        "name": "Verified Image Check",
        "description": "NFT-Objekt"
      }
    ]
  },
  {
    "name": "ATBRS - AI Co-Pilot Browser (ATS)",
    "extensions": [
      {
        "ext": ".atcopilot",
        "name": "Side-panel Chat AI",
        "description": "Browser-Assistent"
      },
      {
        "ext": ".atagentweb",
        "name": "Auto-form filler AI",
        "description": "Browser-Agent"
      },
      {
        "ext": ".atsearchai",
        "name": "Vector query processor",
        "description": "KI-Suche"
      },
      {
        "ext": ".atsummary",
        "name": "TL;DR Page condenser",
        "description": "Webseiten-Zusammenfassung"
      },
      {
        "ext": ".atreasonweb",
        "name": "Fact check / Bias detect",
        "description": "Webseitenanalyse"
      },
      {
        "ext": ".atmemoryweb",
        "name": "User habit learning DB",
        "description": "Browser-Gedächtnis"
      },
      {
        "ext": ".atcontextweb",
        "name": "Currently read topic",
        "description": "Kontextspeicher"
      },
      {
        "ext": ".atresearchweb",
        "name": "Deep-research Agent task",
        "description": "Recherche-Projekt"
      },
      {
        "ext": ".atworkflowweb",
        "name": "RPA Web Automation",
        "description": "Browser-Workflow"
      },
      {
        "ext": ".atautomationweb",
        "name": "Macro / Script player",
        "description": "Browser-Automation"
      }
    ]
  },
  {
    "name": "ATBRS - Native Custom Formats",
    "extensions": [
      {
        "ext": ".atml",
        "name": "Markup XML tree",
        "description": "A-Town Markup Language"
      },
      {
        "ext": ".atstyle",
        "name": "CSS equivalent",
        "description": "A-Town Style Language"
      },
      {
        "ext": ".atscript",
        "name": "JavaScript equivalent",
        "description": "A-Town Script Language"
      },
      {
        "ext": ".atcomponent",
        "name": "Reusable React style",
        "description": "UI-Komponente"
      },
      {
        "ext": ".atlayout",
        "name": "Base page grid",
        "description": "Seitenlayout"
      },
      {
        "ext": ".attemplate",
        "name": "Handlebars style text",
        "description": "Vorlage"
      },
      {
        "ext": ".atrouter",
        "name": "Path-URL manager",
        "description": "Routing"
      },
      {
        "ext": ".atstate",
        "name": "Redux-like memory",
        "description": "Frontend-State"
      },
      {
        "ext": ".atui",
        "name": "Button/Slider defs",
        "description": "Benutzeroberfläche"
      },
      {
        "ext": ".atpageapp",
        "name": "Entire bundle page",
        "description": "Komplette Seite"
      }
    ]
  }
];
