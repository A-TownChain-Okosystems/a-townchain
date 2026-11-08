import React, { useState } from 'react';
import { Cpu, MemoryStick, Layers, Box, Terminal, Play, Lock, Zap, FileCode, Shield } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { TooltipIcon } from './TooltipIcon';

type KernelModule = {
  id: string;
  name: string;
  type: string;
  status: 'active' | 'idle' | 'sandboxed';
  memory: string;
  description: string;
};

const MOCK_MODULES: KernelModule[] = [
  { id: 'km-1', name: 'ZkSnarkVerifier', type: 'Security', status: 'active', memory: '1.2 GB', description: 'Zero-Knowledge Proof verification engine.' },
  { id: 'km-2', name: 'P2P_Gossip', type: 'Networking', status: 'active', memory: '450 MB', description: 'Mesh network routing and state propagation.' },
  { id: 'km-3', name: 'WASM_Sandbox', type: 'Execution', status: 'idle', memory: '0 MB', description: 'WebAssembly execution layer for external plugins.' },
  { id: 'km-4', name: 'BFT_Consensus', type: 'Consensus', status: 'active', memory: '2.1 GB', description: 'Byzantine Fault Tolerance state agreement.' },
  { id: 'km-5', name: 'AI_Agent_Governor', type: 'Intelligence', status: 'sandboxed', memory: '5.4 GB', description: 'LLM-driven system orchestration & optimization.' },
];

export function AtcCoreKernelView() {
  const [activeTab, setActiveTab] = useState<'architecture' | 'modules'>('architecture');

  return (
    <div className="flex flex-col gap-8 w-full max-w-6xl mx-auto py-8">
      <div className="flex items-center gap-4 mb-4">
        <div className="w-14 h-14 rounded-2xl bg-atc-cyan/10 border border-atc-cyan/30 flex items-center justify-center text-atc-cyan shadow-[0_0_20px_rgba(6,182,212,0.2)]">
          <Cpu className="w-7 h-7" />
        </div>
        <div>
          <h2 className="text-3xl font-bold tracking-tight text-white mb-2">
            ATC-Core Kernel <TooltipIcon content="Der zentrale OS Kernel orchestriert alle Blockchain-Validierungen, Netzwerkprotokolle und AI-Layer in einer modularen Sandbox-Architektur." />
          </h2>
          <p className="text-sm text-slate-400 max-w-3xl leading-relaxed">
            Das A-TownChain Core Betriebssystem basiert auf einer Microkernel-Architektur. Nur essenzielle IPC (Inter-Process Communication) und Memory-Management Routinen laufen im Ring 0. Alle anderen Systeme, inklusive Blockchain-Konsens und KI, operieren im Userspace in strikt isolierten Containern.
          </p>
        </div>
      </div>

      <div className="flex items-center gap-4 border-b border-white/10 pb-4">
        <button 
          onClick={() => setActiveTab('architecture')}
          className={`px-4 py-2 font-bold text-sm rounded-lg transition-colors ${activeTab === 'architecture' ? 'bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/40' : 'text-slate-400 hover:text-white'}`}
        >
          Kernel Architektur
        </button>
        <button 
          onClick={() => setActiveTab('modules')}
          className={`px-4 py-2 font-bold text-sm rounded-lg transition-colors ${activeTab === 'modules' ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/40' : 'text-slate-400 hover:text-white'}`}
        >
          Dynamische Module
        </button>
      </div>

      {activeTab === 'architecture' && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div className="bg-[#090b14]/80 backdrop-blur border border-white/10 rounded-2xl p-6 relative overflow-hidden group">
            <div className="absolute top-0 right-0 w-48 h-48 bg-atc-purple/10 rounded-full blur-[80px] pointer-events-none" />
            <h3 className="text-xl font-bold text-white mb-6 flex items-center gap-2">
              <Layers className="w-5 h-5 text-atc-purple" />
              Microkernel Design
            </h3>
            <div className="space-y-4 relative z-10">
               <div className="p-4 border border-rose-500/20 bg-rose-500/5 rounded-xl">
                 <h4 className="text-rose-400 font-bold mb-1 flex items-center gap-2"><Lock className="w-4 h-4" /> Ring 0 (Core)</h4>
                 <p className="text-xs text-slate-400">IPC Routing, Hardware Abstraction, Memory Allocation. Garantiert Crash-Resistenz für den Node.</p>
               </div>
               <div className="p-4 border border-amber-500/20 bg-amber-500/5 rounded-xl">
                 <h4 className="text-amber-400 font-bold mb-1 flex items-center gap-2"><Shield className="w-4 h-4" /> Ring 1 (Driver / Networking)</h4>
                 <p className="text-xs text-slate-400">Network Interfaces, P2P Layer, Storage I/O. Isoliert von Konsensfehlern.</p>
               </div>
               <div className="p-4 border border-emerald-500/20 bg-emerald-500/5 rounded-xl">
                 <h4 className="text-emerald-400 font-bold mb-1 flex items-center gap-2"><Terminal className="w-4 h-4" /> Ring 3 (Userspace - Blockchain/AI)</h4>
                 <p className="text-xs text-slate-400">Smart Contracts (ATVM), AI Agents, Blockchain State Validation.</p>
               </div>
            </div>
          </div>

          <div className="flex flex-col gap-6">
            <div className="bg-gradient-to-br from-indigo-900/40 to-[#090b14] border border-indigo-500/20 rounded-2xl p-6">
               <h3 className="text-lg font-bold text-indigo-300 mb-2">KI-Integration im Kernel</h3>
               <p className="text-sm text-slate-400 font-light leading-relaxed mb-4">
                 Die KI operiert nicht als externes Tool, sondern als tiefgreifender Systemprozess. Sie übernimmt dynamisches Load-Balancing, identifiziert anomale Blockchain-Transaktionen (Fraud Detection) in Echtzeit und optimiert das P2P-Routing durch prädiktive Modelle.
               </p>
               <div className="flex items-center gap-3">
                 <span className="px-3 py-1 bg-indigo-500/20 border border-indigo-500/30 rounded text-xs font-mono text-indigo-300">Fraud Detection</span>
                 <span className="px-3 py-1 bg-indigo-500/20 border border-indigo-500/30 rounded text-xs font-mono text-indigo-300">Auto-Scaling</span>
               </div>
            </div>

            <div className="bg-cyan-900/20 border border-cyan-500/20 rounded-2xl p-6">
               <h3 className="text-lg font-bold text-cyan-400 mb-2">Deterministische State-Engine</h3>
               <p className="text-sm text-slate-400 font-light leading-relaxed">
                 Um Konsens im P2P-Netzwerk zu gewährleisten, führt der Kernel alle Smart Contracts und Systemupdates in einer zu 100% deterministischen Sandbox (ATVM) aus. Fließkomma-Ungenauigkeiten und Systemzeit-Varianzen abstrahiert der OS-Kernel weg.
               </p>
            </div>
          </div>
        </div>
      )}

      {activeTab === 'modules' && (
        <div className="bg-[#090b14] border border-white/10 rounded-2xl p-6">
           <div className="flex items-center justify-between mb-6">
             <h3 className="text-lg font-bold text-white flex items-center gap-2">
               <Box className="w-5 h-5 text-slate-400" />
               Geladene Kernel-Module
             </h3>
             <div className="font-mono text-xs text-slate-500">Live Process Tree</div>
           </div>
           
           <div className="space-y-3">
             {MOCK_MODULES.map(mod => (
               <div key={mod.id} className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 p-4 rounded-xl border border-white/5 bg-black/40 hover:bg-black/60 transition-colors">
                  <div className="flex items-center gap-4">
                     <div className={`w-2 h-2 rounded-full ${mod.status === 'active' ? 'bg-emerald-400 animate-pulse' : mod.status === 'sandboxed' ? 'bg-amber-400' : 'bg-slate-600'}`} />
                     <div>
                       <h4 className="font-bold text-slate-200">{mod.name}</h4>
                       <p className="text-[10px] text-slate-500 font-mono mt-0.5">{mod.description}</p>
                     </div>
                  </div>
                  
                  <div className="flex items-center gap-6">
                    <div className="flex flex-col text-right">
                      <span className="text-[10px] uppercase font-bold tracking-widest text-slate-500 font-mono">Category</span>
                      <span className="text-sm text-slate-300">{mod.type}</span>
                    </div>
                    <div className="flex flex-col text-right">
                      <span className="text-[10px] uppercase font-bold tracking-widest text-slate-500 font-mono">Memory</span>
                      <span className="text-sm font-mono text-atc-cyan">{mod.memory}</span>
                    </div>
                    <button className="px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded text-xs font-bold text-slate-300 transition-colors">
                      {mod.status === 'idle' ? 'Load' : 'Inspect'}
                    </button>
                  </div>
               </div>
             ))}
           </div>
        </div>
      )}
    </div>
  );
}
