import React, { useState, useEffect } from 'react';
import { Package, Search, UploadCloud, Activity, Link, Unlink, Network, Box, Puzzle, RefreshCw, Zap, Cpu, Github, Download } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

type Plugin = {
  id: string;
  name: string;
  author: string;
  version: string;
  status: 'active' | 'inactive' | 'loading' | 'unloading';
  type: 'core' | 'extension' | 'analytics';
  connectedTo?: string[];
  lastUsed?: number;
};

const INITIAL_PLUGINS: Plugin[] = [
  { id: 'wasm-rt', name: 'WASM Compiler', author: 'System', version: '2.1.0', status: 'active', type: 'core', connectedTo: ['nft-bridge'], lastUsed: Date.now() },
  { id: 'p2p-sync', name: 'P2P Net Mesh', author: 'System', version: '1.4.2', status: 'inactive', type: 'core', connectedTo: ['wasm-rt'] },
  { id: 'nft-bridge', name: 'NFT Layer Protocol', author: 'System', version: '1.0.0', status: 'active', type: 'extension', connectedTo: ['wasm-rt'], lastUsed: Date.now() },
  { id: 'oracle-xyz', name: 'Decentralized Oracle', author: 'Community', version: '3.0.0', status: 'inactive', type: 'core', connectedTo: [] },
  { id: 'math-atclib', name: 'Math / Crypto Standard Lib', author: 'System', version: '1.1.0', status: 'active', type: 'extension', connectedTo: ['atvm-rt'], lastUsed: Date.now() },
  { id: 'ai-auditor', name: 'Smart Contract Auditor', author: 'Community', version: '0.9.1', status: 'inactive', type: 'analytics', connectedTo: ['math-atclib'] },
  { id: 'token-faucet', name: 'Testnet Faucet', author: 'DevOps', version: '1.0.1', status: 'inactive', type: 'extension', connectedTo: [] },
];

export function ModulesPluginView() {
  const [plugins, setPlugins] = useState<Plugin[]>(INITIAL_PLUGINS);
  const [search, setSearch] = useState('');
  const [simulating, setSimulating] = useState(false);
  const [registryTab, setRegistryTab] = useState<'local' | 'github'>('local');
  const [isSearchingGithub, setIsSearchingGithub] = useState(false);

  const mockGithubResults = [
    { id: 'gh-zk-rollup', name: 'Zero-Knowledge Rollup Base', author: 'zk-labs', version: '1.2.0', status: 'inactive', type: 'core' },
    { id: 'gh-ipfs-storage', name: 'IPFS Distributed Storage', author: 'ipfs-core', version: '0.8.5', status: 'inactive', type: 'extension' },
    { id: 'gh-defi-dex', name: 'DeFi AMM Exchange', author: 'defi-protocols', version: '2.1.1', status: 'inactive', type: 'analytics' },
    { id: 'gh-cyber-sec', name: 'Smart Contract Firewall', author: 'sec-audits', version: '3.0.0', status: 'inactive', type: 'core' },
    { id: 'gh-bridge-bsc', name: 'BSC Cross-Chain Bridge', author: 'chain-bridges', version: '1.0.4', status: 'inactive', type: 'extension' }
  ];

  const handleGithubSearch = (val: string) => {
    setSearch(val);
    setIsSearchingGithub(true);
    setTimeout(() => setIsSearchingGithub(false), 800);
  };

  // Auto-Unload idle non-used modules (Es ist nur aktiviert was tatsächlich genutzt wird)
  useEffect(() => {
    const interval = setInterval(() => {
        setPlugins(prev => prev.map(p => {
           if (p.status === 'active' && p.lastUsed && (Date.now() - p.lastUsed > 15000)) {
               return { ...p, status: 'unloading' }; // Mark for unload
           }
           if (p.status === 'unloading') {
               return { ...p, status: 'inactive', lastUsed: undefined }; // Fully unloaded
           }
           return p;
        }));
    }, 5000);
    return () => clearInterval(interval);
  }, []);

  const simulateUsage = (id: string) => {
    setPlugins(prev => prev.map(p => {
      if (p.id === id) {
        if (p.status === 'inactive') return { ...p, status: 'loading', lastUsed: Date.now() };
        if (p.status === 'active') return { ...p, lastUsed: Date.now() };
      }
      return p;
    }));

    const plugin = plugins.find(p => p.id === id);
    if (plugin && plugin.status === 'inactive') {
       setTimeout(() => {
         setPlugins(prev => prev.map(p => p.id === id ? { ...p, status: 'active', lastUsed: Date.now() } : p));
       }, 800);
    }
  };

  const simulateWorkload = () => {
     setSimulating(true);
     // Randomly pick a plugin that is inactive and use it
     const inactive = plugins.find(p => p.status === 'inactive');
     if (inactive) simulateUsage(inactive.id);
     
     // Bump usage for some currently active ones so they don't unload
     setPlugins(prev => prev.map(p => p.status === 'active' && Math.random() > 0.5 ? { ...p, lastUsed: Date.now() } : p));
     
     setTimeout(() => setSimulating(false), 1000);
  };

  const activePlugins = plugins.filter(p => ['active', 'loading', 'unloading'].includes(p.status));
  const availablePlugins = plugins.filter(p => p.status === 'inactive' && p.name.toLowerCase().includes(search.toLowerCase()));

  return (
    <div className="flex flex-col h-[calc(100vh-8rem)] w-full max-w-7xl mx-auto px-4 mt-4 pb-6">
      <div className="flex items-center justify-between mb-6 shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-indigo-500/10 border border-indigo-500/20 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
            <Network className="w-5 h-5" />
          </div>
          <div>
            <h2 className="text-2xl font-bold tracking-tight text-white">Dynamic Runtime Mesh</h2>
            <p className="text-sm font-light text-slate-400">Flache Plugin-Architektur. Nur aktiviert, was aktuell genutzt wird.</p>
          </div>
        </div>
        <div className="flex items-center gap-4 relative z-10">
           <div className="hidden sm:flex bg-black/40 border border-white/5 rounded-xl p-1 relative z-10">
              <div className="px-4 py-1.5 flex items-center gap-2 text-sm font-medium text-white bg-indigo-500/20 rounded-lg border border-indigo-500/30">
                 <Network className="w-4 h-4" /> Active Mesh
              </div>
              <div className="px-4 py-1.5 flex items-center gap-2 text-sm font-medium text-slate-400">
                 <Package className="w-4 h-4" /> Registry
              </div>
           </div>
           <button 
             onClick={simulateWorkload} 
             disabled={simulating}
             className="px-4 py-2 bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 rounded-xl hover:bg-emerald-500/20 transition-colors font-bold text-sm flex items-center gap-2"
           >
              <Cpu className={`w-4 h-4 ${simulating ? 'animate-pulse' : ''}`} /> Simulate Workload
           </button>
        </div>
      </div>

      <div className="flex flex-col md:flex-row gap-6 flex-1 min-h-0">
        {/* Dynamic Canvas Area (Active Plugins) */}
        <div className="flex-1 bg-[#090b14]/80 backdrop-blur-md border border-indigo-500/20 rounded-2xl relative overflow-hidden flex flex-col">
          <div className="absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(99,102,241,0.05)_0%,transparent_70%)] pointer-events-none" />
          <div className="absolute inset-0 bg-[linear-gradient(to_right,#ffffff05_1px,transparent_1px),linear-gradient(to_bottom,#ffffff05_1px,transparent_1px)] bg-[size:4rem_4rem] pointer-events-none" />
          
          <div className="p-4 border-b border-white/5 flex items-center justify-between bg-black/20 backdrop-blur-xl relative z-10 shrink-0">
            <div className="flex items-center gap-3">
               <Zap className="w-4 h-4 text-amber-400" />
               <span className="text-sm font-bold text-slate-200">On-Demand Loaded Modules</span>
            </div>
            <div className="text-xs text-slate-500 font-mono flex items-center gap-3">
               <span className="px-2 py-0.5 rounded-full bg-amber-500/10 border border-amber-500/20 text-amber-500">Auto-Unload: Active</span>
               <span>Nodes: <strong className="text-indigo-400">{activePlugins.length}</strong></span>
            </div>
          </div>

          <div className="flex-1 relative p-8 flex items-start content-start overflow-y-auto custom-scrollbar">
             {activePlugins.length === 0 ? (
                <div className="text-slate-500 flex flex-col items-center justify-center gap-3 text-sm font-mono w-full h-full">
                   <Box className="w-8 h-8 opacity-20" />
                   Zero Modules Running. Fully optimized idle state.
                </div>
             ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 w-full max-w-4xl mx-auto h-max relative z-10">
                   <AnimatePresence>
                      {activePlugins.map((plugin) => (
                         <motion.div
                           layout
                           initial={{ opacity: 0, scale: 0.8, y: 20 }}
                           animate={{ opacity: 1, scale: 1, y: 0 }}
                           exit={{ opacity: 0, scale: 0.8 }}
                           key={plugin.id}
                           className={`relative rounded-2xl border p-5 backdrop-blur-xl transition-all shadow-xl ${
                             plugin.status === 'loading' 
                               ? 'border-indigo-500/50 bg-indigo-500/5 shadow-[0_0_20px_rgba(99,102,241,0.2)]'
                               : plugin.status === 'unloading'
                               ? 'border-orange-500/30 bg-orange-500/5 opacity-60 grayscale-[50%]'
                               : 'border-emerald-500/30 bg-emerald-500/5 shadow-[0_0_30px_rgba(16,185,129,0.1)] hover:border-emerald-500/50 hover:shadow-[0_0_40px_rgba(16,185,129,0.2)]'
                           }`}
                         >
                            {plugin.status === 'loading' && (
                               <div className="absolute inset-0 rounded-2xl overflow-hidden pointer-events-none">
                                  <div className="absolute inset-0 bg-gradient-to-r from-transparent via-white/5 to-transparent skew-x-12 -translate-x-[150%] animate-[shimmer_1.5s_infinite]" />
                               </div>
                            )}
                            
                            <div className="flex justify-between items-start mb-4">
                               <div className="w-10 h-10 rounded-xl bg-black/40 border border-white/10 flex items-center justify-center shrink-0">
                                 {plugin.status === 'loading' ? (
                                    <RefreshCw className="w-5 h-5 text-indigo-400 animate-spin" />
                                 ) : plugin.status === 'unloading' ? (
                                    <RefreshCw className="w-5 h-5 text-orange-400 animate-spin" />
                                 ) : plugin.type === 'core' ? (
                                    <Box className="w-5 h-5 text-indigo-400" />
                                 ) : (
                                    <Puzzle className="w-5 h-5 text-emerald-400" />
                                 )}
                               </div>
                               <button 
                                 onClick={() => simulateUsage(plugin.id)}
                                 className="w-8 h-8 rounded-lg hover:bg-white/10 flex items-center justify-center text-slate-400 hover:text-white transition-colors"
                                 title="Simulate usage (keep alive)"
                               >
                                 <Zap className="w-4 h-4" />
                               </button>
                            </div>
                            
                            <div className="space-y-1 relative z-10">
                               <h4 className="text-white font-bold truncate">{plugin.name}</h4>
                               <div className="flex items-center gap-2 text-xs">
                                 <span className="text-slate-400">{plugin.author}</span>
                                 <span className="w-1 h-1 rounded-full bg-white/20" />
                                 <span className="text-indigo-400 font-mono">v{plugin.version}</span>
                               </div>
                            </div>
                            
                            <div className="mt-4 pt-4 border-t border-white/5 flex items-center justify-between text-[10px] font-mono uppercase tracking-widest relative z-10">
                               <span className={plugin.type === 'core' ? 'text-indigo-400' : 'text-emerald-400'}>{plugin.type}</span>
                               <span className={
                                  plugin.status === 'loading' ? 'text-indigo-400 animate-pulse' : 
                                  plugin.status === 'unloading' ? 'text-orange-400' : 
                                  'text-emerald-400 font-bold'
                               }>
                                 {plugin.status === 'loading' ? 'Dynamic Inject...' : plugin.status === 'unloading' ? 'Graceful Unload' : 'Active (In-Use)'}
                               </span>
                            </div>
                         </motion.div>
                      ))}
                   </AnimatePresence>
                </div>
             )}
          </div>
        </div>

        {/* Plugin Registry Sidebar */}
        <div className="w-full md:w-80 bg-[#090b14]/90 backdrop-blur-xl border border-white/10 rounded-2xl flex flex-col shrink-0 overflow-hidden shadow-2xl">
           <div className="p-4 border-b border-white/5 bg-black/20">
              <div className="flex items-center gap-2 mb-4 bg-black/40 p-1.5 rounded-xl border border-white/5">
                 <button 
                   onClick={() => { setRegistryTab('local'); setSearch(''); }}
                   className={`flex-1 py-1.5 rounded-lg text-xs font-bold transition-colors flex items-center justify-center gap-1.5 ${registryTab === 'local' ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-500/30' : 'text-slate-400 hover:text-white'}`}
                 >
                    <Package className="w-3.5 h-3.5" /> Registry
                 </button>
                 <button 
                   onClick={() => { setRegistryTab('github'); setSearch(''); }}
                   className={`flex-1 py-1.5 rounded-lg text-xs font-bold transition-colors flex items-center justify-center gap-1.5 ${registryTab === 'github' ? 'bg-white/10 text-white border border-white/20' : 'text-slate-400 hover:text-white'}`}
                 >
                    <Github className="w-3.5 h-3.5" /> GitHub
                 </button>
              </div>
              <div className="relative">
                 <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
                 <input 
                   type="text" 
                   value={search}
                   onChange={e => registryTab === 'github' ? handleGithubSearch(e.target.value) : setSearch(e.target.value)}
                   placeholder={registryTab === 'github' ? "Search GitHub repos..." : "Search registry..."}
                   className="w-full bg-black/40 border border-white/10 rounded-xl pl-9 pr-4 py-2 text-sm text-white focus:outline-none focus:border-indigo-500/50 transition-colors placeholder:text-slate-600"
                 />
              </div>
           </div>
           
           <div className="flex-1 overflow-y-auto p-4 space-y-3 custom-scrollbar relative">
              {registryTab === 'github' && isSearchingGithub && (
                 <div className="absolute inset-0 bg-[#090b14]/80 backdrop-blur-sm z-10 flex flex-col items-center justify-center text-indigo-400 text-xs font-mono">
                    <RefreshCw className="w-5 h-5 animate-spin mb-2" /> Searching GitHub API...
                 </div>
              )}
              <AnimatePresence>
                 {(registryTab === 'local' ? availablePlugins : mockGithubResults.filter(p => p.name.toLowerCase().includes(search.toLowerCase()))).map(plugin => (
                   <motion.div
                     layout
                     initial={{ opacity: 0, scale: 0.9 }}
                     animate={{ opacity: 1, scale: 1 }}
                     exit={{ opacity: 0, scale: 0.9 }}
                     key={plugin.id}
                     className="p-3 rounded-xl border border-white/5 bg-white/5 hover:bg-white/10 transition-colors group cursor-pointer"
                     onClick={() => {
                        if (registryTab === 'local') {
                           simulateUsage(plugin.id);
                        } else {
                           if (!plugins.find(p => p.id === plugin.id)) {
                             setPlugins(prev => [...prev, { ...plugin, status: 'inactive' } as Plugin]);
                             setSearch('');
                             setRegistryTab('local');
                           }
                        }
                     }}
                   >
                     <div className="flex justify-between items-start mb-2">
                        <div className="flex items-center gap-2">
                           {plugin.type === 'core' ? <Box className="w-4 h-4 text-indigo-400/70" /> : <Puzzle className="w-4 h-4 text-emerald-400/70" />}
                           <h4 className="text-sm font-bold text-slate-200">{plugin.name}</h4>
                        </div>
                        <div className={`w-6 h-6 rounded-lg flex items-center justify-center transition-opacity ${registryTab === 'github' ? 'bg-indigo-500/10 text-indigo-400 opacity-100' : 'bg-emerald-500/10 text-emerald-400 opacity-0 group-hover:opacity-100'}`}>
                           {registryTab === 'github' ? <Download className="w-3 h-3" /> : <Zap className="w-3 h-3" />}
                        </div>
                     </div>
                     <div className="flex items-center gap-2 text-[10px] text-slate-500 font-mono">
                        <span className="flex items-center gap-1">{registryTab === 'github' ? <Github className="w-3 h-3"/> : null} {plugin.author}</span>
                        <span>{plugin.version}</span>
                     </div>
                   </motion.div>
                 ))}
                 {(registryTab === 'local' ? availablePlugins : mockGithubResults.filter(p => p.name.toLowerCase().includes(search.toLowerCase()))).length === 0 && (
                    <div className="text-center text-slate-500 text-xs py-8 px-4 font-mono leading-relaxed">
                       No modules found matching search criteria. 
                    </div>
                 )}
              </AnimatePresence>
           </div>
           
           <div className="p-4 border-t border-white/5 bg-black/20">
              <button className="w-full py-2.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl text-sm font-medium text-slate-300 transition-colors flex items-center justify-center gap-2">
                 <UploadCloud className="w-4 h-4" /> Publish Module
              </button>
           </div>
        </div>
      </div>
    </div>
  );
}
