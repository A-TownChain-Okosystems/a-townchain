import React, { useState } from 'react';
import { Database, Search, Layers, FileJson, Code, TerminalSquare, GitMerge, FileText, Blocks, BrainCircuit, Activity } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

export function DeveloperKnowledgeBaseView() {
  const [activeTab, setActiveTab] = useState<'architecture' | 'search' | 'schema' | 'workflow'>('architecture');

  const jsonSchemaRaw = `{
  "project": "A-Town OS",
  "version": "1.0.0",
  "modules": [
    {
      "name": "Mining Engine",
      "features": ["GPU Scaling", "Hashrate Monitor"],
      "code_blocks": ["src/mining/engine.ts"],
      "dependencies": ["CoreSystem"],
      "todos": ["Implement real-time graph"],
      "risks": ["Memory leak on long sessions"]
    }
  ],
  "global_todos": ["Add localization"],
  "architecture": {
    "auth": "Firebase Auth",
    "db": "Chroma Vector Storage"
  },
  "notes": ["Discuss optimizations for next version"]
}`;

  const pythonScriptRaw = `import json
from sentence_transformers import SentenceTransformer
import chromadb

def build_knowledge_base():
    client = chromadb.PersistentClient(path="./dkb_storage")
    collection = client.get_or_create_collection("dev_chats")
    model = SentenceTransformer('all-MiniLM-L6-v2')
    
    # Load extracted chat JSONs
    with open('parsed_chats.json') as f:
        data = json.load(f)
        
    for idx, feature in enumerate(data['modules']):
        text = str(feature)
        embedding = model.encode(text).tolist()
        
        collection.add(
            embeddings=[embedding],
            documents=[text],
            metadatas=[{"module": feature["name"]}],
            ids=[f"module_{idx}"]
        )
    print("Knowledge Base synchronized and vectorized!")`;

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-200 font-sans p-6 overflow-hidden">
      
      {/* Header */}
      <div className="flex-none border-b border-slate-800 pb-6 mb-6">
        <div className="flex items-center gap-4">
          <div className="p-3 bg-indigo-500/10 text-indigo-400 rounded-xl border border-indigo-500/20">
            <BrainCircuit className="w-8 h-8" />
          </div>
          <div>
            <h1 className="text-2xl font-bold tracking-tight text-white">Developer Intelligence System</h1>
            <p className="text-slate-400 text-sm mt-1">Self-hosted Developer Knowledge Base (DKB)</p>
          </div>
          <div className="ml-auto flex gap-2">
            <span className="px-3 py-1 bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-xs font-bold uppercase tracking-wider rounded-full flex items-center gap-1.5">
              <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
              ChromaDB Online
            </span>
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex gap-2 mb-6 flex-none">
        {[
          { id: 'architecture', label: 'Architektur', icon: Blocks },
          { id: 'search', label: 'Vector Query Engine', icon: Search },
          { id: 'schema', label: 'Parser & Schema', icon: FileJson },
          { id: 'workflow', label: 'Reifegrad & Workflow', icon: Activity },
        ].map(tab => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id as any)}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition-all flex items-center gap-2 border ${
              activeTab === tab.id 
                ? 'bg-indigo-500/20 border-indigo-500/30 text-indigo-300' 
                : 'bg-slate-900 border-slate-800 text-slate-400 hover:text-slate-200 hover:bg-slate-800'
            }`}
          >
            <tab.icon className="w-4 h-4" />
            {tab.label}
          </button>
        ))}
      </div>

      {/* Content Area */}
      <div className="flex-1 overflow-auto min-h-0 bg-slate-900/50 rounded-xl border border-slate-800 p-6">
        <AnimatePresence mode="wait">
          
          {/* ARCHITECTURE */}
          {activeTab === 'architecture' && (
            <motion.div 
              key="arch" 
              initial={{opacity: 0, y: 10}}
              animate={{opacity: 1, y: 0}}
              className="max-w-4xl mx-auto space-y-6"
            >
              <h2 className="text-xl font-bold text-white mb-8 border-b border-slate-800 pb-2">Enterprise DKB Kernarchitektur</h2>
              
              <div className="space-y-4">
                <div className="flex gap-4 items-stretch">
                  <div className="w-12 flex-none flex items-center justify-center bg-slate-800 rounded-lg text-slate-400 font-bold border border-slate-700">L5</div>
                  <div className="flex-1 bg-slate-800/60 p-4 rounded-lg border border-slate-700">
                    <div className="flex items-center gap-2 mb-2 text-indigo-400 font-semibold">
                      <TerminalSquare className="w-5 h-5" />
                      <h3>Interface</h3>
                    </div>
                    <p className="text-xs text-slate-400">Web Dashboard & CLI Tool, integration in A-Town / AT-OS.</p>
                  </div>
                </div>

                <div className="flex gap-4 items-stretch">
                  <div className="w-12 flex-none flex items-center justify-center bg-slate-800 rounded-lg text-slate-400 font-bold border border-slate-700">L4</div>
                  <div className="flex-1 bg-slate-800/60 p-4 rounded-lg border border-slate-700">
                    <div className="flex items-center gap-2 mb-2 text-purple-400 font-semibold">
                      <BrainCircuit className="w-5 h-5" />
                      <h3>Intelligence</h3>
                    </div>
                    <p className="text-xs text-slate-400">Semantic Code Search, Feature-Linking, automatische Refactoring-Vorschläge.</p>
                  </div>
                </div>

                <div className="flex gap-4 items-stretch">
                  <div className="w-12 flex-none flex items-center justify-center bg-slate-800 rounded-lg text-slate-400 font-bold border border-slate-700">L3</div>
                  <div className="flex-1 bg-slate-800/60 p-4 rounded-lg border border-slate-700">
                    <div className="flex items-center gap-2 mb-2 text-emerald-400 font-semibold">
                      <Database className="w-5 h-5" />
                      <h3>Storage</h3>
                    </div>
                    <p className="text-xs text-slate-400">ChromaDB (Vector Search) + aggregierte JSON / Markdown Artefakte.</p>
                  </div>
                </div>

                <div className="flex gap-4 items-stretch">
                  <div className="w-12 flex-none flex items-center justify-center bg-slate-800 rounded-lg text-slate-400 font-bold border border-slate-700">L2</div>
                  <div className="flex-1 bg-slate-800/60 p-4 rounded-lg border border-slate-700">
                    <div className="flex items-center gap-2 mb-2 text-orange-400 font-semibold">
                      <GitMerge className="w-5 h-5" />
                      <h3>Processing</h3>
                    </div>
                    <p className="text-xs text-slate-400">Python Parser, Extraktion, NLP Klassifizierung.</p>
                  </div>
                </div>

                <div className="flex gap-4 items-stretch">
                  <div className="w-12 flex-none flex items-center justify-center bg-slate-800 rounded-lg text-slate-400 font-bold border border-slate-700">L1</div>
                  <div className="flex-1 bg-slate-800/60 p-4 rounded-lg border border-slate-700">
                    <div className="flex items-center gap-2 mb-2 text-blue-400 font-semibold">
                      <Layers className="w-5 h-5" />
                      <h3>Input</h3>
                    </div>
                    <p className="text-xs text-slate-400">ChatGPT Chats, Source Code (Flutter/Python), Issues & ToDos.</p>
                  </div>
                </div>
              </div>
            </motion.div>
          )}

          {/* SEARCH MOCK */}
          {activeTab === 'search' && (
            <motion.div 
              key="search"
              initial={{opacity: 0, y: 10}}
              animate={{opacity: 1, y: 0}}
              className="max-w-4xl mx-auto flex flex-col h-full"
            >
              <div className="bg-slate-800/80 p-4 rounded-xl border border-slate-700 mb-6 flex gap-3 shadow-lg items-center">
                <Search className="w-5 h-5 text-slate-400" />
                <input 
                  type="text" 
                  value="Zeige alle Mining Engine Features und relatierten Code" 
                  readOnly
                  className="bg-transparent border-0 flex-1 text-white outline-none font-medium"
                />
                <button className="px-4 py-1.5 bg-indigo-500 hover:bg-indigo-600 text-white text-sm font-medium rounded-lg">
                  Vector Query
                </button>
              </div>

              <div className="text-sm font-bold text-slate-400 mb-4 px-2 uppercase tracking-wide">Semantische Ergebnisse (Mock)</div>
              
              <div className="space-y-4 flex-1 overflow-auto pr-2 pb-8">
                {[
                  {
                    title: 'Mining Engine Module',
                    version: 'v1.0.0',
                    category: 'Core System',
                    subcategory: 'Background Workers',
                    match: '98.4%',
                    desc: 'Beinhaltet Hashrate Monitor, GPU Scaling logic. Referenzen in src/mining/engine.ts.',
                    badges: ['Module', 'Implemented'],
                    tags: ['mining', 'gpu', 'engine']
                  },
                  {
                    title: 'Optimierung: Memory Leak on Long Sessions',
                    version: 'v1.0.1-patch',
                    category: 'Performance',
                    subcategory: 'Memory Management',
                    match: '82.1%',
                    desc: 'Risiko im Chat vom 24.10. erwähnt. Task: Implement garbage collection in engine ticks.',
                    badges: ['Risk', 'Open ToDo'],
                    tags: ['bug', 'memory', 'leak']
                  },
                  {
                    title: 'Real-time Graph Implementation',
                    version: 'v2.0.0-draft',
                    category: 'User Interface',
                    subcategory: 'Data Visualization',
                    match: '75.3%',
                    desc: 'Architekturentscheidung: D3.js für Real-time Graph in der Mining UI nutzen.',
                    badges: ['Architecture', 'Idea'],
                    tags: ['d3', 'd3js', 'graphs', 'realtime']
                  }
                ].map((res, i) => (
                  <div key={i} className="bg-slate-800/40 border border-slate-700 hover:border-indigo-500/50 p-4 rounded-xl cursor-pointer transition-colors group">
                    <div className="flex justify-between items-start mb-2">
                       <div>
                         <h3 className="text-indigo-400 font-bold group-hover:text-indigo-300">{res.title} <span className="text-slate-500 text-xs font-mono font-normal">({res.version})</span></h3>
                         <div className="text-[10px] text-slate-500 uppercase tracking-widest mt-1">
                           {res.category} <span className="text-slate-600">/</span> {res.subcategory}
                         </div>
                       </div>
                       <span className="text-xs font-mono text-emerald-400 bg-emerald-400/10 px-2 py-0.5 rounded-full border border-emerald-400/20">{res.match}</span>
                    </div>
                    <p className="text-sm text-slate-300 mb-3">{res.desc}</p>
                    <div className="flex flex-wrap gap-2 items-center">
                      <div className="flex gap-1.5 border-r border-slate-700 pr-3 mr-1">
                        {res.badges.map(b => (
                          <span key={b} className="text-[10px] uppercase tracking-wide px-2 py-0.5 bg-slate-700/50 text-slate-400 rounded border border-slate-600">{b}</span>
                        ))}
                      </div>
                      <div className="flex gap-1.5">
                        {res.tags.map(t => (
                          <span key={t} className="text-[9px] uppercase font-mono tracking-wider px-1.5 py-0.5 bg-indigo-500/10 text-indigo-400 rounded-sm">#{t}</span>
                        ))}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </motion.div>
          )}

          {/* SCHEMA & PARSER */}
          {activeTab === 'schema' && (
            <motion.div 
              key="schema"
              initial={{opacity: 0, y: 10}}
              animate={{opacity: 1, y: 0}}
              className="max-w-4xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-6"
            >
              <div className="space-y-3">
                <div className="flex items-center gap-2 text-cyan-400 font-semibold mb-2">
                  <FileJson className="w-5 h-5" />
                  <h2>Core JSON Schema</h2>
                </div>
                <p className="text-xs text-slate-400 mb-4">Das maschinenlesbare Format extrahiert aus den Metadaten der Chats.</p>
                <pre className="p-4 bg-slate-950 text-cyan-300 text-xs font-mono rounded-xl border border-slate-800 overflow-auto max-h-[400px]">
                  {jsonSchemaRaw}
                </pre>
              </div>

              <div className="space-y-3">
                <div className="flex items-center gap-2 text-orange-400 font-semibold mb-2">
                  <Code className="w-5 h-5" />
                  <h2>Python Integration (ChromaDB)</h2>
                </div>
                <p className="text-xs text-slate-400 mb-4">Generiert Embeddings (Semantic Search) für die geparsten Module.</p>
                <pre className="p-4 bg-slate-950 text-orange-300 text-xs font-mono rounded-xl border border-slate-800 overflow-auto max-h-[400px]">
                  {pythonScriptRaw}
                </pre>
              </div>
            </motion.div>
          )}

          {/* WORKFLOW */}
          {activeTab === 'workflow' && (
            <motion.div 
              key="workflow"
              initial={{opacity: 0, y: 10}}
              animate={{opacity: 1, y: 0}}
              className="max-w-4xl mx-auto space-y-12"
            >
              <div>
                <h3 className="text-lg font-bold text-white mb-6 border-b border-slate-800 pb-2">Reifegrad-Modell</h3>
                <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
                  {[
                    { l: 'L1', title: 'Manuelle Exporte', desc: 'Copy & Paste von Chat-Verlauf', active: false },
                    { l: 'L2', title: 'Strukturiertes JSON', desc: 'Auto-Prompting und MD/JSON Logs', active: false },
                    { l: 'L3', title: 'Datenbank', desc: 'Ordner-Parser & Aggregation', active: false },
                    { l: 'L4', title: 'Intelligence', desc: 'Vector Search über alle Chats', active: true },
                    { l: 'L5', title: 'Full AI System', desc: 'Autonomer Entwickler Assist', active: false },
                  ].map(lvl => (
                    <div key={lvl.l} className={`p-4 rounded-xl border flex flex-col ${lvl.active ? 'bg-indigo-500/20 border-indigo-500/50' : 'bg-slate-800/30 border-slate-700/50'}`}>
                      <span className={`text-xs font-bold mb-1 ${lvl.active ? 'text-indigo-400' : 'text-slate-500'}`}>{lvl.l}</span>
                      <strong className={`text-sm block mb-2 ${lvl.active ? 'text-white' : 'text-slate-300'}`}>{lvl.title}</strong>
                      <p className="text-xs text-slate-400 leading-relaxed">{lvl.desc}</p>
                    </div>
                  ))}
                </div>
              </div>

              <div>
                <h3 className="text-lg font-bold text-white mb-6 border-b border-slate-800 pb-2">Standard Integration Workflow</h3>
                <ol className="space-y-4">
                  <li className="flex items-center justify-between p-4 bg-slate-800/40 border border-slate-700 rounded-xl">
                    <div className="flex items-center gap-4">
                      <div className="w-8 h-8 rounded-full bg-slate-700 flex justify-center items-center font-bold text-sm">1</div>
                      <span className="font-medium text-slate-200">Chat & Brainstorming führen</span>
                    </div>
                  </li>
                  <li className="flex items-center justify-between p-4 bg-slate-800/40 border border-slate-700 rounded-xl">
                    <div className="flex items-center gap-4">
                      <div className="w-8 h-8 rounded-full bg-slate-700 flex justify-center items-center font-bold text-sm">2</div>
                      <span className="font-medium text-slate-200">Voll-Export Triggern (Auto-Prompting 100% Extraction)</span>
                    </div>
                  </li>
                  <li className="flex items-center justify-between p-4 bg-indigo-500/10 border border-indigo-500/30 rounded-xl">
                    <div className="flex items-center gap-4">
                      <div className="w-8 h-8 rounded-full bg-indigo-500/20 text-indigo-400 flex justify-center items-center font-bold text-sm">3</div>
                      <span className="font-medium text-indigo-200">Python Parser triggern (Aggregiert Module, extrahiert ToDos)</span>
                    </div>
                  </li>
                  <li className="flex items-center justify-between p-4 bg-emerald-500/10 border border-emerald-500/30 rounded-xl">
                    <div className="flex items-center gap-4">
                      <div className="w-8 h-8 rounded-full bg-emerald-500/20 text-emerald-400 flex justify-center items-center font-bold text-sm">4</div>
                      <span className="font-medium text-emerald-200">Datanbank & Vektor-Index aktualisieren</span>
                    </div>
                  </li>
                  <li className="flex items-center justify-between p-4 bg-slate-800/40 border border-slate-700 rounded-xl">
                    <div className="flex items-center gap-4">
                      <div className="w-8 h-8 rounded-full bg-slate-700 flex justify-center items-center font-bold text-sm">5</div>
                      <span className="font-medium text-slate-200">Im DKB Dashboard verknüpft / durchsuchbar nutzen</span>
                    </div>
                  </li>
                </ol>
              </div>
            </motion.div>
          )}

        </AnimatePresence>
      </div>

    </div>
  );
}
