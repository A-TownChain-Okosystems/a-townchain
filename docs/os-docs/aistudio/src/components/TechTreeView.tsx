import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  Network, Shield, Cpu, Database, Blocks, 
  Lock, Zap, Activity, Coins, Fingerprint, 
  FolderKey, Globe2, Layers, Binary, Box,
  ChevronRight, ChevronDown, Workflow, Bookmark
} from 'lucide-react';
import { TooltipIcon } from './TooltipIcon';

type NodeStatus = 'unlocked' | 'researching' | 'locked';

interface TechNode {
  id: string;
  title: string;
  description: string;
  status: NodeStatus;
  icon: React.ElementType;
  progress?: number;
  dependencies: string[];
  unlocks: string[];
  metrics?: { label: string; value: string }[];
  modules?: { name: string; type: string; status: 'active' | 'pending' | 'offline' }[];
}

interface TechTier {
  level: number;
  title: string;
  nodes: TechNode[];
}

export const TECH_TREE_DATA: TechTier[] = [
  {
    level: 1,
    title: 'Tier I: Foundation',
    nodes: [
      {
        id: 'pqc',
        title: 'Post-Quantum Crypto',
        description: 'Implementierung quantensicherer Signaturalgorithmen (Kyber/Dilithium) für zukunftssichere Transaktionen.',
        status: 'unlocked',
        icon: Shield,
        dependencies: [],
        unlocks: ['did_auth'],
        metrics: [
          { label: 'Security Level', value: 'NIST Grade 5' },
          { label: 'Key Size', value: '2.5 KB' }
        ],
        modules: [
          { name: 'Kyber768KeyGen', type: 'crypto/keygen', status: 'active' },
          { name: 'DilithiumVerify', type: 'crypto/sig', status: 'active' },
        ]
      },
      {
        id: 'p2p_gossip',
        title: 'Gossip Network v2',
        description: 'Optimiertes Peer-to-Peer Routing mit reduzierter Latenz und verbessertem NAT-Traversal.',
        status: 'unlocked',
        icon: Network,
        dependencies: [],
        unlocks: ['sharding_base'],
        metrics: [
          { label: 'Latency', value: '< 50ms' },
          { label: 'Throughput', value: '10k TPS' }
        ],
        modules: [
          { name: 'KademliaDHT', type: 'network/routing', status: 'active' },
          { name: 'NATResolver', type: 'network/stun', status: 'pending' }
        ]
      },
      {
        id: 'core_consensus',
        title: 'ATC Consensus',
        description: 'Basis-Konsens-Engine basierend auf einem modifizierten BFT-Proof-of-Stake Algorithmus.',
        status: 'unlocked',
        icon: Activity,
        dependencies: [],
        unlocks: ['atc_vm', 'sharding_base'],
        metrics: [
          { label: 'Finality', value: 'Achieved' },
          { label: 'Validators', value: '100+' }
        ]
      }
    ]
  },
  {
    level: 2,
    title: 'Tier II: Execution & Data',
    nodes: [
      {
        id: 'atc_vm',
        title: 'ATC VM (GVM)',
        description: 'Turing-vollständige Ausführungsumgebung für Smart Contracts mit nativer WebAssembly Unterstützung.',
        status: 'unlocked',
        icon: Cpu,
        progress: 100,
        dependencies: ['core_consensus'],
        unlocks: ['defi_primitives', 'dao_gov'],
        metrics: [
          { label: 'Runtime', value: 'ATVM' },
          { label: 'Gas Limit', value: 'Dynamic' }
        ]
      },
      {
        id: 'zk_proofs',
        title: 'Zero-Knowledge Circuits',
        description: 'zk-SNARK Integration für private Transaktionen und skalierbare state rollups.',
        status: 'unlocked',
        icon: Binary,
        progress: 100,
        dependencies: ['pqc'],
        unlocks: ['private_tx'],
        metrics: [
          { label: 'Prover Time', value: '~2s' },
          { label: 'Batch Size', value: '1000 tx' }
        ]
      },
      {
        id: 'dist_storage',
        title: 'Decentralized Storage',
        description: 'IPFS-kompatibles, verteiltes Dateisystem mit integrierter Datenverfügbarkeits-Garantie (Data Availability).',
        status: 'unlocked',
        icon: Database,
        progress: 100,
        dependencies: ['p2p_gossip'],
        unlocks: ['nft_standards'],
        metrics: [
          { label: 'Replication', value: 'Target 5x' }
        ]
      }
    ]
  },
  {
    level: 3,
    title: 'Tier III: Scalability & Ecosystem',
    nodes: [
      {
        id: 'sharding_base',
        title: 'Dynamic State Sharding',
        description: 'Automatische Aufteilung des State-Trees in parallele Shards zur horizontalen Skalierung.',
        status: 'unlocked',
        icon: Layers,
        progress: 100,
        dependencies: ['p2p_gossip', 'core_consensus'],
        unlocks: ['infinite_scale']
      },
      {
        id: 'dao_gov',
        title: 'Governance DAOs',
        description: 'On-chain Governance Module für dezentrale Entscheidungsfindung und Protokoll-Upgrades.',
        status: 'unlocked',
        icon: Globe2,
        progress: 100,
        dependencies: ['atc_vm'],
        unlocks: []
      },
      {
        id: 'cross_chain',
        title: 'Cross-Chain Bridges',
        description: 'Trustless Bridges zu anderen Major-Chains (Ethereum, Polkadot, Cosmos).',
        status: 'unlocked',
        icon: Workflow,
        progress: 100,
        dependencies: ['zk_proofs', 'atc_vm'],
        unlocks: []
      }
    ]
  }
];

export function TechTreeView({ addBookmark }: { addBookmark?: (title: string, type: string, path: string) => void }) {
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());

  React.useEffect(() => {
    const handleDeepLink = (e: any) => {
      const id = e.detail.id;
      setExpandedNodes(prev => {
        const next = new Set(prev);
        next.add(id);
        return next;
      });
      setTimeout(() => {
        const el = document.getElementById(`tech-node-${id}`);
        if (el) el.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }, 100);
    };
    window.addEventListener('ATC_DEEP_LINK_TECH', handleDeepLink);
    return () => window.removeEventListener('ATC_DEEP_LINK_TECH', handleDeepLink);
  }, []);

  const toggleNode = (id: string, status: NodeStatus) => {
    if (status === 'locked') return;
    setExpandedNodes(prev => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const getStatusConfig = (status: NodeStatus) => {
    switch (status) {
      case 'unlocked':
        return {
          wrapper: 'border-emerald-500/30 bg-emerald-500/5 hover:border-emerald-500/50 shadow-[0_0_15px_rgba(52,211,153,0.1)] cursor-pointer hover:-translate-y-0.5',
          iconBg: 'bg-emerald-500/10 border-emerald-500/30 text-emerald-400',
          badge: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20',
          label: 'Erforscht'
        };
      case 'researching':
        return {
          wrapper: 'border-atc-cyan/30 bg-atc-cyan/5 hover:border-atc-cyan/60 border-dashed shadow-[0_0_15px_rgba(6,182,212,0.1)] cursor-pointer hover:-translate-y-0.5',
          iconBg: 'bg-atc-cyan/10 border-atc-cyan/30 text-atc-cyan animate-pulse',
          badge: 'bg-atc-cyan/10 text-atc-cyan border-atc-cyan/20',
          label: 'In Entwicklung'
        };
      case 'locked':
      default:
        return {
          wrapper: 'border-atc-border/50 bg-[#060a16] opacity-60 hover:opacity-100 hover:border-slate-500/50 transition-all cursor-not-allowed grayscale hover:grayscale-0',
          iconBg: 'bg-slate-800 border-slate-700 text-slate-500',
          badge: 'bg-slate-800 text-slate-400 border-slate-700',
          label: 'Gesperrt'
        };
    }
  };

  return (
    <div className="flex flex-col gap-6 mt-6 pb-12 w-full max-w-7xl mx-auto h-[800px]">
      <div className="flex items-center gap-4 mb-2">
        <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
          <Workflow className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-1">Technologiebaum <TooltipIcon content="Hierarchical overview of the modular ecosystem architecture. Nodes can be researched to unlock new core layers in the ATC-Lang ecosystem." /></h2>
          <p className="text-sm font-light text-slate-400">
            Forschungs- und Entwicklungsfortschritt des A-TownChain Ökosystems
          </p>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl shadow-2xl relative">
        <div className="absolute top-0 right-1/2 w-96 h-96 bg-indigo-500/5 rounded-full blur-[120px] pointer-events-none" />

        {/* Tree Container */}
        <div className="flex-1 overflow-x-auto overflow-y-auto custom-scrollbar relative z-10 p-8 flex gap-12">
          {TECH_TREE_DATA.map((tier, tIdx) => (
            <div key={tier.level} className="flex flex-col min-w-[320px] w-[320px] relative">
              {/* Connector Line to Next Tier */}
              {tIdx < TECH_TREE_DATA.length - 1 && (
                <div className="absolute top-1/2 right-[-3rem] w-[3rem] h-[2px] bg-atc-border/50 z-0" />
              )}

              <div className="flex items-center justify-between mb-8 pb-4 border-b border-atc-border/50 relative z-10 bg-[#090b14]/80 backdrop-blur px-2 shadow-sm rounded-lg pt-2">
                <h3 className="font-mono font-bold text-white tracking-widest">{tier.title}</h3>
                <span className="text-xs text-slate-500 font-mono">Lvl {tier.level}</span>
              </div>

              <div className="flex flex-col gap-6 relative z-10">
                {tier.nodes.map((node) => {
                  const conf = getStatusConfig(node.status);
                  const Icon = node.status === 'locked' ? Lock : node.icon;
                  const isExpanded = expandedNodes.has(node.id);

                  return (
                    <div 
                      key={node.id}
                      id={`tech-node-${node.id}`}
                      onClick={() => toggleNode(node.id, node.status)}
                      className={`relative flex flex-col p-5 rounded-2xl border transition-all duration-300 ${conf.wrapper} ${isExpanded ? 'ring-2 ring-atc-cyan/50 shadow-xl' : ''}`}
                    >
                      <div className="flex items-start justify-between mb-4">
                        <div className={`p-2.5 rounded-xl border ${conf.iconBg}`}>
                          <Icon className="w-5 h-5" />
                        </div>
                        <div className="flex items-center gap-2">
                          {addBookmark && node.status !== 'locked' && (
                            <button
                              onClick={(e) => {
                                e.stopPropagation();
                                addBookmark(node.title, 'TECH TREE', 'tech_tree');
                              }}
                              className="p-1.5 rounded bg-white/5 border border-white/10 text-slate-400 hover:text-atc-cyan hover:border-atc-cyan/50 hover:bg-atc-cyan/10 transition-colors"
                              title="Bookmark Node"
                            >
                              <Bookmark className="w-3.5 h-3.5" />
                            </button>
                          )}
                          <span className={`px-2.5 py-1 text-[9px] uppercase font-mono font-bold tracking-widest rounded border ${conf.badge}`}>
                            {conf.label}
                          </span>
                        </div>
                      </div>
                      
                      <div className="flex items-center justify-between mb-2">
                        <h4 className="text-lg font-semibold text-slate-200 tracking-tight">{node.title}</h4>
                        {node.status !== 'locked' && (
                          isExpanded ? <ChevronDown className="w-4 h-4 text-slate-500" /> : <ChevronRight className="w-4 h-4 text-slate-500" />
                        )}
                      </div>
                      
                      {node.status === 'researching' && node.progress !== undefined && (
                        <div className="mt-2 mb-2 w-full">
                           <div className="flex justify-between text-[10px] font-mono mb-1 text-slate-400">
                             <span>Progress</span>
                             <span className="text-atc-cyan">{node.progress}%</span>
                           </div>
                           <div className="h-1.5 w-full bg-[#060a16] rounded-full overflow-hidden border border-atc-border/50">
                             <div className="h-full bg-atc-cyan shadow-[0_0_10px_rgba(6,182,212,0.8)]" style={{ width: `${node.progress}%` }} />
                           </div>
                        </div>
                      )}

                      <AnimatePresence>
                        {isExpanded && (
                          <motion.div
                            initial={{ height: 0, opacity: 0, marginTop: 0 }}
                            animate={{ height: 'auto', opacity: 1, marginTop: 16 }}
                            exit={{ height: 0, opacity: 0, marginTop: 0 }}
                            className="overflow-hidden"
                          >
                            <div className="pt-4 border-t border-white/10">
                              <p className="text-slate-400 text-sm leading-relaxed mb-4">
                                {node.description}
                              </p>

                              {node.metrics && node.metrics.length > 0 && (
                                <div className="mb-4">
                                  <h4 className="text-xs font-mono text-slate-500 uppercase tracking-widest mb-2">Spezifikationen</h4>
                                  <div className="flex flex-col gap-2">
                                    {node.metrics.map((m, i) => (
                                      <div key={i} className="flex items-center justify-between p-2 rounded-lg bg-[#090b14] border border-atc-border/50">
                                        <span className="text-xs text-slate-400">{m.label}</span>
                                        <span className="text-xs font-mono text-atc-cyan font-bold">{m.value}</span>
                                      </div>
                                    ))}
                                  </div>
                                </div>
                              )}

                              {node.modules && node.modules.length > 0 && (
                                <div className="mb-4">
                                  <h4 className="text-xs font-mono text-slate-500 uppercase tracking-widest mb-2">API Modules</h4>
                                  <div className="flex flex-col gap-2">
                                    {node.modules.map((mod, i) => (
                                      <div key={i} className="flex flex-col p-2.5 rounded-lg bg-black/40 border border-white/5">
                                        <div className="flex items-center justify-between mb-1">
                                          <span className="text-sm text-slate-200 font-medium">{mod.name}</span>
                                          <div className="flex items-center gap-1.5">
                                            <span className={`w-1.5 h-1.5 rounded-full ${mod.status === 'active' ? 'bg-emerald-500 animate-pulse' : mod.status === 'pending' ? 'bg-amber-500' : 'bg-slate-600'}`} />
                                            <span className="text-[10px] font-mono uppercase text-slate-400">{mod.status}</span>
                                          </div>
                                        </div>
                                        <span className="text-xs font-mono text-indigo-400">{mod.type}</span>
                                      </div>
                                    ))}
                                  </div>
                                </div>
                              )}

                              {node.dependencies.length > 0 && (
                                <div className="mb-4">
                                  <h4 className="text-xs font-mono text-slate-500 uppercase tracking-widest mb-2">Voraussetzungen</h4>
                                  <div className="flex flex-wrap gap-2">
                                    {node.dependencies.map((dep, i) => {
                                      let depName = dep;
                                      for(let tier of TECH_TREE_DATA) {
                                        const n = tier.nodes.find(n => n.id === dep);
                                        if(n) depName = n.title;
                                      }
                                      return (
                                        <div key={i} className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-atc-purple/5 border border-atc-purple/20">
                                          <Lock className="w-3 h-3 text-atc-purple" />
                                          <span className="text-[10px] font-medium text-slate-300">{depName}</span>
                                        </div>
                                      );
                                    })}
                                  </div>
                                </div>
                              )}

                              {node.unlocks.length > 0 && (
                                <div className="mb-2">
                                  <h4 className="text-xs font-mono text-slate-500 uppercase tracking-widest mb-2">Schaltet frei</h4>
                                  <div className="flex flex-col gap-1.5">
                                    {node.unlocks.map((unl, i) => {
                                      let unlName = unl;
                                      for(let tier of TECH_TREE_DATA) {
                                        const n = tier.nodes.find(n => n.id === unl);
                                        if(n) unlName = n.title;
                                      }
                                      return (
                                        <div key={i} className="flex items-center gap-2 p-2 rounded-md bg-[#090b14] border border-emerald-500/20 text-emerald-400">
                                           <ChevronRight className="w-3 h-3" />
                                           <span className="text-xs font-medium">{unlName}</span>
                                        </div>
                                      );
                                    })}
                                  </div>
                                </div>
                              )}

                            </div>
                          </motion.div>
                        )}
                      </AnimatePresence>
                    </div>
                  );
                })}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
