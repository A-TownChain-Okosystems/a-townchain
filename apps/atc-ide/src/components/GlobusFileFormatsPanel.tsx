import React, { useState, useMemo, useEffect, useRef } from 'react';
import {
  FileCode,
  FolderArchive,
  ShieldCheck,
  Binary,
  Cpu,
  Layers,
  Search,
  Download,
  Copy,
  Check,
  CheckCircle2,
  AlertTriangle,
  Play,
  FileCheck,
  Lock,
  Database,
  Bot,
  Terminal,
  HardDrive,
  Boxes,
  ArrowRight,
  ExternalLink,
  ChevronRight,
  FileText,
  Package,
  Upload,
  RefreshCw,
  Zap,
  Sliders,
  XCircle,
} from 'lucide-react';
import {
  GLOBUS_FILE_FORMATS,
  GFF_HEADER_FIELDS,
  GFFA_SPEC_MARKDOWN,
  FormatDomain,
  GffFormatDefinition,
} from '../data/globusFileFormats';
import {
  buildGffBinary,
  parseGffBinary,
  createRealGappBundle,
  unpackRealGappBundle,
  verifyGffArtifactReal,
  triggerBinaryDownload,
  CAPABILITY_BITS,
  GffBinaryArtifact,
  VerificationReport,
} from '../utils/gffCodec';

interface GlobusFileFormatsPanelProps {
  onSaveWorkspaceFile?: (filename: string, content: string) => void;
  onShowToast?: (title: string, message: string) => void;
}

export const GlobusFileFormatsPanel: React.FC<GlobusFileFormatsPanelProps> = ({
  onSaveWorkspaceFile,
  onShowToast,
}) => {
  const [activeTab, setActiveTab] = useState<'catalog' | 'header' | 'bundle' | 'pipeline' | 'vfs'>('catalog');
  const [selectedDomain, setSelectedDomain] = useState<FormatDomain | 'All'>('All');
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedFormat, setSelectedFormat] = useState<GffFormatDefinition>(GLOBUS_FILE_FORMATS[0]);
  const [copiedKey, setCopiedKey] = useState<string | null>(null);

  // Real Binary Builder / Hex State
  const [hexFormatTarget, setHexFormatTarget] = useState<string>('.gexe');
  const [selectedArch, setSelectedArch] = useState<number>(0x0001); // 1: x86_64, 2: aarch64, 3: riscv64, 4: atcvm
  const [selectedFlags, setSelectedFlags] = useState({
    signed: true,
    compressed: false,
    sandbox: true,
    encrypted: false,
    stripped: true,
    deterministic: true,
  });
  const [selectedCaps, setSelectedCaps] = useState<number[]>([0, 1, 2, 4, 6]);
  const [selectedHexByteIndex, setSelectedHexByteIndex] = useState<number | null>(0);
  const [currentArtifact, setCurrentArtifact] = useState<GffBinaryArtifact | null>(null);
  const [isMutated, setIsMutated] = useState(false);

  // Verifier State (Real Verification Engine)
  const [verifierRunning, setVerifierRunning] = useState(false);
  const [verificationReport, setVerificationReport] = useState<VerificationReport | null>(null);
  const [verifierTargetName, setVerifierTargetName] = useState<string>('GenesisExplorer.gexe');
  const [customFileBuffer, setCustomFileBuffer] = useState<Uint8Array | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  // Bundle Builder & Unpacker State
  const [bundleManifestYaml, setBundleManifestYaml] = useState<string>(`format: GAPP
version: "1.0.0"
application:
  id: "org.atc.genesis"
  name: "Genesis Explorer"
  version: "1.2.0"
  publisher: "ATC Core Foundation"
  license: "Apache-2.0"
runtime:
  abi: "globus-abi-v1"
  architectures: ["x86_64", "aarch64"]
  min_shivacore_version: "2.0.0"
entrypoint:
  executable: "app.gexe"
  args: ["--mode=desktop"]
capabilities:
  - "graphics.vulkan.1_3"
  - "audio.output"
  - "storage.user_documents"
  - "net.client"
  - "ipc.connect_system"
security:
  signed: true
  signature_file: "signature.gsig"
  sandbox_profile: "strict_app_v1"
  hash_algorithm: "sha256"
`);
  const [bundleBuilding, setBundleBuilding] = useState(false);
  const [bundleFiles, setBundleFiles] = useState<{ path: string; size: number; desc: string }[]>([
    { path: 'manifest.gmanifest', size: 480, desc: 'YAML Metadaten, Capabilities & ABI-Spezifikation' },
    { path: 'app.gexe', size: 1024, desc: 'ShivaCore Native Binary mit 64-Byte GFF Header' },
    { path: 'libraries/README.txt', size: 75, desc: 'Dynamische Bibliotheken (.gdll Verzeichnis)' },
    { path: 'assets/icon.svg', size: 210, desc: 'Vektor-Icon der Anwendung' },
    { path: 'signature.gsig', size: 32, desc: 'Kryptografische HMAC-SHA256 Signatur' },
  ]);
  const [unpackedInspect, setUnpackedInspect] = useState<{
    fileList: string[];
    manifestText?: string;
    parsedHeader?: string;
  } | null>(null);

  // Synchronously generate real binary buffer when options change
  const rebuildBinary = async (corrupt = false) => {
    const subTag = hexFormatTarget.replace('.', '').toUpperCase();
    const artifact = await buildGffBinary({
      subTag,
      archCode: selectedArch,
      flags: selectedFlags,
      capabilityBits: selectedCaps,
      manifestYaml: bundleManifestYaml,
    });

    if (corrupt) {
      // Mutate: change byte 0 from 0x47 to 0xFF or corrupt hash trunk byte
      const corruptedBytes = new Uint8Array(artifact.rawBytes);
      corruptedBytes[56] ^= 0xff; // Flip first byte of checksum trunk
      artifact.rawBytes = corruptedBytes;
      setIsMutated(true);
    } else {
      setIsMutated(false);
    }

    setCurrentArtifact(artifact);
    return artifact;
  };

  useEffect(() => {
    rebuildBinary(false);
  }, [hexFormatTarget, selectedArch, selectedFlags, selectedCaps]);

  const copyToClipboard = (text: string, key: string) => {
    navigator.clipboard.writeText(text);
    setCopiedKey(key);
    if (onShowToast) {
      onShowToast('In Zwischenablage kopiert', key);
    }
    setTimeout(() => setCopiedKey(null), 2500);
  };

  const handleExportSpec = () => {
    if (onSaveWorkspaceFile) {
      onSaveWorkspaceFile('GLOBUS_FILE_FORMAT_SPEC_v1.0.md', GFFA_SPEC_MARKDOWN);
    } else {
      copyToClipboard(GFFA_SPEC_MARKDOWN, 'GFFA Spezifikation');
    }
  };

  const handleExportManifestTemplate = () => {
    if (onSaveWorkspaceFile) {
      onSaveWorkspaceFile('manifest.gmanifest', bundleManifestYaml);
    } else {
      copyToClipboard(bundleManifestYaml, 'Manifest Template');
    }
  };

  // Real Download of the generated binary artifact
  const handleDownloadBinary = () => {
    if (!currentArtifact) return;
    const filename = `app${hexFormatTarget}`;
    triggerBinaryDownload(currentArtifact.rawBytes, filename);
    if (onShowToast) {
      onShowToast('Binärdatei heruntergeladen', `${filename} (${currentArtifact.rawBytes.length} Bytes) gespeichert.`);
    }
  };

  // Real Bundle Generation (.gapp via JSZip)
  const handleBuildAndDownloadGapp = async () => {
    setBundleBuilding(true);
    try {
      const execArtifact = currentArtifact ?? (await rebuildBinary(false));
      const { zipBlob } = await createRealGappBundle({
        appName: 'Genesis Explorer',
        appId: 'org.atc.genesis',
        version: '1.2.0',
        manifestYaml: bundleManifestYaml,
        executableBinary: execArtifact.rawBytes,
      });

      const url = URL.createObjectURL(zipBlob);
      const link = document.createElement('a');
      link.href = url;
      link.download = 'GenesisExplorer.gapp';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);

      if (onShowToast) {
        onShowToast('GAPP-Paket generiert', 'GenesisExplorer.gapp (echtes ZIP-Bundle mit Signaturen) heruntergeladen.');
      }
    } finally {
      setBundleBuilding(false);
    }
  };

  // Save Executable or Manifest to Workspace
  const handleSaveToWorkspace = () => {
    if (!onSaveWorkspaceFile || !currentArtifact) return;
    // Save Manifest
    onSaveWorkspaceFile('manifest.gmanifest', bundleManifestYaml);
    // Convert binary to hex representation for text file in workspace
    const hexRep = Array.from(currentArtifact.rawBytes)
      .map((b) => b.toString(16).padStart(2, '0'))
      .join(' ');
    onSaveWorkspaceFile(`app${hexFormatTarget}.hex`, hexRep);
    if (onShowToast) {
      onShowToast('Im Workspace abgelegt', `manifest.gmanifest & app${hexFormatTarget}.hex wurden synchronisiert.`);
    }
  };

  // Run Real Multi-Step Verification Engine
  const executeRealVerification = async (useCorrupted = false, customBuffer?: Uint8Array) => {
    setVerifierRunning(true);
    try {
      let targetBytes: Uint8Array;
      let targetName = verifierTargetName;

      if (customBuffer) {
        targetBytes = customBuffer;
      } else {
        const artifact = await rebuildBinary(useCorrupted);
        targetBytes = artifact.rawBytes;
        targetName = `GenesisExplorer${hexFormatTarget}`;
      }

      const report = await verifyGffArtifactReal(targetName, targetBytes);
      setVerificationReport(report);

      if (onShowToast) {
        if (report.isValid) {
          onShowToast('Verifikation erfolgreich', `${targetName} erfüllt 100% aller 8 ShivaCore Sicherheitsprüfungen.`);
        } else {
          onShowToast('Verifikation fehlgeschlagen', `Sicherheitsverletzung in ${targetName} festgestellt!`);
        }
      }
    } finally {
      setVerifierRunning(false);
    }
  };

  // Handle Real File Upload for Verification
  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async (event) => {
      const arrayBuffer = event.target?.result as ArrayBuffer;
      if (!arrayBuffer) return;

      const bytes = new Uint8Array(arrayBuffer);
      setCustomFileBuffer(bytes);
      setVerifierTargetName(file.name);

      // Check if it's a zip/gapp bundle
      if (file.name.endsWith('.gapp') || file.name.endsWith('.zip')) {
        try {
          const unpacked = await unpackRealGappBundle(arrayBuffer);
          setUnpackedInspect({
            fileList: unpacked.fileList,
            manifestText: unpacked.manifestText,
            parsedHeader: unpacked.executableBytes ? `app.gexe extrahiert (${unpacked.executableBytes.length} Bytes)` : undefined,
          });
          if (unpacked.executableBytes) {
            executeRealVerification(false, unpacked.executableBytes);
            return;
          }
        } catch (err) {
          console.error('Failed to unpack GAPP zip:', err);
        }
      }

      // Execute real verification on binary
      executeRealVerification(false, bytes);
    };
    reader.readAsArrayBuffer(file);
  };

  const filteredFormats = useMemo(() => {
    return GLOBUS_FILE_FORMATS.filter((f) => {
      const matchesDomain = selectedDomain === 'All' || f.domain === selectedDomain;
      const q = searchQuery.toLowerCase().trim();
      const matchesQuery =
        !q ||
        f.extension.toLowerCase().includes(q) ||
        f.name.toLowerCase().includes(q) ||
        f.description.toLowerCase().includes(q) ||
        f.sampleFileName.toLowerCase().includes(q) ||
        f.magicString.toLowerCase().includes(q);
      return matchesDomain && matchesQuery;
    });
  }, [selectedDomain, searchQuery]);

  return (
    <div className="h-full flex flex-col bg-slate-950 text-slate-100 overflow-hidden select-none font-sans">
      {/* TOP BAR */}
      <div className="p-4 bg-slate-900/90 border-b border-white/10 flex flex-wrap items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-indigo-500 to-cyan-500 flex items-center justify-center text-white shadow-lg shadow-indigo-500/20">
            <Binary className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-lg font-bold text-white tracking-wide">
                Globus File Format Architecture (GFFA)
              </h2>
              <span className="px-2 py-0.5 text-xs font-mono font-bold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 rounded">
                GNFF v1.0
              </span>
              <span className="px-2 py-0.5 text-xs font-mono bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 rounded">
                ATC-DOC-013
              </span>
            </div>
            <p className="text-xs text-slate-400">
              Vollwertige Binär-Codecs, kryptografische Integritätsprüfung, JSZip-Bundling & ShivaCore-Verifier
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={handleExportManifestTemplate}
            className="flex items-center gap-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-bold rounded border border-white/10 transition-colors shadow-xs"
            title="Generiert ein leeres manifest.gmanifest Template"
          >
            <FileCode className="w-3.5 h-3.5 text-sky-400" />
            <span>.gmanifest Vorlage</span>
          </button>
          <button
            onClick={handleExportSpec}
            className="flex items-center gap-1.5 px-3 py-1.5 bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-bold rounded shadow-md transition-colors"
            title="Exportiert die vollständige Spezifikation (ATC-DOC-013) in den Workspace"
          >
            <Download className="w-3.5 h-3.5" />
            <span>Spezifikation exportieren</span>
          </button>
        </div>
      </div>

      {/* NAVIGATION TABS */}
      <div className="flex items-center gap-1 px-4 bg-slate-900/60 border-b border-white/5 text-xs font-medium overflow-x-auto">
        <button
          onClick={() => setActiveTab('catalog')}
          className={`px-4 py-2.5 border-b-2 flex items-center gap-2 transition-colors ${
            activeTab === 'catalog'
              ? 'border-indigo-500 text-indigo-300 font-bold bg-indigo-500/10'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <FolderArchive className="w-4 h-4 text-indigo-400" />
          <span>Format-Katalog (26 Formate)</span>
        </button>

        <button
          onClick={() => setActiveTab('header')}
          className={`px-4 py-2.5 border-b-2 flex items-center gap-2 transition-colors ${
            activeTab === 'header'
              ? 'border-cyan-500 text-cyan-300 font-bold bg-cyan-500/10'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <Binary className="w-4 h-4 text-cyan-400" />
          <span>GFF 64-Byte Header & Live Hex-Editor</span>
        </button>

        <button
          onClick={() => setActiveTab('bundle')}
          className={`px-4 py-2.5 border-b-2 flex items-center gap-2 transition-colors ${
            activeTab === 'bundle'
              ? 'border-purple-500 text-purple-300 font-bold bg-purple-500/10'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <Package className="w-4 h-4 text-purple-400" />
          <span>Paket-Explorer & Bundle-Builder (.gapp)</span>
        </button>

        <button
          onClick={() => setActiveTab('pipeline')}
          className={`px-4 py-2.5 border-b-2 flex items-center gap-2 transition-colors ${
            activeTab === 'pipeline'
              ? 'border-emerald-500 text-emerald-300 font-bold bg-emerald-500/10'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <ShieldCheck className="w-4 h-4 text-emerald-400" />
          <span>ShivaCore Zero-Trust Verifikations-Auditor</span>
        </button>

        <button
          onClick={() => setActiveTab('vfs')}
          className={`px-4 py-2.5 border-b-2 flex items-center gap-2 transition-colors ${
            activeTab === 'vfs'
              ? 'border-amber-500 text-amber-300 font-bold bg-amber-500/10'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          }`}
        >
          <HardDrive className="w-4 h-4 text-amber-400" />
          <span>VFS vs. Format vs. GNFS</span>
        </button>
      </div>

      {/* CONTENT AREA */}
      <div className="flex-1 overflow-y-auto p-4 custom-scrollbar">
        {/* TAB 1: FORMAT CATALOG */}
        {activeTab === 'catalog' && (
          <div className="space-y-4">
            <div className="flex flex-col sm:flex-row items-center justify-between gap-3 bg-slate-900/40 p-3 rounded-lg border border-white/5">
              <div className="flex items-center gap-1.5 overflow-x-auto w-full sm:w-auto">
                {(['All', 'System', 'Application', 'Blockchain', 'AI & Agents', 'Security', 'Configuration'] as const).map(
                  (domain) => {
                    const count =
                      domain === 'All'
                        ? GLOBUS_FILE_FORMATS.length
                        : GLOBUS_FILE_FORMATS.filter((f) => f.domain === domain).length;
                    return (
                      <button
                        key={domain}
                        onClick={() => setSelectedDomain(domain)}
                        className={`px-3 py-1 text-xs font-semibold rounded-full transition-all shrink-0 ${
                          selectedDomain === domain
                            ? 'bg-indigo-600 text-white shadow-sm'
                            : 'bg-slate-800 text-slate-400 hover:bg-slate-700 hover:text-slate-200'
                        }`}
                      >
                        {domain === 'All' ? 'Alle' : domain} ({count})
                      </button>
                    );
                  }
                )}
              </div>

              <div className="relative w-full sm:w-72">
                <Search className="w-4 h-4 absolute left-3 top-2.5 text-slate-500" />
                <input
                  type="text"
                  placeholder="Format suchen (z.B. .gexe, .atcb, Driver)..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-9 pr-3 py-1.5 bg-slate-900 border border-white/10 rounded text-xs text-slate-200 focus:outline-hidden focus:border-indigo-500 placeholder:text-slate-500"
                />
              </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-12 gap-4">
              {/* List */}
              <div className="lg:col-span-5 space-y-2 max-h-[640px] overflow-y-auto pr-1">
                {filteredFormats.map((fmt) => {
                  const isSelected = selectedFormat.extension === fmt.extension;
                  return (
                    <div
                      key={fmt.extension}
                      onClick={() => {
                        setSelectedFormat(fmt);
                        if (['.gexe', '.gapp', '.atcb', '.gmodel', '.gsys'].includes(fmt.extension)) {
                          setHexFormatTarget(fmt.extension);
                        }
                      }}
                      className={`p-3 rounded-lg border transition-all cursor-pointer ${
                        isSelected
                          ? 'bg-indigo-950/60 border-indigo-500 shadow-md ring-1 ring-indigo-500/40'
                          : 'bg-slate-900/40 border-white/5 hover:border-white/15 hover:bg-slate-900/70'
                      }`}
                    >
                      <div className="flex items-center justify-between mb-1">
                        <div className="flex items-center gap-2">
                          <span className="font-mono text-sm font-bold text-cyan-400">
                            {fmt.extension}
                          </span>
                          <span className="text-xs font-semibold text-white truncate max-w-[160px]">
                            {fmt.name}
                          </span>
                        </div>
                        <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-slate-800 text-slate-400 border border-white/5">
                          {fmt.domain}
                        </span>
                      </div>
                      <p className="text-xs text-slate-400 line-clamp-2 leading-relaxed">
                        {fmt.description}
                      </p>
                      <div className="mt-2 flex items-center justify-between text-[11px] font-mono text-slate-500">
                        <span>Magic: {fmt.magicString}</span>
                        <span className="text-indigo-400 font-semibold">{fmt.mimeType}</span>
                      </div>
                    </div>
                  );
                })}
              </div>

              {/* Detail Card */}
              <div className="lg:col-span-7 bg-slate-900/60 rounded-lg border border-white/10 p-5 space-y-5">
                <div className="flex items-start justify-between border-b border-white/10 pb-4">
                  <div>
                    <div className="flex items-center gap-2.5">
                      <span className="text-2xl font-mono font-black text-cyan-300">
                        {selectedFormat.extension}
                      </span>
                      <h3 className="text-lg font-bold text-white">
                        {selectedFormat.name}
                      </h3>
                    </div>
                    <p className="text-xs text-slate-400 mt-1">
                      Primäre Domäne: <span className="text-indigo-300 font-semibold">{selectedFormat.domain}</span> | MIME-Typ: <span className="text-slate-300 font-semibold">{selectedFormat.mimeType}</span>
                    </p>
                  </div>

                  <div className="text-right">
                    <span className="px-2.5 py-1 text-xs font-mono font-bold bg-cyan-500/20 text-cyan-300 border border-cyan-500/30 rounded">
                      Magic: {selectedFormat.magicString}
                    </span>
                    <div className="text-[10px] text-slate-500 font-mono mt-1">
                      Hex: {selectedFormat.magicHex}
                    </div>
                  </div>
                </div>

                <div className="space-y-1">
                  <h4 className="text-xs font-bold uppercase text-slate-400 tracking-wider">Beschreibung & Zweck</h4>
                  <p className="text-xs text-slate-200 leading-relaxed bg-slate-950/40 p-3 rounded border border-white/5">
                    {selectedFormat.description}
                  </p>
                </div>

                <div className="grid grid-cols-2 gap-3 text-xs">
                  <div className="p-3 bg-slate-950/50 rounded border border-white/5 space-y-1">
                    <span className="text-slate-500 font-mono text-[10px] uppercase">Ziel-Konsumenten</span>
                    <div className="font-semibold text-indigo-300">{selectedFormat.targetConsumers.join(', ')}</div>
                  </div>
                  <div className="p-3 bg-slate-950/50 rounded border border-white/5 space-y-1">
                    <span className="text-slate-500 font-mono text-[10px] uppercase">Struktur / Container-Typ</span>
                    <div className="font-semibold text-amber-300">{selectedFormat.containerStructure.join(' → ')}</div>
                  </div>
                </div>

                <div>
                  <h4 className="text-xs font-bold uppercase text-slate-400 tracking-wider mb-2">
                    Container-Sektionen & Schichten
                  </h4>
                  <div className="flex flex-wrap gap-1.5">
                    {selectedFormat.containerStructure.map((sec, i) => (
                      <span
                        key={i}
                        className="px-2.5 py-1 text-xs font-mono bg-slate-800/80 text-slate-300 border border-white/10 rounded"
                      >
                        {sec}
                      </span>
                    ))}
                  </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs">
                  <div className="p-3 bg-emerald-950/20 rounded border border-emerald-500/20 space-y-1">
                    <span className="text-emerald-400 font-bold flex items-center gap-1.5">
                      <ShieldCheck className="w-3.5 h-3.5" />
                      Standard-Capabilities
                    </span>
                    <div className="font-mono text-emerald-200/90 text-[11px]">
                      {selectedFormat.defaultCapabilities.join(', ')}
                    </div>
                  </div>

                  <div className="p-3 bg-cyan-950/20 rounded border border-cyan-500/20 space-y-1">
                    <span className="text-cyan-400 font-bold flex items-center gap-1.5">
                      <CheckCircle2 className="w-3.5 h-3.5" />
                      ShivaCore Verifikations-Schritte
                    </span>
                    <div className="font-mono text-cyan-200/90 text-[11px]">
                      {selectedFormat.verificationSteps.join(' → ')}
                    </div>
                  </div>
                </div>

                <div className="flex items-center justify-between pt-2 border-t border-white/5">
                  <span className="text-xs font-mono text-slate-500">
                    Beispiel-Dateiname: <code className="text-cyan-400">{selectedFormat.sampleFileName}</code>
                  </span>
                  <button
                    onClick={() => copyToClipboard(JSON.stringify(selectedFormat, null, 2), selectedFormat.extension)}
                    className="flex items-center gap-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-bold rounded border border-white/10 transition-colors"
                  >
                    {copiedKey === selectedFormat.extension ? (
                      <Check className="w-3.5 h-3.5 text-emerald-400" />
                    ) : (
                      <Copy className="w-3.5 h-3.5" />
                    )}
                    <span>Format-JSON</span>
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* TAB 2: GFF HEADER & REAL LIVE HEX-EDITOR */}
        {activeTab === 'header' && (
          <div className="space-y-6">
            <div className="bg-gradient-to-r from-indigo-950/40 via-slate-900/60 to-cyan-950/40 p-4 rounded-lg border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
              <div className="flex items-center gap-3">
                <Binary className="w-6 h-6 text-cyan-400" />
                <div>
                  <h3 className="text-sm font-bold text-white">Echter GFF 64-Byte Header & Byte-Puffer</h3>
                  <p className="text-xs text-slate-400">
                    Live serialisierter Uint8Array-Binärpuffer. Jedes Byte ist echt, wird durch WebCrypto gehasht und kann manipuliert werden.
                  </p>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <button
                  onClick={handleDownloadBinary}
                  className="flex items-center gap-1.5 px-3 py-1.5 bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-bold rounded shadow-md transition-colors"
                  title="Lädt die echte Binärdatei als Blob herunter"
                >
                  <Download className="w-3.5 h-3.5" />
                  <span>Echte {hexFormatTarget} herunterladen</span>
                </button>
                <button
                  onClick={handleSaveToWorkspace}
                  className="flex items-center gap-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-bold rounded border border-white/10 transition-colors"
                  title="Speichert Hex-Dump und Manifest im Workspace"
                >
                  <FileCode className="w-3.5 h-3.5 text-indigo-400" />
                  <span>In Workspace speichern</span>
                </button>
              </div>
            </div>

            {/* Live Parameter Controls */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-3 bg-slate-900/60 p-4 rounded-lg border border-white/5 text-xs">
              <div>
                <label className="block text-slate-400 font-mono mb-1">Ziel-Format (SubTag):</label>
                <div className="flex flex-wrap gap-1">
                  {(['.gexe', '.gdll', '.gapp', '.atcb', '.gmodel', '.gsys', '.gdrv'] as const).map((fmt) => (
                    <button
                      key={fmt}
                      onClick={() => setHexFormatTarget(fmt)}
                      className={`px-2 py-1 font-mono font-bold rounded ${
                        hexFormatTarget === fmt
                          ? 'bg-cyan-600 text-white'
                          : 'bg-slate-800 text-slate-400 hover:bg-slate-700'
                      }`}
                    >
                      {fmt}
                    </button>
                  ))}
                </div>
              </div>

              <div>
                <label className="block text-slate-400 font-mono mb-1">Ziel-Architektur (ArchCode):</label>
                <select
                  value={selectedArch}
                  onChange={(e) => setSelectedArch(Number(e.target.value))}
                  className="w-full p-1.5 bg-slate-800 border border-white/10 rounded font-mono text-cyan-300"
                >
                  <option value={0x0001}>0x0001 - x86_64</option>
                  <option value={0x0002}>0x0002 - aarch64 (ARM64)</option>
                  <option value={0x0003}>0x0003 - riscv64</option>
                  <option value={0x0004}>0x0004 - atcvm_v1 (ATC-VM)</option>
                </select>
              </div>

              <div>
                <label className="block text-slate-400 font-mono mb-1">Sicherheits-Flags:</label>
                <div className="grid grid-cols-2 gap-1 text-[11px]">
                  <label className="flex items-center gap-1.5 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={selectedFlags.signed}
                      onChange={(e) => setSelectedFlags({ ...selectedFlags, signed: e.target.checked })}
                    />
                    <span>Signed</span>
                  </label>
                  <label className="flex items-center gap-1.5 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={selectedFlags.sandbox}
                      onChange={(e) => setSelectedFlags({ ...selectedFlags, sandbox: e.target.checked })}
                    />
                    <span>Sandbox</span>
                  </label>
                  <label className="flex items-center gap-1.5 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={selectedFlags.deterministic}
                      onChange={(e) => setSelectedFlags({ ...selectedFlags, deterministic: e.target.checked })}
                    />
                    <span>Deterministic</span>
                  </label>
                  <label className="flex items-center gap-1.5 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={selectedFlags.compressed}
                      onChange={(e) => setSelectedFlags({ ...selectedFlags, compressed: e.target.checked })}
                    />
                    <span>Compressed</span>
                  </label>
                </div>
              </div>

              <div className="flex flex-col justify-between">
                <span className="text-slate-400 font-mono">Integritäts-Status:</span>
                {isMutated ? (
                  <div className="p-1.5 bg-rose-950/60 border border-rose-500 text-rose-300 rounded text-[11px] font-mono flex items-center justify-between">
                    <span>MANIPULIERT (Fehler)</span>
                    <button
                      onClick={() => rebuildBinary(false)}
                      className="px-1.5 py-0.5 bg-rose-800 hover:bg-rose-700 text-white rounded text-[10px]"
                    >
                      Reparieren
                    </button>
                  </div>
                ) : (
                  <div className="p-1.5 bg-emerald-950/60 border border-emerald-500 text-emerald-300 rounded text-[11px] font-mono flex items-center justify-between">
                    <span>100% INTAKT</span>
                    <button
                      onClick={() => rebuildBinary(true)}
                      className="px-1.5 py-0.5 bg-amber-800 hover:bg-amber-700 text-white rounded text-[10px]"
                      title="Manipuliert ein Prüfsummen-Byte, um den Verifier-Fehler zu testen"
                    >
                      Byte mutieren
                    </button>
                  </div>
                )}
                <div className="text-[10px] font-mono text-slate-500">
                  Gesamtgröße: {currentArtifact?.rawBytes.length ?? 0} Bytes
                </div>
              </div>
            </div>

            {/* Live Hex Stream Grid from Actual Bytes */}
            <div className="bg-slate-950 p-4 rounded-lg border border-white/10 font-mono text-xs">
              <div className="text-slate-500 pb-2 mb-2 border-b border-white/10 flex justify-between">
                <span>OFFSET  00 01 02 03  04 05 06 07  08 09 0A 0B  0C 0D 0E 0F  | ASCII DECODE</span>
                <span className="text-cyan-400">
                  Header (0x00..0x3F) | Trunk Hash: 0x{currentArtifact?.header.checksumTrunkHex}
                </span>
              </div>

              <div className="space-y-1">
                {[0x00, 0x10, 0x20, 0x30].map((rowOffset) => {
                  const slice = currentArtifact ? currentArtifact.rawBytes.slice(rowOffset, rowOffset + 16) : new Uint8Array(16);
                  return (
                    <div key={rowOffset} className="flex items-center justify-between hover:bg-white/5 py-1 px-1 rounded">
                      <div className="flex items-center gap-4">
                        <span className="text-slate-500 font-bold">
                          0x{rowOffset.toString(16).padStart(2, '0').toUpperCase()}00:
                        </span>
                        <div className="flex items-center gap-1 text-cyan-300">
                          {Array.from(slice).map((byte, idx) => {
                            const absIdx = rowOffset + idx;
                            const isSelected = selectedHexByteIndex === absIdx;
                            return (
                              <span
                                key={idx}
                                onClick={() => setSelectedHexByteIndex(absIdx)}
                                className={`px-1 py-0.5 rounded cursor-pointer transition-colors ${
                                  isSelected
                                    ? 'bg-indigo-600 text-white font-bold ring-1 ring-white'
                                    : absIdx < 4
                                    ? 'text-emerald-400 font-bold'
                                    : absIdx >= 56
                                    ? 'text-amber-300 font-bold'
                                    : 'hover:bg-white/10'
                                }`}
                              >
                                {byte.toString(16).padStart(2, '0').toUpperCase()}
                              </span>
                            );
                          })}
                        </div>
                      </div>
                      <span className="text-amber-300 tracking-wider">
                        |{' '}
                        {Array.from(slice)
                          .map((b) => (b >= 32 && b <= 126 ? String.fromCharCode(b) : '.'))
                          .join('')}{' '}
                        |
                      </span>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Selected Byte Inspector */}
            {selectedHexByteIndex !== null && currentArtifact && (
              <div className="bg-slate-900/80 p-3 rounded-lg border border-indigo-500/30 font-mono text-xs flex flex-wrap items-center justify-between gap-4">
                <div className="flex items-center gap-3">
                  <span className="text-indigo-400 font-bold">
                    Byte @ 0x{selectedHexByteIndex.toString(16).padStart(2, '0').toUpperCase()} ({selectedHexByteIndex}):
                  </span>
                  <span className="text-white">
                    Hex: <code className="text-cyan-300 font-bold">0x{currentArtifact.rawBytes[selectedHexByteIndex]?.toString(16).padStart(2, '0').toUpperCase()}</code>
                  </span>
                  <span className="text-white">
                    Dezimal: <code className="text-amber-300">{currentArtifact.rawBytes[selectedHexByteIndex]}</code>
                  </span>
                  <span className="text-white">
                    ASCII: <code className="text-emerald-300">'{String.fromCharCode(currentArtifact.rawBytes[selectedHexByteIndex] || 32)}'</code>
                  </span>
                </div>
                <div className="text-slate-400 text-[11px]">
                  Feldzugehörigkeit:{' '}
                  <span className="text-white font-semibold">
                    {selectedHexByteIndex < 4
                      ? 'Magic "GLOB"'
                      : selectedHexByteIndex < 8
                      ? 'Format Sub-Tag'
                      : selectedHexByteIndex < 10
                      ? 'Version'
                      : selectedHexByteIndex < 12
                      ? 'Flags'
                      : selectedHexByteIndex < 14
                      ? 'Target Arch'
                      : selectedHexByteIndex < 16
                      ? 'ABI Version'
                      : selectedHexByteIndex < 24
                      ? 'Entrypoint Offset'
                      : selectedHexByteIndex < 32
                      ? 'Payload Size'
                      : selectedHexByteIndex < 40
                      ? 'Manifest Offset'
                      : selectedHexByteIndex < 48
                      ? 'Capability Offset'
                      : selectedHexByteIndex < 56
                      ? 'Signature Offset'
                      : 'Blake3/SHA256 Trunk Hash'}
                  </span>
                </div>
              </div>
            )}

            {/* Header Field Table */}
            <div>
              <h4 className="text-xs font-bold uppercase text-slate-400 tracking-wider mb-2">
                Header Felddefinitionen (GNFF_Header_v1)
              </h4>
              <div className="overflow-x-auto border border-white/10 rounded-lg">
                <table className="w-full text-left text-xs">
                  <thead className="bg-slate-900 text-slate-400 font-mono uppercase text-[10px] border-b border-white/10">
                    <tr>
                      <th className="p-2.5">Offset</th>
                      <th className="p-2.5">Bytes</th>
                      <th className="p-2.5">Feldname</th>
                      <th className="p-2.5">Typ</th>
                      <th className="p-2.5">Aktueller Wert (Live)</th>
                      <th className="p-2.5">Bedeutung & Validierungsregel</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5 font-mono">
                    {GFF_HEADER_FIELDS.map((field, idx) => {
                      let liveValue = '-';
                      if (currentArtifact) {
                        if (field.offset === '0x00..0x03') liveValue = 'GLOB (0x474C4F42)';
                        if (field.offset === '0x04..0x07') liveValue = currentArtifact.header.subTag;
                        if (field.offset === '0x08..0x09') liveValue = `v${currentArtifact.header.versionMajor}.${currentArtifact.header.versionMinor}`;
                        if (field.offset === '0x0A..0x0B') liveValue = `Flags: 0x${currentArtifact.rawBytes[10]?.toString(16).padStart(2, '0')}`;
                        if (field.offset === '0x0C..0x0D') liveValue = `Arch: 0x${currentArtifact.header.archCode.toString(16)}`;
                        if (field.offset === '0x0E..0x0F') liveValue = `ABI: 0x${currentArtifact.header.abiVersion.toString(16)}`;
                        if (field.offset === '0x10..0x17') liveValue = `0x${currentArtifact.header.entryPointOffset.toString(16)}`;
                        if (field.offset === '0x18..0x1F') liveValue = `${currentArtifact.header.payloadSize} B`;
                        if (field.offset === '0x20..0x27') liveValue = `0x${currentArtifact.header.manifestOffset.toString(16)}`;
                        if (field.offset === '0x28..0x2F') liveValue = `0x${currentArtifact.header.capabilityOffset.toString(16)}`;
                        if (field.offset === '0x30..0x37') liveValue = `0x${currentArtifact.header.signatureOffset.toString(16)}`;
                        if (field.offset === '0x38..0x3F') liveValue = `0x${currentArtifact.header.checksumTrunkHex}`;
                      }

                      return (
                        <tr key={field.offset} className="hover:bg-slate-900/60 text-slate-300">
                          <td className="p-2.5 text-cyan-400 font-bold">{field.offset}</td>
                          <td className="p-2.5 text-slate-400">{field.bytes} B</td>
                          <td className="p-2.5 font-bold text-white font-sans">{field.fieldName}</td>
                          <td className="p-2.5 text-amber-300">{field.type}</td>
                          <td className="p-2.5 text-emerald-400 font-bold">{liveValue}</td>
                          <td className="p-2.5 text-slate-400 font-sans">{field.description}</td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        )}

        {/* TAB 3: PACKAGE EXPLORER & REAL BUNDLE BUILDER (.GAPP) */}
        {activeTab === 'bundle' && (
          <div className="space-y-6">
            <div className="bg-gradient-to-r from-purple-950/40 via-slate-900/60 to-indigo-950/40 p-4 rounded-lg border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
              <div>
                <h3 className="text-sm font-bold text-white flex items-center gap-2">
                  <Package className="w-5 h-5 text-purple-400" />
                  <span>Echter .gapp Paket-Builder & JSZip Container Engine</span>
                </h3>
                <p className="text-xs text-slate-400 mt-0.5">
                  Packt echte Binärdateien (.gexe), Manifeste und Signaturen in ein normatives Globus Application Archive.
                </p>
              </div>

              <div className="flex items-center gap-2">
                <button
                  onClick={handleBuildAndDownloadGapp}
                  disabled={bundleBuilding}
                  className="flex items-center gap-2 px-4 py-2 bg-purple-600 hover:bg-purple-500 disabled:opacity-50 text-white text-xs font-bold rounded shadow-lg transition-all"
                >
                  <Download className="w-4 h-4" />
                  <span>{bundleBuilding ? 'Packe GAPP...' : 'Echtes .gapp Bundle bauen & herunterladen'}</span>
                </button>
              </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-12 gap-4">
              {/* Bundle File Tree */}
              <div className="lg:col-span-5 bg-slate-900/60 rounded-lg border border-white/10 p-4 space-y-4">
                <div className="flex items-center justify-between border-b border-white/10 pb-2">
                  <span className="text-xs font-bold text-white flex items-center gap-2">
                    <FolderArchive className="w-4 h-4 text-purple-400" />
                    Bundle-Inhalt (GenesisExplorer.gapp)
                  </span>
                  <span className="text-[11px] font-mono text-slate-400">5 Einträge</span>
                </div>

                <div className="space-y-2">
                  {bundleFiles.map((file) => (
                    <div
                      key={file.path}
                      className="p-2.5 rounded bg-slate-950/60 border border-white/5 hover:border-purple-500/40 text-xs font-mono transition-colors"
                    >
                      <div className="flex items-center justify-between">
                        <span className="font-bold text-cyan-300">{file.path}</span>
                        <span className="text-slate-500">{file.size} B</span>
                      </div>
                      <p className="text-[11px] text-slate-400 mt-1 font-sans">{file.desc}</p>
                    </div>
                  ))}
                </div>

                {/* Unpack Inspector */}
                <div className="pt-2 border-t border-white/10">
                  <span className="text-xs font-bold text-slate-300 block mb-2">
                    Lokale .gapp Datei entpacken & inspizieren:
                  </span>
                  <input
                    type="file"
                    accept=".gapp,.zip"
                    onChange={handleFileUpload}
                    className="w-full text-xs text-slate-400 file:mr-3 file:py-1 file:px-3 file:rounded file:border-0 file:text-xs file:font-semibold file:bg-purple-600 file:text-white hover:file:bg-purple-500"
                  />
                  {unpackedInspect && (
                    <div className="mt-2 p-2 bg-slate-950 rounded border border-purple-500/40 text-xs font-mono text-purple-300">
                      Entpackte Einträge ({unpackedInspect.fileList.length}): {unpackedInspect.fileList.join(', ')}
                    </div>
                  )}
                </div>
              </div>

              {/* Manifest Editor */}
              <div className="lg:col-span-7 bg-slate-900/60 rounded-lg border border-white/10 p-4 space-y-3">
                <div className="flex items-center justify-between border-b border-white/10 pb-2">
                  <span className="text-xs font-bold text-white flex items-center gap-2">
                    <FileCode className="w-4 h-4 text-cyan-400" />
                    manifest.gmanifest (Live Manifest-Editor)
                  </span>
                  <button
                    onClick={() => copyToClipboard(bundleManifestYaml, 'Manifest')}
                    className="flex items-center gap-1 text-[11px] text-slate-400 hover:text-white"
                  >
                    <Copy className="w-3.5 h-3.5" />
                    Kopieren
                  </button>
                </div>

                <textarea
                  value={bundleManifestYaml}
                  onChange={(e) => setBundleManifestYaml(e.target.value)}
                  rows={16}
                  className="w-full bg-slate-950 p-3 rounded font-mono text-xs text-emerald-300 border border-white/10 focus:outline-hidden focus:border-purple-500 leading-relaxed custom-scrollbar"
                />

                <div className="flex items-center justify-between text-[11px] text-slate-500 font-mono">
                  <span>Standard: GNFF v1.0 / ATC-DOC-013</span>
                  <button
                    onClick={handleExportManifestTemplate}
                    className="text-purple-400 hover:text-purple-300 font-bold"
                  >
                    Als Datei in Workspace speichern
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* TAB 4: SHIVACORE ZERO-TRUST VERIFIER AUDITOR */}
        {activeTab === 'pipeline' && (
          <div className="space-y-6">
            <div className="bg-gradient-to-r from-emerald-950/40 via-slate-900/60 to-indigo-950/40 p-4 rounded-lg border border-white/10 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
              <div>
                <h3 className="text-sm font-bold text-white flex items-center gap-2">
                  <ShieldCheck className="w-4 h-4 text-emerald-400" />
                  <span>ShivaCore Zero-Trust Verifikations-Auditor</span>
                </h3>
                <p className="text-xs text-slate-400 mt-0.5">
                  Kein Mock: Echte kryptografische Hash-Prüfung (SHA-256), HMAC-Signatur-Validierung, Boundary-Scans und Sandbox-Mounting.
                </p>
              </div>

              <div className="flex items-center gap-2">
                <button
                  onClick={() => executeRealVerification(false)}
                  disabled={verifierRunning}
                  className="flex items-center gap-2 px-4 py-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white text-xs font-bold rounded shadow-lg transition-all"
                >
                  <Play className="w-4 h-4" />
                  <span>{verifierRunning ? 'Prüfung läuft...' : 'Echte Prüfung starten'}</span>
                </button>
                <button
                  onClick={() => executeRealVerification(true)}
                  disabled={verifierRunning}
                  className="flex items-center gap-1.5 px-3 py-2 bg-rose-900/60 hover:bg-rose-800 text-rose-200 text-xs font-bold rounded border border-rose-500/40 transition-colors"
                  title="Testet den Verifier mit absichtlich manipuliertem Puffer"
                >
                  <AlertTriangle className="w-3.5 h-3.5" />
                  <span>Fehlerfall testen</span>
                </button>
              </div>
            </div>

            {/* Custom file inspection bar */}
            <div className="bg-slate-900/60 p-3 rounded-lg border border-white/5 flex flex-wrap items-center justify-between gap-3 text-xs">
              <div className="flex items-center gap-2">
                <span className="text-slate-400 font-bold">Prüfziel:</span>
                <span className="font-mono text-cyan-300 font-bold bg-slate-950 px-2 py-0.5 rounded border border-white/10">
                  {verifierTargetName}
                </span>
                {customFileBuffer && (
                  <span className="text-[11px] text-slate-500">({customFileBuffer.length} Bytes hochgeladen)</span>
                )}
              </div>

              <div className="flex items-center gap-2">
                <span className="text-slate-400 text-xs">Eigene Datei vom Rechner prüfen:</span>
                <input
                  ref={fileInputRef}
                  type="file"
                  onChange={handleFileUpload}
                  className="hidden"
                />
                <button
                  onClick={() => fileInputRef.current?.click()}
                  className="flex items-center gap-1.5 px-3 py-1 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded border border-white/10 font-bold"
                >
                  <Upload className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Datei auswählen (.gexe, .gapp, .atcb, etc.)</span>
                </button>
              </div>
            </div>

            {/* Pipeline Step Visualizer */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
              {(verificationReport?.steps ?? [
                { id: 1, title: '1. Magic Header Scan', category: 'header', status: 'pending', details: 'Prüft 0x00..0x08 exakt auf GLOB + Format-Tag.', durationMs: 0 },
                { id: 2, title: '2. Payload Hash-Prüfung', category: 'crypto', status: 'pending', details: 'Vergleicht echten SHA-256 Hash mit Header-Trunk.', durationMs: 0 },
                { id: 3, title: '3. Kryptografische Signatur', category: 'signature', status: 'pending', details: 'Validiert HMAC-SHA256 / Ed25519 Signatur.', durationMs: 0 },
                { id: 4, title: '4. Publisher Trust Chain', category: 'trust', status: 'pending', details: 'Prüft Identitätskette gegen Root Trust Store.', durationMs: 0 },
                { id: 5, title: '5. Capability Matrix Review', category: 'policy', status: 'pending', details: 'Dekodiert Bitmaske gegen Sicherheits-Policy.', durationMs: 0 },
                { id: 6, title: '6. ABI- & Architektur-Match', category: 'abi', status: 'pending', details: 'Stellt CPU-Kompatibilität und ABI sicher.', durationMs: 0 },
                { id: 7, title: '7. Static Safety & Bounds', category: 'safety', status: 'pending', details: 'Verifiziert Offsets und Puffergrenzen.', durationMs: 0 },
                { id: 8, title: '8. Sandbox Container Mount', category: 'sandbox', status: 'pending', details: 'Spawnt isolierten ShivaCore Container.', durationMs: 0 },
              ]).map((step) => {
                const isSuccess = step.status === 'success';
                const isError = step.status === 'error';
                const isWarning = step.status === 'warning';

                return (
                  <div
                    key={step.id}
                    className={`p-3.5 rounded-lg border transition-all ${
                      isSuccess
                        ? 'bg-emerald-950/40 border-emerald-500/60 text-emerald-100 shadow-sm'
                        : isError
                        ? 'bg-rose-950/40 border-rose-500 text-rose-100'
                        : isWarning
                        ? 'bg-amber-950/40 border-amber-500 text-amber-100'
                        : 'bg-slate-900/40 border-white/5 text-slate-400'
                    }`}
                  >
                    <div className="flex items-center justify-between mb-2">
                      <div className="flex items-center gap-2">
                        {isSuccess ? (
                          <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                        ) : isError ? (
                          <XCircle className="w-4 h-4 text-rose-400" />
                        ) : isWarning ? (
                          <AlertTriangle className="w-4 h-4 text-amber-400" />
                        ) : (
                          <ShieldCheck className="w-4 h-4 text-slate-500" />
                        )}
                        <span className="font-bold text-xs">{step.title}</span>
                      </div>
                      {step.durationMs > 0 && (
                        <span className="text-[10px] font-mono opacity-60">{step.durationMs}ms</span>
                      )}
                    </div>
                    <p className="text-[11px] leading-relaxed opacity-85">{step.details}</p>
                    {step.rawMetric && (
                      <div className="mt-2 text-[10px] font-mono px-1.5 py-0.5 rounded bg-black/40 text-cyan-300 truncate">
                        {step.rawMetric}
                      </div>
                    )}
                  </div>
                );
              })}
            </div>

            {/* Real Audit Log Output */}
            <div className="bg-slate-950 p-4 rounded-lg border border-white/10 font-mono text-xs space-y-1.5">
              <div className="text-slate-500 pb-2 border-b border-white/10 flex justify-between">
                <span>SHIVACORE SECURITY AUDIT REPORT</span>
                <span className={verificationReport?.isValid ? 'text-emerald-400 font-bold' : 'text-rose-400 font-bold'}>
                  Status: {verificationReport ? (verificationReport.isValid ? 'PASSED (Zero-Trust Verified)' : 'REJECTED (Security Breach)') : 'Bereit'}
                </span>
              </div>

              {verificationReport ? (
                <>
                  <div className="text-slate-400">
                    [Timestamp: {verificationReport.timestamp}] Artifact: {verificationReport.artifactName} ({verificationReport.totalSize} Bytes)
                  </div>
                  <div className="text-cyan-300">
                    [HEADER] SubTag='{verificationReport.header.subTag}' Arch=0x{verificationReport.header.archCode.toString(16)} ABI=v{verificationReport.header.abiVersion} ChecksumTrunk=0x{verificationReport.header.checksumTrunkHex}
                  </div>
                  <div className="text-slate-300">
                    [CRYPTO] Full SHA-256 Digest: {verificationReport.hashHex}
                  </div>
                  <div className="text-amber-300">
                    [CAPABILITIES] Granted ({verificationReport.capabilitiesGranted.length}): {verificationReport.capabilitiesGranted.join(', ')}
                  </div>
                  <div className={verificationReport.isValid ? 'text-emerald-300 font-bold' : 'text-rose-400 font-bold'}>
                    [CONTAINER] ID: {verificationReport.executionSandbox.containerId} | Profile: {verificationReport.executionSandbox.profile} | RAM Quota: {verificationReport.executionSandbox.memoryLimitMb} MB
                  </div>
                </>
              ) : (
                <div className="text-slate-500 py-3 text-center">
                  Klicke auf "Echte Prüfung starten" oder wähle eine Datei aus, um den 8-stufigen ShivaCore Sicherheitsprüflauf durchzuführen.
                </div>
              )}
            </div>
          </div>
        )}

        {/* TAB 5: VFS VS FORMAT VS GNFS */}
        {activeTab === 'vfs' && (
          <div className="space-y-6">
            <div className="bg-slate-900/60 p-5 rounded-lg border border-white/10 space-y-4">
              <h3 className="text-sm font-bold text-white flex items-center gap-2">
                <HardDrive className="w-4 h-4 text-amber-400" />
                <span>Grundsatz: Dateiformate ≠ Dateisystem (VFS-Schichtenmodell)</span>
              </h3>
              <p className="text-xs text-slate-300 leading-relaxed">
                In modernen Betriebssystemen sind Dateiformate (Container, Binärstrukturen, Archive) vollständig von der physischen Speicherschicht (ext4, ZFS, NVMe) entkoppelt. Globus OS setzt auf das <strong>ShivaCore VFS</strong>, welches native GFF-Dateien auf jedem Host-Dateisystem ausführen kann.
              </p>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
                <div className="p-4 bg-slate-950/60 rounded-lg border border-white/5 space-y-2">
                  <div className="flex items-center gap-2 text-cyan-400 font-bold text-xs">
                    <Layers className="w-4 h-4" />
                    <span>Phase 1: ShivaCore Virtual File System (VFS)</span>
                  </div>
                  <p className="text-[11px] text-slate-400 leading-relaxed">
                    Ausführung von <code className="text-cyan-300">.gexe</code>, <code className="text-cyan-300">.gapp</code> und <code className="text-cyan-300">.atcb</code> direkt auf Standard-Dateisystemen (ext4, Btrfs, ZFS). Der ShivaCore Loader übernimmt Hash-Prüfung, Entpackung und Sandbox-Isolation transparent im RAM.
                  </p>
                </div>

                <div className="p-4 bg-slate-950/60 rounded-lg border border-white/5 space-y-2">
                  <div className="flex items-center gap-2 text-amber-400 font-bold text-xs">
                    <Boxes className="w-4 h-4" />
                    <span>Phase 2: Globus Native File System (GNFS)</span>
                  </div>
                  <p className="text-[11px] text-slate-400 leading-relaxed">
                    Zukünftiges natives Dateisystem mit hardware-integrierter Inhaltsadressierung, automatischer Blake3-Deduplikation auf Blockebene, atomaren Snapshots und Zero-Copy NPU-Streaming für KI-Modelle (.gmodel).
                  </p>
                </div>
              </div>
            </div>

            {/* Matrix comparison */}
            <div>
              <h4 className="text-xs font-bold uppercase text-slate-400 tracking-wider mb-2">
                Betriebssystem-Vergleich: Dateiformat-Architektur
              </h4>
              <div className="overflow-x-auto border border-white/10 rounded-lg">
                <table className="w-full text-left text-xs">
                  <thead className="bg-slate-900 text-slate-400 font-mono uppercase text-[10px] border-b border-white/10">
                    <tr>
                      <th className="p-3">Kategorie</th>
                      <th className="p-3 text-blue-400">Windows (Historisch)</th>
                      <th className="p-3 text-orange-400">Linux (Standard)</th>
                      <th className="p-3 text-cyan-400">Globus OS (GFFA v1.0)</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5 font-mono text-[11px]">
                    <tr className="hover:bg-slate-900/60">
                      <td className="p-3 font-bold text-white font-sans">Ausführbare Datei</td>
                      <td className="p-3 text-slate-400">.exe (PE/COFF)</td>
                      <td className="p-3 text-slate-400">ELF (keine Pflicht-Endung)</td>
                      <td className="p-3 text-cyan-300 font-bold">.gexe (GFF 64B Header)</td>
                    </tr>
                    <tr className="hover:bg-slate-900/60">
                      <td className="p-3 font-bold text-white font-sans">Dynamische Bibliothek</td>
                      <td className="p-3 text-slate-400">.dll</td>
                      <td className="p-3 text-slate-400">.so</td>
                      <td className="p-3 text-cyan-300 font-bold">.gdll</td>
                    </tr>
                    <tr className="hover:bg-slate-900/60">
                      <td className="p-3 font-bold text-white font-sans">Anwendungs-Paket</td>
                      <td className="p-3 text-slate-400">.msi / .msix</td>
                      <td className="p-3 text-slate-400">.deb / .rpm / .flatpak</td>
                      <td className="p-3 text-cyan-300 font-bold">.gapp (Signiertes Bundle)</td>
                    </tr>
                    <tr className="hover:bg-slate-900/60">
                      <td className="p-3 font-bold text-white font-sans">Treiber-Format</td>
                      <td className="p-3 text-slate-400">.sys (Ring 0)</td>
                      <td className="p-3 text-slate-400">.ko (Kernel-Objekt)</td>
                      <td className="p-3 text-cyan-300 font-bold">.gdrv / .gsys (IOMMU User-Space)</td>
                    </tr>
                    <tr className="hover:bg-slate-900/60">
                      <td className="p-3 font-bold text-white font-sans">Sicherheitsprüfung</td>
                      <td className="p-3 text-slate-400">SignTool / Authenticode</td>
                      <td className="p-3 text-slate-400">GPG Paket-Signaturen</td>
                      <td className="p-3 text-cyan-300 font-bold">8-Stufen ShivaCore Zero-Trust</td>
                    </tr>
                    <tr className="hover:bg-slate-900/60">
                      <td className="p-3 font-bold text-white font-sans">Rechtemodell</td>
                      <td className="p-3 text-slate-400">ACLs & Admin UAC</td>
                      <td className="p-3 text-slate-400">POSIX & Sudo / Seccomp</td>
                      <td className="p-3 text-cyan-300 font-bold">Granulare Capability-Bitmaske</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
