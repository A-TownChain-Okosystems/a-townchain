import React, { useState, useEffect, useRef } from 'react';
import { Rocket, Power, Terminal, Activity, ShieldCheck, Database, Network, CheckCircle2, AlertTriangle } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export function MainnetLaunchView() {
  const [launchState, setLaunchState] = useState<'idle' | 'validating' | 'booting' | 'broadcasting' | 'live'>('idle');
  const [logs, setLogs] = useState<string[]>([]);
  const [progress, setProgress] = useState(0);
  const terminalEndRef = useRef<HTMLDivElement>(null);

  const addLog = (msg: string) => {
    setLogs(prev => [...prev, `[ATC-SYS] ${msg}`]);
  };

  useEffect(() => {
    if (terminalEndRef.current) {
      terminalEndRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [logs]);

  const igniteMainnet = () => {
    if (launchState !== 'idle') return;
    
    setLogs([]);
    setLaunchState('validating');
    addLog("--- Initialisiere A-TownChain Genesis Sequence ---");
    setProgress(10);

    setTimeout(() => {
      addLog("EXEC: ./bin/atc-core-node --validate-genesis");
      setTimeout(() => {
          addLog("Hashing genesis.json...");
          setTimeout(() => {
              addLog("Genesis Validierung erfolgreich. Hash: 0xGENESIS_START_ATC_001");
              setLaunchState('booting');
              setProgress(40);

              setTimeout(() => {
                addLog("Booten des Storage-States...");
                addLog("EXEC: ./bin/atc-core-node --init-db --genesis-file ./genesis.json");
                
                setTimeout(() => {
                    addLog("RocksDB initialisiert. Allocate 10.2GB space.");
                    addLog("Genesis-Block in RocksDB geschrieben.");
                    setLaunchState('broadcasting');
                    setProgress(70);

                    setTimeout(() => {
                      addLog("Aktiviere P2P-Broadcast und Gossipsub...");
                      addLog("EXEC: ./bin/atc-core-node --mode validator --peer-discovery-enabled true");
                      
                      setTimeout(() => {
                          addLog("Peer-Sync-Protokoll-Check: 5 Seed-Nodes in 124ms gefunden.");
                          addLog("Validator-Rotation-Trigger: Leader-Wahl-Zyklus aktiv.");
                          addLog("Epoch 1 / Slot 1 begonnen.");
                          setLaunchState('live');
                          setProgress(100);
                          addLog("--- Mainnet live. A-TownChain pulsiert. ---");
                      }, 2500);
                    }, 2000);
                }, 2000);
              }, 1000);
          }, 1500);
      }, 1000);
    }, 1000);
  };

  const getStepStatus = (stepIndex: number) => {
      const stateMap = { idle: 0, validating: 1, booting: 2, broadcasting: 3, live: 4 };
      const currentStateValue = stateMap[launchState];
      if (currentStateValue > stepIndex) return 'completed';
      if (currentStateValue === stepIndex) return 'active';
      return 'pending';
  };

  return (
    <div className="flex flex-col h-full bg-[#03060c] border border-red-500/10 rounded-xl overflow-hidden font-sans relative">
      {launchState === 'live' && (
         <div className="absolute inset-0 pointer-events-none z-0">
             <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-emerald-500/10 blur-[100px] rounded-full mix-blend-screen opacity-50 animate-pulse" />
         </div>
      )}

      <div className="flex items-center gap-3 px-6 py-4 border-b border-white/5 bg-black/50 z-10 relative">
        <Rocket className={`w-5 h-5 ${launchState === 'live' ? 'text-emerald-400' : 'text-red-500'}`} />
        <div>
          <h2 className="text-sm font-bold text-white tracking-widest font-mono uppercase">Mainnet Launch Sequence</h2>
          <p className="text-xs text-slate-400 mt-0.5">Point Zero: A-TownChain Mainnet Bootstrapper</p>
        </div>
      </div>
      
      <div className="flex-1 flex overflow-hidden z-10 relative">
        {/* Sequence Tracker */}
        <div className="w-1/3 p-8 border-r border-white/5 bg-black/30 flex flex-col items-center justify-center">
            
            <motion.div 
               animate={{ 
                   scale: launchState === 'idle' ? 1 : 1.05,
                   opacity: launchState === 'idle' ? 1 : 0.8
               }}
               className={`mb-12 relative ${launchState === 'idle' ? 'cursor-pointer' : 'cursor-not-allowed pointer-events-none'}`}
               onClick={igniteMainnet}
            >
                <div className={`w-40 h-40 rounded-full flex items-center justify-center border-4 transition-all duration-1000 ${
                    launchState === 'idle' ? 'border-red-500 bg-red-500/20 hover:bg-red-500/30' : 
                    launchState === 'live' ? 'border-emerald-500 bg-emerald-500/20 shadow-[0_0_50px_rgba(16,185,129,0.3)]' : 
                    'border-amber-500 bg-amber-500/20'
                }`}>
                    {launchState === 'idle' && (
                         <div className="flex flex-col items-center gap-2">
                             <Power className="w-10 h-10 text-red-500" />
                             <span className="font-mono font-bold text-red-500 tracking-widest uppercase">Ignite</span>
                         </div>
                    )}
                    {(launchState === 'validating' || launchState === 'booting' || launchState === 'broadcasting') && (
                         <div className="flex flex-col items-center gap-2">
                             <Activity className="w-10 h-10 text-amber-500 animate-pulse" />
                             <span className="font-mono font-bold text-amber-500 tracking-widest uppercase">Sequencing</span>
                         </div>
                    )}
                    {launchState === 'live' && (
                         <div className="flex flex-col items-center gap-2">
                             <CheckCircle2 className="w-10 h-10 text-emerald-500" />
                             <span className="font-mono font-bold text-emerald-500 tracking-widest uppercase">Live</span>
                         </div>
                    )}
                </div>
                {launchState === 'idle' && (
                    <div className="absolute -bottom-8 left-1/2 -translate-x-1/2 text-center w-full">
                        <span className="text-[10px] text-red-500/70 font-mono uppercase tracking-widest">Awaiting Command</span>
                    </div>
                )}
            </motion.div>

            <div className="w-full max-w-xs space-y-6">
                <LaunchStep 
                   icon={<ShieldCheck className="w-5 h-5" />}
                   title="1. Validate Genesis"
                   desc="Hash-Check der genesis.json"
                   status={getStepStatus(1)}
                />
                <LaunchStep 
                   icon={<Database className="w-5 h-5" />}
                   title="2. Boot Storage-State"
                   desc="RocksDB Initialisierung"
                   status={getStepStatus(2)}
                />
                <LaunchStep 
                   icon={<Network className="w-5 h-5" />}
                   title="3. P2P Broadcast"
                   desc="libp2p & Validator-Rotation"
                   status={getStepStatus(3)}
                />
            </div>
            
            <div className="w-full max-w-xs mt-10">
               <div className="flex justify-between text-[10px] font-mono text-slate-500 mb-2 uppercase tracking-widest">
                   <span>Launch Progress</span>
                   <span>{progress}%</span>
               </div>
               <div className="w-full h-1.5 bg-slate-900 rounded-full overflow-hidden border border-white/5">
                   <div 
                      className={`h-full transition-all duration-1000 ${launchState === 'live' ? 'bg-emerald-500' : 'bg-red-500'}`}
                      style={{ width: `${progress}%` }}
                   />
               </div>
            </div>
        </div>

        {/* Terminal output */}
        <div className="w-2/3 flex flex-col bg-[#010204]">
            <div className="p-4 border-b border-white/5 flex items-center justify-between bg-black/30">
               <div className="flex items-center gap-2 text-xs font-mono text-slate-400 uppercase tracking-widest">
                  <Terminal className="w-4 h-4" /> launch_mainnet.sh
               </div>
               {launchState === 'live' && (
                   <div className="flex items-center gap-2 text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-1 rounded border border-emerald-500/20">
                       <span className="relative flex h-2 w-2">
                          <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                          <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                        </span>
                       Mainnet Operational
                   </div>
               )}
            </div>
            <div className="flex-1 p-6 overflow-y-auto custom-scrollbar font-mono text-xs leading-relaxed">
                {logs.length === 0 ? (
                    <div className="h-full flex flex-col items-center justify-center text-slate-600 opacity-50 space-y-4">
                        <AlertTriangle className="w-12 h-12" />
                        <span className="uppercase tracking-widest">System is on Standby</span>
                    </div>
                ) : (
                    <div className="space-y-1.5">
                        <AnimatePresence>
                            {logs.map((log, i) => {
                                const isSuccess = log.includes("erfolgreich") || log.includes("live") || log.includes("aktiv");
                                const isExec = log.includes("EXEC:");
                                const isHeader = log.includes("---");
                                
                                return (
                                    <motion.div 
                                        key={i}
                                        initial={{ opacity: 0, x: -10 }}
                                        animate={{ opacity: 1, x: 0 }}
                                        className={`
                                            ${isSuccess ? 'text-emerald-400' : 
                                              isExec ? 'text-blue-400' : 
                                              isHeader ? 'text-amber-500 font-bold' : 
                                              'text-slate-300'}
                                        `}
                                    >
                                        {log}
                                    </motion.div>
                                );
                            })}
                        </AnimatePresence>
                        <div ref={terminalEndRef} />
                    </div>
                )}
            </div>
        </div>
      </div>
    </div>
  );
}

function LaunchStep({ icon, title, desc, status }: { icon: React.ReactNode, title: string, desc: string, status: 'pending' | 'active' | 'completed' }) {
    return (
        <div className={`flex items-start gap-4 transition-all duration-500 ${
            status === 'pending' ? 'opacity-30' : 
            status === 'active' ? 'opacity-100' : 
            'opacity-60'
        }`}>
            <div className={`w-10 h-10 rounded-full flex items-center justify-center shrink-0 border transition-colors duration-500 ${
                status === 'pending' ? 'bg-slate-900 border-slate-800 text-slate-500' :
                status === 'active' ? 'bg-amber-500/20 border-amber-500 text-amber-500 shadow-[0_0_15px_rgba(245,158,11,0.2)]' :
                'bg-emerald-500/20 border-emerald-500 text-emerald-500'
            }`}>
               {status === 'completed' ? <CheckCircle2 className="w-5 h-5" /> : icon}
            </div>
            <div>
               <h3 className={`font-mono text-sm uppercase tracking-wider mb-0.5 ${
                   status === 'active' ? 'text-amber-400' : 
                   status === 'completed' ? 'text-emerald-400' : 
                   'text-slate-400'
               }`}>{title}</h3>
               <p className="font-mono text-[10px] text-slate-500">{desc}</p>
            </div>
        </div>
    );
}
