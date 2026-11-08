import React, { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { 
  Check, 
  Plus, 
  Trash2, 
  ListTodo, 
  Search, 
  ArrowUpDown, 
  Info, 
  Archive, 
  ChevronRight, 
  ChevronDown, 
  Calendar, 
  CalendarClock, 
  Tag, 
  Settings, 
  X,
  Keyboard,
  FileText,
  Download,
  Upload,
  GripVertical,
  Mic,
  HelpCircle,
  Bookmark,
  Sun,
  Moon,
  Activity,
  Flame,
  TrendingUp,
  CheckCircle2,
  BarChart2,
  PieChart as PieIcon,
  RotateCcw,
  Sparkles,
  Filter,
  Cpu,
  Terminal,
  Shield,
  Layers,
  HardDrive,
  FolderPlus,
  BookOpen,
  CheckCheck,
  Zap
} from 'lucide-react';
import { PieChart, Pie, Cell, LineChart, Line, BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer, Legend } from 'recharts';
import { 
  PROJECT_PRESETS, 
  KERNEL_SUBTASK_PRESETS, 
  ProjectPreset, 
  SubtaskTemplatePreset 
} from '../data/todoPresets';

export type Subtask = { id: string; text: string; done: boolean; };
export type TodoItem = {
  id: string;
  text: string;
  done: boolean;
  category?: string;
  priority?: 'Low' | 'Medium' | 'High';
  expanded?: boolean;
  expandedNotes?: boolean;
  subtasks?: Subtask[];
  dueDate?: string;
  notes?: string;
  completedAt?: string;
  recurring?: 'none' | 'daily' | 'weekly' | 'monthly';
};

export type SubtaskSet = { id: string; name: string; subtasks: string[] };

export function TodoApp() {
  const [subtaskSets, setSubtaskSets] = useState<SubtaskSet[]>(() => {
    try {
      const saved = localStorage.getItem("atos-subtask-sets");
      if (saved) {
        const parsed: SubtaskSet[] = JSON.parse(saved);
        const existingIds = new Set(parsed.map(s => s.id));
        const missingKernel = KERNEL_SUBTASK_PRESETS.filter(k => !existingIds.has(k.id));
        return [...parsed, ...missingKernel];
      }
    } catch (e) {}
    return [
      { id: '1', name: 'Standard Bugfix', subtasks: ['Code analysieren', 'Lokalen Fix testen', 'Einchecken'] },
      ...KERNEL_SUBTASK_PRESETS
    ];
  });

  const [todos, setTodos] = useState<TodoItem[]>(() => {
    try {
      const saved = localStorage.getItem("atos-todos");
      if (saved) return JSON.parse(saved);
    } catch (e) {}
    return [
      { 
        id: '1', 
        text: 'ATOS Engine initialisieren', 
        done: true, 
        category: 'Work', 
        priority: 'High', 
        subtasks: [{ id: '1-1', text: 'Core Modul importieren', done: true }],
        dueDate: '2026-06-10',
        notes: 'Sicherstellen, dass alle Abhängigkeiten korrekt geladen und konfiguriert sind.'
      },
      { 
        id: '2', 
        text: 'Frontend UI anpassen', 
        done: false, 
        category: 'Bug', 
        priority: 'Medium',
        dueDate: '2026-06-20',
        subtasks: [
          { id: '2-1', text: 'Shortcuts hinzufügen', done: false },
          { id: '2-2', text: 'Styling überarbeiten', done: true }
        ],
        expanded: true,
        notes: 'Verwendung von Framer Motion auf der gesamten Aufgaben-Umschaltseite.'
      }
    ];
  });

  const [archivedTodos, setArchivedTodos] = useState<TodoItem[]>(() => {
    try {
      const saved = localStorage.getItem("atos-archived-todos");
      if (saved) return JSON.parse(saved);
    } catch (e) {}
    return [
      { id: 'h1', text: 'Datenbankschema entworfen', done: true, category: 'Work', priority: 'High', completedAt: '2026-06-11' },
      { id: 'h2', text: 'Dunkelmodus-Farben optimiert', done: true, category: 'Personal', priority: 'Medium', completedAt: '2026-06-12' },
      { id: 'h3', text: 'Typisierungsfehler behoben', done: true, category: 'Bug', priority: 'High', completedAt: '2026-06-13' }
    ];
  });
  const [input, setInput] = useState('');
  const [dueDate, setDueDate] = useState('');
  const [searchQuery, setSearchQuery] = useState('');
  const [filter, setFilter] = useState<'all' | 'active' | 'completed'>('all');
  const [filterCategories, setFilterCategories] = useState<string[]>([]);
  const [recurring, setRecurring] = useState<'none' | 'daily' | 'weekly' | 'monthly'>('none');
  const [draggedId, setDraggedId] = useState<string | null>(null);
  const [footerTab, setFooterTab] = useState<'status' | 'trends' | 'priority' | 'heatmap'>('status');
  
  // Categories state
  const [categories, setCategories] = useState<string[]>(() => {
    try {
      const saved = localStorage.getItem("atos-todo-categories");
      if (saved) return JSON.parse(saved);
    } catch (e) {}
    return ['Work', 'Personal', 'Bug', 'Idea'];
  });

  // Custom Category Colors state
  const [categoryColors, setCategoryColors] = useState<Record<string, string>>(() => {
    try {
      const saved = localStorage.getItem("atos-todo-category-colors");
      if (saved) return JSON.parse(saved);
    } catch (e) {}
    return {
      'Work': '#3b82f6',     // blue
      'Personal': '#a855f7', // purple
      'Bug': '#ef4444',      // red
      'Idea': '#14b8a6',     // teal
    };
  });
  
  // Settings Panel state
  const [showSettingsPanel, setShowSettingsPanel] = useState(false);
  const [defaultNewCategory, setDefaultNewCategory] = useState(() => {
    try {
      const saved = localStorage.getItem("atos-todo-default-cat");
      if (saved) return saved;
    } catch (e) {}
    return 'Work';
  });

  const [selectedCategory, setSelectedCategory] = useState(defaultNewCategory);
  const [selectedPriority, setSelectedPriority] = useState<'Low'|'Medium'|'High'>('Medium');
  const [sortByPriority, setSortByPriority] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editText, setEditText] = useState('');
  const [editingSubtaskId, setEditingSubtaskId] = useState<string | null>(null);
  const [editSubtext, setEditSubtext] = useState('');
  const [subtaskInputId, setSubtaskInputId] = useState<string | null>(null);
  const [subtaskInput, setSubtaskInput] = useState('');
  
  // Presets & Templates state
  const [showPresetModal, setShowPresetModal] = useState(false);
  const [selectedPresetId, setSelectedPresetId] = useState<string>(PROJECT_PRESETS[0]?.id || 'baremetal-x86-64');
  const [presetTab, setPresetTab] = useState<'projects' | 'subtasks'>('projects');
  const [expandedPresetTaskIndex, setExpandedPresetTaskIndex] = useState<number | null>(null);

  // Custom Category Label state
  const [showManageCat, setShowManageCat] = useState(false);
  const [showInsightsPanel, setShowInsightsPanel] = useState(false);
  const [insightsTimeframe, setInsightsTimeframe] = useState<7 | 14 | 30>(14);
  const [insightsTab, setInsightsTab] = useState<'stacked' | 'donut' | 'trend'>('stacked');
  const [newCatName, setNewCatName] = useState('');
  const [newCatColor, setNewCatColor] = useState('#8b5cf6');

  // Selected task state for global keyboard shortcuts
  const [selectedTodoId, setSelectedTodoId] = useState<string | null>(null);

  // Custom modals/alerts and file import state
  const [deleteConfirm, setDeleteConfirm] = useState<{
    type: 'todo' | 'subtask';
    todoId: string;
    subtaskId?: string;
    itemName: string;
  } | null>(null);
  const [alertMessage, setAlertMessage] = useState<string | null>(null);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  // Voice command state
  const [isListening, setIsListening] = useState(false);
  const [showVoiceHelp, setShowVoiceHelp] = useState(false);

  // Global theme
  const [globalTheme, setGlobalTheme] = useState(() => localStorage.getItem("atos-theme") || "ATOS Dark");

  useEffect(() => {
    const handleSync = (e: any) => {
      if (e.detail && typeof e.detail === 'string') {
        setGlobalTheme(e.detail);
      }
    };
    window.addEventListener('atos-theme-change', handleSync);
    return () => window.removeEventListener('atos-theme-change', handleSync);
  }, []);

  const toggleGlobalTheme = () => {
    const newTheme = globalTheme === "ATOS Dark" ? "Light Mode" : "ATOS Dark";
    setGlobalTheme(newTheme);
    const event = new CustomEvent('atos-theme-change', { detail: newTheme });
    window.dispatchEvent(event);
  };

  // Context menu position to change priority
  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    todoId: string;
  } | null>(null);

  const inputRef = useRef<HTMLInputElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  // Particle confetti refs
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const particlesRef = useRef<any[]>([]);
  const animatingRef = useRef<boolean>(false);

  // Save state to localStorage
  useEffect(() => {
    localStorage.setItem("atos-todos", JSON.stringify(todos));
  }, [todos]);

  useEffect(() => {
    localStorage.setItem("atos-archived-todos", JSON.stringify(archivedTodos));
  }, [archivedTodos]);

  useEffect(() => {
    localStorage.setItem("atos-todo-categories", JSON.stringify(categories));
  }, [categories]);

  useEffect(() => {
    localStorage.setItem("atos-todo-category-colors", JSON.stringify(categoryColors));
  }, [categoryColors]);

  useEffect(() => {
    localStorage.setItem("atos-todo-default-cat", defaultNewCategory);
  }, [defaultNewCategory]);

  useEffect(() => {
    localStorage.setItem("atos-subtask-sets", JSON.stringify(subtaskSets));
  }, [subtaskSets]);

  // Close context menu on outside click
  useEffect(() => {
    const closeMenu = () => setContextMenu(null);
    window.addEventListener('click', closeMenu);
    return () => window.removeEventListener('click', closeMenu);
  }, []);

  // Helper for computing today's YYYY-MM-DD
  const getTodayDateStr = () => {
    const d = new Date();
    const year = d.getFullYear();
    const month = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  };

  // Toast Notification for tasks due on current date upon load
  useEffect(() => {
    const todayStr = getTodayDateStr();
    const dueToday = todos.filter(t => !t.done && t.dueDate === todayStr);
    if (dueToday.length > 0) {
      setToastMessage(`Heute fällig: Sie haben ${dueToday.length} Aufgabe${dueToday.length > 1 ? 'n' : ''} für heute!`);
    } else {
      setToastMessage("Keine fälligen Aufgaben für heute.");
    }
  }, []);

  // Auto-dismiss Toast Timer
  useEffect(() => {
    if (toastMessage) {
      const timer = setTimeout(() => {
        setToastMessage(null);
      }, 5500);
      return () => clearTimeout(timer);
    }
  }, [toastMessage]);

  const handleEditStart = (id: string, text: string) => {
    setEditingId(id);
    setEditText(text);
  };

  const handleEditSave = () => {
    if (editingId) {
      setTodos(todos.map(t => t.id === editingId ? { ...t, text: editText.trim() || t.text } : t));
      setEditingId(null);
    }
  };

  const calculateNextDueDate = (currentDueDateStr: string, recurringType: 'daily' | 'weekly' | 'monthly'): string => {
    const baseDate = currentDueDateStr ? new Date(currentDueDateStr + 'T00:00:00') : new Date();
    if (isNaN(baseDate.getTime())) {
      return currentDueDateStr;
    }
    if (recurringType === 'daily') {
      baseDate.setDate(baseDate.getDate() + 1);
    } else if (recurringType === 'weekly') {
      baseDate.setDate(baseDate.getDate() + 7);
    } else if (recurringType === 'monthly') {
      baseDate.setMonth(baseDate.getMonth() + 1);
    }
    
    const year = baseDate.getFullYear();
    const month = String(baseDate.getMonth() + 1).padStart(2, '0');
    const day = String(baseDate.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  };

  const toggleTodoDone = (id: string) => {
    let triggeredConfetti = false;
    setTodos(prev => {
      const task = prev.find(t => t.id === id);
      if (!task) return prev;

      const isNowCompleted = !task.done;
      if (isNowCompleted) {
        triggeredConfetti = true;
      }

      const updated = prev.map(t => {
        if (t.id === id) {
          return {
            ...t,
            done: isNowCompleted,
            completedAt: isNowCompleted ? getTodayDateStr() : undefined
          };
        }
        return t;
      });

      if (isNowCompleted && task.recurring && task.recurring !== 'none') {
        const currentDue = task.dueDate || getTodayDateStr();
        const nextDue = calculateNextDueDate(currentDue, task.recurring);
        const nextId = Date.now().toString() + Math.random().toString(36).substr(2, 4);
        
        const duplicatedTask: TodoItem = {
          id: nextId,
          text: task.text,
          done: false,
          category: task.category,
          priority: task.priority,
          dueDate: nextDue,
          notes: task.notes,
          recurring: task.recurring,
          subtasks: task.subtasks?.map(st => ({
            id: nextId + '-' + Math.random().toString(36).substr(2, 3),
            text: st.text,
            done: false
          })),
          expanded: false
        };
        
        return [...updated, duplicatedTask];
      }

      return updated;
    });

    if (triggeredConfetti) {
      spawnConfetti();
    }
  };

  // Text highlight helper for search functionality
  const highlightText = (text: string, search: string) => {
    if (!search.trim()) return <span>{text}</span>;
    const regex = new RegExp(`(${search.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&')})`, 'gi');
    const parts = text.split(regex);
    return (
      <span>
        {parts.map((part, i) => 
          regex.test(part) ? (
            <mark key={i} className="bg-amber-400 text-slate-950 font-bold px-0.5 rounded shadow-xs">
              {part}
            </mark>
          ) : (
            part
          )
        )}
      </span>
    );
  };

  // Drag and Drop reordering handlers
  const handleDragStart = (e: React.DragEvent, id: string) => {
    setDraggedId(id);
    e.dataTransfer.effectAllowed = 'move';
  };

  const handleDragOver = (e: React.DragEvent, id: string) => {
    e.preventDefault();
    if (!draggedId || draggedId === id) return;
    
    const draggedIdx = todos.findIndex(t => t.id === draggedId);
    const overIdx = todos.findIndex(t => t.id === id);
    if (draggedIdx === -1 || overIdx === -1) return;
    
    const updated = [...todos];
    const [removed] = updated.splice(draggedIdx, 1);
    updated.splice(overIdx, 0, removed);
    setTodos(updated);
  };

  const handleDragEnd = () => {
    setDraggedId(null);
  };

  // Canvas-based dynamic resize and lifecycle
  useEffect(() => {
    const handleResize = () => {
      const canvas = canvasRef.current;
      if (canvas && canvas.parentElement) {
        canvas.width = canvas.parentElement.clientWidth;
        canvas.height = canvas.parentElement.clientHeight;
      }
    };
    handleResize();
    const observer = new ResizeObserver(handleResize);
    const canvas = canvasRef.current;
    if (canvas && canvas.parentElement) {
      observer.observe(canvas.parentElement);
    }
    return () => {
      observer.disconnect();
    };
  }, []);

  const spawnConfetti = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const colors = ['#10b981', '#3b82f6', '#8b5cf6', '#ec4899', '#f59e0b', '#ef4444'];
    const newParticles = [];
    
    // Create upward-moving colorful particles from lower-center part of the applet
    for (let i = 0; i < 90; i++) {
      const angle = (Math.PI / 4) + Math.random() * (Math.PI / 2) + Math.PI; // Upward angle range
      const speed = 2 + Math.random() * 6;
      newParticles.push({
        x: canvas.width / 2,
        y: canvas.height * 0.7,
        vx: Math.cos(angle) * speed + (Math.random() - 0.5) * 1.5,
        vy: Math.sin(angle) * speed - 1.5,
        color: colors[Math.floor(Math.random() * colors.length)],
        radius: 2 + Math.random() * 3.5,
        rotation: Math.random() * 360,
        rotationSpeed: -8 + Math.random() * 16,
        alpha: 1,
        shape: ['circle', 'square', 'triangle'][Math.floor(Math.random() * 3)]
      });
    }
    
    particlesRef.current = [...particlesRef.current, ...newParticles];
    
    if (!animatingRef.current) {
      animatingRef.current = true;
      const animate = () => {
        const currentCanvas = canvasRef.current;
        if (!currentCanvas) {
          animatingRef.current = false;
          return;
        }
        const currentCtx = currentCanvas.getContext('2d');
        if (!currentCtx) {
          animatingRef.current = false;
          return;
        }
        
        currentCtx.clearRect(0, 0, currentCanvas.width, currentCanvas.height);
        
        const list = particlesRef.current;
        for (let i = list.length - 1; i >= 0; i--) {
          const p = list[i];
          p.x += p.vx;
          p.y += p.vy;
          p.vy += 0.14; // gravity force
          p.vx *= 0.99; // air friction
          p.rotation += p.rotationSpeed;
          p.alpha -= 0.013; // fade transition speed
          
          if (p.alpha <= 0 || p.y > currentCanvas.height) {
            list.splice(i, 1);
            continue;
          }
          
          currentCtx.save();
          currentCtx.globalAlpha = p.alpha;
          currentCtx.translate(p.x, p.y);
          currentCtx.rotate((p.rotation * Math.PI) / 180);
          currentCtx.fillStyle = p.color;
          
          if (p.shape === 'circle') {
            currentCtx.beginPath();
            currentCtx.arc(0, 0, p.radius, 0, Math.PI * 2);
            currentCtx.fill();
          } else if (p.shape === 'triangle') {
            currentCtx.beginPath();
            currentCtx.moveTo(0, -p.radius);
            currentCtx.lineTo(p.radius, p.radius);
            currentCtx.lineTo(-p.radius, p.radius);
            currentCtx.closePath();
            currentCtx.fill();
          } else {
            currentCtx.fillRect(-p.radius, -p.radius, p.radius * 2, p.radius * 2);
          }
          currentCtx.restore();
        }
        
        if (list.length > 0) {
          requestAnimationFrame(animate);
        } else {
          animatingRef.current = false;
        }
      };
      
      animate();
    }
  };

  const deleteTodo = (id: string) => {
    const todo = todos.find(t => t.id === id);
    if (!todo) return;
    setDeleteConfirm({
      type: 'todo',
      todoId: id,
      itemName: todo.text
    });
  };

  const executeDeleteTodo = (id: string) => {
    setTodos(prev => {
      const updated = prev.filter(t => t.id !== id);
      if (selectedTodoId === id) {
        const index = prev.findIndex(t => t.id === id);
        if (updated.length > 0) {
          const nextIndex = Math.min(index, updated.length - 1);
          setSelectedTodoId(updated[nextIndex].id);
        } else {
          setSelectedTodoId(null);
        }
      }
      return updated;
    });
    setDeleteConfirm(null);
  };

  const deleteSubtask = (todoId: string, subtaskId: string) => {
    const todo = todos.find(t => t.id === todoId);
    const subtask = todo?.subtasks?.find(st => st.id === subtaskId);
    if (!subtask) return;
    setDeleteConfirm({
      type: 'subtask',
      todoId,
      subtaskId,
      itemName: subtask.text
    });
  };

  const executeDeleteSubtask = (todoId: string, subtaskId: string) => {
    setTodos(todos.map(t => {
      if (t.id === todoId) {
        return {
          ...t,
          subtasks: t.subtasks?.filter(st => st.id !== subtaskId)
        };
      }
      return t;
    }));
    setDeleteConfirm(null);
  };

  const addTodo = () => {
    if (!input.trim()) return;
    const newTodo: TodoItem = { 
      id: Date.now().toString(), 
      text: input.trim(), 
      done: false, 
      category: selectedCategory, 
      priority: selectedPriority, 
      recurring: recurring !== 'none' ? recurring : undefined,
      expanded: true, 
      subtasks: [],
      dueDate: dueDate || undefined
    };
    setTodos(prev => [...prev, newTodo]);
    setInput('');
    setDueDate('');
    setRecurring('none');
    setSelectedTodoId(newTodo.id); // auto-select the newly created item
  };

  const handleSubtaskEditStart = (subtaskId: string, text: string) => {
    setEditingSubtaskId(subtaskId);
    setEditSubtext(text);
  };

  const handleSubtaskEditSave = (todoId: string) => {
    if (editingSubtaskId) {
      setTodos(todos.map(t => {
        if (t.id === todoId) {
          return {
            ...t,
            subtasks: t.subtasks?.map(st => st.id === editingSubtaskId ? { ...st, text: editSubtext.trim() || st.text } : st)
          };
        }
        return t;
      }));
      setEditingSubtaskId(null);
    }
  };

  const handleSaveSubtaskSet = (todoId: string) => {
    const todo = todos.find(t => t.id === todoId);
    if (!todo || !todo.subtasks || todo.subtasks.length === 0) return;
    const name = window.prompt("Name für diese Unteraufgaben-Vorlage:");
    if (!name?.trim()) return;
    const texts = todo.subtasks.map(st => st.text);
    const newSet: SubtaskSet = { id: Date.now().toString(), name: name.trim(), subtasks: texts };
    setSubtaskSets([...subtaskSets, newSet]);
    setToastMessage(`Vorlage "${name}" gespeichert`);
  };

  const handleLoadSubtaskSet = (todoId: string, setId: string) => {
    if (!setId) return;
    const set = subtaskSets.find(s => s.id === setId);
    if (!set) return;
    setTodos(todos.map(t => {
      if (t.id === todoId) {
        const newSubtasks = set.subtasks.map((text, i) => ({
          id: `${Date.now()}-${i}-${Math.random().toString(36).substr(2, 4)}`,
          text,
          done: false
        }));
        return { ...t, subtasks: [...(t.subtasks || []), ...newSubtasks] };
      }
      return t;
    }));
    setToastMessage(`Vorlage "${set.name}" angewendet (${set.subtasks.length} Unteraufgaben)`);
  };

  const applyProjectPreset = (preset: ProjectPreset, mode: 'append' | 'replace' = 'append') => {
    // 1. Merge categories and colors
    const newCats = [...categories];
    const newColors = { ...categoryColors };
    preset.categories.forEach(c => {
      if (!newCats.includes(c.name)) {
        newCats.push(c.name);
      }
      if (!newColors[c.name]) {
        newColors[c.name] = c.color;
      }
    });
    setCategories(newCats);
    setCategoryColors(newColors);

    // 2. Generate new todo items with subtasks and notes
    const today = new Date();
    const newTodos: TodoItem[] = preset.tasks.map((task, idx) => {
      let dueStr: string | undefined = undefined;
      if (typeof task.dueDateOffsetDays === 'number') {
        const targetDate = new Date();
        targetDate.setDate(today.getDate() + task.dueDateOffsetDays);
        const y = targetDate.getFullYear();
        const m = String(targetDate.getMonth() + 1).padStart(2, '0');
        const d = String(targetDate.getDate()).padStart(2, '0');
        dueStr = `${y}-${m}-${d}`;
      }
      return {
        id: `preset-${Date.now()}-${idx}-${Math.random().toString(36).substring(2, 6)}`,
        text: task.text,
        done: false,
        category: task.category,
        priority: task.priority,
        expanded: true,
        expandedNotes: !!task.notes,
        notes: task.notes,
        dueDate: dueStr,
        subtasks: task.subtasks.map((subText, sIdx) => ({
          id: `sub-${Date.now()}-${idx}-${sIdx}`,
          text: subText,
          done: false
        }))
      };
    });

    if (mode === 'replace') {
      setTodos(newTodos);
      if (newTodos.length > 0) {
        setSelectedTodoId(newTodos[0].id);
      }
      setToastMessage(`Projekt "${preset.title}" geladen (${newTodos.length} Aufgaben)`);
    } else {
      setTodos(prev => [...newTodos, ...prev]);
      if (newTodos.length > 0) {
        setSelectedTodoId(newTodos[0].id);
      }
      setToastMessage(`${newTodos.length} Aufgaben aus "${preset.title}" hinzugefügt`);
    }
    setShowPresetModal(false);
  };

  const applySubtaskPresetToNewTodo = (subPreset: SubtaskTemplatePreset) => {
    const newCat = categories.includes(subPreset.category) ? subPreset.category : (categories[0] || 'Work');
    const newTodo: TodoItem = {
      id: `preset-task-${Date.now()}`,
      text: `${subPreset.name} implementieren`,
      done: false,
      category: newCat,
      priority: 'High',
      expanded: true,
      subtasks: subPreset.subtasks.map((text, i) => ({
        id: `sub-${Date.now()}-${i}`,
        text,
        done: false
      }))
    };
    setTodos(prev => [newTodo, ...prev]);
    setSelectedTodoId(newTodo.id);
    setToastMessage(`Aufgabe "${subPreset.name}" mit ${subPreset.subtasks.length} Checklisten-Schritten erstellt`);
    setShowPresetModal(false);
  };

  const addSubtask = (todoId: string) => {
    if (!subtaskInput.trim()) return;
    setTodos(todos.map(t => {
      if (t.id === todoId) {
        return {
          ...t,
          subtasks: [...(t.subtasks || []), { id: Date.now().toString(), text: subtaskInput.trim(), done: false }]
        };
      }
      return t;
    }));
    setSubtaskInputId(null);
    setSubtaskInput('');
  };

  const exportTodosJSON = () => {
    try {
      const dataStr = JSON.stringify(todos, null, 2);
      const dataUri = 'data:application/json;charset=utf-8,'+ encodeURIComponent(dataStr);
      const exportFileDefaultName = `lumino_tasks_backup_${new Date().toISOString().slice(0,10)}.json`;
      
      const linkElement = document.createElement('a');
      linkElement.setAttribute('href', dataUri);
      linkElement.setAttribute('download', exportFileDefaultName);
      linkElement.click();
    } catch (e) {
      setAlertMessage('Fehler beim Exportieren der Aufgaben-Backup-Datei.');
    }
  };

  const importTodosJSON = (e: React.ChangeEvent<HTMLInputElement>) => {
    const fileReader = new FileReader();
    const files = e.target.files;
    if (!files || files.length === 0) return;
    
    fileReader.onload = event => {
      try {
        const parsed = JSON.parse(event.target?.result as string);
        if (Array.isArray(parsed)) {
          const isValid = parsed.every(item => item && typeof item === 'object' && 'text' in item);
          if (isValid) {
            setTodos(parsed);
            if (parsed.length > 0) {
              setSelectedTodoId(parsed[0].id);
            }
            setAlertMessage('Backup wurde erfolgreich geladen!');
          } else {
            setAlertMessage('Ungültiges Dateiformat. Die JSON-Datei entspricht nicht dem Lumino-Task-Schema.');
          }
        } else {
          setAlertMessage('Ungültiges Dateiformat: Das Wurzelelement muss ein Array sein.');
        }
      } catch (error) {
        setAlertMessage('Fehler beim Lesen der JSON-Datei. Vergewissern Sie sich, dass es sich um eine gültige Backup-JSON-Datei handelt.');
      }
    };
    fileReader.readAsText(files[0]);
    e.target.value = ''; // Reset input target
  };

  const startVoiceRecognition = () => {
    if (!('webkitSpeechRecognition' in window) && !('SpeechRecognition' in window)) {
      setAlertMessage("Spracherkennung wird in diesem Browser leider nicht unterstützt.");
      return;
    }
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    const recognition = new SpeechRecognition();
    recognition.lang = 'de-DE'; // Assuming german since UI is partially german
    recognition.interimResults = false;
    recognition.maxAlternatives = 1;

    recognition.onstart = () => {
      setIsListening(true);
    };

    recognition.onresult = (event: any) => {
      const transcript = event.results[0][0].transcript;
      
      // Simple command parsing
      const lower = transcript.toLowerCase();
      if (lower.startsWith('füge aufgabe hinzu ') || lower.startsWith('add task ')) {
        const text = transcript.replace(/^(füge aufgabe hinzu|add task)\s+/i, '');
        // auto add
        const newTodo: TodoItem = { 
          id: Date.now().toString(), 
          text: text.charAt(0).toUpperCase() + text.slice(1), 
          done: false, 
          category: selectedCategory, 
          priority: selectedPriority, 
          expanded: true, 
          subtasks: []
        };
        setTodos(prev => [...prev, newTodo]);
        setToastMessage(`Aufgabe per Sprache hinzugefügt: ${text}`);
      } else {
        // Just fill the input
        setInput(transcript);
        inputRef.current?.focus();
      }
    };

    recognition.onerror = (event: any) => {
      console.error("Speech recognition error", event.error);
      setIsListening(false);
    };

    recognition.onend = () => {
      setIsListening(false);
    };

    recognition.start();
  };

  const toggleSubtask = (todoId: string, subtaskId: string) => {
    let triggeredConfetti = false;
    setTodos(prev => prev.map(t => {
      if (t.id === todoId) {
        const updated = t.subtasks?.map(st => st.id === subtaskId ? { ...st, done: !st.done } : st) || [];
        const hasSubAlready = updated.length > 0;
        const allCompletedNow = hasSubAlready && updated.every(st => st.done);
        
        // Ensure they weren't already completed before this toggle
        const wereAllCompletedBefore = t.subtasks?.length ? t.subtasks.every(st => st.done) : false;
        
        if (allCompletedNow && !wereAllCompletedBefore) {
          triggeredConfetti = true;
        }
        
        return {
          ...t,
          subtasks: updated
        };
      }
      return t;
    }));
    
    if (triggeredConfetti) {
      spawnConfetti();
    }
  };

  const toggleTaskExpanded = (todoId: string) => {
    setTodos(todos.map(t => t.id === todoId ? { ...t, expanded: !t.expanded } : t));
  };

  const archiveCompleted = () => {
    const completed = todos.filter(t => t.done);
    if (completed.length === 0) return;
    setArchivedTodos([...archivedTodos, ...completed]);
    setTodos(todos.filter(t => !t.done));
    setSelectedTodoId(null);
  };

  const archiveSingleTodo = (todoId: string) => {
    const todo = todos.find(t => t.id === todoId);
    if (!todo || !todo.done) return;
    setArchivedTodos([...archivedTodos, todo]);
    setTodos(todos.filter(t => t.id !== todoId));
    if (selectedTodoId === todoId) setSelectedTodoId(null);
    setToastMessage(`"${todo.text}" archiviert`);
  };

  // Get Hex Category custom color
  const getCategoryColor = (cat?: string) => {
    if (!cat) return '#64748b';
    return categoryColors[cat] || '#64748b';
  };

  // Dynamic CSS-friendly styles based on custom theme colors
  const getCategoryStyle = (cat?: string) => {
    if (!cat) return {};
    const baseColor = getCategoryColor(cat);
    return {
      backgroundColor: `${baseColor}1d`,
      borderColor: `${baseColor}44`,
      color: baseColor
    };
  };

  const getPriorityColor = (pri?: string) => {
    switch(pri) {
      case 'High': return 'bg-red-500/20 text-red-400 border-red-500/30';
      case 'Medium': return 'bg-amber-500/20 text-amber-400 border-amber-500/30';
      case 'Low': return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30';
      default: return 'bg-slate-500/20 text-slate-400 border-slate-500/30';
    }
  };

  // Filter and sort todos
  let filteredTodos = todos.filter(todo => {
    const q = searchQuery.toLowerCase();
    const textMatch = todo.text.toLowerCase().includes(q);
    const catMatch = (todo.category || '').toLowerCase().includes(q);
    const notesMatch = (todo.notes || '').toLowerCase().includes(q);
    
    if (!textMatch && !catMatch && !notesMatch) return false;
    
    // Filter by assigned category toggle
    if (filterCategories.length > 0 && (!todo.category || !filterCategories.includes(todo.category))) return false;
    
    if (filter === 'active') return !todo.done;
    if (filter === 'completed') return todo.done;
    return true;
  });

  if (sortByPriority) {
    const pValue = { High: 3, Medium: 2, Low: 1 };
    filteredTodos.sort((a, b) => {
      const pA = pValue[a.priority as keyof typeof pValue] || 0;
      const pB = pValue[b.priority as keyof typeof pValue] || 0;
      return pB - pA; // High priority first
    });
  }

  // Overdue calculation
  const checkIfOverdue = (dateStr?: string, done?: boolean) => {
    if (!dateStr || done) return false;
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const due = new Date(dateStr + 'T00:00:00');
    return due < today;
  };

  const formatDueDate = (dateStr: string) => {
    try {
      const date = new Date(dateStr + 'T00:00:00');
      return date.toLocaleDateString(undefined, { day: '2-digit', month: 'short' });
    } catch {
      return dateStr;
    }
  };

  const dueRecurringTasks = todos.filter(t => !t.done && t.recurring && t.recurring !== 'none' && t.dueDate && (t.dueDate === getTodayDateStr() || checkIfOverdue(t.dueDate)));

  // Category Manager Helpers
  const handleAddCategory = () => {
    const trimmed = newCatName.trim();
    if (!trimmed) return;
    if (!categories.includes(trimmed)) {
      setCategories([...categories, trimmed]);
      setCategoryColors(prev => ({ ...prev, [trimmed]: newCatColor }));
      setSelectedCategory(trimmed);
    }
    setNewCatName('');
  };

  const handleDeleteCategory = (catToDelete: string) => {
    setCategories(categories.filter(c => c !== catToDelete));
    if (selectedCategory === catToDelete) {
      setSelectedCategory(categories[0] || 'Work');
    }
  };

  // Global Keyboard Shortcuts Effect
  useEffect(() => {
    const handleGlobalShortcuts = (e: KeyboardEvent) => {
      const activeTag = document.activeElement?.tagName;
      const isTyping = activeTag === 'INPUT' || activeTag === 'TEXTAREA' || document.activeElement?.hasAttribute('contenteditable');
      
      if (isTyping) {
        if (e.key === 'Escape') {
          (document.activeElement as HTMLElement)?.blur();
        }
        return;
      }

      // 'n' - Focus Add Task input field
      if (e.key.toLowerCase() === 'n') {
        e.preventDefault();
        inputRef.current?.focus();
        return;
      }

      // 'Delete' or 'Backspace' - Remove currently selected task
      if (e.key === 'Delete' || e.key === 'Backspace') {
        if (selectedTodoId) {
          e.preventDefault();
          deleteTodo(selectedTodoId);
        }
        return;
      }

      // 'ArrowUp' - Navigate selection upwards
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        if (filteredTodos.length === 0) return;
        const currentIdx = filteredTodos.findIndex(t => t.id === selectedTodoId);
        if (currentIdx === -1) {
          setSelectedTodoId(filteredTodos[filteredTodos.length - 1].id);
        } else {
          const prevIdx = (currentIdx - 1 + filteredTodos.length) % filteredTodos.length;
          setSelectedTodoId(filteredTodos[prevIdx].id);
        }
        return;
      }

      // 'ArrowDown' - Navigate selection downwards
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        if (filteredTodos.length === 0) return;
        const currentIdx = filteredTodos.findIndex(t => t.id === selectedTodoId);
        if (currentIdx === -1) {
          setSelectedTodoId(filteredTodos[0].id);
        } else {
          const nextIdx = (currentIdx + 1) % filteredTodos.length;
          setSelectedTodoId(filteredTodos[nextIdx].id);
        }
        return;
      }

      // 'Space' or 'Enter' - Toggle selection done state
      if (e.key === ' ' || e.key === 'Enter') {
        if (selectedTodoId) {
          e.preventDefault();
          toggleTodoDone(selectedTodoId);
        }
        return;
      }

      // 'c' or 'C' - Toggle task completion status
      if (e.key.toLowerCase() === 'c') {
        if (selectedTodoId) {
          e.preventDefault();
          toggleTodoDone(selectedTodoId);
        }
        return;
      }

      // 'ArrowLeft' - Collapse selected task
      if (e.key === 'ArrowLeft') {
        if (selectedTodoId) {
          e.preventDefault();
          setTodos(prev => prev.map(t => t.id === selectedTodoId ? { ...t, expanded: false } : t));
        }
        return;
      }

      // 'ArrowRight' - Expand selected task
      if (e.key === 'ArrowRight') {
        if (selectedTodoId) {
          e.preventDefault();
          setTodos(prev => prev.map(t => t.id === selectedTodoId ? { ...t, expanded: true } : t));
        }
        return;
      }
    };

    window.addEventListener('keydown', handleGlobalShortcuts);
    return () => {
      window.removeEventListener('keydown', handleGlobalShortcuts);
    };
  }, [selectedTodoId, filteredTodos, categories]);

  return (
    <div className="flex flex-col items-center justify-center p-4 font-sans h-full w-full bg-[#0c0c0e]">
      <div className="bg-slate-900 border border-white/10 rounded-xl p-4 w-[380px] shadow-2xl flex flex-col h-[610px] relative overflow-hidden">
        
        {/* Toast Notification */}
        <AnimatePresence>
          {toastMessage && (
            <motion.div
              initial={{ opacity: 0, y: -20, scale: 0.95 }}
              animate={{ opacity: 1, y: 0, scale: 1 }}
              exit={{ opacity: 0, y: -15, scale: 0.95 }}
              className="absolute top-3 left-3 right-3 z-[60] bg-slate-950/95 backdrop-blur-md border border-emerald-500/30 rounded-lg p-2.5 shadow-2xl flex items-center justify-between gap-3 text-emerald-400 select-none"
            >
              <div className="flex items-center gap-1.5 min-w-0">
                <span className="relative flex h-1.5 w-1.5 shrink-0">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-1.5 w-1.5 bg-emerald-500"></span>
                </span>
                <span className="text-[10px] font-mono leading-tight text-emerald-300 truncate">
                  {toastMessage}
                </span>
              </div>
              <button
                onClick={() => setToastMessage(null)}
                className="text-slate-500 hover:text-white transition-colors cursor-pointer shrink-0"
              >
                <X className="w-3 h-3" />
              </button>
            </motion.div>
          )}
        </AnimatePresence>
        
        {/* Header */}
        <div className="flex items-center justify-between mb-3 relative z-10">
          <div className="flex items-center gap-2 text-emerald-400">
             <ListTodo className="w-5 h-5" />
             <span className="font-bold text-sm tracking-tight">Lumino Tasks</span>
             <div className="group relative ml-1 flex items-center cursor-help">
               <Info className="w-4 h-4 text-slate-500 hover:text-slate-300 transition-colors" />
               <div className="absolute top-full left-0 mt-2 w-48 p-2 bg-slate-800 border border-slate-600 rounded-lg shadow-xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all z-50 text-[10px] text-slate-200">
                 <div className="font-bold mb-1 border-b border-white/10 pb-1 text-slate-400 uppercase tracking-widest text-[9px] flex items-center gap-1">
                   <Keyboard className="w-3 h-3" /> Shortcuts
                 </div>
                 <div className="space-y-1">
                   <div className="flex justify-between"><span>[N]</span> <span className="text-slate-400">Neuer Eintrag</span></div>
                   <div className="flex justify-between"><span>[↑ / ↓]</span> <span className="text-slate-400 text-right">Wählen</span></div>
                   <div className="flex justify-between"><span>[Enter / Space / C]</span> <span className="text-slate-400 text-right">Erledigt</span></div>
                   <div className="flex justify-between"><span>[← / →]</span> <span className="text-slate-400 text-right">Details box</span></div>
                   <div className="flex justify-between"><span>[Del / Backspace]</span> <span className="text-slate-400 text-right">Löschen</span></div>
                 </div>
               </div>
             </div>
          </div>
          <div className="flex items-center gap-1.5 sm:gap-2">
            <button
               onClick={() => setShowPresetModal(true)}
               className="p-1.5 px-2 rounded border border-indigo-500/30 bg-indigo-500/15 text-indigo-300 hover:bg-indigo-500/25 transition-colors flex items-center gap-1.5 text-xs font-semibold cursor-pointer shadow-xs"
               title="Bare Metal Kernel & Projekt-Presets öffnen"
            >
              <Cpu className="w-3.5 h-3.5 text-indigo-400" />
              <span className="text-[11px] font-mono hidden sm:inline">Presets</span>
            </button>
            <button
               onClick={toggleGlobalTheme}
               className="p-1.5 rounded border border-white/10 bg-transparent text-slate-400 hover:bg-white/5 transition-colors flex items-center justify-center"
               title={globalTheme === 'ATOS Dark' ? "Light Mode aktivieren" : "Dark Mode aktivieren"}
            >
              {globalTheme === 'ATOS Dark' ? <Sun className="w-4 h-4" /> : <Moon className="w-4 h-4" />}
            </button>
            <button
               onClick={() => setShowInsightsPanel(true)}
               className="p-1.5 rounded border border-white/10 bg-transparent text-slate-400 hover:bg-white/5 transition-colors flex items-center justify-center"
               title="Productivity Insights"
            >
              <Activity className="w-4 h-4" />
            </button>
            <button
               onClick={() => setShowSettingsPanel(!showSettingsPanel)}
               className={`p-1.5 rounded border transition-colors flex items-center justify-center ${showSettingsPanel ? 'bg-indigo-500/20 border-indigo-500/50 text-indigo-300' : 'bg-transparent border-white/10 text-slate-400 hover:bg-white/5'}`}
               title="Einstellungen & Kategorien"
            >
              <Settings className="w-4 h-4" />
            </button>
            <button
               onClick={archiveCompleted}
               className="p-1.5 rounded border bg-transparent border-white/10 text-slate-400 hover:bg-white/5 transition-colors flex items-center justify-center"
               title="Archive Completed"
               disabled={!todos.some(t => t.done)}
            >
              <Archive className="w-4 h-4" />
            </button>
            <button 
              onClick={() => setSortByPriority(!sortByPriority)}
              className={`p-1.5 rounded border transition-colors flex items-center justify-center ${sortByPriority ? 'bg-emerald-500/20 border-emerald-500/50 text-emerald-300' : 'bg-transparent border-white/10 text-slate-400 hover:bg-white/5'}`}
              title="Sort by Priority"
            >
              <ArrowUpDown className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Settings Panel */}
        {showSettingsPanel && (
          <div className="absolute top-[48px] left-4 right-4 z-[40] bg-slate-950 border border-indigo-500/30 rounded-lg p-3 shadow-2xl flex flex-col max-h-[340px]">
            <div className="flex justify-between items-center mb-3 border-b border-white/5 pb-1">
              <span className="text-xs font-bold text-indigo-400 flex items-center gap-1">
                <Settings className="w-3.5 h-3.5" /> Einstellungen
              </span>
              <button onClick={() => setShowSettingsPanel(false)} className="text-slate-500 hover:text-slate-300">
                <X className="w-3.5 h-3.5" />
              </button>
            </div>
            
            <div className="flex-1 overflow-y-auto pr-1 space-y-3">
              {/* Presets shortcut */}
              <div className="p-2 rounded-lg bg-indigo-950/40 border border-indigo-500/20 flex items-center justify-between gap-2">
                <div className="min-w-0">
                  <div className="text-xs font-semibold text-indigo-300 flex items-center gap-1.5">
                    <Cpu className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
                    <span>Kernel & Projekt-Presets</span>
                  </div>
                  <div className="text-[10px] text-slate-400 truncate mt-0.5">
                    Bare Metal x86_64, Rust OS & Treiber-Vorlagen
                  </div>
                </div>
                <button
                  onClick={() => {
                    setShowSettingsPanel(false);
                    setShowPresetModal(true);
                  }}
                  className="px-2.5 py-1 text-xs font-medium bg-indigo-600 hover:bg-indigo-500 text-white rounded transition-colors shrink-0 cursor-pointer"
                >
                  Öffnen
                </button>
              </div>

              {/* Default Category section */}
              <div>
                <label className="text-[10px] font-bold text-slate-500 uppercase tracking-widest block mb-1">Standard-Kategorie</label>
                <select 
                  value={defaultNewCategory} 
                  onChange={(e) => {
                    setDefaultNewCategory(e.target.value);
                    setSelectedCategory(e.target.value);
                  }}
                  className="bg-black/50 border border-white/10 text-slate-300 rounded px-2 py-1 outline-none text-xs w-full cursor-pointer hover:border-white/20"
                >
                  {categories.map(cat => (
                    <option key={cat} value={cat} className="bg-slate-900">{cat}</option>
                  ))}
                </select>
                <div className="text-[9px] text-slate-500 mt-1">Wird für neu erstellte Aufgaben verwendet.</div>
              </div>

              {/* Tag Manager section */}
              <div className="border-t border-white/5 pt-2">
                <button
                  onClick={() => {
                    setShowSettingsPanel(false);
                    setShowManageCat(true);
                  }}
                  className="w-full py-2 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 border border-indigo-500/20 rounded-md flex items-center justify-center gap-2 text-xs font-semibold transition-colors cursor-pointer"
                >
                  <Tag className="w-3.5 h-3.5" /> Kategorien verwalten
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Searching */}
        <div className="relative mb-1.5 shrink-0">
          <Search className="w-3.5 h-3.5 absolute left-2.5 top-1/2 -translate-y-1/2 text-slate-500" />
          <input 
            type="text" 
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Aufgaben durchsuchen..."
            className="w-full bg-black/40 border border-white/5 rounded pl-8 pr-3 py-1 text-xs text-slate-300 outline-none focus:border-emerald-500/30 transition-colors"
          />
        </div>

        {/* Dynamic Category search filter toggle row */}
        <div className="flex items-center gap-1.5 overflow-x-auto pb-2 mb-1.5 shrink-0 scrollbar-none select-none text-[9px] -mx-1 px-1">
          <button
            onClick={() => setFilterCategories([])}
            className={`px-2 py-0.5 rounded-full border transition-all truncate shrink-0 flex items-center gap-1 ${
              filterCategories.length === 0
                ? 'bg-indigo-500/20 border-indigo-500/45 text-indigo-300 font-bold'
                : 'bg-white/[0.02] border-white/5 text-slate-400 hover:bg-white/5'
            }`}
          >
            <span>All tags</span>
            {filterCategories.length > 0 && (
              <span className="bg-indigo-500/30 text-indigo-300 px-1 rounded-full text-[8px] font-semibold">
                {filterCategories.length} active
              </span>
            )}
          </button>
          {categories.map(cat => {
            const isSelected = filterCategories.includes(cat);
            const badgeStyle = getCategoryStyle(cat);
            return (
              <button
                key={cat}
                onClick={() => {
                  setFilterCategories(prev => 
                    prev.includes(cat) 
                      ? prev.filter(c => c !== cat) 
                      : [...prev, cat]
                  );
                }}
                style={isSelected ? badgeStyle : {}}
                className={`px-2 py-0.5 rounded-full border transition-all truncate shrink-0 flex items-center gap-1 ${
                  isSelected
                    ? 'border font-bold ring-1 ring-white/10'
                    : 'bg-white/[0.02] border-white/5 text-slate-400 hover:text-slate-200 hover:bg-white/5'
                }`}
              >
                {isSelected && <Check className="w-2.5 h-2.5 shrink-0" />}
                <span>{cat}</span>
              </button>
            );
          })}
          {filterCategories.length > 0 && (
            <button
              onClick={() => setFilterCategories([])}
              className="text-[9px] text-slate-400 hover:text-red-300 transition-colors ml-auto pl-1 shrink-0 flex items-center gap-0.5"
              title="Filter zurücksetzen"
            >
              <RotateCcw className="w-2.5 h-2.5" />
              <span>Clear</span>
            </button>
          )}
        </div>
        
        {/* Create Task Form */}
        <div className="flex flex-col gap-1.5 mb-3 bg-white/[0.01] border border-white/5 p-2 rounded-lg shrink-0">
          <div className="flex gap-2">
            <input 
              ref={inputRef}
              type="text" 
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && addTodo()}
              placeholder={isListening ? "Höre zu..." : "Neue Aufgabe fällig..."}
              className={`flex-1 bg-black/50 border rounded px-2.5 py-1 text-xs text-slate-200 outline-none transition-all ${isListening ? 'border-red-500/50 shadow-[0_0_8px_rgba(239,68,68,0.2)]' : 'border-white/10 focus:border-emerald-500/50'}`}
              disabled={isListening}
            />
            <button 
              onClick={startVoiceRecognition}
              className={`p-1 px-2.5 rounded transition-colors flex items-center justify-center ${isListening ? 'bg-red-500/20 text-red-400 animate-pulse' : 'bg-slate-700/30 text-slate-400 hover:bg-slate-700/50'}`} 
              title="Spracherkennung"
            >
              <Mic className="w-4 h-4" />
            </button>
            <button 
              onClick={() => setShowVoiceHelp(true)}
              className="p-1 px-2 text-slate-500 hover:text-slate-300 transition-colors" 
              title="Sprachbefehle Hilfe"
            >
              <HelpCircle className="w-4 h-4" />
            </button>
            <button onClick={addTodo} className="p-1 px-2.5 bg-emerald-500/20 text-emerald-400 hover:bg-emerald-500/30 rounded transition-colors" title="Aufgabe hinzufügen">
              <Plus className="w-4 h-4" />
            </button>
          </div>

          <div className="grid grid-cols-2 gap-x-2 gap-y-1.5 text-[10px] mt-1 pr-1">
            {/* Category selection */}
            <div className="flex items-center gap-1 min-w-0">
              <span className="text-slate-500 select-none shrink-0">Tag:</span>
              <select 
                value={selectedCategory} 
                onChange={(e) => setSelectedCategory(e.target.value)}
                className="bg-black/40 border border-white/10 text-slate-300 rounded px-1 py-0.5 outline-none text-[9px] w-full truncate cursor-pointer hover:border-white/20"
              >
                {categories.map(cat => (
                  <option key={cat} value={cat} className="bg-slate-900 text-slate-300">{cat}</option>
                ))}
              </select>
            </div>

            {/* Priority selection */}
            <div className="flex items-center gap-1 min-w-0">
              <span className="text-slate-500 select-none shrink-0">Prio:</span>
              <select 
                value={selectedPriority} 
                onChange={(e) => setSelectedPriority(e.target.value as any)}
                className="bg-black/40 border border-white/10 text-slate-300 rounded px-1 py-0.5 outline-none text-[9px] w-full truncate cursor-pointer hover:border-white/20"
              >
                <option value="Low" className="bg-slate-900 text-emerald-400">Low</option>
                <option value="Medium" className="bg-slate-900 text-amber-400">Medium</option>
                <option value="High" className="bg-slate-900 text-red-400">High</option>
              </select>
            </div>

            {/* Optional Due Date */}
            <div className="flex items-center gap-1 min-w-0 relative">
              <span className="text-slate-500 select-none shrink-0"><Calendar className="w-3.5 h-3.5 text-slate-500" /></span>
              <input
                type="date"
                value={dueDate}
                onChange={(e) => setDueDate(e.target.value)}
                className="bg-black/40 border border-white/10 text-slate-300 rounded px-1.5 py-0.5 outline-none text-[9px] w-full cursor-pointer hover:border-white/20"
                title="Optionales Fälligkeitsdatum"
              />
            </div>

            {/* Recurrence selection */}
            <div className="flex items-center gap-1 min-w-0">
              <span className="text-slate-500 select-none shrink-0"><CalendarClock className="w-3.5 h-3.5 text-slate-500" /></span>
              <select 
                value={recurring} 
                onChange={(e) => setRecurring(e.target.value as any)}
                className="bg-black/40 border border-white/10 text-slate-300 rounded px-1.5 py-0.5 outline-none text-[9px] w-full truncate cursor-pointer hover:border-white/20"
                title="Wiederholungsintervall"
              >
                <option value="none" className="bg-slate-900 text-slate-450">Einmalig</option>
                <option value="daily" className="bg-slate-900 text-slate-300">Täglich</option>
                <option value="weekly" className="bg-slate-900 text-slate-300">Wöchentlich</option>
                <option value="monthly" className="bg-slate-900 text-slate-300">Monatlich</option>
              </select>
            </div>
          </div>
        </div>

        {dueRecurringTasks.length > 0 && (
          <div className="bg-amber-500/20 border border-amber-500/30 rounded-lg p-2 mb-2 flex items-center gap-2 shrink-0">
            <CalendarClock className="w-4 h-4 text-amber-400 shrink-0" />
            <div className="text-[10px] text-amber-200">
              <span className="font-bold">{dueRecurringTasks.length} fällige wiederkehrende Aufgabe{dueRecurringTasks.length > 1 ? 'n' : ''}:</span>
              <span className="ml-1 opacity-80">{dueRecurringTasks.map(t => t.text).join(", ")}</span>
            </div>
          </div>
        )}

        {/* Filter Tabs */}
        <div className="flex bg-black/40 border border-white/10 rounded-lg p-0.5 mb-2 shrink-0">
          {(['all', 'active', 'completed'] as const).map((f) => (
            <button
              key={f}
              onClick={() => setFilter(f)}
              className={`flex-1 py-1 text-center text-[9px] uppercase font-bold rounded-md transition-all ${
                filter === f
                  ? 'bg-emerald-500/25 text-emerald-400 border border-emerald-500/20 shadow'
                  : 'text-slate-500 hover:text-slate-300 border border-transparent'
              }`}
            >
              {f === 'all' ? 'All' : f === 'active' ? 'Active' : 'Completed'}
            </button>
          ))}
        </div>

        {/* Task List */}
        <div className="flex-1 overflow-y-auto space-y-2 pr-1 min-h-0">
          {filteredTodos.length === 0 ? (
            <div className="text-center py-12 text-slate-600 text-xs">
              Keine Aufgaben gefunden.
            </div>
          ) : (
            <AnimatePresence initial={false}>
              {filteredTodos.map(todo => {
                const isSelected = selectedTodoId === todo.id;
                
                // Progress Bar Logic
                const subtasks = todo.subtasks || [];
                const totalSub = subtasks.length;
                const completedSub = subtasks.filter(st => st.done).length;
                const progressPct = totalSub > 0 ? Math.round((completedSub / totalSub) * 100) : (todo.done ? 100 : 0);
                
                // Overdue Indicator Logic
                const isOverdue = checkIfOverdue(todo.dueDate, todo.done);

                const isDragging = draggedId === todo.id;

                return (
                  <motion.div 
                    key={todo.id} 
                    onClick={() => setSelectedTodoId(todo.id)}
                    onContextMenu={(e) => {
                      e.preventDefault();
                      setContextMenu({
                        x: e.clientX,
                        y: e.clientY,
                        todoId: todo.id
                      });
                    }}
                    layout
                    draggable
                    onDragStart={(e: any) => handleDragStart(e, todo.id)}
                    onDragOver={(e: any) => handleDragOver(e, todo.id)}
                    onDragEnd={() => handleDragEnd()}
                    initial={{ opacity: 0, y: 12, scale: 0.96 }}
                    animate={{ opacity: isDragging ? 0.35 : 1, y: 0, scale: 1 }}
                    exit={{ opacity: 0, scale: 0.94, height: 0, y: -8, transition: { duration: 0.15 } }}
                    transition={{ type: "spring", stiffness: 380, damping: 30 }}
                    className={`flex flex-col gap-1 rounded-lg group cursor-default select-none relative ${
                      isSelected ? 'ring-2 ring-emerald-500/40 border border-emerald-500/20' : 'r-0'
                    } ${isDragging ? 'border border-dashed border-indigo-500/40 bg-indigo-950/10' : ''}`}
                  >
                    {/* Background swipe track */}
                    {todo.done && (
                      <div className="absolute inset-0 bg-red-500/20 flex justify-end items-center px-4 rounded-lg z-0 pointer-events-none overflow-hidden">
                        <Archive className="w-5 h-5 text-red-500/80 mr-2" />
                      </div>
                    )}
                    
                    <motion.div 
                      drag={todo.done ? "x" : false}
                      dragConstraints={{ left: 0, right: 0 }}
                      dragElastic={{ left: 0.6, right: 0 }}
                      onDragEnd={(e, info) => {
                        if (todo.done && info.offset.x < -80) {
                          archiveSingleTodo(todo.id);
                        }
                      }}
                      className={`flex items-start gap-2 p-2.5 rounded-lg border-y border-r border-l-4 transition-all relative overflow-hidden z-10 w-full ${
                        todo.done 
                          ? 'bg-slate-900/90 border-white/5 opacity-80 backdrop-blur-sm' 
                          : 'bg-white/10 border-y-white/10 border-r-white/10'
                      }`}
                      style={!todo.done ? { borderLeftColor: getCategoryColor(todo.category) } : {}}
                    >
                      {/* Progress bar fill effect */}
                      <motion.div
                        className="absolute top-0 left-0 bottom-0 pointer-events-none z-0 bg-gradient-to-r from-emerald-500/0 to-emerald-500/10"
                        initial={{ width: "0%" }}
                        animate={{ width: `${progressPct}%` }}
                        transition={{ duration: 0.5, ease: "easeOut" }}
                      />
                      <div className="absolute top-0 left-0 bottom-0 w-full pointer-events-none z-0 bg-gradient-to-r from-emerald-500/0 to-emerald-500/5 mix-blend-overlay" style={{ width: `${progressPct}%`, transition: 'width 0.5s ease-out' }} />
                      
                      <div className="flex w-full items-start gap-2 z-10 relative">
                        {/* Expand, Drag and Check triggers */}
                        <div className="flex items-center gap-1 mt-0.5 shrink-0 select-none">
                          <div 
                            className="text-slate-650 hover:text-slate-400 cursor-grab active:cursor-grabbing p-0.5 shrink-0"
                            title="Festhalten & ziehen zum Neuanordnen"
                          >
                            <GripVertical className="w-3.5 h-3.5 text-slate-500 hover:text-slate-300" />
                          </div>

                          <button 
                            onClick={(e) => {
                              e.stopPropagation();
                              toggleTaskExpanded(todo.id);
                            }} 
                            className="text-slate-400 hover:text-slate-200 transition-transform opacity-100"
                          >
                            {todo.expanded ? <ChevronDown className="w-3.5 h-3.5" /> : <ChevronRight className="w-3.5 h-3.5" />}
                          </button>
                          
                          <button 
                            onClick={(e) => {
                              e.stopPropagation();
                              toggleTodoDone(todo.id);
                            }}
                            className={`w-4 h-4 flex-shrink-0 rounded flex items-center justify-center border transition-colors ${
                              todo.done ? 'bg-emerald-500 border-emerald-500' : 'border-slate-500 hover:border-emerald-500/50'
                            }`}
                          >
                            {todo.done && <Check className="w-2.5 h-2.5 text-black font-extrabold" />}
                          </button>
                        </div>
                        
                        {/* Task details */}
                        <div className={`text-xs flex-1 flex flex-col ${todo.done ? 'line-through text-slate-400' : 'text-slate-200'}`}>
                        {editingId === todo.id ? (
                          <input
                            type="text"
                            value={editText}
                            onChange={(e) => setEditText(e.target.value)}
                            onBlur={handleEditSave}
                            onKeyDown={(e) => {
                              if (e.key === 'Enter') handleEditSave();
                              if (e.key === 'Escape') setEditingId(null);
                            }}
                            autoFocus
                            className="w-full bg-black/40 border border-emerald-500/50 rounded px-1.5 py-0.5 text-xs text-slate-200 outline-none"
                            onClick={(e) => e.stopPropagation()}
                          />
                        ) : (
                          <span 
                            onClick={(e) => {
                              e.stopPropagation();
                              handleEditStart(todo.id, todo.text);
                            }}
                            className="cursor-text hover:text-emerald-300 transition-colors py-0.5 font-medium leading-tight break-all"
                          >
                            {highlightText(todo.text, searchQuery)}
                          </span>
                        )}

                        {/* Overdue Alert or Due Date display */}
                        {todo.dueDate && (
                          <div className="flex items-center gap-1.5 mt-1 select-none">
                            <span className={`text-[9px] font-mono flex items-center gap-1.5 px-1.5 py-0.5 rounded border border-white/5 bg-slate-950 ${
                              isOverdue ? 'text-red-400 border-red-500/30 bg-red-950/20 animate-pulse' : 'text-slate-400'
                            }`}>
                              {isOverdue ? (
                                <CalendarClock className="w-3 h-3 text-red-500" />
                              ) : (
                                <Calendar className="w-3 h-3 text-slate-500" />
                              )}
                              Fällig: {formatDueDate(todo.dueDate)}
                              {isOverdue && <span className="font-bold text-[8px] tracking-wider uppercase ml-1">Overdue</span>}
                            </span>
                          </div>
                        )}

                        {/* Visual Progress Bar for Tasks with Subtasks */}
                        {totalSub > 0 && (
                          <div className="w-full mt-1.5">
                            <div className="flex justify-between items-center text-[8px] font-mono text-slate-500 mb-0.5">
                              <span>Subtask Fortschritt</span>
                              <span className="font-bold text-slate-400">{progressPct}%</span>
                            </div>
                            <div className="w-full bg-slate-950 rounded-full h-1 overflow-hidden border border-white/5">
                              <div 
                                className="bg-emerald-500 h-full transition-all duration-300 rounded-full" 
                                style={{ width: `${progressPct}%` }}
                              />
                            </div>
                          </div>
                        )}

                        {/* Labels and Priority Badges */}
                        <div className="flex flex-wrap gap-1.5 mt-1.5">
                          {todo.category && (
                            <span 
                              className="text-[8px] font-mono tracking-widest uppercase px-1.5 py-0.5 rounded border whitespace-nowrap"
                              style={getCategoryStyle(todo.category)}
                            >
                              {todo.category}
                            </span>
                          )}
                          {todo.priority && (
                            <span 
                              onClick={(e) => {
                                e.stopPropagation();
                                const rect = e.currentTarget.getBoundingClientRect();
                                setContextMenu({
                                  x: rect.left,
                                  y: rect.bottom + 4,
                                  todoId: todo.id
                                });
                              }}
                              className={`text-[8px] font-mono tracking-widest uppercase px-1.5 py-0.5 rounded border cursor-pointer hover:brightness-110 active:scale-95 transition-all select-none whitespace-nowrap ${getPriorityColor(todo.priority)}`}
                              title="Rechtsklick oder Klick für Priorität"
                            >
                              {todo.priority}
                            </span>
                          )}
                          {todo.recurring && todo.recurring !== 'none' && (
                            <span className="text-[8px] font-mono tracking-widest uppercase px-1.5 py-0.5 rounded border border-indigo-500/20 bg-indigo-950/30 text-indigo-300 flex items-center gap-1 select-none">
                              <CalendarClock className="w-2.5 h-2.5" />
                              {todo.recurring === 'daily' ? 'TÄGLICH' : todo.recurring === 'weekly' ? 'WÖCHENTLICH' : 'MONATLICH'}
                            </span>
                          )}
                          {totalSub > 0 && (
                            <span className="text-[8px] font-mono text-slate-500 flex items-center tracking-widest">
                              {completedSub}/{totalSub} SUB
                            </span>
                          )}
                        </div>
                      </div>

                      {/* Delete action */}
                      <button 
                        onClick={(e) => {
                          e.stopPropagation();
                          deleteTodo(todo.id);
                        }}
                        className="text-slate-500 hover:text-red-400 transition-colors shrink-0 self-start p-1 mt-0.5 opacity-0 group-hover:opacity-100 focus:opacity-100"
                        title="Aufgabe löschen"
                      >
                        <Trash2 className="w-3.5 h-3.5" />
                      </button>
                      </div>
                    </motion.div>

                    {/* Subtask listing with expandable structure */}
                    <AnimatePresence initial={false}>
                      {todo.expanded && (
                        <motion.div 
                          initial={{ height: 0, opacity: 0 }}
                          animate={{ height: "auto", opacity: 1, transition: { height: { duration: 0.2 }, opacity: { duration: 0.15 } } }}
                          exit={{ height: 0, opacity: 0, transition: { height: { duration: 0.2 }, opacity: { duration: 0.1 } } }}
                          style={{ overflow: "hidden" }}
                          className="pl-14 pr-2 space-y-1.5 mt-1 relative mb-2"
                        >
                          <div className="absolute left-[34px] top-0 bottom-4 w-px bg-white/10" />
                          
                          {/* Multi-line notes field */}
                          <div className="relative mb-2 shrink-0">
                            <div 
                              className="flex items-center gap-1.5 text-[9px] font-bold text-slate-500 hover:text-slate-400 uppercase tracking-widest mb-1 select-none cursor-pointer w-max"
                              onClick={(e) => {
                                e.stopPropagation();
                                setTodos(prev => prev.map(t => t.id === todo.id ? { ...t, expandedNotes: !t.expandedNotes } : t));
                              }}
                            >
                              <ChevronRight className={`w-3 h-3 text-slate-500 transition-transform ${todo.expandedNotes ? 'rotate-90' : ''}`} />
                              <FileText className="w-3 h-3 text-slate-500" /> Notizen / Details {todo.notes ? '(Vorhanden)' : ''}
                            </div>
                            
                            <AnimatePresence>
                              {todo.expandedNotes && (
                                <motion.div
                                  initial={{ height: 0, opacity: 0 }}
                                  animate={{ height: "auto", opacity: 1 }}
                                  exit={{ height: 0, opacity: 0 }}
                                  style={{ overflow: "hidden" }}
                                >
                                  <textarea
                                    value={todo.notes || ''}
                                    onChange={(e) => {
                                      setTodos(prev => prev.map(t => t.id === todo.id ? { ...t, notes: e.target.value } : t));
                                    }}
                                    placeholder="Füge hier Notizen zur Aufgabe hinzu..."
                                    rows={2}
                                    className="w-full bg-black/40 border border-white/5 rounded p-1.5 text-[10px] text-slate-300 outline-none focus:border-emerald-500/30 transition-colors resize-none placeholder-slate-600 font-sans leading-relaxed"
                                    onClick={(e) => e.stopPropagation()}
                                  />
                                </motion.div>
                              )}
                            </AnimatePresence>
                          </div>

                          {/* Subtasks header */}
                          <div className="text-[9px] font-bold text-slate-500 uppercase tracking-widest mt-1 select-none">
                            Unteraufgaben
                          </div>

                          {todo.subtasks?.map(subtask => (
                            <div key={subtask.id} className="flex items-center gap-2 text-xs group/sub">
                              <button 
                                 onClick={(e) => {
                                   e.stopPropagation();
                                   toggleSubtask(todo.id, subtask.id);
                                 }}
                                 className={`w-3.5 h-3.5 flex-shrink-0 rounded flex items-center justify-center border transition-colors ${
                                   subtask.done ? 'bg-emerald-500 border-emerald-500' : 'border-slate-600 hover:border-emerald-500/55'
                                 }`}
                              >
                                 {subtask.done && <Check className="w-2 h-2 text-black font-extrabold" />}
                              </button>
                              
                              <div className={`flex-1 break-all ${subtask.done ? 'line-through text-slate-500' : 'text-slate-300'}`}>
                                {editingSubtaskId === subtask.id ? (
                                  <input 
                                    type="text" autoFocus 
                                    value={editSubtext}
                                    onChange={(e) => setEditSubtext(e.target.value)}
                                    onBlur={() => handleSubtaskEditSave(todo.id)}
                                    onKeyDown={e => {
                                       if (e.key === 'Enter') handleSubtaskEditSave(todo.id);
                                       if (e.key === 'Escape') setEditingSubtaskId(null);
                                    }}
                                    className="bg-black/40 border border-emerald-500/50 rounded px-1.5 py-0.5 w-full outline-none text-xs"
                                    onClick={(e) => e.stopPropagation()}
                                  />
                                ) : (
                                  <span 
                                    className="cursor-text hover:text-emerald-300 transition-colors py-0.5 block" 
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      handleSubtaskEditStart(subtask.id, subtask.text);
                                    }}
                                  >
                                    {subtask.text}
                                  </span>
                                )}
                              </div>
                              
                              <button 
                                onClick={(e) => {
                                  e.stopPropagation();
                                  deleteSubtask(todo.id, subtask.id);
                                }} 
                                className="opacity-0 group-hover\sub:opacity-100 text-slate-600 hover:text-red-400 transition-opacity p-0.5 shrink-0"
                              >
                                 <Trash2 className="w-3" />
                              </button>
                            </div>
                          ))}

                          {subtaskInputId === todo.id ? (
                            <div className="flex items-center gap-2 text-xs">
                              <input 
                                type="text" autoFocus
                                value={subtaskInput}
                                onChange={e => setSubtaskInput(e.target.value)}
                                onBlur={() => { if (!subtaskInput.trim()) setSubtaskInputId(null); else addSubtask(todo.id); }}
                                onKeyDown={e => {
                                  if (e.key === 'Enter') addSubtask(todo.id);
                                  if (e.key === 'Escape') setSubtaskInputId(null);
                                }}
                                placeholder="Unteraufgabe hinzufügen..."
                                className="bg-black/40 border border-emerald-500/50 rounded px-1.5 py-0.5 text-slate-300 w-full outline-none text-xs"
                                onClick={(e) => e.stopPropagation()}
                              />
                            </div>
                          ) : (
                            <div className="flex items-center gap-2 justify-between w-full">
                              <button 
                                onClick={(e) => {
                                  e.stopPropagation();
                                  setSubtaskInputId(todo.id);
                                }} 
                                className="text-[9px] text-slate-500 hover:text-emerald-400 flex items-center gap-1 transition-colors uppercase tracking-widest font-bold"
                              >
                                <Plus className="w-3 h-3" /> Add Subtask
                              </button>
                              
                              <div className="flex items-center gap-1">
                                {subtaskSets.length > 0 && (
                                  <select
                                    onChange={(e) => {
                                      e.stopPropagation();
                                      if (e.target.value === '__open_modal__') {
                                        setShowPresetModal(true);
                                      } else {
                                        handleLoadSubtaskSet(todo.id, e.target.value);
                                      }
                                      e.target.value = "";
                                    }}
                                    onClick={(e) => e.stopPropagation()}
                                    className="bg-[#090d16] border border-white/10 text-slate-400 text-[9px] rounded px-1 outline-none w-[100px] cursor-pointer hover:border-indigo-500/30"
                                    defaultValue=""
                                  >
                                    <option value="" disabled>+ Vorlage</option>
                                    <optgroup label="⚡ Bare Metal & Kernel">
                                      {subtaskSets.filter(s => s.id.startsWith('k-')).map(s => (
                                        <option key={s.id} value={s.id}>{s.name}</option>
                                      ))}
                                    </optgroup>
                                    <optgroup label="📁 Eigene Vorlagen">
                                      {subtaskSets.filter(s => !s.id.startsWith('k-')).map(s => (
                                        <option key={s.id} value={s.id}>{s.name}</option>
                                      ))}
                                    </optgroup>
                                    <option value="__open_modal__">🔍 Alle Presets...</option>
                                  </select>
                                )}
                                {todo.subtasks && todo.subtasks.length > 0 && (
                                  <button
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      handleSaveSubtaskSet(todo.id);
                                    }}
                                    className="text-slate-500 hover:text-indigo-400 p-0.5 transition-colors"
                                    title="Als Vorlage speichern"
                                  >
                                    <Bookmark className="w-3.5 h-3.5" />
                                  </button>
                                )}
                              </div>
                            </div>
                          )}
                        </motion.div>
                      )}
                    </AnimatePresence>
                  </motion.div>
                );
              })}
            </AnimatePresence>
          )}
        </div>
        
        {/* Footer with Tabs and Charts */}
        <div className="mt-2 pt-2 border-t border-white/10 flex flex-col gap-2 shrink-0 select-none bg-black/20 p-2 rounded-lg">
          {(() => {
            const completedCount = todos.filter(t => t.done).length;
            const activeCount = todos.filter(t => !t.done).length;
            const totalCount = todos.length;
            const progressPercent = totalCount > 0 ? Math.round((completedCount / totalCount) * 100) : 0;
            const progressData = totalCount > 0 ? [
              { name: 'Erledigt', value: completedCount, color: '#10b981' },
              { name: 'Aktiv', value: activeCount, color: '#6366f1' }
            ] : [
              { name: 'Keine Aufgaben', value: 1, color: '#1e293b' }
            ];

            return (
              <>
                <div className="flex justify-between items-center border-b border-white/5 pb-1 select-none">
                  <div className="flex items-center gap-1.5">
                    {/* Mini circular progress indicator in header */}
                    <div className="w-4 h-4 relative flex items-center justify-center shrink-0">
                      <PieChart width={16} height={16}>
                        <Pie
                          data={progressData}
                          cx={8}
                          cy={8}
                          innerRadius={4}
                          outerRadius={7}
                          dataKey="value"
                          stroke="none"
                          startAngle={90}
                          endAngle={-270}
                        >
                          {progressData.map((entry, idx) => (
                            <Cell key={`header-progress-${idx}`} fill={entry.color} />
                          ))}
                        </Pie>
                      </PieChart>
                    </div>
                    <span className="text-[9px] font-bold text-slate-400 uppercase tracking-wider">
                      Fortschritt & Visualisierung
                    </span>
                  </div>
                  <div className="flex gap-1 bg-black/40 p-0.5 rounded border border-white/5 text-[8px] font-bold">
                    <button 
                      onClick={() => setFooterTab('status')}
                      className={`px-1.5 py-0.5 rounded transition-all cursor-pointer ${footerTab === 'status' ? 'bg-emerald-500/20 text-emerald-400 font-extrabold' : 'text-slate-500 hover:text-slate-300'}`}
                    >
                      Status
                    </button>
                    <button 
                      onClick={() => setFooterTab('trends')}
                      className={`px-1.5 py-0.5 rounded transition-all cursor-pointer ${footerTab === 'trends' ? 'bg-emerald-500/20 text-emerald-400 font-extrabold' : 'text-slate-500 hover:text-slate-300'}`}
                    >
                      7-Tage-Trend
                    </button>
                    <button 
                      onClick={() => setFooterTab('priority')}
                      className={`px-1.5 py-0.5 rounded transition-all cursor-pointer ${footerTab === 'priority' ? 'bg-emerald-500/20 text-emerald-400 font-extrabold' : 'text-slate-500 hover:text-slate-300'}`}
                    >
                      Priorität
                    </button>
                    <button 
                      onClick={() => setFooterTab('heatmap')}
                      className={`px-1.5 py-0.5 rounded transition-all cursor-pointer ${footerTab === 'heatmap' ? 'bg-emerald-500/20 text-emerald-400 font-extrabold' : 'text-slate-500 hover:text-slate-300'}`}
                    >
                      30-Tage-Heatmap
                    </button>
                  </div>
                </div>

                {footerTab === 'status' ? (
                  <div className="flex items-center justify-between gap-2 px-0.5 py-0.5">
                    {/* Left: Task counters breakdown */}
                    <div className="flex flex-col gap-0.5 text-[9px] font-mono min-w-0">
                      <div className="flex items-center gap-1.5">
                        <span className="w-1.5 h-1.5 rounded-full bg-emerald-500 shrink-0 animate-pulse" />
                        <span className="text-slate-300 font-semibold">{completedCount} Erledigt</span>
                      </div>
                      <div className="flex items-center gap-1.5">
                        <span className="w-1.5 h-1.5 rounded-full bg-indigo-500 shrink-0" />
                        <span className="text-slate-400">{activeCount} Aktiv</span>
                      </div>
                      <div className="text-[8px] text-slate-500 mt-0.5 font-bold uppercase tracking-wider">
                        {totalCount > 0 ? `${completedCount} von ${totalCount} Aufgaben (${progressPercent}%)` : 'Keine Aufgaben vorhanden'}
                      </div>
                    </div>

                    {/* Right: Circular Progress Indicator using Recharts PieChart */}
                    <div className="flex items-center gap-3 shrink-0">
                      {/* Export / Import backup tools */}
                      <div className="flex flex-col items-center gap-0.5 text-[9px] text-slate-500">
                        <div className="flex items-center gap-1 border border-white/5 rounded-md p-0.5 bg-slate-950">
                          <button 
                            onClick={exportTodosJSON}
                            className="p-1 hover:text-emerald-400 text-slate-400 transition-colors"
                            title="Backup exportieren (.json)"
                          >
                            <Download className="w-3.5 h-3.5" />
                          </button>
                          <div className="w-px h-3 bg-white/10" />
                          <button 
                            onClick={() => fileInputRef.current?.click()}
                            className="p-1 hover:text-indigo-400 text-slate-400 transition-colors"
                            title="Backup importieren (.json)"
                          >
                            <Upload className="w-3.5 h-3.5" />
                          </button>
                        </div>
                        <span className="font-mono text-center text-[7px] text-slate-600 block leading-none">Backup</span>
                      </div>

                      {/* Recharts Circular Progress Indicator Donut */}
                      <div className="w-[52px] h-[48px] relative flex items-center justify-center shrink-0" title={`Fortschritt: ${progressPercent}% (${completedCount}/${totalCount})`}>
                        <PieChart width={52} height={48}>
                          <Pie
                            data={progressData}
                            cx={26}
                            cy={24}
                            innerRadius={15}
                            outerRadius={22}
                            paddingAngle={totalCount > 0 && completedCount > 0 && activeCount > 0 ? 3 : 0}
                            dataKey="value"
                            stroke="none"
                            startAngle={90}
                            endAngle={-270}
                          >
                            {progressData.map((entry, idx) => (
                              <Cell key={`progress-cell-${idx}`} fill={entry.color} />
                            ))}
                          </Pie>
                          {totalCount > 0 && (
                            <Tooltip
                              contentStyle={{ 
                                backgroundColor: '#090d16', 
                                borderColor: '#334155', 
                                borderRadius: '4px', 
                                padding: '2px 5px',
                                fontSize: '8px'
                              }}
                              itemStyle={{ fontSize: '8px', padding: 0 }}
                              formatter={(val: any, name: string) => [
                                `${val} (${Math.round((Number(val) / totalCount) * 100)}%)`,
                                name
                              ]}
                            />
                          )}
                        </PieChart>
                        <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
                          <span className="text-[8px] font-black text-slate-200 font-mono leading-none">
                            {progressPercent}%
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>
                ) : footerTab === 'trends' ? (
                  <div className="flex flex-col items-center w-full h-[40px] select-none">
                    <LineChart 
                      width={330} 
                      height={40} 
                      data={(() => {
                        const data = [];
                        const d = new Date();
                        for (let i = 6; i >= 0; i--) {
                          const pastDate = new Date();
                          pastDate.setDate(d.getDate() - i);
                          const year = pastDate.getFullYear();
                          const month = String(pastDate.getMonth() + 1).padStart(2, '0');
                          const day = String(pastDate.getDate()).padStart(2, '0');
                          const dateStr = `${year}-${month}-${day}`;
                          const weekdayLabel = pastDate.toLocaleDateString('de-DE', { weekday: 'short' });
                          
                          const count = todos.filter(t => t.done && t.completedAt === dateStr).length +
                                        archivedTodos.filter(t => t.done && t.completedAt === dateStr).length;
                          
                          data.push({
                            name: weekdayLabel,
                            Completions: count
                          });
                        }
                        return data;
                      })()} 
                      margin={{ top: 4, right: 8, left: -24, bottom: 0 }}
                    >
                      <XAxis dataKey="name" stroke="#475569" fontSize={7} tickLine={false} axisLine={false} />
                      <YAxis stroke="#475569" fontSize={7} tickLine={false} axisLine={false} allowDecimals={false} />
                      <Tooltip 
                        contentStyle={{ backgroundColor: '#090d16', borderColor: '#334155', borderRadius: '4px', padding: '2px 4px' }}
                        labelStyle={{ fontSize: '7px', color: '#94a3b8', fontWeight: 'bold' }}
                        itemStyle={{ fontSize: '7px', color: '#10b981', padding: 0 }}
                      />
                      <Line type="monotone" dataKey="Completions" stroke="#10b981" strokeWidth={1.5} dot={{ r: 1.5, fill: '#10b981' }} activeDot={{ r: 3 }} />
                    </LineChart>
                  </div>
                ) : footerTab === 'heatmap' ? (
                  <div className="flex flex-col items-center w-full h-[40px] select-none">
                    <BarChart 
                      width={330} 
                      height={40} 
                      data={(() => {
                        const data = [];
                        const d = new Date();
                        for (let i = 29; i >= 0; i--) {
                          const pastDate = new Date();
                          pastDate.setDate(d.getDate() - i);
                          const year = pastDate.getFullYear();
                          const month = String(pastDate.getMonth() + 1).padStart(2, '0');
                          const day = String(pastDate.getDate()).padStart(2, '0');
                          const dateStr = `${year}-${month}-${day}`;
                          
                          const count = todos.filter(t => t.done && t.completedAt === dateStr).length +
                                        archivedTodos.filter(t => t.done && t.completedAt === dateStr).length;
                          
                          data.push({
                            name: `${day}.${month}.`,
                            count
                          });
                        }
                        return data;
                      })()} 
                      margin={{ top: 2, right: 8, left: 2, bottom: 2 }}
                      barCategoryGap="15%"
                    >
                      <XAxis dataKey="name" hide />
                      <YAxis hide />
                      <Tooltip
                        contentStyle={{ backgroundColor: '#090d16', borderColor: '#334155', borderRadius: '4px', padding: '2px 4px' }}
                        labelStyle={{ fontSize: '7px', color: '#94a3b8', fontWeight: 'bold' }}
                        itemStyle={{ fontSize: '7px', color: '#10b981', padding: 0 }}
                        cursor={{ fill: 'rgba(255, 255, 255, 0.05)' }}
                      />
                      <Bar dataKey="count" radius={[1, 1, 1, 1]}>
                        {(() => {
                          const data = [];
                          const d = new Date();
                          for (let i = 29; i >= 0; i--) {
                            const pastDate = new Date();
                            pastDate.setDate(d.getDate() - i);
                            const year = pastDate.getFullYear();
                            const month = String(pastDate.getMonth() + 1).padStart(2, '0');
                            const day = String(pastDate.getDate()).padStart(2, '0');
                            const dateStr = `${year}-${month}-${day}`;
                            
                            const count = todos.filter(t => t.done && t.completedAt === dateStr).length +
                                          archivedTodos.filter(t => t.done && t.completedAt === dateStr).length;
                            
                            let fill = '#1e293b'; // Base empty
                            if (count > 0) {
                              if (count === 1) fill = '#064e3b';
                              else if (count === 2) fill = '#059669';
                              else if (count === 3) fill = '#10b981';
                              else fill = '#34d399';
                            }
                            
                            data.push({ count, fill });
                          }
                          return data.map((entry, index) => (
                            <Cell key={`cell-${index}`} fill={entry.fill} />
                          ));
                        })()}
                      </Bar>
                    </BarChart>
                  </div>
                ) : (
                  <div className="flex flex-col items-center w-full h-[40px] select-none">
                    <BarChart 
                      width={330} 
                      height={40} 
                      data={[
                        { name: 'Low', count: todos.filter(t => !t.done && t.priority === 'Low').length, fill: '#64748b' },
                        { name: 'Med', count: todos.filter(t => !t.done && t.priority === 'Medium').length, fill: '#f59e0b' },
                        { name: 'High', count: todos.filter(t => !t.done && t.priority === 'High').length, fill: '#ef4444' }
                      ]}
                      margin={{ top: 4, right: 8, left: -24, bottom: 0 }}
                    >
                      <XAxis dataKey="name" stroke="#475569" fontSize={7} tickLine={false} axisLine={false} />
                      <YAxis stroke="#475569" fontSize={7} tickLine={false} axisLine={false} allowDecimals={false} />
                      <Tooltip
                        contentStyle={{ backgroundColor: '#090d16', borderColor: '#334155', borderRadius: '4px', padding: '2px 4px' }}
                        labelStyle={{ fontSize: '7px', color: '#94a3b8', fontWeight: 'bold' }}
                        itemStyle={{ fontSize: '7px', padding: 0 }}
                        cursor={{ fill: 'rgba(255, 255, 255, 0.05)' }}
                      />
                      <Bar dataKey="count" radius={[2, 2, 0, 0]}>
                        {
                          [
                            { name: 'Low', count: todos.filter(t => !t.done && t.priority === 'Low').length, fill: '#64748b' },
                            { name: 'Med', count: todos.filter(t => !t.done && t.priority === 'Medium').length, fill: '#f59e0b' },
                            { name: 'High', count: todos.filter(t => !t.done && t.priority === 'High').length, fill: '#ef4444' }
                          ].map((entry, index) => (
                            <Cell key={`cell-${index}`} fill={entry.fill} />
                          ))
                        }
                      </Bar>
                    </BarChart>
                  </div>
                )}
              </>
            );
          })()}
        </div>
      </div>

      {/* Hidden Backup File Import */}
      <input 
        type="file" 
        ref={fileInputRef} 
        onChange={importTodosJSON} 
        accept=".json" 
        className="hidden" 
      />

      {/* HTML5 Canvas Animating Confetti Particles */}
      <canvas 
        ref={canvasRef} 
        className="absolute inset-0 pointer-events-none z-[80] w-full h-full" 
      />

      {/* custom confirmation modal */}
      <AnimatePresence>
        {deleteConfirm && (
          <div className="absolute inset-0 bg-slate-950/80 backdrop-blur-xs z-[100] flex items-center justify-center p-4">
            <motion.div 
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="bg-slate-900 border border-white/10 rounded-xl p-4 w-[280px] shadow-2xl text-center flex flex-col gap-3"
            >
              <div className="mx-auto w-10 h-10 rounded-full bg-red-500/10 flex items-center justify-center border border-red-500/30 text-red-400 shadow-inner">
                <Trash2 className="w-5 h-5 animate-bounce" />
              </div>
              <div>
                <h4 className="text-xs font-bold text-slate-200">Eintrag löschen?</h4>
                <p className="text-[10px] text-slate-400 mt-1 leading-relaxed break-words px-1">
                  Möchten Sie "<strong>{deleteConfirm.itemName}</strong>" wirklich unwiderruflich löschen?
                </p>
              </div>
              
              <div className="flex gap-2 mt-1">
                <button
                  onClick={() => setDeleteConfirm(null)}
                  className="flex-1 py-1.5 rounded bg-slate-800 hover:bg-slate-750 text-[10.5px] text-slate-300 font-semibold transition-colors border border-white/5 cursor-pointer"
                >
                  Abbrechen
                </button>
                <button
                  onClick={() => {
                    if (deleteConfirm.type === 'todo') {
                      executeDeleteTodo(deleteConfirm.todoId);
                    } else if (deleteConfirm.type === 'subtask' && deleteConfirm.subtaskId) {
                      executeDeleteSubtask(deleteConfirm.todoId, deleteConfirm.subtaskId);
                    }
                  }}
                  className="flex-1 py-1.5 rounded bg-red-500/20 border border-red-500/30 hover:bg-red-500/30 text-[10.5px] text-red-200 font-semibold transition-colors cursor-pointer"
                >
                  Löschen
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>

      {/* custom notification dialog */}
      <AnimatePresence>
        {alertMessage && (
          <div className="absolute inset-0 bg-slate-950/80 backdrop-blur-xs z-[110] flex items-center justify-center p-4">
            <motion.div 
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="bg-slate-900 border border-white/10 rounded-xl p-4 w-[280px] shadow-2xl text-center flex flex-col gap-3"
            >
              <div className="mx-auto w-10 h-10 rounded-full bg-emerald-500/10 flex items-center justify-center border border-emerald-500/30 text-emerald-400">
                <Info className="w-5 h-5" />
              </div>
              <div>
                <h4 className="text-xs font-bold text-slate-200">Benachrichtigung</h4>
                <p className="text-[10px] text-slate-400 mt-1 leading-relaxed whitespace-pre-line px-1">
                  {alertMessage}
                </p>
              </div>
              <button
                onClick={() => setAlertMessage(null)}
                className="w-full py-1.5 rounded bg-slate-800 hover:bg-slate-750 text-[10.5px] text-slate-200 font-semibold transition-colors border border-white/5 mt-1 cursor-pointer"
              >
                OK
              </button>
            </motion.div>
          </div>
        )}
      </AnimatePresence>

      {/* Dynamic context menu for setting task priority */}
      <AnimatePresence>
        {contextMenu && (
          <div className="fixed inset-0 z-[140] pointer-events-auto" onClick={() => setContextMenu(null)}>
            <motion.div 
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0, scale: 0.95 }}
              transition={{ duration: 0.12 }}
              className="absolute z-[150] bg-slate-950/95 border border-white/10 rounded-lg p-1.5 shadow-2 shadow-black w-36 flex flex-col font-sans select-none"
              style={{ 
                top: Math.min(contextMenu.y, window.innerHeight - 110), 
                left: Math.min(contextMenu.x, window.innerWidth - 150) 
              }}
              onClick={(e) => e.stopPropagation()}
            >
              <div className="text-[8px] font-bold text-slate-500 uppercase tracking-widest px-2 py-1 select-none border-b border-white/5 mb-1">
                Priorität ändern
              </div>
              {(['Low', 'Medium', 'High'] as const).map(prio => (
                <button
                  key={prio}
                  onClick={() => {
                    setTodos(prev => prev.map(t => t.id === contextMenu.todoId ? { ...t, priority: prio } : t));
                    setContextMenu(null);
                  }}
                  className="flex items-center gap-1.5 px-2 py-1 text-xs text-left rounded-md hover:bg-white/5 text-slate-300 hover:text-white transition-colors cursor-pointer w-full text-left"
                >
                  <span className={`w-1.5 h-1.5 rounded-full ${
                    prio === 'High' ? 'bg-red-500' :
                    prio === 'Medium' ? 'bg-amber-500' :
                    'bg-emerald-500'
                  }`} />
                  {prio}
                </button>
              ))}
            </motion.div>
          </div>
        )}
      </AnimatePresence>

      {/* Voice Help Modal */}
      <AnimatePresence>
        {showVoiceHelp && (
          <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 p-4">
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="bg-slate-900 border border-white/10 rounded-xl p-5 w-full max-w-sm shadow-2xl space-y-4"
              onClick={(e) => e.stopPropagation()}
            >
              <div className="flex items-center gap-2 text-indigo-400 font-bold mb-2 border-b border-white/10 pb-2">
                <Mic className="w-5 h-5" />
                <span>Sprachbefehle</span>
              </div>
              <div className="text-sm text-slate-300 mb-4 leading-relaxed space-y-2">
                <p>Verwenden Sie das Mikrofon-Symbol, um Aufgaben zu sprechen.</p>
                <div className="bg-black/30 p-2 rounded border border-white/5 font-mono text-[11px] space-y-1">
                  <div className="text-emerald-400">"Füge Aufgabe hinzu [Text]"</div>
                  <div className="text-slate-500 text-[9px] mb-2">- Erstellt die Aufgabe direkt.</div>
                  <div className="text-emerald-400">"Add task [Text]"</div>
                  <div className="text-slate-500 text-[9px] mb-2">- Alternatives englisches Kommando.</div>
                  <div className="text-indigo-400">"[Irgendein Text]"</div>
                  <div className="text-slate-500 text-[9px]">- Füllt das Eingabefeld, ohne direkt zu speichern.</div>
                </div>
              </div>
              <div className="flex justify-end gap-2 pt-2 border-t border-white/5">
                <button
                  onClick={() => setShowVoiceHelp(false)}
                  className="px-4 py-1.5 rounded-lg bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500/30 text-xs font-bold transition-colors"
                >
                  Verstanden
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
      {/* Category Manager Modal */}
      <AnimatePresence>
        {showManageCat && (
          <div className="fixed inset-0 z-[120] flex items-center justify-center bg-black/60 p-4">
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="bg-slate-900 border border-indigo-500/30 rounded-xl p-5 w-full max-w-sm shadow-2xl flex flex-col max-h-[80vh]"
              onClick={(e) => e.stopPropagation()}
            >
              <div className="flex items-center justify-between font-bold mb-4 border-b border-white/10 pb-3">
                <div className="flex items-center gap-2 text-indigo-400">
                  <Tag className="w-5 h-5" />
                  <span>Kategorien Verwalten</span>
                </div>
                <button onClick={() => setShowManageCat(false)} className="text-slate-500 hover:text-slate-300 transition-colors p-1">
                  <X className="w-4 h-4" />
                </button>
              </div>

              {/* Create Category with Color picker option */}
              <div className="flex gap-2 mb-4 items-center">
                <input
                  type="text"
                  value={newCatName}
                  onChange={(e) => setNewCatName(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleAddCategory()}
                  placeholder="Neue Kategorie..."
                  className="flex-1 bg-black/50 border border-white/10 rounded-md px-3 py-2 text-sm text-slate-200 outline-none focus:border-indigo-500/50 transition-colors"
                />
                <div className="relative group">
                  <input 
                    type="color"
                    value={newCatColor}
                    onChange={(e) => setNewCatColor(e.target.value)}
                    className="w-10 h-10 rounded-md cursor-pointer border border-white/10 bg-black/50 p-1 outline-none shrink-0"
                    title="Wähle eine benutzerdefinierte Farbe"
                  />
                  <div className="absolute inset-0 ring-2 ring-transparent group-hover:ring-indigo-500/50 rounded-md pointer-events-none transition-all"></div>
                </div>
                <button 
                  onClick={handleAddCategory}
                  className="px-4 py-2 bg-indigo-500/20 text-indigo-400 hover:bg-indigo-500/30 rounded-md text-sm font-semibold transition-all shrink-0 h-10"
                >
                  <Plus className="w-4 h-4" />
                </button>
              </div>

              {/* List and Delete with Color pickers */}
              <div className="space-y-2 flex-1 overflow-y-auto px-1 modal-scroll">
                {categories.map(cat => {
                  const badgeStyle = getCategoryStyle(cat);
                  return (
                    <div key={cat} className="flex justify-between items-center bg-slate-950/50 hover:bg-slate-800/50 border border-white/5 px-3 py-2.5 rounded-lg transition-colors group">
                      <div className="flex items-center gap-3 min-w-0 flex-1">
                        <div className="relative">
                          <input 
                            type="color"
                            value={getCategoryColor(cat)}
                            onChange={(e) => {
                              const newColor = e.target.value;
                              setCategoryColors(prev => ({ ...prev, [cat]: newColor }));
                            }}
                            className="w-6 h-6 rounded-full border-2 border-slate-700 bg-transparent p-0 cursor-pointer outline-none shrink-0 transition-colors hover:border-indigo-400 focus:border-indigo-400"
                            title="Farbe ändern"
                          />
                        </div>
                        <span 
                          className="text-xs font-mono tracking-wider uppercase px-2 py-1 rounded border truncate"
                          style={badgeStyle}
                        >
                          {cat}
                        </span>
                      </div>
                      {categories.length > 1 && (
                        <button 
                          onClick={() => handleDeleteCategory(cat)}
                          className="text-slate-500 hover:text-red-400 transition-colors p-1.5 shrink-0 opacity-0 group-hover:opacity-100 bg-red-400/10 rounded"
                          title="Löschen"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      )}
                    </div>
                  );
                })}
              </div>
              <div className="mt-4 pt-3 border-t border-white/10 flex justify-end">
                <button
                  onClick={() => setShowManageCat(false)}
                  className="px-5 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-sm text-slate-200 font-semibold transition-colors"
                >
                  Fertig
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>

      {/* Productivity Insights Modal */}
      <AnimatePresence>
        {showInsightsPanel && (
          <div className="fixed inset-0 z-[120] flex items-center justify-center bg-black/70 backdrop-blur-xs p-4">
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.95, opacity: 0 }}
              className="bg-slate-900 border border-indigo-500/30 rounded-2xl p-5 w-full max-w-2xl shadow-2xl flex flex-col max-h-[90vh] overflow-hidden"
              onClick={(e) => e.stopPropagation()}
            >
              {/* Header */}
              <div className="flex items-center justify-between font-bold mb-3 border-b border-white/10 pb-3 shrink-0">
                <div className="flex items-center gap-2 text-indigo-400">
                  <div className="p-1.5 rounded-lg bg-indigo-500/10 border border-indigo-500/20">
                    <Activity className="w-5 h-5" />
                  </div>
                  <div>
                    <h3 className="text-base text-slate-100 font-semibold tracking-wide">Productivity Insights</h3>
                    <p className="text-[11px] text-slate-400 font-normal">Fortschritts- und Aufgabenanalyse nach Kategorien</p>
                  </div>
                </div>
                <button 
                  onClick={() => setShowInsightsPanel(false)} 
                  className="text-slate-400 hover:text-white p-1.5 rounded-lg hover:bg-white/5 transition-colors"
                  title="Schließen"
                >
                  <X className="w-4 h-4" />
                </button>
              </div>

              {/* Scrollable Body */}
              <div className="flex-1 overflow-y-auto pr-1 space-y-4 modal-scroll">
                {(() => {
                  const now = new Date();
                  const allDone = [
                    ...todos.filter(t => t.done),
                    ...archivedTodos.filter(t => t.done)
                  ];

                  // Daily data for chart
                  const chartData: any[] = [];
                  const catTotals: Record<string, number> = {};
                  categories.forEach(c => { catTotals[c] = 0; });

                  let completedInWindow = 0;

                  for (let i = insightsTimeframe - 1; i >= 0; i--) {
                    const pastDate = new Date();
                    pastDate.setDate(now.getDate() - i);
                    const year = pastDate.getFullYear();
                    const month = String(pastDate.getMonth() + 1).padStart(2, '0');
                    const day = String(pastDate.getDate()).padStart(2, '0');
                    const dateStr = `${year}-${month}-${day}`;

                    const dayItem: any = { 
                      name: `${day}.${month}`,
                      dateStr,
                      total: 0
                    };

                    categories.forEach(cat => {
                      const count = allDone.filter(t => t.category === cat && t.completedAt === dateStr).length;
                      dayItem[cat] = count;
                      dayItem.total += count;
                      catTotals[cat] = (catTotals[cat] || 0) + count;
                      completedInWindow += count;
                    });

                    chartData.push(dayItem);
                  }

                  // Top Category in timeframe
                  let topCat = 'Keine';
                  let topCatCount = 0;
                  Object.entries(catTotals).forEach(([cat, count]) => {
                    if (count > topCatCount) {
                      topCat = cat;
                      topCatCount = count;
                    }
                  });

                  // Streak calculation
                  let streak = 0;
                  for (let i = 0; i < 60; i++) {
                    const checkDate = new Date();
                    checkDate.setDate(now.getDate() - i);
                    const year = checkDate.getFullYear();
                    const month = String(checkDate.getMonth() + 1).padStart(2, '0');
                    const day = String(checkDate.getDate()).padStart(2, '0');
                    const dateStr = `${year}-${month}-${day}`;
                    const hasDone = allDone.some(t => t.completedAt === dateStr);
                    if (hasDone) {
                      streak++;
                    } else if (i > 0) {
                      break;
                    }
                  }

                  // Pie data
                  const pieData = categories
                    .map(cat => ({
                      name: cat,
                      value: catTotals[cat] || 0,
                      color: getCategoryColor(cat)
                    }))
                    .filter(item => item.value > 0);

                  const avgPerDay = (completedInWindow / insightsTimeframe).toFixed(1);
                  const totalPending = todos.filter(t => !t.done).length;
                  const totalAll = todos.length + archivedTodos.length;
                  const completionRate = totalAll > 0 ? Math.round((allDone.length / totalAll) * 100) : 0;

                  return (
                    <>
                      {/* Metric KPI cards */}
                      <div className="grid grid-cols-2 sm:grid-cols-4 gap-2.5">
                        <div className="bg-slate-950/60 border border-white/5 rounded-xl p-3 flex flex-col justify-between">
                          <div className="flex items-center justify-between text-slate-400 text-xs mb-1">
                            <span>Erledigt ({insightsTimeframe}T)</span>
                            <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
                          </div>
                          <div className="text-xl font-bold text-slate-100">{completedInWindow}</div>
                          <div className="text-[10px] text-slate-500 mt-0.5">Ø {avgPerDay} / Tag</div>
                        </div>

                        <div className="bg-slate-950/60 border border-white/5 rounded-xl p-3 flex flex-col justify-between">
                          <div className="flex items-center justify-between text-slate-400 text-xs mb-1">
                            <span>Gesamtrate</span>
                            <TrendingUp className="w-3.5 h-3.5 text-indigo-400" />
                          </div>
                          <div className="text-xl font-bold text-slate-100">{completionRate}%</div>
                          <div className="text-[10px] text-slate-500 mt-0.5">{totalPending} offen</div>
                        </div>

                        <div className="bg-slate-950/60 border border-white/5 rounded-xl p-3 flex flex-col justify-between">
                          <div className="flex items-center justify-between text-slate-400 text-xs mb-1">
                            <span>Top Kategorie</span>
                            <Sparkles className="w-3.5 h-3.5 text-amber-400" />
                          </div>
                          <div className="text-base font-bold text-slate-100 truncate">
                            {topCatCount > 0 ? topCat : '—'}
                          </div>
                          <div className="text-[10px] text-slate-500 mt-0.5">
                            {topCatCount > 0 ? `${topCatCount} Aufgaben` : 'Noch keine'}
                          </div>
                        </div>

                        <div className="bg-slate-950/60 border border-white/5 rounded-xl p-3 flex flex-col justify-between">
                          <div className="flex items-center justify-between text-slate-400 text-xs mb-1">
                            <span>Aktivitäts-Streak</span>
                            <Flame className="w-3.5 h-3.5 text-orange-400" />
                          </div>
                          <div className="text-xl font-bold text-slate-100">{streak} {streak === 1 ? 'Tag' : 'Tage'}</div>
                          <div className="text-[10px] text-slate-500 mt-0.5">{streak > 0 ? 'Im Flow 🔥' : 'Neu starten'}</div>
                        </div>
                      </div>

                      {/* Controls Row: Chart Type & Timeframe */}
                      <div className="flex flex-wrap items-center justify-between gap-2 pt-1">
                        {/* Chart switcher */}
                        <div className="flex items-center bg-slate-950/80 p-0.5 rounded-lg border border-white/5 text-xs">
                          <button
                            onClick={() => setInsightsTab('stacked')}
                            className={`flex items-center gap-1 px-2.5 py-1 rounded-md transition-all ${
                              insightsTab === 'stacked' 
                                ? 'bg-indigo-500/20 text-indigo-300 font-semibold shadow-xs' 
                                : 'text-slate-400 hover:text-slate-200'
                            }`}
                          >
                            <BarChart2 className="w-3.5 h-3.5" />
                            <span>Kategorien</span>
                          </button>
                          <button
                            onClick={() => setInsightsTab('trend')}
                            className={`flex items-center gap-1 px-2.5 py-1 rounded-md transition-all ${
                              insightsTab === 'trend' 
                                ? 'bg-indigo-500/20 text-indigo-300 font-semibold shadow-xs' 
                                : 'text-slate-400 hover:text-slate-200'
                            }`}
                          >
                            <TrendingUp className="w-3.5 h-3.5" />
                            <span>Trend</span>
                          </button>
                          <button
                            onClick={() => setInsightsTab('donut')}
                            className={`flex items-center gap-1 px-2.5 py-1 rounded-md transition-all ${
                              insightsTab === 'donut' 
                                ? 'bg-indigo-500/20 text-indigo-300 font-semibold shadow-xs' 
                                : 'text-slate-400 hover:text-slate-200'
                            }`}
                          >
                            <PieIcon className="w-3.5 h-3.5" />
                            <span>Verteilung</span>
                          </button>
                        </div>

                        {/* Timeframe selector */}
                        <div className="flex items-center bg-slate-950/80 p-0.5 rounded-lg border border-white/5 text-xs">
                          {([7, 14, 30] as const).map(tf => (
                            <button
                              key={tf}
                              onClick={() => setInsightsTimeframe(tf)}
                              className={`px-2.5 py-1 rounded-md transition-all ${
                                insightsTimeframe === tf
                                  ? 'bg-white/10 text-white font-semibold'
                                  : 'text-slate-400 hover:text-slate-200'
                              }`}
                            >
                              {tf} Tage
                            </button>
                          ))}
                        </div>
                      </div>

                      {/* Chart Rendering Container */}
                      <div className="bg-slate-950/60 border border-white/5 rounded-xl p-4 min-h-[260px] h-[260px] flex flex-col justify-center select-none">
                        {insightsTab === 'stacked' && (
                          <ResponsiveContainer width="100%" height="100%">
                            <BarChart data={chartData} margin={{ top: 10, right: 10, left: -15, bottom: 0 }}>
                              <XAxis dataKey="name" stroke="#64748b" fontSize={10} tickMargin={6} />
                              <YAxis stroke="#64748b" fontSize={10} allowDecimals={false} />
                              <Tooltip
                                contentStyle={{ backgroundColor: '#0f172a', borderColor: '#334155', borderRadius: '8px', color: '#f8fafc', fontSize: '12px' }}
                                itemStyle={{ padding: '2px 0' }}
                                cursor={{ fill: 'rgba(255, 255, 255, 0.05)' }}
                              />
                              {categories.map((cat) => (
                                <Bar key={cat} dataKey={cat} stackId="a" fill={getCategoryColor(cat)} name={cat} radius={[0, 0, 0, 0]} />
                              ))}
                            </BarChart>
                          </ResponsiveContainer>
                        )}

                        {insightsTab === 'trend' && (
                          <ResponsiveContainer width="100%" height="100%">
                            <LineChart data={chartData} margin={{ top: 10, right: 15, left: -15, bottom: 0 }}>
                              <XAxis dataKey="name" stroke="#64748b" fontSize={10} tickMargin={6} />
                              <YAxis stroke="#64748b" fontSize={10} allowDecimals={false} />
                              <Tooltip
                                contentStyle={{ backgroundColor: '#0f172a', borderColor: '#334155', borderRadius: '8px', color: '#f8fafc', fontSize: '12px' }}
                                formatter={(val: any) => [`${val} Aufgaben`, 'Erledigt']}
                              />
                              <Line 
                                type="monotone" 
                                dataKey="total" 
                                stroke="#6366f1" 
                                strokeWidth={2.5} 
                                dot={{ fill: '#6366f1', r: 3 }} 
                                activeDot={{ r: 5, stroke: '#ffffff', strokeWidth: 2 }} 
                                name="Erledigt" 
                              />
                            </LineChart>
                          </ResponsiveContainer>
                        )}

                        {insightsTab === 'donut' && (
                          pieData.length > 0 ? (
                            <div className="flex items-center justify-around h-full">
                              <div className="w-[180px] h-[180px]">
                                <ResponsiveContainer width="100%" height="100%">
                                  <PieChart>
                                    <Pie
                                      data={pieData}
                                      innerRadius={50}
                                      outerRadius={75}
                                      paddingAngle={3}
                                      dataKey="value"
                                    >
                                      {pieData.map((entry, index) => (
                                        <Cell key={`cell-${index}`} fill={entry.color} />
                                      ))}
                                    </Pie>
                                    <Tooltip 
                                      contentStyle={{ backgroundColor: '#0f172a', borderColor: '#334155', borderRadius: '8px', color: '#f8fafc', fontSize: '12px' }}
                                      formatter={(value: any) => [`${value} Aufgaben`, 'Anzahl']}
                                    />
                                  </PieChart>
                                </ResponsiveContainer>
                              </div>
                              <div className="space-y-1.5 max-w-[200px]">
                                {pieData.map(item => {
                                  const pct = completedInWindow > 0 ? Math.round((item.value / completedInWindow) * 100) : 0;
                                  return (
                                    <div key={item.name} className="flex items-center justify-between gap-3 text-xs">
                                      <div className="flex items-center gap-1.5 truncate">
                                        <div className="w-2.5 h-2.5 rounded-full shrink-0" style={{ backgroundColor: item.color }} />
                                        <span className="text-slate-300 truncate">{item.name}</span>
                                      </div>
                                      <span className="text-slate-400 font-mono text-[11px] shrink-0 font-medium">
                                        {item.value} ({pct}%)
                                      </span>
                                    </div>
                                  );
                                })}
                              </div>
                            </div>
                          ) : (
                            <div className="flex flex-col items-center justify-center text-slate-500 text-xs gap-1.5 h-full">
                              <Activity className="w-6 h-6 stroke-1 text-slate-600" />
                              <span>Keine erledigten Aufgaben in diesem Zeitraum</span>
                            </div>
                          )
                        )}
                      </div>

                      {/* Category Breakdown Badges */}
                      <div className="flex flex-wrap gap-2 pt-1">
                        {categories.map(cat => {
                          const count = catTotals[cat] || 0;
                          const color = getCategoryColor(cat);
                          return (
                            <div 
                              key={cat} 
                              className="flex items-center gap-1.5 px-2.5 py-1 rounded-lg border text-xs bg-slate-950/40"
                              style={{ borderColor: `${color}30` }}
                            >
                              <div className="w-2 h-2 rounded-full" style={{ backgroundColor: color }} />
                              <span className="text-slate-300">{cat}</span>
                              <span className="font-mono text-slate-400 font-bold ml-1">{count}</span>
                            </div>
                          );
                        })}
                      </div>
                    </>
                  );
                })()}
              </div>

              {/* Footer */}
              <div className="mt-3 pt-3 border-t border-white/10 flex justify-end shrink-0">
                <button
                  onClick={() => setShowInsightsPanel(false)}
                  className="px-5 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-sm text-white font-semibold transition-colors shadow-sm cursor-pointer"
                >
                  Fertig
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>

      {/* Bare Metal & Project Presets Modal */}
      <AnimatePresence>
        {showPresetModal && (
          <div className="fixed inset-0 z-[100] flex items-center justify-center p-3 sm:p-4 bg-black/85 backdrop-blur-md">
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 15 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 15 }}
              transition={{ duration: 0.2 }}
              className="bg-slate-900 border border-indigo-500/30 rounded-2xl w-full max-w-4xl max-h-[88vh] flex flex-col shadow-2xl overflow-hidden text-slate-100"
            >
              {/* Modal Header */}
              <div className="px-5 py-4 border-b border-white/10 flex items-center justify-between bg-slate-950/80 shrink-0">
                <div className="flex items-center gap-2.5">
                  <div className="p-2 rounded-xl bg-indigo-500/20 border border-indigo-500/30 text-indigo-400">
                    <Cpu className="w-5 h-5" />
                  </div>
                  <div>
                    <h2 className="text-base font-bold text-slate-100 flex items-center gap-2">
                      <span>Kernel & Projekt-Presets</span>
                      <span className="px-2 py-0.5 rounded-full text-[10px] font-mono font-semibold bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
                        Bare Metal Edition
                      </span>
                    </h2>
                    <p className="text-xs text-slate-400">
                      Fertige Roadmaps & Checklisten für Low-Level, OS- und Treiberentwicklung
                    </p>
                  </div>
                </div>

                <button
                  onClick={() => setShowPresetModal(false)}
                  className="p-1.5 rounded-lg text-slate-400 hover:text-white hover:bg-white/10 transition-colors cursor-pointer"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>

              {/* Navigation Tabs */}
              <div className="px-5 pt-3 pb-2 border-b border-white/5 bg-slate-950/40 flex items-center justify-between gap-3 shrink-0">
                <div className="flex items-center bg-slate-950/80 p-0.5 rounded-lg border border-white/5 text-xs">
                  <button
                    onClick={() => setPresetTab('projects')}
                    className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md font-medium transition-all cursor-pointer ${
                      presetTab === 'projects'
                        ? 'bg-indigo-600 text-white shadow-xs'
                        : 'text-slate-400 hover:text-slate-200'
                    }`}
                  >
                    <Layers className="w-3.5 h-3.5" />
                    <span>Projekt-Roadmaps ({PROJECT_PRESETS.length})</span>
                  </button>
                  <button
                    onClick={() => setPresetTab('subtasks')}
                    className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md font-medium transition-all cursor-pointer ${
                      presetTab === 'subtasks'
                        ? 'bg-indigo-600 text-white shadow-xs'
                        : 'text-slate-400 hover:text-slate-200'
                    }`}
                  >
                    <CheckCheck className="w-3.5 h-3.5" />
                    <span>Unteraufgaben-Checklisten ({subtaskSets.length})</span>
                  </button>
                </div>

                <div className="text-[11px] text-slate-400 hidden sm:flex items-center gap-1 font-mono">
                  <Sparkles className="w-3.5 h-3.5 text-amber-400" />
                  <span>1-Click Vorlagen-Generator</span>
                </div>
              </div>

              {/* Modal Body */}
              <div className="flex-1 overflow-y-auto p-4 sm:p-5">
                {presetTab === 'projects' ? (
                  <div className="grid grid-cols-1 md:grid-cols-12 gap-5 h-full">
                    {/* Preset Picker Sidebar */}
                    <div className="md:col-span-5 space-y-2.5">
                      <div className="text-[10px] font-bold text-slate-500 uppercase tracking-widest px-1">
                        Verfügbare Kernel-Architekturen
                      </div>
                      
                      {PROJECT_PRESETS.map((p) => {
                        const isSelected = selectedPresetId === p.id;
                        const totalSubs = p.tasks.reduce((sum, t) => sum + t.subtasks.length, 0);

                        return (
                          <div
                            key={p.id}
                            onClick={() => {
                              setSelectedPresetId(p.id);
                              setExpandedPresetTaskIndex(null);
                            }}
                            className={`p-3 rounded-xl border transition-all cursor-pointer select-none text-left ${
                              isSelected
                                ? 'bg-indigo-950/40 border-indigo-500/60 ring-1 ring-indigo-500/40 shadow-lg'
                                : 'bg-slate-950/40 border-white/5 hover:bg-slate-950/80 hover:border-white/10'
                            }`}
                          >
                            <div className="flex items-center justify-between gap-2 mb-1.5">
                              <span className="text-[10px] font-mono uppercase tracking-wider font-semibold px-2 py-0.5 rounded bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">
                                {p.categoryBadge}
                              </span>
                              <span className="text-[10px] text-slate-400 font-mono">
                                {p.tasks.length} Aufgaben
                              </span>
                            </div>

                            <h3 className="text-sm font-bold text-slate-100 mb-1 flex items-center gap-1.5">
                              {p.icon === 'Shield' && <Shield className="w-4 h-4 text-emerald-400 shrink-0" />}
                              {p.icon === 'Terminal' && <Terminal className="w-4 h-4 text-cyan-400 shrink-0" />}
                              {p.icon === 'Cpu' && <Cpu className="w-4 h-4 text-indigo-400 shrink-0" />}
                              <span>{p.title}</span>
                            </h3>

                            <p className="text-xs text-slate-400 line-clamp-2 mb-2 leading-relaxed">
                              {p.description}
                            </p>

                            <div className="flex items-center justify-between text-[10px] text-slate-500 pt-1.5 border-t border-white/5">
                              <div className="flex items-center gap-1">
                                {p.categories.slice(0, 4).map(c => (
                                  <span key={c.name} className="w-2 h-2 rounded-full" style={{ backgroundColor: c.color }} title={c.name} />
                                ))}
                                <span className="text-slate-400 ml-1">{p.categories.length} Tags</span>
                              </div>
                              <span className="font-mono text-slate-400">{totalSubs} Checklisten-Punkte</span>
                            </div>
                          </div>
                        );
                      })}
                    </div>

                    {/* Preset Detail & Preview Column */}
                    {(() => {
                      const activePreset = PROJECT_PRESETS.find(p => p.id === selectedPresetId) || PROJECT_PRESETS[0];
                      const totalSubs = activePreset.tasks.reduce((sum, t) => sum + t.subtasks.length, 0);

                      return (
                        <div className="md:col-span-7 bg-slate-950/60 border border-white/5 rounded-xl p-4 flex flex-col justify-between">
                          <div className="space-y-3">
                            {/* Preset Overview Banner */}
                            <div className="border-b border-white/10 pb-3">
                              <div className="flex items-center gap-2 mb-1">
                                <span className="px-2 py-0.5 rounded text-[10px] font-mono font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30">
                                  {activePreset.categoryBadge}
                                </span>
                                <span className="text-[11px] text-slate-400 font-mono">
                                  {activePreset.tasks.length} Hauptaufgaben • {totalSubs} Unteraufgaben
                                </span>
                              </div>
                              <h3 className="text-base font-bold text-slate-100">
                                {activePreset.title}
                              </h3>
                              <p className="text-xs text-slate-300 mt-1 leading-relaxed">
                                {activePreset.description}
                              </p>

                              {/* Category Tag Pills */}
                              <div className="flex flex-wrap gap-1.5 mt-2.5">
                                {activePreset.categories.map(c => (
                                  <span
                                    key={c.name}
                                    className="px-2 py-0.5 rounded-full text-[10px] font-medium border flex items-center gap-1 bg-slate-900"
                                    style={{ borderColor: `${c.color}40`, color: c.color }}
                                  >
                                    <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: c.color }} />
                                    {c.name}
                                  </span>
                                ))}
                              </div>
                            </div>

                            {/* Task Breakdown Preview */}
                            <div className="space-y-1.5">
                              <div className="flex items-center justify-between text-[11px] text-slate-400 px-1 font-medium">
                                <span>Enthaltene Meilensteine ({activePreset.tasks.length}):</span>
                                <span className="text-[10px] text-slate-500 font-mono">Klick zum Aufklappen</span>
                              </div>

                              <div className="space-y-2 max-h-[260px] overflow-y-auto pr-1">
                                {activePreset.tasks.map((task, idx) => {
                                  const isExpanded = expandedPresetTaskIndex === idx;
                                  const catColor = activePreset.categories.find(c => c.name === task.category)?.color || '#38bdf8';

                                  return (
                                    <div
                                      key={idx}
                                      onClick={() => setExpandedPresetTaskIndex(isExpanded ? null : idx)}
                                      className="p-2.5 rounded-lg border border-white/5 bg-slate-900/80 hover:border-white/15 transition-all cursor-pointer"
                                    >
                                      <div className="flex items-start justify-between gap-2">
                                        <div className="flex items-start gap-2 min-w-0">
                                          <span className="text-[10px] font-mono font-bold text-slate-500 shrink-0 mt-0.5">
                                            #{idx + 1}
                                          </span>
                                          <div className="min-w-0">
                                            <div className="text-xs font-semibold text-slate-200 leading-snug">
                                              {task.text}
                                            </div>
                                            <div className="flex items-center gap-2 mt-1">
                                              <span
                                                className="px-1.5 py-0.2 rounded text-[9px] font-medium"
                                                style={{ backgroundColor: `${catColor}20`, color: catColor }}
                                              >
                                                {task.category}
                                              </span>
                                              <span className="text-[9px] text-slate-400 font-mono">
                                                {task.subtasks.length} Checklisten-Punkte
                                              </span>
                                              {task.dueDateOffsetDays && (
                                                <span className="text-[9px] text-slate-500 font-mono">
                                                  Tag +{task.dueDateOffsetDays}
                                                </span>
                                              )}
                                            </div>
                                          </div>
                                        </div>

                                        <ChevronDown className={`w-4 h-4 text-slate-500 transition-transform shrink-0 mt-0.5 ${isExpanded ? 'rotate-180 text-slate-300' : ''}`} />
                                      </div>

                                      {isExpanded && (
                                        <motion.div
                                          initial={{ opacity: 0, height: 0 }}
                                          animate={{ opacity: 1, height: 'auto' }}
                                          exit={{ opacity: 0, height: 0 }}
                                          className="mt-2.5 pt-2 border-t border-white/5 space-y-1.5"
                                        >
                                          {task.notes && (
                                            <div className="p-2 rounded bg-black/40 border border-white/5 text-[11px] text-indigo-300 font-mono leading-relaxed">
                                              💡 {task.notes}
                                            </div>
                                          )}

                                          <div className="space-y-1 pl-1 pt-1">
                                            {task.subtasks.map((st, stIdx) => (
                                              <div key={stIdx} className="flex items-start gap-1.5 text-[11px] text-slate-300">
                                                <Check className="w-3 h-3 text-emerald-400 shrink-0 mt-0.5" />
                                                <span>{st}</span>
                                              </div>
                                            ))}
                                          </div>
                                        </motion.div>
                                      )}
                                    </div>
                                  );
                                })}
                              </div>
                            </div>
                          </div>

                          {/* Preset Action Buttons */}
                          <div className="mt-4 pt-3 border-t border-white/10 flex flex-col sm:flex-row items-center justify-end gap-2.5 shrink-0">
                            <button
                              onClick={() => applyProjectPreset(activePreset, 'append')}
                              className="w-full sm:w-auto px-4 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer border border-white/10"
                            >
                              <Plus className="w-3.5 h-3.5" />
                              <span>Zu Aufgaben hinzufügen</span>
                            </button>

                            <button
                              onClick={() => {
                                if (todos.length > 0 && !window.confirm(`Möchten Sie das Projekt "${activePreset.title}" als aktive Liste laden? Aktuelle Aufgaben werden ersetzt.`)) {
                                  return;
                                }
                                applyProjectPreset(activePreset, 'replace');
                              }}
                              className="w-full sm:w-auto px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer shadow-md shadow-indigo-900/30"
                            >
                              <Zap className="w-3.5 h-3.5" />
                              <span>Als neues Projekt laden</span>
                            </button>
                          </div>
                        </div>
                      );
                    })()}
                  </div>
                ) : (
                  /* Subtask Presets View */
                  <div className="space-y-3">
                    <div className="flex items-center justify-between text-xs text-slate-400 pb-1">
                      <span>Wählen Sie eine Checkliste, um sofort eine neue Aufgabe anzulegen:</span>
                      <span className="font-mono text-[11px]">{subtaskSets.length} Vorlagen verfügbar</span>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                      {subtaskSets.map((set) => {
                        const isKernel = set.id.startsWith('k-');
                        const presetDef = KERNEL_SUBTASK_PRESETS.find(k => k.id === set.id);

                        return (
                          <div
                            key={set.id}
                            className="bg-slate-950/60 border border-white/5 hover:border-indigo-500/30 rounded-xl p-3.5 flex flex-col justify-between transition-all"
                          >
                            <div>
                              <div className="flex items-center justify-between gap-2 mb-1.5">
                                <span className={`text-[10px] font-mono px-2 py-0.5 rounded font-semibold ${
                                  isKernel
                                    ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
                                    : 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
                                }`}>
                                  {isKernel ? (presetDef?.category || 'Bare Metal') : 'Benutzerdefiniert'}
                                </span>
                                <span className="text-[10px] text-slate-500 font-mono">
                                  {set.subtasks.length} Schritte
                                </span>
                              </div>

                              <h4 className="text-sm font-bold text-slate-100 mb-2">
                                {set.name}
                              </h4>

                              <div className="space-y-1 pl-1 mb-3">
                                {set.subtasks.slice(0, 4).map((st, i) => (
                                  <div key={i} className="flex items-start gap-1.5 text-[11px] text-slate-400">
                                    <Check className="w-3 h-3 text-emerald-400 shrink-0 mt-0.5" />
                                    <span className="truncate">{st}</span>
                                  </div>
                                ))}
                                {set.subtasks.length > 4 && (
                                  <div className="text-[10px] text-slate-500 pl-4 font-mono">
                                    + {set.subtasks.length - 4} weitere Schritte
                                  </div>
                                )}
                              </div>
                            </div>

                            <button
                              onClick={() => {
                                applySubtaskPresetToNewTodo({
                                  id: set.id,
                                  name: set.name,
                                  category: presetDef?.category || 'Work',
                                  subtasks: set.subtasks
                                });
                              }}
                              className="w-full py-1.5 bg-indigo-500/15 hover:bg-indigo-500/25 text-indigo-300 border border-indigo-500/30 rounded-lg text-xs font-semibold transition-colors flex items-center justify-center gap-1.5 cursor-pointer"
                            >
                              <Plus className="w-3.5 h-3.5" />
                              <span>Aufgabe aus Vorlage erstellen</span>
                            </button>
                          </div>
                        );
                      })}
                    </div>
                  </div>
                )}
              </div>

              {/* Modal Footer */}
              <div className="px-5 py-3 border-t border-white/10 bg-slate-950/80 flex items-center justify-between text-xs text-slate-400 shrink-0">
                <span className="font-mono text-[11px]">
                  Tipp: Eigene Unteraufgaben-Checklisten können jederzeit über das <Bookmark className="w-3 h-3 inline mx-0.5 text-indigo-400" /> Symbol gespeichert werden.
                </span>
                <button
                  onClick={() => setShowPresetModal(false)}
                  className="px-4 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-lg font-medium transition-colors cursor-pointer"
                >
                  Schließen
                </button>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
    </div>
  );
}
