import React, { useState } from 'react';
import { Search, Hexagon, Component } from 'lucide-react';

const REGISTRY_DATA = [
  {
    category: "A-TownChain (ATC) Standards",
    files: [
      { name: "Application Binary", extension: ".atc", description: "Standard Application Binary" },
      { name: "Network Configuration", extension: ".net", description: "Network Configuration & Protocol" },
      { name: "Security & Crypto", extension: ".sec", description: "Security & Cryptography" },
      { name: "Database Standard", extension: ".db", description: "Database & Storage Layer" },
      { name: "User Interface", extension: ".ui", description: "User Interface Template" }
    ]
  },
  {
    category: "AI-System (ATS) Standards",
    files: [
      { name: "Core Engine", extension: ".core", description: "Core AI Logic Engine" },
      { name: "Memory DB", extension: ".mem", description: "Memory & Data Persistence" },
      { name: "Training Profile", extension: ".train", description: "Training & Weight Distribution" },
      { name: "Communication Protocol", extension: ".comm", description: "Inter-System Communication" }
    ]
  }
];

export function RegistryPanel() {
  const [search, setSearch] = useState('');

  return (
    <div className="flex flex-col h-full bg-[#0c0c0e]">
      <div className="h-10 flex items-center px-6 border-b border-white/5 bg-black/40">
        <Hexagon className="w-4 h-4 mr-2 text-cyan-400" />
        <span className="text-sm font-bold text-slate-200">ATC/ATS Registry</span>
      </div>
      
      <div className="p-4 border-b border-white/5">
        <div className="relative">
          <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
          <input 
            type="text" 
            placeholder="Search by extension or description..."
            value={search}
            onChange={e => setSearch(e.target.value)}
            className="w-full bg-white/5 border border-white/10 rounded-md py-1.5 pl-9 pr-3 text-sm text-slate-200 focus:outline-none focus:border-cyan-500/50"
          />
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto p-4 space-y-6">
        {REGISTRY_DATA.map((category, idx) => {
          const filteredFiles = category.files.filter(f => 
            f.extension.toLowerCase().includes(search.toLowerCase()) || 
            f.description.toLowerCase().includes(search.toLowerCase())
          );
          
          if (filteredFiles.length === 0) return null;
          
          return (
            <div key={idx}>
               <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">{category.category}</h3>
               <div className="space-y-2">
                 {filteredFiles.map((file, i) => (
                   <div key={i} className="flex items-start gap-3 p-3 bg-white/5 hover:bg-white/10 rounded-lg transition-colors border border-white/5">
                     <Component className="w-5 h-5 shrink-0 text-slate-400 mt-0.5" />
                     <div>
                       <div className="flex items-center gap-2">
                         <span className="text-cyan-400 font-mono text-xs">{file.extension}</span>
                         <h4 className="text-sm font-bold text-slate-300">{file.name}</h4>
                       </div>
                       <p className="text-xs text-slate-500 mt-1">{file.description}</p>
                     </div>
                   </div>
                 ))}
               </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
