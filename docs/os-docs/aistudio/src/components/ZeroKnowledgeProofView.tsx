import React, { useState } from 'react';
import { Shield, Fingerprint, Lock, ShieldCheck, CheckCircle2, ChevronRight, Binary } from 'lucide-react';

export function ZeroKnowledgeProofView() {
  const [activeTab, setActiveTab] = useState<'zk-rollups' | 'hardware' | 'formal-methods'>('formal-methods');

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="flex gap-4 p-4 border-b border-white/5 bg-black/20">
        <button
          onClick={() => setActiveTab('formal-methods')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors ${
            activeTab === 'formal-methods'
              ? 'bg-fuchsia-500/20 text-fuchsia-300 border border-fuchsia-500/30'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <Binary className="w-4 h-4" />
          Formal Verification
        </button>
        <button
          onClick={() => setActiveTab('zk-rollups')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors ${
            activeTab === 'zk-rollups'
              ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <Fingerprint className="w-4 h-4" />
          ZK-Rollups
        </button>
        <button
          onClick={() => setActiveTab('hardware')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors ${
            activeTab === 'hardware'
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <Shield className="w-4 h-4" />
          Hardware Enclave
        </button>
      </div>

      <div className="flex-1 overflow-auto p-6">
        {activeTab === 'formal-methods' && (
          <div className="flex flex-col gap-6">
            <div className="p-6 border border-fuchsia-500/20 rounded-xl bg-black/40">
              <h3 className="text-xl font-bold text-white mb-2 flex items-center gap-2">
                <Binary className="w-5 h-5 text-fuchsia-400" />
                Smart Contract Formal Methods
              </h3>
              <p className="text-sm text-slate-300 mb-6">
                Bounded Model Checking & Symbolic Execution for Exploit-Free Smart Contracts.
              </p>
              
              <div className="flex flex-col gap-4">
                <div className="flex items-center justify-between p-4 bg-[#090b14] border border-white/5 rounded-lg">
                  <div className="flex items-center gap-3">
                    <CheckCircle2 className="w-5 h-5 text-emerald-400" />
                    <div>
                      <div className="font-medium text-white">ATVM Deterministic Bytecode Extraction</div>
                      <div className="text-xs text-slate-400">Gas Limit = Σ (OpCode_i × Cost_i)</div>
                    </div>
                  </div>
                  <span className="text-[10px] uppercase font-bold text-emerald-400 bg-emerald-500/20 px-2 py-1 rounded">Verified</span>
                </div>

                <div className="flex items-center justify-between p-4 bg-[#090b14] border border-white/5 rounded-lg">
                  <div className="flex items-center gap-3">
                    <CheckCircle2 className="w-5 h-5 text-emerald-400" />
                    <div>
                      <div className="font-medium text-white">Symbolic Execution Coverage &gt; 99%</div>
                      <div className="text-xs text-slate-400">Coq / Isabelle Automated Theorem Proving</div>
                    </div>
                  </div>
                  <span className="text-[10px] uppercase font-bold text-emerald-400 bg-emerald-500/20 px-2 py-1 rounded">Verified</span>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'zk-rollups' && (
          <div className="flex flex-col gap-6">
            <div className="p-6 border border-emerald-500/20 rounded-xl bg-black/40">
              <h3 className="text-xl font-bold text-white mb-2 flex items-center gap-2">
                <ShieldCheck className="w-5 h-5 text-emerald-400" />
                Groth16 / PLONK Circuits
              </h3>
              <p className="text-sm text-slate-300 mb-6">
                Zero-Knowledge Proof Generator leveraging Polynomial Commitments for Data Availability and privacy.
              </p>

              <div className="grid grid-cols-2 gap-4">
                <div className="p-4 bg-[#090b14] rounded border border-white/5 flex flex-col gap-2">
                  <div className="text-sm font-medium text-slate-400">Avg. Proof Generation</div>
                  <div className="text-2xl font-mono text-white">124ms</div>
                </div>
                <div className="p-4 bg-[#090b14] rounded border border-white/5 flex flex-col gap-2">
                  <div className="text-sm font-medium text-slate-400">Polynomial Commitments</div>
                  <div className="text-2xl font-mono text-white">KZG</div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'hardware' && (
          <div className="flex flex-col gap-6">
            <div className="p-6 border border-indigo-500/20 rounded-xl bg-black/40">
              <h3 className="text-xl font-bold text-white mb-2 flex items-center gap-2">
                <Lock className="w-5 h-5 text-indigo-400" />
                Hardware Enclave (Confidential Computing)
              </h3>
              <p className="text-sm text-slate-300 mb-6">
                Intel SGX / ARM TrustZone Bridge ensuring side-channel resistance for core cryptographic keys and private ATVM execution.
              </p>
              <div className="flex items-center gap-4 text-emerald-400 font-mono text-sm bg-emerald-500/10 border border-emerald-500/20 p-4 rounded-lg">
                <CheckCircle2 className="w-5 h-5" />
                Hardware Root of Trust Verified. Side-channel protection active.
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
