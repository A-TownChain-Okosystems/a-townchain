import React, { useState, useMemo } from 'react';
import {
  Layers,
  Shield,
  Cpu,
  Boxes,
  Lock,
  ArrowRight,
  ChevronRight,
  CheckCircle2,
  Download,
  Copy,
  Check,
  Search,
  Sparkles,
  Workflow,
  Server,
  Activity,
  KeyRound,
  Eye,
  Globe,
  Radio,
  Sliders,
  Zap,
  Terminal,
  FileCode,
  Compass,
  AlertTriangle,
  Scale,
  ShieldAlert,
  ShieldCheck,
  GitFork,
  Binary,
  Database,
  CpuIcon
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import {
  V3_DOMAINS,
  V3_CONTROL_PLANES,
  V3_CONTRACTS,
  V3_SYSTEM_BOUNDARIES,
  V3_UNIFIED_MODELS,
  V3_STANDARDS_FAMILY,
  GLOBUS_V3_MARKDOWN_SPEC,
  DomainV3,
  ControlPlaneV3,
  ContractV3,
  SystemBoundaryV3,
  UnifiedModelV3,
  StandardSpecV3
} from '../data/globusV3Architecture';

interface GlobusV3ArchitecturePanelProps {
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export function GlobusV3ArchitecturePanel({
  onSaveWorkspaceFile,
  onShowToast,
}: GlobusV3ArchitecturePanelProps) {
  // Navigation tabs for v3
  const [activeTab, setActiveTab] = useState<
    'domains' | 'planes' | 'contracts' | 'boundaries' | 'models' | 'chain' | 'standards' | 'export'
  >('domains');

  // Selected domain in D1-D5
  const [selectedDomainCode, setSelectedDomainCode] = useState<string>('D1');

  // Selected contract in C1-C5
  const [selectedContractCode, setSelectedContractCode] = useState<string>('C4');

  // Active step in contract execution flow
  const [activeFlowStep, setActiveFlowStep] = useState<number>(0);

  // Selected system boundary
  const [selectedBoundaryId, setSelectedBoundaryId] = useState<string>('kernel-boundary');

  // Selected unified model
  const [selectedModelId, setSelectedModelId] = useState<string>('gos-res-001');

  // Filter for 19 Standards
  const [standardsFilter, setStandardsFilter] = useState<string>('All');
  const [standardsSearch, setStandardsSearch] = useState<string>('');

  // Copy state
  const [copied, setCopied] = useState<boolean>(false);

  // Current domain
  const currentDomain = useMemo(() => {
    return V3_DOMAINS.find(d => d.code === selectedDomainCode) || V3_DOMAINS[0];
  }, [selectedDomainCode]);

  // Current contract
  const currentContract = useMemo(() => {
    return V3_CONTRACTS.find(c => c.code === selectedContractCode) || V3_CONTRACTS[0];
  }, [selectedContractCode]);

  // Current boundary
  const currentBoundary = useMemo(() => {
    return V3_SYSTEM_BOUNDARIES.find(b => b.id === selectedBoundaryId) || V3_SYSTEM_BOUNDARIES[0];
  }, [selectedBoundaryId]);

  // Current unified model
  const currentModel = useMemo(() => {
    return V3_UNIFIED_MODELS.find(m => m.id === selectedModelId) || V3_UNIFIED_MODELS[0];
  }, [selectedModelId]);

  // Filtered standards
  const filteredStandards = useMemo(() => {
    return V3_STANDARDS_FAMILY.filter(spec => {
      const matchCat = standardsFilter === 'All' || spec.category === standardsFilter;
      const matchQuery =
        standardsSearch === '' ||
        spec.code.toLowerCase().includes(standardsSearch.toLowerCase()) ||
        spec.title.toLowerCase().includes(standardsSearch.toLowerCase()) ||
        spec.description.toLowerCase().includes(standardsSearch.toLowerCase());
      return matchCat && matchQuery;
    });
  }, [standardsFilter, standardsSearch]);

  const handleCopySpec = () => {
    navigator.clipboard.writeText(GLOBUS_V3_MARKDOWN_SPEC);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
    if (onShowToast) onShowToast('Globus OS v3 Referenzspezifikation kopiert!', 'info');
  };

  const handleSaveToWorkspace = () => {
    if (!onSaveWorkspaceFile) return;
    onSaveWorkspaceFile('GLOBUS_SHIVACORE_V3_ARCHITECTURE.md', GLOBUS_V3_MARKDOWN_SPEC);
    if (onShowToast) {
      onShowToast("'GLOBUS_SHIVACORE_V3_ARCHITECTURE.md' erfolgreich im Projekt-Root gespeichert!", 'info');
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#0d1117] text-slate-200 overflow-hidden select-none">
      {/* Top Banner: Architecture v3 Header */}
      <div className="flex-none px-6 py-4 border-b border-slate-800/80 bg-gradient-to-r from-[#0d1117] via-[#111827] to-[#0f172a] shadow-sm">
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-indigo-500 to-cyan-500 flex items-center justify-center text-white shadow-lg shadow-indigo-500/20 ring-1 ring-white/10">
              <Workflow className="w-5 h-5" />
            </div>
            <div>
              <div className="flex items-center gap-2">
                <h1 className="text-lg font-bold text-white tracking-wide">
                  Globus OS / ShivaCore — Reference Architecture v3
                </h1>
                <span className="px-2 py-0.5 rounded-full text-[10px] font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 uppercase tracking-wider">
                  Formal Platform Architecture
                </span>
                <span className="px-2 py-0.5 rounded-full text-[10px] font-bold bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 uppercase tracking-wider">
                  GOS-ARCH-001 v3.0
                </span>
              </div>
              <p className="text-xs text-slate-400 mt-0.5">
                5 Vertikale Domains • 7 Horizontale Control Planes • Verträge C1–C5 • Duale ABI (Host vs. Syscall) • 19 Normierte Standards
              </p>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={handleCopySpec}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 transition cursor-pointer"
              title="Markdown Spezifikation in Zwischenablage kopieren"
            >
              {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
              <span>{copied ? 'Kopiert' : 'Copy Spec'}</span>
            </button>

            {onSaveWorkspaceFile && (
              <button
                onClick={handleSaveToWorkspace}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white shadow-md shadow-indigo-600/20 transition cursor-pointer"
                title="Speichert GLOBUS_SHIVACORE_V3_ARCHITECTURE.md im Workspace"
              >
                <Download className="w-3.5 h-3.5" />
                <span>Save to Workspace (.md)</span>
              </button>
            )}
          </div>
        </div>

        {/* Sub-Navigation Tabs */}
        <div className="flex items-center gap-1 mt-4 overflow-x-auto no-scrollbar border-t border-slate-800/60 pt-3">
          {[
            { id: 'domains', label: '1. Die 5 Domains (D1-D5)', icon: Layers, badge: '5 Domains' },
            { id: 'planes', label: '2. Die 7 Control Planes', icon: Sliders, badge: 'Cross-Cutting' },
            { id: 'contracts', label: '3. Verträge (C1 - C5)', icon: GitFork, badge: 'Contracts' },
            { id: 'boundaries', label: '4. System Boundaries', icon: Shield, badge: 'Isolation' },
            { id: 'models', label: '5. Globale Modelle', icon: Boxes, badge: 'Res/Id/Event' },
            { id: 'chain', label: '6. End-to-End Kette', icon: Workflow, badge: 'E2E Flow' },
            { id: 'standards', label: '7. Standards-Normenfamilie', icon: FileCode, badge: '19 Specs' },
            { id: 'export', label: '8. Volltext-Spezifikation', icon: Terminal, badge: 'Markdown' },
          ].map(tab => {
            const Icon = tab.icon;
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-medium transition cursor-pointer whitespace-nowrap ${
                  isActive
                    ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 font-semibold shadow-inner'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50 border border-transparent'
                }`}
              >
                <Icon className={`w-3.5 h-3.5 ${isActive ? 'text-indigo-400' : 'text-slate-400'}`} />
                <span>{tab.label}</span>
                <span className={`text-[10px] px-1.5 py-0.2 rounded ${isActive ? 'bg-indigo-500/30 text-indigo-200' : 'bg-slate-800 text-slate-400'}`}>
                  {tab.badge}
                </span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto p-6">
        {/* ========================================================
            TAB 1: DIE 5 VERTIKALEN DOMAINS (D1 - D5)
        ======================================================== */}
        {activeTab === 'domains' && (
          <div className="space-y-6">
            {/* Header Callout */}
            <div className="p-4 rounded-xl bg-slate-900/80 border border-slate-800 text-xs text-slate-300 flex items-start gap-3">
              <Layers className="w-5 h-5 text-cyan-400 flex-none mt-0.5" />
              <div>
                <span className="font-semibold text-white">Architektur-Klarheit:</span> Die 5 vertikalen Domains stellen die primären, unüberschreitbaren Schichtengrenzen von Globus OS v3 dar. Jede Domain besitzt eine klar definierte Inbound- und Outbound-Schnittstelle. Es gibt keine unautorisierten Schichten-Sprünge (z.B. UI direkt zu Kernel).
              </div>
            </div>

            {/* Visual Domain Architecture Diagram */}
            <div className="grid grid-cols-1 md:grid-cols-5 gap-3">
              {V3_DOMAINS.map(domain => {
                const isSelected = selectedDomainCode === domain.code;
                return (
                  <button
                    key={domain.code}
                    onClick={() => setSelectedDomainCode(domain.code)}
                    className={`text-left p-4 rounded-xl border transition-all cursor-pointer relative overflow-hidden flex flex-col justify-between ${
                      isSelected
                        ? 'bg-gradient-to-b from-slate-800/90 to-slate-900 border-indigo-500/60 shadow-lg shadow-indigo-500/10 ring-1 ring-indigo-500/30'
                        : 'bg-slate-900/40 hover:bg-slate-900/80 border-slate-800/80 text-slate-400 hover:text-slate-200'
                    }`}
                  >
                    <div className="flex items-center justify-between mb-2">
                      <span className="font-mono text-sm font-bold text-white px-2 py-0.5 rounded bg-slate-800 border border-slate-700">
                        {domain.code}
                      </span>
                      <span className="text-[10px] uppercase font-bold tracking-wider text-cyan-400">
                        {domain.badge}
                      </span>
                    </div>
                    <div className="font-semibold text-white text-sm mt-1">{domain.name}</div>
                    <div className="text-[11px] text-slate-400 mt-1 line-clamp-2 leading-relaxed">
                      {domain.headline}
                    </div>
                    <div className="mt-3 pt-2 border-t border-slate-800/60 flex items-center justify-between text-[11px] text-slate-400">
                      <span>{domain.components.length} Komponenten</span>
                      <ChevronRight className={`w-3.5 h-3.5 ${isSelected ? 'text-indigo-400' : 'text-slate-600'}`} />
                    </div>
                  </button>
                );
              })}
            </div>

            {/* Selected Domain Deep Dive */}
            <div className="p-6 rounded-2xl bg-slate-900/70 border border-slate-800/90 space-y-5">
              <div className="flex flex-col md:flex-row md:items-center justify-between gap-3 border-b border-slate-800 pb-4">
                <div>
                  <div className="flex items-center gap-2">
                    <span className="px-2.5 py-1 rounded-md text-xs font-mono font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/40">
                      {currentDomain.code}
                    </span>
                    <h2 className="text-lg font-bold text-white">{currentDomain.name}</h2>
                    <span className="text-xs text-slate-400">• {currentDomain.headline}</span>
                  </div>
                  <p className="text-xs text-slate-300 mt-1.5 leading-relaxed max-w-4xl">
                    {currentDomain.description}
                  </p>
                </div>
              </div>

              {/* Boundary Rule Warning */}
              <div className="p-3.5 rounded-lg bg-amber-500/10 border border-amber-500/30 text-amber-200/90 text-xs flex items-start gap-2.5">
                <AlertTriangle className="w-4 h-4 text-amber-400 flex-none mt-0.5" />
                <div>
                  <span className="font-semibold text-amber-300">Strikte Schranken-Regel (Boundary Invariant): </span>
                  {currentDomain.boundaryRule}
                </div>
              </div>

              {/* Components Grid */}
              <div>
                <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400 mb-3 flex items-center gap-1.5">
                  <Boxes className="w-4 h-4 text-indigo-400" />
                  Zugehörige Kern-Komponenten in {currentDomain.code}
                </h3>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-2.5">
                  {currentDomain.components.map((comp, idx) => (
                    <div
                      key={idx}
                      className="p-3 rounded-lg bg-slate-950/60 border border-slate-800/80 flex items-start gap-2 text-xs"
                    >
                      <CheckCircle2 className="w-4 h-4 text-emerald-400 flex-none mt-0.5" />
                      <span className="text-slate-200 font-medium">{comp}</span>
                    </div>
                  ))}
                </div>
              </div>

              {/* Formal Contracts In/Out */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
                <div className="p-3.5 rounded-xl bg-slate-950/50 border border-slate-800">
                  <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1 flex items-center gap-1.5">
                    <ArrowRight className="w-3.5 h-3.5 text-cyan-400 rotate-180" />
                    Inbound Vertrag (Eingehend)
                  </div>
                  <div className="text-xs text-cyan-300 font-mono mt-1">
                    {currentDomain.inboundContract}
                  </div>
                </div>

                <div className="p-3.5 rounded-xl bg-slate-950/50 border border-slate-800">
                  <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1 flex items-center gap-1.5">
                    <ArrowRight className="w-3.5 h-3.5 text-indigo-400" />
                    Outbound Vertrag (Ausgehend)
                  </div>
                  <div className="text-xs text-indigo-300 font-mono mt-1">
                    {currentDomain.outboundContract}
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* ========================================================
            TAB 2: DIE 7 HORIZONTALEN CONTROL PLANES
        ======================================================== */}
        {activeTab === 'planes' && (
          <div className="space-y-6">
            {/* The Axiom Callout */}
            <div className="p-4 rounded-xl bg-gradient-to-r from-red-500/10 via-purple-500/10 to-indigo-500/10 border border-purple-500/30 text-xs text-slate-300 flex items-start gap-3">
              <ShieldAlert className="w-5 h-5 text-purple-400 flex-none mt-0.5" />
              <div>
                <div className="font-bold text-white text-sm mb-1">
                  Fundamentales Plattform-Axiom:
                </div>
                <div className="italic text-slate-200 leading-relaxed">
                  "Security, Identity, Policy und Audit sind keine Services. Sie sind horizontale Plattformfunktionen."
                </div>
                <div className="text-slate-400 mt-1">
                  Services (wie ein Token-Dienst oder ein Marketplace) können instanziiert oder ausgetauscht werden. Die 7 Control Planes sind systemweit invariante Querschnitts-Ebenen, die jeden Aufruf über alle 5 Domains hinweg überwachen und durchsetzen.
                </div>
              </div>
            </div>

            {/* 7 Planes List */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {V3_CONTROL_PLANES.map(plane => (
                <div
                  key={plane.id}
                  className="p-5 rounded-xl bg-slate-900/60 border border-slate-800 hover:border-slate-700 transition flex flex-col justify-between"
                >
                  <div>
                    <div className="flex items-center justify-between mb-2">
                      <h3 className="font-bold text-white text-sm">{plane.name}</h3>
                      <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-indigo-500/15 text-indigo-300 border border-indigo-500/30">
                        {plane.nature}
                      </span>
                    </div>
                    <p className="text-xs text-slate-400 mb-3">{plane.tagline}</p>

                    <div className="space-y-1.5 my-3 border-t border-slate-800/80 pt-3">
                      <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider">
                        Kern-Aufgaben:
                      </div>
                      {plane.coreFunctions.map((func, fIdx) => (
                        <div key={fIdx} className="text-xs text-slate-300 flex items-start gap-1.5">
                          <span className="text-indigo-400 mt-0.5">•</span>
                          <span>{func}</span>
                        </div>
                      ))}
                    </div>
                  </div>

                  <div className="mt-4 pt-3 border-t border-slate-800/80 space-y-2 text-[11px]">
                    <div>
                      <span className="text-slate-400 font-semibold">Enforcement: </span>
                      <span className="text-cyan-300 font-mono">{plane.enforcementMechanism}</span>
                    </div>
                    <div>
                      <span className="text-slate-400 font-semibold">Audit-Umfang: </span>
                      <span className="text-slate-300">{plane.auditScope}</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* ========================================================
            TAB 3: DIE 5 ARCHITEKTURVERTRÄGE (C1 - C5)
        ======================================================== */}
        {activeTab === 'contracts' && (
          <div className="space-y-6">
            <div className="p-4 rounded-xl bg-slate-900/80 border border-slate-800 text-xs text-slate-300 flex items-start gap-3">
              <GitFork className="w-5 h-5 text-indigo-400 flex-none mt-0.5" />
              <div>
                <span className="font-semibold text-white">Formale Verträge zwischen den Domains:</span> Ein System ist nur so stabil wie seine Schnittstellen-Spezifikation. Die Verträge C1 bis C5 definieren verbindlich Payload-Schemas, Authentifizierungs-Nachweise, Fehlerbehandlung und Fehlermodi.
              </div>
            </div>

            {/* Contract Selector Bar */}
            <div className="flex flex-wrap gap-2">
              {V3_CONTRACTS.map(c => {
                const isSel = selectedContractCode === c.code;
                return (
                  <button
                    key={c.code}
                    onClick={() => {
                      setSelectedContractCode(c.code);
                      setActiveFlowStep(0);
                    }}
                    className={`px-4 py-2.5 rounded-xl text-xs font-semibold transition cursor-pointer border flex items-center gap-2 ${
                      isSel
                        ? 'bg-indigo-600 text-white border-indigo-400 shadow-md shadow-indigo-600/20'
                        : 'bg-slate-900 text-slate-300 border-slate-800 hover:bg-slate-800'
                    }`}
                  >
                    <span className="font-mono px-1.5 py-0.5 rounded bg-black/20 text-[11px]">{c.code}</span>
                    <span>{c.title.split('—')[1]?.trim() || c.title}</span>
                  </button>
                );
              })}
            </div>

            {/* Selected Contract Inspection */}
            <div className="p-6 rounded-2xl bg-slate-900/70 border border-slate-800/90 space-y-6">
              <div className="flex flex-col md:flex-row md:items-center justify-between gap-2 border-b border-slate-800 pb-4">
                <div>
                  <div className="flex items-center gap-2">
                    <span className="px-2.5 py-1 rounded font-mono font-bold text-xs bg-indigo-500/20 text-indigo-300 border border-indigo-500/40">
                      {currentContract.code}
                    </span>
                    <h2 className="text-lg font-bold text-white">{currentContract.title}</h2>
                  </div>
                  <p className="text-xs text-slate-300 mt-1">{currentContract.description}</p>
                </div>
                <div className="text-xs text-slate-400 font-mono bg-slate-950 px-3 py-1.5 rounded-lg border border-slate-800">
                  {currentContract.sourceLayer} <span className="text-indigo-400">→</span> {currentContract.targetLayer}
                </div>
              </div>

              {/* Special Emphasis for C4 and C5 */}
              {currentContract.code === 'C4' && (
                <div className="p-4 rounded-xl bg-amber-500/10 border border-amber-500/30 text-amber-200 text-xs space-y-2">
                  <div className="font-bold text-amber-300 flex items-center gap-2">
                    <AlertTriangle className="w-4 h-4" />
                    Wesentliche Architektur-Grenze: Duale ABI-Trennung (Host ABI vs. Syscall ABI)
                  </div>
                  <p className="leading-relaxed">
                    Smart Contracts in der ATC VM, WASM-Plugins und autonome KI-Agenten kennen <strong>keine</strong> direkten Kernel-Syscalls. Sie operieren isoliert gegen die <strong>Host ABI (GOS-HABI-001)</strong>. Erst das Host Interface prüft die Berechtigung und ruft gegebenenfalls einen echten Syscall über die <strong>Syscall ABI (GOS-ABI-001)</strong> auf.
                  </p>
                </div>
              )}

              {currentContract.code === 'C5' && (
                <div className="p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-200 text-xs space-y-2">
                  <div className="font-bold text-emerald-300 flex items-center gap-2">
                    <Shield className="w-4 h-4" />
                    Verbindliche 12-Stufen ShivaCore Security Execution Pipeline
                  </div>
                  <p className="leading-relaxed font-mono text-[11px] text-emerald-300/90">
                    Request → ABI → Validation → Identity → Namespace → Handle → Capability → Rights → Policy → Quota → Resource → Audit → Result
                  </p>
                </div>
              )}

              {/* Step-by-Step Flow Simulation */}
              <div>
                <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400 mb-3 flex items-center gap-2">
                  <Workflow className="w-4 h-4 text-indigo-400" />
                  Ausführungs-Sequenz ({currentContract.flowSteps.length} Schritte)
                </h3>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2.5">
                  {currentContract.flowSteps.map((step, idx) => (
                    <div
                      key={idx}
                      onClick={() => setActiveFlowStep(idx)}
                      className={`p-3 rounded-lg border text-xs cursor-pointer transition-all ${
                        activeFlowStep === idx
                          ? 'bg-indigo-950/60 border-indigo-500 text-white shadow-sm ring-1 ring-indigo-500/40'
                          : 'bg-slate-950/50 border-slate-800 text-slate-300 hover:border-slate-700'
                      }`}
                    >
                      <div className="flex items-center justify-between text-[10px] text-slate-400 font-mono mb-1">
                        <span>Schritt {idx + 1}</span>
                        {activeFlowStep === idx && <span className="text-indigo-400 font-bold">AKTIV</span>}
                      </div>
                      <div className="leading-relaxed">{step}</div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Clauses and Failure Modes */}
              <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 pt-2">
                <div className="lg:col-span-2 space-y-3">
                  <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400 flex items-center gap-1.5">
                    <CheckCircle2 className="w-4 h-4 text-cyan-400" />
                    Verbindliche Vertragsklauseln
                  </h3>
                  <div className="space-y-2">
                    {currentContract.contractClauses.map((clause, idx) => (
                      <div key={idx} className="p-3 rounded-lg bg-slate-950/60 border border-slate-800">
                        <div className="font-semibold text-white text-xs mb-1">{clause.name}</div>
                        <div className="text-xs text-slate-300 leading-relaxed">{clause.description}</div>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="space-y-3">
                  <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400 flex items-center gap-1.5">
                    <AlertTriangle className="w-4 h-4 text-amber-400" />
                    Definierte Fehlermodi (Fail-Closed)
                  </h3>
                  <div className="p-3 rounded-lg bg-slate-950/60 border border-slate-800 space-y-2">
                    {currentContract.failureModes.map((fm, idx) => (
                      <div
                        key={idx}
                        className="px-2.5 py-1.5 rounded bg-red-500/10 border border-red-500/30 text-red-300 font-mono text-[11px] flex items-center gap-2"
                      >
                        <span className="w-1.5 h-1.5 rounded-full bg-red-400" />
                        <span>{fm}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* ========================================================
            TAB 4: FORMALE SYSTEM BOUNDARIES
        ======================================================== */}
        {activeTab === 'boundaries' && (
          <div className="space-y-6">
            <div className="p-4 rounded-xl bg-slate-900/80 border border-slate-800 text-xs text-slate-300 flex items-start gap-3">
              <Shield className="w-5 h-5 text-emerald-400 flex-none mt-0.5" />
              <div>
                <span className="font-semibold text-white">Präzise Schranken für Kernsubsysteme:</span> Eine saubere Systemarchitektur definiert nicht nur, was ein Subsystem tut, sondern vor allem auch, was es <strong>strikt nicht</strong> tun darf (Minimal Trusted Computing Base).
              </div>
            </div>

            {/* Boundary Selector Tabs */}
            <div className="grid grid-cols-2 md:grid-cols-5 gap-2">
              {V3_SYSTEM_BOUNDARIES.map(b => {
                const isSel = selectedBoundaryId === b.id;
                return (
                  <button
                    key={b.id}
                    onClick={() => setSelectedBoundaryId(b.id)}
                    className={`p-3 rounded-xl border text-left transition cursor-pointer ${
                      isSel
                        ? 'bg-slate-800 border-indigo-500 shadow-md ring-1 ring-indigo-500/30'
                        : 'bg-slate-900/60 border-slate-800 text-slate-400 hover:text-slate-200'
                    }`}
                  >
                    <div className="font-bold text-white text-xs">{b.title.split('(')[0]}</div>
                    <div className="text-[10px] text-slate-400 mt-1 line-clamp-1">{b.subtitle}</div>
                  </button>
                );
              })}
            </div>

            {/* Selected Boundary Deep Dive */}
            <div className="p-6 rounded-2xl bg-slate-900/70 border border-slate-800/90 space-y-6">
              <div className="border-b border-slate-800 pb-4">
                <h2 className="text-lg font-bold text-white">{currentBoundary.title}</h2>
                <p className="text-xs text-slate-400 mt-1">{currentBoundary.subtitle}</p>
                <div className="mt-3 p-3 rounded-lg bg-indigo-500/10 border border-indigo-500/30 text-xs text-indigo-200">
                  <span className="font-semibold text-indigo-300">Architektonische Rationale: </span>
                  {currentBoundary.rationale}
                </div>
              </div>

              {/* Responsibilities Grid: Included vs. Strictly Excluded */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Inside the Boundary */}
                <div className="p-4 rounded-xl bg-emerald-950/20 border border-emerald-500/30 space-y-3">
                  <div className="font-bold text-emerald-300 text-xs flex items-center gap-1.5 uppercase tracking-wider">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                    Verbindlich INNERHALB der Schranke:
                  </div>
                  <div className="space-y-2">
                    {currentBoundary.internalResponsibilities.map((item, idx) => (
                      <div key={idx} className="text-xs text-slate-200 flex items-start gap-2">
                        <span className="text-emerald-400 mt-0.5">•</span>
                        <span className="leading-relaxed">{item}</span>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Strictly Excluded from Boundary */}
                <div className="p-4 rounded-xl bg-red-950/20 border border-red-500/30 space-y-3">
                  <div className="font-bold text-red-300 text-xs flex items-center gap-1.5 uppercase tracking-wider">
                    <AlertTriangle className="w-4 h-4 text-red-400" />
                    STRIKT AUSGESCHLOSSEN (Verboten in dieser Schicht):
                  </div>
                  <div className="space-y-2">
                    {currentBoundary.strictlyExcludedResponsibilities.map((item, idx) => (
                      <div key={idx} className="text-xs text-red-200/90 flex items-start gap-2">
                        <span className="text-red-400 mt-0.5 font-bold">✕</span>
                        <span className="leading-relaxed">{item}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              {/* Formal Boundary Pipeline */}
              <div className="pt-2">
                <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400 mb-3 flex items-center gap-1.5">
                  <Workflow className="w-4 h-4 text-cyan-400" />
                  Formale Ausführungskette (Pipeline)
                </h3>
                <div className="flex flex-wrap items-center gap-2">
                  {currentBoundary.formalPipeline.map((step, idx) => (
                    <React.Fragment key={idx}>
                      <div className="px-3 py-1.5 rounded-lg bg-slate-950 border border-slate-800 text-xs text-slate-200 font-mono">
                        {step}
                      </div>
                      {idx < currentBoundary.formalPipeline.length - 1 && (
                        <ArrowRight className="w-3.5 h-3.5 text-slate-600" />
                      )}
                    </React.Fragment>
                  ))}
                </div>
              </div>
            </div>
          </div>
        )}

        {/* ========================================================
            TAB 5: EINHEITLICHE GLOBALE MODELLE (RESOURCE / IDENTITY / EVENT)
        ======================================================== */}
        {activeTab === 'models' && (
          <div className="space-y-6">
            <div className="p-4 rounded-xl bg-slate-900/80 border border-slate-800 text-xs text-slate-300 flex items-start gap-3">
              <Boxes className="w-5 h-5 text-indigo-400 flex-none mt-0.5" />
              <div>
                <span className="font-semibold text-white">Systemübergreifende Invarianz:</span> Statt für jede Domain (OS, Container, VM, Agent, Blockchain) getrennte Datenstrukturen zu erfinden, vereinheitlicht Globus OS v3 Ressourcen, Identitäten und Audit-Events auf ein globales Typensystem.
              </div>
            </div>

            {/* Model Selector */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              {V3_UNIFIED_MODELS.map(model => {
                const isSel = selectedModelId === model.id;
                return (
                  <button
                    key={model.id}
                    onClick={() => setSelectedModelId(model.id)}
                    className={`p-4 rounded-xl border text-left transition cursor-pointer ${
                      isSel
                        ? 'bg-slate-800 border-indigo-500 shadow-md ring-1 ring-indigo-500/30'
                        : 'bg-slate-900/60 border-slate-800 text-slate-400 hover:text-slate-200'
                    }`}
                  >
                    <div className="flex items-center justify-between mb-1.5">
                      <span className="px-2 py-0.5 rounded font-mono text-[11px] font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/40">
                        {model.standardCode}
                      </span>
                    </div>
                    <div className="font-bold text-white text-sm">{model.title}</div>
                    <div className="text-xs text-slate-400 mt-1 line-clamp-2">{model.concept}</div>
                  </button>
                );
              })}
            </div>

            {/* Selected Model Schema Table */}
            <div className="p-6 rounded-2xl bg-slate-900/70 border border-slate-800/90 space-y-6">
              <div className="flex flex-col md:flex-row md:items-center justify-between gap-2 border-b border-slate-800 pb-4">
                <div>
                  <div className="flex items-center gap-2">
                    <span className="px-2.5 py-1 rounded font-mono font-bold text-xs bg-indigo-500/20 text-indigo-300 border border-indigo-500/40">
                      {currentModel.standardCode}
                    </span>
                    <h2 className="text-lg font-bold text-white">{currentModel.title}</h2>
                  </div>
                  <p className="text-xs text-slate-300 mt-1">{currentModel.concept}</p>
                </div>
              </div>

              {/* Guarantees */}
              <div className="p-4 rounded-xl bg-slate-950/60 border border-slate-800 space-y-2">
                <div className="text-xs font-bold uppercase tracking-wider text-slate-400 flex items-center gap-1.5">
                  <ShieldCheck className="w-4 h-4 text-emerald-400" />
                  Verbindliche Plattform-Garantien
                </div>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-2 pt-1">
                  {currentModel.guarantees.map((g, idx) => (
                    <div key={idx} className="text-xs text-slate-300 flex items-start gap-1.5">
                      <span className="text-emerald-400">•</span>
                      <span>{g}</span>
                    </div>
                  ))}
                </div>
              </div>

              {/* Fields Table */}
              <div>
                <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400 mb-3 flex items-center gap-1.5">
                  <Terminal className="w-4 h-4 text-indigo-400" />
                  Formale Schema-Definition
                </h3>
                <div className="overflow-x-auto rounded-xl border border-slate-800">
                  <table className="w-full text-left text-xs border-collapse">
                    <thead>
                      <tr className="bg-slate-950 border-b border-slate-800 text-slate-400">
                        <th className="py-2.5 px-3 font-semibold font-mono">Feld</th>
                        <th className="py-2.5 px-3 font-semibold">Datentyp</th>
                        <th className="py-2.5 px-3 font-semibold">Bedeutung</th>
                        <th className="py-2.5 px-3 font-semibold font-mono">Beispielwert</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-slate-800/60 bg-slate-900/40">
                      {currentModel.schemaFields.map((field, idx) => (
                        <tr key={idx} className="hover:bg-slate-800/30">
                          <td className="py-2.5 px-3 font-mono text-cyan-300 font-semibold">{field.field}</td>
                          <td className="py-2.5 px-3 font-mono text-indigo-300">{field.type}</td>
                          <td className="py-2.5 px-3 text-slate-300">{field.description}</td>
                          <td className="py-2.5 px-3 font-mono text-amber-300/90">{field.example}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* ========================================================
            TAB 6: END-TO-END TECHNISCHE KETTE
        ======================================================== */}
        {activeTab === 'chain' && (
          <div className="space-y-6">
            <div className="p-4 rounded-xl bg-slate-900/80 border border-slate-800 text-xs text-slate-300 flex items-start gap-3">
              <Workflow className="w-5 h-5 text-indigo-400 flex-none mt-0.5" />
              <div>
                <span className="font-semibold text-white">Vollständige Vertikale Integration:</span> Diese End-to-End Kette zeigt den realen Kontrollfluss vom Benutzer-Klick über die Zwischenstationen bis zu den physischen CPU-Instruktionen und MMIO-Transaktionen. Parallel dazu greifen permanent die 7 horizontalen Control Planes.
              </div>
            </div>

            {/* Visual ASCII & Node representation */}
            <div className="p-6 rounded-2xl bg-slate-950/80 border border-slate-800 space-y-4 font-mono text-xs overflow-x-auto">
              <div className="text-slate-400 mb-2 font-sans text-sm font-bold flex items-center justify-between">
                <span>End-to-End System Control Flow Diagram</span>
                <span className="text-xs font-normal text-slate-500 font-mono">Dual Control Planes Active</span>
              </div>

              <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
                {/* Vertical Pipeline (8 cols) */}
                <div className="lg:col-span-8 space-y-1.5 text-slate-300">
                  <div className="p-2.5 rounded bg-cyan-950/40 border border-cyan-500/40 text-cyan-200">
                    [USER / ACTOR] → Menschliche Eingabe, Sensor-Event, RPC Client
                  </div>
                  <div className="text-center text-slate-600">↓</div>
                  <div className="p-2.5 rounded bg-blue-950/40 border border-blue-500/40 text-blue-200">
                    [D1: AURORA / EXPERIENCE] → Wayland Compositor / Web WASM Canvas / CLI
                  </div>
                  <div className="text-center text-slate-600">↓ (Contract C1)</div>
                  <div className="p-2.5 rounded bg-indigo-950/40 border border-indigo-500/40 text-indigo-200">
                    [D2: APPLICATION] → Wallet, DeFi, Games, Globus Studio, Native Apps
                  </div>
                  <div className="text-center text-slate-600">↓ (Contract C2)</div>
                  <div className="p-2.5 rounded bg-purple-950/40 border border-purple-500/40 text-purple-200">
                    [D3: PLATFORM SDK & API GATEWAY] → Typisierte Schemas, TLS, Rate Limiting
                  </div>
                  <div className="text-center text-slate-600">↓</div>
                  <div className="p-2.5 rounded bg-purple-950/40 border border-purple-500/40 text-purple-200">
                    [AUTH / IDENTITY & POLICY GATES] → DID Verification & Rego Rules
                  </div>
                  <div className="text-center text-slate-600">↓</div>
                  <div className="grid grid-cols-2 gap-2">
                    <div className="p-2 rounded bg-amber-950/40 border border-amber-500/40 text-amber-200 text-center">
                      AI PLANE (Agent Planner)
                    </div>
                    <div className="p-2 rounded bg-cyan-950/40 border border-cyan-500/40 text-cyan-200 text-center">
                      ATC PLANE (Transaction Engine)
                    </div>
                  </div>
                  <div className="text-center text-slate-600">↓ (Contract C3)</div>
                  <div className="p-2.5 rounded bg-amber-950/40 border border-amber-500/40 text-amber-200">
                    [D4: EXECUTION DOMAIN] → ATC VM / WASM / Containers / Plugins
                  </div>
                  <div className="text-center text-slate-600">↓ (Contract C4: GOS-HABI-001 Host Traps)</div>
                  <div className="p-2.5 rounded bg-emerald-950/40 border border-emerald-500/40 text-emerald-200">
                    [SYSCALL DISPATCHER] → MSR_LSTAR / svc #0 Fast Syscall Entry
                  </div>
                  <div className="text-center text-slate-600">↓ (Contract C5: 12-Step Security Pipeline)</div>
                  <div className="p-2.5 rounded bg-emerald-950/60 border border-emerald-500/60 text-emerald-100 font-bold">
                    [D5: SHIVACORE KERNEL NUCLEUS] → Scheduler, Memory Paging, Fast IPC, Cap Manager
                  </div>
                  <div className="text-center text-slate-600">↓</div>
                  <div className="p-2.5 rounded bg-slate-900 border border-slate-700 text-slate-300">
                    [HAL & DRIVERS] → Hardware Abstraction Layer (x86_64, ARM64, RISC-V)
                  </div>
                  <div className="text-center text-slate-600">↓</div>
                  <div className="p-2.5 rounded bg-slate-900 border border-slate-600 text-white font-bold">
                    [HARDWARE SILICON] → CPU Core, Memory DIMM, NVMe SSD, NIC, GPU, TPM 2.0
                  </div>
                </div>

                {/* Horizontal Control Planes (4 cols) */}
                <div className="lg:col-span-4 p-4 rounded-xl bg-slate-900/90 border border-slate-800 space-y-3 font-sans text-xs">
                  <div className="font-bold text-white text-sm border-b border-slate-800 pb-2 flex items-center gap-1.5">
                    <Sliders className="w-4 h-4 text-indigo-400" />
                    Parallele Control Planes
                  </div>
                  <div className="space-y-2">
                    <div className="p-2 rounded bg-red-950/30 border border-red-500/30 text-red-200">
                      <strong>Security Plane:</strong> DID, Capabilities, Crypto, Secrets
                    </div>
                    <div className="p-2 rounded bg-yellow-950/30 border border-yellow-500/30 text-yellow-200">
                      <strong>Governance Plane:</strong> DAO Rules, Gas Economics, Compliance
                    </div>
                    <div className="p-2 rounded bg-emerald-950/30 border border-emerald-500/30 text-emerald-200">
                      <strong>Observability Plane:</strong> Lockless Metrics, Traces, Health
                    </div>
                    <div className="p-2 rounded bg-cyan-950/30 border border-cyan-500/30 text-cyan-200">
                      <strong>Audit Plane:</strong> Merkle Trails, State Changes, Provenance
                    </div>
                    <div className="p-2 rounded bg-purple-950/30 border border-purple-500/30 text-purple-200">
                      <strong>Policy Plane:</strong> OPA/Rego, Agent Gates, Quota Policies
                    </div>
                    <div className="p-2 rounded bg-blue-950/30 border border-blue-500/30 text-blue-200">
                      <strong>Resource Plane:</strong> CPU Slices, Memory Pages, Network IOPS
                    </div>
                    <div className="p-2 rounded bg-teal-950/30 border border-teal-500/30 text-teal-200">
                      <strong>Configuration Plane:</strong> Immutable Parameter Tries
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* ========================================================
            TAB 7: STANDARDS-NORMENFAMILIE (19 SPECS)
        ======================================================== */}
        {activeTab === 'standards' && (
          <div className="space-y-6">
            <div className="p-4 rounded-xl bg-slate-900/80 border border-slate-800 text-xs text-slate-300 flex items-start gap-3">
              <FileCode className="w-5 h-5 text-indigo-400 flex-none mt-0.5" />
              <div>
                <span className="font-semibold text-white">Der Schritt von der Skizze zur Norm:</span> Die 19 formalen Standards definieren die Spezifikationsgrundlage für alle Globus OS & A-TownChain Implementierungen. Jede Spezifikation besitzt ein klares Artefakt und Referenzen.
              </div>
            </div>

            {/* Filter and Search Bar */}
            <div className="flex flex-col sm:flex-row gap-3 items-center justify-between">
              <div className="flex items-center gap-1.5 overflow-x-auto no-scrollbar w-full sm:w-auto">
                {['All', 'Architecture', 'ABI & Kernel', 'Security & Identity', 'Blockchain & VM', 'Platform API'].map(cat => (
                  <button
                    key={cat}
                    onClick={() => setStandardsFilter(cat)}
                    className={`px-3 py-1.5 rounded-lg text-xs font-medium transition cursor-pointer whitespace-nowrap ${
                      standardsFilter === cat
                        ? 'bg-indigo-600 text-white font-semibold shadow'
                        : 'bg-slate-900 text-slate-400 hover:bg-slate-800 hover:text-slate-200'
                    }`}
                  >
                    {cat}
                  </button>
                ))}
              </div>

              <div className="relative w-full sm:w-64">
                <Search className="w-3.5 h-3.5 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
                <input
                  type="text"
                  placeholder="Standard filtern..."
                  value={standardsSearch}
                  onChange={e => setStandardsSearch(e.target.value)}
                  className="w-full bg-slate-900 border border-slate-800 rounded-lg pl-8 pr-3 py-1.5 text-xs text-slate-200 focus:outline-none focus:border-indigo-500"
                />
              </div>
            </div>

            {/* Standards Table */}
            <div className="overflow-x-auto rounded-xl border border-slate-800 bg-slate-900/40">
              <table className="w-full text-left text-xs border-collapse">
                <thead>
                  <tr className="bg-slate-950 border-b border-slate-800 text-slate-400">
                    <th className="py-3 px-4 font-semibold font-mono">Code</th>
                    <th className="py-3 px-4 font-semibold">Titel & Bereich</th>
                    <th className="py-3 px-4 font-semibold">Status</th>
                    <th className="py-3 px-4 font-semibold">Beschreibung</th>
                    <th className="py-3 px-4 font-semibold font-mono">Primäres Artefakt</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-800/60">
                  {filteredStandards.map((spec, idx) => (
                    <tr key={idx} className="hover:bg-slate-800/30">
                      <td className="py-3 px-4 font-mono font-bold text-cyan-300 whitespace-nowrap">
                        {spec.code}
                      </td>
                      <td className="py-3 px-4">
                        <div className="font-semibold text-white">{spec.title}</div>
                        <div className="text-[11px] text-slate-500">{spec.category}</div>
                      </td>
                      <td className="py-3 px-4 whitespace-nowrap">
                        <span
                          className={`px-2 py-0.5 rounded text-[10px] font-bold ${
                            spec.status === 'Normative'
                              ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40'
                              : spec.status === 'Standardized'
                              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/40'
                              : 'bg-amber-500/20 text-amber-300 border border-amber-500/40'
                          }`}
                        >
                          {spec.status}
                        </span>
                      </td>
                      <td className="py-3 px-4 text-slate-300 max-w-md">
                        {spec.description}
                      </td>
                      <td className="py-3 px-4 font-mono text-amber-300/90 whitespace-nowrap">
                        {spec.primaryArtifact}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {/* ========================================================
            TAB 8: VOLLTEXT-SPEZIFIKATION (MARKDOWN)
        ======================================================== */}
        {activeTab === 'export' && (
          <div className="space-y-4">
            <div className="flex items-center justify-between bg-slate-900 p-4 rounded-xl border border-slate-800">
              <div>
                <h3 className="font-bold text-white text-sm">GLOBUS_SHIVACORE_V3_ARCHITECTURE.md</h3>
                <p className="text-xs text-slate-400 mt-0.5">
                  Vollständiges normatives Referenzdokument (GOS-ARCH-001 v3.0)
                </p>
              </div>
              <div className="flex items-center gap-2">
                <button
                  onClick={handleCopySpec}
                  className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 transition cursor-pointer"
                >
                  {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  <span>{copied ? 'Kopiert' : 'Copy All'}</span>
                </button>
                {onSaveWorkspaceFile && (
                  <button
                    onClick={handleSaveToWorkspace}
                    className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white transition cursor-pointer"
                  >
                    <Download className="w-3.5 h-3.5" />
                    <span>Save to File</span>
                  </button>
                )}
              </div>
            </div>

            <div className="p-4 rounded-xl bg-slate-950 border border-slate-800 font-mono text-xs text-slate-300 overflow-x-auto max-h-[600px] overflow-y-auto leading-relaxed whitespace-pre-wrap select-text">
              {GLOBUS_V3_MARKDOWN_SPEC}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
