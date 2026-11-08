import React, { useState, useMemo } from 'react';
import {
  RefreshCw,
  CheckCircle2,
  AlertCircle,
  Clock,
  Layers,
  Map,
  CheckSquare,
  Zap,
  BookOpen,
  Tag,
  Copy,
  Check,
  Download,
  Sliders,
  ChevronRight,
  TrendingUp,
  FileText,
  ShieldCheck,
  Sparkles,
  ArrowUpRight,
  X,
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { FileState, runProjectAudit, extractProjectTodos, generateLiveArchitectureDocs, generateArchitectureMarkdown, generateTodoMarkdown } from '../utils/auditEngine';
import {
  AutoSyncConfig,
  AutoSyncPillarId,
  PillarSyncResult,
} from '../types/sync';
import {
  generateLiveRoadmap,
  generateRoadmapMarkdown,
  generateLiveSprints,
  generateSprintsMarkdown,
  generateLiveDocumentation,
  generateDocIndexMarkdown,
  generateApiReferenceMarkdown,
  generateVersionMatrixMarkdown,
  generateChangelogMarkdown,
  extractCurrentVersion,
  bumpVersion,
  syncAllProjectArtifacts,
} from '../utils/projectSyncEngine';

interface AutoSyncHubModalProps {
  isOpen: boolean;
  onClose: () => void;
  files: FileState[];
  setFiles: React.Dispatch<React.SetStateAction<FileState[]>>;
  config: AutoSyncConfig;
  setConfig: React.Dispatch<React.SetStateAction<AutoSyncConfig>>;
  onShowToast: (msg: string, type: 'info' | 'warning' | 'error') => void;
  onJumpToFile?: (fileName: string) => void;
}

export function AutoSyncHubModal({
  isOpen,
  onClose,
  files,
  setFiles,
  config,
  setConfig,
  onShowToast,
  onJumpToFile,
}: AutoSyncHubModalProps) {
  const [activeTab, setActiveTab] = useState<'overview' | 'preview' | 'sprints' | 'roadmap' | 'version'>('overview');
  const [selectedPreviewPillar, setSelectedPreviewPillar] = useState<AutoSyncPillarId>('wiki');
  const [isSyncing, setIsSyncing] = useState(false);
  const [copied, setCopied] = useState(false);
  const [lastResults, setLastResults] = useState<PillarSyncResult[]>([]);

  // Computed dynamic data
  const currentVersion = useMemo(() => extractCurrentVersion(files), [files]);
  const roadmapDoc = useMemo(() => generateLiveRoadmap(files), [files]);
  const sprintDoc = useMemo(() => generateLiveSprints(files), [files]);
  const liveDoc = useMemo(() => generateLiveDocumentation(files), [files]);
  const auditSummary = useMemo(() => runProjectAudit(files), [files]);
  const todos = useMemo(() => extractProjectTodos(files, auditSummary.findings), [files, auditSummary]);

  // Preview generator for selected pillar
  const previewContent = useMemo(() => {
    switch (selectedPreviewPillar) {
      case 'wiki': {
        const arch = generateLiveArchitectureDocs(files, auditSummary);
        return {
          fileName: 'ARCHITECTURE.md',
          content: generateArchitectureMarkdown(arch),
        };
      }
      case 'roadmap':
        return {
          fileName: 'ROADMAP.md',
          content: generateRoadmapMarkdown(roadmapDoc),
        };
      case 'todo':
        return {
          fileName: 'TODO.md',
          content: generateTodoMarkdown(todos),
        };
      case 'sprints':
        return {
          fileName: 'SPRINTS.md',
          content: generateSprintsMarkdown(sprintDoc),
        };
      case 'docs':
        return {
          fileName: 'docs/DOCUMENTATION_INDEX.md',
          content: generateDocIndexMarkdown(liveDoc),
        };
      case 'version':
        return {
          fileName: 'VERSION',
          content: `${currentVersion.fullVersion}\nEmpfohlener Bump: ${currentVersion.recommendedBump.toUpperCase()} (${currentVersion.recommendedReason})\nLetzte Aktion: ${currentVersion.lastBumpReason}\nStand: ${currentVersion.timestamp}`,
        };
    }
  }, [selectedPreviewPillar, files, auditSummary, roadmapDoc, sprintDoc, liveDoc, todos, currentVersion]);

  // Master toggle
  const allEnabled = Object.values(config).every(v => v);
  const handleToggleAll = () => {
    const nextVal = !allEnabled;
    const nextConfig: AutoSyncConfig = {
      autoSyncWiki: nextVal,
      autoSyncRoadmap: nextVal,
      autoSyncTodo: nextVal,
      autoSyncSprints: nextVal,
      autoSyncDocs: nextVal,
      autoSyncVersion: nextVal,
    };
    setConfig(nextConfig);
    onShowToast(
      nextVal ? 'Alle 6 Auto-Sync Artefakte aktiviert' : 'Alle Auto-Sync Artefakte pausiert',
      'info'
    );
  };

  const handleTogglePillar = (pillar: AutoSyncPillarId) => {
    let key: keyof AutoSyncConfig = 'autoSyncWiki';
    if (pillar === 'roadmap') key = 'autoSyncRoadmap';
    else if (pillar === 'todo') key = 'autoSyncTodo';
    else if (pillar === 'sprints') key = 'autoSyncSprints';
    else if (pillar === 'docs') key = 'autoSyncDocs';
    else if (pillar === 'version') key = 'autoSyncVersion';

    const nextConfig = { ...config, [key]: !config[key] };
    setConfig(nextConfig);
    onShowToast(`Auto-Sync für '${pillar}' ${nextConfig[key] ? 'aktiviert' : 'deaktiviert'}`, 'info');
  };

  // Perform full sync now
  const handleSyncAllNow = (bumpType?: 'patch' | 'minor' | 'major' | 'auto') => {
    setIsSyncing(true);
    setTimeout(() => {
      const { updatedFiles, results, newVersionString } = syncAllProjectArtifacts(files, config, {
        versionBumpType: bumpType,
        forceAll: true,
      });
      setFiles(updatedFiles);
      setLastResults(results);
      setIsSyncing(false);
      onShowToast(`Alle 6 Artefakte erfolgreich synchronisiert (v${newVersionString})`, 'info');
    }, 400);
  };

  const handleCopyPreview = () => {
    navigator.clipboard.writeText(previewContent.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
    onShowToast(`Inhalt von ${previewContent.fileName} kopiert`, 'info');
  };

  const handleDownloadPreview = () => {
    const blob = new Blob([previewContent.content], { type: 'text/markdown;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = previewContent.fileName.split('/').pop() || 'document.md';
    link.click();
    URL.revokeObjectURL(url);
    onShowToast(`Datei ${previewContent.fileName} heruntergeladen`, 'info');
  };

  if (!isOpen) return null;

  const activePillarsCount = Object.values(config).filter(Boolean).length;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-md">
      <motion.div
        initial={{ opacity: 0, scale: 0.95, y: 10 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.95, y: 10 }}
        className="relative w-full max-w-5xl h-[88vh] bg-[#0c0d12] border border-cyan-500/30 rounded-2xl shadow-2xl flex flex-col overflow-hidden text-slate-200"
      >
        {/* Top Header */}
        <div className="px-6 py-4 border-b border-white/10 bg-black/40 flex items-center justify-between shrink-0">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-cyan-500/20 via-indigo-500/20 to-purple-500/20 border border-cyan-500/30 flex items-center justify-center">
              <RefreshCw className={`w-5 h-5 text-cyan-400 ${isSyncing ? 'animate-spin' : ''}`} />
            </div>
            <div>
              <div className="flex items-center gap-2">
                <h2 className="text-lg font-bold text-white tracking-tight">
                  Auto-Sync & Artefakt-Zentrale
                </h2>
                <span className="px-2 py-0.5 rounded-full text-xs font-mono font-bold bg-cyan-500/20 text-cyan-300 border border-cyan-500/30">
                  {activePillarsCount}/6 Aktiv
                </span>
                <span className="px-2 py-0.5 rounded-full text-xs font-mono bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
                  v{currentVersion.major}.{currentVersion.minor}.{currentVersion.patch}
                </span>
              </div>
              <p className="text-xs text-slate-400 mt-0.5">
                Automatische Echtzeit-Aktualisierung von Wiki, Roadmap, Todos, Sprints, Dokumentation & Versionierung
              </p>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => handleSyncAllNow()}
              disabled={isSyncing}
              className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 text-white text-xs font-bold rounded-lg shadow-lg shadow-cyan-950/40 transition-all disabled:opacity-50"
            >
              <Zap className="w-4 h-4 text-amber-300" />
              {isSyncing ? 'Synchronisiere...' : 'Jetzt alles synchronisieren'}
            </button>
            <button
              onClick={onClose}
              className="p-2 hover:bg-white/10 rounded-lg text-slate-400 hover:text-white transition-colors"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </div>

        {/* Navigation Tabs */}
        <div className="px-6 border-b border-white/5 bg-black/20 flex items-center justify-between shrink-0">
          <div className="flex items-center gap-1">
            <button
              onClick={() => setActiveTab('overview')}
              className={`flex items-center gap-2 px-4 py-3 text-xs font-bold border-b-2 transition-colors ${
                activeTab === 'overview'
                  ? 'border-cyan-400 text-cyan-400'
                  : 'border-transparent text-slate-400 hover:text-slate-200'
              }`}
            >
              <Sliders className="w-3.5 h-3.5" />
              Status & Schalter
            </button>
            <button
              onClick={() => setActiveTab('preview')}
              className={`flex items-center gap-2 px-4 py-3 text-xs font-bold border-b-2 transition-colors ${
                activeTab === 'preview'
                  ? 'border-cyan-400 text-cyan-400'
                  : 'border-transparent text-slate-400 hover:text-slate-200'
              }`}
            >
              <FileText className="w-3.5 h-3.5" />
              Live Datei-Vorschau
            </button>
            <button
              onClick={() => setActiveTab('roadmap')}
              className={`flex items-center gap-2 px-4 py-3 text-xs font-bold border-b-2 transition-colors ${
                activeTab === 'roadmap'
                  ? 'border-cyan-400 text-cyan-400'
                  : 'border-transparent text-slate-400 hover:text-slate-200'
              }`}
            >
              <Map className="w-3.5 h-3.5" />
              Roadmap ({roadmapDoc.overallProgress}%)
            </button>
            <button
              onClick={() => setActiveTab('sprints')}
              className={`flex items-center gap-2 px-4 py-3 text-xs font-bold border-b-2 transition-colors ${
                activeTab === 'sprints'
                  ? 'border-cyan-400 text-cyan-400'
                  : 'border-transparent text-slate-400 hover:text-slate-200'
              }`}
            >
              <TrendingUp className="w-3.5 h-3.5" />
              Sprint #{sprintDoc.sprintNumber} ({sprintDoc.burndownPercentage}%)
            </button>
            <button
              onClick={() => setActiveTab('version')}
              className={`flex items-center gap-2 px-4 py-3 text-xs font-bold border-b-2 transition-colors ${
                activeTab === 'version'
                  ? 'border-cyan-400 text-cyan-400'
                  : 'border-transparent text-slate-400 hover:text-slate-200'
              }`}
            >
              <Tag className="w-3.5 h-3.5" />
              Version & SemVer
            </button>
          </div>

          {/* Quick Master Switch */}
          <div className="flex items-center gap-2">
            <span className="text-xs text-slate-400">Master-Sync:</span>
            <button
              onClick={handleToggleAll}
              className={`px-3 py-1 rounded text-xs font-bold transition-all border ${
                allEnabled
                  ? 'bg-emerald-500/20 text-emerald-300 border-emerald-500/40'
                  : 'bg-white/5 text-slate-400 border-white/10 hover:bg-white/10'
              }`}
            >
              {allEnabled ? 'Alle 6 Aktiv' : 'Teilweise Pausiert'}
            </button>
          </div>
        </div>

        {/* Modal Body */}
        <div className="flex-1 overflow-y-auto p-6">
          {activeTab === 'overview' && (
            <div className="space-y-6">
              {/* Pillar Grid */}
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {/* 1. Wiki */}
                <div
                  className={`p-4 rounded-xl border transition-all ${
                    config.autoSyncWiki
                      ? 'bg-sky-950/20 border-sky-500/40 shadow-lg shadow-sky-950/20'
                      : 'bg-white/5 border-white/10 opacity-70'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2.5">
                      <div className="p-2 rounded-lg bg-sky-500/20 text-sky-400 border border-sky-500/30">
                        <BookOpen className="w-5 h-5" />
                      </div>
                      <div>
                        <h3 className="text-sm font-bold text-white">Wiki & Architektur</h3>
                        <p className="text-[11px] font-mono text-slate-400">ARCHITECTURE.md</p>
                      </div>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input
                        type="checkbox"
                        checked={config.autoSyncWiki}
                        onChange={() => handleTogglePillar('wiki')}
                        className="sr-only peer"
                      />
                      <div className="w-9 h-5 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-200 after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-sky-500"></div>
                    </label>
                  </div>
                  <p className="text-xs text-slate-300 mt-3 leading-relaxed">
                    Automatisches Generieren des Schichtenmodells, der Modul-Kataloge und des interaktiven ASCII-Architekturdiagramms.
                  </p>
                  <div className="mt-4 pt-3 border-t border-white/5 flex items-center justify-between text-xs text-slate-400">
                    <span>Dateien: <code className="text-sky-300">ARCHITECTURE.md</code></span>
                    <button
                      onClick={() => {
                        setSelectedPreviewPillar('wiki');
                        setActiveTab('preview');
                      }}
                      className="text-sky-400 hover:underline flex items-center gap-1 text-[11px]"
                    >
                      Vorschau <ChevronRight className="w-3 h-3" />
                    </button>
                  </div>
                </div>

                {/* 2. Roadmap */}
                <div
                  className={`p-4 rounded-xl border transition-all ${
                    config.autoSyncRoadmap
                      ? 'bg-indigo-950/20 border-indigo-500/40 shadow-lg shadow-indigo-950/20'
                      : 'bg-white/5 border-white/10 opacity-70'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2.5">
                      <div className="p-2 rounded-lg bg-indigo-500/20 text-indigo-400 border border-indigo-500/30">
                        <Map className="w-5 h-5" />
                      </div>
                      <div>
                        <h3 className="text-sm font-bold text-white">Roadmap & Phasen</h3>
                        <p className="text-[11px] font-mono text-slate-400">ROADMAP.md</p>
                      </div>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input
                        type="checkbox"
                        checked={config.autoSyncRoadmap}
                        onChange={() => handleTogglePillar('roadmap')}
                        className="sr-only peer"
                      />
                      <div className="w-9 h-5 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-200 after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-indigo-500"></div>
                    </label>
                  </div>
                  <p className="text-xs text-slate-300 mt-3 leading-relaxed">
                    Verfolgung von 5 Phasen (Boot, Kernel, Treiber, Genesis GUI, Blockchain) mit automatischem Fortschritt ({roadmapDoc.overallProgress}%).
                  </p>
                  <div className="mt-4 pt-3 border-t border-white/5 flex items-center justify-between text-xs text-slate-400">
                    <span>Status: <strong className="text-indigo-300">{roadmapDoc.overallProgress}% erledigt</strong></span>
                    <button
                      onClick={() => {
                        setSelectedPreviewPillar('roadmap');
                        setActiveTab('preview');
                      }}
                      className="text-indigo-400 hover:underline flex items-center gap-1 text-[11px]"
                    >
                      Vorschau <ChevronRight className="w-3 h-3" />
                    </button>
                  </div>
                </div>

                {/* 3. Todos */}
                <div
                  className={`p-4 rounded-xl border transition-all ${
                    config.autoSyncTodo
                      ? 'bg-amber-950/20 border-amber-500/40 shadow-lg shadow-amber-950/20'
                      : 'bg-white/5 border-white/10 opacity-70'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2.5">
                      <div className="p-2 rounded-lg bg-amber-500/20 text-amber-400 border border-amber-500/30">
                        <CheckSquare className="w-5 h-5" />
                      </div>
                      <div>
                        <h3 className="text-sm font-bold text-white">Todos & Aufgaben</h3>
                        <p className="text-[11px] font-mono text-slate-400">TODO.md</p>
                      </div>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input
                        type="checkbox"
                        checked={config.autoSyncTodo}
                        onChange={() => handleTogglePillar('todo')}
                        className="sr-only peer"
                      />
                      <div className="w-9 h-5 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-200 after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-amber-500"></div>
                    </label>
                  </div>
                  <p className="text-xs text-slate-300 mt-3 leading-relaxed">
                    Extrahiert Aufgaben aus Source-Code-Kommentaren (<code className="text-amber-300">TODO</code>, <code className="text-amber-300">FIXME</code>) und Audit-Befunden mit Checkbox-Status.
                  </p>
                  <div className="mt-4 pt-3 border-t border-white/5 flex items-center justify-between text-xs text-slate-400">
                    <span>Aufgaben: <strong className="text-amber-300">{todos.length} ({todos.filter(t => !t.completed).length} offen)</strong></span>
                    <button
                      onClick={() => {
                        setSelectedPreviewPillar('todo');
                        setActiveTab('preview');
                      }}
                      className="text-amber-400 hover:underline flex items-center gap-1 text-[11px]"
                    >
                      Vorschau <ChevronRight className="w-3 h-3" />
                    </button>
                  </div>
                </div>

                {/* 4. Sprints */}
                <div
                  className={`p-4 rounded-xl border transition-all ${
                    config.autoSyncSprints
                      ? 'bg-fuchsia-950/20 border-fuchsia-500/40 shadow-lg shadow-fuchsia-950/20'
                      : 'bg-white/5 border-white/10 opacity-70'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2.5">
                      <div className="p-2 rounded-lg bg-fuchsia-500/20 text-fuchsia-400 border border-fuchsia-500/30">
                        <TrendingUp className="w-5 h-5" />
                      </div>
                      <div>
                        <h3 className="text-sm font-bold text-white">Sprints & Burndown</h3>
                        <p className="text-[11px] font-mono text-slate-400">SPRINTS.md</p>
                      </div>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input
                        type="checkbox"
                        checked={config.autoSyncSprints}
                        onChange={() => handleTogglePillar('sprints')}
                        className="sr-only peer"
                      />
                      <div className="w-9 h-5 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-200 after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-fuchsia-500"></div>
                    </label>
                  </div>
                  <p className="text-xs text-slate-300 mt-3 leading-relaxed">
                    Sprint #{sprintDoc.sprintNumber} Planung, Story Points ({sprintDoc.completedPoints}/{sprintDoc.totalPoints} SP), Velocity und Backlog.
                  </p>
                  <div className="mt-4 pt-3 border-t border-white/5 flex items-center justify-between text-xs text-slate-400">
                    <span>Burndown: <strong className="text-fuchsia-300">{sprintDoc.burndownPercentage}%</strong></span>
                    <button
                      onClick={() => {
                        setSelectedPreviewPillar('sprints');
                        setActiveTab('preview');
                      }}
                      className="text-fuchsia-400 hover:underline flex items-center gap-1 text-[11px]"
                    >
                      Vorschau <ChevronRight className="w-3 h-3" />
                    </button>
                  </div>
                </div>

                {/* 5. Dokumentation */}
                <div
                  className={`p-4 rounded-xl border transition-all ${
                    config.autoSyncDocs
                      ? 'bg-emerald-950/20 border-emerald-500/40 shadow-lg shadow-emerald-950/20'
                      : 'bg-white/5 border-white/10 opacity-70'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2.5">
                      <div className="p-2 rounded-lg bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
                        <FileText className="w-5 h-5" />
                      </div>
                      <div>
                        <h3 className="text-sm font-bold text-white">Dokumentation & Index</h3>
                        <p className="text-[11px] font-mono text-slate-400">docs/INDEX.md, API.md</p>
                      </div>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input
                        type="checkbox"
                        checked={config.autoSyncDocs}
                        onChange={() => handleTogglePillar('docs')}
                        className="sr-only peer"
                      />
                      <div className="w-9 h-5 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-200 after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-emerald-500"></div>
                    </label>
                  </div>
                  <p className="text-xs text-slate-300 mt-3 leading-relaxed">
                    ATC-DOC Master-Index, automatische API-Referenz ({liveDoc.apiEntries.length} Symbole) und Kompatibilitätsmatrix.
                  </p>
                  <div className="mt-4 pt-3 border-t border-white/5 flex items-center justify-between text-xs text-slate-400">
                    <span>Indexiert: <strong className="text-emerald-300">{liveDoc.indexedFiles.length} Dateien</strong></span>
                    <button
                      onClick={() => {
                        setSelectedPreviewPillar('docs');
                        setActiveTab('preview');
                      }}
                      className="text-emerald-400 hover:underline flex items-center gap-1 text-[11px]"
                    >
                      Vorschau <ChevronRight className="w-3 h-3" />
                    </button>
                  </div>
                </div>

                {/* 6. Version */}
                <div
                  className={`p-4 rounded-xl border transition-all ${
                    config.autoSyncVersion
                      ? 'bg-cyan-950/20 border-cyan-500/40 shadow-lg shadow-cyan-950/20'
                      : 'bg-white/5 border-white/10 opacity-70'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2.5">
                      <div className="p-2 rounded-lg bg-cyan-500/20 text-cyan-400 border border-cyan-500/30">
                        <Tag className="w-5 h-5" />
                      </div>
                      <div>
                        <h3 className="text-sm font-bold text-white">Version & SemVer</h3>
                        <p className="text-[11px] font-mono text-slate-400">VERSION, package.json</p>
                      </div>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                      <input
                        type="checkbox"
                        checked={config.autoSyncVersion}
                        onChange={() => handleTogglePillar('version')}
                        className="sr-only peer"
                      />
                      <div className="w-9 h-5 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-200 after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-cyan-500"></div>
                    </label>
                  </div>
                  <p className="text-xs text-slate-300 mt-3 leading-relaxed">
                    SemVer-Synchronisation, Build-Tracking und automatischer Reversions-Detektor ({currentVersion.recommendedBump.toUpperCase()}).
                  </p>
                  <div className="mt-4 pt-3 border-t border-white/5 flex items-center justify-between text-xs text-slate-400">
                    <span>Build: <strong className="text-cyan-300">#{currentVersion.build}</strong></span>
                    <button
                      onClick={() => {
                        setSelectedPreviewPillar('version');
                        setActiveTab('preview');
                      }}
                      className="text-cyan-400 hover:underline flex items-center gap-1 text-[11px]"
                    >
                      Vorschau <ChevronRight className="w-3 h-3" />
                    </button>
                  </div>
                </div>
              </div>

              {/* Action Banner with Version Bumper */}
              <div className="p-5 rounded-2xl bg-gradient-to-r from-blue-950/30 via-cyan-950/20 to-purple-950/30 border border-cyan-500/30 flex flex-col md:flex-row items-center justify-between gap-4">
                <div>
                  <h4 className="text-sm font-bold text-white flex items-center gap-2">
                    <Sparkles className="w-4 h-4 text-amber-400" />
                    SemVer Versions-Bump & Release-Auslöser
                  </h4>
                  <p className="text-xs text-slate-400 mt-1">
                    Aktuelle Version: <strong className="text-cyan-300 font-mono">v{currentVersion.major}.{currentVersion.minor}.{currentVersion.patch}</strong> (Build #{currentVersion.build}) • Empfehlung: <span className="text-amber-300 font-bold">{currentVersion.recommendedReason}</span>
                  </p>
                </div>

                <div className="flex items-center gap-2 flex-wrap">
                  <button
                    onClick={() => handleSyncAllNow('auto')}
                    className="px-3 py-1.5 bg-cyan-500/20 hover:bg-cyan-500/30 text-cyan-300 border border-cyan-500/40 text-xs font-bold rounded-lg transition-all"
                  >
                    Auto-Bump
                  </button>
                  <button
                    onClick={() => handleSyncAllNow('patch')}
                    className="px-3 py-1.5 bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 text-xs font-bold rounded-lg transition-all"
                  >
                    Patch (+0.0.1)
                  </button>
                  <button
                    onClick={() => handleSyncAllNow('minor')}
                    className="px-3 py-1.5 bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 text-xs font-bold rounded-lg transition-all"
                  >
                    Minor (+0.1.0)
                  </button>
                  <button
                    onClick={() => handleSyncAllNow('major')}
                    className="px-3 py-1.5 bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 text-xs font-bold rounded-lg transition-all"
                  >
                    Major (+1.0.0)
                  </button>
                </div>
              </div>

              {/* Last Sync Results Log */}
              {lastResults.length > 0 && (
                <div className="p-4 rounded-xl bg-black/40 border border-white/10 space-y-2">
                  <h4 className="text-xs font-bold text-slate-300 flex items-center gap-2">
                    <Clock className="w-3.5 h-3.5 text-cyan-400" />
                    Protokoll des letzten Synchronisationszyklus
                  </h4>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-2 text-xs">
                    {lastResults.map((r, i) => (
                      <div key={i} className="flex items-center justify-between p-2 rounded bg-white/5 border border-white/5">
                        <div className="flex items-center gap-2">
                          <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                          <span className="font-bold text-slate-200 uppercase">{r.pillar}</span>
                          <span className="text-slate-400 truncate max-w-[280px]">{r.summary}</span>
                        </div>
                        <span className="text-[10px] text-slate-500 font-mono shrink-0">{r.timestamp}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}

          {activeTab === 'preview' && (
            <div className="h-full flex flex-col space-y-4">
              {/* Preview Pillar Selector */}
              <div className="flex items-center gap-2 flex-wrap">
                {(
                  [
                    { id: 'wiki', label: 'ARCHITECTURE.md', icon: BookOpen },
                    { id: 'roadmap', label: 'ROADMAP.md', icon: Map },
                    { id: 'todo', label: 'TODO.md', icon: CheckSquare },
                    { id: 'sprints', label: 'SPRINTS.md', icon: TrendingUp },
                    { id: 'docs', label: 'DOCUMENTATION_INDEX.md', icon: FileText },
                    { id: 'version', label: 'VERSION', icon: Tag },
                  ] as const
                ).map(p => (
                  <button
                    key={p.id}
                    onClick={() => setSelectedPreviewPillar(p.id)}
                    className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-bold transition-all border ${
                      selectedPreviewPillar === p.id
                        ? 'bg-cyan-500/20 text-cyan-300 border-cyan-500/40'
                        : 'bg-white/5 text-slate-400 border-white/5 hover:bg-white/10'
                    }`}
                  >
                    <p.icon className="w-3.5 h-3.5" />
                    {p.label}
                  </button>
                ))}

                <div className="ml-auto flex items-center gap-2">
                  <button
                    onClick={handleCopyPreview}
                    className="flex items-center gap-1.5 px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-xs font-bold transition-colors"
                  >
                    {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                    {copied ? 'Kopiert' : 'Kopieren'}
                  </button>
                  <button
                    onClick={handleDownloadPreview}
                    className="flex items-center gap-1.5 px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-xs font-bold transition-colors"
                  >
                    <Download className="w-3.5 h-3.5" />
                    Download
                  </button>
                </div>
              </div>

              {/* Code / Markdown Display */}
              <div className="flex-1 min-h-[400px] p-4 rounded-xl bg-black/60 border border-white/10 font-mono text-xs overflow-auto text-slate-300 leading-relaxed">
                <pre className="whitespace-pre-wrap">{previewContent.content}</pre>
              </div>
            </div>
          )}

          {activeTab === 'roadmap' && (
            <div className="space-y-6">
              <div className="p-4 rounded-xl bg-indigo-950/20 border border-indigo-500/30 flex items-center justify-between">
                <div>
                  <h3 className="text-sm font-bold text-white">Roadmap Gesamtfortschritt: {roadmapDoc.overallProgress}%</h3>
                  <p className="text-xs text-slate-400 mt-0.5">
                    Aktiver Fokus: {roadmapDoc.activeMilestone} • Release-Ziel: {roadmapDoc.estimatedReleaseDate}
                  </p>
                </div>
                <div className="w-48 bg-black/40 h-3 rounded-full overflow-hidden border border-white/10">
                  <div
                    className="h-full bg-gradient-to-r from-indigo-500 to-cyan-400"
                    style={{ width: `${roadmapDoc.overallProgress}%` }}
                  ></div>
                </div>
              </div>

              <div className="space-y-4">
                {roadmapDoc.phases.map((phase) => (
                  <div key={phase.id} className="p-4 rounded-xl bg-black/30 border border-white/10 space-y-3">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <span className="px-2 py-0.5 rounded text-[10px] font-mono bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
                          {phase.badge}
                        </span>
                        <h4 className="text-sm font-bold text-white">{phase.title}</h4>
                      </div>
                      <span className="text-xs font-bold text-cyan-400">{phase.progress}%</span>
                    </div>
                    <p className="text-xs text-slate-400">{phase.description}</p>

                    <div className="space-y-2 pt-2 border-t border-white/5">
                      {phase.milestones.map(m => (
                        <div key={m.id} className="flex items-start gap-2 text-xs">
                          {m.completed ? (
                            <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0 mt-0.5" />
                          ) : (
                            <Clock className="w-4 h-4 text-slate-500 shrink-0 mt-0.5" />
                          )}
                          <div className="flex-1">
                            <span className={m.completed ? 'text-slate-300 font-semibold' : 'text-slate-400'}>
                              {m.title}
                            </span>
                            <span className="ml-2 text-[10px] text-indigo-400 font-mono">({m.targetDate})</span>
                            <p className="text-[11px] text-slate-500">{m.details}</p>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {activeTab === 'sprints' && (
            <div className="space-y-6">
              <div className="p-4 rounded-xl bg-fuchsia-950/20 border border-fuchsia-500/30 flex items-center justify-between">
                <div>
                  <h3 className="text-sm font-bold text-white">
                    Sprint #{sprintDoc.sprintNumber}: {sprintDoc.sprintName}
                  </h3>
                  <p className="text-xs text-slate-400 mt-0.5">
                    Ziel: {sprintDoc.goal} • Burndown: {sprintDoc.burndownPercentage}% ({sprintDoc.completedPoints}/{sprintDoc.totalPoints} SP)
                  </p>
                </div>
                <div className="w-48 bg-black/40 h-3 rounded-full overflow-hidden border border-white/10">
                  <div
                    className="h-full bg-gradient-to-r from-fuchsia-500 to-purple-400"
                    style={{ width: `${sprintDoc.burndownPercentage}%` }}
                  ></div>
                </div>
              </div>

              {/* Sprint Stories Table */}
              <div className="rounded-xl border border-white/10 overflow-hidden bg-black/30">
                <table className="w-full text-left text-xs">
                  <thead className="bg-white/5 text-slate-400 border-b border-white/10">
                    <tr>
                      <th className="p-3">ID</th>
                      <th className="p-3">Story / Task</th>
                      <th className="p-3">Punkte</th>
                      <th className="p-3">Priorität</th>
                      <th className="p-3">Komponente</th>
                      <th className="p-3">Status</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5 text-slate-300">
                    {sprintDoc.stories.map(s => (
                      <tr key={s.id} className="hover:bg-white/5">
                        <td className="p-3 font-mono text-cyan-400 font-bold">{s.id}</td>
                        <td className="p-3 font-medium text-white">{s.title}</td>
                        <td className="p-3 font-mono font-bold text-amber-300">{s.storyPoints} SP</td>
                        <td className="p-3 capitalize">{s.priority}</td>
                        <td className="p-3 text-slate-400">{s.component}</td>
                        <td className="p-3">
                          <span
                            className={`px-2 py-0.5 rounded text-[10px] font-bold ${
                              s.status === 'done'
                                ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
                                : s.status === 'in_progress'
                                ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30'
                                : 'bg-slate-500/20 text-slate-300 border border-slate-500/30'
                            }`}
                          >
                            {s.status}
                          </span>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {activeTab === 'version' && (
            <div className="space-y-6">
              <div className="p-6 rounded-2xl bg-black/40 border border-white/10 space-y-4">
                <div className="flex items-center justify-between">
                  <div>
                    <span className="text-xs text-slate-400">Aktuelle Semantic Version</span>
                    <h3 className="text-2xl font-bold font-mono text-cyan-400">
                      v{currentVersion.major}.{currentVersion.minor}.{currentVersion.patch}
                      <span className="text-xs text-slate-400 ml-2 font-normal">(Build #{currentVersion.build})</span>
                    </h3>
                  </div>
                  <div className="text-right">
                    <span className="text-xs text-slate-400">Zuletzt aktualisiert</span>
                    <p className="text-xs text-slate-300 font-mono">{currentVersion.timestamp}</p>
                  </div>
                </div>

                <div className="p-4 rounded-xl bg-white/5 border border-white/5 space-y-2 text-xs">
                  <div className="flex items-center justify-between">
                    <span className="text-slate-400">Empfohlene Versionierungs-Aktion:</span>
                    <span className="font-bold text-amber-300 uppercase">{currentVersion.recommendedBump}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-slate-400">Begründung:</span>
                    <span className="text-slate-200">{currentVersion.recommendedReason}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-slate-400">Letzte Ursache:</span>
                    <span className="text-slate-200">{currentVersion.lastBumpReason}</span>
                  </div>
                </div>

                <div className="pt-2 flex items-center gap-3">
                  <button
                    onClick={() => handleSyncAllNow('patch')}
                    className="flex-1 py-2.5 bg-white/10 hover:bg-white/15 border border-white/10 rounded-xl text-xs font-bold text-white transition-all"
                  >
                    Patch-Release (Bugfixes & Wartung)
                  </button>
                  <button
                    onClick={() => handleSyncAllNow('minor')}
                    className="flex-1 py-2.5 bg-indigo-500/20 hover:bg-indigo-500/30 border border-indigo-500/40 rounded-xl text-xs font-bold text-indigo-300 transition-all"
                  >
                    Minor-Release (Neue Features)
                  </button>
                  <button
                    onClick={() => handleSyncAllNow('major')}
                    className="flex-1 py-2.5 bg-purple-500/20 hover:bg-purple-500/30 border border-purple-500/40 rounded-xl text-xs font-bold text-purple-300 transition-all"
                  >
                    Major-Release (Breaking Changes)
                  </button>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Modal Footer */}
        <div className="px-6 py-3 border-t border-white/10 bg-black/40 flex items-center justify-between shrink-0 text-xs text-slate-400">
          <div className="flex items-center gap-4">
            <span>Synchronisationsmodus: <strong className="text-cyan-300">Live-Reaktiv</strong></span>
            <span>Zielartefakte: <strong className="text-slate-300">6/6 Standards konform</strong></span>
          </div>
          <button
            onClick={onClose}
            className="px-4 py-1.5 bg-white/10 hover:bg-white/20 text-white rounded-lg transition-colors font-medium"
          >
            Schließen
          </button>
        </div>
      </motion.div>
    </div>
  );
}
