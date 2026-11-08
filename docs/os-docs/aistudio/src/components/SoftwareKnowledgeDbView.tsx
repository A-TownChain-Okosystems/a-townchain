import React, { useState, useEffect } from 'react';
import { Book, FileText, Bookmark, Target, PenTool, Database, Code, CheckSquare, RefreshCw, Layers, History } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import Markdown from 'react-markdown';
import { WIKI_CONTENT } from '../wikiData';
import { ROADMAP_DATA } from '../roadmapData';
import { syncService, SyncStatus } from '../services/SyncService';
import { SyncHistoryModal } from './SyncHistoryModal';

interface KnowledgeItem {
  id: string;
  type: 'document' | 'code' | 'wiki' | 'roadmap';
  title: string;
  content: string;
  category?: string;
  subcategory?: string;
  tags?: string[];
  version?: string;
}


export function SoftwareKnowledgeDbView() {
  const [activeTab, setActiveTab] = useState('documents');
  const [knowledgeData, setKnowledgeData] = useState<{documents: KnowledgeItem[], code: KnowledgeItem[]}>({ documents: [], code: [] });
  const [isLoading, setIsLoading] = useState(true);
  const [selectedFile, setSelectedFile] = useState<KnowledgeItem | null>(null);
  const [syncStatus, setSyncStatus] = useState<SyncStatus | null>(null);
  const [isHistoryOpen, setIsHistoryOpen] = useState(false);

  useEffect(() => {
    const unsubscribe = syncService.subscribe((status) => {
      setSyncStatus(status);
    });

    const handleSynced = () => fetchKnowledge(false);
    window.addEventListener('atc-knowledge-synced', handleSynced);

    return () => {
      unsubscribe();
      window.removeEventListener('atc-knowledge-synced', handleSynced);
    };
  }, []);

  // Sync selectedFile content automatically when knowledgeData updates
  useEffect(() => {
    if (selectedFile) {
      if (selectedFile.type === 'document') {
        const updated = knowledgeData.documents.find(d => d.id === selectedFile.id);
        if (updated && updated.content !== selectedFile.content) setSelectedFile(updated);
      } else {
        const updated = knowledgeData.code.find(c => c.id === selectedFile.id);
        if (updated && updated.content !== selectedFile.content) setSelectedFile(updated);
      }
    }
  }, [knowledgeData, selectedFile]);

  const fetchKnowledge = async (showLoading = true) => {
    if (showLoading) setIsLoading(true);
    try {
      const res = await fetch('/api/knowledge');
      const data = await res.json();
      
      // Synchronize using new Sync Service
      let combinedDocs: KnowledgeItem[] = [];
      try {
        const savedSync = localStorage.getItem('atc_unified_knowledge_sync');
        if (savedSync) {
          combinedDocs = JSON.parse(savedSync);
        }
      } catch (e) {}

      // Combine with extracted md documents
      setKnowledgeData({
        documents: [...data.documents, ...combinedDocs],
        code: data.code
      });
    } catch (e) {
      console.error("Failed to fetch knowledge base", e);
    } finally {
      if (showLoading) setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchKnowledge(true);
    
    if (!localStorage.getItem('atc_unified_knowledge_sync')) {
      syncService.synchronize();
    }

    const interval = setInterval(() => {
      fetchKnowledge(false);
    }, 5000); // 5 seconds
    
    return () => clearInterval(interval);
  }, []);

  // Compute Todos from code files and local storage
  const todos = React.useMemo(() => {
    const extractedTodos: {file: string, line: string, text: string, type: 'code' | 'hub'}[] = [];
    knowledgeData.code.forEach(file => {
      const lines = file.content.split('\n');
      lines.forEach((line, index) => {
        if (line.includes('TODO:') || line.includes('FIXME:')) {
          extractedTodos.push({ file: file.title, line: (index + 1).toString(), text: line.trim(), type: 'code' });
        }
      });
    });
    
    try {
      const saved = localStorage.getItem('app_todos_v8_enterprise');
      if (saved) {
        const parsed = JSON.parse(saved);
        parsed.filter((t: any) => !t.completed).forEach((t: any) => {
          extractedTodos.push({ file: 'Workspace Tasks (Sync)', line: '-', text: t.text, type: 'hub' });
        });
      }
    } catch (e) {}

    return extractedTodos;
  }, [knowledgeData.code]);

  return (
    <div className="flex h-full bg-[#050811] text-slate-200">
      {/* Sidebar */}
      <div className="w-64 border-r border-white/10 flex flex-col bg-black/20">
        <div className="p-4 border-b border-white/10">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="p-2 rounded bg-indigo-500/10 border border-indigo-500/20">
                <Database className="w-5 h-5 text-indigo-400" />
              </div>
              <h2 className="font-bold text-white tracking-tight">Software DB</h2>
            </div>
            <button onClick={() => fetchKnowledge(true)} disabled={isLoading} className="text-slate-500 hover:text-white transition-colors">
              <RefreshCw className={`w-4 h-4 ${isLoading ? 'animate-spin' : ''}`} />
            </button>
          </div>
        </div>
        
        <div className="flex-1 p-2 flex flex-col gap-1 overflow-y-auto custom-scrollbar">
          <button 
            onClick={() => { setActiveTab('documents'); setSelectedFile(null); }}
            className={`flex items-center justify-between w-full p-3 rounded-lg transition-colors text-left font-medium text-sm ${activeTab === 'documents' ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30' : 'text-slate-400 hover:bg-white/5 border border-transparent hover:text-slate-200'}`}
          >
            <div className="flex items-center gap-3">
              <Book className="w-4 h-4" />
              Documents Base
            </div>
            <span className="text-xs bg-black/30 px-2 py-0.5 rounded-full">{knowledgeData.documents.length}</span>
          </button>
          
          <button 
            onClick={() => { setActiveTab('code'); setSelectedFile(null); }}
            className={`flex items-center justify-between w-full p-3 rounded-lg transition-colors text-left font-medium text-sm ${activeTab === 'code' ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30' : 'text-slate-400 hover:bg-white/5 border border-transparent hover:text-slate-200'}`}
          >
            <div className="flex items-center gap-3">
              <Code className="w-4 h-4" />
              Code Archive
            </div>
            <span className="text-xs bg-black/30 px-2 py-0.5 rounded-full">{knowledgeData.code.length}</span>
          </button>
          
          <button 
            onClick={() => { setActiveTab('todos'); setSelectedFile(null); }}
            className={`flex items-center justify-between w-full p-3 rounded-lg transition-colors text-left font-medium text-sm ${activeTab === 'todos' ? 'bg-amber-500/20 text-amber-300 border border-amber-500/30' : 'text-slate-400 hover:bg-white/5 border border-transparent hover:text-slate-200'}`}
          >
            <div className="flex items-center gap-3">
              <CheckSquare className="w-4 h-4" />
              Extracted Todos
            </div>
            <span className="text-xs bg-black/30 px-2 py-0.5 rounded-full">{todos.length}</span>
          </button>
        </div>

        {/* Sync Service Control */}
        <div className="p-4 border-t border-white/10 bg-black/40">
           <div className="flex gap-2 mb-3">
             <button
               onClick={() => syncService.synchronize()}
               disabled={syncStatus?.status === 'syncing'}
               className="flex-1 py-2 px-3 flex items-center justify-center gap-2 rounded-lg text-sm font-medium transition-colors bg-indigo-500/10 text-indigo-400 border border-indigo-500/30 hover:bg-indigo-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
             >
               <RefreshCw className={`w-4 h-4 ${syncStatus?.status === 'syncing' ? 'animate-spin' : ''}`} />
               Unified Sync
             </button>
             <button
               onClick={() => setIsHistoryOpen(true)}
               className="px-3 py-2 flex items-center justify-center rounded-lg text-sm font-medium transition-colors bg-white/5 text-slate-300 border border-white/10 hover:bg-white/10"
               title="View Sync History"
             >
               <History className="w-4 h-4" />
             </button>
           </div>
           
           {syncStatus && syncStatus.status !== 'idle' && (
             <div className="text-xs text-center font-mono">
                <span className={
                  syncStatus.status === 'success' ? 'text-emerald-400' :
                  syncStatus.status === 'error' ? 'text-red-400' :
                  'text-amber-400'
                }>{syncStatus.message}</span>
             </div>
           )}
           {syncStatus?.lastSync && syncStatus.status === 'idle' && (
             <div className="text-xs text-center text-slate-500 font-mono">
                Last sync: {new Date(syncStatus.lastSync).toLocaleTimeString()}
             </div>
           )}
        </div>
      </div>

      {/* Content Area */}
      <div className="flex-1 overflow-auto bg-[#0a0d18] relative flex flex-col">
        {isLoading && !knowledgeData.documents.length ? (
          <div className="absolute inset-0 flex items-center justify-center bg-[#0a0d18]/50 z-10 backdrop-blur-sm">
            <div className="flex flex-col items-center gap-4">
              <RefreshCw className="w-8 h-8 text-indigo-500 animate-spin" />
              <p className="text-slate-400 font-mono text-sm tracking-widest uppercase">Indexing Knowledge Base...</p>
            </div>
          </div>
        ) : selectedFile ? (
           <div className="flex flex-col h-full">
              <div className="p-4 border-b border-white/10 bg-black/20 flex items-center justify-between sticky top-0 z-10 backdrop-blur-md">
                 <div>
                    <h2 className="text-lg font-bold text-white flex items-center gap-2">
                        {selectedFile.type === 'document' && <Book className="w-5 h-5 text-indigo-400" />}
                        {selectedFile.type === 'wiki' && <Book className="w-5 h-5 text-purple-400" />}
                        {selectedFile.type === 'roadmap' && <Bookmark className="w-5 h-5 text-amber-400" />}
                        {selectedFile.type === 'code' && <Code className="w-5 h-5 text-emerald-400" />}
                        {selectedFile.title}
                    </h2>
                    <p className="text-xs text-slate-500 font-mono mt-1">{selectedFile.content.length.toLocaleString()} bytes</p>
                 </div>
                 <button 
                   onClick={() => setSelectedFile(null)}
                   className="px-3 py-1.5 bg-white/5 hover:bg-white/10 text-white text-sm rounded transition-colors border border-white/10"
                 >
                   Back to Overview
                 </button>
              </div>
              <div className="p-6 overflow-auto flex-1">
                 {selectedFile.type !== 'code' ? (
                   <div className="prose prose-invert prose-indigo max-w-4xl mx-auto markdown-body bg-[#11131f] p-8 rounded-xl border border-white/5">
                     <Markdown>{selectedFile.content}</Markdown>
                   </div>
                 ) : (
                   <pre className="text-xs font-mono text-slate-300 bg-[#11131f] p-6 rounded-xl overflow-x-auto border border-white/5">
                     <code>{selectedFile.content}</code>
                   </pre>
                 )}
              </div>
           </div>
        ) : (
          <motion.div
            key={activeTab}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.2 }}
            className="p-8 max-w-5xl mx-auto w-full"
          >
            {activeTab === 'documents' && (
              <div className="space-y-6">
                <div className="border-b border-white/10 pb-4">
                  <h1 className="text-3xl font-bold text-white mb-2">Documentation & Wiki</h1>
                  <p className="text-slate-400">Automatisch aus dem Projektverzeichnis extrahierte Markdowns und Roadmaps.</p>
                </div>
                
                <div className="grid gap-3">
                  {knowledgeData.documents.map(doc => (
                    <div 
                      key={doc.id} 
                      onClick={() => setSelectedFile(doc)}
                      className="p-4 rounded-xl border border-white/5 bg-[#11131f] hover:bg-white/5 hover:border-indigo-500/30 cursor-pointer transition-all group flex items-start gap-4"
                    >
                      <div className={`p-3 rounded-lg shrink-0 ${doc.type === 'wiki' ? 'bg-purple-500/10' : doc.type === 'roadmap' ? 'bg-amber-500/10' : 'bg-indigo-500/10'}`}>
                        {doc.type === 'wiki' ? <Book className="w-6 h-6 text-purple-400" /> : doc.type === 'roadmap' ? <Bookmark className="w-6 h-6 text-amber-400" /> : <FileText className="w-6 h-6 text-indigo-400" />}
                      </div>
                      <div>
                        <h3 className="font-bold text-slate-200 group-hover:text-indigo-400 mb-1">
                           {doc.title}
                           {doc.version && <span className="ml-2 text-xs font-mono text-slate-500">({doc.version})</span>}
                        </h3>
                        {doc.category && (
                           <div className="text-[10px] text-slate-500 uppercase tracking-widest mb-1.5 flex items-center gap-1.5">
                              {doc.category} <span className="text-slate-600">/</span> {doc.subcategory}
                              {doc.tags && doc.tags.length > 0 && (
                                <div className="flex gap-1 ml-2">
                                  {doc.tags.map(t => <span key={t} className="px-1 py-0.5 bg-indigo-500/10 text-indigo-400 rounded-sm">#{t}</span>)}
                                </div>
                              )}
                           </div>
                        )}
                        <p className="text-sm text-slate-500 font-mono">{doc.content.substring(0, 120)}...</p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {activeTab === 'code' && (
              <div className="space-y-6">
                <div className="border-b border-white/10 pb-4 flex justify-between items-end">
                  <div>
                    <h1 className="text-3xl font-bold text-white mb-2">Code Archive</h1>
                    <p className="text-slate-400">Sämtliche Funktionen, Logik und Scripts aus dem src/ Verzeichnis.</p>
                  </div>
                  <div className="text-sm font-mono text-emerald-400 px-3 py-1 bg-emerald-500/10 rounded border border-emerald-500/20">
                    {knowledgeData.code.length} Files Indexed
                  </div>
                </div>
                
                <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
                  {knowledgeData.code.map(file => (
                    <div 
                      key={file.id} 
                      onClick={() => setSelectedFile(file)}
                      className="p-3 rounded-lg border border-white/5 bg-[#11131f] hover:bg-white/5 hover:border-emerald-500/30 cursor-pointer transition-all flex items-center gap-3 truncate"
                    >
                      <div className="flex flex-col truncate w-full gap-1">
                        <div className="flex items-center gap-2">
                           <Code className="w-4 h-4 text-emerald-500 shrink-0" />
                           <span className="text-sm font-mono text-slate-300 truncate">{file.title.replace('src/', '')}</span>
                        </div>
                        {file.category && (
                           <div className="flex flex-col gap-0.5">
                             <div className="text-[9px] text-slate-500 uppercase tracking-widest pl-6">
                               {file.category} / {file.subcategory}
                             </div>
                             <div className="flex gap-1 pl-6">
                               {file.tags?.slice(0, 3).map(t => <span key={t} className="px-1 py-[1px] bg-emerald-500/10 text-emerald-400 rounded-sm text-[8px] uppercase tracking-wider">#{t}</span>)}
                             </div>
                           </div>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {activeTab === 'todos' && (
              <div className="space-y-6">
                <div className="border-b border-white/10 pb-4">
                  <h1 className="text-3xl font-bold text-white mb-2">Global System Tasks</h1>
                  <p className="text-slate-400">Automatisch gefundene TODOs und FIXMEs aus dem gesamten Codebase.</p>
                </div>
                
                <div className="space-y-3">
                  {todos.length === 0 ? (
                    <div className="p-8 text-center border border-white/5 bg-black/20 rounded-xl">
                       <CheckSquare className="w-8 h-8 text-slate-600 mx-auto mb-3" />
                       <p className="text-slate-400">Keine offenen TODOs im Code gefunden. Alles perfekt!</p>
                    </div>
                  ) : (
                    todos.map((todo, idx) => (
                      <div key={idx} className={`p-4 rounded-lg border flex items-start gap-3 ${todo.type === 'hub' ? 'border-purple-500/20 bg-purple-500/5' : 'border-amber-500/20 bg-amber-500/5'}`}>
                        <Target className={`w-5 h-5 shrink-0 mt-0.5 ${todo.type === 'hub' ? 'text-purple-500' : 'text-amber-500'}`} />
                        <div>
                          <div className={`text-xs font-mono mb-1 ${todo.type === 'hub' ? 'text-purple-500/70' : 'text-amber-500/70'}`}>{todo.file} {todo.type === 'code' && `: Zeile ${todo.line}`}</div>
                          <p className="text-sm text-slate-200">{todo.text}</p>
                        </div>
                      </div>
                    ))
                  )}
                </div>
              </div>
            )}
          </motion.div>
        )}
      </div>

      <SyncHistoryModal 
        isOpen={isHistoryOpen} 
        onClose={() => setIsHistoryOpen(false)} 
      />
    </div>
  );
}
