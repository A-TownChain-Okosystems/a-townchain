import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { CheckCircle, Plus, Trash2, Calendar, LayoutList, GripVertical, Bell, Repeat, Clock } from 'lucide-react';

interface Todo {
  id: string;
  text: string;
  completed: boolean;
  createdAt: number;
  recurring?: 'none' | 'daily' | 'weekly' | 'monthly';
  reminder?: number;
}

export function TodoView({
  storageKey = 'app_todos_v8_enterprise',
  initialConfig
}: {
  storageKey?: string;
  initialConfig?: Todo[];
} = {}) {
  const [todos, setTodos] = useState<Todo[]>(() => {
    try {
      const saved = localStorage.getItem(storageKey);
      if (saved) return JSON.parse(saved);
      
      if (initialConfig) return initialConfig;

      const now = Date.now();
      const H = 1000 * 60 * 60;
      return [
        { id: '1', text: 'Core/Crypto: Post-Quantum Kyber-768 Schlüsselaustausch formal beweisen', completed: true, createdAt: now - H * 72 },
        { id: '2', text: 'Core/Consensus: BFT Modell (N >= 3f + 1) in TLA+ verifizieren', completed: true, createdAt: now - H * 70 },
        { id: '3', text: 'Core/Network: Kademlia O(log N) Routing Metriken auswerten', completed: true, createdAt: now - H * 68 },
        { id: '4', text: 'VM/Compiler: WASM Determinismus für Floating-Point Ops absichern', completed: true, createdAt: now - H * 66 },
        { id: '5', text: 'Storage/CRDT: Vector Clocks V(a) < V(b) Resolution Logs schreiben', completed: true, createdAt: now - H * 64 },
        { id: '6', text: 'AI/Orchestrator: ONNX Quantization auf INT4 für Layer 2 Nodes testen', completed: true, createdAt: now - H * 62 },
        { id: '7', text: 'ZKP/Snarks: PLONK Circuit Parametereinstellung optimieren (O(1) verify)', completed: true, createdAt: now - H * 60 },
        { id: '8', text: 'Repositories: A-TownChain/atc-lang Repository auf GitHub erstellen und Struktur pushen', completed: true, createdAt: now - H * 58 },
        { id: '9', text: 'Repositories: A-TownChain/atc-lang-wiki Repository für ATC-Lang Dokumentation initialisieren', completed: true, createdAt: now - H * 56 },
        { id: '9b', text: 'Repositories: A-TownChain/atc-pack Repository für Package Manager aufsetzen', completed: true, createdAt: now - H * 55 },
        { id: '9c', text: 'Repositories: A-TownChain/atc-trace Repository für Time-Travel Debugger aufsetzen', completed: true, createdAt: now - H * 55 },
        { id: '10', text: 'CI/CD: GitHub Actions Workflow für ATC-Lang Compiler (atc-lang) automatisieren', completed: true, createdAt: now - H * 54 },
        { id: '11', text: 'CI/CD: Dependabot für ATC-Lang Dependencies und Compiler-Crates einrichten', completed: true, createdAt: now - H * 52 },
        { id: '12', text: 'Wiki/Doc: 10 weitere formale Spezifikationen in die Enterprise Wiki einarbeiten', completed: true, createdAt: now - H * 50 },
        { id: '13', text: 'Roadmap/Plan: Phase 9-18 in der D3 Roadmap mit Gantt-Timeline konfigurieren', completed: true, createdAt: now - H * 48 },
        { id: '14', text: 'Smart Contracts: ATC-Lang Hoare-Logik Pre/Post-Conditions in VM integrieren', completed: true, createdAt: now - H * 46 },
        { id: '15', text: 'Netzwerk: Warp Sync / State Snapshot Server Backend aufsetzen', completed: true, createdAt: now - H * 44 },
        { id: '16', text: 'Sicherheit: Threshold-Encryption für Mempool (MEV Schutz) implementieren', completed: true, createdAt: now - H * 42 },
        { id: '17', text: 'UX/Settings: Design-Vorgaben für Ocean-Deep, Sunset-Red, Cyber-Cyan abspeichern', completed: true, createdAt: now - H * 40 }
      ];
    } catch {
      return [];
    }
  });
  const [inputValue, setInputValue] = useState('');
  const [filter, setFilter] = useState<'all' | 'active' | 'completed'>('all');
  const [draggedId, setDraggedId] = useState<string | null>(null);
  const [newRecurring, setNewRecurring] = useState<'none' | 'daily' | 'weekly' | 'monthly'>('none');
  const [newReminder, setNewReminder] = useState<string>('');
  const [showOptions, setShowOptions] = useState(false);
  const [showSaveToast, setShowSaveToast] = useState(false);
  const [lastSaveTime, setLastSaveTime] = useState<string>('');

  useEffect(() => {
    localStorage.setItem(storageKey, JSON.stringify(todos));
    setLastSaveTime(new Date().toLocaleTimeString());
    setShowSaveToast(true);
    const t = setTimeout(() => setShowSaveToast(false), 2000);
    return () => clearTimeout(t);
  }, [todos, storageKey]);

  useEffect(() => {
    // Check reminders occasionally
    const interval = setInterval(() => {
      const now = Date.now();
      todos.forEach(todo => {
        if (!todo.completed && todo.reminder && todo.reminder <= now) {
          // Play a sound or show notification. Alert isn't great.
          // Let's just visually handle it or leave a console log for simplicity.
          console.log(`Reminder: ${todo.text}`);
        }
      });
    }, 60000);
    return () => clearInterval(interval);
  }, [todos]);

  const handleAddTodo = (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputValue.trim()) return;

    const newTodo: Todo = {
      id: Date.now().toString(),
      text: inputValue.trim(),
      completed: true,
      createdAt: Date.now(),
      recurring: newRecurring,
      reminder: newReminder ? new Date(newReminder).getTime() : undefined
    };

    setTodos(prev => [newTodo, ...prev]);
    setInputValue('');
    setNewRecurring('none');
    setNewReminder('');
    setShowOptions(false);
  };

  const handleDragStart = (e: React.DragEvent, id: string) => {
    setDraggedId(id);
    e.dataTransfer.effectAllowed = 'move';
    // Small delay to allow the drag image to be generated before we potentially hide it or add styles
    setTimeout(() => {
       // optional: add a dragging class
    }, 0);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
  };

  const handleDrop = (e: React.DragEvent, dropId: string) => {
    e.preventDefault();
    if (!draggedId || draggedId === dropId) return;

    setTodos(prev => {
      const list = [...prev];
      const dragIdx = list.findIndex(t => t.id === draggedId);
      const dropIdx = list.findIndex(t => t.id === dropId);
      if (dragIdx === -1 || dropIdx === -1) return prev;
      
      const [draggedItem] = list.splice(dragIdx, 1);
      list.splice(dropIdx, 0, draggedItem);
      return list;
    });
    setDraggedId(null);
  };

  const toggleTodo = (id: string) => {
    setTodos(prev => prev.map(todo => 
      todo.id === id ? { ...todo, completed: !todo.completed } : todo
    ));
  };

  const deleteTodo = (id: string) => {
    setTodos(prev => prev.filter(todo => todo.id !== id));
  };

  const filteredTodos = todos.filter(todo => {
    if (filter === 'active') return !todo.completed;
    if (filter === 'completed') return todo.completed;
    return true;
  });

  const stats = {
    total: todos.length,
    completed: todos.filter(t => t.completed).length,
    active: todos.filter(t => !t.completed).length
  };

  const progress = stats.total > 0 ? Math.round((stats.completed / stats.total) * 100) : 0;

  return (
    <div className="flex flex-col gap-6 mt-8 pb-12 w-full max-w-5xl mx-auto">
      <div className="flex items-center gap-4 mb-2">
        <div className="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-500/30 flex items-center justify-center text-indigo-400 shadow-[0_0_15px_rgba(99,102,241,0.2)]">
          <LayoutList className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-3xl font-semibold tracking-tight text-white mb-2">Personal To-Dos</h2>
          <p className="text-sm font-light text-slate-500">Track and manage your tasks and objectives</p>
        </div>
      </div>

      <div className="bg-[#090b14]/80 backdrop-blur-md border border-atc-border/80 rounded-2xl p-6 shadow-2xl relative overflow-hidden">
        <div className="absolute top-0 left-0 w-64 h-64 bg-atc-purple/5 rounded-full blur-[80px] pointer-events-none" />
        <form onSubmit={handleAddTodo} className="flex flex-col gap-3 mb-6 relative z-10">
          <div className="flex gap-3">
            <input
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              placeholder="What needs to be done?"
              className="flex-1 px-4 py-3 bg-[#090b14]/60 backdrop-blur-md border border-atc-border/80 rounded-xl focus:outline-none focus:ring-2 focus:ring-atc-cyan/20 focus:border-atc-cyan/50 transition-all text-slate-300 placeholder:text-slate-500"
            />
            <button
              type="button"
              onClick={() => setShowOptions(!showOptions)}
              className={`px-3 py-3 border rounded-xl transition-colors flex items-center justify-center ${showOptions ? 'bg-atc-purple/20 border-atc-purple text-atc-purple' : 'bg-[#090b14]/60 border-atc-border/80 text-slate-400 hover:text-slate-200 hover:border-slate-500'}`}
              title="Task Options"
            >
              <Calendar className="w-5 h-5" />
            </button>
            <button
              type="submit"
              disabled={!inputValue.trim()}
              className="px-6 py-3 bg-atc-purple hover:bg-atc-purple/80 disabled:bg-atc-border disabled:text-slate-500 text-white font-medium rounded-xl transition-all flex items-center gap-2 shadow-[0_0_15px_rgba(162,89,255,0.4)] disabled:shadow-none"
            >
              <Plus className="w-5 h-5" />
              <span className="hidden sm:inline">Add Task</span>
            </button>
          </div>
          
          <AnimatePresence>
            {showOptions && (
              <motion.div 
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                exit={{ opacity: 0, height: 0 }}
                className="flex flex-col sm:flex-row gap-4 bg-black/40 p-4 rounded-xl border border-white/5 overflow-hidden"
              >
                <div className="flex-1">
                  <label className="text-xs font-mono text-slate-400 uppercase tracking-wider mb-2 block flex items-center gap-2">
                    <Repeat className="w-3.5 h-3.5" /> Recurring
                  </label>
                  <select 
                    value={newRecurring}
                    onChange={(e) => setNewRecurring(e.target.value as any)}
                    className="w-full px-3 py-2 bg-[#090b14]/60 border border-atc-border/80 rounded-lg text-sm text-slate-300 focus:outline-none focus:border-atc-purple/50 appearance-none cursor-pointer"
                  >
                    <option value="none">Does not repeat</option>
                    <option value="daily">Daily</option>
                    <option value="weekly">Weekly</option>
                    <option value="monthly">Monthly</option>
                  </select>
                </div>
                <div className="flex-1">
                  <label className="text-xs font-mono text-slate-400 uppercase tracking-wider mb-2 block flex items-center gap-2">
                    <Bell className="w-3.5 h-3.5" /> Reminder Alert
                  </label>
                  <input 
                    type="datetime-local" 
                    value={newReminder}
                    onChange={(e) => setNewReminder(e.target.value)}
                    className="w-full px-3 py-2 bg-[#090b14]/60 border border-atc-border/80 rounded-lg text-sm text-slate-300 focus:outline-none focus:border-atc-purple/50 [color-scheme:dark]"
                  />
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </form>

        {stats.total > 0 && (
          <div className="mb-6 pb-6 border-b border-atc-border/50 relative z-10">
            <div className="flex justify-between items-center mb-2">
              <span className="text-[10px] font-mono text-slate-500 font-bold uppercase tracking-widest">Progress</span>
              <span className="text-[10px] font-mono text-atc-purple font-semibold">{progress}% ({stats.completed}/{stats.total})</span>
            </div>
            <div className="w-full h-1.5 bg-atc-bg/80 rounded-full overflow-hidden">
              <div 
                className={`h-full rounded-full transition-all duration-700 ${progress === 100 ? 'bg-emerald-400' : 'bg-atc-purple'}`}
                style={{ width: `${progress}%` }}
              />
            </div>
          </div>
        )}

        <div className="flex items-center gap-2 mb-4 relative z-10">
          {(['all', 'active', 'completed'] as const).map((f) => (
            <button
              key={f}
              onClick={() => setFilter(f)}
              className={`px-4 py-1.5 text-xs font-mono uppercase tracking-wider rounded-lg transition-colors ${
                filter === f 
                  ? 'bg-atc-purple/10 text-atc-purple font-bold border border-atc-purple/30' 
                  : 'text-slate-500 hover:bg-[#090b14] hover:text-slate-300 border border-transparent'
              }`}
            >
              {f}
            </button>
          ))}
        </div>

        <div className="flex flex-col gap-2 min-h-[300px] relative z-10">
          <AnimatePresence mode="popLayout">
            {filteredTodos.length === 0 ? (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="flex flex-col items-center justify-center py-12 text-slate-500"
              >
                <div className="w-16 h-16 rounded-full bg-[#090b14]/50 border border-atc-border/50 flex items-center justify-center mb-4 text-slate-400">
                  <LayoutList className="w-8 h-8" />
                </div>
                <p className="text-sm font-medium text-slate-400">No tasks found</p>
                <p className="text-xs font-light mt-1">
                  {filter === 'all' 
                    ? 'Start by adding a new task above' 
                    : `You don't have any ${filter} tasks`}
                </p>
              </motion.div>
            ) : (
              filteredTodos.map(todo => (
                <motion.div
                  key={todo.id}
                  layout
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  draggable={filter === 'all'}
                  onDragStart={((e: React.DragEvent) => handleDragStart(e, todo.id)) as any}
                  onDragOver={handleDragOver as any}
                  onDrop={((e: React.DragEvent) => handleDrop(e, todo.id)) as any}
                  onDragEnd={() => setDraggedId(null)}
                  className={`group flex items-center justify-between p-4 rounded-xl border transition-all ${
                    todo.completed 
                      ? 'bg-[#090b14]/40 border-atc-border/30 opacity-70' 
                      : 'bg-[#090b14]/80 border-atc-border/80 hover:border-atc-purple/30 shadow-[0_4px_20px_rgba(0,0,0,0.5)] hover:shadow-xl'
                  } ${draggedId === todo.id ? 'opacity-30' : ''}`}
                >
                  <div className="flex items-center gap-4 flex-1 overflow-hidden">
                    {filter === 'all' && (
                      <div className="cursor-grab active:cursor-grabbing text-slate-600 hover:text-slate-400">
                        <GripVertical className="w-5 h-5" />
                      </div>
                    )}
                    <button
                      onClick={() => toggleTodo(todo.id)}
                      className={`shrink-0 w-6 h-6 rounded-full border-2 flex items-center justify-center transition-colors ${
                        todo.completed 
                          ? 'bg-emerald-500/100 border-emerald-500 text-white' 
                          : 'border-atc-border-hover hover:border-indigo-400 text-transparent'
                      }`}
                    >
                      <CheckCircle className="w-4 h-4" />
                    </button>
                    <div className="flex flex-col flex-1 overflow-hidden">
                      <span 
                        className={`text-sm break-words flex-1 transition-all ${
                          todo.completed ? 'text-slate-500 line-through' : 'text-slate-300 font-medium'
                        }`}
                      >
                        {todo.text}
                      </span>
                      {(!todo.completed && (todo.recurring !== 'none' || todo.reminder)) && (
                        <div className="flex gap-2 mt-1">
                          {todo.recurring && todo.recurring !== 'none' && (
                            <span className="flex items-center gap-1 text-[10px] font-mono text-atc-cyan bg-atc-cyan/10 px-2 py-0.5 rounded">
                              <Repeat className="w-3 h-3" /> {todo.recurring}
                            </span>
                          )}
                          {todo.reminder && (
                            <span className={`flex items-center gap-1 text-[10px] font-mono px-2 py-0.5 rounded ${todo.reminder < Date.now() ? 'text-red-400 bg-red-400/10' : 'text-amber-400 bg-amber-400/10'}`}>
                              <Bell className="w-3 h-3" /> {new Date(todo.reminder).toLocaleString()}
                            </span>
                          )}
                        </div>
                      )}
                    </div>
                  </div>
                  
                  <div className="flex items-center gap-2 ml-4 shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
                    <button
                      onClick={() => deleteTodo(todo.id)}
                      className="p-2 text-slate-500 hover:text-atc-pink hover:bg-atc-pink/10 rounded-lg transition-colors"
                      title="Delete task"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                </motion.div>
              ))
            )}
          </AnimatePresence>
        </div>
      </div>

      <AnimatePresence>
        {showSaveToast && (
          <motion.div
            initial={{ opacity: 0, y: 50, scale: 0.9 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: 20, scale: 0.9 }}
            className="fixed bottom-6 right-6 flex items-center gap-2 px-4 py-2 bg-[#090b14]/90 backdrop-blur-md border border-emerald-500/30 rounded-full shadow-[0_0_15px_rgba(16,185,129,0.2)] text-emerald-400 z-50 font-mono text-xs"
          >
            <CheckCircle className="w-4 h-4" />
            <span>Auto-saved at {lastSaveTime}</span>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
