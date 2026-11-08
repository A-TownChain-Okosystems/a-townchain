import React, { useState, useEffect } from 'react';
import { Folder, File, HardDrive, Download, UploadCloud, Database, ChevronRight, CornerLeftUp } from 'lucide-react';

const INITIAL_FILES = [
  { id: '1', name: "system", types: "folder", parentId: null, size: "1.2 GB" },
  { id: '2', name: "agents", types: "folder", parentId: null, size: "450 MB" },
  { id: '3', name: "contracts", types: "folder", parentId: null, size: "12 KB" },
  { id: '4', name: "kernel.atc", types: "file", parentId: null, size: "8.4 MB" },
  { id: '5', name: "genesis.block", types: "file", parentId: null, size: "2 KB" }
];

export function FileManagerView() {
  const [files, setFiles] = useState<{ id: string, name: string, types: string, parentId: string | null, size: string }[]>(() => {
    try {
      const stored = localStorage.getItem('atc_files_db');
      return stored ? JSON.parse(stored) : INITIAL_FILES;
    } catch {
      return INITIAL_FILES;
    }
  });

  const [currentFolder, setCurrentFolder] = useState<string | null>(null);

  useEffect(() => {
    localStorage.setItem('atc_files_db', JSON.stringify(files));
  }, [files]);

  const handleDragStart = (e: React.DragEvent, id: string) => {
    e.dataTransfer.setData('fileId', id);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  const handleDrop = (e: React.DragEvent, targetId: string) => {
    e.preventDefault();
    const sourceId = e.dataTransfer.getData('fileId');
    if (!sourceId || sourceId === targetId) return;

    const targetFolder = files.find(f => f.id === targetId);
    if (!targetFolder || targetFolder.types !== 'folder') return;

    setFiles(prev => prev.map(f => f.id === sourceId ? { ...f, parentId: targetId } : f));
  };

  const currentFiles = files.filter(f => f.parentId === currentFolder);

  const breadcrumbs = [];
  let curr = currentFolder;
  while (curr) {
    const folder = files.find(f => f.id === curr);
    if (folder) {
      breadcrumbs.unshift(folder);
      curr = folder.parentId;
    } else {
      break;
    }
  }

  const navigateUp = () => {
    if (currentFolder) {
      const folder = files.find(f => f.id === currentFolder);
      if (folder) setCurrentFolder(folder.parentId);
    }
  };

  const currentPathStr = breadcrumbs.length > 0 ? `/dev/root/${breadcrumbs.map(b => b.name).join('/')}` : '/dev/root/';

  return (
    <div className="flex h-full bg-[#060a16] text-slate-300 rounded-xl border border-white/10">
      {/* Sidebar */}
      <div className="w-64 border-r border-white/5 bg-[#090b14] p-4 flex flex-col gap-6">
        <div>
          <h3 className="text-xs font-semibold text-slate-500 uppercase tracking-widest mb-3">Locations</h3>
          <ul className="space-y-2 text-sm">
            <li 
              className="flex items-center gap-3 text-atc-cyan bg-white/5 p-2 rounded cursor-pointer"
              onClick={() => setCurrentFolder(null)}
            >
              <HardDrive className="w-4 h-4" /> ATC-OS Drive
            </li>
            <li className="flex items-center gap-3 hover:text-white p-2 rounded cursor-pointer transition-colors">
              <Database className="w-4 h-4" /> Decentralized IPFS
            </li>
            <li className="flex items-center gap-3 hover:text-white p-2 rounded cursor-pointer transition-colors">
              <UploadCloud className="w-4 h-4" /> Cloud Nodes
            </li>
            <li className="flex items-center gap-3 hover:text-white p-2 rounded cursor-pointer transition-colors">
              <Download className="w-4 h-4" /> Downloads
            </li>
          </ul>
        </div>
      </div>

      {/* Main Area */}
      <div 
        className="flex-1 p-6"
        onDragOver={handleDragOver}
        onDrop={(e) => {
          // If dropped on the empty space, move back to currentFolder
          e.preventDefault();
          const sourceId = e.dataTransfer.getData('fileId');
          if (sourceId) {
             setFiles(prev => prev.map(f => f.id === sourceId ? { ...f, parentId: currentFolder } : f));
          }
        }}
      >
        <div className="flex items-center justify-between mb-8">
          <div className="flex items-center gap-4">
            {currentFolder && (
              <button 
                onClick={navigateUp}
                className="w-8 h-8 flex items-center justify-center rounded-lg bg-white/5 hover:bg-white/10 transition-colors"
                title="Go up one level"
              >
                <CornerLeftUp className="w-4 h-4 text-slate-400" />
              </button>
            )}
            <h2 className="text-2xl font-bold text-white tracking-tight flex items-center gap-2">
              <span className="cursor-pointer hover:text-atc-cyan transition-colors" onClick={() => setCurrentFolder(null)}>Root</span>
              {breadcrumbs.map(b => (
                <React.Fragment key={b.id}>
                  <ChevronRight className="w-5 h-5 text-slate-500" />
                  <span className="cursor-pointer hover:text-atc-cyan transition-colors" onClick={() => setCurrentFolder(b.id)}>{b.name}</span>
                </React.Fragment>
              ))}
            </h2>
          </div>
          <div className="text-sm text-slate-500 font-mono">{currentPathStr}</div>
        </div>

        <div className="grid grid-cols-4 gap-4">
          {currentFiles.map((f) => (
            <div 
              key={f.id} 
              draggable
              onDragStart={(e) => handleDragStart(e, f.id)}
              onDragOver={f.types === 'folder' ? handleDragOver : undefined}
              onDrop={f.types === 'folder' ? (e) => { e.stopPropagation(); handleDrop(e, f.id); } : undefined}
              onClick={() => {
                if (f.types === 'folder') {
                  setCurrentFolder(f.id);
                }
              }}
              className="bg-white/5 border border-white/10 rounded-lg p-4 flex flex-col items-center gap-3 hover:bg-white/10 transition-colors cursor-pointer group hover:border-indigo-500/50"
            >
              {f.types === 'folder' ? (
                <Folder className="w-12 h-12 text-indigo-400 group-hover:scale-110 transition-transform" />
              ) : (
                <File className="w-12 h-12 text-slate-400 group-hover:scale-110 transition-transform" />
              )}
              <div className="text-center">
                <div className="text-sm font-medium text-white break-all">{f.name}</div>
                <div className="text-xs text-slate-500 mt-1">
                  {f.types === 'folder' ? `${files.filter(child => child.parentId === f.id).length} items` : f.size}
                </div>
              </div>
            </div>
          ))}
          {currentFiles.length === 0 && (
            <div className="col-span-4 py-12 text-center text-slate-500">
              Folder is empty
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
