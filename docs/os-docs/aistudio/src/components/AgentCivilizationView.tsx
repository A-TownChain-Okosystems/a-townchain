import React, { useState } from 'react';
import { Bot, Network, Activity, Zap, Hexagon, Cpu, Workflow, BarChart3, Users, Scale } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export function AgentCivilizationView() {
  const [activeScreen, setActiveScreen] = useState<'swarms' | 'market'>('swarms');

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="flex gap-4 p-4 border-b border-white/5 bg-black/20">
        <button
          onClick={() => setActiveScreen('swarms')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors ${
            activeScreen === 'swarms'
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 shadow-[0_0_15px_rgba(99,102,241,0.2)]'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <Network className="w-4 h-4" />
          Agent Swarms (MARL)
        </button>
        <button
          onClick={() => setActiveScreen('market')}
          className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors ${
            activeScreen === 'market'
              ? 'bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/30 shadow-[0_0_15px_rgba(6,182,212,0.2)]'
              : 'hover:bg-white/5 text-slate-400'
          }`}
        >
          <Scale className="w-4 h-4" />
          Nash Equilibrium Markets
        </button>
      </div>

      <div className="flex-1 overflow-auto p-6">
        <AnimatePresence mode="wait">
          {activeScreen === 'swarms' && (
            <motion.div
              key="swarms"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="flex flex-col gap-6"
            >
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="p-4 border border-white/10 rounded-xl bg-black/40 flex flex-col gap-2">
                  <div className="flex items-center gap-2 text-indigo-400">
                    <Bot className="w-5 h-5" />
                    <h3 className="font-bold">Active Agents</h3>
                  </div>
                  <span className="text-3xl font-mono text-white tracking-widest">1,402</span>
                  <p className="text-xs text-slate-400">Distributed across 14 swarms</p>
                </div>
                <div className="p-4 border border-white/10 rounded-xl bg-black/40 flex flex-col gap-2">
                  <div className="flex items-center gap-2 text-emerald-400">
                    <Activity className="w-5 h-5" />
                    <h3 className="font-bold">Coordination Ops</h3>
                  </div>
                  <span className="text-3xl font-mono text-white tracking-widest">48.2k/s</span>
                  <p className="text-xs text-slate-400">BFT Agent Coordination layer</p>
                </div>
                <div className="p-4 border border-white/10 rounded-xl bg-black/40 flex flex-col gap-2">
                  <div className="flex items-center gap-2 text-amber-400">
                    <Zap className="w-5 h-5" />
                    <h3 className="font-bold">MARL Epoch</h3>
                  </div>
                  <span className="text-3xl font-mono text-white tracking-widest">Ep. 42901</span>
                  <p className="text-xs text-slate-400">Multi-Agent Reinforcement Learning</p>
                </div>
              </div>

              <div className="p-6 border border-white/10 rounded-xl bg-black/20 flex flex-col gap-4">
                <h3 className="text-lg font-bold text-white flex items-center gap-2">
                  <Workflow className="w-5 h-5 text-indigo-400" />
                  Swarm Topology
                </h3>
                <div className="h-64 rounded bg-[#090b14] border border-white/5 relative overflow-hidden flex items-center justify-center">
                  <div className="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:20px_20px]" />
                  <div className="relative z-10 flex flex-col items-center gap-8">
                    <div className="flex gap-16">
                      <div className="w-16 h-16 rounded-full bg-indigo-500/20 border border-indigo-500 flex items-center justify-center animate-pulse shadow-[0_0_20px_rgba(99,102,241,0.4)]">
                        <Hexagon className="w-8 h-8 text-indigo-400" />
                      </div>
                      <div className="w-16 h-16 rounded-full bg-indigo-500/20 border border-indigo-500 flex items-center justify-center animate-pulse delay-75 shadow-[0_0_20px_rgba(99,102,241,0.4)]">
                        <Hexagon className="w-8 h-8 text-indigo-400" />
                      </div>
                    </div>
                    <div className="flex gap-12">
                      <div className="w-12 h-12 rounded-full bg-cyan-500/20 border border-cyan-500 flex items-center justify-center shadow-[0_0_15px_rgba(6,182,212,0.4)]">
                        <Cpu className="w-6 h-6 text-cyan-400" />
                      </div>
                      <div className="w-12 h-12 rounded-full bg-cyan-500/20 border border-cyan-500 flex items-center justify-center shadow-[0_0_15px_rgba(6,182,212,0.4)]">
                        <Cpu className="w-6 h-6 text-cyan-400" />
                      </div>
                      <div className="w-12 h-12 rounded-full bg-cyan-500/20 border border-cyan-500 flex items-center justify-center shadow-[0_0_15px_rgba(6,182,212,0.4)]">
                        <Cpu className="w-6 h-6 text-cyan-400" />
                      </div>
                    </div>
                  </div>
                </div>
                <div className="text-sm text-slate-400">
                  <p>Visualization of active Byzantine Fault Tolerant Agent Coordination clusters.</p>
                </div>
              </div>
            </motion.div>
          )}

          {activeScreen === 'market' && (
            <motion.div
              key="market"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="flex flex-col gap-6"
            >
              <div className="p-6 border border-white/10 rounded-xl bg-black/20 flex flex-col gap-4">
                 <h3 className="text-lg font-bold text-white flex items-center gap-2">
                  <BarChart3 className="w-5 h-5 text-atc-cyan" />
                  Automated Market Maker (AMM) für Agents
                </h3>
                <p className="text-sm text-slate-300">
                  Stable Matching Theory (Gale-Shapley algorithm) is currently maintaining market equilibrium between compute providers and agent task definitions.
                </p>

                <div className="flex flex-col gap-3 mt-4">
                  {[
                    { pair: "Compute_Agent ↔ Validating_Oracle", liquidity: "1.2M ATC", stable: true },
                    { pair: "Storage_Host ↔ ZK_Prover", liquidity: "840k ATC", stable: true },
                    { pair: "Inference_Node ↔ Prompt_Swarms", liquidity: "2.1M ATC", stable: true },
                  ].map((market, idx) => (
                    <div key={idx} className="flex items-center justify-between p-3 rounded bg-[#090b14] border border-white/5">
                      <div className="flex items-center gap-3">
                        <Users className="w-4 h-4 text-slate-500" />
                        <span className="font-mono text-sm text-white">{market.pair}</span>
                      </div>
                      <div className="flex items-center gap-4">
                         <span className="text-sm text-slate-400">Liq: {market.liquidity}</span>
                         <span className={`text-[10px] uppercase tracking-widest px-2 py-0.5 rounded font-bold ${market.stable ? 'bg-emerald-500/20 text-emerald-400' : 'bg-red-500/20 text-red-400'}`}>
                           {market.stable ? 'Nash Eq' : 'Imbalanced'}
                         </span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}
