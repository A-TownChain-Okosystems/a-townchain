import React, { useState, useEffect } from 'react';
import { Shield, Crosshair, AlertTriangle, Fingerprint, Lock, Zap, Activity, Scan, Server, Code } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export function AntiCheatView() {
  const [isScanning, setIsScanning] = useState(false);
  const [scanProgress, setScanProgress] = useState(0);
  const [activeTab, setActiveTab] = useState<'status' | 'logs' | 'settings'>('status');

  const [logs, setLogs] = useState([
    { id: 1, time: '14:28:33', msg: 'Kernel driver integrity verified', status: 'ok' },
    { id: 2, time: '14:28:35', msg: 'Memory obfuscation active', status: 'ok' },
  ]);

  const runFullScan = () => {
    if (isScanning) return;
    setIsScanning(true);
    setScanProgress(0);

    const interval = setInterval(() => {
      setScanProgress(p => {
        if (p >= 100) {
          clearInterval(interval);
          setIsScanning(false);
          setLogs(prev => [{ id: Date.now(), time: new Date().toLocaleTimeString(), msg: 'Deep scan completed. No anomalies found.', status: 'ok' }, ...prev]);
          return 100;
        }
        return p + Math.floor(Math.random() * 10) + 5;
      });
    }, 400);

    setLogs(prev => [
      { id: Date.now(), time: new Date().toLocaleTimeString(), msg: 'Initiating deep memory heuristics...', status: 'warn' },
      ...prev
    ]);
  };

  return (
    <div className="flex flex-col h-full bg-[#050815] text-slate-200 border border-red-500/30 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center justify-between px-4 h-14 bg-[#090b14]/80 border-b border-red-500/20 shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-red-500/10 border border-red-500/30 shadow-[0_0_15px_rgba(239,68,68,0.2)]">
            <Shield className="w-5 h-5 text-red-500" />
          </div>
          <div>
            <h2 className="font-bold text-white tracking-tight leading-tight">
              ATC Anti-Cheat Engine
            </h2>
            <p className="text-[10px] text-red-400/80 font-mono">
              FairPlay Runtime Verification & Memory Scanner
            </p>
          </div>
        </div>
        <div className="flex gap-1 bg-black/40 p-1 rounded-lg border border-white/5">
          {['status', 'logs', 'settings'].map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab as any)}
              className={`px-3 py-1.5 rounded-lg text-xs font-bold transition-all ${activeTab === tab ? "bg-red-500/20 text-red-300 border border-red-500/30 shadow-[0_0_15px_rgba(239,68,68,0.15)]" : "text-slate-400 hover:bg-white/5 border border-transparent"} capitalize`}
            >
              {tab}
            </button>
          ))}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 relative">
        <div className="absolute inset-0 bg-gradient-to-br from-red-500/5 via-transparent to-transparent pointer-events-none" />
        
        {activeTab === 'status' && (
          <div className="flex flex-col gap-6 relative z-10">
            {/* Status Hero */}
            <div className="flex items-center justify-between bg-[#040714]/80 border border-red-500/20 p-6 rounded-xl shadow-[0_0_30px_rgba(239,68,68,0.05)] overflow-hidden relative">
              <div className="absolute right-0 top-0 w-64 h-64 bg-red-500/10 rounded-full blur-[60px] translate-x-1/3 -translate-y-1/3 pointer-events-none" />
              <div className="flex items-center gap-6 relative z-10">
                <div className="relative">
                  <div className="w-20 h-20 rounded-full bg-red-500/10 border-2 border-red-500/40 flex items-center justify-center">
                    <Fingerprint className="w-10 h-10 text-red-400" />
                  </div>
                  {isScanning && (
                    <motion.div 
                      className="absolute inset-0 border-2 border-transparent border-t-red-400 rounded-full"
                      animate={{ rotate: 360 }}
                      transition={{ duration: 1, repeat: Infinity, ease: "linear" }}
                    />
                  )}
                  <div className="absolute -bottom-1 -right-1 w-6 h-6 bg-slate-900 rounded-full border border-red-500/40 flex items-center justify-center">
                    <div className={`w-3 h-3 rounded-full ${isScanning ? 'bg-yellow-400 animate-pulse' : 'bg-emerald-400 shadow-[0_0_10px_rgba(52,211,153,0.8)]'}`} />
                  </div>
                </div>
                <div>
                  <h3 className="text-xl font-black text-white tracking-wide uppercase">System Secure</h3>
                  <p className="text-sm text-slate-400 mt-1 max-w-sm">
                    No unauthorized modifications detected in memory space. Process integrity is intact.
                  </p>
                </div>
              </div>
              <button
                onClick={runFullScan}
                disabled={isScanning}
                className="px-6 py-3 bg-red-500/20 hover:bg-red-500/30 text-red-300 border border-red-500/40 rounded-xl font-bold text-sm tracking-wide transition-all flex items-center gap-2 disabled:opacity-50 relative z-10 hover:scale-105 shadow-[0_0_20px_rgba(239,68,68,0.2)]"
              >
                {isScanning ? <Scan className="w-5 h-5 animate-spin" /> : <Crosshair className="w-5 h-5" />}
                {isScanning ? "Scanning Memory..." : "Initiate Deep Scan"}
              </button>
            </div>

            {isScanning && (
               <div className="p-4 border border-red-500/30 bg-red-500/10 rounded-xl relative z-10">
                 <div className="flex justify-between items-center mb-2">
                   <div className="text-xs text-red-400 font-mono flex items-center gap-2">
                     <span className="w-2 h-2 bg-red-400 rounded-full animate-pulse" />
                     Analyzing Kernel Memory Regions... [{Math.min(scanProgress, 100)}%]
                   </div>
                   <span className="text-xs text-red-300 font-mono">{Math.min(scanProgress, 100)}%</span>
                 </div>
                 <div className="w-full bg-black/50 rounded-full h-1.5 overflow-hidden relative">
                   <div className="absolute top-0 left-0 bottom-0 bg-red-500 shadow-[0_0_10px_rgba(239,68,68,0.8)] transition-all duration-300" style={{ width: `${scanProgress}%` }} />
                 </div>
               </div>
            )}

            {/* Modules */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 relative z-10">
              <div className="bg-[#090b14]/80 border border-white/5 p-4 rounded-xl hover:border-red-500/20 transition-colors">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <Server className="w-5 h-5 text-red-400" />
                    <span className="font-bold text-slate-200">Runtime Integrity</span>
                  </div>
                  <span className="text-[10px] font-mono bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded border border-emerald-500/30">ACTIVE</span>
                </div>
                <p className="text-xs text-slate-500 mb-3">Prevents DLL injection and memory patching attempts automatically.</p>
                <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                  <div className="h-full bg-emerald-500 w-full" />
                </div>
              </div>

              <div className="bg-[#090b14]/80 border border-white/5 p-4 rounded-xl hover:border-red-500/20 transition-colors">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <Activity className="w-5 h-5 text-red-400" />
                    <span className="font-bold text-slate-200">Heuristics Engine</span>
                  </div>
                  <span className="text-[10px] font-mono bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded border border-emerald-500/30">ACTIVE</span>
                </div>
                <p className="text-xs text-slate-500 mb-3">Detects anomalous input patterns and automated macro behaviors.</p>
                <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                  <div className="h-full bg-emerald-500 w-full" />
                </div>
              </div>

              <div className="bg-[#090b14]/80 border border-white/5 p-4 rounded-xl hover:border-red-500/20 transition-colors">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <Code className="w-5 h-5 text-red-400" />
                    <span className="font-bold text-slate-200">Execution Obfuscation</span>
                  </div>
                  <span className="text-[10px] font-mono bg-amber-500/20 text-amber-400 px-2 py-0.5 rounded border border-amber-500/30">DYNAMIC</span>
                </div>
                <p className="text-xs text-slate-500 mb-3">Continuously scrambles critical memory variables to deter scanners.</p>
                <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                  <div className="h-full bg-amber-500 w-[85%]" />
                </div>
              </div>

              <div className="bg-[#090b14]/80 border border-white/5 p-4 rounded-xl hover:border-red-500/20 transition-colors">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <Lock className="w-5 h-5 text-red-400" />
                    <span className="font-bold text-slate-200">Game Client Sync</span>
                  </div>
                  <span className="text-[10px] font-mono bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded border border-emerald-500/30">VERIFIED</span>
                </div>
                <p className="text-xs text-slate-500 mb-3">Validates encrypted handshakes between local client and Battle Arena.</p>
                <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                  <div className="h-full bg-emerald-500 w-full" />
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'logs' && (
          <div className="flex flex-col gap-3 relative z-10">
            <div className="text-xs uppercase font-bold tracking-widest text-slate-500 mb-2">Real-Time Security Feed</div>
            <div className="flex flex-col gap-2 font-mono text-sm">
              <AnimatePresence>
                {logs.map((log) => (
                  <motion.div
                    key={log.id}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    className={`flex items-start gap-4 p-3 rounded-lg border ${
                      log.status === 'ok' ? 'bg-slate-800/30 border-white/5 text-slate-300' :
                      log.status === 'warn' ? 'bg-amber-500/10 border-amber-500/20 text-amber-300' :
                      'bg-red-500/10 border-red-500/20 text-red-300'
                    }`}
                  >
                    <span className="text-slate-500 shrink-0">[{log.time}]</span>
                    <span>{log.msg}</span>
                  </motion.div>
                ))}
              </AnimatePresence>
            </div>
          </div>
        )}

        {activeTab === 'settings' && (
          <div className="flex flex-col gap-6 relative z-10">
            <div className="bg-[#090b14]/80 border border-white/5 p-6 rounded-xl">
              <h3 className="text-sm font-bold text-white mb-6 uppercase tracking-widest">Enforcement Policies</h3>
              
              <div className="space-y-4">
                <div className="flex items-center justify-between p-4 bg-black/40 rounded-lg border border-white/5">
                  <div>
                    <div className="font-bold text-slate-200">Strict Matchmaking Verification</div>
                    <div className="text-xs text-slate-500 mt-1">Requires hardware attestation before queuing.</div>
                  </div>
                  <div className="w-10 h-5 bg-red-500/20 rounded-full border border-red-500/50 p-0.5 flex items-center justify-end cursor-pointer">
                    <div className="w-4 h-4 bg-red-400 rounded-full shadow-[0_0_10px_rgba(239,68,68,0.8)]" />
                  </div>
                </div>

                <div className="flex items-center justify-between p-4 bg-black/40 rounded-lg border border-white/5">
                  <div>
                    <div className="font-bold text-slate-200">Kernel-Level Memory Access</div>
                    <div className="text-xs text-slate-500 mt-1">Allows deep OS integration for detecting advanced exploits.</div>
                  </div>
                  <div className="w-10 h-5 bg-red-500/20 rounded-full border border-red-500/50 p-0.5 flex items-center justify-end cursor-pointer">
                    <div className="w-4 h-4 bg-red-400 rounded-full shadow-[0_0_10px_rgba(239,68,68,0.8)]" />
                  </div>
                </div>

                <div className="flex items-center justify-between p-4 bg-black/40 rounded-lg border border-white/5">
                  <div>
                    <div className="font-bold text-slate-200">Auto-Ban Execution</div>
                    <div className="text-xs text-slate-500 mt-1">Automatically suspend accounts upon confirmation of injected memory.</div>
                  </div>
                  <div className="w-10 h-5 bg-slate-800 rounded-full border border-white/10 p-0.5 flex items-center cursor-pointer">
                    <div className="w-4 h-4 bg-slate-500 rounded-full" />
                  </div>
                </div>
              </div>
            </div>
            
            <div className="p-4 bg-red-500/10 border border-red-500/30 rounded-xl flex items-start gap-4">
              <AlertTriangle className="w-5 h-5 text-red-500 shrink-0 mt-0.5" />
              <div>
                <h4 className="font-bold text-red-400 text-sm">Warning: System Integration</h4>
                <p className="text-xs text-red-400/80 mt-1">
                  Disabling kernel-level access will prevent you from participating in ranked or competitive Battle Arena matches. Any attempt to tamper with the Anti-Cheat process will result in an immediate hardware ban.
                </p>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
