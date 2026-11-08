import React from 'react';
import { Layers, Zap, Command, Settings2, FileCode2, Search } from 'lucide-react';

export function AtcLangPresetsView() {
  const presets = [
    { id: 'p1', title: "Smart Contract Genesis", desc: "Basic template for A-Town smart contracts with state transitions.", type: "Boilerplate" },
    { id: 'p2', title: "ZK Asset Transfer", desc: "Preset for confidential transactions using zk-SNARK constraints.", type: "Security" },
    { id: 'p3', title: "Decentralized Oracle Hook", desc: "ATC-lang bindings to fetch external ATVM oracle data securely.", type: "Integration" },
    { id: 'p4', title: "DAO Voter Module", desc: "Pre-configured logic for quadratic voting within the DAO specifications.", type: "Governance" },
    { id: 'p5', title: "DeFi Liquidity Pool", desc: "Automated market maker pool implementations for basic token pairs.", type: "Finance" },
    { id: 'p6', title: "Identity Registry", desc: "Mapping of decentralized IDs to roles and metadata within ATVM.", type: "Identity" }
  ];

  return (
    <div className="flex flex-col gap-8 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
          <Layers className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            ATC-Lang Presets
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Pre-compiled libraries, syntax bundles, and smart contract templates for rapid ATVM development.
          </p>
        </div>
      </div>
      
      <div className="flex gap-4 mb-4">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
          <input type="text" placeholder="Search presets..." className="w-full bg-[#090b14] border border-white/10 rounded-lg py-2.5 pl-10 pr-4 text-sm text-white focus:outline-none focus:border-atc-cyan transition-colors" />
        </div>
        <button className="px-4 py-2 bg-white/5 border border-white/10 rounded-lg text-slate-300 text-sm hover:bg-white/10 transition-colors flex items-center gap-2">
          <Settings2 className="w-4 h-4" /> Filter
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {presets.map(preset => (
          <div key={preset.id} className="p-6 rounded-2xl bg-[#090b14]/80 backdrop-blur-md border border-white/5 hover:border-indigo-500/30 transition-all group flex flex-col h-full relative overflow-hidden">
             <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/5 rounded-full blur-[40px] pointer-events-none group-hover:bg-indigo-500/10 transition-colors" />
             <div className="flex items-start justify-between mb-4 relative z-10">
               <div className="p-2.5 rounded-xl bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
                 <FileCode2 className="w-5 h-5" />
               </div>
               <span className="text-[10px] font-mono tracking-widest uppercase px-2 py-0.5 rounded border border-white/10 text-slate-400 bg-white/5">
                 {preset.type}
               </span>
             </div>
             <h3 className="text-lg font-bold text-white mb-2 relative z-10">{preset.title}</h3>
             <p className="text-sm text-slate-400 font-light leading-relaxed mb-6 flex-1 relative z-10">
               {preset.desc}
             </p>
             <button className="w-full py-2.5 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 border border-indigo-500/20 rounded-lg text-sm font-medium transition-colors flex items-center justify-center gap-2 relative z-10">
               <Command className="w-4 h-4" /> Use Preset
             </button>
          </div>
        ))}
      </div>
    </div>
  );
}
