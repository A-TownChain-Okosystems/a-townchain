import React from 'react';
import { GitCommit, GitPullRequest, AlertCircle, Tag, GitMerge, Activity } from 'lucide-react';

export function GitOpsView() {
  const commits = [
    { hash: "d0f9a12", msg: "feat(atc-pack): implement decentralized dependency resolution via IPFS", user: "dev-team", time: "5 mins ago" },
    { hash: "b5c6d7e", msg: "fix(atvm): debug trace logs for cross-contract calls", user: "mworob", time: "12 mins ago" },
    { hash: "a1b2c3d", msg: "feat(atvm): implement zero-knowledge verification layer", user: "mworob", time: "1 hour ago" },
    { hash: "e4f5g6h", msg: "fix(network): resolve node synchronization race condition", user: "sys-auto", time: "3 hours ago" },
    { hash: "i7j8k9l", msg: "docs(api): update swagger for billing interface", user: "dev-team", time: "6 hours ago" }
  ];

  const issues = [
    { id: "ATC-145", title: "Integrate Dependabot for atc-lang GitHub Repository", status: "Open", priority: "High" },
    { id: "ATC-144", title: "Setup GitHub Actions workflow for atc-lang Compiler CI", status: "In Progress", priority: "Medium" },
    { id: "ATC-143", title: "Initialize atc-lang-wiki structure and push markdown", status: "Open", priority: "Medium" },
    { id: "ATC-142", title: "Memory leak in transaction mempool", status: "Open", priority: "High" },
    { id: "ATC-140", title: "Implement Stripe fiat on-ramp", status: "Completed", priority: "Medium" },
    { id: "ATC-135", title: "Update React dependencies", status: "Closed", priority: "Low" }
  ];

  return (
    <div className="flex flex-col gap-10 mt-8 pb-12 font-sans">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-orange-500/10 border border-orange-500/30 flex items-center justify-center text-orange-400 shadow-[0_0_15px_rgba(249,115,22,0.2)]">
          <GitMerge className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            GitOps & Version Control
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Issues, Commits, Branches und Tags Tracking für die gesamte A-TownChain Architektur.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col gap-4">
          <h3 className="text-sm font-semibold text-slate-300 flex items-center gap-2">
            <GitCommit className="w-4 h-4 text-orange-400" /> Recent Commits
          </h3>
          <div className="space-y-3">
            {commits.map(c => (
               <div key={c.hash} className="bg-black/40 p-3 rounded-lg border border-white/5">
                 <div className="flex items-center justify-between mb-1">
                   <span className="text-xs font-mono text-orange-400">{c.hash}</span>
                   <span className="text-[10px] text-slate-500">{c.time}</span>
                 </div>
                 <p className="text-xs text-slate-300 mb-2 truncate">{c.msg}</p>
                 <div className="text-[10px] font-mono text-slate-500 bg-white/5 px-2 py-0.5 rounded inline-block">@{c.user}</div>
               </div>
            ))}
          </div>
        </div>

        <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5 flex flex-col gap-4">
          <h3 className="text-sm font-semibold text-slate-300 flex items-center gap-2">
            <AlertCircle className="w-4 h-4 text-red-400" /> Active Issues
          </h3>
          <div className="space-y-3">
            {issues.map(i => (
               <div key={i.id} className="bg-black/40 p-3 rounded-lg border border-white/5 flex flex-col gap-2">
                 <div className="flex items-center justify-between">
                   <span className="text-xs font-mono font-bold text-slate-200">{i.id}</span>
                   <span className={`text-[10px] px-2 py-0.5 rounded font-bold uppercase ${i.status === 'Open' ? 'bg-red-500/10 text-red-400' : i.status === 'In Progress' ? 'bg-amber-500/10 text-amber-400' : 'bg-slate-500/10 text-slate-400'}`}>{i.status}</span>
                 </div>
                 <p className="text-xs text-slate-300">{i.title}</p>
               </div>
            ))}
          </div>
        </div>

        <div className="flex flex-col gap-6">
          <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5">
            <h3 className="text-sm font-semibold text-slate-300 flex items-center gap-2 mb-4">
              <Activity className="w-4 h-4 text-emerald-400" /> Deployment Events Timeline
            </h3>
            <div className="relative border-l border-white/10 ml-3 space-y-4 pb-2">
              <div className="relative pl-6">
                <span className="absolute -left-1.5 top-1.5 w-3 h-3 rounded-full bg-emerald-500 shadow-[0_0_10px_rgba(16,185,129,0.8)]"></span>
                <p className="text-xs font-mono text-emerald-400 mb-0.5">Now</p>
                <p className="text-sm text-white">v2.4.1-rc Deploying...</p>
              </div>
              <div className="relative pl-6 opacity-70">
                <span className="absolute -left-1.5 top-1.5 w-3 h-3 rounded-full bg-slate-500"></span>
                <p className="text-xs font-mono text-slate-400 mb-0.5">Yesterday</p>
                <p className="text-sm text-slate-300">v2.4.0 Released safely</p>
              </div>
              <div className="relative pl-6 opacity-50">
                <span className="absolute -left-1.5 top-1.5 w-3 h-3 rounded-full bg-slate-600"></span>
                <p className="text-xs font-mono text-slate-400 mb-0.5">3 days ago</p>
                <p className="text-sm text-slate-300">Hotfix 2.3.9 injected</p>
              </div>
            </div>
          </div>
          
          <div className="p-6 rounded-2xl bg-gradient-to-br from-[#090b14] to-[#1a120b] border border-orange-500/20 text-center">
             <Tag className="w-8 h-8 text-orange-500 mx-auto mb-3" />
             <h3 className="text-2xl font-bold text-white font-mono mb-1">v2.4.1-rc</h3>
             <p className="text-xs text-slate-400">Current Release Tag</p>
          </div>
          <div className="p-6 rounded-2xl bg-[#090b14] border border-white/5">
            <h3 className="text-sm font-semibold text-slate-300 mb-4 flex items-center gap-2">
              <Activity className="w-4 h-4 text-emerald-400" /> Pipeline Status
            </h3>
            <div className="space-y-2">
               <div className="flex justify-between items-center text-xs">
                 <span className="text-slate-400">Build</span>
                 <span className="text-emerald-400 font-mono">Passing</span>
               </div>
               <div className="flex justify-between items-center text-xs">
                 <span className="text-slate-400">Tests</span>
                 <span className="text-emerald-400 font-mono">842/842</span>
               </div>
               <div className="flex justify-between items-center text-xs">
                 <span className="text-slate-400">Deploy</span>
                 <span className="text-amber-400 font-mono">Pending</span>
               </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
