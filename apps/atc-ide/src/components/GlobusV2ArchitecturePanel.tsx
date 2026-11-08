import React, { useState, useMemo } from 'react';
import {
  Layers,
  Shield,
  Cpu,
  Network,
  Database,
  Terminal,
  Boxes,
  Lock,
  ArrowRight,
  ChevronRight,
  ChevronDown,
  CheckCircle2,
  Download,
  Copy,
  Check,
  Search,
  Folder,
  File,
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
} from 'lucide-react';
import {
  GLOBUS_5_DOMAINS,
  HORIZONTAL_PLANES,
  INFRA_PLANES,
  IPC_PRIMITIVES,
  CONTAINER_FEATURES,
  FORMAL_SYSCALL_SPECS,
  GLOBUS_OS_REPO_TREE,
  GLOBUS_V2_MARKDOWN_SPEC,
  RepoNode,
} from '../data/globusV2Architecture';

interface GlobusV2ArchitecturePanelProps {
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export function GlobusV2ArchitecturePanel({
  onSaveWorkspaceFile,
  onShowToast,
}: GlobusV2ArchitecturePanelProps) {
  // Navigation tabs
  const [activeTab, setActiveTab] = useState<
    'domains' | 'planes' | 'infra' | 'ipc' | 'containers' | 'syscalls' | 'repo' | 'target'
  >('domains');

  // Active domain in 5-Domains view
  const [selectedDomainId, setSelectedDomainId] = useState<string>('experience');

  // Active horizontal plane
  const [selectedPlaneId, setSelectedPlaneId] = useState<string>('security');

  // Active infra plane
  const [selectedInfraId, setSelectedInfraId] = useState<string>('ai-plane');

  // Search & Filters for Syscalls
  const [syscallSearch, setSyscallSearch] = useState<string>('');
  const [selectedSyscall, setSelectedSyscall] = useState<number | null>(1);

  // Active IPC primitive selection
  const [selectedIpcName, setSelectedIpcName] = useState<string>('SC_IPC_CALL');

  // Copy state
  const [copiedKey, setCopiedKey] = useState<string | null>(null);

  // Repo tree expanded folders state
  const [expandedPaths, setExpandedPaths] = useState<Record<string, boolean>>({
    'globus-os/': true,
    'globus-os/kernel/': true,
    'globus-os/syscall/': true,
    'globus-os/blockchain/': true,
    'globus-os/ai/': true,
    'globus-os/runtime/': true,
  });

  const toggleExpand = (path: string) => {
    setExpandedPaths((prev) => ({
      ...prev,
      [path]: !prev[path],
    }));
  };

  const copyToClipboard = (text: string, key: string) => {
    navigator.clipboard.writeText(text);
    setCopiedKey(key);
    setTimeout(() => setCopiedKey(null), 2000);
    if (onShowToast) onShowToast('In Zwischenablage kopiert!', 'info');
  };

  const handleExportMarkdown = () => {
    if (!onSaveWorkspaceFile) return;
    onSaveWorkspaceFile('GLOBUS_SHIVACORE_V2_ARCHITECTURE.md', GLOBUS_V2_MARKDOWN_SPEC);
    if (onShowToast) {
      onShowToast("'GLOBUS_SHIVACORE_V2_ARCHITECTURE.md' erfolgreich im Workspace gespeichert!", 'info');
    }
  };

  const currentDomain = useMemo(() => {
    return GLOBUS_5_DOMAINS.find((d) => d.id === selectedDomainId) || GLOBUS_5_DOMAINS[0];
  }, [selectedDomainId]);

  const currentPlane = useMemo(() => {
    return HORIZONTAL_PLANES.find((p) => p.id === selectedPlaneId) || HORIZONTAL_PLANES[0];
  }, [selectedPlaneId]);

  const currentInfra = useMemo(() => {
    return INFRA_PLANES.find((i) => i.id === selectedInfraId) || INFRA_PLANES[0];
  }, [selectedInfraId]);

  const currentIpc = useMemo(() => {
    return IPC_PRIMITIVES.find((p) => p.name === selectedIpcName) || IPC_PRIMITIVES[3];
  }, [selectedIpcName]);

  const filteredSyscalls = useMemo(() => {
    if (!syscallSearch.trim()) return FORMAL_SYSCALL_SPECS;
    const q = syscallSearch.toLowerCase();
    return FORMAL_SYSCALL_SPECS.filter(
      (s) =>
        s.mnemonic.toLowerCase().includes(q) ||
        s.capability.toLowerCase().includes(q) ||
        s.args.toLowerCase().includes(q) ||
        s.memoryEffects.toLowerCase().includes(q)
    );
  }, [syscallSearch]);

  const activeSyscallDetail = useMemo(() => {
    return FORMAL_SYSCALL_SPECS.find((s) => s.id === selectedSyscall) || FORMAL_SYSCALL_SPECS[0];
  }, [selectedSyscall]);

  // Recursive Tree Node Renderer
  const renderTreeNode = (node: RepoNode, currentPath: string = '') => {
    const fullPath = `${currentPath}${node.name}`;
    const isDir = node.type === 'dir';
    const isExpanded = !!expandedPaths[fullPath];

    return (
      <div key={fullPath} className="text-xs font-mono">
        <div
          onClick={() => isDir && toggleExpand(fullPath)}
          className={`flex items-center gap-2 py-1 px-2 rounded hover:bg-white/5 transition-colors cursor-pointer ${
            isDir ? 'text-slate-200' : 'text-slate-400'
          }`}
        >
          {isDir ? (
            <span className="text-slate-500">
              {isExpanded ? <ChevronDown className="w-3.5 h-3.5" /> : <ChevronRight className="w-3.5 h-3.5" />}
            </span>
          ) : (
            <span className="w-3.5" />
          )}

          {isDir ? (
            <Folder className="w-3.5 h-3.5 text-cyan-400 shrink-0" />
          ) : (
            <File className="w-3.5 h-3.5 text-slate-500 shrink-0" />
          )}

          <span className={`font-semibold ${isDir ? 'text-cyan-300' : 'text-slate-300'}`}>{node.name}</span>
          <span className="text-[11px] text-slate-500 truncate font-sans ml-2">// {node.desc}</span>
        </div>

        {isDir && isExpanded && node.children && (
          <div className="pl-4 ml-2 border-l border-white/10 space-y-0.5">
            {node.children.map((child) => renderTreeNode(child, `${fullPath}/`))}
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="flex-1 flex flex-col h-full bg-[#08080a] text-slate-200 select-none overflow-hidden font-sans">
      {/* Sub-Header / Nav */}
      <div className="h-12 px-4 md:px-6 bg-black/60 border-b border-white/10 flex items-center justify-between shrink-0 overflow-x-auto">
        <div className="flex items-center gap-1.5 shrink-0">
          <button
            onClick={() => setActiveTab('domains')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 ${
              activeTab === 'domains'
                ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <Layers className="w-3.5 h-3.5 text-cyan-400" />
            <span>5-Domain Modell</span>
          </button>

          <button
            onClick={() => setActiveTab('planes')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 ${
              activeTab === 'planes'
                ? 'bg-red-500/20 text-red-300 border border-red-500/30'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <Shield className="w-3.5 h-3.5 text-red-400" />
            <span>Querschnitts-Planes (Security, Obs, Gov)</span>
          </button>

          <button
            onClick={() => setActiveTab('infra')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 ${
              activeTab === 'infra'
                ? 'bg-purple-500/20 text-purple-300 border border-purple-500/30'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <Cpu className="w-3.5 h-3.5 text-purple-400" />
            <span>Spezial-Planes (AI, Chain, Storage, Net)</span>
          </button>

          <button
            onClick={() => setActiveTab('ipc')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 ${
              activeTab === 'ipc'
                ? 'bg-amber-500/20 text-amber-300 border border-amber-500/30'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <Zap className="w-3.5 h-3.5 text-amber-400" />
            <span>Kernel IPC ({IPC_PRIMITIVES.length})</span>
          </button>

          <button
            onClick={() => setActiveTab('containers')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 ${
              activeTab === 'containers'
                ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <Boxes className="w-3.5 h-3.5 text-emerald-400" />
            <span>Container Sandbox</span>
          </button>

          <button
            onClick={() => setActiveTab('syscalls')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 ${
              activeTab === 'syscalls'
                ? 'bg-teal-500/20 text-teal-300 border border-teal-500/30'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <Terminal className="w-3.5 h-3.5 text-teal-400" />
            <span>Syscall ABI ({FORMAL_SYSCALL_SPECS.length})</span>
          </button>

          <button
            onClick={() => setActiveTab('repo')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 ${
              activeTab === 'repo'
                ? 'bg-blue-500/20 text-blue-300 border border-blue-500/30'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <Folder className="w-3.5 h-3.5 text-blue-400" />
            <span>globus-os/ Repo Tree</span>
          </button>

          <button
            onClick={() => setActiveTab('target')}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all flex items-center gap-1.5 ${
              activeTab === 'target'
                ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <Sparkles className="w-3.5 h-3.5 text-indigo-400" />
            <span>Zielbild Visualisierung</span>
          </button>
        </div>

        {onSaveWorkspaceFile && (
          <button
            onClick={handleExportMarkdown}
            className="px-3 py-1.5 rounded-lg bg-cyan-600/30 hover:bg-cyan-600/40 text-cyan-200 border border-cyan-500/40 text-xs font-bold flex items-center gap-1.5 shrink-0 transition-colors ml-2"
            title="Exportiert vollständiges Globus OS v2 Referenzarchitektur-Dokument in den Workspace"
          >
            <Download className="w-3.5 h-3.5" />
            <span className="hidden sm:inline">Export als GLOBUS_SHIVACORE_V2_ARCHITECTURE.md</span>
          </button>
        )}
      </div>

      {/* TAB CONTENT */}
      <div className="flex-1 overflow-hidden">
        {/* 1. 5-DOMAINS TAB */}
        {activeTab === 'domains' && (
          <div className="h-full flex overflow-hidden">
            {/* Left selector */}
            <div className="w-80 md:w-96 border-r border-white/10 bg-black/40 overflow-y-auto p-4 shrink-0 flex flex-col gap-2">
              <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1 px-1">
                Die 5 Kern-Domains
              </div>
              {GLOBUS_5_DOMAINS.map((dom) => {
                const isSelected = dom.id === selectedDomainId;
                return (
                  <button
                    key={dom.id}
                    onClick={() => setSelectedDomainId(dom.id)}
                    className={`w-full text-left p-3.5 rounded-xl border transition-all flex flex-col gap-1.5 ${
                      isSelected
                        ? 'bg-cyan-500/15 border-cyan-500/50 shadow-md ring-1 ring-cyan-500/30 text-white'
                        : 'bg-white/[0.02] border-white/5 hover:bg-white/5 text-slate-300'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="text-xs font-bold font-mono text-cyan-400">{dom.name}</span>
                      <span className="text-[9px] px-2 py-0.5 rounded font-mono uppercase bg-white/5 text-slate-400">
                        {dom.badge}
                      </span>
                    </div>
                    <p className="text-xs text-slate-300 line-clamp-1">{dom.headline}</p>
                  </button>
                );
              })}
            </div>

            {/* Right details */}
            <div className="flex-1 overflow-y-auto p-6 md:p-8 space-y-6">
              <div className="max-w-4xl space-y-6">
                <div className="p-6 rounded-2xl bg-cyan-950/20 border border-cyan-500/30 space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="px-2.5 py-1 rounded-md bg-cyan-500/10 border border-cyan-500/30 text-cyan-400 text-xs font-mono font-bold">
                      {currentDomain.badge}
                    </span>
                    <span className="text-xs font-mono text-slate-400">Architektur-Kategorie: Formal Domain</span>
                  </div>
                  <h1 className="text-2xl font-bold text-white">{currentDomain.name}</h1>
                  <h2 className="text-sm font-semibold text-cyan-300">{currentDomain.headline}</h2>
                  <p className="text-sm text-slate-300 leading-relaxed">{currentDomain.description}</p>
                </div>

                {/* Elements */}
                <div className="space-y-3">
                  <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                    Enthaltene Subsysteme & Kernkomponenten ({currentDomain.elements.length})
                  </h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                    {currentDomain.elements.map((el, idx) => (
                      <div key={idx} className="p-3.5 rounded-xl bg-black/40 border border-white/10 flex items-start gap-2.5">
                        <CheckCircle2 className="w-4 h-4 text-cyan-400 shrink-0 mt-0.5" />
                        <span className="text-xs text-slate-200 font-medium">{el}</span>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Interfaces & Responsibilities */}
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="p-5 rounded-xl bg-black/50 border border-white/10 space-y-3">
                    <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-2">
                      <Network className="w-4 h-4 text-cyan-400" />
                      Schnittstellen & Protokoll-Verträge
                    </h3>
                    <ul className="space-y-2 text-xs text-slate-400">
                      {currentDomain.interfaces.map((iface, iIdx) => (
                        <li key={iIdx} className="font-mono text-cyan-300 flex items-center gap-2">
                          <span className="w-1.5 h-1.5 rounded-full bg-cyan-400" />
                          <span>{iface}</span>
                        </li>
                      ))}
                    </ul>
                  </div>

                  <div className="p-5 rounded-xl bg-black/50 border border-white/10 space-y-3">
                    <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-2">
                      <Shield className="w-4 h-4 text-amber-400" />
                      Kernverantwortlichkeiten & Isolation
                    </h3>
                    <ul className="space-y-2 text-xs text-slate-300">
                      {currentDomain.responsibilities.map((resp, rIdx) => (
                        <li key={rIdx} className="flex items-start gap-2">
                          <span className="w-1.5 h-1.5 rounded-full bg-amber-400 shrink-0 mt-1.5" />
                          <span>{resp}</span>
                        </li>
                      ))}
                    </ul>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 2. QUERSCHNITTS-PLANES TAB */}
        {activeTab === 'planes' && (
          <div className="h-full flex overflow-hidden">
            {/* Plane selector */}
            <div className="w-80 md:w-96 border-r border-white/10 bg-black/40 overflow-y-auto p-4 shrink-0 flex flex-col gap-2">
              <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1 px-1">
                Horizontale Querschnitts-Ebenen
              </div>
              {HORIZONTAL_PLANES.map((plane) => {
                const isSelected = plane.id === selectedPlaneId;
                return (
                  <button
                    key={plane.id}
                    onClick={() => setSelectedPlaneId(plane.id)}
                    className={`w-full text-left p-3.5 rounded-xl border transition-all flex flex-col gap-1.5 ${
                      isSelected
                        ? 'bg-red-500/15 border-red-500/50 shadow-md ring-1 ring-red-500/30 text-white'
                        : 'bg-white/[0.02] border-white/5 hover:bg-white/5 text-slate-300'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="text-xs font-bold text-red-400 font-mono">{plane.name.split(' (')[0]}</span>
                      <span className="text-[9px] px-2 py-0.5 rounded font-mono uppercase bg-white/5 text-slate-400">
                        {plane.badge}
                      </span>
                    </div>
                    <p className="text-xs text-slate-400 line-clamp-1">{plane.description}</p>
                  </button>
                );
              })}
            </div>

            {/* Plane details */}
            <div className="flex-1 overflow-y-auto p-6 md:p-8 space-y-6">
              <div className="max-w-4xl space-y-6">
                <div className="p-6 rounded-2xl bg-red-950/20 border border-red-500/30 space-y-2">
                  <span className="px-2.5 py-1 rounded-md bg-red-500/10 border border-red-500/30 text-red-400 text-xs font-mono font-bold">
                    {currentPlane.badge}
                  </span>
                  <h1 className="text-2xl font-bold text-white">{currentPlane.name}</h1>
                  <p className="text-sm text-slate-300 leading-relaxed">{currentPlane.description}</p>
                </div>

                {/* 10-Step Pipeline or Pillars */}
                <div className="space-y-4">
                  <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                    Säulen & Spezifikationen ({currentPlane.pillars.length})
                  </h3>
                  <div className="space-y-4">
                    {currentPlane.pillars.map((pillar, idx) => (
                      <div key={idx} className="p-5 rounded-xl bg-black/40 border border-white/10 space-y-3">
                        <h4 className="text-sm font-bold text-cyan-300 font-mono">{pillar.title}</h4>
                        <p className="text-xs text-slate-300">{pillar.description}</p>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-2 pt-2 border-t border-white/5">
                          {pillar.features.map((feat, fIdx) => (
                            <div
                              key={fIdx}
                              className="p-2.5 rounded-lg bg-black/60 border border-white/5 text-xs text-slate-300 font-mono flex items-start gap-2"
                            >
                              <span className="text-cyan-400 font-bold shrink-0">&gt;</span>
                              <span>{feat}</span>
                            </div>
                          ))}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 3. SPEZIAL-INFRASTRUCTURE PLANES TAB */}
        {activeTab === 'infra' && (
          <div className="h-full flex overflow-hidden">
            {/* Left list */}
            <div className="w-80 md:w-96 border-r border-white/10 bg-black/40 overflow-y-auto p-4 shrink-0 flex flex-col gap-2">
              <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1 px-1">
                Spezialisierte Plattformen
              </div>
              {INFRA_PLANES.map((infra) => {
                const isSelected = infra.id === selectedInfraId;
                return (
                  <button
                    key={infra.id}
                    onClick={() => setSelectedInfraId(infra.id)}
                    className={`w-full text-left p-3.5 rounded-xl border transition-all flex flex-col gap-1.5 ${
                      isSelected
                        ? 'bg-purple-500/15 border-purple-500/50 shadow-md ring-1 ring-purple-500/30 text-white'
                        : 'bg-white/[0.02] border-white/5 hover:bg-white/5 text-slate-300'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="text-xs font-bold text-purple-300 font-mono">{infra.name}</span>
                      <span className="text-[9px] px-2 py-0.5 rounded font-mono uppercase bg-white/5 text-slate-400">
                        {infra.badge}
                      </span>
                    </div>
                    <p className="text-xs text-slate-400 line-clamp-1">{infra.description}</p>
                  </button>
                );
              })}
            </div>

            {/* Right details */}
            <div className="flex-1 overflow-y-auto p-6 md:p-8 space-y-6">
              <div className="max-w-4xl space-y-6">
                <div className="p-6 rounded-2xl bg-purple-950/20 border border-purple-500/30 space-y-3">
                  <span className="px-2.5 py-1 rounded-md bg-purple-500/10 border border-purple-500/30 text-purple-300 text-xs font-mono font-bold">
                    {currentInfra.badge}
                  </span>
                  <h1 className="text-2xl font-bold text-white">{currentInfra.name}</h1>
                  <p className="text-sm text-slate-300 leading-relaxed">{currentInfra.description}</p>

                  <div className="p-3 rounded-lg bg-black/80 border border-white/10 font-mono text-xs text-cyan-300 mt-2">
                    <span className="text-[10px] text-slate-500 uppercase tracking-wider block mb-1">
                      Vertikaler Kontrollfluss (Pipeline):
                    </span>
                    {currentInfra.flow}
                  </div>
                </div>

                {/* Tree modules */}
                <div className="space-y-3">
                  <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                    Modulbaum & Subsysteme ({currentInfra.tree.length})
                  </h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {currentInfra.tree.map((node, idx) => (
                      <div key={idx} className="p-4 rounded-xl bg-black/40 border border-white/10 space-y-2">
                        <div className="flex items-center gap-2">
                          <span className="w-2 h-2 rounded-full bg-purple-400" />
                          <h4 className="text-xs font-bold text-white font-mono">{node.name}</h4>
                        </div>
                        {node.sub && (
                          <ul className="space-y-1 pl-4 border-l border-white/10 text-xs text-slate-400">
                            {node.sub.map((subItem, sIdx) => (
                              <li key={sIdx}>{subItem}</li>
                            ))}
                          </ul>
                        )}
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 4. KERNEL IPC INFRASTRUCTURE TAB */}
        {activeTab === 'ipc' && (
          <div className="h-full flex overflow-hidden">
            {/* Left list */}
            <div className="w-80 md:w-96 border-r border-white/10 bg-black/40 overflow-y-auto p-4 shrink-0 flex flex-col gap-2">
              <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1 px-1">
                ShivaCore IPC Primitive ({IPC_PRIMITIVES.length})
              </div>
              {IPC_PRIMITIVES.map((ipc) => {
                const isSelected = ipc.name === selectedIpcName;
                return (
                  <button
                    key={ipc.name}
                    onClick={() => setSelectedIpcName(ipc.name)}
                    className={`w-full text-left p-3 rounded-xl border transition-all flex flex-col gap-1 ${
                      isSelected
                        ? 'bg-amber-500/15 border-amber-500/50 shadow-md text-white'
                        : 'bg-white/[0.02] border-white/5 hover:bg-white/5 text-slate-300'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="text-xs font-bold font-mono text-amber-300">{ipc.name}</span>
                      <span className="text-[9px] px-1.5 py-0.5 rounded font-mono bg-white/5 text-slate-400">
                        {ipc.timing.split(' ')[0]}
                      </span>
                    </div>
                    <p className="text-[11px] text-slate-400 line-clamp-1">{ipc.description}</p>
                  </button>
                );
              })}
            </div>

            {/* Right deep dive */}
            <div className="flex-1 overflow-y-auto p-6 md:p-8 space-y-6">
              <div className="max-w-4xl space-y-6">
                <div className="p-6 rounded-2xl bg-amber-950/20 border border-amber-500/30 space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="px-2.5 py-1 rounded-md bg-amber-500/10 border border-amber-500/30 text-amber-300 text-xs font-mono font-bold">
                      {currentIpc.timing}
                    </span>
                    <span className="text-xs font-mono text-slate-400">
                      Benötigt Capability: <strong className="text-cyan-400">{currentIpc.capabilityRequired}</strong>
                    </span>
                  </div>
                  <h1 className="text-2xl font-bold font-mono text-white">{currentIpc.name}</h1>
                  <p className="text-sm text-slate-300 leading-relaxed">{currentIpc.description}</p>
                </div>

                {/* C Signature */}
                <div className="p-5 rounded-xl bg-black/60 border border-white/10 space-y-3">
                  <div className="flex items-center justify-between">
                    <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider font-mono">
                      C-Signatur (Kernel Header)
                    </h3>
                    <button
                      onClick={() => copyToClipboard(currentIpc.signature, 'ipc_sig')}
                      className="px-2 py-1 rounded bg-white/5 hover:bg-white/10 text-xs font-mono text-slate-300 border border-white/5 flex items-center gap-1"
                    >
                      {copiedKey === 'ipc_sig' ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                      <span>Kopieren</span>
                    </button>
                  </div>
                  <pre className="p-4 rounded-lg bg-black font-mono text-xs text-amber-300 border border-white/5 overflow-x-auto whitespace-pre">
                    {currentIpc.signature}
                  </pre>
                </div>

                {/* Flow Diagram */}
                <div className="p-5 rounded-xl bg-black/50 border border-white/10 space-y-3">
                  <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider font-mono">
                    IPC Nachrichten- und Berechtigungsfluss
                  </h3>
                  <pre className="p-4 rounded-lg bg-black font-mono text-xs text-cyan-300 border border-white/5 overflow-x-auto leading-relaxed">
{`Process A (Sender)
   │
   │ IPC Message Payload (${currentIpc.name})
   ▼
IPC Endpoint Buffer
   │
   ├── [1] Capability Check (${currentIpc.capabilityRequired})
   ├── [2] Namespace & Domain Isolation
   ├── [3] Quota & Message Size Limit
   └── [4] Cryptographic Audit Hash
   │
   ▼
Process B (Receiver)`}
                  </pre>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 5. CONTAINER ARCHITECTURE TAB */}
        {activeTab === 'containers' && (
          <div className="h-full overflow-y-auto p-6 md:p-8 bg-[#08080a]">
            <div className="max-w-5xl mx-auto space-y-6">
              <div className="p-6 rounded-2xl bg-emerald-950/20 border border-emerald-500/30 space-y-2">
                <span className="text-xs font-mono text-emerald-400 uppercase tracking-wider font-bold">
                  ShivaBox Micro-Container
                </span>
                <h2 className="text-xl font-bold text-white">
                  Native Container-Sandbox Architektur
                </h2>
                <p className="text-xs text-slate-300 leading-relaxed max-w-3xl">
                  Container sind in ShivaCore keine externe Virtualisierungssoftware, sondern erstklassige Kernel-Sandboxen aus Namespaces, Capability-Whitelists, cgroup-Quotas und Copy-on-Write Overlays.
                </p>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {CONTAINER_FEATURES.map((feat, idx) => (
                  <div key={idx} className="p-5 rounded-xl bg-black/40 border border-white/10 space-y-2">
                    <div className="flex items-center justify-between">
                      <h3 className="text-xs font-bold text-emerald-300 font-mono">{feat.dimension}</h3>
                      <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-white/5 text-slate-400 border border-white/5">
                        {feat.kernelPrimitive}
                      </span>
                    </div>
                    <p className="text-xs text-slate-300">{feat.mechanism}</p>
                    <div className="pt-2 text-[11px] text-slate-400 border-t border-white/5">
                      <strong className="text-slate-300">Enforcement:</strong> {feat.enforcement}
                    </div>
                  </div>
                ))}
              </div>

              {/* Sandbox Diagram */}
              <div className="p-5 rounded-xl bg-black/60 border border-white/10 space-y-2">
                <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider font-mono">
                  Container Ausführungskette
                </h3>
                <pre className="p-4 rounded-lg bg-black font-mono text-xs text-emerald-300 border border-white/5 overflow-x-auto">
{`ATC Service / dApp
      │
      ▼
ATC Container (PID 1, Net NS, Isolated Mounts)
      │
      ▼
Capability Sandbox (Strict Bounded Rights Mask)
      │
      ▼
Syscall Barrier (BPF Filter Trap)
      │
      ▼
ShivaCore Microkernel (Ring 0)`}
                </pre>
              </div>
            </div>
          </div>
        )}

        {/* 6. FORMAL SYSCALL ABI MATRIX */}
        {activeTab === 'syscalls' && (
          <div className="h-full flex overflow-hidden">
            {/* List */}
            <div className="w-80 md:w-96 border-r border-white/10 bg-black/40 overflow-y-auto p-4 shrink-0 flex flex-col gap-2">
              <div className="relative mb-2">
                <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
                <input
                  type="text"
                  value={syscallSearch}
                  onChange={(e) => setSyscallSearch(e.target.value)}
                  placeholder="Syscall filtern..."
                  className="w-full pl-9 pr-3 py-1.5 bg-black/60 border border-white/10 rounded-lg text-xs text-slate-200 outline-none focus:border-cyan-500/50"
                />
              </div>

              <div className="space-y-1">
                {filteredSyscalls.map((sc) => {
                  const isSelected = sc.id === selectedSyscall;
                  return (
                    <button
                      key={sc.id}
                      onClick={() => setSelectedSyscall(sc.id)}
                      className={`w-full text-left p-2.5 rounded-lg border transition-all flex items-center justify-between ${
                        isSelected
                          ? 'bg-teal-500/20 border-teal-500/50 text-white shadow-sm'
                          : 'bg-white/[0.02] border-white/5 hover:bg-white/5 text-slate-400'
                      }`}
                    >
                      <div className="flex items-center gap-2">
                        <span className="text-[10px] font-mono text-slate-500">#{sc.id}</span>
                        <span className={`text-xs font-mono font-bold ${isSelected ? 'text-teal-300' : 'text-slate-300'}`}>
                          {sc.mnemonic}
                        </span>
                      </div>
                      <span className="text-[9px] font-mono px-1.5 py-0.5 rounded bg-white/5 text-slate-400">
                        {sc.abiVersion}
                      </span>
                    </button>
                  );
                })}
              </div>
            </div>

            {/* Details */}
            <div className="flex-1 overflow-y-auto p-6 md:p-8 space-y-6">
              <div className="max-w-4xl space-y-6">
                <div className="p-6 rounded-2xl bg-teal-950/20 border border-teal-500/30 space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="px-2.5 py-1 rounded-md bg-teal-500/10 border border-teal-500/30 text-teal-300 text-xs font-mono font-bold">
                      Syscall ID: {activeSyscallDetail.id} • ABI {activeSyscallDetail.abiVersion}
                    </span>
                    <span className="text-xs font-mono text-slate-400">
                      Rückgabetyp: <strong className="text-cyan-400">{activeSyscallDetail.returnType}</strong>
                    </span>
                  </div>
                  <h1 className="text-2xl font-bold font-mono text-white">{activeSyscallDetail.mnemonic}</h1>
                </div>

                {/* Specification Table */}
                <div className="p-5 rounded-xl bg-black/40 border border-white/10 space-y-3">
                  <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider font-mono">
                    Formale ABI-Spezifikation
                  </h3>
                  <div className="space-y-2 text-xs">
                    <div className="grid grid-cols-3 gap-2 p-2 rounded bg-white/[0.02]">
                      <span className="text-slate-500 font-mono">Argumente:</span>
                      <span className="col-span-2 text-slate-200 font-mono">{activeSyscallDetail.args}</span>
                    </div>
                    <div className="grid grid-cols-3 gap-2 p-2 rounded bg-white/[0.02]">
                      <span className="text-slate-500 font-mono">Benötigte Capability:</span>
                      <span className="col-span-2 text-amber-300 font-mono">{activeSyscallDetail.capability}</span>
                    </div>
                    <div className="grid grid-cols-3 gap-2 p-2 rounded bg-white/[0.02]">
                      <span className="text-slate-500 font-mono">Erforderliche Rechte:</span>
                      <span className="col-span-2 text-slate-200 font-mono">{activeSyscallDetail.rights}</span>
                    </div>
                    <div className="grid grid-cols-3 gap-2 p-2 rounded bg-white/[0.02]">
                      <span className="text-slate-500 font-mono">Speicher-Effekte:</span>
                      <span className="col-span-2 text-cyan-300 font-mono">{activeSyscallDetail.memoryEffects}</span>
                    </div>
                    <div className="grid grid-cols-3 gap-2 p-2 rounded bg-white/[0.02]">
                      <span className="text-slate-500 font-mono">Blockier-Verhalten:</span>
                      <span className="col-span-2 text-slate-200 font-mono">{activeSyscallDetail.blocking}</span>
                    </div>
                    <div className="grid grid-cols-3 gap-2 p-2 rounded bg-white/[0.02]">
                      <span className="text-slate-500 font-mono">Mögliche Fehler-Codes:</span>
                      <span className="col-span-2 text-red-300 font-mono">{activeSyscallDetail.errorCodes}</span>
                    </div>
                    <div className="grid grid-cols-3 gap-2 p-2 rounded bg-white/[0.02]">
                      <span className="text-slate-500 font-mono">Audit Event Trigger:</span>
                      <span className="col-span-2 text-emerald-300 font-mono">{activeSyscallDetail.auditEvent}</span>
                    </div>
                    <div className="grid grid-cols-3 gap-2 p-2 rounded bg-white/[0.02]">
                      <span className="text-slate-500 font-mono">Ressourcen-Quota:</span>
                      <span className="col-span-2 text-slate-200 font-mono">{activeSyscallDetail.quota}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* 7. REPO TREE TAB */}
        {activeTab === 'repo' && (
          <div className="h-full overflow-y-auto p-6 md:p-8 bg-[#08080a]">
            <div className="max-w-4xl mx-auto space-y-4">
              <div className="p-5 rounded-2xl bg-blue-950/20 border border-blue-500/30 space-y-1">
                <span className="text-xs font-mono text-blue-400 uppercase tracking-wider font-bold">
                  Codebase Layout
                </span>
                <h2 className="text-lg font-bold text-white">
                  Globus OS & ShivaCore Monorepo-Struktur (`globus-os/`)
                </h2>
                <p className="text-xs text-slate-400">
                  Klicken Sie auf Ordner, um die Subsysteme und Verzeichnisstrukturen zu inspizieren.
                </p>
              </div>

              <div className="p-5 rounded-xl bg-black/60 border border-white/10 space-y-1 font-mono">
                {renderTreeNode(GLOBUS_OS_REPO_TREE)}
              </div>
            </div>
          </div>
        )}

        {/* 8. TARGET COMPUTING PLATFORM TAB */}
        {activeTab === 'target' && (
          <div className="h-full overflow-y-auto p-6 md:p-8 bg-[#08080a]">
            <div className="max-w-5xl mx-auto space-y-6">
              <div className="p-6 rounded-2xl bg-indigo-950/20 border border-indigo-500/30 space-y-2">
                <span className="text-xs font-mono text-indigo-400 uppercase tracking-wider font-bold">
                  Computing Platform
                </span>
                <h2 className="text-xl font-bold text-white">
                  Das Globus OS Zielbild: Vertikal Integrierte Plattform
                </h2>
                <p className="text-xs text-slate-300 leading-relaxed max-w-3xl">
                  Aus „Blockchain + OS + VM + AI + Frontend“ wird eine einheitliche, vertikal aufeinander abgestimmte Plattformarchitektur mit durchgängigem Kontrollfluss.
                </p>
              </div>

              {/* Architectural Diagram */}
              <div className="p-6 rounded-2xl bg-black/70 border border-white/10 space-y-3">
                <div className="flex items-center justify-between">
                  <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider font-mono">
                    Systemarchitektur-Diagramm
                  </h3>
                  <button
                    onClick={() => copyToClipboard(GLOBUS_V2_MARKDOWN_SPEC, 'diagram')}
                    className="px-2 py-1 rounded bg-white/5 hover:bg-white/10 text-xs font-mono text-slate-300 border border-white/5 flex items-center gap-1"
                  >
                    {copiedKey === 'diagram' ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                    <span>Spezifikation kopieren</span>
                  </button>
                </div>
                <pre className="p-5 rounded-xl bg-black font-mono text-xs text-cyan-300 border border-white/5 overflow-x-auto leading-relaxed whitespace-pre">
{`                            GLOBUS OS
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
     AURORA                 SERVICES                  AGENTS
  (Desktop, Web,         (Wallet, Identity,       (Autonome Tasks,
   Mobile, Games)           NFT, Mining)            Tools, Memory)
        │                       │                       │
        └───────────────────────┼───────────────────────┘
                                │
                            API / SDK
              (ATC SDK, ShivaCore SDK, REST, gRPC, IPC)
                                │
                            MIDDLEWARE
          (API Gateway, Service Mesh, Event Bus, OPA Policy)
                                │
              ┌─────────────────┴─────────────────┐
              │                                   │
         ATC PLATFORM                        OS SERVICES
      (Consensus, Mempool,                (VFS, Storage,
       State Trie, P2P)                    Network, Audio)
              │                                   │
              └─────────────────┬─────────────────┘
                                │
                            RUNTIMES
              ┌─────────────────┼─────────────────┐
              │                 │                 │
           ATC VM             WASM            AGENT VM
        (Bytecode JIT,     (Sandboxed       (Coroutinen,
         Gas-Metered)       Plugins)        Task Planner)
              │                 │                 │
              └─────────────────┼─────────────────┘
                                │
                           SYSCALL ABI
             (Fast Syscall MSR_LSTAR / ARM svc #0, 39 Calls)
                                │
                         SHIVACORE KERNEL
              ┌─────────────────┼─────────────────┐
              │                 │                 │
           MEMORY              IPC             SECURITY
        (PML4 Paging,     (Lockless Ring,     (Zero-Trust,
         Buddy Alloc)       Rendezvous)       Capabilities)
              │                 │                 │
              └─────────────────┼─────────────────┘
                                │
                           HAL / DRIVERS
                 (CPU, GPU, NVMe, PCIe, NIC, Audio)
                                │
                            HARDWARE
            (x86_64 AMD64, ARM64 AArch64, RISC-V RV64GC)`}
                </pre>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
