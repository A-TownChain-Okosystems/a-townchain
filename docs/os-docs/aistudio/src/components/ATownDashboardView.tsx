import React, { useState, useEffect } from 'react';
import { Pickaxe, Cpu, Zap, Activity, Shield, Users, Server, HardDrive, BarChart3, Database } from 'lucide-react';
import { motion } from 'motion/react';

interface MinerStats {
  id: string;
  hashrate: number;
  stake: number;
  importance: number;
  aiCompute: number;
  status: 'active' | 'syncing' | 'offline';
  isLeader: boolean;
  uptime: number;
  blocksMined: number;
  auroraPower: number; // specialized metric for A-Town
}

export function ATownDashboardView() {
  const [miners, setMiners] = useState<MinerStats[]>([]);
  const [networkHashrate, setNetworkHashrate] = useState(0);
  const [currentEpoch, setCurrentEpoch] = useState(1);
  const [currentSlot, setCurrentSlot] = useState(0);
  const [miningStatus, setMiningStatus] = useState('Standby');
  const [auroraIntensity, setAuroraIntensity] = useState(0);
  
  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (miningStatus === 'Mining...') {
      interval = setInterval(() => {
        setAuroraIntensity(prev => (prev >= 100 ? 0 : prev + 2));
      }, 100);
    } else {
      setAuroraIntensity(0);
    }
    return () => clearInterval(interval);
  }, [miningStatus]);
  
  useEffect(() => {
    // Generate initial dummy data
    const initialMiners: MinerStats[] = Array.from({ length: 8 }).map((_, i) => ({
      id: `ATC-Miner-${Math.random().toString(36).substring(2, 8).toUpperCase()}`,
      hashrate: 10 + Math.random() * 90,
      stake: 1000 + Math.floor(Math.random() * 50000),
      importance: Math.floor(Math.random() * 100),
      aiCompute: Math.floor(Math.random() * 500),
      status: Math.random() > 0.1 ? 'active' : 'syncing',
      isLeader: false,
      uptime: 95 + Math.random() * 5,
      blocksMined: Math.floor(Math.random() * 500),
      auroraPower: 50 + Math.random() * 200,
    }));
    
    // Select leader based on importance
    const sortedByPoI = [...initialMiners].sort((a, b) => b.importance - a.importance);
    if (sortedByPoI.length > 0) {
      sortedByPoI[0].isLeader = true;
    }
    
    setMiners(initialMiners);
    setNetworkHashrate(initialMiners.reduce((acc, m) => acc + m.hashrate, 0));
    
    // Simulate real-time updates
    const interval = setInterval(() => {
      setMiners(currentMiners => {
        const updated = currentMiners.map(miner => ({
          ...miner,
          hashrate: miner.status === 'active' ? Math.max(5, miner.hashrate + (Math.random() * 4 - 2)) : 0,
          auroraPower: miner.status === 'active' ? Math.max(10, miner.auroraPower + (Math.random() * 10 - 5)) : 0,
        }));
        
        // Slot updates
        setCurrentSlot(s => {
          if (s >= 10) {
            setCurrentEpoch(e => e + 1);
            
            // Re-elect leader using weighted random selection (simulating PoI logic)
            // Just picking random for UI simulation
            const activeM = updated.filter(m => m.status === 'active');
            if (activeM.length > 0) {
              updated.forEach(m => m.isLeader = false);
              activeM[Math.floor(Math.random() * activeM.length)].isLeader = true;
            }
            return 0;
          }
          return s + 1;
        });
        
        return updated;
      });
    }, 3000);
    
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    setNetworkHashrate(miners.reduce((acc, m) => acc + m.hashrate, 0));
  }, [miners]);

  const activeMiners = miners.filter(m => m.status === 'active').length;
  const currentLeader = miners.find(m => m.isLeader);

  return (
    <div className="flex flex-col h-full bg-[#050B14] text-slate-300 font-sans overflow-hidden">
      {/* Header */}
      <div className="flex-none p-6 border-b border-white/5 bg-slate-900/40 relative overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-r from-blue-500/10 via-purple-500/5 to-transparent pointer-events-none" />
        <div className="relative z-10 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 bg-indigo-500/20 rounded-xl flex items-center justify-center border border-indigo-500/30">
              <Pickaxe className="w-6 h-6 text-indigo-400" />
            </div>
            <div>
              <h1 className="text-xl font-bold font-mono text-white tracking-tight flex items-center gap-2">
                A-Town
                <span className="px-2 py-0.5 rounded bg-indigo-500/20 text-indigo-300 text-[10px] uppercase border border-indigo-500/30">Aurora Edition</span>
              </h1>
              <p className="text-sm text-slate-400 mt-0.5">Advanced Validator Dashboard & Resource Allocation Hub</p>
            </div>
          </div>
          
          <div className="flex gap-4">
            <div className="bg-slate-900/50 border border-white/5 rounded-xl px-4 py-2 flex flex-col items-end">
              <span className="text-[10px] text-slate-400 font-mono uppercase">Current Epoch</span>
              <span className="text-lg font-mono text-white">{currentEpoch} <span className="text-slate-500 text-sm">/ Slot {currentSlot}/10</span></span>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 overflow-y-auto p-6 space-y-6 custom-scrollbar">
        
        {/* Top Stats Grid */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-slate-900/40 border border-white/5 rounded-xl p-4 relative overflow-hidden group">
            <div className="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
            <div className="flex items-start justify-between">
              <div>
                <p className="text-xs text-slate-400 font-mono uppercase mb-1">Global Hashrate</p>
                <div className="text-2xl font-bold text-white font-mono">{networkHashrate.toFixed(2)} TH/s</div>
              </div>
              <div className="w-8 h-8 rounded-lg bg-blue-500/20 flex items-center justify-center">
                <Activity className="w-4 h-4 text-blue-400" />
              </div>
            </div>
          </div>
          
          <div className="bg-slate-900/40 border border-white/5 rounded-xl p-4 relative overflow-hidden group">
            <div className="absolute inset-0 bg-gradient-to-br from-green-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
            <div className="flex items-start justify-between">
              <div>
                <p className="text-xs text-slate-400 font-mono uppercase mb-1">Active Roles</p>
                <div className="text-2xl font-bold text-white font-mono">{activeMiners} <span className="text-sm text-slate-500">/ {miners.length}</span></div>
              </div>
              <div className="w-8 h-8 rounded-lg bg-green-500/20 flex items-center justify-center">
                <Server className="w-4 h-4 text-green-400" />
              </div>
            </div>
          </div>

          <div className="bg-slate-900/40 border border-white/5 rounded-xl p-4 relative overflow-hidden group hover:border-purple-500/30 transition-colors">
            <div className="flex items-start justify-between">
              <div>
                <p className="text-xs text-slate-400 font-mono uppercase mb-1 flex items-center gap-1">
                  Global Aurora Power <Zap className="w-3 h-3 text-purple-400" />
                </p>
                <div className="text-2xl font-bold text-purple-300 font-mono">
                  {miners.reduce((acc, m) => acc + m.auroraPower, 0).toFixed(0)} <span className="text-sm opacity-50">AP</span>
                </div>
              </div>
              <div className="w-8 h-8 rounded-lg bg-purple-500/20 flex items-center justify-center">
                <BarChart3 className="w-4 h-4 text-purple-400" />
              </div>
            </div>
          </div>
          
          <div className="bg-slate-900/40 border border-amber-500/20 rounded-xl p-4 relative overflow-hidden">
            <div className="flex items-start justify-between">
              <div>
                <p className="text-xs text-amber-500/70 font-mono uppercase mb-1">Current Leader</p>
                <div className="text-sm font-bold text-amber-400 font-mono truncate max-w-[120px]">
                  {currentLeader ? currentLeader.id : 'Elections...'}
                </div>
              </div>
              <div className="w-8 h-8 rounded-lg bg-amber-500/20 flex items-center justify-center">
                <Shield className="w-4 h-4 text-amber-400" />
              </div>
            </div>
            {currentLeader && (
              <div className="mt-2 text-[10px] text-amber-500/50 font-mono flex justify-between">
                <span>PoI: {currentLeader.importance}</span>
                <span>Stake: {currentLeader.stake.toLocaleString()}</span>
              </div>
            )}
          </div>
        </div>

        {/* Aurora Visualization Panel */}
        <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 flex flex-col items-center relative overflow-hidden">
          <div className="w-full h-48 bg-[#050B14] rounded-xl mb-6 relative overflow-hidden border border-white/5 shadow-inner">
            <div 
              className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-indigo-500/30 via-purple-500/10 to-transparent transition-all duration-100"
              style={{ height: auroraIntensity + '%' }}
            />
            <div className="absolute inset-0 flex items-center justify-center font-mono text-cyan-500/50 text-xs">
              {miningStatus === 'Mining...' 
                ? `[PoAI-Calculation active: Training... ${(auroraIntensity * 0.94).toFixed(1)}%]`
                : '[PoAI-Calculation inactive: Awaiting Start]'}
            </div>
          </div>
          
          <button 
            onClick={() => setMiningStatus(s => s === 'Standby' ? 'Mining...' : 'Standby')}
            className={`px-12 py-3 rounded-full font-mono font-bold text-sm flex items-center gap-3 transition-all ${
              miningStatus === 'Standby' 
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 hover:bg-indigo-500/30' 
              : 'bg-purple-500/20 text-purple-300 border border-purple-500/50 hover:bg-purple-500/30 shadow-[0_0_15px_rgba(168,85,247,0.3)]'
            }`}
          >
            <Zap className={`w-4 h-4 ${miningStatus === 'Mining...' ? 'animate-pulse text-purple-400' : 'text-indigo-400'}`} />
            {miningStatus === 'Standby' ? 'Start Mining' : 'Mining Active...'}
          </button>
        </div>

        {/* Validation Fleet Grid */}
        <div>
          <h2 className="text-sm font-bold text-white mb-4 font-mono uppercase flex items-center gap-2">
            <Users className="w-4 h-4 text-slate-400" />
            Validation Fleet Ecosystem
          </h2>
          <div className="bg-slate-900/40 border border-white/5 rounded-xl overflow-hidden">
            <div className="overflow-x-auto">
              <table className="w-full text-left text-sm whitespace-nowrap">
                <thead className="bg-slate-800/50 border-b border-white/5">
                  <tr>
                    <th className="px-4 py-3 font-mono text-[10px] text-slate-400 uppercase tracking-wider">Node ID</th>
                    <th className="px-4 py-3 font-mono text-[10px] text-slate-400 uppercase tracking-wider">Status</th>
                    <th className="px-4 py-3 font-mono text-[10px] text-slate-400 uppercase tracking-wider text-right">Hashrate (PoW)</th>
                    <th className="px-4 py-3 font-mono text-[10px] text-slate-400 uppercase tracking-wider text-right">Stake (PoS)</th>
                    <th className="px-4 py-3 font-mono text-[10px] text-indigo-400 uppercase tracking-wider text-right">Importance (PoI)</th>
                    <th className="px-4 py-3 font-mono text-[10px] text-purple-400 uppercase tracking-wider text-right">Aurora Power</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-white/5">
                  {miners.map((miner) => (
                    <motion.tr 
                      key={miner.id}
                      layout
                      className={`hover:bg-white/[0.02] transition-colors ${miner.isLeader ? 'bg-amber-500/[0.05]' : ''}`}
                    >
                      <td className="px-4 py-3">
                        <div className="flex items-center gap-2">
                          {miner.isLeader && <Shield className="w-3.5 h-3.5 text-amber-400" />}
                          <span className={`font-mono text-xs ${miner.isLeader ? 'text-amber-400 font-bold' : 'text-slate-300'}`}>
                            {miner.id}
                          </span>
                        </div>
                      </td>
                      <td className="px-4 py-3">
                        <span className={`inline-flex flex-shrink-0 items-center justify-center px-2 py-0.5 rounded-full text-[10px] font-mono border ${
                          miner.status === 'active' 
                            ? 'bg-green-500/10 text-green-400 border-green-500/20' 
                            : 'bg-amber-500/10 text-amber-400 border-amber-500/20'
                        }`}>
                          {miner.status}
                        </span>
                      </td>
                      <td className="px-4 py-3 text-right">
                        <div className="flex items-center justify-end gap-2">
                          <span className="font-mono text-xs text-slate-300">{miner.hashrate.toFixed(1)} TH/s</span>
                          <div className="w-16 h-1.5 bg-slate-800 rounded-full overflow-hidden">
                            <div 
                              className="h-full bg-blue-500 rounded-full transition-all duration-500"
                              style={{ width: Math.min(100, miner.hashrate) + '%' }}
                            />
                          </div>
                        </div>
                      </td>
                      <td className="px-4 py-3 text-right font-mono text-xs text-slate-300">
                        {miner.stake.toLocaleString()} ATC
                      </td>
                      <td className="px-4 py-3 text-right font-mono text-xs text-indigo-300 font-bold">
                        {miner.importance}
                      </td>
                      <td className="px-4 py-3 text-right">
                        <div className="inline-flex items-center gap-1.5 px-2 py-1 bg-purple-500/10 rounded-lg border border-purple-500/20">
                          <Zap className="w-3 h-3 text-purple-400" />
                          <span className="font-mono text-xs text-purple-300">{miner.auroraPower.toFixed(0)}</span>
                        </div>
                      </td>
                    </motion.tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>

      </div>
    </div>
  );
}
