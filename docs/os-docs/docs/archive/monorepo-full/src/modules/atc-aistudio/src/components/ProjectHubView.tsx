// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { BookOpen, Map as MapIcon, CheckSquare, ChevronRight, Activity, Plus, Search, Folder, Trash2, Calendar, CheckCircle } from 'lucide-react';
import { WIKI_CONTENT } from '../wikiData';
import { ROADMAP_DATA } from '../roadmapData';

interface Todo {
  id: string;
  text: string;
  completed: boolean;
  createdAt: number;
  network: 'devnet' | 'testnet' | 'mainnet' | 'all';
}

export function ProjectHubView() {
  const [activeTab, setActiveTab] = useState<'hub' | 'wiki' | 'roadmap' | 'todos'>('hub');
  const [activeNetwork, setActiveNetwork] = useState<'mainnet' | 'testnet' | 'devnet'>('mainnet');
  
  // ToDo State
  const [todos, setTodos] = useState<Todo[]>(() => {
    try {
      const saved = localStorage.getItem('app_todos_v8_enterprise');
      if (saved) return JSON.parse(saved);
      const now = Date.now();
      const H = 1000 * 60 * 60;
      return [
        { id: '1', text: 'Core/Crypto: Post-Quantum Kyber-768 Schlüsselaustausch formal beweisen', completed: true, createdAt: now - H * 72 },
        { id: '2', text: 'Core/Consensus: BFT Modell (N >= 3f + 1) in TLA+ verifizieren', completed: true, createdAt: now - H * 70 },
        { id: '3', text: 'Core/Network: Kademlia O(log N) Routing Metriken auswerten', completed: true, createdAt: now - H * 68 },
        { id: '4', text: 'VM/Compiler: WASM Determinismus für Floating-Point Ops absichern', completed: true, createdAt: now - H * 66 },
        { id: '5', text: 'Storage/CRDT: Vector Clocks V(a) < V(b) Resolution Logs schreiben', completed: true, createdAt: now - H * 64 },
        { id: '6', text: 'AI/Orchestrator: ONNX Quantization auf INT4 für Layer 2 Nodes testen', completed: true, createdAt: now - H * 62 },
        { id: '7', text: 'ZKP/Snarks: PLONK Circuit Parametereinstellung optimieren (O(1) verify)', completed: true, createdAt: now - H * 60 },
        { id: '8', text: 'Repositories: A-TownChain/atc-lang Repository auf GitHub erstellen und Struktur pushen', completed: true, createdAt: now - H * 58 },
        { id: '9', text: 'Repositories: A-TownChain/atc-lang-wiki Repository für ATC-Lang Dokumentation initialisieren', completed: true, createdAt: now - H * 56 },
        { id: '9b', text: 'Repositories: A-TownChain/atc-pack Repository für Package Manager aufsetzen', completed: true, createdAt: now - H * 55 },
        { id: '9c', text: 'Repositories: A-TownChain/atc-trace Repository für Time-Travel Debugger aufsetzen', completed: true, createdAt: now - H * 55 },
        { id: '10', text: 'CI/CD: GitHub Actions Workflow für ATC-Lang Compiler (atc-lang) automatisieren', completed: true, createdAt: now - H * 54 },
        { id: '11', text: 'CI/CD: Dependabot für ATC-Lang Dependencies und Compiler-Crates einrichten', completed: true, createdAt: now - H * 52 },
        { id: '12', text: 'Wiki/Doc: 10 weitere formale Spezifikationen in die Enterprise Wiki einarbeiten', completed: true, createdAt: now - H * 50 },
        { id: '13', text: 'Roadmap/Plan: Phase 9-18 in der D3 Roadmap mit Gantt-Timeline konfigurieren', completed: true, createdAt: now - H * 48 },
        { id: '14', text: 'Smart Contracts: ATC-Lang Hoare-Logik Pre/Post-Conditions in VM integrieren', completed: true, createdAt: now - H * 46 },
        { id: '15', text: 'Netzwerk: Warp Sync / State Snapshot Server Backend aufsetzen', completed: true, createdAt: now - H * 44 },
        { id: '16', text: 'Sicherheit: Threshold-Encryption für Mempool (MEV Schutz) implementieren', completed: true, createdAt: now - H * 42 },
        { id: '17', text: 'UX/Settings: Design-Vorgaben für Ocean-Deep, Sunset-Red, Cyber-Cyan abspeichern', completed: true, createdAt: now - H * 40 }
      ];
    } catch {
      return [];
    }
  });
  const [newTodo, setNewTodo] = useState('');

  // Roadmap State
  const [roadmapSearch, setRoadmapSearch] = useState('');
  
  // Wiki State
  const [activeWikiCat, setActiveWikiCat] = useState(WIKI_CONTENT[0].id);

  const [showSaveToast, setShowSaveToast] = useState(false);

  useEffect(() => {
    localStorage.setItem('app_todos_v8_enterprise', JSON.stringify(todos));
    if (todos.length > 0) {
      setShowSaveToast(true);
      const timer = setTimeout(() => setShowSaveToast(false), 2000);
      return () => clearTimeout(timer);
    }
  }, [todos]);

  const handleAddTodo = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTodo.trim()) return;
    setTodos([{ id: Date.now().toString(), text: newTodo.trim(), completed: true, createdAt: Date.now(), network: activeNetwork }, ...todos]);
    setNewTodo('');
  };

  const toggleTodo = (id: string) => {
    setTodos(todos.map(t => t.id === id ? { ...t, completed: !t.completed } : t));
  };

  const deleteTodo = (id: string) => {
    setTodos(todos.filter(t => t.id !== id));
  };

  // Derived metrics
  const ROADMAP_ACTIVE = ROADMAP_DATA.filter(p => !p.network || p.network === 'all' || p.network === activeNetwork);
  const totalPhaseCount = ROADMAP_ACTIVE.length;
  const completedPhaseCount = ROADMAP_ACTIVE.filter(p => p.status === 'Abgeschlossen').length;
  const progressPercent = Math.round((completedPhaseCount / totalPhaseCount) * 100) || 0;
  
  const totalTodos = todos.length;
  const completedTodos = todos.filter(t => t.completed).length;

  const wikiDoc = WIKI_CONTENT.find(c => c.id === activeWikiCat) || WIKI_CONTENT[0];

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200">
      {/* Header Navigation */}
      <div className="flex flex-col sm:flex-row items-center justify-between gap-4 p-6 bg-[#090b14] border-b border-atc-border/50 shrink-0">
        <div className="flex items-center gap-4">
          <div className="p-3 rounded-lg bg-indigo-500/10 border border-indigo-500/30 shadow-[0_0_15px_rgba(99,102,241,0.15)]">
            <Activity className="w-6 h-6 text-indigo-400" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-white tracking-tight">ATC-OS Project Hub</h2>
            <p className="text-xs text-slate-400 font-mono mt-0.5">Unified Wiki, Roadmap & Tasks • ATC-SYNC</p>
          </div>
        </div>

        <div className="flex bg-black/40 p-1 rounded-xl border border-white/5 shadow-inner">
          {[
            { id: 'hub', label: 'Dashboard', icon: Activity },
            { id: 'wiki', label: 'Wiki & Docs', icon: BookOpen },
            { id: 'roadmap', label: 'Roadmap', icon: MapIcon },
            { id: 'todos', label: 'Tasks', icon: CheckSquare }
          ].map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`flex items-center gap-2 px-5 py-2 rounded-lg font-medium text-sm transition-all ${
                activeTab === tab.id 
                  ? 'bg-atc-cyan text-slate-900 shadow-sm' 
                  : 'text-slate-400 hover:text-white hover:bg-white/5'
              }`}
            >
              <tab.icon className="w-4 h-4" />
              {tab.label}
            </button>
          ))}
        </div>
      </div>

      {/* Network Context Strip */}
      <div className="flex items-center justify-between px-6 py-2 bg-atc-cyan/5 border-b border-atc-cyan/10 shrink-0">
        <div className="flex items-center gap-2">
          <span className="text-xs font-bold text-atc-cyan uppercase tracking-widest font-mono">Context:</span>
          <div className="flex bg-black/40 rounded-lg p-0.5 border border-white/5">
            {(['devnet', 'testnet', 'mainnet'] as const).map(net => (
               <button
                 key={net}
                 onClick={() => setActiveNetwork(net)}
                 className={`px-3 py-1 text-xs rounded-md uppercase font-mono tracking-wider transition-colors ${
                   activeNetwork === net ? 'bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/30' : 'text-slate-500 hover:text-slate-300'
                 }`}
               >
                 {net}
               </button>
            ))}
          </div>
        </div>
        <div className="hidden sm:flex items-center gap-2 text-[10px] text-slate-400 font-mono">
           <Activity className="w-3 h-3 text-atc-cyan" />
           {activeNetwork !== 'mainnet' ? 'Devnet & Testnet logically mirror Mainnet state' : 'Live Production State'}
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto custom-scrollbar p-6">
        <AnimatePresence mode="wait">
          {activeTab === 'hub' && (
            <motion.div
              key="hub"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="max-w-6xl mx-auto space-y-6"
            >
               {/* Dashboard Widgets */}
               <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                  {/* Progress Card */}
                  <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6">
                     <h3 className="text-sm font-bold text-white mb-4 flex items-center gap-2"><MapIcon className="w-4 h-4 text-indigo-400"/> Roadmap Progress</h3>
                     <div className="text-3xl font-bold text-white mb-2">{completedPhaseCount} <span className="text-sm text-slate-500 font-normal">/ {totalPhaseCount} Phases</span></div>
                     <div className="w-full h-2 bg-black/50 rounded-full overflow-hidden mt-4">
                        <div className="h-full bg-indigo-500 rounded-full" style={{ width: `${progressPercent}%` }} />
                     </div>
                     <p className="text-xs text-slate-400 mt-3">{progressPercent}% Overall Completion</p>
                  </div>

                  {/* Todo Card */}
                  <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6">
                     <h3 className="text-sm font-bold text-white mb-4 flex items-center gap-2"><CheckSquare className="w-4 h-4 text-emerald-400"/> Task Completion</h3>
                     <div className="text-3xl font-bold text-white mb-2">{completedTodos} <span className="text-sm text-slate-500 font-normal">/ {totalTodos} Tasks</span></div>
                     <div className="w-full h-2 bg-black/50 rounded-full overflow-hidden mt-4">
                        <div className="h-full bg-emerald-500 rounded-full" style={{ width: `${totalTodos > 0 ? (completedTodos/totalTodos)*100 : 0}%` }} />
                     </div>
                     <p className="text-xs text-slate-400 mt-3">{todos.filter(t => !t.completed).length} open tasks remaining</p>
                  </div>

                  {/* Quick Wiki Links */}
                  <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6">
                     <h3 className="text-sm font-bold text-white mb-4 flex items-center gap-2"><BookOpen className="w-4 h-4 text-amber-400"/> Latest Documentation</h3>
                     <div className="space-y-3">
                        {WIKI_CONTENT.filter(doc => !(doc as any).network || (doc as any).network === 'all' || (doc as any).network === activeNetwork).slice(0, 3).map(doc => (
                           <button 
                             key={doc.id}
                             onClick={() => { setActiveTab('wiki'); setActiveWikiCat(doc.id); }}
                             className="w-full text-left flex items-center justify-between p-2 rounded-lg bg-white/5 hover:bg-white/10 transition-colors border border-white/5"
                           >
                              <span className="text-sm text-slate-300 truncate">{doc.title}</span>
                              <ChevronRight className="w-4 h-4 text-slate-500" />
                           </button>
                        ))}
                     </div>
                  </div>
               </div>

               {/* Upcoming Phases */}
               <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6">
                 <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">Upcoming Roadmap Phases</h3>
                 <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {ROADMAP_ACTIVE.filter(p => p.status === 'Laufend' || p.status === 'Geplant' || !p.status).slice(0, 3).map(phase => (
                       <div key={phase.id} className="p-4 bg-black/40 border border-white/5 rounded-xl">
                          <div className="flex justify-between items-start mb-2">
                             <div className="px-2 py-1 bg-amber-500/10 text-amber-500 text-[10px] font-bold uppercase rounded border border-amber-500/20">{phase.status || 'Planned'}</div>
                             <span className="text-xs text-slate-500 font-mono">{phase.timeframe}</span>
                          </div>
                          <h4 className="text-sm font-bold text-white mb-2">{phase.title}</h4>
                          <ul className="space-y-1">
                             {phase.goals?.slice(0, 2).map((g, i) => (
                                <li key={i} className="text-xs text-slate-400 flex items-start gap-2">
                                  <div className="w-1 h-1 rounded-full bg-atc-cyan mt-1.5 shrink-0" />
                                  <span className="line-clamp-2">{g.text}</span>
                                </li>
                             ))}
                          </ul>
                       </div>
                    ))}
                 </div>
               </div>
            </motion.div>
          )}

          {activeTab === 'wiki' && (
            <motion.div
              key="wiki"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="flex h-full gap-6"
            >
              {/* Sidebar */}
              <div className="w-64 shrink-0 flex flex-col gap-2">
                 <div className="relative mb-4">
                   <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
                   <input type="text" placeholder="Search docs..." className="w-full bg-black/40 border border-atc-border/50 rounded-lg pl-9 pr-4 py-2 text-sm text-white focus:outline-none focus:border-atc-cyan transition-colors" />
                 </div>
                 <div className="flex-1 overflow-y-auto space-y-1 pr-2 custom-scrollbar">
                   {WIKI_CONTENT.filter(doc => !(doc as any).network || (doc as any).network === 'all' || (doc as any).network === activeNetwork).map(doc => (
                     <button
                       key={doc.id}
                       onClick={() => setActiveWikiCat(doc.id)}
                       className={`w-full text-left px-3 py-2 rounded-lg text-sm transition-all ${
                         activeWikiCat === doc.id ? 'bg-atc-cyan/10 text-atc-cyan border border-atc-cyan/30' : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
                       }`}
                     >
                       <div className="font-medium truncate">{doc.title}</div>
                       <div className="text-[10px] text-slate-500 mt-0.5">{doc.category}</div>
                     </button>
                   ))}
                 </div>
              </div>
              
              {/* Content */}
              <div className="flex-1 bg-[#090b14] border border-atc-border/50 rounded-2xl p-8 overflow-y-auto custom-scrollbar">
                 <div className="max-w-3xl">
                   <div className="flex items-center gap-2 text-xs font-mono text-atc-cyan mb-4">
                      <Folder className="w-4 h-4" /> {wikiDoc.category} / {wikiDoc.subcategory}
                   </div>
                   <h1 className="text-3xl font-bold text-white mb-6 border-b border-white/10 pb-4 flex items-center justify-between">
                     <span>{wikiDoc.title}</span>
                     <span className="text-sm font-mono text-slate-500 bg-white/5 px-3 py-1 rounded-full border border-white/10">Version 1.{wikiDoc.revisions?.length || 0}</span>
                   </h1>
                   <div className="prose prose-invert prose-slate max-w-none prose-headings:text-white prose-a:text-atc-cyan">
                      <p className="text-slate-300 leading-relaxed text-sm whitespace-pre-wrap">{wikiDoc.text}</p>
                      
                      {wikiDoc.ascii && (
                         <div className="mt-8">
                           <h3 className="text-lg font-bold text-white mb-4">System Architecture</h3>
                           <pre className="p-4 bg-black/60 border border-white/10 rounded-xl overflow-x-auto text-xs text-atc-cyan font-mono leading-relaxed">
                             {wikiDoc.ascii}
                           </pre>
                         </div>
                      )}

                      {wikiDoc.table && (
                         <div className="mt-8">
                            <h3 className="text-lg font-bold text-white mb-4">Component Specs</h3>
                            <div className="overflow-x-auto bg-black/40 rounded-xl border border-white/10">
                              <table className="w-full text-sm text-left">
                                <thead className="text-xs text-slate-400 uppercase bg-white/5 border-b border-white/10">
                                   <tr>
                                     <th className="px-4 py-3">Component</th>
                                     <th className="px-4 py-3">Status</th>
                                   </tr>
                                </thead>
                                <tbody className="divide-y divide-white/5">
                                   {wikiDoc.table.map((row: any, i: number) => (
                                     <tr key={i} className="hover:bg-white/5">
                                        <td className="px-4 py-3 font-medium text-slate-200">
                                          <div>{row.component}</div>
                                          <div className="text-xs text-slate-500 font-normal mt-0.5">{row.desc}</div>
                                        </td>
                                        <td className="px-4 py-3">
                                          <span className="px-2 py-1 text-[10px] font-bold uppercase rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">{row.status}</span>
                                        </td>
                                     </tr>
                                   ))}
                                </tbody>
                              </table>
                            </div>
                         </div>
                      )}

                      {/* Sub Tags and Cross References */}
                      <div className="mt-10 pt-6 border-t border-white/10 grid grid-cols-1 md:grid-cols-2 gap-6">
                        {wikiDoc.tags && wikiDoc.tags.length > 0 && (
                          <div>
                            <h4 className="text-sm font-bold text-slate-400 mb-3 flex items-center gap-2">
                              <Search className="w-4 h-4" /> Sub Tags
                            </h4>
                            <div className="flex flex-wrap gap-2">
                              {wikiDoc.tags.map((tag: string) => (
                                <span key={tag} className="px-2.5 py-1 text-xs font-mono rounded-md bg-atc-cyan/10 text-atc-cyan border border-atc-cyan/20">
                                  #{tag}
                                </span>
                              ))}
                            </div>
                          </div>
                        )}
                        
                        {wikiDoc.related && wikiDoc.related.length > 0 && (
                          <div>
                            <h4 className="text-sm font-bold text-slate-400 mb-3 flex items-center gap-2">
                              <BookOpen className="w-4 h-4" /> Querverweise (Related)
                            </h4>
                            <div className="flex flex-col gap-2">
                              {wikiDoc.related.map((relId: string) => {
                                const relDoc = WIKI_CONTENT.find(w => w.id === relId);
                                if (!relDoc) return null;
                                return (
                                  <button 
                                    key={relId}
                                    onClick={() => setActiveWikiCat(relId)}
                                    className="text-left text-xs bg-[#090b14] border border-white/5 hover:border-atc-cyan/50 hover:bg-atc-cyan/5 transition-colors p-2 rounded-lg flex items-center justify-between group"
                                  >
                                    <span className="text-slate-300 group-hover:text-white truncate">{relDoc.title}</span>
                                    <ChevronRight className="w-3 h-3 text-slate-500 group-hover:text-atc-cyan" />
                                  </button>
                                );
                              })}
                            </div>
                          </div>
                        )}
                      </div>
                   </div>
                 </div>
              </div>
            </motion.div>
          )}

          {activeTab === 'roadmap' && (
            <motion.div
              key="roadmap"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="max-w-6xl mx-auto flex flex-col gap-6"
            >
               <div className="flex items-center justify-between bg-[#090b14] border border-atc-border/50 rounded-2xl p-4 shrink-0">
                  <div className="relative w-72">
                    <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
                    <input 
                      type="text" 
                      placeholder="Search phases or goals..." 
                      value={roadmapSearch}
                      onChange={e => setRoadmapSearch(e.target.value)}
                      className="w-full bg-black/40 border border-atc-border/50 rounded-lg pl-9 pr-4 py-2 text-sm text-white focus:outline-none focus:border-atc-cyan transition-colors" 
                    />
                  </div>
               </div>
               
               <div className="space-y-4">
                  {ROADMAP_ACTIVE.filter(p => roadmapSearch ? p.title.toLowerCase().includes(roadmapSearch.toLowerCase()) || p.goals?.some((g: any) => g.text.toLowerCase().includes(roadmapSearch.toLowerCase())) : true).map((phase, idx) => (
                    <div key={phase.id} className="bg-[#090b14] border border-atc-border/50 rounded-2xl p-6 group hover:border-atc-cyan/30 transition-colors relative overflow-hidden">
                       {/* Timeline visual line */}
                       <div className="absolute left-[29px] top-0 bottom-0 w-0.5 bg-white/5 group-hover:bg-atc-cyan/20 transition-colors z-0" />
                       
                       <div className="flex gap-6 relative z-10">
                          <div className="w-4 h-4 rounded-full border-[3px] mt-1 shrink-0 bg-[#090b14] border-slate-600 group-hover:border-atc-cyan transition-colors" />
                          <div className="flex-1">
                             <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-4">
                                <div>
                                  <h3 className="text-lg font-bold text-white tracking-tight">{phase.title}</h3>
                                  <div className="text-xs text-slate-500 font-mono mt-1">{phase.timeframe}</div>
                                </div>
                                <span className={`px-2.5 py-1 text-[10px] font-bold uppercase rounded border ${
                                  phase.status === 'Abgeschlossen' ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20' :
                                  phase.status === 'Laufend' ? 'bg-amber-500/10 text-amber-500 border-amber-500/20' :
                                  'bg-slate-500/10 text-slate-400 border-slate-500/20'
                                }`}>{phase.status || 'Planned'}</span>
                             </div>

                             <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div>
                                   <h4 className="text-xs font-bold text-slate-400 uppercase tracking-widest mb-3">Key Modules (150+)</h4>
                                   <div className="flex flex-wrap gap-2">
                                     {phase.sections?.[0]?.items.map(item => (
                                        <span key={item} className="px-2 py-1 rounded bg-black/40 border border-white/5 text-xs text-slate-300">{item}</span>
                                     ))}
                                     {phase.sections?.length > 1 && <span className="text-xs text-slate-500 self-center">+{phase.sections.length - 1} more sets</span>}
                                   </div>
                                </div>
                                <div>
                                   <h4 className="text-xs font-bold text-slate-400 uppercase tracking-widest mb-3">Target Goals</h4>
                                   <ul className="space-y-2">
                                     {phase.goals?.map((goal: any, index: number) => (
                                        <li key={index} className="flex gap-2 text-sm text-slate-300">
                                          {goal.completed ? <CheckCircle className="w-4 h-4 text-emerald-400 shrink-0 mt-0.5" /> : <div className="w-4 h-4 rounded-full border border-slate-500 shrink-0 mt-0.5" />}
                                          <span>{goal.text}</span>
                                        </li>
                                     ))}
                                   </ul>
                                </div>
                             </div>
                          </div>
                       </div>
                    </div>
                  ))}
               </div>
            </motion.div>
          )}

          {activeTab === 'todos' && (
            <motion.div
              key="todos"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="max-w-4xl mx-auto flex flex-col h-full bg-[#090b14] border border-atc-border/50 rounded-2xl overflow-hidden"
            >
              <div className="p-4 border-b border-white/5 bg-black/20 shrink-0">
                 <form onSubmit={handleAddTodo} className="flex gap-3">
                   <input
                     type="text"
                     value={newTodo}
                     onChange={(e) => setNewTodo(e.target.value)}
                     placeholder="Add a new project task..."
                     className="flex-1 bg-black/40 border border-white/10 rounded-xl px-4 py-2.5 text-sm text-white focus:outline-none focus:border-atc-cyan transition-colors"
                   />
                   <button type="submit" disabled={!newTodo.trim()} className="px-5 py-2.5 flex items-center justify-center rounded-xl bg-atc-cyan text-slate-900 font-bold transition-all hover:bg-atc-cyan/90 disabled:opacity-50">
                     <Plus className="w-5 h-5" />
                   </button>
                 </form>
              </div>
              <div className="flex-1 overflow-y-auto p-4 space-y-2 custom-scrollbar">
                 {todos.filter(t => !t.network || t.network === 'all' || t.network === activeNetwork).length === 0 ? (
                   <div className="h-full flex flex-col items-center justify-center text-slate-500 gap-4">
                     <CheckSquare className="w-12 h-12 opacity-20" />
                     <p>All clear! No pending tasks in ATC-OS.</p>
                   </div>
                 ) : (
                   todos.filter(t => !t.network || t.network === 'all' || t.network === activeNetwork).map(todo => (
                     <div key={todo.id} className={`group flex items-start gap-3 p-4 rounded-xl border transition-all ${todo.completed ? 'bg-white/5 border-transparent opacity-60' : 'bg-black/40 border-white/10 hover:border-atc-cyan/30'}`}>
                        <button onClick={() => toggleTodo(todo.id)} className={`mt-0.5 w-5 h-5 rounded-full border flex items-center justify-center shrink-0 transition-colors ${todo.completed ? 'bg-emerald-500 border-emerald-500 text-white' : 'border-slate-500 hover:border-atc-cyan'}`}>
                          {todo.completed && <CheckSquare className="w-3 h-3" />}
                        </button>
                        <div className="flex-1 min-w-0">
                           <p className={`text-sm ${todo.completed ? 'text-slate-400 line-through' : 'text-slate-200'}`}>{todo.text}</p>
                           <div className="flex items-center gap-2 mt-2 text-[10px] text-slate-500">
                             <Calendar className="w-3 h-3" />
                             {new Date(todo.createdAt).toLocaleString(undefined, { dateStyle: 'medium', timeStyle: 'short' })}
                           </div>
                        </div>
                        <button onClick={() => deleteTodo(todo.id)} className="p-2 text-slate-500 hover:text-red-400 hover:bg-red-400/10 rounded-lg transition-colors opacity-0 group-hover:opacity-100">
                           <Trash2 className="w-4 h-4" />
                        </button>
                     </div>
                   ))
                 )}
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      <AnimatePresence>
        {showSaveToast && (
          <motion.div
            initial={{ opacity: 0, y: 50, scale: 0.9 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: 20, scale: 0.9 }}
            className="absolute bottom-6 right-6 flex items-center gap-2 px-4 py-2 bg-[#090b14]/90 backdrop-blur-md border border-emerald-500/30 rounded-full shadow-[0_0_15px_rgba(16,185,129,0.2)] text-emerald-400 z-50 font-mono text-xs"
          >
            <CheckCircle className="w-4 h-4" /> State Saved
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
