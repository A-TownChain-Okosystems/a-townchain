import React, { useState } from 'react';
import { Cpu, BrainCircuit, Activity, Layers, ShieldAlert, Network } from 'lucide-react';

export function AiKernelView() {
  const [activeTab, setActiveTab] = useState<'inference' | 'orchestration' | 'heuristics'>('inference');

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="flex gap-4 p-4 border-b border-white/5 bg-black/20 overflow-x-auto custom-scrollbar">
        <button
          onClick={() => setActiveTab('inference')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors shrink-0 ${
            activeTab === 'inference'
              ? 'bg-purple-500/20 text-purple-300 border border-purple-500/30 shadow-[0_0_15px_rgba(168,85,247,0.2)]'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <Cpu className="w-4 h-4" />
          Local Inference Layer
        </button>
        <button
          onClick={() => setActiveTab('orchestration')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors shrink-0 ${
            activeTab === 'orchestration'
              ? 'bg-rose-500/20 text-rose-300 border border-rose-500/30 shadow-[0_0_15px_rgba(244,63,94,0.2)]'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <BrainCircuit className="w-4 h-4" />
          Neural Orchestration
        </button>
        <button
          onClick={() => setActiveTab('heuristics')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors shrink-0 ${
            activeTab === 'heuristics'
              ? 'bg-amber-500/20 text-amber-300 border border-amber-500/30 shadow-[0_0_15px_rgba(245,158,11,0.2)]'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <ShieldAlert className="w-4 h-4" />
          Mempool Heuristics
        </button>
      </div>

      <div className="flex-1 overflow-auto p-6">
        {activeTab === 'inference' && (
          <div className="flex flex-col gap-6">
            <div className="p-6 border border-purple-500/20 rounded-xl bg-black/40 flex flex-col gap-4">
              <h3 className="text-lg font-bold text-white flex items-center gap-2">
                <Layers className="w-5 h-5 text-purple-400" />
                Tensor-Parallel Execution
              </h3>
              <p className="text-sm text-slate-300">
                Deterministic LLM Execution running directly in the ATC-VM with guaranteed gas constraints.
              </p>
              
              <div className="grid grid-cols-2 gap-4 mt-2">
                <div className="flex flex-col gap-1 p-3 bg-[#090b14] border border-white/5 rounded">
                  <span className="text-xs text-slate-400">ONNX/GGUF Checkpoint</span>
                  <span className="text-emerald-400 font-mono text-sm">Valid & Loaded</span>
                </div>
                <div className="flex flex-col gap-1 p-3 bg-[#090b14] border border-white/5 rounded">
                  <span className="text-xs text-slate-400">LoRA Injection API</span>
                  <span className="text-emerald-400 font-mono text-sm">Ready (0 Active)</span>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'orchestration' && (
          <div className="flex flex-col gap-6">
            <div className="p-6 border border-rose-500/20 rounded-xl bg-black/40 flex flex-col gap-4">
              <h3 className="text-lg font-bold text-white flex items-center gap-2">
                <Activity className="w-5 h-5 text-rose-400" />
                Neural Orchestration
              </h3>
              <p className="text-sm text-slate-300">
                Self-Attention based task prioritization mapping kernel threads directly to inference sub-requests.
              </p>

              <div className="flex items-center justify-between p-4 bg-[#090b14] border border-white/5 rounded">
                <div className="flex flex-col gap-1">
                  <span className="text-sm font-medium text-white">Dynamic Parameter Quantization</span>
                  <span className="text-xs text-slate-400">Switching dynamically between INT4 / INT8 based on hardware topology.</span>
                </div>
                <span className="px-2 py-1 text-[10px] font-bold uppercase tracking-widest rounded bg-rose-500/20 text-rose-400">
                  Optimized
                </span>
              </div>
            </div>
          </div>
        )}
        {activeTab === 'heuristics' && (
          <div className="flex flex-col gap-6">
            <div className="p-6 border border-amber-500/20 rounded-xl bg-black/40 flex flex-col gap-4">
              <h3 className="text-lg font-bold text-white flex items-center gap-2">
                <ShieldAlert className="w-5 h-5 text-amber-400" />
                Mempool Threat Detection
              </h3>
              <p className="text-sm text-slate-300">
                Pre-validation zero-knowledge scans on floating transactions to prevent Flashbots and MEV exploitation directly within the ATVM sandbox.
              </p>

              <div className="grid grid-cols-2 lg:grid-cols-3 gap-4 mt-2">
                <div className="flex flex-col gap-2 p-4 bg-[#090b14] border border-white/5 rounded-xl">
                  <span className="text-xs text-slate-400 font-bold uppercase tracking-widest flex items-center gap-2"><Network className="w-3.5 h-3.5"/> Subnet Load</span>
                  <span className="text-amber-400 font-mono text-xl">82%</span>
                  <p className="text-[10px] text-slate-500">Heuristics active on 4/5 gossip branches</p>
                </div>
                <div className="flex flex-col gap-2 p-4 bg-[#090b14] border border-white/5 rounded-xl">
                  <span className="text-xs text-slate-400 font-bold uppercase tracking-widest flex items-center gap-2"><Activity className="w-3.5 h-3.5"/> Blocked Attacks</span>
                  <span className="text-emerald-400 font-mono text-xl">14.2k</span>
                  <p className="text-[10px] text-slate-500">Front-running attempts neutralized</p>
                </div>
                <div className="flex flex-col gap-2 p-4 bg-[#090b14] border border-emerald-500/20 rounded-xl">
                  <span className="text-xs text-emerald-400 font-bold uppercase tracking-widest">Model Readiness</span>
                  <span className="text-emerald-400 font-mono text-xl">Active</span>
                  <p className="text-[10px] text-emerald-500/70">Continuous learning ON</p>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
