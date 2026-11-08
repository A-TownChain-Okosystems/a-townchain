import React, { useState, useEffect } from 'react';
import { Play, Code, Cpu, Database, CheckCircle, Terminal, Download, Loader2 } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export default function AtvmSandboxView() {
  const [code, setCode] = useState(() => {
    const saved = localStorage.getItem('atc_sandbox_code');
    if (saved) return saved;
    return `// ATC-Lang Example: Basic Transfer
contract Token {
  @private
  let balances: Map<Address, u64>;

  fn transfer(to: Address, amount: u64) -> bool {
    // Hoare-Logic pre-condition
    assert!(self.balances[msg.sender] >= amount);
    
    self.balances[msg.sender] -= amount;
    self.balances[to] += amount;
    
    // Gas metering is auto-injected
    return true;
  }
}
`;
  });
  const [isCompiling, setIsCompiling] = useState(false);
  const [activeTab, setActiveTab] = useState<'editor' | 'ast' | 'bytecode' | 'execution' | 'systems'>('editor');
  const [bytecode, setBytecode] = useState('');
  
  const OS_CATALOG: Record<string, { name: string, category: string, icon: string, desc: string }> = {
    'atc-native': { name: 'ATC-OS Native Kernel', category: 'atc-native', icon: '🎯', desc: 'Native WASM sandbox optimized for ATC Contracts.' },
    'win10': { name: 'Windows 10', category: 'windows', icon: '🪟', desc: 'Legacy Windows API abstraction.' },
    'win11': { name: 'Windows 11', category: 'windows', icon: '🪟', desc: 'Modern Windows Subsystem for ATC.' },
    'win-server': { name: 'Windows Server 2022', category: 'windows', icon: '🪟', desc: 'Enterprise server container.' },
    'ubuntu22': { name: 'Ubuntu 22.04 LTS', category: 'linux', icon: '🐧', desc: 'Stable Debian-based Linux environment.' },
    'ubuntu24': { name: 'Ubuntu 24.04 LTS', category: 'linux', icon: '🐧', desc: 'Latest LTS Ubuntu Container.' },
    'debian12': { name: 'Debian 12 Bookworm', category: 'linux', icon: '🐧', desc: 'Ultra-stable base server Linux.' },
    'alpine': { name: 'Alpine Linux 3.19', category: 'linux', icon: '🐧', desc: 'Minimal footprint root filesystem.' },
  };

  const [installedOS, setInstalledOS] = useState<string[]>(['atc-native', 'win11', 'ubuntu24']);
  const [osEnv, setOsEnv] = useState(() => localStorage.getItem('atc_os_env') || 'atc-native');

  useEffect(() => {
    localStorage.setItem('atc_os_env', osEnv);
    window.dispatchEvent(new Event('atc_os_env_changed'));
  }, [osEnv]);
  const [installingStatus, setInstallingStatus] = useState<string | null>(null);
  
  const [logs, setLogs] = useState<string[]>([]);
  const [registers, setRegisters] = useState<{r1: number, r2: number, gas: number}>({ r1: 0, r2: 0, gas: 100000 });
  const [showSaveToast, setShowSaveToast] = useState(false);
  const [lastSaveTime, setLastSaveTime] = useState<string>('');

  useEffect(() => {
    const t1 = setTimeout(() => {
      localStorage.setItem('atc_sandbox_code', code);
      setLastSaveTime(new Date().toLocaleTimeString());
      setShowSaveToast(true);
      const t2 = setTimeout(() => setShowSaveToast(false), 2000);
      return () => clearTimeout(t2);
    }, 500); // debounce
    return () => clearTimeout(t1);
  }, [code]);

  const handleCompile = () => {
    setIsCompiling(true);
    setLogs(prev => [...prev, `[ALC] Submitting to ${OS_CATALOG[osEnv]?.name || osEnv} orchestrator...`, '[ALC] Starting compilation...']);

    if (code.includes('@private') && localStorage.getItem('zk_circuit_valid') !== 'true') {
      setTimeout(() => {
        setLogs(prev => [
          ...prev,
          '[ALC] Error: Contract uses @private state but ZK Circuit is not validated!',
          '[ALC] Please use the ZK Circuit Editor to compile valid constraints first.',
          '[ALC] Compilation aborted.'
        ]);
        setIsCompiling(false);
      }, 500);
      return;
    }

    if (!code.includes('contract')) {
      setTimeout(() => {
        setLogs(prev => [
          ...prev, 
          '[ALC] SyntaxError: Expected keyword "contract" at top-level.', 
          '[ALC] Compilation aborted.'
        ]);
        setIsCompiling(false);
      }, 500);
      return;
    }

    setTimeout(() => {
      setBytecode(`0x0001: LOAD_VAR msg.sender, R1
0x0002: LOAD_VAR balances[R1], R2
0x0003: CMP R2, amount
0x0004: JMP_LT 0x000E ; Revert if less
0x0005: SUB R2, amount
0x0006: STORE balances[R1], R2
0x0007: LOAD_VAR to, R1
0x0008: LOAD_VAR balances[R1], R2
0x0009: ADD R2, amount
0x000A: STORE balances[R1], R2
0x000B: CONSUME_GAS 420
0x000C: PUSH true
0x000D: RET
0x000E: REVERT "Insufficient balance"`);
      setLogs(prev => [...prev, '[ALC] AST verified.', `[ALC] Bound to ${OS_CATALOG[osEnv]?.name || osEnv} sys-calls.`, '[ALC] Compiled to ATVM bytecode.']);
      setIsCompiling(false);
      setActiveTab('bytecode');
    }, 1500);
  };

  const handleExecute = () => {
    setActiveTab('execution');
    setLogs(prev => [...prev, `[ATVM] Booting ${OS_CATALOG[osEnv]?.name || osEnv} sub-kernel...`, '[ATVM] Executing bytecode...']);
    let gas = registers.gas;
    let step = 0;
    
    const interval = setInterval(() => {
      if (step > 5) {
        clearInterval(interval);
        setLogs((prev: string[]) => [...prev, '[ATVM] Execution completed successfully.']);
        setRegisters((prev: any) => ({ ...prev, gas: prev.gas - 420, r1: 0, r2: 0 }));
        return;
      }
      gas -= 10;
      setRegisters((prev: any) => ({ ...prev, gas, r1: Math.floor(Math.random() * 100), r2: Math.floor(Math.random() * 100) }));
      step++;
    }, 500);
  };

  return (
    <div className="relative flex flex-col h-full bg-[#090b14] rounded-xl border border-atc-border overflow-hidden">
      <div className="flex items-center justify-between px-4 py-3 border-b border-atc-border bg-[#0a0d18] shrink-0">
        <div className="flex items-center gap-3">
          <Terminal className="w-5 h-5 text-atc-cyan" />
          <h2 className="text-white font-mono font-medium tracking-wide">ATVM Sandbox</h2>
        </div>
        <div className="flex gap-2 relative z-10">
          <select 
            value={osEnv}
            onChange={(e) => setOsEnv(e.target.value)}
            className="px-3 py-1.5 rounded-md text-xs font-bold font-mono bg-[#090b14] border border-white/10 text-atc-cyan focus:outline-none focus:border-atc-cyan transition-colors"
          >
            {installedOS.map(id => (
              <option key={id} value={id}>
                {OS_CATALOG[id]?.icon} {OS_CATALOG[id]?.name}
              </option>
            ))}
          </select>
          <button 
            onClick={() => setActiveTab('editor')}
            className={`px-3 py-1.5 rounded-md text-xs font-mono transition-colors border cursor-pointer ${activeTab === 'editor' ? 'bg-atc-cyan/10 text-atc-cyan border-atc-cyan/30' : 'border-transparent text-slate-400 hover:bg-slate-800'}`}
          >
            Editor
          </button>
          <button 
            onClick={() => setActiveTab('ast')}
            className={`px-3 py-1.5 rounded-md text-xs font-mono transition-colors border cursor-pointer ${activeTab === 'ast' ? 'bg-atc-cyan/10 text-atc-cyan border-atc-cyan/30' : 'border-transparent text-slate-400 hover:bg-slate-800'}`}
          >
            AST
          </button>
          <button 
            onClick={() => setActiveTab('bytecode')}
            className={`px-3 py-1.5 rounded-md text-xs font-mono transition-colors border cursor-pointer ${activeTab === 'bytecode' ? 'bg-atc-cyan/10 text-atc-cyan border-atc-cyan/30' : 'border-transparent text-slate-400 hover:bg-slate-800'}`}
          >
            Bytecode
          </button>
          <button 
            onClick={() => setActiveTab('execution')}
            className={`px-3 py-1.5 rounded-md text-xs font-mono transition-colors border cursor-pointer ${activeTab === 'execution' ? 'bg-atc-cyan/10 text-atc-cyan border-atc-cyan/30' : 'border-transparent text-slate-400 hover:bg-slate-800'}`}
          >
            Execution
          </button>
          <button 
            onClick={() => setActiveTab('systems')}
            className={`px-3 py-1.5 rounded-md text-xs font-mono transition-colors border cursor-pointer ${activeTab === 'systems' ? 'bg-atc-cyan/10 text-atc-cyan border-atc-cyan/30' : 'border-transparent text-slate-400 hover:bg-slate-800'}`}
          >
            OS Environments
          </button>
        </div>
      </div>
      
      <div className="flex flex-1 overflow-hidden">
        {/* Main Content Area */}
        <div className="flex-1 flex flex-col border-r border-atc-border overflow-hidden relative">
          {activeTab === 'editor' && (
            <div className="flex-1 p-4 relative">
              <textarea 
                value={code}
                onChange={(e) => setCode(e.target.value)}
                className="w-full h-full bg-transparent text-slate-300 font-mono text-sm resize-none focus:outline-none focus:ring-0 leading-relaxed"
                spellCheck="false"
              />
              <button 
                onClick={handleCompile}
                disabled={isCompiling}
                className="absolute bottom-6 right-6 px-4 py-2 bg-[#090b14] text-atc-cyan border border-atc-cyan/30 rounded-lg shadow-[0_0_15px_rgba(6,182,212,0.1)] hover:bg-atc-cyan/10 transition-all font-mono text-sm flex items-center gap-2 disabled:opacity-50 cursor-pointer"
              >
                {isCompiling ? <CheckCircle className="w-4 h-4 animate-spin text-atc-cyan" /> : <Code className="w-4 h-4" />}
                {isCompiling ? 'Compiling ALC...' : 'Compile to ATVM ->'}
              </button>
            </div>
          )}
          
          {activeTab === 'ast' && (
            <div className="flex-1 p-6 bg-[#050608] overflow-auto">
              <div className="max-w-2xl mx-auto w-full">
                <div className="bg-[#090b14] border border-atc-border rounded-xl p-4 overflow-x-auto">
                  <pre className="text-pink-400 font-mono text-xs leading-relaxed">
{`{
  "type": "Program",
  "body": [
    {
      "type": "ContractDeclaration",
      "name": "Token",
      "body": [
        {
          "type": "VariableDeclaration",
          "kind": "let",
          "visibility": "private",
          "identifier": "balances",
          "typeAnnotation": "Map<Address, u64>"
        },
        {
          "type": "FunctionDeclaration",
          "name": "transfer",
          "params": [
            { "name": "to", "type": "Address" },
            { "name": "amount", "type": "u64" }
          ],
          "returnType": "bool",
          "body": {
            "type": "BlockStatement",
            "body": [
              {
                "type": "AssertStatement",
                "condition": {
                  "type": "BinaryExpression",
                  "operator": ">=",
                  "left": {
                    "type": "MemberExpression",
                    "object": "self.balances",
                    "property": "msg.sender"
                  },
                  "right": "amount"
                }
              },
...
            ]
          }
        }
      ]
    }
  ]
}`}
                  </pre>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'bytecode' && (
            <div className="flex-1 p-6 bg-[#050608] overflow-auto">
              {bytecode ? (
                <div className="flex flex-col gap-4 max-w-2xl mx-auto">
                  <div className="p-4 bg-[#090b14] border border-atc-border rounded-xl">
                    <pre className="text-emerald-400 font-mono text-sm leading-relaxed">{bytecode}</pre>
                  </div>
                  <button 
                    onClick={handleExecute}
                    className="self-end px-4 py-2 bg-[#090b14] text-atc-purple border border-atc-purple/30 rounded-lg shadow-[0_0_15px_rgba(162,89,255,0.1)] hover:bg-atc-purple/10 transition-all font-mono text-sm flex items-center gap-2 mt-2 cursor-pointer"
                  >
                    <Play className="w-4 h-4 text-atc-purple" /> Execute Context
                  </button>
                </div>
              ) : (
                <div className="flex items-center justify-center h-full text-slate-600 font-mono text-sm">
                  No bytecode generated. Compile code first.
                </div>
              )}
            </div>
          )}
          
          {activeTab === 'execution' && (
            <div className="flex-1 p-6 flex flex-col bg-[#050608] overflow-auto">
              <div className="max-w-2xl mx-auto w-full">
                <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
                  <div className="bg-[#090b14] border border-emerald-500/30 border-l-4 border-l-emerald-500 rounded-xl p-4 col-span-2 sm:col-span-4 flex items-center justify-between">
                    <div className="flex flex-col">
                       <span className="text-[10px] text-emerald-400 font-bold uppercase tracking-widest">Active Host Environment</span>
                       <span className="text-lg text-white font-mono">{OS_CATALOG[osEnv]?.icon} {OS_CATALOG[osEnv]?.name || osEnv}</span>
                    </div>
                    <div className="text-[10px] text-slate-400 font-mono text-right">
                       <span>Arch: {OS_CATALOG[osEnv]?.category === 'atc-native' ? 'ATC-WASM' : 'x86_64-virt'}</span><br/>
                       <span>Allocated: 2048 MB RAM</span>
                    </div>
                  </div>
                  
                  {Math.random() >= 0 && (
                    <div className="col-span-2 sm:col-span-4 bg-[#090b14] border border-white/10 rounded-xl p-4 flex flex-col sm:flex-row gap-6 relative overflow-hidden group">
                       <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-indigo-500/10 via-transparent to-transparent opacity-50" />
                       <div className="flex-1 relative z-10">
                          <div className="flex items-center justify-between mb-3 text-[10px] font-mono text-slate-400 uppercase tracking-widest">
                             <span>CPU Utilization</span>
                             <span className="text-indigo-400 font-bold">{(Math.random() * 8 + 2).toFixed(1)}%</span>
                          </div>
                          <div className="h-6 w-full flex items-end gap-1 overflow-hidden" title="Live CPU usage">
                            {Array(20).fill(0).map((_, i) => (
                               <div key={i} className="flex-1 bg-indigo-500/80 rounded-t transition-all duration-300" style={{ height: `${Math.random() * 80 + 20}%`, opacity: 0.3 + (i / 30) }} />
                            ))}
                          </div>
                       </div>
                       <div className="w-[1px] h-full bg-white/10 hidden sm:block" />
                       <div className="flex-1 relative z-10">
                          <div className="flex items-center justify-between mb-3 text-[10px] font-mono text-slate-400 uppercase tracking-widest">
                             <span>Memory Usage (RSZ)</span>
                             <span className="text-rose-400 font-bold">{Math.floor(Math.random() * 150 + 400)} MB / 2048 MB</span>
                          </div>
                          <div className="w-full h-6 bg-black/40 rounded overflow-hidden relative" title="Live RAM Allocation">
                            <div className="absolute inset-0 bg-rose-500/20" style={{ width: '28%' }} />
                            <div className="absolute inset-0 bg-rose-500 rounded-r shadow-[0_0_10px_rgba(244,63,94,0.5)]" style={{ width: '28%', opacity: 0.8 }} />
                          </div>
                       </div>
                    </div>
                  )}

                  <div className="bg-[#090b14] border border-atc-border rounded-xl p-4">
                    <div className="text-[10px] text-slate-500 font-mono uppercase mb-2 flex items-center gap-1.5"><Database className="w-3.5 h-3.5 text-slate-400" /> Gas Limit</div>
                    <div className="text-2xl text-atc-cyan font-mono">{registers.gas}</div>
                  </div>
                  <div className="bg-[#090b14] border border-atc-border rounded-xl p-4">
                    <div className="text-[10px] text-slate-500 font-mono uppercase mb-2 flex items-center gap-1.5"><Cpu className="w-3.5 h-3.5 text-slate-400" /> Register 1</div>
                    <div className="text-2xl text-emerald-400 font-mono">0x{registers.r1.toString(16).padStart(4, '0')}</div>
                  </div>
                  <div className="bg-[#090b14] border border-atc-border rounded-xl p-4 hidden sm:block">
                    <div className="text-[10px] text-slate-500 font-mono uppercase mb-2 flex items-center gap-1.5"><Cpu className="w-3.5 h-3.5 text-slate-400" /> Register 2</div>
                    <div className="text-2xl text-emerald-400 font-mono">0x{registers.r2.toString(16).padStart(4, '0')}</div>
                  </div>
                </div>
                <div className="bg-[#090b14] border border-atc-border rounded-xl p-4 min-h-[200px]">
                  <div className="text-[10px] text-slate-500 font-mono uppercase mb-4 tracking-wider pb-2 border-b border-atc-border">Memory dump / Stack</div>
                  <pre className="text-slate-400 font-mono text-sm leading-relaxed">
                    {registers.gas < 100000 ? `State updated: \n0x00FA: balances[msg.sender] = ${registers.r1}\n0x00FB: balances[to] = ${registers.r2}\n\n[SUCCESS] TX_HASH: 0x8f43...b96` : 'Empty state bounds before execution.'}
                  </pre>
                </div>
              </div>
            </div>
          )}
          {activeTab === 'systems' && (
            <div className="flex-1 p-6 flex flex-col bg-[#050608] overflow-auto">
              <div className="max-w-4xl mx-auto w-full space-y-6">
                
                <div className="bg-[#090b14] border border-atc-border rounded-xl p-6">
                   <h3 className="text-xl font-bold text-white mb-2 tracking-tight">Virtualized OS Environments</h3>
                   <p className="text-slate-400 text-sm mb-6 max-w-2xl">
                     Manage and distribute alternative operating systems within the 100% deterministic ATC-VM sandbox. 
                     Select an environment below to set it as the active compilation and execution target for your smart contracts.
                   </p>

                   <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                     {Object.entries(OS_CATALOG).map(([id, os]) => {
                        const isInstalled = installedOS.includes(id);
                        const isActive = osEnv === id;
                        const isInstalling = installingStatus === id;

                        return (
                          <div 
                            key={id}
                            className={`border rounded-xl p-5 flex flex-col items-center text-center transition-all ${
                              isActive 
                                ? 'border-atc-cyan bg-atc-cyan/5 shadow-[0_0_20px_rgba(6,182,212,0.15)] scale-[1.02]' 
                                : isInstalled 
                                  ? 'border-white/20 bg-black/40 hover:bg-white/5 cursor-pointer'
                                  : 'border-white/5 bg-black/20 opacity-70 hover:opacity-100'
                            }`}
                            onClick={() => {
                              if (isInstalled && !isActive) setOsEnv(id);
                            }}
                          >
                            <div className={`w-12 h-12 rounded-lg border flex items-center justify-center text-2xl mb-4 ${
                               os.category === 'linux' ? 'bg-orange-500/10 border-orange-500/30' :
                               os.category === 'windows' ? 'bg-blue-500/10 border-blue-500/30' :
                               'bg-atc-cyan/10 border-atc-cyan/30'
                            }`}>
                               {os.icon}
                            </div>
                            <h4 className="text-white font-bold mb-1 text-sm">{os.name}</h4>
                            <span className={`text-[9px] uppercase font-mono mb-3 px-2 py-0.5 rounded ${
                               os.category === 'linux' ? 'text-orange-400 bg-orange-500/10' :
                               os.category === 'windows' ? 'text-blue-400 bg-blue-500/10' :
                               'text-atc-cyan bg-atc-cyan/10'
                            }`}>
                               {os.category}
                            </span>
                            <p className="text-xs text-slate-400 leading-relaxed mb-4 flex-1">
                              {os.desc}
                            </p>
                            
                            <div className="mt-auto w-full">
                               {isInstalling ? (
                                 <button disabled className="w-full py-1.5 rounded bg-indigo-500/20 text-indigo-400 text-xs font-bold uppercase tracking-widest flex items-center justify-center gap-2">
                                    <Loader2 className="w-3 h-3 animate-spin"/> Installing...
                                 </button>
                               ) : isInstalled ? (
                                 <button 
                                   className={`w-full py-1.5 rounded border text-xs font-bold uppercase tracking-widest transition-colors ${
                                      isActive ? 'bg-atc-cyan text-black border-atc-cyan' : 'bg-transparent border-white/10 text-slate-300 hover:bg-white/5'
                                   }`}
                                 >
                                    {isActive ? 'Active Edge' : 'Set Active'}
                                 </button>
                               ) : (
                                 <button 
                                   onClick={(e) => {
                                     e.stopPropagation();
                                     setInstallingStatus(id);
                                     setTimeout(() => {
                                        setInstalledOS(prev => [...prev, id]);
                                        setOsEnv(id);
                                        setLogs(prev => [...prev, `[ATVM] Provisioned new container: ${os.name}`]);
                                        setInstallingStatus(null);
                                     }, 2000);
                                   }}
                                   className="w-full py-1.5 rounded bg-indigo-500 hover:bg-indigo-600 text-white shadow-lg shadow-indigo-500/20 text-xs font-bold uppercase tracking-widest transition-colors flex items-center justify-center gap-1.5"
                                 >
                                    <Download className="w-3 h-3"/> Install
                                 </button>
                               )}
                            </div>
                          </div>
                        );
                     })}
                   </div>
                   
                   <div className="mt-8 pt-6 border-t border-white/10">
                     <div className="flex items-center justify-between">
                       <div className="flex flex-col">
                         <span className="text-sm font-bold text-white">Advanced Configuration</span>
                         <span className="text-xs text-slate-500">Allocate hardware parameters for ATVM containers.</span>
                       </div>
                       <div className="flex gap-4">
                         <button 
                           onClick={() => setLogs(prev => [...prev, `[ATVM] Rebooting ${OS_CATALOG[osEnv]?.name || osEnv} sub-kernel instance...`, '[ATVM] OS state reset completed.'])}
                           className="px-4 py-2 border border-rose-500/30 bg-rose-500/10 hover:bg-rose-500/20 rounded-lg text-xs font-bold text-rose-400 transition-colors uppercase tracking-widest"
                         >
                           Reboot VM
                         </button>
                         <button 
                           onClick={() => setLogs(prev => [...prev, `[ATVM] Mounting ${OS_CATALOG[osEnv]?.name || osEnv} sys-modules...`, '[ATVM] Environment synchronized.'])}
                           className="px-4 py-2 bg-atc-cyan hover:bg-atc-cyan/90 rounded-lg text-xs font-bold text-black transition-colors uppercase tracking-widest shadow-lg shadow-atc-cyan/20"
                         >
                           Apply Settings
                         </button>
                       </div>
                     </div>
                   </div>
                </div>
              </div>
            </div>
          )}
        </div>
        
        {/* Terminal/Logs */}
        <div className="w-72 lg:w-80 bg-[#050608] flex flex-col border-l border-atc-border shrink-0">
          <div className="px-4 py-3 border-b border-atc-border flex items-center gap-2">
            <Terminal className="w-4 h-4 text-slate-500" />
            <span className="text-[10px] uppercase tracking-widest text-slate-500 font-mono font-bold">Compiler Output</span>
          </div>
          <div className="flex-1 p-4 overflow-auto font-mono text-xs text-slate-400 flex flex-col gap-2">
            {logs.length === 0 && <span className="opacity-40">Waiting for actions...</span>}
            {logs.map((log, i) => (
              <div key={i} className={`leading-relaxed ${log.includes('error') ? 'text-rose-400' : log.includes('verified') || log.includes('successfully') ? 'text-emerald-400/90' : 'text-slate-300'}`}>
                <span className="text-slate-600 mr-2 selection:bg-transparent">{'>'}</span>{log}
              </div>
            ))}
          </div>
        </div>
      </div>
      <AnimatePresence>
        {showSaveToast && (
          <motion.div
            initial={{ opacity: 0, y: 50, scale: 0.9 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: 20, scale: 0.9 }}
            className="absolute bottom-6 right-6 flex items-center gap-2 px-4 py-2 bg-[#090b14]/90 backdrop-blur-md border border-emerald-500/30 rounded-full shadow-[0_0_15px_rgba(16,185,129,0.2)] text-emerald-400 z-50 font-mono text-xs"
          >
            <CheckCircle className="w-4 h-4" />
            <span>Sandbox code autosaved at {lastSaveTime}</span>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
