import React, { useState } from 'react';
import { Usb, HardDrive, Wrench, ShieldAlert, Cpu, Download, Activity, FileSpreadsheet, Play, AlertTriangle } from 'lucide-react';
import { motion } from 'motion/react';

export function RescueSystemView() {
  const [activeTab, setActiveTab] = useState('boot');
  const [isProcessing, setIsProcessing] = useState(false);
  const [progress, setProgress] = useState(0);
  const [processMessage, setProcessMessage] = useState('');

  const handleAction = (message: string, duration: number) => {
    setIsProcessing(true);
    setProcessMessage(message);
    setProgress(0);

    const interval = setInterval(() => {
      setProgress(prev => {
        if (prev >= 100) {
          clearInterval(interval);
          setTimeout(() => {
            setIsProcessing(false);
            setProcessMessage('Vorgang erfolgreich abgeschlossen.');
          }, 500);
          return 100;
        }
        return prev + (100 / (duration / 50));
      });
    }, 50);
  };

  const tabs = [
    { id: 'boot', label: 'USB Boot Media', icon: Usb },
    { id: 'disk', label: 'Disk Management', icon: HardDrive },
    { id: 'diag', label: 'System Diagnostics', icon: Activity },
    { id: 'repair', label: 'System Repair', icon: Wrench },
    { id: 'install', label: 'Software Setup', icon: Download },
  ];

  return (
    <div className="flex h-full bg-[#050811] text-slate-200">
      {/* Sidebar */}
      <div className="w-64 border-r border-red-900/30 flex flex-col bg-black/40">
        <div className="p-4 border-b border-red-900/30">
          <div className="flex items-center gap-3 text-red-500">
            <ShieldAlert className="w-6 h-6" />
            <h2 className="font-bold tracking-tight">Rescue Tools</h2>
          </div>
        </div>
        <div className="flex-1 p-3 flex flex-col gap-2 overflow-y-auto">
          {tabs.map(tab => {
            const Icon = tab.icon;
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex items-center gap-3 p-3 rounded-lg transition-all text-sm font-medium ${
                  isActive 
                    ? 'bg-red-500/10 text-red-400 border border-red-500/30 shadow-[0_0_15px_rgba(239,68,68,0.1)]' 
                    : 'text-slate-400 border border-transparent hover:bg-white/5 hover:text-slate-200'
                }`}
              >
                <Icon className="w-4 h-4" />
                {tab.label}
              </button>
            )
          })}
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 p-6 overflow-y-auto relative bg-[#090b14]">
        {isProcessing && (
          <div className="absolute inset-0 z-50 bg-black/80 backdrop-blur-sm flex flex-col items-center justify-center p-8">
            <div className="w-full max-w-md bg-[#11131f] border border-red-500/20 rounded-xl p-6 shadow-2xl">
              <div className="flex items-center gap-4 mb-6">
                <div className="w-10 h-10 rounded-full border-2 border-red-500/30 border-t-red-500 animate-spin" />
                <div>
                  <h3 className="text-white font-medium">{processMessage}</h3>
                  <p className="text-sm text-slate-400">Bitte warten Sie, schalten Sie das Gerät nicht aus.</p>
                </div>
              </div>
              <div className="w-full h-2 bg-black/50 rounded-full overflow-hidden">
                <div 
                  className="h-full bg-red-500 transition-all duration-75 relative"
                  style={{ width: `${progress}%` }}
                >
                  <div className="absolute inset-0 bg-white/20 animate-pulse" />
                </div>
              </div>
              <div className="flex justify-end mt-2">
                <span className="text-xs font-mono text-red-400">{Math.round(progress)}%</span>
              </div>
            </div>
          </div>
        )}

        <motion.div
          key={activeTab}
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          className="max-w-4xl mx-auto space-y-6"
        >
          {activeTab === 'boot' && (
            <>
              <div className="mb-8">
                <h2 className="text-2xl font-bold text-white mb-2">USB Boot Media Creator</h2>
                <p className="text-slate-400">Bootfähigen USB-Stick erstellen um Systeme zu installieren oder zu reparieren.</p>
              </div>
              <div className="grid gap-4">
                <div className="bg-[#11131f] border border-white/5 p-5 rounded-xl flex items-start justify-between group hover:border-red-500/30 transition-all">
                  <div className="flex gap-4">
                    <div className="p-3 bg-red-500/10 rounded-lg text-red-500 h-fit">
                      <Usb className="w-6 h-6" />
                    </div>
                    <div>
                      <h4 className="text-lg font-bold text-slate-200 mb-1">ATC-OS Rescue Stick</h4>
                      <p className="text-sm text-slate-400 mb-4 max-w-md">Brennt das aktuelle Rettungssystem auf /dev/sdb (Cruzer Blade 16GB).</p>
                      <div className="flex items-center gap-3 text-xs bg-black/30 p-2 rounded border border-white/5 w-fit font-mono text-slate-500">
                        <span>Format: FAT32</span>
                        <span>Size: 14.8 GB</span>
                        <span>Disk: /dev/sdb</span>
                      </div>
                    </div>
                  </div>
                  <button 
                    onClick={() => handleAction('Erstelle ATC-OS Rescue Stick auf /dev/sdb...', 4000)}
                    className="px-4 py-2 bg-red-500 hover:bg-red-600 text-white font-medium rounded-lg shadow-lg flex items-center gap-2"
                  >
                    <Play className="w-4 h-4 fill-white" />
                    Start Flash
                  </button>
                </div>
              </div>
            </>
          )}

          {activeTab === 'disk' && (
            <>
              <div className="mb-8 flex items-center gap-4">
                <div>
                  <h2 className="text-2xl font-bold text-white mb-2">Festplatten & Partitionen</h2>
                  <p className="text-slate-400">Speichermedien verwalten, formatieren und sicher löschen.</p>
                </div>
                <div className="ml-auto bg-amber-500/10 border border-amber-500/20 text-amber-500 px-4 py-2 rounded-lg text-sm flex items-center gap-2">
                  <AlertTriangle className="w-4 h-4" />
                  Achtung: Datenverlust bei Formatierung
                </div>
              </div>

              <div className="space-y-6">
                <div className="bg-[#11131f] border border-white/5 rounded-xl overflow-hidden">
                  <div className="p-4 border-b border-white/5 bg-white/5 flex justify-between items-center">
                    <div className="flex items-center gap-3">
                      <HardDrive className="w-5 h-5 text-indigo-400" />
                      <div>
                        <h4 className="font-bold text-slate-200">Disk 0: NVMe Samsung 980</h4>
                        <div className="text-xs text-slate-500 font-mono">1.0 TB • /dev/nvme0n1 • GPT</div>
                      </div>
                    </div>
                    <button 
                      onClick={() => handleAction('Führe Low-Level Format (Secure Erase) auf Disk 0 aus...', 6000)}
                      className="text-xs bg-red-500/10 hover:bg-red-500/20 text-red-400 border border-red-500/20 px-3 py-1.5 rounded transition-colors"
                    >
                      Entire Disk Format
                    </button>
                  </div>
                  <div className="p-4 flex gap-2">
                    <div className="h-12 bg-sky-500/20 border border-sky-500/50 rounded flex-1 flex flex-col justify-center px-3 relative cursor-pointer hover:bg-sky-500/30 transition-colors group">
                      <span className="text-xs font-bold text-sky-400">SYSTEM (EFI)</span>
                      <span className="text-[10px] text-sky-500 font-mono">512 MB • FAT32</span>
                      <div className="absolute top-full left-0 mt-2 bg-[#090b14] border border-white/10 rounded-lg shadow-xl p-2 hidden group-hover:flex flex-col gap-1 z-10 w-48">
                        <button className="text-left text-xs px-2 py-1.5 hover:bg-white/10 rounded text-slate-300">Partition formatieren</button>
                        <button className="text-left text-xs px-2 py-1.5 hover:bg-white/10 rounded text-slate-300">Partition löschen</button>
                      </div>
                    </div>
                    <div className="h-12 bg-indigo-500/20 border border-indigo-500/50 rounded flex-[6] flex flex-col justify-center px-3 relative cursor-pointer hover:bg-indigo-500/30 transition-colors group">
                      <span className="text-xs font-bold text-indigo-400">ATC-OS_ROOT</span>
                      <span className="text-[10px] text-indigo-500 font-mono">820 GB • EXT4</span>
                      <div className="absolute top-full left-0 mt-2 bg-[#090b14] border border-white/10 rounded-lg shadow-xl p-2 hidden group-hover:flex flex-col gap-1 z-10 w-48">
                        <button 
                          onClick={() => handleAction('Partition wird formatiert (EXT4)...', 3000)}
                          className="text-left text-xs px-2 py-1.5 hover:bg-white/10 rounded text-slate-300"
                        >
                          Partition formatieren (EXT4)
                        </button>
                        <button className="text-left text-xs px-2 py-1.5 hover:bg-white/10 rounded text-slate-300">Resize / Move</button>
                        <button className="text-left text-xs px-2 py-1.5 hover:bg-white/10 rounded text-slate-300">Partition löschen</button>
                      </div>
                    </div>
                    <div className="h-12 bg-slate-800/50 border border-slate-700 rounded flex-[2] flex flex-col justify-center px-3 relative cursor-pointer hover:bg-slate-700/50 transition-colors group">
                      <span className="text-xs font-bold text-slate-400">Unallocated</span>
                      <span className="text-[10px] text-slate-500 font-mono">179.5 GB</span>
                      <div className="absolute top-full right-0 mt-2 bg-[#090b14] border border-white/10 rounded-lg shadow-xl p-2 hidden group-hover:flex flex-col gap-1 z-10 w-48">
                        <button 
                          onClick={() => handleAction('Erstelle neue Partition...', 2500)}
                          className="text-left text-xs px-2 py-1.5 hover:bg-white/10 rounded text-slate-300"
                        >
                          Neue Partition erstellen
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </>
          )}

          {activeTab === 'diag' && (
            <>
              <div className="mb-8">
                <h2 className="text-2xl font-bold text-white mb-2">System Check & Diagnostics</h2>
                <p className="text-slate-400">Hardware-Tests und Integritätsprüfung des Dateisystems.</p>
              </div>
              <div className="grid grid-cols-2 gap-4">
                 {[
                   { name: "S.M.A.R.T. Disk Check", desc: "Überprüft den Gesundheitszustand der Festplatten.", icon: HardDrive },
                   { name: "MemTest86+", desc: "Intensiver Arbeitsspeicher (RAM) Belastungstest.", icon: Cpu },
                   { name: "Filesystem Check (fsck)", desc: "Scannt und repariert fehlerhafte Sektoren.", icon: FileSpreadsheet },
                   { name: "Network Interfaces Test", desc: "Überprüft NICs und Firewall Routing.", icon: Activity }
                 ].map(test => (
                    <div key={test.name} className="bg-[#11131f] border border-white/5 p-4 rounded-xl hover:bg-white/5 transition-colors cursor-pointer">
                       <test.icon className="w-6 h-6 text-indigo-400 mb-3" />
                       <h4 className="font-bold text-slate-200 mb-1">{test.name}</h4>
                       <p className="text-xs text-slate-400 mb-4">{test.desc}</p>
                       <button 
                         onClick={() => handleAction(`Führe ${test.name} aus...`, 5000)}
                         className="text-xs font-medium px-3 py-1.5 bg-indigo-500/20 text-indigo-300 rounded hover:bg-indigo-500/30 w-full"
                       >
                         Test starten
                       </button>
                    </div>
                 ))}
              </div>
            </>
          )}

          {activeTab === 'repair' && (
            <>
              <div className="mb-8">
                <h2 className="text-2xl font-bold text-white mb-2">System Repair</h2>
                <p className="text-slate-400">Automatische Reparatur defekter Bootloader oder OS-Komponenten.</p>
              </div>
              <div className="space-y-3">
                 {[
                   { name: "Grub / Bootloader Repair", desc: "Repariert den EFI Bootloader wenn das System nicht startet.", time: 4000 },
                   { name: "Reset User Passwords", desc: "Setzt das Passwort des root/admin Accounts zurück.", time: 1000 },
                   { name: "Restore Registry/Config", desc: "Stellt ein altes Backup der Systemkonfiguration her.", time: 3000 },
                   { name: "Fix Permissions", desc: "Setzt fehlerhafte Datei-Berechtigungen auf Standard zurück.", time: 5000 },
                 ].map((tool, idx) => (
                    <div key={idx} className="flex items-center justify-between p-4 bg-[#11131f] border border-white/5 rounded-xl">
                       <div className="flex items-center gap-4">
                          <div className="p-2 bg-emerald-500/10 rounded-lg"><Wrench className="w-5 h-5 text-emerald-400" /></div>
                          <div>
                             <h4 className="font-bold text-slate-200">{tool.name}</h4>
                             <p className="text-xs text-slate-500">{tool.desc}</p>
                          </div>
                       </div>
                       <button 
                         onClick={() => handleAction(`Executing ${tool.name}...`, tool.time)}
                         className="px-4 py-2 border border-slate-700 text-slate-300 text-sm rounded hover:bg-white/10"
                       >
                         Ausführen
                       </button>
                    </div>
                 ))}
              </div>
            </>
          )}

          {activeTab === 'install' && (
            <>
              <div className="mb-8">
                <h2 className="text-2xl font-bold text-white mb-2">Software Installation</h2>
                <p className="text-slate-400">Download und Setup wichtiger Basissoftware und Treiber.</p>
              </div>
              
              <div className="grid grid-cols-3 gap-4">
                 {[
                    'NVIDIA Graphics Drivers',
                    'ATC Core Kernel',
                    'Docker Engine',
                    'Kubernetes Tools',
                    'ZFS Filesystem Tools',
                    'WireGuard VPN',
                    'Visual Studio Code',
                    'atc-runtime 20 LTS'
                 ].map((software, idx) => (
                    <div key={idx} className="p-4 bg-[#11131f] border border-white/5 rounded-xl flex flex-col items-center text-center gap-2">
                       <Download className="w-6 h-6 text-sky-400 mb-1" />
                       <h4 className="text-sm font-bold text-slate-200 line-clamp-1">{software}</h4>
                       <button 
                         onClick={() => handleAction(`Installing ${software}...`, 3000)}
                         className="mt-2 text-xs w-full py-1.5 bg-sky-500/10 text-sky-400 rounded hover:bg-sky-500/20 transition-colors"
                       >
                         Installieren
                       </button>
                    </div>
                 ))}
              </div>
            </>
          )}
        </motion.div>
      </div>
    </div>
  );
}
