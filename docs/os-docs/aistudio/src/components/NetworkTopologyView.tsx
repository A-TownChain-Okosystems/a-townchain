import React from 'react';
import { Network, Map } from 'lucide-react';
import { StrategicArchitectureMap } from './StrategicArchitectureMap';

export function NetworkTopologyView() {
  return (
    <div className="flex flex-col gap-6 mt-8 pb-12 w-full max-w-7xl mx-auto">
      <div className="flex items-center gap-4 mb-4">
        <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
          <Map className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">Global System Map</h2>
          <p className="text-sm font-light text-slate-400">Interactive D3 force-directed visualization of the Strategic Architecture Layers and their live core components</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
        <div className="bg-[#090b14]/60 border border-white/5 rounded-2xl p-4 flex items-center gap-3">
          <div className="w-3 h-3 rounded-full bg-[#10b981] shadow-[0_0_8px_rgba(16,185,129,0.8)]" />
          <span className="text-sm font-mono text-slate-300">A-TownChain System Core</span>
        </div>
        <div className="bg-[#090b14]/60 border border-white/5 rounded-2xl p-4 flex items-center gap-3">
          <div className="w-3 h-3 rounded-full bg-[#06b6d4] shadow-[0_0_8px_rgba(6,182,212,0.8)]" />
          <span className="text-sm font-mono text-slate-300">Strategic Layers</span>
        </div>
        <div className="bg-[#090b14]/60 border border-white/5 rounded-2xl p-4 flex items-center gap-3">
          <div className="w-3 h-3 rounded-full bg-[#a259ff] shadow-[0_0_8px_rgba(162,89,255,0.8)] animate-pulse" />
          <span className="text-sm font-mono text-slate-300">Active Core Components</span>
        </div>
      </div>

      <div className="bg-[#090b14]/80 backdrop-blur-md border border-white/5 rounded-2xl overflow-hidden h-[700px] shadow-2xl relative">
        <StrategicArchitectureMap />
      </div>
    </div>
  );
}
