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

export const ATC_LANG_ROADMAP_DATA: RoadmapPhase[] = [
  {
    id: "al_phase0",
    title: "Syntax & Grammatik (Lexer/Parser)",
    timeframe: "Phase 0",
    status: "Abgeschlossen",
    sections: [
      {
        title: "Reguläre Sprachen & Grammatik",
        items: [
          "Tokenization / Lexical Analysis (PEg/Regex)",
          "Abstract Syntax Tree (AST) Definition"
        ]
      }
    ],
    goals: [{ text: "Meilenstein L0: Stabiler Parser für Basis-Syntax", completed: true }]
  },
  {
    id: "al_phase1",
    title: "Type Checker & Borrow Semantics",
    timeframe: "Phase 1",
    status: "Abgeschlossen",
    sections: [
      {
        title: "Statisches Typsystem",
        items: [
          "Linear Types für Unforgeable Assets (Tokens)",
          "Ownership & Borrow Checker Constraints"
        ]
      }
    ],
    goals: [{ text: "Meilenstein L1: Soundness des Typsystems bewiesen", completed: true }]
  },
  {
    id: "al_phase2",
    title: "Backend: ATVM Code Generation",
    timeframe: "Phase 2",
    status: "Abgeschlossen",
    sections: [
      {
        title: "Emitter & LLVM IR",
        items: [
          "Übersetzung von AST in deterministischen ATVM-Bytecode",
          "O(1) Garbage Collection Prevention"
        ]
      }
    ],
    goals: [{ text: "Meilenstein L2: Ejecutable Smart Contracts in der VM", completed: true }]
  },
  {
    id: "al_phase3",
    title: "Formale Verifikation & SMT Solver",
    timeframe: "Phase 3",
    status: "Abgeschlossen",
    sections: [
      {
        title: "Z3/Coq Integration",
        items: [
          "Pre/Post-Conditions als Theorem in Coq extrahieren",
          "Automatische Modellprüfung für Reentrancy"
        ]
      }
    ],
    goals: [{ text: "Meilenstein L3: Exploit-Beweis durch atc-verify", completed: true }]
  },
  {
    id: "al_phase4",
    title: "Standardbibliothek & Crypto-Precompiles",
    timeframe: "Phase 4",
    status: "Abgeschlossen",
    sections: [
      { title: "Native Crypto", items: ["Groth16 Verifizierer in std", "BLS12-381 Elliptische Kurven Support"] }
    ],
    goals: [{ text: "Meilenstein L4: Komplexe dApps möglich (ZKP/DEX)", completed: true }]
  },
  {
    id: "al_phase5",
    title: "ZK-SNARK First-Class Syntax",
    timeframe: "Phase 5",
    status: "Abgeschlossen",
    sections: [
      { title: "Circuit Generation", items: ["proof{} Syntaktische Blöcke compilieren zu R1CS"] }
    ],
    goals: [{ text: "Meilenstein L5: Nativer Zero-Knowledge Code in ATC-Lang", completed: true }]
  },
  {
    id: "al_phase6",
    title: "Language Server & IDE Tools",
    timeframe: "Phase 6",
    status: "Abgeschlossen",
    sections: [
      { title: "Developer Experience", items: ["LSP Daemon", "Online Playground Engine"] }
    ],
    goals: [{ text: "Meilenstein L6: World-Class Developer Experience", completed: true }]
  },
  {
    id: "al_phase7",
    title: "Cross-Chain & FFI / C API",
    timeframe: "Phase 7",
    status: "Abgeschlossen",
    sections: [
      { title: "Interoperabilität", items: ["EVM zu ATC-Lang Transpiler", "Foreign Function Interface (FFI) zu externen Orakeln"] }
    ],
    goals: [{ text: "Meilenstein L7: Migration von Ethereum Smart Contracts", completed: true }]
  },
  {
    id: "al_phase8",
    title: "Selbst-Hosting des Compilers",
    timeframe: "Phase 8",
    status: "Abgeschlossen",
    sections: [
      { title: "Meta-Circular Compiler", items: ["ATC-Lang Compiler in ATC-Lang geschrieben"] }
    ],
    goals: [{ text: "Meilenstein L8: Der Compiler bootstapped sich selbst", completed: true }]
  },
  {
    id: "al_phase9",
    title: "Enterprise V1 Release",
    timeframe: "Phase 9",
    status: "Abgeschlossen",
    sections: [
      { title: "Sicherheits-Audit", items: ["ISO/IEC 25010 Evaluierung", "Abschluss von 100% formalen Beweisen"] }
    ],
    goals: [{ text: "Meilenstein L9: Production Ready Enterprise Adoption", completed: true }]
  },
  {
    id: "al_phase10",
    title: "Dezentraler Package Manager",
    timeframe: "Phase 10",
    status: "Abgeschlossen",
    sections: [
      { title: "ATC-Pack Registry", items: ["IPFS-basiertes Hosting für Libraries", "Hash-Pinning für Supply-Chain Sicherheit"] }
    ],
    goals: [{ text: "Meilenstein L10: Sicheres Dependency Management", completed: true }]
  },
  {
    id: "al_phase11",
    title: "Low-Level Optimierung (Unsafe)",
    timeframe: "Phase 11",
    status: "Abgeschlossen",
    sections: [
      { title: "Inline Assembly", items: ["unsafe {} Blöcke für direkte ATVM OpCodes", "Manueller Review-Prozess für Kernel-Code"] }
    ],
    goals: [{ text: "Meilenstein L11: Maximale Hardwareauslastung durch native Primitiven", completed: true }]
  },
  {
    id: "al_phase12",
    title: "Cross-Chain Bridges & Oracles",
    timeframe: "Phase 12",
    status: "Abgeschlossen",
    sections: [
      { title: "Inter-Blockchain Communication", items: ["EVM und Polkadot ABI Translation", "[cross_chain] Makros"] }
    ],
    goals: [{ text: "Meilenstein L12: Native Multi-Chain Kompatibilität im Ast", completed: true }]
  },
  {
    id: "al_phase13",
    title: "ATC-Fuzz & Security Suite",
    timeframe: "Phase 13",
    status: "Abgeschlossen",
    sections: [
      { title: "Stochastische Verifikation", items: ["Coverage-guided Fuzzer", "Automatisierte Edge-Case Generierung via Hoare Logik"] }
    ],
    goals: [{ text: "Meilenstein L13: 100% Path-Coverage bei Contracts", completed: true }]
  },
  {
    id: "al_phase14",
    title: "Time-Travel Debugging",
    timeframe: "Phase 14",
    status: "Abgeschlossen",
    sections: [
      { title: "ATC-Trace IDE", items: ["Reverse Step in ATVM Ausführungen", "Memory Snapshot Deltas"] }
    ],
    goals: [{ text: "Meilenstein L14: Deterministisches Reverse Debugging", completed: true }]
  }
];
