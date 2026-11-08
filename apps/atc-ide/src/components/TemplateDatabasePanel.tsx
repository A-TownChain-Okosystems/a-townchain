import React, { useState } from "react";
import {
  Database,
  Search,
  LayoutTemplate,
  Plus,
  Download,
  Tag,
  Clock,
  Cpu,
  Layers,
  Shield,
  Blocks,
  FileCode,
  Check,
  Copy,
  ChevronRight,
  Sparkles,
  Info,
  ExternalLink,
  Code2,
  TerminalSquare,
  Star,
  Sliders,
  Settings2,
  CheckCircle2,
  Zap,
  Bookmark,
  Workflow
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import {
  HARDWARE_ARCHITECTURES,
  PARADIGMS,
  ARCHITECTURE_TEMPLATES,
  CODE_LIBRARY_MODULES,
  RECOMMENDED_KERNEL_CONFIG,
  HardwareArchId,
  ParadigmId,
  ArchitectureTemplate,
  CodeModuleItem
} from "../data/hardwareArchitectures";

interface TemplateDatabasePanelProps {
  onLoadFiles?: (files: { name: string; content: string }[]) => void;
  onInsertCode?: (code: string) => void;
  onClose?: () => void;
  onOpenWiki?: () => void;
}

export function TemplateDatabasePanel({
  onLoadFiles,
  onInsertCode,
  onClose,
  onOpenWiki
}: TemplateDatabasePanelProps) {
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedArch, setSelectedArch] = useState<HardwareArchId | "all">("all");
  const [selectedParadigm, setSelectedParadigm] = useState<ParadigmId | "all">("all");
  const [selectedTemplateId, setSelectedTemplateId] = useState<string>(ARCHITECTURE_TEMPLATES[0].id);
  const [activeFileTab, setActiveFileTab] = useState<number>(0);
  const [copiedFile, setCopiedFile] = useState<string | null>(null);
  const [appliedTemplateId, setAppliedTemplateId] = useState<string | null>(null);
  const [showArchGuide, setShowArchGuide] = useState<boolean>(false);
  const [onlyRecommended, setOnlyRecommended] = useState<boolean>(false);
  const [viewMode, setViewMode] = useState<"templates" | "modules" | "recommended_settings">("templates");
  const [copiedConfigItem, setCopiedConfigItem] = useState<string | null>(null);

  // Filter templates
  const filteredTemplates = ARCHITECTURE_TEMPLATES.filter((t) => {
    const matchesArch = selectedArch === "all" || t.archId === selectedArch;
    const matchesParadigm = selectedParadigm === "all" || t.paradigmId === selectedParadigm;
    const matchesRecommended = !onlyRecommended || t.isRecommended === true;
    const matchesSearch =
      searchQuery.trim() === "" ||
      t.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      t.summary.toLowerCase().includes(searchQuery.toLowerCase()) ||
      t.keyFeatures.some((f) => f.toLowerCase().includes(searchQuery.toLowerCase())) ||
      t.files.some((f) => f.name.toLowerCase().includes(searchQuery.toLowerCase()));

    return matchesArch && matchesParadigm && matchesRecommended && matchesSearch;
  });

  // Filter code modules
  const filteredModules = CODE_LIBRARY_MODULES.filter((m) => {
    const matchesArch = selectedArch === "all" || !m.archId || m.archId === selectedArch;
    const matchesParadigm = selectedParadigm === "all" || !m.paradigmId || m.paradigmId === selectedParadigm;
    const matchesRecommended = !onlyRecommended || m.isRecommended === true;
    const matchesSearch =
      searchQuery.trim() === "" ||
      m.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      m.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
      m.tags.some((tag) => tag.toLowerCase().includes(searchQuery.toLowerCase()));

    return matchesArch && matchesParadigm && matchesRecommended && matchesSearch;
  });

  const currentTemplate =
    ARCHITECTURE_TEMPLATES.find((t) => t.id === selectedTemplateId) ||
    filteredTemplates[0] ||
    ARCHITECTURE_TEMPLATES[0];

  const currentArchInfo = HARDWARE_ARCHITECTURES[currentTemplate.archId];
  const currentParadigmInfo = PARADIGMS[currentTemplate.paradigmId];
  const activeFile = currentTemplate.files[activeFileTab] || currentTemplate.files[0];

  const handleCopyCode = (content: string, fileName: string) => {
    navigator.clipboard.writeText(content);
    setCopiedFile(fileName);
    setTimeout(() => setCopiedFile(null), 2500);
  };

  const handleCopyText = (text: string, identifier: string) => {
    navigator.clipboard.writeText(text);
    setCopiedConfigItem(identifier);
    setTimeout(() => setCopiedConfigItem(null), 2500);
  };

  const handleApplyTemplate = (template: ArchitectureTemplate) => {
    if (onLoadFiles) {
      onLoadFiles(
        template.files.map((f) => ({
          name: f.name,
          content: f.content
        }))
      );
      setAppliedTemplateId(template.id);
      setTimeout(() => setAppliedTemplateId(null), 3000);
    }
  };

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0a0a0d] text-slate-200 select-none overflow-hidden">
      {/* Top Header */}
      <div className="p-4 md:p-5 border-b border-white/10 flex flex-wrap items-center justify-between gap-4 bg-[#111116] shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2.5 bg-gradient-to-br from-indigo-500/20 to-purple-500/20 border border-indigo-500/30 rounded-xl text-indigo-400 shadow-md">
            <LayoutTemplate className="w-6 h-6" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-lg font-bold text-white tracking-wide">
                Hardware & Architektur Vorlagen-Hub
              </h2>
              <span className="text-[10px] uppercase font-bold tracking-wider px-2 py-0.5 rounded bg-indigo-500/20 border border-indigo-500/40 text-indigo-300">
                Multi-Arch
              </span>
              <span className="text-[10px] uppercase font-bold tracking-wider px-2 py-0.5 rounded bg-amber-500/20 border border-amber-500/40 text-amber-300 flex items-center gap-1">
                <Star className="w-3 h-3 fill-amber-400 text-amber-400" />
                Empfohlen markiert
              </span>
            </div>
            <p className="text-xs text-slate-400 mt-0.5">
              Vorgefertigte Kernels, Treiber und empfohlene Bare-Metal Einstellungen für x86_64, ARM64 & RISC-V.
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2.5 flex-wrap">
          {/* View Mode Tabs */}
          <div className="flex bg-black/40 p-1 rounded-lg border border-white/10 text-xs">
            <button
              onClick={() => setViewMode("templates")}
              className={`px-3 py-1 rounded-md font-semibold transition-all flex items-center gap-1.5 ${
                viewMode === "templates"
                  ? "bg-indigo-600 text-white shadow-sm"
                  : "text-slate-400 hover:text-white"
              }`}
            >
              <LayoutTemplate className="w-3.5 h-3.5" />
              <span>Vorlagen ({filteredTemplates.length})</span>
            </button>
            <button
              onClick={() => setViewMode("modules")}
              className={`px-3 py-1 rounded-md font-semibold transition-all flex items-center gap-1.5 ${
                viewMode === "modules"
                  ? "bg-indigo-600 text-white shadow-sm"
                  : "text-slate-400 hover:text-white"
              }`}
            >
              <Blocks className="w-3.5 h-3.5" />
              <span>Code-Bausteine ({filteredModules.length})</span>
            </button>
            <button
              onClick={() => setViewMode("recommended_settings")}
              className={`px-3 py-1 rounded-md font-semibold transition-all flex items-center gap-1.5 ${
                viewMode === "recommended_settings"
                  ? "bg-amber-500/30 text-amber-300 border border-amber-500/40 shadow-sm"
                  : "text-amber-400/80 hover:text-amber-300 hover:bg-amber-500/10"
              }`}
            >
              <Star className="w-3.5 h-3.5 fill-amber-400 text-amber-400" />
              <span>★ Empfohlene Einstellungen</span>
            </button>
          </div>

          <button
            onClick={() => setShowArchGuide(!showArchGuide)}
            className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold border transition-all ${
              showArchGuide
                ? "bg-indigo-600 text-white border-indigo-500"
                : "bg-white/5 hover:bg-white/10 text-slate-300 border-white/10"
            }`}
          >
            <Info className="w-3.5 h-3.5" />
            <span>{showArchGuide ? "Guide schließen" : "Architektur-Vergleich"}</span>
          </button>

          {onOpenWiki && (
            <button
              onClick={onOpenWiki}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-300 border border-indigo-500/40 transition-all cursor-pointer shadow-sm"
              title="Öffnet das Globus OS / ShivaCore v3 Referenzarchitektur Wiki (Formal Platform Architecture, 5 Domains, 7 Control Planes, Verträge C1-C5, Duale ABI & 19 Standards)"
            >
              <Workflow className="w-3.5 h-3.5" />
              <span>Architektur Wiki v3 (Formal Platform)</span>
            </button>
          )}

          <div className="relative">
            <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              type="text"
              placeholder="Architektur, Syscall, Paging..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-48 md:w-56 bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-1.5 text-xs text-slate-200 outline-none focus:border-indigo-500/50 transition-colors"
            />
          </div>
        </div>
      </div>

      {/* Expandable Architecture Comparison Guide */}
      <AnimatePresence>
        {showArchGuide && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: "auto" }}
            exit={{ opacity: 0, height: 0 }}
            className="bg-[#14141c] border-b border-indigo-500/20 px-6 py-4 overflow-hidden shrink-0"
          >
            <div className="grid grid-cols-1 md:grid-cols-4 gap-3 text-xs">
              {Object.values(PARADIGMS).map((p) => (
                <div
                  key={p.id}
                  onClick={() => setSelectedParadigm(p.id)}
                  className={`p-3 rounded-lg border cursor-pointer transition-all ${
                    selectedParadigm === p.id
                      ? "bg-indigo-500/20 border-indigo-500 text-white"
                      : "bg-black/30 border-white/5 hover:border-white/20 text-slate-400"
                  }`}
                >
                  <div className="font-bold text-slate-200 mb-1 flex items-center justify-between">
                    <div className="flex items-center gap-1.5">
                      {p.isRecommended && <Star className="w-3.5 h-3.5 fill-amber-400 text-amber-400" />}
                      <span>{p.germanName}</span>
                    </div>
                    <span className="text-[9px] px-1.5 py-0.5 rounded bg-white/10 font-mono">
                      {p.id.toUpperCase()}
                    </span>
                  </div>
                  <p className="text-[11px] leading-relaxed line-clamp-2 text-slate-400">{p.tagline}</p>
                </div>
              ))}
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Filter Bar with Recommended Toggle */}
      <div className="px-5 py-2.5 bg-[#0e0e13] border-b border-white/5 flex flex-wrap items-center justify-between gap-3 shrink-0 text-xs">
        {/* Recommended Toggle & Architecture Filters */}
        <div className="flex items-center gap-2 overflow-x-auto py-0.5">
          <button
            onClick={() => setOnlyRecommended(!onlyRecommended)}
            className={`px-3 py-1 rounded-md text-[11px] font-bold transition-all flex items-center gap-1.5 ${
              onlyRecommended
                ? "bg-gradient-to-r from-amber-500/30 to-yellow-500/30 text-amber-300 border border-amber-500/60 shadow-md shadow-amber-950/40 font-extrabold"
                : "bg-white/5 hover:bg-amber-500/10 text-slate-400 hover:text-amber-300 border border-white/10"
            }`}
          >
            <Star className={`w-3.5 h-3.5 ${onlyRecommended ? "fill-amber-400 text-amber-400" : "text-slate-400"}`} />
            <span>★ Nur Empfohlene</span>
          </button>

          <span className="text-slate-600">|</span>

          <span className="text-[11px] font-semibold text-slate-400 flex items-center gap-1 mr-1">
            <Cpu className="w-3.5 h-3.5 text-cyan-400" />
            Hardware:
          </span>
          <button
            onClick={() => setSelectedArch("all")}
            className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all ${
              selectedArch === "all"
                ? "bg-white/15 text-white shadow-xs font-semibold"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            Alle
          </button>
          {Object.values(HARDWARE_ARCHITECTURES).map((arch) => (
            <button
              key={arch.id}
              onClick={() => setSelectedArch(arch.id)}
              className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all flex items-center gap-1 ${
                selectedArch === arch.id
                  ? "bg-cyan-500/20 border border-cyan-500/40 text-cyan-300 font-semibold"
                  : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
              }`}
            >
              {arch.isRecommended && <Star className="w-2.5 h-2.5 fill-amber-400 text-amber-400" />}
              <span>{arch.shortName}</span>
            </button>
          ))}
        </div>

        {/* Paradigm Filters */}
        <div className="flex items-center gap-1.5 overflow-x-auto py-0.5">
          <span className="text-[11px] font-semibold text-slate-400 flex items-center gap-1 mr-1">
            <Layers className="w-3.5 h-3.5 text-purple-400" />
            Paradigma:
          </span>
          <button
            onClick={() => setSelectedParadigm("all")}
            className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all ${
              selectedParadigm === "all"
                ? "bg-white/15 text-white shadow-xs font-semibold"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            Alle
          </button>
          <button
            onClick={() => setSelectedParadigm("monolith")}
            className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all flex items-center gap-1 ${
              selectedParadigm === "monolith"
                ? "bg-blue-500/20 border border-blue-500/40 text-blue-300 font-semibold"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            <Star className="w-2.5 h-2.5 fill-amber-400 text-amber-400" />
            <span>Monolith</span>
          </button>
          <button
            onClick={() => setSelectedParadigm("hierarchy")}
            className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all ${
              selectedParadigm === "hierarchy"
                ? "bg-amber-500/20 border border-amber-500/40 text-amber-300 font-semibold"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            Hierarchie (Ringe)
          </button>
          <button
            onClick={() => setSelectedParadigm("layered")}
            className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all flex items-center gap-1 ${
              selectedParadigm === "layered"
                ? "bg-emerald-500/20 border border-emerald-500/40 text-emerald-300 font-semibold"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            <Star className="w-2.5 h-2.5 fill-amber-400 text-amber-400" />
            <span>Layer (Schichten)</span>
          </button>
          <button
            onClick={() => setSelectedParadigm("modular")}
            className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all ${
              selectedParadigm === "modular"
                ? "bg-purple-500/20 border border-purple-500/40 text-purple-300 font-semibold"
                : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
            }`}
          >
            Modular (Microkernel)
          </button>
        </div>
      </div>

      {/* Main Split Body: Left List + Right Inspector OR Modules View OR Recommended Settings */}
      {viewMode === "recommended_settings" ? (
        /* Dedicated Recommended Settings View */
        <div className="flex-1 overflow-y-auto p-5 md:p-8 bg-[#07070a] space-y-6">
          {/* Header Banner */}
          <div className="p-6 rounded-2xl bg-gradient-to-r from-amber-500/15 via-yellow-500/10 to-transparent border border-amber-500/40 relative overflow-hidden">
            <div className="absolute right-6 top-6 opacity-10 pointer-events-none">
              <Star className="w-48 h-48 text-amber-400" />
            </div>
            <div className="flex items-center gap-2 mb-2">
              <span className="px-2.5 py-0.5 rounded-full text-[10px] font-extrabold uppercase tracking-wider bg-amber-500/20 text-amber-300 border border-amber-500/50 flex items-center gap-1">
                <Star className="w-3 h-3 fill-amber-400 text-amber-400" />
                GOLD STANDARD EMPFEHLUNG
              </span>
              <span className="text-xs text-slate-400">Für Betriebssystem-Entwicklung from Scratch (Bare-Metal)</span>
            </div>
            <h2 className="text-2xl font-black text-white tracking-wide">
              {RECOMMENDED_KERNEL_CONFIG.title}
            </h2>
            <p className="text-sm text-slate-300 mt-1.5 max-w-3xl leading-relaxed">
              {RECOMMENDED_KERNEL_CONFIG.subtitle}
            </p>
          </div>

          {/* Grid: Recommended Paradigm & Arch Cards */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {/* x86_64 Monolith */}
            <div className="p-5 rounded-xl bg-[#0f0f15] border border-amber-500/30 relative">
              <div className="flex items-center justify-between mb-3">
                <span className="px-2 py-0.5 rounded text-xs font-bold bg-cyan-500/20 text-cyan-300 border border-cyan-500/40">
                  x86_64 Monolith
                </span>
                <span className="text-[11px] font-bold text-amber-400 flex items-center gap-1">
                  <Star className="w-3 h-3 fill-amber-400" /> Linux-Style
                </span>
              </div>
              <h3 className="text-base font-bold text-white mb-1.5">Intel/AMD 64-Bit Monolith Kernel</h3>
              <p className="text-xs text-slate-400 leading-relaxed mb-4">
                Beste Dokumentation weltweit (OSDev, Intel SDM). Ring 0 gewährt direkten Speicher- und Portzugriff, 4-Level Paging schützt User- und Kernel-Space.
              </p>
              <div className="space-y-1.5 text-xs text-slate-300 mb-4 font-mono">
                <div className="flex items-center gap-2">
                  <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                  <span>Entry: Multiboot2 / Limine (Direkter 64-Bit Boot)</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                  <span>Higher-Half: 0xFFFF_FFFF_8000_0000</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                  <span>Interrupts: APIC & IDT mit 256 Vektoren</span>
                </div>
              </div>
              <button
                onClick={() => {
                  setSelectedTemplateId("x86-monolith-core");
                  setViewMode("templates");
                }}
                className="w-full py-2 bg-gradient-to-r from-amber-600 to-amber-700 hover:from-amber-500 hover:to-amber-600 text-white font-bold rounded-lg text-xs transition-all shadow-md flex items-center justify-center gap-1.5"
              >
                <span>x86_64 Template anzeigen & laden</span>
                <ChevronRight className="w-3.5 h-3.5" />
              </button>
            </div>

            {/* ARM64 Layered */}
            <div className="p-5 rounded-xl bg-[#0f0f15] border border-cyan-500/30 relative">
              <div className="flex items-center justify-between mb-3">
                <span className="px-2 py-0.5 rounded text-xs font-bold bg-purple-500/20 text-purple-300 border border-purple-500/40">
                  ARM64 Schichten (Layered)
                </span>
                <span className="text-[11px] font-bold text-cyan-400 flex items-center gap-1">
                  <Star className="w-3 h-3 fill-cyan-400" /> NT/XNU-Style
                </span>
              </div>
              <h3 className="text-base font-bold text-white mb-1.5">ARM AArch64 HAL & Executive</h3>
              <p className="text-xs text-slate-400 leading-relaxed mb-4">
                Hervorragende Trennung durch Hardware Abstraction Layer (HAL) wie in Windows NT. Hohe Portabilität zwischen Apple Silicon, Raspberry Pi und Graviton-Servern.
              </p>
              <div className="space-y-1.5 text-xs text-slate-300 mb-4 font-mono">
                <div className="flex items-center gap-2">
                  <CheckCircle2 className="w-3.5 h-3.5 text-cyan-400 shrink-0" />
                  <span>EL1 Kernel & EL0 Isolated Userland</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle2 className="w-3.5 h-3.5 text-cyan-400 shrink-0" />
                  <span>Split TTBR0 (User) & TTBR1 (Kernel) MMU</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle2 className="w-3.5 h-3.5 text-cyan-400 shrink-0" />
                  <span>GICv3 Interrupt Distributor</span>
                </div>
              </div>
              <button
                onClick={() => {
                  setSelectedTemplateId("arm64-layered-arch");
                  setViewMode("templates");
                }}
                className="w-full py-2 bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white font-bold rounded-lg text-xs transition-all shadow-md flex items-center justify-center gap-1.5"
              >
                <span>ARM64 Layered Template anzeigen & laden</span>
                <ChevronRight className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          {/* Essential Compiler Settings Table */}
          <div className="p-5 rounded-xl bg-[#0e0e14] border border-white/10">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-2">
                <Sliders className="w-4 h-4 text-amber-400" />
                <h3 className="text-sm font-bold text-white uppercase tracking-wider">
                  Empfohlene Compiler-Flags (Freestanding Bare-Metal)
                </h3>
              </div>
              <button
                onClick={() =>
                  handleCopyText(
                    RECOMMENDED_KERNEL_CONFIG.compilerFlags.map((c) => c.flag).join(" "),
                    "all-flags"
                  )
                }
                className="flex items-center gap-1.5 px-3 py-1 bg-white/5 hover:bg-white/10 border border-white/10 rounded-md text-xs font-mono text-slate-300 transition-all"
              >
                {copiedConfigItem === "all-flags" ? (
                  <>
                    <Check className="w-3.5 h-3.5 text-emerald-400" />
                    <span className="text-emerald-400 font-bold">Kopiert!</span>
                  </>
                ) : (
                  <>
                    <Copy className="w-3.5 h-3.5" />
                    <span>Alle Flags kopieren</span>
                  </>
                )}
              </button>
            </div>

            <div className="overflow-x-auto">
              <table className="w-full text-left text-xs border-collapse">
                <thead>
                  <tr className="border-b border-white/10 text-slate-400 text-[11px] font-mono">
                    <th className="pb-2 pl-2">Flag / Argument</th>
                    <th className="pb-2">Typ</th>
                    <th className="pb-2">Erklärung & Sicherheitszweck</th>
                    <th className="pb-2 text-right pr-2">Aktion</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-white/5">
                  {RECOMMENDED_KERNEL_CONFIG.compilerFlags.map((cf) => (
                    <tr key={cf.flag} className="hover:bg-white/[0.02]">
                      <td className="py-2.5 pl-2 font-mono font-bold text-amber-300">
                        {cf.flag}
                      </td>
                      <td className="py-2.5 font-mono text-slate-400">
                        <span
                          className={`px-1.5 py-0.5 rounded text-[10px] font-bold ${
                            cf.isEssential
                              ? "bg-amber-500/20 text-amber-300 border border-amber-500/30"
                              : "bg-white/5 text-slate-300"
                          }`}
                        >
                          {cf.value}
                        </span>
                      </td>
                      <td className="py-2.5 text-slate-300">{cf.description}</td>
                      <td className="py-2.5 text-right pr-2">
                        <button
                          onClick={() => handleCopyText(cf.flag, cf.flag)}
                          className="px-2 py-0.5 rounded bg-white/5 hover:bg-white/10 text-[11px] font-mono text-slate-300 hover:text-white"
                        >
                          {copiedConfigItem === cf.flag ? "✓" : "Kopieren"}
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Linker Script & Emulator Command */}
          <div className="grid grid-cols-1 xl:grid-cols-2 gap-4">
            {/* Linker Script Blueprint */}
            <div className="p-5 rounded-xl bg-[#0e0e14] border border-white/10">
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center gap-2">
                  <FileCode className="w-4 h-4 text-cyan-400" />
                  <h3 className="text-sm font-bold text-white">Empfohlenes Linker-Script (Higher-Half)</h3>
                </div>
                <button
                  onClick={() =>
                    handleCopyText(RECOMMENDED_KERNEL_CONFIG.linkerScriptExcerpt, "linker-script")
                  }
                  className="px-2.5 py-1 bg-white/5 hover:bg-white/10 rounded text-xs font-mono text-slate-300 flex items-center gap-1"
                >
                  {copiedConfigItem === "linker-script" ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  <span>{copiedConfigItem === "linker-script" ? "Kopiert!" : "Kopieren"}</span>
                </button>
              </div>
              <pre className="text-[11px] font-mono text-cyan-300 bg-black/60 p-3 rounded-lg border border-white/5 overflow-x-auto leading-relaxed">
                {RECOMMENDED_KERNEL_CONFIG.linkerScriptExcerpt}
              </pre>
            </div>

            {/* Emulator & Verification */}
            <div className="p-5 rounded-xl bg-[#0e0e14] border border-white/10">
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center gap-2">
                  <TerminalSquare className="w-4 h-4 text-emerald-400" />
                  <h3 className="text-sm font-bold text-white">QEMU Bare-Metal Startbefehl</h3>
                </div>
                <button
                  onClick={() =>
                    handleCopyText(RECOMMENDED_KERNEL_CONFIG.qemuRunCommand, "qemu-cmd")
                  }
                  className="px-2.5 py-1 bg-white/5 hover:bg-white/10 rounded text-xs font-mono text-slate-300 flex items-center gap-1"
                >
                  {copiedConfigItem === "qemu-cmd" ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  <span>{copiedConfigItem === "qemu-cmd" ? "Kopiert!" : "Kopieren"}</span>
                </button>
              </div>
              <div className="p-3 bg-black/60 rounded-lg border border-white/5 font-mono text-xs text-emerald-300 mb-3 break-all">
                {RECOMMENDED_KERNEL_CONFIG.qemuRunCommand}
              </div>
              <div className="text-xs text-slate-400 leading-relaxed space-y-1.5">
                <p>
                  <strong>-kernel:</strong> Lädt das kompilierte ELF-Kernelimage direkt ohne MBR-Bootsektoren.
                </p>
                <p>
                  <strong>-serial stdio:</strong> Leitet COM1 / UART serielle Kernel-Printfs direkt in dein Terminal weiter.
                </p>
                <p>
                  <strong>-no-reboot:</strong> Friert die VM bei Triple-Faults zur Fehleranalyse mit CPU-Dump ein.
                </p>
              </div>
            </div>
          </div>
        </div>
      ) : viewMode === "modules" ? (
        /* Code Library Building Blocks View */
        <div className="flex-1 overflow-y-auto p-5 md:p-8 bg-[#07070a] space-y-4">
          <div className="flex items-center justify-between gap-4 flex-wrap pb-2 border-b border-white/10">
            <div>
              <h2 className="text-lg font-bold text-white flex items-center gap-2">
                <Blocks className="w-5 h-5 text-indigo-400" />
                <span>Modulare Kernel-Bausteine ({filteredModules.length})</span>
              </h2>
              <p className="text-xs text-slate-400 mt-0.5">
                Fertige, auditierte Code-Module für Deskriptortabellen, Syscalls, Paging und Treiber.
              </p>
            </div>
          </div>

          <div className="grid grid-cols-1 xl:grid-cols-2 gap-4">
            {filteredModules.map((module) => (
              <div
                key={module.id}
                className={`p-4 rounded-xl border transition-all ${
                  module.isRecommended
                    ? "bg-[#0e0e15] border-amber-500/40 shadow-lg shadow-amber-950/20"
                    : "bg-[#0c0c10] border-white/10 hover:border-white/20"
                }`}
              >
                <div className="flex items-start justify-between gap-2 mb-2">
                  <div>
                    {module.isRecommended && (
                      <span className="inline-flex items-center gap-1 text-[10px] font-extrabold px-2 py-0.5 rounded bg-amber-500/20 border border-amber-500/40 text-amber-300 mb-1.5">
                        <Star className="w-3 h-3 fill-amber-400 text-amber-400" />
                        {module.recommendedBadge || "EMPFOHLEN"}
                      </span>
                    )}
                    <h3 className="font-bold text-sm text-white">{module.title}</h3>
                  </div>
                  <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-white/5 text-slate-400 shrink-0">
                    {module.category}
                  </span>
                </div>

                <p className="text-xs text-slate-300 mb-2 leading-relaxed">
                  {module.description}
                </p>

                {module.recommendedReason && (
                  <div className="p-2 rounded bg-amber-500/10 border border-amber-500/20 text-[11px] text-amber-200/90 mb-3">
                    💡 <strong>Warum empfohlen:</strong> {module.recommendedReason}
                  </div>
                )}

                <div className="flex flex-wrap gap-1 mb-3">
                  {module.tags.map((tag) => (
                    <span
                      key={tag}
                      className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-white/5 text-slate-400"
                    >
                      #{tag}
                    </span>
                  ))}
                </div>

                <pre className="text-[11px] font-mono text-slate-300 bg-black/60 p-3 rounded-lg border border-white/5 overflow-x-auto max-h-48 whitespace-pre leading-relaxed mb-3">
                  {module.code}
                </pre>

                <div className="flex items-center justify-end gap-2">
                  <button
                    onClick={() => handleCopyText(module.code, module.id)}
                    className="flex items-center gap-1 px-3 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 border border-white/10 text-xs font-semibold text-slate-300"
                  >
                    {copiedConfigItem === module.id ? (
                      <>
                        <Check className="w-3.5 h-3.5 text-emerald-400" />
                        <span className="text-emerald-300 font-bold">Kopiert!</span>
                      </>
                    ) : (
                      <>
                        <Copy className="w-3.5 h-3.5" />
                        <span>Code kopieren</span>
                      </>
                    )}
                  </button>

                  {onInsertCode && (
                    <button
                      onClick={() => onInsertCode(module.code)}
                      className="flex items-center gap-1 px-3 py-1.5 rounded-lg bg-indigo-500/20 hover:bg-indigo-500/30 border border-indigo-500/40 text-xs font-semibold text-indigo-300"
                    >
                      <Code2 className="w-3.5 h-3.5" />
                      <span>In Editor einfügen</span>
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      ) : (
        /* Template View: Left List + Right Inspector */
        <div className="flex-1 flex flex-col lg:flex-row overflow-hidden">
          {/* Left: Template Cards List */}
          <div className="w-full lg:w-[380px] xl:w-[420px] border-r border-white/5 overflow-y-auto p-4 space-y-3 shrink-0 bg-[#0d0d11]">
            <div className="text-[11px] font-bold uppercase tracking-wider text-slate-500 mb-2 flex items-center justify-between">
              <span>Verfügbare Vorlagen ({filteredTemplates.length})</span>
              <span className="font-normal lowercase text-[10px]">Klick zur Vorschau</span>
            </div>

            {filteredTemplates.length === 0 ? (
              <div className="text-center py-12 px-4 bg-white/[0.02] rounded-xl border border-white/5">
                <Cpu className="w-8 h-8 text-slate-600 mx-auto mb-2" />
                <div className="text-sm font-semibold text-slate-400">Keine Vorlagen gefunden</div>
                <p className="text-xs text-slate-500 mt-1">
                  Passe die Filter oder den Suchbegriff an.
                </p>
              </div>
            ) : (
              filteredTemplates.map((template) => {
                const isSelected = template.id === currentTemplate.id;
                const arch = HARDWARE_ARCHITECTURES[template.archId];
                const paradigm = PARADIGMS[template.paradigmId];

                return (
                  <div
                    key={template.id}
                    onClick={() => {
                      setSelectedTemplateId(template.id);
                      setActiveFileTab(0);
                    }}
                    className={`p-3.5 rounded-xl border cursor-pointer transition-all relative ${
                      isSelected
                        ? "bg-indigo-500/10 border-indigo-500/50 shadow-lg shadow-indigo-950/40"
                        : template.isRecommended
                        ? "bg-[#101017] border-amber-500/30 hover:border-amber-500/50"
                        : "bg-black/30 border-white/5 hover:border-white/15 hover:bg-white/[0.02]"
                    }`}
                  >
                    {template.isRecommended && (
                      <div className="mb-2 px-2 py-0.5 rounded bg-gradient-to-r from-amber-500/20 to-yellow-500/15 border border-amber-500/40 text-amber-300 text-[10px] font-extrabold flex items-center gap-1">
                        <Star className="w-3 h-3 fill-amber-400 text-amber-400 shrink-0" />
                        <span>{template.recommendedBadge || "★ TOP-EMPFEHLUNG"}</span>
                      </div>
                    )}

                    <div className="flex items-center justify-between gap-2 mb-2">
                      <span className="text-[10px] font-bold px-2 py-0.5 rounded bg-cyan-500/10 border border-cyan-500/30 text-cyan-300">
                        {arch.shortName}
                      </span>
                      <span className={`text-[10px] font-bold px-2 py-0.5 rounded border ${paradigm.badgeColor}`}>
                        {paradigm.germanName}
                      </span>
                    </div>

                    <h3 className="font-bold text-sm text-slate-200 mb-1 leading-snug group-hover:text-white">
                      {template.title}
                    </h3>

                    <p className="text-xs text-slate-400 line-clamp-2 leading-relaxed mb-3">
                      {template.summary}
                    </p>

                    <div className="flex items-center justify-between text-[11px] text-slate-500 pt-2 border-t border-white/5">
                      <span className="flex items-center gap-1 font-mono text-[10px]">
                        <FileCode className="w-3 h-3 text-slate-400" />
                        {template.files.length} Dateien
                      </span>
                      <span className="text-indigo-400 font-semibold flex items-center gap-0.5 hover:underline text-[11px]">
                        Details <ChevronRight className="w-3.5 h-3.5" />
                      </span>
                    </div>
                  </div>
                );
              })
            )}
          </div>

          {/* Right: Selected Template Inspector & Multi-File Code Viewer */}
          <div className="flex-1 flex flex-col overflow-hidden bg-[#08080a]">
            {/* Template Info Header */}
            <div className="p-4 md:p-6 border-b border-white/10 bg-[#111116] shrink-0">
              {/* Highlight Box if Recommended */}
              {currentTemplate.isRecommended && (
                <div className="mb-4 p-3.5 rounded-xl bg-gradient-to-r from-amber-500/15 via-yellow-500/10 to-transparent border border-amber-500/40 text-xs">
                  <div className="flex items-center gap-2 font-bold text-amber-300">
                    <Star className="w-4 h-4 fill-amber-400 text-amber-400 shrink-0" />
                    <span>{currentTemplate.recommendedBadge || "Empfohlene Architektur"}</span>
                  </div>
                  <p className="text-slate-200 text-xs mt-1 leading-relaxed">
                    {currentTemplate.recommendedReason}
                  </p>
                </div>
              )}

              <div className="flex flex-wrap items-start justify-between gap-4">
                <div>
                  <div className="flex items-center gap-2 mb-1.5 flex-wrap">
                    <span className="text-xs font-bold px-2.5 py-0.5 rounded-full bg-cyan-500/20 border border-cyan-500/40 text-cyan-300">
                      {currentArchInfo.name}
                    </span>
                    <span className="text-xs font-bold px-2.5 py-0.5 rounded-full bg-purple-500/20 border border-purple-500/40 text-purple-300">
                      {currentParadigmInfo.germanName}
                    </span>
                    <span className="text-xs font-mono text-slate-500">
                      {currentTemplate.version}
                    </span>
                  </div>
                  <h2 className="text-xl font-extrabold text-white">
                    {currentTemplate.title}
                  </h2>
                  <p className="text-xs text-slate-400 mt-1 max-w-2xl leading-relaxed">
                    {currentTemplate.summary}
                  </p>
                </div>

                {/* Action Buttons */}
                <div className="flex items-center gap-2 flex-wrap">
                  <button
                    onClick={() => handleCopyCode(activeFile.content, activeFile.name)}
                    className="flex items-center gap-1.5 px-3 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-xs font-semibold text-slate-300 transition-all"
                    title="Code in Zwischenablage kopieren"
                  >
                    {copiedFile === activeFile.name ? (
                      <>
                        <Check className="w-3.5 h-3.5 text-emerald-400" />
                        <span className="text-emerald-300">Kopiert!</span>
                      </>
                    ) : (
                      <>
                        <Copy className="w-3.5 h-3.5" />
                        <span>Code kopieren</span>
                      </>
                    )}
                  </button>

                  {onInsertCode && (
                    <button
                      onClick={() => onInsertCode(activeFile.content)}
                      className="flex items-center gap-1.5 px-3 py-2 bg-indigo-500/20 hover:bg-indigo-500/30 border border-indigo-500/40 rounded-lg text-xs font-semibold text-indigo-300 transition-all"
                      title="Code in die aktive Editor-Datei einfügen"
                    >
                      <Code2 className="w-3.5 h-3.5" />
                      <span>In Editor einfügen</span>
                    </button>
                  )}

                  {onLoadFiles && (
                    <button
                      onClick={() => handleApplyTemplate(currentTemplate)}
                      className="flex items-center gap-1.5 px-4 py-2 bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white rounded-lg text-xs font-bold shadow-lg shadow-emerald-950/40 transition-all"
                      title="Gesamtes Template mit allen Dateien in den Workspace laden"
                    >
                      {appliedTemplateId === currentTemplate.id ? (
                        <>
                          <Check className="w-4 h-4 text-white" />
                          <span>Geladen!</span>
                        </>
                      ) : (
                        <>
                          <Download className="w-4 h-4" />
                          <span>In Workspace laden ({currentTemplate.files.length} Dateien)</span>
                        </>
                      )}
                    </button>
                  )}
                </div>
              </div>

              {/* Recommended Settings Table if present on this template */}
              {currentTemplate.recommendedSettings && (
                <div className="mt-4 pt-3 border-t border-white/10">
                  <div className="text-xs font-bold text-amber-300 mb-2 flex items-center gap-1.5">
                    <Sliders className="w-3.5 h-3.5" />
                    <span>Empfohlene Kernel- & Compiler-Einstellungen für {currentArchInfo.shortName}</span>
                  </div>
                  <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-2">
                    {currentTemplate.recommendedSettings.map((s) => (
                      <div
                        key={s.flag}
                        className="p-2 rounded bg-black/40 border border-white/5 flex items-start justify-between gap-2"
                      >
                        <div>
                          <span className="text-[10px] text-slate-500 font-mono block">{s.category}</span>
                          <span className="text-xs font-mono font-bold text-cyan-300 block">{s.flag}</span>
                          <span className="text-[10px] text-slate-400 block mt-0.5">{s.description}</span>
                        </div>
                        <button
                          onClick={() => handleCopyText(s.flag, s.flag)}
                          className="px-1.5 py-0.5 rounded bg-white/5 hover:bg-white/10 text-[10px] font-mono text-slate-400 hover:text-white shrink-0"
                        >
                          {copiedConfigItem === s.flag ? "✓" : "Copy"}
                        </button>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Architecture Specs Badges */}
              <div className="grid grid-cols-2 md:grid-cols-4 gap-3 mt-4 pt-3 border-t border-white/5 text-[11px]">
                <div className="bg-black/40 p-2.5 rounded-lg border border-white/5">
                  <span className="text-slate-500 block text-[10px] uppercase font-bold">Privilege-Stufen</span>
                  <span className="font-semibold text-slate-200 truncate block mt-0.5">
                    {currentArchInfo.privilegeLevels[0]}
                  </span>
                </div>
                <div className="bg-black/40 p-2.5 rounded-lg border border-white/5">
                  <span className="text-slate-500 block text-[10px] uppercase font-bold">Paging / Speicher</span>
                  <span className="font-semibold text-slate-200 truncate block mt-0.5">
                    {currentArchInfo.pagingModel}
                  </span>
                </div>
                <div className="bg-black/40 p-2.5 rounded-lg border border-white/5">
                  <span className="text-slate-500 block text-[10px] uppercase font-bold">Interrupt Controller</span>
                  <span className="font-semibold text-slate-200 truncate block mt-0.5">
                    {currentArchInfo.interruptController}
                  </span>
                </div>
                <div className="bg-black/40 p-2.5 rounded-lg border border-white/5">
                  <span className="text-slate-500 block text-[10px] uppercase font-bold">Kern-Vorteil</span>
                  <span className="font-semibold text-emerald-400 truncate block mt-0.5">
                    {currentParadigmInfo.pros[0]}
                  </span>
                </div>
              </div>
            </div>

            {/* Code Viewer Tab Bar */}
            <div className="bg-[#0f0f14] px-4 py-2 border-b border-white/5 flex items-center justify-between gap-3 shrink-0">
              <div className="flex items-center gap-1.5 overflow-x-auto">
                <span className="text-[11px] font-bold text-slate-500 uppercase tracking-wider mr-1">
                  Dateien:
                </span>
                {currentTemplate.files.map((file, idx) => (
                  <button
                    key={file.name}
                    onClick={() => setActiveFileTab(idx)}
                    className={`px-3 py-1.5 rounded-md text-xs font-mono font-medium transition-all flex items-center gap-2 ${
                      activeFileTab === idx
                        ? "bg-indigo-500/20 text-indigo-300 border border-indigo-500/40 shadow-xs"
                        : "text-slate-400 hover:text-slate-200 hover:bg-white/5"
                    }`}
                  >
                    <FileCode className="w-3.5 h-3.5 text-cyan-400" />
                    <span>{file.name}</span>
                  </button>
                ))}
              </div>

              <div className="text-[11px] text-slate-500 font-mono hidden md:block">
                {activeFile.description}
              </div>
            </div>

            {/* Code Editor Preview Area */}
            <div className="flex-1 overflow-y-auto p-4 md:p-6 font-mono text-xs bg-[#050507]">
              <div className="bg-black/60 border border-white/10 rounded-xl p-4 md:p-5 relative shadow-2xl">
                <div className="flex items-center justify-between pb-3 mb-3 border-b border-white/10 text-[11px] text-slate-400">
                  <div className="flex items-center gap-2">
                    <span className="w-2.5 h-2.5 rounded-full bg-red-500/80"></span>
                    <span className="w-2.5 h-2.5 rounded-full bg-yellow-500/80"></span>
                    <span className="w-2.5 h-2.5 rounded-full bg-green-500/80"></span>
                    <span className="text-slate-300 font-bold ml-2">{activeFile.name}</span>
                  </div>
                  <span className="text-slate-500">{activeFile.content.split("\n").length} Zeilen</span>
                </div>

                <pre className="text-slate-200 leading-relaxed overflow-x-auto whitespace-pre font-mono selection:bg-indigo-500/40">
                  {activeFile.content}
                </pre>
              </div>

              {/* Memory Layout & Architecture Diagram Card */}
              <div className="mt-6 grid grid-cols-1 xl:grid-cols-2 gap-4">
                <div className="bg-black/40 border border-white/10 rounded-xl p-4">
                  <div className="text-xs font-bold text-slate-300 mb-2 flex items-center gap-2">
                    <TerminalSquare className="w-4 h-4 text-cyan-400" />
                    <span>Speicherlayout & Adressraum</span>
                  </div>
                  <pre className="text-[11px] text-cyan-300 font-mono bg-black/60 p-3 rounded-lg border border-white/5 whitespace-pre-wrap leading-relaxed">
                    {currentTemplate.memoryLayout}
                  </pre>
                </div>

                <div className="bg-black/40 border border-white/10 rounded-xl p-4">
                  <div className="text-xs font-bold text-slate-300 mb-2 flex items-center gap-2">
                    <Layers className="w-4 h-4 text-purple-400" />
                    <span>Architektur-Diagramm ({currentParadigmInfo.germanName})</span>
                  </div>
                  <pre className="text-[10px] text-purple-300 font-mono bg-black/60 p-3 rounded-lg border border-white/5 whitespace-pre overflow-x-auto leading-tight">
                    {currentParadigmInfo.diagram}
                  </pre>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
