import React from 'react';
import { BrainCircuit, Share2, Database, Workflow, Cuboid, Factory } from 'lucide-react';

export function SemanticGraphView() {
  const semanticData = [
    { entity: "Smart Contract Logic", relation: "extends", target: "ATVM Core Modules", confidence: "99%" },
    { entity: "Player Identity", relation: "mapped_to", target: "Decentralized Wallet", confidence: "100%" },
    { entity: "DAO Proposal", relation: "requires", target: "Quorum Approval", confidence: "95%" },
    { entity: "Liquidity Pool", relation: "impacts", target: "Token Velocity", confidence: "87%" }
  ];

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-[0_0_15px_rgba(6,182,212,0.2)]">
          <BrainCircuit className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Semantic Graph
          </h2>
          <p className="text-slate-400 font-light max-w-3xl leading-relaxed">
            Wissensgraph und semantische Ontologie-Verwaltung für die ATC Architektur.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="col-span-1 lg:col-span-2 p-6 rounded-2xl bg-[#090b14]/80 border border-white/5 flex flex-col min-h-[400px]">
          <h3 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
            <BrainCircuit className="w-5 h-5 text-cyan-400" />
            Semantic Graph Visualization
          </h3>
          <div className="flex-1 bg-black/40 rounded-xl border border-white/5 flex items-center justify-center relative overflow-hidden">
            <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-cyan-900/20 via-[#090b14] to-[#090b14]"></div>
            
            {/* Mock Graph Elements */}
            <div className="relative z-10 w-full h-full flex items-center justify-center">
               <div className="absolute top-1/4 left-1/4 p-3 bg-white/5 border border-cyan-500/30 rounded-full">
                 <Cuboid className="w-6 h-6 text-cyan-400" />
               </div>
               <div className="absolute bottom-1/4 right-1/4 p-3 bg-white/5 border border-purple-500/30 rounded-full">
                 <Database className="w-6 h-6 text-purple-400" />
               </div>
               <div className="absolute top-1/2 right-1/3 p-3 bg-white/5 border border-emerald-500/30 rounded-full">
                 <Workflow className="w-6 h-6 text-emerald-400" />
               </div>
               <div className="absolute p-4 bg-cyan-500/10 border border-cyan-400/50 rounded-full flex flex-col items-center justify-center text-center">
                 <Factory className="w-8 h-8 text-cyan-300 mb-1" />
                 <span className="text-[10px] font-mono text-cyan-200">CORE</span>
               </div>
               <svg className="absolute inset-0 w-full h-full pointer-events-none opacity-20">
                 <line x1="25%" y1="25%" x2="50%" y2="50%" stroke="cyan" strokeWidth="2" strokeDasharray="4 4" />
                 <line x1="75%" y1="75%" x2="50%" y2="50%" stroke="purple" strokeWidth="2" strokeDasharray="4 4" />
                 <line x1="66%" y1="50%" x2="50%" y2="50%" stroke="emerald" strokeWidth="2" strokeDasharray="4 4" />
               </svg>
            </div>
          </div>
        </div>

        <div className="flex flex-col gap-4">
          <div className="p-5 rounded-2xl bg-[#090b14] border border-white/5">
            <h3 className="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2">
              <Share2 className="w-4 h-4 text-cyan-500" />
              Semantic Queries Live
            </h3>
            <div className="space-y-3">
              {semanticData.map((data, idx) => (
                <div key={idx} className="p-3 rounded-lg bg-black/60 border border-white/5">
                  <div className="flex flex-wrap items-center gap-2 text-xs font-mono">
                    <span className="text-cyan-400">{data.entity}</span>
                    <span className="text-slate-500 italic">[{data.relation}]</span>
                    <span className="text-purple-400">{data.target}</span>
                  </div>
                  <div className="mt-2 text-[10px] text-slate-500 text-right">
                    Confidence: <span className="text-emerald-400">{data.confidence}</span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
