/**
 * ATOS IDE
 * Software Version: 1.2.0
 * Developer: A-TownChain-Ökosystems
 * System: Lumino Language UI & Backend Editor
 */
import React, { useState, useRef, useEffect } from "react";
import {
  Play,
  Code2,
  TerminalSquare,
  FileCode2,
  ChevronRight,
  ChevronDown,
  Bug,
  StepForward,
  Square,
  ListTree,
  Bot,
  Search,
  ArrowUp,
  ArrowDown,
  X,
  Book,
  Wand2,
  Camera,
  Scissors,
  MonitorPlay,
  Save,
  Gamepad2,
  Globe,
  FolderGit2,
  Network,
  Database,
  MessageSquare,
  LayoutTemplate,
  FileCode,
  Volume2,
  Plug,
  Minus,
  Maximize2,
  Grid3x3,
  Wifi,
  Battery,
  Layers,
  Palette,
  GitBranch,
  GitCommit,
  History,
  Keyboard,
  Lightbulb,
  SplitSquareVertical,
  Beaker,
  Github,
  Zap,
  ShieldAlert,
  ShieldCheck,
  Terminal,
  Boxes,
  Server,
  Binary,
} from "lucide-react";
import { Lexer } from "./interpreter/lexer";
import { Parser } from "./interpreter/parser";
import { Evaluator, Environment } from "./interpreter/evaluator";
import { Program } from "./interpreter/types";
import { getDisplayCode, computeActualCode, getFoldRegions } from "./utils/codeFolding";
import { motion, AnimatePresence } from "motion/react";
import { AIChat } from "./components/AIChat";
import { DictionaryPanel } from "./components/DictionaryPanel";
import { ModuleLibrary } from "./components/ModuleLibrary";
import { RegistryPanel } from "./components/RegistryPanel";
import { SnippetsPanel } from "./components/SnippetsPanel";
import { AssetsPanel } from "./components/AssetsPanel";
import { AudioEnginePanel } from "./components/AudioEnginePanel";
import { ArchitectureWiki } from "./components/ArchitectureWiki";
import { WorkflowPanel } from "./components/WorkflowPanel";
import { DataExplorerPanel } from "./components/DataExplorerPanel";
import { ChatbotArchivePanel } from "./components/ChatbotArchivePanel";
import { TemplateDatabasePanel } from "./components/TemplateDatabasePanel";
import { ScriptDatabasePanel } from "./components/ScriptDatabasePanel";
import { PluginsPanel } from "./components/PluginsPanel";
import { VersionControlPanel } from "./components/VersionControlPanel";
import { TestingPanel } from "./components/TestingPanel";
import { CustomLinterPanel } from "./components/CustomLinterPanel";
import { CustomLintRule, DEFAULT_LINT_RULES, evaluateCustomLintRules } from "./types/linter";
import { AuditPanel } from "./components/AuditPanel";
import { AtcVmSimulatorPanel } from "./components/AtcVmSimulatorPanel";
import { AtcDocCompliancePanel } from "./components/AtcDocCompliancePanel";
import { GenesisConfiguratorPanel } from "./components/GenesisConfiguratorPanel";
import { RustWorkspaceGeneratorPanel } from "./components/RustWorkspaceGeneratorPanel";
import { LanguageStrategyPanel } from "./components/LanguageStrategyPanel";
import { GlobusFileFormatsPanel } from "./components/GlobusFileFormatsPanel";
import {
  runProjectAudit,
  applyAllAutoFixes,
  extractProjectTodos,
  syncTodoFileInWorkspace,
  generateLiveArchitectureDocs,
  syncWikiFileInWorkspace,
  generateTodoMarkdown,
  generateArchitectureMarkdown,
} from "./utils/auditEngine";
import { AutoSyncHubModal } from "./components/AutoSyncHubModal";
import { AutoSyncConfig, DEFAULT_AUTO_SYNC_CONFIG } from "./types/sync";
import {
  generateLiveRoadmap,
  generateRoadmapMarkdown,
  syncRoadmapFileInWorkspace,
  generateLiveSprints,
  generateSprintsMarkdown,
  syncSprintFileInWorkspace,
  generateLiveDocumentation,
  generateDocIndexMarkdown,
  syncDocumentationFilesInWorkspace,
  extractCurrentVersion,
  syncVersionFileInWorkspace,
  syncAllProjectArtifacts,
} from "./utils/projectSyncEngine";
import { MiniMap } from "./components/MiniMap";
import { KeyboardShortcutsModal } from "./components/KeyboardShortcutsModal";
import { CommandPalette } from "./components/CommandPalette";
import { ProjectManagerModal } from "./components/ProjectManagerModal";
import { GitHubSyncModal } from "./components/GitHubSyncModal";
import { CiCdGeneratorModal } from "./components/CiCdGeneratorModal";
import { AutoArchitectModal } from "./components/AutoArchitectModal";
import { GameScaffoldModal } from "./components/GameScaffoldModal";
import { PreviewPanel } from "./components/PreviewPanel";
import { TerminalPanel } from "./components/TerminalPanel";
import { SnakeGame } from "./components/SnakeGame";
import { ATXLoader, ATXHeader } from "./interpreter/atx_loader";
import { getCaretCoordinates } from "./utils/caret";
import Editor from "react-simple-code-editor";
import Prism from "prismjs";
import { Blocks } from "lucide-react";

Prism.languages.lumino = {
  comment: /\/\*[\s\S]*?\*\/|\/\/.*/,
  string: {
    pattern: /(^|[^\\])"(?:\\.|[^"\\\r\n])*"/,
    lookbehind: true,
    greedy: true,
  },
  keyword: /\b(?:let|print|if|else|true|false|while|func|return)\b/,
  boolean: /\b(?:true|false)\b/,
  function: /\b[a-zA-Z_]\w*(?=\s*\()/,
  "atc-extension": /\b[a-zA-Z0-9_]+\.(?:atc|atb|ats)\b/i,
  number: /\b\d+(?:\.\d+)?\b/,
  operator: /[+\-*\/=<>!]+/,
  punctuation: /[.,;()[\]{}]/,
  variable: /\b[a-zA-Z_]\w*\b/,
};

const INITIAL_CODE = `let greeting = "Hello from Lumino!"
print greeting

let x = 10
let y = 20
let result = x * y

print "The result is: " + result
`;

import { EXTENSION_CATEGORIES } from "./data/extensions";

export interface LintError {
  line: number;
  message: string;
  type: "warning" | "error";
  match: string;
  quickFix?: { label: string; action: string; replacement?: string; ruleId?: string };
}

export function lintLumino(code: string, customRules: CustomLintRule[] = []): LintError[] {
  const errors: LintError[] = [];
  const lines = code.split("\n");
  
  // 1. Unused variables
  const declarations = new Map<string, { line: number; count: number }>();
  // Match let or const, capture name
  const letPattern = /(?:let|const)\s+([a-zA-Z_]\w*)\b/g;
  let m;
  while ((m = letPattern.exec(code)) !== null) {
      const line = code.substring(0, m.index).split("\n").length - 1;
      declarations.set(m[1], { line, count: 0 });
  }
  
  for (const [varName, info] of declarations.entries()) {
      const usagePattern = new RegExp(`\\b${varName}\\b`, 'g');
      const count = (code.match(usagePattern) || []).length;
      if (count === 1) {
          errors.push({
             line: info.line,
             message: `Variable '${varName}' is declared but never used.`,
             type: "warning",
             match: varName
          });
      }
  }
  
  // 2. Infinite Loops
  const whilePattern = /while\s*\(\s*true\s*\)/g;
  while ((m = whilePattern.exec(code)) !== null) {
      const line = code.substring(0, m.index).split("\n").length - 1;
      errors.push({
          line,
          message: "Potential infinite loop detected.",
          type: "warning",
          match: m[0]
      });
  }
  
  // 3. Duplicate semicolons
  const semiPattern = /;;+/g;
  while ((m = semiPattern.exec(code)) !== null) {
      const line = code.substring(0, m.index).split("\n").length - 1;
      errors.push({
          line,
          message: "Unnecessary multiple semicolons.",
          type: "warning",
          match: m[0],
          quickFix: { label: "Remove duplicate semicolons", action: "remove_duplicate_semicolons" }
      });
  }

  // 4. Print without arguments
  const printPattern = /print\s*$/gm;
  while ((m = printPattern.exec(code)) !== null) {
      const line = code.substring(0, m.index).split("\n").length - 1;
      errors.push({
          line,
          message: "print statement missing arguments.",
          type: "error",
          match: m[0].trim()
      });
  }
  
  // 5. Uninitialized variables
  const uninitPattern = /let\s+([a-zA-Z_]\w*)\s*;/g;
  while ((m = uninitPattern.exec(code)) !== null) {
      const line = code.substring(0, m.index).split("\n").length - 1;
      errors.push({
          line,
          message: `Variable '${m[1]}' should be initialized.`,
          type: "warning",
          match: m[0],
          quickFix: { label: `Initialize '${m[1]}' to null`, action: "init_variable" }
      });
  }

  // 6. Custom User-Defined Lint Rules
  if (customRules && customRules.length > 0) {
    const customErrors = evaluateCustomLintRules(code, customRules);
    for (const cErr of customErrors) {
      errors.push({
        line: cErr.line,
        message: cErr.message,
        type: cErr.type,
        match: cErr.match,
        quickFix: cErr.quickFix ? {
          label: cErr.quickFix.label,
          action: cErr.quickFix.action,
          replacement: cErr.quickFix.replacement,
          ruleId: cErr.quickFix.ruleId,
        } : undefined,
      });
    }
  }

  return errors;
}

function insertSpanIntoHtml(htmlLine: string, searchStr: string, className: string, title: string) {
    const escapedSearchStr = searchStr.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
    const noTags = htmlLine.replace(/<[^>]*>/g, '');
    const matchStartTextIndex = noTags.indexOf(escapedSearchStr);
    
    if (matchStartTextIndex === -1) return htmlLine;
    
    let textIndex = 0;
    let inTag = false;
    let matchStartHtmlIndex = -1;
    let matchEndHtmlIndex = -1;
    
    for (let i = 0; i < htmlLine.length; i++) {
        if (htmlLine[i] === '<') inTag = true;
        
        if (!inTag) {
            if (textIndex === matchStartTextIndex) matchStartHtmlIndex = i;
            if (textIndex === matchStartTextIndex + escapedSearchStr.length) {
                matchEndHtmlIndex = i;
                break;
            }
            textIndex++;
        }
        
        if (htmlLine[i] === '>') {
            inTag = false;
        }
    }
    
    if (matchEndHtmlIndex === -1 && matchStartHtmlIndex !== -1) {
        matchEndHtmlIndex = htmlLine.length;
    }
    
    if (matchStartHtmlIndex !== -1) {
        const before = htmlLine.substring(0, matchStartHtmlIndex);
        const inside = htmlLine.substring(matchStartHtmlIndex, matchEndHtmlIndex);
        const after = htmlLine.substring(matchEndHtmlIndex);
        return `${before}<span class="${className}" title="${title}">${inside}</span>${after}`;
    }
    
    return htmlLine;
}

export interface FileState {
  name: string;
  content: string;
  iconColor: string;
  iconShape: string;
}

const INITIAL_FILES: FileState[] = [
  {
    name: "main.lm",
    content: INITIAL_CODE,
    iconColor: "text-cyan-400",
    iconShape: "◆",
  },
  {
    name: "core.atc",
    content:
      '// A-TownChain Core Configuration\nexport const CHUNK_SIZE = 1024;\nexport const CONSENSUS = "POS";',
    iconColor: "text-indigo-400",
    iconShape: "⬢",
  },
  {
    name: " treasury.ateco",
    content: '{\n  "inflation": 0.02,\n  "burn_rate": 0.001\n}',
    iconColor: "text-emerald-400",
    iconShape: "◈",
  },
  {
    name: "network.vx",
    content: '// Network core\nprint "Network sync..."',
    iconColor: "text-slate-500",
    iconShape: "◇",
  },
  {
    name: "config.json",
    content: '{\n  "network": "A-TownChain",\n  "version": "1.0"\n}',
    iconColor: "text-amber-500",
    iconShape: "◈",
  },
];

type SuggestionItem = { text: string; description?: string };

export const useUISounds = () => {
  const playSound = (type: 'click' | 'select' | 'error' | 'success') => {
    try {
      if (typeof window === 'undefined') return;
      const AudioContextClass = window.AudioContext || (window as any).webkitAudioContext;
      if (!AudioContextClass) return;
      
      const audioCtx = new AudioContextClass();
      const oscillator = audioCtx.createOscillator();
      const gainNode = audioCtx.createGain();

      oscillator.connect(gainNode);
      gainNode.connect(audioCtx.destination);

      if (type === 'select') {
        oscillator.type = 'sine';
        oscillator.frequency.setValueAtTime(600, audioCtx.currentTime); 
        oscillator.frequency.exponentialRampToValueAtTime(1200, audioCtx.currentTime + 0.05); 
        gainNode.gain.setValueAtTime(0, audioCtx.currentTime);
        gainNode.gain.linearRampToValueAtTime(0.05, audioCtx.currentTime + 0.02);
        gainNode.gain.exponentialRampToValueAtTime(0.001, audioCtx.currentTime + 0.1);
        oscillator.start(audioCtx.currentTime);
        oscillator.stop(audioCtx.currentTime + 0.1);
      } else if (type === 'click') {
        oscillator.type = 'triangle';
        oscillator.frequency.setValueAtTime(800, audioCtx.currentTime);
        gainNode.gain.setValueAtTime(0, audioCtx.currentTime);
        gainNode.gain.linearRampToValueAtTime(0.03, audioCtx.currentTime + 0.01);
        gainNode.gain.exponentialRampToValueAtTime(0.001, audioCtx.currentTime + 0.05);
        oscillator.start(audioCtx.currentTime);
        oscillator.stop(audioCtx.currentTime + 0.05);
      } else if (type === 'error') {
        oscillator.type = 'sawtooth';
        oscillator.frequency.setValueAtTime(150, audioCtx.currentTime);
        oscillator.frequency.exponentialRampToValueAtTime(100, audioCtx.currentTime + 0.2);
        gainNode.gain.setValueAtTime(0, audioCtx.currentTime);
        gainNode.gain.linearRampToValueAtTime(0.05, audioCtx.currentTime + 0.05);
        gainNode.gain.exponentialRampToValueAtTime(0.001, audioCtx.currentTime + 0.2);
        oscillator.start(audioCtx.currentTime);
        oscillator.stop(audioCtx.currentTime + 0.2);
      } else if (type === 'success') {
        oscillator.type = 'sine';
        oscillator.frequency.setValueAtTime(400, audioCtx.currentTime);
        oscillator.frequency.setValueAtTime(600, audioCtx.currentTime + 0.1);
        gainNode.gain.setValueAtTime(0, audioCtx.currentTime);
        gainNode.gain.linearRampToValueAtTime(0.05, audioCtx.currentTime + 0.05);
        gainNode.gain.linearRampToValueAtTime(0.001, audioCtx.currentTime + 0.2);
        oscillator.start(audioCtx.currentTime);
        oscillator.stop(audioCtx.currentTime + 0.2);
      }
    } catch (e) {
      // Audio might not be supported or allowed by browser policy before interaction
    }
  };

  return playSound;
};

export default function App() {
  const playUISound = useUISounds();
  const [files, setFiles] = useState<FileState[]>(() => {
    try {
      const saved = localStorage.getItem("atos-files");
      if (saved) return JSON.parse(saved);
    } catch (e) {}
    return INITIAL_FILES;
  });

  const [isAutoSaveEnabled, setIsAutoSaveEnabled] = useState(
    () => localStorage.getItem("atos-autosave") !== "false"
  );

  useEffect(() => {
    if (isAutoSaveEnabled) {
      localStorage.setItem("atos-files", JSON.stringify(files));
    }
  }, [files, isAutoSaveEnabled]);

  useEffect(() => {
    localStorage.setItem("atos-autosave", isAutoSaveEnabled.toString());
  }, [isAutoSaveEnabled]);

  const [theme, setTheme] = useState(
    () => localStorage.getItem("atos-theme") || "ATOS Dark",
  );

  const [editorTheme, setEditorTheme] = useState(
    () => localStorage.getItem("atos-editor-theme") || "prism-tomorrow"
  );

  useEffect(() => {
    localStorage.setItem("atos-theme", theme);
  }, [theme]);

  useEffect(() => {
    const handleThemeChange = (e: any) => {
      if (e.detail && typeof e.detail === 'string') {
        setTheme(e.detail);
      }
    };
    window.addEventListener('atos-theme-change', handleThemeChange);
    return () => window.removeEventListener('atos-theme-change', handleThemeChange);
  }, []);

  useEffect(() => {
    localStorage.setItem("atos-editor-theme", editorTheme);
    let link = document.getElementById("prism-theme-link") as HTMLLinkElement;
    if (!link) {
      link = document.createElement("link");
      link.id = "prism-theme-link";
      link.rel = "stylesheet";
      document.head.appendChild(link);
    }
    link.href = `https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/${editorTheme}.min.css`;
  }, [editorTheme]);


  const [activeFileIndex, setActiveFileIndex] = useState(0);
  const [draggedFileIndex, setDraggedFileIndex] = useState<number | null>(null);
  const [dragOverFileIndex, setDragOverFileIndex] = useState<number | null>(null);
  const [isScaffoldConfirmOpen, setIsScaffoldConfirmOpen] = useState(false);
  const [isAutoArchitectOpen, setIsAutoArchitectOpen] = useState(false);
  const [isProjectManagerOpen, setIsProjectManagerOpen] = useState(false);
  const [isGameScaffoldOpen, setIsGameScaffoldOpen] = useState(false);
  const [projectScaffolded, setProjectScaffolded] = useState(false);
  const [fileHistory, setFileHistory] = useState<FileState[][]>([]);

  const handleDragStart = (index: number) => {
    setDraggedFileIndex(index);
  };

  const handleDragEnter = (index: number) => {
    setDragOverFileIndex(index);
  };

  const handleDragEnd = () => {
    setDraggedFileIndex(null);
    setDragOverFileIndex(null);
  };

  const handleDrop = (index: number) => {
    if (draggedFileIndex === null || draggedFileIndex === index) {
      handleDragEnd();
      return;
    }
    
    const newFiles = [...files];
    const [removed] = newFiles.splice(draggedFileIndex, 1);
    newFiles.splice(index, 0, removed);
    setFiles(newFiles);
    
    if (activeFileIndex === draggedFileIndex) {
      setActiveFileIndex(index);
    } else if (activeFileIndex > draggedFileIndex && activeFileIndex <= index) {
      setActiveFileIndex(activeFileIndex - 1);
    } else if (activeFileIndex < draggedFileIndex && activeFileIndex >= index) {
      setActiveFileIndex(activeFileIndex + 1);
    }
    
    handleDragEnd();
  };

  const handleRollback = () => {
    if (fileHistory.length > 0) {
      const prev = fileHistory[fileHistory.length - 1];
      setFiles(prev);
      setFileHistory((prevHistory) => prevHistory.slice(0, -1));
      if (activeFileIndex >= prev.length) {
        setActiveFileIndex(Math.max(0, prev.length - 1));
      }
    }
  };

  const confirmScaffold = () => {
    setFileHistory((prev) => [...prev, JSON.parse(JSON.stringify(files))]);
    const scaffoldFiles: FileState[] = [
      {
        name: "docs/Roadmap.wiki",
        content: `# Projekt-Roadmap\n\n## Schritt 1: Backend\n- [ ] Express.js Server initialisieren\n- [ ] Umgebungsvariablen konfigurieren\n- [ ] Authentifizierungs-Routen einrichten\n- [ ] Datenbankschema erstellen\n\n## Schritt 2: Frontend\n- [ ] React Router einrichten\n- [ ] Core UI Layout Komponente erstellen\n- [ ] Dashboard View implementieren\n- [ ] Backend API anbinden\n\n## Schritt 3: Tests & Deployment\n- [ ] Funktionen verbinden und testen\n- [ ] Dockerfile schreiben\n- [ ] CI/CD Pipeline einrichten\n`,
        iconColor: "text-yellow-400",
        iconShape: "📝",
      },
      {
        name: "src/backend/server.ts",
        content: `import express from 'express';\n\nconst app = express();\n\napp.listen(3000, () => {\n  console.log("Server läuft...");\n});\n`,
        iconColor: "text-green-500",
        iconShape: "⚡",
      },
      {
        name: "src/frontend/index.tsx",
        content: `import React from 'react';\n\nexport function App() {\n  return <div>Hallo Welt</div>;\n}\n`,
        iconColor: "text-blue-500",
        iconShape: "⚛",
      },
      ...files,
    ];
    setFiles(scaffoldFiles);
    setActiveFileIndex(0);
    setIsScaffoldConfirmOpen(false);
    setProjectScaffolded(true);
    setIsChatOpen(true);
  };

  const code = files[activeFileIndex]?.content || "";
  const setCode = (newContent: string) => {
    setFiles((prev) =>
      prev.map((f, i) =>
        i === activeFileIndex ? { ...f, content: newContent } : f,
      ),
    );
  };

  const insertCodeStr = (snippet: string) => {
    const ta = getTextArea();
    if (ta) {
      const start = ta.selectionStart;
      const end = ta.selectionEnd;
      const newCode = code.substring(0, start) + snippet + code.substring(end);
      setCode(newCode);

      setTimeout(() => {
        const nta = getTextArea();
        if (nta) {
          nta.focus();
          nta.selectionStart = nta.selectionEnd = start + snippet.length;
        }
      }, 0);
    } else {
      setCode(
        code + (code.length > 0 && !code.endsWith("\n") ? "\n" : "") + snippet,
      );
    }
  };

  const [output, setOutput] = useState<string[]>([]);
  const [syntaxErrors, setSyntaxErrors] = useState<{line: number, message: string}[]>([]);

  const applyQuickFix = (fix: { label: string, action: string, replacement?: string }, errorLineNum: number, matchStr: string) => {
    let newCode = code;
    const lines = newCode.split('\n');
    let targetLine = lines[errorLineNum];
    if (targetLine !== undefined) {
       if (fix.action === 'remove_duplicate_semicolons') {
          targetLine = targetLine.replace(matchStr, ';');
       } else if (fix.action === 'init_variable') {
          targetLine = targetLine.replace(matchStr, matchStr.replace(';', ' = null;'));
       } else if (fix.action === 'replace_match' && fix.replacement !== undefined) {
          targetLine = targetLine.replace(matchStr, fix.replacement);
       } else if (fix.action === 'trim_trailing') {
          targetLine = targetLine.trimEnd();
       } else if (fix.action === 'custom_replace' && fix.replacement !== undefined) {
          targetLine = targetLine.replace(matchStr, fix.replacement);
       }
       lines[errorLineNum] = targetLine;
       setCode(lines.join('\n'));
       showEditorToast(`Applied: ${fix.label}`, "info");
    }
  };
  
  const [editorToast, setEditorToast] = useState<{message: string, type: "error" | "warning" | "info", id: number} | null>(null);

  const showEditorToast = (message: string, type: "error" | "warning" | "info") => {
    if (type === 'error') playUISound('error');
    else if (type === 'warning') playUISound('click');
    else if (type === 'info') playUISound('success');
    setEditorToast({ message, type, id: Date.now() });
  };

  useEffect(() => {
    if (editorToast) {
      const timer = setTimeout(() => setEditorToast(null), 5000);
      return () => clearTimeout(timer);
    }
  }, [editorToast]);

  const [isRunning, setIsRunning] = useState(false);
  const [isChatOpen, setIsChatOpen] = useState(false);
  const [isDictionaryOpen, setIsDictionaryOpen] = useState(false);
  const [isLibraryOpen, setIsLibraryOpen] = useState(false);
  const [isRegistryOpen, setIsRegistryOpen] = useState(false);
  const [isSnippetsOpen, setIsSnippetsOpen] = useState(false);
  const [isAssetsOpen, setIsAssetsOpen] = useState(false);
  const [isWikiOpen, setIsWikiOpen] = useState(false);
  const [isWorkflowOpen, setIsWorkflowOpen] = useState(false);
  const [isDataExplorerOpen, setIsDataExplorerOpen] = useState(false);
  const [isChatbotArchiveOpen, setIsChatbotArchiveOpen] = useState(false);
  const [isTemplateDatabaseOpen, setIsTemplateDatabaseOpen] = useState(false);
  const [isScriptDatabaseOpen, setIsScriptDatabaseOpen] = useState(false);
  const [isAudioEngineOpen, setIsAudioEngineOpen] = useState(false);
  const [isPluginsOpen, setIsPluginsOpen] = useState(false);
  const [isVersionControlOpen, setIsVersionControlOpen] = useState(false);
  const [isGitHubSyncOpen, setIsGitHubSyncOpen] = useState(false);
  const [isShortcutsOpen, setIsShortcutsOpen] = useState(false);
  const [isTestingOpen, setIsTestingOpen] = useState(false);
  const [isCiCdGeneratorOpen, setIsCiCdGeneratorOpen] = useState(false);
  const [isCustomLinterOpen, setIsCustomLinterOpen] = useState(false);
  const [isAuditOpen, setIsAuditOpen] = useState(false);
  const [isAtcVmOpen, setIsAtcVmOpen] = useState(false);
  const [isAtcComplianceOpen, setIsAtcComplianceOpen] = useState(false);
  const [isGenesisOpen, setIsGenesisOpen] = useState(false);
  const [isRustWorkspaceOpen, setIsRustWorkspaceOpen] = useState(false);
  const [isLanguageStrategyOpen, setIsLanguageStrategyOpen] = useState(false);
  const [isGlobusFormatsOpen, setIsGlobusFormatsOpen] = useState(false);

  const [autoSyncTodo, setAutoSyncTodo] = useState<boolean>(() => {
    try {
      const saved = localStorage.getItem("atos_auto_sync_todo");
      return saved !== null ? saved === "true" : true;
    } catch {
      return true;
    }
  });

  const [autoSyncWiki, setAutoSyncWiki] = useState<boolean>(() => {
    try {
      const saved = localStorage.getItem("atos_auto_sync_wiki");
      return saved !== null ? saved === "true" : true;
    } catch {
      return true;
    }
  });

  useEffect(() => {
    try {
      localStorage.setItem("atos_auto_sync_todo", String(autoSyncTodo));
    } catch (e) {
      console.warn(e);
    }
  }, [autoSyncTodo]);

  useEffect(() => {
    try {
      localStorage.setItem("atos_auto_sync_wiki", String(autoSyncWiki));
    } catch (e) {
      console.warn(e);
    }
  }, [autoSyncWiki]);

  // Live Audit analysis of the current files
  const liveAuditSummary = React.useMemo(() => {
    return runProjectAudit(files);
  }, [files]);

  // Automatically keep TODO.md synchronized if autoSyncTodo is true
  useEffect(() => {
    if (!autoSyncTodo) return;
    const todos = extractProjectTodos(files, liveAuditSummary.findings);
    const expected = generateTodoMarkdown(todos);
    const existing = files.find((f) => f.name === "TODO.md");
    if (!existing || existing.content !== expected) {
      setFiles((prev) => syncTodoFileInWorkspace(prev, todos));
    }
  }, [files, autoSyncTodo, liveAuditSummary.findings]);

  // Automatically keep ARCHITECTURE.md synchronized if autoSyncWiki is true
  useEffect(() => {
    if (!autoSyncWiki) return;
    const liveDoc = generateLiveArchitectureDocs(files, liveAuditSummary);
    const expected = generateArchitectureMarkdown(liveDoc);
    const existing = files.find((f) => f.name === "ARCHITECTURE.md");
    if (!existing || existing.content !== expected) {
      setFiles((prev) => syncWikiFileInWorkspace(prev, liveDoc));
    }
  }, [files, autoSyncWiki, liveAuditSummary]);

  const [customLintRules, setCustomLintRules] = useState<CustomLintRule[]>(() => {
    try {
      const saved = localStorage.getItem("lumino_custom_lint_rules");
      if (saved) {
        const parsed = JSON.parse(saved);
        if (Array.isArray(parsed) && parsed.length > 0) return parsed;
      }
    } catch (e) {
      console.warn("Failed to load custom lint rules from localStorage", e);
    }
    return DEFAULT_LINT_RULES;
  });

  useEffect(() => {
    try {
      localStorage.setItem("lumino_custom_lint_rules", JSON.stringify(customLintRules));
    } catch (e) {
      console.warn("Failed to save custom lint rules to localStorage", e);
    }
  }, [customLintRules]);

  const handleSaveLintConfigFile = (filename: string, content: string) => {
    setFileHistory((prev) => [...prev, JSON.parse(JSON.stringify(files))]);
    let updatedFiles = [...files];
    const existingIndex = updatedFiles.findIndex((f) => f.name === filename);
    if (existingIndex >= 0) {
      updatedFiles[existingIndex] = { ...updatedFiles[existingIndex], content };
    } else {
      updatedFiles.push({
        name: filename,
        content,
        iconColor: "text-amber-400",
        iconShape: "⚙",
      });
    }
    setFiles(updatedFiles);
    showEditorToast(`'${filename}' erfolgreich im Workspace gespeichert!`, "info");
  };

  const handleSaveWorkflowFile = (
    filePath: string,
    content: string,
    companionFile?: { name: string; content: string }
  ) => {
    setFileHistory((prev) => [...prev, JSON.parse(JSON.stringify(files))]);
    let updatedFiles = [...files];

    const existingIndex = updatedFiles.findIndex((f) => f.name === filePath);
    if (existingIndex >= 0) {
      updatedFiles[existingIndex] = { ...updatedFiles[existingIndex], content };
      setActiveFileIndex(existingIndex);
    } else {
      const newWorkflowFile: FileState = {
        name: filePath,
        content,
        iconColor: "text-amber-400",
        iconShape: "⚙",
      };
      updatedFiles.push(newWorkflowFile);
      setActiveFileIndex(updatedFiles.length - 1);
    }

    if (companionFile) {
      const compIdx = updatedFiles.findIndex((f) => f.name === companionFile.name);
      if (compIdx >= 0) {
        updatedFiles[compIdx] = { ...updatedFiles[compIdx], content: companionFile.content };
      } else {
        const newTestFile: FileState = {
          name: companionFile.name,
          content: companionFile.content,
          iconColor: "text-emerald-400",
          iconShape: "🧪",
        };
        updatedFiles.push(newTestFile);
      }
    }

    setFiles(updatedFiles);
    closeAllPanels();
    showEditorToast(
      `CI/CD Workflow '${filePath}' erfolgreich in Workspace eingebunden!`,
      "info"
    );
  };

  const [isWorkspaceMinimized, setIsWorkspaceMinimized] = useState(false);
  const [isWorkspaceMaximized, setIsWorkspaceMaximized] = useState(false);
  const [isStartMenuOpen, setIsStartMenuOpen] = useState(false);
  const [isCommandPaletteOpen, setIsCommandPaletteOpen] = useState(false);
  const [fileSearchQuery, setFileSearchQuery] = useState("");
  const fileSearchInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'p') {
        e.preventDefault();
        setIsCommandPaletteOpen(true);
      }
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'f') {
        e.preventDefault();
        fileSearchInputRef.current?.focus();
      }
    };
    window.addEventListener('keydown', handleGlobalKeyDown);
    return () => window.removeEventListener('keydown', handleGlobalKeyDown);
  }, []);

  const closeAllPanels = () => {
    setIsDictionaryOpen(false);
    setIsLibraryOpen(false);
    setIsRegistryOpen(false);
    setIsSnippetsOpen(false);
    setIsAssetsOpen(false);
    setIsAudioEngineOpen(false);
    setIsWikiOpen(false);
    setIsWorkflowOpen(false);
    setIsDataExplorerOpen(false);
    setIsChatbotArchiveOpen(false);
    setIsTemplateDatabaseOpen(false);
    setIsScriptDatabaseOpen(false);
    setIsPluginsOpen(false);
    setIsVersionControlOpen(false);
    setIsTestingOpen(false);
    setIsCustomLinterOpen(false);
    setIsAuditOpen(false);
    setIsAtcVmOpen(false);
    setIsAtcComplianceOpen(false);
    setIsGenesisOpen(false);
    setIsRustWorkspaceOpen(false);
    setIsLanguageStrategyOpen(false);
    setIsGlobusFormatsOpen(false);
  };

  const [activeBottomTab, setActiveBottomTab] = useState<
    "console" | "terminal" | "debugger" | "preview" | "game"
  >("console");
  const [debuggerBuffer, setDebuggerBuffer] = useState<ArrayBuffer | null>(
    null,
  );
  const [debuggerHeader, setDebuggerHeader] = useState<any>(null);
  const [debuggerError, setDebuggerError] = useState<string>("");
  const editorRef = useRef<HTMLDivElement>(null);
  const getTextArea = () =>
    editorRef.current?.querySelector("textarea") as HTMLTextAreaElement | null;

  const [searchQuery, setSearchQuery] = useState("");
  const [replaceQuery, setReplaceQuery] = useState("");
  const [searchIndex, setSearchIndex] = useState(-1);
  const [searchResults, setSearchResults] = useState<
    { fileIndex: number; start: number; length: number; preview: string }[]
  >([]);
  const [isSearchVisible, setIsSearchVisible] = useState(false);

  const [suggestions, setSuggestions] = useState<SuggestionItem[]>([]);
  const [suggestionPos, setSuggestionPos] = useState({ top: 0, left: 0 });
  const [selectedSuggestion, setSelectedSuggestion] = useState(0);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [currentWordBounds, setCurrentWordBounds] = useState({
    start: 0,
    end: 0,
  });

  const [breakpoints, setBreakpoints] = useState<Set<number>>(new Set());
  const [foldedLines, setFoldedLines] = useState<Set<number>>(new Set());
  const [debugState, setDebugState] = useState<{
    program: Program | null;
    env: Environment | null;
    evaluator: Evaluator | null;
    currentIndex: number;
    variables: Record<string, any>;
    activeLine: number | null;
  } | null>(null);

  const [advancedSyntaxHighlighting, setAdvancedSyntaxHighlighting] = useState(false);
  const [splitActiveFileIndex, setSplitActiveFileIndex] = useState<number | null>(null);
  const [watches, setWatches] = useState<string[]>([]);
  const [newWatchInput, setNewWatchInput] = useState<string>("");

  const toggleBreakpoint = (line: number) => {
    setBreakpoints((prev) => {
      const next = new Set(prev);
      if (next.has(line)) next.delete(line);
      else next.add(line);
      return next;
    });
  };

  const handleDebugFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (evt) => {
      const buffer = evt.target?.result as ArrayBuffer;
      setDebuggerBuffer(buffer);
      try {
        const header = ATXLoader.parseHeader(buffer);
        setDebuggerHeader(header);
        setDebuggerError("");
      } catch (err: any) {
        setDebuggerHeader(null);
        setDebuggerError(err.message);
        playUISound('error');
      }
    };
    reader.readAsArrayBuffer(file);
  };

  const applyWatermark = (logs: string[]): string[] => {
    return [
      "[SYSTEM] Software Version: 1.2.0 | Developer: A-TownChain-Ökosystems",
      ...logs,
    ];
  };

  const handleSnapshot = () => {
    const snapshotStr = JSON.stringify(
      {
        watermark:
          "Software Version: 1.2.0 | Developer: A-TownChain-Ökosystems",
        version: "1.2.0",
        developer: "A-TownChain-Ökosystems",
        files,
        timestamp: new Date().toISOString(),
      },
      null,
      2,
    );
    const blob = new Blob([snapshotStr], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `atos-snapshot-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const getActiveBottomTabClass = (
    tab: "console" | "terminal" | "debugger" | "preview" | "game",
  ) => {
    return activeBottomTab === tab
      ? "text-[10px] font-bold text-slate-400 uppercase tracking-widest border-b border-cyan-500 h-full flex items-center pt-0.5 cursor-pointer"
      : "text-[10px] font-bold text-slate-600 uppercase tracking-widest flex items-center h-full cursor-pointer hover:text-slate-500";
  };

  const runCode = () => {
    setIsRunning(true);
    setOutput(applyWatermark([]));
    setSyntaxErrors([]);
    setDebugState(null);

    setTimeout(() => {
      const lexer = new Lexer(code);
      const parser = new Parser(lexer);
      const program = parser.parseProgram();

      if (parser.errors.length > 0) {
        const parsedErrors = parser.errors.map(err => {
          const match = err.match(/at line (\d+)/);
          return {
            line: match ? parseInt(match[1], 10) : -1,
            message: err,
          };
        });
        setSyntaxErrors(parsedErrors);
        setOutput(
          applyWatermark(parser.errors.map((err) => `Syntax Error: ${err}`)),
        );
        setIsRunning(false);
        return;
      }

      const env = new Environment();
      const evaluator = new Evaluator(env);
      evaluator.eval(program);

      setOutput(applyWatermark(evaluator.getOutput()));
      setIsRunning(false);
    }, 100);
  };

  const handleFormatCode = () => {
    const formatted = formatLuminoCode(code);
    setCode(formatted);
  };

  const formatLuminoCode = (source: string) => {
    const lines = source.split("\n");
    let formatted: string[] = [];
    let indent = 0;

    for (let i = 0; i < lines.length; i++) {
      let line = lines[i].trim();

      if (!line) {
        if (formatted.length > 0 && formatted[formatted.length - 1] !== "") {
          formatted.push("");
        }
        continue;
      }

      if (line.startsWith("}")) {
        indent = Math.max(0, indent - 1);
      } else if (line.startsWith("else") || line.startsWith("elif")) {
        indent = Math.max(0, indent - 1);
      }

      let formattedLine = "";

      const parts = line.split('"');
      for (let j = 0; j < parts.length; j++) {
        if (j % 2 === 0) {
          let part = parts[j];
          part = part
            .replace(/\s*(==|!=|>=|<=|=>|->)\s*/g, " $1 ")
            .replace(/\s*(=|\+|-|\*|\/|<|>)\s*/g, (match, op, offset, str) => {
              const prev = offset > 0 ? str[offset - 1] : "";
              const next = offset + op.length < str.length ? str[offset + op.length] : "";
              if (["=", "!", "<", ">", "-"].includes(prev)) return match;
              if (["=", "!", "<", ">", ">"].includes(next)) return match;
              return ` ${op} `;
            })
            .replace(/\s*,\s*/g, ", ")
            .replace(/\s+/g, " ");

          part = part
            .replace(/\(\s+/g, "(")
            .replace(/\s+\)/g, ")")
            .replace(/\[\s+/g, "[")
            .replace(/\s+\]/g, "]");

          parts[j] = part;
        }
      }

      formattedLine = "  ".repeat(indent) + parts.join('"').trim();
      formatted.push(formattedLine);

      if (line.endsWith("{") || (line.startsWith("else") && !line.includes("{"))) {
        if (line.endsWith("{")) {
            indent++;
        }
      }
    }
    return formatted.join("\n").trim() + "\n";
  };

  const handleStartDebug = () => {
    setIsRunning(true);
    setOutput([]);

    const lexer = new Lexer(code);
    const parser = new Parser(lexer);
    const program = parser.parseProgram();

    if (parser.errors.length > 0) {
      setOutput(parser.errors.map((err) => `Syntax Error: ${err}`));
      setIsRunning(false);
      return;
    }

    const env = new Environment();
    const evaluator = new Evaluator(env);

    let initialIndex = 0;

    setDebugState({
      program,
      env,
      evaluator,
      currentIndex: initialIndex,
      variables: {},
      activeLine: program.statements[initialIndex]?.line || null,
    });
  };

  const stepDebug = () => {
    if (
      !debugState ||
      !debugState.program ||
      !debugState.evaluator ||
      !debugState.env
    )
      return;

    const { program, env, evaluator, currentIndex } = debugState;
    if (currentIndex >= program.statements.length) {
      stopDebug();
      return;
    }

    const stmt = program.statements[currentIndex];
    const result = evaluator.eval(stmt);

    if (result instanceof Error) {
      playUISound('error');
      setOutput(
        applyWatermark([...evaluator.getOutput(), `Error: ${result.message}`]),
      );
      stopDebug();
      return;
    }

    const nextIndex = currentIndex + 1;
    setOutput(applyWatermark([...evaluator.getOutput()]));

    if (nextIndex >= program.statements.length) {
      setDebugState({
        ...debugState,
        currentIndex: nextIndex,
        variables: env.getStore(),
        activeLine: null,
      });
      setIsRunning(false);
    } else {
      setDebugState({
        ...debugState,
        currentIndex: nextIndex,
        variables: env.getStore(),
        activeLine: program.statements[nextIndex]?.line || null,
      });
    }
  };

  const continueDebug = () => {
    if (
      !debugState ||
      !debugState.program ||
      !debugState.evaluator ||
      !debugState.env
    )
      return;

    let { program, env, evaluator, currentIndex } = debugState;
    let nextIndex = currentIndex;

    while (nextIndex < program.statements.length) {
      const stmt = program.statements[nextIndex];
      const result = evaluator.eval(stmt);

      if (result instanceof Error) {
        playUISound('error');
        setOutput(
          applyWatermark([
            ...evaluator.getOutput(),
            `Error: ${result.message}`,
          ]),
        );
        stopDebug();
        return;
      }

      nextIndex++;
      if (
        nextIndex < program.statements.length &&
        breakpoints.has(program.statements[nextIndex].line)
      ) {
        break;
      }
    }

    setOutput(applyWatermark([...evaluator.getOutput()]));

    if (nextIndex >= program.statements.length) {
      setDebugState({
        ...debugState,
        currentIndex: nextIndex,
        variables: env.getStore(),
        activeLine: null,
      });
      setIsRunning(false);
    } else {
      setDebugState({
        ...debugState,
        currentIndex: nextIndex,
        variables: env.getStore(),
        activeLine: program.statements[nextIndex]?.line || null,
      });
    }
  };

  const stopDebug = () => {
    setIsRunning(false);
    setDebugState(null);
  };

  useEffect(() => {
    if (!searchQuery || !isSearchVisible) {
      setSearchResults([]);
      setSearchIndex(-1);
      return;
    }
    const lowerQuery = searchQuery.toLowerCase();
    const results: {
      fileIndex: number;
      start: number;
      length: number;
      preview: string;
    }[] = [];

    files.forEach((file, fileIndex) => {
      const lowerCode = file.content.toLowerCase();
      let i = lowerCode.indexOf(lowerQuery);
      while (i !== -1) {
        // extract a short preview
        const previewStart = Math.max(0, i - 15);
        const previewEnd = Math.min(
          lowerCode.length,
          i + lowerQuery.length + 15,
        );
        const preview = file.content
          .substring(previewStart, previewEnd)
          .replace(/\n/g, " ");

        results.push({
          fileIndex,
          start: i,
          length: lowerQuery.length,
          preview: "..." + preview + "...",
        });
        i = lowerCode.indexOf(lowerQuery, i + 1);
      }
    });

    setSearchResults(results);
    if (results.length > 0) {
      setSearchIndex(0);
    } else {
      setSearchIndex(-1);
    }
  }, [files, searchQuery, isSearchVisible]);

  useEffect(() => {
    if (searchIndex !== -1 && searchResults.length > 0) {
      const result = searchResults[searchIndex];
      // Switch active file if need be
      if (activeFileIndex !== result.fileIndex) {
        setActiveFileIndex(result.fileIndex);
      }
      setTimeout(() => {
        const textarea = getTextArea();
        if (textarea) {
          textarea.focus();
          textarea.setSelectionRange(
            result.start,
            result.start + result.length,
          );
        }
      }, 50); // slight delay to allow active file state to update
    }
  }, [searchIndex, searchResults]);

  const extractVariables = (currentCode: string) => {
    const vars = new Set<string>();
    const matches = currentCode.matchAll(/let\s+([a-zA-Z_]\w*)/g);
    for (const match of matches) {
      if (match[1]) vars.add(match[1]);
    }
    return Array.from(vars);
  };

  const updateSuggestions = (currentText: string, selectionStart: number) => {
    if (selectionStart === 0) {
      setShowSuggestions(false);
      return;
    }

    const textBeforeCursor = currentText.slice(0, selectionStart);
    const match = textBeforeCursor.match(/(\.?[a-zA-Z_]\w*)$/);

    if (!match) {
      setShowSuggestions(false);
      return;
    }

    const currentWord = match[1];
    const startIndex = match.index!;
    const endIndex = selectionStart;

    const keywordDefs = [
      { text: "let", description: "Declares a new variable" },
      { text: "print", description: "Prints output to the console" },
      { text: "if", description: "Conditional statement" },
      { text: "else", description: "Alternative branch in a conditional" },
      { text: "true", description: "Boolean literal true" },
      { text: "false", description: "Boolean literal false" },
      { text: "while", description: "Loop while a condition is true" },
      { text: "func", description: "Defines a new function" },
      { text: "return", description: "Exit a function and return a value" },
    ];
    const vars = extractVariables(currentText);

    // Extensions processing
    const extensions = EXTENSION_CATEGORIES.flatMap((c) => c.extensions).map(
      (e) => ({
        text: e.ext,
        description: e.description,
      }),
    );

    const allOptions: SuggestionItem[] = [
      ...keywordDefs,
      ...Array.from(new Set(vars)).map((text) => ({ text })),
      ...extensions,
    ];

    const filtered = allOptions.filter(
      (opt) =>
        opt.text.toLowerCase().startsWith(currentWord.toLowerCase()) &&
        opt.text !== currentWord,
    );

    if (filtered.length > 0) {
      setSuggestions(filtered);
      setSelectedSuggestion(0);
      setShowSuggestions(true);
      setCurrentWordBounds({ start: startIndex, end: endIndex });

      const textarea = getTextArea();
      if (textarea) {
        const caret = getCaretCoordinates(textarea, selectionStart);
        setSuggestionPos({
          top: caret.top + caret.height,
          left: caret.left,
        });
      }
    } else {
      setShowSuggestions(false);
    }
  };

  const applySuggestion = (suggestion: SuggestionItem) => {
    const before = code.slice(0, currentWordBounds.start);
    const after = code.slice(currentWordBounds.end);
    const newCode = before + suggestion.text + after;
    setCode(newCode);
    setShowSuggestions(false);

    setTimeout(() => {
      const textarea = getTextArea();
      if (textarea) {
        textarea.focus();
        const nextCursorPos = currentWordBounds.start + suggestion.text.length;
        textarea.setSelectionRange(nextCursorPos, nextCursorPos);
      }
    }, 0);
  };

  const handleNextSearch = () => {
    if (searchResults.length > 0) {
      setSearchIndex((prev) => (prev + 1) % searchResults.length);
    }
  };

  const handlePrevSearch = () => {
    if (searchResults.length > 0) {
      setSearchIndex(
        (prev) => (prev - 1 + searchResults.length) % searchResults.length,
      );
    }
  };

  const handleReplace = () => {
    if (searchIndex === -1 || searchResults.length === 0) return;
    const result = searchResults[searchIndex];
    if (result.fileIndex !== activeFileIndex) {
      setActiveFileIndex(result.fileIndex);
      return;
    }
    
    setFiles((prev) =>
      prev.map((f, i) => {
        if (i === result.fileIndex) {
          const newContent =
            f.content.substring(0, result.start) +
            replaceQuery +
            f.content.substring(result.start + result.length);
          return { ...f, content: newContent };
        }
        return f;
      }),
    );
  };

  const handleReplaceAll = () => {
    if (!searchQuery) return;
    setFiles((prev) =>
      prev.map((f, i) => {
        if (i === activeFileIndex) {
          const escapedQuery = searchQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
          const regex = new RegExp(escapedQuery, 'gi');
          const newContent = f.content.replace(regex, replaceQuery);
          return { ...f, content: newContent };
        }
        return f;
      }),
    );
  };

  const lines = code.split("\n"); // Original lines for navigation

  const foldData = React.useMemo(() => {
      return getDisplayCode(code, foldedLines);
  }, [code, foldedLines]);
  const displayCode = foldData.displayCode;
  const visibleToOriginal = foldData.visibleToOriginal;
  const displayLines = displayCode.split("\n");

  const lintErrors = React.useMemo(() => lintLumino(displayCode, customLintRules), [displayCode, customLintRules]);

  const foldableRegions = React.useMemo(() => {
      return getFoldRegions(code);
  }, [code]);

  const toggleFold = (e: React.MouseEvent, lineNum: number) => {
      e.stopPropagation();
      setFoldedLines((prev) => {
          const next = new Set(prev);
          if (next.has(lineNum)) next.delete(lineNum);
          else next.add(lineNum);
          return next;
      });
  };

  const jumpToLine = (originalLineNum: number) => {
    const ta = getTextArea();
    if (!ta) return;

    let visibleIndex = -1;
    for (let i = 0; i < displayLines.length; i++) {
        if (visibleToOriginal[i] === originalLineNum) {
            visibleIndex = i;
            break;
        }
    }
    
    if (visibleIndex === -1) {
        for (let i = 0; i < displayLines.length; i++) {
           if (visibleToOriginal[i] > originalLineNum) {
               visibleIndex = Math.max(0, i - 1);
               break;
           }
        }
        if (visibleIndex === -1) visibleIndex = displayLines.length - 1;
    }

    let charIndex = 0;
    for (let i = 0; i < visibleIndex && i < displayLines.length; i++) {
        charIndex += displayLines[i].length + 1;
    }
    ta.focus();
    ta.setSelectionRange(charIndex, charIndex);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (showSuggestions) {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        setSelectedSuggestion((prev) => (prev + 1) % suggestions.length);
        return;
      }
      if (e.key === "ArrowUp") {
        e.preventDefault();
        setSelectedSuggestion(
          (prev) => (prev - 1 + suggestions.length) % suggestions.length,
        );
        return;
      }
      if (e.key === "Enter" || e.key === "Tab") {
        e.preventDefault();
        applySuggestion(suggestions[selectedSuggestion]);
        return;
      }
      if (e.key === "Escape") {
        e.preventDefault();
        setShowSuggestions(false);
        return;
      }
    }

    if ((e.ctrlKey || e.metaKey) && e.key === "f") {
      e.preventDefault();
      setIsSearchVisible(true);
      return;
    }
    if ((e.ctrlKey || e.metaKey) && e.key === "s") {
      e.preventDefault();
      // Files auto-save via useEffect, but force save here if auto-save is off
      if (!isAutoSaveEnabled) {
        localStorage.setItem("atos-files", JSON.stringify(files));
        // We could also show a toast, but this guarantees manual save works
      }
      return;
    }
    if (e.key === "Escape" && isSearchVisible) {
      setIsSearchVisible(false);
      return;
    }
    if (e.key === "Tab") {
      e.preventDefault();
      const target = e.target as HTMLTextAreaElement;
      const start = target.selectionStart;
      const end = target.selectionEnd;

      setCode(code.substring(0, start) + "  " + code.substring(end));

      setTimeout(() => {
        const textarea = getTextArea();
        if (textarea) {
          textarea.selectionStart = textarea.selectionEnd = start + 2;
        }
      }, 0);
    }
  };

  return (
    <div
      data-theme={theme}
      className="flex flex-col h-screen w-full bg-[#08080a] text-slate-400 font-sans overflow-hidden select-none relative"
      style={{
        backgroundImage:
          "url('https://images.unsplash.com/photo-1542484080-c0818d1f2b60?q=80&w=2670&auto=format&fit=crop')",
        backgroundSize: "cover",
        backgroundPosition: "center",
      }}
    >
      <style>{`
        [data-theme="Light Mode"] {
          background-color: #f8fafc !important;
          color: #334155 !important;
        }
        [data-theme="Light Mode"] .bg-\\[\\#08080a\\] { background-color: #f8fafc !important; }
        [data-theme="Light Mode"] .bg-\\[\\#0c0c0e\\] { background-color: #ffffff !important; }
        [data-theme="Light Mode"] .bg-\\[\\#1e1e24\\] { background-color: #ffffff !important; box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); }
        [data-theme="Light Mode"] .border-white\\/10, [data-theme="Light Mode"] .border-white\\/20, [data-theme="Light Mode"] .border-white\\/5 { border-color: #e2e8f0 !important; }
        [data-theme="Light Mode"] .text-slate-200, [data-theme="Light Mode"] .text-slate-300, [data-theme="Light Mode"] .text-white { color: #0f172a !important; }
        [data-theme="Light Mode"] .text-slate-400 { color: #475569 !important; }
        [data-theme="Light Mode"] .text-slate-500 { color: #64748b !important; }
        [data-theme="Light Mode"] .bg-white\\/5 { background-color: rgba(15, 23, 42, 0.05) !important; }
        [data-theme="Light Mode"] .hover\\:bg-white\\/5:hover { background-color: rgba(15, 23, 42, 0.08) !important; }
        [data-theme="Light Mode"] .hover\\:bg-white\\/10:hover { background-color: rgba(15, 23, 42, 0.12) !important; }
        [data-theme="Light Mode"] .bg-black\\/20, [data-theme="Light Mode"] .bg-black\\/40, [data-theme="Light Mode"] .bg-black\\/50, [data-theme="Light Mode"] .bg-black\\/80 { background-color: rgba(255, 255, 255, 0.8) !important; backdrop-filter: blur(12px) !important; color: #0f172a !important; }
        [data-theme="Light Mode"] .shadow-2xl { box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.15) !important; }

        [data-theme="High Contrast"] {
          background-color: #000000 !important;
          color: #ffffff !important;
        }
        [data-theme="High Contrast"] .bg-\\[\\#08080a\\], [data-theme="High Contrast"] .bg-\\[\\#0c0c0e\\], [data-theme="High Contrast"] .bg-\\[\\#1e1e24\\] { background-color: #000000 !important; }
        [data-theme="High Contrast"] .border-white\\/10, [data-theme="High Contrast"] .border-white\\/5 { border-color: #ffffff !important; }
        [data-theme="High Contrast"] .text-slate-200, [data-theme="High Contrast"] .text-slate-300, [data-theme="High Contrast"] .text-slate-400, [data-theme="High Contrast"] .text-slate-500, [data-theme="High Contrast"] .text-cyan-400 { color: #ffffff !important; }
        [data-theme="High Contrast"] .bg-white\\/5 { background-color: transparent !important; }
      `}</style>

      {/* Desktop Wrapper */}
      <div
        className="flex-1 relative overflow-hidden backdrop-blur-sm bg-black/50"
        onClick={() => isStartMenuOpen && setIsStartMenuOpen(false)}
      >
        {/* Desktop Icons */}
        <div className="absolute top-4 left-4 flex flex-col gap-6 z-0">
          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsWorkspaceMinimized(false);
              setIsChatOpen(false);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-white/10 flex items-center justify-center group-hover:bg-white/10 group-hover:border-white/30 backdrop-blur transition-all">
              <Code2 className="w-6 h-6 text-slate-200 group-hover:text-cyan-400" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-blue-600/80">
              ATOS IDE
            </span>
          </div>
          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsWorkspaceMinimized(false);
              closeAllPanels();
              setIsDictionaryOpen(true);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-white/10 flex items-center justify-center group-hover:bg-white/10 group-hover:border-white/30 backdrop-blur transition-all">
              <Book className="w-6 h-6 text-slate-200 group-hover:text-indigo-400" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-blue-600/80">
              Dictionary
            </span>
          </div>
          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsProjectManagerOpen(true);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-white/10 flex items-center justify-center group-hover:bg-white/10 group-hover:border-white/30 backdrop-blur transition-all">
              <FolderGit2 className="w-6 h-6 text-slate-200 group-hover:text-emerald-400" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-blue-600/80">
              Projekte
            </span>
          </div>
          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsWorkspaceMinimized(false);
              closeAllPanels();
              setIsPluginsOpen(true);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-white/10 flex items-center justify-center group-hover:bg-white/10 group-hover:border-white/30 backdrop-blur transition-all">
              <Plug className="w-6 h-6 text-slate-200 group-hover:text-fuchsia-400" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-blue-600/80">
              Plugin Store
            </span>
          </div>

          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsWorkspaceMinimized(false);
              closeAllPanels();
              setIsVersionControlOpen(true);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-white/10 flex items-center justify-center group-hover:bg-white/10 group-hover:border-white/30 backdrop-blur transition-all">
              <GitBranch className="w-6 h-6 text-slate-200 group-hover:text-emerald-400" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-blue-600/80">
              Version Control
            </span>
          </div>

          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsGitHubSyncOpen(true);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-indigo-500/30 flex items-center justify-center group-hover:bg-indigo-500/20 group-hover:border-indigo-400 backdrop-blur transition-all shadow-lg shadow-indigo-950/40">
              <Github className="w-6 h-6 text-indigo-300 group-hover:text-white" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-indigo-600/80">
              GitHub Sync
            </span>
          </div>

          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsWorkspaceMinimized(false);
              closeAllPanels();
              setIsTemplateDatabaseOpen(true);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-purple-500/30 flex items-center justify-center group-hover:bg-purple-500/20 group-hover:border-purple-400 backdrop-blur transition-all shadow-lg shadow-purple-950/40">
              <LayoutTemplate className="w-6 h-6 text-purple-300 group-hover:text-white" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-purple-600/80">
              Hardware Vorlagen
            </span>
          </div>

          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsWorkspaceMinimized(false);
              closeAllPanels();
              setIsTestingOpen(true);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-white/10 flex items-center justify-center group-hover:bg-white/10 group-hover:border-white/30 backdrop-blur transition-all">
              <Beaker className="w-6 h-6 text-slate-200 group-hover:text-indigo-400" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-blue-600/80">
              Unit Testing
            </span>
          </div>

          <div
            className="flex flex-col items-center gap-1 w-20 cursor-pointer group"
            onClick={() => {
              setIsWorkspaceMinimized(false);
              closeAllPanels();
              setIsCustomLinterOpen(true);
            }}
          >
            <div className="w-12 h-12 rounded-lg bg-black/40 border border-white/10 flex items-center justify-center group-hover:bg-white/10 group-hover:border-white/30 backdrop-blur transition-all">
              <ShieldAlert className="w-6 h-6 text-slate-200 group-hover:text-amber-400" />
            </div>
            <span className="text-[11px] text-white text-center font-medium drop-shadow-md bg-black/20 rounded px-1 group-hover:bg-amber-600/80">
              Custom Linter
            </span>
          </div>
        </div>

        {/* Workspace Window */}
        <div
          className={`absolute flex flex-col bg-[#08080a] border border-white/20 shadow-2xl overflow-hidden transition-all duration-300 ${isWorkspaceMinimized ? "opacity-0 pointer-events-none scale-95 translate-y-10" : "opacity-100 scale-100"}`}
          style={{
            inset: isWorkspaceMaximized ? "0px" : "40px 100px 60px 100px",
            borderRadius: isWorkspaceMaximized ? "0" : "0.5rem",
            zIndex: 10,
          }}
        >
          {/* Window Header */}
          <div className="h-8 bg-gradient-to-r from-blue-900/80 to-slate-900/80 backdrop-blur border-b border-white/10 flex items-center justify-between px-3 shrink-0 select-none">
            <div className="flex items-center gap-2 text-white text-xs font-bold font-sans tracking-wide">
              <Layers className="w-3.5 h-3.5 opacity-70" />
              ATOS Workspace {isProjectManagerOpen ? "- Projektmanager" : ""}
            </div>
            <div className="flex items-center gap-1">
              <button
                className="w-6 h-6 flex items-center justify-center hover:bg-white/20 text-white rounded transition-colors"
                onClick={() => setIsWorkspaceMinimized(true)}
              >
                <Minus className="w-3.5 h-3.5" />
              </button>
              <button
                className="w-6 h-6 flex items-center justify-center hover:bg-white/20 text-white rounded transition-colors"
                onClick={() => setIsWorkspaceMaximized(!isWorkspaceMaximized)}
              >
                <Maximize2 className="w-3.5 h-3.5" />
              </button>
              <button
                className="w-6 h-6 flex items-center justify-center hover:bg-red-500 text-white rounded transition-colors"
                onClick={() => setIsWorkspaceMinimized(true)}
              >
                <X className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>
          <div className="flex-1 flex overflow-hidden relative bg-[#08080a]">
            {/* Sidebar */}
            <div className="w-64 border-r border-white/5 bg-white/[0.02] backdrop-blur-lg flex flex-col shrink-0">
              <div className="h-14 flex items-center px-6 border-b border-white/10 bg-black/40 backdrop-blur-2xl text-lg font-semibold tracking-tight text-slate-100 shrink-0">
                <Code2 className="w-5 h-5 mr-3 text-cyan-400" />
                A-TownChain - Lumino IDE
              </div>
              <div className="p-4 flex-grow overflow-y-auto space-y-1">
                <div className="text-[10px] font-bold uppercase tracking-[0.2em] text-slate-500 mb-3 px-1 flex justify-between items-center">
                  Explorer
                  <div className="flex gap-2">
                    {fileHistory.length > 0 && (
                      <button
                        onClick={handleRollback}
                        className="text-orange-400 hover:text-orange-300 px-2 py-0.5 rounded bg-orange-500/10 hover:bg-orange-500/20 normal-case tracking-normal flex items-center gap-1"
                        title="Undo last mass change"
                      >
                        Rollback
                      </button>
                    )}
                    {!projectScaffolded && (
                      <>
                        <button
                          onClick={() => setIsScaffoldConfirmOpen(true)}
                          className="text-cyan-400 hover:text-cyan-300 px-2 py-0.5 rounded bg-cyan-500/10 hover:bg-cyan-500/20 normal-case tracking-normal"
                          title="Projektstruktur automatisch generieren"
                        >
                          Projekt einrichten
                        </button>
                        <button
                          onClick={() => setIsAutoArchitectOpen(true)}
                          className="text-purple-400 hover:text-purple-300 px-2 py-0.5 rounded bg-purple-500/10 hover:bg-purple-500/20 normal-case tracking-normal flex items-center gap-1"
                          title="Software per KI generieren"
                        >
                          <Wand2 className="w-3 h-3" />
                          Auto-Gen
                        </button>
                        <button
                          onClick={() => setIsGameScaffoldOpen(true)}
                          className="text-emerald-400 hover:text-emerald-300 px-2 py-0.5 rounded bg-emerald-500/10 hover:bg-emerald-500/20 normal-case tracking-normal flex items-center gap-1"
                          title="Spiel-Gerüst generieren"
                        >
                          <Gamepad2 className="w-3 h-3" />
                          Games
                        </button>
                      </>
                    )}
                  </div>
                </div>
                
                <div className="mb-4 px-1">
                  <div className="relative">
                    <Search className="w-3 h-3 absolute left-2 top-1/2 -translate-y-1/2 text-slate-500" />
                    <input
                      ref={fileSearchInputRef}
                      type="text"
                      placeholder="Search files or content..."
                      value={fileSearchQuery}
                      onChange={(e) => setFileSearchQuery(e.target.value)}
                      className="w-full bg-black/40 border border-white/10 rounded-md py-1 pl-7 pr-2 text-xs text-slate-200 outline-none focus:border-indigo-500/50"
                    />
                  </div>
                </div>
                
                {files.map((file, originalIndex) => {
                  const query = fileSearchQuery.toLowerCase();
                  const matchName = file.name.toLowerCase().includes(query);
                  const matchContentIndex = file.content.toLowerCase().indexOf(query);
                  const matchContent = matchContentIndex !== -1;
                  
                  let previewSnippet = "";
                  if (!matchName && matchContent && query.length > 0) {
                    const start = Math.max(0, matchContentIndex - 10);
                    const end = Math.min(file.content.length, matchContentIndex + query.length + 10);
                    previewSnippet = file.content.substring(start, end).replace(/\n/g, " ");
                  }
                  
                  return { file, originalIndex, matchName, matchContent, previewSnippet };
                })
                  .filter(({ matchName, matchContent }) => fileSearchQuery === "" || matchName || matchContent)
                  .map(({ file, originalIndex: index, previewSnippet }) => (
                  <div
                    key={index}
                    draggable
                    onDragStart={() => handleDragStart(index)}
                    onDragEnter={() => handleDragEnter(index)}
                    onDragEnd={handleDragEnd}
                    onDragOver={(e) => e.preventDefault()}
                    onDrop={(e) => {
                      e.preventDefault();
                      handleDrop(index);
                    }}
                    onClick={() => {
                      playUISound('select');
                      setActiveFileIndex(index);
                      closeAllPanels();
                    }}
                    className={`flex flex-col gap-1 px-3 py-2 rounded-md cursor-pointer transition-all ${
                      index === activeFileIndex
                        ? "bg-white/5 text-cyan-400"
                        : "text-slate-400 hover:bg-white/5 hover:text-slate-300"
                    } ${dragOverFileIndex === index ? "border-t-2 border-cyan-500" : ""}`}
                  >
                    <div className="flex items-center gap-2 text-sm">
                      <span className={file.iconColor}>{file.iconShape}</span>
                      <span>{file.name}</span>
                    </div>
                    {previewSnippet && (
                      <div className="text-[10px] text-slate-500 truncate pl-6">
                        <span className="font-mono bg-indigo-500/10 text-indigo-300 px-1 rounded">...{previewSnippet}...</span>
                      </div>
                    )}
                  </div>
                ))}
                <div
                  className="flex items-center gap-2 px-3 py-2 mt-2 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsDictionaryOpen(true);
                  }}
                >
                  <span className="text-indigo-400 font-bold opacity-80">
                    <Book className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">ATOS Dictionary</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsLibraryOpen(true);
                  }}
                >
                  <span className="text-cyan-400 font-bold opacity-80">
                    <Blocks className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Module Library</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsRegistryOpen(true);
                  }}
                >
                  <span className="text-emerald-400 font-bold opacity-80">
                    <Code2 className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Standard Registry</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsSnippetsOpen(true);
                  }}
                >
                  <span className="text-amber-400 font-bold opacity-80">
                    <Scissors className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Code Snippets</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsAssetsOpen(true);
                  }}
                >
                  <span className="text-pink-400 font-bold opacity-80">
                    <Camera className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Assets</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsWikiOpen(true);
                  }}
                >
                  <span className="text-sky-400 font-bold opacity-80">
                    <Globe className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">KAI-OS Master Wiki</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsWorkflowOpen(true);
                  }}
                >
                  <span className="text-indigo-500 font-bold opacity-80">
                    <Network className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Workflows</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsDataExplorerOpen(true);
                  }}
                >
                  <span className="text-emerald-500 font-bold opacity-80">
                    <Database className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Daten Explorer</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsChatbotArchiveOpen(true);
                  }}
                >
                  <span className="text-blue-500 font-bold opacity-80">
                    <MessageSquare className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Chat Archiv</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsTemplateDatabaseOpen(true);
                  }}
                >
                  <span className="text-purple-500 font-bold opacity-80">
                    <LayoutTemplate className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Template DB</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsScriptDatabaseOpen(true);
                  }}
                >
                  <span className="text-orange-500 font-bold opacity-80">
                    <FileCode className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Skript DB</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsAudioEngineOpen(true);
                  }}
                >
                  <span className="text-violet-500 font-bold opacity-80">
                    <Volume2 className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Audio Engine</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsPluginsOpen(true);
                  }}
                >
                  <span className="text-fuchsia-500 font-bold opacity-80">
                    <Plug className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Plugins</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsVersionControlOpen(true);
                  }}
                >
                  <span className="text-emerald-500 font-bold opacity-80">
                    <GitBranch className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Version Control</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    closeAllPanels();
                    setIsTestingOpen(true);
                  }}
                >
                  <span className="text-indigo-500 font-bold opacity-80">
                    <Beaker className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Unit Testing</span>
                </div>
                <div
                  className="flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors hover:bg-white/5 text-slate-400"
                  onClick={() => {
                    setIsCiCdGeneratorOpen(true);
                  }}
                >
                  <span className="text-amber-400 font-bold opacity-80">
                    <Zap className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">CI/CD Generator</span>
                </div>
                <div
                  className={`flex items-center justify-between px-3 py-1 rounded-md text-sm cursor-pointer transition-colors ${
                    isCustomLinterOpen ? "bg-amber-500/20 text-amber-300 font-bold" : "hover:bg-white/5 text-slate-400"
                  }`}
                  onClick={() => {
                    closeAllPanels();
                    setIsCustomLinterOpen(true);
                  }}
                >
                  <div className="flex items-center gap-2 truncate">
                    <span className="text-amber-400 font-bold opacity-80">
                      <ShieldAlert className="w-3.5 h-3.5" />
                    </span>
                    <span className="truncate">Custom Linter</span>
                  </div>
                  {lintErrors.length > 0 && (
                    <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-red-500/20 text-red-300 font-mono">
                      {lintErrors.length}
                    </span>
                  )}
                </div>
                <div
                  className={`flex items-center justify-between px-3 py-1 rounded-md text-sm cursor-pointer transition-colors ${
                    isAuditOpen ? "bg-cyan-500/20 text-cyan-300 font-bold" : "hover:bg-white/5 text-slate-400"
                  }`}
                  onClick={() => {
                    closeAllPanels();
                    setIsAuditOpen(true);
                  }}
                >
                  <div className="flex items-center gap-2 truncate">
                    <span className="text-cyan-400 font-bold opacity-80">
                      <ShieldCheck className="w-3.5 h-3.5" />
                    </span>
                    <span className="truncate">Audit & Security</span>
                  </div>
                  <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-cyan-500/20 text-cyan-300 font-mono font-bold">
                    {liveAuditSummary.grade}
                  </span>
                </div>
                <div
                  className={`flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors ${
                    isAtcVmOpen ? "bg-cyan-500/20 text-cyan-300 font-bold" : "hover:bg-white/5 text-slate-400"
                  }`}
                  onClick={() => {
                    closeAllPanels();
                    setIsAtcVmOpen(true);
                  }}
                >
                  <span className="text-cyan-400 font-bold opacity-80">
                    <Terminal className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">ATC-VM Simulator</span>
                </div>
                <div
                  className={`flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors ${
                    isAtcComplianceOpen ? "bg-emerald-500/20 text-emerald-300 font-bold" : "hover:bg-white/5 text-slate-400"
                  }`}
                  onClick={() => {
                    closeAllPanels();
                    setIsAtcComplianceOpen(true);
                  }}
                >
                  <span className="text-emerald-400 font-bold opacity-80">
                    <ShieldCheck className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">ATC-DOC Compliance</span>
                </div>
                <div
                  className={`flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors ${
                    isGenesisOpen ? "bg-indigo-500/20 text-indigo-300 font-bold" : "hover:bg-white/5 text-slate-400"
                  }`}
                  onClick={() => {
                    closeAllPanels();
                    setIsGenesisOpen(true);
                  }}
                >
                  <span className="text-indigo-400 font-bold opacity-80">
                    <Layers className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Genesis & Devnet</span>
                </div>
                <div
                  className={`flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors ${
                    isRustWorkspaceOpen ? "bg-orange-500/20 text-orange-300 font-bold" : "hover:bg-white/5 text-slate-400"
                  }`}
                  onClick={() => {
                    closeAllPanels();
                    setIsRustWorkspaceOpen(true);
                  }}
                >
                  <span className="text-orange-400 font-bold opacity-80">
                    <Boxes className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Rust Crates Workspace</span>
                </div>
                <div
                  className={`flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors ${
                    isLanguageStrategyOpen ? "bg-indigo-500/20 text-indigo-300 font-bold" : "hover:bg-white/5 text-slate-400"
                  }`}
                  onClick={() => {
                    closeAllPanels();
                    setIsLanguageStrategyOpen(true);
                  }}
                >
                  <span className="text-indigo-400 font-bold opacity-80">
                    <Code2 className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Sprachstrategie & Matrix</span>
                </div>
                <div
                  className={`flex items-center gap-2 px-3 py-1 rounded-md text-sm cursor-pointer transition-colors ${
                    isGlobusFormatsOpen ? "bg-cyan-500/20 text-cyan-300 font-bold" : "hover:bg-white/5 text-slate-400"
                  }`}
                  onClick={() => {
                    closeAllPanels();
                    setIsGlobusFormatsOpen(true);
                  }}
                >
                  <span className="text-cyan-400 font-bold opacity-80">
                    <Binary className="w-3.5 h-3.5" />
                  </span>
                  <span className="truncate">Dateiformate (.g* GFFA)</span>
                </div>
              </div>
              <div className="p-4 border-t border-white/10 text-xs text-slate-500">
                <div className="font-bold text-slate-300">ATOS IDE v1.2.0</div>
                <div className="opacity-70 mt-0.5">
                  Developer: A-TownChain-Ökosystems
                </div>
                <div className="mt-2 text-[10px] uppercase tracking-wider text-slate-600">
                  Core Runtime
                </div>
                <div className="opacity-70">Lumino Language v1.0</div>
                <div className="mt-4 flex items-center justify-between pointer-events-auto">
                  <span className="text-slate-400">Light Mode</span>
                  <label className="relative inline-flex items-center cursor-pointer">
                    <input 
                      type="checkbox" 
                      className="sr-only peer" 
                      checked={theme === "Light Mode"}
                      onChange={(e) => setTheme(e.target.checked ? "Light Mode" : "ATOS Dark")}
                    />
                    <div className="w-7 h-4 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-300 after:border-gray-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-amber-400/80"></div>
                  </label>
                </div>
                
                <div className="mt-2 flex items-center justify-between pointer-events-auto">
                  <span className="text-slate-400">Auto-Save</span>
                  <label className="relative inline-flex items-center cursor-pointer">
                    <input 
                      type="checkbox" 
                      className="sr-only peer" 
                      checked={isAutoSaveEnabled}
                      onChange={(e) => setIsAutoSaveEnabled(e.target.checked)}
                    />
                    <div className="w-7 h-4 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-300 after:border-gray-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-emerald-500/80"></div>
                  </label>
                </div>
                
                <div className="mt-2 flex items-center justify-between pointer-events-auto">
                  <span className="text-slate-400">Advanced Syntax</span>
                  <label className="relative inline-flex items-center cursor-pointer">
                    <input 
                      type="checkbox" 
                      className="sr-only peer" 
                      checked={advancedSyntaxHighlighting}
                      onChange={(e) => setAdvancedSyntaxHighlighting(e.target.checked)}
                    />
                    <div className="w-7 h-4 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-slate-300 after:border-gray-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-blue-500/80"></div>
                  </label>
                </div>
              </div>
            </div>

            {/* Main Content */}
            <div className="flex-1 flex flex-col min-w-0 bg-[#0c0c0e]">
              {/* Topbar */}
              <div className="h-14 border-b border-white/5 bg-black/40 backdrop-blur-2xl flex items-center justify-between px-6 shrink-0">
                <div className="h-full flex items-center -ml-6">
                  {files.map((file, index) => (
                    <div
                      key={index}
                      onClick={() => {
                        playUISound('select');
                        setActiveFileIndex(index);
                        closeAllPanels();
                      }}
                      className={`group px-6 h-full flex items-center gap-2 border-r border-white/5 cursor-pointer text-xs transition-colors ${
                        index === activeFileIndex
                          ? "bg-[#0c0c0e] text-cyan-400"
                          : "hover:bg-white/5 text-slate-500"
                      }`}
                    >
                      {file.name}
                      <span 
                         onClick={(e) => {
                             e.stopPropagation();
                             setSplitActiveFileIndex(index);
                         }}
                         className="opacity-0 group-hover:opacity-100 hover:text-white transition-opacity ml-1 z-10"
                         title="Split Right"
                      >
                          <SplitSquareVertical className="w-3 h-3" />
                      </span>
                    </div>
                  ))}
                </div>

                <div className="flex items-center gap-3">
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => {
                      closeAllPanels();
                    }}
                    className={`flex items-center px-3 py-1.5 transition-all text-xs font-bold rounded ${!isDictionaryOpen && !isLibraryOpen && !isRegistryOpen && !isSnippetsOpen && !isAssetsOpen && !isWikiOpen && !isWorkflowOpen && !isDataExplorerOpen && !isChatbotArchiveOpen && !isTemplateDatabaseOpen && !isScriptDatabaseOpen && !isAudioEngineOpen && !isPluginsOpen && !isVersionControlOpen && !isTestingOpen && !isCustomLinterOpen && !isAuditOpen ? "bg-white/20 text-white" : "hover:bg-white/10 text-slate-300"}`}
                  >
                    <Code2 className="w-4 h-4 mr-1.5" />
                    EDITOR
                  </motion.button>
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => {
                      closeAllPanels();
                      setIsLibraryOpen(true);
                    }}
                    className={`flex items-center px-3 py-1.5 transition-all text-xs font-bold rounded ${isLibraryOpen ? "bg-white/20 text-white" : "hover:bg-white/10 text-slate-300"}`}
                  >
                    <Blocks className="w-4 h-4 mr-1.5" />
                    MODULES
                  </motion.button>
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => {
                      closeAllPanels();
                      setIsDictionaryOpen(true);
                    }}
                    className={`flex items-center px-3 py-1.5 transition-all text-xs font-bold rounded ${isDictionaryOpen ? "bg-white/20 text-white" : "hover:bg-white/10 text-slate-300"}`}
                  >
                    <Book className="w-4 h-4 mr-1.5" />
                    DICTIONARY
                  </motion.button>
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => {
                      closeAllPanels();
                      setIsAssetsOpen(true);
                    }}
                    className={`flex items-center px-3 py-1.5 transition-all text-xs font-bold rounded ${isAssetsOpen ? "bg-white/20 text-white" : "hover:bg-white/10 text-slate-300"}`}
                  >
                    <Camera className="w-4 h-4 mr-1.5" />
                    ASSETS
                  </motion.button>
                  <div className="w-px h-4 bg-white/10 mx-1"></div>
                  <motion.button
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => setIsSearchVisible(!isSearchVisible)}
                    className={`flex items-center px-3 py-1.5 transition-all text-xs font-bold rounded ${isSearchVisible ? "bg-white/20 text-white" : "hover:bg-white/10 text-slate-300"}`}
                    title="Find (Ctrl+F)"
                  >
                    <Search className="w-4 h-4" />
                  </motion.button>
                  <div className="w-px h-4 bg-white/10 mx-1"></div>
                  {!debugState ? (
                    <>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => setIsProjectManagerOpen(true)}
                        className="flex items-center px-4 py-1.5 hover:bg-white/10 text-indigo-300 text-xs font-bold rounded transition-all"
                        title="Software Projekte & Fusion"
                      >
                        <FolderGit2 className="w-4 h-4 mr-1.5" />
                        PROJECTS
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => setIsGitHubSyncOpen(true)}
                        className="flex items-center px-3.5 py-1.5 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 text-xs font-bold rounded transition-all shadow-xs"
                        title="GitHub Repository anlegen, pushen & synchronisieren"
                      >
                        <Github className="w-4 h-4 mr-1.5 text-white" />
                        GITHUB
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => setIsCiCdGeneratorOpen(true)}
                        className="flex items-center px-3.5 py-1.5 bg-amber-500/10 hover:bg-amber-500/20 text-amber-300 border border-amber-500/30 text-xs font-bold rounded transition-all shadow-xs"
                        title="CI/CD Workflow (.github/workflows/lumino-build.yml) für automatische Tests generieren"
                      >
                        <Zap className="w-4 h-4 mr-1.5 text-amber-400" />
                        CI/CD
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => {
                          closeAllPanels();
                          setIsCustomLinterOpen(true);
                        }}
                        className={`flex items-center px-3.5 py-1.5 transition-all text-xs font-bold rounded shadow-xs ${
                          isCustomLinterOpen
                            ? "bg-amber-500/30 text-amber-200 border border-amber-500/50"
                            : "bg-amber-500/10 hover:bg-amber-500/20 text-amber-300 border border-amber-500/30"
                        }`}
                        title="Benutzerdefinierte Lumino Linting-Regeln & Code-Inspektor"
                      >
                        <ShieldAlert className="w-4 h-4 mr-1.5 text-amber-400" />
                        LINTER
                        {lintErrors.length > 0 && (
                          <span className="ml-1.5 px-1.5 py-0.2 rounded-full text-[10px] bg-red-500/30 text-red-300 border border-red-500/40 font-mono">
                            {lintErrors.length}
                          </span>
                        )}
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => {
                          closeAllPanels();
                          setIsAuditOpen(true);
                        }}
                        className={`flex items-center px-3.5 py-1.5 transition-all text-xs font-bold rounded shadow-xs ${
                          isAuditOpen
                            ? "bg-cyan-500/30 text-cyan-200 border border-cyan-500/50"
                            : "bg-cyan-500/10 hover:bg-cyan-500/20 text-cyan-300 border border-cyan-500/30"
                        }`}
                        title="Security-, Vollständigkeits-, Funktions- & Verknüpft-Audit mit automatischer Verbesserung"
                      >
                        <ShieldCheck className="w-4 h-4 mr-1.5 text-cyan-400" />
                        AUDIT
                        <span className="ml-1.5 px-1.5 py-0.2 rounded-full text-[10px] bg-cyan-500/30 text-cyan-300 border border-cyan-500/40 font-mono font-bold">
                          {liveAuditSummary.grade} • {liveAuditSummary.healthScore}%
                        </span>
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => {
                          closeAllPanels();
                          setIsAtcVmOpen(true);
                        }}
                        className={`flex items-center px-3.5 py-1.5 transition-all text-xs font-bold rounded shadow-xs ${
                          isAtcVmOpen
                            ? "bg-cyan-500/30 text-cyan-200 border border-cyan-500/50"
                            : "bg-cyan-500/10 hover:bg-cyan-500/20 text-cyan-300 border border-cyan-500/30"
                        }`}
                        title="ATC-VM Register-basierter Simulator & Gas-Debugger"
                      >
                        <Terminal className="w-4 h-4 mr-1.5 text-cyan-400" />
                        ATC-VM
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => {
                          closeAllPanels();
                          setIsAtcComplianceOpen(true);
                        }}
                        className={`flex items-center px-3.5 py-1.5 transition-all text-xs font-bold rounded shadow-xs ${
                          isAtcComplianceOpen
                            ? "bg-emerald-500/30 text-emerald-200 border border-emerald-500/50"
                            : "bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-300 border border-emerald-500/30"
                        }`}
                        title="ATC-DOC Repository- und Normen-Compliance Prüfer"
                      >
                        <ShieldCheck className="w-4 h-4 mr-1.5 text-emerald-400" />
                        COMPLIANCE
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => {
                          closeAllPanels();
                          setIsGenesisOpen(true);
                        }}
                        className={`flex items-center px-3.5 py-1.5 transition-all text-xs font-bold rounded shadow-xs ${
                          isGenesisOpen
                            ? "bg-indigo-500/30 text-indigo-200 border border-indigo-500/50"
                            : "bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 border border-indigo-500/30"
                        }`}
                        title="Genesis Block & Devnet Node Konfigurator"
                      >
                        <Server className="w-4 h-4 mr-1.5 text-indigo-400" />
                        GENESIS
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => {
                          closeAllPanels();
                          setIsLanguageStrategyOpen(true);
                        }}
                        className={`flex items-center px-3.5 py-1.5 transition-all text-xs font-bold rounded shadow-xs ${
                          isLanguageStrategyOpen
                            ? "bg-indigo-500/30 text-indigo-200 border border-indigo-500/50"
                            : "bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 border border-indigo-500/30"
                        }`}
                        title="Ökosystem-Sprachstrategie & Systemebenen-Matrix (ATC-DOC-012)"
                      >
                        <Code2 className="w-4 h-4 mr-1.5 text-indigo-400" />
                        SPRACHEN
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => {
                          closeAllPanels();
                          setIsGlobusFormatsOpen(true);
                        }}
                        className={`flex items-center px-3.5 py-1.5 transition-all text-xs font-bold rounded shadow-xs ${
                          isGlobusFormatsOpen
                            ? "bg-cyan-500/30 text-cyan-200 border border-cyan-500/50"
                            : "bg-cyan-500/10 hover:bg-cyan-500/20 text-cyan-300 border border-cyan-500/30"
                        }`}
                        title="Globus File Format Architecture (GFFA & GNFF v1.0 / ATC-DOC-013)"
                      >
                        <Binary className="w-4 h-4 mr-1.5 text-cyan-400" />
                        FORMATE
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => {
                          closeAllPanels();
                          setIsTemplateDatabaseOpen(true);
                        }}
                        className="flex items-center px-3.5 py-1.5 bg-purple-500/10 hover:bg-purple-500/20 text-purple-300 border border-purple-500/30 text-xs font-bold rounded transition-all shadow-xs"
                        title="Hardware- & Architektur-Vorlagen (x86, ARM, RISC-V, Monolith, Schichten)"
                      >
                        <LayoutTemplate className="w-4 h-4 mr-1.5 text-purple-300" />
                        VORLAGEN
                      </motion.button>
                      <div className="w-px h-4 bg-white/10 mx-1"></div>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={handleSnapshot}
                        className="flex items-center px-4 py-1.5 hover:bg-white/10 text-slate-300 text-xs font-bold rounded transition-all"
                        title="Download Project Snapshot"
                      >
                        <Save className="w-4 h-4 mr-1.5" />
                        SNAPSHOT
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={handleFormatCode}
                        className="flex items-center px-4 py-1.5 hover:bg-white/10 text-slate-300 text-xs font-bold rounded transition-all"
                        title="Auto-Format Code"
                      >
                        <Wand2 className="w-4 h-4 mr-1.5" />
                        FORMAT
                      </motion.button>
                      <div className="flex items-center px-2 py-1 hover:bg-white/10 rounded transition-all">
                         <Palette className="w-4 h-4 text-slate-300 mr-1.5" />
                         <select
                           value={editorTheme}
                           onChange={(e) => setEditorTheme(e.target.value)}
                           className="bg-transparent text-slate-300 text-xs font-bold outline-none cursor-pointer appearance-none"
                           title="Editor Theme"
                         >
                           <option value="prism-tomorrow">Tomorrow</option>
                           <option value="prism-okaidia">Okaidia</option>
                           <option value="prism-twilight">Twilight</option>
                           <option value="prism-dark">Dark</option>
                           <option value="prism-funky">Funky</option>
                           <option value="prism-coy">Coy</option>
                           <option value="prism-solarizedlight">Solarized Light</option>
                           <option value="prism">Standard</option>
                         </select>
                      </div>
                      <div className="w-px h-4 bg-white/10 mx-1"></div>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={() => setIsChatOpen(!isChatOpen)}
                        className={`flex items-center px-4 py-1.5 transition-all text-xs font-bold rounded ${isChatOpen ? "bg-purple-500/20 text-purple-400" : "hover:bg-white/10 text-slate-300"}`}
                        title="Toggle AI Assistant"
                      >
                        <Bot className="w-4 h-4 mr-1.5" />
                        AI
                      </motion.button>
                      <div className="w-px h-4 bg-white/10 mx-1"></div>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={handleStartDebug}
                        disabled={isRunning}
                        className="flex items-center px-4 py-1.5 hover:bg-white/10 text-slate-300 text-xs font-bold rounded transition-all disabled:opacity-50"
                        title="Start Debugging"
                      >
                        <Bug className="w-4 h-4 mr-1.5" />
                        DEBUG
                      </motion.button>
                      <motion.button
                        whileHover={{ scale: 1.02 }}
                        whileTap={{ scale: 0.98 }}
                        onClick={runCode}
                        disabled={isRunning}
                        className="flex items-center px-4 py-1.5 bg-cyan-500 text-white text-xs font-bold rounded shadow-lg shadow-cyan-500/20 transition-all disabled:opacity-50"
                      >
                        <Play className="w-4 h-4 mr-1.5 fill-current" />
                        {isRunning ? "RUNNING..." : "RUN"}
                      </motion.button>
                    </>
                  ) : (
                    <div className="flex items-center bg-white/5 p-1 rounded-md border border-white/10 shadow-lg">
                      <button
                        onClick={continueDebug}
                        className="p-1.5 hover:bg-white/10 rounded text-cyan-400 transition-colors"
                        title="Continue"
                      >
                        <Play className="w-4 h-4 fill-current" />
                      </button>
                      <button
                        onClick={stepDebug}
                        className="p-1.5 hover:bg-white/10 rounded text-slate-300 transition-colors"
                        title="Step Over"
                      >
                        <StepForward className="w-4 h-4" />
                      </button>
                      <div className="w-px h-4 bg-white/10 mx-1"></div>
                      <button
                        onClick={stopDebug}
                        className="p-1.5 hover:bg-red-500/20 rounded text-red-400 transition-colors"
                        title="Stop Debugging"
                      >
                        <Square className="w-4 h-4 fill-current" />
                      </button>
                    </div>
                  )}
                </div>
              </div>

              <div className="flex-1 flex overflow-hidden">
                {isDictionaryOpen ? (
                  <DictionaryPanel />
                ) : isLibraryOpen ? (
                  <ModuleLibrary
                    onInsertCode={insertCodeStr}
                    onCreateFile={(name, content) => {
                      setFileHistory((prev) => [...prev, JSON.parse(JSON.stringify(files))]);
                      const newFile: FileState = {
                        name,
                        content,
                        iconColor: "text-cyan-400",
                        iconShape: "◈"
                      };
                      setFiles((prev) => [...prev, newFile]);
                      setActiveFileIndex(files.length);
                      setIsLibraryOpen(false);
                      showEditorToast(`Datei '${name}' aus Modul-Bibliothek erstellt.`, 'info');
                    }}
                  />
                ) : isRegistryOpen ? (
                  <RegistryPanel />
                ) : isSnippetsOpen ? (
                  <SnippetsPanel onInsertCode={insertCodeStr} />
                ) : isAssetsOpen ? (
                  <AssetsPanel />
                ) : isWikiOpen ? (
                  <ArchitectureWiki
                    files={files}
                    onJumpToFileAndLine={(fileName, line) => {
                      closeAllPanels();
                      const idx = files.findIndex((f) => f.name === fileName);
                      if (idx >= 0) setActiveFileIndex(idx);
                      jumpToLine(line);
                    }}
                    onSaveWorkspaceFile={handleSaveLintConfigFile}
                    onShowToast={showEditorToast}
                  />
                ) : isWorkflowOpen ? (
                  <WorkflowPanel
                    files={files}
                    onOpenCiCdGenerator={() => setIsCiCdGeneratorOpen(true)}
                    onOpenEditorFile={(filename) => {
                      const idx = files.findIndex((f) => f.name === filename);
                      if (idx >= 0) {
                        setActiveFileIndex(idx);
                        closeAllPanels();
                      }
                    }}
                  />
                ) : isCustomLinterOpen ? (
                  <CustomLinterPanel
                    rules={customLintRules}
                    onUpdateRules={setCustomLintRules}
                    currentFile={files[activeFileIndex]}
                    allFiles={files}
                    onSaveWorkspaceFile={handleSaveLintConfigFile}
                    onJumpToLine={(line) => {
                      closeAllPanels();
                      jumpToLine(line);
                    }}
                    onApplyQuickFix={applyQuickFix}
                    currentDiagnostics={lintErrors}
                  />
                ) : isAuditOpen ? (
                  <AuditPanel
                    files={files}
                    setFiles={setFiles}
                    onJumpToFileAndLine={(fileName, line) => {
                      closeAllPanels();
                      const idx = files.findIndex((f) => f.name === fileName);
                      if (idx >= 0) setActiveFileIndex(idx);
                      jumpToLine(line);
                    }}
                    onShowToast={showEditorToast}
                    autoSyncTodo={autoSyncTodo}
                    setAutoSyncTodo={setAutoSyncTodo}
                    autoSyncWiki={autoSyncWiki}
                    setAutoSyncWiki={setAutoSyncWiki}
                  />
                ) : isAtcVmOpen ? (
                  <AtcVmSimulatorPanel
                    onSaveWorkspaceFile={handleSaveLintConfigFile}
                    onShowToast={showEditorToast}
                  />
                ) : isAtcComplianceOpen ? (
                  <AtcDocCompliancePanel
                    files={files}
                    onSaveWorkspaceFile={handleSaveLintConfigFile}
                    onShowToast={showEditorToast}
                  />
                ) : isGenesisOpen ? (
                  <GenesisConfiguratorPanel
                    onSaveWorkspaceFile={handleSaveLintConfigFile}
                    onShowToast={showEditorToast}
                  />
                ) : isRustWorkspaceOpen ? (
                  <RustWorkspaceGeneratorPanel
                    onSaveWorkspaceFile={handleSaveLintConfigFile}
                    onShowToast={showEditorToast}
                  />
                ) : isLanguageStrategyOpen ? (
                  <LanguageStrategyPanel
                    onSaveWorkspaceFile={handleSaveLintConfigFile}
                    onShowToast={showEditorToast}
                  />
                ) : isGlobusFormatsOpen ? (
                  <GlobusFileFormatsPanel
                    onSaveWorkspaceFile={handleSaveLintConfigFile}
                    onShowToast={showEditorToast}
                  />
                ) : isDataExplorerOpen ? (
                  <DataExplorerPanel />
                ) : isChatbotArchiveOpen ? (
                  <ChatbotArchivePanel />
                ) : isTemplateDatabaseOpen ? (
                  <TemplateDatabasePanel
                    onLoadFiles={(newFiles) => {
                      setFileHistory((prev) => [...prev, JSON.parse(JSON.stringify(files))]);
                      const mappedFiles: FileState[] = newFiles.map((f) => ({
                        name: f.name,
                        content: f.content,
                        iconColor: "text-indigo-400",
                        iconShape: "⬢"
                      }));
                      setFiles(mappedFiles);
                      setActiveFileIndex(0);
                      setIsTemplateDatabaseOpen(false);
                      showEditorToast(`Architektur-Vorlage geladen (${mappedFiles.length} Dateien).`, 'info');
                    }}
                    onInsertCode={insertCodeStr}
                    onClose={() => setIsTemplateDatabaseOpen(false)}
                    onOpenWiki={() => {
                      setIsTemplateDatabaseOpen(false);
                      setIsWikiOpen(true);
                    }}
                  />
                ) : isScriptDatabaseOpen ? (
                  <ScriptDatabasePanel />
                ) : isAudioEngineOpen ? (
                  <AudioEnginePanel />
                ) : isPluginsOpen ? (
                  <PluginsPanel />
                ) : isVersionControlOpen ? (
                  <VersionControlPanel files={files} setFiles={setFiles} />
                ) : isTestingOpen ? (
                  <TestingPanel
                    files={files}
                    onOpenCiCdGenerator={() => setIsCiCdGeneratorOpen(true)}
                  />
                ) : (
                  <div className="flex-1 flex flex-col overflow-hidden relative">
                    {isSearchVisible && (
                      <div className="flex flex-col bg-black/60 backdrop-blur-xl border-b border-white/10 shrink-0 z-10 shadow-lg">
                        <div className="flex items-center px-4 py-2 gap-2">
                          <Search className="w-4 h-4 text-slate-400" />
                          <input
                            autoFocus
                            type="text"
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            placeholder="Find in file..."
                            className="bg-transparent border-none outline-none text-sm text-slate-200 flex-1 px-2"
                            onKeyDown={(e) => {
                              if (e.key === "Enter") {
                                e.preventDefault();
                                if (e.shiftKey) handlePrevSearch();
                                else handleNextSearch();
                              }
                              if (e.key === "Escape") {
                                setIsSearchVisible(false);
                                getTextArea()?.focus();
                              }
                            }}
                          />
                          {searchQuery && (
                            <span className="text-xs text-slate-500 font-mono select-none">
                              {searchResults.length > 0
                                ? `${searchIndex + 1} of ${searchResults.length}`
                                : "No results"}
                            </span>
                          )}
                          <div className="flex items-center bg-white/5 rounded mx-2 border border-white/10 hover:border-white/20 transition-colors">
                            <button
                              onClick={handlePrevSearch}
                              disabled={searchResults.length === 0}
                              className="p-1.5 hover:bg-white/10 text-slate-300 disabled:opacity-30 transition-colors rounded-l"
                              title="Previous match (Shift+Enter)"
                            >
                              <ArrowUp className="w-3.5 h-3.5" />
                            </button>
                            <button
                              onClick={handleNextSearch}
                              disabled={searchResults.length === 0}
                              className="p-1.5 hover:bg-white/10 text-slate-300 disabled:opacity-30 transition-colors border-l border-white/5 rounded-r"
                              title="Next match (Enter)"
                            >
                              <ArrowDown className="w-3.5 h-3.5" />
                            </button>
                          </div>
                          <button
                            onClick={() => {
                              setIsSearchVisible(false);
                              getTextArea()?.focus();
                            }}
                            className="p-1.5 hover:text-pink-400 hover:bg-white/5 rounded-md text-slate-400 transition-colors"
                            title="Close (Escape)"
                          >
                            <X className="w-4 h-4" />
                          </button>
                        </div>
                        <div className="flex items-center px-4 py-1.5 gap-2 border-t border-white/5">
                          <div className="w-4 h-4" /> {/* Spacer */}
                          <input
                            type="text"
                            value={replaceQuery}
                            onChange={(e) => setReplaceQuery(e.target.value)}
                            placeholder="Replace with..."
                            className="bg-transparent border-none outline-none text-sm text-slate-200 flex-1 px-2"
                            onKeyDown={(e) => {
                              if (e.key === "Enter") {
                                e.preventDefault();
                                if (e.shiftKey) handleReplaceAll();
                                else handleReplace();
                              }
                              if (e.key === "Escape") {
                                setIsSearchVisible(false);
                                getTextArea()?.focus();
                              }
                            }}
                          />
                          <div className="flex items-center gap-1 shrink-0 mr-[34px]">
                            <button
                              onClick={handleReplace}
                              disabled={searchResults.length === 0}
                              className="px-2 py-1 text-xs bg-white/5 hover:bg-white/10 rounded text-slate-300 disabled:opacity-30 transition-colors"
                              title="Replace match (Enter)"
                            >
                              Replace
                            </button>
                            <button
                              onClick={handleReplaceAll}
                              disabled={searchResults.length === 0}
                              className="px-2 py-1 text-xs bg-white/5 hover:bg-white/10 rounded text-slate-300 disabled:opacity-30 transition-colors"
                              title="Replace all (Shift+Enter)"
                            >
                              Replace All
                            </button>
                          </div>
                        </div>
                      </div>
                    )}
                    {/* Editor Area Wrapper */}
                    <div className="flex-1 flex overflow-hidden relative">
                      {/* Editor Area */}
                      <div className={`flex-1 overflow-hidden relative flex ${splitActiveFileIndex !== null ? 'border-r border-white/5 shadow-[10px_0_15px_-3px_rgba(0,0,0,0.5)] z-10' : ''}`}>
                        <div 
                          id="editor-scroll-container"
                          className="flex-1 overflow-y-auto relative flex"
                        >
                        {/* Line Numbers */}
                      <div className="w-16 py-4 flex flex-col items-center bg-white/[0.02] border-r border-white/5 font-mono text-sm leading-6 shrink-0 select-none">
                        {displayLines.map((_, i) => {
                          const originalLineNum = visibleToOriginal[i];
                          if (originalLineNum === undefined) return null;
                          const isBreakpoint = breakpoints.has(originalLineNum);
                          const isActive = debugState?.activeLine === originalLineNum;
                          const isFoldable = foldableRegions.has(originalLineNum);
                          const isFolded = foldedLines.has(originalLineNum);
                          const lineErrors = lintErrors.filter(e => e.line === i);
                          const hasLintError = lineErrors.some(e => e.type === "error");
                          const hasWarning = lineErrors.some(e => e.type === "warning");
                          const syntaxErrsForLine = syntaxErrors.filter(e => e.line === originalLineNum);
                          const hasSyntaxError = syntaxErrsForLine.length > 0;
                          
                          const errorMessages = [
                              ...lineErrors.map(e => e.message),
                              ...syntaxErrsForLine.map(e => e.message)
                          ].join('\n');
                          const titleText = errorMessages || "Toggle Breakpoint";

                          return (
                            <div
                              key={i}
                              className={`w-full flex relative group px-1 ${isActive ? "text-cyan-400 bg-cyan-500/10 font-bold" : isBreakpoint ? "text-slate-300" : "text-slate-600 hover:text-slate-400"}`}
                            >
                              {/* Breakpoint indicator area */}
                              <div
                                onClick={() => toggleBreakpoint(originalLineNum)}
                                className="w-4 h-full relative cursor-pointer flex-shrink-0"
                                title={titleText}
                              >
                                {isBreakpoint && (
                                  <div className="absolute left-1 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.6)]"></div>
                                )}
                                {!isBreakpoint && (hasLintError || hasSyntaxError) && (
                                  <div 
                                    className="absolute left-1 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-red-500 hover:scale-150 transition-transform cursor-pointer"
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      showEditorToast(errorMessages, "error");
                                    }}
                                  ></div>
                                )}
                                {!isBreakpoint && !(hasLintError || hasSyntaxError) && hasWarning && (
                                  <div 
                                    className="absolute left-1 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-amber-500 hover:scale-150 transition-transform cursor-pointer"
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      showEditorToast(errorMessages, "warning");
                                    }}
                                  ></div>
                                )}
                                {!isBreakpoint && !(hasLintError || hasSyntaxError) && !hasWarning && (
                                  <div className="absolute left-1 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full bg-red-500/30 opacity-0 group-hover:opacity-100 transition-opacity"></div>
                                )}
                              </div>
                              {/* Line number area */}
                              <div 
                                onClick={() => jumpToLine(originalLineNum)}
                                className="flex-1 text-center cursor-pointer"
                                title={`Jump to line ${originalLineNum}`}
                              >
                                {originalLineNum}
                              </div>
                              {/* Quick Fix Toggle */}
                              {lineErrors.some(e => Boolean(e.quickFix)) && (
                                <div className="w-4 h-full flex items-center justify-center flex-shrink-0 z-10 relative">
                                  <div 
                                    className="cursor-pointer text-yellow-500 hover:text-yellow-300 transition-colors"
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      const err = lineErrors.find(e => Boolean(e.quickFix));
                                      if (err && err.quickFix) applyQuickFix(err.quickFix, i, err.match);
                                    }}
                                    title={lineErrors.find(e => Boolean(e.quickFix))?.quickFix?.label}
                                  >
                                    <Lightbulb className="w-3.5 h-3.5" />
                                  </div>
                                </div>
                              )}
                              {/* Fold Toggle */}
                              <div className="w-4 h-full flex items-center justify-center flex-shrink-0">
                                {isFoldable && (
                                   <div 
                                      className="cursor-pointer text-slate-500 hover:text-slate-300 opacity-0 group-hover:opacity-100 transition-opacity"
                                      onClick={(e) => toggleFold(e, originalLineNum)}
                                   >
                                      {isFolded ? <ChevronRight className="w-3.5 h-3.5" /> : <ChevronDown className="w-3.5 h-3.5" />}
                                   </div>
                                )}
                              </div>
                            </div>
                          );
                        })}
                      </div>

                      {/* Text Area */}
                      <div className="flex-1 w-full relative" ref={editorRef}>
                        <Editor
                          value={displayCode}
                          onValueChange={(newDisplayCode) => {
                            const actualNewCode = computeActualCode(newDisplayCode, code, foldedLines);
                            setCode(actualNewCode);
                            const ta = getTextArea();
                            if (ta)
                              updateSuggestions(actualNewCode, ta.selectionStart);
                          }}
                          highlight={(c) => {
                            let html = Prism.highlight(c, Prism.languages.lumino, "lumino");
                            if (advancedSyntaxHighlighting) {
                                html = html
                                    .replace(/\b([a-zA-Z_]\w*)\s*\(/g, '<span class="token function font-bold text-pink-300 drop-shadow-[0_0_8px_rgba(236,72,153,0.8)]">$1</span>(')
                                    .replace(/\b(let|const|fn|if|else|while|return)\b/g, '<span class="token keyword font-extrabold text-fuchsia-400 drop-shadow-[0_0_5px_rgba(232,121,249,0.5)]">$1</span>');
                            }
                            const currentErrors = c === displayCode ? lintErrors : lintLumino(c, customLintRules);
                            if (currentErrors.length > 0) {
                                const lines = html.split('\n');
                                currentErrors.forEach(err => {
                                   let lineHtml = lines[err.line];
                                   if (lineHtml !== undefined) {
                                      const wavyClass = err.type === 'error'
                                        ? "underline decoration-wavy decoration-red-500 cursor-help"
                                        : "underline decoration-wavy decoration-amber-500 cursor-help";
                                      lines[err.line] = insertSpanIntoHtml(lineHtml, err.match, wavyClass, err.message.replace(/"/g, '&quot;'));
                                   }
                                });
                                html = lines.join('\n');
                            }
                            return html;
                          }}
                          padding={16}
                          style={{
                            fontFamily:
                              '"JetBrains Mono", "Fira Code", monospace',
                            fontSize: 14,
                            lineHeight: "24px",
                            minHeight: "100%",
                            backgroundColor: "transparent",
                            outline: "none",
                          }}
                          textareaClassName="focus:outline-none"
                          spellCheck={false}
                          onKeyDown={handleKeyDown as any}
                          onClick={(e: any) =>
                            updateSuggestions(
                              code,
                              (e.target as HTMLTextAreaElement).selectionStart,
                            )
                          }
                          onKeyUp={(e: any) => {
                            if (
                              [
                                "ArrowLeft",
                                "ArrowRight",
                                "ArrowUp",
                                "ArrowDown",
                              ].includes(e.key)
                            ) {
                              updateSuggestions(
                                code,
                                (e.target as HTMLTextAreaElement)
                                  .selectionStart,
                              );
                            }
                          }}
                        />
                        {showSuggestions && (
                          <div
                            className="absolute z-50 bg-[#1e1e24] border border-white/10 rounded shadow-xl py-1 max-h-48 overflow-y-auto"
                            style={{
                              top:
                                suggestionPos.top -
                                (getTextArea()?.scrollTop || 0) +
                                24, // adjust down below text
                              left:
                                suggestionPos.left -
                                (getTextArea()?.scrollLeft || 0) +
                                16, // add padding offset
                            }}
                          >
                            {suggestions.map((sugg, i) => (
                              <div
                                key={sugg.text}
                                onMouseDown={(e) => {
                                  e.preventDefault(); // Prevent blur when clicking
                                  applySuggestion(sugg);
                                }}
                                className={`px-4 py-2 font-mono text-sm cursor-pointer transition-colors flex items-center justify-between gap-4 group relative ${i === selectedSuggestion ? "bg-indigo-500/30 text-indigo-300" : "text-slate-300 hover:bg-white/5"}`}
                              >
                                <span>{sugg.text}</span>
                                {sugg.description && (
                                  <>
                                    <span className="text-[10px] text-slate-500 max-w-[200px] truncate">
                                      {sugg.description}
                                    </span>
                                    <div className="absolute left-[105%] top-0 w-64 bg-[#1e1e24] border border-indigo-500/30 rounded p-3 shadow-2xl opacity-0 group-hover:opacity-100 pointer-events-none transition-opacity z-[60]">
                                      <div className="text-indigo-400 font-bold mb-1">{sugg.text}</div>
                                      <div className="text-xs text-slate-300 whitespace-normal break-words leading-relaxed">{sugg.description}</div>
                                    </div>
                                  </>
                                )}
                              </div>
                            ))}
                          </div>
                        )}
                      </div>
                    </div>
                    <MiniMap code={displayCode} containerId="editor-scroll-container" />
                  </div>
                  
                  {/* Split Editor Pane */}
                  {splitActiveFileIndex !== null && files[splitActiveFileIndex] && (
                     <div className="flex-1 overflow-hidden relative flex flex-col bg-[#050505]">
                        <div className="flex items-center justify-between px-3 h-10 border-b border-white/5 shrink-0 bg-white/[0.02]">
                           <span className="text-xs text-slate-400 font-mono flex items-center gap-2"><FileCode className="w-3.5 h-3.5 text-cyan-400" /> {files[splitActiveFileIndex].name}</span>
                           <button onClick={() => setSplitActiveFileIndex(null)} className="text-slate-500 hover:text-red-400 p-1 hover:bg-white/5 rounded-md transition-colors"><X className="w-3.5 h-3.5" /></button>
                        </div>
                        <div className="flex-1 overflow-y-auto w-full relative">
                           <Editor
                             value={files[splitActiveFileIndex].content}
                             onValueChange={(newCode) => setFiles(prev => prev.map((f, i) => i === splitActiveFileIndex ? { ...f, content: newCode } : f))}
                             highlight={(c) => {
                               let html = Prism.highlight(c, Prism.languages.lumino, "lumino");
                               if (advancedSyntaxHighlighting) {
                                   html = html
                                       .replace(/\b([a-zA-Z_]\w*)\s*\(/g, '<span class="token function font-bold text-pink-300 drop-shadow-[0_0_8px_rgba(236,72,153,0.8)]">$1</span>(')
                                       .replace(/\b(let|const|fn|if|else|while|return)\b/g, '<span class="token keyword font-extrabold text-fuchsia-400 drop-shadow-[0_0_5px_rgba(232,121,249,0.5)]">$1</span>');
                               }
                               return html;
                             }}
                             padding={16}
                             style={{ fontFamily: '"JetBrains Mono", "Fira Code", monospace', fontSize: 14, lineHeight: "24px", minHeight: "100%", backgroundColor: "transparent", outline: "none" }}
                             textareaClassName="focus:outline-none"
                             spellCheck={false}
                           />
                        </div>
                     </div>
                  )}
                  {/* End Wrapper */}
                  </div>

                    {/* Editor Status Bar */}
                    <div className="h-6 flex items-center justify-between px-4 py-1 text-[10px] text-slate-500 font-mono bg-white/[0.02] border-t border-white/5 shrink-0 select-none">
                      <button
                        onClick={() => setIsShortcutsOpen(true)}
                        className="flex items-center gap-1.5 hover:text-slate-300 transition-colors"
                        title="Keyboard Shortcuts"
                      >
                        <Keyboard className="w-3 h-3" />
                        <span>Shortcuts</span>
                      </button>
                        <AnimatePresence mode="wait">
                          <motion.div
                            key={`${code.split(/\s+/).filter(w => w.length > 0).length}-${code.length}`}
                            initial={{ opacity: 0.3 }}
                            animate={{ opacity: 1 }}
                            transition={{ duration: 0.3 }}
                            className="flex items-center"
                          >
                            <span>{code.split(/\s+/).filter(w => w.length > 0).length} Words</span>
                            <span className="mx-2 opacity-50">|</span>
                            <span>{code.length} Characters</span>
                          </motion.div>
                        </AnimatePresence>
                    </div>

                    {/* Console Area */}
                    <div className="h-64 bg-black/80 backdrop-blur-xl border-t border-white/10 flex flex-col shrink-0 relative z-10">
                      <div className="h-10 flex items-center px-4 border-b border-white/5 gap-4">
                        <span
                          onClick={() => setActiveBottomTab("console")}
                          className={getActiveBottomTabClass("console")}
                        >
                          Console
                        </span>
                        <span
                          onClick={() => setActiveBottomTab("terminal")}
                          className={getActiveBottomTabClass("terminal")}
                        >
                          Terminal
                        </span>
                        <span
                          onClick={() => setActiveBottomTab("debugger")}
                          className={getActiveBottomTabClass("debugger")}
                        >
                          ATX Debugger
                        </span>
                        <span
                          onClick={() => setActiveBottomTab("preview")}
                          className={getActiveBottomTabClass("preview")}
                        >
                          Live Preview
                        </span>
                        <span
                          onClick={() => setActiveBottomTab("game")}
                          className={getActiveBottomTabClass("game")}
                        >
                          <span className="flex items-center gap-1.5">
                            <Gamepad2 className="w-3 h-3" /> Minigame
                          </span>
                        </span>
                      </div>
                      <div className="flex-1 p-4 font-mono text-[11px] overflow-y-auto space-y-1">
                        {activeBottomTab === "console" && (
                          <>
                            {output.length === 0 ? (
                              <span className="text-slate-500 italic">
                                No output yet. Click RUN to execute code.
                              </span>
                            ) : (
                              output.map((line, index) => {
                                let colorClass = "text-emerald-400";
                                let label = "[LUMINO]";
                                const processLine = line.toLowerCase();
                                if (processLine.startsWith("error")) {
                                  colorClass = "text-red-400";
                                } else if (processLine.startsWith("warn")) {
                                  colorClass = "text-yellow-400";
                                } else if (processLine.startsWith("info")) {
                                  colorClass = "text-blue-400";
                                }

                                return (
                                  <div key={index} className={colorClass}>
                                    <span className="text-slate-600 mr-2">
                                      {label}
                                    </span>
                                    {line}
                                  </div>
                                );
                              })
                            )}
                            {isRunning && !debugState && (
                              <div className="flex animate-pulse mt-2">
                                <span className="text-cyan-400">&gt;</span>
                                <span className="w-2 bg-cyan-400 ml-1"></span>
                              </div>
                            )}
                            {debugState && (
                              <div className="flex animate-pulse mt-2">
                                <span className="text-amber-400 font-bold">
                                  &gt;
                                </span>
                                <span className="text-amber-500 ml-2 italic text-[10px]">
                                  paused on line {debugState.activeLine}
                                </span>
                              </div>
                            )}
                          </>
                        )}
                        {activeBottomTab === "terminal" && (
                          <div
                            className="h-full w-full bg-[#0c0c0e] -mx-4 -mt-4 py-8 px-4"
                            style={{ height: "calc(100% + 2rem)" }}
                          >
                            <TerminalPanel />
                          </div>
                        )}
                        {activeBottomTab === "debugger" && (
                          <div className="flex flex-col gap-4">
                            <div className="flex items-center gap-4">
                              <label className="bg-white/10 hover:bg-white/20 text-slate-300 px-3 py-1.5 rounded cursor-pointer transition-colors border border-white/5">
                                <span>Select .ATX Binary File</span>
                                <input
                                  type="file"
                                  className="hidden"
                                  onChange={handleDebugFileUpload}
                                />
                              </label>
                              {debuggerBuffer && (
                                <span className="text-slate-400">
                                  Loaded: {debuggerBuffer.byteLength} bytes
                                </span>
                              )}
                            </div>
                            {debuggerError ? (
                              <div className="text-red-400 flex items-center gap-2">
                                <span className="font-bold">ERROR:</span>{" "}
                                {debuggerError}
                              </div>
                            ) : debuggerHeader ? (
                              <div className="bg-black/50 border border-white/10 rounded p-4 space-y-2">
                                <h3 className="font-bold text-cyan-400 mb-2">
                                  VALID ATX HEADER FOUND
                                </h3>
                                <div className="grid grid-cols-2 gap-2 max-w-sm">
                                  <span className="text-slate-500 text-right">
                                    Magic Bytes:
                                  </span>
                                  <span className="text-emerald-400 font-bold">
                                    {debuggerHeader.magicBytes}
                                  </span>
                                  <span className="text-slate-500 text-right">
                                    Version:
                                  </span>
                                  <span className="text-slate-300">
                                    {debuggerHeader.version}
                                  </span>
                                  <span className="text-slate-500 text-right">
                                    Entry Point:
                                  </span>
                                  <span className="text-slate-300">
                                    0x{debuggerHeader.entryPoint.toString(16)}
                                  </span>
                                  <span className="text-slate-500 text-right">
                                    Flags:
                                  </span>
                                  <span className="text-slate-300">
                                    0b
                                    {debuggerHeader.flags
                                      .toString(2)
                                      .padStart(16, "0")}
                                  </span>
                                  <span className="text-slate-500 text-right">
                                    Object Type:
                                  </span>
                                  <span className="text-slate-300">
                                    {debuggerHeader.objectType}
                                  </span>
                                </div>
                              </div>
                            ) : (
                              <div className="text-slate-600 italic">
                                Select a binary file to inspect its header
                                structure according to the ATX executable
                                specification.
                              </div>
                            )}
                          </div>
                        )}
                        {activeBottomTab === "preview" && (
                          <div
                            className="h-full w-full bg-[#0c0c0e] -mx-4 -mt-4"
                            style={{ height: "calc(100% + 2rem)" }}
                          >
                            <PreviewPanel
                              files={files}
                              activeIndex={activeFileIndex}
                            />
                          </div>
                        )}
                        {activeBottomTab === "game" && (
                          <div
                            className="h-full w-full bg-[#0c0c0e] -mx-4 -mt-4 py-8 px-4"
                            style={{ height: "calc(100% + 2rem)" }}
                          >
                            <SnakeGame />
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                )}

                {/* Inspector Panel */}
                {!isDictionaryOpen && !isLibraryOpen && !isRegistryOpen && (
                  <aside className="w-56 border-l border-white/5 bg-white/[0.01] backdrop-blur-lg flex flex-col shrink-0 overflow-y-auto">
                    <div className="p-4">
                      <span className="text-[10px] font-bold uppercase tracking-[0.2em] text-slate-500 flex items-center gap-2">
                        <ListTree className="w-3 h-3" /> Variables
                      </span>
                      <div className="mt-4 space-y-2">
                        {debugState ? (
                          Object.entries(debugState.variables).length > 0 ? (
                            Object.entries(debugState.variables).map(
                              ([key, val]) => (
                                <div
                                  key={key}
                                  className="bg-white/5 rounded p-2 border border-white/5 text-xs font-mono"
                                >
                                  <span className="text-pink-400">{key}</span>:{" "}
                                  <span className="text-cyan-400">
                                    {typeof val === "string"
                                      ? `"${val}"`
                                      : String(val)}
                                  </span>
                                </div>
                              ),
                            )
                          ) : (
                            <div className="text-[10px] text-slate-600 italic">
                              No variables set yet.
                            </div>
                          )
                        ) : (
                          <div className="text-[10px] text-slate-600 italic">
                            Start debugging to inspect variables.
                          </div>
                        )}
                      </div>
                    </div>

                    {/* Watch Window */}
                    <div className="p-4 border-t border-white/5">
                      <span className="text-[10px] font-bold uppercase tracking-[0.2em] text-slate-500 flex items-center gap-2">
                        <Bug className="w-3 h-3" /> Watches
                      </span>
                      <div className="mt-4 space-y-2">
                        {watches.map((w, idx) => {
                          let displayVal = "undefined";
                          if (debugState && debugState.evaluator && debugState.env) {
                             try {
                               const lexer = new Lexer(w);
                               const parser = new Parser(lexer);
                               const prog = parser.parseProgram();
                               if (parser.errors.length === 0 && prog.statements.length > 0 && prog.statements[0].type === "ExpressionStatement") {
                                 const res = debugState.evaluator.eval(prog.statements[0].expression, debugState.env);
                                 if (res && res.type) displayVal = res.inspect();
                               }
                             } catch(e) {}
                          }
                          return (
                            <div key={idx} className="bg-white/5 rounded p-2 border border-white/5 text-xs font-mono relative group">
                              <span className="text-yellow-400">{w}</span>:{" "}
                              <span className="text-emerald-400">{displayVal}</span>
                              <button 
                                onClick={() => setWatches(watches.filter((_, i) => i !== idx))}
                                className="absolute right-1 top-1 opacity-0 group-hover:opacity-100 hover:text-red-400 p-0.5 pointer-events-auto"
                              >
                                <X className="w-3 h-3" />
                              </button>
                            </div>
                          );
                        })}
                        <div className="flex items-center gap-1 mt-2">
                          <input 
                            type="text" 
                            value={newWatchInput}
                            onChange={(e) => setNewWatchInput(e.target.value)}
                            placeholder="Add expression..."
                            className="bg-black/30 border border-white/10 rounded px-2 py-1 flex-1 text-xs text-slate-300 outline-none focus:border-white/20"
                            onKeyDown={(e) => {
                              if (e.key === "Enter" && newWatchInput.trim()) {
                                setWatches([...watches, newWatchInput.trim()]);
                                setNewWatchInput("");
                              }
                            }}
                          />
                        </div>
                      </div>
                    </div>
                  </aside>
                )}

                <AnimatePresence>
                  {isChatOpen && (
                    <motion.div
                      initial={{ width: 0, opacity: 0 }}
                      animate={{ width: 320, opacity: 1 }}
                      exit={{ width: 0, opacity: 0 }}
                      transition={{ duration: 0.3, ease: "easeInOut" }}
                      className="overflow-hidden shrink-0 border-l border-white/5 h-full flex flex-col"
                    >
                      <div className="w-[320px] h-full flex flex-col">
                        <AIChat
                          onClose={() => setIsChatOpen(false)}
                          onApplyCode={(c) => setCode(c)}
                          guidedMode={projectScaffolded}
                        />
                      </div>
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            </div>

            {/* End Workspace Window Wrapper */}
          </div>
        </div>
      </div>

      {/* Taskbar */}
      <div className="h-12 bg-black/80 backdrop-blur-xl border-t border-white/10 flex items-center justify-between px-2 shrink-0 z-[100] relative">
        {/* Start Button & Quick Actions */}
        <div className="flex items-center gap-2">
          <div className="relative">
            <button
              onClick={() => setIsStartMenuOpen(!isStartMenuOpen)}
              className="w-10 h-10 hover:bg-white/10 rounded flex items-center justify-center transition-colors"
            >
              <Grid3x3 className="w-5 h-5 text-cyan-400" />
            </button>

            {/* Start Menu Dropdown */}
            <AnimatePresence>
              {isStartMenuOpen && (
                <motion.div
                  initial={{ opacity: 0, y: 10, scale: 0.95 }}
                  animate={{ opacity: 1, y: 0, scale: 1 }}
                  exit={{ opacity: 0, y: 10, scale: 0.95 }}
                  className="absolute bottom-14 left-0 w-64 bg-[#08080a]/95 backdrop-blur-xl border border-white/20 rounded-lg shadow-2xl p-2 flex flex-col gap-1 overflow-hidden z-[100]"
                >
                  <div className="text-[10px] font-bold text-slate-500 tracking-wider uppercase px-2 py-1 mb-1">
                    Programme
                  </div>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      setIsChatOpen(false);
                      closeAllPanels();
                    }}
                  >
                    <Code2 className="w-4 h-4 text-cyan-400" /> ATOS IDE
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsProjectManagerOpen(true);
                    }}
                  >
                    <FolderGit2 className="w-4 h-4 text-emerald-400" />{" "}
                    Projektmanager
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsDictionaryOpen(true);
                    }}
                  >
                    <Book className="w-4 h-4 text-indigo-400" /> Dictionary
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsPluginsOpen(true);
                    }}
                  >
                    <Plug className="w-4 h-4 text-fuchsia-400" /> Plugin Store
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsVersionControlOpen(true);
                    }}
                  >
                    <GitBranch className="w-4 h-4 text-emerald-400" /> Version Control
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsTestingOpen(true);
                    }}
                  >
                    <Beaker className="w-4 h-4 text-indigo-400" /> Unit Testing
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsCustomLinterOpen(true);
                    }}
                  >
                    <ShieldAlert className="w-4 h-4 text-amber-400" /> Custom Linter & Rules
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsAuditOpen(true);
                    }}
                  >
                    <ShieldCheck className="w-4 h-4 text-cyan-400" /> Audit & Code-Health Suite
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsAtcVmOpen(true);
                    }}
                  >
                    <Terminal className="w-4 h-4 text-cyan-400" /> ATC-VM Simulator & Debugger
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsAtcComplianceOpen(true);
                    }}
                  >
                    <ShieldCheck className="w-4 h-4 text-emerald-400" /> ATC-DOC Linter & Compliance
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsGenesisOpen(true);
                    }}
                  >
                    <Server className="w-4 h-4 text-indigo-400" /> Genesis & Devnet Configurator
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsRustWorkspaceOpen(true);
                    }}
                  >
                    <Boxes className="w-4 h-4 text-orange-400" /> Rust Cargo Workspace
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsLanguageStrategyOpen(true);
                    }}
                  >
                    <Code2 className="w-4 h-4 text-indigo-400" /> Sprachstrategie & Matrix
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsGlobusFormatsOpen(true);
                    }}
                  >
                    <Binary className="w-4 h-4 text-cyan-400" /> Dateiformate (GFFA / .g*)
                  </button>
                  <button
                    className="flex items-center gap-3 px-2 py-2 hover:bg-white/10 rounded-md text-sm text-slate-300 w-full text-left"
                    onClick={() => {
                      setIsStartMenuOpen(false);
                      setIsWorkspaceMinimized(false);
                      closeAllPanels();
                      setIsAudioEngineOpen(true);
                    }}
                  >
                    <Volume2 className="w-4 h-4 text-violet-400" /> Audio Engine
                  </button>
                </motion.div>
              )}
            </AnimatePresence>
          </div>

          <div className="w-px h-6 bg-white/10 mx-1"></div>

          {/* Open Apps Indicators */}
          <button
            className={`w-10 h-10 rounded flex items-center justify-center transition-all ${!isWorkspaceMinimized ? "bg-white/10 border-b-2 border-cyan-500" : "hover:bg-white/5"}`}
            onClick={() => setIsWorkspaceMinimized(false)}
            title="ATOS Workspace"
          >
            <Layers
              className={`w-5 h-5 ${!isWorkspaceMinimized ? "text-cyan-400" : "text-slate-400"}`}
            />
          </button>
          <button
            className={`w-10 h-10 rounded flex items-center justify-center transition-all hover:bg-white/5`}
            onClick={() => {
              setIsChatOpen(!isChatOpen);
              setIsWorkspaceMinimized(false);
            }}
            title="AI Assistant"
          >
            <Bot
              className={`w-5 h-5 ${isChatOpen ? "text-purple-400 drop-shadow-[0_0_5px_rgba(168,85,247,0.5)]" : "text-slate-400"}`}
            />
          </button>
        </div>

        {/* System Tray */}
        <div className="flex items-center gap-3 text-white px-2">
          <button className="hover:bg-white/10 p-1.5 rounded-md transition-colors">
            <Wifi className="w-4 h-4" />
          </button>
          <button className="hover:bg-white/10 p-1.5 rounded-md transition-colors">
            <Volume2 className="w-4 h-4" />
          </button>
          <button className="hover:bg-white/10 p-1.5 rounded-md transition-colors">
            <Battery className="w-4 h-4" />
          </button>
          <div className="text-xs font-medium px-2 py-1 hover:bg-white/10 rounded-md transition-colors cursor-pointer text-right leading-tight">
            <div>
              {new Date().toLocaleTimeString("de-DE", {
                hour: "2-digit",
                minute: "2-digit",
              })}
            </div>
            <div>{new Date().toLocaleDateString("de-DE")}</div>
          </div>
        </div>
      </div>

      {/* Desktop Wrapper End */}

      {isAutoArchitectOpen && (
        <AutoArchitectModal
          onClose={() => setIsAutoArchitectOpen(false)}
          onComplete={(prompt, generatedFiles) => {
            setIsAutoArchitectOpen(false);
            setFileHistory((prev) => [
              ...prev,
              JSON.parse(JSON.stringify(files)),
            ]);
            setFiles([...files, ...generatedFiles]);
            setProjectScaffolded(true);
            setIsChatOpen(true);
            setActiveFileIndex(files.length);
          }}
        />
      )}

      {isGameScaffoldOpen && (
        <GameScaffoldModal
          onClose={() => setIsGameScaffoldOpen(false)}
          onComplete={(genre, generatedFiles) => {
            setIsGameScaffoldOpen(false);
            setFileHistory((prev) => [
              ...prev,
              JSON.parse(JSON.stringify(files)),
            ]);
            setFiles([...files, ...generatedFiles]);
            setProjectScaffolded(true);
            setIsChatOpen(true);
            setActiveFileIndex(files.length);
          }}
        />
      )}

      <CommandPalette 
        isOpen={isCommandPaletteOpen}
        onClose={() => setIsCommandPaletteOpen(false)}
        actions={{
          runCode,
          formatCode: handleFormatCode,
          saveFile: () => {
            localStorage.setItem("atos-files", JSON.stringify(files));
            showEditorToast("All files saved successfully.", "info");
          },
          toggleTerminal: () => {
            setActiveBottomTab(activeBottomTab === 'terminal' ? 'console' : 'terminal');
          },
          openGitHub: () => {
            setIsGitHubSyncOpen(true);
          },
          openCiCd: () => {
            setIsCiCdGeneratorOpen(true);
          },
          openCustomLinter: () => {
            closeAllPanels();
            setIsCustomLinterOpen(true);
          },
          openAuditSuite: () => {
            closeAllPanels();
            setIsAuditOpen(true);
          },
          applyAllAutoFixes: () => {
            const { updatedFiles, fixedCount } = applyAllAutoFixes(files, liveAuditSummary.findings);
            if (fixedCount > 0) {
              setFiles(updatedFiles);
              showEditorToast(`${fixedCount} Verbesserungen automatisch angewendet!`, "info");
            } else {
              showEditorToast("Keine automatisch behebbaren Probleme gefunden.", "info");
            }
          }
        }}
      />

      <KeyboardShortcutsModal
        isOpen={isShortcutsOpen}
        onClose={() => setIsShortcutsOpen(false)}
      />

      {isProjectManagerOpen && (
        <ProjectManagerModal
          onClose={() => setIsProjectManagerOpen(false)}
          currentFiles={files}
          onLoadProject={(loadedFiles) => {
            setFileHistory((prev) => [
              ...prev,
              JSON.parse(JSON.stringify(files)),
            ]);
            setFiles(loadedFiles);
            setActiveFileIndex(0);
          }}
        />
      )}

      {isGitHubSyncOpen && (
        <GitHubSyncModal
          isOpen={isGitHubSyncOpen}
          onClose={() => setIsGitHubSyncOpen(false)}
          files={files}
          onOpenCiCd={() => {
            setIsGitHubSyncOpen(false);
            setIsCiCdGeneratorOpen(true);
          }}
          onLoadFiles={(loadedFiles) => {
            setFileHistory((prev) => [
              ...prev,
              JSON.parse(JSON.stringify(files)),
            ]);
            setFiles(loadedFiles);
            setActiveFileIndex(0);
            showEditorToast("GitHub Repository erfolgreich geladen.", "info");
          }}
        />
      )}

      {isCiCdGeneratorOpen && (
        <CiCdGeneratorModal
          isOpen={isCiCdGeneratorOpen}
          onClose={() => setIsCiCdGeneratorOpen(false)}
          files={files}
          onSaveWorkflowFile={handleSaveWorkflowFile}
        />
      )}

      {isScaffoldConfirmOpen && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm">
          <div className="bg-[#1e1e24] border border-white/10 p-6 rounded-lg shadow-2xl max-w-sm w-full mx-4">
            <h2 className="text-lg font-bold text-slate-200 mb-2">
              ATOS Projekt initialisieren
            </h2>
            <p className="text-sm text-slate-400 mb-6">
              Dadurch wird eine Standard-Projektordnerstruktur und ein
              Roadmap-Wiki mit allen To-Dos erstellt. Bist du sicher?
            </p>
            <div className="flex justify-end gap-3">
              <button
                onClick={() => setIsScaffoldConfirmOpen(false)}
                className="px-4 py-2 text-sm text-slate-300 hover:text-slate-100 transition-colors"
              >
                Abbrechen
              </button>
              <button
                onClick={confirmScaffold}
                className="px-4 py-2 bg-cyan-500 hover:bg-cyan-400 text-white rounded text-sm font-bold shadow-lg shadow-cyan-500/20 transition-all"
              >
                Projekt generieren
              </button>
            </div>
          </div>
        </div>
      )}

      <AnimatePresence>
        {editorToast && (
          <motion.div
            initial={{ opacity: 0, y: 50, scale: 0.95 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: 20, scale: 0.95 }}
            className={`fixed bottom-16 right-6 z-[200] max-w-md p-4 rounded-xl shadow-2xl border ${
              editorToast.type === 'error' ? 'bg-red-950/80 border-red-800/50 text-red-200' :
              editorToast.type === 'warning' ? 'bg-amber-950/80 border-amber-800/50 text-amber-200' :
              'bg-blue-950/80 border-blue-800/50 text-blue-200'
            } backdrop-blur-xl pointer-events-auto`}
          >
            <div className="flex items-start gap-4">
              <div className="mt-1 flex-shrink-0">
                {editorToast.type === 'error' ? (
                  <div className="w-6 h-6 rounded-full bg-red-500/20 flex items-center justify-center border border-red-500/50">
                    <div className="w-2.5 h-2.5 rounded-full bg-red-500 animate-pulse"></div>
                  </div>
                ) : editorToast.type === 'warning' ? (
                  <div className="w-6 h-6 rounded-full bg-amber-500/20 flex items-center justify-center border border-amber-500/50">
                    <div className="w-2.5 h-2.5 rounded-full bg-amber-500 animate-pulse"></div>
                  </div>
                ) : (
                  <div className="w-6 h-6 rounded-full bg-blue-500/20 flex items-center justify-center border border-blue-500/50">
                    <div className="w-2.5 h-2.5 rounded-full bg-blue-500 animate-pulse"></div>
                  </div>
                )}
              </div>
              <div className="flex-1 min-w-0">
                <h4 className={`font-bold text-sm mb-1.5 uppercase tracking-wider ${
                  editorToast.type === 'error' ? 'text-red-400' : 
                  editorToast.type === 'warning' ? 'text-amber-400' : 
                  'text-blue-400'
                }`}>
                  {editorToast.type === 'error' ? 'Syntax Error' : 
                   editorToast.type === 'warning' ? 'Linter Warning' : 
                   'Information'}
                </h4>
                <div className="text-[13px] whitespace-pre-wrap break-words opacity-90 leading-relaxed font-mono">
                  {editorToast.message}
                </div>
              </div>
              <button 
                onClick={() => setEditorToast(null)}
                className="opacity-50 hover:opacity-100 transition-opacity p-1 text-2xl leading-none"
              >
                &times;
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
