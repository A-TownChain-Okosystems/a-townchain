import React from 'react';
import { ShieldCheck, EyeOff, Key, Database, Globe } from 'lucide-react';

export function ComplianceEngineView() {
  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200 overflow-auto">
      <div className="flex items-center gap-3 p-6 border-b border-white/5 bg-black/20">
        <div className="p-2 rounded-lg bg-indigo-500/20 border border-indigo-500/30">
          <ShieldCheck className="w-6 h-6 text-indigo-400" />
        </div>
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Compliance & Privacy Engine</h2>
          <p className="text-sm text-slate-400">DSGVO/MiCA Alignment via Zero-Knowledge Mechanics</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 p-6">
        <div className="p-6 rounded-xl border border-white/10 bg-black/40 flex flex-col gap-4">
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            <EyeOff className="w-5 h-5 text-indigo-400" />
            Selective Disclosure & VCs
          </h3>
          <p className="text-sm text-slate-300">
            Identities are kept sovereign. Users prove cryptographic attributes without revealing plaintext documentation.
          </p>
          <div className="flex flex-col gap-2 font-mono text-xs">
            <div className="flex items-center justify-between p-2 rounded bg-[#090b14] border border-white/5">
              <span>KYC: Verification Hash</span>
              <span className="text-emerald-400">Valid</span>
            </div>
            <div className="flex items-center justify-between p-2 rounded bg-[#090b14] border border-white/5">
              <span>Reg: MiCA AML Compliance</span>
              <span className="text-emerald-400">Passed ZK-Proof</span>
            </div>
          </div>
        </div>

        <div className="p-6 rounded-xl border border-white/10 bg-black/40 flex flex-col gap-4">
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            <Database className="w-5 h-5 text-indigo-400" />
            Off-Chain Enclaves
          </h3>
          <p className="text-sm text-slate-300">
            Right to be forgotten enforced. PII never touches the blockchain. Data fingerprints (Hashes) map to local protected data vaults.
          </p>
           <div className="flex flex-col gap-2 font-mono text-xs">
            <div className="p-2 rounded bg-[#090b14] border border-white/5 text-slate-400">
              SHA256: e3b0c44298fc1c149afbf4c...
            </div>
            <div className="p-2 rounded bg-[#090b14] border border-white/5 text-slate-400">
              Vault Status: Intact, Erasure Allowed
            </div>
          </div>
        </div>

        <div className="p-6 rounded-xl border border-white/10 bg-black/40 flex flex-col gap-4">
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            <Key className="w-5 h-5 text-indigo-400" />
            Audit View Keys
          </h3>
          <p className="text-sm text-slate-300">
            Regulators and auditors are provided specific cryptographic read-keys to inspect compliance flows safely.
          </p>
          <div className="flex items-center gap-2 p-3 bg-indigo-500/10 border border-indigo-500/20 rounded text-indigo-300 text-sm">
            <ShieldCheck className="w-4 h-4" /> Authority View Subsystem Active
          </div>
        </div>

        <div className="p-6 rounded-xl border border-white/10 bg-black/40 flex flex-col gap-4">
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            <Globe className="w-5 h-5 text-indigo-400" />
            Stealth Addresses
          </h3>
          <p className="text-sm text-slate-300">
            Disposable single-use cryptographic addresses isolate transaction graphs preventing network intelligence gathering by hostile third parties.
          </p>
          <div className="flex items-center gap-2 p-3 bg-emerald-500/10 border border-emerald-500/20 rounded text-emerald-400 text-sm font-mono">
            Generated: 0xStealth...7F2A
          </div>
        </div>
      </div>
    </div>
  );
}
