import React, { useState } from 'react';
import { Landmark, Vote, FileText, CheckCircle2, XCircle, ArrowUpRight, X, AlertTriangle } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

type Proposal = {
  id: string;
  title: string;
  description: string;
  status: 'Active' | 'Passed' | 'Rejected';
  yesVotes: number;
  noVotes: number;
  abstainVotes: number;
  daysLeft: number;
  voted?: boolean;
};

const INITIAL_PROPOSALS: Proposal[] = [
  { id: 'ATC-PROP-42', title: 'Aktivierung des WASM Execution Layers auf Mainnet', description: 'Dieses Update ermöglicht die Ausführung von WASM-kompilierten Smart Contracts auf dem Mainnet und erhöht so die Ausführungsgeschwindigkeit.', status: 'Active', yesVotes: 78, noVotes: 12, abstainVotes: 10, daysLeft: 3 },
  { id: 'ATC-PROP-41', title: 'Reduzierung der P2P Gossip Protokoll Latenz', description: 'Optimierung des P2P Netzwerks.', status: 'Passed', yesVotes: 92, noVotes: 5, abstainVotes: 3, daysLeft: 0 },
  { id: 'ATC-PROP-40', title: 'Integration von Kyber Post-Quantum Signaturen (Testnet)', description: 'Sicherheits-Upgrade gegen Post-Quantum Angriffe.', status: 'Passed', yesVotes: 85, noVotes: 10, abstainVotes: 5, daysLeft: 0 },
  { id: 'ATC-PROP-39', title: 'Erhöhung der Validator Base Rewards um 5%', description: 'Netzwerkauslastungsbedingte Budgetanpassung.', status: 'Rejected', yesVotes: 35, noVotes: 60, abstainVotes: 5, daysLeft: 0 },
];

export function GovernanceView() {
  const [proposals, setProposals] = useState<Proposal[]>(INITIAL_PROPOSALS);
  const [showCreateFlow, setShowCreateFlow] = useState(false);
  const [newTitle, setNewTitle] = useState('');
  const [newDescription, setNewDescription] = useState('');

  const handleVote = (id: string, type: 'yes' | 'no') => {
    setProposals(prev => prev.map(p => {
      if (p.id === id && !p.voted && p.status === 'Active') {
         return {
           ...p,
           yesVotes: type === 'yes' ? p.yesVotes + 1 : p.yesVotes,
           noVotes: type === 'no' ? p.noVotes + 1 : p.noVotes,
           voted: true
         };
      }
      return p;
    }));
  };

  const activeCount = proposals.filter(p => p.status === 'Active').length;

  const submitProposal = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTitle.trim()) return;

    const newProp: Proposal = {
      id: `ATC-PROP-${proposals.length + 40}`,
      title: newTitle,
      description: newDescription,
      status: 'Active',
      yesVotes: 1, // Automatically adds creator's YES vote
      noVotes: 0,
      abstainVotes: 0,
      daysLeft: 7,
      voted: true 
    };

    setProposals(prev => [newProp, ...prev]);
    setShowCreateFlow(false);
    setNewTitle('');
    setNewDescription('');
  };

  return (
    <div className="flex flex-col gap-6 mt-8 pb-12 w-full max-w-6xl mx-auto relative px-4 sm:px-0">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-4">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-2xl bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400 shadow-[0_0_15px_rgba(245,158,11,0.2)] shrink-0">
            <Landmark className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">On-Chain Governance</h2>
            <p className="text-sm font-light text-slate-400">ATC-9900 DAO Abstimmungsprozesse, Proposals und Updates</p>
          </div>
        </div>

        <button 
          onClick={() => setShowCreateFlow(true)}
          className="flex items-center justify-center gap-2 px-6 py-3 bg-amber-500 hover:bg-amber-400 text-slate-900 rounded-xl transition-all shadow-[0_0_20px_rgba(245,158,11,0.3)] font-bold text-sm md:w-auto w-full"
        >
          <FileText className="w-4 h-4" />
          Update vorschlagen
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
        <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl p-6 relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-amber-500/5 rounded-full blur-[50px]" />
          <div className="text-slate-500 text-xs font-mono font-bold tracking-widest uppercase mb-2">Treasury Balance</div>
          <div className="text-3xl font-semibold text-white mb-1">4.5M <span className="text-lg text-slate-500">ATC</span></div>
          <div className="text-sm text-amber-400 flex items-center gap-1"><ArrowUpRight className="w-3 h-3" /> +12.5k this epoch</div>
        </div>
        <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl p-6 relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-blue-500/5 rounded-full blur-[50px]" />
          <div className="text-slate-500 text-xs font-mono font-bold tracking-widest uppercase mb-2">Aktive Proposals</div>
          <div className="text-3xl font-semibold text-white mb-1">{activeCount}</div>
          <div className="text-sm text-slate-400">Needs quorum in 3 days</div>
        </div>
        <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl p-6 relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-emerald-500/5 rounded-full blur-[50px]" />
          <div className="text-slate-500 text-xs font-mono font-bold tracking-widest uppercase mb-2">Voter Participation</div>
          <div className="text-3xl font-semibold text-white mb-1">68.2%</div>
          <div className="text-sm text-emerald-400 flex items-center gap-1"><ArrowUpRight className="w-3 h-3" /> +2.4% vs last</div>
        </div>
      </div>

      <div className="bg-[#090b14]/60 border border-atc-border/50 rounded-2xl overflow-hidden">
         <div className="px-6 py-5 border-b border-atc-border/50 bg-[#060a16] flex items-center justify-between">
             <h3 className="text-lg font-medium text-white flex items-center gap-2">
               <Vote className="w-5 h-5 text-slate-500" /> Aktuelle & Vergangene Proposals
             </h3>
         </div>
         <div className="divide-y divide-atc-border/30">
            {proposals.map((prop) => {
              const total = prop.yesVotes + prop.noVotes + prop.abstainVotes;
              const yesPct = total > 0 ? Math.round((prop.yesVotes / total) * 100) : 0;
              const noPct = total > 0 ? Math.round((prop.noVotes / total) * 100) : 0;
              const abstainPct = total > 0 ? Math.round((prop.abstainVotes / total) * 100) : 0;

              return (
              <div key={prop.id} className="p-6 hover:bg-[#060a16]/50 transition-colors">
                 <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-4">
                    <div>
                       <div className="flex items-center gap-3 mb-2">
                          <span className="text-xs font-mono font-bold text-slate-500 px-2 py-1 bg-atc-border/20 rounded-md border border-atc-border/50">{prop.id}</span>
                          <span className={`text-[10px] uppercase font-mono tracking-widest font-bold px-2 py-1 rounded-md border ${
                             prop.status === 'Active' ? 'bg-indigo-500/10 text-indigo-400 border-indigo-500/30' :
                             prop.status === 'Passed' ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30' :
                             'bg-rose-500/10 text-rose-400 border-rose-500/30'
                          }`}>
                             {prop.status}
                          </span>
                          {prop.status === 'Active' && <span className="text-xs text-slate-500 cursor-default" title="Days until voting closes">{prop.daysLeft} days left</span>}
                       </div>
                       <h4 className="text-white font-medium text-lg">{prop.title}</h4>
                       {prop.description && <p className="text-slate-400 text-sm mt-1">{prop.description}</p>}
                    </div>
                    {prop.status === 'Active' && (
                       <div className="flex items-center gap-2 shrink-0 border border-white/5 p-1 rounded-xl bg-black/20">
                          <button 
                            onClick={() => handleVote(prop.id, 'yes')}
                            disabled={prop.voted}
                            className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 ${
                              prop.voted ? 'bg-emerald-500/5 text-emerald-500/50 border border-transparent cursor-not-allowed' : 'bg-emerald-500/10 text-emerald-500 border border-emerald-500/30 hover:bg-emerald-500/20'
                            }`}
                          >
                             <CheckCircle2 className="w-4 h-4" /> {prop.voted ? 'Voted' : 'Yes'}
                          </button>
                          <button 
                            onClick={() => handleVote(prop.id, 'no')}
                            disabled={prop.voted}
                            className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 ${
                              prop.voted ? 'bg-rose-500/5 text-rose-500/50 border border-transparent cursor-not-allowed' : 'bg-rose-500/10 text-rose-500 border border-rose-500/30 hover:bg-rose-500/20'
                            }`}
                          >
                             <XCircle className="w-4 h-4" /> {prop.voted ? 'Voted' : 'No'}
                          </button>
                       </div>
                    )}
                 </div>

                 {/* Voting Results Bar */}
                 <div className="mt-4">
                    <div className="flex justify-between text-xs font-mono mb-2">
                       <span className="text-emerald-400">{yesPct}% Yes ({prop.yesVotes})</span>
                       <span className="text-slate-500">{abstainPct}% Abstain</span>
                       <span className="text-rose-400">{noPct}% No ({prop.noVotes})</span>
                    </div>
                    <div className="w-full h-2 rounded-full overflow-hidden flex">
                       <div className="bg-emerald-500 transition-all duration-500" style={{ width: `${yesPct}%` }} />
                       <div className="bg-slate-600 transition-all duration-500" style={{ width: `${abstainPct}%` }} />
                       <div className="bg-rose-500 transition-all duration-500" style={{ width: `${noPct}%` }} />
                    </div>
                 </div>
              </div>
            )})}
         </div>
      </div>

      {/* Create Proposal Modal/Overlay */}
      <AnimatePresence>
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
                    <h3 className="text-xl font-bold text-white">Update vorschlagen</h3>
                    <p className="text-xs text-slate-400">Neues Proposal für die Blockchain einreichen</p>
                 </div>
              </div>

              <div className="mb-4 bg-blue-500/10 border border-blue-500/20 rounded-xl p-3 flex gap-3 text-sm text-blue-200">
                 <AlertTriangle className="w-4 h-4 shrink-0 mt-0.5 text-blue-400" />
                 <p>Ein Update-Vorschlag wird on-chain gespeichert. Die Community hat 7 Tage Zeit, darüber abzustimmen, bevor das Protokoll automatisch aktualisiert wird.</p>
              </div>

              <form onSubmit={submitProposal} className="space-y-4">
                 <div>
                    <label className="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-1">Titel des Updates</label>
                    <input 
                      type="text" 
                      required
                      value={newTitle}
                      onChange={e => setNewTitle(e.target.value)}
                      placeholder="z.B. Erhöhung der Blockgröße"
                      className="w-full bg-black/40 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-amber-500/50 transition-colors"
                    />
                 </div>
                 <div>
                    <label className="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-1">Beschreibung & Begründung</label>
                    <textarea 
                      required
                      value={newDescription}
                      onChange={e => setNewDescription(e.target.value)}
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
