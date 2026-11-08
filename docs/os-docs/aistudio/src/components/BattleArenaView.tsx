import React, { useState } from 'react';
import { Sword, Shield, Zap, Skull, CloudLightning, Activity } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export function BattleArenaView() {
  const [battleLogs, setBattleLogs] = useState<{ id: number, text: string, type: 'hit'|'crit'|'miss'|'sys' }[]>([
    { id: 1, text: "Battle Engine initalisiert. Warte auf Parameter...", type: 'sys' }
  ]);
  const [isBattling, setIsBattling] = useState(false);
  const [targetId1, setTargetId1] = useState('SHV-001');
  const [targetId2, setTargetId2] = useState('SHV-089');

  const startBattle = () => {
    if (!targetId1 || !targetId2) return;
    setIsBattling(true);
    setBattleLogs([{ id: Date.now(), text: `Starte Match: ${targetId1} VS ${targetId2}`, type: 'sys' }]);
    
    let step = 0;
    const interval = setInterval(() => {
      step++;
      if (step === 1) {
        setBattleLogs(prev => [...prev, { id: Date.now(), text: `${targetId1} nutzt Void Gaze (-42 HP)`, type: 'hit' }]);
      } else if (step === 2) {
        setBattleLogs(prev => [...prev, { id: Date.now(), text: `${targetId2} greift an... verfehlt!`, type: 'miss' }]);
      } else if (step === 3) {
        setBattleLogs(prev => [...prev, { id: Date.now(), text: `${targetId1} führt Phantom Strike aus. CRITICAL HIT! (-120 HP)`, type: 'crit' }]);
      } else {
        setBattleLogs(prev => [...prev, { id: Date.now(), text: `${targetId2} wurde vernichtet. Sieger: ${targetId1}`, type: 'sys' }]);
        clearInterval(interval);
        setIsBattling(false);
      }
    }, 1500);
  };

  return (
    <div className="flex flex-col h-full bg-[#060a16] text-slate-200">
      {/* Header */}
      <div className="flex items-center gap-3 px-6 py-4 bg-[#090b14] border-b border-atc-border/50 shadow-sm">
         <div className="p-2 rounded-lg bg-orange-500/10 border border-orange-500/30">
            <Sword className="w-6 h-6 text-orange-400" />
         </div>
         <div>
            <h2 className="text-xl font-bold text-white tracking-tight">Battle Arena — Smart Contract Execution</h2>
            <p className="text-xs text-slate-400 font-mono">PVP Matchmaking • Blockchain Verification</p>
         </div>
      </div>

      <div className="flex-1 overflow-y-auto p-6 custom-scrollbar">
         <div className="max-w-4xl mx-auto flex flex-col gap-8">
            
            {/* Arena Display */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 items-center bg-[#090b14] border border-atc-border/50 p-8 rounded-2xl relative overflow-hidden">
               <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03]" />
               
               {/* Fighter 1 */}
               <div className="flex flex-col items-center gap-4 z-10">
                  <div className="text-xs font-bold tracking-widest text-slate-500 mb-2">ANGREIFER</div>
                  <input 
                    value={targetId1} 
                    onChange={e => setTargetId1(e.target.value)}
                    className="bg-black/50 border border-white/10 rounded px-3 py-1.5 text-center text-sm font-mono text-orange-400 focus:outline-none focus:border-orange-500/50 w-32"
                  />
                  <div className="w-32 h-32 rounded-2xl bg-orange-500/10 border border-orange-500/30 flex items-center justify-center text-5xl relative group shadow-[0_0_30px_rgba(249,115,22,0.15)]">
                     {targetId1 === 'SHV-001' ? '👻' : '❓'}
                     {isBattling && <motion.div animate={{ opacity: [0, 1, 0] }} transition={{ repeat: Infinity, duration: 1 }} className="absolute inset-0 border border-orange-400 rounded-2xl" />}
                  </div>
                  <div className="w-full max-w-[200px] mt-2">
                     <div className="flex justify-between text-[10px] text-slate-400 font-mono mb-1"><span>HP</span><span>{isBattling ? '100 / 100' : '---'}</span></div>
                     <div className="h-2 bg-black/50 rounded-full border border-white/5 overflow-hidden">
                        <div className="h-full bg-emerald-500 w-full" />
                     </div>
                  </div>
               </div>

               {/* VS Central */}
               <div className="flex flex-col items-center justify-center gap-6 z-10 py-10 md:py-0">
                  <div className="text-4xl font-black italic text-transparent bg-clip-text bg-gradient-to-br from-orange-400 to-pink-500 drop-shadow-md">
                    VS
                  </div>
                  <button 
                    onClick={startBattle}
                    disabled={isBattling || !targetId1 || !targetId2}
                    className="px-8 py-3 bg-red-500/20 text-red-400 font-bold tracking-widest border border-red-500/30 rounded-xl hover:bg-red-500/30 transition-all shadow-[0_0_20px_rgba(239,68,68,0.2)] disabled:opacity-50 disabled:cursor-not-allowed hover:scale-105"
                  >
                    KAMPF STARTEN
                  </button>
               </div>

               {/* Fighter 2 */}
               <div className="flex flex-col items-center gap-4 z-10">
                  <div className="text-xs font-bold tracking-widest text-slate-500 mb-2">VERTEIDIGER</div>
                  <input 
                    value={targetId2} 
                    onChange={e => setTargetId2(e.target.value)}
                    className="bg-black/50 border border-white/10 rounded px-3 py-1.5 text-center text-sm font-mono text-cyan-400 focus:outline-none focus:border-cyan-500/50 w-32"
                  />
                  <div className="w-32 h-32 rounded-2xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-5xl relative group shadow-[0_0_30px_rgba(6,182,212,0.15)]">
                     {targetId2 === 'SHV-089' ? '🐉' : '❓'}
                     {isBattling && <motion.div animate={{ opacity: [0, 1, 0], scale: [1, 1.05, 1] }} transition={{ repeat: Infinity, duration: 1.5 }} className="absolute inset-0 border border-red-500/50 rounded-2xl" />}
                  </div>
                  <div className="w-full max-w-[200px] mt-2">
                     <div className="flex justify-between text-[10px] text-slate-400 font-mono mb-1"><span>HP</span><span>{isBattling ? '0 / 100' : '---'}</span></div>
                     <div className="h-2 bg-black/50 rounded-full border border-white/5 overflow-hidden transition-all">
                        <div className={`h-full ${isBattling ? 'w-0' : 'w-full'} bg-emerald-500 transition-all duration-1000 delay-[3000ms]`} />
                     </div>
                  </div>
               </div>
            </div>

            {/* Battle Log */}
            <div className="bg-[#090b14] border border-atc-border/50 rounded-2xl flex flex-col h-[250px] overflow-hidden">
               <div className="px-4 py-3 border-b border-atc-border/50 bg-[#060a16] flex items-center gap-2">
                  <Activity className="w-4 h-4 text-slate-400" />
                  <h3 className="text-sm font-bold text-slate-300">Live KAMPF-LOG</h3>
               </div>
               <div className="flex-1 p-4 overflow-y-auto custom-scrollbar flex flex-col gap-2 font-mono text-sm">
                  <AnimatePresence>
                     {battleLogs.map((log) => (
                        <motion.div 
                          key={log.id}
                          initial={{ opacity: 0, x: -10 }}
                          animate={{ opacity: 1, x: 0 }}
                          className={`
                            px-3 py-2 rounded border 
                            ${log.type === 'hit' ? 'bg-orange-500/10 border-orange-500/20 text-orange-200' : ''}
                            ${log.type === 'miss' ? 'bg-slate-500/10 border-slate-500/20 text-slate-300' : ''}
                            ${log.type === 'crit' ? 'bg-red-500/20 border-red-500/30 text-red-300 font-bold shadow-[0_0_10px_rgba(239,68,68,0.2)]' : ''}
                            ${log.type === 'sys' ? 'bg-blue-500/10 border-blue-500/20 text-blue-300' : ''}
                          `}
                        >
                           <span className="text-slate-500 mr-2 text-xs">[{new Date(log.id).toISOString().split('T')[1].slice(0,8)}]</span>
                           {log.text}
                        </motion.div>
                     ))}
                  </AnimatePresence>
               </div>
            </div>

         </div>
      </div>
    </div>
  );
}
