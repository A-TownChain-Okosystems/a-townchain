# ARCHITECTURE.md — atc-devtools
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-devtools/
├── package.json               # Node.js package configuration & dependencies
├── tsconfig.json             # TypeScript configuration
├── src/
│   ├── index.ts              # Main devtools interface export
│   ├── analyzer/             # Code analysis engine for ATCLang and smart contracts
│   ├── debugger/             # Interactive execution debugger and breakpoint manager
│   ├── editor/               # IDE integration & web editor component
│   └── linter/               # AST linter and static analysis rules
├── analyzer/
│   └── code_analyzer.atc     # ATCLang static analyzer module contract
├── debugger/
│   └── debugger.atc          # ATCLang runtime debugger contract
├── editor/
│   └── code_editor.atc       # Code editor backend integration contract
├── playground/
│   └── playground.atc        # Interactive execution sandbox contract
├── profiler/
│   └── profiler.atc          # Performance profiling and gas estimation tool
└── viewer/
    └── source_viewer.atc     # Source code & AST visualization contract
```

## Module Descriptions
- package.json — Package manifest, scripts, and dependency declarations
- tsconfig.json — TypeScript compiler configuration
- src/index.ts — Central entry point exposing devtools APIs
- src/analyzer/ — Code analysis module providing syntax checking and AST verification
- src/debugger/ — Runtime step-execution debugger for ATCLang contracts
- src/editor/ — Web-based code editor and language server interface
- src/linter/ — Code style and security vulnerability linter
- analyzer/code_analyzer.atc — ATCLang code analyzer contract
- debugger/debugger.atc — Debug state monitoring contract
- editor/code_editor.atc — Language server integration contract
- playground/playground.atc — Execution sandbox for contract testing
- profiler/profiler.atc — Performance and execution profiling contract
- viewer/source_viewer.atc — Source tree and AST visualizer contract

## Build System
- TypeScript compiler (tsc) / npm build scripts

## Dependencies
- Node.js, TypeScript

## Status (Active/Migrated/Legacy)
Active (TypeScript)
