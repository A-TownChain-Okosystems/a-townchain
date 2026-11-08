import React from 'react';
import { useSyncMetrics } from '../contexts/SyncMetricsContext';
import { X, HardDrive, Database, FileText, CalendarDays, CheckCircle2, XCircle } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export function SyncDashboardModal({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) {
  const { syncHistory } = useSyncMetrics();

  const services = [
    { id: 'drive', name: 'Google Drive', icon: HardDrive, color: 'text-blue-400' },
    { id: 'sheets', name: 'Google Sheets', icon: Database, color: 'text-emerald-400' },
    { id: 'docs', name: 'Google Docs', icon: FileText, color: 'text-indigo-400' },
    { id: 'calendar', name: 'Google Calendar', icon: CalendarDays, color: 'text-orange-400' }
  ] as const;

  return (
    <AnimatePresence>
      {isOpen && (
        <React.Fragment>
          <motion.div 
            initial={{ opacity: 0 }} 
            animate={{ opacity: 1 }} 
            exit={{ opacity: 0 }} 
            className="fixed inset-0 z-[100] bg-black/60 backdrop-blur-sm"
            onClick={onClose}
          />
          <motion.div 
            initial={{ opacity: 0, scale: 0.95, y: 20 }} 
            animate={{ opacity: 1, scale: 1, y: 0 }} 
            exit={{ opacity: 0, scale: 0.95, y: 20 }} 
            className="fixed inset-0 z-[101] flex items-center justify-center p-4 pointer-events-none"
          >
            <div className="bg-[#0a0d16] border border-white/10 rounded-xl w-full max-w-3xl max-h-[85vh] flex flex-col shadow-2xl overflow-hidden pointer-events-auto">
              <div className="flex items-center justify-between p-4 border-b border-white/10 bg-white/5">
                <h2 className="text-xl font-semibold text-white tracking-tight">Synchronization Dashboard</h2>
                <button onClick={onClose} className="p-1.5 hover:bg-white/10 rounded-lg text-slate-400 hover:text-white transition-colors">
                  <X className="w-5 h-5" />
                </button>
              </div>
              
              <div className="flex-1 overflow-y-auto p-6 bg-[#0a0d16]">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  {services.map(service => {
                    const history = syncHistory.filter(h => h.service === service.id);
                    return (
                      <div key={service.id} className="border border-white/10 rounded-xl bg-[#111625] overflow-hidden flex flex-col">
                        <div className="p-4 border-b border-white/5 bg-white/[0.02] flex items-center gap-3">
                          <div className={`w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center ${service.color}`}>
                            <service.icon className="w-4 h-4" />
                          </div>
                          <h3 className="text-sm font-medium text-slate-200">{service.name}</h3>
                        </div>
                        <div className="p-4 flex-1 flex flex-col gap-2 min-h-[160px] max-h-[240px] overflow-y-auto">
                          {history.length > 0 ? (
                            history.slice().reverse().map((entry, idx) => (
                              <div key={idx} className="flex items-center justify-between text-sm bg-black/40 p-2.5 rounded-lg border border-white/5">
                                <div className="flex items-center gap-2.5">
                                  {entry.status === 'success' ? (
                                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                                  ) : (
                                    <XCircle className="w-4 h-4 text-red-500" />
                                  )}
                                  <span className="text-slate-300 text-xs">
                                    {new Date(entry.timestamp).toLocaleString([], { dateStyle: 'short', timeStyle: 'medium' })}
                                  </span>
                                </div>
                                <span className="text-slate-500 font-mono text-[10px]">{entry.latency}ms</span>
                              </div>
                            ))
                          ) : (
                            <div className="flex-1 flex flex-col items-center justify-center text-slate-500 gap-2 opacity-60">
                              <service.icon className="w-6 h-6 mb-1" />
                              <p className="text-xs font-mono">No sync history</p>
                            </div>
                          )}
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            </div>
          </motion.div>
        </React.Fragment>
      )}
    </AnimatePresence>
  );
}
