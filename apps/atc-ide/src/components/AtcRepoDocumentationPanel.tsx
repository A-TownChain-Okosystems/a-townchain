import React, { useState, useMemo } from 'react';
import {
  FolderGit2,
  FileCode,
  FileText,
  Shield,
  ShieldCheck,
  ShieldAlert,
  AlertTriangle,
  Layers,
  BookOpen,
  Terminal,
  Cpu,
  Network,
  Binary,
  Database,
  GitBranch,
  GitMerge,
  Workflow,
  Download,
  Copy,
  Check,
  Search,
  ExternalLink,
  ChevronRight,
  ChevronDown,
  Sparkles,
  Info,
  CheckCircle2,
  Boxes,
  Lock,
  ArrowRight,
  Code,
  Hash,
  Eye,
  RefreshCw,
  Zap,
  Flame,
  Scale
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import {
  REPOSITORY_STANDARD_FILES,
  DOC_CATEGORIES,
  ATC_DOC_STANDARDS,
  ADR_REGISTRY,
  ATC_PROTOCOL_STANDARDS,
  StandardDocFile,
  AtcDocStandard,
  AdrEntry,
  AtcProtocolStandard
} from '../data/atcRepoDocumentationStandards';

interface AtcRepoDocumentationPanelProps {
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export function AtcRepoDocumentationPanel({
  onSaveWorkspaceFile,
  onShowToast
}: AtcRepoDocumentationPanelProps) {
  // Navigation tabs
  const [activeTab, setActiveTab] = useState<
    'explorer' | 'root' | 'github' | 'arch' | 'standards' | 'api' | 'vm' | 'blockchain' | 'adr' | 'manifest' | 'atc-doc'
  >('explorer');

  // Search & Filtering
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedFileIndex, setSelectedFileIndex] = useState<number>(0);
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [copiedPath, setCopiedPath] = useState<string | null>(null);

  // Standards specific state
  const [selectedStandardCode, setSelectedStandardCode] = useState<string>('ATC-0001');
  const [selectedStandardSection, setSelectedStandardSection] = useState<string>('all');

  // ADR specific state
  const [selectedAdrId, setSelectedAdrId] = useState<string>('ADR-0001');

  // ATC-DOC meta-standard specific state
  const [selectedAtcDocId, setSelectedAtcDocId] = useState<string>('ATC-DOC-001');

  // Filtered files list
  const filteredFiles = useMemo(() => {
    return REPOSITORY_STANDARD_FILES.filter(file => {
      const matchesCat = selectedCategory === 'all' || file.category === selectedCategory;
      const matchesSearch =
        searchQuery.trim() === '' ||
        file.path.toLowerCase().includes(searchQuery.toLowerCase()) ||
        file.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        file.summary.toLowerCase().includes(searchQuery.toLowerCase());
      return matchesCat && matchesSearch;
    });
  }, [selectedCategory, searchQuery]);

  // Current active file
  const activeFile: StandardDocFile = useMemo(() => {
    if (filteredFiles.length === 0) return REPOSITORY_STANDARD_FILES[0];
    const found = filteredFiles[selectedFileIndex] || filteredFiles[0];
    return found;
  }, [filteredFiles, selectedFileIndex]);

  // Copy helper
  const handleCopy = (text: string, identifier: string) => {
    navigator.clipboard.writeText(text);
    setCopiedPath(identifier);
    if (onShowToast) onShowToast(`In die Zwischenablage kopiert!`, 'info');
    setTimeout(() => setCopiedPath(null), 2000);
  };

  // Save single file to workspace
  const handleSaveFile = (file: StandardDocFile) => {
    if (!onSaveWorkspaceFile) return;
    onSaveWorkspaceFile(file.path, file.content);
    if (onShowToast) onShowToast(`'${file.path}' im Workspace gespeichert!`, 'info');
  };

  // Scaffold all files in the current view or category
  const handleScaffoldCategory = (category: string) => {
    if (!onSaveWorkspaceFile) return;
    const targetFiles =
      category === 'all'
        ? REPOSITORY_STANDARD_FILES
        : REPOSITORY_STANDARD_FILES.filter(f => f.category === category);

    targetFiles.forEach(f => {
      onSaveWorkspaceFile(f.path, f.content);
    });

    if (onShowToast) {
      onShowToast(`${targetFiles.length} Dokumente erfolgreich in Workspace generiert!`, 'info');
    }
  };

  // Export full consolidated markdown bundle
  const handleExportFullBundle = () => {
    if (!onSaveWorkspaceFile) return;

    let bundle = `# A-TownChain / ShivaCore / Globus OS Ecosystem
## Consolidated Master Engineering Documentation Bundle
Generated according to ATC-DOC-001 through ATC-DOC-008 Standards.

---

# Table of Contents
1. Mandatory Root Files
2. GitHub Engineering Governance (.github)
3. Technical Architecture Suite (docs/)
4. ATC Protocol Standards (ATC-0001 to ATC-0094)
5. API Specifications (REST, JSON-RPC, WebSocket, OpenAPI)
6. Software-Specific ATC-VM Modular Specifications
7. Blockchain-Specific Protocol Specifications
8. Architecture Decision Records (ADRs)
9. Repository Manifests & Registers
10. ATC-DOC Overarching Meta-Standards

---
`;

    // Add standard files
    REPOSITORY_STANDARD_FILES.forEach(f => {
      bundle += `\n\n## File: ${f.path}\n**Category**: ${f.category} | **Status**: ${f.status} | **Version**: ${f.version} | **Owner**: ${f.owner}\n\n\`\`\`markdown\n${f.content}\n\`\`\`\n`;
    });

    // Add protocol standards
    bundle += `\n\n# SECTION: ATC PROTOCOL STANDARDS REGISTRY\n`;
    ATC_PROTOCOL_STANDARDS.forEach(s => {
      bundle += `\n\n### Standard: ${s.code} - ${s.name}\nStatus: ${s.status} | Version: ${s.version} | Authors: ${s.authors.join(', ')}\n\n`;
      bundle += `#### Abstract\n${s.sections.abstract}\n\n`;
      bundle += `#### Motivation\n${s.sections.motivation}\n\n`;
      bundle += `#### Specification\n${s.sections.specification}\n\n`;
      bundle += `#### Terminology\n${s.sections.terminology}\n\n`;
      bundle += `#### Data Structures\n\`\`\`rust\n${s.sections.dataStructures}\n\`\`\`\n\n`;
      bundle += `#### Encoding\n${s.sections.encoding}\n\n`;
      bundle += `#### State Transitions\n${s.sections.stateTransitions}\n\n`;
      bundle += `#### Validation Rules\n${s.sections.validationRules}\n\n`;
      bundle += `#### Error Conditions\n${s.sections.errorConditions}\n\n`;
      bundle += `#### Security Considerations\n${s.sections.securityConsiderations}\n\n`;
      bundle += `#### Compatibility\n${s.sections.compatibility}\n\n`;
      bundle += `#### Test Vectors\n\`\`\`text\n${s.sections.testVectors}\n\`\`\`\n\n`;
      bundle += `#### Reference Implementation\n${s.sections.referenceImplementation}\n\n`;
      bundle += `#### Changelog\n${s.sections.changelog}\n\n`;
    });

    // Add ADRs
    bundle += `\n\n# SECTION: ARCHITECTURE DECISION RECORDS (ADRs)\n`;
    ADR_REGISTRY.forEach(adr => {
      bundle += `\n\n### ${adr.id}: ${adr.title}\nStatus: ${adr.status} | Date: ${adr.date} | Author: ${adr.author}\n\n`;
      bundle += `#### Context\n${adr.context}\n\n`;
      bundle += `#### Decision\n${adr.decision}\n\n`;
      bundle += `#### Alternatives Evaluated\n${adr.alternatives.map(a => `- ${a}`).join('\n')}\n\n`;
      bundle += `#### Consequences (Positive)\n${adr.consequences.positive.map(p => `- ${p}`).join('\n')}\n\n`;
      bundle += `#### Consequences (Negative / Trade-offs)\n${adr.consequences.negative.map(n => `- ${n}`).join('\n')}\n\n`;
    });

    // Add ATC-DOC Meta standards
    bundle += `\n\n# SECTION: ATC-DOC OVERARCHING META-STANDARDS\n`;
    ATC_DOC_STANDARDS.forEach(m => {
      bundle += `\n\n### ${m.id}: ${m.title}\nCategory: ${m.category} | Version: ${m.version} | Status: ${m.status}\n\n`;
      bundle += `**Abstract**: ${m.abstract}\n\n`;
      bundle += `**Mandatory Rules**:\n${m.rules.map(r => `1. ${r}`).join('\n')}\n\n`;
    });

    onSaveWorkspaceFile('docs/ATC_MASTER_DOCUMENTATION_BUNDLE.md', bundle);
    if (onShowToast) onShowToast("'docs/ATC_MASTER_DOCUMENTATION_BUNDLE.md' im Workspace generiert!", 'info');
  };

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-100 overflow-hidden">
      {/* HEADER BAR */}
      <div className="p-4 border-b border-white/10 bg-slate-900/90 backdrop-blur flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-xl bg-gradient-to-br from-indigo-500/20 to-cyan-500/20 border border-indigo-500/30 text-indigo-400">
            <FolderGit2 className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-base font-bold text-white tracking-tight">
                Repository Documentation Standard (ATC-DOC Framework)
              </h2>
              <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30">
                GOS & ATC Governance
              </span>
            </div>
            <p className="text-xs text-slate-400">
              Maschinenlesbare, standardisierte Dokumentationsarchitektur für A-TownChain, ShivaCore, Globus OS & ATC-VM
            </p>
          </div>
        </div>

        {/* Global Action Buttons */}
        <div className="flex items-center gap-2 flex-wrap">
          <button
            onClick={() => handleScaffoldCategory('all')}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white shadow-sm transition-all cursor-pointer"
            title="Generiert alle definierten Standarddokumente direkt in das aktive Workspace"
          >
            <Sparkles className="w-3.5 h-3.5" />
            <span>Alle Docs im Workspace anlegen</span>
          </button>

          <button
            onClick={handleExportFullBundle}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-white/10 hover:bg-white/15 text-slate-200 border border-white/10 transition-all cursor-pointer"
            title="Exportiert das gesamte normierte Dokumentations-Bundle als Master Markdown Datei"
          >
            <Download className="w-3.5 h-3.5 text-cyan-400" />
            <span>Master Bundle exportieren</span>
          </button>
        </div>
      </div>

      {/* SUB-NAVIGATION TABS (The 10 Areas of the Standard) */}
      <div className="px-4 py-2 border-b border-white/5 bg-slate-900/50 overflow-x-auto flex items-center gap-1 scrollbar-none text-xs">
        <button
          onClick={() => { setActiveTab('explorer'); setSelectedCategory('all'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'explorer'
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <FolderGit2 className="w-3.5 h-3.5 text-indigo-400" />
          <span>Tree & Explorer</span>
        </button>

        <button
          onClick={() => { setActiveTab('root'); setSelectedCategory('root'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'root'
              ? 'bg-rose-500/20 text-rose-300 border border-rose-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <ShieldAlert className="w-3.5 h-3.5 text-rose-400" />
          <span>1. Root Pflichtdokumente</span>
        </button>

        <button
          onClick={() => { setActiveTab('github'); setSelectedCategory('github'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'github'
              ? 'bg-amber-500/20 text-amber-300 border border-amber-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <GitMerge className="w-3.5 h-3.5 text-amber-400" />
          <span>2. .github Governance</span>
        </button>

        <button
          onClick={() => { setActiveTab('arch'); setSelectedCategory('arch'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'arch'
              ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <Layers className="w-3.5 h-3.5 text-cyan-400" />
          <span>3. Technische Architektur</span>
        </button>

        <button
          onClick={() => { setActiveTab('standards'); setSelectedCategory('standards'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'standards'
              ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <Hash className="w-3.5 h-3.5 text-emerald-400" />
          <span>4. Standards (ATC-0001 - 0094)</span>
        </button>

        <button
          onClick={() => { setActiveTab('api'); setSelectedCategory('api'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'api'
              ? 'bg-sky-500/20 text-sky-300 border border-sky-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <Terminal className="w-3.5 h-3.5 text-sky-400" />
          <span>5. API & OpenAPI</span>
        </button>

        <button
          onClick={() => { setActiveTab('vm'); setSelectedCategory('vm'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'vm'
              ? 'bg-purple-500/20 text-purple-300 border border-purple-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <Cpu className="w-3.5 h-3.5 text-purple-400" />
          <span>6. ATC-VM Modular (15 Specs)</span>
        </button>

        <button
          onClick={() => { setActiveTab('blockchain'); setSelectedCategory('blockchain'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'blockchain'
              ? 'bg-orange-500/20 text-orange-300 border border-orange-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <Boxes className="w-3.5 h-3.5 text-orange-400" />
          <span>7. Blockchain Docs</span>
        </button>

        <button
          onClick={() => { setActiveTab('adr'); setSelectedCategory('adr'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'adr'
              ? 'bg-blue-500/20 text-blue-300 border border-blue-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <Scale className="w-3.5 h-3.5 text-blue-400" />
          <span>8. ADRs (Entscheidungen)</span>
        </button>

        <button
          onClick={() => { setActiveTab('manifest'); setSelectedCategory('manifest'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'manifest'
              ? 'bg-teal-500/20 text-teal-300 border border-teal-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <Database className="w-3.5 h-3.5 text-teal-400" />
          <span>9. Manifeste & Register</span>
        </button>

        <button
          onClick={() => { setActiveTab('atc-doc'); setSelectedCategory('atc-doc'); }}
          className={`px-3 py-1.5 rounded-lg font-medium whitespace-nowrap transition-colors flex items-center gap-1.5 ${
            activeTab === 'atc-doc'
              ? 'bg-fuchsia-500/20 text-fuchsia-300 border border-fuchsia-500/40'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          <BookOpen className="w-3.5 h-3.5 text-fuchsia-400" />
          <span>10. ATC-DOC Standards</span>
        </button>
      </div>

      {/* MAIN VIEW AREA */}
      <div className="flex-1 overflow-hidden flex flex-col md:flex-row">
        {/* VIEW 1: EXPLORER & TREE VIEW */}
        {(activeTab === 'explorer' || activeTab === 'root' || activeTab === 'github' || activeTab === 'arch' || activeTab === 'api' || activeTab === 'vm' || activeTab === 'blockchain' || activeTab === 'manifest') && (
          <>
            {/* Left Column: File List & Search */}
            <div className="w-full md:w-80 lg:w-96 border-r border-white/10 flex flex-col bg-slate-900/40 overflow-hidden">
              {/* Search box */}
              <div className="p-3 border-b border-white/10">
                <div className="relative">
                  <Search className="w-4 h-4 text-slate-400 absolute left-3 top-2.5" />
                  <input
                    type="text"
                    value={searchQuery}
                    onChange={e => { setSearchQuery(e.target.value); setSelectedFileIndex(0); }}
                    placeholder="Dateiname oder Inhalt suchen..."
                    className="w-full pl-9 pr-3 py-1.5 rounded-lg bg-white/5 border border-white/10 text-xs text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition-colors"
                  />
                </div>

                <div className="flex items-center justify-between mt-2 text-[11px] text-slate-400">
                  <span>{filteredFiles.length} Dokumente verfügbar</span>
                  {activeTab !== 'explorer' && (
                    <button
                      onClick={() => handleScaffoldCategory(selectedCategory)}
                      className="text-indigo-400 hover:text-indigo-300 font-semibold"
                    >
                      Kategorie im Workspace anlegen
                    </button>
                  )}
                </div>
              </div>

              {/* Scrollable File List */}
              <div className="flex-1 overflow-y-auto p-2 space-y-1">
                {filteredFiles.map((file, idx) => {
                  const isSelected = activeFile.path === file.path;
                  return (
                    <button
                      key={file.path}
                      onClick={() => setSelectedFileIndex(idx)}
                      className={`w-full text-left px-3 py-2.5 rounded-lg text-xs transition-all flex items-start gap-2.5 ${
                        isSelected
                          ? 'bg-indigo-500/20 text-white border border-indigo-500/40 shadow-sm'
                          : 'text-slate-300 hover:bg-white/5 hover:text-white border border-transparent'
                      }`}
                    >
                      <div className="mt-0.5">
                        {file.priority === 'mandatory' ? (
                          <ShieldAlert className="w-3.5 h-3.5 text-rose-400" />
                        ) : file.category === 'github' ? (
                          <GitMerge className="w-3.5 h-3.5 text-amber-400" />
                        ) : file.category === 'vm' ? (
                          <Cpu className="w-3.5 h-3.5 text-purple-400" />
                        ) : (
                          <FileCode className="w-3.5 h-3.5 text-indigo-400" />
                        )}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center justify-between gap-1">
                          <span className="font-semibold truncate font-mono">{file.path}</span>
                          <span
                            className={`px-1.5 py-0.2 rounded text-[9px] font-semibold uppercase ${
                              file.priority === 'mandatory'
                                ? 'bg-rose-500/20 text-rose-300'
                                : 'bg-slate-700/50 text-slate-400'
                            }`}
                          >
                            {file.priority}
                          </span>
                        </div>
                        <p className="text-[11px] text-slate-400 truncate mt-0.5">{file.title}</p>
                      </div>
                    </button>
                  );
                })}
              </div>
            </div>

            {/* Right Column: File Preview, Details & Workspace Actions */}
            <div className="flex-1 flex flex-col bg-slate-950 overflow-hidden">
              {/* File Meta Header */}
              <div className="p-4 border-b border-white/10 bg-slate-900/60 flex flex-col md:flex-row md:items-center justify-between gap-3">
                <div>
                  <div className="flex items-center gap-2 flex-wrap">
                    <span className="font-mono text-sm font-bold text-white">{activeFile.path}</span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
                      v{activeFile.version}
                    </span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-emerald-500/20 text-emerald-300">
                      {activeFile.status}
                    </span>
                    <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-slate-800 text-slate-300">
                      Owner: {activeFile.owner}
                    </span>
                  </div>
                  <p className="text-xs text-slate-400 mt-1">{activeFile.summary}</p>
                </div>

                <div className="flex items-center gap-2">
                  <button
                    onClick={() => handleCopy(activeFile.content, activeFile.path)}
                    className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-white/5 hover:bg-white/10 text-slate-200 border border-white/10 transition-all cursor-pointer"
                  >
                    {copiedPath === activeFile.path ? (
                      <Check className="w-3.5 h-3.5 text-emerald-400" />
                    ) : (
                      <Copy className="w-3.5 h-3.5 text-slate-400" />
                    )}
                    <span>{copiedPath === activeFile.path ? 'Kopiert' : 'Kopieren'}</span>
                  </button>

                  <button
                    onClick={() => handleSaveFile(activeFile)}
                    className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white transition-all cursor-pointer"
                  >
                    <Download className="w-3.5 h-3.5" />
                    <span>In Workspace speichern</span>
                  </button>
                </div>
              </div>

              {/* File Content Display */}
              <div className="flex-1 overflow-y-auto p-4 font-mono text-xs text-slate-300 leading-relaxed bg-slate-950 select-text">
                <pre className="whitespace-pre-wrap">{activeFile.content}</pre>
              </div>
            </div>
          </>
        )}

        {/* VIEW 2: STANDARDS & PROTOCOLS REGISTRY (14 Formal Chapters) */}
        {activeTab === 'standards' && (
          <div className="flex-1 flex flex-col md:flex-row overflow-hidden">
            {/* Standards List */}
            <div className="w-full md:w-80 border-r border-white/10 bg-slate-900/40 p-3 overflow-y-auto space-y-1.5">
              <div className="text-xs font-semibold text-slate-400 mb-2 uppercase tracking-wider">
                ATC Protocol Standards (ATC-0001 - ATC-0094)
              </div>
              {ATC_PROTOCOL_STANDARDS.map(std => {
                const isSelected = selectedStandardCode === std.code;
                return (
                  <button
                    key={std.code}
                    onClick={() => setSelectedStandardCode(std.code)}
                    className={`w-full text-left p-2.5 rounded-lg text-xs transition-all border ${
                      isSelected
                        ? 'bg-emerald-500/20 text-white border-emerald-500/40 shadow-sm'
                        : 'text-slate-300 hover:bg-white/5 border-transparent'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-mono font-bold text-emerald-400">{std.code}</span>
                      <span className="text-[10px] px-1.5 py-0.5 rounded bg-white/10 text-slate-300">
                        {std.status}
                      </span>
                    </div>
                    <div className="font-semibold text-slate-200 mt-1 truncate">{std.name}</div>
                    <div className="text-[11px] text-slate-400 mt-0.5">Kategorie: {std.category}</div>
                  </button>
                );
              })}
            </div>

            {/* Standard Detail: 14 Formal Chapters */}
            <div className="flex-1 overflow-y-auto p-6 space-y-6">
              {(() => {
                const currentStd =
                  ATC_PROTOCOL_STANDARDS.find(s => s.code === selectedStandardCode) ||
                  ATC_PROTOCOL_STANDARDS[0];

                return (
                  <div className="space-y-6 max-w-4xl mx-auto">
                    {/* Header */}
                    <div className="p-5 rounded-xl bg-slate-900 border border-emerald-500/30">
                      <div className="flex items-center justify-between gap-4 flex-wrap">
                        <div>
                          <div className="flex items-center gap-2">
                            <span className="font-mono font-bold text-lg text-emerald-400">
                              {currentStd.code}
                            </span>
                            <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-emerald-500/20 text-emerald-300">
                              Version {currentStd.version}
                            </span>
                            <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-indigo-500/20 text-indigo-300">
                              {currentStd.category}
                            </span>
                          </div>
                          <h1 className="text-xl font-bold text-white mt-1">{currentStd.name}</h1>
                          <p className="text-xs text-slate-400 mt-1">
                            Authors: {currentStd.authors.join(', ')}
                          </p>
                        </div>

                        <button
                          onClick={() => {
                            if (!onSaveWorkspaceFile) return;
                            const path = `docs/standards/${currentStd.code}-${currentStd.name.replace(/[^a-zA-Z0-9]/g, '_').toUpperCase()}.md`;
                            const md = `# ${currentStd.code} - ${currentStd.name}\n\nStatus: ${currentStd.status}\nVersion: ${currentStd.version}\nAuthors: ${currentStd.authors.join(', ')}\n\n## Abstract\n${currentStd.sections.abstract}\n\n## Motivation\n${currentStd.sections.motivation}\n\n## Specification\n${currentStd.sections.specification}\n\n## Terminology\n${currentStd.sections.terminology}\n\n## Data Structures\n\`\`\`rust\n${currentStd.sections.dataStructures}\n\`\`\`\n\n## Encoding\n${currentStd.sections.encoding}\n\n## State Transitions\n${currentStd.sections.stateTransitions}\n\n## Validation Rules\n${currentStd.sections.validationRules}\n\n## Error Conditions\n${currentStd.sections.errorConditions}\n\n## Security Considerations\n${currentStd.sections.securityConsiderations}\n\n## Compatibility\n${currentStd.sections.compatibility}\n\n## Test Vectors\n${currentStd.sections.testVectors}\n\n## Reference Implementation\n${currentStd.sections.referenceImplementation}\n\n## Changelog\n${currentStd.sections.changelog}\n`;
                            onSaveWorkspaceFile(path, md);
                            if (onShowToast) onShowToast(`'${path}' im Workspace gespeichert!`, 'info');
                          }}
                          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-emerald-600 hover:bg-emerald-500 text-white cursor-pointer transition-all"
                        >
                          <Download className="w-3.5 h-3.5" />
                          <span>Standard in Workspace speichern</span>
                        </button>
                      </div>
                    </div>

                    {/* The 14 Standard Chapters */}
                    <div className="grid grid-cols-1 gap-4">
                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-indigo-400">1. Abstract</div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.abstract}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-indigo-400">2. Motivation</div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.motivation}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-emerald-400">3. Specification</div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.specification}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-indigo-400">4. Terminology</div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.terminology}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-amber-400">5. Data Structures</div>
                        <pre className="p-3 rounded bg-black/50 font-mono text-xs text-amber-300 overflow-x-auto">
                          {currentStd.sections.dataStructures}
                        </pre>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-indigo-400">6. Encoding</div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.encoding}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-cyan-400">7. State Transitions</div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.stateTransitions}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-emerald-400">8. Validation Rules</div>
                        <p className="text-xs text-slate-300 leading-relaxed whitespace-pre-line">{currentStd.sections.validationRules}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-rose-400">9. Error Conditions</div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.errorConditions}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-rose-400 flex items-center gap-1.5">
                          <ShieldAlert className="w-4 h-4" />
                          <span>10. Security Considerations</span>
                        </div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.securityConsiderations}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-indigo-400">11. Compatibility</div>
                        <p className="text-xs text-slate-300 leading-relaxed">{currentStd.sections.compatibility}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-teal-400">12. Test Vectors</div>
                        <pre className="p-3 rounded bg-black/50 font-mono text-xs text-teal-300 overflow-x-auto">
                          {currentStd.sections.testVectors}
                        </pre>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-slate-400">13. Reference Implementation</div>
                        <p className="text-xs text-slate-300 font-mono">{currentStd.sections.referenceImplementation}</p>
                      </div>

                      <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-slate-400">14. Changelog</div>
                        <p className="text-xs text-slate-300 whitespace-pre-line">{currentStd.sections.changelog}</p>
                      </div>
                    </div>
                  </div>
                );
              })()}
            </div>
          </div>
        )}

        {/* VIEW 3: ARCHITECTURE DECISION RECORDS (ADRs) */}
        {activeTab === 'adr' && (
          <div className="flex-1 flex flex-col md:flex-row overflow-hidden">
            {/* ADR List */}
            <div className="w-full md:w-80 border-r border-white/10 bg-slate-900/40 p-3 overflow-y-auto space-y-1.5">
              <div className="text-xs font-semibold text-slate-400 mb-2 uppercase tracking-wider">
                Architecture Decision Records
              </div>
              {ADR_REGISTRY.map(adr => {
                const isSelected = selectedAdrId === adr.id;
                return (
                  <button
                    key={adr.id}
                    onClick={() => setSelectedAdrId(adr.id)}
                    className={`w-full text-left p-2.5 rounded-lg text-xs transition-all border ${
                      isSelected
                        ? 'bg-blue-500/20 text-white border-blue-500/40 shadow-sm'
                        : 'text-slate-300 hover:bg-white/5 border-transparent'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-mono font-bold text-blue-400">{adr.id}</span>
                      <span className="text-[10px] px-1.5 py-0.5 rounded bg-emerald-500/20 text-emerald-300">
                        {adr.status}
                      </span>
                    </div>
                    <div className="font-semibold text-slate-200 mt-1 line-clamp-2">{adr.title}</div>
                    <div className="text-[11px] text-slate-400 mt-0.5">{adr.date}</div>
                  </button>
                );
              })}
            </div>

            {/* ADR Detail */}
            <div className="flex-1 overflow-y-auto p-6">
              {(() => {
                const currentAdr = ADR_REGISTRY.find(a => a.id === selectedAdrId) || ADR_REGISTRY[0];
                return (
                  <div className="space-y-6 max-w-4xl mx-auto">
                    <div className="p-5 rounded-xl bg-slate-900 border border-blue-500/30">
                      <div className="flex items-center justify-between gap-4 flex-wrap">
                        <div>
                          <div className="flex items-center gap-2">
                            <span className="font-mono font-bold text-blue-400 text-lg">{currentAdr.id}</span>
                            <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-emerald-500/20 text-emerald-300">
                              {currentAdr.status}
                            </span>
                            <span className="text-xs text-slate-400">{currentAdr.date}</span>
                          </div>
                          <h1 className="text-xl font-bold text-white mt-1">{currentAdr.title}</h1>
                          <p className="text-xs text-slate-400 mt-1">Author: {currentAdr.author}</p>
                        </div>

                        <button
                          onClick={() => {
                            if (!onSaveWorkspaceFile) return;
                            const path = `docs/adr/${currentAdr.id.toLowerCase()}-${currentAdr.title.toLowerCase().replace(/[^a-z0-9]/g, '-')}.md`;
                            const content = `# ${currentAdr.id}: ${currentAdr.title}\n\n## Status\n${currentAdr.status}\n\n## Date\n${currentAdr.date}\n\n## Author\n${currentAdr.author}\n\n## Context\n${currentAdr.context}\n\n## Decision\n${currentAdr.decision}\n\n## Alternatives Considered\n${currentAdr.alternatives.map(a => `- ${a}`).join('\n')}\n\n## Consequences (Positive)\n${currentAdr.consequences.positive.map(p => `- ${p}`).join('\n')}\n\n## Consequences (Negative)\n${currentAdr.consequences.negative.map(n => `- ${n}`).join('\n')}\n`;
                            onSaveWorkspaceFile(path, content);
                            if (onShowToast) onShowToast(`'${path}' im Workspace angelegt!`, 'info');
                          }}
                          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-blue-600 hover:bg-blue-500 text-white cursor-pointer transition-all"
                        >
                          <Download className="w-3.5 h-3.5" />
                          <span>In docs/adr speichern</span>
                        </button>
                      </div>
                    </div>

                    <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                      <div className="text-xs font-bold uppercase tracking-wider text-slate-400">Context & Problem Statement</div>
                      <p className="text-xs text-slate-200 leading-relaxed">{currentAdr.context}</p>
                    </div>

                    <div className="p-4 rounded-xl bg-blue-950/30 border border-blue-500/20 space-y-2">
                      <div className="text-xs font-bold uppercase tracking-wider text-blue-400">Decision</div>
                      <p className="text-xs text-slate-200 leading-relaxed font-semibold">{currentAdr.decision}</p>
                    </div>

                    <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                      <div className="text-xs font-bold uppercase tracking-wider text-amber-400">Alternatives Evaluated & Why Rejected</div>
                      <ul className="space-y-1.5">
                        {currentAdr.alternatives.map((alt, i) => (
                          <li key={i} className="text-xs text-slate-300 flex items-start gap-2">
                            <span className="text-amber-400 font-bold">•</span>
                            <span>{alt}</span>
                          </li>
                        ))}
                      </ul>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <div className="p-4 rounded-xl bg-emerald-950/20 border border-emerald-500/20 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-emerald-400 flex items-center gap-1">
                          <CheckCircle2 className="w-3.5 h-3.5" />
                          <span>Positive Consequences</span>
                        </div>
                        <ul className="space-y-1.5">
                          {currentAdr.consequences.positive.map((pos, i) => (
                            <li key={i} className="text-xs text-emerald-200/90 flex items-start gap-2">
                              <span className="text-emerald-400 font-bold">+</span>
                              <span>{pos}</span>
                            </li>
                          ))}
                        </ul>
                      </div>

                      <div className="p-4 rounded-xl bg-rose-950/20 border border-rose-500/20 space-y-2">
                        <div className="text-xs font-bold uppercase tracking-wider text-rose-400 flex items-center gap-1">
                          <AlertTriangle className="w-3.5 h-3.5" />
                          <span>Negative Trade-offs</span>
                        </div>
                        <ul className="space-y-1.5">
                          {currentAdr.consequences.negative.map((neg, i) => (
                            <li key={i} className="text-xs text-rose-200/90 flex items-start gap-2">
                              <span className="text-rose-400 font-bold">-</span>
                              <span>{neg}</span>
                            </li>
                          ))}
                        </ul>
                      </div>
                    </div>
                  </div>
                );
              })()}
            </div>
          </div>
        )}

        {/* VIEW 4: ATC-DOC META-STANDARDS (ATC-DOC-001 to ATC-DOC-008) */}
        {activeTab === 'atc-doc' && (
          <div className="flex-1 flex flex-col md:flex-row overflow-hidden">
            {/* Meta-Standards List */}
            <div className="w-full md:w-80 border-r border-white/10 bg-slate-900/40 p-3 overflow-y-auto space-y-1.5">
              <div className="text-xs font-semibold text-slate-400 mb-2 uppercase tracking-wider">
                Engineering Documentation Standard Family
              </div>
              {ATC_DOC_STANDARDS.map(meta => {
                const isSelected = selectedAtcDocId === meta.id;
                return (
                  <button
                    key={meta.id}
                    onClick={() => setSelectedAtcDocId(meta.id)}
                    className={`w-full text-left p-2.5 rounded-lg text-xs transition-all border ${
                      isSelected
                        ? 'bg-fuchsia-500/20 text-white border-fuchsia-500/40 shadow-sm'
                        : 'text-slate-300 hover:bg-white/5 border-transparent'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-mono font-bold text-fuchsia-400">{meta.id}</span>
                      <span className="text-[10px] px-1.5 py-0.5 rounded bg-white/10 text-slate-300">
                        {meta.category}
                      </span>
                    </div>
                    <div className="font-semibold text-slate-200 mt-1 line-clamp-2">{meta.title}</div>
                  </button>
                );
              })}
            </div>

            {/* Meta-Standard Detail */}
            <div className="flex-1 overflow-y-auto p-6">
              {(() => {
                const currentDoc =
                  ATC_DOC_STANDARDS.find(d => d.id === selectedAtcDocId) || ATC_DOC_STANDARDS[0];
                return (
                  <div className="space-y-6 max-w-4xl mx-auto">
                    <div className="p-5 rounded-xl bg-slate-900 border border-fuchsia-500/30">
                      <div className="flex items-center justify-between gap-4 flex-wrap">
                        <div>
                          <div className="flex items-center gap-2">
                            <span className="font-mono font-bold text-fuchsia-400 text-lg">
                              {currentDoc.id}
                            </span>
                            <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-emerald-500/20 text-emerald-300">
                              {currentDoc.status}
                            </span>
                            <span className="px-2 py-0.5 rounded text-[10px] font-semibold bg-fuchsia-500/20 text-fuchsia-300">
                              {currentDoc.category}
                            </span>
                          </div>
                          <h1 className="text-xl font-bold text-white mt-1">{currentDoc.title}</h1>
                          <p className="text-xs text-slate-400 mt-1">Version: {currentDoc.version}</p>
                        </div>

                        <button
                          onClick={() => {
                            if (!onSaveWorkspaceFile) return;
                            const path = `docs/standards/${currentDoc.id}.md`;
                            const content = `# ${currentDoc.id}: ${currentDoc.title}\n\nStatus: ${currentDoc.status}\nVersion: ${currentDoc.version}\nCategory: ${currentDoc.category}\n\n## Abstract\n${currentDoc.abstract}\n\n## Mandatory Invariants\n${currentDoc.rules.map((r, i) => `${i + 1}. ${r}`).join('\n')}\n\n## Canonical Template Skeleton\n\`\`\`markdown\n${currentDoc.templateSkeleton}\n\`\`\`\n`;
                            onSaveWorkspaceFile(path, content);
                            if (onShowToast) onShowToast(`'${path}' im Workspace gespeichert!`, 'info');
                          }}
                          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-fuchsia-600 hover:bg-fuchsia-500 text-white cursor-pointer transition-all"
                        >
                          <Download className="w-3.5 h-3.5" />
                          <span>Standard in Workspace speichern</span>
                        </button>
                      </div>
                    </div>

                    <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                      <div className="text-xs font-bold uppercase tracking-wider text-slate-400">Abstract</div>
                      <p className="text-xs text-slate-200 leading-relaxed">{currentDoc.abstract}</p>
                    </div>

                    <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                      <div className="text-xs font-bold uppercase tracking-wider text-emerald-400">Mandatory Rules & Governance Checklist</div>
                      <ul className="space-y-2">
                        {currentDoc.rules.map((r, i) => (
                          <li key={i} className="text-xs text-slate-200 flex items-start gap-2.5">
                            <span className="w-5 h-5 rounded bg-emerald-500/20 text-emerald-300 font-bold flex items-center justify-center text-[11px] shrink-0 mt-0.5">
                              {i + 1}
                            </span>
                            <span className="leading-relaxed">{r}</span>
                          </li>
                        ))}
                      </ul>
                    </div>

                    <div className="p-4 rounded-xl bg-slate-900/60 border border-white/10 space-y-2">
                      <div className="text-xs font-bold uppercase tracking-wider text-fuchsia-400">Canonical Document Template Skeleton</div>
                      <pre className="p-3 rounded bg-black/50 font-mono text-xs text-fuchsia-200 overflow-x-auto whitespace-pre-wrap">
                        {currentDoc.templateSkeleton}
                      </pre>
                    </div>
                  </div>
                );
              })()}
            </div>
          </div>
        )}
      </div>

      {/* FOOTER BAR: QUICK STATS & GOVERNANCE COMPLIANCE */}
      <div className="p-2.5 px-4 border-t border-white/10 bg-slate-900/90 text-xs flex flex-col md:flex-row md:items-center justify-between gap-2 text-slate-400">
        <div className="flex items-center gap-4 flex-wrap">
          <div className="flex items-center gap-1.5">
            <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
            <span>Root-Pflicht: <strong>10/10 Dateien</strong></span>
          </div>
          <div className="flex items-center gap-1.5">
            <Cpu className="w-3.5 h-3.5 text-purple-400" />
            <span>ATC-VM Modular: <strong>15 Spezifikationen</strong></span>
          </div>
          <div className="flex items-center gap-1.5">
            <Boxes className="w-3.5 h-3.5 text-orange-400" />
            <span>Blockchain: <strong>14 Spezifikationen</strong></span>
          </div>
          <div className="flex items-center gap-1.5">
            <Scale className="w-3.5 h-3.5 text-blue-400" />
            <span>ADRs: <strong>6 Akzeptiert</strong></span>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <span className="text-[11px] text-slate-500">Multi-Repo Documentation Standard:</span>
          <span className="font-mono text-indigo-400 font-semibold">ATC-DOC-001 - 008</span>
        </div>
      </div>
    </div>
  );
}
