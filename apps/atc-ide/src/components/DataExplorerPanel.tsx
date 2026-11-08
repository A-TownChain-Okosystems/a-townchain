import React, { useState } from "react";
import {
  Database,
  Search,
  Filter,
  Server,
  HardDrive,
  FileJson,
  Table,
  RefreshCw,
  Download,
  ChevronRight,
} from "lucide-react";

export function DataExplorerPanel() {
  const [activeTab, setActiveTab] = useState<"tables" | "files">("tables");

  const tables = [
    { name: "users", rows: 12450, size: "2.4 MB", updated: "vor 5 Min" },
    {
      name: "transactions",
      rows: 890200,
      size: "145 MB",
      updated: "vor 1 Min",
    },
    { name: "logs", rows: 120500, size: "45 MB", updated: "vor 12 Min" },
    { name: "settings", rows: 45, size: "12 KB", updated: "vor 2 Std" },
  ];

  const files = [
    {
      name: "config_backup_v1.json",
      type: "JSON",
      size: "150 KB",
      date: "12.05.2026",
    },
    {
      name: "training_data_batch1.csv",
      type: "CSV",
      size: "1.2 GB",
      date: "11.05.2026",
    },
    {
      name: "model_weights_epoch10.bin",
      type: "BIN",
      size: "4.5 GB",
      date: "10.05.2026",
    },
  ];

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0c0c0e]">
      <div className="p-6 border-b border-white/10 flex items-center justify-between bg-black/20">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-emerald-500/10 rounded-lg">
            <Database className="w-5 h-5 text-emerald-400" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-slate-200">Daten Explorer</h2>
            <p className="text-sm text-slate-500 mt-1">
              Verwalte Datenbanken, Tabellen und Raw Data Files.
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <button className="p-2 rounded bg-white/5 hover:bg-white/10 text-slate-400 transition-colors">
            <RefreshCw className="w-4 h-4" />
          </button>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar */}
        <div className="w-64 border-r border-white/10 bg-black/20 flex flex-col">
          <div className="p-4 border-b border-white/10">
            <div className="relative">
              <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
              <input
                type="text"
                placeholder="Suchen..."
                className="w-full bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-slate-200 outline-none focus:border-emerald-500/50"
              />
            </div>
          </div>
          <div className="flex-1 overflow-y-auto p-2 space-y-1">
            <button
              onClick={() => setActiveTab("tables")}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm transition-colors ${activeTab === "tables" ? "bg-emerald-500/10 text-emerald-400 font-bold" : "text-slate-400 hover:bg-white/5 hover:text-slate-200"}`}
            >
              <Server className="w-4 h-4" /> Relationale DBs
            </button>
            <button
              onClick={() => setActiveTab("files")}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm transition-colors ${activeTab === "files" ? "bg-emerald-500/10 text-emerald-400 font-bold" : "text-slate-400 hover:bg-white/5 hover:text-slate-200"}`}
            >
              <HardDrive className="w-4 h-4" /> Object Storage
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 p-6 overflow-y-auto">
          {activeTab === "tables" && (
            <div className="space-y-4">
              <h3 className="text-sm font-bold text-slate-500 uppercase tracking-widest flex items-center gap-2 mb-4">
                <Table className="w-4 h-4" /> Aktive Tabellen
              </h3>
              <div className="bg-black/40 border border-white/10 rounded-xl overflow-hidden">
                <table className="w-full text-left text-sm">
                  <thead>
                    <tr className="bg-white/[0.02] border-b border-white/10">
                      <th className="px-4 py-3 font-medium text-slate-400">
                        Tabellenname
                      </th>
                      <th className="px-4 py-3 font-medium text-slate-400">
                        Einträge
                      </th>
                      <th className="px-4 py-3 font-medium text-slate-400">
                        Größe
                      </th>
                      <th className="px-4 py-3 font-medium text-slate-400">
                        Letztes Update
                      </th>
                      <th className="px-4 py-3 font-medium text-slate-400"></th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5">
                    {tables.map((t) => (
                      <tr
                        key={t.name}
                        className="hover:bg-white/[0.02] transition-colors group cursor-pointer"
                      >
                        <td className="px-4 py-3 font-mono text-emerald-400">
                          {t.name}
                        </td>
                        <td className="px-4 py-3 text-slate-300">
                          {t.rows.toLocaleString()}
                        </td>
                        <td className="px-4 py-3 text-slate-300">{t.size}</td>
                        <td className="px-4 py-3 text-slate-500">
                          {t.updated}
                        </td>
                        <td className="px-4 py-3 text-right">
                          <ChevronRight className="w-4 h-4 text-slate-600 group-hover:text-emerald-400 ml-auto" />
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {activeTab === "files" && (
            <div className="space-y-4">
              <h3 className="text-sm font-bold text-slate-500 uppercase tracking-widest flex items-center gap-2 mb-4">
                <FileJson className="w-4 h-4" /> Dateisystem
              </h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {files.map((f) => (
                  <div
                    key={f.name}
                    className="bg-black/40 border border-white/10 rounded-xl p-4 hover:border-emerald-500/30 transition-colors group flex flex-col justify-between"
                  >
                    <div className="flex items-start justify-between mb-4">
                      <div className="p-2 bg-white/5 rounded-lg text-slate-400 group-hover:bg-emerald-500/10 group-hover:text-emerald-400 transition-colors">
                        <FileJson className="w-5 h-5" />
                      </div>
                      <span className="text-[10px] font-bold px-2 py-1 bg-white/5 text-slate-500 rounded uppercase">
                        {f.type}
                      </span>
                    </div>
                    <div>
                      <h4
                        className="text-sm font-bold text-slate-200 truncate mb-1"
                        title={f.name}
                      >
                        {f.name}
                      </h4>
                      <div className="text-xs text-slate-500 flex justify-between">
                        <span>{f.size}</span>
                        <span>{f.date}</span>
                      </div>
                    </div>
                    <div className="mt-4 pt-3 border-t border-white/5 flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                      <button className="flex-1 bg-white/5 hover:bg-emerald-500/20 hover:text-emerald-400 text-slate-300 text-xs py-1.5 rounded transition-colors flex justify-center items-center gap-1">
                        <Download className="w-3.5 h-3.5" /> Download
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
