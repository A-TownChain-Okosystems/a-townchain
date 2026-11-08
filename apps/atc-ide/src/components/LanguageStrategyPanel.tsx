import React, { useState, useMemo } from 'react';
import {
  Code2,
  Cpu,
  Layers,
  Terminal,
  Server,
  Zap,
  Boxes,
  FileCode,
  Sparkles,
  Workflow,
  Database,
  Search,
  Download,
  Copy,
  Check,
  BookOpen,
  Binary,
  ArrowRight,
  Shield,
  Gauge,
  Laptop,
  CheckCircle2,
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import {
  LANGUAGE_PROFILES,
  STRATEGIC_CORE_6,
  COMPONENT_LANGUAGE_MATRIX,
  LANGUAGE_STRATEGY_MARKDOWN,
  LanguageProfile,
} from '../data/ecosystemLanguageStrategy';

interface LanguageStrategyPanelProps {
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export function LanguageStrategyPanel({
  onSaveWorkspaceFile,
  onShowToast,
}: LanguageStrategyPanelProps) {
  const [activeTab, setActiveTab] = useState<'overview' | 'matrix' | 'components' | 'decisions' | 'doc'>('overview');
  const [categoryFilter, setCategoryFilter] = useState<string>('All');
  const [roleFilter, setRoleFilter] = useState<string>('All');
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [selectedLanguage, setSelectedLanguage] = useState<LanguageProfile>(LANGUAGE_PROFILES[0]);
  const [copied, setCopied] = useState<boolean>(false);
  const [componentCategoryFilter, setComponentCategoryFilter] = useState<string>('All');

  // Filtered languages
  const filteredLanguages = useMemo(() => {
    return LANGUAGE_PROFILES.filter((lang) => {
      const matchesCategory = categoryFilter === 'All' || lang.category === categoryFilter;
      const matchesRole = roleFilter === 'All' || lang.ecosystemRole === roleFilter;
      const matchesSearch =
        lang.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        lang.bestFor.toLowerCase().includes(searchQuery.toLowerCase()) ||
        lang.strengths.some((s) => s.toLowerCase().includes(searchQuery.toLowerCase())) ||
        lang.weaknesses.some((w) => w.toLowerCase().includes(searchQuery.toLowerCase()));
      return matchesCategory && matchesRole && matchesSearch;
    });
  }, [categoryFilter, roleFilter, searchQuery]);

  // Filtered components
  const filteredComponents = useMemo(() => {
    return COMPONENT_LANGUAGE_MATRIX.filter((comp) => {
      if (componentCategoryFilter === 'All') return true;
      return comp.category === componentCategoryFilter;
    });
  }, [componentCategoryFilter]);

  const handleCopyDoc = () => {
    navigator.clipboard.writeText(LANGUAGE_STRATEGY_MARKDOWN);
    setCopied(true);
    if (onShowToast) onShowToast('Sprachstrategie-Dokument in die Zwischenablage kopiert!', 'info');
    setTimeout(() => setCopied(false), 2000);
  };

  const handleSaveToWorkspace = () => {
    if (onSaveWorkspaceFile) {
      onSaveWorkspaceFile('LANGUAGE_STRATEGY.md', LANGUAGE_STRATEGY_MARKDOWN);
      if (onShowToast) onShowToast("'LANGUAGE_STRATEGY.md' erfolgreich im Workspace gespeichert!", 'info');
    }
  };

  const categories = ['All', 'Core Systems', 'Engine & Graphics', 'Managed & Apps', 'AI & Automation', 'Web & Cloud', 'Smart Contracts & Data'];

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-100 overflow-hidden select-none">
      {/* Top Banner Header */}
      <div className="px-6 py-4 border-b border-slate-800 bg-gradient-to-r from-slate-900 via-indigo-950/40 to-slate-900 flex flex-col md:flex-row md:items-center justify-between gap-4 shrink-0">
        <div>
          <div className="flex items-center gap-2.5">
            <span className="p-2 rounded-lg bg-indigo-500/20 border border-indigo-500/40 text-indigo-300">
              <Code2 className="w-5 h-5" />
            </span>
            <div>
              <h2 className="text-lg font-bold text-white tracking-wide flex items-center gap-2">
                Ökosystem-Sprachstrategie & System-Matrix
                <span className="text-xs px-2 py-0.5 rounded-full bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 font-mono">
                  ATC-DOC-012
                </span>
              </h2>
              <p className="text-xs text-slate-400 mt-0.5">
                Architektonische Sprachauswahl nach Systemebene, Memory-Safety, Performance & Entwicklerproduktivität
              </p>
            </div>
          </div>
        </div>

        {/* Action Buttons */}
        <div className="flex items-center gap-2">
          <button
            onClick={handleCopyDoc}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 transition-colors"
            title="Kopiere die vollständige Sprachstrategie als Markdown"
          >
            {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5 text-slate-400" />}
            <span>{copied ? 'Kopiert!' : 'Kopieren'}</span>
          </button>
          <button
            onClick={handleSaveToWorkspace}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white shadow-sm transition-all"
            title="Speichere LANGUAGE_STRATEGY.md in den Workspace"
          >
            <Download className="w-3.5 h-3.5" />
            <span>In Workspace anlegen</span>
          </button>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="flex items-center gap-1 px-6 py-2.5 bg-slate-900/90 border-b border-slate-800 text-xs font-medium overflow-x-auto shrink-0">
        <button
          onClick={() => setActiveTab('overview')}
          className={`flex items-center gap-2 px-3.5 py-1.5 rounded-md transition-all ${
            activeTab === 'overview'
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 font-bold'
              : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60'
          }`}
        >
          <Sparkles className="w-3.5 h-3.5 text-indigo-400" />
          <span>Die 6 Kern-Sprachen</span>
        </button>

        <button
          onClick={() => setActiveTab('matrix')}
          className={`flex items-center gap-2 px-3.5 py-1.5 rounded-md transition-all ${
            activeTab === 'matrix'
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 font-bold'
              : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60'
          }`}
        >
          <Binary className="w-3.5 h-3.5 text-cyan-400" />
          <span>18-Sprachen Gesamtvergleich</span>
        </button>

        <button
          onClick={() => setActiveTab('components')}
          className={`flex items-center gap-2 px-3.5 py-1.5 rounded-md transition-all ${
            activeTab === 'components'
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 font-bold'
              : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60'
          }`}
        >
          <Layers className="w-3.5 h-3.5 text-emerald-400" />
          <span>25 Systemkomponenten-Zuordnung</span>
        </button>

        <button
          onClick={() => setActiveTab('decisions')}
          className={`flex items-center gap-2 px-3.5 py-1.5 rounded-md transition-all ${
            activeTab === 'decisions'
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 font-bold'
              : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60'
          }`}
        >
          <Workflow className="w-3.5 h-3.5 text-purple-400" />
          <span>Architektur-Entscheidungen</span>
        </button>

        <button
          onClick={() => setActiveTab('doc')}
          className={`flex items-center gap-2 px-3.5 py-1.5 rounded-md transition-all ${
            activeTab === 'doc'
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 font-bold'
              : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60'
          }`}
        >
          <BookOpen className="w-3.5 h-3.5 text-amber-400" />
          <span>Vollständiges Dokument</span>
        </button>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        {/* TAB 1: DIE 6 KERN-SPRACHEN */}
        {activeTab === 'overview' && (
          <div className="space-y-6 max-w-7xl mx-auto">
            {/* Lead Strategy Statement Banner */}
            <div className="p-5 rounded-xl border border-indigo-500/30 bg-gradient-to-br from-indigo-950/40 via-slate-900 to-slate-950 relative overflow-hidden">
              <div className="relative z-10 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div className="space-y-1.5">
                  <div className="inline-flex items-center gap-2 px-2.5 py-0.5 rounded-full bg-indigo-500/20 text-indigo-300 text-xs font-mono font-semibold">
                    <Sparkles className="w-3.5 h-3.5" />
                    STRATEGISCHE KONSOLIDIERUNG
                  </div>
                  <h3 className="text-base md:text-lg font-bold text-white">
                    Kein Wildwuchs aus 15 Sprachen: Die 6 Kern-Sprachen des ATC-Ökosystems
                  </h3>
                  <p className="text-xs text-slate-300 max-w-3xl leading-relaxed">
                    Große Software-Ökosysteme scheitern an babylonischer Sprachverwirrung. Für ShivaCore, Globus OS,
                    ATC-Blockchain, Genesis Engine und Aurora AI gilt daher eine unmissverständliche Zuweisung:
                  </p>
                </div>
              </div>

              {/* Formula Strip */}
              <div className="mt-4 pt-4 border-t border-indigo-500/20 grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2 text-xs font-mono">
                <div className="p-2 rounded bg-orange-950/30 border border-orange-500/20 text-orange-200">
                  <span className="font-bold text-orange-400">1. Rust</span>
                  <div className="text-[11px] text-slate-400 mt-0.5">Plattform & Core</div>
                </div>
                <div className="p-2 rounded bg-blue-950/30 border border-blue-500/20 text-blue-200">
                  <span className="font-bold text-blue-400">2. C++</span>
                  <div className="text-[11px] text-slate-400 mt-0.5">Game Engine</div>
                </div>
                <div className="p-2 rounded bg-cyan-950/30 border border-cyan-500/20 text-cyan-200">
                  <span className="font-bold text-cyan-400">3. TypeScript</span>
                  <div className="text-[11px] text-slate-400 mt-0.5">UI & Control Plane</div>
                </div>
                <div className="p-2 rounded bg-yellow-950/30 border border-yellow-500/20 text-yellow-200">
                  <span className="font-bold text-yellow-400">4. Python</span>
                  <div className="text-[11px] text-slate-400 mt-0.5">AI & Automation</div>
                </div>
                <div className="p-2 rounded bg-sky-950/30 border border-sky-500/20 text-sky-200">
                  <span className="font-bold text-sky-400">5. Go</span>
                  <div className="text-[11px] text-slate-400 mt-0.5">Cloud Services</div>
                </div>
                <div className="p-2 rounded bg-emerald-950/30 border border-emerald-500/20 text-emerald-200">
                  <span className="font-bold text-emerald-400">6. ATCLang</span>
                  <div className="text-[11px] text-slate-400 mt-0.5">ATC Smart Contracts</div>
                </div>
              </div>
            </div>

            {/* 6 Core Pillars Cards Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {STRATEGIC_CORE_6.map((pillar) => (
                <div
                  key={pillar.number}
                  className="rounded-xl border border-slate-800 bg-slate-900/60 p-5 flex flex-col justify-between hover:border-slate-700 transition-all shadow-md group"
                >
                  <div className="space-y-3">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <span className="w-6 h-6 rounded-full bg-slate-800 text-slate-300 font-mono text-xs flex items-center justify-center font-bold">
                          {pillar.number}
                        </span>
                        <h4 className={`text-base font-bold ${pillar.colorClass}`}>{pillar.name}</h4>
                      </div>
                      <span className={`text-[11px] font-mono px-2 py-0.5 rounded border ${pillar.badgeBg}`}>
                        {pillar.role}
                      </span>
                    </div>

                    <div className="p-2.5 rounded-lg bg-slate-950/80 border border-slate-800/80 text-xs italic font-medium text-slate-200">
                      „{pillar.motto}“
                    </div>

                    <p className="text-xs text-slate-400 leading-relaxed">{pillar.why}</p>

                    <div className="pt-2 border-t border-slate-800/60">
                      <div className="text-[11px] uppercase tracking-wider text-slate-500 font-mono font-bold mb-1.5">
                        Zugeordnete Teilsysteme:
                      </div>
                      <ul className="space-y-1">
                        {pillar.domains.map((dom, i) => (
                          <li key={i} className="text-xs text-slate-300 flex items-center gap-1.5">
                            <span className="w-1.5 h-1.5 rounded-full bg-slate-600 group-hover:bg-indigo-400 transition-colors" />
                            <span>{dom}</span>
                          </li>
                        ))}
                      </ul>
                    </div>
                  </div>
                </div>
              ))}
            </div>

            {/* Architecture Hierarchy Diagram */}
            <div className="p-5 rounded-xl border border-slate-800 bg-slate-900/70 space-y-4">
              <h4 className="text-sm font-bold text-white flex items-center gap-2 font-mono">
                <Workflow className="w-4 h-4 text-indigo-400" />
                SYSTEMEBENEN-VERZWEIGUNG NACH SPRACHEN
              </h4>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Globus OS Tree */}
                <div className="p-4 rounded-lg bg-slate-950 border border-slate-800 font-mono text-xs text-slate-300 space-y-1">
                  <div className="text-indigo-400 font-bold mb-2">GLOBUS OS & SHIVACORE</div>
                  <div>GLOBUS OS</div>
                  <div>│</div>
                  <div>├── ShivaCore Kernel       → <span className="text-orange-400 font-bold">Rust</span></div>
                  <div>├── ATC Blockchain Core    → <span className="text-orange-400 font-bold">Rust</span></div>
                  <div>├── ATC-VM (Virtual Mach.) → <span className="text-orange-400 font-bold">Rust</span></div>
                  <div>├── ATCLang Compiler       → <span className="text-orange-400 font-bold">Rust</span></div>
                  <div>├── P2P Network (libp2p)   → <span className="text-orange-400 font-bold">Rust</span></div>
                  <div>├── Crypto & Zero-Knowledge→ <span className="text-orange-400 font-bold">Rust</span></div>
                  <div>└── Storage & State Engine → <span className="text-orange-400 font-bold">Rust</span></div>
                </div>

                {/* Genesis Engine Tree */}
                <div className="p-4 rounded-lg bg-slate-950 border border-slate-800 font-mono text-xs text-slate-300 space-y-1">
                  <div className="text-blue-400 font-bold mb-2">GENESIS ENGINE (3D GAME ENGINE)</div>
                  <div>GENESIS ENGINE</div>
                  <div>│</div>
                  <div>├── Core & Memory Pools    → <span className="text-blue-400 font-bold">C++</span></div>
                  <div>├── Vulkan 1.3 Renderer    → <span className="text-blue-400 font-bold">C++</span></div>
                  <div>├── Rigid Body Physics     → <span className="text-blue-400 font-bold">C++</span></div>
                  <div>├── Spatial Audio DSP      → <span className="text-blue-400 font-bold">C++</span></div>
                  <div>├── World Editor GUI       → <span className="text-violet-400 font-bold">C++ / C#</span></div>
                  <div>├── Gameplay API           → <span className="text-violet-400 font-bold">C#</span></div>
                  <div>└── Game & Asset Scripts   → <span className="text-emerald-400 font-bold">ATCLang / Lua</span></div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* TAB 2: 18-SPRACHEN MATRIX */}
        {activeTab === 'matrix' && (
          <div className="space-y-5 max-w-7xl mx-auto">
            {/* Filter Controls */}
            <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60 flex flex-col md:flex-row md:items-center justify-between gap-4">
              <div className="flex flex-wrap items-center gap-1.5">
                {categories.map((cat) => (
                  <button
                    key={cat}
                    onClick={() => setCategoryFilter(cat)}
                    className={`px-3 py-1 rounded-md text-xs font-medium transition-colors ${
                      categoryFilter === cat
                        ? 'bg-indigo-600 text-white font-bold shadow-sm'
                        : 'bg-slate-800/80 text-slate-400 hover:text-slate-200'
                    }`}
                  >
                    {cat}
                  </button>
                ))}
              </div>

              <div className="relative min-w-[240px]">
                <Search className="w-3.5 h-3.5 absolute left-3 top-1/2 -translate-y-1/2 text-slate-400" />
                <input
                  type="text"
                  placeholder="Sprache, Stärke, Einsatzbereich..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-9 pr-3 py-1.5 text-xs rounded-md bg-slate-950 border border-slate-800 text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500"
                />
              </div>
            </div>

            {/* Split View: List on Left, Deep Dive on Right */}
            <div className="grid grid-cols-1 lg:grid-cols-12 gap-5">
              {/* Table / List (7 cols) */}
              <div className="lg:col-span-7 space-y-2">
                <div className="rounded-xl border border-slate-800 bg-slate-900/60 overflow-hidden">
                  <div className="px-4 py-2.5 bg-slate-900 border-b border-slate-800 grid grid-cols-12 text-[11px] font-mono font-bold text-slate-400">
                    <span className="col-span-3">SPRACHE</span>
                    <span className="col-span-4">BESONDERS GUT FÜR</span>
                    <span className="col-span-3">SYSTEMEBENE</span>
                    <span className="col-span-2 text-right">ROLLE</span>
                  </div>

                  <div className="divide-y divide-slate-800/60 max-h-[580px] overflow-y-auto">
                    {filteredLanguages.map((lang) => {
                      const isSelected = selectedLanguage.id === lang.id;
                      return (
                        <div
                          key={lang.id}
                          onClick={() => setSelectedLanguage(lang)}
                          className={`px-4 py-3 grid grid-cols-12 items-center cursor-pointer transition-colors text-xs ${
                            isSelected
                              ? 'bg-indigo-950/40 border-l-4 border-indigo-500 text-white font-medium'
                              : 'hover:bg-slate-800/40 text-slate-300'
                          }`}
                        >
                          <div className="col-span-3 flex items-center gap-2">
                            <span className="font-bold">{lang.name}</span>
                          </div>
                          <div className="col-span-4 text-slate-400 truncate pr-2" title={lang.bestFor}>
                            {lang.bestFor}
                          </div>
                          <div className="col-span-3">
                            <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-400">
                              {lang.systemTier.split(' ')[0]}
                            </span>
                          </div>
                          <div className="col-span-2 text-right">
                            <span
                              className={`text-[10px] font-mono px-2 py-0.5 rounded font-bold ${
                                lang.ecosystemRole === 'Core 6 Primary'
                                  ? 'bg-orange-500/20 text-orange-300 border border-orange-500/30'
                                  : lang.ecosystemRole === 'Engine Native'
                                  ? 'bg-blue-500/20 text-blue-300 border border-blue-500/30'
                                  : 'bg-slate-800 text-slate-400'
                              }`}
                            >
                              {lang.ecosystemRole.split(' ')[0]}
                            </span>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              </div>

              {/* Detail Profile Card (5 cols) */}
              <div className="lg:col-span-5">
                <div className="rounded-xl border border-slate-800 bg-slate-900/80 p-5 space-y-4 sticky top-6">
                  <div className="flex items-start justify-between">
                    <div>
                      <div className="flex items-center gap-2">
                        <h3 className="text-xl font-bold text-white">{selectedLanguage.name}</h3>
                        <span className="text-xs px-2 py-0.5 rounded bg-indigo-500/20 text-indigo-300 font-mono border border-indigo-500/30">
                          {selectedLanguage.category}
                        </span>
                      </div>
                      <p className="text-xs text-slate-400 mt-1">{selectedLanguage.bestFor}</p>
                    </div>
                  </div>

                  {/* Ratings Metrics */}
                  <div className="grid grid-cols-2 gap-2 p-3 rounded-lg bg-slate-950/80 border border-slate-800">
                    <div>
                      <div className="flex justify-between text-[11px] text-slate-400 mb-1">
                        <span className="flex items-center gap-1"><Shield className="w-3 h-3 text-emerald-400" /> Safety:</span>
                        <span className="font-mono font-bold text-slate-200">{selectedLanguage.safetyRating}/10</span>
                      </div>
                      <div className="h-1.5 rounded-full bg-slate-800 overflow-hidden">
                        <div
                          className="h-full bg-emerald-500 rounded-full"
                          style={{ width: `${selectedLanguage.safetyRating * 10}%` }}
                        />
                      </div>
                    </div>

                    <div>
                      <div className="flex justify-between text-[11px] text-slate-400 mb-1">
                        <span className="flex items-center gap-1"><Gauge className="w-3 h-3 text-amber-400" /> Performance:</span>
                        <span className="font-mono font-bold text-slate-200">{selectedLanguage.performanceRating}/10</span>
                      </div>
                      <div className="h-1.5 rounded-full bg-slate-800 overflow-hidden">
                        <div
                          className="h-full bg-amber-500 rounded-full"
                          style={{ width: `${selectedLanguage.performanceRating * 10}%` }}
                        />
                      </div>
                    </div>

                    <div>
                      <div className="flex justify-between text-[11px] text-slate-400 mb-1">
                        <span className="flex items-center gap-1"><Zap className="w-3 h-3 text-cyan-400" /> Productivity:</span>
                        <span className="font-mono font-bold text-slate-200">{selectedLanguage.productivityRating}/10</span>
                      </div>
                      <div className="h-1.5 rounded-full bg-slate-800 overflow-hidden">
                        <div
                          className="h-full bg-cyan-500 rounded-full"
                          style={{ width: `${selectedLanguage.productivityRating * 10}%` }}
                        />
                      </div>
                    </div>

                    <div>
                      <div className="flex justify-between text-[11px] text-slate-400 mb-1">
                        <span className="flex items-center gap-1"><Boxes className="w-3 h-3 text-indigo-400" /> Tooling:</span>
                        <span className="font-mono font-bold text-slate-200">{selectedLanguage.toolingRating}/10</span>
                      </div>
                      <div className="h-1.5 rounded-full bg-slate-800 overflow-hidden">
                        <div
                          className="h-full bg-indigo-500 rounded-full"
                          style={{ width: `${selectedLanguage.toolingRating * 10}%` }}
                        />
                      </div>
                    </div>
                  </div>

                  {/* Strengths & Weaknesses */}
                  <div className="space-y-3 text-xs">
                    <div>
                      <div className="font-bold text-emerald-400 flex items-center gap-1.5 mb-1.5">
                        <CheckCircle2 className="w-3.5 h-3.5" />
                        Stärken:
                      </div>
                      <ul className="space-y-1 text-slate-300">
                        {selectedLanguage.strengths.map((s, idx) => (
                          <li key={idx} className="flex items-start gap-1.5">
                            <span className="text-emerald-500">•</span>
                            <span>{s}</span>
                          </li>
                        ))}
                      </ul>
                    </div>

                    <div>
                      <div className="font-bold text-rose-400 flex items-center gap-1.5 mb-1.5">
                        <Shield className="w-3.5 h-3.5" />
                        Schwächen / Kompromisse:
                      </div>
                      <ul className="space-y-1 text-slate-300">
                        {selectedLanguage.weaknesses.map((w, idx) => (
                          <li key={idx} className="flex items-start gap-1.5">
                            <span className="text-rose-500">•</span>
                            <span>{w}</span>
                          </li>
                        ))}
                      </ul>
                    </div>

                    <div>
                      <div className="font-bold text-indigo-400 flex items-center gap-1.5 mb-1.5">
                        <Layers className="w-3.5 h-3.5" />
                        Komponenten im Ökosystem:
                      </div>
                      <div className="flex flex-wrap gap-1">
                        {selectedLanguage.ecosystemComponents.map((comp, idx) => (
                          <span
                            key={idx}
                            className="text-[11px] px-2 py-0.5 rounded bg-slate-800 text-slate-300 border border-slate-700"
                          >
                            {comp}
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* TAB 3: 25 SYSTEM-KOMPONENTEN */}
        {activeTab === 'components' && (
          <div className="space-y-5 max-w-7xl mx-auto">
            <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60 flex flex-col md:flex-row md:items-center justify-between gap-4">
              <div>
                <h3 className="text-sm font-bold text-white font-mono">
                  KOMPONENTEN-MATRIX (25 SUBSYSTEME)
                </h3>
                <p className="text-xs text-slate-400">
                  Offizielle Zuweisung primärer und sekundärer Programmiersprachen inklusive architektonischer Begründung
                </p>
              </div>

              <div className="flex flex-wrap gap-1">
                {['All', 'OS & Kernel', 'Blockchain & VM', 'AI & Intelligence', 'Engine & Gaming', 'Web, Cloud & Data'].map(
                  (cat) => (
                    <button
                      key={cat}
                      onClick={() => setComponentCategoryFilter(cat)}
                      className={`px-3 py-1 rounded-md text-xs font-medium transition-colors ${
                        componentCategoryFilter === cat
                          ? 'bg-indigo-600 text-white font-bold'
                          : 'bg-slate-800 text-slate-400 hover:text-slate-200'
                      }`}
                    >
                      {cat}
                    </button>
                  )
                )}
              </div>
            </div>

            <div className="rounded-xl border border-slate-800 bg-slate-900/60 overflow-hidden">
              <div className="divide-y divide-slate-800/80">
                {filteredComponents.map((comp, idx) => (
                  <div key={idx} className="p-4 hover:bg-slate-800/30 transition-colors flex flex-col md:flex-row md:items-center justify-between gap-4">
                    <div className="space-y-1 max-w-xl">
                      <div className="flex items-center gap-2">
                        <span className="text-xs font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-400">
                          {comp.category}
                        </span>
                        <h4 className="text-sm font-bold text-white">{comp.systemComponent}</h4>
                      </div>
                      <p className="text-xs text-slate-400 leading-relaxed">{comp.reasoning}</p>
                    </div>

                    <div className="flex items-center gap-3 shrink-0">
                      <div className="text-right">
                        <div className="text-[10px] text-slate-500 uppercase font-mono">Primär</div>
                        <span className="text-xs font-bold font-mono px-2.5 py-1 rounded bg-orange-500/20 text-orange-300 border border-orange-500/30">
                          {comp.primaryLanguage}
                        </span>
                      </div>
                      {comp.secondaryLanguage && (
                        <div className="text-right">
                          <div className="text-[10px] text-slate-500 uppercase font-mono">Sekundär</div>
                          <span className="text-xs font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-300 border border-slate-700">
                            {comp.secondaryLanguage}
                          </span>
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* TAB 4: ARCHITEKTUR-ENTSCHEIDUNGEN */}
        {activeTab === 'decisions' && (
          <div className="space-y-6 max-w-7xl mx-auto">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-5">
              {/* Rust vs. C++ */}
              <div className="p-5 rounded-xl border border-slate-800 bg-slate-900/70 space-y-3">
                <div className="flex items-center justify-between">
                  <h4 className="text-sm font-bold text-white font-mono flex items-center gap-2">
                    <Workflow className="w-4 h-4 text-indigo-400" />
                    Rust vs. C++
                  </h4>
                  <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-400">
                    ADR-0007-A
                  </span>
                </div>
                <p className="text-xs text-slate-300 leading-relaxed">
                  <strong>Entscheidung:</strong> Rust für ShivaCore Kernel & ATC-VM; C++ für Genesis Game Engine Core.
                </p>
                <div className="p-3 rounded bg-slate-950 border border-slate-800 text-xs space-y-1 text-slate-400">
                  <div className="text-orange-400 font-bold">Warum nicht alles in C++?</div>
                  <div>
                    70% aller Sicherheitslücken in C/C++ rühren von Speicherfehlern (Use-After-Free, Buffer Overflows).
                    Im Blockchain- & Kernel-Bereich ist ein solcher Bug fatal (Geldverlust, Systemabsturz). Rust eliminiert
                    diese Klasse komplett zur Compile-Zeit.
                  </div>
                </div>
              </div>

              {/* Python vs. Rust for AI */}
              <div className="p-5 rounded-xl border border-slate-800 bg-slate-900/70 space-y-3">
                <div className="flex items-center justify-between">
                  <h4 className="text-sm font-bold text-white font-mono flex items-center gap-2">
                    <Workflow className="w-4 h-4 text-indigo-400" />
                    Python vs. Rust (Aurora AI)
                  </h4>
                  <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-400">
                    ADR-0007-B
                  </span>
                </div>
                <p className="text-xs text-slate-300 leading-relaxed">
                  <strong>Entscheidung:</strong> Zweigeteilte AI-Architektur: Python orchestriert, Rust führt aus.
                </p>
                <div className="p-3 rounded bg-slate-950 border border-slate-800 text-xs space-y-1 text-slate-400">
                  <div className="text-yellow-400 font-bold">Die Orchestrierungs-Regel:</div>
                  <div>
                    Python bleibt ungeschlagen bei PyTorch-Trainings-Skripten, Forschungs-Prototypen und Agenten-Toolchains.
                    Sobald ein Modell jedoch in Produktion geht (Echtzeit-Inferenz im Globus OS), wird es via Candle/ONNX
                    in nativer Rust-Runtime kompiliert (10x geringere Latenz, keine Python-Laufzeitumgebung nötig).
                  </div>
                </div>
              </div>

              {/* Go vs. Rust for Cloud */}
              <div className="p-5 rounded-xl border border-slate-800 bg-slate-900/70 space-y-3">
                <div className="flex items-center justify-between">
                  <h4 className="text-sm font-bold text-white font-mono flex items-center gap-2">
                    <Workflow className="w-4 h-4 text-indigo-400" />
                    Go vs. Rust (Cloud Services)
                  </h4>
                  <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-400">
                    ADR-0007-C
                  </span>
                </div>
                <p className="text-xs text-slate-300 leading-relaxed">
                  <strong>Entscheidung:</strong> Rust für State-kritische Knoten; Go für Cloud-Gateways & Kubernetes-Tools.
                </p>
                <div className="p-3 rounded bg-slate-950 border border-slate-800 text-xs space-y-1 text-slate-400">
                  <div className="text-sky-400 font-bold">Pragmatismus-Faktor:</div>
                  <div>
                    Go ermöglicht extrem schnelle Iterationszyklen für Microservices, Envoy-Extensions und Cloud-Deployments.
                    Wenn maximale Latenzkontrolle und Null-GC benötigt werden (Blockchain State Engine), gilt ausnahmslos Rust.
                  </div>
                </div>
              </div>

              {/* ATCLang vs. Solidity */}
              <div className="p-5 rounded-xl border border-slate-800 bg-slate-900/70 space-y-3">
                <div className="flex items-center justify-between">
                  <h4 className="text-sm font-bold text-white font-mono flex items-center gap-2">
                    <Workflow className="w-4 h-4 text-indigo-400" />
                    ATCLang vs. Solidity
                  </h4>
                  <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-400">
                    ADR-0007-D
                  </span>
                </div>
                <p className="text-xs text-slate-300 leading-relaxed">
                  <strong>Entscheidung:</strong> ATCLang als nativer Standard; Solidity nur als Brücken-Subnetz.
                </p>
                <div className="p-3 rounded bg-slate-950 border border-slate-800 text-xs space-y-1 text-slate-400">
                  <div className="text-emerald-400 font-bold">Capability & Asset Security:</div>
                  <div>
                    Solidity leidet unter EVM-Designfehlern (Reentrancy, fragile Call-Semantik, Stack-Limits). ATCLang
                    nutzt Register-Bytecode und ein Capability-System: Tokens sind echte Ressourcen, die nicht versehentlich
                    dupliziert oder entwendet werden können.
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* TAB 5: DOKUMENT */}
        {activeTab === 'doc' && (
          <div className="max-w-5xl mx-auto space-y-4">
            <div className="p-4 rounded-xl border border-slate-800 bg-slate-900/60 flex items-center justify-between">
              <div>
                <h3 className="text-sm font-bold text-white font-mono">LANGUAGE_STRATEGY.md</h3>
                <p className="text-xs text-slate-400">Vollständiges normatives Architektur-Dokument zum Exportieren</p>
              </div>
              <div className="flex items-center gap-2">
                <button
                  onClick={handleCopyDoc}
                  className="px-3 py-1.5 rounded-md text-xs font-semibold bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 flex items-center gap-1.5"
                >
                  {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  <span>{copied ? 'Kopiert' : 'Kopieren'}</span>
                </button>
                <button
                  onClick={handleSaveToWorkspace}
                  className="px-3 py-1.5 rounded-md text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white flex items-center gap-1.5 shadow-sm"
                >
                  <Download className="w-3.5 h-3.5" />
                  <span>Im Workspace speichern</span>
                </button>
              </div>
            </div>

            <pre className="p-6 rounded-xl border border-slate-800 bg-slate-950 font-mono text-xs text-slate-300 leading-relaxed overflow-x-auto whitespace-pre-wrap">
              {LANGUAGE_STRATEGY_MARKDOWN}
            </pre>
          </div>
        )}
      </div>
    </div>
  );
}
