import React, { useState, useEffect } from "react";
import { MonitorPlay, RefreshCw, LayoutTemplate } from "lucide-react";
import { FileState } from "../App";

interface PreviewPanelProps {
  files: FileState[];
  activeIndex: number;
}

export function PreviewPanel({ files, activeIndex }: PreviewPanelProps) {
  const activeFile = files[activeIndex];
  const [htmlContent, setHtmlContent] = useState("");
  const [isHtml, setIsHtml] = useState(false);

  useEffect(() => {
    if (activeFile) {
      if (activeFile.name.endsWith(".html")) {
        setHtmlContent(activeFile.content);
        setIsHtml(true);
      } else if (
        activeFile.content.includes("<html") ||
        activeFile.content.includes("<div")
      ) {
        setHtmlContent(activeFile.content);
        setIsHtml(true);
      } else {
        setIsHtml(false);
      }
    }
  }, [activeFile]);

  if (!isHtml) {
    return (
      <div className="h-full flex items-center justify-center font-sans bg-[#0c0c0e]">
        <div className="text-slate-500 italic flex flex-col items-center gap-2 max-w-sm text-center p-6 border border-white/5 bg-black/40 rounded-xl relative overflow-hidden group">
          <div className="absolute inset-0 bg-gradient-to-b from-purple-500/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
          <MonitorPlay className="w-10 h-10 opacity-50 mb-2 text-purple-400 group-hover:scale-110 transition-transform" />
          <span className="font-bold text-slate-300">Live HTML/UI Preview</span>
          <span className="text-[11px] leading-relaxed">
            Aktuelle Datei ist kein HTML. Rendern von{" "}
            {activeFile?.name || "Dateien"} wird in diesem Modus nicht direkt
            unterstützt. Erstelle eine HTML-Datei oder kompiliere Lumino /
            React-Code zu HTML.
          </span>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col bg-white rounded-lg overflow-hidden border border-slate-700 font-sans">
      <div className="bg-slate-800 text-slate-200 px-3 py-2 flex items-center gap-3 text-xs border-b border-slate-700">
        <div className="flex gap-1.5">
          <div className="w-2.5 h-2.5 rounded-full bg-red-400"></div>
          <div className="w-2.5 h-2.5 rounded-full bg-yellow-400"></div>
          <div className="w-2.5 h-2.5 rounded-full bg-emerald-400"></div>
        </div>
        <div className="flex-1 text-center bg-slate-900 rounded-md py-1 border border-slate-700/50 flex items-center justify-center gap-2">
          <LayoutTemplate className="w-3.5 h-3.5 text-slate-400" />
          <span>http://localhost:3000/{activeFile.name}</span>
        </div>
        <RefreshCw className="w-3.5 h-3.5 text-slate-400 cursor-pointer hover:text-slate-200" />
      </div>
      <div className="flex-1 bg-white relative">
        <iframe
          title="preview"
          sandbox="allow-scripts allow-same-origin"
          className="w-full h-full border-none absolute inset-0"
          srcDoc={htmlContent}
        />
      </div>
    </div>
  );
}
