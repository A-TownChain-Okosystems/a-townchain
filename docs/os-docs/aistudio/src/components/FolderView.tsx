import React, { useState } from 'react';
import { Folder, File, ChevronRight, HardDrive, ArrowLeft, Plus } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

interface FileItem {
  id: string;
  name: string;
  type: 'file' | 'folder';
  parentId: string | null;
}

export function FolderView() {
  const [items, setItems] = useState<FileItem[]>([
    { id: '1', name: 'Documents', type: 'folder', parentId: null },
    { id: '2', name: 'Images', type: 'folder', parentId: null },
    { id: '3', name: 'system.log', type: 'file', parentId: null },
    { id: '4', name: 'work.md', type: 'file', parentId: '1' }
  ]);
  
  const [currentFolder, setCurrentFolder] = useState<string | null>(null);
  
  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    const fileId = e.dataTransfer.getData('application/atc-file');
    if (fileId) {
      setItems(prev => prev.map(item => 
        item.id === fileId ? { ...item, parentId: currentFolder } : item
      ));
    } else {
      // Just simulate dropping external files
      setItems(prev => [
        ...prev, 
        { id: Math.random().toString(36).substring(7), name: `New Item ${prev.length}`, type: 'file', parentId: currentFolder }
      ]);
    }
  };

  const currentItems = items.filter(i => i.parentId === currentFolder);
  const currentFolderName = currentFolder ? items.find(i => i.id === currentFolder)?.name : 'Root';

  return (
    <div className="flex flex-col h-full bg-[#050811] text-slate-200">
      <div className="p-3 border-b border-white/5 bg-black/40 flex items-center gap-2">
        <button 
           disabled={!currentFolder}
           onClick={() => {
             if (!currentFolder) return;
             const parent = items.find(i => i.id === currentFolder)?.parentId;
             setCurrentFolder(parent || null);
           }}
           className="p-1.5 rounded hover:bg-white/10 disabled:opacity-30 disabled:hover:bg-transparent transition-colors"
        >
          <ArrowLeft className="w-4 h-4" />
        </button>
        <div className="flex items-center gap-2 text-sm text-slate-400">
          <HardDrive className="w-4 h-4" />
          <ChevronRight className="w-3 h-3" />
          <span className="font-medium text-white">{currentFolderName}</span>
        </div>
        <div className="flex-1" />
        <button 
           onClick={() => setItems(prev => [...prev, { id: Math.random().toString(), name: 'New Folder', type: 'folder', parentId: currentFolder }])}
           className="p-1.5 rounded hover:bg-white/10 transition-colors title='New Folder'"
        >
           <Plus className="w-4 h-4" />
        </button>
      </div>

      <div 
        className="flex-1 p-4 overflow-y-auto"
        onDragOver={e => e.preventDefault()}
        onDrop={handleDrop}
      >
        <div className="flex flex-wrap gap-4">
          <AnimatePresence>
            {currentItems.map(item => (
              <motion.div
                key={item.id}
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.9 }}
                draggable
                onDragStart={(e: any) => e.dataTransfer.setData('application/atc-file', item.id)}
                onClick={() => {
                  if (item.type === 'folder') {
                    setCurrentFolder(item.id);
                  }
                }}
                className="w-24 p-3 flex flex-col items-center gap-2 rounded-xl hover:bg-white/5 cursor-pointer border border-transparent hover:border-white/10 transition-all group"
              >
                {item.type === 'folder' ? (
                  <Folder className="w-10 h-10 text-indigo-400 group-hover:text-indigo-300 transition-colors" />
                ) : (
                  <File className="w-10 h-10 text-slate-400 group-hover:text-slate-300 transition-colors" />
                )}
                <span className="text-xs text-center text-slate-300 break-words w-full truncate">{item.name}</span>
              </motion.div>
            ))}
          </AnimatePresence>
          {currentItems.length === 0 && (
            <div className="w-full h-full flex flex-col items-center justify-center text-slate-500 gap-2 mt-20 opacity-50">
              <Folder className="w-12 h-12" />
              <p className="text-sm">Folder is empty</p>
              <p className="text-xs">Drag and drop files here</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
