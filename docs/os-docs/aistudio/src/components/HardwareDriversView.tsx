import React, { useState } from 'react';
import { 
  Cpu, HardDrive, Monitor, Speaker, Keyboard, Mouse, 
  Wifi, Bluetooth, Printer, Camera, Usb, Laptop, Server,
  CheckCircle2, AlertTriangle, XCircle, ArrowUpCircle, RotateCcw,
  RefreshCw, Search, ShieldCheck, FileText
} from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

type DriverStatus = 'up-to-date' | 'update-available' | 'error' | 'missing';

interface HardwareDevice {
  id: string;
  name: string;
  type: string;
  manufacturer: string;
  driverVersion: string;
  driverDate: string;
  status: DriverStatus;
  icon: React.ElementType;
  category: string;
  subcategory: string;
  tags: string[];
  version: string;
}

const INITIAL_DEVICES: HardwareDevice[] = [
  { id: 'dev-001', name: 'Nvidia RTX 5090 Ti', type: 'Display Adapter', manufacturer: 'Nvidia Corporation', driverVersion: '560.12', driverDate: '2026-05-14', status: 'up-to-date', icon: Monitor, category: 'Hardware Component', subcategory: 'GPU', tags: ['graphics', 'display'], version: 'v5.6.0' },
  { id: 'dev-002', name: 'Intel Core Ultra 9 285K', type: 'Processor', manufacturer: 'Intel', driverVersion: '10.0.22621.1', driverDate: '2025-10-21', status: 'up-to-date', icon: Cpu, category: 'Hardware Component', subcategory: 'CPU', tags: ['processor', 'core'], version: 'v1.0.0' },
  { id: 'dev-003', name: 'Realtek High Definition Audio', type: 'Audio, Video & Game Controllers', manufacturer: 'Realtek', driverVersion: '6.0.9542.1', driverDate: '2025-08-30', status: 'update-available', icon: Speaker, category: 'Peripherals', subcategory: 'Audio', tags: ['sound', 'multimedia'], version: 'v6.0.9' },
  { id: 'dev-004', name: 'Intel Wi-Fi 7 BE200 320MHz', type: 'Network Adapter', manufacturer: 'Intel', driverVersion: '23.20.0.4', driverDate: '2026-01-10', status: 'up-to-date', icon: Wifi, category: 'Networking', subcategory: 'Wireless', tags: ['wifi', 'network'], version: 'v23.2.0' },
  { id: 'dev-005', name: 'WD_BLACK SN850X 2TB', type: 'Disk Drive', manufacturer: 'Western Digital', driverVersion: '10.0.22621.1', driverDate: '2023-05-06', status: 'error', icon: HardDrive, category: 'Storage', subcategory: 'NVMe', tags: ['disk', 'storage'], version: 'v1.0.0' },
  { id: 'dev-006', name: 'Logitech MX Master 4', type: 'Mice and other pointing devices', manufacturer: 'Logitech', driverVersion: '8.40.100.0', driverDate: '2026-02-15', status: 'update-available', icon: Mouse, category: 'Peripherals', subcategory: 'Input Devices', tags: ['mouse', 'pointer'], version: 'v8.4.0' },
  { id: 'dev-007', name: 'Keychron Q1 Pro', type: 'Keyboard', manufacturer: 'Keychron', driverVersion: '1.2.0.0', driverDate: '2025-11-01', status: 'up-to-date', icon: Keyboard, category: 'Peripherals', subcategory: 'Input Devices', tags: ['keyboard', 'typing'], version: 'v1.2.0' },
  { id: 'dev-008', name: 'Generic Bluetooth Adapter', type: 'Bluetooth', manufacturer: 'Microsoft', driverVersion: '10.0.22621.3235', driverDate: '2024-03-20', status: 'missing', icon: Bluetooth, category: 'Networking', subcategory: 'Bluetooth', tags: ['wireless', 'adapter'], version: 'v10.0.2' },
  { id: 'dev-009', name: 'USB Composite Device', type: 'Universal Serial Bus controllers', manufacturer: 'Standard USB Host Controller', driverVersion: '10.0.22621.2506', driverDate: '2023-01-15', status: 'up-to-date', icon: Usb, category: 'System Devices', subcategory: 'USB', tags: ['controller', 'bus'], version: 'v1.0.0' },
  { id: 'dev-010', name: 'Sony A7IV Web Camera', type: 'Cameras', manufacturer: 'Sony', driverVersion: '3.1.0', driverDate: '2025-07-22', status: 'up-to-date', icon: Camera, category: 'Peripherals', subcategory: 'Video', tags: ['camera', 'webcam'], version: 'v3.1.0' },
];

export function HardwareDriversView() {
  const [devices, setDevices] = useState<HardwareDevice[]>(INITIAL_DEVICES);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedDevice, setSelectedDevice] = useState<HardwareDevice | null>(null);
  const [isUpdating, setIsUpdating] = useState<Record<string, boolean>>({});

  const filteredDevices = devices.filter(d => 
    d.name.toLowerCase().includes(searchTerm.toLowerCase()) || 
    d.type.toLowerCase().includes(searchTerm.toLowerCase()) ||
    d.manufacturer.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleUpdate = async (id: string) => {
    setIsUpdating(prev => ({ ...prev, [id]: true }));
    // Simulate update process
    await new Promise(resolve => setTimeout(resolve, 2500));
    setDevices(prev => prev.map(d => 
      d.id === id 
        ? { ...d, status: 'up-to-date', driverVersion: `${d.driverVersion}.1 (Updated)`, driverDate: new Date().toISOString().split('T')[0] } 
        : d
    ));
    setIsUpdating(prev => ({ ...prev, [id]: false }));
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Hardware driver updated successfully.' }));
  };

  const handleRollback = async (id: string) => {
    setIsUpdating(prev => ({ ...prev, [id]: true }));
    // Simulate rollback
    await new Promise(resolve => setTimeout(resolve, 1500));
    setDevices(prev => prev.map(d => 
      d.id === id 
        ? { ...d, status: 'update-available', driverVersion: `${d.driverVersion} (Reverted)` } 
        : d
    ));
    setIsUpdating(prev => ({ ...prev, [id]: false }));
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Driver rollback completed.' }));
  };

  const handleScan = async () => {
    setIsUpdating(prev => ({ ...prev, 'scan': true }));
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Scanne Hardware...' }));
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    const tempId = `dev-${Date.now()}`;
    const newUnknownDevice: HardwareDevice = {
      id: tempId,
      name: 'Unbekanntes Gerät (PCIe)',
      type: 'Andere Geräte',
      manufacturer: 'Unbekannt',
      driverVersion: 'Fehlt',
      driverDate: 'N/A',
      status: 'missing',
      icon: AlertTriangle,
      category: 'Unknown',
      subcategory: 'Undefined',
      tags: ['unknown', 'pcie'],
      version: 'N/A'
    };
    
    setDevices(prev => [newUnknownDevice, ...prev]);
    setSelectedDevice(newUnknownDevice);
    setIsUpdating(prev => ({ ...prev, 'scan': false, [tempId]: true }));
    
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'Hardware gefunden. KI generiert Treiber...' }));
    
    await new Promise(resolve => setTimeout(resolve, 4000));
    
    setDevices(prev => prev.map(d => 
      d.id === tempId ? {
        ...d,
        name: 'ATC Quantum Neural Coprocessor',
        type: 'ATVM KI-Beschleuniger',
        manufacturer: 'A-TownChain Auto-Gen',
        driverVersion: '1.0.0-ai-synthesized',
        driverDate: new Date().toISOString().split('T')[0],
        status: 'up-to-date',
        icon: Cpu,
        category: 'Hardware Component',
        subcategory: 'AI Accelerator',
        tags: ['neural', 'quantum', 'ai'],
        version: 'v1.0.0'
      } : d
    ));
    
    setIsUpdating(prev => ({ ...prev, [tempId]: false }));
    window.dispatchEvent(new CustomEvent('ATC_SYSTEM_ALERT', { detail: 'KI generierte Treiber & Datenbank-Update erfolgreich.' }));
  };

  const getStatusIcon = (status: DriverStatus) => {
    switch(status) {
      case 'up-to-date': return <CheckCircle2 className="w-4 h-4 text-emerald-400" />;
      case 'update-available': return <ArrowUpCircle className="w-4 h-4 text-blue-400" />;
      case 'error': return <XCircle className="w-4 h-4 text-rose-400" />;
      case 'missing': return <AlertTriangle className="w-4 h-4 text-amber-400" />;
    }
  };

  const getStatusText = (status: DriverStatus) => {
    switch(status) {
      case 'up-to-date': return 'Up to date';
      case 'update-available': return 'Update available';
      case 'error': return 'Device Error (Code 43)';
      case 'missing': return 'Driver missing';
    }
  };

  const getStatusColor = (status: DriverStatus) => {
    switch(status) {
      case 'up-to-date': return 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20';
      case 'update-available': return 'bg-blue-500/10 text-blue-400 border-blue-500/20';
      case 'error': return 'bg-rose-500/10 text-rose-400 border-rose-500/20';
      case 'missing': return 'bg-amber-500/10 text-amber-400 border-amber-500/20';
    }
  };

  return (
    <div className="flex h-full bg-[#050811] text-slate-200 overflow-hidden font-sans">
      
      {/* Sidebar - Device List */}
      <div className="w-1/2 md:w-1/3 lg:w-2/5 border-r border-white/10 flex flex-col bg-[#080d1a] relative z-10 shadow-[5px_0_15px_rgba(0,0,0,0.5)]">
        {/* Header */}
        <div className="p-4 border-b border-white/10 bg-gradient-to-b from-white/5 to-transparent">
          <div className="flex items-center gap-3 mb-4">
            <div className="p-2 bg-atc-cyan/10 rounded-lg border border-atc-cyan/20 text-atc-cyan shadow-[0_0_10px_rgba(0,120,212,0.2)]">
               <Server className="w-5 h-5" />
            </div>
            <h1 className="text-lg font-bold text-white tracking-tight">Treiber Datenbank</h1>
          </div>
          
          <div className="relative">
            <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" />
            <input 
              type="text" 
              placeholder="Hardware durchsuchen..."
              value={searchTerm}
              onChange={e => setSearchTerm(e.target.value)}
              className="w-full bg-black/40 border border-white/10 rounded-xl py-2 pl-9 pr-4 text-sm text-slate-200 placeholder-slate-500 focus:outline-none focus:border-atc-cyan/50 focus:ring-1 focus:ring-atc-cyan/50 transition-all"
            />
          </div>
        </div>

        {/* List */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-2 space-y-1">
          {filteredDevices.map(device => {
            const isSelected = selectedDevice?.id === device.id;
            return (
              <button
                key={device.id}
                onClick={() => setSelectedDevice(device)}
                className={`w-full text-left p-3 rounded-xl flex items-start gap-3 transition-all ${
                  isSelected 
                    ? 'bg-atc-cyan/15 border border-atc-cyan/30 shadow-[inset_0_0_15px_rgba(0,120,212,0.1)]' 
                    : 'bg-transparent border border-transparent hover:bg-white/5 hover:border-white/10'
                }`}
              >
                <div className={`p-2 rounded-lg ${isSelected ? 'bg-atc-cyan/20 text-atc-cyan' : 'bg-slate-800 text-slate-400'}`}>
                  <device.icon className="w-5 h-5" />
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center justify-between gap-2 mb-1">
                    <h3 className={`font-medium truncate text-sm ${isSelected ? 'text-white' : 'text-slate-200'}`}>{device.name}</h3>
                    {getStatusIcon(device.status)}
                  </div>
                  <p className="text-xs text-slate-500 truncate">{device.type}</p>
                </div>
              </button>
            )
          })}
        </div>
        
        {/* Footer actions */}
        <div className="p-4 border-t border-white/10 bg-black/20">
           <button 
             onClick={handleScan}
             disabled={isUpdating['scan']}
             className="atc-btn-primary w-full shadow-none hover:-translate-y-0 disabled:opacity-50 disabled:cursor-not-allowed"
           >
             {isUpdating['scan'] ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Search className="w-4 h-4" />}
             {isUpdating['scan'] ? 'KI analysiert Hardware...' : 'Scan nach neuen Geräten starten'}
           </button>
        </div>
      </div>

      {/* Main Content - Device Details */}
      <div className="flex-1 flex flex-col bg-gradient-to-br from-[#050811] to-[#0a1020] relative">
        {selectedDevice ? (
          <AnimatePresence mode="wait">
            <motion.div 
              key={selectedDevice.id}
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.2 }}
              className="flex-1 overflow-y-auto custom-scrollbar p-6 lg:p-10"
            >
              <div className="max-w-2xl mx-auto space-y-8">
                
                {/* Header Information */}
                <div className="flex items-start gap-6">
                  <div className="p-4 bg-slate-800/80 rounded-2xl border border-white/10 shadow-lg">
                    <selectedDevice.icon className="w-12 h-12 text-slate-300" />
                  </div>
                  <div className="flex-1">
                    <h2 className="text-2xl font-bold text-white tracking-tight mb-2">{selectedDevice.name}</h2>
                    <div className="flex flex-wrap gap-2 mb-4">
                      <span className="px-2.5 py-1 bg-white/5 border border-white/10 rounded-md text-xs font-medium text-slate-400">
                        {selectedDevice.type}
                      </span>
                      <span className="px-2.5 py-1 bg-white/5 border border-white/10 rounded-md text-xs font-medium text-slate-400">
                        Mfg: {selectedDevice.manufacturer}
                      </span>
                      <span className="px-2.5 py-1 bg-white/5 border border-white/10 rounded-md text-xs font-medium text-slate-400">
                        {selectedDevice.category}
                      </span>
                      <span className="px-2.5 py-1 bg-white/5 border border-white/10 rounded-md text-xs font-medium text-slate-400">
                        {selectedDevice.subcategory}
                      </span>
                      <span className="px-2.5 py-1 bg-white/5 border border-white/10 rounded-md text-xs font-medium text-slate-400">
                        {selectedDevice.version}
                      </span>
                    </div>
                    
                    <div className="flex flex-wrap gap-2 mb-4">
                      {selectedDevice.tags.map(tag => (
                        <span key={tag} className="px-2 py-0.5 bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 rounded text-[10px] uppercase font-mono tracking-wider">
                          {tag}
                        </span>
                      ))}
                    </div>

                    <div className={`inline-flex items-center gap-2 px-3 py-1.5 rounded-lg border text-sm font-medium ${getStatusColor(selectedDevice.status)}`}>
                      {getStatusIcon(selectedDevice.status)}
                      {getStatusText(selectedDevice.status)}
                    </div>
                  </div>
                </div>

                {/* Driver Details Card */}
                <div className="bg-white/5 border border-white/10 rounded-2xl p-6 relative overflow-hidden backdrop-blur-sm">
                  <div className="absolute top-0 right-0 p-4 opacity-5 pointer-events-none">
                     <ShieldCheck className="w-32 h-32" />
                  </div>
                  
                  <h3 className="text-lg font-semibold text-white mb-6 flex items-center gap-2">
                    <FileText className="w-5 h-5 text-slate-400" />
                    Driver Information
                  </h3>
                  
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 relative z-10">
                    <div>
                      <p className="text-xs text-slate-500 uppercase tracking-wider font-semibold mb-1">Driver Provider</p>
                      <p className="font-medium text-slate-200">{selectedDevice.manufacturer}</p>
                    </div>
                    <div>
                      <p className="text-xs text-slate-500 uppercase tracking-wider font-semibold mb-1">Driver Date</p>
                      <p className="font-medium text-slate-200 text-mono">{selectedDevice.driverDate}</p>
                    </div>
                    <div>
                      <p className="text-xs text-slate-500 uppercase tracking-wider font-semibold mb-1">Driver Version</p>
                      <p className="font-medium text-slate-200 text-mono">{selectedDevice.driverVersion}</p>
                    </div>
                    <div>
                      <p className="text-xs text-slate-500 uppercase tracking-wider font-semibold mb-1">Digital Signer</p>
                      <p className="font-medium text-slate-200 flex items-center gap-1">
                         <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
                         Microsoft Windows Hardware Compatibility Publisher
                      </p>
                    </div>
                  </div>
                </div>

                {/* Action Buttons */}
                <div className="space-y-4">
                  <h3 className="text-sm font-semibold text-slate-400 uppercase tracking-wider pl-1">Device Actions</h3>
                  
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                    <button 
                      onClick={() => handleUpdate(selectedDevice.id)}
                      disabled={selectedDevice.status === 'up-to-date' || isUpdating[selectedDevice.id]}
                      className={`flex items-center gap-3 p-4 rounded-xl border transition-all text-left ${
                        selectedDevice.status === 'up-to-date'
                          ? 'bg-white/5 border-white/5 opacity-50 cursor-not-allowed'
                          : 'bg-indigo-500/10 border-indigo-500/30 hover:bg-indigo-500/20 hover:border-indigo-500/50 cursor-pointer'
                      }`}
                    >
                      <div className={`p-2 rounded-lg ${selectedDevice.status === 'up-to-date' ? 'bg-slate-700 text-slate-400' : 'bg-indigo-500/20 text-indigo-400'}`}>
                        {isUpdating[selectedDevice.id] ? <RefreshCw className="w-5 h-5 animate-spin" /> : <ArrowUpCircle className="w-5 h-5" />}
                      </div>
                      <div>
                        <div className={`font-medium ${selectedDevice.status === 'up-to-date' ? 'text-slate-300' : 'text-indigo-200'}`}>Update Driver</div>
                        <div className="text-xs text-slate-500">Search automatically for updated driver software</div>
                      </div>
                    </button>

                    <button 
                      onClick={() => handleRollback(selectedDevice.id)}
                      disabled={isUpdating[selectedDevice.id]}
                      className="flex items-center gap-3 p-4 rounded-xl border border-white/10 bg-white/5 hover:bg-white/10 hover:border-white/20 transition-all text-left cursor-pointer"
                    >
                      <div className="p-2 rounded-lg bg-slate-700/50 text-slate-400">
                        <RotateCcw className="w-5 h-5" />
                      </div>
                      <div>
                        <div className="font-medium text-slate-200">Roll Back Driver</div>
                        <div className="text-xs text-slate-500">Revert to the previously installed driver</div>
                      </div>
                    </button>
                    
                    <button className="flex items-center gap-3 p-4 rounded-xl border border-rose-500/20 bg-rose-500/5 hover:bg-rose-500/10 hover:border-rose-500/30 transition-all text-left cursor-pointer sm:col-span-2">
                      <div className="p-2 rounded-lg bg-rose-500/10 text-rose-400">
                        <XCircle className="w-5 h-5" />
                      </div>
                      <div>
                        <div className="font-medium text-rose-200">Uninstall Device</div>
                        <div className="text-xs text-slate-500">Remove from the system (Advanced)</div>
                      </div>
                    </button>
                  </div>
                </div>

              </div>
            </motion.div>
          </AnimatePresence>
        ) : (
          <div className="flex-1 flex flex-col items-center justify-center text-center p-8 opacity-50">
            <div className="w-24 h-24 mb-6 rounded-3xl bg-slate-800/50 border border-slate-700/50 flex items-center justify-center shadow-2xl">
              <Server className="w-10 h-10 text-slate-400" />
            </div>
            <h2 className="text-xl font-medium text-white mb-2">No Device Selected</h2>
            <p className="text-slate-400 max-w-sm">Select a hardware device from the list on the left to view details, update drivers, or troubleshoot issues.</p>
          </div>
        )}
      </div>

    </div>
  );
}
