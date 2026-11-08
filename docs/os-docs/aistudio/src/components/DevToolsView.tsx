import React, { useState } from "react";
import {
  Terminal,
  Code,
  Cpu,
  Network,
  Search,
  Play,
  Pause,
  Square,
  Bug,
} from "lucide-react";

export function DevToolsView() {
  const [logs, setLogs] = useState([
    "[Network] Intercepted Request to contract 0xabc123... (Gas: 2400)",
    "[VM] JIT compiled subroutine 'auth_layer' in 2.1ms",
    "[P2P] Discovered 3 new nodes in local subnet",
    "[Warning] Deprecated usage of system API at module_x2.ats line 45",
  ]);

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200 border border-atc-border/50 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center justify-between px-4 h-14 bg-[#090b14] border-b border-atc-border/50 shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-emerald-500/10 border border-emerald-500/30">
            <Code className="w-5 h-5 text-emerald-400" />
          </div>
          <div>
            <h2 className="font-bold text-white tracking-tight leading-tight">
              ATC Developer Tools
            </h2>
            <p className="text-[10px] text-slate-400 font-mono">
              Kernel Access & Network Profiling
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <div className="flex items-center px-3 py-1.5 bg-black/40 border border-white/5 rounded-lg">
            <div className="w-2 h-2 rounded-full bg-emerald-500 mr-2 animate-pulse" />
            <span className="text-[10px] font-mono text-slate-300">
              Live Profiling
            </span>
          </div>
        </div>
      </div>

      <div className="flex-1 flex flex-col md:flex-row overflow-hidden">
        {/* Sidebar Tools */}
        <div className="w-full md:w-56 bg-[#090b14] border-r border-white/5 flex flex-col p-2 space-y-1">
          {[
            "Inspector",
            "Network Hooks",
            "Performance",
            "Memory Heap",
            "Local Storage",
            "Debug Console",
          ].map((tool, idx) => (
            <button
              key={idx}
              className={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors ${idx === 0 ? "bg-indigo-500/20 text-indigo-300 font-medium" : "text-slate-400 hover:bg-white/5 hover:text-slate-200"}`}
            >
              {tool}
            </button>
          ))}
        </div>

        {/* Main Content Area */}
        <div className="flex-1 flex flex-col overflow-hidden bg-black/20">
          <div className="h-10 border-b border-white/5 flex items-center px-4 bg-black/40 gap-4 shrink-0">
            <button className="text-slate-400 hover:text-white">
              <Play className="w-4 h-4" />
            </button>
            <button className="text-slate-400 hover:text-white">
              <Pause className="w-4 h-4" />
            </button>
            <button className="text-slate-400 hover:text-white text-emerald-400">
              <Square className="w-4 h-4 fill-current" />
            </button>
            <div className="h-4 w-px bg-white/10 mx-2" />
            <div className="flex items-center gap-2 text-xs font-mono text-slate-500 relative flex-1">
              <Search className="w-3 h-3 absolute left-2" />
              <input
                type="text"
                placeholder="Filter by regex..."
                className="bg-[#090b14] border border-white/10 rounded px-2 w-full py-1 pl-7 outline-none text-slate-300"
              />
            </div>
          </div>

          <div className="flex-1 flex flex-col p-4 overflow-y-auto custom-scrollbar font-mono text-xs gap-3">
            <div className="text-indigo-400 font-bold mb-2">
              &gt; ATC-OS Profiler Attached (PID 0)
            </div>

            <div className="grid grid-cols-2 gap-4 mb-4 select-none">
              <div className="border border-white/5 bg-[#090b14] rounded p-3">
                <div className="text-slate-500 mb-1">Heap Used</div>
                <div className="text-xl text-emerald-400 tracking-tighter">
                  184 MB{" "}
                  <span className="text-[10px] text-slate-600">/ 512 MB</span>
                </div>
              </div>
              <div className="border border-white/5 bg-[#090b14] rounded p-3">
                <div className="text-slate-500 mb-1">Live Objects</div>
                <div className="text-xl text-amber-400 tracking-tighter">
                  14,392
                </div>
              </div>
            </div>

            <div className="bg-[#090b14] border border-white/5 rounded flex-1 overflow-y-auto p-3 flex flex-col gap-1.5 custom-scrollbar">
              {logs.map((log, i) => (
                <div
                  key={i}
                  className={`py-1 ${log.includes("Warning") ? "text-amber-400" : "text-slate-300"} border-b border-white/5 border-dashed last:border-0`}
                >
                  <span className="text-slate-600 mr-2">
                    [{new Date().toISOString().split("T")[1].slice(0, -1)}]
                  </span>
                  {log}
                </div>
              ))}
              <div className="flex items-center gap-2 mt-2 text-slate-500">
                <span className="animate-pulse">_</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
