import React, { useState, useMemo } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import {
  ShieldAlert,
  ShieldCheck,
  CheckCircle2,
  AlertTriangle,
  AlertOctagon,
  Info,
  Wand2,
  RefreshCw,
  Search,
  FileCode,
  ArrowRight,
  GitBranch,
  Network,
  ListTodo,
  BookOpen,
  Layers,
  Sparkles,
  ExternalLink,
  Plus,
  Check,
  Download,
  FilePlus,
  Trash2,
  Flame,
  Bug,
  Filter,
  Eye,
  Settings,
  Cpu,
  Map,
  TrendingUp,
  Tag,
  FileText,
  Zap,
  Sliders,
  Clock,
} from 'lucide-react';
import {
  AuditCategory,
  AuditSeverity,
  AuditFinding,
  ProjectAuditSummary,
  TodoItem,
  LiveArchitectureDoc
} from '../types/audit';
import {
  FileState,
  runProjectAudit,
  applyFindingAutoFix,
  applyAllAutoFixes,
  extractProjectTodos,
  toggleTodoStatusInFile,
  syncTodoFileInWorkspace,
  generateLiveArchitectureDocs,
  syncWikiFileInWorkspace,
} from '../utils/auditEngine';
import {
  AutoSyncConfig,
  AutoSyncPillarId,
} from '../types/sync';
import {
  generateLiveRoadmap,
  generateRoadmapMarkdown,
  syncRoadmapFileInWorkspace,
  generateLiveSprints,
  generateSprintsMarkdown,
  syncSprintFileInWorkspace,
  generateLiveDocumentation,
  syncDocumentationFilesInWorkspace,
  extractCurrentVersion,
  syncVersionFileInWorkspace,
  syncAllProjectArtifacts,
} from '../utils/projectSyncEngine';

interface AuditPanelProps {
  files: FileState[];
  setFiles: React.Dispatch<React.SetStateAction<FileState[]>>;
  onJumpToFileAndLine: (fileName: string, line: number) => void;
  onShowToast: (msg: string, type: 'info' | 'warning' | 'error') => void;
  autoSyncTodo: boolean;
  setAutoSyncTodo: (val: boolean) => void;
  autoSyncWiki: boolean;
  setAutoSyncWiki: (val: boolean) => void;
  autoSyncConfig?: AutoSyncConfig;
  setAutoSyncConfig?: React.Dispatch<React.SetStateAction<AutoSyncConfig>>;
  onOpenAutoSyncHub?: () => void;
}

export function AuditPanel({
  files,
  setFiles,
  onJumpToFileAndLine,
  onShowToast,
  autoSyncTodo,
  setAutoSyncTodo,
  autoSyncWiki,
  setAutoSyncWiki,
  autoSyncConfig,
  setAutoSyncConfig,
  onOpenAutoSyncHub,
}: AuditPanelProps) {
  // Current active sub-tab
  const [activeTab, setActiveTab] = useState<
    'overview' | 'security' | 'completeness' | 'functionality' | 'linkage' | 'autofix' | 'todos' | 'wiki' | 'roadmap' | 'sprints' | 'docs' | 'version'
  >('overview');

  // Search & Filters
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedSeverity, setSelectedSeverity] = useState<string>('all');
  const [selectedFile, setSelectedFile] = useState<string>('all');

  // New TODO Modal state
  const [isNewTodoOpen, setIsNewTodoOpen] = useState(false);
  const [newTodoFile, setNewTodoFile] = useState(files[0]?.name || 'main.lm');
  const [newTodoText, setNewTodoText] = useState('');
  const [newTodoTag, setNewTodoTag] = useState<'TODO' | 'FIXME' | 'BUG' | 'HACK'>('TODO');

  // Dynamic audit execution
  const auditSummary: ProjectAuditSummary = useMemo(() => {
    return runProjectAudit(files);
  }, [files]);

  // Live extracted TODOs
  const todos: TodoItem[] = useMemo(() => {
    return extractProjectTodos(files, auditSummary.findings);
  }, [files, auditSummary.findings]);

  // Live Architecture Docs
  const liveWiki: LiveArchitectureDoc = useMemo(() => {
    return generateLiveArchitectureDocs(files, auditSummary);
  }, [files, auditSummary]);

  // Live 6-Pillar Artifacts
  const roadmapDoc = useMemo(() => generateLiveRoadmap(files), [files]);
  const sprintDoc = useMemo(() => generateLiveSprints(files), [files]);
  const liveDocs = useMemo(() => generateLiveDocumentation(files), [files]);
  const currentVersion = useMemo(() => extractCurrentVersion(files), [files]);

  // Filtered findings
  const filteredFindings = useMemo(() => {
    return auditSummary.findings.filter(f => {
      // Tab category filter
      if (activeTab === 'security' && f.category !== 'security') return false;
      if (activeTab === 'completeness' && f.category !== 'completeness') return false;
      if (activeTab === 'functionality' && f.category !== 'functionality') return false;
      if (activeTab === 'linkage' && f.category !== 'linkage') return false;
      if (activeTab === 'autofix' && !f.autoFixable) return false;

      // Severity filter
      if (selectedSeverity !== 'all' && f.severity !== selectedSeverity) return false;

      // File filter
      if (selectedFile !== 'all' && f.fileName !== selectedFile) return false;

      // Search query
      if (searchQuery.trim()) {
        const q = searchQuery.toLowerCase();
        return (
          f.title.toLowerCase().includes(q) ||
          f.description.toLowerCase().includes(q) ||
          f.ruleId.toLowerCase().includes(q) ||
          f.fileName.toLowerCase().includes(q) ||
          f.recommendation.toLowerCase().includes(q)
        );
      }

      return true;
    });
  }, [auditSummary.findings, activeTab, selectedSeverity, selectedFile, searchQuery]);

  // Single Auto-Fix
  const handleApplySingleFix = (finding: AuditFinding) => {
    const updated = applyFindingAutoFix(files, finding);
    setFiles(updated);
    onShowToast(`Verbesserung angewendet: ${finding.autoFix?.label || finding.title}`, 'info');

    // If auto-sync is enabled, update TODO or Wiki files
    if (autoSyncTodo) {
      const refreshedTodos = extractProjectTodos(updated, auditSummary.findings);
      setFiles(syncTodoFileInWorkspace(updated, refreshedTodos));
    }
  };

  // Batch Auto-Fix All
  const handleApplyAllFixes = () => {
    const { updatedFiles, fixedCount } = applyAllAutoFixes(files, auditSummary.findings);
    if (fixedCount === 0) {
      onShowToast('Keine automatisch behebbaren Probleme vorhanden.', 'info');
      return;
    }
    setFiles(updatedFiles);
    onShowToast(`Erfolgreich ${fixedCount} Verbesserungen automatisch integriert!`, 'info');

    if (autoSyncTodo) {
      const refreshedTodos = extractProjectTodos(updatedFiles, auditSummary.findings);
      setFiles(syncTodoFileInWorkspace(updatedFiles, refreshedTodos));
    }
    if (autoSyncWiki) {
      const refreshedAudit = runProjectAudit(updatedFiles);
      const refreshedDoc = generateLiveArchitectureDocs(updatedFiles, refreshedAudit);
      setFiles(syncWikiFileInWorkspace(updatedFiles, refreshedDoc));
    }
  };

  // Toggle TODO item
  const handleToggleTodo = (todo: TodoItem) => {
    if (todo.source === 'code_comment') {
      const updated = toggleTodoStatusInFile(files, todo);
      setFiles(updated);
      onShowToast(`Aufgabe markiert: ${todo.text.slice(0, 30)}...`, 'info');
      if (autoSyncTodo) {
        const refreshedTodos = extractProjectTodos(updated, auditSummary.findings);
        setFiles(syncTodoFileInWorkspace(updated, refreshedTodos));
      }
    } else {
      onShowToast('Audit-Aufgaben werden durch Beheben des Audit-Problems im Code gelöst.', 'info');
    }
  };

  // Add new TODO comment to selected file
  const handleAddNewTodo = () => {
    if (!newTodoText.trim()) return;
    const targetFile = files.find(f => f.name === newTodoFile);
    if (!targetFile) return;

    const newComment = `\n// ${newTodoTag}: ${newTodoText.trim()}`;
    const updatedFiles = files.map(f => {
      if (f.name === newTodoFile) {
        return { ...f, content: f.content + newComment };
      }
      return f;
    });

    setFiles(updatedFiles);
    setIsNewTodoOpen(false);
    setNewTodoText('');
    onShowToast(`Neues ${newTodoTag} in '${newTodoFile}' eingefügt.`, 'info');

    if (autoSyncTodo) {
      const refreshedTodos = extractProjectTodos(updatedFiles, auditSummary.findings);
      setFiles(syncTodoFileInWorkspace(updatedFiles, refreshedTodos));
    }
  };

  // Sync TODO.md explicitly
  const handleSyncTodoFile = () => {
    const updated = syncTodoFileInWorkspace(files, todos);
    setFiles(updated);
    onShowToast(`'TODO.md' erfolgreich im Workspace synchronisiert!`, 'info');
  };

  // Sync ARCHITECTURE.md explicitly
  const handleSyncWikiFile = () => {
    const updated = syncWikiFileInWorkspace(files, liveWiki);
    setFiles(updated);
    onShowToast(`'ARCHITECTURE.md' erfolgreich im Workspace synchronisiert!`, 'info');
  };

  // Count fixable
  const fixableCount = useMemo(() => {
    return auditSummary.findings.filter(f => f.autoFixable).length;
  }, [auditSummary.findings]);

  // Color helper for severity
  const getSeverityBadge = (sev: AuditSeverity) => {
    switch (sev) {
      case 'critical':
        return <span className="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-red-500/20 text-red-400 border border-red-500/30 flex items-center gap-1"><AlertOctagon className="w-3 h-3" /> Kritisch</span>;
      case 'high':
        return <span className="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-amber-500/20 text-amber-400 border border-amber-500/30 flex items-center gap-1"><AlertTriangle className="w-3 h-3" /> Hoch</span>;
      case 'medium':
        return <span className="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-yellow-500/20 text-yellow-400 border border-yellow-500/30 flex items-center gap-1"><Info className="w-3 h-3" /> Mittel</span>;
      case 'low':
        return <span className="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-blue-500/20 text-blue-400 border border-blue-500/30 flex items-center gap-1"><Info className="w-3 h-3" /> Niedrig</span>;
      default:
        return <span className="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-slate-500/20 text-slate-400 border border-slate-500/30">Info</span>;
    }
  };

  // Health grade badge
  const getGradeColor = (grade: string) => {
    switch (grade) {
      case 'A+':
      case 'A':
        return 'text-emerald-400 border-emerald-500/40 bg-emerald-500/10';
      case 'B':
        return 'text-cyan-400 border-cyan-500/40 bg-cyan-500/10';
      case 'C':
        return 'text-yellow-400 border-yellow-500/40 bg-yellow-500/10';
      case 'D':
        return 'text-amber-500 border-amber-500/40 bg-amber-500/10';
      default:
        return 'text-red-400 border-red-500/40 bg-red-500/10';
    }
  };

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#08080a] text-slate-200 select-none overflow-hidden">
      {/* Top Header */}
      <div className="h-16 flex items-center justify-between px-6 border-b border-white/10 bg-black/40 backdrop-blur-2xl shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-xl bg-cyan-500/15 border border-cyan-500/30 shadow-lg">
            <ShieldCheck className="w-6 h-6 text-cyan-400" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-sm font-extrabold text-slate-100 tracking-wide uppercase">
                ATOS Audit & Code-Health Suite
              </h2>
              <span className={`px-2 py-0.5 rounded-full text-xs font-mono font-bold border ${getGradeColor(auditSummary.grade)}`}>
                Note {auditSummary.grade} • {auditSummary.healthScore}/100
              </span>
            </div>
            <p className="text-[11px] text-slate-500">
              Sicherheit • Vollständigkeit • Funktionen • Verknüpfungen • Auto-Verbesserung
            </p>
          </div>
        </div>

        {/* Global Action Buttons */}
        <div className="flex items-center gap-2.5">
          {fixableCount > 0 && (
            <motion.button
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
              onClick={handleApplyAllFixes}
              className="flex items-center gap-1.5 px-3.5 py-1.5 bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white font-bold text-xs rounded-lg shadow-lg border border-emerald-400/30 transition-all cursor-pointer"
              title="Behebt alle uninitialisierten Variablen, fehlende Returns, unsichere Blöcke und Imports"
            >
              <Wand2 className="w-3.5 h-3.5 animate-pulse" />
              <span>Alle Verbesserungen anwenden ({fixableCount})</span>
            </motion.button>
          )}

          {/* Auto-Sync Controls & Hub Trigger */}
          <div className="flex items-center gap-1.5">
            {onOpenAutoSyncHub && (
              <motion.button
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
                onClick={onOpenAutoSyncHub}
                className="flex items-center gap-1.5 px-3 py-1.5 bg-gradient-to-r from-cyan-500/20 via-blue-500/20 to-purple-500/20 hover:from-cyan-500/30 hover:to-purple-500/30 text-cyan-300 border border-cyan-500/40 rounded-lg text-xs font-bold transition-all shadow-md cursor-pointer"
                title="Auto-Sync & Artefakt-Zentrale: Wiki, Roadmap, Todos, Sprints, Dokumentation, Version"
              >
                <RefreshCw className="w-3.5 h-3.5 text-cyan-400" />
                <span>Auto-Sync Zentrale</span>
                <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-cyan-500/30 text-cyan-200 font-mono">
                  {autoSyncConfig ? Object.values(autoSyncConfig).filter(Boolean).length : 6}/6
                </span>
              </motion.button>
            )}

            <div className="flex items-center gap-1 bg-white/5 border border-white/10 rounded-lg p-1 text-xs">
              <button
                onClick={() => {
                  const next = !autoSyncTodo;
                  setAutoSyncTodo(next);
                  if (setAutoSyncConfig) setAutoSyncConfig(prev => ({ ...prev, autoSyncTodo: next }));
                  onShowToast(`Auto-Sync TODO.md ${next ? 'aktiviert' : 'deaktiviert'}`, 'info');
                }}
                className={`px-2 py-1 rounded text-[11px] font-semibold transition-colors flex items-center gap-1 ${
                  autoSyncTodo ? 'bg-amber-500/20 text-amber-300 border border-amber-500/30' : 'text-slate-400 hover:text-slate-200'
                }`}
                title="Aktualisiert automatisch TODO.md bei jeder Code-Änderung"
              >
                <ListTodo className="w-3 h-3" />
                <span>TODO</span>
              </button>
              <button
                onClick={() => {
                  const next = !autoSyncWiki;
                  setAutoSyncWiki(next);
                  if (setAutoSyncConfig) setAutoSyncConfig(prev => ({ ...prev, autoSyncWiki: next }));
                  onShowToast(`Auto-Sync Wiki ${next ? 'aktiviert' : 'deaktiviert'}`, 'info');
                }}
                className={`px-2 py-1 rounded text-[11px] font-semibold transition-colors flex items-center gap-1 ${
                  autoSyncWiki ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30' : 'text-slate-400 hover:text-slate-200'
                }`}
                title="Aktualisiert automatisch ARCHITECTURE.md bei jeder Code-Änderung"
              >
                <BookOpen className="w-3 h-3" />
                <span>Wiki</span>
              </button>
              {autoSyncConfig && setAutoSyncConfig && (
                <>
                  <button
                    onClick={() => {
                      const next = !autoSyncConfig.autoSyncRoadmap;
                      setAutoSyncConfig(prev => ({ ...prev, autoSyncRoadmap: next }));
                      onShowToast(`Auto-Sync Roadmap ${next ? 'aktiviert' : 'deaktiviert'}`, 'info');
                    }}
                    className={`px-2 py-1 rounded text-[11px] font-semibold transition-colors flex items-center gap-1 ${
                      autoSyncConfig.autoSyncRoadmap ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30' : 'text-slate-400 hover:text-slate-200'
                    }`}
                    title="Aktualisiert automatisch ROADMAP.md"
                  >
                    <Map className="w-3 h-3" />
                    <span>Roadmap</span>
                  </button>
                  <button
                    onClick={() => {
                      const next = !autoSyncConfig.autoSyncSprints;
                      setAutoSyncConfig(prev => ({ ...prev, autoSyncSprints: next }));
                      onShowToast(`Auto-Sync Sprints ${next ? 'aktiviert' : 'deaktiviert'}`, 'info');
                    }}
                    className={`px-2 py-1 rounded text-[11px] font-semibold transition-colors flex items-center gap-1 ${
                      autoSyncConfig.autoSyncSprints ? 'bg-fuchsia-500/20 text-fuchsia-300 border border-fuchsia-500/30' : 'text-slate-400 hover:text-slate-200'
                    }`}
                    title="Aktualisiert automatisch SPRINTS.md"
                  >
                    <TrendingUp className="w-3 h-3" />
                    <span>Sprints</span>
                  </button>
                </>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* Navigation Sub-Tabs */}
      <div className="flex items-center justify-between px-6 border-b border-white/10 bg-black/20 text-xs font-semibold overflow-x-auto">
        <div className="flex items-center space-x-1 py-2">
          <button
            onClick={() => setActiveTab('overview')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'overview'
                ? 'bg-white/10 text-white font-bold border border-white/15'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <Layers className="w-3.5 h-3.5 text-cyan-400" />
            <span>Übersicht & Analyse</span>
          </button>

          <button
            onClick={() => setActiveTab('security')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'security'
                ? 'bg-red-500/20 text-red-300 font-bold border border-red-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <ShieldAlert className="w-3.5 h-3.5 text-red-400" />
            <span>Security-Audit</span>
            {auditSummary.byCategory.security > 0 && (
              <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-red-500/30 text-red-300 font-mono">
                {auditSummary.byCategory.security}
              </span>
            )}
          </button>

          <button
            onClick={() => setActiveTab('completeness')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'completeness'
                ? 'bg-amber-500/20 text-amber-300 font-bold border border-amber-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <CheckCircle2 className="w-3.5 h-3.5 text-amber-400" />
            <span>Vollständigkeit</span>
            {auditSummary.byCategory.completeness > 0 && (
              <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-amber-500/30 text-amber-300 font-mono">
                {auditSummary.byCategory.completeness}
              </span>
            )}
          </button>

          <button
            onClick={() => setActiveTab('functionality')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'functionality'
                ? 'bg-purple-500/20 text-purple-300 font-bold border border-purple-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <Cpu className="w-3.5 h-3.5 text-purple-400" />
            <span>Funktions-Audit</span>
            {auditSummary.byCategory.functionality > 0 && (
              <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-purple-500/30 text-purple-300 font-mono">
                {auditSummary.byCategory.functionality}
              </span>
            )}
          </button>

          <button
            onClick={() => setActiveTab('linkage')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'linkage'
                ? 'bg-emerald-500/20 text-emerald-300 font-bold border border-emerald-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <Network className="w-3.5 h-3.5 text-emerald-400" />
            <span>Verknüpft-Audit</span>
            {auditSummary.byCategory.linkage > 0 && (
              <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-emerald-500/30 text-emerald-300 font-mono">
                {auditSummary.byCategory.linkage}
              </span>
            )}
          </button>

          <button
            onClick={() => setActiveTab('autofix')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'autofix'
                ? 'bg-teal-500/20 text-teal-300 font-bold border border-teal-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <Wand2 className="w-3.5 h-3.5 text-teal-400" />
            <span>Auto-Verbesserung</span>
            {fixableCount > 0 && (
              <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-teal-500/30 text-teal-300 font-mono">
                {fixableCount}
              </span>
            )}
          </button>

          <div className="w-px h-4 bg-white/10 mx-1"></div>

          <button
            onClick={() => setActiveTab('todos')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'todos'
                ? 'bg-amber-500/20 text-amber-300 font-bold border border-amber-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <ListTodo className="w-3.5 h-3.5 text-amber-400" />
            <span>TODOs</span>
            <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-white/10 text-slate-300 font-mono">
              {todos.filter(t => !t.completed).length}
            </span>
          </button>

          <button
            onClick={() => setActiveTab('wiki')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'wiki'
                ? 'bg-cyan-500/20 text-cyan-300 font-bold border border-cyan-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <BookOpen className="w-3.5 h-3.5 text-cyan-400" />
            <span>Wiki</span>
          </button>

          <button
            onClick={() => setActiveTab('roadmap')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'roadmap'
                ? 'bg-indigo-500/20 text-indigo-300 font-bold border border-indigo-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <Map className="w-3.5 h-3.5 text-indigo-400" />
            <span>Roadmap</span>
            <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-indigo-500/30 text-indigo-300 font-mono">
              {roadmapDoc.overallProgress}%
            </span>
          </button>

          <button
            onClick={() => setActiveTab('sprints')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'sprints'
                ? 'bg-fuchsia-500/20 text-fuchsia-300 font-bold border border-fuchsia-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <TrendingUp className="w-3.5 h-3.5 text-fuchsia-400" />
            <span>Sprints</span>
            <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-fuchsia-500/30 text-fuchsia-300 font-mono">
              #{sprintDoc.sprintNumber}
            </span>
          </button>

          <button
            onClick={() => setActiveTab('docs')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'docs'
                ? 'bg-emerald-500/20 text-emerald-300 font-bold border border-emerald-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <FileText className="w-3.5 h-3.5 text-emerald-400" />
            <span>Doku</span>
            <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-emerald-500/30 text-emerald-300 font-mono">
              {liveDocs.indexedFiles.length}
            </span>
          </button>

          <button
            onClick={() => setActiveTab('version')}
            className={`px-3 py-1.5 rounded-lg flex items-center gap-2 transition-all ${
              activeTab === 'version'
                ? 'bg-cyan-500/20 text-cyan-300 font-bold border border-cyan-500/30'
                : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
            }`}
          >
            <Tag className="w-3.5 h-3.5 text-cyan-400" />
            <span>Version</span>
            <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-cyan-500/30 text-cyan-200 font-mono">
              v{currentVersion.major}.{currentVersion.minor}.{currentVersion.patch}
            </span>
          </button>
        </div>

        {/* Quick info */}
        <div className="text-[11px] text-slate-500 font-mono hidden md:block">
          {files.length} Dateien • {auditSummary.analyzedLinesCount} Zeilen Code analysiert
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 overflow-y-auto p-6 relative">
        <div className="max-w-7xl mx-auto space-y-6">

          {/* ========================================================================= */}
          {/* TAB 1: OVERVIEW & ANALYSIS                                                */}
          {/* ========================================================================= */}
          {activeTab === 'overview' && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-6">
              {/* Scorecard Hero */}
              <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                {/* Health Score Gauge */}
                <div className="p-5 rounded-2xl bg-gradient-to-br from-black/60 to-black/30 border border-white/10 flex items-center justify-between shadow-xl">
                  <div>
                    <span className="text-xs font-semibold text-slate-400 uppercase tracking-wider block">Health Score</span>
                    <div className="flex items-baseline gap-2 mt-1">
                      <span className="text-4xl font-extrabold text-white tracking-tight">{auditSummary.healthScore}</span>
                      <span className="text-slate-500 text-sm font-mono">/100</span>
                    </div>
                    <span className="text-xs text-slate-400 mt-1 block">
                      Qualitätsnote: <strong className="text-cyan-400">{auditSummary.grade}</strong>
                    </span>
                  </div>
                  <div className={`w-16 h-16 rounded-full border-4 flex items-center justify-center font-mono text-xl font-bold shadow-inner ${getGradeColor(auditSummary.grade)}`}>
                    {auditSummary.grade}
                  </div>
                </div>

                {/* Security Status */}
                <div
                  onClick={() => setActiveTab('security')}
                  className="p-5 rounded-2xl bg-black/40 border border-red-500/20 hover:border-red-500/40 transition-all cursor-pointer shadow-lg group"
                >
                  <div className="flex items-center justify-between text-xs font-semibold text-red-400 uppercase tracking-wider">
                    <span>Security-Audit</span>
                    <ShieldAlert className="w-4 h-4 text-red-400 group-hover:scale-110 transition-transform" />
                  </div>
                  <div className="text-3xl font-extrabold text-white mt-2 font-mono">
                    {auditSummary.byCategory.security}
                  </div>
                  <div className="text-[11px] text-slate-400 mt-1 flex items-center justify-between">
                    <span>{auditSummary.criticalCount} kritisch • {auditSummary.highCount} hoch</span>
                    <ArrowRight className="w-3 h-3 text-red-400 opacity-0 group-hover:opacity-100 transition-opacity" />
                  </div>
                </div>

                {/* Completeness Status */}
                <div
                  onClick={() => setActiveTab('completeness')}
                  className="p-5 rounded-2xl bg-black/40 border border-amber-500/20 hover:border-amber-500/40 transition-all cursor-pointer shadow-lg group"
                >
                  <div className="flex items-center justify-between text-xs font-semibold text-amber-400 uppercase tracking-wider">
                    <span>Vollständigkeit</span>
                    <CheckCircle2 className="w-4 h-4 text-amber-400 group-hover:scale-110 transition-transform" />
                  </div>
                  <div className="text-3xl font-extrabold text-white mt-2 font-mono">
                    {auditSummary.byCategory.completeness}
                  </div>
                  <div className="text-[11px] text-slate-400 mt-1 flex items-center justify-between">
                    <span>Initialisierung & Stubs</span>
                    <ArrowRight className="w-3 h-3 text-amber-400 opacity-0 group-hover:opacity-100 transition-opacity" />
                  </div>
                </div>

                {/* Linkage & Dependencies */}
                <div
                  onClick={() => setActiveTab('linkage')}
                  className="p-5 rounded-2xl bg-black/40 border border-emerald-500/20 hover:border-emerald-500/40 transition-all cursor-pointer shadow-lg group"
                >
                  <div className="flex items-center justify-between text-xs font-semibold text-emerald-400 uppercase tracking-wider">
                    <span>Verknüpft-Audit</span>
                    <Network className="w-4 h-4 text-emerald-400 group-hover:scale-110 transition-transform" />
                  </div>
                  <div className="text-3xl font-extrabold text-white mt-2 font-mono">
                    {auditSummary.linkageGraph.edges.length}
                  </div>
                  <div className="text-[11px] text-slate-400 mt-1 flex items-center justify-between">
                    <span>{auditSummary.orphanedFiles.length} verwaist • {auditSummary.circularDeps.length} zirkulär</span>
                    <ArrowRight className="w-3 h-3 text-emerald-400 opacity-0 group-hover:opacity-100 transition-opacity" />
                  </div>
                </div>
              </div>

              {/* Quick Actions Bar */}
              <div className="p-4 rounded-xl bg-gradient-to-r from-cyan-950/40 via-black/50 to-indigo-950/40 border border-cyan-500/20 flex flex-col sm:flex-row items-center justify-between gap-4">
                <div className="flex items-center gap-3">
                  <Sparkles className="w-5 h-5 text-cyan-400 shrink-0" />
                  <div>
                    <h4 className="text-xs font-bold text-slate-200">Automatische Reparatur & Synchronisation bereit</h4>
                    <p className="text-[11px] text-slate-400">
                      {fixableCount} von {auditSummary.totalFindings} Problemen können mit 1 Klick automatisch behoben werden.
                    </p>
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  {fixableCount > 0 && (
                    <button
                      onClick={handleApplyAllFixes}
                      className="px-3 py-1.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-xs flex items-center gap-1.5 shadow transition-all cursor-pointer"
                    >
                      <Wand2 className="w-3.5 h-3.5" />
                      <span>Alle beheben ({fixableCount})</span>
                    </button>
                  )}
                  <button
                    onClick={handleSyncTodoFile}
                    className="px-3 py-1.5 rounded-lg bg-white/10 hover:bg-white/20 text-slate-200 font-semibold text-xs flex items-center gap-1.5 transition-all cursor-pointer"
                  >
                    <ListTodo className="w-3.5 h-3.5 text-amber-400" />
                    <span>TODO.md schreiben</span>
                  </button>
                  <button
                    onClick={handleSyncWikiFile}
                    className="px-3 py-1.5 rounded-lg bg-white/10 hover:bg-white/20 text-slate-200 font-semibold text-xs flex items-center gap-1.5 transition-all cursor-pointer"
                  >
                    <BookOpen className="w-3.5 h-3.5 text-cyan-400" />
                    <span>ARCHITECTURE.md schreiben</span>
                  </button>
                </div>
              </div>

              {/* Critical Findings Spotlight */}
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-2">
                    <Flame className="w-4 h-4 text-red-400" />
                    Dringende Handlungsfelder & Befunde
                  </h3>
                  <span className="text-xs text-slate-500">{auditSummary.findings.length} Gesamtbefunde</span>
                </div>

                {auditSummary.findings.length === 0 ? (
                  <div className="p-8 rounded-2xl bg-emerald-500/10 border border-emerald-500/20 text-center text-emerald-300">
                    <CheckCircle2 className="w-10 h-10 mx-auto mb-2 text-emerald-400" />
                    <h4 className="font-bold text-base">Hervorragend! Keine Audit-Probleme gefunden.</h4>
                    <p className="text-xs text-emerald-400/80 mt-1">Ihr Lumino-Quellcode erfüllt alle Sicherheits-, Vollständigkeits- und Verknüpfungsstandards.</p>
                  </div>
                ) : (
                  <div className="space-y-2">
                    {auditSummary.findings.slice(0, 5).map((f) => (
                      <div
                        key={f.id}
                        className="p-4 rounded-xl bg-black/40 border border-white/10 hover:border-white/20 transition-all flex items-start justify-between gap-4"
                      >
                        <div className="space-y-1">
                          <div className="flex items-center gap-2">
                            {getSeverityBadge(f.severity)}
                            <span className="text-xs font-mono font-bold text-slate-400">{f.ruleId}</span>
                            <span className="text-xs text-slate-500">•</span>
                            <button
                              onClick={() => onJumpToFileAndLine(f.fileName, f.line)}
                              className="text-xs font-mono text-cyan-400 hover:underline flex items-center gap-1"
                            >
                              <FileCode className="w-3 h-3" />
                              {f.fileName}:{f.line}
                            </button>
                          </div>
                          <h4 className="text-sm font-bold text-slate-100">{f.title}</h4>
                          <p className="text-xs text-slate-400 leading-relaxed">{f.description}</p>
                          <div className="text-[11px] text-emerald-400/90 font-mono bg-emerald-500/10 px-2 py-1 rounded mt-1 border border-emerald-500/20">
                            Empfehlung: {f.recommendation}
                          </div>
                        </div>

                        <div className="shrink-0 flex flex-col gap-1.5 items-end">
                          {f.autoFixable && (
                            <button
                              onClick={() => handleApplySingleFix(f)}
                              className="px-3 py-1 bg-teal-500/20 hover:bg-teal-500/30 text-teal-300 border border-teal-500/40 rounded-lg text-xs font-bold flex items-center gap-1 transition-all cursor-pointer shadow-sm"
                            >
                              <Wand2 className="w-3 h-3" />
                              <span>Beheben</span>
                            </button>
                          )}
                          <button
                            onClick={() => onJumpToFileAndLine(f.fileName, f.line)}
                            className="px-2.5 py-1 bg-white/5 hover:bg-white/10 text-slate-300 rounded text-xs transition-colors flex items-center gap-1"
                          >
                            <ExternalLink className="w-3 h-3" />
                            <span>Im Editor</span>
                          </button>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </motion.div>
          )}

          {/* ========================================================================= */}
          {/* TABS 2, 3, 4, 6: DETAILED FINDINGS LISTS (Security, Comp, Func, AutoFix) */}
          {/* ========================================================================= */}
          {(activeTab === 'security' ||
            activeTab === 'completeness' ||
            activeTab === 'functionality' ||
            activeTab === 'autofix') && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-4">
              {/* Category Header Banner */}
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                <div>
                  <h3 className="text-base font-bold text-slate-100 flex items-center gap-2">
                    {activeTab === 'security' && <ShieldAlert className="w-5 h-5 text-red-400" />}
                    {activeTab === 'completeness' && <CheckCircle2 className="w-5 h-5 text-amber-400" />}
                    {activeTab === 'functionality' && <Cpu className="w-5 h-5 text-purple-400" />}
                    {activeTab === 'autofix' && <Wand2 className="w-5 h-5 text-teal-400" />}
                    {activeTab === 'security' && 'Security-Audit (Schwachstellen, Secrets, Unsafe Code)'}
                    {activeTab === 'completeness' && 'Vollständigkeits-Prüfung (Initialisierung, Returns, Stubs)'}
                    {activeTab === 'functionality' && 'Funktions-Audit (Komplexität, Dead Code, Signaturen)'}
                    {activeTab === 'autofix' && 'Automatische Verbesserungen (1-Click Auto-Remediation)'}
                  </h3>
                  <p className="text-xs text-slate-400 mt-1">
                    {activeTab === 'security' && 'Prüft auf Klartext-Geheimnisse, ungeprüfte Syscalls, Endlosschleifen (DoS) und unsichere Speicherzugriffe.'}
                    {activeTab === 'completeness' && 'Prüft auf fehlende Rückgabewerte, unvollständige Verzweigungen, verschluckte Fehler und uninitialisierte Variablen.'}
                    {activeTab === 'functionality' && 'Analysiert zyklomatische Komplexität, überzählige Parameter, Funktionslängen und toten Code.'}
                    {activeTab === 'autofix' && 'Alle für diesen Workspace generierten automatischen Reparaturen und Code-Transformationen.'}
                  </p>
                </div>

                {activeTab === 'autofix' && fixableCount > 0 && (
                  <button
                    onClick={handleApplyAllFixes}
                    className="px-4 py-2 bg-gradient-to-r from-teal-600 to-emerald-600 hover:from-teal-500 hover:to-emerald-500 text-white font-bold text-xs rounded-xl shadow-lg flex items-center gap-2 shrink-0 cursor-pointer"
                  >
                    <Wand2 className="w-4 h-4" />
                    <span>Alle {fixableCount} Verbesserungen anwenden</span>
                  </button>
                )}
              </div>

              {/* Filter Bar */}
              <div className="flex flex-wrap items-center gap-3 bg-black/20 p-3 rounded-xl border border-white/5 text-xs">
                <div className="relative flex-1 min-w-[200px]">
                  <Search className="w-3.5 h-3.5 absolute left-3 top-2.5 text-slate-500" />
                  <input
                    type="text"
                    placeholder="Befunde filtern nach Regel, Titel oder Empfehlung..."
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    className="w-full pl-9 pr-3 py-1.5 rounded-lg bg-white/5 border border-white/10 text-slate-200 placeholder-slate-500 text-xs focus:outline-none focus:border-cyan-500"
                  />
                </div>

                <div className="flex items-center gap-2">
                  <span className="text-slate-500">Schweregrad:</span>
                  <select
                    value={selectedSeverity}
                    onChange={(e) => setSelectedSeverity(e.target.value)}
                    className="bg-black/60 border border-white/10 rounded-lg px-2.5 py-1.5 text-slate-300 text-xs focus:outline-none cursor-pointer"
                  >
                    <option value="all">Alle Schweregrade</option>
                    <option value="critical">Kritisch</option>
                    <option value="high">Hoch</option>
                    <option value="medium">Mittel</option>
                    <option value="low">Niedrig</option>
                  </select>
                </div>

                <div className="flex items-center gap-2">
                  <span className="text-slate-500">Datei:</span>
                  <select
                    value={selectedFile}
                    onChange={(e) => setSelectedFile(e.target.value)}
                    className="bg-black/60 border border-white/10 rounded-lg px-2.5 py-1.5 text-slate-300 text-xs focus:outline-none cursor-pointer"
                  >
                    <option value="all">Alle Dateien ({files.length})</option>
                    {files.map(f => (
                      <option key={f.name} value={f.name}>{f.name}</option>
                    ))}
                  </select>
                </div>
              </div>

              {/* Findings List */}
              <div className="space-y-3">
                {filteredFindings.length === 0 ? (
                  <div className="p-8 rounded-2xl bg-black/30 border border-white/10 text-center text-slate-400">
                    <CheckCircle2 className="w-8 h-8 mx-auto mb-2 text-emerald-400" />
                    <p className="text-xs">Keine Befunde für die aktuellen Filterkriterien gefunden.</p>
                  </div>
                ) : (
                  filteredFindings.map((finding) => (
                    <div
                      key={finding.id}
                      className="p-5 rounded-2xl bg-black/40 border border-white/10 hover:border-white/20 transition-all space-y-3 shadow-lg"
                    >
                      <div className="flex items-start justify-between gap-4">
                        <div className="space-y-1">
                          <div className="flex items-center gap-2">
                            {getSeverityBadge(finding.severity)}
                            <span className="text-xs font-mono font-bold text-slate-400">{finding.ruleId}</span>
                            <span className="text-slate-600">•</span>
                            <button
                              onClick={() => onJumpToFileAndLine(finding.fileName, finding.line)}
                              className="text-xs font-mono text-cyan-400 hover:underline flex items-center gap-1"
                            >
                              <FileCode className="w-3.5 h-3.5" />
                              {finding.fileName}:{finding.line}
                            </button>
                          </div>
                          <h4 className="text-sm font-bold text-slate-100">{finding.title}</h4>
                        </div>

                        {/* Actions */}
                        <div className="flex items-center gap-2 shrink-0">
                          {finding.autoFixable && (
                            <button
                              onClick={() => handleApplySingleFix(finding)}
                              className="px-3.5 py-1.5 bg-teal-500/20 hover:bg-teal-500/30 text-teal-300 border border-teal-500/40 rounded-lg text-xs font-bold flex items-center gap-1.5 transition-all cursor-pointer shadow-sm"
                            >
                              <Wand2 className="w-3.5 h-3.5" />
                              <span>{finding.autoFix?.label || 'Automatisch beheben'}</span>
                            </button>
                          )}
                          <button
                            onClick={() => onJumpToFileAndLine(finding.fileName, finding.line)}
                            className="px-3 py-1.5 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs transition-colors flex items-center gap-1"
                          >
                            <ExternalLink className="w-3.5 h-3.5" />
                            <span>Im Editor</span>
                          </button>
                        </div>
                      </div>

                      <p className="text-xs text-slate-300 leading-relaxed">{finding.description}</p>

                      {/* Code Snippet Match */}
                      {finding.match && (
                        <div className="bg-black/70 border border-white/5 rounded-lg p-3 font-mono text-xs text-amber-300/90 overflow-x-auto">
                          <span className="text-[10px] text-slate-500 block mb-1 uppercase font-sans font-bold">Betroffener Code-Ausschnitt:</span>
                          <code>{finding.match}</code>
                        </div>
                      )}

                      {/* Recommendation & Impact */}
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs pt-1">
                        <div className="p-3 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-300">
                          <span className="font-bold block text-[10px] uppercase text-emerald-400 mb-1">Empfohlene Verbesserung:</span>
                          {finding.recommendation}
                        </div>
                        <div className="p-3 rounded-xl bg-red-500/10 border border-red-500/20 text-red-300">
                          <span className="font-bold block text-[10px] uppercase text-red-400 mb-1">Risiko & Auswirkung:</span>
                          {finding.impact}
                        </div>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </motion.div>
          )}

          {/* ========================================================================= */}
          {/* TAB 5: LINKAGE AUDIT (Verknüpft-Audit & Dependency Graph)                  */}
          {/* ========================================================================= */}
          {activeTab === 'linkage' && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-6">
              {/* Header Banner */}
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 flex items-center justify-between">
                <div>
                  <h3 className="text-base font-bold text-slate-100 flex items-center gap-2">
                    <Network className="w-5 h-5 text-emerald-400" />
                    Verknüpft-Audit & Modul-Abhängigkeitsgraph
                  </h3>
                  <p className="text-xs text-slate-400 mt-1">
                    Prüft dateiübergreifende Imports, fehlende Modulreferenzen, zirkuläre Abhängigkeiten und verwaiste Dateien im Workspace.
                  </p>
                </div>
              </div>

              {/* Status Grid */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="p-4 rounded-xl bg-black/40 border border-white/10">
                  <span className="text-xs text-slate-400 font-semibold block uppercase">Aktive Verknüpfungen</span>
                  <div className="text-2xl font-bold font-mono text-emerald-400 mt-1">
                    {auditSummary.linkageGraph.edges.length}
                  </div>
                  <span className="text-[11px] text-slate-500 mt-1 block">
                    {auditSummary.linkageGraph.edges.filter(e => e.resolved).length} aufgelöst,{' '}
                    {auditSummary.linkageGraph.edges.filter(e => !e.resolved).length} fehlerhaft
                  </span>
                </div>

                <div className="p-4 rounded-xl bg-black/40 border border-white/10">
                  <span className="text-xs text-slate-400 font-semibold block uppercase">Verwaiste Dateien</span>
                  <div className="text-2xl font-bold font-mono text-amber-400 mt-1">
                    {auditSummary.orphanedFiles.length}
                  </div>
                  <span className="text-[11px] text-slate-500 mt-1 block">
                    Werden von keinem anderen Modul referenziert
                  </span>
                </div>

                <div className="p-4 rounded-xl bg-black/40 border border-white/10">
                  <span className="text-xs text-slate-400 font-semibold block uppercase">Zirkuläre Zyklen</span>
                  <div className="text-2xl font-bold font-mono text-red-400 mt-1">
                    {auditSummary.circularDeps.length}
                  </div>
                  <span className="text-[11px] text-slate-500 mt-1 block">
                    Kreisabhängigkeiten (A ➔ B ➔ A)
                  </span>
                </div>
              </div>

              {/* Circular Dependencies Alert */}
              {auditSummary.circularDeps.length > 0 && (
                <div className="p-4 rounded-xl bg-red-500/10 border border-red-500/30 space-y-2">
                  <h4 className="text-xs font-bold text-red-400 uppercase flex items-center gap-1.5">
                    <AlertOctagon className="w-4 h-4" />
                    Zirkuläre Abhängigkeiten gefunden:
                  </h4>
                  {auditSummary.circularDeps.map((cd, i) => (
                    <div key={i} className="text-xs font-mono text-red-300 bg-black/40 p-2 rounded border border-red-500/20">
                      {cd.description}
                    </div>
                  ))}
                </div>
              )}

              {/* Visual Dependency Nodes & Edges */}
              <div className="p-6 rounded-2xl bg-black/60 border border-white/10 space-y-4">
                <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-2">
                  <GitBranch className="w-4 h-4 text-cyan-400" />
                  Workspace Modul-Topologie
                </h4>

                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {auditSummary.linkageGraph.nodes.map((node) => {
                    const outgoing = auditSummary.linkageGraph.edges.filter(e => e.from === node.id);
                    const incoming = auditSummary.linkageGraph.edges.filter(e => e.to === node.id);

                    return (
                      <div
                        key={node.id}
                        className={`p-4 rounded-xl border transition-all ${
                          node.isOrphaned
                            ? 'bg-amber-500/5 border-amber-500/20'
                            : 'bg-white/[0.02] border-white/10 hover:border-white/20'
                        }`}
                      >
                        <div className="flex items-center justify-between">
                          <div className="flex items-center gap-2">
                            <span className="p-1 rounded bg-white/5 font-mono text-xs text-cyan-400">.{node.extension}</span>
                            <span className="font-bold text-xs text-slate-200">{node.name}</span>
                          </div>
                          {node.isOrphaned && (
                            <span className="px-1.5 py-0.5 rounded text-[9px] bg-amber-500/20 text-amber-300 border border-amber-500/30">
                              Verwaist
                            </span>
                          )}
                          {node.isEntry && (
                            <span className="px-1.5 py-0.5 rounded text-[9px] bg-emerald-500/20 text-emerald-300 border border-emerald-500/30">
                              Entry
                            </span>
                          )}
                        </div>

                        <div className="text-[11px] text-slate-400 mt-2 space-y-1">
                          <div className="flex justify-between">
                            <span>Zeilen:</span>
                            <span className="font-mono text-slate-300">{node.linesCount}</span>
                          </div>
                          <div className="flex justify-between">
                            <span>Importiert ({outgoing.length}):</span>
                            <span className="font-mono text-slate-300 truncate max-w-[150px]">
                              {outgoing.map(e => e.to).join(', ') || 'Keine'}
                            </span>
                          </div>
                          <div className="flex justify-between">
                            <span>Referenziert von ({incoming.length}):</span>
                            <span className="font-mono text-slate-300 truncate max-w-[150px]">
                              {incoming.map(e => e.from).join(', ') || 'Niemand'}
                            </span>
                          </div>
                        </div>

                        <div className="mt-3 pt-2 border-t border-white/5 flex justify-end">
                          <button
                            onClick={() => onJumpToFileAndLine(node.name, 1)}
                            className="text-[11px] text-cyan-400 hover:underline flex items-center gap-1"
                          >
                            <span>Öffnen</span>
                            <ArrowRight className="w-3 h-3" />
                          </button>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>

              {/* Edge Connections Table */}
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 space-y-3">
                <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider">Alle Import-Verknüpfungen im Detail</h4>
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-xs">
                    <thead>
                      <tr className="border-b border-white/10 text-slate-400 font-mono">
                        <th className="pb-2">Quelle</th>
                        <th className="pb-2">Ziel</th>
                        <th className="pb-2">Import-Syntax</th>
                        <th className="pb-2">Status</th>
                        <th className="pb-2 text-right">Aktion</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-white/5">
                      {auditSummary.linkageGraph.edges.map((edge, idx) => (
                        <tr key={idx} className="hover:bg-white/[0.02]">
                          <td className="py-2.5 font-mono text-cyan-400">{edge.from}:{edge.line}</td>
                          <td className="py-2.5 font-mono text-slate-200">{edge.to}</td>
                          <td className="py-2.5 font-mono text-slate-400">{edge.importStatement}</td>
                          <td className="py-2.5">
                            {edge.resolved ? (
                              <span className="text-emerald-400 flex items-center gap-1 font-semibold">
                                <CheckCircle2 className="w-3 h-3" /> Aufgelöst
                              </span>
                            ) : (
                              <span className="text-red-400 flex items-center gap-1 font-semibold">
                                <AlertOctagon className="w-3 h-3" /> Nicht gefunden!
                              </span>
                            )}
                          </td>
                          <td className="py-2.5 text-right">
                            {!edge.resolved && (
                              <button
                                onClick={() => {
                                  const fixFinding = auditSummary.findings.find(f => f.category === 'linkage' && f.match.includes(edge.to));
                                  if (fixFinding) handleApplySingleFix(fixFinding);
                                }}
                                className="px-2.5 py-1 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-300 border border-emerald-500/30 rounded text-[11px] font-bold inline-flex items-center gap-1"
                              >
                                <FilePlus className="w-3 h-3" /> Datei erstellen
                              </button>
                            )}
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            </motion.div>
          )}

          {/* ========================================================================= */}
          {/* TAB 7: LIVE TODO-LISTE                                                     */}
          {/* ========================================================================= */}
          {activeTab === 'todos' && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-4">
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                <div>
                  <h3 className="text-base font-bold text-slate-100 flex items-center gap-2">
                    <ListTodo className="w-5 h-5 text-amber-400" />
                    Live Projekt-TODOs (Automatisch aus Quellcode & Audit generiert)
                  </h3>
                  <p className="text-xs text-slate-400 mt-1">
                    Verfolgt alle Kommentar-Marker (TODO, FIXME, BUG) in Echtzeit und synchronisiert sie optional direkt mit `TODO.md`.
                  </p>
                </div>

                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setIsNewTodoOpen(true)}
                    className="px-3.5 py-1.5 bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 border border-amber-500/40 rounded-lg text-xs font-bold flex items-center gap-1.5 transition-all cursor-pointer"
                  >
                    <Plus className="w-3.5 h-3.5" />
                    <span>Neues TODO anlegen</span>
                  </button>
                  <button
                    onClick={handleSyncTodoFile}
                    className="px-3.5 py-1.5 bg-white/10 hover:bg-white/20 text-slate-200 rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-all cursor-pointer"
                  >
                    <Download className="w-3.5 h-3.5 text-cyan-400" />
                    <span>In TODO.md speichern</span>
                  </button>
                </div>
              </div>

              {/* Stats: todo overview */}
              <div className="grid grid-cols-2 md:grid-cols-4 gap-3 text-xs">
                <div className="p-3.5 rounded-xl bg-black/30 border border-white/5">
                  <span className="text-slate-400 block">Offene Aufgaben</span>
                  <span className="text-xl font-bold text-amber-400 font-mono mt-1 block">
                    {todos.filter(t => !t.completed).length}
                  </span>
                </div>
                <div className="p-3.5 rounded-xl bg-black/30 border border-white/5">
                  <span className="text-slate-400 block">Erledigte Aufgaben</span>
                  <span className="text-xl font-bold text-emerald-400 font-mono mt-1 block">
                    {todos.filter(t => t.completed).length}
                  </span>
                </div>
                <div className="p-3.5 rounded-xl bg-black/30 border border-white/5">
                  <span className="text-slate-400 block">Aus Code-Kommentaren</span>
                  <span className="text-xl font-bold text-cyan-400 font-mono mt-1 block">
                    {todos.filter(t => t.source === 'code_comment').length}
                  </span>
                </div>
                <div className="p-3.5 rounded-xl bg-black/30 border border-white/5">
                  <span className="text-slate-400 block">Aus Audit-Befunden</span>
                  <span className="text-xl font-bold text-red-400 font-mono mt-1 block">
                    {todos.filter(t => t.source === 'audit_finding').length}
                  </span>
                </div>
              </div>

              {/* Cards: todo items */}
              <div className="space-y-2">
                {todos.length === 0 ? (
                  <div className="p-8 rounded-2xl bg-black/30 border border-white/10 text-center text-slate-400">
                    <CheckCircle2 className="w-8 h-8 mx-auto mb-2 text-emerald-400" />
                    <p className="text-xs">Keine TODOs gefunden. Fügen Sie einen TODO-Marker in Ihren Code ein oder erstellen Sie eines oben!</p>
                  </div>
                ) : (
                  todos.map((todo) => (
                    <div
                      key={todo.id}
                      className={`p-3.5 rounded-xl border transition-all flex items-center justify-between gap-4 ${
                        todo.completed
                          ? 'bg-black/20 border-white/5 opacity-60'
                          : 'bg-black/40 border-white/10 hover:border-white/20'
                      }`}
                    >
                      <div className="flex items-center gap-3">
                        <button
                          onClick={() => handleToggleTodo(todo)}
                          className={`w-5 h-5 rounded flex items-center justify-center border transition-colors cursor-pointer ${
                            todo.completed
                              ? 'bg-emerald-500 border-emerald-400 text-black font-bold'
                              : 'border-white/20 hover:border-white/40'
                          }`}
                        >
                          {todo.completed && <Check className="w-3.5 h-3.5" />}
                        </button>

                        <div className="space-y-0.5">
                          <div className="flex items-center gap-2 text-xs">
                            <span className={`px-1.5 py-0.2 rounded text-[10px] font-mono font-bold uppercase ${
                              todo.tag === 'BUG' || todo.tag === 'FIXME'
                                ? 'bg-red-500/20 text-red-400 border border-red-500/30'
                                : todo.tag === 'AUDIT'
                                ? 'bg-purple-500/20 text-purple-300 border border-purple-500/30'
                                : 'bg-amber-500/20 text-amber-300 border border-amber-500/30'
                            }`}>
                              {todo.tag}
                            </span>
                            <button
                              onClick={() => onJumpToFileAndLine(todo.fileName, todo.line)}
                              className="text-[11px] font-mono text-cyan-400 hover:underline"
                            >
                              {todo.fileName}:{todo.line}
                            </button>
                          </div>
                          <p className={`text-xs ${todo.completed ? 'line-through text-slate-500' : 'text-slate-200'}`}>
                            {todo.text}
                          </p>
                        </div>
                      </div>

                      <button
                        onClick={() => onJumpToFileAndLine(todo.fileName, todo.line)}
                        className="px-2.5 py-1 bg-white/5 hover:bg-white/10 text-slate-300 rounded text-xs transition-colors flex items-center gap-1 shrink-0"
                      >
                        <ExternalLink className="w-3 h-3" />
                        <span>Zum Code</span>
                      </button>
                    </div>
                  ))
                )}
              </div>
            </motion.div>
          )}

          {/* ========================================================================= */}
          {/* TAB 8: LIVE WIKI & ARCHITEKTUR                                             */}
          {/* ========================================================================= */}
          {activeTab === 'wiki' && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-6">
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                <div>
                  <h3 className="text-base font-bold text-slate-100 flex items-center gap-2">
                    <BookOpen className="w-5 h-5 text-cyan-400" />
                    Live Systemarchitektur & Wiki (Automatisch aktualisiert)
                  </h3>
                  <p className="text-xs text-slate-400 mt-1">
                    Aktualisiert sich automatisch bei Änderungen im Quellcode. Synchronisiert Schichtenmodell, Exports und Audit-Status.
                  </p>
                </div>

                <button
                  onClick={handleSyncWikiFile}
                  className="px-4 py-2 bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white font-bold text-xs rounded-xl shadow-lg flex items-center gap-2 shrink-0 cursor-pointer"
                >
                  <Download className="w-4 h-4" />
                  <span>In ARCHITECTURE.md synchronisieren</span>
                </button>
              </div>

              {/* Architectural Layers */}
              <div className="space-y-3">
                <h4 className="text-xs font-bold text-slate-400 uppercase tracking-wider">Erkannte Systemschichten (Layer-Modell)</h4>
                <div className="space-y-3">
                  {liveWiki.layers.map((layer, idx) => (
                    <div key={idx} className="p-4 rounded-xl bg-black/40 border border-white/10 space-y-2">
                      <div className="flex items-center justify-between">
                        <h5 className="text-xs font-bold text-cyan-300 font-mono">{layer.name}</h5>
                        <span className="text-[11px] text-slate-500 font-mono">{layer.files.length} Dateien</span>
                      </div>
                      <p className="text-xs text-slate-400">{layer.description}</p>
                      <div className="flex flex-wrap gap-2 pt-1">
                        {layer.files.length === 0 ? (
                          <span className="text-xs text-slate-600 italic">Keine Dateien zugeordnet</span>
                        ) : (
                          layer.files.map(f => (
                            <button
                              key={f}
                              onClick={() => onJumpToFileAndLine(f, 1)}
                              className="px-2.5 py-1 rounded bg-white/5 hover:bg-white/10 text-slate-300 text-xs font-mono border border-white/5 flex items-center gap-1.5 transition-colors"
                            >
                              <FileCode className="w-3 h-3 text-cyan-400" />
                              <span>{f}</span>
                            </button>
                          ))
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* ASCII Architecture Diagram */}
              <div className="p-5 rounded-2xl bg-black/60 border border-white/10 space-y-3">
                <div className="flex items-center justify-between">
                  <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider">Live System-Topologie-Diagramm</h4>
                  <span className="text-[10px] text-slate-500 font-mono">Stand: {liveWiki.lastUpdated}</span>
                </div>
                <pre className="p-4 rounded-xl bg-black/90 font-mono text-[11px] text-cyan-300 overflow-x-auto whitespace-pre leading-relaxed border border-white/5">
                  {liveWiki.asciiDiagram}
                </pre>
              </div>

              {/* Module Catalog Table */}
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 space-y-3">
                <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider">Modul-Katalog & API-Schnittstellen</h4>
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-xs">
                    <thead>
                      <tr className="border-b border-white/10 text-slate-400 font-mono">
                        <th className="pb-2">Modul</th>
                        <th className="pb-2">Rolle</th>
                        <th className="pb-2">Zeilen</th>
                        <th className="pb-2">Öffentliche Exports</th>
                        <th className="pb-2">Sicherheitsstatus</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-white/5">
                      {liveWiki.modules.map((mod) => (
                        <tr key={mod.fileName} className="hover:bg-white/[0.02]">
                          <td className="py-2.5 font-mono text-cyan-400 font-semibold">{mod.fileName}</td>
                          <td className="py-2.5 text-slate-300">{mod.role}</td>
                          <td className="py-2.5 font-mono text-slate-400">{mod.lines}</td>
                          <td className="py-2.5 font-mono text-slate-300">
                            {mod.exports.length > 0 ? mod.exports.join(', ') : '-'}
                          </td>
                          <td className="py-2.5">
                            {mod.securityRating === 'Secure' ? (
                              <span className="text-emerald-400 font-semibold flex items-center gap-1">
                                <ShieldCheck className="w-3 h-3" /> Sicher
                              </span>
                            ) : mod.securityRating === 'Needs Review' ? (
                              <span className="text-amber-400 font-semibold flex items-center gap-1">
                                <AlertTriangle className="w-3 h-3" /> Review nötig
                              </span>
                            ) : (
                              <span className="text-red-400 font-semibold flex items-center gap-1">
                                <AlertOctagon className="w-3 h-3" /> Verwundbar
                              </span>
                            )}
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            </motion.div>
          )}

          {/* ========================================================================= */}
          {/* TAB 9: LIVE ROADMAP & MEILENSTEINE                                        */}
          {/* ========================================================================= */}
          {activeTab === 'roadmap' && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-6">
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                <div>
                  <h3 className="text-base font-bold text-slate-100 flex items-center gap-2">
                    <Map className="w-5 h-5 text-indigo-400" />
                    Live Projekt-Roadmap (Fortschritt: {roadmapDoc.overallProgress}%)
                  </h3>
                  <p className="text-xs text-slate-400 mt-1">
                    Aktiver Fokus: <strong className="text-indigo-300">{roadmapDoc.activeMilestone}</strong> • Geplanter Release: <strong className="text-slate-300">{roadmapDoc.estimatedReleaseDate}</strong>
                  </p>
                </div>

                <div className="flex items-center gap-2 shrink-0">
                  <button
                    onClick={() => {
                      const updated = syncRoadmapFileInWorkspace(files, roadmapDoc);
                      setFiles(updated);
                      onShowToast("'ROADMAP.md' erfolgreich im Workspace synchronisiert!", 'info');
                    }}
                    className="px-4 py-2 bg-gradient-to-r from-indigo-600 to-blue-600 hover:from-indigo-500 hover:to-blue-500 text-white font-bold text-xs rounded-xl shadow-lg flex items-center gap-2 cursor-pointer"
                  >
                    <Download className="w-4 h-4" />
                    <span>In ROADMAP.md synchronisieren</span>
                  </button>
                </div>
              </div>

              {/* Overall Progress Meter */}
              <div className="p-4 rounded-xl bg-indigo-950/20 border border-indigo-500/30 flex flex-col md:flex-row items-center justify-between gap-4">
                <div className="space-y-1">
                  <span className="text-xs text-slate-400">Gesamtrealisierungsgrad aller 5 Meilenstein-Phasen</span>
                  <div className="text-xl font-bold font-mono text-white flex items-center gap-2">
                    <span>{roadmapDoc.overallProgress}% Abgeschlossen</span>
                    <span className="text-xs font-normal text-indigo-400 font-mono">({roadmapDoc.phases.filter(p => p.progress === 100).length}/{roadmapDoc.phases.length} Phasen fertig)</span>
                  </div>
                </div>
                <div className="w-full md:w-72 bg-black/50 h-3.5 rounded-full overflow-hidden border border-white/10">
                  <div
                    className="h-full bg-gradient-to-r from-indigo-500 via-cyan-400 to-emerald-400 transition-all duration-500"
                    style={{ width: `${roadmapDoc.overallProgress}%` }}
                  ></div>
                </div>
              </div>

              {/* Phases Grid */}
              <div className="space-y-4">
                {roadmapDoc.phases.map(phase => (
                  <div key={phase.id} className="p-5 rounded-xl bg-black/40 border border-white/10 space-y-4">
                    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-white/5 pb-3">
                      <div className="flex items-center gap-2.5">
                        <span className="px-2.5 py-0.5 rounded text-xs font-mono font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
                          {phase.badge}
                        </span>
                        <h4 className="text-sm font-bold text-white">{phase.title}</h4>
                      </div>
                      <div className="flex items-center gap-3">
                        <div className="w-32 bg-black/40 h-2 rounded-full overflow-hidden border border-white/5">
                          <div
                            className="h-full bg-indigo-500"
                            style={{ width: `${phase.progress}%` }}
                          ></div>
                        </div>
                        <span className="text-xs font-bold font-mono text-cyan-400">{phase.progress}%</span>
                      </div>
                    </div>
                    <p className="text-xs text-slate-400 leading-relaxed">{phase.description}</p>

                    {/* Milestones in phase */}
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-3 pt-1">
                      {phase.milestones.map(m => (
                        <div
                          key={m.id}
                          className={`p-3 rounded-lg border text-xs space-y-1 transition-all ${
                            m.completed
                              ? 'bg-emerald-950/10 border-emerald-500/30'
                              : 'bg-white/[0.02] border-white/5'
                          }`}
                        >
                          <div className="flex items-center justify-between">
                            <div className="flex items-center gap-2">
                              {m.completed ? (
                                <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                              ) : (
                                <Clock className="w-3.5 h-3.5 text-slate-500 shrink-0" />
                              )}
                              <span className={`font-semibold ${m.completed ? 'text-slate-200 line-through opacity-80' : 'text-white'}`}>
                                {m.title}
                              </span>
                            </div>
                            <span className="text-[10px] font-mono text-indigo-400">{m.targetDate}</span>
                          </div>
                          <p className="text-[11px] text-slate-500 pl-5.5">{m.details}</p>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </motion.div>
          )}

          {/* ========================================================================= */}
          {/* TAB 10: LIVE SPRINTS & BURNDOWN                                           */}
          {/* ========================================================================= */}
          {activeTab === 'sprints' && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-6">
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                <div>
                  <h3 className="text-base font-bold text-slate-100 flex items-center gap-2">
                    <TrendingUp className="w-5 h-5 text-fuchsia-400" />
                    Sprint #{sprintDoc.sprintNumber}: {sprintDoc.sprintName}
                  </h3>
                  <p className="text-xs text-slate-400 mt-1">
                    Ziel: <span className="text-slate-200 font-medium">{sprintDoc.goal}</span> • Laufzeit: <span className="text-fuchsia-300 font-mono">{sprintDoc.startDate} bis {sprintDoc.endDate}</span>
                  </p>
                </div>

                <div className="flex items-center gap-2 shrink-0">
                  <button
                    onClick={() => {
                      const updated = syncSprintFileInWorkspace(files, sprintDoc);
                      setFiles(updated);
                      onShowToast("'SPRINTS.md' erfolgreich im Workspace synchronisiert!", 'info');
                    }}
                    className="px-4 py-2 bg-gradient-to-r from-fuchsia-600 to-purple-600 hover:from-fuchsia-500 hover:to-purple-500 text-white font-bold text-xs rounded-xl shadow-lg flex items-center gap-2 cursor-pointer"
                  >
                    <Download className="w-4 h-4" />
                    <span>In SPRINTS.md synchronisieren</span>
                  </button>
                </div>
              </div>

              {/* Sprint Metrics Bar */}
              <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                <div className="p-4 rounded-xl bg-black/40 border border-white/10 space-y-1">
                  <span className="text-xs text-slate-400">Burndown / Erfüllung</span>
                  <div className="flex items-center justify-between">
                    <span className="text-2xl font-bold font-mono text-fuchsia-400">{sprintDoc.burndownPercentage}%</span>
                    <span className="text-xs text-slate-500 font-mono">{sprintDoc.completedPoints} / {sprintDoc.totalPoints} SP</span>
                  </div>
                  <div className="w-full bg-black/60 h-2 rounded-full overflow-hidden border border-white/5 mt-2">
                    <div
                      className="h-full bg-gradient-to-r from-fuchsia-500 to-purple-400"
                      style={{ width: `${sprintDoc.burndownPercentage}%` }}
                    ></div>
                  </div>
                </div>

                <div className="p-4 rounded-xl bg-black/40 border border-white/10 space-y-1">
                  <span className="text-xs text-slate-400">Team Velocity & Kapazität</span>
                  <div className="text-2xl font-bold font-mono text-cyan-400">{sprintDoc.velocity} SP / Sprint</div>
                  <p className="text-[11px] text-slate-500">Geschätzte Arbeitsleistung pro Zyklus</p>
                </div>

                <div className="p-4 rounded-xl bg-black/40 border border-white/10 space-y-1">
                  <span className="text-xs text-slate-400">Offene Sprint-Stories</span>
                  <div className="text-2xl font-bold font-mono text-amber-400">
                    {sprintDoc.stories.filter(s => s.status !== 'done').length} / {sprintDoc.stories.length}
                  </div>
                  <p className="text-[11px] text-slate-500">Inklusive automatischer Audit-Prioritäten</p>
                </div>
              </div>

              {/* Stories Table */}
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 space-y-3">
                <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider">Sprint-Backlog & User Stories</h4>
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-xs">
                    <thead>
                      <tr className="border-b border-white/10 text-slate-400 font-mono">
                        <th className="pb-2">ID</th>
                        <th className="pb-2">User Story / Aufgabe</th>
                        <th className="pb-2">Story Points</th>
                        <th className="pb-2">Priorität</th>
                        <th className="pb-2">Komponente</th>
                        <th className="pb-2">Status</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-white/5">
                      {sprintDoc.stories.map(story => (
                        <tr key={story.id} className="hover:bg-white/[0.02]">
                          <td className="py-2.5 font-mono text-fuchsia-400 font-bold">{story.id}</td>
                          <td className="py-2.5 font-medium text-slate-200">{story.title}</td>
                          <td className="py-2.5 font-mono font-bold text-amber-300">{story.storyPoints} SP</td>
                          <td className="py-2.5">
                            <span className={`px-2 py-0.5 rounded text-[10px] font-bold uppercase ${
                              story.priority === 'critical' ? 'bg-red-500/20 text-red-300' :
                              story.priority === 'high' ? 'bg-amber-500/20 text-amber-300' :
                              story.priority === 'medium' ? 'bg-blue-500/20 text-blue-300' :
                              'bg-slate-500/20 text-slate-400'
                            }`}>
                              {story.priority}
                            </span>
                          </td>
                          <td className="py-2.5 text-slate-400 font-mono text-[11px]">{story.component}</td>
                          <td className="py-2.5">
                            <span className={`px-2 py-0.5 rounded text-[10px] font-bold uppercase ${
                              story.status === 'done' ? 'bg-emerald-500/20 text-emerald-300' :
                              story.status === 'in_progress' ? 'bg-cyan-500/20 text-cyan-300' :
                              'bg-slate-500/20 text-slate-400'
                            }`}>
                              {story.status}
                            </span>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            </motion.div>
          )}

          {/* ========================================================================= */}
          {/* TAB 11: LIVE DOKUMENTATION & INDEX                                        */}
          {/* ========================================================================= */}
          {activeTab === 'docs' && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-6">
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                <div>
                  <h3 className="text-base font-bold text-slate-100 flex items-center gap-2">
                    <FileText className="w-5 h-5 text-emerald-400" />
                    Live Projekt-Dokumentation ({liveDocs.indexedFiles.length} indexierte Dateien)
                  </h3>
                  <p className="text-xs text-slate-400 mt-1">
                    Erstellt und synchronisiert automatisch den ATC-DOC Index, die API-Referenz ({liveDocs.apiEntries.length} Symbole) und die Changelog-Historie.
                  </p>
                </div>

                <button
                  onClick={() => {
                    const currentVersionStr = `${currentVersion.major}.${currentVersion.minor}.${currentVersion.patch}`;
                    const updated = syncDocumentationFilesInWorkspace(files, liveDocs, currentVersionStr);
                    setFiles(updated);
                    onShowToast("Dokumentations-Artefakte (docs/DOCUMENTATION_INDEX.md etc.) synchronisiert!", 'info');
                  }}
                  className="px-4 py-2 bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white font-bold text-xs rounded-xl shadow-lg flex items-center gap-2 shrink-0 cursor-pointer"
                >
                  <Download className="w-4 h-4" />
                  <span>In docs/* synchronisieren</span>
                </button>
              </div>

              {/* API Reference Table */}
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 space-y-3">
                <div className="flex items-center justify-between">
                  <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider">
                    Automatisch extrahierte API-Schnittstellen ({liveDocs.apiEntries.length})
                  </h4>
                  <span className="text-[11px] text-slate-500 font-mono">docs/API_REFERENCE.md</span>
                </div>
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-xs">
                    <thead>
                      <tr className="border-b border-white/10 text-slate-400 font-mono">
                        <th className="pb-2">Symbol / Funktion</th>
                        <th className="pb-2">Signatur</th>
                        <th className="pb-2">Rückgabetyp</th>
                        <th className="pb-2">Quelldatei</th>
                        <th className="pb-2">Aktion</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-white/5 font-mono">
                      {liveDocs.apiEntries.slice(0, 30).map((api, idx) => (
                        <tr key={idx} className="hover:bg-white/[0.02]">
                          <td className="py-2 text-cyan-300 font-bold">{api.name}</td>
                          <td className="py-2 text-slate-300 font-sans text-xs">{api.signature}</td>
                          <td className="py-2 text-amber-300">{api.returnType}</td>
                          <td className="py-2 text-slate-400 text-[11px]">{api.fileName}:{api.line}</td>
                          <td className="py-2">
                            <button
                              onClick={() => onJumpToFileAndLine(api.fileName, api.line)}
                              className="px-2 py-0.5 rounded bg-white/5 hover:bg-white/10 text-slate-300 text-[11px] transition-colors"
                            >
                              Springen
                            </button>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>

              {/* Indexed Files Grid */}
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 space-y-3">
                <h4 className="text-xs font-bold text-slate-300 uppercase tracking-wider">
                  Indexierte Projektdateien & Modulrollen
                </h4>
                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3">
                  {liveDocs.indexedFiles.map(file => (
                    <div
                      key={file.fileName}
                      onClick={() => onJumpToFileAndLine(file.fileName, 1)}
                      className="p-3 rounded-xl bg-black/30 border border-white/5 hover:border-cyan-500/30 transition-all cursor-pointer space-y-1"
                    >
                      <div className="flex items-center justify-between">
                        <span className="font-mono text-xs font-bold text-cyan-300 truncate">{file.fileName}</span>
                        <span className="text-[10px] text-slate-500 font-mono">{file.lines} Zeilen</span>
                      </div>
                      <p className="text-[11px] text-slate-400">{file.purpose}</p>
                    </div>
                  ))}
                </div>
              </div>
            </motion.div>
          )}

          {/* ========================================================================= */}
          {/* TAB 12: LIVE VERSION & SEMVER                                             */}
          {/* ========================================================================= */}
          {activeTab === 'version' && (
            <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} className="space-y-6">
              <div className="p-5 rounded-2xl bg-black/40 border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                <div>
                  <h3 className="text-base font-bold text-slate-100 flex items-center gap-2">
                    <Tag className="w-5 h-5 text-cyan-400" />
                    Live Versionierung & Semantic Versioning (SemVer)
                  </h3>
                  <p className="text-xs text-slate-400 mt-1">
                    Aktuelle Version: <strong className="text-cyan-300 font-mono">v{currentVersion.major}.{currentVersion.minor}.{currentVersion.patch}</strong> (Build #{currentVersion.build})
                  </p>
                </div>

                <div className="flex items-center gap-2 shrink-0">
                  <button
                    onClick={() => {
                      const { updatedFiles } = syncVersionFileInWorkspace(files, currentVersion);
                      setFiles(updatedFiles);
                      onShowToast("VERSION und package.json erfolgreich im Workspace synchronisiert!", 'info');
                    }}
                    className="px-4 py-2 bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white font-bold text-xs rounded-xl shadow-lg flex items-center gap-2 cursor-pointer"
                  >
                    <Download className="w-4 h-4" />
                    <span>In VERSION synchronisieren</span>
                  </button>
                </div>
              </div>

              {/* Version Controls */}
              <div className="p-6 rounded-2xl bg-gradient-to-r from-cyan-950/30 via-black/40 to-blue-950/30 border border-cyan-500/30 space-y-4">
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                  <div>
                    <span className="text-xs text-slate-400 uppercase tracking-wider">Erkannte Empfehlung</span>
                    <div className="text-lg font-bold text-amber-300 flex items-center gap-2 mt-0.5">
                      <Sparkles className="w-4 h-4" />
                      Empfohlener Bump: {currentVersion.recommendedBump.toUpperCase()}
                    </div>
                    <p className="text-xs text-slate-400 mt-1">{currentVersion.recommendedReason}</p>
                  </div>

                  <div className="flex items-center gap-2 flex-wrap">
                    <button
                      onClick={() => {
                        const { updatedFiles, newVersionString } = syncAllProjectArtifacts(files, autoSyncConfig || {
                          autoSyncWiki: true,
                          autoSyncRoadmap: true,
                          autoSyncTodo: true,
                          autoSyncSprints: true,
                          autoSyncDocs: true,
                          autoSyncVersion: true,
                        }, { versionBumpType: 'patch', forceAll: true });
                        setFiles(updatedFiles);
                        onShowToast(`Patch-Release erstellt: v${newVersionString}`, 'info');
                      }}
                      className="px-3.5 py-2 bg-white/10 hover:bg-white/20 text-white text-xs font-bold rounded-xl transition-all"
                    >
                      Patch (+0.0.1)
                    </button>
                    <button
                      onClick={() => {
                        const { updatedFiles, newVersionString } = syncAllProjectArtifacts(files, autoSyncConfig || {
                          autoSyncWiki: true,
                          autoSyncRoadmap: true,
                          autoSyncTodo: true,
                          autoSyncSprints: true,
                          autoSyncDocs: true,
                          autoSyncVersion: true,
                        }, { versionBumpType: 'minor', forceAll: true });
                        setFiles(updatedFiles);
                        onShowToast(`Minor-Release erstellt: v${newVersionString}`, 'info');
                      }}
                      className="px-3.5 py-2 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 border border-indigo-500/40 text-xs font-bold rounded-xl transition-all"
                    >
                      Minor (+0.1.0)
                    </button>
                    <button
                      onClick={() => {
                        const { updatedFiles, newVersionString } = syncAllProjectArtifacts(files, autoSyncConfig || {
                          autoSyncWiki: true,
                          autoSyncRoadmap: true,
                          autoSyncTodo: true,
                          autoSyncSprints: true,
                          autoSyncDocs: true,
                          autoSyncVersion: true,
                        }, { versionBumpType: 'major', forceAll: true });
                        setFiles(updatedFiles);
                        onShowToast(`Major-Release erstellt: v${newVersionString}`, 'info');
                      }}
                      className="px-3.5 py-2 bg-purple-500/20 hover:bg-purple-500/30 text-purple-300 border border-purple-500/40 text-xs font-bold rounded-xl transition-all"
                    >
                      Major (+1.0.0)
                    </button>
                  </div>
                </div>
              </div>
            </motion.div>
          )}

        </div>
      </div>

      {/* ========================================================================= */}
      {/* MODAL: Neues TODO anlegen                                                 */}
      {/* ========================================================================= */}
      <AnimatePresence>
        {isNewTodoOpen && (
          <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="w-full max-w-md bg-[#121217] border border-white/10 rounded-2xl shadow-2xl p-6 space-y-4"
            >
              <h3 className="text-sm font-bold text-slate-100 flex items-center gap-2">
                <ListTodo className="w-4 h-4 text-amber-400" />
                Neues TODO in Quelldatei einfügen
              </h3>

              <div className="space-y-3 text-xs">
                <div>
                  <label className="text-slate-400 block mb-1">Zieldatei:</label>
                  <select
                    value={newTodoFile}
                    onChange={(e) => setNewTodoFile(e.target.value)}
                    className="w-full bg-black/60 border border-white/10 rounded-lg px-3 py-2 text-slate-200 text-xs focus:outline-none"
                  >
                    {files.map(f => (
                      <option key={f.name} value={f.name}>{f.name}</option>
                    ))}
                  </select>
                </div>

                <div>
                  <label className="text-slate-400 block mb-1">Tag-Typ:</label>
                  <select
                    value={newTodoTag}
                    onChange={(e) => setNewTodoTag(e.target.value as any)}
                    className="w-full bg-black/60 border border-white/10 rounded-lg px-3 py-2 text-slate-200 text-xs focus:outline-none"
                  >
                    <option value="TODO">TODO (Standard-Aufgabe)</option>
                    <option value="FIXME">FIXME (Zu behebender Fehler)</option>
                    <option value="BUG">BUG (Kritischer Bug)</option>
                    <option value="HACK">HACK (Workaround / Refactoring)</option>
                  </select>
                </div>

                <div>
                  <label className="text-slate-400 block mb-1">Beschreibung:</label>
                  <textarea
                    rows={3}
                    placeholder="z.B. Eingabevalidierung für Transaktionsgebühren implementieren..."
                    value={newTodoText}
                    onChange={(e) => setNewTodoText(e.target.value)}
                    className="w-full bg-black/60 border border-white/10 rounded-lg px-3 py-2 text-slate-200 text-xs focus:outline-none focus:border-amber-500"
                  />
                </div>
              </div>

              <div className="flex justify-end gap-2 pt-2">
                <button
                  onClick={() => setIsNewTodoOpen(false)}
                  className="px-3.5 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-slate-400 text-xs transition-colors"
                >
                  Abbrechen
                </button>
                <button
                  onClick={handleAddNewTodo}
                  disabled={!newTodoText.trim()}
                  className="px-4 py-1.5 rounded-lg bg-amber-500 hover:bg-amber-400 text-black font-bold text-xs transition-colors disabled:opacity-50"
                >
                  In Code einfügen
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
    </div>
  );
}
