import React from 'react';
import { Database, Network, HardDrive, Shield } from 'lucide-react';

export function DistributedDatalakeView() {
  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200 p-6 overflow-auto">
      <div className="flex items-center gap-3 mb-6">
        <div className="p-2 rounded-lg bg-blue-500/20 border border-blue-500/30">
          <Database className="w-6 h-6 text-blue-400" />
        </div>
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Distributed Datalake & Rollups</h2>
          <p className="text-sm text-slate-400">Phase 4 Infrastructure: Data Availability (DA) and Storage Layout</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="p-6 rounded-xl border border-white/10 bg-black/40 flex flex-col gap-4">
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            <Network className="w-5 h-5 text-blue-400" />
            Data Availability (DA)
          </h3>
          <p className="text-sm text-slate-300">
            Guaranteeing verifiable data availability for rollups across the P2P network.
          </p>
          <ul className="flex flex-col gap-3 mt-2">
            <li className="flex items-start gap-3 p-3 bg-[#090b14] rounded border border-white/5">
              <Shield className="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" />
              <div>
                <span className="font-medium text-white block">Erasure Coding</span>
                <span className="text-xs text-slate-400 font-mono">Reed-Solomon RS(N, K)</span>
              </div>
            </li>
            <li className="flex items-start gap-3 p-3 bg-[#090b14] rounded border border-white/5">
              <Shield className="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" />
              <div>
                <span className="font-medium text-white block">KZG Commitments</span>
                <span className="text-xs text-slate-400 font-mono">Cryptographic commitments for Data Blobs</span>
              </div>
            </li>
          </ul>
        </div>

        <div className="p-6 rounded-xl border border-white/10 bg-black/40 flex flex-col gap-4">
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            <HardDrive className="w-5 h-5 text-blue-400" />
            Storage Layout
          </h3>
          <p className="text-sm text-slate-300">
            Content-addressed distributed storage engineered for post-quantum safety and infinite scalability.
          </p>

          <ul className="flex flex-col gap-3 mt-2">
            <li className="flex items-start gap-3 p-3 bg-[#090b14] rounded border border-white/5">
              <Database className="w-5 h-5 text-indigo-400 shrink-0 mt-0.5" />
              <div>
                <span className="font-medium text-white block">Content-Addressed Vector DB</span>
                <span className="text-xs text-slate-400">For semantic AI retrieval mapped to CID hashes on IPFS.</span>
              </div>
            </li>
            <li className="flex items-start gap-3 p-3 bg-[#090b14] rounded border border-white/5">
              <Database className="w-5 h-5 text-indigo-400 shrink-0 mt-0.5" />
              <div>
                <span className="font-medium text-white block">Post-Quantum Cryptographic Sharding</span>
                <span className="text-xs text-slate-400">Lattice-based encryption applied to distributed datalake shards.</span>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
}
