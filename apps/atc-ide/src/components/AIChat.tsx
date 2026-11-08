import React, { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Send, Bot, User, Paperclip, X, Download, Loader2, Play } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { SnakeGame } from './SnakeGame';
import { CalculatorApp, TodoApp } from '../apps';

interface Message {
  id: string;
  role: 'user' | 'model';
  text: string;
  files?: Array<{ name: string; data: string; type: string }>;
}

export function AIChat({ onClose, onApplyCode, guidedMode }: { onClose: () => void, onApplyCode: (code: string) => void, guidedMode?: boolean }) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [previewMode, setPreviewMode] = useState<'chat' | 'gallery' | 'snake' | 'calc' | 'todo'>('chat');
  
  useEffect(() => {
    if (guidedMode && messages.length === 0) {
      setMessages([{
        id: 'init-guided',
        role: 'model',
        text: 'Ich habe die Grundstruktur (Ordner & Dateien) für das Projekt erfolgreich generiert!\n\nDu kannst sie im Explorer auf der linken Seite betrachten. Sag mir einfach, womit ich beginnen soll, oder nutze den "Templates & Apps"-Button oben, um weitere Funktionen zu integrieren und zu testen.'
      }]);
    }
  }, [guidedMode]);

  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [attachedFiles, setAttachedFiles] = useState<Array<{ name: string; data: string; type: string }>>([]);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const scrollToBottom = () => {
    if (previewMode === 'chat') {
      messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isLoading, previewMode]);

  const handleSendMessage = async () => {
    if (!input.trim() && attachedFiles.length === 0) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      text: input,
      files: attachedFiles.length > 0 ? [...attachedFiles] : undefined
    };

    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setAttachedFiles([]);
    setIsLoading(true);

    const modelMessageId = (Date.now() + 1).toString();
    setMessages(prev => [...prev, { id: modelMessageId, role: 'model', text: '' }]);

    try {
      const response = await fetch('/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          prompt: input,
          history: messages.map(m => ({ role: m.role, text: m.text })),
          files: userMessage.files
        })
      });

      if (!response.ok) throw new Error('API Error');

      const reader = response.body?.getReader();
      const decoder = new TextDecoder();

      if (reader) {
        let aiText = '';
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          
          const chunk = decoder.decode(value);
          const lines = chunk.split('\n');
          
          for (const line of lines) {
            if (line.startsWith('data: ')) {
              const data = line.slice(6);
              if (data === '[DONE]') continue;
              try {
                const parsed = JSON.parse(data);
                if (parsed.text) {
                  aiText += parsed.text;
                  setMessages(prev => prev.map(msg => 
                    msg.id === modelMessageId ? { ...msg, text: aiText } : msg
                  ));
                }
              } catch (e) {
                // Ignore parse errors for incomplete chunks
              }
            }
          }
        }
      }
    } catch (e) {
       console.error(e);
       setMessages(prev => prev.map(msg => 
          msg.id === modelMessageId ? { ...msg, text: '*Error: Failed to connect to AI.*' } : msg
        ));
    } finally {
      setIsLoading(false);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      handleFiles(Array.from(e.target.files));
    }
  };

  const handleFiles = (files: File[]) => {
    files.forEach(file => {
      const reader = new FileReader();
      reader.onload = (e) => {
        if (e.target?.result && typeof e.target.result === 'string') {
          setAttachedFiles(prev => [...prev, {
            name: file.name,
            type: file.type,
            data: e.target!.result as string
          }]);
        }
      };
      reader.readAsDataURL(file);
    });
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    if (e.dataTransfer.files) {
      handleFiles(Array.from(e.dataTransfer.files));
    }
  };

  const exportChat = () => {
    const text = messages.map(m => `[${m.role.toUpperCase()}]:\n${m.text}`).join('\n\n');
    const blob = new Blob([text], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'lumino-chat-export.txt';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="w-full bg-black/60 backdrop-blur-xl flex flex-col shrink-0 h-full border-l border-white/10">
      {/* Header */}
      <div className="h-14 flex items-center justify-between px-4 border-b border-white/10 shrink-0">
        <div className="flex items-center gap-2 text-slate-200 font-semibold text-sm">
          <Bot className="w-5 h-5 text-purple-400" />
          AI Assistant
        </div>
        <div className="flex items-center gap-1">
           <button onClick={() => setPreviewMode(previewMode !== 'chat' ? 'chat' : 'gallery')} title={previewMode !== 'chat' ? "Zurück zum Chat" : "Mögliche Funktionen & Vorlagen"} className={`p-1.5 rounded transition-colors flex items-center gap-1 text-xs font-bold ${previewMode !== 'chat' ? "bg-cyan-500/20 text-cyan-400" : "text-slate-400 hover:text-cyan-400 hover:bg-white/5"}`}>
              {previewMode !== 'chat' ? <Bot className="w-4 h-4" /> : <Play className="w-4 h-4" />}
              {previewMode !== 'chat' ? "Chat" : "Templates & Apps"}
           </button>
           <button onClick={exportChat} title="Export Chat" className="p-1.5 text-slate-400 hover:text-cyan-400 hover:bg-white/5 rounded transition-colors">
              <Download className="w-4 h-4" />
           </button>
           <button onClick={onClose} title="Close AI Panel" className="p-1.5 text-slate-400 hover:text-pink-400 hover:bg-white/5 rounded transition-colors">
              <X className="w-4 h-4" />
           </button>
        </div>
      </div>

      {previewMode === 'gallery' ? (
        <div className="flex-1 overflow-auto bg-[#0c0c0e] p-4 font-sans">
          <h2 className="text-lg font-bold text-slate-200 mb-2">Vorlagen & Fertige Module</h2>
          <p className="text-xs text-slate-500 mb-6">Wähle eine Funktion, um sie direkt im Chatfenster zu testen. Über den Code-Button kannst du sie in dein Projekt übernehmen.</p>
          
          <div className="space-y-4">
             <div onClick={() => setPreviewMode('snake')} className="bg-white/5 border border-white/10 p-4 rounded-xl hover:bg-white/10 hover:border-emerald-500/30 cursor-pointer transition-all">
                <div className="font-bold text-emerald-400 mb-1 flex items-center gap-2"><Play className="w-4 h-4" /> Snake Game (HTML5 Canvas)</div>
                <p className="text-xs text-slate-400 mb-2">Ein komplettes Snake-Spiel mit Score, Highscore und Game-Over Logik.</p>
             </div>
             
             <div onClick={() => setPreviewMode('calc')} className="bg-white/5 border border-white/10 p-4 rounded-xl hover:bg-white/10 hover:border-cyan-500/30 cursor-pointer transition-all">
                <div className="font-bold text-cyan-400 mb-1 flex items-center gap-2"><Play className="w-4 h-4" /> Taschenrechner (React State)</div>
                <p className="text-xs text-slate-400 mb-2">Taschenrechner mit Fehlerhandling, Clear-Funktion und einfachem Layout.</p>
             </div>

             <div onClick={() => setPreviewMode('todo')} className="bg-white/5 border border-white/10 p-4 rounded-xl hover:bg-white/10 hover:border-purple-500/30 cursor-pointer transition-all">
                <div className="font-bold text-purple-400 mb-1 flex items-center gap-2"><Play className="w-4 h-4" /> ToDo Liste (State & Styling)</div>
                <p className="text-xs text-slate-400 mb-2">Fertiger Code für eine Todo-Liste inklusive Löschen und Abhaken.</p>
             </div>
          </div>
        </div>
      ) : previewMode !== 'chat' ? (
        <div className="flex-1 overflow-auto bg-[#0c0c0e] flex items-center justify-center relative">
          <div className="absolute top-4 left-4 right-4 flex items-center justify-between pb-2 border-b border-white/10 z-10">
             <span className="text-xs text-slate-400 uppercase tracking-widest font-bold">Live Vorschau</span>
             <button onClick={() => setPreviewMode('gallery')} className="text-xs bg-white/5 hover:bg-white/10 text-slate-300 px-2 py-1 rounded">Zurück zur Übersicht</button>
          </div>
          <div className="w-full h-full flex pt-14 pb-4 px-4 overflow-y-auto">
            {previewMode === 'snake' && <SnakeGame />}
            {previewMode === 'calc' && <CalculatorApp />}
            {previewMode === 'todo' && <TodoApp />}
          </div>
        </div>
      ) : (
        <>
          {/* Messages */}
          <div className="flex-1 overflow-y-auto p-4 space-y-6">
            {messages.length === 0 && (
              <div className="text-center text-slate-500 text-sm mt-10">
                Ask me anything about Lumino code or ask me to write a frontend.
              </div>
            )}
            {messages.map(msg => (
              <div key={msg.id} className="flex flex-col gap-1">
                <div className="flex items-center gap-2 text-xs font-bold text-slate-500 uppercase tracking-wider">
                   {msg.role === 'user' ? (
                     <><User className="w-3 h-3 text-cyan-400" /> You</>
                   ) : (
                     <><Bot className="w-3 h-3 text-purple-400" /> Assistant</>
                   )}
                </div>
                
                {msg.files && msg.files.length > 0 && (
                   <div className="flex flex-wrap gap-2 mb-2">
                     {msg.files.map((file, i) => (
                        <div key={i} className="flex items-center gap-1.5 px-2 py-1 bg-white/5 rounded text-xs border border-white/10 text-slate-300">
                           <Paperclip className="w-3 h-3 text-cyan-500" />
                           <span className="truncate max-w-[120px]">{file.name}</span>
                        </div>
                     ))}
                   </div>
                )}

                <div className={`text-sm ${msg.role === 'user' ? 'text-slate-300' : 'text-slate-200'} prose prose-invert max-w-none prose-pre:bg-white/5 prose-pre:border prose-pre:border-white/10 prose-p:leading-relaxed prose-sm`}>
                  <ReactMarkdown 
                    remarkPlugins={[remarkGfm]}
                    components={{
                      code: ({ node, inline, className, children, ...props }: any) => {
                        const match = /language-(\w+)/.exec(className || '');
                        const isCodeBlock = !inline && match;
                        return isCodeBlock ? (
                          <div className="relative group">
                            <button 
                              onClick={() => onApplyCode(String(children).replace(/\n$/, ''))}
                              className="absolute right-2 top-2 bg-white/10 hover:bg-white/20 p-1 rounded text-xs text-slate-300 opacity-0 group-hover:opacity-100 transition-opacity"
                            >
                              Apply Code
                            </button>
                            <code className={className} {...props}>
                              {children}
                            </code>
                          </div>
                        ) : (
                          <code className={className} {...props}>
                            {children}
                          </code>
                        );
                      }
                    }}
                  >
                    {msg.text || (msg.role === 'model' && isLoading && msg.id === messages[messages.length-1].id ? '...' : '')}
                  </ReactMarkdown>
                </div>
              </div>
            ))}
            <div ref={messagesEndRef} />
          </div>

          {/* Input Area */}
          <div 
            className="p-3 border-t border-white/10 bg-black/40 relative"
            onDragOver={(e) => e.preventDefault()}
            onDrop={handleDrop}
          >
            {attachedFiles.length > 0 && (
               <div className="flex flex-wrap gap-2 mb-2 px-1">
                 {attachedFiles.map((file, i) => (
                    <div key={i} className="flex items-center gap-1 px-2 py-1 bg-white/10 rounded text-[10px] border border-white/10 text-cyan-300">
                       <span className="truncate max-w-[80px]">{file.name}</span>
                       <button onClick={() => setAttachedFiles(prev => prev.filter((_, idx) => idx !== i))} className="hover:text-pink-400">
                         <X className="w-3 h-3" />
                       </button>
                    </div>
                 ))}
               </div>
            )}
            <div className="flex items-end gap-2 bg-white/5 border border-white/10 rounded-lg p-1.5 focus-within:border-cyan-500/50 transition-colors">
              <button 
                onClick={() => fileInputRef.current?.click()}
                className="p-1.5 text-slate-400 hover:text-cyan-400 rounded-md transition-colors shrink-0"
                title="Upload File or Image"
              >
                <Paperclip className="w-4 h-4" />
              </button>
              <input
                type="file"
                multiple
                ref={fileInputRef}
                onChange={handleFileChange}
                className="hidden"
              />
              <textarea
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    handleSendMessage();
                  }
                }}
                placeholder="Frag die KI... (Dateien per Drag & Drop)"
                className="flex-1 bg-transparent border-none text-sm text-slate-200 resize-none outline-none max-h-32 min-h-[36px] py-2 px-1 placeholder-slate-600"
                rows={1}
              />
              <button 
                onClick={handleSendMessage}
                disabled={isLoading || (!input.trim() && attachedFiles.length === 0)}
                className="p-1.5 bg-cyan-500 text-white rounded-md hover:bg-cyan-400 transition-colors shrink-0 disabled:opacity-50 disabled:hover:bg-cyan-500"
              >
                {isLoading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Send className="w-4 h-4" />}
              </button>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
