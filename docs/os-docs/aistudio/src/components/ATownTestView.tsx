import React, { useState, useEffect } from 'react';
import { motion } from 'motion/react';
import { Server, Cpu, Database, Network, Wallet, Activity, Code, Settings, Power, FileCode, Globe, Bot, Factory, Terminal } from 'lucide-react';

export function ATownTestView() {
  const [isRunning, setIsRunning] = useState(false);
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (isRunning) {
      interval = setInterval(() => {
        const events = [
          "VM: Executing Smart Contract 0x...",
          "NODE: Pinging peer 10.4.x.x...",
          "MINER: Hash rate currently 42 TH/s",
          "BLOCKCHAIN: New block minted #34992",
          "WALLET: Tx 0x8a9bf received.",
          "TEST: Suite passing 44/44",
          "CONTRACT: Validating A-TOWN smartcontract...",
          "BROWSER: GateToHell navigating to Web/Web3/Tor...",
          "OS: ATC-OS scheduling AI tasks...",
          "FACTORY: Franchise Factury spawning new sidechain...",
          "LANG: Compiling A-TOWN custom programming language script..."
        ];
        const event = events[Math.floor(Math.random() * events.length)];
        setLogs(prev => [event, ...prev].slice(0, 15));
      }, 1500);
    }
    return () => clearInterval(interval);
  }, [isRunning]);

  const components = [
    { title: 'A-TOWN Test', icon: Code, desc: "Testing suite and QA environment", status: isRunning ? "Active" : "Idle", color: "text-blue-400" },
    { title: 'A-TOWN VM', icon: Cpu, desc: "Virtual Machine execution engine", status: isRunning ? "Running" : "Idle", color: "text-purple-400" },
    { title: 'A-TOWN BLOCKCHAIN', icon: Database, desc: "Core ledger and consensus", status: isRunning ? "Synced" : "Paused", color: "text-emerald-400" },
    { title: 'A-TOWN NODE', icon: Server, desc: "P2P network peer instance", status: isRunning ? "Connected" : "Offline", color: "text-cyan-400" },
    { title: 'A-TOWN MINER', icon: Activity, desc: "Proof-of-Work / Validation", status: isRunning ? "Hashing" : "Idle", color: "text-amber-400" },
    { title: 'A-TOWN wallet', icon: Wallet, desc: "Key management & transactions", status: isRunning ? "Secure" : "Locked", color: "text-rose-400" },
    { title: 'A-TOWN smartcontract', icon: FileCode, desc: "Immutable rule execution", status: isRunning ? "Verified" : "Compiled", color: "text-fuchsia-400" },
    { title: 'A-TOWN Browser "GateToHell"', icon: Globe, desc: "Web, Web3 & Tor network browser", status: isRunning ? "Active" : "Offline", color: "text-red-500" },
    { title: 'ATC-OS (Cryptographic AI OS)', icon: Bot, desc: "AI Orchestration layer", status: isRunning ? "Learning" : "Sleeping", color: "text-teal-400" },
    { title: 'ATC-OS System', icon: Network, desc: "Decentralized AI execution and agents", status: isRunning ? "Processing" : "Idle", color: "text-cyan-300" },
    { title: 'A-TOWN Franchise Factury', icon: Factory, desc: "Scalable ecosystem generator", status: isRunning ? "Producing" : "Standby", color: "text-orange-400" },
    { title: 'A-TOWN eigene Programmiersprache', icon: Terminal, desc: "Native smart contract execution language", status: isRunning ? "Compiling" : "Idle", color: "text-lime-400" }
  ];

  return (
    <div className="flex flex-col gap-6 mt-8 pb-12 w-full max-w-6xl mx-auto">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-4">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
            <Server className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">A-TOWN Test Dashboard</h2>
            <p className="text-sm font-light text-slate-400">Simulation environment for core A-TOWN components.</p>
          </div>
        </div>
        
        <button 
          onClick={() => setIsRunning(!isRunning)}
          className={`flex items-center gap-2 px-6 py-3 rounded-lg font-medium transition-colors border shadow-[0_0_15px_rgba(0,0,0,0.5)] ${isRunning ? 'bg-red-500/20 text-red-500 border-red-500/50 hover:bg-red-500/30' : 'bg-emerald-500/20 text-emerald-500 border-emerald-500/50 hover:bg-emerald-500/30'}`}
        >
          <Power className="w-5 h-5" />
          {isRunning ? "Stop Sandbox" : "Start Sandbox"}
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-6">
        {components.map((comp, idx) => (
          <motion.div 
            key={idx}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: idx * 0.1 }}
            className={`p-6 rounded-2xl border ${isRunning ? 'border-indigo-500/30 shadow-[0_0_15px_rgba(99,102,241,0.1)]' : 'border-atc-border/50'} bg-[#090b14]/60 backdrop-blur-md relative overflow-hidden group`}
          >
            {isRunning && <div className="absolute inset-0 bg-gradient-to-br from-white/5 to-transparent pointer-events-none" />}
            <div className="flex items-start justify-between mb-4">
              <div className={`p-3 rounded-xl bg-[#060a16] border border-atc-border/50 ${comp.color}`}>
                <comp.icon className="w-6 h-6" />
              </div>
              <div className={`px-3 py-1 rounded-full border text-xs font-mono font-bold tracking-wider ${isRunning ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30' : 'bg-slate-800 text-slate-400 border-slate-700'}`}>
                {comp.status}
              </div>
            </div>
            <h3 className="text-lg font-medium text-white mb-2">{comp.title}</h3>
            <p className="text-sm text-slate-400 leading-relaxed">{comp.desc}</p>
          </motion.div>
        ))}
      </div>

      <div className="p-6 rounded-2xl border border-atc-border/50 bg-[#060a16]/80 flex flex-col h-64">
        <div className="flex items-center gap-2 mb-4 border-b border-atc-border/30 pb-4">
          <Settings className="w-5 h-5 text-slate-500" />
          <h3 className="text-base font-medium text-slate-300">Sandbox Terminal Logs</h3>
        </div>
        <div className="flex-1 overflow-y-auto custom-scrollbar font-mono text-sm space-y-2 flex flex-col">
          {logs.map((log, i) => (
            <div key={i} className="text-slate-400">
              <span className="text-indigo-400 mr-3">[{new Date().toLocaleTimeString()}]</span>
              {log}
            </div>
          ))}
          {logs.length === 0 && <div className="text-slate-600">Waiting for components to start...</div>}
        </div>
      </div>
    </div>
  );
}
