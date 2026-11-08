import React, { useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Network, Database, Server, Zap, Globe, Lock, Code, Layers, Shield, Cpu, Box, Boxes, Compass, Link as LinkIcon, Users, Activity } from 'lucide-react';

export function BlockchainEcosystemView() {
  const [activeLayer, setActiveLayer] = useState<string>('all');

  const layers = [
    { id: 'all', label: 'Full Ecosystem', icon: Compass },
    { id: 'consensus', label: 'Layer 0 / Consensus', icon: Shield },
    { id: 'execution', label: 'Layer 1 / Execution', icon: Zap },
    { id: 'infrastructure', label: 'Infrastructure & RPC', icon: Server },
    { id: 'application', label: 'dApps & Services', icon: Boxes },
    { id: 'interop', label: 'Bridges & Interop', icon: LinkIcon },
  ];

  const components = [
    {
      id: 'validator_nodes',
      layer: 'consensus',
      name: 'Validator Nodes',
      description: 'Secures the network via staking and consensus algorithms (e.g., PoS, Tendermint, PoH).',
      icon: Shield,
      color: 'text-emerald-400',
      bg: 'bg-emerald-500/10 border-emerald-500/30'
    },
    {
      id: 'consensus_engine',
      layer: 'consensus',
      name: 'Consensus Engine',
      description: 'Handles block proposal, voting, and finality mechanisms.',
      icon: Network,
      color: 'text-emerald-400',
      bg: 'bg-emerald-500/10 border-emerald-500/30'
    },
    {
      id: 'vm',
      layer: 'execution',
      name: 'Virtual Machine / Execution',
      description: 'Processes smart contracts (e.g., EVM, SVM, CosmWasm, eBPF).',
      icon: Cpu,
      color: 'text-atc-cyan',
      bg: 'bg-atc-cyan/10 border-atc-cyan/30'
    },
    {
      id: 'mempool',
      layer: 'execution',
      name: 'Mempool & State',
      description: 'Pending transaction pool and global state trie/tree management.',
      icon: Database,
      color: 'text-atc-cyan',
      bg: 'bg-atc-cyan/10 border-atc-cyan/30'
    },
    {
      id: 'rpc_nodes',
      layer: 'infrastructure',
      name: 'RPC / API Gateways',
      description: 'Provides access interfaces for external apps to communicate with the chain.',
      icon: Server,
      color: 'text-indigo-400',
      bg: 'bg-indigo-500/10 border-indigo-500/30'
    },
    {
      id: 'indexers',
      layer: 'infrastructure',
      name: 'Data Indexers',
      description: 'Indexes chain data for fast querying (e.g., The Graph, Subsquid).',
      icon: Activity,
      color: 'text-indigo-400',
      bg: 'bg-indigo-500/10 border-indigo-500/30'
    },
    {
      id: 'explorers',
      layer: 'infrastructure',
      name: 'Block Explorers',
      description: 'Visual interfaces for transparent on-chain data inspection.',
      icon: Globe,
      color: 'text-indigo-400',
      bg: 'bg-indigo-500/10 border-indigo-500/30'
    },
    {
      id: 'defi',
      layer: 'application',
      name: 'DeFi Protocols',
      description: 'DEXs, Lending, Liquid Staking, and Money Markets.',
      icon: Database,
      color: 'text-atc-purple',
      bg: 'bg-atc-purple/10 border-atc-purple/30'
    },
    {
      id: 'nfts',
      layer: 'application',
      name: 'NFTs & Gaming',
      description: 'Digital assets, marketplaces, and on-chain gaming infrastructure.',
      icon: Box,
      color: 'text-atc-purple',
      bg: 'bg-atc-purple/10 border-atc-purple/30'
    },
    {
      id: 'wallets',
      layer: 'application',
      name: 'Wallets & Identity',
      description: 'Key management, account abstraction, and DID solutions.',
      icon: Lock,
      color: 'text-atc-purple',
      bg: 'bg-atc-purple/10 border-atc-purple/30'
    },
    {
      id: 'bridges',
      layer: 'interop',
      name: 'Cross-Chain Bridges',
      description: 'Lock-and-mint or native transfers to other Layer 1s / Layer 2s.',
      icon: LinkIcon,
      color: 'text-amber-400',
      bg: 'bg-amber-500/10 border-amber-500/30'
    },
    {
      id: 'oracles',
      layer: 'interop',
      name: 'Oracles',
      description: 'Feeds off-chain data (prices, weather) securely into on-chain smart contracts.',
      icon: Layers,
      color: 'text-amber-400',
      bg: 'bg-amber-500/10 border-amber-500/30'
    }
  ];

  const filteredComponents = activeLayer === 'all' 
    ? components 
    : components.filter(c => c.layer === activeLayer);

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 p-6 bg-[#090b14] border-b border-atc-border/50 shadow-sm shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-indigo-500/10 border border-indigo-500/30">
            <Globe className="w-6 h-6 text-indigo-400" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-white tracking-tight">L1 Blockchain Ecosystem</h2>
            <p className="text-xs text-slate-400">Architecture modeled after Ethereum, Solana, and Cosmos</p>
          </div>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* Layer Sidemenu */}
        <div className="w-64 bg-black/20 border-r border-white/5 p-4 flex flex-col gap-2 overflow-y-auto custom-scrollbar shrink-0">
          <div className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-2 px-2">Ecosystem Layers</div>
          {layers.map(layer => (
            <button
              key={layer.id}
              onClick={() => setActiveLayer(layer.id)}
              className={`flex items-center gap-3 px-4 py-3 rounded-xl text-left transition-all ${
                activeLayer === layer.id
                  ? 'bg-atc-cyan/10 border border-atc-cyan/30 text-atc-cyan shadow-[0_0_15px_rgba(6,182,212,0.15)]'
                  : 'border border-transparent text-slate-400 hover:bg-white/5 hover:text-white'
              }`}
            >
              <layer.icon className="w-4 h-4 shrink-0" />
              <span className="text-sm font-medium">{layer.label}</span>
            </button>
          ))}
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
          <div className="max-w-6xl mx-auto">
            <AnimatePresence mode="popLayout">
              <motion.div 
                key={activeLayer}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -20 }}
                className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
              >
                {filteredComponents.map(comp => (
                  <motion.div 
                    layout
                    key={comp.id}
                    className={`p-6 rounded-2xl border bg-black/40 ${comp.bg} backdrop-blur-sm hover:scale-[1.02] transition-transform`}
                  >
                    <div className="flex items-start gap-4 mb-4">
                      <div className={`p-3 rounded-xl bg-black/50 border border-white/10 ${comp.color}`}>
                        <comp.icon className="w-6 h-6" />
                      </div>
                      <div>
                        <h3 className="text-lg font-bold text-white tracking-tight">{comp.name}</h3>
                        <div className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mt-1">
                          Layer: {comp.layer}
                        </div>
                      </div>
                    </div>
                    <p className="text-sm text-slate-300 leading-relaxed">
                      {comp.description}
                    </p>
                  </motion.div>
                ))}
              </motion.div>
            </AnimatePresence>

            {activeLayer === 'all' && (
              <div className="mt-12 p-8 rounded-2xl bg-gradient-to-br from-[#090b14] to-black border border-white/10 relative overflow-hidden">
                <div className="absolute top-0 right-0 w-96 h-96 bg-atc-cyan/5 rounded-full blur-[100px] -translate-y-1/2 translate-x-1/3" />
                <div className="relative z-10 flex flex-col items-center text-center">
                  <Network className="w-12 h-12 text-atc-cyan mb-4" />
                  <h3 className="text-2xl font-bold text-white mb-2">A-Town Native Chain</h3>
                  <p className="text-slate-400 max-w-2xl mx-auto text-sm leading-relaxed mb-6">
                    A comprehensive Layer 1 architecture supporting high-throughput Parallel Execution (Solana-like), Sovereign IBC Interoperability (Cosmos-like), and robust decentralized validation/smart-contract functionality (Ethereum-like).
                  </p>
                  <div className="flex flex-wrap justify-center gap-4">
                    <span className="px-3 py-1 rounded-full border border-atc-cyan/30 text-atc-cyan text-xs font-mono bg-atc-cyan/10">TPS: ~65,000</span>
                    <span className="px-3 py-1 rounded-full border border-atc-purple/30 text-atc-purple text-xs font-mono bg-atc-purple/10">Finality: 400ms</span>
                    <span className="px-3 py-1 rounded-full border border-emerald-500/30 text-emerald-400 text-xs font-mono bg-emerald-500/10">PoS + IBC</span>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
