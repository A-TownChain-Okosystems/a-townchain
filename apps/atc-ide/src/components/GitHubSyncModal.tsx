import React, { useState, useEffect } from "react";
import {
  Github,
  GitBranch,
  GitCommit,
  Plus,
  RefreshCw,
  UploadCloud,
  DownloadCloud,
  ExternalLink,
  Lock,
  Globe,
  Key,
  CheckCircle2,
  AlertCircle,
  FolderGit2,
  LogOut,
  ChevronRight,
  Sparkles,
  Info,
  Shield,
  FileCode,
  Check,
  X
} from "lucide-react";
import { FileState } from "../App";
import { GitHubRepo, GitHubUser, GitHubCommitItem } from "../types/github";
import {
  getStoredGitHubAuth,
  saveGitHubAuth,
  clearGitHubAuth,
  fetchGitHubUser,
  fetchUserRepos,
  createGitHubRepo,
  fetchRepoCommits,
  pushToGitHubRepo,
  pullFromGitHubRepo,
  GitHubAuthState
} from "../utils/githubService";

interface GitHubSyncModalProps {
  isOpen: boolean;
  onClose: () => void;
  files: FileState[];
  onLoadFiles: (files: FileState[]) => void;
  projectName?: string;
  onOpenCiCd?: () => void;
}

export function GitHubSyncModal({
  isOpen,
  onClose,
  files,
  onLoadFiles,
  projectName = "atos-lumino-project",
  onOpenCiCd,
}: GitHubSyncModalProps) {
  const [auth, setAuth] = useState<GitHubAuthState>(getStoredGitHubAuth());
  const [patInput, setPatInput] = useState("");
  const [isAuthenticating, setIsAuthenticating] = useState(false);
  const [authError, setAuthError] = useState<string | null>(null);

  // Repositories state
  const [repos, setRepos] = useState<GitHubRepo[]>([]);
  const [selectedRepo, setSelectedRepo] = useState<GitHubRepo | null>(null);
  const [isLoadingRepos, setIsLoadingRepos] = useState(false);
  const [repoSearch, setRepoSearch] = useState("");

  // Create repo state
  const [activeTab, setActiveTab] = useState<"push" | "create" | "pull" | "commits" | "settings">("push");
  const [newRepoName, setNewRepoName] = useState(() =>
    projectName.toLowerCase().replace(/[^a-z0-9-_]/g, "-") || "my-kernel-os"
  );
  const [newRepoDesc, setNewRepoDesc] = useState("Bare Metal OS / Lumino Project created with ATOS IDE");
  const [isPrivateRepo, setIsPrivateRepo] = useState(false);
  const [isCreatingRepo, setIsCreatingRepo] = useState(false);

  // Push / Commit state
  const [commitMessage, setCommitMessage] = useState("");
  const [targetBranch, setTargetBranch] = useState("main");
  const [isPushing, setIsPushing] = useState(false);
  const [pushStatus, setPushStatus] = useState<{ success: boolean; message: string; url?: string } | null>(null);

  // Pull / Commits state
  const [commits, setCommits] = useState<GitHubCommitItem[]>([]);
  const [isLoadingCommits, setIsLoadingCommits] = useState(false);
  const [isPulling, setIsPulling] = useState(false);
  const [pullStatus, setPullStatus] = useState<string | null>(null);

  // Listen for OAuth Popup PostMessage
  useEffect(() => {
    const handleOAuthMessage = async (event: MessageEvent) => {
      if (event.data?.type === "GITHUB_AUTH_CALLBACK") {
        const { code, error } = event.data;
        if (error) {
          setAuthError(`OAuth Authorization error: ${error}`);
          setIsAuthenticating(false);
          return;
        }

        if (code) {
          setIsAuthenticating(true);
          try {
            const tokenRes = await fetch("/api/github/token", {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({ code }),
            });

            const tokenData = await tokenRes.json();
            if (!tokenRes.ok || !tokenData.access_token) {
              throw new Error(tokenData.error || "Failed to retrieve access token");
            }

            const token = tokenData.access_token;
            const user = await fetchGitHubUser(token);
            saveGitHubAuth(token, user, "oauth");
            setAuth({ token, user, authMethod: "oauth" });
            setAuthError(null);
            loadRepositories(token);
          } catch (err: any) {
            setAuthError(err.message || "Failed to finish GitHub authentication");
          } finally {
            setIsAuthenticating(false);
          }
        }
      }
    };

    window.addEventListener("message", handleOAuthMessage);
    return () => window.removeEventListener("message", handleOAuthMessage);
  }, []);

  // Load repos when authenticated
  useEffect(() => {
    if (auth.token && isOpen) {
      loadRepositories(auth.token);
    }
  }, [auth.token, isOpen]);

  // Load commits when selected repo changes
  useEffect(() => {
    if (auth.token && selectedRepo && activeTab === "commits") {
      loadCommits();
    }
  }, [selectedRepo, activeTab]);

  const loadRepositories = async (token: string) => {
    setIsLoadingRepos(true);
    setAuthError(null);
    try {
      const list = await fetchUserRepos(token);
      setRepos(list);
      if (list.length > 0 && !selectedRepo) {
        // Auto select repo matching project name or first repo
        const match = list.find((r) => r.name.toLowerCase() === newRepoName.toLowerCase());
        setSelectedRepo(match || list[0]);
      }
    } catch (err: any) {
      setAuthError(err.message || "Failed to load repositories");
      if (err.message?.includes("401")) {
        clearGitHubAuth();
        setAuth({ token: null, user: null, authMethod: null });
      }
    } finally {
      setIsLoadingRepos(false);
    }
  };

  const loadCommits = async () => {
    if (!auth.token || !selectedRepo) return;
    setIsLoadingCommits(true);
    try {
      const [owner, name] = selectedRepo.full_name.split("/");
      const list = await fetchRepoCommits(auth.token, owner, name);
      setCommits(list);
    } catch (err: any) {
      console.error("Failed to load commits", err);
    } finally {
      setIsLoadingCommits(false);
    }
  };

  const handleOAuthLogin = async () => {
    setIsAuthenticating(true);
    setAuthError(null);
    try {
      const res = await fetch("/api/github/auth-url");
      const data = await res.json();
      if (!res.ok || !data.url) {
        throw new Error(
          data.error ||
          "GitHub Client ID ist nicht in den Umgebungsvariablen konfiguriert. Bitte verwende alternativ ein Personal Access Token (PAT)."
        );
      }

      // Open OAuth popup window directly to GitHub
      const popup = window.open(data.url, "github_oauth", "width=600,height=750");
      if (!popup) {
        setAuthError("Popup blockiert. Bitte Popups im Browser für diese Seite erlauben.");
        setIsAuthenticating(false);
      }
    } catch (err: any) {
      setAuthError(err.message || "OAuth Start fehlgeschlagen");
      setIsAuthenticating(false);
    }
  };

  const handlePatLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!patInput.trim()) return;

    setIsAuthenticating(true);
    setAuthError(null);
    try {
      const token = patInput.trim();
      const user = await fetchGitHubUser(token);
      saveGitHubAuth(token, user, "token");
      setAuth({ token, user, authMethod: "token" });
      setPatInput("");
      loadRepositories(token);
    } catch (err: any) {
      setAuthError(err.message || "Ungültiges Personal Access Token (PAT). Bitte Rechte ('repo') prüfen.");
    } finally {
      setIsAuthenticating(false);
    }
  };

  const handleLogout = () => {
    clearGitHubAuth();
    setAuth({ token: null, user: null, authMethod: null });
    setRepos([]);
    setSelectedRepo(null);
    setCommits([]);
  };

  const handleCreateNewRepo = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!auth.token || !newRepoName.trim()) return;

    setIsCreatingRepo(true);
    setPushStatus(null);
    try {
      const created = await createGitHubRepo(auth.token, {
        name: newRepoName.trim(),
        description: newRepoDesc.trim(),
        isPrivate: isPrivateRepo,
        autoInit: true,
      });

      // Update repo list & select
      setRepos([created, ...repos]);
      setSelectedRepo(created);
      setActiveTab("push");
      setPushStatus({
        success: true,
        message: `Repository "${created.full_name}" erfolgreich erstellt! Du kannst jetzt die Dateien hochladen.`,
        url: created.html_url,
      });
    } catch (err: any) {
      setPushStatus({
        success: false,
        message: err.message || "Fehler beim Erstellen des Repositories",
      });
    } finally {
      setIsCreatingRepo(false);
    }
  };

  const handlePushFiles = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!auth.token || !selectedRepo) return;

    const defaultMsg = `Update ${files.length} project files (${new Date().toLocaleTimeString()})`;
    const msg = commitMessage.trim() || defaultMsg;

    setIsPushing(true);
    setPushStatus(null);

    try {
      const [owner, name] = selectedRepo.full_name.split("/");
      const payloadFiles = files.map((f) => ({
        path: f.name,
        content: f.content,
      }));

      const result = await pushToGitHubRepo(auth.token, {
        owner,
        repo: name,
        branch: targetBranch || selectedRepo.default_branch || "main",
        message: msg,
        files: payloadFiles,
      });

      setPushStatus({
        success: true,
        message: `Erfolgreich ${files.length} Dateien als Commit zu "${selectedRepo.full_name}@${result.branch}" übertragen!`,
        url: result.url,
      });
      setCommitMessage("");
      loadCommits();
    } catch (err: any) {
      setPushStatus({
        success: false,
        message: err.message || "Fehler beim Hochladen der Änderungen",
      });
    } finally {
      setIsPushing(false);
    }
  };

  const handlePullRepoFiles = async () => {
    if (!auth.token || !selectedRepo) return;

    if (!window.confirm(`Möchtest du die Dateien aus dem Repository "${selectedRepo.full_name}" in den aktuellen Workspace laden? Ungespeicherte lokale Änderungen werden ersetzt.`)) {
      return;
    }

    setIsPulling(true);
    setPullStatus(null);
    try {
      const [owner, name] = selectedRepo.full_name.split("/");
      const result = await pullFromGitHubRepo(
        auth.token,
        owner,
        name,
        selectedRepo.default_branch || "main"
      );

      if (result.files.length === 0) {
        setPullStatus("Keine Textdateien im ausgewählten Branch gefunden.");
        return;
      }

      // Convert to FileState format
      const convertedFiles: FileState[] = result.files.map((f) => {
        let iconColor = "text-cyan-400";
        let iconShape = "◆";

        if (f.name.endsWith(".wiki") || f.name.endsWith(".md")) {
          iconColor = "text-yellow-400";
          iconShape = "📝";
        } else if (f.name.endsWith(".ts") || f.name.endsWith(".tsx")) {
          iconColor = "text-blue-400";
          iconShape = "⚡";
        } else if (f.name.endsWith(".rs")) {
          iconColor = "text-orange-400";
          iconShape = "🦀";
        } else if (f.name.endsWith(".c") || f.name.endsWith(".h")) {
          iconColor = "text-indigo-400";
          iconShape = "⚙";
        } else if (f.name.endsWith(".asm") || f.name.endsWith(".s")) {
          iconColor = "text-emerald-400";
          iconShape = "⚡";
        }

        return {
          name: f.name,
          content: f.content,
          iconColor,
          iconShape,
        };
      });

      onLoadFiles(convertedFiles);
      setPullStatus(`Erfolgreich ${convertedFiles.length} Dateien aus GitHub importiert!`);
    } catch (err: any) {
      setPullStatus(`Fehler beim Laden: ${err.message}`);
    } finally {
      setIsPulling(false);
    }
  };

  if (!isOpen) return null;

  const filteredRepos = repos.filter(
    (r) =>
      r.name.toLowerCase().includes(repoSearch.toLowerCase()) ||
      r.full_name.toLowerCase().includes(repoSearch.toLowerCase())
  );

  return (
    <div className="fixed inset-0 z-[200] flex items-center justify-center bg-black/85 backdrop-blur-md p-3 sm:p-5 font-sans">
      <div className="bg-slate-900 border border-indigo-500/30 rounded-2xl w-full max-w-4xl max-h-[90vh] flex flex-col shadow-2xl overflow-hidden text-slate-100">
        {/* Header */}
        <div className="px-5 py-4 border-b border-white/10 flex items-center justify-between bg-slate-950/80 shrink-0">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-xl bg-slate-800 border border-white/10 text-white flex items-center justify-center shadow-inner">
              <Github className="w-6 h-6" />
            </div>
            <div>
              <h2 className="text-base sm:text-lg font-bold text-white flex items-center gap-2">
                <span>GitHub Repository Manager</span>
                <span className="px-2 py-0.5 rounded-full text-[10px] font-mono font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30">
                  Live Sync
                </span>
              </h2>
              <p className="text-xs text-slate-400">
                Repositories anlegen, Commits pushen und Änderungen direkt mit GitHub synchronisieren
              </p>
            </div>
          </div>

          <button
            onClick={onClose}
            className="p-1.5 rounded-lg text-slate-400 hover:text-white hover:bg-white/10 transition-colors cursor-pointer"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content Body */}
        {!auth.token ? (
          /* Authentication Screen */
          <div className="flex-1 overflow-y-auto p-6 flex flex-col items-center justify-center max-w-lg mx-auto text-center space-y-6">
            <div className="p-4 bg-indigo-500/10 rounded-2xl border border-indigo-500/20 text-indigo-400">
              <FolderGit2 className="w-12 h-12" />
            </div>

            <div>
              <h3 className="text-lg font-bold text-white">Mit GitHub verbinden</h3>
              <p className="text-xs text-slate-400 mt-1 leading-relaxed">
                Verbinde deinen GitHub-Account, um Repositories für deine Bare-Metal-Kernel, Betriebssysteme und Softwareprojekte direkt aus der ATOS IDE zu verwalten.
              </p>
            </div>

            {authError && (
              <div className="w-full p-3 rounded-xl bg-red-950/50 border border-red-500/30 text-red-300 text-xs text-left flex items-start gap-2">
                <AlertCircle className="w-4 h-4 text-red-400 shrink-0 mt-0.5" />
                <span>{authError}</span>
              </div>
            )}

            {/* Option 1: OAuth */}
            <div className="w-full space-y-3">
              <button
                onClick={handleOAuthLogin}
                disabled={isAuthenticating}
                className="w-full py-3 bg-white hover:bg-slate-100 text-slate-900 rounded-xl font-bold text-sm flex items-center justify-center gap-2.5 transition-all shadow-lg hover:shadow-xl cursor-pointer disabled:opacity-50"
              >
                {isAuthenticating ? (
                  <RefreshCw className="w-4 h-4 animate-spin text-slate-900" />
                ) : (
                  <Github className="w-4 h-4 text-slate-900" />
                )}
                <span>1-Click Login mit GitHub (OAuth)</span>
              </button>

              <div className="flex items-center gap-3 text-xs text-slate-500 my-2">
                <div className="h-px bg-white/10 flex-1"></div>
                <span>ODER MIT PERSONAL ACCESS TOKEN</span>
                <div className="h-px bg-white/10 flex-1"></div>
              </div>

              {/* Option 2: Personal Access Token */}
              <form onSubmit={handlePatLogin} className="space-y-2 text-left">
                <div className="relative">
                  <Key className="w-4 h-4 text-slate-400 absolute left-3 top-3" />
                  <input
                    type="password"
                    value={patInput}
                    onChange={(e) => setPatInput(e.target.value)}
                    placeholder="ghp_xxxxxxxxxxxxxxxxxx (Classic Token)"
                    className="w-full bg-slate-950/80 border border-white/10 rounded-xl pl-9 pr-3 py-2.5 text-xs text-slate-200 outline-none focus:border-indigo-500/60 font-mono"
                  />
                </div>
                <button
                  type="submit"
                  disabled={!patInput.trim() || isAuthenticating}
                  className="w-full py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl text-xs font-semibold flex items-center justify-center gap-1.5 transition-colors cursor-pointer disabled:opacity-50"
                >
                  <Key className="w-3.5 h-3.5" />
                  <span>Mit Token verbinden</span>
                </button>
              </form>

              <div className="text-[11px] text-slate-500 text-left bg-slate-950/40 p-3 rounded-xl border border-white/5 space-y-1">
                <div className="font-semibold text-slate-400 flex items-center gap-1">
                  <Info className="w-3.5 h-3.5 text-indigo-400" />
                  <span>So erstellst du ein GitHub Token:</span>
                </div>
                <p>
                  1. Gehe zu <a href="https://github.com/settings/tokens" target="_blank" rel="noreferrer" className="text-indigo-400 hover:underline">github.com/settings/tokens</a> (Tokens classic).
                </p>
                <p>2. Setze den Haken bei <strong className="text-slate-300">repo</strong> (Full control of private repositories) und klicke auf "Generate token".</p>
              </div>
            </div>
          </div>
        ) : (
          /* Authenticated Dashboard */
          <div className="flex-1 flex flex-col md:flex-row overflow-hidden">
            {/* Left Column: Repository Picker */}
            <div className="w-full md:w-80 border-r border-white/10 bg-slate-950/60 flex flex-col shrink-0">
              {/* User Bar */}
              <div className="p-3.5 border-b border-white/5 flex items-center justify-between">
                <div className="flex items-center gap-2.5 min-w-0">
                  {auth.user?.avatar_url ? (
                    <img
                      src={auth.user.avatar_url}
                      alt={auth.user.login}
                      className="w-8 h-8 rounded-full border border-indigo-500/40"
                    />
                  ) : (
                    <div className="w-8 h-8 rounded-full bg-indigo-600 text-white font-bold text-xs flex items-center justify-center">
                      GH
                    </div>
                  )}
                  <div className="min-w-0">
                    <div className="text-xs font-bold text-slate-200 truncate flex items-center gap-1">
                      <span>{auth.user?.name || auth.user?.login}</span>
                    </div>
                    <div className="text-[10px] text-slate-400 truncate font-mono">
                      @{auth.user?.login}
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-1">
                  <button
                    onClick={() => auth.token && loadRepositories(auth.token)}
                    disabled={isLoadingRepos}
                    className="p-1.5 text-slate-400 hover:text-white rounded-md hover:bg-white/5 transition-colors cursor-pointer"
                    title="Repositories aktualisieren"
                  >
                    <RefreshCw className={`w-3.5 h-3.5 ${isLoadingRepos ? "animate-spin text-indigo-400" : ""}`} />
                  </button>
                  <button
                    onClick={handleLogout}
                    className="p-1.5 text-slate-400 hover:text-red-400 rounded-md hover:bg-white/5 transition-colors cursor-pointer"
                    title="Abmelden"
                  >
                    <LogOut className="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>

              {/* Search & New Button */}
              <div className="p-2.5 border-b border-white/5 space-y-2">
                <div className="flex items-center gap-1.5">
                  <input
                    type="text"
                    value={repoSearch}
                    onChange={(e) => setRepoSearch(e.target.value)}
                    placeholder="Repository suchen..."
                    className="flex-1 bg-slate-900 border border-white/10 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 outline-none focus:border-indigo-500/50"
                  />
                  <button
                    onClick={() => setActiveTab("create")}
                    className="p-1.5 px-2.5 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-xs font-semibold flex items-center gap-1 transition-colors shrink-0 cursor-pointer shadow-xs"
                    title="Neues Repository auf GitHub erstellen"
                  >
                    <Plus className="w-3.5 h-3.5" />
                    <span>Neu</span>
                  </button>
                </div>
              </div>

              {/* Repos List */}
              <div className="flex-1 overflow-y-auto p-2 space-y-1">
                {isLoadingRepos && repos.length === 0 ? (
                  <div className="text-center py-8 text-slate-500 text-xs flex flex-col items-center gap-2">
                    <RefreshCw className="w-5 h-5 animate-spin text-indigo-400" />
                    <span>Repositories laden...</span>
                  </div>
                ) : filteredRepos.length === 0 ? (
                  <div className="text-center py-8 text-slate-500 text-xs">
                    Keine Repositories gefunden
                  </div>
                ) : (
                  filteredRepos.map((repo) => {
                    const isSelected = selectedRepo?.id === repo.id;
                    return (
                      <div
                        key={repo.id}
                        onClick={() => {
                          setSelectedRepo(repo);
                          setPushStatus(null);
                        }}
                        className={`p-2.5 rounded-xl border transition-all cursor-pointer select-none text-left ${
                          isSelected
                            ? "bg-indigo-950/50 border-indigo-500/60 ring-1 ring-indigo-500/40 text-white"
                            : "bg-slate-900/40 border-white/5 hover:bg-slate-900/80 hover:border-white/10 text-slate-300"
                        }`}
                      >
                        <div className="flex items-center justify-between gap-1 mb-1">
                          <span className="text-xs font-bold truncate flex items-center gap-1.5">
                            {repo.private ? (
                              <Lock className="w-3 h-3 text-amber-400 shrink-0" />
                            ) : (
                              <Globe className="w-3 h-3 text-cyan-400 shrink-0" />
                            )}
                            <span className="truncate">{repo.name}</span>
                          </span>
                          <span className="text-[9px] font-mono text-slate-400 px-1.5 py-0.2 rounded bg-white/5">
                            {repo.default_branch}
                          </span>
                        </div>
                        {repo.description && (
                          <p className="text-[10px] text-slate-400 line-clamp-1">
                            {repo.description}
                          </p>
                        )}
                      </div>
                    );
                  })
                )}
              </div>
            </div>

            {/* Right Column: Active Action & Details */}
            <div className="flex-1 flex flex-col bg-slate-900/80 overflow-hidden">
              {/* Navigation Tabs */}
              <div className="px-4 pt-3 pb-2 border-b border-white/10 flex items-center justify-between gap-2 bg-slate-950/40 shrink-0 overflow-x-auto">
                <div className="flex items-center bg-slate-950/80 p-0.5 rounded-lg border border-white/5 text-xs">
                  <button
                    onClick={() => setActiveTab("push")}
                    className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md font-medium transition-all cursor-pointer ${
                      activeTab === "push"
                        ? "bg-indigo-600 text-white shadow-xs"
                        : "text-slate-400 hover:text-slate-200"
                    }`}
                  >
                    <UploadCloud className="w-3.5 h-3.5" />
                    <span>Änderungen pushen</span>
                  </button>

                  <button
                    onClick={() => setActiveTab("create")}
                    className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md font-medium transition-all cursor-pointer ${
                      activeTab === "create"
                        ? "bg-indigo-600 text-white shadow-xs"
                        : "text-slate-400 hover:text-slate-200"
                    }`}
                  >
                    <Plus className="w-3.5 h-3.5" />
                    <span>Neues Repo</span>
                  </button>

                  <button
                    onClick={() => setActiveTab("pull")}
                    className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md font-medium transition-all cursor-pointer ${
                      activeTab === "pull"
                        ? "bg-indigo-600 text-white shadow-xs"
                        : "text-slate-400 hover:text-slate-200"
                    }`}
                  >
                    <DownloadCloud className="w-3.5 h-3.5" />
                    <span>Von GitHub laden</span>
                  </button>

                  <button
                    onClick={() => setActiveTab("commits")}
                    className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md font-medium transition-all cursor-pointer ${
                      activeTab === "commits"
                        ? "bg-indigo-600 text-white shadow-xs"
                        : "text-slate-400 hover:text-slate-200"
                    }`}
                  >
                    <GitCommit className="w-3.5 h-3.5" />
                    <span>Historie</span>
                  </button>
                </div>

                {selectedRepo && (
                  <a
                    href={selectedRepo.html_url}
                    target="_blank"
                    rel="noreferrer"
                    className="text-[11px] text-indigo-400 hover:text-indigo-300 flex items-center gap-1 font-mono transition-colors shrink-0"
                  >
                    <span>{selectedRepo.name}</span>
                    <ExternalLink className="w-3 h-3" />
                  </a>
                )}
              </div>

              {/* Tab Viewport */}
              <div className="flex-1 overflow-y-auto p-4 sm:p-6 space-y-4">
                {activeTab === "push" && (
                  <div className="space-y-4 max-w-xl">
                    {selectedRepo ? (
                      <>
                        <div className="p-3.5 rounded-xl bg-slate-950/60 border border-white/5 space-y-1">
                          <div className="text-xs font-bold text-slate-200 flex items-center gap-1.5">
                            <FolderGit2 className="w-4 h-4 text-emerald-400" />
                            <span>Ziel-Repository: {selectedRepo.full_name}</span>
                          </div>
                          <div className="text-[11px] text-slate-400 flex items-center gap-3 pt-1">
                            <span>Branch: <code className="text-indigo-300 font-mono">{selectedRepo.default_branch || "main"}</code></span>
                            <span>•</span>
                            <span>Dateien im Workspace: <strong className="text-white">{files.length}</strong></span>
                          </div>
                        </div>

                        {/* Files to commit preview */}
                        <div className="space-y-1.5">
                          <label className="text-[11px] font-bold text-slate-400 uppercase tracking-wider block">
                            Enthaltene Dateien ({files.length}):
                          </label>
                          <div className="max-h-36 overflow-y-auto p-2 rounded-lg bg-black/40 border border-white/5 space-y-1 font-mono text-[11px]">
                            {files.map((f, i) => (
                              <div key={i} className="flex items-center justify-between text-slate-300">
                                <span className="flex items-center gap-1.5">
                                  <FileCode className="w-3 h-3 text-cyan-400" />
                                  <span>{f.name}</span>
                                </span>
                                <span className="text-[10px] text-slate-500">{f.content.length} B</span>
                              </div>
                            ))}
                          </div>
                        </div>

                        {/* CI/CD Workflow Status for Push */}
                        {files.some((f) => f.name === ".github/workflows/lumino-build.yml") ? (
                          <div className="p-2.5 rounded-lg bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-between text-xs text-emerald-300">
                            <div className="flex items-center gap-2">
                              <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
                              <span>
                                <strong>CI/CD aktiv:</strong> <code className="font-mono text-[11px] bg-black/30 px-1 py-0.5 rounded">.github/workflows/lumino-build.yml</code> wird mit gepusht. Unit-Tests laufen automatisch auf GitHub Actions!
                              </span>
                            </div>
                            {onOpenCiCd && (
                              <button
                                type="button"
                                onClick={onOpenCiCd}
                                className="px-2 py-0.5 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-200 rounded text-[11px] font-semibold transition-colors cursor-pointer shrink-0 ml-2"
                              >
                                Bearbeiten
                              </button>
                            )}
                          </div>
                        ) : (
                          <div className="p-2.5 rounded-lg bg-indigo-500/10 border border-indigo-500/20 flex items-center justify-between text-xs text-indigo-300">
                            <div className="flex items-center gap-2">
                              <Sparkles className="w-4 h-4 text-indigo-400 shrink-0" />
                              <span>
                                <strong>Automatisierte Tests beim Push:</strong> .github/workflows/lumino-build.yml fehlt noch.
                              </span>
                            </div>
                            {onOpenCiCd && (
                              <button
                                type="button"
                                onClick={onOpenCiCd}
                                className="px-2.5 py-1 bg-indigo-600 hover:bg-indigo-500 text-white rounded-md text-[11px] font-bold shadow transition-colors cursor-pointer shrink-0 ml-2"
                              >
                                + Workflow generieren
                              </button>
                            )}
                          </div>
                        )}

                        {/* Commit Message & Push Form */}
                        <form onSubmit={handlePushFiles} className="space-y-3">
                          <div>
                            <label className="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
                              Commit Nachricht
                            </label>
                            <input
                              type="text"
                              value={commitMessage}
                              onChange={(e) => setCommitMessage(e.target.value)}
                              placeholder={`z. B. feat(kernel): Paging & IDT Exception Handling implementiert`}
                              className="w-full bg-slate-950 border border-white/10 rounded-xl px-3 py-2 text-xs text-slate-100 outline-none focus:border-indigo-500/60"
                            />
                          </div>

                          <div className="flex items-center gap-2">
                            <div className="flex-1">
                              <label className="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
                                Branch
                              </label>
                              <input
                                type="text"
                                value={targetBranch}
                                onChange={(e) => setTargetBranch(e.target.value)}
                                placeholder="main"
                                className="w-full bg-slate-950 border border-white/10 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 outline-none font-mono"
                              />
                            </div>
                            <div className="pt-4 flex-1">
                              <button
                                type="submit"
                                disabled={isPushing || files.length === 0}
                                className="w-full py-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white rounded-lg text-xs font-bold flex items-center justify-center gap-2 transition-all cursor-pointer shadow-lg shadow-emerald-900/30"
                              >
                                {isPushing ? (
                                  <RefreshCw className="w-4 h-4 animate-spin text-white" />
                                ) : (
                                  <UploadCloud className="w-4 h-4" />
                                )}
                                <span>Änderungen hochladen (Push)</span>
                              </button>
                            </div>
                          </div>
                        </form>

                        {/* Push Status feedback */}
                        {pushStatus && (
                          <div
                            className={`p-3.5 rounded-xl border text-xs leading-relaxed flex items-start gap-2.5 ${
                              pushStatus.success
                                ? "bg-emerald-950/40 border-emerald-500/30 text-emerald-200"
                                : "bg-red-950/40 border-red-500/30 text-red-200"
                            }`}
                          >
                            {pushStatus.success ? (
                              <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0 mt-0.5" />
                            ) : (
                              <AlertCircle className="w-4 h-4 text-red-400 shrink-0 mt-0.5" />
                            )}
                            <div className="flex-1 min-w-0">
                              <p>{pushStatus.message}</p>
                              {pushStatus.url && (
                                <a
                                  href={pushStatus.url}
                                  target="_blank"
                                  rel="noreferrer"
                                  className="text-emerald-400 hover:underline font-mono text-[11px] mt-1 inline-flex items-center gap-1 font-semibold"
                                >
                                  <span>Auf GitHub ansehen</span>
                                  <ExternalLink className="w-3 h-3" />
                                </a>
                              )}
                            </div>
                          </div>
                        )}
                      </>
                    ) : (
                      <div className="p-8 text-center bg-slate-950/40 rounded-xl border border-white/5 space-y-3">
                        <FolderGit2 className="w-10 h-10 mx-auto text-slate-500" />
                        <h4 className="text-sm font-bold text-slate-300">Kein Repository ausgewählt</h4>
                        <p className="text-xs text-slate-400 max-w-sm mx-auto">
                          Wähle links ein bestehendes Repository aus oder erstelle über den Reiter "Neues Repo" ein frisches GitHub-Projekt.
                        </p>
                        <button
                          onClick={() => setActiveTab("create")}
                          className="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-lg text-xs font-semibold inline-flex items-center gap-1.5 transition-colors cursor-pointer"
                        >
                          <Plus className="w-3.5 h-3.5" />
                          <span>Jetzt neues Repository anlegen</span>
                        </button>
                      </div>
                    )}
                  </div>
                )}

                {activeTab === "create" && (
                  <form onSubmit={handleCreateNewRepo} className="space-y-4 max-w-lg">
                    <div>
                      <h3 className="text-sm font-bold text-white mb-1">Neues GitHub Repository erstellen</h3>
                      <p className="text-xs text-slate-400">
                        Erstellt ein neues Repository unter deinem Account <strong>@{auth.user?.login}</strong> und initialisiert es direkt.
                      </p>
                    </div>

                    <div>
                      <label className="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
                        Repository Name <span className="text-red-400">*</span>
                      </label>
                      <input
                        type="text"
                        required
                        value={newRepoName}
                        onChange={(e) => setNewRepoName(e.target.value)}
                        placeholder="z. B. my-x86-kernel"
                        className="w-full bg-slate-950 border border-white/10 rounded-xl px-3 py-2 text-xs text-slate-100 outline-none focus:border-indigo-500/60 font-mono"
                      />
                    </div>

                    <div>
                      <label className="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
                        Beschreibung (optional)
                      </label>
                      <textarea
                        value={newRepoDesc}
                        onChange={(e) => setNewRepoDesc(e.target.value)}
                        rows={2}
                        placeholder="Kurze Projektbeschreibung..."
                        className="w-full bg-slate-950 border border-white/10 rounded-xl px-3 py-2 text-xs text-slate-100 outline-none focus:border-indigo-500/60 resize-none"
                      />
                    </div>

                    <div className="flex items-center gap-3 p-3 rounded-xl bg-slate-950/60 border border-white/5">
                      <input
                        type="checkbox"
                        id="private-repo-toggle"
                        checked={isPrivateRepo}
                        onChange={(e) => setIsPrivateRepo(e.target.checked)}
                        className="w-4 h-4 rounded bg-black border-white/20 text-indigo-600 focus:ring-indigo-500 cursor-pointer"
                      />
                      <label htmlFor="private-repo-toggle" className="text-xs text-slate-300 cursor-pointer select-none">
                        <div className="font-semibold text-white flex items-center gap-1.5">
                          {isPrivateRepo ? <Lock className="w-3.5 h-3.5 text-amber-400" /> : <Globe className="w-3.5 h-3.5 text-cyan-400" />}
                          <span>{isPrivateRepo ? "Privates Repository" : "Öffentliches Repository (Public)"}</span>
                        </div>
                        <div className="text-[10px] text-slate-400">
                          {isPrivateRepo ? "Nur du hast Zugriff auf den Quellcode." : "Jeder auf GitHub kann das Projekt einsehen."}
                        </div>
                      </label>
                    </div>

                    <button
                      type="submit"
                      disabled={isCreatingRepo || !newRepoName.trim()}
                      className="w-full py-2.5 bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white rounded-xl text-xs font-bold flex items-center justify-center gap-2 transition-all cursor-pointer shadow-md shadow-indigo-900/30"
                    >
                      {isCreatingRepo ? (
                        <RefreshCw className="w-4 h-4 animate-spin text-white" />
                      ) : (
                        <Plus className="w-4 h-4" />
                      )}
                      <span>Repository auf GitHub erstellen</span>
                    </button>
                  </form>
                )}

                {activeTab === "pull" && (
                  <div className="space-y-4 max-w-lg">
                    <div>
                      <h3 className="text-sm font-bold text-white mb-1">Dateien von GitHub importieren</h3>
                      <p className="text-xs text-slate-400">
                        Lädt alle Quellcodedateien aus dem ausgewählten Repository herunter und öffnet sie in der ATOS IDE.
                      </p>
                    </div>

                    {selectedRepo ? (
                      <div className="p-4 rounded-xl bg-slate-950/60 border border-white/5 space-y-3">
                        <div className="flex items-center justify-between">
                          <div className="text-xs font-bold text-white flex items-center gap-1.5">
                            <FolderGit2 className="w-4 h-4 text-cyan-400" />
                            <span>{selectedRepo.full_name}</span>
                          </div>
                          <span className="text-[10px] font-mono text-indigo-300 bg-indigo-500/10 px-2 py-0.5 rounded border border-indigo-500/20">
                            Branch: {selectedRepo.default_branch || "main"}
                          </span>
                        </div>

                        <p className="text-xs text-slate-400">
                          {selectedRepo.description || "Keine Beschreibung hinterlegt"}
                        </p>

                        <button
                          onClick={handlePullRepoFiles}
                          disabled={isPulling}
                          className="w-full py-2.5 bg-cyan-600 hover:bg-cyan-500 disabled:opacity-50 text-white rounded-xl text-xs font-bold flex items-center justify-center gap-2 transition-all cursor-pointer shadow-md shadow-cyan-900/30"
                        >
                          {isPulling ? (
                            <RefreshCw className="w-4 h-4 animate-spin text-white" />
                          ) : (
                            <DownloadCloud className="w-4 h-4" />
                          )}
                          <span>Dateien in Editor laden (Pull)</span>
                        </button>
                      </div>
                    ) : (
                      <div className="text-xs text-slate-500 italic p-4 text-center">
                        Bitte wähle links ein Repository aus.
                      </div>
                    )}

                    {pullStatus && (
                      <div className="p-3 rounded-xl bg-slate-950 border border-white/10 text-xs text-slate-300 flex items-start gap-2">
                        <Info className="w-4 h-4 text-cyan-400 shrink-0 mt-0.5" />
                        <span>{pullStatus}</span>
                      </div>
                    )}
                  </div>
                )}

                {activeTab === "commits" && (
                  <div className="space-y-3">
                    <div className="flex items-center justify-between">
                      <h3 className="text-sm font-bold text-white flex items-center gap-1.5">
                        <GitCommit className="w-4 h-4 text-indigo-400" />
                        <span>Commit-Historie für {selectedRepo?.name || "Projekt"}</span>
                      </h3>
                      <button
                        onClick={loadCommits}
                        disabled={isLoadingCommits}
                        className="text-xs text-slate-400 hover:text-white flex items-center gap-1 cursor-pointer"
                      >
                        <RefreshCw className={`w-3 h-3 ${isLoadingCommits ? "animate-spin text-indigo-400" : ""}`} />
                        <span>Neu laden</span>
                      </button>
                    </div>

                    <div className="space-y-2 max-h-96 overflow-y-auto pr-1">
                      {isLoadingCommits ? (
                        <div className="text-center py-8 text-slate-500 text-xs">
                          Commits laden...
                        </div>
                      ) : commits.length === 0 ? (
                        <div className="text-center py-8 text-slate-500 text-xs">
                          Keine Commits im Repository vorhanden.
                        </div>
                      ) : (
                        commits.map((c) => (
                          <div
                            key={c.sha}
                            className="p-3 rounded-xl bg-slate-950/60 border border-white/5 hover:border-white/10 flex items-start justify-between gap-3 transition-colors"
                          >
                            <div className="min-w-0">
                              <div className="text-xs font-semibold text-slate-200 line-clamp-2">
                                {c.commit.message}
                              </div>
                              <div className="flex items-center gap-2 mt-1 text-[10px] text-slate-400">
                                <span>{c.commit.author.name}</span>
                                <span>•</span>
                                <span>{new Date(c.commit.author.date).toLocaleString()}</span>
                              </div>
                            </div>
                            <div className="flex items-center gap-1.5 shrink-0">
                              <code className="text-[10px] font-mono bg-white/5 px-1.5 py-0.5 rounded text-indigo-300 border border-white/5">
                                {c.sha.substring(0, 7)}
                              </code>
                              <a
                                href={c.html_url}
                                target="_blank"
                                rel="noreferrer"
                                className="p-1 text-slate-400 hover:text-white"
                                title="Auf GitHub öffnen"
                              >
                                <ExternalLink className="w-3.5 h-3.5" />
                              </a>
                            </div>
                          </div>
                        ))
                      )}
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        )}

        {/* Modal Footer */}
        <div className="px-5 py-3 border-t border-white/10 bg-slate-950/80 flex items-center justify-between text-xs text-slate-400 shrink-0">
          <div className="flex items-center gap-2">
            <Shield className="w-3.5 h-3.5 text-emerald-400" />
            <span className="text-[11px]">Sichere HTTPS GitHub REST & Git Data API Verbindung</span>
          </div>
          <button
            onClick={onClose}
            className="px-4 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-lg font-medium transition-colors cursor-pointer"
          >
            Schließen
          </button>
        </div>
      </div>
    </div>
  );
}
