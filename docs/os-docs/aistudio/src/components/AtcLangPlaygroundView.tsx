import React, { useState } from 'react';
import { Play, Code2, Terminal, Cpu, Database, Save, CheckCircle, FileJson, Zap, Link } from 'lucide-react';

export function AtcLangPlaygroundView() {
  const [code, setCode] = useState(`// A-TownChain (ATC-Lang) Smart Contract Example
layer TokenBridge {
    field admin: Address;
    field balances: Map<Address, §Asset>;

    // Constructor runs once upon instantiation
    routine init() {
        admin = System.sender;
    }

    // Native asset handling prevents cloning
    @payable
    routine deposit(amount: §Asset) -> bool {
        balances[System.sender].push(move amount);
        emit Deposited(System.sender, amount.value);
        return true;
    }

    // ZK proof generation routing
    @zk_proof
    routine verifyOrigin(proof: Bytes) -> bool {
        return Quantum.verify(proof);
    }
}`);
  const [output, setOutput] = useState('');
  const [isCompiling, setIsCompiling] = useState(false);
  const [isCompiled, setIsCompiled] = useState(false);
  const [activeTab, setActiveTab] = useState<'editor'|'abi'|'interact'>('editor');
  const [deployedAddress, setDeployedAddress] = useState('');
  const [balances, setBalances] = useState<Record<string, number>>({'0xUSER_WALLET': 0});

  const handleCompile = () => {
    setIsCompiling(true);
    setIsCompiled(false);
    setOutput('Compiling ATC-Lang to ATVM Bytecode...\nLoading standard libraries...\n');
    
    setTimeout(() => {
      setOutput(prev => prev + '\n[LEXER] Tokenizing input...\n  > Extracted 42 Tokens.\n  > Keywords matched: layer, field, routine, move.\n');
    }, 500);

    setTimeout(() => {
      setOutput(prev => prev + '\n[PARSER] Generating Abstract Syntax Tree (AST)...\n  > Validated LayerDecl(TokenBridge)\n  > Validated RoutineDecl(init)\n  > Validated RoutineDecl(deposit)\n  > Validated RoutineDecl(verifyOrigin)\n');
    }, 1200);

    setTimeout(() => {
      setOutput(prev => prev + '\n[SEMANTICS] Running Type-Safety pass...\n  > Validated §Asset move semantics.\n  > Resolving @zk_proof decorator logic...\n');
    }, 1800);

    setTimeout(() => {
      setOutput(prev => prev + '\nSUCCESS: Compiled successfully in 2.2s.\nBytecode Hash: 0x9f86d081884c7d659a2... \nGas Estimate: 45,210 ATC units.');
      setIsCompiling(false);
      setIsCompiled(true);
      setActiveTab('abi');
    }, 2400);
  };

  const handleDeploy = () => {
    setOutput(prev => prev + '\n\n> Deploying contract to local ATVM sandbox...');
    setTimeout(() => {
      setDeployedAddress('0xATC_' + Math.random().toString(16).substr(2, 8).toUpperCase());
      setActiveTab('interact');
      setOutput(prev => prev + '\n> Contract deployed successfully!');
    }, 800);
  }

  const handleInteract = () => {
     setBalances(prev => ({...prev, '0xUSER_WALLET': prev['0xUSER_WALLET'] + 50}));
     setOutput(prev => prev + `\n> TX Executed: deposit() called with 50 §Asset.\n  Gas used: 24,101`);
  }

  return (
    <div className="flex flex-col gap-6 mt-8 pb-12 font-sans w-full max-w-6xl mx-auto">
      <div className="mb-4 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-atc-cyan/10 border border-atc-cyan/30 flex items-center justify-center text-atc-cyan shadow-[0_0_15px_rgba(0,209,255,0.2)]">
            <Code2 className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
              ATC-Lang Playground & Compiler
            </h2>
            <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
              Interaktive Entwicklungsumgebung für A-TownChain Smart Contracts, ZK-Circuits und System-Module.
            </p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <button className="flex items-center gap-2 px-4 py-2 border border-white/10 hover:border-white/30 hover:bg-white/5 rounded-lg text-sm text-white transition-colors">
            <Save className="w-4 h-4 text-slate-400" /> Save Snippet
          </button>
          <button 
            onClick={handleCompile}
            disabled={isCompiling}
            className="flex items-center gap-2 px-5 py-2 bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/40 hover:bg-atc-cyan/30 rounded-lg text-sm font-bold transition-colors disabled:opacity-50"
          >
           {isCompiling ? <Cpu className="w-4 h-4 animate-spin" /> : <Play className="w-4 h-4" />}
           {isCompiling ? 'Compiling...' : 'Run Compiler'}
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 h-[600px]">
        {/* Left pane: Editor or alternative views */}
        <div className="flex flex-col border border-white/10 rounded-xl overflow-hidden bg-[#090b14] h-full shadow-lg">
          <div className="flex items-center px-4 bg-black/40 border-b border-white/10">
             <button onClick={() => setActiveTab('editor')} className={`py-3 px-4 text-xs font-mono uppercase tracking-widest border-b-2 transition-colors ${activeTab === 'editor' ? 'border-atc-cyan text-atc-cyan' : 'border-transparent text-slate-400 hover:text-slate-300'}`}>
                Source Code
             </button>
             <button disabled={!isCompiled} onClick={() => setActiveTab('abi')} className={`py-3 px-4 text-xs font-mono uppercase tracking-widest border-b-2 transition-colors disabled:opacity-30 disabled:cursor-not-allowed ${activeTab === 'abi' ? 'border-atc-cyan text-atc-cyan' : 'border-transparent text-slate-400 hover:text-slate-300'}`}>
                Bytecode & ABI
             </button>
             <button disabled={!deployedAddress} onClick={() => setActiveTab('interact')} className={`py-3 px-4 text-xs font-mono uppercase tracking-widest border-b-2 transition-colors disabled:opacity-30 disabled:cursor-not-allowed ${activeTab === 'interact' ? 'border-atc-cyan text-atc-cyan' : 'border-transparent text-slate-400 hover:text-slate-300'}`}>
                Interact
             </button>
          </div>
          
          <div className="flex-1 relative overflow-auto">
             {activeTab === 'editor' && (
                <textarea
                  value={code}
                  onChange={(e) => setCode(e.target.value)}
                  className="w-full h-full p-4 bg-transparent text-slate-300 font-mono text-sm focus:outline-none resize-none leading-relaxed"
                  spellCheck={false}
                />
             )}
             
             {activeTab === 'abi' && (
               <div className="p-4 flex flex-col gap-6 w-full h-full">
                  <div className="p-4 bg-black/50 border border-white/5 rounded-xl">
                     <h4 className="text-xs font-bold text-slate-500 uppercase flex items-center gap-2 mb-3"><FileJson className="w-4 h-4"/> Interface (ABI)</h4>
                     <pre className="text-[10px] text-fuchsia-400 font-mono leading-relaxed">
{`[
  {
    "type": "routine",
    "name": "deposit",
    "inputs": [
      { "name": "amount", "type": "§Asset" }
    ],
    "outputs": [
      { "name": "", "type": "bool" }
    ],
    "decorators": ["@payable"]
  },
  {
    "type": "routine",
    "name": "verifyOrigin",
    "inputs": [
      { "name": "proof", "type": "Bytes" }
    ],
    "outputs": [
      { "name": "", "type": "bool" }
    ],
    "decorators": ["@zk_proof"]
  },
  {
    "type": "event",
    "name": "Deposited",
    "inputs": [
      { "name": "user", "type": "Address", "indexed": true },
      { "name": "amount", "type": "u256", "indexed": false }
    ]
  }
]`}
                     </pre>
                  </div>
                  {!deployedAddress && (
                     <button onClick={handleDeploy} className="mt-auto py-3 bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 rounded-xl font-bold uppercase tracking-widest text-xs hover:bg-indigo-500/30 transition-colors flex items-center justify-center gap-2">
                        <Zap className="w-4 h-4"/> Deploy to Sandbox ATVM
                     </button>
                  )}
               </div>
             )}

             {activeTab === 'interact' && (
               <div className="p-4 flex flex-col gap-6 w-full h-full">
                  <div className="p-4 bg-emerald-500/10 border border-emerald-500/20 rounded-xl">
                     <p className="text-xs text-emerald-400 font-mono flex items-center gap-2"><CheckCircle className="w-4 h-4"/> Contract live at: {deployedAddress}</p>
                  </div>

                  <div className="p-4 bg-white/5 border border-white/10 rounded-xl">
                     <h4 className="text-sm font-bold text-white mb-4">Read Operations</h4>
                     <div className="flex items-center justify-between p-3 bg-black/40 rounded-lg">
                        <span className="font-mono text-xs text-slate-300">balances('0xUSER_WALLET')</span>
                        <span className="font-mono text-xs text-atc-cyan font-bold">{balances['0xUSER_WALLET']} ATC</span>
                     </div>
                  </div>

                  <div className="p-4 bg-white/5 border border-white/10 rounded-xl">
                     <h4 className="text-sm font-bold text-white mb-4">Write Operations</h4>
                     <div className="flex items-center gap-2">
                        <input type="number" defaultValue="50" className="w-24 bg-black/40 border border-white/10 rounded-lg py-2 px-3 text-sm text-white focus:outline-none focus:border-indigo-500" />
                        <button onClick={handleInteract} className="flex-1 py-2 bg-rose-500/20 text-rose-400 border border-rose-500/30 hover:bg-rose-500/30 rounded-lg font-bold text-sm transition-colors flex items-center justify-center gap-2">
                           <Play className="w-3 h-3"/> deposit() (payable)
                        </button>
                     </div>
                  </div>
               </div>
             )}
          </div>
        </div>

        {/* Terminal / Output */}
        <div className="flex flex-col gap-6 h-full">
           <div className="flex-1 border border-white/10 rounded-xl overflow-hidden bg-[#090b14] flex flex-col shadow-lg">
              <div className="flex items-center px-4 py-2 border-b border-white/10 bg-black/40">
                <div className="flex items-center gap-2 text-slate-400 text-xs font-mono uppercase tracking-widest">
                  <Terminal className="w-4 h-4 text-slate-500" /> Compiler Output
                </div>
              </div>
              <div className="flex-1 p-4 bg-black/60 overflow-y-auto custom-scrollbar">
                 {output ? (
                   <pre className="text-xs font-mono whitespace-pre-wrap leading-relaxed">
                     {output.split('\n').map((line, i) => (
                        <div key={i} className={line.includes('SUCCESS') ? 'text-emerald-400' : line.includes('Deploy') ? 'text-indigo-400 font-bold' : line.includes('TX Executed') ? 'text-rose-400' : 'text-slate-300'}>
                          {line}
                        </div>
                     ))}
                   </pre>
                 ) : (
                   <div className="h-full flex flex-col items-center justify-center text-slate-600">
                     <Cpu className="w-8 h-8 mb-2 opacity-50" />
                     <p className="text-sm font-mono">Waiting for compilation...</p>
                   </div>
                 )}
              </div>
           </div>

           <div className="h-48 border border-white/10 rounded-xl overflow-hidden bg-[#090b14] p-4 flex flex-col justify-center shadow-lg">
             <div className="flex items-center justify-between mb-4">
                <h4 className="text-xs font-bold uppercase tracking-widest text-slate-400 flex items-center gap-2">
                  <Database className="w-4 h-4" /> VM State Inspection
                </h4>
                <div className="px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 text-[10px] font-bold uppercase flex items-center gap-1">
                   <CheckCircle className="w-3 h-3" /> Ready
                </div>
             </div>
             <div className="grid grid-cols-2 gap-4">
                <div className="p-3 bg-white/5 rounded border border-white/5">
                   <p className="text-[10px] text-slate-500 font-mono mb-1">ALLOCATED MEMORY</p>
                   <p className="text-lg font-mono text-white">1.04 MB</p>
                </div>
                <div className="p-3 bg-white/5 rounded border border-white/5">
                   <p className="text-[10px] text-slate-500 font-mono mb-1">OPCODES GENERATED</p>
                   <p className="text-lg font-mono text-white text-atc-cyan">{isCompiled ? '482' : '0'}</p>
                </div>
             </div>
           </div>
        </div>
      </div>
    </div>
  );
}
