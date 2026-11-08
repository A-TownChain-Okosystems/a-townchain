import React, { useState } from 'react';
import { Search, FileCode } from 'lucide-react';

const rawFiles = import.meta.glob('/src/**/*.{ts,tsx,css,py}', { query: '?raw', import: 'default', eager: true });
const fileKeys = Object.keys(rawFiles);

export function SystemFinderView() {
  const [query, setQuery] = useState('');
  
  const filteredFiles = fileKeys.filter(f => f.toLowerCase().includes(query.toLowerCase()));

  const openInIde = (path: string) => {
    // Set active file
    localStorage.setItem('atc_ide_active_file', path);
    // Dispatch event to open ide and tell it to load the file
    window.dispatchEvent(new CustomEvent('open-file-in-ide', { detail: { path } }));
    window.dispatchEvent(new CustomEvent('open-window', { detail: { id: 'source_code' } }));
  };

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="p-4 border-b border-white/10 flex items-center gap-3">
        <Search className="w-5 h-5 text-slate-400" />
        <input 
          autoFocus
          className="flex-1 bg-transparent border-none outline-none text-white placeholder-slate-500" 
          placeholder="Search files (e.g. App.tsx)..."
          value={query}
          onChange={e => setQuery(e.target.value)}
        />
      </div>
      <div className="flex-1 overflow-auto p-2">
        {filteredFiles.map(file => (
          <button 
            key={file} 
            onClick={() => openInIde(file)} 
            className="flex items-center gap-3 w-full p-2 hover:bg-white/5 rounded text-left transition-colors"
          >
            <div className="w-8 h-8 rounded bg-teal-500/10 border border-teal-500/20 flex items-center justify-center shrink-0">
               <FileCode className="w-4 h-4 text-teal-400" />
            </div>
            <div className="flex flex-col overflow-hidden">
               <span className="text-sm font-bold text-slate-200 truncate">{file.split('/').pop()}</span>
               <span className="text-[10px] font-mono text-slate-500 truncate">{file}</span>
            </div>
          </button>
        ))}
        {filteredFiles.length === 0 && (
          <div className="text-center p-8 text-slate-500 text-sm">
             No files found matching "{query}"
          </div>
        )}
      </div>
    </div>
  );
}
