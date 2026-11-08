import React, { useState, useEffect } from 'react';
import { Activity, Cpu, Server, MemoryStick } from 'lucide-react';

export function TaskManagerView() {
  const [processes, setProcesses] = useState([
    { id: 1021, name: "atc_kernel.exe", cpu: "12%", mem: "140 MB", status: "Running" },
    { id: 1042, name: "atc_node_p2p", cpu: "8%", mem: "256 MB", status: "Running" },
    { id: 1105, name: "aurora_ai_agent", cpu: "45%", mem: "1.2 GB", status: "Running" },
    { id: 1198, name: "ipfs_daemon", cpu: "2%", mem: "64 MB", status: "Running" },
    { id: 1240, name: "desktop_ui", cpu: "5%", mem: "120 MB", status: "Running" }
  ]);

  useEffect(() => {
    // Simulate fluctuating usage
    const interval = setInterval(() => {
      setProcesses(prev => prev.map(p => {
        if (p.name === "aurora_ai_agent") {
          return { ...p, cpu: `${Math.floor(Math.random() * 30 + 30)}%` };
        }
        if (p.name === "desktop_ui") {
          return { ...p, cpu: `${Math.floor(Math.random() * 10 + 1)}%` };
        }
        return p;
      }));
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex flex-col h-full bg-[#060a16] p-6 text-slate-300 rounded-xl border border-white/10">
      <div className="flex items-center gap-3 mb-6">
        <Activity className="w-6 h-6 text-indigo-400" />
        <h2 className="text-xl font-bold text-white tracking-tight">System Monitor (Task Manager)</h2>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-3 gap-6 mb-8">
        <div className="bg-white/5 border border-white/10 rounded-lg p-4 flex flex-col gap-2">
          <div className="flex justify-between items-center text-slate-400">
            <span className="text-xs uppercase tracking-wider">CPU Usage</span>
            <Cpu className="w-4 h-4" />
          </div>
          <div className="text-3xl font-light text-white">42<span className="text-lg text-slate-500">%</span></div>
        </div>
        <div className="bg-white/5 border border-white/10 rounded-lg p-4 flex flex-col gap-2">
          <div className="flex justify-between items-center text-slate-400">
            <span className="text-xs uppercase tracking-wider">Memory</span>
            <MemoryStick className="w-4 h-4" />
          </div>
          <div className="text-3xl font-light text-white">1.8<span className="text-lg text-slate-500"> / 16 GB</span></div>
        </div>
        <div className="bg-white/5 border border-white/10 rounded-lg p-4 flex flex-col gap-2">
          <div className="flex justify-between items-center text-slate-400">
            <span className="text-xs uppercase tracking-wider">Network</span>
            <Server className="w-4 h-4" />
          </div>
          <div className="text-3xl font-light text-white">3.2<span className="text-lg text-slate-500"> MB/s</span></div>
        </div>
      </div>

      {/* Process Table */}
      <div className="flex-1 bg-white/5 border border-white/10 rounded-lg overflow-hidden flex flex-col">
        <div className="grid grid-cols-5 gap-4 p-3 border-b border-white/10 bg-black/20 text-xs font-semibold text-slate-400 uppercase tracking-widest">
          <div className="col-span-2">Process Name</div>
          <div>PID</div>
          <div>CPU</div>
          <div>Memory</div>
        </div>
        <div className="flex-1 overflow-y-auto p-2">
          {processes.map(p => (
            <div key={p.id} className="grid grid-cols-5 gap-4 px-3 py-2 text-sm hover:bg-white/5 rounded transition-colors group cursor-default">
              <div className="col-span-2 font-medium text-white group-hover:text-atc-cyan transition-colors">{p.name}</div>
              <div className="text-slate-500 font-mono">{p.id}</div>
              <div className="text-amber-400 font-mono">{p.cpu}</div>
              <div className="text-indigo-400 font-mono">{p.mem}</div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
