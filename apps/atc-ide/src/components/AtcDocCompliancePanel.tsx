import React, { useState, useMemo } from 'react';
import {
  ShieldCheck,
  ShieldAlert,
  AlertTriangle,
  CheckCircle2,
  FileText,
  FolderGit2,
  GitBranch,
  FileCode,
  Wand2,
  Sparkles,
  RefreshCw,
  ExternalLink,
  ChevronRight,
  Download,
  Info,
  Check,
  Zap,
  Lock,
  Boxes,
  Layers,
  Scale
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { FileState } from '../utils/auditEngine';
import {
  REPOSITORY_STANDARD_FILES,
  ATC_DOC_STANDARDS,
  ADR_REGISTRY,
  ATC_PROTOCOL_STANDARDS
} from '../data/atcRepoDocumentationStandards';

interface AtcDocCompliancePanelProps {
  files: FileState[];
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export interface ComplianceCheckResult {
  id: string;
  category: 'root' | 'github' | 'standards' | 'adr' | 'manifests';
  title: string;
  description: string;
  expectedPath: string;
  isMandatory: boolean;
  isCompliant: boolean;
  statusText: string;
  missingSections?: string[];
  remedyAction?: string;
  fileTemplate?: string;
}

export function AtcDocCompliancePanel({
  files,
  onSaveWorkspaceFile,
  onShowToast
}: AtcDocCompliancePanelProps) {
  const [activeCategory, setActiveCategory] = useState<'all' | 'root' | 'github' | 'standards' | 'adr' | 'manifests'>('all');
  const [isFixing, setIsFixing] = useState<boolean>(false);

  // File lookup map
  const fileMap = useMemo(() => {
    const map = new Map<string, FileState>();
    files.forEach((f) => {
      map.set(f.name.toLowerCase(), f);
    });
    return map;
  }, [files]);

  // Mandatory 14 normative standard sections according to ATC-DOC-003
  const NORMATIVE_14_SECTIONS = useMemo(() => [
    'Abstract',
    'Motivation',
    'Specification',
    'Terminology',
    'Data Structures',
    'Encoding',
    'State Transitions',
    'Validation Rules',
    'Error Conditions',
    'Security Considerations',
    'Compatibility',
    'Test Vectors',
    'Reference Implementation',
    'Changelog'
  ], []);

  // Run comprehensive compliance check
  const complianceResults = useMemo<ComplianceCheckResult[]>(() => {
    const results: ComplianceCheckResult[] = [];

    // 1. Check Root Mandatory & Recommended Files
    const rootFiles = [
      { path: 'README.md', mandatory: true, title: 'Repository Readme & Overview' },
      { path: 'LICENSE', mandatory: true, title: 'Open Source License (Dual MIT/Apache)' },
      { path: 'SECURITY.md', mandatory: true, title: 'Vulnerability Disclosure Policy' },
      { path: '.gitignore', mandatory: true, title: 'Git Ignore Configuration' },
      { path: 'CONTRIBUTING.md', mandatory: false, title: 'Contributor Guidelines & DCO' },
      { path: 'CODE_OF_CONDUCT.md', mandatory: false, title: 'Contributor Covenant v2.1' },
      { path: 'CHANGELOG.md', mandatory: false, title: 'Keep a Changelog (SemVer)' },
      { path: 'ROADMAP.md', mandatory: false, title: 'Development Milestones' },
      { path: 'AUTHORS.md', mandatory: false, title: 'Authors & Working Groups' },
      { path: 'NOTICE', mandatory: false, title: 'Legal & Third-Party Notices' },
    ];

    rootFiles.forEach((rf) => {
      const match = fileMap.get(rf.path.toLowerCase());
      const stdDoc = REPOSITORY_STANDARD_FILES.find((s) => s.path === rf.path);
      const isPresent = !!match && match.content.trim().length > 20;

      results.push({
        id: `root-${rf.path}`,
        category: 'root',
        title: rf.title,
        description: `Pfad: ${rf.path} im Root-Verzeichnis`,
        expectedPath: rf.path,
        isMandatory: rf.mandatory,
        isCompliant: isPresent,
        statusText: isPresent ? 'Vorhanden & Vollständig' : rf.mandatory ? 'Kritisch: Datei fehlt' : 'Empfohlen: Datei fehlt',
        remedyAction: `Generiere ${rf.path}`,
        fileTemplate: stdDoc?.content || `# ${rf.title}\n\nStandard placeholder content for ${rf.path}.`,
      });
    });

    // 2. Check GitHub Governance Suite (.github/)
    const ghFiles = [
      { path: '.github/CODEOWNERS', mandatory: true, title: 'Code Ownership & Review Gates' },
      { path: '.github/PULL_REQUEST_TEMPLATE.md', mandatory: true, title: 'Pull Request Review Template' },
      { path: '.github/workflows/ci.yml', mandatory: true, title: 'CI/CD Automated Test Pipeline' },
      { path: '.github/dependabot.yml', mandatory: false, title: 'Automated Dependency Security Audits' },
      { path: '.github/ISSUE_TEMPLATE/bug_report.md', mandatory: false, title: 'Standard Bug Report Template' },
      { path: '.github/ISSUE_TEMPLATE/security_issue.md', mandatory: true, title: 'Security Vulnerability Form' },
    ];

    ghFiles.forEach((gf) => {
      const match = fileMap.get(gf.path.toLowerCase());
      const stdDoc = REPOSITORY_STANDARD_FILES.find((s) => s.path === gf.path);
      const isPresent = !!match && match.content.trim().length > 20;

      results.push({
        id: `gh-${gf.path}`,
        category: 'github',
        title: gf.title,
        description: `GitHub Governance: ${gf.path}`,
        expectedPath: gf.path,
        isMandatory: gf.mandatory,
        isCompliant: isPresent,
        statusText: isPresent ? 'Konfiguriert' : gf.mandatory ? 'Fehlt: Review-Gate ungeschützt' : 'Empfohlen',
        remedyAction: `Erstelle ${gf.path}`,
        fileTemplate: stdDoc?.content || `# ${gf.title}\n\nConfiguration for ${gf.path}`,
      });
    });

    // 3. Check ATC Protocol Standards & 14-Section Normative Compliance
    const standardSpecs = [
      { path: 'docs/standards/ATC-0001_CORE_IDENTITY.md', id: 'ATC-0001', title: 'Decentralized Core Identity' },
      { path: 'docs/standards/ATC-0002_ANS_RESOLVER.md', id: 'ATC-0002', title: 'A-Town Naming Service (ANS)' },
      { path: 'docs/standards/ATC-0003_ADDRESS_FORMAT.md', id: 'ATC-0003', title: 'Cryptographic Bech32m Address Format' },
      { path: 'docs/standards/ATC-0004_TOKEN_STANDARDS.md', id: 'ATC-0004', title: 'Multi-Token & Capability Standard' },
      { path: 'docs/standards/ATC-0005_TRANSACTION_FORMAT.md', id: 'ATC-0005', title: 'Transaction Wire Envelope & Fees' },
      { path: 'docs/standards/ATC-0094_VM_SPEC.md', id: 'ATC-0094', title: 'ATC-VM Virtual Machine Architecture' },
    ];

    standardSpecs.forEach((spec) => {
      const match = fileMap.get(spec.path.toLowerCase());
      const stdDoc = REPOSITORY_STANDARD_FILES.find((s) => s.path === spec.path);
      const isPresent = !!match && match.content.trim().length > 50;

      let missingSections: string[] = [];
      if (isPresent && match) {
        // Scan for 14 required sections
        missingSections = NORMATIVE_14_SECTIONS.filter((sec) => {
          const reg = new RegExp(`##\\s*\\d*\\.?\\s*${sec}`, 'i');
          return !reg.test(match.content);
        });
      }

      const isCompliant = isPresent && missingSections.length === 0;

      results.push({
        id: `std-${spec.id}`,
        category: 'standards',
        title: `${spec.id}: ${spec.title}`,
        description: `Normative Spezifikation nach ATC-DOC-003 (${spec.path})`,
        expectedPath: spec.path,
        isMandatory: true,
        isCompliant,
        statusText: !isPresent
          ? 'Spezifikation fehlt im Workspace'
          : missingSections.length > 0
          ? `${missingSections.length} von 14 Kapiteln fehlen`
          : '14/14 Normative Kapitel verifiziert',
        missingSections,
        remedyAction: `Scaffold ${spec.id}`,
        fileTemplate: stdDoc?.content || `# ${spec.id}: ${spec.title}\n\nStandard specification.`,
      });
    });

    // 4. Check Architecture Decision Records (ADRs 0001 - 0006)
    const adrFiles = [
      { path: 'docs/adr/ADR-0001-rust-implementation.md', id: 'ADR-0001', title: 'Rust as Core Kernel & VM Language' },
      { path: 'docs/adr/ADR-0002-microkernel-capabilities.md', id: 'ADR-0002', title: 'Microkernel Architecture with Capabilities' },
      { path: 'docs/adr/ADR-0003-dual-engine-wasm-atcvm.md', id: 'ADR-0003', title: 'Dual-Engine WebAssembly & ATC-VM Sandbox' },
      { path: 'docs/adr/ADR-0004-fixed-width-bytecode.md', id: 'ADR-0004', title: 'Fixed-Width Bytecode with Explicit Gas' },
      { path: 'docs/adr/ADR-0005-tendermint-bft-consensus.md', id: 'ADR-0005', title: 'Tendermint-Derived Fast-Finality Consensus' },
      { path: 'docs/adr/ADR-0006-tiered-merkle-patricia-trie.md', id: 'ADR-0006', title: 'Tiered Merkle Patricia Trie Storage' },
    ];

    adrFiles.forEach((adr) => {
      const match = fileMap.get(adr.path.toLowerCase());
      const stdDoc = REPOSITORY_STANDARD_FILES.find((s) => s.path === adr.path);
      const isPresent = !!match && match.content.trim().length > 50;

      results.push({
        id: `adr-${adr.id}`,
        category: 'adr',
        title: `${adr.id}: ${adr.title}`,
        description: `Architecture Decision Record (${adr.path})`,
        expectedPath: adr.path,
        isMandatory: true,
        isCompliant: isPresent,
        statusText: isPresent ? 'Accepted & Dokumentiert' : 'ADR fehlt im Repository',
        remedyAction: `Scaffold ${adr.id}`,
        fileTemplate: stdDoc?.content || `# ${adr.id}: ${adr.title}\n\nArchitecture Decision Record.`,
      });
    });

    // 5. Check Documentation Indexes & Cross-References
    const manifestFiles = [
      { path: 'docs/DOCUMENTATION_INDEX.md', title: 'Zentraler Dokumentations-Index' },
      { path: 'docs/VERSION_MATRIX.md', title: 'Crate- & Protokoll-Versionsmatrix' },
      { path: 'docs/FILE_REGISTER.md', title: 'Kanonisches Datei-Register' },
    ];

    manifestFiles.forEach((mf) => {
      const match = fileMap.get(mf.path.toLowerCase());
      const stdDoc = REPOSITORY_STANDARD_FILES.find((s) => s.path === mf.path);
      const isPresent = !!match && match.content.trim().length > 50;

      results.push({
        id: `manifest-${mf.path}`,
        category: 'manifests',
        title: mf.title,
        description: `Repository-Manifest (${mf.path})`,
        expectedPath: mf.path,
        isMandatory: true,
        isCompliant: isPresent,
        statusText: isPresent ? 'Index aktuell' : 'Manifest fehlt',
        remedyAction: `Generiere ${mf.path}`,
        fileTemplate: stdDoc?.content || `# ${mf.title}\n\nRepository Manifest.`,
      });
    });

    return results;
  }, [fileMap, files, NORMATIVE_14_SECTIONS]);

  // Aggregate Metrics
  const metrics = useMemo(() => {
    const total = complianceResults.length;
    const compliant = complianceResults.filter((r) => r.isCompliant).length;
    const mandatoryTotal = complianceResults.filter((r) => r.isMandatory).length;
    const mandatoryCompliant = complianceResults.filter((r) => r.isMandatory && r.isCompliant).length;

    const score = Math.round((compliant / total) * 100);
    const mandatoryScore = Math.round((mandatoryCompliant / mandatoryTotal) * 100);

    return {
      total,
      compliant,
      missing: total - compliant,
      score,
      mandatoryScore,
      grade: score >= 90 ? 'A+' : score >= 75 ? 'A' : score >= 60 ? 'B' : score >= 40 ? 'C' : 'D',
    };
  }, [complianceResults]);

  // Filtered by category
  const filteredResults = useMemo(() => {
    if (activeCategory === 'all') return complianceResults;
    return complianceResults.filter((r) => r.category === activeCategory);
  }, [complianceResults, activeCategory]);

  // Auto-Fix single item
  const handleFixItem = (result: ComplianceCheckResult) => {
    if (!onSaveWorkspaceFile) {
      onShowToast?.('Kein Workspace-Handler verfügbar.', 'error');
      return;
    }
    const template = result.fileTemplate || `# ${result.title}\n\nInitial content.`;
    onSaveWorkspaceFile(result.expectedPath, template);
    onShowToast?.(`Datei '${result.expectedPath}' erfolgreich im Workspace angelegt.`, 'info');
  };

  // Auto-Fix ALL missing items
  const handleFixAll = () => {
    if (!onSaveWorkspaceFile) {
      onShowToast?.('Kein Workspace-Handler verfügbar.', 'error');
      return;
    }

    setIsFixing(true);
    let count = 0;

    complianceResults.forEach((item) => {
      if (!item.isCompliant && item.fileTemplate) {
        onSaveWorkspaceFile(item.expectedPath, item.fileTemplate);
        count++;
      }
    });

    setIsFixing(false);
    onShowToast?.(`${count} fehlende Standard-Dokumente erfolgreich generiert!`, 'info');
  };

  return (
    <div className="flex-1 flex flex-col bg-[#0b0f19] text-slate-200 overflow-hidden font-sans">
      {/* Top Header Bar */}
      <div className="flex flex-wrap items-center justify-between px-4 py-3 bg-[#0d1322] border-b border-white/10 gap-3 shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-lg bg-emerald-500/20 border border-emerald-500/40 flex items-center justify-center text-emerald-400">
            <ShieldCheck className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-sm font-bold text-white tracking-wide">ATC-DOC Linter & Compliance Suite</h2>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono font-bold bg-emerald-500/20 text-emerald-300 border border-emerald-500/40">
                ATC-DOC-001..008
              </span>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono bg-blue-500/20 text-blue-300 border border-blue-500/40">
                14 Normative Kapitel
              </span>
            </div>
            <p className="text-xs text-slate-400">
              Validiert Root-Dateien, GitHub-Governance, ADR-Integrität und normative Protokoll-Standards
            </p>
          </div>
        </div>

        {/* Action Button: Auto-Fix All */}
        <div className="flex items-center gap-2">
          {onSaveWorkspaceFile && (
            <button
              onClick={handleFixAll}
              disabled={isFixing || metrics.missing === 0}
              className="px-3.5 py-1.5 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white rounded-lg text-xs font-bold flex items-center gap-1.5 shadow-lg shadow-emerald-600/20 transition-all cursor-pointer"
            >
              <Wand2 className="w-3.5 h-3.5" />
              <span>{isFixing ? 'Wird generiert...' : `Alle ${metrics.missing} beheben (1-Klick Auto-Fix)`}</span>
            </button>
          )}
        </div>
      </div>

      {/* Compliance Scoreboard Banner */}
      <div className="grid grid-cols-1 sm:grid-cols-4 gap-3 p-4 bg-black/40 border-b border-white/10 text-xs shrink-0">
        <div className="p-3 rounded-lg bg-white/5 border border-white/5 flex items-center justify-between">
          <div>
            <span className="text-slate-400 font-medium">Compliance-Score</span>
            <div className="text-xl font-bold font-mono text-emerald-400 flex items-center gap-1.5 mt-0.5">
              <span>{metrics.score}%</span>
              <span className="text-xs px-1.5 py-0.2 rounded bg-emerald-500/20 border border-emerald-500/30">
                Note {metrics.grade}
              </span>
            </div>
          </div>
          <div className="w-10 h-10 rounded-full border-2 border-emerald-500/40 flex items-center justify-center font-bold text-emerald-400 font-mono">
            {metrics.compliant}/{metrics.total}
          </div>
        </div>

        <div className="p-3 rounded-lg bg-white/5 border border-white/5">
          <span className="text-slate-400 font-medium">Mandatory Standards</span>
          <div className="text-xl font-bold font-mono text-cyan-400 mt-0.5">
            {metrics.mandatoryScore}% Konform
          </div>
          <p className="text-[10px] text-slate-500 mt-1">Strikte Pflichtprüfungen</p>
        </div>

        <div className="p-3 rounded-lg bg-white/5 border border-white/5">
          <span className="text-slate-400 font-medium">Fehlende Artefakte</span>
          <div className={`text-xl font-bold font-mono mt-0.5 ${metrics.missing > 0 ? 'text-amber-400' : 'text-emerald-400'}`}>
            {metrics.missing} Dokumente
          </div>
          <p className="text-[10px] text-slate-500 mt-1">Automatisch behebbar</p>
        </div>

        <div className="p-3 rounded-lg bg-white/5 border border-white/5">
          <span className="text-slate-400 font-medium">Normen-Struktur</span>
          <div className="text-xl font-bold font-mono text-indigo-400 mt-0.5">
            14-Kapitel Standard
          </div>
          <p className="text-[10px] text-slate-500 mt-1">ATC-DOC-003 Spezifikationsnorm</p>
        </div>
      </div>

      {/* Category Tabs */}
      <div className="flex items-center px-4 py-2 bg-[#0c1222] border-b border-white/10 gap-1 overflow-x-auto text-xs shrink-0">
        {[
          { id: 'all', label: 'Alle Prüfungen', count: complianceResults.length },
          { id: 'root', label: '1. Root Pflichtdateien', count: complianceResults.filter((r) => r.category === 'root').length },
          { id: 'github', label: '2. GitHub Governance', count: complianceResults.filter((r) => r.category === 'github').length },
          { id: 'standards', label: '3. Normative Standards', count: complianceResults.filter((r) => r.category === 'standards').length },
          { id: 'adr', label: '4. ADRs (0001-0006)', count: complianceResults.filter((r) => r.category === 'adr').length },
          { id: 'manifests', label: '5. Repository Manifeste', count: complianceResults.filter((r) => r.category === 'manifests').length },
        ].map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveCategory(tab.id as any)}
            className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 whitespace-nowrap ${
              activeCategory === tab.id
                ? 'bg-emerald-500/25 text-emerald-300 border border-emerald-500/40 shadow-sm'
                : 'text-slate-400 hover:text-slate-200'
            }`}
          >
            <span>{tab.label}</span>
            <span className="px-1.5 py-0.2 rounded-full text-[10px] bg-white/10 font-mono">
              {tab.count}
            </span>
          </button>
        ))}
      </div>

      {/* Findings List */}
      <div className="flex-1 overflow-y-auto p-4 space-y-2.5">
        {filteredResults.map((res) => {
          return (
            <div
              key={res.id}
              className={`p-3.5 rounded-xl border transition-all flex flex-col sm:flex-row sm:items-center justify-between gap-3 ${
                res.isCompliant
                  ? 'bg-white/[0.02] border-white/5 hover:border-white/10'
                  : res.isMandatory
                  ? 'bg-red-950/20 border-red-500/30 hover:border-red-500/40'
                  : 'bg-amber-950/20 border-amber-500/30 hover:border-amber-500/40'
              }`}
            >
              <div className="flex items-start gap-3 min-w-0">
                <div className="mt-0.5">
                  {res.isCompliant ? (
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                  ) : res.isMandatory ? (
                    <ShieldAlert className="w-4 h-4 text-red-400" />
                  ) : (
                    <AlertTriangle className="w-4 h-4 text-amber-400" />
                  )}
                </div>

                <div className="min-w-0">
                  <div className="flex items-center gap-2 flex-wrap">
                    <span className="font-bold text-xs text-white tracking-wide">
                      {res.title}
                    </span>
                    <span className="font-mono text-[10px] text-slate-400 bg-white/5 px-1.5 py-0.5 rounded border border-white/5">
                      {res.expectedPath}
                    </span>
                    {res.isMandatory && (
                      <span className="px-1.5 py-0.2 rounded text-[9px] font-bold bg-red-500/20 text-red-300 border border-red-500/30">
                        PFLICHT
                      </span>
                    )}
                  </div>

                  <p className="text-[11px] text-slate-400 mt-0.5">
                    {res.description}
                  </p>

                  {/* Missing 14-section warning */}
                  {res.missingSections && res.missingSections.length > 0 && (
                    <div className="mt-1.5 text-[10px] text-amber-300 bg-amber-500/10 border border-amber-500/20 rounded p-1.5">
                      <span className="font-bold">Fehlende normative Kapitel: </span>
                      <span>{res.missingSections.join(', ')}</span>
                    </div>
                  )}
                </div>
              </div>

              {/* Status & Remedy Button */}
              <div className="flex items-center gap-3 self-end sm:self-center shrink-0">
                <span
                  className={`text-xs font-semibold ${
                    res.isCompliant
                      ? 'text-emerald-400'
                      : res.isMandatory
                      ? 'text-red-400'
                      : 'text-amber-400'
                  }`}
                >
                  {res.statusText}
                </span>

                {!res.isCompliant && onSaveWorkspaceFile && (
                  <button
                    onClick={() => handleFixItem(res)}
                    className="px-2.5 py-1 bg-white/10 hover:bg-white/20 text-slate-200 border border-white/10 rounded-md text-xs font-semibold flex items-center gap-1 transition-colors cursor-pointer"
                  >
                    <Wand2 className="w-3 h-3 text-cyan-400" />
                    <span>Beheben</span>
                  </button>
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
