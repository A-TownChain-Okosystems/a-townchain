import React from 'react';
import { ArrowRightLeft, Activity, Layers, Repeat, ShieldCheck, Zap, GitCommit } from 'lucide-react';

export function TxOrchestratorView() {
  const mempool = [
    { hash: "0x4a...f92", type: "Contract Execution", fee: "0.002 ATC", status: "Completed", age: "2s" },
    { hash: "0x8b...321", type: "Token Transfer", fee: "0.0005 ATC", status: "Validating", age: "4s" },
    { hash: "0xc1...110", type: "State Pinning", fee: "0.015 ATC", status: "Completed", age: "1s" },
    { hash: "0xf2...9aa", type: "ZK Proof Submission", fee: "0.008 ATC", status: "Queued", age: "8s" }
  ];

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-emerald-500/10 border border-emerald-500/30 flex items-center justify-center text-emerald-400 shadow-[0_0_15px_rgba(16,185,129,0.2)]">
          <ArrowRightLeft className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Transactions Orchestrator
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            ATVM Transaktionssteuerung, Mempool-Management und Validierungslogik für den globalen State.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="p-6 rounded-2xl bg-gradient-to-br from-[#090b14] to-[#111827] border border-white/5 relative overflow-hidden">
           <Zap className="absolute top-4 right-4 w-24 h-24 text-emerald-500/5 rotate-12" />
           <div className="relative z-10">
             <h3 className="text-sm font-medium text-slate-400 mb-1">Current TPS</h3>
             <div className="text-4xl font-bold text-white font-mono mb-2">1,245.8</div>
             <div className="flex items-center gap-2 text-xs text-emerald-400">
               <Activity className="w-3 h-3" /> +12% from last hour
             </div>
           </div>
        </div>

        <div className="p-6 rounded-2xl bg-gradient-to-br from-[#090b14] to-[#111827] border border-white/5 relative overflow-hidden">
           <Layers className="absolute top-4 right-4 w-24 h-24 text-blue-500/5 rotate-12" />
           <div className="relative z-10">
             <h3 className="text-sm font-medium text-slate-400 mb-1">Block Time</h3>
             <div className="text-4xl font-bold text-white font-mono mb-2">2.4<span className="text-2xl text-slate-500">s</span></div>
             <div className="flex items-center gap-2 text-xs text-blue-400">
               <ShieldCheck className="w-3 h-3" /> Finality confirmed
             </div>
           </div>
        </div>

        <div className="p-6 rounded-2xl bg-gradient-to-br from-[#090b14] to-[#111827] border border-white/5 relative overflow-hidden">
           <Repeat className="absolute top-4 right-4 w-24 h-24 text-purple-500/5 -rotate-12" />
           <div className="relative z-10">
             <h3 className="text-sm font-medium text-slate-400 mb-1">Mempool Size</h3>
             <div className="text-4xl font-bold text-white font-mono mb-2">8,421</div>
             <div className="flex items-center gap-2 text-xs text-purple-400">
               <GitCommit className="w-3 h-3" /> Processing...
             </div>
           </div>
        </div>
      </div>

      <div className="p-6 rounded-2xl bg-[#090b14]/80 backdrop-blur-md border border-white/5">
        <h3 className="text-lg font-semibold text-white mb-6 flex items-center gap-2">
           Live Transaction Mempool
        </h3>
        <div className="space-y-3">
          {mempool.map((tx, idx) => (
            <div key={idx} className="flex flex-col sm:flex-row items-start sm:items-center justify-between p-4 rounded-xl bg-black/40 border border-white/5 hover:border-emerald-500/30 transition-colors gap-4">
               <div className="flex items-center gap-4">
                 <div className="p-2 rounded-lg bg-white/5 border border-white/10">
                   <GitCommit className="w-4 h-4 text-slate-400" />
                 </div>
                 <div>
                   <div className="text-sm font-medium text-slate-200">{tx.type}</div>
                   <div className="text-xs font-mono text-emerald-400/70">{tx.hash}</div>
                 </div>
               </div>
               
               <div className="flex items-center gap-6 text-sm">
                 <div className="text-right">
                   <div className="text-slate-400 text-xs">Fee</div>
                   <div className="font-mono text-slate-200">{tx.fee}</div>
                 </div>
                 <div className="text-right">
                   <div className="text-slate-400 text-xs">Age</div>
                   <div className="font-mono text-slate-200">{tx.age}</div>
                 </div>
                 <div className="w-24 text-right">
                   <span className={`px-2 py-1 rounded text-[10px] uppercase tracking-widest font-bold ${
                     tx.status === 'Pending' ? 'bg-amber-500/10 text-amber-400' : 
                     tx.status === 'Validating' ? 'bg-blue-500/10 text-blue-400' : 
                     'bg-slate-500/10 text-slate-400'
                   }`}>
                     {tx.status}
                   </span>
                 </div>
               </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
