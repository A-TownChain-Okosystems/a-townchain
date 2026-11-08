import React, { useState, useEffect } from 'react';
import { SettingsView } from './SettingsView';
import { Settings, Cpu, HardDrive, Wifi, Bell, Shield, Network, Zap, Check, Eye, Trash, BellRing, RefreshCw, MousePointer2, Gamepad2, MonitorSmartphone, Maximize, BatteryWarning, Sun } from 'lucide-react';

export function SystemSettingsView() {
  return <SettingsView initialTab="system" />;
}

export function SoftwareSettingsView() {
  const [apps, setApps] = useState([
    { id: 'atc-word', name: 'A-TownChain Word', bg: true, notifs: true, updates: 'auto' },
    { id: 'atc-excel', name: 'A-TownChain Excel', bg: false, notifs: false, updates: 'auto' },
    { id: 'atc-present', name: 'A-TownChain Present', bg: true, notifs: true, updates: 'manual' },
    { id: 'atc-paint', name: 'dPaint 3D Mesh', bg: false, notifs: false, updates: 'auto' },
    { id: 'atc-benchmark', name: 'Benchmark Center', bg: false, notifs: true, updates: 'auto' },
  ]);

  const toggleBg = (id: string) => setApps(apps.map(a => a.id === id ? { ...a, bg: !a.bg } : a));
  const toggleNotifs = (id: string) => setApps(apps.map(a => a.id === id ? { ...a, notifs: !a.notifs } : a));
  const toggleUpdates = (id: string) => setApps(apps.map(a => a.id === id ? { ...a, updates: a.updates === 'auto' ? 'manual' : 'auto' } : a));

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="p-6 border-b border-white/5 bg-black/40">
        <h2 className="text-xl font-bold font-mono text-white mb-1"><Settings className="inline-block w-5 h-5 mr-2" />Ecosystem Software & Apps</h2>
        <p className="text-sm text-slate-400">Manage background services, notifications, and update preferences for installed apps.</p>
      </div>
      <div className="flex-1 overflow-y-auto p-6">
        <div className="max-w-4xl mx-auto space-y-4">
          {apps.map(app => (
            <div key={app.id} className="bg-white/5 border border-white/10 rounded-xl p-5 flex flex-col sm:flex-row gap-6">
              <div className="flex-1">
                <h3 className="font-bold text-white text-lg">{app.name}</h3>
                <p className="text-xs text-slate-400 font-mono mt-1">ID: {app.id}</p>
              </div>
              <div className="flex flex-wrap gap-4 items-center">
                <button onClick={() => toggleBg(app.id)} className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-colors ${app.bg ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30' : 'bg-white/5 text-slate-400 border border-transparent hover:bg-white/10'}`}>
                  <Zap className="w-4 h-4" /> {app.bg ? 'BG Active' : 'BG Disabled'}
                </button>
                <button onClick={() => toggleNotifs(app.id)} className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-colors ${app.notifs ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30' : 'bg-white/5 text-slate-400 border border-transparent hover:bg-white/10'}`}>
                  <BellRing className="w-4 h-4" /> {app.notifs ? 'Notifs On' : 'Notifs Off'}
                </button>
                <button onClick={() => toggleUpdates(app.id)} className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-colors ${app.updates === 'auto' ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30' : 'bg-amber-500/20 text-amber-300 border border-amber-500/30'}`}>
                  <RefreshCw className="w-4 h-4" /> {app.updates === 'auto' ? 'Auto-Update' : 'Manual'}
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export function HardwareSettingsView() {
  const [cpuCores, setCpuCores] = useState(8);
  const [vram, setVram] = useState(4);
  const [gpuHardwareAccel, setGpuHardwareAccel] = useState(true);
  const [isLowPowerMode, setIsLowPowerMode] = useState(false);
  const [themeSync, setThemeSync] = useState(() => localStorage.getItem('atc_theme_sync') === 'true');
  const [ambientLight, setAmbientLight] = useState<number | null>(null);
  const [isStressTest, setIsStressTest] = useState(false);
  const [powerMode, setPowerMode] = useState<'high-performance' | 'balanced' | 'eco'>(() => {
    return (localStorage.getItem('atc_power_mode') as any) || 'balanced';
  });

  useEffect(() => {
    window.dispatchEvent(new CustomEvent('ATC_SET_STRESS_TEST', { detail: isStressTest }));
  }, [isStressTest]);

  useEffect(() => {
    localStorage.setItem('atc_power_mode', powerMode);
    window.dispatchEvent(new CustomEvent('ATC_SET_POWER_MODE', { detail: powerMode }));
  }, [powerMode]);

  useEffect(() => {
    const handleGlobalPowerMode = (e: any) => {
      if (e.detail !== powerMode) {
        setPowerMode(e.detail);
      }
    };
    window.addEventListener('ATC_SET_POWER_MODE', handleGlobalPowerMode);
    return () => window.removeEventListener('ATC_SET_POWER_MODE', handleGlobalPowerMode);
  }, [powerMode]);

  useEffect(() => {
    const handleLowPower = (e: Event) => {
      const customEvent = e as CustomEvent;
      setIsLowPowerMode(customEvent.detail);
    };
    window.addEventListener('ATC_LOW_POWER_MODE', handleLowPower);
    return () => window.removeEventListener('ATC_LOW_POWER_MODE', handleLowPower);
  }, []);

  useEffect(() => {
    localStorage.setItem('atc_theme_sync', themeSync ? 'true' : 'false');
    if (!themeSync) return;
    
    let sensor: any = null;
    let fallbackInterval: NodeJS.Timeout | null = null;

    const handleReading = (illuminance: number) => {
      setAmbientLight(illuminance);
      const newMode = illuminance > 50 ? 'light' : 'dark';
      if (localStorage.getItem('atc_os_mode') !== newMode) {
        localStorage.setItem('atc_os_mode', newMode);
        window.dispatchEvent(new Event('storage'));
      }
    };

    if ('AmbientLightSensor' in window) {
      try {
        // @ts-ignore
        sensor = new window.AmbientLightSensor();
        sensor.addEventListener('reading', () => {
          handleReading(sensor.illuminance);
        });
        sensor.start();
      } catch(err) {
        console.log('ambient light sensor err', err);
        fallbackInterval = setInterval(() => {
          setAmbientLight(prev => {
            const next = prev === null ? 40 : prev > 50 ? 40 : 60;
            handleReading(next);
            return next;
          });
        }, 5000);
      }
    } else {
      fallbackInterval = setInterval(() => {
        setAmbientLight(prev => {
          const next = prev === null ? 40 : prev > 50 ? 40 : 60;
          handleReading(next);
          return next;
        });
      }, 5000);
    }
    
    return () => {
      if (sensor) sensor.stop();
      if (fallbackInterval) clearInterval(fallbackInterval);
    }
  }, [themeSync]);

  const applyPowerSavingAction = () => {
    if (cpuCores > 4) setCpuCores(4);
    if (vram > 2) setVram(2);
    setGpuHardwareAccel(false);
  };

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="p-6 border-b border-white/5 bg-black/40">
        <div className="flex items-center justify-between mb-1">
          <h2 className="text-xl font-bold font-mono text-white"><HardDrive className="inline-block w-5 h-5 mr-2" />Virtual Hardware Config</h2>
          {isLowPowerMode && (
            <div className="flex items-center gap-1.5 px-2 py-1 bg-red-500/20 text-red-400 rounded text-xs font-bold tracking-wider uppercase animate-pulse">
              <BatteryWarning className="w-3.5 h-3.5" /> Low Power Mode Active
            </div>
          )}
        </div>
        <p className="text-sm text-slate-400">Configure virtualized resources and acceleration for the ATC Virtual Machine.</p>
      </div>
      <div className="flex-1 overflow-y-auto p-6">
        <div className="max-w-2xl mx-auto space-y-8">
          
          {isLowPowerMode && (
            <div className="bg-red-500/10 border border-red-500/20 rounded-xl p-4 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
              <div>
                <h4 className="text-red-400 font-bold mb-1">Power Saving Actions Suggested</h4>
                <p className="text-sm text-slate-400">Reduce CPU cores, VRAM, and disable hardware acceleration to extend battery life.</p>
              </div>
              <button 
                onClick={applyPowerSavingAction}
                className="px-4 py-2 bg-red-500/20 hover:bg-red-500/30 text-red-300 rounded-lg text-sm font-medium transition-colors border border-red-500/30 whitespace-nowrap"
              >
                Apply Optimal Settings
              </button>
            </div>
          )}

          <div id="power-management-section" className="bg-white/5 border border-white/10 rounded-xl p-6">
             <h3 className="font-bold text-white mb-4 flex items-center gap-2"><BatteryWarning className="w-5 h-5 text-amber-400" /> Power Management</h3>
             <div className="flex gap-2">
               <button
                 className={`flex-1 flex flex-col items-center justify-center p-4 rounded-xl border transition-all ${powerMode === 'high-performance' ? 'bg-indigo-500/20 border-indigo-500/50 text-indigo-300' : 'bg-black/20 border-white/10 text-slate-400 hover:border-white/20'}`}
                 onClick={() => setPowerMode('high-performance')}
               >
                 <Zap className={`w-6 h-6 mb-2 ${powerMode === 'high-performance' ? 'text-indigo-400' : 'text-slate-500'}`} />
                 <span className="font-medium text-sm">High Performance</span>
                 <span className="text-[10px] opacity-70 mt-1 text-center">Max CPU/GPU clock</span>
               </button>
               <button
                 className={`flex-1 flex flex-col items-center justify-center p-4 rounded-xl border transition-all ${powerMode === 'balanced' ? 'bg-indigo-500/20 border-indigo-500/50 text-indigo-300' : 'bg-black/20 border-white/10 text-slate-400 hover:border-white/20'}`}
                 onClick={() => setPowerMode('balanced')}
               >
                 <Shield className={`w-6 h-6 mb-2 ${powerMode === 'balanced' ? 'text-indigo-400' : 'text-slate-500'}`} />
                 <span className="font-medium text-sm">Balanced</span>
                 <span className="text-[10px] opacity-70 mt-1 text-center">Recommended</span>
               </button>
               <button
                 className={`flex-1 flex flex-col items-center justify-center p-4 rounded-xl border transition-all ${powerMode === 'eco' ? 'bg-indigo-500/20 border-indigo-500/50 text-indigo-300' : 'bg-black/20 border-white/10 text-slate-400 hover:border-white/20'}`}
                 onClick={() => setPowerMode('eco')}
               >
                 <BatteryWarning className={`w-6 h-6 mb-2 ${powerMode === 'eco' ? 'text-indigo-400' : 'text-slate-500'}`} />
                 <span className="font-medium text-sm">Eco Mode</span>
                 <span className="text-[10px] opacity-70 mt-1 text-center">Save battery</span>
               </button>
             </div>
          </div>

          <div className="bg-white/5 border border-white/10 rounded-xl p-6">
             <div className="flex items-center justify-between">
                <div>
                  <h3 className="font-bold text-white mb-1"><Zap className="inline w-4 h-4 mr-1 text-orange-400" /> Virtual Stress Test</h3>
                  <p className="text-xs text-slate-400">Trigger accelerated virtual battery drain to simulate high-load scenarios.</p>
                </div>
                <button
                  onClick={() => setIsStressTest(!isStressTest)}
                  className={`w-14 h-8 rounded-full border p-1 transition-colors ${isStressTest ? 'bg-orange-500/20 border-orange-500/50' : 'bg-slate-800 border-slate-700'}`}
                >
                  <div className={`w-6 h-6 rounded-full transition-transform ${isStressTest ? 'translate-x-6 bg-orange-400 shadow-[0_0_10px_rgba(249,115,22,0.5)]' : 'bg-slate-500'}`} />
                </button>
             </div>
          </div>

          <div className="bg-white/5 border border-white/10 rounded-xl p-6">
             <div className="flex items-center justify-between">
                <div>
                  <h3 className="font-bold text-white mb-1"><Sun className="inline w-4 h-4 mr-1 text-amber-400" /> Theme Sync (Ambient Light)</h3>
                  <p className="text-xs text-slate-400">Match desktop theme with virtual ambient light sensor data</p>
                </div>
                <button
                  onClick={() => setThemeSync(!themeSync)}
                  className={`w-14 h-8 rounded-full border p-1 transition-colors ${themeSync ? 'bg-amber-500/20 border-amber-500/50' : 'bg-slate-800 border-slate-700'}`}
                >
                  <div className={`w-6 h-6 rounded-full transition-transform ${themeSync ? 'translate-x-6 bg-amber-400 shadow-[0_0_10px_rgba(251,191,36,0.5)]' : 'bg-slate-500'}`} />
                </button>
             </div>
             {themeSync && (
               <div className="mt-4 p-3 bg-black/40 rounded-lg text-xs font-mono text-slate-300">
                 Ambient Light: {ambientLight !== null ? `${Math.round(ambientLight)} lux` : 'reading...'} (System Theme: {ambientLight !== null ? (ambientLight > 50 ? 'Light' : 'Dark') : '...'})
               </div>
             )}
          </div>

          <div className="bg-white/5 border border-white/10 rounded-xl p-6">
            <h3 className="font-bold text-white mb-4 flex items-center gap-2"><Cpu className="w-5 h-5 text-indigo-400" /> Virtual CPU Allocation</h3>
            <div className="flex items-center gap-6">
               <input type="range" min="1" max="32" value={cpuCores} onChange={(e) => setCpuCores(parseInt(e.target.value))} className="flex-1" />
               <div className="w-24 text-right font-mono text-xl text-white">{cpuCores} Cores</div>
            </div>
            <p className="text-xs text-slate-400 mt-3">Allocates simulated CPU threads to the ATVM environment.</p>
          </div>

          <div className="bg-white/5 border border-white/10 rounded-xl p-6">
            <h3 className="font-bold text-white mb-4 flex items-center gap-2"><div className="p-1 bg-emerald-500/20 rounded"><HardDrive className="w-4 h-4 text-emerald-400" /></div> VRAM Allocation (GB)</h3>
            <div className="flex items-center gap-6">
               <input type="range" min="1" max="16" step="0.5" value={vram} onChange={(e) => setVram(parseFloat(e.target.value))} className="flex-1" />
               <div className="w-24 text-right font-mono text-xl text-white">{vram.toFixed(1)} GB</div>
            </div>
            <p className="text-xs text-slate-400 mt-3">Dedicated video memory for graphics rendering and dPaint 3D.</p>
          </div>

          <div className="bg-white/5 border border-white/10 rounded-xl p-6">
             <div className="flex items-center justify-between">
                <div>
                  <h3 className="font-bold text-white mb-1">GPU Hardware Acceleration</h3>
                  <p className="text-xs text-slate-400">Pass-through host GPU capabilities to the WebGL context</p>
                </div>
                <button
                  onClick={() => setGpuHardwareAccel(!gpuHardwareAccel)}
                  className={`w-14 h-8 rounded-full border p-1 transition-colors ${gpuHardwareAccel ? 'bg-cyan-500/20 border-cyan-500/50' : 'bg-slate-800 border-slate-700'}`}
                >
                  <div className={`w-6 h-6 rounded-full transition-transform ${gpuHardwareAccel ? 'translate-x-6 bg-cyan-400 shadow-[0_0_10px_rgba(34,211,238,0.5)]' : 'bg-slate-500'}`} />
                </button>
             </div>
          </div>

          <div className="bg-white/5 border border-white/10 rounded-xl p-6">
            <h3 className="font-bold text-white mb-4 flex items-center gap-2"><MousePointer2 className="w-5 h-5 text-rose-400" /> Eingabegeräte & Peripherie</h3>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
               {[
                 { id: 'touch', name: 'Touchscreen', icon: MonitorSmartphone },
                 { id: 'mouse_kb', name: 'Maus + Tastatur', icon: MousePointer2 },
                 { id: 'controller', name: 'Controller Steuerung', icon: Gamepad2 },
                 { id: 'vr', name: 'VR (Virtual Reality)', icon: Eye },
                 { id: 'ar', name: 'AR (Augmented Reality)', icon: Maximize },
               ].map(device => (
                 <div key={device.id} className="flex items-center justify-between p-3 bg-black/20 border border-white/5 rounded-lg hover:border-white/10 transition-colors">
                    <div className="flex items-center gap-3">
                       <div className="p-2 bg-white/5 rounded"><device.icon className="w-4 h-4 text-slate-300" /></div>
                       <span className="text-sm font-medium text-white">{device.name}</span>
                    </div>
                    <button className="w-10 h-6 rounded-full border p-0.5 bg-rose-500/20 border-rose-500/50">
                       <div className="w-4 h-4 rounded-full bg-rose-400 translate-x-4 shadow-[0_0_10px_rgba(251,113,133,0.5)]" />
                    </button>
                 </div>
               ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
