import React from 'react';
import { motion } from 'motion/react';
import { 
  Network, Database, Globe, Layers, 
  Cpu, Shield, Box, Smartphone, Hammer, Lock
} from 'lucide-react';

export const EcosystemUmlView: React.FC = () => {
  return (
    <div className="w-full h-full bg-slate-900 border border-slate-800 rounded-lg p-6 overflow-y-auto">
      <div className="flex items-center gap-3 mb-6">
        <Network className="w-6 h-6 text-atc-cyan" />
        <h2 className="text-xl font-mono text-white tracking-widest uppercase">Ecosystem UML Diagram</h2>
      </div>

      <div className="mb-8 text-slate-400 text-sm max-w-3xl">
        Grafische Baumstruktur & Abhängigkeiten (UML-ähnliches Diagramm) der Hauptsysteme im GlobusOS & A-Town Ökosystem.
      </div>

      <div className="relative min-w-[800px] overflow-x-auto pb-20">
        
        {/* UML CORE */}
        <div className="flex justify-center mb-12 relative">
          <motion.div 
            initial={{ scale: 0.9, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            className="bg-slate-800 border-2 border-atc-cyan rounded-xl p-4 shadow-[0_0_20px_rgba(34,211,238,0.2)] w-72 text-center relative z-10"
          >
            <Layers className="w-8 h-8 text-atc-cyan mx-auto mb-2" />
            <h3 className="font-bold text-lg text-white mb-1">A-Town Blockchain</h3>
            <p className="text-xs text-slate-400">Layered Architecture</p>
            <div className="mt-4 text-left text-xs bg-slate-900 rounded p-2 text-slate-300 space-y-1">
              <div className="flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-slate-500"/>Layer 0 (Network, Mesh)</div>
              <div className="flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-slate-500"/>Layer 1 (Consensus, PoW/PoS)</div>
              <div className="flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-slate-500"/>Layer 2 (Smart Contracts, dApps)</div>
              <div className="flex items-center gap-2"><div className="w-1.5 h-1.5 rounded-full bg-slate-500"/>Layer 3 (AI Orchestrator)</div>
            </div>
          </motion.div>
        </div>

        {/* CONNECTION LINES */}
        <div className="absolute top-[180px] left-1/2 w-[70%] -translate-x-1/2 h-px bg-slate-700 z-0"></div>
        <div className="absolute top-[180px] left-[15%] w-px h-10 bg-slate-700 z-0"></div>
        <div className="absolute top-[180px] left-[50%] w-px h-10 bg-slate-700 z-0"></div>
        <div className="absolute top-[180px] left-[85%] w-px h-10 bg-slate-700 z-0"></div>

        {/* THREE PILLARS */}
        <div className="flex justify-between w-full relative z-10">
          
          {/* GlobusOS */}
          <motion.div 
            initial={{ y: 20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.2 }}
            className="w-[30%] bg-slate-800/80 border border-slate-700 rounded-lg p-4 shadow-lg"
          >
            <div className="flex items-center justify-center gap-2 mb-3 bg-blue-500/20 py-2 rounded border border-blue-500/30">
              <Smartphone className="w-5 h-5 text-blue-400" />
              <h3 className="font-bold text-white">GlobusOS</h3>
            </div>
            <ul className="text-xs text-slate-300 space-y-3">
              <li className="bg-slate-900 border border-slate-700 rounded p-2">
                <span className="font-semibold text-blue-300 block mb-1">Multi-Device OS</span>
                Touch, Maus, Controller, Remote
              </li>
              <li className="bg-slate-900 border border-slate-700 rounded p-2">
                <span className="font-semibold text-blue-300 block mb-1">User Management</span>
                Profile, Wallet, Cloud-Sync
              </li>
              <li className="bg-slate-900 border border-slate-700 rounded p-2 flex items-center gap-2">
                <Lock className="w-4 h-4 text-slate-500 shrink-0"/> E2E Encryption & Light Nodes
              </li>
            </ul>
          </motion.div>

          {/* Franchise Factory */}
          <motion.div 
            initial={{ y: 20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.3 }}
            className="w-[30%] bg-slate-800/80 border border-slate-700 rounded-lg p-4 shadow-lg relative"
          >
            {/* DEPENDENCY LINE */}
            <div className="absolute -top-10 left-1/2 w-4 h-10 border-l-2 border-dashed border-green-500/50 -translate-x-1/2"></div>
            
            <div className="flex items-center justify-center gap-2 mb-3 bg-green-500/20 py-2 rounded border border-green-500/30">
              <Hammer className="w-5 h-5 text-green-400" />
              <h3 className="font-bold text-white">Franchise Factury KI</h3>
            </div>
            <ul className="text-xs text-slate-300 space-y-3">
              <li className="bg-slate-900 border border-slate-700 rounded p-2">
                <span className="font-semibold text-green-300 block mb-1">Asset-Generierung</span>
                Standard + Online Templates DB
              </li>
              <li className="bg-slate-900 border border-slate-700 rounded p-2">
                <span className="font-semibold text-green-300 block mb-1">Software-Erstellung</span>
                Text → Code → EXE/APK/Web
              </li>
              <li className="bg-slate-900 border border-slate-700 rounded p-2 flex items-center gap-2">
                <Cpu className="w-4 h-4 text-slate-500 shrink-0"/> KI Routing & Workflow-Opt.
              </li>
            </ul>
          </motion.div>

          {/* GateToHell Browser */}
          <motion.div 
            initial={{ y: 20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.4 }}
            className="w-[30%] bg-slate-800/80 border border-slate-700 rounded-lg p-4 shadow-lg relative"
          >
            <div className="flex items-center justify-center gap-2 mb-3 bg-red-500/20 py-2 rounded border border-red-500/30">
              <Globe className="w-5 h-5 text-red-400" />
              <h3 className="font-bold text-white">GateToHell Browser</h3>
            </div>
            <ul className="text-xs text-slate-300 space-y-3">
              <li className="bg-slate-900 border border-slate-700 rounded p-2 bg-red-900/10 border-red-500/20">
                <span className="font-semibold text-red-300 block mb-1">Web3-kompatibel</span>
                Nutzer Wallet Integration & Bookmarks
              </li>
              <li className="bg-slate-900 border border-slate-700 rounded p-2">
                <span className="font-semibold text-red-300 block mb-1">Dezentrale Datenströme</span>
                Light Nodes & P2P Transfer
              </li>
              <li className="bg-slate-900 border border-slate-700 rounded p-2 flex items-center gap-2">
                <Shield className="w-4 h-4 text-slate-500 shrink-0"/> Full Node Support & Anti-Track
              </li>
            </ul>
          </motion.div>

        </div>

        {/* BOTTOM SHARED LAYER */}
        <div className="mt-12 max-w-2xl mx-auto bg-slate-800/50 border border-slate-700 rounded p-4 text-center">
          <Database className="w-6 h-6 text-slate-400 mx-auto mb-2" />
          <h4 className="text-white font-bold text-sm mb-1">Shared Template & Asset DB</h4>
          <p className="text-xs text-slate-500">Gemeinsame Offline/Online Datenbank für Franchises, Assets und Nutzerprofile.</p>
        </div>

      </div>
    </div>
  );
};
