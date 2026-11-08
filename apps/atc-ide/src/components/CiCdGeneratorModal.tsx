import React, { useState, useMemo } from "react";
import {
  X,
  Copy,
  Check,
  Download,
  FileCode,
  FolderGit2,
  Play,
  Settings2,
  Sparkles,
  GitBranch,
  ShieldCheck,
  Cpu,
  Layers,
  CheckCircle2,
  AlertCircle,
  FileCheck,
  Terminal,
  Zap
} from "lucide-react";
import { FileState } from "../App";
import {
  LuminoCiCdOptions,
  DEFAULT_CICD_OPTIONS,
  generateLuminoWorkflowYaml,
  generateLuminoSampleTestFile
} from "../utils/cicdGenerator";

interface CiCdGeneratorModalProps {
  isOpen: boolean;
  onClose: () => void;
  files: FileState[];
  onSaveWorkflowFile: (filePath: string, content: string, companionFile?: { name: string; content: string }) => void;
}

type PresetType = "standard" | "kernel_matrix" | "strict_pr" | "custom";

export function CiCdGeneratorModal({
  isOpen,
  onClose,
  files,
  onSaveWorkflowFile
}: CiCdGeneratorModalProps) {
  const [selectedPreset, setSelectedPreset] = useState<PresetType>("standard");
  const [copied, setCopied] = useState(false);
  const [createSampleTest, setCreateSampleTest] = useState(() => {
    // Default to true if no test file exists yet
    return !files.some((f) => f.name.includes("test"));
  });

  const [options, setOptions] = useState<LuminoCiCdOptions>({ ...DEFAULT_CICD_OPTIONS });
  const [newBranchInput, setNewBranchInput] = useState("");

  // Check if .github/workflows/lumino-build.yml already exists in project
  const existingWorkflow = useMemo(() => {
    return files.find((f) => f.name === ".github/workflows/lumino-build.yml");
  }, [files]);

  // Check if any test file exists
  const existingTestFile = useMemo(() => {
    return files.find((f) => f.name.toLowerCase().includes("test"));
  }, [files]);

  // Handle Preset Change
  const handlePresetSelect = (preset: PresetType) => {
    setSelectedPreset(preset);
    if (preset === "standard") {
      setOptions({
        ...DEFAULT_CICD_OPTIONS,
        enableArchMatrix: false,
        enablePullRequest: true,
        enableSyntaxCheck: true,
      });
    } else if (preset === "kernel_matrix") {
      setOptions({
        ...DEFAULT_CICD_OPTIONS,
        enableArchMatrix: true,
        archTargets: ["x86_64", "arm64", "riscv"],
        enablePullRequest: true,
        enableSyntaxCheck: true,
      });
    } else if (preset === "strict_pr") {
      setOptions({
        ...DEFAULT_CICD_OPTIONS,
        failFast: true,
        enablePullRequest: true,
        enableSyntaxCheck: true,
        enableArchMatrix: false,
      });
    }
  };

  const yamlContent = useMemo(() => {
    return generateLuminoWorkflowYaml(options);
  }, [options]);

  if (!isOpen) return null;

  const handleCopyYaml = () => {
    navigator.clipboard.writeText(yamlContent);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownloadYaml = () => {
    const blob = new Blob([yamlContent], { type: "text/yaml;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "lumino-build.yml";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  const handleAddToWorkspace = () => {
    const companion = createSampleTest
      ? {
          name: "tests/kernel_tests.lumino",
          content: generateLuminoSampleTestFile(),
        }
      : undefined;

    onSaveWorkflowFile(".github/workflows/lumino-build.yml", yamlContent, companion);
    onClose();
  };

  const addBranch = () => {
    const trimmed = newBranchInput.trim();
    if (trimmed && !options.triggerPushBranches.includes(trimmed)) {
      setOptions({
        ...options,
        triggerPushBranches: [...options.triggerPushBranches, trimmed],
      });
      setNewBranchInput("");
    }
  };

  const removeBranch = (branchToRemove: string) => {
    if (options.triggerPushBranches.length <= 1) return; // Keep at least one
    setOptions({
      ...options,
      triggerPushBranches: options.triggerPushBranches.filter((b) => b !== branchToRemove),
    });
  };

  const toggleArchTarget = (arch: "x86_64" | "arm64" | "riscv" | "cortex_m") => {
    if (options.archTargets.includes(arch)) {
      if (options.archTargets.length > 1) {
        setOptions({
          ...options,
          archTargets: options.archTargets.filter((a) => a !== arch),
        });
      }
    } else {
      setOptions({
        ...options,
        archTargets: [...options.archTargets, arch],
      });
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-md">
      <div className="bg-[#0e0e13] border border-white/10 rounded-2xl w-full max-w-5xl h-[88vh] flex flex-col shadow-2xl overflow-hidden animate-in fade-in zoom-in-95 duration-150">
        
        {/* Modal Header */}
        <div className="px-6 py-4 border-b border-white/10 bg-[#14141c] flex items-center justify-between shrink-0">
          <div className="flex items-center gap-3">
            <div className="p-2.5 bg-gradient-to-br from-indigo-500/20 to-purple-500/20 border border-indigo-500/30 rounded-xl text-indigo-400">
              <Zap className="w-5 h-5" />
            </div>
            <div>
              <div className="flex items-center gap-2">
                <h2 className="text-base font-bold text-slate-100">
                  Lumino CI/CD Workflow Generator
                </h2>
                <span className="px-2 py-0.5 text-[11px] font-mono bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 rounded-full font-semibold">
                  .github/workflows/lumino-build.yml
                </span>
              </div>
              <p className="text-xs text-slate-400 mt-0.5">
                Automatische Ausführung aller Lumino Unit-Tests direkt bei jedem Git Push.
              </p>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={onClose}
              className="p-1.5 text-slate-400 hover:text-slate-200 hover:bg-white/5 rounded-lg transition-colors cursor-pointer"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </div>

        {/* Modal Body */}
        <div className="flex-1 flex overflow-hidden">
          
          {/* Left Column: Configuration Controls */}
          <div className="w-[48%] border-r border-white/10 flex flex-col bg-[#101017] overflow-y-auto custom-scrollbar p-5 space-y-5">
            
            {/* Presets */}
            <div>
              <label className="text-xs font-bold text-slate-400 uppercase tracking-wider block mb-2">
                CI/CD Profil-Vorlagen
              </label>
              <div className="grid grid-cols-2 gap-2">
                <button
                  type="button"
                  onClick={() => handlePresetSelect("standard")}
                  className={`p-3 rounded-xl border text-left transition-all cursor-pointer ${
                    selectedPreset === "standard"
                      ? "bg-indigo-600/20 border-indigo-500/60 text-white shadow-sm"
                      : "bg-white/[0.02] border-white/5 text-slate-400 hover:bg-white/[0.05] hover:text-slate-200"
                  }`}
                >
                  <div className="flex items-center gap-2 text-xs font-bold text-slate-200 mb-1">
                    <ShieldCheck className="w-4 h-4 text-emerald-400" />
                    Standard Test CI
                  </div>
                  <div className="text-[11px] text-slate-400 leading-tight">
                    Push auf main/master, Syntax-Prüfung & automatische Testläufe
                  </div>
                </button>

                <button
                  type="button"
                  onClick={() => handlePresetSelect("kernel_matrix")}
                  className={`p-3 rounded-xl border text-left transition-all cursor-pointer ${
                    selectedPreset === "kernel_matrix"
                      ? "bg-indigo-600/20 border-indigo-500/60 text-white shadow-sm"
                      : "bg-white/[0.02] border-white/5 text-slate-400 hover:bg-white/[0.05] hover:text-slate-200"
                  }`}
                >
                  <div className="flex items-center gap-2 text-xs font-bold text-slate-200 mb-1">
                    <Cpu className="w-4 h-4 text-cyan-400" />
                    Multi-Arch Matrix
                  </div>
                  <div className="text-[11px] text-slate-400 leading-tight">
                    x86_64, ARM64 & RISC-V Architekturen parallel testen
                  </div>
                </button>
              </div>
            </div>

            {/* Workflow Trigger Configuration */}
            <div className="space-y-3 bg-white/[0.02] p-4 rounded-xl border border-white/5">
              <div className="flex items-center gap-2 text-xs font-bold text-slate-300">
                <GitBranch className="w-4 h-4 text-indigo-400" />
                <span>Git Push Auslöser (Triggers)</span>
              </div>

              <div>
                <div className="text-[11px] text-slate-400 mb-1.5">
                  Bei Push auf folgende Branches ausführen:
                </div>
                <div className="flex flex-wrap gap-1.5 mb-2">
                  {options.triggerPushBranches.map((b) => (
                    <span
                      key={b}
                      className="inline-flex items-center gap-1.5 px-2.5 py-1 bg-indigo-500/20 border border-indigo-500/40 text-indigo-200 rounded-md text-xs font-mono"
                    >
                      {b}
                      {options.triggerPushBranches.length > 1 && (
                        <button
                          type="button"
                          onClick={() => removeBranch(b)}
                          className="hover:text-red-400 text-slate-400 cursor-pointer"
                        >
                          ×
                        </button>
                      )}
                    </span>
                  ))}
                </div>

                <div className="flex gap-2">
                  <input
                    type="text"
                    value={newBranchInput}
                    onChange={(e) => setNewBranchInput(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") {
                        e.preventDefault();
                        addBranch();
                      }
                    }}
                    placeholder="Weiteren Branch hinzufügen (z.B. dev)"
                    className="flex-1 bg-black/40 border border-white/10 rounded-lg px-3 py-1.5 text-xs text-slate-200 font-mono outline-none focus:border-indigo-500/50"
                  />
                  <button
                    type="button"
                    onClick={addBranch}
                    className="px-3 py-1.5 bg-white/10 hover:bg-white/15 text-slate-200 rounded-lg text-xs font-semibold cursor-pointer"
                  >
                    + Add
                  </button>
                </div>
              </div>

              <div className="pt-2 border-t border-white/5 space-y-2">
                <label className="flex items-center gap-2 text-xs text-slate-300 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={options.enablePullRequest}
                    onChange={(e) =>
                      setOptions({ ...options, enablePullRequest: e.target.checked })
                    }
                    className="rounded border-white/20 bg-black/40 text-indigo-600 focus:ring-0 cursor-pointer"
                  />
                  <span>Auch bei Pull Requests (PRs) ausführen</span>
                </label>

                <label className="flex items-center gap-2 text-xs text-slate-300 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={options.enableWorkflowDispatch}
                    onChange={(e) =>
                      setOptions({ ...options, enableWorkflowDispatch: e.target.checked })
                    }
                    className="rounded border-white/20 bg-black/40 text-indigo-600 focus:ring-0 cursor-pointer"
                  />
                  <span>Manuelles Auslösen (workflow_dispatch) erlauben</span>
                </label>
              </div>
            </div>

            {/* Test & Quality Settings */}
            <div className="space-y-3 bg-white/[0.02] p-4 rounded-xl border border-white/5">
              <div className="flex items-center gap-2 text-xs font-bold text-slate-300">
                <Terminal className="w-4 h-4 text-emerald-400" />
                <span>Test-Schritte & Validierung</span>
              </div>

              <div className="space-y-2">
                <label className="flex items-start gap-2 text-xs text-slate-300 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={options.enableSyntaxCheck}
                    onChange={(e) =>
                      setOptions({ ...options, enableSyntaxCheck: e.target.checked })
                    }
                    className="mt-0.5 rounded border-white/20 bg-black/40 text-indigo-600 focus:ring-0 cursor-pointer"
                  />
                  <div>
                    <span className="font-semibold text-slate-200">
                      Lumino Lexer & Parser Syntaxprüfung
                    </span>
                    <p className="text-[11px] text-slate-400">
                      Validiert Klammern und Syntax aller .lumino Dateien vor dem Testlauf (Fail Fast).
                    </p>
                  </div>
                </label>

                <label className="flex items-start gap-2 text-xs text-slate-300 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={options.enableStepSummary}
                    onChange={(e) =>
                      setOptions({ ...options, enableStepSummary: e.target.checked })
                    }
                    className="mt-0.5 rounded border-white/20 bg-black/40 text-indigo-600 focus:ring-0 cursor-pointer"
                  />
                  <div>
                    <span className="font-semibold text-slate-200">
                      GitHub Step Summary Report
                    </span>
                    <p className="text-[11px] text-slate-400">
                      Generiert schöne Markdown-Tabellen direkt im GitHub Actions Ausführungs-Dashboard.
                    </p>
                  </div>
                </label>

                <label className="flex items-start gap-2 text-xs text-slate-300 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={options.enableArtifacts}
                    onChange={(e) =>
                      setOptions({ ...options, enableArtifacts: e.target.checked })
                    }
                    className="mt-0.5 rounded border-white/20 bg-black/40 text-indigo-600 focus:ring-0 cursor-pointer"
                  />
                  <div>
                    <span className="font-semibold text-slate-200">
                      Test-Ergebnis als Build-Artefakt speichern
                    </span>
                    <p className="text-[11px] text-slate-400">
                      Speichert lumino-test-results.json als downloadable GitHub Actions Artefakt.
                    </p>
                  </div>
                </label>
              </div>
            </div>

            {/* Architecture Matrix */}
            <div className="space-y-3 bg-white/[0.02] p-4 rounded-xl border border-white/5">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2 text-xs font-bold text-slate-300">
                  <Layers className="w-4 h-4 text-cyan-400" />
                  <span>Hardware-Architektur Matrix</span>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={options.enableArchMatrix}
                    onChange={(e) =>
                      setOptions({ ...options, enableArchMatrix: e.target.checked })
                    }
                    className="sr-only peer"
                  />
                  <div className="w-7 h-4 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-300 after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-indigo-600"></div>
                </label>
              </div>

              {options.enableArchMatrix && (
                <div className="pt-2 border-t border-white/5 space-y-2">
                  <div className="text-[11px] text-slate-400">
                    Aktive Ziel-Architekturen in der Matrix:
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {(["x86_64", "arm64", "riscv", "cortex_m"] as const).map((arch) => {
                      const isActive = options.archTargets.includes(arch);
                      return (
                        <button
                          key={arch}
                          type="button"
                          onClick={() => toggleArchTarget(arch)}
                          className={`px-3 py-1 rounded-md text-xs font-mono font-bold transition-all cursor-pointer ${
                            isActive
                              ? "bg-cyan-500/20 border border-cyan-500/40 text-cyan-300"
                              : "bg-white/5 border border-white/5 text-slate-500 hover:text-slate-300"
                          }`}
                        >
                          {arch}
                        </button>
                      );
                    })}
                  </div>
                </div>
              )}
            </div>

            {/* Companion Sample Test File */}
            <div className="p-3.5 rounded-xl border border-indigo-500/20 bg-indigo-500/5">
              <label className="flex items-start gap-2 text-xs text-slate-200 cursor-pointer">
                <input
                  type="checkbox"
                  checked={createSampleTest}
                  onChange={(e) => setCreateSampleTest(e.target.checked)}
                  className="mt-0.5 rounded border-white/20 bg-black/40 text-indigo-600 focus:ring-0 cursor-pointer"
                />
                <div>
                  <div className="font-semibold text-indigo-300 flex items-center gap-1.5">
                    <FileCheck className="w-3.5 h-3.5" />
                    <span>Zusätzlich Unit-Test Suite generieren</span>
                  </div>
                  <p className="text-[11px] text-slate-400 mt-0.5">
                    Erstellt die Datei <code className="text-amber-300 font-mono">tests/kernel_tests.lumino</code> mit Tests für Arithmetik, Rekursion, Speicherausrichtung & Syscalls, damit der CI-Workflow sofort grün durchläuft!
                  </p>
                </div>
              </label>
            </div>

          </div>

          {/* Right Column: Live YAML Code Preview */}
          <div className="w-[52%] flex flex-col bg-[#0b0b10]">
            
            {/* Preview Toolbar */}
            <div className="px-4 py-2.5 border-b border-white/10 bg-[#12121a] flex items-center justify-between shrink-0">
              <div className="flex items-center gap-2 text-xs font-mono text-slate-300">
                <FileCode className="w-4 h-4 text-amber-400" />
                <span>.github/workflows/lumino-build.yml</span>
                {existingWorkflow && (
                  <span className="px-1.5 py-0.5 text-[10px] bg-amber-500/20 text-amber-300 rounded border border-amber-500/30">
                    Aktualisiert existierende Datei
                  </span>
                )}
              </div>

              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={handleCopyYaml}
                  className="flex items-center gap-1 px-2.5 py-1 rounded bg-white/5 hover:bg-white/10 text-slate-300 hover:text-white text-xs font-medium transition-colors cursor-pointer"
                  title="YAML kopieren"
                >
                  {copied ? (
                    <>
                      <Check className="w-3.5 h-3.5 text-emerald-400" />
                      <span className="text-emerald-400">Kopiert!</span>
                    </>
                  ) : (
                    <>
                      <Copy className="w-3.5 h-3.5" />
                      <span>Kopieren</span>
                    </>
                  )}
                </button>

                <button
                  type="button"
                  onClick={handleDownloadYaml}
                  className="flex items-center gap-1 px-2.5 py-1 rounded bg-white/5 hover:bg-white/10 text-slate-300 hover:text-white text-xs font-medium transition-colors cursor-pointer"
                  title="YAML-Datei herunterladen"
                >
                  <Download className="w-3.5 h-3.5" />
                  <span>Download</span>
                </button>
              </div>
            </div>

            {/* YAML Code Display */}
            <div className="flex-1 overflow-auto p-4 font-mono text-xs text-slate-300 bg-[#09090d] select-text custom-scrollbar">
              <pre className="whitespace-pre leading-relaxed">
                {yamlContent.split("\n").map((line, i) => {
                  let colorClass = "text-slate-300";
                  if (line.trim().startsWith("#")) {
                    colorClass = "text-slate-500 italic";
                  } else if (line.includes(":") && !line.trim().startsWith("-")) {
                    colorClass = "text-indigo-300 font-semibold";
                  } else if (line.trim().startsWith("-")) {
                    colorClass = "text-cyan-300";
                  } else if (line.includes('"') || line.includes("'")) {
                    colorClass = "text-emerald-300";
                  }

                  return (
                    <div key={i} className="flex hover:bg-white/[0.02]">
                      <span className="w-9 text-slate-600 select-none text-right pr-3 shrink-0">
                        {i + 1}
                      </span>
                      <span className={colorClass}>{line}</span>
                    </div>
                  );
                })}
              </pre>
            </div>

            {/* Status Footer */}
            <div className="p-4 border-t border-white/10 bg-[#12121a] flex items-center justify-between shrink-0">
              <div className="flex items-center gap-2 text-xs text-slate-400">
                <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                <span>
                  GitHub Actions kompatibel (Push-Trigger & Runner integriert)
                </span>
              </div>

              <div className="flex items-center gap-3">
                <button
                  type="button"
                  onClick={onClose}
                  className="px-4 py-2 text-xs font-bold text-slate-400 hover:text-slate-200 transition-colors cursor-pointer"
                >
                  Abbrechen
                </button>

                <button
                  type="button"
                  onClick={handleAddToWorkspace}
                  className="px-5 py-2 bg-gradient-to-r from-indigo-600 to-indigo-500 hover:from-indigo-500 hover:to-indigo-400 text-white rounded-xl text-xs font-bold shadow-lg shadow-indigo-600/30 flex items-center gap-2 transition-all cursor-pointer"
                >
                  <FolderGit2 className="w-4 h-4" />
                  <span>In Workspace einbinden</span>
                </button>
              </div>
            </div>

          </div>

        </div>

      </div>
    </div>
  );
}
