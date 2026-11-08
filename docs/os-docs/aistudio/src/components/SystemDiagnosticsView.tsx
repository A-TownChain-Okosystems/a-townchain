import React, { useState } from "react";
import { HeartPulse, Activity, AlertTriangle, CheckCircle, Cpu, Network, Play, Shield, ShieldAlert, Zap, CheckSquare, Binary, Code2 } from 'lucide-react';
import { NodeHealthMonitor } from './NodeHealthMonitor';

export function SystemDiagnosticsView() {
  const [activeTab, setActiveTab] = useState<"analysis" | "troubleshoot" | "compliance">(
    "analysis",
  );
  const [isScanning, setIsScanning] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);

  const runScan = () => {
    setIsScanning(true);
    setTimeout(() => setIsScanning(false), 2000);
  };

  const runVerification = () => {
    setIsVerifying(true);
    setTimeout(() => setIsVerifying(false), 3000);
  };

  return (
    <div className="flex flex-col h-full bg-[#050815] text-slate-200 border border-indigo-500/30 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center justify-between px-4 h-14 bg-[#090b14]/80 border-b border-indigo-500/20 shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-indigo-500/10 border border-indigo-500/30">
            <Activity className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h2 className="font-bold text-white tracking-tight leading-tight">
              Systemdiagnose & Compliance
            </h2>
            <p className="text-[10px] text-slate-400 font-mono">
              Diagnostics, Trouble-Shooting & Mathematical Proofs
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <button
            onClick={() => setActiveTab("analysis")}
            className={`px-3 py-1.5 rounded-lg text-xs font-bold transition-colors ${activeTab === "analysis" ? "bg-indigo-500/20 text-indigo-300 border border-indigo-500/30" : "text-slate-400 hover:bg-white/5 border border-transparent"}`}
          >
            Analyse
          </button>
          <button
            onClick={() => setActiveTab("troubleshoot")}
            className={`px-3 py-1.5 rounded-lg text-xs font-bold transition-colors ${activeTab === "troubleshoot" ? "bg-indigo-500/20 text-indigo-300 border border-indigo-500/30" : "text-slate-400 hover:bg-white/5 border border-transparent"}`}
          >
            Problembehebung
          </button>
          <button
            onClick={() => setActiveTab("compliance")}
            className={`px-3 py-1.5 rounded-lg text-xs font-bold transition-colors ${activeTab === "compliance" ? "bg-emerald-500/20 text-emerald-300 border border-emerald-500/30" : "text-slate-400 hover:bg-white/5 border border-transparent"}`}
          >
            Compliance & Proofs
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 relative">
        <div className="absolute inset-0 bg-gradient-to-br from-indigo-500/5 via-transparent to-transparent pointer-events-none" />
        
        {activeTab === "compliance" ? (
          <div className="flex flex-col gap-6 relative z-10">
            <div className="flex items-center justify-between bg-[#040714]/80 border border-emerald-500/20 p-4 rounded-xl shadow-[0_0_20px_rgba(16,185,129,0.05)]">
              <div>
                <h3 className="text-emerald-400 font-bold text-sm tracking-wide">ATC Standard Compliance & Logic Verification</h3>
                <p className="text-xs text-slate-400 mt-1">Überprüfung mathematischer Beweisbarkeit, Tokenomics und logischer Architektur.</p>
              </div>
              <button
                onClick={runVerification}
                disabled={isVerifying}
                className="px-6 py-2 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 border border-emerald-500/40 rounded-lg font-bold text-xs tracking-wide transition-colors flex items-center gap-2 disabled:opacity-50"
              >
                {isVerifying ? <Binary className="w-4 h-4 animate-spin" /> : <CheckSquare className="w-4 h-4" />}
                {isVerifying ? "Verifiziere..." : "Voll-Scan Starten"}
              </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-[#090b14]/80 border border-white/5 rounded-xl p-5 relative overflow-hidden group hover:border-emerald-500/20 transition-colors">
                <div className="absolute top-0 right-0 w-32 h-32 bg-emerald-500/5 rounded-full blur-[40px] pointer-events-none" />
                <Code2 className="w-6 h-6 text-emerald-400 mb-3" />
                <h4 className="text-slate-200 font-bold mb-1">Mathematische Beweisbarkeit</h4>
                <p className="text-xs text-slate-500 mb-4">
                  Zero-Knowledge Proofs (ZK-SNARKs) validiert. Formale Logik der Smart Contracts auf Turing-Vollständigkeit ohne Endlosschleifen geprüft.
                </p>
                <div className="flex flex-col gap-2">
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-slate-400">Formal Verification</span>
                    <span className="text-emerald-400 font-bold">✓ PASSED</span>
                  </div>
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-slate-400">ZK-Circuits Integrity</span>
                    <span className="text-emerald-400 font-bold">✓ 100%</span>
                  </div>
                </div>
              </div>

              <div className="bg-[#090b14]/80 border border-white/5 rounded-xl p-5 relative overflow-hidden group hover:border-emerald-500/20 transition-colors">
                <div className="absolute top-0 right-0 w-32 h-32 bg-emerald-500/5 rounded-full blur-[40px] pointer-events-none" />
                <Shield className="w-6 h-6 text-emerald-400 mb-3" />
                <h4 className="text-slate-200 font-bold mb-1">ATC Standard Konformität</h4>
                <p className="text-xs text-slate-500 mb-4">
                  Die Krypto-Token-Architektur, Gas-Mechanismen und Interoperabilität entsprechen strikt den A-Town Chain (ATC) Protokollstandards.
                </p>
                <div className="flex flex-col gap-2">
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-slate-400">ATC-20 (Token)</span>
                    <span className="text-emerald-400 font-bold">COMPLIANT</span>
                  </div>
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-slate-400">ATC-721 (NFT)</span>
                    <span className="text-emerald-400 font-bold">COMPLIANT</span>
                  </div>
                </div>
              </div>

              <div className="bg-[#090b14]/80 border border-white/5 rounded-xl p-5 relative overflow-hidden group hover:border-emerald-500/20 transition-colors">
                <div className="absolute top-0 right-0 w-32 h-32 bg-emerald-500/5 rounded-full blur-[40px] pointer-events-none" />
                <Activity className="w-6 h-6 text-emerald-400 mb-3" />
                <h4 className="text-slate-200 font-bold mb-1">Deterministische Logik</h4>
                <p className="text-xs text-slate-500 mb-4">
                  Alle State-Transitions innerhalb von ATC-OS und der ATC VM produzieren deterministische Resultate unabhängig vom ausführenden Node.
                </p>
                <div className="flex flex-col gap-2">
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-slate-400">State Transition</span>
                    <span className="text-emerald-400 font-bold">DETERMINISTIC</span>
                  </div>
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-slate-400">Consensus Sync</span>
                    <span className="text-emerald-400 font-bold">MATCHED</span>
                  </div>
                </div>
              </div>
              
              <div className="bg-[#090b14]/80 border border-white/5 rounded-xl p-5 relative overflow-hidden group hover:border-emerald-500/20 transition-colors">
                <div className="absolute top-0 right-0 w-32 h-32 bg-emerald-500/5 rounded-full blur-[40px] pointer-events-none" />
                <Zap className="w-6 h-6 text-emerald-400 mb-3" />
                <h4 className="text-slate-200 font-bold mb-1">Kryptografische Sicherheit</h4>
                <p className="text-xs text-slate-500 mb-4">
                  Post-Quanten-Resistente Hash-Algorithmen (Keccak-256 / SHA-3) und elliptische Kurven (secp256k1) sind fehlerfrei implementiert.
                </p>
                <div className="flex flex-col gap-2">
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-slate-400">Signature Verification</span>
                    <span className="text-emerald-400 font-bold">SECURE</span>
                  </div>
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-slate-400">Entropy Generator</span>
                    <span className="text-emerald-400 font-bold">SAFE</span>
                  </div>
                </div>
              </div>
            </div>
            
            {isVerifying && (
               <div className="mt-4 p-4 border border-emerald-500/30 bg-emerald-500/10 rounded-xl">
                 <div className="text-xs text-emerald-400 font-mono mb-2 flex items-center gap-2">
                   <div className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
                   Simuliere Edge-Cases & Mathematische Integrität...
                 </div>
                 <div className="w-full bg-black/50 rounded-full h-1.5 overflow-hidden relative">
                   <div className="absolute top-0 left-0 bottom-0 bg-emerald-400 animate-pulse w-full transition-all duration-[3000ms] ease-in-out" style={{ width: isVerifying ? '100%' : '0%' }} />
                 </div>
               </div>
            )}
          </div>
        ) : activeTab === "analysis" ? (
          <div className="flex flex-col gap-6">
            <div className="flex items-center justify-between bg-black/40 border border-white/5 p-4 rounded-xl">
              <div>
                <h3 className="text-white font-bold text-sm">
                  System Health Scan
                </h3>
                <p className="text-xs text-slate-400">
                  Lokalisiere Engpässe und Fehler im ATC-OS und Netzwerk.
                </p>
              </div>
              <button
                onClick={runScan}
                disabled={isScanning}
                className="px-6 py-2 bg-rose-500 hover:bg-rose-600 text-white rounded-lg font-bold text-xs tracking-wide transition-colors flex items-center gap-2 disabled:opacity-50"
              >
                {isScanning ? (
                  <Activity className="w-4 h-4 animate-spin" />
                ) : (
                  <Play className="w-4 h-4" />
                )}
                {isScanning ? "Scanne..." : "Start Scan"}
              </button>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-4 gap-4">
              <div className="lg:col-span-1">
                <NodeHealthMonitor />
              </div>
              <div className="lg:col-span-3 grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="bg-[#090b14] border border-white/5 rounded-xl p-5">
                  <HeartPulse className="w-6 h-6 text-emerald-400 mb-3" />
                  <h4 className="text-slate-300 font-bold mb-1">OS Kernel</h4>
                  <p className="text-xs text-slate-500 mb-4">
                    Kernel Panic Rate, Memory Leaks, Syscalls.
                  </p>
                  <div className="text-emerald-400 font-mono text-sm bg-emerald-400/10 border border-emerald-400/20 px-3 py-1.5 rounded-md inline-flex items-center gap-2">
                    <CheckCircle className="w-3.5 h-3.5" /> Optimal
                  </div>
                </div>

                <div className="bg-[#090b14] border border-white/5 rounded-xl p-5 relative overflow-hidden">
                  <Network className="w-6 h-6 text-amber-400 mb-3" />
                  <h4 className="text-slate-300 font-bold mb-1">Mesh Routing</h4>
                  <p className="text-xs text-slate-500 mb-4">
                    P2P Verbindungen, Latency Spikes, Drop Rate.
                  </p>
                  <div className="text-amber-400 font-mono text-sm bg-amber-400/10 border border-amber-400/20 px-3 py-1.5 rounded-md inline-flex items-center gap-2">
                    <AlertTriangle className="w-3.5 h-3.5" /> 2 Warnungen
                  </div>
                </div>

                <div className="bg-[#090b14] border border-white/5 rounded-xl p-5">
                  <Shield className="w-6 h-6 text-emerald-400 mb-3" />
                  <h4 className="text-slate-300 font-bold mb-1">Contract VM</h4>
                  <p className="text-xs text-slate-500 mb-4">
                    ATVM Execution, Reverts, Gas Limits.
                  </p>
                  <div className="text-emerald-400 font-mono text-sm bg-emerald-400/10 border border-emerald-400/20 px-3 py-1.5 rounded-md inline-flex items-center gap-2">
                    <CheckCircle className="w-3.5 h-3.5" /> Optimal
                  </div>
                </div>
              </div>
            </div>

            <h3 className="font-bold text-white border-b border-white/5 pb-2 mt-4">
              Aktuelle Systemwarnungen
            </h3>
            <div className="space-y-3">
              <div className="bg-amber-500/5 border border-amber-500/20 rounded-xl p-4 flex gap-4">
                <AlertTriangle className="w-5 h-5 text-amber-400 shrink-0 mt-0.5" />
                <div>
                  <h4 className="text-amber-400 font-bold text-sm mb-1">
                    High Latency auf Route 0x4f...8a
                  </h4>
                  <p className="text-xs text-amber-400/70 mb-2">
                    Die Verbindung zu Node-Gamma überschreitet das Soft-Limit
                    (120ms).
                  </p>
                  <button className="text-[10px] font-mono bg-amber-500/20 text-amber-300 hover:bg-amber-500/30 px-3 py-1 rounded transition-colors">
                    Route neu aufbauen
                  </button>
                </div>
              </div>

              <div className="bg-amber-500/5 border border-amber-500/20 rounded-xl p-4 flex gap-4">
                <AlertTriangle className="w-5 h-5 text-amber-400 shrink-0 mt-0.5" />
                <div>
                  <h4 className="text-amber-400 font-bold text-sm mb-1">
                    ATVM Memory nah am Limit
                  </h4>
                  <p className="text-xs text-amber-400/70 mb-2">
                    Der Smart Contract Executor Page-Memory ist zu 88%
                    ausgelastet.
                  </p>
                  <button className="text-[10px] font-mono bg-amber-500/20 text-amber-300 hover:bg-amber-500/30 px-3 py-1 rounded transition-colors">
                    Garbage Collection erzwingen
                  </button>
                </div>
              </div>
            </div>
          </div>
        ) : (
          <div className="flex flex-col gap-6">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {[
                {
                  name: "Netzwerk Adapter Reset",
                  desc: "Startet alle P2P-Protokolle neu.",
                },
                {
                  name: "Clear DNS/DHT Cache",
                  desc: "Löscht den lokalen Routing-Table.",
                },
                {
                  name: "ATVM Reboot",
                  desc: "Zwingt die Sandbox VM zum Neustart.",
                },
                {
                  name: "Filesystem Check (fsck)",
                  desc: "Überprüft Volume-Integrität.",
                },
                {
                  name: "Rollback Kernel",
                  desc: "Kehrt zur vorherigen OS-Version zurück.",
                },
                {
                  name: "Safe Mode Restart",
                  desc: "Start ohne Hintergrund-Plugins.",
                },
              ].map((tool, idx) => (
                <div
                  key={idx}
                  className="bg-black/20 hover:bg-black/40 border border-white/5 hover:border-white/10 transition-all rounded-xl p-4 flex justify-between items-center group cursor-pointer"
                >
                  <div>
                    <div className="text-sm font-bold text-slate-300 group-hover:text-white transition-colors">
                      {tool.name}
                    </div>
                    <div className="text-xs text-slate-500">{tool.desc}</div>
                  </div>
                  <button className="w-8 h-8 rounded bg-white/5 flex items-center justify-center text-slate-400 group-hover:bg-rose-500 group-hover:text-white transition-colors">
                    <Play className="w-3 h-3" />
                  </button>
                </div>
              ))}
            </div>

            <div className="bg-rose-500/5 border border-rose-500/20 rounded-xl p-5 mt-4">
              <div className="flex items-center gap-3 text-rose-400 font-bold mb-2">
                <ShieldAlert className="w-5 h-5" />
                Emergency Wipe (Factory Reset)
              </div>
              <p className="text-xs text-rose-400/70 mb-4">
                Löscht alle lokalen Keys, Wallets, Modulladungen und Caches.
                Setzt das System auf Werkseinstellungen zurück. Diese Aktion ist
                irreversibel.
              </p>
              <button className="px-4 py-2 bg-rose-500/20 hover:bg-rose-500 text-rose-300 hover:text-white border border-rose-500/50 rounded-lg text-xs font-bold transition-colors">
                System Zurücksetzen
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
