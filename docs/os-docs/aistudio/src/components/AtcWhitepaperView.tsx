import React, { useState } from 'react';
import { BookOpen, Download, FileText, ChevronRight, Activity, Globe, Database, Network, Shield, Cpu, Zap, Share2 } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

const WHITEPAPER_SECTIONS = [
  {
    id: "abstract",
    title: "1. Abstract",
    icon: <Globe className="w-4 h-4" />,
    content: `Die A-TownChain (ATC) stellt einen Paradigmenwechsel in der dezentralen Netzwerkinfrastruktur dar. Anstatt sich auf einen einzelnen Konsensmechanismus zu verlassen, führt ATC einen "Hybrid Consensus" ein: Eine Verschmelzung von Proof of Work (PoW), Proof of Stake (PoS), Proof of Importance (PoI), Proof of History (PoH) und Proof of AI (PoAI). 
    
Dieses Dokument skizziert die Systemarchitektur für ein autarkes, hochperformantes und KI-gestütztes Ökosystem, das speziell für den Betrieb der AtownOS Agenten-Zivilisation konzipiert wurde.`
  },
  {
    id: "consensus",
    title: "2. The Penta-Hybrid Consensus",
    icon: <Network className="w-4 h-4" />,
    content: `Der Kern von ATC ist der hybride Validierungsmechanismus.
    
• PoW (Proof of Work): Physische Berechnung für Sybil-Resistance und Entropie-Erzeugung.
• PoS (Proof of Stake): Ökonomische Sicherheit und "Skin in the game" der Validatoren.
• PoI (Proof of Importance): Bewertung des aktiven Beitrags eines Nodes zum Netzwerk (z.B. Bandbreite, Speicher).
• PoH (Proof of History): Kryptografische Zeitstempel für asynchrone Skalierbarkeit und Order-Einhaltung.
• PoAI (Proof of AI): Training und Ausführung autonomer Agenten-Modelle als Teil des Mining-Prozesses.`
  },
  {
    id: "tokenomics",
    title: "3. Tokenomics & Emission",
    icon: <Activity className="w-4 h-4" />,
    content: `Die ökonomische Struktur ist auf Langfristigkeit und Werterhalt ausgelegt (139-jährige Emissionskurve).

Initial Supply (Genesis): 1.000.000.000 ATC
Max Supply: 21.000.000.000 ATC

Die Block-Rewards verteilen sich dynamisch basierend auf der Netzwerkauslastung. Validatoren, die KI-Workloads (PoAI) berechnen, erhalten einen Reward-Multiplier von 1.5x.`
  },
  {
    id: "architecture",
    title: "4. Node Architecture (ShivaCore)",
    icon: <Server className="w-4 h-4" />,
    content: `ATC-Nodes sind in Rust geschrieben, um Speichersicherheit und C-Level-Performance zu garantieren.
    
Der "ShivaCore" nutzt:
• RocksDB für hochperformantes State-Storage
• libp2p für das Gossipsub-Netzwerk
• tokio für asynchronen Konsens und PoAI-Loops
• gRPC für Inter-Process-Communication mit GlobusOS`
  },
  {
    id: "zk",
    title: "5. Zero-Knowledge Privacy",
    icon: <Shield className="w-4 h-4" />,
    content: `Um die Privatsphäre der autonomen Agenten zu schützen, integriert ATC ZK-SNARKs.
    
Transaktionsdaten, Wallet-Salden und Smart Contract-Ausführungen können über Zero-Knowledge Proofs validiert werden, ohne die zugrunde liegenden Daten offenzulegen. Dies ist entscheidend für das A-Town Private Ledger.`
  }
];

// Fallback Server icon since it's not imported at the top
function Server(props: any) {
  return <Database {...props} />;
}

export function AtcWhitepaperView() {
  const [activeSection, setActiveSection] = useState(WHITEPAPER_SECTIONS[0].id);

  const activeContent = WHITEPAPER_SECTIONS.find(s => s.id === activeSection);

  return (
    <div className="flex flex-col h-full bg-[#050B14] border border-white/5 rounded-xl overflow-hidden font-sans">
      <div className="flex items-center justify-between px-6 py-4 border-b border-white/5 bg-slate-900/50">
        <div className="flex items-center gap-3">
            <BookOpen className="w-5 h-5 text-indigo-400" />
            <div>
            <h2 className="text-sm font-bold text-white tracking-widest font-mono uppercase">A-TownChain Whitepaper</h2>
            <p className="text-xs text-slate-400 mt-0.5">Offizielles Dokumentations-Skelett & Ökonomisches Modell</p>
            </div>
        </div>
        
        <button 
            className="px-4 py-2 bg-indigo-500/10 text-indigo-400 border border-indigo-500/30 hover:bg-indigo-500/20 rounded font-mono text-[10px] uppercase tracking-wider transition-all flex items-center gap-2"
        >
            <Download className="w-3.5 h-3.5" /> Export PDF
        </button>
      </div>
      
      <div className="flex-1 flex overflow-hidden">
         {/* Sidebar table of contents */}
         <div className="w-64 border-r border-white/5 bg-[#020408] overflow-y-auto custom-scrollbar p-4">
             <div className="text-[10px] uppercase tracking-widest font-mono text-slate-500 mb-4 px-2">Inhaltsverzeichnis</div>
             <div className="space-y-1">
                 {WHITEPAPER_SECTIONS.map((section) => (
                    <button
                       key={section.id}
                       onClick={() => setActiveSection(section.id)}
                       className={`w-full text-left px-3 py-2.5 rounded-lg flex items-center justify-between transition-colors ${
                           activeSection === section.id 
                           ? 'bg-indigo-500/10 text-indigo-300 border border-indigo-500/20' 
                           : 'text-slate-400 hover:bg-white/5 hover:text-slate-300 border border-transparent'
                       }`}
                    >
                        <div className="flex items-center gap-2.5 text-xs font-mono">
                            {section.icon}
                            <span className="truncate">{section.title}</span>
                        </div>
                        {activeSection === section.id && <ChevronRight className="w-3 h-3 text-indigo-400" />}
                    </button>
                 ))}
             </div>
             
             <div className="mt-8 px-2">
                 <div className="p-4 bg-slate-900/50 border border-white/5 rounded-xl">
                    <div className="text-[10px] font-mono text-slate-500 uppercase mb-2">Network Status</div>
                    <div className="flex items-center gap-2 text-xs font-mono text-emerald-400 mb-1">
                        <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                        Mainnet Ready
                    </div>
                    <div className="text-[10px] text-slate-500 font-mono">Genesis Block: VALID</div>
                 </div>
             </div>
         </div>

         {/* Content Area */}
         <div className="flex-1 bg-slate-950 p-10 overflow-y-auto custom-scrollbar flex justify-center">
             <div className="max-w-2xl w-full">
                 <AnimatePresence mode="wait">
                     <motion.div
                        key={activeSection}
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        transition={{ duration: 0.2 }}
                     >
                         {activeContent && (
                             <div className="space-y-8">
                                 <div>
                                     <h1 className="text-2xl font-serif text-white mb-2">{activeContent.title}</h1>
                                     <div className="w-12 h-1 bg-indigo-500 rounded-full" />
                                 </div>
                                 
                                 <div className="prose prose-invert prose-slate max-w-none">
                                     <div className="text-slate-300 leading-relaxed font-serif whitespace-pre-wrap">
                                         {activeContent.content}
                                     </div>
                                 </div>

                                 {activeSection === "tokenomics" && (
                                     <div className="mt-8 p-6 bg-[#050B14] border border-indigo-500/10 rounded-xl relative overflow-hidden">
                                         <div className="absolute top-0 right-0 p-4 opacity-5">
                                             <Activity className="w-32 h-32 text-indigo-500" />
                                         </div>
                                         <h3 className="font-mono text-sm text-indigo-300 uppercase tracking-widest mb-4">139-Year Emission Curve</h3>
                                         <div className="h-32 w-full flex items-end gap-1">
                                             {/* Abstract curve representation */}
                                             {Array.from({ length: 40 }).map((_, i) => (
                                                 <div 
                                                    key={i} 
                                                    className="flex-1 bg-indigo-500/20 hover:bg-indigo-500/40 transition-colors rounded-t-sm"
                                                    style={{ height: `${Math.max(10, 100 * Math.exp(-i / 10))}%` }}
                                                 />
                                             ))}
                                         </div>
                                     </div>
                                 )}

                                 {activeSection === "consensus" && (
                                     <div className="grid grid-cols-2 gap-4 mt-8">
                                         {['PoW', 'PoS', 'PoI', 'PoH', 'PoAI'].map(mechanism => (
                                            <div key={mechanism} className="p-4 bg-slate-900/40 border border-white/5 rounded-xl flex items-center gap-3">
                                                <div className="w-8 h-8 rounded-full bg-indigo-500/10 flex items-center justify-center border border-indigo-500/20 text-indigo-400">
                                                    <Zap className="w-4 h-4" />
                                                </div>
                                                <div className="font-mono text-sm text-slate-200">{mechanism}</div>
                                            </div>
                                         ))}
                                     </div>
                                 )}
                             </div>
                         )}
                     </motion.div>
                 </AnimatePresence>
             </div>
         </div>
      </div>
    </div>
  );
}
