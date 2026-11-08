import React, { useState, useEffect, useRef } from 'react';
import { X, Minus, Maximize2, Monitor, Book, Database } from 'lucide-react';
import { WindowWikiView, WindowDbView } from './WindowExtras';

export function Window({ 
  id, 
  title, 
  icon: Icon, 
  children, 
  onClose, 
  isActive, 
  onFocus,
  initialWidth = 800,
  initialHeight = 600
}: any) {
  const [pos, setPos] = useState({ x: window.innerWidth / 2 - initialWidth / 2, y: window.innerHeight / 2 - initialHeight / 2 });
  const [size, setSize] = useState({ width: initialWidth, height: initialHeight });
  const [isMaximized, setIsMaximized] = useState(false);
  const [snapPreview, setSnapPreview] = useState<'left' | 'right' | null>(null);
  const [viewMode, setViewMode] = useState<'app' | 'wiki' | 'db'>('app');
  
  const isDragging = useRef(false);
  const dragOffset = useRef({ x: 0, y: 0 });
  const windowRef = useRef<HTMLDivElement>(null);

  const startDrag = (e: React.MouseEvent) => {
    if (isMaximized) return;
    isDragging.current = true;
    dragOffset.current = {
      x: e.clientX - pos.x,
      y: e.clientY - pos.y
    };
    onFocus(id);
  };

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging.current) return;
      
      let newX = e.clientX - dragOffset.current.x;
      let newY = e.clientY - dragOffset.current.y;
      
      // Update position
      setPos({ x: newX, y: newY });
      
      // Check snap areas
      if (e.clientX < 20) {
        setSnapPreview('left');
      } else if (e.clientX > window.innerWidth - 20) {
        setSnapPreview('right');
      } else {
        setSnapPreview(null);
      }
    };

    const handleMouseUp = (e: MouseEvent) => {
      if (!isDragging.current) return;
      isDragging.current = false;
      
      if (e.clientX < 20) {
        setPos({ x: 0, y: 0 });
        setSize({ width: window.innerWidth / 2, height: window.innerHeight - 40 }); // Assuming taskbar is 40px
        setSnapPreview(null);
      } else if (e.clientX > window.innerWidth - 20) {
        setPos({ x: window.innerWidth / 2, y: 0 });
        setSize({ width: window.innerWidth / 2, height: window.innerHeight - 40 });
        setSnapPreview(null);
      }
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [pos.x, pos.y]);

  const toggleMaximize = () => {
    setIsMaximized(!isMaximized);
  };

  return (
    <>
      {snapPreview === 'left' && (
        <div className="fixed top-0 left-0 w-1/2 h-[calc(100vh-40px)] bg-white/10 border-2 border-white/20 z-[90] pointer-events-none transition-all" />
      )}
      {snapPreview === 'right' && (
        <div className="fixed top-0 right-0 w-1/2 h-[calc(100vh-40px)] bg-white/10 border-2 border-white/20 z-[90] pointer-events-none transition-all" />
      )}
      <div
        ref={windowRef}
        onMouseDown={() => onFocus(id)}
        className={`absolute bg-[#090b14] border-atc-border shadow-2xl flex flex-col overflow-hidden transition-shadow ${
          isActive ? 'z-[100] shadow-[0_0_30px_rgba(0,0,0,0.8)] border border-white/20' : 'z-50 border border-white/10 opacity-95'
        } ${isMaximized ? 'rounded-none' : 'rounded-lg'}`}
        style={isMaximized ? {
          top: 0, left: 0, width: '100%', height: 'calc(100vh - 40px)'
        } : {
          top: pos.y,
          left: pos.x,
          width: size.width,
          height: size.height
        }}
      >
        <div 
          className="h-10 bg-black/40 flex items-center justify-between select-none border-b border-white/10 cursor-move"
          onMouseDown={startDrag}
        >
          <div className="flex items-center px-4 space-x-2 pointer-events-none">
            {Icon && <Icon className="w-4 h-4 text-atc-cyan" />}
            <span className="text-xs font-semibold text-slate-200">{title}</span>
          </div>
          
          <div className="flex-1 flex justify-center items-center pointer-events-auto" onMouseDown={(e) => e.stopPropagation()}>
            <div className="flex p-0.5 bg-black/30 border border-white/5 rounded mx-2">
              <button 
                onClick={() => setViewMode('app')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded-sm text-[10px] font-bold uppercase tracking-wider transition-colors ${viewMode === 'app' ? 'bg-white/10 text-white' : 'text-slate-500 hover:text-slate-300'}`}
              >
                 <Monitor className="w-3 h-3" /> App
              </button>
              <button 
                onClick={() => setViewMode('wiki')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded-sm text-[10px] font-bold uppercase tracking-wider transition-colors ${viewMode === 'wiki' ? 'bg-emerald-500/20 text-emerald-400' : 'text-slate-500 hover:text-emerald-400/50'}`}
              >
                 <Book className="w-3 h-3" /> Wiki
              </button>
              <button 
                onClick={() => setViewMode('db')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded-sm text-[10px] font-bold uppercase tracking-wider transition-colors ${viewMode === 'db' ? 'bg-indigo-500/20 text-indigo-400' : 'text-slate-500 hover:text-indigo-400/50'}`}
              >
                 <Database className="w-3 h-3" /> DB
              </button>
            </div>
          </div>

          <div className="flex items-stretch h-full">
            <button className="px-4 hover:bg-white/10 transition-colors flex items-center justify-center">
              <Minus className="w-4 h-4 text-slate-300" />
            </button>
            <button onClick={toggleMaximize} className="px-4 hover:bg-white/10 transition-colors flex items-center justify-center">
              <Maximize2 className="w-3.5 h-3.5 text-slate-300" />
            </button>
            <button onClick={(e) => { e.stopPropagation(); onClose(id); }} className="px-4 hover:bg-red-500 hover:text-white transition-colors flex items-center justify-center">
              <X className="w-4 h-4 text-slate-300" />
            </button>
          </div>
        </div>
        <div className="flex-1 overflow-y-auto bg-atc-bg relative">
          {viewMode === 'app' && children}
          {viewMode === 'wiki' && <WindowWikiView id={id} title={title} />}
          {viewMode === 'db' && <WindowDbView id={id} title={title} />}
        </div>
      </div>
    </>
  );
}
