import React, { useState } from 'react';
import { Plug, Zap, CheckCircle2, Circle, Settings, Download } from 'lucide-react';

export function PluginsPanel() {
  const [plugins, setPlugins] = useState([
    { id: '1', name: 'ATOS Blockchain Link', desc: 'Verbindet die IDE mit dem A-TownChain Testnet.', author: 'Community', version: '2.1.0', installed: true, active: true },
    { id: '2', name: 'React UI Generator', desc: 'Erweitert Lumino um automatische React-Frontend-Kompilierung.', author: 'Offiziell', version: '1.5.2', installed: true, active: false },
    { id: '3', name: 'Advanced 3D Physics', desc: 'Physik-Engine Integration für Canvas/Spiele.', author: 'Offiziell', version: '0.9.beta', installed: false, active: false },
    { id: '4', name: 'Docker Auto-Deploy', desc: 'Automatische Container-Virtualisierung beim Speichern.', author: 'Community', version: '3.0.1', installed: false, active: false },
  ]);

  const togglePlugin = (id: string) => {
    setPlugins(plugins.map(p => {
      if (p.id === id) {
        if (!p.installed) return p;
        return { ...p, active: !p.active };
      }
      return p;
    }));
  };

  const installPlugin = (id: string) => {
    setPlugins(plugins.map(p => {
      if (p.id === id) {
        return { ...p, installed: true, active: true };
      }
      return p;
    }));
  };

  return (
    <div className="flex-1 bg-[#101014] text-slate-200 outline-none flex flex-col font-sans h-full overflow-hidden">
      <div className="px-6 py-4 border-b border-white/5 flex items-center justify-between bg-black/20">
        <div className="flex items-center gap-3">
          <div className="bg-fuchsia-500/20 p-2 rounded-lg">
            <Plug className="w-5 h-5 text-fuchsia-400" />
          </div>
          <div>
            <h1 className="text-lg font-bold text-white tracking-tight">Plugin Store</h1>
            <p className="text-xs text-slate-400">Erweitere ATOS mit Modulen und Drittanbieter-Tools.</p>
          </div>
        </div>
        <div className="flex gap-2">
          <span className="text-xs text-slate-500 font-medium px-3 py-1.5 bg-black/50 rounded border border-white/5 flex items-center gap-2">
            <Zap className="w-3.5 h-3.5 text-yellow-400" />
            0.5s Load Time
          </span>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 xl:grid-cols-3 gap-4">
          {plugins.map(plugin => (
            <div key={plugin.id} className="bg-black/40 border border-white/5 p-4 rounded-xl hover:border-white/10 transition-colors flex flex-col">
              <div className="flex items-start justify-between mb-2">
                <h3 className="font-bold text-slate-200 text-sm">{plugin.name}</h3>
                {plugin.installed ? (
                   <button 
                     onClick={() => togglePlugin(plugin.id)} 
                     className="text-slate-400 hover:text-white transition-colors"
                   >
                     {plugin.active ? <CheckCircle2 className="w-5 h-5 text-emerald-400" /> : <Circle className="w-5 h-5" />}
                   </button>
                ) : (
                   <button 
                     onClick={() => installPlugin(plugin.id)}
                     className="bg-fuchsia-500/20 hover:bg-fuchsia-500/30 text-fuchsia-400 p-1.5 rounded-lg transition-colors"
                   >
                     <Download className="w-4 h-4" />
                   </button>
                )}
              </div>
              <p className="text-xs text-slate-400 mb-4 flex-1 line-clamp-2 leading-relaxed">
                {plugin.desc}
              </p>
              <div className="flex items-center justify-between text-[10px] text-slate-500 pt-3 border-t border-white/5">
                <span className="flex items-center gap-1.5">
                  <span className={`w-1.5 h-1.5 rounded-full ${plugin.author === 'Offiziell' ? 'bg-cyan-500' : 'bg-orange-500'}`}></span>
                  {plugin.author}
                </span>
                <span className="font-mono">{plugin.version}</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
