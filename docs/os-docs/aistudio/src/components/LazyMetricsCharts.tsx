import React, { useState, useEffect, useRef, useMemo } from "react";
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
  Database
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import * as d3 from "d3";
import { TooltipIcon } from "./TooltipIcon";
import { BarChart, Bar, XAxis, YAxis, Tooltip, Legend, ResponsiveContainer, LineChart, Line, CartesianGrid, AreaChart, Area } from 'recharts';

export function CommitChart({ commits }: { commits: any[] }) {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!commits.length || !svgRef.current) return;

    // Aggregate commits by date string (e.g., YYYY-MM-DD)
    const counts: Record<string, number> = {};
    commits.forEach((c) => {
      const d = new Date(c.time).toISOString().split("T")[0];
      counts[d] = (counts[d] || 0) + 1;
    });

    const data = Object.keys(counts)
      .sort()
      .map((date) => ({
        date: new Date(date),
        count: counts[date],
      }));

    if (data.length === 0) return;

    const width = 600;
    const height = 160;
    const margin = { top: 10, right: 10, bottom: 20, left: 30 };

    const svg = d3
      .select(svgRef.current)
      .attr("viewBox", `0 0 ${width} ${height}`)
      .style("width", "100%")
      .style("height", "auto");

    svg.selectAll("*").remove();

    const x = d3
      .scaleTime()
      .domain(d3.extent(data, (d) => d.date) as [Date, Date])
      .range([margin.left, width - margin.right]);

    const maxCount = d3.max(data, (d) => d.count) || 1;
    const y = d3
      .scaleLinear()
      .domain([0, maxCount])
      .range([height - margin.bottom, margin.top]);

    const line = d3
      .line<{ date: Date; count: number }>()
      .x((d) => x(d.date))
      .y((d) => y(d.count))
      .curve(d3.curveMonotoneX);

    // Axes
    svg
      .append("g")
      .attr("transform", `translate(0,${height - margin.bottom})`)
      .call(
        d3
          .axisBottom(x)
          .ticks(5)
          .tickFormat(d3.timeFormat("%m/%d") as any),
      )
      .attr("color", "#64748b");

    svg
      .append("g")
      .attr("transform", `translate(${margin.left},0)`)
      .call(d3.axisLeft(y).ticks(3))
      .attr("color", "#64748b");

    // Line path
    svg
      .append("path")
      .datum(data)
      .attr("fill", "none")
      .attr("stroke", "#06b6d4")
      .attr("stroke-width", 2)
      .attr("d", line);

    // Dots
    svg
      .selectAll("circle")
      .data(data)
      .enter()
      .append("circle")
      .attr("cx", (d) => x(d.date))
      .attr("cy", (d) => y(d.count))
      .attr("r", 4)
      .attr("fill", "#090b14")
      .attr("stroke", "#06b6d4")
      .attr("stroke-width", 2);
  }, [commits]);

  return <svg ref={svgRef} className="w-full h-40 mt-4 overflow-visible" />;
}

export function TransactionVolumeChart() {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!svgRef.current) return;

    // Generate 30 days mock data
    const data = Array.from({ length: 30 }).map((_, i) => {
      const d = new Date();
      d.setDate(d.getDate() - (29 - i));
      return {
        date: d,
        volume: Math.floor(Math.random() * 5000) + 500,
        count: Math.floor(Math.random() * 50) + 5,
      };
    });

    const width = 600;
    const height = 180;
    const margin = { top: 20, right: 30, bottom: 20, left: 40 };

    const svg = d3
      .select(svgRef.current)
      .attr("viewBox", `0 0 ${width} ${height}`)
      .style("width", "100%")
      .style("height", "auto");

    svg.selectAll("*").remove();

    const x = d3
      .scaleTime()
      .domain(d3.extent(data, (d) => d.date) as [Date, Date])
      .range([margin.left, width - margin.right]);

    const yVolume = d3
      .scaleLinear()
      .domain([0, d3.max(data, (d) => d.volume) || 1])
      .range([height - margin.bottom, margin.top]);

    const lineVolume = d3
      .line<(typeof data)[0]>()
      .x((d) => x(d.date))
      .y((d) => yVolume(d.volume))
      .curve(d3.curveMonotoneX);

    svg
      .append("g")
      .attr("transform", `translate(0,${height - margin.bottom})`)
      .call(
        d3
          .axisBottom(x)
          .ticks(6)
          .tickFormat(d3.timeFormat("%m/%d") as any),
      )
      .attr("color", "#64748b");

    // Add Area for volume
    const areaVolume = d3
      .area<(typeof data)[0]>()
      .x((d) => x(d.date))
      .y0(height - margin.bottom)
      .y1((d) => yVolume(d.volume))
      .curve(d3.curveMonotoneX);

    // Gradient
    const defs = svg.append("defs");
    const gradient = defs
      .append("linearGradient")
      .attr("id", "volGradient")
      .attr("x1", "0%")
      .attr("y1", "0%")
      .attr("x2", "0%")
      .attr("y2", "100%");
    gradient
      .append("stop")
      .attr("offset", "0%")
      .attr("stop-color", "#3b82f6")
      .attr("stop-opacity", 0.5);
    gradient
      .append("stop")
      .attr("offset", "100%")
      .attr("stop-color", "#3b82f6")
      .attr("stop-opacity", 0);

    svg
      .append("path")
      .datum(data)
      .attr("fill", "url(#volGradient)")
      .attr("d", areaVolume);

    svg
      .append("path")
      .datum(data)
      .attr("fill", "none")
      .attr("stroke", "#3b82f6")
      .attr("stroke-width", 2)
      .attr("d", lineVolume);

    // Bars for transaction count
    const yCount = d3
      .scaleLinear()
      .domain([0, (d3.max(data, (d) => d.count) || 1) * 2])
      .range([height - margin.bottom, margin.top]);

    svg
      .selectAll(".bar")
      .data(data)
      .enter()
      .append("rect")
      .attr("class", "bar")
      .attr("x", (d) => x(d.date) - 2)
      .attr("y", (d) => yCount(d.count))
      .attr("width", 4)
      .attr("height", (d) =>
        Math.max(0, height - margin.bottom - yCount(d.count)),
      )
      .attr("fill", "#f59e0b")
      .attr("opacity", 0.6)
      .attr("rx", 2);
  }, []);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl col-span-1 md:col-span-2 lg:col-span-3 relative overflow-hidden group">
      <div className="flex items-center justify-between mb-4 relative z-10">
        <div className="flex items-center gap-2 text-slate-300">
          <Activity className="w-5 h-5 text-blue-400" />
          <span className="text-sm font-bold uppercase tracking-wider font-mono">
            Wallet Tx Frequency & Volume (30D)
          </span>
          <TooltipIcon content="Tracks historical user interaction volume via transaction count and value." />
        </div>
        <div className="flex gap-4 text-xs font-mono">
          <div className="flex items-center gap-1">
            <div className="w-2 h-2 rounded bg-blue-500"></div>
            <span className="text-slate-400">Total Volume</span>
          </div>
          <div className="flex items-center gap-1">
            <div className="w-2 h-2 rounded bg-amber-500"></div>
            <span className="text-slate-400">Tx Count</span>
          </div>
        </div>
      </div>
      <div className="w-full relative z-10">
        <svg ref={svgRef} className="w-full h-48 overflow-visible" />
      </div>
    </div>
  );
}

export function IndexedDBLatencyChart() {
  const [data, setData] = useState<any[]>([]);

  useEffect(() => {
    // Generate initial specific dummy data for latency comparion
    const history = Array.from({ length: 20 }, (_, i) => ({
      time: new Date(Date.now() - (20 - i) * 60000).toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
      }),
      idbLatency: Math.floor(Math.random() * 5) + 2, // 2-6ms
      networkLatency: Math.floor(Math.random() * 80) + 40, // 40-120ms
    }));
    setData(history);

    const interval = setInterval(() => {
      setData((prev) => {
        const newData = [...prev.slice(1)];
        newData.push({
          time: new Date().toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
          }),
          idbLatency: Math.floor(Math.random() * 5) + 2,
          networkLatency: Math.floor(Math.random() * 80) + 40,
        });
        return newData;
      });
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-indigo-500/30 transition-all duration-300 w-full h-[500px] flex flex-col">
      <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-[60px] opacity-0 group-hover:opacity-100 transition-opacity" />
      <div className="flex justify-between items-center mb-4 relative z-10">
        <div className="flex items-center gap-2 text-slate-300">
          <Database className="w-4 h-4 text-indigo-400" />
          <span className="text-sm font-bold uppercase tracking-wider font-mono">
            Latency Comparison
          </span>
          <TooltipIcon content="Compares local IndexedDB read speeds against simulated network RPC request latencies." />
        </div>
      </div>
      <div className="flex-1 relative z-10 w-full min-h-0 mt-4">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" stroke="#2a2e3d" vertical={false} />
            <XAxis
              dataKey="time"
              stroke="#64748b"
              fontSize={10}
              tickLine={false}
              axisLine={false}
              minTickGap={20}
            />
            <YAxis
              stroke="#64748b"
              fontSize={10}
              tickLine={false}
              axisLine={false}
              tickFormatter={(v) => `${v}ms`}
            />
            <Tooltip
              contentStyle={{
                backgroundColor: "rgba(9, 11, 20, 0.9)",
                borderColor: "rgba(99, 102, 241, 0.3)",
                borderRadius: "8px",
                fontFamily: "monospace",
                color: "#e2e8f0",
              }}
              itemStyle={{ color: "#e2e8f0" }}
            />
            <Line
              type="monotone"
              dataKey="idbLatency"
              name="IndexedDB"
              stroke="#818cf8"
              strokeWidth={3}
              dot={false}
              activeDot={{ r: 6, fill: "#818cf8", stroke: "#090b14", strokeWidth: 2 }}
            />
            <Line
              type="monotone"
              dataKey="networkLatency"
              name="Network Fetch"
              stroke="#f43f5e"
              strokeWidth={2}
              strokeDasharray="4 4"
              dot={false}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

export function NetworkStatusHistoryWidget() {
  const data = useMemo(() => {
    return Array.from({ length: 30 }).map((_, i) => {
      const d = new Date();
      d.setDate(d.getDate() - (29 - i));
      
      // Simulate status: 0 = Devnet, 1 = Testnet, 2 = Mainnet
      let statusValue = 0;
      if (i > 25) statusValue = 2; // Last few days Mainnet
      else if (i > 10) statusValue = 1; // Middle days Testnet
      else statusValue = 0; // Early days Devnet
      
      // Add a little wobble for visual flair, though it's discrete states.
      // Actually step chart is better, but user asked for LineChart. We will use a regular LineChart.
      return {
        date: d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' }),
        status: statusValue
      };
    });
  }, []);

  const formatStatus = (val: number) => {
    if (val === 0) return 'Devnet';
    if (val === 1) return 'Testnet';
    return 'Mainnet';
  };

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-blue-500/50 transition-all duration-300 col-span-1 md:col-span-2 lg:col-span-3">
      <div className="flex items-center gap-2 text-slate-300 mb-6 relative z-10">
        <Activity className="w-5 h-5 text-blue-400" />
        <span className="text-sm font-bold uppercase tracking-wider font-mono">
          Network Status History (30 Days)
        </span>
        <TooltipIcon content="Historical record of environments accessed over the last 30 days." />
      </div>
      <div className="h-48 w-full relative z-10 text-xs font-mono">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data} margin={{ top: 5, right: 20, bottom: 5, left: 20 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" />
            <XAxis dataKey="date" stroke="#64748b" tick={{ fill: '#64748b' }} axisLine={{ stroke: '#1e293b' }} />
            <YAxis 
              stroke="#64748b" 
              tick={{ fill: '#64748b' }} 
              axisLine={{ stroke: '#1e293b' }}
              tickFormatter={formatStatus}
              domain={[0, 2]}
              ticks={[0, 1, 2]}
            />
            <Tooltip 
              contentStyle={{ backgroundColor: '#090b14', border: '1px solid #1e293b', borderRadius: '8px' }}
              itemStyle={{ color: '#3b82f6' }}
              formatter={(value: number) => [formatStatus(value), 'Status']}
            />
            <Line type="stepAfter" dataKey="status" stroke="#3b82f6" strokeWidth={2} dot={{ r: 3, fill: '#090b14', stroke: '#3b82f6' }} activeDot={{ r: 5 }} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

export function SyncDurationWidget() {
  const data = useMemo(() => Array.from({ length: 10 }).map((_, i) => ({
    attempt: `Sync #${10 - i}`,
    Auth: Math.floor(Math.random() * 200) + 50,
    Fetch: Math.floor(Math.random() * 1000) + 300,
    Transform: Math.floor(Math.random() * 500) + 100,
    Push: Math.floor(Math.random() * 600) + 200
  })).reverse(), []);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-indigo-500/50 transition-all duration-300 col-span-1 md:col-span-2 lg:col-span-2">
      <div className="flex items-center gap-2 text-slate-300 mb-6 relative z-10">
        <Activity className="w-5 h-5 text-indigo-400" />
        <span className="text-sm font-bold uppercase tracking-wider font-mono">
          Sync Stage Bottlenecks (Last 10 Attempts - ms)
        </span>
        <TooltipIcon content="Identifies which data sync phase takes the longest time." />
      </div>
      <div className="h-48 w-full relative z-10 text-xs font-mono">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart data={data} margin={{ top: 5, right: 20, bottom: 5, left: 0 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} />
            <XAxis dataKey="attempt" stroke="#64748b" tick={{ fill: '#64748b' }} axisLine={{ stroke: '#1e293b' }} />
            <YAxis stroke="#64748b" tick={{ fill: '#64748b' }} axisLine={{ stroke: '#1e293b' }} />
            <Tooltip 
              contentStyle={{ backgroundColor: '#090b14', border: '1px solid #1e293b', borderRadius: '8px' }}
              itemStyle={{ color: '#818cf8' }}
              cursor={{ fill: '#1e293b', opacity: 0.4 }}
            />
            <Legend />
            <Bar dataKey="Auth" stackId="a" fill="#6366f1" />
            <Bar dataKey="Fetch" stackId="a" fill="#3b82f6" />
            <Bar dataKey="Transform" stackId="a" fill="#10b981" />
            <Bar dataKey="Push" stackId="a" fill="#8b5cf6" radius={[4, 4, 0, 0]} />
          </BarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

export function APIHealthWidget() {
  const [showLogsModal, setShowLogsModal] = useState(false);
  const data = useMemo(() => {
    return Array.from({ length: 20 }).map((_, i) => {
      const d = new Date();
      d.setMinutes(d.getMinutes() - (19 - i) * 5); // every 5 minutes
      return {
        time: d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        githubLatency: 150 + Math.random() * 50 - 25,
        notionLatency: 300 + Math.random() * 100 - 50,
      };
    });
  }, []);

  const errorLogs = useMemo(() => [
    { source: 'GitHub', error: '403 Forbidden: API rate limit exceeded for 185.122.x.x', time: '10:45 AM' },
    { source: 'Notion', error: '502 Bad Gateway: Upstream server timed out', time: '11:15 AM' },
    { source: 'GitHub', error: '401 Unauthorized: Invalid or expired token', time: '01:20 PM' },
    { source: 'Notion', error: '400 Bad Request: Missing required header', time: '02:05 PM' },
    { source: 'GitHub', error: '500 Internal Server Error: Database timeout', time: '03:10 PM' },
    { source: 'Notion', error: '429 Too Many Requests: Rate limit exceeded', time: '04:00 PM' },
    { source: 'GitHub', error: 'ECONNRESET: Connection reset by peer', time: '04:30 PM' },
    { source: 'Notion', error: '504 Gateway Timeout: Fetching page metadata failed', time: '05:15 PM' },
    { source: 'GitHub', error: '422 Unprocessable Entity: Validation failed', time: '06:05 PM' },
    { source: 'Notion', error: '404 Not Found: Page configuration missing', time: '06:50 PM' },
  ], []);

  return (
    <>
      <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-cyan-500/50 transition-all duration-300 col-span-1 md:col-span-2 lg:col-span-2">
        <div className="flex items-center justify-between mb-6 relative z-10">
          <div className="flex items-center gap-2 text-slate-300">
            <Server className="w-5 h-5 text-cyan-400" />
            <span className="text-sm font-bold uppercase tracking-wider font-mono">
              API Health & Latency
            </span>
            <TooltipIcon content="Real-time latency metrics for core integrated API endpoints." />
          </div>
          <div className="flex items-center gap-4">
            <button 
              onClick={() => setShowLogsModal(true)}
              className="px-3 py-1 bg-white/5 hover:bg-white/10 text-slate-300 text-xs font-mono rounded border border-white/10 transition-colors"
            >
              View Logs
            </button>
            <div className="flex flex-col items-end">
              <span className="text-[10px] text-slate-500 font-mono tracking-widest uppercase">GitHub Uptime</span>
              <span className="text-sm font-bold text-emerald-400 font-mono">99.98%</span>
            </div>
            <div className="flex flex-col items-end">
              <span className="text-[10px] text-slate-500 font-mono tracking-widest uppercase">Notion Uptime</span>
              <span className="text-sm font-bold text-emerald-400 font-mono">99.95%</span>
            </div>
          </div>
        </div>
        <div className="h-48 w-full relative z-10 text-xs font-mono">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={data} margin={{ top: 5, right: 20, bottom: 5, left: 0 }}>
              <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" />
              <XAxis dataKey="time" stroke="#64748b" tick={{ fill: '#64748b' }} axisLine={{ stroke: '#1e293b' }} />
              <YAxis stroke="#64748b" tick={{ fill: '#64748b' }} axisLine={{ stroke: '#1e293b' }} />
              <Tooltip 
                contentStyle={{ backgroundColor: '#090b14', border: '1px solid #1e293b', borderRadius: '8px' }}
              />
              <Legend />
              <Line type="monotone" dataKey="githubLatency" name="GitHub API (ms)" stroke="#06b6d4" strokeWidth={2} dot={false} />
              <Line type="monotone" dataKey="notionLatency" name="Notion API (ms)" stroke="#8b5cf6" strokeWidth={2} dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>
      
      <AnimatePresence>
        {showLogsModal && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="absolute inset-0 bg-black/60 backdrop-blur-sm"
              onClick={() => setShowLogsModal(false)}
            />
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="bg-[#090b14] border border-white/10 rounded-2xl w-full max-w-2xl max-h-[80vh] flex flex-col shadow-2xl relative z-10 overflow-hidden"
            >
              <div className="p-4 border-b border-white/10 flex items-center justify-between bg-black/50">
                <h3 className="text-sm font-bold font-mono text-slate-200">Raw API Error Logs</h3>
                <button
                  onClick={() => setShowLogsModal(false)}
                  className="text-slate-500 hover:text-white p-1 rounded-md"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>
              <div className="p-4 overflow-y-auto font-mono text-xs text-slate-300 flex flex-col gap-2 bg-[#090b14]">
                {errorLogs.map((log, i) => (
                  <div key={i} className="p-3 bg-black/40 border border-white/5 rounded-lg flex flex-col gap-1">
                    <div className="flex justify-between items-center text-[10px] text-slate-500 mb-1">
                      <span className={`px-2 py-0.5 rounded font-bold ${log.source === 'GitHub' ? 'bg-cyan-500/20 text-cyan-400' : 'bg-purple-500/20 text-purple-400'}`}>
                        {log.source}
                      </span>
                      <span>{log.time}</span>
                    </div>
                    <code className="text-red-400 font-mono text-xs">{log.error}</code>
                  </div>
                ))}
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
    </>
  );
}

export function NetworkLatencyFluctuationWidget() {
  const [data, setData] = useState<{ time: string; latency: number }[]>([]);

  useEffect(() => {
    // Generate initial data for the last 60 minutes
    const initialData = Array.from({ length: 60 }).map((_, i) => {
      const d = new Date();
      d.setMinutes(d.getMinutes() - (59 - i));
      return {
        time: d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        latency: 40 + Math.random() * 20,
      };
    });
    setData(initialData);

    const interval = setInterval(() => {
      setData((prev) => {
        const d = new Date();
        const nextTime = d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
        // prevent duplicate time entries if updating in same minute, mostly for aesthetic in "real-time" update
        const newData = [...prev];
        const lastEntry = newData[newData.length - 1];
        if (lastEntry.time === nextTime) {
           newData[newData.length - 1] = { time: nextTime, latency: 40 + Math.random() * 30 + (Math.random() > 0.9 ? 50 : 0) };
        } else {
           newData.shift();
           newData.push({ time: nextTime, latency: 40 + Math.random() * 30 + (Math.random() > 0.9 ? 50 : 0) });
        }
        return newData;
      });
    }, 2000); // update every 2 seconds roughly

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-pink-500/50 transition-all duration-300 col-span-1 md:col-span-2 lg:col-span-3">
      <div className="flex items-center gap-2 text-slate-300 mb-6 relative z-10">
        <Activity className="w-5 h-5 text-pink-400" />
        <span className="text-sm font-bold uppercase tracking-wider font-mono">
          Network Latency Fluctuations (Last Hour)
        </span>
        <TooltipIcon content="Short term jitter and response time variance across the mesh." />
      </div>
      <div className="h-48 w-full relative z-10 text-xs font-mono">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data} margin={{ top: 5, right: 20, bottom: 5, left: 0 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} />
            <XAxis 
              dataKey="time" 
              stroke="#64748b" 
              tick={{ fill: '#64748b' }} 
              axisLine={{ stroke: '#1e293b' }} 
              minTickGap={30}
            />
            <YAxis 
              stroke="#64748b" 
              tick={{ fill: '#64748b' }} 
              axisLine={{ stroke: '#1e293b' }} 
              domain={['auto', 'auto']}
              tickFormatter={(v) => `${v.toFixed(0)}ms`}
            />
            <Tooltip 
              contentStyle={{ backgroundColor: '#090b14', border: '1px solid #1e293b', borderRadius: '8px' }}
              itemStyle={{ color: '#ec4899' }}
              labelStyle={{ color: '#94a3b8' }}
              formatter={(val: number) => [`${val.toFixed(2)} ms`, 'Latency']}
            />
            <Line 
              type="monotone" 
              dataKey="latency" 
              stroke="#ec4899" 
              strokeWidth={2} 
              dot={false}
              activeDot={{ r: 4, fill: '#ec4899', stroke: '#090b14' }}
              isAnimationActive={false}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

export function APIResponseTimeWidget() {
  const [data, setData] = useState(
    Array.from({ length: 10 }).map((_, i) => ({
      time: `-${10 - i}m`,
      timeMs: 150 + Math.random() * 50
    }))
  );

  useEffect(() => {
    const interval = setInterval(() => {
      setData(prev => {
        const next = [...prev.slice(1)];
        next.push({
          time: 'now',
          timeMs: 150 + Math.random() * 50 + (Math.random() > 0.8 ? 400 : 0) // some spikes over 500ms
        });
        return next;
      });
    }, 5000);
    return () => clearInterval(interval);
  }, []);

  const isWarning = data.length > 0 && data[data.length - 1].timeMs > 500;

  return (
    <div className={`bg-[#090b14]/80 backdrop-blur-md border ${isWarning ? 'border-red-500/50 shadow-[0_0_20px_rgba(239,68,68,0.2)]' : 'border-atc-border/80 shadow-2xl'} rounded-2xl p-6 relative overflow-hidden group hover:border-atc-purple/50 hover:-translate-y-1 transition-all duration-300`}>
      <div className="flex items-center justify-between mb-6 relative z-10">
        <div className="flex items-center gap-2 text-slate-300">
          <Activity className={`w-4 h-4 ${isWarning ? 'text-red-500 animate-pulse' : 'text-atc-purple'}`} />
          <span className={`text-sm font-medium uppercase tracking-wider font-mono ${isWarning ? 'text-red-400' : ''}`}>
            API Response Time
          </span>
        </div>
        {isWarning && (
          <span className="text-[10px] bg-red-500/10 text-red-400 border border-red-500/30 px-2 py-0.5 rounded font-mono font-bold animate-pulse">
            HIGH LATENCY WARNING
          </span>
        )}
      </div>
      <div className="h-48 w-full mt-4">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" stroke="#2a3040" vertical={false} />
            <XAxis dataKey="time" stroke="#64748b" tick={{ fontSize: 10, fill: '#64748b' }} tickLine={false} axisLine={false} />
            <YAxis stroke="#64748b" tick={{ fontSize: 10, fill: '#64748b' }} tickLine={false} axisLine={false} width={40} />
            <Tooltip
              contentStyle={{ backgroundColor: '#090b14', border: '1px solid #1e293b', borderRadius: '8px' }}
              itemStyle={{ color: '#a259ff', fontFamily: 'monospace' }}
              labelStyle={{ color: '#94a3b8', marginBottom: '4px' }}
              formatter={(value: any) => [
                <span style={{ color: value > 500 ? '#ef4444' : '#a259ff' }}>{value.toFixed(0)} ms</span>,
                "Latency"
              ]}
            />
            <Line type="monotone" dataKey="timeMs" stroke="#a259ff" strokeWidth={2} dot={<CustomResponseTimeDot />} activeDot={{ r: 5 }} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

export const CustomResponseTimeDot = (props: any) => {
  const { cx, cy, payload } = props;
  const isHigh = payload.timeMs > 500;
  return (
    <circle 
      cx={cx} 
      cy={cy} 
      r={isHigh ? 5 : 3} 
      fill={isHigh ? '#ef4444' : '#a259ff'} 
      stroke={isHigh ? '#fca5a5' : 'none'} 
      strokeWidth={isHigh ? 2 : 0} 
      className={isHigh ? "animate-pulse" : ""}
    />
  );
};

export function SyncHealthWidget() {
  const data = useMemo(() => {
    return Array.from({ length: 15 }).map((_, i) => {
      const d = new Date();
      d.setDate(d.getDate() - (14 - i));
      return {
        date: d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' }),
        githubSuccess: 80 + Math.random() * 20,
        githubFail: Math.random() * 5,
        notionSuccess: 75 + Math.random() * 25,
        notionFail: Math.random() * 8,
      };
    });
  }, []);

  return (
    <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden group hover:border-emerald-500/50 transition-all duration-300 col-span-1 md:col-span-2 lg:col-span-3">
      <div className="flex items-center gap-2 text-slate-300 mb-6 relative z-10">
        <HeartPulse className="w-5 h-5 text-emerald-400" />
        <span className="text-sm font-bold uppercase tracking-wider font-mono">
          Sync Health: Success vs Failure
        </span>
      </div>
      <div className="h-48 w-full relative z-10 text-xs font-mono">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={data} margin={{ top: 5, right: 20, bottom: 5, left: 0 }}>
            <defs>
              <linearGradient id="colorGS" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#10b981" stopOpacity={0.8}/>
                <stop offset="95%" stopColor="#10b981" stopOpacity={0}/>
              </linearGradient>
              <linearGradient id="colorGF" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#ef4444" stopOpacity={0.8}/>
                <stop offset="95%" stopColor="#ef4444" stopOpacity={0}/>
              </linearGradient>
              <linearGradient id="colorNS" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.8}/>
                <stop offset="95%" stopColor="#3b82f6" stopOpacity={0}/>
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} />
            <XAxis dataKey="date" stroke="#64748b" tick={{ fill: '#64748b' }} axisLine={{ stroke: '#1e293b' }} />
            <YAxis stroke="#64748b" tick={{ fill: '#64748b' }} axisLine={{ stroke: '#1e293b' }} />
            <Tooltip 
              contentStyle={{ backgroundColor: '#090b14', border: '1px solid #1e293b', borderRadius: '8px' }}
            />
            <Legend />
            <Area type="monotone" dataKey="githubSuccess" name="GH Success" stroke="#10b981" fillOpacity={1} fill="url(#colorGS)" />
            <Area type="monotone" dataKey="githubFail" name="GH Fail" stroke="#ef4444" fillOpacity={1} fill="url(#colorGF)" />
            <Area type="monotone" dataKey="notionSuccess" name="Notion Success" stroke="#3b82f6" fillOpacity={1} fill="url(#colorNS)" />
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

