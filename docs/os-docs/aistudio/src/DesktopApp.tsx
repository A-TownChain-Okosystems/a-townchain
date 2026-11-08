import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import {
  BookOpen,
  Layers,
  Boxes,
  Map,
  List,
  Scale,
  Shield,
  Cpu,
  Folder,
  Activity,
  FileText,
  FileMusic,
  Gamepad2,
  ChevronLeft,
  ChevronRight,
  BarChart2,
  Component,
  CheckSquare,
  Globe,
  Gavel,
  Terminal,
  X,
  Minus,
  Maximize2,
  Minimize2,
  Clock,
  Monitor,
  Book,
  Database,
  Code,
  ShieldCheck,
  Power,
  Search,
  Compass,
  Network,
  Bot,
  Plus,
  MonitorSmartphone,
  LayoutGrid,
  Settings,
  Calendar,
  HardDrive,
  Wrench,
  AlertTriangle,
  PenTool,
  Wallet,
  User,
  RefreshCw,
  Image,
  Bell,
  Wifi,
  BrainCircuit,
  ShoppingCart,
  Server,
  Box,
  Zap,
  ShieldAlert,
  Github,
  Table,
  Presentation,
  Palette,
  SquareTerminal,
  Video,
  Music,
  PlaySquare,
  Share2,
  Users,
  MessageSquare,
  FileCode,
  Headphones,
  Workflow,
  Rocket,
  Download,
  Pickaxe,
  Coins,
  CheckCircle,
  Save,
  Lock,
  Eye,
  MonitorPlay,
  ArrowLeftRight
} from "lucide-react";

import App, {
  EcosystemView,
  HierarchyView,
  StandardsView,
  AuditView,
  TechStackView,
} from "./App.tsx";
import { ArchitectureView } from "./components/ArchitectureView.tsx";
import { BatteryStatus } from "./components/BatteryStatus.tsx";
import { TechDocsView } from "./components/TechDocsView.tsx";
import { MetricsView } from "./components/MetricsView.tsx";
import { ModulesPluginView } from "./components/ModulesPluginView.tsx";
const MarketplaceView = React.lazy(() => import("./components/MarketplaceView.tsx").then(m => ({ default: m.MarketplaceView })));
import { BlockchainLedgerView } from "./components/BlockchainLedgerView.tsx";
import { StructureView } from "./components/StructureView.tsx";
import { TechTreeView } from "./components/TechTreeView.tsx";
import { NetworkExplorerView } from "./components/NetworkExplorerView.tsx";
const GovernanceView = React.lazy(() => import("./components/GovernanceView.tsx").then(m => ({ default: m.GovernanceView })));
import { ATownOSNode } from "./components/ATownOSNode.tsx";
import { GateToHellBrowser } from "./components/GateToHellBrowser.tsx";
import { ApiInterfacesView } from "./components/ApiInterfacesView.tsx";
import { SourceCodeViewer } from "./components/SourceCodeViewer.tsx";
import { ATCWalletView } from "./components/ATCWalletView.tsx";
import { ATCAssetView } from "./components/ATCAssetView.tsx";
import { BattleArenaView } from "./components/BattleArenaView.tsx";
import { ATCLangEditor } from "./components/ATCLangEditor.tsx";
import { TerminalView } from "./components/TerminalView.tsx";
import { FileManagerView } from "./components/FileManagerView.tsx";
import { TaskManagerView } from "./components/TaskManagerView.tsx";
import { SettingsView } from "./components/SettingsView.tsx";
import { LoginOverlay } from "./components/LoginOverlay.tsx";
import { ProjectHubView } from "./components/ProjectHubView.tsx";
import { UserProfileView } from "./components/UserProfileView.tsx";

import {
  OfficeCanvasUI, PlaygroundUI, GenesisCockpitUI, DAOGovernanceUI, 
  FinalIntegratorUI, ArchiveGuardianUI, HashManifestUI, SingularityMonitorUI, 
  AnonymityShieldUI, TransparencyMonitorUI, PrivacyGatekeeperUI, LEXAIMonitorUI, 
  TruthGateUI, ATCLangTerminalUI, ProtocolIntegratorUI, SurvivalControlUI, 
  EncyclopediaNexusUI, EvolutionNexusUI, EvolutionJournalUI, MultiPassMonitorUI, 
  RollbackManagerUI, AutoOptimizerUI, GovernanceSentinelUI, CivilizationCommandUI
} from './components/AtsSuite.tsx';

import { CalculatorView } from "./components/CalculatorView.tsx";
import { NotepadView } from "./components/NotepadView.tsx";
import { CalendarView } from "./components/CalendarView.tsx";
import { ClockView } from "./components/ClockView.tsx";
import { ATCDjStudioView } from "./components/ATCDjStudioView.tsx";
import { BlockchainEcosystemView } from "./components/BlockchainEcosystemView.tsx";

import { StorageManagerView } from "./components/StorageManagerView.tsx";
import { SystemDiagnosticsView } from "./components/SystemDiagnosticsView.tsx";
import { DevToolsView } from "./components/DevToolsView.tsx";
import { OfficeSuiteView } from "./components/OfficeSuiteView.tsx";

import { useKeyboardShortcut } from "./hooks/useKeyboardShortcut.ts";

import { AssetVaultView } from './components/AssetVaultView.tsx';
import { AntiCheatView } from './components/AntiCheatView.tsx';

import { AgentCivilizationView } from './components/AgentCivilizationView.tsx';
import { ZeroKnowledgeProofView } from './components/ZeroKnowledgeProofView.tsx';
import { DistributedDatalakeView } from './components/DistributedDatalakeView.tsx';
import { ComplianceEngineView } from './components/ComplianceEngineView.tsx';
import { AiKernelView } from './components/AiKernelView.tsx';
import { DependencyMapView } from './components/DependencyMapView.tsx';
import { ZkCircuitEditorView } from './components/ZkCircuitEditorView.tsx';

import { MARKETPLACE_EXTRA_APPS } from './marketplaceApps.ts';
import { Package } from 'lucide-react';

const WINDOW_CATEGORIES = [
  { id: "dokumente", label: "Dokumente Bereich", icon: FileText },
  { id: "medien", label: "Medien Bereich", icon: FileMusic },
  { id: "sozial", label: "Sozialmedia Bereich", icon: Users },
  { id: "system", label: "System Bereich", icon: Settings },
  { id: "blockchain", label: "Blockchain Bereich", icon: Database },
  { id: "ki", label: "KI Bereich", icon: Bot },
  { id: "gaming", label: "Gaming Bereich", icon: Gamepad2 },
];

import { AppGlobeView } from './components/AppGlobeView.tsx';
import { SystemFinderView } from './components/SystemFinderView.tsx';
import { GpuPerformanceWidget } from './components/GpuPerformanceWidget.tsx';
import { SoftwareKnowledgeDbView } from './components/SoftwareKnowledgeDbView.tsx';
import { RescueSystemView } from './components/RescueSystemView.tsx';
import { FranchiseFactoryView } from './components/FranchiseFactoryView.tsx';
import { RoadmapView } from './components/RoadmapView.tsx';
import { ReportsView } from './components/ReportsView.tsx';
import { GitHubRepoSyncView } from './components/GitHubRepoSyncView.tsx';
import { ATCWordView, ATCExcelView, ATCPowerPointView } from './components/OfficeApps.tsx';
import { Paint3DView } from './components/Paint3DView.tsx';
import { BenchmarkCenterView } from './components/BenchmarkCenterView.tsx';
import { SystemSettingsView, SoftwareSettingsView, HardwareSettingsView } from './components/SpecificSettingsViews.tsx';
import { ATCAudioPlayerView, ATCVideoPlayerView, ATCImageViewerView, ATCMediaPlayerHub } from './components/MediaApps.tsx';
import { FolderView } from './components/FolderView.tsx';
import { SocialMediaView } from './components/SocialMediaView.tsx';
import { P2PChatView } from './components/P2PChatView.tsx';
import { INITIAL_STRUCTURE } from './components/StructureView.tsx';
import { TECH_TREE_DATA } from './components/TechTreeView.tsx';
import { SystemHealthDashboardWidget } from './components/SystemHealthDashboardWidget.tsx';
import { ATownDashboardView } from './components/ATownDashboardView.tsx';
import { CiCdPipelineView } from './components/CiCdPipelineView.tsx';
import { TestnetSimulationView } from './components/TestnetSimulationView.tsx';
import { PoAITrainingEngineView } from './components/PoAITrainingEngineView.tsx';
import { GenesisBlockGeneratorView } from './components/GenesisBlockGeneratorView.tsx';
import { AtcWhitepaperView } from './components/AtcWhitepaperView.tsx';
import { TestnetOrchestrationView } from './components/TestnetOrchestrationView.tsx';
import { MainnetLaunchView } from './components/MainnetLaunchView.tsx';
import { DeFiLiquidityPoolView } from './components/DeFiLiquidityPoolView.tsx';
import { CryptoVisualizationView } from './components/CryptoVisualizationView.tsx';
import { ZkVisualizationView } from './components/ZkVisualizationView.tsx';
import { EcosystemTreeOverlay } from './components/EcosystemTreeOverlay.tsx';
import { AiSoftwareWorkflowView } from './components/AiSoftwareWorkflowView.tsx';
import { EcosystemUmlView } from './components/EcosystemUmlView.tsx';
import { IdeaToAppFlowchartView } from './components/IdeaToAppFlowchartView.tsx';
import { EcosystemInstaller } from './components/EcosystemInstaller.tsx';
import { SyncMetricsView } from './components/SyncMetricsView.tsx';

import { GitHubStatusDashboard } from './components/GitHubStatusDashboard.tsx';
import { EcosystemVisualizerView } from './components/EcosystemVisualizerView.tsx';
import { ApiOrchestratorView } from './components/ApiOrchestratorView.tsx';
import { ProjectAuditDashboard } from './components/ProjectAuditDashboard.tsx';
import { ProtocolsView } from './components/ProtocolsView.tsx';
import { SessionExportView } from './components/SessionExportView.tsx';
import { DeveloperKnowledgeBaseView } from './components/DeveloperKnowledgeBaseView.tsx';
import { HardwareDriversView } from './components/HardwareDriversView.tsx';
import IntegrationsWindow from './components/IntegrationsWindow.tsx';
import { WindowWikiView, WindowDbView } from './components/WindowExtras.tsx';

export const WINDOWS_MAP: Record<
  string,
  {
    label: string;
    icon: React.ElementType;
    component: React.ElementType;
    category: string;
  }
> = {
  sync_metrics: {
    label: "Sync Metrics",
    icon: Activity,
    component: SyncMetricsView,
    category: "system",
  },
  installer: {
    label: "A-Town Setup/Installer",
    icon: Settings,
    component: EcosystemInstaller,
    category: "system",
  },
  ecosystem_tree: {
    label: "GlobusOS Ökosystem",
    icon: Globe,
    component: EcosystemTreeOverlay,
    category: "system",
  },
  ecosystem_uml: {
    label: "Ecosystem UML",
    icon: Network,
    component: EcosystemUmlView,
    category: "system",
  },
  ai_software_flowchart: {
    label: "Idee zu App Flow",
    icon: Rocket,
    component: IdeaToAppFlowchartView,
    category: "system",
  },
  ai_software_workflow: {
    label: "KI-Software Workflow",
    icon: Bot,
    component: AiSoftwareWorkflowView,
    category: "system",
  },
  zk_visualizer: {
    label: "ZKP Visualizer",
    icon: Shield,
    component: ZkVisualizationView,
    category: "system",
  },
  ecosystem_designer: {
    label: "Ecosystem Builder",
    icon: Github,
    component: EcosystemVisualizerView,
    category: "system",
  },
  api_orchestrator: {
    label: "API Orchestrator",
    icon: Workflow,
    component: ApiOrchestratorView,
    category: "system",
  },
  project_audit: {
    label: "Projekt Audit",
    icon: ShieldCheck,
    component: ProjectAuditDashboard,
    category: "system",
  },
  protocols: {
    label: "PROTOCOLS",
    icon: Network,
    component: ProtocolsView,
    category: "system",
  },
  session_export: {
    label: "Session Exporter",
    icon: Download,
    component: SessionExportView,
    category: "system",
  },
  dkb_system: {
    label: "Dev Intelligence",
    icon: BrainCircuit,
    component: DeveloperKnowledgeBaseView,
    category: "system",
  },
  hardware_drivers: {
    label: "Hardware Treiber",
    icon: Server,
    component: HardwareDriversView,
    category: "system",
  },
  github_status: {
    label: "AT-Explorer CI/CD",
    icon: Github,
    component: GitHubStatusDashboard,
    category: "system",
  },
  integrations: {
    label: "Integrations & Sync",
    icon: Layers,
    component: IntegrationsWindow,
    category: "system",
  },
  roadmap: {
    label: "Roadmap Dashboard",
    icon: Activity,
    component: RoadmapView,
    category: "dokumente",
  },
  rescue_tools: {
    label: "Rescue & Diagnostik",
    icon: ShieldAlert,
    component: RescueSystemView,
    category: "system",
  },
  system_finder: {
    label: "System Finder",
    icon: Search,
    component: SystemFinderView,
    category: "system",
  },
  software_db: {
    label: "Knowledge DB",
    icon: Database,
    component: SoftwareKnowledgeDbView,
    category: "dokumente",
  },
  overview: {
    label: "A-Town Dashboard",
    icon: LayoutGrid || Component,
    component: App,
    category: "system",
  },
  blockchain_eco: {
    label: "L1 Blockchain Platform",
    icon: Globe,
    component: BlockchainEcosystemView,
    category: "blockchain",
  },
  project_hub: {
    label: "PROJECT HUB",
    icon: Map,
    component: ProjectHubView,
    category: "dokumente",
  },
  hierarchy: {
    label: "TIER ARCHITECTURE",
    icon: Layers,
    component: HierarchyView,
    category: "system",
  },
  architecture: {
    label: "MULTI-LAYER ARCHITEKTUR",
    icon: Boxes,
    component: ArchitectureView,
    category: "system",
  },
  reports: {
    label: "REPORTS",
    icon: Activity,
    component: ReportsView,
    category: "dokumente",
  },
  github_sync: {
    label: "GITHUB REPOS",
    icon: Github,
    component: GitHubRepoSyncView,
    category: "system",
  },
  ecosystem: {
    label: "STAGES",
    icon: Compass,
    component: EcosystemView,
    category: "blockchain",
  },
  standards: {
    label: "Protocol und Standards Bibliothek",
    icon: BookOpen,
    component: StandardsView,
    category: "dokumente",
  },
  audit: {
    label: "AUDIT & GAP ANALYSIS",
    icon: Shield,
    component: AuditView,
    category: "system",
  },
  tech_stack: {
    label: "TECH STACK",
    icon: Cpu,
    component: TechStackView,
    category: "system",
  },
  structure: {
    label: "FILE STRUCTURE",
    icon: Folder,
    component: StructureView,
    category: "system",
  },
  tech_tree: {
    label: "TECH TREE",
    icon: Activity,
    component: TechTreeView,
    category: "ki",
  },
  tech_docs: {
    label: "TECH DOCS",
    icon: FileText,
    component: TechDocsView,
    category: "dokumente",
  },
  metrics: {
    label: "SYSTEM METRICS",
    icon: BarChart2,
    component: MetricsView,
    category: "system",
  },
  modules: {
    label: "MODULES & PLUGINS",
    icon: Component,
    component: ModulesPluginView,
    category: "system",
  },
  marketplace: {
    label: "Marktplatz",
    icon: ShoppingCart,
    component: MarketplaceView,
    category: "blockchain",
  },
  blockchain_ledger: {
    label: "Blockchain",
    icon: Box,
    component: BlockchainLedgerView,
    category: "blockchain",
  },
  explorer: {
    label: "NETWORK EXPLORER",
    icon: Globe,
    component: NetworkExplorerView,
    category: "blockchain",
  },
  governance: {
    label: "DAO GOVERNANCE",
    icon: Gavel,
    component: GovernanceView,
    category: "blockchain",
  },
  atown_os: {
    label: "ATC-OS (+ Node)",
    icon: Bot,
    component: ATownOSNode,
    category: "ki",
  },
  api_interfaces: {
    label: "API & Integrations",
    icon: Network,
    component: ApiInterfacesView,
    category: "system",
  },
  source_code: {
    label: "System Source Code",
    icon: Code,
    component: SourceCodeViewer,
    category: "system",
  },
  atc_wallet: {
    label: "ATC Wallet",
    icon: Component,
    component: ATCWalletView,
    category: "blockchain",
  },
  asset_vault: {
    label: "Asset Vault",
    icon: Database,
    component: AssetVaultView,
    category: "dokumente",
  },
  anti_cheat: {
    label: "ATC Anti-Cheat",
    icon: Shield,
    component: AntiCheatView,
    category: "system",
  },
  atclang: {
    label: "ATCLang Native",
    icon: Code,
    component: ATCLangEditor,
    category: "system",
  },
  terminal: {
    label: "ATC-Shell",
    icon: Terminal,
    component: TerminalView,
    category: "system",
  },
  file_mgr: {
    label: "Daten Explorer",
    icon: Folder,
    component: FileManagerView,
    category: "dokumente",
  },
  storage_mgr: {
    label: "Laufwerke",
    icon: HardDrive,
    component: StorageManagerView,
    category: "system",
  },
  task_mgr: {
    label: "System Manager",
    icon: Activity,
    component: TaskManagerView,
    category: "system",
  },
  settings: {
    label: "System Einstellungen",
    icon: Settings,
    component: SettingsView,
    category: "system",
  },
  user_profile: {
    label: "Nutzer Profil",
    icon: User,
    component: UserProfileView,
    category: "system",
  },
  diagnostics: {
    label: "Problembehebung & Diagnose",
    icon: AlertTriangle,
    component: SystemDiagnosticsView,
    category: "system",
  },
  dev_tools: {
    label: "Dev Tools",
    icon: Wrench,
    component: DevToolsView,
    category: "system",
  },
  agent_civ: {
    label: "Agent Civilization",
    icon: Network,
    component: AgentCivilizationView,
    category: "ki",
  },
  zk_proofs: {
    label: "ZK Hardware & Formal",
    icon: Shield,
    component: ZeroKnowledgeProofView,
    category: "blockchain",
  },
  datalake: {
    label: "Data Lake & Rollups",
    icon: Database,
    component: DistributedDatalakeView,
    category: "blockchain",
  },
  compliance: {
    label: "Compliance Engine",
    icon: ShieldCheck,
    component: ComplianceEngineView,
    category: "system",
  },
  ai_kernel: {
    label: "AI Kernel",
    icon: BrainCircuit,
    component: AiKernelView,
    category: "system",
  },
  dependency_map: {
    label: "Dependency Map",
    icon: Network,
    component: DependencyMapView,
    category: "system",
  },
  zk_circuit: {
    label: "ZK Circuit Editor",
    icon: Boxes,
    component: ZkCircuitEditorView,
    category: "blockchain",
  },
  atc_benchmark: {
    label: "Benchmark Test Center",
    icon: Zap,
    component: BenchmarkCenterView,
    category: "system",
  },
  atc_sys_settings: {
    label: "System Einstellungen",
    icon: Cpu,
    component: SystemSettingsView,
    category: "system",
  },
  atc_soft_settings: {
    label: "Software Einstellungen",
    icon: Settings,
    component: SoftwareSettingsView,
    category: "system",
  },
  atc_hard_settings: {
    label: "Hardware Einstellungen",
    icon: HardDrive,
    component: HardwareSettingsView,
    category: "system",
  },
  a_town_dashboard: {
    label: "A-Town",
    icon: Pickaxe,
    component: ATownDashboardView,
    category: "blockchain",
  },
  cicd_pipeline: {
    label: "CI/CD Pipeline",
    icon: Server,
    component: CiCdPipelineView,
    category: "system",
  },
  testnet_simulation: {
    label: "Testnet Simulation",
    icon: Network,
    component: TestnetSimulationView,
    category: "blockchain",
  },
  poai_engine: {
    label: "PoAI Training Engine",
    icon: BrainCircuit,
    component: PoAITrainingEngineView,
    category: "blockchain",
  },
  genesis_generator: {
    label: "Genesis Block Generator",
    icon: Database,
    component: GenesisBlockGeneratorView,
    category: "blockchain",
  },
  atc_whitepaper: {
    label: "ATC Whitepaper",
    icon: BookOpen,
    component: AtcWhitepaperView,
    category: "blockchain",
  },
  testnet_orchestration: {
    label: "Testnet Orchestration",
    icon: Server,
    component: TestnetOrchestrationView,
    category: "blockchain",
  },
  mainnet_launch: {
    label: "Mainnet Launch",
    icon: Rocket,
    component: MainnetLaunchView,
    category: "blockchain",
  },
  defi_liquidity: {
    label: "DeFi Liquidity Pool",
    icon: Coins,
    component: DeFiLiquidityPoolView,
    category: "blockchain",
  },
  crypto_engine: {
    label: "Crypto Engine Ed25519",
    icon: Shield,
    component: CryptoVisualizationView,
    category: "blockchain",
  },
  office_canvas: { label: "ATS-OFFICE", icon: FileText, component: OfficeCanvasUI, category: "system" },
  playground_ui: { label: "ATS-PLAY", icon: PlaySquare, component: PlaygroundUI, category: "system" },
  genesis_cockpit: { label: "Genesis Cockpit", icon: Zap, component: GenesisCockpitUI, category: "blockchain" },
  dao_governance: { label: "DAO Governance UI", icon: Network, component: DAOGovernanceUI, category: "blockchain" },
  final_integrator: { label: "Final Integrator", icon: CheckCircle, component: FinalIntegratorUI, category: "system" },
  archive_guardian: { label: "Archive Guardian", icon: Save, component: ArchiveGuardianUI, category: "system" },
  hash_manifest: { label: "Hash Manifest", icon: Database, component: HashManifestUI, category: "blockchain" },
  singularity_monitor: { label: "Singularity Monitor", icon: Activity, component: SingularityMonitorUI, category: "system" },
  anonymity_shield: { label: "Anonymity Shield", icon: Lock, component: AnonymityShieldUI, category: "blockchain" },
  transparency_monitor: { label: "Transparency Monitor", icon: Eye, component: TransparencyMonitorUI, category: "system" },
  privacy_gatekeeper: { label: "Privacy Gatekeeper", icon: Shield, component: PrivacyGatekeeperUI, category: "system" },
  lex_ai_monitor: { label: "LEX-AI Monitor", icon: FileText, component: LEXAIMonitorUI, category: "ki" },
  truth_gate: { label: "Truth Gate UI", icon: CheckCircle, component: TruthGateUI, category: "ki" },
  atc_lang_terminal: { label: "ATC-LANG Terminal", icon: MonitorPlay, component: ATCLangTerminalUI, category: "system" },
  protocol_integrator: { label: "Protocol Integrator", icon: ArrowLeftRight, component: ProtocolIntegratorUI, category: "blockchain" },
  survival_control: { label: "Survival Control", icon: Server, component: SurvivalControlUI, category: "system" },
  encyclopedia_nexus: { label: "Encyclopedia Nexus", icon: Search, component: EncyclopediaNexusUI, category: "system" },
  evolution_nexus: { label: "Evolution Nexus", icon: Network, component: EvolutionNexusUI, category: "system" },
  evolution_journal: { label: "Evolution Journal", icon: FileText, component: EvolutionJournalUI, category: "system" },
  multi_pass_monitor: { label: "Multi-Pass Monitor", icon: CheckCircle, component: MultiPassMonitorUI, category: "system" },
  rollback_manager: { label: "Rollback Manager", icon: Clock, component: RollbackManagerUI, category: "system" },
  auto_optimizer: { label: "Auto-Optimizer", icon: Cpu, component: AutoOptimizerUI, category: "system" },
  governance_sentinel: { label: "Governance Sentinel", icon: Shield, component: GovernanceSentinelUI, category: "blockchain" },
  civilization_command: { label: "Civilization Command", icon: Network, component: CivilizationCommandUI, category: "system" },
  folder_view: {
    label: "Folder",
    icon: Folder,
    component: FolderView,
    category: "dokumente",
  },
};

interface WindowState {
  id: string;
  isMaximized: boolean;
  isMinimized: boolean;
  zIndex: number;
  snappedPosition?: 'left' | 'right' | 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | null;
  x?: number;
  y?: number;
  width?: number | string;
  height?: number | string;
  viewMode?: 'app' | 'wiki' | 'db';
}

function OfflineQueueWidget() {
  const [queue, setQueue] = useState<any[]>([]);
  const [totalItems, setTotalItems] = useState(0);
  const [offlineMode, setOfflineMode] = useState(false);
  const [isFlushing, setIsFlushing] = useState(false);

  useEffect(() => {
    const handleStorageChange = () => {
      try {
        const storedQueue = JSON.parse(localStorage.getItem('atc_offline_queue') || '[]');
        setQueue(storedQueue);
        setOfflineMode(localStorage.getItem('atc_offline_mode') === 'true');
        
        // Track the highest queue size seen for the denominator
        if (storedQueue.length > totalItems) {
           setTotalItems(storedQueue.length);
        } else if (storedQueue.length === 0) {
           setTotalItems(0);
        }
      } catch {}
    };
    handleStorageChange();
    window.addEventListener('storage', handleStorageChange);
    const interval = setInterval(handleStorageChange, 2000);
    return () => {
      window.removeEventListener('storage', handleStorageChange);
      clearInterval(interval);
    };
  }, [totalItems]);

  if (queue.length === 0) return null;

  const syncedCount = totalItems - queue.length;

  return (
    <div className="relative flex items-center gap-2 px-2 py-1 rounded bg-black/40 border border-white/5 hover:bg-white/5 transition-colors cursor-pointer group" title="Offline Sync Queue">
      <div className="relative">
        <Server className="w-4 h-4 text-amber-500" />
        <span className="absolute -top-1 -right-1 w-2 h-2 rounded-full bg-red-500 animate-pulse border border-[#050811]" />
      </div>
      <span className="text-[10px] font-bold text-amber-500">{queue.length}</span>

      {/* Hover Dropdown */}
      <div className="absolute bottom-full right-0 mb-3 w-64 bg-[#090b14]/95 backdrop-blur-xl border border-white/10 rounded-xl shadow-2xl opacity-0 invisible group-hover:opacity-100 group-hover:visible pointer-events-none transition-all z-[10000] overflow-hidden">
        <div className="p-3 border-b border-white/5 bg-black/20 flex flex-col gap-1">
          <div className="flex items-center justify-between">
            <h4 className="text-xs font-bold text-white flex items-center gap-1.5"><Server className="w-3.5 h-3.5 text-amber-400" /> Sync Progress</h4>
            <span className="text-[10px] bg-indigo-500/20 text-indigo-400 px-1.5 py-0.5 rounded font-mono font-bold leading-none">{syncedCount}/{totalItems} SYNCED</span>
          </div>
          <div className="w-full h-1 bg-white/10 rounded-full mt-2 overflow-hidden">
             <div className="h-full bg-indigo-500 transition-all duration-500" style={{ width: `${totalItems > 0 ? (syncedCount / totalItems) * 100 : 0}%` }} />
          </div>
          <p className="text-[10px] text-slate-400 mt-2">{queue.length} items waiting in cache. They will be retried when online.</p>
        </div>
        <div className="max-h-48 overflow-y-auto px-2 py-2 flex flex-col gap-1 custom-scrollbar">
          {queue.map((item, idx) => (
            <div key={idx} className="bg-white/5 border border-white/5 rounded-lg p-2 flex flex-col gap-1.5">
              <div className="flex items-center justify-between">
                <span className={`text-[9px] font-bold uppercase tracking-widest px-1.5 py-0.5 rounded leading-none ${item.action === 'CREATE' ? 'bg-emerald-500/20 text-emerald-400' : item.action === 'UPDATE' ? 'bg-amber-500/20 text-amber-400' : 'bg-rose-500/20 text-rose-400'}`}>{item.action}</span>
                <span className="text-[9px] text-slate-500 font-mono text-right">{new Date(item.timestamp).toLocaleTimeString()}</span>
              </div>
              <div className="flex items-center justify-between mt-0.5">
                <span className="text-xs text-slate-300 font-medium">{item.entity}</span>
                {isFlushing ? (
                  <RefreshCw className="w-3 h-3 text-indigo-400 animate-spin" />
                ) : (
                  <span className="text-[10px] text-slate-500">Queued</span>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

function ChainPulseWidget() {
  const [block, setBlock] = useState(145000);
  const [tps, setTps] = useState(254);

  useEffect(() => {
    const interval = setInterval(() => {
      setBlock(b => b + 1);
      setTps(Math.floor(Math.random() * 500) + 100);
    }, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="relative flex flex-col justify-center px-2 py-1 rounded bg-black/40 border border-white/5 hover:bg-white/5 transition-colors cursor-pointer group" title="Chain Pulse">
      <div className="flex items-center gap-1.5">
        <Box className="w-3 h-3 text-cyan-400" />
        <span className="text-[9px] font-bold text-cyan-400 font-mono">BK {block}</span>
      </div>
      <div className="flex items-center gap-1.5">
        <Zap className="w-3 h-3 text-amber-400" />
        <span className="text-[9px] font-bold text-amber-400 font-mono">{tps} TPS</span>
      </div>
    </div>
  );
}

function NodePulseWidget({ onClick }: { onClick?: () => void }) {
  const [latency, setLatency] = useState(12);
  const [nodes, setNodes] = useState(99.9);
  const [history, setHistory] = useState<number[]>(Array(30).fill(12));

  useEffect(() => {
    const interval = setInterval(() => {
      setLatency(prev => {
        const jump = (Math.random() - 0.5) * 4;
        let next = prev + jump;
        if (next < 5) next = 5;
        if (next > 45) next = 45;
        const val = Number(next.toFixed(0));
        setHistory(h => [...h.slice(1, 30), val]);
        return val;
      });
      setNodes(prev => {
        const jump = (Math.random() - 0.5) * 0.1;
        let next = prev + jump;
        if (next > 100) next = 100;
        if (next < 98.0) next = 98.0;
        return Number(next.toFixed(1));
      });
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  const maxValue = Math.max(...history, 30);
  const points = history.map((val, i) => `${(i / (history.length - 1)) * 40},${16 - (val / maxValue) * 16}`).join(' ');

  return (
    <div onClick={onClick} className="relative flex items-center gap-2 px-2 py-1 rounded bg-black/40 border border-white/5 hover:bg-white/5 transition-colors cursor-pointer group" title="Network Explorer">
      <div className="flex items-center justify-center w-10 h-4">
        <svg viewBox="0 0 40 16" className="w-full h-full overflow-visible">
          <polyline
            points={points}
            fill="none"
            stroke="#10b981"
            strokeWidth="1.5"
            strokeLinejoin="round"
            strokeLinecap="round"
            className="drop-shadow-[0_0_2px_rgba(16,185,129,0.5)]"
          />
        </svg>
      </div>
      <div className="flex flex-col text-left gap-0.5 w-[70px]">
        <span className="text-[9px] font-mono leading-none text-emerald-400">{latency}ms Latency</span>
        <span className="text-[9px] font-mono leading-none text-slate-400">{nodes}% Nodes</span>
      </div>

      <div className="absolute bottom-full right-0 mb-4 w-64 p-4 bg-[#0a0d17]/95 backdrop-blur-xl border border-emerald-500/30 rounded-xl shadow-[0_0_30px_rgba(16,185,129,0.15)] z-[10000] opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all">
        <h4 className="text-xs font-bold text-white uppercase tracking-widest mb-3 flex items-center gap-2">
          <Network className="w-4 h-4 text-emerald-400" />
          Visual Node Cluster
        </h4>
        <div className="relative w-full h-32 bg-black/40 border border-white/5 rounded-lg overflow-hidden flex items-center justify-center">
          <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-emerald-500/10 via-transparent to-transparent" />
          <svg className="w-full h-full">
            <g transform="translate(110, 60)">
              {/* Lines */}
              <line x1="0" y1="0" x2="-40" y2="-20" stroke="#10b981" strokeWidth="1" strokeOpacity="0.4" />
              <line x1="0" y1="0" x2="40" y2="-30" stroke="#10b981" strokeWidth="1" strokeOpacity="0.4" />
              <line x1="0" y1="0" x2="-30" y2="30" stroke="#10b981" strokeWidth="1" strokeOpacity="0.4" />
              <line x1="0" y1="0" x2="50" y2="20" stroke="#10b981" strokeWidth="1" strokeOpacity="0.4" />
              <line x1="-40" y1="-20" x2="-30" y2="30" stroke="#10b981" strokeWidth="1" strokeOpacity="0.2" />
              <line x1="40" y1="-30" x2="50" y2="20" stroke="#10b981" strokeWidth="1" strokeOpacity="0.2" />
              
              {/* Nodes */}
              <circle cx="0" cy="0" r="6" fill="#10b981" className="animate-pulse" />
              <circle cx="-40" cy="-20" r="4" fill="#34d399" />
              <circle cx="40" cy="-30" r="3" fill="#059669" />
              <circle cx="-30" cy="30" r="4" fill="#34d399" />
              <circle cx="50" cy="20" r="5" fill="#10b981" />
              <circle cx="-60" cy="0" r="2" fill="#059669" />
              <line x1="-40" y1="-20" x2="-60" y2="0" stroke="#10b981" strokeWidth="1" strokeOpacity="0.4" />
            </g>
          </svg>
        </div>
        <div className="mt-3 flex justify-between text-[10px] font-mono text-slate-400">
           <span>Healthy Peers: 142</span>
           <span className="text-emerald-400">Sync: 100%</span>
        </div>
      </div>
    </div>
  );
}

function useWorkspaceState() {
  const [windows, setWindows] = useState<WindowState[]>(() => {
    try {
      const saved = localStorage.getItem('atc_workspace');
      if (saved) return JSON.parse(saved);
    } catch (e) {}
    return [];
  });
  
  useEffect(() => {
    localStorage.setItem('atc_workspace', JSON.stringify(windows));
  }, [windows]);

  return [windows, setWindows] as const;
}

// Inside DesktopApp.tsx, let's create a small toast component
function SystemTrayToast() {
  const [toast, setToast] = useState<{ id: number; message: string } | null>(null);

  useEffect(() => {
    let timeoutId: NodeJS.Timeout;
    const handleAlert = (e: any) => {
      setToast({ id: Date.now(), message: e.detail });
      if (timeoutId) clearTimeout(timeoutId);
      timeoutId = setTimeout(() => setToast(null), 4000);
    };
    
    window.addEventListener('ATC_SYSTEM_ALERT', handleAlert);
    return () => window.removeEventListener('ATC_SYSTEM_ALERT', handleAlert);
  }, []);

  return (
    <AnimatePresence>
      {toast && (
        <motion.div
           key={toast.id}
           initial={{ opacity: 0, y: 50, scale: 0.9 }}
           animate={{ opacity: 1, y: 0, scale: 1 }}
           exit={{ opacity: 0, y: 20, scale: 0.9 }}
           className="absolute bottom-20 right-6 z-[10000] p-4 bg-red-950/90 border border-red-500/50 rounded-xl shadow-[0_10px_40px_rgba(239,68,68,0.3)] backdrop-blur-md flex items-center gap-3 w-80"
        >
          <div className="p-2 bg-red-500/20 rounded-full animate-pulse">
            <Activity className="w-5 h-5 text-red-500" />
          </div>
          <div className="flex flex-col">
            <span className="text-xs font-bold text-red-400 tracking-widest uppercase">System Alert</span>
            <span className="text-sm text-slate-200 mt-0.5">{toast.message}</span>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

import { useSyncMetrics } from "./contexts/SyncMetricsContext.tsx";
import { useGoogleWorkspace } from "./contexts/GoogleWorkspaceContext.tsx";
import { CalendarDays } from "lucide-react";
import { SyncDashboardModal } from "./components/SyncDashboardModal.tsx";
import { SyncStatusOverview } from "./components/SyncStatusOverview.tsx";

function WorkspaceSyncStatusBar() {
  const { isSyncing, lastSyncTime, serviceSyncTimes } = useSyncMetrics();
  const { isAuthenticated } = useGoogleWorkspace();
  const [isModalOpen, setIsModalOpen] = useState(false);

  const getTooltip = (service: string, time: Date | null) => {
    return `${service}: ${time ? 'Last synced at ' + time.toLocaleTimeString() + ' (Click to copy)' : 'Not synced yet'}`;
  };

  const copyTimestamp = (e: React.MouseEvent, service: string, time: Date | null) => {
    e.stopPropagation();
    const textToCopy = time ? time.toLocaleTimeString() : 'Not synced yet';
    navigator.clipboard.writeText(textToCopy).then(() => {
      window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: `Copied ${service} sync time to clipboard.` }));
    });
  };

  return (
    <>
      <div onClick={() => setIsModalOpen(true)} className="relative flex items-center gap-2 px-2 py-1 rounded bg-black/40 border border-white/5 hover:bg-white/5 transition-colors cursor-pointer group" title="Workspace Sync Status">
        <div className="flex items-center gap-1.5 opacity-80 group-hover:opacity-100 transition-opacity">
          <span onClick={(e) => copyTimestamp(e, 'Drive', serviceSyncTimes.drive)} title={getTooltip('Drive', serviceSyncTimes.drive)} className="hover:scale-110 transition-transform"><HardDrive className={`w-3.5 h-3.5 ${isAuthenticated ? 'text-blue-400' : 'text-slate-600'}`} /></span>
          <span onClick={(e) => copyTimestamp(e, 'Sheets', serviceSyncTimes.sheets)} title={getTooltip('Sheets', serviceSyncTimes.sheets)} className="hover:scale-110 transition-transform"><Database className={`w-3.5 h-3.5 ${isAuthenticated ? 'text-emerald-400' : 'text-slate-600'}`} /></span>
          <span onClick={(e) => copyTimestamp(e, 'Docs', serviceSyncTimes.docs)} title={getTooltip('Docs', serviceSyncTimes.docs)} className="hover:scale-110 transition-transform"><FileText className={`w-3.5 h-3.5 ${isAuthenticated ? 'text-indigo-400' : 'text-slate-600'}`} /></span>
          <span onClick={(e) => copyTimestamp(e, 'Calendar', serviceSyncTimes.calendar)} title={getTooltip('Calendar', serviceSyncTimes.calendar)} className="hover:scale-110 transition-transform"><CalendarDays className={`w-3.5 h-3.5 ${isAuthenticated ? 'text-orange-400' : 'text-slate-600'}`} /></span>
        </div>
        {(isSyncing || lastSyncTime) && (
          <>
            <div className="w-[1px] h-3 bg-white/10 mx-0.5" />
            <div 
              className="relative flex items-center gap-1.5 hover:bg-white/10 px-1 rounded transition-colors" 
              title="Copy to clipboard"
              onClick={(e) => copyTimestamp(e, 'Last sync', lastSyncTime)}
            >
              <div className="relative">
                <Activity className={`w-3.5 h-3.5 ${isSyncing ? 'text-emerald-400 animate-pulse' : 'text-slate-400 group-hover:text-emerald-400'}`} />
                {isSyncing && <span className="absolute -top-1 -right-1 w-2 h-2 rounded-full bg-emerald-500 animate-ping" />}
              </div>
              <SyncStatusOverview />
            </div>
          </>
        )}
      </div>
      <SyncDashboardModal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} />
    </>
  );
}

export default function DesktopApp() {
  const checkSession = () => {
    const sessionExpires = localStorage.getItem('atc_session_expires');
    if (!sessionExpires) return false;
    return new Date(sessionExpires).getTime() > new Date().getTime();
  };

  const [windowsMapVersion, setWindowsMapVersion] = useState(0);

  const [isHighLoad, setIsHighLoad] = useState(false);

  useEffect(() => {
    const handleStress = (e: any) => setIsHighLoad(e.detail);
    window.addEventListener('ATC_SET_STRESS_TEST', handleStress);
    return () => window.removeEventListener('ATC_SET_STRESS_TEST', handleStress);
  }, []);

  useEffect(() => {
    const handleInstall = () => setWindowsMapVersion(v => v + 1);
    window.addEventListener("atc_app_installed", handleInstall);
    
    const handleOpenWindow = (e: any) => {
      const { id } = e.detail;
      openWindow(id);
    };
    const handleOpenApp = (e: any) => {
      openWindow(e.detail);
    };
    window.addEventListener("open-window", handleOpenWindow);
    window.addEventListener("ATC_OPEN_APP", handleOpenApp);

    return () => {
      window.removeEventListener("atc_app_installed", handleInstall);
      window.removeEventListener("open-window", handleOpenWindow);
      window.removeEventListener("ATC_OPEN_APP", handleOpenApp);
    };
  }, []);

  useEffect(() => {
    import('./utils/appSync.tsx').then(({ syncInstalledApps }) => {
      if (syncInstalledApps()) {
        setWindowsMapVersion(v => v + 1);
      }
    });
  }, []);

  const [isLoggedIn, setIsLoggedIn] = useState(checkSession);
  const [windows, setWindows] = useWorkspaceState();
  const [focusedId, setFocusedId] = useState<string | null>(() => localStorage.getItem('atc_focused_id'));
  const [isStartOpen, setIsStartOpen] = useState(false);
  const [shareDialogId, setShareDialogId] = useState<string | null>(null);
  
  useEffect(() => {
    if (focusedId) localStorage.setItem('atc_focused_id', focusedId);
    else localStorage.removeItem('atc_focused_id');
  }, [focusedId]);

  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      if (['INPUT', 'TEXTAREA', 'SELECT'].includes((e.target as HTMLElement).tagName)) {
        return;
      }

      const shortcuts = JSON.parse(localStorage.getItem('atc_shortcuts') || 'null') || [
        { id: 'search', keys: ['Ctrl', 'K'] },
        { id: 'save', keys: ['Ctrl', 'S'] },
        { id: 'switch', keys: ['Alt', 'Tab'] },
        { id: 'close', keys: ['Esc'] },
        { id: 'reopen', keys: ['Ctrl', 'Shift', 'T'] },
        { id: 'build', keys: ['Ctrl', 'B'] },
        { id: 'sync', keys: ['Alt', 'S'] },
        { id: 'settings', keys: ['Alt', 'O'] },
        { id: 'focus_mode', keys: ['Alt', 'F'] },
      ];

      const keys = [];
      if (e.ctrlKey) keys.push('Ctrl');
      if (e.altKey) keys.push('Alt');
      if (e.shiftKey) keys.push('Shift');
      if (e.metaKey) keys.push('Meta');

      const charKey = e.key.length === 1 ? e.key.toUpperCase() : e.key;
      // Handle "Escape" to "Esc" normalization for matching
      let normalizedCharKey = charKey;
      if (normalizedCharKey === "Escape") normalizedCharKey = "Esc";
      
      if (!['Control', 'Alt', 'Shift', 'Meta'].includes(charKey)) {
        keys.push(normalizedCharKey);
      }

      const pressedCombo = keys.join('+');

      shortcuts.forEach((sc: any) => {
        if (sc.keys?.join('+') === pressedCombo) {
          e.preventDefault();
          switch (sc.id) {
            case 'sync':
              window.dispatchEvent(new CustomEvent('atc-trigger-sync-shortcut'));
              break;
            case 'settings':
              window.dispatchEvent(new CustomEvent('ATC_OPEN_APP', { detail: 'settings' }));
              break;
            case 'focus_mode':
              const currentFocus = localStorage.getItem('atc_focus_mode') === 'true';
              localStorage.setItem('atc_focus_mode', (!currentFocus).toString());
              window.dispatchEvent(new Event('atc_focus_mode_changed'));
              break;
            case 'close':
              // We rely on standard propagation or explicit modals listening to Esc 
              break;
            // Optionally add other actions natively here if required
          }
        }
      });
    };

    window.addEventListener('keydown', handleGlobalKeyDown);
    return () => window.removeEventListener('keydown', handleGlobalKeyDown);
  }, []);

  const [activeCategory, setActiveCategory] = useState<string | null>(null);
  const [maxZIndex, setMaxZIndex] = useState(10);
  const [time, setTime] = useState(new Date());
  const [iconPage, setIconPage] = useState(0);
  const [contextMenu, setContextMenu] = useState<{x: number, y: number} | null>(null);
  
  const [windowOpacity, setWindowOpacity] = useState(() => parseInt(localStorage.getItem('atc_window_opacity') || '100'));
  useEffect(() => {
    const handleOpacity = () => {
      setWindowOpacity(parseInt(localStorage.getItem('atc_window_opacity') || '100'));
    };
    window.addEventListener('atc_window_opacity_changed', handleOpacity);
    return () => window.removeEventListener('atc_window_opacity_changed', handleOpacity);
  }, []);

  const touchTimerRef = React.useRef<NodeJS.Timeout | null>(null);
  const touchStartPosRef = React.useRef<{y1: number, y2: number} | null>(null);
  const [showTouchTutorial, setShowTouchTutorial] = useState(() => !localStorage.getItem('atc_touch_tutorial'));

  const closeTouchTutorial = () => {
    localStorage.setItem('atc_touch_tutorial', 'true');
    setShowTouchTutorial(false);
  };

  const closeContextMenu = () => {
    if (contextMenu) setContextMenu(null);
  };

  useEffect(() => {
    document.addEventListener("mousedown", closeContextMenu);
    return () => document.removeEventListener("mousedown", closeContextMenu);
  }, [contextMenu]);

  const [showSessionWarning, setShowSessionWarning] = useState(false);
  const hasWarnedRef = React.useRef(false);
  const [osMode, setOsMode] = useState(() => localStorage.getItem('atc_os_mode') || 'default');
  const [globalOsEnv, setGlobalOsEnv] = useState(() => localStorage.getItem('atc_os_env') || 'atc-native');

  const WALLPAPERS = [
    { id: 'aurora', name: 'Aurora Fjord', url: 'https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/371a23000_file_0000000046a0720a8c30add446651917.png' },
    { id: 'ember', name: 'Ember Forest', url: 'https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/6a9bc513c_Imagine_1570227521780807.jpg' },
    { id: 'mandala', name: 'Genesis Mandala', url: 'https://media.base44.com/images/public/6a2756186106d6f0fbb105b5/8fad3f789_Imagine_1570228721780687.jpg' },
    { id: 'classic', name: 'Classic Grid', url: 'https://images.unsplash.com/photo-1550751827-4bd374c3f58b?q=80&w=2070&auto=format&fit=crop' },
  ];
  const [wallpaperUrl, setWallpaperUrl] = useState(() => localStorage.getItem('atc_wallpaper') || WALLPAPERS[0].url);
  const [showWallpaperPicker, setShowWallpaperPicker] = useState(false);
  useEffect(() => { localStorage.setItem('atc_wallpaper', wallpaperUrl); }, [wallpaperUrl]);
  const [showCommandPalette, setShowCommandPalette] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');

  useKeyboardShortcut({ key: 'k', ctrlKey: true }, (e) => {
    e.preventDefault();
    setShowCommandPalette(prev => !prev);
  });
  useKeyboardShortcut({ key: 'k', metaKey: true }, (e) => {
    e.preventDefault();
    setShowCommandPalette(prev => !prev);
  });

  useKeyboardShortcut({ key: 'ArrowLeft', metaKey: true }, (e) => {
    e.preventDefault();
    setWindows(prev => {
      const highestZ = Math.max(...prev.map(w => w.zIndex), 0);
      return prev.map(w => {
        if (w.zIndex === highestZ && !w.isMinimized && highestZ > 0) {
          if (w.snappedPosition === 'top-right') return { ...w, snappedPosition: 'top-left' };
          if (w.snappedPosition === 'bottom-right') return { ...w, snappedPosition: 'bottom-left' };
          return { ...w, isMaximized: false, snappedPosition: 'left' };
        }
        return w;
      });
    });
  });

  useKeyboardShortcut({ key: 'ArrowRight', metaKey: true }, (e) => {
    e.preventDefault();
    setWindows(prev => {
      const highestZ = Math.max(...prev.map(w => w.zIndex), 0);
      return prev.map(w => {
        if (w.zIndex === highestZ && !w.isMinimized && highestZ > 0) {
          if (w.snappedPosition === 'top-left') return { ...w, snappedPosition: 'top-right' };
          if (w.snappedPosition === 'bottom-left') return { ...w, snappedPosition: 'bottom-right' };
          return { ...w, isMaximized: false, snappedPosition: 'right' };
        }
        return w;
      });
    });
  });

  useKeyboardShortcut({ key: 'ArrowUp', metaKey: true }, (e) => {
    e.preventDefault();
    setWindows(prev => {
      const highestZ = Math.max(...prev.map(w => w.zIndex), 0);
      return prev.map(w => {
        if (w.zIndex === highestZ && !w.isMinimized && highestZ > 0) {
          if (w.snappedPosition === 'left') return { ...w, snappedPosition: 'top-left' };
          if (w.snappedPosition === 'right') return { ...w, snappedPosition: 'top-right' };
          if (w.snappedPosition === 'bottom-left') return { ...w, snappedPosition: 'top-left' };
          if (w.snappedPosition === 'bottom-right') return { ...w, snappedPosition: 'top-right' };
          if (!w.snappedPosition) return { ...w, isMaximized: true };
        }
        return w;
      });
    });
  });

  useKeyboardShortcut({ key: 'ArrowDown', metaKey: true }, (e) => {
    e.preventDefault();
    setWindows(prev => {
      const highestZ = Math.max(...prev.map(w => w.zIndex), 0);
      return prev.map(w => {
        if (w.zIndex === highestZ && !w.isMinimized && highestZ > 0) {
          if (w.isMaximized) return { ...w, isMaximized: false, snappedPosition: null };
          if (w.snappedPosition === 'left') return { ...w, snappedPosition: 'bottom-left' };
          if (w.snappedPosition === 'right') return { ...w, snappedPosition: 'bottom-right' };
          if (w.snappedPosition === 'top-left') return { ...w, snappedPosition: 'bottom-left' };
          if (w.snappedPosition === 'top-right') return { ...w, snappedPosition: 'bottom-right' };
          if (w.snappedPosition === 'bottom-left' || w.snappedPosition === 'bottom-right') return { ...w, isMinimized: true, snappedPosition: null };
          if (!w.snappedPosition && !w.isMaximized) return { ...w, isMinimized: true }; // actually default windows down minimizes
        }
        return w;
      });
    });
  });

  useEffect(() => {
    const handleOsEnvChange = () => setGlobalOsEnv(localStorage.getItem('atc_os_env') || 'atc-native');
    window.addEventListener('atc_os_env_changed', handleOsEnvChange);
    return () => window.removeEventListener('atc_os_env_changed', handleOsEnvChange);
  }, []);

  useEffect(() => {
    const handleStorageChange = () => {
      setOsMode(localStorage.getItem('atc_os_mode') || 'default');
    };
    window.addEventListener('storage', handleStorageChange);
    // Also set interval to check for it as some local changes don't fire the event
    const checkModeInterval = setInterval(handleStorageChange, 2000);
    return () => {
      window.removeEventListener('storage', handleStorageChange);
      clearInterval(checkModeInterval);
    };
  }, []);

  useEffect(() => {
    const timer = setInterval(() => {
      const now = new Date();
      setTime(now);
      
      const sessionExpiresStr = localStorage.getItem('atc_session_expires');
      if (sessionExpiresStr) {
        const expiresAt = new Date(sessionExpiresStr).getTime();
        const remainingMs = expiresAt - now.getTime();
        
        if (remainingMs <= 0) {
          localStorage.removeItem('atc_session_expires');
          setIsLoggedIn(false);
          setShowSessionWarning(false);
          hasWarnedRef.current = false;
        } else if (remainingMs <= 5 * 60 * 1000 && !hasWarnedRef.current) {
          setShowSessionWarning(true);
          hasWarnedRef.current = true;
        } else if (remainingMs > 5 * 60 * 1000) {
          hasWarnedRef.current = false;
        }
      }
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const getExpirationDate = (duration: string) => {
    const now = new Date();
    switch (duration) {
      case 'custom': {
        const customDaysStr = localStorage.getItem('atc_session_custom_days');
        const customDays = customDaysStr ? parseInt(customDaysStr) : 30;
        return new Date(now.getTime() + customDays * 24 * 60 * 60 * 1000);
      }
      case '3_tage': return new Date(now.getTime() + 3 * 24 * 60 * 60 * 1000);
      case '7_tage': return new Date(now.getTime() + 7 * 24 * 60 * 60 * 1000);
      case '14_tage': return new Date(now.getTime() + 14 * 24 * 60 * 60 * 1000);
      case '1_monat': return new Date(now.getTime() + 30 * 24 * 60 * 60 * 1000);
      case 'restlicher_monat': return new Date(now.getFullYear(), now.getMonth() + 1, 0, 23, 59, 59);
      case 'heute':
      default:
        return new Date(now.getTime() + 24 * 60 * 60 * 1000);
    }
  };

  const handleLogin = (duration: string) => {
    localStorage.setItem('atc_session_duration', duration);
    localStorage.setItem('atc_session_expires', getExpirationDate(duration).toISOString());
    
    if (!localStorage.getItem('atc_session_created')) {
      localStorage.setItem('atc_session_created', new Date().toISOString());
      localStorage.setItem('atc_session_token', 'tk_atc_' + Math.random().toString(36).substr(2, 9).toUpperCase());
    }

    const extensions = JSON.parse(localStorage.getItem('atc_session_extensions') || '[]');
    extensions.unshift({ timestamp: new Date().toISOString(), duration, ip: '192.168.1.1', location: 'Frankfurt, Germany' });
    if (extensions.length > 10) extensions.length = 10;
    localStorage.setItem('atc_session_extensions', JSON.stringify(extensions));

    setIsLoggedIn(true);
  };
  
  const handleLogout = () => {
    localStorage.removeItem('atc_session_expires');
    localStorage.removeItem('atc_session_created');
    localStorage.removeItem('atc_session_token');
    localStorage.removeItem('atc_session_extensions');
    setIsLoggedIn(false);
  };

  const handleContextMenu = (e: React.MouseEvent) => {
    if ((e.target as Element).id === 'desktop-area') {
      e.preventDefault();
      // Keep menu within screen bounds
      const x = Math.min(e.clientX, window.innerWidth - 220);
      const y = Math.min(e.clientY, window.innerHeight - 150);
      setContextMenu({ x, y });
    }
  };

  let sessionTimeRemaining = "";
  let sessionTimeColor = "text-emerald-400";
  let sessionProgressColor = "bg-emerald-400";
  let sessionPercent = 100;
  
  if (isLoggedIn) {
    const sessionExpiresStr = localStorage.getItem('atc_session_expires');
    const sessionCreatedStr = localStorage.getItem('atc_session_created');
    if (sessionExpiresStr) {
      const expiresAt = new Date(sessionExpiresStr).getTime();
      const createdStr = sessionCreatedStr || sessionExpiresStr; // fallback
      const totalMs = expiresAt - new Date(createdStr).getTime();
      const remainingMs = expiresAt - time.getTime();
      
      if (remainingMs > 0) {
        sessionPercent = totalMs > 0 ? (remainingMs / totalMs) * 100 : 100;

        if (remainingMs < 5 * 60 * 1000) {
          sessionTimeColor = "text-red-500 animate-pulse";
          sessionProgressColor = "bg-red-500";
        } else if (remainingMs < 30 * 60 * 1000) {
          sessionTimeColor = "text-amber-500 animate-pulse";
          sessionProgressColor = "bg-amber-500";
        } else if (remainingMs < 60 * 60 * 1000) {
          sessionTimeColor = "text-amber-500";
          sessionProgressColor = "bg-amber-500";
        } else {
          sessionTimeColor = "text-emerald-400";
          sessionProgressColor = "bg-emerald-400";
        }
        
        const diffDays = Math.floor(remainingMs / (1000 * 60 * 60 * 24));
        const diffHours = Math.floor((remainingMs % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
        const diffMinutes = Math.floor((remainingMs % (1000 * 60 * 60)) / (1000 * 60));
        const diffSeconds = Math.floor((remainingMs % (1000 * 60)) / 1000);
        
        if (diffDays > 0) {
          sessionTimeRemaining = `${diffDays}d ${diffHours}h`;
        } else {
          sessionTimeRemaining = `${diffHours.toString().padStart(2, '0')}:${diffMinutes.toString().padStart(2, '0')}:${diffSeconds.toString().padStart(2, '0')}`;
        }
      } else {
        sessionPercent = 0;
      }
    }
  }

  if (!isLoggedIn) {
    return <LoginOverlay onLogin={handleLogin} />;
  }

  const openWindow = (id: string) => {
    setIsStartOpen(false);
    setWindows((prev) => {
      if (prev.some((w) => w.id === id)) {
        setTimeout(() => restoreWindow(id), 0);
        return prev;
      }
      setTimeout(() => {
        setFocusedId(id);
      }, 0);
      const newZIndex = prev.length > 0 ? Math.max(...prev.map(w => w.zIndex || 0)) + 1 : 10;
      setTimeout(() => setMaxZIndex(newZIndex), 0);
      return [
        ...prev,
        { id, isMaximized: true, isMinimized: false, zIndex: newZIndex },
      ];
    });
  };

  const closeWindow = (id: string) => {
    setWindows((prev) => prev.filter((w) => w.id !== id));
    if (focusedId === id) setFocusedId(null);
  };

  const setWindowViewMode = (id: string, mode: 'app' | 'wiki' | 'db') => {
    setWindows((prev) => prev.map((w) => w.id === id ? { ...w, viewMode: mode } : w));
  };

  const tileWindows = () => {
    setWindows((prev) => {
      const openWindows = prev.filter(w => !w.isMinimized);
      if (openWindows.length === 0) return prev;
      
      const count = openWindows.length;
      const cols = Math.ceil(Math.sqrt(count));
      const rows = Math.ceil(count / cols);
      
      const screenW = window.innerWidth - 100; // 100px desktop sidebar
      const screenH = window.innerHeight - 56; // 56px taskbar bottom
      
      const winW = screenW / cols;
      const winH = screenH / rows;
      
        let index = 0;
        return prev.map(w => {
          if (w.isMinimized) return w;
          
          const r = Math.floor(index / cols);
          const c = index % cols;
          
          let snappedPosition: WindowState['snappedPosition'] = null;
          if (count === 2) {
            snappedPosition = index === 0 ? 'left' : 'right';
          } else if (count === 4) {
            if (index === 0) snappedPosition = 'top-left';
            if (index === 1) snappedPosition = 'top-right';
            if (index === 2) snappedPosition = 'bottom-left';
            if (index === 3) snappedPosition = 'bottom-right';
          }

          index++;
          
          if (snappedPosition) {
            return {
              ...w,
              isMaximized: false,
              snappedPosition,
              x: undefined,
              y: undefined,
              width: undefined,
              height: undefined
            };
          }

          return {
             ...w,
             x: c * winW,
             y: r * winH,
             width: winW,
             height: winH,
             isMaximized: false,
             snappedPosition: null
          };
        });
    });
  };

  const setMaximized = (id: string, max: boolean) => {
    setWindows((prev) =>
      prev.map((w) => (w.id === id ? { ...w, isMaximized: max } : w)),
    );
  };

  const setMinimized = (id: string) => {
    setWindows((prev) =>
      prev.map((w) => (w.id === id ? { ...w, isMinimized: true } : w)),
    );
    if (focusedId === id) setFocusedId(null);
  };

  const restoreWindow = (id: string) => {
    setWindows((prev) => {
      const newZIndex = prev.length > 0 ? Math.max(...prev.map(w => w.zIndex || 0)) + 1 : 10;
      setTimeout(() => setMaxZIndex(newZIndex), 0);
      return prev.map((w) =>
        w.id === id ? { ...w, isMinimized: false, zIndex: newZIndex } : w,
      );
    });
    setFocusedId(id);
  };

  const focusWindow = (id: string) => {
    if (focusedId === id) return;
    setWindows((prev) => {
      const newZIndex = prev.length > 0 ? Math.max(...prev.map(w => w.zIndex || 0)) + 1 : 10;
      setTimeout(() => setMaxZIndex(newZIndex), 0);
      return prev.map((w) => (w.id === id ? { ...w, zIndex: newZIndex } : w));
    });
    setFocusedId(id);
  };

  return (
    <div 
      className="relative w-screen h-screen overflow-hidden bg-slate-900 text-slate-100 font-sans selection:bg-atc-cyan/20"
      id="desktop-area"
      onContextMenu={handleContextMenu}
      onClick={closeContextMenu}
      onTouchStart={(e) => {
        if (e.touches.length === 1 && (e.target as Element).id === 'desktop-area') {
          touchTimerRef.current = setTimeout(() => {
            if (navigator.vibrate) navigator.vibrate(50);
            const touch = e.touches[0];
            const x = Math.min(touch.clientX, window.innerWidth - 220);
            const y = Math.min(touch.clientY, window.innerHeight - 150);
            setContextMenu({ x, y });
          }, 800);
        } else if (e.touches.length === 2) {
          touchStartPosRef.current = {
            y1: e.touches[0].clientY,
            y2: e.touches[1].clientY
          };
        }
      }}
      onTouchMove={(e) => {
        if (touchTimerRef.current) clearTimeout(touchTimerRef.current);
        
        if (e.touches.length === 2 && touchStartPosRef.current) {
          const y1 = e.touches[0].clientY;
          const y2 = e.touches[1].clientY;
          
          const dy1 = y1 - touchStartPosRef.current.y1;
          const dy2 = y2 - touchStartPosRef.current.y2;
          
          if (dy1 > 100 && dy2 > 100) {
            // Minimize all on two-finger swipe down
            if (navigator.vibrate) navigator.vibrate([30, 50, 30]);
            setWindows(prev => prev.map(w => ({ ...w, isMinimized: true })));
            touchStartPosRef.current = null;
          }
        }
      }}
      onTouchEnd={() => {
        if (touchTimerRef.current) clearTimeout(touchTimerRef.current);
        touchStartPosRef.current = null;
      }}
    >
      {/* OS Background */}
      <div className="absolute inset-0 z-0 select-none pointer-events-none overflow-hidden bg-[#02040a]">
        {/* Futuristic Image Background */}
        <div className="absolute inset-0 bg-cover bg-center opacity-30 mix-blend-screen transition-all duration-700" style={{ backgroundImage: `url('${wallpaperUrl}')` }} />
        
        {/* Core Gradients */}
        <div className="absolute inset-0 bg-gradient-to-br from-indigo-900/60 via-[#040714]/90 to-black/90" />
        
        {/* Cyber Grid */}
        <div className="absolute inset-0 bg-[linear-gradient(rgba(99,102,241,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(99,102,241,0.03)_1px,transparent_1px)] bg-[size:40px_40px] [mask-image:radial-gradient(ellipse_60%_60%_at_50%_0%,#000_20%,transparent_100%)]" />

        {/* Outer Glows */}
        <div className="absolute top-0 right-0 w-[1000px] h-[1000px] bg-indigo-500/10 rounded-full blur-[150px] -translate-y-1/2 translate-x-1/3" />
        <div className="absolute bottom-0 left-0 w-[800px] h-[800px] bg-cyan-500/10 rounded-full blur-[120px] translate-y-1/3 -translate-x-1/4" />
        
        {/* Grain */}
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.04]" />
      </div>

      <div className="absolute inset-0 z-0">
        <AppGlobeView />
      </div>

      {/* Top Left Profile Widget */}
      <div className="absolute top-4 left-[116px] z-10">
        <button 
          onClick={() => openWindow('user_profile')}
          className="flex items-center gap-3 px-4 py-2 bg-[#090b14]/80 backdrop-blur-md border border-white/10 hover:bg-white/5 transition-colors rounded-xl shadow-lg group focus:outline-none"
        >
          <div className="w-8 h-8 rounded-full bg-indigo-500/20 border border-indigo-500/30 flex items-center justify-center text-indigo-400 group-hover:scale-105 transition-transform">
            <User className="w-4 h-4" />
          </div>
          <div className="flex flex-col text-left">
            <span className="text-sm font-bold text-slate-200">Local Profile</span>
            <span className="text-[10px] text-emerald-400 font-mono tracking-widest uppercase">Decentralized</span>
          </div>
        </button>
      </div>

      {/* Top Right Wallet Widget */}
      <div className="absolute top-4 right-4 z-10">
        <button 
          onClick={() => openWindow('atc_wallet')}
          className="flex items-center gap-3 px-4 py-2 bg-[#090b14]/80 backdrop-blur-md border border-white/10 hover:bg-white/5 transition-colors rounded-xl shadow-lg group focus:outline-none"
        >
          <div className="flex flex-col text-right">
            <span className="text-sm font-bold text-slate-200">1,248.50 ATC</span>
            <span className="text-[10px] text-emerald-400 font-mono flex items-center justify-end gap-1">
              <span className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" /> Connected
            </span>
          </div>
          <div className="w-8 h-8 rounded-full bg-emerald-500/10 border border-emerald-500/30 flex items-center justify-center text-emerald-400 group-hover:scale-105 transition-transform">
            <Wallet className="w-4 h-4" />
          </div>
        </button>
      </div>

      {/* Desktop Icons Sidebar */}
      <div className="absolute top-0 bottom-[56px] left-0 w-[100px] bg-[#040714]/60 backdrop-blur-md border-r border-white/5 z-0 flex flex-col py-6">
        {(() => {
          const DESKTOP_ICONS = [
            { id: "overview", label: "Overview", icon: LayoutGrid },
            { id: "integrations", label: "Integrations", icon: Layers },
            { id: "ai_software_flowchart", label: "Idea to App", icon: Rocket },
            { id: "ai_software_workflow", label: "KI Workflow", icon: Bot },
            { id: "ecosystem_tree", label: "GlobusOS Öko", icon: Globe },
            { id: "ecosystem_uml", label: "Ecosystem UML", icon: Network },
            { id: "zk_visualizer", label: "ZKP Visualisierung", icon: Shield },
            { id: "roadmap", label: "Roadmap UI", icon: Activity },
            { id: "ecosystem_designer", label: "Ecosystem Builder", icon: Layers },
            { id: "api_orchestrator", label: "API Orchestrator", icon: Workflow },
            { id: "project_audit", label: "Audit & QA", icon: ShieldCheck },
            { id: "rescue_tools", label: "Rescue System", icon: ShieldAlert },
            { id: "system_finder", label: "System Finder", icon: Search },
            { id: "software_db", label: "Knowledge DB", icon: Database },
            { id: "atown_os", label: "ATC-OS Node", icon: Bot },
            { id: "ai_kernel", label: "AI Kernel", icon: BrainCircuit },
            { id: "dependency_map", label: "Dependency Map", icon: Network },
            { id: "agent_civ", label: "Agent Civ", icon: Network },
            { id: "atclang", label: "ATCLang Engine", icon: Code },
            { id: "api_interfaces", label: "API Gateways", icon: Network },
            { id: "zk_proofs", label: "ZK Subsystem", icon: Shield },
            { id: "crypto_engine", label: "Ed25519 Engine", icon: Shield },
            { id: "zk_circuit", label: "ZK Circuit", icon: Boxes },
            { id: "cicd_pipeline", label: "CI/CD Pipeline", icon: Server },
            { id: "testnet_simulation", label: "Testnet Simulation", icon: Network },
            { id: "poai_engine", label: "PoAI Engine", icon: BrainCircuit },
            { id: "genesis_generator", label: "Genesis Generator", icon: Database },
            { id: "atc_whitepaper", label: "ATC Whitepaper", icon: BookOpen },
            { id: "testnet_orchestration", label: "Testnet Setup", icon: Server },
            { id: "mainnet_launch", label: "Mainnet Launch", icon: Rocket },
            { id: "defi_liquidity", label: "ATC Liquidity", icon: Coins },
            { id: "datalake", label: "Data Lake", icon: Database },
            { id: "a_town_dashboard", label: "A-Town", icon: Pickaxe },
            { id: "compliance", label: "Compliance", icon: ShieldCheck },
            { id: "source_code", label: "Source Code", icon: Code },
            { id: "project_hub", label: "Project Hub", icon: Map },
            { id: "file_mgr", label: "Daten Explorer", icon: Folder },
            { id: "storage_mgr", label: "Laufwerke", icon: HardDrive },
            { id: "diagnostics", label: "Diagnose", icon: AlertTriangle },
            { id: "task_mgr", label: "System Mgr", icon: Activity },
            { id: "settings", label: "Einstellungen", icon: Settings },
            { id: "dev_tools", label: "Dev Tools", icon: Wrench },
            { id: "atc_benchmark", label: "Benchmark", icon: Zap },
            { id: "atc_sys_settings", label: "System", icon: Cpu },
            { id: "atc_soft_settings", label: "Software", icon: Settings },
            { id: "atc_hard_settings", label: "Hardware", icon: HardDrive },
            { id: "asset_vault", label: "Asset Vault", icon: Database },
            { id: "anti_cheat", label: "Anti-Cheat", icon: Shield },
            { id: "office_canvas", label: "ATS-OFFICE", icon: FileText },
            { id: "playground_ui", label: "ATS-PLAY", icon: PlaySquare },
            { id: "genesis_cockpit", label: "Genesis Cockpit", icon: Zap },
            { id: "dao_governance", label: "DAO Governance", icon: Network },
            { id: "final_integrator", label: "Final Integrator", icon: CheckCircle },
            { id: "archive_guardian", label: "Archive Guardian", icon: Save },
            { id: "hash_manifest", label: "Hash Manifest", icon: Database },
            { id: "singularity_monitor", label: "Singularity Monitor", icon: Activity },
            { id: "anonymity_shield", label: "Anonymity Shield", icon: Lock },
            { id: "transparency_monitor", label: "Transparency", icon: Eye },
            { id: "privacy_gatekeeper", label: "Privacy Gate", icon: Shield },
            { id: "lex_ai_monitor", label: "LEX-AI Monitor", icon: FileText },
            { id: "truth_gate", label: "Truth Gate", icon: CheckCircle },
            { id: "atc_lang_terminal", label: "ATC-LANG", icon: MonitorPlay },
            { id: "protocol_integrator", label: "Protocol Integrator", icon: ArrowLeftRight },
            { id: "survival_control", label: "Survival Control", icon: Server },
            { id: "encyclopedia_nexus", label: "Living Codex", icon: Search },
            { id: "evolution_nexus", label: "Evolution Nexus", icon: Network },
            { id: "evolution_journal", label: "Evolution Journal", icon: FileText },
            { id: "multi_pass_monitor", label: "Multi-Pass", icon: CheckCircle },
            { id: "rollback_manager", label: "Rollback Manager", icon: Clock },
            { id: "auto_optimizer", label: "Auto-Optimizer", icon: Cpu },
            { id: "governance_sentinel", label: "Gov Sentinel", icon: Shield },
            { id: "civilization_command", label: "Civilization CMD", icon: Network },
          ];
          const ICONS_PER_PAGE = 7;
          const totalPages = Math.ceil(DESKTOP_ICONS.length / ICONS_PER_PAGE);
          const currentIcons = DESKTOP_ICONS.slice(iconPage * ICONS_PER_PAGE, (iconPage + 1) * ICONS_PER_PAGE);

          const handleKeyDown = (e: React.KeyboardEvent, index: number) => {
            if (e.key === 'ArrowDown') {
              e.preventDefault();
              const next = document.getElementById(`desktop-icon-${currentIcons[(index + 1) % currentIcons.length]?.id}`);
              next?.focus();
            } else if (e.key === 'ArrowUp') {
              e.preventDefault();
              const prev = document.getElementById(`desktop-icon-${currentIcons[(index - 1 + currentIcons.length) % currentIcons.length]?.id}`);
              prev?.focus();
            } else if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              openWindow(currentIcons[index].id);
            }
          };

          return (
            <>
              <div className="flex-1 flex flex-col gap-6 items-center w-full px-2" role="tablist">
                <AnimatePresence mode="wait">
                  <motion.div
                    key={iconPage}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, x: -10 }}
                    transition={{ duration: 0.15 }}
                    className="flex flex-col gap-4 w-full items-center"
                  >
                    {currentIcons.map((item, index) => (
                      <button
                        key={`desktop-icon-${item.id}`}
                        id={`desktop-icon-${item.id}`}
                        role="tab"
                        onKeyDown={(e) => handleKeyDown(e, index)}
                        onClick={() => openWindow(item.id)}
                        className="flex flex-col items-center gap-1.5 w-full py-1.5 rounded-xl hover:bg-white/5 transition-all focus:bg-white/10 outline-none group text-center"
                      >
                        <div
                          className={`w-[48px] h-[48px] rounded-[14px] bg-gradient-to-br from-[#0a0d1e] to-[#050711] border shadow flex items-center justify-center transition-all group-hover:-translate-y-0.5 group-hover:shadow-[0_4px_12px_rgba(0,0,0,0.4)] ${item.id === "overview" ? "text-atc-purple border-atc-purple/40 shadow-atc-purple/10" : "text-atc-cyan border-white/10 group-hover:border-atc-cyan/40"}`}
                        >
                          <item.icon className="w-5 h-5 drop-shadow-[0_0_5px_rgba(6,182,212,0.3)]" />
                        </div>
                        <span className="text-[10px] w-full px-1 font-medium drop-shadow-md tracking-tight leading-tight line-clamp-2 select-none text-slate-400 group-hover:text-slate-200">
                          {item.label}
                        </span>
                      </button>
                    ))}
                  </motion.div>
                </AnimatePresence>
              </div>

              {/* Pagination Controls */}
              {totalPages > 1 && (
                <div className="mt-4 pt-4 border-t border-white/10 flex flex-col items-center justify-center w-[72px] mx-auto shrink-0">
                  <div className="flex w-full justify-between items-center bg-[#050711] rounded-lg border border-white/10 p-1 shadow-inner">
                    <button
                      onClick={() => setIconPage(p => Math.max(0, p - 1))}
                      disabled={iconPage === 0}
                      className="p-1 rounded bg-transparent hover:bg-white/10 disabled:opacity-30 disabled:cursor-not-allowed text-slate-300 transition-colors"
                    >
                      <ChevronLeft className="w-3.5 h-3.5" />
                    </button>
                    <span className="text-[10px] font-mono text-slate-400 font-bold select-none cursor-default">
                      {iconPage + 1}/{totalPages}
                    </span>
                    <button
                      onClick={() => setIconPage(p => Math.min(totalPages - 1, p + 1))}
                      disabled={iconPage === totalPages - 1}
                      className="p-1 rounded bg-transparent hover:bg-white/10 disabled:opacity-30 disabled:cursor-not-allowed text-slate-300 transition-colors"
                    >
                      <ChevronRight className="w-3.5 h-3.5" />
                    </button>
                  </div>
                </div>
              )}
            </>
          );
        })()}
      </div>

      {/* Windows Area */}
      {windows.map((win) => {
        const config = WINDOWS_MAP[win.id];
        if (!config || win.isMinimized) return null;

        const ComponentNode = config.component;
        const isFocused = focusedId === win.id;

        const handleFocusTrap = (e: React.KeyboardEvent) => {
          if (!isFocused || e.key !== 'Tab') return;
          const focusableElements = e.currentTarget.querySelectorAll('a[href], button, textarea, input[type="text"], input[type="radio"], input[type="checkbox"], select, [tabindex]:not([tabindex="-1"])');
          const firstElement = focusableElements[0] as HTMLElement;
          const lastElement = focusableElements[focusableElements.length - 1] as HTMLElement;

          if (e.shiftKey) {
            if (document.activeElement === firstElement) {
              lastElement?.focus();
              e.preventDefault();
            }
          } else {
            if (document.activeElement === lastElement) {
              firstElement?.focus();
              e.preventDefault();
            }
          }
        };

        return (
          <motion.div
            key={`workspace-${win.id}`}
            drag={!win.isMaximized}
            dragMomentum={false}
            onDragStart={() => {
              if (navigator.vibrate) navigator.vibrate(10);
              if (win.snappedPosition || win.isMaximized) {
                setWindows(prev => prev.map(w => w.id === win.id ? { ...w, snappedPosition: null, isMaximized: false } : w));
              }
            }}
            onDragEnd={(e, info) => {
              const { point, offset } = info;
              
              const isLeftEdge = point.x < 20;
              const isRightEdge = point.x > window.innerWidth - 20;
              const isTopEdge = point.y < 20;
              const isBottomEdge = point.y > window.innerHeight - 80;

              let updatedPosition: 'left' | 'right' | 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | null = null;
              
              if (isTopEdge && isLeftEdge) updatedPosition = 'top-left';
              else if (isTopEdge && isRightEdge) updatedPosition = 'top-right';
              else if (isBottomEdge && isLeftEdge) updatedPosition = 'bottom-left';
              else if (isBottomEdge && isRightEdge) updatedPosition = 'bottom-right';
              else if (isLeftEdge) updatedPosition = 'left';
              else if (isRightEdge) updatedPosition = 'right';

              if (updatedPosition || isTopEdge) {
                if ('vibrate' in navigator) navigator.vibrate(20);
              }

              if (updatedPosition) {
                setWindows(prev => prev.map(w => w.id === win.id ? { ...w, snappedPosition: updatedPosition, isMaximized: false, x: undefined, y: undefined } : w));
              } else if (isTopEdge) {
                setWindows(prev => prev.map(w => w.id === win.id ? { ...w, isMaximized: true, snappedPosition: null, x: undefined, y: undefined } : w));
              } else {
                setWindows(prev => prev.map(w => w.id === win.id ? { 
                  ...w, 
                  snappedPosition: null, 
                  x: (w.x || 0) + offset.x, 
                  y: (w.y || 0) + offset.y 
                } : w));
              }
            }}
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{
              opacity: 1,
              scale: 1,
              width: win.isMaximized ? "100vw" : win.snappedPosition ? "50vw" : (win.width || "85vw"),
              height: win.isMaximized ? "calc(100vh - 56px)" : (win.snappedPosition && win.snappedPosition.includes('-')) ? "calc(50vh - 28px)" : win.snappedPosition ? "calc(100vh - 56px)" : (win.height || "80vh"),
              x: win.isMaximized ? 0 : win.snappedPosition?.includes('left') ? 0 : win.snappedPosition?.includes('right') ? window.innerWidth / 2 : win.x || undefined,
              y: win.isMaximized || win.snappedPosition === 'left' || win.snappedPosition === 'right' || win.snappedPosition?.includes('top') ? 0 : win.snappedPosition?.includes('bottom') ? (window.innerHeight - 56) / 2 : win.y || undefined,
            }}
            transition={{
              type: "spring",
              stiffness: win.snappedPosition ? 300 : 400,
              damping: win.snappedPosition ? 14 : 30,
            }}
            onMouseDownCapture={() => focusWindow(win.id)}
            onTouchStartCapture={() => focusWindow(win.id)}
            onKeyDown={handleFocusTrap}
            style={{
              position: "absolute",
              zIndex: win.zIndex,
              top: win.isMaximized || win.snappedPosition ? 0 : "10vh",
              left: win.isMaximized || win.snappedPosition ? 0 : "7.5vw",
              backgroundColor: `rgba(5, 8, 21, ${windowOpacity / 100})`,
              backdropFilter: windowOpacity < 100 ? 'blur(20px)' : 'none',
              touchAction: "none"
            }}
            className={`flex flex-col overflow-hidden shadow-[0_0_50px_rgba(0,0,0,0.8)] ${
              isFocused
                ? "border border-indigo-500/40 shadow-[0_0_40px_rgba(99,102,241,0.2)]"
                : "border border-white/10"
            } ${win.isMaximized ? "rounded-none" : "rounded-xl"}`}
          >
            {/* Titlebar */}
            <div 
              className={`window-titlebar shrink-0 flex items-center px-3 select-none cursor-pointer ${
              osMode === 'macos' 
                ? 'h-12 bg-white/5 border-b border-white/10 justify-center relative' 
                : osMode === 'windows'
                ? 'h-10 bg-black/50 border-b border-white/5 justify-between'
                : 'h-10 bg-gradient-to-r from-indigo-900/40 to-transparent border-b border-indigo-500/20 justify-between'
            }`}>
              
              {/* macOS Window Controls */}
              {osMode === 'macos' && (
                <div className="flex items-center gap-2 sm:gap-2 absolute left-4">
                  <button
                    onClick={(e) => {
                      if (navigator.vibrate) navigator.vibrate(10);
                      e.stopPropagation();
                      closeWindow(win.id);
                    }}
                    className="w-3 h-3 rounded-full bg-red-500 hover:bg-red-400 border border-black/20 min-w-[20px] min-h-[20px] sm:min-w-[12px] sm:min-h-[12px]"
                  />
                  <button
                    onClick={(e) => {
                      if (navigator.vibrate) navigator.vibrate(10);
                      e.stopPropagation();
                      setMinimized(win.id);
                    }}
                    className="w-3 h-3 rounded-full bg-amber-500 hover:bg-amber-400 border border-black/20 min-w-[20px] min-h-[20px] sm:min-w-[12px] sm:min-h-[12px]"
                  />
                  <button
                    onClick={(e) => {
                      if (navigator.vibrate) navigator.vibrate(10);
                      e.stopPropagation();
                      setMaximized(win.id, !win.isMaximized);
                    }}
                    className="w-3 h-3 rounded-full bg-green-500 hover:bg-green-400 border border-black/20 min-w-[20px] min-h-[20px] sm:min-w-[12px] sm:min-h-[12px]"
                  />
                </div>
              )}

              {/* Title & Icon */}
              <div className="flex items-center gap-3">
                {osMode !== 'macos' && (
                  <div className="p-1.5 rounded bg-indigo-500/10 border border-indigo-500/30">
                    <config.icon className="w-3.5 h-3.5 text-indigo-400" />
                  </div>
                )}
                <span className={`font-bold text-xs tracking-widest uppercase font-mono ${osMode === 'macos' ? 'text-slate-300' : 'text-indigo-100'}`}>
                  {config.label}
                </span>
                <button
                   onClick={(e) => {
                     if (navigator.vibrate) navigator.vibrate(10);
                     e.stopPropagation();
                     setShareDialogId(win.id);
                   }}
                   className="ml-2 p-1.5 text-slate-400 hover:text-white hover:bg-white/10 rounded transition-colors"
                   title="Share Applet View"
                >
                   <Share2 className="w-3.5 h-3.5" />
                </button>
              </div>

              {/* View Mode Toggle Controls */}
              <div className="flex-1 flex justify-center items-center pointer-events-auto" onPointerDown={e => e.stopPropagation()}>
                <div className="flex p-0.5 bg-black/30 border border-white/5 rounded mx-2">
                  <button 
                    onClick={() => setWindowViewMode(win.id, 'app')}
                    className={`flex items-center gap-1.5 px-3 py-1 rounded-sm text-[10px] font-bold uppercase tracking-wider transition-colors ${(!win.viewMode || win.viewMode === 'app') ? 'bg-white/10 text-white shadow-sm' : 'text-slate-500 hover:text-slate-300'}`}
                  >
                     <Monitor className="w-3 h-3" /> App
                  </button>
                  <button 
                    onClick={() => setWindowViewMode(win.id, 'wiki')}
                    className={`flex items-center gap-1.5 px-3 py-1 rounded-sm text-[10px] font-bold uppercase tracking-wider transition-colors ${win.viewMode === 'wiki' ? 'bg-emerald-500/20 text-emerald-400 shadow-sm' : 'text-slate-500 hover:text-emerald-400/50'}`}
                  >
                     <Book className="w-3 h-3" /> Wiki
                  </button>
                  <button 
                    onClick={() => setWindowViewMode(win.id, 'db')}
                    className={`flex items-center gap-1.5 px-3 py-1 rounded-sm text-[10px] font-bold uppercase tracking-wider transition-colors ${win.viewMode === 'db' ? 'bg-indigo-500/20 text-indigo-400 shadow-sm' : 'text-slate-500 hover:text-indigo-400/50'}`}
                  >
                     <Database className="w-3 h-3" /> DB
                  </button>
                </div>
              </div>

              {/* Windows / Default Controls */}
              {osMode !== 'macos' && (
                <div className="flex items-center gap-1.5">
                  <button
                    onClick={(e) => {
                      if (navigator.vibrate) navigator.vibrate(10);
                      e.stopPropagation();
                      setMinimized(win.id);
                    }}
                    className={`w-10 h-10 sm:w-8 sm:h-8 flex items-center justify-center text-slate-400 hover:bg-white/10 hover:text-white transition-colors ${osMode==='windows'?'rounded-none':'rounded'}`}
                  >
                    <Minus className="w-4 h-4" />
                  </button>
                  <button
                    onClick={(e) => {
                      if (navigator.vibrate) navigator.vibrate(10);
                      e.stopPropagation();
                      setMaximized(win.id, !win.isMaximized);
                    }}
                    className={`w-10 h-10 sm:w-8 sm:h-8 flex items-center justify-center text-slate-400 hover:bg-white/10 hover:text-white transition-colors ${osMode==='windows'?'rounded-none':'rounded'}`}
                  >
                    {win.isMaximized ? (
                      <Minimize2 className="w-4 h-4" />
                    ) : (
                      <Maximize2 className="w-4 h-4" />
                    )}
                  </button>
                  <button
                    onClick={(e) => {
                      if (navigator.vibrate) navigator.vibrate(10);
                      e.stopPropagation();
                      closeWindow(win.id);
                    }}
                    className={`w-10 h-10 sm:w-8 sm:h-8 flex items-center justify-center text-slate-400 hover:bg-red-500 hover:text-white transition-colors ${osMode==='windows'?'rounded-none':'rounded'}`}
                  >
                    <X className="w-4 h-4" />
                  </button>
                </div>
              )}
            </div>

            {/* Window Content */}
            <div 
              className="flex-1 overflow-y-auto custom-scrollbar relative p-4"
              onPointerDown={e => e.stopPropagation()}
              style={{ touchAction: "auto" }}
            >
              {/* Pass openWindow directly so links within WikiView open new windows */}
              <React.Suspense fallback={
                <div className="flex items-center justify-center h-full text-slate-400">
                  <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-500"></div>
                </div>
              }>
                {(!win.viewMode || win.viewMode === 'app') && <ComponentNode setActiveTab={openWindow} />}
                {win.viewMode === 'wiki' && <WindowWikiView id={win.id} title={config.label} />}
                {win.viewMode === 'db' && <WindowDbView id={win.id} title={config.label} />}
              </React.Suspense>
            </div>

            {/* Custom Resize Handle */}
            {!win.isMaximized && !win.snappedPosition && (
              <div
                className="absolute bottom-0 right-0 w-6 h-6 cursor-se-resize z-[1000] opacity-50 hover:opacity-100 flex items-end justify-end p-1.5"
                onPointerDown={(e) => {
                  e.stopPropagation();
                  e.preventDefault();
                  const startX = e.clientX;
                  const startY = e.clientY;
                  const targetElement = e.currentTarget.parentElement;
                  if (!targetElement) return;
                  const startWidth = targetElement.offsetWidth;
                  const startHeight = targetElement.offsetHeight;

                  const handlePointerMove = (moveEvent: PointerEvent) => {
                    const newWidth = Math.max(300, startWidth + (moveEvent.clientX - startX));
                    const newHeight = Math.max(200, startHeight + (moveEvent.clientY - startY));
                    
                    setWindows((prev) =>
                      prev.map((w) =>
                        w.id === win.id
                          ? { ...w, width: newWidth, height: newHeight }
                          : w
                      )
                    );
                  };

                  const handlePointerUp = () => {
                    document.removeEventListener("pointermove", handlePointerMove);
                    document.removeEventListener("pointerup", handlePointerUp);
                  };

                  document.addEventListener("pointermove", handlePointerMove);
                  document.addEventListener("pointerup", handlePointerUp);
                }}
              >
                  <div className="w-2.5 h-2.5 bg-indigo-500/50 rounded-br-sm" />
              </div>
            )}
          </motion.div>
        );
      })}

      {/* Start Menu */}
      <AnimatePresence>
        {isStartOpen && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 20 }}
            className={`absolute z-[9999] flex flex-col overflow-hidden bg-[#050811]/95 backdrop-blur-3xl border border-indigo-500/30 rounded-2xl shadow-[0_0_50px_rgba(0,0,0,0.8)] ${
              osMode === 'windows'
                ? 'bottom-[60px] left-1/2 -translate-x-1/2 w-[400px] max-h-[80vh]'
                : osMode === 'macos'
                ? 'bottom-[80px] left-1/2 -translate-x-1/2 w-[400px] max-h-[80vh]'
                : osMode === 'linux'
                ? 'top-[40px] left-4 w-[400px] max-h-[80vh]'
                : 'bottom-[70px] left-4 w-[400px] max-h-[80vh]'
            }`}
          >
            <div className="absolute inset-0 bg-gradient-to-br from-indigo-500/5 to-transparent pointer-events-none" />
            <div className="p-6 pb-2 relative z-10 overflow-y-auto custom-scrollbar">
              <div className="flex items-center gap-3 mb-6">
                <div className="w-10 h-10 rounded-xl bg-indigo-500/20 flex items-center justify-center text-indigo-400 border border-indigo-500/40 shadow-[0_0_15px_rgba(99,102,241,0.3)]">
                  <Component className="w-5 h-5" />
                </div>
                <div>
                  <h3 className="text-xl font-bold tracking-tight text-white">
                    ATC-OS Server
                  </h3>
                  <p className="text-xs text-slate-400 font-mono uppercase tracking-widest">
                    Master Control Panel
                  </p>
                </div>
              </div>

              <div className="relative overflow-hidden min-h-[300px]">
                <AnimatePresence mode="wait">
                  {!activeCategory ? (
                    <motion.div
                      key="categories"
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      exit={{ opacity: 0, x: -20 }}
                      transition={{ duration: 0.2 }}
                      className="flex flex-col gap-2 mb-6"
                    >
                      {WINDOW_CATEGORIES.map((category) => {
                        const appsInCategory = Object.entries(WINDOWS_MAP).filter(
                          ([_, config]) => config.category === category.id,
                        );
                        if (appsInCategory.length === 0) return null;
  
                        return (
                          <button
                            key={`category-${category.id}`}
                            onClick={() => setActiveCategory(category.id)}
                            className="flex items-center justify-between p-3 rounded-xl hover:bg-white/5 border border-transparent hover:border-white/10 transition-all group"
                          >
                            <div className="flex items-center gap-4">
                              <div className="p-2.5 rounded-xl bg-black/40 border border-white/5 shadow-inner group-hover:border-indigo-500/30 group-hover:bg-indigo-500/10 transition-colors shrink-0">
                                <category.icon className="w-5 h-5 text-indigo-400 group-hover:text-indigo-300" />
                              </div>
                              <div className="flex flex-col text-left">
                                <span className="text-sm font-bold text-slate-200 group-hover:text-white transition-colors">{category.label}</span>
                                <span className="text-xs text-slate-500 font-mono group-hover:text-indigo-200/70 transition-colors">{appsInCategory.length} Apps & Tools</span>
                              </div>
                            </div>
                            <ChevronRight className="w-5 h-5 text-slate-600 group-hover:text-indigo-400 group-hover:translate-x-1 transition-all shrink-0" />
                          </button>
                        );
                      })}
                    </motion.div>
                  ) : (
                    <motion.div
                      key="apps"
                      initial={{ opacity: 0, x: 20 }}
                      animate={{ opacity: 1, x: 0 }}
                      exit={{ opacity: 0, x: 20 }}
                      transition={{ duration: 0.2 }}
                      className="flex flex-col h-full"
                    >
                      <button
                        onClick={() => setActiveCategory(null)}
                        className="flex items-center gap-2 text-xs font-bold text-slate-400 hover:text-white mb-4 px-2 tracking-widest transition-colors group"
                      >
                        <ChevronLeft className="w-4 h-4 shrink-0 group-hover:-translate-x-1 transition-transform" /> 
                        <span className="uppercase">Zurück zur Übersicht</span>
                      </button>
                      <div className="grid grid-cols-2 gap-3 mb-6">
                        {Object.entries(WINDOWS_MAP)
                          .filter(([_, config]) => config.category === activeCategory)
                          .map(([id, config]) => (
                          <button
                            key={`start-menu-${id}`}
                            onClick={() => {
                              openWindow(id);
                              setIsStartOpen(false);
                              setActiveCategory(null);
                            }}
                            className={`flex flex-col items-start gap-4 p-4 rounded-xl border border-white/5 bg-black/20 hover:bg-white/10 hover:border-white/20 transition-all text-left group`}
                          >
                            <div
                              className={`p-2.5 rounded-lg bg-black/40 border border-white/10 shadow-inner group-hover:scale-110 transition-transform shrink-0`}
                            >
                              <config.icon
                                className={`w-5 h-5 ${id === "overview" ? "text-atc-purple" : "text-atc-cyan"}`}
                              />
                            </div>
                            <span className="text-sm font-semibold text-slate-200 group-hover:text-white line-clamp-2 w-full leading-tight">
                              {config.label}
                            </span>
                          </button>
                        ))}
                      </div>
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            </div>
            <div className="mt-auto p-4 border-t border-white/5 bg-black/20 flex justify-between items-center">
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 rounded-full bg-gradient-to-br from-indigo-100 to-blue-100 flex items-center justify-center text-[#060a16] font-bold text-xs">
                  MW
                </div>
                <span className="text-sm font-medium">Michael W.</span>
              </div>
              <button onClick={handleLogout} className="p-2 text-slate-400 hover:text-white transition-colors">
                <Power className="w-4 h-4" />
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      <AnimatePresence>
        {showTouchTutorial && (
          <motion.div 
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
            className="fixed inset-0 z-[10000] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
          >
            <div className="bg-[#090b14] border border-indigo-500/50 shadow-[0_0_50px_rgba(99,102,241,0.15)] rounded-2xl w-full max-w-md overflow-hidden flex flex-col relative" style={{ touchAction: 'auto' }}>
              <div className="absolute top-0 left-0 right-0 h-1 bg-indigo-500/50" />
              <div className="p-8 pb-4 flex flex-col items-center text-center mt-2">
                <div className="w-12 h-12 rounded-full bg-indigo-500/10 flex items-center justify-center mb-4 text-indigo-400 shadow-inner border border-indigo-500/20">
                  <MonitorSmartphone className="w-6 h-6" />
                </div>
                <h3 className="text-xl font-bold text-white mb-4">Touch Controls Optimized</h3>
                <div className="space-y-4 text-left w-full mb-4">
                   <div className="flex items-start gap-3">
                      <div className="p-1.5 bg-white/5 rounded-lg border border-white/5"><div className="w-4 h-4 bg-white/40 rounded-full" /></div>
                      <div>
                         <p className="text-sm font-semibold text-white">Long Press</p>
                         <p className="text-xs text-slate-400">Hold anywhere on the desktop to open the Context Menu.</p>
                      </div>
                   </div>
                   <div className="flex items-start gap-3">
                      <div className="flex gap-1 p-1.5 bg-white/5 rounded-lg border border-white/5"><div className="w-4 h-4 bg-white/40 rounded-full" /><div className="w-4 h-4 bg-white/40 rounded-full" /></div>
                      <div>
                         <p className="text-sm font-semibold text-white">Two-Finger Pull Down</p>
                         <p className="text-xs text-slate-400">Swipe down with two fingers on the workspace to minimize all active windows.</p>
                      </div>
                   </div>
                   <div className="flex items-start gap-3">
                      <div className="p-1.5 bg-white/5 rounded-lg border border-white/5"><LayoutGrid className="w-4 h-4 text-slate-400" /></div>
                      <div>
                         <p className="text-sm font-semibold text-white">Taskbar Interactions</p>
                         <p className="text-xs text-slate-400">Taskbar icons have expanded touch targets. Tap to focus or restore windows.</p>
                      </div>
                   </div>
                </div>
              </div>
              <div className="p-6 pt-2 bg-[#060a16]">
                <button
                  onClick={closeTouchTutorial}
                  className="w-full py-2.5 rounded-lg bg-indigo-500 hover:bg-indigo-600 text-white font-medium transition-colors border-none outline-none shadow-lg shadow-indigo-500/20"
                >
                  Got it, thanks!
                </button>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      <AnimatePresence>
        {showSessionWarning && (
          <motion.div 
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
            className="fixed inset-0 z-[10000] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
          >
            <div className="bg-[#090b14] border border-amber-500/50 shadow-[0_0_50px_rgba(245,158,11,0.15)] rounded-2xl w-full max-w-sm overflow-hidden flex flex-col relative">
              <div className="absolute top-0 left-0 right-0 h-1 bg-amber-500/50" />
              <div className="p-6 pb-4 flex flex-col items-center text-center mt-2">
                <div className="w-12 h-12 rounded-full bg-amber-500/10 flex flex-items-center justify-center mb-4 text-amber-500 shadow-inner border border-amber-500/20">
                  <ShieldCheck className="w-6 h-6 mt-3" />
                </div>
                <h3 className="text-xl font-bold text-white mb-2">Session Expiring Soon</h3>
                <p className="text-sm text-slate-400">
                  For your security, your session will automatically expire in <span className="font-bold text-amber-400">{sessionTimeRemaining}</span>. Would you like to extend it?
                </p>
              </div>
              <div className="grid grid-cols-2 p-4 gap-3 bg-[#060a16] border-t border-white/5">
                <button
                  onClick={() => setShowSessionWarning(false)}
                  className="py-2.5 px-4 rounded-xl text-sm font-medium text-slate-300 hover:text-white bg-white/5 hover:bg-white/10 transition-colors border border-transparent hover:border-white/10"
                >
                  Dismiss
                </button>
                <button
                  onClick={() => {
                    handleLogin(localStorage.getItem('atc_session_duration') || 'heute');
                    setShowSessionWarning(false);
                  }}
                  className="py-2.5 px-4 rounded-xl text-sm font-bold text-[#060a16] bg-amber-500 hover:bg-amber-400 transition-colors shadow-lg shadow-amber-500/20"
                >
                  Extend Session
                </button>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      <AnimatePresence>
        {shareDialogId && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 z-[10000] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
          >
            <motion.div
              initial={{ scale: 0.95, y: 20 }}
              animate={{ scale: 1, y: 0 }}
              exit={{ scale: 0.95, y: 20 }}
              className="w-full max-w-sm bg-[#0a0d16] border border-white/10 rounded-2xl p-6 shadow-2xl relative"
            >
              <button 
                onClick={() => setShareDialogId(null)}
                className="absolute top-4 right-4 p-1.5 text-slate-400 hover:text-white rounded-lg hover:bg-white/10 transition-colors"
               >
                 <X className="w-5 h-5" />
              </button>
              <div className="w-12 h-12 bg-indigo-500/20 rounded-xl flex items-center justify-center mb-4 border border-indigo-500/30">
                <Share2 className="w-6 h-6 text-indigo-400" />
              </div>
              <h3 className="text-xl font-bold text-white mb-2">Share Application State</h3>
              <p className="text-sm text-slate-400 mb-6">
                Generate a temporary, distinct link to current view within {WINDOWS_MAP[shareDialogId]?.label || 'this applet'}.
              </p>
              <div className="bg-black/50 p-3 rounded-xl border border-white/5 font-mono text-xs text-indigo-300 break-all mb-6 select-all">
                https://atc-network.local/shared/{shareDialogId}/{Math.random().toString(36).substring(7)}
              </div>
              <div className="flex flex-col gap-3">
                <button
                  onClick={() => {
                    if (navigator.vibrate) navigator.vibrate(20);
                    const link = `https://atc-network.local/shared/${shareDialogId}/${Math.random().toString(36).substring(7)}`;
                    window.dispatchEvent(new CustomEvent('ATC_OPEN_SOCIAL_POST', { detail: `Check out this applet view!\n${link}` }));
                    openWindow('social_app');
                    setShareDialogId(null);
                  }}
                  className="flex flex-1 items-center justify-center gap-2 py-2.5 rounded-xl text-sm font-bold text-white bg-white/10 hover:bg-white/20 border border-white/10 transition-colors"
                >
                  <Users className="w-4 h-4" /> Share to ATC Social
                </button>
                <button
                  onClick={() => {
                    if (navigator.vibrate) navigator.vibrate(20);
                    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Share link copied to clipboard.' }));
                    setShareDialogId(null);
                  }}
                  className="flex-1 py-2.5 rounded-xl text-sm font-bold text-[#060a16] bg-indigo-400 hover:bg-white transition-colors"
                >
                  Copy Link
                </button>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Open Windows Taskbar (Level 2) */}
      {windows.length > 0 && (
        <div 
          className={
            osMode === 'macos'
              ? 'absolute z-[9989] bottom-[84px] left-1/2 -translate-x-1/2 h-12 bg-white/10 backdrop-blur-3xl border border-white/20 rounded-2xl px-4 flex items-center gap-2 shadow-[0_20px_40px_rgba(0,0,0,0.5)] select-none'
              : osMode === 'linux'
              ? 'absolute z-[9989] top-8 left-0 right-0 h-10 bg-[#050811]/90 backdrop-blur-3xl border-b border-indigo-500/20 px-4 flex items-center gap-2 select-none'
              : osMode === 'windows'
              ? 'absolute z-[9989] bottom-12 left-0 right-0 h-10 bg-black/90 backdrop-blur-3xl border-t border-white/10 px-4 flex items-center gap-2 select-none shadow-[0_-5px_20px_rgba(0,0,0,0.6)]'
              : 'absolute z-[9989] bottom-14 left-0 right-0 h-10 bg-[#050811]/90 backdrop-blur-3xl border-t border-indigo-500/20 px-4 flex items-center gap-2 select-none shadow-[0_-10px_40px_rgba(0,0,0,0.5)]'
          }
        >
          <div className="flex items-center gap-2 overflow-x-auto w-full no-scrollbar">
            {windows.map((win) => {
              const config = WINDOWS_MAP[win.id];
              return (
                <button
                  key={`taskbar-${win.id}`}
                  onClick={() => {
                    if (navigator.vibrate) navigator.vibrate(20);
                    win.isMinimized ? restoreWindow(win.id) : focusWindow(win.id);
                  }}
                  className={`relative group flex items-center gap-2 px-4 py-2 sm:px-3 sm:py-0 sm:h-8 rounded-lg transition-all flex-shrink-0 min-h-[44px] sm:min-h-0 ${
                    focusedId === win.id && !win.isMinimized
                      ? "bg-indigo-500/20 text-indigo-300 border border-indigo-500/30"
                      : "hover:bg-white/5 border border-transparent text-slate-400"
                  }`}
                >
                  {config && (
                    <config.icon
                      className={`w-3.5 h-3.5 ${focusedId === win.id && !win.isMinimized ? "opacity-100" : "opacity-70"}`}
                    />
                  )}
                  <span className="text-xs font-medium max-w-[120px] truncate">
                    {config ? config.label : win.id}
                  </span>
                  <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-3 px-3 py-1.5 bg-[#090b14]/90 backdrop-blur-md border border-white/10 text-white text-xs font-medium whitespace-nowrap rounded-lg shadow-xl opacity-0 invisible group-hover:opacity-100 group-hover:visible pointer-events-none transition-all z-[10000]">
                    {config ? config.label : win.id}
                  </div>
                </button>
              );
            })}
          </div>
        </div>
      )}

      {/* Main Taskbar (Level 1) */}
      <div 
        className={
          osMode === 'macos' 
            ? 'absolute z-[9990] bottom-4 left-1/2 -translate-x-1/2 h-14 bg-white/10 backdrop-blur-3xl border border-white/20 rounded-2xl px-4 flex items-center gap-3 shadow-[0_20px_40px_rgba(0,0,0,0.5)] select-none'
            : osMode === 'windows'
            ? 'absolute z-[9990] bottom-0 left-0 right-0 h-12 bg-black/95 border-t border-white/10 px-4 flex items-center justify-between select-none shadow-[0_-5px_20px_rgba(0,0,0,0.6)]'
            : osMode === 'linux'
            ? 'absolute z-[9990] top-0 left-0 right-0 h-8 bg-gradient-to-r from-gray-900 via-black to-gray-900 border-b border-white/10 px-4 flex items-center justify-between select-none shadow-lg'
            : 'absolute bottom-0 left-0 right-0 h-14 bg-[#050811]/90 backdrop-blur-3xl border-t border-indigo-500/20 z-[9990] flex items-center px-4 justify-between select-none shadow-[0_-10px_40px_rgba(0,0,0,0.5)]'
        }
      >
        {(osMode === 'default' || osMode === 'windows' || osMode === 'linux') && (
          <div className="absolute top-0 left-0 right-0 h-[1px] bg-gradient-to-r from-transparent via-indigo-500/50 to-transparent" />
        )}
        
        <div className={`flex items-center gap-3 relative z-10 ${osMode === 'windows' ? '' : ''}`}>
          <button
            onClick={() => {
              setIsStartOpen(!isStartOpen);
              if (isStartOpen) setActiveCategory(null);
            }}
            className={`w-10 h-10 rounded-xl flex items-center justify-center transition-all ${isStartOpen ? "bg-indigo-500/20 text-indigo-300 border border-indigo-500/50 shadow-[0_0_15px_rgba(99,102,241,0.4)]" : "hover:bg-white/10 text-slate-300 border border-transparent"}`}
          >
            <Component className="w-5 h-5" />
          </button>
        </div>

        <div className="flex items-center gap-4 text-slate-300 relative z-10">
          <div className="hidden sm:flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-indigo-500/10 border border-indigo-500/30">
            <Bot className="w-3.5 h-3.5 text-indigo-400" />
            <span className="text-[10px] font-bold text-indigo-400 tracking-widest uppercase truncate max-w-[120px]" title="Active Sandbox Kernel">
              {globalOsEnv.includes('win') ? `Windows [${globalOsEnv}]` : globalOsEnv.includes('ubuntu') || globalOsEnv.includes('debian') || globalOsEnv.includes('alpine') ? `Linux [${globalOsEnv}]` : 'ATC-Native'}
            </span>
          </div>
          <div className="hidden sm:flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-emerald-500/10 border border-emerald-500/30">
            <Shield className="w-3.5 h-3.5 text-emerald-400" />
            <span className="text-[10px] font-bold text-emerald-400 tracking-widest uppercase">ATC Secure</span>
          </div>
          
          <GpuPerformanceWidget />
          
          <div id="system-tray" className="flex items-center gap-3 text-slate-400 [&_.lucide]:transition-all [&_.lucide:hover]:scale-110 [&_.lucide:hover]:animate-pulse">
            <WorkspaceSyncStatusBar />
            <SystemHealthDashboardWidget />
            <ChainPulseWidget />
            <OfflineQueueWidget />
            <NodePulseWidget onClick={() => openWindow('explorer')} />
            <button onClick={tileWindows} title="Tile Windows" className="p-1 hover:bg-white/10 rounded group">
              <LayoutGrid className="w-4 h-4 cursor-pointer text-slate-400 group-hover:text-white" />
            </button>
            <div className={isHighLoad ? "animate-bounce" : ""}>
              <BatteryStatus />
            </div>
            <span title="Notifications"><Bell className="w-4 h-4 cursor-pointer hover:text-white" /></span>
            <span title="Network Status" className={isHighLoad ? "animate-bounce" : ""}><Wifi className="w-4 h-4 cursor-pointer hover:text-white" /></span>
            <span title="Verifiably Secure"><ShieldCheck className="w-4 h-4 cursor-pointer text-emerald-400 ml-1" /></span>
            <span title="Diagnose Tools"><Activity className="w-4 h-4 text-emerald-400 cursor-pointer" onClick={() => openWindow("diagnostics")} /></span>
          </div>
          <div className="w-[1px] h-6 bg-white/10" />
          <div className="flex flex-col items-end justify-center px-2">
            <span className="text-xs font-semibold">
              {time.toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
              })}
            </span>
            <span className="text-[10px] text-slate-400">
              {time.toLocaleDateString()}
            </span>
          </div>
          {sessionTimeRemaining && (
            <>
              <div className="w-[1px] h-6 bg-white/10" />
              <div className="flex flex-col items-start justify-center px-2 group relative">
                <span className="text-[10px] font-mono text-slate-400 uppercase tracking-widest leading-none mb-1">Session Exp.</span>
                <div className="flex items-center gap-2">
                  <span className={`text-xs font-mono font-medium ${sessionTimeColor}`}>{sessionTimeRemaining}</span>
                  <div className="w-12 h-1.5 bg-white/10 rounded-full overflow-hidden">
                    <div className={`h-full ${sessionProgressColor} transition-all duration-1000`} style={{ width: `${Math.max(0, Math.min(100, sessionPercent))}%` }}></div>
                  </div>
                </div>
                
                {/* One Click Extend Button (shows on hover) */}
                <div className="absolute top-full right-0 mt-2 opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-50">
                   <button 
                     onClick={() => {
                       const currentDur = localStorage.getItem('atc_session_duration') || '1hour';
                       handleLogin(currentDur);
                     }}
                     className="px-3 py-1.5 bg-[#090b14] border border-emerald-500/30 rounded shadow-xl text-emerald-400 text-xs font-mono hover:bg-emerald-500/10 whitespace-nowrap"
                   >
                     + Extend Session
                   </button>
                </div>
              </div>
            </>
          )}
        </div>
      </div>

      {/* Context Menu */}
      {contextMenu && (
        <div 
          className="absolute z-[9999] atc-menu-wrapper py-2 w-64 sm:w-56 animate-in fade-in zoom-in-95 duration-200"
          style={{ left: contextMenu.x, top: contextMenu.y }}
        >
          <button 
            className="atc-menu-item group"
            onClick={(e) => {
              if (navigator.vibrate) navigator.vibrate(15);
              e.stopPropagation();
              window.location.reload();
              closeContextMenu();
            }}
          >
            <div className="p-1.5 bg-indigo-500/10 rounded-lg group-hover:bg-indigo-500/20 transition-colors">
              <RefreshCw className="w-4 h-4 text-indigo-400 group-hover:rotate-180 transition-transform duration-500" />
            </div>
            <span className="font-medium">Refresh</span>
          </button>
          
          <button 
            className="atc-menu-item group"
            onClick={(e) => {
              if (navigator.vibrate) navigator.vibrate(15);
              e.stopPropagation();
              openWindow("terminal");
              closeContextMenu();
            }}
          >
            <div className="p-1.5 bg-emerald-500/10 rounded-lg group-hover:bg-emerald-500/20 transition-colors">
               <SquareTerminal className="w-4 h-4 text-emerald-400 group-hover:scale-110 transition-transform" />
            </div>
            <span className="font-medium">New Terminal</span>
          </button>
          
          <div className="atc-menu-divider" />
          
          <button 
            className="atc-menu-item group"
            onClick={(e) => {
              if (navigator.vibrate) navigator.vibrate(15);
              e.stopPropagation();
              setShowWallpaperPicker(true);
              closeContextMenu();
            }}
          >
            <div className="p-1.5 bg-blue-500/10 rounded-lg group-hover:bg-blue-500/20 transition-colors">
              <Image className="w-4 h-4 text-blue-400 group-hover:scale-110 transition-transform" />
            </div>
            <span className="font-medium">Change Wallpaper</span>
          </button>

          <button 
            className="atc-menu-item group"
            onClick={(e) => {
              if (navigator.vibrate) navigator.vibrate(15);
              e.stopPropagation();
              openWindow("settings");
              closeContextMenu();
            }}
          >
            <div className="p-1.5 bg-slate-500/10 rounded-lg group-hover:bg-slate-500/20 transition-colors">
              <Settings className="w-4 h-4 text-slate-400 group-hover:rotate-90 transition-transform duration-500" />
            </div>
            <span className="font-medium">Settings</span>
          </button>
        </div>
      )}

      {/* Wallpaper Picker */}
      <AnimatePresence>
        {showWallpaperPicker && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 z-[10000] bg-black/60 backdrop-blur-sm flex items-center justify-center"
            onClick={() => setShowWallpaperPicker(false)}
          >
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="w-full max-w-2xl bg-[#090b14] border border-white/10 rounded-xl shadow-2xl overflow-hidden p-6"
              onClick={e => e.stopPropagation()}
            >
              <div className="flex items-center justify-between mb-5">
                <h3 className="text-lg font-semibold text-white flex items-center gap-2">
                  <Image className="w-5 h-5 text-blue-400" />
                  Change Wallpaper
                </h3>
                <button
                  onClick={() => setShowWallpaperPicker(false)}
                  className="p-1.5 rounded-lg hover:bg-white/10 transition-colors"
                >
                  <X className="w-4 h-4 text-slate-400" />
                </button>
              </div>
              <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
                {WALLPAPERS.map((wp) => (
                  <button
                    key={wp.id}
                    onClick={() => {
                      setWallpaperUrl(wp.url);
                      if (navigator.vibrate) navigator.vibrate(15);
                      setShowWallpaperPicker(false);
                    }}
                    className={`group relative aspect-[9/16] rounded-lg overflow-hidden border-2 transition-all ${
                      wallpaperUrl === wp.url ? 'border-blue-400 ring-2 ring-blue-400/40' : 'border-white/10 hover:border-white/30'
                    }`}
                  >
                    <img src={wp.url} alt={wp.name} className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" />
                    <div className="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/80 to-transparent p-2">
                      <span className="text-xs font-medium text-white">{wp.name}</span>
                    </div>
                    {wallpaperUrl === wp.url && (
                      <div className="absolute top-2 right-2 w-5 h-5 rounded-full bg-blue-500 flex items-center justify-center">
                        <CheckSquare className="w-3 h-3 text-white" />
                      </div>
                    )}
                  </button>
                ))}
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Command Palette */}
      <AnimatePresence>
        {showCommandPalette && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 z-[10000] bg-black/60 backdrop-blur-sm flex items-start justify-center pt-[20vh]"
            onClick={() => setShowCommandPalette(false)}
          >
            <motion.div
              initial={{ scale: 0.95, y: -20 }}
              animate={{ scale: 1, y: 0 }}
              exit={{ scale: 0.95, y: -20 }}
              className="w-full max-w-xl bg-[#090b14] border border-white/10 rounded-xl shadow-2xl overflow-hidden"
              onClick={e => e.stopPropagation()}
            >
              <div className="flex items-center px-4 py-3 border-b border-white/10">
                <Search className="w-5 h-5 text-slate-400 mr-3" />
                <input
                  type="text"
                  autoFocus
                  placeholder="Search apps, commands, or documentation... (Ctrl+K)"
                  value={searchQuery}
                  onChange={e => setSearchQuery(e.target.value)}
                  className="flex-1 bg-transparent border-none outline-none text-slate-200 placeholder:text-slate-600"
                />
                <div className="text-[10px] font-mono font-bold text-slate-500 bg-white/5 px-2 py-0.5 rounded">ESC</div>
              </div>
              <div className="max-h-64 overflow-y-auto custom-scrollbar p-2">
                {Object.entries(WINDOWS_MAP)
                  .map(([id, config]) => ({ id, type: 'app', label: config.label, icon: config.icon, category: config.category }))
                  .concat(
                    (function flattenStructure(nodes: typeof INITIAL_STRUCTURE): any[] {
                      return nodes.flatMap(n => [{ id: n.id, type: 'file', label: n.name, icon: FileCode, category: 'File Structure' }, ...(n.children ? flattenStructure(n.children) : [])]);
                    })(INITIAL_STRUCTURE)
                  )
                  .concat(
                    TECH_TREE_DATA.flatMap(tier => tier.nodes.map(node => ({ id: node.id, type: 'tech', label: node.title, icon: node.icon, category: 'Tech Tree' })))
                  )
                  .filter(item => 
                    item.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
                    item.category.toLowerCase().includes(searchQuery.toLowerCase())
                  )
                  .map((item) => {
                    const Icon = item.icon as any;
                    return (
                    <button
                      key={`cmd-${item.type}-${item.id}`}
                      onClick={() => {
                        if (item.type === 'app') {
                          openWindow(item.id);
                        } else if (item.type === 'file') {
                          window.dispatchEvent(new CustomEvent('ATC_DEEP_LINK_STRUCTURE', { detail: { id: item.id } }));
                          openWindow('structure');
                        } else if (item.type === 'tech') {
                          window.dispatchEvent(new CustomEvent('ATC_DEEP_LINK_TECH', { detail: { id: item.id } }));
                          openWindow('tech_tree');
                        }
                        setShowCommandPalette(false);
                        setSearchQuery('');
                      }}
                      className="w-full flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-indigo-500/10 text-left transition-colors group"
                    >
                      <div className="p-1.5 rounded bg-white/5 text-slate-400 group-hover:text-indigo-400 group-hover:bg-indigo-500/20 flex-shrink-0">
                        <Icon className="w-4 h-4" />
                      </div>
                      <div className="flex flex-col flex-1 overflow-hidden">
                        <span className="text-sm font-medium text-slate-200 group-hover:text-white truncate">{item.label}</span>
                        <span className="text-[10px] text-slate-500 font-mono tracking-wider uppercase truncate">{item.category}</span>
                      </div>
                    </button>
                    );
                  })}
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
      <SystemTrayToast />
    </div>
  );
}
