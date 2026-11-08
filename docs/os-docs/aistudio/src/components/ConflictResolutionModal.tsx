import React, { useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { X, Check, ArrowRight, Github, FileText, FileJson, AlertTriangle, ShieldCheck } from 'lucide-react';
import { Light as SyntaxHighlighter } from 'react-syntax-highlighter';
import json from 'react-syntax-highlighter/dist/esm/languages/hljs/json';
import { atomOneDark } from 'react-syntax-highlighter/dist/esm/styles/hljs';

SyntaxHighlighter.registerLanguage('json', json);

interface NodeConflict {
  id: string;
  name: string;
  localState: string;
  githubState: string;
  notionState: string;
  resolved: boolean;
  selectedSource?: 'local' | 'github' | 'notion';
  risk: 'low' | 'high';
}

interface ConflictResolutionModalProps {
  isOpen: boolean;
  onClose: () => void;
  onResolve: (resolutions: Record<string, 'local' | 'github' | 'notion'>) => void;
}

const mockConflicts: NodeConflict[] = [
  {
    id: 'node-auth-gateway',
    name: 'AuthGateway Node',
    localState: '{\n  "version": "1.2.0",\n  "status": "deprecated",\n  "endpoints": [\n    "/login"\n  ]\n}',
    githubState: '{\n  "version": "1.2.0",\n  "status": "active",\n  "endpoints": [\n    "/login",\n    "/register",\n    "/oauth"\n  ]\n}',
    notionState: '{\n  "version": "1.1.5",\n  "status": "legacy",\n  "endpoints": [\n    "/login",\n    "/register"\n  ]\n}',
    resolved: false,
    risk: 'high'
  },
  {
    id: 'node-payment-processor',
    name: 'PaymentProcessor Node',
    localState: '{\n  "version": "2.0.1",\n  "status": "beta",\n  "providers": ["stripe", "paypal"]\n}',
    githubState: '{\n  "version": "2.0.0",\n  "status": "beta",\n  "providers": ["stripe"]\n}',
    notionState: '{\n  "version": "2.0.1",\n  "status": "active",\n  "providers": ["stripe", "paypal"]\n}',
    resolved: false,
    risk: 'low'
  },
  {
    id: 'node-data-lake',
    name: 'DataLake Orchestrator',
    localState: '{\n  "region": "eu-central",\n  "capacity": "100TB",\n  "sync": "batch"\n}',
    githubState: '{\n  "region": "eu-central",\n  "capacity": "100TB",\n  "sync": "realtime"\n}',
    notionState: '{\n  "region": "eu-west",\n  "capacity": "50TB",\n  "sync": "batch"\n}',
    resolved: false,
    risk: 'low'
  }
];

export function ConflictResolutionModal({ isOpen, onClose, onResolve }: ConflictResolutionModalProps) {
  const [conflicts, setConflicts] = useState<NodeConflict[]>(mockConflicts);

  const handleSelectSource = (id: string, source: 'local' | 'github' | 'notion') => {
    setConflicts(prev => 
      prev.map(c => 
        c.id === id ? { ...c, resolved: true, selectedSource: source } : c
      )
    );
  };

  const handleResolveAllLowRisk = () => {
    setConflicts(prev => 
      prev.map(c => 
        c.risk === 'low' && !c.resolved ? { ...c, resolved: true, selectedSource: 'github' } : c
      )
    );
  };

  const handleApplyAll = () => {
    const resolutions: Record<string, 'local' | 'github' | 'notion'> = {};
    conflicts.forEach(c => {
      if (c.selectedSource) {
        resolutions[c.id] = c.selectedSource;
      }
    });
    onResolve(resolutions);
    onClose();
  };

  const remaining = conflicts.filter(c => !c.resolved).length;

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
      <motion.div 
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.95 }}
        className="w-full max-w-5xl max-h-[90vh] bg-[#0b0f19] border border-red-500/30 rounded-xl shadow-2xl flex flex-col overflow-hidden"
      >
        <div className="flex items-center justify-between p-6 border-b border-white/5 bg-red-500/10">
          <div className="flex items-center gap-3">
            <AlertTriangle className="w-6 h-6 text-red-500" />
            <div>
              <h2 className="text-xl font-bold text-white">State Conflict Resolution</h2>
              <p className="text-sm text-red-400">Please select the source of truth for conflicting architecture nodes. Check differences below.</p>
            </div>
          </div>
          <div className="flex gap-4 items-center">
            <button 
              onClick={handleResolveAllLowRisk}
              className="flex items-center gap-2 px-4 py-2 bg-slate-800 text-slate-300 hover:text-white transition-colors rounded-lg text-sm font-mono border border-slate-700 hover:bg-slate-700"
            >
              <ShieldCheck className="w-4 h-4" />
              Resolve All Low Risk
            </button>
            <button 
              onClick={onClose}
              className="p-2 text-slate-400 hover:text-white transition-colors rounded-lg hover:bg-white/5"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto p-6 space-y-8 custom-scrollbar">
          {conflicts.map(conflict => (
            <div key={conflict.id} className="flex flex-col gap-4 p-4 border border-white/10 rounded-xl bg-black/20">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <FileJson className="w-5 h-5 text-slate-400" />
                  <h3 className="text-lg font-mono text-white">{conflict.name}</h3>
                  <span className={`px-2 py-0.5 rounded-md text-xs font-mono ml-3 ${
                    conflict.risk === 'high' ? 'bg-red-500/20 text-red-400 border border-red-500/30' : 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30'
                  }`}>
                    {conflict.risk === 'high' ? 'High Risk' : 'Low Risk'}
                  </span>
                </div>
                {conflict.resolved && (
                  <span className="flex items-center gap-1 text-xs font-mono text-emerald-400 bg-emerald-400/10 px-2 py-1 rounded-md">
                    <Check className="w-3 h-3" />
                    Resolved: Using {conflict.selectedSource}
                  </span>
                )}
              </div>

              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 relative">
                {/* Local Option */}
                <div 
                  className={`border rounded-lg p-4 cursor-pointer transition-all flex flex-col ${
                    conflict.selectedSource === 'local' 
                      ? 'border-indigo-500/50 bg-indigo-500/10' 
                      : 'border-white/10 bg-black/40 hover:border-slate-500/50'
                  }`}
                  onClick={() => handleSelectSource(conflict.id, 'local')}
                >
                  <div className="flex items-center gap-2 mb-3 text-slate-300">
                    <FileJson className="w-4 h-4" />
                    <span className="font-semibold text-sm">Local State</span>
                  </div>
                  <div className="text-xs font-mono rounded-md overflow-hidden bg-[#282c34] border border-white/5 flex-1">
                    <SyntaxHighlighter language="json" style={atomOneDark} customStyle={{ padding: '0.75rem', margin: 0, backgroundColor: 'transparent', height: '100%' }}>
                      {conflict.localState}
                    </SyntaxHighlighter>
                  </div>
                  <button 
                    className={`mt-4 w-full py-2 rounded-md font-mono text-sm transition-colors ${
                      conflict.selectedSource === 'local' 
                        ? 'bg-indigo-500 text-white' 
                        : 'bg-white/5 text-slate-400 hover:bg-white/10'
                    }`}
                  >
                    {conflict.selectedSource === 'local' ? 'Selected' : 'Use Local State'}
                  </button>
                </div>

                {/* GitHub Option */}
                <div 
                  className={`border rounded-lg p-4 cursor-pointer transition-all flex flex-col ${
                    conflict.selectedSource === 'github' 
                      ? 'border-emerald-500/50 bg-emerald-500/10' 
                      : 'border-white/10 bg-black/40 hover:border-slate-500/50'
                  }`}
                  onClick={() => handleSelectSource(conflict.id, 'github')}
                >
                  <div className="flex items-center gap-2 mb-3 text-slate-300">
                    <Github className="w-4 h-4" />
                    <span className="font-semibold text-sm">GitHub Repository</span>
                  </div>
                  <div className="text-xs font-mono rounded-md overflow-hidden bg-[#282c34] border border-white/5 flex-1">
                    <SyntaxHighlighter language="json" style={atomOneDark} customStyle={{ padding: '0.75rem', margin: 0, backgroundColor: 'transparent', height: '100%' }}>
                      {conflict.githubState}
                    </SyntaxHighlighter>
                  </div>
                  <button 
                    className={`mt-4 w-full py-2 rounded-md font-mono text-sm transition-colors ${
                      conflict.selectedSource === 'github' 
                        ? 'bg-emerald-500 text-white' 
                        : 'bg-white/5 text-slate-400 hover:bg-white/10'
                    }`}
                  >
                    {conflict.selectedSource === 'github' ? 'Selected' : 'Use GitHub Source'}
                  </button>
                </div>

                {/* Notion Option */}
                <div 
                  className={`border rounded-lg p-4 cursor-pointer transition-all flex flex-col ${
                    conflict.selectedSource === 'notion' 
                      ? 'border-blue-500/50 bg-blue-500/10' 
                      : 'border-white/10 bg-black/40 hover:border-slate-500/50'
                  }`}
                  onClick={() => handleSelectSource(conflict.id, 'notion')}
                >
                  <div className="flex items-center gap-2 mb-3 text-slate-300">
                    <FileText className="w-4 h-4" />
                    <span className="font-semibold text-sm">Notion Workspace</span>
                  </div>
                  <div className="text-xs font-mono rounded-md overflow-hidden bg-[#282c34] border border-white/5 flex-1">
                    <SyntaxHighlighter language="json" style={atomOneDark} customStyle={{ padding: '0.75rem', margin: 0, backgroundColor: 'transparent', height: '100%' }}>
                      {conflict.notionState}
                    </SyntaxHighlighter>
                  </div>
                  <button 
                    className={`mt-4 w-full py-2 rounded-md font-mono text-sm transition-colors ${
                      conflict.selectedSource === 'notion' 
                        ? 'bg-blue-500 text-white' 
                        : 'bg-white/5 text-slate-400 hover:bg-white/10'
                    }`}
                  >
                    {conflict.selectedSource === 'notion' ? 'Selected' : 'Use Notion Source'}
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>

        <div className="p-6 border-t border-white/5 bg-black/40 flex items-center justify-between">
          <div className="text-sm font-mono text-slate-400">
            {remaining > 0 ? (
              <span className="text-red-400">{remaining} conflicts remaining</span>
            ) : (
              <span className="text-emerald-400">All conflicts resolved</span>
            )}
          </div>
          <button 
            onClick={handleApplyAll}
            disabled={remaining > 0}
            className="flex items-center gap-2 px-6 py-2.5 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed font-medium"
          >
            Apply Resolutions
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>
      </motion.div>
    </div>
  );
}
