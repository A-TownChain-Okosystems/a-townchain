import React, { useState } from 'react';
import { Book, Database, Plus, Search, FileText } from 'lucide-react';
import Markdown from 'react-markdown';
import { WIKI_CONTENT } from '../wikiData';

export function WindowWikiView({ id, title }: { id: string, title?: string }) {
  // Check if we have wiki content in global WIKI_CONTENT that might match loosely
  const safeTitle = (title || '').toLowerCase();
  const matchingWiki = WIKI_CONTENT.find(w => (w.title || '').toLowerCase().includes(safeTitle) || (w.id || '').includes(id || ''));
  
  const displayTitle = title || id || 'App';
  const defaultWiki = matchingWiki ? matchingWiki.text : `# ${displayTitle} Documentation\n\nWelcome to the official wiki for **${displayTitle}**.\n\n## Overview\nThis software module is part of the A-Town OS ecosystem, designed to provide specialized functionality inside a dedicated sandboxed window.\n\n## Features\n- Real-time synchronization\n- Isolated state management\n- Direct access to local storage containers\n\n## Architecture\nThe ${displayTitle} architecture employs a robust component-based structure. It links dynamically with the core ATC system layers.\n\n*(Auto-generated workspace wiki for ${id}.)*`;
  
  return (
    <div className="p-6 h-full flex flex-col bg-slate-900/50">
      <div className="flex items-center gap-2 mb-6">
        <div className="p-2 bg-emerald-500/10 rounded-lg border border-emerald-500/20">
          <Book className="w-5 h-5 text-emerald-400" />
        </div>
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">Software Wiki</h2>
          <div className="text-xs text-slate-500 font-mono">APP-ID: {id}</div>
        </div>
      </div>
      <div className="flex-1 overflow-auto bg-[#0a0a0f] border border-white/5 shadow-inner rounded-xl p-6 custom-scrollbar">
        <div className="prose prose-invert prose-emerald max-w-none prose-headings:text-slate-200 prose-a:text-emerald-400">
          <Markdown>{defaultWiki}</Markdown>
        </div>
      </div>
    </div>
  );
}

export function WindowDbView({ id, title }: { id: string, title: string }) {
  const [entries, setEntries] = useState([
    { key: 'init_state', value: 'system_ready', timestamp: new Date().toISOString() },
    { key: 'cache_validation', value: 'ok', timestamp: new Date().toISOString() },
    { key: 'version_check', value: '1.0.0', timestamp: new Date().toISOString() }
  ]);

  return (
    <div className="p-6 h-full flex flex-col bg-slate-900/50">
       <div className="flex items-center justify-between mb-6">
         <div className="flex items-center gap-2">
           <div className="p-2 bg-indigo-500/10 rounded-lg border border-indigo-500/20">
             <Database className="w-5 h-5 text-indigo-400" />
           </div>
           <div>
             <h2 className="text-xl font-bold text-white tracking-tight">Isolated Database</h2>
             <div className="text-xs text-slate-500 font-mono">NAMESPACE: db_{id}</div>
           </div>
         </div>
         <div className="flex gap-2">
           <button className="flex items-center gap-1.5 bg-white/5 hover:bg-white/10 px-3 py-1.5 rounded-lg border border-white/10 text-xs font-medium text-slate-300 transition-colors">
             <Search className="w-3.5 h-3.5" /> Query
           </button>
           <button className="flex items-center gap-1.5 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 px-3 py-1.5 rounded-lg border border-indigo-500/50 text-xs font-medium transition-colors">
             <Plus className="w-3.5 h-3.5" /> New Record
           </button>
         </div>
       </div>
       
       <div className="flex-1 bg-[#0a0a0f] border border-white/5 rounded-xl overflow-hidden flex flex-col shadow-inner">
          <div className="overflow-auto custom-scrollbar">
            <table className="w-full text-sm text-left text-slate-400">
               <thead className="text-[10px] uppercase tracking-widest bg-black/40 text-slate-500 border-b border-white/5 sticky top-0 backdrop-blur-md">
                  <tr>
                     <th className="px-4 py-3 font-mono">Key</th>
                     <th className="px-4 py-3 font-mono">Value</th>
                     <th className="px-4 py-3 font-mono">Timestamp</th>
                  </tr>
               </thead>
               <tbody>
                  {entries.map((e, i) => (
                    <tr key={i} className="border-b border-white/5 last:border-0 hover:bg-white/[0.02]">
                       <td className="px-4 py-4 font-mono text-emerald-400/90 text-xs">{e.key}</td>
                       <td className="px-4 py-4 font-mono text-slate-300 text-xs">{e.value}</td>
                       <td className="px-4 py-4 text-[10px] text-slate-500">{new Date(e.timestamp).toLocaleString()}</td>
                    </tr>
                  ))}
               </tbody>
            </table>
          </div>
       </div>
    </div>
  );
}
