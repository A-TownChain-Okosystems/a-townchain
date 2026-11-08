import React, { useState } from 'react';
import { Activity, ServerCrash, CheckCircle, AlertTriangle } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export function ApiHealthWidget() {
  const [isOpen, setIsOpen] = useState(false);

  const endpoints = [
    { name: "ATC Gateway", status: "online", ms: 14 },
    { name: "RPC Node M1", status: "online", ms: 28 },
    { name: "Index Service", status: "degraded", ms: 450 },
    { name: "Auth Provider", status: "offline", ms: 0 },
  ];

  const overallStatus = endpoints.some(e => e.status === "offline") 
    ? "error" 
    : endpoints.some(e => e.status === "degraded") 
      ? "warning" 
      : "ok";

  return (
    <div 
      className="relative api-health-widget"
      onMouseEnter={() => setIsOpen(true)}
      onMouseLeave={() => setIsOpen(false)}
    >
      <div className="flex items-center gap-2 px-3 py-1.5 bg-black/40 border border-white/5 rounded-full cursor-pointer hover:border-white/20 transition-colors">
        <Activity className={`w-3.5 h-3.5 ${
          overallStatus === 'error' ? 'text-red-400' : 
          overallStatus === 'warning' ? 'text-amber-400' : 'text-emerald-400'
        }`} />
        <span className="text-xs font-mono font-medium text-slate-300">API</span>
        <div className="flex gap-0.5 mt-0.5">
          {endpoints.map((ep, i) => (
             <div 
               key={i} 
               className={`w-1.5 h-1.5 rounded-full ${
                 ep.status === 'online' ? 'bg-emerald-500' :
                 ep.status === 'degraded' ? 'bg-amber-500' : 'bg-red-500'
               }`} 
             />
          ))}
        </div>
      </div>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 10 }}
            className="absolute top-full right-0 mt-3 w-64 p-3 bg-[#090b14]/95 backdrop-blur-xl border border-white/10 shadow-2xl rounded-xl z-50 pointer-events-none"
          >
            <div className="flex items-center justify-between mb-3 border-b border-white/10 pb-2">
               <span className="text-xs font-mono uppercase tracking-widest text-slate-400">Endpoint Health</span>
               {overallStatus === 'ok' ? <CheckCircle className="w-4 h-4 text-emerald-500" /> : <AlertTriangle className="w-4 h-4 text-amber-500" />}
            </div>
            <div className="flex flex-col gap-2">
              {endpoints.map((ep, i) => (
                <div key={i} className="flex flex-col gap-1">
                  <div className="flex items-center justify-between">
                    <span className="text-xs font-medium text-slate-200">{ep.name}</span>
                    <span className={`text-[10px] uppercase font-bold tracking-wider ${
                      ep.status === 'online' ? 'text-emerald-500' :
                      ep.status === 'degraded' ? 'text-amber-500' : 'text-red-500'
                    }`}>{ep.status}</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="flex-1 h-1 bg-black/60 rounded-full overflow-hidden">
                       <div 
                         className={`h-full ${ep.status === 'online' ? 'bg-emerald-500' : ep.status === 'degraded' ? 'bg-amber-500' : 'bg-red-500'}`} 
                         style={{ width: ep.status === 'offline' ? '0%' : ep.status === 'degraded' ? '40%' : '100%' }}
                       />
                    </div>
                    <span className="text-[10px] font-mono text-slate-500 w-8 text-right">{ep.status === 'offline' ? '--' : `${ep.ms}ms`}</span>
                  </div>
                </div>
              ))}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
