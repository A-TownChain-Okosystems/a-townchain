import React, { useState } from 'react';
import { Ghost, Sparkles, Dna, BarChart3, Database, Search, ShieldAlert, Cpu } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

// Mock data
const mockCollection = [
  { id: 'SHV-001', name: 'Phantom-X', element: 'Void', level: 42, rarity: 'Epic', image: '👻' },
  { id: 'SHV-089', name: 'Cyber-Drake', element: 'Tech', level: 18, rarity: 'Rare', image: '🐉' },
  { id: 'SHV-442', name: 'Neon-Gale', element: 'Wind', level: 5, rarity: 'Common', image: '🦅' },
];

export function ATCAssetView() {
  const [activeTab, setActiveTab] = useState<'collection' | 'mint' | 'breed' | 'stats'>('collection');
  const [collection, setCollection] = useState<typeof mockCollection | null>(null);
  const [loadingCode, setLoadingCode] = useState(false);

  const loadCollection = () => {
    setLoadingCode(true);
    setTimeout(() => {
      setCollection(mockCollection);
      setLoadingCode(false);
    }, 800);
  };

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200">
      {/* Header */}
      <div className="flex items-center justify-between px-6 py-4 bg-[#090b14] border-b border-atc-border/50 shadow-sm">
         <div className="flex items-center gap-3">
           <div className="p-2 rounded-lg bg-pink-500/10 border border-pink-500/30">
              <Ghost className="w-6 h-6 text-pink-400" />
           </div>
           <div>
              <h2 className="text-xl font-bold text-white tracking-tight">ATCAsset Engine</h2>
              <p className="text-xs text-slate-400 font-mono">ATC-9000 NFT Standard • Battle Data Logic</p>
           </div>
         </div>
      </div>

      <div className="flex flex-col flex-1 p-6 overflow-hidden">
         {/* Navigation Tabs */}
         <div className="flex gap-2 mb-6 border-b border-white/5 pb-4">
            <button 
              onClick={() => setActiveTab('collection')}
              className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${activeTab === 'collection' ? 'bg-pink-500/20 text-pink-400 border border-pink-500/30' : 'bg-[#090b14] text-slate-400 hover:text-white border border-white/5'}`}
            >
              <Database className="w-4 h-4" /> Collection
            </button>
            <button 
              onClick={() => setActiveTab('mint')}
              className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${activeTab === 'mint' ? 'bg-pink-500/20 text-pink-400 border border-pink-500/30' : 'bg-[#090b14] text-slate-400 hover:text-white border border-white/5'}`}
            >
              <Sparkles className="w-4 h-4" /> Minting
            </button>
            <button 
              onClick={() => setActiveTab('breed')}
              className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${activeTab === 'breed' ? 'bg-pink-500/20 text-pink-400 border border-pink-500/30' : 'bg-[#090b14] text-slate-400 hover:text-white border border-white/5'}`}
            >
              <Dna className="w-4 h-4" /> Evolution Lab
            </button>
            <button 
              onClick={() => setActiveTab('stats')}
              className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${activeTab === 'stats' ? 'bg-pink-500/20 text-pink-400 border border-pink-500/30' : 'bg-[#090b14] text-slate-400 hover:text-white border border-white/5'}`}
            >
              <BarChart3 className="w-4 h-4" /> Network Limit Stats
            </button>
         </div>

         <div className="flex-1 overflow-y-auto custom-scrollbar">
            {activeTab === 'collection' && (
              <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="flex flex-col gap-6 h-full">
                 <div className="flex gap-4">
                    <div className="flex-1 relative">
                       <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" />
                       <input 
                         type="text" 
                         placeholder="Wallet-Addresse scannen..." 
                         className="w-full bg-[#090b14] border border-white/10 rounded-lg py-2.5 pl-10 pr-4 text-sm font-mono focus:outline-none focus:border-pink-500/50"
                       />
                    </div>
                    <button 
                      onClick={loadCollection}
                      className="px-6 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg font-bold text-sm transition-colors"
                    >
                       LOAD
                    </button>
                 </div>

                 <div className="flex-1 border border-white/5 rounded-2xl bg-[#090b14]/50 p-6">
                    {!collection ? (
                       <div className="h-full flex flex-col items-center justify-center text-slate-500 gap-4">
                          {loadingCode ? (
                             <Cpu className="w-12 h-12 animate-pulse text-pink-400/50" />
                          ) : (
                             <Database className="w-12 h-12 opacity-30" />
                          )}
                          <p className="text-sm font-mono">{loadingCode ? 'Wird vom Smart Contract abgerufen...' : 'Keine Wallet geladen.'}</p>
                       </div>
                    ) : (
                       <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                          {collection.map(shv => (
                             <motion.div 
                               initial={{ scale: 0.9, opacity: 0 }} 
                               animate={{ scale: 1, opacity: 1 }} 
                               key={shv.id} 
                               className="bg-black/40 border border-white/10 rounded-xl p-4 flex flex-col items-center text-center hover:border-pink-500/50 hover:bg-pink-500/5 cursor-pointer transition-all group"
                             >
                                <div className="text-5xl mb-3 mt-2 group-hover:scale-110 transition-transform duration-300">
                                   {shv.image}
                                </div>
                                <div className="font-bold text-white mb-1 tracking-tight">{shv.name}</div>
                                <div className="font-mono text-[10px] text-pink-400 bg-pink-500/10 px-2 py-0.5 rounded border border-pink-500/20 mb-3">{shv.id}</div>
                                
                                <div className="flex items-center justify-center gap-4 w-full pt-3 border-t border-white/5">
                                   <div className="text-left">
                                      <div className="text-[9px] uppercase tracking-wider text-slate-500">Level</div>
                                      <div className="text-sm font-bold text-slate-300">{shv.level}</div>
                                   </div>
                                   <div className="text-right">
                                      <div className="text-[9px] uppercase tracking-wider text-slate-500">Rarity</div>
                                      <div className={`text-sm font-bold ${shv.rarity === 'Epic' ? 'text-amber-400' : shv.rarity === 'Rare' ? 'text-blue-400' : 'text-slate-400'}`}>{shv.rarity}</div>
                                   </div>
                                </div>
                             </motion.div>
                          ))}
                       </div>
                    )}
                 </div>
              </motion.div>
            )}

            {activeTab === 'mint' && (
              <motion.div initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} className="max-w-xl mx-auto flex flex-col gap-6">
                 <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6">
                    <h3 className="text-lg font-bold text-white mb-4">Neues ATCAsset prägen</h3>
                    <p className="text-sm text-slate-400 mb-6">Jeder Mintvorgang ist algorithmisch gesichert. Das System bestimmt Element, Name und Rarity basierend auf dem Blockchain-Hash.</p>

                    <div className="flex flex-col gap-4">
                       <div>
                         <label className="block text-xs font-medium text-slate-400 mb-1.5">Besitzer-Adresse</label>
                         <input type="text" placeholder="ATC-..." className="w-full bg-black/40 border border-atc-border/50 rounded-lg px-4 py-2.5 text-sm text-white focus:outline-none focus:border-pink-500 transition-colors font-mono" />
                       </div>
                       
                       <div className="flex gap-4">
                          <button className="flex-1 py-3 rounded-xl border border-pink-500/30 bg-pink-500/10 text-pink-400 font-bold hover:bg-pink-500/20 transition-colors flex items-center justify-center gap-2">
                            <Sparkles className="w-4 h-4" /> 1 MINTEN (500 ATC)
                          </button>
                       </div>
                    </div>
                 </div>

                 <div className="bg-amber-500/10 border border-amber-500/20 rounded-xl p-4 flex items-start gap-3">
                    <ShieldAlert className="w-5 h-5 text-amber-500 shrink-0 mt-0.5" />
                    <div className="text-sm text-amber-400/90">
                       Die Genesis-Kollektion ist auf 9.900 Stück limitiert. Aktuell verbleiben noch ca. 3.420 Einheiten im Smart Contract.
                    </div>
                 </div>
              </motion.div>
            )}
            
            {activeTab === 'breed' && (
              <motion.div initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }}>
                 <div className="flex items-center justify-center h-48 border border-white/5 border-dashed rounded-2xl bg-[#090b14]/50 text-slate-500 font-mono text-sm">
                    // Evolution Lab - Modul deaktiviert im ReadOnly Modus
                 </div>
              </motion.div>
            )}
            {activeTab === 'stats' && (
              <motion.div initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }}>
                 <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6 text-center">
                       <div className="text-4xl font-bold text-pink-400 mb-2">6,480</div>
                       <div className="text-xs uppercase tracking-widest text-slate-500">Geminted Total</div>
                    </div>
                    <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6 text-center">
                       <div className="text-4xl font-bold text-white mb-2">9,900</div>
                       <div className="text-xs uppercase tracking-widest text-slate-500">Max Supply Cap</div>
                    </div>
                    <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6 text-center">
                       <div className="text-4xl font-bold text-emerald-400 mb-2">1.2M+</div>
                       <div className="text-xs uppercase tracking-widest text-slate-500">Battle Events logged</div>
                    </div>
                 </div>
              </motion.div>
            )}

         </div>
      </div>
    </div>
  );
}
