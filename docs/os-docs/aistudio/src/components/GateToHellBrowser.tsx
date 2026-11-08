import React, { useState } from 'react';
import { Globe, Lock, RefreshCw, ChevronLeft, ChevronRight, Menu, Search, ShieldCheck } from 'lucide-react';
import { motion } from 'motion/react';

export function GateToHellBrowser() {
  const [url, setUrl] = useState('gth://ai-swarm-marketplace.sys');
  const [inputUrl, setInputUrl] = useState(url);
  const [isLoading, setIsLoading] = useState(false);

  const navigate = (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    setIsLoading(true);
    setUrl(inputUrl);
    setTimeout(() => setIsLoading(false), 1200);
  };

  return (
    <div className="flex flex-col h-full bg-[#060a16] rounded-xl overflow-hidden border border-atc-border/50">
      
      {/* Browser Chrome (Top) */}
      <div className="bg-[#090b14] border-b border-white/5 flex flex-col pt-2">
         {/* Tabs */}
         <div className="flex px-2 gap-1 mb-1">
            <div className="flex items-center gap-2 px-3 py-1.5 bg-[#060a16] rounded-t-lg border-t border-x border-atc-border/50 text-sm text-white w-56 relative group">
               <Globe className="w-3.5 h-3.5 text-red-500" />
               <span className="truncate">Web / Web3 / Tor Hub</span>
               <div className="absolute top-0 left-0 w-full h-[1px] bg-red-500" />
            </div>
            <div className="flex items-center gap-2 px-3 py-1.5 bg-transparent hover:bg-white/5 rounded-t-lg text-sm text-slate-400 w-48 transition-colors cursor-pointer">
               <Globe className="w-3.5 h-3.5 text-slate-500" />
               <span className="truncate">New Browser Tab</span>
            </div>
         </div>

         {/* Tollbar */}
         <div className="flex items-center gap-3 px-3 py-2 bg-[#060a16]">
            <div className="flex items-center gap-1">
               <button className="p-1.5 rounded hover:bg-white/10 text-slate-400"><ChevronLeft className="w-5 h-5" /></button>
               <button className="p-1.5 rounded hover:bg-white/10 text-slate-600"><ChevronRight className="w-5 h-5" /></button>
               <button onClick={() => navigate()} className={`p-1.5 rounded hover:bg-white/10 text-slate-400 ${isLoading ? 'animate-spin' : ''}`}><RefreshCw className="w-4 h-4" /></button>
            </div>
            
            {/* Address Bar */}
            <form onSubmit={navigate} className="flex-1 flex items-center bg-[#090b14] border border-white/10 rounded-full px-3 py-1.5 gap-2 focus-within:border-red-500/50 focus-within:shadow-[0_0_10px_rgba(239,68,68,0.2)] transition-all">
               <ShieldCheck className="w-4 h-4 text-emerald-500" />
               <Lock className="w-3 h-3 text-emerald-500" />
                 <input 
                 value={inputUrl}
                 onChange={e => setInputUrl(e.target.value)}
                 className="flex-1 bg-transparent border-none outline-none text-sm text-slate-300 font-mono"
                 placeholder="Search or enter gth://, http://, or .onion address..."
               />
               <Search className="w-4 h-4 text-slate-500" />
            </form>

            <button className="p-1.5 rounded hover:bg-white/10 text-slate-400"><Menu className="w-5 h-5" /></button>
         </div>
      </div>

       {/* Browser Viewport */}
       <div className="flex-1 relative bg-slate-900 overflow-y-auto">
          {isLoading ? (
            <div className="absolute inset-0 flex items-center justify-center bg-slate-900">
               <div className="w-8 h-8 border-2 border-red-500 border-t-transparent rounded-full animate-spin" />
            </div>
          ) : (
             <div className="p-8 max-w-4xl mx-auto font-sans">
                {url.includes('ai-swarm') ? (
                  <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
                     <div className="flex items-center gap-4 mb-4">
                        <div className="w-16 h-16 rounded-2xl bg-red-500/20 flex items-center justify-center text-red-500 border border-red-500/30">
                           <Globe className="w-8 h-8" />
                        </div>
                        <div>
                           <h1 className="text-3xl font-bold text-white tracking-tight">AI Agent Marketplace</h1>
                           <p className="text-slate-400 font-mono text-sm mt-1">Hosted on A-TownChain • Verifiable Logic</p>
                        </div>
                     </div>
                     
                     <div className="grid grid-cols-2 gap-4">
                        {[1, 2, 3, 4].map(i => (
                           <div key={i} className="p-4 rounded-xl bg-white/5 border border-white/10 hover:border-red-500/50 transition-colors cursor-pointer group">
                              <h3 className="text-lg font-bold text-white group-hover:text-red-400 transition-colors">Semantic Parser Core v{i}.0</h3>
                              <p className="text-sm text-slate-400 mt-2 mb-4">Specialized AI agent for deep data extraction across fragmented networks. 100% decentralized.</p>
                              <div className="flex items-center justify-between text-xs font-mono text-slate-500">
                                 <span>Price: 50 ATC/hr</span>
                                 <span>Deploy</span>
                              </div>
                           </div>
                        ))}
                     </div>
                  </motion.div>
                ) : (
                  <div className="flex flex-col items-center justify-center h-full text-slate-500 py-32">
                     <Globe className="w-16 h-16 mb-4 opacity-50" />
                     <h2 className="text-xl font-medium text-white mb-2">Decentralized Content Rendered</h2>
                     <p className="font-mono text-sm max-w-md text-center">Connected securely to {url} via A-Town Network P2P routing.</p>
                  </div>
                )}
             </div>
          )}
       </div>

    </div>
  );
}
