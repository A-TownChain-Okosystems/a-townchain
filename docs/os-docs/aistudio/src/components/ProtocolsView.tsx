import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Shield, Brain, Activity, Database, CheckCircle, Lock, RefreshCw, Layers } from 'lucide-react';

type Phase = {
  id: string;
  title: string;
  layer: 'ATS' | 'ATC';
  icon: React.ElementType;
  description: string;
  details: string[];
  output: string;
};

const PHASES: Phase[] = [
  {
    id: 'phase1',
    title: 'Decision Generation',
    layer: 'ATS',
    icon: Brain,
    description: 'ATS erzeugt strukturierte Entscheidungsobjekte basierend auf probabilistischer Intelligenz.',
    details: [
      'Policy Generation (Rule Sets, Constraints)',
      'Multi-Agent Coordination',
      'Economic Simulation',
      'Risk Scoring & Predictive Modeling',
    ],
    output: 'ActionPackage (intent_id, objective_vector, etc.)',
  },
  {
    id: 'phase2',
    title: 'Structural & Cryptographic Validation',
    layer: 'ATC',
    icon: Shield,
    description: 'ATC validiert die Entscheidungen kryptografisch und strukturell.',
    details: [
      'Schema-Konformität (ATC Transaction Schema)',
      'Kryptografische Signaturen',
      'State Consistency Rules',
      'Policy Compliance (ATS Governance)',
    ],
    output: 'ValidatedExecutionPlan',
  },
  {
    id: 'phase3',
    title: 'Deterministic Execution',
    layer: 'ATC',
    icon: Lock,
    description: 'ATC führt die validierten Pläne deterministisch und reproduzierbar aus.',
    details: [
      'State Transition Execution',
      'Smart Contract Invocation',
      'Cross-Chain Messaging',
      'Ledger Update (Block Finalization)',
    ],
    output: 'State Transition Output',
  },
  {
    id: 'phase4',
    title: 'Feedback & Reinforcement Loop',
    layer: 'ATS',
    icon: RefreshCw,
    description: 'ATS konsumiert Telemetrie/Logs und optimiert kontinuierlich Modelle und Policys.',
    details: [
      'Konsumiert Execution Trace Logs & Performance Metrics',
      'Risk Deviations & Economic Impact Signals',
      'Aktualisiert Policy Weights & Routing Logic',
      'Agent Behavior Models Anpassung',
    ],
    output: 'Telemetry Stream',
  },
];

export function ProtocolsView() {
  const [activePhase, setActivePhase] = useState<string>(PHASES[0].id);

  return (
    <div className="flex-1 overflow-auto bg-slate-900 text-slate-200 p-8 h-full">
      <div className="max-w-6xl mx-auto flex flex-col h-full gap-8">
        
        <div className="flex items-center gap-4 border-b border-slate-700 pb-4">
          <Layers className="w-8 h-8 text-indigo-400" />
          <div>
            <h1 className="text-2xl font-bold text-white">ATC ↔ ATS Protocol Model</h1>
            <p className="text-sm text-slate-400">Deterministic Execution & Probabilistic Intelligence Loop</p>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 flex-1 min-h-0">
          
          {/* Left: Interactive Diagram */}
          <div className="bg-slate-800/50 p-6 rounded-xl border border-slate-700/50 flex flex-col gap-6 relative">
            <h2 className="text-lg font-bold text-slate-100 mb-2">Interaktionspipeline</h2>
            
            <div className="relative flex-1 flex flex-col gap-4">
              {PHASES.map((phase, index) => {
                const isActive = activePhase === phase.id;
                const isATS = phase.layer === 'ATS';
                
                return (
                  <motion.div 
                    key={phase.id}
                    className={`relative rounded-lg p-4 cursor-pointer border transition-all ${
                      isActive 
                        ? isATS ? 'border-purple-500/50 bg-purple-500/10' : 'border-emerald-500/50 bg-emerald-500/10'
                        : 'border-slate-700 bg-slate-800/80 hover:border-slate-600'
                    }`}
                    onClick={() => setActivePhase(phase.id)}
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                  >
                    <div className="flex items-start gap-4">
                      <div className={`p-2 rounded-md ${isATS ? 'bg-purple-500/20 text-purple-400' : 'bg-emerald-500/20 text-emerald-400'}`}>
                        <phase.icon className="w-5 h-5" />
                      </div>
                      <div className="flex-1">
                        <div className="flex items-center justify-between mb-1">
                          <h3 className={`font-semibold text-sm ${isActive ? 'text-white' : 'text-slate-300'}`}>
                            {index + 1}. {phase.title}
                          </h3>
                          <span className={`text-[10px] font-bold px-2 py-0.5 rounded-full ${isATS ? 'bg-purple-500/20 text-purple-300' : 'bg-emerald-500/20 text-emerald-300'}`}>
                            {phase.layer}
                          </span>
                        </div>
                        <p className="text-xs text-slate-400 line-clamp-1">{phase.description}</p>
                      </div>
                    </div>
                  </motion.div>
                );
              })}

              {/* Connecting Arrows / Flow lines */}
              <div className="absolute left-[38px] top-[72px] bottom-[72px] w-0.5 bg-slate-700 -z-10" />
            </div>
            
            {/* Legend */}
            <div className="flex gap-4 pt-4 border-t border-slate-700/50 text-xs text-slate-400 mt-4">
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded-full bg-emerald-500/20 border border-emerald-500/50" />
                <span>ATC (Trust Layer)</span>
              </div>
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded-full bg-purple-500/20 border border-purple-500/50" />
                <span>ATS (Decision Layer)</span>
              </div>
            </div>
          </div>

          {/* Right: Detailed View */}
          <div className="bg-slate-800/50 p-6 rounded-xl border border-slate-700/50 overflow-auto">
            <AnimatePresence mode="wait">
              {PHASES.map((phase) => phase.id === activePhase && (
                <motion.div
                  key={phase.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  transition={{ duration: 0.2 }}
                  className="flex flex-col gap-6"
                >
                  <div>
                    <div className="flex items-center gap-3 mb-4">
                      <div className={`p-3 rounded-lg ${phase.layer === 'ATS' ? 'bg-purple-500/20 text-purple-400' : 'bg-emerald-500/20 text-emerald-400'}`}>
                        <phase.icon className="w-6 h-6" />
                      </div>
                      <div>
                        <h2 className="text-xl font-bold text-white">{phase.title}</h2>
                        <p className={`text-sm font-medium ${phase.layer === 'ATS' ? 'text-purple-400' : 'text-emerald-400'}`}>
                          {phase.layer === 'ATS' ? 'Intelligence & Orchestration Layer' : 'Deterministic Execution Substrate'}
                        </p>
                      </div>
                    </div>
                    
                    <p className="text-slate-300 leading-relaxed mb-6">
                      {phase.description}
                    </p>

                    <div className="mb-6">
                      <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider mb-3">Kernaufgaben</h3>
                      <ul className="space-y-2">
                        {phase.details.map((detail, idx) => (
                          <li key={idx} className="flex items-start gap-2 text-sm text-slate-300">
                            <CheckCircle className={`w-4 h-4 mt-0.5 shrink-0 ${phase.layer === 'ATS' ? 'text-purple-400' : 'text-emerald-400'}`} />
                            <span>{detail}</span>
                          </li>
                        ))}
                      </ul>
                    </div>

                    <div className="bg-slate-900/80 p-4 rounded-lg border border-slate-700/80">
                      <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider mb-2">Output Artifact</h3>
                      <div className="flex items-center gap-2 text-sm font-mono text-cyan-400">
                        <Database className="w-4 h-4" />
                        {phase.output}
                      </div>
                    </div>
                  </div>
                </motion.div>
              ))}
            </AnimatePresence>
          </div>
          
        </div>
      </div>
    </div>
  );
}
