import React, { useState } from 'react';
import {
  Blocks,
  Search,
  Plus,
  Copy,
  Check,
  Cpu,
  Layers,
  Shield,
  FilePlus,
  Code2,
  Tag,
  ChevronRight,
  ExternalLink
} from 'lucide-react';
import {
  CODE_LIBRARY_MODULES,
  HARDWARE_ARCHITECTURES,
  PARADIGMS,
  HardwareArchId,
  ParadigmId,
  CodeModuleItem
} from '../data/hardwareArchitectures';

interface ModuleLibraryProps {
  onInsertCode: (code: string) => void;
  onCreateFile?: (fileName: string, content: string) => void;
}

export function ModuleLibrary({ onInsertCode, onCreateFile }: ModuleLibraryProps) {
  const [search, setSearch] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [selectedArch, setSelectedArch] = useState<HardwareArchId | 'all'>('all');
  const [selectedParadigm, setSelectedParadigm] = useState<ParadigmId | 'all'>('all');
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [insertedId, setInsertedId] = useState<string | null>(null);

  // Derive categories
  const categories = Array.from(new Set(CODE_LIBRARY_MODULES.map((m) => m.category)));

  const filteredModules = CODE_LIBRARY_MODULES.filter((m) => {
    const matchesCategory = selectedCategory === 'all' || m.category === selectedCategory;
    const matchesArch = selectedArch === 'all' || m.archId === selectedArch;
    const matchesParadigm = selectedParadigm === 'all' || m.paradigmId === selectedParadigm;
    const matchesSearch =
      search.trim() === '' ||
      m.title.toLowerCase().includes(search.toLowerCase()) ||
      m.description.toLowerCase().includes(search.toLowerCase()) ||
      m.tags.some((t) => t.toLowerCase().includes(search.toLowerCase())) ||
      m.code.toLowerCase().includes(search.toLowerCase());

    return matchesCategory && matchesArch && matchesParadigm && matchesSearch;
  });

  const handleCopy = (code: string, id: string) => {
    navigator.clipboard.writeText(code);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2500);
  };

  const handleInsert = (code: string, id: string) => {
    onInsertCode(code);
    setInsertedId(id);
    setTimeout(() => setInsertedId(null), 2500);
  };

  const handleCreateNewFile = (module: CodeModuleItem) => {
    if (onCreateFile) {
      const sanitizedName = module.title
        .toLowerCase()
        .replace(/[^a-z0-9_]/g, '_')
        .replace(/_+/g, '_') + '.lumino';
      onCreateFile(sanitizedName, module.code);
    }
  };

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0a0a0e] text-slate-200 select-none overflow-hidden">
      {/* Header */}
      <div className="p-4 md:p-5 border-b border-white/10 flex flex-wrap items-center justify-between gap-4 bg-[#111116] shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2.5 bg-gradient-to-br from-cyan-500/20 to-blue-500/20 border border-cyan-500/30 rounded-xl text-cyan-400 shadow-md">
            <Blocks className="w-6 h-6" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-lg font-bold text-white tracking-wide">
                Hardware & Kernel Code-Bibliothek
              </h2>
              <span className="text-[10px] uppercase font-bold tracking-wider px-2 py-0.5 rounded bg-cyan-500/20 border border-cyan-500/40 text-cyan-300">
                {CODE_LIBRARY_MODULES.length} Module
              </span>
            </div>
            <p className="text-xs text-slate-400 mt-0.5">
              Wiederverwendbare Kernkomponenten: Paging, GDT/IDT, Syscall Gates, HAL & IPC.
            </p>
          </div>
        </div>

        <div className="relative">
          <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
          <input
            type="text"
            placeholder="Module, Register, Trap suchen..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-56 md:w-64 bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-1.5 text-xs text-slate-200 outline-none focus:border-cyan-500/50 transition-colors"
          />
        </div>
      </div>

      {/* Filter Bar */}
      <div className="px-5 py-2.5 bg-[#0e0e13] border-b border-white/5 flex flex-wrap items-center justify-between gap-3 shrink-0 text-xs">
        {/* Category Pills */}
        <div className="flex items-center gap-1.5 overflow-x-auto py-0.5">
          <button
            onClick={() => setSelectedCategory('all')}
            className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all ${
              selectedCategory === 'all'
                ? 'bg-white/15 text-white shadow-xs font-semibold'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            Alle Kategorien
          </button>
          {categories.map((cat) => (
            <button
              key={cat}
              onClick={() => setSelectedCategory(cat)}
              className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all ${
                selectedCategory === cat
                  ? 'bg-indigo-500/20 border border-indigo-500/40 text-indigo-300 font-semibold'
                  : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
              }`}
            >
              {cat}
            </button>
          ))}
        </div>

        {/* Arch & Paradigm Filter Quick-Pills */}
        <div className="flex items-center gap-2">
          <select
            value={selectedArch}
            onChange={(e) => setSelectedArch(e.target.value as any)}
            className="bg-black/50 border border-white/10 rounded-md px-2 py-1 text-[11px] text-slate-300 outline-none focus:border-cyan-500/40"
          >
            <option value="all">Alle Architekturen</option>
            <option value="x86_64">x86_64</option>
            <option value="arm64">ARM AArch64</option>
            <option value="cortex_m">ARM Cortex-M</option>
            <option value="riscv">RISC-V</option>
          </select>

          <select
            value={selectedParadigm}
            onChange={(e) => setSelectedParadigm(e.target.value as any)}
            className="bg-black/50 border border-white/10 rounded-md px-2 py-1 text-[11px] text-slate-300 outline-none focus:border-purple-500/40"
          >
            <option value="all">Alle Paradigmen</option>
            <option value="monolith">Monolith</option>
            <option value="hierarchy">Hierarchie (Ringe)</option>
            <option value="layered">Layer (Schichten)</option>
            <option value="modular">Modular (IPC/Microkernel)</option>
          </select>
        </div>
      </div>

      {/* Modules Grid */}
      <div className="flex-1 overflow-y-auto p-4 md:p-6 bg-[#08080a]">
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 md:gap-5">
          {filteredModules.map((item) => {
            const arch = item.archId ? HARDWARE_ARCHITECTURES[item.archId] : null;
            const paradigm = item.paradigmId ? PARADIGMS[item.paradigmId] : null;

            return (
              <div
                key={item.id}
                className="bg-black/40 border border-white/10 rounded-xl p-4 flex flex-col justify-between hover:border-cyan-500/40 transition-all group shadow-md"
              >
                <div>
                  <div className="flex items-center justify-between gap-2 mb-2">
                    <span className="text-[10px] font-semibold text-slate-400 bg-white/5 px-2 py-0.5 rounded border border-white/5">
                      {item.category}
                    </span>
                    <div className="flex items-center gap-1">
                      {arch && (
                        <span className="text-[9px] font-bold px-1.5 py-0.2 rounded bg-cyan-500/10 text-cyan-300 border border-cyan-500/30">
                          {arch.shortName}
                        </span>
                      )}
                      {paradigm && (
                        <span className={`text-[9px] font-bold px-1.5 py-0.2 rounded border ${paradigm.badgeColor}`}>
                          {paradigm.id.toUpperCase()}
                        </span>
                      )}
                    </div>
                  </div>

                  <h3 className="font-bold text-sm text-slate-200 mb-1 group-hover:text-white">
                    {item.title}
                  </h3>

                  <p className="text-xs text-slate-400 leading-relaxed mb-3">
                    {item.description}
                  </p>

                  <div className="flex flex-wrap gap-1 mb-3">
                    {item.tags.map((t) => (
                      <span
                        key={t}
                        className="text-[9px] font-mono px-1.5 py-0.5 rounded bg-white/[0.03] text-slate-400 border border-white/5"
                      >
                        #{t}
                      </span>
                    ))}
                  </div>

                  {/* Code Snippet Preview */}
                  <div className="relative rounded-lg overflow-hidden border border-white/5 bg-black/70 mb-3">
                    <pre className="p-3 text-[11px] font-mono text-cyan-300/90 leading-relaxed overflow-x-auto max-h-36 whitespace-pre">
                      {item.code}
                    </pre>
                  </div>
                </div>

                {/* Actions Footer */}
                <div className="flex items-center justify-between gap-2 pt-2 border-t border-white/5">
                  <button
                    onClick={() => handleCopy(item.code, item.id)}
                    className="flex items-center gap-1 px-2.5 py-1.5 bg-white/5 hover:bg-white/10 rounded-md text-[11px] font-medium text-slate-300 transition-colors"
                  >
                    {copiedId === item.id ? (
                      <>
                        <Check className="w-3 h-3 text-emerald-400" />
                        <span className="text-emerald-300">Kopiert</span>
                      </>
                    ) : (
                      <>
                        <Copy className="w-3 h-3" />
                        <span>Kopieren</span>
                      </>
                    )}
                  </button>

                  <div className="flex items-center gap-1.5">
                    {onCreateFile && (
                      <button
                        onClick={() => handleCreateNewFile(item)}
                        className="flex items-center gap-1 px-2.5 py-1.5 bg-purple-500/15 hover:bg-purple-500/25 border border-purple-500/30 rounded-md text-[11px] font-semibold text-purple-300 transition-colors"
                        title="Als neue Datei in den Workspace anlegen"
                      >
                        <FilePlus className="w-3 h-3" />
                        <span>Neue Datei</span>
                      </button>
                    )}

                    <button
                      onClick={() => handleInsert(item.code, item.id)}
                      className="flex items-center gap-1 px-3 py-1.5 bg-cyan-600 hover:bg-cyan-500 text-black font-bold rounded-md text-[11px] shadow-sm transition-all"
                      title="Direkt in den aktiven Editor einfügen"
                    >
                      {insertedId === item.id ? (
                        <>
                          <Check className="w-3 h-3 text-black" />
                          <span>Eingefügt!</span>
                        </>
                      ) : (
                        <>
                          <Plus className="w-3 h-3 text-black" />
                          <span>Einfügen</span>
                        </>
                      )}
                    </button>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
