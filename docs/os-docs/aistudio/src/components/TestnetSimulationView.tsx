import React, { useState, useEffect } from 'react';
import { Activity, Server, Network, Wifi, ShieldAlert, Zap, Box, Terminal } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

interface NodeStatus {
  id: number;
  status: 'offline' | 'discovering' | 'syncing' | 'active';
  isLeader: boolean;
  latency: number;
  tps: number;
  dbWrites: number;
}

export function TestnetSimulationView() {
  const [nodes, setNodes] = useState<NodeStatus[]>([]);
  const [simulationState, setSimulationState] = useState<'idle' | 'starting' | 'running' | 'chaos' | 'recovered'>('idle');
  const [logs, setLogs] = useState<string[]>([]);
  const [globalTps, setGlobalTps] = useState(0);

  const addLog = (msg: string) => {
    setLogs(prev => [...prev.slice(-10), `[${new Date().toISOString().split('T')[1].slice(0,-1)}] ${msg}`]);
  };

  const startSimulation = () => {
    setSimulationState('starting');
    setLogs([]);
    addLog("Starte A-TownChain Testnet Simulation (20 Nodes)...");

    // Initialize 20 offine nodes
    const initialNodes = Array.from({ length: 20 }, (_, i) => ({
      id: i + 1,
      status: 'offline' as const,
      isLeader: false,
      latency: 0,
      tps: 0,
      dbWrites: 0
    }));
    setNodes(initialNodes);

    setTimeout(() => {
      addLog("Node-Container via Docker-Compose gestartet...");
      setNodes(initialNodes.map(n => ({ ...n, status: 'discovering' })));
      
      setTimeout(() => {
        addLog("Peer Discovery via libp2p erfolgreich. Syncing...");
        setNodes(initialNodes.map(n => ({ ...n, status: 'syncing', latency: 45 + Math.random() * 50 })));
        
        setTimeout(() => {
          setSimulationState('running');
          addLog("Netzwerk live. 20 Nodes aktiv. Leader-Election gestartet.");
          
          let activeNodes = initialNodes.map(n => ({ 
            ...n, 
            status: 'active' as const, 
            isLeader: false, 
            latency: 10 + Math.random() * 40,
            tps: Math.floor(Math.random() * 5),
            dbWrites: 0
          }));
          
          // Pick a leader
          const leaderIdx = Math.floor(Math.random() * 20);
          activeNodes[leaderIdx].isLeader = true;
          setNodes(activeNodes);
          addLog(`Node ${leaderIdx + 1} wurde als Leader gewählt. Setze Load auf 100 TPS.`);

        }, 2000);
      }, 2000);
    }, 1000);
  };

  useEffect(() => {
    if (simulationState !== 'running' && simulationState !== 'chaos' && simulationState !== 'recovered') return;

    const interval = setInterval(() => {
      setNodes(prev => {
        let totalTps = 0;
        const updated = prev.map(n => {
          if (n.status === 'offline') return n;
          // Random fluctuation
          const newTps = n.isLeader ? 80 + Math.random() * 40 : 2 + Math.random() * 8;
          totalTps += newTps;
          return {
            ...n,
            latency: Math.max(5, n.latency + (Math.random() * 10 - 5)),
            tps: newTps,
            dbWrites: n.dbWrites + Math.floor(newTps / 2)
          };
        });
        setGlobalTps(totalTps);
        return updated;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [simulationState]);

  const triggerChaos = () => {
    setSimulationState('chaos');
    addLog("INITIATING STRESS-TEST: Stopping 5 random nodes (Chaos Monkey)");
    
    setNodes(prev => {
      const updated = [...prev];
      const activeIdxs = updated.map((n, i) => n.status === 'active' ? i : -1).filter(i => i !== -1);
      
      // Pick 5 random
      for (let k = 0; k < 5; k++) {
        if (activeIdxs.length === 0) break;
        const rPos = Math.floor(Math.random() * activeIdxs.length);
        const idx = activeIdxs.splice(rPos, 1)[0];
        updated[idx].status = 'offline';
        updated[idx].isLeader = false;
        updated[idx].latency = 0;
        updated[idx].tps = 0;
      }
      return updated;
    });

    setTimeout(() => {
      addLog("Netzwerk hat Ausfälle erkannt. Re-Configuring & Re-Election...");
      
      setNodes(prev => {
        const updated = [...prev];
        const activeIdxs = updated.map((n, i) => n.status === 'active' ? i : -1).filter(i => i !== -1);
        if (activeIdxs.length > 0 && !updated.some(n => n.isLeader)) {
           const newLeaderIdx = activeIdxs[Math.floor(Math.random() * activeIdxs.length)];
           updated[newLeaderIdx].isLeader = true;
           addLog(`Netzwerk stabilisiert in < 3s. Neuer Leader: Node ${newLeaderIdx + 1}`);
        }
        return updated;
      });
      setSimulationState('recovered');
    }, 2800);
  };

  return (
    <div className="flex flex-col h-full bg-[#050B14] border border-white/5 rounded-xl overflow-hidden font-sans">
      {/* Header */}
      <div className="flex items-center gap-3 px-6 py-4 border-b border-white/5 bg-slate-900/50">
        <Network className="w-5 h-5 text-emerald-400" />
        <div>
          <h2 className="text-sm font-bold text-white tracking-widest font-mono uppercase">Hybrid-Consensus Testnet</h2>
          <p className="text-xs text-slate-400 mt-0.5">20-Node P2P Stresstest Simulation & Gossipsub Load</p>
        </div>
      </div>
      
      <div className="flex-1 flex overflow-hidden">
        {/* Left Panel - Nodes Grid */}
        <div className="flex-1 p-6 overflow-y-auto custom-scrollbar">
          <div className="flex justify-between items-center mb-6">
            <h3 className="font-mono text-sm text-slate-300 flex items-center gap-2">
              <Server className="w-4 h-4 text-indigo-400" /> Active Fleet
            </h3>
            <div className="flex gap-2">
              <button 
                onClick={startSimulation}
                disabled={simulationState !== 'idle'}
                className="px-4 py-2 bg-indigo-500/10 text-indigo-400 border border-indigo-500/30 hover:bg-indigo-500/20 disabled:opacity-50 disabled:cursor-not-allowed rounded font-mono text-xs transition-colors"
               >
                Boot Simulation
              </button>
              <button 
                onClick={triggerChaos}
                disabled={simulationState !== 'running'}
                className="px-4 py-2 bg-rose-500/10 text-rose-400 border border-rose-500/30 hover:bg-rose-500/20 disabled:opacity-50 disabled:cursor-not-allowed rounded font-mono text-xs transition-colors flex items-center gap-2"
               >
                <ShieldAlert className="w-4 h-4" /> Trigger Split (Chaos)
              </button>
            </div>
          </div>

          <div className="grid grid-cols-2 lg:grid-cols-4 xl:grid-cols-5 gap-4">
            {nodes.length === 0 ? (
                <div className="col-span-full h-64 flex items-center justify-center text-slate-500 font-mono text-sm border border-dashed border-slate-800 rounded-xl">
                    Netzwerk Offline. Wähle "Boot Simulation".
                </div>
            ) : null}
            <AnimatePresence>
              {nodes.map(node => (
                <motion.div 
                  key={node.id}
                  layout
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  className={`p-4 rounded-xl border relative overflow-hidden transition-colors ${
                    node.status === 'offline' ? 'bg-slate-900/20 border-rose-500/10 opacity-50' :
                    node.isLeader ? 'bg-amber-500/10 border-amber-500/30 shadow-[0_0_15px_rgba(245,158,11,0.1)]' :
                    'bg-slate-900/40 border-indigo-500/10'
                  }`}
                >
                  <div className="flex justify-between items-start mb-2">
                    <span className={`font-mono text-sm font-bold ${node.isLeader ? 'text-amber-400' : 'text-slate-300'}`}>
                      Node-{node.id.toString().padStart(2, '0')}
                    </span>
                    <span className="flex h-2 w-2 relative">
                      {node.status === 'active' && (
                        <span className={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 ${node.isLeader ? 'bg-amber-400' : 'bg-emerald-400'}`}></span>
                      )}
                      <span className={`relative inline-flex rounded-full h-2 w-2 ${
                        node.status === 'active' ? (node.isLeader ? 'bg-amber-500' : 'bg-emerald-500') :
                        node.status === 'syncing' ? 'bg-blue-500' : 
                        node.status === 'discovering' ? 'bg-purple-500' : 'bg-rose-500'
                      }`}></span>
                    </span>
                  </div>
                  
                  <div className="space-y-1.5 mt-4">
                    <div className="flex justify-between text-[10px] font-mono">
                      <span className="text-slate-500">Status</span>
                      <span className={node.status === 'offline' ? 'text-rose-400/50' : 'text-blue-300'}>{node.status.toUpperCase()}</span>
                    </div>
                    <div className="flex justify-between text-[10px] font-mono">
                      <span className="text-slate-500 text-xs flex items-center gap-1"><Wifi className="w-3 h-3"/> Ping</span>
                      <span className="text-slate-400">{node.latency.toFixed(0)} ms</span>
                    </div>
                    <div className="flex justify-between text-[10px] font-mono">
                      <span className="text-slate-500 text-xs flex items-center gap-1"><Zap className="w-3 h-3"/> TPS</span>
                      <span className={node.isLeader ? 'text-amber-300' : 'text-slate-400'}>{node.tps.toFixed(1)}</span>
                    </div>
                    <div className="flex justify-between text-[10px] font-mono">
                      <span className="text-slate-500 text-xs flex items-center gap-1"><Box className="w-3 h-3"/> DB Writes</span>
                      <span className="text-slate-400">{node.dbWrites}</span>
                    </div>
                  </div>
                </motion.div>
              ))}
            </AnimatePresence>
          </div>
        </div>

        {/* Right Panel - Terminal & Stats */}
        <div className="w-80 border-l border-white/5 bg-[#020408] flex flex-col">
          <div className="p-4 border-b border-white/5">
            <h3 className="font-mono text-xs text-slate-400 uppercase mb-4 text-center">Global Network Metriken</h3>
            <div className="grid grid-cols-2 gap-3">
               <div className="bg-[#050B14] p-3 rounded-lg border border-white/5 text-center">
                 <div className="text-[10px] text-slate-500 font-mono mb-1">Active Nodes</div>
                 <div className="text-xl font-mono text-emerald-400">
                    {nodes.filter(n => n.status === 'active').length}
                 </div>
               </div>
               <div className="bg-[#050B14] p-3 rounded-lg border border-white/5 text-center">
                 <div className="text-[10px] text-slate-500 font-mono mb-1">Total TPS</div>
                 <div className="text-xl font-mono text-blue-400">
                    {globalTps.toFixed(0)}
                 </div>
               </div>
            </div>
            
            {(simulationState === 'running' || simulationState === 'recovered') && (
                <div className="mt-4 p-3 bg-indigo-500/10 border border-indigo-500/20 rounded-lg">
                    <div className="text-xs text-indigo-300 font-mono flex items-center gap-2 mb-2">
                        <Activity className="w-4 h-4" /> Consensus Health
                    </div>
                    <div className="text-[10px] text-slate-400 font-mono space-y-1">
                        <div className="flex justify-between"><span>Gossipsub Latency:</span> <span className="text-emerald-400">&lt;200ms</span></div>
                        <div className="flex justify-between"><span>Leader Status:</span> <span className="text-amber-400">Stable</span></div>
                        <div className="flex justify-between"><span>RocksDB Writes:</span> <span className="text-emerald-400">Optimal</span></div>
                    </div>
                </div>
            )}
          </div>
          
          <div className="flex-1 p-4 overflow-y-auto flex flex-col">
             <div className="text-xs font-mono text-slate-500 uppercase flex items-center gap-2 mb-3">
                 <Terminal className="w-4 h-4" /> Container Logs
             </div>
             <div className="flex-1 bg-[#050B14] rounded-lg border border-white/5 p-3 overflow-y-auto custom-scrollbar">
                {logs.length === 0 ? (
                    <div className="text-[10px] text-slate-600 font-mono italic">Waiting for init...</div>
                ) : (
                    logs.map((L, i) => (
                        <div key={i} className="text-[9px] font-mono mb-1.5 leading-relaxed text-slate-400">
                           <span className="text-blue-500/50">sys</span> <span className={L.includes('erfolgreich') || L.includes('stabilisiert') ? 'text-emerald-400/80' : L.includes('Chaos') || L.includes('Ausfälle') ? 'text-rose-400/80' : L.includes('Leader') ? 'text-amber-400/80' : 'text-slate-300'}>{L}</span>
                        </div>
                    ))
                )}
             </div>
          </div>
          
          <div className="p-4 border-t border-white/5 bg-slate-900/30">
              <h4 className="text-[10px] font-mono font-bold text-slate-400 uppercase mb-2 text-center">Nächste Option</h4>
              <div className="space-y-2">
                 <button className="w-full text-left px-3 py-2 rounded-lg bg-emerald-500/10 border border-emerald-500/20 hover:bg-emerald-500/20 transition-colors">
                     <span className="text-[11px] font-bold text-emerald-300 block">Genesis-Block-Generator</span>
                     <span className="text-[9px] text-emerald-400/60 font-mono">Mainnet-Bootstrap vorbereiten</span>
                 </button>
                 <button className="w-full text-left px-3 py-2 rounded-lg bg-purple-500/10 border border-purple-500/20 hover:bg-purple-500/20 transition-colors">
                     <span className="text-[11px] font-bold text-purple-300 block">PoAI Engine Simulation</span>
                     <span className="text-[9px] text-purple-400/60 font-mono">KI-Trainings-Overhead prüfen</span>
                 </button>
              </div>
          </div>
        </div>
      </div>
    </div>
  );
}
