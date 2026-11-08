import React, { useState } from 'react';
import { Book, Search, Filter, AppWindow } from 'lucide-react';
import { EXTENSION_CATEGORIES } from '../data/extensions';

export function DictionaryPanel() {
  const [searchQuery, setSearchQuery] = useState('');
  const [activeCategory, setActiveCategory] = useState<string | null>(null);

  const filteredCategories = EXTENSION_CATEGORIES.map(cat => ({
    ...cat,
    extensions: cat.extensions.filter(ext => 
      ext.ext.toLowerCase().includes(searchQuery.toLowerCase()) ||
      ext.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      ext.description.toLowerCase().includes(searchQuery.toLowerCase())
    )
  })).filter(cat => 
    (activeCategory ? cat.name === activeCategory : true) && cat.extensions.length > 0
  );

  return (
    <div className="flex-1 overflow-hidden flex flex-col h-full bg-[#0a0a0c]">
      <div className="h-14 flex items-center px-6 border-b border-white/10 bg-black/40 backdrop-blur-2xl shrink-0 gap-3">
        <Book className="w-5 h-5 text-indigo-400" />
        <h2 className="text-sm font-bold text-slate-200 tracking-wide uppercase">ATOS File Dictionary</h2>
      </div>

      <div className="p-4 border-b border-white/5 bg-white/[0.02] shrink-0">
        <div className="flex items-center gap-2 bg-black/40 border border-white/10 rounded-md px-3 py-2 mb-3 focus-within:border-indigo-500/50 transition-colors">
          <Search className="w-4 h-4 text-slate-500 shrink-0" />
          <input 
            type="text" 
            placeholder="Search extensions (e.g. .atc, smart contract)..." 
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
            className="bg-transparent border-none outline-none text-sm text-slate-200 flex-1 placeholder-slate-600"
          />
        </div>
        
        <div className="flex gap-2 overflow-x-auto pb-1 scrollbar-hide">
          <button 
            onClick={() => setActiveCategory(null)}
            className={`whitespace-nowrap px-3 py-1 rounded-full text-xs font-medium transition-colors border ${!activeCategory ? 'bg-indigo-500/20 text-indigo-300 border-indigo-500/30' : 'bg-transparent text-slate-400 border-white/10 hover:border-white/20'}`}
          >
            All Standards
          </button>
          {EXTENSION_CATEGORIES.map(cat => (
            <button 
              key={cat.name}
              onClick={() => setActiveCategory(cat.name === activeCategory ? null : cat.name)}
              className={`whitespace-nowrap px-3 py-1 rounded-full text-xs font-medium transition-colors border ${cat.name === activeCategory ? 'bg-indigo-500/20 text-indigo-300 border-indigo-500/30' : 'bg-transparent text-slate-400 border-white/10 hover:border-white/20'}`}
            >
              {cat.name}
            </button>
          ))}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-6">
        {filteredCategories.length === 0 ? (
          <div className="text-center py-10 text-slate-500 text-sm">
            No matching extensions found.
          </div>
        ) : (
          filteredCategories.map(cat => (
            <div key={cat.name} className="space-y-3">
              <h3 className="text-xs font-bold uppercase tracking-widest text-indigo-400/80 flex items-center gap-2">
                <AppWindow className="w-3.5 h-3.5" /> {cat.name}
              </h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                {cat.extensions.map(ext => (
                  <div key={ext.ext} className="bg-white/[0.03] border border-white/5 hover:border-indigo-500/30 rounded-lg p-3 transition-colors group">
                    <div className="flex items-baseline justify-between mb-1.5">
                      <span className="text-indigo-400 font-mono font-bold text-sm bg-indigo-500/10 px-1.5 py-0.5 rounded">{ext.ext}</span>
                      <span className="text-[10px] text-slate-500 uppercase tracking-widest opacity-0 group-hover:opacity-100 transition-opacity">Standard</span>
                    </div>
                    <h4 className="text-sm font-medium text-slate-200 mb-1">{ext.name}</h4>
                    <p className="text-xs text-slate-400 leading-relaxed">{ext.description}</p>
                  </div>
                ))}
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
