import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { X, Github, FileText, CheckCircle, AlertCircle, Clock, GitMerge, ChevronDown, ChevronUp } from 'lucide-react';

interface SyncLog {
  id: string;
  source: 'GitHub' | 'Notion';
  action: string;
  status: 'Success' | 'Error' | 'Pending' | 'Conflict';
  timestamp: string;
  diff?: {
    property: string;
    githubLines: string[];
    notionLines: string[];
  };
}

const MOCK_LOGS: SyncLog[] = Array.from({ length: 20 }).map((_, i) => {
  const source: 'GitHub' | 'Notion' = Math.random() > 0.5 ? 'GitHub' : 'Notion';
  const rand = Math.random();
  const status: 'Success' | 'Error' | 'Pending' | 'Conflict' = rand > 0.4 ? 'Success' : rand > 0.2 ? 'Conflict' : rand > 0.1 ? 'Pending' : 'Error';
  
  const sources = {
    GitHub: ['Fetched open issues', 'Synced milestones', 'Pulled latest commits', 'Updated PR status'],
    Notion: ['Synced roadmap database', 'Updated task board', 'Fetched sprint details', 'Synced project notes']
  };

  const actionList = sources[source];
  const action = actionList[Math.floor(Math.random() * actionList.length)];
  
  const now = new Date();
  now.setMinutes(now.getMinutes() - Math.floor(Math.random() * 600));

  let diff;
  if (status === 'Conflict') {
    diff = {
      property: 'Issue Description / Notes',
      githubLines: [
        '- Update main header for mobile view',
        '- Changed network layer typing to strict mode',
        '- Fixed minor typo in readme'
      ],
      notionLines: [
        '- Refactored header responsiveness',
        '- Network layer typings updated',
        '- Needs further review on mobile layout'
      ]
    };
  }

  return {
    id: `log-${i}`,
    source,
    action,
    status,
    timestamp: now.toISOString(),
    diff
  };
}).sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime());

interface SyncHistoryModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SyncHistoryModal({ isOpen, onClose }: SyncHistoryModalProps) {
  const [logs, setLogs] = useState<SyncLog[]>([]);
  const [filter, setFilter] = useState<'All' | 'GitHub' | 'Notion'>('All');

  useEffect(() => {
    if (isOpen) {
      // Simulate fetch
      setLogs(MOCK_LOGS);
    }
  }, [isOpen]);

  const filteredLogs = logs.filter(log => filter === 'All' || log.source === filter);

  const getStatusIcon = (status: SyncLog['status']) => {
    switch (status) {
      case 'Success': return <CheckCircle className="w-4 h-4 text-emerald-400" />;
      case 'Error': return <AlertCircle className="w-4 h-4 text-red-400" />;
      case 'Pending': return <Clock className="w-4 h-4 text-amber-400" />;
      case 'Conflict': return <GitMerge className="w-4 h-4 text-indigo-400" />;
    }
  };

  const [expandedLogId, setExpandedLogId] = useState<string | null>(null);

  const toggleExpand = (id: string) => {
    setExpandedLogId(prev => (prev === id ? null : id));
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
          <motion.div
            initial={{ opacity: 0, scale: 0.95, y: 20 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: 20 }}
            className="w-full max-w-2xl bg-[#0a0d16] border border-white/10 rounded-2xl shadow-2xl flex flex-col"
            style={{ maxHeight: '80vh' }}
          >
            {/* Header */}
            <div className="flex items-center justify-between p-6 border-b border-white/5">
              <div>
                <h2 className="text-xl font-bold text-white flex items-center gap-2">
                  <Clock className="w-5 h-5 text-indigo-400" />
                  API Sync History
                </h2>
                <p className="text-sm text-slate-400 mt-1">Last 20 synchronization events from external services.</p>
              </div>
              <button 
                onClick={onClose}
                className="p-2 text-slate-400 hover:text-white hover:bg-white/5 rounded-lg transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Filters */}
            <div className="px-6 py-4 border-b border-white/5 flex gap-2">
              <button
                onClick={() => setFilter('All')}
                className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors ${filter === 'All' ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30' : 'bg-white/5 text-slate-400 border border-transparent hover:bg-white/10'}`}
              >
                All
              </button>
              <button
                onClick={() => setFilter('GitHub')}
                className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors flex items-center gap-1.5 ${filter === 'GitHub' ? 'bg-slate-700 text-white border border-slate-500' : 'bg-white/5 text-slate-400 border border-transparent hover:bg-white/10'}`}
              >
                <Github className="w-4 h-4" />
                GitHub
              </button>
              <button
                onClick={() => setFilter('Notion')}
                className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors flex items-center gap-1.5 ${filter === 'Notion' ? 'bg-orange-500/20 text-orange-300 border border-orange-500/30' : 'bg-white/5 text-slate-400 border border-transparent hover:bg-white/10'}`}
              >
                <FileText className="w-4 h-4" />
                Notion
              </button>
            </div>

            {/* Logs List */}
            <div className="flex-1 overflow-y-auto p-4 space-y-2">
              <AnimatePresence mode="popLayout">
                {filteredLogs.map(log => (
                  <motion.div
                    key={log.id}
                    layout
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, scale: 0.95 }}
                    transition={{ duration: 0.2 }}
                    className="p-3 rounded-xl bg-white/[0.02] border border-white/5 hover:bg-white/[0.04] transition-colors overflow-hidden"
                  >
                    <div 
                      className={`flex justify-between items-center ${log.status === 'Conflict' ? 'cursor-pointer' : ''}`}
                      onClick={() => log.status === 'Conflict' && toggleExpand(log.id)}
                    >
                      <div className="flex items-center gap-4">
                        <div className={`p-2 rounded-lg ${log.source === 'GitHub' ? 'bg-slate-800' : 'bg-orange-500/10'}`}>
                          {log.source === 'GitHub' ? <Github className="w-4 h-4 text-slate-300" /> : <FileText className="w-4 h-4 text-orange-400" />}
                        </div>
                        <div>
                          <p className="text-sm font-medium text-slate-200">{log.action}</p>
                          <p className="text-xs text-slate-500 font-mono mt-0.5">{new Date(log.timestamp).toLocaleString()}</p>
                        </div>
                      </div>
                      <div className="flex items-center gap-2">
                        <span className={`text-xs font-mono px-2 py-1 rounded-md border flex items-center gap-1.5 ${
                          log.status === 'Success' ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20' :
                          log.status === 'Error' ? 'bg-red-500/10 text-red-400 border-red-500/20' :
                          log.status === 'Conflict' ? 'bg-indigo-500/10 text-indigo-400 border-indigo-500/20' :
                          'bg-amber-500/10 text-amber-400 border-amber-500/20'
                        }`}>
                          {getStatusIcon(log.status)}
                          {log.status}
                        </span>
                        {log.status === 'Conflict' && (
                          <div className="ml-2 text-slate-500 text-xs">
                            {expandedLogId === log.id ? <ChevronUp className="w-4 h-4" /> : <ChevronDown className="w-4 h-4" />}
                          </div>
                        )}
                      </div>
                    </div>
                    
                    <AnimatePresence>
                      {log.status === 'Conflict' && expandedLogId === log.id && log.diff && (
                        <motion.div
                          initial={{ height: 0, opacity: 0 }}
                          animate={{ height: 'auto', opacity: 1 }}
                          exit={{ height: 0, opacity: 0 }}
                          className="mt-4 pt-4 border-t border-white/5"
                        >
                          <div className="flex items-center justify-between mb-2">
                            <span className="text-xs font-bold font-mono tracking-wider text-slate-400 uppercase">Conflict Detected: {log.diff.property}</span>
                            <span className="text-[10px] text-slate-500">Manual resolution required</span>
                          </div>
                          <div className="grid grid-cols-2 gap-4 mt-3">
                            <div className="bg-slate-900/50 rounded-lg p-3 border border-slate-700/50">
                              <div className="flex items-center gap-2 mb-2 pb-2 border-b border-white/5 text-xs text-slate-300 font-bold">
                                <Github className="w-3 h-3" /> GitHub Version
                              </div>
                              <div className="font-mono text-xs text-red-400 space-y-1">
                                {log.diff.githubLines.map((line, idx) => (
                                  <div key={idx} className="bg-red-500/10 px-2 py-1 rounded">- {line}</div>
                                ))}
                              </div>
                            </div>
                            <div className="bg-orange-900/10 rounded-lg p-3 border border-orange-500/20">
                              <div className="flex items-center gap-2 mb-2 pb-2 border-b border-orange-500/20 text-xs text-orange-300 font-bold">
                                <FileText className="w-3 h-3" /> Notion Version
                              </div>
                              <div className="font-mono text-xs text-emerald-400 space-y-1">
                                {log.diff.notionLines.map((line, idx) => (
                                  <div key={idx} className="bg-emerald-500/10 px-2 py-1 rounded">+ {line}</div>
                                ))}
                              </div>
                            </div>
                          </div>
                          <div className="flex justify-end gap-2 mt-4">
                            <button className="px-3 py-1.5 text-xs font-medium rounded border border-slate-600 text-slate-300 hover:bg-slate-800 transition-colors">
                              Keep GitHub
                            </button>
                            <button className="px-3 py-1.5 text-xs font-medium rounded border border-orange-500/30 text-orange-300 bg-orange-500/10 hover:bg-orange-500/20 transition-colors">
                              Keep Notion
                            </button>
                          </div>
                        </motion.div>
                      )}
                    </AnimatePresence>
                  </motion.div>
                ))}
                {filteredLogs.length === 0 && (
                  <div className="py-10 text-center text-slate-500 font-mono text-sm">
                    No logs found for selected filter.
                  </div>
                )}
              </AnimatePresence>
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
}
