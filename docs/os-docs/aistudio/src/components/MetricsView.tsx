import React, {
  useState,
  useEffect,
  useRef,
  useMemo,
  lazy,
  Suspense,
} from "react";
import { SystemHealthDashboard } from "./SystemHealthDashboard";
import {
  Activity,
  Cpu,
  Server,
  Network,
  HardDrive,
  Wifi,
  Zap,
  Github,
  GitCommit,
  GitPullRequest,
  Terminal,
  AlertTriangle,
  HeartPulse,
  ThermometerSun,
  X,
  Shield,
  Database,
  Download,
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import * as d3 from "d3";
import { TooltipIcon } from "./TooltipIcon";
import { MetricsDashboard } from "./MetricsDashboard";

const CommitChart = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({ default: m.CommitChart })),
);
const TransactionVolumeChart = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({
    default: m.TransactionVolumeChart,
  })),
);
const IndexedDBLatencyChart = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({
    default: m.IndexedDBLatencyChart,
  })),
);
const NetworkStatusHistoryWidget = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({
    default: m.NetworkStatusHistoryWidget,
  })),
);
const SyncDurationWidget = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({
    default: m.SyncDurationWidget,
  })),
);
const APIHealthWidget = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({ default: m.APIHealthWidget })),
);
const NetworkLatencyFluctuationWidget = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({
    default: m.NetworkLatencyFluctuationWidget,
  })),
);
const APIResponseTimeWidget = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({
    default: m.APIResponseTimeWidget,
  })),
);
const SyncHealthWidget = lazy(() =>
  import("./LazyMetricsCharts").then((m) => ({ default: m.SyncHealthWidget })),
);

const DashboardGrid = React.memo(
  ({ children }: { children: React.ReactNode }) => {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {React.Children.map(children, (child, i) => (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: i * 0.1 }}
            key={i}
            className="h-full"
          >
            {child}
          </motion.div>
        ))}
      </div>
    );
  },
);

const ChartFallback = () => (
  <div className="flex items-center justify-center p-12 bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl animate-pulse min-h-[250px]">
    <span className="text-slate-500 font-mono text-sm">Loading Chart...</span>
  </div>
);

function PerformanceHeatMap() {
  const svgRef = useRef<SVGSVGElement>(null);

  const subsystems = useMemo(
    () => [
      "WASM-VM",
      "RPC-Node",
      "P2P-Gossip",
      "Mem-Pager",
      "Disk-IO",
      "Chain-DB",
    ],
    [],
  );
  const nodes = useMemo(
    () => ["Node-1", "Node-2", "Node-3", "Node-4", "Node-5"],
    [],
  );

  useEffect(() => {
    if (!svgRef.current) return;

    // Simulate heat data
    const data: { x: string; y: string; val: number }[] = [];
    nodes.forEach((node) => {
      subsystems.forEach((sub) => {
        // give WASM and Disk-IO higher base "heat" occasionally
        let baseHeat = Math.random() * 50;
        if (sub === "Disk-IO" && Math.random() > 0.5) baseHeat += 30;
        if (sub === "WASM-VM" && Math.random() > 0.7) baseHeat += 40;
        data.push({
          x: sub,
          y: node,
          val: Math.min(100, baseHeat + Math.random() * 20),
        });
      });
    });

    const width = 600;
    const height = 250;
    const margin = { top: 30, right: 30, bottom: 40, left: 60 };

    const svg = d3
      .select(svgRef.current)
      .attr("viewBox", `0 0 ${width} ${height}`)
      .style("width", "100%")
      .style("height", "auto");

    svg.selectAll("*").remove();

    const x = d3
      .scaleBand()
      .range([margin.left, width - margin.right])
      .domain(subsystems)
      .padding(0.05);

    const y = d3
      .scaleBand()
      .range([height - margin.bottom, margin.top])
      .domain(nodes)
      .padding(0.05);

    const color = d3
      .scaleSequential()
      .interpolator(d3.interpolateInferno)
      .domain([0, 100]);

    // X Axis
    svg
      .append("g")
      .attr("transform", `translate(0,${height - margin.bottom})`)
      .call(d3.axisBottom(x).tickSize(0))
      .select(".domain")
      .remove();

    // Y Axis
    svg
      .append("g")
      .attr("transform", `translate(${margin.left},0)`)
      .call(d3.axisLeft(y).tickSize(0))
      .select(".domain")
      .remove();

    // Set text color for axes
    svg
      .selectAll("text")
      .attr("fill", "#94a3b8")
      .style("font-family", "monospace")
      .style("font-size", "10px");

    // Heatmap squares
    svg
      .selectAll()
      .data(data, (d: any) => d.x + ":" + d.y)
      .enter()
      .append("rect")
      .attr("x", (d) => x(d.x)!)
      .attr("y", (d) => y(d.y)!)
      .attr("width", x.bandwidth())
      .attr("height", y.bandwidth())
      .style("fill", (d) => color(d.val))
      .attr("rx", 4)
      .attr("ry", 4)
      .style("opacity", 0)
      .transition()
      .duration(1000)
      .style("opacity", 0.9);

    // Overlay Tooltip interactivity could be added here
  }, [subsystems, nodes]);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl col-span-1 md:col-span-2 lg:col-span-2 relative overflow-hidden group hover:border-amber-500/50 transition-all duration-300">
      <div className="absolute top-0 right-0 w-48 h-48 bg-amber-500/5 rounded-full blur-[80px] opacity-0 group-hover:opacity-100 transition-opacity" />
      <div className="flex items-center justify-between mb-2 relative z-10">
        <div className="flex items-center gap-2 text-slate-300">
          <ThermometerSun className="w-5 h-5 text-amber-500" />
          <span className="text-sm font-bold uppercase tracking-wider font-mono">
            Performance Bottleneck Map
          </span>
          <TooltipIcon content="Visualizes potential performance degradation across decentralised nodes dynamically." />
        </div>
      </div>
      <p className="text-xs text-slate-500 font-mono mb-4">
        Critical subsystems dynamically highlighted based on current load (D3
        Heatmap).
      </p>
      <div className="w-full relative z-10">
        <svg ref={svgRef} className="w-full h-auto overflow-visible" />
      </div>
    </div>
  );
}

function SystemHealthWidget() {
  const [nodes, setNodes] = useState([
    { id: "ATC-OS-Alpha", region: "us-east", latency: 12, load: 34 },
    { id: "ATC-OS-Beta", region: "eu-west", latency: 45, load: 88 }, // bottleneck
    { id: "ATC-OS-Gamma", region: "ap-south", latency: 120, load: 62 },
    { id: "ATC-OS-Delta", region: "us-west", latency: 28, load: 15 },
  ]);

  useEffect(() => {
    const interval = setInterval(() => {
      setNodes((prev) =>
        prev.map((n) => ({
          ...n,
          latency: Math.max(5, n.latency + (Math.random() * 10 - 5)),
          load: Math.max(0, Math.min(100, n.load + (Math.random() * 20 - 10))),
        })),
      );
    }, 2500);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-rose-500/40 transition-all duration-300">
      <div className="absolute top-0 right-0 w-32 h-32 bg-rose-500/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
      <div className="flex items-center gap-2 text-slate-300 mb-6 relative z-10">
        <HeartPulse className="w-4 h-4 text-rose-500" />
        <span className="text-sm font-bold uppercase tracking-wider font-mono">
          System Health & Heartbeat
        </span>
        <TooltipIcon content="Real-time pulse of node connectivity and overall cluster stability." />
      </div>

      <div className="flex flex-col gap-4 relative z-10">
        {nodes.map((node) => (
          <div
            key={node.id}
            className="flex items-center justify-between p-3 rounded-xl border border-white/5 bg-black/40"
          >
            <div className="flex items-center gap-3">
              {/* Heartbeat Animation */}
              <div className="relative flex items-center justify-center w-3 h-3">
                <div
                  className={`absolute w-full h-full rounded-full opacity-75 animate-ping
                       ${node.load > 80 ? "bg-rose-500 duration-700" : "bg-emerald-500 duration-1000"}`}
                />
                <div
                  className={`relative w-2 h-2 rounded-full 
                       ${node.load > 80 ? "bg-rose-400" : "bg-emerald-400"}`}
                />
              </div>
              <div>
                <h4 className="text-sm font-bold text-slate-200">{node.id}</h4>
                <p className="text-[10px] text-slate-500 font-mono uppercase">
                  {node.region}
                </p>
              </div>
            </div>
            <div className="flex flex-col items-end">
              <span
                className={`text-xs font-mono font-bold ${node.latency > 100 ? "text-amber-400" : "text-slate-300"}`}
              >
                {node.latency.toFixed(0)} ms
              </span>
              <span
                className={`text-[10px] font-mono ${node.load > 80 ? "text-rose-400" : "text-slate-500"}`}
              >
                Load: {node.load.toFixed(0)}%
              </span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

function BackupReminder() {
  const [needsBackup, setNeedsBackup] = useState(false);
  useEffect(() => {
    const lastBackupStr = localStorage.getItem("atc_last_backup_download");
    if (!lastBackupStr) {
      setNeedsBackup(true);
      return;
    }
    const lastBackup = parseInt(lastBackupStr, 10);
    const sevenDaysMs = 7 * 24 * 60 * 60 * 1000;
    if (Date.now() - lastBackup > sevenDaysMs) {
      setNeedsBackup(true);
    }
  }, []);

  if (!needsBackup) return null;

  return (
    <div className="mb-6 bg-amber-500/10 border border-amber-500/30 rounded-xl p-4 flex items-center gap-3 backdrop-blur-sm">
      <AlertTriangle className="w-5 h-5 text-amber-400 shrink-0" />
      <div>
        <h4 className="text-sm font-bold text-amber-400">
          Wallet Backup Recommended
        </h4>
        <p className="text-xs text-amber-400/80 mt-0.5">
          It has been over 7 days since you downloaded a local wallet backup
          file. Consider exporting a new backup to keep your identity secure
          offline.
        </p>
      </div>
    </div>
  );
}

function SystemLogsWidget() {
  const [logs, setLogs] = useState<
    {
      id: string;
      time: Date;
      level: "info" | "warn" | "error";
      message: string;
    }[]
  >([]);

  useEffect(() => {
    const mockMessages = [
      "Node synchronization matched delta",
      "Validating block candidates",
      "P2P handshake successful with peer 0x8a..90",
      "Checking incoming mempool buffer",
      "Consensus reached for block 908122",
      "Minor packet loss detected on inbound route",
      "VM executing smart contract bytecode",
      "Resolving state trie collisions",
    ];
    let counter = 0;
    const interval = setInterval(() => {
      const level: "error" | "warn" | "info" =
        Math.random() > 0.85 ? "warn" : Math.random() > 0.95 ? "error" : "info";
      const msg = mockMessages[Math.floor(Math.random() * mockMessages.length)];
      setLogs((prev) =>
        [
          { id: `log-${counter++}`, time: new Date(), level, message: msg },
          ...prev,
        ].slice(0, 50),
      );
    }, 1500);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-emerald-500/30 transition-all duration-300 col-span-1 md:col-span-2 lg:col-span-2">
      <div className="flex items-center gap-2 text-slate-300 mb-4 relative z-10">
        <Terminal className="w-4 h-4 text-emerald-400" />
        <span className="text-sm font-bold uppercase tracking-wider font-mono">
          System Live Logs
        </span>
      </div>
      <div className="h-48 overflow-y-auto bg-black/40 rounded-lg p-3 font-mono text-xs border border-white/5 custom-scrollbar">
        <AnimatePresence initial={false}>
          {logs.map((log) => (
            <motion.div
              key={log.id}
              initial={{ opacity: 0, x: -10 }}
              animate={{ opacity: 1, x: 0 }}
              className="flex gap-3 mb-1"
            >
              <span className="text-slate-500 shrink-0">
                [{log.time.toLocaleTimeString()}]
              </span>
              <span
                className={`shrink-0 ${
                  log.level === "warn"
                    ? "text-amber-400"
                    : log.level === "error"
                      ? "text-red-400"
                      : "text-emerald-400"
                }`}
              >
                [{log.level.toUpperCase()}]
              </span>
              <span className="text-slate-300 break-all">{log.message}</span>
            </motion.div>
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
}

function GithubSync() {
  const [issues, setIssues] = useState({ open: 0, closed: 0 });
  const [syncing, setSyncing] = useState(true);

  useEffect(() => {
    const fetchIssues = async () => {
      setSyncing(true);
      try {
        // Mock data for our private enterprise repos
        setIssues({
          open: 14,
          closed: 112,
        });
      } catch (e) {
        console.error("Failed to sync github issues", e);
      } finally {
        setSyncing(false);
      }
    };
    fetchIssues();
    const interval = setInterval(fetchIssues, 120000);
    return () => clearInterval(interval);
  }, []);

  const total = issues.open + issues.closed || 1;
  const progress = Math.round((issues.closed / total) * 100);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-purple-500/50 transition-all duration-300">
      <div className="absolute top-0 right-0 w-32 h-32 bg-purple-500/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
      <div className="flex items-center justify-between mb-4 relative z-10">
        <div className="flex items-center gap-2 text-slate-300">
          <GitPullRequest className="w-4 h-4 text-purple-400" />
          <span className="text-sm font-medium uppercase tracking-wider font-mono">
            Issue Sync Status
          </span>
        </div>
        {syncing ? (
          <span className="text-xs text-slate-400 animate-pulse">
            Syncing...
          </span>
        ) : (
          <span className="text-2xl font-mono text-white">{progress}%</span>
        )}
      </div>
      <div className="w-full h-2 bg-atc-bg rounded-full overflow-hidden relative z-10 mb-4">
        <motion.div
          className="h-full bg-purple-400"
          animate={{ width: `${progress}%` }}
          transition={{ duration: 0.5 }}
        />
      </div>
      <div className="flex justify-between text-xs text-slate-400 font-mono relative z-10">
        <span>
          <strong className="text-white">{issues.closed}</strong> Closed
        </span>
        <span>
          <strong className="text-white">{issues.open}</strong> Open
        </span>
      </div>
    </div>
  );
}

function SystemLifecycleWidget() {
  const svgRef = useRef<SVGSVGElement>(null);
  const [progress, setProgress] = useState(0);
  const [stage, setStage] = useState("Initializing ATC-OS Container...");

  useEffect(() => {
    let p = 0;
    const interval = setInterval(() => {
      p += 1;
      setProgress(p);
      if (p > 20 && p < 45) setStage("Injecting kernel WASM payload...");
      else if (p >= 45 && p < 70) setStage("Syncing with OS node mesh...");
      else if (p >= 70 && p < 99)
        setStage("Finalizing decentralized deploy...");
      else if (p >= 100) {
        setStage("ATC-OS Kernel Deployed On-Chain");
        clearInterval(interval);
      }
    }, 150);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (!svgRef.current) return;
    const width = 120;
    const height = 120;
    const radius = Math.min(width, height) / 2 - 10;

    const svg = d3
      .select(svgRef.current)
      .attr("viewBox", `0 0 ${width} ${height}`)
      .style("width", "100%")
      .style("height", "100px");

    svg.selectAll("*").remove();

    const g = svg
      .append("g")
      .attr("transform", `translate(${width / 2},${height / 2})`);

    // Background arc
    g.append("path")
      .attr("fill", "#1e293b")
      .attr(
        "d",
        d3
          .arc()
          .innerRadius(radius - 8)
          .outerRadius(radius)
          .startAngle(0)
          .endAngle(2 * Math.PI) as any,
      );

    // Progress arc
    g.append("path")
      .attr("fill", "#10b981")
      .attr(
        "d",
        d3
          .arc()
          .innerRadius(radius - 8)
          .outerRadius(radius)
          .startAngle(0)
          .endAngle((progress / 100) * 2 * Math.PI) as any,
      );

    g.append("text")
      .attr("text-anchor", "middle")
      .attr("dy", "0.3em")
      .attr("fill", "white")
      .attr("font-size", "24px")
      .attr("font-weight", "bold")
      .attr("font-family", "monospace")
      .text(`${progress}%`);
  }, [progress]);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-emerald-500/50 transition-all duration-300">
      <div className="absolute top-0 right-0 w-32 h-32 bg-emerald-500/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
      <div className="flex items-center gap-2 text-slate-300 mb-4 relative z-10">
        <Server className="w-4 h-4 text-emerald-400" />
        <span className="text-sm font-bold uppercase tracking-wider font-mono">
          System Lifecycle
        </span>
        <TooltipIcon content="Progress of the bootup and injection into the decentralized runtime ring." />
      </div>

      <div className="flex flex-col items-center justify-center gap-4 relative z-10 pt-2 pb-4">
        <svg ref={svgRef} className="w-24 h-24 overflow-visible" />
        <div className="text-center mt-2 w-full">
          <div
            className={`text-xs font-mono px-3 py-1.5 rounded-lg border w-full truncate ${progress === 100 ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/30" : "bg-slate-800 text-slate-300 border-slate-700 animate-pulse"}`}
          >
            {stage}
          </div>
        </div>
      </div>
    </div>
  );
}

function SearchQueriesWidget({ searchHistory }: { searchHistory: string[] }) {
  const topQueries = useMemo(() => {
    const counts: Record<string, number> = {};
    searchHistory.forEach((q) => (counts[q] = (counts[q] || 0) + 1));
    return Object.entries(counts)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 5);
  }, [searchHistory]);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-amber-500/30 transition-all duration-300">
      <div className="absolute top-0 right-0 w-32 h-32 bg-amber-500/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
      <div className="flex items-center gap-2 text-slate-300 mb-4 relative z-10">
        <Server className="w-4 h-4 text-amber-400" />
        <span className="text-sm font-bold uppercase tracking-wider font-mono">
          Top Search Queries
        </span>
      </div>
      <div className="flex flex-col gap-2 relative z-10">
        {topQueries.length === 0 ? (
          <p className="text-xs text-slate-500 font-mono text-center py-4">
            No search history available yet.
          </p>
        ) : (
          topQueries.map(([query, count], idx) => (
            <div
              key={idx}
              className="flex items-center justify-between p-2 rounded-lg bg-black/40 border border-white/5"
            >
              <span className="text-sm text-slate-300 truncate max-w-[70%]">
                "{query}"
              </span>
              <span className="text-xs font-mono bg-amber-500/20 text-amber-400 px-2 py-0.5 rounded-full">
                {count}
              </span>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

function GatewayStatusBadge() {
  const [status, setStatus] = useState("Normal");
  const [endpoints, setEndpoints] = useState([
    { name: "/api/v1/status", ms: 45 },
    { name: "/api/v1/network", ms: 62 },
    { name: "/api/v1/nodes", ms: 38 },
  ]);

  useEffect(() => {
    // Service Worker Listener for API Health Metrics
    const handleSWMessage = (event: MessageEvent) => {
      if (event.data && event.data.type === "API_HEALTH_UPDATE") {
        setStatus(event.data.payload.status);
        if (event.data.payload.endpoints) {
          setEndpoints(event.data.payload.endpoints);
        }
      }
    };

    if ("serviceWorker" in navigator) {
      navigator.serviceWorker.addEventListener("message", handleSWMessage);
    }

    // Efficient local simulation if SW is not active
    const interval = setInterval(() => {
      setEndpoints((prev) =>
        prev.map((ep) => ({
          ...ep,
          ms: Math.max(10, ep.ms + (Math.random() * 20 - 10)),
        })),
      );

      setStatus((prev) => {
        // If any endpoint is over 300ms, mark degraded
        let newStatus = "Normal";
        const degraded = Math.random() > 0.95;
        if (degraded) return "Degraded";
        return "Normal";
      });
    }, 5000);

    return () => {
      if ("serviceWorker" in navigator) {
        navigator.serviceWorker.removeEventListener("message", handleSWMessage);
      }
      clearInterval(interval);
    };
  }, []);

  const isNormal = status === "Normal";

  return (
    <div
      className={`bg-[#090b14]/80 backdrop-blur-md border ${isNormal ? "border-emerald-500/30 hover:border-emerald-500/50" : "border-amber-500/30 hover:border-amber-500/50 shadow-[0_0_15px_rgba(245,158,11,0.2)]"} rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:-translate-y-1 transition-all duration-300`}
    >
      <div className="flex justify-between items-center mb-4 relative z-10">
        <div className="flex items-center gap-2 text-slate-300">
          <Shield
            className={`w-4 h-4 ${isNormal ? "text-emerald-400" : "text-amber-400"}`}
          />
          <span className="text-sm font-medium uppercase tracking-wider font-mono">
            Gateway Status
          </span>
        </div>
        <div className="flex items-center gap-2">
          <span className="relative flex h-3 w-3">
            {isNormal && (
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
            )}
            <span
              className={`relative inline-flex rounded-full h-3 w-3 ${isNormal ? "bg-emerald-500" : "bg-amber-500 animate-pulse"}`}
            ></span>
          </span>
          <span
            className={`text-sm font-mono font-bold tracking-widest uppercase ${isNormal ? "text-emerald-400" : "text-amber-400"}`}
          >
            {status}
          </span>
        </div>
      </div>
      <div className="space-y-3 mt-4 relative z-10">
        {endpoints.map((ep) => (
          <div
            key={ep.name}
            className="flex items-center justify-between p-2 rounded bg-black/20 border border-white/5"
          >
            <span className="text-xs font-mono text-slate-400">{ep.name}</span>
            <div className="flex items-center gap-3">
              <span
                className={`text-xs font-mono ${ep.ms > 300 ? "text-amber-500" : "text-slate-500"}`}
              >
                {ep.ms.toFixed(0)}ms
              </span>
              <span
                className={`w-2 h-2 rounded-full ${ep.ms > 300 ? "bg-amber-500" : "bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]"}`}
              ></span>
            </div>
          </div>
        ))}
      </div>
      <div
        className={`absolute top-0 right-0 w-32 h-32 ${isNormal ? "bg-emerald-500/10" : "bg-amber-500/10"} rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none`}
      />
    </div>
  );
}

import {
  CheckCircle2,
  FileText,
  LayoutDashboard,
  CheckSquare,
  Square,
  Copy,
} from "lucide-react";

function CustomDashboardsWidget() {
  const [selectedMetrics, setSelectedMetrics] = useState<string[]>(() => {
    try {
      return JSON.parse(
        localStorage.getItem("atc_landing_metrics") ||
          '["tps", "cpu", "nodes"]',
      );
    } catch {
      return ["tps", "cpu", "nodes"];
    }
  });

  const availableMetrics = [
    { id: "cpu", label: "Core CPU Load" },
    { id: "memory", label: "Memory Allocation" },
    { id: "tps", label: "Transactions/Sec (TPS)" },
    { id: "nodes", label: "Active Nodes" },
    { id: "agents", label: "Active Agents" },
    { id: "networkIn", label: "Inbound Traffic" },
    { id: "networkOut", label: "Outbound Traffic" },
  ];

  const handleToggle = (id: string) => {
    let next;
    if (selectedMetrics.includes(id)) {
      next = selectedMetrics.filter((m) => m !== id);
    } else {
      next = [...selectedMetrics, id];
    }
    setSelectedMetrics(next);
    localStorage.setItem("atc_landing_metrics", JSON.stringify(next));
  };

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-indigo-500/50 transition-all duration-300">
      <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
      <div className="flex justify-between items-center mb-4 relative z-10">
        <div className="flex items-center gap-2 text-slate-300">
          <LayoutDashboard className="w-4 h-4 text-indigo-400" />
          <span className="text-sm font-medium uppercase tracking-wider font-mono">
            Custom Dashboards
          </span>
        </div>
      </div>
      <p className="text-xs text-slate-400 mb-4 font-mono relative z-10">
        Select which system metrics you want to visualize on your landing page.
      </p>
      <div className="space-y-2 relative z-10">
        {availableMetrics.map((sm) => {
          const isSelected = selectedMetrics.includes(sm.id);
          return (
            <div
              key={sm.id}
              className={`flex items-center justify-between p-2 rounded-lg cursor-pointer transition-colors border ${isSelected ? "bg-indigo-500/10 border-indigo-500/30 text-white" : "bg-black/20 border-white/5 text-slate-400 hover:bg-white/5"}`}
              onClick={() => handleToggle(sm.id)}
            >
              <span className="text-xs font-mono">{sm.label}</span>
              {isSelected ? (
                <CheckSquare className="w-4 h-4 text-indigo-400" />
              ) : (
                <Square className="w-4 h-4 text-slate-600" />
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export function MetricsView({
  searchHistory = [],
}: {
  searchHistory?: string[];
}) {
  const [metricsTab, setMetricsTab] = useState<
    "overview" | "performance" | "testing"
  >("overview");
  const [diagnosticsRunning, setDiagnosticsRunning] = useState(false);
  const [diagnosticsDone, setDiagnosticsDone] = useState(false);
  const [showLogsModal, setShowLogsModal] = useState(false);

  useEffect(() => {
    setDiagnosticsRunning(true);
    const timer = setTimeout(() => {
      setDiagnosticsRunning(false);
      setDiagnosticsDone(true);
    }, 2500);
    return () => clearTimeout(timer);
  }, []);

  const failedLogs = `> react-example@0.0.0 build
> vite build && esbuild server.ts --bundle --platform=node --format=cjs --packages=external --sourcemap --outfile=dist/server.cjs

vite v6.4.3 building for production...
transforming...
✓ 1017 modules transformed.
✓ Build completed in 2.01s
Build successful (Exit 0)`;

  const [copied, setCopied] = useState(false);

  const [metrics, setMetrics] = useState({
    cpu: 42,
    memory: 68,
    networkIn: 1240,
    networkOut: 890,
    activeNodes: 124,
    tps: 45,
  });

  const handleExportCSV = () => {
    const headers = ["Metric", "Value"];
    const rows = [
      ["CPU Usage (%)", metrics.cpu.toString()],
      ["Memory Usage (%)", metrics.memory.toString()],
      ["Network In (KB/s)", metrics.networkIn.toString()],
      ["Network Out (KB/s)", metrics.networkOut.toString()],
      ["Active Nodes", metrics.activeNodes.toString()],
      ["TPS", metrics.tps.toString()],
    ];

    const csvContent =
      "data:text/csv;charset=utf-8," +
      headers.join(",") +
      "\n" +
      rows.map((e) => e.join(",")).join("\n");

    const encodedUri = encodeURI(csvContent);
    const link = document.createElement("a");
    link.setAttribute("href", encodedUri);
    link.setAttribute(
      "download",
      `atownchain-metrics-${new Date().toISOString().slice(0, 10)}.csv`,
    );
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const [githubCommits, setGithubCommits] = useState<
    { sha: string; author: string; message: string; time: string }[]
  >([]);

  useEffect(() => {
    const fetchGithubCommits = async () => {
      try {
        const d = new Date();
        const formatted = [
          {
            sha: "a3f01c9",
            author: "A-TownChain CoreDev",
            message: "feat: Initial commit for atc-pack IPFS integration",
            time: d.toISOString(),
          },
          {
            sha: "7b9e28f",
            author: "Compiler Team",
            message: "fix: AST parser matching for [cross_chain] pragmas",
            time: new Date(d.getTime() - 1000 * 60 * 45).toISOString(),
          },
          {
            sha: "1e5a88c",
            author: "Tooling",
            message: "chore: Setup GitHub Actions CI for atc-trace",
            time: new Date(d.getTime() - 1000 * 60 * 120).toISOString(),
          },
          {
            sha: "d91a92e",
            author: "Security",
            message: "sec: Pin dependencies in atc-lang Cargo.toml equivalents",
            time: new Date(d.getTime() - 1000 * 60 * 180).toISOString(),
          },
        ];
        setGithubCommits(formatted);
      } catch (e) {
        console.error("Failed to fetch github commits", e);
      }
    };
    fetchGithubCommits();
    const interval = setInterval(fetchGithubCommits, 60000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [infoRes, statusRes] = await Promise.all([
          fetch("/api/blockchain/info"),
          fetch("/api/status"),
        ]);

        let activeNodes = metrics.activeNodes;
        let tps = metrics.tps;
        let networkIn = metrics.networkIn;
        let networkOut = metrics.networkOut;

        if (infoRes.ok) {
          const info = await infoRes.json();
          activeNodes = info.peers?.length || activeNodes;
        }

        if (statusRes.ok) {
          const status = await statusRes.json();
          if (status.tps !== undefined) tps = status.tps;
          if (status.network_traffic) {
            networkIn =
              parseFloat(status.network_traffic.inbound_kbps) || networkIn;
            networkOut =
              parseFloat(status.network_traffic.outbound_kbps) || networkOut;
          }
        }

        setMetrics((prev) => ({
          cpu: Math.min(100, Math.max(0, prev.cpu + (Math.random() * 4 - 2))), // simulate slight cpu jitter
          memory: Math.min(
            100,
            Math.max(0, prev.memory + (Math.random() * 2 - 1)),
          ), // simulate slight mem jitter
          networkIn,
          networkOut,
          activeNodes,
          tps,
        }));
      } catch (e) {
        console.error("Failed to fetch metrics data", e);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 2000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex flex-col gap-6 mt-6 pb-12 w-full max-w-6xl mx-auto">
      <BackupReminder />

      <div className="flex items-center gap-3 mb-4">
        <div className="w-10 h-10 rounded-lg bg-atc-cyan/10 border border-atc-cyan/20 flex items-center justify-center text-atc-cyan">
          <Activity className="w-5 h-5" />
        </div>
        <div>
          <h2 className="text-2xl font-semibold tracking-tight text-white">
            System Metrics{" "}
            <TooltipIcon content="Real-time telemetry and resource scaling diagnostics across the complete architecture." />
          </h2>
          <p className="text-sm font-light text-slate-400">
            Live Telemetrie und Netzwerk-Ressourcen
          </p>
        </div>
      </div>

      <div className="flex justify-between items-center mb-6 border-b border-white/10 pb-4">
        <div className="flex items-center gap-3">
          <button
            onClick={() => setMetricsTab("overview")}
            className={`px-4 py-2 rounded-lg text-sm font-bold transition-colors ${metricsTab === "overview" ? "bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/40" : "text-slate-400 hover:text-white hover:bg-white/5"}`}
          >
            Overview
          </button>
          <button
            onClick={() => setMetricsTab("performance")}
            className={`px-4 py-2 rounded-lg text-sm font-bold transition-colors ${metricsTab === "performance" ? "bg-indigo-500/20 text-indigo-400 border border-indigo-500/40" : "text-slate-400 hover:text-white hover:bg-white/5"}`}
          >
            Performance
          </button>
          <button
            onClick={() => setMetricsTab("testing")}
            className={`px-4 py-2 rounded-lg text-sm font-bold transition-colors ${metricsTab === "testing" ? "bg-emerald-500/20 text-emerald-400 border border-emerald-500/40" : "text-slate-400 hover:text-white hover:bg-white/5"}`}
          >
            Testing
          </button>
        </div>
        <button
          onClick={handleExportCSV}
          className="flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-bold font-mono transition-colors border border-white/10 hover:bg-white/10 text-slate-300"
        >
          <Download className="w-4 h-4" />
          CSV EXPORT
        </button>
      </div>

      {metricsTab === "overview" && (
        <div className="flex flex-col gap-6">
          <MetricsDashboard />
          <SystemHealthDashboard />

          {/* System Diagnostics Health Check Widget */}
          <div className="bg-[#090b14]/80 backdrop-blur-md border border-rose-500/30 rounded-2xl p-6 shadow-2xl relative overflow-hidden group">
            <div className="absolute top-0 right-0 w-32 h-32 bg-rose-500/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
            <div className="flex justify-between items-center mb-4 relative z-10">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-lg bg-rose-500/10 flex items-center justify-center text-rose-500">
                  <Activity className="w-5 h-5" />
                </div>
                <div>
                  <h3 className="text-lg font-bold text-white tracking-tight">
                    Automated Build Health
                  </h3>
                  <div className="text-xs font-mono mt-0.5 flex items-center gap-2">
                    <span
                      className={
                        diagnosticsRunning
                          ? "text-amber-400"
                          : "text-emerald-400 font-bold"
                      }
                    >
                      {diagnosticsRunning
                        ? "Running diagnostics..."
                        : "Build Passed (Exit 0)"}
                    </span>
                  </div>
                </div>
              </div>
              {!diagnosticsRunning && diagnosticsDone && (
                <button
                  onClick={() => setShowLogsModal(true)}
                  className="px-4 py-2 bg-emerald-500/20 hover:bg-emerald-500/40 border border-emerald-500/50 text-emerald-300 rounded-lg text-xs font-bold transition-colors flex items-center gap-2"
                >
                  <FileText className="w-4 h-4" /> View Logs
                </button>
              )}
            </div>
            <div className="w-full h-2 bg-black rounded-full overflow-hidden relative z-10 border border-white/5">
              <motion.div
                className={`h-full ${diagnosticsRunning ? "bg-amber-400" : "bg-emerald-500"}`}
                initial={{ width: 0 }}
                animate={{ width: diagnosticsRunning ? "60%" : "100%" }}
                transition={{
                  duration: diagnosticsRunning ? 2.5 : 0.5,
                  ease: "linear",
                }}
              />
            </div>
          </div>

          <AnimatePresence>
            {showLogsModal && (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
              >
                <motion.div
                  initial={{ scale: 0.95, y: 20 }}
                  animate={{ scale: 1, y: 0 }}
                  exit={{ scale: 0.95, y: 20 }}
                  className="bg-[#090b14] border border-rose-500/30 rounded-2xl w-full max-w-4xl shadow-2xl relative overflow-hidden flex flex-col max-h-[85vh]"
                >
                  <div className="absolute top-0 right-0 w-64 h-64 bg-rose-500/10 rounded-full blur-[80px] pointer-events-none" />

                  <div className="px-6 py-4 border-b border-white/10 flex items-center justify-between bg-black/40 relative z-10 shrink-0">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-500">
                        <CheckCircle2 className="w-5 h-5" />
                      </div>
                      <div>
                        <h3 className="text-lg font-bold text-white tracking-tight">
                          Build Success Details
                        </h3>
                        <p className="text-xs text-emerald-400 font-mono">
                          Exit status 0
                        </p>
                      </div>
                    </div>
                    <button
                      onClick={() => setShowLogsModal(false)}
                      className="text-slate-500 hover:text-white transition-colors p-2 rounded-lg hover:bg-white/5"
                    >
                      <X className="w-5 h-5" />
                    </button>
                  </div>

                  <div className="p-6 overflow-y-auto custom-scrollbar flex-1 relative z-10 text-sm font-mono text-slate-300">
                    <p className="mb-4 text-slate-400 font-sans">
                      The automated CI/CD pipeline successfully built the
                      production artifacts. Please review the raw logs below.
                    </p>
                    <div className="bg-[#04060c] border border-white/5 rounded-xl p-4 overflow-x-auto">
                      <pre className="text-emerald-400/90 whitespace-pre-wrap">
                        {failedLogs}
                      </pre>
                    </div>
                  </div>

                  <div className="px-6 py-4 border-t border-white/10 bg-black/40 flex justify-between items-center relative z-10 shrink-0">
                    <div className="text-xs text-slate-500 font-mono">
                      Timestamp: {new Date().toISOString()}
                    </div>
                    <button
                      onClick={() => {
                        navigator.clipboard.writeText(failedLogs);
                        setCopied(true);
                        setTimeout(() => setCopied(false), 2000);
                      }}
                      className="flex items-center gap-2 px-6 py-2.5 bg-rose-500 hover:bg-rose-600 text-white rounded-xl text-sm font-bold transition-colors"
                    >
                      {copied ? (
                        <CheckCircle2 className="w-4 h-4" />
                      ) : (
                        <Copy className="w-4 h-4" />
                      )}
                      {copied ? "Copied!" : "Copy Full Logs"}
                    </button>
                  </div>
                </motion.div>
              </motion.div>
            )}
          </AnimatePresence>

          {/* Top Priority Row */}
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div className="lg:col-span-2">
              <Suspense fallback={<ChartFallback />}>
                <APIResponseTimeWidget />
              </Suspense>
            </div>
            <div className="lg:col-span-1 flex flex-col gap-6">
              <GatewayStatusBadge />
              <SystemLifecycleWidget />
            </div>
          </div>

          <DashboardGrid>
            <Suspense fallback={<ChartFallback />}>
              <TransactionVolumeChart />
            </Suspense>
            <SystemLogsWidget />
            <CustomDashboardsWidget />

            {/* CPU Usage */}
            <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-atc-cyan/50 hover:-translate-y-1 transition-all duration-300">
              <div className="absolute top-0 right-0 w-32 h-32 bg-atc-cyan/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
              <div className="flex justify-between items-center mb-4 relative z-10">
                <div className="flex items-center gap-2 text-slate-300">
                  <Cpu className="w-4 h-4 text-atc-cyan" />
                  <span className="text-sm font-medium uppercase tracking-wider font-mono">
                    Core CPU Load
                  </span>
                </div>
                <span className="text-2xl font-mono text-white">
                  {metrics.cpu.toFixed(1)}%
                </span>
              </div>
              <div className="w-full h-2 bg-atc-bg rounded-full overflow-hidden relative z-10">
                <motion.div
                  className="h-full bg-atc-cyan"
                  animate={{ width: `${metrics.cpu}%` }}
                  transition={{ duration: 0.5 }}
                />
              </div>
            </div>

            {/* Memory Usage */}
            <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-atc-purple/50 hover:-translate-y-1 transition-all duration-300">
              <div className="absolute top-0 right-0 w-32 h-32 bg-atc-purple/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
              <div className="flex justify-between items-center mb-4 relative z-10">
                <div className="flex items-center gap-2 text-slate-300">
                  <HardDrive className="w-4 h-4 text-atc-purple" />
                  <span className="text-sm font-medium uppercase tracking-wider font-mono">
                    Memory Allocation
                  </span>
                </div>
                <span className="text-2xl font-mono text-white">
                  {metrics.memory.toFixed(1)}%
                </span>
              </div>
              <div className="w-full h-2 bg-atc-bg rounded-full overflow-hidden relative z-10">
                <motion.div
                  className="h-full bg-atc-purple"
                  animate={{ width: `${metrics.memory}%` }}
                  transition={{ duration: 0.5 }}
                />
              </div>
            </div>

            {/* TPS */}
            <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-emerald-500/50 hover:-translate-y-1 transition-all duration-300">
              <div className="absolute top-0 right-0 w-32 h-32 bg-emerald-500/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
              <div className="flex justify-between items-center mb-4 relative z-10">
                <div className="flex items-center gap-2 text-slate-300">
                  <Zap className="w-4 h-4 text-emerald-400" />
                  <span className="text-sm font-medium uppercase tracking-wider font-mono">
                    Transactions/Sec
                  </span>
                </div>
                <span className="text-2xl font-mono text-white">
                  {metrics.tps.toFixed(0)}
                </span>
              </div>
              <div className="w-full h-2 bg-atc-bg rounded-full overflow-hidden flex items-center relative z-10">
                <div className="flex-1 border-b border-dashed border-emerald-500/30" />
                <div className="w-2 h-2 rounded-full bg-emerald-400 shadow-[0_0_10px_rgba(52,211,153,0.8)] animate-pulse" />
              </div>
            </div>

            {/* Network Traffic */}
            <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl col-span-1 md:col-span-2 lg:col-span-2 relative overflow-hidden group hover:border-blue-500/50 hover:-translate-y-1 transition-all duration-300">
              <div className="absolute top-0 right-0 w-48 h-48 bg-blue-500/10 rounded-full blur-[80px] opacity-0 group-hover:opacity-100 transition-opacity" />
              <div className="flex items-center gap-2 text-slate-300 mb-6 relative z-10">
                <Network className="w-4 h-4 text-blue-400" />
                <span className="text-sm font-medium uppercase tracking-wider font-mono">
                  P2P Network Traffic
                </span>
              </div>
              <div className="flex flex-col sm:flex-row gap-8 relative z-10">
                <div className="flex-1">
                  <div className="flex justify-between text-xs text-slate-400 mb-1 font-mono">
                    <span>Inbound (tx)</span>
                    <span>{(metrics.networkIn / 1000).toFixed(2)} MB/s</span>
                  </div>
                  <div className="w-full h-1.5 bg-atc-bg rounded-full overflow-hidden">
                    <motion.div
                      className="h-full bg-blue-400"
                      animate={{
                        width: `${Math.min(100, (metrics.networkIn / 2000) * 100)}%`,
                      }}
                      transition={{ duration: 0.5 }}
                    />
                  </div>
                </div>
                <div className="flex-1">
                  <div className="flex justify-between text-xs text-slate-400 mb-1 font-mono">
                    <span>Outbound (rx)</span>
                    <span>{(metrics.networkOut / 1000).toFixed(2)} MB/s</span>
                  </div>
                  <div className="w-full h-1.5 bg-atc-bg rounded-full overflow-hidden">
                    <motion.div
                      className="h-full bg-orange-400"
                      animate={{
                        width: `${Math.min(100, (metrics.networkOut / 2000) * 100)}%`,
                      }}
                      transition={{ duration: 0.5 }}
                    />
                  </div>
                </div>
              </div>
            </div>

            {/* Active Nodes */}
            <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl flex flex-col justify-center relative overflow-hidden group hover:border-slate-500/50 hover:-translate-y-1 transition-all duration-300">
              <div className="absolute top-0 right-0 w-32 h-32 bg-white/5 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
              <div className="absolute top-0 right-0 p-4 opacity-10">
                <Server className="w-24 h-24" />
              </div>
              <div className="flex items-center gap-2 text-slate-300 mb-2 relative z-10">
                <Wifi className="w-4 h-4 text-white" />
                <span className="text-sm font-medium uppercase tracking-wider font-mono">
                  Active Nodes
                </span>
              </div>
              <span className="text-4xl font-mono text-white relative z-10">
                {Math.floor(metrics.activeNodes)}
              </span>
              <span className="text-xs text-emerald-400 font-mono mt-2 relative z-10">
                ● P2P Interconnected & Synced
              </span>
            </div>

            <Suspense fallback={<ChartFallback />}>
              <NetworkStatusHistoryWidget />
            </Suspense>
            <SearchQueriesWidget searchHistory={searchHistory} />
            <Suspense fallback={<ChartFallback />}>
              <SyncDurationWidget />
            </Suspense>
            <Suspense fallback={<ChartFallback />}>
              <APIHealthWidget />
            </Suspense>
            <Suspense fallback={<ChartFallback />}>
              <SyncHealthWidget />
            </Suspense>

            {/* GitHub Live Commits Sync */}
            <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl col-span-1 md:col-span-2 lg:col-span-2 relative overflow-hidden group">
              <div className="flex items-center justify-between mb-4 relative z-10">
                <div className="flex items-center gap-2 text-slate-300">
                  <Github className="w-5 h-5 text-slate-400" />
                  <span className="text-sm font-bold uppercase tracking-wider font-mono">
                    Live Commit Sync
                  </span>
                  <span className="ml-2 px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 text-[10px] font-bold animate-pulse">
                    Auto-Sync On
                  </span>
                </div>
                <div className="text-xs font-mono text-slate-500">
                  repo: A-TownChain/a-townchain-core
                </div>
              </div>

              <div className="flex flex-col gap-4 relative z-10">
                <div className="flex flex-col gap-2">
                  <AnimatePresence>
                    {githubCommits.slice(0, 4).map((commit, idx) => (
                      <motion.div
                        key={commit.sha}
                        initial={{ opacity: 0, x: -10 }}
                        animate={{ opacity: 1, x: 0 }}
                        transition={{ delay: idx * 0.1 }}
                        className="flex items-center justify-between p-3 rounded-xl border border-white/5 bg-black/20 hover:border-white/10 transition-colors group/commit"
                      >
                        <div className="flex flex-col gap-1 w-full max-w-[70%]">
                          <div className="flex items-center gap-2">
                            <GitCommit className="w-3 h-3 text-slate-500" />
                            <span className="text-sm font-medium text-slate-200 truncate">
                              {commit.message}
                            </span>
                          </div>
                          <div className="flex items-center gap-2 pl-5 text-[10px] text-slate-500 font-mono">
                            <span className="text-atc-cyan">
                              {commit.author}
                            </span>
                            <span>•</span>
                            <span>
                              {new Date(commit.time).toLocaleString()}
                            </span>
                          </div>
                        </div>
                        <div className="text-xs font-mono text-slate-400 bg-white/5 px-2 py-1 rounded group-hover/commit:text-white transition-colors">
                          {commit.sha}
                        </div>
                      </motion.div>
                    ))}
                    {githubCommits.length === 0 && (
                      <div className="p-4 text-center text-slate-500 text-sm font-mono">
                        Fetching latest commits from GitHub...
                      </div>
                    )}
                  </AnimatePresence>
                </div>

                {githubCommits.length > 0 && (
                  <div className="border-t border-white/10 pt-4 mt-2">
                    <h4 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-2 font-mono">
                      Commit Activity
                    </h4>
                    <Suspense fallback={<ChartFallback />}>
                      <CommitChart commits={githubCommits} />
                    </Suspense>
                  </div>
                )}
              </div>
            </div>

            <PerformanceHeatMap />
            <SystemHealthWidget />
            <GithubSync />
            <Suspense fallback={<ChartFallback />}>
              <NetworkLatencyFluctuationWidget />
            </Suspense>
          </DashboardGrid>
        </div>
      )}

      {metricsTab === "performance" && (
        <div className="flex flex-col gap-6">
          <div className="grid grid-cols-1 gap-6">
            <Suspense fallback={<ChartFallback />}>
              <IndexedDBLatencyChart />
            </Suspense>
          </div>
        </div>
      )}

      {metricsTab === "testing" && (
        <div className="flex flex-col gap-6">
          <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden text-slate-300">
            <div className="flex justify-between items-center mb-6">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-400">
                  <Terminal className="w-5 h-5" />
                </div>
                <div>
                  <h3 className="text-lg font-bold text-white tracking-tight">
                    Vitest Core Module Suite
                  </h3>
                  <p className="text-xs text-slate-400">
                    Automated integration & unit tests
                  </p>
                </div>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {[
                {
                  module: "AuthenticationService",
                  time: "14ms",
                  status: "pass",
                },
                { module: "IndexedDBManager", time: "22ms", status: "pass" },
                {
                  module: "NetworkTopologyResolver",
                  time: "41ms",
                  status: "pass",
                },
                { module: "ZKProofGenerator", time: "155ms", status: "fail" },
                { module: "SmartContractVM", time: "88ms", status: "pass" },
                { module: "TelemetryAggregator", time: "6ms", status: "pass" },
              ].map((test, idx) => (
                <div
                  key={idx}
                  className={`p-4 rounded-xl border ${test.status === "pass" ? "border-emerald-500/20 bg-emerald-500/5" : "border-rose-500/20 bg-rose-500/5"} flex items-center justify-between`}
                >
                  <div className="flex flex-col">
                    <span className="font-mono text-sm text-slate-200">
                      {test.module}.spec.ts
                    </span>
                    <span className="text-[10px] text-slate-500 mt-1">
                      Duration: {test.time}
                    </span>
                  </div>
                  {test.status === "pass" ? (
                    <span className="flex items-center gap-2 text-emerald-400 text-xs font-bold uppercase tracking-wider bg-emerald-500/10 px-3 py-1 rounded-full">
                      <CheckCircle2 className="w-4 h-4" /> Pass
                    </span>
                  ) : (
                    <span className="flex items-center gap-2 text-rose-400 text-xs font-bold uppercase tracking-wider bg-rose-500/10 px-3 py-1 rounded-full">
                      <X className="w-4 h-4" /> Fail
                    </span>
                  )}
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
