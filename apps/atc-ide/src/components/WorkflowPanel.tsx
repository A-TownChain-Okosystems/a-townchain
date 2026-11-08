import React, { useState, useMemo } from "react";
import {
  Network,
  Plus,
  Play,
  CheckCircle2,
  Circle,
  ArrowRight,
  Save,
  Trash2,
  Clock,
  Activity,
  GripVertical,
  Settings2,
  ShieldCheck,
  Zap,
  FileCode,
  Sparkles,
  GitBranch,
  FileCheck,
  AlertTriangle,
  RotateCcw,
  ExternalLink
} from "lucide-react";
import { FileState } from "../App";
import { Lexer } from "../interpreter/lexer";
import { Parser } from "../interpreter/parser";
import { Evaluator, Environment } from "../interpreter/evaluator";

interface WorkflowNode {
  id: string;
  title: string;
  type: "trigger" | "action" | "test" | "deploy";
  status: "idle" | "running" | "success" | "error";
  icon: React.ReactNode;
  description: string;
}

interface WorkflowPanelProps {
  files: FileState[];
  onOpenCiCdGenerator: () => void;
  onOpenEditorFile?: (filename: string) => void;
}

export function WorkflowPanel({
  files = [],
  onOpenCiCdGenerator,
  onOpenEditorFile,
}: WorkflowPanelProps) {
  // Check if .github/workflows/lumino-build.yml exists
  const workflowFile = useMemo(() => {
    return files.find((f) => f.name === ".github/workflows/lumino-build.yml");
  }, [files]);

  // Check if any unit test file exists
  const testFiles = useMemo(() => {
    return files.filter(
      (f) =>
        f.name.toLowerCase().includes("test") ||
        f.name.startsWith("tests/")
    );
  }, [files]);

  const [nodes, setNodes] = useState<WorkflowNode[]>([
    {
      id: "1",
      title: "Git Push (main)",
      type: "trigger",
      status: "success",
      icon: <Network className="w-5 h-5 text-indigo-400" />,
      description: "Triggered on push to main / master with .lumino files",
    },
    {
      id: "2",
      title: "Lexer & Parser AST Check",
      type: "action",
      status: "idle",
      icon: <ShieldCheck className="w-5 h-5 text-emerald-400" />,
      description: "Validates syntax and delimiter balance for all sources",
    },
    {
      id: "3",
      title: "Lumino Unit Tests",
      type: "test",
      status: "idle",
      icon: <Settings2 className="w-5 h-5 text-amber-400" />,
      description: "Executes unit test suites and validates assertions",
    },
    {
      id: "4",
      title: "GitHub Step Summary",
      type: "deploy",
      status: "idle",
      icon: <Zap className="w-5 h-5 text-cyan-400" />,
      description: "Generates Markdown report table & test artifact logs",
    },
  ]);

  const [isRunning, setIsRunning] = useState(false);
  const [logs, setLogs] = useState<string[]>([
    "[CI/CD] Workflow-System initialisiert.",
    workflowFile
      ? "[CI/CD] .github/workflows/lumino-build.yml aktiv im Projekt erkannt."
      : "[CI/CD] Hinweis: .github/workflows/lumino-build.yml noch nicht im Workspace. Nutze den CI/CD Generator!",
  ]);

  const runWorkflowPipeline = () => {
    setIsRunning(true);
    const timestamp = () => new Date().toLocaleTimeString();

    setLogs([
      `[${timestamp()}] 🚀 GitHub Actions Runner gestartet (ubuntu-latest)...`,
      `[${timestamp()}] 📥 Schritt 1: actions/checkout@v4 - Workspace mit ${files.length} Dateien ausgecheckt.`,
      `[${timestamp()}] ⚙️ Schritt 2: actions/setup-node@v4 - Node.js 20.x initialisiert.`,
    ]);

    // Update node 1
    setNodes((prev) =>
      prev.map((n, i) => (i === 0 ? { ...n, status: "success" } : { ...n, status: "running" }))
    );

    // Step 2: Syntax Check
    setTimeout(() => {
      let syntaxErrors = 0;
      const luminoFiles = files.filter((f) => f.name.endsWith(".lumino"));
      
      luminoFiles.forEach((file) => {
        try {
          const lexer = new Lexer(file.content);
          const parser = new Parser(lexer);
          parser.parseProgram();
          if (parser.errors.length > 0) {
            syntaxErrors += parser.errors.length;
          }
        } catch {
          syntaxErrors++;
        }
      });

      if (syntaxErrors > 0) {
        setLogs((prev) => [
          ...prev,
          `[${timestamp()}] ❌ Syntax-Check: ${syntaxErrors} Parser-Fehler in .lumino Dateien gefunden!`,
        ]);
        setNodes((prev) =>
          prev.map((n, i) => (i === 1 ? { ...n, status: "error" } : n))
        );
        setIsRunning(false);
        return;
      }

      setLogs((prev) => [
        ...prev,
        `[${timestamp()}] ✅ Lexer & Parser Syntaxprüfung: ${luminoFiles.length} Dateien syntaktisch valide.`,
      ]);
      setNodes((prev) =>
        prev.map((n, i) => (i === 1 ? { ...n, status: "success" } : n))
      );

      // Step 3: Unit Tests
      setTimeout(() => {
        let passedCount = 0;
        let failedCount = 0;

        const targetFiles = testFiles.length > 0 ? testFiles : luminoFiles.slice(0, 2);

        targetFiles.forEach((testFile) => {
          try {
            const lexer = new Lexer(testFile.content);
            const parser = new Parser(lexer);
            const program = parser.parseProgram();
            const env = new Environment();
            const evaluator = new Evaluator(env);
            evaluator.eval(program);
            const output = evaluator.getOutput();

            const hasFailed = output.some((line) => line.includes("[FAIL]"));
            if (hasFailed) {
              failedCount++;
            } else {
              passedCount++;
            }
          } catch {
            failedCount++;
          }
        });

        if (failedCount > 0) {
          setLogs((prev) => [
            ...prev,
            `[${timestamp()}] ❌ Unit Tests: ${failedCount} Test-Suite(s) fehlgeschlagen!`,
          ]);
          setNodes((prev) =>
            prev.map((n, i) => (i === 2 ? { ...n, status: "error" } : n))
          );
          setIsRunning(false);
          return;
        }

        setLogs((prev) => [
          ...prev,
          `[${timestamp()}] 🧪 Unit Tests: ${passedCount} Suite(s) erfolgreich ausgeführt. Alle Assertions grün!`,
        ]);
        setNodes((prev) =>
          prev.map((n, i) => (i === 2 ? { ...n, status: "success" } : n))
        );

        // Step 4: Step Summary & Artifacts
        setTimeout(() => {
          setLogs((prev) => [
            ...prev,
            `[${timestamp()}] 📊 GitHub Step Summary: Markdown-Report in $GITHUB_STEP_SUMMARY geschrieben.`,
            `[${timestamp()}] 📦 Artefakte: lumino-test-results.json hochgeladen (actions/upload-artifact@v4).`,
            `[${timestamp()}] 🎉 CI/CD Pipeline erfolgreich abgeschlossen!`,
          ]);
          setNodes((prev) =>
            prev.map((n, i) => (i === 3 ? { ...n, status: "success" } : n))
          );
          setIsRunning(false);
        }, 600);
      }, 700);
    }, 600);
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case "success":
        return "border-emerald-500/50 bg-emerald-500/10 text-emerald-400";
      case "running":
        return "border-amber-500/50 bg-amber-500/10 text-amber-400 animate-pulse";
      case "error":
        return "border-red-500/50 bg-red-500/10 text-red-400";
      default:
        return "border-white/10 bg-white/5 text-slate-400";
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "success":
        return <CheckCircle2 className="w-5 h-5" />;
      case "running":
        return <Activity className="w-5 h-5" />;
      case "error":
        return <Circle className="w-5 h-5 text-red-400" />;
      default:
        return <Circle className="w-5 h-5 opacity-50" />;
    }
  };

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0c0c0e]">
      
      {/* Top Banner & Status Bar */}
      <div className="p-5 border-b border-white/10 flex items-center justify-between bg-black/30 shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2.5 bg-gradient-to-br from-indigo-500/20 to-purple-500/20 border border-indigo-500/30 rounded-xl text-indigo-400">
            <Network className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-lg font-bold text-slate-100">
                Lumino CI/CD & Test Pipelines
              </h2>
              {workflowFile ? (
                <span className="flex items-center gap-1 px-2 py-0.5 rounded-full text-[11px] font-mono bg-emerald-500/15 border border-emerald-500/30 text-emerald-300 font-semibold">
                  <CheckCircle2 className="w-3 h-3 text-emerald-400" />
                  .github/workflows/lumino-build.yml aktiv
                </span>
              ) : (
                <span className="flex items-center gap-1 px-2 py-0.5 rounded-full text-[11px] font-mono bg-amber-500/15 border border-amber-500/30 text-amber-300 font-semibold">
                  <AlertTriangle className="w-3 h-3 text-amber-400" />
                  Keine CI/CD Konfiguration
                </span>
              )}
            </div>
            <p className="text-xs text-slate-400 mt-0.5">
              Automatisiere Unit-Tests, Syntax-Checks und Artefakte bei jedem Git Push auf GitHub.
            </p>
          </div>
        </div>

        <div className="flex items-center gap-3">
          {/* CI/CD Generator Button */}
          <button
            onClick={onOpenCiCdGenerator}
            className="flex items-center gap-2 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white px-3.5 py-2 rounded-xl font-bold text-xs shadow-lg shadow-indigo-600/20 transition-all cursor-pointer"
          >
            <Zap className="w-4 h-4" />
            <span>CI/CD Generator</span>
          </button>

          {/* If file exists, open in editor */}
          {workflowFile && onOpenEditorFile && (
            <button
              onClick={() => onOpenEditorFile(".github/workflows/lumino-build.yml")}
              className="flex items-center gap-1.5 bg-white/5 hover:bg-white/10 text-slate-300 px-3 py-2 rounded-xl text-xs font-semibold transition-colors cursor-pointer border border-white/10"
              title="YAML im Editor öffnen"
            >
              <FileCode className="w-3.5 h-3.5 text-amber-400" />
              <span>YAML öffnen</span>
            </button>
          )}

          {/* Run Pipeline Execution */}
          <button
            disabled={isRunning}
            onClick={runWorkflowPipeline}
            className={`flex items-center gap-2 px-4 py-2 rounded-xl font-bold text-xs transition-all cursor-pointer ${
              isRunning
                ? "bg-amber-500/20 text-amber-300 border border-amber-500/40"
                : "bg-emerald-600 hover:bg-emerald-500 text-white shadow-lg shadow-emerald-700/20"
            }`}
          >
            {isRunning ? (
              <>
                <Activity className="w-4 h-4 animate-spin" />
                <span>Pipeline läuft...</span>
              </>
            ) : (
              <>
                <Play className="w-4 h-4" />
                <span>CI/CD Pipeline ausführen</span>
              </>
            )}
          </button>
        </div>
      </div>

      {/* Info notification if workflow missing */}
      {!workflowFile && (
        <div className="px-6 py-2.5 bg-amber-500/10 border-b border-amber-500/20 flex items-center justify-between text-xs text-amber-300">
          <div className="flex items-center gap-2">
            <AlertTriangle className="w-4 h-4 text-amber-400 shrink-0" />
            <span>
              Dieses Repository enthält noch keine GitHub Actions Workflow-Datei. Klicke auf <strong>CI/CD Generator</strong>, um <code className="bg-black/30 px-1 py-0.5 rounded font-mono">.github/workflows/lumino-build.yml</code> für automatische Unit-Tests bei jedem Git Push zu erstellen.
            </span>
          </div>
          <button
            onClick={onOpenCiCdGenerator}
            className="px-2.5 py-1 bg-amber-500/20 hover:bg-amber-500/30 text-amber-200 rounded font-semibold text-[11px] transition-colors shrink-0 ml-4 cursor-pointer"
          >
            Jetzt generieren
          </button>
        </div>
      )}

      {/* Pipeline Diagram */}
      <div className="flex-1 overflow-x-auto p-8 relative flex items-center justify-start min-h-[360px]">
        {/* Background Grid */}
        <div
          className="absolute inset-0"
          style={{
            backgroundImage:
              "radial-gradient(circle at 2px 2px, rgba(255,255,255,0.05) 1px, transparent 0)",
            backgroundSize: "24px 24px",
          }}
        ></div>

        <div className="relative z-10 flex items-center mx-auto">
          {nodes.map((node, index) => (
            <React.Fragment key={node.id}>
              {/* Node Card */}
              <div className="w-64 shrink-0 bg-black/60 backdrop-blur-xl border border-white/10 rounded-xl overflow-hidden shadow-2xl relative group hover:border-indigo-500/50 transition-colors">
                <div className="absolute left-0 top-0 bottom-0 w-1 bg-gradient-to-b from-indigo-500 to-transparent opacity-50"></div>

                <div className="px-4 py-2 border-b border-white/10 flex justify-between items-center bg-white/[0.02]">
                  <div className="flex items-center gap-2">
                    {node.icon}
                    <span className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                      {node.type}
                    </span>
                  </div>
                  <span className="text-[10px] font-mono text-slate-500">
                    Step {index + 1}
                  </span>
                </div>

                <div className="p-4">
                  <h3 className="font-bold text-sm text-slate-200 mb-1">
                    {node.title}
                  </h3>
                  <p className="text-[11px] text-slate-400 mb-3 min-h-[32px] leading-tight">
                    {node.description}
                  </p>
                  <div
                    className={`inline-flex items-center gap-2 px-3 py-1 rounded-lg text-xs border ${getStatusColor(
                      node.status
                    )}`}
                  >
                    {getStatusIcon(node.status)}
                    <span className="capitalize font-medium">
                      {node.status}
                    </span>
                  </div>
                </div>

                <div className="px-4 py-2 border-t border-white/10 flex justify-between items-center bg-black/20 text-[11px] text-slate-500">
                  <span className="flex items-center gap-1">
                    <Clock className="w-3 h-3" /> Step {index + 1} of 4
                  </span>
                  <span className="font-mono text-indigo-400/80">GitHub Actions</span>
                </div>
              </div>

              {/* Connecting Arrow */}
              {index < nodes.length - 1 && (
                <div className="w-14 flex items-center justify-center shrink-0">
                  <div
                    className={`h-0.5 w-full ${
                      nodes[index].status === "success"
                        ? "bg-emerald-500"
                        : "bg-white/10"
                    }`}
                  ></div>
                  <ArrowRight
                    className={`w-5 h-5 -ml-3 z-10 ${
                      nodes[index].status === "success"
                        ? "text-emerald-500"
                        : "text-slate-600"
                    }`}
                  />
                </div>
              )}
            </React.Fragment>
          ))}
        </div>
      </div>

      {/* Execution Logs */}
      <div className="h-44 border-t border-white/10 bg-[#08080b] p-4 flex flex-col shrink-0">
        <div className="flex items-center justify-between mb-2 shrink-0">
          <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest flex items-center gap-2">
            <Activity className="w-3.5 h-3.5 text-indigo-400" />
            <span>GitHub Actions Live CI/CD Runner Konsole</span>
          </h3>
          <div className="flex items-center gap-3 text-[11px] text-slate-500 font-mono">
            <span>Runner: ubuntu-latest</span>
            <span>•</span>
            <span>Trigger: on.push</span>
          </div>
        </div>
        <div className="font-mono text-xs space-y-1 overflow-y-auto custom-scrollbar flex-1 bg-black/40 p-2.5 rounded-lg border border-white/5 select-text">
          {logs.map((line, idx) => (
            <div
              key={idx}
              className={
                line.includes("❌")
                  ? "text-red-400 font-semibold"
                  : line.includes("✅") || line.includes("🎉")
                  ? "text-emerald-400"
                  : line.includes("🚀") || line.includes("🧪")
                  ? "text-indigo-300 font-semibold"
                  : line.includes("⚠️")
                  ? "text-amber-300"
                  : "text-slate-400"
              }
            >
              {line}
            </div>
          ))}
        </div>
      </div>

    </div>
  );
}
