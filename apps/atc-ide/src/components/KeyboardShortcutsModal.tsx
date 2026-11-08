import React from "react";
import { X, Keyboard } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

export function KeyboardShortcutsModal({
  isOpen,
  onClose,
}: {
  isOpen: boolean;
  onClose: () => void;
}) {
  const shortcuts = [
    { keys: ["Ctrl", "F"], description: "Find in file" },
    { keys: ["Ctrl", "S"], description: "Save file (Auto-saved)" },
    { keys: ["Tab"], description: "Indent / Accept Autocomplete" },
    { keys: ["Esc"], description: "Close search / Dialogs" },
    { keys: ["Enter"], description: "Next search match" },
    { keys: ["Shift", "Enter"], description: "Previous search match / Replace all" },
  ];

  return (
    <AnimatePresence>
      {isOpen && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm">
          <motion.div
            initial={{ opacity: 0, scale: 0.95, y: 10 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: 10 }}
            className="w-[450px] bg-[#0f0f13] border border-white/10 rounded-xl shadow-2xl overflow-hidden flex flex-col"
          >
            <div className="h-12 border-b border-white/10 flex items-center justify-between px-4 bg-white/[0.02]">
              <div className="flex items-center gap-2">
                <Keyboard className="w-5 h-5 text-fuchsia-400" />
                <span className="font-bold text-sm text-slate-200">
                  Keyboard Shortcuts
                </span>
              </div>
              <button
                onClick={onClose}
                className="p-1.5 hover:bg-white/10 rounded-md text-slate-400 hover:text-white transition-colors"
                title="Close"
              >
                <X className="w-4 h-4" />
              </button>
            </div>
            <div className="p-4 flex-1 overflow-y-auto">
              <div className="space-y-2">
                {shortcuts.map((shortcut, i) => (
                  <div
                    key={i}
                    className="flex items-center justify-between py-2 border-b border-white/5 last:border-0"
                  >
                    <span className="text-sm text-slate-400">
                      {shortcut.description}
                    </span>
                    <div className="flex items-center gap-1.5">
                      {shortcut.keys.map((key, j) => (
                        <React.Fragment key={j}>
                          <kbd className="px-2 py-1 bg-white/10 border border-white/10 rounded text-xs font-mono text-slate-200 shadow-sm min-w-[24px] text-center">
                            {key}
                          </kbd>
                          {j < shortcut.keys.length - 1 && (
                            <span className="text-slate-500 text-xs">+</span>
                          )}
                        </React.Fragment>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
            <div className="p-4 border-t border-white/5 bg-black/20 text-center">
              <p className="text-xs text-slate-500">
                Note: macOS users should use <kbd className="font-mono text-slate-400">Cmd</kbd> instead of <kbd className="font-mono text-slate-400">Ctrl</kbd>.
              </p>
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
}
