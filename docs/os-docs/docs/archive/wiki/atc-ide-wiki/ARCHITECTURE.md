# ARCHITECTURE.md — atc-ide
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
├── debugger.atc
├── editor.atc
├── lsp_server.atc
├── package.json
├── project_manager.atc
├── repl.atc
├── src/
│   ├── debugger/
│   ├── editor/
│   ├── index.ts
│   └── repl/
├── syntax_highlight.atc
└── tsconfig.json
```

## Module Descriptions
- **src/editor/**: Web-based code editor core featuring ATC syntax highlighting, autocompletion, and error diagnostics.
- **src/debugger/**: Interactive debugging module supporting breakpoints, step-by-step execution, and variable state inspection.
- **src/repl/**: Interactive Read-Eval-Print Loop terminal interface for real-time ATC snippet evaluation.
- **package.json** & **tsconfig.json**: Project manifest and TypeScript configuration files.

## Build System
Node.js / npm environment with TypeScript (`tsc`) compiler and Vite frontend bundler.

## Dependencies
TypeScript 5.0+, Monaco Editor / CodeMirror, `vscode-languageclient`, `xterm.js` terminal emulator.
