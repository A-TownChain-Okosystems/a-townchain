import React, { useState, useMemo } from 'react';
import {
  BookOpen,
  Shield,
  Cpu,
  Network,
  Layers,
  GitMerge,
  Globe,
  Eye,
  Flame,
  Hexagon,
  Component,
  Radio,
  Zap,
  Server,
  KeyRound,
  Boxes,
  Compass,
  FileCode,
  ShieldCheck,
  AlertTriangle,
  AlertOctagon,
  RefreshCw,
  Download,
  Sparkles,
  Workflow,
  FolderGit2,
  Terminal,
  Lock,
  ChevronRight,
  CheckCircle2,
  ArrowRight,
  Binary,
  Database,
  Activity,
  Copy,
  Check,
  Code2,
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { HARDWARE_ARCHITECTURES, PARADIGMS } from '../data/hardwareArchitectures';
import {
  SHIVACORE_LAYERS,
  SHIVACORE_SYSCALLS,
  CAPABILITY_VERIFICATION_FLOW,
  VERTICAL_CHAIN_GENERAL,
  VERTICAL_CHAIN_BLOCKCHAIN,
  TEN_CORE_PLATFORMS,
  SHIVACORE_FULLSTACK_MARKDOWN,
  ArchLayerDetail,
} from '../data/shivaCoreArchitecture';
import { GlobusV2ArchitecturePanel } from './GlobusV2ArchitecturePanel';
import { GlobusV3ArchitecturePanel } from './GlobusV3ArchitecturePanel';
import { AtcRepoDocumentationPanel } from './AtcRepoDocumentationPanel';
import { AtcVmSimulatorPanel } from './AtcVmSimulatorPanel';
import { AtcDocCompliancePanel } from './AtcDocCompliancePanel';
import { GenesisConfiguratorPanel } from './GenesisConfiguratorPanel';
import { RustWorkspaceGeneratorPanel } from './RustWorkspaceGeneratorPanel';
import { LanguageStrategyPanel } from './LanguageStrategyPanel';
import { GlobusFileFormatsPanel } from './GlobusFileFormatsPanel';
import { FileState, generateLiveArchitectureDocs, runProjectAudit, syncWikiFileInWorkspace } from '../utils/auditEngine';

interface ArchitectureWikiProps {
  files?: FileState[];
  onJumpToFileAndLine?: (file: string, line: number) => void;
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export function ArchitectureWiki({
  files = [],
  onJumpToFileAndLine,
  onSaveWorkspaceFile,
  onShowToast,
}: ArchitectureWikiProps) {
  const [wikiMode, setWikiMode] = useState<'v3' | 'languages' | 'formats' | 'repo-docs' | 'compliance' | 'vm-sim' | 'genesis' | 'crates' | 'v2' | 'fullstack' | 'live' | 'system'>('v3');
  const [fullstackSubTab, setFullstackSubTab] = useState<'layers' | 'chains' | 'platforms' | 'capabilities' | 'syscalls'>('layers');
  const [selectedLayerId, setSelectedLayerId] = useState<string>('layer-8-kernel');
  const [selectedChainType, setSelectedChainType] = useState<'general' | 'blockchain'>('general');
  const [activeChainStep, setActiveChainStep] = useState<number>(0);
  const [syscallCategoryFilter, setSyscallCategoryFilter] = useState<string>('All');
  const [syscallSearch, setSyscallSearch] = useState<string>('');
  const [copiedCode, setCopiedCode] = useState<string | null>(null);

  // System Paradigm mode active section
  const [activeSection, setActiveSection] = useState(0);

  // Live dynamically calculated architecture docs from workspace files
  const liveDoc = useMemo(() => {
    if (!files || files.length === 0) return null;
    const audit = runProjectAudit(files);
    return generateLiveArchitectureDocs(files, audit);
  }, [files]);

  const handleSyncToWorkspace = () => {
    if (!liveDoc || !onSaveWorkspaceFile) return;
    const updated = syncWikiFileInWorkspace(files, liveDoc);
    const archFile = updated.find(f => f.name === 'ARCHITECTURE.md');
    if (archFile) {
      onSaveWorkspaceFile('ARCHITECTURE.md', archFile.content);
      if (onShowToast) onShowToast("'ARCHITECTURE.md' im Workspace aktualisiert!", 'info');
    }
  };

  const handleExportShivaCoreDoc = () => {
    if (!onSaveWorkspaceFile) return;
    onSaveWorkspaceFile('SHIVACORE_ARCHITECTURE.md', SHIVACORE_FULLSTACK_MARKDOWN);
    if (onShowToast) onShowToast("'SHIVACORE_ARCHITECTURE.md' erfolgreich im Workspace gespeichert!", 'info');
  };

  const copyToClipboard = (text: string, id: string) => {
    navigator.clipboard.writeText(text);
    setCopiedCode(id);
    setTimeout(() => setCopiedCode(null), 2000);
    if (onShowToast) onShowToast("Code in Zwischenablage kopiert!", "info");
  };

  const currentLayer = useMemo(() => {
    return SHIVACORE_LAYERS.find(l => l.id === selectedLayerId) || SHIVACORE_LAYERS[7];
  }, [selectedLayerId]);

  const filteredSyscalls = useMemo(() => {
    return SHIVACORE_SYSCALLS.filter(sc => {
      const matchesCategory = syscallCategoryFilter === 'All' || sc.category === syscallCategoryFilter;
      const matchesSearch = syscallSearch.trim() === '' ||
        sc.code.toLowerCase().includes(syscallSearch.toLowerCase()) ||
        sc.description.toLowerCase().includes(syscallSearch.toLowerCase()) ||
        sc.capabilityRequired.toLowerCase().includes(syscallSearch.toLowerCase());
      return matchesCategory && matchesSearch;
    });
  }, [syscallCategoryFilter, syscallSearch]);

  const activeChainSteps = selectedChainType === 'general' ? VERTICAL_CHAIN_GENERAL : VERTICAL_CHAIN_BLOCKCHAIN;

  const sections = [
    {
      title: "1. Vision & Konzept",
      icon: <Globe className="w-5 h-5 text-cyan-400" />,
      content: "ATOS / A-TownChain ist eine KI-native Blockchain- und Systemplattform mit Fokus auf autonome Betriebssysteme, Multi-Architektur-Hardwareunterstützung, semantische KI und skalierbare Microkernels."
    },
    {
      title: "2. Monolithischer Kernel (Monolith)",
      icon: <Server className="w-5 h-5 text-blue-400" />,
      content: "Im monolithischen Kernel laufen Scheduler, VFS, Speichermanagement und alle Gerätetreiber in einem gemeinsamen Ring 0 Adressraum. Vorteile: Maximale I/O-Performance und keine Context-Switch-Penalties. Nachteile: Fehler in Treibern können Kernel-Panics verursachen."
    },
    {
      title: "3. Hierarchische Schutzringe (Hierarchy)",
      icon: <Shield className="w-5 h-5 text-amber-400" />,
      content: "Hierarchische Ring-Architektur trennt Ring 0 (Core Nucleus), Ring 1/2 (Treiber & Systemdienste mit IOPB-Restriktion) und Ring 3 (Userland-Anwendungen). Privileg-Übergänge erfolgen kontrolliert über Call-Gates, TSS-Stack-Switches (RSP0/RSP1) und Interrupt Stack Tables (IST)."
    },
    {
      title: "4. Schichtenarchitektur (Layered)",
      icon: <Layers className="w-5 h-5 text-emerald-400" />,
      content: "Strikte 5-Ebenen Schichtung: Layer 0 (HAL / MMIO), Layer 1 (Kernel Core & Scheduler), Layer 2 (Executive Services & VFS), Layer 3 (Syscall Translation Gateway) und Layer 4 (Userland Apps & GUI). Schichten greifen ausschließlich unidirektional auf darunterliegende Layer zu."
    },
    {
      title: "5. Modulare Systeme & Microkernel",
      icon: <Boxes className="w-5 h-5 text-purple-400" />,
      content: "Modulare Microkernel reduzieren den Ring-0-Code auf Scheduling, Adressraumverwaltung und Fast-IPC Message Passing. Alle Dienste (Dateisysteme, Netzwerk, GPU) laufen als isolierte User-Space Server. Dynamische Kernel-Module (.mod) ermöglichen Live-Patching ohne Reboot."
    },
    {
      title: "6. Hardware-Architektur: x86_64",
      icon: <Cpu className="w-5 h-5 text-cyan-400" />,
      content: "x86_64 / AMD64 bietet 64-Bit Long Mode, 4-Level Paging (PML4 -> PDPT -> PD -> PT, 48-Bit VA), 256 IDT-Vektoren, APIC/x2APIC Interrupt Controller und MSR_LSTAR-basierte Fast Syscalls mit Ring 0-3 Hardware-Enforcement."
    },
    {
      title: "7. Hardware-Architektur: ARM AArch64",
      icon: <Cpu className="w-5 h-5 text-teal-400" />,
      content: "ARMv8/v9 AArch64 nutzt Exception Levels (EL0 User, EL1 Kernel, EL2 Hypervisor, EL3 TrustZone Secure Monitor), VBAR_EL1 Vektortabellen, TTBR0/TTBR1 Split-Paging und GICv3 Interrupt Distributor Architekturen."
    },
    {
      title: "8. Hardware-Architektur: RISC-V",
      icon: <Compass className="w-5 h-5 text-violet-400" />,
      content: "RISC-V (RV64GC) definiert M-Mode (Machine Firmware), S-Mode (Supervisor OS) und U-Mode (User). Die Steuerung erfolgt über strukturierte Control & Status Register (CSR: satp Sv39 Paging, sstatus, stvec, scause, sepc)."
    },
    {
      title: "9. Embedded SoC & Cortex-M RTOS",
      icon: <Zap className="w-5 h-5 text-yellow-400" />,
      content: "Deterministische Echtzeitsysteme (RTOS) für STM32/NXP Cortex-M mit Hardware-MPU-Speicherschutz (Stack Overflow Guards), NVIC-Interrupt-Priorisierung, SysTick 1ms Hardware-Ticks und PendSV Context-Switching."
    },
    {
      title: "10. Security Layer L0–L5",
      icon: <KeyRound className="w-5 h-5 text-red-400" />,
      content: "Mehrstufige Sicherheitsarchitektur mit Zero Trust, Secure Enclaves, TPM 2.0 Hardware-Root-of-Trust, AI Threat Detection und autonomer Cyber-Defense."
    }
  ];

  const stacks = [
    { area: "x86_64 Core", tech: "Long Mode, PML4 Paging, TSS/IST, APIC, MSR Syscalls" },
    { area: "ARM AArch64", tech: "EL0-EL3, VBAR_EL1, TTBR0/TTBR1 Split, GICv3, PSCI" },
    { area: "RISC-V (RV64)", tech: "Sv39 Paging, CSR satp/stvec, PLIC/CLINT, S-Mode Kernel" },
    { area: "ARM Cortex-M", tech: "NVIC Preemption, MPU Regions, PendSV, SysTick RTOS" },
    { area: "Kernel Paradigms", tech: "Monolith, Hierarchische Ringe, Schichten (Layer 0-4), Microkernel IPC" },
    { area: "Security & Enclaves", tech: "Zero Trust, Ring 0-3 Isolation, Capability Tokens, TPM 2.0" }
  ];

  return (
    <div className="flex-1 flex flex-col font-sans h-full bg-[#0c0c0e] text-slate-200 select-none overflow-hidden">
      {/* Top Header */}
      <div className="h-14 flex items-center justify-between px-4 md:px-6 border-b border-white/10 bg-black/40 backdrop-blur-2xl shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center">
            <BookOpen className="w-4 h-4 text-cyan-400" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-sm font-bold text-slate-100 tracking-wide">
                ShivaCore & Globus OS Architektur-Wiki
              </h2>
              <span className="px-2 py-0.5 bg-cyan-500/10 border border-cyan-500/30 rounded text-[10px] text-cyan-400 uppercase tracking-wider font-mono">
                10-Layer Stack
              </span>
            </div>
          </div>
        </div>

        {/* Primary Mode Switcher */}
        <div className="flex items-center gap-2">
          <div className="flex bg-white/5 p-1 rounded-lg border border-white/10 text-xs">
            <button
              onClick={() => setWikiMode('v3')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'v3'
                  ? 'bg-indigo-500/25 text-indigo-300 border border-indigo-500/40 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Workflow className="w-3.5 h-3.5 text-indigo-400" />
              <span>Zielarchitektur v3</span>
            </button>

            <button
              onClick={() => setWikiMode('languages')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'languages'
                  ? 'bg-indigo-500/25 text-indigo-300 border border-indigo-500/40 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Code2 className="w-3.5 h-3.5 text-indigo-400" />
              <span>Sprachstrategie</span>
            </button>

            <button
              onClick={() => setWikiMode('formats')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'formats'
                  ? 'bg-cyan-500/25 text-cyan-300 border border-cyan-500/40 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Binary className="w-3.5 h-3.5 text-cyan-400" />
              <span>Dateiformate (.g*)</span>
            </button>

            <button
              onClick={() => setWikiMode('repo-docs')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'repo-docs'
                  ? 'bg-emerald-500/25 text-emerald-300 border border-emerald-500/40 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <FolderGit2 className="w-3.5 h-3.5 text-emerald-400" />
              <span>Repo & Docs</span>
            </button>

            <button
              onClick={() => setWikiMode('compliance')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'compliance'
                  ? 'bg-emerald-500/25 text-emerald-300 border border-emerald-500/40 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
              <span>Linter & Compliance</span>
            </button>

            <button
              onClick={() => setWikiMode('vm-sim')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'vm-sim'
                  ? 'bg-cyan-500/25 text-cyan-300 border border-cyan-500/40 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Terminal className="w-3.5 h-3.5 text-cyan-400" />
              <span>ATC-VM Simulator</span>
            </button>

            <button
              onClick={() => setWikiMode('genesis')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'genesis'
                  ? 'bg-indigo-500/25 text-indigo-300 border border-indigo-500/40 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Server className="w-3.5 h-3.5 text-indigo-400" />
              <span>Genesis & Devnet</span>
            </button>

            <button
              onClick={() => setWikiMode('crates')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'crates'
                  ? 'bg-orange-500/25 text-orange-300 border border-orange-500/40 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Boxes className="w-3.5 h-3.5 text-orange-400" />
              <span>Rust Workspace</span>
            </button>

            <button
              onClick={() => setWikiMode('v2')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'v2'
                  ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Workflow className="w-3.5 h-3.5 text-cyan-400" />
              <span>Zielarchitektur v2</span>
            </button>

            <button
              onClick={() => setWikiMode('fullstack')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'fullstack'
                  ? 'bg-blue-500/20 text-blue-300 border border-blue-500/30 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Layers className="w-3.5 h-3.5 text-blue-400" />
              <span>10-Layer Stack</span>
            </button>

            <button
              onClick={() => setWikiMode('live')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'live'
                  ? 'bg-purple-500/20 text-purple-300 border border-purple-500/30 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Sparkles className="w-3.5 h-3.5 text-purple-400" />
              <span>Live Workspace ({files.length})</span>
            </button>

            <button
              onClick={() => setWikiMode('system')}
              className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                wikiMode === 'system'
                  ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 shadow-sm'
                  : 'text-slate-400 hover:text-slate-200'
              }`}
            >
              <Cpu className="w-3.5 h-3.5 text-emerald-400" />
              <span>Hardware & Kern-Paradigmen</span>
            </button>
          </div>

          {wikiMode === 'fullstack' && onSaveWorkspaceFile && (
            <button
              onClick={handleExportShivaCoreDoc}
              className="px-3 py-1.5 bg-cyan-600/30 hover:bg-cyan-600/40 text-cyan-200 border border-cyan-500/40 rounded-lg text-xs font-bold flex items-center gap-1.5 transition-all cursor-pointer shadow-sm"
              title="Exportiert die vollständige 10-Schichten-Spezifikation als SHIVACORE_ARCHITECTURE.md in den Workspace"
            >
              <Download className="w-3.5 h-3.5" />
              <span className="hidden sm:inline">Als SHIVACORE_ARCHITECTURE.md speichern</span>
            </button>
          )}

          {wikiMode === 'live' && onSaveWorkspaceFile && (
            <button
              onClick={handleSyncToWorkspace}
              className="px-3 py-1.5 bg-purple-600/30 hover:bg-purple-600/40 text-purple-200 border border-purple-500/40 rounded-lg text-xs font-bold flex items-center gap-1.5 transition-all cursor-pointer shadow-sm"
              title="Schreibt ARCHITECTURE.md direkt in den Workspace"
            >
              <Download className="w-3.5 h-3.5" />
              <span className="hidden sm:inline">In ARCHITECTURE.md synchronisieren</span>
            </button>
          )}
        </div>
      </div>

      {/* VIEW MODE -1: ZIELARCHITEKTUR V3 (FORMAL PLATFORM ARCHITECTURE) */}
      {wikiMode === 'v3' && (
        <GlobusV3ArchitecturePanel
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE LANGUAGES: SPRACHSTRATEGIE & SYSTEMEBENEN-MATRIX */}
      {wikiMode === 'languages' && (
        <LanguageStrategyPanel
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE FORMATS: GLOBUS FILE FORMAT ARCHITECTURE (GFFA / GNFF v1.0) */}
      {wikiMode === 'formats' && (
        <GlobusFileFormatsPanel
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE REPO-DOCS: REPOSITORY & ENGINEERING DOCUMENTATION STANDARD (ATC-DOC) */}
      {wikiMode === 'repo-docs' && (
        <AtcRepoDocumentationPanel
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE COMPLIANCE: ATC-DOC LINTER & AUDIT SUITE */}
      {wikiMode === 'compliance' && (
        <AtcDocCompliancePanel
          files={files}
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE VM-SIM: ATC-VM REGISTER SIMULATOR & STEP DEBUGGER */}
      {wikiMode === 'vm-sim' && (
        <AtcVmSimulatorPanel
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE GENESIS: GENESIS BLOCK & DEVNET NODE CONFIGURATOR */}
      {wikiMode === 'genesis' && (
        <GenesisConfiguratorPanel
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE CRATES: RUST CARGO WORKSPACE ARCHITECTURE */}
      {wikiMode === 'crates' && (
        <RustWorkspaceGeneratorPanel
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE 0: ZIELARCHITEKTUR V2 */}
      {wikiMode === 'v2' && (
        <GlobusV2ArchitecturePanel
          onSaveWorkspaceFile={onSaveWorkspaceFile}
          onShowToast={onShowToast}
        />
      )}

      {/* VIEW MODE 1: FULL-STACK SHIVACORE 10-LAYER STACK */}
      {wikiMode === 'fullstack' && (
        <div className="flex-1 flex flex-col overflow-hidden">
          {/* Sub-Navigation Tabs */}
          <div className="h-11 px-6 bg-black/50 border-b border-white/10 flex items-center justify-between shrink-0">
            <div className="flex items-center gap-1">
              <button
                onClick={() => setFullstackSubTab('layers')}
                className={`px-3 py-1.5 rounded-md text-xs font-medium transition-all flex items-center gap-2 ${
                  fullstackSubTab === 'layers'
                    ? 'bg-white/10 text-cyan-400 font-bold border border-white/10 shadow-sm'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
                }`}
              >
                <Layers className="w-3.5 h-3.5" />
                <span>10-Schichten-Modell</span>
              </button>

              <button
                onClick={() => setFullstackSubTab('chains')}
                className={`px-3 py-1.5 rounded-md text-xs font-medium transition-all flex items-center gap-2 ${
                  fullstackSubTab === 'chains'
                    ? 'bg-white/10 text-cyan-400 font-bold border border-white/10 shadow-sm'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
                }`}
              >
                <Workflow className="w-3.5 h-3.5" />
                <span>Vertikale Ausführungsketten</span>
              </button>

              <button
                onClick={() => setFullstackSubTab('platforms')}
                className={`px-3 py-1.5 rounded-md text-xs font-medium transition-all flex items-center gap-2 ${
                  fullstackSubTab === 'platforms'
                    ? 'bg-white/10 text-cyan-400 font-bold border border-white/10 shadow-sm'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
                }`}
              >
                <Boxes className="w-3.5 h-3.5" />
                <span>10 Hauptplattformen</span>
              </button>

              <button
                onClick={() => setFullstackSubTab('capabilities')}
                className={`px-3 py-1.5 rounded-md text-xs font-medium transition-all flex items-center gap-2 ${
                  fullstackSubTab === 'capabilities'
                    ? 'bg-white/10 text-cyan-400 font-bold border border-white/10 shadow-sm'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
                }`}
              >
                <Shield className="w-3.5 h-3.5" />
                <span>Capability-Security Primitive</span>
              </button>

              <button
                onClick={() => setFullstackSubTab('syscalls')}
                className={`px-3 py-1.5 rounded-md text-xs font-medium transition-all flex items-center gap-2 ${
                  fullstackSubTab === 'syscalls'
                    ? 'bg-white/10 text-cyan-400 font-bold border border-white/10 shadow-sm'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
                }`}
              >
                <Terminal className="w-3.5 h-3.5" />
                <span>Syscall ABI Dispatcher ({SHIVACORE_SYSCALLS.length})</span>
              </button>
            </div>

            <span className="text-[11px] text-slate-500 font-mono hidden md:inline">
              Architektur-Grundsatz: Vertikale Kette API → Middleware → Runtime → Syscall ABI → Kernel → HAL → Hardware
            </span>
          </div>

          {/* SubTab Content */}
          <div className="flex-1 overflow-hidden">
            {/* SUBTAB 1: 10-LAYERS STACK */}
            {fullstackSubTab === 'layers' && (
              <div className="h-full flex overflow-hidden">
                {/* Left Stack Column */}
                <div className="w-80 md:w-96 border-r border-white/10 bg-black/30 overflow-y-auto p-4 shrink-0 flex flex-col gap-2">
                  <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1 px-1">
                    System-Schichten (L1 - L10)
                  </div>
                  {SHIVACORE_LAYERS.map(layer => {
                    const isSelected = layer.id === selectedLayerId;
                    return (
                      <button
                        key={layer.id}
                        onClick={() => setSelectedLayerId(layer.id)}
                        className={`w-full text-left p-3 rounded-xl border transition-all flex flex-col gap-1.5 ${
                          isSelected
                            ? `${layer.badgeBg} ${layer.badgeBorder} shadow-lg ring-1 ring-cyan-500/30`
                            : 'bg-white/[0.02] border-white/5 hover:bg-white/5 text-slate-300'
                        }`}
                      >
                        <div className="flex items-center justify-between">
                          <div className="flex items-center gap-2">
                            <span className={`w-5 h-5 rounded-md flex items-center justify-center text-xs font-mono font-bold ${
                              isSelected ? 'bg-white/20 text-white' : 'bg-white/5 text-slate-400'
                            }`}>
                              L{layer.number}
                            </span>
                            <span className={`text-xs font-bold ${isSelected ? layer.textColor : 'text-slate-200'}`}>
                              {layer.name}
                            </span>
                          </div>
                          {layer.number >= 7 && (
                            <span className="text-[9px] px-1.5 py-0.5 rounded font-mono uppercase bg-red-500/10 text-red-400 border border-red-500/20">
                              {layer.number === 7 ? 'Trap Barrier' : layer.number === 8 ? 'Ring 0' : layer.number === 9 ? 'HAL' : 'Silicon'}
                            </span>
                          )}
                          {layer.number <= 6 && (
                            <span className="text-[9px] px-1.5 py-0.5 rounded font-mono uppercase bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
                              User Space
                            </span>
                          )}
                        </div>
                        <p className="text-[11px] text-slate-400 line-clamp-1">
                          {layer.headline}
                        </p>
                      </button>
                    );
                  })}
                </div>

                {/* Right Layer Inspector */}
                <div className="flex-1 overflow-y-auto p-6 md:p-8 bg-[#08080a] space-y-6">
                  <div className="max-w-4xl space-y-6">
                    {/* Layer Header Card */}
                    <div className={`p-6 rounded-2xl ${currentLayer.badgeBg} border ${currentLayer.badgeBorder} space-y-3`}>
                      <div className="flex items-center justify-between">
                        <span className="text-xs font-mono font-bold uppercase tracking-wider px-2.5 py-1 rounded-md bg-white/10 text-white">
                          Layer {currentLayer.number} / 10 • {currentLayer.shortName}
                        </span>
                        <span className="text-xs font-mono text-slate-400">
                          Sicherheitsdomäne: {currentLayer.number >= 8 ? 'Ring 0 (Supervisor / Nucleus)' : currentLayer.number === 7 ? 'MSR Trap Transition' : 'Ring 3 (PoLP Sandbox)'}
                        </span>
                      </div>
                      <h1 className={`text-2xl font-bold ${currentLayer.textColor}`}>
                        {currentLayer.name}
                      </h1>
                      <p className="text-sm text-slate-300 leading-relaxed">
                        {currentLayer.headline}
                      </p>
                      <div className="pt-2 text-xs text-slate-400 border-t border-white/10 flex items-center gap-2">
                        <Shield className="w-4 h-4 text-amber-400 shrink-0" />
                        <span><strong>Security Policy:</strong> {currentLayer.securityRole}</span>
                      </div>
                    </div>

                    {/* Architectural Components Grid */}
                    <div className="space-y-3">
                      <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                        Schichtkomponenten & Module ({currentLayer.components.length})
                      </h3>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {currentLayer.components.map((comp, idx) => (
                          <div key={idx} className="p-4 rounded-xl bg-black/40 border border-white/10 space-y-2">
                            <div className="flex items-center justify-between">
                              <h4 className="text-xs font-bold text-slate-100">{comp.title}</h4>
                              <span className="text-[10px] text-cyan-400 font-mono bg-cyan-500/10 px-2 py-0.5 rounded">
                                {comp.role}
                              </span>
                            </div>
                            <p className="text-xs text-slate-300 leading-relaxed">
                              {comp.description}
                            </p>
                            <div className="flex flex-wrap gap-1 pt-1">
                              {comp.techStack.map((tech, tIdx) => (
                                <span key={tIdx} className="text-[10px] font-mono px-2 py-0.5 rounded bg-white/5 text-slate-400 border border-white/5">
                                  {tech}
                                </span>
                              ))}
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>

                    {/* Interfaces & Protocols */}
                    <div className="p-5 rounded-xl bg-black/50 border border-white/10 space-y-3">
                      <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-2">
                        <Network className="w-4 h-4 text-cyan-400" />
                        Schnittstellen, Protokolle & Datenfluss
                      </h3>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                        {currentLayer.interfaces.map((iface, idx) => (
                          <div key={idx} className="p-3 rounded-lg bg-white/[0.02] border border-white/5 space-y-1">
                            <div className="flex items-center justify-between text-xs font-mono">
                              <span className="text-cyan-400 font-bold">{iface.type}</span>
                              <span className="text-slate-400">{iface.protocol}</span>
                            </div>
                            <p className="text-xs text-slate-400">{iface.description}</p>
                          </div>
                        ))}
                      </div>
                    </div>

                    {/* Layer ASCII Layout */}
                    <div className="p-5 rounded-xl bg-black/70 border border-white/10 space-y-2">
                      <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                        Schichtdiagramm
                      </h3>
                      <pre className="p-4 rounded-lg bg-black font-mono text-xs text-cyan-300 border border-white/5 overflow-x-auto leading-relaxed">
                        {currentLayer.diagram}
                      </pre>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* SUBTAB 2: VERTICAL EXECUTION CHAINS */}
            {fullstackSubTab === 'chains' && (
              <div className="h-full overflow-y-auto p-6 md:p-8 bg-[#08080a]">
                <div className="max-w-5xl mx-auto space-y-6">
                  {/* Chain Switcher & Info */}
                  <div className="p-6 rounded-2xl bg-gradient-to-r from-cyan-950/40 via-black/80 to-purple-950/40 border border-cyan-500/30 space-y-4">
                    <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
                      <div>
                        <span className="text-xs font-mono text-cyan-400 uppercase tracking-wider font-bold">
                          End-to-End Execution Trace
                        </span>
                        <h2 className="text-xl font-bold text-white mt-1">
                          Vertikale Systemausführungsketten
                        </h2>
                        <p className="text-xs text-slate-300 mt-1 max-w-2xl">
                          Die eigentliche Plattform entsteht durch die durchgängige Kette aus API → Middleware → Runtime → Syscall ABI → Kernel → HAL → Hardware.
                        </p>
                      </div>

                      {/* Chain Toggle Buttons */}
                      <div className="flex bg-black/60 p-1 rounded-xl border border-white/10 text-xs shrink-0">
                        <button
                          onClick={() => {
                            setSelectedChainType('general');
                            setActiveChainStep(0);
                          }}
                          className={`px-3 py-1.5 rounded-lg font-bold transition-all ${
                            selectedChainType === 'general'
                              ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/40'
                              : 'text-slate-400 hover:text-slate-200'
                          }`}
                        >
                          1. Standard OS Chain ({VERTICAL_CHAIN_GENERAL.length} Stufen)
                        </button>
                        <button
                          onClick={() => {
                            setSelectedChainType('blockchain');
                            setActiveChainStep(0);
                          }}
                          className={`px-3 py-1.5 rounded-lg font-bold transition-all ${
                            selectedChainType === 'blockchain'
                              ? 'bg-purple-500/20 text-purple-300 border border-purple-500/40'
                              : 'text-slate-400 hover:text-slate-200'
                          }`}
                        >
                          2. Blockchain Web3 Chain ({VERTICAL_CHAIN_BLOCKCHAIN.length} Stufen)
                        </button>
                      </div>
                    </div>
                  </div>

                  {/* Step Interactive Visualizer */}
                  <div className="space-y-3">
                    <div className="text-xs font-bold text-slate-400 uppercase tracking-wider flex items-center justify-between">
                      <span>Schritt-für-Schritt Ablaufverfolgung</span>
                      <span className="text-[11px] font-mono text-cyan-400">
                        Schritt {activeChainStep + 1} von {activeChainSteps.length}: {activeChainSteps[activeChainStep]?.layer}
                      </span>
                    </div>

                    {/* Progress Track */}
                    <div className="flex items-center gap-1 overflow-x-auto pb-2">
                      {activeChainSteps.map((step, idx) => (
                        <button
                          key={idx}
                          onClick={() => setActiveChainStep(idx)}
                          className={`px-3 py-2 rounded-lg text-xs font-mono transition-all flex items-center gap-2 shrink-0 border ${
                            activeChainStep === idx
                              ? 'bg-cyan-500/20 border-cyan-500 text-cyan-300 font-bold shadow-md'
                              : 'bg-white/5 border-white/5 text-slate-400 hover:bg-white/10'
                          }`}
                        >
                          <span className="text-[10px] opacity-60">#{step.index}</span>
                          <span>{step.layer}</span>
                          {step.boundary && (
                            <span className="w-2 h-2 rounded-full bg-red-400 animate-pulse" title="Hardware/Privilege Barrier" />
                          )}
                        </button>
                      ))}
                    </div>

                    {/* Active Step Deep Dive Card */}
                    <div className="p-6 rounded-2xl bg-black/60 border border-white/10 space-y-4">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-3">
                          <span className="px-2.5 py-1 rounded-md bg-cyan-500/10 border border-cyan-500/30 text-cyan-400 text-xs font-mono font-bold">
                            Schritt {activeChainSteps[activeChainStep]?.index}: {activeChainSteps[activeChainStep]?.layer}
                          </span>
                          <span className="text-xs font-mono text-slate-400">
                            Privileg-Domäne: <strong className="text-slate-200">{activeChainSteps[activeChainStep]?.privilege}</strong>
                          </span>
                        </div>
                        {activeChainSteps[activeChainStep]?.boundary && (
                          <span className="px-2.5 py-1 rounded-md bg-red-500/20 border border-red-500/40 text-red-400 text-xs font-mono font-bold flex items-center gap-1">
                            <Lock className="w-3.5 h-3.5" />
                            {activeChainSteps[activeChainStep]?.boundary}
                          </span>
                        )}
                      </div>

                      <div className="space-y-2">
                        <div className="text-xs text-slate-400 uppercase tracking-wider font-bold">
                          Ausgeführte Aktion:
                        </div>
                        <p className="text-sm text-slate-100 font-medium">
                          {activeChainSteps[activeChainStep]?.action}
                        </p>
                      </div>

                      <div className="space-y-1.5 p-3 rounded-lg bg-black border border-white/5 font-mono text-xs">
                        <div className="text-[10px] text-slate-500 uppercase tracking-wider font-bold">
                          Daten-Payload / Registerzustand:
                        </div>
                        <div className="text-cyan-300">
                          {activeChainSteps[activeChainStep]?.payload}
                        </div>
                      </div>

                      {/* Navigation buttons */}
                      <div className="flex justify-between items-center pt-2">
                        <button
                          disabled={activeChainStep === 0}
                          onClick={() => setActiveChainStep(prev => Math.max(0, prev - 1))}
                          className="px-3 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-slate-300 text-xs font-semibold disabled:opacity-30 disabled:pointer-events-none transition-colors"
                        >
                          &larr; Vorheriger Schritt
                        </button>
                        <button
                          disabled={activeChainStep >= activeChainSteps.length - 1}
                          onClick={() => setActiveChainStep(prev => Math.min(activeChainSteps.length - 1, prev + 1))}
                          className="px-3 py-1.5 rounded-lg bg-cyan-600/30 hover:bg-cyan-600/40 text-cyan-200 border border-cyan-500/40 text-xs font-bold disabled:opacity-30 disabled:pointer-events-none transition-colors flex items-center gap-1.5"
                        >
                          <span>Nächster Schritt</span>
                          <ArrowRight className="w-3.5 h-3.5" />
                        </button>
                      </div>
                    </div>
                  </div>

                  {/* Complete Chain Flowchart Overview */}
                  <div className="p-6 rounded-2xl bg-black/40 border border-white/10 space-y-3">
                    <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider">
                      Vollständige Kette im Überblick
                    </h3>
                    <div className="space-y-2">
                      {activeChainSteps.map((step, idx) => (
                        <div
                          key={idx}
                          onClick={() => setActiveChainStep(idx)}
                          className={`p-3 rounded-xl border transition-all cursor-pointer flex items-center justify-between ${
                            activeChainStep === idx
                              ? 'bg-cyan-500/15 border-cyan-500/40 text-white shadow-sm'
                              : 'bg-white/[0.02] border-white/5 hover:bg-white/5 text-slate-300'
                          }`}
                        >
                          <div className="flex items-center gap-3">
                            <span className="w-6 h-6 rounded-md bg-white/5 flex items-center justify-center text-xs font-mono text-slate-400">
                              {step.index}
                            </span>
                            <div>
                              <div className="text-xs font-bold font-mono text-cyan-400">
                                {step.layer}
                              </div>
                              <div className="text-xs text-slate-300 line-clamp-1">
                                {step.action}
                              </div>
                            </div>
                          </div>
                          <div className="text-right shrink-0">
                            <span className="text-[10px] font-mono text-slate-400 block">
                              {step.privilege}
                            </span>
                            {step.boundary && (
                              <span className="text-[9px] font-mono text-red-400 font-bold block">
                                [Barrier]
                              </span>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* SUBTAB 3: 10 HAUPTPLATTFORMEN */}
            {fullstackSubTab === 'platforms' && (
              <div className="h-full overflow-y-auto p-6 md:p-8 bg-[#08080a]">
                <div className="max-w-5xl mx-auto space-y-6">
                  <div className="p-6 rounded-2xl bg-gradient-to-r from-blue-950/40 via-black/80 to-indigo-950/40 border border-blue-500/30 space-y-2">
                    <span className="text-xs font-mono text-blue-400 uppercase tracking-wider font-bold">
                      Organisationsmodell
                    </span>
                    <h2 className="text-xl font-bold text-white">
                      Die 10 Kernplattformen von ShivaCore / Globus OS
                    </h2>
                    <p className="text-xs text-slate-300 leading-relaxed max-w-3xl">
                      Jede Plattform bündelt einen klar abgegrenzten Verantwortungsbereich, von den visuellen Frontends über standardisierte SDKs bis hin zur physischen Hardware-Infrastruktur.
                    </p>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {TEN_CORE_PLATFORMS.map(platform => (
                      <div
                        key={platform.id}
                        className="p-5 rounded-2xl bg-black/40 border border-white/10 space-y-3 hover:border-cyan-500/30 transition-all flex flex-col justify-between"
                      >
                        <div className="space-y-2">
                          <div className="flex items-center justify-between">
                            <h3 className="text-sm font-bold text-white font-mono">
                              {platform.name}
                            </h3>
                            <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
                              {platform.scope}
                            </span>
                          </div>
                          <p className="text-xs text-slate-300 leading-relaxed">
                            {platform.objective}
                          </p>
                        </div>

                        <div className="space-y-2 pt-2 border-t border-white/5">
                          <div className="text-[10px] font-mono text-slate-500 uppercase tracking-wider font-bold">
                            Kern-Liefergegenstände (Deliverables):
                          </div>
                          <ul className="space-y-1 text-xs text-slate-400">
                            {platform.keyDeliverables.map((item, dIdx) => (
                              <li key={dIdx} className="flex items-start gap-1.5">
                                <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0 mt-0.5" />
                                <span>{item}</span>
                              </li>
                            ))}
                          </ul>
                          <div className="pt-2 text-[11px] font-mono text-slate-400 flex items-center gap-1.5">
                            <span className="text-slate-500">Tech:</span>
                            <span className="text-slate-300">{platform.leadTech}</span>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}

            {/* SUBTAB 4: CAPABILITY SECURITY PRIMITIVE */}
            {fullstackSubTab === 'capabilities' && (
              <div className="h-full overflow-y-auto p-6 md:p-8 bg-[#08080a]">
                <div className="max-w-5xl mx-auto space-y-6">
                  <div className="p-6 rounded-2xl bg-gradient-to-r from-red-950/40 via-black/80 to-amber-950/40 border border-red-500/30 space-y-3">
                    <span className="text-xs font-mono text-red-400 uppercase tracking-wider font-bold">
                      Zero-Trust Security Primitive
                    </span>
                    <h2 className="text-xl font-bold text-white">
                      Capabilities als zentrale Sicherheitsprimitive
                    </h2>
                    <p className="text-xs text-slate-300 leading-relaxed max-w-3xl">
                      In ShivaCore sind Capabilities keine nachgelagerte Berechtigungstabelle, sondern das unumgängliche Token für jeden einzelnen Kernel- und Ressourcenzugriff. Kein Prozess kann Speicher, IPC, Dateien oder Geräte manipulieren, ohne ein gültiges, unverändertes Handle vorzuweisen.
                    </p>
                  </div>

                  {/* Verification Pipeline Steps */}
                  <div className="space-y-3">
                    <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                      6-Stufige Validierungskette bei jedem Zugriff
                    </h3>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      {CAPABILITY_VERIFICATION_FLOW.map(flow => (
                        <div key={flow.step} className="p-5 rounded-xl bg-black/40 border border-white/10 space-y-3">
                          <div className="flex items-center justify-between">
                            <h4 className="text-xs font-bold text-cyan-300 font-mono">
                              {flow.name}
                            </h4>
                            <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-white/5 text-slate-400">
                              Stufe {flow.step}
                            </span>
                          </div>
                          <p className="text-xs text-slate-300 leading-relaxed">
                            {flow.description}
                          </p>
                          <div className="p-3 rounded-lg bg-black font-mono text-[11px] text-emerald-300 border border-white/5 overflow-x-auto">
                            <pre className="whitespace-pre">{flow.codeSnippet}</pre>
                          </div>
                          <div className="text-[11px] text-slate-400 flex items-center gap-1.5">
                            <ShieldCheck className="w-3.5 h-3.5 text-amber-400 shrink-0" />
                            <span><strong>Enforcement:</strong> {flow.enforcement}</span>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>

                  {/* C/Rust Struct Definition */}
                  <div className="p-6 rounded-2xl bg-black/70 border border-white/10 space-y-3">
                    <div className="flex items-center justify-between">
                      <h3 className="text-xs font-bold text-slate-200 uppercase tracking-wider font-mono">
                        Kernel-Datenstrukturen (C / Rust / ShivaCore Core)
                      </h3>
                      <button
                        onClick={() => copyToClipboard(`typedef struct sc_capability {
    uint64_t cap_id;           // Eindeutige Capability-ID
    uint32_t object_type;      // SC_OBJ_MEMORY, SC_OBJ_IPC, SC_OBJ_FILE...
    uint32_t object_id;        // Ziel-Objekt
    uint64_t rights_mask;      // Erlaubte Bitmasken (READ, WRITE, EXEC...)
    uint64_t parent_cap_id;    // Lineage-Tracker für Delegation
    uint64_t expiry_timestamp; // Monotone Expiry (0 = unbegrenzt)
    uint32_t ns_domain;        // Namensraum-Kapselung
    uint32_t flags;            // SC_CAP_REVOKED, SC_CAP_DELEGATABLE
} sc_cap_entry_t;`, 'cap_struct')}
                        className="px-2 py-1 rounded bg-white/5 hover:bg-white/10 text-xs font-mono text-slate-300 border border-white/5 flex items-center gap-1"
                      >
                        {copiedCode === 'cap_struct' ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                        <span>Kopieren</span>
                      </button>
                    </div>

                    <pre className="p-4 rounded-xl bg-black font-mono text-xs text-cyan-300 border border-white/5 overflow-x-auto whitespace-pre leading-relaxed">
{`typedef struct sc_capability {
    uint64_t cap_id;           // Eindeutige Capability-ID
    uint32_t object_type;      // SC_OBJ_MEMORY, SC_OBJ_IPC, SC_OBJ_FILE, SC_OBJ_DEVICE
    uint32_t object_id;        // Referenz auf Kernel-Objekt
    uint64_t rights_mask;      // Erlaubte Bitmasken (READ, WRITE, EXEC, DELEGATE, IOCTL)
    uint64_t parent_cap_id;    // Lineage-Tracker für Delegation (DAG)
    uint64_t expiry_timestamp; // Monotone Expiry (0 = unbegrenzt)
    uint32_t ns_domain;        // Namensraum-Kapselung (Isolation)
    uint32_t flags;            // SC_CAP_REVOKED, SC_CAP_DELEGATABLE, SC_CAP_IMMUTABLE
} sc_cap_entry_t;

// Validierungsfunktion im Syscall-Handler:
sc_status_t sc_cap_validate(sc_process_t* proc, sc_handle_t handle, uint64_t requested_rights) {
    if (handle >= proc->cap_table.capacity) return SC_ERR_INVALID_HANDLE;
    sc_cap_entry_t* cap = &proc->cap_table.entries[handle];
    if (cap->flags & SC_CAP_REVOKED) return SC_ERR_CAPABILITY_REVOKED;
    if ((cap->rights_mask & requested_rights) != requested_rights) return SC_ERR_PERMISSION_DENIED;
    return SC_SUCCESS;
}`}
                    </pre>
                  </div>
                </div>
              </div>
            )}

            {/* SUBTAB 5: SYSCALL ABI DISPATCHER MATRIX */}
            {fullstackSubTab === 'syscalls' && (
              <div className="h-full overflow-y-auto p-6 md:p-8 bg-[#08080a]">
                <div className="max-w-5xl mx-auto space-y-6">
                  {/* Syscall Header & Search */}
                  <div className="p-6 rounded-2xl bg-gradient-to-r from-teal-950/40 via-black/80 to-cyan-950/40 border border-teal-500/30 space-y-4">
                    <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
                      <div>
                        <span className="text-xs font-mono text-teal-400 uppercase tracking-wider font-bold">
                          Hardware-Software-Schnittstelle (ABI)
                        </span>
                        <h2 className="text-xl font-bold text-white mt-1">
                          ShivaCore System Call Dispatcher
                        </h2>
                        <p className="text-xs text-slate-300 mt-1 max-w-2xl">
                          Register-Marshalling über x86_64 `syscall` (MSR_LSTAR) und ARM64 `svc #0`. Alle Aufrufe erfordern passende Capability-Handles.
                        </p>
                      </div>

                      <div className="w-full md:w-64">
                        <input
                          type="text"
                          value={syscallSearch}
                          onChange={(e) => setSyscallSearch(e.target.value)}
                          placeholder="Syscall filtern..."
                          className="w-full px-3 py-1.5 bg-black/50 border border-white/10 rounded-lg text-xs text-slate-200 outline-none focus:border-cyan-500/50"
                        />
                      </div>
                    </div>

                    {/* Category Filter Chips */}
                    <div className="flex flex-wrap gap-1.5 pt-1">
                      {['All', 'Process', 'Thread', 'Memory', 'IPC', 'Handle', 'File', 'Network', 'Capability', 'Device', 'Time', 'Audit'].map(cat => (
                        <button
                          key={cat}
                          onClick={() => setSyscallCategoryFilter(cat)}
                          className={`px-2.5 py-1 rounded-md text-xs font-mono transition-colors ${
                            syscallCategoryFilter === cat
                              ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/40 font-bold'
                              : 'bg-white/5 text-slate-400 hover:bg-white/10 hover:text-slate-200'
                          }`}
                        >
                          {cat}
                        </button>
                      ))}
                    </div>
                  </div>

                  {/* Syscall Table */}
                  <div className="space-y-3">
                    <div className="text-xs font-bold text-slate-400 uppercase tracking-wider">
                      Syscall-Vektoren ({filteredSyscalls.length})
                    </div>

                    <div className="space-y-3">
                      {filteredSyscalls.map((sc) => (
                        <div key={sc.code} className="p-4 rounded-xl bg-black/40 border border-white/10 space-y-2 hover:border-cyan-500/30 transition-all">
                          <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2">
                            <div className="flex items-center gap-2">
                              <span className="text-xs font-mono font-bold text-cyan-400">
                                {sc.code}
                              </span>
                              <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-white/5 text-slate-400 border border-white/5">
                                Opcode {sc.opcode}
                              </span>
                              <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-cyan-500/10 text-cyan-300">
                                {sc.category}
                              </span>
                            </div>

                            <div className="flex items-center gap-2 text-[10px] font-mono">
                              <span className="text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded border border-amber-500/20">
                                Cap: {sc.capabilityRequired}
                              </span>
                              <span className="text-red-400 bg-red-500/10 px-2 py-0.5 rounded border border-red-500/20">
                                {sc.ring}
                              </span>
                            </div>
                          </div>

                          <div className="p-2.5 rounded-lg bg-black border border-white/5 font-mono text-xs text-slate-300 overflow-x-auto">
                            <code>{sc.signature}</code>
                          </div>

                          <div className="grid grid-cols-1 md:grid-cols-2 gap-2 text-xs pt-1">
                            <div className="text-slate-300">
                              <span className="text-slate-500 font-mono text-[10px] block uppercase font-bold">Beschreibung:</span>
                              {sc.description}
                            </div>
                            <div className="font-mono text-xs text-slate-400">
                              <span className="text-slate-500 font-mono text-[10px] block uppercase font-bold">Register-Konvention:</span>
                              <span className="text-teal-300">{sc.registers}</span>
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      {/* VIEW MODE 2: LIVE WORKSPACE ARCHITECTURE */}
      {wikiMode === 'live' && liveDoc && (
        <div className="flex-1 overflow-y-auto p-6 md:p-8 bg-[#08080a]">
          <div className="max-w-5xl mx-auto space-y-6">
            {/* Live Overview Card */}
            <div className="p-6 rounded-2xl bg-gradient-to-r from-purple-950/40 via-black/80 to-indigo-950/40 border border-purple-500/30 space-y-2 shadow-xl">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-purple-400 uppercase tracking-wider flex items-center gap-1.5">
                  <Sparkles className="w-4 h-4" /> Live-Projektstatus
                </span>
                <span className="text-[11px] text-slate-500 font-mono">Synchronisiert: {liveDoc.lastUpdated}</span>
              </div>
              <h1 className="text-2xl font-bold text-white tracking-tight">{liveDoc.projectName}</h1>
              <p className="text-sm text-slate-300 leading-relaxed">{liveDoc.overview}</p>
            </div>

            {/* Architectural Layers */}
            <div className="space-y-3">
              <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider">Erkannte Architektur-Schichten</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {liveDoc.layers.map((layer, idx) => (
                  <div key={idx} className="p-5 rounded-xl bg-black/40 border border-white/10 space-y-2.5">
                    <div className="flex items-center justify-between">
                      <h4 className="text-xs font-bold text-cyan-300 font-mono">{layer.name}</h4>
                      <span className="text-[10px] text-slate-500 font-mono">{layer.files.length} Dateien</span>
                    </div>
                    <p className="text-xs text-slate-400">{layer.description}</p>
                    <div className="flex flex-wrap gap-1.5 pt-1">
                      {layer.files.length === 0 ? (
                        <span className="text-[11px] text-slate-600 italic">Keine Dateien zugeordnet</span>
                      ) : (
                        layer.files.map(f => (
                          <button
                            key={f}
                            onClick={() => onJumpToFileAndLine?.(f, 1)}
                            className="px-2 py-0.5 rounded bg-white/5 hover:bg-white/10 text-slate-300 text-[11px] font-mono border border-white/5 flex items-center gap-1 transition-colors"
                          >
                            <FileCode className="w-3 h-3 text-cyan-400" />
                            <span>{f}</span>
                          </button>
                        ))
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* ASCII Architecture Diagram */}
            <div className="p-5 rounded-2xl bg-black/70 border border-white/10 space-y-2">
              <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider">Live Systemarchitektur-Diagramm</h3>
              <pre className="p-4 rounded-xl bg-black font-mono text-[11px] text-cyan-300 overflow-x-auto whitespace-pre leading-relaxed border border-white/5">
                {liveDoc.asciiDiagram}
              </pre>
            </div>

            {/* Module Interfaces & Security Rating */}
            <div className="p-5 rounded-2xl bg-black/40 border border-white/10 space-y-3">
              <h3 className="text-xs font-bold text-slate-300 uppercase tracking-wider">Modul-Schnittstellen & Audit-Status</h3>
              <div className="overflow-x-auto">
                <table className="w-full text-left text-xs">
                  <thead>
                    <tr className="border-b border-white/10 text-slate-400 font-mono">
                      <th className="pb-2">Modul</th>
                      <th className="pb-2">Rolle</th>
                      <th className="pb-2">Zeilen</th>
                      <th className="pb-2">Öffentliche Exports</th>
                      <th className="pb-2">Sicherheitsbewertung</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5">
                    {liveDoc.modules.map((mod) => (
                      <tr key={mod.fileName} className="hover:bg-white/[0.02]">
                        <td className="py-2.5 font-mono text-cyan-400 font-semibold">{mod.fileName}</td>
                        <td className="py-2.5 text-slate-300">{mod.role}</td>
                        <td className="py-2.5 font-mono text-slate-400">{mod.lines}</td>
                        <td className="py-2.5 font-mono text-slate-300">
                          {mod.exports.length > 0 ? mod.exports.join(', ') : '-'}
                        </td>
                        <td className="py-2.5">
                          {mod.securityRating === 'Secure' ? (
                            <span className="text-emerald-400 font-semibold flex items-center gap-1">
                              <ShieldCheck className="w-3.5 h-3.5" /> Sicher
                            </span>
                          ) : mod.securityRating === 'Needs Review' ? (
                            <span className="text-amber-400 font-semibold flex items-center gap-1">
                              <AlertTriangle className="w-3.5 h-3.5" /> Überprüfung nötig
                            </span>
                          ) : (
                            <span className="text-red-400 font-semibold flex items-center gap-1">
                              <AlertOctagon className="w-3.5 h-3.5" /> Verwundbar
                            </span>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* VIEW MODE 3: HARDWARE & KERN-PARADIGMEN */}
      {wikiMode === 'system' && (
        <div className="flex flex-1 overflow-hidden">
          {/* Sidebar */}
          <div className="w-72 border-r border-white/10 bg-black/20 overflow-y-auto shrink-0">
            <div className="p-4">
              <div className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">Architektur-Kapitel</div>
              <div className="space-y-1">
                {sections.map((sec, idx) => (
                  <button
                    key={idx}
                    onClick={() => setActiveSection(idx)}
                    className={`w-full text-left px-3 py-2 rounded-lg text-xs flex items-center gap-3 transition-colors ${
                      activeSection === idx
                        ? 'bg-emerald-500/15 text-emerald-300 font-semibold border border-emerald-500/30'
                        : 'text-slate-400 hover:bg-white/5 hover:text-slate-200'
                    }`}
                  >
                    <div className={`p-1.5 rounded ${activeSection === idx ? 'bg-emerald-500/20' : 'bg-white/5'}`}>
                      {sec.icon}
                    </div>
                    <span className="truncate">{sec.title}</span>
                  </button>
                ))}
              </div>

              <div className="text-xs font-bold text-slate-500 uppercase tracking-widest mt-6 mb-3">Hardware & Tech Stacks</div>
              <div className="space-y-2 px-2">
                {stacks.map(stack => (
                  <div key={stack.area} className="text-xs border-l-2 border-emerald-500/30 pl-2 py-1 bg-white/[0.01] rounded-r">
                    <div className="font-bold text-slate-300">{stack.area}</div>
                    <div className="text-slate-500 text-[11px] line-clamp-1 opacity-80" title={stack.tech}>{stack.tech}</div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Content */}
          <div className="flex-1 overflow-y-auto p-6 md:p-8 relative bg-[#08080a]">
            <motion.div
              key={activeSection}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              className="max-w-4xl relative z-10"
            >
              <div className="flex items-center gap-4 mb-6">
                <div className="p-4 bg-white/5 border border-white/10 rounded-2xl shadow-xl">
                  {React.cloneElement(sections[activeSection].icon as any, { className: "w-8 h-8" })}
                </div>
                <div>
                  <h1 className="text-2xl md:text-3xl font-bold text-slate-100">{sections[activeSection].title}</h1>
                  <span className="text-xs text-slate-500 font-mono mt-1 block">ATOS Hardware & Systems Architecture Reference</span>
                </div>
              </div>

              <div className="bg-black/40 border border-white/10 p-6 rounded-xl text-base text-slate-300 leading-relaxed shadow-inner mb-6">
                {sections[activeSection].content}
              </div>

              {/* If section is Monolith / Hierarchy / Layer / Modular, show detailed specs */}
              {activeSection >= 1 && activeSection <= 4 && (
                <div className="mt-6 space-y-4">
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div className="bg-black/30 border border-emerald-500/20 p-4 rounded-xl">
                      <h4 className="text-xs font-bold uppercase tracking-wider text-emerald-400 mb-2">Vorteile (Pros)</h4>
                      <ul className="space-y-1 text-xs text-slate-300 list-disc list-inside">
                        {Object.values(PARADIGMS)[activeSection - 1]?.pros.map((pro, i) => (
                          <li key={i}>{pro}</li>
                        ))}
                      </ul>
                    </div>

                    <div className="bg-black/30 border border-amber-500/20 p-4 rounded-xl">
                      <h4 className="text-xs font-bold uppercase tracking-wider text-amber-400 mb-2">Kompromisse (Cons)</h4>
                      <ul className="space-y-1 text-xs text-slate-300 list-disc list-inside">
                        {Object.values(PARADIGMS)[activeSection - 1]?.cons.map((con, i) => (
                          <li key={i}>{con}</li>
                        ))}
                      </ul>
                    </div>
                  </div>

                  <div className="bg-black/60 border border-white/10 rounded-xl p-4">
                    <h4 className="text-xs font-bold uppercase tracking-wider text-slate-400 mb-2">Architektur-Diagramm</h4>
                    <pre className="font-mono text-[11px] text-cyan-300 bg-black/80 p-4 rounded-lg overflow-x-auto whitespace-pre leading-tight border border-white/5">
                      {Object.values(PARADIGMS)[activeSection - 1]?.diagram}
                    </pre>
                  </div>
                </div>
              )}

              {/* If section is Hardware Architecture (x86_64, ARM64, RISC-V) */}
              {activeSection >= 5 && activeSection <= 8 && (
                <div className="mt-6 bg-black/40 border border-white/10 rounded-xl p-5">
                  <h3 className="text-sm font-bold text-cyan-400 mb-3 uppercase tracking-wider">Hardware-Spezifikation</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs font-mono">
                    <div className="bg-white/5 p-3 rounded-lg border border-white/5">
                      <span className="text-slate-500 block text-[10px] uppercase font-bold">Privilege Levels</span>
                      <span className="text-slate-200 mt-1 block">
                        {Object.values(HARDWARE_ARCHITECTURES)[activeSection - 5]?.privilegeLevels.join(" • ")}
                      </span>
                    </div>
                    <div className="bg-white/5 p-3 rounded-lg border border-white/5">
                      <span className="text-slate-500 block text-[10px] uppercase font-bold">Kernregister</span>
                      <span className="text-cyan-300 mt-1 block">
                        {Object.values(HARDWARE_ARCHITECTURES)[activeSection - 5]?.registers.slice(0, 10).join(", ")}...
                      </span>
                    </div>
                    <div className="bg-white/5 p-3 rounded-lg border border-white/5">
                      <span className="text-slate-500 block text-[10px] uppercase font-bold">Paging & Adressräume</span>
                      <span className="text-slate-200 mt-1 block">
                        {Object.values(HARDWARE_ARCHITECTURES)[activeSection - 5]?.pagingModel}
                      </span>
                    </div>
                    <div className="bg-white/5 p-3 rounded-lg border border-white/5">
                      <span className="text-slate-500 block text-[10px] uppercase font-bold">Interrupt Controller</span>
                      <span className="text-slate-200 mt-1 block">
                        {Object.values(HARDWARE_ARCHITECTURES)[activeSection - 5]?.interruptController}
                      </span>
                    </div>
                  </div>
                </div>
              )}
            </motion.div>
          </div>
        </div>
      )}
    </div>
  );
}
