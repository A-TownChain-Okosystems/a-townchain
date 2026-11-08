import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import {
  FolderGit2,
  X,
  Plus,
  Trash2,
  Combine,
  Save,
  FolderOpen,
  Layers,
  GitMerge,
  Github,
} from "lucide-react";
import { GitHubSyncModal } from "./GitHubSyncModal";

interface FileState {
  name: string;
  content: string;
  iconColor: string;
  iconShape: string;
}

export interface Project {
  id: string;
  name: string;
  files: FileState[];
}

interface ProjectManagerModalProps {
  onClose: () => void;
  currentFiles: FileState[];
  onLoadProject: (files: FileState[]) => void;
}

export function ProjectManagerModal({
  onClose,
  currentFiles,
  onLoadProject,
}: ProjectManagerModalProps) {
  const [projects, setProjects] = useState<Project[]>(() => {
    try {
      const saved = localStorage.getItem("atos-saved-projects");
      return saved ? JSON.parse(saved) : [];
    } catch {
      return [];
    }
  });

  const [selectedProjects, setSelectedProjects] = useState<Set<string>>(
    new Set(),
  );
  const [newProjectName, setNewProjectName] = useState("");
  const [isGitHubModalOpen, setIsGitHubModalOpen] = useState(false);

  useEffect(() => {
    localStorage.setItem("atos-saved-projects", JSON.stringify(projects));
  }, [projects]);

  const handleSaveCurrentWorkspace = () => {
    const name = prompt(
      "Name für dieses Projekt eingeben:",
      `Projekt ${projects.length + 1}`,
    );
    if (name) {
      setProjects([
        ...projects,
        { id: Date.now().toString(), name, files: currentFiles },
      ]);
    }
  };

  const handleCreateEmpty = () => {
    if (!newProjectName.trim()) return;
    const newProject: Project = {
      id: Date.now().toString(),
      name: newProjectName,
      files: [
        {
          name: "main.lm",
          content: "// Neues leeres Projekt",
          iconColor: "text-cyan-400",
          iconShape: "◆",
        },
      ],
    };
    setProjects([...projects, newProject]);
    setNewProjectName("");
  };

  const toggleSelect = (id: string) => {
    const newSel = new Set(selectedProjects);
    if (newSel.has(id)) newSel.delete(id);
    else newSel.add(id);
    setSelectedProjects(newSel);
  };

  const handleDelete = (id: string) => {
    setProjects(projects.filter((p) => p.id !== id));
    if (selectedProjects.has(id)) {
      const newSel = new Set(selectedProjects);
      newSel.delete(id);
      setSelectedProjects(newSel);
    }
  };

  const handleFusion = () => {
    const selected = projects.filter((p) => selectedProjects.has(p.id));
    if (selected.length < 2) return;

    const name = prompt(
      "Name für das fusionierte Projekt:",
      "Fusioniertes Projekt",
    );
    if (!name) return;

    const mergedFiles: FileState[] = [];
    const existingNames = new Set<string>();

    selected.forEach((proj) => {
      proj.files.forEach((file) => {
        let finalName = file.name;
        // Avoid duplicate filenames
        let counter = 1;
        while (existingNames.has(finalName)) {
          const parts = file.name.split(".");
          if (parts.length > 1) {
            const ext = parts.pop();
            finalName = `${parts.join(".")}_${counter}.${ext}`;
          } else {
            finalName = `${file.name}_${counter}`;
          }
          counter++;
        }
        existingNames.add(finalName);
        mergedFiles.push({ ...file, name: finalName });
      });
    });

    const newProject: Project = {
      id: Date.now().toString(),
      name,
      files: mergedFiles,
    };

    setProjects([...projects, newProject]);
    setSelectedProjects(new Set());
  };

  const handleFileDrop = (e: React.DragEvent) => {
    e.preventDefault();
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      if (file.name.endsWith('.json')) {
        const reader = new FileReader();
        reader.onload = (event) => {
          try {
            const data = JSON.parse(event.target?.result as string);
            // Verify it has the structure of FileState[]
            if (Array.isArray(data) && data.length > 0 && data[0].name && typeof data[0].content === 'string') {
               onLoadProject(data);
               onClose();
            } else {
               alert("Invalid JSON format. Expected an array of files.");
            }
          } catch (err) {
            alert("Error parsing JSON file.");
          }
        };
        reader.readAsText(file);
      }
    }
  };

  return (
    <div className="fixed inset-0 z-[200] flex items-center justify-center bg-black/80 backdrop-blur-md font-sans">
      <motion.div
        initial={{ scale: 0.9, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        exit={{ scale: 0.9, opacity: 0 }}
        onDragOver={(e) => e.preventDefault()}
        onDrop={handleFileDrop}
        className="bg-[#0c0c0e] border border-white/10 rounded-xl shadow-2xl overflow-hidden max-w-4xl w-full mx-4 flex flex-col h-[80vh]"
      >
        <div className="px-6 py-4 border-b border-white/10 flex items-center justify-between bg-white/[0.02]">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-indigo-500/10 rounded-lg">
              <FolderGit2 className="w-5 h-5 text-indigo-400" />
            </div>
            <div>
              <h2 className="text-lg font-bold text-slate-100 font-sans tracking-wide">
                Software Projekte & Fusion
              </h2>
              <p className="text-xs text-slate-500">
                Erstelle, speichere und verschmelze verschiedene Projekte.
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 text-slate-400 hover:text-white hover:bg-white/10 rounded-lg transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="flex flex-1 overflow-hidden">
          {/* Main List */}
          <div className="flex-1 overflow-y-auto p-6 space-y-6">
            <div className="flex gap-2">
              <input
                value={newProjectName}
                onChange={(e) => setNewProjectName(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && handleCreateEmpty()}
                placeholder="Neues Projekt benennen..."
                className="flex-1 bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-sm text-slate-200 outline-none focus:border-indigo-500/50"
              />
              <button
                onClick={handleCreateEmpty}
                disabled={!newProjectName.trim()}
                className="flex items-center gap-2 bg-indigo-500 hover:bg-indigo-400 disabled:opacity-50 disabled:cursor-not-allowed text-white px-4 py-2 rounded-lg font-bold text-sm transition-colors"
              >
                <Plus className="w-4 h-4" /> Erstellen
              </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {projects.map((project) => (
                <div
                  key={project.id}
                  className={`p-4 rounded-xl border transition-all ${selectedProjects.has(project.id) ? "bg-indigo-500/10 border-indigo-500/50" : "bg-white/5 border-white/10 hover:border-white/30"}`}
                >
                  <div className="flex justify-between items-start mb-3">
                    <div className="flex items-center gap-3">
                      <input
                        type="checkbox"
                        checked={selectedProjects.has(project.id)}
                        onChange={() => toggleSelect(project.id)}
                        className="w-4 h-4 rounded bg-black border-white/20 text-indigo-500 focus:ring-indigo-500"
                      />
                      <h3 className="font-bold text-slate-200 text-lg">
                        {project.name}
                      </h3>
                    </div>
                    <button
                      onClick={() => handleDelete(project.id)}
                      className="text-slate-500 hover:text-red-400 transition-colors"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>

                  <p className="text-xs text-slate-500 flex items-center gap-2 mb-4">
                    <Layers className="w-3.5 h-3.5" />
                    {project.files.length} Dateien
                  </p>

                  <div className="flex gap-2">
                    <button
                      onClick={() => {
                        onLoadProject(project.files);
                        onClose();
                      }}
                      className="flex-1 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 text-xs font-bold py-2 rounded transition-colors flex items-center justify-center gap-1.5"
                    >
                      <FolderOpen className="w-3.5 h-3.5" /> Öffnen
                    </button>
                    <button
                      onClick={() => toggleSelect(project.id)}
                      className="flex-1 bg-white/5 hover:bg-white/10 text-slate-300 text-xs font-bold py-2 rounded transition-colors"
                    >
                      {selectedProjects.has(project.id)
                        ? "Abwählen"
                        : "Auswählen"}
                    </button>
                  </div>
                </div>
              ))}

              {projects.length === 0 && (
                <div className="col-span-full py-12 text-center border-2 border-dashed border-white/10 rounded-xl text-slate-500">
                  <FolderGit2 className="w-12 h-12 mx-auto mb-3 opacity-50" />
                  <p>Keine gespeicherten Projekte vorhanden.</p>
                  <p className="text-xs mt-2 opacity-75">Du kannst auch eine .json Projektdatei hierher ziehen.</p>
                </div>
              )}
            </div>
          </div>

          {/* Right Sidebar - Actions */}
          <div className="w-72 bg-black/40 border-l border-white/10 p-6 flex flex-col">
            <h3 className="text-sm font-bold text-slate-200 mb-6 uppercase tracking-wider">
              Aktionen
            </h3>

            <button
              onClick={handleSaveCurrentWorkspace}
              className="w-full flex items-center gap-3 p-4 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl transition-all mb-3 text-left group"
            >
              <div className="p-2 bg-emerald-500/20 text-emerald-400 rounded-lg group-hover:scale-110 transition-transform">
                <Save className="w-5 h-5" />
              </div>
              <div>
                <div className="font-bold text-slate-200 text-sm">
                  Workspace speichern
                </div>
                <div className="text-[10px] text-slate-500 mt-0.5">
                  Aktuelle Dateien als neues Projekt sichern
                </div>
              </div>
            </button>

            <button
              onClick={() => setIsGitHubModalOpen(true)}
              className="w-full flex items-center gap-3 p-4 bg-indigo-500/10 hover:bg-indigo-500/20 border border-indigo-500/30 rounded-xl transition-all mb-4 text-left group"
            >
              <div className="p-2 bg-slate-800 text-white rounded-lg group-hover:scale-110 transition-transform border border-white/10">
                <Github className="w-5 h-5" />
              </div>
              <div>
                <div className="font-bold text-indigo-200 text-sm flex items-center gap-1.5">
                  <span>GitHub Sync</span>
                  <span className="text-[9px] px-1.5 py-0.2 rounded bg-indigo-500/30 text-indigo-300">Live</span>
                </div>
                <div className="text-[10px] text-slate-400 mt-0.5">
                  Repo erstellen & Änderungen pushen
                </div>
              </div>
            </button>

            <div className="my-2 border-t border-white/10"></div>

            <div className="mt-4 flex-1">
              <h4 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">
                Fusionierung
              </h4>
              <p className="text-xs text-slate-400 mb-4 leading-relaxed">
                Wähle mindestens 2 Projekte aus der Liste, um ihre Dateien zu
                einem neuen großen Projekt zusammenzuführen.
              </p>

              <button
                onClick={handleFusion}
                disabled={selectedProjects.size < 2}
                className="w-full flex items-center justify-center gap-2 py-3 bg-indigo-500 hover:bg-indigo-400 disabled:opacity-30 disabled:cursor-not-allowed text-white rounded-lg font-bold transition-all shadow-lg shadow-indigo-500/20"
              >
                <GitMerge className="w-5 h-5" />
                {selectedProjects.size} Projekte fusionieren
              </button>
            </div>
          </div>
        </div>
      </motion.div>

      {isGitHubModalOpen && (
        <GitHubSyncModal
          isOpen={isGitHubModalOpen}
          onClose={() => setIsGitHubModalOpen(false)}
          files={currentFiles}
          onLoadFiles={(newFiles) => {
            onLoadProject(newFiles);
            setIsGitHubModalOpen(false);
            onClose();
          }}
        />
      )}
    </div>
  );
}
