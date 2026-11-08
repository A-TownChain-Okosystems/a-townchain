import React, { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Search, Play, Wand2, Save, TerminalSquare, Github, Zap, ShieldAlert, ShieldCheck, Sparkles, RefreshCw } from 'lucide-react';

interface CommandPaletteProps {
  isOpen: boolean;
  onClose: () => void;
  actions: {
    runCode: () => void;
    formatCode: () => void;
    saveFile: () => void;
    toggleTerminal: () => void;
    openGitHub?: () => void;
    openCiCd?: () => void;
    openCustomLinter?: () => void;
    openAuditSuite?: () => void;
    applyAllAutoFixes?: () => void;
    openAutoSyncHub?: () => void;
  };
}

export function CommandPalette({ isOpen, onClose, actions }: CommandPaletteProps) {
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const commands = [
    { id: 'run', label: 'Run Code', icon: Play, action: actions.runCode },
    { id: 'format', label: 'Format Code', icon: Wand2, action: actions.formatCode },
    { id: 'save', label: 'Save All', icon: Save, action: actions.saveFile },
    { id: 'terminal', label: 'Toggle Terminal', icon: TerminalSquare, action: actions.toggleTerminal },
    ...(actions.openAutoSyncHub ? [{ id: 'autosync', label: 'Auto-Sync & Artefakt-Zentrale (Wiki, Roadmap, Todos, Sprints, Docs, Version)', icon: RefreshCw, action: actions.openAutoSyncHub }] : []),
    ...(actions.openAuditSuite ? [{ id: 'audit', label: 'ATOS Audit & Code-Health Suite (Security, Vollständigkeit, Verknüpfung)', icon: ShieldCheck, action: actions.openAuditSuite }] : []),
    ...(actions.applyAllAutoFixes ? [{ id: 'autofix', label: 'Alle Code-Verbesserungen automatisch anwenden (Auto-Fix)', icon: Sparkles, action: actions.applyAllAutoFixes }] : []),
    ...(actions.openCustomLinter ? [{ id: 'linter', label: 'Custom Linting Rules (Eigene Regeln & Prüfungen)', icon: ShieldAlert, action: actions.openCustomLinter }] : []),
    ...(actions.openGitHub ? [{ id: 'github', label: 'GitHub Sync & Push', icon: Github, action: actions.openGitHub }] : []),
    ...(actions.openCiCd ? [{ id: 'cicd', label: 'CI/CD Workflow Generator (.github/workflows/lumino-build.yml)', icon: Zap, action: actions.openCiCd }] : []),
  ].filter(cmd => cmd.label.toLowerCase().includes(query.toLowerCase()));

  useEffect(() => {
    if (isOpen) {
      setQuery('');
      setSelectedIndex(0);
      setTimeout(() => inputRef.current?.focus(), 50);
    }
  }, [isOpen]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((s) => Math.min(s + 1, commands.length - 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex((s) => Math.max(s - 1, 0));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (commands[selectedIndex]) {
        commands[selectedIndex].action();
        onClose();
      }
    } else if (e.key === 'Escape') {
      e.preventDefault();
      onClose();
    }
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 bg-black/60 backdrop-blur-sm z-[9990]"
            onClick={onClose}
          />
          <motion.div
            initial={{ opacity: 0, scale: 0.95, y: -20 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: -20 }}
            transition={{ duration: 0.15 }}
            className="fixed top-[20vh] left-1/2 -translate-x-1/2 w-full max-w-lg bg-slate-900 border border-white/20 rounded-xl shadow-2xl z-[9999] overflow-hidden flex flex-col"
          >
            <div className="p-3 border-b border-white/10 flex items-center gap-3">
              <Search className="w-5 h-5 text-slate-400" />
              <input
                ref={inputRef}
                type="text"
                value={query}
                onChange={(e) => {
                  setQuery(e.target.value);
                  setSelectedIndex(0);
                }}
                onKeyDown={handleKeyDown}
                placeholder="Type a command or search..."
                className="flex-1 bg-transparent text-slate-200 outline-none placeholder:text-slate-500"
              />
            </div>
            <div className="max-h-[60vh] overflow-y-auto p-2">
              {commands.length > 0 ? (
                commands.map((cmd, idx) => (
                  <div
                    key={cmd.id}
                    onClick={() => {
                      cmd.action();
                      onClose();
                    }}
                    onMouseEnter={() => setSelectedIndex(idx)}
                    className={`flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-colors ${
                      idx === selectedIndex ? 'bg-cyan-500/20 text-cyan-400' : 'text-slate-300 hover:bg-white/5'
                    }`}
                  >
                    <cmd.icon className="w-4 h-4" />
                    <span className="text-sm font-medium">{cmd.label}</span>
                  </div>
                ))
              ) : (
                <div className="p-4 text-center text-slate-500 text-sm">
                  No commands found
                </div>
              )}
            </div>
            <div className="p-2 border-t border-white/5 flex gap-4 text-[10px] text-slate-500 bg-white/5">
              <span><kbd className="bg-white/10 px-1 py-0.5 rounded mr-1">↑↓</kbd> to navigate</span>
              <span><kbd className="bg-white/10 px-1 py-0.5 rounded mr-1">Enter</kbd> to select</span>
              <span><kbd className="bg-white/10 px-1 py-0.5 rounded mr-1">Esc</kbd> to close</span>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
}
