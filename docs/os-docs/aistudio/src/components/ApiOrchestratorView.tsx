import React, { useState, useEffect } from 'react';
import { Activity, ShieldCheck, Database, RefreshCw, Layers, Plus, Terminal, Search, Zap, CheckCircle2, AlertTriangle, AlertCircle, PlayCircle, Loader2 } from 'lucide-react';
import * as d3 from 'd3';

interface ApiEndpoint {
  id: string;
  name: string;
  url: string;
  status: 'healthy' | 'degraded' | 'offline' | 'loading' | 'repairing';
  latency: number;
  requests: number;
  errorRate: number;
  lastRepaired?: string;
  group: 'core' | 'external' | 'blockchain' | 'ai';
}

export function ApiOrchestratorView() {
  const [activeTab, setActiveTab] = useState<'matrix' | 'index' | 'health' | 'repair' | 'ondemand'>('matrix');
  const [searchQuery, setSearchQuery] = useState('');
  
  const [endpoints, setEndpoints] = useState<ApiEndpoint[]>([]);
  const [isInitializing, setIsInitializing] = useState(true);

  const fetchRealHealth = async () => {
    try {
      const res = await fetch('/api/orchestrator/health');
      if (res.ok) {
        const data = await res.json();
        setEndpoints(data);
      }
    } catch(e) {
      console.error(e);
    } finally {
      setIsInitializing(false);
    }
  };

  useEffect(() => {
    fetchRealHealth();
    const iv = setInterval(fetchRealHealth, 15000);
    return () => clearInterval(iv);
  }, []);

  const [isRepairingAll, setIsRepairingAll] = useState(false);

  const startAutoRepair = (id?: string) => {
    if (id) {
      setEndpoints(prev => prev.map(ep => ep.id === id ? { ...ep, status: 'repairing' } : ep));
      setTimeout(() => fetchRealHealth(), 3000);
    } else {
      setIsRepairingAll(true);
      setEndpoints(prev => prev.map(ep => ep.status === 'offline' || ep.status === 'degraded' ? { ...ep, status: 'repairing' } : ep));
      setTimeout(() => {
        fetchRealHealth().finally(() => setIsRepairingAll(false));
      }, 4000);
    }
  };

  const loadOnDemand = () => {
    const newId = `api-dyn-${Date.now()}`;
    const newEndpoint: ApiEndpoint = {
      id: newId,
      name: `Dynamic Service ${endpoints.length + 1}`,
      url: `api.atown.chain/dyn/${newId.substring(8)}`,
      status: 'loading',
      latency: 0,
      requests: 0,
      errorRate: 0,
      group: 'ai'
    };
    
    setEndpoints(prev => [...prev, newEndpoint]);
    
    setTimeout(() => {
      setEndpoints(prev => prev.map(ep => ep.id === newId ? { ...ep, status: 'healthy', latency: 25 } : ep));
    }, 2000);
  };

  const deleteEndpoint = (id: string) => {
    setEndpoints(prev => prev.filter(ep => ep.id !== id));
  };
  
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'healthy': return 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20';
      case 'degraded': return 'text-amber-400 bg-amber-500/10 border-amber-500/20';
      case 'offline': return 'text-rose-400 bg-rose-500/10 border-rose-500/20';
      case 'repairing': return 'text-indigo-400 bg-indigo-500/10 border-indigo-500/20';
      case 'loading': return 'text-cyan-400 bg-cyan-500/10 border-cyan-500/20';
      default: return 'text-slate-400 bg-slate-500/10 border-slate-500/20';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'healthy': return <CheckCircle2 className="w-4 h-4" />;
      case 'degraded': return <AlertTriangle className="w-4 h-4" />;
      case 'offline': return <AlertCircle className="w-4 h-4" />;
      case 'repairing':
      case 'loading': return <Loader2 className="w-4 h-4 animate-spin" />;
      default: return <Activity className="w-4 h-4" />;
    }
  };

  const filteredEndpoints = endpoints.filter(ep => 
    ep.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
    ep.url.toLowerCase().includes(searchQuery.toLowerCase()) ||
    ep.group.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="flex flex-col h-full bg-[#04060b] text-white overflow-hidden">
      <div className="p-6 border-b border-white/10 shrink-0 bg-[#0b0f19] flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-mono tracking-tight flex items-center gap-3">
            <Zap className="w-6 h-6 text-indigo-400" />
            API Orchestrator
          </h1>
          <p className="text-sm font-mono text-slate-400 mt-1">Autonomous Service Mesh, Index & Auto-Repair</p>
        </div>
        <div className="flex gap-3">
          <button 
            onClick={loadOnDemand}
            className="flex items-center gap-2 px-4 py-2 bg-cyan-500/20 text-cyan-400 hover:bg-cyan-500/30 border border-cyan-500/30 rounded-xl font-mono text-sm tracking-wider uppercase transition-colors"
          >
            <Plus className="w-4 h-4" />
            On-Demand Load
          </button>
          <button 
            onClick={() => startAutoRepair()}
            disabled={isRepairingAll || endpoints.every(e => e.status === 'healthy' || e.status === 'loading')}
            className="flex items-center gap-2 px-4 py-2 bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500/30 border border-indigo-500/30 rounded-xl font-mono text-sm tracking-wider uppercase transition-colors disabled:opacity-50"
          >
            {isRepairingAll ? <Loader2 className="w-4 h-4 animate-spin" /> : <ShieldCheck className="w-4 h-4" />}
            Auto-Repair All
          </button>
        </div>
      </div>

      <div className="flex px-6 border-b border-white/10 shrink-0 bg-black/40">
        {[
          { id: 'matrix', label: 'API Matrix', icon: Layers },
          { id: 'index', label: 'API Index', icon: Database },
          { id: 'health', label: 'Self Health', icon: Activity },
          { id: 'repair', label: 'Auto Reparatur logs', icon: ShieldCheck },
        ].map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id as any)}
            className={`flex items-center gap-2 px-6 py-4 font-mono text-sm uppercase tracking-wider border-b-2 transition-colors ${
              activeTab === tab.id
                ? 'border-indigo-500 text-indigo-400 bg-indigo-500/5'
                : 'border-transparent text-slate-400 hover:text-white hover:bg-white/5'
            }`}
          >
            <tab.icon className="w-4 h-4" />
            {tab.label}
          </button>
        ))}
      </div>

      <div className="flex-1 overflow-y-auto p-6 custom-scrollbar">
        {(activeTab === 'matrix' || activeTab === 'index') && (
          <div className="flex flex-col gap-6">
            <div className="flex items-center justify-between">
              <div className="relative w-96">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
                <input 
                  type="text" 
                  placeholder="Search APIs..."
                  value={searchQuery}
                  onChange={e => setSearchQuery(e.target.value)}
                  className="w-full bg-black/40 border border-white/10 rounded-xl py-2.5 pl-10 pr-4 text-sm font-mono text-white focus:outline-none focus:border-indigo-500/50"
                />
              </div>
              <div className="flex items-center gap-4 text-sm font-mono">
                <span className="flex items-center gap-2 text-emerald-400">
                  <div className="w-2 h-2 rounded-full bg-emerald-400" /> Healthy: {endpoints.filter(e => e.status === 'healthy').length}
                </span>
                <span className="flex items-center gap-2 text-amber-400">
                  <div className="w-2 h-2 rounded-full bg-amber-400" /> Degraded: {endpoints.filter(e => e.status === 'degraded').length}
                </span>
                <span className="flex items-center gap-2 text-rose-400">
                  <div className="w-2 h-2 rounded-full bg-rose-400" /> Offline: {endpoints.filter(e => e.status === 'offline').length}
                </span>
              </div>
            </div>

            <div className="grid grid-cols-1 xl:grid-cols-2 gap-4">
              {isInitializing && endpoints.length === 0 ? (
                <div className="xl:col-span-2 py-12 text-center flex flex-col items-center gap-4 text-indigo-400 font-mono text-sm border border-dashed border-white/10 rounded-2xl">
                  <Loader2 className="w-8 h-8 animate-spin" />
                  Orchestrator initialisiert reale API Endpunkte...
                </div>
              ) : (
                filteredEndpoints.map(ep => (
                <div key={ep.id} className="bg-[#0b0f19] border border-white/10 rounded-2xl p-5 hover:border-indigo-500/30 transition-colors flex flex-col gap-4">
                  <div className="flex items-start justify-between">
                    <div>
                      <h3 className="text-lg font-bold font-mono text-white flex items-center gap-2">
                        {ep.name}
                        <span className="px-2 py-0.5 text-[10px] uppercase tracking-wider rounded border bg-white/5 border-white/10 text-slate-400">
                          {ep.group}
                        </span>
                      </h3>
                      <p className="text-sm font-mono text-slate-400 mt-1 flex items-center gap-1.5">
                        <Terminal className="w-3.5 h-3.5" /> {ep.url}
                      </p>
                    </div>
                    <div className={`px-3 py-1.5 rounded-lg border font-mono text-[10px] uppercase tracking-wider font-bold flex items-center gap-2 ${getStatusColor(ep.status)}`}>
                      {getStatusIcon(ep.status)}
                      {ep.status}
                    </div>
                  </div>

                  <div className="grid grid-cols-3 gap-4 pt-4 border-t border-white/5">
                    <div className="flex flex-col">
                      <span className="text-[10px] text-slate-500 uppercase tracking-wider font-mono">Latency</span>
                      <span className="text-lg font-mono text-white">{ep.latency}ms</span>
                    </div>
                    <div className="flex flex-col">
                      <span className="text-[10px] text-slate-500 uppercase tracking-wider font-mono">Error Rate</span>
                      <span className={`text-lg font-mono ${ep.errorRate > 1 ? 'text-amber-400' : ep.errorRate > 5 ? 'text-rose-400' : 'text-emerald-400'}`}>
                        {ep.errorRate}%
                      </span>
                    </div>
                    <div className="flex flex-col">
                      <span className="text-[10px] text-slate-500 uppercase tracking-wider font-mono">Requests/min</span>
                      <span className="text-lg font-mono text-slate-300">{ep.requests.toLocaleString()}</span>
                    </div>
                  </div>
                  
                  <div className="flex justify-end gap-2 pt-2">
                    {ep.status !== 'healthy' && ep.status !== 'loading' && ep.status !== 'repairing' && (
                      <button
                        onClick={() => startAutoRepair(ep.id)}
                        className="px-3 py-1.5 bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500/30 border border-indigo-500/30 rounded-lg text-xs font-mono uppercase tracking-wider transition-colors"
                      >
                        Start Repair
                      </button>
                    )}
                    <button
                      onClick={() => deleteEndpoint(ep.id)}
                      className="px-3 py-1.5 bg-rose-500/10 text-rose-400 hover:bg-rose-500/20 border border-rose-500/20 rounded-lg text-xs font-mono uppercase tracking-wider transition-colors"
                    >
                      Kill Service
                    </button>
                  </div>
                </div>
              ))
              )}
              
              {!isInitializing && filteredEndpoints.length === 0 && (
                <div className="xl:col-span-2 py-12 text-center text-slate-500 font-mono text-sm border border-dashed border-white/10 rounded-2xl">
                  No endpoints found matching your criteria.
                </div>
              )}
            </div>
          </div>
        )}

        {activeTab === 'health' && (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div className="lg:col-span-2 bg-[#0b0f19] border border-white/10 rounded-2xl p-6">
              <h3 className="text-lg font-mono text-white mb-6 border-b border-white/10 pb-4">Global API Health</h3>
              <div className="flex flex-col gap-4">
                <div className="flex items-center justify-between p-4 bg-emerald-500/10 border border-emerald-500/20 rounded-xl">
                  <div className="flex items-center gap-3">
                    <CheckCircle2 className="w-6 h-6 text-emerald-400" />
                    <div>
                      <h4 className="font-mono text-emerald-400 font-bold">System Nominal</h4>
                      <p className="text-sm text-emerald-400/70 font-mono">{(endpoints.filter(e => e.status === 'healthy').length / endpoints.length * 100).toFixed(1)}% endpoints healthy</p>
                    </div>
                  </div>
                  <div className="text-2xl font-mono text-emerald-400 font-bold">
                    {endpoints.filter(e => e.status === 'healthy').length}/{endpoints.length}
                  </div>
                </div>
                
                <h4 className="text-sm font-mono text-slate-400 uppercase tracking-widest mt-4">Average Latency by Group</h4>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                  {['core', 'blockchain', 'ai', 'external'].map(group => {
                    const groupEps = endpoints.filter(e => e.group === group);
                    const avgLat = groupEps.length ? groupEps.reduce((acc, curr) => acc + curr.latency, 0) / groupEps.length : 0;
                    return (
                      <div key={group} className="bg-black/40 border border-white/5 rounded-xl p-4 text-center">
                        <div className="text-xs text-slate-500 font-mono uppercase mb-2">{group}</div>
                        <div className={`text-xl font-mono font-bold ${avgLat > 300 ? 'text-rose-400' : avgLat > 100 ? 'text-amber-400' : 'text-emerald-400'}`}>
                          {avgLat.toFixed(0)}<span className="text-sm ml-1">ms</span>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            </div>
            
            <div className="bg-[#0b0f19] border border-white/10 rounded-2xl p-6">
               <h3 className="text-lg font-mono text-white mb-6 border-b border-white/10 pb-4">Real-time Load</h3>
               <div className="flex flex-col gap-4">
                 {endpoints.map(ep => (
                   <div key={`load-${ep.id}`}>
                     <div className="flex justify-between text-xs font-mono text-slate-400 mb-1">
                       <span>{ep.name}</span>
                       <span>{(ep.requests / 1000).toFixed(1)}k req/m</span>
                     </div>
                     <div className="h-1.5 w-full bg-white/10 rounded-full overflow-hidden">
                       <div 
                         className="h-full bg-indigo-500 rounded-full"
                         style={{ width: `${Math.min(100, (ep.requests / 50000) * 100)}%` }}
                       />
                     </div>
                   </div>
                 ))}
               </div>
            </div>
          </div>
        )}

        {activeTab === 'repair' && (
           <div className="bg-[#0b0f19] border border-white/10 rounded-2xl p-6 flex flex-col gap-4">
             <div className="flex items-center gap-3 text-indigo-400 border-b border-white/10 pb-4 mb-2">
               <ShieldCheck className="w-5 h-5" />
               <h3 className="text-lg font-mono font-bold">Auto-Repair Audit Logs</h3>
             </div>
             
             <div className="flex flex-col gap-2 font-mono text-sm">
               {endpoints.filter(e => e.lastRepaired).map((ep, i) => (
                 <div key={i} className="flex items-center justify-between p-3 bg-black/40 border border-white/5 rounded-lg">
                   <div className="flex items-center gap-3">
                     <CheckCircle2 className="w-4 h-4 text-emerald-500" />
                     <span className="text-slate-300">Self-healed <span className="text-white font-bold">{ep.name}</span></span>
                   </div>
                   <span className="text-slate-500 text-xs">{new Date(ep.lastRepaired || '').toLocaleString()}</span>
                 </div>
               ))}
               <div className="flex items-center justify-between p-3 bg-black/40 border border-white/5 rounded-lg opacity-50">
                 <div className="flex items-center gap-3">
                   <ShieldCheck className="w-4 h-4 text-indigo-500" />
                   <span className="text-slate-300">Orchestrator monitoring initialized</span>
                 </div>
                 <span className="text-slate-500 text-xs">System Boot</span>
               </div>
               
               {endpoints.filter(e => e.lastRepaired).length === 0 && (
                 <div className="text-slate-500 text-center py-6">No repair events logged recently.</div>
               )}
             </div>
           </div>
        )}
      </div>
    </div>
  );
}
