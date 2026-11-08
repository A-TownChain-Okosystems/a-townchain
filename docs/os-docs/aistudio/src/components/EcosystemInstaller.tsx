import React, { useState, useEffect } from "react";
import { 
  CheckCircle2, 
  Circle, 
  HardDrive, 
  Cpu, 
  Network, 
  Database, 
  Bot, 
  Terminal, 
  ShieldCheck, 
  Zap,
  Loader2,
  Settings,
  ArrowRight
} from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

const INSTALLATION_STEPS = [
  {
    id: "core",
    title: "Core Kernel & Runtime",
    description: "Installing A-TownOS-Kernel, ATVM Sandbox, and basic OS configurations.",
    icon: Settings,
    duration: 3000,
    logs: [
      "Initializing A-TownOS Kernel v3.1...",
      "Allocating memory mapped regions...",
      "Compiling ATVM Sandbox instructions...",
      "Starting System Daemons...",
      "Core Kernel setup complete."
    ]
  },
  {
    id: "blockchain",
    title: "Blockchain & Consensus",
    description: "Setting up local node, ledger database, and Zero-Knowledge circuit templates.",
    icon: Network,
    duration: 4000,
    logs: [
      "Generating cryptographic keypairs...",
      "Allocating ledger storage (LevelDB)...",
      "Bootstrapping P2P Subnet...",
      "Synchronizing initial block headers...",
      "Zero-Knowledge circuit compilation finished.",
      "Blockchain Node Online."
    ]
  },
  {
    id: "storage",
    title: "Distributed Datalake",
    description: "Configuring IPFS modules, decentralized object storage, and sharding logic.",
    icon: Database,
    duration: 3500,
    logs: [
      "Configuring distributed hash tables (DHT)...",
      "Setting up redundancy protocols...",
      "Mounting Virtual Filesystem (VFS)...",
      "Storage node registered successfully."
    ]
  },
  {
    id: "ai",
    title: "AI Agency & Governors",
    description: "Integrating Gemini API weights, agent sandbox, and neural workflow orchestrators.",
    icon: Bot,
    duration: 4500,
    logs: [
      "Connecting to local AI Service Layer...",
      "Loading Gemini-3.1-pro models...",
      "Initializing AI Governor heuristics...",
      "Evaluating agent system prompt integrity...",
      "AI Service Layer active and ready."
    ]
  },
  {
    id: "devtools",
    title: "Developer Tools & UI",
    description: "Installing ATC-Lang compiler, standard libraries, and desktop interface apps.",
    icon: Terminal,
    duration: 2500,
    logs: [
      "Unpacking ATC-Lang standard library...",
      "Setting up IDE syntax configurations...",
      "Installing global UI registry...",
      "Developer stack installation complete."
    ]
  }
];

export function EcosystemInstaller() {
  const [currentStepIndex, setCurrentStepIndex] = useState(-1); // -1 means wait to start
  const [completedSteps, setCompletedSteps] = useState<string[]>([]);
  const [isFinished, setIsFinished] = useState(false);
  const [currentLogs, setCurrentLogs] = useState<string[]>([]);
  const [progress, setProgress] = useState(0);

  const startInstallation = () => {
    setCurrentStepIndex(0);
    setCompletedSteps([]);
    setIsFinished(false);
    setCurrentLogs([]);
    setProgress(0);
  };

  useEffect(() => {
    if (currentStepIndex === -1 || isFinished) return;

    if (currentStepIndex >= INSTALLATION_STEPS.length) {
      setIsFinished(true);
      return;
    }

    const step = INSTALLATION_STEPS[currentStepIndex];
    let logIndex = 0;
    
    // Distribute logs evenly over the duration
    const logIntervalTime = step.duration / step.logs.length;
    
    const logInterval = setInterval(() => {
      if (logIndex < step.logs.length) {
        setCurrentLogs(prev => [...prev, step.logs[logIndex]]);
        logIndex++;
      }
    }, logIntervalTime);

    // Progress bar animation
    let p = 0;
    const pInterval = setInterval(() => {
        p += 2;
        if(p > 100) p = 100;
        setProgress(p);
    }, (step.duration / 50));

    const completeTimeout = setTimeout(() => {
      clearInterval(logInterval);
      clearInterval(pInterval);
      setCompletedSteps(prev => [...prev, step.id]);
      setProgress(0);
      setCurrentStepIndex(prev => prev + 1);
    }, step.duration);

    return () => {
      clearTimeout(completeTimeout);
      clearInterval(logInterval);
      clearInterval(pInterval);
    };
  }, [currentStepIndex, isFinished]);

  return (
    <div className="w-full h-full bg-[#0a0d16] text-white flex flex-col font-sans relative overflow-hidden">
      {/* Background decorations */}
      <div className="absolute top-[-20%] left-[-10%] w-96 h-96 bg-atc-cyan/5 rounded-full blur-[100px] pointer-events-none" />
      <div className="absolute bottom-[-20%] right-[-10%] w-96 h-96 bg-indigo-500/10 rounded-full blur-[100px] pointer-events-none" />

      <div className="flex-1 overflow-y-auto p-8 z-10 flex flex-col md:flex-row gap-8 max-w-7xl mx-auto w-full">
        
        {/* Left Column: Flow/Progress */}
        <div className="w-full md:w-1/2 flex flex-col space-y-6">
          <div className="mb-6">
            <h1 className="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-atc-cyan to-indigo-400 mb-2 flex items-center gap-3">
              <Zap className="text-atc-cyan w-8 h-8"/>
              A-TownChain Ecosystem
            </h1>
            <p className="text-slate-400">Initialize and configure your centralized local OS environment.</p>
          </div>

          {currentStepIndex === -1 ? (
             <motion.div 
               initial={{ opacity: 0, y: 20 }}
               animate={{ opacity: 1, y: 0 }}
               className="bg-[#111625] border border-white/10 p-8 rounded-2xl flex flex-col items-center justify-center text-center space-y-6 h-64"
             >
                <div className="w-16 h-16 rounded-full bg-atc-cyan/20 flex items-center justify-center">
                   <HardDrive className="w-8 h-8 text-atc-cyan" />
                </div>
                <div>
                   <h3 className="text-xl font-bold text-white mb-2">Ready to Install</h3>
                   <p className="text-sm text-slate-400 max-w-xs mx-auto">This will deploy the core services and prepare your environment for A-TownChain development.</p>
                </div>
                <button
                   onClick={startInstallation}
                   className="px-6 py-3 bg-atc-cyan hover:bg-atc-cyan/80 text-black font-bold rounded-lg shadow-[0_0_15px_rgba(45,212,191,0.3)] transition-all flex items-center gap-2"
                >
                   Start Setup <ArrowRight className="w-4 h-4" />
                </button>
             </motion.div>
          ) : (
            <div className="space-y-4">
              {INSTALLATION_STEPS.map((step, index) => {
                const isActive = index === currentStepIndex;
                const isCompleted = completedSteps.includes(step.id);
                const Icon = step.icon;

                return (
                  <div 
                    key={step.id} 
                    className={`relative p-4 rounded-xl border transition-all duration-300 ${
                      isActive 
                        ? "bg-atc-cyan/10 border-atc-cyan/50 shadow-[0_0_20px_rgba(45,212,191,0.1)]" 
                        : isCompleted 
                          ? "bg-[#111625] border-emerald-500/30" 
                          : "bg-[#111625]/50 border-white/5 opacity-50"
                    }`}
                  >
                    <div className="flex items-start gap-4">
                       <div className="mt-1">
                          {isCompleted ? (
                             <CheckCircle2 className="w-6 h-6 text-emerald-400" />
                          ) : isActive ? (
                             <Loader2 className="w-6 h-6 text-atc-cyan animate-spin" />
                          ) : (
                             <Circle className="w-6 h-6 text-slate-600" />
                          )}
                       </div>
                       <div className="flex-1">
                          <h3 className={`font-bold text-lg flex items-center gap-2 ${isActive ? 'text-atc-cyan' : isCompleted ? 'text-white' : 'text-slate-400'}`}>
                             <Icon className="w-4 h-4" />
                             {step.title}
                          </h3>
                          <p className="text-sm text-slate-400 mt-1">{step.description}</p>
                          
                          {isActive && (
                            <div className="mt-4 h-1 w-full bg-black/50 rounded-full overflow-hidden">
                               <div 
                                 className="h-full bg-gradient-to-r from-atc-cyan to-indigo-500 transition-all duration-100"
                                 style={{ width: `${progress}%` }}
                               />
                            </div>
                          )}
                       </div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>

        {/* Right Column: Terminal Logs */}
        <div className="w-full md:w-1/2 flex flex-col h-[500px] md:h-auto">
          <div className="bg-[#0b0e14] border border-white/10 rounded-xl overflow-hidden flex-1 flex flex-col shadow-2xl">
            <div className="bg-[#151a28] px-4 py-2 border-b border-white/10 flex items-center gap-2">
              <Terminal className="w-4 h-4 text-slate-400" />
              <span className="text-xs font-mono text-slate-400">setup-activity.log</span>
            </div>
            <div className="flex-1 p-4 font-mono text-xs overflow-y-auto space-y-2 bg-[#050608]">
               {currentStepIndex === -1 && (
                  <div className="text-slate-500 italic">Waiting for installation command...</div>
               )}
               {currentLogs.map((log, i) => (
                  <motion.div 
                    initial={{ opacity: 0, x: -10 }} 
                    animate={{ opacity: 1, x: 0 }} 
                    key={i}
                    className="text-slate-300"
                  >
                     <span className="text-atc-cyan mr-2">[{new Date().toLocaleTimeString()}]</span>
                     {log}
                  </motion.div>
               ))}
               {isFinished && (
                  <motion.div 
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    className="mt-6 p-4 bg-emerald-500/10 border border-emerald-500/30 rounded text-emerald-400 flex items-center gap-3"
                  >
                     <ShieldCheck className="w-6 h-6" />
                     <div>
                        <strong className="block text-sm">Installation Complete</strong>
                        <span className="text-emerald-500/70">A-TownChain Ecosystem is fully operational.</span>
                     </div>
                  </motion.div>
               )}
            </div>
          </div>

          {isFinished && (
             <motion.div 
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                className="mt-6 flex justify-end"
             >
                <button 
                  onClick={() => {/* could trigger some reset or launch action */}}
                  className="px-6 py-3 bg-[#111625] border border-white/10 hover:border-atc-cyan/50 text-white rounded-lg transition-colors flex items-center gap-2 font-bold"
                >
                   Close Installer
                </button>
             </motion.div>
          )}
        </div>

      </div>
    </div>
  );
}
