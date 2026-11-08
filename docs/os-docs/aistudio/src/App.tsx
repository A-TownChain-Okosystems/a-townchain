import React, { useState, useEffect, useRef } from "react";
import * as d3 from "d3";
import jsPDF from "jspdf";
import { motion, AnimatePresence } from "motion/react";
import {
  BookOpen,
  Code2,
  Layers,
  Map,
  Calendar,
  FileCode2,
  ShieldCheck,
  Wrench,
  Activity,
  Terminal,
  Settings2,
  PenTool,
  Coins,
  Cpu,
  Banknote,
  Scale,
  Brain,
  FlaskConical,
  TrendingUp,
  Gavel,
  Users,
  Server,
  Shield,
  Globe,
  CheckCircle,
  ArrowRight,
  Database,
  Component,
  Microscope,
  Network,
  Link,
  CheckSquare,
  Search,
  AlertTriangle,
  FileText,
  ClipboardList,
  List,
  ChevronRight,
  ChevronLeft,
  Bookmark,
  RefreshCw,
  Github,
  Mic,
  Bell,
  Wifi,
  LayoutGrid,
  Settings,
  User,
  Plus,
  Download,
  Share2,
  Clock,
  X,
  CornerDownRight,
  Folder,
  MessageSquare,
  BrainCircuit,
  Sparkles,
  History,
  Minus,
  Maximize2,
  Minimize2,
  Boxes,
  Menu,
  ArrowDownToLine,
  Monitor,
  ArrowUpDown,
} from "lucide-react";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  Cell,
  CartesianGrid,
} from "recharts";
import { LAYERS, LONG_TERM_VISION, TARGET_STATE } from "./data";
import { ROADMAP_DATA as INITIAL_ROADMAP_DATA } from "./roadmapData";
import { ECOSYSTEM_DATA } from "./ecosystemData";
import {
  FINAL_ARCHITECTURE_LEVELS,
  REFERENCE_ARCHITECTURE,
  META_SYSTEMS,
  END_STATE_METRICS,
  TIERS,
  ULTIMATE_SECURITY_STACK,
  ULTIMATE_VERIFICATION_STACK,
} from "./tierData";
import { ATC_STANDARDS, ATS_STANDARDS } from "./standardsData";
import {
  ARCHITECTURE_REVIEW,
  REPOSITORY_REVIEW,
  ENTERPRISE_GAP_ANALYSIS,
  DOCUMENTATION_SCORE,
  ADDITIONAL_ANALYSIS,
} from "./auditData";
import { ATC_OS_REQUIREMENTS } from "./requirementsData";
import { WIKI_CONTENT } from "./wikiData";
import { ATC_LANG_WIKI_CONTENT } from "./atcLangWikiData";
import { ATC_LANG_ROADMAP_DATA } from "./atcLangRoadmapData";
import type { IconName } from "./types";
import confetti from "canvas-confetti";
import { TodoView } from "./components/TodoView";
import { TechDocsView } from "./components/TechDocsView";
import { MetricsView } from "./components/MetricsView";
import { ModulesPluginView } from "./components/ModulesPluginView";
import { StructureView } from "./components/StructureView";
import { TechTreeView } from "./components/TechTreeView";
import { SocialMediaView } from "./components/SocialMediaView";
import { NetworkExplorerView } from "./components/NetworkExplorerView";
import { NetworkTopologyView } from "./components/NetworkTopologyView";
import { SettingsView } from "./components/SettingsView";
const GovernanceView = React.lazy(() => import("./components/GovernanceView").then(m => ({ default: m.GovernanceView })));
import { ATownTestView } from "./components/ATownTestView";
import { AiOsEngineView } from "./components/AiOsEngineView";
import AtvmSandboxView from "./components/AtvmSandboxView";
import { ComplianceView } from "./components/ComplianceView";
import { ThemeSwitcher } from "./components/ThemeSwitcher";
import { TooltipIcon } from "./components/TooltipIcon";
import { ApiHealthWidget } from "./components/ApiHealthWidget";
import { SoftwareAuditView } from "./components/SoftwareAuditView";
import { LegalView } from "./components/LegalView";
import { AtcLangPresetsView } from "./components/AtcLangPresetsView";
import { AtcLangArchitectureView } from "./components/AtcLangArchitectureView";
import { AtcCoreKernelView } from "./components/AtcCoreKernelView";
import { AtcLangPlaygroundView } from "./components/AtcLangPlaygroundView";
import { AtcAssetsDbView } from "./components/AtcAssetsDbView";
import { FranchiseFactoryView } from "./components/FranchiseFactoryView";
import { SemanticGraphView } from "./components/SemanticGraphView";
import { DbOrchestratorView } from "./components/DbOrchestratorView";
import { TxOrchestratorView } from "./components/TxOrchestratorView";
import { GitOpsView } from "./components/GitOpsView";
import { CodeAnalyzerView } from "./components/CodeAnalyzerView";
import { PaymentSystemView } from "./components/PaymentSystemView";
import { DataProcessingView } from "./components/DataProcessingView";
import { BatteryStatus } from "./components/BatteryStatus";
import { Window } from "./components/Window";
import { InterfacesView } from "./components/InterfacesView";
import { ReportsView } from "./components/ReportsView";
import { GitHubRepoSyncView } from "./components/GitHubRepoSyncView";
import { useWallet } from "./contexts/WalletContext";
import { Wallet } from "lucide-react";

const BEREICH_ICON_MAP: Record<string, React.ElementType> = {
  Allgemein: BookOpen,
  Sprache: Code2,
  "Architektur & System": Layers,
  Strategie: Map,
  Planung: Calendar,
  "Code & Specs": FileCode2,
  Security: ShieldCheck,
  Engineering: Wrench,
  "R&D": FlaskConical,
  "Monitoring & Ops": Activity,
  System: Terminal,
  "System Administration": Settings2,
  Tools: PenTool,
  "Blockchain & Token": Coins,
  "System Core": Cpu,
  Finance: Banknote,
};

const ICON_MAP: Record<string, React.ElementType> = {
  Scale,
  Brain,
  FlaskConical,
  TrendingUp,
  Gavel,
  Users,
  Server,
  Shield,
  Globe,
  CheckCircle,
};

export default function App() {
  const { address, balance, isConnecting, connectWallet } = useWallet();
  const [activeLayer, setActiveLayer] = useState<number | null>(null);
  const [sessionTime, setSessionTime] = useState(0);
  const [isFocusMode, setIsFocusMode] = useState(false);
  const [language, setLanguage] = useState<"DE" | "EN">("DE");
  const [bookmarks, setBookmarks] = useState<
    { id: string; title: string; type: string; path: string }[]
  >(() => {
    try {
      const stored = localStorage.getItem("atc_bookmarks");
      return stored ? JSON.parse(stored) : [];
    } catch {
      return [];
    }
  });

  useEffect(() => {
    const t = setInterval(() => setSessionTime((s) => s + 1), 1000);
    return () => clearInterval(t);
  }, []);

  useEffect(() => {
    localStorage.setItem("atc_bookmarks", JSON.stringify(bookmarks));
  }, [bookmarks]);

  const addBookmark = (title: string, type: string, path: string) => {
    const id = title.toLowerCase().replace(/[^a-z0-9]+/g, "-") + "-" + path;
    setBookmarks((prev) => {
      if (prev.find((b) => b.id === id)) return prev;
      return [...prev, { id, title, type, path }];
    });
  };

  const removeBookmark = (id: string) => {
    setBookmarks((prev) => prev.filter((b) => b.id !== id));
  };

  const formatTimer = (seconds: number) => {
    const m = Math.floor(seconds / 60)
      .toString()
      .padStart(2, "0");
    const s = (seconds % 60).toString().padStart(2, "0");
    const h = Math.floor(seconds / 3600)
      .toString()
      .padStart(2, "0");
    return seconds >= 3600 ? `${h}:${m}:${s}` : `${m}:${s}`;
  };

  const [isSidebarOpen, setIsSidebarOpen] = useState(false);

  const [activeTab, setActiveTab] = useState<string>("atvm_sandbox");

  const [conflictItems, setConflictItems] = useState<
    { id: string; title: string; githubSource: any; notionSource: any }[]
  >([]);
  const [showConflictModal, setShowConflictModal] = useState(false);
  const [toastMessage, setToastMessage] = useState<{
    message: string;
    type: "error" | "success";
    link?: { label: string; tab: string; settingsTab: string };
  } | null>(null);

  const [showGlobalBuildErrorModal, setShowGlobalBuildErrorModal] =
    useState(false);
  const [copiedLogs, setCopiedLogs] = useState(false);
  const mockBuildLogs = `> react-example@0.0.0 build
> vite build && esbuild server.ts --bundle --platform=node --format=cjs --packages=external --sourcemap --outfile=dist/server.cjs

vite v6.4.3 building for production...
transforming...
✓ 1017 modules transformed.
✓ Build succeeded in 2.01s
`;

  useEffect(() => {
    if (conflictItems.length > 0) {
      setActiveTab("settings");
      setShowConflictModal(false); // Disable old modal, we are using settings now
    }
  }, [conflictItems.length]);

  const [isSyncPaused, setIsSyncPaused] = useState(false);
  const [offlineQueue, setOfflineQueue] = useState<any[]>(() => {
    try {
      const stored = localStorage.getItem("atc_offline_sync_queue");
      return stored ? JSON.parse(stored) : [];
    } catch {
      return [];
    }
  });

  const [isOnline, setIsOnline] = useState(navigator.onLine);

  useEffect(() => {
    localStorage.setItem(
      "atc_offline_sync_queue",
      JSON.stringify(offlineQueue),
    );
  }, [offlineQueue]);

  useEffect(() => {
    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);
    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);
    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);
  const handleFlushQueue = async () => {
    if (!isOnline || offlineQueue.length === 0) return;
    for (const item of offlineQueue) {
      if (item.target === "GitHub")
        window.dispatchEvent(new CustomEvent("trigger-github-sync"));
      else if (item.target === "Notion")
        window.dispatchEvent(new CustomEvent("trigger-notion-sync"));
    }
    setOfflineQueue([]);
  };

  useEffect(() => {
    const handleAddOffline = (e: any) =>
      setOfflineQueue((prev) => [...prev, e.detail]);
    window.addEventListener("add-offline-queue", handleAddOffline);
    return () =>
      window.removeEventListener("add-offline-queue", handleAddOffline);
  }, []);

  useEffect(() => {
    const handleApiError = (e: any) => {
      setToastMessage({
        message: `${e.detail.source || "API"} Error: ${e.detail.message || "Check Diagnostics"}`,
        type: "error",
        link: {
          label: "View Diagnostics",
          tab: "settings",
          settingsTab: "sync-diagnostics",
        },
      });
      setTimeout(() => setToastMessage(null), 5000);
    };
    window.addEventListener("api-error", handleApiError);
    return () => window.removeEventListener("api-error", handleApiError);
  }, []);

  const [networkStatus, setNetworkStatus] = useState<
    "devnet" | "testnet" | "mainnet"
  >("devnet");
  const [isSyncing, setIsSyncing] = useState(false);
  const [lastSync, setLastSync] = useState<Date>(new Date());
  const [syncProgress, setSyncProgress] = useState(0);

  const [isNotionSyncing, setIsNotionSyncing] = useState(false);
  const [notionSyncStatus, setNotionSyncStatus] = useState<string | null>(null);

  const [githubSyncStep, setGithubSyncStep] = useState<string | null>(null);
  const [notionSyncStep, setNotionSyncStep] = useState<string | null>(null);

  const [syncHistory, setSyncHistory] = useState<
    {
      timestamp: Date;
      type: "Manual" | "Auto";
      target: string;
      outcome: string;
      success: boolean;
      id: string;
      content: string;
    }[]
  >([
    {
      timestamp: new Date(Date.now() - 1000 * 60 * 5),
      type: "Auto",
      target: "GitHub",
      outcome: "Synced 15 commits from main branch",
      success: true,
      id: "sync-01",
      content: "Auto-sync GitHub",
    },
    {
      timestamp: new Date(Date.now() - 1000 * 60 * 30),
      type: "Manual",
      target: "Notion",
      outcome: "Updated 3 documents, 1 conflict resolved",
      success: true,
      id: "sync-02",
      content: "Manual sync Notion",
    },
    {
      timestamp: new Date(Date.now() - 1000 * 60 * 60 * 2),
      type: "Auto",
      target: "GitHub",
      outcome: "Connection timeout while fetching repository tree",
      success: false,
      id: "sync-03",
      content: "Auto-sync GitHub",
    },
    {
      timestamp: new Date(Date.now() - 1000 * 60 * 60 * 24),
      type: "Manual",
      target: "GitHub",
      outcome: "Full repository clone and index completed",
      success: true,
      id: "sync-04",
      content: "Manual sync GitHub",
    },
  ]);

  const hasSyncErrors =
    conflictItems.length > 0 ||
    (syncHistory.length > 0 && !syncHistory[0].success);
  const isCurrentlySyncing = isSyncing || isNotionSyncing;
  const hasPendingItems = offlineQueue.length > 0;
  const syncBadgeColor = hasSyncErrors
    ? "bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.6)]"
    : isCurrentlySyncing
      ? "bg-amber-400 animate-pulse"
      : hasPendingItems
        ? "bg-amber-500"
        : "bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.3)]";

  // Global Shortcuts & Cheat Sheet
  const [showCheatSheet, setShowCheatSheet] = useState(false);
  const searchInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+K to search
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        searchInputRef.current?.focus();
      }
      // Ctrl+Alt+S or Cmd+Opt+S to sync GitHub and Notion
      if ((e.ctrlKey || e.metaKey) && e.altKey && (e.key.toLowerCase() === "s" || e.code === "KeyS")) {
        e.preventDefault();
        setToastMessage({
          message: "Triggering comprehensive system sync...",
          type: "success",
        });
        window.dispatchEvent(new CustomEvent("trigger-github-sync"));
        window.dispatchEvent(new CustomEvent("trigger-notion-sync"));
        return; // Prevent triggering the standard Save
      }
      
      // Ctrl+S to save docs
      if ((e.ctrlKey || e.metaKey) && !e.altKey && e.key.toLowerCase() === "s") {
        e.preventDefault();
        setToastMessage({
          message: "Documents saved securely to A-TownChain.",
          type: "success",
        });
      }
      // Esc to close modals
      if (e.key === "Escape") {
        setShowCheatSheet(false);
        // also blur search if focused
        if (document.activeElement === searchInputRef.current) {
          searchInputRef.current?.blur();
        }
      }
      // Ctrl+/ for cheat sheet
      if ((e.ctrlKey || e.metaKey) && e.key === "/") {
        e.preventDefault();
        setShowCheatSheet((prev) => !prev);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const [theme, setTheme] = useState<
    "default" | "oceanic-deep" | "sunset-red" | "cyber-cyan"
  >(() => {
    return (localStorage.getItem("atc_theme") as any) || "default";
  });

  useEffect(() => {
    if (theme === "default") {
      document.documentElement.removeAttribute("data-theme");
    } else {
      document.documentElement.setAttribute("data-theme", theme);
    }
    localStorage.setItem("atc_theme", theme);
    window.dispatchEvent(new Event("theme_changed"));
  }, [theme]);

  useEffect(() => {
    const handleSync = () => {
      const stored = localStorage.getItem("atc_theme") || "default";
      setTheme(stored as any);
    };
    window.addEventListener("storage", handleSync);
    window.addEventListener("theme_changed_external", handleSync);
    return () => {
      window.removeEventListener("storage", handleSync);
      window.removeEventListener("theme_changed_external", handleSync);
    };
  }, []);

  useEffect(() => {
    const handleSyncStep = (e: any) => {
      if (e.detail.target === "github") setGithubSyncStep(e.detail.step);
      if (e.detail.target === "notion") setNotionSyncStep(e.detail.step);
    };
    window.addEventListener("sync-step", handleSyncStep);
    return () => window.removeEventListener("sync-step", handleSyncStep);
  }, []);

  // Search logic and Voice Recognition
  const handleVoiceSearch = () => {
    const SpeechRecognition =
      (window as any).SpeechRecognition ||
      (window as any).webkitSpeechRecognition;
    if (!SpeechRecognition) {
      setToastMessage({
        message: "Speech Recognition is not supported in this browser.",
        type: "error",
      });
      return;
    }
    const recognition = new SpeechRecognition();
    recognition.lang = "en-US";
    recognition.start();

    setToastMessage({
      message: "Listening for voice search...",
      type: "success",
    });

    recognition.onresult = (event: any) => {
      const transcript = event.results[0][0].transcript;
      setSearchQuery(transcript);
      setToastMessage({
        message: `Voice search: "${transcript}"`,
        type: "success",
      });
      setRecentSearches((prev) => {
        const unique = prev.filter((q: string) => q !== transcript);
        return [transcript, ...unique].slice(0, 5);
      });
      setTimeout(() => setSearchQuery(""), 4000);
    };
    recognition.onerror = (event: any) => {
      setToastMessage({
        message: `Voice recognition error: ${event.error}`,
        type: "error",
      });
    };
  };

  const [searchQuery, setSearchQuery] = useState("");
  const [searchError, setSearchError] = useState("");
  const [isSearchFocused, setIsSearchFocused] = useState(false);
  const [recentSearches, setRecentSearches] = useState<string[]>(() => {
    try {
      const stored = localStorage.getItem("recentSearches");
      return stored ? JSON.parse(stored) : [];
    } catch {
      return [];
    }
  });

  useEffect(() => {
    localStorage.setItem("recentSearches", JSON.stringify(recentSearches));
  }, [recentSearches]);

  const handleSearchSubmit = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      const query = searchQuery.trim();

      if (!query) {
        setSearchError("Search cannot be empty.");
        setTimeout(() => setSearchError(""), 3000);
        return;
      }

      setSearchError("");
      setSearchQuery(query);

      setRecentSearches((prev) => {
        const unique = prev.filter((q) => q !== query);
        return [query, ...unique].slice(0, 10); // Keep last 10
      });
      // Optional: actually perform search logic here or navigate
      setSearchQuery("");
      e.currentTarget.blur();
    }
  };

  const fetchNotionProgress = () => {
    setIsNotionSyncing(true);
    setNotionSyncStatus("Connecting...");
    window.dispatchEvent(new Event("trigger-notion-sync"));
    setTimeout(() => {
      setIsNotionSyncing(false);
      setNotionSyncStatus("Synced via auto");
      setTimeout(() => setNotionSyncStatus(null), 3000);
    }, 1500);
  };

  const handleSync = async () => {
    setIsSyncing(true);
    setSyncProgress(25);
    window.dispatchEvent(new Event("trigger-github-sync"));

    setTimeout(() => {
      setSyncProgress(65);
    }, 400);

    setTimeout(() => {
      window.dispatchEvent(new Event("trigger-notion-sync"));
      setSyncProgress(100);
    }, 800);

    setTimeout(() => {
      setIsSyncing(false);
      setLastSync(new Date());
      setSyncProgress(0);
    }, 1500);
  };

  useEffect(() => {
    const triggerSync = () => handleSync();
    window.addEventListener("atc-trigger-sync-shortcut", triggerSync);
    return () => window.removeEventListener("atc-trigger-sync-shortcut", triggerSync);
  }, [handleSync]);

  const cycleNetworkStatus = () => {
    if (networkStatus === "devnet") setNetworkStatus("testnet");
    else if (networkStatus === "testnet") setNetworkStatus("mainnet");
    else setNetworkStatus("devnet");
  };

  const wallpapers = [
    "https://images.unsplash.com/photo-1662955543165-bc0f35af9610?q=80&w=2560&auto=format&fit=crop",
    "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?q=80&w=2564&auto=format&fit=crop",
    "https://images.unsplash.com/photo-1614850523459-c2f4c699c52e?q=80&w=2560&auto=format&fit=crop",
  ];
  const [wallpaperIndex, setWallpaperIndex] = useState(0);

  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
  } | null>(null);

  useEffect(() => {
    const handleDocumentClick = () => {
      setContextMenu(null);
    };
    document.addEventListener("click", handleDocumentClick);
    return () => {
      document.removeEventListener("click", handleDocumentClick);
    };
  }, []);

  const handleContextMenu = (e: React.MouseEvent) => {
    const target = e.target as HTMLElement;
    if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") {
      return;
    }
    e.preventDefault();
    setContextMenu({ x: e.clientX, y: e.clientY });
  };

  const statusConfig = {
    devnet: {
      label: "Devnet",
      colorClass: "text-orange-400",
      borderClass: "border-orange-500/30",
      bgClass: "bg-orange-500/10",
      dotClass: "bg-orange-500/100 shadow-[0_0_8px_rgba(245,158,11,0.2)]",
    },
    testnet: {
      label: "Testnet",
      colorClass: "text-atc-cyan",
      borderClass: "border-atc-cyan/30",
      bgClass: "bg-atc-purple/10",
      dotClass: "bg-atc-cyan shadow-[0_0_8px_rgba(59,130,246,0.2)]",
    },
    mainnet: {
      label: "Live",
      colorClass: "text-green-400",
      borderClass: "border-green-500/30",
      bgClass: "bg-green-500/10",
      dotClass: "bg-green-500/100 shadow-[0_0_8px_rgba(34,197,94,0.2)]",
    },
  }[networkStatus];

  const selectedLayer = LAYERS.find((l) => l.id === activeLayer);

  return (
    <div
      className="min-h-full bg-atc-bg text-slate-100 font-sans selection:bg-atc-cyan/10 flex flex-col h-screen overflow-hidden relative"
      onContextMenu={handleContextMenu}
    >
      {contextMenu && (
        <div
          className="fixed z-[9999] w-48 py-1 bg-[#1a1b26]/90 backdrop-blur-xl border border-white/10 rounded-lg shadow-2xl text-xs font-mono"
          style={{ top: contextMenu.y, left: contextMenu.x }}
        >
          <button
            className="w-full text-left px-4 py-2 hover:bg-white/10 text-slate-300 hover:text-white transition-colors"
            onClick={() => window.location.reload()}
          >
            Refresh
          </button>
          <button
            className="w-full text-left px-4 py-2 hover:bg-white/10 text-slate-300 hover:text-white transition-colors"
            onClick={() => {
              setContextMenu(null);
              setActiveTab("settings");
            }}
          >
            Settings
          </button>
          <div className="h-px bg-white/10 my-1"></div>
          <button
            className="w-full text-left px-4 py-2 hover:bg-white/10 text-slate-300 hover:text-white transition-colors"
            onClick={() => {
              setWallpaperIndex((prev) => (prev + 1) % wallpapers.length);
              setContextMenu(null);
            }}
          >
            Change Wallpaper
          </button>
        </div>
      )}

      {/* Windows 11 Background Decor */}
      <div className="fixed inset-0 z-0 overflow-hidden pointer-events-none">
        <div
          className="absolute inset-0 bg-cover bg-center opacity-30 mix-blend-screen transition-all duration-1000"
          style={{ backgroundImage: `url('${wallpapers[wallpaperIndex]}')` }}
        />
        <div className="absolute top-1/4 left-1/4 w-[800px] h-[800px] bg-blue-500/20 rounded-full blur-[150px]" />
        <div className="absolute bottom-1/4 right-1/4 w-[800px] h-[800px] bg-cyan-500/20 rounded-full blur-[150px]" />
      </div>

      {/* Windows Title Bar */}
      <div className="relative z-50 h-8 bg-black/40 backdrop-blur-md flex items-center justify-between select-none border-b border-white/10">
        <div className="flex items-center px-3 space-x-3">
          <Database className="w-3.5 h-3.5 text-atc-cyan" />
          <span className="text-[11px] font-medium text-slate-300">
            GateToHell - Windows Edition preview
          </span>
        </div>
        <div className="flex items-stretch h-full">
          <button className="px-4 hover:bg-white/10 transition-colors flex items-center justify-center">
            <Minus className="w-3.5 h-3.5 text-slate-300" />
          </button>
          <button className="px-4 hover:bg-white/10 transition-colors flex items-center justify-center">
            <Maximize2 className="w-3 h-3 text-slate-300" />
          </button>
          <button className="px-4 hover:bg-red-500 hover:text-white transition-colors flex items-center justify-center">
            <X className="w-4 h-4 text-slate-300" />
          </button>
        </div>
      </div>

      <div className="relative z-10 flex-1 overflow-y-auto px-4 sm:px-6 lg:px-8 py-6 flex flex-col gap-6">
        {/* Top Navbar */}
        <nav className="flex flex-col sm:flex-row sm:items-center justify-between py-3 border border-white/5 mb-2 bg-black/40 backdrop-blur-2xl px-4 rounded-xl shadow-lg ring-1 ring-white/10">
          <div className="flex items-center gap-6">
            <div className="flex items-center gap-2">
              <div className="bg-atc-purple p-1.5 rounded-md">
                <Database className="w-4 h-4 text-white" />
              </div>
              <span className="font-bold text-slate-100 tracking-tight hidden sm:inline-block">
                GateTo<span className="text-atc-cyan">Hell</span>
              </span>
            </div>
            <div className="hidden lg:flex relative items-center bg-atc-card rounded-md px-3 py-1.5 border border-atc-border w-[400px] focus-within:ring-2 focus-within:ring-atc-cyan/20 focus-within:border-atc-cyan/50 transition-all shadow-[0_0_15px_rgba(0,0,0,0.5)]">
              <Search className="w-4 h-4 text-slate-500 mr-2" />
              <input
                ref={searchInputRef}
                type="text"
                placeholder="Search across departments & documentation..."
                className="bg-transparent border-none outline-none text-sm w-full text-slate-300 placeholder:text-slate-500"
                value={searchQuery}
                onChange={(e) => {
                  setSearchError("");
                  setSearchQuery(e.target.value.replace(/^\s+/, ""));
                }}
                onFocus={() => setIsSearchFocused(true)}
                onBlur={() => setTimeout(() => setIsSearchFocused(false), 200)}
                onKeyDown={handleSearchSubmit}
              />
              <button
                onClick={handleVoiceSearch}
                className="p-1 hover:bg-slate-800 rounded mx-1 text-slate-500 hover:text-white transition-colors"
                title="Voice Search"
              >
                <Mic className="w-4 h-4" />
              </button>
              <div className="flex items-center gap-1 ml-2 opacity-60 pointer-events-none">
                <kbd className="px-1.5 py-0.5 text-[10px] font-mono bg-atc-bg border border-atc-border rounded text-slate-500">
                  ⌘
                </kbd>
                <kbd className="px-1.5 py-0.5 text-[10px] font-mono bg-atc-bg border border-atc-border rounded text-slate-500">
                  K
                </kbd>
              </div>

              {searchError && (
                <div className="absolute top-1/2 -translate-y-1/2 right-12 text-xs text-red-400 font-medium whitespace-nowrap pointer-events-none bg-atc-card px-2 rounded">
                  {searchError}
                </div>
              )}

              {isSearchFocused && !searchError && recentSearches.length > 0 && (
                <div className="absolute top-full left-0 right-0 mt-2 bg-atc-card rounded-md border border-atc-border shadow-lg z-50 overflow-hidden">
                  <div className="px-3 py-2 border-b border-transparent text-[10px] font-bold text-slate-500 uppercase tracking-wider bg-atc-bg">
                    Recent Searches
                  </div>
                  <ul className="max-h-[200px] overflow-y-auto custom-scrollbar">
                    {recentSearches.map((term, i) => (
                      <li key={i}>
                        <button
                          className="w-full text-left px-3 py-2 text-sm text-slate-500 hover:bg-atc-bg hover:text-atc-cyan flex items-center gap-2 transition-colors focus:bg-atc-bg outline-none"
                          onMouseDown={() => {
                            setSearchQuery(term);
                            setRecentSearches((prev) => {
                              const unique = prev.filter((q) => q !== term);
                              return [term, ...unique].slice(0, 10);
                            });
                          }}
                        >
                          <Clock className="w-3.5 h-3.5 text-slate-500" />
                          {term}
                        </button>
                      </li>
                    ))}
                    <li>
                      <button
                        onMouseDown={(e) => {
                          e.preventDefault();
                          setRecentSearches([]);
                          localStorage.removeItem("recentSearches");
                        }}
                        className="w-full text-left px-3 py-2 text-sm text-red-500 hover:bg-atc-bg transition-colors outline-none font-medium text-center border-t border-atc-border mt-1"
                      >
                        Clear History
                      </button>
                    </li>
                  </ul>
                </div>
              )}
            </div>
          </div>
          <div className="flex items-center gap-1 sm:gap-2">
            <button
              onClick={() => setLanguage((l) => (l === "DE" ? "EN" : "DE"))}
              className="flex items-center gap-1.5 px-2.5 py-1.5 bg-black/40 border border-white/5 rounded-md text-xs font-semibold text-slate-300 hover:text-white hover:border-atc-cyan/50 transition-colors mr-1 shadow-[0_0_10px_rgba(0,0,0,0.3)]"
            >
              <Globe className="w-3.5 h-3.5" />
              {language}
            </button>
            <div className="hidden sm:flex items-center gap-2 px-3 py-1.5 bg-black/40 border border-white/5 rounded-md text-xs font-mono text-slate-400 mr-2 shadow-[0_0_10px_rgba(0,0,0,0.3)]">
              <Clock className="w-3.5 h-3.5 text-slate-500" />
              {formatTimer(sessionTime)}
            </div>
            {!address ? (
              <button
                onClick={connectWallet}
                disabled={isConnecting}
                className="flex items-center gap-1.5 px-3 py-1.5 bg-atc-cyan hover:bg-atc-cyan/80 text-black text-sm font-bold rounded-md transition-colors shadow-[0_0_15px_rgba(45,212,191,0.3)] mr-2 disabled:opacity-50"
              >
                {isConnecting ? (
                  <RefreshCw className="w-4 h-4 animate-spin" />
                ) : (
                  <Wallet className="w-4 h-4" />
                )}
                <span className="hidden sm:inline">
                  {isConnecting ? "Connecting..." : "Connect Wallet"}
                </span>
              </button>
            ) : (
              <div className="flex items-center gap-2 mr-2 bg-atc-bg border border-atc-cyan/30 px-3 py-1.5 rounded-md">
                <Wallet className="w-4 h-4 text-atc-cyan" />
                <span className="text-sm font-mono text-slate-300">
                  {address.substring(0, 6)}...
                  {address.substring(address.length - 4)}
                </span>
              </div>
            )}
            <button className="flex items-center gap-1.5 px-3 py-1.5 bg-atc-purple hover:bg-atc-purple/80 text-white text-sm font-medium rounded-md transition-colors shadow-[0_0_15px_rgba(0,0,0,0.5)] mr-1">
              <Plus className="w-4 h-4" />
              <span className="hidden sm:inline">New Form</span>
            </button>
            <div className="w-[1px] h-6 bg-atc-border mx-1 hidden sm:block" />

            <div className="hidden sm:block relative mr-2">
              <select
                value={theme}
                onChange={(e) => setTheme(e.target.value as any)}
                className="appearance-none bg-atc-card border border-atc-cyan/30 text-xs text-slate-300 px-3 py-1.5 pr-8 rounded-md outline-none cursor-pointer hover:border-atc-cyan/50 focus:border-atc-cyan transition-colors font-mono"
              >
                <option value="default">Default Dark</option>
                <option value="oceanic-deep">Oceanic Deep</option>
                <option value="sunset-red">Sunset Red</option>
                <option value="cyber-cyan">Cyber Cyan</option>
              </select>
              <div className="absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none text-slate-400">
                <ArrowDownToLine className="w-3 h-3" />
              </div>
            </div>

            <ApiHealthWidget />
            <ThemeSwitcher />
            <button className="p-2 text-slate-500 hover:text-atc-cyan hover:bg-atc-purple/10 rounded-full transition-colors relative">
              <Bell className="w-4 h-4" />
              <span className="absolute top-1.5 right-1.5 w-1.5 h-1.5 bg-red-500 rounded-full border-2 border-white box-content" />
            </button>
            <button
              onClick={() => setActiveTab("settings")}
              className="p-2 text-slate-500 hover:text-atc-cyan hover:bg-atc-purple/10 rounded-full transition-colors hidden sm:block"
            >
              <Settings className="w-4 h-4" />
            </button>
            <button className="p-1 sm:pl-2 ml-1 hover:bg-atc-card-hover rounded-full transition-colors flex items-center gap-2">
              <div className="w-8 h-8 rounded-full bg-gradient-to-br from-indigo-100 to-blue-100 border border-atc-cyan/30 flex items-center justify-center text-atc-cyan font-medium text-xs">
                MW
              </div>
            </button>
          </div>
        </nav>

        {/* Header Section */}
        {!isFocusMode && (
          <>
            <header className="flex flex-col md:flex-row md:items-end justify-between gap-6 pb-2">
              <div>
                <div className="flex items-center flex-wrap gap-4 mb-4">
                  <div className="flex items-center gap-3">
                    <div className="p-2 border border-atc-cyan/30 rounded bg-atc-purple/10">
                      <Terminal className="w-5 h-5 text-atc-cyan" />
                    </div>
                    <span className="font-mono text-atc-cyan tracking-wider text-sm uppercase">
                      A-TownChain-Ökosystems
                    </span>
                  </div>
                  <button
                    onClick={cycleNetworkStatus}
                    className={`flex items-center gap-2 px-3 py-1 rounded-full border transition-colors hover:bg-opacity-50 ${statusConfig.borderClass} ${statusConfig.bgClass} ml-0 md:ml-2 cursor-pointer`}
                  >
                    <span
                      className={`w-2 h-2 rounded-full animate-pulse ${statusConfig.dotClass}`}
                    />
                    <span
                      className={`font-mono text-xs tracking-wider uppercase ${statusConfig.colorClass}`}
                    >
                      {statusConfig.label}
                    </span>
                  </button>
                </div>
                <h1 className="text-4xl md:text-5xl font-mono tracking-tighter text-white font-medium">
                  A-TownChain-Ökosystems - ATC-OS
                </h1>
                <p className="mt-3 text-slate-500 max-w-2xl text-lg font-light">
                  Hierarchisierung in Tiers: Von der MVP-Grundlage bis zur
                  planetaren Infrastruktur.
                </p>
              </div>

              <div className="flex flex-col sm:flex-row gap-4">
                <div className="flex flex-col gap-2 p-3 rounded-lg bg-atc-card border border-atc-border shrink-0 min-w-44">
                  <span className="text-xs font-mono text-slate-500 font-medium">
                    GITHUB SYNC (ENTERPRISE)
                  </span>
                  <button
                    onClick={handleSync}
                    className="flex flex-col items-start w-full px-3 py-1.5 rounded bg-atc-bg border border-atc-border hover:border-slate-500 transition-colors group relative overflow-hidden"
                    disabled={isSyncing}
                  >
                    {isSyncing && (
                      <div
                        className="absolute inset-0 bg-atc-cyan/10 transition-all duration-75 ease-linear"
                        style={{ width: `${syncProgress}%` }}
                      />
                    )}
                    <div className="flex items-center justify-between w-full relative z-10">
                      <div className="flex items-center gap-2">
                        <div className="relative group/badge">
                          <Github
                            className={`w-4 h-4 transition-colors ${isSyncing ? "text-indigo-400 animate-pulse" : "text-slate-500 group-hover:text-white"}`}
                          />
                          <span
                            className={`absolute -bottom-0.5 -right-0.5 w-2 h-2 rounded-full border border-atc-bg transition-colors duration-500 ${isSyncing ? "bg-amber-400 animate-pulse" : "bg-emerald-500"}`}
                          />
                          {lastSync && (
                            <div className="absolute top-full left-1/2 -translate-x-1/2 mt-2 px-2 py-1.5 bg-[#090b14] border border-white/10 rounded shadow-xl text-[10px] font-mono text-slate-300 whitespace-nowrap opacity-0 invisible group-hover/badge:opacity-100 group-hover/badge:visible transition-all z-[100] text-center leading-tight">
                              <span className="text-emerald-400 font-bold mb-0.5 block">
                                SYNC SUCCESS
                              </span>
                              {lastSync.toLocaleString()}
                            </div>
                          )}
                        </div>
                        <span className="text-xs text-slate-300 font-mono relative overflow-hidden h-4 flex items-center min-w-[120px]">
                          <AnimatePresence mode="popLayout">
                            <motion.span
                              key={
                                isSyncing
                                  ? githubSyncStep || "Syncing..."
                                  : "ATC Repositories"
                              }
                              initial={{ y: 15, opacity: 0 }}
                              animate={{ y: 0, opacity: 1 }}
                              exit={{ y: -15, opacity: 0 }}
                              transition={{
                                duration: 0.3,
                                type: "spring",
                                bounce: 0.4,
                              }}
                              className="whitespace-nowrap absolute"
                            >
                              {isSyncing
                                ? githubSyncStep || "Syncing..."
                                : "ATC Repositories"}
                            </motion.span>
                          </AnimatePresence>
                        </span>
                      </div>
                      <RefreshCw
                        className={`w-3.5 h-3.5 text-atc-cyan relative z-10 transition-transform duration-500 ${isSyncing ? "animate-spin" : ""}`}
                      />
                    </div>
                    <div className="text-[9px] text-slate-500 font-mono pl-6 relative z-10 mt-1">
                      + a-townchain-core <br />
                      + atc-lang <br />
                      + atc-lang-wiki <br />
                      + atc-pack <br />+ atc-trace
                    </div>
                  </button>
                  {lastSync && !isSyncing && (
                    <span className="text-[10px] text-slate-500 font-mono text-right mt-0.5">
                      Last Sync:{" "}
                      {lastSync.toLocaleTimeString([], {
                        hour: "2-digit",
                        minute: "2-digit",
                        second: "2-digit",
                      })}
                    </span>
                  )}
                </div>

                <div className="flex flex-col gap-2 p-3 rounded-lg bg-atc-card border border-atc-border shrink-0 min-w-44">
                  <span className="text-xs font-mono text-slate-500 font-medium">
                    NOTION SYNC
                  </span>
                  <button
                    onClick={() => {
                      setIsNotionSyncing(true);
                      window.dispatchEvent(new Event("trigger-notion-sync"));
                      setTimeout(() => setIsNotionSyncing(false), 2000); // UI reset fallback
                    }}
                    className="flex flex-col items-start w-full px-3 py-1.5 rounded bg-atc-bg border border-atc-border hover:border-slate-500 transition-colors group relative overflow-hidden text-left"
                    disabled={isNotionSyncing}
                  >
                    {isNotionSyncing && (
                      <div className="absolute inset-0 bg-atc-cyan/10 transition-all duration-75 ease-linear animate-pulse" />
                    )}
                    <div className="flex items-center justify-between w-full gap-2 relative z-10">
                      <div className="flex items-center gap-2">
                        <div className="relative group/badge">
                          <FileText
                            className={`w-4 h-4 transition-colors ${isNotionSyncing ? "text-indigo-400 animate-pulse" : "text-slate-500 group-hover:text-white"}`}
                          />
                          <span
                            className={`absolute -bottom-0.5 -right-0.5 w-2 h-2 rounded-full border border-atc-bg transition-colors duration-500 ${syncBadgeColor}`}
                          />
                          {lastSync && (
                            <div className="absolute top-full left-1/2 -translate-x-1/2 mt-2 px-2 py-1.5 bg-[#090b14] border border-white/10 rounded shadow-xl text-[10px] font-mono text-slate-300 whitespace-nowrap opacity-0 invisible group-hover/badge:opacity-100 group-hover/badge:visible transition-all z-[100] text-center leading-tight">
                              <span className="text-emerald-400 font-bold mb-0.5 block">
                                SYNC SUCCESS
                              </span>
                              {lastSync.toLocaleString()}
                            </div>
                          )}
                        </div>
                        <span className="text-xs text-slate-300 font-mono relative overflow-hidden h-4 flex items-center min-w-[120px]">
                          <AnimatePresence mode="popLayout">
                            <motion.span
                              key={
                                isNotionSyncing
                                  ? notionSyncStep || "Syncing..."
                                  : "A-TownChain Wiki"
                              }
                              initial={{ y: 15, opacity: 0 }}
                              animate={{ y: 0, opacity: 1 }}
                              exit={{ y: -15, opacity: 0 }}
                              transition={{
                                duration: 0.3,
                                type: "spring",
                                bounce: 0.4,
                              }}
                              className="whitespace-nowrap absolute"
                            >
                              {isNotionSyncing
                                ? notionSyncStep || "Syncing..."
                                : "A-TownChain Wiki"}
                            </motion.span>
                          </AnimatePresence>
                        </span>
                      </div>
                      <RefreshCw
                        className={`w-3.5 h-3.5 text-atc-cyan relative z-10 transition-transform duration-500 ${isNotionSyncing ? "animate-spin" : ""}`}
                      />
                    </div>
                  </button>
                  {notionSyncStatus && (
                    <span className="text-[10px] text-slate-500 font-mono text-right mt-0.5 max-w-[150px] truncate">
                      {notionSyncStatus}
                    </span>
                  )}
                </div>

                {(offlineQueue.length > 0 || !isOnline) && (
                  <div className="flex flex-col gap-2 p-3 rounded-lg bg-red-950/20 border border-red-900/50 shrink-0 min-w-44">
                    <div className="flex items-center justify-between gap-4">
                      <span className="text-xs font-mono text-slate-500 font-medium whitespace-nowrap">
                        OFFLINE QUEUE
                      </span>
                      <span className="text-[10px] font-bold text-red-500 uppercase px-1.5 py-0.5 rounded bg-red-500/10">
                        {offlineQueue.length} PENDING
                      </span>
                    </div>
                    <button
                      onClick={handleFlushQueue}
                      disabled={
                        !isOnline ||
                        isSyncing ||
                        isNotionSyncing ||
                        offlineQueue.length === 0
                      }
                      className="flex items-center justify-center gap-2 w-full px-3 py-1.5 rounded bg-red-500/10 border border-red-500/20 text-red-400 hover:bg-red-500/20 hover:text-red-300 transition-colors disabled:opacity-50"
                      title={
                        !isOnline ? "Waiting for connection..." : "Flush Queue"
                      }
                    >
                      <Network className="w-3.5 h-3.5" />
                      <span className="text-xs font-semibold uppercase tracking-wider">
                        {!isOnline ? "Offline" : "Flush Queue"}
                      </span>
                    </button>
                  </div>
                )}

                <div className="flex flex-col gap-1 p-4 rounded-lg bg-atc-bg border border-atc-border-hover shrink-0">
                  <span className="text-xs font-mono text-slate-500 font-medium">
                    VERGLEICHS-KOMPLEXITÄT
                  </span>
                  <div className="flex items-center gap-2 mt-1">
                    <span className="w-1.5 h-1.5 rounded-full bg-atc-purple animate-pulse" />
                    <span className="text-sm text-slate-300 font-mono tracking-tight leading-tight">
                      Linux + K8s +<br />
                      Ethereum + AWS + OpenAI
                    </span>
                  </div>
                </div>
              </div>
            </header>

            {/* Navigation Tabs Header / Toggle */}
            <div className="flex md:hidden bg-black/40 backdrop-blur-2xl p-2 rounded-xl mb-2 items-center justify-between border border-white/5 ring-1 ring-white/10 mx-1">
              <span className="text-xs font-bold font-mono text-atc-cyan flex items-center gap-2">
                <Menu
                  className="w-5 h-5 cursor-pointer text-slate-300"
                  onClick={() => setIsSidebarOpen(true)}
                />
                MENU & NAVIGATION
              </span>
            </div>

            {/* Navigation Tabs (Desktop / Mobile Sidebar) */}
            {isSidebarOpen && (
              <div
                className="fixed inset-0 bg-black/60 backdrop-blur-md z-40 md:hidden"
                onClick={() => setIsSidebarOpen(false)}
              />
            )}
            <div
              className={`md:flex bg-black/40 backdrop-blur-2xl p-1.5 md:rounded-xl border border-white/5 ring-1 ring-white/10 md:overflow-x-auto custom-scrollbar md:gap-1 mb-2 items-center mx-1 transition-all z-[100] ${isSidebarOpen ? "fixed inset-y-0 left-0 w-64 bg-[#111111]/90 backdrop-blur-2xl flex flex-col p-4 shadow-2xl items-stretch border-r border-white/10 z-50" : "hidden"} md:static md:w-auto md:bg-black/40 md:flex-row md:items-center`}
            >
              {/* Close button for mobile sidebar */}
              <div className="md:hidden flex items-center justify-between mb-4 border-b border-white/10 pb-2">
                <span className="font-bold text-atc-cyan text-xs">MENU</span>
                <X
                  className="w-5 h-5 cursor-pointer text-slate-400 hover:text-white"
                  onClick={() => setIsSidebarOpen(false)}
                />
              </div>

              {[
                { id: "wiki", label: "WIKI & DOCS", bereich: "Allgemein" },
                {
                  id: "atc_lang_wiki",
                  label: "ATC-LANG WIKI",
                  bereich: "Sprache",
                },
                {
                  id: "hierarchy",
                  label: "TIER ARCHITECTURE",
                  bereich: "Architektur & System",
                },
                {
                  id: "architecture",
                  label: "STRATEGIC LAYERS",
                  bereich: "Architektur & System",
                },
                {
                  id: "reports",
                  label: "SYSTEM REPORTS",
                  bereich: "Architektur & System",
                },
                { id: "roadmap", label: "ROADMAP", bereich: "Strategie" },
                {
                  id: "atc_lang_roadmap",
                  label: "ATC-LANG ROADMAP",
                  bereich: "Sprache",
                },
                { id: "ecosystem", label: "STAGES", bereich: "Planung" },
                {
                  id: "standards",
                  label: "A-TOWN REGELN ATC-STANDARDS & ATS-STANDARDS",
                  bereich: "Code & Specs",
                },
                {
                  id: "audit",
                  label: "AUDIT & GAP ANALYSIS",
                  bereich: "Security",
                },
                {
                  id: "software_audit",
                  label: "SOFTWARE KOMPLETT AUDIT",
                  bereich: "Security",
                },
                {
                  id: "tech_stack",
                  label: "TECH STACK",
                  bereich: "Engineering",
                },
                {
                  id: "structure",
                  label: "FILE STRUCTURE",
                  bereich: "Architektur & System",
                },
                { id: "tech_tree", label: "TECH TREE", bereich: "R&D" },
                {
                  id: "tech_docs",
                  label: "TECH DOCS",
                  bereich: "Code & Specs",
                },
                {
                  id: "metrics",
                  label: "SYSTEM METRICS",
                  bereich: "Monitoring & Ops",
                },
                {
                  id: "modules",
                  label: "MODULES & PLUGINS",
                  bereich: "Architektur & System",
                },
                { id: "todos", label: "TO-DOS", bereich: "Planung" },
                {
                  id: "atc_lang_presets",
                  label: "ATC-LANG PRESETS",
                  bereich: "Sprache",
                },
                {
                  id: "atc_lang_architecture",
                  label: "ATC-LANG ARCHITEKTUR",
                  bereich: "Sprache",
                },
                {
                  id: "atc_core_kernel",
                  label: "ATC-CORE OS KERNEL",
                  bereich: "System",
                },
                {
                  id: "atc_lang_playground",
                  label: "ATC-LANG PLAYGROUND",
                  bereich: "Sprache",
                },
                {
                  id: "atc_assets_db",
                  label: "ATC-LANG ASSETS DB",
                  bereich: "Sprache",
                },
                {
                  id: "atc_lang_todos",
                  label: "ATC-LANG TO-DOS",
                  bereich: "Sprache",
                },
                {
                  id: "github_sync",
                  label: "GITHUB REPOS",
                  bereich: "System Administration",
                },
                {
                  id: "atvm_sandbox",
                  label: "ATVM COMPILER SANDBOX",
                  bereich: "Tools",
                },
                {
                  id: "explorer",
                  label: "ATC BLOCKCHAIN DATEN & STATISTIKEN",
                  bereich: "Monitoring & Ops",
                },
                {
                  id: "topology",
                  label: "NETWORK TOPOLOGY",
                  bereich: "Monitoring & Ops",
                },
                {
                  id: "governance",
                  label: "DAO GOVERNANCE",
                  bereich: "Blockchain & Token",
                },
                {
                  id: "compliance",
                  label: "PRIVACY & COMPLIANCE",
                  bereich: "Security",
                },
                {
                  id: "legal",
                  label: "LEGAL & IMPRESSUM",
                  bereich: "Security",
                },
                { id: "ai_os", label: "AI & OS CORE", bereich: "System Core" },
                {
                  id: "gitops",
                  label: "GITOPS & VERSION CONTROL",
                  bereich: "System Core",
                },
                {
                  id: "code_analyzer",
                  label: "CODEBASE ANALYZER & VERBESSERUNGEN",
                  bereich: "System Core",
                },
                {
                  id: "payment",
                  label: "ZAHLUNGSSYSTEM & TOKENOMICS",
                  bereich: "Finance",
                },
                {
                  id: "data_processing",
                  label: "DATENVERARBEITUNG & ETL",
                  bereich: "Architektur & System",
                },
                {
                  id: "interfaces",
                  label: "SCHNITTSTELLEN & APIs",
                  bereich: "Architektur & System",
                },
                {
                  id: "vision_milestones",
                  label: "VISION & MEILENSTEINE",
                  bereich: "Planung",
                },
                {
                  id: "semantic_graph",
                  label: "SEMANTIC GRAPH & ONTOLOGY",
                  bereich: "System Core",
                },
                {
                  id: "db_orchestrator",
                  label: "DATABASE ORCHESTRATOR",
                  bereich: "Architektur & System",
                },
                {
                  id: "tx_orchestrator",
                  label: "TRANSACTIONS ORCHESTRATOR",
                  bereich: "Architektur & System",
                },
              ].map((tab) => {
                const Icon = BEREICH_ICON_MAP[tab.bereich] || Settings2;
                return (
                  <button
                    key={tab.id}
                    onClick={() => {
                      setActiveTab(tab.id as any);
                      setIsSidebarOpen(false);
                    }}
                    className={`flex flex-col items-start px-4 py-2 transition-all rounded-lg shrink-0 ${
                      activeTab === tab.id
                        ? "bg-atc-card shadow-[0_0_15px_rgba(6,182,212,0.15)] ring-1 ring-atc-cyan/50"
                        : "hover:bg-atc-card/50"
                    }`}
                  >
                    <div className="flex items-center gap-2 mb-0.5">
                      <Icon
                        className={`w-3 h-3 ${activeTab === tab.id ? "text-atc-cyan" : "text-slate-500"}`}
                      />
                      <span
                        className={`text-[9px] uppercase tracking-widest font-mono ${
                          activeTab === tab.id
                            ? "text-atc-cyan/80"
                            : "text-slate-500"
                        }`}
                      >
                        {tab.bereich}
                      </span>
                    </div>
                    <span
                      className={`text-[11px] font-mono tracking-widest uppercase ${
                        activeTab === tab.id
                          ? "text-atc-cyan font-bold"
                          : "text-slate-400 font-medium"
                      }`}
                    >
                      {tab.label}
                    </span>
                  </button>
                );
              })}

              <div className="md:border-l border-white/10 md:ml-2 md:pl-2 mt-4 md:mt-0 pt-4 md:pt-0 border-t md:border-t-0 flex flex-col shrink-0">
                <select
                  value={theme}
                  onChange={(e) => setTheme(e.target.value as any)}
                  className="bg-black/40 border border-white/10 text-slate-400 font-mono text-[9px] uppercase tracking-widest py-1.5 px-2 rounded-lg focus:outline-none focus:border-atc-cyan cursor-pointer hover:bg-white/5 transition-colors"
                >
                  <option value="default">Default Dark</option>
                  <option value="oceanic-deep">Oceanic Deep</option>
                  <option value="sunset-red">Sunset Red</option>
                  <option value="cyber-cyan">Cyber Cyan</option>
                </select>
              </div>
            </div>
          </>
        )}

        {activeTab === "wiki" && (
          <WikiView setActiveTab={setActiveTab} addBookmark={addBookmark} />
        )}

        {activeTab === "atc_lang_wiki" && (
          <WikiView
            setActiveTab={setActiveTab}
            addBookmark={addBookmark}
            wikiContent={ATC_LANG_WIKI_CONTENT}
          />
        )}

        {activeTab === "hierarchy" && <HierarchyView />}

        {activeTab === "standards" && <StandardsView />}

        {activeTab === "audit" && <AuditView />}

        {activeTab === "software_audit" && <SoftwareAuditView />}

        {activeTab === "tech_stack" && <TechStackView />}

        {activeTab === "structure" && <StructureView />}

        {activeTab === "tech_tree" && (
          <TechTreeView addBookmark={addBookmark} />
        )}

        {activeTab === "todos" && <TodoView />}

        {activeTab === "atc_lang_todos" && (
          <TodoView
            storageKey="atc_lang_todos_v1"
            initialConfig={[
              {
                id: "1",
                text: "Syntax: Formaler Regex und BNF Parser definieren",
                completed: true,
                createdAt: Date.now() - 1000000,
              },
              {
                id: "2",
                text: "Typ-System: Affine und lineare Typen implementieren",
                completed: true,
                createdAt: Date.now() - 900000,
              },
              {
                id: "3",
                text: "Borrow Checker: Mutabilität via `&mut` einschränken",
                completed: true,
                createdAt: Date.now() - 800000,
              },
              {
                id: "4",
                text: "Compiler: ATVM Targets für O(1) Memory Layout generieren",
                completed: true,
                createdAt: Date.now() - 700000,
              },
              {
                id: "5",
                text: "Kryptographie: std::crypto mit secp256k1 bestücken",
                completed: true,
                createdAt: Date.now() - 600000,
              },
              {
                id: "6",
                text: "ZK: `proof {}` block als PLONK circuit auflösen",
                completed: true,
                createdAt: Date.now() - 500000,
              },
              {
                id: "7",
                text: "Verifikation: `atc-verify` Tool für Z3-Modellierung binden",
                completed: true,
                createdAt: Date.now() - 400000,
              },
              {
                id: "8",
                text: "Gas: Worst-Case Execution Time Analyzer für jeden AST-Node",
                completed: true,
                createdAt: Date.now() - 300000,
              },
              {
                id: "9",
                text: "LSP: Auto-complete für VS Code Plugin bauen",
                completed: true,
                createdAt: Date.now() - 200000,
              },
              {
                id: "10",
                text: "Makros: Hygienische `atc_macro!` Parser-Extension publishen",
                completed: true,
                createdAt: Date.now() - 100000,
              },
              {
                id: "11",
                text: "ATC-Pack: IPFS Resolver für dezentrale Dependencies schreiben",
                completed: true,
                createdAt: Date.now() - 95000,
              },
              {
                id: "12",
                text: "ATC-Pack: Hash-Pinning Verifikation in Cargo.toml-like Syntax",
                completed: true,
                createdAt: Date.now() - 90000,
              },
              {
                id: "13",
                text: "Unsafe: `unsafe {}` Block Support und ATVM-Intrinsics mappen",
                completed: true,
                createdAt: Date.now() - 85000,
              },
              {
                id: "14",
                text: "Unsafe: Audit-Flags für DAO Review Pipeline generieren",
                completed: true,
                createdAt: Date.now() - 80000,
              },
              {
                id: "15",
                text: "Interoperabilität: `[cross_chain]` Pragma für IBC Messages umsetzen",
                completed: true,
                createdAt: Date.now() - 75000,
              },
              {
                id: "16",
                text: "Fuzzer: ATC-Fuzz Coverage-map Generator implementieren",
                completed: true,
                createdAt: Date.now() - 70000,
              },
              {
                id: "17",
                text: "Fuzzer: State-Mutation Stresstest gegen Underflow-Vulnerabilities",
                completed: true,
                createdAt: Date.now() - 65000,
              },
              {
                id: "18",
                text: "Debugger: ATC-Trace Memory-Delta Caching für Reverse-Step",
                completed: true,
                createdAt: Date.now() - 60000,
              },
              {
                id: "19",
                text: "Debugger: VS Code Adapter Protocol (DAP) für Web-IDE",
                completed: true,
                createdAt: Date.now() - 55000,
              },
              {
                id: "20",
                text: "Repositories: Dependabot / Renovate an `atc-lang` GitHub Repo anbinden",
                completed: true,
                createdAt: Date.now() - 50000,
              },
            ]}
          />
        )}

        {activeTab === "explorer" && <NetworkExplorerView />}

        {activeTab === "topology" && <NetworkTopologyView />}

        {activeTab === "governance" && (
          <React.Suspense fallback={<div className="flex justify-center p-8"><div className="animate-spin rounded-full h-8 w-8 border-b-2 border-atc-cyan"></div></div>}>
            <GovernanceView />
          </React.Suspense>
        )}

        {activeTab === "compliance" && <ComplianceView />}

        {activeTab === "legal" && <LegalView language={language} />}

        {activeTab === "atc_lang_presets" && <AtcLangPresetsView />}

        {activeTab === "atc_lang_architecture" && <AtcLangArchitectureView />}

        {activeTab === "atc_core_kernel" && <AtcCoreKernelView />}

        {activeTab === "atc_lang_playground" && <AtcLangPlaygroundView />}

        {activeTab === "atc_assets_db" && <AtcAssetsDbView />}

        {activeTab === "franchise_factory" && <FranchiseFactoryView />}

        {activeTab === "semantic_graph" && <SemanticGraphView />}

        {activeTab === "db_orchestrator" && <DbOrchestratorView />}

        {activeTab === "tx_orchestrator" && <TxOrchestratorView />}

        {activeTab === "gitops" && <GitOpsView />}

        {activeTab === "code_analyzer" && <CodeAnalyzerView />}

        {activeTab === "payment" && <PaymentSystemView />}

        {activeTab === "data_processing" && <DataProcessingView />}

        {activeTab === "interfaces" && <InterfacesView />}

        {activeTab === "vision_milestones" && (
          <RoadmapView
            conflictItems={conflictItems}
            setConflictItems={setConflictItems}
            showConflictModal={showConflictModal}
            setShowConflictModal={setShowConflictModal}
            syncHistory={syncHistory}
            setSyncHistory={setSyncHistory}
          />
        )}

        {activeTab === "ai_os" && <AiOsEngineView />}

        {activeTab === "atown_test" && <ATownTestView />}
        {activeTab === "social_media" && <SocialMediaView />}

        {activeTab === "tech_docs" && <TechDocsView />}

        {activeTab === "metrics" && (
          <MetricsView searchHistory={recentSearches} />
        )}

        {activeTab === "modules" && <ModulesPluginView />}

        {activeTab === "settings" && (
          <SettingsView
            syncHistory={syncHistory}
            offlineQueue={offlineQueue}
            onClearOfflineQueue={() => {
              setOfflineQueue([]);
              localStorage.removeItem("atc_offline_sync_queue");
            }}
            isSyncPaused={isSyncPaused}
            setIsSyncPaused={setIsSyncPaused}
            isFocusMode={isFocusMode}
            setIsFocusMode={setIsFocusMode}
            bookmarks={bookmarks}
            removeBookmark={removeBookmark}
            onNavigate={(tab) => setActiveTab(tab)}
            conflictItems={conflictItems}
            onResolveConflict={(id, resolution) => {
              setConflictItems((prev) => prev.filter((i) => i.id !== id));
              if (conflictItems.length <= 1) setShowConflictModal(false);
            }}
          />
        )}

        {activeTab === "atvm_sandbox" && <AtvmSandboxView />}

        {activeTab === "github_sync" && (
          <GitHubRepoSyncView language={language} />
        )}

        {activeTab === "architecture" && (
          <div className="flex flex-col gap-12 mt-4">
            {/* Target State Dashboard */}
            <section>
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
                <div className="flex items-center gap-2">
                  <Activity className="w-5 h-5 text-atc-purple" />
                  <h2 className="text-xl font-mono text-white">
                    Scale & Complexity Shift
                  </h2>
                </div>
                <button
                  onClick={async () => {
                    try {
                      // jsPDF is imported statically at the top of the file
                      const doc = new jsPDF();

                      doc.setFillColor(5, 8, 15);
                      doc.rect(0, 0, 210, 297, "F");

                      doc.setTextColor(255, 255, 255);
                      doc.setFont("helvetica", "bold");
                      doc.setFontSize(22);
                      doc.text("A-TOWN OS - SYSTEM REPORT", 20, 30);

                      doc.setFontSize(12);
                      doc.setFont("helvetica", "normal");
                      doc.setTextColor(150, 150, 150);
                      doc.text(
                        `Generated on: ${new Date().toLocaleString()}`,
                        20,
                        40,
                      );

                      doc.setDrawColor(40, 40, 60);
                      doc.line(20, 45, 190, 45);

                      doc.setFontSize(16);
                      doc.setTextColor(0, 209, 255);
                      doc.text("System Metrics", 20, 60);

                      doc.setFontSize(12);
                      doc.setTextColor(220, 220, 220);

                      let yPos = 75;
                      doc.text(
                        `Main Systems: ${TARGET_STATE.mainSystems.current} / ${TARGET_STATE.mainSystems.vision}`,
                        20,
                        yPos,
                      );
                      yPos += 10;
                      doc.text(
                        `Subsystems: ${TARGET_STATE.subsystems.current} / ${TARGET_STATE.subsystems.vision}`,
                        20,
                        yPos,
                      );
                      yPos += 10;
                      doc.text(
                        `Core Components: ${TARGET_STATE.coreComponents.current} / ${TARGET_STATE.coreComponents.vision}`,
                        20,
                        yPos,
                      );
                      yPos += 10;
                      doc.text(
                        `Services: ${TARGET_STATE.services.current} / ${TARGET_STATE.services.vision}`,
                        20,
                        yPos,
                      );
                      yPos += 20;

                      doc.setFontSize(16);
                      doc.setTextColor(162, 89, 255);
                      doc.text("Health Status", 20, yPos);
                      yPos += 15;

                      doc.setFontSize(12);
                      doc.setTextColor(220, 220, 220);
                      doc.text("Overall Health: OPTIMAL", 20, yPos);
                      yPos += 10;
                      doc.text(
                        "Network Status: Connected to Peer-to-Peer Relay",
                        20,
                        yPos,
                      );
                      yPos += 10;
                      doc.text(
                        "Last Audit: Passed without vulnerabilities",
                        20,
                        yPos,
                      );
                      yPos += 20;

                      doc.save("atown_os_system_report.pdf");
                    } catch (err) {
                      console.error("Failed to generate PDF", err);
                    }
                  }}
                  className="flex items-center gap-2 bg-atc-card hover:bg-white/10 text-slate-300 hover:text-white px-4 py-2 rounded-lg transition-colors text-sm font-medium border border-atc-border whitespace-nowrap shadow-md"
                >
                  <Search className="w-4 h-4 hidden" />{" "}
                  {/* Just to make sure we don't need a new import if missing */}
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  >
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                    <polyline points="7 10 12 15 17 10" />
                    <line x1="12" x2="12" y1="15" y2="3" />
                  </svg>
                  Download Info Report
                </button>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                <StatCard
                  icon={Layers}
                  title="Hauptsysteme"
                  current={TARGET_STATE.mainSystems.current.toString()}
                  vision={TARGET_STATE.mainSystems.vision.toString()}
                  info="The number of primary architectural domains currently active."
                />
                <StatCard
                  icon={Database}
                  title="Subsysteme"
                  current={TARGET_STATE.subsystems.current.toString()}
                  vision={TARGET_STATE.subsystems.vision.toString()}
                  info="The distinct subsystems operating underneath the main domains."
                />
                <StatCard
                  icon={Component}
                  title="Kernkomponenten"
                  current={TARGET_STATE.coreComponents.current.toString()}
                  vision={TARGET_STATE.coreComponents.vision.toString()}
                  info="The number of active foundational modules supporting system logic."
                />
                <StatCard
                  icon={Network}
                  title="Services"
                  current={TARGET_STATE.services.current.toString()}
                  vision={TARGET_STATE.services.vision.toString()}
                  highlight
                  info="Microservices running actively within the architecture."
                />
              </div>
            </section>

            {/* The 10 Strategic Layers */}
            <section>
              <div className="flex justify-between items-center mb-6">
                <div className="flex items-center gap-2">
                  <Microscope className="w-5 h-5 text-atc-cyan" />
                  <h2 className="text-xl font-mono text-white">
                    Full Infrastructure Layers (1–30)
                  </h2>
                </div>
              </div>

              <div className="w-full flex items-center gap-2 overflow-x-auto pb-4 mb-4 custom-scrollbar">
                <span className="text-xs font-mono text-slate-500 uppercase tracking-widest whitespace-nowrap hidden md:block">
                  Quick Jump:
                </span>
                {Array.from({ length: 30 }, (_, i) => i + 1).map((id) => {
                  const hasData = LAYERS.some((l) => l.id === id);
                  return (
                    <button
                      key={id}
                      onClick={() => {
                        if (hasData) {
                          setActiveLayer(id);
                        }
                      }}
                      disabled={!hasData}
                      className={`flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center font-mono text-xs font-bold transition-colors ${
                        activeLayer === id
                          ? "bg-atc-cyan text-black shadow-[0_0_10px_rgba(6,182,212,0.5)]"
                          : hasData
                            ? "bg-white/5 text-slate-400 border border-white/10 hover:bg-white/10 hover:text-white"
                            : "bg-transparent text-slate-700 cursor-not-allowed hidden"
                      }`}
                    >
                      {id}
                    </button>
                  );
                })}
              </div>

              <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 relative">
                {/* List View */}
                <div className="lg:col-span-5 flex flex-col gap-3 h-[600px] overflow-y-auto pr-2 custom-scrollbar">
                  {LAYERS.map((layer) => {
                    const Icon = ICON_MAP[layer.iconName] || Database;
                    const isActive = activeLayer === layer.id;
                    return (
                      <button
                        key={layer.id}
                        onClick={() =>
                          setActiveLayer(isActive ? null : layer.id)
                        }
                        className={`text-left group flex items-start gap-4 p-4 rounded-xl border transition-all duration-300 ${
                          isActive
                            ? "bg-atc-purple/10 border-atc-cyan/50 shadow-[0_0_20px_rgba(6,182,212,0.1)]"
                            : "bg-atc-card border-atc-border hover:bg-atc-bg hover:border-atc-border-hover"
                        }`}
                      >
                        <div
                          className={`mt-0.5 p-2 rounded-lg transition-colors duration-300 ${isActive ? "bg-atc-cyan/20 text-atc-cyan" : "bg-atc-bg text-slate-500 group-hover:text-slate-300 group-hover:bg-atc-card/10"}`}
                        >
                          <Icon className="w-5 h-5" />
                        </div>
                        <div className="flex-1">
                          <div className="flex items-center gap-3">
                            <span className="font-mono text-xs font-semibold text-slate-500">
                              L{layer.id}
                            </span>
                            <h3
                              className={`font-medium tracking-tight ${isActive ? "text-white" : "text-slate-300"}`}
                            >
                              {layer.name}
                            </h3>
                          </div>
                          <p className="mt-1.5 text-sm text-slate-500 line-clamp-2">
                            {layer.purpose}
                          </p>
                        </div>
                      </button>
                    );
                  })}
                </div>

                {/* Detail View */}
                <div className="lg:col-span-7 h-[600px]">
                  <AnimatePresence mode="popLayout">
                    {selectedLayer ? (
                      <motion.div
                        key="detail"
                        initial={{ opacity: 0, x: 20 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, scale: 0.95 }}
                        transition={{ duration: 0.3 }}
                        className="h-full bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-8 flex flex-col relative overflow-hidden shadow-2xl"
                      >
                        <div className="absolute top-0 right-0 w-64 h-64 bg-atc-purple/5 rounded-full blur-[100px] pointer-events-none" />
                        <div className="flex items-center gap-4 mb-6 relative z-10">
                          <div className="w-12 h-12 rounded-xl bg-atc-purple/10 border border-atc-cyan/30 flex items-center justify-center text-atc-cyan shadow-[0_0_15px_rgba(6,182,212,0.2)]">
                            {(() => {
                              const Icon =
                                ICON_MAP[selectedLayer.iconName] || Database;
                              return <Icon className="w-6 h-6" />;
                            })()}
                          </div>
                          <div>
                            <div className="text-atc-cyan font-mono text-sm tracking-wide mb-1 drop-shadow-[0_0_5px_rgba(6,182,212,0.5)]">
                              LAYER {selectedLayer.id}
                            </div>
                            <h2 className="text-2xl text-white font-medium">
                              {selectedLayer.name}
                            </h2>
                          </div>
                        </div>

                        <p className="text-slate-300 text-lg mb-8 leading-relaxed relative z-10">
                          {selectedLayer.purpose}
                        </p>

                        <div className="flex-1 overflow-y-auto pr-2 custom-scrollbar relative z-10">
                          <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
                            Sub-Layers
                          </h3>
                          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                            {selectedLayer.subLayers.map((comp: string, idx: number) => (
                              <div
                                key={idx}
                                className="flex items-center gap-3 p-3 rounded-xl bg-[#090b14]/60 border border-atc-border/50 hover:bg-[#090b14] hover:border-atc-purple/30 transition-colors"
                              >
                                <div className="w-1.5 h-1.5 rounded-full bg-atc-purple shadow-[0_0_5px_rgba(162,89,255,0.8)]" />
                                <span className="text-sm text-slate-300 font-medium">
                                  {comp}
                                </span>
                              </div>
                            ))}
                          </div>
                        </div>
                      </motion.div>
                    ) : (
                      <motion.div
                        key="empty"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        className="h-full flex flex-col items-center justify-center text-center p-8 rounded-2xl border border-dashed border-atc-border/50 bg-[#090b14]/30 backdrop-blur-sm shadow-xl"
                      >
                        <div className="w-16 h-16 mb-4 rounded-full bg-[#090b14]/50 flex items-center justify-center border border-atc-border/50 shadow-[0_0_15px_rgba(0,0,0,0.5)]">
                          <Layers className="w-8 h-8 text-slate-500" />
                        </div>
                        <div className="text-slate-300 font-medium tracking-wide">
                          Select a structural layer
                        </div>
                        <div className="text-slate-500 text-sm mt-2 max-w-sm">
                          Inspect the inner architectural components of the 10
                          extended strategic systems
                        </div>
                      </motion.div>
                    )}
                  </AnimatePresence>
                </div>
              </div>
            </section>

            {/* Global Infrastructure Banner */}
            <section className="pb-12">
              <div className="p-8 rounded-2xl bg-gradient-to-br from-slate-50 to-white border border-atc-border relative overflow-hidden">
                <div className="absolute right-0 top-0 w-[400px] h-[400px] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.05] mix-blend-overlay pointer-events-none" />

                <div className="relative z-10 flex flex-col md:flex-row gap-8 justify-between">
                  <div className="max-w-xl">
                    <h2 className="text-2xl font-mono text-white mb-4">
                      Vision: Infrastruktur
                    </h2>
                    <p className="text-slate-500 mb-6 font-light leading-relaxed">
                      Der endgültige Ausbau gleicht einer globalen, in sich
                      verzahnten Zivilisation aus Code, Agenten und
                      Validierungsnetzen.
                    </p>
                    <div className="flex flex-wrap gap-2">
                      {LONG_TERM_VISION.map((tag, i) => (
                        <span
                          key={i}
                          className="px-3 py-1 text-xs font-mono rounded-full bg-atc-card border border-atc-border text-slate-300"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  </div>

                  <div className="flex flex-col items-start justify-center">
                    <div className="text-sm font-mono text-slate-500 uppercase tracking-wider mb-2">
                      Scope Target
                    </div>
                    <div className="text-4xl text-white font-medium tracking-tighter">
                      5.000+
                    </div>
                    <div className="text-atc-cyan font-mono text-sm">
                      Kernkomponenten
                    </div>
                    <div className="w-32 h-[1px] bg-gradient-to-r from-blue-500/50 to-transparent my-3" />
                    <div className="text-slate-500 text-sm">
                      Millions of lines of verified code
                    </div>
                  </div>
                </div>
              </div>
            </section>
          </div>
        )}

        {activeTab === "roadmap" && (
          <RoadmapView
            conflictItems={conflictItems}
            setConflictItems={setConflictItems}
            showConflictModal={showConflictModal}
            setShowConflictModal={setShowConflictModal}
            syncHistory={syncHistory}
            setSyncHistory={setSyncHistory}
          />
        )}

        {activeTab === "reports" && <ReportsView />}

        {activeTab === "atc_lang_roadmap" && (
          <RoadmapView
            initialData={ATC_LANG_ROADMAP_DATA}
            conflictItems={conflictItems}
            setConflictItems={setConflictItems}
            showConflictModal={showConflictModal}
            setShowConflictModal={setShowConflictModal}
            syncHistory={syncHistory}
            setSyncHistory={setSyncHistory}
          />
        )}

        {activeTab === "ecosystem" && <EcosystemView />}
      </div>

      {/* Keyboard Cheat Sheet Modal */}
      <AnimatePresence>
        {showCheatSheet && (
          <div className="fixed inset-0 z-[100] flex items-center justify-center p-4">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="absolute inset-0 bg-black/60 backdrop-blur-sm"
              onClick={() => setShowCheatSheet(false)}
            />
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 20 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 20 }}
              className="relative w-full max-w-md bg-atc-card border border-atc-border rounded-xl shadow-2xl overflow-hidden"
            >
              <div className="p-4 bg-atc-bg border-b border-atc-border flex justify-between items-center">
                <h3 className="font-semibold text-lg text-white">
                  Keyboard Shortcuts
                </h3>
                <button
                  onClick={() => setShowCheatSheet(false)}
                  className="text-slate-500 hover:text-white transition-colors"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>
              <div className="p-6 bg-black/20 space-y-4">
                <div className="flex items-center justify-between border-b border-atc-border pb-4">
                  <span className="text-sm font-medium text-slate-300">
                    Global Search
                  </span>
                  <div className="flex items-center gap-1">
                    <kbd className="px-2 py-1 bg-atc-bg border border-atc-border rounded text-slate-400 font-mono text-xs shadow-inner">
                      Ctrl/Cmd
                    </kbd>
                    <span className="text-slate-500">+</span>
                    <kbd className="px-2 py-1 bg-atc-bg border border-atc-border rounded text-slate-400 font-mono text-xs shadow-inner">
                      K
                    </kbd>
                  </div>
                </div>
                <div className="flex items-center justify-between border-b border-atc-border pb-4">
                  <span className="text-sm font-medium text-slate-300">
                    Save Documents
                  </span>
                  <div className="flex items-center gap-1">
                    <kbd className="px-2 py-1 bg-atc-bg border border-atc-border rounded text-slate-400 font-mono text-xs shadow-inner">
                      Ctrl/Cmd
                    </kbd>
                    <span className="text-slate-500">+</span>
                    <kbd className="px-2 py-1 bg-atc-bg border border-atc-border rounded text-slate-400 font-mono text-xs shadow-inner">
                      S
                    </kbd>
                  </div>
                </div>
                <div className="flex items-center justify-between border-b border-atc-border pb-4">
                  <span className="text-sm font-medium text-slate-300">
                    Toggle Cheat Sheet
                  </span>
                  <div className="flex items-center gap-1">
                    <kbd className="px-2 py-1 bg-atc-bg border border-atc-border rounded text-slate-400 font-mono text-xs shadow-inner">
                      Ctrl/Cmd
                    </kbd>
                    <span className="text-slate-500">+</span>
                    <kbd className="px-2 py-1 bg-atc-bg border border-atc-border rounded text-slate-400 font-mono text-xs shadow-inner">
                      /
                    </kbd>
                  </div>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium text-slate-300">
                    Close Modals / Blur Search
                  </span>
                  <div className="flex items-center gap-1">
                    <kbd className="px-2 py-1 bg-atc-bg border border-atc-border rounded text-slate-400 font-mono text-xs shadow-inner">
                      Esc
                    </kbd>
                  </div>
                </div>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>

      {/* Global Toast */}
      <AnimatePresence>
        {toastMessage && (
          <motion.div
            initial={{ opacity: 0, y: 50, scale: 0.9 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, scale: 0.9, y: 20 }}
            className={`fixed bottom-6 right-6 z-50 p-4 rounded-xl border shadow-2xl flex items-center gap-3 backdrop-blur-md ${
              toastMessage.type === "error"
                ? "bg-red-500/10 border-red-500/30 text-red-400"
                : "bg-emerald-500/10 border-emerald-500/30 text-emerald-400"
            }`}
          >
            <span className="font-semibold text-sm">
              {toastMessage.type === "error" ? "Action Failed" : "Success"}
            </span>
            <div className="w-[1px] h-4 bg-current opacity-30" />
            <span className="text-sm text-white">{toastMessage.message}</span>
            <button
              onClick={() => setToastMessage(null)}
              className="ml-2 opacity-60 hover:opacity-100 cursor-pointer p-2 relative z-50 pointer-events-auto"
            >
              <X className="w-4 h-4" />
            </button>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

// Note: Gantt implementation using D3.
function RoadmapGanttChart({ filteredRoadmapData, setSelectedPhaseId }: any) {
  const svgRef = React.useRef<SVGSVGElement>(null);

  React.useEffect(() => {
    if (!svgRef.current || filteredRoadmapData.length === 0) return;

    const width = 1000;
    const itemHeight = 40;
    const height = filteredRoadmapData.length * itemHeight + 60;
    const margin = { top: 30, right: 30, bottom: 20, left: 200 };

    const svg = d3
      .select(svgRef.current)
      .attr("viewBox", `0 0 ${width} ${height}`)
      .style("width", "100%")
      .style("height", "auto");

    svg.selectAll("*").remove();

    // Generate dummy date ranges for phases based on their index
    const data = filteredRoadmapData.map((d: any, i: number) => {
      const start = new Date(2024, i * 2, 1);
      const end = new Date(start);
      end.setMonth(end.getMonth() + 3);
      return { ...d, start, end };
    });

    const x = d3
      .scaleTime()
      .domain([
        d3.min(data, (d: any) => d.start) as unknown as Date,
        d3.max(data, (d: any) => d.end) as unknown as Date,
      ])
      .range([margin.left, width - margin.right]);

    const y = d3
      .scaleBand()
      .domain(data.map((d: any) => d.title))
      .range([margin.top, height - margin.bottom])
      .padding(0.2);

    const xAxis = d3.axisTop(x).tickFormat(d3.timeFormat("%b %Y") as any);

    svg
      .append("g")
      .attr("transform", `translate(0,${margin.top})`)
      .call(xAxis)
      .selectAll("text")
      .attr("fill", "#64748b")
      .style("font-family", "monospace")
      .style("font-size", "12px");

    svg.selectAll(".domain, .tick line").attr("stroke", "#1e293b");

    svg
      .append("g")
      .attr("transform", `translate(${margin.left},0)`)
      .call(d3.axisLeft(y).tickSize(0))
      .selectAll("text")
      .attr("fill", "#cbd5e1")
      .style("font-size", "12px")
      .style("font-family", "sans-serif")
      .style("font-weight", "500");

    svg.select(".domain").remove();

    // Draw bar groups
    const barGroups = svg
      .selectAll(".bar-group")
      .data(data)
      .enter()
      .append("g")
      .attr("class", "bar-group cursor-pointer transition-all hover:opacity-80")
      .on("click", (event: any, d: any) => {
        setSelectedPhaseId(d.id);
      });

    barGroups
      .append("rect")
      .attr("class", "bar")
      .attr("x", (d: any) => x(d.start))
      .attr("y", (d: any) => y(d.title)!)
      .attr("width", (d: any) => x(d.end) - x(d.start))
      .attr("height", y.bandwidth())
      .attr("rx", 6)
      .attr("fill", (d: any) =>
        d.status === "Abgeschlossen"
          ? "#10b981"
          : d.status?.includes("Laufend")
            ? "#a259ff"
            : "#475569",
      );

    barGroups
      .append("rect")
      .attr("class", "bar-progress")
      .attr("x", (d: any) => x(d.start))
      .attr("y", (d: any) => y(d.title)! + y.bandwidth() - 4)
      .attr("width", (d: any) => {
        const comp = d.goals
          ? d.goals.filter((g: any) => g.completed).length
          : 0;
        const total = d.goals ? d.goals.length : 0;
        const prog = total > 0 ? comp / total : 0;
        return (x(d.end) - x(d.start)) * prog;
      })
      .attr("height", 4)
      .attr("fill", "#ffffff")
      .attr("opacity", 0.3)
      .style("border-bottom-left-radius", "6px")
      .style("border-bottom-right-radius", "6px"); // rx/ry would apply to all corners, but maybe just setting rx is enough? Or clip path... D3 rect rx makes rounded corners, so small height might look like a capsule. We can just use rect.

    // SVG standard way for rx ry is to apply it directly. It will round all 4.
    // We can just add rx=2 ry=2. Let's do that.
    barGroups.selectAll(".bar-progress").attr("rx", 2);

    barGroups
      .append("text")
      .attr("x", (d: any) => x(d.start) + 10)
      .attr("y", (d: any) => y(d.title)! + y.bandwidth() / 2 + 4)
      .attr("fill", "#ffffff")
      .style("font-size", "10px")
      .style("font-family", "monospace")
      .style("pointer-events", "none")
      .text((d: any) => {
        const comp = d.goals
          ? d.goals.filter((g: any) => g.completed).length
          : 0;
        const total = d.goals ? d.goals.length : 0;
        return total > 0 ? `${Math.round((comp / total) * 100)}%` : "0%";
      });

    barGroups
      .append("title")
      .text((d: any) => `${d.title}\nStatus: ${d.status}`);
  }, [filteredRoadmapData, setSelectedPhaseId]);

  return (
    <div className="w-full bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-x-auto min-h-[300px]">
      <svg
        ref={svgRef}
        className="w-full h-full min-w-[800px] overflow-visible"
      />
    </div>
  );
}

export function RoadmapView({
  initialData = INITIAL_ROADMAP_DATA,
  conflictItems,
  setConflictItems,
  showConflictModal,
  setShowConflictModal,
  syncHistory = [],
  setSyncHistory,
}: any) {
  const [roadmapData, setRoadmapData] = useState(initialData);
  const [selectedPhaseId, setSelectedPhaseId] = useState<string | null>(null);
  const selectedPhase =
    roadmapData.find((p) => p.id === selectedPhaseId) || null;
  const scrollRef = React.useRef<HTMLDivElement>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [startX, setStartX] = useState(0);
  const [scrollLeft, setScrollLeft] = useState(0);

  const [roadmapSearch, setRoadmapSearch] = useState("");
  const [roadmapFilter, setRoadmapFilter] = useState("Alle"); // 'Alle', 'Abgeschlossen', 'Laufend', 'Planned' (or 'Geplant')
  const [roadmapSort, setRoadmapSort] = useState("default");
  const [activeTab, setActiveTab] = useState("Details");

  // Prepare chart data and filtered list
  let filteredRoadmapData = roadmapData.filter((phase) => {
    const matchesSearch =
      phase.title.toLowerCase().includes(roadmapSearch.toLowerCase()) ||
      phase.goals?.some((g) =>
        g.text.toLowerCase().includes(roadmapSearch.toLowerCase()),
      );

    if (!matchesSearch) return false;

    if (roadmapFilter === "Alle") return true;
    if (roadmapFilter === "Abgeschlossen" && phase.status === "Abgeschlossen")
      return true;
    if (roadmapFilter === "Laufend" && phase.status?.includes("Laufend"))
      return true;
    if (
      roadmapFilter === "Planned" &&
      (!phase.status ||
        phase.status === "Geplant" ||
        (!phase.status.includes("Abgeschlossen") &&
          !phase.status.includes("Laufend")))
    )
      return true;

    return false;
  });

  if (roadmapSort !== "default") {
    filteredRoadmapData.sort((a, b) => {
      const getProgress = (phase: any) => {
        const totalGoals =
          phase.githubTotalIssues !== undefined
            ? phase.githubTotalIssues
            : phase.goals?.length || 0;
        const completedGoals =
          phase.githubClosedIssues !== undefined
            ? phase.githubClosedIssues
            : phase.goals?.filter((g: any) => g.completed).length || 0;
        return totalGoals > 0 ? (completedGoals / totalGoals) * 100 : 0;
      };
      return roadmapSort === "progress-desc"
        ? getProgress(b) - getProgress(a)
        : getProgress(a) - getProgress(b);
    });
  }

  const chartData = roadmapData.map((phase) => {
    const totalGoals =
      phase.githubTotalIssues !== undefined
        ? phase.githubTotalIssues
        : phase.goals?.length || 0;
    const completedGoals =
      phase.githubClosedIssues !== undefined
        ? phase.githubClosedIssues
        : phase.goals?.filter((g: any) => g.completed).length || 0;
    const progress =
      totalGoals > 0 ? Math.round((completedGoals / totalGoals) * 100) : 0;
    return {
      name: phase.timeframe || phase.title,
      fullTitle: phase.title,
      progress,
      status: phase.status || "Planned",
    };
  });

  const [isSyncing, setIsSyncing] = useState(false);
  const [githubRepo, setGithubRepo] = useState("A-TownChain/a-townchain-core");
  const [githubRoadmapRepo, setGithubRoadmapRepo] = useState(
    "A-TownChain/atc-lang",
  );
  const [syncStatus, setSyncStatus] = useState<string | null>(null);

  const [isNotionSyncing, setIsNotionSyncing] = useState(false);
  const [notionWorkspace, setNotionWorkspace] = useState(
    "A-TownChain/atc-lang-wiki",
  );
  const [notionSyncStatus, setNotionSyncStatus] = useState<string | null>(null);

  // Custom Sync Features
  const [toastMessage, setToastMessage] = useState<{
    message: string;
    type: "error" | "success";
    link?: { label: string; tab: string; settingsTab: string };
  } | null>(null);
  const [showSyncHistory, setShowSyncHistory] = useState(false);
  const [remoteHistory, setRemoteHistory] = useState<any[]>([]);
  const [loadingHistory, setLoadingHistory] = useState(false);
  const [expandedDiffs, setExpandedDiffs] = useState<string[]>([]);

  const hasSyncErrors =
    (conflictItems && conflictItems.length > 0) ||
    (syncHistory.length > 0 && !syncHistory[0].success);
  const isCurrentlySyncing = isSyncing || isNotionSyncing;
  const syncBadgeColor = hasSyncErrors
    ? "bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.6)]"
    : isCurrentlySyncing
      ? "bg-amber-400 animate-pulse"
      : "bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.3)]";

  useEffect(() => {
    if (showSyncHistory) {
      setLoadingHistory(true);
      fetch("/api/sync-history")
        .then((r) => r.json())
        .then((data) => {
          setRemoteHistory(data);
          setLoadingHistory(false);
        })
        .catch((err) => {
          console.error(err);
          setLoadingHistory(false);
        });
    }
  }, [showSyncHistory]);

  // Keep other variables, we're sharing conflict Items through window events or props! Wait, RoadmapView can dispatch an event or read from a store, but it's easier.
  // Actually, wait, let's just use window events for setting conflict items. We don't even need to move conflictItems out,
  // I can declare a SECOND conflictItems state in App that syncs with RoadmapView via events, OR just lift them up.
  // Let's lift them up by removing them from here.

  const [githubSyncStep, setGithubSyncStep] = useState<string | null>(null);
  const [notionSyncStep, setNotionSyncStep] = useState<string | null>(null);

  const [isAssessing, setIsAssessing] = useState(false);
  const [assessmentResult, setAssessmentResult] = useState<string | null>(null);

  const handleAssessMilestone = async () => {
    if (!selectedPhase) return;
    setIsAssessing(true);
    setAssessmentResult(null);
    try {
      const response = await fetch("/api/assess-milestone", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          title: selectedPhase.title,
          goals: selectedPhase.goals,
        }),
      });
      if (response.ok) {
        const data = await response.json();
        setAssessmentResult(data.assessment);
      } else {
        setAssessmentResult(
          "Failed to assess milestone. Make sure GenAI API is configured.",
        );
      }
    } catch {
      setAssessmentResult("Network error while assessing milestone.");
    } finally {
      setIsAssessing(false);
    }
  };

  const downloadMilestoneData = () => {
    if (!selectedPhase) return;

    // Extract milestone data
    const totalGoals =
      selectedPhase.githubTotalIssues !== undefined
        ? selectedPhase.githubTotalIssues
        : selectedPhase.goals?.length || 0;
    const completedGoals =
      selectedPhase.githubClosedIssues !== undefined
        ? selectedPhase.githubClosedIssues
        : selectedPhase.goals?.filter((g: any) => g.completed).length || 0;
    const progress =
      totalGoals > 0 ? Math.round((completedGoals / totalGoals) * 100) : 0;

    const dataToDownload = {
      title: selectedPhase.title,
      timeframe: selectedPhase.timeframe,
      status: selectedPhase.status,
      progress: `${progress}% (${completedGoals}/${totalGoals})`,
      goals:
        selectedPhase.goals?.map((g) => ({
          text: g.text,
          completed: g.completed,
          notes: g.notes || "",
          tags: g.tags || [],
        })) || [],
    };

    const blob = new Blob([JSON.stringify(dataToDownload, null, 2)], {
      type: "application/json",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `milestone-${selectedPhase.title.replace(/\s+/g, "-").toLowerCase()}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  // Replace setGithubSyncStep inside fetchGithubProgress
  const downloadPdfRoadmap = () => {
    const doc = new jsPDF();
    doc.setFont("helvetica", "bold");
    doc.setFontSize(20);
    doc.text("Roadmap Summary", 20, 20);

    doc.setFontSize(12);
    doc.setFont("helvetica", "normal");
    let yPos = 40;

    roadmapData.forEach((phase) => {
      if (yPos > 270) {
        doc.addPage();
        yPos = 20;
      }

      const totalGoals =
        phase.githubTotalIssues !== undefined
          ? phase.githubTotalIssues
          : phase.goals?.length || 0;
      const completedGoals =
        phase.githubClosedIssues !== undefined
          ? phase.githubClosedIssues
          : phase.goals?.filter((g: any) => g.completed).length || 0;
      const progress =
        totalGoals > 0 ? Math.round((completedGoals / totalGoals) * 100) : 0;

      doc.setFont("helvetica", "bold");
      doc.text(`${phase.title} (${phase.status})`, 20, yPos);
      yPos += 7;

      doc.setFont("helvetica", "normal");
      doc.text(`Timeframe: ${phase.timeframe}`, 20, yPos);
      yPos += 7;

      doc.text(
        `Progress: ${progress}% (${completedGoals} / ${totalGoals} tasks)`,
        20,
        yPos,
      );
      yPos += 15;
    });

    doc.save("roadmap_summary.pdf");
  };

  const setGithubSyncStepEvent = (step: string | null) => {
    setGithubSyncStep(step);
    window.dispatchEvent(
      new CustomEvent("sync-step", { detail: { target: "github", step } }),
    );
  };

  const fetchGithubProgress = async (isAuto = false) => {
    setIsSyncing(true);
    setGithubSyncStepEvent("Starting GitHub sync...");
    if (!isAuto) {
      setSyncStatus("Fetching milestones...");
    }

    let outcomeStr = "";
    let isSuccess = false;

    try {
      setGithubSyncStepEvent("Fetching GitHub Dev Repo...");
      let milestones: any[] = [];
      let roadmapMilestones: any[] = [];

      // DATA for A-TownChain private/enterprise repos
      if (githubRepo.startsWith("A-TownChain/")) {
        await new Promise((r) => setTimeout(r, 800));
        setGithubSyncStepEvent("Parsing GitHub issues...");
        await new Promise((r) => setTimeout(r, 600));
        milestones = [
          {
            title: "Syntax & Grammatik (Lexer/Parser)",
            open_issues: 0,
            closed_issues: 12,
          },
          {
            title: "Type Checker & Borrow Semantics",
            open_issues: 2,
            closed_issues: 14,
          },
          {
            title: "Backend: ATVM Code Generation",
            open_issues: 5,
            closed_issues: 8,
          },
        ];
      }

      setGithubSyncStepEvent("Fetching roadmap repo metadata...");
      if (githubRoadmapRepo && githubRoadmapRepo.startsWith("A-TownChain/")) {
        await new Promise((r) => setTimeout(r, 800));
        setGithubSyncStepEvent("Parsing GitHub milestones...");
        await new Promise((r) => setTimeout(r, 600));
        roadmapMilestones = [
          {
            title: "Formale Verifikation & SMT Solver",
            open_issues: 8,
            closed_issues: 1,
          },
          {
            title: "Standardbibliothek & Crypto-Precompiles",
            open_issues: 4,
            closed_issues: 2,
          },
        ];
      }

      setGithubSyncStepEvent("Merging definitions...");
      if (milestones.length > 0 || roadmapMilestones.length > 0) {
        const updatedRoadmap = [...roadmapData];
        let matchedSections = 0;
        let newAdded = 0;

        // Combine all unique milestone titles
        const allMilestonesMap = new globalThis.Map<string, any>();

        milestones.forEach((m: any) => {
          allMilestonesMap.set(m.title.toLowerCase(), { ...m, fromDev: true });
        });

        roadmapMilestones.forEach((rm: any) => {
          const key = rm.title.toLowerCase();
          if (allMilestonesMap.has(key)) {
            const existing = allMilestonesMap.get(key);
            existing.open_issues += rm.open_issues;
            existing.closed_issues += rm.closed_issues;
            existing.fromRoadmap = true;
          } else {
            allMilestonesMap.set(key, { ...rm, fromRoadmap: true });
          }
        });

        const combinedMilestones = Array.from(allMilestonesMap.values());

        setGithubSyncStepEvent("Applying updates...");
        // Update existing roadmap phases or add new ones
        combinedMilestones.forEach((m: any) => {
          const exists = updatedRoadmap.find(
            (phase) =>
              m.title.toLowerCase().includes(phase.title.toLowerCase()) ||
              phase.title.toLowerCase().includes(m.title.toLowerCase()) ||
              (phase.timeframe &&
                m.title.toLowerCase().includes(phase.timeframe.toLowerCase())),
          );

          if (!exists) {
            updatedRoadmap.push({
              id: `github-${m.id}`,
              title: m.title,
              timeframe: m.due_on
                ? new Date(m.due_on).toLocaleDateString()
                : "",
              status: m.state === "closed" ? "Abgeschlossen" : "Geplant",
              goals: [
                {
                  text: m.description || "Milestone added from GitHub sync.",
                  completed: m.state === "closed",
                },
              ],
              githubTotalIssues: m.open_issues + m.closed_issues,
              githubClosedIssues: m.closed_issues,
            });
            newAdded++;
          } else {
            exists.githubTotalIssues = m.open_issues + m.closed_issues;
            exists.githubClosedIssues = m.closed_issues;
            matchedSections++;
          }
        });

        setRoadmapData(updatedRoadmap);
        if (matchedSections > 0 || newAdded > 0) {
          outcomeStr = `Synced ${matchedSections} milestones. Added ${newAdded} new.`;
        } else {
          outcomeStr = `No matching milestones found.`;
        }
        setSyncStatus(outcomeStr);
        isSuccess = true;
      } else {
        outcomeStr = "Failed to fetch repos.";
        setSyncStatus(outcomeStr);
      }
    } catch (error) {
      console.error(error);
      outcomeStr = "Network error.";
      setSyncStatus(outcomeStr);
      if (isAuto) {
        setToastMessage({
          message: "Auto-sync failed. Check connection.",
          type: "error",
        });
      }
    } finally {
      if (!isSuccess) {
        window.dispatchEvent(
          new CustomEvent("add-offline-queue", {
            detail: {
              target: "GitHub",
              type: isAuto ? "Auto" : "Manual",
              timestamp: Date.now(),
            },
          }),
        );
      }
      setSyncHistory((prev) =>
        [
          {
            timestamp: new Date(),
            type: isAuto ? "Auto" : "Manual",
            target: "GitHub",
            outcome: outcomeStr,
            success: isSuccess,
          },
          ...prev,
        ].slice(0, 10),
      ); // keep last 10
      setIsSyncing(false);
      setGithubSyncStepEvent(null);
    }
  };

  const setNotionSyncStepEvent = (step: string | null) => {
    setNotionSyncStep(step);
    window.dispatchEvent(
      new CustomEvent("sync-step", { detail: { target: "notion", step } }),
    );
  };

  const fetchNotionProgress = async (isAuto = false) => {
    setIsNotionSyncing(true);
    setNotionSyncStepEvent("Connecting to Notion API...");
    if (!isAuto) setNotionSyncStatus("Authenticating...");

    let outcomeStr = "";
    let isSuccess = false;

    try {
      // Simulate authenticating
      await new Promise((r) => setTimeout(r, 600));
      setNotionSyncStepEvent("Fetching Notion metadata...");
      await new Promise((r) => setTimeout(r, 600));

      // Simulate Conflict Detection
      setNotionSyncStepEvent("Checking for conflicts...");
      await new Promise((r) => setTimeout(r, 600));

      // Just for demonstration, we flag the first roadmap item as conflicted if it hasn't been flagged yet
      if (roadmapData.length > 0 && conflictItems.length === 0) {
        setConflictItems([
          {
            id: roadmapData[0].id,
            title: roadmapData[0].title,
            githubSource: {
              status: roadmapData[0].status,
              goals: roadmapData[0].goals,
            },
            notionSource: {
              status: "Abgeschlossen",
              goals: [
                ...roadmapData[0].goals,
                { text: "Extra Notion text", completed: true },
              ],
            },
          },
        ]);
        setShowConflictModal(true);
      }

      setNotionSyncStepEvent("Syncing documents...");
      await new Promise((r) => setTimeout(r, 600));

      outcomeStr = "Synced 34 documents.";
      setNotionSyncStatus(outcomeStr);
      isSuccess = true;
    } catch (error) {
      console.error(error);
      outcomeStr = "Failed to fetch workspace.";
      setNotionSyncStatus(outcomeStr);
      if (isAuto) {
        setToastMessage({
          message: "Notion background sync failed.",
          type: "error",
        });
      }
    } finally {
      if (!isSuccess) {
        window.dispatchEvent(
          new CustomEvent("add-offline-queue", {
            detail: {
              target: "Notion",
              type: isAuto ? "Auto" : "Manual",
              timestamp: Date.now(),
            },
          }),
        );
      }
      setSyncHistory((prev) =>
        [
          {
            timestamp: new Date(),
            type: isAuto ? "Auto" : "Manual",
            target: "Notion",
            outcome: outcomeStr,
            success: isSuccess,
          },
          ...prev,
        ].slice(0, 10),
      );

      setIsNotionSyncing(false);
      setNotionSyncStepEvent(null);
      setTimeout(() => setNotionSyncStatus(null), 3000);
    }
  };

  // Background auto-sync every 5 minutes
  useEffect(() => {
    const interval = setInterval(
      () => {
        fetchGithubProgress(true);
        fetchNotionProgress(true);
      },
      5 * 60 * 1000,
    ); // 5 minutes

    const githubListener = () => fetchGithubProgress(false);
    const notionListener = () => fetchNotionProgress(false);

    window.addEventListener("trigger-github-sync", githubListener);
    window.addEventListener("trigger-notion-sync", notionListener);

    return () => {
      clearInterval(interval);
      window.removeEventListener("trigger-github-sync", githubListener);
      window.removeEventListener("trigger-notion-sync", notionListener);
    };
  }, [githubRepo, githubRoadmapRepo, roadmapData]);

  useEffect(() => {
    if (selectedPhase) {
      document.body.style.overflow = "hidden";
    } else {
      document.body.style.overflow = "unset";
    }
    return () => {
      document.body.style.overflow = "unset";
    };
  }, [selectedPhase]);

  const onMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    if (scrollRef.current) {
      scrollRef.current.classList.add("cursor-grabbing");
      scrollRef.current.classList.remove("cursor-grab");
      setStartX(e.pageX - scrollRef.current.offsetLeft);
      setScrollLeft(scrollRef.current.scrollLeft);
    }
  };

  const onMouseUpOrLeave = () => {
    setIsDragging(false);
    if (scrollRef.current) {
      scrollRef.current.classList.add("cursor-grab");
      scrollRef.current.classList.remove("cursor-grabbing");
    }
  };

  const onMouseMove = (e: React.MouseEvent) => {
    if (!isDragging) return;
    e.preventDefault();
    if (scrollRef.current) {
      const x = e.pageX - scrollRef.current.offsetLeft;
      const walk = (x - startX) * 2;
      scrollRef.current.scrollLeft = scrollLeft - walk;
    }
  };

  return (
    <div className="flex flex-col gap-6 mt-8 pb-12 w-full">
      {/* Roadmap Summary Chart */}
      <div className="w-full h-48 mb-6 mt-4 w-full max-w-[1400px]">
        <div className="bg-[#090b14] border border-atc-border/50 rounded-xl p-4 h-full shadow-[0_0_15px_rgba(0,0,0,0.5)]">
          <ResponsiveContainer width="100%" height="100%">
            <BarChart
              data={chartData}
              margin={{ top: 10, right: 30, left: 0, bottom: 0 }}
            >
              <CartesianGrid
                strokeDasharray="3 3"
                stroke="#2a3553"
                vertical={false}
              />
              <XAxis
                dataKey="name"
                stroke="#64748b"
                fontSize={10}
                tickLine={false}
                axisLine={false}
              />
              <YAxis
                stroke="#64748b"
                fontSize={10}
                tickLine={false}
                axisLine={false}
                tickFormatter={(val) => `${val}%`}
              />
              <Tooltip
                cursor={{ fill: "rgba(162, 89, 255, 0.1)" }}
                contentStyle={{
                  backgroundColor: "#0f172a",
                  borderColor: "#2a3553",
                  borderRadius: "12px",
                }}
                labelStyle={{
                  color: "#00d1ff",
                  fontWeight: "bold",
                  marginBottom: "8px",
                }}
                formatter={(value: any, name: any, props: any) => {
                  return [`${value}% Finished`, props.payload.status];
                }}
                labelFormatter={(name) => `Phase: ${name}`}
              />
              <Bar dataKey="progress" radius={[4, 4, 0, 0]}>
                {chartData.map((entry, index) => (
                  <Cell
                    key={`cell-${index}`}
                    fill={
                      entry.status === "Abgeschlossen"
                        ? "#10b981"
                        : entry.status?.includes("Laufend")
                          ? "#a259ff"
                          : "#475569"
                    }
                  />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      <div className="flex flex-col md:flex-row md:justify-between items-start md:items-end gap-6 mb-12 relative z-10 w-full max-w-[1400px]">
        <div className="flex flex-col gap-6">
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
              <Map className="w-6 h-6" />
            </div>
            <div>
              <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
                Vision & Milestone Masterplan
              </h2>
              <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
                Strategische Roadmap, Zwischenziele, Vision und Phasenplanung
                für das A-TownChain Ökosystem. Drag to scroll through the
                timeline.
              </p>
            </div>
          </div>

          <div className="flex flex-wrap items-center gap-3">
            <div className="flex items-center gap-2 bg-[#090b14] border border-atc-border/50 rounded-xl px-3 py-1.5 shadow-[0_0_15px_rgba(0,0,0,0.5)]">
              <Search className="w-4 h-4 text-slate-400" />
              <input
                type="text"
                placeholder="Search milestones..."
                value={roadmapSearch}
                onChange={(e) => setRoadmapSearch(e.target.value)}
                className="bg-transparent border-none text-sm text-slate-300 w-48 focus:outline-none focus:ring-0 placeholder-slate-600 outline-none ring-0"
              />
            </div>
            <div className="flex items-center gap-1 bg-[#090b14] border border-atc-border/50 rounded-xl p-1 shadow-[0_0_15px_rgba(0,0,0,0.5)]">
              {["Alle", "Abgeschlossen", "Laufend", "Planned"].map((opt) => (
                <button
                  key={opt}
                  onClick={() => setRoadmapFilter(opt)}
                  className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors ${roadmapFilter === opt ? "bg-atc-purple text-white shadow-md" : "text-slate-400 hover:text-white hover:bg-transparent"}`}
                >
                  {opt}
                </button>
              ))}
            </div>
            <div className="flex items-center gap-2 bg-[#090b14] border border-atc-border/50 rounded-xl px-3 py-1.5 shadow-[0_0_15px_rgba(0,0,0,0.5)]">
              <ArrowUpDown className="w-4 h-4 text-slate-400" />
              <select
                value={roadmapSort}
                onChange={(e) => setRoadmapSort(e.target.value)}
                className="bg-transparent border-none text-xs font-semibold text-slate-300 focus:outline-none focus:ring-0 outline-none ring-0 appearance-none cursor-pointer"
              >
                <option value="default" className="bg-[#090b14]">
                  Default Order
                </option>
                <option value="progress-desc" className="bg-[#090b14]">
                  Progress: High to Low
                </option>
                <option value="progress-asc" className="bg-[#090b14]">
                  Progress: Low to High
                </option>
              </select>
            </div>
          </div>
        </div>
        <div className="hidden md:flex flex-col items-end gap-1">
          <div className="flex items-center gap-2">
            <div className="flex items-center gap-2 mr-2 bg-[#090b14] border border-atc-border/50 rounded-xl p-1 shadow-[0_0_15px_rgba(0,0,0,0.5)]">
              <Github className="w-4 h-4 text-slate-400 ml-2" />
              <input
                type="text"
                value={githubRepo}
                onChange={(e) => setGithubRepo(e.target.value)}
                className="bg-transparent border-none text-xs text-slate-300 w-32 focus:outline-none focus:ring-0 placeholder-slate-600 outline-none ring-0"
                placeholder="Dev Repo"
                title="Development Repository"
              />
              <div className="w-px h-4 bg-slate-700 mx-1 border-slate-700 border-l" />
              <input
                type="text"
                value={githubRoadmapRepo}
                onChange={(e) => setGithubRoadmapRepo(e.target.value)}
                className="bg-transparent border-none text-xs text-slate-300 w-32 focus:outline-none focus:ring-0 placeholder-slate-600 outline-none ring-0"
                placeholder="Roadmap Repo"
                title="Roadmap Repository"
              />
              <button
                onClick={() => fetchGithubProgress(false)}
                disabled={isSyncing}
                className="p-1 rounded-lg bg-atc-purple/10 text-atc-purple hover:bg-atc-purple/20 transition-colors disabled:opacity-50 flex items-center gap-1 px-2 relative group/badge"
              >
                <div
                  className={`absolute -top-1 -right-1 w-2.5 h-2.5 rounded-full border border-atc-bg transition-colors duration-500 z-10 ${syncBadgeColor}`}
                />
                <RefreshCw
                  className={`w-3 h-3 relative z-10 ${isSyncing ? "animate-spin" : ""}`}
                />
                <span className="text-[10px] font-bold tracking-wider uppercase relative z-10">
                  Sync
                </span>
                <div className="absolute top-full left-1/2 -translate-x-1/2 mt-2 px-2 py-1.5 bg-[#090b14] border border-white/10 rounded shadow-xl text-[10px] font-mono text-slate-300 whitespace-nowrap opacity-0 invisible group-hover/badge:opacity-100 group-hover/badge:visible transition-all z-[100] text-center leading-tight">
                  <span className="text-emerald-400 font-bold mb-0.5 block">
                    SYNC COMMAND
                  </span>
                </div>
              </button>
              <button
                onClick={() => setShowSyncHistory(true)}
                className="p-1.5 rounded-lg text-slate-400 hover:bg-atc-card-hover hover:text-white transition-colors ml-1"
                title="View Sync History"
              >
                <History className="w-3.5 h-3.5" />
              </button>
            </div>
            {syncStatus && (
              <span className="text-[10px] text-slate-500 font-mono text-right mr-2 h-4">
                {syncStatus}
              </span>
            )}
            <div className="flex gap-2">
              <button
                onClick={downloadPdfRoadmap}
                className="p-2.5 bg-[#090b14] border border-atc-border/50 rounded-xl shadow-[0_0_15px_rgba(0,0,0,0.5)] hover:border-slate-500 text-slate-500 hover:text-white transition-colors flex items-center gap-2"
                title="Download Roadmap PDF"
              >
                <ArrowDownToLine className="w-5 h-5" />
                <span className="text-xs font-semibold hidden md:inline">
                  Download Roadmap
                </span>
              </button>
              <button
                onClick={() =>
                  scrollRef.current?.scrollBy({
                    left: -400,
                    behavior: "smooth",
                  })
                }
                className="p-2.5 bg-[#090b14] border border-atc-border/50 rounded-xl shadow-[0_0_15px_rgba(0,0,0,0.5)] hover:border-slate-500 text-slate-500 hover:text-white transition-colors"
              >
                <ChevronLeft className="w-5 h-5" />
              </button>
              <button
                onClick={() =>
                  scrollRef.current?.scrollBy({ left: 400, behavior: "smooth" })
                }
                className="p-2.5 bg-[#090b14] border border-atc-border/50 rounded-xl shadow-[0_0_15px_rgba(0,0,0,0.5)] hover:border-slate-500 text-slate-500 hover:text-white transition-colors"
              >
                <ChevronRight className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <RoadmapGanttChart
        filteredRoadmapData={filteredRoadmapData}
        setSelectedPhaseId={setSelectedPhaseId}
      />

      {/* End State Summary Banner */}
      <div className="mt-4 p-6 md:p-8 rounded-2xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl relative overflow-hidden group">
        <div className="absolute top-0 right-0 w-64 h-64 bg-atc-cyan/5 rounded-full blur-[100px] pointer-events-none group-hover:bg-atc-cyan/10 transition-opacity" />
        <div className="flex flex-col sm:flex-row gap-6 justify-between items-start sm:items-center relative z-10">
          <div>
            <h3 className="text-lg font-medium text-white mb-2">Endzustand</h3>
            <p className="text-sm text-slate-500 max-w-2xl leading-relaxed">
              A-TownChain + ATC-OS + ATS: Vollständig verifizierte, KI-native,
              selbstverwaltende Infrastrukturplattform mit autonomer digitaler
              Gesellschaft.
            </p>
            <div className="flex flex-wrap gap-2 mt-4">
              {[
                "Blockchain",
                "ATC-OS",
                "Agenten-Netzwerk",
                "DAO-Governance",
                "Compute-Markt",
                "Cross-Chain-Netzwerk",
                "Enterprise-Plattform",
                "Verifizierte Sicherheit",
                "Selbstheilende Infrastruktur",
                "Autonome Gesellschaft",
              ].map((tag, i) => (
                <span
                  key={i}
                  className="text-xs bg-atc-purple/10 text-indigo-300 px-2 py-1 rounded border border-atc-purple/30"
                >
                  {tag}
                </span>
              ))}
            </div>
          </div>
          <div className="flex flex-col gap-3 shrink-0 text-right">
            <div>
              <div className="text-atc-purple font-mono text-lg font-bold">
                10–15 Jahre
              </div>
              <div className="text-[10px] text-slate-500 font-mono tracking-widest uppercase">
                Entwicklungszeitraum
              </div>
            </div>
            <div>
              <div className="text-atc-cyan font-mono text-lg font-bold">
                500+
              </div>
              <div className="text-[10px] text-slate-500 font-mono tracking-widest uppercase">
                Kernkomponenten
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Detail Modal */}
      <AnimatePresence>
        {selectedPhase && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="absolute inset-0 bg-[#060a16]/40 backdrop-blur-sm"
              onClick={() => setSelectedPhaseId(null)}
            />
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 10 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 10 }}
              className="relative w-full max-w-4xl max-h-[90vh] bg-atc-card rounded-2xl shadow-xl overflow-hidden flex flex-col border border-atc-border"
            >
              <div className="flex items-start md:items-center justify-between p-6 border-b border-atc-purple/20 bg-atc-purple/10">
                <div className="flex flex-col md:flex-row md:items-center gap-4">
                  <div className="w-12 h-12 rounded-xl bg-atc-card border border-atc-purple/20 flex items-center justify-center text-atc-purple shadow-[0_0_15px_rgba(0,0,0,0.5)] shrink-0">
                    <Map className="w-6 h-6" />
                  </div>
                  <div>
                    <div className="flex items-center gap-3 mb-1">
                      {selectedPhase.timeframe && (
                        <span className="text-atc-purple font-mono text-xs tracking-wider font-semibold">
                          {selectedPhase.timeframe}
                        </span>
                      )}
                      {selectedPhase.status && (
                        <span className="px-2 py-0.5 rounded-md bg-atc-card text-slate-300 text-[10px] uppercase tracking-wider font-mono border border-atc-border">
                          {selectedPhase.status}
                        </span>
                      )}
                    </div>
                    <h2 className="text-2xl text-white font-medium leading-tight">
                      {selectedPhase.title}
                    </h2>
                  </div>
                </div>
                <button
                  onClick={() => setSelectedPhaseId(null)}
                  className="p-2 -mr-2 -mt-2 text-slate-500 hover:text-slate-500 hover:bg-atc-card rounded-lg transition-colors border border-transparent hover:border-atc-border shrink-0"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>

              <div className="flex border-b border-atc-border bg-atc-card px-6 shrink-0">
                <button
                  onClick={() => setActiveTab("Details")}
                  className={`px-4 py-3 font-medium text-sm font-mono tracking-wider border-b-2 transition-colors ${activeTab === "Details" ? "border-atc-cyan text-atc-cyan" : "border-transparent text-slate-400 hover:text-slate-200"}`}
                >
                  DETAILS
                </button>
                <button
                  onClick={() => setActiveTab("History")}
                  className={`px-4 py-3 font-medium text-sm font-mono tracking-wider border-b-2 transition-colors ${activeTab === "History" ? "border-atc-cyan text-atc-cyan" : "border-transparent text-slate-400 hover:text-slate-200"}`}
                >
                  HISTORY
                </button>
              </div>

              <div className="p-6 overflow-y-auto custom-scrollbar flex-1 bg-atc-bg/60">
                {activeTab === "Details" ? (
                  <>
                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                      {selectedPhase.sections.map(
                        (section: any, idx: number) => (
                          <div
                            key={idx}
                            className="p-5 rounded-xl bg-atc-card border border-atc-border shadow-[0_0_15px_rgba(0,0,0,0.5)] transition-all hover:shadow-[0_0_25px_rgba(0,209,255,0.05)] hover:border-atc-purple/30"
                          >
                            <h3 className="text-sm font-mono text-atc-cyan uppercase tracking-wider mb-4 border-b border-atc-border pb-2">
                              {section.title}
                            </h3>
                            <ul className="flex flex-col gap-3">
                              {section.items.map((item: string, i: number) => (
                                <li
                                  key={i}
                                  className="flex items-start gap-3 group"
                                >
                                  <div className="mt-1 p-0.5 rounded-full bg-atc-purple/10 border border-atc-purple/30 flex-shrink-0 transition-colors">
                                    <div className="w-1.5 h-1.5 rounded-full bg-atc-purple" />
                                  </div>
                                  <span className="text-sm text-slate-300 leading-snug font-medium">
                                    {item}
                                  </span>
                                </li>
                              ))}
                            </ul>
                          </div>
                        ),
                      )}
                    </div>

                    {selectedPhase.goals && selectedPhase.goals.length > 0 && (
                      <div className="mt-6">
                        <div className="p-5 rounded-xl bg-atc-purple/10 border border-atc-purple/20 flex flex-col gap-3">
                          <span className="text-xs font-mono text-atc-purple uppercase tracking-widest font-semibold">
                            Milestone / Goals
                          </span>
                          <div className="flex flex-col gap-2">
                            {selectedPhase.goals.map((goal: any, i: number) => (
                              <div
                                key={i}
                                className={`flex flex-col gap-2 bg-atc-card px-4 py-3 rounded-lg border shadow-[0_0_15px_rgba(0,0,0,0.5)] transition-all ${goal.completed ? "border-emerald-500/20 bg-emerald-500/10" : "border-atc-purple/20"}`}
                              >
                                <button
                                  onClick={() => {
                                    const phaseIndex = roadmapData.findIndex(
                                      (p) => p.id === selectedPhase.id,
                                    );
                                    if (phaseIndex === -1) return;

                                    const newData = [...roadmapData];
                                    const newPhase = { ...newData[phaseIndex] };
                                    const newGoals = [...newPhase.goals];
                                    const wasCompleted = newGoals.filter(
                                      (g) => g.completed,
                                    ).length;
                                    const total = newGoals.length;

                                    newGoals[i] = {
                                      ...newGoals[i],
                                      completed: !newGoals[i].completed,
                                    };
                                    newPhase.goals = newGoals;
                                    newData[phaseIndex] = newPhase;

                                    setRoadmapData(newData);

                                    const isNowCompleted = newGoals.filter(
                                      (g) => g.completed,
                                    ).length;
                                    if (
                                      wasCompleted !== total &&
                                      isNowCompleted === total
                                    ) {
                                      confetti({
                                        particleCount: 150,
                                        spread: 70,
                                        origin: { y: 0.6 },
                                        colors: [
                                          "#4f46e5",
                                          "#10b981",
                                          "#f43f5e",
                                          "#f59e0b",
                                          "#3b82f6",
                                        ],
                                      });
                                    }
                                  }}
                                  className="flex items-start text-left w-full gap-3 focus:outline-none"
                                >
                                  {goal.completed ? (
                                    <CheckCircle className="w-5 h-5 text-emerald-500 shrink-0 mt-0.5" />
                                  ) : !goal.completed &&
                                    (selectedPhase.status === "Abgeschlossen" ||
                                      selectedPhase.status?.includes(
                                        "Laufend",
                                      )) ? (
                                    <AlertTriangle className="w-5 h-5 text-amber-500 shrink-0 mt-0.5" />
                                  ) : (
                                    <div className="w-4 h-4 rounded-full border-2 border-atc-purple/30 shrink-0 mx-0.5 mt-1" />
                                  )}
                                  <div className="flex flex-col items-start w-full">
                                    <span
                                      className={`text-sm leading-tight ${goal.completed ? "text-emerald-300 font-medium" : !goal.completed && (selectedPhase.status === "Abgeschlossen" || selectedPhase.status?.includes("Laufend")) ? "text-amber-400 font-medium" : "text-slate-100 font-medium"}`}
                                    >
                                      {goal.text}
                                    </span>
                                    {goal.tags && goal.tags.length > 0 && (
                                      <div className="flex flex-wrap gap-2 mt-2">
                                        {goal.tags.map(
                                          (tag: string, tIdx: number) => (
                                            <span
                                              key={tIdx}
                                              className={`text-[10px] uppercase font-mono tracking-wider px-2 py-0.5 rounded-full border ${goal.completed ? "bg-emerald-500/20 text-emerald-400 border-emerald-500/40" : !goal.completed && (selectedPhase.status === "Abgeschlossen" || selectedPhase.status?.includes("Laufend")) ? "bg-amber-500/20 text-amber-400 border-amber-500/40" : "bg-atc-purple/20 text-atc-purple border-atc-purple/40"}`}
                                            >
                                              {tag}
                                            </span>
                                          ),
                                        )}
                                      </div>
                                    )}
                                  </div>
                                </button>
                                <textarea
                                  value={goal.notes || ""}
                                  onChange={(e) => {
                                    const phaseIndex = roadmapData.findIndex(
                                      (p) => p.id === selectedPhase.id,
                                    );
                                    if (phaseIndex === -1) return;

                                    const newData = [...roadmapData];
                                    const newPhase = { ...newData[phaseIndex] };
                                    const newGoals = [...newPhase.goals];

                                    newGoals[i] = {
                                      ...newGoals[i],
                                      notes: e.target.value,
                                    };
                                    newPhase.goals = newGoals;
                                    newData[phaseIndex] = newPhase;

                                    setRoadmapData(newData);
                                  }}
                                  placeholder="Add personal notes or observations for this milestone..."
                                  className={`mt-2 w-full text-sm p-3 rounded-md border focus:ring-2 focus:outline-none resize-none min-h-[80px] ${goal.completed ? "border-emerald-500/30 bg-emerald-500/10 text-emerald-300 focus:border-emerald-400 focus:ring-emerald-400/20 placeholder:text-emerald-400/40" : "border-atc-purple/20 bg-atc-bg/50 text-slate-300 focus:border-indigo-400 focus:ring-indigo-400/20 placeholder:text-slate-500"}`}
                                />
                              </div>
                            ))}
                          </div>
                        </div>
                      </div>
                    )}
                    {assessmentResult && (
                      <div className="mt-6 p-5 rounded-xl bg-atc-cyan/10 border border-atc-cyan/30 flex flex-col gap-3">
                        <div className="flex items-center gap-2 text-atc-cyan font-mono text-xs uppercase tracking-widest font-semibold pb-2 border-b border-atc-cyan/20">
                          <Sparkles className="w-4 h-4" />
                          AI Assessment (Endzustand Goals)
                        </div>
                        <p className="text-sm text-slate-300 leading-relaxed font-medium">
                          {assessmentResult}
                        </p>
                      </div>
                    )}
                  </>
                ) : (
                  <div className="flex flex-col gap-4">
                    {selectedPhase.history &&
                    selectedPhase.history.length > 0 ? (
                      selectedPhase.history.map((entry: any, hIdx: number) => (
                        <div
                          key={hIdx}
                          className="flex gap-4 items-start p-4 rounded-xl bg-atc-card border border-atc-border"
                        >
                          <div className="p-2 rounded-lg bg-atc-purple/10 border border-atc-purple/30">
                            <History className="w-4 h-4 text-atc-purple" />
                          </div>
                          <div>
                            <p className="text-sm font-semibold text-white">
                              {entry.status}
                            </p>
                            <p className="text-xs text-slate-500 font-mono mt-1">
                              {entry.timestamp}
                            </p>
                          </div>
                        </div>
                      ))
                    ) : (
                      <div className="p-8 text-center border focus:outline-none border-dashed border-atc-border rounded-xl">
                        <History className="w-8 h-8 text-slate-600 mx-auto mb-3" />
                        <p className="text-sm text-slate-400">
                          No history available for this milestone.
                        </p>
                      </div>
                    )}
                  </div>
                )}
              </div>

              <div className="p-4 border-t border-atc-border bg-atc-card flex justify-between items-center gap-3">
                <div className="flex gap-3">
                  <button
                    onClick={handleAssessMilestone}
                    disabled={isAssessing}
                    className="flex items-center gap-2 px-5 py-2 min-w-24 bg-atc-purple/10 border border-atc-purple/30 text-atc-purple rounded-lg font-medium hover:bg-atc-purple/20 transition-colors shadow-[0_0_15px_rgba(162,89,255,0.2)] disabled:opacity-50"
                  >
                    {isAssessing ? (
                      <RefreshCw className="w-4 h-4 animate-spin" />
                    ) : (
                      <BrainCircuit className="w-4 h-4" />
                    )}
                    Assess Impact
                  </button>
                  <button
                    onClick={downloadMilestoneData}
                    className="flex items-center gap-2 px-5 py-2 bg-[#090b14] border border-atc-border/50 text-slate-300 rounded-lg font-medium hover:text-white hover:border-slate-500 transition-colors shadow-[0_0_15px_rgba(0,0,0,0.5)]"
                  >
                    <Download className="w-4 h-4" />
                    Download Data
                  </button>
                </div>
                <button
                  onClick={() => setSelectedPhaseId(null)}
                  className="px-5 py-2 min-w-24 bg-atc-card border border-atc-border-hover rounded-lg text-slate-300 font-medium hover:bg-atc-bg transition-colors shadow-[0_0_15px_rgba(0,0,0,0.5)]"
                >
                  Close
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>

      {/* Toast Notification */}
      {toastMessage && (
        <div
          className={`fixed bottom-6 right-6 z-[200] p-4 rounded-xl shadow-2xl border flex items-center gap-4 ${toastMessage.type === "error" ? "bg-red-950/90 border-red-900/50 text-red-200" : "bg-emerald-950/90 border-emerald-900/50 text-emerald-200"} backdrop-blur-md animate-in slide-in-from-bottom-5 fade-in duration-200`}
        >
          <div className="flex flex-col gap-1">
            <span className="font-semibold text-sm">
              {toastMessage.type === "error" ? "Sync Failed" : "Success"}
            </span>
            <span className="text-xs opacity-90">{toastMessage.message}</span>
          </div>
          {toastMessage.type === "error" && (
            <button
              onClick={() => {
                setToastMessage(null);
                fetchGithubProgress(false);
              }}
              className="px-3 py-1.5 bg-white/10 hover:bg-white/20 rounded-md text-xs font-medium transition-colors cursor-pointer z-50 relative pointer-events-auto"
            >
              Retry
            </button>
          )}
          <button
            onClick={() => setToastMessage(null)}
            className="ml-2 opacity-60 hover:opacity-100 cursor-pointer p-2 relative z-50 pointer-events-auto"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      )}

      {/* Conflict Modal */}
      {showConflictModal && conflictItems.length > 0 && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
          <div className="bg-atc-bg border border-amber-500/30 rounded-xl shadow-[0_0_50px_rgba(245,158,11,0.1)] w-full max-w-2xl overflow-hidden flex flex-col">
            <div className="p-4 border-b border-atc-border flex items-center justify-between bg-amber-500/10">
              <div className="flex items-center gap-2 text-amber-500">
                <AlertTriangle className="w-5 h-5" />
                <h3 className="font-semibold text-lg">
                  Sync Conflict Detected
                </h3>
              </div>
              <button
                onClick={() => setShowConflictModal(false)}
                className="text-slate-500 hover:text-white"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
            <div className="p-6 overflow-y-auto max-h-[60vh]">
              <p className="text-sm text-slate-300 mb-6">
                The following items were modified in both GitHub and Notion.
                Please select the source of truth to proceed with the sync.
              </p>

              {conflictItems.map((item) => (
                <div
                  key={item.id}
                  className="mb-6 border border-atc-border rounded-lg p-4 bg-[#090b14]"
                >
                  <h4 className="font-medium text-white mb-4 text-base">
                    {item.title}
                  </h4>
                  <div className="grid grid-cols-2 gap-4">
                    <div
                      className="border border-atc-border hover:border-atc-cyan/50 p-4 rounded-lg bg-atc-card cursor-pointer transition-colors"
                      onClick={() => {
                        setConflictItems((prev) =>
                          prev.filter((i) => i.id !== item.id),
                        );
                        if (conflictItems.length <= 1)
                          setShowConflictModal(false);
                      }}
                    >
                      <div className="flex items-center gap-2 text-atc-cyan mb-2">
                        <Github className="w-4 h-4" />
                        <span className="font-medium text-sm">Keep GitHub</span>
                      </div>
                      <div className="text-xs text-slate-400">
                        <p>Status: {item.githubSource.status}</p>
                        <p>Goals: {item.githubSource.goals?.length} items</p>
                      </div>
                    </div>
                    <div
                      className="border border-atc-border hover:border-atc-purple/50 p-4 rounded-lg bg-atc-card cursor-pointer transition-colors"
                      onClick={() => {
                        setConflictItems((prev) =>
                          prev.filter((i) => i.id !== item.id),
                        );
                        if (conflictItems.length <= 1)
                          setShowConflictModal(false);
                      }}
                    >
                      <div className="flex items-center gap-2 text-atc-purple mb-2">
                        <FileText className="w-4 h-4" />
                        <span className="font-medium text-sm">Keep Notion</span>
                      </div>
                      <div className="text-xs text-slate-400">
                        <p>Status: {item.notionSource.status}</p>
                        <p>Goals: {item.notionSource.goals?.length} items</p>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Sync History Modal */}
      {showSyncHistory && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
          <div className="bg-atc-bg border border-atc-border rounded-xl shadow-2xl w-full max-w-xl overflow-hidden flex flex-col max-h-[80vh]">
            <div className="p-4 border-b border-atc-border flex items-center justify-between">
              <div className="flex items-center gap-2">
                <span className="p-1.5 bg-atc-card rounded-md border border-atc-border">
                  <History className="w-4 h-4 text-slate-400" />
                </span>
                <h3 className="font-semibold text-lg text-white">
                  Sync History
                </h3>
              </div>
              <button
                onClick={() => {
                  setShowSyncHistory(false);
                  setExpandedDiffs([]);
                }}
                className="text-slate-500 hover:text-white"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
            <div className="p-4 overflow-y-auto bg-black/40">
              {loadingHistory ? (
                <div className="flex items-center justify-center py-8">
                  <div className="w-6 h-6 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin" />
                </div>
              ) : remoteHistory.length === 0 ? (
                <p className="text-center text-slate-500 text-sm py-8 font-mono">
                  No remote agent sync history available.
                </p>
              ) : (
                <div className="flex flex-col gap-3">
                  {remoteHistory.map((log) => {
                    const isSuccess = log.content.includes("✅");
                    const isAutomated = log.content.includes("scheduled");
                    const isExpanded = expandedDiffs.includes(log.id);

                    // Simple mock diff logic based on log string length parsing to look somewhat realistic
                    const lines = log.content.split("\n");
                    const titleLine = lines[0] || "Sync execution";

                    const generateMockDiffs = () => [
                      {
                        file: "src/config/flags.json",
                        type: "modified",
                        additions: Math.floor(Math.random() * 10),
                        deletions: Math.floor(Math.random() * 5),
                      },
                      {
                        file: "README.md",
                        type: "modified",
                        additions: 2,
                        deletions: 1,
                      },
                      ...(Math.random() > 0.5
                        ? [
                            {
                              file: "docs/architecture.md",
                              type: "added",
                              additions: 45,
                              deletions: 0,
                            },
                          ]
                        : []),
                    ];
                    // Cache the mock diffs on the log object so it doesn't re-render differently
                    if (!log._mockDiff) log._mockDiff = generateMockDiffs();

                    return (
                      <div
                        key={log.id}
                        className={`p-3 rounded-lg border flex flex-col gap-2 ${isSuccess ? "border-emerald-500/30" : "border-indigo-500/30"} bg-[#090b14] hover:bg-white/5 transition-colors`}
                      >
                        <div className="flex items-center justify-between">
                          <span
                            className={`text-[10px] uppercase font-bold tracking-widest px-2 py-0.5 rounded-full ${isAutomated ? "bg-indigo-500/20 text-indigo-400" : "bg-fuchsia-500/20 text-fuchsia-400"}`}
                          >
                            {isAutomated ? "Auto-Sync" : "Agent Action"}
                          </span>
                          <span className="text-[10px] font-mono text-slate-500">
                            {new Date(log.timestamp).toLocaleString()}
                          </span>
                        </div>
                        <div className="flex items-start justify-between gap-4 mt-1">
                          <div className="text-xs text-slate-300 font-mono whitespace-pre-wrap leading-relaxed select-text flex-1">
                            <div className="font-semibold text-white mb-1">
                              {titleLine}
                            </div>
                            {!isExpanded && (
                              <div className="text-slate-500 line-clamp-2">
                                {lines.slice(1).join(" ")}
                              </div>
                            )}
                          </div>
                          <button
                            onClick={() =>
                              setExpandedDiffs((prev) =>
                                isExpanded
                                  ? prev.filter((id) => id !== log.id)
                                  : [...prev, log.id],
                              )
                            }
                            className="px-2 py-1 bg-white/5 hover:bg-white/10 rounded border border-white/10 text-xs font-medium text-slate-300 whitespace-nowrap"
                          >
                            {isExpanded ? "Hide Logs" : "View Diffs"}
                          </button>
                        </div>

                        {isExpanded && (
                          <div className="mt-3 pt-3 border-t border-white/10 flex flex-col gap-3">
                            <div className="text-xs text-slate-400 font-mono whitespace-pre-wrap">
                              {lines.slice(1).join("\n")}
                            </div>

                            {!isSuccess ? (
                              <div className="mt-2 flex flex-col gap-3">
                                <h4 className="text-[10px] font-mono font-bold tracking-widest text-amber-500 uppercase flex items-center gap-2">
                                  <AlertTriangle className="w-3.5 h-3.5" />{" "}
                                  Conflict Resolution Required
                                </h4>
                                <div className="grid grid-cols-2 gap-3">
                                  <div className="border border-white/10 bg-black/40 rounded-lg p-3 flex flex-col justify-between hover:border-atc-cyan/50 transition-colors">
                                    <div>
                                      <div className="flex items-center gap-2 text-atc-cyan mb-2">
                                        <Github className="w-4 h-4" />
                                        <span className="font-semibold text-xs">
                                          GitHub Version
                                        </span>
                                      </div>
                                      <div className="text-xs text-slate-400 font-mono mb-3 line-clamp-3">
                                        {
                                          "Latest commit affects core logic and layout parameters."
                                        }
                                      </div>
                                    </div>
                                    <button className="w-full py-1.5 bg-atc-cyan/10 hover:bg-atc-cyan/20 text-atc-cyan text-xs font-medium rounded border border-atc-cyan/30 transition-colors">
                                      Keep GitHub Version
                                    </button>
                                  </div>
                                  <div className="border border-white/10 bg-black/40 rounded-lg p-3 flex flex-col justify-between hover:border-atc-purple/50 transition-colors">
                                    <div>
                                      <div className="flex items-center gap-2 text-atc-purple mb-2">
                                        <FileText className="w-4 h-4" />
                                        <span className="font-semibold text-xs">
                                          Notion Version
                                        </span>
                                      </div>
                                      <div className="text-xs text-slate-400 font-mono mb-3 line-clamp-3">
                                        {
                                          "Updated documentation and state status directly in workspace."
                                        }
                                      </div>
                                    </div>
                                    <button className="w-full py-1.5 bg-atc-purple/10 hover:bg-atc-purple/20 text-atc-purple text-xs font-medium rounded border border-atc-purple/30 transition-colors">
                                      Keep Notion Version
                                    </button>
                                  </div>
                                </div>
                              </div>
                            ) : (
                              <div className="bg-black/60 border border-white/5 rounded-lg p-3">
                                <h4 className="text-[10px] font-mono font-bold tracking-widest text-slate-500 uppercase mb-2">
                                  Changed Files
                                </h4>
                                <div className="space-y-2">
                                  {log._mockDiff.map(
                                    (diff: any, idx: number) => (
                                      <div
                                        key={idx}
                                        className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-white/5 last:border-0 pb-2 last:pb-0"
                                      >
                                        <div className="flex items-center gap-2">
                                          <span
                                            className={`w-2 h-2 rounded-full ${diff.type === "added" ? "bg-emerald-500" : "bg-amber-500"}`}
                                          />
                                          <span className="text-xs font-mono text-slate-300">
                                            {diff.file}
                                          </span>
                                        </div>
                                        <div className="flex items-center gap-3 text-xs font-mono">
                                          <span className="text-emerald-400">
                                            +{diff.additions}
                                          </span>
                                          <span className="text-rose-400">
                                            -{diff.deletions}
                                          </span>
                                        </div>
                                      </div>
                                    ),
                                  )}
                                </div>
                              </div>
                            )}
                          </div>
                        )}
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function StatCard({
  icon: Icon,
  title,
  current,
  vision,
  highlight = false,
  info,
}: {
  icon: React.ElementType;
  title: string;
  current: string;
  vision: string;
  highlight?: boolean;
  info?: string;
}) {
  return (
    <div
      className={`p-6 rounded-2xl border transition-all shadow-[0_4px_20px_rgba(0,0,0,0.2)] ${highlight ? "bg-indigo-500/10 border-indigo-500/30 text-indigo-400" : "bg-[#090b14]/60 border-atc-border/50 hover:bg-[#090b14]"}`}
    >
      <div className="flex items-center gap-3 mb-5">
        <div
          className={`p-2 rounded-lg ${highlight ? "bg-indigo-500/20" : "bg-[#060a16] border border-atc-border/50 text-slate-400"}`}
        >
          <Icon className="w-5 h-5" />
        </div>
        <div className="flex items-center gap-1.5">
          <span
            className={`font-mono text-xs tracking-widest font-bold uppercase ${highlight ? "text-indigo-400" : "text-slate-500"}`}
          >
            {title}
          </span>
          {info && <TooltipIcon content={info} />}
        </div>
      </div>
      <div className="flex items-end justify-between">
        <div>
          <div className="text-[10px] text-slate-500 font-mono font-bold tracking-widest uppercase mb-1">
            Status Quo
          </div>
          <div className="text-lg font-medium text-slate-300">{current}</div>
        </div>
        <div className="px-3 flex items-center justify-center opacity-50 pb-1">
          <ArrowRight
            className={`w-4 h-4 ${highlight ? "text-indigo-400" : "text-slate-500"}`}
          />
        </div>
        <div className="text-right">
          <div className="text-[10px] text-atc-cyan/70 font-mono font-bold tracking-widest uppercase mb-1">
            Target Vision
          </div>
          <div
            className={`text-xl font-semibold tracking-tight ${highlight ? "text-atc-cyan" : "text-white"}`}
          >
            {vision}
          </div>
        </div>
      </div>
    </div>
  );
}

export function EcosystemView() {
  return (
    <div className="flex flex-col gap-10 mt-8 pb-12">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
          <Layers className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Ausbaustufen & Ecosystem
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Systematische Darstellung der technischen Evolution des A-TownChain
            Ökosystems über die Kernbereiche hinweg.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-8">
        {ECOSYSTEM_DATA.map((layer) => {
          const colorIndex = layer.id - 1;
          const getColorClass = (i: number) => {
            if (i < 6) return "text-atc-cyan border-atc-cyan/30 bg-atc-cyan/10";
            if (i < 12)
              return "text-indigo-400 border-indigo-400/30 bg-indigo-400/10";
            if (i < 18)
              return "text-atc-purple border-atc-purple/30 bg-atc-purple/10";
            if (i < 24)
              return "text-pink-400 border-pink-400/30 bg-pink-400/10";
            return "text-emerald-400 border-emerald-400/30 bg-emerald-400/10";
          };
          const themeClass = getColorClass(colorIndex);
          const glowClass = `bg-${themeClass.split("-")[1]}/10`;

          return (
            <div
              key={layer.id}
              className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-[0_4px_30px_rgba(0,0,0,0.5)] hover:shadow-2xl hover:border-slate-500/50 hover:-translate-y-1 transition-all duration-500 flex flex-col gap-8 relative overflow-hidden group"
            >
              <div
                className={`absolute top-0 right-0 w-48 h-48 rounded-full blur-[80px] opacity-0 group-hover:opacity-100 transition-opacity duration-700 ${themeClass.split(" ")[2]}`}
              />

              {/* Category Header */}
              <div className="flex items-center gap-5 relative z-10 border-b border-atc-border pb-6">
                <div
                  className={`w-12 h-12 rounded-xl flex items-center justify-center border ${themeClass}`}
                >
                  <Component className="w-6 h-6" />
                </div>
                <div>
                  <div className="text-[10px] font-mono text-slate-500 font-bold uppercase tracking-widest mb-1.5">
                    Layer {layer.id}
                  </div>
                  <h3 className="text-xl text-white font-semibold tracking-tight">
                    {layer.title}
                  </h3>
                </div>
              </div>

              {/* Subcategories */}
              <div className="flex flex-col gap-6 relative z-10 flex-1">
                {layer.subcategories.map((sub, idx) => (
                  <div key={idx} className="flex flex-col gap-3">
                    <h4
                      className={`text-[10px] font-mono font-bold tracking-widest uppercase ${themeClass.split(" ")[0]}`}
                    >
                      {sub.title}
                    </h4>
                    <div className="flex flex-wrap gap-2.5">
                      {sub.items.map((item, idxi) => (
                        <span
                          key={idxi}
                          className="px-3 py-1.5 text-[11px] font-mono font-medium text-slate-300 bg-atc-bg/80 border border-atc-border/60 rounded-lg whitespace-nowrap hover:bg-atc-card transition-colors"
                        >
                          {item}
                        </span>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          );
        })}
      </div>

      {/* Ecosystem Summary */}
      <div className="mt-8 p-10 rounded-3xl bg-[#060a16] shadow-2xl relative overflow-hidden flex flex-col sm:flex-row gap-8 justify-between items-start sm:items-center">
        <div className="absolute top-0 right-0 w-96 h-96 bg-pink-600/10 rounded-full blur-[80px] pointer-events-none" />
        <div className="relative z-10">
          <h3 className="text-2xl font-semibold text-white mb-3">
            Gesamtsystem-Skalierung
          </h3>
          <p className="text-sm text-slate-500 font-light max-w-2xl leading-relaxed">
            A-TownChain / ATC-OS vereint Blockchain, ATC-OS, Agentennetzwerk,
            DAO, Compute-Markt, Sicherheitsplattform, Verifikationssystem und
            autonome Governance in einem gemeinsamen Stack.
          </p>
        </div>
        <div className="flex flex-wrap gap-6 shrink-0 relative z-10">
          <div className="flex flex-col">
            <div className="text-pink-400 font-mono text-3xl font-light">
              ~30
            </div>
            <div className="text-[10px] text-slate-500 font-mono tracking-widest uppercase mt-1">
              Hauptsysteme
            </div>
          </div>
          <div className="hidden sm:block w-[1px] bg-atc-card" />
          <div className="flex flex-col">
            <div className="text-pink-400 font-mono text-3xl font-light">
              ~400
            </div>
            <div className="text-[10px] text-slate-500 font-mono tracking-widest uppercase mt-1">
              Subsysteme
            </div>
          </div>
          <div className="hidden sm:block w-[1px] bg-atc-card" />
          <div className="flex flex-col">
            <div className="text-pink-400 font-mono text-3xl font-light">
              5.000+
            </div>
            <div className="text-[10px] text-slate-500 font-mono tracking-widest uppercase mt-1">
              Komponenten
            </div>
          </div>
          <div className="hidden sm:block w-[1px] bg-atc-card" />
          <div className="flex flex-col">
            <div className="text-pink-400 font-mono text-3xl font-light">
              10.000+
            </div>
            <div className="text-[10px] text-slate-500 font-mono tracking-widest uppercase mt-1">
              Services
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export function HierarchyView() {
  return (
    <div className="flex flex-col gap-12 mt-8 pb-12">
      {/* Tiers Section */}
      <section>
        <div className="flex items-center gap-4 mb-8">
          <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
            <Layers className="w-6 h-6" />
          </div>
          <div>
            <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
              Tier Hierarchisierung
            </h2>
            <p className="text-sm font-light text-slate-500">
              SYSTEM ARCHITECTURE CLASSIFICATION
            </p>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 lg:gap-8">
          {TIERS.map((tier, idx) => {
            const getColorClass = (i: number) => {
              if (i < 3)
                return "text-atc-cyan border-atc-cyan/30 bg-atc-cyan/10";
              if (i < 6)
                return "text-indigo-400 border-indigo-400/30 bg-indigo-400/10";
              if (i < 9)
                return "text-atc-purple border-atc-purple/30 bg-atc-purple/10";
              if (i < 12)
                return "text-pink-400 border-pink-400/30 bg-pink-400/10";
              return "text-emerald-400 border-emerald-400/30 bg-emerald-400/10";
            };
            const glowClass = (i: number) => {
              if (i < 3) return "bg-atc-cyan/10";
              if (i < 6) return "bg-indigo-400/10";
              if (i < 9) return "bg-atc-purple/10";
              if (i < 12) return "bg-pink-400/10";
              return "bg-emerald-400/10";
            };
            const themeClass = getColorClass(idx);

            return (
              <div
                key={tier.id}
                className="p-6 md:p-8 rounded-2xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-[0_4px_30px_rgba(0,0,0,0.5)] hover:shadow-2xl hover:border-slate-500/50 hover:-translate-y-1 transition-all duration-500 flex flex-col h-full relative overflow-hidden group"
              >
                <div
                  className={`absolute top-0 right-0 w-32 h-32 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity duration-700 ${glowClass(idx)}`}
                />
                <div className="flex items-start justify-between mb-6 relative z-10">
                  <div>
                    <h3 className="text-xl text-white font-semibold tracking-wide mb-2 transition-colors">
                      {tier.name}
                    </h3>
                    <p className="text-sm text-slate-400 font-light leading-relaxed">
                      {tier.description}
                    </p>
                  </div>
                </div>

                <div className="flex-1 flex flex-col gap-4 mt-2 relative z-10">
                  <div className="flex flex-col gap-3">
                    {tier.systems.map((sys, i) => {
                      if (typeof sys === "string") {
                        return (
                          <span
                            key={i}
                            className={`px-3 py-1.5 text-xs font-medium border rounded-lg w-max shadow-[0_0_10px_rgba(0,0,0,0.2)] ${themeClass}`}
                          >
                            {sys}
                          </span>
                        );
                      }
                      return (
                        <div
                          key={i}
                          className={`flex flex-col gap-1.5 p-3.5 bg-[#090b14]/60 border rounded-xl w-full transition-colors hover:bg-[#090b14] shadow-[0_0_10px_rgba(0,0,0,0.2)] ${themeClass.split(" ")[1]}`}
                        >
                          <span
                            className={`text-sm font-semibold tracking-wide ${themeClass.split(" ")[0]}`}
                          >
                            {sys.name}
                          </span>
                          {sys.description && (
                            <span className="text-[11px] font-mono text-slate-400 tracking-wide">
                              {sys.description}
                            </span>
                          )}
                        </div>
                      );
                    })}
                  </div>
                </div>

                <div className="mt-8 pt-5 border-t border-atc-border/50 grid grid-cols-3 gap-2 text-xs font-mono relative z-10">
                  <div className="flex flex-col gap-1 border-r border-atc-border/50 pr-2">
                    <span className="text-slate-500 tracking-wider text-[9px] uppercase">
                      Subsystems
                    </span>
                    <span className="text-slate-200">
                      {tier.metrics.subsystems}
                    </span>
                  </div>
                  <div className="flex flex-col gap-1 border-r border-atc-border/50 px-2">
                    <span className="text-slate-500 tracking-wider text-[9px] uppercase">
                      Components
                    </span>
                    <span className="text-slate-200">
                      {tier.metrics.components}
                    </span>
                  </div>
                  <div className="flex flex-col gap-1 pl-2">
                    <span className="text-slate-500 tracking-wider text-[9px] uppercase">
                      Timeline
                    </span>
                    <span className="text-slate-200">{tier.timeline}</span>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </section>

      {/* Meta Systems Section */}
      <section>
        <div className="flex items-center gap-3 mb-8">
          <div className="p-2.5 rounded-xl bg-atc-purple/10 border border-atc-purple/30 shadow-[0_0_15px_rgba(0,0,0,0.5)]">
            <Link className="w-6 h-6 text-atc-purple" />
          </div>
          <div>
            <h2 className="text-2xl font-semibold tracking-tight text-white">
              Querschnittssysteme
            </h2>
            <p className="text-sm font-mono text-slate-500">
              META-FABRIC & CONNECTIVITY
            </p>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {META_SYSTEMS.map((meta) => (
            <div
              key={meta.id}
              className="p-6 rounded-2xl border bg-atc-card shadow-[0_0_15px_rgba(0,0,0,0.5)] hover:shadow-[0_0_25px_rgba(0,209,255,0.05)] border-atc-border transition-all duration-300"
            >
              <div className="flex items-center gap-4 mb-5">
                <div className="w-10 h-10 rounded-xl bg-atc-purple/10 flex flex-col items-center justify-center border border-atc-purple/30 shrink-0">
                  <span className="text-atc-purple font-mono font-bold">
                    {meta.id}
                  </span>
                </div>
                <h3 className="font-semibold text-white tracking-wide leading-tight">
                  {meta.name}
                </h3>
              </div>
              <p className="text-sm text-slate-500 font-light leading-relaxed mb-6 h-12">
                {meta.description}
              </p>
              <div className="flex flex-col gap-2.5">
                {meta.items.map((item, i) => (
                  <div
                    key={i}
                    className="text-xs text-slate-500 font-mono flex items-start gap-3 bg-atc-bg border border-atc-border p-2 rounded-lg"
                  >
                    <span className="w-1.5 h-1.5 mt-1 rounded-full bg-atc-purple shrink-0" />
                    <span className="leading-tight">{item}</span>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* End State Metrics Section */}
      <section className="mt-6">
        <div className="p-8 md:p-10 rounded-3xl bg-[#060a16] border border-atc-border-hover shadow-2xl relative overflow-hidden">
          <div className="absolute top-0 right-0 w-[600px] h-[600px] bg-atc-purple/10 rounded-full blur-[100px] pointer-events-none translate-x-1/3 -translate-y-1/3" />
          <div className="relative z-10 flex flex-col md:flex-row md:items-center justify-between gap-8">
            <div className="max-w-xs">
              <h3 className="text-2xl font-semibold text-white mb-2">
                Long-Term Vision
              </h3>
              <p className="text-slate-500 text-sm font-light leading-relaxed">
                Geschätzter Endausbau der Infrastruktur bis 2026/2027
              </p>
            </div>
            <div className="grid grid-cols-2 lg:grid-cols-5 gap-6 md:gap-10">
              {END_STATE_METRICS.map((metric, i) => (
                <div key={i} className="flex flex-col">
                  <span className="text-atc-purple font-mono text-3xl md:text-4xl font-light tracking-tight mb-2">
                    {metric.value}
                  </span>
                  <span className="text-[10px] font-mono text-slate-500 uppercase tracking-widest">
                    {metric.label}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* Ultimate Stacks & Reference Architecture */}
      <section className="mt-12 border-t border-atc-border pt-12">
        <div className="flex flex-col lg:flex-row gap-10">
          <div className="flex-1 flex flex-col gap-10">
            {/* Verification Stack */}
            <div>
              <div className="flex items-center gap-3 mb-6">
                <div className="p-2 rounded-lg bg-atc-purple/10 border border-atc-purple/30">
                  <CheckSquare className="w-5 h-5 text-atc-purple" />
                </div>
                <h2 className="text-xl font-semibold tracking-tight text-white">
                  Ultimate Verification Stack
                </h2>
                <TooltipIcon content="A modular verification ring consisting of runtime analyzers, semantic indexing, and node validators to guarantee state integrity." />
              </div>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                {ULTIMATE_VERIFICATION_STACK.map((category, i) => (
                  <div
                    key={i}
                    className="p-5 rounded-2xl bg-atc-card border border-atc-border shadow-[0_0_15px_rgba(0,0,0,0.5)] transition-all hover:shadow-[0_0_25px_rgba(0,209,255,0.05)]"
                  >
                    <h4 className="text-sm font-semibold text-slate-100 mb-4 tracking-wide">
                      {category.title}
                    </h4>
                    <div className="flex flex-col gap-3">
                      {category.items.map((item, idx) => (
                        <div
                          key={idx}
                          className="text-xs text-slate-500 font-mono flex items-start gap-3"
                        >
                          <div className="w-1.5 h-1.5 mt-1 rounded-full bg-atc-purple/50 shrink-0" />
                          <span className="leading-tight">{item}</span>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Security Stack */}
            <div>
              <div className="flex items-center gap-3 mb-6">
                <div className="p-2 rounded-lg bg-atc-purple/10 border border-atc-purple/30">
                  <ShieldCheck className="w-5 h-5 text-atc-purple" />
                </div>
                <h2 className="text-xl font-semibold tracking-tight text-white">
                  Ultimate Security Stack
                </h2>
                <TooltipIcon content="Ensures cryptographic boundaries, zero-knowledge proofs, and continuous adversarial penetration testing across all network layers." />
              </div>
              <div className="flex flex-col gap-4">
                {ULTIMATE_SECURITY_STACK.map((category, i) => (
                  <div
                    key={i}
                    className="flex flex-col sm:flex-row sm:items-center gap-6 p-5 rounded-2xl bg-atc-bg border border-atc-border/60"
                  >
                    <h4 className="text-sm font-mono font-semibold text-atc-cyan whitespace-nowrap min-w-40">
                      {category.title}
                    </h4>
                    <div className="flex flex-wrap gap-2">
                      {category.items.map((item, idx) => (
                        <span
                          key={idx}
                          className="px-3 py-1.5 text-xs font-medium text-slate-300 bg-atc-card border border-atc-border/80 shadow-[0_0_15px_rgba(0,0,0,0.5)] rounded-lg"
                        >
                          {item}
                        </span>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Reference Architecture */}
          <div className="lg:w-[400px] shrink-0 flex flex-col gap-8">
            <div className="p-8 rounded-3xl bg-gradient-to-b from-indigo-50/40 to-white border border-atc-purple/20 shadow-[0_0_15px_rgba(0,0,0,0.5)] h-full">
              <div className="flex items-center gap-3 mb-6">
                <div className="p-2 rounded-lg bg-atc-purple/20 border border-atc-purple/30 text-atc-purple">
                  <Link className="w-5 h-5 flex-shrink-0" />
                </div>
                <h2 className="text-xl font-semibold tracking-tight text-white">
                  Fundament (Säulen)
                </h2>
              </div>
              <p className="text-sm text-slate-500 font-light mb-8 leading-relaxed">
                Trotz der Skalierung im Endzustand baut alles auf wenigen
                unveränderlichen Säulen auf:
              </p>
              <div className="flex flex-col gap-4">
                {REFERENCE_ARCHITECTURE.map((layer, idx) => (
                  <div
                    key={idx}
                    className="flex items-center gap-4 p-4 rounded-xl bg-atc-card border border-atc-border shadow-[0_0_15px_rgba(0,0,0,0.5)] transition-all hover:-translate-y-0.5 hover:shadow-[0_0_25px_rgba(0,209,255,0.05)] hover:border-indigo-300 group"
                  >
                    <div className="w-8 h-8 rounded-lg bg-atc-purple/10 flex items-center justify-center border border-atc-purple/20 shrink-0 group-hover:bg-atc-purple/20 transition-colors">
                      <span className="text-xs text-atc-purple font-bold">
                        {idx + 1}
                      </span>
                    </div>
                    <span className="text-sm text-slate-300 font-medium">
                      {layer}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* Final Architecture Levels */}
        <div className="mt-12">
          <div className="p-8 md:p-10 rounded-3xl border border-atc-border bg-atc-bg relative overflow-hidden flex flex-col md:flex-row gap-8 items-center">
            <div className="absolute top-0 right-0 w-96 h-96 bg-atc-purple/20 rounded-full blur-[80px] pointer-events-none" />
            <div className="flex-1 relative z-10">
              <h3 className="text-2xl font-semibold text-white tracking-tight mb-4">
                Finale Architektur-Ebenen
              </h3>
              <p className="text-slate-500 text-sm font-light leading-relaxed max-w-2xl">
                Im Endausbau ist A-TownChain/ATC-OS eine vollständige digitale
                Infrastruktur- und Koordinationsplattform, aufgebaut aus 15
                wesentlichen Makro-Layern, wobei sich jede Schicht modular und
                überprüfbar weiterentwickeln lässt.
              </p>
            </div>
            <div className="flex gap-2.5 flex-wrap flex-1 relative z-10 justify-end">
              {FINAL_ARCHITECTURE_LEVELS.map((level, i) => (
                <span
                  key={i}
                  className="px-4 py-2 text-xs font-mono font-medium bg-atc-card text-atc-cyan border border-atc-border/80 shadow-[0_0_15px_rgba(0,0,0,0.5)] rounded-lg hover:border-indigo-300 transition-colors cursor-default"
                >
                  {level}
                </span>
              ))}
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}

export function StandardsView() {
  return (
    <div className="flex flex-col gap-10 mt-8 pb-12">
      <div className="mb-6 border-b border-atc-border/50 pb-8 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-atc-purple/10 border border-atc-purple/30 flex items-center justify-center text-atc-purple shadow-[0_0_15px_rgba(162,89,255,0.2)]">
          <BookOpen className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Protocol und Standards Bibliothek
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Das formale Proposal-System für die Evolution der A-TownChain
            Architektur. Unterschieden wird zwischen ATC (Core
            Protocol/Consensus) und ATS (Applicational/Governance).
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-10">
        {/* ATC Core Standards */}
        <div className="flex flex-col gap-6">
          <div className="flex items-center gap-4 mb-2 relative">
            <div className="p-2.5 rounded-xl bg-atc-cyan/10 border border-atc-cyan/30 shadow-[0_0_10px_rgba(6,182,212,0.2)]">
              <Server className="w-5 h-5 text-atc-cyan" />
            </div>
            <h3 className="text-xl font-semibold text-white tracking-tight">
              ATC-STANDARDS{" "}
              <span className="text-sm font-normal text-slate-500 font-mono ml-2">
                (Core Architecture)
              </span>
              <TooltipIcon content="Core data models, schemas, and state representation on A-TownChain." />
            </h3>
          </div>
          {ATC_STANDARDS.map((cat) => (
            <div
              key={cat.id}
              className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl rounded-3xl p-6 md:p-8 hover:border-atc-cyan/30 transition-all duration-300 relative overflow-hidden group"
            >
              <div className="absolute top-0 right-0 w-48 h-48 bg-atc-cyan/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-atc-cyan/10 transition-opacity" />
              <h4 className="text-lg font-semibold text-white mb-2 relative z-10">
                {cat.title}
              </h4>
              <p className="text-sm text-slate-500 font-light mb-8 relative z-10">
                {cat.description}
              </p>

              <div className="flex flex-col gap-4 relative z-10">
                {cat.items.map((item) => (
                  <div
                    key={item.id}
                    className="group/item flex flex-col sm:flex-row sm:items-start gap-4 p-5 rounded-2xl bg-[#090b14]/60 border border-atc-border/50 hover:border-atc-cyan/30 hover:bg-[#090b14] transition-colors relative overflow-hidden shadow-[0_4px_20px_rgba(0,0,0,0.2)]"
                  >
                    <div className="absolute top-0 left-0 w-1 h-full bg-atc-cyan/30 opacity-0 group-hover/item:opacity-100 transition-opacity" />
                    <div className="flex flex-col gap-2 min-w-[140px] pt-0.5">
                      <span className="font-mono text-sm font-semibold text-slate-300 group-hover/item:text-atc-cyan transition-colors">
                        {item.id}
                      </span>
                      <span
                        className={`inline-flex px-2 py-1 text-[10px] uppercase font-mono tracking-widest font-bold rounded-md border w-max shadow-[0_0_10px_rgba(0,0,0,0.2)] ${item.status === "Core" ? "bg-atc-purple/10 text-atc-cyan border-atc-cyan/30" : item.status === "Active" ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/30" : item.status === "Draft" ? "bg-amber-500/10 text-amber-400 border-amber-500/30" : "bg-[#060a16] border-atc-border/80 text-slate-500"}`}
                      >
                        {item.status}
                      </span>
                    </div>
                    <div className="flex-1">
                      <h5 className="text-sm font-semibold text-slate-100 mb-1">
                        {item.title}
                      </h5>
                      <p className="text-sm text-slate-400 font-light leading-relaxed">
                        {item.description}
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>

        {/* ATS Ecosystem Standards */}
        <div className="flex flex-col gap-6">
          <div className="flex items-center gap-4 mb-2">
            <div className="p-2.5 rounded-xl bg-orange-500/10 border border-orange-500/30 shadow-[0_0_10px_rgba(249,115,22,0.2)]">
              <BookOpen className="w-5 h-5 text-orange-400" />
            </div>
            <h3 className="text-xl font-semibold text-white tracking-tight">
              ATS-STANDARDS{" "}
              <span className="text-sm font-normal text-slate-500 font-mono ml-2">
                (Applicational)
              </span>
            </h3>
          </div>
          {ATS_STANDARDS.map((cat) => (
            <div
              key={cat.id}
              className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl rounded-3xl p-6 md:p-8 hover:border-orange-500/30 transition-all duration-300 relative overflow-hidden group"
            >
              <div className="absolute top-0 right-0 w-48 h-48 bg-orange-500/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-orange-500/10 transition-opacity" />
              <h4 className="text-lg font-semibold text-white mb-2 relative z-10">
                {cat.title}
              </h4>
              <p className="text-sm text-slate-500 font-light mb-8 relative z-10">
                {cat.description}
              </p>

              <div className="flex flex-col gap-4 relative z-10">
                {cat.items.map((item) => (
                  <div
                    key={item.id}
                    className="group/item flex flex-col sm:flex-row sm:items-start gap-4 p-5 rounded-2xl bg-[#090b14]/60 border border-atc-border/50 hover:border-orange-500/30 hover:bg-[#090b14] transition-colors relative overflow-hidden shadow-[0_4px_20px_rgba(0,0,0,0.2)]"
                  >
                    <div className="absolute top-0 left-0 w-1 h-full bg-orange-500/30 opacity-0 group-hover/item:opacity-100 transition-opacity" />
                    <div className="flex flex-col gap-2 min-w-[140px] pt-0.5">
                      <span className="font-mono text-sm font-semibold text-slate-300 group-hover/item:text-orange-400 transition-colors">
                        {item.id}
                      </span>
                      <span
                        className={`inline-flex px-2 py-1 text-[10px] uppercase font-mono tracking-widest font-bold rounded-md border w-max shadow-[0_0_10px_rgba(0,0,0,0.2)] ${item.status === "Core" ? "bg-atc-purple/10 text-atc-cyan border-atc-cyan/30" : item.status === "Active" ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/30" : item.status === "Draft" ? "bg-amber-500/10 text-amber-400 border-amber-500/30" : "bg-[#060a16] border-atc-border/80 text-slate-500"}`}
                      >
                        {item.status}
                      </span>
                    </div>
                    <div className="flex-1">
                      <h5 className="text-sm font-semibold text-slate-100 mb-1">
                        {item.title}
                      </h5>
                      <p className="text-sm text-slate-400 font-light leading-relaxed">
                        {item.description}
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export function AuditView() {
  return (
    <div className="flex flex-col gap-10 mt-8 pb-12">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-atc-purple/10 border border-atc-purple/30 flex items-center justify-center text-atc-purple shadow-[0_0_15px_rgba(162,89,255,0.2)]">
          <ShieldCheck className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Audit & Gap Analysis
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Das formale Framework zur Bewertung der A-TownChain
            Architekturgrundlagen. Auswertung des GitHub-Repositories{" "}
            <code className="px-2 py-1 bg-[#090b14] text-atc-cyan font-mono text-xs rounded-md mx-1 border border-atc-cyan/20 shadow-[0_0_10px_rgba(6,182,212,0.1)]">
              A-TownChain/a-townchain-core
            </code>
            .
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-2 gap-8">
        <div className="flex flex-col gap-8">
          <div className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl relative overflow-hidden group hover:border-pink-500/30 transition-all duration-300">
            <div className="absolute top-0 right-0 w-64 h-64 bg-pink-500/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-pink-500/10 transition-opacity" />
            <div className="flex items-center gap-4 mb-8 relative z-10">
              <div className="p-3 bg-pink-500/10 border border-pink-500/30 rounded-xl shadow-[0_0_15px_rgba(236,72,153,0.2)]">
                <Search className="w-6 h-6 text-pink-400" />
              </div>
              <h3 className="text-xl font-semibold text-white tracking-tight">
                {ARCHITECTURE_REVIEW.title}
              </h3>
            </div>
            <div className="flex flex-col gap-4 relative z-10">
              {ARCHITECTURE_REVIEW.items.map((item, idx) => (
                <div
                  key={idx}
                  className="flex flex-col gap-2 p-5 rounded-2xl bg-[#090b14]/60 hover:bg-[#090b14] border border-atc-border/50 hover:border-pink-500/30 transition-all shadow-[0_4px_20px_rgba(0,0,0,0.2)]"
                >
                  <div className="flex items-center gap-3">
                    <span
                      className={`px-2.5 py-1 rounded-md text-[10px] font-mono font-bold tracking-wide uppercase shadow-[0_0_10px_rgba(0,0,0,0.2)] ${
                        item.status === "Pass"
                          ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/30"
                          : item.status === "Warn"
                            ? "bg-amber-500/10 text-amber-400 border border-amber-500/30"
                            : item.status === "Fail"
                              ? "bg-rose-500/10 text-rose-400 border border-rose-500/30"
                              : "bg-atc-purple/10 text-atc-cyan border border-atc-cyan/30"
                      }`}
                    >
                      {item.status}
                    </span>
                    <span className="text-sm font-semibold text-slate-100">
                      {item.label}
                    </span>
                  </div>
                  <span className="text-sm text-slate-500 font-light leading-relaxed ml-14">
                    {item.description}
                  </span>
                </div>
              ))}
            </div>
          </div>

          <div className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl relative overflow-hidden group hover:border-rose-500/30 transition-all duration-300">
            <div className="absolute top-0 right-0 w-64 h-64 bg-rose-500/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-rose-500/10 transition-opacity" />
            <div className="flex items-center gap-4 mb-8 relative z-10">
              <div className="p-3 bg-rose-500/10 border border-rose-500/30 rounded-xl shadow-[0_0_15px_rgba(244,63,94,0.2)]">
                <AlertTriangle className="w-6 h-6 text-rose-500" />
              </div>
              <h3 className="text-xl font-semibold text-white tracking-tight">
                {ENTERPRISE_GAP_ANALYSIS.title}
              </h3>
            </div>
            <div className="flex flex-col gap-4 relative z-10">
              {ENTERPRISE_GAP_ANALYSIS.items.map((item, idx) => (
                <div
                  key={idx}
                  className="flex flex-col gap-2 p-5 rounded-2xl bg-[#090b14]/60 hover:bg-[#090b14] border border-atc-border/50 hover:border-rose-500/30 transition-all shadow-[0_4px_20px_rgba(0,0,0,0.2)]"
                >
                  <div className="flex items-center gap-3">
                    <span
                      className={`px-2.5 py-1 rounded-md text-[10px] font-mono font-bold tracking-wide uppercase shadow-[0_0_10px_rgba(0,0,0,0.2)] ${
                        item.status === "Pass"
                          ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/30"
                          : item.status === "Warn"
                            ? "bg-amber-500/10 text-amber-400 border border-amber-500/30"
                            : item.status === "Fail"
                              ? "bg-rose-500/10 text-rose-400 border border-rose-500/30"
                              : "bg-atc-purple/10 text-atc-cyan border border-atc-cyan/30"
                      }`}
                    >
                      {item.status}
                    </span>
                    <span className="text-sm font-semibold text-slate-100">
                      {item.label}
                    </span>
                  </div>
                  <span className="text-sm text-slate-500 font-light leading-relaxed ml-14">
                    {item.description}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className="flex flex-col gap-8">
          <div className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl relative overflow-hidden group hover:border-atc-purple/30 transition-all duration-300">
            <div className="absolute top-0 right-0 w-64 h-64 bg-atc-purple/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-atc-purple/10 transition-opacity" />
            <div className="flex items-center gap-4 mb-8 relative z-10">
              <div className="p-3 bg-atc-purple/10 border border-atc-purple/30 rounded-xl shadow-[0_0_15px_rgba(162,89,255,0.2)]">
                <ClipboardList className="w-6 h-6 text-atc-purple" />
              </div>
              <h3 className="text-xl font-semibold text-white tracking-tight">
                {REPOSITORY_REVIEW.title}
              </h3>
            </div>
            <div className="flex flex-col gap-4 relative z-10">
              {REPOSITORY_REVIEW.items.map((item, idx) => (
                <div
                  key={idx}
                  className="flex flex-col gap-2 p-5 rounded-2xl bg-[#090b14]/60 hover:bg-[#090b14] border border-atc-border/50 hover:border-atc-purple/30 transition-all shadow-[0_4px_20px_rgba(0,0,0,0.2)]"
                >
                  <div className="flex items-center gap-3">
                    <span
                      className={`px-2.5 py-1 rounded-md text-[10px] font-mono font-bold tracking-wide uppercase shadow-[0_0_10px_rgba(0,0,0,0.2)] ${
                        item.status === "Pass"
                          ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/30"
                          : item.status === "Warn"
                            ? "bg-amber-500/10 text-amber-400 border border-amber-500/30"
                            : item.status === "Fail"
                              ? "bg-rose-500/10 text-rose-400 border border-rose-500/30"
                              : "bg-atc-purple/10 text-atc-cyan border border-atc-cyan/30"
                      }`}
                    >
                      {item.status}
                    </span>
                    <span className="text-sm font-semibold text-slate-100">
                      {item.label}
                    </span>
                  </div>
                  <span className="text-sm text-slate-500 font-light leading-relaxed ml-14">
                    {item.description}
                  </span>
                </div>
              ))}
            </div>
          </div>

          <div className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl relative overflow-hidden group hover:border-emerald-500/30 transition-all duration-300">
            <div className="absolute top-0 right-0 w-[400px] h-[400px] bg-emerald-500/5 rounded-full blur-[80px] pointer-events-none translate-x-1/3 -translate-y-1/3 group-hover:bg-emerald-500/10 transition-opacity" />
            <div className="relative z-10">
              <div className="flex items-center gap-4 mb-8">
                <div className="p-3 bg-emerald-500/10 border border-emerald-500/30 rounded-xl shadow-[0_0_15px_rgba(52,211,153,0.2)]">
                  <FileText className="w-6 h-6 text-emerald-400" />
                </div>
                <h3 className="text-xl font-semibold text-white tracking-tight">
                  Dokumentations-Score
                </h3>
              </div>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-5">
                {DOCUMENTATION_SCORE.map((metric, idx) => (
                  <div
                    key={idx}
                    className="flex flex-col justify-between gap-3 p-5 rounded-2xl bg-[#090b14]/60 border border-atc-border/50 hover:bg-[#090b14] hover:border-emerald-500/30 transition-all shadow-[0_4px_20px_rgba(0,0,0,0.2)]"
                  >
                    <div className="flex items-center justify-between gap-3">
                      <span className="text-[10px] text-slate-500 uppercase tracking-widest font-mono">
                        {metric.label}
                      </span>
                      <span
                        className={`text-xl font-light font-mono drop-shadow-[0_0_5px_rgba(0,0,0,0.5)] ${String(metric.score).includes("0/") || String(metric.score).includes("2/") || String(metric.score).includes("3/") ? "text-rose-400" : String(metric.score).includes("4/") || String(metric.score).includes("5/") || String(metric.score).includes("6/") ? "text-amber-400" : "text-emerald-400"}`}
                      >
                        {metric.score}
                      </span>
                    </div>
                    <span className="text-xs text-slate-300 font-light leading-relaxed">
                      {metric.insight}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>

          <div className="p-6 rounded-2xl bg-[#090b14]/60 backdrop-blur-md border border-atc-border/50 shadow-2xl relative overflow-hidden">
            <div className="relative z-10">
              <h3 className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-4 font-bold">
                ZUSÄTZLICH MÖGLICHE ANALYSEN
              </h3>
              <div className="flex flex-wrap gap-2">
                {ADDITIONAL_ANALYSIS.map((item, idx) => (
                  <span
                    key={idx}
                    className="px-3 py-1.5 text-xs font-medium text-slate-400 bg-[#060a16] border border-atc-border/50 rounded-lg shadow-[0_0_10px_rgba(0,0,0,0.2)]"
                  >
                    {item}
                  </span>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export function TechStackView() {
  return (
    <div className="flex flex-col gap-10 mt-8 pb-12">
      <div className="mb-4 flex items-center gap-4">
        <div className="w-12 h-12 rounded-2xl bg-atc-purple/10 border border-atc-purple/30 flex items-center justify-center text-atc-purple shadow-[0_0_15px_rgba(162,89,255,0.2)]">
          <Terminal className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">
            Tech Stack & Dependencies
          </h2>
          <p className="text-slate-500 font-light max-w-3xl leading-relaxed">
            Die erforderlichen atc-lang-Abhängigkeiten und Tech-Stacks zur
            Integration des Systems, gegliedert nach architektonischen Modulen.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        {ATC_OS_REQUIREMENTS.map((category) => (
          <div
            key={category.id}
            className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl relative overflow-hidden group hover:border-atc-cyan/30 transition-all duration-300 flex flex-col h-full"
          >
            <div className="absolute top-0 right-0 w-48 h-48 bg-atc-cyan/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-atc-cyan/10 transition-opacity" />

            <div className="flex items-center gap-4 mb-6 border-b border-atc-border/50 pb-6 relative z-10">
              <div className="w-12 h-12 rounded-xl bg-atc-cyan/10 flex items-center justify-center border border-atc-cyan/30 text-atc-cyan shrink-0 shadow-[0_0_15px_rgba(6,182,212,0.2)]">
                <Terminal className="w-5 h-5" />
              </div>
              <h3 className="text-lg font-semibold text-white tracking-tight">
                {category.title}
              </h3>
            </div>

            <div className="flex flex-col gap-3 flex-1 relative z-10">
              {category.packages.map((pkg, idx) => (
                <div
                  key={idx}
                  className="flex flex-col gap-1 p-3.5 rounded-xl bg-[#090b14]/60 hover:bg-[#090b14] border border-atc-border/50 hover:border-atc-cyan/30 transition-colors shadow-[0_4px_20px_rgba(0,0,0,0.2)] group/pkg"
                >
                  <span className="text-sm font-mono font-medium text-slate-300 group-hover/pkg:text-atc-cyan transition-colors">
                    {pkg.split(">=")[0]}
                  </span>
                  {pkg.includes(">=") && (
                    <span className="text-[10px] font-mono text-slate-500 font-bold uppercase tracking-widest pl-0.5">
                      ver &gt;= {pkg.split(">=")[1]}
                    </span>
                  )}
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>

      <div className="mt-6 p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md shadow-2xl border border-atc-border/80 flex flex-col md:flex-row gap-8 justify-between items-center relative overflow-hidden group hover:border-emerald-500/30 transition-colors">
        <div className="absolute top-0 right-0 w-64 h-64 bg-emerald-500/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-emerald-500/10 transition-opacity" />
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none mix-blend-overlay" />

        <div className="relative z-10 max-w-xl">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 rounded-lg bg-emerald-500/10 border border-emerald-500/30 shadow-[0_0_10px_rgba(52,211,153,0.2)]">
              <Terminal className="w-4 h-4 text-emerald-400" />
            </div>
            <h3 className="text-xl font-semibold text-white tracking-tight">
              Installation (requirements.txt)
            </h3>
          </div>
          <p className="text-sm text-slate-400 font-light leading-relaxed mt-3">
            Für Blockchain, AI & ML Module. Core Execution Environment basiert
            auf atc-lang.
          </p>
        </div>
        <div className="flex flex-col gap-2 p-5 bg-[#060a16] border border-atc-border/50 rounded-2xl shrink-0 relative z-10 w-full md:w-auto shadow-inner">
          <span className="text-[10px] text-slate-500 font-mono font-bold uppercase tracking-widest pl-1">
            CLI Command
          </span>
          <code className="text-emerald-400 font-mono text-sm bg-emerald-500/10 px-3 py-1.5 rounded-lg border border-emerald-500/20 shadow-[0_0_10px_rgba(52,211,153,0.1)]">
            pip install -r requirements.txt
          </code>
        </div>
      </div>
    </div>
  );
}

// Comments State Interface
interface Comment {
  id: string;
  author: string;
  initials: string;
  text: string;
  timestamp: string;
  replies?: Comment[];
}

function CommentItem({
  comment,
  onReply,
}: {
  key?: string | number;
  comment: Comment;
  onReply: (id: string, text: string) => void;
}) {
  const [isReplying, setIsReplying] = useState(false);
  const [replyText, setReplyText] = useState("");

  const handleReplySubmit = () => {
    if (replyText.trim()) {
      onReply(comment.id, replyText);
      setReplyText("");
      setIsReplying(false);
    }
  };

  return (
    <div className="flex flex-col gap-3">
      <div className="flex gap-4">
        <div className="w-10 h-10 rounded-xl bg-[#090b14] border border-atc-border/50 flex items-center justify-center text-slate-300 text-sm shrink-0 font-medium shadow-[0_0_10px_rgba(0,0,0,0.5)]">
          {comment.initials}
        </div>
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-1">
            <span className="font-semibold tracking-wide text-sm text-slate-200">
              {comment.author}
            </span>
            <span className="text-xs text-slate-500 font-mono">
              {comment.timestamp}
            </span>
          </div>
          <p className="text-sm text-slate-400 mb-2 leading-relaxed">
            {comment.text}
          </p>
          <div className="flex items-center gap-4">
            <button
              onClick={() => setIsReplying(!isReplying)}
              className="text-xs text-slate-500 hover:text-atc-purple transition-colors font-semibold flex items-center gap-1.5"
            >
              <CornerDownRight className="w-3.5 h-3.5" />
              Reply
            </button>
          </div>
        </div>
      </div>

      {isReplying && (
        <div className="flex gap-3 ml-14 mt-2">
          <div className="w-8 h-8 rounded-xl bg-atc-purple/10 flex items-center justify-center text-atc-purple text-xs shrink-0 font-medium border border-atc-purple/20">
            MW
          </div>
          <div className="flex-1 bg-[#090b14]/60 p-3 rounded-xl border border-atc-border/50 shadow-[0_4px_20px_rgba(0,0,0,0.2)] focus-within:border-atc-purple/50 transition-all">
            <textarea
              value={replyText}
              onChange={(e) => setReplyText(e.target.value)}
              placeholder="Write a reply..."
              className="w-full bg-transparent border-none outline-none text-sm resize-none text-slate-300 placeholder:text-slate-500"
              rows={2}
            />
            <div className="flex justify-end gap-2 mt-2">
              <button
                onClick={() => setIsReplying(false)}
                className="px-3 py-1.5 text-slate-500 hover:text-white text-xs font-semibold transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleReplySubmit}
                className="px-3 py-1.5 bg-atc-purple/20 text-atc-purple border border-atc-purple/30 rounded-lg text-xs font-bold hover:bg-atc-purple/30 transition-colors"
              >
                Reply
              </button>
            </div>
          </div>
        </div>
      )}

      {comment.replies && comment.replies.length > 0 && (
        <div className="ml-14 flex flex-col gap-6 mt-4 pl-4 border-l-2 border-[#090b14]/80">
          {comment.replies.map((reply) => (
            <CommentItem key={reply.id} comment={reply} onReply={onReply} />
          ))}
        </div>
      )}
    </div>
  );
}

function CommentSection({ sectionId }: { sectionId: string }) {
  // Mock initial comments state, keyed by sectionId
  const [commentsBySection, setCommentsBySection] = useState<
    Record<string, Comment[]>
  >({
    overview: [
      {
        id: "c1",
        author: "Sarah Jen",
        initials: "SJ",
        text: "Will the ATC Wallet support hardware ledger integration initially?",
        timestamp: "1 day ago",
        replies: [
          {
            id: "c1-r1",
            author: "A-TownChain CoreDev",
            initials: "CD",
            text: "Yes, Ledger support is planned for Phase 2 beta.",
            timestamp: "20 hours ago",
          },
        ],
      },
    ],
  });

  const [newCommentText, setNewCommentText] = useState("");

  const comments = commentsBySection[sectionId] || [];

  const handleAddComment = () => {
    if (!newCommentText.trim()) return;

    const newComment: Comment = {
      id: Math.random().toString(36).substr(2, 9),
      author: "Michael W.",
      initials: "MW",
      text: newCommentText,
      timestamp: "Just now",
    };

    setCommentsBySection((prev) => ({
      ...prev,
      [sectionId]: [...(prev[sectionId] || []), newComment],
    }));
    setNewCommentText("");
  };

  const handleReply = (parentId: string, text: string) => {
    const newReply: Comment = {
      id: Math.random().toString(36).substr(2, 9),
      author: "Michael W.",
      initials: "MW",
      text,
      timestamp: "Just now",
    };

    setCommentsBySection((prev) => {
      const sectionComments = [...(prev[sectionId] || [])];

      // Basic recursive finding for n-level nesting (though mostly 1 or 2 levels are used here)
      const addReplyToNode = (nodes: Comment[]): boolean => {
        for (let node of nodes) {
          if (node.id === parentId) {
            node.replies = [...(node.replies || []), newReply];
            return true;
          }
          if (node.replies && addReplyToNode(node.replies)) {
            return true;
          }
        }
        return false;
      };

      addReplyToNode(sectionComments);

      return {
        ...prev,
        [sectionId]: sectionComments,
      };
    });
  };

  return (
    <div className="p-8 rounded-3xl bg-[#090b14]/60 backdrop-blur-md border border-atc-border/50 shadow-inner mt-4 relative overflow-hidden group hover:border-atc-purple/30 transition-all">
      <div className="absolute top-0 right-0 w-64 h-64 bg-atc-purple/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-atc-purple/10 transition-opacity" />
      <h3 className="text-xl font-semibold tracking-tight text-white mb-8 relative z-10 flex items-center gap-3">
        <MessageSquare className="w-5 h-5 text-slate-500" />
        Discussion & Comments
      </h3>

      <div className="flex flex-col gap-8 mb-8 relative z-10">
        {comments.map((comment) => (
          <CommentItem
            key={comment.id}
            comment={comment}
            onReply={handleReply}
          />
        ))}
        {comments.length === 0 && (
          <div className="text-sm text-slate-500 italic text-center py-8">
            No comments yet. Start the discussion!
          </div>
        )}
      </div>

      <div className="flex flex-col gap-4 relative z-10">
        <div className="flex gap-4 border-t border-atc-border/50 pt-8 mt-4">
          <div className="w-10 h-10 rounded-xl bg-atc-cyan/10 flex items-center justify-center text-atc-cyan text-sm shrink-0 font-medium border border-atc-cyan/30 shadow-[0_0_10px_rgba(6,182,212,0.2)]">
            MW
          </div>
          <div className="flex-1 bg-[#060a16] p-4 rounded-xl border border-atc-border/50 shadow-[0_4px_20px_rgba(0,0,0,0.2)] focus-within:border-atc-cyan/50 focus-within:ring-1 focus-within:ring-atc-cyan/20 transition-all">
            <textarea
              value={newCommentText}
              onChange={(e) => setNewCommentText(e.target.value)}
              placeholder="Add a new comment to this documentation page..."
              className="w-full bg-transparent border-none outline-none text-sm resize-none text-slate-300 placeholder:text-slate-500"
              rows={3}
            />
            <div className="flex justify-end mt-2 pt-2 border-t border-atc-border/30">
              <button
                onClick={handleAddComment}
                className="px-4 py-2 bg-atc-cyan/10 text-atc-cyan border border-atc-cyan/30 rounded-lg text-xs font-bold hover:bg-atc-cyan/20 transition-colors tracking-wide uppercase"
              >
                Post Comment
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export function WikiView({
  setActiveTab,
  wikiContent = WIKI_CONTENT,
  addBookmark,
}: {
  setActiveTab: (tab: any) => void;
  wikiContent?: any[];
  addBookmark?: (title: string, type: string, path: string) => void;
}) {
  // Use useEffect to update activeSection when wikiContent changes if needed, but for simplicity:
  const [activeSection, setActiveSection] = useState(wikiContent[0]?.id);
  const [showHistoryModal, setShowHistoryModal] = useState(false);

  // Sync active section if wikiContent prop changes entirely
  useEffect(() => {
    if (
      wikiContent.length > 0 &&
      !wikiContent.find((s) => s.id === activeSection)
    ) {
      setActiveSection(wikiContent[0].id);
    }
  }, [wikiContent, activeSection]);

  const selectedData = wikiContent.find((s) => s.id === activeSection);

  const renderContentWithLinks = (text: string) => {
    const regex = /\[\[(.*?)\|(.*?)\]\]/g;
    const parts = [];
    let lastIndex = 0;
    let match;

    while ((match = regex.exec(text)) !== null) {
      if (match.index > lastIndex) {
        parts.push(text.substring(lastIndex, match.index));
      }
      const target = match[1];
      const label = match[2];

      parts.push(
        <button
          key={match.index}
          onClick={() => {
            if (target.startsWith("wiki:")) {
              setActiveSection(target.split(":")[1]);
            } else if (target.startsWith("tab:")) {
              setActiveTab(target.split(":")[1] as any);
            }
          }}
          className="text-atc-purple hover:text-purple-300 underline underline-offset-4 decoration-purple-500/30 hover:decoration-purple-500/80 transition-all font-medium inline"
        >
          {label}
        </button>,
      );
      lastIndex = regex.lastIndex;
    }

    if (lastIndex < text.length) {
      parts.push(text.substring(lastIndex));
    }

    return parts.length > 0 ? parts : text;
  };

  return (
    <div className="flex flex-col lg:flex-row gap-8 mt-8 pb-12">
      {/* Table of Contents (Sidebar) */}
      <div className="w-full lg:w-72 shrink-0">
        <div className="sticky top-8 flex flex-col p-4 rounded-2xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl">
          <div className="flex items-center gap-3 mb-6 px-2">
            <div className="p-2.5 rounded-lg bg-atc-purple/10 border border-atc-purple/30 shadow-[0_0_10px_rgba(162,89,255,0.2)]">
              <Bookmark className="w-5 h-5 text-atc-purple" />
            </div>
            <h3 className="text-lg font-mono text-white tracking-tight">
              Inhaltsverzeichnis
            </h3>
          </div>

          <nav className="flex flex-col gap-4">
            {Object.entries<any>(
              wikiContent.reduce(
                (acc, section) => {
                  const cat = (section as any).category || "Allgemein";
                  const subcat = (section as any).subcategory;
                  if (!acc[cat]) acc[cat] = { _items: [] };
                  if (subcat) {
                    if (!acc[cat][subcat]) acc[cat][subcat] = [];
                    acc[cat][subcat].push(section);
                  } else {
                    acc[cat]._items.push(section);
                  }
                  return acc;
                },
                {} as Record<string, any>,
              ),
            ).map(([category, content]) => (
              <div key={category} className="flex flex-col gap-1">
                <h4 className="text-[10px] font-mono text-slate-500 uppercase tracking-widest px-3 mb-1 mt-1 font-bold">
                  {category}
                </h4>
                {content._items.map((section: any) => (
                  <button
                    key={section.id}
                    onClick={() => setActiveSection(section.id)}
                    className={`flex items-center gap-3 w-full px-3 py-2.5 rounded-lg text-left transition-all ${
                      activeSection === section.id
                        ? "bg-atc-purple/10 border border-atc-purple/30 text-atc-purple font-semibold shadow-[0_0_10px_rgba(162,89,255,0.1)]"
                        : "border border-transparent text-slate-500 hover:text-slate-300 hover:bg-[#090b14]"
                    }`}
                  >
                    <ChevronRight
                      className={`w-4 h-4 transition-transform ${activeSection === section.id ? "text-atc-purple translate-x-1" : "text-slate-500"}`}
                    />
                    <span className="text-sm">{section.title}</span>
                  </button>
                ))}
                {Object.entries(content)
                  .filter(([k]) => k !== "_items")
                  .map(([subcat, sections]) => (
                    <div key={subcat} className="flex flex-col">
                      <div className="flex items-center gap-2 px-3 py-2 text-slate-500">
                        <Folder className="w-3.5 h-3.5 text-slate-500" />
                        <span className="text-[11px] font-bold uppercase tracking-wider">
                          {subcat}
                        </span>
                      </div>
                      <div className="flex flex-col gap-0.5 ml-4 pl-1 mt-0.5 border-l border-atc-border/50">
                        {(sections as any[]).map((section: any) => (
                          <button
                            key={section.id}
                            onClick={() => setActiveSection(section.id)}
                            className={`flex items-center gap-3 w-full px-3 py-2 rounded-lg text-left transition-all ${
                              activeSection === section.id
                                ? "bg-atc-purple/10 text-atc-purple font-semibold shadow-[0_0_10px_rgba(162,89,255,0.1)]"
                                : "text-slate-500 hover:text-slate-300 hover:bg-[#090b14]"
                            }`}
                          >
                            <span className="text-sm border-l-2 pl-2 transition-colors border-transparent hover:border-slate-500">
                              {section.title}
                            </span>
                          </button>
                        ))}
                      </div>
                    </div>
                  ))}
              </div>
            ))}
          </nav>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 min-w-0">
        <AnimatePresence mode="wait">
          <motion.div
            key={activeSection}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            transition={{ duration: 0.2 }}
            className="flex flex-col gap-6"
          >
            {selectedData && (
              <div className="p-8 rounded-3xl bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 shadow-2xl relative overflow-hidden group">
                <div className="absolute top-0 right-0 w-64 h-64 bg-atc-purple/5 rounded-full blur-[80px] pointer-events-none group-hover:bg-atc-purple/10 transition-opacity" />

                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8 pb-6 border-b border-atc-border/50 relative z-10">
                  <div className="flex items-center gap-3 text-sm text-slate-500 font-medium">
                    <div className="p-1.5 rounded-md bg-atc-primary/10 border border-atc-primary/20">
                      <BookOpen className="w-4 h-4 text-slate-400" />
                    </div>
                    <span className="tracking-wide">Documentation</span>
                    <ChevronRight className="w-3 h-3 text-slate-600" />
                    <span className="text-slate-200">{selectedData.title}</span>
                  </div>
                  <div className="flex items-center gap-2">
                    {addBookmark && (
                      <button
                        onClick={() =>
                          addBookmark(selectedData.title, "WIKI", "wiki")
                        }
                        className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-[#060a16] border border-atc-border/50 hover:border-atc-cyan/50 text-slate-400 hover:text-atc-cyan transition-colors text-sm font-medium shadow-[0_0_10px_rgba(0,0,0,0.2)]"
                      >
                        <Bookmark className="w-4 h-4" />
                        <span className="hidden sm:inline">Bookmark</span>
                      </button>
                    )}
                    <button className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-[#060a16] border border-atc-border/50 hover:border-slate-500 text-slate-400 hover:text-white transition-colors text-sm font-medium shadow-[0_0_10px_rgba(0,0,0,0.2)]">
                      <Share2 className="w-4 h-4" />
                      <span className="hidden sm:inline">Share</span>
                    </button>
                    <button className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-[#060a16] border border-atc-border/50 hover:border-slate-500 text-slate-400 hover:text-white transition-colors text-sm font-medium shadow-[0_0_10px_rgba(0,0,0,0.2)]">
                      <Download className="w-4 h-4" />
                      <span className="hidden sm:inline">Export PDF</span>
                    </button>
                  </div>
                </div>

                <h2 className="text-3xl font-mono text-white mb-6 font-medium relative z-10 tracking-tight">
                  {selectedData.title}
                </h2>

                {selectedData.text && (
                  <p className="text-slate-400 text-lg leading-relaxed mb-8 relative z-10">
                    {renderContentWithLinks(selectedData.text)}
                  </p>
                )}

                {selectedData.images && selectedData.images.length > 0 && (
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-8 mb-8 relative z-10">
                    {selectedData.images.map((img: any, idx: number) => (
                      <div key={idx} className="flex flex-col gap-2 p-2 rounded-2xl bg-[#060a16] border border-atc-border/50 shadow-[0_4px_20px_rgba(0,0,0,0.2)]">
                        <img src={img.url} alt={img.alt} className="w-full h-48 md:h-64 rounded-xl object-cover" />
                        {img.caption && <span className="text-xs text-slate-500 font-mono text-center px-4 pb-2 pt-1">{img.caption}</span>}
                      </div>
                    ))}
                  </div>
                )}

                {selectedData.table && (
                  <div className="flex flex-col gap-3 font-mono relative z-10">
                    {selectedData.table.map((row, idx) => (
                      <div
                        key={idx}
                        className="flex flex-col md:flex-row md:items-center gap-4 p-5 rounded-2xl bg-[#060a16] border border-atc-border/50 hover:border-atc-purple/30 transition-colors shadow-[0_4px_20px_rgba(0,0,0,0.2)]"
                      >
                        <div className="w-full md:w-1/3 shrink-0">
                          <span className="text-slate-200 font-semibold">
                            {row.component}
                          </span>
                        </div>
                        <div className="flex-1">
                          <span className="text-sm text-slate-500">
                            {row.desc}
                          </span>
                        </div>
                        <div className="shrink-0 flex items-center">
                          <span
                            className={`px-2.5 py-1.5 rounded-md text-[10px] tracking-widest font-bold uppercase border shadow-[0_0_10px_rgba(0,0,0,0.2)] ${
                              row.status.includes("Active") ||
                              row.status.includes("v2.0") ||
                              row.status.includes("v2.1") ||
                              row.status.includes("Online")
                                ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/30"
                                : row.status.includes("Draft") ||
                                    row.status.includes("Phase 3")
                                  ? "bg-amber-500/10 text-amber-400 border-amber-500/30"
                                  : "bg-indigo-500/10 text-indigo-400 border-indigo-500/30"
                            }`}
                          >
                            {row.status}
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                )}

                {selectedData.ascii && (
                  <div className="mt-8 rounded-2xl bg-[#060a16] border border-atc-border/50 overflow-x-auto p-6 relative z-10 shadow-inner">
                    <div className="mb-4 flex items-center gap-2 text-slate-500">
                      <Terminal className="w-4 h-4" />
                      <span className="text-xs font-mono uppercase tracking-widest font-bold">
                        Architecture Map
                      </span>
                    </div>
                    <pre className="text-atc-cyan/80 font-mono text-sm leading-normal">
                      <code>{selectedData.ascii}</code>
                    </pre>
                  </div>
                )}

                <div className="mt-12 pt-6 border-t border-atc-border/50 flex flex-col sm:flex-row justify-between items-center text-sm text-slate-500 gap-4 relative z-10">
                  <div className="flex items-center gap-4">
                    <div className="flex items-center gap-2">
                      <div className="w-6 h-6 rounded-full bg-atc-purple/20 flex items-center justify-center text-atc-purple">
                        <User className="w-3 h-3" />
                      </div>
                      <span>
                        Last edited by{" "}
                        <span className="font-semibold text-slate-300">
                          A-TownChain CoreDev
                        </span>
                      </span>
                    </div>
                    <span className="hidden sm:inline text-slate-600">•</span>
                    <span>Modified: 2 hrs ago</span>
                  </div>
                  <div className="flex gap-4">
                    <button
                      onClick={() => setShowHistoryModal(true)}
                      className="cursor-pointer hover:text-atc-cyan transition-colors font-medium flex items-center gap-1.5"
                    >
                      <Clock className="w-4 h-4" /> Revision History
                    </button>
                    <span className="hidden sm:inline text-slate-600">•</span>
                    <button className="cursor-pointer hover:text-white transition-colors font-medium flex items-center gap-1.5">
                      <FileText className="w-4 h-4" /> Page Info
                    </button>
                  </div>
                </div>
              </div>
            )}

            {/* Thread/Comments Section below page */}
            {selectedData && <CommentSection sectionId={selectedData.id} />}
          </motion.div>
        </AnimatePresence>
      </div>

      {/* Revision History Modal */}
      <AnimatePresence>
        {showHistoryModal && selectedData && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="absolute inset-0 bg-[#060a16]/40 backdrop-blur-sm"
              onClick={() => setShowHistoryModal(false)}
            />
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 10 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 10 }}
              className="relative w-full max-w-lg bg-atc-card rounded-xl shadow-2xl overflow-hidden flex flex-col"
            >
              <div className="flex items-center justify-between p-5 border-b border-atc-border">
                <div className="flex items-center gap-2 text-white">
                  <Clock className="w-5 h-5 text-atc-cyan" />
                  <h3 className="font-medium text-lg tracking-tight">
                    Revision History
                  </h3>
                </div>
                <button
                  onClick={() => setShowHistoryModal(false)}
                  className="p-1.5 text-slate-500 hover:text-slate-500 hover:bg-atc-card-hover rounded-lg transition-colors"
                >
                  <X className="w-4 h-4" />
                </button>
              </div>

              <div className="p-5 max-h-[60vh] overflow-y-auto">
                {selectedData.revisions && selectedData.revisions.length > 0 ? (
                  <div className="relative border-l border-atc-border ml-3 pl-6 flex flex-col gap-8 py-2">
                    {selectedData.revisions.map((rev: any, idx: number) => (
                      <div key={idx} className="relative">
                        <div className="absolute -left-[31px] top-1 w-4 h-4 rounded-full bg-atc-cyan/10 border-2 border-white flex items-center justify-center shadow-[0_0_15px_rgba(0,0,0,0.5)]">
                          <div className="w-1.5 h-1.5 rounded-full bg-atc-purple" />
                        </div>
                        <div className="flex flex-col gap-1 text-sm">
                          <p className="text-slate-100">{rev.message}</p>
                          <div className="flex items-center gap-2 text-xs text-slate-500 font-medium">
                            <span className="text-atc-cyan font-mono tracking-tight">
                              {rev.editor}
                            </span>
                            <span className="text-slate-300">•</span>
                            <span>
                              {new Date(rev.timestamp).toLocaleString(
                                undefined,
                                {
                                  year: "numeric",
                                  month: "short",
                                  day: "numeric",
                                  hour: "2-digit",
                                  minute: "2-digit",
                                },
                              )}
                            </span>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-center py-8 flex flex-col items-center justify-center text-slate-500">
                    <FileText className="w-8 h-8 text-slate-300 mb-3" />
                    <p className="text-sm">
                      No revision history available for this document.
                    </p>
                  </div>
                )}
              </div>

              <div className="p-4 bg-atc-bg border-t border-atc-border flex justify-end">
                <button
                  onClick={() => setShowHistoryModal(false)}
                  className="px-4 py-2 bg-atc-card border border-atc-border hover:bg-atc-bg text-slate-300 text-sm font-medium rounded-lg transition-colors shadow-[0_0_15px_rgba(0,0,0,0.5)]"
                >
                  Close
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
    </div>
  );
}
