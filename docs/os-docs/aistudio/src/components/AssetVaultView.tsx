import React, { useState, useEffect, useCallback, useMemo, useRef } from 'react';
import { Box, RefreshCw, Search, ShieldCheck, Database, Filter, ArrowUpRight } from 'lucide-react';
import { useVirtualizer } from '@tanstack/react-virtual';
import { motion } from 'motion/react';

// Mock asset generator
const generateAssets = (start: number, count: number) => {
  return Array.from({ length: count }).map((_, i) => {
    const id = start + i;
    return {
      id: `AST-${id.toString().padStart(6, '0')}`,
      name: `ATCAsset #${id}`,
      type: ['NFT', 'Token', 'Document', 'License'][Math.floor(Math.random() * 4)],
      rarity: ['Common', 'Rare', 'Epic', 'Legendary'][Math.floor(Math.random() * 4)],
      value: (Math.random() * 1000).toFixed(2),
    };
  });
};

export function AssetVaultView() {
  const [assets, setAssets] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [search, setSearch] = useState('');
  const parentRef = useRef<HTMLDivElement>(null);
  
  // Asynchronous batch fetching simulation
  const fetchBatch = useCallback(async (start: number, count: number) => {
    setIsLoading(true);
    return new Promise<any[]>((resolve) => {
      setTimeout(() => {
        const newAssets = generateAssets(start, count);
        resolve(newAssets);
      }, 600); // simulate network latency
    });
  }, []);

  useEffect(() => {
    // Initial fetch of a large dataset
    const loadInitial = async () => {
      const initialAssets = await fetchBatch(0, 10000); // Loading 10k assets
      setAssets(initialAssets);
      setIsLoading(false);
    };
    loadInitial();
  }, [fetchBatch]);

  const filteredAssets = useMemo(() => {
    if (!search) return assets;
    return assets.filter(a => a.id.toLowerCase().includes(search.toLowerCase()) || a.name.toLowerCase().includes(search.toLowerCase()) || a.type.toLowerCase().includes(search.toLowerCase()));
  }, [assets, search]);

  const gridColumnCount = Math.max(1, Math.floor((window.innerWidth > 1200 ? 1100 : window.innerWidth * 0.8) / 250));
  const gridRowCount = Math.ceil(filteredAssets.length / gridColumnCount);

  const rowVirtualizer = useVirtualizer({
    count: gridRowCount,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 180,
    overscan: 5,
  });

  const getRarityColor = (rarity: string) => {
    switch(rarity) {
       case 'Legendary': return 'text-amber-400 bg-amber-400/10 border-amber-400/30';
       case 'Epic': return 'text-purple-400 bg-purple-400/10 border-purple-400/30';
       case 'Rare': return 'text-blue-400 bg-blue-400/10 border-blue-400/30';
       default: return 'text-slate-400 bg-slate-400/10 border-slate-400/30';
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200 border border-atc-border/50 rounded-xl overflow-hidden font-sans relative">
      <div className="flex items-center justify-between px-4 h-14 bg-[#090b14] border-b border-atc-border/50 shrink-0 relative z-20">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-indigo-500/10 border border-indigo-500/30">
            <Database className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h2 className="font-bold text-white tracking-tight leading-tight">Asset Vault</h2>
            <p className="text-[10px] text-slate-400 font-mono">High-Performance Virtualized Ledger</p>
          </div>
        </div>
        <div className="flex items-center gap-4">
           {isLoading && (
              <div className="flex items-center gap-2 text-xs text-indigo-400 font-mono">
                 <RefreshCw className="w-3 h-3 animate-spin" /> Batch Fetching...
              </div>
           )}
           <div className="flex items-center bg-black/40 border border-white/10 rounded-lg px-3 py-1.5 gap-2 w-64 focus-within:border-indigo-500/50 transition-colors">
             <Search className="w-4 h-4 text-slate-500" />
             <input 
               type="text" 
               placeholder="Search thousands of assets..." 
               value={search}
               onChange={(e) => setSearch(e.target.value)}
               className="bg-transparent border-none outline-none text-sm text-white w-full placeholder:text-slate-600"
             />
           </div>
        </div>
      </div>

      <div className="flex-1 flex flex-col p-4 overflow-hidden relative z-10 w-full">
        <div className="flex items-center justify-between mb-4 px-2">
           <div className="flex items-center gap-4 text-xs font-mono text-slate-400">
             <div className="flex items-center gap-2">
                <ShieldCheck className="w-4 h-4 text-emerald-400" /> 
                <span>Secured by ATC-OS Vault</span>
             </div>
             <span>|</span>
             <span>Total Indexed: <strong className="text-white">{filteredAssets.length.toLocaleString()}</strong> Assets</span>
           </div>
           <div className="flex gap-2">
             <button className="flex items-center gap-2 px-3 py-1.5 bg-white/5 hover:bg-white/10 rounded-lg text-xs font-bold transition-colors border border-white/5 text-slate-300">
               <Filter className="w-3 h-3" /> Filters
             </button>
             <button className="flex items-center gap-2 px-3 py-1.5 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 rounded-lg text-xs font-bold transition-colors border border-indigo-500/20">
               <ArrowUpRight className="w-3 h-3" /> Export
             </button>
           </div>
        </div>

        <div className="flex-1 overflow-auto rounded-xl border border-white/5 bg-[#090b14]/50 relative custom-scrollbar w-full" ref={parentRef}>
           {filteredAssets.length === 0 && !isLoading ? (
             <div className="flex items-center justify-center h-full text-slate-500 text-sm font-mono">
                No assets found matching criteria.
             </div>
           ) : (
              <div
                style={{
                  height: `${rowVirtualizer.getTotalSize()}px`,
                  width: '100%',
                  position: 'relative',
                }}
              >
                {rowVirtualizer.getVirtualItems().map((virtualRow) => (
                  <div
                    key={virtualRow.index}
                    style={{
                      position: 'absolute',
                      top: 0,
                      left: 0,
                      width: '100%',
                      height: `${virtualRow.size}px`,
                      transform: `translateY(${virtualRow.start}px)`,
                      display: 'grid',
                      gridTemplateColumns: `repeat(${gridColumnCount}, minmax(0, 1fr))`,
                      gap: '16px',
                      padding: '8px 16px',
                    }}
                  >
                     {Array.from({ length: gridColumnCount }).map((_, colIndex) => {
                        const cellIndex = virtualRow.index * gridColumnCount + colIndex;
                        if (cellIndex >= filteredAssets.length) return <div key={colIndex} />;
                        
                        const asset = filteredAssets[cellIndex];
                        return (
                          <div key={asset.id} className="h-full bg-black/40 border border-atc-border/50 rounded-xl p-4 flex flex-col justify-between group hover:border-indigo-500/50 transition-all hover:bg-[#0d1120] relative overflow-hidden">
                            <div className="absolute inset-0 bg-indigo-500/5 opacity-0 group-hover:opacity-100 transition-opacity" />
                            
                            <div className="flex justify-between items-start z-10 relative">
                              <div className={`text-[10px] uppercase font-bold px-2 py-0.5 rounded border ${getRarityColor(asset.rarity)}`}>
                                {asset.rarity}
                              </div>
                              <Box className="w-4 h-4 text-slate-500 group-hover:text-indigo-400 transition-colors" />
                            </div>
                            
                            <div className="mt-4 z-10 relative">
                              <div className="text-xs text-slate-500 font-mono mb-1">{asset.id}</div>
                              <div className="text-sm font-bold text-white truncate">{asset.name}</div>
                            </div>
                            
                            <div className="mt-4 flex justify-between items-end border-t border-white/5 pt-3 z-10 relative">
                              <div className="text-xs text-slate-400">{asset.type}</div>
                              <div className="text-sm font-mono text-emerald-400">{asset.value} ATC</div>
                            </div>
                          </div>
                        );
                     })}
                  </div>
                ))}
              </div>
           )}
        </div>
      </div>
    </div>
  );
}
