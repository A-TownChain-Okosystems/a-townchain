import React, { useState } from "react";
import {
  HardDrive,
  Server,
  Cloud,
  Database,
  BarChart,
  Settings,
  RefreshCw,
  UploadCloud,
  DownloadCloud,
  Activity,
} from "lucide-react";
import { motion } from "motion/react";

export function StorageManagerView() {
  const [drives, setDrives] = useState([
    {
      id: "C:",
      name: "ATC-OS Kernel",
      type: "NVMe Local",
      total: 512,
      used: 124,
      status: "Healthy",
      temp: 38,
    },
    {
      id: "D:",
      name: "ATC Ledger Mem",
      type: "Decentralized",
      total: 4096,
      used: 2048,
      status: "Synced",
      temp: 42,
    },
    {
      id: "E:",
      name: "Swarm Cache",
      type: "IPFS Node",
      total: 10240,
      used: 8192,
      status: "Syncing...",
      temp: 45,
    },
    {
      id: "Z:",
      name: "Cold Storage",
      type: "Encrypted Vault",
      total: 2048,
      used: 10,
      status: "Locked",
      temp: 25,
    },
  ]);

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200 border border-atc-border/50 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center justify-between px-4 h-14 bg-[#090b14] border-b border-atc-border/50 shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-indigo-500/10 border border-indigo-500/30">
            <HardDrive className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h2 className="font-bold text-white tracking-tight leading-tight">
              Datenträgerverwaltung
            </h2>
            <p className="text-[10px] text-slate-400 font-mono">
              System Drives & Decentralized Storage
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <button className="p-2 bg-black/40 hover:bg-white/5 rounded-lg border border-white/5 transition-colors text-slate-300">
            <RefreshCw className="w-4 h-4" />
          </button>
          <button className="px-3 py-1.5 bg-indigo-500 hover:bg-indigo-600 rounded-lg text-white font-bold text-xs tracking-wide transition-colors">
            Format / Mount
          </button>
        </div>
      </div>

      <div className="flex-1 p-6 overflow-y-auto custom-scrollbar">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
          {/* Drive Volumes Summary */}
          <div className="bg-[#090b14]/80 border border-white/5 rounded-2xl p-6 shadow-xl relative overflow-hidden group">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/5 rounded-full blur-[50px]" />
            <h3 className="text-sm font-bold text-slate-300 flex items-center gap-2 mb-6">
              <Database className="w-4 h-4 text-indigo-400" /> Array Capacity
            </h3>

            <div className="flex items-center justify-center w-48 h-48 mx-auto relative mb-4">
              <svg
                viewBox="0 0 100 100"
                className="w-full h-full transform -rotate-90"
              >
                <circle
                  cx="50"
                  cy="50"
                  r="40"
                  stroke="rgba(255,255,255,0.05)"
                  strokeWidth="12"
                  fill="none"
                />
                <circle
                  cx="50"
                  cy="50"
                  r="40"
                  stroke="#6366f1"
                  strokeWidth="12"
                  fill="none"
                  strokeDasharray="251.2"
                  strokeDashoffset={251.2 * (1 - 10374 / 16896)}
                  strokeLinecap="round"
                  className="drop-shadow-[0_0_8px_rgba(99,102,241,0.5)]"
                />
              </svg>
              <div className="absolute inset-0 flex flex-col items-center justify-center">
                <span className="text-2xl font-bold text-white tracking-tighter">
                  61%
                </span>
                <span className="text-[10px] text-slate-500 uppercase tracking-widest">
                  Used
                </span>
              </div>
            </div>
            <div className="flex justify-between text-xs font-mono text-slate-400 mt-4 px-4">
              <div className="flex flex-col items-center">
                <span className="text-white">10.37 TB</span>
                <span className="text-[10px]">Allocated</span>
              </div>
              <div className="flex flex-col items-center">
                <span className="text-indigo-400">16.89 TB</span>
                <span className="text-[10px]">Total</span>
              </div>
            </div>
          </div>

          {/* Metrics */}
          <div className="flex flex-col gap-4">
            <div className="flex-1 bg-black/40 border border-white/5 rounded-2xl p-5 shadow-xl flex flex-col justify-center relative overflow-hidden">
              <Activity className="absolute right-4 top-4 w-24 h-24 text-white/5" />
              <h4 className="text-xs font-bold text-slate-400 mb-2 uppercase tracking-wider">
                I/O Performance
              </h4>
              <div className="flex gap-8 mt-2">
                <div>
                  <div className="text-2xl font-black text-emerald-400 tracking-tighter">
                    2,450
                  </div>
                  <div className="text-xs text-slate-500 flex items-center gap-1 font-mono mt-1">
                    <UploadCloud className="w-3 h-3" /> MB/s Read
                  </div>
                </div>
                <div>
                  <div className="text-2xl font-black text-rose-400 tracking-tighter">
                    1,210
                  </div>
                  <div className="text-xs text-slate-500 flex items-center gap-1 font-mono mt-1">
                    <DownloadCloud className="w-3 h-3" /> MB/s Write
                  </div>
                </div>
              </div>
            </div>

            <div className="flex-1 bg-black/40 border border-white/5 rounded-2xl p-5 shadow-xl">
              <h4 className="text-xs font-bold text-slate-400 mb-4 uppercase tracking-wider">
                System Health
              </h4>
              <div className="space-y-3">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-300">Disk Integrity</span>
                  <span className="font-mono text-emerald-400">100% OK</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-300">SMART Status</span>
                  <span className="font-mono text-emerald-400">Passed</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-300">Network Raids</span>
                  <span className="font-mono text-amber-400">
                    Degraded (1 offline)
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <h3 className="text-sm font-bold text-white mb-4 border-b border-white/10 pb-2">
          Mounted Volumes
        </h3>
        <div className="flex flex-col gap-3">
          {drives.map((drive) => (
            <div
              key={drive.id}
              className="bg-[#0b0e1a] border border-white/5 rounded-xl p-4 flex flex-col sm:flex-row items-center gap-6 hover:bg-[#0d1120] transition-colors"
            >
              <div className="flex items-center gap-4 w-48 shrink-0">
                <div className="w-10 h-10 rounded-lg bg-indigo-500/10 flex items-center justify-center border border-indigo-500/20">
                  {drive.type.includes("Local") ? (
                    <HardDrive className="w-5 h-5 text-indigo-400" />
                  ) : (
                    <Cloud className="w-5 h-5 text-emerald-400" />
                  )}
                </div>
                <div>
                  <div className="font-bold text-white text-sm">
                    {drive.id} {drive.name}
                  </div>
                  <div className="text-[10px] uppercase font-mono text-slate-500">
                    {drive.type}
                  </div>
                </div>
              </div>

              <div className="flex-1 w-full">
                <div className="flex justify-between text-xs mb-1.5 font-mono">
                  <span className="text-slate-300">{drive.used} GB used</span>
                  <span className="text-slate-500">{drive.total} GB total</span>
                </div>
                <div className="h-2 bg-black rounded-full overflow-hidden border border-white/5">
                  <motion.div
                    initial={{ width: 0 }}
                    animate={{ width: `${(drive.used / drive.total) * 100}%` }}
                    className={`h-full ${drive.used / drive.total > 0.8 ? "bg-rose-500" : "bg-indigo-500"} rounded-full`}
                  />
                </div>
              </div>

              <div className="flex items-center gap-6 shrink-0 text-xs font-mono w-48 justify-end">
                <div className="flex flex-col items-end">
                  <span className="text-slate-500">Temp</span>
                  <span
                    className={`${drive.temp > 40 ? "text-amber-400" : "text-emerald-400"}`}
                  >
                    {drive.temp}°C
                  </span>
                </div>
                <div className="flex flex-col items-end">
                  <span className="text-slate-500">Status</span>
                  <span
                    className={
                      drive.status === "Locked"
                        ? "text-rose-400"
                        : "text-emerald-400"
                    }
                  >
                    {drive.status}
                  </span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
