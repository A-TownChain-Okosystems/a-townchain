import React, { useState, useEffect, useRef } from "react";
import {
  Terminal,
  Cpu,
  BrainCircuit,
  Activity,
  Zap,
  Server,
  Code,
  PlaySquare,
  Workflow,
  ChevronRight,
  ShieldAlert,
  Cpu as CpuIcon,
  MemoryStick,
  Database,
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

type Message = {
  id: string;
  sender: "user" | "atc";
  text: string;
  timestamp: Date;
  status?: "processing" | "done";
};

const OS_PROCESSES = [
  {
    name: "ATC-Core Intelligence",
    type: "AI Process",
    usage: 42,
    status: "Active",
    id: "pid-001",
  },
  {
    name: "ATVM Runtime Sandbox",
    type: "Execution",
    usage: 15,
    status: "Active",
    id: "pid-002",
  },
  {
    name: "P2P Gossip Router",
    type: "Network",
    usage: 8,
    status: "Active",
    id: "pid-003",
  },
  {
    name: "ZK-Snark Verifier",
    type: "Cryptography",
    usage: 22,
    status: "Active",
    id: "pid-004",
  },
  {
    name: "State Sync Daemon",
    type: "Storage",
    usage: 5,
    status: "Idle",
    id: "pid-005",
  },
];

export function AiOsEngineView() {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: "msg-1",
      sender: "atc",
      text: "Initializing ATC-OS Core Intelligence... System operational. Blockchain integration verified. How may I assist you with network operations or OS management today?",
      timestamp: new Date(),
    },
  ]);
  const [input, setInput] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSend = () => {
    if (!input.trim()) return;

    const userMsg: Message = {
      id: Date.now().toString(),
      sender: "user",
      text: input,
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, userMsg]);
    setInput("");
    setIsProcessing(true);

    // Simulate AI response
    setTimeout(() => {
      let replyText =
        "I have processed your request. Analyzing current network state and OS parameters.";

      const lowerInput = userMsg.text.toLowerCase();
      if (lowerInput.includes("contract") || lowerInput.includes("smart")) {
        replyText =
          "Initiating AI-driven smart contract generation sandbox. Compiling reference ATVM bytecode...";
      } else if (
        lowerInput.includes("analyze") ||
        lowerInput.includes("mempool")
      ) {
        replyText =
          "Analyzing mempool congestion... Found 142 pending transactions. Gas optimization protocols have been activated.";
      } else if (
        lowerInput.includes("health") ||
        lowerInput.includes("status")
      ) {
        replyText =
          "System health is optimal. Active nodes: 124. TPS: 4500. Memory usage across distributed OS instances: 42%.";
      } else if (
        lowerInput.includes("update") ||
        lowerInput.includes("blockchain")
      ) {
        replyText =
          "Preparing blockchain update sequence. ATC Core will now dynamically adjust the consensus weights to prioritize transaction speed.";
      }

      setMessages((prev) => [
        ...prev,
        {
          id: (Date.now() + 1).toString(),
          sender: "atc",
          text: replyText,
          timestamp: new Date(),
        },
      ]);
      setIsProcessing(false);
    }, 1500);
  };

  const executeQuickAction = (action: string) => {
    setInput(action);
    setTimeout(() => handleSend(), 100);
  };

  return (
    <div className="flex flex-col xl:flex-row gap-6 mt-8 pb-12 w-full max-w-7xl mx-auto px-4 sm:px-0">
      {/* ATC Interaction Panel */}
      <div className="flex-1 bg-[#090b14]/80 backdrop-blur-md border border-atc-border/50 rounded-3xl flex flex-col overflow-hidden shadow-2xl relative">
        <div className="absolute top-0 right-0 w-64 h-64 bg-atc-cyan/5 rounded-full blur-[80px]" />

        <div className="p-6 border-b border-white/5 bg-white/5 flex items-center justify-between relative z-10">
          <div className="flex items-center gap-4">
            <div className="relative">
              <div className="w-12 h-12 rounded-xl bg-atc-cyan/20 border border-atc-cyan/40 flex items-center justify-center text-atc-cyan">
                <BrainCircuit className="w-6 h-6" />
              </div>
              {isProcessing && (
                <span className="absolute -top-1 -right-1 w-3 h-3 bg-atc-cyan rounded-full animate-ping" />
              )}
            </div>
            <div>
              <h2 className="text-xl font-bold text-white tracking-tight">
                ATC Core Intelligence
              </h2>
              <div className="flex items-center gap-2">
                <div className="relative flex h-2 w-2">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                </div>
                <span className="text-xs font-mono text-emerald-400">
                  Model: ATC-X v4.2 / On-Chain Active
                </span>
              </div>
            </div>
          </div>

          <div className="hidden lg:flex gap-2">
            <button
              onClick={() => executeQuickAction("Analyze mempool")}
              className="px-3 py-1.5 bg-white/5 hover:bg-white/10 rounded-lg text-xs font-medium text-slate-300 transition-colors border border-white/5"
            >
              Analyze State
            </button>
            <button
              onClick={() => executeQuickAction("Audit Smart Contract")}
              className="px-3 py-1.5 bg-white/5 hover:bg-white/10 rounded-lg text-xs font-medium text-slate-300 transition-colors border border-white/5"
            >
              Audit Code
            </button>
            <button
              onClick={() => executeQuickAction("Optimize Network Routing")}
              className="px-3 py-1.5 bg-white/5 hover:bg-white/10 rounded-lg text-xs font-medium text-slate-300 transition-colors border border-white/5"
            >
              Optimize OS
            </button>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto p-6 space-y-6 custom-scrollbar relative z-10 min-h-[400px]">
          <AnimatePresence initial={false}>
            {messages.map((msg) => (
              <motion.div
                key={msg.id}
                initial={{ opacity: 0, y: 10, scale: 0.98 }}
                animate={{ opacity: 1, y: 0, scale: 1 }}
                className={`flex gap-4 ${msg.sender === "user" ? "flex-row-reverse" : "flex-row"}`}
              >
                <div
                  className={`w-8 h-8 rounded-lg flex items-center justify-center shrink-0 ${
                    msg.sender === "user"
                      ? "bg-slate-700 text-white"
                      : "bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/30"
                  }`}
                >
                  {msg.sender === "user" ? (
                    <Terminal className="w-4 h-4" />
                  ) : (
                    <BrainCircuit className="w-4 h-4" />
                  )}
                </div>
                <div
                  className={`max-w-[80%] rounded-2xl p-4 ${
                    msg.sender === "user"
                      ? "bg-slate-800 text-slate-200 rounded-tr-sm border border-slate-700"
                      : "bg-black/40 text-atc-cyan/90 border border-atc-cyan/20 rounded-tl-sm shadow-[0_0_15px_rgba(45,212,191,0.05)]"
                  }`}
                >
                  <p className="text-sm leading-relaxed">{msg.text}</p>
                  <p className="text-[10px] opacity-50 mt-2 text-right">
                    {msg.timestamp.toLocaleTimeString()}
                  </p>
                </div>
              </motion.div>
            ))}
          </AnimatePresence>
          {isProcessing && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="flex gap-4"
            >
              <div className="w-8 h-8 rounded-lg bg-atc-cyan/20 text-atc-cyan border border-atc-cyan/30 flex items-center justify-center shrink-0">
                <BrainCircuit className="w-4 h-4 animate-pulse" />
              </div>
              <div className="bg-black/40 border border-atc-cyan/20 rounded-2xl rounded-tl-sm p-4 flex items-center gap-2">
                <span
                  className="w-2 h-2 bg-atc-cyan rounded-full animate-bounce"
                  style={{ animationDelay: "0ms" }}
                />
                <span
                  className="w-2 h-2 bg-atc-cyan rounded-full animate-bounce"
                  style={{ animationDelay: "150ms" }}
                />
                <span
                  className="w-2 h-2 bg-atc-cyan rounded-full animate-bounce"
                  style={{ animationDelay: "300ms" }}
                />
              </div>
            </motion.div>
          )}
          <div ref={messagesEndRef} />
        </div>

        <div className="p-4 border-t border-white/5 bg-black/40 relative z-10">
          <form
            onSubmit={(e) => {
              e.preventDefault();
              handleSend();
            }}
            className="flex gap-3 relative"
          >
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              disabled={isProcessing}
              placeholder="Ask ATC to optimize the blockchain or manage OS processes..."
              className="flex-1 bg-slate-900/50 border border-white/10 rounded-xl px-5 py-4 text-sm text-white focus:outline-none focus:border-atc-cyan transition-colors disabled:opacity-50 font-mono placeholder:font-sans"
            />
            <button
              type="submit"
              disabled={!input.trim() || isProcessing}
              className="bg-atc-cyan hover:bg-atc-cyan/90 disabled:opacity-50 disabled:cursor-not-allowed text-slate-900 rounded-xl px-6 flex items-center justify-center transition-colors shadow-[0_0_20px_rgba(45,212,191,0.2)] hover:shadow-[0_0_30px_rgba(45,212,191,0.4)]"
            >
              <ChevronRight className="w-6 h-6" />
            </button>
          </form>
        </div>
      </div>

      {/* OS & Blockchain Insights Side Panel */}
      <div className="w-full xl:w-96 flex flex-col gap-6">
        {/* OS Resources */}
        <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/50 rounded-3xl p-6 shadow-xl">
          <h3 className="text-white font-bold mb-4 flex items-center gap-2 text-lg">
            <Server className="w-5 h-5 text-indigo-400" /> ATC-OS Processes
          </h3>

          <div className="grid grid-cols-2 gap-4 mb-6">
            <div className="bg-white/5 rounded-xl p-3 border border-white/5">
              <div className="flex items-center gap-2 mb-1">
                <CpuIcon className="w-4 h-4 text-emerald-400" />
                <span className="text-xs text-slate-400 font-mono">CPU</span>
              </div>
              <div className="text-lg font-bold text-white">42%</div>
              <div className="w-full bg-black rounded-full h-1 mt-2">
                <div
                  className="bg-emerald-400 h-1 rounded-full"
                  style={{ width: "42%" }}
                />
              </div>
            </div>
            <div className="bg-white/5 rounded-xl p-3 border border-white/5">
              <div className="flex items-center gap-2 mb-1">
                <MemoryStick className="w-4 h-4 text-blue-400" />
                <span className="text-xs text-slate-400 font-mono">RAM</span>
              </div>
              <div className="text-lg font-bold text-white">16.4 GB</div>
              <div className="w-full bg-black rounded-full h-1 mt-2">
                <div
                  className="bg-blue-400 h-1 rounded-full"
                  style={{ width: "68%" }}
                />
              </div>
            </div>
          </div>

          <div className="flex gap-2">
            <span className="px-2 py-1 rounded-md bg-atc-purple/10 border border-atc-purple/30 text-[10px] uppercase font-mono tracking-widest text-atc-purple flex items-center gap-1"><ShieldAlert className="w-3 h-3" /> AGENTS.md Enforced</span>
            <span className="px-2 py-1 rounded-md bg-indigo-500/10 border border-indigo-500/30 text-[10px] uppercase font-mono tracking-widest text-indigo-400 flex items-center gap-1"><Terminal className="w-3 h-3" /> GEMINI.md Active</span>
          </div>

          <div className="space-y-3">
            <div className="text-xs font-mono text-slate-500 flex justify-between px-2 uppercase tracking-widest">
              <span>Process</span>
              <span>Load</span>
            </div>
            {OS_PROCESSES.map((proc, i) => (
              <div
                key={i}
                className="flex items-center justify-between p-3 bg-black/40 border border-white/5 rounded-xl hover:border-white/10 transition-colors"
              >
                <div className="flex items-center gap-3">
                  {proc.status === "Active" ? (
                    <div className="relative flex h-2 w-2">
                      <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                      <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                    </div>
                  ) : (
                    <div className="w-2 h-2 rounded-full bg-slate-600" />
                  )}
                  <div>
                    <div className="text-sm text-slate-200 font-medium">
                      {proc.name}
                    </div>
                    <div className="text-[10px] text-slate-500 font-mono mt-0.5">
                      {proc.type} | {proc.id}
                    </div>
                  </div>
                </div>
                <div className="text-xs font-mono text-atc-cyan">
                  {proc.usage}%
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Blockchain AI Capabilities */}
        <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/50 rounded-3xl p-6 shadow-xl relative overflow-hidden flex-1">
          <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-[40px] pointer-events-none" />
          <h3 className="text-white font-bold mb-4 flex items-center gap-2 text-lg relative z-10">
            <Workflow className="w-5 h-5 text-fuchsia-400" /> AI Capabilities
          </h3>

          <div className="space-y-3 relative z-10">
            <div
              className="p-4 rounded-xl border border-fuchsia-500/20 bg-fuchsia-500/5 hover:bg-fuchsia-500/10 transition-colors group cursor-pointer"
              onClick={() =>
                executeQuickAction(
                  "Generate a decentralized exchange smart contract",
                )
              }
            >
              <div className="flex items-center gap-3 mb-2">
                <Code className="w-5 h-5 text-fuchsia-400" />
                <h4 className="text-sm font-bold text-white">
                  Smart Contract Gen
                </h4>
              </div>
              <p className="text-xs text-slate-400 leading-relaxed">
                Let ATC write, audit, and compile secure ATVM code using natural
                language prompts.
              </p>
            </div>

            <div
              className="p-4 rounded-xl border border-amber-500/20 bg-amber-500/5 hover:bg-amber-500/10 transition-colors group cursor-pointer"
              onClick={() =>
                executeQuickAction("Scan mempool for front-running attacks")
              }
            >
              <div className="flex items-center gap-3 mb-2">
                <ShieldAlert className="w-5 h-5 text-amber-400" />
                <h4 className="text-sm font-bold text-white">
                  Threat Detection
                </h4>
              </div>
              <p className="text-xs text-slate-400 leading-relaxed">
                Continuous deep learning analysis of pending transactions to
                prevent MEV attacks.
              </p>
            </div>

            <div
              className="p-4 rounded-xl border border-blue-500/20 bg-blue-500/5 hover:bg-blue-500/10 transition-colors group cursor-pointer"
              onClick={() => executeQuickAction("Analyze chain state dynamics")}
            >
              <div className="flex items-center gap-3 mb-2">
                <Database className="w-5 h-5 text-blue-400" />
                <h4 className="text-sm font-bold text-white">
                  State Resonance
                </h4>
              </div>
              <p className="text-xs text-slate-400 leading-relaxed">
                Predictive scaling of validator subsets based on upcoming
                computational load.
              </p>
            </div>
          </div>
        </div>

        {/* AI Heuristics Optimization Panel */}
        <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/50 rounded-3xl p-6 shadow-xl relative overflow-hidden">
          <div className="flex items-center justify-between mb-4 relative z-10">
            <h3 className="text-white font-bold flex items-center gap-2 text-sm">
              <Zap className="w-4 h-4 text-emerald-400" /> Adaptive Optimization
            </h3>
            <span className="px-2 py-0.5 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-[10px] text-emerald-400 font-mono tracking-widest uppercase">
              Auto-Pilot
            </span>
          </div>

          <div className="space-y-4">
            <div className="flex flex-col gap-2 p-3 rounded-xl border border-white/5 bg-white/5">
              <div className="flex items-center justify-between text-xs">
                <span className="text-slate-300 font-medium">
                  Memory Allocation
                </span>
                <span className="text-slate-400 font-mono">
                  Suggested: +12%
                </span>
              </div>
              <div className="text-[10px] text-slate-500">
                Heuristics detect increased smart contract compilation load.
                Reallocating from State Sync...
              </div>
              <button
                onClick={() =>
                  executeQuickAction("Apply memory reallocation heuristics")
                }
                className="mt-1 w-full py-1.5 rounded-lg border border-emerald-500/50 text-emerald-400 text-xs font-medium hover:bg-emerald-500/10 transition-colors"
              >
                Apply Preset
              </button>
            </div>

            <div className="flex flex-col gap-2 p-3 rounded-xl border border-white/5 bg-white/5">
              <div className="flex items-center justify-between text-xs">
                <span className="text-slate-300 font-medium">
                  Gossip Subnets
                </span>
                <span className="text-amber-400 font-mono">Degraded (89%)</span>
              </div>
              <div className="text-[10px] text-slate-500">
                Network topology favors restructuring due to new cross-region
                nodes.
              </div>
              <button
                onClick={() => executeQuickAction("Optimize subnet topology")}
                className="mt-1 w-full py-1.5 rounded-lg border border-amber-500/50 text-amber-400 text-xs font-medium hover:bg-amber-500/10 transition-colors"
              >
                Reroute Nodes
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
