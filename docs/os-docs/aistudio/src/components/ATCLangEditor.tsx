import React, { useState, useRef, useEffect } from "react";
import {
  Terminal,
  Play,
  FileCode,
  Code2,
  RefreshCw,
  Cpu,
  Activity,
  Folder,
  FolderOpen,
  ChevronRight,
  ChevronDown,
  Package,
  Layers,
  Library,
  Bot,
  Sparkles,
  Send,
  Upload,
  Globe
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

// Custom files structure
const INITIAL_FILES = {
  "main.atc": `// main.atc - ATCLang Main Entry Point
import "contracts/TokenData.atcnft" as NFTLayer;
import "standard/math.atclib" as Math;
import "neural/ai.atclib" as AIEngine;
use System from "@kernel/sys";

routine main() -> int {
    System.log("Booting ATCLang VM 2.0 Native Execution");
    
    // Test AI capabilities
    let sentiment = AIEngine.analyzeText("ATC-Lang is a revolutionary language without any external dependencies.");
    System.log("AI Sentiment Analysis: " + sentiment);
    
    // Instantiate Native NFT Layer Object
    let artifact = init NFTLayer.TokenData("0xATC_MASTER_WALLET");
    artifact.metadata.set("title", "Genesis Node Controller");
    
    let payload = artifact.id + artifact.owner;
    let signature = Math.hash_sha3(payload);
    
    System.log("Artifact forged & secured. Signature: " + signature);
    
    return 0; // Exit Success
}
`,
  "contracts/TokenData.atcnft": `// TokenData.atcnft - Native NFT Layer Specification
// Provides built-in layer architecture directly in the language

layer TokenData {
    field id: u256;
    field metadata: Map<String, String>;
    field owner: Address;

    construct(owner_addr: Address) {
        self.id = System.generate_uid();
        self.owner = owner_addr;
        self.metadata = init Map();
    }

    routine transfer(new_owner: Address) -> bool {
        self.owner = new_owner;
        emit Transfer(self.id, new_owner);
        return true;
    }
}
`,
  "standard/math.atclib": `// math.atclib - Standard ATCLang Library
// Contains highly optimized routines and direct ASM access

module Math {
    export routine hash_sha3(payload: String) -> String {
        // Native optimized assembly block bypassing V8/LLVM
        asm {
            LOAD_REG_A payload
            SHA3_OP
            STORE_REG_B
            RETURN_B
        }
        
        // Fallback for simulation
        return "0xATC_HASH_" + payload.length() * 42;
    }
}
`,
  "neural/ai.atclib": `// ai.atclib - Native AI integration layer
// Contains built-in neural inference commands specific to ATC-OS

module AIEngine {
    export routine analyzeText(input: String) -> f64 {
        // Native neural instruction
        asm {
            NEURAL_LOAD input
            NEURAL_INFER "sentiment_model_v1"
            RETURN_FLOAT
        }
        
        return 0.99; // Mock high positive sentiment
    }
    
    export routine optimizeSmartContract(bytecode: List<u8>) -> List<u8> {
        // Self-optimizing AI routing
        return bytecode.compress().obfuscate();
    }
}
`,
  "system/quantum.atcop": `// quantum.atcop - Post-quantum lattice ops

module QuantumOps {
    export routine generate_kyber_keypair() -> (String, String) {
        asm {
            Q_LATTICE_GEN 1024
            RETURN_TUPLE
        }
        return ("pk_q...", "sk_q...");
    }
}
`,
  "system/blockchain.atc": `use System from "@kernel/sys";
use "system/quantum.atcop" as Quantum;

layer ValidatorConfig {
    field isActive: bool;
    field stakedAmount: u256;
    
    routine slash(validator: Address, amount: u256) -> bool {
       // Core consensus action running natively
       System.slash_stake(validator, amount);
       emit SlashingEvent(validator, amount);
       return true;
    }
}
`
};

type FileType = "module" | "library" | "nft_layer" | "op_layer";

function getFileType(filename: string): FileType {
  if (filename.endsWith(".atclib")) return "library";
  if (filename.endsWith(".atcnft")) return "nft_layer";
  if (filename.endsWith(".atcop")) return "op_layer";
  return "module";
}

function getFileIcon(filename: string) {
  if (filename.endsWith(".atclib"))
    return <Library className="w-3.5 h-3.5 text-blue-400 shrink-0" />;
  if (filename.endsWith(".atcnft"))
    return <Layers className="w-3.5 h-3.5 text-fuchsia-400 shrink-0" />;
  if (filename.endsWith(".atcop"))
    return <Cpu className="w-3.5 h-3.5 text-amber-500 shrink-0" />;
  return <FileCode className="w-3.5 h-3.5 text-emerald-400 shrink-0" />;
}

export function ATCLangEditor() {
  const [files, setFiles] = useState<Record<string, string>>(INITIAL_FILES);
  const [activeFile, setActiveFile] = useState<string>("main.atc");
  const [output, setOutput] = useState<string[]>([
    "ATCLang Core Compiler v2.0.0 (Zero-Dependency Engine)",
    "System verified. Ready for compilation.",
  ]);
  const [isRunning, setIsRunning] = useState(false);
  const [isAiMode, setIsAiMode] = useState(false);
  const [aiPrompts, setAiPrompts] = useState<{role: 'user'|'ai', text: string}[]>([
    {role: 'ai', text: 'Hello! I am your native ATC-Lang AI Assistant. I can generate unique layer structures, libraries, and logic directly in ATC-Lang without external APIs. How can I help you program today?'}
  ]);
  const [currentAiInput, setCurrentAiInput] = useState('');
  const [isPublishing, setIsPublishing] = useState(false);

  // Folder state
  const [openFolders, setOpenFolders] = useState<Record<string, boolean>>({
    contracts: true,
    standard: true,
    neural: true,
    system: true,
  });

  const toggleFolder = (folder: string) => {
    setOpenFolders((prev) => ({ ...prev, [folder]: !prev[folder] }));
  };

  const createProject = (type: string) => {
    if (type === "dex") {
      setFiles(prev => ({
        ...prev,
        "contracts/DexExchange.atcnft": `// Decentralized Exchange via Native ATC
layer DexExchange {
    field liquidityPool: Map<Address, u256>;
    field exchangeRate: u64;

    routine swap(amountIn: §Asset) -> §Asset {
        // Zk-verified cross-chain swap
        let outValue = amountIn.value * self.exchangeRate;
        logger.debug("Swapping " + amountIn.value + " to " + outValue);
        return init Asset(outValue);
    }
}
`,
        "main.atc": `import "contracts/DexExchange.atcnft" as DEX;
use System from "@kernel/sys";

routine main() -> int {
    System.log("Deploying DEX core...");
    let exchange = init DEX.DexExchange();
    System.log("Exchange initialized with native zero-knowledge verifier.");
    return 0;
}
`
      }));
      setActiveFile("contracts/DexExchange.atcnft");
      setOpenFolders(prev => ({...prev, contracts: true}));
    }
  }

  const handleAiSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentAiInput.trim()) return;
    
    setAiPrompts(prev => [...prev, { role: 'user', text: currentAiInput }]);
    
    setTimeout(() => {
      let responseText = 'I have analyzed your request. I recommend using the `layer` directive to structure this, combined with `asm { NEURAL_INFER }` for the advanced logic. Let me generate a template for you in your current file.';
      let appended = "\n\n// AI Generated Snippet:\nroutine aiAutomated() -> bool {\n    System.log(\"AI-Assisted routine running\");\n    return true;\n}\n";

      if (currentAiInput.toLowerCase().includes("smart") || currentAiInput.toLowerCase().includes("contract") || currentAiInput.toLowerCase().includes("token")) {
        responseText = "I will construct a new Layer structure for your token using native ATC-Lang semantics. Inserting directly into your active editor.";
        appended = "\n\n// AI Generated Token Layer:\nlayer CustomToken {\n    field supply: u256;\n    field balances: Map<Address, u256>;\n\n    construct(initial_supply: u256) {\n        self.supply = initial_supply;\n        self.balances.set(msg.sender, initial_supply);\n    }\n\n    routine transfer(to: Address, amount: u256) -> bool {\n        if (self.balances.get(msg.sender) < amount) {\n            System.panic(\"Insufficient funds\");\n        }\n        self.balances.sub(msg.sender, amount);\n        self.balances.add(to, amount);\n        emit Transfer(msg.sender, to, amount);\n        return true;\n    }\n}\n";
      } else if (currentAiInput.toLowerCase().includes("ai") || currentAiInput.toLowerCase().includes("neural")) {
         responseText = "Native neural inference can be bound directly to standard layers. Let me show you an implementation blueprint.";
         appended = "\n\n// AI Generated Neural Binding:\nroutine processData(input: String) -> f64 {\n    let inference_result = 0.0;\n    asm {\n        NEURAL_LOAD input\n        NEURAL_INFER \"custom_model_v2\"\n        STORE_FLOAT inference_result\n    }\n    return inference_result;\n}\n";
      }
      
      setAiPrompts(prev => [...prev, { role: 'ai', text: responseText }]);
      
      // Inject code directly
      if (activeFile) {
        setFiles(prev => ({...prev, [activeFile]: (prev[activeFile] || "") + appended}));
      }
    }, 1200);
    
    setCurrentAiInput('');
  };

  const runCode = () => {
    setIsRunning(true);
    setOutput([]);

    setTimeout(() => {
      let logs = [
        "> ATCLang Compiler Pipeline Started...",
        "> Initializing Threat Modeling & Sandbox Environment...",
        "  [ZKP] Generating Zero-Knowledge constraints...",
        "  [MEM] Allocating quantum-safe registers...",
        "  ✓ Access Control verified (Role-Based ACL)",
        "  ✓ Reentrancy guards activated (Static Analysis)",
        "> Resolving ATC Native Dependencies...",
        "  - Linked standard/math.atclib (ASM-Optimized)",
        "  - Linked contracts/TokenData.atcnft (L2 Enabled)",
        "  - Linked neural/ai.atclib (NPU Accelerated)",
        "> Running AST Optimizations...",
        "  - Dead code elimination: 12 bytes freed",
        "  - Loop unrolling applied to hash functions",
        "> Compiling to ATC Native Bytecode (ATVM)...",
        "> Launching secure execution environment...",
      ];

      const mainCode = files["main.atc"];

      logs.push("");
      logs.push("--- STDOUT ---");
      logs.push("System: Booting ATCLang VM 2.0 Native Execution");
      logs.push("AIEngine: AI Sentiment Analysis: [POSITIVE] 0.99942");
      logs.push("NFTLayer: Artifact forged & secured. Signature: 0xATC_HASH_187291a8f9...");
      
      logs.push("");
      logs.push("--- EXECUTION PROFILE ---");
      logs.push("Process exited with code 0 (Success).");
      logs.push("Gas Consumed: 14,240 ATC units (Optimized)");
      logs.push("Memory Peak: 104 KB");
      logs.push("Total execution time: 1.2ms");

      setOutput(logs);
      setIsRunning(false);
    }, 1200);
  };

  const handlePublish = () => {
    setIsPublishing(true);
    // Switch to output panel if not already
    setIsAiMode(false);
    setIsPackMode(false);
    let logs = [
      "> Bundling ATCLang Project...",
      "> Generating ZK-Proofs for Marketplace verification...",
    ];
    setOutput(logs);
    
    setTimeout(() => {
      logs.push("  ✓ Proofs verified.");
      logs.push("> Deploying to Decentralized Marketplace...");
      setOutput([...logs]);
      
      setTimeout(() => {
        logs.push("  ✓ Successfully published 'Current Project' to ATC Store!");
        logs.push("  Public Address: 0xATC_MARKET_PUB_8b41...");
        setOutput([...logs]);
        setIsPublishing(false);
        alert("Software has been published to the Marketplace successfully!");
      }, 1500);
    }, 1000);
  };

  const handleEditorChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setFiles((prev) => ({
      ...prev,
      [activeFile]: e.target.value,
    }));
  };

  const [isPackMode, setIsPackMode] = useState(false);

  // Group files by directory
  const rootFiles: string[] = [];
  const folders: Record<string, string[]> = {};

  Object.keys(files).forEach((path) => {
    if (path.includes("/")) {
      const parts = path.split("/");
      const folder = parts[0];
      if (!folders[folder]) folders[folder] = [];
      if (!folders[folder].includes(path)) {
        folders[folder].push(path);
      }
    } else {
      if (!rootFiles.includes(path)) {
        rootFiles.push(path);
      }
    }
  });

  const createNewFile = () => {
    const filename = prompt("Enter new filename (e.g. system/custom.atclib):");
    if (filename && !files[filename]) {
      setFiles(prev => ({...prev, [filename]: `// ${filename} \n\n`}));
      setActiveFile(filename);
      if (filename.includes("/")) {
        setOpenFolders(prev => ({...prev, [filename.split('/')[0]]: true}));
      }
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200 border border-atc-border/50 rounded-xl overflow-hidden font-sans relative">
      {/* Header Bar */}
      <div className="flex items-center justify-between px-4 h-14 bg-[#090b14] border-b border-atc-border/50">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-indigo-500/10 border border-indigo-500/30">
            <Code2 className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h2 className="font-bold text-white tracking-tight leading-tight flex items-center gap-2">
              ATCLang Compiler IDE
              <span className="px-2 py-0.5 rounded text-[10px] bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 font-mono tracking-widest">
                NATIVE ENGINE
              </span>
            </h2>
            <p className="text-[10px] text-slate-400 font-mono hidden sm:block">
              Vollständige Software mit der eigenen Programmiersprache atc-lang bauen
            </p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={() => {
              setIsPackMode(!isPackMode);
              if (!isPackMode) setIsAiMode(false);
            }}
            className={`hidden md:flex items-center gap-2 px-3 py-1.5 rounded-lg border transition-colors text-xs font-bold ${isPackMode ? "bg-fuchsia-500/10 border-fuchsia-500/30 text-fuchsia-400" : "bg-white/5 border-white/10 text-slate-400 hover:text-white"}`}
          >
            <Layers className="w-4 h-4" /> atc-pack (PM)
          </button>
          <button
            onClick={() => {
              setIsAiMode(!isAiMode);
              if (!isAiMode) setIsPackMode(false);
            }}
            className={`hidden md:flex items-center gap-2 px-3 py-1.5 rounded-lg border transition-colors text-xs font-bold ${isAiMode ? "bg-amber-500/10 border-amber-500/30 text-amber-400" : "bg-white/5 border-white/10 text-slate-400 hover:text-white"}`}
          >
            <Bot className="w-4 h-4" /> ATC AI Pilot
          </button>
          <button
            onClick={handlePublish}
            disabled={isPublishing}
            className="hidden md:flex items-center gap-2 px-3 py-1.5 rounded-lg border transition-colors text-xs font-bold bg-blue-500/10 border-blue-500/30 text-blue-400 hover:bg-blue-500/20 disabled:opacity-50"
          >
            <Globe className="w-4 h-4" /> {isPublishing ? "Publishing..." : "Publish to Marketplace"}
          </button>
          <div className="hidden sm:flex items-center gap-2 text-xs font-mono text-emerald-400 bg-emerald-500/10 px-3 py-1.5 rounded-lg border border-emerald-500/20">
            <Activity className="w-3 h-3" /> Runtime: Active
          </div>
          <button
            onClick={runCode}
            disabled={isRunning}
            className="flex items-center gap-2 px-4 sm:px-6 py-1.5 rounded-lg bg-indigo-500 text-white font-bold transition-all hover:bg-indigo-600 shadow-[0_0_15px_rgba(99,102,241,0.3)] disabled:opacity-50 disabled:cursor-not-allowed text-sm"
          >
            {isRunning ? (
              <RefreshCw className="w-4 h-4 animate-spin" />
            ) : (
              <Play className="w-4 h-4 fill-current" />
            )}
            BUILD
          </button>
        </div>
      </div>

      <div className="flex flex-1 min-h-0">
        {/* File Explorer Sidebar */}
        <div className="w-56 sm:w-64 bg-[#090b14]/50 border-r border-atc-border/50 flex flex-col pt-2 select-none overflow-y-auto custom-scrollbar shrink-0">
          <div className="px-4 py-2 text-xs font-bold text-slate-500 uppercase tracking-widest flex items-center justify-between mb-2">
            <span className="flex items-center gap-2"><Package className="w-4 h-4" /> Workspace</span>
            <button onClick={createNewFile} className="text-xl font-light hover:text-indigo-400 leading-none pb-1" title="New File">+</button>
          </div>

          <div className="flex flex-col space-y-0.5 px-2">
            {/* Root Files */}
            {rootFiles.map((file) => (
              <div
                key={file}
                onClick={() => setActiveFile(file)}
                className={`flex items-center gap-2 px-2 py-1.5 rounded-md cursor-pointer text-sm ml-2 mt-1 ${activeFile === file ? "bg-indigo-500/20 text-indigo-300 font-medium" : "hover:bg-white/5 text-slate-400 hover:text-slate-200"}`}
              >
                {getFileIcon(file)}
                <span className="truncate">{file}</span>
              </div>
            ))}

            {/* Folders */}
            <div className="mt-2"></div>
            {Object.entries(folders).map(([folderName, folderFiles]) => (
              <div key={folderName}>
                <div
                  onClick={() => toggleFolder(folderName)}
                  className="flex items-center gap-2 px-2 py-1.5 rounded-md hover:bg-white/5 cursor-pointer text-sm text-slate-300"
                >
                  {openFolders[folderName] ? (
                    <ChevronDown className="w-4 h-4 text-slate-500 transition-transform" />
                  ) : (
                    <ChevronRight className="w-4 h-4 text-slate-500 transition-transform" />
                  )}
                  {openFolders[folderName] ? (
                    <FolderOpen className="w-4 h-4 text-amber-400" />
                  ) : (
                    <Folder className="w-4 h-4 text-amber-500" />
                  )}
                  <span className="truncate">{folderName}</span>
                </div>

                <AnimatePresence>
                  {openFolders[folderName] && (
                    <motion.div
                      initial={{ height: 0, opacity: 0 }}
                      animate={{ height: "auto", opacity: 1 }}
                      exit={{ height: 0, opacity: 0 }}
                      className="overflow-hidden flex flex-col ml-6 pl-2 border-l border-white/10"
                    >
                      {folderFiles.map((file) => (
                        <div
                          key={file}
                          onClick={() => setActiveFile(file)}
                          className={`flex items-center gap-2 px-2 py-1.5 rounded-md cursor-pointer text-sm mb-0.5 ${activeFile === file ? "bg-indigo-500/20 text-indigo-300 font-medium" : "hover:bg-white/5 text-slate-400 hover:text-slate-200"}`}
                        >
                          {getFileIcon(file)}
                          <span className="truncate">{file.split("/")[1]}</span>
                        </div>
                      ))}
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            ))}
          </div>

          <div className="mt-6 px-4 py-2 border-t border-white/5">
             <h4 className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-3">Scaffold Project</h4>
             <div className="flex flex-col gap-2">
                <button onClick={() => createProject('dex')} className="flex items-center gap-2 justify-start text-xs bg-black/40 hover:bg-indigo-500/20 text-slate-300 hover:text-indigo-300 px-3 py-2 rounded-lg border border-white/5 transition-colors">
                   <Activity className="w-3.5 h-3.5 text-emerald-400" /> DeFi Dex Core
                </button>
             </div>
          </div>

          <div className="mt-auto p-4 flex flex-col gap-2">
            <div className="bg-black/40 rounded-lg p-3 border border-white/5">
              <h4 className="text-xs font-bold text-slate-400 mb-2 uppercase tracking-wide">
                Library Types
              </h4>
              <ul className="text-[10px] space-y-1.5 text-slate-500 font-mono">
                <li className="flex items-center gap-1.5">
                  <FileCode className="w-3 h-3 text-emerald-400" /> .atc (Core Logic)
                </li>
                <li className="flex items-center gap-1.5">
                  <Layers className="w-3 h-3 text-fuchsia-400" /> .atcnft (NFT Schema)
                </li>
                <li className="flex items-center gap-1.5">
                  <Library className="w-3 h-3 text-blue-400" /> .atclib (Module)
                </li>
                <li className="flex items-center gap-1.5">
                  <Cpu className="w-3 h-3 text-amber-500" /> .atcop (Hardware Op)
                </li>
              </ul>
            </div>
          </div>
        </div>

        {/* Editor Area */}
        <div className="flex-1 flex flex-col border-r border-atc-border/50 relative">
          <div className="flex h-10 items-center px-4 bg-[#090b14]/80 border-b border-atc-border/50 text-xs font-mono font-medium text-slate-300 gap-2">
            {getFileIcon(activeFile)} {activeFile}
          </div>

          <div className="flex-1 relative flex bg-[#04060d]">
            {/* Line Numbers */}
            <div className="w-12 bg-[#060812] border-r border-atc-border/50 flex flex-col text-right pr-3 py-4 text-xs font-mono text-slate-600 select-none overflow-hidden">
              {files[activeFile] && files[activeFile].split("\\n").map((_, i) => (
                <div key={i}>{i + 1}</div>
              ))}
            </div>
            {/* Textarea */}
            <textarea
              value={files[activeFile] || ""}
              onChange={handleEditorChange}
              className="flex-1 p-4 bg-transparent outline-none resize-none font-mono text-sm leading-relaxed text-slate-300 custom-scrollbar"
              spellCheck={false}
            />
          </div>
        </div>

        {/* Output Panel + AI Assistant Panel (Split or toggled) */}
        {!isAiMode && !isPackMode ? (
          <div className="w-[300px] lg:w-[400px] bg-[#050810] flex flex-col hidden md:flex shrink-0">
            <div className="flex h-10 items-center px-4 bg-[#090b14]/80 border-b border-atc-border/50 text-xs font-mono font-medium text-slate-400 gap-2">
              <Terminal className="w-4 h-4" /> COMPILER STDOUT
            </div>
            <div className="flex-1 p-4 font-mono text-xs overflow-y-auto custom-scrollbar flex flex-col gap-1.5 leading-relaxed">
              {output.map((line, idx) => (
                <div
                  key={idx}
                  className={`${line.includes("Error") ? "text-red-400" : line.includes("ATCLang") ? "text-indigo-400 font-bold text-sm mb-1" : line.includes(">") ? "text-emerald-400 font-bold mb-1" : "text-slate-300"}`}
                >
                  {line}
                </div>
              ))}
              {isRunning && (
                <div className="flex items-center gap-2 text-indigo-400 animate-pulse mt-2 font-bold">
                  <Cpu className="w-4 h-4" /> Compiling native binary...
                </div>
              )}
            </div>
          </div>
        ) : isPackMode ? (
          <div className="w-[300px] lg:w-[400px] bg-[#050608] flex flex-col hidden md:flex shrink-0 border-l border-fuchsia-500/20">
             <div className="flex h-10 items-center px-4 bg-fuchsia-500/10 border-b border-fuchsia-500/20 text-xs font-mono font-bold text-fuchsia-400 gap-2">
               <Layers className="w-4 h-4" /> ATC-PACK MANAGER
             </div>
             <div className="flex-1 p-4 overflow-y-auto custom-scrollbar flex flex-col gap-4 text-xs font-mono">
                <div className="p-3 bg-black/40 border border-white/5 rounded-xl text-slate-300">
                   <p className="text-fuchsia-400 mb-2 font-bold flex items-center gap-2"><Activity className="w-3.5 h-3.5"/> NATIVE REPOSITORY</p>
                   <p className="mb-2 text-slate-400">Install Zero-Knowledge dependencies directly into the compiler. These are fully audited and native to the ATC kernel.</p>
                </div>
                
                <div className="flex flex-col gap-2">
                   <div className="p-3 bg-black/60 border border-fuchsia-500/10 rounded-lg flex flex-col gap-2">
                      <div className="flex justify-between items-center text-slate-300 font-bold">
                         <span>@std/zk-identity v1.2</span>
                         <button className="px-2 py-1 bg-fuchsia-500/20 hover:bg-fuchsia-500/40 text-fuchsia-300 rounded">Install</button>
                      </div>
                      <p className="text-[10px] text-slate-500">Implements privacy-preserving identity verification using bulletproofs.</p>
                   </div>
                   
                   <div className="p-3 bg-black/60 border border-fuchsia-500/10 rounded-lg flex flex-col gap-2">
                      <div className="flex justify-between items-center text-slate-300 font-bold">
                         <span>@defi/amm-core v2.0</span>
                         <button className="px-2 py-1 bg-fuchsia-500/20 hover:bg-fuchsia-500/40 text-fuchsia-300 rounded">Install</button>
                      </div>
                      <p className="text-[10px] text-slate-500">Constant product formula implementation optimized for ATVM.</p>
                   </div>
                </div>
             </div>
          </div>
        ) : (
          <div className="w-[300px] lg:w-[400px] bg-[#050608] flex flex-col hidden md:flex shrink-0 border-l border-amber-500/20">
            <div className="flex h-10 items-center px-4 bg-amber-500/10 border-b border-amber-500/20 text-xs font-mono font-bold text-amber-400 gap-2">
              <Sparkles className="w-4 h-4" /> ATC AI PILOT (NATIVE NPU)
            </div>
            <div className="flex-1 p-4 overflow-y-auto custom-scrollbar flex flex-col gap-4">
              {aiPrompts.map((p, i) => (
                <div key={i} className={`flex flex-col gap-1 ${p.role === 'user' ? 'items-end' : 'items-start'}`}>
                  <div className={`px-3 py-2 rounded-xl text-sm max-w-[90%] ${p.role === 'user' ? 'bg-indigo-500/20 text-slate-200 border border-indigo-500/30' : 'bg-black/40 text-slate-300 border border-amber-500/20'}`}>
                    {p.text}
                  </div>
                </div>
              ))}
            </div>
            <form onSubmit={handleAiSubmit} className="p-3 border-t border-white/10 bg-black/40 flex items-center gap-2">
               <input 
                 value={currentAiInput}
                 onChange={e => setCurrentAiInput(e.target.value)}
                 className="flex-1 bg-white/5 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-amber-500/50"
                 placeholder="Ask ATC AI Pilot..."
               />
               <button type="submit" className="p-2 bg-amber-500/20 text-amber-400 hover:bg-amber-500/30 rounded-lg border border-amber-500/30">
                 <Send className="w-4 h-4" />
               </button>
            </form>
          </div>
        )}
      </div>
    </div>
  );
}
