import React, { useState, useEffect } from "react";
import { History, GitCommit, GitBranch, Check, Clock, CloudUpload, CloudDownload, RefreshCw, AlertCircle, Archive, Download, Github } from "lucide-react";
import { FileState } from "../App";
import JSZip from 'jszip';
import { GitHubSyncModal } from "./GitHubSyncModal";

interface CommitData {
  id: string;
  message: string;
  timestamp: number;
  files: FileState[];
}

// ----- IndexedDB Mock API -----
const DB_NAME = "GithubMockDB";
const STORE_NAME = "repository";

const initDB = (): Promise<IDBDatabase> => {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, 1);
    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve(request.result);
    request.onupgradeneeded = (e) => {
      const db = (e.target as IDBOpenDBRequest).result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        db.createObjectStore(STORE_NAME, { keyPath: "id" });
      }
    };
  });
};

const mockApiPush = async (commits: CommitData[]): Promise<void> => {
  return new Promise(async (resolve, reject) => {
    try {
      // Simulate network delay
      await new Promise(r => setTimeout(r, 800));
      const db = await initDB();
      const transaction = db.transaction(STORE_NAME, "readwrite");
      const store = transaction.objectStore(STORE_NAME);
      
      // Clear existing commits (simplified full sync)
      store.clear();
      
      commits.forEach(commit => {
        store.put(commit);
      });
      
      transaction.oncomplete = () => resolve();
      transaction.onerror = () => reject(transaction.error);
    } catch (e) {
      reject(e);
    }
  });
};

const mockApiPull = async (): Promise<CommitData[]> => {
  return new Promise(async (resolve, reject) => {
    try {
      // Simulate network delay
      await new Promise(r => setTimeout(r, 800));
      const db = await initDB();
      const transaction = db.transaction(STORE_NAME, "readonly");
      const store = transaction.objectStore(STORE_NAME);
      const request = store.getAll();
      
      request.onsuccess = () => {
        // Sort by timestamp descending
        const data = (request.result as CommitData[]).sort((a, b) => b.timestamp - a.timestamp);
        resolve(data);
      };
      request.onerror = () => reject(request.error);
    } catch (e) {
      reject(e);
    }
  });
};
// ------------------------------

export function VersionControlPanel({ files, setFiles }: { files: FileState[], setFiles: React.Dispatch<React.SetStateAction<FileState[]>> }) {
  const [commits, setCommits] = useState<CommitData[]>([]);
  const [commitMessage, setCommitMessage] = useState("");
  const [syncState, setSyncState] = useState<"idle" | "syncing" | "success" | "error">("idle");
  const [errorMessage, setErrorMessage] = useState("");
  const [isGitHubModalOpen, setIsGitHubModalOpen] = useState(false);

  useEffect(() => {
    const saved = localStorage.getItem("atos-git-history");
    if (saved) {
      try {
        setCommits(JSON.parse(saved));
      } catch (e) {
        console.error("Failed to parse git history", e);
      }
    }
  }, []);

  const handleCommit = () => {
    if (!commitMessage.trim()) return;

    const newCommit: CommitData = {
      id: Math.random().toString(36).substring(2, 9),
      message: commitMessage.trim(),
      timestamp: Date.now(),
      files: JSON.parse(JSON.stringify(files)), // Deep copy
    };

    const newCommits = [newCommit, ...commits];
    setCommits(newCommits);
    localStorage.setItem("atos-git-history", JSON.stringify(newCommits));
    setCommitMessage("");
  };

  const handleRestore = (commit: CommitData) => {
    if (window.confirm(`Are you sure you want to restore to commit "${commit.message}"? This will overwrite your current files.`)) {
      setFiles(commit.files);
    }
  };

  const handlePush = async () => {
    setSyncState("syncing");
    try {
      await mockApiPush(commits);
      setSyncState("success");
      setTimeout(() => setSyncState("idle"), 2000);
    } catch (e) {
      setSyncState("error");
      setErrorMessage(e instanceof Error ? e.message : "Sync failed");
      setTimeout(() => setSyncState("idle"), 4000);
    }
  };

  const handlePull = async () => {
    setSyncState("syncing");
    try {
      const data = await mockApiPull();
      setCommits(data);
      localStorage.setItem("atos-git-history", JSON.stringify(data));
      setSyncState("success");
      setTimeout(() => setSyncState("idle"), 2000);
    } catch (e) {
      setSyncState("error");
      setErrorMessage(e instanceof Error ? e.message : "Sync failed");
      setTimeout(() => setSyncState("idle"), 4000);
    }
  };

  const handleSyncToOffline = async () => {
    setSyncState("syncing");
    try {
      const zip = new JSZip();
      
      files.forEach(file => {
        zip.file(file.name, file.content);
      });
      zip.file('.git-history.json', JSON.stringify(commits, null, 2));

      const blob = await zip.generateAsync({ type: 'blob' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `project-backup-${new Date().toISOString().split('T')[0]}.zip`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      
      setSyncState("success");
      setTimeout(() => setSyncState("idle"), 2000);
    } catch (e) {
      setSyncState("error");
      setErrorMessage(e instanceof Error ? e.message : "Export failed");
      setTimeout(() => setSyncState("idle"), 4000);
    }
  };

  const handleSyncGitHub = () => {
    setIsGitHubModalOpen(true);
  };

  return (
    <div className="flex-1 flex flex-col bg-[#0f0f13] text-slate-300 relative z-10 overflow-hidden shadow-2xl">
      {isGitHubModalOpen && (
        <GitHubSyncModal
          isOpen={isGitHubModalOpen}
          onClose={() => setIsGitHubModalOpen(false)}
          files={files}
          onLoadFiles={setFiles}
        />
      )}

      <div className="h-10 bg-[#15151a] border-b border-white/5 flex items-center justify-between px-4 shrink-0">
        <div className="flex items-center">
          <GitBranch className="w-5 h-5 text-emerald-400 mr-2" />
          <span className="font-bold text-sm tracking-wide text-slate-200">VERSION CONTROL</span>
        </div>
        <div className="flex gap-2 items-center">
          {syncState === "syncing" && <RefreshCw className="w-4 h-4 text-emerald-400 animate-spin" />}
          {syncState === "success" && <Check className="w-4 h-4 text-emerald-400" />}
          {syncState === "error" && <span className="text-xs text-red-400 flex items-center"><AlertCircle className="w-4 h-4 mr-1"/> {errorMessage}</span>}
          
          <button 
            onClick={handleSyncGitHub}
            disabled={syncState === "syncing"}
            className="flex items-center text-xs font-medium px-2 py-1 rounded bg-white/5 hover:bg-white/10 transition-colors disabled:opacity-50"
            title="Sync with GitHub Repository"
          >
            <Github className="w-3.5 h-3.5 mr-1.5 text-white" />
            Sync GitHub
          </button>
          <button 
            onClick={handleSyncToOffline}
            disabled={syncState === "syncing"}
            className="flex items-center text-xs font-medium px-2 py-1 rounded bg-white/5 hover:bg-white/10 transition-colors disabled:opacity-50"
            title="Sync to Offline (Zip Export)"
          >
            <Archive className="w-3.5 h-3.5 mr-1.5 text-amber-400" />
            Sync to Offline
          </button>
          <button 
            onClick={() => {
              const dataStr = JSON.stringify(files, null, 2);
              const blob = new Blob([dataStr], { type: 'application/json' });
              const url = URL.createObjectURL(blob);
              const a = document.createElement('a');
              a.href = url;
              a.download = `project-export-${new Date().toISOString().split('T')[0]}.json`;
              document.body.appendChild(a);
              a.click();
              document.body.removeChild(a);
              URL.revokeObjectURL(url);
            }}
            className="flex items-center text-xs font-medium px-2 py-1 rounded bg-white/5 hover:bg-white/10 transition-colors"
            title="Export Project (JSON)"
          >
            <Download className="w-3.5 h-3.5 mr-1.5 text-pink-400" />
            Export JSON
          </button>
          <button 
            onClick={handlePull}
            disabled={syncState === "syncing"}
            className="flex items-center text-xs font-medium px-2 py-1 rounded bg-white/5 hover:bg-white/10 transition-colors disabled:opacity-50"
            title="Pull from Cloud (Mock GitHub)"
          >
            <CloudDownload className="w-3.5 h-3.5 mr-1.5 text-blue-400" />
            Pull
          </button>
          <button 
            onClick={handlePush}
            disabled={syncState === "syncing" || commits.length === 0}
            className="flex items-center text-xs font-medium px-2 py-1 rounded bg-white/5 hover:bg-white/10 transition-colors disabled:opacity-50"
            title="Push to Cloud (Mock GitHub)"
          >
            <CloudUpload className="w-3.5 h-3.5 mr-1.5 text-cyan-400" />
            Push
          </button>
        </div>
      </div>

      <div className="flex-1 flex overflow-hidden">
        {/* Commit form */}
        <div className="w-64 border-r border-white/5 flex flex-col bg-black/20 p-4 shrink-0 gap-4">
           <div>
              <h3 className="text-xs font-bold text-slate-400 mb-2 uppercase tracking-wide">Changes</h3>
              <div className="text-sm text-slate-300 bg-white/5 p-2 rounded-md border border-white/10 mb-4">
                {files.length} modified files
              </div>
              <h3 className="text-xs font-bold text-slate-400 mb-2 uppercase tracking-wide">New Commit</h3>
              <textarea
                value={commitMessage}
                onChange={(e) => setCommitMessage(e.target.value)}
                placeholder="Commit message..."
                className="w-full bg-black/40 border border-white/10 rounded px-3 py-2 text-sm outline-none focus:border-emerald-500/50 resize-none h-24 mb-3"
              />
              <button
                onClick={handleCommit}
                disabled={!commitMessage.trim()}
                className="w-full bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 disabled:opacity-50 disabled:hover:bg-emerald-500/20 border border-emerald-500/30 py-2 rounded text-sm font-bold flex items-center justify-center transition-colors"
               >
                 <Check className="w-4 h-4 mr-2" />
                 Commit Changes
              </button>
           </div>
        </div>

        {/* Commit History */}
        <div className="flex-1 overflow-y-auto bg-black/10 p-4">
           <h3 className="text-xs font-bold text-slate-400 mb-4 uppercase tracking-wide flex items-center">
             <History className="w-4 h-4 mr-2" />
             Commit History
           </h3>
           
           <div className="space-y-3">
             {commits.length === 0 ? (
               <div className="text-slate-500 text-sm italic text-center py-8">No commits yet</div>
             ) : (
               commits.map((commit, index) => (
                 <div key={commit.id} className="bg-white/[0.02] border border-white/5 rounded-lg p-3 relative group hover:border-emerald-500/20 transition-colors">
                    <div className="absolute left-0 top-0 bottom-0 w-1 bg-emerald-500/20 rounded-l-lg opacity-0 group-hover:opacity-100 transition-opacity"></div>
                    <div className="flex items-start justify-between">
                       <div>
                         <div className="flex items-center gap-2 mb-1">
                           <GitCommit className="w-4 h-4 text-emerald-400" />
                           <span className="font-mono text-xs text-emerald-400 bg-emerald-400/10 px-1.5 py-0.5 rounded">
                             {commit.id}
                           </span>
                           <span className="text-xs text-slate-500 flex items-center">
                             <Clock className="w-3 h-3 mr-1" />
                             {new Date(commit.timestamp).toLocaleString()}
                           </span>
                         </div>
                         <div className="text-sm text-slate-200 mt-2 font-medium">
                           {commit.message}
                         </div>
                       </div>
                       <div className="flex gap-2">
                         <button
                           onClick={() => handleRestore(commit)}
                           className="text-xs px-3 py-1.5 bg-white/5 hover:bg-white/10 rounded text-slate-300 font-medium transition-colors opacity-0 group-hover:opacity-100"
                         >
                           Restore
                         </button>
                       </div>
                    </div>
                 </div>
               ))
             )}
           </div>
        </div>
      </div>
    </div>
  );
}
