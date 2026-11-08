import React, { useState, useEffect } from 'react';
import { AreaChart, Area, ResponsiveContainer, YAxis } from 'recharts';
import { Cpu, MemoryStick, Users, Activity } from 'lucide-react';

interface MetricPoint {
  time: number;
  cpu: number;
  ram: number;
  peers: number;
}

export function NodeHealthMonitor() {
  const [data, setData] = useState<MetricPoint[]>(() => {
    const initData = [];
    let now = Date.now();
    for (let i = 20; i >= 0; i--) {
      initData.push({
        time: now - i * 2000,
        cpu: 30 + Math.random() * 20,
        ram: 40 + Math.random() * 10,
        peers: 12 + Math.floor(Math.random() * 5),
      });
    }
    return initData;
  });

  useEffect(() => {
    const interval = setInterval(() => {
      setData((prev) => {
        const last = prev[prev.length - 1];
        const next = {
          time: Date.now(),
          cpu: Math.max(0, Math.min(100, last.cpu + (Math.random() * 10 - 5))),
          ram: Math.max(0, Math.min(100, last.ram + (Math.random() * 4 - 2))),
          peers: Math.max(0, Math.min(50, last.peers + Math.floor(Math.random() * 3 - 1))),
        };
        return [...prev.slice(1), next];
      });
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  const current = data[data.length - 1];

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/50 rounded-xl p-4 overflow-hidden relative group">
      <div className="flex items-center justify-between mb-4 relative z-10">
        <h3 className="text-sm font-bold font-mono text-slate-200 tracking-wider flex items-center gap-2">
          <Activity className="w-4 h-4 text-emerald-400" />
          Node Health
        </h3>
        <div className="flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
          <span className="text-[10px] font-mono text-emerald-400 uppercase tracking-widest">Live</span>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-3 mb-4 relative z-10">
        <div className="bg-white/5 border border-white/5 rounded-lg p-2.5">
          <div className="flex items-center gap-1.5 text-slate-400 mb-1">
            <Cpu className="w-3.5 h-3.5" />
            <span className="text-[10px] font-mono uppercase tracking-wider">CPU</span>
          </div>
          <div className="text-lg font-bold text-slate-200">
            {current.cpu.toFixed(1)}<span className="text-xs text-slate-500 ml-0.5">%</span>
          </div>
        </div>
        <div className="bg-white/5 border border-white/5 rounded-lg p-2.5">
          <div className="flex items-center gap-1.5 text-slate-400 mb-1">
            <MemoryStick className="w-3.5 h-3.5" />
            <span className="text-[10px] font-mono uppercase tracking-wider">RAM</span>
          </div>
          <div className="text-lg font-bold text-slate-200">
            {current.ram.toFixed(1)}<span className="text-xs text-slate-500 ml-0.5">%</span>
          </div>
        </div>
        <div className="bg-white/5 border border-white/5 rounded-lg p-2.5">
          <div className="flex items-center gap-1.5 text-slate-400 mb-1">
            <Users className="w-3.5 h-3.5" />
            <span className="text-[10px] font-mono uppercase tracking-wider">Peers</span>
          </div>
          <div className="text-lg font-bold text-atc-cyan">
            {current.peers}
          </div>
        </div>
      </div>

      <div className="h-16 w-full -mx-4 -mb-4 relative">
        <div className="absolute inset-x-4 top-0 text-[9px] font-mono text-slate-600 z-10 hidden group-hover:block transition-opacity opacity-0 group-hover:opacity-100">Load Trend</div>
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={data} margin={{ top: 5, right: 0, left: 0, bottom: 0 }}>
            <defs>
              <linearGradient id="colorCpu" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#10b981" stopOpacity={0.3}/>
                <stop offset="95%" stopColor="#10b981" stopOpacity={0}/>
              </linearGradient>
            </defs>
            <YAxis domain={[0, 100]} hide />
            <Area 
              type="monotone" 
              dataKey="cpu" 
              stroke="#10b981" 
              strokeWidth={1.5}
              fillOpacity={1} 
              fill="url(#colorCpu)" 
              isAnimationActive={false}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
