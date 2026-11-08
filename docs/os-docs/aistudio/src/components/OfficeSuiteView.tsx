import React, { useState } from "react";
import DOMPurify from "dompurify";
import { evaluate } from "mathjs";
import {
  FileText,
  Table,
  Presentation,
  Grid,
  Download,
  Share2,
  Save,
  Printer,
  ChevronDown,
  List,
  AlignLeft,
  Bold,
  Italic,
  Plus,
  Play
} from "lucide-react";

export function OfficeSuiteView({ defaultAppType = "docs" }: { defaultAppType?: "docs" | "sheets" | "slides" }) {
  const [appType, setAppType] = useState<"docs" | "sheets" | "slides">(defaultAppType);
  
  // Docs State
  const [docContent, setDocContent] = useState(
    `<h1 class="text-3xl font-bold mb-4">A-Town Decentralized Manifesto</h1><p class="text-slate-700 leading-relaxed mb-4">This document is stored securely on the ATC-OS decentralized mesh. Editing occurs in real-time...</p>`
  );

  // Sheets State
  const [cells, setCells] = useState<{ [key: string]: string }>({
    "A1": "Revenue", "B1": "1200",
    "A2": "Costs", "B2": "400",
    "A3": "Profit", "B3": "=B1-B2"
  });
  const [activeCell, setActiveCell] = useState("A1");

  const updateCell = (id: string, value: string) => {
    setCells(prev => ({ ...prev, [id]: value }));
  };

  const evaluateCell = (value: string) => {
    if (value && value.startsWith("=")) {
      try {
        let formula = value.substring(1).toUpperCase();
        Object.keys(cells).forEach(key => {
          formula = formula.replace(key, cells[key] || "0");
        });
        // Basic eval for simple math using math.js
        return evaluate(formula);
      } catch (e) {
        return "ERROR";
      }
    }
    return value;
  };

  // Slides State
  const [currentSlide, setCurrentSlide] = useState(0);
  const slides = [
    { title: "A-Town Network", subtitle: "The Future of Decentralized OS" },
    { title: "Layer 1 Convergence", subtitle: "Seamlessly bridging legacy protocols" },
    { title: "Zero Trust Compute", subtitle: "Every node verified, every cycle secured" }
  ];

  const handleCommand = (command: string) => {
    document.execCommand(command, false, undefined);
  };

  return (
    <div className="flex flex-col h-full bg-[#f8fafc] text-slate-800 border border-atc-border/50 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center justify-between px-4 h-14 bg-indigo-600 text-white shrink-0 shadow-md z-10">
        <div className="flex items-center gap-4">
          <div className="flex gap-1 bg-black/20 rounded-lg p-1">
            <button
              onClick={() => setAppType("docs")}
              className={`p-1.5 rounded flex items-center justify-center transition-colors ${appType === "docs" ? "bg-white/20 text-white" : "text-white/60 hover:text-white"}`}
            >
              <FileText className="w-4 h-4" />
            </button>
            <button
              onClick={() => setAppType("sheets")}
              className={`p-1.5 rounded flex items-center justify-center transition-colors ${appType === "sheets" ? "bg-white/20 text-white" : "text-white/60 hover:text-white"}`}
            >
              <Table className="w-4 h-4" />
            </button>
            <button
              onClick={() => setAppType("slides")}
              className={`p-1.5 rounded flex items-center justify-center transition-colors ${appType === "slides" ? "bg-white/20 text-white" : "text-white/60 hover:text-white"}`}
            >
              <Presentation className="w-4 h-4" />
            </button>
          </div>
          <div>
            <h2 className="font-bold text-sm tracking-tight flex items-center gap-2">
              Untitled Document{" "}
              <ChevronDown className="w-3 h-3 text-white/60" />
            </h2>
            <div className="flex items-center gap-3 text-[10px] text-indigo-200 mt-0.5">
              <span className="hover:underline cursor-pointer">File</span>
              <span className="hover:underline cursor-pointer">Edit</span>
              <span className="hover:underline cursor-pointer">View</span>
              <span className="hover:underline cursor-pointer">Insert</span>
              <span className="hover:underline cursor-pointer">Format</span>
            </div>
          </div>
        </div>
        <div className="flex gap-2">
          {appType === "slides" && (
            <button className="flex items-center gap-2 bg-indigo-400 hover:bg-indigo-500 px-4 py-1.5 rounded font-bold text-xs shadow transition-colors">
              <Play className="w-3 h-3" /> Present
            </button>
          )}
          <button className="flex items-center gap-2 bg-emerald-500 hover:bg-emerald-600 px-4 py-1.5 rounded font-bold text-xs shadow transition-colors">
            <Share2 className="w-3 h-3" /> Share
          </button>
          <button className="p-1.5 hover:bg-white/10 rounded-full transition-colors">
            <div className="w-6 h-6 rounded-full bg-white/20" />
          </button>
        </div>
      </div>

      {/* Toolbar */}
      <div className="h-10 bg-white border-b border-slate-200 flex items-center px-4 gap-4 shrink-0 overflow-x-auto custom-scrollbar">
        <div className="flex items-center gap-1">
          <button className="p-1.5 hover:bg-slate-100 rounded text-slate-600">
            <Save className="w-4 h-4" />
          </button>
          <button className="p-1.5 hover:bg-slate-100 rounded text-slate-600">
            <Printer className="w-4 h-4" />
          </button>
        </div>
        <div className="w-px h-5 bg-slate-200" />
        <div className="flex items-center gap-1">
          <span className="text-sm px-2 py-1 hover:bg-slate-100 rounded cursor-pointer font-serif border border-transparent hover:border-slate-200">
            Inter
          </span>
          <div className="w-px h-5 bg-slate-200 mx-1" />
          <span className="text-sm px-2 py-1 hover:bg-slate-100 rounded cursor-pointer border border-transparent hover:border-slate-200">
            11
          </span>
        </div>
        <div className="w-px h-5 bg-slate-200" />
        <div className="flex items-center gap-1">
          <button onClick={() => handleCommand('bold')} className="p-1.5 hover:bg-slate-100 rounded text-slate-600">
            <Bold className="w-4 h-4" />
          </button>
          <button onClick={() => handleCommand('italic')} className="p-1.5 hover:bg-slate-100 rounded text-slate-600">
            <Italic className="w-4 h-4" />
          </button>
          <button onClick={() => handleCommand('underline')} className="p-1.5 hover:bg-slate-100 rounded text-slate-600 underline decoration-slate-600 underline-offset-2">
            U
          </button>
        </div>
        <div className="w-px h-5 bg-slate-200" />
        <div className="flex items-center gap-1">
          <button onClick={() => handleCommand('justifyLeft')} className="p-1.5 hover:bg-slate-100 rounded text-slate-600">
            <AlignLeft className="w-4 h-4" />
          </button>
          <button onClick={() => handleCommand('insertUnorderedList')} className="p-1.5 hover:bg-slate-100 rounded text-slate-600">
            <List className="w-4 h-4" />
          </button>
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Sidebar for Slides */}
        {appType === "slides" && (
          <div className="w-48 bg-slate-50 border-r border-slate-200 overflow-y-auto p-4 flex flex-col gap-4">
            {slides.map((s, idx) => (
              <div 
                key={idx} 
                onClick={() => setCurrentSlide(idx)}
                className={`aspect-video rounded border-2 overflow-hidden cursor-pointer flex flex-col items-center justify-center p-2 text-center transition-colors ${currentSlide === idx ? "border-indigo-500 bg-white" : "border-transparent bg-slate-200 hover:bg-slate-300"}`}
              >
                 <div className="text-[8px] font-bold">{s.title}</div>
                 <div className="text-[6px] text-slate-500">{s.subtitle}</div>
              </div>
            ))}
            <button className="aspect-video rounded border-2 border-dashed border-slate-300 flex items-center justify-center text-slate-400 hover:text-indigo-500 hover:border-indigo-500 hover:bg-indigo-50 transition-colors">
               <Plus className="w-6 h-6" />
            </button>
          </div>
        )}

        <div className="flex-1 bg-[#f1f5f9] overflow-y-auto p-8 flex justify-center custom-scrollbar">
          {appType === "docs" && (
            <div 
              className="w-full max-w-[800px] h-max min-h-[1056px] bg-white border border-slate-200 shadow-sm p-[1in] focus:outline-none"
              contentEditable
              dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(docContent) }}
              onBlur={(e) => setDocContent(e.currentTarget.innerHTML)}
            />
          )}

          {appType === "sheets" && (
            <div className="w-full h-full bg-white border border-slate-200 shadow-sm flex flex-col font-sans">
              <div className="flex items-center bg-slate-50 border-b border-slate-200 px-2 py-1 text-xs">
                <div className="w-10 font-bold text-center border-r border-slate-200 text-slate-500">
                  {activeCell}
                </div>
                <input
                  type="text"
                  className="flex-1 bg-transparent px-2 outline-none font-mono text-slate-700"
                  value={cells[activeCell] || ""}
                  onChange={(e) => updateCell(activeCell, e.target.value)}
                  placeholder="fx"
                />
              </div>
              <div className="flex-1 overflow-auto">
                <table className="w-full text-sm border-collapse">
                  <thead className="bg-slate-100 border-b border-slate-300 sticky top-0 z-10">
                    <tr>
                      <th className="w-12 border-r border-slate-300 p-1 bg-slate-100"></th>
                      {['A', 'B', 'C', 'D', 'E', 'F'].map(col => (
                        <th key={col} className="w-32 border-r border-slate-300 p-1 font-normal text-slate-600 bg-slate-100">
                          {col}
                        </th>
                      ))}
                    </tr>
                  </thead>
                  <tbody>
                    {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].map((row) => (
                      <tr key={row}>
                        <td className="bg-slate-100 border-r border-b border-slate-300 text-center text-xs text-slate-500 sticky left-0 z-10">
                          {row}
                        </td>
                        {['A', 'B', 'C', 'D', 'E', 'F'].map(col => {
                          const id = `${col}${row}`;
                          const rawVal = cells[id] || "";
                          const evalVal = evaluateCell(rawVal);
                          return (
                            <td
                              key={col}
                              className={`border-r border-b border-slate-200 px-2 py-1 outline-none transition-colors ${activeCell === id ? "ring-2 ring-inset ring-indigo-500 bg-indigo-50/30" : "hover:bg-slate-50"}`}
                              onClick={() => setActiveCell(id)}
                            >
                              <input 
                                className="w-full bg-transparent outline-none"
                                value={activeCell === id ? rawVal : evalVal}
                                onChange={(e) => updateCell(id, e.target.value)}
                                onFocus={() => setActiveCell(id)}
                              />
                            </td>
                          );
                        })}
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {appType === "slides" && (
            <div className="w-full flex justify-center items-center h-full">
              <div className="w-[80%] max-w-[900px] aspect-video bg-gradient-to-br from-indigo-900 to-slate-900 shadow-2xl relative flex flex-col items-center justify-center border border-slate-800 rounded-lg">
                <h1 className="text-6xl font-black text-white mb-6 drop-shadow-lg px-12 text-center" contentEditable suppressContentEditableWarning>
                  {slides[currentSlide].title}
                </h1>
                <p className="text-2xl text-indigo-300 font-light tracking-wide px-12 text-center" contentEditable suppressContentEditableWarning>
                  {slides[currentSlide].subtitle}
                </p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
