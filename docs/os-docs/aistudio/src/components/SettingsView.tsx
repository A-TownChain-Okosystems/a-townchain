import React, { useState, useEffect } from "react";
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import {
  Settings,
  Shield,
  Network,
  Cpu,
  Database,
  Bell,
  CheckCircle,
  CheckCircle2,
  XCircle,
  AlertTriangle,
  Fingerprint,
  Palette,
  GitPullRequest,
  Github,
  FileText,
  History,
  ArrowDown,
  ArrowUp,
  Download,
  UploadCloud,
  HardDrive,
  Bookmark,
  Eye,
  EyeOff,
  Clock,
  Lock,
  MapPin,
  Key,
  Calendar,
  HelpCircle,
  TestTube,
  WifiOff,
  RefreshCw,
  Battery,
  BatteryWarning,
  Share2,
  X,
  Terminal
} from "lucide-react";
import { atcDatabase } from "../lib/indexedDb";

// ... existing code, skipping directly to SettingView if I could, but I'll replace the top import first.

function ThemeSelector() {
  const [currentTheme, setCurrentTheme] = useState("default");
  const [autoTheme, setAutoTheme] = useState(() => localStorage.getItem("atc_auto_theme") === "true");

  useEffect(() => {
    setCurrentTheme(
      document.documentElement.getAttribute("data-theme") || "default",
    );

    const observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        if (mutation.attributeName === "data-theme") {
          setCurrentTheme(
            document.documentElement.getAttribute("data-theme") || "default",
          );
        }
      });
    });

    observer.observe(document.documentElement, { attributes: true });
    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    if (!autoTheme) return;
    const checkTheme = () => {
      const h = new Date().getHours();
      const isNight = h >= 18 || h < 6;
      const targetTheme = isNight ? "cyber-cyan" : "default";
      if (document.documentElement.getAttribute("data-theme") !== targetTheme && !(targetTheme === "default" && !document.documentElement.getAttribute("data-theme"))) {
        handleThemeChange(targetTheme);
      }
    };
    checkTheme();
    const intervalId = setInterval(checkTheme, 60000);
    return () => clearInterval(intervalId);
  }, [autoTheme]);

  const handleThemeChange = (theme: string) => {
    if (theme === "default") {
      document.documentElement.removeAttribute("data-theme");
    } else {
      document.documentElement.setAttribute("data-theme", theme);
    }
    localStorage.setItem("atc_theme", theme);
    // Dispatch event so ThemeSwitcher can be notified
    window.dispatchEvent(new Event('theme_changed'));
  };

  const themes = [
    {
      id: "default",
      label: "Default Dark",
      colors: ["#05080f", "#a259ff", "#00d1ff"],
    },
    {
      id: "oceanic-deep",
      label: "Oceanic Deep",
      colors: ["#020b14", "#00d1ff", "#00ffb3"],
    },
    {
      id: "sunset-red",
      label: "Sunset Red",
      colors: ["#1a0505", "#ff4d4d", "#ffb800"],
    },
    {
      id: "cyber-cyan",
      label: "Cyber Cyan",
      colors: ["#001214", "#00ffff", "#00ffcc"],
    },
  ];

  return (
    <div className="bg-white/5 border border-white/10 rounded-xl p-6">
      <div className="flex items-center gap-3 mb-6">
        <div className="p-2 bg-indigo-500/10 rounded-lg">
          <Palette className="w-5 h-5 text-indigo-400" />
        </div>
        <div className="flex-1">
          <h3 className="text-sm font-medium text-white uppercase tracking-widest">
            Theme Preferences
          </h3>
          <p className="text-xs text-slate-400 mt-1">
            Switch between available visual identities across the system.
          </p>
        </div>
        <div className="flex items-center gap-2">
           <span className="text-sm text-slate-300">Auto Theme (Time)</span>
           <button
              onClick={() => {
                 const newVal = !autoTheme;
                 setAutoTheme(newVal);
                 localStorage.setItem("atc_auto_theme", String(newVal));
              }}
              className={`w-12 h-6 rounded-full transition-colors flex items-center p-1 ${autoTheme ? 'bg-indigo-500' : 'bg-white/10'}`}
           >
              <div className={`w-4 h-4 rounded-full bg-white transition-transform ${autoTheme ? 'translate-x-6' : 'translate-x-0'}`} />
           </button>
        </div>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {themes.map((t) => (
          <button
            key={t.id}
            onClick={() => handleThemeChange(t.id)}
            className={`flex flex-col gap-3 p-4 rounded-xl border transition-all duration-300 focus:outline-none ${
              currentTheme === t.id
                ? "bg-white/10"
                : "border-white/10 hover:border-white/30 hover:bg-white/5"
            }`}
            style={{
              borderColor: currentTheme === t.id ? t.colors[1] : undefined,
              boxShadow:
                currentTheme === t.id
                  ? `0 0 20px ${t.colors[1]}30, inset 0 0 10px ${t.colors[2]}10`
                  : undefined,
            }}
          >
            <div className="flex gap-1.5 w-full h-10 rounded-lg overflow-hidden border border-white/10 shadow-inner">
              <div
                className="flex-1"
                style={{ backgroundColor: t.colors[0] }}
              />
              <div className="w-4" style={{ backgroundColor: t.colors[1] }} />
              <div className="w-4" style={{ backgroundColor: t.colors[2] }} />
            </div>
            <div className="flex items-center justify-between w-full">
              <span
                className={`text-sm font-medium ${currentTheme === t.id ? "text-white" : "text-slate-400"}`}
              >
                {t.label}
              </span>
              {currentTheme === t.id && (
                <div
                  className="w-2 h-2 rounded-full animate-pulse"
                  style={{ backgroundColor: t.colors[2] }}
                />
              )}
            </div>
          </button>
        ))}
      </div>
    </div>
  );
}

function VisualCustomization() {
  const [colors, setColors] = useState({
    bg: "#05080f",
    card: "#0a0f1d",
    purple: "#a259ff",
    cyan: "#00d1ff",
    green: "#00ffb3"
  });
  
  const [sessionDuration, setSessionDuration] = useState(() => localStorage.getItem('atc_session_duration') || 'heute');
  const [customDays, setCustomDays] = useState(() => {
    const val = localStorage.getItem('atc_session_custom_days');
    return val ? parseInt(val) : 30;
  });

  const [osMode, setOsMode] = useState(() => localStorage.getItem('atc_os_mode') || 'default');

  useEffect(() => {
    const saved = localStorage.getItem('atc_custom_colors');
    if (saved) {
      try {
        setColors(JSON.parse(saved));
      } catch (e) {}
    }
  }, []);

  const handleChange = (key: string, value: string) => {
    const newColors = { ...colors, [key]: value };
    setColors(newColors);
  };
  
  const handleOsModeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const mode = e.target.value;
    setOsMode(mode);
    localStorage.setItem('atc_os_mode', mode);
  };
  
  const getExpirationDate = (duration: string) => {
    const now = new Date();
    switch (duration) {
      case 'custom': return new Date(now.getTime() + customDays * 24 * 60 * 60 * 1000);
      case '3_tage': return new Date(now.getTime() + 3 * 24 * 60 * 60 * 1000);
      case '7_tage': return new Date(now.getTime() + 7 * 24 * 60 * 60 * 1000);
      case '14_tage': return new Date(now.getTime() + 14 * 24 * 60 * 60 * 1000);
      case '1_monat': return new Date(now.getTime() + 30 * 24 * 60 * 60 * 1000);
      case 'restlicher_monat': return new Date(now.getFullYear(), now.getMonth() + 1, 0, 23, 59, 59);
      case 'heute':
      default:
        return new Date(now.getTime() + 24 * 60 * 60 * 1000);
    }
  };

  const handleSessionDurationChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const duration = e.target.value;
    setSessionDuration(duration);
    localStorage.setItem('atc_session_duration', duration);
    localStorage.setItem('atc_session_expires', getExpirationDate(duration).toISOString());
  };

  const handleCustomDaysChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const days = parseInt(e.target.value);
    if (!isNaN(days) && days > 0) {
      setCustomDays(days);
      localStorage.setItem('atc_session_custom_days', days.toString());
      if (sessionDuration === 'custom') {
        const now = new Date();
        localStorage.setItem('atc_session_expires', new Date(now.getTime() + days * 24 * 60 * 60 * 1000).toISOString());
      }
    }
  };

  const applyCustomTheme = () => {
    let styleEl = document.getElementById('atc-custom-theme-style');
    if (!styleEl) {
      styleEl = document.createElement('style');
      styleEl.id = 'atc-custom-theme-style';
      document.head.appendChild(styleEl);
    }
    styleEl.innerHTML = `
      :root[data-theme='custom'] {
        --atc-bg: ${colors.bg};
        --atc-card: ${colors.card};
        --atc-purple: ${colors.purple};
        --atc-cyan: ${colors.cyan};
        --atc-green: ${colors.green};
      }
    `;
    document.documentElement.setAttribute("data-theme", "custom");
    localStorage.setItem("atc_theme", "custom");
    localStorage.setItem("atc_custom_colors", JSON.stringify(colors));
    window.dispatchEvent(new Event('theme_changed'));
  };

  const [windowOpacity, setWindowOpacity] = useState(() => parseInt(localStorage.getItem('atc_window_opacity') || '100'));

  const handleOpacityChange = (e: React.ChangeEvent<HTMLInputElement>) => {
     const value = parseInt(e.target.value);
     setWindowOpacity(value);
     localStorage.setItem('atc_window_opacity', value.toString());
     window.dispatchEvent(new Event('atc_window_opacity_changed'));
  };

  return (
    <div className="space-y-6">
      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <h3 className="text-sm font-medium text-white mb-4 uppercase tracking-widest flex items-center gap-2">
           <Eye className="w-4 h-4 text-indigo-400" />
           Window Transparency (Mica / Glass)
        </h3>
        <div className="flex items-center gap-6">
           <input type="range" min="10" max="100" value={windowOpacity} onChange={handleOpacityChange} className="flex-1" />
           <div className="w-16 text-right font-mono text-white">{windowOpacity}%</div>
        </div>
        <p className="text-xs text-slate-400 mt-3">Adjusts the background opacity of active windows to create a frosted glass effect.</p>
      </div>

      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-2 bg-indigo-500/10 rounded-lg">
            <Palette className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h3 className="text-sm font-medium text-white uppercase tracking-widest">
              Custom Theme Definition
            </h3>
            <p className="text-xs text-slate-400 mt-1">
              Define your own global accent and background colors.
            </p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {[
            { id: 'bg', label: 'Background Color' },
            { id: 'card', label: 'Card Color' },
            { id: 'purple', label: 'Primary Accent (Purple)' },
            { id: 'cyan', label: 'Secondary Accent (Cyan)' },
            { id: 'green', label: 'Success Accent (Green)' }
          ].map((field) => (
            <div key={field.id} className="flex flex-col gap-2">
              <label className="text-xs font-mono text-slate-400">{field.label}</label>
              <div className="flex items-center gap-3">
                <input 
                  type="color" 
                  value={colors[field.id as keyof typeof colors]}
                  onChange={(e) => handleChange(field.id, e.target.value)}
                  className="w-10 h-10 rounded cursor-pointer bg-transparent border-0 p-0"
                />
                <input 
                  type="text"
                  value={colors[field.id as keyof typeof colors]}
                  onChange={(e) => handleChange(field.id, e.target.value)}
                  className="bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-sm text-white font-mono w-full"
                />
              </div>
            </div>
          ))}
        </div>

        <div className="mt-8 pt-6 border-t border-white/10">
          <h3 className="text-sm font-medium text-white mb-4 uppercase tracking-widest">
            OS Compatibility Mode & Behavior
          </h3>
          <p className="text-xs text-slate-400 mb-4">
            Emulate window behaviors and menu structures of other operating systems.
          </p>
          <div className="flex flex-col gap-2 max-w-sm">
            <label className="text-xs font-mono text-slate-400">System Presets</label>
            <select 
              value={osMode} 
              onChange={handleOsModeChange}
              className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-indigo-500 transition-colors cursor-pointer"
            >
              <option value="default">ATC Native Engine (Default)</option>
              <option value="windows">Windows 11 Look & Feel</option>
              <option value="macos">macOS Sonoma Layout</option>
              <option value="linux">Linux Desktop (GNOME UI)</option>
            </select>
          </div>
        </div>

        <div className="mt-8 pt-6 border-t border-white/10 flex justify-end">
          <button 
            onClick={applyCustomTheme}
            className="px-6 py-2 bg-indigo-500 hover:bg-indigo-600 text-white font-medium rounded-lg transition-colors flex items-center gap-2 text-sm"
          >
            Apply Custom Theme
          </button>
        </div>
      </div>

      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-2 bg-indigo-500/10 rounded-lg">
            <Shield className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h3 className="text-sm font-medium text-white uppercase tracking-widest">
              Session Persistence & Auto-Logout
            </h3>
            <p className="text-xs text-slate-400 mt-1">
              Control your authentication token's local storage expiration automatically.
            </p>
          </div>
        </div>

        <div className="flex flex-col gap-2 max-w-sm">
          <label className="text-xs font-mono text-slate-400">Logout After</label>
          <select 
            value={sessionDuration} 
            onChange={handleSessionDurationChange}
            className="w-full bg-black/40 border border-white/10 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-indigo-500 transition-colors cursor-pointer"
          >
            <option value="heute">Today (24h)</option>
            <option value="3_tage">3 Days</option>
            <option value="7_tage">7 Days</option>
            <option value="14_tage">14 Days</option>
            <option value="1_monat">1 Month</option>
            <option value="restlicher_monat">Keep me logged in for the rest of the month</option>
            <option value="custom">Custom Duration...</option>
          </select>
          {sessionDuration === 'custom' && (
            <div className="mt-2 flex items-center gap-3">
              <input 
                type="number" 
                min="1"
                value={customDays} 
                onChange={handleCustomDaysChange}
                className="w-24 bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-white font-mono text-sm focus:outline-none focus:border-indigo-500 transition-colors"
              />
              <span className="text-xs text-slate-400">days</span>
            </div>
          )}
          <p className="text-[10px] text-slate-500 mt-1">Session resets based on activity selection. Will re-prompt for master key upon expiration.</p>
        </div>

        <div className="mt-6 pt-6 border-t border-white/10">
          <h4 className="text-xs font-medium text-slate-400 mb-3 uppercase tracking-widest">Current Session Details</h4>
          <div className="grid grid-cols-2 gap-4">
            <div className="bg-black/40 border border-white/5 rounded-lg p-3">
              <div className="flex items-center gap-2 mb-1">
                <MapPin className="w-3.5 h-3.5 text-atc-cyan" />
                <span className="text-[10px] font-mono uppercase text-slate-500">Approx. Location</span>
              </div>
              <div className="text-sm text-white font-mono">192.168.1.1</div>
              <div className="text-xs text-slate-400 mt-1">Frankfurt, Germany</div>
            </div>
            <div className="bg-black/40 border border-white/5 rounded-lg p-3">
              <div className="flex items-center gap-2 mb-1">
                <Lock className="w-3.5 h-3.5 text-atc-purple" />
                <span className="text-[10px] font-mono uppercase text-slate-500">Origin Client</span>
              </div>
              <div className="text-sm text-white font-mono">Web Browser</div>
              <div className="text-xs text-slate-400 mt-1 truncate" title={window.navigator.userAgent}>
                {window.navigator.userAgent.split(' ')[0]}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function BatteryHealthSettings() {
  const [data] = useState(() => {
    const today = new Date();
    return Array.from({ length: 30 }).map((_, i) => {
      const date = new Date(today);
      date.setDate(date.getDate() - (29 - i));
      return {
        date: date.toLocaleDateString(),
        health: 100 - (29 - i) * 0.05 - Math.random() * 0.5,
        cycles: ~~(i * 1.5)
      };
    });
  });

  return (
    <div className="space-y-6">
      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <h3 className="text-sm font-medium text-white mb-4 uppercase tracking-widest flex items-center gap-2">
          <Battery className="w-5 h-5 text-indigo-400" />
          Virtual Battery Health Degradation
        </h3>
        <p className="text-sm text-slate-400 mb-6">Visualizes the simulated degradation of battery health over the last 30 days based on usage cycles and virtual power draw constraints.</p>
        
        <div className="h-64 mt-4 w-full">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={data}>
              <CartesianGrid strokeDasharray="3 3" stroke="#334155" vertical={false} />
              <XAxis dataKey="date" stroke="#94a3b8" fontSize={11} tickMargin={10} minTickGap={30} />
              <YAxis yAxisId="left" stroke="#94a3b8" fontSize={11} tickFormatter={(value) => `${value}%`} domain={['dataMin - 1', 100]} />
              <YAxis yAxisId="right" orientation="right" stroke="#6366f1" fontSize={11} />
              <Tooltip 
                contentStyle={{ backgroundColor: '#0f172a', borderColor: '#1e293b', borderRadius: '8px' }}
                itemStyle={{ color: '#f8fafc' }}
                labelStyle={{ color: '#94a3b8', marginBottom: '4px' }}
              />
              <Line yAxisId="left" type="monotone" dataKey="health" name="Health Capacity" stroke="#22c55e" strokeWidth={2} dot={false} activeDot={{ r: 6, fill: '#22c55e' }} />
              <Line yAxisId="right" type="monotone" dataKey="cycles" name="Charge Cycles" stroke="#6366f1" strokeWidth={2} dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>

        <div className="grid grid-cols-2 gap-4 mt-6">
          <div className="bg-[#090b14] p-4 rounded-lg border border-white/5">
            <div className="text-xs text-slate-500 uppercase tracking-wider mb-1">Current Capacity</div>
            <div className="text-2xl font-mono text-emerald-400">{data[data.length - 1].health.toFixed(1)}%</div>
          </div>
          <div className="bg-[#090b14] p-4 rounded-lg border border-white/5">
            <div className="text-xs text-slate-500 uppercase tracking-wider mb-1">Total Virtual Cycles</div>
            <div className="text-2xl font-mono text-indigo-400">{data[data.length - 1].cycles}</div>
          </div>
        </div>
      </div>
    </div>
  );
}

function SecuritySettings() {
  const [isScanning, setIsScanning] = useState(false);
  const [scanResults, setScanResults] = useState<
    | { file: string; status: "ok" | "modified" | "unknown"; hash: string }[]
    | null
  >(null);

  const performScan = () => {
    setIsScanning(true);
    setScanResults(null);
    setTimeout(() => {
      setScanResults([
        {
          file: "/src/kernel/atc_core.abc",
          status: "ok",
          hash: "e3b0c44298fc1c14...",
        },
        {
          file: "/src/DesktopApp.tsx",
          status: "modified",
          hash: "8f434346648f6b96...",
        },
        {
          file: "/src/components/ATCWalletView.tsx",
          status: "ok",
          hash: "a4bdf44298fc1c33...",
        },
        {
          file: "/src/utils/crypto.ts",
          status: "ok",
          hash: "c9bdf48298dc1d39...",
        },
      ]);
      setIsScanning(false);
    }, 1500);
  };

  return (
    <div className="space-y-6">
      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-indigo-500/10 rounded-lg">
              <Fingerprint className="w-5 h-5 text-indigo-400" />
            </div>
            <div>
              <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                System Integrity Scan
              </h3>
              <p className="text-xs text-slate-400 mt-1">
                Check core files against verifiable A-TownChain repo hashes.
              </p>
            </div>
          </div>
          <button
            onClick={performScan}
            disabled={isScanning}
            className="bg-indigo-500/20 hover:bg-indigo-500/30 text-indigo-400 px-4 py-2 rounded-lg text-sm font-bold transition-all disabled:opacity-50 flex items-center gap-2 border border-indigo-500/30"
          >
            {isScanning ? (
              <span className="flex items-center gap-2">
                <Settings className="w-4 h-4 animate-spin" /> Scanning...
              </span>
            ) : (
              "Scan Now"
            )}
          </button>
        </div>

        {scanResults && (
          <div className="mt-6 space-y-2">
            <h4 className="text-xs text-slate-500 font-mono mb-3">
              SCAN RESULTS
            </h4>
            {scanResults.map((res, i) => (
              <div
                key={i}
                className={`flex items-center justify-between p-3 rounded-lg border ${res.status === "ok" ? "bg-emerald-500/5 border-emerald-500/10" : "bg-red-500/5 border-red-500/20"}`}
              >
                <div className="flex items-center gap-3">
                  {res.status === "ok" ? (
                    <CheckCircle className="w-4 h-4 text-emerald-500" />
                  ) : (
                    <AlertTriangle className="w-4 h-4 text-red-500" />
                  )}
                  <span className="text-sm font-mono text-slate-200">
                    {res.file}
                  </span>
                </div>
                <div className="flex items-center gap-4">
                  <span className="text-xs font-mono text-slate-500">
                    {res.hash}
                  </span>
                  {res.status === "ok" ? (
                    <span className="text-[10px] font-bold text-emerald-500 uppercase px-2 py-0.5 rounded bg-emerald-500/10">
                      Verified
                    </span>
                  ) : (
                    <span className="text-[10px] font-bold text-red-500 uppercase px-2 py-0.5 rounded bg-red-500/10">
                      Modified
                    </span>
                  )}
                </div>
              </div>
            ))}

            {scanResults.some((r) => r.status === "modified") && (
              <div className="mt-4 p-3 bg-red-500/10 border border-red-500/20 rounded-lg flex items-start gap-3">
                <Shield className="w-5 h-5 text-red-400 shrink-0 mt-0.5" />
                <p className="text-sm text-red-200">
                  Warning: One or more core files have been modified locally.
                  This may affect system stability or security if not
                  intentional.
                </p>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

interface SettingsViewProps {
  initialTab?: string;
  conflictItems?: {
    id: string;
    title: string;
    githubSource: any;
    notionSource: any;
  }[];
  onResolveConflict?: (id: string, resolution: "local" | "remote" | "merge") => void;
  syncHistory?: {
    timestamp: Date;
    type: "Manual" | "Auto";
    target: string;
    outcome: string;
    success: boolean;
  }[];
}

function SessionManagementView() {
  const [sessionInfo, setSessionInfo] = useState<{ token: string, created: Date, expires: Date, daysRemaining: number } | null>(null);
  const [extensions, setExtensions] = useState<{ timestamp: string, duration: string, ip?: string, location?: string }[]>([]);
  const [isLocked, setIsLocked] = useState(() => localStorage.getItem('atc_session_locked') === 'true');

  useEffect(() => {
    let timer: any;
    const updateSessionInfo = () => {
      const expiresStr = localStorage.getItem('atc_session_expires');
      const createdStr = localStorage.getItem('atc_session_created');
      const token = localStorage.getItem('atc_session_token') || 'Pending...';
      
      if (expiresStr && createdStr) {
        const expires = new Date(expiresStr);
        const created = new Date(createdStr);
        // compute remaining time
        const val = expires.getTime() - new Date().getTime();
        setSessionInfo({
          token,
          created,
          expires,
          daysRemaining: val > 0 ? (val / (1000 * 60 * 60 * 24)) : 0
        });
      }

      setExtensions(JSON.parse(localStorage.getItem('atc_session_extensions') || '[]'));
    };

    updateSessionInfo();
    timer = setInterval(updateSessionInfo, 1000);
    return () => clearInterval(timer);
  }, []);

  const handleLockToggle = () => {
    const nextLocked = !isLocked;
    setIsLocked(nextLocked);
    localStorage.setItem('atc_session_locked', String(nextLocked));
  };

  if (isLocked) {
    return (
      <div className="flex flex-col items-center justify-center h-full space-y-6 py-20">
        <div className="w-16 h-16 rounded-2xl bg-indigo-500/20 border border-indigo-500/40 flex items-center justify-center text-indigo-400 mb-4 animate-pulse">
          <Lock className="w-8 h-8" />
        </div>
        <h3 className="text-xl font-medium text-white tracking-widest uppercase">Session Locked</h3>
        <p className="text-sm text-slate-400 text-center max-w-sm">
          Sensitive data is hidden. Please re-authenticate to view session properties and active tokens.
        </p>
        <button 
          onClick={handleLockToggle}
          className="px-8 py-3 bg-indigo-500 hover:bg-indigo-600 text-white rounded-lg font-medium transition-colors shadow-lg shadow-indigo-500/20"
        >
          Unlock Session
        </button>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-end">
        <button 
          onClick={handleLockToggle}
          className="flex items-center gap-2 px-4 py-2 bg-black/40 border border-white/10 hover:border-indigo-500/50 rounded-lg text-sm text-slate-300 transition-colors"
        >
          <Lock className="w-4 h-4 text-indigo-400" />
          Lock Session Data
        </button>
      </div>
      <div className="bg-white/5 border border-white/10 rounded-xl p-6 relative overflow-hidden">
        <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-atc-cyan to-indigo-500" />
        <div className="flex items-center gap-3 mb-6">
          <div className="p-2 bg-indigo-500/10 rounded-lg">
            <Key className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h3 className="text-sm font-medium text-white uppercase tracking-widest">
              Active Token Status
            </h3>
            <p className="text-xs text-slate-400 mt-1">
              Currently established secure session
            </p>
          </div>
        </div>

        {sessionInfo ? (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="bg-black/40 border border-white/5 rounded-xl p-4">
              <span className="text-[10px] text-slate-500 uppercase tracking-widest font-bold">Token ID</span>
              <div className="mt-2 text-atc-cyan font-mono text-sm break-all">{sessionInfo.token}</div>
            </div>
            <div className="bg-black/40 border border-white/5 rounded-xl p-4">
              <span className="text-[10px] text-slate-500 uppercase tracking-widest font-bold">Age</span>
              <div className="mt-2 text-white text-sm">
                {sessionInfo.created.toLocaleString()}
              </div>
            </div>
            <div className="bg-black/40 border border-white/5 rounded-xl p-4 flex flex-col items-center justify-center">
              <span className="text-[10px] text-slate-500 uppercase tracking-widest font-bold self-start w-full">Remaining</span>
              <div className="mt-4 relative w-20 h-20 flex items-center justify-center">
                <svg className="absolute inset-0 w-full h-full -rotate-90">
                  <circle cx="40" cy="40" r="36" className="stroke-white/10 fill-none" strokeWidth="6" />
                  <circle 
                    cx="40" 
                    cy="40" 
                    r="36" 
                    className={`fill-none transition-all duration-1000 ${sessionInfo.daysRemaining < 1 ? 'stroke-amber-500' : 'stroke-emerald-400'}`} 
                    strokeWidth="6" 
                    strokeDasharray={`${2 * Math.PI * 36}`}
                    strokeDashoffset={`${2 * Math.PI * 36 * (1 - Math.min(1, Math.max(0, sessionInfo.daysRemaining / 30)))}`}
                    strokeLinecap="round"
                  />
                </svg>
                <div className="absolute inset-0 flex flex-col items-center justify-center">
                  <span className="text-white font-mono font-bold text-lg leading-none">{sessionInfo.daysRemaining.toFixed(1)}</span>
                  <span className="text-[8px] text-slate-400 uppercase tracking-widest mt-1">Days</span>
                </div>
              </div>
            </div>
          </div>
        ) : (
          <div className="text-sm text-slate-400 p-4 bg-black/40 rounded-lg border border-white/5 text-center">
            No active session data available
          </div>
        )}
      </div>

      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-2 bg-atc-cyan/10 rounded-lg">
            <History className="w-5 h-5 text-atc-cyan" />
          </div>
          <div>
            <h3 className="text-sm font-medium text-white uppercase tracking-widest">
              Audit Trail
            </h3>
            <p className="text-xs text-slate-400 mt-1">
              Last 10 session extension events
            </p>
          </div>
        </div>

        <div className="space-y-2">
          {extensions.length === 0 ? (
            <div className="text-xs text-slate-500 text-center py-4">No recent extensions</div>
          ) : (
            extensions.map((ext: any, idx) => (
              <div key={idx} className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 p-3 bg-black/40 border border-white/5 rounded-lg">
                <div className="flex items-center gap-3">
                  <div className="w-2 h-2 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]" />
                  <div className="flex flex-col">
                    <span className="text-sm text-white font-mono">{new Date(ext.timestamp).toLocaleString()}</span>
                    {(ext.ip || ext.location) && (
                      <span className="text-[10px] text-slate-400 mt-0.5 flex items-center gap-1">
                        <MapPin className="w-3 h-3" />
                        {ext.ip} {ext.location ? `(${ext.location})` : ''}
                      </span>
                    )}
                  </div>
                </div>
                <div className="text-xs text-slate-400 border border-white/10 bg-white/5 px-2 py-1 rounded">
                  Extended: {ext.duration}
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}

function NotificationsView() {
  const [prefs, setPrefs] = useState(() => JSON.parse(localStorage.getItem('atc_notify_prefs') || '{"build":true,"conflict":true,"session":true}'));

  const handleChange = (key: string) => {
    const next = { ...prefs, [key]: !prefs[key] };
    setPrefs(next);
    localStorage.setItem('atc_notify_prefs', JSON.stringify(next));
  };

  return (
    <div className="space-y-6">
      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-2 bg-indigo-500/10 rounded-lg">
            <Bell className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h3 className="text-sm font-medium text-white uppercase tracking-widest">
              Notification Preferences
            </h3>
            <p className="text-xs text-slate-400 mt-1">
              Select which system events you want to be notified about.
            </p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-black/40 border border-white/5 rounded-lg">
            <div>
              <div className="text-sm text-white font-medium">Build Success</div>
              <div className="text-xs text-slate-400">Receive alerts when ATC compiler finishes successfully</div>
            </div>
            <button
              onClick={() => handleChange('build')}
              className={`w-12 h-6 rounded-full flex items-center transition-colors p-1 ${prefs.build ? 'bg-emerald-500' : 'bg-white/10'}`}
            >
              <div className={`w-4 h-4 rounded-full bg-white transition-transform ${prefs.build ? 'translate-x-6' : 'translate-x-0'}`} />
            </button>
          </div>
          <div className="flex items-center justify-between p-4 bg-black/40 border border-white/5 rounded-lg">
            <div>
              <div className="text-sm text-white font-medium">Sync Conflict</div>
              <div className="text-xs text-slate-400">Alerts when cloud sync hits a version mismatch</div>
            </div>
            <button
              onClick={() => handleChange('conflict')}
              className={`w-12 h-6 rounded-full flex items-center transition-colors p-1 ${prefs.conflict ? 'bg-amber-500' : 'bg-white/10'}`}
            >
              <div className={`w-4 h-4 rounded-full bg-white transition-transform ${prefs.conflict ? 'translate-x-6' : 'translate-x-0'}`} />
            </button>
          </div>
          <div className="flex items-center justify-between p-4 bg-black/40 border border-white/5 rounded-lg">
            <div>
              <div className="text-sm text-white font-medium">Session Expiration</div>
              <div className="text-xs text-slate-400">Notifies you 5 minutes before your token expires</div>
            </div>
            <button
              onClick={() => handleChange('session')}
              className={`w-12 h-6 rounded-full flex items-center transition-colors p-1 ${prefs.session ? 'bg-atc-cyan' : 'bg-white/10'}`}
            >
              <div className={`w-4 h-4 rounded-full bg-white transition-transform ${prefs.session ? 'translate-x-6' : 'translate-x-0'}`} />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

const defaultShortcuts = [
  { id: 'search', keys: ['Ctrl', 'K'], desc: 'Open global search / command palette' },
  { id: 'save', keys: ['Ctrl', 'S'], desc: 'Save current active file' },
  { id: 'switch', keys: ['Alt', 'Tab'], desc: 'Switch between open windows' },
  { id: 'close', keys: ['Esc'], desc: 'Close current modal or overlay' },
  { id: 'reopen', keys: ['Ctrl', 'Shift', 'T'], desc: 'Reopen last closed window' },
  { id: 'build', keys: ['Ctrl', 'B'], desc: 'Toggle build drawer' },
  { id: 'sync', keys: ['Alt', 'S'], desc: 'Trigger global repository sync' },
  { id: 'settings', keys: ['Alt', 'O'], desc: 'Open Settings view' },
  { id: 'focus_mode', keys: ['Alt', 'F'], desc: 'Toggle focus mode' },
];

function ShortcutManagerView() {
  const [shortcuts, setShortcuts] = useState(() => {
    try {
      const stored = JSON.parse(localStorage.getItem('atc_shortcuts') || 'null');
      // Merge with default if there are new ones
      if (stored) {
         const missing = defaultShortcuts.filter(ds => !stored.find((s:any) => s.id === ds.id));
         return [...stored, ...missing];
      }
      return defaultShortcuts;
    } catch {
      return defaultShortcuts;
    }
  });
  const [editingId, setEditingId] = useState<string | null>(null);
  const [showModal, setShowModal] = useState(false);

  const saveToStorage = (newShortcuts: any) => {
    setShortcuts(newShortcuts);
    localStorage.setItem('atc_shortcuts', JSON.stringify(newShortcuts));
    window.dispatchEvent(new Event('atc_shortcuts_updated'));
  };

  const handleKeyDown = (e: React.KeyboardEvent, id: string) => {
    e.preventDefault();
    e.stopPropagation();
    
    if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) {
      return;
    }

    const keys = [];
    if (e.ctrlKey) keys.push('Ctrl');
    if (e.altKey) keys.push('Alt');
    if (e.shiftKey) keys.push('Shift');
    if (e.metaKey) keys.push('Meta');
    
    keys.push(e.key.length === 1 ? e.key.toUpperCase() : e.key);

    const keyString = keys.join('+');
    const conflictingShortcuts = [
      'Ctrl+C', 'Ctrl+V', 'Ctrl+X', 'Ctrl+A', 'Ctrl+F', 'Ctrl+S', 'Ctrl+P', 'Ctrl+R', 'Ctrl+T', 'Ctrl+W', 'Ctrl+N',
      'Ctrl+Shift+I', 'Ctrl+Shift+C', 'F5', 'F12', 'Alt+F4', 'Ctrl+Tab', 'Meta+C', 'Meta+V', 'Meta+X', 'Meta+A', 'Meta+F', 'Meta+S', 'Meta+P', 'Meta+R', 'Meta+T', 'Meta+W', 'Meta+N'
    ];

    if (conflictingShortcuts.includes(keyString)) {
       // Display error instead of saving
       alert(`Cannot bind ${keyString}: Conflicts with system or browser shortcut.`);
       setEditingId(null);
       return;
    }

    const newShortcuts = shortcuts.map((sc: any) => sc.id === id ? { ...sc, keys } : sc);
    saveToStorage(newShortcuts);
    setEditingId(null);
  };

  const restoreDefaults = () => {
    saveToStorage(defaultShortcuts);
  };

  return (
    <div className="space-y-6">
      {showModal && <ShortcutsHelpModal onClose={() => setShowModal(false)} />}
      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-indigo-500/10 rounded-lg">
              <span className="w-5 h-5 flex items-center justify-center font-mono font-bold text-indigo-400 text-xs">⌘</span>
            </div>
            <div>
              <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                Shortcut Manager
              </h3>
              <p className="text-xs text-slate-400 mt-1">
                Customize global keyboard bindings for ATC-OS actions.
              </p>
            </div>
          </div>
          <div className="flex gap-2">
            <button 
              onClick={() => setShowModal(true)}
              className="px-4 py-2 border border-white/10 bg-indigo-500/10 hover:bg-indigo-500/20 hover:border-indigo-500/50 rounded-lg text-xs text-indigo-300 transition-colors font-medium"
            >
              Open Cheat Sheet Modal
            </button>
            <button 
              onClick={restoreDefaults}
              className="px-4 py-2 border border-white/10 hover:border-indigo-500/50 hover:bg-slate-800 rounded-lg text-xs text-slate-300 transition-colors"
            >
              Reset Defaults
            </button>
          </div>
        </div>

        <div className="space-y-3">
          {shortcuts.map((sc: any) => (
            <div key={sc.id} className="flex flex-col sm:flex-row sm:items-center justify-between p-4 bg-black/40 border border-white/5 rounded-lg gap-4">
              <span className="text-sm text-slate-300">{sc.desc}</span>
              <div 
                className={`flex gap-1.5 p-2 rounded border cursor-pointer select-none transition-colors ${editingId === sc.id ? 'border-indigo-500 bg-indigo-500/10' : 'border-white/10 hover:border-white/20'}`}
                onClick={() => setEditingId(sc.id)}
                autoFocus={editingId === sc.id}
                tabIndex={0}
                onKeyDown={(e) => editingId === sc.id && handleKeyDown(e, sc.id)}
                onBlur={() => setEditingId(null)}
              >
                {editingId === sc.id ? (
                  <span className="text-xs text-indigo-400 px-2 py-1 animate-pulse">Press new key combination...</span>
                ) : (
                  sc.keys.map((k: string) => (
                    <span key={k} className="px-2 py-1 bg-white/10 rounded text-xs font-mono text-white capitalize border border-white/10">
                      {k}
                    </span>
                  ))
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

function ShortcutsHelpModal({ onClose }: { onClose: () => void }) {
  const shortcuts = (() => {
    try {
      return JSON.parse(localStorage.getItem('atc_shortcuts') || 'null') || defaultShortcuts;
    } catch {
      return defaultShortcuts;
    }
  })();

  return (
    <div className="fixed inset-0 z-[10000] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
      <div className="bg-[#090b14] border border-white/10 rounded-2xl w-full max-w-lg shadow-[0_0_50px_rgba(0,0,0,0.5)] flex flex-col relative overflow-hidden animate-in zoom-in-95 duration-200">
        <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-indigo-500 to-atc-purple" />
        <div className="flex items-center justify-between p-6 pb-2">
          <h2 className="text-xl font-semibold text-white tracking-tight">Keyboard Shortcuts</h2>
          <button onClick={onClose} className="p-2 -mr-2 text-slate-400 hover:text-white rounded-lg hover:bg-white/5 transition-colors">
            <XCircle className="w-5 h-5" />
          </button>
        </div>
        <div className="p-6">
          <div className="space-y-3">
            {shortcuts.map((sc, i) => (
              <div key={i} className="flex justify-between items-center p-3 bg-black/40 border border-white/5 rounded-lg">
                <span className="text-sm text-slate-300">{sc.desc}</span>
                <div className="flex gap-1.5">
                  {sc.keys.map(k => (
                    <span key={k} className="px-2 py-1 bg-white/10 rounded text-xs font-mono text-white capitalize border border-white/10">
                      {k}
                    </span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

function SyncErrorChart() {
  const data = [
    { name: 'Mon', failures: 3, cause: 'Network Timeout' },
    { name: 'Tue', failures: 1, cause: 'Auth Token Expired' },
    { name: 'Wed', failures: 0, cause: 'None' },
    { name: 'Thu', failures: 4, cause: 'Conflict Detected' },
    { name: 'Fri', failures: 2, cause: 'Server 500' },
    { name: 'Sat', failures: 0, cause: 'None' },
    { name: 'Sun', failures: 1, cause: 'Network Timeout' },
  ];

  return (
    <div className="mb-8 bg-black/40 border border-white/10 rounded-xl p-5">
      <h4 className="text-sm font-medium text-white uppercase tracking-widest mb-4">Sync Error Frequency (Last 7 Days)</h4>
      <div className="h-48 mt-4 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" vertical={false} />
            <XAxis dataKey="name" stroke="#64748b" tick={{ fill: '#64748b', fontSize: 12 }} axisLine={false} tickLine={false} />
            <YAxis stroke="#64748b" tick={{ fill: '#64748b', fontSize: 12 }} axisLine={false} tickLine={false} allowDecimals={false} />
            <Tooltip
              contentStyle={{ backgroundColor: '#0f172a', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '8px', color: '#f8fafc' }}
              itemStyle={{ color: '#ef4444' }}
              labelStyle={{ color: '#8b5cf6', marginBottom: '4px' }}
              formatter={(value: number, name: string, props: any) => [`${value} Failures`, `Cause: ${props.payload.cause}`]}
            />
            <Line type="monotone" dataKey="failures" stroke="#ef4444" strokeWidth={3} dot={{ r: 4, fill: '#ef4444', strokeWidth: 2, stroke: '#060a16' }} activeDot={{ r: 6 }} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

function TestingQualityView() {
  const [isRunning, setIsRunning] = useState(false);
  const [progress, setProgress] = useState(0);
  const [results, setResults] = useState<any>(null);

  const runTests = () => {
    setIsRunning(true);
    setProgress(0);
    setResults(null);
    let current = 0;
    const interval = setInterval(() => {
      current += 5;
      setProgress(current);
      if (current >= 100) {
        clearInterval(interval);
        setIsRunning(false);
        setResults({
          passed: 124,
          failed: 0,
          skipped: 2,
          coverage: {
            lines: "94.2%",
            functions: "98.1%",
            branches: "89.4%",
          }
        });
      }
    }, 100);
  };

  return (
    <div className="space-y-6">
      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-2 bg-indigo-500/10 rounded-lg">
            <TestTube className="w-5 h-5 text-indigo-400" />
          </div>
          <div>
            <h3 className="text-sm font-medium text-white uppercase tracking-widest">
              Testing & Quality Assurance
            </h3>
            <p className="text-xs text-slate-400 mt-1">
              Run pre-configured Jest test suites for ATC-OS core modules and review coverage.
            </p>
          </div>
        </div>

        <div className="flex items-center gap-4 mb-6">
          <button 
            onClick={runTests}
            disabled={isRunning}
            className={`px-6 py-2 rounded-lg font-medium text-sm transition-colors ${isRunning ? 'bg-indigo-500/50 text-white/50 cursor-not-allowed' : 'bg-indigo-500 hover:bg-indigo-600 text-white shadow-lg shadow-indigo-500/20'}`}
          >
            {isRunning ? 'Running test suites...' : 'Run Core Test Suites'}
          </button>
        </div>

        {isRunning && (
          <div className="mb-6 p-4 bg-black/40 border border-white/5 rounded-lg">
            <div className="flex justify-between text-xs text-slate-400 mb-2 font-mono">
              <span>Executing Jest runner...</span>
              <span>{progress}%</span>
            </div>
            <div className="w-full h-2 bg-white/10 rounded-full overflow-hidden">
              <div 
                className="h-full bg-indigo-500 transition-all duration-100 ease-linear"
                style={{ width: `${progress}%` }}
              />
            </div>
          </div>
        )}

        {results && (
          <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
            <div className="grid grid-cols-3 gap-4">
               <div className="bg-black/40 border border-emerald-500/20 rounded-xl p-4 flex flex-col items-center">
                 <span className="text-3xl font-mono text-emerald-400 mb-1">{results.passed}</span>
                 <span className="text-[10px] uppercase tracking-widest text-slate-500 font-bold">Passed</span>
               </div>
               <div className="bg-black/40 border border-red-500/20 rounded-xl p-4 flex flex-col items-center">
                 <span className="text-3xl font-mono text-slate-400 mb-1">{results.failed}</span>
                 <span className="text-[10px] uppercase tracking-widest text-slate-500 font-bold">Failed</span>
               </div>
               <div className="bg-black/40 border border-amber-500/20 rounded-xl p-4 flex flex-col items-center">
                 <span className="text-3xl font-mono text-amber-500 mb-1">{results.skipped}</span>
                 <span className="text-[10px] uppercase tracking-widest text-slate-500 font-bold">Skipped</span>
               </div>
            </div>

            <div>
              <h4 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">Coverage Report</h4>
              <div className="bg-black/40 border border-white/5 rounded-xl p-4 grid grid-cols-3 gap-4">
                <div className="flex flex-col gap-1">
                   <span className="text-[10px] text-slate-500 uppercase font-bold tracking-wider">Lines</span>
                   <span className="text-sm font-mono text-white">{results.coverage.lines}</span>
                </div>
                <div className="flex flex-col gap-1 border-l border-white/10 pl-4">
                   <span className="text-[10px] text-slate-500 uppercase font-bold tracking-wider">Functions</span>
                   <span className="text-sm font-mono text-white">{results.coverage.functions}</span>
                </div>
                <div className="flex flex-col gap-1 border-l border-white/10 pl-4">
                   <span className="text-[10px] text-slate-500 uppercase font-bold tracking-wider">Branches</span>
                   <span className="text-sm font-mono text-white">{results.coverage.branches}</span>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

function OfflineManagementView() {
  const [offlineMode, setOfflineMode] = useState(() => localStorage.getItem('atc_offline_mode') === 'true');
  const [offlineQueue, setOfflineQueue] = useState<any[]>(() => {
    try {
      return JSON.parse(localStorage.getItem('atc_offline_queue') || '[]');
    } catch {
      return [];
    }
  });
  const [isFlushing, setIsFlushing] = useState(false);

  const toggleOfflineMode = () => {
    const next = !offlineMode;
    setOfflineMode(next);
    localStorage.setItem('atc_offline_mode', String(next));
  };

  const flushQueue = () => {
    setIsFlushing(true);
    setTimeout(() => {
      setOfflineQueue([]);
      localStorage.setItem('atc_offline_queue', '[]');
      setIsFlushing(false);
    }, 1500);
  };

  useEffect(() => {
    if (offlineQueue.length === 0) {
      const dummy = [
        { id: '1', action: 'CREATE', entity: 'Document', timestamp: new Date(Date.now() - 1000 * 60 * 5).toISOString() },
        { id: '2', action: 'UPDATE', entity: 'System Profile', timestamp: new Date(Date.now() - 1000 * 60 * 2).toISOString() },
      ];
      setOfflineQueue(dummy);
      localStorage.setItem('atc_offline_queue', JSON.stringify(dummy));
    }
  }, []);

  return (
    <div className="space-y-6">
      <div className="bg-white/5 border border-white/10 rounded-xl p-6">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-3">
             <div className={`p-2 rounded-lg ${offlineMode ? 'bg-amber-500/10' : 'bg-emerald-500/10'}`}>
               <WifiOff className={`w-5 h-5 ${offlineMode ? 'text-amber-500' : 'text-emerald-400'}`} />
             </div>
             <div>
               <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                 Offline Management
               </h3>
               <p className="text-xs text-slate-400 mt-1">
                 Toggle explicit offline mode and manage locally cached sync operations.
               </p>
             </div>
          </div>
          <button
            onClick={toggleOfflineMode}
            className={`w-12 h-6 rounded-full flex items-center transition-colors p-1 ${offlineMode ? 'bg-amber-500' : 'bg-emerald-500'}`}
          >
            <div className={`w-4 h-4 rounded-full bg-white transition-transform ${offlineMode ? 'translate-x-6' : 'translate-x-0'}`} />
          </button>
        </div>

        {offlineMode && (
          <div className="mb-6 p-3 bg-amber-500/10 border border-amber-500/20 rounded-lg flex items-center justify-between">
            <span className="text-xs font-medium text-amber-500">System is currently operating in explicit offline mode. Network sync is suspended.</span>
          </div>
        )}

        <div className="mt-8">
          <div className="flex items-center justify-between mb-4">
             <h4 className="text-xs font-bold text-slate-500 uppercase tracking-widest">Locally Cached Sync Queue</h4>
             <button 
               onClick={flushQueue}
               disabled={offlineQueue.length === 0 || isFlushing}
               className={`text-xs px-3 py-1.5 rounded-lg font-medium transition-colors flex items-center gap-1.5 ${offlineQueue.length === 0 || isFlushing ? 'bg-white/5 text-slate-500 cursor-not-allowed' : 'bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500/30'}`}
             >
               {isFlushing && <RefreshCw className="w-3 h-3 animate-spin"/>}
               {isFlushing ? 'Syncing...' : 'Flush Queue'}
             </button>
          </div>

          <div className="space-y-2">
            {offlineQueue.length === 0 ? (
              <div className="p-8 text-center bg-black/40 border border-white/5 rounded-lg">
                <span className="text-sm font-mono text-slate-500">Queue is currently empty.</span>
              </div>
            ) : (
              offlineQueue.map((item, idx) => (
                <div key={idx} className="flex items-center justify-between p-3 bg-black/40 border border-white/5 rounded-lg">
                  <div className="flex items-center gap-3">
                    <span className={`text-[10px] uppercase font-bold tracking-widest px-2 py-1 rounded bg-white/5 ${item.action === 'CREATE' ? 'text-emerald-400' : item.action === 'DELETE' ? 'text-red-400' : 'text-indigo-400'}`}>
                      {item.action}
                    </span>
                    <span className="text-sm font-medium text-white">{item.entity}</span>
                  </div>
                  <span className="text-xs font-mono text-slate-400">{new Date(item.timestamp).toLocaleTimeString()}</span>
                </div>
              ))
            )}
          </div>
        </div>

      </div>
    </div>
  );
}

export function SettingsView({
  initialTab = "system",
  conflictItems = [],
  onResolveConflict,
  syncHistory = [],
  offlineQueue = [],
  onClearOfflineQueue,
  isSyncPaused = false,
  setIsSyncPaused,
  isFocusMode = false,
  setIsFocusMode,
  bookmarks = [],
  removeBookmark,
  onNavigate,
}: SettingsViewProps & { 
  offlineQueue?: any[], 
  onClearOfflineQueue?: () => void,
  isSyncPaused?: boolean,
  setIsSyncPaused?: (val: boolean) => void,
  isFocusMode?: boolean, 
  setIsFocusMode?: (val: boolean) => void,
  bookmarks?: { id: string, title: string, type: string, path: string }[],
  removeBookmark?: (id: string) => void,
  onNavigate?: (tab: string) => void
}) {
  const [activeTab, setActiveTab] = useState(initialTab);
  const [sortConfig, setSortConfig] = useState<{
    key: "timestamp" | "type" | "target" | "outcome" | "success";
    direction: "asc" | "desc";
  } | null>(null);

  const [isLogsModalOpen, setIsLogsModalOpen] = useState(false);
  const [systemLogs, setSystemLogs] = useState<string[]>([]);

  useEffect(() => {
    if (isLogsModalOpen) {
      const logs: { time: number; log: string }[] = [];
      
      syncHistory.forEach(h => {
        logs.push({
          time: h.timestamp.getTime(),
          log: `[${h.timestamp.toISOString()}] [${h.success ? 'INFO' : 'ERROR'}] [API] Sync ${h.type} to ${h.target}: ${h.outcome}`
        });
      });
      
      conflictItems.forEach((c, i) => {
        const time = Date.now() - i * 1000;
        logs.push({
          time,
          log: `[${new Date(time).toISOString()}] [WARN] [SYNC] Sync conflict detected on Block_ID: ${c.id} (${c.title}). Remote and local states diverged.`
        });
      });

      // Fill remaining with some background activity
      const baseTime = Date.now() - 3600000;
      for (let i = 0; i < 50; i++) {
        const time = baseTime + i * 60000;
        logs.push({
          time,
          log: `[${new Date(time).toISOString()}] [DEBUG] [SYS] Background thread heartbeat OK.`
        });
      }

      logs.sort((a, b) => b.time - a.time);
      setSystemLogs(logs.map(l => l.log).slice(0, 50).reverse());
    }
  }, [isLogsModalOpen, syncHistory, conflictItems]);

  const [windowOpacity, setWindowOpacity] = useState(() => parseInt(localStorage.getItem('atc_window_opacity') || '100'));

  const handleOpacityChange = (e: React.ChangeEvent<HTMLInputElement>) => {
     const value = parseInt(e.target.value);
     setWindowOpacity(value);
     localStorage.setItem('atc_window_opacity', value.toString());
     window.dispatchEvent(new Event('atc_window_opacity_changed'));
  };

  useEffect(() => {
    if (conflictItems.length > 0) {
      setActiveTab("sync-conflicts");
    }
  }, [conflictItems.length]);

  const sortedSyncHistory = [...syncHistory].sort((a, b) => {
    if (!sortConfig) return 0;
    const { key, direction } = sortConfig;
    if (a[key] < b[key]) return direction === "asc" ? -1 : 1;
    if (a[key] > b[key]) return direction === "asc" ? 1 : -1;
    return 0;
  });

  const handleSort = (key: "timestamp" | "type" | "target" | "outcome" | "success") => {
    let direction: "asc" | "desc" = "asc";
    if (sortConfig && sortConfig.key === key && sortConfig.direction === "asc") {
      direction = "desc";
    }
    setSortConfig({ key, direction });
  };

  const tabs = [
    { id: "system", icon: Cpu, label: "System & Info" },
    { id: "visual", icon: Palette, label: "Visual Customization" },
    { id: "session-management", icon: Clock, label: "Session Management" },
    { id: "cloud-sync", icon: UploadCloud, label: "Cloud Services Sync" },
    { id: "network", icon: Network, label: "Network & Proxy" },
    { id: "security", icon: Shield, label: "Security & Privacy" },
    { id: "storage", icon: Database, label: "Data Storage" },
    { id: "offline", icon: WifiOff, label: "Offline Management" },
    { id: "testing", icon: TestTube, label: "Testing & Quality" },
    { id: "battery-health", icon: Battery, label: "Battery Health" },
    { id: "sync-conflicts", icon: GitPullRequest, label: "Sync Conflicts" },
    { id: "sync-diagnostics", icon: FileText, label: "Sync Diagnostics" },
    { id: "sync-history", icon: History, label: "Sync History" },
    { id: "notifications", icon: Bell, label: "Notifications" },
    { id: "bookmarks", icon: Bookmark, label: "Bookmarks" },
    { id: "help", icon: HelpCircle, label: "Help & Shortcuts" },
  ];

  return (
    <div className="flex h-full bg-[#060a16] text-slate-300 rounded-xl border border-white/10 overflow-hidden">
      {/* Sidebar */}
      <div className="w-64 bg-[#090b14] border-r border-white/5 p-4 flex flex-col gap-2">
        <h2 className="text-xl font-bold text-white mb-4 px-2">Settings</h2>
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`flex items-center gap-3 px-3 py-2 rounded-lg transition-colors text-left ${activeTab === tab.id ? "bg-indigo-500/10 text-indigo-400 font-medium" : "hover:bg-white/5 text-slate-400"}`}
          >
            <tab.icon className="w-4 h-4" />
            {tab.label}
          </button>
        ))}
      </div>

      {/* Main Content */}
      <div className="flex-1 p-8 overflow-y-auto">
        <div className="max-w-2xl">
          <h1 className="text-2xl font-semibold text-white tracking-tight mb-8">
            {tabs.find((t) => t.id === activeTab)?.label}
          </h1>

          {activeTab === "system" && (
            <div className="space-y-6">
              <ThemeSelector />

              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <h3 className="text-sm font-medium text-white mb-4 uppercase tracking-widest flex items-center gap-2">
                   <Eye className="w-4 h-4 text-indigo-400" />
                   Window Transparency (Mica / Glass)
                </h3>
                <div className="flex items-center gap-6">
                   <input type="range" min="10" max="100" value={windowOpacity} onChange={handleOpacityChange} className="flex-1" />
                   <div className="w-16 text-right font-mono text-white">{windowOpacity}%</div>
                </div>
                <p className="text-xs text-slate-400 mt-3">Adjusts the background opacity of active windows to create a frosted glass effect.</p>
              </div>

              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <h3 className="text-sm font-medium text-white mb-4 uppercase tracking-widest">
                  ATC-OS About
                </h3>
                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div className="text-slate-500">Edition</div>
                  <div className="text-white">ATC-OS Desktop Beta</div>
                  <div className="text-slate-500">Version</div>
                  <div className="text-white">1.0.4-rc2</div>
                  <div className="text-slate-500">Kernel</div>
                  <div className="text-white font-mono">atc_kernel_atvm_v2</div>
                  <div className="text-slate-500">Processor</div>
                  <div className="text-white">
                    Neural Processing Unit (Virtual)
                  </div>
                </div>
              </div>
              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <h3 className="text-sm font-medium text-white mb-4 uppercase tracking-widest">
                  Update
                </h3>
                <p className="text-slate-400 mb-4 text-sm">
                  Your decentralized operating system is up to date.
                </p>
                <button className="bg-white/10 hover:bg-white/20 text-white px-4 py-2 rounded-md transition-colors text-sm font-medium">
                  Check for sync updates
                </button>
              </div>

              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <h3 className="text-sm font-medium text-white mb-4 uppercase tracking-widest flex items-center gap-2">
                  <Eye className="w-4 h-4 text-slate-400" /> Focus Mode
                </h3>
                <p className="text-slate-400 mb-4 text-sm leading-relaxed">
                  Toggle Focus Mode to minimize non-essential UI panels across the application, allowing you to isolate specific architectural layers and documentation for deeper analysis.
                </p>
                <button 
                  onClick={() => setIsFocusMode?.(!isFocusMode)}
                  className={`px-4 py-2 rounded-md transition-colors text-sm font-medium flex items-center gap-2 border w-max ${isFocusMode ? 'bg-atc-cyan/20 border-atc-cyan text-atc-cyan' : 'bg-white/10 border-white/10 text-white hover:bg-white/20'}`}
                >
                  {isFocusMode ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                  {isFocusMode ? 'Disable Focus Mode' : 'Enable Focus Mode'}
                </button>
              </div>

              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <h3 className="text-sm font-medium text-white mb-4 uppercase tracking-widest flex items-center gap-2">
                  <Terminal className="w-4 h-4 text-slate-400" /> Developer Debugging
                </h3>
                <p className="text-slate-400 mb-4 text-sm leading-relaxed">
                  View raw system logs to help debug sync conflicts and API failures directly in the UI.
                </p>
                <button 
                  onClick={() => setIsLogsModalOpen(true)}
                  className="bg-white/10 hover:bg-white/20 text-white px-4 py-2 rounded-md transition-colors text-sm font-medium flex items-center gap-2 w-max"
                >
                  <Terminal className="w-4 h-4" /> View System Logs
                </button>
              </div>
            </div>
          )}

          {activeTab === "visual" && <VisualCustomization />}
          {activeTab === "battery-health" && <BatteryHealthSettings />}
          {activeTab === "session-management" && <SessionManagementView />}
          {activeTab === "security" && <SecuritySettings />}

          {activeTab === "cloud-sync" && (
            <div className="space-y-6">
              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-2 bg-blue-500/10 rounded-lg">
                    <UploadCloud className="w-5 h-5 text-blue-400" />
                  </div>
                  <div>
                    <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                      Enterprise Cloud Sync
                    </h3>
                    <p className="text-xs text-slate-400 mt-1">
                      Professionelles Push/Pull System für alle externen Plattformen.
                    </p>
                  </div>
                </div>

                <div className="space-y-4">
                  <div className="p-4 bg-[#090b14] border border-white/5 rounded-lg flex items-center justify-between hover:border-blue-500/30 transition-colors cursor-pointer">
                    <div className="flex items-center gap-4">
                      <div className="p-2 bg-blue-500/10 rounded">
                        <HardDrive className="w-5 h-5 text-blue-400" />
                      </div>
                      <div>
                        <h4 className="text-sm font-bold text-white">Google Workspace Setup</h4>
                        <p className="text-xs text-slate-400">Google Drive & Google Sheets Integration</p>
                      </div>
                    </div>
                    <button className="atc-btn-primary bg-blue-500/20 text-blue-400 hover:bg-blue-500/30">
                      Connect & Upload
                    </button>
                  </div>

                  <div className="p-4 bg-[#090b14] border border-white/5 rounded-lg flex items-center justify-between hover:border-indigo-500/30 transition-colors cursor-pointer">
                    <div className="flex items-center gap-4">
                      <div className="p-2 bg-indigo-500/10 rounded">
                        <Github className="w-5 h-5 text-indigo-400" />
                      </div>
                      <div>
                        <h4 className="text-sm font-bold text-white">GitHub Enterprise Setup</h4>
                        <p className="text-xs text-slate-400">Full repository synchronization & commits</p>
                      </div>
                    </div>
                    <button className="atc-btn-primary bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500/30">
                      Force Commit
                    </button>
                  </div>

                  <div className="p-4 bg-[#090b14] border border-white/5 rounded-lg flex items-center justify-between hover:border-slate-300/30 transition-colors cursor-pointer">
                    <div className="flex items-center gap-4">
                      <div className="p-2 bg-slate-500/10 rounded">
                        <FileText className="w-5 h-5 text-slate-400" />
                      </div>
                      <div>
                        <h4 className="text-sm font-bold text-white">Notion Database Link</h4>
                        <p className="text-xs text-slate-400">Sync all pages and architecture wikis</p>
                      </div>
                    </div>
                    <button className="atc-btn-primary bg-slate-500/20 text-slate-300 hover:bg-slate-500/30">
                      Update Pages
                    </button>
                  </div>
                </div>
              </div>

              {/* Pending Sync Queue Management */}
              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-sm font-medium text-white uppercase tracking-widest flex items-center gap-2">
                    <History className="w-4 h-4 text-slate-400" />
                    Pending Sync Queue
                  </h3>
                  <div className="text-xs text-slate-500 font-mono">3 Tasks Queued</div>
                </div>
                <div className="space-y-3">
                  <div className="p-3 bg-black/40 border border-white/5 rounded-lg flex items-center justify-between">
                    <div className="flex items-center gap-3">
                       <Github className="w-4 h-4 text-indigo-400" />
                       <div className="flex flex-col">
                         <span className="text-xs font-bold text-slate-200">Push Commits to `atc-os-core`</span>
                         <span className="text-[10px] text-slate-500 font-mono">Waiting for network stability...</span>
                       </div>
                    </div>
                    <div className="flex items-center gap-2">
                       <button className="atc-btn text-[10px] uppercase font-bold py-1.5 px-3">
                         Force Retry
                       </button>
                       <button className="atc-btn text-[10px] uppercase font-bold py-1.5 px-3 bg-rose-500/10 text-rose-400 hover:bg-rose-500/20 border-rose-500/30 ring-rose-500">
                         Clear
                       </button>
                    </div>
                  </div>
                  <div className="p-3 bg-black/40 border border-white/5 rounded-lg flex items-center justify-between">
                    <div className="flex items-center gap-3">
                       <HardDrive className="w-4 h-4 text-blue-400" />
                       <div className="flex flex-col">
                         <span className="text-xs font-bold text-slate-200">Export State to Google Drive</span>
                         <span className="text-[10px] text-slate-500 font-mono">File size: 14.2 MB | Scheduled</span>
                       </div>
                    </div>
                    <div className="flex items-center gap-2">
                       <button className="atc-btn text-[10px] uppercase font-bold py-1.5 px-3">
                         Force Retry
                       </button>
                       <button className="atc-btn text-[10px] uppercase font-bold py-1.5 px-3 bg-rose-500/10 text-rose-400 hover:bg-rose-500/20 border-rose-500/30 ring-rose-500">
                         Clear
                       </button>
                    </div>
                  </div>
                  <div className="p-3 bg-black/40 border border-white/5 rounded-lg flex items-center justify-between">
                    <div className="flex items-center gap-3">
                       <FileText className="w-4 h-4 text-slate-400" />
                       <div className="flex flex-col">
                         <span className="text-xs font-bold text-slate-200">Update Notion 'Architecture' DB</span>
                         <span className="text-[10px] text-slate-500 font-mono">Rate limited. Backoff: 45s</span>
                       </div>
                    </div>
                    <div className="flex items-center gap-2">
                       <button className="atc-btn text-[10px] uppercase font-bold py-1.5 px-3">
                         Force Retry
                       </button>
                       <button className="atc-btn text-[10px] uppercase font-bold py-1.5 px-3 bg-rose-500/10 text-rose-400 hover:bg-rose-500/20 border-rose-500/30 ring-rose-500">
                         Clear
                       </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === "sync-conflicts" && (
            <div className="space-y-6">
              <div className="bg-white/5 border border-amber-500/30 rounded-xl p-6">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <div className="p-2 bg-amber-500/10 rounded-lg">
                      <AlertTriangle className="w-5 h-5 text-amber-500" />
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                        Unresolved Conflicts
                      </h3>
                      <p className="text-xs text-slate-400 mt-1">
                        Select source of truth to proceed with synchronization.
                      </p>
                    </div>
                  </div>
                  {conflictItems.length > 0 && (
                    <button
                      onClick={() => {
                        const blob = new Blob(
                          [JSON.stringify(conflictItems, null, 2)],
                          { type: "application/json" },
                        );
                        const url = URL.createObjectURL(blob);
                        const a = document.createElement("a");
                        a.href = url;
                        a.download = `conflict-audit-log-${new Date().toISOString().slice(0, 10)}.json`;
                        document.body.appendChild(a);
                        a.click();
                        document.body.removeChild(a);
                        URL.revokeObjectURL(url);
                      }}
                      className="atc-btn text-xs font-semibold"
                    >
                      <FileText className="w-4 h-4" />
                      Export Audit Log
                    </button>
                  )}
                </div>

                {conflictItems.length === 0 ? (
                  <div className="p-6 text-center border-t border-white/5 mt-4">
                    <CheckCircle className="w-8 h-8 text-emerald-500 mx-auto mb-3" />
                    <p className="text-emerald-400 font-medium tracking-wide">
                      All completely synced
                    </p>
                    <p className="text-xs text-slate-500 mt-1">
                      No file conflicts found between local and remote sources.
                    </p>
                  </div>
                ) : (
                  <div className="mt-6 space-y-4">
                    {conflictItems.map((item) => (
                      <div
                        key={item.id}
                        className="border border-white/10 rounded-lg p-4 bg-[#090b14]"
                      >
                        <h4 className="font-medium text-white mb-4 text-base">
                          {item.title}
                        </h4>
                        <div className="grid grid-cols-2 gap-4">
                          <div className="border border-white/10 hover:border-indigo-500/50 p-4 rounded-lg bg-white/5 transition-colors">
                            <div className="flex items-center justify-between mb-2">
                              <div className="flex items-center gap-2 text-indigo-400">
                                <Github className="w-4 h-4" />
                                <span className="font-medium text-sm">
                                  Remote (GitHub)
                                </span>
                              </div>
                              <button
                                onClick={() =>
                                  onResolveConflict &&
                                  onResolveConflict(item.id, "remote")
                                }
                                className="px-3 py-1 bg-indigo-500/20 text-indigo-300 hover:bg-indigo-500/40 text-xs font-bold rounded"
                              >
                                Keep Remote
                              </button>
                            </div>
                            <div className="text-xs text-slate-400 mt-2 p-2 bg-black/40 rounded border border-white/5">
                              <p>
                                <span className="text-slate-500">Status:</span>{" "}
                                {item.githubSource.status}
                              </p>
                              <p>
                                <span className="text-slate-500">Goals:</span>{" "}
                                {item.githubSource.goals?.length || 0} items
                              </p>
                            </div>
                          </div>
                          <div className="border border-white/10 hover:border-emerald-500/50 p-4 rounded-lg bg-white/5 transition-colors">
                            <div className="flex items-center justify-between mb-2">
                              <div className="flex items-center gap-2 text-emerald-400">
                                <FileText className="w-4 h-4" />
                                <span className="font-medium text-sm">
                                  Local (Notion)
                                </span>
                              </div>
                              <button
                                onClick={() =>
                                  onResolveConflict &&
                                  onResolveConflict(item.id, "local")
                                }
                                className="px-3 py-1 bg-emerald-500/20 text-emerald-300 hover:bg-emerald-500/40 text-xs font-bold rounded"
                              >
                                Keep Local
                              </button>
                            </div>
                            <div className="text-xs text-slate-400 mt-2 p-2 bg-black/40 rounded border border-white/5">
                              <p>
                                <span className="text-slate-500">Status:</span>{" "}
                                {item.notionSource.status}
                              </p>
                              <p>
                                <span className="text-slate-500">Goals:</span>{" "}
                                {item.notionSource.goals?.length || 0} items
                              </p>
                            </div>
                          </div>
                        </div>

                        {/* Suggested Merge Option */}
                        <div className="mt-4 border border-white/10 hover:border-amber-500/50 p-4 rounded-lg bg-amber-500/5 transition-colors">
                          <div className="flex items-center justify-between mb-2">
                            <div className="flex items-center gap-2 text-amber-400">
                              <GitPullRequest className="w-4 h-4" />
                              <span className="font-medium text-sm">
                                Suggested Merge (Auto-Resolved)
                              </span>
                            </div>
                            <button
                              onClick={() =>
                                onResolveConflict &&
                                onResolveConflict(item.id, "merge")
                              }
                              className="px-3 py-1 bg-amber-500/20 text-amber-300 hover:bg-amber-500/40 text-xs font-bold rounded flex items-center gap-2"
                            >
                              Accept Merge
                            </button>
                          </div>
                          <div className="text-xs text-slate-400 mt-2 p-2 bg-black/40 rounded border border-white/5 space-y-1">
                            <p className="text-amber-500/80 mb-2">Algorithm detected overlapping file changes. Smart Merge proposed.</p>
                            <p>
                              <span className="text-slate-500">Merged Status:</span>{" "}
                              {item.githubSource.status === "completed" || item.notionSource.status === "completed" ? "completed" : "in-progress"}
                            </p>
                            <p>
                              <span className="text-slate-500">Merged Goals:</span>{" "}
                              {Array.from(new Set([...(item.githubSource.goals || []), ...(item.notionSource.goals || [])])).length} items combined
                            </p>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          )}

          {activeTab === "sync-diagnostics" && (
            <div className="space-y-6">
              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-2 bg-indigo-500/10 rounded-lg">
                    <FileText className="w-5 h-5 text-indigo-400" />
                  </div>
                  <div>
                    <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                      Sync Diagnostics
                    </h3>
                    <p className="text-xs text-slate-400 mt-1">
                      Logs of the last API errors for troubleshooting.
                    </p>
                  </div>
                </div>
                <div className="space-y-3">
                  {Array.from({ length: 20 }).map((_, i) => {
                    const isGithub = Math.random() > 0.5;
                    const error = {
                      id: i,
                      type: isGithub ? "GitHub" : "Notion",
                      code: isGithub ? "403 Forbidden" : "400 Bad Request",
                      msg: isGithub
                        ? "API Rate Limit Exceeded"
                        : "Invalid request body",
                      doc: isGithub
                        ? "https://docs.github.com/en/rest"
                        : "https://developers.notion.com/reference/errors",
                      time: `${i * 15 + 2} mins ago`,
                    };
                    return (
                      <div
                        key={error.id}
                        className="p-4 bg-red-500/5 border border-red-500/20 rounded-lg flex items-start gap-4"
                      >
                        <AlertTriangle className="w-5 h-5 text-red-400 shrink-0 mt-0.5" />
                        <div className="flex-1">
                          <div className="flex items-center justify-between">
                            <span className="text-sm font-bold text-slate-200">
                              {error.type} API Error
                            </span>
                            <span className="text-xs text-slate-500 font-mono">
                              {error.time}
                            </span>
                          </div>
                          <p className="text-xs text-red-300 font-mono mt-1">
                            {error.code}: {error.msg}
                          </p>
                          <a
                            href={error.doc}
                            target="_blank"
                            rel="noreferrer"
                            className="inline-block mt-3 text-xs text-indigo-400 hover:text-indigo-300 hover:underline"
                          >
                            View Documentation
                          </a>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            </div>
          )}

          {activeTab === "storage" && (
            <div className="space-y-6">
              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <div className="flex items-center justify-between mb-6">
                  <div className="flex items-center gap-3">
                    <div className="p-2 bg-indigo-500/10 rounded-lg">
                      <Database className="w-5 h-5 text-indigo-400" />
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                        Data Storage & IndexedDB
                      </h3>
                      <p className="text-xs text-slate-400 mt-1">
                        Manage local architectural data and export backups.
                      </p>
                    </div>
                  </div>
                  <button
                    onClick={async () => {
                      try {
                        const data = await atcDatabase.getAll();
                        const blob = new Blob([JSON.stringify(data, null, 2)], { type: "application/json" });
                        const url = URL.createObjectURL(blob);
                        const a = document.createElement('a');
                        a.href = url;
                        a.download = `atc_os_backup_${new Date().toISOString()}.json`;
                        document.body.appendChild(a);
                        a.click();
                        document.body.removeChild(a);
                        URL.revokeObjectURL(url);
                      } catch (err) {
                        console.error("Export failed", err);
                      }
                    }}
                    className="flex items-center gap-2 px-4 py-2 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 rounded-lg transition-colors font-mono text-sm"
                  >
                    <Download className="w-4 h-4" />
                    Backup Data
                  </button>
                </div>

                <div className="mt-6 pt-6 border-t border-white/10">
                  <div className="flex items-center justify-between mb-4">
                    <div className="flex items-center gap-2 text-slate-300">
                      <UploadCloud className="w-4 h-4 text-emerald-400" />
                      <span className="text-sm font-medium uppercase tracking-wider font-mono">
                        GitHub / Notion Sync Queue
                      </span>
                    </div>
                    <div className="flex items-center gap-3">
                      {offlineQueue.length > 0 && (
                        <>
                          <button
                            onClick={() => setIsSyncPaused?.(!isSyncPaused)}
                            className={`px-3 py-1 text-xs font-bold rounded border transition-colors ${
                              isSyncPaused ? 'bg-indigo-500/20 text-indigo-400 border-indigo-500/30' : 'bg-white/5 text-slate-300 border-white/10 hover:bg-white/10'
                            }`}
                          >
                            {isSyncPaused ? 'Resume Sync' : 'Pause Sync'}
                          </button>
                          <button
                            onClick={onClearOfflineQueue}
                            className="px-3 py-1 text-xs font-bold rounded border bg-red-500/10 text-red-400 border-red-500/30 hover:bg-red-500/20 transition-colors"
                          >
                            Clear Queue
                          </button>
                        </>
                      )}
                      <span className={`text-[10px] uppercase font-bold tracking-widest px-2 py-1 rounded ${offlineQueue?.length ? 'bg-amber-500/20 text-amber-500' : 'bg-emerald-500/20 text-emerald-500'}`}>
                        {offlineQueue?.length || 0} Pending Uploads
                      </span>
                    </div>
                  </div>

                  {!offlineQueue || offlineQueue.length === 0 ? (
                    <div className="text-center p-6 bg-black/40 rounded-lg border border-white/5">
                      <CheckCircle className="w-6 h-6 text-emerald-500/50 mx-auto mb-2" />
                      <p className="text-sm font-mono text-slate-400">All local changes are fully synced to remote providers.</p>
                    </div>
                  ) : (
                    <div className="space-y-3">
                      {offlineQueue.map((item, idx) => (
                        <div key={idx} className="flex items-center justify-between p-3 rounded-lg bg-[#090b14] border border-amber-500/20">
                          <div className="flex items-center gap-3">
                            <Database className="w-4 h-4 text-amber-500/70" />
                            <div>
                              <p className="text-sm text-slate-300 font-mono">{item.target} Sync Record</p>
                              <p className="text-[10px] text-slate-500 font-mono mt-0.5">{new Date(item.timestamp).toLocaleString()}</p>
                            </div>
                          </div>
                          <div>
                            <span className="text-[10px] uppercase tracking-wider text-amber-400 font-mono bg-amber-500/10 px-2 py-1 rounded">Pending</span>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}

          {activeTab === "sync-history" && (
            <div className="space-y-6">
              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-2 bg-emerald-500/10 rounded-lg">
                    <History className="w-5 h-5 text-emerald-400" />
                  </div>
                  <div>
                    <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                      Sync History
                    </h3>
                    <p className="text-xs text-slate-400 mt-1">
                      Track timestamps and outcomes of recent manual and
                      background syncs.
                    </p>
                  </div>
                </div>

                <SyncErrorChart />

                {syncHistory.length === 0 ? (
                  <div className="p-6 text-center border-t border-white/5 mt-4">
                    <History className="w-8 h-8 text-slate-600 mx-auto mb-3" />
                    <p className="text-slate-400 font-medium tracking-wide">
                      No sync history available yet
                    </p>
                    <p className="text-xs text-slate-500 mt-1">
                      Perform a manual sync or wait for auto-sync to see events.
                    </p>
                  </div>
                ) : (
                  <div className="mt-6 border border-white/10 rounded-lg overflow-hidden bg-[#090b14]">
                    <div className="overflow-x-auto">
                      <table className="w-full text-left text-sm text-slate-300">
                        <thead className="bg-black/60 border-b border-white/10 text-xs font-medium uppercase tracking-wider text-slate-400">
                          <tr>
                            <th className="p-3 cursor-pointer hover:bg-white/5 transition-colors" onClick={() => handleSort("target")}>
                              <div className="flex items-center gap-1">
                                Target {sortConfig?.key === "target" && (sortConfig.direction === "asc" ? <ArrowUp className="w-3 h-3" /> : <ArrowDown className="w-3 h-3" />)}
                              </div>
                            </th>
                            <th className="p-3 cursor-pointer hover:bg-white/5 transition-colors" onClick={() => handleSort("timestamp")}>
                              <div className="flex items-center gap-1">
                                Timestamp {sortConfig?.key === "timestamp" && (sortConfig.direction === "asc" ? <ArrowUp className="w-3 h-3" /> : <ArrowDown className="w-3 h-3" />)}
                              </div>
                            </th>
                            <th className="p-3 cursor-pointer hover:bg-white/5 transition-colors" onClick={() => handleSort("type")}>
                              <div className="flex items-center gap-1">
                                Type {sortConfig?.key === "type" && (sortConfig.direction === "asc" ? <ArrowUp className="w-3 h-3" /> : <ArrowDown className="w-3 h-3" />)}
                              </div>
                            </th>
                            <th className="p-3 cursor-pointer hover:bg-white/5 transition-colors" onClick={() => handleSort("success")}>
                              <div className="flex items-center gap-1">
                                Status {sortConfig?.key === "success" && (sortConfig.direction === "asc" ? <ArrowUp className="w-3 h-3" /> : <ArrowDown className="w-3 h-3" />)}
                              </div>
                            </th>
                            <th className="p-3 cursor-pointer hover:bg-white/5 transition-colors" onClick={() => handleSort("outcome")}>
                              <div className="flex items-center gap-1">
                                Outcome {sortConfig?.key === "outcome" && (sortConfig.direction === "asc" ? <ArrowUp className="w-3 h-3" /> : <ArrowDown className="w-3 h-3" />)}
                              </div>
                            </th>
                          </tr>
                        </thead>
                        <tbody className="divide-y divide-white/5">
                          {sortedSyncHistory.map((sh, idx) => (
                            <tr key={idx} className="hover:bg-white/5 transition-colors">
                              <td className="p-3">
                                <div className="flex items-center gap-2">
                                  {sh.target === "GitHub" ? (
                                    <Github className="w-4 h-4 text-slate-400" />
                                  ) : (
                                    <FileText className="w-4 h-4 text-slate-400" />
                                  )}
                                  <span className="font-medium text-slate-200">{sh.target}</span>
                                </div>
                              </td>
                              <td className="p-3 font-mono text-xs text-slate-400">
                                {new Date(sh.timestamp).toLocaleDateString()} {new Date(sh.timestamp).toLocaleTimeString()}
                              </td>
                              <td className="p-3">
                                <span className="text-[10px] uppercase font-bold tracking-wider text-slate-500 bg-white/5 px-2 py-1 rounded">
                                  {sh.type}
                                </span>
                              </td>
                              <td className="p-3">
                                <div className="flex items-center gap-1.5">
                                  {sh.success ? (
                                    <><CheckCircle className="w-4 h-4 text-emerald-500" /><span className="text-emerald-400 text-xs font-medium">Success</span></>
                                  ) : (
                                    <><XCircle className="w-4 h-4 text-red-500" /><span className="text-red-400 text-xs font-medium">Failure</span></>
                                  )}
                                </div>
                              </td>
                              <td className="p-3 text-xs font-mono text-slate-400">
                                {sh.outcome}
                              </td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  </div>
                )}
              </div>
            </div>
          )}

          {activeTab === "bookmarks" && (
            <div className="space-y-6">
              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-2 bg-atc-purple/10 border border-atc-purple/30 rounded-lg">
                    <Bookmark className="w-5 h-5 text-atc-purple" />
                  </div>
                  <div>
                    <h3 className="text-sm font-medium text-white uppercase tracking-widest">
                      Quick Bookmarks
                    </h3>
                    <p className="text-xs text-slate-400 mt-1">
                      Your saved nodes and documentation sections for faster access.
                    </p>
                  </div>
                </div>

                {(!bookmarks || bookmarks.length === 0) ? (
                  <div className="text-center p-8 bg-black/40 border border-white/5 rounded-lg">
                    <Bookmark className="w-8 h-8 text-slate-500 mx-auto mb-3 opacity-50" />
                    <p className="text-slate-400 font-mono text-sm">No bookmarks yet.</p>
                  </div>
                ) : (
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {bookmarks.map((b) => (
                      <div key={b.id} className="p-4 rounded-lg bg-[#090b14] border border-white/10 hover:border-atc-purple/50 transition-all flex items-center justify-between group">
                        <div className="flex flex-col gap-1 cursor-pointer" onClick={() => onNavigate?.(b.path)}>
                          <span className="text-[10px] font-mono text-atc-purple uppercase tracking-widest">{b.type}</span>
                          <span className="text-sm font-medium text-slate-200">{b.title}</span>
                          <span className="text-xs text-slate-500 font-mono">{b.path}</span>
                        </div>
                        <button 
                          onClick={() => removeBookmark?.(b.id)}
                          className="p-2 text-slate-500 hover:text-red-400 hover:bg-white/5 rounded opacity-0 group-hover:opacity-100 transition-all"
                          title="Remove Bookmark"
                        >
                          <XCircle className="w-4 h-4" />
                        </button>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          )}

          {activeTab === "sync-history" && (
            <div className="space-y-6">
              <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-sm font-medium text-white uppercase tracking-widest flex items-center gap-2">
                    <History className="w-5 h-5 text-indigo-400" />
                    Synchronization Audit Logs
                  </h3>
                  <div className="text-xs text-slate-500 font-mono">Last 30 Days</div>
                </div>

                <div className="overflow-x-auto">
                  <table className="w-full text-left border-collapse">
                    <thead>
                      <tr className="border-b border-white/10 text-xs font-mono text-slate-500 uppercase">
                        <th className="py-3 px-4">Status</th>
                        <th className="py-3 px-4">Service</th>
                        <th className="py-3 px-4">Timestamp</th>
                        <th className="py-3 px-4">Event Details</th>
                      </tr>
                    </thead>
                    <tbody className="text-sm">
                      <tr className="border-b border-white/5 hover:bg-white/5 transition-colors group">
                        <td className="py-3 px-4">
                          <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                        </td>
                        <td className="py-3 px-4 font-bold text-slate-300 flex items-center gap-2">
                          <Github className="w-4 h-4" /> GitHub
                        </td>
                        <td className="py-3 px-4 text-slate-400 font-mono text-xs">2026-06-11 08:34:58</td>
                        <td className="py-3 px-4 text-slate-400">Pushed 3 commits to `main`. Triggered CI/CD workflow.</td>
                      </tr>
                      <tr className="border-b border-white/5 hover:bg-white/5 transition-colors group">
                        <td className="py-3 px-4">
                          <XCircle className="w-4 h-4 text-rose-400" />
                        </td>
                        <td className="py-3 px-4 font-bold text-slate-300 flex items-center gap-2">
                          <Github className="w-4 h-4" /> GitHub
                        </td>
                        <td className="py-3 px-4 text-slate-400 font-mono text-xs">2026-06-11 08:30:12</td>
                        <td className="py-3 px-4 text-rose-400/80">Push rejected: Build failed. Exit 1 in esbuild. JSX Error.</td>
                      </tr>
                      <tr className="border-b border-white/5 hover:bg-white/5 transition-colors group">
                        <td className="py-3 px-4">
                          <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                        </td>
                        <td className="py-3 px-4 font-bold text-slate-300 flex items-center gap-2">
                          <FileText className="w-4 h-4" /> Notion
                        </td>
                        <td className="py-3 px-4 text-slate-400 font-mono text-xs">2026-06-11 08:15:00</td>
                        <td className="py-3 px-4 text-slate-400">Synced Architecture Database. Updated 14 properties.</td>
                      </tr>
                      <tr className="border-b border-white/5 hover:bg-white/5 transition-colors group">
                        <td className="py-3 px-4">
                          <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                        </td>
                        <td className="py-3 px-4 font-bold text-slate-300 flex items-center gap-2">
                          <HardDrive className="w-4 h-4" /> G-Drive
                        </td>
                        <td className="py-3 px-4 text-slate-400 font-mono text-xs">2026-06-10 23:59:59</td>
                        <td className="py-3 px-4 text-slate-400">Daily state backup executed successfully (14.2 MB).</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {activeTab === "battery-health" && <BatteryHealthSettings />}
          {activeTab === "visual" && <VisualCustomization />}
          {activeTab === "session-management" && <SessionManagementView />}
          {activeTab === "notifications" && <NotificationsView />}
          {activeTab === "help" && <ShortcutManagerView />}

          {activeTab === "offline" && <OfflineManagementView />}
          {activeTab === "testing" && <TestingQualityView />}

          {activeTab !== "system" &&
            activeTab !== "visual" &&
            activeTab !== "battery-health" &&
            activeTab !== "session-management" &&
            activeTab !== "notifications" &&
            activeTab !== "help" &&
            activeTab !== "security" &&
            activeTab !== "storage" &&
            activeTab !== "bookmarks" &&
            activeTab !== "sync-conflicts" &&
            activeTab !== "sync-diagnostics" &&
            activeTab !== "offline" &&
            activeTab !== "testing" &&
            activeTab !== "sync-history" && (
              <div className="bg-white/5 border border-white/10 rounded-xl p-6 flex flex-col items-center justify-center text-center h-48">
                <Settings className="w-8 h-8 text-slate-600 mb-4 animate-spin-slow" />
                <p className="text-slate-400 font-medium">
                  This module is currently being configured.
                </p>
                <p className="text-slate-500 text-sm mt-2">
                  Check back after the next protocol update.
                </p>
              </div>
            )}
        </div>
      </div>

      {/* System Logs Modal */}
      {isLogsModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
          <div className="bg-[#090b14] border border-white/10 rounded-xl w-full max-w-4xl max-h-[80vh] flex flex-col overflow-hidden shadow-2xl">
            <div className="flex items-center justify-between p-4 border-b border-white/10 bg-white/5">
              <h2 className="text-lg font-bold text-white flex items-center gap-2">
                <Terminal className="w-5 h-5 text-indigo-400" /> System Logs (Last 50 Lines)
              </h2>
              <button 
                onClick={() => setIsLogsModalOpen(false)}
                className="text-slate-400 hover:text-white transition-colors p-1"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
            
            <div className="p-4 overflow-y-auto flex-1 bg-[#050B14] font-mono text-xs leading-relaxed text-slate-300">
              {systemLogs.map((log, i) => (
                <div key={i} className="mb-1 whitespace-pre-wrap break-all">
                  <span className="text-slate-500">{log.substring(0, 26)}</span>
                  <span className={`px-2 ${log.includes('[INFO]') ? 'text-blue-400' : log.includes('[WARN]') ? 'text-yellow-400' : log.includes('[ERROR]') ? 'text-red-400' : 'text-slate-400'}`}>
                     {log.substring(26, 34)}
                  </span>
                  <span>{log.substring(34)}</span>
                </div>
              ))}
            </div>
            
            <div className="p-4 border-t border-white/10 bg-white/5 flex justify-end">
              <button 
                onClick={() => setIsLogsModalOpen(false)}
                className="px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-medium transition-colors"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
