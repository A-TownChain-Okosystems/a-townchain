import React from 'react';
import { 
  Bot, Code, TerminalSquare, Smartphone, 
  Monitor, Cpu, ArrowRight, Hammer, GitPullRequest, 
  Lightbulb, Zap, Rocket, AlertTriangle, CheckCircle 
} from 'lucide-react';
import { motion } from 'motion/react';

export const AiSoftwareWorkflowView: React.FC = () => {
  const tools = [
    {
      name: "Replit AI",
      type: "Online IDE + KI-Assistent",
      features: "Code schreiben, ausführen, testen. Projekte hosten / kleine Builds starten. Direkt im Browser.",
      limits: "Komplexe Builds (native EXE/APK) müssen ggf. exportiert werden.",
      icon: <Monitor className="w-5 h-5 text-cyan-400" />
    },
    {
      name: "GitHub Copilot X",
      type: "IDE Plugin (Free via Edu/Trial)",
      features: "Autovervollständigung, ganze Module generieren basierend auf Beschreibungen.",
      limits: "Braucht lokale IDE. Builds müssen lokal/extern ausgeführt werden.",
      icon: <Code className="w-5 h-5 text-purple-400" />
    },
    {
      name: "CodeLlama / StarCoder",
      type: "HuggingFace Spaces Playgrounds",
      features: "Open-Source Modelle direkt im Browser nutzbar. Generiert ausführlichen Code.",
      limits: "Kein Build-Service, nur Text-zu-Code. Muss manuell rauskopiert werden.",
      icon: <TerminalSquare className="w-5 h-5 text-blue-400" />
    },
    {
      name: "Tabnine (Free Tier)",
      type: "Browser/IDE Coding-Assistent",
      features: "Unterstützt viele Sprachen, kostenloser Funktionsumfang.",
      limits: "Kein automatisches Hosting / Build.",
      icon: <Bot className="w-5 h-5 text-green-400" />
    }
  ];

  const workflowSteps = [
    {
      step: 1,
      title: "Idee & Ziel definieren",
      description: "Ziel klar formulieren: Funktionalität & Kernfeatures (Login, Musik speichern, Settings) auflisten.",
      example: "Ich möchte eine App, die meine Lieblingsmusik speichert, sortiert und offline abspielen kann.",
      icon: <Lightbulb className="w-6 h-6 text-yellow-400" />
    },
    {
      step: 2,
      title: "Planung & Struktur mit KI",
      description: "Replit AI öffnen. Sprache wählen (JS / Python / Flutter). Die KI plant Ordner, Dateien, Klassen und Grundlogik.",
      example: "Bitte plane die komplette Struktur, Ordner, Dateien, Klassen und Grundlogik für die Musik-App.",
      icon: <Monitor className="w-6 h-6 text-blue-400" />
    },
    {
      step: 3,
      title: "Code-Generierung mit KI",
      description: "Die KI schreibt den Code für jedes Feature. Direkt online testen: 'Run'-Button drücken und Vorschau prüfen.",
      example: "Schreibe mir den Code für das Login-Feature in Flutter.",
      icon: <Code className="w-6 h-6 text-emerald-400" />
    },
    {
      step: 4,
      title: "Benutzeroberfläche gestalten (optional)",
      description: "Die KI erstellt Widgets, Buttons, Menüs und das komplette Theme für die Benutzeroberfläche.",
      example: "Baue eine Login-Seite mit Nutzername, Passwort und Login-Button passend zum Dark-Theme.",
      icon: <TerminalSquare className="w-6 h-6 text-purple-400" />
    },
    {
      step: 5,
      title: "Build / Fertige Software erstellen",
      description: "Code für Zielplattform exportieren/kompilieren (z.B. Python -> PyInstaller für EXE, Flutter -> APK für Android).",
      example: "Exportiere als EXE über Replit Shell: pyinstaller main.py",
      icon: <Hammer className="w-6 h-6 text-orange-400" />
    },
    {
      step: 6,
      title: "Testen & Feedback",
      description: "Software auf dem Zielgerät testen. Fehlerbeschreibungen an die KI zurückfüttern, um Bugs zu beheben.",
      example: "Behebe den Fehler: Die App stürzt beim Musik abspielen ab.",
      icon: <GitPullRequest className="w-6 h-6 text-pink-400" />
    },
    {
      step: 7,
      title: "Deployment & Verteilung (optional)",
      description: "Automatisches Hosting via Replit/GitHub Pages für Web-Apps, oder Teilen von EXE/APK via Cloud-Speicher.",
      example: "App kostenlos auf GitHub Pages hosten & Link teilen.",
      icon: <Rocket className="w-6 h-6 text-cyan-400" />
    }
  ];

  return (
    <div className="w-full h-full bg-slate-900 border border-slate-800 rounded-lg p-6 overflow-y-auto">
      <div className="flex items-center gap-3 mb-2">
        <Bot className="w-8 h-8 text-atc-cyan" />
        <h2 className="text-2xl font-mono text-white tracking-widest uppercase">Kostenlose KI-Code-Generatoren</h2>
      </div>
      <p className="text-slate-400 mb-8 max-w-3xl">
        Eine präzise kategorisierte Liste von online nutzbaren KIs, um direkt Code/Softwareprojekte zu erstellen (ohne Abo).
      </p>

      {/* Tools Section */}
      <div className="mb-12">
        <h3 className="text-lg font-bold text-white mb-4 border-b border-slate-800 pb-2 flex items-center gap-2">
          <TerminalSquare className="w-5 h-5 text-slate-400" />
          1. Sofort online nutzbare, kostenlose Tools
        </h3>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          {tools.map((tool, idx) => (
            <motion.div 
              key={idx}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: idx * 0.1 }}
              className="bg-slate-800/40 border border-slate-700/50 p-4 rounded-lg hover:border-atc-cyan/50 transition-colors"
            >
              <div className="flex items-center gap-3 mb-2">
                {tool.icon}
                <h4 className="font-bold text-slate-200">{tool.name}</h4>
              </div>
              <div className="space-y-1 text-sm">
                <p><span className="text-slate-500">Typ:</span> <span className="text-slate-300">{tool.type}</span></p>
                <p><span className="text-slate-500">Features:</span> <span className="text-emerald-400/80">{tool.features}</span></p>
                <p className="text-red-400/80"><span className="text-slate-500">Limit:</span> {tool.limits}</p>
              </div>
            </motion.div>
          ))}
        </div>
        
        <div className="mt-4 grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-green-500/10 border border-green-500/20 p-4 rounded-lg text-sm text-green-200/80">
              <h4 className="flex items-center gap-2 font-bold text-green-400 mb-2">
                <CheckCircle className="w-4 h-4" /> Was diese Tools leisten
              </h4>
              <ul className="list-disc pl-5 space-y-1">
                <li>Sofort online verfügbar</li>
                <li>Code generieren (Funktionen, Module, UI)</li>
                <li>Projektstruktur & Tests</li>
                <li>Vorschläge & Refactoring</li>
              </ul>
            </div>
            
            <div className="bg-red-500/10 border border-red-500/20 p-4 rounded-lg text-sm text-red-200/80">
              <h4 className="flex items-center gap-2 font-bold text-red-400 mb-2">
                <AlertTriangle className="w-4 h-4" /> Was sie NICHT machen
              </h4>
              <ul className="list-disc pl-5 space-y-1">
                <li>Fertiger EXE/APK Download (ohne eigenes Build)</li>
                <li>Automatisches Hosting/Release ohne Nutzereingriff</li>
                <li>(Bezahlt-Tools gemieden: OpenAI GPT-4 API)</li>
              </ul>
            </div>
        </div>
      </div>

      {/* Workflow Section */}
      <div className="mb-12">
        <h3 className="text-lg font-bold text-white mb-6 border-b border-slate-800 pb-2 flex items-center gap-2">
          <ArrowRight className="w-5 h-5 text-slate-400" />
          2. Praxis-Workflow ohne Kosten
        </h3>
        <div className="space-y-6 relative before:absolute before:inset-0 before:ml-6 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-slate-700 before:to-transparent">
          {workflowSteps.map((step, idx) => (
            <motion.div 
              key={idx}
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: idx * 0.15 + 0.3 }}
              className="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active"
            >
              <div className="flex items-center justify-center w-12 h-12 rounded-full border-4 border-slate-900 bg-slate-800 text-slate-400 shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 z-10">
                {step.icon}
              </div>
              <div className="w-[calc(100%-4rem)] md:w-[calc(50%-3rem)] bg-slate-800/40 border border-slate-700/50 p-4 rounded-lg shadow-sm">
                <div className="flex items-center justify-between mb-1">
                  <h4 className="font-bold text-atc-cyan">Schritt {step.step}: {step.title}</h4>
                </div>
                <p className="text-slate-300 text-sm mb-2">{step.description}</p>
                <div className="bg-slate-900/50 p-2 rounded text-xs font-mono text-slate-400 border border-slate-800">
                  <span className="text-slate-500">Aktion: </span>"{step.example}"
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </div>

      {/* Extra Tips */}
      <div className="bg-gradient-to-r from-atc-cyan/20 to-blue-600/20 border border-atc-cyan/30 rounded-lg p-6 relative overflow-hidden">
        <div className="absolute -right-4 -top-4 opacity-10">
          <Lightbulb className="w-48 h-48" />
        </div>
        <h3 className="text-xl font-bold text-white mb-4 flex items-center gap-2">
          <Zap className="w-6 h-6 text-yellow-400" />
          Extra Tipps für Non-IT-Nutzer
        </h3>
        <ul className="space-y-3 relative z-10 text-sm text-slate-200">
          <li className="flex items-start gap-2">
            <CheckCircle className="w-5 h-5 text-green-400 shrink-0 mt-0.5" />
            <div><strong className="text-white">Einfache Sprache nutzen:</strong> Sprich mit der KI wie mit einem Assistenten ("Ich möchte...", "Füge hinzu...", "Mach einfacher...").</div>
          </li>
          <li className="flex items-start gap-2">
            <CheckCircle className="w-5 h-5 text-green-400 shrink-0 mt-0.5" />
            <div><strong className="text-white">Schritt für Schritt (Modularität):</strong> Jedes Feature einzeln generieren → weniger Fehler, besseres Debugging.</div>
          </li>
          <li className="flex items-start gap-2">
            <CheckCircle className="w-5 h-5 text-green-400 shrink-0 mt-0.5" />
            <div><strong className="text-white">Kontinuierliches Testen:</strong> Teste regelmäßig online in der Sandbox, bevor du einen komplexen Export/Build durchführst.</div>
          </li>
          <li className="flex items-start gap-2">
            <CheckCircle className="w-5 h-5 text-green-400 shrink-0 mt-0.5" />
            <div><strong className="text-white">Regelmäßige Backups:</strong> Plattformen wie Replit speichern automatisch, aber den Code gelegentlich lokal als .zip zu sichern ist Best Practice.</div>
          </li>
          <li className="flex items-start gap-2">
            <CheckCircle className="w-5 h-5 text-green-400 shrink-0 mt-0.5" />
            <div><strong className="text-white">Kostenlos bleiben:</strong> Nur Open-Source & Free-Tier Tools verwenden (Replit Free, StarCoder, GitHub Free). Keine Kreditkarten hinterlegen.</div>
          </li>
        </ul>
        <div className="mt-6 pt-4 border-t border-atc-cyan/20">
          <button className="bg-atc-cyan hover:bg-cyan-400 text-slate-900 font-bold px-6 py-2 rounded-md font-mono text-sm shadow-[0_0_15px_rgba(34,211,238,0.4)] transition-all">
            Beispiel-Workflow (Idea-to-App) starten
          </button>
        </div>
      </div>

    </div>
  );
};