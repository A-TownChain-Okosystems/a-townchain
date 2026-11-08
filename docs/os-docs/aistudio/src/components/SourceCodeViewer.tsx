/// <reference types="vite/client" />
import React, { useState, useRef, useEffect } from "react";
import {
  Code,
  Copy,
  Check,
  FileCode,
  FolderClosed,
  ChevronRight,
  Wand2,
  Plus,
  GripVertical,
  File,
  FolderPlus,
  FilePlus,
  Play,
  Save,
  Activity,
  LayoutTemplate,
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import CodeMirror from "@uiw/react-codemirror";
import { javascript } from "@codemirror/lang-javascript";
import { python } from "@codemirror/lang-python";
import { oneDark } from "@codemirror/theme-one-dark";
import * as d3 from "d3";

// Verwenden von Vite's import.meta.glob um alle Quellcodes dynamisch als Text zu laden
const initialFiles = import.meta.glob("/src/**/*.{ts,tsx,css,py}", {
  query: "?raw",
  import: "default",
  eager: true,
}) as Record<string, string>;

export function SourceCodeViewer() {
  const [files, setFiles] = useState<Record<string, string>>(initialFiles);
  const [activeFile, setActiveFile] = useState<string>(
    () => localStorage.getItem("atc_ide_active_file") || "/src/App.tsx",
  );
  const [copied, setCopied] = useState(false);
  const [showAssistant, setShowAssistant] = useState(false);
  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    path: string;
  } | null>(null);
  const [draggedFile, setDraggedFile] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);
  const [terminalOutput, setTerminalOutput] = useState<{
    time: string;
    memory: string;
    status: number;
    raw: string;
  } | null>(null);
  const [showDependencyMap, setShowDependencyMap] = useState(false);
  const d3Container = useRef<HTMLDivElement>(null);

  useEffect(() => {
    localStorage.setItem("atc_ide_active_file", activeFile);
  }, [activeFile]);

  useEffect(() => {
    const handleOpenFile = (e: any) => {
      if (e.detail && e.detail.path && files[e.detail.path]) {
        setActiveFile(e.detail.path);
      }
    };
    window.addEventListener("open-file-in-ide", handleOpenFile);
    return () => window.removeEventListener("open-file-in-ide", handleOpenFile);
  }, [files]);

  const handleCopy = () => {
    if (files[activeFile]) {
      navigator.clipboard.writeText(files[activeFile]);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const runScript = () => {
    setTerminalOutput(null);
    setTimeout(() => {
      setTerminalOutput({
        time: (Math.random() * 1.5 + 0.1).toFixed(2) + "s",
        memory: Math.floor(Math.random() * 50 + 10) + "MB",
        status: 0,
        raw: "Executing script...\n[INFO] Loaded modules\n[SUCCESS] Execution completed successfully.",
      });
    }, 800);
  };

  const handleCodeChange = (val: string) => {
    setFiles((prev) => ({ ...prev, [activeFile]: val }));
    setIsSaving(true);
    // Simulate auto-save delay
    setTimeout(() => setIsSaving(false), 500);
  };

  const getBoilerplate = (ext: string) => {
    if (ext.endsWith(".tsx"))
      return `import React from 'react';\n\nexport function NewComponent() {\n  return (\n    <div>New Component</div>\n  );\n}`;
    if (ext.endsWith(".ts"))
      return `export const newModule = () => {\n  // Implementation here\n};`;
    return `/* New file */`;
  };

  useEffect(() => {
    if (showDependencyMap && d3Container.current) {
      d3.select(d3Container.current).selectAll("*").remove();

      const width = d3Container.current.clientWidth;
      const height = d3Container.current.clientHeight || 400;

      const svg = d3
        .select(d3Container.current)
        .append("svg")
        .attr("width", "100%")
        .attr("height", "100%")
        .attr("viewBox", `0 0 ${width} ${height}`);

      // Basic force graph of project files
      const nodes = Object.keys(files).map((id) => ({
        id,
        radius: 10 + Math.random() * 10,
      }));
      const links = [];
      for (let i = 1; i < nodes.length; i++) {
        // Randomly link nodes to simulate dependencies
        const target = nodes[Math.floor(Math.random() * i)];
        links.push({ source: nodes[i].id, target: target.id });
      }

      const simulation = d3
        .forceSimulation(nodes as any)
        .force(
          "link",
          d3
            .forceLink(links)
            .id((d: any) => d.id)
            .distance(80),
        )
        .force("charge", d3.forceManyBody().strength(-200))
        .force("center", d3.forceCenter(width / 2, height / 2));

      const link = svg
        .append("g")
        .attr("stroke", "#334155")
        .attr("stroke-width", 1.5)
        .selectAll("line")
        .data(links)
        .join("line");

      const nodeGroup = svg.append("g").selectAll("g").data(nodes).join("g");

      nodeGroup
        .append("circle")
        .attr("r", (d) => d.radius)
        .attr("fill", "#14b8a6")
        .attr("stroke", "#fff")
        .attr("stroke-width", 1.5);

      nodeGroup
        .append("text")
        .attr("dx", 15)
        .attr("dy", 4)
        .text((d: any) => d.id.split("/").pop() || "")
        .style("fill", "#94a3b8")
        .style("font-size", "10px");

      simulation.on("tick", () => {
        link
          .attr("x1", (d: any) => d.source.x)
          .attr("y1", (d: any) => d.source.y)
          .attr("x2", (d: any) => d.target.x)
          .attr("y2", (d: any) => d.target.y);

        nodeGroup.attr("transform", (d: any) => `translate(${d.x},${d.y})`);
      });

      return () => {
        simulation.stop();
      };
    }
  }, [showDependencyMap, files]);

  const fileList = Object.keys(files).sort();

  const containerRef = useRef<HTMLDivElement>(null);

  const handleContextMenu = (e: React.MouseEvent, path: string) => {
    e.preventDefault();
    e.stopPropagation();
    if (containerRef.current) {
      const rect = containerRef.current.getBoundingClientRect();
      setContextMenu({
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
        path,
      });
    }
  };

  const handleCreateFile = () => {
    const newName = prompt(
      "New file name (e.g. /src/components/NewFile.tsx):",
      "/src/components/NewFile.tsx",
    );
    if (newName && !files[newName]) {
      setFiles({ ...files, [newName]: getBoilerplate(newName) });
      setActiveFile(newName);
    }
    setContextMenu(null);
  };

  const handleCreateFolder = () => {
    prompt("New folder name (e.g. /src/utils/):", "/src/utils/");
    // Just a stub for UI, since we only show files in flat list currently
    setContextMenu(null);
  };

  const isCodeOpenSource = (path: string) => {
    return (
      path.includes("/backend/blockchain/") ||
      path.includes("/backend/p2p/") ||
      path.includes("/backend/crypto/") ||
      path.includes("smart-contracts")
    );
  };

  const onDragStart = (e: React.DragEvent, path: string) => {
    setDraggedFile(path);
    e.dataTransfer.setData("text/plain", path);
  };

  const onDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  const onDrop = (e: React.DragEvent, targetPath: string) => {
    e.preventDefault();
    if (!draggedFile || draggedFile === targetPath) return;

    // Simulate move by renaming
    const targetDir = targetPath.substring(0, targetPath.lastIndexOf("/"));
    const fileName = draggedFile.substring(draggedFile.lastIndexOf("/"));
    const newPath = `${targetDir}${fileName}`;

    if (newPath !== draggedFile) {
      const newFiles = { ...files };
      newFiles[newPath] = newFiles[draggedFile];
      delete newFiles[draggedFile];
      setFiles(newFiles);
      if (activeFile === draggedFile) setActiveFile(newPath);
    }
    setDraggedFile(null);
  };

  return (
    <div
      className="flex flex-col h-full bg-[#060a16] text-slate-200 relative"
      onClick={() => setContextMenu(null)}
      ref={containerRef}
    >
      <div className="win-titlebar flex items-center justify-between px-4 py-3 bg-[#090b14] border-b border-atc-border/50">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-teal-500/10 border border-teal-500/30">
            <Code className="w-5 h-5 text-teal-400" />
          </div>
          <div>
            <h1 className="font-bold text-white tracking-tight flex items-center gap-2">
              Code Center{" "}
              {isSaving ? (
                <span className="flex items-center text-[10px] text-amber-500">
                  <Activity className="w-3 h-3 mr-1 animate-spin" /> Saving...
                </span>
              ) : (
                <span className="flex items-center text-[10px] text-teal-500">
                  <Check className="w-3 h-3 mr-1" /> Saved
                </span>
              )}
            </h1>
            <p className="text-xs text-slate-400">
              Vollständiger Zugriff auf den Code von ATC-OS
            </p>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={() => setShowDependencyMap(!showDependencyMap)}
            className={`flex items-center gap-2 px-3 py-2 rounded-lg transition-colors text-sm font-bold border ${showDependencyMap ? "bg-indigo-500/20 text-indigo-400 border-indigo-500/30" : "bg-white/5 text-slate-400 border-white/10 hover:bg-white/10 hover:text-white"}`}
          >
            <LayoutTemplate className="w-4 h-4" /> Dependency Map
          </button>
          <button
            onClick={() => setShowAssistant(!showAssistant)}
            className={`flex items-center gap-2 px-3 py-2 rounded-lg transition-colors text-sm font-bold border ${showAssistant ? "bg-indigo-500/20 text-indigo-400 border-indigo-500/30" : "bg-white/5 text-slate-400 border-white/10 hover:bg-white/10 hover:text-white"}`}
          >
            <Wand2 className="w-4 h-4" /> ATC Assistant
          </button>
          <button
            onClick={runScript}
            className="flex items-center gap-2 px-4 py-2 rounded-lg bg-teal-500/20 text-teal-400 hover:bg-teal-500/30 transition-colors text-sm font-bold border border-teal-500/30"
          >
            <Play className="w-4 h-4" /> Run
          </button>
          <button
            onClick={handleCopy}
            className="flex items-center gap-2 px-4 py-2 rounded-lg bg-white/5 text-slate-300 hover:bg-white/10 transition-colors text-sm font-bold border border-white/10"
          >
            {copied ? (
              <Check className="w-4 h-4 text-emerald-400" />
            ) : (
              <Copy className="w-4 h-4" />
            )}
          </button>
        </div>
      </div>

      <div className="flex flex-1 min-h-0 relative">
        {/* Sidebar */}
        <div
          className="w-64 bg-[#090b14]/50 border-r border-atc-border/50 flex flex-col p-3"
          onContextMenu={(e) => handleContextMenu(e, "")}
        >
          <div className="flex items-center justify-between px-2 mb-3">
            <span className="text-xs font-bold tracking-wider text-slate-500 uppercase">
              Projektdateien
            </span>
            <button
              onClick={handleCreateFile}
              className="p-1 hover:bg-white/10 rounded text-slate-400 hover:text-white"
              title="New File"
            >
              <Plus className="w-3 h-3" />
            </button>
          </div>
          <div className="flex flex-col gap-1 overflow-y-auto custom-scrollbar">
            {fileList.map((path) => {
              const openSource = isCodeOpenSource(path);
              return (
                <div
                  key={path}
                  draggable
                  onDragStart={(e) => onDragStart(e, path)}
                  onDragOver={onDragOver}
                  onDrop={(e) => onDrop(e, path)}
                  onContextMenu={(e) => handleContextMenu(e, path)}
                  className={`flex items-center gap-2 px-3 py-2 text-xs text-left rounded-lg transition-colors cursor-grab active:cursor-grabbing border ${
                    draggedFile === path ? "opacity-50" : ""
                  } ${
                    activeFile === path
                      ? "bg-teal-500/10 text-teal-300 border-teal-500/20"
                      : "text-slate-400 border-transparent hover:bg-white/5 hover:text-slate-200"
                  }`}
                  onClick={() => setActiveFile(path)}
                >
                  <GripVertical className="w-3 h-3 text-slate-600 opacity-0 group-hover:opacity-100" />
                  <FileCode
                    className={`w-3.5 h-3.5 shrink-0 ${openSource ? "text-emerald-400" : "text-slate-500"}`}
                  />
                  <span className="truncate flex-1">
                    {path.replace("/src/", "")}
                  </span>
                  {openSource ? (
                    <span className="text-[8px] bg-emerald-500/20 text-emerald-400 px-1 rounded uppercase tracking-wider">
                      OSS
                    </span>
                  ) : (
                    <span className="text-[8px] bg-rose-500/20 text-rose-400 px-1 rounded uppercase tracking-wider">
                      PRO
                    </span>
                  )}
                </div>
              );
            })}
          </div>
        </div>

        {/* Editor Area */}
        <div className="flex-1 flex flex-col bg-[#050810] relative min-w-0">
          <div className="flex h-10 bg-[#090b14] border-b border-atc-border/50 items-center px-4 shrink-0">
            <span className="text-xs font-medium text-slate-400 flex items-center gap-2">
              <FolderClosed className="w-4 h-4 text-slate-500" /> src{" "}
              <ChevronRight className="w-3 h-3 text-slate-600" />{" "}
              <span className="text-slate-200">
                {activeFile.replace("/src/", "")}
              </span>
            </span>
          </div>

          <div className="flex-1 flex min-h-0">
            {showDependencyMap ? (
              <div
                id="page-code"
                className="flex-1 flex flex-col bg-[#050810] relative"
              >
                <div className="absolute inset-0" ref={d3Container}></div>
              </div>
            ) : (
              <div
                id="code-editor"
                className="flex-1 overflow-auto bg-[#282c34] relative"
              >
                {!isCodeOpenSource(activeFile) && (
                  <div className="absolute top-2 right-4 z-10 px-2 py-1 bg-rose-500/10 border border-rose-500/20 text-rose-300 text-[10px] font-bold tracking-wider rounded backdrop-blur-md uppercase">
                    Closed Source (Read Only)
                  </div>
                )}
                {isCodeOpenSource(activeFile) && (
                  <div className="absolute top-2 right-4 z-10 px-2 py-1 bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-[10px] font-bold tracking-wider rounded backdrop-blur-md uppercase">
                    MIT License (Open Source)
                  </div>
                )}
                <CodeMirror
                  value={files[activeFile] || "// Datei nicht gefunden"}
                  height="100%"
                  theme={oneDark}
                  readOnly={!isCodeOpenSource(activeFile)}
                  extensions={[
                    activeFile.endsWith(".py")
                      ? python()
                      : javascript({
                          jsx: true,
                          typescript:
                            activeFile.endsWith(".ts") ||
                            activeFile.endsWith(".tsx"),
                        }),
                  ]}
                  onChange={handleCodeChange}
                  className={`h-full text-sm font-mono ${!isCodeOpenSource(activeFile) ? "opacity-80" : ""}`}
                />
              </div>
            )}
          </div>

          {/* Terminal Output */}
          <div
            id="code-output"
            className="h-[200px] bg-[#090b14] border-t border-white/10 flex flex-col shrink-0"
          >
            <div className="flex items-center justify-between px-4 py-2 bg-black/40 border-b border-white/10 shrink-0">
              <span className="text-xs font-bold font-mono text-slate-400">
                TERMINAL OUTPUT
              </span>
              {terminalOutput && (
                <div className="flex items-center gap-4 text-[10px] font-mono font-bold bg-white/5 px-3 py-1 rounded">
                  <span className="text-teal-400">
                    Status: {terminalOutput.status}
                  </span>
                  <span className="text-slate-400">
                    Time: {terminalOutput.time}
                  </span>
                  <span className="text-indigo-400">
                    Memory: {terminalOutput.memory}
                  </span>
                </div>
              )}
            </div>
            <div className="flex-1 p-4 overflow-y-auto font-mono text-xs text-slate-300 whitespace-pre-wrap selection:bg-teal-500/30">
              {terminalOutput ? (
                <span
                  className={
                    terminalOutput.status === 0
                      ? "text-slate-300"
                      : "text-rose-400"
                  }
                >
                  {terminalOutput.raw}
                </span>
              ) : (
                <span className="text-slate-600">
                  Run a script to see output here.
                </span>
              )}
            </div>
          </div>

          <AnimatePresence>
            {showAssistant && (
              <motion.div
                initial={{ opacity: 0, x: 20 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: 20 }}
                className="absolute right-4 bottom-4 w-80 bg-[#090b14]/90 backdrop-blur-xl border border-indigo-500/30 rounded-xl shadow-2xl p-4 flex flex-col z-20"
              >
                <div className="flex items-center gap-2 mb-3 border-b border-white/10 pb-2">
                  <Wand2 className="w-4 h-4 text-indigo-400" />
                  <span className="text-sm font-bold text-white tracking-tight">
                    Code Assistant
                  </span>
                </div>
                <div className="flex-1 overflow-y-auto custom-scrollbar space-y-3 mb-3">
                  <div className="bg-indigo-500/10 border border-indigo-500/20 p-3 rounded-lg text-xs text-indigo-200">
                    <strong>Suggestion:</strong> This file is heavily importing
                    UI context. Consider breaking down the dashboard widgets
                    into smaller pure components.
                  </div>
                  <div className="bg-white/5 border border-white/10 p-3 rounded-lg flex flex-col gap-2">
                    <span className="text-xs text-slate-400 font-mono">
                      Quick Boilerplate
                    </span>
                    <button className="text-xs bg-white/10 hover:bg-white/20 text-white rounded p-1.5 transition-colors text-left font-mono">
                      {`export const withTheme = () => ...`}
                    </button>
                    <button className="text-xs bg-white/10 hover:bg-white/20 text-white rounded p-1.5 transition-colors text-left font-mono">
                      {`useATownSync()`} Hook Setup
                    </button>
                  </div>
                </div>
                <div className="relative">
                  <input
                    type="text"
                    placeholder="Ask ATC for code snippets..."
                    className="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-xs text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500/50"
                  />
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>

      {contextMenu && (
        <div
          className="absolute bg-[#090b14] border border-white/10 rounded-lg shadow-xl py-1 z-50 min-w-[160px]"
          style={{ top: contextMenu.y, left: contextMenu.x }}
        >
          <div className="px-3 py-1.5 text-[10px] uppercase font-bold text-slate-500 border-b border-white/5 mb-1">
            {contextMenu.path ? "File Action" : "Workspace Action"}
          </div>
          <button
            onClick={handleCreateFile}
            className="w-full text-left px-3 py-2 text-xs text-slate-300 hover:bg-white/10 hover:text-white flex items-center gap-2"
          >
            <FilePlus className="w-3 h-3" /> New File
          </button>
          <button
            onClick={handleCreateFolder}
            className="w-full text-left px-3 py-2 text-xs text-slate-300 hover:bg-white/10 hover:text-white flex items-center gap-2"
          >
            <FolderPlus className="w-3 h-3" /> New Folder
          </button>
        </div>
      )}
    </div>
  );
}
