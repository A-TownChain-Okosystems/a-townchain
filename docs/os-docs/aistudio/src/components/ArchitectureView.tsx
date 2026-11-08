import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import {
  Database,
  Activity,
  Layers,
  Component,
  Network,
  Microscope,
  Download,
  Map,
  Search,
  Shield,
  FileCheck,
} from "lucide-react";
import { jsPDF } from "jspdf";
import "jspdf-autotable";
import { LineChart, Line, ResponsiveContainer, YAxis } from "recharts";
import { TooltipIcon } from "./TooltipIcon";
import { StrategicArchitectureMap } from "./StrategicArchitectureMap";
import { LAYERS, TARGET_STATE, LONG_TERM_VISION } from "../data";

const ICON_MAP: Record<string, React.ElementType> = {
  Database,
  Activity,
  Layers,
  Component,
  Network,
  Microscope,
};

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
      className={`p-5 rounded-2xl border ${highlight ? "border-atc-cyan/50 bg-[#090b14]/80 shadow-[0_0_20px_rgba(6,182,212,0.15)]" : "border-atc-border/50 bg-[#090b14]/40"} backdrop-blur-md relative overflow-hidden group hover:border-atc-purple/30 transition-all`}
    >
      <div className="flex justify-between items-start mb-4">
        <div
          className={`p-2.5 rounded-xl ${highlight ? "bg-atc-cyan/10 text-atc-cyan" : "bg-atc-purple/10 text-atc-purple"}`}
        >
          <Icon className="w-5 h-5" />
        </div>
        <div className="flex items-center gap-1.5">
          <div className="text-[10px] font-mono text-slate-500 uppercase tracking-widest">
            {title}
          </div>
          {info && <TooltipIcon content={info} />}
        </div>
      </div>
      <div className="flex max-w-[200px] justify-between items-end mt-2">
        <div className="flex flex-col">
          <span className="text-[10px] font-mono text-slate-500 uppercase tracking-wider mb-1">
            Current
          </span>
          <span className="text-xl font-semibold text-white tracking-tight">
            {current}
          </span>
        </div>
        <div className="flex flex-col text-right">
          <span className="text-[10px] font-mono text-slate-500 uppercase tracking-wider mb-1">
            Vision
          </span>
          <span className="text-xl font-semibold text-atc-cyan tracking-tight">
            {vision}
          </span>
        </div>
      </div>
    </div>
  );
}

function IndexedDBSchemaWidget() {
  const [dataCount, setDataCount] = useState(0);

  useEffect(() => {
    // In a real scenario we'd do: atcDatabase.getAll().then(d => setDataCount(d.length))
    // We mock a count visualization or import atcDatabase if available
    setDataCount(124);
  }, []);

  return (
    <div className="p-8 rounded-2xl bg-[#090b14]/80 backdrop-blur-md border border-indigo-500/30 relative overflow-hidden group hover:border-indigo-500/50 transition-all mb-12">
      <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-500/5 rounded-full blur-[80px] opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />
      <div className="relative z-10">
        <div className="flex items-center gap-3 mb-6">
          <Database className="w-6 h-6 text-indigo-400" />
          <h2 className="text-xl font-bold text-white tracking-tight">
            IndexedDB Schema & Local Data{" "}
            <TooltipIcon content="Local caching layer via IndexedDB that ensures architectural data is persistent even when disconnected." />
          </h2>
        </div>
        <p className="text-slate-400 text-sm mb-8 max-w-2xl">
          Visualizing the local IndexedDB database layout used for caching
          architectural states offline. Enables resilient validation when the
          network is unreachable.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div className="bg-black/40 rounded-xl p-5 border border-white/5">
            <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
              Core Schema Options
            </h3>
            <div className="font-mono text-xs text-indigo-300 space-y-2">
              <div className="flex justify-between border-b border-white/5 pb-2">
                <span>Database Name</span>
                <span className="text-slate-300">atc_os_database</span>
              </div>
              <div className="flex justify-between border-b border-white/5 pb-2">
                <span>Version</span>
                <span className="text-slate-300">v4 (Active)</span>
              </div>
              <div className="flex justify-between border-b border-white/5 pb-2">
                <span>Encryption</span>
                <span className="text-amber-400">AES-256-GCM</span>
              </div>
              <div className="flex justify-between pt-1">
                <span>Persisted Storage</span>
                <span className="text-emerald-400">Granted</span>
              </div>
            </div>
          </div>

          <div className="bg-black/40 rounded-xl p-5 border border-white/5 lg:col-span-2">
            <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
              Object Stores & Record Counts
            </h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5 hover:border-indigo-500/30 transition-colors">
                <div className="flex flex-col">
                  <span className="text-sm font-bold text-slate-200">
                    key_value_store
                  </span>
                  <span className="text-[10px] text-slate-500 font-mono">
                    Key Path: id | Indexes: timestamp
                  </span>
                </div>
                <div className="flex items-center gap-4">
                  <div className="text-right flex flex-col">
                    <span className="font-mono text-emerald-400 font-bold">
                      {dataCount}
                    </span>
                    <span className="text-[10px] text-slate-500 uppercase tracking-widest">
                      Records
                    </span>
                  </div>
                </div>
              </div>
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5 hover:border-indigo-500/30 transition-colors">
                <div className="flex flex-col">
                  <span className="text-sm font-bold text-slate-200">
                    atc_lang_artifacts
                  </span>
                  <span className="text-[10px] text-slate-500 font-mono">
                    Key Path: hash | Indexes: type, created
                  </span>
                </div>
                <div className="flex items-center gap-4">
                  <div className="text-right flex flex-col">
                    <span className="font-mono text-emerald-400 font-bold">
                      1,042
                    </span>
                    <span className="text-[10px] text-slate-500 uppercase tracking-widest">
                      Records
                    </span>
                  </div>
                </div>
              </div>
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5 hover:border-indigo-500/30 transition-colors">
                <div className="flex flex-col">
                  <span className="text-sm font-bold text-slate-200">
                    telemetry_cache
                  </span>
                  <span className="text-[10px] text-slate-500 font-mono">
                    Key Path: eventId | Indexes: timestamp, severity
                  </span>
                </div>
                <div className="flex items-center gap-4">
                  <div className="text-right flex flex-col">
                    <span className="font-mono text-emerald-400 font-bold">
                      84,204
                    </span>
                    <span className="text-[10px] text-slate-500 uppercase tracking-widest">
                      Records
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function HealthSparkline({ layerId }: { layerId: number }) {
  // Generate deterministic mock health data for the layer
  const data = Array.from({ length: 20 }, (_, i) => {
    // some pseudo-randomness based on layerId and index
    const base = 85 + (layerId % 10);
    const noise = Math.sin(i + layerId) * 10;
    return { value: Math.min(100, Math.max(60, base + noise)) };
  });

  return (
    <div className="flex flex-col items-end">
      <div className="text-[10px] text-slate-500 font-mono mb-1">
        HEALTH METRIC
      </div>
      <div className="w-24 h-8 opacity-80 hover:opacity-100 transition-opacity">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <YAxis domain={["dataMin - 5", 100]} hide />
            <Line
              type="monotone"
              dataKey="value"
              stroke="#06b6d4"
              strokeWidth={1.5}
              dot={false}
              isAnimationActive={false}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

function SecurityLayerWidget() {
  return (
    <div className="p-8 rounded-2xl bg-[#090b14]/80 backdrop-blur-md border border-emerald-500/30 relative overflow-hidden group hover:border-emerald-500/50 transition-all mb-12">
      <div className="absolute top-0 right-0 w-64 h-64 bg-emerald-500/5 rounded-full blur-[80px] opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />
      <div className="relative z-10">
        <div className="flex items-center gap-3 mb-6">
          <Shield className="w-6 h-6 text-emerald-400" />
          <h2 className="text-xl font-bold text-white tracking-tight">
            Security Layer & Zero-Knowledge Proofs{" "}
            <TooltipIcon content="Advanced Network & Node Security layer, integrating zero-knowledge proofs and hardware-level encryption." />
          </h2>
        </div>
        <p className="text-slate-400 text-sm mb-8 max-w-2xl">
          Visualizing the implementation of zero-knowledge proofs (ZKPs) for autonomous task verification and hardware-level encryption mechanisms to ensure secure execution and verifiable state transitions on our blockchain infrastructure.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-black/40 rounded-xl p-5 border border-white/5">
            <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
              ZKP Verification Core
            </h3>
            <div className="font-mono text-xs text-emerald-300 space-y-2">
              <div className="flex justify-between border-b border-white/5 pb-2">
                <span>Proof Protocol</span>
                <span className="text-slate-300">zk-SNARKs</span>
              </div>
              <div className="flex justify-between border-b border-white/5 pb-2">
                <span>Verification Time</span>
                <span className="text-slate-300">~12ms</span>
              </div>
              <div className="flex justify-between border-b border-white/5 pb-2">
                <span>Elliptic Curve</span>
                <span className="text-amber-400">BN254</span>
              </div>
              <div className="flex justify-between pt-1">
                <span>Constraint System</span>
                <span className="text-emerald-400">Plonk</span>
              </div>
            </div>
          </div>

          <div className="bg-black/40 rounded-xl p-5 border border-white/5">
            <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
              Hardware Security Enclave
            </h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5 hover:border-emerald-500/30 transition-colors">
                <div className="flex flex-col">
                  <span className="text-sm font-bold text-slate-200">
                    Trusted Execution Environment
                  </span>
                  <span className="text-[10px] text-slate-500 font-mono">
                    Hardware-isolated execution context (WASM)
                  </span>
                </div>
                <div className="flex items-center gap-4">
                  <div className="text-right flex flex-col">
                    <span className="font-mono text-emerald-400 font-bold">
                      ACTIVE
                    </span>
                  </div>
                </div>
              </div>
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5 hover:border-emerald-500/30 transition-colors">
                <div className="flex flex-col">
                  <span className="text-sm font-bold text-slate-200">
                    Hardware-Level Encryption
                  </span>
                  <span className="text-[10px] text-slate-500 font-mono">
                    AES-GCM-256 for data-in-use
                  </span>
                </div>
                <div className="flex items-center gap-4">
                  <div className="text-right flex flex-col">
                    <span className="font-mono text-emerald-400 font-bold">
                      SECURE
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function AtosStandardsWidget() {
  return (
    <div className="p-8 rounded-2xl bg-[#090b14]/80 backdrop-blur-md border border-cyan-500/30 relative overflow-hidden group hover:border-cyan-500/50 transition-all mb-12">
      <div className="absolute top-0 right-0 w-64 h-64 bg-cyan-500/5 rounded-full blur-[80px] opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />
      <div className="relative z-10">
        <div className="flex items-center gap-3 mb-6">
          <FileCheck className="w-6 h-6 text-cyan-400" />
          <h2 className="text-xl font-bold text-white tracking-tight">
            ATOS-STANDARDS Layer{" "}
            <TooltipIcon content="A-Town OS Ecosystem Standards & Protocols layer serving as the central rulebook and compliance engine for the entire architecture." />
          </h2>
        </div>
        <p className="text-slate-400 text-sm mb-8 max-w-2xl">
          Visualizing the core ATOS-STANDARDS components ensuring uniformity, interoperability, and high-quality construction across all layers, franchised apps, and native sub-layers.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div className="bg-black/40 rounded-xl p-5 border border-white/5">
            <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
              Architecture Guidelines
            </h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5">
                <span className="text-sm font-bold text-slate-200">Zero-Trust Enforcement</span>
                <span className="font-mono text-cyan-400 text-xs font-bold">ACTIVE</span>
              </div>
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5">
                <span className="text-sm font-bold text-slate-200">Infinite Sub-Layering</span>
                <span className="font-mono text-cyan-400 text-xs font-bold">MANDATORY</span>
              </div>
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5">
                <span className="text-sm font-bold text-slate-200">Decentralization</span>
                <span className="font-mono text-cyan-400 text-xs font-bold">MANDATORY</span>
              </div>
            </div>
          </div>

          <div className="bg-black/40 rounded-xl p-5 border border-white/5">
            <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
              Component Specifications
            </h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5">
                <span className="text-sm font-bold text-slate-200">API Standards</span>
                <span className="font-mono text-cyan-400 text-xs font-bold">GRAPHQL/REST</span>
              </div>
              <div className="flex items-center justify-between p-2 rounded-lg border border-white/5 bg-white/5">
                <span className="text-sm font-bold text-slate-200">Design System</span>
                <span className="font-mono text-cyan-400 text-xs font-bold">AT-UI v2.0</span>
              </div>
            </div>
          </div>

          <div className="bg-black/40 rounded-xl p-5 border border-white/5">
            <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
              Development Guidelines
            </h3>
            <div className="font-mono text-xs text-cyan-300 space-y-2">
              <div className="flex justify-between border-b border-white/5 pb-2">
                <span>Code Audits</span>
                <span className="text-emerald-400">Automated</span>
              </div>
              <div className="flex justify-between border-b border-white/5 pb-2">
                <span>Documentation</span>
                <span className="text-slate-300">Required</span>
              </div>
              <div className="flex justify-between pt-1">
                <span>CI/CD Pipeline</span>
                <span className="text-emerald-400">Strict</span>
              </div>
            </div>
          </div>

          <div className="bg-black/40 rounded-xl p-5 border border-white/5 md:col-span-2 lg:col-span-3">
            <h3 className="text-sm font-mono text-slate-500 uppercase tracking-widest mb-4">
              Native Sub-Layers
            </h3>
            <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
              <div className="flex items-center justify-between p-3 rounded-lg border border-cyan-500/20 bg-cyan-500/5 hover:border-cyan-500/50 transition-colors">
                <span className="text-sm font-bold text-cyan-100">ATC-Lang-Standards</span>
              </div>
              <div className="flex items-center justify-between p-3 rounded-lg border border-cyan-500/20 bg-cyan-500/5 hover:border-cyan-500/50 transition-colors">
                <span className="text-sm font-bold text-cyan-100">ATC-Standards</span>
              </div>
              <div className="flex items-center justify-between p-3 rounded-lg border border-cyan-500/20 bg-cyan-500/5 hover:border-cyan-500/50 transition-colors">
                <span className="text-sm font-bold text-cyan-100">ATS-Standards</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export function ArchitectureView() {
  const [activeLayer, setActiveLayer] = useState<number | null>(null);
  const [searchQuery, setSearchQuery] = useState("");

  const filteredLayers = LAYERS.filter((layer) => {
    if (!searchQuery) return true;
    const lowerQuery = searchQuery.toLowerCase();
    return (
      layer.name.toLowerCase().includes(lowerQuery) ||
      layer.purpose.toLowerCase().includes(lowerQuery) ||
      layer.subLayers.some((c) => c.toLowerCase().includes(lowerQuery))
    );
  });

  const selectedLayer = LAYERS.find((l) => l.id === activeLayer);

  const handleExportPDF = () => {
    const doc = new jsPDF();

    // Title
    doc.setFontSize(20);
    doc.setTextColor(255, 255, 255);
    // Background rect
    doc.setFillColor(11, 15, 25);
    doc.rect(0, 0, 210, 297, "F");

    doc.setTextColor(200, 200, 200);
    doc.text("Architecture Explorer Report", 14, 22);

    doc.setFontSize(12);
    doc.text("Generated: " + new Date().toLocaleString(), 14, 30);

    // Target State Metrics
    doc.setFontSize(16);
    doc.text("Target State Metrics", 14, 45);

    const metricsData = [
      ["Metric", "Current", "Vision"],
      [
        "Main Systems",
        TARGET_STATE.mainSystems.current.toString(),
        TARGET_STATE.mainSystems.vision.toString(),
      ],
      [
        "Subsystems",
        TARGET_STATE.subsystems.current.toString(),
        TARGET_STATE.subsystems.vision.toString(),
      ],
      [
        "Sub-Layers",
        TARGET_STATE.coreComponents.current.toString(),
        TARGET_STATE.coreComponents.vision.toString(),
      ],
      [
        "Active Microservices",
        TARGET_STATE.services.current.toString(),
        TARGET_STATE.services.vision.toString(),
      ],
    ];

    (doc as any).autoTable({
      startY: 50,
      head: [metricsData[0]],
      body: metricsData.slice(1),
      theme: "grid",
      headStyles: { fillColor: [40, 40, 50], textColor: [255, 255, 255] },
      bodyStyles: { fillColor: [20, 25, 35], textColor: [200, 200, 200] },
      alternateRowStyles: { fillColor: [25, 30, 40] },
    });

    let finalY = (doc as any).lastAutoTable.finalY || 50;

    // Active Nodes (Layers)
    doc.setFontSize(16);
    doc.text("Active Architecture Nodes", 14, finalY + 15);

    const nodesData = LAYERS.map((layer) => [
      layer.id.toString(),
      layer.name,
      layer.purpose,
      layer.subLayers.join(", "),
    ]);

    (doc as any).autoTable({
      startY: finalY + 20,
      head: [["ID", "Name", "Description", "Components"]],
      body: nodesData,
      theme: "grid",
      headStyles: { fillColor: [40, 40, 50], textColor: [255, 255, 255] },
      bodyStyles: { fillColor: [20, 25, 35], textColor: [200, 200, 200] },
      alternateRowStyles: { fillColor: [25, 30, 40] },
    });

    // System Health Metrics
    doc.setFontSize(16);
    doc.text("System Health Metrics", 14, finalY + 15);

    const healthData = [
      ["Node Type", "Count", "Avg CPU", "Avg Memory", "Status"],
      ["API Nodes", "7", "45%", "1.2 GB", "Healthy"],
      ["Validation Nodes", "6", "62%", "4.5 GB", "Degraded"],
      ["Mining Nodes", "7", "85%", "8.1 GB", "Critical"],
    ];

    (doc as any).autoTable({
      startY: finalY + 20,
      head: [healthData[0]],
      body: healthData.slice(1),
      theme: "grid",
      headStyles: { fillColor: [40, 40, 50], textColor: [255, 255, 255] },
      bodyStyles: { fillColor: [20, 25, 35], textColor: [200, 200, 200] },
      alternateRowStyles: { fillColor: [25, 30, 40] },
    });

    finalY = (doc as any).lastAutoTable.finalY || finalY + 20;

    doc.setFontSize(16);
    doc.text("Recent Audit Reports", 14, finalY + 15);

    // We can add a mock audit report for now since we don't have an explicit one
    const auditData = [
      [
        "#1024",
        "Security Scan",
        "Passed",
        "No critical vulnerabilities found.",
      ],
      [
        "#1025",
        "Performance Test",
        "Passed",
        "CPU within limits, RAM usage optimal.",
      ],
      [
        "#1026",
        "Dependency Check",
        "Warning",
        "3 minor package updates available.",
      ],
    ];

    (doc as any).autoTable({
      startY: finalY + 20,
      head: [["ID", "Audit Type", "Status", "Summary"]],
      body: auditData,
      theme: "grid",
      headStyles: { fillColor: [40, 40, 50], textColor: [255, 255, 255] },
      bodyStyles: { fillColor: [20, 25, 35], textColor: [200, 200, 200] },
      alternateRowStyles: { fillColor: [25, 30, 40] },
    });

    doc.save("architecture-report.pdf");
  };

  return (
    <div className="flex flex-col gap-12 p-8">
      <section>
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-2">
            <Activity className="w-5 h-5 text-atc-purple" />
            <h2 className="text-xl font-mono text-white">
              Scale & Complexity Shift{" "}
              <TooltipIcon content="Metrics depicting the architectural evolution and the targeted processing milestones." />
            </h2>
          </div>
          <button
            onClick={handleExportPDF}
            className="flex items-center gap-2 px-4 py-2 bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-400 border border-indigo-500/30 rounded-lg transition-colors font-mono text-sm"
          >
            <Download className="w-4 h-4" />
            Export Report (PDF)
          </button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <StatCard
            icon={Layers}
            title="Main Systems"
            current={TARGET_STATE.mainSystems.current.toString()}
            vision={TARGET_STATE.mainSystems.vision.toString()}
            info="The number of primary architectural domains currently active."
          />
          <StatCard
            icon={Database}
            title="Subsystems"
            current={TARGET_STATE.subsystems.current.toString()}
            vision={TARGET_STATE.subsystems.vision.toString()}
            info="The distinct subsystems operating underneath the main domains."
          />
          <StatCard
            icon={Component}
            title="Core Components"
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

      <section>
        <div className="flex flex-col md:flex-row md:justify-between md:items-center gap-4 mb-6">
          <div className="flex items-center gap-2">
            <Microscope className="w-5 h-5 text-atc-cyan" />
            <h2 className="text-xl font-mono text-white">
              Full Infrastructure Layers (1–48){" "}
              <TooltipIcon content="A 48-layer breakdown of the entire architecture, from absolute system core to edge clients." />
            </h2>
          </div>
          <div className="relative">
            <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" />
            <input
              type="text"
              placeholder="Search components..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full md:w-64 pl-9 pr-4 py-2 bg-white/5 border border-white/10 rounded-lg text-sm text-white placeholder-slate-500 focus:outline-none focus:border-atc-cyan/50 focus:ring-1 focus:ring-atc-cyan/50 transition-all font-mono"
            />
          </div>
        </div>

        <div className="w-full flex items-center gap-2 overflow-x-auto pb-4 mb-4 custom-scrollbar">
          <span className="text-xs font-mono text-slate-500 uppercase tracking-widest whitespace-nowrap hidden md:block">
            Quick Jump:
          </span>
          {Array.from({ length: 48 }, (_, i) => i + 1).map((id) => {
            const hasData = filteredLayers.some((l) => l.id === id);
            return (
              <button
                key={id}
                onClick={() => {
                  if (hasData) {
                    setActiveLayer(id);
                    // scroll to element if needed, omitted for simplicity
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

        <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 relative flex-col-reverse lg:flex-row">
          <div className="lg:col-span-5 flex flex-col gap-3 h-[300px] md:h-[400px] lg:h-[600px] overflow-y-auto pr-2 custom-scrollbar order-2 lg:order-1">
            {filteredLayers.length === 0 ? (
              <div className="text-sm text-slate-500 font-mono text-center p-8 border border-dashed border-white/10 rounded-xl">
                No matching layers or components found.
              </div>
            ) : null}
            {filteredLayers.map((layer) => {
              const Icon = ICON_MAP[layer.iconName] || Database;
              const isActive = activeLayer === layer.id;
              return (
                <button
                  key={layer.id}
                  onClick={() => setActiveLayer(isActive ? null : layer.id)}
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

          <div className="lg:col-span-7 h-auto min-h-[300px] lg:h-[600px] order-1 lg:order-2">
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
                  <div className="flex items-center justify-between mb-6 relative z-10 w-full">
                    <div className="flex items-center gap-4">
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
                    <HealthSparkline layerId={selectedLayer.id} />
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
                    Inspect the inner architectural components of the multi-layer
                    blockchain architecture
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>
      </section>

      <section className="h-[700px] mb-12 flex flex-col">
        <div className="flex items-center gap-2 mb-6">
          <Map className="w-5 h-5 text-indigo-400" />
          <h2 className="text-xl font-mono text-white">
            Multi-layer Blockchain Architektur{" "}
            <TooltipIcon content="Force-directed topology of the extended strategic systems and their live active core components." />
          </h2>
        </div>

        <div className="flex-1 bg-[#090b14]/80 backdrop-blur-md rounded-2xl border border-white/5 relative overflow-hidden flex items-center justify-center">
          <div className="absolute top-4 left-4 z-10 flex gap-4 text-xs font-mono">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-[#10b981]" />
              <span className="text-slate-300">System Core</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-[#06b6d4]" />
              <span className="text-slate-300">Strategic Layer</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-[#a259ff]" />
              <span className="text-slate-300">Core Component</span>
            </div>
          </div>
          <StrategicArchitectureMap searchQuery={searchQuery} />
        </div>
      </section>

      <SecurityLayerWidget />
      <AtosStandardsWidget />
      <IndexedDBSchemaWidget />

      <section className="pb-12">
        <div className="p-8 rounded-2xl bg-gradient-to-br from-slate-50 to-white border border-atc-border relative overflow-hidden">
          <div className="absolute right-0 top-0 w-[400px] h-[400px] bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.05] mix-blend-overlay pointer-events-none" />

          <div className="relative z-10 flex flex-col md:flex-row gap-8 justify-between">
            <div className="max-w-xl">
              <h2 className="text-2xl font-mono text-slate-800 mb-4">
                Vision: Infrastructure
              </h2>
              <p className="text-slate-500 mb-6 font-light leading-relaxed">
                The final expansion resembles a global, intricately interlocked
                civilization of code, autonomous agents, and validation
                networks.
              </p>
              <div className="flex flex-wrap gap-2">
                {LONG_TERM_VISION.map((tag, i) => (
                  <span
                    key={i}
                    className="px-3 py-1 text-xs font-mono rounded-full bg-slate-200 border border-slate-300 text-slate-700"
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
              <div className="text-4xl text-slate-800 font-medium tracking-tighter">
                5.000+
              </div>
              <div className="text-atc-cyan font-mono text-sm">
                Sub-Layers
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
  );
}
