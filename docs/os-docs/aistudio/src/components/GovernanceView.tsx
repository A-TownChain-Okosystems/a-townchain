import React, { useState, useEffect } from "react";
import {
  Landmark,
  Vote,
  FileText,
  CheckCircle2,
  XCircle,
  ArrowUpRight,
  X,
  AlertTriangle,
  ShieldCheck,
  Cpu,
  Network,
  Server,
  Github,
  GitCommit,
  CheckCircle,
  AlertOctagon
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

type Proposal = {
  id: string;
  title: string;
  description: string;
  status: "Active" | "Passed" | "Rejected";
  yesVotes: number;
  noVotes: number;
  abstainVotes: number;
  daysLeft: number;
  voted?: boolean;
};

const INITIAL_PROPOSALS: Proposal[] = [
  {
    id: "ATC-PROP-42",
    title: "Aktivierung des WASM Execution Layers auf Mainnet",
    description:
      "Dieses Update ermöglicht die Ausführung von WASM-kompilierten Smart Contracts auf dem Mainnet und erhöht so die Ausführungsgeschwindigkeit.",
    status: "Active",
    yesVotes: 78,
    noVotes: 12,
    abstainVotes: 10,
    daysLeft: 3,
  },
  {
    id: "ATC-PROP-41",
    title: "Reduzierung der P2P Gossip Protokoll Latenz",
    description: "Optimierung des P2P Netzwerks.",
    status: "Passed",
    yesVotes: 92,
    noVotes: 5,
    abstainVotes: 3,
    daysLeft: 0,
  },
  {
    id: "ATC-PROP-40",
    title: "Integration von Kyber Post-Quantum Signaturen (Testnet)",
    description: "Sicherheits-Upgrade gegen Post-Quantum Angriffe.",
    status: "Passed",
    yesVotes: 85,
    noVotes: 10,
    abstainVotes: 5,
    daysLeft: 0,
  },
  {
    id: "ATC-PROP-39",
    title: "Erhöhung der Validator Base Rewards um 5%",
    description: "Netzwerkauslastungsbedingte Budgetanpassung.",
    status: "Rejected",
    yesVotes: 35,
    noVotes: 60,
    abstainVotes: 5,
    daysLeft: 0,
  },
];

function PreFlightCheckModal({
  onComplete,
  onCancel,
}: {
  onComplete: () => void;
  onCancel: () => void;
}) {
  const [step, setStep] = useState(0);

  useEffect(() => {
    if (step >= 4) {
      const t = setTimeout(() => onComplete(), 800);
      return () => clearTimeout(t);
    }
    const t = setTimeout(() => setStep((s) => s + 1), 1000);
    return () => clearTimeout(t);
  }, [step, onComplete]);

  const checks = [
    {
      icon: <Network className="w-4 h-4" />,
      name: "P2P Network Stability",
      status: step > 0 ? "Passed" : "Checking...",
    },
    {
      icon: <Cpu className="w-4 h-4" />,
      name: "WASM Compiler Sandbox",
      status: step > 1 ? "Passed" : step === 1 ? "Checking..." : "Pending",
    },
    {
      icon: <Server className="w-4 h-4" />,
      name: "Node Consensus Sync",
      status: step > 2 ? "Passed" : step === 2 ? "Checking..." : "Pending",
    },
    {
      icon: <ShieldCheck className="w-4 h-4" />,
      name: "Protocol Identity Auth",
      status: step > 3 ? "Passed" : step === 3 ? "Checking..." : "Pending",
    },
  ];

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 z-[60] flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
    >
      <motion.div
        initial={{ scale: 0.95, y: 10 }}
        animate={{ scale: 1, y: 0 }}
        exit={{ scale: 0.95, y: 10 }}
        className="bg-slate-900 border border-indigo-500/30 rounded-2xl p-6 w-full max-w-sm shadow-2xl relative overflow-hidden"
      >
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-[40px] pointer-events-none" />

        <h3 className="text-xl font-bold text-white mb-2 flex items-center gap-2">
          <ShieldCheck className="w-5 h-5 text-indigo-400" />
          Pre-Flight Check
        </h3>
        <p className="text-xs text-slate-400 mb-6 font-mono">
          Validating ATC-OS Kernel Dependencies
        </p>

        <div className="space-y-4 mb-8">
          {checks.map((check, i) => (
            <div key={i} className="flex items-center justify-between">
              <div
                className={`flex items-center gap-3 ${i <= step ? "text-slate-200" : "text-slate-600"}`}
              >
                <div
                  className={`w-8 h-8 rounded-lg flex items-center justify-center border ${
                    check.status === "Passed"
                      ? "bg-emerald-500/10 border-emerald-500/30 text-emerald-400"
                      : check.status === "Checking..."
                        ? "bg-amber-500/10 border-amber-500/30 text-amber-400 animate-pulse"
                        : "bg-slate-800 border-slate-700 text-slate-500"
                  }`}
                >
                  {check.icon}
                </div>
                <span className="text-sm font-medium">{check.name}</span>
              </div>
              {check.status === "Passed" && (
                <CheckCircle2 className="w-4 h-4 text-emerald-400" />
              )}
              {check.status === "Checking..." && (
                <span className="w-4 h-4 border-2 border-amber-400 border-t-transparent rounded-full animate-spin" />
              )}
            </div>
          ))}
        </div>

        <button
          onClick={onCancel}
          className="w-full py-2.5 rounded-xl border border-white/10 text-white hover:bg-white/5 transition-colors font-medium text-sm"
        >
          Cancel Update
        </button>
      </motion.div>
    </motion.div>
  );
}

function GitOpsAutoAudit() {
  const [isAuditing, setIsAuditing] = useState(false);
  const [auditLog, setAuditLog] = useState<{
    id: string;
    commitMessage: string;
    author: string;
    hash: string;
    status: 'passed' | 'failed' | 'pending';
    issues: string[];
  }[]>([
    {
      id: "1",
      commitMessage: "feat: Add zero-knowledge proof verification endpoint",
      author: "Michael W.",
      hash: "a4f8e9b",
      status: 'passed',
      issues: []
    },
    {
      id: "2",
      commitMessage: "fix: Bypass auth check for local testing",
      author: "DevUser3",
      hash: "c2d9a1f",
      status: 'failed',
      issues: ["Critical: Authentication bypass detected in core module", "Violation of ATC Security Standard A-4"]
    },
    {
      id: "3",
      commitMessage: "refactor: Optimize consensus loops",
      author: "Michael W.",
      hash: "8f5b3c2",
      status: 'passed',
      issues: []
    }
  ]);

  const runAudit = () => {
    setIsAuditing(true);
    setTimeout(() => {
      setAuditLog(prev => [
        {
          id: Date.now().toString(),
          commitMessage: "Update smart contract variables",
          author: "System",
          hash: Math.random().toString(16).substring(2, 9),
          status: 'failed',
          issues: ["Warning: Use of non-deterministic function in consensus critical code", "Violation: Variable mutability outside transaction scope"]
        },
        ...prev
      ]);
      setIsAuditing(false);
    }, 2000);
  };

  return (
    <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl overflow-hidden mt-6">
      <div className="px-6 py-5 border-b border-atc-border/50 bg-[#060a16] flex items-center justify-between">
        <h3 className="text-lg font-medium text-white flex items-center gap-2">
          <Github className="w-5 h-5 text-slate-500" /> GitOps Auto-Audit
        </h3>
        <button
          onClick={runAudit}
          disabled={isAuditing}
          className="flex items-center gap-2 px-3 py-1.5 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 rounded-lg text-sm transition-colors border border-indigo-500/30 disabled:opacity-50"
        >
          {isAuditing ? <span className="animate-spin w-4 h-4 border-2 border-indigo-400 border-t-transparent rounded-full" /> : <GitCommit className="w-4 h-4" />}
          {isAuditing ? 'Auditing...' : 'Run Audit'}
        </button>
      </div>
      <div className="divide-y divide-atc-border/30">
        {auditLog.map((log) => (
          <div key={log.id} className="p-6 hover:bg-[#060a16]/50 transition-colors flex flex-col gap-3">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                {log.status === 'passed' ? <CheckCircle className="w-5 h-5 text-emerald-500" /> : <AlertOctagon className="w-5 h-5 text-red-500" />}
                <div>
                  <h4 className="text-white font-medium text-sm">{log.commitMessage}</h4>
                  <div className="flex items-center gap-2 mt-1 text-xs text-slate-500 font-mono">
                    <span className="text-indigo-300">{log.hash}</span>
                    <span>•</span>
                    <span>{log.author}</span>
                  </div>
                </div>
              </div>
              <span className={`text-[10px] uppercase font-bold px-2 py-1 rounded border ${log.status === 'passed' ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30' : 'bg-red-500/10 text-red-400 border-red-500/30'}`}>
                {log.status}
              </span>
            </div>
            {log.issues.length > 0 && (
              <div className="bg-red-500/5 border border-red-500/20 rounded-lg p-3 space-y-2 mt-2">
                {log.issues.map((issue, idx) => (
                  <div key={idx} className="flex items-start gap-2 text-xs text-red-300">
                    <AlertTriangle className="w-3.5 h-3.5 text-red-400 shrink-0 mt-0.5" />
                    <span>{issue}</span>
                  </div>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

export function GovernanceView() {
  const [proposals, setProposals] = useState<Proposal[]>(INITIAL_PROPOSALS);
  const [showCreateFlow, setShowCreateFlow] = useState(false);
  const [showPreFlight, setShowPreFlight] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [newDescription, setNewDescription] = useState("");

  const handleVote = (id: string, type: "yes" | "no") => {
    setProposals((prev) =>
      prev.map((p) => {
        if (p.id === id && !p.voted && p.status === "Active") {
          return {
            ...p,
            yesVotes: type === "yes" ? p.yesVotes + 1 : p.yesVotes,
            noVotes: type === "no" ? p.noVotes + 1 : p.noVotes,
            voted: true,
          };
        }
        return p;
      }),
    );
  };

  const activeCount = proposals.filter((p) => p.status === "Active").length;

  const submitProposal = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTitle.trim()) return;

    const newProp: Proposal = {
      id: `ATC-PROP-${proposals.length + 40}`,
      title: newTitle,
      description: newDescription,
      status: "Active",
      yesVotes: 1, // Automatically adds creator's YES vote
      noVotes: 0,
      abstainVotes: 0,
      daysLeft: 7,
      voted: true,
    };

    setProposals((prev) => [newProp, ...prev]);
    setShowCreateFlow(false);
    setNewTitle("");
    setNewDescription("");
  };

  return (
    <div className="flex flex-col gap-6 mt-8 pb-12 w-full max-w-6xl mx-auto relative px-4 sm:px-0">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-4">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400 shadow-[0_0_15px_rgba(245,158,11,0.2)] shrink-0">
            <Landmark className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
              On-Chain Governance
            </h2>
            <p className="text-sm font-light text-slate-400">
              ATC-9900 DAO Abstimmungsprozesse, Proposals und Updates
            </p>
          </div>
        </div>

        <button
          onClick={() => setShowPreFlight(true)}
          className="flex items-center justify-center gap-2 px-6 py-3 bg-amber-500 hover:bg-amber-400 text-slate-900 rounded-xl transition-all shadow-[0_0_20px_rgba(245,158,11,0.3)] font-bold text-sm md:w-auto w-full"
        >
          <FileText className="w-4 h-4" />
          Update vorschlagen
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
        <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl p-6 relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-amber-500/5 rounded-full blur-[50px]" />
          <div className="text-slate-500 text-xs font-mono font-bold tracking-widest uppercase mb-2">
            Treasury Balance
          </div>
          <div className="text-3xl font-semibold text-white mb-1">
            4.5M <span className="text-lg text-slate-500">ATC</span>
          </div>
          <div className="text-sm text-amber-400 flex items-center gap-1">
            <ArrowUpRight className="w-3 h-3" /> +12.5k this epoch
          </div>
        </div>
        <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl p-6 relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-blue-500/5 rounded-full blur-[50px]" />
          <div className="text-slate-500 text-xs font-mono font-bold tracking-widest uppercase mb-2">
            Active Proposals
          </div>
          <div className="text-3xl font-semibold text-white mb-1">1</div>
          <div className="text-sm text-slate-400">Needs quorum in 3 days</div>
        </div>
        <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl p-6 relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-emerald-500/5 rounded-full blur-[50px]" />
          <div className="text-slate-500 text-xs font-mono font-bold tracking-widest uppercase mb-2">
            Voter Participation
          </div>
          <div className="text-3xl font-semibold text-white mb-1">68.2%</div>
          <div className="text-sm text-emerald-400 flex items-center gap-1">
            <ArrowUpRight className="w-3 h-3" /> +2.4% vs last
          </div>
        </div>
      </div>

      <GitOpsAutoAudit />

      <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl overflow-hidden mt-6">
        <div className="px-6 py-5 border-b border-atc-border/50 bg-[#060a16] flex items-center justify-between">
          <h3 className="text-lg font-medium text-white flex items-center gap-2">
            <Vote className="w-5 h-5 text-slate-500" /> Recent Proposals
          </h3>
        </div>
        <div className="divide-y divide-atc-border/30">
          {proposals.map((prop, i) => {
            const total = prop.yesVotes + prop.noVotes + prop.abstainVotes;
            const yesPct =
              total > 0 ? Math.round((prop.yesVotes / total) * 100) : 0;
            const noPct =
              total > 0 ? Math.round((prop.noVotes / total) * 100) : 0;
            const abstainPct =
              total > 0 ? Math.round((prop.abstainVotes / total) * 100) : 0;

            return (
              <div
                key={prop.id}
                className="p-6 hover:bg-[#060a16]/50 transition-colors"
              >
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-4">
                  <div>
                    <div className="flex items-center gap-3 mb-2">
                      <span className="text-xs font-mono font-bold text-slate-500 px-2 py-1 bg-atc-border/20 rounded-md border border-atc-border/50">
                        {prop.id}
                      </span>
                      <span
                        className={`text-[10px] uppercase font-mono tracking-widest font-bold px-2 py-1 rounded-md border ${
                          prop.status === "Active"
                            ? "bg-indigo-500/10 text-indigo-400 border-indigo-500/30"
                            : prop.status === "Passed"
                              ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/30"
                              : "bg-rose-500/10 text-rose-400 border-rose-500/30"
                        }`}
                      >
                        {prop.status}
                      </span>
                      {prop.status === "Active" && (
                        <span className="text-xs text-slate-500">
                          {prop.daysLeft} days left
                        </span>
                      )}
                    </div>
                    <h4 className="text-white font-medium text-lg">
                      {prop.title}
                    </h4>
                    {prop.description && (
                      <p className="text-slate-400 text-sm mt-1">
                        {prop.description}
                      </p>
                    )}
                  </div>
                  {prop.status === "Active" && (
                    <div className="flex items-center gap-2">
                      <button
                        onClick={() => handleVote(prop.id, "yes")}
                        disabled={prop.voted}
                        className={`px-4 py-2 bg-emerald-500/10 border rounded-lg text-sm font-medium transition-colors flex items-center gap-1 ${prop.voted ? "text-emerald-500/50 border-emerald-500/10" : "text-emerald-500 hover:bg-emerald-500/20 border-emerald-500/30"}`}
                      >
                        <CheckCircle2 className="w-4 h-4" /> Yes
                      </button>
                      <button
                        onClick={() => handleVote(prop.id, "no")}
                        disabled={prop.voted}
                        className={`px-4 py-2 bg-rose-500/10 border rounded-lg text-sm font-medium transition-colors flex items-center gap-1 ${prop.voted ? "text-rose-500/50 border-rose-500/10" : "text-rose-500 hover:bg-rose-500/20 border-rose-500/30"}`}
                      >
                        <XCircle className="w-4 h-4" /> No
                      </button>
                    </div>
                  )}
                </div>

                <div className="mt-4">
                  <div className="flex justify-between text-xs font-mono mb-2">
                    <span className="text-emerald-400">{yesPct}% Yes</span>
                    <span className="text-slate-500">
                      {abstainPct}% Abstain
                    </span>
                    <span className="text-rose-400">{noPct}% No</span>
                  </div>
                  <div className="w-full h-2 rounded-full overflow-hidden flex">
                    <div
                      className="bg-emerald-500 transition-all duration-500"
                      style={{ width: `${yesPct}%` }}
                    />
                    <div
                      className="bg-slate-600 transition-all duration-500"
                      style={{ width: `${abstainPct}%` }}
                    />
                    <div
                      className="bg-rose-500 transition-all duration-500"
                      style={{ width: `${noPct}%` }}
                    />
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      <AnimatePresence>
        {showPreFlight && (
          <PreFlightCheckModal
            onComplete={() => {
              setShowPreFlight(false);
              setShowCreateFlow(true);
            }}
            onCancel={() => setShowPreFlight(false)}
          />
        )}

        {showCreateFlow && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
          >
            <motion.div
              initial={{ scale: 0.95, y: 10 }}
              animate={{ scale: 1, y: 0 }}
              exit={{ scale: 0.95, y: 10 }}
              className="bg-slate-900 border border-amber-500/20 rounded-2xl p-6 w-full max-w-lg shadow-2xl relative"
            >
              <button
                onClick={() => setShowCreateFlow(false)}
                className="absolute top-4 right-4 text-slate-500 hover:text-white transition-colors"
                type="button"
              >
                <X className="w-5 h-5" />
              </button>

              <div className="flex items-center gap-3 mb-6">
                <div className="w-10 h-10 rounded-lg bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400">
                  <FileText className="w-5 h-5" />
                </div>
                <div>
                  <h3 className="text-xl font-bold text-white">
                    Update vorschlagen
                  </h3>
                  <p className="text-xs text-slate-400">
                    Neues Proposal für die Blockchain einreichen
                  </p>
                </div>
              </div>

              <div className="mb-4 bg-blue-500/10 border border-blue-500/20 rounded-xl p-3 flex gap-3 text-sm text-blue-200">
                <AlertTriangle className="w-4 h-4 shrink-0 mt-0.5 text-blue-400" />
                <p>
                  Ein Update-Vorschlag wird on-chain gespeichert. Die Community
                  hat 7 Tage Zeit, darüber abzustimmen, bevor das Protokoll
                  automatisch aktualisiert wird.
                </p>
              </div>

              <form onSubmit={submitProposal} className="space-y-4">
                <div>
                  <label className="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-1">
                    Titel des Updates
                  </label>
                  <input
                    type="text"
                    required
                    value={newTitle}
                    onChange={(e) => setNewTitle(e.target.value)}
                    placeholder="z.B. Erhöhung der Blockgröße"
                    className="w-full bg-black/40 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-amber-500/50 transition-colors"
                  />
                </div>
                <div>
                  <label className="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-1">
                    Beschreibung & Begründung
                  </label>
                  <textarea
                    required
                    value={newDescription}
                    onChange={(e) => setNewDescription(e.target.value)}
                    placeholder="Warum sollte dieses Update durchgeführt werden?"
                    className="w-full bg-black/40 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-amber-500/50 transition-colors min-h-[120px] resize-none"
                  />
                </div>
                <div className="pt-2 flex justify-end gap-3">
                  <button
                    type="button"
                    onClick={() => setShowCreateFlow(false)}
                    className="px-5 py-2.5 rounded-xl border border-white/10 text-white hover:bg-white/5 transition-colors font-medium text-sm"
                  >
                    Abbrechen
                  </button>
                  <button
                    type="submit"
                    disabled={!newTitle.trim() || !newDescription.trim()}
                    className="px-5 py-2.5 bg-amber-500 hover:bg-amber-400 disabled:opacity-50 disabled:cursor-not-allowed text-slate-900 font-bold rounded-xl transition-colors text-sm"
                  >
                    Proposal einreichen
                  </button>
                </div>
              </form>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
