import React, { useState, useEffect } from 'react';
import { FileText, Save, Trash2 } from 'lucide-react';

export function NotepadView() {
  const [notes, setNotes] = useState(() => {
    try {
      return localStorage.getItem('atc_notepad') || '';
    } catch {
      return '';
    }
  });

  const [savedStatus, setSavedStatus] = useState('');

  const handleSave = () => {
    localStorage.setItem('atc_notepad', notes);
    setSavedStatus('Saved successfully');
    setTimeout(() => setSavedStatus(''), 2000);
  };

  const clear = () => {
    setNotes('');
    localStorage.removeItem('atc_notepad');
  };

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200">
      <div className="flex items-center justify-between px-6 py-4 bg-[#090b14] border-b border-atc-border/50 shadow-sm shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-amber-500/10 border border-amber-500/30">
            <FileText className="w-5 h-5 text-amber-500" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-white tracking-tight">Notepad</h2>
            <p className="text-xs text-slate-400 font-mono">Quick Text Editor</p>
          </div>
        </div>

        <div className="flex items-center gap-3">
          {savedStatus && <span className="text-xs text-emerald-400 font-mono">{savedStatus}</span>}
          <button 
            onClick={clear}
            className="flex items-center gap-2 px-3 py-1.5 rounded-lg border border-red-500/30 text-red-400 hover:bg-red-500/10 transition-colors text-sm font-medium"
          >
            <Trash2 className="w-4 h-4" /> Clear
          </button>
          <button 
            onClick={handleSave}
            className="flex items-center gap-2 px-4 py-1.5 rounded-lg bg-atc-cyan text-slate-900 shadow-[0_0_15px_rgba(6,182,212,0.2)] hover:bg-atc-cyan/90 transition-colors text-sm font-bold"
          >
            <Save className="w-4 h-4" /> Save
          </button>
        </div>
      </div>

      <div className="flex-1 p-6">
        <textarea
          value={notes}
          onChange={e => setNotes(e.target.value)}
          placeholder="Start typing..."
          className="w-full h-full bg-black/40 border border-white/5 rounded-2xl p-6 text-slate-300 focus:outline-none focus:border-atc-cyan/50 resize-none font-mono custom-scrollbar"
          spellCheck={false}
        />
      </div>
    </div>
  );
}
