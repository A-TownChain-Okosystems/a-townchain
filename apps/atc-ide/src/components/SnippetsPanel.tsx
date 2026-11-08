import React, { useState } from 'react';
import { Scissors, Copy, Check, Plus, Search, Tag, Cpu, Layers } from 'lucide-react';

interface SnippetItem {
  id: string;
  name: string;
  category: string;
  arch?: string;
  paradigm?: string;
  code: string;
}

const SNIPPETS: SnippetItem[] = [
  {
    id: "snip-x86-longmode",
    name: "x86_64 Long Mode GDT Setup",
    category: "x86_64",
    arch: "x86_64",
    paradigm: "Monolith",
    code: `// x86_64 64-Bit GDT Table
let GDT_KERNEL_CODE = 0x08
let GDT_KERNEL_DATA = 0x10
let GDT_USER_DATA   = 0x23
let GDT_USER_CODE   = 0x2B

func reloadSegmentRegisters() {
  print "[x86_64] Lade CS=0x08, DS=0x10, SS=0x10"
}`
  },
  {
    id: "snip-x86-pml4",
    name: "x86_64 4-Level Page Directory Entry",
    category: "x86_64",
    arch: "x86_64",
    paradigm: "Monolith",
    code: `// PML4 / PDPT Page Table Entry
func createPte(physAddr: number, isWritable: boolean, isUser: boolean) {
  let flags = 0x01 // Present
  if isWritable { flags = flags | 0x02 }
  if isUser { flags = flags | 0x04 }
  return (physAddr & 0x000FFFFFFFFFF000) | flags
}`
  },
  {
    id: "snip-arm64-vbar",
    name: "ARM64 Vector Base Address Register",
    category: "ARM64",
    arch: "AArch64",
    paradigm: "Layered",
    code: `// ARM64 VBAR_EL1 Setup
func setVbarEl1(vbarAddress: number) {
  print "[AArch64] msr vbar_el1, " + vbarAddress
  print "[AArch64] isb (Instruction Synchronization Barrier)"
}`
  },
  {
    id: "snip-arm64-ttbr",
    name: "ARM64 TTBR0 & TTBR1 Translation Switch",
    category: "ARM64",
    arch: "AArch64",
    paradigm: "Layered",
    code: `// ARM64 Split Virtual Address Translation
func switchProcessAddressSpace(userTtbr0: number, asid: number) {
  print "[AArch64] msr ttbr0_el1, " + (userTtbr0 | (asid << 48))
  print "[AArch64] tlbi vmalle1is (Flush TLB across cores)"
}`
  },
  {
    id: "snip-cortex-pendsv",
    name: "Cortex-M PendSV Trigger Context Switch",
    category: "Cortex-M",
    arch: "Cortex-M",
    paradigm: "Layered",
    code: `// ARM Cortex-M PendSV Trigger
let NVIC_ICSR = 0xE000ED04
let ICSR_PENDSVSET = (1 << 28)

func triggerPendSV() {
  print "[Cortex-M] Setze ICSR.PENDSVSET Bit fuer Taskwechsel"
}`
  },
  {
    id: "snip-riscv-satp",
    name: "RISC-V satp Register Page Table Activation",
    category: "RISC-V",
    arch: "RISC-V",
    paradigm: "Modular",
    code: `// RISC-V Sv39 Page Table Activation
func writeSatp(mode: number, asid: number, rootPpn: number) {
  let satpVal = (mode << 60) | (asid << 44) | rootPpn
  print "[RISC-V] csrw satp, " + satpVal
  print "[RISC-V] sfence.vma zero, zero"
}`
  },
  {
    id: "snip-mono-syscall",
    name: "Monolith Kernel Syscall Dispatch Table",
    category: "Paradigmen",
    paradigm: "Monolith",
    code: `// Monolith Fast Syscall Dispatcher Table
let syscallTable = {
  1: func(fd, buf, len) { print "[SYS_WRITE] " + buf },
  2: func(fd, buf, len) { print "[SYS_READ] FD " + fd },
  3: func() { print "[SYS_GETPID] PID: 100" }
}`
  },
  {
    id: "snip-hier-iopb",
    name: "Hierarchische I/O Permission Bitmap (IOPB)",
    category: "Paradigmen",
    paradigm: "Hierarchie",
    code: `// TSS I/O Permission Bitmap Check
func isPortAllowedInRing1(portNumber: number, iopbArray: any) {
  let byteIndex = portNumber >> 3
  let bitIndex = portNumber & 7
  return ((iopbArray[byteIndex] >> bitIndex) & 1) == 0
}`
  },
  {
    id: "snip-layer-hal",
    name: "Schichtenmodell: HAL Register Abstraction",
    category: "Paradigmen",
    paradigm: "Layered",
    code: `// Layer 0 Hardware Abstraction Layer
struct HalRegisterBank {
  base: number,
  read: (offset: number) => number,
  write: (offset: number, val: number) => void
}`
  },
  {
    id: "snip-mod-ipc",
    name: "Microkernel IPC Message Envelope",
    category: "Paradigmen",
    paradigm: "Modular",
    code: `// Modulares Microkernel IPC Message Envelope
struct IPCMessageHeader {
  senderId: number,
  receiverId: number,
  methodId: number,
  grantCapabilityToken: number,
  payloadLength: number
}`
  }
];

interface SnippetsPanelProps {
  onInsertCode?: (code: string) => void;
}

export function SnippetsPanel({ onInsertCode }: SnippetsPanelProps) {
  const [search, setSearch] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [insertedId, setInsertedId] = useState<string | null>(null);

  const categories = Array.from(new Set(SNIPPETS.map((s) => s.category)));

  const filteredSnippets = SNIPPETS.filter((s) => {
    const matchesCat = selectedCategory === 'all' || s.category === selectedCategory;
    const matchesSearch =
      search.trim() === '' ||
      s.name.toLowerCase().includes(search.toLowerCase()) ||
      s.code.toLowerCase().includes(search.toLowerCase()) ||
      (s.arch && s.arch.toLowerCase().includes(search.toLowerCase())) ||
      (s.paradigm && s.paradigm.toLowerCase().includes(search.toLowerCase()));
    return matchesCat && matchesSearch;
  });

  const handleCopy = (code: string, id: string) => {
    navigator.clipboard.writeText(code);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  const handleInsert = (code: string, id: string) => {
    if (onInsertCode) {
      onInsertCode(code);
      setInsertedId(id);
      setTimeout(() => setInsertedId(null), 2000);
    }
  };

  return (
    <div className="flex-1 flex flex-col bg-[#0a0a0e] text-slate-200 font-sans select-none overflow-hidden h-full">
      {/* Top Header */}
      <div className="px-6 py-4 border-b border-white/10 flex flex-wrap items-center justify-between gap-4 bg-[#111116] shrink-0">
        <div className="flex items-center gap-3">
          <div className="p-2.5 bg-amber-500/15 border border-amber-500/30 rounded-xl text-amber-400 shadow-sm">
            <Scissors className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-lg font-bold text-white tracking-wide">
                Hardware & Kernel Code-Snippets
              </h2>
              <span className="text-[10px] uppercase font-bold px-2 py-0.5 rounded bg-amber-500/20 text-amber-300 border border-amber-500/30">
                {SNIPPETS.length} Snippets
              </span>
            </div>
            <p className="text-xs text-slate-400 mt-0.5">
              Kompakte Bausteine für GDT, Paging, VBAR, PendSV, CSRs, IPC und Schichten.
            </p>
          </div>
        </div>

        <div className="relative">
          <Search className="w-4 h-4 text-slate-500 absolute left-3 top-1/2 -translate-y-1/2" />
          <input
            type="text"
            placeholder="Snippets filtern..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-56 bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-1.5 text-xs text-slate-200 outline-none focus:border-amber-500/50 transition-colors"
          />
        </div>
      </div>

      {/* Category Pills */}
      <div className="px-6 py-2 bg-[#0e0e13] border-b border-white/5 flex items-center gap-1.5 overflow-x-auto shrink-0 text-xs">
        <button
          onClick={() => setSelectedCategory('all')}
          className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all ${
            selectedCategory === 'all'
              ? 'bg-white/15 text-white shadow-xs font-semibold'
              : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
          }`}
        >
          Alle Snippets
        </button>
        {categories.map((cat) => (
          <button
            key={cat}
            onClick={() => setSelectedCategory(cat)}
            className={`px-2.5 py-1 rounded-md text-[11px] font-medium transition-all ${
              selectedCategory === cat
                ? 'bg-amber-500/20 border border-amber-500/40 text-amber-300 font-semibold'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            {cat}
          </button>
        ))}
      </div>

      {/* Snippet List */}
      <div className="flex-1 overflow-y-auto p-4 md:p-6 space-y-4 bg-[#08080a]">
        {filteredSnippets.map((snippet) => (
          <div
            key={snippet.id}
            className="bg-black/40 border border-white/10 rounded-xl p-4 hover:border-amber-500/30 transition-all shadow-md"
          >
            <div className="flex items-center justify-between gap-3 mb-2">
              <div className="flex items-center gap-2">
                <h3 className="font-bold text-sm text-slate-200">{snippet.name}</h3>
                {snippet.arch && (
                  <span className="text-[9px] font-bold px-1.5 py-0.2 rounded bg-cyan-500/10 text-cyan-300 border border-cyan-500/30">
                    {snippet.arch}
                  </span>
                )}
                {snippet.paradigm && (
                  <span className="text-[9px] font-bold px-1.5 py-0.2 rounded bg-purple-500/10 text-purple-300 border border-purple-500/30">
                    {snippet.paradigm}
                  </span>
                )}
              </div>

              <div className="flex items-center gap-1.5">
                <button
                  onClick={() => handleCopy(snippet.code, snippet.id)}
                  className="flex items-center gap-1 px-2.5 py-1 bg-white/5 hover:bg-white/10 rounded-md text-xs text-slate-300 transition-colors"
                >
                  {copiedId === snippet.id ? (
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

                {onInsertCode && (
                  <button
                    onClick={() => handleInsert(snippet.code, snippet.id)}
                    className="flex items-center gap-1 px-2.5 py-1 bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 border border-amber-500/40 rounded-md text-xs font-semibold transition-colors"
                  >
                    {insertedId === snippet.id ? (
                      <>
                        <Check className="w-3 h-3 text-amber-300" />
                        <span>Eingefügt</span>
                      </>
                    ) : (
                      <>
                        <Plus className="w-3 h-3" />
                        <span>Einfügen</span>
                      </>
                    )}
                  </button>
                )}
              </div>
            </div>

            <pre className="bg-black/80 border border-white/5 p-3 rounded-lg font-mono text-xs text-cyan-300/90 overflow-x-auto whitespace-pre leading-relaxed">
              {snippet.code}
            </pre>
          </div>
        ))}
      </div>
    </div>
  );
}
