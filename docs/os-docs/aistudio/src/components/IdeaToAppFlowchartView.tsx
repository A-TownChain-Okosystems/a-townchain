import React from 'react';
import { motion } from 'motion/react';
import { 
  Lightbulb, Bot, TerminalSquare, Settings, 
  Smartphone, Code, Play, Hammer, Rocket, Cloud, ArrowDown
} from 'lucide-react';

export const IdeaToAppFlowchartView: React.FC = () => {
  return (
    <div className="w-full h-full bg-slate-900 border border-slate-800 rounded-lg p-6 overflow-y-auto">
      <div className="flex items-center gap-3 mb-8">
        <Rocket className="w-6 h-6 text-atc-cyan" />
        <h2 className="text-xl font-mono text-white tracking-widest uppercase">Idea-to-App Flowchart</h2>
      </div>

      <div className="mb-12 text-slate-400 text-sm max-w-3xl">
        Visuelles Flussdiagramm des Software-Erstellungsprozesses: Von der Idee über die KI-Generierung in der Franchise Factory bis zum finalen Deployment.
      </div>

      <div className="flex flex-col items-center max-w-4xl mx-auto pb-20">
        
        {/* Step 1: The Idea */}
        <motion.div 
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-slate-800/80 border-2 border-yellow-500/50 rounded-xl p-4 w-80 text-center shadow-[0_0_15px_rgba(234,179,8,0.2)]"
        >
          <Lightbulb className="w-8 h-8 text-yellow-500 mx-auto mb-2" />
          <h3 className="text-white font-bold mb-1">1. Idee & Prompt</h3>
          <p className="text-xs text-slate-400">Nutzer definiert App-Funktionen in natürlicher Sprache via GlobusOS / GateToHell Browser.</p>
        </motion.div>

        <ArrowDown className="w-6 h-6 text-slate-600 my-4" />

        {/* Step 2: AI Orchestration */}
        <motion.div 
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="bg-slate-800/80 border-2 border-purple-500/50 rounded-xl p-4 w-80 text-center shadow-[0_0_15px_rgba(168,85,247,0.2)] relative"
        >
          <div className="absolute -left-32 top-1/2 -translate-y-1/2 right-full flex items-center justify-end pr-2 text-xs text-slate-500">
            A-Town Layer 3
          </div>
          <Bot className="w-8 h-8 text-purple-400 mx-auto mb-2" />
          <h3 className="text-white font-bold mb-1">2. KI Orchestrator</h3>
          <p className="text-xs text-slate-400">Routing der Anfrage an spezialisierte KIs (Franchise Factory / Open-Source LLMs).</p>
        </motion.div>

        <ArrowDown className="w-6 h-6 text-slate-600 my-4" />

        {/* Step 3: Generation Block */}
        <div className="border border-slate-700 bg-slate-900/50 rounded-xl p-6 w-full max-w-3xl relative">
          <div className="absolute -top-3 left-6 bg-slate-900 px-2 text-xs font-bold text-slate-500 tracking-widest uppercase">
            Franchise Factory / KI Workflow
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <motion.div 
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ delay: 0.2 }}
              className="bg-slate-800 border border-atc-cyan/30 rounded-lg p-4 text-center"
            >
              <Settings className="w-6 h-6 text-atc-cyan mx-auto mb-2" />
              <h4 className="text-sm font-bold text-white mb-1">Struktur</h4>
              <p className="text-[10px] text-slate-400">Dateien, DB-Planung & Asset-Templates laden</p>
            </motion.div>

            <motion.div 
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ delay: 0.3 }}
              className="bg-slate-800 border border-blue-500/30 rounded-lg p-4 text-center"
            >
              <TerminalSquare className="w-6 h-6 text-blue-400 mx-auto mb-2" />
              <h4 className="text-sm font-bold text-white mb-1">Code</h4>
              <p className="text-[10px] text-slate-400">Text → Code Generierung (JS, Python, Flutter)</p>
            </motion.div>

            <motion.div 
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ delay: 0.4 }}
              className="bg-slate-800 border border-green-500/30 rounded-lg p-4 text-center"
            >
              <Smartphone className="w-6 h-6 text-green-400 mx-auto mb-2" />
              <h4 className="text-sm font-bold text-white mb-1">UI/UX</h4>
              <p className="text-[10px] text-slate-400">Widgets, Menüs & Theme Design integrieren</p>
            </motion.div>
          </div>
        </div>

        <ArrowDown className="w-6 h-6 text-slate-600 my-4" />

        {/* Step 4: Testing Block */}
        <motion.div 
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5 }}
          className="bg-slate-800/80 border-2 border-orange-500/50 rounded-xl p-4 w-80 text-center shadow-[0_0_15px_rgba(249,115,22,0.2)]"
        >
          <Play className="w-8 h-8 text-orange-400 mx-auto mb-2" />
          <h3 className="text-white font-bold mb-1">3. Live Test & Debug</h3>
          <p className="text-xs text-slate-400">Direktes Ausführen im Browser (z.B. Replit-Vorschau). KI behebt Bugs iterativ.</p>
        </motion.div>

        <ArrowDown className="w-6 h-6 text-slate-600 my-4" />

        {/* Step 5: Build Router */}
        <motion.div 
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.6 }}
          className="bg-slate-800/80 border-2 border-slate-600/50 rounded-xl p-4 w-full max-w-2xl text-center"
        >
          <Hammer className="w-8 h-8 text-slate-300 mx-auto mb-2" />
          <h3 className="text-white font-bold mb-4">4. Final Build Platform</h3>
          
          <div className="flex justify-center gap-4">
            <div className="flex-1 bg-slate-900 border border-slate-700 rounded p-2">
              <span className="text-blue-400 font-bold text-xs block mb-1">Windows</span>
              <span className="text-[10px] text-slate-400">Python + PyInstaller → .EXE</span>
            </div>
            <div className="flex-1 bg-slate-900 border border-slate-700 rounded p-2">
              <span className="text-green-400 font-bold text-xs block mb-1">Android</span>
              <span className="text-[10px] text-slate-400">Flutter Build → .APK</span>
            </div>
            <div className="flex-1 bg-slate-900 border border-slate-700 rounded p-2">
              <span className="text-cyan-400 font-bold text-xs block mb-1">Web</span>
              <span className="text-[10px] text-slate-400">React/Node → HTML/JS</span>
            </div>
          </div>
        </motion.div>

        <ArrowDown className="w-6 h-6 text-slate-600 my-4" />

        {/* Step 6: Deployment */}
        <motion.div 
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.7 }}
          className="bg-slate-800/80 border-2 border-green-500/50 rounded-xl p-4 w-80 text-center shadow-[0_0_15px_rgba(34,197,94,0.2)]"
        >
          <Cloud className="w-8 h-8 text-green-400 mx-auto mb-2" />
          <h3 className="text-white font-bold mb-1">5. Deployment / Sharing</h3>
          <p className="text-xs text-slate-400">Web-Hosting (Replit/Pages) oder Download & Teilen via Cloud/A-Town P2P.</p>
        </motion.div>

      </div>
    </div>
  );
};
