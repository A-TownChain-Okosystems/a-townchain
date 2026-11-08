import React, { useState, useEffect, useRef } from 'react';
import { Terminal, AlertTriangle, Info, ShieldAlert } from 'lucide-react';

export interface LogEntry {
  id: string;
  timestamp: Date;
  level: 'info' | 'warn' | 'error' | 'critical';
  message: string;
  source: string;
}

export function SystemLogsView() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const logsEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Auto scroll
    logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]);

  useEffect(() => {
    const fetchLogs = async () => {
      try {
        const res = await fetch('/api/system/logs');
        if (res.ok) {
          const data = await res.json();
          // parse timestamps back to date objects
          setLogs(data.map((l: any) => ({ ...l, timestamp: new Date(l.timestamp) })));
        }
      } catch(e) { console.error(e); }
    };
    
    fetchLogs();
    const interval = setInterval(fetchLogs, 2500);

    return () => clearInterval(interval);
  }, []);

  const getLevelColor = (level: string) => {
    switch(level) {
      case 'info': return 'text-cyan-400';
      case 'warn': return 'text-amber-400';
      case 'error': return 'text-rose-400';
      case 'critical': return 'text-red-500 font-bold';
      default: return 'text-slate-400';
    }
  };

  const getLevelIcon = (level: string) => {
    switch(level) {
      case 'warn': return <AlertTriangle className="w-3 h-3 text-amber-400" />;
      case 'error': return <ShieldAlert className="w-3 h-3 text-rose-400" />;
      case 'critical': return <ShieldAlert className="w-3 h-3 text-red-500" />;
      default: return <Info className="w-3 h-3 text-cyan-400" />;
    }
  };

  return (
    <div className="bg-[#0b0f19] border border-white/10 rounded-2xl p-6 shadow-2xl flex flex-col gap-4 h-[400px]">
      <div className="flex items-center gap-2 border-b border-white/10 pb-4">
        <Terminal className="w-5 h-5 text-indigo-400" />
        <h2 className="text-xl font-mono text-white">System Logs</h2>
      </div>
      
      <div className="flex-1 overflow-y-auto custom-scrollbar font-mono text-xs flex flex-col gap-1 pr-2">
        {logs.map(log => (
          <div key={log.id} className="flex gap-3 py-1 hover:bg-white/5 px-2 rounded transition-colors group">
            <span className="text-slate-500 shrink-0 select-none">
              [{log.timestamp.toLocaleTimeString()}]
            </span>
            <span className="shrink-0 flex items-center justify-center w-4 mt-[1px]">
              {getLevelIcon(log.level)}
            </span>
            <span className={`shrink-0 w-16 uppercase ${getLevelColor(log.level)}`}>
              {log.level}
            </span>
            <span className="text-slate-400 shrink-0 w-24 truncate" title={log.source}>
              [{log.source}]
            </span>
            <span className="text-slate-300 group-hover:text-white transition-colors break-words">
              {log.message}
            </span>
          </div>
        ))}
        <div ref={logsEndRef} />
      </div>
    </div>
  );
}
