import React, { useState } from 'react';
import { Download, Copy, CheckCircle, FileJson, Code, TerminalSquare, Database, Layers, GitMerge } from 'lucide-react';

export function SessionExportView() {
  const [copied, setCopied] = useState<string | null>(null);

  const handleCopy = (text: string, id: string) => {
    navigator.clipboard.writeText(text);
    setCopied(id);
    setTimeout(() => setCopied(null), 2000);
  };

  const autoPrompt = `Extrahiere 100% aller Inhalte aus diesem Chat auf Expertenniveau.

Strukturiere in:
1. Features
2. Module
3. Code (komplett, bereinigt)
4. Architektur
5. Abhängigkeiten
6. ToDos (offen / erledigt)
7. Optimierungen
8. Fehler & Risiken

Formate:
- Markdown (menschenlesbar)
- JSON (maschinenlesbar)

Identifiziere alle impliziten Annahmen, finde fehlende Komponenten und schlage skalierbare Verbesserungen vor. Keine Informationen auslassen.`;

  const jsonSchema = `{
  "project_name": "A-Town OS",
  "version": "1.0.0",
  "modules": [
    {
      "name": "ModuleName",
      "description": "Was tut es?",
      "status": "planned|in-progress|completed",
      "dependencies": ["OtherModule"],
      "files": ["src/bla.ts"]
    }
  ],
  "architecture": {
    "decisions": ["Why X over Y?"]
  },
  "todos": [
    { "task": "Implement X", "status": "open" }
  ],
  "risks": ["Risk description"]
}`;

  const masterMarkdown = `# Master System Architecture

## 1. Übersicht
Zusammenführung aller extrahierten Chat-Sessions.

## 2. Module
- **[Modul Name]**: (Status: in-progress)
  - Abhängigkeiten: ...
  - Dateien: \`src/bla.ts\`

## 3. Architektur-Entscheidungen
- Entscheidung A: Grund...

## 4. Master ToDo List
- [ ] Ticket 1
- [x] Ticket 2`;

  const pythonParser = `import json
import re
import glob

def merge_chat_exports(export_dir):
    all_modules = {}
    master_todos = []
    
    # Process all markdown exports
    for filepath in glob.glob(f"{export_dir}/*.md"):
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
            
        json_blocks = re.findall(r'\`\`\`json(.*?(?=\`\`\`))\`\`\`', content, re.DOTALL)
        
        for block in json_blocks:
            try:
                data = json.loads(block.strip())
                # Merge Modules (Upsert logic)
                for mod in data.get('modules', []):
                    all_modules[mod['name']] = mod
                # Merge ToDos
                master_todos.extend(data.get('todos', []))
            except json.JSONDecodeError:
                pass
                
    master_db = {
        "modules": list(all_modules.values()),
        "todos": master_todos
    }
    
    with open('MASTER_DB.json', 'w') as f:
        json.dump(master_db, f, indent=2)
        
    print(f"Merged {len(master_db['modules'])} modules.")

# Usage
# merge_chat_exports('./exports')`;

  const copyButton = (text: string, id: string) => (
    <button 
      onClick={() => handleCopy(text, id)}
      className="p-1.5 rounded bg-slate-800 border border-slate-700 text-slate-300 hover:text-white hover:bg-slate-700 transition"
      title="Copy to clipboard"
    >
      {copied === id ? <CheckCircle className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
    </button>
  );

  return (
    <div className="flex-1 overflow-auto bg-slate-900 font-sans text-slate-200 p-6 h-full">
      <div className="max-w-5xl mx-auto space-y-8 pb-12">
        <div className="border-b border-slate-700/50 pb-6 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2.5 bg-indigo-500/20 text-indigo-400 rounded-lg">
              <Database className="w-6 h-6" />
            </div>
            <div>
              <h1 className="text-2xl font-bold text-white tracking-tight">AI Knowledge Extraction</h1>
              <p className="text-slate-400">Skalierbarer Extraktions-Workflow: Aus Chat-Logs eine Entwickler-Datenbank bauen.</p>
            </div>
          </div>
          <div className="flex gap-2">
            <span className="px-3 py-1 bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded-full text-xs font-bold uppercase tracking-wider">
              Level 4: System
            </span>
          </div>
        </div>

        {/* Strategy Overview */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
          <div className="bg-slate-800/40 p-4 rounded-xl border border-slate-700/50">
            <Layers className="w-5 h-5 text-blue-400 mb-2" />
            <h3 className="font-bold text-slate-200 text-sm mb-1">1. Extraktion</h3>
            <p className="text-xs text-slate-400">Kein Informationsverlust. Alles aus dem Chat als Markdown & JSON ausleiten.</p>
          </div>
          <div className="bg-slate-800/40 p-4 rounded-xl border border-slate-700/50">
            <GitMerge className="w-5 h-5 text-purple-400 mb-2" />
            <h3 className="font-bold text-slate-200 text-sm mb-1">2. Aggregation</h3>
            <p className="text-xs text-slate-400">Multi-Chat Meta-Zusammenführung. Duplikate entfernen, Architektur verknüpfen.</p>
          </div>
          <div className="bg-slate-800/40 p-4 rounded-xl border border-slate-700/50">
            <Database className="w-5 h-5 text-emerald-400 mb-2" />
            <h3 className="font-bold text-slate-200 text-sm mb-1">3. Master System</h3>
            <p className="text-xs text-slate-400">Deine private Entwickler-Wissensdatenbank. "Frag deine eigenen Chats".</p>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          
          {/* Prompt */}
          <div className="bg-slate-800/60 rounded-xl p-5 border border-slate-700/50 shadow-sm flex flex-col">
            <div className="flex items-center gap-2 text-emerald-400 font-semibold border-b border-slate-700 pb-3 mb-4">
              <TerminalSquare className="w-5 h-5" />
              <span>High-Performance Prompt</span>
            </div>
            <p className="text-sm text-slate-400 mb-4">Standard-Prompt für das Ende JEDER Chat-Sitzung. Holt 100% raus.</p>
            <div className="relative flex-1">
              <pre className="text-xs font-mono text-slate-300 p-4 bg-slate-900 rounded-lg whitespace-pre-wrap leading-relaxed border border-slate-800 h-full">{autoPrompt}</pre>
              <div className="absolute top-2 right-2 flex gap-2">
                {copyButton(autoPrompt, 'prompt')}
              </div>
            </div>
          </div>

          {/* Master DB Format */}
          <div className="flex flex-col gap-6">
            <div className="bg-slate-800/60 rounded-xl p-5 border border-slate-700/50 shadow-sm">
              <div className="flex items-center gap-2 text-cyan-400 font-semibold border-b border-slate-700 pb-3 mb-4">
                <FileJson className="w-5 h-5" />
                <span>JSON Master Schema</span>
              </div>
              <div className="relative">
                <pre className="text-xs font-mono text-cyan-300 p-4 bg-slate-900 rounded-lg whitespace-pre-wrap leading-relaxed border border-slate-800 max-h-[200px] overflow-auto">{jsonSchema}</pre>
                <div className="absolute top-2 right-2">
                  {copyButton(jsonSchema, 'json')}
                </div>
              </div>
            </div>

            <div className="bg-slate-800/60 rounded-xl p-5 border border-slate-700/50 shadow-sm">
              <div className="flex items-center gap-2 text-blue-400 font-semibold border-b border-slate-700 pb-3 mb-4">
                <Layers className="w-5 h-5" />
                <span>Master Markdown Struktur</span>
              </div>
              <div className="relative">
                <pre className="text-xs font-mono text-blue-300 p-4 bg-slate-900 rounded-lg whitespace-pre-wrap leading-relaxed border border-slate-800 max-h-[200px] overflow-auto">{masterMarkdown}</pre>
                <div className="absolute top-2 right-2">
                  {copyButton(masterMarkdown, 'markdown')}
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Python Parser */}
        <div className="bg-slate-800/60 rounded-xl p-5 border border-slate-700/50 shadow-sm space-y-4">
          <div className="flex items-center gap-2 text-orange-400 font-semibold border-b border-slate-700 pb-3">
            <Code className="w-5 h-5" />
            <span>Python Auto-Parser & Meta-Aggregation</span>
          </div>
          <p className="text-sm text-slate-400">Skript zur Verarbeitung und Reduzierung mehrerer Chat-Exports (<code className="text-xs bg-slate-900 px-1 py-0.5 rounded">.md</code> files) in ein zentrales Master-System.</p>
          <div className="relative">
            <pre className="text-xs font-mono text-orange-300 p-4 bg-slate-900 rounded-lg whitespace-pre-wrap leading-relaxed overflow-x-auto border border-slate-800">{pythonParser}</pre>
            <div className="absolute top-2 right-2">
              {copyButton(pythonParser, 'python')}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
