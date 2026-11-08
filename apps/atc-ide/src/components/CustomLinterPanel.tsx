import React, { useState, useMemo } from "react";
import {
  ShieldAlert,
  Sliders,
  Plus,
  Search,
  Trash2,
  Edit3,
  Copy,
  RotateCcw,
  FileCode,
  CheckCircle2,
  AlertTriangle,
  XCircle,
  Lightbulb,
  Download,
  Upload,
  Code2,
  Sparkles,
  Save,
  Check,
  X,
  Play,
  FileCheck,
  Zap,
} from "lucide-react";
import {
  CustomLintRule,
  CustomLintError,
  DEFAULT_LINT_RULES,
  evaluateCustomLintRules,
  LintSeverity,
  LintPatternType,
} from "../types/linter";
import { FileState, LintError } from "../App";

interface CustomLinterPanelProps {
  rules: CustomLintRule[];
  onUpdateRules: (newRules: CustomLintRule[]) => void;
  currentFile?: FileState;
  allFiles: FileState[];
  onSaveWorkspaceFile?: (filename: string, content: string) => void;
  onJumpToLine?: (line: number) => void;
  onApplyQuickFix?: (fix: { label: string; action: string; replacement?: string }, line: number, match: string) => void;
  currentDiagnostics?: LintError[];
}

export function CustomLinterPanel({
  rules,
  onUpdateRules,
  currentFile,
  allFiles,
  onSaveWorkspaceFile,
  onJumpToLine,
  onApplyQuickFix,
  currentDiagnostics = [],
}: CustomLinterPanelProps) {
  const [activeTab, setActiveTab] = useState<"rules" | "diagnostics" | "sandbox">("rules");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategory, setSelectedCategory] = useState<string>("all");
  const [severityFilter, setSeverityFilter] = useState<"all" | "error" | "warning">("all");
  const [editingRule, setEditingRule] = useState<CustomLintRule | null>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [sandboxCode, setSandboxCode] = useState<string>(
    currentFile?.content ||
`// Test your custom linting rules here
var legacyVar = 10;
const invalid_constant = 42;
// Refactor this loop later
while (true) {
  print "Running unsafe execution"
  unsafe {
    eval("system_reset");
  }
}
let uninitialized;
fn calculateTotal(amount) {
  return amount * 1.19;;
}
`
  );

  // Statistics
  const stats = useMemo(() => {
    const total = rules.length;
    const active = rules.filter((r) => r.enabled).length;
    const errorsCount = currentDiagnostics.filter((d) => d.type === "error").length;
    const warningsCount = currentDiagnostics.filter((d) => d.type === "warning").length;
    return { total, active, errorsCount, warningsCount };
  }, [rules, currentDiagnostics]);

  // Categories list
  const categories = ["all", "Style", "Best Practices", "Security", "Formatting", "Custom"];

  // Filtered rules
  const filteredRules = useMemo(() => {
    return rules.filter((rule) => {
      const matchesSearch =
        rule.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        rule.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (rule.pattern && rule.pattern.toLowerCase().includes(searchQuery.toLowerCase()));

      const matchesCat =
        selectedCategory === "all" || rule.category === selectedCategory;

      const matchesSev =
        severityFilter === "all" || rule.severity === severityFilter;

      return matchesSearch && matchesCat && matchesSev;
    });
  }, [rules, searchQuery, selectedCategory, severityFilter]);

  // Toggle single rule
  const handleToggleRule = (id: string) => {
    const updated = rules.map((r) => (r.id === id ? { ...r, enabled: !r.enabled } : r));
    onUpdateRules(updated);
  };

  // Delete rule
  const handleDeleteRule = (id: string) => {
    const updated = rules.filter((r) => r.id !== id);
    onUpdateRules(updated);
  };

  // Duplicate rule
  const handleDuplicateRule = (rule: CustomLintRule) => {
    const newRule: CustomLintRule = {
      ...rule,
      id: `rule-custom-${Date.now()}`,
      name: `${rule.name} (Kopie)`,
      isPreset: false,
    };
    onUpdateRules([...rules, newRule]);
  };

  // Reset to default presets
  const handleResetDefaults = () => {
    if (window.confirm("Möchtest du die Linting-Regeln wirklich auf die Standard-Vorlagen zurücksetzen?")) {
      onUpdateRules(DEFAULT_LINT_RULES);
    }
  };

  // Export rules to JSON
  const handleExportJson = () => {
    const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(rules, null, 2));
    const downloadAnchor = document.createElement("a");
    downloadAnchor.setAttribute("href", dataStr);
    downloadAnchor.setAttribute("download", "lumino-lint-rules.json");
    document.body.appendChild(downloadAnchor);
    downloadAnchor.click();
    downloadAnchor.remove();
  };

  // Save to .luminolint.json in workspace
  const handleSaveToWorkspace = () => {
    if (onSaveWorkspaceFile) {
      const content = JSON.stringify(rules, null, 2);
      onSaveWorkspaceFile(".luminolint.json", content);
    }
  };

  // Import rules from JSON
  const handleImportJson = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const parsed = JSON.parse(event.target?.result as string);
        if (Array.isArray(parsed)) {
          onUpdateRules(parsed);
          alert(`Erfolgreich ${parsed.length} Lint-Regeln importiert!`);
        } else {
          alert("Ungültiges Format: Die JSON-Datei muss ein Array von Regeln sein.");
        }
      } catch (err) {
        alert("Fehler beim Parsen der JSON-Datei: " + err);
      }
    };
    reader.readAsText(file);
    e.target.value = "";
  };

  // Start new rule
  const handleStartNewRule = () => {
    const newRuleTemplate: CustomLintRule = {
      id: `rule-custom-${Date.now()}`,
      name: "Neue Lumino Lint-Regel",
      description: "Prüft auf unerwünschte Muster im Quellcode.",
      enabled: true,
      severity: "warning",
      category: "Custom",
      patternType: "regex",
      pattern: "\\bprint\\s*;",
      flags: "g",
      errorMessage: "Unvollständiges print-Statement erkannt.",
      quickFix: {
        label: "Entfernen",
        actionType: "replace",
        replacement: "",
      },
      isPreset: false,
    };
    setEditingRule(newRuleTemplate);
    setIsCreating(true);
  };

  // Save rule from editor modal
  const handleSaveRuleEdit = (savedRule: CustomLintRule) => {
    if (isCreating) {
      onUpdateRules([...rules, savedRule]);
    } else {
      onUpdateRules(rules.map((r) => (r.id === savedRule.id ? savedRule : r)));
    }
    setEditingRule(null);
    setIsCreating(false);
  };

  // Sandbox evaluation
  const sandboxErrors = useMemo(() => {
    return evaluateCustomLintRules(sandboxCode, rules);
  }, [sandboxCode, rules]);

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0c0c0e] text-slate-200 overflow-hidden">
      {/* Top Banner Header */}
      <div className="p-4 border-b border-white/10 flex items-center justify-between bg-black/40 shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2.5 bg-gradient-to-br from-amber-500/20 to-orange-500/20 border border-amber-500/30 rounded-xl text-amber-400">
            <ShieldAlert className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-lg font-bold text-slate-100">
                Lumino Linter & Benutzerdefinierte Regeln
              </h2>
              <span className="px-2 py-0.5 rounded-full text-[11px] font-mono bg-amber-500/15 border border-amber-500/30 text-amber-300 font-semibold">
                {stats.active} / {stats.total} Aktiv
              </span>
            </div>
            <p className="text-xs text-slate-400 mt-0.5">
              Definiere eigene Prüfregeln, Naming-Conventions und Quick-Fixes, die live parallel zur Standard-Analyse ausgeführt werden.
            </p>
          </div>
        </div>

        {/* Global Action Buttons */}
        <div className="flex items-center gap-2">
          <button
            onClick={handleSaveToWorkspace}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 transition-colors cursor-pointer"
            title="Als .luminolint.json im Projekt-Workspace speichern"
          >
            <Save className="w-3.5 h-3.5 text-cyan-400" />
            <span>.luminolint.json</span>
          </button>

          <button
            onClick={handleExportJson}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 transition-colors cursor-pointer"
            title="Regelwerk als JSON exportieren"
          >
            <Download className="w-3.5 h-3.5 text-indigo-400" />
            <span>Export</span>
          </button>

          <label
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 transition-colors cursor-pointer"
            title="Regeln aus JSON importieren"
          >
            <Upload className="w-3.5 h-3.5 text-emerald-400" />
            <span>Import</span>
            <input type="file" accept=".json" onChange={handleImportJson} className="hidden" />
          </label>

          <button
            onClick={handleResetDefaults}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-white/5 hover:bg-white/10 text-slate-400 hover:text-slate-200 border border-white/10 transition-colors cursor-pointer"
            title="Auf Standard-Vorlagen zurücksetzen"
          >
            <RotateCcw className="w-3.5 h-3.5" />
            <span>Presets</span>
          </button>

          <button
            onClick={handleStartNewRule}
            className="flex items-center gap-1.5 px-3.5 py-1.5 bg-gradient-to-r from-amber-600 to-orange-600 hover:from-amber-500 hover:to-orange-500 text-white rounded-lg text-xs font-bold shadow-lg shadow-amber-600/20 transition-all cursor-pointer ml-1"
          >
            <Plus className="w-4 h-4" />
            <span>Neue Regel</span>
          </button>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="px-6 py-2 bg-black/20 border-b border-white/5 flex items-center justify-between shrink-0">
        <div className="flex items-center gap-2">
          <button
            onClick={() => setActiveTab("rules")}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer ${
              activeTab === "rules"
                ? "bg-amber-500/20 text-amber-300 border border-amber-500/30"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            <Sliders className="w-3.5 h-3.5" />
            <span>Regelwerk ({rules.length})</span>
          </button>

          <button
            onClick={() => setActiveTab("diagnostics")}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer ${
              activeTab === "diagnostics"
                ? "bg-amber-500/20 text-amber-300 border border-amber-500/30"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            <FileCheck className="w-3.5 h-3.5" />
            <span>Aktuelle Datei-Diagnose</span>
            {stats.errorsCount + stats.warningsCount > 0 && (
              <span className="flex items-center gap-1 px-1.5 py-0.2 rounded text-[10px] bg-red-500/20 text-red-300 border border-red-500/30 font-mono">
                {stats.errorsCount}E • {stats.warningsCount}W
              </span>
            )}
          </button>

          <button
            onClick={() => setActiveTab("sandbox")}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer ${
              activeTab === "sandbox"
                ? "bg-amber-500/20 text-amber-300 border border-amber-500/30"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            <Code2 className="w-3.5 h-3.5" />
            <span>Regel-Tester / Sandbox</span>
          </button>
        </div>

        {/* Live status badge */}
        <div className="flex items-center gap-3 text-xs text-slate-400">
          <span className="flex items-center gap-1.5">
            <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
            Live-Inspektion im Editor aktiv
          </span>
          {currentFile && (
            <span className="font-mono text-[11px] text-slate-500">
              Datei: {currentFile.name}
            </span>
          )}
        </div>
      </div>

      {/* Main Tab Views */}
      <div className="flex-1 overflow-hidden flex flex-col">
        {/* TAB 1: RULES MANAGER */}
        {activeTab === "rules" && (
          <div className="flex-1 flex flex-col overflow-hidden">
            {/* Filter Bar */}
            <div className="p-4 border-b border-white/5 bg-black/10 flex flex-wrap items-center justify-between gap-3 shrink-0">
              <div className="flex items-center gap-2 flex-1 max-w-md">
                <div className="relative w-full">
                  <Search className="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
                  <input
                    type="text"
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    placeholder="Regeln durchsuchen (Name, Pattern, Beschreibung)..."
                    className="w-full bg-black/40 border border-white/10 rounded-lg pl-9 pr-3 py-1.5 text-xs text-slate-200 outline-none focus:border-amber-500/50"
                  />
                </div>
              </div>

              {/* Category Pills */}
              <div className="flex items-center gap-1.5 flex-wrap">
                {categories.map((cat) => (
                  <button
                    key={cat}
                    onClick={() => setSelectedCategory(cat)}
                    className={`px-2.5 py-1 rounded-md text-xs font-semibold transition-colors cursor-pointer ${
                      selectedCategory === cat
                        ? "bg-amber-500/20 text-amber-300 border border-amber-500/40"
                        : "bg-white/5 text-slate-400 hover:text-slate-200 hover:bg-white/10 border border-white/5"
                    }`}
                  >
                    {cat === "all" ? "Alle Kategorien" : cat}
                  </button>
                ))}

                <div className="w-px h-4 bg-white/10 mx-1"></div>

                {/* Severity Filter */}
                <button
                  onClick={() =>
                    setSeverityFilter(
                      severityFilter === "all"
                        ? "error"
                        : severityFilter === "error"
                        ? "warning"
                        : "all"
                    )
                  }
                  className={`px-2.5 py-1 rounded-md text-xs font-semibold border transition-colors cursor-pointer ${
                    severityFilter === "error"
                      ? "bg-red-500/20 text-red-300 border-red-500/40"
                      : severityFilter === "warning"
                      ? "bg-amber-500/20 text-amber-300 border-amber-500/40"
                      : "bg-white/5 text-slate-400 border-white/5"
                  }`}
                >
                  {severityFilter === "all"
                    ? "Alle Schweregrade"
                    : severityFilter === "error"
                    ? "Nur Fehler"
                    : "Nur Warnungen"}
                </button>
              </div>
            </div>

            {/* Rules Cards List */}
            <div className="flex-1 overflow-y-auto p-6 space-y-3 custom-scrollbar">
              {filteredRules.length === 0 ? (
                <div className="text-center py-16 text-slate-500 bg-white/[0.01] rounded-2xl border border-dashed border-white/10">
                  <ShieldAlert className="w-10 h-10 mx-auto mb-3 opacity-30 text-amber-400" />
                  <p className="text-sm font-semibold text-slate-400">Keine Linting-Regeln gefunden</p>
                  <p className="text-xs text-slate-500 mt-1 max-w-sm mx-auto">
                    Passe deinen Filter an oder erstelle eine neue eigene Regel für Lumino-Quellcode.
                  </p>
                  <button
                    onClick={handleStartNewRule}
                    className="mt-4 px-4 py-2 bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 rounded-lg text-xs font-bold transition-colors cursor-pointer"
                  >
                    + Neue Regel definieren
                  </button>
                </div>
              ) : (
                filteredRules.map((rule) => (
                  <div
                    key={rule.id}
                    className={`bg-black/30 border rounded-xl p-4 transition-all duration-150 flex flex-col md:flex-row md:items-center justify-between gap-4 ${
                      rule.enabled
                        ? "border-white/10 hover:border-amber-500/40"
                        : "border-white/5 opacity-60 bg-black/10"
                    }`}
                  >
                    <div className="flex items-start gap-3 flex-1 min-w-0">
                      {/* Active toggle switch */}
                      <button
                        onClick={() => handleToggleRule(rule.id)}
                        className={`mt-1 w-9 h-5 rounded-full p-0.5 transition-colors cursor-pointer flex items-center shrink-0 ${
                          rule.enabled ? "bg-amber-500 justify-end" : "bg-white/10 justify-start"
                        }`}
                        title={rule.enabled ? "Regel deaktivieren" : "Regel aktivieren"}
                      >
                        <span className="w-4 h-4 rounded-full bg-white shadow-sm block" />
                      </button>

                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2 flex-wrap mb-1">
                          <h4 className="text-sm font-bold text-slate-100 truncate">
                            {rule.name}
                          </h4>

                          {/* Severity Pill */}
                          <span
                            className={`px-2 py-0.5 rounded text-[10px] font-bold uppercase font-mono border ${
                              rule.severity === "error"
                                ? "bg-red-500/15 border-red-500/30 text-red-300"
                                : "bg-amber-500/15 border-amber-500/30 text-amber-300"
                            }`}
                          >
                            {rule.severity}
                          </span>

                          {/* Category */}
                          <span className="px-2 py-0.5 rounded text-[10px] bg-white/5 border border-white/10 text-slate-400">
                            {rule.category}
                          </span>

                          {rule.isPreset && (
                            <span className="px-1.5 py-0.5 rounded text-[9px] bg-indigo-500/15 border border-indigo-500/30 text-indigo-300 font-mono">
                              Preset
                            </span>
                          )}

                          {rule.quickFix && (
                            <span className="flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] bg-emerald-500/15 border border-emerald-500/30 text-emerald-300">
                              <Lightbulb className="w-2.5 h-2.5" />
                              QuickFix
                            </span>
                          )}
                        </div>

                        <p className="text-xs text-slate-400 mb-2 leading-relaxed">
                          {rule.description}
                        </p>

                        {/* Pattern Details */}
                        <div className="flex items-center gap-2 flex-wrap text-[11px] font-mono">
                          <span className="px-2 py-0.5 bg-black/50 border border-white/10 rounded text-slate-300">
                            Typ: <strong className="text-amber-400">{rule.patternType}</strong>
                          </span>

                          {rule.pattern && (
                            <span className="px-2 py-0.5 bg-black/50 border border-white/10 rounded text-cyan-300 truncate max-w-xs">
                              Regex: <code className="text-cyan-200">/{rule.pattern}/{rule.flags || "g"}</code>
                            </span>
                          )}

                          {rule.keyword && (
                            <span className="px-2 py-0.5 bg-black/50 border border-white/10 rounded text-fuchsia-300">
                              Keyword: <code>{rule.keyword}</code>
                            </span>
                          )}

                          {rule.maxLength && (
                            <span className="px-2 py-0.5 bg-black/50 border border-white/10 rounded text-purple-300">
                              Max: {rule.maxLength} Zeichen
                            </span>
                          )}

                          {rule.convention && (
                            <span className="px-2 py-0.5 bg-black/50 border border-white/10 rounded text-emerald-300">
                              Konvention: {rule.convention} ({rule.targetEntity})
                            </span>
                          )}

                          {rule.quickFix && (
                            <span className="px-2 py-0.5 bg-emerald-500/10 border border-emerald-500/20 rounded text-emerald-400">
                              Fix: {rule.quickFix.label}
                            </span>
                          )}
                        </div>
                      </div>
                    </div>

                    {/* Actions on Rule */}
                    <div className="flex items-center gap-1.5 self-end md:self-center shrink-0">
                      <button
                        onClick={() => {
                          setEditingRule(rule);
                          setIsCreating(false);
                        }}
                        className="p-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-slate-300 hover:text-white transition-colors cursor-pointer"
                        title="Regel bearbeiten"
                      >
                        <Edit3 className="w-4 h-4" />
                      </button>

                      <button
                        onClick={() => handleDuplicateRule(rule)}
                        className="p-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-slate-300 hover:text-white transition-colors cursor-pointer"
                        title="Regel duplizieren"
                      >
                        <Copy className="w-4 h-4" />
                      </button>

                      <button
                        onClick={() => handleDeleteRule(rule.id)}
                        className="p-1.5 rounded-lg bg-red-500/10 hover:bg-red-500/20 text-red-400 transition-colors cursor-pointer"
                        title="Regel löschen"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>
        )}

        {/* TAB 2: CURRENT FILE DIAGNOSTICS */}
        {activeTab === "diagnostics" && (
          <div className="flex-1 flex flex-col overflow-hidden p-6 space-y-4">
            <div className="flex items-center justify-between bg-black/30 p-4 rounded-xl border border-white/10">
              <div className="flex items-center gap-3">
                <FileCode className="w-5 h-5 text-amber-400" />
                <div>
                  <h3 className="text-sm font-bold text-slate-100">
                    Aktuelle Datei: <span className="text-amber-400 font-mono">{currentFile?.name || "Keine Datei gewählt"}</span>
                  </h3>
                  <p className="text-xs text-slate-400">
                    {currentDiagnostics.length === 0
                      ? "Keine Linting-Verletzungen im aktuellen Quellcode gefunden. Alles sauber!"
                      : `${currentDiagnostics.length} Befund(e) gefunden (Standard-Linter + Benutzerdefinierte Regeln).`}
                  </p>
                </div>
              </div>

              <div className="flex items-center gap-3">
                <div className="flex items-center gap-1.5 font-mono text-xs">
                  <span className="px-2 py-0.5 rounded bg-red-500/15 border border-red-500/30 text-red-300">
                    {stats.errorsCount} Fehler
                  </span>
                  <span className="px-2 py-0.5 rounded bg-amber-500/15 border border-amber-500/30 text-amber-300">
                    {stats.warningsCount} Warnungen
                  </span>
                </div>
              </div>
            </div>

            {/* Diagnostics List */}
            <div className="flex-1 overflow-y-auto space-y-2.5 custom-scrollbar">
              {currentDiagnostics.length === 0 ? (
                <div className="text-center py-20 text-slate-500 bg-white/[0.01] rounded-xl border border-white/5">
                  <CheckCircle2 className="w-12 h-12 mx-auto mb-3 text-emerald-400 opacity-60" />
                  <p className="text-sm font-bold text-slate-300">Keine Warnungen oder Fehler</p>
                  <p className="text-xs text-slate-500 mt-1">
                    Dein Lumino-Quellcode erfüllt alle aktiven Qualitäts- und Style-Regeln.
                  </p>
                </div>
              ) : (
                currentDiagnostics.map((diag, index) => {
                  const lineText = currentFile?.content.split("\n")[diag.line] || "";

                  return (
                    <div
                      key={index}
                      className={`p-3.5 rounded-xl border flex flex-col md:flex-row md:items-center justify-between gap-3 ${
                        diag.type === "error"
                          ? "bg-red-500/5 border-red-500/20 hover:border-red-500/40"
                          : "bg-amber-500/5 border-amber-500/20 hover:border-amber-500/40"
                      }`}
                    >
                      <div className="flex items-start gap-3 flex-1 min-w-0">
                        <div className="mt-0.5 shrink-0">
                          {diag.type === "error" ? (
                            <XCircle className="w-4 h-4 text-red-400" />
                          ) : (
                            <AlertTriangle className="w-4 h-4 text-amber-400" />
                          )}
                        </div>

                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-1 flex-wrap">
                            <span className="px-2 py-0.5 bg-black/40 border border-white/10 rounded font-mono text-[11px] text-slate-300 font-semibold">
                              Zeile {diag.line + 1}
                            </span>
                            <span
                              className={`px-1.5 py-0.2 rounded font-mono text-[10px] font-bold uppercase ${
                                diag.type === "error"
                                  ? "bg-red-500/20 text-red-300"
                                  : "bg-amber-500/20 text-amber-300"
                              }`}
                            >
                              {diag.type}
                            </span>
                            <span className="text-xs font-semibold text-slate-200">
                              {diag.message}
                            </span>
                          </div>

                          {/* Code Preview */}
                          <div className="bg-black/40 px-3 py-1.5 rounded-md border border-white/5 font-mono text-xs text-slate-400 truncate mt-1.5 flex items-center gap-2">
                            <span className="text-slate-600 select-none">{diag.line + 1} |</span>
                            <span className="truncate text-slate-300">{lineText}</span>
                          </div>
                        </div>
                      </div>

                      {/* Jump & QuickFix Buttons */}
                      <div className="flex items-center gap-2 shrink-0 self-end md:self-center">
                        {diag.quickFix && onApplyQuickFix && (
                          <button
                            onClick={() => {
                              onApplyQuickFix(
                                {
                                  label: diag.quickFix!.label,
                                  action: diag.quickFix!.action,
                                  replacement: diag.quickFix!.replacement,
                                },
                                diag.line,
                                diag.match
                              );
                            }}
                            className="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-300 border border-emerald-500/40 text-xs font-semibold transition-colors cursor-pointer"
                            title={diag.quickFix.label}
                          >
                            <Lightbulb className="w-3.5 h-3.5 text-emerald-400" />
                            <span>Quick Fix</span>
                          </button>
                        )}

                        {onJumpToLine && (
                          <button
                            onClick={() => onJumpToLine(diag.line)}
                            className="px-2.5 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 text-xs font-semibold transition-colors cursor-pointer"
                          >
                            Zu Zeile springen
                          </button>
                        )}
                      </div>
                    </div>
                  );
                })
              )}
            </div>
          </div>
        )}

        {/* TAB 3: SANDBOX & LIVE RULE TESTER */}
        {activeTab === "sandbox" && (
          <div className="flex-1 flex flex-col md:flex-row overflow-hidden p-6 gap-6">
            {/* Code Input */}
            <div className="flex-1 flex flex-col bg-black/40 rounded-xl border border-white/10 overflow-hidden">
              <div className="px-4 py-2.5 bg-black/60 border-b border-white/10 flex items-center justify-between">
                <span className="text-xs font-bold text-slate-300 flex items-center gap-2">
                  <Code2 className="w-4 h-4 text-cyan-400" />
                  Lumino Test-Code Playground
                </span>
                <button
                  onClick={() => setSandboxCode(currentFile?.content || "")}
                  className="text-[11px] text-slate-400 hover:text-white px-2 py-0.5 rounded bg-white/5 hover:bg-white/10 transition-colors"
                >
                  Aus Editor kopieren
                </button>
              </div>
              <textarea
                value={sandboxCode}
                onChange={(e) => setSandboxCode(e.target.value)}
                className="flex-1 p-4 bg-transparent text-slate-200 font-mono text-xs leading-relaxed resize-none outline-none custom-scrollbar"
                placeholder="Gib hier Lumino Code zum Testen der Regeln ein..."
                spellCheck={false}
              />
            </div>

            {/* Sandbox Results */}
            <div className="w-full md:w-96 flex flex-col bg-black/40 rounded-xl border border-white/10 overflow-hidden">
              <div className="px-4 py-2.5 bg-black/60 border-b border-white/10 flex items-center justify-between">
                <span className="text-xs font-bold text-slate-300 flex items-center gap-2">
                  <Sparkles className="w-4 h-4 text-amber-400" />
                  Erkannte Regelverletzungen ({sandboxErrors.length})
                </span>
              </div>

              <div className="flex-1 overflow-y-auto p-4 space-y-2.5 custom-scrollbar">
                {sandboxErrors.length === 0 ? (
                  <div className="text-center py-16 text-slate-500">
                    <CheckCircle2 className="w-8 h-8 mx-auto mb-2 text-emerald-400 opacity-60" />
                    <p className="text-xs font-semibold text-slate-400">Keine Fehler im Sandbox-Code</p>
                  </div>
                ) : (
                  sandboxErrors.map((err, idx) => (
                    <div
                      key={idx}
                      className={`p-3 rounded-lg border text-xs ${
                        err.type === "error"
                          ? "bg-red-500/10 border-red-500/30 text-red-300"
                          : "bg-amber-500/10 border-amber-500/30 text-amber-300"
                      }`}
                    >
                      <div className="flex items-center justify-between font-mono text-[11px] mb-1">
                        <span className="font-bold">Zeile {err.line + 1}</span>
                        <span className="uppercase text-[10px] font-bold">{err.type}</span>
                      </div>
                      <p className="font-sans text-slate-200 mb-1">{err.message}</p>
                      <div className="bg-black/50 px-2 py-0.5 rounded font-mono text-[10px] text-slate-400">
                        Match: <code className="text-amber-300">"{err.match}"</code>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          </div>
        )}
      </div>

      {/* RULE EDITOR MODAL */}
      {editingRule && (
        <RuleEditModal
          rule={editingRule}
          isCreating={isCreating}
          onSave={handleSaveRuleEdit}
          onClose={() => {
            setEditingRule(null);
            setIsCreating(false);
          }}
        />
      )}
    </div>
  );
}

// ----------------------------------------------------------------------
// MODAL: RULE CREATION / EDITING
// ----------------------------------------------------------------------
interface RuleEditModalProps {
  rule: CustomLintRule;
  isCreating: boolean;
  onSave: (rule: CustomLintRule) => void;
  onClose: () => void;
}

function RuleEditModal({ rule, isCreating, onSave, onClose }: RuleEditModalProps) {
  const [formData, setFormData] = useState<CustomLintRule>({ ...rule });
  const [testString, setTestString] = useState("var testVar = 100;");
  const [regexError, setRegexError] = useState<string | null>(null);

  // Validate regex on input
  const validateRegex = (pattern: string, flags: string = "g") => {
    try {
      new RegExp(pattern, flags);
      setRegexError(null);
      return true;
    } catch (e: any) {
      setRegexError(e.message);
      return false;
    }
  };

  const testMatches = useMemo(() => {
    if (formData.patternType === "regex" && formData.pattern) {
      try {
        const rx = new RegExp(formData.pattern, formData.flags || "g");
        return (testString.match(rx) || []).length;
      } catch {
        return 0;
      }
    }
    return 0;
  }, [formData, testString]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.name.trim()) {
      alert("Bitte einen Regelnamen eingeben.");
      return;
    }
    if (formData.patternType === "regex") {
      if (!validateRegex(formData.pattern || "", formData.flags)) {
        alert("Ungültiger regulärer Ausdruck!");
        return;
      }
    }
    onSave(formData);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
      <div className="bg-[#121217] border border-white/10 rounded-2xl w-full max-w-2xl max-h-[90vh] flex flex-col shadow-2xl overflow-hidden">
        {/* Modal Header */}
        <div className="px-6 py-4 border-b border-white/10 flex items-center justify-between bg-black/40">
          <div className="flex items-center gap-2.5">
            <div className="p-2 bg-amber-500/20 border border-amber-500/30 rounded-lg text-amber-400">
              <Sliders className="w-5 h-5" />
            </div>
            <div>
              <h3 className="text-base font-bold text-slate-100">
                {isCreating ? "Neue Linting-Regel definieren" : `Regel bearbeiten: ${formData.name}`}
              </h3>
              <p className="text-xs text-slate-400">
                Konfiguriere Muster, Schweregrad, Fehlermeldung und Quick-Fixes.
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-1.5 rounded-lg text-slate-400 hover:text-white hover:bg-white/10 transition-colors cursor-pointer"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Modal Form Content */}
        <form onSubmit={handleSubmit} className="flex-1 overflow-y-auto p-6 space-y-4 custom-scrollbar">
          {/* Name & Category */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-xs font-bold text-slate-300 mb-1.5">
                Regelname <span className="text-amber-400">*</span>
              </label>
              <input
                type="text"
                required
                value={formData.name}
                onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                placeholder="z. B. Keine 'var'-Deklaration"
                className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-xs text-slate-200 outline-none focus:border-amber-500/50"
              />
            </div>

            <div>
              <label className="block text-xs font-bold text-slate-300 mb-1.5">
                Kategorie
              </label>
              <select
                value={formData.category}
                onChange={(e) => setFormData({ ...formData, category: e.target.value as any })}
                className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-xs text-slate-200 outline-none focus:border-amber-500/50"
              >
                <option value="Style">Style</option>
                <option value="Best Practices">Best Practices</option>
                <option value="Security">Security</option>
                <option value="Formatting">Formatting</option>
                <option value="Custom">Custom</option>
              </select>
            </div>
          </div>

          {/* Description */}
          <div>
            <label className="block text-xs font-bold text-slate-300 mb-1.5">
              Beschreibung & Begründung
            </label>
            <input
              type="text"
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              placeholder="Erklärung, warum diese Regel für den Quellcode wichtig ist..."
              className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-xs text-slate-200 outline-none focus:border-amber-500/50"
            />
          </div>

          {/* Severity & Pattern Type */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-xs font-bold text-slate-300 mb-1.5">
                Schweregrad (Severity)
              </label>
              <div className="grid grid-cols-2 gap-2">
                <button
                  type="button"
                  onClick={() => setFormData({ ...formData, severity: "warning" })}
                  className={`px-3 py-2 rounded-lg text-xs font-bold border transition-colors cursor-pointer flex items-center justify-center gap-1.5 ${
                    formData.severity === "warning"
                      ? "bg-amber-500/20 border-amber-500/40 text-amber-300"
                      : "bg-black/30 border-white/10 text-slate-400 hover:text-slate-200"
                  }`}
                >
                  <AlertTriangle className="w-3.5 h-3.5" />
                  Warning
                </button>
                <button
                  type="button"
                  onClick={() => setFormData({ ...formData, severity: "error" })}
                  className={`px-3 py-2 rounded-lg text-xs font-bold border transition-colors cursor-pointer flex items-center justify-center gap-1.5 ${
                    formData.severity === "error"
                      ? "bg-red-500/20 border-red-500/40 text-red-300"
                      : "bg-black/30 border-white/10 text-slate-400 hover:text-slate-200"
                  }`}
                >
                  <XCircle className="w-3.5 h-3.5" />
                  Error
                </button>
              </div>
            </div>

            <div>
              <label className="block text-xs font-bold text-slate-300 mb-1.5">
                Muster-Typ (Pattern Type)
              </label>
              <select
                value={formData.patternType}
                onChange={(e) => setFormData({ ...formData, patternType: e.target.value as LintPatternType })}
                className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-xs text-slate-200 outline-none focus:border-amber-500/50"
              >
                <option value="regex">Regulärer Ausdruck (Regex)</option>
                <option value="keyword">Verbotenes Schlüsselwort (Keyword)</option>
                <option value="max_line_length">Maximale Zeilenlänge</option>
                <option value="naming_convention">Namenskonvention (camelCase, snake_case, etc.)</option>
                <option value="missing_pattern">Fehlendes Begleitmuster (z.B. Doc-Kommentar)</option>
              </select>
            </div>
          </div>

          {/* Pattern Type Specific Fields */}
          {formData.patternType === "regex" && (
            <div className="p-4 bg-black/30 rounded-xl border border-white/10 space-y-3">
              <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                <div className="md:col-span-2">
                  <label className="block text-xs font-bold text-slate-300 mb-1">
                    Regex Pattern
                  </label>
                  <input
                    type="text"
                    required
                    value={formData.pattern || ""}
                    onChange={(e) => {
                      const pat = e.target.value;
                      setFormData({ ...formData, pattern: pat });
                      validateRegex(pat, formData.flags);
                    }}
                    placeholder="z. B. \bvar\s+"
                    className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 font-mono text-xs text-cyan-300 outline-none focus:border-cyan-500/50"
                  />
                  {regexError && (
                    <span className="text-[11px] text-red-400 mt-1 block">
                      Regex-Fehler: {regexError}
                    </span>
                  )}
                </div>

                <div>
                  <label className="block text-xs font-bold text-slate-300 mb-1">
                    Flags
                  </label>
                  <input
                    type="text"
                    value={formData.flags || "g"}
                    onChange={(e) => {
                      const fl = e.target.value;
                      setFormData({ ...formData, flags: fl });
                      validateRegex(formData.pattern || "", fl);
                    }}
                    placeholder="g, i, m"
                    className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 font-mono text-xs text-slate-200 outline-none focus:border-cyan-500/50"
                  />
                </div>
              </div>

              {/* Live Regex Tester */}
              <div className="pt-2 border-t border-white/5">
                <div className="flex items-center justify-between text-[11px] text-slate-400 mb-1">
                  <span>Muster-Tester:</span>
                  <span className={testMatches > 0 ? "text-emerald-400 font-bold" : "text-slate-500"}>
                    {testMatches} Treffer im Teststring
                  </span>
                </div>
                <input
                  type="text"
                  value={testString}
                  onChange={(e) => setTestString(e.target.value)}
                  placeholder="Teststring hier eingeben..."
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 font-mono text-xs text-slate-300 outline-none"
                />
              </div>
            </div>
          )}

          {formData.patternType === "keyword" && (
            <div className="p-4 bg-black/30 rounded-xl border border-white/10">
              <label className="block text-xs font-bold text-slate-300 mb-1">
                Verbotenes Keyword / Wort
              </label>
              <input
                type="text"
                required
                value={formData.keyword || ""}
                onChange={(e) => setFormData({ ...formData, keyword: e.target.value })}
                placeholder="z. B. unsafe, goto, eval"
                className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 font-mono text-xs text-fuchsia-300 outline-none focus:border-fuchsia-500/50"
              />
            </div>
          )}

          {formData.patternType === "max_line_length" && (
            <div className="p-4 bg-black/30 rounded-xl border border-white/10">
              <label className="block text-xs font-bold text-slate-300 mb-1">
                Maximale Zeichen pro Zeile
              </label>
              <input
                type="number"
                min={20}
                max={500}
                value={formData.maxLength || 120}
                onChange={(e) => setFormData({ ...formData, maxLength: parseInt(e.target.value, 10) || 120 })}
                className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 font-mono text-xs text-purple-300 outline-none focus:border-purple-500/50"
              />
            </div>
          )}

          {formData.patternType === "naming_convention" && (
            <div className="p-4 bg-black/30 rounded-xl border border-white/10 grid grid-cols-2 gap-3">
              <div>
                <label className="block text-xs font-bold text-slate-300 mb-1">
                  Ziel-Element
                </label>
                <select
                  value={formData.targetEntity || "variable"}
                  onChange={(e) => setFormData({ ...formData, targetEntity: e.target.value as any })}
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 text-xs text-slate-200 outline-none"
                >
                  <option value="variable">Variablen (let / const)</option>
                  <option value="constant">Konstanten (const)</option>
                  <option value="function">Funktionen (fn)</option>
                </select>
              </div>

              <div>
                <label className="block text-xs font-bold text-slate-300 mb-1">
                  Erforderliche Konvention
                </label>
                <select
                  value={formData.convention || "camelCase"}
                  onChange={(e) => setFormData({ ...formData, convention: e.target.value as any })}
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 text-xs text-slate-200 outline-none"
                >
                  <option value="camelCase">camelCase (z. B. myVariable)</option>
                  <option value="snake_case">snake_case (z. B. my_variable)</option>
                  <option value="UPPER_CASE">UPPER_CASE (z. B. MAX_VALUE)</option>
                </select>
              </div>
            </div>
          )}

          {formData.patternType === "missing_pattern" && (
            <div className="p-4 bg-black/30 rounded-xl border border-white/10 space-y-3">
              <div>
                <label className="block text-xs font-bold text-slate-300 mb-1">
                  Auslöse-Muster (z.B. Funktionsbeginn)
                </label>
                <input
                  type="text"
                  value={formData.matchPattern || ""}
                  onChange={(e) => setFormData({ ...formData, matchPattern: e.target.value })}
                  placeholder="^\s*fn\s+[a-zA-Z_]\w*"
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 font-mono text-xs text-slate-200 outline-none"
                />
              </div>

              <div>
                <label className="block text-xs font-bold text-slate-300 mb-1">
                  Erforderliches Begleitmuster (vorangestellte Zeile)
                </label>
                <input
                  type="text"
                  value={formData.requirePattern || ""}
                  onChange={(e) => setFormData({ ...formData, requirePattern: e.target.value })}
                  placeholder="^\s*\/\/"
                  className="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-1.5 font-mono text-xs text-slate-200 outline-none"
                />
              </div>
            </div>
          )}

          {/* Error Message */}
          <div>
            <label className="block text-xs font-bold text-slate-300 mb-1.5">
              Fehlermeldung (im Editor und Tooltip angezeigt)
            </label>
            <input
              type="text"
              required
              value={formData.errorMessage}
              onChange={(e) => setFormData({ ...formData, errorMessage: e.target.value })}
              placeholder="z. B. Veraltetes 'var'-Keyword gefunden. Nutze 'let'."
              className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-xs text-slate-200 outline-none focus:border-amber-500/50"
            />
          </div>

          {/* Quick Fix Configuration */}
          <div className="p-4 bg-black/20 rounded-xl border border-white/5 space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-xs font-bold text-slate-300 flex items-center gap-1.5">
                <Lightbulb className="w-3.5 h-3.5 text-amber-400" />
                Automatischer Quick Fix (1-Klick-Korrektur)
              </span>
              <input
                type="checkbox"
                checked={Boolean(formData.quickFix)}
                onChange={(e) => {
                  if (e.target.checked) {
                    setFormData({
                      ...formData,
                      quickFix: {
                        label: "Automatisch korrigieren",
                        actionType: "replace",
                        replacement: "",
                      },
                    });
                  } else {
                    const { quickFix, ...rest } = formData;
                    setFormData(rest as any);
                  }
                }}
                className="rounded accent-amber-500 cursor-pointer"
              />
            </div>

            {formData.quickFix && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-3 pt-2">
                <div>
                  <label className="block text-xs text-slate-400 mb-1">
                    Button Label
                  </label>
                  <input
                    type="text"
                    value={formData.quickFix.label}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        quickFix: { ...formData.quickFix!, label: e.target.value },
                      })
                    }
                    placeholder="z. B. 'var' durch 'let' ersetzen"
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-1.5 text-xs text-slate-200 outline-none"
                  />
                </div>

                <div>
                  <label className="block text-xs text-slate-400 mb-1">
                    Ersetzungstext
                  </label>
                  <input
                    type="text"
                    value={formData.quickFix.replacement ?? ""}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        quickFix: { ...formData.quickFix!, replacement: e.target.value },
                      })
                    }
                    placeholder="z. B. let "
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-1.5 font-mono text-xs text-slate-200 outline-none"
                  />
                </div>
              </div>
            )}
          </div>

          {/* Modal Footer Buttons */}
          <div className="pt-4 border-t border-white/10 flex items-center justify-end gap-2.5">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 rounded-lg text-xs font-semibold text-slate-400 hover:text-white hover:bg-white/5 transition-colors cursor-pointer"
            >
              Abbrechen
            </button>
            <button
              type="submit"
              className="px-5 py-2 rounded-lg text-xs font-bold bg-amber-500 hover:bg-amber-400 text-black shadow-lg shadow-amber-500/20 transition-colors cursor-pointer flex items-center gap-1.5"
            >
              <Check className="w-4 h-4" />
              <span>Regel speichern</span>
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
