import React, { useState, useEffect } from 'react';
import { Network, Zap, BrainCircuit, Activity, Cpu, Play, CheckCircle2, StopCircle } from 'lucide-react';

interface TrainingTask {
  id: string;
  progress: number;
  loss: number | null;
  status: 'idle' | 'training' | 'completed' | 'failed';
}

export function PoAITrainingEngineView() {
  const [tasks, setTasks] = useState<TrainingTask[]>([]);
  const [isEngineActive, setIsEngineActive] = useState(false);
  const [mainLoopRate, setMainLoopRate] = useState(15.0); // Simulated TPS
  const [totalModelsTrained, setTotalModelsTrained] = useState(0);
  const [bestLoss, setBestLoss] = useState(1.0);

  // Background main loop (PoS/PoH/P2P) simulated
  useEffect(() => {
    const loop = setInterval(() => {
      // Fluctuate TPS a bit
      setMainLoopRate(prev => Math.max(10, Math.min(20, prev + (Math.random() * 2 - 1))));
    }, 1000);
    return () => clearInterval(loop);
  }, []);

  // Background PoAI training interval
  useEffect(() => {
    let trainingInterval: NodeJS.Timeout;
    
    if (isEngineActive) {
      trainingInterval = setInterval(() => {
        setTasks(prev => {
          let updated = [...prev];
          
          updated = updated.map(task => {
            if (task.status === 'training') {
              const newProgress = task.progress + Math.random() * 10 + 5;
              if (newProgress >= 100) {
                const finalLoss = Math.random() * 0.2; // simulate rust loss
                if (finalLoss < bestLoss) {
                  setBestLoss(finalLoss);
                }
                setTotalModelsTrained(t => t + 1);
                return { ...task, progress: 100, status: 'completed', loss: finalLoss };
              }
              return { ...task, progress: newProgress };
            }
            return task;
          });

          // Add a new task if we have less than 3 active
          const activeTasks = updated.filter(t => t.status === 'training').length;
          if (activeTasks < 3 && Math.random() > 0.4) {
             updated.push({
               id: `AI-Job-${Math.random().toString(36).substring(2, 8).toUpperCase()}`,
               progress: 0,
               loss: null,
               status: 'training'
             });
          }

          // Cleanup old completed Tasks
          if (updated.length > 5) {
             updated = updated.filter(t => t.status === 'training' || t.progress >= 0).slice(-5);
          }

          return updated;
        });
      }, 500);
    }

    return () => {
      if (trainingInterval) clearInterval(trainingInterval);
    };
  }, [isEngineActive, bestLoss]);

  return (
    <div className="flex flex-col h-full bg-[#050B14] border border-white/5 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center justify-between px-6 py-4 border-b border-white/5 bg-slate-900/50">
        <div className="flex items-center gap-3">
            <BrainCircuit className="w-5 h-5 text-purple-400" />
            <div>
            <h2 className="text-sm font-bold text-white tracking-widest font-mono uppercase">PoAI Asynchronous Engine</h2>
            <p className="text-xs text-slate-400 mt-0.5">tokio::spawn Non-Blocking Training Module</p>
            </div>
        </div>
        
        <button 
            onClick={() => setIsEngineActive(!isEngineActive)}
            className={`px-4 py-2 font-mono text-[10px] uppercase tracking-wider rounded border transition-all flex items-center gap-2 ${
                isEngineActive 
                ? 'bg-rose-500/10 text-rose-400 border-rose-500/30 hover:bg-rose-500/20'
                : 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30 hover:bg-emerald-500/20'
            }`}
        >
            {isEngineActive ? <><StopCircle className="w-3.5 h-3.5" /> Stop Engine</> : <><Play className="w-3.5 h-3.5" /> Boot Engine</>}
        </button>
      </div>
      
      <div className="flex-1 p-6 overflow-y-auto custom-scrollbar">
         
         <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
            <div className="bg-slate-900/40 p-4 border border-white/5 rounded-xl relative overflow-hidden group">
               <div className="absolute inset-0 bg-blue-500/5 opacity-0 group-hover:opacity-100 transition-opacity" />
               <div className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-1">Consensus Loop (TPS)</div>
               <div className="text-2xl font-mono text-blue-400 flex items-center gap-2">
                 <Activity className="w-5 h-5" /> {mainLoopRate.toFixed(1)}
               </div>
               <div className="text-[10px] text-slate-400 mt-2 font-mono">Running asynchronously</div>
            </div>

            <div className="bg-slate-900/40 p-4 border border-white/5 rounded-xl relative overflow-hidden group">
               <div className="absolute inset-0 bg-purple-500/5 opacity-0 group-hover:opacity-100 transition-opacity" />
               <div className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-1">Total Models Trained</div>
               <div className="text-2xl font-mono text-purple-400 flex items-center gap-2">
                 <Cpu className="w-5 h-5" /> {totalModelsTrained}
               </div>
               <div className="text-[10px] text-slate-400 mt-2 font-mono">Completed local PoAI proofs</div>
            </div>

            <div className="bg-slate-900/40 p-4 border border-white/5 rounded-xl relative overflow-hidden group">
               <div className="absolute inset-0 bg-emerald-500/5 opacity-0 group-hover:opacity-100 transition-opacity" />
               <div className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-1">Best Accuracy (Inverse Loss)</div>
               <div className="text-2xl font-mono text-emerald-400 flex items-center gap-2">
                 <Zap className="w-5 h-5" /> {(1.0 - bestLoss).toFixed(4)}
               </div>
               <div className="text-[10px] text-slate-400 mt-2 font-mono">{bestLoss < 0.2 ? 'Network Standard Met' : 'Training Required'}</div>
            </div>
         </div>

         <h3 className="font-mono text-sm text-white mb-4 uppercase tracking-widest flex items-center gap-2">
            <Network className="w-4 h-4 text-slate-400" /> Active tokio::spawn Tasks
         </h3>
         
         <div className="space-y-3">
             {tasks.length === 0 ? (
                 <div className="h-32 flex items-center justify-center border border-dashed border-white/10 rounded-xl text-slate-500 font-mono text-[10px] uppercase">
                     Engine is Idle. Boot to dispatch thread pool.
                 </div>
             ) : (
                 tasks.map(task => (
                     <div key={task.id} className="bg-slate-900/60 p-4 rounded-xl border border-white/5">
                        <div className="flex items-center justify-between mb-2">
                           <span className="font-mono text-xs text-slate-300 font-bold">{task.id}</span>
                           <span className={`text-[10px] font-mono uppercase px-2 py-0.5 rounded border ${
                               task.status === 'training' ? 'bg-purple-500/10 text-purple-400 border-purple-500/20' :
                               'bg-emerald-500/10 text-emerald-400 border-emerald-500/20'
                           }`}>
                               {task.status}
                           </span>
                        </div>
                        <div className="w-full h-1.5 bg-slate-800 rounded-full overflow-hidden mb-2">
                            <div 
                               className={`h-full rounded-full transition-all duration-300 ${task.status === 'completed' ? 'bg-emerald-500' : 'bg-purple-500'}`}
                               style={{ width: task.progress + '%' }}
                            />
                        </div>
                        <div className="flex justify-between font-mono text-[10px]">
                            <span className="text-slate-500">Progress: {task.progress.toFixed(1)}%</span>
                            <span className="text-slate-400">
                                {task.loss !== null ? `Loss: ${task.loss.toFixed(4)}` : 'Computing...'}
                            </span>
                        </div>
                     </div>
                 ))
             )}
         </div>
         
      </div>
    </div>
  );
}
