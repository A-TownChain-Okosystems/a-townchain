import React, { useState } from 'react';
import { Book, Code, Terminal, Key, Shield, Database, Webhook, Zap, FileJson, Brain, Network, Play, CheckCircle, Github, Activity } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { JsExampleRunner } from './JsExampleRunner';

function ApiExplorerWidget() {
  const [endpoint, setEndpoint] = useState('/v1/chain/status');
  const [method, setMethod] = useState('GET');
  const [response, setResponse] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleTest = () => {
    setLoading(true);
    setResponse(null);
    setTimeout(() => {
      let resData = '';
      if (endpoint === '/v1/chain/status') {
        resData = JSON.stringify({ status: 'online', currentBlock: 1400329, network: 'testnet', atvVersion: '1.2.0' }, null, 2);
      } else if (endpoint === '/v1/ai/orchestrator/health') {
        resData = JSON.stringify({ status: 'healthy', activeAgents: 42, queuedTasks: 5, lastCheck: new Date().toISOString() }, null, 2);
      } else if (endpoint === '/v1/contracts/deploy') {
        resData = JSON.stringify({ error: 'Unauthorized', message: 'Bearer token missing or invalid' }, null, 2);
      }
      setResponse(resData);
      setLoading(false);
    }, 800);
  };

  return (
    <div className="mt-4 p-5 bg-[#050608] border border-atc-border rounded-xl">
      <div className="flex flex-col sm:flex-row gap-3 mb-4">
        <select 
          value={method} 
          onChange={(e) => setMethod(e.target.value)}
          className="bg-[#090b14] border border-atc-border text-slate-300 text-sm font-mono rounded-lg px-3 py-2 outline-none focus:border-atc-purple"
        >
          <option>GET</option>
          <option>POST</option>
        </select>
        <select 
          value={endpoint} 
          onChange={(e) => setEndpoint(e.target.value)}
          className="flex-1 bg-[#090b14] border border-atc-border text-slate-300 text-sm font-mono rounded-lg px-3 py-2 outline-none focus:border-atc-purple"
        >
          <option value="/v1/chain/status">/v1/chain/status</option>
          <option value="/v1/ai/orchestrator/health">/v1/ai/orchestrator/health</option>
          <option value="/v1/contracts/deploy">/v1/contracts/deploy</option>
        </select>
        <button 
          onClick={handleTest}
          disabled={loading}
          className="flex items-center gap-2 bg-atc-purple hover:bg-atc-purple/80 text-white px-4 py-2 rounded-lg font-medium transition-colors disabled:opacity-50"
        >
          <Play className="w-4 h-4" /> {loading ? 'Testing...' : 'Send'}
        </button>
      </div>
      <div className="relative min-h-[120px] bg-black/40 border border-white/5 rounded-lg p-4 font-mono text-xs overflow-auto">
        {!response && !loading && <span className="text-slate-600">Hit Send to fetch response...</span>}
        {loading && <span className="text-slate-400 animate-pulse">Awaiting gateway...</span>}
        {response && (
          <pre className={response.includes('error') ? 'text-red-400' : 'text-emerald-400'}>
            {response}
          </pre>
        )}
      </div>
    </div>
  );
}

const DOCS_DATA = [
  {
    id: 'master-architecture',
    title: 'ATC Master Ecosystem',
    icon: Database,
    sections: [
      {
        title: '30-Layer Production Kernel Integration Layer (PKIL)',
        content: 'Das ATC Ecosystem ist nicht als einfache Blockchain konzipiert, sondern als vollständiges ATC-OS Operating System. Es umfasst 30 aufeinander aufbauende Core-Layer, über 150 verteilte Subsysteme und mehr als 10 native Dateiformate. \n\nDie Integration verläuft vom Boot System (L1), durch die Bridge Compatibility Layer (L3), zur ATC VM (L5) bis hin zu P2P Network (L9), AI Layer (L14) und schließlich zur Globalen Autonomen Economy (L29). Die PKIL (Layer 30) verbindet alle Systeme für Mainnet-Bereitschaft.'
      },
      {
        title: 'Native Dateisystem-Architektur',
        content: 'Das Ökosystem verwendet spezielle Formate zur Isolierung und Sicherheit:\n- `.atc`: Binäre Executables\n- `.atvm`: ATC Virtual Machine Module\n- `.aes`: Execution Units (Sandboxed)\n- `.ata`: AI Agenten Files\n- `.ats` & `.atsdb`: State & Storage Journals\n- `.atn`: P2P Network Configs\n- `.atk`: Kryptografische Schlüsselspeicher (MPC)\n- `.atg`: GPU Shader Definitionen\n- `.atb`: Bridge-Konnektoren\n- `.atpkg`: ATC Packages'
      }
    ]
  },
  {
    id: 'kernel-runtime',
    title: 'OS Kernel & Runtime (L1-L5)',
    icon: Terminal,
    sections: [
      {
        title: 'Layer 1 & 2: Hardware Foundation & Kernel Core',
        content: 'Das ATC Kernel-System regelt Hardware-Ressourcen: Memory Management Unit (MMU), Boot System, Virtual Memory und Custom Device Drivers. Der Runtime Dispatcher (L2) übernimmt Multitasking und Process Management für alle laufenden Services.'
      },
      {
        title: 'Layer 3: Bridge Compatibility Layer (BCL)',
        content: 'Alle externen Schnittstellen (Module A zu Module B) müssen durch die BCL geroutet werden. Direkte Cross-Modul-Kommunikation ist auf Protokollebene verboten. Formate werden über Schema Translations in das `.atb` Format normiert.'
      },
      {
        title: 'Layer 4 & 5: ATC VM & Execution',
        content: 'Die ATC VM führt Bytegraph-basierten Code deterministisch in einer Sandbox aus. Die VM Metering Engine überwacht den Gas- und Ressourcenverbrauch in Echtzeit, während der State Access Layer kontrollierten Zugriff auf den Ledger gewährt.'
      }
    ]
  },
  {
    id: 'network-consensus',
    title: 'Network, Storage & Consensus (L8-L10)',
    icon: Network,
    sections: [
      {
        title: 'ATC Transport Protocol (ATP) [L9]',
        content: 'Alle Nodes kommunizieren über P2P mit Kademlia DHT Discovery und Gossip Protokollen. Zustandsaktualisierungen verteilen sich via `.atn` Pakete.'
      },
      {
        title: 'Proof of AI (PoAI) [L10]',
        content: 'Der finale Konsens (PoI + PoS) wird durch eine KI-Überprüfung (PoAI) ergänzt. Die Modelle validieren Blöcke, bewerten Smart Contract Sicherheitsregeln und überwachen anomale Transaktionsmuster via Aurora Orchestrator.'
      }
    ]
  },
  {
    id: 'gateway-api',
    title: 'API Gateway Integration',
    icon: Webhook,
    sections: [
      {
        title: 'Architektur & Endpunkte',
        content: 'Das zentrale API Gateway fungiert als Vermittler zwischen dem A-TownChain Explorer Dashboard und den internen A-TownChain Services. Es übernimmt Rate-Limiting, Authentifizierung und Service-Routing (Core, Chain, Wallet, AI). \n\nBasis-URL: `https://api.a-townchain.network/v1`'
      },
      {
        title: 'Authentifizierung',
        content: 'Alle Client-Anfragen müssen signiert werden. Der API-Schlüssel oder das generierte Session-Token des Wallets muss als Bearer-Token im Header übergeben werden.\n\n`Authorization: Bearer <ATC-Auth-Token>`'
      },
      {
        title: 'Rate Limits',
        content: 'Die Standard-Richtlinien sind:\n- `Core & Chain`: 1000 Anfragen / Minute\n- `Aurora AI`: 50 Anfragen / Minute\n\nÜberschreitungen führen zu einer `429 Too Many Requests` Antwort mit `Retry-After` Header.'
      }
    ]
  },
  {
    id: 'smart-contracts',
    title: 'Smart Contracts',
    icon: FileJson,
    sections: [
      {
        title: 'EVM Kompatibilität & Native ATC Contracts',
        content: 'Die Engine unterstützt vollständige EVM-Kompatibilität, wodurch bestehende atc-lang-Contracts nativ in der A-TownChain deployed werden können. Zusätzlich priorisiert das System native ATC-Extensions (ATS-1, ATS-20, ATS-721).'
      },
      {
        title: 'ATC-9000 (ATCAsset Standard)',
        content: 'ATC-9000 ist ein spezialisierter dynamischer NFT-Standard für in-chain Gamification. ATCAssets (Assets) entwickeln sich basierend auf On-Chain-Metriken (Erfahrung, Transaktionshistorie des Wallets). Interaktionen erfolgen per `/contracts/{address}/evolve`.'
      }
    ]
  },
  {
    id: 'aurora-ai',
    title: 'ATS Autonomous AI (L14)',
    icon: Brain,
    sections: [
      {
        title: 'LLM Orchestrierung & Governance',
        content: 'Die ATS Autonomous AI Engine fungiert als intelligentes Überwachungssystem, das lokale Modelle und Cloud-LLMs kombiniert. Sie überwacht die Runtime, analysiert Governance-Vorschläge und validiert die Systemwahrheit (Universal Verification Layer).'
      },
      {
        title: 'Agenten-Kommunikation',
        content: 'Agenten innerhalb des Franchises kommunizieren über Message-Queues als Schwarm. Die API erlaubt autonome Delegationen von Aufgaben (z.B. AI-gestützte Fehlerbehebung im Mainnet).'
      }
    ]
  },
  {
    id: 'atc-lang-examples',
    title: 'atc-lang Setup & API Examples',
    icon: Code,
    sections: [
      {
        title: 'Querying Network Status',
        content: 'Dieser interaktive atc-lang Snippet zeigt, wie der aktuelle Netzwerkstatus über das Gateway abgefragt wird:',
        interactiveJsType: 'network'
      },
      {
        title: 'Fetching Node Information',
        content: 'Um Daten zu aktiven Nodes abzurufen:',
        interactiveJsType: 'nodes'
      }
    ]
  },
  {
    id: 'interactive-api-docs',
    title: 'Interactive API Explorer',
    icon: Code,
    sections: [
      {
        title: 'Gateway Live Tester',
        content: 'Nutzen Sie die interaktive Konsole unten, um simulierte Anfragen an das A-TownChain API Gateway zu senden. Wählen Sie einen Endpunkt und überprüfen Sie die formatierte JSON Antwort live.',
        interactive: true
      }
    ]
  },
  {
    id: 'ci-cd-pipeline',
    title: 'CI/CD Pipeline & Build Tools',
    icon: Github,
    sections: [
      {
        title: 'Automated Build Checks',
        content: 'Das Projekt nutzt umfassende CI/CD-Pipelines via GitHub Actions. Jeder Commit auf den main-Branch triggert einen automatisierten Produktions-Build, bei dem "atpm run build" ausgeführt wird, welches durch Vite und esbuild das atc-lang Bundle (server.cjs) sowie die atc-ui App transpiliert. Sollten atc-lang Syntaxfehler oder unzulässige Modulkombinationen vorliegen, bricht der Build mit einem Exit Status 1 (Failure) ab.'
      },
      {
        title: 'Reproducing Build Failures Locally',
        content: `Um Produktionsfehler lokal zu debuggen:\n1. Pulle den neusten Zustand aus dem Repository.\n2. Lösche "node_modules" und run "atpm install".\n3. Führe "atpm run build" identisch zur Build-Server Umgebung aus, oder verwende "atpm run lint" um statische Codeanalyse-Fehler frühzeitig zu erkennen.\n4. Prüfe die Logs nach ESBuild Transformation Errors. Die häufigsten Fehler sind hier ungeschlossene Tags in JSX ("The character '}' is not valid inside a JSX element") oder fehlende Imports.`
      }
    ]
  },
  {
    id: 'build-performance',
    title: 'Build Performance',
    icon: Activity,
    sections: [
      {
        title: 'Current Build Analysis',
        content: 'Die aktuelle Vite Produktions-Build Analyse identifizierte Asset-Mapping Optimierungspotentiale. Warnungen bezüglich Chunks größer als 500 kB nach Minification ("(!) Some chunks are larger than 500 kB after minification") deuten auf fehlendes Code-Splitting hin. Zudem wurde ein Warnhinweis gefunden bezüglich der Verwendung von "eval()" in Recharts-Komponenten (was Sicherheitsrisiken birgt).'
      },
      {
        title: 'Asset-Optimization Strategies',
        content: `Um Build-Performance und initiale Ladezeit zu verbessern:\n1. Nutzen Sie "build.rollupOptions.output.manualChunks" in "vite.config.ts", um große externe Libraries wie "react" in separate Vendor-Chunks auszulagern.\n2. Vermeiden Sie gemischte (statische und dynamische) Imports derselben Library, um doppelte Pakete (wie bei jspdf beobachtet) zu verhindern.\n3. Verkleinern Sie die Chunk Size, indem Sie UI-Schwergewichte über dynamisches Importieren laden (z.B. für Dashboard Graphen).`
      }
    ]
  }
];

export function TechDocsView() {
  const [activeDoc, setActiveDoc] = useState(DOCS_DATA[0].id);

  const activeContent = DOCS_DATA.find(d => d.id === activeDoc);

  return (
    <div className="flex flex-col gap-6 mt-6 pb-12 w-full max-w-6xl mx-auto h-[800px]">
      <div className="flex items-center gap-3 mb-2">
        <div className="w-10 h-10 rounded-lg bg-atc-purple/10 border border-atc-purple/20 flex items-center justify-center text-atc-purple">
          <Book className="w-5 h-5" />
        </div>
        <div>
          <h2 className="text-2xl font-semibold tracking-tight text-white">Technische Dokumentation</h2>
          <p className="text-sm font-light text-slate-400">API Referenz, Integrationen und Entwickler-Ressourcen</p>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl shadow-2xl relative">
        <div className="absolute top-0 right-0 w-64 h-64 bg-atc-cyan/5 rounded-full blur-[100px] pointer-events-none" />
        {/* Sidebar */}
        <div className="w-64 border-r border-atc-border/50 bg-[#090b14]/60 flex flex-col p-4 gap-2 overflow-y-auto custom-scrollbar relative z-10">
          <h3 className="text-[10px] font-mono text-slate-500 uppercase tracking-widest pl-2 mb-2">Themen</h3>
          {DOCS_DATA.map((doc) => {
            const Icon = doc.icon;
            const isActive = activeDoc === doc.id;
            return (
              <button
                key={doc.id}
                onClick={() => setActiveDoc(doc.id)}
                className={`flex items-center gap-3 w-full px-3 py-2.5 rounded-lg text-left transition-all ${
                  isActive 
                    ? 'bg-atc-purple/10 border border-atc-purple/30 text-atc-cyan font-medium shadow-[0_0_10px_rgba(162,89,255,0.1)]'
                    : 'border border-transparent text-slate-400 hover:text-slate-200 hover:bg-atc-border/50'
                }`}
              >
                <Icon className={`w-4 h-4 ${isActive ? 'text-atc-cyan' : 'text-slate-500'}`} />
                <span className="text-sm">{doc.title}</span>
              </button>
            );
          })}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-8 bg-transparent relative z-10">
          <AnimatePresence mode="wait">
            {activeContent && (
              <motion.div
                key={activeContent.id}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                transition={{ duration: 0.2 }}
                className="max-w-3xl"
              >
                <div className="flex items-center gap-4 mb-8 pb-6 border-b border-atc-border">
                  <div className="p-3 bg-atc-purple/10 rounded-xl border border-atc-purple/20 text-atc-purple">
                    <activeContent.icon className="w-6 h-6" />
                  </div>
                  <h1 className="text-3xl font-semibold text-white tracking-tight">{activeContent.title}</h1>
                </div>

                <div className="flex flex-col gap-8">
                  {activeContent.sections.map((section, idx) => (
                    <div key={idx} className="flex flex-col gap-3">
                      <h3 className="text-xl font-medium text-slate-200">{section.title}</h3>
                      <div className="text-slate-400 font-light leading-relaxed whitespace-pre-wrap">
                        {section.content.split('```').map((blockPart, blockIdx) => {
                          if (blockIdx % 2 === 1) {
                            // Trim the language identifier (e.g., "atc-lang") if present
                            const codeContent = blockPart.replace(/^[a-z]+\n/, '');
                            return (
                              <pre key={blockIdx} className="my-3 p-4 rounded-xl bg-black/50 border border-white/10 overflow-x-auto">
                                <code className="text-emerald-400 font-mono text-sm leading-relaxed">{codeContent}</code>
                              </pre>
                            );
                          }
                          // Process inline backticks for the text parts
                          return (
                            <span key={blockIdx}>
                              {blockPart.split('`').map((inlinePart, inlineIdx) => {
                                if (inlineIdx % 2 === 1) {
                                  return (
                                    <code key={inlineIdx} className="px-1.5 py-0.5 mx-1.5 rounded-md bg-[#05080f] border border-atc-border text-atc-cyan font-mono text-[13px]">
                                      {inlinePart}
                                    </code>
                                  );
                                }
                                return <span key={inlineIdx}>{inlinePart}</span>;
                              })}
                            </span>
                          );
                        })}
                      </div>
                      {(section as any).interactive && <ApiExplorerWidget />}
                      {(section as any).interactiveJsType && <JsExampleRunner type={(section as any).interactiveJsType} />}
                    </div>
                  ))}
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
}
