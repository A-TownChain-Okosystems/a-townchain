// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export const ATC_LANG_WIKI_CONTENT = [
  {
    id: "overview",
    category: "ATC-Lang",
    subcategory: "Einführung",
    tags: ["smart-contracts", "dsl", "verification"],
    title: "1. ATC-Lang: Die formale Smart Contract Sprache",
    text: "ATC-Lang ist die offizielle domänenspezifische Sprache (DSL) für Smart Contracts auf der A-TownChain. Entworfen mit starkem Fokus auf Sicherheit, kombiniert sie Turing-Vollständigkeit mit strikten Ressourcengrenzen (Gas) und formaler Verifizierbarkeit.\n\nDurch die statische Typisierung und die eingebettete Hoare-Logik (Pre- und Post-Conditions) lassen sich Invarianten mathematisch beweisen, noch bevor der Code zu deterministischem ATVM-Bytecode kompiliert wird.",
    revisions: [
      { timestamp: "2026-06-09T14:00:00Z", editor: "Language Designer", message: "Initial ATC-Lang specification" }
    ],
    table: [
      { component: "Lineare Typen", desc: "Verhindern Reentrancy-Angriffe", status: "Active" },
      { component: "Hoare-Logik Assertions", desc: "Formale Verifikation der Invarianten", status: "Active" },
      { component: "ATVM Bytecode", desc: "Deterministisches Kompilierungsziel", status: "Active" }
    ]
  },
  {
    id: "semantics",
    category: "ATC-Lang",
    subcategory: "Semantik",
    title: "2. Formale Semantik und Typsystem",
    text: "Die formale Semantik von ATC-Lang wird im pi-Kalkül beschrieben, was die Analyse von Nebenläufigkeit und Kommunikation zwischen Smart Contracts erlaubt.\n\n**Typ-Sicherheit:**\nJede Zuweisung und Typ-Konsistenz wird zur Compile-Zeit geprüft. Das Typsystem verwendet *Ownership* und *Borrowing* ähnlich wie Rust, um Speicherzugriffsfehler und Race-Conditions auszuschließen.",
    revisions: [
      { timestamp: "2026-06-09T14:15:00Z", editor: "Compiler Team", message: "Pi-calculus extensions added" }
    ],
    table: [
      { component: "Borrow Checker", desc: "Sicheres Memory Management", status: "Active" },
      { component: "Pi-Kalkül Modeler", desc: "Inter-contract communication analysis", status: "Active" }
    ]
  },
  {
    id: "compiler",
    category: "ATC-Lang",
    subcategory: "Compiler",
    title: "3. Der ATC-Lang Compiler (ALC)",
    text: "Der ATC-Lang Compiler ('ALC') ist in Rust geschrieben und übersetzt den Source-Code in deterministischen Custom Bytecode (ATVM). Er besteht aus Lexer, Parser, AST-Generator, Type Checker, und dem Emitter.\n\nZudem injiziert der ALC Gas-Metering-Instruktionen (`consume_gas(amount)`), um sicherzustellen, dass Endlosschleifen OOM-Fehler beim Ausführen der VM hervorrufen und abgebrochen werden.",
    revisions: [
      { timestamp: "2026-06-09T14:30:00Z", editor: "Core Compiler Eng", message: "AST to ATVM pipeline defined" }
    ],
    table: [
      { component: "Gas Injection Pass", desc: "Baut Metering in den AST ein", status: "Active" },
      { component: "ATVM Backend", desc: "Generiert optimierten Bytecode", status: "Active" }
    ]
  },
  {
    id: "standard_lib",
    category: "ATC-Lang",
    subcategory: "Bibliotheken",
    title: "4. Cryptographic Standard Library",
    text: "Eine stark optimierte, vom Node vorkompilierte Bibliothek, die kryptologische und mathematische Operationen kapselt. Dies verhindert, dass jeder Contract diese Operationen im Bytecode replizieren muss (reduziert Größe und Gas-Kosten).\n\n**Enthaltene Primitiven:**\n- `Keccak256`, `SHA3-512`\n- secp256k1 Curve Operations\n- BLS12-381 für Zero-Knowledge SNARK Verifizierungen",
    revisions: [
      { timestamp: "2026-06-09T14:45:00Z", editor: "StdLib maintainer", message: "BLS12-381 routines integrated" }
    ],
    table: [
      { component: "std::crypto", desc: "Native Crypto Precompiles", status: "Active" },
      { component: "std::math::FixedPoint", desc: "Präzises Rechnen mit Token", status: "Active" }
    ]
  },
  {
    id: "zero_knowledge",
    category: "ATC-Lang",
    subcategory: "Features",
    title: "5. ZK-SNARK Integration Syntax",
    text: "ATC-Lang besitzt First-Class Support für Zero-Knowledge SNARKs. Entwickler können `proof { ... }` Blöcke verwenden, in denen Operationen direkt in PLONK-Constraints (R1CS) umgewandelt werden statt in regulären Bytecode.\n\nDadurch lassen sich Private State-Variablen verwalten, ohne dass der Compiler gewechselt werden muss.",
    revisions: [
      { timestamp: "2026-06-09T15:00:00Z", editor: "ZK Specialist", message: "Unified ZK-block syntax added" }
    ],
    table: [
      { component: "R1CS Compiler Pass", desc: "Zusätzliches Target neben ATVM", status: "Active" },
      { component: "Private Variables", desc: "@private Decorator für State", status: "Abgeschlossen" }
    ]
  },
  {
    id: "actor_model",
    category: "ATC-Lang",
    subcategory: "Semantik",
    title: "6. Actor-Model & Asynchronität",
    text: "Contracts in ATC-Lang sind eigenständige **Actors**. Sie verwalten ihren eigenen Zustand isoliert und kommunizieren nur über Message Passing (`send` und `await`).\n\nDas verhindert komplexe Reentrancy-Gefahren, da ein Contract während des Wartens auf eine Antwort keine Aufrufe mehr in den gleichen State zulässt, außer explizit durch `[allow_reentrant]` markiert.",
    revisions: [
      { timestamp: "2026-06-09T15:15:00Z", editor: "Language Designer", message: "Actor Model Concurrency specifics" }
    ],
    table: [
      { component: "Message Passing", desc: "Lock-Free Kommunikation", status: "Active" },
      { component: "Mailbox Queue", desc: "Definierte Abarbeitung von TXs", status: "Active" }
    ]
  },
  {
    id: "formal_verification",
    category: "ATC-Lang",
    subcategory: "Verifikation",
    title: "7. Automatisierter Theorem-Beweiser",
    text: "Das `atc-verify` Tool ist Teil der Compiler-Suite und übersetzt den ATC-Lang Code in Coq/Z3 Modelle, um sicherzustellen, dass keine Overflows, Underflows, oder Invalid State Transitions möglich sind.\n\nEine Deployment an das Mainnet wird erst erlaubt, wenn der Theorem-Beweiser eine `[Prove]-Success` Flag zurückgibt. So werden Multimillionen-Dollar-Exploits proaktiv verhindert.",
    revisions: [
      { timestamp: "2026-06-09T15:30:00Z", editor: "Security Eng", message: "Z3 SMT Solver integration" }
    ],
    table: [
      { component: "atc-verify CLI", desc: "Lokaler SMT Solver für Contracts", status: "Active" }
    ]
  },
  {
    id: "gas_calculus",
    category: "ATC-Lang",
    subcategory: "Ressourcen",
    title: "8. Gas & O(1) Ressourcen-Kalkül",
    text: "Der Compiler berechnet für jeden Kontrollfluss-Ast ein Worst-Case-Gas-Limit. Wenn der Obergrenzen-Wert zur Compile-Zeit unbestimmbar ist (z.B. unendliche Schleifen ohne Abbruchbedingung), wird die Kompilierung verweigert.\n\nDamit garantiert ATC-Lang Liveness und verhindert Spam-Angriffe auf das Netzwerk effektiv.",
    revisions: [
      { timestamp: "2026-06-09T15:45:00Z", editor: "Ops Team", message: "Worst-Case Execution Time (WCET) constraints" }
    ],
    table: [
      { component: "WCET Analyzer", desc: "Branch Prediction für Gas", status: "Active" }
    ]
  },
  {
    id: "ide_support",
    category: "ATC-Lang",
    subcategory: "Tools",
    title: "9. IDE & Language Server (LSP)",
    text: "Um die Developer-Experience (DX) zu fördern, bietet ATC-Lang einen offiziellen Language Server Protocol (LSP) Daemon. Er bietet Autocomplete, Type-Inference Tooltips und Echtzeit-Syntax-Checks für VS Code, JetBrains und Neovim.\n\nAuch der `SmartContractIDE` im A-Town OS bindet diesen per Web-Worker direkt ein.",
    revisions: [
      { timestamp: "2026-06-09T16:00:00Z", editor: "Tools Dev", message: "LSP Web-Worker support for in-browser IDE" }
    ],
    table: [
      { component: "atc-lsp", desc: "Der Language Server Daemon", status: "Active" },
      { component: "Syntax Highlighting", desc: "TextMate Grammar (.tmLanguage)", status: "Active" }
    ]
  },
  {
    id: "atc_lang_macros",
    category: "ATC-Lang",
    subcategory: "Features",
    title: "10. Makros & Metaprogrammierung",
    text: "Mit ATC-Lang können Entwickler hygienische Makros definieren, ähnlich wie in Rust. Diese werden während der AST-Generierung aufgelöst und erlauben Code-Wiederverwendung für Boilerplate (wie Standard-Token-Implementationen).\n\nDie `atc_macro!` Syntax ist Turing-vollständig, wird aber strikt vor der Verifikation entpackt, um Beweisbarkeit zu garantieren.",
    revisions: [
      { timestamp: "2026-06-09T16:15:00Z", editor: "Compiler Team", message: "Hygienic macros implementation" }
    ],
    table: [
      { component: "AST Expander", desc: "Resolver für Makros", status: "Active" }
    ]
  },
  {
    id: "package_manager",
    category: "ATC-Lang",
    subcategory: "Tools",
    title: "11. ATC-Pack (Package Manager)",
    text: "Damit Smart Contracts modular entwickelt werden können, bietet ATC-Lang einen offiziellen Package Manager namens ATC-Pack. Er nutzt IPFS als dezentralisiertes Repository für Libraries und Crates.\n\nAbhängigkeiten werden kryptographisch über Hash-Pinning gesichert, um Supply-Chain-Attacken auszuschließen.",
    revisions: [
      { timestamp: "2026-06-09T16:30:00Z", editor: "DevOps", message: "IPFS base implementation for Package Manager" }
    ],
    table: [
      { component: "atc-pack CLI", desc: "Dependency Resolver für Smart Contracts", status: "Active" },
      { component: "IPFS Gateway", desc: "Dezentrales Code-Hosting", status: "Active" }
    ]
  },
  {
    id: "unsafe_blocks",
    category: "ATC-Lang",
    subcategory: "Features",
    title: "12. Inline Assembly & Unsafe Blocks",
    text: "Für hochgradig optimierte Smart Contracts oder spezielle kryptographische Routinen, die nicht vom Standard-Compiler generiert werden können, erlaubt ATC-Lang `unsafe {}` Blöcke. Innerhalb dieser können Entwickler direkt auf ATVM OpCodes oder Hardware-Register zugreifen.\n\nEin Smart Contract mit `unsafe` Block muss jedoch durch einen manuellen Audit-Prozess der DAO, da der formale Theorem-Beweiser diesen Teil nicht garantieren kann.",
    revisions: [
      { timestamp: "2026-06-09T16:45:00Z", editor: "Kernel Dev", message: "Added unsafe bindings to standard specifications" }
    ],
    table: [
      { component: "Inline ATVM ASM", desc: "Hand-optimierter Code", status: "Active" },
      { component: "Audit Flags", desc: "Manuelle Review-Trigger in CI", status: "Abgeschlossen" }
    ]
  },
  {
    id: "cross_chain",
    category: "ATC-Lang",
    subcategory: "Interoperabilität",
    title: "13. Cross-Chain Interoperability",
    text: "ATC-Lang bietet native Sprachkonstrukte für das Senden und Empfangen von Inter-Blockchain Communication (IBC) Nachrichten. Entwickler können Typen mit dem `[cross_chain]` Pragma markieren, woraufhin der Compiler automatisch ABI-kompatible Brücken zu EVM, Polkadot oder Cosmos generiert.",
    revisions: [
      { timestamp: "2026-06-09T17:00:00Z", editor: "Bridges Team", message: "Cross-chain RPC bindings in AST" }
    ],
    table: [
      { component: "IBC Relayer Types", desc: "Native Typen für Cross-Chain State", status: "Abgeschlossen" },
      { component: "EVM ABI Generator", desc: "Export zu Solidity Interfaces", status: "Abgeschlossen" }
    ]
  },
  {
    id: "fuzzer",
    category: "ATC-Lang",
    subcategory: "Verifikation",
    title: "14. Formaler Property Fuzzer (ATC-Fuzz)",
    text: "Zusätzlich zum Theorem-Beweiser enthält die ATC-Lang Toolchain einen stochastischen Property-Fuzzer (ATC-Fuzz). Er generiert automatisch Tausende von Millionen Edge-Cases basierend auf den Hoare-Logik-Assertions und testet den State-Machinecode gegen Underflows und Reentrancy.",
    revisions: [
      { timestamp: "2026-06-09T17:15:00Z", editor: "Security Team", message: "Fuzzing engine integrated into CI pipeline" }
    ],
    table: [
      { component: "Coverage-Guided Fuzzer", desc: "Optimierte Execution-Path Discovery", status: "Active" }
    ]
  },
  {
    id: "debugger",
    category: "ATC-Lang",
    subcategory: "Tools",
    title: "15. Time-Travel Debugger (ATC-Trace)",
    text: "ATC-Trace ist ein Time-Travel Debugger für Smart Contracts. Durch die Determinismus-Eigenschaft der A-Town Virtual Machine lassen sich Vertragsausführungen rückwärts abspielen (`Reverse Step`).\n\nEntwickler können so in der SmartContractIDE durch vergangene Transaktionen steppen und den genauen Memory- und Stack-State bei einem Fehler analysieren.",
    revisions: [
      { timestamp: "2026-06-09T17:30:00Z", editor: "Tooling Team", message: "State caching for reverse debugging" }
    ],
    table: [
      { component: "State Snapshotting", desc: "Deltas für jeden OpCode generieren", status: "Active" },
      { component: "VS Code Extension", desc: "Visuelles Step-through Interface", status: "Active" }
    ]
  },
  {
    id: "feature_gap",
    category: "ATC-Lang",
    subcategory: "Features",
    title: "16. Feature-Lücken: Was ATC-Lang im Vergleich zu anderen Sprachen bewusst nicht hat",
    text: "Beim Design einer Sprache für formale Verifikation und sichere Smart Contracts (wie ATC-Lang) geht es weniger darum, möglichst viele Features zu bieten, sondern eher um eine bewusste Einschränkung. Im Folgenden ist eine detaillierte Liste von Features aufgeführt, die in herkömmlichen oder anderen Blockchain-Sprachen häufig zu finden sind, in ATC-Lang aber absichtlich fehlen:\n\n**1. Dynamische Typisierung (wie in Python/JavaScript)**\nJeder Typ muss zur Compile-Zeit statisch bekannt sein und geprüft werden können, um Laufzeitfehler zu vermeiden.\n\n**2. Laufzeit-Garbage-Collection (wie in Java/Go)**\nEin Garbage Collector sorgt für unvorhersehbare Pausen und Gas-Kosten-Explosionen. ATC-Lang nutzt ein Ownership- und Borrowing-System (ähnlich Rust).\n\n**3. Unbeschnittene Rekursion oder freie `while(true)`-Schleifen**\nUm die Worst-Case Execution Time (WCET) sicherstellen zu können, muss der Compiler absehen können, wann eine Schleife terminiert oder ein festes Limit durch Gas-Kalkulation vorliegt.\n\n**4. Traditionelle Exceptions (`try/catch`)**\nAnstelle von unvorhersehbarem Kontrollfluss durch Exceptions, verwendet ATC-Lang strikte Typen wie `Result<T, E>` oder `Option<T>` für die explizite Fehlerbehandlung.\n\n**5. Float-Arithmetik (IEEE 754 Gleitkommazahlen)**\nFließkommazahlen sind auf verschiedenen Prozessorarchitekturen nicht 100% deterministisch. ATC-Lang verwendet ausschließlich `FixedPoint`- und BigInteger-Mathematik (`std::math::FixedPoint`).\n\n**6. Klassenbasierte Objektorientierung & Vererbung**\nTiefe Vererbungshierarchien (wie in Solidity oder Java) führen oft zum 'Diamond Problem' und undurchsichtigem State. ATC-Lang setzt stattdessen auf Traits/Interfaces und Komposition im Actor-Modell.\n\n**7. System-Calls (File I/O, Sockets)**\nDa Contracts in der isolierten ATVM (A-Town Virtual Machine) leben, existiert kein Zugriff auf das Host-OS, das Dateisystem oder externe APIs (kein `std::fs` oder `std::net`). Die Kommunikation erfolgt strikt über Oracles und Message Passing.\n\n**8. Reflection und dynamisches Dispatch (RTTI)**\nDas Inspizieren und Aufrufen von dynamischen Typen zur Laufzeit verhindert eine saubere statische Code-Verifikation und erhöht die Angriffsfläche.\n\n**9. Globale mutierbare Variablen**\nDer State existiert nur im isolierten Kontext eines Actors (Contracts). Es gibt keine programmenweiten globalen Variablen, die über alle Contracts hinweg Zustand undefiniert teilen.\n\n**10. String Evaluation (`eval()`)**\nDie Ausführung von Strings oder dynamischem Bytecode zur Laufzeit ist komplett untersagt, um Injektions-Attacken abzuwehren.\n\n**11. Unstrukturierte GOTO-Befehle**\nUnstrukturierte Sprünge machen jeden Versuch der formalen Verifikation oder des Control-Flow-Graphs zunichte.\n\nAll diese 'Einschränkungen' sind in Wirklichkeit Sicherheitsgarantien. Sie reduzieren die Komplexität und machen mathematische Beweisführung via `atc-verify` erst möglich.",
    revisions: [
      { timestamp: "2026-06-09T18:00:00Z", editor: "Language Architect", message: "Added comprehensive comparison of missing features." }
    ],
    table: [
      { component: "Dynamische Typen", desc: "Verboten (Sichert statische Analyse)", status: "Active" },
      { component: "Garbage Collector", desc: "Verboten (Ownership statt GC)", status: "Active" },
      { component: "Fließkommazahlen", desc: "Verboten (FixedPoint für Determinismus)", status: "Active" },
      { component: "Exceptions", desc: "Verboten (Result-Types für Errors)", status: "Active" }
    ]
  }
];
