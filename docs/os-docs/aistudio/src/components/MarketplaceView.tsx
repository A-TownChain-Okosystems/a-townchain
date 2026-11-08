import React, { useState, useMemo } from 'react';
import { ShoppingCart, Search, Package, Star, Download, TrendingUp, Filter, Tag, ShieldCheck, Zap, Grid, Gamepad2, ChevronRight, LayoutTemplate, Briefcase, Plus, X, Server } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { ECOSYSTEM_DATA } from '../ecosystemData.ts';
import { useWallet } from '../contexts/WalletContext.tsx';
import { WINDOWS_MAP } from '../DesktopApp.tsx';
import { MARKETPLACE_EXTRA_APPS } from '../marketplaceApps.ts';

export function MarketplaceView() {
  const { balance, setBalance } = useWallet();
  const [activeTab, setActiveTab] = useState<'home' | 'apps' | 'games' | 'dev'>('home');
  const [searchQuery, setSearchQuery] = useState('');
  
  const [selectedCategory, setSelectedCategory] = useState<string | 'All'>('All');
  const [selectedRarity, setSelectedRarity] = useState<string | 'All'>('All');
  const [selectedElement, setSelectedElement] = useState<string | 'All'>('All');
  
  const [selectedItem, setSelectedItem] = useState<any | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');
  const [batchMode, setBatchMode] = useState(false);
  const [selectedBatch, setSelectedBatch] = useState<any[]>([]);

  const allItems = useMemo(() => {
    const RARITIES = ['Common', 'Rare', 'Epic', 'Genesis'];
    const ELEMENTS = ['NFT', 'Token', 'Skin', 'Plugin', 'Contract'];

    const ecoItems = ECOSYSTEM_DATA.flatMap((cat) => 
      cat.subcategories.flatMap(sub => 
        sub.items.map((item, idx) => ({
          id: `mod-${cat.id}-${sub.title}-${idx}`.replace(/\s+/g, '-'),
          name: item,
          author: 'ATC Certified',
          rating: (Math.random() * 0.5 + 4.5).toFixed(1),
          downloads: Math.floor(Math.random() * 500 + 10) + 'k',
          type: cat.id, 
          rarity: RARITIES[Math.floor(Math.random() * RARITIES.length)],
          element: ELEMENTS[Math.floor(Math.random() * ELEMENTS.length)],
          price: Math.random() > 0.8 ? 'Premium' : 'Free',
          numPrice: Math.floor(Math.random() * 50) + 10,
          category: "System Modules",
          icon: cat.id % 2 === 0 ? Zap : Package,
          verified: true
        }))
      )
    );

    const extraApps = MARKETPLACE_EXTRA_APPS.map(app => ({
       id: app.id,
       name: app.name,
       author: app.author,
       rating: app.rating,
       downloads: app.downloads,
       type: 0,
       rarity: 'Epic',
       element: 'Application',
       price: app.price,
       numPrice: app.numPrice,
       category: app.category,
       icon: app.icon,
       verified: true,
       isRealApp: true,
       component: app.component,
    }));

    return [...extraApps, ...ecoItems];
  }, []);

  const featuredBanners = [
    { title: 'Optimistic Rollup Engine', subtitle: 'Scale your dApps effortlessly', color: 'from-indigo-500/80 to-blue-600/80' },
    { title: 'Zero-Knowledge Toolkit', subtitle: 'Privacy preserving smart contracts', color: 'from-emerald-500/80 to-teal-600/80' }
  ];

  const handlePurchase = () => {
    if (!selectedItem) return;
    setErrorMsg('');
    if (selectedItem.price === 'Premium' && balance < selectedItem.numPrice) {
      setErrorMsg('Insufficient ATC balance.');
      return;
    }

    setIsProcessing(true);
    setTimeout(() => {
      // Deduct balance
      if (selectedItem.price === 'Premium') {
        setBalance(b => b - selectedItem.numPrice);
      }
      
      import('../utils/appSync.tsx').then(({ installApplet }) => {
         installApplet(selectedItem);
         window.dispatchEvent(new Event("atc_app_installed"));
         setIsProcessing(false);
         setSelectedItem(null);
      });
    }, 1500);
  };

  const handleBatchPurchase = () => {
    if (selectedBatch.length === 0) return;
    setErrorMsg('');
    const totalCost = selectedBatch.filter(i => i.price === 'Premium').reduce((sum, i) => sum + i.numPrice, 0);
    if (balance < totalCost) {
      setErrorMsg(`Insufficient ATC balance. You need ${totalCost} ATC.`);
      return;
    }

    setIsProcessing(true);
    setTimeout(() => {
      setBalance(b => b - totalCost);
      import('../utils/appSync.tsx').then(({ installApplets }) => {
         installApplets(selectedBatch);
         window.dispatchEvent(new Event("atc_app_installed"));
         setIsProcessing(false);
         setSelectedBatch([]);
         setBatchMode(false);
      });
    }, 1500);
  };

  const handleItemClick = (item: any) => {
    if (batchMode) {
      if (selectedBatch.find(i => i.id === item.id)) {
        setSelectedBatch(prev => prev.filter(i => i.id !== item.id));
      } else {
        setSelectedBatch(prev => [...prev, item]);
      }
    } else {
      setSelectedItem(item);
    }
  };

  const filteredItems = useMemo(() => {
    return allItems.filter(item => {
      const matchRarity = selectedRarity === 'All' || item.rarity === selectedRarity;
      const matchElement = selectedElement === 'All' || item.element === selectedElement;
      return matchRarity && matchElement;
    });
  }, [allItems, selectedRarity, selectedElement]);

  const appRows = [
    { title: 'Top Free Apps', items: filteredItems.filter(i => i.price === 'Free').slice(0, 8) },
    { title: 'Trending Developer Tools', items: filteredItems.filter(i => i.type === 2 || i.type === 4).slice(0, 8) },
    { title: 'Popular in DeFi', items: filteredItems.slice(8, 16) }
  ];

  return (
    <div className="flex flex-col h-full bg-[#050811] text-white overflow-hidden relative font-sans">
      
      {/* Top Navigation Bar */}
      <div className="flex-none px-6 py-4 border-b border-white/5 bg-[#090b14] flex items-center justify-between z-10 sticky top-0">
        <div className="flex items-center gap-6">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center">
              <ShoppingCart className="w-4 h-4 text-white" />
            </div>
            <span className="font-semibold text-lg tracking-tight">ATC Store</span>
          </div>
          <div className="hidden md:flex items-center gap-1 mx-4">
            {[
              { id: 'home', label: 'Home', icon: Grid },
              { id: 'apps', label: 'Apps', icon: LayoutTemplate },
              { id: 'games', label: 'Gaming', icon: Gamepad2 },
              { id: 'dev', label: 'Developer', icon: Briefcase }
            ].map(tab => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                  activeTab === tab.id ? 'bg-white/10 text-white' : 'text-slate-400 hover:text-white hover:bg-white/5'
                }`}
              >
                <tab.icon className="w-4 h-4" />
                {tab.label}
              </button>
            ))}
          </div>
        </div>

        <div className="flex items-center gap-4">
          <button
            onClick={() => {
              setBatchMode(!batchMode);
              if (batchMode) setSelectedBatch([]);
            }}
            className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors border ${
              batchMode ? 'bg-indigo-500/20 text-indigo-400 border-indigo-500/50' : 'bg-white/5 text-slate-400 border-white/10 hover:text-white'
            }`}
          >
            {batchMode ? 'Cancel Batch' : 'Batch Mode'}
          </button>
          <div className="relative w-full max-w-xs hidden sm:block">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
            <input
              type="text"
              placeholder="Search apps, games, tools..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-black/50 border border-white/10 rounded-full pl-9 pr-4 py-1.5 text-sm focus:outline-none focus:border-indigo-500/50 transition-colors placeholder:text-slate-500"
            />
          </div>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* Filter Sidebar */}
        <div className="hidden lg:flex w-64 flex-col border-r border-white/5 bg-[#090b14] overflow-y-auto p-6 shrink-0 gap-8">
           <div>
              <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3 flex items-center gap-2"><Filter className="w-3.5 h-3.5" /> Rarity</h3>
              <div className="flex flex-col gap-1">
                 {['All', 'Common', 'Rare', 'Epic', 'Genesis'].map(r => (
                   <label key={r} className="flex items-center gap-3 p-2 rounded hover:bg-white/5 cursor-pointer text-sm">
                      <input type="radio" name="rarity" checked={selectedRarity === r} onChange={() => setSelectedRarity(r)} className="accent-indigo-500" />
                      <span className={selectedRarity === r ? 'text-white font-medium' : 'text-slate-400'}>{r}</span>
                   </label>
                 ))}
              </div>
           </div>
           <div>
              <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3 flex items-center gap-2"><Tag className="w-3.5 h-3.5" /> Type / Element</h3>
              <div className="flex flex-col gap-1">
                 {['All', 'NFT', 'Token', 'Skin', 'Plugin', 'Contract'].map(e => (
                   <label key={e} className="flex items-center gap-3 p-2 rounded hover:bg-white/5 cursor-pointer text-sm">
                      <input type="radio" name="element" checked={selectedElement === e} onChange={() => setSelectedElement(e)} className="accent-indigo-500" />
                      <span className={selectedElement === e ? 'text-white font-medium' : 'text-slate-400'}>{e}</span>
                   </label>
                 ))}
              </div>
           </div>
        </div>

        {/* Main Store Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-6 lg:px-12 bg-gradient-to-b from-[#090b14] to-black relative">
          {activeTab === 'home' && !searchQuery && (
            <div className="max-w-7xl mx-auto space-y-12">
              
              {/* Hero Banners */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {featuredBanners.map((banner, i) => (
                  <div key={i} className={`relative overflow-hidden rounded-2xl p-8 h-64 bg-gradient-to-br ${banner.color} flex flex-col justify-end shadow-2xl cursor-pointer group`}>
                    <div className="absolute inset-0 bg-black/20 group-hover:bg-black/10 transition-colors" />
                    <div className="relative z-10">
                      <h2 className="text-3xl font-bold text-white mb-2">{banner.title}</h2>
                      <p className="text-white/80 mb-4">{banner.subtitle}</p>
                      <button className="bg-white text-black px-6 py-2 rounded-full text-sm font-bold shadow-lg hover:scale-105 transition-transform">
                        Explore now
                      </button>
                    </div>
                  </div>
                ))}
              </div>

              {/* Horizontal App Rows */}
              {appRows.map((row, rowIdx) => (
                <div key={rowIdx}>
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-xl font-bold text-white">{row.title}</h3>
                    <button className="text-sm font-medium text-indigo-400 flex items-center hover:underline">See all <ChevronRight className="w-4 h-4" /></button>
                  </div>
                  <div className="flex gap-4 overflow-x-auto pb-4 custom-scrollbar snap-x">
                    {row.items.map(item => {
                      const isSelected = batchMode && !!selectedBatch.find(i => i.id === item.id);
                      return (
                      <motion.div
                        key={item.id}
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => handleItemClick(item)}
                        className={`snap-start min-w-[280px] w-[280px] bg-white/5 border rounded-2xl p-4 flex flex-col gap-4 cursor-pointer transition-all shadow-lg ${isSelected ? 'border-indigo-500 bg-indigo-500/10' : 'border-white/5 hover:bg-white/10'}`}
                      >
                      <div className="flex items-center gap-4">
                        <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-indigo-500/20 to-purple-500/20 shadow-inner flex items-center justify-center shrink-0">
                          <item.icon className="w-8 h-8 text-indigo-300" />
                        </div>
                        <div className="flex flex-col overflow-hidden">
                          <h4 className="font-bold text-white truncate">{item.name}</h4>
                          <span className="text-xs text-slate-400 truncate">{item.author}</span>
                          <div className="flex items-center gap-2 mt-1">
                            <span className="flex items-center gap-0.5 text-xs text-amber-400 font-medium">
                              {item.rating} <Star className="w-3 h-3 fill-current" />
                            </span>
                            <span className="text-[10px] bg-white/10 px-1.5 py-0.5 rounded text-white">{item.price}</span>
                          </div>
                        </div>
                      </div>
                    </motion.div>
                    );})}
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Search Results */}
        {searchQuery && (
          <div className="max-w-7xl mx-auto">
            <h3 className="text-xl font-bold text-white mb-6">Search Results for "{searchQuery}"</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
              {allItems.filter(i => i.name.toLowerCase().includes(searchQuery.toLowerCase()) || i.id.toLowerCase().includes(searchQuery.toLowerCase())).map(item => {
                 const isSelected = batchMode && !!selectedBatch.find(i => i.id === item.id);
                 return (
                 <div key={item.id} onClick={() => handleItemClick(item)} className={`bg-white/5 border rounded-2xl p-4 flex items-center gap-4 cursor-pointer transition-all ${isSelected ? 'border-indigo-500 bg-indigo-500/10' : 'border-white/5 hover:bg-white/10'}`}>
                    <div className="w-12 h-12 rounded-xl bg-indigo-500/20 flex items-center justify-center shrink-0">
                      <item.icon className="w-6 h-6 text-indigo-300" />
                    </div>
                    <div className="flex flex-col overflow-hidden">
                      <h4 className="font-bold text-white text-sm truncate">{item.name}</h4>
                      <span className="text-xs text-slate-400 truncate">{item.author}</span>
                      <span className="text-[10px] text-indigo-400 mt-1 font-medium">{item.price}</span>
                    </div>
                 </div>
              );})}
            </div>
          </div>
        )}

        {activeTab !== 'home' && !searchQuery && (
          <div className="flex flex-col items-center justify-center h-full text-center py-20 opacity-60">
             <LayoutTemplate className="w-16 h-16 text-slate-500 mb-4" />
             <h2 className="text-2xl font-bold text-white mb-2">Coming Soon</h2>
             <p className="text-slate-400 max-w-md">The {activeTab} section is currently being curated. Check back later for more modules.</p>
          </div>
        )}
        </div>
      </div>

      {/* Batch Mode Footer */}
      <AnimatePresence>
        {batchMode && (
          <motion.div
            initial={{ y: 100, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            exit={{ y: 100, opacity: 0 }}
            className="absolute bottom-6 left-1/2 -translate-x-1/2 z-[500] w-full max-w-2xl bg-slate-900 border border-slate-700 shadow-2xl rounded-2xl p-4 flex items-center justify-between"
          >
            <div className="flex flex-col">
              <span className="text-white font-bold">{selectedBatch.length} {selectedBatch.length === 1 ? 'Applet' : 'Applets'} Selected</span>
              <span className="text-sm text-slate-400">
                Total: {selectedBatch.filter(i => i.price === 'Premium').reduce((sum, i) => sum + i.numPrice, 0).toFixed(2)} ATC
              </span>
            </div>
            
            <div className="flex items-center gap-4">
              {errorMsg && batchMode && (
                 <span className="text-red-400 text-sm font-medium">{errorMsg}</span>
              )}
              <button
                onClick={() => {
                  setSelectedBatch([]);
                  setBatchMode(false);
                  setErrorMsg('');
                }}
                className="px-4 py-2 rounded-lg text-sm font-medium bg-white/5 text-slate-300 hover:text-white"
              >
                Cancel
              </button>
              <button
                onClick={handleBatchPurchase}
                disabled={isProcessing || selectedBatch.length === 0}
                className="px-6 py-2 rounded-lg text-sm font-bold bg-indigo-600 hover:bg-indigo-500 text-white shadow-lg disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isProcessing ? 'Installing...' : 'Install All'}
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Item Modal */}
      <AnimatePresence>
        {selectedItem && !batchMode && (
          <div className="fixed inset-0 z-[1000] flex items-center justify-center p-4">
            <motion.div
              initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
              className="absolute inset-0 bg-[#050811]/90 backdrop-blur-md"
              onClick={() => !isProcessing && setSelectedItem(null)}
            />
            <motion.div
              initial={{ scale: 0.95, opacity: 0, y: 20 }}
              animate={{ scale: 1, opacity: 1, y: 0 }}
              exit={{ scale: 0.95, opacity: 0, y: 20 }}
              className="relative w-full max-w-xl bg-gradient-to-br from-[#121626] to-[#0a0d17] border border-white/10 shadow-2xl rounded-3xl overflow-hidden"
            >
              <div className="p-8">
                <button 
                  onClick={() => !isProcessing && setSelectedItem(null)}
                  className="absolute top-6 right-6 p-2 rounded-full hover:bg-white/10 text-slate-400 hover:text-white transition-colors"
                >
                  <X className="w-5 h-5" />
                </button>
                
                <div className="flex gap-6 items-start">
                   <div className="w-24 h-24 shrink-0 rounded-2xl bg-gradient-to-br from-indigo-500/20 to-purple-500/20 flex items-center justify-center border border-indigo-500/30 shadow-inner">
                      <selectedItem.icon className="w-12 h-12 text-indigo-300" />
                   </div>
                   <div className="flex flex-col overflow-hidden">
                      <h2 className="text-2xl font-bold text-white truncate">{selectedItem.name}</h2>
                      <span className="text-slate-500 font-mono text-[10px] mt-0.5 truncate uppercase">{selectedItem.id}</span>
                      <span className="text-indigo-400 font-medium text-sm mt-1">{selectedItem.author}</span>
                      <div className="flex items-center gap-3 mt-3">
                         <span className="flex items-center gap-1 text-sm text-slate-300 bg-white/5 px-2 py-1 rounded">
                            <Star className="w-4 h-4 text-amber-400 fill-current" /> {selectedItem.rating}
                         </span>
                         <span className="flex items-center gap-1 text-sm text-slate-300 bg-white/5 px-2 py-1 rounded">
                            <Download className="w-4 h-4" /> {selectedItem.downloads}
                         </span>
                      </div>
                   </div>
                </div>

                <div className="mt-8 pt-8 border-t border-white/10 flex flex-col gap-4">
                  <div className="grid grid-cols-2 gap-4">
                     <div className="bg-black/30 p-3 rounded-xl border border-white/5">
                        <span className="text-xs text-slate-500 font-bold uppercase tracking-widest">Rarity</span>
                        <div className="text-white font-medium mt-1">{selectedItem.rarity}</div>
                     </div>
                     <div className="bg-black/30 p-3 rounded-xl border border-white/5">
                        <span className="text-xs text-slate-500 font-bold uppercase tracking-widest">Element</span>
                        <div className="text-white font-medium mt-1">{selectedItem.element}</div>
                     </div>
                  </div>
                  
                  {errorMsg && (
                    <div className="p-3 bg-red-500/10 border border-red-500/30 rounded-xl text-red-400 text-sm font-medium">
                      {errorMsg}
                    </div>
                  )}

                  <button 
                    onClick={handlePurchase}
                    disabled={isProcessing}
                    className="w-full mt-4 py-4 rounded-xl font-bold text-lg bg-gradient-to-r from-indigo-500 to-purple-600 hover:from-indigo-400 hover:to-purple-500 text-white shadow-[0_0_20px_rgba(99,102,241,0.4)] hover:shadow-[0_0_30px_rgba(99,102,241,0.6)] transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
                  >
                    {isProcessing ? (
                      <>Processing...</>
                    ) : (
                       selectedItem.price === 'Free' ? `Install (Free)` : `Purchase & Install (${selectedItem.numPrice.toFixed(2)} ATC)`
                    )}
                  </button>
                  <p className="text-center text-xs text-slate-500 mt-2">
                     By installing, you agree to the mock smart contract terms. Current Balance: {balance.toFixed(2)} ATC.
                  </p>
                </div>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
    </div>
  );
}
