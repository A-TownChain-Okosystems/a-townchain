import React from 'react';
import { Microscope, Code2, Layers, Cpu, Wrench, Bug } from 'lucide-react';

export function CodeAnalyzerView() {
  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-purple-500/10 border border-purple-500/30 flex items-center justify-center text-purple-400 shadow-[0_0_15px_rgba(168,85,247,0.2)]">
          <Microscope className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Codebase Analyzer & Verbesserungen
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Statische Code-Analyse, Klassen-Hierarchien, Funktionstests und automatisierte Verbesserungsvorschläge.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="col-span-1 lg:col-span-2 p-6 rounded-2xl bg-[#090b14]/80 border border-white/5">
          <h3 className="text-lg font-semibold text-white mb-6 flex items-center gap-2">
            <Layers className="w-5 h-5 text-purple-400" /> System Architektur Analyse
          </h3>
          <div className="grid grid-cols-2 gap-4 mb-6">
            <div className="p-4 bg-black/40 border border-white/5 rounded-xl text-center">
              <span className="block text-2xl font-bold text-white font-mono mb-1">284</span>
              <span className="text-xs text-slate-500 uppercase tracking-widest">Classes</span>
            </div>
            <div className="p-4 bg-black/40 border border-white/5 rounded-xl text-center">
              <span className="block text-2xl font-bold text-white font-mono mb-1">1,902</span>
              <span className="text-xs text-slate-500 uppercase tracking-widest">Functions</span>
            </div>
            <div className="p-4 bg-black/40 border border-white/5 rounded-xl text-center">
              <span className="block text-2xl font-bold text-emerald-400 font-mono mb-1">94%</span>
              <span className="text-xs text-slate-500 uppercase tracking-widest">Test Cov.</span>
            </div>
            <div className="p-4 bg-black/40 border border-white/5 rounded-xl text-center">
              <span className="block text-2xl font-bold text-amber-400 font-mono mb-1">12</span>
              <span className="text-xs text-slate-500 uppercase tracking-widest">Tech Debt</span>
            </div>
          </div>
          <div className="p-4 rounded-xl bg-purple-500/5 border border-purple-500/20 text-sm list-disc font-mono text-purple-200">
             <div className="font-bold mb-2 flex items-center gap-2 text-purple-400"><Code2 className="w-4 h-4" /> Structure Output:</div>
             <pre className="text-xs opacity-70 whitespace-pre-wrap">
{`class ATCNode extends Web3Provider {
  + verifyState(hash: string): boolean
  + broadcast(tx: Transaction): void
  - privateKey: SecureEnclave<Key>
}

function processBatch(txs: Transaction[]): Result { ... }`}
             </pre>
          </div>
        </div>

        <div className="flex flex-col gap-6">
          <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5">
            <h3 className="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2">
              <Wrench className="w-4 h-4 text-emerald-400" /> AI Suggestions
            </h3>
            <ul className="space-y-3 text-xs text-slate-300">
              <li className="flex items-start gap-2 bg-black/40 p-3 rounded border border-white/5 hover:border-emerald-500/30 transition-colors">
                <span className="w-1.5 h-1.5 bg-emerald-500 rounded-full mt-1 shrink-0" />
                Refactor <span className="font-mono text-emerald-400">verifyState</span> to use WebWorkers for better CPU utilization.
              </li>
              <li className="flex items-start gap-2 bg-black/40 p-3 rounded border border-white/5 hover:border-emerald-500/30 transition-colors">
                <span className="w-1.5 h-1.5 bg-emerald-500 rounded-full mt-1 shrink-0" />
                Deduplicate database querying logic in <span className="font-mono text-emerald-400">TxOrchestrator.ts</span>.
              </li>
            </ul>
          </div>
          
          <div className="p-6 rounded-2xl bg-amber-500/5 border border-amber-500/20">
            <h3 className="text-sm font-semibold text-amber-500 mb-2 flex items-center gap-2">
              <Bug className="w-4 h-4" /> Security Hotspots
            </h3>
            <p className="text-xs text-amber-200/70 leading-relaxed mb-3">
              Static analysis found 2 potential reentrancy vectors in smart contract templates.
            </p>
            <button className="w-full py-2 bg-amber-500/10 text-amber-400 text-xs font-bold rounded border border-amber-500/30 hover:bg-amber-500/20">
              Review Hotspots
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
