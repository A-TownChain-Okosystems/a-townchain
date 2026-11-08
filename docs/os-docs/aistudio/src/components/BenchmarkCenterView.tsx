import React, { useState, useEffect } from 'react';
import { Cpu, Server, Activity, Database, Zap, HardDrive, CheckCircle2, Flame, Navigation, Hash } from 'lucide-react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip as RechartsTooltip, ResponsiveContainer, Legend } from 'recharts';

export function BenchmarkCenterView() {
  const [activeTab, setActiveTab] = useState<'benchmark' | 'stress'>('stress');
  const [isTesting, setIsTesting] = useState(false);
  const [progress, setProgress] = useState(0);
  const [scores, setScores] = useState<any>(null);

  const startTest = () => {
    setIsTesting(true);
    setProgress(0);
    setScores(null);
    
    let current = 0;
    const interval = setInterval(() => {
      current += 2.5;
      setProgress(current);
      
      // Periodically trigger a system alert toast during stress test
      if (current % 25 === 0 && current < 100) {
        const alerts = [
          "CPU Core 4 Temperature Spike (+15°C)",
          "VRAM Allocation Reaching Limits (85%)",
          "Network Node Stress Detected (Latency: 45ms)",
          "Disk I/O Queue Depth Increasing",
          "Thermal Throttling Engaged on GPU Unit",
        ];
        const randomAlert = alerts[Math.floor(Math.random() * alerts.length)];
        window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: `Stress Test: ${randomAlert}` }));
      }

      if (current >= 100) {
        clearInterval(interval);
        setIsTesting(false);
        setScores({
          cpu: 18452,
          memory: 4520,
          disk: 980,
          hashRate: 450,
          overall: 95
        });
        window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Benchmark Completed Successfully' }));
      }
    }, 100);
  };

  return (
    <div className="flex flex-col h-full bg-[#090b14] text-slate-200">
      <div className="flex border-b border-white/5 px-6 pt-4 gap-6 bg-black/40 shrink-0">
        <button
          onClick={() => setActiveTab('benchmark')}
          className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === 'benchmark' ? 'text-white' : 'text-slate-500 hover:text-slate-300'}`}
        >
          <div className="flex items-center gap-2"><Zap className="w-4 h-4" /> Benchmark</div>
          {activeTab === 'benchmark' && <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-red-500" />}
        </button>
        <button
          onClick={() => setActiveTab('stress')}
          className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === 'stress' ? 'text-white' : 'text-slate-500 hover:text-slate-300'}`}
        >
          <div className="flex items-center gap-2"><Flame className="w-4 h-4" /> Hardware Stress Test</div>
          {activeTab === 'stress' && <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-orange-500" />}
        </button>
      </div>

      <div className="p-8 flex-1 overflow-y-auto">
        {activeTab === 'benchmark' ? (
          <div className="max-w-4xl mx-auto flex flex-col gap-8 flex-1">
            <div className="flex items-center gap-4 border-b border-white/10 pb-6 shrink-0">
              <div className="p-3 bg-red-500/10 border border-red-500/30 rounded-xl text-red-400">
                <Zap className="w-6 h-6" />
              </div>
              <div>
                <h1 className="text-2xl font-bold text-white">ATC Benchmark Test Center</h1>
                <p className="text-slate-400 text-sm">Evaluate virtual hardware performance metrics under A-Town OS</p>
              </div>
              <div className="ml-auto">
                <button 
                  onClick={startTest}
                  disabled={isTesting}
                  className={`px-6 py-2.5 rounded-lg font-bold shadow-lg transition-all ${isTesting ? 'bg-slate-700 text-slate-400 cursor-not-allowed' : 'bg-red-500 hover:bg-red-600 text-white shadow-red-500/20'}`}
                >
                  {isTesting ? 'Testing...' : 'RUN BENCHMARK'}
                </button>
              </div>
            </div>

            {isTesting && (
              <div className="bg-slate-800/50 border border-white/10 p-6 rounded-2xl flex flex-col gap-4">
                <div className="flex justify-between text-sm font-mono text-emerald-400">
                  <span>Running Synthetic Stress Tests...</span>
                  <span>{Math.round(progress)}%</span>
                </div>
                <div className="h-2 bg-black rounded-full overflow-hidden">
                   <div className="h-full bg-emerald-500 transition-all duration-200" style={{ width: `${progress}%` }} />
                </div>
              </div>
            )}

            {scores && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6 animate-in slide-in-from-bottom-4 fade-in duration-500">
                <div className="bg-white/5 border border-white/10 rounded-2xl p-6 flex flex-col items-center justify-center gap-2 relative overflow-hidden group">
                   <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-emerald-400 to-cyan-400" />
                   <div className="text-6xl font-black text-transparent bg-clip-text bg-gradient-to-br from-emerald-400 to-cyan-400 drop-shadow-lg">
                     {scores.overall}
                   </div>
                   <div className="text-sm tracking-widest text-slate-400 uppercase">ATC Certified Score</div>
                   <CheckCircle2 className="w-12 h-12 text-emerald-400/20 absolute -right-2 -bottom-2 group-hover:scale-110 transition-transform" />
                </div>

                <div className="grid grid-cols-2 gap-4">
                   <div className="bg-white/5 border border-white/10 rounded-xl p-4 flex flex-col gap-2">
                     <div className="flex items-center gap-2 text-slate-400 text-xs font-mono uppercase"><Cpu className="w-4 h-4 text-indigo-400" /> CPU Threads</div>
                     <div className="text-2xl font-bold text-white tracking-widest">{scores.cpu}</div>
                   </div>
                   <div className="bg-white/5 border border-white/10 rounded-xl p-4 flex flex-col gap-2">
                     <div className="flex items-center gap-2 text-slate-400 text-xs font-mono uppercase"><Database className="w-4 h-4 text-atc-cyan" /> Memory I/O</div>
                     <div className="text-2xl font-bold text-white tracking-widest">{scores.memory}</div>
                   </div>
                   <div className="bg-white/5 border border-white/10 rounded-xl p-4 flex flex-col gap-2">
                     <div className="flex items-center gap-2 text-slate-400 text-xs font-mono uppercase"><HardDrive className="w-4 h-4 text-emerald-400" /> Disk Speed</div>
                     <div className="text-2xl font-bold text-white tracking-widest">{scores.disk}</div>
                   </div>
                   <div className="bg-white/5 border border-white/10 rounded-xl p-4 flex flex-col gap-2">
                     <div className="flex items-center gap-2 text-slate-400 text-xs font-mono uppercase"><Activity className="w-4 h-4 text-amber-400" /> Hash Rate (MH/s)</div>
                     <div className="text-2xl font-bold text-white tracking-widest">{scores.hashRate}</div>
                   </div>
                </div>
              </div>
            )}
          </div>
        ) : (
          <HardwareStressTestView />
        )}
      </div>
    </div>
  );
}

function HardwareStressTestView() {
  const [loadOptions, setLoadOptions] = useState({ cpu: false, gpu: false, ram: false });
  const [metricsData, setMetricsData] = useState<{ time: string; thermal: number; power: number }[]>([
    { time: '0s', thermal: 35, power: 15 }
  ]);
  const [timer, setTimer] = useState(0);

  useEffect(() => {
    const isAnyRunning = loadOptions.cpu || loadOptions.gpu || loadOptions.ram;
    if (!isAnyRunning) return;

    const intervalId = setInterval(() => {
      setTimer(prev => prev + 1);
      setMetricsData(prev => {
        const newTime = `${prev.length}s`;
        const lastData = prev[prev.length - 1];
        
        let thermalDelta = 0;
        let powerDelta = 0;
        
        if (loadOptions.cpu) { thermalDelta += 2; powerDelta += 30; }
        if (loadOptions.gpu) { thermalDelta += 3; powerDelta += 60; }
        if (loadOptions.ram) { thermalDelta += 0.5; powerDelta += 10; }
        
        // Add random fluctuation and gradual decline/rise
        let newThermal = lastData.thermal + (thermalDelta > 0 ? (thermalDelta + (Math.random() * 2 - 1)) : -1);
        let newPower = 15 + powerDelta + (Math.random() * 5 - 2.5); // base power 15W
        
        // Clamp values
        newThermal = Math.min(95, Math.max(35, newThermal));
        newPower = Math.min(500, Math.max(15, newPower));
        
        const newData = [...prev, { time: newTime, thermal: newThermal, power: newPower }];
        if (newData.length > 20) newData.shift();
        return newData;
      });
    }, 1000);

    return () => clearInterval(intervalId);
  }, [loadOptions]);

  // Gradual cool down when all off
  useEffect(() => {
    const isAnyRunning = loadOptions.cpu || loadOptions.gpu || loadOptions.ram;
    if (isAnyRunning) return;
    
    const intervalId = setInterval(() => {
      setMetricsData(prev => {
        const lastData = prev[prev.length - 1];
        if (lastData.thermal <= 35 && lastData.power <= 15) return prev; // Fully cooled
        
        const newTime = `${prev.length + timer}s`;
        let newThermal = Math.max(35, lastData.thermal - 1.5 - Math.random());
        let newPower = Math.max(15, lastData.power - 10 - Math.random() * 5);
        
        const newData = [...prev, { time: newTime, thermal: newThermal, power: newPower }];
        if (newData.length > 20) newData.shift();
        return newData;
      });
    }, 1000);
    
    return () => clearInterval(intervalId);
  }, [loadOptions, timer]);

  return (
    <div className="max-w-5xl mx-auto flex flex-col gap-8 animate-in fade-in duration-300">
      <div className="flex items-center justify-between border-b border-white/10 pb-6 shrink-0">
        <div>
           <h1 className="text-2xl font-bold text-white flex items-center gap-3">
             <Flame className="w-6 h-6 text-orange-500" /> Synthetic Load Generator
           </h1>
           <p className="text-slate-400 text-sm mt-1">Stress test physical or virtual hardware boundaries and monitor thermal/power response.</p>
        </div>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
         {/* CPU Toggle */}
         <div className={`p-6 rounded-2xl border transition-all ${loadOptions.cpu ? 'bg-orange-500/10 border-orange-500/50 shadow-[0_0_20px_rgba(249,115,22,0.2)]' : 'bg-white/5 border-white/10 hover:border-white/20'}`}>
            <div className="flex justify-between items-center mb-4">
              <div className="p-3 bg-indigo-500/20 text-indigo-400 rounded-xl"><Cpu className="w-6 h-6" /></div>
              <button 
                 onClick={() => setLoadOptions(p => ({ ...p, cpu: !p.cpu }))} 
                 className={`px-4 py-1.5 rounded-full text-xs font-bold uppercase tracking-wider transition-colors ${loadOptions.cpu ? 'bg-orange-500 text-white' : 'bg-white/10 text-slate-300 hover:bg-white/20'}`}
              >
                {loadOptions.cpu ? 'Running' : 'Start'}
              </button>
            </div>
            <h3 className="font-bold text-white text-lg">CPU Burn-in</h3>
            <p className="text-sm text-slate-400 mt-2">Maxes out all symmetric threads using floating point matrix operations.</p>
         </div>

         {/* GPU Toggle */}
         <div className={`p-6 rounded-2xl border transition-all ${loadOptions.gpu ? 'bg-orange-500/10 border-orange-500/50 shadow-[0_0_20px_rgba(249,115,22,0.2)]' : 'bg-white/5 border-white/10 hover:border-white/20'}`}>
            <div className="flex justify-between items-center mb-4">
              <div className="p-3 bg-cyan-500/20 text-cyan-400 rounded-xl"><Navigation className="w-6 h-6" /></div>
              <button 
                 onClick={() => setLoadOptions(p => ({ ...p, gpu: !p.gpu }))} 
                 className={`px-4 py-1.5 rounded-full text-xs font-bold uppercase tracking-wider transition-colors ${loadOptions.gpu ? 'bg-orange-500 text-white' : 'bg-white/10 text-slate-300 hover:bg-white/20'}`}
              >
                {loadOptions.gpu ? 'Running' : 'Start'}
              </button>
            </div>
            <h3 className="font-bold text-white text-lg">GPU Tensor Stress</h3>
            <p className="text-sm text-slate-400 mt-2">Floods internal VRAM and maximizes shader core utilization.</p>
         </div>
         
         {/* RAM Toggle */}
         <div className={`p-6 rounded-2xl border transition-all ${loadOptions.ram ? 'bg-orange-500/10 border-orange-500/50 shadow-[0_0_20px_rgba(249,115,22,0.2)]' : 'bg-white/5 border-white/10 hover:border-white/20'}`}>
            <div className="flex justify-between items-center mb-4">
              <div className="p-3 bg-emerald-500/20 text-emerald-400 rounded-xl"><Hash className="w-6 h-6" /></div>
              <button 
                 onClick={() => setLoadOptions(p => ({ ...p, ram: !p.ram }))} 
                 className={`px-4 py-1.5 rounded-full text-xs font-bold uppercase tracking-wider transition-colors ${loadOptions.ram ? 'bg-orange-500 text-white' : 'bg-white/10 text-slate-300 hover:bg-white/20'}`}
              >
                {loadOptions.ram ? 'Running' : 'Start'}
              </button>
            </div>
            <h3 className="font-bold text-white text-lg">Memory I/O Flood</h3>
            <p className="text-sm text-slate-400 mt-2">Rapid allocations and random access reads to overwhelm cache bandwidth.</p>
         </div>
      </div>
      
      <div className="bg-[#050811] border border-white/10 rounded-2xl p-6 shadow-2xl relative">
        <h3 className="text-sm font-mono tracking-widest text-slate-400 tracking-wider mb-6 flex items-center justify-between">
          <span>Real-time Telemetry Data</span>
          <span className="flex gap-4">
             <span className="flex items-center gap-2"><div className="w-3 h-3 rounded-full bg-orange-500"></div> Temp (°C)</span>
             <span className="flex items-center gap-2"><div className="w-3 h-3 rounded-full bg-cyan-400"></div> Power (W)</span>
          </span>
        </h3>
        <div className="h-72 w-full">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={metricsData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
              <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" />
              <XAxis dataKey="time" stroke="#64748b" tick={{ fill: '#64748b' }} axisLine={false} tickLine={false} />
              <YAxis yAxisId="left" stroke="#f97316" domain={[20, 100]} />
              <YAxis yAxisId="right" orientation="right" stroke="#22d3ee" domain={[0, 600]} />
              <RechartsTooltip contentStyle={{ backgroundColor: '#0f172a', borderColor: '#334155', borderRadius: '0.5rem' }} />
              <Line yAxisId="left" type="monotone" dataKey="thermal" stroke="#f97316" strokeWidth={3} dot={false} activeDot={{ r: 6 }} animationDuration={500} />
              <Line yAxisId="right" type="monotone" dataKey="power" stroke="#22d3ee" strokeWidth={3} dot={false} activeDot={{ r: 6 }} animationDuration={500} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>
    </div>
  );
}
