import React, { useState } from "react";
import {
  Database,
  Search,
  FileCode,
  Plus,
  Download,
  Terminal,
  Clock,
} from "lucide-react";

export function ScriptDatabasePanel() {
  const [searchQuery, setSearchQuery] = useState("");

  const scripts = [
    {
      id: "s1",
      name: "Database Backup",
      type: "Bash",
      runtime: "Node",
      executions: 124,
      lastRun: "vor 2 Stunden",
    },
    {
      id: "s2",
      name: "Auto Deploy",
      type: "YAML",
      runtime: "CI/CD",
      executions: 856,
      lastRun: "vor 5 Minuten",
    },
    {
      id: "s3",
      name: "Data Cleanup",
      type: "Python",
      runtime: "KAI-OS",
      executions: 42,
      lastRun: "vor 1 Tag",
    },
    {
      id: "s4",
      name: "Stats Generator",
      type: "TypeScript",
      runtime: "Node",
      executions: 15,
      lastRun: "vor 3 Tagen",
    },
  ];

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0c0c0e]">
      <div className="p-6 border-b border-white/10 flex items-center justify-between bg-black/20">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-orange-500/10 rounded-lg">
            <FileCode className="w-5 h-5 text-orange-400" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-slate-200">
              Skript Datenbank
            </h2>
            <p className="text-sm text-slate-500 mt-1">
              Automatisierungsskripte, Cronjobs und Helper-Scripts.
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <div className="relative">
            <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              type="text"
              placeholder="Skripte suchen..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-64 bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-slate-200 outline-none focus:border-orange-500/50"
            />
          </div>
          <button className="flex items-center gap-2 px-4 py-2 bg-orange-500 hover:bg-orange-600 text-white rounded-lg font-bold text-sm transition-colors">
            <Plus className="w-4 h-4" /> Neu
          </button>
        </div>
      </div>

      <div className="flex-1 p-6 overflow-y-auto">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {scripts.map((script) => (
            <div
              key={script.id}
              className="bg-black/40 border border-white/10 rounded-xl p-5 hover:border-orange-500/30 transition-colors group flex flex-col"
            >
              <div className="flex justify-between items-start mb-4">
                <div className="p-2 bg-white/5 rounded-lg text-orange-400">
                  <Terminal className="w-5 h-5" />
                </div>
                <span className="text-xs font-medium px-2 py-1 bg-white/5 text-slate-400 rounded-full">
                  {script.type}
                </span>
              </div>

              <h3 className="font-bold text-slate-200 text-lg mb-2">
                {script.name}
              </h3>

              <div className="bg-black/40 rounded p-2 text-xs font-mono text-slate-400 mb-4 border border-white/5">
                Runtime:{" "}
                <span className="text-slate-300">{script.runtime}</span>
              </div>

              <div className="flex items-center justify-between text-xs text-slate-500 mt-auto pt-4 border-t border-white/5">
                <span className="flex items-center gap-1">
                  <Clock className="w-3.5 h-3.5" /> Letzter Run:{" "}
                  {script.lastRun}
                </span>
                <span>{script.executions} Execs</span>
              </div>

              <div className="mt-4 pt-4 border-t border-white/5 opacity-0 group-hover:opacity-100 transition-opacity">
                <button className="w-full flex items-center justify-center gap-2 bg-white/5 hover:bg-orange-500/20 hover:text-orange-400 text-slate-300 py-2 rounded-lg text-sm font-bold transition-colors">
                  <Terminal className="w-4 h-4" /> Skript ausführen
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
