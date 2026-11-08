import React, { useState, useEffect, useRef, useMemo } from 'react';
import { Activity, GitCommit, GitBranch, GitPullRequest, GitMerge, CheckCircle2, Clock, RefreshCw, AlertCircle, Github, PlayCircle, CheckCircle, XCircle, Loader2, BarChart, TrendingUp, Zap, X, Wand2 } from 'lucide-react';
import { AnimatePresence, motion } from 'motion/react';
import { ResponsiveContainer, LineChart, Line, Tooltip as RechartsTooltip, XAxis, YAxis } from 'recharts';
import Markdown from 'react-markdown';

interface SyncMetric {
  repo: string;
  status: 'syncing' | 'synced' | 'failed';
  lastSync: string;
  uptime: string;
}

interface CommitActivity {
  id: string;
  message: string;
  author: string;
  time: string;
  branch: string;
}

interface PendingPR {
  id: number;
  title: string;
  author: string;
  comments: number;
  time: string;
}

interface WorkflowRun {
  id: string;
  name: string;
  status: 'success' | 'failed' | 'in_progress';
  time: string;
  duration: string;
  repo: string;
}

export function GitHubStatusDashboard() {
  const [activeTab, setActiveTab] = useState<'status' | 'insights'>('status');
  const [workflowViewMode, setWorkflowViewMode] = useState<'timeline' | 'errors'>('timeline');
  
  const [syncMetrics, setSyncMetrics] = useState<SyncMetric[]>([
    { repo: 'atown-chain/core-node', status: 'synced', lastSync: '2 min ago', uptime: '99.9%' },
    { repo: 'atown-chain/smart-contracts', status: 'syncing', lastSync: 'Just now', uptime: '99.9%' },
    { repo: 'atown-chain/explorer-ui', status: 'synced', lastSync: '5 min ago', uptime: '99.8%' }
  ]);

  const [commits, setCommits] = useState<CommitActivity[]>([
    { id: 'a1b2c3d', message: 'feat: add zero-knowledge proof verifier circuit', author: 'mworob', time: '10 min ago', branch: 'main' },
    { id: 'x9y8z7w', message: 'fix: concurrent transaction locking issue', author: 'devteam', time: '1 hour ago', branch: 'core-fixes' },
    { id: 'j5k4l3m', message: 'update: smart contract ABI definitions', author: 'buildbot', time: '3 hours ago', branch: 'main' }
  ]);

  const [pullRequests, setPullRequests] = useState<PendingPR[]>([
    { id: 142, title: 'Implement Cross-Rollup Data Availability', author: 'mworob', comments: 12, time: '2 hours ago' },
    { id: 145, title: 'Dashboard UI Responsive Fixes', author: 'frontend-dev', comments: 3, time: '5 hours ago' },
    { id: 147, title: 'Update dependencies: Vite and React', author: 'dependabot', comments: 0, time: '1 day ago' }
  ]);

  const [workflows, setWorkflows] = useState<WorkflowRun[]>([
    { id: 'run-101', name: 'Production Deployment', status: 'success', time: '10 min ago', duration: '2m 14s', repo: 'atown-chain/core-node' },
    { id: 'run-102', name: 'CI/CD Pipeline', status: 'in_progress', time: '12 min ago', duration: 'Running...', repo: 'atown-chain/smart-contracts' },
    { id: 'run-103', name: 'End-to-End Tests', status: 'failed', time: '1 hour ago', duration: '45s', repo: 'atown-chain/explorer-ui' },
    { id: 'run-104', name: 'Lint & Formatting', status: 'success', time: '1 hour ago', duration: '1m 02s', repo: 'atown-chain/explorer-ui' },
    { id: 'run-105', name: 'Security Audit', status: 'success', time: '2 hours ago', duration: '5m 30s', repo: 'atown-chain/smart-contracts' },
    { id: 'run-106', name: 'Build Docker Image', status: 'success', time: '3 hours ago', duration: '3m 45s', repo: 'atown-chain/core-node' },
    { id: 'run-107', name: 'Release Tagging', status: 'success', time: '1 day ago', duration: '2m 20s', repo: 'atown-chain/core-node' },
    { id: 'run-108', name: 'Integration Tests', status: 'failed', time: '1 day ago', duration: '1m 15s', repo: 'atown-chain/core-node' },
    { id: 'run-109', name: 'Update Documentation', status: 'success', time: '2 days ago', duration: '40s', repo: 'atown-chain/core-node' },
    { id: 'run-110', name: 'TypeScript Build', status: 'success', time: '2 days ago', duration: '1m 10s', repo: 'atown-chain/explorer-ui' },
  ]);

  const [toasts, setToasts] = useState<{id: string, runName: string, repo: string}[]>([]);
  const previousWorkflowsRef = useRef<WorkflowRun[]>(workflows);

  const [suggestingFixId, setSuggestingFixId] = useState<string | null>(null);
  const [suggestions, setSuggestions] = useState<Record<string, string>>({});

  interface AnalyzedLog {
    lineNumber: string;
    snippet: string;
    explanation: string;
    loading: boolean;
  }
  const [analyzedLogs, setAnalyzedLogs] = useState<Record<string, AnalyzedLog>>({});

  useEffect(() => {
    workflows.filter(wf => wf.status === 'failed').forEach(wf => {
      if (!analyzedLogs[wf.id]) {
        // Optimistically set to loading
        setAnalyzedLogs(prev => ({ ...prev, [wf.id]: { loading: true, lineNumber: '', snippet: '', explanation: '' } }));
        
        fetch('/api/gemini/analyze-log', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            runName: wf.name,
            repo: wf.repo,
            errorLog: "Error: Process completed with exit code 1.\nModule not found: Can't resolve '@atown-chain/crypto' in '/workspace/src/utils'\nLine 42: import { signTransaction } from '@atown-chain/crypto';"
          })
        }).then(res => res.json()).then(data => {
          setAnalyzedLogs(prev => ({ 
            ...prev, 
            [wf.id]: { 
              loading: false, 
              lineNumber: data.lineNumber || 'N/A', 
              snippet: data.snippet || 'No snippet', 
              explanation: data.explanation || 'No explanation' 
            } 
          }));
        }).catch(err => {
          setAnalyzedLogs(prev => ({ 
            ...prev, 
            [wf.id]: { 
              loading: false, 
              lineNumber: 'N/A', 
              snippet: 'Failed to extract snippet.', 
              explanation: 'Service error.' 
            } 
          }));
        });
      }
    });
  }, [workflows, analyzedLogs]);

  const handleSuggestFix = async (wf: WorkflowRun) => {
    if (suggestions[wf.id] || suggestingFixId === wf.id) return;
    
    setSuggestingFixId(wf.id);
    try {
      const response = await fetch('/api/gemini/suggest-fix', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          runName: wf.name,
          repo: wf.repo,
          errorLog: "Error: Process completed with exit code 1. Dependency resolution failed due to version mismatch."
        })
      });
      const data = await response.json();
      if (data.suggestion) {
        setSuggestions(prev => ({ ...prev, [wf.id]: data.suggestion }));
      }
    } catch (e) {
      console.error(e);
    } finally {
      setSuggestingFixId(null);
    }
  };

  useEffect(() => {
    // Check for new failures
    const newFailures = workflows.filter(wf => {
      const prev = previousWorkflowsRef.current.find(p => p.id === wf.id);
      return wf.status === 'failed' && prev && prev.status !== 'failed';
    });
    
    if (newFailures.length > 0) {
      newFailures.forEach(wf => {
        const toastId = Date.now() + Math.random().toString();
        setToasts(prev => [...prev, { id: toastId, runName: wf.name, repo: wf.repo }]);
        setTimeout(() => {
          setToasts(current => current.filter(t => t.id !== toastId));
        }, 5000);
      });
    }
    
    previousWorkflowsRef.current = workflows;
  }, [workflows]);

  useEffect(() => {
    const interval = setInterval(() => {
      setSyncMetrics(prev => prev.map(m => {
        if (m.repo === 'atown-chain/smart-contracts') {
          return { ...m, status: m.status === 'syncing' ? 'synced' : 'syncing', lastSync: m.status === 'syncing' ? 'Just now' : '1 min ago' };
        }
        return m;
      }));
      
      // Simulate random workflow completion/failure
      setWorkflows(prev => {
        const inProgressIdx = prev.findIndex(w => w.status === 'in_progress');
        if (inProgressIdx !== -1 && Math.random() > 0.5) {
          const newWfs = [...prev];
          newWfs[inProgressIdx] = { 
            ...newWfs[inProgressIdx], 
            status: 'failed', 
            duration: '1m 30s',
            time: 'Just now'
          };
          return newWfs;
        }
        return prev;
      });
    }, 10000);
    return () => clearInterval(interval);
  }, []);

  const calculateInsights = () => {
    const completed = workflows.filter(w => w.status !== 'in_progress');
    const successCount = completed.filter(w => w.status === 'success').length;
    const successRate = completed.length > 0 ? (successCount / completed.length * 100).toFixed(1) : '0.0';
    
    const totalSeconds = completed.reduce((acc, w) => {
        let sec = 0;
        const mMatch = w.duration.match(/(\d+)m/);
        const sMatch = w.duration.match(/(\d+)s/);
        if (mMatch) sec += parseInt(mMatch[1]) * 60;
        if (sMatch) sec += parseInt(sMatch[1]);
        return acc + sec;
    }, 0);
    const avgDuration = completed.length > 0 ? totalSeconds / completed.length : 0;
    const formatDuration = (s: number) => {
        const m = Math.floor(s / 60);
        return m > 0 ? `${m}m ${Math.floor(s % 60)}s` : `${Math.floor(s)}s`;
    };

    return {
        successRate,
        avgDuration: formatDuration(avgDuration)
    };
  };

  const insights = calculateInsights();

  return (
    <div className="bg-[#0b0f19] border border-white/10 rounded-2xl p-6 shadow-2xl flex flex-col gap-6 w-full h-full overflow-y-auto custom-scrollbar relative">
      <div className="flex flex-col sm:flex-row sm:items-end justify-between border-b border-white/10 pb-4 gap-4 sticky top-0 bg-[#0b0f19] z-10">
        <div className="flex flex-col gap-4">
          <div>
            <h2 className="text-xl font-bold text-white flex items-center gap-2 mb-2 tracking-tight">
              <Github className="w-6 h-6 text-indigo-400" />
              GitHub Status Dashboard
            </h2>
            <p className="text-xs font-mono text-slate-400 flex items-center gap-1 mb-3">
              Real-time synchronization metrics across A-TownChain repositories
            </p>
          </div>
          <div className="flex items-center gap-6 text-sm font-mono uppercase tracking-wider">
             <button onClick={() => setActiveTab('status')} className={`pb-2 border-b-2 transition-colors ${activeTab === 'status' ? 'border-indigo-400 text-indigo-400 font-bold' : 'border-transparent text-slate-500 hover:text-slate-300'}`}>Status</button>
             <button onClick={() => setActiveTab('insights')} className={`pb-2 border-b-2 transition-colors ${activeTab === 'insights' ? 'border-emerald-400 text-emerald-400 font-bold' : 'border-transparent text-slate-500 hover:text-slate-300'}`}>Insights</button>
          </div>
        </div>
        <div className="flex flex-col sm:flex-row gap-2 items-end sm:items-center mb-2">
          <div className="text-xs font-mono text-slate-300 bg-white/5 border border-white/10 px-3 py-1.5 rounded-lg flex items-center gap-2 shadow-sm">
            <Activity className="w-3.5 h-3.5 text-indigo-400" />
            30d Stability: <span className={Number(insights.successRate) >= 80 ? "text-emerald-400 font-bold" : "text-amber-400 font-bold"}>{insights.successRate}%</span>
          </div>
          <div className="text-xs font-mono text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 px-3 py-1.5 rounded-lg flex items-center gap-2 shadow-sm">
            <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse shadow-[0_0_8px_rgba(52,211,153,0.8)]"></div>
            Live Sync Active
          </div>
        </div>
      </div>

      {activeTab === 'status' ? (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Sync Metrics Section */}
        <div className="lg:col-span-3 flex flex-col gap-4">
          <h3 className="text-sm font-bold text-slate-300 flex items-center gap-2 uppercase tracking-widest border-b border-white/5 pb-2">
            <Activity className="w-4 h-4 text-emerald-400" /> Repository Sync Status
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {syncMetrics.map((metric, idx) => (
              <div key={idx} className="bg-white/5 border border-white/10 rounded-xl p-4 flex flex-col gap-3 relative overflow-hidden group">
                <div className={`absolute top-0 inset-x-0 h-0.5 ${metric.status === 'synced' ? 'bg-emerald-500/50' : metric.status === 'failed' ? 'bg-red-500/50' : 'bg-indigo-500/50'}`} />
                <div className="flex justify-between items-start">
                  <div className="flex items-center gap-2">
                    <Github className="w-4 h-4 text-slate-400" />
                    <span className="text-sm font-mono font-bold text-white max-w-[150px] truncate" title={metric.repo}>{metric.repo.split('/')[1]}</span>
                  </div>
                  {metric.status === 'synced' ? (
                     <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                  ) : metric.status === 'syncing' ? (
                     <RefreshCw className="w-4 h-4 text-indigo-400 animate-spin" />
                  ) : (
                     <AlertCircle className="w-4 h-4 text-red-400" />
                  )}
                </div>
                <div className="flex justify-between mt-auto items-end">
                  <div className="flex flex-col">
                    <span className="text-[10px] uppercase text-slate-500 tracking-wider">Last Sync</span>
                    <span className="text-xs font-mono text-slate-300">{metric.lastSync}</span>
                  </div>
                  <div className="flex flex-col items-end">
                    <span className="text-[10px] uppercase text-slate-500 tracking-wider">Uptime</span>
                    <span className="text-xs font-mono text-emerald-400">{metric.uptime}</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Recent Commits Section */}
        <div className="lg:col-span-2 flex flex-col gap-4">
          <h3 className="text-sm font-bold text-slate-300 flex items-center gap-2 uppercase tracking-widest border-b border-white/5 pb-2">
            <GitCommit className="w-4 h-4 text-indigo-400" /> Recent Commits
          </h3>
          <div className="flex flex-col gap-3">
            {commits.map((commit, idx) => (
              <div key={idx} className="bg-black/30 border border-white/5 rounded-xl p-4 flex items-center gap-4 hover:bg-white/5 transition-colors">
                <div className="p-2 bg-indigo-500/10 rounded-full border border-indigo-500/20">
                  <GitCommit className="w-4 h-4 text-indigo-400" />
                </div>
                <div className="flex-1 flex flex-col gap-1 overflow-hidden">
                  <div className="text-sm font-medium text-slate-200 truncate">{commit.message}</div>
                  <div className="flex items-center gap-3 text-xs font-mono text-slate-500">
                    <span className="flex items-center gap-1 text-slate-400"><GitBranch className="w-3 h-3" /> {commit.branch}</span>
                    <span>•</span>
                    <span className="text-indigo-300">{commit.author}</span>
                    <span>•</span>
                    <span>{commit.time}</span>
                  </div>
                </div>
                <div className="font-mono text-xs text-slate-400 bg-white/5 px-2 py-1 rounded">
                  {commit.id}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Pending Pull Requests Section */}
        <div className="lg:col-span-1 flex flex-col gap-4">
          <h3 className="text-sm font-bold text-slate-300 flex items-center gap-2 uppercase tracking-widest border-b border-white/5 pb-2">
            <GitPullRequest className="w-4 h-4 text-purple-400" /> Pending PRs
          </h3>
          <div className="flex flex-col gap-3">
            {pullRequests.map((pr, idx) => (
              <div key={idx} className="bg-black/30 border border-white/5 rounded-xl p-4 flex flex-col gap-2 hover:bg-white/5 transition-colors">
                <div className="flex items-start gap-2">
                  <GitPullRequest className="w-4 h-4 text-purple-400 mt-0.5 shrink-0" />
                  <div className="text-sm font-medium text-slate-200 leading-tight">{pr.title}</div>
                </div>
                <div className="flex items-center justify-between mt-2 pt-2 border-t border-white/5">
                  <div className="flex items-center gap-2 text-xs font-mono text-slate-500">
                    <span className="text-slate-400">#{pr.id}</span>
                    <span>by {pr.author}</span>
                  </div>
                  <div className="flex items-center gap-1 text-[10px] font-mono bg-white/5 px-1.5 py-0.5 rounded text-slate-400">
                     <GitMerge className="w-3 h-3 text-slate-500" /> {pr.comments}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Workflow Runs Section */}
        <div className="lg:col-span-3 flex flex-col gap-4 mt-2">
          <div className="flex items-center justify-between border-b border-white/5 pb-2">
            <h3 className="text-sm font-bold text-slate-300 flex items-center gap-2 uppercase tracking-widest">
              <PlayCircle className="w-4 h-4 text-emerald-400" /> Recent Workflows
            </h3>
            <div className="flex bg-black/40 rounded-lg p-0.5 border border-white/10">
              <button 
                onClick={() => setWorkflowViewMode('timeline')}
                className={`px-3 py-1 text-[10px] uppercase font-bold tracking-wider rounded-md transition-colors ${workflowViewMode === 'timeline' ? 'bg-white/10 text-white shadow-sm' : 'text-slate-500 hover:text-slate-300'}`}
              >
                Timeline
              </button>
              <button 
                onClick={() => setWorkflowViewMode('errors')}
                className={`px-3 py-1 text-[10px] uppercase font-bold tracking-wider rounded-md transition-colors gap-1 flex items-center ${workflowViewMode === 'errors' ? 'bg-rose-500/20 text-rose-400 shadow-sm' : 'text-slate-500 hover:text-rose-400/50'}`}
              >
                Error Logs
              </button>
            </div>
          </div>
          <div className="flex flex-col gap-0 relative mt-2">
            <div className="absolute left-6 top-4 bottom-4 w-px bg-white/10 z-0"></div>
            
            {workflows.filter(wf => workflowViewMode === 'timeline' || wf.status === 'failed').length === 0 && workflowViewMode === 'errors' ? (
               <div className="py-12 text-center text-slate-500 text-sm font-mono relative z-10 bg-[#0b0f19]">
                 <CheckCircle className="w-8 h-8 text-emerald-500/50 mx-auto mb-3" />
                 No failed workflows detected.
               </div>
            ) : workflows.filter(wf => workflowViewMode === 'timeline' || wf.status === 'failed').map((wf, idx) => (
              <div key={wf.id} className="relative z-10 py-3 hover:bg-white/5 px-2 rounded-lg transition-colors group">
                <div className="flex items-center gap-4">
                  <div className="w-8 h-8 rounded-full bg-[#0b0f19] flex items-center justify-center shrink-0">
                    {wf.status === 'success' ? (
                       <CheckCircle className="w-5 h-5 text-emerald-400 bg-emerald-500/10 rounded-full" />
                    ) : wf.status === 'failed' ? (
                       <XCircle className="w-5 h-5 text-rose-400 bg-rose-500/10 rounded-full" />
                    ) : (
                       <Loader2 className="w-5 h-5 text-indigo-400 animate-spin" />
                    )}
                  </div>
                  <div className="flex flex-col flex-1">
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-bold text-slate-200 group-hover:text-indigo-300 transition-colors">{wf.name}</span>
                      <span className="text-xs text-slate-500 font-mono flex items-center gap-1"><Clock className="w-3 h-3" /> {wf.time}</span>
                    </div>
                    <div className="flex items-center justify-between mt-1">
                      <span className="text-xs text-slate-500 font-mono flex items-center gap-1"><Github className="w-3 h-3" /> {wf.repo}</span>
                      <div className="flex items-center gap-3">
                        <span className="text-xs font-mono text-slate-500">{wf.duration}</span>
                        <span className={`text-[10px] uppercase tracking-wider font-bold px-2 py-0.5 rounded ${
                          wf.status === 'success' ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' :
                          wf.status === 'failed' ? 'bg-rose-500/10 text-rose-400 border border-rose-500/20' :
                          'bg-indigo-500/10 text-indigo-400 border border-indigo-500/20'
                        }`}>
                          {wf.status.replace('_', ' ')}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
                
                {wf.status === 'failed' && workflowViewMode === 'errors' && (
                  <div className="ml-12 mt-3 p-3 bg-black/40 border border-rose-500/20 rounded-lg flex flex-col gap-3">
                    <div className="flex items-center justify-between">
                      <span className="text-xs text-rose-400 font-mono flex items-center gap-2">
                        <Zap className="w-3 h-3" /> Actions error segment detected.
                      </span>
                      {!suggestions[wf.id] && (
                        <button
                          onClick={() => handleSuggestFix(wf)}
                          disabled={suggestingFixId === wf.id}
                          className="flex items-center gap-1.5 px-3 py-1 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 border border-indigo-500/30 rounded text-xs transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                          {suggestingFixId === wf.id ? <Loader2 className="w-3 h-3 animate-spin"/> : <Wand2 className="w-3 h-3" />}
                          {suggestingFixId === wf.id ? 'Analyzing...' : 'Suggest Fix (Gemini)'}
                        </button>
                      )}
                    </div>
                    <div className="bg-[#0b0f19]/80 border border-rose-500/10 p-3 rounded font-mono text-[10px] text-slate-400 leading-relaxed overflow-x-auto">
                      {analyzedLogs[wf.id]?.loading ? (
                        <div className="flex items-center gap-2 text-indigo-400">
                          <Loader2 className="w-3 h-3 animate-spin" /> Extracting exact failure cause snippet...
                        </div>
                      ) : (
                        <>
                          {(analyzedLogs[wf.id]?.lineNumber || analyzedLogs[wf.id]?.snippet) ? (
                             <>
                               <div className="flex justify-between items-start mb-2 border-b border-white/5 pb-2">
                                 <span className="text-rose-400 font-bold uppercase tracking-wider flex items-center gap-1"><Zap className="w-3 h-3"/> Line {analyzedLogs[wf.id]?.lineNumber || 'N/A'}</span>
                                 <span className="text-indigo-400 text-[9px] bg-indigo-500/10 border border-indigo-500/20 px-2 py-0.5 rounded uppercase tracking-widest flex items-center gap-1"><Wand2 className="w-2 h-2"/> Extracted via AI</span>
                               </div>
                               {analyzedLogs[wf.id].snippet.split('\n').map((line, i) => (
                                 <React.Fragment key={i}>
                                   <span className={line.toLowerCase().includes('error') ? 'text-rose-400 font-bold' : 'text-slate-300'}>{line}</span><br/>
                                 </React.Fragment>
                               ))}
                               {analyzedLogs[wf.id].explanation && (
                                 <div className="mt-3 p-2 bg-rose-500/5 text-rose-300/80 rounded border-l-2 border-rose-500/50 mb-1">
                                   // AI Explanation: {analyzedLogs[wf.id].explanation}
                                 </div>
                               )}
                             </>
                          ) : (
                             <>
                              <span className="text-slate-500">{`[12:34:01] Building dependencies...`}</span><br/>
                              <span className="text-rose-400 font-bold">{`[12:34:05] Error: Process completed with exit code 1.`}</span><br/>
                              <span className="text-rose-300/80">{`[12:34:05] Failed to resolve version conflict for package '@atown-chain/crypto'.`}</span><br/>
                              <span className="text-rose-300/80">{`[12:34:05] Check package.json for peer dependency constraints.`}</span>
                             </>
                          )}
                        </>
                      )}
                    </div>
                    {suggestions[wf.id] && (
                       <div className="text-sm text-slate-300 bg-white/5 p-3 rounded border border-white/10 custom-scrollbar overflow-x-auto">
                         <div className="flex items-center gap-2 mb-2 text-indigo-300 text-xs uppercase font-bold tracking-widest"><Wand2 className="w-3 h-3" /> AI Suggestion</div>
                         <div className="markdown-body text-xs prose prose-invert max-w-none">
                           <Markdown>{suggestions[wf.id]}</Markdown>
                         </div>
                       </div>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>

      </div>
      ) : (
        <div className="flex flex-col gap-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="bg-black/30 border border-emerald-500/20 rounded-xl p-6 flex items-center gap-6">
               <div className="p-4 bg-emerald-500/10 rounded-2xl border border-emerald-500/20">
                 <CheckCircle2 className="w-8 h-8 text-emerald-400" />
               </div>
               <div className="flex flex-col">
                 <span className="text-xs font-mono uppercase tracking-widest text-slate-500">Success Rate (Last 10)</span>
                 <span className="text-3xl font-bold text-emerald-400">{insights.successRate}%</span>
               </div>
            </div>
            <div className="bg-black/30 border border-indigo-500/20 rounded-xl p-6 flex items-center gap-6">
               <div className="p-4 bg-indigo-500/10 rounded-2xl border border-indigo-500/20">
                 <Clock className="w-8 h-8 text-indigo-400" />
               </div>
               <div className="flex flex-col">
                 <span className="text-xs font-mono uppercase tracking-widest text-slate-500">Avg Duration (Last 10)</span>
                 <span className="text-3xl font-bold text-indigo-400">{insights.avgDuration}</span>
               </div>
            </div>
          </div>
          
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="bg-white/5 border border-white/10 rounded-xl p-6">
              <h3 className="text-sm font-bold text-slate-300 flex items-center gap-2 uppercase tracking-widest mb-6">
                <TrendingUp className="w-4 h-4 text-emerald-400" /> Success Rate Trend (30 Days)
              </h3>
              <div className="h-[200px] w-full">
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart data={useMemo(() => {
                    const dayBuckets = Array.from({ length: 30 }).map(() => ({ total: 0, success: 0 }));
                    const now = new Date();
                    
                    workflows.forEach(w => {
                      let daysAgo = 0;
                      if (w.time.includes('day')) {
                        daysAgo = parseInt(w.time) || 0;
                      } else if (w.time.includes('week')) {
                        daysAgo = (parseInt(w.time) || 0) * 7;
                      } else if (w.time.includes('month')) {
                        daysAgo = 30; // Just dump in the oldest bucket for simplicity
                      }
                      
                      if (daysAgo < 30) {
                        const idx = 29 - daysAgo; // idx 29 is today
                        if (w.status !== 'in_progress') {
                          dayBuckets[idx].total += 1;
                          if (w.status === 'success') {
                            dayBuckets[idx].success += 1;
                          }
                        }
                      }
                    });

                    // We need a baseline to fill days with no workflows, instead of dropping to 0
                    const overallCompleted = workflows.filter(w => w.status !== 'in_progress');
                    const overallRate = overallCompleted.length > 0 
                      ? (overallCompleted.filter(w => w.status === 'success').length / overallCompleted.length) * 100 
                      : 100;

                    return dayBuckets.map((bucket, i) => {
                      const date = new Date(now);
                      date.setDate(date.getDate() - (29 - i));
                      
                      const rate = bucket.total > 0 
                        ? (bucket.success / bucket.total) * 100 
                        : overallRate;

                      return {
                        date: date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
                        rate: Number(rate.toFixed(1))
                      };
                    });
                  }, [workflows])}>
                    <XAxis dataKey="date" stroke="#64748b" fontSize={10} tickLine={false} axisLine={false} minTickGap={20} />
                    <YAxis stroke="#64748b" fontSize={10} tickLine={false} axisLine={false} domain={[0, 100]} />
                    <RechartsTooltip 
                      contentStyle={{ backgroundColor: '#0f172a', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '8px', fontSize: '12px', color: '#f8fafc' }}
                      itemStyle={{ color: '#34d399' }}
                      labelStyle={{ color: '#94a3b8', marginBottom: '4px' }}
                      formatter={(value: number) => [`${value.toFixed(1)}%`, 'Success Rate']}
                    />
                    <Line type="monotone" dataKey="rate" stroke="#34d399" strokeWidth={2} dot={false} activeDot={{ r: 4, fill: '#34d399', stroke: '#0f172a', strokeWidth: 2 }} />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </div>

            <div className="bg-white/5 border border-white/10 rounded-xl p-6">
               <h3 className="text-sm font-bold text-slate-300 flex items-center gap-2 uppercase tracking-widest mb-6">
                 <BarChart className="w-4 h-4 text-purple-400" /> Activity Breakdown
               </h3>
               <div className="space-y-4">
               {workflows.slice(0, 5).map(wf => {
                 const mMatch = wf.duration.match(/(\d+)m/);
                 const sMatch = wf.duration.match(/(\d+)s/);
                 let sec = 0;
                 if (mMatch) sec += parseInt(mMatch[1]) * 60;
                 if (sMatch) sec += parseInt(sMatch[1]);
                 if (wf.status === 'in_progress') sec = 60; // Just visual fallback
                 
                 const maxDuration = 330; // 5.5 mins approx max length
                 const percentage = Math.min((sec / maxDuration) * 100, 100);
                 
                 return (
                   <div key={wf.id} className="flex flex-col gap-2">
                     <div className="flex justify-between items-end">
                       <span className="text-xs font-bold text-slate-300">{wf.name}</span>
                       <span className="text-[10px] font-mono text-slate-500">{wf.duration}</span>
                     </div>
                     <div className="w-full bg-black/40 h-2 rounded-full overflow-hidden flex">
                       <div 
                         className={`h-full rounded-full ${wf.status === 'success' ? 'bg-emerald-500' : wf.status === 'failed' ? 'bg-rose-500' : 'bg-indigo-500 animate-pulse'}`}
                         style={{ width: `${percentage}%` }}
                       ></div>
                     </div>
                   </div>
                 );
               })}
             </div>
          </div>
        </div>
        </div>
      )}
      
      {/* Toast Notifications */}
      <div className="absolute bottom-6 right-6 flex flex-col gap-3 z-50 pointer-events-none">
        <AnimatePresence>
          {toasts.map(toast => (
            <motion.div
              key={toast.id}
              initial={{ opacity: 0, y: 50, scale: 0.9 }}
              animate={{ opacity: 1, y: 0, scale: 1 }}
              exit={{ opacity: 0, x: 50, scale: 0.9 }}
              className="bg-black/80 backdrop-blur-md border border-rose-500/30 rounded-xl p-4 shadow-2xl shadow-rose-500/10 flex items-start gap-4 min-w-[300px] pointer-events-auto"
            >
              <div className="bg-rose-500/20 p-2 rounded-full border border-rose-500/30 shrink-0 mt-0.5">
                <XCircle className="w-5 h-5 text-rose-400" />
              </div>
              <div className="flex-1 flex flex-col">
                <div className="flex items-center justify-between gap-4">
                  <span className="font-bold text-white text-sm">Workflow Failed</span>
                  <button 
                    onClick={() => setToasts(t => t.filter(x => x.id !== toast.id))}
                    className="text-slate-500 hover:text-white transition-colors"
                  >
                    <X className="w-4 h-4" />
                  </button>
                </div>
                <p className="text-xs text-slate-300 mt-1">{toast.runName}</p>
                <div className="flex items-center gap-1 mt-2 text-[10px] uppercase tracking-wider font-mono text-slate-500">
                  <Github className="w-3 h-3" /> {toast.repo}
                </div>
              </div>
            </motion.div>
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}

