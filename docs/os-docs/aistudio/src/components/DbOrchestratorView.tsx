import React from 'react';
import { ServerCog, Database, Network, HardDrive, RefreshCw, Activity, Cpu } from 'lucide-react';

export function DbOrchestratorView() {
  const nodes = [
    { name: "Global Master Node", type: "Primary", status: "Active", latency: "12ms", load: "45%", category: "Infrastructure", subcategory: "Database", tags: ["master", "global"], version: "v4.2.1" },
    { name: "EU West Replica", type: "Secondary", status: "Syncing", latency: "24ms", load: "60%", category: "Infrastructure", subcategory: "Replica Node", tags: ["eu-west", "sync"], version: "v4.2.1" },
    { name: "US East Replica", type: "Secondary", status: "Active", latency: "18ms", load: "30%", category: "Infrastructure", subcategory: "Replica Node", tags: ["us-east"], version: "v4.2.1" },
    { name: "AP South Edge", type: "Edge Cache", status: "Active", latency: "40ms", load: "15%", category: "Infrastructure", subcategory: "CDN", tags: ["ap-south", "edge"], version: "v4.2.0" },
  ];

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400 shadow-[0_0_15px_rgba(245,158,11,0.2)]">
          <ServerCog className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Database Orchestrator
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Überwachung, Skalierung und Replikation der A-Town Datenbankinfrastruktur über verteilte Knotenpunkte.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="p-5 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col">
          <div className="flex items-center justify-between mb-4">
            <span className="text-sm text-slate-400 font-medium">Cluster Health</span>
            <Activity className="w-4 h-4 text-emerald-400" />
          </div>
          <span className="text-3xl font-bold text-white mb-1">99.9%</span>
          <span className="text-xs text-slate-500">Uptime last 30d</span>
        </div>
        <div className="p-5 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col">
          <div className="flex items-center justify-between mb-4">
            <span className="text-sm text-slate-400 font-medium">Active Nodes</span>
            <Network className="w-4 h-4 text-blue-400" />
          </div>
          <span className="text-3xl font-bold text-white mb-1">1,204</span>
          <span className="text-xs text-emerald-400 flex items-center gap-1"><RefreshCw className="w-3 h-3" /> Fully Synced</span>
        </div>
        <div className="p-5 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col">
          <div className="flex items-center justify-between mb-4">
            <span className="text-sm text-slate-400 font-medium">Storage Used</span>
            <HardDrive className="w-4 h-4 text-amber-400" />
          </div>
          <span className="text-3xl font-bold text-white mb-1">4.2 PB</span>
          <span className="text-xs text-slate-500">Across 14 Regions</span>
        </div>
        <div className="p-5 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col">
          <div className="flex items-center justify-between mb-4">
            <span className="text-sm text-slate-400 font-medium">Query Load</span>
            <Cpu className="w-4 h-4 text-purple-400" />
          </div>
          <span className="text-3xl font-bold text-white mb-1">84K</span>
          <span className="text-xs text-slate-500">Queries per second</span>
        </div>
      </div>

      <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5">
        <h3 className="text-lg font-semibold text-white mb-6 flex items-center gap-2">
          <Database className="w-5 h-5 text-amber-400" /> Topology Status
        </h3>
        <div className="overflow-x-auto">
          <table className="w-full text-sm text-left text-slate-400">
            <thead className="text-xs uppercase bg-black/40 text-slate-500">
              <tr>
                <th className="px-4 py-3 rounded-l-lg font-mono">Node Name <span className="text-[9px] text-slate-500">(Version)</span></th>
                <th className="px-4 py-3 font-mono">Type / Category</th>
                <th className="px-4 py-3 font-mono">Status</th>
                <th className="px-4 py-3 font-mono">Latency</th>
                <th className="px-4 py-3 font-mono">Load</th>
                <th className="px-4 py-3 rounded-r-lg font-mono">Tags</th>
              </tr>
            </thead>
            <tbody>
              {nodes.map((node, i) => (
                <tr key={i} className="border-b border-white/5 hover:bg-white/[0.02]">
                  <td className="px-4 py-4 font-medium text-slate-200">
                    {node.name}
                    <div className="text-[10px] text-slate-500 font-mono mt-0.5">VER: {node.version}</div>
                  </td>
                  <td className="px-4 py-4">
                    <div className="text-slate-300">{node.type}</div>
                    <div className="text-[10px] text-slate-500 uppercase tracking-widest">{node.category} &gt; {node.subcategory}</div>
                  </td>
                  <td className="px-4 py-4">
                    <span className={`px-2 py-1 rounded text-xs ${node.status === 'Active' ? 'bg-emerald-500/10 text-emerald-400' : 'bg-amber-500/10 text-amber-400'}`}>
                      {node.status}
                    </span>
                  </td>
                  <td className="px-4 py-4 font-mono">{node.latency}</td>
                  <td className="px-4 py-4 font-mono">{node.load}</td>
                  <td className="px-4 py-4">
                    <div className="flex flex-wrap gap-1">
                      {node.tags.map(t => (
                        <span key={t} className="px-1.5 py-0.5 bg-white/5 border border-white/10 rounded text-[9px] text-slate-400 uppercase font-mono tracking-wider">{t}</span>
                      ))}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
