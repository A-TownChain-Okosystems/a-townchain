import React, { useState, useMemo, useEffect } from 'react';
import { Search, Hash, Clock, Box, Shield, ArrowRightLeft, CheckCircle2 } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

// Generates some mock blocks and txs using "network topology" semantics
const createMockBlock = (i: number, baseHeight: number = 145000) => {
  const height = baseHeight + i;
  const txCount = Math.floor(Math.random() * 50) + 1;
  const hash = '0x' + Array.from({ length: 64 }, () => Math.floor(Math.random() * 16).toString(16)).join('');
  const miner = 'node-' + Math.floor(Math.random() * 40);
  
  return {
    height,
    hash,
    miner,
    txCount,
    timestamp: new Date(),
    size: (Math.random() * 2 + 0.5).toFixed(2) + ' MB',
    gasUsed: Math.floor(Math.random() * 15000000)
  };
};

const initialBlocks = Array.from({ length: 20 }, (_, i) => {
  const b = createMockBlock(20 - i);
  b.timestamp = new Date(Date.now() - i * 15000);
  return b;
});

const mockTransactions = Array.from({ length: 15 }, (_, i) => ({
  hash: '0x' + Array.from({ length: 64 }, () => Math.floor(Math.random() * 16).toString(16)).join(''),
  block: 145050 - Math.floor(Math.random() * 5),
  from: '0x' + Array.from({ length: 40 }, () => Math.floor(Math.random() * 16).toString(16)).join(''),
  to: '0x' + Array.from({ length: 40 }, () => Math.floor(Math.random() * 16).toString(16)).join(''),
  value: (Math.random() * 100).toFixed(4),
  timestamp: new Date(Date.now() - Math.random() * 60000),
}));

export function BlockchainLedgerView() {
  const [search, setSearch] = useState("");
  const [blocks, setBlocks] = useState(initialBlocks);
  const [currentHeight, setCurrentHeight] = useState(145020);

  const [activeTab, setActiveTab] = useState<'blocks' | 'mempool'>('blocks');

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentHeight(h => {
        const nextHeight = h + 1;
        const newBlock = createMockBlock(0, nextHeight);
        setBlocks(prev => [newBlock, ...prev].slice(0, 50));
        return nextHeight;
      });
    }, 5000);
    return () => clearInterval(interval);
  }, []);
  
  const filteredBlocks = useMemo(() => {
    return blocks.filter(b => b.hash.includes(search.toLowerCase()) || b.height.toString().includes(search));
  }, [search, blocks]);

  return (
    <div className="flex bg-[#050811] text-slate-300 h-full w-full font-mono">
      <div className="flex-1 flex flex-col h-full overflow-hidden">
        
        <div className="p-6 border-b border-white/10 bg-white/5 shrink-0">
          <h2 className="text-xl font-bold text-white flex items-center gap-2 tracking-tight">
            <Box className="w-6 h-6 text-indigo-400" />
            Blockchain Ledger
          </h2>
          <p className="text-sm text-slate-400 mt-1">Real-time block exploration based on active topology</p>
          
          <div className="mt-4 flex items-center justify-between">
            <div className="relative w-full max-w-sm">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
              <input 
                type="text"
                value={search}
                onChange={e => setSearch(e.target.value)}
                placeholder="Search by block hash or height..."
                className="w-full bg-black/40 border border-white/10 rounded-lg pl-10 pr-4 py-2 text-sm outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all placeholder:text-slate-600"
              />
            </div>
            
            <div className="flex gap-2 bg-black/40 p-1 rounded-lg border border-white/5">
              <button 
                onClick={() => setActiveTab('blocks')}
                className={`px-4 py-1.5 rounded-md text-sm font-bold transition-all ${activeTab === 'blocks' ? 'bg-indigo-500/20 text-indigo-400' : 'text-slate-400 hover:text-white'}`}
              >
                Blocks
              </button>
              <button 
                onClick={() => setActiveTab('mempool')}
                className={`px-4 py-1.5 rounded-md text-sm font-bold transition-all flex items-center gap-2 ${activeTab === 'mempool' ? 'bg-rose-500/20 text-rose-400' : 'text-slate-400 hover:text-white'}`}
              >
                 Mempool <span className="w-2 h-2 rounded-full bg-rose-500 animate-pulse"></span>
              </button>
            </div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto p-6 flex flex-col lg:flex-row gap-6 custom-scrollbar">
          
          {activeTab === 'blocks' ? (
            <>
              <div className="flex-1 flex flex-col gap-4">
            <h3 className="text-sm font-bold text-white flex items-center gap-2">
              <Box className="w-4 h-4 text-emerald-400" />
              Recent Blocks
            </h3>
            
            <div className="flex flex-col gap-2">
              <AnimatePresence initial={false}>
              {filteredBlocks.slice(0, 20).map(block => (
                <motion.div 
                  layout
                  initial={{ opacity: 0, y: -20, scale: 0.95 }}
                  animate={{ opacity: 1, y: 0, scale: 1 }}
                  key={block.hash} 
                  className="bg-white/5 border border-white/5 rounded-xl p-4 hover:bg-white/10 transition-colors flex flex-col sm:flex-row sm:items-center justify-between gap-4"
                >
                  <div className="flex items-center gap-4">
                    <div className="w-10 h-10 rounded bg-indigo-500/20 flex flex-col items-center justify-center border border-indigo-500/30 shrink-0">
                      <span className="text-[9px] text-indigo-400 font-bold">BK</span>
                      <span className="text-xs text-white font-bold">{block.height % 1000}</span>
                    </div>
                    <div className="flex flex-col gap-1">
                      <span className="text-sm font-bold text-white cursor-pointer hover:text-indigo-300">
                        {block.height}
                      </span>
                      <span className="text-xs text-slate-500 truncate max-w-[150px] sm:max-w-[200px]" title={block.hash}>
                        {block.hash}
                      </span>
                    </div>
                  </div>
                  
                  <div className="flex items-center gap-6 text-sm">
                    <div className="flex flex-col items-start sm:items-end gap-1">
                      <span className="text-xs text-slate-400 flex items-center gap-1">
                        <ArrowRightLeft className="w-3 h-3" /> {block.txCount} txns
                      </span>
                      <span className="text-[10px] text-emerald-500 font-bold bg-emerald-500/10 px-1.5 py-0.5 rounded flex items-center gap-1">
                        <Shield className="w-2.5 h-2.5" /> {block.miner}
                      </span>
                    </div>
                    <div className="flex flex-col items-end gap-1 text-right min-w-[80px]">
                      <span className="text-xs text-slate-300">{block.size}</span>
                      <span className="text-[10px] text-slate-500">{Math.floor((Date.now() - block.timestamp.getTime()) / 1000)}s ago</span>
                    </div>
                  </div>
                </motion.div>
              ))}
              </AnimatePresence>
              {filteredBlocks.length === 0 && (
                 <div className="text-center py-12 text-slate-500 text-sm bg-white/5 rounded-xl border border-dashed border-white/10">No blocks found matching query.</div>
              )}
            </div>
          </div>
          
          <div className="w-full lg:w-1/3 flex flex-col gap-4">
            <h3 className="text-sm font-bold text-white flex items-center gap-2">
              <ArrowRightLeft className="w-4 h-4 text-cyan-400" />
              Recent Transactions
            </h3>
            
            <div className="flex flex-col gap-2">
              {mockTransactions.map(tx => (
                <div key={tx.hash} className="bg-white/5 border border-white/5 rounded-xl p-3 flex flex-col gap-2">
                  <div className="flex items-center justify-between">
                     <span className="text-[10px] text-cyan-400 font-bold truncate max-w-[120px]" title={tx.hash}>{tx.hash}</span>
                     <span className="text-[10px] text-slate-500">{Math.floor((Date.now() - tx.timestamp.getTime()) / 1000)}s ago</span>
                  </div>
                  <div className="flex items-center justify-between">
                     <div className="flex flex-col gap-1 text-[10px]">
                        <span className="text-slate-400">From: <span className="text-slate-300 truncate inline-block max-w-[80px] align-bottom">{tx.from}</span></span>
                        <span className="text-slate-400">To: <span className="text-slate-300 truncate inline-block max-w-[80px] align-bottom">{tx.to}</span></span>
                     </div>
                     <div className="flex flex-col items-end">
                        <span className="text-xs font-bold text-white">{tx.value} ATC</span>
                        <CheckCircle2 className="w-3 h-3 text-emerald-500 mt-1" />
                     </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
            </>
          ) : (
             <div className="flex-1 flex flex-col gap-6">
                <div className="p-6 bg-gradient-to-r from-rose-900/20 to-black border border-rose-500/20 rounded-2xl flex flex-col lg:flex-row gap-6 justify-between items-start lg:items-center">
                   <div>
                      <h3 className="text-lg font-bold text-rose-400 flex items-center gap-2 mb-2">
                        <Shield className="w-5 h-5" /> Threat Detection Mempool Engine
                      </h3>
                      <p className="text-sm text-slate-300 max-w-2xl leading-relaxed">
                        The ATC OS uses AI to proactively scan all transactions in the mempool before they are included in a block. MEV actions (Miner Extractable Value) or malicious contract interactions are flagged and sanitized.
                      </p>
                   </div>
                   <div className="flex items-center gap-4 bg-black/40 p-4 rounded-xl border border-white/5">
                      <div className="flex flex-col text-center">
                        <span className="text-[10px] text-slate-500 font-bold uppercase tracking-widest">Pending TX</span>
                        <span className="text-xl font-mono text-white mt-1">1,482</span>
                      </div>
                      <div className="w-px h-8 bg-white/10" />
                      <div className="flex flex-col text-center">
                        <span className="text-[10px] text-slate-500 font-bold uppercase tracking-widest">Threats Found</span>
                        <span className="text-xl font-mono text-rose-400 mt-1">3</span>
                      </div>
                   </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                   {mockTransactions.slice(0, 6).map((tx, idx) => (
                     <div key={idx} className="bg-white/5 border border-white/5 hover:border-white/10 transition-colors rounded-xl p-4 flex flex-col gap-3">
                        <div className="flex justify-between items-start">
                           <div className="flex flex-col gap-1">
                              <span className="text-xs font-mono text-indigo-300">Hash: {tx.hash.substring(0, 20)}...</span>
                              <span className="text-[10px] text-slate-500">{Math.floor((Date.now() - tx.timestamp.getTime()) / 1000)}s ago in mempool</span>
                           </div>
                           {idx % 3 === 0 ? (
                              <span className="px-2 py-1 bg-amber-500/10 border border-amber-500/30 text-amber-500 text-[10px] font-bold uppercase tracking-widest rounded">
                                High MEV Risk
                              </span>
                           ) : (
                              <span className="px-2 py-1 bg-emerald-500/10 border border-emerald-500/30 text-emerald-500 text-[10px] font-bold uppercase tracking-widest rounded">
                                Safe
                              </span>
                           )}
                        </div>
                        <div className="flex items-center justify-between text-sm mt-2">
                           <span className="text-slate-400 text-xs">Value: <span className="text-white font-bold">{tx.value} ATC</span></span>
                           {idx % 3 === 0 && (
                              <button className="text-[10px] text-rose-400 hover:text-rose-300 flex items-center gap-1 font-bold tracking-widest uppercase transition-colors">
                                <Shield className="w-3 h-3" /> Block TX
                              </button>
                           )}
                        </div>
                     </div>
                   ))}
                </div>
             </div>
          )}
          
        </div>
      </div>
    </div>
  );
}
